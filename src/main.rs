use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddrV4},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use krpc::{dump_services_info, KrpcConnection};
use midi::{
    axiom::{AxiomAirController, AxiomMessage, Button},
    cache::Cache,
};

// WORKAROUND: When using Ctrl+C without handler,
// `midir` crate for some reason hangs program for long time
static STOP: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        // On first Ctrl+C it will signal program to properly close down
        // On second Ctrl+C it will forcefully exit process
        if STOP.swap(true, Ordering::SeqCst) {
            std::process::exit(1);
        }
    })?;

    match std::env::args().nth(1).as_deref() {
        Some("run") => run()?,
        Some("dump_services") => {
            let mut krpc = KrpcConnection::connect("127.0.0.1:50000", "midi")?;
            println!("TCP connected.");
            // Get list of available procedures on server and dump it to file
            let services = krpc.get_services()?.unwrap();
            let file = std::fs::File::create("procs.bin")?;
            bincode::serialize_into(file, &services)?;
            println!("Dumping Done.");
        }
        Some("process_services") => {
            let file = std::fs::File::open("procs.bin")?;
            let services = bincode::deserialize_from(file)?;
            std::fs::write("procs.rs", dump_services_info(&services))?;
            println!("Processing Done.");
        }
        _ => println!("Usage: midi [run|dump_services|process_services]"),
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 50000).into();
    let mut krpc = KrpcConnection::connect_timeout(
        &socket_addr,
        Duration::from_secs(3),
        "midi",
    )?;
    println!("TCP connected.");
    let status = krpc.get_status()?;
    match status {
        Ok(status) => println!(
            "Krpc version: {}",
            status.version.as_deref().unwrap_or("<missing>")
        ),
        Err(e) => println!("Error getting status: {:?}", e),
    }

    let (controller, recv) = AxiomAirController::new()?;
    println!("Midi controller connected.");
    println!("System: [ARMED]");

    let mut state = ControlState::new();

    let mut timer = Instant::now();
    while !STOP.load(Ordering::SeqCst) {
        for e in recv.try_iter() {
            match e {
                AxiomMessage::Sustain(Button::Pressed) => {
                    let is_paused: bool = krpc.is_paused()?.unwrap();
                    krpc.pause(!is_paused)?.unwrap();
                }
                AxiomMessage::Knob(1, v) => state.throttle.set(v),
                AxiomMessage::Pad(1, v) if v != 0 => {
                    state.gear.update(|v| *v = !*v)
                }
                _ => (),
            }
        }
        // Batch network calls every frame
        if timer.elapsed() > Duration::from_secs(1) / 60 {
            if state.any_has_changed() {
                if let Err(e) = state.update_server(&mut krpc) {
                    eprintln!("Update error: {:?}", e);
                }
            }
            timer = Instant::now();
        }
    }

    controller.close()?;
    Ok(())
}

struct ControlState {
    pub throttle: Cache<u8>,
    pub gear: Cache<bool>,
}

impl ControlState {
    pub fn new() -> Self {
        Self {
            throttle: Cache::new(0),
            gear: Cache::new(false),
        }
    }

    pub fn any_has_changed(&self) -> bool {
        self.throttle.has_changed() || self.gear.has_changed()
    }

    pub fn update_server(
        &mut self,
        krpc: &mut KrpcConnection,
    ) -> Result<(), Box<dyn Error>> {
        let vessel =
            krpc.get_active_vessel()?.map_err(|x| format!("{:?}", x))?;
        let control =
            vessel.get_control(krpc)?.map_err(|x| format!("{:?}", x))?;
        if let Some(throttle) = self.throttle.get() {
            let throttle = throttle as f32 / 127.0;
            control
                .set_throttle(krpc, throttle)?
                .map_err(|x| format!("{:?}", x))?;
        }
        if let Some(gear) = self.gear.get() {
            control
                .set_gear(krpc, gear)?
                .map_err(|x| format!("{:?}", x))?;
        }
        Ok(())
    }
}

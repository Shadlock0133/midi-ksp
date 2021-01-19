use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use krpc::{dump_procs_sigs, KrpcConnection};
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
        Some("dump_services") => {
            let mut krpc = KrpcConnection::connect("127.0.0.1:50000", "midi")?;
            println!("TCP connected.");
            // Get list of available procedures on server and dump it to file
            let services = krpc.get_services()?.unwrap();
            std::fs::write("procs.rs", dump_procs_sigs(&services))?;
            println!("Done.");
        }
        Some("run") => run()?,
        _ => println!("Usage: midi [run|dump_services]"),
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
    let (controller, recv) = AxiomAirController::new()?;
    println!("Midi controller connected.");

    let mut throttle = Cache::new(0);

    let mut timer = Instant::now();
    while !STOP.load(Ordering::SeqCst) {
        for e in recv.try_iter() {
            match e {
                AxiomMessage::Sustain(Button::Pressed) => {
                    let is_paused: bool = krpc.is_paused()?.unwrap();
                    krpc.pause(!is_paused)?.unwrap();
                }
                AxiomMessage::Stop(Button::Pressed) => {
                    let mut vessel = krpc.get_active_vessel()?.unwrap();
                    let mut control = vessel.get_control(&mut krpc)?.unwrap();
                    let throttle = control.get_throttle(&mut krpc)?.unwrap();
                    println!("Throttle: {}", throttle);
                }
                AxiomMessage::Knob(1, v) => throttle.set(v),
                _ => (),
            }
        }
        // Batch network calls
        if timer.elapsed() > Duration::from_secs(1) / 60 {
            let mut vessel = krpc.get_active_vessel()?.unwrap();
            let mut control = vessel.get_control(&mut krpc)?.unwrap();
            if let Some(throttle) = throttle.get() {
                control
                    .set_throttle(&mut krpc, throttle as f32 / 127.0)?
                    .unwrap();
            }
            timer = Instant::now();
        }
    }

    controller.close()?;
    Ok(())
}

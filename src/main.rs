use std::sync::atomic::{AtomicBool, Ordering};

use midi::{
    axiom::{AxiomAirController, AxiomMessage, Button},
    krpc::{dump_procs_sigs, KrpcConnection},
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
        Some("run") => {
            let mut krpc = KrpcConnection::connect("127.0.0.1:50000", "midi")?;
            println!("TCP connected.");
            let (controller, recv) = AxiomAirController::new()?;
            println!("Midi controller connected.");

            while !STOP.load(Ordering::SeqCst) {
                for e in recv.try_iter() {
                    match e {
                        AxiomMessage::Sustain(Button::Pressed) => {
                            let is_paused: bool = krpc.is_paused()?.unwrap();
                            krpc.pause(!is_paused)?.unwrap();
                        }
                        AxiomMessage::Stop(Button::Pressed) => {
                            let mut vessel = krpc.get_active_vessel()?.unwrap();
                            let mut control = vessel.get_control()?.unwrap();
                            let throttle = control.get_throttle()?.unwrap();
                            println!("Throttle: {}", throttle);
                        }
                        AxiomMessage::Knob(1, v) => {
                            let mut vessel = krpc.get_active_vessel()?.unwrap();
                            let mut control = vessel.get_control()?.unwrap();
                            control.set_throttle(v as f32 / 127.0)?.unwrap();
                        }
                        _ => (),
                    }
                }
            }

            controller.close()?;
        }
        _ => println!("Usage: midi [run|dump_services]"),
    }

    Ok(())
}

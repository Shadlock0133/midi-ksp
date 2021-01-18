use std::sync::atomic::{AtomicBool, Ordering};

use midi::{
    krpc::{dump_procs_sigs, KrpcConnection},
    midi::{AxiomAirController, Channel, MidiMessageIn},
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

    let (controller, recv) = AxiomAirController::new()?;
    println!("Midi controller connected.");
    let mut krpc = KrpcConnection::connect("127.0.0.1:50000", "midi")?;
    println!("TCP connected.");

    // Get list of available procedures on server and dump it to file
    let services = krpc.get_services()?.unwrap();
    std::fs::write("procs.rs", dump_procs_sigs(&services))?;

    while !STOP.load(Ordering::SeqCst) {
        for e in recv.try_iter() {
            match e {
                // Maps to Sustain button
                (1, MidiMessageIn::ControlChange(Channel::Ch1, 64, 127)) => {
                    let is_paused: bool = krpc.get_paused()?.unwrap();
                    krpc.pause(!is_paused)?.unwrap();
                }
                _ => (),
            }
        }
    }

    controller.close()?;

    Ok(())
}

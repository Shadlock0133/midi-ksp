use std::sync::atomic::{AtomicBool, Ordering};

use midi::{
    krpc::{dump_procs_sigs, KrpcConnection},
    midi::{AxiomAirController, Channel, MidiMessageIn},
};

static STOP: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        if STOP.swap(true, Ordering::SeqCst) {
            std::process::exit(1);
        }
    })?;

    let (controller, recv) = AxiomAirController::new()?;
    println!("Midi controller connected.");
    let mut krpc = KrpcConnection::connect("127.0.0.1:50000", "midi")?;
    println!("TCP connected.");

    let services = krpc.get_services()?.unwrap();
    std::fs::write("procs.rs", dump_procs_sigs(&services))?;

    while !STOP.load(Ordering::SeqCst) {
        for e in recv.try_iter() {
            match e {
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

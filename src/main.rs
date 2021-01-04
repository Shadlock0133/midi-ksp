#![allow(deprecated)]

use std::{
    net::TcpStream,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use midi::{AxiomAirController, Channel, MidiMessageIn};
use protobuf::Message;

const TIMEOUT: Duration = Duration::from_secs(10);

static STOP: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| STOP.store(true, Ordering::SeqCst))?;
    let (controller, recv) = AxiomAirController::new()?;
    println!("Midi controller connected.");

    let mut stream = TcpStream::connect("127.0.0.1:50000")?;
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    println!("TCP connected.");

    let crq = krpc::ConnectionRequest {
        client_name: "midi".to_string(),
        client_identifier: vec![],
        field_type: krpc::ConnectionRequest_Type::RPC,
        ..Default::default()
    };
    crq.write_length_delimited_to_writer(&mut stream)?;
    eprintln!("Sent");

    let crp: krpc::ConnectionResponse = protobuf::parse_length_delimited_from_reader(&mut stream)?;

    eprintln!("Response: {:?}", crp);
    assert_eq!(crp.status, krpc::ConnectionResponse_Status::OK);

    while !STOP.load(Ordering::SeqCst) {
        for e in recv.try_iter() {
            eprintln!("{:?}", e);
            // match e {
            //     (2, MidiMessageIn::ControlChange(Channel::Ch16, 33, b)) => {}
            //     _ => (),
            // }
        }
    }

    controller.close()?;

    Ok(())
}

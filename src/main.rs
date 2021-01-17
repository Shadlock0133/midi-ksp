use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::atomic::{AtomicBool, Ordering},
    // time::Duration,
};

use krpc::{
    connection_request::Type, connection_response::Status, Argument,
    ConnectionRequest, ConnectionResponse, Procedure,
};
use midi::{AxiomAirController, Channel, MidiMessageIn};
use protobuf_but_worse::encoding::*;

// const TIMEOUT: Duration = Duration::from_secs(4);

static STOP: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        if STOP.swap(true, Ordering::SeqCst) {
            std::process::exit(1);
        }
    })?;
    let (controller, recv) = AxiomAirController::new()?;
    println!("Midi controller connected.");

    let mut stream = TcpStream::connect("127.0.0.1:50000")?;
    // stream.set_read_timeout(Some(TIMEOUT))?;
    // stream.set_write_timeout(Some(TIMEOUT))?;
    println!("TCP connected.");

    let crq = ConnectionRequest {
        r#type: Some(Type::Rpc),
        client_name: Some("midi".to_string()),
        client_identifier: None,
    };
    crq.encode_with_len(&mut stream)?;
    stream.flush()?;
    eprintln!("Sent");

    let crp = ConnectionResponse::decode_with_len(&mut stream)?;

    eprintln!("Response: {:?}", crp);
    assert!(matches!(crp.status, Some(Status::Ok) | None));
    let services: krpc::Services =
        call(&mut stream, "KRPC", "GetServices", &[])?.unwrap();
    eprintln!("{:#?}", procedure_info(&services, "KRPC", "get_Paused"));
    // eprintln!(
    //     "{:#?}",
    //     procedure_info(&services, "SpaceCenter", "Vessel_get_Control")
    // );
    // eprintln!(
    //     "{:#?}",
    //     procedure_info(&services, "SpaceCenter", "Control_set_Throttle")
    // );
    // let game_scene: GameScene =
    //     call(&mut write, &mut read, "KRPC", "get_CurrentGameScene")?.unwrap();
    // dbg!(game_scene);
    // let vessel: Class =
    //     call(&mut write, &mut read, "SpaceCenter", "get_ActiveVessel")?
    //         .unwrap();

    while !STOP.load(Ordering::SeqCst) {
        for e in recv.try_iter() {
            match e {
                (1, MidiMessageIn::ControlChange(Channel::Ch1, 64, 127)) => {
                    let is_paused: bool =
                        call(&mut stream, "KRPC", "get_Paused", &[])?.unwrap();
                    let _: () = call(
                        &mut stream,
                        "KRPC",
                        "set_Paused",
                        &[&!is_paused],
                    )?
                    .unwrap();
                }
                _ => (),
            }
        }
    }

    controller.close()?;

    Ok(())
}

fn list_procedures<'a>(
    services: &'a krpc::Services,
    module: &str,
) -> Vec<&'a str> {
    services
        .services
        .iter()
        .find(|x| x.name.as_ref().map(|name| name == module).unwrap_or(false))
        .unwrap()
        .procedures
        .iter()
        .filter_map(|x| x.name.as_ref().map(|name| name.as_str()))
        .collect()
}

fn procedure_info<'a>(
    services: &'a krpc::Services,
    module: &str,
    procedure: &str,
) -> &'a Procedure {
    services
        .services
        .iter()
        .find(|x| x.name.as_ref().map(|name| name == module).unwrap_or(false))
        .unwrap()
        .procedures
        .iter()
        .find(|x| {
            x.name
                .as_ref()
                .map(|name| name == procedure)
                .unwrap_or(false)
        })
        .unwrap()
}

fn call<T: Decode, RW: Read + Write>(
    mut stream: RW,
    service: impl Into<String>,
    procedure: impl Into<String>,
    arguments: &[&dyn EncodeDyn],
) -> EncodingResult<Result<T, krpc::Error>> {
    let service = Some(service.into());
    let procedure = Some(procedure.into());
    let arguments = arguments
        .iter()
        .enumerate()
        .map(|(i, x)| {
            Ok(Argument {
                position: Some(i as u32),
                value: Some(x.encode_to_vec()?),
            })
        })
        .collect::<EncodingResult<_>>()?;
    let call = krpc::ProcedureCall {
        service,
        procedure,
        arguments,
        procedure_id: None,
        service_id: None,
    };
    let request = krpc::Request { calls: vec![call] };

    request.encode_with_len(&mut stream)?;
    stream.flush()?;
    let krpc::Response { error, mut results } =
        Decode::decode_with_len(&mut stream)?;

    if let Some(error) = error {
        return Ok(Err(error));
    }
    let result = results.remove(0);
    if let Some(error) = result.error {
        return Ok(Err(error));
    }
    T::decode(
        result
            .value
            .ok_or(EncodingError::MissingField(!0))?
            .as_slice(),
    )
    .map(Ok)
}

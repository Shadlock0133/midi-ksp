use std::{
    net::TcpStream,
    sync::atomic::{AtomicBool, Ordering},
    // time::Duration,
};

use krpc::{
    connection_request::Type,
    connection_response::Status,
    protobuf::{
        self, CodedInputStream, CodedOutputStream, Message, ProtobufResult,
    },
    Argument, ConnectionRequest, ConnectionResponse, Procedure,
};
use midi::{AxiomAirController, Channel, MidiMessageIn};
use protobuf::{
    well_known_types::{BoolValue, Empty, Value},
    MessageDyn,
};

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
    let mut stream2 = stream.try_clone()?;
    let mut read = CodedInputStream::new(&mut stream2);
    let mut write = CodedOutputStream::new(&mut stream);
    println!("TCP connected.");

    let crq = ConnectionRequest {
        client_name: "midi".to_string(),
        field_type: protobuf::ProtobufEnumOrUnknown::new(Type::RPC),
        ..Default::default()
    };
    crq.write_length_delimited_to(&mut write)?;
    write.flush()?;
    eprintln!("Sent");

    let crp = read_packet::<ConnectionResponse>(&mut read)?;

    eprintln!("Response: {:?}", crp);
    assert_eq!(crp.status.unwrap(), Status::OK);
    let services: krpc::Services =
        call(&mut write, &mut read, "KRPC", "GetServices", &[])?.unwrap();
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
                        call(&mut write, &mut read, "KRPC", "get_Paused", &[])?
                            .unwrap();
                    let _: Empty = call(
                        &mut write,
                        &mut read,
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

fn read_packet<M: Message>(read: &mut CodedInputStream) -> ProtobufResult<M> {
    let len = read.read_uint32()?;
    let bytes = read.read_raw_bytes(len)?;
    M::parse_from_bytes(&bytes)
}

fn list_procedures<'a>(
    services: &'a krpc::Services,
    module: &str,
) -> Vec<&'a str> {
    services
        .services
        .iter()
        .find(|x| x.name == module)
        .unwrap()
        .procedures
        .iter()
        .map(|x| x.name.as_str())
        .collect()
}

fn procedure_info<'a>(
    services: &'a krpc::Services,
    module: &str,
    name: &str,
) -> &'a Procedure {
    services
        .services
        .iter()
        .find(|x| x.name == module)
        .unwrap()
        .procedures
        .iter()
        .find(|x| x.name == name)
        .unwrap()
}

fn call<R: Message>(
    write: &mut CodedOutputStream,
    read: &mut CodedInputStream,
    service: impl Into<String>,
    procedure: impl Into<String>,
    arguments: &[&dyn MessageDyn],
) -> ProtobufResult<Result<R, krpc::Error>> {
    let service = service.into();
    let procedure = procedure.into();
    let arguments = arguments
        .iter()
        .enumerate()
        .map(|(i, x)| {
            Ok(Argument {
                position: i as u32,
                value: x.write_to_bytes_dyn()?,
            })
        })
        .collect::<ProtobufResult<_>>()?;
    let call = krpc::ProcedureCall {
        service,
        procedure,
        arguments,
        ..Default::default()
    };
    let request = krpc::Request { calls: vec![call] };
    request.write_length_delimited_to(write)?;
    write.flush()?;
    let krpc::Response { error, mut results } = read_packet(read)?;
    if let Some(error) = error.into_option() {
        return Ok(Err(error));
    }
    let result = results.remove(0);
    if let Some(error) = result.error.into_option() {
        return Ok(Err(error));
    }
    R::parse_from_bytes(&result.value).map(Ok)
}

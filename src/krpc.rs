use std::{
    io::Write,
    net::{TcpStream, ToSocketAddrs},
};

use krpc::{
    connection_request::Type, connection_response::Status, Argument,
    ConnectionRequest, ConnectionResponse, Procedure, Services,
};
use protobuf_but_worse::encoding::*;

pub struct KrpcConnection {
    stream: TcpStream,
}

impl KrpcConnection {
    pub fn connect<A: ToSocketAddrs>(
        addr: A,
        name: impl Into<String>,
    ) -> Result<Self, EncodingError> {
        let mut stream = TcpStream::connect(addr)?;
        let crq = ConnectionRequest {
            r#type: Some(Type::Rpc),
            client_name: Some(name.into()),
            client_identifier: None,
        };
        crq.encode_with_len(&mut stream)?;
        stream.flush()?;
        let crp = ConnectionResponse::decode_with_len(&mut stream)?;
        if !matches!(crp.status, Some(Status::Ok) | None) {
            let io_error = std::io::ErrorKind::ConnectionRefused.into();
            return Err(EncodingError::Io(io_error)
                .context(crp.message.unwrap_or_default()));
        }
        Ok(Self { stream })
    }

    pub fn get_paused(&mut self) -> EncodingResult<Result<bool, krpc::Error>> {
        self.call("KRPC", "get_Paused", &[])
    }

    pub fn pause(
        &mut self,
        value: bool,
    ) -> EncodingResult<Result<(), krpc::Error>> {
        self.call("KRPC", "set_Paused", &[&value])
    }

    pub fn get_services(
        &mut self,
    ) -> EncodingResult<Result<Services, krpc::Error>> {
        self.call("KRPC", "GetServices", &[])
    }

    pub fn call<T: Decode>(
        &mut self,
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

        request.encode_with_len(&mut self.stream)?;
        self.stream.flush()?;
        let krpc::Response { error, mut results } =
            Decode::decode_with_len(&mut self.stream)?;

        if let Some(error) = error {
            return Ok(Err(error));
        }
        let result = results.remove(0);
        if let Some(error) = result.error {
            return Ok(Err(error));
        }
        T::decode(result.value.as_deref().unwrap_or_default()).map(Ok)
    }
}

pub fn print_procedure_signature(procedure: &Procedure) -> String {
    use std::fmt::Write;
    let mut res = String::new();

    let mut doc = procedure
        .documentation
        .as_deref()
        .unwrap_or_default()
        .to_string();
    doc = doc.replace("<doc>", "");
    doc = doc.replace("</doc>", "");
    doc = doc.replace("<summary>", "");
    doc = doc.replace("</summary>", "");
    doc = doc.replace("<returns>", "# Returns\n\n");
    doc = doc.replace("</returns>", "\n");
    let doc = doc.trim();
    for doc_line in doc.lines() {
        writeln!(res, "/// {}", doc_line).unwrap();
    }

    write!(res, "fn {}(", procedure.name.as_ref().unwrap()).unwrap();
    if let Some(param) = &procedure.parameters.first() {
        let name = match param.name.as_deref().unwrap().trim() {
            "type" => "r#type",
            n => n,
        };
        write!(
            res,
            "{}: {:?}",
            name,
            param.r#type.as_ref().unwrap().code.as_ref().unwrap()
        )
        .unwrap();
        for param in &procedure.parameters[1..] {
            let name = match param.name.as_deref().unwrap().trim() {
                "type" => "r#type",
                n => n,
            };
            write!(
                res,
                ", {}: {:?}",
                name,
                param.r#type.as_ref().unwrap().code.as_ref().unwrap()
            )
            .unwrap();
        }
    }
    write!(res, ")").unwrap();

    if let Some(ret) =
        procedure.return_type.as_ref().and_then(|x| x.code.as_ref())
    {
        write!(res, " -> {:?}", ret).unwrap();
    }
    writeln!(res, ";").unwrap();
    res
}

pub fn dump_procs_sigs(services: &krpc::Services) -> String {
    use std::fmt::Write;
    let mut res = String::new();
    for service in &services.services {
        let service_name = service.name.as_deref().unwrap();
        writeln!(res, "mod {} {{", service_name).unwrap();
        for proc in &service.procedures {
            let text = print_procedure_signature(proc);
            for line in text.lines() {
                writeln!(res, "    {}", line).unwrap();
            }
            writeln!(res).unwrap();
        }
        writeln!(res, "}}").unwrap();
        writeln!(res).unwrap();
    }
    res
}

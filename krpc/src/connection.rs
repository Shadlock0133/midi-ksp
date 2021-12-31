use std::{
    io::Write,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
    time::Duration,
};

use krpc_proto::{
    connection_request::Type, connection_response::Status, Argument,
    ConnectionRequest, ConnectionResponse, ProcedureCall, Request, Response,
    Services,
};
use protobuf_but_worse::encoding::*;

use crate::{vessel::Vessel, CallResult};

pub struct KrpcConnection {
    stream: TcpStream,
}

impl KrpcConnection {
    /// Connects to KRPC server, sends connection request,
    /// and checks response status
    pub fn connect<A: ToSocketAddrs>(
        addr: A,
        name: impl Into<String>,
    ) -> Result<Self, EncodingError> {
        let stream = TcpStream::connect(addr)?;
        Self::_connect(stream, name.into())
    }

    /// Connects to KRPC server, sends connection request,
    /// and checks response status
    pub fn connect_timeout(
        addr: &SocketAddr,
        timeout: Duration,
        name: impl Into<String>,
    ) -> Result<Self, EncodingError> {
        let stream = TcpStream::connect_timeout(addr, timeout)?;
        Self::_connect(stream, name.into())
    }

    fn _connect(
        mut stream: TcpStream,
        name: String,
    ) -> Result<Self, EncodingError> {
        let crq = ConnectionRequest {
            r#type: Some(Type::Rpc),
            client_name: Some(name),
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

    pub fn get_status(&mut self) -> CallResult<krpc_proto::Status> {
        self.call("KRPC", "GetStatus", &[])
    }

    pub fn get_services(&mut self) -> CallResult<Services> {
        self.call("KRPC", "GetServices", &[])
    }

    pub fn is_paused(&mut self) -> CallResult<bool> {
        self.call("KRPC", "get_Paused", &[])
    }

    pub fn pause(&mut self, value: bool) -> CallResult {
        self.call("KRPC", "set_Paused", &[&value])
    }

    pub fn get_active_vessel(&mut self) -> CallResult<Vessel> {
        self.call("SpaceCenter", "get_ActiveVessel", &[])
            .map(Vessel::new)
    }

    /// Performs a remote procedure call
    ///
    /// Returns double Result, because
    /// EncodingError is a connection error and
    /// krpc_proto::Error is a server error
    pub fn call<T: Decode>(
        &mut self,
        service: impl Into<String>,
        procedure: impl Into<String>,
        arguments: &[&dyn EncodeDyn],
    ) -> CallResult<T> {
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
        let call = ProcedureCall {
            service,
            procedure,
            arguments,
            procedure_id: None,
            service_id: None,
        };
        let request = Request { calls: vec![call] };

        request.encode_with_len(&mut self.stream)?;
        self.stream.flush().map_err(EncodingError::from)?;
        let Response { error, mut results } =
            Decode::decode_with_len(&mut self.stream)?;

            if let Some(error) = error {
            return Err(error.into());
        }
        let result = results.remove(0);
        if let Some(error) = result.error {
            return Err(error.into());
        }
        Ok(T::decode(result.value.as_deref().unwrap_or_default())?)
    }
}

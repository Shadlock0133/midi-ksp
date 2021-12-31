mod class;
mod connection;
mod control;
mod dump_docs;
mod vessel;

use krpc_proto::Error as ProtoError;
use protobuf_but_worse::encoding::EncodingError;

pub use connection::KrpcConnection;
pub use control::Control;
pub use vessel::Vessel;

pub use dump_docs::dump_services_info;

#[derive(Debug, thiserror::Error)]
pub enum CallError {
    #[error("Encoding error: {0}")]
    Encoding(#[from] EncodingError),
    #[error("Protocol error: {0:?}")]
    Proto(#[from] ProtoError),
}

type CallResult<T = ()> = Result<T, CallError>;

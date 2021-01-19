use krpc_proto::Error;
use protobuf_but_worse::encoding::EncodingResult;

use crate::{class::Class, KrpcConnection};

pub struct Control {
    pub(crate) class: Class,
}

impl Control {
    pub fn set_throttle(
        &mut self,
        krpc: &mut KrpcConnection,
        value: f32,
    ) -> EncodingResult<Result<(), Error>> {
        krpc.call(
            "SpaceCenter",
            "Control_set_Throttle",
            &[&self.class, &value],
        )
    }

    pub fn get_throttle(
        &mut self,
        krpc: &mut KrpcConnection,
    ) -> EncodingResult<Result<f32, Error>> {
        krpc.call("SpaceCenter", "Control_get_Throttle", &[&self.class])
    }
}

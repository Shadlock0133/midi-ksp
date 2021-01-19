use krpc_proto::Error;
use protobuf_but_worse::encoding::EncodingResult;

use crate::{class::Class, control::Control, KrpcConnection};

pub struct Vessel {
    pub(crate) class: Class,
}

impl Vessel {
    pub fn name(
        &mut self,
        krpc: &mut KrpcConnection,
    ) -> EncodingResult<Result<String, Error>> {
        krpc.call("SpaceCenter", "Vessel_get_Name", &[&self.class])
    }

    pub fn get_control(
        &mut self,
        krpc: &mut KrpcConnection,
    ) -> EncodingResult<Result<Control, Error>> {
        krpc.call("SpaceCenter", "Vessel_get_Control", &[&self.class])
            .map(move |r| {
                let class = r?;
                Ok(Control { class })
            })
    }
}

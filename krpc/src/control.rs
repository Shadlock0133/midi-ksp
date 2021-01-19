use krpc_proto::{Error, List};
use protobuf_but_worse::encoding::EncodingResult;

use crate::{class::Class, KrpcConnection};

pub struct Control {
    pub(crate) class: Class,
}

impl Control {
    /// The state of the throttle. A value between 0 and 1.
    pub fn set_throttle(
        &self,
        krpc: &mut KrpcConnection,
        value: f32,
    ) -> EncodingResult<Result<(), Error>> {
        krpc.call(
            "SpaceCenter",
            "Control_set_Throttle",
            &[&self.class, &value],
        )
    }

    /// The state of the throttle. A value between 0 and 1.
    pub fn get_throttle(
        &self,
        krpc: &mut KrpcConnection,
    ) -> EncodingResult<Result<f32, Error>> {
        krpc.call("SpaceCenter", "Control_get_Throttle", &[&self.class])
    }

    /// Activates the next stage. Equivalent to pressing the space bar in-game.
    ///
    /// # Returns
    ///
    /// A list of vessel objects that are jettisoned from the active vessel.
    ///
    /// # Note
    ///
    /// When called, the active vessel may change. It is therefore possible that,
    /// after calling this function, the object(s) returned by previous call(s) to
    /// KrpcConnection::get_active_vessel no longer refer to the active vessel.
    pub fn activate_next_stage(
        &self,
        krpc: &mut KrpcConnection,
    ) -> EncodingResult<Result<List, Error>> {
        krpc.call("SpaceCenter", "Control_ActivateNextStage", &[&self.class])
    }

    /// The state of the landing gear/legs.
    pub fn get_gear(
        &self,
        krpc: &mut KrpcConnection,
    ) -> EncodingResult<Result<bool, Error>> {
        krpc.call("SpaceCenter", "Control_get_Gear", &[&self.class])
    }

    /// The state of the landing gear/legs.
    pub fn set_gear(
        &self,
        krpc: &mut KrpcConnection,
        value: bool,
    ) -> EncodingResult<Result<(), Error>> {
        krpc.call("SpaceCenter", "Control_set_Gear", &[&self.class, &value])
    }
}

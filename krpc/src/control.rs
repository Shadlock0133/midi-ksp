use krpc_proto::List;
use protobuf_but_worse::encoding::Varint;

use crate::{class::Class, CallResult, KrpcConnection};

pub struct Control {
    class: Class,
}

impl Control {
    pub(crate) fn new(class: Class) -> Self {
        Self { class }
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
    ) -> CallResult<List> {
        krpc.call("SpaceCenter", "Control_ActivateNextStage", &[&self.class])
    }

    // Toggles the state of the given action group.
    ///
    /// `group` is a number between 0 and 9 inclusive,
    /// or between 0 and 250 inclusive when the Extended Action Groups mod is
    /// installed.
    pub fn toggle_action_group(
        &self,
        krpc: &mut KrpcConnection,
        group: u32,
    ) -> CallResult {
        krpc.call(
            "SpaceCenter",
            "Control_ToggleActionGroup",
            &[&self.class, &Varint(group)],
        )
    }

    /// The state of the throttle. A value between 0 and 1.
    pub fn set_throttle(
        &self,
        krpc: &mut KrpcConnection,
        value: f32,
    ) -> CallResult {
        krpc.call(
            "SpaceCenter",
            "Control_set_Throttle",
            &[&self.class, &value],
        )
    }

    /// The state of the throttle. A value between 0 and 1.
    pub fn get_throttle(&self, krpc: &mut KrpcConnection) -> CallResult<f32> {
        krpc.call("SpaceCenter", "Control_get_Throttle", &[&self.class])
    }

    /// The state of the landing gear/legs.
    pub fn get_gear(&self, krpc: &mut KrpcConnection) -> CallResult<bool> {
        krpc.call("SpaceCenter", "Control_get_Gear", &[&self.class])
    }

    /// The state of the landing gear/legs.
    pub fn set_gear(
        &self,
        krpc: &mut KrpcConnection,
        value: bool,
    ) -> CallResult {
        krpc.call("SpaceCenter", "Control_set_Gear", &[&self.class, &value])
    }
}

use crate::{CallResult, KrpcConnection, class::Class, control::Control};

pub struct Vessel {
    class: Class,
}

impl Vessel {
    pub(crate) fn new(class: Class) -> Self {
        Self { class }
    }

    pub fn name(
        &self,
        krpc: &mut KrpcConnection,
    ) -> CallResult<String> {
        krpc.call("SpaceCenter", "Vessel_get_Name", &[&self.class])
    }

    pub fn get_control(
        &self,
        krpc: &mut KrpcConnection,
    ) -> CallResult<Control> {
        krpc.call("SpaceCenter", "Vessel_get_Control", &[&self.class])
            .map(|r| {
                let class = r?;
                Ok(Control::new(class))
            })
    }
}

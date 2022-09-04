mod camera;
mod controller;

use bevy::{prelude::*, time::FixedTimestep};

pub use crate::camera::{camera_system, ThirdPersonCamera};
pub use crate::controller::{controller_system, ThirdPersonController};

const TIMESTEP_50_PER_SECOND: f64 = 1.0 / 50.0;

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThirdPersonCamera>()
            .register_type::<ThirdPersonController>()
            .add_system_to_stage(CoreStage::PostUpdate, camera_system)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIMESTEP_50_PER_SECOND))
                    .with_system(controller_system),
            );
    }
}

pub mod prelude {
    pub use crate::camera::{camera_system, ThirdPersonCamera};
    pub use crate::controller::{controller_system, ThirdPersonController};
    pub use crate::ThirdPersonControllerPlugin;
}

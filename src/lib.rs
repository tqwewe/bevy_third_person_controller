pub mod camera;
pub mod controller;

use bevy::{prelude::*, time::FixedTimestep};
use camera::camera_orbit_target_system;
use controller::{controller_system, ThirdPersonController};

const TIMESTEP_50_PER_SECOND: f64 = 50.0 / 60.0 / 60.0;

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThirdPersonController>().add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_50_PER_SECOND))
                .with_system(camera_orbit_target_system)
                .with_system(controller_system.before(camera_orbit_target_system)),
        );
    }
}

pub mod camera;
pub mod controller;

use bevy::prelude::*;
use camera::camera_orbit_target_system;
use controller::controller_system;

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_orbit_target_system)
            .add_system(controller_system.after(camera_orbit_target_system));
    }
}

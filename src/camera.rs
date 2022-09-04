use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::controller::ThirdPersonController;

const ANGLE_EPSILON_LOWER: f32 = 1.4;
const ANGLE_EPSILON_UPPER: f32 = 0.3;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ThirdPersonCamera {
    pub target_id: u64,
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
    pub distance: f32,
    pub min_height: f32,
    pub target_offset: Vec3,
    pub position_offset: Vec3,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            target_id: 0,
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.001,
            distance: 8.0,
            min_height: 0.2,
            target_offset: Vec3::ZERO,
            position_offset: Vec3::ZERO,
        }
    }
}

pub fn camera_system(
    windows: Res<Windows>,
    mut mouse_events: EventReader<MouseMotion>,
    mut camera_query: Query<
        (&mut Transform, &mut ThirdPersonCamera),
        Without<ThirdPersonController>,
    >,
    controller_query: Query<(&ThirdPersonController, &Transform), Without<ThirdPersonCamera>>,
) {
    // If window is focused
    if windows
        .get_primary()
        .map(|window| window.is_focused())
        .unwrap_or(false)
    {
        let mut mouse_delta = mouse_events
            .iter()
            .fold(Vec2::ZERO, |acc, event| acc + event.delta);

        for (mut camera_transform, mut camera) in &mut camera_query {
            mouse_delta *= camera.sensitivity;

            camera.pitch = (camera.pitch - mouse_delta.y).clamp(
                -FRAC_PI_2 + ANGLE_EPSILON_UPPER,
                FRAC_PI_2 - ANGLE_EPSILON_LOWER,
            );
            camera.yaw -= mouse_delta.x;
            camera.yaw %= TAU;

            let targets = controller_query.iter().filter_map(|(controller, target)| {
                if controller.id == camera.target_id {
                    Some(target)
                } else {
                    None
                }
            });
            for target_transform in targets {
                let mut target = target_transform.translation;
                target.y = 0.0;

                let mut pos = Transform::from_translation(
                    target + Vec3::Z * camera.distance + camera.target_offset,
                );
                pos.rotate_around(
                    target,
                    Quat::from_euler(EulerRot::ZYX, 0.0, camera.yaw, camera.pitch),
                );
                pos.translation += camera.position_offset;

                *camera_transform = pos;
            }
        }
    }
}

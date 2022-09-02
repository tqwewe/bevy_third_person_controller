use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::controller::ThirdPersonController;

const ANGLE_EPSILON: f32 = 0.001953125;

#[derive(Component)]
pub struct ThirdPersonCamera {
    pub target_id: u8,
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
    pub distance: f32,
    pub offset: Vec3,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            target_id: 0,
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.001,
            distance: 8.0,
            offset: Vec3::ZERO,
        }
    }
}

// #[derive(Component, Default)]
// pub struct ThirdPersonCameraTarget(pub u8);

pub fn camera_orbit_target_system(
    windows: Res<Windows>,
    mut mouse_events: EventReader<MouseMotion>,
    mut camera_query: Query<
        (&mut Transform, &mut ThirdPersonCamera),
        Without<ThirdPersonController>,
    >,
    controller_query: Query<&Transform, (With<ThirdPersonController>, Without<ThirdPersonCamera>)>,
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

        // If mouse moved
        if mouse_delta != Vec2::ZERO {
            for (mut camera_transform, mut camera) in &mut camera_query {
                mouse_delta *= camera.sensitivity;

                camera.pitch = (camera.pitch - mouse_delta.y)
                    .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
                camera.yaw -= mouse_delta.x;

                for target_transform in &controller_query {
                    let mut target = target_transform.translation;
                    target.y = 0.0;

                    let mut behind = Transform::from_translation(
                        target + Vec3::Z * camera.distance + camera.offset,
                    );
                    behind.rotate_around(
                        target,
                        Quat::from_euler(EulerRot::ZYX, 0.0, camera.yaw, camera.pitch),
                    );

                    *camera_transform = behind;
                }
            }
        }
    }
}

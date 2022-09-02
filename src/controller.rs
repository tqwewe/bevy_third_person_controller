use bevy::prelude::*;

use crate::camera::ThirdPersonCamera;

#[derive(Component)]
pub struct ThirdPersonController {
    pub id: u8,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
}

impl Default for ThirdPersonController {
    fn default() -> Self {
        Self {
            id: 0,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
        }
    }
}

pub fn controller_system(
    key_input: Res<Input<KeyCode>>,
    mut controller_query: Query<(&ThirdPersonController, &mut Transform)>,
    camera_query: Query<(&ThirdPersonCamera, &Transform), Without<ThirdPersonController>>,
) {
    for (controller, mut controller_transform) in &mut controller_query {
        let camera_transform = camera_query.iter().find_map(|(camera, camera_transform)| {
            (camera.target_id == controller.id).then_some(camera_transform)
        });
        if let Some(camera_transform) = camera_transform {
            let move_delta = Vec2::new(
                get_axis(&key_input, controller.key_right, controller.key_left), // -1, 1
                get_axis(&key_input, controller.key_forward, controller.key_back), // -1, 1
            );

            if move_delta != Vec2::ZERO {
                controller_transform.rotation.y =
                    f32::atan2(move_delta.y, move_delta.x) + camera_transform.rotation.y;
            }
        }
    }
}

fn get_axis(key_input: &Res<Input<KeyCode>>, key_pos: KeyCode, key_neg: KeyCode) -> f32 {
    get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
}

fn get_pressed(key_input: &Res<Input<KeyCode>>, key: KeyCode) -> f32 {
    if key_input.pressed(key) {
        1.0
    } else {
        0.0
    }
}

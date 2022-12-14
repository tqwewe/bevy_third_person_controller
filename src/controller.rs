use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_polyline::prelude::Polyline;
use bevy_rapier3d::prelude::*;

use crate::camera::ThirdPersonCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ThirdPersonController {
    pub id: u64,
    pub rotation_speed: f32,
    pub movement_speed: f32,
    pub sprint_speed: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    #[reflect(ignore)]
    pub key_forward: KeyCode,
    #[reflect(ignore)]
    pub key_back: KeyCode,
    #[reflect(ignore)]
    pub key_left: KeyCode,
    #[reflect(ignore)]
    pub key_right: KeyCode,
    #[reflect(ignore)]
    pub key_sprint: KeyCode,
}

impl Default for ThirdPersonController {
    fn default() -> Self {
        Self {
            id: 0,
            rotation_speed: 160.0,
            movement_speed: 20.0,
            sprint_speed: 30.0,
            max_speed: 3500.0,
            friction: 100.0,
            velocity: Vec3::ZERO,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_sprint: KeyCode::LShift,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct FixedUpdateTimer(Timer);

impl Default for FixedUpdateTimer {
    fn default() -> Self {
        let timer = Timer::from_seconds(1.0 / 50.0, true);
        Self(timer)
    }
}

pub fn controller_system(
    time: Res<Time>,
    mut timer: Local<FixedUpdateTimer>,
    key_input: Res<Input<KeyCode>>,
    physics_context: Res<RapierContext>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut controller_query: Query<(
        Entity,
        &mut ThirdPersonController,
        &mut Transform,
        &mut Velocity,
        &Collider,
    )>,
    camera_query: Query<(&ThirdPersonCamera, &Transform), Without<ThirdPersonController>>,
    colliders: Query<&Transform, (Without<ThirdPersonController>, With<Collider>)>,
) {
    timer.tick(time.delta());

    for (entity, mut controller, mut transform, mut velocity, collider) in &mut controller_query {
        let camera_transform = camera_query.iter().find_map(|(camera, camera_transform)| {
            (camera.target_id == controller.id).then_some(camera_transform)
        });
        if let Some(camera_transform) = camera_transform {
            let move_delta = Vec2::new(
                get_axis(&key_input, controller.key_right, controller.key_left),
                get_axis(&key_input, controller.key_forward, controller.key_back),
            );

            let acc = if move_delta != Vec2::ZERO {
                let target_rotation = Quat::from_rotation_y(
                    f32::atan2(move_delta.y, move_delta.x)
                        + camera_transform.rotation.to_scaled_axis().y
                        - FRAC_PI_2,
                );
                let s = (controller.rotation_speed * time.delta_seconds())
                    .max(0.0)
                    .min(1.0);
                transform.rotation = transform.rotation.lerp(target_rotation, s);

                transform.forward()
            } else {
                Vec3::ZERO
            };

            let movement_speed = key_input
                .pressed(controller.key_sprint)
                .then_some(controller.sprint_speed)
                .unwrap_or(controller.movement_speed);

            let mut new_velocity = move_ground(
                acc,
                controller.velocity, // velocity.linvel,
                controller.friction,
                movement_speed,
                controller.max_speed,
                timer.elapsed_secs(),
            );

            if new_velocity != Vec3::ZERO {
                let length = (new_velocity).length_squared();
                let shape_pos = transform.translation;
                let cast = physics_context.cast_shape(
                    shape_pos,
                    transform.rotation,
                    new_velocity,
                    collider,
                    length,
                    QueryFilter::default()
                        .exclude_rigid_body(entity)
                        .exclude_sensors(),
                );

                if let Some((_other_entity, toi)) = cast {
                    for (_, polyline) in polylines.iter_mut() {
                        // if let Ok(_) = colliders.get(other_entity) {
                        // wirness1 is wall offset?
                        polyline.vertices = vec![
                            toi.witness1,
                            shape_pos + new_velocity + toi.witness2,
                            // transform.translation + new_velocity + new_velocity.normalize() * 0.5,
                            // Vec3::ZERO, //  transform.translation + new_velocity * toi.toi,
                        ];

                        // polyline.vertices = vec![
                        // transform.translation + Vec3::Y * 0.5,
                        // transform.translation + Vec3::Y * 0.5 + Transform::from_translation(translation),
                        // ];
                        // }
                    }
                    // dbg!(length, toi.toi, toi.status);
                    // new_velocity -= Vec3::ONE * (length - toi.toi);
                    // new_velocity *= (toi.toi - 0.2).max(0.0);
                    new_velocity *= toi.toi;

                    // Get distance of pos + vel
                    // new_velocity -= (shape_pos + new_velocity) - toi.witness1;
                }
            }

            // velocity.linvel = new_velocity;
            controller.velocity = new_velocity;
            transform.translation += controller.velocity;
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

/// accelDir: normalized direction that the player has requested to move (taking into account the movement keys and look direction)
/// prevVelocity: The current velocity of the player, before any additional calculations
/// accelerate: The server-defined player acceleration value
/// max_velocity: The server-defined maximum player velocity (this is not strictly adhered to due to strafejumping)
///
/// Thanks to this article.
/// <https://adrianb.io/2015/02/14/bunnyhop.html>
fn accelerate(
    accel_dir: Vec3,
    prev_velocity: Vec3,
    accelerate: f32,
    max_velocity: f32,
    dt: f32,
) -> Vec3 {
    let proj_vel = Vec3::dot(prev_velocity, accel_dir); // Vector projection of Current velocity onto accelDir.
    let mut accel_vel = accelerate * dt; // Accelerated velocity in direction of movment

    // If necessary, truncate the accelerated velocity so the vector projection does not exceed max_velocity
    if proj_vel + accel_vel > max_velocity {
        accel_vel = max_velocity - proj_vel;
    }

    prev_velocity + accel_dir * accel_vel
}

fn move_ground(
    accel_dir: Vec3,
    mut prev_velocity: Vec3,
    friction: f32,
    ground_accelerate: f32,
    max_velocity_ground: f32,
    dt: f32,
) -> Vec3 {
    // Apply Friction
    let speed = prev_velocity.length();
    // To avoid divide by zero errors
    if speed != 0.0 {
        let drop = speed * friction * dt;
        prev_velocity *= f32::max(speed - drop, 0.0) / speed; // Scale the velocity based on friction.
    }

    // ground_accelerate and max_velocity_ground are server-defined movement variables
    accelerate(
        accel_dir,
        prev_velocity,
        ground_accelerate,
        max_velocity_ground,
        dt,
    )
}

fn _move_air(
    accel_dir: Vec3,
    prev_velocity: Vec3,
    air_accelerate: f32,
    max_velocity_air: f32,
    dt: f32,
) -> Vec3 {
    // air_accelerate and max_velocity_air are server-defined movement variables
    accelerate(
        accel_dir,
        prev_velocity,
        air_accelerate,
        max_velocity_air,
        dt,
    )
}

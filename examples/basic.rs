//! A simple 3D scene with light shining over a cube sitting on a plane.

use std::f32::consts::FRAC_PI_2;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_polyline::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_controller::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PolylinePlugin)
        .add_plugin(ThirdPersonControllerPlugin)
        .add_startup_system(setup)
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
) {
    commands.spawn_bundle(PolylineBundle {
        polyline: polylines.add(Polyline {
            vertices: vec![-Vec3::ONE, Vec3::ONE],
        }),
        material: polyline_materials.add(PolylineMaterial {
            width: 5.0,
            color: Color::RED,
            perspective: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    // Plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, -0.5, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.05, 50.0));

    // Wall
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgba(0.3, 0.1, 0.6, 0.5).into()),
            transform: Transform::from_xyz(0.0, 5.0, -10.0).with_scale(Vec3::new(10.0, 10.0, 0.2)),
            ..default()
        })
        .insert(Collider::cuboid(0.5, 0.5, 0.5));

    // Player
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { ..default() })),
            material: materials.add(Color::rgba(0.8, 0.7, 0.6, 0.01).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(ThirdPersonController::default())
        .insert(RigidBody::KinematicPositionBased)
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule(Vec3::Y * 0.0, Vec3::Y * 0.85, 0.5))
        .insert(Ccd { enabled: true })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(0.7, 0.2, 0.2))),
                material: materials.add(Color::rgb(0.3, 0.7, 0.3).into()),
                transform: Transform::from_xyz(0.0, 0.4, -0.5),
                ..default()
            });

            // parent
            //     .spawn()
            //     .insert(Collider::capsule(Vec3::Y * 0.0, Vec3::Y * 0.85, 0.5))
            //     .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -0.5, 0.0)));
        });

    // Light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(ThirdPersonCamera {
            target_offset: Vec3::Y * 1.5,
            position_offset: Vec3::Y * 0.8,
            ..default()
        });
}

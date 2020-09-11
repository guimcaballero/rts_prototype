use crate::bundles::*;
use crate::systems::{drone, unit::TargetIndicator};
use bevy::{math::Quat, prelude::*};
use bevy_contrib_colors::Tailwind;
use bevy_mod_picking::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_entity = Entity::new();

    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
            material: materials.add(Tailwind::RED100.into()),
            ..Default::default()
        })
        .with(PickableMesh::new(camera_entity))
        // Units
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .with_bundle(UnitBundle::new(camera_entity))
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::new(5.0, 1.0, 3.0),
            ..Default::default()
        })
        .with_bundle(UnitBundle::new(camera_entity))
        // Drone
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::new(-25.0, 40.0, 0.0),
            rotation: Rotation(Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize()),
            ..Default::default()
        })
        .with(drone::Drone::default())
        .with_bundle(UnitBundle::new_with_has_camera(camera_entity))
        // Target sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Tailwind::GREEN400.into()),
            translation: Translation::new(5.0, 1.0, 3.0),
            ..Default::default()
        })
        .with(TargetIndicator)
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        // camera
        .spawn_as_entity(
            camera_entity,
            Camera3dComponents {
                translation: Translation::new(0.0, 0.0, 0.0),
                rotation: Rotation(Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize()),
                ..Default::default()
            },
        );
}

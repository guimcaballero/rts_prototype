use crate::systems::movement::TargetPosition;
use bevy::{math::Quat, prelude::*};
use bevy_contrib_colors::Tailwind;
use bevy_mod_picking::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_entity = Entity::new();
    let cube1_entity = Entity::new();
    let cube2_entity = Entity::new();

    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 40.0 })),
            material: materials.add(Tailwind::RED100.into()),
            ..Default::default()
        })
        .with(PickableMesh::new(camera_entity))
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .with(TargetPosition::new(cube1_entity))
        .with(PickableMesh::new(camera_entity))
        .with(HighlightablePickMesh::new())
        .with(SelectablePickMesh::new())
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::new(5.0, 1.0, 3.0),
            ..Default::default()
        })
        .with(TargetPosition::new(cube2_entity))
        .with(PickableMesh::new(camera_entity))
        .with(HighlightablePickMesh::new())
        .with(SelectablePickMesh::new())
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        // camera
        .spawn_as_entity(
            camera_entity,
            Camera3dComponents {
                translation: Translation::new(0.0, 20.0, 0.0),
                rotation: Rotation(Quat::from_xyzw(-0.5, -0.5, -0.5, 0.5).normalize()),
                ..Default::default()
            },
        );
}

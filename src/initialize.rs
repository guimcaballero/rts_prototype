use crate::bundles::*;
use crate::helpers::shapes::*;
use crate::systems::{drone, selection::DragSelectionRectangle, unit::TargetIndicator, walker};
use bevy::{math::Quat, prelude::*};
use bevy_contrib_colors::Tailwind;
use bevy_mod_picking::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
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
        // Target sphere
        .spawn(SpriteComponents {
            material: color_materials.add(Color::rgb(0.0, 0.0, 0.8).into()),
            mesh: meshes.add(circle_mesh()),
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            scale: Scale(0.01),
            ..Default::default()
        })
        .with(TargetIndicator)
        // Drag Selection rectangle
        .spawn(SpriteComponents {
            material: color_materials.add(Color::rgba(0.0, 0.0, 0.8, 0.1).into()),
            mesh: meshes.add(rectangle_mesh()),
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            translation: Vec3::new(0.0, 0.1, 0.0).into(),
            ..Default::default()
        })
        .with(DragSelectionRectangle)
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

    create_walker(
        &mut commands,
        &mut meshes,
        &mut materials,
        camera_entity,
        Vec3::zero(),
        false,
    );
    create_walker(
        &mut commands,
        &mut meshes,
        &mut materials,
        camera_entity,
        Vec3::new(10.0, 1.0, 0.0),
        false,
    );
    create_drone(
        &mut commands,
        &mut meshes,
        &mut materials,
        camera_entity,
        Vec3::new(10.0, 20.0, 5.0),
        false,
    );
    create_drone(
        &mut commands,
        &mut meshes,
        &mut materials,
        camera_entity,
        Vec3::new(-25.0, 60.0, 0.0),
        true,
    );
}

fn create_walker(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    camera_entity: Entity,
    position: Vec3,
    with_camera: bool,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::new(position.x(), 1.0, position.z()),
            ..Default::default()
        })
        .with(walker::Walker::default())
        .with_bundle(if with_camera {
            UnitBundle::new_with_has_camera(camera_entity)
        } else {
            UnitBundle::new(camera_entity)
        });
}

fn create_drone(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    camera_entity: Entity,
    position: Vec3,
    with_camera: bool,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 1.0,
            })),
            material: materials.add(Tailwind::RED400.into()),
            translation: Translation::from(position),
            rotation: Rotation(Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize()),
            ..Default::default()
        })
        .with(drone::Drone::default())
        .with_bundle(if with_camera {
            UnitBundle::new_with_has_camera(camera_entity)
        } else {
            UnitBundle::new(camera_entity)
        });
}

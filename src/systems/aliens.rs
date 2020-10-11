use crate::{
    bundles::*,
    systems::{attack, faction::*, unit::*},
};
use bevy::{math::Vec3, prelude::*};
use bevy_contrib_colors::Tailwind;

struct SpawnTimer(Timer);
fn create_random_aliens(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    resource: Res<AlienMeshResource>,
) {
    timer.0.tick(time.delta_seconds);

    if timer.0.finished {
        let position = Vec3::new(
            50. * time.seconds_since_startup.to_degrees().sin() as f32,
            1.0,
            50. * time.seconds_since_startup.to_degrees().cos() as f32,
        );
        commands
            .spawn(PbrComponents {
                mesh: resource.mesh.unwrap(),
                material: resource.material.unwrap(),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with(Faction::new(Factions::Aliens))
            .with(attack::Ranged::default())
            .with_bundle(UnitBundle {
                target_position: TargetPosition {
                    pos: Some(Vec3::zero()),
                },
                ..UnitBundle::default()
            });
    }
}

#[derive(Default)]
struct AlienMeshResource {
    mesh: Option<Handle<Mesh>>,
    material: Option<Handle<StandardMaterial>>,
}
fn init_alien_mesh_resource(
    mut resource: ResMut<AlienMeshResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    resource.mesh = Some(meshes.add(Mesh::from(shape::Cube { size: 1.0 })));
    resource.material = Some(materials.add(Tailwind::PURPLE400.into()));
}

pub struct AliensPlugin;
impl Plugin for AliensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AlienMeshResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(3.0, true)))
            .add_startup_system(init_alien_mesh_resource.system())
            .add_system(create_random_aliens.system());
    }
}

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
                mesh: resource.mesh.clone(),
                material: resource.material.clone(),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .with_bundle(UnitBundle {
                target_position: TargetPosition {
                    pos: Some(Vec3::zero()),
                },
                faction: Faction::new(Factions::Aliens),
                ..UnitBundle::default()
            })
            .with(attack::Ranged::default());
    }
}

struct AlienMeshResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromResources for AlienMeshResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        AlienMeshResource {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Tailwind::PURPLE400.into()),
        }
    }
}

pub struct AliensPlugin;
impl Plugin for AliensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AlienMeshResource>()
            .add_resource(SpawnTimer(Timer::from_seconds(3.0, true)))
            .add_system(create_random_aliens.system());
    }
}

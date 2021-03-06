use crate::systems::{faction::*, health::Health, time::*, unit::*};
use bevy::{math::Vec3, prelude::*};
use bevy_contrib_colors::Tailwind;

const BULLET_SPEED: f32 = 30.;
// Seconds before the bullet is despawned
const BULLET_LIFETIME: f64 = 10.;

pub struct Bullet {
    pub direction: Vec3,
    pub should_despawn_at: f64,
}
impl Bullet {
    pub fn spawn(
        commands: &mut Commands,
        resource: &BulletMeshResource,
        seconds_since_startup: f64,
        origin: Vec3,
        direction: Vec3,
        faction: Factions,
    ) {
        commands
            .spawn(PbrBundle {
                mesh: resource.mesh.clone(),
                material: resource.material.clone(),
                transform: Transform::from_translation(origin),
                ..Default::default()
            })
            .with(Bullet {
                direction,
                should_despawn_at: seconds_since_startup + BULLET_LIFETIME,
            })
            .with(Faction::new(faction));
    }
}

fn move_bullet(time: Res<ControlledTime>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in query.iter_mut() {
        transform.translation += BULLET_SPEED * bullet.direction * time.delta_seconds;
    }
}

fn kill_after_lifetime_over(
    commands: &mut Commands,
    time: Res<ControlledTime>,
    query: Query<(&Bullet, Entity)>,
) {
    for (bullet, entity) in query.iter() {
        if time.seconds_since_startup >= bullet.should_despawn_at {
            commands.despawn(entity);
        }
    }
}

fn bullet_collision(
    commands: &mut Commands,
    bullet_query: Query<(&Bullet, &Transform, &Faction, Entity)>,
    mut unit_query: Query<(&Unit, &Transform, &mut Health, &Faction)>,
) {
    for (_, bullet_transform, faction, bullet_entity) in bullet_query.iter() {
        let bullet_translation = bullet_transform.translation;

        for (_, enemy_transform, mut health, enemy_faction) in unit_query.iter_mut() {
            // Skip units in same faction
            if enemy_faction.faction == faction.faction {
                continue;
            }

            let enemy_translation = enemy_transform.translation;
            let distance = (bullet_translation - enemy_translation).length();

            if distance < 1.0 {
                health.damage(1);

                commands.despawn(bullet_entity);
            }
        }
    }
}

pub struct BulletMeshResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromResources for BulletMeshResource {
    fn from_resources(resources: &Resources) -> Self {
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        BulletMeshResource {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.3,
            })),
            material: materials.add(Tailwind::BLACK.into()),
        }
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<BulletMeshResource>()
            .add_system(move_bullet)
            .add_system(kill_after_lifetime_over)
            .add_system(bullet_collision);
    }
}

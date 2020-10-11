use crate::systems::{faction::*, health::Health, unit::*};
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
        time: &Time,
        origin: Vec3,
        direction: Vec3,
        faction: Factions,
    ) {
        commands
            .spawn(PbrComponents {
                // TODO We should deal with this being None
                mesh: resource.mesh.unwrap(),
                material: resource.material.unwrap(),
                transform: Transform::from_translation(origin),
                ..Default::default()
            })
            .with(Bullet {
                direction,
                should_despawn_at: time.seconds_since_startup + BULLET_LIFETIME,
            })
            .with(Faction::new(faction));
    }
}

fn move_bullet(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut query.iter() {
        transform.translate(BULLET_SPEED * bullet.direction * time.delta_seconds);
    }
}

fn kill_after_lifetime_over(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Bullet, Entity)>,
) {
    for (bullet, entity) in &mut query.iter() {
        if time.seconds_since_startup >= bullet.should_despawn_at {
            commands.despawn(entity);
        }
    }
}

fn bullet_collision(
    mut commands: Commands,
    mut bullet_query: Query<(&Bullet, &Transform, &Faction, Entity)>,
    mut unit_query: Query<(&Unit, &Transform, &mut Health, &Faction)>,
) {
    for (_, bullet_transform, faction, bullet_entity) in &mut bullet_query.iter() {
        let bullet_translation = bullet_transform.translation();

        for (_, enemy_transform, mut health, enemy_faction) in &mut unit_query.iter() {
            // Skip units in same faction
            if enemy_faction.faction == faction.faction {
                continue;
            }

            let enemy_translation = enemy_transform.translation();
            let distance = (bullet_translation - enemy_translation).length();

            if distance < 1.0 {
                health.health_value -= 1;

                commands.despawn(bullet_entity);
            }
        }
    }
}

#[derive(Default)]
pub struct BulletMeshResource {
    mesh: Option<Handle<Mesh>>,
    material: Option<Handle<StandardMaterial>>,
}

fn init_bullet_mesh_resource(
    mut resource: ResMut<BulletMeshResource>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    resource.mesh = Some(meshes.add(Mesh::from(shape::Icosphere {
        subdivisions: 4,
        radius: 0.3,
    })));
    resource.material = Some(materials.add(Tailwind::BLACK.into()));
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<BulletMeshResource>()
            .add_startup_system(init_bullet_mesh_resource.system())
            .add_system(move_bullet.system())
            .add_system(kill_after_lifetime_over.system())
            .add_system(bullet_collision.system());
    }
}

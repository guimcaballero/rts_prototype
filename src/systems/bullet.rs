use crate::systems::{enemy::*, unit::*};
use bevy::{math::Vec3, prelude::*};
use bevy_contrib_colors::Tailwind;

const BULLET_SPEED: f32 = 30.;

pub struct Bullet {
    pub direction: Vec3,
}
impl Bullet {
    pub fn new(
        commands: &mut Commands,
        resource: &BulletMeshResource,
        origin: Vec3,
        direction: Vec3,
    ) {
        println!("shooting");
        commands
            .spawn(PbrComponents {
                // TODO We should deal with this being None
                mesh: resource.mesh.unwrap(),
                material: resource.material.unwrap(),
                transform: Transform::from_translation(origin),
                ..Default::default()
            })
            .with(Bullet { direction });
    }
}

fn move_bullet(time: Res<Time>, mut query: Query<(&Bullet, &mut Transform)>) {
    for (bullet, mut transform) in &mut query.iter() {
        transform.translate(BULLET_SPEED * bullet.direction * time.delta_seconds);
    }
}

// TODO Make bullet collision function

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
    resource.material = Some(materials.add(Tailwind::RED400.into()));
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<BulletMeshResource>()
            .add_startup_system(init_bullet_mesh_resource.system())
            .add_system(move_bullet.system());
    }
}

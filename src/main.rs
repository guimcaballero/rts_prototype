use bevy::{prelude::*, render::camera::Camera};
use bevy_mod_picking::*;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(PickingPlugin)
        .add_startup_system(setup.system())
        .add_system(fly_camera.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // mesh
        .spawn(PbrComponents {
            // load a mesh from glTF
            mesh: asset_server.load("assets/models/kx139/scene.gltf").unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(0.0, 0.0, 0.0),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 5.0, 4.0),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            translation: Translation::new(0.0, 0.0, 2.0),
            rotation: Rotation::from_rotation_yxz(0.0, 0.0, 0.0),
            ..Default::default()
        });
}

fn fly_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Translation, &mut Rotation)>,
) {
    for (_, mut translation, mut rotation) in &mut query.iter() {
        println!("{}", translation.0);
        if keyboard_input.pressed(KeyCode::W) {
            translation.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, -1.0)) * time.delta_seconds;
        }
        if keyboard_input.pressed(KeyCode::A) {
            translation.0 += rotation.0.mul_vec3(Vec3::new(-1.0, 0.0, 0.0)) * time.delta_seconds;
        }
        if keyboard_input.pressed(KeyCode::S) {
            translation.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 1.0)) * time.delta_seconds;
        }
        if keyboard_input.pressed(KeyCode::D) {
            translation.0 += rotation.0.mul_vec3(Vec3::new(1.0, 0.0, 0.0)) * time.delta_seconds;
        }

        if keyboard_input.pressed(KeyCode::Q) {
            rotation.0 *= Quat::from_rotation_ypr(1.0 * time.delta_seconds, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::E) {
            rotation.0 *= Quat::from_rotation_ypr(-1.0 * time.delta_seconds, 0.0, 0.0);
        }
    }
}

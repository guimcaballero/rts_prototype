use crate::systems::{camera::*, health::*, ui::*};
use bevy::prelude::*;
use rand::Rng;

struct HealthDifferenceNumber {
    should_despawn_at: f64,
}

const TEXT_LIFETIME: f64 = 0.5;
const TEXT_SPEED: f32 = 10.;

fn spawn_health_numbers(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<UiAssetsResource>,
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Texture>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Mutated<Health>, &Transform)>,
) {
    if let Some(font) = fonts.get(assets.font.clone()) {
        let mut rng = rand::thread_rng();

        for (health, transform) in query.iter() {
            let diff = health.difference();

            if diff == 0 {
                continue;
            }

            let text = font.render_text(
                &*format!("{}", diff.abs()),
                if diff > 0 {
                    Color::rgb(0., 0.8, 0.)
                } else {
                    Color::rgb(1., 0., 0.)
                },
                50.,
                100,
                100,
            );
            let text_handle = textures.add(text);

            let position_offset = Vec3::new(
                rng.gen_range(-0.5, 0.5),
                rng.gen_range(1.5, 2.5),
                rng.gen_range(0.5, 1.5),
            );

            let scale = 0.03 + (0.14 - 0.03) * ((diff.abs() as f32 - 1.) / (30. - 1.));

            commands
                .spawn(SpriteComponents {
                    material: color_materials.add(text_handle.into()),

                    sprite: Sprite {
                        size: Vec2::new(1.0, 1.0),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: transform.translation + position_offset,
                        scale: Vec3::new(-scale, scale, scale),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(HealthDifferenceNumber {
                    should_despawn_at: time.seconds_since_startup + TEXT_LIFETIME,
                });
        }
    }
}

fn move_numbers_up_and_rotate(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &HealthDifferenceNumber)>,
    camera_query: Query<(&CameraFollow, &Transform)>,
) {
    let (_camera, camera_transform) = camera_query.iter().next().unwrap();

    for (mut transform, _) in query.iter_mut() {
        transform.look_at(camera_transform.translation, Vec3::unit_y());
        transform.translation += Vec3::unit_y() * time.delta_seconds * TEXT_SPEED;
    }
}

fn despawn_numbers(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &HealthDifferenceNumber)>,
) {
    for (entity, number) in &mut query.iter() {
        if time.seconds_since_startup >= number.should_despawn_at {
            commands.despawn(entity);
        }
    }
}

pub struct HealthNumbersPlugin;
impl Plugin for HealthNumbersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(spawn_health_numbers.system())
            .add_system(move_numbers_up_and_rotate.system())
            .add_system(despawn_numbers.system());
    }
}

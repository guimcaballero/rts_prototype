use crate::systems::{camera::*, health::*, time::*, ui::*};
use ab_glyph::{self, FontVec, Glyph, InvalidFont, OutlinedGlyph, Point, PxScale, ScaleFont};
use bevy::prelude::*;
use bevy::render::texture::TextureFormat;
use rand::Rng;

struct HealthDifferenceNumber {
    should_despawn_at: f64,
}

const TEXT_LIFETIME: f64 = 0.5;
const TEXT_SPEED: f32 = 10.;

// TODO Move to another file
fn render_text(
    font: &Font,
    text: &str,
    color: Color,
    font_size: f32,
    width: usize,
    height: usize,
) -> Texture {
    let scale = ab_glyph::PxScale::from(font_size);

    let scaled_font = ab_glyph::Font::as_scaled(&font.font, scale);

    let mut glyphs = Vec::new();
    layout_paragraph(
        scaled_font,
        ab_glyph::point(0.0, 0.0),
        width as f32,
        text,
        &mut glyphs,
    );

    let color_u8 = [
        (color.r() * 255.0) as u8,
        (color.g() * 255.0) as u8,
        (color.b() * 255.0) as u8,
    ];

    // TODO: this offset is a bit hackey
    let mut alpha = vec![0.0; width * height];
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                // Turn the coverage into an alpha value (blended with any previous)
                let offset_x = x as usize + bounds.min.x as usize;
                let offset_y = y as usize + bounds.min.y as usize;
                if offset_x >= width || offset_y >= height {
                    return;
                }
                alpha[offset_y * width + offset_x] = v;
            });
        }
    }

    Texture::new(
        Vec2::new(width as f32, height as f32),
        alpha
            .iter()
            .map(|a| {
                vec![
                    color_u8[0],
                    color_u8[1],
                    color_u8[2],
                    (color.a() * a * 255.0) as u8,
                ]
            })
            .flatten()
            .collect::<Vec<u8>>(),
        TextureFormat::Rgba8UnormSrgb,
    )
}

// TODO Move to another file
fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: ab_glyph::Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + ab_glyph::point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = ab_glyph::point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = ab_glyph::point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}

fn spawn_health_numbers(
    commands: &mut Commands,
    time: Res<ControlledTime>,
    assets: Res<UiAssetsResource>,
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Texture>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Health, &Transform), Mutated<Health>>,
) {
    if let Some(font) = fonts.get(assets.font.clone()) {
        let mut rng = rand::thread_rng();

        for (health, transform) in query.iter() {
            let diff = health.difference();

            if diff == 0 {
                continue;
            }

            let text = render_text(
                font,
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
                .spawn(SpriteBundle {
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
    time: Res<ControlledTime>,
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
    commands: &mut Commands,
    time: Res<ControlledTime>,
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
        app.add_system(spawn_health_numbers)
            .add_system(move_numbers_up_and_rotate)
            .add_system(despawn_numbers);
    }
}

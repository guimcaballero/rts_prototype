use crate::systems::ability::*;
use bevy::prelude::*;
use bevy_mod_picking::*;

struct DebugCursor;
/// Updates the 3d cursor to be in the pointed world coordinates
fn update_debug_cursor_position(
    pick_state: ResMut<PickState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&DebugCursor, &mut Transform, &mut Draw)>,
) {
    // Set the cursor translation to the top pick's world coordinates
    if let Some(top_pick) = pick_state.top(PickGroup::default()) {
        let pos = *top_pick.position();

        for (_, mut transform, mut draw) in &mut query.iter() {
            if keyboard_input.pressed(KeyCode::P) {
                transform.translation = pos;
                draw.is_visible = true;
            } else {
                draw.is_visible = false;
            }
        }
    }
}
/// Start up system to create 3d Debug cursor
fn setup_debug_cursor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        // cursor
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.1,
            })),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(1.5, 1.5, 1.5)),
            ..Default::default()
        })
        .with(DebugCursor);
}

fn ability_debug(ability: Res<CurrentAbility>) {
    println!("Current ability: {:?}", ability.ability);
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_debug_cursor.system())
            .add_system(update_debug_cursor_position.system())
            .add_system(ability_debug.system());
    }
}

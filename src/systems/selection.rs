use crate::systems::unit::Unit;
use bevy::prelude::*;
use bevy_contrib_colors::*;
use bevy_mod_picking::*;

/// Selects a single unit
fn select_single_unit(
    pick_state: Res<PickState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<&mut Unit>,
) {
    // Only run when control is not pressed and we just clicked the left button
    if keyboard_input.pressed(KeyCode::LControl)
        || !mouse_button_inputs.just_pressed(MouseButton::Left)
    {
        return;
    }

    // Deselect all units
    for mut unit in &mut query.iter() {
        unit.selected = false;
    }

    // Select the top pick
    if let Some(top_pick) = pick_state.top() {
        let entity = top_pick.get_entity();
        if let Ok(mut unit) = query.entity(entity) {
            if let Some(mut unit) = unit.get() {
                unit.selected = true;
            }
        }
    }
}

/// Selects multiple units
fn select_multiple_units(
    pick_state: Res<PickState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    query: Query<&mut Unit>,
) {
    // Only run when control is pressed and we just clicked the left button
    if !keyboard_input.pressed(KeyCode::LControl)
        || !mouse_button_inputs.just_pressed(MouseButton::Left)
    {
        return;
    }

    // Select the top pick
    if let Some(top_pick) = pick_state.top() {
        let entity = top_pick.get_entity();
        if let Ok(mut unit) = query.entity(entity) {
            if let Some(mut unit) = unit.get() {
                unit.selected = true;
            }
        }
    }
}

/// Changes the color of a unit depending on it's selection status
fn change_color_for_highlighted_units(
    pick_state: Res<PickState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Unit, &Handle<StandardMaterial>, Entity)>,
) {
    for (unit, material_handle, entity) in &mut query.iter() {
        let current_color = &mut materials.get_mut(material_handle).unwrap().albedo;

        // Strong blue if selected, red if not
        *current_color = if unit.selected {
            Tailwind::BLUE600
        } else {
            Tailwind::RED400
        };

        // If the mouse is over it, light blue
        if let Some(top_pick) = pick_state.top() {
            let top_entity = top_pick.get_entity();

            if entity == top_entity {
                *current_color = Tailwind::BLUE300;
            }
        }
    }
}

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(select_single_unit.system())
            .add_system(select_multiple_units.system())
            .add_system(change_color_for_highlighted_units.system());
    }
}

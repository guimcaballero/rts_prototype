use crate::helpers::shapes::*;
use crate::systems::unit::TargetPosition;
use bevy::prelude::*;
use bevy_contrib_colors::*;
use bevy_mod_picking::*;

#[derive(Default)]
pub struct Selectable {
    pub selected: bool,
}

/// Selects units
fn select_units(
    pick_state: Res<PickState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<&mut Selectable>,
) {
    // Only run when control is not pressed and we just clicked the left button
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if !keyboard_input.pressed(KeyCode::LControl) {
        // Deselect all units
        for mut selectable in &mut query.iter() {
            selectable.selected = false;
        }
    }

    // Select the top pick
    if let Some(top_pick) = pick_state.top(PickGroup::default()) {
        let entity = top_pick.entity();
        if let Ok(mut selectable) = query.entity(entity) {
            if let Some(mut unit) = selectable.get() {
                unit.selected = true;
            }
        }
    }
}

struct SelectionState {
    initial_position: Option<Vec3>,
}
impl Default for SelectionState {
    fn default() -> Self {
        Self {
            initial_position: None,
        }
    }
}

pub struct DragSelectionRectangle;

fn drag_select(
    mut selection_state: ResMut<SelectionState>,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Selectable, &Transform)>,
    mut drag_selection_rectangle: Query<(&Handle<Mesh>, &DragSelectionRectangle, &mut Draw)>,
) {
    // If we start clicking, save the initial_position
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        if let Some(top_pick) = pick_state.top(PickGroup::default()) {
            let pos = top_pick.position();
            selection_state.initial_position = Some(*pos);
        } else {
            // if there is no top pick, set it to none
            selection_state.initial_position = None;
        }
    } else if !mouse_button_inputs.pressed(MouseButton::Left) {
        // If the mouse is not pressed, we're not dragging
        selection_state.initial_position = None;
        for (_, _, mut draw) in &mut drag_selection_rectangle.iter() {
            draw.is_visible = false;
        }
    }

    // If initial_pos is a Some, it means we just finished dragging
    if let Some(initial_position) = selection_state.initial_position {
        if let Some(top_pick) = pick_state.top(PickGroup::default()) {
            let final_position = *top_pick.position();

            // Fix for clicking
            if (final_position - initial_position).length() < 0.1 {
                return;
            }

            // Modify the drag rectangle
            for (mesh_handle, _, mut draw) in &mut drag_selection_rectangle.iter() {
                draw.is_visible = true;
                let mesh_option = meshes.get_mut(mesh_handle);
                if let Some(mut mesh) = mesh_option {
                    mesh.attributes = rectangle_attributes(initial_position, final_position);
                }
            }

            // Select the units
            for (mut selectable, transform) in &mut query.iter() {
                // Mark the units as selected if they are inside the rectangle
                selectable.selected = is_between_two_values(
                    transform.translation().x(),
                    initial_position.x(),
                    final_position.x(),
                ) && is_between_two_values(
                    transform.translation().z(),
                    initial_position.z(),
                    final_position.z(),
                );
            }
        }
    }
}

fn is_between_two_values(x: f32, a: f32, b: f32) -> bool {
    return (a < x && x < b) || (b < x && x < a);
}

/// Changes the color of a unit depending on it's selection status
fn change_color_for_highlighted_units(
    pick_state: Res<PickState>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Selectable, &Handle<StandardMaterial>, Entity)>,
) {
    for (selectable, material_handle, entity) in &mut query.iter() {
        let current_color = &mut materials.get_mut(material_handle).unwrap().albedo;

        // Strong blue if selected, red if not
        *current_color = if selectable.selected {
            Tailwind::BLUE600
        } else {
            Tailwind::RED400
        };

        // If the mouse is over it, light blue
        if let Some(top_pick) = pick_state.top(PickGroup::default()) {
            let top_entity = top_pick.entity();

            if entity == top_entity {
                *current_color = Tailwind::BLUE300;
            }
        }
    }
}

fn set_target_for_selected(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(&Selectable, &mut TargetPosition)>,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        // Get the world position
        if let Some(top_pick) = pick_state.top(PickGroup::default()) {
            let pos = top_pick.position();

            for (selectable, mut target) in &mut query.iter() {
                if selectable.selected {
                    target.update_to_vec(pos);
                }
            }
        }
    }
}

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectionState>()
            .add_system(select_units.system())
            .add_system(drag_select.system())
            .add_system(set_target_for_selected.system())
            .add_system(change_color_for_highlighted_units.system());
    }
}

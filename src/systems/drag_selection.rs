use crate::helpers::shapes::*;
use crate::systems::{ability::*, health::*, selection::*, unit::*};
use bevy::prelude::*;
use bevy_mod_picking::*;

pub struct SelectionState {
    initial_position: Option<Vec3>,
    current_position: Option<Vec3>,

    last_rectangle: Option<(Vec3, Vec3)>,
}
impl Default for SelectionState {
    fn default() -> Self {
        Self {
            initial_position: None,
            current_position: None,

            last_rectangle: None,
        }
    }
}

struct DragSelectionRectangle;
fn create_drag_rectangle(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    // Drag Selection rectangle
    commands
        .spawn(SpriteBundle {
            material: color_materials.add(Color::rgba(0.0, 0.0, 0.8, 0.1).into()),
            mesh: meshes.add(rectangle_mesh()),
            sprite: Sprite {
                size: Vec2::new(1.0, 1.0),
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.05, 0.0)),
            ..Default::default()
        })
        .with(DragSelectionRectangle);
}

fn start_drag_select(
    ability: Res<CurrentAbility>,
    mut selection_state: ResMut<SelectionState>,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
) {
    if ability.ability != Ability::Select && ability.ability != Ability::HealArea {
        return;
    }

    // If we start clicking, save the initial_position
    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        if let Some((_top_entity, intersection)) = pick_state.top(Group::default()) {
            let pos = intersection.position();
            selection_state.initial_position = Some(*pos);
        } else {
            // if there is no top pick, set it to none
            selection_state.initial_position = None;
            selection_state.current_position = None;
        }
    }
}

fn finish_drag_select(
    ability: Res<CurrentAbility>,
    mut selection_state: ResMut<SelectionState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
) {
    if ability.ability != Ability::Select && ability.ability != Ability::HealArea {
        return;
    }

    // If we release the button, save the rectangle and reset the values
    if mouse_button_inputs.just_released(MouseButton::Left) {
        if ability.ability == Ability::HealArea {
            if let Some(initial_position) = selection_state.initial_position {
                if let Some(current_position) = selection_state.current_position {
                    selection_state.last_rectangle = Some((initial_position, current_position));
                }
            }
        }

        selection_state.initial_position = None;
        selection_state.current_position = None;
    }
}

fn drag_select(
    ability: Res<CurrentAbility>,
    mut selection_state: ResMut<SelectionState>,
    pick_state: Res<PickState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut drag_selection_rectangle: Query<(&Handle<Mesh>, &DragSelectionRectangle, &mut Draw)>,
) {
    if ability.ability != Ability::Select && ability.ability != Ability::HealArea {
        return;
    }

    if let Some(initial_position) = selection_state.initial_position {
        if let Some((_top_entity, intersection)) = pick_state.top(Group::default()) {
            let current_position = *intersection.position();

            // Fix for clicking
            if (current_position - initial_position).length() < 0.1 {
                return;
            }

            // Modify the drag rectangle
            for (mesh_handle, _, mut draw) in drag_selection_rectangle.iter_mut() {
                draw.is_visible = true;
                let mesh_option = meshes.get_mut(mesh_handle);
                if let Some(mut mesh) = mesh_option {
                    set_rectangle_attributes(&mut mesh, initial_position, current_position);
                }
            }

            selection_state.current_position = Some(current_position);
        }
    } else {
        for (_, _, mut draw) in drag_selection_rectangle.iter_mut() {
            draw.is_visible = false;
        }
    }
}

fn is_between_two_values(x: f32, a: f32, b: f32) -> bool {
    (a < x && x < b) || (b < x && x < a)
}

fn select_inside_rectangle(
    selection_state: Res<SelectionState>,
    ability: Res<CurrentAbility>,
    mut query: Query<(&mut Selectable, &Transform)>,
) {
    if ability.ability != Ability::Select {
        return;
    }

    if let Some(initial_position) = selection_state.initial_position {
        if let Some(current_position) = selection_state.current_position {
            // Select the units
            for (mut selectable, transform) in query.iter_mut() {
                // Mark the units as selected if they are inside the rectangle
                selectable.set_selected(
                    is_between_two_values(
                        transform.translation.x,
                        initial_position.x,
                        current_position.x,
                    ) && is_between_two_values(
                        transform.translation.z,
                        initial_position.z,
                        current_position.z,
                    ),
                );
            }
        }
    }
}

fn heal_area_ability(
    mut selection_state: ResMut<SelectionState>,
    mut ability: ResMut<CurrentAbility>,
    mut query: Query<(&mut Health, &Unit, &Transform)>,
) {
    if ability.ability != Ability::HealArea {
        return;
    }

    if let Some((beginning, end)) = selection_state.last_rectangle {
        for (mut health, _unit, transform) in query.iter_mut() {
            // Heal the units inside the rectangle
            if is_between_two_values(transform.translation.x, beginning.x, end.x)
                && is_between_two_values(transform.translation.z, beginning.z, end.z)
            {
                health.heal(3);
            }
        }

        selection_state.last_rectangle = None;
        ability.ability = Ability::Select;
    }
}

pub struct DragSelectionPlugin;
impl Plugin for DragSelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectionState>()
            .add_system(start_drag_select)
            .add_system(finish_drag_select)
            .add_system(drag_select)
            .add_system(select_inside_rectangle)
            .add_startup_system(create_drag_rectangle)
            .add_system(heal_area_ability);
    }
}

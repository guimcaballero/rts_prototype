use crate::helpers::shapes::*;
use crate::systems::{ability::*, unit::TargetPosition};
use bevy::prelude::*;
use bevy_contrib_colors::*;
use bevy_mod_picking::*;

pub struct Selectable {
    pub selected: bool,
    pub previously_selected: bool,
    pub circle: Entity,
    pub entity: Entity,
}

impl Selectable {
    pub fn set_selected(&mut self, selected: bool) {
        self.previously_selected = self.selected;
        self.selected = selected;
    }
}

#[derive(Default)]
pub struct SelectableBuilder;
fn selectable_builder(
    mut commands: Commands,
    resource: Res<SelectionCircleMaterial>,
    mut query: Query<(Entity, &SelectableBuilder)>,
) {
    for (entity, _) in &mut query.iter() {
        let circle = commands
            .spawn(SpriteComponents {
                material: resource.circle_material.clone(),
                mesh: resource.circle_mesh.clone(),
                sprite: Sprite {
                    size: Vec2::new(1.0, 1.0),
                    ..Default::default()
                },
                draw: Draw {
                    is_visible: false,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.1, 0.0),
                    scale: Vec3::splat(0.03),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SelectionCircle)
            .current_entity()
            .unwrap();

        commands.insert_one(
            entity,
            Selectable {
                selected: false,
                previously_selected: false,
                circle,
                entity,
            },
        );
        commands.remove_one::<SelectableBuilder>(entity);
    }
}

/// Selects units
fn select_units(
    ability: Res<CurrentAbility>,
    pick_state: Res<PickState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<&mut Selectable>,
) {
    if ability.ability != Ability::Select {
        return;
    }

    // Only run when control is not pressed and we just clicked the left button
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(top_pick) = pick_state.top(PickGroup::default()) {
        if !keyboard_input.pressed(KeyCode::LControl) {
            // Deselect all units
            for mut selectable in &mut query.iter() {
                selectable.set_selected(false);
            }
        }

        // Select the top pick
        let entity = top_pick.entity();
        if let Ok(mut selectable) = query.entity(entity) {
            if let Some(mut unit) = selectable.get() {
                unit.set_selected(true);
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

struct DragSelectionRectangle;
fn drag_select(
    ability: Res<CurrentAbility>,
    mut selection_state: ResMut<SelectionState>,
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&mut Selectable, &Transform)>,
    mut drag_selection_rectangle: Query<(&Handle<Mesh>, &DragSelectionRectangle, &mut Draw)>,
) {
    if ability.ability != Ability::Select {
        return;
    }

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
                selectable.set_selected(
                    is_between_two_values(
                        transform.translation.x(),
                        initial_position.x(),
                        final_position.x(),
                    ) && is_between_two_values(
                        transform.translation.z(),
                        initial_position.z(),
                        final_position.z(),
                    ),
                );
            }
        }
    }
}
fn is_between_two_values(x: f32, a: f32, b: f32) -> bool {
    (a < x && x < b) || (b < x && x < a)
}

fn create_drag_rectangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    // Drag Selection rectangle
    commands
        .spawn(SpriteComponents {
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

struct SelectionCircle;
fn move_circle_for_selected_units(
    pick_state: Res<PickState>,
    resource: Res<SelectionCircleMaterial>,
    mut query: Query<(&Selectable, &Transform, Entity)>,
    circle_query: Query<(
        &SelectionCircle,
        &mut Draw,
        &mut Transform,
        &mut Handle<ColorMaterial>,
    )>,
) {
    for (selectable, transform, entity) in &mut query.iter() {
        let mut is_hovered = false;

        if let Some(top_pick) = pick_state.top(PickGroup::default()) {
            let top_entity = top_pick.entity();

            if entity == top_entity {
                is_hovered = true;
            }
        }

        let mut draw = match circle_query.get_mut::<Draw>(selectable.circle) {
            Ok(draw) => draw,
            _ => continue,
        };
        let mut circle_transform = match circle_query.get_mut::<Transform>(selectable.circle) {
            Ok(transform) => transform,
            _ => continue,
        };
        let mut material_handle =
            match circle_query.get_mut::<Handle<ColorMaterial>>(selectable.circle) {
                Ok(material) => material,
                _ => continue,
            };

        if is_hovered || selectable.selected {
            draw.is_visible = true;
            let translation = transform.translation;
            circle_transform.translation = Vec3::new(translation.x(), 0.1, translation.z());

            *material_handle = if is_hovered {
                resource.hover_material.clone()
            } else {
                resource.selected_material.clone()
            };
        } else {
            draw.is_visible = false;
        }
    }
}

struct SelectionCircleMaterial {
    circle_mesh: Handle<Mesh>,
    circle_material: Handle<ColorMaterial>,
    selected_material: Handle<ColorMaterial>,
    hover_material: Handle<ColorMaterial>,
}
impl FromResources for SelectionCircleMaterial {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        let mut meshes = resources.get_mut::<Assets<Mesh>>().unwrap();
        SelectionCircleMaterial {
            circle_mesh: meshes.add(circle_mesh()),
            circle_material: materials.add(Tailwind::BLUE500.into()),
            selected_material: materials.add(Tailwind::BLUE500.into()),
            hover_material: materials.add(Tailwind::BLUE300.into()),
        }
    }
}

fn set_target_for_selected(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    ability: Res<CurrentAbility>,
    mut query: Query<(&Selectable, &mut TargetPosition)>,
) {
    if ability.ability != Ability::Select {
        return;
    }

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
            .init_resource::<SelectionCircleMaterial>()
            .add_system(selectable_builder.system())
            .add_system(select_units.system())
            .add_system(drag_select.system())
            .add_startup_system(create_drag_rectangle.system())
            .add_system(set_target_for_selected.system())
            .add_system(move_circle_for_selected_units.system());
    }
}

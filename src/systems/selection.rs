use crate::systems::{
    ability::*,
    selection_circle::*,
    unit::{TargetPosition, UnitSize},
};
use bevy::prelude::*;
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
    commands: &mut Commands,
    resource: Res<SelectionCircleMaterial>,
    query: Query<(Entity, &SelectableBuilder, &UnitSize)>,
) {
    for (entity, _, size) in &mut query.iter() {
        let circle = commands
            .spawn(SpriteBundle {
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
                    scale: Vec3::splat(0.03 * size.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(SelectionCircle::default())
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

    if let Some((top_entity, _intersection)) = pick_state.top(Group::default()) {
        if !keyboard_input.pressed(KeyCode::LControl) {
            // Deselect all units
            for mut selectable in query.iter_mut() {
                selectable.set_selected(false);
            }
        }

        // Select the top pick
        if let Ok(mut selectable) = query.get_mut(*top_entity) {
            selectable.set_selected(true);
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
        if let Some((_top_entity, intersection)) = pick_state.top(Group::default()) {
            let pos = intersection.position();

            for (selectable, mut target) in query.iter_mut() {
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
        app.add_system(selectable_builder)
            .add_system(select_units)
            .add_system(set_target_for_selected);
    }
}

use crate::systems::{health::*, selection::*, ui::*};
use crate::unit::Unit;
use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(PartialEq, Debug)]
pub enum Ability {
    Select,
    SwitchCamera,
    SwitchBack,
    Teleport(Entity),
    HealUnit,
    HealArea,
}

pub struct CurrentAbility {
    pub ability: Ability,
}
impl Default for CurrentAbility {
    fn default() -> Self {
        Self {
            ability: Ability::Select,
        }
    }
}

#[derive(Default)]
pub struct UnitAbilities {
    pub abilities: Vec<AbilityButton>,
}
pub struct AbilityButton {
    pub name: String,
    pub id: &'static str,
    pub callback: AbilityChangeCallback,
}

fn add_ability_buttons_for_selected_units(
    mut buttons: ResMut<AvailableButtons>,
    query: Query<(&Selectable, &UnitAbilities, Entity), Mutated<Selectable>>,
) {
    for (selectable, abilities, entity) in &mut query.iter() {
        if selectable.selected && !selectable.previously_selected {
            for ability in &abilities.abilities {
                let _ = buttons.add_button((
                    ability.name.clone(),
                    format!("{}-{:?}", ability.id, entity),
                    ability.callback,
                    CallbackData {
                        entity: Some(entity),
                        associated_circle: Some(selectable.circle),
                    },
                ));
            }
        } else if !selectable.selected {
            for ability in &abilities.abilities {
                let _ = buttons.remove_button(format!("{}-{:?}", ability.id, entity));
            }
        }
    }
}

fn teleport_ability(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut ability: ResMut<CurrentAbility>,
    mut query: Query<(&mut Transform, &Unit)>,
) {
    if let Ability::Teleport(entity) = ability.ability {
        if mouse_button_inputs.just_pressed(MouseButton::Right) {
            // Get the world position
            if let Some((_top_entity, intersection)) = pick_state.top(Group::default()) {
                let mut pos = *intersection.position();
                pos.y = 1.;

                if let Ok(mut transform) = query.get_component_mut::<Transform>(entity) {
                    transform.translation = pos;
                }
            }

            ability.ability = Ability::Select;
        }
    }
}

fn heal_unit_ability(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut ability: ResMut<CurrentAbility>,
    mut query: Query<(&mut Health, &Unit)>,
) {
    if Ability::HealUnit != ability.ability {
        return;
    }

    if mouse_button_inputs.just_pressed(MouseButton::Left) {
        // Get the world position
        if let Some((top_entity, _intersection)) = pick_state.top(Group::default()) {
            if let Ok(mut health) = query.get_component_mut::<Health>(*top_entity) {
                health.heal(20);
            }
            ability.ability = Ability::Select;
        }
    }
}

pub struct AbilityPlugin;
impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CurrentAbility>()
            .add_system(add_ability_buttons_for_selected_units.system())
            .add_system(teleport_ability.system())
            .add_system(heal_unit_ability.system());
    }
}

use std::fmt;
impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ability::Select => write!(f, "Select"),
            Ability::SwitchCamera => write!(f, "Switch Camera"),
            Ability::SwitchBack => write!(f, "Switch Back"),
            Ability::Teleport(_) => write!(f, "Teleport"),
            Ability::HealUnit => write!(f, "Heal unit"),
            Ability::HealArea => write!(f, "Heal area"),
        }
    }
}

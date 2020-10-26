use crate::systems::{selection::*, ui::*};
use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub enum Ability {
    Select,
    SwitchCamera,
    SwitchBack,
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
    mut query: Query<(Mutated<Selectable>, &UnitAbilities, Entity)>,
) {
    for (selectable, abilities, entity) in &mut query.iter() {
        if selectable.selected && !selectable.previously_selected {
            for ability in &abilities.abilities {
                let _ = buttons.add_button((
                    ability.name.clone(),
                    format!("{}{:?}", ability.id.clone(), entity),
                    ability.callback.clone(),
                    CallbackData {
                        entity: Some(entity),
                    },
                ));
            }
        } else if !selectable.selected {
            for ability in &abilities.abilities {
                let _ = buttons.remove_button(format!("{}{:?}", ability.id.clone(), entity));
            }
        }
    }
}

pub struct AbilityPlugin;
impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CurrentAbility>()
            .add_system(add_ability_buttons_for_selected_units.system());
    }
}

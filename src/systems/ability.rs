use crate::systems::{health::*, selection::*, ui::*};
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

pub struct UnitAbilities {
    abilities: Vec<ButtonTuple>,
}

fn add_ability_buttons_for_selected_units(
    mut buttons: ResMut<AvailableButtons>,
    mut query: Query<(Mutated<Selectable>, Entity)>,
) {
    for (selectable, entity) in &mut query.iter() {
        if selectable.selected && !selectable.previously_selected {
            // TODO Add the abilities as buttons here
            let _ = buttons.add_button((
                "Kill unit".to_string(),
                format!("button-{:?}", entity),
                |mut commands, _, mut buttons, callback_data| {
                    buttons.remove_button(format!("button-{:?}", callback_data.entity.unwrap()));
                    commands.insert_one(callback_data.entity.unwrap(), Dead {});
                },
                CallbackData {
                    entity: Some(entity),
                },
            ));
        } else if selectable.selected {
            // Don't remove the buttons and don't add them again
        } else {
            let _ = buttons.remove_button(format!("button-{:?}", entity));
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

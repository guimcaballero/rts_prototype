use crate::systems::ui::ButtonTuple;
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

pub struct AbilityPlugin;
impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CurrentAbility>();
    }
}

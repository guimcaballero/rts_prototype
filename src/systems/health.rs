use crate::systems::{ability::*, selection::Selectable};
use crate::ui::*;
use bevy::prelude::*;

pub struct Health {
    pub value: i16,
    max_health: i16,
    previous_value: i16, // Used for the displayed number
}

impl Default for Health {
    fn default() -> Self {
        Self {
            value: 3,
            max_health: 3,
            previous_value: 3,
        }
    }
}

impl Health {
    pub fn new(value: i16) -> Self {
        Self {
            value,
            max_health: value,
            previous_value: value,
        }
    }

    pub fn damage(&mut self, value: i16) {
        self.previous_value = self.value;
        self.value = (self.value - value).min(self.max_health);
    }

    pub fn heal(&mut self, value: i16) {
        self.previous_value = self.value;
        self.value = (self.value + value).min(self.max_health);
    }

    pub fn difference(&self) -> i16 {
        self.value - self.previous_value
    }
}

fn kill_if_health_0(commands: &mut Commands, query: Query<(&Health, Entity), Mutated<Health>>) {
    for (health, entity) in &mut query.iter() {
        if health.value <= 0 {
            commands.insert_one(entity, Dead {});
        }
    }
}

pub struct Dead;
fn remove_if_dead(
    commands: &mut Commands,
    mut buttons: ResMut<AvailableButtons>,
    query: Query<(&Dead, Entity, Option<&Selectable>, Option<&UnitAbilities>)>,
) {
    for (_dead, entity, option_selectable, option_abilities) in query.iter() {
        // If it's a selectable, despawn it's circle too
        if let Some(selectable) = option_selectable {
            commands.despawn(selectable.circle);
        }

        if let Some(abilities) = option_abilities {
            for ability in &abilities.abilities {
                let _ = buttons.remove_button(format!("{}{:?}", ability.id, entity));
            }
        }

        commands.despawn(entity);
    }
}

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(kill_if_health_0).add_system(remove_if_dead);
    }
}

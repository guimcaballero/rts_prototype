use crate::systems::selection::Selectable;
use bevy::prelude::*;

pub struct Health {
    pub health_value: i16,
}

impl Default for Health {
    fn default() -> Self {
        Self { health_value: 3 }
    }
}

fn remove_if_dead(
    mut commands: Commands,
    mut query: Query<(Mutated<Health>, Entity, Option<&mut Selectable>)>,
) {
    for (health, entity, option_selectable) in &mut query.iter() {
        if health.health_value <= 0 {
            // If it's a selectable, despawn it's circle too
            if let Some(selectable) = option_selectable {
                commands.despawn(selectable.circle);
            }
            commands.despawn(entity);
        }
    }
}

pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(remove_if_dead.system());
    }
}

use bevy::prelude::*;

pub struct Health {
    pub health_value: i16,
}

impl Default for Health {
    fn default() -> Self {
        Self { health_value: 1 }
    }
}

fn remove_if_dead(mut commands: Commands, mut query: Query<(Mutated<Health>, Entity)>) {
    for (health, entity) in &mut query.iter() {
        if health.health_value <= 0 {
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

use crate::systems::unit::TargetPosition;
use bevy::{math::Vec3, prelude::*};

#[derive(Default)]
pub struct Enemy {}

fn set_dummy_target(mut query: Query<(Added<Enemy>, &mut TargetPosition)>) {
    // TODO This will eventually be replaced by something that isn't dumb that decides where to go
    for (_, mut target) in &mut query.iter() {
        target.update_to_vec(&Vec3::zero());
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(set_dummy_target.system());
    }
}

use crate::systems::{faction::*, unit::TargetPosition};
use bevy::{math::Vec3, prelude::*};

fn set_dummy_target_for_aliens(mut query: Query<(Added<Faction>, &mut TargetPosition)>) {
    // TODO This will eventually be replaced by something that isn't dumb that decides where to go
    for (faction, mut target) in &mut query.iter() {
        if faction.faction == Factions::Aliens {
            target.update_to_vec(&Vec3::zero());
        }
    }
}

pub struct AliensPlugin;
impl Plugin for AliensPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(set_dummy_target_for_aliens.system());
    }
}

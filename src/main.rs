use crate::systems::movement;
use bevy::prelude::*;
use bevy_mod_picking::*;

mod initialize;
mod systems;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "bavy".to_string(),
            width: 1600,
            height: 1600,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(PickingPlugin)
        .add_startup_system(initialize::setup.system())
        .add_system(movement::wasd_movement.system())
        .add_system(movement::move_to_target.system())
        .add_system(movement::set_target_for_selected.system())
        //.add_system(get_picks.system())
        .run();
}

fn get_picks(pick_state: ResMut<PickState>) {
    println!("All entities:\n{:?}", pick_state.list());
    println!("Top entity:\n{:?}", pick_state.top());
}

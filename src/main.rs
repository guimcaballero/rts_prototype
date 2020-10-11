#[allow(unused_imports)]
use crate::systems::{
    aliens, attack, axes, bullet, camera, debug, drone, health, selection, target_indicator, unit,
    walker,
};
use bevy::prelude::*;
use bevy_mod_picking::*;

mod bundles;
mod helpers;
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
        // .add_plugin(debug::DebugPlugin)
        .add_plugin(PickingPlugin)
        .add_startup_system(initialize::setup.system())
        .add_plugin(drone::DronePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(unit::UnitPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(walker::WalkerPlugin)
        .add_plugin(aliens::AliensPlugin)
        .add_plugin(axes::AxesPlugin)
        .add_plugin(target_indicator::TargetIndicatorPlugin)
        .add_plugin(attack::AttackPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(health::HealthPlugin)
        .run();
}

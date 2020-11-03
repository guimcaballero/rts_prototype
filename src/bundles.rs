use crate::systems::{
    ability::UnitAbilities,
    drone::Drone,
    faction::Faction,
    health::Health,
    unit::{TargetPosition, Unit, UnitSize},
    walker::Walker,
};
use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub unit: Unit,
    pub health: Health,
    pub target_position: TargetPosition,
    pub pickable_mesh: PickableMesh,
    pub faction: Faction,
    pub abilities: UnitAbilities,
    pub size: UnitSize,
}

#[derive(Bundle, Default)]
pub struct DroneBundle {
    pub drone: Drone,
}

#[derive(Bundle, Default)]
pub struct WalkerBundle {
    pub walker: Walker,
}

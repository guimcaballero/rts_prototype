use crate::systems::{
    health::Health,
    unit::{TargetPosition, Unit},
};
use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub unit: Unit,
    pub health: Health,
    pub target_position: TargetPosition,
    pub pickable_mesh: PickableMesh,
}

impl UnitBundle {
    pub fn new() -> Self {
        Self {
            unit: Unit::default(),
            health: Health::default(),
            target_position: TargetPosition::default(),
            pickable_mesh: PickableMesh::default(),
        }
    }
}

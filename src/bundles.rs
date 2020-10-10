use crate::systems::{
    camera::CanHaveCamera,
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
    pub can_have_camera: CanHaveCamera,
}

impl UnitBundle {
    pub fn new(camera_entity: Option<Entity>) -> Self {
        Self {
            unit: Unit::default(),
            health: Health::default(),
            target_position: TargetPosition::default(),
            pickable_mesh: PickableMesh::default(),
            can_have_camera: if let Some(entity) = camera_entity {
                CanHaveCamera::new_with_camera(entity)
            } else {
                CanHaveCamera::default()
            },
        }
    }
}

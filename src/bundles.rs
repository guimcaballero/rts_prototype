use crate::systems::{
    camera::CanHaveCamera,
    unit::{TargetPosition, Unit},
};
use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    target_position: TargetPosition,
    pickable_mesh: PickableMesh,
    can_have_camera: CanHaveCamera,
}

impl UnitBundle {
    pub fn new(camera_entity: Option<Entity>) -> Self {
        Self {
            unit: Unit::new(),
            target_position: TargetPosition::new(),
            pickable_mesh: PickableMesh::default(),
            can_have_camera: if let Some(entity) = camera_entity {
                CanHaveCamera::new_with_camera(entity)
            } else {
                CanHaveCamera::new()
            },
        }
    }
}

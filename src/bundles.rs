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
    pub fn new(camera_entity: Entity) -> Self {
        Self {
            unit: Unit::new(),
            target_position: TargetPosition::new(),
            pickable_mesh: PickableMesh::new(camera_entity),
            can_have_camera: CanHaveCamera::new(),
        }
    }
    pub fn new_with_has_camera(camera_entity: Entity) -> Self {
        Self {
            unit: Unit::new(),
            target_position: TargetPosition::new(),
            pickable_mesh: PickableMesh::new(camera_entity),
            can_have_camera: CanHaveCamera::new_with_camera(camera_entity),
        }
    }
}
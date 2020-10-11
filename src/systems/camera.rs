use crate::systems::{
    selection::Selectable,
    unit::{TargetPosition, Unit},
};
use bevy::{prelude::*, render::camera::Camera};
use bevy_mod_picking::*;

#[derive(Default)]
pub struct CanHaveCamera {
    translation_offset: Vec3,
    pub rotation_offset: Quat,
    camera_entity: Option<Entity>,
}
impl CanHaveCamera {
    pub fn new_with_camera(camera_entity: Entity) -> Self {
        Self {
            translation_offset: Vec3::zero(),
            rotation_offset: Quat::identity(),
            camera_entity: Some(camera_entity),
        }
    }

    pub fn has_camera(&self) -> bool {
        self.camera_entity.is_some()
    }
}

/// Sets the camera position to whatever the current object that has it
fn update_camera_position(
    camera_query: Query<(&Camera, &mut Transform)>,
    mut has_camera_query: Query<(&CanHaveCamera, &Transform)>,
) {
    for (can_have_camera, parent_transform) in &mut has_camera_query.iter() {
        if let Some(camera_entity) = can_have_camera.camera_entity {
            if let Ok(mut transform) = camera_query.get_mut::<Transform>(camera_entity) {
                let new_translation =
                    parent_transform.translation() + can_have_camera.translation_offset;
                let new_rotation = parent_transform.rotation() * can_have_camera.rotation_offset;

                *transform.value_mut() =
                    Mat4::from_rotation_translation(new_rotation, new_translation);
            }
        }
    }
}

fn reset_unit_target_if_it_has_camera(
    mut query: Query<(
        Changed<CanHaveCamera>,
        &mut TargetPosition,
        &mut Selectable,
        &mut Draw,
    )>,
) {
    for (can_have_camera, mut target, mut selectable, mut draw) in &mut query.iter() {
        if can_have_camera.camera_entity.is_some() {
            target.pos = None;
            selectable.selected = false;
            draw.is_visible = false;
        } else {
            draw.is_visible = true;
        }
    }
}

fn switch_camera_to_entity(
    pick_state: Res<PickState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(&mut CanHaveCamera, &Unit)>,
) {
    if !keyboard_input.pressed(KeyCode::M) || !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    // Get the entity selected
    if let Some(top_pick) = pick_state.top(PickGroup::default()) {
        let camera_entity = {
            let mut camera: Option<Entity> = None;
            for (mut can_have_camera, _) in &mut query.iter() {
                if can_have_camera.camera_entity.is_some() {
                    camera = can_have_camera.camera_entity;
                }
                can_have_camera.camera_entity = None;
            }
            camera
        };

        let entity = top_pick.entity();
        let unit_result = query.entity(entity);

        if let Ok(mut unit) = unit_result {
            if let Some((mut can_have_camera, _)) = unit.get() {
                // This is the selected unit
                can_have_camera.camera_entity = camera_entity;
            }
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_camera_position.system())
            .add_system(switch_camera_to_entity.system())
            .add_system(reset_unit_target_if_it_has_camera.system());
    }
}

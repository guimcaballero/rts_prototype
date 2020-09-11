use bevy::{prelude::*, render::camera::Camera};

pub struct CanHaveCamera {
    translation_offset: Vec3,
    rotation_offset: Quat,
    camera_entity: Option<Entity>,
}
impl CanHaveCamera {
    pub fn new() -> Self {
        Self {
            translation_offset: Vec3::zero(),
            rotation_offset: Quat::identity(),
            camera_entity: None,
        }
    }
    pub fn new_with_camera(camera_entity: Entity) -> Self {
        Self {
            translation_offset: Vec3::zero(),
            rotation_offset: Quat::identity(),
            camera_entity: Some(camera_entity),
        }
    }
}

/// Sets the camera position to whatever the current object that has it
fn update_camera_position(
    camera_query: Query<(&Camera, &mut Translation, &mut Rotation)>,
    mut has_camera_query: Query<(&CanHaveCamera, &Translation, &Rotation)>,
) {
    for (can_have_camera, parent_trans, parent_rot) in &mut has_camera_query.iter() {
        if let Some(camera_entity) = can_have_camera.camera_entity {
            if let Ok(mut camera) = camera_query.entity(camera_entity) {
                if let Some((_, mut translation, mut rotation)) = camera.get() {
                    translation.0 = parent_trans.0 + can_have_camera.translation_offset;
                    // TODO I'm pretty sure this isn't correct
                    rotation.0 = parent_rot.0 * can_have_camera.rotation_offset;
                }
            }
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update_camera_position.system());
    }
}

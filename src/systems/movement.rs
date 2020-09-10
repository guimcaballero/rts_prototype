use bevy::{math::Vec3, prelude::*, render::camera::Camera};
use bevy_mod_picking::*;

const SPEED: f32 = 0.1;
const CAMERA_SPEED: f32 = 3.0;

pub struct TargetPosition {
    pub pos: Option<Vec3>,
}
impl TargetPosition {
    #[inline(always)]
    pub fn new() -> Self {
        Self { pos: None }
    }

    pub fn update_to_vec(&mut self, vec: &Vec3) {
        if let Some(pos) = self.pos.as_mut() {
            pos.set_x(vec.x());
            pos.set_y(vec.y());
            pos.set_z(vec.z());
        } else {
            self.pos = Some(vec.clone());
        }
    }
}

// Moves towards the target while it's not selected
pub fn move_to_target(mut query: Query<(&mut TargetPosition, &mut Translation)>) {
    for (mut target, mut translation) in &mut query.iter() {
        if let Some(target_pos) = target.pos {
            let mut direction = target_pos - translation.0;
            direction.set_y(0.0);
            if direction.length() > 0.3 {
                let direction = direction.normalize() * SPEED;
                translation.0 += direction;
            } else {
                println!("reached destination {}", target_pos);
                target.pos = None;
            }
        }
    }
}

pub fn set_target_for_selected(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(&SelectablePickMesh, &mut TargetPosition)>,
    mut camera_query: Query<(&Transform, &Camera)>,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        // Get the camera
        let mut view_matrix = Mat4::zero();
        let mut projection_matrix = Mat4::zero();
        for (transform, camera) in &mut camera_query.iter() {
            view_matrix = transform.value.inverse();
            projection_matrix = camera.projection_matrix;
        }

        // Get the world position
        if let Some(top_pick) = pick_state.top() {
            let pos = top_pick.get_pick_coord_world(projection_matrix, view_matrix);

            for (selectable, mut target) in &mut query.iter() {
                if selectable.selected() {
                    target.update_to_vec(&pos);
                }
            }
        } else {
            println!("can't find position");
        }
    }
}

pub fn wasd_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query_camera: Query<(&Camera, &mut Translation, &mut Rotation)>,
) {
    for (_, mut translation, mut rotation) in &mut query_camera.iter() {
        if !keyboard_input.pressed(KeyCode::Space) {
            if keyboard_input.pressed(KeyCode::W) {
                translation.0 += rotation.0.mul_vec3(Vec3::new(0.0, 1.0, 0.0))
                    * time.delta_seconds
                    * CAMERA_SPEED;
            }
            if keyboard_input.pressed(KeyCode::A) {
                translation.0 += rotation.0.mul_vec3(Vec3::new(-1.0, 0.0, 0.0))
                    * time.delta_seconds
                    * CAMERA_SPEED;
            }
            if keyboard_input.pressed(KeyCode::S) {
                translation.0 += rotation.0.mul_vec3(Vec3::new(0.0, -1.0, 0.0))
                    * time.delta_seconds
                    * CAMERA_SPEED;
            }
            if keyboard_input.pressed(KeyCode::D) {
                translation.0 += rotation.0.mul_vec3(Vec3::new(1.0, 0.0, 0.0))
                    * time.delta_seconds
                    * CAMERA_SPEED;
            }
            if keyboard_input.pressed(KeyCode::Q) {
                translation.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, -1.0))
                    * time.delta_seconds
                    * CAMERA_SPEED;
            }
            if keyboard_input.pressed(KeyCode::E) {
                translation.0 += rotation.0.mul_vec3(Vec3::new(0.0, 0.0, 1.0))
                    * time.delta_seconds
                    * CAMERA_SPEED;
            }
        }

        if keyboard_input.pressed(KeyCode::Space) {
            if keyboard_input.pressed(KeyCode::A) {
                rotation.0 *=
                    Quat::from_rotation_ypr(1.0 * time.delta_seconds * CAMERA_SPEED, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::D) {
                rotation.0 *=
                    Quat::from_rotation_ypr(-1.0 * time.delta_seconds * CAMERA_SPEED, 0.0, 0.0);
            }
            if keyboard_input.pressed(KeyCode::W) {
                rotation.0 *=
                    Quat::from_rotation_ypr(0.0, 1.0 * time.delta_seconds * CAMERA_SPEED, 0.0);
            }
            if keyboard_input.pressed(KeyCode::S) {
                rotation.0 *=
                    Quat::from_rotation_ypr(0.0, -1.0 * time.delta_seconds * CAMERA_SPEED, 0.0);
            }
        }
    }
}

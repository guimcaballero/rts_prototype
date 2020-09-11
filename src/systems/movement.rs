use bevy::{math::Vec3, prelude::*, render::camera::Camera};
use bevy_mod_picking::*;

const SPEED: f32 = 0.1;

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

pub struct TargetIndicator;
pub fn show_target_indicator(
    mut indicator_query: Query<(&TargetIndicator, &mut Translation, &mut Draw)>,
    mut selected_query: Query<(&SelectablePickMesh, &TargetPosition)>,
) {
    let mut selections_with_target_exist = false;
    for (selectable, target) in &mut selected_query.iter() {
        // We only want selected items
        if !selectable.selected() {
            continue;
        }

        // Set the Indicator to the Target position
        if let Some(target_pos) = target.pos {
            selections_with_target_exist = true;

            for (_, mut translation, _) in &mut indicator_query.iter() {
                translation.0 = target_pos;
            }
        }
    }

    // Toggle drawability according to if there is anything selected
    for (_, _, mut draw) in &mut indicator_query.iter() {
        draw.is_visible = selections_with_target_exist;
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

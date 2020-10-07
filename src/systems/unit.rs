use bevy::{math::Vec3, prelude::*};
use bevy_mod_picking::*;

const SPEED: f32 = 0.1;

pub struct Unit {
    pub selected: bool,
}
impl Unit {
    pub fn new() -> Self {
        Self { selected: false }
    }
}

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
fn show_target_indicator(
    mut indicator_query: Query<(&TargetIndicator, &mut Transform, &mut Draw)>,
    mut unit_query: Query<(&Unit, &TargetPosition)>,
) {
    let mut selections_with_target_exist = false;
    for (unit, target) in &mut unit_query.iter() {
        // We only want selected items
        if !unit.selected {
            continue;
        }

        // Set the Indicator to the Target position
        if let Some(target_position) = target.pos {
            selections_with_target_exist = true;

            for (_, mut transform, _) in &mut indicator_query.iter() {
                transform.set_translation(Vec3::new(target_position.x(), 0.3, target_position.z()));
            }
        }
    }

    // Toggle drawability according to if there is anything selected
    for (_, _, mut draw) in &mut indicator_query.iter() {
        draw.is_visible = selections_with_target_exist;
    }
}

// Moves towards the target while it's not selected
fn move_to_target(mut query: Query<(&Unit, &mut TargetPosition, &mut Transform)>) {
    for (_, mut target, mut transform) in &mut query.iter() {
        if let Some(target_pos) = target.pos {
            let mut direction = target_pos - transform.translation();
            direction.set_y(0.0);
            if direction.length() > 0.3 {
                let direction = direction.normalize() * SPEED;
                transform.translate(direction);
            } else {
                target.pos = None;
            }
        }
    }
}

fn set_target_for_selected(
    pick_state: Res<PickState>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<(&Unit, &mut TargetPosition)>,
) {
    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        // Get the world position
        if let Some(top_pick) = pick_state.top(PickGroup::default()) {
            let pos = top_pick.position();

            for (unit, mut target) in &mut query.iter() {
                if unit.selected {
                    target.update_to_vec(pos);
                }
            }
        }
    }
}

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(move_to_target.system())
            .add_system(set_target_for_selected.system())
            .add_system(show_target_indicator.system());
    }
}

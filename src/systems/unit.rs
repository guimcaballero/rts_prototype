use bevy::{math::Vec3, prelude::*};
use bevy_mod_picking::*;

pub struct Unit {
    pub selected: bool,
    pub speed: f32,
    pub social_distance: f32,
}
impl Default for Unit {
    fn default() -> Self {
        Self {
            selected: false,
            speed: 0.1,
            social_distance: 1.5,
        }
    }
}

#[derive(Default)]
pub struct TargetPosition {
    pub pos: Option<Vec3>,
}

impl TargetPosition {
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
fn unit_movement(mut query: Query<(&Unit, &mut TargetPosition, &mut Transform, Entity)>) {
    // TODO Do something to divide by space or something
    let mut unit_positions = Vec::new();
    for (unit, _, transform, entity) in &mut query.iter() {
        unit_positions.push((entity, transform.translation(), unit.social_distance));
    }

    for (unit, mut target, mut transform, entity) in &mut query.iter() {
        let translation = transform.translation();
        let mut velocity = Vec3::zero();

        // Keep a distance to other units
        // Inspired from https://github.com/JohnPeel/flock-rs
        let mut separation = Vec3::zero();
        let mut units_nearby = 0;
        for (other_entity, other_translation, social_distance) in &unit_positions {
            if *other_entity != entity {
                let difference = translation - *other_translation;
                let distance_squared = difference.length_squared();
                let minimum_distance = unit.social_distance + social_distance;

                if distance_squared < minimum_distance * minimum_distance {
                    units_nearby += 1;
                    separation += difference.normalize()
                        * (minimum_distance - distance_squared.sqrt())
                        / minimum_distance;
                }
            }
        }
        velocity += separation;

        // Move towards target
        if let Some(target_pos) = target.pos {
            let mut direction = target_pos - transform.translation();
            direction.set_y(0.0);

            if direction.length() > 0.3 + units_nearby as f32 {
                let direction = direction.normalize() * unit.speed;
                velocity += direction;
            } else {
                // When we reach the target, remove it
                target.pos = None;
            }
        }

        // If unit is on the floor, we don't allow going down
        if translation.y() <= 1.01 && velocity.y() < 0. {
            velocity.set_y(0.);
        }

        transform.translate(velocity);
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
        app.add_system(unit_movement.system())
            .add_system(set_target_for_selected.system())
            .add_system(show_target_indicator.system());
    }
}

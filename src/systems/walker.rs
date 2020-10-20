use crate::helpers::movement::*;
use crate::systems::camera::CanHaveCamera;
use bevy::{input::mouse::MouseMotion, math::Vec3, prelude::*};

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

pub struct Walker {
    /// The speed the Walker moves at. Defaults to `1.0`
    pub speed: f32,
    /// The maximum speed the Walker can move at. Defaults to `0.5`
    pub max_speed: f32,
    /// The sensitivity of the Walker's motion based on mouse movement. Defaults to `3.0`
    pub sensitivity: f32,
    /// The amount of deceleration to apply to the camera's motion. Defaults to `1.0`
    pub friction: f32,
    /// The current pitch of the Walker in degrees. This value is always up-to-date
    pub pitch: f32,
    /// The current pitch of the Walker in degrees. This value is always up-to-date
    pub yaw: f32,
    /// The current velocity of the Walker. This value is always up-to-date
    pub velocity: Vec3,
}
impl Default for Walker {
    fn default() -> Self {
        Self {
            speed: 1.0,
            max_speed: 0.5,
            sensitivity: 1.0,
            friction: 10.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::zero(),
        }
    }
}

fn wasd_walk_for_camera_holder(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Walker, &CanHaveCamera, &mut Transform)>,
) {
    for (mut options, can_have_camera, mut transform) in &mut query.iter() {
        if !can_have_camera.has_camera() {
            continue;
        }

        let axis_h = movement_axis(&keyboard_input, KeyCode::D, KeyCode::A);
        let axis_v = movement_axis(&keyboard_input, KeyCode::S, KeyCode::W);

        let any_button_down = axis_h != 0.0 || axis_v != 0.0;

        let rotation = transform.rotation;
        let accel: Vec3 = ((strafe_vector(&rotation) * axis_h)
            + (forward_walk_vector(&rotation) * axis_v))
            * options.speed;

        let friction: Vec3 = if options.velocity.length() != 0.0 && !any_button_down {
            options.velocity.normalize() * -1.0 * options.friction
        } else {
            Vec3::zero()
        };

        options.velocity += accel * time.delta_seconds;

        // clamp within max speed
        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds;

        options.velocity =
            if (options.velocity + delta_friction).signum() != options.velocity.signum() {
                Vec3::zero()
            } else {
                options.velocity + delta_friction
            };

        transform.translation += options.velocity;
    }
}

/// Rotate according to mouse if the LShift key is pressed
fn walker_mouse_rotation_system(
    time: Res<Time>,
    mut state: ResMut<State>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Walker, &mut CanHaveCamera, &mut Transform)>,
) {
    // Only enable rotation while the LShift is pressed
    if !keyboard_input.pressed(KeyCode::LShift) {
        return;
    }

    let mut delta: Vec2 = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        delta += event.delta;
    }
    if delta == Vec2::zero() {
        return;
    }

    for (mut options, mut can_have_camera, mut transform) in &mut query.iter() {
        if !can_have_camera.has_camera() {
            continue;
        }

        options.yaw -= delta.x() * options.sensitivity * time.delta_seconds;
        options.pitch += delta.y() * options.sensitivity * time.delta_seconds;

        if options.pitch > 89.9 {
            options.pitch = 89.9;
        }
        if options.pitch < -89.9 {
            options.pitch = -89.9;
        }

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), yaw_radians);
        can_have_camera.rotation_offset = Quat::from_axis_angle(-Vec3::unit_x(), pitch_radians);
    }
}

pub struct WalkerPlugin;
impl Plugin for WalkerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_system(wasd_walk_for_camera_holder.system())
            .add_system(walker_mouse_rotation_system.system());
    }
}

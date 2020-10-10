use crate::helpers::movement::*;
use crate::systems::camera::CanHaveCamera;
use bevy::{input::mouse::MouseMotion, prelude::*};

// From https://github.com/mcpar-land/bevy_fly_camera/blob/master/src/lib.rs

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

pub struct Drone {
    /// The speed the Drone moves at. Defaults to `1.0`
    pub speed: f32,
    /// The maximum speed the Drone can move at. Defaults to `0.5`
    pub max_speed: f32,
    /// The amount of deceleration to apply to the camera's motion. Defaults to `1.0`
    pub friction: f32,
    /// The current pitch of the Drone in degrees. This value is always up-to-date
    pub pitch: f32,
    /// The current pitch of the Drone in degrees. This value is always up-to-date
    pub yaw: f32,
    /// The current velocity of the Drone. This value is always up-to-date
    pub velocity: Vec3,
}
impl Default for Drone {
    fn default() -> Self {
        Self {
            speed: 1.0,
            max_speed: 0.5,
            friction: 1.3,
            pitch: 60.0,
            yaw: -90.0,
            velocity: Vec3::zero(),
        }
    }
}

/// Move the Drone according to keys pressed
fn drone_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Drone, &CanHaveCamera, &mut Transform)>,
) {
    for (mut options, can_have_camera, mut transform) in &mut query.iter() {
        if !can_have_camera.has_camera() {
            continue;
        }

        if keyboard_input.pressed(KeyCode::C) {
            options.velocity = Vec3::zero();
            continue;
        }

        let axis_h = movement_axis(&keyboard_input, KeyCode::D, KeyCode::A);
        let axis_v = movement_axis(&keyboard_input, KeyCode::S, KeyCode::W);
        let axis_float = movement_axis(&keyboard_input, KeyCode::E, KeyCode::Q);

        let any_button_down = axis_h != 0.0 || axis_v != 0.0 || axis_float != 0.0;

        let rotation = transform.rotation();
        let mut accel: Vec3 = ((strafe_vector(&rotation) * axis_h)
            + (forward_walk_vector(&rotation) * axis_v)
            + (Vec3::unit_y() * axis_float))
            * options.speed;

        let translation = transform.translation();
        let y = translation.y();
        if y <= 10. {
            accel += Vec3::unit_y() * (10. - y).abs();
        }

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

        options.velocity = if (options.velocity + delta_friction).sign() != options.velocity.sign()
        {
            Vec3::zero()
        } else {
            options.velocity + delta_friction
        };

        // If unit is on the floor, we don't allow going down
        if translation.y() <= 1.01 && options.velocity.y() < 0. {
            options.velocity.set_y(0.);
        }

        transform.translate(options.velocity);
    }
}

/// Rotate according to mouse if the LShift key is pressed
fn drone_mouse_rotation_system(
    time: Res<Time>,
    mut state: ResMut<State>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Drone, &CanHaveCamera, &mut Transform)>,
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

    for (mut options, can_have_camera, mut transform) in &mut query.iter() {
        if !can_have_camera.has_camera() {
            continue;
        }

        options.yaw -= delta.x() * 3.0 * time.delta_seconds;
        options.pitch += delta.y() * 3.0 * time.delta_seconds;

        if options.pitch > 89.9 {
            options.pitch = 89.9;
        }
        if options.pitch < -89.9 {
            options.pitch = -89.9;
        }

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.set_rotation(
            Quat::from_axis_angle(Vec3::unit_y(), yaw_radians)
                * Quat::from_axis_angle(-Vec3::unit_x(), pitch_radians),
        );
    }
}

pub struct DronePlugin;
impl Plugin for DronePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<State>()
            .add_system(drone_movement_system.system())
            .add_system(drone_mouse_rotation_system.system());
    }
}

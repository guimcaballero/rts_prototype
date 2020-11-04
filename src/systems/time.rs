use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use instant::Instant;
use std::time::Duration;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

pub struct ControlledTime {
    pub paused: bool,

    pub delta: Duration,
    pub instant: Option<Instant>,
    pub delta_seconds_f64: f64,
    pub delta_seconds: f32,
    pub seconds_since_startup: f64,
    pub startup: Instant,
}
impl Default for ControlledTime {
    fn default() -> Self {
        Self {
            paused: false,

            delta: Duration::from_secs(0),
            instant: None,
            startup: Instant::now(),
            delta_seconds_f64: 0.0,
            seconds_since_startup: 0.0,
            delta_seconds: 0.0,
        }
    }
}

impl ControlledTime {
    fn update(&mut self) {
        let now = Instant::now();
        if self.paused {
            self.delta = Duration::from_secs(0);
            self.delta_seconds_f64 = self.delta.as_secs_f64();
            self.delta_seconds = self.delta.as_secs_f32();
        } else {
            if let Some(instant) = self.instant {
                self.delta = now - instant;
                self.delta_seconds_f64 = self.delta.as_secs_f64();
                self.delta_seconds = self.delta.as_secs_f32();
            }

            self.seconds_since_startup += self.delta_seconds_f64;
        }
        self.instant = Some(now);
    }
}

fn update_time(mut time: ResMut<ControlledTime>) {
    time.update();
}

fn pause_with_space(mut time: ResMut<ControlledTime>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        time.paused = !time.paused;
        dbg!(time.paused);
    }
}

pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ControlledTime>()
            .add_system(update_time.system())
            .add_system(pause_with_space.system());
    }
}

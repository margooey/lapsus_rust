// warning: this is llm code because i did not want to write a complete reimplementation of OpenMultitouchSupport in rust
// i will need to rewrite this myself since i think this is bad
// update: it's actually fine and i'm not gonna rewrite it but i do want to understand it a bit more than i currently do

use cidre::cg::{Float, Point, Vector};
use macos_multitouch::{self, MultitouchDevice};
use std::sync::{Arc, Mutex};

use crate::config;

#[derive(Clone, Copy, Debug)]
pub struct TouchMetrics {
    pub centroid: Option<Point>,
    pub normalized_velocity: Vector,
    pub is_touching: bool,
}

struct TrackpadState {
    is_touching: bool,
    latest_positions: Vec<Point>,
    latest_centroid: Option<Point>,
    previous_centroid: Option<Point>,
    last_sample_timestamp: f64,
    normalized_velocity: Vector,
    suppress_glide_deadline: f64,
}

pub struct TrackpadMonitor {
    devices: Vec<MultitouchDevice>,
    state: Arc<Mutex<TrackpadState>>,
    listener_started: bool,
}

impl TrackpadMonitor {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            state: Arc::new(Mutex::new(TrackpadState {
                is_touching: false,
                latest_positions: Vec::new(),
                latest_centroid: None,
                previous_centroid: None,
                last_sample_timestamp: 0.0,
                normalized_velocity: Vector { dx: 0.0, dy: 0.0 },
                suppress_glide_deadline: 0.0,
            })),
            listener_started: false,
        }
    }

    pub fn start(&mut self) {
        if self.listener_started {
            return;
        }
        self.listener_started = true;

        let state = self.state.clone();
        let mut devices = macos_multitouch::get_multitouch_devices();
        log::info!("trackpad devices: {}", devices.len());
        if devices.is_empty() {
            log::warn!("no multitouch devices detected");
        }
        for device in devices.iter_mut() {
            let state = state.clone();
            let _ = device.register_contact_frame_callback(
                move |_device, data: &[macos_multitouch::Finger], timestamp, _frame| {
                    let positions: Vec<Point> = data
                        .iter()
                        .map(|finger| Point {
                            x: finger.normalized.pos.x as Float,
                            y: finger.normalized.pos.y as Float,
                        })
                        .collect();

                    let mut state = state.lock().expect("trackpad state lock poisoned");
                    update_touch_metrics(&mut state, &positions, timestamp);
                },
            );
        }

        self.devices = devices;
    }

    pub fn stop(&mut self) {
        for device in self.devices.iter_mut() {
            device.stop();
        }
        self.devices.clear();
        self.listener_started = false;
    }

    pub fn is_touching(&self) -> bool {
        self.state
            .lock()
            .expect("trackpad state lock poisoned")
            .is_touching
    }

    pub fn current_touch_positions(&self) -> Vec<Point> {
        self.state
            .lock()
            .expect("trackpad state lock poisoned")
            .latest_positions
            .clone()
    }

    pub fn current_touch_centroid(&self) -> Option<Point> {
        self.state
            .lock()
            .expect("trackpad state lock poisoned")
            .latest_centroid
    }

    pub fn current_normalized_velocity(&self) -> Option<Vector> {
        let state = self.state.lock().expect("trackpad state lock poisoned");
        if state.is_touching {
            Some(state.normalized_velocity)
        } else {
            None
        }
    }

    pub fn metrics(&self) -> TouchMetrics {
        let state = self.state.lock().expect("trackpad state lock poisoned");
        TouchMetrics {
            centroid: state.latest_centroid,
            normalized_velocity: state.normalized_velocity,
            is_touching: state.is_touching,
        }
    }

    pub fn should_suppress_glide(&self) -> bool {
        let deadline = self
            .state
            .lock()
            .expect("trackpad state lock poisoned")
            .suppress_glide_deadline;
        objc2_core_foundation::CFAbsoluteTimeGetCurrent() < deadline
    }
}

fn update_touch_metrics(state: &mut TrackpadState, positions: &[Point], timestamp: f64) {
    state.latest_positions.clear();
    state.latest_positions.extend_from_slice(positions);
    if positions.len() > 1 {
        let now = objc2_core_foundation::CFAbsoluteTimeGetCurrent();
        let duration = config().multi_finger_suppression_deadline;
        state.suppress_glide_deadline = now + duration;
    }
    let was_touching = state.is_touching;
    state.is_touching = !positions.is_empty();
    if state.is_touching != was_touching {
        log::info!("touch {}", if state.is_touching { "start" } else { "end" });
    }

    if positions.is_empty() {
        state.latest_centroid = None;
        state.previous_centroid = None;
        state.normalized_velocity = Vector { dx: 0.0, dy: 0.0 };
        state.last_sample_timestamp = timestamp;
        return;
    }

    let mut centroid = Point { x: 0.0, y: 0.0 };
    for point in positions {
        centroid.x += point.x;
        centroid.y += point.y;
    }
    let divisor = positions.len() as Float;
    centroid.x /= divisor;
    centroid.y /= divisor;

    state.latest_centroid = Some(centroid);

    if let Some(previous) = state.previous_centroid {
        if state.last_sample_timestamp > 0.0 {
            let mut delta_time = (timestamp - state.last_sample_timestamp) as Float;
            if delta_time < config().min_dt {
                delta_time = config().min_dt;
            }
            let raw_velocity = Vector {
                dx: (centroid.x - previous.x) / delta_time,
                dy: (centroid.y - previous.y) / delta_time,
            };
            state.normalized_velocity = Vector {
                dx: state.normalized_velocity.dx
                    * (1.0 - config().velocity_smoothing)
                    + raw_velocity.dx * config().velocity_smoothing,
                dy: state.normalized_velocity.dy
                    * (1.0 - config().velocity_smoothing)
                    + raw_velocity.dy * config().velocity_smoothing,
            };
        } else {
            state.normalized_velocity = Vector { dx: 0.0, dy: 0.0 };
        }
    } else {
        state.normalized_velocity = Vector { dx: 0.0, dy: 0.0 };
    }

    state.previous_centroid = Some(centroid);
    state.last_sample_timestamp = timestamp;
}

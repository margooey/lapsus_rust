// warning: this is llm code because i did not want to write a complete reimplementation of OpenMultitouchSupport in rust

use cidre::cg;
use macos_multitouch::{self, MultitouchDevice};
use std::sync::{Arc, Mutex};

const MIN_DT: cg::Float = 1.0 / 500.0;

#[derive(Clone, Copy, Debug)]
pub struct TouchMetrics {
    pub centroid: Option<cg::Point>,
    pub normalized_velocity: cg::Vector,
    pub is_touching: bool,
}

struct TrackpadState {
    is_touching: bool,
    latest_positions: Vec<cg::Point>,
    latest_centroid: Option<cg::Point>,
    previous_centroid: Option<cg::Point>,
    last_sample_timestamp: f64,
    normalized_velocity: cg::Vector,
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
                normalized_velocity: cg::Vector { dx: 0.0, dy: 0.0 },
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
        for device in devices.iter_mut() {
            let state = state.clone();
            let _ = device.register_contact_frame_callback(
                move |_device, data: &[macos_multitouch::Finger], timestamp, _frame| {
                    let positions: Vec<cg::Point> = data
                        .iter()
                        .map(|finger| cg::Point {
                            x: finger.normalized.pos.x as cg::Float,
                            y: finger.normalized.pos.y as cg::Float,
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

    pub fn current_touch_positions(&self) -> Vec<cg::Point> {
        self.state
            .lock()
            .expect("trackpad state lock poisoned")
            .latest_positions
            .clone()
    }

    pub fn current_touch_centroid(&self) -> Option<cg::Point> {
        self.state
            .lock()
            .expect("trackpad state lock poisoned")
            .latest_centroid
    }

    pub fn current_normalized_velocity(&self) -> Option<cg::Vector> {
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
}

fn update_touch_metrics(state: &mut TrackpadState, positions: &[cg::Point], timestamp: f64) {
    state.latest_positions.clear();
    state.latest_positions.extend_from_slice(positions);
    state.is_touching = !positions.is_empty();

    if positions.is_empty() {
        state.latest_centroid = None;
        state.previous_centroid = None;
        state.normalized_velocity = cg::Vector { dx: 0.0, dy: 0.0 };
        state.last_sample_timestamp = timestamp;
        return;
    }

    let mut centroid = cg::Point { x: 0.0, y: 0.0 };
    for point in positions {
        centroid.x += point.x;
        centroid.y += point.y;
    }
    let divisor = positions.len() as cg::Float;
    centroid.x /= divisor;
    centroid.y /= divisor;

    state.latest_centroid = Some(centroid);

    if let Some(previous) = state.previous_centroid {
        if state.last_sample_timestamp > 0.0 {
            let mut delta_time = (timestamp - state.last_sample_timestamp) as cg::Float;
            if delta_time < MIN_DT {
                delta_time = MIN_DT;
            }
            let raw_velocity = cg::Vector {
                dx: (centroid.x - previous.x) / delta_time,
                dy: (centroid.y - previous.y) / delta_time,
            };
            state.normalized_velocity = cg::Vector {
                dx: state.normalized_velocity.dx * (1.0 - env!("VELOCITY_SMOOTHING").parse::<cg::Float>().unwrap())
                    + raw_velocity.dx * env!("VELOCITY_SMOOTHING").parse::<cg::Float>().unwrap(),
                dy: state.normalized_velocity.dy * (1.0 - env!("VELOCITY_SMOOTHING").parse::<cg::Float>().unwrap())
                    + raw_velocity.dy * env!("VELOCITY_SMOOTHING").parse::<cg::Float>().unwrap(),
            };
        } else {
            state.normalized_velocity = cg::Vector { dx: 0.0, dy: 0.0 };
        }
    } else {
        state.normalized_velocity = cg::Vector { dx: 0.0, dy: 0.0 };
    }

    state.previous_centroid = Some(centroid);
    state.last_sample_timestamp = timestamp;
}

use cidre::cg;
use cacao::appkit;
use objc2_application_services;
use macos_multitouch;

enum VelocitySource {
    Pointer,
    Trackpad,
}

struct State {
    position: cg::Point,
    previous_position: cg::Point,
    last_input_delta: cg::Vector,
    velocity: cg::Vector,
    is_gliding: bool,
    velocity_source: VelocitySource,
}

pub struct Engine {
    state: State,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            state: State {
                position: cg::Point { x: 0.0, y: 0.0 },
                previous_position: cg::Point { x: 0.0, y: 0.0 },
                last_input_delta: cg::Vector { dx: 0.0, dy: 0.0 },
                velocity: cg::Vector { dx: 0.0, dy: 0.0 },
                is_gliding: false,
                velocity_source: VelocitySource::Pointer,
            },
        }
    }
}
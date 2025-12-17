// use crate::trackpad;
// use cacao::appkit;
use cidre::cg;
// use objc2_application_services;
use crate::utils::{min, max, min_x, max_x, min_y, max_y};

pub const MAXIMUM_MOMENTUM_SPEED: cg::Float = 9000.0;

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
    last_physical_mouse_position: cg::Point,
    desktop_bounds: cg::Rect,
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
            last_physical_mouse_position: cg::Point { x: 0.0, y: 0.0 },
            desktop_bounds: cg::Rect::null(),
        }
    }

    pub fn set_gliding(&mut self, value: bool) {
        self.state.is_gliding = value;
    }

    pub fn begin_touch(&mut self, position: cg::Point) {
        self.state.position = position;
        self.state.previous_position = position;
        self.state.last_input_delta = cg::Vector { dx: 0.0, dy: 0.0 };
        self.state.velocity = cg::Vector { dx: 0.0, dy: 0.0 };
        self.set_gliding(false);
    }

    pub fn handle_touch(
        &mut self,
        physical_location: cg::Point,
        delta_time: cg::Float,
        normalized_trackpad_velocity: Option<cg::Vector>,
    ) {
        let delta_pos = cg::Point {
            x: physical_location.x - self.last_physical_mouse_position.x,
            y: physical_location.y - self.last_physical_mouse_position.y,
        };
        self.last_physical_mouse_position = physical_location;
        self.state.previous_position = self.state.position;

        let pointer_velocity = cg::Vector {
            dx: delta_pos.x / delta_time,
            dy: delta_pos.y / delta_time,
        };

        let mut velocity = pointer_velocity;
        let mut source: VelocitySource = VelocitySource::Pointer;
        if let Some(trackpad_velocity) =
            self.trackpad_velocity_in_pixels(&normalized_trackpad_velocity)
        {
            if Self::magnitude(&trackpad_velocity) > Self::magnitude(&pointer_velocity) {
                velocity = trackpad_velocity;
                source = VelocitySource::Trackpad;
            }
        }
        self.state.velocity = velocity;
        self.state.velocity_source = source;
        self.state.position.x += delta_pos.x;
        self.state.position.y += delta_pos.y;
        self.state.last_input_delta = cg::Vector {dx: delta_pos.x, dy: delta_pos.y};

        self.clamp_position_to_desktop();
        
        if self.state.is_gliding {
            self.set_gliding(false);
        }
    }

    fn magnitude(vector: &cg::Vector) -> cg::Float {
        (vector.dx * vector.dx + vector.dy * vector.dy).sqrt()
    }

    fn clamp_position_to_desktop(&mut self) {
        if self.desktop_bounds == cg::Rect::null() {
            return;
        }
        self.state.position.x = min(
            max(self.state.position.x, min_x(&self.desktop_bounds)),
            max_x(&self.desktop_bounds),
        );
        self.state.position.y = min(
            max(self.state.position.y, min_y(&self.desktop_bounds)),
            max_y(&self.desktop_bounds),
        );
    }

    fn trackpad_velocity_in_pixels(
        &mut self,
        normalized_velocity: &Option<cg::Vector>,
    ) -> Option<cg::Vector> {
        if self.desktop_bounds != cg::Rect::null() {
            return None;
        }
        if let Some(normalized_velocity) = normalized_velocity {
            let scaled = cg::Vector {
                dx: normalized_velocity.dx * self.desktop_bounds.size.width,
                dy: normalized_velocity.dy * self.desktop_bounds.size.height,
            };
            return Some(Self::clamped_velocity(&scaled, MAXIMUM_MOMENTUM_SPEED));
        } else {
            return None;
        }
    }

    fn clamped_velocity(vector: &cg::Vector, max_magnitude: cg::Float) -> cg::Vector {
        let magnitude = Self::magnitude(vector);
        if magnitude > max_magnitude && magnitude > 0.0 {
            let scale = max_magnitude / magnitude;
            return cg::Vector {
                dx: vector.dx * scale,
                dy: vector.dy * scale,
            };
        } else {
            return *vector;
        }
    }
}

use crate::utils::{max, max_x, max_y, min, min_x, min_y};
use cidre::cg::{Float, Point, Rect, Vector};
use core_graphics::display;
use objc2_app_kit::NSScreen;

pub const ZERO_VECTOR: Vector = Vector { dx: 0.0, dy: 0.0 };

enum VelocitySource {
    Pointer,
    Trackpad,
}

pub struct State {
    position: Point,
    previous_position: Point,
    last_input_delta: Vector,
    velocity: Vector,
    pub is_gliding: bool,
    velocity_source: VelocitySource,
}

pub struct Engine {
    pub state: State,
    last_physical_mouse_position: Point,
    desktop_bounds: Rect,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            state: State {
                position: Point { x: 0.0, y: 0.0 },
                previous_position: Point { x: 0.0, y: 0.0 },
                last_input_delta: Vector { dx: 0.0, dy: 0.0 },
                velocity: Vector { dx: 0.0, dy: 0.0 },
                is_gliding: false,
                velocity_source: VelocitySource::Pointer,
            },
            last_physical_mouse_position: Point { x: 0.0, y: 0.0 },
            desktop_bounds: Rect::null(),
        }
    }

    pub fn set_gliding(&mut self, value: bool) {
        self.state.is_gliding = value;
    }

    pub fn begin_touch(&mut self, position: Point) {
        self.state.position = position;
        self.state.previous_position = position;
        self.state.last_input_delta = ZERO_VECTOR;
        self.state.velocity = ZERO_VECTOR;
        self.set_gliding(false);
    }

    pub fn handle_touch(
        &mut self,
        physical_position: Point,
        delta_time: Float,
        normalized_trackpad_velocity: Option<Vector>,
    ) {
        let delta_pos = Point {
            x: physical_position.x - self.last_physical_mouse_position.x,
            y: physical_position.y - self.last_physical_mouse_position.y,
        };
        self.last_physical_mouse_position = physical_position;
        self.state.previous_position = self.state.position;

        let pointer_velocity = Vector {
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
        self.state.last_input_delta = Vector {
            dx: delta_pos.x,
            dy: delta_pos.y,
        };

        self.clamp_position_to_desktop();

        if self.state.is_gliding {
            self.set_gliding(false);
        }
    }

    pub fn handle_no_touch(
        &mut self,
        physical_position: Point,
        delta_time: Float,
        suppress_glide: bool,
        touch_ended_recently: bool,
    ) {
        self.last_physical_mouse_position = physical_position;
        if touch_ended_recently {
            if suppress_glide {
                self.set_gliding(false);
            } else {
                self.begin_glide_if_needed();
            }
        }

        if self.state.is_gliding {
            self.apply_momentum(delta_time);
        } else {
            self.state.last_input_delta = ZERO_VECTOR;
        }
    }

    fn begin_glide_if_needed(&mut self) {
        let speed = Self::magnitude(&self.state.velocity);
        if speed < env!("MINIMUM_GLIDE_VELOCITY").parse::<Float>().unwrap() {
            self.set_gliding(false);
            self.state.velocity = ZERO_VECTOR;
            return;
        } else {
            self.set_gliding(true);
            self.sync_to_virtual_position();
        }
    }

    pub fn apply_momentum(&mut self, delta_time: Float) {
        let decay_factor = max(
            0.0,
            1.0 - env!("GLIDE_DECAY_PER_SECOND").parse::<Float>().unwrap() * delta_time,
        );
        self.state.velocity.dx *= decay_factor;
        self.state.velocity.dy *= decay_factor;

        let momentum_delta = Vector {
            dx: self.state.velocity.dx * delta_time,
            dy: self.state.velocity.dy * delta_time,
        };

        self.state.previous_position = self.state.position;
        self.state.position.x += momentum_delta.dx;
        self.state.position.y += momentum_delta.dy;
        self.state.last_input_delta = momentum_delta;

        self.clamp_position_to_desktop();
        self.sync_to_virtual_position();

        let speed = Self::magnitude(&self.state.velocity);
        if speed
            < env!("MINIMUM_GLIDE_VELOCITY").parse::<Float>().unwrap()
                * env!("GLIDE_STOP_SPEED_FACTOR").parse::<Float>().unwrap()
        {
            self.set_gliding(false);
            self.state.velocity = ZERO_VECTOR;
            self.sync_to_virtual_position();
        }
    }

    fn magnitude(vector: &Vector) -> Float {
        (vector.dx * vector.dx + vector.dy * vector.dy).sqrt()
    }

    pub fn update_desktop_bounds(&mut self, bounds: Rect) {
        self.desktop_bounds = bounds;
        self.clamp_position_to_desktop();
    }

    pub fn sync_to_virtual_position(&mut self) {
        let target = self.state.position;
        let mtm = objc2::MainThreadMarker::new().expect("must be on the main thread");
        if let Some(screen) = NSScreen::mainScreen(mtm) {
            let display_id = unsafe { display::CGMainDisplayID() };
            let local_x = target.x - screen.frame().min().x;
            let local_y_from_bottom = target.y - screen.frame().min().y;
            let local_y = screen.frame().size.height - local_y_from_bottom;
            let _error = unsafe {
                display::CGDisplayMoveCursorToPoint(
                    display_id,
                    display::CGPoint {
                        x: local_x,
                        y: local_y,
                    },
                )
            };
        } else {
            return;
        }
    }

    pub fn sync_state(&mut self, physical_position: Point) {
        self.state.position = physical_position;
        self.state.previous_position = physical_position;
        self.state.last_input_delta = ZERO_VECTOR;
        self.last_physical_mouse_position = physical_position;
    }

    pub fn clamp_position_to_desktop(&mut self) {
        if self.desktop_bounds == Rect::null() {
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
        normalized_velocity: &Option<Vector>,
    ) -> Option<Vector> {
        if self.desktop_bounds == Rect::null() {
            return None;
        }
        if let Some(normalized_velocity) = normalized_velocity {
            let scaled = Vector {
                dx: normalized_velocity.dx
                    * self.desktop_bounds.size.width
                    * env!("TRACKPAD_VELOCITY_GAIN").parse::<Float>().unwrap(),
                dy: normalized_velocity.dy
                    * self.desktop_bounds.size.height
                    * env!("TRACKPAD_VELOCITY_GAIN").parse::<Float>().unwrap(),
            };
            return Some(Self::clamped_velocity(
                &scaled,
                env!("MAXIMUM_MOMENTUM_SPEED").parse::<Float>().unwrap(),
            ));
        } else {
            return None;
        }
    }

    fn clamped_velocity(vector: &Vector, max_magnitude: Float) -> Vector {
        let magnitude = Self::magnitude(vector);
        if magnitude > max_magnitude && magnitude > 0.0 {
            let scale = max_magnitude / magnitude;
            return Vector {
                dx: vector.dx * scale,
                dy: vector.dy * scale,
            };
        } else {
            return *vector;
        }
    }
}

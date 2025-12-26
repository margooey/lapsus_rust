pub mod controller;
pub mod engine;
pub mod helper;
pub mod tests;
pub mod trackpad;
pub mod utils;

fn main() {
    let mut controller = controller::Controller::new();
    controller.start();
    use std::{thread, time::Duration};
    loop {
        helper::fix_cursor();
        if controller.is_touching || controller.engine.state.is_gliding {
            controller.update_state();
        }
        thread::sleep(Duration::from_millis(2)); // 500hz
    }
}

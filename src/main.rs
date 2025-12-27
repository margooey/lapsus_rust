pub mod controller;
pub mod engine;
pub mod helper;
pub mod tests;
pub mod trackpad;
pub mod utils;

fn main() {
    use log::LevelFilter;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let mut controller = controller::Controller::new();
    controller.start();
    use std::{thread, time::Duration};
    loop {
        helper::fix_cursor();
        controller.update_state();
        thread::sleep(Duration::from_millis(2)); // 500hz
    }
}

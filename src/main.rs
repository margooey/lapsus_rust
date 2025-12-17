pub mod helper;
pub mod engine;
pub mod trackpad;
pub mod utils;
pub mod controller;

fn main() {
    helper::fix_cursor();
    controller::Controller::new();
    trackpad::start_stream();
    println!("Hello, world!");
}

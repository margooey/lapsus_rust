pub mod controller;
pub mod engine;
pub mod helper;
pub mod tests;
pub mod trackpad;
pub mod utils;

fn main() {
    // let controller = controller::Controller::new();
    use std::{thread, time::Duration};

    let mut monitor = trackpad::TrackpadMonitor::new();
    monitor.start();

    loop {
        helper::fix_cursor();
        let metrics = monitor.metrics();
        println!(
            "touching: {}, centroid: {:?}, velocity: ({:.4}, {:.4})",
            metrics.is_touching,
            metrics.centroid,
            metrics.normalized_velocity.dx,
            metrics.normalized_velocity.dy
        );
        thread::sleep(Duration::from_millis(2)); // 500hz
    }
}

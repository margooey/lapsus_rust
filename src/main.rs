pub mod controller;
pub mod engine;
pub mod helper;
pub mod tests;
pub mod trackpad;
pub mod utils;

use chrono::Local;
use log::LevelFilter;
use std::fs::File;
use std::io::Write;

fn main() {
    let target = Box::new(File::create("lapsus_log.txt").expect("Can't create file"));

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
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

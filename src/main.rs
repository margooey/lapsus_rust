pub mod controller;
pub mod engine;
pub mod tests;
pub mod trackpad;
pub mod utils;

use chrono::Local;
use cidre::cg::Float;
use log::LevelFilter;
use objc2::rc::autoreleasepool;
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSEventMask, NSStatusBar,
    NSVariableStatusItemLength,
};
use objc2_foundation::{MainThreadMarker, NSDate, NSDefaultRunLoopMode, NSString};
use std::env;
use std::fs::File;
use std::io::Write;
use std::sync::OnceLock;

pub struct Config {
    maximum_momentum_speed: f64,
    trackpad_velocity_gain: f64,
    glide_decay_per_second: f64,
    minimum_glide_velocity: f64,
    glide_stop_speed_factor: f64,
    velocity_smoothing: f64,
    min_dt: f64,
    multi_finger_suppression_deadline: f64,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        maximum_momentum_speed: env!("MAXIMUM_MOMENTUM_SPEED").parse::<Float>().unwrap(),
        trackpad_velocity_gain: env!("TRACKPAD_VELOCITY_GAIN").parse::<Float>().unwrap(),
        glide_decay_per_second: env!("GLIDE_DECAY_PER_SECOND").parse::<Float>().unwrap(),
        minimum_glide_velocity: env!("MINIMUM_GLIDE_VELOCITY").parse::<Float>().unwrap(),
        glide_stop_speed_factor: env!("GLIDE_STOP_SPEED_FACTOR").parse::<Float>().unwrap(),
        velocity_smoothing: env!("VELOCITY_SMOOTHING").parse::<Float>().unwrap(),
        min_dt: env!("MIN_DT").parse::<Float>().unwrap(),
        multi_finger_suppression_deadline: env!("MULTI_FINGER_SUPPRESSION_DEADLINE")
            .parse::<f64>()
            .unwrap(),
    })
}
fn main() {
    let target = Box::new(File::create("lapsus_log.txt").expect("Can't create file"));

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Info)
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

    let mtm = MainThreadMarker::new().expect("must be on the main thread");
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

    let status_bar = NSStatusBar::systemStatusBar();
    let status_item = status_bar.statusItemWithLength(NSVariableStatusItemLength);
    let button = status_item
        .button(mtm)
        .expect("status bar item should have a button");
    let title = NSString::from_str("Lapsus");
    button.setTitle(&title);
    let _status_item = status_item;

    let mut controller = controller::Controller::new();
    controller.start();

    loop {
        autoreleasepool(|_pool| {
            let _ = &_status_item;
            let expiration = NSDate::dateWithTimeIntervalSinceNow(config().min_dt);
            if let Some(event) = app.nextEventMatchingMask_untilDate_inMode_dequeue(
                NSEventMask::Any,
                Some(&expiration),
                unsafe { NSDefaultRunLoopMode },
                true,
            ) {
                app.sendEvent(&event);
            }
            app.updateWindows();
            utils::disable_local_event_suppression();
            controller.update_state();
        });
    }
}

use std::{
    f64::consts::{PI, TAU}, mem::MaybeUninit, process::exit, thread::{self, sleep}, time::Duration
};

use embedded_graphics::{
    pixelcolor::{Rgb888, raw::RawU24},
    prelude::RawData,
};
use tinybmp::Bmp;
use tracing_subscriber::EnvFilter;
use vex_sdk::*;
use vexide::prelude::Peripherals;

mod common;

common::create_main!(entry);

async fn entry(_p: Peripherals) {
    unsafe {
        loop {
            let mut touch_status = MaybeUninit::uninit();
            vexTouchDataGet(touch_status.as_mut_ptr());
            let touch_status = touch_status.assume_init();

            vexDisplayRectClear(0, 0, 512, 512);
            vexDisplayForegroundColor(match touch_status.lastEvent {
                V5_TouchEvent::kTouchEventPress => 0xFF_FF_FF,
                V5_TouchEvent::kTouchEventPressAuto => 0x00_FF_00,
                _ => 0xFF_00_FF,
            });
            vexDisplayCircleDraw(touch_status.lastXpos as i32, touch_status.lastYpos as i32, 10);

            vexDisplayRender(true, false);
            vexTasksRun();

            // Intentionally make it slow so you can see the events better.
            sleep(Duration::from_millis(100));
        }
    }
}

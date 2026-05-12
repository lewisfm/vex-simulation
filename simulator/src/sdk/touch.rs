//! Brain Screen Touchscreen

pub use vex_sdk::{V5_TouchEvent, V5_TouchStatus};

use crate::{display::DISPLAY, sdk::task::update_touch_status};

#[unsafe(no_mangle)]
pub extern "system" fn vexTouchUserCallbackSet(callback: extern "C" fn(V5_TouchEvent, i32, i32)) {
    super::sdk_unimplemented!("vexTouchUserCallbackSet");
}

/// Get the status of the brain's touchscreen.
///
/// # Safety
///
/// `status` must be valid for writes.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexTouchDataGet(status: *mut V5_TouchStatus) {
    update_touch_status(); // TODO: Should this be in a task instead?

    let display = DISPLAY.lock();
    unsafe {
        *status = display.touch();
    }
}

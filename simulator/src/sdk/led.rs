//! V5 LED
//!
//! This device is not sold by VEX and only exists as development hardware.

pub use vex_sdk::V5_DeviceLedColor;
use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor) {
    super::sdk_unimplemented!("vexDeviceLedSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32) {
    super::sdk_unimplemented!("vexDeviceLedRgbSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor {
    super::sdk_unimplemented!("vexDeviceLedGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceLedRgbGet");
    Default::default()
}

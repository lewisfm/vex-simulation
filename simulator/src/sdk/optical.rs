//! V5 Optical Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceOpticalGesture, V5_DeviceOpticalRaw, V5_DeviceOpticalRgb};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceOpticalHueGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceOpticalSatGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceOpticalBrightnessGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceOpticalProximityGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb) {
    super::sdk_unimplemented!("vexDeviceOpticalRgbGet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32) {
    super::sdk_unimplemented!("vexDeviceOpticalLedPwmSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceOpticalLedPwmGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceOpticalStatusGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw) {
    super::sdk_unimplemented!("vexDeviceOpticalRawGet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32) {
    super::sdk_unimplemented!("vexDeviceOpticalModeSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceOpticalModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalGestureGet(
    device: V5_DeviceT,
    pData: *mut V5_DeviceOpticalGesture,
) -> u32 {
    super::sdk_unimplemented!("vexDeviceOpticalGestureGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalGestureEnable(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceOpticalGestureEnable");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalGestureDisable(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceOpticalGestureDisable");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32) {
    super::sdk_unimplemented!("vexDeviceOpticalProximityThreshold");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double) {
    super::sdk_unimplemented!("vexDeviceOpticalIntegrationTimeSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceOpticalIntegrationTimeGet");
    Default::default()
}

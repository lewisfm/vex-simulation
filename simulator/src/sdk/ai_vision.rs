//! V5 AI Vision Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceAiVisionCode, V5_DeviceAiVisionColor, V5_DeviceAiVisionObject};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionClassNameGet(
    device: V5_DeviceT,
    id: i32,
    pName: *mut u8,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceAiVisionClassNameGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionCodeGet(
    device: V5_DeviceT,
    id: u32,
    pCode: *mut V5_DeviceAiVisionCode,
) -> bool {
    super::sdk_unimplemented!("vexDeviceAiVisionCodeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionCodeSet(device: V5_DeviceT, pCode: *mut V5_DeviceAiVisionCode) {
    super::sdk_unimplemented!("vexDeviceAiVisionCodeSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionColorGet(
    device: V5_DeviceT,
    id: u32,
    pColor: *mut V5_DeviceAiVisionColor,
) -> bool {
    super::sdk_unimplemented!("vexDeviceAiVisionColorGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionColorSet(
    device: V5_DeviceT,
    pColor: *mut V5_DeviceAiVisionColor,
) {
    super::sdk_unimplemented!("vexDeviceAiVisionColorSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionModeGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceAiVisionModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionModeSet(device: V5_DeviceT, mode: u32) {
    super::sdk_unimplemented!("vexDeviceAiVisionModeSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionObjectCountGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceAiVisionObjectCountGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionObjectGet(
    device: V5_DeviceT,
    indexObj: u32,
    pObject: *mut V5_DeviceAiVisionObject,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceAiVisionObjectGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionSensorSet(
    device: V5_DeviceT,
    brightness: c_double,
    contrast: c_double,
) {
    super::sdk_unimplemented!("vexDeviceAiVisionSensorSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceAiVisionStatusGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAiVisionTemperatureGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceAiVisionTemperatureGet");
    Default::default()
}

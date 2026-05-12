//! V5 Vision Sensor

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{
    V5_DeviceVisionObject, V5_DeviceVisionRgb, V5_DeviceVisionSignature, V5VisionLedMode,
    V5VisionMode, V5VisionWBMode, V5VisionWifiMode,
};

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode) {
    super::sdk_unimplemented!("vexDeviceVisionModeSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode {
    super::sdk_unimplemented!("vexDeviceVisionModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceVisionObjectCountGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionObjectGet(
    device: V5_DeviceT,
    index: u32,
    object: *mut V5_DeviceVisionObject,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceVisionObjectGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionSignatureSet(
    device: V5_DeviceT,
    signature: *mut V5_DeviceVisionSignature,
) {
    super::sdk_unimplemented!("vexDeviceVisionSignatureSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionSignatureGet(
    device: V5_DeviceT,
    id: u32,
    signature: *mut V5_DeviceVisionSignature,
) -> bool {
    super::sdk_unimplemented!("vexDeviceVisionSignatureGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8) {
    super::sdk_unimplemented!("vexDeviceVisionBrightnessSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8 {
    super::sdk_unimplemented!("vexDeviceVisionBrightnessGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionWhiteBalanceModeSet(
    device: V5_DeviceT,
    mode: V5VisionWBMode,
) {
    super::sdk_unimplemented!("vexDeviceVisionWhiteBalanceModeSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode {
    super::sdk_unimplemented!("vexDeviceVisionWhiteBalanceModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionWhiteBalanceSet(
    device: V5_DeviceT,
    color: V5_DeviceVisionRgb,
) {
    super::sdk_unimplemented!("vexDeviceVisionWhiteBalanceSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    super::sdk_unimplemented!("vexDeviceVisionWhiteBalanceGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode) {
    super::sdk_unimplemented!("vexDeviceVisionLedModeSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode {
    super::sdk_unimplemented!("vexDeviceVisionLedModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8) {
    super::sdk_unimplemented!("vexDeviceVisionLedBrigntnessSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8 {
    super::sdk_unimplemented!("vexDeviceVisionLedBrigntnessGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb) {
    super::sdk_unimplemented!("vexDeviceVisionLedColorSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb {
    super::sdk_unimplemented!("vexDeviceVisionLedColorGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode) {
    super::sdk_unimplemented!("vexDeviceVisionWifiModeSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode {
    super::sdk_unimplemented!("vexDeviceVisionWifiModeGet");
    Default::default()
}

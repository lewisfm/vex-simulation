//! V5 GPS

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceGpsAttitude, V5_DeviceGpsQuaternion, V5_DeviceGpsRaw};

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsReset(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceGpsReset");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceGpsHeadingGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceGpsDegreesGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsQuaternionGet(
    device: V5_DeviceT,
    data: *mut V5_DeviceGpsQuaternion,
) {
    super::sdk_unimplemented!("vexDeviceGpsQuaternionGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsAttitudeGet(
    device: V5_DeviceT,
    data: *mut V5_DeviceGpsAttitude,
    bRaw: bool,
) {
    super::sdk_unimplemented!("vexDeviceGpsAttitudeGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw) {
    super::sdk_unimplemented!("vexDeviceGpsRawGyroGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw) {
    super::sdk_unimplemented!("vexDeviceGpsRawAccelGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceGpsStatusGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsModeSet(device: V5_DeviceT, mode: u32) {
    super::sdk_unimplemented!("vexDeviceGpsModeSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsModeGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceGpsModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsDataRateSet(device: V5_DeviceT, rate: u32) {
    super::sdk_unimplemented!("vexDeviceGpsDataRateSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsOriginSet(device: V5_DeviceT, ox: c_double, oy: c_double) {
    super::sdk_unimplemented!("vexDeviceGpsOriginSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsOriginGet(
    device: V5_DeviceT,
    ox: *mut c_double,
    oy: *mut c_double,
) {
    super::sdk_unimplemented!("vexDeviceGpsOriginGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsRotationSet(device: V5_DeviceT, value: c_double) {
    super::sdk_unimplemented!("vexDeviceGpsRotationSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsRotationGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceGpsRotationGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsInitialPositionSet(
    device: V5_DeviceT,
    initial_x: c_double,
    initial_y: c_double,
    initial_rotation: c_double,
) {
    super::sdk_unimplemented!("vexDeviceGpsInitialPositionSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGpsErrorGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceGpsErrorGet");
    Default::default()
}

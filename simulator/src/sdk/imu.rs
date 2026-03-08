//! V5 Inertial Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_DeviceImuAttitude, V5_DeviceImuQuaternion, V5_DeviceImuRaw};

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuReset(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceImuReset");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceImuHeadingGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceImuDegreesGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuQuaternionGet(device: V5_DeviceT, data: *mut V5_DeviceImuQuaternion) {
    super::sdk_unimplemented!("vexDeviceImuQuaternionGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude) {
    super::sdk_unimplemented!("vexDeviceImuAttitudeGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {
    super::sdk_unimplemented!("vexDeviceImuRawGyroGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw) {
    super::sdk_unimplemented!("vexDeviceImuRawAccelGet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceImuStatusGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuTemperatureGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceImuTemperatureGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32) {
    super::sdk_unimplemented!("vexDeviceImuModeSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceImuModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32) {
    super::sdk_unimplemented!("vexDeviceImuDataRateSet");
}

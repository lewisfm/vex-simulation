//! ADI Devices

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{V5_AdiPortConfiguration, V5_DeviceBumperState};

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceAdiPortConfigSet(
    device: V5_DeviceT,
    port: u32,
    config: V5_AdiPortConfiguration,
) {
    super::sdk_unimplemented!("vexDeviceAdiPortConfigSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceAdiPortConfigGet(
    device: V5_DeviceT,
    port: u32,
) -> V5_AdiPortConfiguration {
    super::sdk_unimplemented!("vexDeviceAdiPortConfigGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32) {
    super::sdk_unimplemented!("vexDeviceAdiValueSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32 {
    super::sdk_unimplemented!("vexDeviceAdiValueGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceAdiAddrLedSet(
    device: V5_DeviceT,
    port: u32,
    pData: *mut u32,
    nOffset: u32,
    nLength: u32,
    options: u32,
) {
    super::sdk_unimplemented!("vexDeviceAdiAddrLedSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState {
    super::sdk_unimplemented!("vexDeviceBumperGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGyroReset(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceGyroReset");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceGyroHeadingGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceGyroDegreesGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceSonarValueGet");
    Default::default()
}

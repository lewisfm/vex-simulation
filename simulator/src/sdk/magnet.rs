//! V5 Workcell Electromagnet

use core::ffi::c_double;

pub use vex_sdk::V5_DeviceMagnetDuration;
use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32) {
    super::sdk_unimplemented!("vexDeviceMagnetPowerSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMagnetPowerGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetPickup(
    device: V5_DeviceT,
    duration: V5_DeviceMagnetDuration,
) {
    super::sdk_unimplemented!("vexDeviceMagnetPickup");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration) {
    super::sdk_unimplemented!("vexDeviceMagnetDrop");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMagnetTemperatureGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMagnetCurrentGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceMagnetStatusGet");
    Default::default()
}

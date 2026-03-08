//! Rangefinder/Lidar Sensor
//!
//! This sensor is not sold by VEX.

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceRangeValueGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceRangeValueGet");
    Default::default()
}

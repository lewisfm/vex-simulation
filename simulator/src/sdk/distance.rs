//! V5 Distance Sensor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceDistanceDistanceGet");
    9999
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceDistanceConfidenceGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceDistanceStatusGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceDistanceObjectSizeGet");
    -1
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceDistanceObjectVelocityGet");
    0.0
}

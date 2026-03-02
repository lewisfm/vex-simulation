//! V5 Rotation Sensor

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReset(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceAbsEncReset");
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32) {
    super::sdk_unimplemented!("vexDeviceAbsEncPositionSet");
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceAbsEncPositionGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceAbsEncVelocityGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceAbsEncAngleGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool) {
    super::sdk_unimplemented!("vexDeviceAbsEncReverseFlagSet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceAbsEncReverseFlagGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceAbsEncStatusGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32) {
    super::sdk_unimplemented!("vexDeviceAbsEncDataRateSet");
}

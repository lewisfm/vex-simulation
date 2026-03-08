//! CTE Workcell Pneumatics Control

pub use vex_sdk::V5_DevicePneumaticCtrl;
use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticActuationStatusGet(
    device: V5_DeviceT,
    ac1: *mut u16,
    ac2: *mut u16,
    ac3: *mut u16,
    ac4: *mut u16,
) -> u32 {
    super::sdk_unimplemented!("vexDevicePneumaticActuationStatusGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticCompressorSet(device: V5_DeviceT, bState: bool) {
    super::sdk_unimplemented!("vexDevicePneumaticCompressorSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticCtrlSet(
    device: V5_DeviceT,
    pCtrl: *mut V5_DevicePneumaticCtrl,
) {
    super::sdk_unimplemented!("vexDevicePneumaticCtrlSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticCylinderPwmSet(
    device: V5_DeviceT,
    id: u32,
    bState: bool,
    pwm: u8,
) {
    super::sdk_unimplemented!("vexDevicePneumaticCylinderPwmSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticCylinderSet(device: V5_DeviceT, id: u32, bState: bool) {
    super::sdk_unimplemented!("vexDevicePneumaticCylinderSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticPwmGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDevicePneumaticPwmGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticPwmSet(device: V5_DeviceT, pwm: u8) {
    super::sdk_unimplemented!("vexDevicePneumaticPwmSet");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicePneumaticStatusGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDevicePneumaticStatusGet");
    Default::default()
}

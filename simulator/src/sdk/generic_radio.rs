//! V5 Smart Radio

use core::ffi::{c_char, c_int};

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericRadioConnection(
    device: V5_DeviceT,
    link_id: *mut c_char,
    r#type: c_int,
    ov: bool,
) {
    super::sdk_unimplemented!("vexDeviceGenericRadioConnection");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericRadioWriteFree");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericRadioTransmit(
    device: V5_DeviceT,
    data: *const u8,
    size: u16,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericRadioTransmit");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceGenericRadioReceiveAvail");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericRadioReceive(
    device: V5_DeviceT,
    data: *mut u8,
    size: u16,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericRadioReceive");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceGenericRadioLinkStatus");
    Default::default()
}

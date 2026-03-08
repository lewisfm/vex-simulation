//! Smart Port Generic Serial Communication

use vex_sdk::V5_DeviceT;

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32) {
    super::sdk_unimplemented!("vexDeviceGenericSerialEnable");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32) {
    super::sdk_unimplemented!("vexDeviceGenericSerialBaudrate");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialWriteChar");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialWriteFree");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialTransmit(
    device: V5_DeviceT,
    buffer: *const u8,
    length: i32,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialTransmit");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialReadChar");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialPeekChar");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialReceiveAvail");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialReceive(
    device: V5_DeviceT,
    buffer: *mut u8,
    length: i32,
) -> i32 {
    super::sdk_unimplemented!("vexDeviceGenericSerialReceive");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericSerialFlush(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceGenericSerialFlush");
}

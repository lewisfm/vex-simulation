//! USB Serial Communication

#[unsafe(no_mangle)]
pub extern "system" fn vexSerialWriteChar(channel: u32, c: u8) -> i32 {
    super::sdk_unimplemented!("vexSerialWriteChar");
    Default::default()
}

/// # Safety
///
/// - `data` must be a valid pointer to a buffer of length `data_len`.
pub unsafe fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32 {
    super::sdk_unimplemented!("vexSerialWriteBuffer");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "system" fn vexSerialReadChar(channel: u32) -> i32 {
    super::sdk_unimplemented!("vexSerialReadChar");
    -1
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSerialPeekChar(channel: u32) -> i32 {
    super::sdk_unimplemented!("vexSerialPeekChar");
    -1
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSerialWriteFree(channel: u32) -> i32 {
    super::sdk_unimplemented!("vexSerialWriteFree");
    Default::default()
}

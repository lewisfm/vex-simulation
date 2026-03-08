//! V5 Controller

pub use vex_sdk::{V5_ControllerId, V5_ControllerIndex, V5_ControllerStatus};

#[unsafe(no_mangle)]
pub extern "system" fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32 {
    super::sdk_unimplemented!("vexControllerGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus {
    super::sdk_unimplemented!("vexControllerConnectionStatusGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32 {
    super::sdk_unimplemented!("vexControllerTextSet");
    Default::default()
}

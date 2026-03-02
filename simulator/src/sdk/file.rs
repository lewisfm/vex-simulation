//! Filesystem Access

use core::ffi::c_char;

pub use vex_sdk::{FIL, FRESULT};

#[unsafe(no_mangle)]
pub extern "C" fn vexFileMountSD() -> FRESULT {
    super::sdk_unimplemented!("vexFileMountSD");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileDirectoryGet(
    path: *const c_char,
    buffer: *mut c_char,
    len: u32,
) -> FRESULT {
    super::sdk_unimplemented!("vexFileDirectoryGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL {
    super::sdk_unimplemented!("vexFileOpen");
    core::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL {
    super::sdk_unimplemented!("vexFileOpenWrite");
    core::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL {
    super::sdk_unimplemented!("vexFileOpenCreate");
    core::ptr::null_mut()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileClose(fdp: *mut FIL) {
    super::sdk_unimplemented!("vexFileClose");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32 {
    super::sdk_unimplemented!("vexFileWrite");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileSize(fdp: *mut FIL) -> i32 {
    super::sdk_unimplemented!("vexFileSize");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT {
    super::sdk_unimplemented!("vexFileSeek");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32 {
    super::sdk_unimplemented!("vexFileRead");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileDriveStatus(drive: u32) -> bool {
    super::sdk_unimplemented!("vexFileDriveStatus");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileTell(fdp: *mut FIL) -> i32 {
    super::sdk_unimplemented!("vexFileTell");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileSync(fdp: *mut FIL) {
    super::sdk_unimplemented!("vexFileSync");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexFileStatus(filename: *const c_char) -> u32 {
    super::sdk_unimplemented!("vexFileStatus");
    Default::default()
}

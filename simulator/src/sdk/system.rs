//! VEXos System Functions

use core::ffi::{VaList, c_char, c_void};

pub use vex_sdk::{
    EX_SIG_MAGIC, V5_SIG_MAGIC, V5_SIG_OPTIONS_EXIT, V5_SIG_OPTIONS_INDG, V5_SIG_OPTIONS_NONE,
    V5_SIG_OPTIONS_THDG, V5_SIG_OWNER_PARTNER, V5_SIG_OWNER_SYS, V5_SIG_OWNER_VEX,
    V5_SIG_TYPE_USER, date, time, vcodesig,
};

#[unsafe(no_mangle)]
pub extern "system" fn vexPrivateApiDisable(sig: u32) {
    super::sdk_unimplemented!("vexPrivateApiDisable");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexStdlibMismatchError(installed_version: u32, required_version: u32) {
    super::sdk_unimplemented!("vexStdlibMismatchError");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexScratchMemoryLock() -> bool {
    super::sdk_unimplemented!("vexScratchMemoryLock");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexScratchMemoryUnlock() {
    super::sdk_unimplemented!("vexScratchMemoryUnlock");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemTimeGet() -> u32 {
    super::sdk_unimplemented!("vexSystemTimeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexGettime(pTime: *mut time) {
    super::sdk_unimplemented!("vexGettime");
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexGetdate(pDate: *mut date) {
    super::sdk_unimplemented!("vexGetdate");
    unsafe {
        *pDate = date {
            da_year: 2016,
            da_day: 16,
            da_mon: 11,
        }
    }
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemMemoryDump() {
    super::sdk_unimplemented!("vexSystemMemoryDump");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemDigitalIO(pin: u32, value: u32) {
    super::sdk_unimplemented!("vexSystemDigitalIO");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemStartupOptions() -> u32 {
    super::sdk_unimplemented!("vexSystemStartupOptions");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemExitRequest() {
    super::sdk_unimplemented!("vexSystemExitRequest");
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemHighResTimeGet() -> u64 {
    super::sdk_unimplemented!("vexSystemHighResTimeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemPowerupTimeGet() -> u64 {
    super::sdk_unimplemented!("vexSystemPowerupTimeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemLinkAddrGet() -> u32 {
    super::sdk_unimplemented!("vexSystemLinkAddrGet");
    0
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemTimerGet(timer: u32) -> u32 {
    super::sdk_unimplemented!("vexSystemTimerGet");
    0
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemUsbStatus() -> u32 {
    super::sdk_unimplemented!("vexSystemUsbStatus");
    0x3
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemTimerStop() {
    super::sdk_unimplemented!("vexSystemTimerStop");
}

#[unsafe(no_mangle)]
pub extern "system" fn vexSystemTimerClearInterrupt() {
    super::sdk_unimplemented!("vexSystemTimerClearInterrupt");
}

#[unsafe(no_mangle)]
pub extern "system" fn vexSystemTimerReinitForRtos(
    priority: u32,
    handler: extern "C" fn(data: *mut c_void),
) -> i32 {
    super::sdk_unimplemented!("vexSystemTimerReinitForRtos");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "system" fn vexSystemApplicationIRQHandler(ulICCIAR: u32) {
    super::sdk_unimplemented!("vexSystemApplicationIRQHandler");
}

#[unsafe(no_mangle)]
pub extern "system" fn vexSystemWatchdogReinitRtos() -> i32 {
    super::sdk_unimplemented!("vexSystemWatchdogReinitRtos");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemWatchdogGet() -> u32 {
    super::sdk_unimplemented!("vexSystemWatchdogGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "system" fn vexSystemUndefinedException() {
    unimplemented!()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemFIQInterrupt() {
    unimplemented!()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemIRQInterrupt() {
    unimplemented!()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemSWInterrupt() {
    unimplemented!()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemDataAbortInterrupt() {
    unimplemented!()
}
#[unsafe(no_mangle)]
pub extern "system" fn vexSystemPrefetchAbortInterrupt() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vex_vprintf(format: *const c_char, args: VaList) -> i32 {
    super::sdk_unimplemented!("vex_vprintf");
    -1
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vex_vsprintf(
    out: *mut c_char,
    format: *const c_char,
    args: VaList,
) -> i32 {
    super::sdk_unimplemented!("vex_vsprintf");
    -1
}
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vex_vsnprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    args: VaList,
) -> i32 {
    super::sdk_unimplemented!("vex_vsnprintf");
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_printf(format: *const c_char, args: ...) -> i32 {
    super::sdk_unimplemented!("vex_printf");
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_sprintf(out: *mut c_char, format: *const c_char, args: ...) -> i32 {
    super::sdk_unimplemented!("vex_sprintf");
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn vex_snprintf(
    out: *mut c_char,
    max_len: u32,
    format: *const c_char,
    args: ...
) -> i32 {
    super::sdk_unimplemented!("vex_snprintf");
    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexSystemVersion() -> u32 {
    super::sdk_unimplemented!("vexSystemVersion");
    Default::default()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexStdlibVersion() -> u32 {
    super::sdk_unimplemented!("vexStdlibVersion");
    Default::default()
}

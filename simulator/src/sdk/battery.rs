//! V5 Smart Battery

use core::ffi::c_double;

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryVoltageGet() -> i32 {
    super::sdk_unimplemented!("vexBatteryVoltageGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryCurrentGet() -> i32 {
    super::sdk_unimplemented!("vexBatteryCurrentGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryTemperatureGet() -> c_double {
    super::sdk_unimplemented!("vexBatteryTemperatureGet");
    Default::default()
}

#[unsafe(no_mangle)]
pub extern "C" fn vexBatteryCapacityGet() -> c_double {
    super::sdk_unimplemented!("vexBatteryCapacityGet");
    Default::default()
}

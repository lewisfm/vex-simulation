//! V5 Smart Motor

use core::ffi::c_double;

use vex_sdk::V5_DeviceT;
pub use vex_sdk::{
    V5_DeviceMotorPid, V5MotorBrakeMode, V5MotorControlMode, V5MotorEncoderUnits, V5MotorGearset,
};

#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32) {
    super::sdk_unimplemented!("vexDeviceMotorVelocitySet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorVelocityGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorActualVelocityGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorDirectionGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode) {
    super::sdk_unimplemented!("vexDeviceMotorModeSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode {
    super::sdk_unimplemented!("vexDeviceMotorModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32) {
    super::sdk_unimplemented!("vexDeviceMotorPwmSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorPwmGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32) {
    super::sdk_unimplemented!("vexDeviceMotorCurrentLimitSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorCurrentLimitGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorCurrentGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorPowerGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorTorqueGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorEfficiencyGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorTemperatureGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceMotorOverTempFlagGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceMotorCurrentLimitFlagGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceMotorZeroVelocityFlagGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceMotorZeroPositionFlagGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool) {
    super::sdk_unimplemented!("vexDeviceMotorReverseFlagSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool {
    super::sdk_unimplemented!("vexDeviceMotorReverseFlagGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits) {
    super::sdk_unimplemented!("vexDeviceMotorEncoderUnitsSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits {
    super::sdk_unimplemented!("vexDeviceMotorEncoderUnitsGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode) {
    super::sdk_unimplemented!("vexDeviceMotorBrakeModeSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode {
    super::sdk_unimplemented!("vexDeviceMotorBrakeModeGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double) {
    super::sdk_unimplemented!("vexDeviceMotorPositionSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorPositionGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPositionRawGet(device: V5_DeviceT, timestamp: *mut u32) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorPositionRawGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPositionReset(device: V5_DeviceT) {
    super::sdk_unimplemented!("vexDeviceMotorPositionReset");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double {
    super::sdk_unimplemented!("vexDeviceMotorTargetGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double) {
    super::sdk_unimplemented!("vexDeviceMotorServoTargetSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorAbsoluteTargetSet(
    device: V5_DeviceT,
    position: c_double,
    veloctiy: i32,
) {
    super::sdk_unimplemented!("vexDeviceMotorAbsoluteTargetSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorRelativeTargetSet(
    device: V5_DeviceT,
    position: c_double,
    velocity: i32,
) {
    super::sdk_unimplemented!("vexDeviceMotorRelativeTargetSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceMotorFaultsGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32 {
    super::sdk_unimplemented!("vexDeviceMotorFlagsGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32) {
    super::sdk_unimplemented!("vexDeviceMotorVoltageSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorVoltageGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset) {
    super::sdk_unimplemented!("vexDeviceMotorGearingSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset {
    super::sdk_unimplemented!("vexDeviceMotorGearingGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32) {
    super::sdk_unimplemented!("vexDeviceMotorVoltageLimitSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32 {
    super::sdk_unimplemented!("vexDeviceMotorVoltageLimitGet");
    Default::default()
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32) {
    super::sdk_unimplemented!("vexDeviceMotorVelocityUpdate");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid) {
    super::sdk_unimplemented!("vexDeviceMotorPositionPidSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid) {
    super::sdk_unimplemented!("vexDeviceMotorVelocityPidSet");
}
#[unsafe(no_mangle)]
pub extern "C" fn vexDeviceMotorExternalProfileSet(
    device: V5_DeviceT,
    position: c_double,
    velocity: i32,
) {
    super::sdk_unimplemented!("vexDeviceMotorExternalProfileSet");
}

//! V5 Distance Sensor

use core::ffi::c_double;

use roboscope_ipc::snapshot::DistanceSnapshot;
use vex_sdk::V5_DeviceT;

use crate::device::DEVICES;

/// Get the measured distance.
///
/// # Panics
///
/// Panics if the device handle is invalid.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    let mut ctx = DEVICES.lock();

    if let Some((snapshot, _)) = ctx.resolve::<DistanceSnapshot>(device) {
        snapshot.distance
    } else {
        9999
    }
}

/// Get the measured confidence.
///
/// # Panics
///
/// Panics if the device handle is invalid.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    let mut ctx = DEVICES.lock();

    if let Some((snapshot, _)) = ctx.resolve::<DistanceSnapshot>(device) {
        snapshot.confidence
    } else {
        0
    }
}

/// Get the sensor status.
///
/// # Panics
///
/// Panics if the device handle is invalid.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    let mut ctx = DEVICES.lock();

    if let Some((snapshot, _)) = ctx.resolve::<DistanceSnapshot>(device) {
        snapshot.status
    } else {
        0
    }
}

/// Get the measured object size.
///
/// # Panics
///
/// Panics if the device handle is invalid.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    let mut ctx = DEVICES.lock();

    if let Some((snapshot, _)) = ctx.resolve::<DistanceSnapshot>(device) {
        snapshot.object_size
    } else {
        -1
    }
}

/// Get the measured velocity.
///
/// # Panics
///
/// Panics if the device handle is invalid.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    let mut ctx = DEVICES.lock();

    if let Some((snapshot, _)) = ctx.resolve::<DistanceSnapshot>(device) {
        snapshot.object_velocity
    } else {
        0.0
    }
}

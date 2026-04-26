//! V5 Distance Sensor

use core::ffi::c_double;

use roboscope_ipc::snapshot::DistanceSnapshot;
use vex_sdk::V5_DeviceT;

use crate::device::DEVICES;

/// Get the measured distance.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { DEVICES.get_by_handle_unchecked(device) }
        .as_ref()
        .lock();

    if let Some(snapshot) = device.readings::<DistanceSnapshot>() {
        snapshot.distance
    } else {
        9999
    }
}

/// Get the measured confidence.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { DEVICES.get_by_handle_unchecked(device) }
        .as_ref()
        .lock();

    if let Some(snapshot) = device.readings::<DistanceSnapshot>() {
        snapshot.confidence
    } else {
        0
    }
}

/// Get the sensor status.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32 {
    let device = unsafe { DEVICES.get_by_handle_unchecked(device) }
        .as_ref()
        .lock();

    if let Some(snapshot) = device.readings::<DistanceSnapshot>() {
        snapshot.status
    } else {
        0
    }
}

/// Get the measured object size.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32 {
    let device = unsafe { DEVICES.get_by_handle_unchecked(device) }
        .as_ref()
        .lock();

    if let Some(snapshot) = device.readings::<DistanceSnapshot>() {
        snapshot.object_size
    } else {
        -1
    }
}

/// Get the measured velocity.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double {
    let device = unsafe { DEVICES.get_by_handle_unchecked(device) }
        .as_ref()
        .lock();

    if let Some(snapshot) = device.readings::<DistanceSnapshot>() {
        snapshot.object_velocity
    } else {
        0.0
    }
}

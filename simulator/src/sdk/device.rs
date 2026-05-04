//! V5 Smart Devices

use core::ffi::{c_double, c_int};
use std::{mem::offset_of, ptr};

use roboscope_ipc::{
    SMART_DEVICES_COUNT,
    snapshot::{DeviceSnapshot, GenericSnapshot},
};
pub use vex_sdk::{V5_DeviceT, V5_DeviceType};

use crate::device::{DEVICES, Devices, NUM_DEVICES_HANDLES};

/// Get the number of device handles tracked by the system.
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicesGetNumber() -> u32 {
    NUM_DEVICES_HANDLES as u32
}

/// Count the number of devices of the given type which are currently connected.
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32 {
    let mut count = 0;
    if let Some(readings) = DEVICES.lock().readings.as_ref() {
        for device in &readings.snapshots {
            if device.kind() == device_type {
                count += 1;
            }
        }
    }
    count
}

/// Get a pointer to the internal array of devices.
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicesGet() -> V5_DeviceT {
    DEVICES.lock().smart_devices.as_ptr().cast_mut().cast()
}

/// Get a device handle using its index.
///
/// Returns null if the device doesn't exist.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT {
    if let Some(device) = DEVICES.lock().smart_devices.get(index as usize) {
        ptr::from_ref(device).cast_mut().cast()
    } else {
        ptr::null_mut()
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceFlagsGetByIndex(index: u32) -> u32 {
    // Unclear if anyone knows what this does concretely, it's probably some sort of internal state.
    // Better not to touch unless anyone is actually using it for something.
    super::sdk_unimplemented!("vexDeviceFlagsGetByIndex");
    Default::default()
}

/// Write the types of all connected devices to the `devices` parameter.
///
/// Returns the number of devices statuses that were written to the array, or -1 if `devices` was
/// null.
///
/// # Safety
///
/// `devices` must either be null or valid for writes from offsets zero (inclusive) through
/// [`vex_sdk::V5_MAX_DEVICE_PORTS`] (non-inclusive).
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32 {
    if devices.is_null() {
        return -1;
    }

    if let Some(readings) = &DEVICES.lock().readings {
        let snapshots = &readings.payload().snapshots;

        for (i, reading) in snapshots.iter().enumerate() {
            // SAFETY: devices is valid for writes in this range
            unsafe {
                devices.add(i).write(reading.kind());
            }
        }
    } else {
        // No data from the physics backend yet, fall back to no devices available.
        for i in 0..SMART_DEVICES_COUNT {
            unsafe {
                devices.add(i).write(V5_DeviceType::kDeviceTypeNoSensor);
            }
        }
    }

    SMART_DEVICES_COUNT as i32
}

/// Get the low-res timestamp of when this device's data was last processed.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceGetTimestamp(_device: V5_DeviceT) -> u32 {
    let ctx = DEVICES.lock();
    ctx.readings_timestamp() // Currently all devices have the same timestamp.
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericValueGet(device: V5_DeviceT) -> i32 {
    let ctx = DEVICES.lock();
    let readings = ctx
        .readings_for::<GenericSnapshot>(device)
        .copied()
        .unwrap_or_default();

    readings.value
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceTypeGetByIndex(index: u32) -> V5_DeviceType {
    let index = index as usize;
    let ctx = DEVICES.lock();

    if index < SMART_DEVICES_COUNT
        && let Some(snapshot) = ctx.readings_for::<DeviceSnapshot>(index)
    {
        snapshot.kind()
    } else {
        V5_DeviceType::kDeviceTypeNoSensor
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceButtonStateGet() -> c_int {
    // Possibly a no-op? There's no way to specify what button, and it's definitely not the power
    // button.
    super::sdk_unimplemented!("vexDeviceButtonStateGet");
    Default::default()
}

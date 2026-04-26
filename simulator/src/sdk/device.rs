//! V5 Smart Devices

use core::ffi::{c_double, c_int};

use roboscope_ipc::snapshot::GenericSnapshot;
pub use vex_sdk::{V5_DeviceT, V5_DeviceType};

use crate::device::DEVICES;

/// Get the number of device handles tracked by the system.
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicesGetNumber() -> u32 {
    DEVICES.smart_devices.len() as u32
}

/// Count the number of devices of the given type which are currently connected.
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32 {
    let mut count = 0;
    for device in &DEVICES.smart_devices {
        if device.kind() == device_type {
            count += 1;
        }
    }
    count
}

/// Get a pointer to the internal array of devices.
#[unsafe(no_mangle)]
pub extern "system" fn vexDevicesGet() -> V5_DeviceT {
    DEVICES.smart_devices.as_ptr().cast_mut().cast()
}

/// Get a device handle using its index.
///
/// Returns null if the device doesn't exist.
#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT {
    DEVICES.handle_for(index).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceFlagsGetByIndex(index: u32) -> u32 {
    // Unclear if anyone knows what this does concretely, it's probably some sort of internal state
    super::sdk_unimplemented!("vexDeviceFlagsGetByIndex");
    Default::default()
}

/// Write the types of all connected devices to the `devices` parameter.
///
/// # Safety
///
/// `devices` must be null or valid for writes from offsets zero (inclusive) through
/// [`vex_sdk::V5_MAX_DEVICE_PORTS`] (non-inclusive).
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32 {
    if devices.is_null() {
        return -1;
    }

    for (i, dev) in DEVICES.smart_devices.iter().enumerate() {
        // SAFETY: devices is valid for writes in this range
        unsafe {
            devices.add(i).write(dev.kind());
        }
    }

    DEVICES.smart_devices.len() as i32
}

/// Get the low-res timestamp of when this device's data was last processed.
///
/// # Safety
///
/// The device handle must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32 {
    // SAFETY: caller guarantees device is valid
    let device = unsafe { DEVICES.get_by_handle(device).unwrap_unchecked() };
    let data = device.as_ref().lock();
    data.timestamp()
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceGenericValueGet(device: V5_DeviceT) -> i32 {
    // SAFETY: caller guarantees device is valid
    let device = unsafe { DEVICES.get_by_handle(device).unwrap_unchecked() };
    let mut data = device.as_ref().lock();

    let readings = data.readings::<GenericSnapshot>().copied().unwrap_or_default();

    readings.value
}

#[unsafe(no_mangle)]
pub extern "system" fn vexDeviceTypeGetByIndex(index: u32) -> V5_DeviceType {
    if let Some(device) = DEVICES.smart_devices.get(index as usize) {
        device.kind()
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

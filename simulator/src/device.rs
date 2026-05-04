//! Smart device registry and state management.

#![allow(unused)]

use std::{
    ptr,
    sync::{
        Arc, LazyLock, OnceLock,
        atomic::{AtomicU32, Ordering},
    },
    thread,
    time::{Duration, Instant, SystemTime},
};

use derive_more::{AsRef, From, TryInto};
use parking_lot::{Mutex, MutexGuard};
use roboscope_ipc::{
    PHYSICS_UPDATE_PERIOD, SMART_DEVICES_COUNT, Sample, SimServices, Subscriber,
    snapshot::{DeviceReadings, DeviceSnapshot},
};
use tracing::{debug, trace};
use vex_sdk::{V5_Device, V5_DeviceT, V5_DeviceType};

use crate::sdk::vexSystemTimeGet;

pub static TIMESTAMP_EPOCH: LazyLock<SystemTime> = LazyLock::new(SystemTime::now);
pub static DEVICES: Mutex<Devices> = Mutex::new(Devices::new());
pub static DEVICES_STREAM: OnceLock<DevicesStream> = OnceLock::new();
pub const NUM_DEVICES_HANDLES: usize = 23;

/// Access to device readings and device I/O.
///
/// This data is exposed via the VEX SDK implementation in the [`crate::sdk`] module.
pub struct Devices {
    pub readings: Option<Sample<DeviceReadings>>,
    /// The state for each smart device.
    pub smart_devices: [V5DeviceState; NUM_DEVICES_HANDLES],
}

impl Devices {
    pub const fn new() -> Self {
        Self {
            readings: None,
            smart_devices: [const { V5DeviceState::new() }; _],
        }
    }

    /// Get the timestamp at which the most recent device packets were created.
    ///
    /// On a real device, this value indicates when CPU0 parsed the device packet, which happens
    /// every 10ms. In the simulator, the physics backend takes the role of CPU0 in providing us
    /// with physics updates every 10ms (see [`PHYSICS_UPDATE_PERIOD`]), so the timestamp indicates
    /// when the physics backend created the packet.
    ///
    /// Just like [`vexSystemTimeGet`], device timestamps are millisecond precision and timestamp
    /// `0` occurs at the start of the robot program.
    pub fn readings_timestamp(&self) -> u32 {
        if let Some(readings) = &self.readings {
            let creation_time = readings.system_time();
            let time_since_epoch = creation_time
                .duration_since(*TIMESTAMP_EPOCH)
                .unwrap_or_default();
            time_since_epoch.as_millis() as u32
        } else {
            0
        }
    }

    /// Apply the given device readings.
    pub fn update_readings(&mut self, sample: Sample<DeviceReadings>) {
        trace!("Committing queued sample");
        self.readings = Some(sample);
    }

    pub fn handle_for(&self, port: u32) -> Option<V5_DeviceT> {
        let device = self.smart_devices.get(port as usize)?;
        Some(ptr::from_ref(device).cast_mut().cast())
    }

    /// Try to resolve the given device handle to readings for a certain kind of device.
    ///
    /// Returns `None` if the device is the wrong type.
    ///
    /// # Panics
    ///
    /// Panics if the device handle is invalid.
    pub fn readings_for<T>(&self, device: impl DeviceResolvable) -> Option<&T>
    where
        for<'a> &'a T: TryFrom<&'a DeviceSnapshot>,
    {
        let readings = self.readings.as_ref()?;
        let snapshot = &readings.snapshots[device.to_port(self)];
        snapshot.try_into().ok()
    }
}

/// Can be resolved to a smart port index.
trait DeviceResolvable {
    fn to_port(&self, devices: &Devices) -> usize;
}

/// A direct smart port index.
impl DeviceResolvable for usize {
    fn to_port(&self, devices: &Devices) -> usize {
        *self
    }
}

/// A device handle.
impl DeviceResolvable for *mut V5_Device {
    /// Convert the device handle to a port number.
    ///
    /// # Panics
    ///
    /// Panics if the device handle is unaligned or out of bounds.
    fn to_port(&self, devices: &Devices) -> usize {
        let offset = self
            .addr()
            .checked_sub(devices.smart_devices.as_ptr().addr())
            .unwrap_or(usize::MAX); // Force an out-of-bounds panic.

        let index = offset / size_of::<V5DeviceState>();
        if index >= devices.smart_devices.len() {
            panic!("Device handle is out of bounds");
        }

        if offset % size_of::<V5DeviceState>() != 0 {
            panic!("Device handle is improperly aligned");
        }

        index
    }
}

pub struct V5DeviceState {
    _placeholder: u32,
}

impl V5DeviceState {
    pub const fn new() -> Self {
        Self { _placeholder: 0 }
    }
}

/// Updates the global device registry with the latest readings.
pub struct DevicesStream {
    readings: Subscriber<DeviceReadings>,
}

impl DevicesStream {
    /// Subscribe to the system device readings channel using the given IPC client.
    pub fn new(ipc: Arc<SimServices>) -> anyhow::Result<Self> {
        let captures = ipc.device_readings()?.subscriber_builder().create()?;

        Ok(Self { readings: captures })
    }

    /// Publish the latest readings to the process's device registry.
    pub fn update(&self) -> anyhow::Result<()> {
        if let Some(sample) = self.readings.receive()? {
            DEVICES.lock().update_readings(sample);
        }

        Ok(())
    }
}

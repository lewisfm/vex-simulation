//! Commands sent from user code to devices.
//!
//! These events are intended to be consumed by physics simulators or device controllers.

use derive_more::{From, TryInto};
use iceoryx2::prelude::ZeroCopySend;
use vex_sdk::V5MotorGearset;

use crate::SMART_DEVICES_COUNT;

/// A packet which reports the most recent commands from a robot controller to its peripherals.
#[derive(Debug, Copy, Clone, PartialEq, ZeroCopySend, Default)]
#[repr(C)]
pub struct RobotOutputs(pub [DeviceCommand; SMART_DEVICES_COUNT]);

/// A type-erased device command.
#[derive(Debug, Copy, Clone, PartialEq, ZeroCopySend, Default, From, TryInto)]
#[repr(C)]
pub enum DeviceCommand {
    #[default]
    Empty,
    Motor(MotorCommand),
}

#[derive(Debug, Copy, Clone, PartialEq, ZeroCopySend, Default)]
#[repr(C)]
pub struct MotorCommand {
    /// Requested mode of motor control
    mode: MotorControlMode,
    /// Presumed gearset of motor
    gearset: MotorGearset,
    /// Target position in ticks
    target_position: f64,
    /// Target velocity in RPM
    target_velocity: i32,
    /// PWM control % with the range `[-100, 100]`
    pwm: i32,
    /// Requested current limit in milliamperes.
    current_limit: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, ZeroCopySend, Default, From)]
#[repr(C)]
pub enum MotorControlMode {
    /// The motor will be set to idle.
    #[default]
    Off,
    /// The motor will apply a braking force.
    Brake,
    /// The motor will use position control to hold its current rotation.
    Hold,
    /// The motor will use position control to rotate to the target position.
    Profile,
    /// The motor will use velocity control to rotate at the target velocity.
    Velocity,
    /// The motor will use pulse-width modulation to rotate using a percent of the max voltage.
    Pwm,
}

#[derive(Debug, Copy, Clone, PartialEq, ZeroCopySend, Default, From)]
#[repr(C)]
pub enum MotorGearset {
    /// 36&times; slower than the base motor speed. (Red cartridge)
    Ratio36To1,
    /// The V5's "Standard" gearing, 18&times; slower than the base motor speed. (Green cartridge)
    #[default]
    Ratio18To1,
    /// 6&times; slower than the base motor speed. (Blue cartridge)
    Ratio06To1,
}

impl MotorGearset {
    pub const fn new(gearset: V5MotorGearset) -> Option<Self> {
        Some(match gearset {
            V5MotorGearset::kMotorGearSet_06 => Self::Ratio06To1,
            V5MotorGearset::kMotorGearSet_18 => Self::Ratio18To1,
            V5MotorGearset::kMotorGearSet_36 => Self::Ratio36To1,
            _ => return None,
        })
    }

    #[must_use]
    pub const fn multiplier(self) -> f64 {
        match self {
            Self::Ratio36To1 => 1.0 / 36.0,
            Self::Ratio18To1 => 1.0 / 18.0,
            Self::Ratio06To1 => 1.0 / 6.0,
        }
    }
}

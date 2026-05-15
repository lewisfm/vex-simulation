use roboscope_ipc::{
    Config, SimServices,
    cmd::{DeviceCommand, MotorControlMode, RobotOutputs},
    snapshot::{DeviceReadings, DistanceSnapshot, MotorSnapshot},
};

fn main() {
    let sim = SimServices::join(Some("Motor example"), &Config::default()).unwrap();

    let mut readings = DeviceReadings::default();

    // This is a really crude simulation of a motor that basically just does "voltage = velocity".
    let mut motor_pwm = 0;
    let mut motor_position = 0;

    sim.publish_device_readings(|cmds| {
        if let Some(cmds) = cmds
            && let DeviceCommand::Motor(motor) = cmds[0]
        {
            println!("motor {motor:?}");
            if motor.mode == MotorControlMode::Pwm {
                motor_pwm = motor.pwm;
            } else {
                motor_pwm = 0;
            }
        }

        motor_position += motor_pwm * 10;
        println!("Motor position: {motor_position}");

        readings.snapshots[0] = MotorSnapshot {
            applied_voltage: motor_pwm * 120, // 100% PWM = 12_000 mV
            raw_position: motor_position,
            ..Default::default()
        }
        .into();

        readings
    })
    .unwrap();
}

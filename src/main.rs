#![allow(unused)] // disbales error if code is unused > good for debugging because cleaner terminal
#![allow(non_snake_case)] // disables error if variable or file is non snake case > annoying because of filename

extern crate ev3dev_lang_rust;
use ev3dev_lang_rust::motors::{MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::{ColorSensor, CompassSensor, IrSeekerSensor, UltrasonicSensor};
use ev3dev_lang_rust::{Ev3Button, Ev3Result};

use WRO_goalkeeper_v002::OmniController;

use std::f64;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Ev3Result<()> {
    // get motors
    let motor_a = MediumMotor::get(MotorPort::OutA)?;
    let motor_b = MediumMotor::get(MotorPort::OutB)?;
    let motor_c = MediumMotor::get(MotorPort::OutC)?;
    // init omni controller (motors)
    let omni_controller = OmniController::<60, 180, -60>::new(motor_a, motor_b, motor_c);
    // get sensors
    let button = Ev3Button::new()?;
    let ir_seeker = IrSeekerSensor::find()?;
    let mut compass = CompassSensor::find()?;
    // init sensors
    ir_seeker.set_mode_ac_all()?;
    compass.start_calibration()?;
    println!("Turn the robot 360°, then press the button in the middle");
    loop {
        button.process();
        if button.is_enter() {
            break;
        } else {
            sleep(Duration::from_millis(100));
        }
    }
    compass.stop_calibration()?;
    sleep(Duration::from_millis(100));
    compass.set_zero()?;

    Ok(())
}

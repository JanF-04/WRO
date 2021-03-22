#![allow(unused)] // disbales error if code is unused > good for debugging because cleaner terminal
#![allow(non_snake_case)] // disables error if variable or file is non snake case > annoying because of filename

use ev3dev_lang_rust::motors::MediumMotor;
use ev3dev_lang_rust::Ev3Result;

// in main.rs: use `<name of own crate>::OmniController` to import
pub struct OmniController<const ADIR: i16, const BDIR: i16, const CDIR: i16> {
    motor_a: MediumMotor,
    motor_b: MediumMotor,
    motor_c: MediumMotor,
    default_directions: [[i32;3];6],
}

// !!! you might have to use the beta or nightly channel for this to compile
impl<const ADIR: i16, const BDIR: i16, const CDIR: i16> OmniController<ADIR, BDIR, CDIR> {
    /// Creates a new Omni-Wheel controller.
    ///
    /// The const generic parameters are the angles (in degrees) the motors are facing in relative to the
    /// front direction.
    ///
    /// #Examples
    /// ```norun
    /// let motor_a = MediumMotor::get(MotorPort::OutA)?;
    /// let motor_b = MediumMotor::get(MotorPort::OutB)?;
    /// let motor_c = MediumMotor::get(MotorPort::OutC)?;
    /// let omni_controller = OmniController::<60, 180, -60>::new(motor_a, motor_b, motor_c);
    /// ```
    pub fn new(motor_a: MediumMotor, motor_b: MediumMotor, motor_c: MediumMotor) -> Self {
        motor_a.run_direct().unwrap();
        motor_b.run_direct().unwrap();
        motor_c.run_direct().unwrap();
        let mut default_directions: [[i32;3];6] = [[0;3];6];
        let a_dir = (ADIR as f64).to_radians();
        let b_dir = (BDIR as f64).to_radians();
        let c_dir = (CDIR as f64).to_radians();
        let directions: [f64;6] = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0];
        for (i, dir) in directions.iter().enumerate() {
            let dir = dir.to_radians();
            default_directions[i][0] = ((dir - a_dir).sin() * 100.0) as i32;
            default_directions[i][1] = ((dir - b_dir).sin() * 100.0) as i32;
            default_directions[i][2] = ((dir - c_dir).sin() * 100.0) as i32;
        }
        OmniController {
            motor_a,
            motor_b,
            motor_c,
            default_directions,
        }
    }

    pub fn drive_direction(&self, direction: f64, speed: f64) -> Ev3Result<()> {
        let a_dir = (ADIR as f64).to_radians();
        let b_dir = (BDIR as f64).to_radians();
        let c_dir = (CDIR as f64).to_radians();
        let direction = direction.to_radians();

        let a_speed = (direction - a_dir).sin() * speed;
        let b_speed = (direction - b_dir).sin() * speed;
        let c_speed = (direction - c_dir).sin() * speed;

        self.set_motors(a_speed as i32, b_speed as i32, c_speed as i32)?;

        Ok(())
    }

    fn set_motors(&self, a: i32, b: i32, c: i32) -> Ev3Result<()> {
        self.motor_a.set_duty_cycle_sp(a)?;
        self.motor_b.set_duty_cycle_sp(b)?;
        self.motor_c.set_duty_cycle_sp(c)?;
        Ok(())
    }

    pub fn turn(&self, speed: i32) ->  Ev3Result<()> {
        self.set_motors(speed, speed, speed)?;
        Ok(())
    }

    fn drive_sector(&self, dir: i32) -> Ev3Result<()> {
        let dir = dir as usize;
        if (dir > 0 && dir < self.default_directions.len()) {
            let a_speed = self.default_directions[dir][0];
            let b_speed = self.default_directions[dir][1];
            let c_speed = self.default_directions[dir][2];
            self.set_motors(a_speed, b_speed, c_speed)?;
        } 
        Ok(())
    }

    pub fn stop(&self) -> Ev3Result<()> {
        self.motor_a.stop()?;
        self.motor_b.stop()?;
        self.motor_c.stop()?;
        Ok(())
    }
}
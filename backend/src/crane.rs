// Here we will build mock actuators and our mock crane.
// In a real application, we would be reading from a sensor or performing some state estimation.

mod actuators;
use actuators::{MockActuator, LinearActuator, RotaryActuator};

pub struct Crane {
    lift: LinearActuator,
    swing: RotaryActuator,
    elbow: RotaryActuator,
    wrist: RotaryActuator,
    gripper: LinearActuator,
    d3: f64,
    d4: f64,
    r3: f64,
    r4: f64,
}

impl Crane {
    pub fn new(d2_max: f64, d3: f64, d4: f64, r3: f64, r4: f64) -> Self {
        const GENERIC_ROTARY_KP: f64 = 3.0;
        const GENERIC_ROTARY_KD: f64 = 5.0;

        const GENERIC_LINEAR_KP: f64 = 2.5;
        const GENERIC_LINEAR_KD: f64 = 3.0;

        Self {
            swing: RotaryActuator::new(
                0.0, -180.0, 180.0,
                GENERIC_ROTARY_KP, 0.0, GENERIC_ROTARY_KD,
            ),
            lift: LinearActuator::new(
                d2_max, 0.0, d2_max,
                GENERIC_LINEAR_KP, 0.0, GENERIC_LINEAR_KD,
            ),
            elbow: RotaryActuator::new(
                0.0, -180.0, 180.0,
                GENERIC_ROTARY_KP, 0.0, GENERIC_ROTARY_KD,
            ),
            wrist: RotaryActuator::new(
                0.0, -180.0, 180.0,
                GENERIC_ROTARY_KP, 0.0, GENERIC_ROTARY_KD,
            ),
            gripper: LinearActuator::new(
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ),
            d3,
            d4,
            r3,
            r4,
        }
    }

    pub fn get_state(&self) -> CraneState {
        CraneState {
            swing_deg: self.swing.get_position(),
            lift_mm: self.lift.get_position() * 1000.0,
            elbow_deg: self.elbow.get_position(),
            wrist_deg: self.wrist.get_position(),
            gripper_mm: self.gripper.get_position() * 1000.0,
        }
    }

    pub fn set_velocity(&mut self, swing_v: f64, lift_v: f64, elbow_v: f64, wrist_v: f64, gripper_v: f64) {
        self.swing.set_velocity(swing_v);
        self.lift.set_velocity(lift_v);
        self.elbow.set_velocity(elbow_v);
        self.wrist.set_velocity(wrist_v);
        self.gripper.set_velocity(gripper_v);
    }

    pub fn set_actuator_setpoints(&mut self, swing: f64, lift: f64, elbow: f64, wrist: f64, gripper: f64) {
        self.swing.set_setpoint(swing);
        self.lift.set_setpoint(lift);
        self.elbow.set_setpoint(elbow);
        self.wrist.set_setpoint(wrist);
        self.gripper.set_setpoint(gripper)
    }

    pub fn update_state(&mut self, dt: f64) {
        self.swing.update_state(dt);
        self.lift.update_state(dt);
        self.elbow.update_state(dt);
        self.wrist.update_state(dt);
        // to do: model/constrain gripper
        // self.gripper.update_state(dt);
    }

    fn calculate_ik(&self, x: f64, y: f64, z: f64) -> Result<(f64, f64, f64), &'static str> {
        let d2 = y - self.d3 - self.d4;
        // take first solution
        let (theta_1, theta_3) = ikcalc_2rmanip(z, x, self.r3, self.r4)?.0;
        Ok((theta_1, d2, theta_3))
    }

    pub fn set_crane_setpoint(&mut self, x: f64, y: f64, z: f64) {
        match self.calculate_ik(x, y, z) {
            Ok((theta_1, d2, theta_3)) => {
                self.swing.set_setpoint(theta_1);
                self.lift.set_setpoint(d2);
                self.elbow.set_setpoint(theta_3);
            },
            Err(msg) => println!("{}", msg)
        }
    }
}

pub struct CraneState{
    // The robotic crane is represented by the current actuator positions
    // for each joint: swing rotation in degrees, lift elevation in mm,
    // elbow rotation in degrees, wrist rotation in degrees, and gripper
    // open/close state in mm.
    pub swing_deg: f64,
    pub lift_mm: f64,
    pub elbow_deg: f64,
    pub wrist_deg: f64,
    pub gripper_mm: f64,
}

fn ikcalc_2rmanip(x: f64, y: f64, l1: f64, l2: f64) -> Result<((f64, f64), (f64, f64)), &'static str> {
    // Inverse kinematics solver for a 2R-manipulator.
    // transform to polar coordinates:
    let r_squared = x * x + y * y;
    let phi = y.atan2(x);

    let cos_theta2 = (r_squared - l1 * l1 - l2 * l2) / (2.0 * l1 * l2);
    if cos_theta2.abs() > 1.0 {
        return Err("No solution: unreachable position.");
    }

    let theta2 = cos_theta2.acos();
    let solution_1 = {
        let k1_1 = l1 + l2 * theta2.cos();
        let k2_1 = l2 * theta2.sin();
        let theta1 = phi - k2_1.atan2(k1_1);
        (theta1.to_degrees(), theta2.to_degrees())
    };

    let theta2 = -theta2;
    let solution_2 = {
        let k1 = l1 + l2 * theta2.cos();
        let k2 = l2 * theta2.sin();
        let theta1 = phi - k2.atan2(k1);
        (theta1.to_degrees(), theta2.to_degrees())
    };

    Ok((solution_1, solution_2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ikcalc_2rmanip_basic() {
        let l1 = 1.0;
        let l2 = 1.0;
        let x = 1.0;
        let y = 1.0;

        assert_eq!(ikcalc_2rmanip(x, y, l1, l2), Ok(((0.0, 90.0), (90.0, -90.0))));
    }
}
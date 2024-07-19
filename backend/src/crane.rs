// Here we will build mock actuators and our mock crane.
// In a real application, we would be reading from a sensor or performing some state estimation.

trait Actuator {
    fn get_position(&self) -> f64;
    fn set_position(&mut self, position: f64);
    fn get_velocity(&self) -> f64;
    fn set_velocity(&mut self, speed: f64);
    fn update_position(&mut self, dt: f64);
}

struct LinearActuator {
    pos: f64,
    lin_v: f64,
    max_pos: f64,
}

impl LinearActuator {
    pub fn new(starting_position: f64, max_position: f64) -> Self {
        Self {
            pos: starting_position,
            lin_v: 0.0,
            max_pos: max_position,
        }
    }
}

impl Actuator for LinearActuator {
    fn get_position(&self) -> f64 {
        self.pos
    }

    fn set_position(&mut self, position: f64) {
        if position >= 0.0 && position <= self.max_pos {
            self.pos = position;
        } else {
            println!("Angle out of range.")
        }
    }

    fn get_velocity(&self) -> f64 {
        self.lin_v
    }

    fn set_velocity(&mut self, speed: f64) {
        self.lin_v = speed;
    }

    fn update_position(&mut self, dt: f64) {
        let new_pos = self.pos + self.lin_v * dt;
        self.pos = new_pos.max(self.max_pos)
    }
}

struct RotaryActuator {
    angle: f64,
    ang_v: f64,
    max_angle: f64,
}

impl RotaryActuator {
    pub fn new(starting_angle: f64, max_angle: f64) -> Self {
        Self {
            angle: starting_angle,
            ang_v: 0.0,
            max_angle,
        }
    }
}

impl Actuator for RotaryActuator {
    fn get_position(&self) -> f64 {
        self.angle
    }

    fn set_position(&mut self, angle: f64) {
        if angle >= 0.0 && angle <= self.max_angle {
            self.angle = angle;
        } else {
            println!("Angle out of range.")
        }
    }

    fn get_velocity(&self) -> f64 {
        self.ang_v
    }

    fn set_velocity(&mut self, speed: f64) {
        self.ang_v = speed;
    }

    fn update_position(&mut self, dt: f64) {
        let new_pos = self.angle + self.ang_v * dt;
        self.angle = new_pos.max(self.angle)
    }
}

#[derive(Default)]
struct Gripper {
    aperture: f64,
    speed: i32,
}

struct WorldOriginSensor {
    x: f64,
    y: f64,
    z: f64
}

pub struct Crane {
    lift: LinearActuator,
    swing: RotaryActuator,
    elbow: RotaryActuator,
    wrist: RotaryActuator,
    gripper: Gripper,
}

impl Crane {
    pub fn new() -> Self {
        Self {
            lift: LinearActuator::new(0.0, 100.0),
            swing: RotaryActuator::new(0.0, 90.0),
            elbow: RotaryActuator::new(0.0, 90.0),
            wrist: RotaryActuator::new(0.0, 90.0),
            gripper: Gripper::default(),
        }
    }

    pub fn get_state(&self) -> CraneState {
        CraneState {
            swing_deg: self.swing.angle,
            lift_mm: self.lift.pos,
            elbow_deg: self.elbow.angle,
            wrist_deg: self.wrist.angle,
            gripper_mm: self.gripper.aperture,
        }
    }

    pub fn set_state(&mut self, cs: CraneState) {
        self.swing.angle = cs.swing_deg;
        self.lift.pos = cs.lift_mm;
        self.elbow.angle = cs.elbow_deg;
        self.wrist.angle = cs.wrist_deg;
        self.gripper.aperture = cs.gripper_mm;
    }
}

pub struct CraneState{
    pub swing_deg: f64,
    pub lift_mm: f64,
    pub elbow_deg: f64,
    pub wrist_deg: f64,
    pub gripper_mm: f64,
}

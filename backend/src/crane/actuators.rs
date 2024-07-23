pub trait MockActuator {
    fn get_position(&self) -> f64;
    fn set_position(&mut self, position: f64);
    fn set_velocity(&mut self, speed: f64);
    fn set_acceleration(&mut self, acc: f64);
    fn set_setpoint(&mut self, setpoint: f64);
    fn update_state(&mut self, dt: f64);
}

pub struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    prev_err: f64,
    integral: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            prev_err: 0.0,
            integral: 0.0,
        }
    }

    pub fn update(&mut self, setpoint: f64, measured: f64, dt: f64) -> f64 {
        let error = setpoint - measured;
        self.integral += error * dt;

        let derivative = (error - self.prev_err) / dt;
        self.prev_err = error;

        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}

pub struct LinearActuator {
    min_pos: f64,
    max_pos: f64,
    max_vel: f64,
    max_acc: f64,
    pos: f64,
    vel: f64,
    acc: f64,
    setpoint: Option<f64>,
    pid: PIDController,
}

impl LinearActuator {
    pub fn new(start_pos: f64, min_pos: f64, max_pos: f64, kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            pos: start_pos,
            vel: 0.0,
            acc: 0.0,
            max_vel: 2.0,
            max_acc: 0.8,
            min_pos,
            max_pos,
            setpoint: None,
            pid: PIDController::new(kp, ki, kd)
        }
    }
}

impl MockActuator for LinearActuator {
    fn get_position(&self) -> f64 {
        self.pos
    }

    fn set_position(&mut self, position: f64) {
        if position <= self.min_pos {
            self.pos = self.min_pos;
            self.vel = 0.0;
            self.acc = 0.0;
        } else if position >= self.max_pos {
            self.pos = self.max_pos;
            self.vel = 0.0;
            self.acc = 0.0;
        } else {
            self.pos = position
        }
    }

    fn set_velocity(&mut self, vel: f64) {
        if vel >= self.max_vel {
            self.vel = self.max_vel
        } else if vel <= -self.max_vel {
            self.vel = -self.max_vel
        } else {
            self.vel = vel
        }
    }

    fn set_acceleration(&mut self, acc: f64) {
        if acc >= self.max_acc {
            self.acc = self.max_acc
        } else if acc <= -self.max_acc {
            self.acc = -self.max_acc
        } else {
            self.acc = acc
        }
    }

    fn update_state(&mut self, dt: f64) {
        let control = self.setpoint.map_or(
            0.0,
             |setpoint| self.pid.update(setpoint, self.pos, dt)
        );

        self.set_acceleration(control - 0.3 * self.acc); // static resistance

        // update dependent variables
        self.set_velocity(self.vel + self.acc * dt);
        self.set_position(self.pos + self.vel * dt);
    }

    fn set_setpoint(&mut self, setpoint: f64) {
        self.setpoint = Some(setpoint);
    }
}

pub struct RotaryActuator {
    angle: f64,
    ang_vel: f64,
    ang_acc: f64,
    max_vel: f64,
    max_acc: f64,
    min_angle: f64,
    max_angle: f64,
    setpoint: Option<f64>,
    pid: PIDController,
}

impl RotaryActuator {
    pub fn new(start_angle: f64, min_angle: f64, max_angle: f64, kp: f64, ki: f64, kd: f64) -> Self {
        Self {
            angle: start_angle,
            ang_vel: 0.0,
            ang_acc: 0.0,
            max_vel: 90.0,
            max_acc: 45.0,
            min_angle,
            max_angle,
            setpoint: None,
            pid: PIDController::new(kp, ki, kd),
        }
    }
}

impl MockActuator for RotaryActuator {
    fn get_position(&self) -> f64 {
        self.angle
    }

    fn set_position(&mut self, angle: f64) {
        if angle <= self.min_angle {
            self.angle = self.min_angle;
            self.ang_vel = 0.0;
        } else if angle >= self.max_angle {
            self.angle = self.max_angle;
            self.ang_vel = 0.0;
        } else {
            self.angle = angle
        }
    }

    fn set_velocity(&mut self, vel: f64) {
        if vel >= self.max_vel {
            self.ang_vel = self.max_vel
        } else if vel <= -self.max_vel {
            self.ang_vel = -self.max_vel
        } else {
            self.ang_vel = vel
        }
    }

    fn set_acceleration(&mut self, acc: f64) {
        if acc >= self.max_acc {
            self.ang_acc = self.max_acc
        } else if acc <= -self.max_acc {
            self.ang_acc = -self.max_acc
        } else {
            self.ang_acc = acc
        }
    }

    fn update_state(&mut self, dt: f64) {
        let control = self.setpoint.map_or(
            0.0,
             |setpoint| self.pid.update(setpoint, self.angle, dt)
        );

        self.set_acceleration(control - 0.3 * self.ang_acc); // static resistance

        // update dependent variables
        self.set_velocity(self.ang_vel + self.ang_acc * dt);
        self.set_position(self.angle + self.ang_vel * dt);
    }

    fn set_setpoint(&mut self, setpoint: f64) {
        self.setpoint = Some(setpoint);
    }
}

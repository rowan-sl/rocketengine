pub mod spec;
mod utils;

use rand::prelude::*;

use spec::MotorSpec;

#[derive(Debug, Clone, PartialEq)]
pub struct RocketMotor {
    base_spec: MotorSpec,
    thrust_list: Vec<f32>,
    ignition_time: Option<f32>,
    lit: bool,
    // not changed, but the starting mass of the motor
    mass: f32,
}

impl RocketMotor {
    /// creates a new motor from the given spec, time step (for interpolation), and maximum ignition delay
    pub fn new(spec: MotorSpec, time_step: f32, mass: f32) -> Self {
        Self {
            base_spec: spec,
            thrust_list: spec.into_interpolated_thrust(time_step),
            ignition_time: None,
            lit: false,
            mass,
        }
    }

    pub fn mass(&self) -> f32 {
        self.mass
    }

    pub fn light(&mut self, time: f32, time_step: f32, max_delay: f32) {
        if !self.lit {
            self.lit = true;
            self.ignition_time = Some(
                time + (thread_rng().gen_range(80..=100) as f32 / 100.0 * max_delay) * time_step,
            );
        }
    }

    pub fn lit(&self) -> bool {
        self.lit
    }

    /// #\[must_not_use]
    pub const unsafe fn explode(self) -> ! {
        std::hint::unreachable_unchecked();
    }
}

//TODO make this use a ordered sequence of motors (fire one after another) instead of a list of names and motors
pub struct RocketEngineSystem {
    motors: Vec<(String, RocketMotor)>,
    time_step: f32,
    max_ignition_delay: f32,
    total_mass: f32,
    last_time: f32,
    // 1 = full
    throttle_precent: f32,
    current_thrust: f32,
}

impl RocketEngineSystem {
    pub fn new(time_step: f32, max_ignition_delay: f32) -> Self {
        Self {
            motors: vec![],
            time_step,
            max_ignition_delay,
            total_mass: 0.0,
            last_time: 0.0,
            throttle_precent: 1.0,
            current_thrust: 0.0,
        }
    }

    /// Add a new motor after the others to be fired.
    pub fn add_motor(&mut self, name: String, motor: RocketMotor) {
        self.total_mass += motor.mass();
        self.motors.push((name, motor));
    }

    pub fn ignite(&mut self, name: String, time: f32) {
        for (n, m) in &mut self.motors {
            if *n == name {
                m.light(time, self.time_step, self.max_ignition_delay);
            }
        }
    }

    pub fn update(&mut self, time: f32) {
        let dt = time - self.last_time;
        for (_, m) in &mut self.motors {
            if m.lit() {
                //TODO make this operate inside the RocketMotor class instead of here
                let counter = ((time * self.time_step) - m.ignition_time.unwrap()) as usize;
                if 0 < counter && counter < m.thrust_list.len() {
                    self.current_thrust = m.thrust_list[counter] * self.throttle_precent;
                    self.total_mass -= 0.004 * dt;
                } else {
                    self.current_thrust = 0.0;
                }
            }
        }

        self.last_time = time;
    }

    pub fn throttle(&mut self, percent: f32) {
        self.throttle_precent = 1.0 - percent;
    }

    pub fn total_mass(&self) -> f32 {
        self.total_mass
    }

    pub fn current_thrust(&self) -> f32 {
        self.current_thrust
    }
}

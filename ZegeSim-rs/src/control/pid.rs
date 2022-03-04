use std::time::Duration;

/**
 * ye olde proportional–integral–derivative controller
 *
 * base of what allows rocket to go brr
 */
#[derive(Debug, Clone, PartialEq)]
pub struct PID {
    // its in the name
    // setpoints
    kp: f32,
    ki: f32,
    kd: f32,
    // working variables
    p: f32,
    i: f32,
    d: f32,
    // yoinked from ZegeSim
    // limit on how big the intigral can get
    i_max: f32,
    // also yoinked from ZegeSim
    // you are even less likely to see this in your run-of-the-mill PID library
    // this is basically a different implementation of the proportional gain, where instead of looking at the error
    // the proportional part of the controller looks at the change between the initial sensed value and the current sensed value
    usepnom: bool,
    // target
    setpoint: f32,
    // last processed value, description from ZegeSim
    // this variable stores the most recent process variable, so that when update is called the PID controller can
    // know not only the current error, but also the speed at which it is approaching or departing from the setpoint!
    // this helps with reducing overshoot by feeding it into the derivitive value, which basically functions as a break on a car
    last_process: f32,
    // current result
    current_result: f32,
    // error ammount
    error: f32,
}

impl PID {
    pub const fn new(kp: f32, ki: f32, kd: f32, setpoint: f32, i_max: f32, usepnom: bool) -> Self {
        Self {
            kp,
            ki,
            kd,
            p: 0.0,
            i: 0.0,
            d: 0.0,
            i_max,
            usepnom,
            setpoint,
            last_process: 0.0,
            current_result: 0.0,
            error: 0.0,
        }
    }

    pub fn set_setpoint(&mut self, setpoint: f32) {
        self.setpoint = setpoint;
    }

    pub fn zero_integral(&mut self) {
        self.i = 0.0;
    }

    pub fn compute(&mut self, process: f32, dt: Duration) {
        let dt = dt.as_secs_f32();
        let change = process - self.last_process;

        self.last_process = process;

        self.error = self.setpoint - process;

        if self.usepnom {
            // whats happening here is the proportional changing with the process variable
            self.p -= change * self.kp;
        } else {
            self.p = self.error * self.kp;
        }

        // checking that the integral will not exceed the maximum value allowed by the user
        if (self.i + self.error * self.ki * dt).abs() < self.i_max {
            self.i += self.error * self.ki * dt;
        }

        self.d = change / dt * self.kd;

        if self.usepnom {
            self.current_result = self.p + self.i - self.d;
        } else {
            self.current_result = self.p + self.i - self.d;
        }
    }

    pub fn output(&self) -> f32 {
        self.current_result
    }
}

/**
ye olde proportional–integral–derivative controller

base of what allows rocket to go brr

and boy does it go brr
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PID {
    // its in the name
    // setpoints
    kp: f64,
    ki: f64,
    kd: f64,
    // working variables
    p: f64,
    i: f64,
    d: f64,
    // limit on intigral size
    i_max: f64,
    // this is basically a different implementation of the proportional gain, where instead of looking at the error
    // the proportional part of the controller looks at the change between the initial sensed value and the current sensed value
    pnom: bool,
    // target
    setpoint: f64,
    // this variable stores the most recent process variable, so that when update is called the PID controller can
    // know not only the current error, but also the speed at which it is approaching or departing from the setpoint!
    // this helps with reducing overshoot by feeding it into the derivitive value, which basically functions as a break on a car
    last_process: f64,
    // current result
    current_result: f64,
    // error ammount
    error: f64,
}

impl PID {
    pub const fn new(kp: f64, ki: f64, kd: f64, setpoint: f64, i_max: f64, pnom: bool) -> Self {
        Self {
            kp,
            ki,
            kd,
            p: 0.0,
            i: 0.0,
            d: 0.0,
            i_max,
            pnom,
            setpoint,
            last_process: 0.0,
            current_result: 0.0,
            error: 0.0,
        }
    }

    pub fn set_setpoint(&mut self, setpoint: f64) {
        self.setpoint = setpoint;
    }

    pub fn zero_integral(&mut self) {
        self.i = 0.0;
    }

    pub fn compute(&mut self, process: f64, dt: f64) {
        let change = process - self.last_process;

        self.last_process = process;

        self.error = self.setpoint - process;

        if self.pnom {
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

        self.current_result = self.p + self.i - self.d;
    }

    pub fn output(&self) -> f64 {
        self.current_result
    }
}

/// Full State Feedback
///
/// (pid but better and more computationaly intensive)
pub struct FSF {
    gain_a: f32,
    gain_b: f32,
    setpoint: f32,
    last_ori: f32,
    output: f32,
}

impl FSF {
    pub fn new(gain_a: f32, gain_b: f32) -> Self {
        Self {
            gain_a,
            gain_b,
            setpoint: 0.0,
            last_ori: 0.0,
            output: 0.0,
        }
    }

    pub fn compute(&mut self, ori: f32, ori_rate: f32) {
        self.output = -self.gain_a * (ori - self.setpoint);
        self.output += -self.gain_b * ori_rate;
        self.last_ori = ori;
    }

    pub fn get_output(&self) -> f32 {
        self.output
    }
}

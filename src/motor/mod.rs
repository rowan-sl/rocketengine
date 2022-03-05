pub mod raw;

use crate::{secs, math::F64x3};

fn interpolate_two(current: (f64, F64x3), next: (f64, F64x3), interpolated_step: f64) -> Vec<(f64, F64x3)> {
    //TODO check if this creates two many intermediate steps
    let dt = next.0 - current.0;
    let num_steps = (dt/interpolated_step).ceil();
    let change = next.1 - current.1;
    let change_per_step = change / num_steps;

    let mut res = vec![];

    for i in 0..=num_steps as i64 {
        res.push((interpolated_step, current.1 + change_per_step * i as f64))
    }

    res
}

pub fn interpolate_thrust(original: Vec<(secs, F64x3)>, interpolated_step: secs) -> Vec<(secs, F64x3)> {
    let mut original_i = original.into_iter();
    let mut last = match original_i.next() {
        Some(v) => {v},
        None => {return vec![]},
    };

    let mut res = vec![];

    for next in original_i {
        res.append(&mut interpolate_two(last, next, interpolated_step));
        last = next;
    }

    res
}

#[derive(Clone, Debug)]
pub struct Motor {
    /// time left for each step of forces, and the force for that period of time (interpolated)
    forces: Vec<(secs, F64x3)>,
    original_forces_len: usize,
    lit: bool,
    dry_weight: f64,
    fuel_weight: f64,
}

impl Motor {
    pub fn new(raw_forces: Vec<(secs, F64x3)>, interp_step: secs, dry_weight: f64, fuel_weight: f64) -> Self {
        Self::from_interpolated(interpolate_thrust(raw_forces, interp_step), dry_weight, fuel_weight)
    }

    pub fn from_interpolated(interpolated_forces: Vec<(secs, F64x3)>, dry_weight: f64, fuel_weight: f64) -> Self {
        let interp_len = interpolated_forces.len();
        Self {
            forces: interpolated_forces,
            original_forces_len: interp_len,
            dry_weight,
            fuel_weight,
            lit: false,
        }
    }

    pub fn thrust_for(&mut self, dt: secs) -> Option<F64x3> {
        if self.forces.is_empty() {
            None
        } else if !self.lit {
            Some(F64x3::zero())
        } else {
            let mut latest_force = vec![self.forces[0].1];
            let mut change = dt;
            loop {
                if self.forces.is_empty() {
                    break;
                }
                self.forces[0].0 -= change;
                if self.forces[0].0 <= 0.0 {
                    change = self.forces[0].0;
                    latest_force.push(self.forces.remove(0).1);
                } else {
                    break;
                }
            }
            let mut sum = F64x3::zero();
            let len = latest_force.len();
            for i in latest_force {
                sum += i;
            }
            Some(sum/len as f64)
        }
    }

    pub fn burnt_out(&self) -> bool {
        self.forces.is_empty()
    }

    pub fn lit(&self) -> bool {
        self.lit
    }

    pub fn light(&mut self) {
        self.lit = true;
    }

    pub fn precentage_done(&self) -> f64 {
        self.forces.len() as f64 / self.original_forces_len as f64
    }

    pub fn weight(&self) -> f64 {
        self.precentage_done() * self.fuel_weight + self.dry_weight
    }

    pub const unsafe fn explode(self) -> ! {
        std::hint::unreachable_unchecked();
    }
}

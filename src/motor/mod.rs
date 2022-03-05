pub mod raw;

use crate::{secs, math::F64x3, GRAMS_TO_KG};

fn interpolate_two(current: (f64, F64x3, f64), next: (f64, F64x3, f64), interpolated_step: f64) -> Vec<(f64, F64x3, f64)> {
    //TODO check if this creates two many intermediate steps
    let dt = next.0 - current.0;
    let num_steps = (dt/interpolated_step).ceil();
    let change_thrust = next.1 - current.1;
    let change_thrust_per_step = change_thrust / num_steps;
    let change_wt = next.2 - current.2;
    let change_wt_per_step = change_wt / num_steps;

    let mut res = vec![];

    for i in 0..=num_steps as i64 {
        res.push((interpolated_step, current.1 + change_thrust_per_step * i as f64, current.2 + change_wt_per_step * i as f64))
    }

    res
}

pub fn interpolate_thrust(original: Vec<(secs, F64x3, f64)>, interpolated_step: secs) -> Vec<(secs, F64x3, f64)> {
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
    /// 
    /// UNIT IS GRAMS AEEEEEEEE
    forces: Vec<(secs, F64x3, f64)>,
    lit: bool,
    dry_weight: f64,
}

impl Motor {
    pub fn new(raw: Vec<[f64; 3]>, dry_weight: f64) -> Self {
        let forces = raw.into_iter().map(|i| {(i[0], F64x3::new(0.0, 0.0, i[1]), i[2])}).collect();
        Self {
            forces,
            dry_weight,
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

    pub fn weight(&self) -> f64 {
        (self.forces.get(0).unwrap_or(&(0.0, F64x3::zero(), 0.0)).2 + self.dry_weight) * GRAMS_TO_KG
    }

    pub const unsafe fn explode(self) -> ! {
        std::hint::unreachable_unchecked();
    }
}
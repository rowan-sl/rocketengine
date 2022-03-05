use crate::math::{F64x3, Quaternion};

// ! do NOT use this, make a point thing like in ZegeSim as this is too complex and will not work well

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    /// offset from the parent shape
    pub offset: F64x3,
    pub length: f64,
    pub radius: f64,
    pub mass: f64,
}

impl Cylinder {
    /// volume of the cylinder (PIr^2*h)
    pub fn volume(&self) -> f64 {
        (std::f64::consts::PI * self.radius).powi(2) * self.length
    }

    pub fn density(&self) -> f64 {
        self.mass / self.volume()
    }

    /// calculate the moment of inertia for this cylinder around its symmetry axis
    pub fn moi_v(&self) -> f64 {
        0.5 * self.mass * self.radius.powi(2)
    }

    pub fn moi_r(&self) -> f64 {
        let base = self.mass * self.radius.powi(2);
        0.25 * base + (1.0 / 12.0) * base
    }
}

pub struct CylinderBody {
    /// position of the main body. all components are stored as a offset of this, to reduce errors
    pub position: F64x3,
    pub components: Vec<Cylinder>,
    /// m/s
    pub velocity: F64x3,
    /// m/s^2
    pub acceleration: F64x3,

    pub rotation: Quaternion,
    pub rotational_velocity: F64x3,
    pub rotational_acceleration: F64x3,
}

//https://en.wikipedia.org/wiki/Parallel_axis_theorem
fn do_a_parallel_axis_theorem(original_moi: f64, mass: f64, distance: f64) -> f64 {
    original_moi + mass * distance.powi(2)
}

impl CylinderBody {
    /// center of mass of the object in global space
    pub fn com(&self) -> F64x3 {
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut z_sum = 0.0;
        let mut x_cord_sum = 0.0;
        let mut y_cord_sum = 0.0;
        let mut z_cord_sum = 0.0;

        for cyl in &self.components {
            x_sum += cyl.mass * (cyl.offset.x + self.position.x);
            y_sum += cyl.mass * (cyl.offset.y + self.position.y);
            z_sum += cyl.mass * (cyl.offset.y + self.position.z);
            x_cord_sum += cyl.offset.x + self.position.x;
            y_cord_sum += cyl.offset.y + self.position.y;
            z_cord_sum += cyl.offset.y + self.position.z;
        }

        F64x3::new(
            x_sum / x_cord_sum,
            y_sum / y_cord_sum,
            z_sum / z_cord_sum,
        )
    }

    /// calculate the moment of inertia around the vertical axis (vertical if cylinder is vertical)
    /// 
    /// inertia moment lmao
    pub fn moi_v(&self) -> f64 {
        let mut sum = 0.0;
        for part in &self.components {
            // calc individual moment of inertias, then  use the parallel axis theorem to apply that as if it was the center of this shape, and then add them together
            sum += do_a_parallel_axis_theorem(part.moi_v(), part.mass, (part.offset.x.powi(2) + part.offset.y.powi(2)).sqrt());
        }
        sum
    }
}

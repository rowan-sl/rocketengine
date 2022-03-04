use std::{fmt::{Display, Formatter}, ops::{Mul, Add, Sub, MulAssign}};

use num::traits::real::Real;

use super::vec3::Vec3;

pub type QuaternionF32 = Quaternion<f32>;
pub type QuaternionF64 = Quaternion<f64>;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Default> Default for Quaternion<T> {
    fn default() -> Self {
        Self {
            w: T::default(),
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T: Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}, {}, {}, {}", self.w, self.x, self.y, self.z)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy> Mul for Quaternion<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let w = (self.w * other.w) - (self.x * other.x) - (self.y * other.y) - (self.z * other.z);
        let x = (self.w * other.x) + (self.x * other.w) + (self.y * other.z) - (self.z * other.y);
        let y = (self.w * other.y) - (self.x * other.z) + (self.y * other.w) + (self.z * other.x);
        let z = (self.w * other.z) + (self.x * other.y) - (self.y * other.x) + (self.z * other.w);
    
        Self {w, x, y, z}
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Copy> MulAssign for Quaternion<T> {
    fn mul_assign(&mut self, other: Self) {
        //hehe
        *self = *self * other; 
    }
}

impl<T: Default> From<Vec3<T>> for Quaternion<T> {
    fn from(v: Vec3<T>) -> Self {
        Self::new(T::default(), v.x, v.y, v.z)
    }
}

impl<T> Quaternion<T> {
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { w, x, y, z }
    }
}

impl<T: Real + Default + MulAssign> Quaternion<T> {
    /// Return the conjugate of the quaternion.
    pub fn conj(self) -> Self {
        Self { w: self.w, x: -self.x, y: -self.y, z: -self.z }
    }

    /// Return the norm of the quaternion.
    pub fn norm(&self) -> T {
        (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    //TODO look into this and improve?
    /// Normalize the quaternion.
    pub fn normalize(self) -> Self {
        let n = self.norm();
        Self::new(self.w / n, self.x / n, self.y / n, self.z / n)
    }

    /// Rotate a vector by the quaternion.
    pub fn rotate(&self, vector: Vec3<T>) -> Vec3<T> {
        let mut q: Quaternion<T> = vector.into();

        q = *self * q * self.conj();

        Vec3 {
            x: q.x,
            y: q.y,
            z: q.z,
        }
    }


    /// Return the fractional of the quaternion.
    pub fn fractional(mut self, a: T) -> Self {
        self.w = 1.0 - a + a * self.w;
        self.x *= a;
        self.y *= a;
        self.z *= a;

        self.normalize()
    }

    /// Return the rotation quaternion between two vectors.
    pub fn rotation_between_vectors(self, vector: Vec3<T>) -> Self {
        let mut q = Quaternion::new(0.0, vector.x, vector.y, vector.z);

        q = self * q;
        q.w = 1.0 - q.w;

        q.normalize()
    }

    /// Return the quaternion from an axis and angle.
    pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
        let sa = (angle / 2.0).cos();

        let w = (angle / 2.0).cos();
        let x = axis.x * sa;
        let y = axis.y * sa;
        let z = axis.z * sa;

        Quaternion::new(w, x, y, z)
    }

    /// Convert euler angles to a quaternion.
    pub fn from_euler(euler_angles: Vec3<T>) -> Self {
        let cr = (euler_angles.x / 2.0).cos();
        let cp = (euler_angles.y / 2.0).cos();
        let cy = (euler_angles.z / 2.0).cos();

        let sr = (euler_angles.x / 2.0).sin();
        let sp = (euler_angles.y / 2.0).sin();
        let sy = (euler_angles.z / 2.0).sin();

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        Quaternion::new(w, x, y, z)
    }

    /// Convert a quaternion to euler angles.
    pub fn to_euler(self) -> Vec3<T> {
        let x = 2.0
            * (self.w * self.x + self.y * self.z)
                .atan2(1.0 - 2.0 * (self.x.powi(2) + self.y.powi(2)));
        let y = (2.0 * (self.w * self.y - self.z * self.x)).asin();
        let z = 2.0
            * (self.w * self.z + self.x * self.y)
                .atan2(1.0 - 2.0 * (self.y.powi(2) + self.z.powi(2)));

        Vec3 { x, y, z }
    }
}

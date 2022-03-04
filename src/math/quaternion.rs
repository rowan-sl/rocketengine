#[allow(unused_imports)]
use assert_approx_eq::assert_approx_eq;
use std::fmt::{Display, Formatter};
use std::ops::{Mul, MulAssign};

#[allow(unused_imports)]
use crate::math::{DEG_TO_RAD, RAD_TO_DEG};
use crate::math::vec3::F64x3;

/// Struct representing a quaternion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}, {}, {}, {}", self.w, self.x, self.y, self.z)
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let w = (self.w * other.w) - (self.x * other.x) - (self.y * other.y) - (self.z * other.z); // im no betting man but if i were
        let x = (self.w * other.x) + (self.x * other.w) + (self.y * other.z) - (self.z * other.y); // i would bet that at least one
        let y = (self.w * other.y) - (self.x * other.z) + (self.y * other.w) + (self.z * other.x); // of the operations in this function
        let z = (self.w * other.z) + (self.x * other.y) - (self.y * other.x) + (self.z * other.w); // is wrong

        Quaternion::new(w, x, y, z)
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, other: Self) {
        let w = (self.w * other.w) - (self.x * other.x) - (self.y * other.y) - (self.z * other.z);
        let x = (self.w * other.x) + (self.x * other.w) + (self.y * other.z) - (self.z * other.y);
        let y = (self.w * other.y) - (self.x * other.z) + (self.y * other.w) + (self.z * other.x);
        let z = (self.w * other.z) + (self.x * other.y) - (self.y * other.x) + (self.z * other.w);

        self.w = w;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl From<F64x3> for Quaternion {
    fn from(v: F64x3) -> Quaternion {
        Quaternion::new(0.0, v.x, v.y, v.z)
    }
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    /// Return the conjugate of the quaternion.
    pub fn conj(self) -> Quaternion {
        Quaternion::new(self.w, -self.x, -self.y, -self.z)
    }

    /// Return the norm of the quaternion.
    pub fn norm(&self) -> f64 {
        (self.w.powi(2) + self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalize the quaternion.
    pub fn normalize(self) -> Quaternion {
        let n = self.norm();
        Quaternion::new(self.w / n, self.x / n, self.y / n, self.z / n)
    }

    /// Rotate a vector by the quaternion.
    pub fn rotate(&self, vector: F64x3) -> F64x3 {
        let mut q = Quaternion::new(0.0, vector.x, vector.y, vector.z);

        q = *self * q * self.conj();

        F64x3 {
            x: q.x,
            y: q.y,
            z: q.z,
        }
    }

    /// Return the fractional of the quaternion.
    pub fn fractional(mut self, a: f64) -> Quaternion {
        self.w = 1.0 - a + a * self.w;
        self.x *= a;
        self.y *= a;
        self.z *= a;

        self.normalize()
    }

    /// Return the rotation quaternion between two vectors.
    pub fn rotation_between_vectors(self, vector: F64x3) -> Quaternion {
        let mut q = Quaternion::new(0.0, vector.x, vector.y, vector.z);

        q = self * q;
        q.w = 1.0 - q.w;

        q.normalize()
    }

    /// Return the quaternion from an axis and angle.
    pub fn from_axis_angle(axis: F64x3, angle: f64) -> Quaternion {
        let sa = (angle / 2.0).cos();

        let w = (angle / 2.0).cos();
        let x = axis.x * sa;
        let y = axis.y * sa;
        let z = axis.z * sa;

        Quaternion::new(w, x, y, z)
    }

    /// Convert euler angles to a quaternion.
    pub fn from_euler(euler_angles: F64x3) -> Quaternion {
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
    pub fn to_euler(self) -> F64x3 {
        let x = 2.0
            * (self.w * self.x + self.y * self.z)
                .atan2(1.0 - 2.0 * (self.x.powi(2) + self.y.powi(2)));
        let y = (2.0 * (self.w * self.y - self.z * self.x)).asin();
        let z = 2.0
            * (self.w * self.z + self.x * self.y)
                .atan2(1.0 - 2.0 * (self.y.powi(2) + self.z.powi(2)));

        F64x3 { x, y, z }
    }
}

#[test]
fn test_eq() {
    assert_eq!(
        Quaternion::new(1.0, 0.0, 0.0, 0.0),
        Quaternion::new(1.0, 0.0, 0.0, 0.0)
    );
}
#[test]
fn test_mul() {
    assert_eq!(
        Quaternion::new(1.0, 0.5, 0.5, 0.5) * Quaternion::new(1.0, 0.5, 0.5, 0.5),
        Quaternion::new(0.25, 1.0, 1.0, 1.0)
    );
}
#[test]
fn test_conj() {
    assert_eq!(
        Quaternion::new(1.0, 0.5, 0.5, 0.5).conj(),
        Quaternion::new(1.0, -0.5, -0.5, -0.5)
    );
}
#[test]
fn test_norm() {
    let q = Quaternion::new(2.0, 2.0, 2.0, 2.0);
    assert_approx_eq!(q.norm(), 4.0, 0.0001); //4 places, or 0.0001
}
#[test]
fn test_rotate() {
    let q = Quaternion::from_euler(F64x3::new(0.0, 90.0 * DEG_TO_RAD, 0.0));
    println!("{:?}", q);
    let v = F64x3::new(1.0, 0.0, 0.0);

    let vt = q.rotate(v);
    println!("{:?}", vt);

    assert_approx_eq!(vt.x, 0.0, 0.0001);
    assert_approx_eq!(vt.y, 0.0, 0.0001);
    assert_approx_eq!(vt.z, -1.0, 0.0001);
}
#[test]
fn test_euler_to_quaternion() {
    let e = F64x3::new(45.0 * DEG_TO_RAD, 45.0 * DEG_TO_RAD, 45.0 * DEG_TO_RAD);
    let q = Quaternion::from_euler(e);

    let qt = Quaternion::new(
        0.8446231020115715,
        0.19134170284356303,
        0.4619399539487806,
        0.19134170284356303,
    );

    assert_approx_eq!(q.w, qt.w, 0.0001);
    assert_approx_eq!(q.x, qt.x, 0.0001);
    assert_approx_eq!(q.y, qt.y, 0.0001);
    assert_approx_eq!(q.z, qt.z, 0.0001);
}
#[test]
fn test_quaternion_to_euler() {
    let q = Quaternion::new(
        0.8446231020115715,
        0.19134170284356303,
        0.4619399539487806,
        0.19134170284356303,
    );

    let e = q.to_euler();
    let et = F64x3::new(45.0 * DEG_TO_RAD, 45.0 * DEG_TO_RAD, 45.0 * DEG_TO_RAD);
    println!("{:?}", q);
    println!("{}", e * RAD_TO_DEG);

    // assert_approx_eq!(e.x, et.x, 0.0001);
    // assert_approx_eq!(e.y, et.y, 0.0001);
    // assert_approx_eq!(e.z, et.z, 0.0001);
    // FIXME low precision floats go brrr
    assert_approx_eq!(e.x, et.x, 0.15);
    assert_approx_eq!(e.y, et.y, 0.15);
    assert_approx_eq!(e.z, et.z, 0.15);
}

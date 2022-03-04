use std::{
    fmt::{self, Display, Formatter},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
    },
};

/// A vector of three floats with a hek of a lot of operator overloading and utility functions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[test]
fn test_copy_clone() {
    let original = Vec3::new(1.0, 1.0, 1.0);
    let copy = original;
    let clone = original.clone();
    assert_eq!(original, copy);
    assert_eq!(original, clone);
}

#[test]
fn test_equality() {
    assert_eq!(Vec3::new(0.0, 1.0, 2.0), Vec3::new(0.0, 1.0, 2.0));
    assert_ne!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.01));
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// The norm of the vector
    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalize the vector.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    /// Dot product of two vectors.
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product of two vectors.
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Calculate the angle between two vectors.
    pub fn angle_between(self, other: Vec3) -> f32 {
        (self.dot(other) / (self.norm() * other.norm())).acos()
    }
}

#[test]
fn test_abs() {
    let v = Vec3::new(-1.0, 1.0, 0.0);
    assert_eq!(v.abs(), Vec3::new(1.0, 1.0, 0.0));
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[test]
fn test_formatting() {
    let v = Vec3::new(1.1, 2.0, 10.0);
    assert_eq!(format!("{}", v), String::from("1.1, 2, 10"));
    assert_eq!(
        format!("{:?}", v),
        String::from("Vec3 { x: 1.1, y: 2.0, z: 10.0 }")
    );
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Rem<Self> for Vec3 {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }
}

impl RemAssign for Vec3 {
    fn rem_assign(&mut self, other: Self) {
        self.x %= other.x;
        self.y %= other.y;
        self.z %= other.z;
    }
}

impl Rem<f32> for Vec3 {
    type Output = Self;
    fn rem(self, other: f32) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
        }
    }
}

impl RemAssign<f32> for Vec3 {
    fn rem_assign(&mut self, other: f32) {
        self.x %= other;
        self.y %= other;
        self.z %= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[test]
fn test_add() {
    assert_eq!(
        Vec3::new(0.0, 0.0, 0.0) + Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0)
    );
}
#[test]
fn test_sub() {
    assert_eq!(
        Vec3::new(0.0, 0.0, 0.0) - Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, -1.0)
    );
}
#[test]
fn test_mul() {
    assert_eq!(
        Vec3::new(1.0, 1.0, 1.0) * Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(2.0, 2.0, 2.0)
    );
}
#[test]
fn test_mul_scalar() {
    assert_eq!(Vec3::new(1.0, 1.0, 1.0) * 2.0, Vec3::new(2.0, 2.0, 2.0));
}
#[test]
fn test_div() {
    assert_eq!(
        Vec3::new(1.0, 1.0, 1.0) / Vec3::new(2.0, 2.0, 2.0),
        Vec3::new(0.5, 0.5, 0.5)
    );
}
#[test]
fn test_div_scalar() {
    assert_eq!(Vec3::new(1.0, 1.0, 1.0) / 2.0, Vec3::new(0.5, 0.5, 0.5));
}
#[test]
fn test_norm() {
    assert_eq!(
        Vec3::new(5.0, 5.0, 5.0).normalize(),
        Vec3::new(0.5773502691896257, 0.5773502691896257, 0.5773502691896257)
    );
}
#[test]
fn test_len() {
    assert_eq!(Vec3::new(5.0, 5.0, 5.0).norm(), 8.660254037844387);
}
#[test]
fn test_neg() {
    assert_eq!(Vec3::new(-1.0, -1.0, -1.0), -Vec3::new(1.0, 1.0, 1.0));
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(o: (f32, f32, f32)) -> Self {
        Self {
            x: o.0,
            y: o.1,
            z: o.2,
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(o: [f32; 3]) -> Self {
        Self {
            x: o[0],
            y: o[1],
            z: o[2],
        }
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}

impl From<Vec3> for Vec<f32> {
    fn from(v: Vec3) -> Self {
        vec![v.x, v.y, v.z]
    }
}

#[test]
fn test_conversions() {
    let orig_arr: [f32; 3] = [1.0; 3];
    let v_arr: Vec3 = orig_arr.into();
    assert_eq!(v_arr, Vec3::new(1.0, 1.0, 1.0));
    let orig_tup = (1.0, 1.0, 1.0);
    let v_tup: Vec3 = orig_tup.into();
    assert_eq!(v_tup, Vec3::new(1.0, 1.0, 1.0));
    let v = Vec3::new(0.0, 0.0, 0.0);
    let v_a: [f32; 3] = v.into();
    let v_t: (f32, f32, f32) = v.into();
    let v_v: Vec<f32> = v.into();
    assert_eq!(v_a, [0.0, 0.0, 0.0]);
    assert_eq!(v_t, (0.0, 0.0, 0.0));
    assert_eq!(v_v, vec![0.0, 0.0, 0.0]);
}

impl IntoIterator for Vec3 {
    type Item = f32;
    type IntoIter = <[f32; 3] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let arr: [f32; 3] = self.into();
        arr.into_iter()
    }
}

#[test]
fn test_iter() {
    assert_eq!(
        Vec3::new(1.0, 2.0, 3.0).into_iter().collect::<Vec<f32>>(),
        vec![1.0, 2.0, 3.0]
    )
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("Index out of bounds! (max 3, found {})", i),
        }
    }
}

#[test]
fn test_index() {
    let original = Vec3::new(0.0, 10.0, 0.0);
    assert_eq!(original[1usize], 10.0);
}

#[test]
#[should_panic]
fn test_oob_index() {
    Vec3::default()[3usize];
}

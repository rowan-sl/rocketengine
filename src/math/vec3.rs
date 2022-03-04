use std::{
    fmt::{self, Display, Formatter},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
    },
};
use num::{Num, traits::real::Real, Signed};

pub type F32x3 = Vec3<f32>;
pub type F64x3 = Vec3<f64>;

/// A vector of three floats with a hek of a lot of operator overloading and utility functions
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T>
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + Clone + Copy + Real + Signed + Mul + Sub> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
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
    pub fn norm(&self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Normalize the vector.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    /// Dot product of two vectors.
    pub fn dot(self, other: Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product of two vectors.
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Calculate the angle between two vectors.
    pub fn angle_between(self, other: Vec3<T>) -> T {
        (self.dot(other) / (self.norm() * other.norm())).acos()
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Mul<Output = T>> Mul<Self> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T: MulAssign> MulAssign for Vec3<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T: Div<Output = T>> Div<Self> for Vec3<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<T: DivAssign> DivAssign for Vec3<T> {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T: Rem<Output = T>> Rem<Self> for Vec3<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        Self {
            x: self.x % other.x,
            y: self.y % other.y,
            z: self.z % other.z,
        }
    }
}

impl<T: RemAssign> RemAssign for Vec3<T> {
    fn rem_assign(&mut self, other: Self) {
        self.x %= other.x;
        self.y %= other.y;
        self.z %= other.z;
    }
}

impl<T: Rem<Output = T> + Copy> Rem<T> for Vec3<T> {
    type Output = Self;
    fn rem(self, other: T) -> Self {
        Self {
            x: self.x % other,
            y: self.y % other,
            z: self.z % other,
        }
    }
}

impl<T: RemAssign + Copy> RemAssign<T> for Vec3<T> {
    fn rem_assign(&mut self, other: T) {
        self.x %= other;
        self.y %= other;
        self.z %= other;
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(o: (T, T, T)) -> Self {
        Self {
            x: o.0,
            y: o.1,
            z: o.2,
        }
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(o: [T; 3]) -> Self {
        Self {
            x: o[0],
            y: o[1],
            z: o[2],
        }
    }
}

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> Self {
        [v.x, v.y, v.z]
    }
}

impl<T> From<Vec3<T>> for (T, T, T) {
    fn from(v: Vec3<T>) -> Self {
        (v.x, v.y, v.z)
    }
}

impl<T> From<Vec3<T>> for Vec<T> {
    fn from(v: Vec3<T>) -> Self {
        vec![v.x, v.y, v.z]
    }
}

impl<T> IntoIterator for Vec3<T> {
    type Item = T;
    type IntoIter = <[T; 3] as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let arr: [T; 3] = self.into();
        arr.into_iter()
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            i => panic!("Index out of bounds! (max 3, found {})", i),
        }
    }
}

#[cfg(test)]
macro_rules! test_vec3_impl {
    ($Vec3:ty, $Vec3Elem:ty) => {
    #[test]
    fn test_copy_clone() {
        let original = <$Vec3>::new(1.0, 1.0, 1.0);
        let copy = original;
        let clone = original.clone();
        assert_eq!(original, copy);
        assert_eq!(original, clone);
    }

    #[test]
    fn test_equality() {
        assert_eq!(<$Vec3>::new(0.0, 1.0, 2.0), <$Vec3>::new(0.0, 1.0, 2.0));
        assert_ne!(<$Vec3>::new(1.0, 2.0, 3.0), <$Vec3>::new(1.0, 2.0, 3.01));
    }

    #[test]
    fn test_formatting() {
        let v = <$Vec3>::new(1.1, 2.0, 10.0);
        assert_eq!(format!("{}", v), String::from("1.1, 2, 10"));
        assert_eq!(
            format!("{:?}", v),
            String::from("Vec3 { x: 1.1, y: 2.0, z: 10.0 }")
        );
    }

    #[test]
    fn test_abs() {
        let v = <$Vec3>::new(-1.0, 1.0, 0.0);
        assert_eq!(v.abs(), <$Vec3>::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            <$Vec3>::new(0.0, 0.0, 0.0) + <$Vec3>::new(1.0, 1.0, 1.0),
            <$Vec3>::new(1.0, 1.0, 1.0)
        );
    }
    #[test]
    fn test_sub() {
        assert_eq!(
            <$Vec3>::new(0.0, 0.0, 0.0) - <$Vec3>::new(1.0, 1.0, 1.0),
            <$Vec3>::new(-1.0, -1.0, -1.0)
        );
    }
    #[test]
    fn test_mul() {
        assert_eq!(
            <$Vec3>::new(1.0, 1.0, 1.0) * <$Vec3>::new(2.0, 2.0, 2.0),
            <$Vec3>::new(2.0, 2.0, 2.0)
        );
    }
    #[test]
    fn test_mul_scalar() {
        assert_eq!(<$Vec3>::new(1.0, 1.0, 1.0) * 2.0, <$Vec3>::new(2.0, 2.0, 2.0));
    }
    #[test]
    fn test_div() {
        assert_eq!(
            <$Vec3>::new(1.0, 1.0, 1.0) / <$Vec3>::new(2.0, 2.0, 2.0),
            <$Vec3>::new(0.5, 0.5, 0.5)
        );
    }
    #[test]
    fn test_div_scalar() {
        assert_eq!(<$Vec3>::new(1.0, 1.0, 1.0) / 2.0, <$Vec3>::new(0.5, 0.5, 0.5));
    }
    #[test]
    fn test_norm() {
        assert_eq!(
            <$Vec3>::new(5.0, 5.0, 5.0).normalize(),
            <$Vec3>::new(0.5773502691896257, 0.5773502691896257, 0.5773502691896257)
        );
    }
    #[test]
    fn test_len() {
        assert_eq!(<$Vec3>::new(5.0, 5.0, 5.0).norm(), 8.660254037844387);
    }
    #[test]
    fn test_neg() {
        assert_eq!(<$Vec3>::new(-1.0, -1.0, -1.0), -<$Vec3>::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_iter() {
        assert_eq!(
            <$Vec3>::new(1.0, 2.0, 3.0).into_iter().collect::<Vec<$Vec3Elem>>(),
            vec![1.0, 2.0, 3.0]
        )
    }

    #[test]
    fn test_conversions() {
        let orig_arr: [$Vec3Elem; 3] = [1.0; 3];
        let v_arr: $Vec3 = orig_arr.into();
        assert_eq!(v_arr, <$Vec3>::new(1.0, 1.0, 1.0));
        let orig_tup = (1.0, 1.0, 1.0);
        let v_tup: $Vec3 = orig_tup.into();
        assert_eq!(v_tup, <$Vec3>::new(1.0, 1.0, 1.0));
        let v = <$Vec3>::new(0.0, 0.0, 0.0);
        let v_a: [$Vec3Elem; 3] = v.into();
        let v_t: ($Vec3Elem, $Vec3Elem, $Vec3Elem) = v.into();
        let v_v: Vec<$Vec3Elem> = v.into();
        assert_eq!(v_a, [0.0, 0.0, 0.0]);
        assert_eq!(v_t, (0.0, 0.0, 0.0));
        assert_eq!(v_v, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_index() {
        let original = <$Vec3>::new(0.0, 10.0, 0.0);
        assert_eq!(original[1usize], 10.0);
    }

    #[test]
    #[should_panic]
    fn test_oob_index() {
        <$Vec3>::default()[3usize];
    }
    };
}

#[cfg(test)]
mod vec3f32_tests {
    test_vec3_impl!(super::F32x3, f32);
}

#[cfg(test)]
mod vec3f64_tests {
    test_vec3_impl!(super::F64x3, f64);
}

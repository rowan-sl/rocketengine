use crate::math::F64x3;

/// gains this ammount of velocity every second
/// 
/// unit: m/s^2
pub const GRAVITY: F64x3 = F64x3 { x: 0.0, y: 0.0, z: -9.80665 };

pub const GRAMS_TO_KG: f64 = 0.001;
pub const KG_TO_GRAMS: f64 = 1000.0;

#[allow(non_camel_case_types)]
pub type secs = f64;
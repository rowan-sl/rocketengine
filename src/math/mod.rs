mod vec3;
mod quaternion;

pub use vec3::*;
pub use quaternion::*;

pub const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
pub const RAD_TO_DEG: f64 = 180.0 / std::f64::consts::PI;
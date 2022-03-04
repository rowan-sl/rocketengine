/*!
 * remember kids, dont do math
*/

pub mod quaternion;
pub mod utils;
pub mod vec3;

pub use quaternion::Quaternion;
pub use utils::{
    calculate_angle_fom_desired_torque, clamp, lpf, positive_or_negative, rotate, DEG_TO_RAD,
    RAD_TO_DEG,
};
pub use vec3::Vec3;

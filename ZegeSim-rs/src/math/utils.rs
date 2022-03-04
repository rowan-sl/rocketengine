use std::f32::consts::PI;

use micromath::vector::F32x2;
use rand::{thread_rng, Rng};

pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;

// rotate a point around (0, 0)
pub fn rotate(pt: F32x2, theta: f32) -> F32x2 {
    let cs = theta.cos();
    let sn = theta.sin();

    let rotated_x = pt.x * cs - pt.y * sn;
    let rotated_y = pt.x * sn + pt.y * cs;

    F32x2 {
        x: rotated_x,
        y: rotated_y,
    }
}

pub fn calculate_angle_fom_desired_torque(
    moment_arm: f32,
    force: f32,
    mmoi: f32,
    desired_torque: f32,
) -> f32 {
    if force != 0.0 {
        let calcval = desired_torque * mmoi / force / moment_arm;
        if calcval.abs() > 1.0 {
            return 0.0;
        } else {
            return calcval.asin();
        }
    } else {
        return 0.0;
    }
}

pub fn positive_or_negative() -> i8 {
    if thread_rng().gen_bool(0.5) {
        1
    } else {
        -1
    }
}

pub fn lpf(new: f32, old: f32, gain: f32) -> f32 {
    new * (1.0 - gain) + old * gain
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn clamp<T: PartialOrd>(n: T, minn: T, maxn: T) -> T {
    // Clamps output to a range
    max(min(maxn, n), minn)
}

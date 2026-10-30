// src/main.rs
use core::math::mat3::Mat3;
use core::math::vec3::Vec3;
use std::f64::consts::FRAC_PI_2; // 90 degrees in radians

fn main() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(4.0, 5.0, 6.0);

    println!("Dot product: {}", a.dot(b)); // 1*4 + 2*5 + 3*6 = 32

    let rotation = Mat3::from_rotation_z(FRAC_PI_2);
    let point = Vec3::new(1.0, 0.0, 0.0); // pointing right

    let rotated = rotation * point;
    println!("Rotated point: {:?}", rotated);
}

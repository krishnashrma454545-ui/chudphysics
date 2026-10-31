use crate::math::Vec2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Circle {
    /// Radius of the circle
    pub radius: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    /// Width (breadth) and height
    pub size: Vec2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Triangle {
    /// Three side lengths of the triangle
    pub size: f64,
}

#[allow(non_camel_case_types)]
pub type triangle = Triangle;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Aabb { half_extents: (f64, f64) },
    Triangle { size: f64 },
}

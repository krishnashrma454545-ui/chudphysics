use crate::collision::Shape;
use crate::math::Vec2;
pub struct Body {
    pub id: u32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub shape: Shape,
    pub restitution: f64,
    pub mass: f64,
    pub inv_mass: f64,
    pub friction: f64,
    pub angle: f64,
    pub angular_vel: f64,
    pub inertia: f64,
    pub inv_inertia: f64,
}

impl Body {
    pub fn new(
        id: u32,
        pos: Vec2,
        shape: Shape,
        mass: f64,
        friction: f64,
        angle: f64,
        angular_vel: f64,
        inertia: f64,
        inv_inertia: f64,
    ) -> Self {
        Self {
            id,
            pos,
            shape,
            mass,
            vel: Vec2::ZERO,
            restitution: 0.8,
            inv_mass: if mass > 0.0 { 1.0 / mass } else { 0.0 },
            friction,
            angle,
            angular_vel,
            inertia,
            inv_inertia,
        }
    }
}

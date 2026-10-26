use crate::math::Vec2;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f64,
}

impl Transform {
    pub const IDENTITY: Transform = Transform {
        position: Vec2::ZERO,
        rotation: 0.0,
    };

    pub const fn new(position: Vec2, rotation: f64) -> Self {
        Self { position, rotation }
    }

    pub const fn from_rotation(rotation: f64) -> Self {
        Self {
            position: Vec2::ZERO,
            rotation,
        }
    }
}

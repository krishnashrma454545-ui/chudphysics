use crate::collision::Shape;
use crate::math::Vec2;
#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    pub id: u32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub shape: Shape,
    pub restitution: f64,
}

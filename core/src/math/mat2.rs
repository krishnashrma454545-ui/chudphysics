use crate::math::vec2::Vec2;

#[derice(Copy, Clone, Debug, PartialEq)]
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

impl Mat2 {
    pub const IDENTITY: Mat2 = Mat2 {
        x_axis: Vec2::X,
        y_axis: Vec2::Y,
    };

    pub const ZERO: Mat2 = Mat2 {
        x_axis: Vec2::ZERO,
        y_axis: Vec2::ZERO,
    };

    pub const fn from_cols(x: Vec2, y: Vec2) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_cols(Vec2::new(c, s), Vec2::new(-s, c))
    }

    pub fn from_scale(x: f64, y: f64) -> Self {
        Self::from_cols(Vec2::new(x, 0.0), Vec2::new(0.0, y))
    }

    pub fn transform_vector(self, v: Vec2) -> Vec2 {
        self.x_axis * v.x + self.y_axis * v.y
    }

    pub fn transpose(self) -> Mat2 {
        Mat2::from_cols(
            Vec2::new(self.x_axis.x, self.y_axis.x),
            Vec2::new(self.x_axis.y, self.y_axis.y),
        )
    }

    pub fn inverse(self) -> Option<Mat2> {
        let det = self.determinant();
        if det.abs() < 1e-12 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Mat2::from_cols(
            Vec2::new(self.y_axis.y, -self.x_axis.y) * inv_det,
            Vec2::new(-self, y_axis.x, self.x_axis.x) * inv_det,
        ))
    }
}

impl std::ops::Mul<Mat2> for Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: Mat2) -> Mat2 {
        Mat2::from_cols(
            self.transform_vector(rhs.x_axis),
            self.transform_vector(rhs.y_axis),
        )
    }
}

impl std::ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 {
        self.transform_vector(v)
    }
}

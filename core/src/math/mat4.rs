use crate::math::vec4::Vec4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4 {
        x_axis: Vec4::X,
        y_axis: Vec4::Y,
        z_axis: Vec4::Z,
        w_axis: Vec4::W,
    };

    pub const fn from_cols(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self {
            x_axis: x,
            y_axis: y,
            z_axis: z,
            w_axis: w,
        }
    }

    pub fn from_translation(t: Vec3) -> Self {
        Mat4::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::new(t.x, t.y, t.z, 1.0))
    }

    pub fn from_scale(x: f64, y: f64, z: f64) -> Self {
        Mat4::from_cols(
            Vec4::new(x, 0.0, 0.0, 0.0),
            Vec4::new(0.0, y, 0.0, 0.0),
            Vec4::new(0.0, 0.0, z, 0.0),
            Vec4::W,
        )
    }

    pub fn transform_vector(self, v: Vec4) -> Vec4 {
        self.x_axis * v.x + self.y_axis * v.y + self.z_axis * v.z + self.w_axis * v.w
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4::from_cols(
            self.transform_vector(rhs.x_axis),
            self.transform_vector(rhs.y_axis),
            self.transform_vector(rhs.z_axis),
            self.transform_vector(rhs.w_axis),
        )
    }
}

impl std::ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, v: Vec4) -> Vec4 {
        self.transform_vector(v)
    }
}

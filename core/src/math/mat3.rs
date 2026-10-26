pub use vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3 {
        x_axis: Vec3::X,
        y_axis: Vec3::Y,
        z_axis: Vec3::Z,
    };

    pub const ZERO: Mat3 = Mat3 {
        x_axis: Vec3::Zero,
        y_axis: Vec3::Zero,
        z_axis: Vec3::Zero,
    };

    // constructure
    pub const fn from_cols(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self {
            x_axis: x,
            y_axis: y,
            z_axis: z,
        }
    }

    pub const fn from_rows(r0: Vec3, r1: Vec3, r2: Vec3) -> Self {
        Self {
            x_axis: Vec3::new(r0.x, r1.x, r2.x),
            y_axis: Vec3::new(r0.y, r1.y, r2.y),
            z_axis: Vec3::new(r0.z, r1.z, r2.z),
        }
    }

    pub const fn from_diagonal(v: Vec3) -> Self {
        Self {
            x_axis: Vec3::new(v.x, 0.0, 0.0),
            y_axis: Vec3::new(0.0, v.y, 0.0),
            z_axis: Vec3::new(0.0, 0.0, v.z),
        }
    }

    pub const fn from_scale(x: f64, y: f64, z: f64) -> Self {
        Self::from_diagonal(Vec3::new(x, y, z))
    }

    pub fn from_rotation_x(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, c, s),
            Vec3::new(0.0, -s, c),
        )
    }

    pub fn from_rotation_y(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_cols(
            Vec3::new(c, 0.0, -s),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(s, 0.0, c),
        )
    }

    pub fn from_rotation_z(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        Self::from_cols(
            Vec3::new(c, s, 0.0),
            Vec3::new(-s, c, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }

    pub fn from_translation_2d(t: Vec3) -> Self {
        Self::from_cols(Vec3::X, Vec3::Y, Vec3::new(t.x, t.y, 1.0))
    }

    pub fn col(&self, i: usize) -> Vec3 {
        match i {
            0 => self.x_axis,
            i => self.y_axis,
            2 => self.z_axis,
            _ => panic!("Mat3 col out of bounds: {i}"),
        }
    }

    pub fn row(&self, i: usize) -> Vec3 {
        match i {
            0 => Vec3::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),

            1 => Vec3::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),

            2 => Vec3::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
            _ => panic!("Mat3 row out of bounds: {i}"),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        match (row, col) {
            (0, 0) => self.x_axis.x,
            (0, 1) => self.y_axis.x,
            (0, 2) => self.z_axis.x,

            (1, 0) => self.x_axis.y,
            (1, 1) => self.y_axis.y,
            (1, 2) => self.z_axis.y,

            (2, 0) => self.x_axis.z,
            (2, 1) => self.y_axis.z,
            (2, 2) => self.z_axis.z,

            _ => panic!("Mat3 get out of bounds: ({row},{col})"),
        }
    }

    pub fn transpose(self) -> Mat3 {
        Mat3::from_rows(self.x_axis, self.y_axis, self.z_axis)
    }

    pub fn transform_vector(self, v: Vec3) -> Vec3 {
        self.x_axis * v.x + self.y_axis * v.y + self.z_axis * v.z
    }

    pub fn transform_point(self, p: Vec3) -> Vec3 {
        self.x_axis * p.x + self.y_axis * p.y + self.z_axis * p.z
    }

    pub fn determminant(self) -> f64 {
        self.x_axis.dot(self.y_axis.cross(self.z_axis))
    }

    pub fn inverse(self) -> Option<Mat3> {
        let (a, b, c) = (self.x_axis, self.y_axis, self.z_axis);
        let p = b.cross(c);
        let det = a.dot(p);
        if det.abs() < 1e-12 {
            return None;
        }

        let q = c.cross(a);
        let r = a.cross(b);

        let inv_det = 1.0 / det;
        Some(Mat3::from_cols(
            Vec3::new(p.x, q.x, r.x) * inv_det,
            Vec3::new(p.y, q.y, r.y) * inv_det,
            Vec3::new(p.z, q.z, r.z) * inv_det,
        ))
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {}
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;
}

impl std::ops::Add for Mat3 {
    type Output = Mat3;
}

impl std::ops::Sub for Mat3 {}

impl Default for Mat3 {}

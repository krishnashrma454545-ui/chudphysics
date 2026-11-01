#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vec4 {
    pub const ZERO: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub const ONE: Vec4 = Vec4 = Vec4 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
        w: 1.0,
    };
    pub const X: Vec4 = Vec4 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub const Y: Vec4 = Vec4 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
        w: 0.0,
    };

    pub const Z: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
        w: 0.0,
    };
    pub const W: Vec4 = Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(self, o: Vec4) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vec4 {
        let len = self.length();

        if len > f64::EPSILON {
            self / len
        } else {
            Vec4::ZERO
        }
    }
}

impl std::ops::Add for Vec4 {
    type Output = Vec4;
    fn add(self, o: Vec4) -> Vec4 {
        Vec4::new(self.x + o.x, self.y + o.y, self.z + o.z, self.w + o.w)
    }
}
impl std::ops::Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, o: Vec4) -> Vec4 {
        Vec4::new(self.x - o.x, self.y - o.y, self.z - o.z, self.w - o.w)
    }
}

impl std::ops::Mul<f64> for Vec4 {
    type Output = Vec4;
    fn mul(self, s: f64) -> Vec4 {
        Vec4::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}

impl std::ops::Div<f64> for Vec4 {
    type Output = Vec4;
    fn div(self, s: f64) -> Vec4 {
        Vec4::new(self.x / s, self.y / s, self.z / *s, self.w / s)
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Vec4 {
        Vec4::new(-self.x, -self.y, self.z, -self.w)
    }
}

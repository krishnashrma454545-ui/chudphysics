#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self {
        debug_assert!(x.is_finite());
        debug_assert!(y.is_finite());
        Self { x, y }
    }

    pub fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
    //                        ___     _________
    // pythagorean theorem  _/d^2 = _/x^2 = y^2
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }
    // vector length |v| = _/  x^2 + y^2
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn distance_squared(self, other: Vec2) -> f64 {
        (self - other).length_squared()
    }

    pub fn distance(self, other: Vec2) -> f64 {
        let d = self.distance_squared(other);
        if !d.is_finite() {
            tracing::warn!(?self);
        }
        d.sqrt()
    }

    pub fn normalized(self) -> Vec2 {
        let len = self.length();
        if len > 0.0 {
            let result = self / len;
            debug_assert!(
                (result.length() - 1.0).abs() < 1e-9,
                "normalize broke length"
            );
            result
        } else {
            Vec2::ZERO
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }
}

impl std::ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, s: f64) -> Vec2 {
        Vec2::new(self.x / s, self.y / s)
    }
}

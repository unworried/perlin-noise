#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_angle(angle: f32, length: Option<f32>) -> Self {
        if let Some(length) = length {
            Self::new(angle.cos() * length, angle.sin() * length)
        } else {
            Self::new(angle.cos(), angle.sin())
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn multiply(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn set_magnitude(&mut self, new_mag: f32) {
        let current_mag = self.magnitude();
        self.multiply(new_mag / current_mag);
    }

    pub fn limit(&mut self, max: f32) {
        let mag = self.magnitude();
        if mag > max {
            self.multiply(max / mag);
        }
    }
}

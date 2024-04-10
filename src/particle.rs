use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

use crate::vector::Vector;
use crate::SCALE;

const MAX_SPEED: f32 = 4.;

pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
}

impl Particle {
    pub fn new() -> Self {
        let x = thread_rng().gen_range(0..screen_width() as u32);
        let y = thread_rng().gen_range(0..screen_height() as u32);
        let position = Vector::new(x as f32, y as f32);

        Self {
            position,
            velocity: Vector::new(0., 0.),
            acceleration: Vector::new(0., 0.),
        }
    }

    pub fn follow(&mut self, flowfield: &[Vector]) {
        let x = (self.position.x / SCALE).floor() as usize;
        let y = (self.position.y / SCALE).floor() as usize;

        let index = x + y * ((screen_height() / SCALE).floor() as usize);
        let force = flowfield[index];
        self.acceleration.add(&force);
    }

    pub fn update(&mut self) {
        self.velocity.add(&self.acceleration);
        self.velocity.limit(MAX_SPEED);
        self.position.add(&self.velocity);
        self.acceleration.multiply(0.);
    }

    pub fn draw(&self) {
        let color = Color::from_rgba(0, 0, 0, 255);
        draw_circle(self.position.x, self.position.y, 1., color);
    }

    pub fn wrap(&mut self) {
        if self.position.x > screen_width() {
            self.position.x = 0.;
        }

        if self.position.x < 0. {
            self.position.x = screen_width();
        }

        if self.position.y > screen_height() {
            self.position.y = 0.;
        }

        if self.position.y < 0. {
            self.position.y = screen_height();
        }
    }
}

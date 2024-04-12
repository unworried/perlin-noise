use macroquad::{prelude::*, rand};

use crate::vector::Vector;
use crate::SCALE;

const MAX_SPEED: f32 = 1.;

pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
    previous_position: Vector,
}

impl Particle {
    pub fn new() -> Self {
        let x = rand::gen_range(0, 1200);
        let y = rand::gen_range(0, 800);
        let position = Vector::new(x as f32, y as f32);

        Self {
            position,
            velocity: Vector::new(0., 0.),
            acceleration: Vector::new(0., 0.),
            previous_position: position,
        }
    }

    pub fn follow(&mut self, flowfield: &[Vector]) {
        let x = (self.position.x / SCALE).floor() as usize;
        let y = (self.position.y / SCALE).floor() as usize;

        let index = x + y * ((800. / SCALE).floor() as usize);
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
        let color = Color::from_rgba(255, 255, 255, 120);
        //draw_circle(self.position.x, self.position.y, 1., color);
        draw_line(
            self.position.x,
            self.position.y,
            self.previous_position.x,
            self.previous_position.y,
            1.,
            color,
        );
        

    }

    pub fn update_previous_position(&mut self) {
        self.previous_position = self.position;
    }

    pub fn wrap(&mut self) {
        if self.position.x > 1200. {
            self.position.x = 0.;
        }

        if self.position.x < 0. {
            self.position.x = 1200.;
        }

        if self.position.y > 800. {
            self.position.y = 0.;
        }

        if self.position.y < 0. {
            self.position.y = 800.;
        }

        self.update_previous_position();
    }
}

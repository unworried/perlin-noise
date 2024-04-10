use std::f32::consts::PI;

use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

use crate::particle::Particle;
use crate::vector::Vector;

mod particle;
mod vector;

const INCREMENT: f64 = 0.1;
const SCALE: f32 = 10.;

fn window_conf() -> Conf {
    Conf {
        window_title: "Perlin Noise".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let cols = (screen_width() / SCALE).floor() as usize;
    let rows = (screen_height() / SCALE).floor() as usize;

    let seed = thread_rng().gen_range(0..u32::MAX);
    let perlin = Perlin::new(seed);
    println!("Seed: {}", seed);

    let mut particles: Vec<Particle> = Vec::new();
    for _ in 0..10000 {
        particles.push(Particle::new());
    }

    // See if there is a better way to initialize this vector
    let mut flowfield: Vec<Vector> = Vec::with_capacity(cols * rows);
    #[allow(clippy::uninit_vec)]
    unsafe {
        flowfield.set_len(cols * rows)
    };

    let mut z_offset = 0.0;
    loop {
        clear_background(WHITE);

        let mut y_offset = 0.0;
        for y in 0..rows {
            let mut x_offset = 0.0;
            for x in 0..cols {
                let index = x + y * cols;
                let angle = perlin.get([x_offset, y_offset, z_offset]) * (PI * 2. * 4.) as f64;
                let mut vector = Vector::from_angle(angle as f32, None);
                vector.set_magnitude(1.);
                flowfield[index] = vector;
                x_offset += INCREMENT;
                /*let color = Color::from_rgba(0, 0, 0, 50);
                draw_line(
                    x as f32 * SCALE,
                    y as f32 * SCALE,
                    x as f32 * SCALE + vector.x * SCALE,
                    y as f32 * SCALE + vector.y * SCALE,
                    4.,
                    color,
                );*/
            }
            y_offset += INCREMENT;

            z_offset += 0.0003;
        }

        for particle in particles.iter_mut() {
            particle.follow(&flowfield);
            particle.update();
            particle.draw();
            particle.wrap();
        }

        next_frame().await;
    }
}

use std::f32::consts::PI;

use macroquad::{prelude::*, rand};
use noise::{NoiseFn, Perlin};

use crate::particle::Particle;
use crate::vector::Vector;

mod particle;
mod vector;

const INCREMENT: f64 = 1.;
const SCALE: f32 = 100.;

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

    let seed = rand::gen_range(0, u32::MAX);
    let perlin = Perlin::default();

    println!("Seed: {}", seed);

    let mut particles: Vec<Particle> = Vec::new();
    for _ in 0..1000 {
        particles.push(Particle::new());
    }

    // See if there is a better way to initialize this vector
    let mut flowfield: Vec<Vector> = Vec::with_capacity(cols * rows);
    #[allow(clippy::uninit_vec)]
    unsafe {
        flowfield.set_len(cols * rows)
    };

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    //render_target.texture.set_filter(FilterMode::Linear);
    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., screen_width(), screen_height()));
    render_target_cam.render_target = Some(render_target.clone());

    let mut z_offset = 0.0;
    loop {
        set_camera(&render_target_cam);

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
                    2.,
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
            particle.update_previous_position();
            particle.wrap();
        }

        set_default_camera();
        clear_background(BLACK);
        draw_texture(&render_target.texture, 0., 0., WHITE);

        next_frame().await;
    }
}

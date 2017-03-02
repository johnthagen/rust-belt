//! Defines the asteroid component.
use std::f64;


use opengl_graphics::GlGraphics;
use piston_window::{Context, Size, polygon, Transformed, UpdateArgs};
use rand;

use super::super::color;
use super::{Collidable, Drawable, Positioned, Updateable, Vector};

const NUM_SEGMENTS: usize = 20;
const ANGULAR_SEGMENT: f64 = f64::consts::PI * 2.0 / NUM_SEGMENTS as f64;
pub struct Asteroid {
    pos: Vector,
    vel: Vector,
    rot: f64,
    spin: f64,
    shape: [[f64; 2]; NUM_SEGMENTS],
    window_size: Size,
}

fn generate_circle(radius: f64) -> [[f64; 2]; NUM_SEGMENTS] {
    let mut circle: [[f64; 2]; NUM_SEGMENTS] = [[0.0; 2]; NUM_SEGMENTS];
    for (index, mut vertex) in circle.iter_mut().enumerate() {
        let index_float = index as f64;
        vertex[0] = radius * (index_float * ANGULAR_SEGMENT).cos();
        vertex[1] = radius * (index_float * ANGULAR_SEGMENT).sin();
    }
    circle
}

fn randomize_shape(mut shape: [[f64; 2]; NUM_SEGMENTS], max: f64) -> [[f64; 2]; NUM_SEGMENTS] {
    let mut av_x = 0.0;
    let mut av_y = 0.0;
    for mut vertex in &mut shape {
        vertex[0] += rand::random::<f64>() * max; // % max;
        vertex[1] += rand::random::<f64>() * max;
        av_x += vertex[0];
        av_y += vertex[1];
    }
    av_x /= NUM_SEGMENTS as f64;
    av_y /= NUM_SEGMENTS as f64;
    for mut vertex in &mut shape {
        vertex[0] -= av_x; // % max;
        vertex[1] -= av_y;
    }
    shape
}

const RADIUS: f64 = 70.0;
fn generate_shape() -> [[f64; 2]; NUM_SEGMENTS] {
    let new_shape: [[f64; 2]; NUM_SEGMENTS] = generate_circle(RADIUS);
    let max_mut = RADIUS / 4.0;
    randomize_shape(new_shape, max_mut)
}

impl Asteroid {
    pub fn new(window_size: Size) -> Asteroid {
        Asteroid {
            pos: Vector {
                x: 400.0,
                y: 400.0,
            },
            vel: Vector {
                x: rand::random::<f64>() - 0.5,
                y: rand::random::<f64>() - 0.5,
            },
            rot: 0.0,
            spin: rand::random::<f64>() * f64::consts::PI / 180.0,
            shape: generate_shape(),
            window_size: window_size,
        }
    }
}

impl Updateable for Asteroid {
    #[allow(unused_variables)]
    fn update(&mut self, args: UpdateArgs) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;
        self.rot += self.spin;
    }
}

impl Drawable for Asteroid {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        polygon(color::WHITE,
                &self.shape,
                context.transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot),
                graphics)
    }
}

impl Positioned for Asteroid {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for Asteroid {
    fn radius(&self) -> f64 {
        RADIUS / 2.0
    }
}

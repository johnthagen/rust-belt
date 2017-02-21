//! Defines the asteroid component.
use std::f64;


use opengl_graphics::GlGraphics;
use piston_window::{Context, Size, polygon, Transformed, types, UpdateArgs};

use super::super::color;
use super::{Drawable, Updateable, Vector};

const NUM_SEGMENTS : usize = 20;
const ANGULAR_SEGMENT : f64 = f64::consts::PI*2.0/NUM_SEGMENTS as f64;
pub struct Asteroid {
    pos: Vector,
    vel: Vector,
    rot: f64,
    shape: [[f64;2];NUM_SEGMENTS],
    window_size: Size,
}

fn generate_circle(radius: f64) -> [[f64;2]; NUM_SEGMENTS]{
    let mut circle : [[f64;2];NUM_SEGMENTS] = [[0.0;2]; NUM_SEGMENTS];
    for (index, mut vertex) in circle.iter_mut().enumerate() {
        let index_float = index as f64;
        vertex[0] = radius*(index_float*ANGULAR_SEGMENT).cos();
        vertex[1] = radius*(index_float*ANGULAR_SEGMENT).sin();
    }
    circle
}

fn generate_shape() -> [[f64;2]; NUM_SEGMENTS]{
    let radius = 70.0;
    let new_shape : [[f64;2];NUM_SEGMENTS] = generate_circle(radius);
    new_shape
}

impl Asteroid {
    pub fn new(window_size: Size) -> Asteroid {
        let speed_multiplier: f64 = 4.0;
        Asteroid {
            pos: Vector{x: 400.0, y: 400.0},
            vel: Vector{x: 0.0, y: 0.0},
            rot: 0.0,
            shape: generate_shape(),
            window_size: window_size,
        }
    }
}

impl Updateable for Asteroid {
    fn update(&mut self, args: UpdateArgs) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;
    }
}

impl Drawable for Asteroid {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        polygon(color::WHITE,
                &self.shape,
                context.transform.trans(self.pos.x, self.pos.y),
                graphics)
    }
}

//! Defines the player component.
use std::f64;


use opengl_graphics::GlGraphics;
use piston_window::{Context, Size, polygon, Transformed, types, UpdateArgs};

use super::super::color;
use super::{Drawable, Updateable, Vector};

pub struct Bullet {
    pos: Vector,
    vel: Vector,
    pub ttl: i32,
    window_size: Size,
}

impl Bullet {
    pub fn new(position: Vector, velocity: Vector, direction: f64, window_size: &Size) -> Bullet {
        Bullet {
            pos: position,
            vel: Vector {
                x: direction.cos() + velocity.x,
                y: direction.sin() + velocity.y,
            },
            ttl: 200,
            window_size: *window_size,
        }
    }
}

impl Updateable for Bullet {
    #[allow(unused_variables)]
    fn update(&mut self, args: UpdateArgs) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;
        self.ttl -= 1;
    }
}

const BULLET: &'static types::Triangle = &[[0.0, -1.0 * 8.0 / 2.0], [10.0, 0.0], [0.0, 8.0 / 2.0]];

impl Drawable for Bullet {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        polygon(color::WHITE,
                BULLET,
                context.transform.trans(self.pos.x, self.pos.y),
                graphics)
    }
}

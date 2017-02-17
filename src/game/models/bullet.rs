//! Defines the player component.
use std::f64;


use opengl_graphics::GlGraphics;
use piston_window::{Context, Size, ellipse, Transformed, types, UpdateArgs};

use super::super::color;
use super::{Drawable, Updateable, Vector};

pub struct Bullet {
    pos: Vector,
    vel: Vector,
    ttl: f64,
    window_size: Size,
}

const SPEED_MULTIPLIER: f64 = 4.0;

impl Bullet {
    pub fn new(position: Vector, velocity: Vector, direction: f64, window_size: Size) -> Bullet {
        Bullet {
            pos: position,
            vel: Vector {
                x: SPEED_MULTIPLIER * direction.cos() + velocity.x,
                y: SPEED_MULTIPLIER * direction.sin() + velocity.y,
            },
            ttl: 2.0,
            window_size: window_size,
        }
    }

    pub fn ttl(&self) -> f64 {
        self.ttl
    }
}

impl Updateable for Bullet {
    fn update(&mut self, args: UpdateArgs) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;
        self.ttl -= args.dt;
    }
}

const BULLET_DIAMETER: f64 = 3.0;
const BULLET: types::Rectangle = [0.0, 0.0, BULLET_DIAMETER, BULLET_DIAMETER];

impl Drawable for Bullet {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        ellipse(color::WHITE,
                BULLET,
                context.transform.trans(self.pos.x, self.pos.y),
                graphics)
    }
}

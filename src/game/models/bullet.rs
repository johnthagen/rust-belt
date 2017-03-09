//! Defines the bullet component.
use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{Context, ellipse, Size, Transformed, types, UpdateArgs};

use super::super::color;
use super::{Collidable, Drawable, Positioned, Updateable, Vector};

pub struct Bullet {
    pos: Vector,
    vel: Vector,
    ttl: f64,
    window_size: Size,
}

impl Bullet {
    pub fn new(position: Vector, velocity: Vector, direction: f64, window_size: Size) -> Bullet {
        const SPEED_MULTIPLIER: f64 = 4.0;
        Bullet {
            pos: position,
            vel: Vector {
                x: SPEED_MULTIPLIER * direction.cos() + velocity.x,
                y: SPEED_MULTIPLIER * direction.sin() + velocity.y,
            },
            ttl: 1.0,
            window_size: window_size,
        }
    }

    pub fn ttl(&self) -> f64 {
        self.ttl
    }
}

impl Updateable for Bullet {
    fn update(&mut self, args: UpdateArgs) {
        self.pos += self.vel + self.window_size.into();
        self.pos %= self.window_size.into();
        self.ttl -= args.dt;
    }
}

const BULLET_DIAMETER: f64 = 3.0;
impl Drawable for Bullet {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        const BULLET: types::Rectangle = [0.0, 0.0, BULLET_DIAMETER, BULLET_DIAMETER];

        ellipse(color::WHITE,
                BULLET,
                context.transform.trans(self.pos.x, self.pos.y),
                graphics)
    }
}

impl Positioned for Bullet {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for Bullet {
    fn radius(&self) -> f64 {
        BULLET_DIAMETER / 2.0
    }
}

//! Defines the bullet component.
//!
//! `Bullet`s are created with a specific position and velocity
//! which they use to move across the screen. They are not
//! responsible for removing themselves or handling collisions with
//! other models.

use opengl_graphics::GlGraphics;
use piston_window::{ellipse, types, Context, Size, Transformed, UpdateArgs};

use game::color;
use game::models::{Collidable, Drawable, Positioned, Updateable};
use game::models::vector::Vector;

pub struct Bullet {
    pos: Vector,
    vel: Vector,
    ttl: f64,
    window_size: Size,
}

impl Bullet {
    pub fn new(position: Vector, velocity: Vector, direction: f64, window_size: Size) -> Self {
        let speed_multiplier = 4.0;
        Bullet {
            pos: position,
            vel: Vector {
                x: speed_multiplier * direction.cos() + velocity.x,
                y: speed_multiplier * direction.sin() + velocity.y,
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

        ellipse(
            color::WHITE,
            BULLET,
            context.transform.trans(self.pos.x, self.pos.y),
            graphics,
        )
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

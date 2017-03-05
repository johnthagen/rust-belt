//! Traits and types common to models.

pub mod player;
pub mod bullet;
pub mod asteroid;

use std::ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign};

use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs};

/// Models an (x, y) coordinate value (such as position or velocity).
#[derive(Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Rem for Vector {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Vector {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl RemAssign for Vector {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

/// Trait implemented by types that can be drawn to a window.
pub trait Drawable {
    /// Draws the entity to the screen.
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}

/// Trait implemented by types that are updated by a game loop.
pub trait Updateable {
    /// Update the state of a type within a game loop.
    fn update(&mut self, args: UpdateArgs);
}

pub trait Positioned {
    fn x(&self) -> f64 {
        self.pos().x
    }

    fn y(&self) -> f64 {
        self.pos().y
    }

    fn pos(&self) -> Vector;
}

pub trait Collidable: Positioned {
    fn radius(&self) -> f64;

    fn collides_with<C: Collidable>(&self, other: &C) -> bool {
        // The Distance Formula.
        let distance = ((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).sqrt();
        distance < self.radius() + other.radius()
    }
}

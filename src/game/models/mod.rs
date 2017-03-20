//! Traits and types common to models.
use std::f64;
use std::f64::consts::PI;

use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs};

pub const PI_MULT_2: f64 = 2.0 * PI;

pub mod player;
pub mod bullet;
pub mod asteroid;
pub mod vector;

/// Trait implemented by types that can be drawn to a window.
pub trait Drawable {
    /// Draws oneself to the screen.
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}

/// Trait implemented by types that are updated by a game loop.
pub trait Updateable {
    /// Update the state of a type within a game loop.
    fn update(&mut self, args: UpdateArgs);
}

/// Defines how a type can expose its position.
pub trait Positioned {
    fn x(&self) -> f64 {
        self.pos().x
    }

    fn y(&self) -> f64 {
        self.pos().y
    }

    fn pos(&self) -> vector::Vector;
}

/// Defines how types can expose how they can check for collisions with each other.
pub trait Collidable: Positioned {
    fn radius(&self) -> f64;

    /// Check another `Collidable` type to see if it's radius overlaps with this instance's.
    fn collides_with<C: Collidable>(&self, other: &C) -> bool {
        // The Distance Formula.
        let distance = ((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).sqrt();
        distance < self.radius() + other.radius()
    }
}

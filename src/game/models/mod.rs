//! Traits and types common to models.

pub mod player;

use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs};

/// Models an (x, y) coordinate value (such as position or velocity).
pub struct Vector {
    x: f64,
    y: f64,
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

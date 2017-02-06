//! Trait implemented by types that can be drawn to a window.

use opengl_graphics::GlGraphics;
use piston_window::Context;

pub trait Drawable {
    /// Draws the entity to the screen.
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}

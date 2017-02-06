//! Trait implemented by types that are updated by a game loop.

use piston_window::UpdateArgs;

pub trait Updateable {
    /// Update the state of a type within a game loop.
    fn update(&mut self, args: UpdateArgs);
}
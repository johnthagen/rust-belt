//! Trait implemented by types that are updated by a game loop.

use piston_window::UpdateArgs;

pub trait Updateable {
    /// Update the state of a type within a game loop.
    fn update_logic(&mut self, args: UpdateArgs);

    fn update_location(&mut self);
}

pub fn update<T: Updateable>(x: &mut T, args: UpdateArgs) {
    x.update_location();
    x.update_logic(args);
}

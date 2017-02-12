//! Defines the game component.
use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Input, Key, PistonWindow, Size};

pub mod color;
mod models;

use self::models::{Drawable, player, Updateable};

/// Stores Game state.
pub struct Game {
    player: player::Player,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Game { player: player::Player::new(window_size) }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics) {
        while let Some(event) = window.next() {
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        clear(color::BLACK, graphics);
                        self.player.draw(context, graphics);
                    });
                }

                Input::Update(args) => {
                    self.player.update(args);
                }

                Input::Press(Button::Keyboard(key)) => {
                    match key {
                        Key::D => self.player.actions.rotate_cw = true,
                        Key::A => self.player.actions.rotate_ccw = true,
                        Key::S => self.player.actions.fire_rev_boosters = true,
                        Key::W => self.player.actions.fire_boosters = true,
                        Key::X => break,
                        _ => {}
                    }
                }

                Input::Release(Button::Keyboard(key)) => {
                    match key {
                        Key::D => self.player.actions.rotate_cw = false,
                        Key::A => self.player.actions.rotate_ccw = false,
                        Key::S => self.player.actions.fire_rev_boosters = false,
                        Key::W => self.player.actions.fire_boosters = false,
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}

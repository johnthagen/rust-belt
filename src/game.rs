//! Defines the game component.
use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Input, Key, PistonWindow, polygon, Transformed, types, Size};

use color;
use drawable::Drawable;
use player;

/// Stores Game state.
pub struct Game {
    player: player::Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: player::Player::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, window_size: Size) {
        self.player.set_window_size(window_size.width, window_size.height);
        while let Some(event) = window.next() {
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        clear(color::BLACK, graphics);
                        self.player.draw(context, graphics);
                    });
                }

                Input::Update(args) => {
                    if self.player.actions.rotate_cw {
                        self.player.rotate_cw(args.dt)
                    }
                    if self.player.actions.rotate_ccw {
                        self.player.rotate_ccw(args.dt)
                    }
                    if self.player.actions.fire_rev_boosters {
                        self.player.fire_rev_boosters(args.dt)
                    }
                    if self.player.actions.fire_boosters {
                        self.player.fire_boosters(args.dt)
                    }
                    self.player.update();
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

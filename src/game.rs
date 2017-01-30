//! Defines the game component.

use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Input, Key, PistonWindow, polygon, Transformed, types};

use color;

use player;

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;

const SHIP: &'static types::Triangle =
    &[[0.0, -1.0 * SHIP_HEIGHT / 2.0], [SHIP_WIDTH, 0.0], [0.0, SHIP_HEIGHT / 2.0]];

#[derive(Clone, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

/// Stores Game state.
pub struct Game {
    player: player::Player,
}

impl Game {
    pub fn new() -> Self {
        Game { player: player::Player::new() }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, window_size: &Size) {
        self.player.set_window_size(window_size.width, window_size.height);
        while let Some(event) = window.next() {
            //Self::wrap_position(&mut self.position, window_size);
            let (x, y) = self.player.get_position();
            let rot = self.player.get_rotation();
            match event {
                Input::Render(args) => {
                    self.player.update();
                    opengl.draw(args.viewport(), |context, graphics| {
                        clear(color::BLACK, graphics);
                        polygon(color::CYAN,
                                SHIP,
                                context.transform
                                    .trans(x,
                                           y)
                                    .rot_rad(rot)
                                    // Without this trans(), rotation occurs around the
                                    // upper left corner rather than the center.
                                    .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                                graphics);
                    });
                }

                Input::Press(Button::Keyboard(key)) => {
                    match key {
                        Key::D => self.player.rotate_cw(),
                        Key::A => self.player.rotate_ccw(),
                        Key::S => self.player.fire_rev_boosters(),
                        Key::W => self.player.fire_boosters(),
                        //Key::Q => self.rotation -= 0.1,
                        //Key::E => self.rotation += 0.1,
                        Key::X => break,
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}

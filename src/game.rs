//! Defines the game component.

use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Input, Key, PistonWindow, polygon, Transformed, types, Size};

use color;
use player;

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;

const SHIP: &'static types::Triangle =
    &[[0.0, -1.0 * SHIP_HEIGHT / 2.0], [SHIP_WIDTH, 0.0], [0.0, SHIP_HEIGHT / 2.0]];

/// Stores Game state.
pub struct Game {
    player: player::Player,
    actions: Actions,
}

/// Currently active user actions.
#[derive(Default)]
pub struct Actions {
    rotate_cw: bool,
    rotate_ccw: bool,
    fire_boosters: bool,
    fire_rev_boosters: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            player: player::Player::new(),
            actions: Actions::default(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, window_size: Size) {
        self.player.set_window_size(window_size.width, window_size.height);
        while let Some(event) = window.next() {
            let (pos_x, pos_y) = self.player.pos();
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        clear(color::BLACK, graphics);
                        polygon(color::CYAN,
                                SHIP,
                                context.transform
                                    .trans(pos_x, pos_y)
                                    .rot_rad(self.player.rot())
                                    // Without this trans(), rotation occurs around the
                                    // upper left corner rather than the center.
                                    .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                                graphics);
                    });
                }

                Input::Update(args) => {
                    if self.actions.rotate_cw {
                        self.player.rotate_cw(args.dt)
                    }
                    if self.actions.rotate_ccw {
                        self.player.rotate_ccw(args.dt)
                    }
                    if self.actions.fire_rev_boosters {
                        self.player.fire_rev_boosters(args.dt)
                    }
                    if self.actions.fire_boosters {
                        self.player.fire_boosters(args.dt)
                    }
                    self.player.update();
                }

                Input::Press(Button::Keyboard(key)) => {
                    match key {
                        Key::D => self.actions.rotate_cw = true,
                        Key::A => self.actions.rotate_ccw = true,
                        Key::S => self.actions.fire_rev_boosters = true,
                        Key::W => self.actions.fire_boosters = true,
                        Key::X => break,
                        _ => {}
                    }
                }

                Input::Release(Button::Keyboard(key)) => {
                    match key {
                        Key::D => self.actions.rotate_cw = false,
                        Key::A => self.actions.rotate_ccw = false,
                        Key::S => self.actions.fire_rev_boosters = false,
                        Key::W => self.actions.fire_boosters = false,
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}

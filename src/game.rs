//! Defines the game component.

use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Event, Input, Key, PistonWindow, polygon, Transformed, types};

use color;

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;

const SHIP: &'static types::Triangle = &[
    [0.0, -1.0 * SHIP_HEIGHT / 2.0],
    [SHIP_WIDTH, 0.0],
    [0.0, SHIP_HEIGHT / 2.0]
];

#[derive(Clone, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

pub struct Position {
    x: f64,
    y: f64,
}

/// Stores Game state.
pub struct Game {
    position: Position,

    /// Rotation in radians.
    rotation: f64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            position: Position {
                x: 10.0,
                y: 10.0,
            },
            rotation: 0.0,
        }
    }

    fn wrap(k: &mut f64, bound: f64) {
        if *k < 0.0 {
            *k += bound;
        } else if *k >= bound {
            *k -= bound;
        }
    }

    /// Wraps a position within a Size (e.g. the Window size).
    fn wrap_position(position: &mut Position, size: &Size) {
        Self::wrap(&mut position.x, size.width);
        Self::wrap(&mut position.y, size.height);
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics, window_size: &Size) {
        while let Some(event) = window.next() {
            Self::wrap_position(&mut self.position, window_size);

            match event {
                Event::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        clear(color::BLACK, graphics);
                        polygon(color::CYAN,
                                SHIP,
                                context.transform
                                    .trans(self.position.x,
                                           self.position.y)
                                    .rot_rad(self.rotation)
                                    // Without this trans(), rotation occurs around the
                                    // upper left corner rather than the center.
                                    .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                                graphics);
                    });
                }

                Event::Input(Input::Press(Button::Keyboard(key))) => {
                    match key {
                        Key::D => { self.position.x += 1.0 }
                        Key::A => { self.position.x -= 1.0 }
                        Key::S => { self.position.y += 1.0 }
                        Key::W => { self.position.y -= 1.0 }
                        Key::Q => { self.rotation -= 0.1 }
                        Key::E => { self.rotation += 0.1 }
                        Key::X => { break }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}
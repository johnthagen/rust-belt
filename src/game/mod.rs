//! Defines the game component.
use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Input, Key, PistonWindow, Size};

pub mod color;
mod models;

use self::models::{Drawable, player, Updateable, bullet, asteroid};

/// Stores Game state.
pub struct Game {
    player: player::Player,
    bullets: Vec<bullet::Bullet>,
    asteroids: Vec<asteroid::Asteroid>,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Game {
            player: player::Player::new(window_size),
            bullets: Vec::new(),
            asteroids: Vec::new(),
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics) {
        self.asteroids.push(asteroid::Asteroid::new(self.player.window_size));
        while let Some(event) = window.next() {
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        clear(color::BLACK, graphics);
                        for bullet in &self.bullets {
                            bullet.draw(context, graphics);
                        }
                        self.player.draw(context, graphics);
                        for asteroid in &self.asteroids {
                            asteroid.draw(context, graphics);
                        }
                    });
                }

                Input::Update(args) => {
                    self.player.update(args);
                    if self.player.should_shoot() {
                        self.bullets.push(bullet::Bullet::new(self.player.pos,
                                                              self.player.vel,
                                                              self.player.rot,
                                                              self.player.window_size));
                        self.player.reset_cooldown();
                    }
                    for bullet in &mut self.bullets {
                        bullet.update(args);
                    }
                    self.bullets.retain(|bullet| bullet.ttl() > 0.0);
                    for asteroid in &mut self.asteroids {
                        asteroid.update(args);
                    }
                }

                Input::Press(Button::Keyboard(key)) => {
                    match key {
                        Key::D => self.player.actions.rotate_cw = true,
                        Key::A => self.player.actions.rotate_ccw = true,
                        Key::S => self.player.actions.fire_rev_boosters = true,
                        Key::W => self.player.actions.fire_boosters = true,
                        Key::Space => self.player.actions.is_shooting = true,
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
                        Key::Space => self.player.actions.is_shooting = false,
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}

//! Defines the game component.
use opengl_graphics::GlGraphics;
use piston_window::{Button, clear, Input, Key, PistonWindow, Size};

pub mod color;
mod models;

use self::models::{Collidable, Drawable, player, Updateable, bullet, asteroid};

/// Stores Game state.
pub struct Game {
    player: player::Player,
    bullets: Vec<bullet::Bullet>,
    asteroids: Vec<asteroid::Asteroid>,
    num_asteroids: f64,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Game {
            player: player::Player::new(window_size),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            num_asteroids: 1.0,
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow, opengl: &mut GlGraphics) {
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

                    // Shorten lifetimes due to issues trying to pass `self` into a closure.
                    let mut num_asteroids_destroyed = 0;
                    {
                        let bullets = &mut self.bullets;
                        let asteroids = &mut self.asteroids;
                        let player = &mut self.player;

                        bullets.retain(|bullet| {
                            // Remove the first asteroid that collides with a bullet, if any.
                            if let Some(index) = asteroids.iter()
                                .position(|asteroid| asteroid.collides_with(bullet)) {
                                asteroids.remove(index);
                                num_asteroids_destroyed += 1;
                                return false;
                            }
                            true
                        });

                        // If player hits an asteroid, return to the main menu.
                        if asteroids.iter().any(|asteroid| asteroid.collides_with(player)) {
                            break;
                        }
                    }
                    self.num_asteroids += 0.3 * num_asteroids_destroyed as f64;
                    if self.asteroids.len() == 0 {
                        for _ in 0..self.num_asteroids.floor() as u32 {
                            self.asteroids.push(asteroid::Asteroid::new(self.player.window_size,
                                                                        self.player.pos));
                        }
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

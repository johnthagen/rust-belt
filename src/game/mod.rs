//! Defines the game component.
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Input, Key, PistonWindow, Size, text, Transformed};

pub mod color;
mod models;

use self::models::{Collidable, Drawable, player, Updateable, bullet, asteroid};

/// Stores Game state.
pub struct Game {
    player: player::Player,
    bullets: Vec<bullet::Bullet>,
    asteroids: Vec<asteroid::Asteroid>,
    score: i64,
    glyph_cache: GlyphCache<'static>,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Game {
            player: player::Player::new(window_size),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            score: 0,
            glyph_cache: GlyphCache::new("./assets/FiraSans-Regular.ttf").unwrap(),
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

                        text(color::YELLOW,
                             26,
                             format!("Score: {}", self.score).as_str(),
                             &mut self.glyph_cache,
                             context.transform
                                 .trans(10.0, 20.0),
                             graphics);
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
                    {
                        let bullets = &mut self.bullets;
                        let asteroids = &mut self.asteroids;
                        let player = &mut self.player;
                        let score = &mut self.score;

                        bullets.retain(|bullet| {
                            // Remove the first asteroid that collides with a bullet, if any.
                            if let Some(index) = asteroids.iter()
                                .position(|asteroid| asteroid.collides_with(bullet)) {
                                asteroids.remove(index);
                                *score += 10;
                                return false;
                            }
                            true
                        });

                        // If player hits an asteroid, return to the main menu.
                        if asteroids.iter().any(|asteroid| asteroid.collides_with(player)) {
                            break;
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

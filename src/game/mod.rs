//! Defines the game component.
//!
//! `Game` is the primary type that holds information about the
//! current state of the game and the objects that exist within it.
//!
//! `Game` takes user input from the keyboard in order to control the `Player`
//! and handles collision detection and the TTL for `Bullet`s.

use glutin_window::GlutinWindow;
use graphics::{clear, Context, text, Transformed};
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::Events;
use piston::input::{Button, Input, Key, UpdateArgs};
use piston::window::Size;

use self::models::{asteroid, bullet, Collidable, Drawable, player, Updateable};

pub mod color;
mod models;

/// Stores Game state and all objects that exist.
pub struct Game {
    player: player::Player,

    /// The bullets that are currently live in the window.
    /// Bullets are removed when their TTL expires.
    bullets: Vec<bullet::Bullet>,
    asteroids: Vec<asteroid::Asteroid>,
    score: i64,
    window_size: Size,
    asteroid_timer: f64,
    asteroid_timer_max: f64,

    /// A flag indicating if the player has lost.
    game_over: bool,
}

impl Game {
    pub fn new(window_size: Size) -> Self {
        Game {
            player: player::Player::new(window_size),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            score: 0,
            window_size: window_size,
            asteroid_timer: 0.1,
            asteroid_timer_max: 4.0,
            game_over: false,
        }
    }

    pub fn run(&mut self,
               events: &mut Events,
               window: &mut GlutinWindow,
               opengl: &mut GlGraphics,
               glyph_cache: &mut GlyphCache) {
        while let Some(event) = events.next(window) {
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(),
                                |context, graphics| self.draw(context, graphics, glyph_cache));
                }

                Input::Update(args) => {
                    self.update(args);
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

            if self.game_over {
                break;
            }
        }
    }

    /// Draws all current live objects onto the screen as well as the current score.
    fn draw(&self, context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
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
             glyph_cache,
             context.transform.trans(10.0, 20.0),
             graphics);
    }
}

impl Updateable for Game {
    fn update(&mut self, args: UpdateArgs) {
        self.player.update(args);
        if self.player.should_shoot() {
            self.bullets
                .push(bullet::Bullet::new(self.player.pos,
                                          self.player.vel,
                                          self.player.rot,
                                          self.window_size));
            self.player.reset_weapon_cooldown();
        }

        // Update bullet position and remove those that time out.
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
                if let Some(index) = asteroids
                       .iter()
                       .position(|asteroid| asteroid.collides_with(bullet)) {
                    asteroids.remove(index);
                    *score += 10;
                    return false;
                }
                true
            });

            // If player hits an asteroid, return to the main menu.
            if asteroids
                   .iter()
                   .any(|asteroid| asteroid.collides_with(player)) {
                self.game_over = true;
            }
        }

        // Countdown a timer which controls when the next asteroid is spawned.
        self.asteroid_timer -= args.dt;
        if self.asteroid_timer < 0.0 {
            self.asteroids
                .push(asteroid::Asteroid::new(self.window_size));

            // After spawning an asteroid, reduce the timer to spawn the next
            // so that asteroids gradually being spawning faster and faster, up to a
            // certain limit.
            if self.asteroid_timer_max > 0.5 {
                self.asteroid_timer_max -= 0.075;
            }
            self.asteroid_timer = self.asteroid_timer_max;
        }
    }
}

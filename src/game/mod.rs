//! Defines the game component.
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Context, Input, Key, PistonWindow, Size, text, Transformed,
                    UpdateArgs};

use self::models::{asteroid, bullet, Collidable, Drawable, player, Updateable};

pub mod color;
mod models;

/// Stores Game state.
pub struct Game {
    player: player::Player,
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
               window: &mut PistonWindow,
               opengl: &mut GlGraphics,
               glyph_cache: &mut GlyphCache) {
        while let Some(event) = window.next() {
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
            self.bullets.push(bullet::Bullet::new(self.player.pos,
                                                  self.player.vel,
                                                  self.player.rot,
                                                  self.window_size));
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
                if let Some(index) = asteroids.iter().position(|asteroid| {
                                                                   asteroid.collides_with(bullet)
                                                               }) {
                    asteroids.remove(index);
                    *score += 10;
                    return false;
                }
                true
            });

            // If player hits an asteroid, return to the main menu.
            if asteroids.iter().any(|asteroid| asteroid.collides_with(player)) {
                self.game_over = true;
            }
        }
        self.asteroid_timer -= args.dt;
        if self.asteroid_timer < 0.0 {
            self.asteroids.push(asteroid::Asteroid::new(self.window_size));
            if self.asteroid_timer_max > 0.5 {
                self.asteroid_timer_max -= 0.075;
            }
            self.asteroid_timer = self.asteroid_timer_max;
        }
    }
}

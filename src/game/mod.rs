//! Defines the game component.
//!
//! `Game` is the primary type that holds information about the
//! current state of the game and the objects that exist within it.
//!
//! `Game` takes user input from the keyboard in order to control the `Player`
//! and handles collision detection and the TTL for `Bullet`s.

use music;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, ReleaseEvent,
                    RenderEvent, Size, Transformed, UpdateArgs, UpdateEvent};

use self::models::{asteroid, bullet, player, powerup, Collidable, Drawable, Updateable};
use menu::{Sound, Volume};

pub mod color;
mod models;

/// Stores Game state and all objects that exist.
pub struct Game {
    player: player::Player,

    /// The bullets that are currently live in the window.
    /// Bullets are removed when their TTL expires.
    bullets: Vec<bullet::Bullet>,
    asteroids: Vec<asteroid::Asteroid>,
    powerups: Vec<powerup::PowerUp>,
    score: i64,
    window_size: Size,
    asteroid_timer: f64,
    asteroid_timer_max: f64,
    powerup_timer: f64,
    powerup_timer_max: f64,
    /// A flag indicating if the player has lost.
    /// This should not be set if the player simply quits.
    game_over: bool,
    volume: Volume,
}

impl Game {
    pub fn new(window_size: Size, volume: Volume) -> Self {
        Game {
            player: player::Player::new(window_size),
            bullets: Vec::new(),
            asteroids: Vec::new(),
            powerups: Vec::new(),
            score: 0,
            window_size: window_size,
            asteroid_timer: 0.1,
            asteroid_timer_max: 4.0,
            powerup_timer: 10.0,
            powerup_timer_max: 30.0,
            game_over: false,
            volume: volume,
        }
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn run(
        &mut self,
        window: &mut PistonWindow,
        opengl: &mut GlGraphics,
        glyph_cache: &mut GlyphCache,
    ) {
        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    self.draw(context, graphics, glyph_cache)
                });
            }

            if let Some(args) = event.update_args() {
                self.update(args);
            }

            if let Some(Button::Keyboard(key)) = event.press_args() {
                match key {
                    Key::D => self.player.actions.rotate_cw = true,
                    Key::A => self.player.actions.rotate_ccw = true,
                    Key::S => self.player.actions.fire_rev_boosters = true,
                    Key::W => self.player.actions.fire_boosters = true,
                    Key::Space => self.player.actions.is_shooting = true,
                    Key::X => {
                        music::play_sound(
                            &Sound::MenuBack,
                            music::Repeat::Times(0),
                            self.volume.sound,
                        );
                        break;
                    }
                    _ => {}
                }
            }

            if let Some(Button::Keyboard(key)) = event.release_args() {
                match key {
                    Key::D => self.player.actions.rotate_cw = false,
                    Key::A => self.player.actions.rotate_ccw = false,
                    Key::S => self.player.actions.fire_rev_boosters = false,
                    Key::W => self.player.actions.fire_boosters = false,
                    Key::Space => self.player.actions.is_shooting = false,
                    _ => {}
                }
            }

            if self.game_over {
                break;
            }
        }
    }

    /// Game over screen logic.
    pub fn run_game_over(
        &self,
        window: &mut PistonWindow,
        opengl: &mut GlGraphics,
        glyph_cache: &mut GlyphCache,
    ) {
        // Wait for the player to have pressed and release a key before
        // continuing in case they were holding a button down during
        // game over.
        let mut has_pressed = false;
        let mut has_released = false;
        while let Some(event) = window.next() {
            if let Some(args) = event.render_args() {
                opengl.draw(args.viewport(), |context, graphics| {
                    clear(color::BLACK, graphics);
                    text(
                        color::WHITE,
                        50,
                        "Game Over",
                        glyph_cache,
                        context.transform.trans(
                            f64::from(self.window_size.width / 2 - 120),
                            f64::from(self.window_size.height / 2 - 30),
                        ),
                        graphics,
                    ).unwrap();
                    let offset = (self.score.to_string().len() * 5) as u32;
                    text(
                        color::WHITE,
                        50,
                        format!("Score: {}", self.score).as_str(),
                        glyph_cache,
                        context.transform.trans(
                            f64::from(self.window_size.width / 2 - 90 - offset),
                            f64::from(self.window_size.height / 2 + 30),
                        ),
                        graphics,
                    ).unwrap();
                });
            }

            if let Some(Button::Keyboard(_)) = event.press_args() {
                if has_released {
                    break;
                }
                has_pressed = true;
            }

            if let Some(Button::Keyboard(_)) = event.release_args() {
                if has_pressed {
                    has_released = true;
                }
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

        for power in &self.powerups {
            power.draw(context, graphics);
        }

        text(
            color::YELLOW,
            26,
            format!("Score: {}", self.score).as_str(),
            glyph_cache,
            context.transform.trans(10.0, 20.0),
            graphics,
        ).unwrap();
    }
}

impl Updateable for Game {
    fn update(&mut self, args: UpdateArgs) {
        self.player.update(args);
        if self.player.should_shoot() {
            music::play_sound(
                &Sound::WeaponShoot,
                music::Repeat::Times(0),
                self.volume.sound,
            );
            self.bullets.push(bullet::Bullet::new(
                self.player.pos,
                self.player.vel,
                self.player.rot,
                self.window_size,
            ));
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

        for powerup in &mut self.powerups {
            powerup.update(args);
        }
        // Shorten lifetimes due to issues trying to pass `self` into a closure.
        {
            let bullets = &mut self.bullets;
            let asteroids = &mut self.asteroids;
            let player = &mut self.player;
            let score = &mut self.score;
            let volume = self.volume;

            bullets.retain(|bullet| {
                // Remove the first asteroid that collides with a bullet, if any.
                if let Some(index) = asteroids
                    .iter()
                    .position(|asteroid| asteroid.collides_with(bullet))
                {
                    if asteroids[index].can_split() {
                        let new_asteroids = asteroids[index].split(bullet);
                        asteroids.extend(new_asteroids);
                    }
                    asteroids.remove(index);
                    *score += 10;
                    music::play_sound(
                        &Sound::AsteroidExplosion,
                        music::Repeat::Times(0),
                        volume.sound,
                    );
                    return false;
                }
                true
            });

            // If player hits an asteroid, return to the main menu.
            if asteroids
                .iter()
                .any(|asteroid| asteroid.collides_with(player))
            {
                self.game_over = true;
            }
        }

        {
            let powerups = &mut self.powerups;
            let player = &mut self.player;


            powerups.retain(|power| {
                // Remove the first asteroid that collides with a bullet, if any.
                if player.collides_with(power){
                    player.set_powerup(power.powerup_type);
                    return false
                }
                true
            });
        }

        // Countdown a timer which controls when the next asteroid is spawned.
        self.asteroid_timer -= args.dt;
        self.powerup_timer -= args.dt;
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
        if self.powerup_timer < 0.0 {
            self.powerups.push(powerup::PowerUp::new(self.window_size));
            self.powerup_timer = self.powerup_timer_max;
        }
    }
}

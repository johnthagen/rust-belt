//! Defines the player component.
//!
//! The `Player` is the only model that reacts to user input
//! to control its movement and actions. It captures the current
//! state of the player, but does not handle any of the interaction
//! with other models itself.

use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{polygon, types, Context, Size, Transformed, UpdateArgs};

use game::color;
use game::models::{Collidable, Drawable, PI_MULT_2, Positioned, Updateable};
use game::models::powerup::PowerUpType;
use game::models::vector::Vector;

pub struct Player {
    pub pos: Vector,
    pub vel: Vector,
    pub rot: f64,
    pub actions: Actions,
    powerup_timer: f64,
    powerup_type: PowerUpType,
    weapon_cooldown: f64,
    weapon_cooldown_reset: f64,
    window_size: Size,
}

/// Currently active user actions.
#[derive(Default)]
pub struct Actions {
    pub rotate_cw: bool,
    pub rotate_ccw: bool,
    pub fire_boosters: bool,
    pub fire_rev_boosters: bool,
    pub is_shooting: bool,
}

enum Direction {
    Forward,
    Backward,
}

const ROTATION_INCREMENT: f64 = 5.0;
const THRUST_INCREMENT: f64 = 5.0;

impl Player {
    pub fn new(window_size: Size) -> Self {
        Player {
            pos: Vector {
                x: f64::from(window_size.width) / 2.0,
                y: f64::from(window_size.height) / 2.0,
            },
            vel: Vector { x: 0.0, y: 0.0 },
            rot: 0.0,
            actions: Actions::default(),
            powerup_timer: 0.0,
            powerup_type: PowerUpType::None,
            weapon_cooldown: 0.0,
            weapon_cooldown_reset: 0.25,
            window_size: window_size,
        }
    }

    fn rotate(&mut self, rot: f64) {
        self.rot = (self.rot + rot) % PI_MULT_2;
    }

    fn rotate_cw(&mut self, delta: f64) {
        self.rotate(ROTATION_INCREMENT * delta)
    }

    fn rotate_ccw(&mut self, delta: f64) {
        self.rotate(-1.0 * ROTATION_INCREMENT * delta)
    }

    fn accelerate(&mut self, delta: f64, direction: Direction) {
        let acceleration = Vector {
            x: self.rot.cos() * THRUST_INCREMENT * delta,
            y: self.rot.sin() * THRUST_INCREMENT * delta,
        };

        match direction {
            Direction::Forward => self.vel += acceleration,
            Direction::Backward => self.vel -= acceleration,
        }
    }

    fn fire_forward_boosters(&mut self, delta: f64) {
        self.accelerate(delta, Direction::Forward);
    }

    fn fire_rev_boosters(&mut self, delta: f64) {
        self.accelerate(delta, Direction::Backward);
    }

    pub fn reset_weapon_cooldown(&mut self) {
        self.weapon_cooldown = self.weapon_cooldown_reset;
    }

    pub fn set_powerup(&mut self, powerup: PowerUpType){
        self.powerup_type = powerup;
        match powerup {
            PowerUpType::FastShoot => {
                self.weapon_cooldown_reset = 0.08;
                self.powerup_timer = 5.0;
            },
            _ => {},
        }
    }

    pub fn should_shoot(&self) -> bool {
        self.weapon_cooldown == 0.0 && self.actions.is_shooting
    }
}

impl Updateable for Player {
    fn update(&mut self, args: UpdateArgs) {
        self.pos += self.vel + self.window_size.into();
        self.pos %= self.window_size.into();

        if self.actions.rotate_cw {
            self.rotate_cw(args.dt)
        }
        if self.actions.rotate_ccw {
            self.rotate_ccw(args.dt)
        }
        if self.actions.fire_rev_boosters {
            self.fire_rev_boosters(args.dt)
        }
        if self.actions.fire_boosters {
            self.fire_forward_boosters(args.dt)
        }

        if self.weapon_cooldown > 0.0 {
            self.weapon_cooldown = (self.weapon_cooldown - args.dt).max(0.0);
        }

        if self.powerup_timer > 0.0 {
            self.powerup_timer = (self.powerup_timer - args.dt).max(0.0)
        }

        if self.powerup_timer == 0.0{
            match self.powerup_type {
                PowerUpType::FastShoot => {
                    self.powerup_type = PowerUpType::None;
                    self.weapon_cooldown_reset = 0.25;
                },
                _ => {},
            }
        }
    }
}

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;
const SHIP: &types::Triangle = &[
    [0.0, -1.0 * SHIP_HEIGHT / 2.0],
    [SHIP_WIDTH, 0.0],
    [0.0, SHIP_HEIGHT / 2.0],
];
impl Drawable for Player {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        const BOOSTER_HEIGHT: f64 = 8.0;
        const BOOSTER_WIDTH: f64 = 10.0;
        const BOOSTER: &types::Triangle = &[
            [0.0, -1.0 * BOOSTER_HEIGHT / 2.0],
            [BOOSTER_WIDTH, 0.0],
            [0.0, BOOSTER_HEIGHT / 2.0],
        ];

        // Draw the boosters first, so that they look like they are coming
        // from underneath the ship.
        if self.actions.fire_boosters {
            polygon(
                color::DIM_RED,
                BOOSTER,
                context
                    .transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot + f64::consts::PI)
                    .trans(BOOSTER_HEIGHT, 0.0),
                graphics,
            );
        }
        if self.actions.fire_rev_boosters {
            polygon(
                color::DIM_RED,
                BOOSTER,
                context
                    .transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot)
                    .trans(SHIP_HEIGHT - BOOSTER_HEIGHT, 0.0),
                graphics,
            );
        }
        if self.actions.rotate_cw {
            polygon(
                color::DIM_RED,
                BOOSTER,
                context
                    .transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot - f64::consts::FRAC_PI_3),
                graphics,
            );
        }
        if self.actions.rotate_ccw {
            polygon(
                color::DIM_RED,
                BOOSTER,
                context
                    .transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot + f64::consts::FRAC_PI_3),
                graphics,
            );
        }
        polygon(
            color::CYAN,
            SHIP,
            context.transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot)
                    // Without this trans(), rotation occurs around the
                    // upper left corner rather than the center.
                    .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
            graphics,
        );
    }
}

impl Positioned for Player {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for Player {
    fn radius(&self) -> f64 {
        SHIP_WIDTH / 2.0
    }
}

//! Defines the player component.
use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{Context, polygon, Size, Transformed, types, UpdateArgs};

use super::super::color;
use super::{Collidable, Drawable, Positioned, Updateable, Vector};

pub struct Player {
    pub pos: Vector,
    pub vel: Vector,
    pub rot: f64,
    pub actions: Actions,
    weapon_cooldown: f64,
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
    pub fn new(window_size: Size) -> Player {
        Player {
            pos: Vector {
                x: window_size.width as f64 / 2.0,
                y: window_size.height as f64 / 2.0,
            },
            vel: Vector { x: 0.0, y: 0.0 },
            rot: 0.0,
            actions: Actions::default(),
            weapon_cooldown: 0.0,
            window_size: window_size,
        }
    }

    fn rotate(&mut self, rot: f64) {
        const TAU: f64 = 2.0 * f64::consts::PI;
        self.rot = (self.rot + rot) % TAU;
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

    pub fn reset_cooldown(&mut self) {
        self.weapon_cooldown = 0.25;
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
    }
}

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;
const SHIP: &'static types::Triangle =
    &[[0.0, -1.0 * SHIP_HEIGHT / 2.0], [SHIP_WIDTH, 0.0], [0.0, SHIP_HEIGHT / 2.0]];
impl Drawable for Player {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        const BOOSTER_HEIGHT: f64 = 8.0;
        const BOOSTER_WIDTH: f64 = 10.0;
        const BOOSTER: &'static types::Triangle = &[[0.0, -1.0 * BOOSTER_HEIGHT / 2.0],
                                                    [BOOSTER_WIDTH, 0.0],
                                                    [0.0, BOOSTER_HEIGHT / 2.0]];
        if self.actions.fire_boosters {
            polygon(color::DIM_RED,
                    BOOSTER,
                    context.transform
                        .trans(self.pos.x, self.pos.y)
                        .rot_rad(self.rot + f64::consts::PI)
                        .trans(BOOSTER_HEIGHT, 0.0),
                    graphics);
        }
        if self.actions.fire_rev_boosters {
            polygon(color::DIM_RED,
                    BOOSTER,
                    context.transform
                        .trans(self.pos.x, self.pos.y)
                        .rot_rad(self.rot)
                        .trans(SHIP_HEIGHT - BOOSTER_HEIGHT, 0.0),
                    graphics);
        }
        if self.actions.rotate_cw {
            polygon(color::DIM_RED,
                    BOOSTER,
                    context.transform
                        .trans(self.pos.x, self.pos.y)
                        .rot_rad(self.rot - f64::consts::FRAC_PI_3),
                    graphics);
        }
        if self.actions.rotate_ccw {
            polygon(color::DIM_RED,
                    BOOSTER,
                    context.transform
                        .trans(self.pos.x, self.pos.y)
                        .rot_rad(self.rot + f64::consts::FRAC_PI_3),
                    graphics);
        }
        polygon(color::CYAN,
                SHIP,
                context.transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot)
                    // Without this trans(), rotation occurs around the
                    // upper left corner rather than the center.
                    .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                graphics);
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

//! Defines the player component.
use std::f64;


use opengl_graphics::GlGraphics;
use piston_window::{Context, polygon, Size, Transformed, types, UpdateArgs};

use super::super::color;
use super::{Drawable, Updateable, Vector};


pub struct Player {
    pos: Vector,
    vel: Vector,
    rot: f64,
    pub actions: Actions,
    window_size: Size,
}

/// Currently active user actions.
#[derive(Default)]
pub struct Actions {
    pub rotate_cw: bool,
    pub rotate_ccw: bool,
    pub fire_boosters: bool,
    pub fire_rev_boosters: bool,
}

const ROTATION_INCREMENT: f64 = 5.0;
const THRUST_INCREMENT: f64 = 5.0;
const PI_TIMES_2: f64 = f64::consts::PI * 2.0;

impl Player {
    pub fn new(window_size: Size) -> Player {
        Player {
            pos: Vector { x: 10.0, y: 10.0 },
            vel: Vector { x: 0.0, y: 0.0 },
            rot: 0.0,
            actions: Actions::default(),
            window_size: window_size,
        }
    }

    fn rotate(&mut self, rot: f64) {
        self.rot = (self.rot + rot) % PI_TIMES_2;
    }

    fn rotate_cw(&mut self, delta: f64) {
        self.rotate(ROTATION_INCREMENT * delta)
    }

    fn rotate_ccw(&mut self, delta: f64) {
        self.rotate(-1.0 * ROTATION_INCREMENT * delta)
    }

    fn fire_boosters(&mut self, delta: f64) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT * delta;
        let boost_y = self.rot.sin() * THRUST_INCREMENT * delta;
        self.vel.x += boost_x;
        self.vel.y += boost_y;
    }

    fn fire_rev_boosters(&mut self, delta: f64) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT * delta;
        let boost_y = self.rot.sin() * THRUST_INCREMENT * delta;
        self.vel.x -= boost_x;
        self.vel.y -= boost_y;
    }
}

impl Updateable for Player {
    fn update(&mut self, args: UpdateArgs) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;

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
            self.fire_boosters(args.dt)
        }
    }
}

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;
const SHIP: &'static types::Triangle =
    &[[0.0, -1.0 * SHIP_HEIGHT / 2.0], [SHIP_WIDTH, 0.0], [0.0, SHIP_HEIGHT / 2.0]];

const BOOSTER_HEIGHT: f64 = 8.0;
const BOOSTER_WIDTH: f64 = 10.0;
const BOOSTER: &'static types::Triangle =
    &[[0.0, -1.0 * BOOSTER_HEIGHT / 2.0], [BOOSTER_WIDTH, 0.0], [0.0, BOOSTER_HEIGHT / 2.0]];

impl Drawable for Player {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
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

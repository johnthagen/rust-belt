//! Defines the player component.
use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{Context, polygon, Transformed, types};

use color;
use drawable::Drawable;

pub struct Vect {
    x: f64,
    y: f64,
}

pub struct Player {
    pos: Vect,
    max_pos: Vect,
    vel: Vect,
    rot: f64,
    actions: Actions,
}

/// Currently active user actions.
#[derive(Default)]
struct Actions {
    rotate_cw: bool,
    rotate_ccw: bool,
    fire_boosters: bool,
    fire_rev_boosters: bool,
}

const ROTATION_INCREMENT: f64 = 5.0;
const THRUST_INCREMENT: f64 = 5.0;
const PI_TIMES_2: f64 = f64::consts::PI * 2.0;

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vect { x: 10.0, y: 10.0 },
            max_pos: Vect {
                x: 1000.0,
                y: 1000.0,
            },
            vel: Vect { x: 0.0, y: 0.0 },
            rot: 0.0,
            actions: Actions::default(),
        }
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.max_pos.x = width as f64;
        self.max_pos.y = height as f64;
    }

    pub fn pos(&self) -> (f64, f64) {
        (self.pos.x, self.pos.y)
    }

    pub fn rot(&self) -> f64 {
        self.rot
    }

    pub fn update(&mut self) {
        let x = self.pos.x + self.vel.x + self.max_pos.x;
        let y = self.pos.y + self.vel.y + self.max_pos.y;
        self.pos.x = x % self.max_pos.x;
        self.pos.y = y % self.max_pos.y;
    }

    fn rotate(&mut self, rot: f64) {
        self.rot = (self.rot + rot) % PI_TIMES_2;
    }

    pub fn rotate_cw(&mut self, delta: f64) {
        self.rotate(ROTATION_INCREMENT * delta)
    }

    pub fn rotate_ccw(&mut self, delta: f64) {
        self.rotate(-1.0 * ROTATION_INCREMENT * delta)
    }

    pub fn fire_boosters(&mut self, delta: f64) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT * delta;
        let boost_y = self.rot.sin() * THRUST_INCREMENT * delta;
        self.vel.x += boost_x;
        self.vel.y += boost_y;
    }

    pub fn fire_rev_boosters(&mut self, delta: f64) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT * delta;
        let boost_y = self.rot.sin() * THRUST_INCREMENT * delta;
        self.vel.x -= boost_x;
        self.vel.y -= boost_y;
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
        //        if self.actions.fire_boosters {
        //            polygon(color::DIM_RED,
        //                    BOOSTER,
        //                    context.transform
        //                        .trans(pos_x, pos_y)
        //                        .rot_rad(self.player.rot() + f64::consts::PI)
        //                        .trans(BOOSTER_HEIGHT, 0.0),
        //                    graphics);
        //        }
        //        if self.actions.fire_rev_boosters {
        //            polygon(color::DIM_RED,
        //                    BOOSTER,
        //                    context.transform
        //                        .trans(pos_x, pos_y)
        //                        .rot_rad(self.player.rot())
        //                        .trans(SHIP_HEIGHT - BOOSTER_HEIGHT, 0.0),
        //                    graphics);
        //        }
        //        if self.actions.rotate_cw {
        //            polygon(color::DIM_RED,
        //                    BOOSTER,
        //                    context.transform
        //                        .trans(pos_x, pos_y)
        //                        .rot_rad(self.player.rot() - f64::consts::FRAC_PI_3),
        //                    graphics);
        //        }
        //        if self.actions.rotate_ccw {
        //            polygon(color::DIM_RED,
        //                    BOOSTER,
        //                    context.transform
        //                        .trans(pos_x, pos_y)
        //                        .rot_rad(self.player.rot() + f64::consts::FRAC_PI_3),
        //                    graphics);
        //        }
        polygon(color::CYAN,
                SHIP,
                context.transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot())
                    // Without this trans(), rotation occurs around the
                    // upper left corner rather than the center.
                    .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                graphics);
    }
}
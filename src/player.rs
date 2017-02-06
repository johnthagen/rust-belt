//! Defines the player component.
use std::f64;

pub struct Vector {
    x: f64,
    y: f64,
}

pub struct Player {
    pos: Vector,
    max_pos: Vector,
    vel: Vector,
    rot: f64,
}

const ROTATION_INCREMENT: f64 = 5.0;
const THRUST_INCREMENT: f64 = 5.0;
const PI_TIMES_2: f64 = f64::consts::PI * 2.0;

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vector { x: 10.0, y: 10.0 },
            max_pos: Vector {
                x: 1000.0,
                y: 1000.0,
            },
            vel: Vector { x: 0.0, y: 0.0 },
            rot: 0.0,
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

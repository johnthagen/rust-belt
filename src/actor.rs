//! Defines the Actor component.
use std::f64;

pub struct Vect {
    x: f64,
    y: f64,
}

pub struct Actor {
    pos: Vect,
    max_pos: Vect,
    vel: Vect,
    rot: f64,
}

const ROTATION_INCREMENT: f64 = 5.0;
const THRUST_INCREMENT: f64 = 5.0;
use std::f64::consts::PI;
const PI_TIMES_2: f64 = PI * 2.0;

impl Actor {
    pub fn new(pos_x: f64,
               pos_y: f64,
               width: f64,
               height: f64,
               vel_x: f64,
               vel_y: f64,
               rot: f64)
               -> Actor {
        Actor {
            pos: Vect {
                x: pos_x,
                y: pos_y,
            },
            max_pos: Vect {
                x: width,
                y: height,
            },
            vel: Vect {
                x: vel_x,
                y: vel_y,
            },
            rot: rot,
        }
    }

    pub fn set_window_size(&mut self, width: f64, height: f64) {
        self.max_pos.x = width;
        self.max_pos.y = height;
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
        self.rot = (self.rot + rot + PI_TIMES_2) % PI_TIMES_2;
    }

    pub fn get_angle_to_point(&self, dest_x: f64, dest_y: f64) -> f64 {

        let (my_x, my_y) = self.pos();
        let diff_x = dest_x - my_x;
        let diff_y = dest_y - my_y;
        let mut angle_to_point = (diff_y / diff_x).atan();
        if diff_y < 0.0 {
            angle_to_point += PI;
            if diff_x > 0.0 {
                angle_to_point += PI;
            }
        } else if diff_x < 0.0 {
            angle_to_point += PI;
        }
        return angle_to_point;
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

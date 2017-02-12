//! Defines the player component.
use std::f64;

use opengl_graphics::GlGraphics;
use piston_window::{Context, polygon, Size, Transformed, types, UpdateArgs};

use color;
use drawable::Drawable;
use updateable::Updateable;
use player::Actions;

pub struct Vector {
    x: f64,
    y: f64,
}

pub struct Enemy {
    pos: Vector,
    vel: Vector,
    rot: f64,
    actions: Actions,
    window_size: Size,
}

const ROTATION_INCREMENT: f64 = 2.0;
const THRUST_INCREMENT: f64 = 0.7;
use std::f64::consts::PI;
const PI_TIMES_2: f64 = f64::consts::PI * 2.0;
const PI_OVER_16: f64 = f64::consts::PI / 16.0;
impl Enemy {
    pub fn new(pos_x: f64,
               pos_y: f64,
               vel_x: f64,
               vel_y: f64,
               rot: f64,
               window_size: Size)
               -> Enemy {
        Enemy {
            pos: Vector {
                x: pos_x,
                y: pos_y,
            },
            vel: Vector {
                x: vel_x,
                y: vel_y,
            },
            rot: rot,
            actions: Actions::default(),
            window_size: window_size,
        }
    }

    fn get_angle_to_point(&self, dest_x: f64, dest_y: f64) -> f64 {

        let my_x = self.pos.x;
        let my_y = self.pos.y;
        let mut diff_x = dest_x - my_x;
        let mut diff_y = dest_y - my_y;
        let window_width = self.window_size.width as f64;
        let window_height = self.window_size.height as f64;
        if diff_x > window_width / 2.0 {
            diff_x -= window_width;
        } else if diff_x < -window_width / 2.0 {
            diff_x += window_width;
        }
        if diff_y > window_height / 2.0 {
            diff_y -= window_height;
        } else if diff_y < -window_height / 2.0 {
            diff_y += window_height;
        }
        let mut angle_to_point = (diff_y / diff_x).atan();
        if diff_y < 0.0 {
            angle_to_point += PI;
            if diff_x > 0.0 {
                angle_to_point += PI;
            }
        } else if diff_x < 0.0 {
            angle_to_point += PI;
        }
        angle_to_point
    }

    fn rotate(&mut self, rot: f64) {
        self.rot = (self.rot + rot + PI_TIMES_2) % PI_TIMES_2;
    }

    fn rotate_cw(&mut self, delta: f64) {
        self.rotate(ROTATION_INCREMENT * delta);
        self.actions.rotate_cw = true;
        self.actions.rotate_ccw = false;
        self.actions.fire_boosters = false;
    }

    fn rotate_ccw(&mut self, delta: f64) {
        self.rotate(-1.0 * ROTATION_INCREMENT * delta);
        self.actions.rotate_cw = false;
        self.actions.rotate_ccw = true;
        self.actions.fire_boosters = false;
    }

    fn fire_boosters(&mut self, delta: f64) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT * delta;
        let boost_y = self.rot.sin() * THRUST_INCREMENT * delta;
        self.vel.x += boost_x;
        self.vel.y += boost_y;
        self.actions.rotate_cw = false;
        self.actions.rotate_ccw = false;
        self.actions.fire_boosters = true;
    }
}

impl Updateable for Enemy {
    fn update_logic(&mut self, args: UpdateArgs) {
        //TODO: insert enemy logic
        //let distance = (diff_x.powi(2) + diff_y.powi(2)).sqrt();

        let angle_to_player = self.get_angle_to_point(500.0, 300.0);

        let angle_diff = (angle_to_player - self.rot + PI_TIMES_2) % PI_TIMES_2;

        if (angle_diff > -PI && angle_diff < -PI_OVER_16) ||
           (angle_diff < PI_TIMES_2 && angle_diff > PI) {
            self.rotate_ccw(args.dt);
        } else if angle_diff < PI && angle_diff > PI_OVER_16 {
            self.rotate_cw(args.dt);
        } else {
            self.fire_boosters(args.dt);
        }

    }

    fn update_location(&mut self) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;
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

impl Drawable for Enemy {
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
        polygon(color::YELLOW,
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

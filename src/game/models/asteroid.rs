//! Defines the asteroid component.
use std::f64;
use std::cmp;

use opengl_graphics::GlGraphics;
use piston_window::{Context, Size, polygon, Transformed, UpdateArgs};
use rand;

use super::super::color;
use super::{Collidable, Drawable, Positioned, Updateable, Vector};

const NUM_SEGMENTS: usize = 20;
type CircularPolygon = [[f64; 2]; NUM_SEGMENTS];

pub struct Asteroid {
    pos: Vector,
    vel: Vector,
    rot: f64,
    spin: f64,
    shape: CircularPolygon,
    window_size: Size,
    on_screen: bool,
}

fn generate_circle(radius: f64) -> CircularPolygon {
    const ANGULAR_SEGMENT: f64 = f64::consts::PI * 2.0 / NUM_SEGMENTS as f64;
    let mut circle = [[0.0; 2]; NUM_SEGMENTS];
    for (index, mut vertex) in circle.iter_mut().enumerate() {
        let index_float = index as f64;
        vertex[0] = radius * (index_float * ANGULAR_SEGMENT).cos();
        vertex[1] = radius * (index_float * ANGULAR_SEGMENT).sin();
    }
    circle
}

fn randomize_shape(mut shape: CircularPolygon, max: f64) -> CircularPolygon {
    let mut average_x = 0.0;
    let mut average_y = 0.0;
    for mut vertex in &mut shape {
        vertex[0] += rand::random::<f64>() * max;
        vertex[1] += rand::random::<f64>() * max;
        average_x += vertex[0];
        average_y += vertex[1];
    }
    average_x /= NUM_SEGMENTS as f64;
    average_y /= NUM_SEGMENTS as f64;
    for mut vertex in &mut shape {
        vertex[0] -= average_x;
        vertex[1] -= average_y;
    }
    shape
}

const RADIUS: f64 = 70.0;
fn generate_jagged_shape() -> CircularPolygon {
    let new_shape = generate_circle(RADIUS);
    const MAX_MUT: f64 = RADIUS / 4.0;
    randomize_shape(new_shape, MAX_MUT)
}

impl Asteroid {
    pub fn new(window_size: Size) -> Asteroid {
        let radius = cmp::max(window_size.width, window_size.height) as f64 + RADIUS;
        let angle = f64::consts::PI * 2.0 * rand::random::<f64>();
        let target = Vector::new_rand(RADIUS,
                                      window_size.width as f64,
                                      RADIUS,
                                      window_size.height as f64);
        let vel_multipler = 0.5 + rand::random::<f64>() * 0.7;
        let new_pos = Vector {
            x: window_size.width as f64 / 2.0 + radius * angle.cos(),
            y: window_size.height as f64 / 2.0 + radius * angle.sin(),
        };
        Asteroid {
            pos: new_pos,
            vel: Vector {
                x: new_pos.angle_to_vector(target).cos() * vel_multipler,
                y: new_pos.angle_to_vector(target).sin() * vel_multipler,
            },
            rot: 0.0,
            spin: (rand::random::<f64>() - 0.5) * f64::consts::PI / 180.0,
            shape: generate_jagged_shape(),
            window_size: Size {
                width: window_size.width * 2,
                height: window_size.height * 2,
            },
            on_screen: false,
        }
    }
}

impl Updateable for Asteroid {
    #[allow(unused_variables)]
    fn update(&mut self, args: UpdateArgs) {
        let x = self.pos.x + self.vel.x + self.window_size.width as f64;
        let y = self.pos.y + self.vel.y + self.window_size.height as f64;
        self.pos.x = x % self.window_size.width as f64;
        self.pos.y = y % self.window_size.height as f64;
        self.rot += self.spin;
        if self.on_screen == false {
            if self.pos.x > RADIUS && self.pos.x + RADIUS < self.window_size.width as f64 / 2.0 &&
               self.pos.y > RADIUS &&
               self.pos.y + RADIUS < self.window_size.height as f64 / 2.0 {
                self.window_size.width /= 2;
                self.window_size.height /= 2;
                self.on_screen = true;
            }
        }
    }
}

impl Drawable for Asteroid {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        polygon(color::WHITE,
                &self.shape,
                context.transform
                    .trans(self.pos.x, self.pos.y)
                    .rot_rad(self.rot),
                graphics);

        if self.pos.x + RADIUS > self.window_size.width as f64 {
            polygon(color::WHITE,
                    &self.shape,
                    context.transform
                        .trans(self.pos.x - self.window_size.width as f64, self.pos.y)
                        .rot_rad(self.rot),
                    graphics)

        } else if self.pos.x < RADIUS {
            polygon(color::WHITE,
                    &self.shape,
                    context.transform
                        .trans(self.pos.x + self.window_size.width as f64, self.pos.y)
                        .rot_rad(self.rot),
                    graphics)
        }
        if self.pos.y + RADIUS > self.window_size.height as f64 {
            polygon(color::WHITE,
                    &self.shape,
                    context.transform
                        .trans(self.pos.x, self.pos.y - self.window_size.height as f64)
                        .rot_rad(self.rot),
                    graphics)

        } else if self.pos.y < RADIUS {
            polygon(color::WHITE,
                    &self.shape,
                    context.transform
                        .trans(self.pos.x, self.pos.y + self.window_size.height as f64)
                        .rot_rad(self.rot),
                    graphics)

        }

    }
}

impl Positioned for Asteroid {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for Asteroid {
    fn radius(&self) -> f64 {
        RADIUS
    }
}

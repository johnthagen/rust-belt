//! Defines the asteroid component.

use std::{cmp, f64};

use opengl_graphics::GlGraphics;
use piston_window::{Context, polygon, Size, Transformed, UpdateArgs};
use rand;

use super::super::color;
use super::{Collidable, Drawable, PI_MULT_2, Positioned, Updateable, Vector};

const NUM_SEGMENTS: usize = 20;
const RADIUS_MIN: f64 = 15.0;
const RADIUS_MAX: f64 = 70.0;
const MAX_MUT_FACTOR: f64 = 4.0;

type CircularPolygon = [[f64; 2]; NUM_SEGMENTS];

pub struct Asteroid {
    pos: Vector,
    vel: Vector,
    rot: f64,
    spin: f64,
    radius: f64,
    shape: CircularPolygon,
    window_size: Size,
    on_screen: bool,
}

fn generate_circle(radius: f64) -> CircularPolygon {
    const ANGULAR_SEGMENT: f64 = PI_MULT_2 / NUM_SEGMENTS as f64;
    let mut circle = [[0.0; 2]; NUM_SEGMENTS];
    for (index, mut vertex) in circle.iter_mut().enumerate() {
        let index_float = index as f64;
        vertex[0] = radius * (index_float * ANGULAR_SEGMENT).cos();
        vertex[1] = radius * (index_float * ANGULAR_SEGMENT).sin();
    }
    circle
}

fn randomize_shape(mut shape: CircularPolygon, max: f64) -> CircularPolygon {
    let mut average : Vector = Default::default();
    for mut vertex in &mut shape {
        let rand_vect = Vector::new_rand(0.0,0.0,max,max);
        vertex[0] += rand_vect.x;
        vertex[1] += rand_vect.y;
        average += vertex.clone().into();
    }
    average /= NUM_SEGMENTS as f64;
    for mut vertex in &mut shape {
        vertex[0] -= average.x;
        vertex[1] -= average.y;
    }
    shape
}

fn generate_jagged_shape(radius: f64) -> CircularPolygon {
    let new_shape = generate_circle(radius);
    let max_mut = radius / MAX_MUT_FACTOR;
    randomize_shape(new_shape, max_mut)
}

impl Asteroid {
    pub fn new(window_size: Size) -> Self {
        let asteroid_radius = RADIUS_MIN + rand::random::<f64>() * (RADIUS_MAX - RADIUS_MIN);
        let spawn_radius = cmp::max(window_size.width, window_size.height) as f64 + RADIUS_MAX;
        let angle = PI_MULT_2 * rand::random::<f64>();
        let target = Vector::new_rand(RADIUS_MAX, RADIUS_MAX,
                                      window_size.width as f64 - RADIUS_MAX,
                                      window_size.height as f64 - RADIUS_MAX);
        let vel_multiplier = 0.5 + rand::random::<f64>() * 0.7;
        let new_pos = Vector {
            x: window_size.width as f64 / 2.0 + spawn_radius * angle.cos(),
            y: window_size.height as f64 / 2.0 + spawn_radius * angle.sin(),
        };
        Asteroid {
            pos: new_pos,
            vel: Vector {
                x: new_pos.angle_to_vector(target).cos() * vel_multiplier,
                y: new_pos.angle_to_vector(target).sin() * vel_multiplier,
            },
            rot: 0.0,
            spin: (rand::random::<f64>() - 0.5) * f64::consts::PI / 180.0,
            radius: asteroid_radius,
            shape: generate_jagged_shape(asteroid_radius),
            window_size: window_size,
            on_screen: false,
        }
    }
}

impl Updateable for Asteroid {
    #[allow(unused_variables)]
    fn update(&mut self, args: UpdateArgs) {
        if self.on_screen {
            self.pos += self.vel + self.window_size.into();
            self.pos %= self.window_size.into();
        } else {
            self.pos += self.vel;
        }
        self.rot += self.spin;
        if !self.on_screen && self.pos.x > self.radius &&
           self.pos.x + self.radius < self.window_size.width as f64 &&
           self.pos.y > self.radius &&
           self.pos.y + self.radius < self.window_size.height as f64 {
            self.on_screen = true;
        }
    }
}

impl Drawable for Asteroid {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
        polygon(color::WHITE,
                &self.shape,
                context.transform.trans(self.pos.x, self.pos.y).rot_rad(self.rot),
                graphics);
        if self.on_screen {
            if self.pos.x + self.radius > self.window_size.width as f64 {
                polygon(color::WHITE,
                        &self.shape,
                        context.transform
                            .trans(self.pos.x - self.window_size.width as f64, self.pos.y)
                            .rot_rad(self.rot),
                        graphics)

            } else if self.pos.x < self.radius {
                polygon(color::WHITE,
                        &self.shape,
                        context.transform
                            .trans(self.pos.x + self.window_size.width as f64, self.pos.y)
                            .rot_rad(self.rot),
                        graphics)
            }
            if self.pos.y + self.radius > self.window_size.height as f64 {
                polygon(color::WHITE,
                        &self.shape,
                        context.transform
                            .trans(self.pos.x, self.pos.y - self.window_size.height as f64)
                            .rot_rad(self.rot),
                        graphics)

            } else if self.pos.y < self.radius {
                polygon(color::WHITE,
                        &self.shape,
                        context.transform
                            .trans(self.pos.x, self.pos.y + self.window_size.height as f64)
                            .rot_rad(self.rot),
                        graphics)

            }
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
        self.radius
    }
}

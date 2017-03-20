//! This module defines the asteroid component.
use std::{cmp, f64};

use opengl_graphics::GlGraphics;
use piston_window::{Context, polygon, Size, Transformed, UpdateArgs};
use rand;

use super::super::color;
use super::{Collidable, Drawable, PI_MULT_2, Positioned, Updateable};
use super::vector::Vector;

/// The number of segments in an asteroid's shape is currently set at compile-time.
const NUM_SEGMENTS: usize = 20;

/// Asteroids have random radii within a defined range.
const RADIUS_MIN: f64 = 15.0;
const RADIUS_MAX: f64 = 70.0;

/// Asteroids' shapes are made by mutating a circle, this is a magic number used to tune that.
const MAX_MUT_FACTOR: f64 = 4.0;

/// `CircularPolygon` is a piece of syntactic sugar to avoid having to use that ugly list type.
type CircularPolygon = [[f64; 2]; NUM_SEGMENTS];

/// Asteroids are shapes that randomly float around the screen.
/// They have several properties:
/// * `pos`: the asteroid's position
/// * `vel`: the asteroid's velocity
/// * `rot`: the asteroid's current rotation
/// * `spin`: the asteroid's angular velocity
/// * `radius`: the average radius of the asteroid, used for collision detection
/// * `shape`: an array representing the the drawn shape of the asteroid
/// * `window_size`: the size of the opengl window, used to wrap position
/// * `on_screen`: a flag storing whether the asteroid is fully on-screen
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

/// This function ingests a radius as a float and generates a parametric circle of
/// the given radius. It does this by calculating the angular increment needed to
/// achieve the given number of segments, and then uses trig functions to create
/// a unit circle which it then scales
fn generate_circle(radius: f64) -> CircularPolygon {
    let angular_segment = PI_MULT_2 / NUM_SEGMENTS as f64;
    let mut circle = [[0.0; 2]; NUM_SEGMENTS];
    for (index, mut vertex) in circle.iter_mut().enumerate() {
        let index_float = index as f64;
        vertex[0] = radius * (index_float * angular_segment).cos();
        vertex[1] = radius * (index_float * angular_segment).sin();
    }
    circle
}

/// This function takes in a shape (`CircularPolygon`) and mutates its vertices.
/// There are few steps in this process:
/// * add a (scaled) random amount to each dimension of each vertex
/// * calculate the average location of the vertices. As a circle their average
///   was [0,0], but the mutating changes this slightly
/// * subtract the average position of the vertices from each of them. This
///   ensures that the shape is roughly centered around 0. We aren't actually
///   doing a real center-of-mass calculation, but this looks pretty good.
fn randomize_shape(mut shape: CircularPolygon, max: f64) -> CircularPolygon {
    let mut average = Vector::default();
    for mut vertex in &mut shape {

        // Here we create a pair of random values and add them to a vertex.
        let rand_vect = Vector::new_rand(0.0, 0.0, max, max);
        vertex[0] += rand_vect.x;
        vertex[1] += rand_vect.y;

        // Here, we are adding the new vertex location into what will be our average location.
        average += (*vertex).into();
    }
    // Now we divide the 'average' by the number of segments to convert it from a sum of coordinates
    // into an average of each coordinate. This isn't a real center-of-mass calculation,
    // but it's good enough for this purpose (because we aren't mutating *that* far from a circle)
    average /= NUM_SEGMENTS as f64;
    for mut vertex in &mut shape {
        vertex[0] -= average.x;
        vertex[1] -= average.y;
    }
    shape
}

/// Given a radius, this function returns a `CircularPolygon`
/// containing a jagged 'randomized' circle. This is then
/// used as the drawn shape of the asteroid
fn generate_jagged_shape(radius: f64) -> CircularPolygon {
    let new_shape = generate_circle(radius);

    // Here we are setting a maximum distance to mutate a vertex.
    let max_mut = radius / MAX_MUT_FACTOR;
    randomize_shape(new_shape, max_mut)
}

impl Asteroid {
    pub fn new(window_size: Size) -> Self {

        // First, we generate a random radius, within the specified range, for the new asteroid.
        let asteroid_radius = RADIUS_MIN + rand::random::<f64>() * (RADIUS_MAX - RADIUS_MIN);

        // Asteroids spawn off-screen at a random point along a circle of a set radius,
        // centered at the middle of the screen. Here we are defining that radius.
        let spawn_radius = cmp::max(window_size.width, window_size.height) as f64 + RADIUS_MAX;

        // Here we are generating a random angle, which we will use along with the above radius
        // to calculate the starting point for the new asteroid.
        let angle = PI_MULT_2 * rand::random::<f64>();

        // The asteroid also has an initial velocity. Right here, we are selecting a random point
        // on the screen for the asteroid to float towards. The "RADIUS_MAX" sized gaps at the edges
        // of the range are there to ensure that every asteroid will, for at least one frame, come
        // fully on-screen, so that the on-screen flag is properly flipped
        let target = Vector::new_rand(RADIUS_MAX,
                                      RADIUS_MAX,
                                      window_size.width as f64 - RADIUS_MAX,
                                      window_size.height as f64 - RADIUS_MAX);

        // Now that the asteroid's direction is decided, we decide its speed.
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

            // Spin rate is random within a fixed range.
            spin: (rand::random::<f64>() - 0.5) * f64::consts::PI / 180.0,
            radius: asteroid_radius,
            shape: generate_jagged_shape(asteroid_radius),
            window_size: window_size,

            // All asteroids start off-screen.
            on_screen: false,
        }
    }
}

impl Updateable for Asteroid {
    #[allow(unused_variables)]
    fn update(&mut self, args: UpdateArgs) {

        // If the on-screen flag is true, then the update logic
        // works like every other model. If not, then we don't
        // apply the modulus operation, allowing our asteroid to fly
        // in the same straight line indefinitely. Since we are ensuring
        // that each asteroid gets on-screen at some point, we don't
        // have to worry about 'losing' one forever off-screen.
        if self.on_screen {

            // This verison of the logic uses modulus.
            self.pos += self.vel + self.window_size.into();
            self.pos %= self.window_size.into();
        } else {
            // This is the "floating onto screen" logic which does not use modulus.
            self.pos += self.vel;
        }
        self.rot += self.spin;

        // This code is useful at the beginning of an asteroid's life.
        // It checks whether the asteroid is fully on-screen. If it is,
        // it sets the on_Screen flag and this code isn't touched again.
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

        // This polygon is the "main" asteroid shape within the frame. It is
        // drawn at the location specified in `pos`. The CircularPolygon type,
        // being a list of lists of length 2, is an acceptable "shape" for
        // the polygon function
        polygon(color::WHITE,
                &self.shape,
                context.transform.trans(self.pos.x, self.pos.y).rot_rad(self.rot),
                graphics);

        // Asteroids are large enough that we need to render them on the opposite
        // side of the canvas whenever they start to go off-screen. However, we
        // do not want to do this while the asteroid is still entering the screen.
        // Hence the check for self.on_screen. This code correctly handles wrapped
        // drawing of asteroid drawing along the edges of the frame, but fails in
        // the (literal) corner case, as it will never produce a wrapped asteroid
        // drawing in a corner when an asteroid approaches the opposing corner.
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

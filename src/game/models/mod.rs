//! Traits and types common to models.

use std::f64;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign};

use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs, Size};
use rand;

pub mod player;
pub mod bullet;
pub mod asteroid;

pub const PI_MULT_2: f64 = 2.0 * PI;

/// Models an (x, y) coordinate value (such as position or velocity).
#[derive(Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Vector {
    fn new_rand(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Vector {
        Vector {
            x: rand::random::<f64>() * (x_max - x_min) + x_min,
            y: rand::random::<f64>() * (y_max - y_min) + y_min,
        }
    }
    fn angle_to_vector(self, other: Vector) -> f64 {
        let diff = other - self;
        let mut angle_to_point = (diff.y / diff.x).atan();
        if diff.y < 0.0 {
            angle_to_point += PI;
            if diff.x > 0.0 {
                angle_to_point += PI;
            }
        } else if diff.x < 0.0 {
            angle_to_point += PI;
        }
        angle_to_point
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Rem for Vector {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Vector {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl RemAssign for Vector {
    fn rem_assign(&mut self, other: Self) {
        *self = *self % other;
    }
}

/// Define how a two dimensional `Size` can be converted to a two dimensional `Vector`.
/// Width is defined as the x unit and height is defined as the y unit.
impl From<Size> for Vector {
    fn from(size: Size) -> Self {
        Vector {
            x: size.width as f64,
            y: size.height as f64,
        }
    }
}

/// Trait implemented by types that can be drawn to a window.
pub trait Drawable {
    /// Draws the entity to the screen.
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}

/// Trait implemented by types that are updated by a game loop.
pub trait Updateable {
    /// Update the state of a type within a game loop.
    fn update(&mut self, args: UpdateArgs);
}

pub trait Positioned {
    fn x(&self) -> f64 {
        self.pos().x
    }

    fn y(&self) -> f64 {
        self.pos().y
    }

    fn pos(&self) -> Vector;
}

pub trait Collidable: Positioned {
    fn radius(&self) -> f64;

    fn collides_with<C: Collidable>(&self, other: &C) -> bool {
        // The Distance Formula.
        let distance = ((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).sqrt();
        distance < self.radius() + other.radius()
    }
}

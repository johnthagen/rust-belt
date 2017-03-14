//! Traits and types common to models.

use std::f64;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign, Div, DivAssign};

use opengl_graphics::GlGraphics;
use piston_window::{Context, UpdateArgs, Size};
use rand;

pub mod player;
pub mod bullet;
pub mod asteroid;

pub const PI_MULT_2: f64 = 2.0 * PI;

/// Models an (x, y) coordinate value (such as position or velocity).
#[derive(Copy, Clone, Default)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn new_rand(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
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

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

impl Div<Vector> for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut new_x = self.x;
        let mut new_y = self.y;
        if other.x == 0.0 || other.y == 0.0 {
            new_x = 0.0;
            new_y = 0.0;
        } else {
            new_x /= other.x;
            new_y /= other.y;
        }
        Vector {
            x: new_x,
            y: new_y,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self /
        Vector {
            x: other,
            y: other,
        }
    }
}

impl DivAssign<Vector> for Vector {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        *self = *self / other;
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

impl From<[f64; 2]> for Vector {
    fn from(list: [f64; 2]) -> Self {
        Vector {
            x: list[0],
            y: list[1],
        }
    }
}

/// Trait implemented by types that can be drawn to a window.
pub trait Drawable {
    /// Draws oneself to the screen.
    fn draw(&self, context: Context, graphics: &mut GlGraphics);
}

/// Trait implemented by types that are updated by a game loop.
pub trait Updateable {
    /// Update the state of a type within a game loop.
    fn update(&mut self, args: UpdateArgs);
}

/// Defines how a type can expose its position.
pub trait Positioned {
    fn x(&self) -> f64 {
        self.pos().x
    }

    fn y(&self) -> f64 {
        self.pos().y
    }

    fn pos(&self) -> Vector;
}

/// Defines how types can expose how they can check for collisions with each other.
pub trait Collidable: Positioned {
    fn radius(&self) -> f64;

    /// Check another `Collidable` type to see if it's radius overlaps with this instance's.
    fn collides_with<C: Collidable>(&self, other: &C) -> bool {
        // The Distance Formula.
        let distance = ((self.x() - other.x()).powi(2) + (self.y() - other.y()).powi(2)).sqrt();
        distance < self.radius() + other.radius()
    }
}

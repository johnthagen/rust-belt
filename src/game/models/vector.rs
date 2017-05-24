use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign, Div, DivAssign};

use piston::window::Size;
use rand;

/// Models an (x, y) coordinate value (such as position or velocity).
#[derive(Copy, Clone, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new_rand(x_min: f64, y_min: f64, x_max: f64, y_max: f64) -> Self {
        Vector {
            x: rand::random::<f64>() * (x_max - x_min) + x_min,
            y: rand::random::<f64>() * (y_max - y_min) + y_min,
        }
    }
    pub fn angle_to_vector(self, other: Vector) -> f64 {
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
        if other.x == 0.0 || other.y == 0.0 {
            Vector { x: 0.0, y: 0.0 }
        } else {
            Vector {
                x: self.x / other.x,
                y: self.y / other.y,
            }
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self / Vector { x: other, y: other }
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

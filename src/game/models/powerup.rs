//! Defines the powerup component.
//!
//! Powerups, yo

use std::{cmp, f64};

use opengl_graphics::GlGraphics;
use piston_window::{ellipse, Context, Size, Transformed, UpdateArgs};

use game::color;
use game::models::{Collidable, Drawable, PI_MULT_2, Positioned, Updateable};
use game::models::vector::Vector;

use rand;

#[derive(Copy, Clone)]

pub enum PowerUpType {
	None,
    //Turret,
    //Shield,
    FastShoot,
    //Shotgun,
}

const RADIUS: f64 = 20.0;
#[derive(Clone)]
pub struct PowerUp {
    pub pos: Vector,
    pub vel: Vector,
    radius: f64,
    pub powerup_type: PowerUpType,
    window_size: Size,
    on_screen: bool,
}
impl PowerUp {
    // add code here
    pub fn new(window_size: Size) -> Self {
        // Asteroids spawn off-screen at a random point along a circle of a set radius,
        // centered at the middle of the screen. Here we are defining that radius.
        let spawn_radius = f64::from(cmp::max(window_size.width, window_size.height));

        // Here we are generating a random angle, which we will use along with the above radius
        // to calculate the starting point for the new asteroid.
        let angle = PI_MULT_2 * rand::random::<f64>();

        // The asteroid also has an initial velocity. Right here, we are selecting a random point
        // on the screen for the asteroid to float towards. The "RADIUS_MAX" sized gaps at the edges
        // of the range are there to ensure that every asteroid will, for at least one frame, come
        // fully on-screen, so that the on-screen flag is properly flipped
        let target = Vector::new_rand(
            RADIUS,
            RADIUS,
            f64::from(window_size.width) - RADIUS,
            f64::from(window_size.height) - RADIUS,
        );

        // Now that the asteroid's direction is decided, we decide its speed.
        let vel_multiplier = 0.5 + rand::random::<f64>() * 0.7;
        let new_pos = Vector {
            x: f64::from(window_size.width) / 2.0 + spawn_radius * angle.cos(),
            y: f64::from(window_size.height) / 2.0 + spawn_radius * angle.sin(),
        };
        PowerUp {
            pos: new_pos,
            vel: Vector {
                x: new_pos.angle_to_vector(target).cos() * vel_multiplier,
                y: new_pos.angle_to_vector(target).sin() * vel_multiplier,
            },
            // Spin rate is random within a fixed range.
            powerup_type: PowerUpType::FastShoot,
            radius: RADIUS,
            window_size: window_size,

            // All asteroids start off-screen.
            on_screen: false,
        }
    }
}

impl Updateable for PowerUp {
    fn update(&mut self, _: UpdateArgs) {
        self.pos += self.vel + self.window_size.into();
        self.pos %= self.window_size.into();
    }
}

impl Drawable for PowerUp {
    fn draw(&self, context: Context, graphics: &mut GlGraphics) {
    	ellipse(color::WHITE, ellipse::circle(0.0, 0.0, 4.0), 
    		            context.transform
                    .trans(self.pos.x, self.pos.y)
                    // Without this trans(), rotation occurs around the
                    // upper left corner rather than the center.
                    , graphics)
    }
}
impl Positioned for PowerUp {
    fn pos(&self) -> Vector {
        self.pos
    }
}

impl Collidable for PowerUp {
    fn radius(&self) -> f64 {
        self.radius
    }
}

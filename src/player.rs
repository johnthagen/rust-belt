//! Defines the player component.

pub struct Vect {
    x: f64,
    y: f64,
}

pub struct Player {
    pos: Vect,
    max_pos: Vect,
    vel: Vect,
    rot: f64,
}

const ROTATION_INCREMENT: f64 = 0.1;
const THRUST_INCREMENT: f64 = 0.1;

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vect { x: 10.0, y: 10.0 },
            max_pos: Vect {
                x: 1000.0,
                y: 1000.0,
            },
            vel: Vect { x: 0.0, y: 0.0 },
            rot: 0.0,
        }
    }

    pub fn set_window_size(&mut self, width: f64, height: f64) {
        self.max_pos.x = width;
        self.max_pos.y = height;
    }

    pub fn get_position(&self) -> (f64, f64) {
        (self.pos.x, self.pos.y)
    }

    pub fn get_rotation(&self) -> f64 {
        self.rot
    }

    pub fn update(&mut self) {
        let x = self.pos.x + self.vel.x + self.max_pos.x;
        let y = self.pos.y + self.vel.y + self.max_pos.y;
        self.pos.x = x % self.max_pos.x;
        self.pos.y = y % self.max_pos.y;

    }

    fn rotate(&mut self, rot: f64) {
        self.rot += rot;
    }

    pub fn rotate_cw(&mut self) {
        self.rotate(ROTATION_INCREMENT)
    }

    pub fn rotate_ccw(&mut self) {
        self.rotate(-1.0 * ROTATION_INCREMENT)
    }

    pub fn fire_boosters(&mut self) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT;
        let boost_y = self.rot.sin() * THRUST_INCREMENT;
        self.vel.x += boost_x;
        self.vel.y += boost_y;
    }

    pub fn fire_rev_boosters(&mut self) {
        let boost_x = self.rot.cos() * THRUST_INCREMENT;
        let boost_y = self.rot.sin() * THRUST_INCREMENT;
        self.vel.x -= boost_x;
        self.vel.y -= boost_y;
    }
}

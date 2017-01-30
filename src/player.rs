//! Defines the player component.

pub struct Vect {
    x: f64,
    y: f64,
}

pub struct Player {
    pos: Vect,
    maxPos: Vect,
    vel: Vect,
    rot: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pos: Vect { x: 10.0, y: 10.0 },
            maxPos: Vect {
                x: 1000.0,
                y: 1000.0,
            },
            vel: Vect { x: 0.0, y: 0.0 },
            rot: 0.0,
        }
    }

    pub fn setWindowSize(&mut self, width: f64, height: f64) {
        self.maxPos.x = width;
        self.maxPos.y = height;
    }
    pub fn getPosition(&self) -> (f64, f64) {
        (self.pos.x, self.pos.y)
    }

    pub fn getRotation(&self) -> f64 {
        self.rot
    }
    pub fn update(&mut self) {
        let x = self.pos.x + self.vel.x + self.maxPos.x;
        let y = self.pos.y + self.vel.y + self.maxPos.y;
        self.pos.x = x % self.maxPos.x;
        self.pos.y = y % self.maxPos.y;

    }

    fn rotate(&mut self, rot: f64) {
        self.rot += rot;
    }

    pub fn rotate_cw(&mut self) {
        self.rotate(0.1)
    }

    pub fn rotate_ccw(&mut self) {
        self.rotate(-0.1)
    }

    pub fn fire_boosters(&mut self) {
        let boost_scale = 0.1;
        let boost_x = self.rot.cos() * boost_scale;
        let boost_y = self.rot.sin() * boost_scale;
        self.vel.x += boost_x;
        self.vel.y += boost_y;
    }

    pub fn fire_rev_boosters(&mut self) {
        let boost_scale = 0.1;
        let boost_x = self.rot.cos() * boost_scale;
        let boost_y = self.rot.sin() * boost_scale;
        self.vel.x -= boost_x;
        self.vel.y -= boost_y;
    }
}

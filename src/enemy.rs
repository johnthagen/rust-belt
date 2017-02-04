//! Defines the player component.
use std::f64;
use actor::Actor;

pub struct Enemy {
    actor: Actor,
}

use std::f64::consts::PI;
const PI_TIMES_2: f64 = PI * 2.0;
const PI_OVER_16: f64 = PI / 16.0;
impl Enemy {
    pub fn new(pos_x: f64,
               pos_y: f64,
               width: f64,
               height: f64,
               vel_x: f64,
               vel_y: f64,
               rot: f64)
               -> Enemy {
        Enemy { actor: Actor::new(pos_x, pos_y, width, height, vel_x, vel_y, rot) }
    }

    pub fn set_window_size(&mut self, width: f64, height: f64) {
        self.actor.set_window_size(width, height)
    }

    pub fn pos(&self) -> (f64, f64) {
        self.actor.pos()
    }

    pub fn rot(&self) -> f64 {
        self.actor.rot()
    }

    pub fn update(&mut self, delta: f64, player_x: f64, player_y: f64) {
        //TODO: insert enemy logic
        //let distance = (diff_x.powi(2) + diff_y.powi(2)).sqrt();

        let angle_to_player = self.actor.get_angle_to_point(player_x, player_y);

        let angle_diff = (angle_to_player - self.actor.rot() + PI_TIMES_2) % PI_TIMES_2;

        if (angle_diff > -PI && angle_diff < -PI_OVER_16) ||
           (angle_diff < PI_TIMES_2 && angle_diff > PI) {
            self.rotate_ccw(delta);
        } else if angle_diff < PI && angle_diff > PI_OVER_16{
            self.rotate_cw(delta);
        } else {

            self.fire_boosters(delta / 8.0);
        }

        //self.rotate_cw(delta);

        self.actor.update()
    }

    fn rotate_cw(&mut self, delta: f64) {
        self.actor.rotate_cw(delta)
    }

    fn rotate_ccw(&mut self, delta: f64) {
        self.actor.rotate_ccw(delta)
    }

    fn fire_boosters(&mut self, delta: f64) {
        self.actor.fire_boosters(delta)
    }

    fn fire_rev_boosters(&mut self, delta: f64) {
        self.actor.fire_rev_boosters(delta)
    }
}

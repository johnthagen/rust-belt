//! Defines the player component.
use std::f64;
use actor::Actor;

pub struct Player {
    actor: Actor,
}

impl Player {
    pub fn new() -> Player {
        Player { actor: Actor::new(10.0, 10.0, 1000.0, 1000.0, 0.0, 0.0, 0.0) }
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

    pub fn update(&mut self) {
        self.actor.update()
    }

    pub fn rotate_cw(&mut self, delta: f64) {
        self.actor.rotate_cw(delta)
    }

    pub fn rotate_ccw(&mut self, delta: f64) {
        self.actor.rotate_ccw(delta)
    }

    pub fn fire_boosters(&mut self, delta: f64) {
        self.actor.fire_boosters(delta)
    }

    pub fn fire_rev_boosters(&mut self, delta: f64) {
        self.actor.fire_rev_boosters(delta)
    }
}

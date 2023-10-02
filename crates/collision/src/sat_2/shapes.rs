// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

pub struct Line {
    pub(crate) origin:    Vec2,
    pub(crate) direction: Vec2,
    pub(crate) distance:  f32,
}

pub struct Circle {
    pub(crate) origin: Vec2,
    pub(crate) radius: f32,
}

pub struct Rect {
    pub(crate) origin: Vec2,
    pub(crate) size:   Vec2,
}

impl Rect {
    
    pub fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            if v.x <= self.origin.x { self.origin.x } else { self.origin.x + self.size.x },
            if v.y <= self.origin.y { self.origin.y } else { self.origin.y + self.size.y },
        )
    }

}

pub struct Capsule {
    pub(crate) origin: Vec2,
    pub(crate) height: f32,
    pub(crate) radius: f32,
}

impl Capsule {

    pub fn get_top(&self) -> Vec2 {
        self.origin + Vec2::new(0.0, self.height)
    }

    
    pub fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.origin.x,
            if v.y <= self.origin.y { self.origin.y } else { self.origin.y + self.height },
        )
    }
}

pub struct Slope {
    pub(crate) origin:    Vec2,
    pub(crate) direction: Vec2,
    pub(crate) distance:  f32,
}

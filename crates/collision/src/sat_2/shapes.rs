// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;


pub trait NearestPoint {
    fn nearest_point_to(&self, v: Vec2) -> Vec2;
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub(crate) start:  Vec2,
    pub(crate) end:    Vec2,
    pub(crate) normal: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
}

impl NearestPoint for Circle {
    fn nearest_point_to(&self, _v: Vec2) -> Vec2 {
        self.origin
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub start: Vec2,
    pub end:   Vec2,
}

impl NearestPoint for Rect {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            if v.x <= self.start.x { self.start.x } else { self.end.x },
            if v.y <= self.start.y { self.start.y } else { self.end.y },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Capsule {
    pub start:  Vec2,
    pub height: f32,
    pub radius: f32,
}
    
impl Capsule {
    pub fn end(&self) -> Vec2 {
        Vec2::new(self.start.x, self.start.y + self.height)
    }
}

impl NearestPoint for Capsule {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.start.x,
            if v.y <= self.start.y { self.start.y } else { self.start.y + self.height },
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Slope {
    pub(crate) origin: Vec2,
    pub(crate) rise:   f32,
    pub(crate) run:    f32,
    pub(crate) normal: Vec2,
}

impl Slope {

    pub fn point_run(&self) -> Vec2 {
        Vec2::new(self.origin.x + self.run, self.origin.y)
    }

    pub fn point_rise(&self) -> Vec2 {
        Vec2::new(self.origin.x, self.origin.y + self.rise)
    }

}
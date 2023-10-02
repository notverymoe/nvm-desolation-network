// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

pub trait ShapesCommon {
    fn nearest_point_to(&self, v: Vec2) -> Vec2;
}


pub struct Line {
    pub(crate) origin:    Vec2,
    pub(crate) direction: Vec2,
    pub(crate) distance:  f32,
}

impl Line {
    pub fn run(&self) -> f32 {
        self.direction.x * self.distance
    }

    pub fn rise(&self) -> f32 {
        self.direction.y * self.distance
    }

    pub fn get_end(&self) -> Vec2 {
        self.origin + self.direction*self.distance
    }
}


impl ShapesCommon for Line {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        let x = if v.x < self.origin.x { self.origin.x +  self.run().max(0.0) } else { self.origin.x -  self.run().min(0.0) };
        let y = if v.y < self.origin.y { self.origin.y + self.rise().max(0.0) } else { self.origin.y - self.rise().min(0.0) };
        Vec2::new(x, y)
    }
}

pub struct Circle {
    pub(crate) origin: Vec2,
    pub(crate) radius: f32,
}

impl ShapesCommon for Circle {
    fn nearest_point_to(&self, _v: Vec2) -> Vec2 {
        self.origin
    }
}

pub struct Rect {
    pub(crate) origin: Vec2,
    pub(crate) size:   Vec2,
}

impl ShapesCommon for Rect {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
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
}
    
impl ShapesCommon for Capsule {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
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

impl Slope {
    pub fn run(&self) -> f32 {
        self.direction.x * self.distance
    }

    pub fn rise(&self) -> f32 {
        self.direction.y * self.distance
    }
}

impl ShapesCommon for Slope {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        let x = if v.x < self.origin.x { self.origin.x +  self.run().max(0.0) } else { self.origin.x -  self.run().min(0.0) };
        let y = if v.y < self.origin.y { self.origin.y + self.rise().max(0.0) } else { self.origin.y - self.rise().min(0.0) };
        Vec2::new(x, y)
    }
}

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

pub trait ShapesCommon {
    fn nearest_point_to(&self, v: Vec2) -> Vec2;
    fn get_aabb(&self) -> Rect;
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub(crate) start:  Vec2,
    pub(crate) end:    Vec2,
    pub(crate) normal: Vec2,
}

impl Line {
    pub fn run(&self) -> f32 {
        self.end.x - self.start.x
    }

    pub fn rise(&self) -> f32 {
        self.end.y - self.start.y
    }
}

impl ShapesCommon for Line {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        let x = if v.x < self.start.x { self.start.x +  self.run().max(0.0) } else { self.start.x -  self.run().min(0.0) };
        let y = if v.y < self.start.y { self.start.y + self.rise().max(0.0) } else { self.start.y - self.rise().min(0.0) };
        Vec2::new(x, y)
    }

    fn get_aabb(&self) -> Rect {
        Rect{
            start: self.start.min(self.end),
            end:    self.start.max(self.end),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
}

impl ShapesCommon for Circle {
    fn nearest_point_to(&self, _v: Vec2) -> Vec2 {
        self.origin
    }

    fn get_aabb(&self) -> Rect {
        Rect{
            start: self.origin - Vec2::new(self.radius, self.radius),
            end:   self.origin + Vec2::new(self.radius, self.radius),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub(crate) start: Vec2,
    pub(crate) end:   Vec2,
}

impl ShapesCommon for Rect {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            if v.x <= self.start.x { self.start.x } else { self.end.x },
            if v.y <= self.start.y { self.start.y } else { self.end.y },
        )
    }

    fn get_aabb(&self) -> Rect {
        *self
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
    
impl ShapesCommon for Capsule {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.start.x,
            if v.y <= self.start.y { self.start.y } else { self.start.y + self.height },
        )
    }

    fn get_aabb(&self) -> Rect {
        Rect{
            start: self.start - Vec2::new(self.radius, self.radius),
            end:    self.start + Vec2::new(self.radius, self.radius + self.height),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Slope {
    pub(crate) origin: Vec2,
    pub(crate) end:    Vec2,
    pub(crate) normal: Vec2,
}

impl Slope {
    pub fn run(&self) -> f32 {
        self.end.x - self.origin.x
    }

    pub fn rise(&self) -> f32 {
        self.end.y - self.origin.y
    }
}

impl ShapesCommon for Slope {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        let x = if v.x < self.origin.x { self.origin.x +  self.run().max(0.0) } else { self.origin.x -  self.run().min(0.0) };
        let y = if v.y < self.origin.y { self.origin.y + self.rise().max(0.0) } else { self.origin.y - self.rise().min(0.0) };
        Vec2::new(x, y)
    }

    fn get_aabb(&self) -> Rect {
        Rect{
            start: self.origin.min(self.end),
            end:    self.origin.max(self.end),
        }
    }
}

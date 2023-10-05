// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

use super::{NearestPointTo, Project};

#[derive(Debug, Clone, Copy)]
pub struct Capsule {
    pub start:  Vec2,
    pub height: f32,
    pub radius: f32,
}

impl Capsule {

    pub fn new(start: Vec2, radius: f32, height: f32) -> Self {
        Self{start, radius, height}
    }

}
    
impl Capsule {
    pub fn end(&self) -> Vec2 {
        Vec2::new(self.start.x, self.start.y + self.height)
    }
}

impl NearestPointTo for Capsule {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            self.start.x,
            if v.y <= self.start.y { self.start.y } else { self.start.y + self.height },
        )
    }
}

impl Project for Capsule {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([self.start.x - self.radius, self.start.x + self.radius]),
            Projection([self.start.y - self.radius, self.start.y + self.radius + self.height]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        // TODO confirm, I believe that this workds fine,
        // since this is effectively a swept circle, we
        // shouldn't need to explicitly test points along
        // the body for the projection
        Projection([
            axis.dot(self.start) - self.radius, 
            axis.dot(self.end()) + self.radius,
        ])
    }

    fn with_offset(&self, o: Vec2) -> Self {
        Self{start: self.start + o, height: self.height, radius: self.radius}
    }
}
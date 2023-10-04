// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use super::NearestPoint;

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

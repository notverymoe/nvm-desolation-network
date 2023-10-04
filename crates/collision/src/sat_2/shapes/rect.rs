// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use super::NearestPoint;

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
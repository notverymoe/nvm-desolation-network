// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

use super::{NearestPointTo, Project};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl NearestPointTo for Rect {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            if v.x <= self.min.x { self.min.x } else { self.max.x },
            if v.y <= self.min.y { self.min.y } else { self.max.y },
        )
    }
}

impl Project for Rect {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([self.min.x, self.max.x]),
            Projection([self.min.y, self.max.y]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points())
    }
}

impl Rect {

    pub fn points(&self) -> [Vec2; 4] {
        [
            Vec2::new(self.min.x, self.min.y),
            Vec2::new(self.max.x, self.min.y),
            Vec2::new(self.max.x, self.max.y),
            Vec2::new(self.min.x, self.max.y),
        ]
    }

}
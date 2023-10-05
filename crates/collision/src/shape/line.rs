// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

use super::Project;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub(crate) start:  Vec2,
    pub(crate) end:    Vec2,
    pub(crate) normal: Vec2,
}

impl Project for Line {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection::new_unsorted(self.start.x, self.end.x),
            Projection::new_unsorted(self.start.y, self.end.y),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, [self.start, self.end])
    }
}
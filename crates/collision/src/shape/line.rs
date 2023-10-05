// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

use super::Project;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start:  Vec2,
    end:    Vec2,
    test_axis: Vec2,
}

impl Line {

    pub fn new(start: Vec2, end: Vec2) -> Self {
        let test_axis = (end - start).normalize().perp();
        Self{start, end, test_axis}
    }

    pub fn new_offset(start: Vec2, offset: Vec2) -> Self {
        Self::new(start, start + offset)
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn end(&self) -> Vec2 {
        self.end
    }

    pub fn test_axis(&self) -> Vec2 {
        self.test_axis
    }

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

    fn with_offset(&self, o: Vec2) -> Self {
        Self{start: self.start + o, end: self.end + o, test_axis: self.test_axis}
    }

}
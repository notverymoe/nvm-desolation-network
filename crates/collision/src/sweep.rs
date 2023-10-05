// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{shape::{Shape, Project}, Projection};

pub struct Sweep {
    start:    Shape,
    end:      Shape,
    motion:   Vec2,
    test_axis: Vec2,
    test_cache:  Projection,
}

impl Sweep {
    
    pub fn new(shape: Shape, motion: Vec2) -> Self {
        let test_dir = motion.normalize().perp();
        Self {
            test_cache: shape.project_on_axis(test_dir),
            test_axis: test_dir,
            motion,
            end:   shape.with_offset(motion),
            start: shape,
        }
    }

    pub fn start(&self) -> &Shape {
        &self.start
    }

    pub fn end(&self) -> &Shape {
        &self.end
    }

    pub fn motion(&self) -> Vec2 {
        self.motion
    }

    pub fn test_axis(&self) -> Vec2 {
        self.test_axis
    }

    pub fn test_cache(&self) -> Projection {
        self.test_cache
    }

}

impl Project for Sweep {
    fn project_aabb(&self) -> [Projection; 2] {
        let [x, y] = self.start.project_aabb();
        [
            x.smeared_by(self.motion.x),
            y.smeared_by(self.motion.y),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        self.start.project_on_axis(axis).smeared_by(axis.dot(self.motion))
    }

    fn with_offset(&self, o: Vec2) -> Self {
        Self{
            start: self.start.with_offset(o),
            end:   self.end.with_offset(o),
            motion: self.motion,
            test_axis: self.test_axis,
            test_cache: self.test_cache.offset_by(self.test_axis.dot(o)),
        }
    }
}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use enum_dispatch::enum_dispatch;
use static_assertions::const_assert_eq;

use crate::Projection;

#[enum_dispatch(Project)]
pub enum Shape {
    Point(Vec2),
    Line(Line),
    Circle(Circle),
    Rect(Rect),
    Capsule(Capsule),
    Slope(Slope),
}

// We don't want shape to grow larger than this on accident, edit to confirm size change.
const_assert_eq!(std::mem::size_of::<Shape>(), 28);

pub trait NearestPointTo {
    fn nearest_point_to(&self, v: Vec2) -> Vec2;
}

#[enum_dispatch]
pub trait Project {
    fn project_aabb(&self) -> [Projection; 2];
    fn project_on_axis(&self, axis: Vec2) -> Projection;
    fn with_offset(&self, o: Vec2) -> Self;
}

impl Project for Vec2 {
    fn project_aabb(&self) -> [Projection; 2] {
        [Projection::new(self.x), Projection::new(self.y)]
    }

    fn project_on_axis(&self, axis:Vec2) -> Projection {
        Projection::new(axis.dot(*self))
    }

    fn with_offset(&self, o: Vec2) -> Self {
        *self + o
    }
}

mod line;
pub use line::*;

mod circle;
pub use circle::*;

mod rect;
pub use rect::*;

mod capsule;
pub use capsule::*;

mod slope;
pub use slope::*;
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

impl Shape {

    pub fn offset_by(&self, o: Vec2) -> Self {
        match self {
            Shape::Point(s)   => (*s + o).into(),
            Shape::Line(s)    => Line{start: s.start + o, end: s.end + o, normal: s.normal}.into(),
            Shape::Circle(s)  => Circle{origin: s.origin + o, radius: s.radius}.into(),
            Shape::Rect(s)    => Rect{min: s.min + o, max: s.max + o}.into(),
            Shape::Capsule(s) => Capsule{start: s.start, height: s.height, radius: s.radius}.into(),
            Shape::Slope(s)   => Slope{origin: s.origin() + o, rise: s.rise, run: s.run, normal_scl: s.normal_scl}.into(),
        }
    }

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
}

impl Project for Vec2 {
    fn project_aabb(&self) -> [Projection; 2] {
        [Projection::new(self.x), Projection::new(self.y)]
    }

    fn project_on_axis(&self, axis:Vec2) -> Projection {
        Projection::new(axis.dot(*self))
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
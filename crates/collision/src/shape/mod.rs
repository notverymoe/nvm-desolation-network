// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

pub enum Shape {
    Point(Vec2),
    Line(Line),
    Circle(Circle),
    Rect(Rect),
    Capsule(Capsule),
    Slope(Slope),
}

pub trait NearestPoint {
    fn nearest_point_to(&self, v: Vec2) -> Vec2;
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
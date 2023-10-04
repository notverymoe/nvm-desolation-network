// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

mod shapes;
pub use shapes::*;

mod sweep;
pub use sweep::*;

mod candidate_axes;
pub use candidate_axes::*;

pub enum Shape {
    Point(Vec2),
    Line(Line),
    Circle(Circle),
    Rect(Rect),
    Capsule(Capsule),
    Slope(Slope),
}


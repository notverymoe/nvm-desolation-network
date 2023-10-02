// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

mod shapes;
pub use shapes::*;

mod sweep;
pub use sweep::*;
use tinyvec::{ArrayVec, array_vec};

pub enum Shape {
    Point(Vec2),
    Line(Line),
    Circle(Circle),
    Rect(Rect),
    Capsule(Capsule),
    Slope(Slope),
}

pub fn get_seperating_axis_candidates(a: Shape, b: Shape) -> ArrayVec<[Vec2; 4]> {
    match (a, b) {
        (Shape::Point(a),   Shape::Point(b)) => array_vec!({axis_between(a, b)}),
        (Shape::Point(_),    Shape::Line(b)) => array_vec!(b.direction, b.direction.perp()),
        (Shape::Point(a),  Shape::Circle(b)) => array_vec!({axis_between(a, b.origin)}),
        (Shape::Point(_),    Shape::Rect(_)) => array_vec!(Vec2::X, Vec2::Y),
        (Shape::Point(a), Shape::Capsule(b)) => array_vec!(Vec2::X, axis_between(a, b.origin), axis_between(a, b.get_top())),
        (Shape::Point(_),   Shape::Slope(b)) => array_vec!(Vec2::X, Vec2::Y, b.direction.perp()),

        (Shape::Line(a),    Shape::Line(b)) => array_vec!(),
        (Shape::Line(a),  Shape::Circle(b)) => array_vec!(),
        (Shape::Line(a),    Shape::Rect(b)) => array_vec!(Vec2::X, Vec2::Y, a.direction.perp()), // HACK Is omitting Line::direction fine?
        (Shape::Line(a), Shape::Capsule(b)) => array_vec!(),
        (Shape::Line(a),   Shape::Slope(b)) => array_vec!(),

        (Shape::Circle(a),  Shape::Circle(b)) => array_vec!({axis_between(a.origin, b.origin)}),
        (Shape::Circle(a),    Shape::Rect(b)) => array_vec!(Vec2::X, Vec2::Y, axis_between(a.origin, b.nearest_point_to(a.origin))),
        (Shape::Circle(a), Shape::Capsule(b)) => array_vec!(Vec2::X, b.nearest_point_to(a.origin)),
        (Shape::Circle(a),   Shape::Slope(b)) => array_vec!(),

        (Shape::Rect(_),    Shape::Rect(_)) => array_vec!(Vec2::X, Vec2::Y),
        (Shape::Rect(a), Shape::Capsule(b)) => array_vec!(Vec2::X, Vec2::Y, axis_between(a.nearest_point_to(b.origin), b.origin), axis_between(a.nearest_point_to(b.get_top()), b.get_top())),
        (Shape::Rect(_),   Shape::Slope(b)) => array_vec!(Vec2::X, Vec2::Y, b.direction.perp()),

        (Shape::Capsule(a), Shape::Capsule(b)) => array_vec!(Vec2::X, axis_between(a.nearest_point_to(b.origin), b.origin), axis_between(a.nearest_point_to(b.get_top()), b.get_top())),
        (Shape::Capsule(a),   Shape::Slope(b)) => array_vec!(),

        (Shape::Slope(a), Shape::Slope(b)) => array_vec!(Vec2::X, Vec2::Y, a.direction.perp(), b.direction.perp()),

        (a, b) => get_seperating_axis_candidates(b, a), // TODO inline
    }
}

fn axis_between(a: Vec2, b: Vec2) -> Vec2 {
    (b - a).normalize()
}
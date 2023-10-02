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

pub fn get_seperating_axis_candidates(a: Shape, b: Shape) -> ArrayVec<[Vec2; 3]> {
    match (a, b) {
        (Shape::Point(_),   Shape::Point(_)) => array_vec!(),
        (Shape::Point(_),    Shape::Line(b)) => array_vec!(b.direction, b.direction.perp()),
        (Shape::Point(a),  Shape::Circle(b)) => array_vec!({axis_between(a, b.origin)}),
        (Shape::Point(_),    Shape::Rect(_)) => array_vec!(),
        (Shape::Point(a), Shape::Capsule(b)) => array_vec!({axis_between(a, b.origin)}, axis_between(a, b.get_top())),
        (Shape::Point(_),   Shape::Slope(b)) => array_vec!(b.direction.perp()),

        (Shape::Line(a),    Shape::Line(b)) => array_vec!(a.direction.perp(), b.direction.perp()),
        (Shape::Line(a),  Shape::Circle(b)) => array_vec!({nearest_axis(b.origin, &a)}),
        (Shape::Line(a),    Shape::Rect(_)) => array_vec!(a.direction.perp()),
        (Shape::Line(a), Shape::Capsule(b)) => array_vec!({nearest_axis(a.origin, &b)}, nearest_axis(a.end, &b)),
        (Shape::Line(a),   Shape::Slope(b)) => array_vec!(a.direction.perp(), b.direction.perp()),

        (Shape::Circle(a),  Shape::Circle(b)) => array_vec!({axis_between(a.origin, b.origin)}),
        (Shape::Circle(a),    Shape::Rect(b)) => array_vec!({nearest_axis(a.origin, &b)}),
        (Shape::Circle(a), Shape::Capsule(b)) => array_vec!({nearest_axis(b.origin, &a)}, nearest_axis(b.get_top(), &a)),
        (Shape::Circle(a),   Shape::Slope(b)) => array_vec!({nearest_axis(a.origin, &b)}, b.direction.perp()),

        (Shape::Rect(_),    Shape::Rect(_)) => array_vec!(),
        (Shape::Rect(a), Shape::Capsule(b)) => array_vec!({nearest_axis(b.origin, &a)}, nearest_axis(b.get_top(), &a)),
        (Shape::Rect(_),   Shape::Slope(b)) => array_vec!(b.direction.perp()),

        (Shape::Capsule(a), Shape::Capsule(b)) => array_vec!({nearest_axis(b.origin, &a)}, nearest_axis(b.get_top(), &a)),
        (Shape::Capsule(a),   Shape::Slope(b)) => array_vec!({nearest_axis(a.origin, &b)}, nearest_axis(a.get_top(), &b), b.direction.perp()),

        (Shape::Slope(a), Shape::Slope(b)) => array_vec!(a.direction.perp(), b.direction.perp()),

        (a, b) => get_seperating_axis_candidates(b, a), // TODO inline?
    }
}

fn axis_between(a: Vec2, b: Vec2) -> Vec2 {
    (b - a).normalize()
}

fn nearest_axis(from: Vec2, to: &impl ShapesCommon) -> Vec2 {
    axis_between(from, to.nearest_point_to(from))
}
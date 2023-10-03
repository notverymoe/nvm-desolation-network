// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

mod shapes;
pub use shapes::*;

mod sweep;
pub use sweep::*;

pub enum Shape {
    Point(Vec2),
    Line(Line),
    Circle(Circle),
    Rect(Rect),
    Capsule(Capsule),
    Slope(Slope),
}

pub fn get_seperating_axis_candidates(a: Shape, b: Shape) -> ([Vec2; 2], usize) {
    // TODO OPT nearest_axis in some cases could use AABB information
    
    match (a, b) {
        (  Shape::Point(_),   Shape::Point(_)) => AXES_0,
        (   Shape::Line(a),    Shape::Line(b)) => axes_2(a.normal, b.normal),
        ( Shape::Circle(a),  Shape::Circle(b)) => axes_1(axis_between(a.origin, b.origin)),
        (   Shape::Rect(_),    Shape::Rect(_)) => AXES_0,
        (Shape::Capsule(a), Shape::Capsule(b)) => axes_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
        (  Shape::Slope(a),   Shape::Slope(b)) => axes_2(a.normal, b.normal),

        (Shape::Point(_),    Shape::Line(b)) | (   Shape::Line(b), Shape::Point(_)) => axes_1(b.normal),
        (Shape::Point(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Point(a)) => axes_1(axis_between(a, b.origin)),
        (Shape::Point(_),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Point(_)) => AXES_0,
        (Shape::Point(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Point(a)) => axes_2(axis_between(a, b.start), axis_between(a, b.end())),
        (Shape::Point(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Point(_)) => axes_1(b.normal),

        (Shape::Line(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Line(a)) => axes_2(axis_between(a.start, b.origin), axis_between(a.end(), b.origin)),
        (Shape::Line(a),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Line(a)) => axes_1(a.normal),
        (Shape::Line(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Line(a)) => axes_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
        (Shape::Line(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Line(a)) => axes_2(a.normal, b.normal),

        (Shape::Circle(a),    Shape::Rect(b)) | (   Shape::Rect(b), Shape::Circle(a)) => axes_1(nearest_axis(a.origin, &b)),
        (Shape::Circle(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Circle(a)) => axes_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
        (Shape::Circle(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Circle(a)) => axes_2(nearest_axis(a.origin, &b), b.normal),

        (Shape::Rect(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Rect(a)) => axes_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
        (Shape::Rect(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Rect(_)) => axes_1(b.normal),

        (Shape::Capsule(a),   Shape::Slope(b)) | (Shape::Slope(b), Shape::Capsule(a)) => {
            // if nearest is above the base of the cylinder, then we can exclude it
            // since the cylinder hemisphere could only collide with points bellow it
            // we'll include the nearest to the top without checking in that case
            // because we either need it, or it's not going to cause us harm to test
            let nearest = b.nearest_point_to(a.start);
            if nearest.y > a.start.y {
                axes_2(b.normal, axis_between(a.start, nearest))
            } else {
                axes_2(b.normal, nearest_axis(a.end(), &b))
            }
        }
    }
}

const AXES_0: ([Vec2; 2], usize) = ([Vec2::ZERO, Vec2::ZERO], 0);

fn axes_1(a: Vec2) -> ([Vec2; 2], usize) {
    ([a, Vec2::ZERO], 1)
}

fn axes_2(a: Vec2, b: Vec2) -> ([Vec2; 2], usize) {
    ([a, b], 2)
}

fn axis_between(a: Vec2, b: Vec2) -> Vec2 {
    (b - a).normalize()
}

fn nearest_axis(from: Vec2, to: &impl NearestPoint) -> Vec2 {
    axis_between(from, to.nearest_point_to(from))
}
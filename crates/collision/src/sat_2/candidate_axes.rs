// Copyright 2023 Natalie Baker // AGPLv3 //

use std::{slice::Iter, iter::Take};

use bevy::prelude::Vec2;

use super::{Shape, NearestPoint};

pub struct CandidateAxes(([Vec2; 7], usize));


impl CandidateAxes {

    pub fn as_slice(&self) -> &[Vec2] {
        &self.0.0[0..self.0.1]
    }

    pub fn iter(&self) -> Iter<'_, Vec2> {
        self.as_slice().iter()
    }

}

impl IntoIterator for CandidateAxes {
    type Item = Vec2;
    type IntoIter = Take<std::array::IntoIter<bevy::prelude::Vec2, 7>>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.0.into_iter().take(self.0.1)
    }
}



impl CandidateAxes {
    pub const NONE: CandidateAxes = CandidateAxes(([Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 0));

    pub fn new_1(a: Vec2) -> CandidateAxes {
        CandidateAxes(([a, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 1))
    }

    pub fn new_2(a: Vec2, b: Vec2) -> CandidateAxes {
        CandidateAxes(([a, b, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 2))
    }

    pub fn new_3(a: Vec2, b: Vec2, c: Vec2) -> CandidateAxes {
        CandidateAxes(([a, b, c, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 3))
    }

    pub fn new_4(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> CandidateAxes {
        CandidateAxes(([a, b, c, d, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 4))
    }

    pub fn new_5(a: Vec2, b: Vec2, c: Vec2, d: Vec2, e: Vec2) -> CandidateAxes {
        CandidateAxes(([a, b, c, d, e, Vec2::ZERO, Vec2::ZERO], 5))
    }

    pub fn new_6(a: Vec2, b: Vec2, c: Vec2, d: Vec2, e: Vec2, f: Vec2) -> CandidateAxes {
        CandidateAxes(([a, b, c, d, e, f, Vec2::ZERO], 6))
    }

    pub fn new_7(a: Vec2, b: Vec2, c: Vec2, d: Vec2, e: Vec2, f: Vec2, g: Vec2) -> CandidateAxes {
        CandidateAxes(([a, b, c, d, e, f, g], 7))
    }
}

impl CandidateAxes {
    pub fn find_between(a: Shape, b: Shape) -> CandidateAxes {    
        match (a, b) {
            (  Shape::Point(_),   Shape::Point(_)) => CandidateAxes::NONE,
            (   Shape::Line(a),    Shape::Line(b)) => CandidateAxes::new_2(a.normal, b.normal),
            ( Shape::Circle(a),  Shape::Circle(b)) => CandidateAxes::new_1(axis_between(a.origin, b.origin)),
            (   Shape::Rect(_),    Shape::Rect(_)) => CandidateAxes::NONE,
            (Shape::Capsule(a), Shape::Capsule(b)) => CandidateAxes::new_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
            (  Shape::Slope(a),   Shape::Slope(b)) => CandidateAxes::new_2(a.normal, b.normal),
    
            (Shape::Point(_),    Shape::Line(b)) | (   Shape::Line(b), Shape::Point(_)) => CandidateAxes::new_1(b.normal),
            (Shape::Point(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Point(a)) => CandidateAxes::new_1(axis_between(a, b.origin)),
            (Shape::Point(_),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Point(_)) => CandidateAxes::NONE,
            (Shape::Point(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Point(a)) => CandidateAxes::new_2(axis_between(a, b.start), axis_between(a, b.end())),
            (Shape::Point(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Point(_)) => CandidateAxes::new_1(b.normal),
    
            (Shape::Line(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Line(a)) => CandidateAxes::new_2(axis_between(a.start, b.origin), axis_between(a.end, b.origin)),
            (Shape::Line(a),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Line(a)) => CandidateAxes::new_1(a.normal),
            (Shape::Line(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Line(a)) => CandidateAxes::new_5(
                a.normal,
                axis_between(a.start, b.start),
                axis_between(a.start, b.end()),
                axis_between(a.end,   b.start),
                axis_between(a.end,   b.end()),
            ), // OPT
            (Shape::Line(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Line(a)) => CandidateAxes::new_2(a.normal, b.normal),
    
            (Shape::Circle(a),    Shape::Rect(b)) | (   Shape::Rect(b), Shape::Circle(a)) => CandidateAxes::new_1(nearest_axis(a.origin, &b)),
            (Shape::Circle(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Circle(a)) => CandidateAxes::new_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
            (Shape::Circle(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Circle(a)) => CandidateAxes::new_4(
                b.normal,
                axis_between(a.origin, b.origin),
                axis_between(a.origin, b.point_run()),
                axis_between(a.origin, b.point_rise()),
            ), // OPT
    
            (Shape::Rect(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Rect(a)) => CandidateAxes::new_2(nearest_axis(b.start, &a), nearest_axis(b.end(), &a)),
            (Shape::Rect(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Rect(_)) => CandidateAxes::new_1(b.normal),
    
            (Shape::Capsule(a),   Shape::Slope(b)) | (Shape::Slope(b), Shape::Capsule(a)) => CandidateAxes::new_7(
                b.normal,
                axis_between(a.start, b.origin),
                axis_between(a.start, b.point_run()),
                axis_between(a.start, b.point_rise()),
                axis_between(a.end(), b.origin),
                axis_between(a.end(), b.point_run()),
                axis_between(a.end(), b.point_rise()),
            ), // OPT
        }
    }
}


fn axis_between(a: Vec2, b: Vec2) -> Vec2 {
    (b - a).normalize()
}

fn nearest_axis(from: Vec2, to: &impl NearestPoint) -> Vec2 {
    axis_between(from, to.nearest_point_to(from))
}
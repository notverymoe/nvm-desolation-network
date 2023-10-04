// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Deref};
use tinyvec::ArrayVec;

use super::{Shape, NearestPoint};

#[derive(Deref, Default)]
#[repr(transparent)]
pub struct CandidateAxes(ArrayVec<[Vec2; 7]>);

impl CandidateAxes {

    pub const NONE: CandidateAxes = CandidateAxes(ArrayVec::from_array_empty([Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO]));

    pub fn find_between(a: Shape, b: Shape) -> CandidateAxes {    
        match (a, b) {
            (  Shape::Point(_),   Shape::Point(_)) => CandidateAxes::NONE,
            (   Shape::Line(a),    Shape::Line(b)) => [a.normal, b.normal].into(),
            ( Shape::Circle(a),  Shape::Circle(b)) => [axis_between(a.origin, b.origin)].into(),
            (   Shape::Rect(_),    Shape::Rect(_)) => CandidateAxes::NONE,
            (Shape::Capsule(a), Shape::Capsule(b)) => [nearest_axis(b.start, &a), nearest_axis(b.end(), &a)].into(),
            (  Shape::Slope(a),   Shape::Slope(b)) => [a.normal(), b.normal()].into(),
    
            (Shape::Point(_),    Shape::Line(b)) | (   Shape::Line(b), Shape::Point(_)) => [b.normal].into(),
            (Shape::Point(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Point(a)) => [axis_between(a, b.origin)].into(),
            (Shape::Point(_),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Point(_)) => CandidateAxes::NONE,
            (Shape::Point(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Point(a)) => [axis_between(a, b.start), axis_between(a, b.end())].into(),
            (Shape::Point(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Point(_)) => [b.normal()].into(),
    
            (Shape::Line(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Line(a)) => [axis_between(a.start, b.origin), axis_between(a.end, b.origin)].into(),
            (Shape::Line(a),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Line(a)) => [a.normal].into(),
            (Shape::Line(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Line(a)) => [
                a.normal,
                axis_between(a.start, b.start),
                axis_between(a.start, b.end()),
                axis_between(a.end,   b.start),
                axis_between(a.end,   b.end()),
            ].into(), // OPT
            (Shape::Line(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Line(a)) => [a.normal, b.normal()].into(),
    
            (Shape::Circle(a),    Shape::Rect(b)) | (   Shape::Rect(b), Shape::Circle(a)) => [nearest_axis(a.origin, &b)].into(),
            (Shape::Circle(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Circle(a)) => [axis_between(b.start, a.origin), axis_between(b.start, a.origin)].into(),
            (Shape::Circle(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Circle(a)) => [
                b.normal(),
                axis_between(a.origin, b.origin),
                axis_between(a.origin, b.point_run()),
                axis_between(a.origin, b.point_rise()),
            ].into(), // OPT
    
            (Shape::Rect(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Rect(a)) => [nearest_axis(b.start, &a), nearest_axis(b.end(), &a)].into(),
            (Shape::Rect(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Rect(_)) => [b.normal()].into(),
    
            (Shape::Capsule(a),   Shape::Slope(b)) | (Shape::Slope(b), Shape::Capsule(a)) => [
                b.normal(),
                axis_between(a.start, b.origin),
                axis_between(a.start, b.point_run()),
                axis_between(a.start, b.point_rise()),
                axis_between(a.end(), b.origin),
                axis_between(a.end(), b.point_run()),
                axis_between(a.end(), b.point_rise()),
            ].into(), // OPT
        }
    }
}


fn axis_between(a: Vec2, b: Vec2) -> Vec2 {
    (b - a).normalize()
}

fn nearest_axis(from: Vec2, to: &impl NearestPoint) -> Vec2 {
    axis_between(from, to.nearest_point_to(from))
}

impl From<[Vec2; 1]> for CandidateAxes {
    fn from(value: [Vec2; 1]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 1))
    }
}

impl From<[Vec2; 2]> for CandidateAxes {
    fn from(value: [Vec2; 2]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], value[1], Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 2))
    }
}

impl From<[Vec2; 3]> for CandidateAxes {
    fn from(value: [Vec2; 3]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], value[1], value[2], Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 3))
    }
}

impl From<[Vec2; 4]> for CandidateAxes {
    fn from(value: [Vec2; 4]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], value[1], value[2], value[3], Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 4))
    }
}

impl From<[Vec2; 5]> for CandidateAxes {
    fn from(value: [Vec2; 5]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], value[1], value[2], value[3], value[4], Vec2::ZERO, Vec2::ZERO], 5))
    }
}

impl From<[Vec2; 6]> for CandidateAxes {
    fn from(value: [Vec2; 6]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], value[1], value[2], value[3], value[4], value[5], Vec2::ZERO], 6))
    }
}

impl From<[Vec2; 7]> for CandidateAxes {
    fn from(value: [Vec2; 7]) -> Self {
        CandidateAxes(ArrayVec::from_array_len([value[0], value[1], value[2], value[3], value[4], value[5], value[6]], 7))
    }
}
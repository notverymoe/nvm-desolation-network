// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

use crate::shape::{Shape, NearestPointTo};

pub const CANDIDATE_AXES_SIZE: usize = 7;

pub type CandidateAxes = ArrayVec<[Vec2; CANDIDATE_AXES_SIZE]>;

macro_rules! axes {
    () => { ArrayVec::from_array_empty([Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO]) };
    ($a:expr) => { ArrayVec::from_array_len([$a, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 1) };
    ($a:expr,$b:expr) => { ArrayVec::from_array_len([$a, $b, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 2) };
    ($a:expr,$b:expr,$c:expr) => { ArrayVec::from_array_len([$a, $b, $c, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 3) };
    ($a:expr,$b:expr,$c:expr,$d:expr) => { ArrayVec::from_array_len([$a, $b, $c, $d, Vec2::ZERO, Vec2::ZERO, Vec2::ZERO], 4) };
    ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr) => { ArrayVec::from_array_len([$a, $b, $c, $d, $e, Vec2::ZERO, Vec2::ZERO], 5) };
    ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr) => { ArrayVec::from_array_len([$a, $b, $c, $d, $e, $f, Vec2::ZERO], 6) };
    ($a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr,$g:expr) => { ArrayVec::from([$a, $b, $c, $d, $e, $f, $g])};
}

pub fn find_candidates_between(a: &Shape, b: &Shape, dest: &mut CandidateAxes) {    
    *dest = match (a, b) {
        (  Shape::Point(_),   Shape::Point(_)) => axes!(),
        (   Shape::Line(a),    Shape::Line(b)) => axes!(a.normal, b.normal),
        ( Shape::Circle(a),  Shape::Circle(b)) => axes!(axis_between(a.origin, b.origin)),
        (   Shape::Rect(_),    Shape::Rect(_)) => axes!(),
        (Shape::Capsule(a), Shape::Capsule(b)) => axes!(nearest_axis(b.start, a), nearest_axis(b.end(), a)),
        (  Shape::Slope(a),   Shape::Slope(b)) => axes!(a.normal(), b.normal()),

        (Shape::Point(_),    Shape::Line(b)) | (   Shape::Line(b), Shape::Point(_)) => axes!(b.normal),
        (Shape::Point(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Point(a)) => axes!(axis_between(*a, b.origin)),
        (Shape::Point(_),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Point(_)) => axes!(),
        (Shape::Point(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Point(a)) => axes!(axis_between(*a, b.start), axis_between(*a, b.end())),
        (Shape::Point(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Point(_)) => axes!(b.normal()),

        (Shape::Line(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Line(a)) => axes!(axis_between(a.start, b.origin), axis_between(a.end, b.origin)),
        (Shape::Line(a),    Shape::Rect(_)) | (   Shape::Rect(_), Shape::Line(a)) => axes!(a.normal),
        (Shape::Line(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Line(a)) => axes!(
            a.normal,
            axis_between(a.start, b.start),
            axis_between(a.start, b.end()),
            axis_between(a.end,   b.start),
            axis_between(a.end,   b.end())
        ).into(), // OPT
        (Shape::Line(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Line(a)) => axes!(a.normal, b.normal()),

        (Shape::Circle(a),    Shape::Rect(b)) | (   Shape::Rect(b), Shape::Circle(a)) => axes!(nearest_axis(a.origin, b)),
        (Shape::Circle(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Circle(a)) => axes!(axis_between(b.start, a.origin), axis_between(b.start, a.origin)),
        (Shape::Circle(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Circle(a)) => axes!(
            b.normal(),
            axis_between(a.origin, b.origin()),
            axis_between(a.origin, b.point_run()),
            axis_between(a.origin, b.point_rise())
        ), // OPT

        (Shape::Rect(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Rect(a)) => axes!(nearest_axis(b.start, a), nearest_axis(b.end(), a)),
        (Shape::Rect(_),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Rect(_)) => axes!(b.normal()),

        (Shape::Capsule(a), Shape::Slope(b)) | (Shape::Slope(b), Shape::Capsule(a)) => axes!(
            b.normal(),
            axis_between(a.start, b.origin()),
            axis_between(a.start, b.point_run()),
            axis_between(a.start, b.point_rise()),
            axis_between(a.end(), b.origin()),
            axis_between(a.end(), b.point_run()),
            axis_between(a.end(), b.point_rise())
        ), // OPT
    }
}

pub fn find_dynamic_candidates(a: &Shape, b: &Shape, dest: &mut CandidateAxes) {
    *dest = match (a, b) {
        (Shape::Point(_), Shape::Point(_)) |
        ( Shape::Line(_),  Shape::Line(_)) |
        ( Shape::Rect(_),  Shape::Rect(_)) |
        (Shape::Slope(_), Shape::Slope(_)) |
        (Shape::Point(_),  Shape::Line(_)) | ( Shape::Line(_), Shape::Point(_)) |
        (Shape::Point(_),  Shape::Rect(_)) | ( Shape::Rect(_), Shape::Point(_)) |
        (Shape::Point(_), Shape::Slope(_)) | (Shape::Slope(_), Shape::Point(_)) |
        ( Shape::Line(_),  Shape::Rect(_)) | ( Shape::Rect(_),  Shape::Line(_)) |
        ( Shape::Line(_), Shape::Slope(_)) | (Shape::Slope(_),  Shape::Line(_)) |
        ( Shape::Rect(_), Shape::Slope(_)) | (Shape::Slope(_),  Shape::Rect(_)) => Default::default(),

        ( Shape::Circle(a),  Shape::Circle(b)) => axes!(axis_between(a.origin, b.origin)),
        (Shape::Capsule(a), Shape::Capsule(b)) => axes!(nearest_axis(b.start, a), nearest_axis(b.end(), a)),

        (Shape::Point(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Point(a)) => axes!(axis_between(*a, b.origin)),
        (Shape::Point(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Point(a)) => axes!(axis_between(*a, b.start), axis_between(*a, b.end())),

        (Shape::Line(a),  Shape::Circle(b)) | ( Shape::Circle(b), Shape::Line(a)) => axes!(axis_between(a.start, b.origin), axis_between(a.end, b.origin)),
        (Shape::Line(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Line(a)) => axes!(
            axis_between(a.start, b.start),
            axis_between(a.start, b.end()),
            axis_between(a.end,   b.start),
            axis_between(a.end,   b.end())
        ).into(), // OPT

        (Shape::Circle(a),    Shape::Rect(b)) | (   Shape::Rect(b), Shape::Circle(a)) => axes!(nearest_axis(a.origin, b)),
        (Shape::Circle(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Circle(a)) => axes!(axis_between(b.start, a.origin), axis_between(b.start, a.origin)),
        (Shape::Circle(a),   Shape::Slope(b)) | (  Shape::Slope(b), Shape::Circle(a)) => axes!(
            axis_between(a.origin, b.origin()),
            axis_between(a.origin, b.point_run()),
            axis_between(a.origin, b.point_rise())
        ), // OPT

        (Shape::Rect(a), Shape::Capsule(b)) | (Shape::Capsule(b), Shape::Rect(a)) => axes!(nearest_axis(b.start, a), nearest_axis(b.end(), a)),

        (Shape::Capsule(a), Shape::Slope(b)) | (Shape::Slope(b), Shape::Capsule(a)) => axes!(
            axis_between(a.start, b.origin()),
            axis_between(a.start, b.point_run()),
            axis_between(a.start, b.point_rise()),
            axis_between(a.end(), b.origin()),
            axis_between(a.end(), b.point_run()),
            axis_between(a.end(), b.point_rise())
        ), // OPT
    }
}

fn axis_between(a: Vec2, b: Vec2) -> Vec2 {
    (b - a).normalize()
}

fn nearest_axis(from: Vec2, to: &impl NearestPointTo) -> Vec2 {
    axis_between(from, to.nearest_point_to(from))
}


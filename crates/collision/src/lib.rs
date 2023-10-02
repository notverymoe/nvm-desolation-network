// Copyright 2023 Natalie Baker // AGPLv3 //

pub mod shape;

mod sat;
pub use sat::*;

pub mod sat_2;

use shape::Rect;

pub trait HasBoundingBox {
    fn get_bounding_box(&self) -> Rect;
}

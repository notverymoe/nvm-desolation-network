// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::BoxAligned;

pub trait HasBoundingBox {
    fn bounding_box(&self) -> BoxAligned;
}
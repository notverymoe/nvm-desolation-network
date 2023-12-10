// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::prelude::BoxAligned;

pub trait HasBoundingBox {
    fn bounding_box(&self) -> BoxAligned;
}
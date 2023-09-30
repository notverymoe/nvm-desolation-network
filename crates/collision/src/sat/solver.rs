// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

use crate::{Contact, Projection, SAT_BUFFER_CAP};

pub struct Solver {
    pub overlaps:           Vec<Contact>,
    pub buffer_axes:        ArrayVec<[      Vec2; SAT_BUFFER_CAP]>,
    pub buffer_projections: ArrayVec<[Projection; SAT_BUFFER_CAP]>,
}

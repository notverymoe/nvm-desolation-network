// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub(crate) start:  Vec2,
    pub(crate) end:    Vec2,
    pub(crate) normal: Vec2,
}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

pub trait NormalAtPoint {
    fn normal_at(&self, point: Vec2) -> Vec2;
}

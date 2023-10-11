// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::projection::{ProjectOnAxis, Projection};

mod rect;
pub use rect::*;

mod circle;
pub use circle::*;

#[derive(Debug, Clone, Copy)]
pub enum ShapeData {
    Rect(RectData),
    Circle(CircleData),
}

impl ProjectOnAxis for ShapeData {
    fn project_on_axis(&self, axis: Vec2) -> crate::projection::Projection {
        match self {
            ShapeData::Rect(data)   => data.project_on_axis(axis),
            ShapeData::Circle(data) => data.project_on_axis(axis),
        }
    }
}

pub struct Shape {
    pub origin: Vec2,
    pub data:   ShapeData,
}

impl ProjectOnAxis for Shape {
    fn project_on_axis(&self, axis: Vec2) -> Projection {
        self.data.project_on_axis(axis).offset_by(axis.dot(self.origin))
    }
}
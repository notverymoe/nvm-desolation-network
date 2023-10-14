// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{projection::{ProjectOnAxis, Projection}, ray::{RaycastTarget, RayCaster}};

mod rect;
pub use rect::*;

mod rect_rounded;
pub use rect_rounded::*;

mod circle;
pub use circle::*;

#[derive(Debug, Clone, Copy)]
pub enum ShapeData {
    Rect(RectData),
    Circle(CircleData),
    RectRounded(RectRoundedData),
}

impl From<RectData> for ShapeData {
    fn from(value: RectData) -> Self {
        Self::Rect(value)
    }
}

impl From<CircleData> for ShapeData {
    fn from(value: CircleData) -> Self {
        Self::Circle(value)
    }
}

impl From<RectRoundedData> for ShapeData {
    fn from(value: RectRoundedData) -> Self {
        Self::RectRounded(value)
    }
}

impl ProjectOnAxis for ShapeData {
    fn project_aabb(&self) -> [Projection; 2] {
        match self {
            ShapeData::Rect(data)        => data.project_aabb(),
            ShapeData::Circle(data)      => data.project_aabb(),
            ShapeData::RectRounded(data) => data.project_aabb(),
        }
    }

    fn project_on_axis(&self, axis: Vec2) -> crate::projection::Projection {
        match self {
            ShapeData::Rect(data)        => data.project_on_axis(axis),
            ShapeData::Circle(data)      => data.project_on_axis(axis),
            ShapeData::RectRounded(data) => data.project_on_axis(axis),
        }
    }
}

impl RaycastTarget for ShapeData {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        match self {
            ShapeData::Rect(data)        => data.raycast(ray),
            ShapeData::Circle(data)      => data.raycast(ray),
            ShapeData::RectRounded(data) => data.raycast(ray),
        }
    }
}

impl ShapeData {

    pub fn rect(size: Vec2) -> Self {
        Self::Rect(RectData{ size })
    }

    pub fn circle(radius: f32) -> Self {
        Self::Circle(CircleData{ radius })
    }

    pub fn rect_rounded(size: Vec2, radius: f32) -> Self {
        Self::RectRounded(RectRoundedData{ size, radius })
    }

    pub fn combine(self, other: Self) -> Self {
        match (self, other) {
            (ShapeData::Rect(r0),        ShapeData::Rect(r1)       ) => Self::rect(r0.size + r1.size),
            (ShapeData::Circle(c0),      ShapeData::Circle(c1)     ) => Self::circle(c0.radius + c1.radius),
            (ShapeData::RectRounded(r0), ShapeData::RectRounded(r1)) => Self::rect_rounded(r0.size + r1.size, r0.radius + r1.radius),
            (ShapeData::Rect(r),         ShapeData::Circle(c)) | (ShapeData::Circle(c), ShapeData::Rect(r)        ) => Self::rect_rounded(r.size, c.radius),
            (ShapeData::RectRounded(r0), ShapeData::Rect(r1) ) | (ShapeData::Rect(r1),  ShapeData::RectRounded(r0)) => Self::rect_rounded(r0.size + r1.size, r0.radius),
            (ShapeData::RectRounded(r),  ShapeData::Circle(c)) | (ShapeData::Circle(c), ShapeData::RectRounded(r) ) => Self::rect_rounded(r.size, c.radius + r.radius),
        }
    }

}

pub struct Shape {
    pub origin: Vec2,
    pub data:   ShapeData,
}

impl Shape {

    pub fn new(origin: Vec2, data: impl Into<ShapeData>) -> Self {
        Self{origin, data: data.into()}
    }

}

impl ProjectOnAxis for Shape {
    fn project_aabb(&self) -> [Projection; 2] {
        let [x, y] = self.data.project_aabb();
        [x.offset_by(self.origin.x), y.offset_by(self.origin.y)]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        self.data.project_on_axis(axis).offset_by(axis.dot(self.origin))
    }
}

impl RaycastTarget for Shape {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        self.data.raycast(&ray.with_offset(-self.origin))
    }
}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Color, Gizmos};

use crate::{Projection, ProjectOnAxis, RaycastTarget, RayCaster, GizmoRenderable};

mod rect;
pub use rect::*;

mod rect_rounded;
pub use rect_rounded::*;

mod circle;
pub use circle::*;

mod slope;
pub use slope::*;

mod slope_rounded;
pub use slope_rounded::*;

mod ngon;
pub use ngon::*;

mod ngon_traced;
pub use ngon_traced::*;

#[derive(Debug, Clone, Copy)]
pub enum ShapeData {
    Rect(RectData),
    Circle(CircleData),
    RectRounded(RectRoundedData),
    Slope(SlopeData),
    SlopeRounded(SlopeRoundedData),
    NGon3(NGonData<3>),
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

impl From<SlopeData> for ShapeData {
    fn from(value: SlopeData) -> Self {
        Self::Slope(value)
    }
}

impl From<SlopeRoundedData> for ShapeData {
    fn from(value: SlopeRoundedData) -> Self {
        Self::SlopeRounded(value)
    }
}

impl From<NGonData<3>> for ShapeData {
    fn from(value: NGonData<3>) -> Self {
        Self::NGon3(value)
    }
}

impl ProjectOnAxis for ShapeData {
    fn project_aabb(&self) -> [Projection; 2] {
        match self {
            ShapeData::Rect(data)         => data.project_aabb(),
            ShapeData::Circle(data)       => data.project_aabb(),
            ShapeData::RectRounded(data)  => data.project_aabb(),
            ShapeData::Slope(data)        => data.project_aabb(),
            ShapeData::SlopeRounded(data) => data.project_aabb(),
            ShapeData::NGon3(data)        => data.project_aabb(),
        }
    }

    fn project_on_axis(&self, axis: Vec2) -> crate::projection::Projection {
        match self {
            ShapeData::Rect(data)         => data.project_on_axis(axis),
            ShapeData::Circle(data)       => data.project_on_axis(axis),
            ShapeData::RectRounded(data)  => data.project_on_axis(axis),
            ShapeData::Slope(data)        => data.project_on_axis(axis),
            ShapeData::SlopeRounded(data) => data.project_on_axis(axis),
            ShapeData::NGon3(data)        => data.project_on_axis(axis),
        }
    }
}

impl RaycastTarget for ShapeData {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        match self {
            ShapeData::Rect(data)         => data.raycast(ray),
            ShapeData::Circle(data)       => data.raycast(ray),
            ShapeData::RectRounded(data)  => data.raycast(ray),
            ShapeData::Slope(data)        => data.raycast(ray),
            ShapeData::SlopeRounded(data) => data.raycast(ray),
            ShapeData::NGon3(data)        => data.raycast(ray),
        }
    }
    
    fn normal_at(&self, point: Vec2) -> Vec2 {
        match self {
            ShapeData::Rect(data)         => data.normal_at(point),
            ShapeData::Circle(data)       => data.normal_at(point),
            ShapeData::RectRounded(data)  => data.normal_at(point),
            ShapeData::Slope(data)        => data.normal_at(point),
            ShapeData::SlopeRounded(data) => data.normal_at(point),
            ShapeData::NGon3(data)        => data.normal_at(point),
        }
    }
}

impl GizmoRenderable for ShapeData {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        match self {
            ShapeData::Rect(data)         => data.render(gizmos, offset, color),
            ShapeData::Circle(data)       => data.render(gizmos, offset, color),
            ShapeData::RectRounded(data)  => data.render(gizmos, offset, color),
            ShapeData::Slope(data)        => data.render(gizmos, offset, color),
            ShapeData::SlopeRounded(data) => data.render(gizmos, offset, color),
            ShapeData::NGon3(data)        => data.render(gizmos, offset, color),
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
            
            (ShapeData::Rect(_), ShapeData::Slope(_)) => todo!(),
            (ShapeData::Circle(_), ShapeData::Slope(_)) => todo!(),
            (ShapeData::RectRounded(_), ShapeData::Slope(_)) => todo!(),
            (ShapeData::Slope(_), ShapeData::Rect(_)) => todo!(),
            (ShapeData::Slope(_), ShapeData::Circle(_)) => todo!(),
            (ShapeData::Slope(_), ShapeData::RectRounded(_)) => todo!(),
            (ShapeData::Slope(_), ShapeData::Slope(_)) => todo!(),
            (ShapeData::Rect(_), ShapeData::SlopeRounded(_)) => todo!(),
            (ShapeData::Circle(_), ShapeData::SlopeRounded(_)) => todo!(),
            (ShapeData::RectRounded(_), ShapeData::SlopeRounded(_)) => todo!(),
            (ShapeData::Slope(_), ShapeData::SlopeRounded(_)) => todo!(),
            (ShapeData::SlopeRounded(_), ShapeData::Rect(_)) => todo!(),
            (ShapeData::SlopeRounded(_), ShapeData::Circle(_)) => todo!(),
            (ShapeData::SlopeRounded(_), ShapeData::RectRounded(_)) => todo!(),
            (ShapeData::SlopeRounded(_), ShapeData::Slope(_)) => todo!(),
            (ShapeData::SlopeRounded(_), ShapeData::SlopeRounded(_)) => todo!(),
            (ShapeData::Rect(_), ShapeData::NGon3(_)) => todo!(),
            (ShapeData::Circle(_), ShapeData::NGon3(_)) => todo!(),
            (ShapeData::RectRounded(_), ShapeData::NGon3(_)) => todo!(),
            (ShapeData::Slope(_), ShapeData::NGon3(_)) => todo!(),
            (ShapeData::SlopeRounded(_), ShapeData::NGon3(_)) => todo!(),
            (ShapeData::NGon3(_), ShapeData::Rect(_)) => todo!(),
            (ShapeData::NGon3(_), ShapeData::Circle(_)) => todo!(),
            (ShapeData::NGon3(_), ShapeData::RectRounded(_)) => todo!(),
            (ShapeData::NGon3(_), ShapeData::Slope(_)) => todo!(),
            (ShapeData::NGon3(_), ShapeData::SlopeRounded(_)) => todo!(),
            (ShapeData::NGon3(_), ShapeData::NGon3(_)) => todo!(),
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
    
    fn normal_at(&self, point: Vec2) -> Vec2 {
        self.data.normal_at(point - self.origin)
    }
}

impl GizmoRenderable for Shape {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        self.data.render(gizmos, self.origin + offset, color)
    }
}
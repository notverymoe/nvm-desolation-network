// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;
use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::{prelude::{BoxAligned, Ball, Ramp, BoxOriented, BoxAlignedRound, BoxOrientedRound, RampRound, RampBoxy, RampBoxyRound, BoxOrientedBoxy, BoxOrientedBoxyRound, RaycastTarget, RayCaster, RayIntersection, DebugShapeData, DebugShape}, shape_common::ShapeCommon};

macro_attr! {
    #[derive(EnumFromInner!)]
    pub enum ShapeMoving {
        Ball(Ball),
        BoxAligned(BoxAligned),
    }
}

impl ShapeMoving {

    pub fn origin(&self) -> Vec2 {
        match self {
            ShapeMoving::Ball(s)       => s.origin,
            ShapeMoving::BoxAligned(s) => s.origin,
        }
    }

}

impl ShapeCommon for ShapeMoving {
    fn bounding_box(&self) -> BoxAligned {
        match self {
            ShapeMoving::Ball(s)       => s.bounding_box(),
            ShapeMoving::BoxAligned(s) => s.bounding_box(),
        }
    }

    fn origin(&self) -> Vec2 {
        match self {
            ShapeMoving::Ball(s)       => s.origin(),
            ShapeMoving::BoxAligned(s) => s.origin(),
        }
    }

    fn set_origin(&mut self, origin: Vec2) {
        match self {
            ShapeMoving::Ball(s)       => s.set_origin(origin),
            ShapeMoving::BoxAligned(s) => s.set_origin(origin),
        }
    }
}

impl DebugShape for ShapeMoving {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        match self {
            ShapeMoving::Ball(s) => s.get_debug_shape_data(),
            ShapeMoving::BoxAligned(s) => s.get_debug_shape_data(),
        }
    }
}

impl RaycastTarget for ShapeMoving {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeMoving::Ball(s) => s.raycast(ray),
            ShapeMoving::BoxAligned(s) => s.raycast(ray),
        }
    }

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeMoving::Ball(s) => s.raycast_enter(ray),
            ShapeMoving::BoxAligned(s) => s.raycast_enter(ray),
        }
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeMoving::Ball(s) => s.raycast_exit(ray),
            ShapeMoving::BoxAligned(s) => s.raycast_exit(ray),
        }
    }
}

macro_attr! {
    #[derive(EnumFromInner!, Debug, Copy, Clone)]
    pub enum ShapeStatic {
        Ball(Ball),
        BoxAligned(BoxAligned),
        BoxAlignedRound(BoxAlignedRound),
        BoxOriented(BoxOriented),
        BoxOrientedRound(BoxOrientedRound),
        Ramp(Ramp),
        RampRound(RampRound),
    }
}

impl ShapeCommon for ShapeStatic {
    fn bounding_box(&self) -> BoxAligned {
        match self {
            ShapeStatic::Ball(s)       => s.bounding_box(),
            ShapeStatic::BoxAligned(s) => s.bounding_box(),
            ShapeStatic::BoxAlignedRound(s) => s.bounding_box(),
            ShapeStatic::BoxOriented(s) => s.bounding_box(),
            ShapeStatic::BoxOrientedRound(s) => s.bounding_box(),
            ShapeStatic::Ramp(s) => s.bounding_box(),
            ShapeStatic::RampRound(s) => s.bounding_box(),
        }
    }

    fn origin(&self) -> Vec2 {
        match self {
            ShapeStatic::Ball(s)       => s.origin(),
            ShapeStatic::BoxAligned(s) => s.origin(),
            ShapeStatic::BoxAlignedRound(s) => s.origin(),
            ShapeStatic::BoxOriented(s) => s.origin(),
            ShapeStatic::BoxOrientedRound(s) => s.origin(),
            ShapeStatic::Ramp(s) => s.origin(),
            ShapeStatic::RampRound(s) => s.origin(),
        }
    }

    fn set_origin(&mut self, origin: Vec2) {
        match self {
            ShapeStatic::Ball(s)       => s.set_origin(origin),
            ShapeStatic::BoxAligned(s) => s.set_origin(origin),
            ShapeStatic::BoxAlignedRound(s) => s.set_origin(origin),
            ShapeStatic::BoxOriented(s) => s.set_origin(origin),
            ShapeStatic::BoxOrientedRound(s) => s.set_origin(origin),
            ShapeStatic::Ramp(s) => s.set_origin(origin),
            ShapeStatic::RampRound(s) => s.set_origin(origin),
        }
    }
}

impl DebugShape for ShapeStatic {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        match self {
            ShapeStatic::Ball(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxAligned(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxAlignedRound(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxOriented(s) => s.get_debug_shape_data(),
            ShapeStatic::BoxOrientedRound(s) => s.get_debug_shape_data(),
            ShapeStatic::Ramp(s) => s.get_debug_shape_data(),
            ShapeStatic::RampRound(s) => s.get_debug_shape_data(),
        }
    }
}

impl RaycastTarget for ShapeStatic {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeStatic::Ball(s) => s.raycast(ray),
            ShapeStatic::BoxAligned(s) => s.raycast(ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast(ray),
            ShapeStatic::BoxOriented(s) => s.raycast(ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast(ray),
            ShapeStatic::Ramp(s) => s.raycast(ray),
            ShapeStatic::RampRound(s) => s.raycast(ray),
        }
    }

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeStatic::Ball(s) => s.raycast_enter(ray),
            ShapeStatic::BoxAligned(s) => s.raycast_enter(ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast_enter(ray),
            ShapeStatic::BoxOriented(s) => s.raycast_enter(ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast_enter(ray),
            ShapeStatic::Ramp(s) => s.raycast_enter(ray),
            ShapeStatic::RampRound(s) => s.raycast_enter(ray),
        }
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeStatic::Ball(s) => s.raycast_exit(ray),
            ShapeStatic::BoxAligned(s) => s.raycast_exit(ray),
            ShapeStatic::BoxAlignedRound(s) => s.raycast_exit(ray),
            ShapeStatic::BoxOriented(s) => s.raycast_exit(ray),
            ShapeStatic::BoxOrientedRound(s) => s.raycast_exit(ray),
            ShapeStatic::Ramp(s) => s.raycast_exit(ray),
            ShapeStatic::RampRound(s) => s.raycast_exit(ray),
        }
    }
}

macro_attr! {
    #[derive(EnumFromInner!)]
    pub enum ShapeCombined {
        Ball(Ball),
        
        BoxAligned(BoxAligned),
        BoxAlignedRound(BoxAlignedRound),

        BoxOrientedRound(BoxOrientedRound),
        BoxOrientedBoxy(BoxOrientedBoxy),
        BoxOrientedBoxyRound(BoxOrientedBoxyRound),

        RampRound(RampRound),
        RampBoxy(RampBoxy),
        RampBoxyRound(RampBoxyRound),
    }
}

impl ShapeCommon for ShapeCombined {
    fn bounding_box(&self) -> BoxAligned {
        match self {
            ShapeCombined::Ball(s)       => s.bounding_box(),
            ShapeCombined::BoxAligned(s) => s.bounding_box(),
            ShapeCombined::BoxAlignedRound(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedRound(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedBoxy(s) => s.bounding_box(),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.bounding_box(),
            ShapeCombined::RampRound(s) => s.bounding_box(),
            ShapeCombined::RampBoxy(s) => s.bounding_box(),
            ShapeCombined::RampBoxyRound(s) => s.bounding_box(),
        }
    }

    fn origin(&self) -> Vec2 {
        match self {
            ShapeCombined::Ball(s)       => s.origin(),
            ShapeCombined::BoxAligned(s) => s.origin(),
            ShapeCombined::BoxAlignedRound(s) => s.origin(),
            ShapeCombined::BoxOrientedRound(s) => s.origin(),
            ShapeCombined::BoxOrientedBoxy(s) => s.origin(),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.origin(),
            ShapeCombined::RampRound(s) => s.origin(),
            ShapeCombined::RampBoxy(s) => s.origin(),
            ShapeCombined::RampBoxyRound(s) => s.origin(),
        }
    }

    fn set_origin(&mut self, origin: Vec2) {
        match self {
            ShapeCombined::Ball(s)       => s.set_origin(origin),
            ShapeCombined::BoxAligned(s) => s.set_origin(origin),
            ShapeCombined::BoxAlignedRound(s) => s.set_origin(origin),
            ShapeCombined::BoxOrientedRound(s) => s.set_origin(origin),
            ShapeCombined::BoxOrientedBoxy(s) => s.set_origin(origin),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.set_origin(origin),
            ShapeCombined::RampRound(s) => s.set_origin(origin),
            ShapeCombined::RampBoxy(s) => s.set_origin(origin),
            ShapeCombined::RampBoxyRound(s) => s.set_origin(origin),
        }
    }
}

impl DebugShape for ShapeCombined {
    fn get_debug_shape_data(&self) -> DebugShapeData {
        match self {
            ShapeCombined::Ball(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxAligned(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxAlignedRound(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedRound(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedBoxy(s) => s.get_debug_shape_data(),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.get_debug_shape_data(),
            ShapeCombined::RampRound(s) => s.get_debug_shape_data(),
            ShapeCombined::RampBoxy(s) => s.get_debug_shape_data(),
            ShapeCombined::RampBoxyRound(s) => s.get_debug_shape_data(),
        }
    }
}

impl RaycastTarget for ShapeCombined {
    fn raycast(&self, ray: &RayCaster) -> Option<[RayIntersection; 2]> {
        match self {
            ShapeCombined::Ball(s) => s.raycast(ray),
            ShapeCombined::BoxAligned(s) => s.raycast(ray),
            ShapeCombined::BoxAlignedRound(s) => s.raycast(ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast(ray),
            ShapeCombined::BoxOrientedBoxy(s) => s.raycast(ray),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.raycast(ray),
            ShapeCombined::RampRound(s) => s.raycast(ray),
            ShapeCombined::RampBoxy(s) => s.raycast(ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast(ray),
        }
    }

    fn raycast_enter(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeCombined::Ball(s) => s.raycast_enter(ray),
            ShapeCombined::BoxAligned(s) => s.raycast_enter(ray),
            ShapeCombined::BoxAlignedRound(s) => s.raycast_enter(ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast_enter(ray),
            ShapeCombined::BoxOrientedBoxy(s) => s.raycast_enter(ray),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.raycast_enter(ray),
            ShapeCombined::RampRound(s) => s.raycast_enter(ray),
            ShapeCombined::RampBoxy(s) => s.raycast_enter(ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast_enter(ray),
        }
    }

    fn raycast_exit(&self, ray: &RayCaster) -> Option<RayIntersection> {
        match self {
            ShapeCombined::Ball(s) => s.raycast_exit(ray),
            ShapeCombined::BoxAligned(s) => s.raycast_exit(ray),
            ShapeCombined::BoxAlignedRound(s) => s.raycast_exit(ray),
            ShapeCombined::BoxOrientedRound(s) => s.raycast_exit(ray),
            ShapeCombined::BoxOrientedBoxy(s) => s.raycast_exit(ray),
            ShapeCombined::BoxOrientedBoxyRound(s) =>s.raycast_exit(ray),
            ShapeCombined::RampRound(s) => s.raycast_exit(ray),
            ShapeCombined::RampBoxy(s) => s.raycast_exit(ray),
            ShapeCombined::RampBoxyRound(s) => s.raycast_exit(ray),
        }
    }
}

impl ShapeCombined {

    pub fn between_moving_and_static(a: &ShapeMoving, b: &ShapeStatic) -> Self {
        match (a, b) {
            (ShapeMoving::Ball(a),       ShapeStatic::Ball(b)           ) => Ball::new(b.origin, a.radius+b.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::BoxAligned(b)     ) => BoxAlignedRound::new(b.origin, b.size, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::BoxAlignedRound(b)) => BoxAlignedRound::new(b.origin, b.size, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::Ball(b)           ) => BoxAlignedRound::new(b.origin, a.size, b.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxAligned(b)     ) => BoxAligned::new(b.origin, a.size + b.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxAlignedRound(b)) => BoxAlignedRound::new(b.origin, a.size + b.size, b.radius).into(),

            (ShapeMoving::Ball(a),       ShapeStatic::BoxOriented(b)     ) => BoxOrientedRound::new(b.origin, b.size, b.direction, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::BoxOrientedRound(b)) => BoxOrientedRound::new(b.origin, b.size, b.direction, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxOriented(b)     ) => BoxOrientedBoxy::new(b.origin, b.size, b.direction, a.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::BoxOrientedRound(b)) => BoxOrientedBoxyRound::new(b.origin, b.size, b.direction, a.size, b.radius).into(),

            // TODO do we need to "invert" ramps?
            // NOTE seems not?
            (ShapeMoving::Ball(a),       ShapeStatic::Ramp(b)     ) => RampRound::new(b.origin, b.direction, b.length, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::RampRound(b)) => RampRound::new(b.origin, b.direction, b.length, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::Ramp(b)     ) => RampBoxy::new(b.origin + b.get_normal().signum()*a.size, b.direction, b.length, a.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::RampRound(b)) => RampBoxyRound::new(b.origin + b.get_normal().signum()*a.size, b.direction, b.length, a.size, b.radius).into(),
        } 
    }

    pub fn between_moving(a: &ShapeMoving, b: &ShapeMoving) -> Self {
        match (a, b) {
            (ShapeMoving::Ball(a),       ShapeMoving::Ball(b)      ) => Ball::new(b.origin, a.radius+b.radius).into(),
            (ShapeMoving::Ball(a),       ShapeMoving::BoxAligned(b)) => BoxAlignedRound::new(b.origin, b.size, a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeMoving::Ball(b)      ) => BoxAlignedRound::new(b.origin, a.size, b.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeMoving::BoxAligned(b)) => BoxAligned::new(b.origin, a.size + b.size).into(),
        }
    }

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::{BoxAligned, Ball, Ramp, BoxOriented, BoxAlignedRound, BoxOrientedRound, RampRound, RampBoxy, RampBoxyRound, BoxOrientedBoxy, BoxOrientedBoxyRound, RaycastTarget, RayCaster, RayIntersection};

macro_attr! {
    #[derive(EnumFromInner!)]
    pub enum ShapeMoving {
        Ball(Ball),
        BoxAligned(BoxAligned),
    }
}

macro_attr! {
    #[derive(EnumFromInner!)]
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
            (ShapeMoving::Ball(a),       ShapeStatic::Ramp(b)     ) => RampRound::new(b.origin, b.direction, b.length, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::RampRound(b)) => RampRound::new(b.origin, b.direction, b.length, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::Ramp(b)     ) => RampBoxy::new(b.origin, b.direction, b.length, a.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::RampRound(b)) => RampBoxyRound::new(b.origin, b.direction, b.length, a.size, b.radius).into(),
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
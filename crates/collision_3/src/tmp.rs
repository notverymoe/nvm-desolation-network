// Copyright 2023 Natalie Baker // AGPLv3 //

use macro_attr_2018::macro_attr;
use enum_derive_2018::EnumFromInner;

use crate::{BoxAligned, Ball, Ramp, BoxOriented, BoxAlignedRound, BoxOrientedRound, RampRound, RampBoxy, RampBoxyRound, BoxOrientedBoxy, BoxOrientedBoxyRound};

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

impl ShapeCombined {

    pub fn between_moving_and_static(a: ShapeMoving, b: ShapeStatic) -> Self {
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

            (ShapeMoving::Ball(a),       ShapeStatic::Ramp(b)     ) => RampRound::new(b.origin, b.direction, b.length, a.radius).into(),
            (ShapeMoving::Ball(a),       ShapeStatic::RampRound(b)) => RampRound::new(b.origin, b.direction, b.length, b.radius + a.radius).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::Ramp(b)     ) => RampBoxy::new(b.origin, b.direction, b.length, a.size).into(),
            (ShapeMoving::BoxAligned(a), ShapeStatic::RampRound(b)) => RampBoxyRound::new(b.origin, b.direction, b.length, a.size, b.radius).into(),
        } 
    }

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

mod projection;
pub use projection::*;

mod sweep;
pub use sweep::*;

pub trait SATShape {
    const CAN_SMEAR_PROJECTION: bool;

    fn project_on_axis(&self, axis: Vec2) -> Projection;
    
    fn get_points(&self, out_points: &mut Vec<Vec2>);
    
    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>);
    
    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut Vec<Vec2>);

    fn with_offset(self, offset: Vec2) -> Self;
}

impl SATShape for Vec2 {
    const CAN_SMEAR_PROJECTION: bool = true;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::new(axis.dot(*self))
    }

    fn get_points(&self, out_points: &mut Vec<Vec2>) {
        out_points.push(*self)
    }

    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>) {
        out_axes.extend_from_slice(&[Vec2::X, Vec2::Y]);
        out_cache.extend_from_slice(&[Projection::new(self.x), Projection::new(self.y)])
    }

    fn get_axes_derived(&self, _other: &[Vec2], _out_axes: &mut Vec<Vec2>) {
        // no derived axes to test
    }

    fn with_offset(self, offset: Vec2) -> Self {
        self + offset
    }
}
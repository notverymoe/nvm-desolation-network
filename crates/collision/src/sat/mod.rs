// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

mod projection;
pub use projection::*;

mod sweep;
pub use sweep::*;

pub trait SATShape {
    fn project_on_axis(&self, axis: Vec2) -> Projection;
    fn get_points(&self, out_points: &mut Vec<Vec2>);
    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>);
    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut Vec<Vec2>);
}

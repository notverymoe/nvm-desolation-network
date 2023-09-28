// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::{SlopeOriented, Rect}, Projection, SATShape};

#[derive(Clone, Copy)]
pub struct Slope {
    pub origin: Vec2,
    pub run:    f32,
    pub rise:   f32,
}

impl SATShape for Slope {

    const CAN_SMEAR_PROJECTION: bool = true;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points())
    }

    fn get_points(&self, out_points: &mut Vec<Vec2>) {
        out_points.extend_from_slice(&self.points())
    }

    fn get_axes(&self, out_axes: &mut Vec<Vec2>, out_cache: &mut Vec<Projection>) {
        // OPT
        let norms  = [Vec2::X, Vec2::Y, Vec2::new(self.rise, self.run).normalize()];
        let points = self.points();
        out_axes.extend_from_slice(&norms);
        out_cache.extend_from_slice(&[
            Projection::from_points_iter(norms[0], points),
            Projection::from_points_iter(norms[1], points),
            Projection::from_points_iter(norms[2], points),
        ])
    }

    fn get_axes_derived(&self, _other: &[Vec2], _out_axes: &mut Vec<Vec2>) {
        // no derived axes
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.origin += offset;
        self
    }

}

impl HasBoundingBox for Slope {

    fn get_bounding_box(&self) -> Rect {
        let (min_x, max_x) = if self.run  <= 0.0 { (self.origin.x + self.run,  self.origin.x) } else { (self.origin.x, self.origin.x + self.run ) };
        let (min_y, max_y) = if self.rise <= 0.0 { (self.origin.y + self.rise, self.origin.y) } else { (self.origin.x, self.origin.y + self.rise) };
        Rect{
            min: Vec2::new(min_x, max_x),
            max: Vec2::new(min_y, max_y),
        }
    }

}

impl Slope {

    pub fn points(&self) -> [Vec2; 3] {
        let point_run  = self.origin + Vec2::new(self.run,       0.0);
        let point_rise = self.origin + Vec2::new(     0.0, self.rise);

        if (self.run >= 0.0) == (self.rise >= 0.0) {
            [self.origin, point_run, point_rise]
        } else {
            [self.origin, point_rise, point_run]
        }
    }

    pub fn with_orientation(self, up: Vec2) -> SlopeOriented {
        let Slope{origin, run, rise} = self;
        SlopeOriented{origin, run, rise, up}
    }
}

impl From<SlopeOriented> for Slope {
    fn from(value: SlopeOriented) -> Self {
        let SlopeOriented{origin, run, rise, up: _} = value;
        Self{origin, run, rise}
    }
}
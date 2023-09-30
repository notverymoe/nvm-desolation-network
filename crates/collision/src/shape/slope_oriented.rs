// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::math::Vec2;

use crate::{HasBoundingBox, shape::Rect, Projection, SATShape, VecLike};

#[derive(Clone, Copy)]
pub struct SlopeOriented {
    pub origin: Vec2,
    pub run:    f32,
    pub rise:   f32,
    pub up:     Vec2,
}

impl SATShape for SlopeOriented {

    const CAN_SMEAR_PROJECTION: bool = true;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points())
    }
    
    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        out_points.extend_from_slice(&self.points())
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<Projection>) {
        // OPT
        let norms  = [self.up.perp(), self.up, (self.up*self.rise + self.up.perp()*self.run).perp().normalize()];
        let points = self.points();
        out_axes.extend_from_slice(&norms);
        out_projections.extend_from_slice(&[
            Projection::from_points_iter(norms[0], points),
            Projection::from_points_iter(norms[1], points),
            Projection::from_points_iter(norms[2], points),
        ])
    }

    fn get_axes_derived(&self, _other: &[Vec2], _out_axes: &mut impl VecLike<Vec2>) {
        // no derived axes
    }

    fn with_offset(mut self, offset: Vec2) -> Self {
        self.origin += offset;
        self
    }

}

impl HasBoundingBox for SlopeOriented {

    fn get_bounding_box(&self) -> Rect {
        // TODO optimize this gross thing
        let points = self.points();

        let [min_x, max_x] = points[1..].iter().fold([self.origin.x, self.origin.x], |mut p, c| {
            p[0] = p[0].min(c.x);
            p[1] = p[1].max(c.x);
            p
        });

        let [min_y, max_y] = points[1..].iter().fold([self.origin.y, self.origin.y], |mut p, c| {
            p[0] = p[0].min(c.y);
            p[1] = p[1].max(c.y);
            p
        });

        Rect{
            min: Vec2::new(min_x, max_x),
            max: Vec2::new(min_y, max_y),
        }
    }

}

impl SlopeOriented {

    pub fn points(&self) -> [Vec2; 3] {
        let point_run  = self.origin + self.run  * self.up.perp();
        let point_rise = self.origin + self.rise * self.up;

        if (self.run >= 0.0) == (self.rise >= 0.0) {
            [self.origin, point_run, point_rise]
        } else {
            [self.origin, point_rise, point_run]
        }
    }

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{projection::{Projection, ProjectOnAxis}, ray::{RaycastTarget, Ray}};

#[derive(Debug, Clone, Copy)]
pub struct RectRoundedData {
    pub size:   Vec2,
    pub radius: f32,
}

impl ProjectOnAxis for RectRoundedData {

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        // Don't ask, this works, it's magic. See RectData for some info.
        let axis_dp = axis.abs().dot(self.size) + self.radius;
        Projection([-axis_dp, axis_dp])
    }

}

impl RaycastTarget for RectRoundedData {

    fn raycast(&self, ray: &Ray) -> Option<Projection> {
        // OPT we could use a modified rect intersecton to only operate on one axis
        [
            ray.find_circle_intersection(Vec2::new( self.size.x,  self.size.y), self.radius),
            ray.find_circle_intersection(Vec2::new(-self.size.x,  self.size.y), self.radius),
            ray.find_circle_intersection(Vec2::new(-self.size.x, -self.size.y), self.radius),
            ray.find_circle_intersection(Vec2::new( self.size.x, -self.size.y), self.radius),
            ray.find_rect_intersection(
                Vec2::new(-(self.size.x - self.radius), -self.size.y), 
                Vec2::new(  self.size.x - self.radius,  -self.size.y)
            ),
            ray.find_rect_intersection(
                Vec2::new(-self.size.x, -(self.size.y - self.radius)), 
                Vec2::new( self.size.x,   self.size.y - self.radius )
            ),
        ].iter().filter_map(|v| *v).reduce(|p, c| p.merged_with(c))
        
    }

}
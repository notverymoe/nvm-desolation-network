// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::projection::Projection;

pub trait RaycastTarget {
    fn raycast(&self, ray: &Ray) -> Option<Projection>;
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin:         Vec2,
    origin_dp:      [f32; 2],
    direction:      Vec2,
    direction_inv:  Vec2,
}

impl Ray {

    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Self{
            origin, 
            origin_dp: [direction.dot(origin), direction.perp_dot(origin)],
            direction,
            direction_inv: Vec2::new(1.0/direction.x, 1.0/direction.y),
        }
    }

    pub fn origin(&self) -> Vec2 {
        self.origin
    }

    pub fn direction(&self) -> Vec2 {
        self.direction
    }

    pub fn with_offset(self, offset: Vec2) -> Self {
        let origin = self.origin + offset;
        Self{
            origin, 
            origin_dp:      [self.direction.dot(origin), self.direction.perp_dot(origin)],
            direction:      self.direction, 
            direction_inv:  self.direction_inv
        }
    }

} 

impl Ray {

    pub fn find_circle_intersection(&self, origin: Vec2, radius: f32) -> Option<Projection> {
        let shifted_dp = [self.origin_dp[0] - self.direction.dot(origin), self.origin_dp[1] - self.direction.perp_dot(origin)];
        find_circle_intersection_at_origin(shifted_dp, radius)
    }

    pub fn find_circle_intersection_at_origin(&self, radius: f32) -> Option<Projection> {
        find_circle_intersection_at_origin(self.origin_dp, radius)
    }

    pub fn find_rect_intersection(&self, min: Vec2, max: Vec2) -> Option<Projection> {
        let mut tmin = 0.0;
        let mut tmax = f32::INFINITY;

        for d in 0..2 {
            let t1 = (min[d] - self.origin[d]) * self.direction_inv[d];
            let t2 = (max[d] - self.origin[d]) * self.direction_inv[d];
            tmin = f32::min(f32::max(t1, tmin), f32::max(t2, tmin));
            tmax = f32::max(f32::min(t1, tmax), f32::min(t2, tmax));
        }
    
        if tmin < tmax {
            Some(Projection([tmin, tmax]))
        } else {
            None
        }
    }

}

fn find_circle_intersection_at_origin(
    ray_dp: [f32; 2],
    radius: f32
) -> Option<Projection> {
    if radius < ray_dp[1] { return None; }
    let offset = radius*(1.0-(ray_dp[1]/radius).powi(2)).sqrt();
    Some(Projection([
        -offset - ray_dp[0], 
         offset - ray_dp[0]
    ]))
}
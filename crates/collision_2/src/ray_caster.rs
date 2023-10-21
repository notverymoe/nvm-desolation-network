// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

pub trait RaycastTarget {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection>;
    fn normal_at(&self, point: Vec2) -> Vec2;
}

#[derive(Debug, Clone, Copy)]
pub struct RayCaster {
    origin:         Vec2,
    origin_dp:      [f32; 2],
    direction:      Vec2,
    direction_inv:  Vec2,
}

impl RayCaster {

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

impl RayCaster {

    pub fn test_static(&self, other: &impl RaycastTarget) -> Option<Projection> {
        other.raycast(self)
    }

}

impl RayCaster {

    pub fn find_circle_intersection(&self, origin: Vec2, radius: f32) -> Option<Projection> {
        let shifted_dp = [
            self.origin_dp[0] - self.direction.dot(origin),
            self.origin_dp[1] - self.direction.perp_dot(origin)
        ];
        find_circle_intersection_at_origin(shifted_dp, radius)
    }

    pub fn find_circle_intersection_at_origin(&self, radius: f32) -> Option<Projection> {
        find_circle_intersection_at_origin(self.origin_dp, radius)
    }

    pub fn find_rect_intersection(&self, min: Vec2, max: Vec2) -> Option<Projection> {
        let mut tmin = -f32::INFINITY;
        let mut tmax =  f32::INFINITY;

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

    pub fn find_ray_intersection(&self, other_origin: Vec2, other_dir: Vec2) -> Option<[Projection; 2]> {
        let inv_pdp = 1.0/self.direction.perp_dot(other_dir);
        if inv_pdp != f32::INFINITY {  
            Some([
                Projection::new(      other_dir.perp_dot( self.origin - other_origin) * inv_pdp),
                Projection::new(-self.direction.perp_dot(other_origin -  self.origin) * inv_pdp),
            ])
        } else {
            None
        }
    }

    pub fn find_line_intersection(&self, from: Vec2, to: Vec2) -> Option<Projection> {
        let offset = to - from;
        let len = offset.length();
        let dir = offset/len;
        self.find_bounded_ray_intersection(from, dir, len)
    }

    pub fn find_bounded_ray_intersection(&self, from: Vec2, dir: Vec2, len: f32) -> Option<Projection> {
        self.find_ray_intersection(from, dir).and_then(|[r, p]| if p.min() >= 0.0 && p.min() <= len { Some(r) } else { None })
    }

}

fn find_circle_intersection_at_origin(
    ray_dp: [f32; 2],
    radius: f32
) -> Option<Projection> { 
    if radius < ray_dp[1].abs() { return None; }
    let offset = radius*(1.0-(ray_dp[1]/radius).powi(2)).sqrt();
    Some(Projection([
        -offset - ray_dp[0], 
         offset - ray_dp[0]
    ]))
}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{RayCaster, Projection};

    use std::f32::consts as f32;

    fn test_ray_on(a: &RayCaster, b: &RayCaster) -> Option<[Projection; 2]> {
        a.find_ray_intersection(b.origin(), b.direction())
    }

    macro_rules! assert_ray_test {
        ($a:expr, $b:expr, $c:expr, $d:expr) => { assert_eq!(test_ray_on($a, $b), Some([Projection::new($c), Projection::new($d)])); };
        ($a:expr, $b:expr) => { assert_eq!(test_ray_on($a, $b), None); };
    }

    #[test]
    fn test_rays() {
        let ray_py_along_px = RayCaster::new(Vec2::Y,  Vec2::X);
        let ray_py_along_nx = RayCaster::new(Vec2::Y, -Vec2::X);
        let ray_px_along_py = RayCaster::new(Vec2::X,  Vec2::Y);
        let ray_px_along_ny = RayCaster::new(Vec2::X, -Vec2::Y);
        let ray_zero_on_p45 = RayCaster::new(Vec2::ZERO, Vec2::ONE.normalize());
        let ray_zero_on_ang = RayCaster::new(Vec2::ZERO, Vec2::new(10.0, 1.0).normalize());

        assert_ray_test!(&ray_py_along_px, &ray_px_along_py,  1.0,  1.0);
        assert_ray_test!(&ray_py_along_px, &ray_px_along_ny,  1.0, -1.0);
        assert_ray_test!(&ray_py_along_px, &ray_zero_on_p45,  1.0,  f32::SQRT_2);
        assert_ray_test!(&ray_py_along_px, &ray_zero_on_ang, 10.0,  (1.0_f32 + 10.0*10.0).sqrt());

        assert_ray_test!(&ray_py_along_nx, &ray_px_along_py,  -1.0,  1.0);
        assert_ray_test!(&ray_py_along_nx, &ray_px_along_ny,  -1.0, -1.0);
        assert_ray_test!(&ray_py_along_nx, &ray_zero_on_p45,  -1.0,  f32::SQRT_2);
        assert_ray_test!(&ray_py_along_nx, &ray_zero_on_ang, -10.0,  (1.0_f32 + 10.0*10.0).sqrt());
    }

    #[test]
    fn test_lines() {
        let ray_under = RayCaster::new(Vec2::Y * -100.0, Vec2::X);
        let ray_above = RayCaster::new(Vec2::Y *  100.0, Vec2::X);
        let ray_in    = RayCaster::new(Vec2::Y, Vec2::X);

        let line_org = Vec2::X;
        let line_dir = Vec2::Y;
        let line_len = 10.0;

        assert_eq!(ray_under.find_bounded_ray_intersection(line_org, line_dir, line_len), None);
        assert_eq!(   ray_in.find_bounded_ray_intersection(line_org, line_dir, line_len), Some(Projection::new(1.0)));
        assert_eq!(ray_above.find_bounded_ray_intersection(line_org, line_dir, line_len), None);
    }

}
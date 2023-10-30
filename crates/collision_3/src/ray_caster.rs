// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct RayIntersection {
    pub distance: f32,
    pub point:    Vec2,
    pub normal:   Vec2,
}

pub trait RaycastTarget {
    fn raycast(&self, ray: RayCaster) -> Option<[RayIntersection; 2]>;

    fn raycast_enter(&self, ray: RayCaster) -> Option<RayIntersection> {
        self.raycast(ray).map(|[v, _]| v)
    }

    fn raycast_exit(&self, ray: RayCaster) -> Option<RayIntersection>{
        self.raycast(ray).map(|[_, v]| v)
    }
}

pub struct RayCaster {
    origin:        Vec2,
    origin_dp:     [f32; 2],
    direction:     Vec2,
    direction_inv: Vec2,
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

}

// ///////////////////// //
// // Raytest Circles // //
// ///////////////////// //

impl RayCaster {

    pub fn test_circle(&self, origin: Vec2, radius: f32) -> Option<[RayIntersection; 2]> {
        let ray_dp = self.offset_origin_dp(origin);
        RayCaster::calc_circle_center_offset(ray_dp, radius).map(|offset| {
            let distances = [-offset - ray_dp[0], offset - ray_dp[0]];
            let points  = distances.map(|d| self.origin + self.direction*d);
            let normals = points.map(|p| p.normalize());
    
            [
                RayIntersection{distance: distances[0], point: points[0], normal: normals[0]},
                RayIntersection{distance: distances[1], point: points[1], normal: normals[1]},
            ]
        })
    }

    pub fn test_circle_enter(&self, origin: Vec2, radius: f32) -> Option<RayIntersection> {
        let ray_dp = self.offset_origin_dp(origin);
        RayCaster::calc_circle_center_offset(ray_dp, radius).map(|offset| {

            let distance = -offset - ray_dp[0];
            let point  = self.origin + self.direction*distance;
            let normal = point.normalize();
    
            RayIntersection{distance, point, normal}
        })
    }

    pub fn test_circle_exit(&self, origin: Vec2, radius: f32) -> Option<RayIntersection> {
        let ray_dp = self.offset_origin_dp(origin);
        RayCaster::calc_circle_center_offset(ray_dp, radius).map(|offset| {

            let distance = offset - ray_dp[0];
            let point  = self.origin + self.direction*distance;
            let normal = point.normalize();
    
            RayIntersection{distance, point, normal}
        })
    }

    fn offset_origin_dp(&self, origin: Vec2) ->[f32; 2] {
        [
            self.origin_dp[0] - self.direction.dot(origin),
            self.origin_dp[1] - self.direction.perp_dot(origin)
        ]
    }

    fn calc_circle_center_offset(ray_dp: [f32; 2], radius: f32) -> Option<f32> {
        if radius < ray_dp[1].abs() { 
            None 
        } else {
            Some(radius*(1.0-(ray_dp[1]/radius).powi(2)).sqrt())
        }
    }

}

// /////////////////// //
// // Raytest Rects // //
// /////////////////// //

impl RayCaster {

    pub fn test_rect(&self, origin: Vec2, size: Vec2) -> Option<[RayIntersection; 2]> {
        let min = origin - size;
        let max = origin + size;

        let mut t = [-f32::INFINITY, f32::INFINITY];

        for d in 0..2 {
            t = Self::test_rect_minmax(d, &self.origin, &self.direction_inv, &min, &max, t);
        }
    
        if t[0] < t[1] {
            let points  = t.map(|t| self.origin + self.direction*t);
            let normals = points.map(|p| Self::find_rect_normal_at(p - origin, size)) ;
            Some([
                RayIntersection{distance: t[0], point: points[0], normal: normals[0]},
                RayIntersection{distance: t[1], point: points[1], normal: normals[1]},
            ])
        } else {
            None
        }
    }

    pub fn test_rect_enter(&self, origin: Vec2, size: Vec2) -> Option<RayIntersection> {
        self.test_rect(origin, size).map(|[v, _]| v)
    }

    pub fn test_rect_exit(&self, origin: Vec2, size: Vec2) -> Option<RayIntersection>{
        self.test_rect(origin, size).map(|[_, v]| v)
    }

    pub fn test_rect_rounded(&self, origin: Vec2, size: Vec2, radius: f32) -> Option<[RayIntersection; 2]> {
        // OPT axis aligned
        self.test_polygon_rounded_at_origin(
            &[
                origin + Vec2::new( size.x, -size.y),
                origin + Vec2::new( size.x,  size.y),
                origin + Vec2::new(-size.x,  size.y),
                origin + Vec2::new(-size.x, -size.y),
            ],
            &[
                 Vec2::X,
                 Vec2::Y,
                -Vec2::X,
                -Vec2::X,
            ],
            &[
                size.y,
                size.x,
                size.y,
                size.x
            ],
            radius
        )
    }

    pub fn test_rect_rounded_enter(&self, origin: Vec2, size: Vec2, radius: f32) -> Option<RayIntersection> {
        // OPT sided polygon test
        self.test_rect_rounded(origin, size, radius).map(|[v, _]| v)
    }

    pub fn test_rect_rounded_exit(&self, origin: Vec2, size: Vec2, radius: f32) -> Option<RayIntersection>{
        // OPT sided polygon test
        self.test_rect_rounded(origin, size, radius).map(|[_, v]| v)
    }

    fn test_rect_minmax(
        idx: usize, 
        origin: &Vec2, 
        direction_inv: &Vec2, 
        min: &Vec2, 
        max: &Vec2, 
        t: [f32; 2]
    ) -> [f32; 2] {
        let t1 = (min[idx] - origin[idx]) * direction_inv[idx];
        let t2 = (max[idx] - origin[idx]) * direction_inv[idx];
        [
            f32::min(f32::max(t1, t[0]), f32::max(t2, t[0])),
            f32::max(f32::min(t1, t[1]), f32::min(t2, t[1])),
        ]
    }
    
    fn find_rect_normal_at(point: Vec2, size: Vec2) -> Vec2 {
        let pnt_abs = point.abs();
        let dist_x = pnt_abs.x - size.x; 
        let dist_y = pnt_abs.y - size.y;

        // TODO OPT make this better

        // +/- XY Quad
        if dist_x >= 0.0 && dist_y >= 0.0 {
            return Vec2::new(
                point.x.signum() * std::f32::consts::FRAC_1_SQRT_2,
                point.y.signum() * std::f32::consts::FRAC_1_SQRT_2,
            );
        }

        // +Y Quad
        if dist_x <= 0.0 && dist_y >= 0.0 {
            return Vec2::new(0.0, point.y.signum());
        }

        // +X Quad
        if dist_x >= 0.0 && dist_y <= 0.0 {
            return Vec2::new(point.x.signum(), 0.0);
        }

        let pnt_scl  = pnt_abs/size;

        // -Y Quad
        if pnt_scl.y > pnt_scl.x {
            return Vec2::new(0.0, point.y.signum());
        }

        // -X Quad
        if pnt_scl.x > pnt_scl.y {
            return Vec2::new(point.x.signum(), 0.0);
        }

        // Inside && x == y
        Vec2::new(
            point.x.signum() * std::f32::consts::FRAC_1_SQRT_2,
            point.y.signum() * std::f32::consts::FRAC_1_SQRT_2,
        )
    }

}

// ////////////////////// //
// // Raytest Polygons // //
// ////////////////////// //

impl RayCaster {

    pub fn test_polygon(&self, origin: Vec2, points: &[Vec2], normals: &[Vec2], lengths: &[f32]) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit((0..points.len()).filter_map(|i| self.test_line_opt(origin + points[i], normals[i].perp(), lengths[i])))
    }

    pub fn test_polygon_rounded(&self, origin: Vec2, points: &[Vec2], normals: &[Vec2], lengths: &[f32], radius: f32) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit_signed(self.direction, (0..points.len()).flat_map(|i| {
            let point  = origin + points[i] + normals[i]*radius;
            let segment = self.test_line_opt(point, normals[i].perp(), lengths[i]);
            if let Some([c_a, c_b]) = self.test_circle(point, radius) {
                [segment, Some(c_a), Some(c_b)]
            } else {
                [segment, None, None]
            }
        }).flatten())
    }

    pub fn test_polygon_rounded_at_origin(&self, points: &[Vec2], normals: &[Vec2], lengths: &[f32], radius: f32) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit_signed(self.direction, (0..points.len()).flat_map(|i| {
            let point  = points[i] + normals[i]*radius;
            let segment = self.test_line_opt(point, normals[i].perp(), lengths[i]);
            if let Some([c_a, c_b]) = self.test_circle(point, radius) {
                [segment, Some(c_a), Some(c_b)]
            } else {
                [segment, None, None]
            }
        }).flatten())
    }

    pub fn test_polygon_at_origin(&self, points: &[Vec2], normals: &[Vec2], lengths: &[f32]) -> Option<[RayIntersection; 2]> {
        RayIntersection::find_polygon_entry_exit((0..points.len()).filter_map(|i| self.test_line_opt(points[i], normals[i].perp(), lengths[i])))
    }

}

// /////////////////// //
// // Raytest Lines // //
// /////////////////// //

impl RayCaster {

    pub fn test_line(&self, from: Vec2, to: Vec2) -> Option<RayIntersection> {
        let offset = to - from;
        let len = offset.length();
        let dir = offset/len;
        self.test_line_opt(from, dir, len)
    }

    pub fn test_line_opt(&self, from: Vec2, dir: Vec2, len: f32) -> Option<RayIntersection> {
        self.calc_ray_intersection_dp(from, dir).and_then(|[distance, p]| if p >= 0.0 && p <= len { 
            Some(RayIntersection {
                distance, 
                point:  self.origin + self.direction*distance, 
                normal: self.direction.perp() 
            }) 
        } else { 
            None 
        })
    }

    pub fn test_line_infinite(&self, from: Vec2, to: Vec2) -> Option<RayIntersection> {
        let dir = (to - from).normalize();
        self.test_line_infinite_opt(from, dir)
    }

    pub fn test_line_infinite_opt(&self, from: Vec2, dir: Vec2) -> Option<RayIntersection> {
        self.calc_ray_intersection_dp(from, dir).map(|[distance, _]| RayIntersection {
            distance, 
            point: self.origin + self.direction*distance, 
            normal: self.direction.perp() 
        })
    }

    fn calc_ray_intersection_dp(&self, other_origin: Vec2, other_dir: Vec2) -> Option<[f32; 2]> {
        let inv_pdp = 1.0/self.direction.perp_dot(other_dir);
        if inv_pdp != f32::INFINITY {  
            Some([
                      other_dir.perp_dot( self.origin - other_origin) * inv_pdp,
                -self.direction.perp_dot(other_origin -  self.origin) * inv_pdp,
            ])
        } else {
            None
        }
    }

}

impl RayIntersection {

    pub fn find_polygon_entry_exit(v: impl IntoIterator<Item = RayIntersection>) -> Option<[RayIntersection; 2]> {

        let mut entry = RayIntersection{ distance:  f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        let mut exit  = RayIntersection{ distance: -f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        for intersection in v.into_iter() {
            if intersection.distance < entry.distance {
                entry = intersection;
            } 

            if intersection.distance > exit.distance {
                exit = intersection;
            }
        }

        (exit.distance >= entry.distance).then_some([entry, exit])
    }

    pub fn find_polygon_entry_exit_signed(direction: Vec2, v: impl IntoIterator<Item = RayIntersection>) -> Option<[RayIntersection; 2]> {

        let mut entry = RayIntersection{ distance:  f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        let mut exit  = RayIntersection{ distance: -f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        for intersection in v.into_iter() {
            if intersection.distance < entry.distance && direction.dot(intersection.normal) > 0.0 {
                entry = intersection;
            } 

            if intersection.distance > exit.distance && direction.dot(intersection.normal) < 0.0 {
                exit = intersection;
            }
        }

        (exit.distance >= entry.distance).then_some([entry, exit])
    }

    pub fn find_polygon_entry_exit_pairs(v: impl IntoIterator<Item = [RayIntersection; 2]>) -> Option<[RayIntersection; 2]> {

        let mut entry = RayIntersection{ distance:  f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        let mut exit  = RayIntersection{ distance: -f32::MAX, point: Vec2::ZERO, normal: Vec2::ZERO };
        for [entry_intersection, exit_intersection] in v.into_iter() {
            if entry_intersection.distance < entry.distance {
                entry = entry_intersection;
            } 

            if exit_intersection.distance > exit.distance {
                exit = exit_intersection;
            }
        }

        (exit.distance >= entry.distance).then_some([entry, exit])
    }
    
}
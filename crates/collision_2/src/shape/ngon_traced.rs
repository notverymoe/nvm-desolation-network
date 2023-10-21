// Copyright 2023 Natalie Baker // AGPLv3 //

use std::fmt::Debug;

use bevy::prelude::{Vec2, Gizmos, Color};

use crate::{CollectSizedArray, RaycastTarget, RayCaster, Projection, ProjectOnAxis, GizmoRenderable, calculate_offsets_for, calculate_normals_for, calculate_lengths_for, PairIndexIter};

pub trait NGonInner: GizmoRenderable + RaycastTarget + ProjectOnAxis + Debug + Sync + Send + Clone + Copy {}
impl<T: GizmoRenderable + RaycastTarget + ProjectOnAxis + Debug + Clone + Sync + Send + Copy> NGonInner for T {}

#[derive(Debug, Clone, Copy)]
pub struct NGonDataTraced<T: NGonInner, const N: usize> {
    inner:     T,
    points:    [Vec2; N],
    normals:   [Vec2; N],
    lengths:   [ f32; N],
    offsets:   [ f32; N],
    proj_aabb: [Projection; 2],
}

impl<T: NGonInner, const N: usize> NGonDataTraced<T, N> {

    pub fn new(points: [Vec2; N], inner: T) -> Self {
        let other_aabb = inner.project_aabb();
        let normals = calculate_normals_for(&points);
        Self{
            lengths: calculate_lengths_for(&points),
            offsets: calculate_offsets_for(&normals, &inner),
            normals,
            proj_aabb: [
                Projection::from_points_iter(Vec2::X, points).swept_by(other_aabb[0]), 
                Projection::from_points_iter(Vec2::Y, points).swept_by(other_aabb[1]), 
            ],
            points,
            inner,
        }
    }

    pub fn points(&self) -> &[Vec2; N] {
        &self.points
    }

} 

impl<T: NGonInner, const N: usize> RaycastTarget for NGonDataTraced<T, N> {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        (0..N)
            .filter_map(|i| 
                ray.find_bounded_ray_intersection(
                    self.points[i] + self.normals[i]*self.offsets[i], 
                    self.normals[i].perp(), self.lengths[i]
                )
            )
            .chain(self.points.iter().filter_map(|&p| self.inner.raycast(&ray.with_offset(-p))))
            .reduce(|c, v| c.merged_with(v))
    }
    
    fn normal_at(&self, point: Vec2) -> Vec2 {
        let dp: [[f32; 2]; N] = (0..N).map(|i| {
            let point = point - self.points[i];
            [
                self.normals[i].dot(point),
                self.normals[i].perp_dot(point),
            ]
        }).try_collect_array().unwrap();

        // TODO OPT lazy evaluation, iterator with a ring buffer of n=2, caches this first
        for [i, j] in PairIndexIter::new(N) {
            let [i_dp, i_pdp] = dp[i];

            if i_dp < 0.0 {
                // is behind this segment
                continue;
            }

            let len = self.lengths[i];
            if (i_pdp >= 0.0) && (i_pdp <= len) {
                // is within the edge of this segment
                return self.normals[i];
            }

            let [j_dp, j_pdp] = dp[j];
            if j_dp < 0.0 {
                // is behind the next segment
                continue;
            }

            if (i_pdp > len) && (j_pdp < 0.0) {
                // is between segments, vertex is nearest
                return self.inner.normal_at(point - self.points[j]);
            }
        }

        self.normals[dp.iter().enumerate().max_by(|(_, [a, _]), (_, [b, _])| a.total_cmp(b)).unwrap().0]
    }
}

impl<T: NGonInner, const N: usize> ProjectOnAxis for NGonDataTraced<T, N> {
    fn project_aabb(&self) -> [Projection; 2] {
        self.proj_aabb
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points)
    }
}


impl<T: NGonInner, const N: usize> GizmoRenderable for NGonDataTraced<T, N> {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        gizmos.linestrip_2d(
            self.points.iter().map(|&v| v + offset).chain(std::iter::once(self.points[0] + offset)),
            color
        );

        for [i, j] in PairIndexIter::new(N) {
            gizmos.line_2d(
                self.points[i] + self.normals[i]*self.offsets[i] + offset,
                self.points[j] + self.normals[i]*self.offsets[i] + offset,
                color
            );
        }

        self.points.iter().for_each(|&v| self.inner.render(gizmos, v + offset, color))
    }
}

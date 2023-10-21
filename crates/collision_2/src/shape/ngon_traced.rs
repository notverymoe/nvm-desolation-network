// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Gizmos, Color};

use crate::{CollectSizedArray, RaycastTarget, RayCaster, Projection, ProjectOnAxis, GizmoRenderable, calculate_offsets_for, calculate_normals_for, calculate_lengths_for, PairIndexIter};

pub struct NGonDataTraced<const N: usize> {
    inner:     Box<dyn RaycastTarget>,
    points:    [Vec2; N],
    normals:   [Vec2; N],
    lengths:   [ f32; N],
    offsets:   [ f32; N],
    proj_aabb: [Projection; 2],
}

impl<const N: usize> NGonDataTraced<N> {

    pub fn new(points: [Vec2; N], other: impl RaycastTarget + ProjectOnAxis + 'static) -> Self {
        let normals = calculate_normals_for(&points);
        Self{
            lengths: calculate_lengths_for(&points),
            offsets: calculate_offsets_for(&normals, &other),
            inner: Box::new(other),
            normals,
            proj_aabb: [ // OPT we can just min-max components
                Projection::from_points_iter(Vec2::X, points), 
                Projection::from_points_iter(Vec2::Y, points), 
            ],
            points,
        }
    }

} 

impl<const N: usize> RaycastTarget for NGonDataTraced<N> {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        (0..N)
            .filter_map(|i| 
                ray.find_bounded_ray_intersection(
                    self.points[i] + self.normals[i]*self.offsets[i], 
                    self.normals[i].perp(), self.lengths[i]
                )
            )
            .chain(std::iter::once(self.inner.raycast(ray)).flatten())
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

impl<const N: usize> ProjectOnAxis for NGonDataTraced<N> {
    fn project_aabb(&self) -> [Projection; 2] {
        self.proj_aabb
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points)
    }
}


impl<const N: usize> GizmoRenderable for NGonDataTraced<N> {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        gizmos.linestrip_2d(
            self.points.iter().map(|&v| v + offset).chain(std::iter::once(self.points[0] + offset)),
            color
        );
    }
}

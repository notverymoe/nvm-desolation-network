// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Vec2, Gizmos, Color};

use crate::{CollectSizedArray, RaycastTarget, RayCaster, Projection, NormalAtPoint, ProjectOnAxis, GizmoRenderable};

#[derive(Debug, Clone, Copy)]
pub struct NGonData<const N: usize> {
    points:  [Vec2; N],
    normals: [Vec2; N],
    lengths: [ f32; N],
    proj_aabb: [Projection; 2],
}

impl<const N: usize> NGonData<N> {

    pub fn new(points: [Vec2; N]) -> Self {
        Self{
            normals: calculate_normals_for(&points),
            lengths: calculate_lengths_for(&points),
            proj_aabb: [ // OPT we can just min-max components
                Projection::from_points_iter(Vec2::X, points), 
                Projection::from_points_iter(Vec2::Y, points), 
            ],
            points,
        }
    }

} 

impl<const N: usize> RaycastTarget for NGonData<N> {
    fn raycast(&self, ray: &RayCaster) -> Option<Projection> {
        (0..N).filter_map(|i| ray.find_bounded_ray_intersection(self.points[i], self.normals[i].perp(), self.lengths[i])).reduce(|c, v| c.merged_with(v))
    }
}

impl<const N: usize> NormalAtPoint for NGonData<N> {
    fn normal_at(&self, point: Vec2) -> Vec2 {
        let dp: [[f32; 2]; N] = (0..N).map(|i| [
            self.normals[i].dot(point - self.points[i]),
            self.normals[i].perp_dot(point - self.points[i])
        ]).try_collect_array().unwrap();

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
                return (point - self.points[j]).normalize();
            }
        }

        self.normals[dp.iter().enumerate().max_by(|(_, [a, _]), (_, [b, _])| a.total_cmp(b)).unwrap().0]
    }
}

impl<const N: usize> ProjectOnAxis for NGonData<N> {
    fn project_aabb(&self) -> [Projection; 2] {
        self.proj_aabb
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points)
    }
}


impl<const N: usize> GizmoRenderable for NGonData<N> {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color) {
        gizmos.linestrip_2d(
            self.points.iter().map(|&v| v + offset).chain(std::iter::once(self.points[0] + offset)),
            color
        );
    }
}




struct PairIndexIter {
    current: usize,
    size: usize,
}

impl PairIndexIter {
    pub fn new(size: usize) -> Self {
        Self{current: 0, size}
    }
}

impl Iterator for PairIndexIter {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.size {
            let result = Some([self.current, (self.current + 1) % self.size]);
            self.current += 1;
            result
        } else {
            None
        }
    }
}










fn calculate_lengths_for<const N: usize>(points: &[Vec2; N]) -> [f32; N] {
    points.iter().enumerate().map(|(i, &v)| (v - points[(i+1) % N]).length()).try_collect_array().unwrap()
}

fn calculate_normals_for<const N: usize>(points: &[Vec2; N]) -> [Vec2; N] {
    points.iter().enumerate().map(|(i, &v)| (v - points[(i+1) % N]).normalize().perp()).try_collect_array().unwrap()
}

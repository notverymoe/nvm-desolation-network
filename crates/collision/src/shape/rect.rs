// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::Projection;

use super::{NearestPointTo, Project};

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {

    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self{min, max}
    }

    pub fn new_unsorted(a: Vec2, b: Vec2) -> Self {
        Self::new(a.min(b), a.max(b))
    }

    pub fn new_centered(center: Vec2, size: Vec2) -> Self {
        Self::new(center - size * 0.5, center + size + 0.5)
    }

    pub fn new_sized(origin: Vec2, size: Vec2) -> Self {
        Self::new(origin, origin + size)
    }

}

impl NearestPointTo for Rect {
    fn nearest_point_to(&self, v: Vec2) -> Vec2 {
        Vec2::new(
            if v.x <= self.min.x { self.min.x } else { self.max.x },
            if v.y <= self.min.y { self.min.y } else { self.max.y },
        )
    }
}

impl Project for Rect {
    fn offset(&self) -> Vec2 {
        self.min
    }

    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([self.min.x, self.max.x]),
            Projection([self.min.y, self.max.y]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        Projection::from_points_iter(axis, self.points())
    }

    fn with_offset(&self, o: Vec2) -> Self {
        Self{min: self.min + o, max: self.max + o}
    }
}

impl Rect {

    pub fn points(&self) -> [Vec2; 4] {
        [
            Vec2::new(self.min.x, self.min.y),
            Vec2::new(self.max.x, self.min.y),
            Vec2::new(self.max.x, self.max.y),
            Vec2::new(self.min.x, self.max.y),
        ]
    }

}

#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{shape::Project, Projection};

    use super::Rect;

    #[test]
    fn test_rect_projection() {
        let dp = (2.0_f32).sqrt();
        assert_eq!(Rect::new(Vec2::ZERO, Vec2::ONE    ).project_on_axis(Vec2::ONE.normalize()), Projection([0.0, dp    ]));
        assert_eq!(Rect::new( Vec2::ONE, Vec2::ONE*2.0).project_on_axis(Vec2::ONE.normalize()), Projection([ dp, dp*2.0]));
    }

}
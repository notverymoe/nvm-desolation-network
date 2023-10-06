// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use super::{Projection, Project};

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub origin: Vec2,
    pub radius: f32,
}

impl Circle {

    pub fn new(origin: Vec2, radius: f32) -> Self {
        Self{origin, radius}
    }

}

impl Project for Circle {
    fn project_aabb(&self) -> [Projection; 2] {
        [
            Projection([self.origin.x - self.radius, self.origin.x + self.radius]),
            Projection([self.origin.y - self.radius, self.origin.y + self.radius]),
        ]
    }

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        let origin = axis.dot(self.origin);
        Projection([origin - self.radius, origin + self.radius])
    }


    fn with_offset(&self, o: Vec2) -> Self {
        Self{origin: self.origin + o, radius: self.radius}
    }
}


#[cfg(test)]
mod test {
    use bevy::prelude::Vec2;

    use crate::{shape::Project, Projection};

    use super::Circle;

    #[test]
    fn test_circle_projection() {
        assert!(Circle::new(            Vec2::ZERO, 1.0).project_on_axis(Vec2::ONE.normalize()).is_almost(Projection([-1.0, 1.0])));
        assert!(Circle::new( Vec2::ONE.normalize(), 1.0).project_on_axis(Vec2::ONE.normalize()).is_almost(Projection([ 0.0, 2.0])));
    }

}
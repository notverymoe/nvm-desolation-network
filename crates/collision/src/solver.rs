// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{ShapeMoving, ShapeStatic, ShapeCombined, RayCaster};

pub fn test_moving_vs_static(a: &ShapeMoving, b: &ShapeStatic, raycaster: RayCaster) -> Option<[crate::RayIntersection; 2]> {
    raycaster.test(&ShapeCombined::between_moving_and_static(a, b))
}

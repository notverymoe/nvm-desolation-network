// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{ShapeData, RayCaster, Shape, Projection};

pub struct ShapeCaster {
    pub shape: ShapeData,
    pub caster: RayCaster,
}

impl ShapeCaster {

    pub fn new(shape: &Shape, direction: Vec2) -> Self {
        Self{
            shape: shape.data,
            caster: RayCaster::new(shape.origin, direction),
        }
    }

}

impl ShapeCaster {

    pub fn test_static(&self, other: &Shape) -> Option<(Projection, Shape)> {
        let target = Shape{
            origin: other.origin,
            data: other.data.combine(self.shape),
        };
        self.caster.test_static(&target).map(|v| (v, target))
    }


}
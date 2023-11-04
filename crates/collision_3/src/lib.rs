// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

mod ray_caster;
pub use ray_caster::*;

mod shape;
pub use shape::*;

pub enum RenderData {
    Circle{
        origin: Vec2,
        radius: f32,
    },
    Polygon{
        points:  Box<[Vec2]>,
        normals: Box<[Vec2]>,
    },
    RoundedPoly{
        points:  Box<[Vec2]>,
        normals: Box<[Vec2]>,
        radius:  f32,
    }
}

pub trait CollisionDebugShape {
    fn get_debug_render_data(&self) -> RenderData;
}
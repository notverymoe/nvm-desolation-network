// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{math::{IVec2, Vec2}, ecs::system::Resource};
use nvm_collide::prelude::*;

use crate::{CollisionBroadphase, CollisionCandidate};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Empty,
    Full,
    Slope,
}

#[derive(Resource)]
pub struct Map {
    width:  i32,
    height: i32,
    data:   Box<[TileKind]>,
}

impl Map {

    pub fn new(lines: &[&str]) -> Self {
        let height = lines.len();
        let width  = lines[0].len();

        let mut data = Vec::with_capacity(height*width);
        for line in lines.iter().rev() {
            assert_eq!(line.len(), width);
            for char in line.chars() {
                data.push(match char {
                    ' ' => TileKind::Empty,
                    '=' => TileKind::Full,
                    '-' => TileKind::Slope,
                    _   => panic!("Invalid character"),
                });
            }
        }

        Self{
            width:  width as i32, 
            height: height as i32, 
            data:   data.into_boxed_slice()
        }
    }

    pub fn get(&self, pos: IVec2) -> TileKind {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.width || pos.y >= self.height {
            TileKind::Empty
        } else {
            self.data[(pos.x + pos.y*self.width) as usize]
        }
    }

    pub fn get_shape(&self, pos: IVec2) -> Option<ShapeStatic> {
        match self.get(pos) {
            TileKind::Empty => None,
            TileKind::Full  => Some(BoxAligned::new(pos.as_vec2(), Vec2::ONE * 0.5).into()),
            TileKind::Slope => {
                // OPT precalc this but like, whatevs
                let x_sign = if self.get(pos - IVec2::X) != TileKind::Empty {  1.0 } else { -1.0 };
                let y_sign = if self.get(pos - IVec2::Y) != TileKind::Empty {  1.0 } else { -1.0 };
                Some(Ramp::new_from_size_centered(
                    pos.as_vec2(), 
                    Vec2::new(x_sign, y_sign), 
                    Vec2::ONE
                ).into())
            },
        }
    }

    pub fn width(&self) -> i32 {
        self.width 
    }

    pub fn height(&self) -> i32 {
        self.height 
    }

}

impl CollisionBroadphase for Map {
    fn find_candidates(&self, collider: &ShapeMoving, direction: Vec2, max_dist: f32, out: &mut tinyvec::SliceVec<CollisionCandidate>) -> usize {
        let offset = direction * max_dist;
        let [mut min, mut max] = collider.bounding_box().bounds();
        min = min.min(min + offset);
        max = max.max(max + offset);

        let x_min = min.x.floor() as i32;
        let x_max = min.x.ceil()  as i32;
        let y_min = min.y.floor() as i32;
        let y_max = max.y.ceil()  as i32;

        let size = out.len();

        for x in x_min..x_max {
            for y in y_min..y_max {
                // TODO opt check box overlap (?)
                // TODO opt DDA (?)
                if let Some(collider) = self.get_shape(IVec2::new(x, y)) {
                    out.push(CollisionCandidate{ entity: None, collider });
                }
            }
        }

        out.len() - size
    }
}
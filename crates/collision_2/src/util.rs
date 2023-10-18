// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{Gizmos, Vec2, Color};

#[macro_export]
macro_rules! assert_vec_eq {
    ($a:expr, $b:expr) => {
        assert!((($a.x - $b.x).abs() < 1e-6) && (($a.y - $b.y).abs() < 1e-6), "assertion failed: \"{:?} != {:?}\"", $a, $b);
    };
}

pub trait GizmoRenderable {
    fn render(&self, gizmos: &mut Gizmos, offset: Vec2, color: Color);
}
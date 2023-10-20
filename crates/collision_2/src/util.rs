// Copyright 2023 Natalie Baker // AGPLv3 //

use std::mem::MaybeUninit;

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

pub trait CollectSizedArray<T> {
    fn try_collect_array<const N: usize>(&mut self) -> Option<[T; N]>;
}

impl<T, I: Iterator<Item = T>> CollectSizedArray<T> for I {
    fn try_collect_array<const N: usize>(&mut self) -> Option<[T; N]> {
        let mut result: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for value in result.iter_mut() {
            value.write(self.next()?);
        }
        Some(result.map(|x| unsafe { x.assume_init() }))
    }
}

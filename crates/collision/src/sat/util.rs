// Copyright 2023 Natalie Baker // AGPLv3 //

use tinyvec::{ArrayVec, Array};

pub trait VecLike<T> {
    fn push(&mut self, v: T);
    fn extend(&mut self, v: impl IntoIterator<Item = T>);
    fn extend_from_slice(&mut self, v: &[T]);
}

impl<T: Clone> VecLike<T> for Vec<T> {
    fn push(&mut self, v: T) {
        self.push(v);
    }

    fn extend(&mut self, v: impl IntoIterator<Item = T>) {
        Extend::extend(self, v);
    }

    fn extend_from_slice(&mut self, v: &[T]) {
        Vec::<T>::extend_from_slice(self, v);
    }
}

impl<A: Array> VecLike<A::Item> for ArrayVec<A> where A::Item: Clone {
    fn push(&mut self, v: A::Item) {
        self.push(v);
    }

    fn extend(&mut self, v: impl IntoIterator<Item = A::Item>) {
        Extend::extend(self, v);
    }

    fn extend_from_slice(&mut self, v: &[A::Item]) {
        ArrayVec::<A>::extend_from_slice(self, v);
    }
}
// Copyright 2023 Natalie Baker // AGPLv3 //

use std::{slice::Iter, ops::Index};

use tinyvec::{ArrayVec, Array};

pub trait VecLike<T>: Index<usize, Output = T> {
    fn push(&mut self, v: T);
    fn extend(&mut self, v: impl IntoIterator<Item = T>);
    fn extend_from_slice(&mut self, v: &[T]);
    fn iter(&self) -> Iter<T>;
    fn clear(&mut self);
    fn as_slice(&self) -> &[T];
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

    fn iter(&self) -> Iter<T> {
        self.as_slice().iter()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn as_slice(&self) -> &[T] {
        self.as_slice()
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

    fn iter(&self) -> Iter<A::Item> {
        self.as_slice().iter()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn as_slice(&self) -> &[A::Item] {
        self.as_slice()
    }
}
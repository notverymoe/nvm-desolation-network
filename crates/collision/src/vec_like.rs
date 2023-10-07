// Copyright 2023 Natalie Baker // AGPLv3 //

use std::{slice::{Iter, IterMut}, ops::Index, marker::PhantomData};

use tinyvec::{ArrayVec, Array};



pub trait VecLike<T>: Index<usize, Output = T> {
    fn push(&mut self, v: T);
    fn extend(&mut self, v: impl IntoIterator<Item = T>);
    fn extend_from_slice(&mut self, v: &[T]);
    fn iter(&self) -> Iter<T>;
    fn iter_mut(&mut self) -> IterMut<'_, T>;
    fn clear(&mut self);
    fn as_slice(&self) -> &[T];
    fn len(&self) -> usize;
    fn truncate(&mut self, len: usize);
    fn reserve(&mut self, len: usize);
    fn is_empty(&self) -> bool;
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

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }

    fn reserve(&mut self, len: usize) {
        self.reserve(len);
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
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

    fn iter_mut(&mut self) -> IterMut<'_, A::Item> {
        self.as_mut_slice().iter_mut()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn as_slice(&self) -> &[A::Item] {
        self.as_slice()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }

    fn reserve(&mut self, _len: usize) {
        // do nothing
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct DummyVec<T>(PhantomData<T>);

impl<T> Index<usize> for DummyVec<T> {
    type Output = T;
    fn index(&self, _index: usize) -> &Self::Output {
        panic!("Attempt to index out of bounds")
    }
}

impl<T> VecLike<T> for DummyVec<T> {

    fn push(&mut self, _v: T) {
        // Do Nothing
    }

    fn extend(&mut self, _v: impl IntoIterator<Item = T>) {
        // Do Nothing
    }

    fn extend_from_slice(&mut self, _v: &[T]) {
        // Do Nothing
    }

    fn iter(&self) -> Iter<T> {
        [].iter()
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        [].iter_mut()
    }

    fn clear(&mut self) {
        // Do Nothing
    }

    fn as_slice(&self) -> &[T] {
        &[]
    }

    fn len(&self) -> usize {
        0
    }

    fn truncate(&mut self, _len: usize) {
        // Do Nothing
    }

    fn reserve(&mut self, _len: usize) {
        // Do Nothing
    }

    fn is_empty(&self) -> bool {
        true
    }
}
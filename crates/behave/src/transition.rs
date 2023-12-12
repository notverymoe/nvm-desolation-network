// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{prelude::State, newtype_str_id};

newtype_str_id!(pub Transition);

pub struct TransitionRecord<T: 'static> {
    pub id: Transition<T>,
    pub target: State<T>,
    pub sources: &'static [State<T>],
}

impl<T> From<&TransitionRecord<T>> for Transition<T> {
    fn from(value: &TransitionRecord<T>) -> Self {
        value.id
    }
}

impl<T> From<TransitionRecord<T>> for Transition<T> {
    fn from(value: TransitionRecord<T>) -> Self {
        value.id
    }
}
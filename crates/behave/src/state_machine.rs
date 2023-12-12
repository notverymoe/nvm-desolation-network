// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{prelude::Transition, newtype_str_id};

newtype_str_id!(pub State);

#[derive(Debug, Clone, Component)]
pub struct StateMachine<T: 'static> {
    last:    (State<T>, Transition<T>),
    current: State<T>,
    next:    Vec<Transition<T>>,
}

impl<T> StateMachine<T> {
    pub fn is(&self, id: State<T>) -> bool {
        self.current == id
    }

    pub fn last(&self) -> (State<T>, Transition<T>) {
        self.last
    }

    pub fn trigger(&mut self, value: impl Into<Transition<T>>) -> bool {
        let value = value.into();
        if !self.next.contains(&value) {
            self.next.push(value);
            true
        } else {
            false
        }
    }

    pub fn force_transition(&mut self, transition: impl Into<Transition<T>>, state: impl Into<State<T>>) {
        let transition = transition.into();
        self.last = (self.current, transition);
        self.current = state.into();
        self.next.clear();
    }
}

impl<T> StateMachine<T> {
    pub fn get_transitions(&self) -> &Vec<Transition<T>> {
        &self.next
    }

    pub fn current(&self) -> State<T> {
        self.current
    }
}
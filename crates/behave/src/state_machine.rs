// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::*;

use crate::{Transition, StateId};


#[derive(Debug, Clone, Copy, Component)]
pub struct StateMachine<T> {
    current: Transition<T>,
    next:    Option<Transition<T>>,
}

impl<T> StateMachine<T> {

    pub fn is(&self, id: StateId<T>) -> bool {
        self.current.target() == id
    }

    pub fn current(&self) -> StateId<T> {
        self.current.target()
    }

    pub fn set_transition(&mut self, transition: Transition<T>) -> bool {
        if transition.can_transition_from(self.current.target()) {
            self.next = Some(transition);
            true
        } else {
            false
        }
    }

    pub(crate) fn apply_transition(&mut self) -> Option<[StateId<T>; 2]> {
        if let Some(next) = std::mem::take(&mut self.next) {
            self.current = next.resolve_source(self.current.target());
            Some([self.current.source().unwrap(), self.current.target()])
        } else {
            None
        }
    }

}
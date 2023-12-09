// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{StateId, TransitionId};

pub const SOURCE_MAX: usize = 4;

#[derive(Default, Debug, Hash)]
pub struct Transition<T> {
    id:     TransitionId<T>,
    target: StateId<T>,
    source: [StateId<T>; SOURCE_MAX],
    source_len: usize,
}

impl<T> Transition<T> {

    pub const EMPTY: Transition<T> = Self{
        id: TransitionId::EMPTY,
        target: StateId::EMPTY,
        source: [StateId::EMPTY, StateId::EMPTY, StateId::EMPTY, StateId::EMPTY],
        source_len: 0,
    };

    pub const fn from_name(name: &str, target: StateId<T>, source: &[StateId<T>]) -> Self {
        Self::from_id(TransitionId::from_name(name), target, source)
    }

    pub const fn from_raw(id: u128, target: StateId<T>, source: &[StateId<T>]) -> Self {
        Self::from_id(TransitionId::from_raw(id), target, source)
    }

    pub const fn from_id(id: TransitionId<T>, target: StateId<T>, source: &[StateId<T>]) -> Self {
        Self{
            id, 
            target, 
            source: [
                if source.len() > 1 { source[0] } else { StateId::EMPTY },
                if source.len() > 2 { source[1] } else { StateId::EMPTY },
                if source.len() > 3 { source[2] } else { StateId::EMPTY },
                if source.len() > 4 { source[3] } else { StateId::EMPTY },
            ], 
            source_len: if source.len() > SOURCE_MAX { SOURCE_MAX } else { source.len() }
        }
    }
}

impl<T> Transition<T> {
    pub fn id(&self) -> TransitionId<T> {
        self.id
    }

    pub fn source(&self) -> &[StateId<T>] {
        &self.source[0..self.source_len]
    }

    pub fn target(&self) -> StateId<T> {
        self.target
    }

    pub fn can_transition_from(&self, source: StateId<T>) -> bool {
        self.source.is_empty() || self.source.contains(&source)
    }
}


impl<T> Copy for Transition<T> { }

impl<T> Clone for Transition<T> {
    fn clone(&self) -> Self {
        *self
    }
}

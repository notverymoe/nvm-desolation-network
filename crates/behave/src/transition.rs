// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{State, newtype_str_id};

pub const SOURCE_MAX: usize = 4;

newtype_str_id!(pub TransitionId);

#[derive(Default, Debug, Hash)]
pub struct Transition<T> {
    id:     TransitionId<T>,
    target: State<T>,
    source: [State<T>; SOURCE_MAX],
    source_len: usize,
}

impl<T> Transition<T> {

    pub const EMPTY: Transition<T> = Self{
        id: TransitionId::EMPTY,
        target: State::EMPTY,
        source: [State::EMPTY; SOURCE_MAX],
        source_len: 0,
    };

    pub const fn from_name(name: &str, target: State<T>, source: &[State<T>]) -> Self {
        Self::from_id(TransitionId::from_name(name), target, source)
    }

    pub const fn from_raw(id: u128, target: State<T>, source: &[State<T>]) -> Self {
        Self::from_id(TransitionId::from_raw(id), target, source)
    }

    pub const fn from_id(id: TransitionId<T>, target: State<T>, source_in: &[State<T>]) -> Self {
        let mut source = Self::EMPTY.source;
        let mut i = 0;
        loop {
            if i >= SOURCE_MAX { break; }
            if i >= source_in.len() { break; }
            source[i] = source_in[i];
            i += 1;
        }

        Self{
            id, 
            target, 
            source, 
            source_len: if source.len() > SOURCE_MAX { SOURCE_MAX } else { source.len() }
        }
    }
}

impl<T> Transition<T> {
    pub fn id(&self) -> TransitionId<T> {
        self.id
    }

    pub fn source(&self) -> &[State<T>] {
        &self.source[0..self.source_len]
    }

    pub fn target(&self) -> State<T> {
        self.target
    }

    pub fn can_transition_from(&self, source: State<T>) -> bool {
        self.source.is_empty() || self.source.contains(&source)
    }
}


impl<T> Copy for Transition<T> { }

impl<T> Clone for Transition<T> {
    fn clone(&self) -> Self {
        *self
    }
}

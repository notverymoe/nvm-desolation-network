// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{StateId, StrId, newtype_str_id};

newtype_str_id!(pub TransitionId);

#[derive(Debug, Hash)]
pub struct Transition<T: 'static> {
    id:     TransitionId<T>,
    source: &'static [StateId<T>],
    target: StateId<T>,
}

impl<T: 'static> Transition<T> {

    pub const fn new(id: &'static str, source: &'static [StateId<T>], target: StateId<T>) -> Self {
        Self{id: TransitionId::new(id), source, target}
    }

    pub fn id(&self) -> TransitionId<T> {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.id.name()
    }

    pub fn source(&self) -> &'static [StateId<T>] {
        self.source
    }

    pub fn target(&self) -> StateId<T> {
        self.target
    }

    pub fn can_transition_from(&self, source: StateId<T>) -> bool {
        self.source.is_empty() || self.source.contains(&source)
    }

}



impl<T> Copy for Transition<T> {
            
}

impl<T> Clone for Transition<T> {
    fn clone(&self) -> Self {
        Self{id: self.id, source: self.source, target: self.target}
    }
}

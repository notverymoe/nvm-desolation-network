// Copyright 2023 Natalie Baker // AGPLv3 //

use crate::{StateId, StrId, newtype_str_id};

newtype_str_id!(pub TransitionId);

#[derive(Debug, Hash)]
pub struct Transition<T> {
    id:     TransitionId<T>,
    source: Option<StateId<T>>,
    target: StateId<T>,
}

impl<T> Transition<T> {

    pub const fn new(id: &'static str, source: Option<StateId<T>>, target: StateId<T>) -> Self {
        Self{id: TransitionId::new(id), source, target}
    }

    pub fn id(&self) -> TransitionId<T> {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.id.name()
    }

    pub fn source(&self) -> Option<StateId<T>> {
        self.source
    }

    pub fn target(&self) -> StateId<T> {
        self.target
    }

    pub fn can_transition_from(&self, source: StateId<T>) -> bool {
        self.source.is_none() || self.source == Some(source)
    }

    pub fn resolve_source(&self, source: StateId<T>) -> Self {
        Self{id: self.id, source: self.source.or(Some(source)), target: self.target}
    }

}



impl<T> Copy for Transition<T> {
            
}

impl<T> Clone for Transition<T> {
    fn clone(&self) -> Self {
        Self{id: self.id, source: self.source, target: self.target}
    }
}

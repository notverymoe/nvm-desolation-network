// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, utils::{HashMap, Entry}};

use crate::prelude::{State, StateMachine, Transition};

#[derive(Debug)]
pub struct TransitionStore<T> {
    sources: Vec<State<T>>,
    target: State<T>,
}

#[derive(Debug, Resource)]
pub struct StateEngine<T> {
    transitions: HashMap<Transition<T>, TransitionStore<T>>,
    by_entered: HashMap<State<T>, Vec<Entity>>,
    by_current: HashMap<State<T>, Vec<Entity>>,
    by_leaving: HashMap<State<T>, Vec<Entity>>,
}

impl<T> Default for StateEngine<T> {
    fn default() -> Self {
        Self { 
            transitions: Default::default(),
            by_entered:  Default::default(), 
            by_current:  Default::default(), 
            by_leaving:  Default::default() 
        }
    }
}

impl<T> StateEngine<T> {
    pub fn get_entering(&self, state: State<T>) -> Option<&[Entity]> {
        self.by_entered.get(&state).map(|v| v.as_slice())
    }

    pub fn get_current(&self, state: State<T>) -> Option<&[Entity]>   {
        self.by_current.get(&state).map(|v| v.as_slice())
    }

    pub fn get_leaving(&self, state: State<T>) -> Option<&[Entity]>   {
        self.by_leaving.get(&state).map(|v| v.as_slice())
    }
}

impl<T> StateEngine<T> {
    pub fn add_transition(&mut self, id: impl Into<Transition<T>>, target: impl Into<State<T>>, sources: &[State<T>]) -> bool {
        match self.transitions.entry(id.into()) {
            Entry::Occupied(mut e) => {
                if e.get().target == target.into() {
                    e.get_mut().sources.extend_from_slice(sources);
                    true
                } else {
                    false
                }
            },
            Entry::Vacant(e) => {
                e.insert(TransitionStore{
                    target: target.into(),
                    sources: sources.to_vec(),
                });
                true
            }
        }
    }
    
    pub fn get_transition(&self, id: impl Into<Transition<T>>) -> Option<&TransitionStore<T>> {
        self.transitions.get(&id.into())
    }
}

impl<T> StateEngine<T> {
    pub fn apply_transition(&mut self, entity: Entity, state_machine: &mut StateMachine<T>) -> bool {
        let mut did_transition = false;
        for transition_id in state_machine.get_transitions().iter().copied() {
            if let Some(transition) = self.get_transition(transition_id) {
                if transition.sources.contains(&state_machine.current()) {
                    state_machine.force_transition(transition_id, transition.target);
                    self.by_leaving.entry(state_machine.last().0).or_default().push(entity);
                    self.by_entered.entry(state_machine.current()).or_default().push(entity);
                    did_transition = true;
                    break;
                }
            }
        }
        self.by_current.entry(state_machine.current()).or_default().push(entity);
        did_transition
    }

    pub fn clear(&mut self) {
        self.by_current.clear();
        self.by_leaving.clear();
        self.by_entered.clear();
    }
}
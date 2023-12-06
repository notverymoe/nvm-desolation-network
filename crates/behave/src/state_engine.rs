// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, utils::HashMap};

use crate::{StateId, StateMachine};

#[derive(Resource)]
pub struct StateEngine<T> {
    by_entered: HashMap<StateId<T>, Vec<Entity>>,
    by_current: HashMap<StateId<T>, Vec<Entity>>,
    by_leaving: HashMap<StateId<T>, Vec<Entity>>,
}

impl<T> StateEngine<T> {

    pub fn apply_transition(&mut self, entity: Entity, state_machine: &mut StateMachine<T>) -> bool {
        if let Some([leaving, entered]) = state_machine.apply_transition() {
            self.by_leaving.entry(leaving).or_default().push(entity);
            self.by_entered.entry(entered).or_default().push(entity);
            self.by_current.entry(entered).or_default().push(entity);
            true
        } else {
            self.by_current.entry(state_machine.current()).or_default().push(entity);
            false
        }
    }

}
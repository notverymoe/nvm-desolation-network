// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, utils::HashMap, ecs::schedule::ScheduleLabel};

use crate::prelude::{State, StateMachine};

#[derive(Debug, Resource)]
pub struct StateEngine<T> {
    pub(crate) label: Box<dyn ScheduleLabel>,
    by_entered: HashMap<State<T>, Vec<Entity>>,
    by_current: HashMap<State<T>, Vec<Entity>>,
    by_leaving: HashMap<State<T>, Vec<Entity>>,
}

impl<T> StateEngine<T> {

    pub(crate) fn new(label: Box<dyn ScheduleLabel>) -> Self {
        Self{
            label,
            by_current: Default::default(),
            by_entered: Default::default(),
            by_leaving: Default::default(),
        }
    }

    pub(crate) fn is_in_label(&self, label: &impl ScheduleLabel) -> bool {
        self.label.as_dyn_eq().dyn_eq(label.as_dyn_eq())
    }

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

    pub fn clear(&mut self) {
        self.by_current.clear();
        self.by_leaving.clear();
        self.by_entered.clear();
    }
}
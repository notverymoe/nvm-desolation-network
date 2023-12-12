// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

use crate::prelude::{StateMachine, StateEngine, TransitionRecord, Transition, State};

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StateMachineUpdate {
    Process,
    OnLeave,
    OnEnter,
    OnUpdate,
}

pub trait AppAddStateEngine {
    fn add_state_engine<T: 'static>(&mut self, schedule: impl ScheduleLabel + Clone) -> &mut Self;
    fn add_state_transitions<T: 'static>(&mut self, transitions: &[&TransitionRecord<T>]) -> &mut Self;
    fn add_state_transition<T: 'static>(&mut self, id: Transition<T>, target: State<T>, sources: &[State<T>]) -> &mut Self;
}

impl AppAddStateEngine for App {
    fn add_state_engine<T: 'static>(&mut self, schedule: impl ScheduleLabel + Clone) -> &mut Self {
        self.init_resource::<StateEngine<T>>();
        self.configure_sets(
            schedule.clone(),
            (
                StateMachineUpdate::Process,
                StateMachineUpdate::OnLeave,
                StateMachineUpdate::OnEnter,
                StateMachineUpdate::OnUpdate,
            ).chain()
        );
        self.add_systems(schedule, system_apply_state_transitions::<T>.in_set(StateMachineUpdate::Process));
        self
    }

    fn add_state_transitions<T: 'static>(&mut self, transitions: &[&TransitionRecord<T>]) -> &mut Self {
        for transition in transitions {
            self.add_state_transition(transition.id, transition.target, transition.sources);
        }
        self
    }

    fn add_state_transition<T: 'static>(&mut self, id: Transition<T>, target: State<T>, sources: &[State<T>]) -> &mut Self {
        self.init_resource::<StateEngine<T>>();
        let mut engine = self.world.resource_mut::<StateEngine<T>>();
        assert!(engine.add_transition(id, target, sources));
        self
    }
}

pub fn system_apply_state_transitions<T>(
    mut query: Query<(Entity, &mut StateMachine<T>)>,
    mut engine: ResMut<StateEngine<T>>,
) {
    let engine = &mut *engine;
    engine.clear();
    for (entity, mut state_machine) in query.iter_mut() {
        engine.apply_transition(entity, &mut *state_machine);
    }
}
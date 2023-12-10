// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, ecs::schedule::ScheduleLabel};

use crate::prelude::{StateMachine, StateEngine};

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StateMachineUpdate {
    Process,
    OnLeave,
    OnEnter,
    OnUpdate,
}

pub trait AppAddStateEngine {
    fn add_state_engine<T: 'static>(&mut self, schedule: impl ScheduleLabel + Clone);
}

impl AppAddStateEngine for App {
    fn add_state_engine<T: 'static>(&mut self, schedule: impl ScheduleLabel + Clone) {

        if let Some(resource) = self.world.get_resource::<StateEngine<T>>() {
            assert!(resource.is_in_label(&schedule), "State machine already registered, but in different label. Expected {:?} got {:?}", schedule, resource.label);
        } else {
            self.insert_resource(StateEngine::<T>::new(schedule.dyn_clone()));
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

        }
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
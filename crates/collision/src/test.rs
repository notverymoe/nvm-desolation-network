// Copyright 2023 Natalie Baker // AGPLv3 //

// pub fn test_sweep_vs_sweep(sweep_a: &Sweep, shape_a: &Shape, sweep_b: &Sweep, shape_b: &Shape);
// pub fn test_sweep_vs_static(sweep_a: &Sweep, shape_a: &Shape, shape_b: &Shape);

use bevy::prelude::Vec2;

use crate::{shape::{Shape, Project}, Contact, VecLike, find_candidates_between, CandidateAxes, CANDIDATE_AXES_SIZE};

pub fn test_static_vs_static<const TEST_ALL: bool>(shape_a: &Shape, shape_b: &Shape, dest: &mut impl VecLike<Contact>) -> bool {
    dest.reserve(CANDIDATE_AXES_SIZE + 2);

    let [aabb_x_a, aabb_y_a] = shape_a.project_aabb();
    let [aabb_x_b, aabb_y_b] = shape_b.project_aabb();

    let contact = Contact::from_overlap(Vec2::X, aabb_x_a, aabb_x_b);
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let contact = Contact::from_overlap(Vec2::Y, aabb_y_a, aabb_y_b);
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let mut axes: CandidateAxes = Default::default();
    find_candidates_between(shape_a, shape_b, &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, shape_a, shape_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    true
}

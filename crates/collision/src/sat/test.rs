// Copyright 2023 Natalie Baker // AGPLv3 //

// pub fn test_sweep_vs_sweep(sweep_a: &Sweep, shape_a: &Shape, sweep_b: &Sweep, shape_b: &Shape);
// pub fn test_sweep_vs_static(sweep_a: &Sweep, shape_a: &Shape, shape_b: &Shape);

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

use crate::{shape::{Shape, Project}, Contact, find_candidates_between};

pub fn test_static_vs_static(shape_a: &Shape, shape_b: &Shape) -> (bool, ArrayVec<[Contact; 9]>) {
    let mut result = ArrayVec::<[Contact; 9]>::new();
    
    let [aabb_x_a, aabb_y_a] = shape_a.project_aabb();
    let [aabb_x_b, aabb_y_b] = shape_b.project_aabb();

    let contact = Contact::from_overlap(Vec2::X, aabb_x_a, aabb_x_b);
    if !contact.is_penetration() { return (false, result); }
    result.push(contact);

    let contact = Contact::from_overlap(Vec2::Y, aabb_y_a, aabb_y_b);
    if !contact.is_penetration() { return (false, result); }
    result.push(contact);

    for contact in find_candidates_between(shape_a, shape_b).into_iter().map(|a| Contact::from_projections(a, shape_a, shape_b)) {
        if !contact.is_penetration() { return (false, result); }
        result.push(contact);
    }

    (true, result)
}

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;

use crate::{shape::{Shape, Project}, Contact, VecLike, find_candidates_between, CandidateAxes, CANDIDATE_AXES_SIZE, Sweep, find_dynamic_candidates, Contacts};


pub struct SolverSweep {          
    pub target: Sweep,
    pub contacts: Contacts,
}

impl SolverSweep {

    pub fn test_sweep_pen(&mut self, b: &Sweep) -> bool {
        self.test_sweep::<false>(b)
    }

    pub fn test_sweep_all(&mut self, b: &Sweep) {
        self.test_sweep::<true>(b);
    }

    pub fn test_static_pen(&mut self, b: &Shape) -> bool {
        self.test_static::<false>(b)
    }

    pub fn test_static_all(&mut self, b: &Shape) {
        self.test_static::<true>(b);
    }

}

impl SolverSweep {

    fn test_sweep<const TEST_ALL: bool>(&mut self, b: &Sweep) -> bool {
        test_sweep_vs_sweep::<TEST_ALL>(&self.target, b, &mut *self.contacts)
    }

    fn test_static<const TEST_ALL: bool>(&mut self, b: &Shape) -> bool {
        test_sweep_vs_static::<TEST_ALL>(&self.target, b, &mut *self.contacts)
    }

}

pub struct SolverStatic {
    pub target: Shape,
    pub contacts: Contacts,
}

impl SolverStatic {

    pub fn test_sweep_pen(&mut self, b: &Sweep) -> bool {
        self.test_sweep::<false>(b)
    }

    pub fn test_sweep_all(&mut self, b: &Sweep) {
        self.test_sweep::<true>(b);
    }

    pub fn test_static_pen(&mut self, b: &Shape) -> bool {
        self.test_static::<false>(b)
    }

    pub fn test_static_all(&mut self, b: &Shape) {
        self.test_static::<true>(b);
    }

}

impl SolverStatic {

    fn test_sweep<const TEST_ALL: bool>(&mut self, b: &Sweep) -> bool {
        let contact_len = self.contacts.len();
        let result = test_sweep_vs_static::<TEST_ALL>(b, &self.target, &mut *self.contacts);
        for contact in self.contacts.iter_mut().skip(contact_len) {
            contact.reverse();
        }
        result
    }

    fn test_static<const TEST_ALL: bool>(&mut self, b: &Shape) -> bool {
        test_static_vs_static::<TEST_ALL>(&self.target, b, &mut *self.contacts)
    }

}

pub fn test_sweep_vs_sweep<const TEST_ALL: bool>(sweep_a: &Sweep, sweep_b: &Sweep, dest: &mut impl VecLike<Contact>) -> bool {
    dest.reserve(4*CANDIDATE_AXES_SIZE + 4);

    let [aabb_x_a, aabb_y_a] = sweep_a.project_aabb();
    let [aabb_x_b, aabb_y_b] = sweep_b.project_aabb();

    let contact = Contact::from_overlap(Vec2::X, aabb_x_a, aabb_x_b);
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let contact = Contact::from_overlap(Vec2::Y, aabb_y_a, aabb_y_b);
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let contact = Contact::from_overlap(sweep_a.test_axis(), sweep_a.test_cache(), sweep_b.project_on_axis(sweep_a.test_axis()));
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let contact = Contact::from_overlap(sweep_b.test_axis(), sweep_a.project_on_axis(sweep_a.test_axis()), sweep_b.test_cache());
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let mut axes: CandidateAxes = Default::default();
    find_candidates_between(&sweep_a.start(), &sweep_b.start(), &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, sweep_a, sweep_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    find_dynamic_candidates(&sweep_a.end(), &sweep_b.start(), &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, sweep_a, sweep_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    find_dynamic_candidates(&sweep_a.start(), &sweep_b.end(), &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, sweep_a, sweep_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    find_dynamic_candidates(&sweep_a.end(), &sweep_b.end(), &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, sweep_a, sweep_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    true
}

pub fn test_sweep_vs_static<const TEST_ALL: bool>(sweep_a: &Sweep, shape_b: &Shape, dest: &mut impl VecLike<Contact>) -> bool {
    dest.reserve(2*CANDIDATE_AXES_SIZE + 3);

    let [aabb_x_a, aabb_y_a] = sweep_a.project_aabb();
    let [aabb_x_b, aabb_y_b] = shape_b.project_aabb();

    let contact = Contact::from_overlap(Vec2::X, aabb_x_a, aabb_x_b);
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let contact = Contact::from_overlap(Vec2::Y, aabb_y_a, aabb_y_b);
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let contact = Contact::from_overlap(sweep_a.test_axis(), sweep_a.test_cache(), shape_b.project_on_axis(sweep_a.test_axis()));
    if TEST_ALL && !contact.is_penetration() { return false; }
    dest.push(contact);

    let mut axes: CandidateAxes = Default::default();
    find_candidates_between(&sweep_a.start(), shape_b, &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, sweep_a, shape_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    find_dynamic_candidates(&sweep_a.end(), shape_b, &mut axes);
    for contact in axes.into_iter().map(|a| Contact::from_projections(a, sweep_a, shape_b)) {
        if TEST_ALL && !contact.is_penetration() { return false; }
        dest.push(contact);
    }

    true
}

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

// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

use crate::{Contact, Projection, VecLike, Shape};

// Swept slope/capsule vs self = 8 axes
// Swept rect                  = 8 points
const SOLVER_BUFFER_CAP: usize = 8;

pub const CONTACT_ALL:       usize = 0;
pub const CONTACT_PENETRATE: usize = 1;
pub const CONTACT_SEPERATE:  usize = 2;

pub struct Solver {
    pub contacts: Vec<Contact>,
    buffer_points:      ArrayVec<[      Vec2; SOLVER_BUFFER_CAP]>,
    buffer_axes:        ArrayVec<[      Vec2; SOLVER_BUFFER_CAP]>,
    buffer_projections: ArrayVec<[Projection; SOLVER_BUFFER_CAP]>,
}

impl Solver {

    pub fn find_min(&self) -> Option<&Contact> {
        self.contacts.iter().reduce(|p, c| if p.contact_min.abs() < c.contact_min.abs() { p } else { c })
    }

    pub fn add_contacts<const CONSTRAINT: usize, const REQUIRED: bool>(&mut self, a: &impl Shape, b: &impl Shape) -> bool {
        generate_contacts::<CONSTRAINT, REQUIRED>(a, b, &mut self.buffer_points, &mut self.buffer_axes, &mut self.buffer_projections, &mut self.contacts)
    }

}

pub fn generate_contacts<const CONSTRAINT: usize, const REQUIRED: bool>(
    a: &impl Shape, 
    b: &impl Shape,
    buffer_points:      &mut impl VecLike<Vec2      >,
    buffer_axes:        &mut impl VecLike<Vec2      >,
    buffer_projections: &mut impl VecLike<Projection>,
    out_contacts:       &mut impl VecLike<Contact   >,
) -> bool {
    let orig_len = out_contacts.len();
    let mut try_add_contact = |contact: Contact| -> bool {
        if CONSTRAINT != CONTACT_ALL && !contact.is_overlaped() { 
            if REQUIRED { 
                out_contacts.shrink_to(orig_len);
                return false; 
            }
        } else {
            out_contacts.push(contact);
        }
        true
    };

    // STATIC A 
    buffer_axes.clear();
    buffer_projections.clear();
    a.get_axes(buffer_axes, buffer_projections);
    for (i, &axis) in buffer_axes.iter().enumerate() {
        let contact = Contact::from_overlap(axis, buffer_projections[i], b.project_on_axis(axis));
        if !try_add_contact(contact) { return CONSTRAINT == CONTACT_PENETRATE; }
    }

    // STATIC B
    buffer_axes.clear();
    buffer_projections.clear();
    b.get_axes(buffer_axes, buffer_projections);
    for (i, axis) in buffer_axes.iter().map(|&v| -v).enumerate() {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), buffer_projections[i].reversed());
        if !try_add_contact(contact) { return CONSTRAINT == CONTACT_PENETRATE; }
    }

    // DERIVED A 
    buffer_axes.clear();
    buffer_points.clear();
    buffer_projections.clear();
    b.get_points(buffer_points);
    a.get_axes_derived(buffer_points.as_slice(), buffer_axes);
    for &axis in buffer_axes.iter() {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis));
        if !try_add_contact(contact) { return CONSTRAINT == CONTACT_PENETRATE; }
    }

    // DERIVED B
    buffer_axes.clear();
    buffer_points.clear();
    buffer_projections.clear();
    a.get_points(buffer_points);
    b.get_axes_derived(buffer_points.as_slice(), buffer_axes);
    for axis in buffer_axes.iter().map(|&v| -v) {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis));
        if !try_add_contact(contact) { return CONSTRAINT == CONTACT_PENETRATE; }
    }

    CONSTRAINT != CONTACT_PENETRATE
}


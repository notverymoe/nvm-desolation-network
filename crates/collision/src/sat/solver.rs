// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

use crate::{Contact, Projection, VecLike, Shape};

// Swept slope/capsule vs self = 8 axes
// Swept rect                  = 8 points
const SOLVER_BUFFER_CAP: usize = 8;

pub const CONTACTS_ALL:           usize = 0;
pub const CONTACTS_PENETRATE:     usize = 1;
pub const CONTACTS_SEPERATE:      usize = 2;
pub const CONTACTS_PENETRATE_REQ: usize = 3;

#[derive(Default)]
pub struct Solver {
    pub contacts: Vec<Contact>,
    buffer_points:      ArrayVec<[      Vec2; SOLVER_BUFFER_CAP]>,
    buffer_axes:        ArrayVec<[      Vec2; SOLVER_BUFFER_CAP]>,
    buffer_projections: ArrayVec<[Projection; SOLVER_BUFFER_CAP]>,
}

impl Solver {

    pub fn find_min(&self) -> Option<&Contact> {
        self.contacts.iter().reduce(|p, c| if p.contact_min.abs() <= c.contact_min.abs() { p } else { c })
    }

    pub fn add_contacts<const CONSTRAINT: usize>(&mut self, a: &impl Shape, b: &impl Shape) -> bool {
        generate_contacts::<CONSTRAINT>(a, b, &mut self.buffer_points, &mut self.buffer_axes, &mut self.buffer_projections, &mut self.contacts)
    }

}

pub fn generate_contacts<const CONSTRAINT: usize>(
    a: &impl Shape, 
    b: &impl Shape,
    buffer_points:      &mut impl VecLike<Vec2      >,
    buffer_axes:        &mut impl VecLike<Vec2      >,
    buffer_projections: &mut impl VecLike<Projection>,
    out_contacts:       &mut impl VecLike<Contact   >,
) -> bool {
    let orig_len = out_contacts.len();
    let mut is_overlapping = true;
    let mut try_add_contact = |contact: Contact| -> bool {
        let is_contact_pen = contact.is_penetration();
        is_overlapping &= is_contact_pen;
        let add_contact = match (CONSTRAINT, is_contact_pen) {
            (CONTACTS_PENETRATE,     false) => false,
            (CONTACTS_SEPERATE,       true) => false,
            (CONTACTS_PENETRATE_REQ, false) => {
                out_contacts.truncate(orig_len); // Clear penetrations
                return false; // early-out exit
            },
            _ => true,
        };
        
        if add_contact {
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
        if !try_add_contact(contact) { return false; }
    }

    // STATIC B
    buffer_axes.clear();
    buffer_projections.clear();
    b.get_axes(buffer_axes, buffer_projections);
    for (i, axis) in buffer_axes.iter().map(|&v| -v).enumerate() {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), buffer_projections[i].reversed());
        if !try_add_contact(contact) { return false; }
    }

    // DERIVED A 
    buffer_axes.clear();
    buffer_points.clear();
    buffer_projections.clear();
    b.get_points(buffer_points);
    a.get_axes_derived(buffer_points.as_slice(), buffer_axes);
    for &axis in buffer_axes.iter() {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis));
        if !try_add_contact(contact) { return false; }
    }

    // DERIVED B
    buffer_axes.clear();
    buffer_points.clear();
    buffer_projections.clear();
    a.get_points(buffer_points);
    b.get_axes_derived(buffer_points.as_slice(), buffer_axes);
    for axis in buffer_axes.iter().map(|&v| -v) {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis));
        if !try_add_contact(contact) { return false; }
    }

    is_overlapping
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;

    use crate::{shape::Rect, Solver, CONTACTS_PENETRATE, CONTACTS_SEPERATE, CONTACTS_PENETRATE_REQ};

    #[test]
    fn test_solver_seperate() {
        let a = Rect{min: 1.0*Vec2::ONE, max: 2.0*Vec2::ONE};
        let b = Rect{min: 3.0*Vec2::ONE, max: 4.0*Vec2::ONE};
        let mut solver = Solver::default();
        assert!(!solver.add_contacts::<CONTACTS_SEPERATE>(&a, &b), "Rects falsely penetrate");
        assert_eq!(solver.contacts.len(), 4, "Solver generated wrong contact count");
        if let Some(contact) = solver.find_min().copied() {
            assert_eq!(contact.axis,        Vec2::X);
            assert_eq!(contact.contact_min, 1.0);
            assert_eq!(contact.contact_max, 3.0);
        } else {
            panic!("No contacts in solver");
        }

    }

    #[test]
    fn test_solver_overlap() {
        let a = Rect{min: 2.5*Vec2::ONE, max: 3.5*Vec2::ONE};
        let b = Rect{min: 3.0*Vec2::ONE, max: 4.0*Vec2::ONE};
        let mut solver = Solver::default();
        assert!(solver.add_contacts::<CONTACTS_PENETRATE>(&a, &b), "Rects falsely seperate");
        assert_eq!(solver.contacts.len(), 4, "Solver generated wrong contact count");
        if let Some(contact) = solver.find_min().copied() {
            assert_eq!(contact.axis,        Vec2::X);
            assert_eq!(contact.contact_min, -0.5);
            assert_eq!(contact.contact_max,  1.5);
        } else {
            panic!("No contacts in solver");
        }

    }

    #[test]
    fn test_solver_early_out() {
        let a = Rect{min: Vec2::new(0.0, 0.0), max: Vec2::new(10.0, 1.0)};
        let b = Rect{min: Vec2::new(2.0, 2.0), max: Vec2::new( 8.0, 2.0)};
        let mut solver = Solver::default();
        assert!(!solver.add_contacts::<CONTACTS_PENETRATE_REQ>(&a, &b), "Rects falsely penetrate");
        assert_eq!(solver.contacts.len(), 0, "Solver falsely generated contacts");
    }

}
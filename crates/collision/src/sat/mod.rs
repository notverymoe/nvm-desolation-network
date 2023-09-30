// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

mod sat_shape;
pub use sat_shape::*;

mod projection;
pub use projection::*;

mod sweep;
pub use sweep::*;

mod cache;
pub use cache::*;

mod util;
pub use util::*;

mod contact;
pub use contact::*;

mod solver;
pub use solver::*;

pub const SAT_BUFFER_CAP: usize = 8;

pub const CONTACT_ALL:      usize = 0;
pub const CONTACT_OVERLAP:  usize = 1;
pub const CONTACT_SEPERATE: usize = 2;

pub fn generate_contacts_all(
    a: &impl SATShape, 
    b: &impl SATShape,
    buffer_points:      &mut impl VecLike<Vec2      >,
    buffer_axes:        &mut impl VecLike<Vec2      >,
    buffer_projections: &mut impl VecLike<Projection>,
    out_contacts:       &mut impl VecLike<Contact>,
) -> bool { 
    generate_contacts::<CONTACT_ALL>(a, b, buffer_points, buffer_axes, buffer_projections, out_contacts)
}

pub fn generate_contacts_overlap(
    a: &impl SATShape, 
    b: &impl SATShape,
    buffer_points:      &mut impl VecLike<Vec2      >,
    buffer_axes:        &mut impl VecLike<Vec2      >,
    buffer_projections: &mut impl VecLike<Projection>,
    out_contacts:       &mut impl VecLike<Contact>,
) -> bool { 
    generate_contacts::<CONTACT_OVERLAP>(a, b, buffer_points, buffer_axes, buffer_projections, out_contacts)
}

pub fn generate_contacts_serperated(
    a: &impl SATShape, 
    b: &impl SATShape,
    buffer_points:      &mut impl VecLike<Vec2      >,
    buffer_axes:        &mut impl VecLike<Vec2      >,
    buffer_projections: &mut impl VecLike<Projection>,
    out_contacts:       &mut impl VecLike<Contact>,
) -> bool { 
    generate_contacts::<CONTACT_SEPERATE>(a, b, buffer_points, buffer_axes, buffer_projections, out_contacts)
}

pub fn generate_contacts<const CONSTRAINT: usize>(
    a: &impl SATShape, 
    b: &impl SATShape,
    buffer_points:      &mut impl VecLike<Vec2      >,
    buffer_axes:        &mut impl VecLike<Vec2      >,
    buffer_projections: &mut impl VecLike<Projection>,
    out_contacts:       &mut impl VecLike<Contact>,
) -> bool {
    let mut contacts: ArrayVec<[Contact; SAT_BUFFER_CAP]> = ArrayVec::new();

    // STATIC A 
    a.get_axes(buffer_axes, buffer_projections);
    for (i, &axis) in buffer_axes.iter().enumerate() {
        let contact = Contact::from_overlap(axis, buffer_projections[i], b.project_on_axis(axis));
        match CONSTRAINT {
            CONTACT_OVERLAP  => if !contact.is_overlaped() { return false; }
            CONTACT_SEPERATE => if  contact.is_overlaped() { return false; }
            _ => {},
        }
        contacts.push(contact);
    }

    // STATIC B
    buffer_axes.clear();
    buffer_projections.clear();
    b.get_axes(buffer_axes, buffer_projections);
    for (i, axis) in buffer_axes.iter().map(|&v| -v).enumerate() {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), buffer_projections[i].reversed());
        match CONSTRAINT {
            CONTACT_OVERLAP  => if !contact.is_overlaped() { return false; }
            CONTACT_SEPERATE => if  contact.is_overlaped() { return false; }
            _ => {},
        }
        contacts.push(contact);
    }

    // DERIVED A 
    buffer_axes.clear();
    buffer_points.clear();
    buffer_projections.clear();
    b.get_points(buffer_points);
    a.get_axes_derived(buffer_points.as_slice(), buffer_axes);
    for &axis in buffer_axes.iter() {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis));
        match CONSTRAINT {
            CONTACT_OVERLAP  => if !contact.is_overlaped() { return false; }
            CONTACT_SEPERATE => if  contact.is_overlaped() { return false; }
            _ => {},
        }
        contacts.push(contact);
    }

    // DERIVED B
    buffer_axes.clear();
    buffer_points.clear();
    buffer_projections.clear();
    a.get_points(buffer_points);
    b.get_axes_derived(buffer_points.as_slice(), buffer_axes);
    for axis in buffer_axes.iter().map(|&v| -v) {
        let contact = Contact::from_overlap(axis, a.project_on_axis(axis), b.project_on_axis(axis));
        match CONSTRAINT {
            CONTACT_OVERLAP  => if !contact.is_overlaped() { return false; }
            CONTACT_SEPERATE => if  contact.is_overlaped() { return false; }
            _ => {},
        }
        contacts.push(contact);
    }

    out_contacts.extend_from_slice(&contacts);
    true
}


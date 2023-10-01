// Copyright 2023 Natalie Baker // AGPLv3 //

mod shape;
pub use shape::*;

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

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;

    use crate::{shape::Rect, Solver, CONTACTS_PENETRATE, CONTACTS_SEPERATE, CONTACTS_PENETRATE_REQ};

    #[test]
    fn test_rect_seperate() {
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
    fn test_rect_overlap() {
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
    fn test_rect_early_out() {
        let a = Rect{min: Vec2::new(0.0, 0.0), max: Vec2::new(10.0, 1.0)};
        let b = Rect{min: Vec2::new(2.0, 2.0), max: Vec2::new( 8.0, 2.0)};
        let mut solver = Solver::default();
        assert!(!solver.add_contacts::<CONTACTS_PENETRATE_REQ>(&a, &b), "Rects falsely penetrate");
        assert_eq!(solver.contacts.len(), 0, "Solver falsely generated contacts");
    }

}
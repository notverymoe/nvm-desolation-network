// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::{DerefMut, Deref, Vec2};

use crate::Contact;

#[derive(DerefMut, Deref)]
pub struct Contacts(Vec<Contact>);

impl Contacts {

    pub fn find_min_contact(&self) -> Option<&Contact> {
        self.iter().reduce(|p, c| if c.contact_min.abs() < p.contact_min.abs() { c } else { p })
    }

    pub fn find_min_contact_along_axis(&self, axis: Vec2) -> Option<(Contact, f32)> {
        if self.is_empty() { return None; }
        Some(self.iter().fold((Contact::default(), 0.0), |p: (Contact, f32), c| {
            let contact_on_axis = axis.dot(c.axis * c.contact_min);
            if contact_on_axis.abs() < p.1.abs() {
                (*c, contact_on_axis)
            } else {
                p
            }
        }))
    } 

}
// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use crate::{Shape, VecLike, Contact};

pub struct Sweep<T: Shape + Copy> {
    motion: Vec2,
    start: T,
    end:   T,
}

impl<T: Shape + Copy> Sweep<T> {
    pub fn new(shape: T, motion: Vec2) -> Self {
        Self{motion, start: shape, end: shape.with_offset(motion)}
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }

    pub fn motion(&self) -> &Vec2 {
        &self.motion
    }

    pub fn get_contact_along_motion(&self, contact: Contact) -> Contact {
        contact.reproject(self.motion.normalize())
    }
}

impl<T: Shape + Copy> Shape for Sweep<T> {
    const CAN_SMEAR_PROJECTION: bool = T::CAN_SMEAR_PROJECTION;

    fn project_on_axis(&self, axis: Vec2) -> crate::Projection {
        let result = self.start.project_on_axis(axis);
        if T::CAN_SMEAR_PROJECTION {
            result.smeared_by(axis.dot(self.motion))
        } else {
            result.merged_with(self.end.project_on_axis(axis))
        }
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        self.start.get_points(out_points);
        self.end.get_points(out_points);
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<crate::Projection>) {
        let from = out_projections.len();
        self.start.get_axes(out_axes, out_projections);
        for (i, projection) in out_projections.iter_mut().enumerate().skip(from) {
            let smear = out_axes[i].dot(self.motion);
            projection.smear(smear); // I think smear is safe here?
        }
        out_axes.push(self.motion.perp().normalize());
        out_projections.push(self.project_on_axis(self.motion.perp().normalize()));
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut impl VecLike<Vec2>) {
        if !T::CAN_SMEAR_PROJECTION {
            self.start.get_axes_derived(other, out_axes);
            self.end.get_axes_derived(other, out_axes);
        }
        // Smearable projections can't have derived axes, no-op
    }

    fn with_offset(self, offset: Vec2) -> Self {
        Self { 
            motion: self.motion, 
            start:  self.start.with_offset(offset), 
            end:    self.end.with_offset(offset) 
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec2;

    use crate::{Sweep, Shape, shape::{Rect, Circle, Capsule}, Projection};

    #[test]
    fn test_sweep_point() {
        let sweep = Sweep::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
        assert_eq!(sweep.project_on_axis(Vec2::X), Projection([0.0, 10.0]), "Incorrect sweep projection of point on X");
        assert_eq!(sweep.project_on_axis(Vec2::Y), Projection([0.0,  0.0]), "Incorrect sweep projection of point on Y");

        let mut axes = Vec::new();
        let mut proj = Vec::new();
        sweep.get_axes(&mut axes, &mut proj);
        assert_eq!(axes.as_slice(), [Vec2::X, Vec2::Y, Vec2::Y], "Invalid sweep axes returned");
        assert_eq!(proj.as_slice(), [Projection([0.0, 10.0]), Projection([0.0, 0.0]), Projection([0.0, 0.0])], "Invalid sweep projection returned");
    }

    #[test]
    fn test_sweep_rect() {
        let sweep = Sweep::new(Rect{min: Vec2::ZERO, max: Vec2::ONE}, Vec2::new(10.0, 0.0));
        assert_eq!(sweep.project_on_axis(Vec2::X), Projection([0.0, 11.0]), "Incorrect sweep projection of point on X");
        assert_eq!(sweep.project_on_axis(Vec2::Y), Projection([0.0,  1.0]), "Incorrect sweep projection of point on Y");

        let mut axes = Vec::new();
        let mut proj = Vec::new();
        sweep.get_axes(&mut axes, &mut proj);
        assert_eq!(axes.as_slice(), [Vec2::X, Vec2::Y, Vec2::Y], "Invalid sweep axes returned");
        assert_eq!(proj.as_slice(), [Projection([0.0, 11.0]), Projection([0.0, 1.0]), Projection([0.0, 1.0])], "Invalid sweep projection returned");
    }

    #[test]
    fn test_sweep_circle() {
        let sweep = Sweep::new(Circle{origin: Vec2::ONE * 0.5, radius: 0.5}, Vec2::new(10.0, 0.0));
        assert_eq!(sweep.project_on_axis(Vec2::X), Projection([0.0, 11.0]), "Incorrect sweep projection of point on X");
        assert_eq!(sweep.project_on_axis(Vec2::Y), Projection([0.0,  1.0]), "Incorrect sweep projection of point on Y");

        let mut axes = Vec::new();
        let test_point = Vec2::new(5.0, 1.0);
        sweep.get_axes_derived(&[test_point], &mut axes);
        assert_eq!(axes.as_slice(), [(test_point - sweep.start.origin).normalize(), (test_point - sweep.end.origin).normalize()], "Invalid sweep axes returned");
    }

    #[test]
    fn test_sweep_capsule() {
        let sweep = Sweep::new(Capsule{origin: Vec2::ONE * 0.5,  height: 1.0, radius: 0.5}, Vec2::new(10.0, 0.0));
        assert_eq!(sweep.project_on_axis(Vec2::X), Projection([0.0, 11.0]), "Incorrect sweep projection of point on X");
        assert_eq!(sweep.project_on_axis(Vec2::Y), Projection([0.0,  2.0]), "Incorrect sweep projection of point on Y");

        let mut axes = Vec::new();
        let mut proj = Vec::new();
        sweep.get_axes(&mut axes, &mut proj);
        assert_eq!(axes.as_slice(), [Vec2::X, Vec2::Y], "Invalid sweep axes returned");
        assert_eq!(proj.as_slice(), [Projection([0.0, 11.0]), Projection([0.0, 2.0])], "Invalid sweep projection returned");

        let mut axes = Vec::new();
        let test_point = Vec2::new(5.0, 10.0);
        sweep.get_axes_derived(&[test_point], &mut axes);
        assert_eq!(axes.as_slice(), [
            (test_point - sweep.start.origin).normalize(), 
            (test_point - (sweep.start.origin + Vec2::Y*sweep.start.height)).normalize(), 
            (test_point - sweep.end.origin).normalize(), 
            (test_point - (sweep.end.origin + Vec2::Y*sweep.start.height)).normalize(), 
        ], "Invalid sweep axes returned");
    }

}
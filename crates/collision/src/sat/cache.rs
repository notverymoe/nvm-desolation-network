// Copyright 2023 Natalie Baker // AGPLv3 //

use bevy::prelude::Vec2;
use tinyvec::ArrayVec;

use crate::{SATShape, Projection, VecLike};

// Slope has 3 axes + 1 for sweep
pub const CACHE_DEFAULT_CAP: usize = 4;

#[derive(Clone, Copy)]
pub struct Cache<T: SATShape, const CAP: usize = CACHE_DEFAULT_CAP> 
where 
    [Vec2;       CAP] : tinyvec::Array<Item = Vec2      > + Copy,
    [Projection; CAP] : tinyvec::Array<Item = Projection> + Copy,
{
    pub shape:       T,
    pub axes:        ArrayVec<[Vec2;       CAP]>,
    pub projections: ArrayVec<[Projection; CAP]>,
}

impl<T: SATShape, const CAP: usize> Cache<T, CAP> 
where 
    [Vec2;       CAP] : tinyvec::Array<Item = Vec2      > + Copy,
    [Projection; CAP] : tinyvec::Array<Item = Projection> + Copy,
{

    pub fn new(shape: T) -> Self {
        let mut axes  = ArrayVec::new();
        let mut projections = ArrayVec::new();
        shape.get_axes(&mut axes, &mut projections);
        Self{ shape, axes, projections }
    }

}

impl<T: SATShape, const CAP: usize> SATShape for Cache<T, CAP> 
where 
    [Vec2;       CAP] : tinyvec::Array<Item = Vec2      > + Copy,
    [Projection; CAP] : tinyvec::Array<Item = Projection> + Copy,
{
    const CAN_SMEAR_PROJECTION: bool = T::CAN_SMEAR_PROJECTION;

    fn project_on_axis(&self, axis: Vec2) -> Projection {
        self.shape.project_on_axis(axis)
    }

    fn get_points(&self, out_points: &mut impl VecLike<Vec2>) {
        self.shape.get_points(out_points)
    }

    fn get_axes(&self, out_axes: &mut impl VecLike<Vec2>, out_projections: &mut impl VecLike<Projection>) {
        out_axes.extend_from_slice(self.axes.as_slice());
        out_projections.extend_from_slice(&self.projections);
    }

    fn get_axes_derived(&self, other: &[Vec2], out_axes: &mut impl VecLike<Vec2>) {
        self.shape.get_axes_derived(other, out_axes)
    }

    fn with_offset(self, offset: Vec2) -> Self {
        Self { 
            shape: self.shape.with_offset(offset), 
            axes: self.axes.clone(), 
            projections: self.projections.iter().enumerate().map(|(i, v)| v.offset_by(self.axes[i].dot(offset))).collect::<ArrayVec<[Projection; CAP]>>()
        }
    }
}

impl<T: SATShape> From<T> for Cache<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
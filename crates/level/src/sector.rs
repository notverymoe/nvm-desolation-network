use tinyvec::SliceVec;

use crate::{SectorPoint, SectorSlope, SlopeKind};

pub struct Sector {
    pub points: Box<[SectorPoint]>,
    pub slope: SectorSlope,
}

impl Sector {

    pub fn new(points: Box<[SectorPoint]>, slope: SectorSlope) -> Self {
        Self{points, slope}
    }

    pub fn generate_wall_loop(&self, kind: Option<SlopeKind>, buffer: &mut SliceVec<[f32; 3]>) {
        buffer.clear();
        buffer.extend(self.points.iter().map(SectorPoint::to_world));
        if let Some(kind) = kind {
            self.slope.apply_to(buffer.as_mut_slice(), kind);
        }
    }

    pub fn generate_surface(&self, kind: Option<SlopeKind>, buffer_loop: &mut SliceVec<[f32; 3]>, buffer_surface: &mut SliceVec<[[f32; 3]; 3]>) {
        self.generate_wall_loop(kind, buffer_loop);
        self.slope.tessslate(buffer_loop, buffer_surface);
    }
    
}
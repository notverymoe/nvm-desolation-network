
pub const UNIT_SCALE: f32 = 1.0;

#[derive(Debug, Default, Clone, Copy)]
pub struct SectorPoint {
    pub x: i16,
    pub y: i16,
}

impl SectorPoint {

    pub fn new(x: i16, y: i16) -> Self {
        Self{x, y}
    }

    pub fn to_world(&self) -> [f32; 3] {
        [
            to_world_pos(self.x),
            to_world_pos(self.y),
            0.0
        ]
    }
}

pub fn to_world_pos(v: i16) -> f32 {
    (v as f32)/UNIT_SCALE
}

pub fn from_world_pos(v: f32) -> i16 {
    (v * UNIT_SCALE) as i16
}
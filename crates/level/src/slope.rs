use tinyvec::SliceVec;

use crate::to_world_pos;

#[derive(Debug, Default, Clone, Copy)]
pub struct SectorSlope {
    pub anchor: SlopeAnchor,
    pub start:  i16,
    pub end:   [i16; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlopeKind {
    Roof,
    Floor,
}

impl SectorSlope {

    pub fn new(anchor: SlopeAnchor, start: i16, end: [i16; 2]) -> Self {
        Self{anchor, start, end}
    }

    pub fn apply_to(&self, points: &mut [[f32; 3]], kind: SlopeKind) {

        let kind_idx: usize = if kind == SlopeKind::Floor { 0 } else { 1 };

        let point_len    = points.len() as isize;
        let slope_start  = to_world_pos([0, self.start][kind_idx]);  
        let slope_height = to_world_pos(self.end[kind_idx]);
        let slope_len    = point_len/2;
        let slope_inc    = slope_height/(slope_len as f32);

        let wrap_index = |i: isize| (if i >= point_len { i - point_len } else if i < 0 { i + point_len} else { i }) as usize;

        let [anchor_0, anchor_1] = self.anchor.index_pair().map(|v| v as isize);

        for offset in 0..=slope_len {
            let idx_0 = wrap_index(anchor_0 - offset);
            let idx_1 = wrap_index(anchor_1 + offset);
            points[idx_0][2] = slope_start + slope_inc*(offset as f32);
            points[idx_1][2] = slope_start + slope_inc*(offset as f32);
        }
    }

    pub fn tessslate(&self, points: &[[f32; 3]], out: &mut SliceVec<[[f32; 3]; 3]>) {

        let point_len = points.len() as isize;
        let slope_len = point_len/2;

        let wrap_index = |i: isize| (if i >= point_len { i - point_len } else if i < 0 { i + point_len} else { i }) as usize;

        let [anchor_0, anchor_1] = self.anchor.index_pair().map(|v| v as isize);

        for offset in 0..slope_len {
            let idx_00 = wrap_index(anchor_0 - offset);
            let idx_01 = wrap_index(anchor_1 + offset);

            let offset_next = offset + 1;

            let idx_10 = wrap_index(anchor_0 - offset_next);
            let idx_11 = wrap_index(anchor_1 + offset_next);

            if idx_10 == idx_11 {
                // Triangle
                out.push([
                    points[idx_00],
                    points[idx_01],
                    points[idx_11],
                ]);
            } else {
                // Quad
                out.push([
                    points[idx_00],
                    points[idx_01],
                    points[idx_11],
                ]);
                out.push([
                    points[idx_00],
                    points[idx_11],
                    points[idx_10],
                ]);
            }
        }

    }


}

#[derive(Debug, Default, Clone, Copy)]
pub struct SlopeAnchor(u8);

impl SlopeAnchor {

    pub fn from_point(idx: usize) -> Self {
        assert!(idx < 128, "Anchor index can only include indicies [0, 127]");
        Self(idx as u8)
    }

    pub fn from_edge(idx: usize) -> Self {
        assert!(idx < 128, "Anchor index can only include indicies [0, 127]");
        Self(idx as u8 | 0x80)
    }

    pub fn is_point(&self) -> bool {
        self.0 & 0x80 == 0
    }

    pub fn is_edge(&self) -> bool {
        !self.is_point()
    }

    pub fn index_pair(&self) -> [usize; 2] {
        let idx = self.to_raw();
        [
            if self.is_point() { idx + 1 } else { idx + 1 },
            if self.is_point() { idx + 1 } else { idx + 2 },
        ]
    }

    pub fn to_raw(&self) -> usize {
        (self.0 & 0x7F) as usize
    }

}
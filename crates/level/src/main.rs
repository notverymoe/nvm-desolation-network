use level::{Sector, SectorPoint, SectorSlope, SlopeAnchor, SlopeKind, Buffer};



pub fn main() {

    let sector = Sector::new(
        Box::new([
            SectorPoint::new(   0,   0),
            SectorPoint::new( 100, 000),
            SectorPoint::new( 200, 100),
            SectorPoint::new( 200, 200),
            SectorPoint::new( 100, 300),
            SectorPoint::new(   0, 300),
            SectorPoint::new(-100, 200),
            SectorPoint::new(-100, 100),
        ]),
        SectorSlope::new(
            SlopeAnchor::from_edge(3),
            200,
            [80, -80]
        ),
    );

    let mut buffer_loop = Buffer::new(128);
    let mut buffer_loop = buffer_loop.get();

    let mut buffer_surface = Buffer::new(128);
    let mut buffer_surface = buffer_surface.get();

    sector.generate_surface(Some(SlopeKind::Floor), &mut buffer_loop, &mut buffer_surface);
    //sector.generate_surface(Some(SlopeKind::Roof ), &mut buffer_loop, &mut buffer_surface);
    std::fs::write("./room.obj", write_obj(&buffer_surface)).unwrap();
}

fn write_obj(triangles: &[[[f32; 3]; 3]]) -> String {

    let mut result = String::new();

    for (idx_tri, triangle) in triangles.iter().enumerate() {
        let idx_vert = idx_tri*3;
        for [x, y, z] in triangle {
            result.push_str(&format!("v {} {} {}\n", x, y, z));
        }
        result.push_str(&format!("f {} {} {}\n\n", idx_vert+1, idx_vert+2, idx_vert+3));
    }

    result

    

}
use math::*;
use geometry::*;
use core::*;

pub struct DisplacementMap {
    texture: Box<Texture>,
    min: f64,
    max: f64,
}

impl DisplacementMap {
    pub fn new(texture: Box<Texture>, min: f64, max: f64) -> DisplacementMap {
        DisplacementMap {
            texture,
            min,
            max,
        }
    }
}

pub fn displace_triangle_mesh(map: DisplacementMap, data: TriangleMeshData, smoothing: Smoothing) -> TriangleMeshData {
    let map_extent = map.max - map.min;
    let uvs = data.uvs.expect("cannot displace a mesh without uvs");
    let new_positions: Vec<Point> = data.positions
        .into_iter()
        .enumerate()
        .map(|(i, position)| {
            let scale = map.texture.get_color(Some(uvs[i])).average() * map_extent + map.min;
            position * scale
        })
        .collect();

    TriangleMeshData::new(new_positions, smoothing, Some(uvs), data.indices, true)
}

use std::path::Path;
use image::open as openImage;
use math::*;
use geometry::*;
use core::*;

pub struct DisplacementMap {
    map: Box<UvMap>,
    min: f64,
    max: f64,
}

impl DisplacementMap {
    pub fn from_path(path: &Path, min: f64, max: f64) -> DisplacementMap {
        match openImage(path) {
            Ok(img) => DisplacementMap { map: Box::new(img.to_rgb()), min, max },
            Err(reason) => { panic!("could not open image at {:?}: {:?}", path, reason); }
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
            let scale = map.map.get_color(uvs[i]).average() * map_extent + map.min;
            position * scale
        })
        .collect();

    TriangleMeshData::new(new_positions, smoothing, Some(uvs), data.indices, true)
}

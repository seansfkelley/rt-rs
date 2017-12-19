use std::path::Path;
use image::{ RgbImage, open as openImage };
use math::*;
use geometry::*;
use util::*;

pub struct DisplacementMap {
    map: RgbImage,
    min: f64,
    max: f64,
}

impl DisplacementMap {
    pub fn from_path(path: &Path, min: f64, max: f64) -> DisplacementMap {
        match openImage(path) {
            Ok(img) => DisplacementMap { map: img.to_rgb(), min, max },
            Err(reason) => { panic!("could not open image at {:?}: {:?}", path, reason); }
        }
    }
}

pub fn displace_triangle_mesh(map: DisplacementMap, mesh: TriangleMesh, smoothing: Smoothing) -> TriangleMesh {
    let data = mesh.get_data();
    let map_extent = map.max - map.min;
    match data.uvs {
        Some(ref uvs) => {
            let new_positions: Vec<Point> = data.positions
                .iter()
                .enumerate()
                .map(|(i, position)| {
                    let scale = map.map.get_color(uvs[i]).average() * map_extent + map.min;
                    position.clone() * scale
                })
                .collect();

            TriangleMesh::new(new_positions, smoothing, Some(uvs.clone()), data.indices.clone(), true)
        }
        None => { panic!("cannot displace a mesh without uvs"); }
    }
}

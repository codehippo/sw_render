use crate::common::space::{ModelBox, ModelPoint};
use derive_more::Constructor;
use obj::raw::object::Polygon;
use obj::raw::RawObj;
use obj::ObjError;
use std::io::BufRead;

#[derive(Constructor, Default)]
pub struct Mesh {
    pub vertices: Vec<ModelPoint>,
    tris_face_indices: Vec<[usize; 3]>,
    bounding_box: ModelBox,
}

impl Mesh {
    pub fn from_obj<B: BufRead>(reader: B) -> Result<Self, ObjError> {
        let raw_obj: RawObj = obj::raw::parse_obj(reader)?;
        let mut mesh = Self {
            vertices: vec![],
            tris_face_indices: vec![],
            bounding_box: ModelBox {
                min: ModelPoint::new(f32::MAX, f32::MAX, f32::MAX),
                max: ModelPoint::new(f32::MIN, f32::MIN, f32::MIN),
            },
        };

        mesh.vertices = raw_obj
            .positions
            .iter()
            .map(|&(x, y, z, _)| ModelPoint::new(x, y, z))
            .collect();

        mesh.tris_face_indices = raw_obj
            .polygons
            .iter()
            .flat_map(|polygon| {
                let indices: Vec<usize> = match polygon {
                    Polygon::P(indices) => indices.to_vec(),
                    Polygon::PT(indices) => indices.iter().map(|&(pos, _)| pos).collect(),
                    Polygon::PN(indices) => indices.iter().map(|&(pos, _)| pos).collect(),
                    Polygon::PTN(indices) => indices.iter().map(|&(pos, _, _)| pos).collect(),
                };
                indices
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                    .collect::<Vec<_>>()
            })
            .collect();

        for vertex in &mesh.vertices {
            mesh.bounding_box.min = mesh.bounding_box.min.min(*vertex);
            mesh.bounding_box.max = mesh.bounding_box.max.max(*vertex);
        }

        Ok(mesh)
    }

    pub fn tris_faces(&self) -> impl Iterator<Item = [ModelPoint; 3]> + '_ {
        self.tris_face_indices
            .iter()
            .map(|&[a, b, c]| [self.vertices[a], self.vertices[b], self.vertices[c]])
    }
}

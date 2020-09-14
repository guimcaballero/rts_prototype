use bevy::{prelude::*, render::mesh::VertexAttribute};

pub fn rectangle_mesh() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.indices = Some(Vec::from([0, 1, 2, 0, 2, 3]));
    mesh.attributes = rectangle_attributes(Vec3::zero(), Vec3::zero());

    mesh
}

pub fn rectangle_attributes(a: Vec3, b: Vec3) -> Vec<VertexAttribute> {
    // TODO Figure out the 4 vertices
    let max_x = f32::max(a.x(), b.x());
    let min_x = f32::min(a.x(), b.x());
    let max_z = f32::max(a.z(), b.z());
    let min_z = f32::min(a.z(), b.z());

    let mut attributes = Vec::new();
    attributes.push(VertexAttribute::position(Vec::from([
        [min_x, 0.0, min_z],
        [min_x, 0.0, max_z],
        [max_x, 0.0, max_z],
        [max_x, 0.0, min_z],
    ])));

    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for _ in 0..4 {
        normals.push([0.0, 0.0, 0.0]);
        uvs.push([0.0, 0.0]);
    }

    attributes.push(VertexAttribute::normal(normals));
    attributes.push(VertexAttribute::uv(uvs));
    attributes
}

use bevy::{prelude::*, render::mesh::Indices};

pub fn rectangle_mesh() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    set_rectangle_attributes(&mut mesh, Vec3::zero(), Vec3::zero());
    mesh.set_indices(Some(Indices::U32(Vec::from([0, 1, 2, 0, 2, 3]))));

    mesh
}

pub fn set_rectangle_attributes(mesh: &mut Mesh, a: Vec3, b: Vec3) {
    let max_x = f32::max(a.x(), b.x());
    let min_x = f32::min(a.x(), b.x());
    let max_z = f32::max(a.z(), b.z());
    let min_z = f32::min(a.z(), b.z());

    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for _ in 0..4 {
        normals.push([0.0, 0.0, 0.0]);
        uvs.push([0.0, 0.0]);
    }

    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        Vec::from([
            [min_x, 0.0, min_z],
            [min_x, 0.0, max_z],
            [max_x, 0.0, max_z],
            [max_x, 0.0, min_z],
        ])
        .into(),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals.into());
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs.into());
}

pub fn circle_mesh() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    set_circle_attributes(&mut mesh);
    mesh.set_indices(Some(Indices::U32(vec![
        0, 3, 1, 1, 3, 2, 1, 4, 0, 4, 5, 0, 5, 6, 0, 4, 7, 5, 1, 8, 4, 8, 9, 4, 1, 10, 8, 2, 11, 1,
        11, 12, 1, 12, 13, 1, 11, 14, 12, 2, 15, 11, 15, 16, 11, 2, 17, 15, 3, 18, 2, 18, 19, 2,
        19, 20, 2, 18, 21, 19, 3, 22, 18, 22, 23, 18, 3, 24, 22, 0, 25, 3, 25, 26, 3, 26, 27, 3,
        25, 28, 26, 0, 29, 25, 29, 30, 25, 0, 31, 29,
    ])));

    mesh
}

pub fn set_circle_attributes(mesh: &mut Mesh) {
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for _ in 0..32 {
        normals.push([0.0, 0.0, 0.0]);
        uvs.push([0.0, 0.0]);
    }

    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        Vec::from([
            [-60.0, 0.0, 0.0],
            [0.0, 0.0, -60.0],
            [60.0, 0.0, 0.0],
            [0.0, 0.0, 60.0],
            [-42.4264, 0.0, -42.426414],
            [-55.43277, 0.0, -22.961006],
            [-58.84712, 0.0, -11.705415],
            [-49.888172, 0.0, -33.334217],
            [-22.961014, 0.0, -55.43277],
            [-33.3342, 0.0, -49.888187],
            [-11.705423, 0.0, -58.847115],
            [42.426422, 0.0, -42.42639],
            [22.961016, 0.0, -55.432766],
            [11.705425, 0.0, -58.847115],
            [33.334225, 0.0, -49.88817],
            [55.432785, 0.0, -22.96098],
            [49.88819, 0.0, -33.334194],
            [58.84712, 0.0, -11.705414],
            [42.426407, 0.0, 42.426407],
            [55.43277, 0.0, 22.961008],
            [58.847115, 0.0, 11.70542],
            [49.888176, 0.0, 33.334213],
            [22.961006, 0.0, 55.43277],
            [33.33421, 0.0, 49.88818],
            [11.705414, 0.0, 58.84712],
            [-42.426407, 0.0, 42.426407],
            [-22.96101, 0.0, 55.43277],
            [-11.70542, 0.0, 58.847115],
            [-33.33421, 0.0, 49.88818],
            [-55.432777, 0.0, 22.960997],
            [-49.88818, 0.0, 33.33421],
            [-58.84712, 0.0, 11.705405],
        ])
        .into(),
    );
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals.into());
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs.into());
}

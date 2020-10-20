// From https://gist.github.com/superdump/202d9db357a4fae8e845deb98ebaf14c
use bevy::{
    prelude::*,
    render::{
        camera::Camera,
        mesh::{Indices, Mesh, VertexAttribute},
        pipeline::PrimitiveTopology,
    },
};

struct Cone {
    radius: f32,
    segments: usize,
    height: f32,
}

impl Default for Cone {
    fn default() -> Self {
        Cone {
            radius: 0.5f32,
            segments: 32,
            height: 1.0f32,
        }
    }
}

impl From<Cone> for Mesh {
    fn from(cone: Cone) -> Self {
        let mut positions = Vec::with_capacity(cone.segments + 2);
        let mut normals = Vec::with_capacity(cone.segments + 2);
        let mut uvs = Vec::with_capacity(cone.segments + 2);
        let mut indices = Vec::with_capacity(cone.segments + 2);

        // bottom
        positions.push([0.0, 0.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5, 0.0]);

        let angle = 2.0f32 * std::f32::consts::PI / cone.segments as f32;

        // circular base of cone
        let frac_h_2 = Vec3::new(0.0f32, 0.5f32 * cone.height, 0.0f32);
        for i in 0..cone.segments {
            let (z, x) = (angle * i as f32).sin_cos();
            let (z, x) = (cone.radius * z, cone.radius * x);
            let position = Vec3::new(x, 0.0f32, z);
            positions.push(*position.as_ref());
            let normal = (position - frac_h_2).normalize();
            normals.push(*normal.as_ref());
            // FIXME
            uvs.push([0.5, 0.0]);
        }

        // top
        positions.push([0.0, cone.height, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5, 1.0]);

        for i in 0..cone.segments {
            // bottom circle
            indices.append(&mut vec![
                0u32,
                (1 + (i % cone.segments)) as u32,
                (1 + ((i + 1) % cone.segments)) as u32,
            ]);
            // cone
            indices.append(&mut vec![
                (cone.segments + 1) as u32,
                (1 + ((i + 1) % cone.segments)) as u32,
                (1 + (i % cone.segments)) as u32,
            ]);
        }

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(positions),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
            ],
            indices: Some(Indices::U32(indices)),
        }
    }
}

struct Cylinder {
    radius: f32,
    segments: usize,
    height: f32,
}

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            radius: 0.5f32,
            segments: 32,
            height: 1.0f32,
        }
    }
}

impl From<Cylinder> for Mesh {
    fn from(cylinder: Cylinder) -> Self {
        let mut positions = Vec::with_capacity(cylinder.segments + 2);
        let mut normals = Vec::with_capacity(cylinder.segments + 2);
        let mut uvs = Vec::with_capacity(cylinder.segments + 2);
        let mut indices = Vec::with_capacity(cylinder.segments + 2);

        // bottom
        positions.push([0.0, 0.0, 0.0]);
        normals.push([0.0, -1.0, 0.0]);
        uvs.push([0.5, 0.0]);

        // top
        positions.push([0.0, cylinder.height, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([0.5, 1.0]);

        let angle = 2.0f32 * std::f32::consts::PI / cylinder.segments as f32;

        // circular base of cylinder
        for i in 0..cylinder.segments {
            let (z, x) = (angle * i as f32).sin_cos();
            let (z, x) = (cylinder.radius * z, cylinder.radius * x);
            let magnitude = (x * x + z * z).sqrt();
            positions.push([x, 0.0, z]);
            normals.push([x / magnitude, 0.0, z / magnitude]);
            // FIXME
            uvs.push([0.5, 0.0]);
        }

        // circular top of cylinder
        for i in 0..cylinder.segments {
            let (z, x) = (angle * i as f32).sin_cos();
            let (z, x) = (cylinder.radius * z, cylinder.radius * x);
            let magnitude = (x * x + z * z).sqrt();
            positions.push([x, cylinder.height, z]);
            normals.push([x / magnitude, 0.0, z / magnitude]);
            // FIXME
            uvs.push([0.5, cylinder.height]);
        }

        for i in 0..cylinder.segments {
            let bottom_offset = 2;
            let top_offset = 2 + cylinder.segments;

            // bottom circle
            indices.append(&mut vec![
                0u32,
                (bottom_offset + (i % cylinder.segments)) as u32,
                (bottom_offset + ((i + 1) % cylinder.segments)) as u32,
            ]);

            // cylinder
            indices.append(&mut vec![
                (bottom_offset + ((i + 1) % cylinder.segments)) as u32,
                (bottom_offset + (i % cylinder.segments)) as u32,
                (top_offset + (i % cylinder.segments)) as u32,
            ]);
            indices.append(&mut vec![
                (top_offset + (i % cylinder.segments)) as u32,
                (top_offset + ((i + 1) % cylinder.segments)) as u32,
                (bottom_offset + ((i + 1) % cylinder.segments)) as u32,
            ]);

            // top circle
            indices.append(&mut vec![
                1u32,
                (top_offset + ((i + 1) % cylinder.segments)) as u32,
                (top_offset + (i % cylinder.segments)) as u32,
            ]);
        }

        Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
                VertexAttribute::position(positions),
                VertexAttribute::normal(normals),
                VertexAttribute::uv(uvs),
            ],
            indices: Some(Indices::U32(indices)),
        }
    }
}

#[derive(Default)]
pub struct AxesPlugin;
impl Plugin for AxesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(axes_setup.system())
            .add_system(axes_system.system());
    }
}

struct AxesTag;
fn axes_system(
    mut camera_query: Query<(&Camera, &Transform)>,
    mut axes_query: Query<(&AxesTag, &mut Transform)>,
) {
    let mut cam_temp = camera_query.iter();
    let (camera, camera_transform) = cam_temp.iter().next().unwrap();
    let mut axes_temp = axes_query.iter();
    let (_, mut axes_transform) = axes_temp.iter().next().unwrap();

    let view_matrix = camera_transform.compute_matrix();
    let projection_matrix = camera.projection_matrix;
    let world_pos: Vec4 =
        (view_matrix * projection_matrix.inverse()).mul_vec4(Vec4::new(0.7, -0.8, 0.3, 1.0));
    let position: Vec3 = (world_pos / world_pos.w()).truncate().into();

    axes_transform.translation = position;
}

fn axes_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // In-scene
    let cylinder_mesh = meshes.add(Mesh::from(Cylinder {
        height: 0.85f32,
        radius: 0.03f32,
        ..Default::default()
    }));
    let cone_mesh = meshes.add(Mesh::from(Cone {
        height: 0.15f32,
        radius: 0.07f32,
        ..Default::default()
    }));
    let red = standard_materials.add(Color::RED.into());
    let green = standard_materials.add(Color::GREEN.into());
    let blue = standard_materials.add(Color::BLUE.into());

    commands
        .spawn((
            GlobalTransform::identity(),
            Transform::from_scale(Vec3::splat(0.1)),
            AxesTag,
        ))
        .with_children(|axes_root| {
            axes_root
                .spawn((
                    GlobalTransform::identity(),
                    Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2)),
                ))
                .with_children(|axis_root| {
                    axis_root
                        .spawn(PbrComponents {
                            material: red.clone(),
                            mesh: cone_mesh.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                0.0f32, 0.85f32, 0.0f32,
                            )),
                            ..Default::default()
                        })
                        .spawn(PbrComponents {
                            material: red,
                            mesh: cylinder_mesh.clone(),
                            ..Default::default()
                        });
                })
                .spawn((GlobalTransform::identity(), Transform::identity()))
                .with_children(|axis_root| {
                    axis_root
                        .spawn(PbrComponents {
                            material: green.clone(),
                            mesh: cone_mesh.clone(),
                            transform: Transform::from_translation(Vec3::new(
                                0.0f32, 0.85f32, 0.0f32,
                            )),
                            ..Default::default()
                        })
                        .spawn(PbrComponents {
                            material: green,
                            mesh: cylinder_mesh.clone(),
                            ..Default::default()
                        });
                })
                .spawn((
                    GlobalTransform::identity(),
                    Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ))
                .with_children(|axis_root| {
                    axis_root
                        .spawn(PbrComponents {
                            material: blue.clone(),
                            mesh: cone_mesh,
                            transform: Transform::from_translation(Vec3::new(
                                0.0f32, 0.85f32, 0.0f32,
                            )),
                            ..Default::default()
                        })
                        .spawn(PbrComponents {
                            material: blue,
                            mesh: cylinder_mesh,
                            ..Default::default()
                        });
                });
        });
}

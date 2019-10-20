use amethyst::prelude::*;
use amethyst::{
    assets::AssetLoaderSystemData,
    core::{
        math,
        math::geometry::{Rotation, Rotation3},
        Transform,
    },
    renderer::{
        loaders,
        palette::LinSrgba,
        rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
        types::MeshData,
        Camera, Material, MaterialDefaults, Texture,
    },
};
use float_cmp::ApproxEq;
use std::f32::consts::PI;

pub struct VoxelState {}

#[derive(Debug)]
struct ChunkMesh {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    uv: Vec<TexCoord>,
    indices: Vec<u16>,
}

impl ChunkMesh {
    fn new() -> Self {
        ChunkMesh {
            positions: Vec::new(),
            normals: Vec::new(),
            uv: Vec::new(),
            indices: Vec::new(),
        }
    }

    fn insert_quad(&mut self, pos: math::Vector3<f32>, dir: math::Vector3<f32>) {
        let count = self.positions.len() as u16;
        /*
        2-------3   ^
        |       |  x|
        0-------1  y->
        */
        let verts: [(
            math::Vector3<f32>,
            math::Vector3<f32>,
            math::Vector3<f32>,
            math::Vector3<f32>,
        ); 6] = [
            (
                [-0.5, 0.5, 0.5].into(),
                [0.5, 0.5, 0.5].into(),
                [-0.5, -0.5, 0.5].into(),
                [0.5, -0.5, 0.5].into(),
            ),
            (
                [-0.5, -0.5, -0.5].into(),
                [0.5, -0.5, -0.5].into(),
                [-0.5, 0.5, -0.5].into(),
                [0.5, 0.5, -0.5].into(),
            ),
            (
                [-0.5, 0.5, -0.5].into(),
                [0.5, 0.5, -0.5].into(),
                [-0.5, 0.5, 0.5].into(),
                [0.5, 0.5, 0.5].into(),
            ),
            (
                [-0.5, -0.5, 0.5].into(),
                [0.5, -0.5, 0.5].into(),
                [-0.5, -0.5, -0.5].into(),
                [0.5, -0.5, -0.5].into(),
            ),
            (
                [0.5, -0.5, 0.5].into(),
                [0.5, 0.5, 0.5].into(),
                [0.5, -0.5, -0.5].into(),
                [0.5, 0.5, -0.5].into(),
            ),
            (
                [-0.5, -0.5, -0.5].into(),
                [-0.5, 0.5, -0.5].into(),
                [-0.5, -0.5, 0.5].into(),
                [-0.5, 0.5, 0.5].into(),
            ),
        ];

        let (vert0, vert1, vert2, vert3) = match dir {
            x if x[1] == 1. => verts[2],
            x if x[1] == -1. => verts[3],
            x if x[0] == 1. => verts[4],
            x if x[0] == -1. => verts[5],
            x if x[2] == 1. => verts[0],
            x if x[2] == -1. => verts[1],
            _ => unreachable!(),
        };

        self.positions.push((pos + vert0).into());
        self.positions.push((pos + vert1).into());
        self.positions.push((pos + vert2).into());
        self.positions.push((pos + vert3).into());

        self.normals.push(dir.into());
        self.normals.push(dir.into());
        self.normals.push(dir.into());
        self.normals.push(dir.into());

        self.uv.push([0., 0.].into());
        self.uv.push([1., 0.].into());
        self.uv.push([1., 1.].into());
        self.uv.push([0., 1.].into());

        self.indices.push(count + 0);
        self.indices.push(count + 2);
        self.indices.push(count + 1);
        self.indices.push(count + 3);
        self.indices.push(count + 1);
        self.indices.push(count + 2);
    }

    fn build_mesh<'a>(&self) -> MeshBuilder<'a> {
        amethyst::renderer::rendy::mesh::MeshBuilder::new()
            .with_vertices(self.positions.clone())
            .with_vertices(self.normals.clone())
            .with_vertices(self.uv.clone())
            .with_indices(Indices::from(self.indices.clone()))
    }
}

impl SimpleState for VoxelState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_camera(data.world);
        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);
        create_cube(data.world, transform);
    }
}

fn create_cube(world: &mut World, pos: Transform) {
    let default_mat = world.read_resource::<MaterialDefaults>().0.clone();

    let mut chunk_mesh = ChunkMesh::new();

    chunk_mesh.insert_quad([0., 0., 0.].into(), math::Vector3::y());
    chunk_mesh.insert_quad([0., 0., 0.].into(), -math::Vector3::y());
    chunk_mesh.insert_quad([0., 0., 0.].into(), math::Vector3::x());
    chunk_mesh.insert_quad([0., 0., 0.].into(), -math::Vector3::x());
    chunk_mesh.insert_quad([0., 0., 0.].into(), math::Vector3::z());
    chunk_mesh.insert_quad([0., 0., 0.].into(), -math::Vector3::z());

    let mesh = world.exec(
        |loader: AssetLoaderSystemData<amethyst::renderer::types::Mesh>| {
            loader.load_from_data(
                amethyst::renderer::types::MeshData(chunk_mesh.build_mesh()),
                (),
            )
        },
    );

    let albedo = world.exec(|loader: AssetLoaderSystemData<Texture>| {
        loader.load_from_data(
            loaders::load_from_linear_rgba(LinSrgba::new(0.0, 100.0, 0.0, 1.0)).into(),
            (),
        )
    });

    let mat = world.exec(|loader: AssetLoaderSystemData<Material>| {
        loader.load_from_data(
            Material {
                albedo,
                ..default_mat.clone()
            },
            (),
        )
    });

    world.create_entity().with(mesh).with(mat).with(pos).build();
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 2.);

    world
        .create_entity()
        .with(Camera::standard_3d(1.4, 1.))
        .with(transform)
        .build();
}

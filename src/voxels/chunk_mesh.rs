use crate::{directions::Directions, ui::init_fps_counter};
use amethyst::{
    assets::AssetLoaderSystemData,
    core::{
        math::{
            self,
            geometry::{Rotation, Rotation3},
        },
        Transform,
    },
    prelude::*,
    renderer::{
        loaders,
        palette::LinSrgba,
        rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
        types::MeshData,
        Camera, Material, MaterialDefaults, Texture,
    },
};
use std::f32::consts::PI;

#[derive(Debug)]
pub struct ChunkMeshData {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    uv: Vec<TexCoord>,
    indices: Vec<u16>,
}

impl ChunkMeshData {
    pub fn new() -> Self {
        ChunkMeshData {
            positions: Vec::new(),
            normals: Vec::new(),
            uv: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn insert_quad(&mut self, pos: math::Vector3<f32>, dir: Directions) {
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
            // TODO: remove clones when Clion is happy without them
            x if x.intersects(Directions::UP) => verts[2].clone(),
            x if x.intersects(Directions::DOWN) => verts[3].clone(),
            x if x.intersects(Directions::WEST) => verts[4].clone(),
            x if x.intersects(Directions::EAST) => verts[5].clone(),
            x if x.intersects(Directions::NORTH) => verts[0].clone(),
            x if x.intersects(Directions::SOUTH) => verts[1].clone(),
            _ => unreachable!(),
        };

        self.positions.push((pos.clone() + vert0).into());
        self.positions.push((pos.clone() + vert1).into());
        self.positions.push((pos.clone() + vert2).into());
        self.positions.push((pos.clone() + vert3).into());

        self.normals.push(dir.to_vec().into());
        self.normals.push(dir.to_vec().into());
        self.normals.push(dir.to_vec().into());
        self.normals.push(dir.to_vec().into());

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

    pub fn build_mesh<'a>(&self) -> MeshBuilder<'a> {
        amethyst::renderer::rendy::mesh::MeshBuilder::new()
            .with_vertices(self.positions.clone())
            .with_vertices(self.normals.clone())
            .with_vertices(self.uv.clone())
            .with_indices(Indices::from(self.indices.clone()))
    }
}

pub fn create_cube(world: &mut World, pos: Transform) {
    let default_mat = world.read_resource::<MaterialDefaults>().0.clone();

    let mut chunk_mesh = ChunkMeshData::new();

    chunk_mesh.insert_quad([0., 0., 0.].into(), Directions::UP);
    chunk_mesh.insert_quad([0., 0., 0.].into(), Directions::DOWN);
    chunk_mesh.insert_quad([0., 0., 0.].into(), Directions::EAST);
    chunk_mesh.insert_quad([0., 0., 0.].into(), Directions::WEST);
    chunk_mesh.insert_quad([0., 0., 0.].into(), Directions::SOUTH);
    chunk_mesh.insert_quad([0., 0., 0.].into(), Directions::NORTH);

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
            loaders::load_from_linear_rgba(LinSrgba::new(1.0, 0.0, 0.0, 0.5)).into(),
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

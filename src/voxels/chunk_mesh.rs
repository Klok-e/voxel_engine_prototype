use crate::directions::Directions;
use amethyst::{
    core::math::{self},
    renderer::rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
};

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
        if dir.into_iter().count() > 1 {
            panic!("insert_quad called with more than one direction");
        }

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
            // south
            (
                [-0.5, 0.5, 0.5].into(),
                [0.5, 0.5, 0.5].into(),
                [-0.5, -0.5, 0.5].into(),
                [0.5, -0.5, 0.5].into(),
            ),
            // north
            (
                [-0.5, -0.5, -0.5].into(),
                [0.5, -0.5, -0.5].into(),
                [-0.5, 0.5, -0.5].into(),
                [0.5, 0.5, -0.5].into(),
            ),
            // up
            (
                [-0.5, 0.5, -0.5].into(),
                [0.5, 0.5, -0.5].into(),
                [-0.5, 0.5, 0.5].into(),
                [0.5, 0.5, 0.5].into(),
            ),
            // down
            (
                [-0.5, -0.5, 0.5].into(),
                [0.5, -0.5, 0.5].into(),
                [-0.5, -0.5, -0.5].into(),
                [0.5, -0.5, -0.5].into(),
            ),
            // east
            (
                [0.5, -0.5, 0.5].into(),
                [0.5, 0.5, 0.5].into(),
                [0.5, -0.5, -0.5].into(),
                [0.5, 0.5, -0.5].into(),
            ),
            // west
            (
                [-0.5, -0.5, -0.5].into(),
                [-0.5, 0.5, -0.5].into(),
                [-0.5, -0.5, 0.5].into(),
                [-0.5, 0.5, 0.5].into(),
            ),
        ];

        let (vert0, vert1, vert2, vert3) = match dir {
            x if x.intersects(Directions::UP) => verts[2],
            x if x.intersects(Directions::DOWN) => verts[3],
            x if x.intersects(Directions::WEST) => verts[5],
            x if x.intersects(Directions::EAST) => verts[4],
            x if x.intersects(Directions::NORTH) => verts[1],
            x if x.intersects(Directions::SOUTH) => verts[0],
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
        self.uv.push([0., 1.].into());
        self.uv.push([1., 1.].into());

        self.indices.push(count + 0);
        self.indices.push(count + 2);
        self.indices.push(count + 1);
        self.indices.push(count + 3);
        self.indices.push(count + 1);
        self.indices.push(count + 2);
    }
    /// Returns a mesh. None if mesh is empty.
    pub fn build_mesh<'a>(&self) -> Option<MeshBuilder<'a>> {
        if self.positions.is_empty() {
            None
        } else {
            Some(
                amethyst::renderer::rendy::mesh::MeshBuilder::new()
                    .with_vertices(self.positions.clone())
                    .with_vertices(self.normals.clone())
                    .with_vertices(self.uv.clone())
                    .with_indices(Indices::from(self.indices.clone())),
            )
        }
    }
}

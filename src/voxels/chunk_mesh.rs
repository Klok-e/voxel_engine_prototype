use bevy::{
    prelude::{Vec2, Vec3},
    render::mesh::{Indices, Mesh},
};

use crate::directions::Directions;

#[derive(Debug, Default)]
pub struct ChunkMeshData {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uv: Vec<Vec2>,
    indices: Vec<u16>,
}

impl ChunkMeshData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_quad(&mut self, pos: Vec3, dir: Directions) {
        if dir.into_iter().count() > 1 {
            panic!("insert_quad called with more than one direction");
        }

        let count = self.positions.len() as u16;
        /*
        2-------3   ^
        |       |  x|
        0-------1  y->
        */
        let verts: [(Vec3, Vec3, Vec3, Vec3); 6] = [
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

        self.positions.push(pos + vert0);
        self.positions.push(pos + vert1);
        self.positions.push(pos + vert2);
        self.positions.push(pos + vert3);

        self.normals.push(dir.to_fvec());
        self.normals.push(dir.to_fvec());
        self.normals.push(dir.to_fvec());
        self.normals.push(dir.to_fvec());

        self.uv.push([0., 0.].into());
        self.uv.push([1., 0.].into());
        self.uv.push([0., 1.].into());
        self.uv.push([1., 1.].into());

        self.indices.push(count);
        self.indices.push(count + 2);
        self.indices.push(count + 1);
        self.indices.push(count + 3);
        self.indices.push(count + 1);
        self.indices.push(count + 2);
    }

    /// Returns a mesh. None if mesh is empty.
    pub fn build_mesh(&self) -> Option<Mesh> {
        if self.positions.is_empty() {
            None
        } else {
            let mut mesh =
                Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                self.positions
                    .iter()
                    .map(|&x| x.into())
                    .collect::<Vec<[f32; 3]>>(),
            );
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                self.normals
                    .iter()
                    .map(|&x| x.into())
                    .collect::<Vec<[f32; 3]>>(),
            );
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                self.uv.iter().map(|&x| x.into()).collect::<Vec<[f32; 2]>>(),
            );
            mesh.set_indices(Some(Indices::U16(self.indices.clone())));
            Some(mesh)
        }
    }
}

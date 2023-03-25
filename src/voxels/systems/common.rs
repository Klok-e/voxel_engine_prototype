use crate::{
    directions::Directions,
    voxels::{chunk::ChunkPosition, resources::EntityChunks, world::VoxelWorldProcedural},
};
use bevy::prelude::{Commands, Component, Entity, IVec3, Query, ResMut, Transform, With};

pub fn process_chunk_edges<TEdgeChunk: Component, TProcessAround: Component>(
    edge_chunks: Query<(Entity, &ChunkPosition), (With<TEdgeChunk>,)>,
    vox_world: &mut VoxelWorldProcedural,
    loaders: &Query<&Transform, (With<TProcessAround>,)>,
    ent_chunks: &mut ResMut<EntityChunks>,
    commands: &mut Commands,
    apply: impl Fn(
        IVec3,
        &Query<&Transform, (With<TProcessAround>,)>,
        &mut VoxelWorldProcedural,
        &mut EntityChunks,
        &mut Commands,
    ),
) {
    for (ent, chpos) in edge_chunks.iter() {
        vox_world
            .chunk_at(chpos)
            .expect("Chunk wasn't generated, but an entity exists!");
        let mut is_edge = false;
        for dir in Directions::all().into_iter().map(|d| d.to_ivec()) {
            let edge_chunk_pos = chpos.pos + dir;
            if vox_world.chunk_at(&edge_chunk_pos.into()).is_none() {
                is_edge = true;

                apply(edge_chunk_pos, loaders, vox_world, ent_chunks, commands);
            }
        }
        if !is_edge {
            commands.entity(ent).remove::<TEdgeChunk>();
        }
    }
}

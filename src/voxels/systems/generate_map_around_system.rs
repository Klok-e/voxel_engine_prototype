use crate::{
    directions::Directions,
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZEI},
        resources::EntityChunks,
        world::VoxelWorldProcedural,
    },
};
use bevy::prelude::{Commands, Entity, IVec3, PbrBundle, Query, Res, ResMut, Transform, With};

use super::components::{EdgeChunk, EdgeRenderChunk, GenerateMapAround};

pub fn generate_map_around_system(
    mut vox_world: ResMut<VoxelWorldProcedural>,
    mut ent_chunks: ResMut<EntityChunks>,
    config: Res<RuntimeGameConfig>,
    loaders: Query<&Transform, (With<GenerateMapAround>,)>,
    edge_chunks: Query<(Entity, &ChunkPosition), (With<EdgeChunk>,)>,
    mut commands: Commands,
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

                generate_chunks_on_edge(
                    &loaders,
                    edge_chunk_pos,
                    config.config.generate_around_bubble,
                    &mut vox_world,
                    &mut ent_chunks,
                    &mut commands,
                );
            }
        }
        if !is_edge {
            commands.entity(ent).remove::<EdgeChunk>();
        }
    }

    for transform in loaders.iter() {
        let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);

        // chunk loader currently occupies MUST be generated
        if vox_world.chunk_at(&curr_chpos).is_none() {
            create_chunk(&mut vox_world, &mut ent_chunks, curr_chpos, &mut commands);
        };
    }
}

fn generate_chunks_on_edge(
    loaders: &Query<&Transform, (With<GenerateMapAround>,)>,
    edge_chunk_pos: IVec3,
    generate_around_bubble: usize,
    vox_world: &mut VoxelWorldProcedural,
    ent_chunks: &mut EntityChunks,
    commands: &mut Commands,
) {
    for transform in loaders.iter() {
        let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);

        if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize <= generate_around_bubble {
            create_chunk(vox_world, ent_chunks, edge_chunk_pos.into(), commands);
        }
    }
}

fn create_chunk(
    vox_world: &mut VoxelWorldProcedural,
    ent_chunks: &mut EntityChunks,
    chpos: ChunkPosition,
    commands: &mut Commands,
) {
    let new_chunk = vox_world.gen_chunk(&chpos);
    vox_world.insert_at(&chpos, new_chunk);
    let ent = commands
        .spawn((
            chpos,
            EdgeChunk,
            EdgeRenderChunk,
            PbrBundle {
                transform: Transform {
                    translation: (chpos.pos * CHSIZEI).as_vec3(),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    ent_chunks.map.insert(chpos, ent);
}

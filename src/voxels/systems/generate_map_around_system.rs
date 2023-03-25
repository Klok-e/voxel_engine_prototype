use crate::{
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZEI},
        resources::EntityChunks,
        world::VoxelWorldProcedural,
    },
};
use bevy::prelude::{Commands, Entity, IVec3, PbrBundle, Query, Res, ResMut, Transform, With};

use super::components::{EdgeChunk, GenerateMapAround};

pub fn generate_map_around_system(
    mut vox_world: ResMut<VoxelWorldProcedural>,
    mut ent_chunks: ResMut<EntityChunks>,
    config: Res<RuntimeGameConfig>,
    loaders: Query<&Transform, (With<GenerateMapAround>,)>,
    edge_chunks: Query<(Entity, &ChunkPosition), (With<EdgeChunk>,)>,
    mut commands: Commands,
) {
    // info!("gen edge_chunks {}", edge_chunks.iter().count());

    let mut chunks_generated = 0;
    for (ent, chpos) in edge_chunks.iter() {
        let mut is_edge = false;
        for dir in crate::directions::Directions::all()
            .into_iter()
            .map(|d| d.to_ivec())
        {
            let edge_chunk_pos = chpos.pos + dir;
            if vox_world.get_chunk_at(&edge_chunk_pos.into()).is_none() {
                is_edge = true;

                generate_chunks_on_edge(
                    &loaders,
                    edge_chunk_pos,
                    &config,
                    &mut vox_world,
                    &mut ent_chunks,
                    &mut commands,
                    &mut chunks_generated,
                )
            }
        }
        if !is_edge {
            commands.entity(ent).remove::<EdgeChunk>();
        }
    }

    for transform in loaders.iter() {
        let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);

        // chunk loader currently occupies MUST be generated
        if vox_world.get_chunk_at(&curr_chpos).is_none() {
            create_chunk(&mut vox_world, &mut ent_chunks, curr_chpos, &mut commands);
        };
    }
}

fn generate_chunks_on_edge(
    loaders: &Query<&Transform, (With<GenerateMapAround>,)>,
    edge_chunk_pos: IVec3,
    config: &RuntimeGameConfig,
    vox_world: &mut VoxelWorldProcedural,
    ent_chunks: &mut EntityChunks,
    commands: &mut Commands,
    chunks_generated: &mut usize,
) {
    for transform in loaders.iter() {
        let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);

        if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize
            <= config.config.render_around_bubble + 1
        {
            create_chunk(vox_world, ent_chunks, edge_chunk_pos.into(), commands);
        } else if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize
            <= config.config.generate_around_bubble
            && *chunks_generated < config.chunks_generate_per_frame as usize
        {
            create_chunk(vox_world, ent_chunks, edge_chunk_pos.into(), commands);
            *chunks_generated += 1;
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

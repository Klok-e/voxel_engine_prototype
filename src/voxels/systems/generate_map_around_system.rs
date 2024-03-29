use crate::{
    directions::Directions,
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZEF, CHSIZEI},
        resources::EntityChunks,
        world::VoxelWorldProcedural,
    },
};
use bevy::prelude::{
    Color, Commands, Entity, IVec3, PbrBundle, Query, Res, ResMut, Transform, Vec3, With,
};
use bevy_prototype_debug_lines::DebugShapes;

use super::{
    common::may_neighbours_produce_mesh,
    components::{EdgeChunk, GenerateMapAround},
};

pub fn generate_map_around_system(
    mut vox_world: ResMut<VoxelWorldProcedural>,
    mut ent_chunks: ResMut<EntityChunks>,
    config: Res<RuntimeGameConfig>,
    loaders: Query<&Transform, (With<GenerateMapAround>,)>,
    edge_chunks: Query<(Entity, &ChunkPosition), (With<EdgeChunk>,)>,
    mut commands: Commands,
    mut lines: ResMut<DebugShapes>,
) {
    // info!("gen edge_chunks {}", edge_chunks.iter().count());

    let mut chunks_generated = 0;
    for (ent, chpos) in edge_chunks.iter() {
        if config.debug_show_edge_chunks {
            lines
                .cuboid()
                .position((chpos.pos * CHSIZEI).as_vec3())
                .size(Vec3::ONE * CHSIZEF)
                .color(Color::PURPLE);
        }

        let mut is_edge = false;
        for dir in Directions::all().into_iter().map(|d| d.to_ivec()) {
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

    let pos_with_changes = vox_world
        .chunk_changes()
        .pin()
        .iter()
        .filter(|(_, changes)| changes.lock().unwrap().len() > 0)
        .flat_map(|(pos, _1)| {
            std::iter::once(pos.pos)
                .chain(Directions::all().into_iter().map(|d| pos.pos + d.to_ivec()))
        })
        .collect::<Vec<_>>();
    for chpos in pos_with_changes {
        if vox_world.get_chunk_at(&chpos.into()).is_none() {
            create_chunk(&mut vox_world, &mut ent_chunks, chpos.into(), &mut commands);
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

        let may_neighbours_mesh = || may_neighbours_produce_mesh(vox_world, edge_chunk_pos);
        if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize <= 2 {
            create_chunk(vox_world, ent_chunks, edge_chunk_pos.into(), commands);
        } else if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize
            <= config.config.render_around_bubble
        {
            if may_neighbours_mesh() {
                create_chunk(vox_world, ent_chunks, edge_chunk_pos.into(), commands);
            }
        } else if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize
            <= config.config.generate_around_bubble
            && *chunks_generated < config.chunks_generate_per_frame as usize
            && may_neighbours_mesh()
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

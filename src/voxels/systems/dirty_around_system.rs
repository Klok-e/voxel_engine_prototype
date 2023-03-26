use crate::{
    game_config::RuntimeGameConfig,
    voxels::{chunk::ChunkPosition, resources::EntityChunks, world::VoxelWorldProcedural},
};

use bevy::prelude::{Commands, Entity, IVec3, Query, Res, ResMut, Transform, With};

use super::components::{EdgeRenderChunk, RenderAround, RenderedTag};

pub fn dirty_around_system(
    vox_world: ResMut<VoxelWorldProcedural>,
    config: Res<RuntimeGameConfig>,
    render_bubbles: Query<&Transform, (With<RenderAround>,)>,
    rendered_chunks: Query<&ChunkPosition, (With<RenderedTag>,)>,
    ent_chunks: Res<EntityChunks>,
    edge_chunks: Query<(Entity, &ChunkPosition), (With<EdgeRenderChunk>,)>,
    mut commands: Commands,
) {
    // info!("edge_chunks {}", edge_chunks.iter().count());
    // info!("dirty {}", vox_world.dirty().len());

    for (ent, chpos) in edge_chunks.iter() {
        let mut is_edge = false;
        for dir in crate::directions::Directions::all()
            .into_iter()
            .map(|d| d.to_ivec())
        {
            let edge_chunk_pos = chpos.pos + dir;
            if !rendered_chunks.contains(ent_chunks.map[&edge_chunk_pos.into()]) {
                is_edge = true;

                mark_dirty_on_edge(
                    &render_bubbles,
                    edge_chunk_pos,
                    config.config.render_around_bubble,
                    &vox_world,
                    &mut commands,
                    &ent_chunks,
                )
            }
        }
        if !is_edge {
            commands.entity(ent).remove::<EdgeRenderChunk>();
        }
    }

    for transform in render_bubbles.iter() {
        let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);

        // chunk loader currently occupies MUST be generated
        if !rendered_chunks.contains(ent_chunks.map[&curr_chpos]) {
            mark_for_render(&vox_world, &ent_chunks, curr_chpos, &mut commands);
        };
    }
}

fn mark_dirty_on_edge(
    loaders: &Query<&Transform, (With<RenderAround>,)>,
    edge_chunk_pos: IVec3,
    render_around_bubble: usize,
    vox_world: &VoxelWorldProcedural,
    commands: &mut Commands,
    ent_chunks: &EntityChunks,
) {
    for transform in loaders.iter() {
        let (curr_chpos, _) = VoxelWorldProcedural::to_ch_pos_index(&transform.translation);

        if (curr_chpos.pos - edge_chunk_pos).as_vec3().length() as usize <= render_around_bubble {
            let may_produce_mesh = may_chunk_produce_mesh(vox_world, edge_chunk_pos);
            if may_produce_mesh {
                mark_for_render(vox_world, ent_chunks, edge_chunk_pos.into(), commands);
            }
        }
    }
}

fn may_chunk_produce_mesh(vox_world: &VoxelWorldProcedural, pos: IVec3) -> bool {
    enum Foo {
        Transparent,
        Nontransparent,
    }

    let chunk_at = vox_world.chunk_at(&pos.into());
    let is_transparent = chunk_at.is_transparent();
    let is_nontransparent = chunk_at.is_nontransparent();
    if !is_transparent && !is_nontransparent {
        return true;
    }

    let variant = if is_transparent {
        Foo::Transparent
    } else {
        Foo::Nontransparent
    };

    let mut will_produce_mesh = false;
    for dir in crate::directions::Directions::all()
        .into_iter()
        .map(|d| d.to_ivec())
    {
        let edge_chunk_pos = pos + dir;
        let Some(next_chunk) = &vox_world.get_chunk_at(&edge_chunk_pos.into()) else {
            return false;
        };
        let is_next_transparent = match variant {
            Foo::Transparent => next_chunk.is_transparent(),
            Foo::Nontransparent => next_chunk.is_nontransparent(),
        };
        if match variant {
            Foo::Transparent => chunk_at.is_transparent(),
            Foo::Nontransparent => chunk_at.is_nontransparent(),
        } != is_next_transparent
        {
            will_produce_mesh = true;
            break;
        }
    }
    will_produce_mesh
}

fn mark_for_render(
    vox_world: &VoxelWorldProcedural,
    ent_chunks: &EntityChunks,
    curr_chpos: ChunkPosition,
    commands: &mut Commands,
) {
    let dirty = vox_world.dirty().pin();
    dirty.insert(curr_chpos);
    let entity = ent_chunks.map[&curr_chpos];
    commands.entity(entity).insert(EdgeRenderChunk);

    // info!("mark {}", curr_chpos.pos);
}

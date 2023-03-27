use bevy::prelude::IVec3;

use crate::voxels::world::VoxelWorldProcedural;

pub fn may_chunk_produce_mesh(vox_world: &VoxelWorldProcedural, pos: IVec3) -> bool {
    let chunk_at = vox_world.chunk_at(&pos.into());
    let is_transparent = chunk_at.is_transparent();
    let is_nontransparent = chunk_at.is_nontransparent();
    if !is_transparent && !is_nontransparent {
        return true;
    }

    let mut will_produce_mesh = false;
    for dir in crate::directions::Directions::all()
        .into_iter()
        .map(|d| d.to_ivec())
    {
        let edge_chunk_pos = pos + dir;
        let Some(next_chunk) = &vox_world.get_chunk_at(&edge_chunk_pos.into()) else {
            return true;
        };
        let is_next = (next_chunk.is_transparent(), next_chunk.is_nontransparent());
        if (chunk_at.is_transparent(), chunk_at.is_nontransparent()) != is_next {
            will_produce_mesh = true;
            break;
        }
    }
    will_produce_mesh
}

pub fn may_neighbours_produce_mesh(vox_world: &VoxelWorldProcedural, pos: IVec3) -> bool {
    let mut prev_indicators = None;

    let mut may_produce_mesh = false;
    for dir in crate::directions::Directions::all()
        .into_iter()
        .map(|d| d.to_ivec())
    {
        let edge_chunk_pos = pos + dir;
        let Some(next_chunk) = &vox_world.get_chunk_at(&edge_chunk_pos.into()) else {
            continue;
        };

        let is_transparent = next_chunk.is_transparent();
        let is_nontransparent = next_chunk.is_nontransparent();
        if !is_transparent && !is_nontransparent {
            return true;
        }
        let var = (is_transparent, is_nontransparent);

        if let Some(prev_indicators) = prev_indicators {
            if prev_indicators != var {
                may_produce_mesh = true;
                break;
            }
        } else {
            prev_indicators = Some(var);
        }
    }
    may_produce_mesh
}

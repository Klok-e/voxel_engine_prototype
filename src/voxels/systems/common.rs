use bevy::prelude::IVec3;

use crate::voxels::world::VoxelWorldProcedural;

pub fn may_chunk_produce_mesh(vox_world: &VoxelWorldProcedural, pos: IVec3) -> bool {
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
            return true;
        };
        let is_next = match variant {
            Foo::Transparent => next_chunk.is_transparent(),
            Foo::Nontransparent => next_chunk.is_nontransparent(),
        };
        if match variant {
            Foo::Transparent => chunk_at.is_transparent(),
            Foo::Nontransparent => chunk_at.is_nontransparent(),
        } != is_next
        {
            will_produce_mesh = true;
            break;
        }
    }
    will_produce_mesh
}

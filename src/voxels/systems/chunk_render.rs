use crate::{
    game_config::RuntimeGameConfig,
    voxels::{chunk::ChunkPosition, systems::components::RenderedTag, world::VoxelWorldProcedural},
};
use bevy::prelude::{Assets, Commands, Entity, Mesh, Query, Res, ResMut};
use rayon::prelude::*;
use std::{collections::HashMap, sync::mpsc::channel};

use super::materials::Materials;

pub fn chunk_render_system(
    mut commands: Commands,
    vox_world: Res<VoxelWorldProcedural>,
    config: Res<RuntimeGameConfig>,
    mats: Res<Materials>,
    mut chunks: Query<(Entity, &mut ChunkPosition)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let chunk_entities = {
        let mut map = HashMap::new();
        for (ent, chunk_pos) in chunks.iter_mut() {
            map.insert(*chunk_pos, ent);
        }
        map
    };

    let (sender, receiver) = channel();

    let dirty = vox_world.dirty().pin();
    dirty
        .iter()
        .take(config.chunks_render_per_frame as usize)
        .collect::<Vec<_>>()
        .into_par_iter()
        .copied()
        .map_with(sender, |s, pos| (pos, s.clone()))
        .for_each(|(to_clean, sender)| {
            let mesh = vox_world.mesh(&to_clean);

            // create mesh
            let mesh: Option<Mesh> = mesh.build_mesh();

            // get entity from hashmap or create a new one
            let ent = chunk_entities[&to_clean];
            sender.send((mesh, ent, to_clean)).unwrap();
        });

    for cmd in receiver.into_iter() {
        let (mesh, ent, to_clean) = cmd;
        let mut entity = commands.entity(ent);
        if let Some(mesh) = mesh {
            entity.insert((meshes.add(mesh), mats.material.clone()));
        }
        entity.insert(RenderedTag);

        dirty.remove(&to_clean);
    }
}

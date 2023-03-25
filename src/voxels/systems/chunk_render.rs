use crate::{
    game_config::RuntimeGameConfig,
    voxels::{chunk::ChunkPosition, systems::components::RenderedTag, world::VoxelWorldProcedural},
};
use bevy::prelude::{Assets, Commands, Entity, Mesh, Query, Res, ResMut};
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc::channel,
    },
};

use super::materials::Materials;

pub fn chunk_render_system(
    mut commands: Commands,
    vox_world: Res<VoxelWorldProcedural>,
    config: Res<RuntimeGameConfig>,
    mats: Res<Materials>,
    mut chunks: Query<(Entity, &mut ChunkPosition)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    #[derive(Debug)]
    struct SetMesh(Mesh, Entity);

    let chunk_entities = {
        let mut map = HashMap::new();
        for (ent, chunk_pos) in chunks.iter_mut() {
            map.insert(*chunk_pos, ent);
        }
        map
    };

    let (sender, receiver) = channel();

    let rendered = AtomicU32::new(0);
    vox_world
        .dirty()
        .pin()
        .iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .copied()
        .map_with(sender, |s, pos| (pos, s.clone()))
        .for_each(|(to_clean, sender)| {
            if rendered.load(Ordering::SeqCst) >= config.chunks_render_per_frame {
                return;
            }
            let mesh = if let Some(m) = vox_world.mesh(&to_clean) {
                m
            } else {
                return;
            };

            // create mesh
            let mesh: Option<Mesh> = mesh.build_mesh();

            // get entity from hashmap or create a new one
            let mut command = None;
            let ent = chunk_entities[&to_clean];
            if let Some(m) = mesh {
                command = Some(SetMesh(m, ent));
            }
            sender.send(command).unwrap();

            vox_world.dirty().pin().remove(&to_clean);

            let r = rendered.load(Ordering::SeqCst);
            rendered.store(r + 1, Ordering::SeqCst)
        });

    for cmd in receiver.into_iter().flatten() {
        let SetMesh(mesh, ent) = cmd;
        commands
            .entity(ent)
            .insert((meshes.add(mesh), mats.material.clone(), RenderedTag));
    }
}

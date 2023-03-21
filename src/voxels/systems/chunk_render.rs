use crate::{
    core::ConvertVecExtension,
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZEI},
        systems::components::RenderedTag,
        world::VoxelWorldProcedural,
    },
};
use bevy::prelude::{
    default, Assets, Commands, Entity, Mesh, PbrBundle, Query, Res, ResMut, Transform,
};
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
    mut q1: Query<(Entity, &mut ChunkPosition)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    struct CreateNew(ChunkPosition, Transform, RenderedTag);
    struct SetMesh(Mesh, Option<Entity>);

    let chunk_entities = {
        let mut map = HashMap::new();
        for (ent, chunk_pos) in q1.iter_mut() {
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
            let mut command = (None, None);
            let ent = chunk_entities.get(&to_clean).cloned();
            if ent.is_none() {
                let mut transform = Transform::default();
                transform.translation = (to_clean.pos * CHSIZEI).convert_vec();

                // create entity
                command = (Some(CreateNew(to_clean, transform, RenderedTag)), None);
            };
            if let Some(m) = mesh {
                command.1 = Some(SetMesh(m, ent));
            }
            sender.send(command).unwrap();

            vox_world.dirty().pin().remove(&to_clean);

            let r = rendered.load(Ordering::SeqCst);
            rendered.store(r + 1, Ordering::SeqCst)
        });

    for cmd in receiver.into_iter() {
        match cmd {
            (Some(CreateNew(chpos, pos, render)), Some(SetMesh(mesh, _))) => {
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: mats.material.clone(),
                        transform: pos,
                        ..default()
                    })
                    .insert((chpos, render));
            }
            (Some(CreateNew(chpos, pos, render)), None) => {
                commands
                    .spawn(PbrBundle {
                        material: mats.material.clone(),
                        transform: pos,
                        ..default()
                    })
                    .insert((chpos, render));
            }
            (None, Some(SetMesh(mesh, ent))) => {
                commands.entity(ent.unwrap()).insert(meshes.add(mesh));
            }
            (None, None) => {}
        }
    }
}

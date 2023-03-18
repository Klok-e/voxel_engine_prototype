use crate::{
    core::ConvertVecExtension,
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZEI},
        world::VoxelWorldProcedural,
    },
};
use bevy::prelude::{
    default, Assets, Commands, Entity, Mesh, PbrBundle, Query, Res, ResMut, Transform,
};
use flurry::epoch::pin;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc::channel,
    },
};

use super::{dirty_around_system::RenderedTag, materials::Materials};

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

    // debug!("chunk_render");

    let chunk_entities = {
        let mut map = HashMap::new();
        for (ent, chunk_pos) in q1.iter_mut() {
            map.insert(*chunk_pos, ent);
        }
        map
    };

    let (sender, receiver) = channel();

    let rendered = AtomicU32::new(0);
    let guard = pin();
    vox_world
        .dirty()
        .iter(&guard)
        .collect::<Vec<_>>()
        .into_par_iter()
        .copied()
        .map_with(sender, |s, pos| (pos, s.clone()))
        .for_each_init(pin, |guard, (to_clean, sender)| {
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

                // draw debug lines
                // let mut debug_lines = DebugLinesComponent::new();
                // debug_lines.add_box(
                //     (to_vecf(to_clean.pos) * CHSIZEF).into(),
                //     ((to_vecf(to_clean.pos) + Vec3f::from([1., 1., 1.])) * CHSIZEF).into(),
                //     Srgba::new(0.1, 0.1, 0.1, 0.5),
                // );

                // create entity
                command = (Some(CreateNew(to_clean, transform, RenderedTag)), None);
            };
            if let Some(m) = mesh {
                command.1 = Some(SetMesh(m, ent));
            }
            sender.send(command).unwrap();

            vox_world.dirty().remove(&to_clean, guard);

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

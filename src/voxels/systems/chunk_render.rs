use crate::{
    core::{to_vecf, Vec3f},
    game_config::RuntimeGameConfig,
    voxels::{
        chunk::{ChunkPosition, CHSIZEF, CHSIZEI},
        materials::Materials,
        world::VoxelWorldProcedural,
    },
};
use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::Transform,
    ecs::{CommandBuffer, IntoQuery, Runnable, SubWorld},
    renderer::{
        debug_drawing::DebugLinesComponent, palette::Srgba, types::MeshData,
        visibility::BoundingSphere, Material, Mesh,
    },
};
use crossbeam::queue::SegQueue;
use flurry::epoch::pin;
use legion::{query::Query, Entity, SystemBuilder};
use rayon::prelude::*;
use std::{collections::HashMap, sync::mpsc::channel};

use super::dirty_around_system::RenderedTag;

pub fn chunk_render_system() -> impl Runnable {
    SystemBuilder::new("chunk_render_system")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<RuntimeGameConfig>()
        .read_resource::<Materials>()
        .read_resource::<DefaultLoader>()
        .read_resource::<ProcessingQueue<MeshData>>()
        .with_query(<(Entity, &mut ChunkPosition)>::query())
        .with_query(<(&mut Handle<Mesh>,)>::query())
        .with_query(<(
            Entity,
            &mut ChunkPosition,
            &mut Transform,
            &mut Handle<Mesh>,
            &mut Handle<Material>,
            &mut DebugLinesComponent,
            &mut BoundingSphere,
        )>::query())
        .build(move |commands, world, resources, query| {
            chunk_render(
                commands,
                world,
                &resources.0,
                &resources.1,
                &resources.2,
                &resources.3,
                &resources.4,
                &mut query.0,
                &mut query.1,
            )
        })
}

fn chunk_render(
    commands: &mut CommandBuffer,
    w: &mut SubWorld,
    vox_world: &VoxelWorldProcedural,
    config: &RuntimeGameConfig,
    materials: &Materials,
    loader: &DefaultLoader,
    mesh_queue: &ProcessingQueue<MeshData>,
    q1: &mut Query<(Entity, &mut ChunkPosition)>,
    _meshes: &mut Query<(&mut Handle<Mesh>,)>,
) {
    struct CreateNew(
        ChunkPosition,
        Transform,
        Handle<Material>,
        DebugLinesComponent,
        BoundingSphere,
        RenderedTag,
    );
    struct SetMesh(Handle<Mesh>, Option<Entity>);

    let chunk_entities = {
        let mut map = HashMap::new();
        for (ent, chunk_pos) in q1.iter_mut(w) {
            map.insert(*chunk_pos, *ent);
        }
        map
    };

    let (sender, receiver) = channel();

    let guard = pin();
    vox_world
        .dirty()
        .iter(&guard)
        .collect::<Vec<_>>()
        .into_par_iter()
        .copied()
        .take(config.chunks_render_per_frame)
        .map_with(sender, |sender, to_clean| (sender.clone(), to_clean))
        .for_each_init(
            || pin(),
            |guard, (sender, to_clean)| {
                let mesh = if let Some(m) = vox_world.mesh(&to_clean) {
                    m
                } else {
                    return;
                };

                // create mesh
                let mesh: Option<Handle<Mesh>> = mesh
                    .build_mesh()
                    .map(|m| MeshData(m.into_owned()))
                    .map(|m| loader.load_from_data(m, (), mesh_queue));

                // get entity from hashmap or create a new one
                let mut command = (None, None);
                let ent = chunk_entities.get(&to_clean).cloned();
                if ent.is_none() {
                    let mut transform = Transform::default();
                    transform.set_translation(to_vecf(to_clean.pos * CHSIZEI));

                    // draw debug lines
                    let mut debug_lines = DebugLinesComponent::new();
                    debug_lines.add_box(
                        (to_vecf(to_clean.pos) * CHSIZEF).into(),
                        ((to_vecf(to_clean.pos) + Vec3f::from([1., 1., 1.])) * CHSIZEF).into(),
                        Srgba::new(0.1, 0.1, 0.1, 0.5),
                    );

                    let def_mat = materials.chunks.clone();

                    // create entity
                    command = (
                        Some(CreateNew(
                            to_clean,
                            transform,
                            def_mat,
                            debug_lines,
                            BoundingSphere::new(
                                (Vec3f::from([1., 1., 1.]) * CHSIZEF / 2.).into(),
                                // distance from center to outermost vertex of a cube
                                CHSIZEF * 3. / 2.,
                            ),
                            RenderedTag,
                        )),
                        None,
                    );
                };
                if let Some(m) = mesh {
                    command.1 = Some(SetMesh(m, ent));
                }
                sender.send(command).unwrap();

                vox_world.dirty().remove(&to_clean, &guard);
            },
        );

    for cmd in receiver.into_iter() {
        match cmd {
            (Some(CreateNew(chpos, pos, mat, lines, bound, render)), Some(SetMesh(mesh, _))) => {
                commands.push((chpos, pos, mat, lines, bound, render, mesh));
            }
            (Some(CreateNew(chpos, pos, mat, lines, bound, render)), None) => {
                commands.push((chpos, pos, mat, lines, bound, render));
            }
            (None, Some(SetMesh(mesh, ent))) => {
                commands.add_component(ent.unwrap(), mesh);
            }
            (None, None) => {}
        }
    }
}

use crate::{
    core::{to_vecf, Vec3f},
    game_config::GameConfig,
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
use flurry::epoch::pin;
use legion::{query::Query, Entity, SystemBuilder};
use std::collections::HashMap;

pub fn chunk_render_system(/*mut readerid: ReaderId<InputEvent>*/) -> impl Runnable {
    SystemBuilder::new("chunk_render_system")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<GameConfig>()
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
    config: &GameConfig,
    materials: &Materials,
    loader: &DefaultLoader,
    mesh_queue: &ProcessingQueue<MeshData>,
    q1: &mut Query<(Entity, &mut ChunkPosition)>,
    _meshes: &mut Query<(&mut Handle<Mesh>,)>,
) {
    let mut chunk_entities = HashMap::new();
    for (ent, chunk_pos) in q1.iter_mut(w) {
        chunk_entities.insert(*chunk_pos, ent);
    }

    let guard = pin();
    for to_clean in vox_world
        .dirty()
        .iter(&guard)
        .take(config.chunks_render_per_frame)
    {
        // create mesh
        let mesh: Option<Handle<Mesh>> = match vox_world.mesh(&to_clean, &guard) {
            Some(m) => m
                .build_mesh()
                .map(|m| MeshData(m.into_owned()))
                .map(|m| loader.load_from_data(m, (), mesh_queue)),
            None => continue,
        };

        // get entity from hashmap or create a new one
        let entity: Entity = chunk_entities
            .get(to_clean)
            .map(|v| **v)
            .unwrap_or_else(|| {
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
                commands.push((
                    *to_clean,
                    transform,
                    def_mat,
                    debug_lines,
                    BoundingSphere::new(
                        (Vec3f::from([1., 1., 1.]) * CHSIZEF / 2.).into(),
                        // distance from center to outermost vertex of a cube
                        CHSIZEF * 3. / 2.,
                    ),
                ))
            });
        if let Some(m) = mesh {
            commands.add_component(entity, m);
        }

        vox_world.dirty().remove(to_clean, &guard);
    }
}

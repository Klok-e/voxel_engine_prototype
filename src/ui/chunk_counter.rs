use amethyst::{
    ecs::{system, systems, Entity, IntoQuery, Runnable, SubWorld},
    ui::UiText,
};
use legion::{query::Query, SystemBuilder};
use log;

use crate::voxels::{chunk::ChunkPosition, world::VoxelWorldProcedural};

pub struct GeneratedCounterText {
    pub entity: Entity,
}

pub struct RenderedCounterText {
    pub entity: Entity,
}

pub fn chunk_counter_ui_system() -> impl Runnable {
    SystemBuilder::new("chunk_counter_ui_system")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<GeneratedCounterText>()
        .read_resource::<RenderedCounterText>()
        .with_query(<(&mut UiText,)>::query())
        .with_query(<(&ChunkPosition,)>::query())
        .build(move |_, world, resources, query| {
            chunk_counter_ui(
                world,
                &resources.0,
                &resources.1,
                &resources.2,
                &mut query.0,
                &mut query.1,
            )
        })
}

fn chunk_counter_ui(
    w: &mut SubWorld,
    voxel_world: &VoxelWorldProcedural,
    generated_text: &GeneratedCounterText,
    rendered_text: &RenderedCounterText,
    ui_text: &mut Query<(&mut UiText,)>,
    ch_positions: &mut Query<(&ChunkPosition,)>,
) {
    match ui_text.get_mut(w, generated_text.entity) {
        Ok((t,)) => {
            t.text = format!("ch gen: {}", voxel_world.chunks().len());
        }
        Err(e) => {
            log::warn!("No GeneratedCounterText UiText found! {}", e);
        }
    }
    let (mut w_chpos, mut w_uit) = w.split_for_query(ui_text);
    match ui_text.get_mut(&mut w_chpos, rendered_text.entity) {
        Ok((t,)) => {
            t.text = format!("ch rend: {}", ch_positions.iter(&mut w_uit).count());
        }
        Err(e) => {
            log::warn!("No RenderedCounterText UiText found! {}", e);
        }
    }
}

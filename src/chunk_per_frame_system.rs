use amethyst::{
    ecs::{Runnable},
    utils::fps_counter::FpsCounter,
};
use flurry::epoch::pin;
use amethyst::ecs::SystemBuilder;
use log::info;

use crate::{game_config::RuntimeGameConfig, voxels::world::VoxelWorldProcedural};

pub fn chunk_per_frame_system() -> impl Runnable {
    SystemBuilder::new("chunk_per_frame_system")
        .read_resource::<VoxelWorldProcedural>()
        .read_resource::<FpsCounter>()
        .write_resource::<RuntimeGameConfig>()
        .build(move |_, _, resources, _| {
            chunk_per_frame(&resources.0, &resources.1, &mut resources.2)
        })
}

fn chunk_per_frame(
    vox_world: &VoxelWorldProcedural,
    fps: &FpsCounter,
    config: &mut RuntimeGameConfig,
) {
    let fps = fps.sampled_fps();
    if fps < config.config.generation_maintain_fps {
        config.chunks_render_per_frame = 1.max(config.chunks_render_per_frame - 1);
        config.chunks_generate_per_frame = 1.max(config.chunks_generate_per_frame - 1);
    } else {
        let guard = pin();
        if vox_world.dirty().iter(&guard).count() > 0 {
            config.chunks_render_per_frame += 1;
        } else {
            config.chunks_generate_per_frame += 1;
        }
    }
    info!(
        "chunks_generate_per_frame: {}; chunks_render_per_frame: {}.",
        config.chunks_generate_per_frame, config.chunks_render_per_frame
    );
}

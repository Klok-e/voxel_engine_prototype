use super::{world::VoxelWorld, ChunkPosition};
use crate::{core::Vec3i, game_config::GameConfig};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::prelude::*,
    shred::{Read, System, World},
};
use flurry::epoch::pin;

pub struct GenerateMapAround {
    pub distance: i32,
}

impl GenerateMapAround {
    pub fn new(distance: i32) -> Self {
        Self { distance }
    }
}

impl Component for GenerateMapAround {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemDesc)]
pub struct GenerateMapAroundSystem;

impl<'a> System<'a> for GenerateMapAroundSystem {
    type SystemData = (
        Read<'a, VoxelWorld>,
        ReadStorage<'a, GenerateMapAround>,
        ReadExpect<'a, GameConfig>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (voxel_world, generate_around, config, transforms): Self::SystemData) {
        let guard = pin();
        let mut generated = 0;
        'outer: for (around, transform) in (&generate_around, &transforms).join() {
            let (pos, _) = VoxelWorld::to_ch_pos_index(transform.translation());
            for z in -around.distance..=around.distance {
                for y in -around.distance..=around.distance {
                    for x in -around.distance..=around.distance {
                        let pos = ChunkPosition::new(pos.pos + Vec3i::from([x, y, z]));
                        match voxel_world.chunk_at(&pos, &guard) {
                            Some(_) => {}
                            None => {
                                voxel_world.chunk_at_or_create(&pos, &guard);
                                generated += 1;
                                if generated > config.chunks_generate_per_frame {
                                    break 'outer;
                                }
                            }
                        };
                    }
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        <Self as System<'_>>::SystemData::setup(world);
        world.register::<GenerateMapAround>();
    }
}

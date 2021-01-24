use std::cell::RefCell;

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use flurry::epoch::pin;
use ndarray::ArrayViewMut3;
use rand::prelude::*;
use voxel_engine_prototype_lib::{
    core::Vec3i,
    directions::Directions,
    voxels::{
        chunk::ChunkPosition, terrain_generation::VoxelGenerator, voxel::Voxel, world::VoxelWorld,
    },
};

struct RandomGenerator<const N: usize> {
    rng: RefCell<SmallRng>,
}

impl<const N: usize> RandomGenerator<N> {
    fn new(seed: u64) -> Self {
        Self {
            rng: RefCell::new(SmallRng::seed_from_u64(seed)),
        }
    }
}

impl<const N: usize> VoxelGenerator<N> for RandomGenerator<N> {
    fn fill_random(&self, _: &ChunkPosition, arr: &mut ArrayViewMut3<Voxel>) {
        arr.map_inplace(|v| v.id = self.rng.borrow_mut().gen());
    }
}

fn setup<const N: usize>() -> VoxelWorld<RandomGenerator<N>, N> {
    let world = VoxelWorld::new(RandomGenerator::new(42));
    let pos = Vec3i::new(0, 0, 0);
    let guard = pin();
    world.chunk_at_or_create(&ChunkPosition::new(pos), &guard);
    for dir in Directions::all().into_iter() {
        let dir_vec = dir.to_vec::<i32>();
        world.chunk_at_or_create(&ChunkPosition::new(pos + dir_vec), &guard);
    }
    world
}

pub fn meshing(c: &mut Criterion) {
    fn bench_const<const N: usize>(group: &mut BenchmarkGroup<WallTime>, id: BenchmarkId) {
        group.bench_function(id, |b| {
            b.iter_batched(
                || setup::<N>(),
                |world| {
                    let guard = pin();
                    world.mesh(&ChunkPosition::new([0, 0, 0].into()), &guard)
                },
                BatchSize::SmallInput,
            )
        });
    }

    let mut group = c.benchmark_group("meshing");

    group.noise_threshold(0.1);

    bench_const::<16>(&mut group, BenchmarkId::new("mesh", 16));
    bench_const::<32>(&mut group, BenchmarkId::new("mesh", 32));
    bench_const::<64>(&mut group, BenchmarkId::new("mesh", 64));

    group.finish();
}

criterion_group!(benches, meshing);
criterion_main!(benches);

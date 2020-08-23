#![feature(min_const_generics)]

use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup,
    Criterion,PlotConfiguration,BenchmarkId,SamplingMode
};
use rand::prelude::*;
use voxel_engine_prototype_lib::voxels::{Chunk, CHSIZE};
use std::time::Duration;

fn create_random_chunk<const N: usize>(rng: &mut StdRng) -> Chunk<N> {
    let mut ch = Chunk::new();
    ch.data_mut().map_inplace(|v| v.id = rng.gen());
    ch
}

pub fn meshing(c: &mut Criterion) {
    fn bench_const<const N: usize>(group: &mut BenchmarkGroup<WallTime>, id: BenchmarkId) {
        group.bench_function(id, |b| {
            b.iter_batched(
                || {
                    let mut random = StdRng::seed_from_u64(42);
                    create_random_chunk::<N>(&mut random)
                },
                |ch| ch.mesh(),
                BatchSize::SmallInput,
            )
        });
    }

    let mut group = c.benchmark_group("meshing");

    group.significance_level(0.01).sample_size(500);

    bench_const::<2>(&mut group, BenchmarkId::new("mesh", 2));
    bench_const::<4>(&mut group, BenchmarkId::new("mesh", 4));
    bench_const::<6>(&mut group, BenchmarkId::new("mesh", 6));
    bench_const::<8>(&mut group, BenchmarkId::new("mesh", 8));
    bench_const::<10>(&mut group, BenchmarkId::new("mesh", 10));
    bench_const::<12>(&mut group, BenchmarkId::new("mesh", 12));
    bench_const::<14>(&mut group, BenchmarkId::new("mesh", 14));
    bench_const::<16>(&mut group, BenchmarkId::new("mesh", 16));
    bench_const::<18>(&mut group, BenchmarkId::new("mesh", 18));
    bench_const::<20>(&mut group, BenchmarkId::new("mesh", 20));
    bench_const::<22>(&mut group, BenchmarkId::new("mesh",22));
    bench_const::<24>(&mut group, BenchmarkId::new("mesh", 24));
    bench_const::<26>(&mut group, BenchmarkId::new("mesh", 26));
    bench_const::<28>(&mut group, BenchmarkId::new("mesh", 28));
    bench_const::<30>(&mut group, BenchmarkId::new("mesh", 30));
    bench_const::<32>(&mut group, BenchmarkId::new("mesh", 32));
    
    group.finish();
}

criterion_group!(benches, meshing);
criterion_main!(benches);

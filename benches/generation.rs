use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use voxel_engine_prototype_lib::{
    core::Vec3i,
    voxels::{
        chunk::{Chunk, ChunkPosition},
        terrain_generation::ProceduralGenerator,
    },
};

pub fn generation(c: &mut Criterion) {
    fn bench_const<const N: usize>(group: &mut BenchmarkGroup<WallTime>, id: BenchmarkId) {
        group.bench_function(id, |b| {
            b.iter_batched(
                || (ProceduralGenerator::<N>::new(), Chunk::<N>::new()),
                |(gen, mut ch)| {
                    gen.fill_random(
                        &ChunkPosition::new(Vec3i::from([0, 0, 0])),
                        &mut ch.data_mut(),
                    )
                },
                BatchSize::SmallInput,
            )
        });
    }

    let mut group = c.benchmark_group("generation");

    group.noise_threshold(0.1);

    bench_const::<16>(&mut group, BenchmarkId::new("generate", 16));
    bench_const::<18>(&mut group, BenchmarkId::new("generate", 18));
    bench_const::<20>(&mut group, BenchmarkId::new("generate", 20));
    bench_const::<22>(&mut group, BenchmarkId::new("generate", 22));
    bench_const::<24>(&mut group, BenchmarkId::new("generate", 24));
    bench_const::<26>(&mut group, BenchmarkId::new("generate", 26));
    bench_const::<28>(&mut group, BenchmarkId::new("generate", 28));
    bench_const::<30>(&mut group, BenchmarkId::new("generate", 30));
    bench_const::<32>(&mut group, BenchmarkId::new("generate", 32));

    group.finish();
}

criterion_group!(benches, generation);
criterion_main!(benches);

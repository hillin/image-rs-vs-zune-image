use criterion::{criterion_group, criterion_main, Criterion};
use image_bench::image_rs::ImageRsBenchmark;
use image_bench::zune::ZuneImageBenchmark;

fn benchmark_load_image(c: &mut Criterion) {
    c.bench_function("image-rs load", |b| {
        b.iter(|| ImageRsBenchmark::load_image())
    });

    c.bench_function("zune-image load", |b| {
        b.iter(|| ZuneImageBenchmark::load_image())
    });
}

fn benchmark_render(c: &mut Criterion) {
    c.bench_function("image-rs render", |b| b.iter(|| ImageRsBenchmark::render()));

    c.bench_function("zune-image render", |b| {
        b.iter(|| ZuneImageBenchmark::render())
    });
}

criterion_group!(benches, benchmark_load_image, benchmark_render);
criterion_main!(benches);

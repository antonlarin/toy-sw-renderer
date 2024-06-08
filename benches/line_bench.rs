use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use swrender::renderer::line::{draw_line_naive, draw_line_faster};
use swrender::tgaimage::{TGAImage, TGAColor};

pub fn line_benchmark(c: &mut Criterion) {
    let image = TGAImage::with_size(32, 32, 1);
    let color = TGAColor::from_rgb(255, 255, 255);

    let mut group = c.benchmark_group("draw_line");
    let setup = || (image.clone(), color);

    group.bench_function("naive", |b| b.iter_batched_ref(
            setup,
            |(ref mut image, color)| {
                draw_line_naive(11, 5, 30, 21, image, *color)
            },
            BatchSize::SmallInput));
    group.bench_function("faster", |b| b.iter_batched_ref(
            setup,
            |(ref mut image, color)| {
                draw_line_faster(11, 5, 30, 21, image, *color)
            },
            BatchSize::SmallInput));
}

criterion_group!(benches, line_benchmark);
criterion_main!(benches);


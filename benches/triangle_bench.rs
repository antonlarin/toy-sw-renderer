use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use swrender::renderer::triangle::draw_triangle;
use swrender::tgaimage::{TGAImage, TGAColor};
use swrender::math::Point2i;

pub fn line_benchmark(c: &mut Criterion) {
    let image = TGAImage::with_size(32, 32, 1);
    let color = TGAColor::from_rgb(255, 255, 255);
    let v1 = Point2i { x: 30, y: 30 };
    let v2 = Point2i { x: 11, y: 1 };
    let v3 = Point2i { x: 3, y: 18 };

    let setup = || (image.clone(), color);
    let mut group = c.benchmark_group("draw_triangle");

    group.bench_function("naive", |b| b.iter_batched_ref(
            setup,
            |(ref mut image, color)| {
                draw_triangle(v1, v2, v3, image, *color)
            },
            BatchSize::SmallInput));
}

criterion_group!(benches, line_benchmark);
criterion_main!(benches);



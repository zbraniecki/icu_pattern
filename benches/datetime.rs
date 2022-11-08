use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_pattern::datetime::DateTimeData;
use icu_pattern::Pattern;

fn pattern_benches(c: &mut Criterion) {
    let data = DateTimeData::default();
    let pattern = data.get_date_pattern();

    let mut group = c.benchmark_group("date");

    group.bench_function("overview", |b| {
        b.iter(|| {
            let elements = pattern.interpolate(&data, None, None);
            let _ = black_box(elements).count();
        })
    });
    group.finish();

    let pattern = data.get_datetime_pattern();

    let mut group = c.benchmark_group("datetime");

    group.bench_function("overview", |b| {
        b.iter(|| {
            let elements = pattern.interpolate(&data, None, None);
            let _ = black_box(elements).count();
        })
    });
    group.finish();
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use icu_pattern::data::{get_data, output::Output, resolvers::DateTimeResolver};

fn pattern_benches(c: &mut Criterion) {
    let ds = get_data();
    let combination = &ds.date.date_combination;

    let mut group = c.benchmark_group("datetime");

    group.bench_function("overview", |b| {
        b.iter(|| {
            let mut output = Output::default();
            DateTimeResolver::resolve(&ds, combination, &mut output);
        })
    });
    group.finish();
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);

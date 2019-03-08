// cargo bench --bench benchmarks -- --save-baseline master
// cargo bench --bench benchmarks -- --baseline master

use criterion::{criterion_group, criterion_main, Criterion};

use wordcount;

fn criterion_benchmark(c: &mut Criterion) {
    let path = std::path::PathBuf::from("./");
    c.bench_function("count_words big.txt", move |b| {
        b.iter(|| wordcount::count_words_single(&path))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

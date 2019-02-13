use criterion::{Criterion, criterion_group, criterion_main};

use wordcount;

fn criterion_benchmark(c: &mut Criterion) {
    let path = std::path::PathBuf::from("./big.txt");
    c.bench_function("count_words big.txt", move |b| b.iter(|| wordcount::count_words(&path)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
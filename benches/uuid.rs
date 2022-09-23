use std::io::Write;

use allo_isolate::{ffi::DartCObject, IntoDart};
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};

fn simple(uuids: Vec<uuid::Uuid>) -> DartCObject {
    uuids
        .into_iter()
        .map(<uuid::Uuid as IntoDart>::into_dart)
        .collect::<Vec<DartCObject>>()
        .into_dart()
}
fn packed(uuids: Vec<uuid::Uuid>) -> DartCObject {
    let mut buffer = Vec::<u8>::with_capacity(uuids.len() * 16);
    for id in uuids {
        let _ = buffer.write(id.as_bytes());
    }
    buffer.into_dart()
}

fn uuids(count: usize) -> Vec<uuid::Uuid> {
    let mut buffer = Vec::with_capacity(count);
    for _ in 0..count {
        buffer.push(uuid::Uuid::new_v4());
    }
    buffer
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("uuid into dart");
    for i in [100, 1000, 100_000, 1000_000].iter() {
        let input = uuids(*i);
        group.bench_with_input(
            BenchmarkId::new("Delegate to inner type", i),
            i,
            |b, _| b.iter(|| simple(black_box(input.clone()))),
        );
        group.bench_with_input(
            BenchmarkId::new("Pack all in one", i),
            i,
            |b, _| b.iter(|| packed(black_box(input.clone()))),
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

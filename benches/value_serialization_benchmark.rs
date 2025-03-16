use criterion::{criterion_group, criterion_main, Criterion};
use storage::value::Value;

fn benchmark_serialization(c: &mut Criterion) {
    let value = 42;

    c.bench_function("serialize i64", |b| {
        b.iter(|| {
            let _ = Value::from_serializable(&value);
        })
    });
}

fn benchmark_deserialization(c: &mut Criterion) {
    let value = Value::from_serializable(&42).unwrap();

    c.bench_function("deserialize i64", |b| {
        b.iter(|| {
            let _ = value.to_deserializable::<i64>();
        })
    });
}

criterion_group!(benches, benchmark_serialization, benchmark_deserialization);
criterion_main!(benches);

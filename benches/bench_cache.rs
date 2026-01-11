use criterion::{criterion_group, criterion_main, Criterion};
use cache::cache::Cache;
use std::collections::HashMap;
use std::hint::black_box;

fn bench_cache_get(c: &mut Criterion) {
    let mut cache = Cache::<String, String>::new(10_000);
    for i in 0..10_000 {
        cache.put(i.to_string(), format!("value_{}", i));
    }
    c.bench_function("cache get", |b| {
        b.iter(|| {
            for i in 0..10_000 {
                black_box(cache.get(&i.to_string()));
            }
        })
    });
}

fn bench_hashmap_get(c: &mut Criterion) {
    let mut map = HashMap::new();
    for i in 0..10_000 {
        map.insert(i.to_string(), format!("value_{}", i));
    }
    c.bench_function("hashmap get", |b| {
        b.iter(|| {
            for i in 0..10_000 {
                black_box(map.get(&i.to_string()));
            }
        })
    });
}

criterion_group!(benches, bench_cache_get, bench_hashmap_get);
criterion_main!(benches);

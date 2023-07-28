use criterion::{criterion_group, criterion_main, Criterion};

use flo_sparse_array::*;

use std::collections::{HashMap};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("store_hashmap_100k_to_self", |b| b.iter(|| {
        let mut array = HashMap::new();
        for p in 0..100000usize {
            array.insert(p, p);
        }
    }));

    c.bench_function("store_sparse_array_100k_to_self", |b| b.iter(|| {
        let mut array = SparseArray::empty();

        for p in 0..100000usize {
            array.insert(p, p);
        }
    }));

    let mut hash_100k           = HashMap::new();
    let mut sparse_array_100k   = SparseArray::empty();

    for p in 0..100000usize {
        hash_100k.insert(p, p);
        sparse_array_100k.insert(p, p);
    }

    c.bench_function("fetch_hashmap_100k", |b| b.iter(|| {
        for p in 0..100000usize {
            assert!(hash_100k.get(&p) == Some(&p));
        }
    }));

    c.bench_function("fetch_sparse_array_100k", |b| b.iter(|| {
        for p in 0..100000usize {
            assert!(sparse_array_100k.get(p) == Some(&p));
        }
    }));

    c.bench_function("insert_overwrite_hashmap_100k", |b| b.iter(|| {
        for p in 0..100000usize {
            hash_100k.insert(p, p);
        }
    }));

    c.bench_function("insert_overwrite_sparse_array_100k", |b| b.iter(|| {
        for p in 0..100000usize {
            sparse_array_100k.insert(p, p);
        }
    }));

    c.bench_function("update_hashmap_100k", |b| b.iter(|| {
        for p in 0..100000usize {
            (*hash_100k.get_mut(&p).unwrap()) = p;
        }
    }));

    c.bench_function("insert_overwrite_sparse_array_100k", |b| b.iter(|| {
        for p in 0..100000usize {
            (*sparse_array_100k.get_mut(p).unwrap()) = p;
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//! Path lookup cache benchmarks for Day 5 optimization work.

use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use vantis_verified::path_lookup_cache::PathLookupCache;
use vantis_verified::syscall_file_ops::{
    sys_rename_with_cache, sys_stat_with_cache, sys_unlink_with_cache, FileStatPathCache,
};

fn bench_raw_path_lookup_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_lookup_cache_raw");

    group.bench_function("hit", |b| {
        let mut cache = PathLookupCache::new(1024);
        let key = PathBuf::from("/usr/bin/vantis");
        cache.insert(&key, 42u64);

        b.iter(|| black_box(cache.get(black_box(key.as_path()))));
    });

    group.bench_function("miss", |b| {
        let mut cache = PathLookupCache::new(1024);
        let key = PathBuf::from("/usr/bin/unknown");

        b.iter(|| black_box(cache.get(black_box(key.as_path()))));
    });

    group.bench_function("insert_and_evict", |b| {
        b.iter(|| {
            let mut cache = PathLookupCache::new(256);
            for idx in 0..512u64 {
                let path = PathBuf::from(format!("/cache/{idx}"));
                cache.insert(&path, idx);
            }
            black_box(cache.stats().evictions);
        });
    });

    group.finish();
}

fn bench_syscall_path_cache_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_lookup_cache_syscalls");

    group.bench_function("sys_stat_with_cache_hit", |b| {
        let mut cache = FileStatPathCache::new(512);
        let path = PathBuf::from("/var/lib/vantis.db");
        sys_stat_with_cache(&path, &mut cache).expect("cache warmup should succeed");

        b.iter(|| {
            black_box(sys_stat_with_cache(black_box(path.as_path()), &mut cache))
                .expect("stat from cache should succeed");
        });
    });

    group.bench_function("sys_unlink_with_cache", |b| {
        b.iter_batched(
            || {
                let mut cache = FileStatPathCache::new(128);
                let path = PathBuf::from("/tmp/package.vnt");
                sys_stat_with_cache(&path, &mut cache).expect("cache warmup should succeed");
                (cache, path)
            },
            |(mut cache, path)| {
                black_box(sys_unlink_with_cache(path.as_path(), &mut cache))
                    .expect("unlink should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sys_rename_with_cache", |b| {
        b.iter_batched(
            || {
                let mut cache = FileStatPathCache::new(128);
                let old_path = PathBuf::from("/tmp/old.vnt");
                let new_path = PathBuf::from("/tmp/new.vnt");
                sys_stat_with_cache(&old_path, &mut cache).expect("cache warmup should succeed");
                (cache, old_path, new_path)
            },
            |(mut cache, old_path, new_path)| {
                black_box(sys_rename_with_cache(
                    old_path.as_path(),
                    new_path.as_path(),
                    &mut cache,
                ))
                .expect("rename should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_raw_path_lookup_cache, bench_syscall_path_cache_integration);
criterion_main!(benches);

//! Benchmarks for directory entry cache optimization (Day 11).

use std::path::PathBuf;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use vantis_verified::syscall_dir_ops::{
    sys_chdir_with_cache, sys_getcwd_with_cache, sys_mkdir_with_cache, sys_rmdir_with_cache,
    DirectoryEntryCacheEntry, DirectoryEntryPathCache, WorkingDirectory,
};

fn bench_raw_directory_entry_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("directory_entry_cache_raw");

    group.bench_function("hit", |b| {
        let mut cache = DirectoryEntryPathCache::new(1024);
        let key = PathBuf::from("/usr/bin");
        cache.insert(key.as_path(), DirectoryEntryCacheEntry::directory());

        b.iter(|| black_box(cache.get(black_box(key.as_path()))));
    });

    group.bench_function("miss", |b| {
        let mut cache = DirectoryEntryPathCache::new(1024);
        let key = PathBuf::from("/missing/path");

        b.iter(|| black_box(cache.get(black_box(key.as_path()))));
    });

    group.bench_function("insert_and_evict", |b| {
        b.iter(|| {
            let mut cache = DirectoryEntryPathCache::new(256);
            for idx in 0..512u64 {
                let path = PathBuf::from(format!("/tree/{idx}"));
                cache.insert(path.as_path(), DirectoryEntryCacheEntry::directory());
            }
            black_box(cache.stats().evictions);
        });
    });

    group.finish();
}

fn bench_directory_syscall_cache_integration(c: &mut Criterion) {
    let mut group = c.benchmark_group("directory_entry_cache_syscalls");

    group.bench_function("sys_chdir_with_cache_hit", |b| {
        let mut wd = WorkingDirectory::new();
        let mut cache = DirectoryEntryPathCache::new(512);
        let path = PathBuf::from("/usr/local/bin");
        sys_chdir_with_cache(&mut wd, path.as_path(), &mut cache)
            .expect("cache warmup for chdir should succeed");

        b.iter(|| {
            black_box(sys_chdir_with_cache(
                &mut wd,
                black_box(path.as_path()),
                &mut cache,
            ))
            .expect("cached chdir should succeed");
        });
    });

    group.bench_function("sys_getcwd_with_cache", |b| {
        let wd = WorkingDirectory::new();
        let mut cache = DirectoryEntryPathCache::new(256);
        let mut buf = [0u8; 256];
        let size = buf.len();

        b.iter(|| {
            black_box(sys_getcwd_with_cache(
                black_box(&wd),
                black_box(&mut buf),
                black_box(size),
                black_box(&mut cache),
            ))
            .expect("getcwd should succeed");
        });
    });

    group.bench_function("sys_mkdir_with_cache", |b| {
        b.iter_batched(
            || {
                let cache = DirectoryEntryPathCache::new(128);
                let path = PathBuf::from("/tmp/newdir");
                (cache, path)
            },
            |(mut cache, path)| {
                black_box(sys_mkdir_with_cache(path.as_path(), Some(0o755), &mut cache))
                    .expect("mkdir should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("sys_rmdir_with_cache", |b| {
        b.iter_batched(
            || {
                let mut cache = DirectoryEntryPathCache::new(128);
                let path = PathBuf::from("/tmp/demo");
                let _ = sys_mkdir_with_cache(path.as_path(), Some(0o755), &mut cache);
                cache
            },
            |mut cache| {
                let root = PathBuf::from("/tmp");
                black_box(sys_rmdir_with_cache(root.as_path(), &mut cache)).expect("rmdir should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_raw_directory_entry_cache,
    bench_directory_syscall_cache_integration
);
criterion_main!(benches);

# Week 7 Day 11: Directory Entry Caching Report

**Date**: 2026-02-10  
**Status**: Completed  
**Scope**: Directory cache design, syscall integration, cache coherency, tests, benchmarks

---

## 1) Objective

Day 11 goal was to introduce a dedicated directory entry cache to improve directory-oriented syscall paths and establish coherency rules for create/remove/navigation operations.

Target tasks:

1. Design directory cache.
2. Implement cache structure.
3. Integrate with directory syscalls.
4. Add cache coherency behavior.
5. Test and benchmark.

All tasks were completed in this iteration.

---

## 2) Implementation summary

### 2.1 New cache model in `syscall_dir_ops.rs`

Added:

- `DIRECTORY_ENTRY_CACHE_CAPACITY` (default: 256)
- `DirectoryEntryCacheEntry { is_directory: bool }`
- `DirectoryEntryPathCache = PathLookupCache<DirectoryEntryCacheEntry>`
- Global cache singleton:
  - `OnceLock<Mutex<DirectoryEntryPathCache>>`
- Global cache API:
  - `directory_entry_cache_stats()`
  - `reset_directory_entry_cache()`
  - `invalidate_directory_entry_cache(path)`
  - `invalidate_directory_entry_cache_prefix(prefix)`

### 2.2 Syscall integration (`*_with_cache` + wrappers)

Added cache-aware forms:

- `sys_mkdir_with_cache`
- `sys_rmdir_with_cache`
- `sys_chdir_with_cache`
- `sys_getcwd_with_cache`

Kept compatibility wrappers:

- `sys_mkdir`
- `sys_rmdir`
- `sys_chdir`
- `sys_getcwd`

Wrappers use global directory cache and preserve existing file-stat cache coherency behavior through:

- `invalidate_path_lookup_cache`
- `invalidate_path_lookup_cache_prefix`

### 2.3 Coherency policy

Implemented coherency semantics:

1. **mkdir**:
   - Insert created directory path in directory cache.
   - Invalidate parent directory entry in directory cache.
   - Invalidate corresponding file-stat cache entries.
2. **rmdir**:
   - Invalidate removed subtree via prefix invalidation.
   - Invalidate parent directory entry.
   - Invalidate corresponding file-stat cache entries.
3. **chdir**:
   - Resolve and normalize path against current working directory.
   - Use cache hit fast-path for directory validation metadata.
   - Insert resolved target on miss.
4. **getcwd**:
   - Warm/cache current working directory entry.

---

## 3) Tests executed

### 3.1 Directory syscall-focused tests

Command:

```bash
cd src/verified
cargo test --locked syscall_dir_ops
```

Result:

- 22 passed
- 0 failed

### 3.2 New/updated cache tests

Key validations added:

- repeated `chdir` updates hit/miss stats deterministically (delta-based assertions),
- mkdir/rmdir cache coherency with prefix invalidation,
- getcwd cache hit/miss accounting,
- global prefix invalidation behavior.

---

## 4) Benchmarking

### 4.1 New benchmark target

Added:

- `benches/directory_entry_cache_benchmark.rs`
- registered in `src/verified/Cargo.toml`
- wired into `scripts/run_benchmarks.sh` (`--syscall` and `--all` paths)

Command:

```bash
cd src/verified
cargo bench --bench directory_entry_cache_benchmark -- --save-baseline directory_entry_cache_day11
```

### 4.2 Observed results (snapshot)

Raw cache:

- `directory_entry_cache_raw/hit`: ~38.7 ns
- `directory_entry_cache_raw/miss`: ~1.01 ns
- `directory_entry_cache_raw/insert_and_evict`: ~2.97 ms

Syscall integration:

- `sys_chdir_with_cache_hit`: ~108.6 ns
- `sys_getcwd_with_cache`: ~31.3 ns
- `sys_mkdir_with_cache`: ~146.3 ns
- `sys_rmdir_with_cache`: ~222.4 ns

Interpretation:

- Fast-path directory lookup and cwd operations are in low-nanosecond range in synthetic benchmark conditions.
- Mutation operations remain sub-microsecond in this benchmark model.

---

## 5) Validation and repository checks

Executed:

```bash
./scripts/verify_repo.sh
```

Result:

- Passed checks: 57
- Warnings: 0
- Errors: 0

---

## 6) Day 11 conclusion

Day 11 goals are complete:

- Dedicated directory entry cache implemented.
- Directory syscalls integrated with cache-aware variants.
- Cache coherency behavior added for create/remove/navigation paths.
- Unit/integration syscall tests passed.
- Dedicated benchmark created and executed.

This prepares Day 12 work (timer queue optimization) with directory-path hot path improvements already in place.


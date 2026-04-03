use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_bench::{hashmap_ops, json_ops, matrix, parsing, primes, sorting};
use std::time::Duration;

// ─── 1. Prime Numbers ────────────────────────────────────────────────────────

fn bench_primes(c: &mut Criterion) {
    c.bench_function("primes/sum_primes_10M", |b| {
        b.iter(|| primes::sum_primes_sieve(black_box(10_000_000)))
    });
}

// ─── 2. Sorting ───────────────────────────────────────────────────────────────

fn bench_sorting(c: &mut Criterion) {
    let arr = sorting::generate_array(42, 1_000_000);
    // iter_batched gives each iteration a fresh clone so we measure sort only.
    c.bench_function("sorting/sort_1M", |b| {
        b.iter_batched(
            || arr.clone(),
            |data| sorting::sort_array(data),
            criterion::BatchSize::LargeInput,
        )
    });
}

// ─── 3. JSON Serialization ────────────────────────────────────────────────────

fn bench_json(c: &mut Criterion) {
    let records = json_ops::generate_records(100_000);
    let json = json_ops::serialize(&records);

    let mut group = c.benchmark_group("json");
    group.bench_function("serialize_100K", |b| {
        b.iter(|| json_ops::serialize(black_box(&records)))
    });
    group.bench_function("deserialize_100K", |b| {
        b.iter(|| json_ops::deserialize(black_box(&json)))
    });
    group.finish();
}

// ─── 4. Matrix Multiplication ─────────────────────────────────────────────────

fn bench_matrix(c: &mut Criterion) {
    let a = matrix::generate_matrix(42, 512);
    let b = matrix::generate_matrix(43, 512);

    let mut group = c.benchmark_group("matrix");
    // 512³ ≈ 134M FMAs; allow generous measurement time.
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);
    group.bench_function("multiply_512x512", |bencher| {
        bencher.iter(|| matrix::multiply(black_box(&a), black_box(&b)))
    });
    group.finish();
}

// ─── 5. String Parsing ────────────────────────────────────────────────────────

fn bench_parsing(c: &mut Criterion) {
    // Generate once; measure only the parsing operations.
    let text = parsing::generate_text(50 * 1024 * 1024); // 50 MB

    let mut group = c.benchmark_group("parsing");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);
    group.bench_function("count_words_50MB", |b| {
        b.iter(|| parsing::count_words(black_box(&text)))
    });
    group.bench_function("to_uppercase_50MB", |b| {
        b.iter(|| parsing::to_uppercase(black_box(&text)))
    });
    group.finish();
}

// ─── 6. HashMap Operations ────────────────────────────────────────────────────

fn bench_hashmap(c: &mut Criterion) {
    let map = hashmap_ops::insert_entries(1_000_000);

    let mut group = c.benchmark_group("hashmap");
    group.bench_function("insert_1M", |b| {
        b.iter(|| hashmap_ops::insert_entries(black_box(1_000_000)))
    });
    group.bench_function("lookup_1M", |b| {
        b.iter(|| hashmap_ops::lookup_entries(black_box(&map), black_box(1_000_000)))
    });
    group.finish();
}

// ─── Registration ─────────────────────────────────────────────────────────────

criterion_group!(
    benches,
    bench_primes,
    bench_sorting,
    bench_json,
    bench_matrix,
    bench_parsing,
    bench_hashmap
);
criterion_main!(benches);

# Benchmark Results — Rust vs C# .NET 10

> **Environment**: Fill in after running benchmarks.  
> **Date**: ___________  
> **CPU**: ___________  
> **RAM**: ___________  
> **OS**: ___________  
> **Rust**: ___________  
> **.NET SDK**: 10.0.201  

---

## Executive Summary

| # | Category | Rust (mean) | C# (mean) | Faster | Δ % |
|---|----------|-------------|-----------|--------|-----|
| 1 | Prime Numbers (10M) | — | — | — | — |
| 2 | Sorting (1M elements) | — | — | — | — |
| 3 | JSON Serialize (100K records) | — | — | — | — |
| 4 | JSON Deserialize (100K records) | — | — | — | — |
| 5 | Matrix Multiply (512×512) | — | — | — | — |
| 6 | String Count Words (50 MB) | — | — | — | — |
| 7 | String ToUppercase (50 MB) | — | — | — | — |
| 8 | HashMap Insert (1M) | — | — | — | — |
| 9 | HashMap Lookup (1M) | — | — | — | — |

---

## Detailed Results

### 1. Prime Numbers — Sieve of Eratosthenes (N = 10,000,000)

| Metric | Rust | C# |
|--------|------|----|
| Mean time | — | — |
| Std Dev | — | — |
| Min | — | — |
| Max | — | — |
| Memory alloc | — | — |

**Notes**: _Both use identical sieve algorithm._

---

### 2. Sorting — `sort_unstable` / `Array.Sort` (1,000,000 × u32/int, seed 42)

| Metric | Rust | C# |
|--------|------|----|
| Mean time | — | — |
| Std Dev | — | — |
| Memory alloc | — | — |

**Notes**: _Rust uses pdqsort (`sort_unstable`). C# uses introsort (`Array.Sort`)._

---

### 3 & 4. JSON Serialization / Deserialization (100,000 records)

| Operation | Rust (serde_json) | C# (System.Text.Json) |
|-----------|-------------------|-----------------------|
| Serialize mean | — | — |
| Deserialize mean | — | — |
| Serialize alloc | — | — |
| Deserialize alloc | — | — |

---

### 5. Matrix Multiplication — Naïve O(N³) (512×512 f64)

| Metric | Rust | C# |
|--------|------|----|
| Mean time | — | — |
| Std Dev | — | — |
| Memory alloc | — | — |

**Notes**: _Both use ikj loop order. Rust uses `Vec<Vec<f64>>`, C# uses `double[,]`._

---

### 6 & 7. String Parsing (50 MB synthetic text)

| Operation | Rust | C# |
|-----------|------|----|
| Count Words mean | — | — |
| ToUppercase mean | — | — |
| ToUppercase alloc | — | — |

---

### 8 & 9. HashMap Operations (1,000,000 entries)

| Operation | Rust (`HashMap`) | C# (`Dictionary`) |
|-----------|-----------------|-------------------|
| Insert mean | — | — |
| Lookup mean | — | — |
| Insert alloc | — | — |

---

## Observations & Anomalies

_Fill in after running._

---

## How to Run

### Rust
```powershell
cd rust_bench
cargo bench
# HTML reports → rust_bench/target/criterion/report/index.html
```

### C#
```powershell
cd csharp_bench
dotnet run -c Release -- --filter *
# Results → csharp_bench/BenchmarkDotNet.Artifacts/results/
```

### Run a single category
```powershell
# Rust: single group
cargo bench -- primes

# C#: single benchmark class
dotnet run -c Release -- --filter *Prime*
```

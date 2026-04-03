# Benchmark Results — Rust vs C# .NET 10

> **Environment**: Windows 10 (10.0.19045), Intel Core i7-10510U 1.80GHz
> **Date**: 2026-04-03
> **OS**: Windows 10
> **Rust**: stable
> **.NET SDK**: 10.0.201

---

## Executive Summary

| # | Category | Rust (mean) | C# (mean) | Faster |
|---|----------|-------------|-----------|--------|
| 1 | Prime Numbers (10M) | *Pendiente* | **99.60 ms** | - |
| 2 | Sorting (1M elements) | *Pendiente* | **108.3 ms** | - |
| 3 | JSON Serialize (100K) | *Pendiente* | **77.79 ms** | - |
| 4 | JSON Deserialize (100K) | **93.34 ms** | 211.98 ms | **Rust** (~2.2x) |
| 5 | Matrix Multiply (512x512)| **197.86 ms** | 438.00 ms | **Rust** (~2.2x) |
| 6 | String Count Words | **66.74 ms** | 70.00 ms | **Rust** (~1.1x) |
| 7 | String ToUppercase | **28.45 ms** | 40.11 ms | **Rust** (~1.4x) |
| 8 | HashMap Insert (1M) | 68.89 ms | **11.65 ms** | **C#** (~5.9x) |
| 9 | HashMap Lookup (1M) | 111.95 ms | **6.60 ms** | **C#** (~17.0x) |

---

## Observaciones

1. **Eficiencia en HashMaps (.NET vs Rust)**: Los resultados demuestran una ventaja masiva para C# en cuanto a inserción y búsqueda de diccionarios (Dictonary nativo contra HashMap). .NET 10 parece estar fuertemente optimizado para el uso agresivo de memoria manejada en este tipo de colecciones por sobre la versión genérica en Rust.
2. **Capacidad de Cómputo e Indirección**: Como era de esperarse, Rust es visiblemente más veloz (casi el doble o más) para tareas de indirección, parsing complejo y des-serialización gracias a su ausencia de pausas por recolección de basura (Garbage Collector) y su control superior del layout de la memoria RAM.

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

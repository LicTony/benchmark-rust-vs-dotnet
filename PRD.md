# PRD: Rust vs C# .NET 10 Benchmark Suite

## 1. Overview

### 1.1 Objective
Create a comprehensive benchmark suite comparing performance between **Rust** and **C# .NET 10** across multiple computational workloads.

### 1.2 Scope
- Two independent projects: `rust_bench` and `csharp_bench`
- Identical algorithms implemented in both languages
- Fair comparison with optimized release builds
- Multiple benchmark categories beyond prime numbers

### 1.3 Success Criteria
- Reproducible, statistically significant benchmark results
- Clear documentation of methodology and environment
- Side-by-side comparison of execution time and memory usage
- At least 5 different benchmark categories

---

## 2. Technical Specifications

### 2.1 Rust Project (`rust_bench`)

#### Dependencies
```toml
[dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
rayon = "1.8"  # For parallel benchmarks (optional)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[[bench]]
name = "benchmark_suite"
harness = false
```

#### Benchmark Categories
| # | Category | Description | Input Size |
|---|----------|-------------|------------|
| 1 | **Prime Numbers** | Sum all primes up to N | 10M |
| 2 | **Sorting** | Sort random arrays | 1M elements |
| 3 | **JSON Serialization** | Serialize/deserialize complex object | 100K records |
| 4 | **Matrix Multiplication** | Multiply two NxN matrices | 1000x1000 |
| 5 | **String Parsing** | Parse and transform large text | 50MB file |
| 6 | **Hash Map Operations** | Insert/lookup 1M entries | 1M entries |

#### Project Structure
```
rust_bench/
├── Cargo.toml
├── benches/
│   └── benchmark_suite.rs
├── src/
│   ├── lib.rs          # Core algorithms
│   ├── primes.rs
│   ├── sorting.rs
│   ├── json_ops.rs
│   ├── matrix.rs
│   ├── parsing.rs
│   └── hashmap_ops.rs
└── target/             # Build artifacts
```

---

### 2.2 C# .NET 10 Project (`csharp_bench`)

#### Dependencies
```xml
<PackageReference Include="BenchmarkDotNet" Version="0.13.12" />
<PackageReference Include="System.Text.Json" Version="8.0.0" />
```

#### Project Structure
```
csharp_bench/
├── csharp_bench.csproj
├── Program.cs
├── Benchmarks/
│   ├── PrimeBenchmark.cs
│   ├── SortingBenchmark.cs
│   ├── JsonBenchmark.cs
│   ├── MatrixBenchmark.cs
│   ├── ParsingBenchmark.cs
│   └── HashMapBenchmark.cs
├── Core/
│   ├── PrimeEngine.cs
│   ├── SortEngine.cs
│   ├── JsonEngine.cs
│   ├── MatrixEngine.cs
│   ├── ParseEngine.cs
│   └── HashMapEngine.cs
└── bin/                # Build artifacts
```

---

## 3. Benchmark Methodology

### 3.1 Environment Configuration
```
Hardware:
- CPU: [Document processor model]
- RAM: [Document available RAM]
- OS: Windows 11 / Linux [specify]

Software:
- Rust: 1.75+ (stable)
- .NET SDK: 10.0.x
- Criterion: 0.5.x
- BenchmarkDotNet: 0.13.12+
```

### 3.2 Execution Guidelines

#### Pre-Execution Checklist
- [ ] Close all non-essential applications
- [ ] Disable real-time antivirus scanning
- [ ] Set power plan to "High Performance"
- [ ] Ensure no background updates/telemetry
- [ ] Run at least 3 iterations per benchmark
- [ ] Use Release builds exclusively

#### Rust Execution
```powershell
cd rust_bench
cargo bench  # Automatically uses Release mode
```

#### C# Execution
```powershell
cd csharp_bench
dotnet build -c Release
dotnet run -c Release -- --filter *  # Run all benchmarks
```

### 3.3 Statistical Validity
- **Warmup iterations**: Minimum 5
- **Actual iterations**: Minimum 10
- **Outlier rejection**: Use statistical methods (MAD, IQR)
- **Confidence interval**: 95% minimum

---

## 4. Expected Deliverables

### 4.1 Source Code
- Complete Rust project with all benchmarks
- Complete C# project with all benchmarks
- Identical test data (seeded random generators)

### 4.2 Results Documentation
| File | Content |
|------|---------|
| `results/summary.md` | Executive summary table |
| `results/rust-output.txt` | Criterion HTML report links |
| `results/csharp-output.txt` | BenchmarkDotNet console output |
| `results/comparison.png` | Visual comparison chart |

### 4.3 Comparison Metrics
For each benchmark category, report:
- **Mean execution time** (ms/μs)
- **Standard deviation**
- **Throughput** (operations/sec)
- **Memory allocation** (bytes)
- **GC pressure** (C# specific)
- **Peak RSS** (Rust specific, if available)

---

## 5. Acceptance Criteria

### 5.1 Functional Requirements
- [ ] Both projects compile without warnings
- [ ] All 6 benchmark categories implemented in both languages
- [ ] Identical input data across implementations
- [ ] Results are reproducible (±2% variance max)
- [ ] Memory measurements included

### 5.2 Quality Requirements
- [ ] Code follows idiomatic patterns for each language
- [ ] No artificial throttling or optimization barriers
- [ ] Algorithms have equivalent complexity (same Big-O)
- [ ] All benchmarks complete in <5 minutes each
- [ ] HTML reports generated for visual analysis

### 5.3 Documentation Requirements
- [ ] Setup instructions for both projects
- [ ] Environment specifications documented
- [ ] Raw benchmark output files preserved
- [ ] Comparison summary with percentage differences
- [ ] Lessons learned and anomalies noted

---

## 6. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| JIT warmup differences | High | Use built-in warmup from both frameworks |
| OS background processes | Medium | Document and minimize interference |
| Algorithm implementation differences | High | Code review both implementations |
| Memory measurement inconsistency | Medium | Use Task Manager + dotMemory for C#, criterion for Rust |
| Different random seed usage | Low | Use hardcoded seed (42) for all RNG |

---

## 7. Timeline & Phases

### Phase 1: Prime Numbers Baseline ✅ (Current)
- Implement `is_prime()` and `sum_primes()` in both languages
- Validate benchmark framework setup
- Establish baseline comparison methodology

### Phase 2: Additional Benchmarks
- Sorting (QuickSort/MergeSort)
- Matrix multiplication
- JSON operations

### Phase 3: Advanced Benchmarks
- String parsing
- Hash map operations
- (Optional) Web server: Axum vs Minimal API

### Phase 4: Analysis & Reporting
- Run final benchmarks in controlled environment
- Generate comparison charts
- Document findings and anomalies

---

## 8. References

### Useful Commands

**Rust:**
```powershell
cargo bench                     # Run all benchmarks
cargo bench -- --noplot        # Disable plot generation
cargo build --release          # Release build only
```

**C# .NET 10:**
```powershell
dotnet build -c Release
dotnet run -c Release
dotnet run -c Release -- --filter PrimeBenchmark  # Run specific benchmark
```

**Comparison:**
```powershell
# After running both, compare results
cat rust_bench/target/criterion/report/index.html
cat csharp_bench/BenchmarkDotNet.Artifacts/results/*.md
```

---

## 9. Appendix

### 9.1 Prime Number Algorithm (Reference Implementation)
```rust
fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt = (n as f64).sqrt() as u32;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}
```

### 9.2 Memory Measurement Tools
- **Rust**: `criterion` + Windows Task Manager, or `valgrind` (Linux only)
- **C#**: `BenchmarkDotNet` (built-in), `dotMemory`, or Visual Studio Profiler

### 9.3 Additional Resources
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [BenchmarkDotNet Documentation](https://benchmarkdotnet.org/)
- [.NET 10 Release Notes](https://learn.microsoft.com/en-us/dotnet/core/whats-new/dotnet-10)
- [Rust Performance Tips](https://nnethercote.github.io/perf-book/)

---

**Document Version**: 1.0  
**Created**: 2026-04-03  
**Status**: Draft - Ready for Implementation

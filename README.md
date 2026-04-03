# Benchmark: Rust vs C# .NET 10

Este repositorio contiene una suite completa de pruebas de rendimiento (benchmarks) diseñada para comparar **Rust** y **C# (.NET 10)**. El objetivo es medir y contrastar el rendimiento de ambos lenguajes bajo cargas de trabajo algorítmicas idénticas utilizando implementaciones nativas optimizadas.

## 🚀 Características Principales

Se utilizan las herramientas estándar de benchmarking de cada ecosistema (`Criterion` para Rust y `BenchmarkDotNet` para C#) para asegurar tiempos de calentamiento adecuados y alta precisión estadística.

Las pruebas están divididas en 6 categorías computacionales:

1. **Números Primos (Sieve of Eratosthenes)**: Evaluación de pura capacidad de cómputo iterativo y manipulación de arreglos booleanos (N = 10.000.000).
2. **Ordenamiento de Arreglos**: Comparación de algoritmos introspectivos nativos (`sort_unstable` en Rust vs `Array.Sort` en C#) utilizando un generador pseudoaleatorio predecible para igualar la carga (1.000.000 de enteros).
3. **Serialización JSON**: Operaciones de parsing usando bibliotecas estándar modernas (`serde_json` en Rust vs `System.Text.Json` en C#) con una estructura de datos mixta de 100.000 registros.
4. **Multiplicación de Matrices**: Test de localidad de caché y rendimiento en arreglos multidimensionales (Naïve O(N³) con bucle interior ikj sobre 512x512).
5. **Parseo de Cadenas de Texto**: Evaluaciones sobre alojamiento temporal y manipulación de texto in-memory generado sintéticamente (Word counting y ToUppercase sobre un buffer de 50 MB).
6. **Operaciones en HashMaps**: Test de dispersión e indirección en colección clave-valor estándar nativa, evaluando inserciones masivas y consultas continuas (1.000.000 entradas).

## 📁 Estructura del Repositorio

* `/rust_bench`: Proyecto de cargo configurado para optimización máxima empleando el framework `criterion`.
* `/csharp_bench`: Solución .NET 10 configurada con `.csproj` apuntando a `BenchmarkDotNet` con parámetros estrictos contra la compilación JIT/AOT.
* `/results/summary.md`: Documento de evaluación técnica con el compendio e insights obtenidos al ejecutar la suite.

## ⚙️ Cómo Ejecutar

Asegúrate de contar con la versión nativa de la cadena de compilación de Rust (1.75+) y el SDK Preview de .NET 10.

**Para ejecutar las pruebas en Rust:**
```powershell
cd rust_bench
cargo bench
```

**Para ejecutar las pruebas en C#:**
```powershell
cd csharp_bench
dotnet run -c Release -- --filter *
```

*(Nota: Ejecuta las pruebas en modo 'Alto Rendimiento' del sistema operativo, con todas las demás aplicaciones cerradas para no alterar la precisión)*.

## 📝 Resultado (Resumen Orientativo)

Ambos proyectos producen resultados altamente detallados bajo `target/criterion/report` y `BenchmarkDotNet.Artifacts`, los cuales son consolidados posteriormente en la carpeta `/results` del repositorio con una comparativa porcentual y análisis de tiempos medios de ejecución de instrucciones y métricas de consumo en memoria.

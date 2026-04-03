using BenchmarkDotNet.Running;
using System.Reflection;

// Usage:
//   dotnet run -c Release                          → detect benchmark (interactive)
//   dotnet run -c Release -- --filter *            → run all
//   dotnet run -c Release -- --filter *Prime*      → run prime only
BenchmarkSwitcher.FromAssembly(Assembly.GetEntryAssembly()!).Run(args);

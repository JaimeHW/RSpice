# 8 · Performance

## The benchmark rig

`rspice-bench` measures end-to-end wall clock against a local ngspice
over the deck set in `benchmarks/circuits/` and writes a JSON
scoreboard. Methodology, conventions, and the published baselines live
in [benchmarks/README.md](../../benchmarks/README.md) and
`diagnostics/benchmarks/`. House rule: **no optimization claim without a
before/after scoreboard from the rig.**

```sh
cargo build --release -p rspice-cli -p rspice-bench
RSPICE_BENCH_NGSPICE=/path/to/ngspice target/release/rspice-bench run
```

## Practical levers for long runs

- **Release build.** Fat LTO + single codegen unit are part of the
  release profile; debug builds are several times slower.
- **`--compress`** caps waveform memory and file size for long
  transients (lossy within `--compress-tol`).
- **Checkpointing** (`--checkpoint` / `--restore`) splits very long
  transients into segments without losing integrator state.
- **Output selection** (`.save`/`.probe`) trims recording overhead on
  large circuits.

## Linear-solver backends

The default solver is faer's sparse LU with a cached symbolic
factorization. An experimental KLU-class backend is selectable per
process:

```sh
RSPICE_SOLVER=klu rspice run big_postlayout.cir
```

It exploits the circuit-simulation workload — one frozen sparsity
pattern factored many times with changing values — with a stored pivot
sequence and values-only refactorization (minimum-degree ordering,
Gilbert-Peierls factorization, diagonal-preference threshold pivoting,
pivot-growth alarm with automatic re-pivoting, and a transparent
fallback to the default path on any backend failure). Conformance
suites run identically under either backend; the scoreboard decides
when it becomes the default.

## Parallelism

AC frequency sweeps and Monte Carlo runs parallelize across cores by
default (deterministic per-run RNG streams keep statistical results
reproducible regardless of thread schedule). Transient device
evaluation is intentionally serial for now: measured fork/join overhead
exceeds the ~100 ns cost of typical compact-model evaluations, and the
planned persistent-worker design is tracked in the roadmap (WS3).

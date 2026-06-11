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

The default real-valued solver is a KLU-class backend built for the
circuit-simulation workload — one frozen sparsity pattern factored many
times with changing values: minimum-degree ordering, Gilbert-Peierls
factorization with diagonal-preference threshold pivoting, and a
values-only refactorization that reuses the stored pivot sequence (a
pivot-growth alarm re-pivots automatically, and any backend failure
falls through to the faer path transparently). On the published
scoreboards it improves solver-bound decks 14-15% end-to-end while the
full conformance suite reproduces the previous baseline exactly.

```sh
RSPICE_SOLVER=faer rspice run deck.cir    # opt out to the faer backend
```

AC and other complex-valued solves use faer's sparse LU with a cached
symbolic factorization shared from the real matrix.

## Parallelism

AC frequency sweeps and Monte Carlo runs parallelize across cores by
default (deterministic per-run RNG streams keep statistical results
reproducible regardless of thread schedule). Transient device
evaluation is intentionally serial for now: measured fork/join overhead
exceeds the ~100 ns cost of typical compact-model evaluations, and the
planned persistent-worker design is tracked in the roadmap (WS3).

# rspice-matrix

Sparse matrix storage and factorization for RSpice: a frozen CSC topology with
O(1) stamping handles, a KLU-class real LU, faer-backed complex and fallback
LU, and the dense eigensolve the periodic analyses share. It is a crate of its
own, depending on no other RSpice crate, because the generated Verilog-A
models stamp into these same matrices through `rspice-veriloga-runtime`: were
the types inside `rspice-core`, every generated model would have to be a
source module of the engine rather than an artifact compiled once. The
KLU-class kernel is written in Rust and does not link or wrap the SuiteSparse
KLU C library.

## What's in here

- **`sparse`**: `StaticMatrix` (real, frozen pattern), `ComplexMatrix` (the
  same structure with complex values, for the AC-family analyses),
  `TripletMatrix` for accumulating stamps, the `CscIndex`/`CscPatternToken`
  stamp handles, componentwise backward-error certification, explicit dense
  and extended-precision fallback solves, and `SparseLuSolver`, which keeps the
  pattern and analysis of the last matrix it solved until the pattern changes.
- **`klu`**: `KluSolver`, with maximum structural matching, block-triangular form
  and AMD fill reduction inside each irreducible block, computed once per
  pattern; Gilbert-Peierls LU with diagonal-preference threshold pivoting
  confined to each block; allocation-free values-only refactorization on the
  stored pivots; and triangular solves, transposed and batched included.
- **`dense_eigen`**: `qualified_dense_eigenspectrum`, a complete dense
  eigenspectrum with a per-eigenpair residual check. It lives here so faer's
  generic eigen code compiles in a package the workspace optimizes in its
  development and test profiles.
- **Crate root**: `SolverOptions` and its policy enums
  (`RealSolverBackend`, `NumericFactorizationPolicy`, the circuit-LU
  orientation, scaling and robustness selectors), and `SolverError`.

## Boundaries

- The Newton loop, damping and convergence aids that drive these solves belong
  to `rspice-core`, whose `solver` module re-exports the matrix and solver
  types.
- Generated Verilog-A stamps reach these matrices through the stampers in
  `rspice-veriloga-runtime`.

## Entry points

- `StaticMatrix::from_triplets_with_options`: freeze a topology under an
  explicit `SolverOptions`. `StaticMatrix::from_triplets` reads
  `SolverOptions::from_env()` instead (`RSPICE_SOLVER`, `RSPICE_PIVREL`,
  `RSPICE_PIVTOL`); embedding code should pass options so process
  environment cannot change numeric policy.
- `StaticMatrix::get_index` and `stamp_direct`: resolve a stamp location once,
  then accumulate into it with no search.
- `StaticMatrix::solve_into`: solve into a caller-owned vector, reusing it and
  the internal workspaces after warm-up.
- `ComplexMatrix::from_real_structure`: the complex twin of a real topology.
- `StaticMatrixSolverCheckpoint`: the retained factor state a transient
  checkpoint carries, so a resumed run continues on the same factors.

```rust
use rspice_matrix::{RealSolverBackend, SolverOptions, StaticMatrix};

let triplets = [
    (0, 0, 4.0),
    (1, 0, -1.0),
    (0, 1, -1.0),
    (1, 1, 3.0),
];
let mut matrix = StaticMatrix::from_triplets_with_options(
    2,
    2,
    &triplets,
    SolverOptions {
        real_backend: RealSolverBackend::Klu,
        ..SolverOptions::default()
    },
)?;

let diagonal = matrix.get_index(0, 0).expect("topology contains diagonal");
matrix.clear_values();
matrix.stamp_direct(diagonal, 4.0);
matrix.add(1, 0, -1.0);
matrix.add(0, 1, -1.0);
matrix.add(1, 1, 3.0);

let mut solution = Vec::new();
matrix.solve_into(&[1.0, 2.0], &mut solution)?;
# Ok::<(), rspice_matrix::SolverError>(())
```

`SolverOptions::default()` is deterministic and selects
`RealSolverBackend::Auto`: the KLU-class kernel, except for patterns whose
measured fill favors faer's supernodal LU. Under the default robustness
policy, any circuit-LU solve that cannot certify its result falls back to
equilibrated faer. A complex factorization is reused until the matrix's values
are next written, so a noise analysis solving many right-hand sides factors
once.

A successful sparse solve satisfies a scale-invariant componentwise
backward-error bound; a finite but inaccurate candidate is refined or
rejected, never returned. Finiteness is checked again at every solve
boundary, including after unchecked hot-path stamping, and a `CscIndex` from
another topology is refused rather than written.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `faer-parallel` | faer's Rayon-parallel factorization. WebAssembly consumers leave it off: Rayon is unavailable on the baseline `wasm32-unknown-unknown` target |

## Testing

```text
cargo test -p rspice-matrix --all-features
cargo doc -p rspice-matrix --no-deps
cargo run --release -p rspice-bench -- klu
```

This file is the crate's rustdoc: `cargo test` runs the example above as a
doctest, and `cargo doc` fails on a broken intra-doc link in it. The `klu`
benchmark times analyze, factor, refactor and solve on circuit-shaped
patterns, checks each case's backward error outside the timed region, and
gates only on budgets passed to it: fill, absolute time per LU nonzero, or
refactor and solve cost relative to a full factorization. CI runs the fill and
relative gates and uploads the JSON report.

Licensed under the [RSpice Personal Use License](../../LICENSE).

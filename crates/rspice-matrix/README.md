# rspice-matrix

`rspice-matrix` is RSpice's dependency-neutral sparse matrix package. It owns
the frozen CSC topology, O(1) stamping handles, real KLU-class factorization,
equilibrated faer fallback, complex sparse LU, residual evaluation, and dense
reference fallback used by the simulator.

## Solver pipeline

For a real square system, the default policy performs:

1. one-time CSC validation and structural ownership;
2. maximum structural matching and block-triangular decomposition (BTF);
3. AMD fill reduction inside every irreducible diagonal block;
4. Gilbert-Peierls LU with threshold pivoting constrained to each BTF block;
5. allocation-free values-only refactorization on later Newton iterations;
6. triangular solves using cached diagonal reciprocals;
7. componentwise backward-error validation and iterative refinement; and
8. equilibrated faer LU fallback if KLU cannot produce an accepted result.

Complex systems use equilibrated faer sparse LU with the same backward-error
acceptance and iterative-refinement policy. A complex factorization is reused
while matrix values remain unchanged, which is important for noise analyses
with many right-hand sides.

The implementation is KLU-class and written in Rust; it does not link or wrap
the SuiteSparse KLU C library.

## Recommended API

Build topology once, retain direct stamp tokens, and reuse both the matrix and
the caller-owned output vector:

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

`CscIndex` is bound to its originating topology. Its numeric offset remains
public for source compatibility, while stamping validates the hidden pattern
identity so a token from another topology cannot silently corrupt the matrix.

`SolverOptions::default()` is deterministic and selects KLU. The compatibility
constructor `StaticMatrix::from_triplets` reads `RSPICE_SOLVER=faer`; commercial
embedding code should use `from_triplets_with_options` so process environment
cannot change numeric policy.

## Numeric contract

- Dimensions, CSC pointers, row ordering, duplicate coordinates, and stamp
  tokens are checked at construction or checked-mutation boundaries. Numeric
  finiteness is checked by checked mutators and again at every solve boundary,
  including after unchecked hot-path stamping.
- A successful sparse solve satisfies a scale-invariant componentwise
  backward-error bound proportional to machine epsilon and row sparsity.
- Finite-but-inaccurate candidates are refined or rejected; they are never
  exposed as successful solutions.
- KLU analysis is fail-closed. A rejected new pattern invalidates the prior
  pattern association.
- `solve_into` reuses output and internal workspaces after warmup. `solve` is a
  convenience ownership API and necessarily allocates its returned vector.
- The package contains one narrowly scoped `unsafe` faer LU-reference
  construction. Its symbolic/numeric provenance and mutation invalidation
  invariant are documented at the call site.

## Qualification

Run the package's correctness and adversarial contracts with:

```text
cargo test -p rspice-matrix --all-features
cargo clippy -p rspice-matrix --all-targets --all-features -- -D warnings
cargo doc -p rspice-matrix --no-deps
```

The KLU kernel gate validates every timed solution outside the timed region and
ratchets refactor latency, solve latency, and circuit-pattern fill:

```text
cargo run --release -p rspice-bench -- klu \
  --max-refactor-ns-per-lu-nnz 4.0 \
  --max-solve-ns-per-lu-nnz 2.5 \
  --max-fill-ratio 1.60
```

CI runs this gate and uploads the machine-readable qualification report. The
benchmark includes irreducible ladder/ring matrices, a reducible BTF chain, and
an ungated random-expander reference.

## Current scope

- scalar type: `f64`;
- storage: square or rectangular CSC, with sparse solves requiring square
  systems;
- KLU path: real, single right-hand side, non-transposed solves;
- complex path: faer sparse LU;
- parallel faer execution: optional native-only `faer-parallel` feature; wasm
  consumers leave it disabled because Rayon is not available on the baseline
  `wasm32-unknown-unknown` target.

The workspace MSRV and license are inherited from RSpice's root manifest.

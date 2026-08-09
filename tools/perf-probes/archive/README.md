# Archived compiler performance probes

These standalone crates preserve two bounded experiments used to choose an
older generated-Verilog-A lowering strategy. They are historical design
evidence, not maintained benchmarks, workspace members, CI gates, or current
product-performance claims. Their local `target/` trees and generated
`Cargo.lock` files are intentionally excluded.

## `lowering/`

The probe compares an indexed derivative workspace, a fixed-width `[f64; L]`
array, and flattened scalar lanes while keeping every lane observable. Its
2026-07-27 result was:

| Form | L=12 | L=32 |
| :--- | ---: | ---: |
| workspace | 9.819 ns/op | 23.753 ns/op |
| array | 3.692 ns/op | 4.491 ns/op |
| flat | 3.744 ns/op | unavailable |

The experiment supported fixed-width packed lanes: LLVM promoted the array form
well enough to match flattened scalars and substantially beat the indexed
workspace. Every lane must remain live; returning one lane permits dead-code
elimination and invalidates the comparison.

```text
cargo run --release --manifest-path tools/perf-probes/archive/lowering/Cargo.toml
```

## `split/`

The probe generates a BSIMBulk-scale synthetic packed-lane function either as
one body or as `#[inline(never)]` blocks. Its recorded 2026-07-27 result was:

| Block size | Source | rustc | Runtime |
| ---: | ---: | ---: | ---: |
| monolithic | 1,670 KB | 51.6 s | 25,233 ns/eval |
| 500 | 1,677 KB | 30.2 s | 61,681 ns/eval |
| 2,000 | 1,672 KB | 34.4 s | 61,875 ns/eval |

The result favored a monolithic packed lowering: splitting reduced compilation
time but forced lane arrays across function boundaries and cost about 2.4x at
runtime. Only the ratio was used; the serial synthetic dependency chain is not
a compact-model latency prediction.

```text
cargo run --release --manifest-path tools/perf-probes/archive/split/Cargo.toml \
  -- 12028 17 0 mono.rs
rustc -O -o mono.exe mono.rs
./mono.exe
```

Re-running either probe requires recording toolchain, target CPU, host, raw
samples, and source commit separately. A current production decision should be
validated in the maintained benchmark or generated-model pipeline instead of
promoting these numbers into `benchmarks/baselines/`.

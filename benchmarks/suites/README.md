# Benchmark suites

Every trusted benchmark suite is immutable and explicitly versioned. A suite
manifest names each deck, authenticates its bytes, identifies the analysis and
workload category, and binds it to a correctness contract. Adding, removing, or
changing a deck requires a new `suite_version`; changing timing conventions
requires a new `methodology_version` and a fresh baseline.

`macro-v1` is the native end-to-end suite. Validate it without running a timing
measurement:

```text
cargo run --locked -p rspice-bench -- suite
```

Directory discovery is deliberately unsupported. A stray `.cir` file must not
silently redefine an approved performance gate.

## `macro-v1` inventory

| Deck | Coverage |
| :--- | :--- |
| `divider_ac.cir` | Process startup, parse, output, and dense AC sweep overhead |
| `diode_rectifier.cir` | Nonlinear transient convergence and diode limiting |
| `ring51.cir` | MOS evaluation in a 51-stage oscillating transient |
| `mos_array_4096.cir` | Large independent MOS evaluation/stamp throughput |
| `rc_ladder_100.cir` | Small sparse linear transient |
| `rc_ladder_1000.cir` | Medium sparse stamp/solve balance |
| `rc_ladder_10000.cir` | Large sparse factor/solve scale tier |

The three RC ladders and MOS array are generated. Change
`crates/rspice-bench/src/generate.rs`, run `rspice-bench gen`, and review the
byte diff. The benchmark crate's generator test independently recreates all
four files and requires byte equality. If workload bytes intentionally change,
update the manifest digest and increment `suite_version`; if timing semantics
change, increment `methodology_version` and retire existing baselines.

The divider, rectifier, and ring decks are hand-maintained in the common
RSpice/ngspice dialect. Every deck also has an independent behavior contract in
`crates/rspice-core/tests/macro_benchmark_contract.rs`.

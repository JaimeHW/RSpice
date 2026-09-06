# rspice-conformance

The regression suites that validate RSpice against reference simulators. Each
suite discovers a vendored deck corpus, runs every deck through `rspice_core`,
and compares the result against a reference contract.

## Why this is a separate crate

A conformance suite exists to answer one question: does the simulator a user
gets produce the right numbers? A suite that can reach into engine internals
cannot answer it: assertions on private state survive real numerical
regressions, because they never travel the path a user's deck takes. Living
outside `rspice-core` makes that a compiler-enforced property rather than a
convention: these suites see only the public API, so every assertion is made
through the same surface a frontend uses.

The dependency direction is one-way and is the whole point. This crate depends
on `rspice-core`; nothing a user installs may depend on this crate, which
`tools/ci/test_ci_configuration.py` asserts. `rspice-bench` is the deliberate
exception, a benchmark rig rather than a product, whose generated-stamp timing
harness consumes the hand-written Verilog-A suite.

## Suites, and what each one can actually prove

| Suite | Oracle | Strength |
| :--- | :--- | :--- |
| `suites::ngspice` | another simulator's published output | external conformance |
| `suites::xyce` | another simulator's published output | external conformance |
| `suites::gf180mcu` | per-case ngspice references over a released foundry PDK | external conformance on real process models |
| `suites::veriloga` | RSpice's own captured stamps plus a finite-difference oracle sharing no code with the chain rule under test | snapshot plus independent mathematics |
| `suites::verilog` | Icarus Verilog and Verilator, run live | two independent implementations diffed against each other *before* RSpice is involved |
| `suites::execution` | none; ISCAS85 and the ngspice examples ship no reference data | that a deck loaded, built, and either ran or refused cleanly |

The Verilog-A suite uses snapshots and an independent oracle rather than
external conformance because the thing under test is a code generator, and
there is no second simulator computing the same Jacobian to ask.

`suites::verilog` costs what a checked-in reference does not: with neither
oracle binary installed (the current state everywhere, CI included), it can
only check itself, and it says so loudly rather than passing quietly.

`suites::execution` is the weakest instrument here and says so. It earns its
place because its decks are unlike the others: a reference suite's decks are, by
construction, ones their authors could already simulate, while these are real
circuits reaching for whatever dialect corner they happened to need. Every
failure there is a named gap in ingestion, device coverage, or solver
robustness.

## Corpora

Deck corpora are vendored at the workspace root, not inside this crate:
`tests/ngspice/`, `tests/xyce/`, `tests/gf180mcu/`, `tests/iscas85/`,
`tests/paranoia/`, and `tests/verilog/`. They are shared data with their own
licensing and provenance, and they outlive any one runner. Each carries an
`RSPICE-VENDORING.md` recording what was trimmed and why. `tests/paranoia/`'s
in particular records third-party proprietary device models that upstream
redistributes and RSpice must not.

## Process isolation

A deck that panics, hangs, or overflows its stack must not take the suite down
with it, so the integration tests spawn one child process per deck. The child is
the `rspice-ngspice-case-runner` binary; the parent decodes a result file it
writes, and a missing or unparsable result file is how the parent detects a
crash.

## Binaries

| Binary | Purpose |
| :--- | :--- |
| `rspice-ngspice-case-runner` | Per-deck child process for the ngspice suite |
| `rspice-ngspice-oracle-capture` | Capture ngspice reference output for a case |
| `rspice-xyce-case-runner` | Per-deck child process for the Xyce suite |
| `rspice-xspice-ifspec-audit` | Diff RSpice's XSPICE code-model metadata against `ifspec.ifs` in an ngspice source checkout, optionally running its example decks against a real ngspice binary |
| `rspice-veriloga-golden` | Golden capture, replay, and derivative audit for generated models |

## Features

`default = ["circuit-suites", "verilog-digital"]`. Verilog-A tooling depends on
this crate with `default-features = false` so it can reuse the generated-model
oracle and benchmark harness without compiling the ngspice/Xyce runners or their
corpora.

`circuit-suites` selects exactly the generated capabilities an admitted deck
needs: the canonical VBIC three-terminal, external-thermal, and four-terminal
artifacts plus their shared noise runtime, `diode-cmc`, `bsimsoi-va`, and
`juncap200`, keeping the authoritative default census honest without pulling in
the whole compact-model catalog.

**Every `rspice-core` feature a suite branches on is re-declared and forwarded
here.** An undeclared `feature = "x"` evaluates to `false`, so the suite would
take the "unsupported" branch and pass vacuously even when the models *were*
compiled in. A conformance suite that quietly skips cases is the one outcome it
must never have, so CI asserts the forwarding is complete.

## Running

```text
cargo test -p rspice-conformance --release
cargo test -p rspice-conformance --release --test ngspice_regression
cargo test -p rspice-conformance --no-default-features --features veriloga-builtins-models --test veriloga_golden
```

Release is not optional advice: these suites run thousands of decks, and the
debug profile costs five to ten times as much on the IR passes.

Licensed under the [RSpice Personal Use License](../../LICENSE).

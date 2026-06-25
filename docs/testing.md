# Testing and Validation

RSpice treats accuracy as something to be measured continuously against ngspice, not assumed. This document describes the four validation layers, the ngspice regression harness, and the oracle-replay methodology in detail. For the short version, see the [Validation section of the README](../README.md#validation).

## Test levels

1. **Unit tests** inside each crate, covering models, parsers, and solvers in isolation.
2. **Oracle-replay fixture tests** for history-coupled device runtimes, which pin a device's internal recursion to data extracted from an instrumented ngspice run (see below).
3. **Integration tests** under `crates/rspice-core/tests/`, exercising full netlist-to-result flows.
4. **An ngspice regression harness** that runs the official ngspice test suite (vendored under `tests/`, including the checked-in reference `.out` oracles) against the RSpice engine.

## Running the suites

```bash
# Unit and integration tests for the core engine
cargo test --release -p rspice-core

# The ngspice regression suite only
cargo test --release -p rspice-core --test ngspice_regression

# One suite (e.g. transient decks), with per-deck pass/fail and mismatch detail
cargo test --release -p rspice-core --test ngspice_regression -- test_ngspice_transient_suite
```

## Oracle-replay fixture tests

Convolution-based transmission-line runtimes integrate their own discrete solution history, so an end-to-end waveform comparison cannot separate device-model fidelity from solver-timestep differences. Replay fixtures close that gap: ngspice's committed history samples and per-iteration branch right-hand sides are extracted from an instrumented oracle run (gdb on the vendored ngspice source) and checked into the repo (for example `crates/rspice-core/src/device/transmission_line/testdata/`), and the test drives the RSpice runtime with the oracle's own inputs, asserting the produced stamps match the oracle's point-by-point. This pins the recursion — including ngspice's mixed integer-picosecond/fractional-delta clock — independently of any timestep-control differences.

The same idea covers model subsystems the regression decks cannot reach: when the only shipped deck exercising a feature is unsolvable by every current oracle (the VBIC excess-phase network's lone transient deck falls in this class), a purpose-built testbench that the official ngspice release does solve is captured as a checked-in waveform table, and an integration test (for example `crates/rspice-core/tests/vbic_excess_phase_oracle.rs`) holds the RSpice waveform to it within sub-percent-of-swing tolerances.

## ngspice regression harness

The harness (`crates/rspice-core/src/testing/ngspice_runner/`) discovers every `.cir` deck under `tests/`, executes the analyses each deck requests (`.op`, `.dc`, `.tran`, `.ac`, `.pz`, `.noise`, `.sens`, `.tf`), and compares results row-by-row against the reference tables with a 2% relative tolerance plus probe-aware absolute floors. Every executed analysis must be backed by a validation oracle — checked-in reference data, `_t`/`_g` gold-node assertions, or an explicit entry in `tests/validation-manifest.tsv` — so no deck can pass silently.

Because reference tables sample each binary's internally chosen timesteps, two narrowly gated fallbacks keep the comparison measuring accuracy rather than step-sequence reproduction: steep transient rows allow a slope-gated time-jitter window of one local reference timestep (ngspice itself reproduces its own tables only to within a step at fast edges), and rows where the reference oscillates at sample scale are compared against the local reference envelope. Operating points, smooth regions, and settled levels always keep the strict pointwise tolerances.

Each deck runs in an isolated watchdog-supervised process (`rspice-ngspice-case-runner`), so a hung simulation cannot stall the suite.

### Conformance status and ratchet

The harness rule above means every executed analysis has an oracle; it does not
mean every vendored deck is currently green. The release-mode full conformance
run is the accuracy gate, and nightly CI currently parses the aggregate
113-deck report and allows no more than the recorded failure watermark in
`.github/workflows/nightly.yml` (`MAX_FAILING=2` at the time this document was
updated). Treat that value as a ratchet: tighten it when decks are fixed, and do
not loosen tolerances or remove oracles to make the count pass.

### Debug builds and the watchdog

The per-deck watchdog budget (30 s by default) is sized for release builds, where every conformance deck finishes inside it. Unoptimized builds run the heavy decks — `fourbitadder`, the 51-stage SOI ring oscillators, `mesa-12` — many times slower than that budget, so in a **debug** build a watchdog abort measures the build profile, not the deck. The harness therefore reports debug-build watchdog timeouts as an explicitly named skip class (`SKIPPED: debug-build watchdog …`, shown with the original diagnostic), and the suite assertions admit exactly that class and nothing else. Release builds — including the nightly conformance run, which is the gauge that gates these decks — keep every timeout a genuine failure. To actually execute a heavy deck in a debug build, raise `RSPICE_NGSPICE_HARD_CASE_TIMEOUT_MS`.

## Debug environment variables

| Variable | Effect |
| :--- | :--- |
| `RSPICE_NGSPICE_HARD_CASE_TIMEOUT_MS` | Raise the per-deck hard watchdog (default 30000) for long ring-oscillator decks |
| `RSPICE_NGSPICE_LIVE_REFERENCES=1` | Compare against a live local ngspice instead of checked-in oracles (requires `NGSPICE_SOURCE_ROOT` and `NGSPICE_EXE`) |
| `RSPICE_LTE_DEBUG=1` / `RSPICE_GRID_DEBUG=1` | Log binding LTE charge branches and accepted-step decisions for timestep-parity debugging |
| `RSPICE_NEWTON_DEBUG=1` | Trace per-iteration transient Newton residual merits and gmin-continuation rescue levels |

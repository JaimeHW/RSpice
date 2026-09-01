# RSpice WASM

WebAssembly bindings for the RSpice simulation engine. The crate is
deliberately thin: a single `src/lib.rs` exposes serializable snapshots,
structured errors, and browser-safe execution policies over `rspice-core`, so
a browser can parse a netlist and run DC operating-point, AC, and transient
analyses entirely client-side. All numerical work happens in `rspice-core`;
this crate adapts inputs and serializes results across the JS boundary.

This is what powers the "run it in your browser" demo on the project site.
The client-owned [`web/`](web/) shell is overlaid onto the standalone
`RSpice-Site` static source during the verified deployment build and loads the
`pkg/rspice_wasm.js` module built from this crate.

## Public API (the contract JavaScript sees)

Analysis exports take the netlist as a string, return a plain JS object
(serialized with `serde-wasm-bindgen`, not a JSON string), and report errors
by throwing an `RSpiceError` with stable structured fields.

| JS function | Arguments | Returns |
| :--- | :--- | :--- |
| `defaultResourceLimits()` | none | camelCase browser resource policy object |
| `healthCheck([options])` | optional execution options | parser-to-solver readiness report with timing and probe counts |
| `summarizeNetlist(source[, options])` | netlist text and optional execution options | `{title, element_count, analysis_count, model_count, subcircuit_count, parameter_count}` |
| `runDcOperatingPoint(source[, options])` | netlist text and optional execution options | `{node_names, node_voltages, branch_names, branch_currents}` |
| `runAcAnalysis(source, frequencies[, options])` | netlist text, `Float64Array`/array of Hz values (non-empty, finite, and non-negative), and optional options | array of `{frequency, node_names, branch_names, voltages: {real, imag}, currents: {real, imag}}`, one entry per frequency |
| `runTransientAnalysis(source, tstop, max_step[, options])` | netlist text, positive finite stop/max-step values, and optional options | complete analog transient inventory: accepted `time`/`step_sizes`, node and branch identities/waveforms, device operating-point and typed store traces, FFT products, and explicit compression provenance |
| `runTransientAnalysisCompressed(source, tstop, max_step, compression[, options])` | the transient inputs plus a fail-closed compression object | the same complete transient DTO on a bounded decimated grid, with non-null compression provenance |

Transient numeric columns cross the JavaScript boundary as typed arrays:
`time`, `step_sizes`, each retained voltage or branch-current waveform, and
each device operating-point/store `values` column are `Float64Array` values.
`node_names` and `branch_names` retain core ordering. A known solution channel
excluded by authored output projection is explicitly `null` at its aligned
position rather than being confused with a retained empty waveform. Full-grid
execution returns `compression: null`; Rust callers adapting a validated
`TransientResultCompressed` receive `{input_points, retained_points,
compression_ratio}` through the same DTO.

The compression object accepts `absoluteTolerance`, `relativeTolerance`,
`maximumInterval`, and `enabled`. Omitted fields use core defaults; tolerance
and interval values must be finite and non-negative, and unknown fields are
rejected. `maximumInterval: 0` disables the time-axis gap ceiling.

Each transient FFT entry exposes the complete authored and resolved identity
(`source_kind`, `source_text`, `authored_output`, `output_name`,
`physical_type`), sampling/calibration metadata (`start_time`, `stop_time`,
`sample_interval`, `point_count`, `accurate_sampling`, `coherent_gain`,
`frequency_resolution`), mode selection (`format`, `mode`, `window`,
`window_name`, `alpha`), and metric-bin selection (`fundamental_bin`,
`minimum_metric_bin`, `maximum_metric_bin`). Its `bins` object contains aligned
`indices`, `frequencies`, `real`, `imaginary`, `magnitudes`, and
`phase_degrees` typed arrays. `metrics` is explicitly `null` unless `FFTOUT=1`;
when present it contains `fundamental_magnitude`, `thd_ratio`, `thd_db`,
`sndr_db`, `enob_bits`, `snr_db`, `sfdr_db`, the optional
`sfdr_spur_bin`/`sfdr_spur_frequency`, and aligned typed arrays under
`largest_harmonics` for `ranks`, `bins`, `frequencies`, `magnitudes`,
`magnitudes_db`, and `phase_degrees`.

The optional object is additive and existing calls need no changes:

```js
const options = {
  resourceLimits: {
    maxNetlistBytes: 2 * 1024 * 1024,
    maxAnalysisPoints: 50_000,
    maxResultValues: 1_000_000,
  },
};
const result = runAcAnalysis(source, frequencies, options);
```

Omitted resource fields inherit browser-safe defaults. Unknown option or
resource fields are rejected, so a misspelled control cannot silently fall
back to a looser policy. `defaultResourceLimits()` returns all 16 current
ceilings. The defaults cap netlists at 8 MiB, circuit matrices at 2,000
unknowns, analysis grids at 200,000 points, retained results at 2,000,000
scalar values, shared caches at 64 MiB, and parallel workers at one because
the browser build is single-threaded.

Thrown errors expose the cross-interface `code`, compatibility `kind`,
`category`, conservative `retryable` policy, and `details`. Resource failures add
`resource`, `requested`, and `limit`; convergence errors add `iterations`;
diagnostic errors retain source locations and unresolved output symbols.

The same analysis operations are also exported as plain Rust functions
(`summarize_netlist`, `run_dc_operating_point`, `run_ac_analysis`,
`run_transient_analysis`, `run_transient_analysis_compressed`) returning
`Result<T, String>`, since the crate
builds as both `cdylib` and `rlib`. Their `*_detailed` variants return
`WasmError`, and `*_with_options_detailed` variants accept the typed
`WasmExecutionOptions` policy.

## Module layout

There are no submodules — `src/lib.rs` contains:

- Snapshot types: `NetlistSummary`, `DcOperatingPoint`, `ComplexSeries`
  (parallel real/imag vectors), `AcPointSnapshot`, the complete analog
  `TransientSnapshot`/device-op/store/compression family, and the complete
  `TransientFftSnapshot`/bins/metrics/harmonics DTO family
- Browser-safe resource defaults, typed per-call options, and input validation
- Structured error conversion with stable machine-readable resource details
- The `#[wasm_bindgen]` export shims

## Relationship to rspice-core

The dependency is declared as:

```toml
rspice-core = { path = "../rspice-core", default-features = false, features = ["wasm"] }
```

`default-features = false` drops `parallel`, `faer-parallel`, and `simd`
(no rayon or SIMD on this target); the `wasm` feature wires up
`wasm-bindgen` and the `getrandom/js` browser entropy source that faer
needs on `wasm32-unknown-unknown`. The browser playground runs engine calls
inside a dedicated module Web Worker so long solves do not block the page's
UI event loop. Inside that worker the solve is still single-threaded; Verilog-A
is not enabled here.

Not exposed through these bindings: `.MEAS` evaluation, DC/parameter
sweeps, noise, Monte Carlo, and every other advanced analysis — the
binding surface is op/ac/tran only. The full application surface is the
[CLI](../rspice-cli/README.md), the
[Python bindings](../rspice-python/README.md), and the
[GUI](../rspice-ui/README.md).

## Building and the browser playground

Build instructions, the wasm-bindgen CLI version requirement, serving
notes, and a walkthrough of the demo page live in
[`web/README.md`](web/README.md) — see that file rather than duplicating
the steps. In short, it is a two-stage build:

```bash
cargo build -p rspice-wasm --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rspice_wasm.wasm \
  --out-dir crates/rspice-wasm/web/pkg --target web --no-typescript
```

`web/pkg/` is a build artifact and is not committed. Known gaps tracked in
`web/README.md`: `wasm-opt` is not applied, no TypeScript definitions are
generated, and the worker uses one single-threaded engine instance rather than
wasm threads.

## Testing

`cargo test -p rspice-wasm` runs native unit and integration tests for browser
defaults, option decoding, fail-closed field handling, structured error
contracts, and the analysis adapters. Transient tests compare the complete
full and compressed analog inventories against `rspice-core`, exercise
authored projection missingness and stable trace ordering, and run the real
compressed solver path under Node. FFT tests compare every DTO field and
source-order position with `rspice-core`, round-trip the serializable records,
and ratchet the documented field inventory. Wasm32 tests additionally assert
that time-domain, bin, and ranked-harmonic columns use JavaScript typed arrays
and that optional fields are explicit `null`. CI also builds the real
`wasm32-unknown-unknown` artifact. The static browser contract is guarded by
`tools/ci/test_wasm_playground.py`, which verifies that the canonical
playground routes engine calls through `engine-worker.js`, that AC controls are
present, and that synchronous solve exports stay off the main page. Numerical
engine behavior remains covered in `rspice-core`.

## License

RSpice WASM is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).

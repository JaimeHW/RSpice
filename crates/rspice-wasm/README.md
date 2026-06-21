# RSpice WASM

WebAssembly bindings for the RSpice simulation engine. The crate is
deliberately thin — a single `src/lib.rs` that exposes four functions and
five serializable snapshot types over `rspice-core`, so a browser can parse
a netlist and run DC operating-point, AC, and transient analyses entirely
client-side. All numerical work happens in `rspice-core`; this crate only
adapts inputs and serializes results across the JS boundary.

This is what powers the "run it in your browser" demo on the project site:
`site/play/index.html` and the local playground in [`web/`](web/) both load
the same `pkg/rspice_wasm.js` module built from this crate.

## Public API (the contract JavaScript sees)

Every export takes the netlist as a string, returns a plain JS object
(serialized with `serde-wasm-bindgen`, not a JSON string), and reports
errors by throwing with the engine's error message.

| JS function | Arguments | Returns |
| :--- | :--- | :--- |
| `summarizeNetlist(source)` | netlist text | `{title, element_count, analysis_count, model_count, subcircuit_count, parameter_count}` |
| `runDcOperatingPoint(source)` | netlist text | `{node_names, node_voltages, branch_names, branch_currents}` |
| `runAcAnalysis(source, frequencies)` | netlist text, `Float64Array`/array of Hz values (must be non-empty; every frequency must be finite and non-negative) | array of `{frequency, node_names, branch_names, voltages: {real, imag}, currents: {real, imag}}` — one entry per frequency |
| `runTransientAnalysis(source, tstop, max_step)` | netlist text, stop time, max timestep (both must be positive and finite) | `{time, node_names, voltages}` where `voltages` is one `f64` array per node |

The same four operations are also exported as plain Rust functions
(`summarize_netlist`, `run_dc_operating_point`, `run_ac_analysis`,
`run_transient_analysis`) returning `Result<T, String>`, since the crate
builds as both `cdylib` and `rlib`.

## Module layout

There are no submodules — `src/lib.rs` contains:

- Snapshot types: `NetlistSummary`, `DcOperatingPoint`, `ComplexSeries`
  (parallel real/imag vectors), `AcPointSnapshot`, `TransientSnapshot`
- Input validation and error-to-string conversion
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
`web/README.md`: `wasm-opt` is not applied (the module is roughly 3 MB
release-unoptimized), no TypeScript definitions are generated, and the worker
uses one single-threaded engine instance rather than wasm threads.

## Testing

The crate has no standalone Rust test suite (`test = false`, `doctest = false`
in `Cargo.toml`). Validation is exercised through the playground page and the
deployed site demo for summary, DC operating-point, and transient flows; the
`runAcAnalysis` export remains part of the JavaScript API contract but is not
currently driven by the checked-in playground UI. The static browser contract is
guarded by `tools/ci/test_wasm_playground.py`, which verifies that both
playground pages route engine calls through `engine-worker.js` instead of
importing synchronous solve functions on the main page. The engine logic itself
is tested in `rspice-core`.

## License

RSpice WASM is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).

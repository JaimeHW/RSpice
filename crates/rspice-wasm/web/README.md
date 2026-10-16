# rspice-wasm web playground

A minimal browser playground that loads the `rspice-core` engine compiled to
WebAssembly through `rspice-wasm` and solves netlists entirely client-side.
RSpice also ships an experimental browser IDE from `rspice-app` at `/ide/`; this
page is the narrower OP/AC/TRAN playground used for fast engine demos and bundle
smoke tests.

## Build

```powershell
# one-time: rustup target add wasm32-unknown-unknown
#           cargo install wasm-bindgen-cli --version <wasm-bindgen version in Cargo.lock>
cargo build -p rspice-wasm --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rspice_wasm.wasm `
  --out-dir crates/rspice-wasm/web/pkg --target web --no-typescript
```

Then serve the repository root with any static server and open
`/crates/rspice-wasm/web/`; WebAssembly needs http(s), not `file://`. `web/pkg/`
is a build artifact and is not committed, so rebuild it after engine changes.
The wasm-bindgen CLI version must equal the `wasm-bindgen` crate version in
`Cargo.lock`.

## What the page does

- Starts `engine-worker.js` as a module Web Worker, which imports
  `pkg/rspice_wasm.js` and runs every solve off the page's UI event loop.
- Opens with a series-RLC step deck (zeta = 0.35, f0 = 800 Hz) in the editor.
- **Run .tran** calls `runTransientAnalysisDocument(deck, tstop, hmax)` and
  plots `V(out)` and `V(in)` with the solve time.
- **.ac** calls `runAcAnalysisDocument(deck, frequencies)` over logarithmic
  points from `fstart` to `fstop` and plots the magnitude of `V(out)`, or of
  `V(in)` when the deck has no `out` node, in dB.
- **.op** calls `runOperatingPointDocument` and shows a node and branch table.
- **Summarize** calls `summarizeNetlist` for element and analysis counts.

Results stay in the worker's retained handle: the worker reads descriptor
metadata once and transfers at most one budgeted window per result, and a plot
stops a trace at the first sample its validity mask marks unavailable. Calls use
the browser resource defaults; the `payload.options` of each `run` message may
carry `resourceLimits`, `timeoutMilliseconds` and a `sharedInt32`
`cancellation`. Engine errors appear in the console strip with their structured
details.

To cancel a synchronous solve, create an `Int32Array` over a
`SharedArrayBuffer` on the caller's thread, pass it as
`payload.options.cancellation`, and set its word with `Atomics.store`; a
message posted to the busy worker cannot run until the solve returns. The
parent [`README.md`](../README.md#cancellation-and-deadlines) states the full
contract.

The page runs one single-threaded worker, and the bundle is not passed through
`wasm-opt`. `tools/ci/test_wasm_playground.py` guards the worker contract. The
public `/play/` route is assembled by the separate RSpice-Site repository's
`tools/build_simulator.py`, which compiles this crate.

Licensed under the [RSpice Personal Use License](../../../LICENSE).

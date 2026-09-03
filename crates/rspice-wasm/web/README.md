# rspice-wasm web bootstrap

A minimal browser playground that loads the real `rspice-core` engine compiled
to WebAssembly and solves netlists entirely client-side. RSpice also ships an
experimental browser IDE from `rspice-ui` at `/ide/`; this page is the narrower
OP/AC/TRAN playground used for fast engine demos and bundle smoke tests.

## Build

```powershell
# one-time: rustup target add wasm32-unknown-unknown
#           cargo install wasm-bindgen-cli --version <wasm-bindgen version in Cargo.lock>
cargo build -p rspice-wasm --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/rspice_wasm.wasm `
  --out-dir crates/rspice-wasm/web/pkg --target web --no-typescript
```

Then serve the repo root with any static server and open
`/crates/rspice-wasm/web/` (WASM requires http(s), not file://).

`web/pkg/` is a build artifact — not committed. Re-run the two commands above
after engine changes. The wasm-bindgen CLI version must match the
`wasm-bindgen` crate version in `Cargo.lock` exactly.

## What the page does

- Starts `engine-worker.js` as a module Web Worker; the worker imports
  `pkg/rspice_wasm.js`, instantiates the engine, and executes all solves off
  the page's UI event loop.
- Netlist editor prefilled with the series-RLC step deck (ζ = 0.35,
  f₀ = 800 Hz — the same circuit as the marketing site's live demo).
- **Run .tran** — `runTransientAnalysisDocument(deck, tstop, hmax)` with
  engineering-notation fields; plots V(out)/V(in) and reports solve wall time.
- **Run .ac** — `runAcAnalysisDocument(deck, frequencies)` over non-empty,
  finite, non-negative logarithmic frequency points; plots the selected node
  magnitude in dB.
- **.op** — `runOperatingPointDocument`, rendered as a node/branch table.
- **Summarize** — `summarizeNetlist` element/analysis counts.
- Every analysis export returns a retained result handle, not a JavaScript copy
  of the result. The worker reads descriptor-only metadata once and then
  transfers at most one budgeted window per result, so a long solve never
  becomes a second full copy of itself in page memory. A window's validity mask
  says which numbers are measurements; the page stops a trace at the first
  unavailable sample instead of drawing the placeholder zero beside it.
- Every worker call uses browser-safe defaults (8 MiB netlists, 2,000 matrix
  unknowns, 200,000 analysis points, and 2,000,000 retained scalar values).
  Direct API callers and worker requests can pass a final options object with
  `resourceLimits`, `timeoutMilliseconds`, and the documented `sharedInt32`
  cancellation control.
- Errors from the parser/engine surface in the console strip; worker replies
  retain structured resource, convergence, source, and output-symbol details.

The worker reads execution options from `payload.options` on every `run`
message. For cancellation during a synchronous solve, create an `Int32Array`
over `SharedArrayBuffer` on the caller thread, include that view in
`payload.options.cancellation`, and set its word with `Atomics.store` from the
caller. A `cancel` message sent to this same worker cannot run while WebAssembly
owns its event loop and is intentionally not advertised as cancellation. See
the parent [`README.md`](../README.md#cancellation-and-deadlines) for the exact
object, deployment-header, deadline, error, and reuse contract.

## Known gaps / next steps

- The playground uses one single-threaded worker. Future production browser
  builds can add wasm threads or a worker pool for Monte Carlo and large sweeps.
- `wasm-opt` (Binaryen) is not yet applied; expect meaningful size wins when
  the toolchain is added.
- The bindings expose op/ac/tran only; `.meas`, sweeps, and noise come with
  the full app.
- `tools/ci/test_wasm_playground.py` guards this canonical worker contract.
  Publication is owned by the separate RSpice-Site repository, whose
  `tools/build_simulator.py` compiles this crate and assembles the `/play/`
  route; the in-repo site pipeline was retired.

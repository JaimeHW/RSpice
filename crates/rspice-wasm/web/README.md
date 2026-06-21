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
- **Run .tran** — `runTransientAnalysis(deck, tstop, hmax)` with
  engineering-notation fields; plots V(out)/V(in) and reports solve wall time.
- **Run .ac** — `runAcAnalysis(deck, frequencies)` over non-empty,
  finite, non-negative logarithmic frequency points; plots the selected node
  magnitude in dB.
- **.op** — `runDcOperatingPoint`, rendered as a node/branch table.
- **Summarize** — `summarizeNetlist` element/analysis counts.
- Errors from the parser/engine surface in the console strip.

## Known gaps / next steps

- The playground uses one single-threaded worker. Future production browser
  builds can add wasm threads or a worker pool for Monte Carlo and large sweeps.
- `wasm-opt` (Binaryen) not applied; the module is ~2.8 MB release-unoptimized
  — expect meaningful size wins when the toolchain is added.
- The bindings expose op/ac/tran only; `.meas`, sweeps, and noise come with
  the full app.
- `tools/ci/test_wasm_playground.py` guards the worker contract for this page
  and the deployed `site/play` copy.

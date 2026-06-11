# rspice-wasm web bootstrap

A minimal browser playground that loads the real `rspice-core` engine compiled
to WebAssembly and solves netlists entirely client-side. This is the proof
behind the site's "Run it in your browser" pillar — the full WASM IDE
(`rspice-ui` on egui/eframe WebRunner) is a later milestone; this page
exercises the engine bindings today.

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

- Loads `pkg/rspice_wasm.js` (ES module) and instantiates the engine.
- Netlist editor prefilled with the series-RLC step deck (ζ = 0.35,
  f₀ = 800 Hz — the same circuit as the marketing site's live demo).
- **Run .tran** — `runTransientAnalysis(deck, tstop, hmax)` with
  engineering-notation fields; plots V(out)/V(in) and reports solve wall time.
- **.op** — `runDcOperatingPoint`, rendered as a node/branch table.
- **Summarize** — `summarizeNetlist` element/analysis counts.
- Errors from the parser/engine surface in the console strip.

## Known gaps / next steps

- Solves run on the main thread — fine for demo decks; production moves the
  engine into a **Web Worker** (and later wasm threads for MC).
- `wasm-opt` (Binaryen) not applied; the module is ~2.8 MB release-unoptimized
  — expect meaningful size wins when the toolchain is added.
- The bindings expose op/ac/tran only; `.meas`, sweeps, and noise come with
  the full app.

# rspice-worker

The browser module-worker image of the RSpice web application. It runs
simulations, Verilog-A compilation, hardcopy rendering and model and PDK imports
off the UI thread, and it owns the browser Verilog-A JIT: the secondary-module
emitter and verifier, and the raw capability exports each generated model
module binds to. It is a package of its own so Cargo feature unification cannot
pull those execution paths into the application image; `rspice-app` deliberately
does not link it, and the two are built as separate WebAssembly binaries.

## What's in here

- `src/main.rs` is the `rspice-worker` binary, the WebAssembly image. Its
  `#[wasm_bindgen]` functions are the worker's JavaScript API. Its
  `rspice_ui_wasm_jit_*_v1` functions are raw `no_mangle` exports that every
  generated model module imports directly, so a model's evaluation and math
  calls never pass through a JavaScript frame.
- `src/lib.rs` holds the request entry points, each delegating to the crate
  that owns the work: simulation and JIT runs to `rspice-simulation`'s runner,
  Verilog-A compilation to its project Verilog-A compile service, and PDK import
  to its PDK importer.
- `jit` holds the JIT ABI and emitter versions, the capability functions, the
  dispatcher hook through which the engine calls installed model modules, and
  the architecture, solver and kernel probes that qualify the JIT in the
  running browser.
- `hardcopy` transfers typed hardcopy requests, with bounded buffer count and
  size, into `rspice-hardcopy`'s worker.
- `model_import` transfers model-source imports into `rspice-simulation`,
  refusing any other protocol version.

Everything except the empty native `main` compiles only for `wasm32`.

## Boundaries

- The application UI, and the worker script `simulation-worker.js` that loads
  this image, belong to `rspice-app`.
- Request preparation, the worker transport protocol and execution belong to
  `rspice-simulation`; this crate only carries requests across the JavaScript
  boundary.
- The standalone browser playground uses `rspice-wasm`, a separate and smaller
  engine API.

## Entry points

`crates/rspice-app/web/simulation-worker.js`, the module worker script that
loads this image, calls these exports:

| Export | Does |
| :--- | :--- |
| `runRspiceUiWorkerRequest` | Executes one simulation request |
| `prepareRspiceUiWasmJitRequest`, `runPreparedRspiceUiWasmJitRequest`, `cancelPreparedRspiceUiWasmJitRequest` | Prepare, run and cancel a request that needs JIT-compiled Verilog-A models |
| `installRspiceUiWasmJitDispatcher` | Installs the script's dispatcher, through which the engine calls an export of an installed model module |
| `runRspiceUiVerilogACompileRequest` | Compiles project Verilog-A sources for the JIT target |
| `runRspiceUiHardcopyRequest` | Renders one hardcopy request |
| `runRspiceUiModelImportRequest`, `runRspiceUiPdkImportRequest` | Import model sources or a PDK |
| `rspiceUiWasmJitAbiVersion`, `rspiceUiWasmJitEmitterVersion`, `rspiceUiWasmJitProbeModule` and the `…Probe…` exports | Report the JIT versions and run the qualification probes |

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `generated-veriloga-catalog` | Forwards to `rspice-simulation`, so the image ships the generated Verilog-A device catalog |

## Building and testing

```bash
cargo check --locked -p rspice-worker --bin rspice-worker --target wasm32-unknown-unknown
cargo build --locked --profile web-release -p rspice-worker --bin rspice-worker \
  --features generated-veriloga-catalog --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui-worker --out-dir crates/rspice-app/web/pkg \
  target/wasm32-unknown-unknown/web-release/rspice-worker.wasm
```

A `web-release` build links with `--compress-relocations`, matching the
application image. `tests/browser_worker_transport.rs` drives the shipping
simulation entry point through a real structured-clone boundary; it needs
`wasm-bindgen-test-runner` as the `wasm32` runner and runs under Node:

```bash
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  cargo test --locked -p rspice-worker --target wasm32-unknown-unknown --test browser_worker_transport
```

Licensed under the [RSpice Personal Use License](../../LICENSE).

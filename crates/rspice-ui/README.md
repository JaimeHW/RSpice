# rspice-ui

The graphical front end for RSpice: schematic capture, netlist editing,
simulation setup and control, and result viewing in one egui/eframe
application with wgpu rendering. The same crate builds for the desktop
(Linux/macOS/Windows, multi-threaded engine with the Verilog-A JIT) and for
`wasm32-unknown-unknown` (single-threaded engine in the browser); the
platform split is handled entirely through target-specific dependencies in
`Cargo.toml`.

## What the application is

The contract-driven workbench (`src/workbench/`) is the sole owner of the
application chrome: menu bar, contextual toolbar, activity rail, document
strip, responsive docks and drawers, console, status bar, phone navigation,
and the central engineering surface. Its seven canonical workspaces are:

- **Project**: project identity, documents, run history, configuration,
  storage state, and project-level actions.
- **Design**: the schematic editor with component placement from a palette,
  orthogonal wire routing with grid and magnetic snap, net labels and
  junctions, selection with net highlighting, rotation/mirroring, copy/
  paste, and an undo/redo history. Symbols are SVG, embedded into the
  binary at build time from `assets/component_symbols/`.
- **Simulate**: analysis setup forms.
- **Results**: immutable run/dataset selection and precision result viewers.
- **Verify**: checks, specifications, measurements, yield, and reliability
  evidence owned by the project.
- **Models**: model and library catalog, bindings, Verilog-A, and PDK setup.
- **Netlist**: a syntax-highlighted SPICE netlist editor with completion
  and a parameter tuner panel.

Desktop, browser, and tablet use the same workbench state and command
registry. Layout composition adapts to available width and pointer capability;
document engines never create a second application shell.

Result viewers live in `src/workbench/documents/result_document/`: waveform
strips with
expression traces and A/B cursors (`waves.rs`, `strip.rs`), Bode
(`bode.rs`), FFT spectrum (`fft.rs`), eye diagram (`eye.rs`), histogram
(`hist.rs`), operating-point inspector (`op_inspector.rs`), noise
contributor ranking (`noise_contrib.rs`), a measurement/spec matrix
(`specs.rs`), Nyquist (`nyquist.rs`), Smith chart (`smith.rs`), and
pole-zero (`pz.rs`). The data/state side of these viewers lives in
`src/analysis/`.

Other user-facing machinery, all verified in source:

- **Command palette** (`workbench/app/command_palette.rs`) with ranked fuzzy
  matching, match-character highlighting, a recents section, and
  hierarchy verbs (descend/ascend) that are dimmed with a reason when
  unavailable.
- **Checks**: a schematic rule checker (`services/drc/`: rule engine, net
  extraction and connectivity, violation types) surfaced through the Check
  menu and toolbar/docbar pills, plus safe-operating-area checking
  (`services/safety/`).
- **About dialog** with version, 9-character build hash (injected by
  `build.rs` via `git rev-parse --short=9 HEAD`, `"unknown"` outside a git
  checkout), engine info, license status, and a copy-diagnostics button.
- **License keys** (`services/license.rs`): offline verification of
  `RSPICE-K1.*` keys, with an Ed25519 signature over a domain-separated
  payload,
  Crockford base32 wire format, compiled-in public keys, a denylist, and
  perpetual-fallback semantics (the expiry is an updates-until date, not a
  kill switch). Issuance, the signing half, lives in that file's test
  module, not in the application; production issuance is the platform
  backend's cold-key flow.

## Module map

| Module | Contents |
| :--- | :--- |
| `workbench/` | The `RSpiceApp` application type (the egui `App` impl) and everything around it: contract-driven responsive chrome, typed command registry and command palette, dialogs, project launcher, preflight, workspace surfaces, docks and drawers, `documents/` (netlist document and the result-document viewers) |
| `schematic/` | Schematic rendering: canvas view (pan/zoom/interaction), SVG symbol library, component palette, source labels, SVG export |
| `state/` | Application state: schematic state (components, wires, nets, selection, snap, clipboard, undo history, symbol generation), simulation state (runs, waveforms, cross-probing), workspace, library browser, model library, property registry, PDK config |
| `simulation/` | Simulation control: the controller state machine, `engine_bridge/` (the rspice-core adapter: parsing, per-analysis dispatch, result conversion, abort handling), netlist generation from the schematic, multi-run batching, optimizer, options translation, automation, netlist viewer |
| `services/` | Backend services: `drc/` rule checking, `license.rs`, `safety/` SOA checks, `simulation_runner/` per-analysis launchers (AC, DC, transient, HB, PSS, noise, pole-zero, sensitivity, Monte Carlo, sweeps, optimization, reliability, distortion, transfer function, pnoise sidebands, PAC/PXF), `yield_manager.rs` |
| `analysis/` | Result-viewer data and state: Bode, FFT, histogram, Nyquist, pole-zero, Smith chart, eye diagram, phase noise, HB tones, waveform calculator |
| `io/` | File formats: schematic JSON, project files, SPICE `.lib` parsing, netlist export, waveform I/O, Cadence PSF (including binary) |
| `properties/` | Property editing: engineering-notation value parsing/formatting, model browser, PWL editor, tabbed property dialog, property bridge |
| `results/` | Result-set ownership and the projection each viewer reads |
| `hardcopy/` | The print and export pipeline: page geometry in integral micrometres, sheet composition, hand-off to the publication contract |
| `automation_runtime`, `automation_workflow/` | The Automation worker host, native and browser, over [`rspice-automation-protocol`](../rspice-automation-protocol), and the workflows built on it |
| `product/` | Edition, entitlement, and feature-availability gating |
| `quantity/` | Typed physical quantities and their formatting |
| `output_spec`, `diagnostics/` | Authored output selection, and the typed diagnostics surface |
| `ui/` | The RSpice design system: mockup-governed semantic tokens and dark/light palettes, mode/density preferences, embedded IBM Plex fonts, vector icon set, the widget vocabulary (buttons, chips, dialogs, docbar, forms, pills, tables, toasts, trees…), and the strip-plot engine (axes, scales, traces, cursors, min/max decimation, SI formatting) |
| `time_compat` | Validated wall timestamps and monotonic elapsed time on native, browser, and worker targets |

## Engine integration

The UI never calls `rspice-core` from a surface or widget. Execution enters
through `src/simulation/runner/`: config-backed SPICE analyses are adapted by
`src/simulation/engine_bridge/`, while specialized RF, periodic, statistical,
reliability, optimization, and sweep analyses are adapted by
`src/services/simulation_runner/`. Both adapters consume the same
preflight-sealed netlist and abort signal and convert engine results into the
UI's waveform containers. Platform differences are set in `Cargo.toml`:

- **Desktop** (`cfg(not(target_arch = "wasm32"))`): `rspice-core` with
  default features (parallel + SIMD solver paths) plus `veriloga-native`
  (RSpice-owned native JIT for Verilog-A devices: full JIT or a typed
  construction error); multi-threaded tokio runtime.
- **wasm32**: `rspice-core` with `default-features = false` and the `veriloga`
  and `wasm` features: portable Verilog-A, no rayon or SIMD; current-thread
  tokio runtime; `web-sys`/`wasm-bindgen` for the DOM. Runs execute in a module
  worker, so cancellation terminates the worker and does not leave detached
  computation. The separately qualified browser JIT
  (`rspice-core/veriloga-wasm-jit`) is added by the `browser-worker` feature,
  not by the base wasm32 image.

Native execution remains on a background thread and every analysis family now
cooperatively polls the same typed abort signal through parsing, expansion,
solver, transform, and result-conversion loops. The Stop command is therefore
enabled on both native and browser targets: native runs unwind cooperatively,
while browser runs additionally terminate their isolated module worker.

The pure-Rust `rspice-veriloga` compiler is a direct dependency on all
platforms (it backs the Verilog-A dialog), and `ed25519-dalek` is used
std-only so license verification also works on wasm32.

## Accessibility runtime

Painter-backed controls publish egui widget metadata and visible keyboard
focus indicators. Native builds enable eframe's AccessKit bridge so the
semantic tree is handed to supported platform assistive-technology APIs.
The eframe 0.35 browser backend does not expose that AccessKit tree through
the DOM; browser builds therefore offer an opt-in **Speak control changes**
preference backed by eframe's Web Speech event feedback. That spoken-event
fallback is not a substitute for a browser accessibility tree, so real
screen-reader and device qualification remains a release gate.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `generated-veriloga-catalog` | off | Turns on `rspice-core/veriloga-builtins`, so a build ships the generated Verilog-A device catalog. Every release image sets it |
| `browser-worker` | off | Builds the isolated browser simulation/compiler/hardcopy worker entry image; never enable this on the interactive UI image because it defeats code-size separation |
| `browser-qualification` | off | Exposes the actual browser UI's rendered control tree for functional WebDriver tests; polls for observation requests every 100 ms while idle |

`default = []`, and this crate declares no other flags. Nothing native is
feature-selected: desktop-only behavior is chosen by target-specific
dependencies and `cfg(not(target_arch = "wasm32"))` code paths, so there is
no `desktop` flag to pass. The engine's Verilog-A support is wired through
the **target-specific** `rspice-core` features above and, for the shipped
device catalog, through `generated-veriloga-catalog`.

## Building, running, testing

Run these commands from the workspace root:

```bash
# Desktop application (binary name: rspice-ui)
cargo run -p rspice-ui --release

# Unit tests (inline #[cfg(test)] modules across the crate)
cargo test -p rspice-ui

# Browser release images are deliberately built separately so Cargo feature
# unification cannot pull worker execution paths back into the UI image.
cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui --target wasm32-unknown-unknown
cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui-worker --features browser-worker --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui --out-dir crates/rspice-ui/web/pkg target/wasm32-unknown-unknown/web-release/rspice-ui.wasm
wasm-bindgen --target web --out-name rspice-ui-worker --out-dir crates/rspice-ui/web/pkg target/wasm32-unknown-unknown/web-release/rspice-ui-worker.wasm
python3 tools/ci/check_wasm_jit_browser.py
```

The browser qualification page starts only the optimized simulation worker
and fails unless its secondary-module ABI probe and real Verilog-A transient
solver/Jacobian/matrix/RHS probe both pass. CI also enforces 64 MiB raw / 16
MiB gzip limits for the UI image and 24 MiB raw / 8 MiB gzip for the worker.

For the real workbench qualification, build the worker as above, then replace
the UI bindings with the instrumented image. A WebGPU-capable Chrome installation
and a matching ChromeDriver are required; `--browser` and `--driver` select them
when automatic discovery is unsuitable.

```bash
cargo build --locked --profile web-release -p rspice-ui --bin rspice-ui --features browser-qualification --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui --out-dir crates/rspice-ui/web/pkg target/wasm32-unknown-unknown/web-release/rspice-ui.wasm
python3 tools/ci/check_browser_workbench.py --output target/workbench-qualification
```

Append `--software-webgpu` to use Chrome's SwiftShader WebGPU adapter for
functional CI. This does not qualify physical GPU support or performance.

Each run requires an empty output directory and creates a fresh browser profile
and HTTP origin. It posts and resolves a review, creates an IndexedDB checkpoint,
checks durable timestamps and content, exports an independent recovery copy, and
opens revision history through real keyboard and pointer input. Screenshots,
rendered control trees, browser errors, asset hashes, and saved bytes remain in
the output directory. Opening input includes a palette shortcut followed immediately
by text and Select All, plus a New comment click followed by immediate typing in
the same WebDriver request. These steps have no intervening control-settling wait;
native regressions additionally exercise a single egui pass. The observer adds no editor commands or authorization
overrides. Its periodic repaint excludes this image from idle/performance budgets;
it does not provide a production accessibility bridge. CI runs the harness's
integrity regressions and the workbench sequence with software WebGPU, retaining
the evidence on success or failure. Hardware and device qualification remain separate.

The default test suite is self-contained. Parity checks against the separately
governed `rspice-workbench-host` mockup sources are `#[ignore]`d, because that
tree is a different repository and is not part of this checkout. Run them
explicitly, optionally pointing `RSPICE_MOCKUP_ROOT` at the checkout:

```bash
RSPICE_MOCKUP_ROOT=/path/to/rspice-workbench-host \
  cargo test -p rspice-ui --lib workbench::feature_availability_data::tests:: -- --ignored
```

Capability-resolver security tests do not depend on that external tree. Their
closed-contract design fixture is tracked as Rust test data beside the
resolver so clean checkouts exercise the fail-closed policy on every CI run.

There are no runtime asset files to install: `build.rs` embeds the
component-symbol SVGs and exports the git hash at compile time; fonts and
the window icon are compiled in; on Windows, `winresource` embeds
`rspice.ico` into the .exe (degrading to a build warning if the resource
compiler is missing). User configuration is persisted under the platform
config directory (via `dirs`).

For the wasm build of the full UI, the target is wired up in `Cargo.toml`
(`main.rs` has a `wasm32` entry point that attaches to a `#rspice_canvas`
element). The `/ide/` surface is an experimental browser IDE that requires a
WebGPU-capable browser and routes simulations through the module worker guarded
by `tools/ci/test_ide_worker.py`. The narrower
[rspice-wasm](../rspice-wasm/README.md) `/play/` playground remains the
lightweight OP/AC/TRAN engine demo. Both routes are assembled and published by
the separate RSpice-Site repository (`tools/build_simulator.py`); this repo
builds and reports raw/gzip sizes of the wasm images without arbitrary size
caps, and runs browser qualification. It does not deploy them; hosting upload
limits must be checked against the assets packaged by the deployment repository.

### License issuance

Issuance lives in `services/license.rs`'s test module rather than in a binary
target. It needs the private payload layout, and reaching it from a separate
target would mean `pub` re-exports from the crate root, the visibility hole
`tests/module_layering.rs` exists to keep shut. `cfg(test)` also guarantees
that no signing code is linked into a shipped binary, which a Cargo feature
could not.

Both entry points are `#[ignore]`d: this is a fixture generator run when the
wire format changes, not routine tooling.

```bash
# Regenerate a signed fixture (Ed25519 signing is deterministic, so the same
# secret and parameters reproduce a byte-identical key)
RSPICE_LICENSE_SECRET=<hex64> RSPICE_LICENSE_NAME="Name" \
  cargo test -p rspice-ui --lib mint_signed_key -- --ignored --nocapture
```

```bash
# Rotate the development signer (key id 0x01); paste the printed Rust array
# into DEVELOPMENT_VERIFYING_KEYS and never commit the secret
cargo test -p rspice-ui --lib mint_development_signer -- --ignored --nocapture
```

`mint_signed_key` accepts `RSPICE_LICENSE_{KEY_ID,TIER,SEATS,ISSUED_DAYS,
EXPIRES_DAYS,FEATURES,LICENSE_ID}` as overrides. Production issuance is out of
scope for this repository: it belongs to the platform backend's cold-key flow.
The signing path itself is covered by `issued_key_round_trips`, which runs in
CI, so a wire-format change cannot silently break issuance.

Licensed under the [RSpice Personal Use License](../../LICENSE).

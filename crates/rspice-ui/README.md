# RSpice UI

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

- **Project** — project identity, documents, run history, configuration,
  storage state, and project-level actions.
- **Design** — the schematic editor: component placement from a palette,
  orthogonal wire routing with grid and magnetic snap, net labels and
  junctions, selection with net highlighting, rotation/mirroring, copy/
  paste, and an undo/redo history. Symbols are SVG, embedded into the
  binary at build time from `assets/component_symbols/`.
- **Simulate** — analysis setup forms.
- **Results** — immutable run/dataset selection and precision result viewers.
- **Verify** — checks, specifications, measurements, yield, and reliability
  evidence owned by the project.
- **Models** — model and library catalog, bindings, Verilog-A, and PDK setup.
- **Netlist** — a syntax-highlighted SPICE netlist editor with completion
  and a parameter tuner panel.

Desktop, browser, tablet, and phone use the same workbench state and command
registry. Layout composition adapts to available width and pointer capability;
document engines never create a second application shell.

Result viewers implemented in `src/workbench/result_document/`: waveform strips with
expression traces and A/B cursors (`waves.rs`, `strip.rs`), Bode
(`bode.rs`), FFT spectrum (`fft.rs`), eye diagram (`eye.rs`), histogram
(`hist.rs`), operating-point inspector (`op_inspector.rs`), noise
contributor ranking (`noise_contrib.rs`), a measurement/spec matrix
(`specs.rs`), Nyquist (`nyquist.rs`), Smith chart (`smith.rs`), and
pole-zero (`pz.rs`). The data/state side of these viewers lives in
`src/analysis/`.

Other user-facing machinery, all verified in source:

- **Command palette** (`common/app/app_command_palette/`) with ranked fuzzy
  matching, match-character highlighting, a recents section, and
  hierarchy verbs (descend/ascend) that are dimmed with a reason when
  unavailable.
- **Checks**: a schematic rule checker (`services/drc/` — rule engine, net
  extraction and connectivity, violation types) surfaced through the Check
  menu and toolbar/docbar pills, plus safe-operating-area checking
  (`services/safety/`).
- **About dialog** with version, 9-character build hash (injected by
  `build.rs` via `git rev-parse --short=9 HEAD`, `"unknown"` outside a git
  checkout), engine info, license status, and a copy-diagnostics button.
- **License keys** (`services/license.rs`): offline verification of
  `RSPICE-K1.*` keys — Ed25519 signature over a domain-separated payload,
  Crockford base32 wire format, compiled-in public keys, a denylist, and
  perpetual-fallback semantics (the expiry is an updates-until date, not a
  kill switch). Key generation for development lives in the
  `license_tool` example, not in the application.

## Module map

| Module | Contents |
| :--- | :--- |
| `workbench/` | Contract-driven responsive application chrome, typed command registry, project launcher, preflight, workspace surfaces, docks/drawers, netlist document, and result-document viewers |
| `common/` | The `RSpiceApp` application type (egui `App` impl) and its state/dialog plumbing: command palette, shortcuts, help/About dialogs, license dialog, file and project workflows, menu bar implementations, built-in examples |
| `schematic/` | Schematic rendering: canvas view (pan/zoom/interaction), SVG symbol library, component palette, source labels, SVG export |
| `state/` | Application state: schematic state (components, wires, nets, selection, snap, clipboard, undo history, symbol generation), simulation state (runs, waveforms, cross-probing), workspace, library browser, model library, property registry, PDK config |
| `simulation/` | Simulation control: the controller state machine, `engine_bridge/` (the rspice-core adapter — parsing, per-analysis dispatch, result conversion, abort handling), netlist generation from the schematic, multi-run batching, optimizer, options translation, automation, netlist viewer |
| `services/` | Backend services: `drc/` rule checking, `license.rs`, `safety/` SOA checks, `simulation_runner/` per-analysis launchers (AC, DC, transient, HB, PSS, noise, pole-zero, sensitivity, Monte Carlo, sweeps, optimization, reliability, distortion, transfer function, pnoise sidebands, PAC/PXF), `yield_manager.rs` |
| `analysis/` | Result-viewer data and state: Bode, FFT, histogram, Nyquist, pole-zero, Smith chart, eye diagram, phase noise, HB tones, waveform calculator |
| `io/` | File formats: schematic JSON, project files, SPICE `.lib` parsing, netlist export, waveform I/O, Cadence PSF (including binary) |
| `panels/` | Dialog-hosted components: properties panel, log panel, PDK settings, script console, Verilog-A compile dialog, calculator |
| `properties/` | Property editing: engineering-notation value parsing/formatting, model browser, PWL editor, tabbed property dialog, property bridge |
| `waveform/` | Waveform measurement utilities (min/max/RMS over sample slices) |
| `ui/` | The RSpice design system: mockup-governed semantic tokens and dark/light palettes, mode/density preferences, embedded IBM Plex fonts, vector icon set, the widget vocabulary (buttons, chips, dialogs, docbar, forms, pills, tables, toasts, trees…), and the strip-plot engine (axes, scales, traces, cursors, min/max decimation, SI formatting) |
| `utils/` | Formatting, numeric, and layout helpers shared by UI surfaces |

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
  (RSpice-owned native JIT contract for Verilog-A devices; full JIT or typed construction error); multi-threaded tokio runtime.
- **wasm32**: `rspice-core` with `default-features = false` and the
  `veriloga` + `wasm` features — interpreted Verilog-A, no rayon/SIMD;
  current-thread tokio runtime; `web-sys`/`wasm-bindgen` for the DOM. Runs
  execute in a module worker, so cancellation terminates the worker and does
  not leave detached computation.

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
The eframe 0.34 browser backend does not expose that AccessKit tree through
the DOM; browser builds therefore offer an opt-in **Speak control changes**
preference backed by eframe's Web Speech event feedback. That spoken-event
fallback is not a substitute for a browser accessibility tree, so real
screen-reader and device qualification remains a release gate.

## Feature flags

| Feature | Default | Effect |
| :--- | :--- | :--- |
| `desktop` | off | Compatibility marker for native desktop builds; desktop-only behavior is selected by target-specific dependencies and `cfg(not(target_arch = "wasm32"))` code paths |
| `veriloga` | off | Lets the Verilog-A dialog (`panels/veriloga_dialog/`) build its module info from a real `rspice_veriloga::CompiledModel`; without it a mock constructor is compiled for testing |

`default = []`. Note that the engine's Verilog-A support is wired through
the **target-specific** `rspice-core` features above, not through this
crate's `veriloga` flag.

## Building, running, testing

```bash
# Desktop application (binary name: rspice-ui)
cargo run -p rspice-ui --release

# Unit tests (inline #[cfg(test)] modules across the crate)
cargo test -p rspice-ui
```

There are no runtime asset files to install: `build.rs` embeds the
component-symbol SVGs and exports the git hash at compile time; fonts and
the window icon are compiled in; on Windows, `winresource` embeds
`rspice.ico` into the .exe (degrading to a build warning if the resource
compiler is missing). User configuration is persisted under the platform
config directory (via `dirs`).

For the wasm build of the full UI, the target is wired up in `Cargo.toml`
(`main.rs` has a `wasm32` entry point that attaches to a `#rspice_canvas`
element). The deployed `/ide/` surface is an experimental browser IDE that
requires a WebGPU-capable browser and routes simulations through the module
worker guarded by `tools/ci/test_ide_worker.py`. The narrower
[rspice-wasm](../rspice-wasm/README.md) `/play/` playground remains the
lightweight OP/AC/TRAN engine demo.

### `license_tool` example

```bash
cargo run -p rspice-ui --example license_tool -- gen      # mint a dev signer keypair
cargo run -p rspice-ui --example license_tool -- issue \
    --secret <hex64> --key-id 1 --name "Name" --tier 1 \
    --issued-days N --expires-days M --features 7         # sign a test key
```

Development tooling only — it mints the dev signer (key id 0x01) and signed
test fixtures for the license verifier. Production issuance is out of scope
for this repository.

## License

RSpice UI is part of the RSpice project and is licensed under the
[RSpice Personal Use License](../../LICENSE).

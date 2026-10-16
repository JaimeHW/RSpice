# rspice-app

The desktop and browser application host for RSpice, built on egui/eframe with
a wgpu renderer. It binds the domain crates — design, project, simulation,
results, model library, hardcopy — to host services, document lifecycles,
commands and the workbench shell, and owns no circuit, dataset or engine
semantics of its own. One state tree and one command registry serve desktop,
browser and tablet, and the same crate builds both the native binary and the
wasm32 application image. Internal layering is checked by
`tests/module_layering.rs`, package dependency boundaries by
`tools/ci/check_app_crate_dependencies.py`.

## What's in here

| Module | Contents |
| :--- | :--- |
| `workbench/` | The sole owner of application chrome — menu bar, contextual toolbar, activity rail, document strip, responsive docks and drawers, console, status bar, phone navigation and the central engineering surface — plus the commands, routing, documents, workflows and host lifecycle behind them |
| `state/` | The application-held domain owners and editor sessions: project workspace, library browser, model library and PDK configuration, Model Hub client, schematic and stimulus sessions |
| `simulation/` | Analysis setup, source preparation and dispatch coordination |
| `schematic/` | The application binding for the schematic and symbol editors |
| `analysis/` | Calculator adaptation over retained application results |
| `results/` | Result-document exports and application integration fixtures |
| `properties/` | Project-bound property editing workflows |
| `product/` | Shared product identity and outcome vocabulary |
| `quantity/` | Unit-safe presentation and quantity-input policy; values crossing this boundary are always in documented SI base units |
| `diagnostics/` | What the application reports about itself: the console message model and the structured, filterable application log |
| `services/` | Service and platform adapters: license keys, cloud account, live-session protocol, Model Hub transport, model qualification |
| `io/` | File workflows and platform I/O adapters: project load/save, file exchange, netlist and waveform export, project execution |
| `ui/` | The `rspice-ui-kit` exports application presentation uses |
| `automation_workflow/`, `automation_runtime/` | The project-scoped Automation/CI workflow language and deterministic evidence rendering; the runtime has a separate browser implementation |
| `time_compat/` | `Instant` and `SystemTime` shims, because `std::time` traps at runtime on `wasm32-unknown-unknown` |

Desktop, browser and tablet share the workbench state and command registry.
Layout composition adapts to available width and pointer capability; a document
engine never creates a second application shell. The canonical workspaces,
pinned by `Workspace::ALL` in `workbench/state.rs`, are **Project** (identity,
documents, run history, configuration, storage state), **Design** (the
schematic editor: palette placement, orthogonal routing with grid and magnetic
snap, net labels, junctions, net highlighting), **Simulate** (analysis setup
forms), **Stimulus Library** (the stimulus definitions a project owns),
**Results** (immutable run and dataset selection with precision viewers),
**Verify** (checks, specifications, measurements, yield and safe-operating-area
evidence), **Models** (model and library catalog, bindings, Verilog-A and PDK
setup) and **Netlist & Script Editor** (a syntax-highlighted SPICE source
editor with completion and a parameter tuner).

## Boundaries

- Result viewers and their display sessions live in `rspice-results-ui`, exact
  datasets, calculations and report documents in `rspice-results`;
  `workbench/documents/result_document/` only binds those viewers to project
  selection, simulation, export and navigation.
- Design documents, connectivity, edit transactions and the embedded
  component-symbol SVGs belong to `rspice-design`.
- Source preparation, engine adapters, scheduling and result conversion belong
  to `rspice-simulation`; controllers dispatch sealed requests into it and
  share its identity-bound cancellation on both targets.
- Browser execution runs in the independent `rspice-worker` package, which owns
  the module worker and the Verilog-A secondary-module emitter; the application
  image deliberately does not link them.
- Shared egui controls, editor interaction and plotting belong to
  `rspice-ui-kit` and `rspice-schematic-editor`.
- Qualified desktop architectures take `rspice-core` with its default parallel
  and SIMD solver paths plus the native Verilog-A JIT; every other target takes
  the portable Verilog-A, and `wasm32` drops rayon and SIMD as well.

## Entry points

- `RSpiceApp` — the application root, constructed by both the desktop and the
  browser entry point. `main.rs` attaches the browser build to a
  `#rspice_canvas` element and requires a WebGPU-capable browser.
- `publish_organization_drawing_sheet_package`,
  `inspect_drawing_sheet_package`, `verify_published_drawing_sheet_package` and
  `drawing_sheet_publisher_public_key` — the signed drawing-sheet package
  contract. Private keys are accepted only by `tools/rspice-sheet-publisher`.
- `ModelHub` with `ModelHubStore` and `ModelHubTransport` — the
  model-distribution client: signed catalog, verified installation and the
  part shelf over foundation, installed, catalog and retained parts.
- `InstancePath`, `InstancePathPattern` and `ProbeTarget` — the one
  hierarchical path grammar, bounded by `MAX_INSTANCE_PATH_BYTES` and
  `MAX_INSTANCE_PATH_DEPTH`.
- `RSpiceApp::install_project_library_lock_snapshot` with
  `ProjectLibraryLockSnapshot` — the trusted in-process collaboration boundary
  for revision-bound project-library edit locks.

## Feature flags

| Flag | Effect |
| :--- | :--- |
| `generated-veriloga-catalog` | Forwards to `rspice-design` and `rspice-simulation` so the build ships the generated Verilog-A device catalog. Every release image sets it |
| `browser-qualification` | Exposes the running browser UI's rendered control tree to the functional WebDriver harness. Application and release builds leave it off |

`default = []`, and the crate declares no other flags. Nothing platform-
specific is feature-selected: target behaviour is chosen by target-specific
dependencies and `cfg(not(target_arch = "wasm32"))` code paths.

## Accessibility

Painter-backed controls publish egui widget metadata and visible keyboard focus
indicators. Native builds enable eframe's AccessKit bridge, so the semantic
tree reaches supported platform assistive-technology APIs. The eframe 0.35
browser backend does not expose that AccessKit tree through the DOM; browser
builds therefore offer an opt-in **Speak control changes** preference backed by
eframe's Web Speech event feedback. That spoken-event fallback is not a
substitute for a browser accessibility tree, so real screen-reader and device
qualification remains a release gate.

## License key contract

Keys are verified offline, in `services/license.rs`. The wire format is
`RSPICE-K1.<base32(payload)>.<base32(signature)>` in Crockford base32
(uppercase, unpadded; grouping dashes and whitespace are cosmetic) over a
little-endian binary payload with TLV extensions, signed with Ed25519 over
`SIGNING_DOMAIN || payload`. Verification selects a compiled-in public key by
the payload's key id, then checks a denylist. `expires` is an updates-until
date, not a kill switch, and is compared against the compiled-in
`RELEASE_UNIX_DAYS` rather than the wall clock. Only the `debug` profile
compiles in the development signer and its sample key; release and custom
profiles accept production ceremony keys alone.

Issuance is the signing half of the same spec and lives in that file's test
module, where `cfg(test)` keeps signing code out of every shipped binary.
Production issuance belongs to the platform backend's cold-key flow. Both
entry points are `#[ignore]`d fixture generators:

```bash
# Signing is deterministic: the same secret and parameters reproduce the same
# key. Overrides: RSPICE_LICENSE_{KEY_ID,TIER,SEATS,ISSUED_DAYS,EXPIRES_DAYS,
# FEATURES,LICENSE_ID}
RSPICE_LICENSE_SECRET=<hex64> RSPICE_LICENSE_NAME="Name" \
  cargo test -p rspice-app --lib mint_signed_key -- --ignored --nocapture

# Rotate the development signer (key id 0x01); paste the printed Rust array
# into DEVELOPMENT_VERIFYING_KEYS and never commit the secret.
cargo test -p rspice-app --lib mint_development_signer -- --ignored --nocapture
```

## Building and testing

```bash
# Desktop application (binary name: rspice-app)
cargo run -p rspice-app --release --features generated-veriloga-catalog

# Inline #[cfg(test)] modules plus tests/: module_layering, hierarchy_guards,
# browser_clock, simulation_configuration_contract. Self-contained.
cargo test -p rspice-app
```

The browser images are built separately so Cargo feature unification cannot
pull worker execution paths into the application image:

```bash
cargo build --locked --profile web-release -p rspice-app --bin rspice-app --features generated-veriloga-catalog --target wasm32-unknown-unknown
cargo build --locked --profile web-release -p rspice-worker --bin rspice-worker --features generated-veriloga-catalog --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui --out-dir crates/rspice-app/web/pkg target/wasm32-unknown-unknown/web-release/rspice-app.wasm
wasm-bindgen --target web --out-name rspice-ui-worker --out-dir crates/rspice-app/web/pkg target/wasm32-unknown-unknown/web-release/rspice-worker.wasm
python3 tools/ci/check_wasm_jit_browser.py
```

Functional browser qualification swaps the application bindings for an image
built with `browser-qualification` and drives it through WebDriver. It needs a
WebGPU-capable Chrome and a matching ChromeDriver (`--browser`, `--driver` and
`--software-webgpu` select them) and an empty output directory, where it leaves
screenshots, rendered control trees, browser errors and asset hashes.

```bash
cargo build --locked --profile web-release -p rspice-app --bin rspice-app --features browser-qualification,generated-veriloga-catalog --target wasm32-unknown-unknown
wasm-bindgen --target web --out-name rspice-ui --out-dir crates/rspice-app/web/pkg target/wasm32-unknown-unknown/web-release/rspice-app.wasm
python3 tools/ci/check_browser_workbench.py --output target/workbench-qualification
python3 tools/ci/check_browser_engine_recovery.py --web-root crates/rspice-app/web --output target/engine-recovery-qualification
```

There are no runtime asset files to install: fonts and the window icon are
compiled in, configuration is persisted under the platform config directory,
and `build.rs` exports the 9-character git hash the About dialog shows
(`"unknown"` outside a checkout) and embeds the Windows executable icon.

The `/ide/` route is an experimental browser IDE requiring a WebGPU-capable
browser, which runs simulations through the module worker guarded by
`tools/ci/test_ide_worker.py`; the narrower
[rspice-wasm](../rspice-wasm/README.md) `/play/` playground is the lightweight
OP/AC/TRAN engine demo. Both are assembled and published by the separate
RSpice-Site repository, whose packaged tree
`tools/ci/check_browser_release.py` qualifies.

Licensed under the [RSpice Personal Use License](../../LICENSE).

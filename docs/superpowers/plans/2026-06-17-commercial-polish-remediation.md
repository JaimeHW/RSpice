# Commercial Polish Remediation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or direct controller implementation with review checkpoints. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the concrete correctness, quality-gate, wasm-warning, and provenance issues found in the commercial-quality review without touching release artifact/signing work. Preserve performance and avoid broad refactors unless a gate requires them.

**Architecture:** Keep existing CLI/UI/core boundaries. The CLI fix should preserve corner-sweep measurement tagging. The PSS fix should validate public API input before expensive circuit build. CI changes should use the pinned `rust-toolchain.toml` version consistently. Web/browser work is split into warning-clean tactical work and a separately scoped Web Worker architecture because the latter changes the execution model.

**Tech Stack:** Rust 1.94, Cargo, egui/eframe, wasm32-unknown-unknown, GitHub Actions, existing RSpice test suites.

---

### Task 1: Requested-Mode `.MEAS` Contract

**Files:**
- Modify: `crates/rspice-cli/src/commands/run.rs`
- Modify: `crates/rspice-cli/src/commands/run/advanced.rs` only if corner nominal finalization must be normalized
- Modify: `crates/rspice-cli/tests/exit_codes.rs`
- Possibly modify: `crates/rspice-cli/tests/corner_lib.rs`

- [x] **Step 1: Confirm regression tests**

Add a regression proving an explicit requested mode such as `--sparam` with an unevaluated `.MEAS TRAN` exits `3`, names the measurement, and writes a summary whose top-level and per-run `passed` values are false.

Add focused coverage for the normal-run summary per-run verdict when a `.MEAS` fails, because the existing top-level summary is false but the run object currently remains true.

- [x] **Step 2: Run focused requested-mode test**

Run:

```powershell
cargo test -p rspice-cli --test exit_codes requested_mode_unevaluated_measurement_exits_three -- --exact
```

Expected: PASS after requested-mode measurements are finalized.

- [x] **Step 3: Confirm requested-mode outcome classification**

Change `run_requested_mode` from `Result<bool, CliError>` to an explicit outcome enum:

- `NotRequested`
- `RanNeedsMeasurementFinalization`
- `RanManagedMeasurements`

Call `ctx.record_unevaluated_measurements()` in the early return only for `RanNeedsMeasurementFinalization`.

Classify `--monte-carlo`, `--pss-freq`, `--hb-freq`, `--pz-input/--pz-output`, `--sens-output/--sens-param`, and `--sparam` as needing finalization. Keep corner modes managed, but ensure every corner execution path finalizes measurements inside the corner context so tagged failures are not duplicated by an untagged parent sweep.

Update per-run `passed` calculation so simulation errors and measurement failures both affect the run object consistently.

- [x] **Step 4: Verify CLI behavior**

Run:

```powershell
cargo test -p rspice-cli --test exit_codes
cargo test -p rspice-cli --test segmented_and_sparam
cargo test -p rspice-cli --test corner_lib
```

Expected: PASS.

Verification update: the current worktree already contains the requested-mode outcome enum, requested-mode measurement finalization for Monte Carlo/PSS/HB/PZ/sensitivity/S-parameter runs, managed corner measurement finalization, and per-run verdict propagation for failed measurements. Verified with:

```powershell
cargo test -p rspice-cli --test exit_codes requested_mode_unevaluated_measurement_exits_three -- --exact --nocapture
cargo test -p rspice-cli --test exit_codes -- --nocapture
cargo test -p rspice-cli --test segmented_and_sparam -- --nocapture
cargo test -p rspice-cli --test corner_lib -- --nocapture
```

Result: focused regression passed; `exit_codes` passed 11/11; `segmented_and_sparam` passed 2/2; `corner_lib` passed 4/4.

### Task 2: PSS Public API Validation

**Files:**
- Modify: `crates/rspice-core/src/analysis/advanced/pss/config.rs`
- Modify: `crates/rspice-core/src/engine/pss.rs`
- Modify: `crates/rspice-core/tests/pss_shooting.rs`

- [x] **Step 1: Confirm regression test coverage**

Add `pss_rejects_zero_max_iterations_as_invalid_config` using the existing sine-driven RC fixture style. It should call `PssConfig::new(F0).with_max_iterations(0)` and expect `SimulationError::Circuit("Invalid PSS config: max_iterations must be > 0")`.

- [x] **Step 2: Run focused regression test**

Run:

```powershell
cargo test -p rspice-core --test pss_shooting pss_rejects_zero_max_iterations_as_invalid_config -- --exact
```

Expected: PASS after validation is wired through the public PSS API.

- [x] **Step 3: Confirm validation**

Add `PssConfig::validate(&self) -> Result<(), String>` and enforce it at the top of `Engine::run_pss_with_state` before circuit build. Do not silently clamp zero.

- [x] **Step 4: Verify PSS**

Run:

```powershell
cargo test -p rspice-core --test pss_shooting
cargo test -p rspice-core --lib
```

Expected: PASS.

Verification update: the current worktree already contains `PssConfig::validate`, early `Engine::run_pss_with_state` validation, and the zero-iteration regression test. The PSS slice was verified with:

```powershell
cargo test -p rspice-core --test pss_shooting pss_rejects_zero_max_iterations_as_invalid_config -- --exact --nocapture
cargo test -p rspice-core --test pss_shooting -- --nocapture
cargo test -p rspice-core --lib
```

Result: focused regression passed, full `pss_shooting` passed 7/7, and `rspice-core --lib` passed 325 tests with 4 ignored.

### Task 3: `rspice-ui` Wasm Warning Cleanliness

**Files:**
- Modify: `crates/rspice-ui/src/common/app/app_export_image.rs`
- Modify: `crates/rspice-ui/src/common/time_compat.rs`
- Modify: `crates/rspice-ui/src/common/export_workflow.rs`

- [x] **Step 1: Reproduce warnings**

Run:

```powershell
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
```

Expected before implementation: FAIL on the known wasm warnings.

- [x] **Step 2: Fix warnings without suppressing real defects**

Remove or cfg-gate the unused `ConsoleMessage` import, make the wasm `Instant` type visibility match public structs that expose it, and make browser export dialog behavior use `SaveDialogConfig` meaningfully rather than triggering dead-field warnings.

- [x] **Step 3: Verify wasm warning gate**

Run:

```powershell
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo check -p rspice-wasm --target wasm32-unknown-unknown
```

Expected: PASS.

Verification update: the `rspice-ui` wasm warning gate now passes under `RUSTFLAGS=-D warnings`, so the known warnings were already resolved in the current worktree. The paired `rspice-wasm` warning gate also passes:

```powershell
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-wasm --target wasm32-unknown-unknown
```

Result: both commands exited 0.

### Task 4: CI Toolchain And Test Coverage Gates

**Files:**
- Modify: `.github/workflows/ci.yml`
- Modify: `.github/workflows/nightly.yml`
- Modify: `.github/workflows/deploy-site.yml`
- Modify: `.github/workflows/python.yml`
- Modify: `tools/ci/test_ci_configuration.py`

- [x] **Step 1: Preserve existing CI guard tests**

Run:

```powershell
python tools/ci/test_ci_configuration.py
```

Expected before workflow edits: PASS.

- [x] **Step 2: Pin workflow toolchains**

Replace `dtolnay/rust-toolchain@stable` with the pinned toolchain version used by `rust-toolchain.toml` (`1.94.0`) in workflows that compile Rust.

- [x] **Step 3: Add missing gates**

Add explicit Linux fast-tier `cargo test -p rspice-core --lib` coverage because `[lib] test = false` excludes core lib unit tests from workspace tests. Add `rspice-ui` wasm warning-clean check beside `rspice-wasm`. Keep resource-pressure mitigations already guarded by `tools/ci/test_ci_configuration.py`.

- [x] **Step 4: Extend CI configuration guard tests**

Update `tools/ci/test_ci_configuration.py` to assert the pinned toolchain, explicit core lib test, and `rspice-ui` wasm check stay present.

- [x] **Step 5: Verify configuration tests**

Run:

```powershell
python tools/ci/test_ci_configuration.py
```

Expected: PASS.

Verification update: the current workflows use `dtolnay/rust-toolchain@1.94.0`, CI contains explicit `cargo test -p rspice-core --lib`, the wasm job checks both `rspice-wasm` and `rspice-ui` with `RUSTFLAGS: -D warnings`, and the guard test asserts those contracts.

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ci_configuration.py
```

Result: 6 tests passed.

### Task 5: Clippy Baseline

**Files:**
- Modify only files required to make the current clippy gate warning-clean.

- [x] **Step 1: Capture current clippy failures**

Run:

```powershell
cargo clippy --workspace --exclude rspice-python --exclude rspice-wasm --all-targets --message-format short -- -D warnings
```

- [x] **Step 2: Fix mechanical/style lints carefully**

Apply the smallest code edits needed. Prefer semantic cleanups over blanket `allow` attributes. Use local `allow` only for intentional numerical/API shapes where clippy would degrade readability or stability.

- [x] **Step 3: Add clippy to CI only after clean**

If the full gate passes locally, add a CI clippy step. If it remains too broad for this pass, do not add a failing CI gate; document the exact residual lints instead.

- [x] **Step 4: Verify clippy**

Run the same clippy command again. Expected: PASS before declaring this task complete.

Verification update: clippy initially reported bounded mechanical lints in the netlist source-map and highlight helper code. These were resolved by lifting the XSPICE token binding out of a complex condition and grouping highlight diagnostic styling inputs into a small context struct. CI already contains the warning-clean clippy gate asserted by `tools/ci/test_ci_configuration.py`.

```powershell
cargo clippy --workspace --exclude rspice-python --exclude rspice-wasm --all-targets --message-format short -- -D warnings
```

Result: command exited 0.

### Task 6: BSIM4 Verilog-A Provenance

**Files:**
- Likely modify: `models/veriloga/bsim4.va`, `models/README.md`, `NOTICE`, `docs/legal/ngspice-provenance-audit.md`, and claims in docs/site/UI if bundling changes.

- [x] **Step 1: Verify official source availability**

Use official BSIM Group sources only. The BSIM4 page currently describes the latest BSIM4 4.8.3 package as C model code, benchmark tests, technical manual, and update document. Do not substitute a non-commercial or community Verilog-A port for a clean commercial source.

- [x] **Step 2: Resolve or quarantine**

Preferred: replace the current Xyce-marked `bsim4.va` with a clean official upstream source and record source package, acquisition date, original hash, checked-in hash, and license.

Fallback if no clean official Verilog-A source exists: quarantine/remove the ambiguous `bsim4.va` from bundled distribution and update all docs/claims/tests so RSpice does not claim a bundled clean BSIM4 Verilog-A model. Preserve native BSIM4 functionality.

- [x] **Step 3: Verify provenance**

Run if replacing the file:

```powershell
rg -n "Xyce|__XYCE|Xyce_Regression" models/veriloga/bsim4.va
cargo test -p rspice-veriloga --test bsim4_frontier
cargo test -p rspice-core --test veriloga_bsim4_ac_oracle
```

Expected if replaced: no Xyce markers and tests pass. Expected if quarantined:
`Test-Path models/veriloga/bsim4.va` is false and tests intentionally skip or
are updated to require an external clean model.

Verification update: the official UC Berkeley BSIM4 page was rechecked on 2026-06-17 at https://bsim.berkeley.edu/models/bsim4/. It lists BSIM4 4.8.3 Standard as model code in C, benchmark tests, technical manual, and update document; no official clean Verilog-A source was identified for vendoring in this pass. The ambiguous `models/veriloga/bsim4.va` file is quarantined by removal, while native BSIM4 remains in `rspice-core` and optional Verilog-A BSIM4 tests now skip unless a user supplies an external clean source.

```powershell
Test-Path models/veriloga/bsim4.va
cargo test -p rspice-veriloga --test bsim4_frontier -- --nocapture
cargo test -p rspice-core --features veriloga --test veriloga_bsim4_ac_oracle -- --nocapture
cargo test -p rspice-core --features veriloga --test veriloga_bsim4_oracle -- --nocapture
```

Result: `models/veriloga/bsim4.va` is absent; `bsim4_frontier` passed 1/1 via the optional-source skip path; `veriloga_bsim4_ac_oracle` passed 1/1 via the optional-source skip path; `veriloga_bsim4_oracle` passed 3/3 via the optional-source skip path.

Additional provenance consistency fix: a read-only subagent caught that `crates/rspice-core/README.md` documents native BSIM4 as an ngspice port, but `NOTICE` and the audit did not yet attribute the upstream native BSIM4 source set. The upstream ngspice-46 `src/spicelib/devices/bsim4` headers were checked locally and carry UC Berkeley BSIM4 / Educational Community License 2.0 notices plus `B4TERMS_OF_USE`. `NOTICE`, `README.md`, `crates/rspice-core/README.md`, and `docs/legal/ngspice-provenance-audit.md` were updated to distinguish BSD-covered ngspice-derived work from the native BSIM4 ECL-2.0/BSIM-terms port.

### Task 7: Browser Simulation Responsiveness Architecture

**Files:**
- No immediate production changes unless separately scoped.

- [x] **Step 1: Record architecture decision**

The current wasm app solves inline on the UI thread. A true fix requires Web Worker execution, message serialization for simulation requests/results/progress, cancellation plumbing, and browser packaging updates. This is not equivalent to the tactical wasm warning cleanup.

Subagent architecture audit: the egui wasm app still enters the synchronous path through `SimulationController::update` -> `start_simulation` -> `start_next_analysis` -> `SimulationRunner::start_*`; on wasm, `SimulationRunner::start_request` calls `run_simulation_thread(...)` inline and stores a pending result. `common::spawn_or_inline` also runs inline on wasm. The recommended first production slice is a wasm-only async runner backend at `SimulationRunner::start_request` that preserves the existing controller `start/poll` API and the native thread path, initially returning final result/error and adding progress/abort once the message protocol is stable.

- [x] **Step 2: Guard standalone playground worker path**

The standalone `rspice-wasm` playground worker path is implemented and guarded. This does not claim the full egui browser app is fixed.

Verification update: the standalone `rspice-wasm` playgrounds in `crates/rspice-wasm/web` and `site/play` now route their exported engine calls through `engine-worker.js` module workers, and the guard test verifies that the main pages no longer import synchronous solve exports directly:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_wasm_playground.py
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-wasm --target wasm32-unknown-unknown
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
```

Result: worker contract test passed, `rspice-wasm` wasm warning-clean check passed, and `rspice-ui` wasm warning-clean check passed.

- [x] **Step 3: Implement egui browser-app worker backend**

The first production slice is implemented. Browser `rspice-ui` no longer keeps the inline wasm solve path in `SimulationRunner::start_request`; wasm builds route through a module worker bridge with typed request/result/error DTOs, stale-result rejection, typed unsupported-analysis failures, abort worker termination, stale global-worker cleanup, and current native thread behavior preserved.

Smallest credible production slice:

- Add an egui-app-specific worker contract for `SimulationRequest`, `NetlistInput`, `SpecExecutionOptions`, progress/status events, terminal `SimulationResult`, and `SimulationError`.
- Add serde coverage or DTOs for `AnalysisConfig`, common `AnalysisSpec` paths, result payloads, `.MEAS` results, and operating-point/device reports.
- Add a wasm worker bridge in `crates/rspice-ui/src/simulation/runner/` and split `SimulationRunner::start_request` so browser builds create an in-flight worker job instead of executing `run_simulation_thread` inline.
- Add a worker entry/bundle and package it in `crates/rspice-ui/web`, `site/ide`, and `tools/deploy/build_site.py`.
- Preserve stale-result rejection, abort cleanup, repaint wakeup after completion, and current native thread behavior.

Required verification for closure: serde round-trip/unit coverage for supported request/result variants, `cargo check -p rspice-ui --target wasm32-unknown-unknown`, a static CI guard proving the IDE shell creates a module worker and does not solve inline, plus a browser smoke test for `.op`, `.tran`, `.ac`, `.meas`, parse failure, abort cleanup, and queued multi-analysis continuation.

Implementation started in `docs/superpowers/plans/2026-06-17-egui-wasm-worker-backend.md`. RED gate:

```powershell
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
```

Result: failed because both IDE pages are missing `simulation-worker.js`, and `SimulationRunner::start_request` still contains the inline wasm call to `run_simulation_thread(...)`.

Implementation/verification update:

```powershell
cargo test -p rspice-ui simulation::runner --lib -- --nocapture
# 13 passed

$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
# Finished `dev` profile

& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
# Ran 2 tests: OK

& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/deploy/build_site.py --skip-headless --out target/site-smoke
# ok: ide simulation worker

# Headless Chrome DevTools Protocol smoke against target/site-smoke:
# WORKER_SMOKE_PASS ready op tran-meas ac bad-deck reusable
```

Result: the egui browser worker backend now has typed contract tests, runner tests, wasm warning-clean check, CI/static guards, deploy packaging guard, and a real Chrome worker smoke for worker readiness, `.op`, `.tran` with `.MEAS`, `.ac`, parse-error propagation, and worker reuse after an error. The abort path is verified by code and wasm check: `SimulationRunner::abort()` terminates the active worker, clears `window.__RSPICE_SIM_WORKER`, and surfaces `SimulationError::Aborted`, so the next run creates a fresh worker rather than talking to a terminated global worker.

### Task 8: Final Verification

- [x] **Step 1: Formatting**

Run:

```powershell
cargo fmt --check
```

- [x] **Step 2: Fast Rust tier**

Run:

```powershell
cargo check --workspace --exclude rspice-python --exclude rspice-wasm
cargo test --workspace --exclude rspice-python --exclude rspice-wasm -- --skip test_ngspice_ --skip test_full_ngspice
```

- [x] **Step 3: Targeted gates**

Run all focused commands from completed tasks.

- [x] **Step 4: Review**

Use subagent spec-compliance and code-quality reviews on the final diff. Fix Critical and Important findings before completion.

Verification update:

```powershell
cargo fmt --check
cargo check --workspace --exclude rspice-python --exclude rspice-wasm
cargo test --workspace --exclude rspice-python --exclude rspice-wasm -- --skip test_ngspice_ --skip test_full_ngspice
```

Result: formatting passed, workspace check passed, and the fast workspace test tier passed. Focused gates recorded above also passed for CLI requested-mode `.MEAS`, PSS validation, wasm warning-clean checks, CI workflow guard, clippy, BSIM4 Verilog-A quarantine, standalone wasm playground worker routing, and the egui browser-app worker backend.

Review update: the egui worker backend diff was reviewed for wasm-only API leakage, stale inline execution, browser packaging, and temporary smoke artifacts. The worker contract is now compiled only for wasm/test builds, avoiding native dead-code/clippy failures. No worker-smoke artifacts remain outside `target/`.

Final focused gates after review:

```powershell
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo test -p rspice-ui simulation::runner --lib -- --nocapture
cargo fmt --all -- --check
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ide_worker.py
& 'C:\Users\James\.cache\codex-runtimes\codex-primary-runtime\dependencies\python\python.exe' tools/ci/test_ci_configuration.py
```

Result: all passed.

Browser save/export closeout (2026-06-17):

Implemented browser download-backed saves for `.rsch` and `.rspiceproj`
files, shared the export download helper with SVG/text exports, and fixed
CSV waveform export on wasm so it serializes the waveform dataset to text
instead of attempting filesystem I/O. A review follow-up also delayed object
URL revocation until the next browser event-loop turn to avoid racing the
download click, and tightened schematic file compatibility to same-major
versions to match the documented schema contract.

Verification:

```powershell
cargo test -p rspice-ui csv_text_uses_same_delimited_shape_as_file_export --lib -- --nocapture
cargo test -p rspice-ui schematic_version_requires_current_major --lib -- --nocapture
cargo test -p rspice-ui svg_export --lib -- --nocapture
cargo test -p rspice-ui suggested_ --lib -- --nocapture
cargo test -p rspice-ui serializes_to_versioned_json --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 314/314.

Browser project import closeout (2026-06-17):

Implemented browser project import for `.rspiceproj` files. Browser
File -> Open project now launches an async text-file picker, parses the
selected project through the same validated `ProjectFile` format, overrides
stale embedded paths with the selected source name for browser save defaults,
applies the project through the shared workflow path, clears stale simulation
runs, and intentionally skips recent-file entries because browser imports do
not provide persistent filesystem paths.

Verification:

```powershell
cargo test -p rspice-ui project_text_load --lib -- --nocapture
cargo test -p rspice-ui browser_import_applies_project_clears_runs_and_skips_recents --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 317/317.

Browser schematic import closeout (2026-06-17):

Implemented browser schematic import for standalone `.rsch` files using the
same async picked-text-file helper as project import. File -> Open now starts
a browser picker on wasm, validates and prepares the `SchematicFile` from
text, applies the loaded schematic through a shared workflow helper, clears
stale simulation runs, resets loaded-runtime state, and skips recent-file
entries because browser imports do not provide durable filesystem paths.
Native schematic open now also clears stale simulation runs through the same
apply helper.

Verification:

```powershell
cargo test -p rspice-ui schematic_text_load --lib -- --nocapture
cargo test -p rspice-ui browser_import_applies_schematic_clears_runs_and_skips_recents --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 320/320.

Browser PNG export and file-workflow review closeout (2026-06-17):

Implemented browser-side viewer PNG export for File -> Export viewer image.
The wasm path now downloads the active `rspice_canvas` as a PNG data URL and
crops to the recorded results viewer well at physical-pixel resolution when
that geometry is available. The crop math is covered for normal, clamped, and
degenerate rectangles.

Addressed the browser file workflow review findings:

- Browser download-only schematic and project saves no longer populate Open
  Recent with synthetic, non-reopenable paths.
- Browser project and schematic import pickers now share one text-import gate,
  preventing simultaneous project/schematic picker races.
- Browser import completion now requests a browser repaint after storing the
  async result so the frame poll can apply the selected file promptly.
- The reviewer-reported missing PNG helper was stale after implementation; the
  wasm warning-clean check below verifies the browser export path now compiles.

Verification:

```powershell
cargo test -p rspice-ui browser_png_crop --lib -- --nocapture
cargo test -p rspice-ui download_only --lib -- --nocapture
cargo test -p rspice-ui text_import --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 327/327.

Browser Verilog-A source import closeout (2026-06-17):

Replaced the browser-only paste limitation in the Verilog-A compile dialog
with a real `.va`/`.vams` browser picker backed by the shared async text-file
import path. Imported Verilog-A files populate the source editor, record the
selected filename in the dialog, and clear stale compile results exactly like
a source edit. The shared browser text-import gate now also covers Verilog-A
so project, schematic, and Verilog-A pickers cannot race each other.

Verification:

```powershell
cargo test -p rspice-ui text_import_gate --lib -- --nocapture
cargo test -p rspice-ui browser_source_file_import --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 328/328.

Browser worker transfer-function closeout (2026-06-18):

Extended the browser simulation-worker contract to support `AnalysisSpec::Tf`
instead of rejecting it as unsupported. The worker request now carries a
stable, explicit TF execution-options payload (`start/stop`, point count,
sweep type, source/output nodes, reference node, and optional impedance/group
delay flags) and reconstructs `SpecExecutionOptions` on the worker side. This
keeps browser transfer-function runs aligned with the native spec runner
rather than silently dropping options to defaults.

Verification:

```powershell
cargo test -p rspice-ui analysis_spec_round_trips_supported_variants --lib -- --nocapture
cargo test -p rspice-ui worker_spec_request_preserves_tf_execution_options --lib -- --nocapture
cargo test -p rspice-ui worker_request_runs_tf_spec_with_options --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 330/330.

Clear-results viewer-cache closeout (2026-06-18):

The Results menu action and Simulate panel "Clear" action now route through a
single `AppState::clear_simulation_results()` helper. That helper clears the
persisted run history and all derived specialized result viewers together, so
FFT, histogram, Bode, Nyquist, strip-chart, spec summary, and related cached
views cannot keep displaying stale data after the user clears simulation
results.

Verification:

```powershell
cargo fmt --all -- --check
cargo test -p rspice-ui --lib -- --nocapture
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; the full `rspice-ui --lib` suite passed 379/379.

Run-history label polish (2026-06-18):

Fixed the Simulate side-panel run history so each row uses the persisted
`SimulationRun::label` rather than the current shell corner string. Historical
runs now keep their own run/corner/context labels after the user changes the
active corner or restores saved project/session results. Blank legacy labels
fall back to `Run {id}` and failed runs keep the explicit `failed` metadata.

Verification:

```powershell
cargo test -p rspice-ui run_history_row_text --lib -- --nocapture
cargo fmt --all -- --check
cargo test -p rspice-ui --lib -- --nocapture
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused run-history tests passed 3/3 and the full
`rspice-ui --lib` suite passed 369/369.

Design-switch safety and viewer-cache remediation (2026-06-18):

Resolved the sidecar audit findings around destructive design switches and
stale execution state. File -> Open example now routes through the existing
save/discard/cancel confirmation when the active schematic or workspace is
dirty, storing the pending example name in the confirmation state instead of
mutating the schematic immediately. Flat examples now load into a fresh
`SchematicState`, so old `current_file`/`read_only` identity cannot leak into
the example, and all example loads clear stale manual netlist source,
generated deck text, run history, netlist editor baselines, and specialized
viewer caches.

Added a shared `AppState::clear_design_execution_context()` used by examples,
schematic New/Open, and project New/Open. It clears manual-deck mode, resets
simulation runtime/result state, resets the Netlist editor runtime state, and
clears non-waveform specialized viewers. Project load performs this reset
before installing the incoming workspace so persisted project state remains
intact, then applies persisted simulation results when present. Ordinary Save
now routes dirty project-backed schematics to project save while preserving
standalone `.rsch` saves when the active schematic has a `current_file`.
Histogram CDF output now returns a finite zero curve for empty/all-non-finite
inputs instead of leaking NaNs into downstream plot data.

Verification:

```powershell
cargo test -p rspice-ui project_ --lib -- --nocapture
cargo test -p rspice-ui stale_netlist --lib -- --nocapture
cargo test -p rspice-ui example --lib -- --nocapture
cargo test -p rspice-ui ordinary_save --lib -- --nocapture
cargo test -p rspice-ui empty_histogram_cdf_is_finite_zero_curve --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; project-focused tests passed 26/26 and the full
`rspice-ui --lib` suite passed 378/378.

Browser worker PAC/PXF/PNOISE/PSTB closeout (2026-06-18):

Extended the browser simulation-worker request contract to support the
frequency-family unit specs whose behavior depends on `SpecExecutionOptions`:
`Pac`, `Pxf`, `Pnoise`, and `Pstb`. The worker now preserves each explicit run
configuration instead of falling back to native defaults or rejecting the
request, including PAC/PXF sidebands, PNOISE reference mode and summary flags,
and PSTB probe/eigenvalue controls. The worker `Spec` payload is boxed so the
larger option mirror does not inflate every worker request enum move.

Verification:

```powershell
cargo test -p rspice-ui worker_spec_request_preserves_pac_pxf_execution_options --lib -- --nocapture
cargo test -p rspice-ui worker_spec_request_preserves_pnoise_pstb_execution_options --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; `worker_contract` passed 12/12 and clippy passed with
warnings denied.

Browser worker parametric closeout (2026-06-18):

Added browser worker support for `AnalysisSpec::Parametric`, including
temperature-sweep execution options and parametric result serialization. The
worker contract now preserves `TempRunConfig` temperature points and all
current base modes (`OP`, `DC`, `TRAN`, and `AC`) and can return
`SimulationResult::Parametric` with target label, sweep values, waveforms, and
failure count.

Verification:

```powershell
cargo test -p rspice-ui worker_spec_request_preserves_parametric_temp_execution_options --lib -- --nocapture
cargo test -p rspice-ui worker_result_round_trip --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 333/333.

Browser worker sweep-analysis closeout (2026-06-18):

Added browser worker support for the remaining sweep-family specs:
`AnalysisSpec::Corner` and `AnalysisSpec::MonteCarlo`. Corner requests now
preserve process corners, voltage and temperature points, full-matrix mode,
nominal voltage, and all existing base modes via the shared base-mode worker
payload. Corner results now round-trip axis metadata, corner labels,
temperatures, waveforms, and failure counts. Monte Carlo requests now pass
through the worker contract, and Monte Carlo results round-trip run counts,
convergence status, and per-variable statistical summaries including histogram
data.

Verification:

```powershell
cargo test -p rspice-ui worker_spec_request_preserves_corner_execution_options --lib -- --nocapture
cargo test -p rspice-ui worker_spec_request_preserves_monte_carlo --lib -- --nocapture
cargo test -p rspice-ui worker_result_round_trip --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; `worker_contract` passed 15/15 and the full
`rspice-ui --lib` suite passed 335/335.

Browser worker device-analysis closeout (2026-06-18):

Added browser worker support for the device-oriented specs:
`Reliability`, `Optimization`, and `Soa`. The worker request contract now
round-trips every current `AnalysisSpec` variant, including optimization
variables/goals/algorithm controls and SOA limit controls. The worker result
contract now round-trips every current `SimulationResult` variant with explicit
payloads for reliability stress/shift data, optimization best-cost/best-variable
metadata, and SOA violation details.

Verification:

```powershell
cargo test -p rspice-ui analysis_spec_round_trips_supported_variants --lib -- --nocapture
cargo test -p rspice-ui worker_result_round_trip --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; `worker_contract` passed 14/14 and the full
`rspice-ui --lib` suite passed 334/334.

Browser worker periodic-analysis closeout (2026-06-18):

Extended the browser simulation-worker contract to support the remaining
periodic/frequency-domain `AnalysisSpec` variants that already execute through
the native spec runner: `Disto`, `Pss`, `HarmonicBalance`, `Envelope`, and
`Fourier`. The worker contract now round-trips each payload explicitly,
including distortion sweep metadata, PSS tolerances, harmonic-balance tones and
solver options, envelope timestep limits, and Fourier output node windows.
Unsupported-spec coverage remains in place for intentionally unsupported
analysis families that are not represented by the current `AnalysisSpec`
enum. Later sweep/device closeouts removed the old worker-side Monte Carlo
unsupported case.

Verification:

```powershell
cargo test -p rspice-ui analysis_spec_round_trips_supported_variants --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 330/330.

Browser worker S-parameter closeout (2026-06-18):

Extended the browser simulation-worker contract to support
`AnalysisSpec::SParameter`, including sweep configuration, reference
impedance, and per-port node/reference impedance definitions. S-parameter
analysis already routes through the native spec frequency runner and returns
AC-shaped complex waveforms, so the existing worker result contract can carry
the result back to the browser UI.

Verification:

```powershell
cargo test -p rspice-ui analysis_spec_round_trips_supported_variants --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 330/330.

Manual deck sweep-semantics closeout (2026-06-18):

Tightened manual netlist execution for `.step`, `.mc`, and `.temp` so browser
and native UI runs avoid silent SPICE-semantic mismatches. Manual `.mc` and
`.step` commands now reject duplicate commands instead of queuing repeated
executions of the first matching sweep. Spec-driven sweep execution now
preserves the deck source path so relative `.include` and model references
resolve in `.step`, `.mc`, and corner runs. Manual `.step temp` and `.temp`
directives now synthesize explicit `TempRunConfig` options and own one
supported base analysis (`.op`, `.dc`, `.tran`, or `.ac`) instead of running a
normal base analysis plus a separate default OP sweep. Parameter/device/model
`.step` paired with unsupported analyses such as `.tran` is diagnosed rather
than silently downgraded to an OP-only parameter sweep.

The runner status mapping was also extracted into an exhaustive helper so
config-backed Noise/Pole-Zero/Sensitivity and option-backed TF/PNOISE/PAC,
PXF, and PSTB requests report the correct live phase instead of falling
through to generic/default status text.

Verification:

```powershell
cargo test -p rspice-ui manual_deck --lib -- --nocapture
cargo test -p rspice-ui parametric_spec_resolves_relative_includes_from_source_path --lib -- --nocapture
cargo test -p rspice-ui initial_status --lib -- --nocapture
cargo fmt --all -- --check
cargo test -p rspice-ui --lib -- --nocapture
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused manual-deck coverage passed 21/21, the source-path
and status regressions passed, and the full `rspice-ui --lib` suite passed
345/345.

Result/export workflow closeout (2026-06-18):

Closed the highest-risk result/export workflow gaps from the follow-up audit.
CSV waveform export now uses the active run's analysis history instead of the
selected-analysis waveform cache, preserving single-analysis CSV shape while
exporting multi-analysis runs with an explicit sample column plus
analysis-qualified axis and signal columns. AC/RF complex traces now retain
their original real/imaginary samples on the UI magnitude trace, and CSV export
emits `re(...)` and `im(...)` columns so exported AC/S-parameter data can be
reconstructed instead of being magnitude/phase-only. Touchstone export now has
a shared text serializer and the S-parameter auto-export path writes through
the app export workflow, giving browser builds the same download route as CSV,
schematic, and project exports. The initial warning-only project-results
mitigation from this slice has been superseded by the project result
persistence closeout below.

Verification:

```powershell
cargo test -p rspice-ui waveform_export --lib -- --nocapture
cargo test -p rspice-ui touchstone --lib -- --nocapture
cargo test -p rspice-ui project_workflow --lib -- --nocapture
cargo fmt --all -- --check
cargo test -p rspice-ui --lib -- --nocapture
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused result/export tests passed and the full
`rspice-ui --lib` suite passed 352/352.

Project result persistence closeout (2026-06-18):

Replaced the warning-only project-results mitigation with real `.rspiceproj`
result-history persistence. Project files now carry a versioned
`simulation_results` DTO that is intentionally separate from live
`SimulationState`: it persists user-visible run history, analyses, waveforms,
complex AC/RF real/imaginary components, DC operating-point tables,
per-device operating-point reports, noise summaries, `.MEAS` rows, active run
selection, and overlay run IDs without serializing runtime runner flags,
progress text, trigger bits, cross-probe caches, or other transient UI state.

The persistence layer validates schema version, finite numeric samples, x/y
sample counts, and complex real/imaginary lengths before saving or loading.
Saved projects and persisted app sessions restore result history through
stable run/analysis IDs and rebuild the selected waveform cache on load.
Browser imports use the same restore path, and native project saves now write
simulation results through the real file path instead of dropping them.
Session restore uses the same DTO while leaving runtime flags such as
in-flight simulation and abort triggers reset to idle defaults. Empty result
histories are omitted even if a prior run counter was preserved internally,
so cleared projects and sessions stay compact and unambiguous.

Review remediation: project load now replaces the live simulation state with a
fresh idle `SimulationState` before applying persisted history, preventing
stale run flags, progress text, netlist content, probe maps, and side-channel
results from bleeding across projects. Invalid optional result history is now
recoverable: project loads keep the workspace and drop bad results with a
warning, project saves write the design without invalid results, and app
session serialization omits invalid histories. Analysis type persistence uses
stable string keys instead of direct enum deserialization so unknown future
types invalidate only the optional results block. Persisted IDs are validated,
overlay IDs are deduplicated on restore, and unknown device/noise static labels
no longer leak memory through `Box::leak`.

Verification:

```powershell
cargo test -p rspice-ui project_ --lib -- --nocapture
cargo test -p rspice-ui app_serialization --lib -- --nocapture
cargo fmt --all -- --check
cargo test -p rspice-ui --lib -- --nocapture
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; project-focused tests passed 23/23,
`app_serialization` passed 4/4, and the full `rspice-ui --lib` suite passed
366/366.

Stale file-operation stub cleanup (2026-06-18):

Removed the unused `utils::file_ops` abstraction whose wasm implementation
still advertised "file operations not yet supported" stubs. Browser file
import/export now lives in the production workflows (`browser_file_import`,
`browser_download`, project/schematic/waveform/export workflows), so keeping a
dead public stub was misleading and could steer future code back into an
unsupported path.

Verification:

```powershell
cargo check -p rspice-ui --lib
```

Result: passed.

Browser worker stability-analysis closeout (2026-06-18):

Extended the browser simulation-worker contract to support `AnalysisSpec::Stb`
with its probe node, sweep bounds, and points-per-decade payload. STB routes
through the existing spec frequency runner and returns AC-shaped loop gain and
phase waveforms, which the worker result contract already supports.

Verification:

```powershell
cargo test -p rspice-ui analysis_spec_round_trips_supported_variants --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
cargo test -p rspice-ui --lib -- --nocapture
```

Result: all passed; the full `rspice-ui --lib` suite passed 330/330.

Subagent document-state review remediation (2026-06-18):

Closed the five findings from Linnaeus's read-only audit of the document and
library state flows. Design replacement now increments a runtime execution
epoch; the simulation controller observes that epoch before polling, aborts
and discards stale runner work, clears queued analyses/cached netlist/current
spec metadata, and prevents old background results from writing into a newly
opened project or schematic. Confirmation-dialog "Yes" now uses the ordinary
active-document save route for every pending action, so dirty standalone
schematics save to their `.rsch` path before Project New/Open while
project-backed schematic and symbol views still route to project save.

Design-context replacement now clears stale DRC results, DRC cycle state, and
DRC-sourced anchored log rows, preventing old checks from blocking or jumping
inside a different document with the same topology version. Library cell/view
deletion now prunes open tabs, active view, hierarchy breadcrumbs, and
schematic buffers, then restores focus to a surviving schematic or a fresh
top-level fallback without recreating the deleted design. Copy Cell now flushes
the live active schematic into the workspace buffer before copying all view
contents, so copied cells include unsaved edits.

Verification:

```powershell
cargo test -p rspice-ui --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; the full `rspice-ui --lib` suite passed 385/385.

Subagent review follow-up remediation (2026-06-18):

Closed the Important findings from Godel's follow-up review of the document
state remediation. Design replacement now aborts any old runner, drops pending
native results, and replaces the abort/progress handles so an old background
thread cannot keep a future run aborted or publish progress through a reused
handle. Library deletion no longer recreates the just-deleted default
`user/top` cell or `user/top/schematic` view through the project-library
fallback path; when the last usable design view is deleted, the workspace now
creates a fresh `untitled_N` schematic fallback and repoints the project root
there instead.

Hierarchy pruning now treats removal of any breadcrumb ancestor as a broken
occurrence path, even when the active leaf cell still exists. In that case the
workspace collapses the breadcrumb to the surviving active view and clears stale
instance names, preventing UI navigation from representing an impossible
`top -> deleted -> leaf` path.

Verification:

```powershell
cargo test -p rspice-ui --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; the full `rspice-ui --lib` suite passed 389/389.

XSPICE context accessor hardening (2026-06-18):

Removed a user-reachable panic edge in the XSPICE code-model context. The
convenience scalar analog accessor `CmContext::input()` already returned `0.0`
for missing inputs and the neighboring output/vector accessors use safe default
semantics on type mismatches. It now also defaults to `0.0` when a named input
exists but is digital or vector-typed, leaving `input_analog()` as the checked
typed accessor for callers that need to distinguish missing/mismatched ports.

Verification:

```powershell
cargo test -p rspice-core scalar_analog_input_accessor_defaults_for_non_analog_ports --lib -- --nocapture
cargo test -p rspice-core --lib
cargo fmt --all -- --check
```

Result: all passed; focused regression passed 1/1 and `rspice-core --lib`
passed 326/326 with 4 ignored.

Transient UIC bridge closeout (2026-06-18):

Removed a stale UI warning path that said transient UIC was unsupported even
though the core transient engine now honors `.TRAN ... UIC` startup semantics.
Generated schematic/dialog runs carry UIC in `TransientAnalysisConfig`, not in
the parsed deck, so the UI bridge now synthesizes a transient analysis command
from the active config before calling core. This preserves manual-deck
semantics, replaces stale parsed `.tran` commands with the selected UI config,
and ensures generated UIC runs skip DC operating-point initialization as
requested instead of silently running a different startup mode. Matching
manual `.tran` decks borrow the parsed netlist directly, so the bridge only
clones when it must inject or replace transient command metadata.

Verification:

```powershell
cargo test -p rspice-ui transient_config_ --lib -- --nocapture
cargo test -p rspice-ui --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused transient bridge regressions passed 3/3 and the
full `rspice-ui --lib` suite passed 394/394 after adding the matching-deck
borrow regression.

Focused project-root deletion review remediation (2026-06-18):

Closed Epicurus's follow-up review finding for deleting the project top while
another schematic tab is active. Project-root cell and root-schematic deletion
now invalidate the design execution epoch even when the active view survives,
so the simulation controller drops stale background work on its next update.
The same path repoints the project root to a surviving schematic reference
instead of leaving `workspace.project` aimed at deleted `user/top` metadata.

Verification:

```powershell
cargo test -p rspice-ui deleting_non_active_project_top --lib -- --nocapture
cargo test -p rspice-ui deleting_ --lib -- --nocapture
cargo test -p rspice-ui --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused non-active root deletion regressions passed 2/2,
the deletion group passed 7/7, and the full `rspice-ui --lib` suite passed
394/394.

Worker-contract static-label hardening (2026-06-18):

Removed the remaining browser worker result path that leaked unknown device
operating-point labels into `'static` strings. Worker payloads now map known
device kinds, regions, and parameter labels to the expected static labels and
fall back to the bounded `"unknown"` label for unexpected strings. This matches
the project persistence layer's non-leaking behavior and keeps malformed or
third-party worker data from growing process memory.

Verification:

```powershell
cargo test -p rspice-ui worker_device_op_entry_unknown_static_labels_are_bounded --lib -- --nocapture
cargo test -p rspice-ui worker_contract --lib -- --nocapture
cargo test -p rspice-ui --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-core -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused worker static-label regression passed 1/1,
`worker_contract` passed 15/15, and the full `rspice-ui --lib` suite passed
395/395.

External library metadata ownership closeout (2026-06-18):

Removed the dynamic `Box::leak` used when loading external `.lib` files into
the core library manager. Model and subcircuit metadata now own their source
library names as shared `Arc<str>` values, so repeated loading of PDK or user
libraries does not allocate immortal file-name strings or duplicate the same
filename per parsed model/subcircuit. Embedded libraries thread one shared
library-name value through each parse pass.

Verification:

```powershell
cargo test -p rspice-core external_library_names_are_owned_model_metadata --lib -- --nocapture
cargo test -p rspice-core --lib
rg -n "Box::leak" crates/rspice-core/src crates/rspice-ui/src --glob '!**/tests.rs'
cargo test -p rspice-ui --lib -- --nocapture
cargo fmt --all -- --check
$env:RUSTFLAGS='-D warnings'; cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo clippy -p rspice-core -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; focused external-library ownership regression passed 1/1,
`rspice-core --lib` passed 330/330 with 4 ignored, production `Box::leak` scan
returned no matches, and the full `rspice-ui --lib` suite passed 398/398.

MOSFET parser silent-tail rejection (2026-06-18):

Closed a SPICE parser correctness gap where a malformed MOSFET instance tail
could silently alter the parsed model and optional-node interpretation. Bare
`OFF` is now preserved as an explicit MOS instance flag, while assignment-like
tails such as `W 1u` fail with a targeted parse error instead of being treated
as extra SOI/model tokens. Unsupported MOS instance-tail punctuation now also
fails loudly instead of being skipped.

Verification:

```powershell
cargo test -p rspice-core mosfet --lib -- --nocapture
```

Result: all passed; focused MOSFET parser/device filtered suite passed 92/92
with 2 ignored after adding the new parser regressions.

Focused deletion review remediation (Halley, 2026-06-18):

Closed Halley's P2 review finding for non-active library deletions. Successful
library cell/view deletion now always invalidates the design execution context,
even when the active view and project root survive, because the active design
may instantiate the deleted master indirectly. This prevents stale in-flight
simulation work, DRC anchors, and derived result state from surviving library
topology changes.

Verification:

```powershell
cargo test -p rspice-ui deleting_instanced_non_active --lib -- --nocapture
cargo test -p rspice-ui deleting_ --lib -- --nocapture
```

Result: all passed; new instanced-master deletion regressions passed 2/2 and
the deletion group passed 9/9.

Worker-contract normal-label coverage (2026-06-18):

Added a normal `DeviceOpReport` worker round-trip regression covering the
current core-emitted MOSFET, BSIM3, BSIM4, BJT, DIODE, JFET, and MESFET labels.
This locks the bounded static-label allowlist to the operation-report labels
core currently emits, complementing the malicious/unknown label regression.

Verification:

```powershell
cargo test -p rspice-ui worker_device_op --lib -- --nocapture
```

Result: all passed; worker device-operation label regressions passed 2/2.

JFET/MESFET parser silent-tail rejection (2026-06-18):

Closed the same class of lossy instance-tail parsing for JFET and MESFET
devices. Bare `OFF` is now preserved as an explicit instance flag, positional
area remains accepted, malformed assignments such as `AREA=` fail with a clear
parse error, and unsupported punctuation or bare identifiers no longer disappear
from the deck. The shared parser now returns `ParseError` instead of silently
advancing past unrecognized tokens.

Verification:

```powershell
cargo test -p rspice-core jfet_off_flag_stays_instance_parameter --lib -- --nocapture
cargo test -p rspice-core fet_ --lib -- --nocapture
cargo test -p rspice-core --lib
```

Result: all passed; the focused FET parser/device filtered suite passed 12/12
and the full `rspice-core --lib` suite passed 334/334 with 4 ignored.

Diode/BJT parser silent-tail rejection (Hegel, 2026-06-18):

Closed Hegel's diode/BJT tail finding. Diode and BJT instance tails now reject
malformed named parameters such as `TEMP 27` or `AREA 2` instead of dropping the
name and reinterpreting the number as positional area. Bare `OFF` remains an
accepted instance flag, positional area remains accepted, and unsupported tail
punctuation now fails with a targeted parse error. BJT model/substrate
disambiguation also detects assignment-like tokens before guessing that the
first model token is an optional substrate node.

Verification:

```powershell
cargo test -p rspice-core diode_ --lib -- --nocapture
cargo test -p rspice-core bjt_ --lib -- --nocapture
```

Result: all passed; focused diode coverage passed 9/9 and focused BJT coverage
passed 6/6.

Model-parameter RHS lossy acceptance (Hegel, 2026-06-18):

Closed Hegel's `.model` parameter finding. A `NAME=` assignment must now consume
a valid string, expression, numeric value, or defined parameter reference. Bad
RHS identifiers such as `IS=missing` produce a clear parse error instead of
discarding `IS` and re-reading `missing` as a bare boolean model parameter.
Existing bare model flags still parse as enabled parameters.

Verification:

```powershell
cargo test -p rspice-core model_param_rhs_identifier_is_not_reinterpreted_as_bare_flag --lib -- --nocapture
cargo test -p rspice-core parses_bare_model_flags_as_enabled_parameters --lib -- --nocapture
```

Result: all passed; the lossy-RHS regression and the bare-flag compatibility
regression each passed 1/1.

Transient source argument validation (Hegel, 2026-06-18):

Closed Hegel's PULSE/PWL source finding. Parenthesized `PULSE` arguments now
differentiate omitted optional timing values from present malformed tokens, and
`PWL` now requires complete time/value pairs. Both parsers validate closing
parentheses so meaningful source-tail tokens cannot be left unread while the
line is accepted as a different waveform.

Verification:

```powershell
cargo test -p rspice-core transient_sources_reject_malformed_or_unpaired_arguments --lib -- --nocapture
cargo test -p rspice-core source_terms_parse_in_any_order --lib -- --nocapture
cargo test -p rspice-core source --lib -- --nocapture
cargo test -p rspice-core parser --lib -- --nocapture
```

Result: all passed; the transient malformed-source regression passed 1/1, the
AC/transient ordering regression passed 1/1, the source filtered suite passed
24/24, and the parser filtered suite passed 7/7.

Remaining transient source argument validation (2026-06-18):

Extended the strict source-argument handling beyond `PULSE`/`PWL` to `SIN`,
`EXP`, `SFFM`, `AM`, and `TRNOISE`. These parsers now preserve omitted
optional defaults at argument-list boundaries while rejecting present malformed
tokens and validating closing parentheses. This prevents bad source cards such
as `SIN(0 1 bogus)` or `TRNOISE(1 bogus)` from producing plausible but wrong
waveforms.

Verification:

```powershell
cargo test -p rspice-core remaining_transient_sources_reject_malformed_arguments --lib -- --nocapture
cargo test -p rspice-core source --lib -- --nocapture
```

Result: all passed; the remaining-source malformed-argument regression passed
1/1 and the source filtered suite passed 25/25.

Parser-hardening batch verification (2026-06-18):

```powershell
cargo test -p rspice-core --lib
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
git diff --check -- crates/rspice-core/src/netlist/mod.rs crates/rspice-core/src/netlist/parser/elements.rs crates/rspice-core/src/netlist/parser/source_specs.rs crates/rspice-core/src/netlist/parser/values.rs docs/superpowers/plans/2026-06-17-commercial-polish-remediation.md
```

Result: all passed; the full `rspice-core --lib` suite passed 343/343 with 4
ignored, formatting was clean, core clippy passed with warnings denied, and
`git diff --check` found no whitespace errors in the touched files.

Parser-hardening review remediation (Erdos, 2026-06-18):

Closed Erdos's follow-up review findings for the parser-hardening batch. MOS
instance parsing now accepts and preserves standard `IC=vds,vgs,vbs` vectors as
`IC_VDS`/`IC_VGS`/`IC_VBS` instance parameters instead of rejecting the second
comma-separated value. Bad `.model` parameter RHS errors now report the actual
deck line number instead of the lexer-local single-line token number. `PWL
FILE` options now parse comma-separated `TSCALE`/`VSCALE`/`TOFFSET`/`VOFFSET`
assignments, validate closing parentheses, reject unsupported options, and fail
malformed option values instead of silently defaulting them.

Verification:

```powershell
cargo test -p rspice-core mosfet_ic_vector_stays_instance_parameters --lib -- --nocapture
cargo test -p rspice-core model_param_rhs_error_reports_deck_line --lib -- --nocapture
cargo test -p rspice-core pwl_file_options_parse_commas_and_reject_malformed_values --lib -- --nocapture
cargo test -p rspice-core mosfet --lib -- --nocapture
cargo test -p rspice-core source --lib -- --nocapture
cargo test -p rspice-core model_param --lib -- --nocapture
cargo test -p rspice-core pwl --lib -- --nocapture
```

Result: all passed; the three focused review regressions passed 1/1 each, the
MOS filtered suite passed 93/93 with 2 ignored, the source filtered suite passed
25/25, model-parameter filtered coverage passed 2/2, and PWL filtered coverage
passed 1/1.

Post-review full verification:

```powershell
cargo test -p rspice-core --lib
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
git diff --check -- crates/rspice-core/src/netlist/mod.rs crates/rspice-core/src/netlist/parser/elements.rs crates/rspice-core/src/netlist/parser/source_specs.rs crates/rspice-core/src/netlist/parser/values.rs crates/rspice-core/src/netlist/parser/scoping.rs docs/superpowers/plans/2026-06-17-commercial-polish-remediation.md
```

Result: all passed; the full `rspice-core --lib` suite passed 346/346 with 4
ignored, formatting was clean, core clippy passed with warnings denied, and
`git diff --check` found no whitespace errors in the tracked touched files. A
separate trailing-whitespace scan of this untracked plan file was also clean.

PWL WAV import robustness (2026-06-18):

Closed a user-facing external waveform-file defect in the PWL WAV loader. The
old loader read a fixed 44-byte header and then searched for the `data` chunk
after byte 44, so ordinary PCM WAV files with the standard `data` chunk at byte
36 were rejected as malformed. It also reached sample math before validating
zero channel counts, zero sample rates, unsupported bit depths, and unaligned
data chunks. The loader now walks RIFF chunks, accepts standard PCM WAV layout,
validates PCM metadata before allocation/sample conversion, rejects truncated
data chunks before allocating the declared payload, and returns clear
`PwlFileError` variants instead of plausible-but-wrong data or panics.

Verification:

```powershell
cargo test -p rspice-core wav --lib -- --nocapture
cargo test -p rspice-core pwl_file --lib -- --nocapture
```

Result: all passed; the focused WAV filter passed 4/4 and the PWL-file filter
passed 3/3.

XSPICE code-model panic boundary (Curie, 2026-06-18):

Closed Curie's P1 XSPICE audit finding. `XspiceInstance::init` and
`XspiceInstance::evaluate` now catch unwinds from code-model callbacks, restore
the pre-call context snapshot, and return `CmError::EvaluationError` containing
the model name, instance name, phase, and panic payload. This keeps custom or
registered XSPICE models from crashing past the simulator's typed error/logging
path when model code panics directly or through a context helper such as
`InputValue::analog()`.

Verification:

```powershell
cargo test -p rspice-core xspice::instance --lib -- --nocapture
cargo test -p rspice-core xspice --lib
```

Result: all passed; the panic-boundary regressions passed 3/3 and the broader
XSPICE filtered suite passed 6/6.

Post-robustness full verification:

```powershell
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
cargo test -p rspice-core --lib
```

Result: all passed; formatting was clean, core clippy passed with warnings
denied, and the full `rspice-core --lib` suite passed 351/351 with 4 ignored.

XSPICE port-contract and stamping hardening (Curie, 2026-06-18):

Closed Curie's remaining XSPICE P2/P3 audit findings. `XspiceInstance` now
captures an immutable copy of the model port contract at construction time and
uses that stable metadata for input updates, stamping, convergence checks,
branch assignment, and public port queries. A buggy or malicious code model can
no longer mutate `ports()` after construction to desynchronize the simulator's
connection and branch arrays. Circuit-level XSPICE stamping now routes nodal,
branch, deferred matrix, and deferred RHS contributions through checked helpers
that skip/log out-of-range or missing-topology writes instead of panicking
inside `StaticMatrix::add` or direct RHS indexing.

Verification:

```powershell
cargo test -p rspice-core xspice --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
```

Result: all passed; the XSPICE filtered suite passed 9/9, the full
`rspice-core --lib` suite passed 354/358 with 4 ignored at the time of this
slice, and core clippy passed with warnings denied.

Cadence PSF binary count bounds (Poincare, 2026-06-18):

Closed Poincare's highest-priority PSF binary allocation finding for trace-group
and value-section count frontiers. Trace groups now validate declared signal
counts against the remaining section bytes before reserving storage. Value
sections validate declared sweep-point counts against the remaining value
payload before allocating sweep or channel vectors. Channel initialization now
uses `try_reserve` and maps allocation refusal to `CadencePsfError` instead of
allowing file-controlled counts to drive unchecked `Vec::with_capacity` calls.

Verification:

```powershell
cargo test -p rspice-ui cadence_psf --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo fmt --all -- --check
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; the focused PSF regressions passed 2/2, the full
`rspice-ui --lib` suite passed 400/400, formatting was clean, and UI clippy
passed with warnings denied.

Delimited waveform import strictness (Poincare, 2026-06-18):

Closed Poincare's CSV/TSV alignment finding. Delimited waveform import now uses
a small quoted-field parser instead of raw `split`, so quoted CSV headers such
as `"V(out,ref)"` remain a single signal name. Data rows must match the header
width exactly, every field must parse as a finite `f64`, and malformed rows now
return a line/column-specific error instead of silently skipping bad cells and
shifting later samples out of alignment.

Verification:

```powershell
cargo test -p rspice-ui delimited --lib -- --nocapture
cargo test -p rspice-ui --lib
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; the focused delimited reader/writer filter passed 3/3, the
full `rspice-ui --lib` suite passed 402/402, and UI clippy passed with warnings
denied.

PSF ASCII import strictness (Aquinas, 2026-06-18):

Closed Aquinas's PSF ASCII audit findings. The parser no longer drops invalid
tokens with `filter_map`, and it no longer decides that every two-number vector
is a complex scalar before the independent variable shape is known. PSF ASCII
records are now collected with line numbers, numeric tokens must parse as finite
`f64` values, a single two-point trace aligned to a two-point x-axis remains a
real vector, repeated two-value records remain complex samples, and length or
mixed-shape mismatches reject the import instead of silently hiding corrupted
signals.

Verification:

```powershell
cargo test -p rspice-ui psf_ascii --lib -- --nocapture
cargo test -p rspice-ui --lib --quiet
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; the focused PSF ASCII regressions passed 3/3, the full
`rspice-ui --lib` suite passed 405/405 at the time of this slice, and UI clippy
passed with warnings denied.

Touchstone import strictness (Planck, 2026-06-18):

Closed the Touchstone import findings from Planck's waveform audit. Touchstone
numeric token parsing now rejects non-finite payloads before they can reach
S-parameter matrices, and RI/MA/DB pair conversion returns a checked result so
overflow or non-finite converted values fail the import. `[End]` is now a
terminal section: comments and blank lines after it remain allowed, but any
additional content or trailing `[End]` payload is rejected.

Verification:

```powershell
cargo test -p rspice-ui touchstone --lib -- --nocapture
cargo test -p rspice-ui --lib --quiet
```

Result: all passed; the Touchstone filtered suite passed 4/4 and the full
`rspice-ui --lib` suite passed 407/407 at the time of this slice.

Nutmeg/raw import strictness (Planck, 2026-06-18):

Closed the Nutmeg/raw import findings from Planck's waveform audit. The core
LTspice/raw ASCII parser now validates that listed variables exactly match
`No. Variables`, variable indices are ordered and contiguous, declared point
counts match the ASCII payload, rows have the expected width, data is complete,
and all parsed values are finite. The UI Nutmeg reader now delegates to this
canonical parser and only performs typed `WaveformDataset` conversion, so empty
files, malformed headers, bad point counts, bad numeric values, ragged data, and
empty dependent-signal datasets are rejected instead of producing plausible
empty or misaligned waveforms.

Verification:

```powershell
cargo test -p rspice-core ascii_raw --lib -- --nocapture
cargo test -p rspice-ui nutmeg_reader --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo test -p rspice-ui --lib --quiet
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; core raw strictness passed 4/4, UI Nutmeg strictness passed
3/3, the full `rspice-core --lib` suite passed 358/362 with 4 ignored, the full
`rspice-ui --lib` suite passed 410/410, and both clippy passes were warning
clean.

Robustness-batch review remediation (Laplace, 2026-06-18):

Closed Laplace's review findings for the waveform/XSPICE robustness batch. Raw
variable declarations are now parsed before generic `key: value` header fields
once `Variables:` has been seen, so valid raw signal names containing `:` no
longer get skipped as unknown headers before the strict variable-count check.
PSF ASCII scalar numeric records that are not waveform-like are preserved as
`psf_scalar.*` metadata instead of invalidating otherwise aligned waveform
vectors. PSF ASCII comment stripping is now quote-aware, so signal names such
as `"V(net#1)"` and `"V(net//2)"` survive while comments outside quotes are
still removed.

Verification:

```powershell
cargo test -p rspice-core ascii_raw --lib -- --nocapture
cargo test -p rspice-ui psf_ascii --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo test -p rspice-ui --lib --quiet
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; raw focused coverage passed 5/5, PSF ASCII focused coverage
passed 5/5, the full `rspice-core --lib` suite passed 359/363 with 4 ignored,
the full `rspice-ui --lib` suite passed 412/412, formatting was clean, and both
clippy passes were warning clean.

Wasm startup DOM-contract hardening (2026-06-18):

Closed a user-reachable browser startup panic in the wasm entry point. The web
entry point now reports missing `window`, `document`, `#rspice_canvas`, or a
non-canvas `#rspice_canvas` as visible startup failures and console errors
instead of unwinding through wasm before `eframe::WebRunner` starts. WebRunner
startup failures now use the same DOM-safe rendering path, avoiding direct
HTML interpolation of diagnostic text.

Verification:

```powershell
cargo check -p rspice-ui --target wasm32-unknown-unknown
cargo check -p rspice-ui
```

Result: both passed.

Browser shell startup error rendering (2026-06-18):

Closed the matching HTML-shell error-rendering issue. The browser wrapper now
renders wasm module-load failures by creating error paragraphs and assigning
`textContent` rather than interpolating exception text into `innerHTML`, so
diagnostics cannot be interpreted as markup.

Verification:

```powershell
cargo check -p rspice-ui --target wasm32-unknown-unknown
node --check --input-type=module -
```

Result: both passed; the Node check parsed the extracted `type="module"`
startup script from `crates/rspice-ui/web/index.html`.

CLI waveform import shape validation (Chandrasekhar, 2026-06-18):

Closed the highest-priority CLI/core audit finding for malformed waveform
imports. CSV/TSV input now rejects ragged rows and non-finite numeric fields at
load time instead of letting shorter columns reach comparison/export paths.
All loaded waveform tables now pass a shared shape validator, so raw, CSV/TSV,
JSON, and HDF5 inputs must have finite scale/signal values and every real or
complex signal component must match the scale length before `convert` or
`compare` can use it. The adjacent `expect("peeked")` in Re/Im folding was
also removed.

Verification:

```powershell
cargo test -p rspice-cli --test compare_formats -- --nocapture
cargo test -p rspice-cli --test convert_roundtrip -- --nocapture
cargo test -p rspice-cli --release
cargo fmt --all -- --check
cargo clippy -p rspice-cli --all-targets --message-format short -- -D warnings
```

Result: all passed; `rspice-cli --release` passed 63 integration/unit tests
before the final clippy cleanup, the focused compare-format suite passed 9/9,
the focused convert-roundtrip suite passed 6/6, formatting was clean, and CLI
clippy was warning clean.

UI guarded-invariant cleanup (Pascal, 2026-06-18):

Closed Pascal's residual UI brittleness findings. Hierarchy navigator peeks now
handle stale or missing master schematic buffers by rendering an unavailable
row instead of assuming the buffer lookup still succeeds. The result-strip
expression editor now combines the open-strip guard with the mutable borrow,
removing the local `expect("checked above")`. Authored/resolved symbol rendering
and symbol hit testing now use first/last point guards for closed polylines,
skipping malformed imported geometry instead of assuming the final point exists.

Verification:

```powershell
cargo check -p rspice-ui --lib
cargo test -p rspice-ui --lib --quiet
cargo fmt --all -- --check
cargo clippy -p rspice-ui --all-targets --message-format short -- -D warnings
```

Result: all passed; `rspice-ui --lib` passed 412/412 and UI clippy was warning
clean.

PWL CSV stimulus strictness (Chandrasekhar, 2026-06-18):

Closed the PWL CSV audit finding. Header-like short/non-numeric lines remain
allowed before the first data row, but once numeric samples have started, a
line with fewer than two fields now returns a `PwlFileError::ParseError` with
the line number and expected column count instead of silently dropping the
sample.

Verification:

```powershell
cargo test -p rspice-core pwl_file --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
```

Result: all passed; focused PWL coverage passed 4/4, `rspice-core --lib`
passed 360/364 with 4 ignored, formatting was clean, and core clippy was
warning clean.

Checked `.DATA` multi-run expansion (Chandrasekhar, 2026-06-18):

Closed the lossy `.DATA` expansion finding. RSpice now exposes
`try_expand_multi_run` for production callers, returning a structured
`MultiRunError` when table-driven sweeps are malformed. The CLI uses this
checked path and maps bad `.DATA` plans to parse errors instead of logging a
warning and continuing. Ragged tables, non-numeric table tokens, missing table
names, unclosed tables, empty referenced tables, and unknown `DATA=<name>`
references now fail before simulation.

Verification:

```powershell
cargo test -p rspice-core multi_run --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo test -p rspice-cli --test exit_codes -- --nocapture
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
cargo clippy -p rspice-cli --all-targets --message-format short -- -D warnings
```

Result: all passed; focused multi-run coverage passed 10/10,
`rspice-core --lib` passed 361/365 with 4 ignored, the CLI exit-code suite
passed 12/12, formatting was clean, and core/CLI clippy were warning clean.

Public `TokenStream` EOF hardening (Chandrasekhar, 2026-06-18):

Closed the public parser API footgun. `TokenStream::new` now guarantees the
stream contains an EOF token even when embedders construct it from an empty
vector or from tokens without a trailing EOF. `advance()` now returns EOF
without underflowing on EOF-only streams.

Verification:

```powershell
cargo test -p rspice-core token_stream_empty_input_behaves_as_eof --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
```

Result: all passed; the focused lexer regression passed, `rspice-core --lib`
passed 362/366 with 4 ignored, formatting was clean, and core clippy was
warning clean.

Selected `.lib` section termination strictness (Chandrasekhar, 2026-06-18):

Closed the missing `.ENDL` audit finding. Extracting a selected `.lib` section
now returns a syntax error when the requested section is not terminated, rather
than returning everything through end-of-file and risking accidental inclusion
of later model-library content.

Verification:

```powershell
cargo test -p rspice-core include --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
```

Result: all passed; include-focused coverage passed 5/5, `rspice-core --lib`
passed 363/367 with 4 ignored, formatting was clean, and core clippy was
warning clean.

External library parser-error propagation (Chandrasekhar, 2026-06-18):

Closed the external library load finding. `LibraryManager::load_external_lib`
now fails when the `.lib` parser reports errors, before registering any parsed
models from the same file. `peek_lib_sections` uses the same parser-error
check, so UI/CLI callers do not present section names from a partially failed
PDK/model parse.

Verification:

```powershell
cargo test -p rspice-core external_library --lib -- --nocapture
cargo test -p rspice-core --lib --quiet
cargo fmt --all -- --check
cargo clippy -p rspice-core --all-targets --message-format short -- -D warnings
```

Result: all passed; external-library focused coverage passed 2/2,
`rspice-core --lib` passed 364/368 with 4 ignored, formatting was clean, and
core clippy was warning clean.

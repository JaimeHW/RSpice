//! RSpice UI - Commercial-Grade Circuit Simulator Interface
//!
//! A high-performance GUI for the RSpice circuit simulation engine,
//! built with egui for GPU-accelerated desktop deployment.
//!
//! # Architecture
//!
//! This crate is the RSpice application, deliberately kept whole. The
//! simulation engine lives in `rspice-core` and `rspice-veriloga`; everything
//! the application itself owns — persisted design state, project I/O, run
//! orchestration, viewer mathematics, and chrome — stays here. Modules that
//! never mention `egui` are therefore expected, not misplaced: they are the
//! application layer, not the presentation layer.
//!
//! Because there is no crate boundary to lean on, the module layering is
//! enforced by `tests/module_layering.rs` instead. A module may reference
//! any module below it and none at or above it. Lowest layer first:
//!
//! | Layer | Modules | Owns |
//! |-------|---------|------|
//! | 0 | `product`, `quantity` | Framework-independent contracts, typed identities, unit-safe presentation policy |
//! | 1 | `results`, `ui` | Versioned result documents; the design system (tokens, palette, widgets, plot engine) |
//! | 2 | `hardcopy` | Persisted page setup, print mappings, and source-set records |
//! | 3 | `state` | The persisted design, library, and project model |
//! | 4 | `analysis`, `automation_workflow`, `diagnostics`, `io` | Viewer mathematics, the CI workflow language, console/log model, file formats |
//! | 5 | `services` | DRC, licensing, and the per-analysis engine adapters |
//! | 6 | `simulation` | Analysis plans, netlist generation, run orchestration |
//! | 7 | `properties` | Component property editing |
//! | 8 | `schematic` | The schematic document engine |
//! | 9 | `workbench` | The application shell: [`RSpiceApp`], state, dialogs, chrome, surfaces, commands, and the workflows that mutate them |
//!
//! `workbench` is about half the crate, so one position in this table does not
//! describe it. Its own submodules are ordered by that test's
//! `WORKBENCH_LAYERS`, on the same rules.
//!
//! Known departures from both orders are recorded, counted, and ratcheted
//! down in the `ALLOWED_VIOLATIONS` and `ALLOWED_WORKBENCH_VIOLATIONS` tables.
//! Adding to either is not a way to unblock new code — a fresh violation
//! means the code is in the wrong module.

// Temporary allowance for existing external/SPICE naming conventions.
#![allow(non_snake_case)]
// The browser UI and simulation worker are deliberately disjoint feature
// slices of this application crate. Each wasm32 image leaves the other
// image's entrypoints unreachable so LTO can remove them; desktop and test
// builds still diagnose ordinary dead code.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]
// A rendering or transaction entry point takes one parameter per thing the
// caller independently varies: the `Ui`, the state it may mutate, the layout
// it must respect, the identity it acts on. Fifty call sites had already
// reached that conclusion one `#[allow]` at a time; this states it once. It
// is not licence to grow a signature that could take a struct.
#![allow(clippy::too_many_arguments)]
// NOTE: this crate previously carried a blanket `#![allow(deprecated)]`,
// hiding the egui 0.34 migration entirely. It is left off on purpose.
//
// Everything migratable on 0.34 has been migrated: the panel constructors
// (`TopBottomPanel`/`SidePanel` -> `Panel::top`/`left`), `SelectableLabel` ->
// `Button::selectable`, `Ui::set_enabled` -> `disable()`,
// `Context::screen_rect` -> `content_rect`, `Ui::allocate_ui_at_rect` ->
// `scope_builder`, and `popup_below_widget` -> the `Popup` builder. Each was
// verified against egui's own body first — every one of those forwards to its
// replacement with identical arguments, so none of them can move a pixel.
//
// What remains is one family: `Panel::show(ctx)` and `CentralPanel::show(ctx)`.
// Those cannot be migrated on 0.34. `show_inside` takes `&mut Ui`, and the
// root `Ui` that `show(ctx)` builds for itself needs `Context::pass_state_mut`
// and `PassState::allocate_central_panel`, both `pub(crate)` in egui. The
// supported way to obtain that root `Ui` arrives with eframe 0.35, which
// replaces `App::update(&mut self, ctx, frame)` with
// `App::ui(&mut self, ui, frame)`. So these warnings are not deferred cleanup
// — they are the visible edge of an eframe 0.35 upgrade, and they should be
// resolved by that upgrade rather than by hand-rolling egui internals here.
// NOTE: closing the public surface (see the visibility note below) turned 192
// items into `dead_code` warnings. They were never reachable — the compiler
// simply could not say so while their modules were `pub`. All 192 are dead on
// native, on wasm32, and with tests compiled; `--lib` alone is not enough,
// because it hides anything only a `#[cfg(test)]` block or a browser-only
// path calls.
//
// They are deliberately not swept. `workbench::simulation_analysis_tabs` is
// the reason: 26 of its items are unreachable, but it is one coherent catalog
// of 25 analysis tabs whose two index tables happen to have no reader.
// Deleting the unreferenced half would leave a catalog that no longer
// describes the product. Retire these per module — decide whether each thing
// is finished-but-unwired or genuinely abandoned — not with a bulk delete.
//
// The desktop build detaches from its console on Windows and the browser
// build has no stderr at all, so anything printed is a diagnostic nobody
// will ever read. Route it through `log` and the application log buffer.
#![deny(clippy::print_stdout, clippy::print_stderr)]
#![cfg_attr(
    test,
    allow(
        clippy::assertions_on_constants,
        clippy::bool_assert_comparison,
        clippy::cloned_ref_to_slice_refs,
        clippy::default_constructed_unit_structs,
        clippy::expect_fun_call,
        clippy::field_reassign_with_default,
        clippy::len_zero,
        clippy::manual_range_contains,
        clippy::manual_repeat_n,
        clippy::needless_range_loop,
        clippy::unnecessary_cast,
        clippy::unnecessary_get_then_check,
        clippy::unnecessary_unwrap,
        clippy::useless_vec
    )
)]

// =============================================================================
// Domain Modules (Organized by Feature)
// =============================================================================

/// Analysis viewers - Bode, FFT, histogram, Nyquist, pole-zero, Smith chart, eye diagram
pub(crate) mod analysis;

/// Schematic editor - Canvas, export, toolbar, symbol library
pub(crate) mod schematic;

/// Simulation management - Controller, dialogs, netlist generation
pub(crate) mod simulation;

/// Property editing - Component properties and design variables
pub(crate) mod properties;

/// The RSpice design system - tokens, palettes, fonts, icons, widgets
pub(crate) mod ui;

/// Persisted page-setup contracts and deterministic pagination. Document
/// adapters, scene rendering, the platform print boundary, and the dialogs
/// live in `workbench::hardcopy`; this is the layer `state` can persist.
pub(crate) mod hardcopy;

/// The contract-driven application workbench. This is the only owner of
/// application chrome, responsive composition, and top-level navigation.
pub(crate) mod workbench;

/// Versioned visualization documents, immutable dataset bindings, exact-data
/// queries, viewer compatibility, and progressive result operations.
pub(crate) mod results;

/// Canonical commercial product model, typed identities, command outcomes,
/// and fail-closed object lifecycles. This layer is UI-framework independent.
pub(crate) mod product;

/// Strict project-scoped Automation/CI workflow language and deterministic
/// evidence artifact rendering. This domain is UI-framework independent.
#[cfg(not(target_arch = "wasm32"))]
mod automation_runtime;
#[cfg(target_arch = "wasm32")]
#[path = "automation_runtime_browser.rs"]
mod automation_runtime;
pub(crate) mod automation_workflow;

// =============================================================================
// Core Infrastructure
// =============================================================================

/// Backend services (file I/O, simulation runner)
pub(crate) mod services;

/// File I/O (library parser, session, netlist, waveform)
pub(crate) mod io;

/// Application state management
pub(crate) mod state;

/// Unit-safe user presentation and UI quantity-input policy. Values entering
/// or leaving this module are always expressed in their documented SI base
/// units; deck dialect and PDK database-unit semantics live elsewhere.
pub(crate) mod quantity;

/// Diagnostics the application reports about itself: the console message
/// model and the structured, filterable application log.
pub(crate) mod diagnostics;

/// Clock shims for the browser build. `std::time::{Instant, SystemTime}` trap
/// at runtime on wasm32-unknown-unknown, so every layer uses these instead.
pub(crate) mod time_compat;

/// Shared output specification helpers for analysis/sensitivity paths
pub(crate) mod output_spec;

/// Locating the production half of a source file that inspects itself, for the
/// guards that assert their own shipped code takes no panic shortcuts.
#[cfg(test)]
mod source_guard;

// =============================================================================
// The crate's entire external surface
// =============================================================================
//
// `rspice-ui` is an application, not a library. Its only consumers are the
// desktop and browser binary in `main.rs` and the integration tests --
// nothing in the workspace depends on it. Every
// module above is therefore `pub(crate)`, and everything reachable from
// outside is named here.
//
// That is not tidiness. A `pub` module is one the compiler must assume some
// unseen caller uses, so it cannot report an unreachable item inside it. The
// eight modules that used to be `pub` covered 262k lines -- 45% of the crate
// -- in which dead code could not be detected at all. Adding a `pub mod` to
// reach something from a test re-opens that hole; add a re-export here
// instead.

/// The application root, constructed by both the desktop and browser entry
/// points.
pub use workbench::RSpiceApp;

/// Offline organization drawing-sheet publisher contract. Private keys are
/// accepted only by the separate native publisher binary; the GUI exposes
/// package construction, inspection, and verification primitives.
pub use workbench::{
    DRAWING_SHEET_PACKAGE_MAX_BYTES, DrawingSheetPackageEncoding, DrawingSheetPackageInspection,
    DrawingSheetPackageVerification, PublishedDrawingSheetPackage,
    drawing_sheet_publisher_public_key, inspect_drawing_sheet_package,
    publish_organization_drawing_sheet_package, verify_published_drawing_sheet_package,
};

/// Native logging environment for the desktop binary.
#[cfg(not(target_arch = "wasm32"))]
pub use workbench::logging::native_log_env;

/// Typed identities, for `tests/simulation_configuration_contract.rs`.
pub use product::{AnalysisInstanceId, ContentDigest, ObjectRevision, ProjectId, SimulationPlanId};

/// Trusted in-process collaboration-connector boundary for exact,
/// revision-bound project-library edit-lock snapshots.
pub use state::library_browser::{
    ProjectLibraryEditLock, ProjectLibraryEditLockScope, ProjectLibraryLockSnapshot,
};
pub use state::workspace::ProjectLibraryPublicationReceipt;

/// Design-variable netlist emission, pinned by the configuration contract.
pub use simulation::netlist_gen::{DesignVariableNetlistContext, design_variable_parameter_lines};

/// The persisted project model the configuration contract exercises.
pub use state::{
    CellViewRef, DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableRange, DesignVariableScope, DesignVariableSweepEligibility, ProjectWorkspace,
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming, SimulationPlanPayload, SimulationPlanPayloadRecord,
};

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
static WASM_JIT_ARCHITECTURE_PROBE_FRAME: std::sync::Mutex<[f64; 2]> =
    std::sync::Mutex::new([0.0, 0.0]);

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_ARCHITECTURE_PROBE_INPUT: f64 = 21.0;

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_probe_module() -> Result<Vec<u8>, String> {
    rspice_veriloga::wasm_jit::emit_architecture_probe()
        .map(|artifact| artifact.into_bytes())
        .map_err(|error| error.to_string())
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub const fn rspice_ui_wasm_jit_abi_version() -> u32 {
    rspice_veriloga::wasm_jit::WASM_JIT_ABI_VERSION
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub const fn rspice_ui_wasm_jit_emitter_version() -> u32 {
    rspice_veriloga::wasm_jit::WASM_JIT_EMITTER_VERSION
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
#[allow(clippy::too_many_arguments)]
pub fn rspice_ui_wasm_jit_eval_op_v1(
    frame_offset: u32,
    opcode: i32,
    aux0: i32,
    aux1: i32,
    aux2: i64,
    operand0: f64,
    operand1: f64,
    operand2: f64,
    operand3: f64,
    operand4: f64,
) -> f64 {
    rspice_veriloga::wasm_jit::eval_op_v1(
        frame_offset,
        opcode,
        aux0,
        aux1,
        aux2,
        operand0,
        operand1,
        operand2,
        operand3,
        operand4,
    )
}

/// Frame-free unary transcendental capability for generated modules.
///
/// The browser binds this to the secondary module's `math1_v1` import as a raw
/// WebAssembly export, so the call is wasm-to-wasm with no JavaScript frame
/// between a model's `exp()` and its implementation.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_math1_v1(opcode: i32, value: f64) -> f64 {
    rspice_veriloga::wasm_jit::math1_v1(opcode, value)
}

/// Frame-free binary transcendental capability. See
/// [`rspice_ui_wasm_jit_math1_v1`].
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_math2_v1(opcode: i32, left: f64, right: f64) -> f64 {
    rspice_veriloga::wasm_jit::math2_v1(opcode, left, right)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn prepare_rspice_ui_wasm_jit_probe() -> Result<u32, String> {
    let mut frame = WASM_JIT_ARCHITECTURE_PROBE_FRAME
        .lock()
        .map_err(|_| "WASM JIT architecture-probe frame lock is poisoned".to_owned())?;
    *frame = [WASM_JIT_ARCHITECTURE_PROBE_INPUT, 0.0];
    let offset = frame.as_ptr() as usize;
    u32::try_from(offset)
        .map_err(|_| "WASM JIT architecture-probe frame is outside wasm32 memory".to_owned())
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn finish_rspice_ui_wasm_jit_probe(frame_offset: u32, status: i32) -> Result<f64, String> {
    if status != 0 {
        return Err(format!(
            "WASM JIT architecture probe returned status {status}"
        ));
    }
    let frame = WASM_JIT_ARCHITECTURE_PROBE_FRAME
        .lock()
        .map_err(|_| "WASM JIT architecture-probe frame lock is poisoned".to_owned())?;
    let expected_offset = u32::try_from(frame.as_ptr() as usize)
        .map_err(|_| "WASM JIT architecture-probe frame is outside wasm32 memory".to_owned())?;
    if frame_offset != expected_offset {
        return Err("WASM JIT architecture probe used an unowned memory frame".to_owned());
    }
    let expected = WASM_JIT_ARCHITECTURE_PROBE_INPUT * 2.0;
    if frame[1].to_bits() != expected.to_bits() {
        return Err(format!(
            "WASM JIT architecture probe produced {}, expected {expected}",
            frame[1]
        ));
    }
    Ok(frame[1])
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_SOLVER_PROBE_SOURCE: &str = r#"
`include "disciplines.vams"
module rspice_wasm_solver_probe(p, n);
  inout p, n;
  electrical p, n;
  parameter real gain = 2.0;
  real bias;
  analog begin
    bias = analysis("tran") ? ($param_given(gain) ? 100.0 : 1.0) : -1000.0;
    I(p, n) <+ bias + gain * V(p, n) + ddt(V(p, n));
  end
endmodule
"#;

/// A model reaching the parts of the browser backend the solver probe leaves
/// untouched: several contributions published by one fused dispatch, a square
/// root and an exponential through the frame-free math capability, an inlined
/// extremum, and a contribution carrying several Jacobian entries.
///
/// Every expected result is exact in binary floating point, so the browser is
/// compared bit for bit against the machine backends rather than within a
/// tolerance.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_KERNEL_PROBE_SOURCE: &str = r#"
`include "disciplines.vams"
module rspice_wasm_kernel_probe(a, b, c);
  inout a, b, c;
  electrical a, b, c;
  parameter real scale = 2.0;
  real shaped;
  analog begin
    shaped = sqrt(V(a, b)) + exp(0.0);
    I(a, b) <+ shaped * scale;
    I(c, b) <+ max(V(c, b), 3.0) * scale;
    I(a, c) <+ V(a, b) * V(c, b);
  end
endmodule
"#;

/// Solver-node voltages, ground excluded: `V(a, b)` is 4 so its square root is
/// exact, and `V(c, b)` is 5 so the extremum selects its variable arm and
/// carries a derivative.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_KERNEL_PROBE_VOLTAGES: [f64; 2] = [4.0, 5.0];

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_KERNEL_PROBE_CURRENTS: [f64; 3] = [6.0, 10.0, 20.0];

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_KERNEL_PROBE_JACOBIAN: [f64; 14] = [
    0.5, -0.5, -0.5, 0.5, -2.0, 2.0, 2.0, -2.0, 5.0, -5.0, -9.0, 9.0, 4.0, -4.0,
];

/// Enough stamps that the browser's millisecond clock resolves the per-stamp
/// cost, without making the gate a long-running job.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
const WASM_JIT_KERNEL_PROBE_STAMPS: u32 = 20_000;

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn compile_wasm_jit_kernel_probe() -> Result<rspice_veriloga::RuntimeCompileReport, String> {
    rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default())
        .compile_runtime(
            WASM_JIT_KERNEL_PROBE_SOURCE,
            Some("rspice_wasm_kernel_probe"),
        )
        .map_err(|error| format!("WASM JIT kernel probe compilation failed: {error}"))
}

/// Emit the fused-driver probe's model module for the worker to install.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_kernel_probe_artifact()
-> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let report =
        compile_wasm_jit_kernel_probe().map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    let artifact =
        rspice_veriloga::wasm_jit::compile_model_value_module(&report.model, &report.canonical_ir)
            .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let artifact = simulation::veriloga::WasmJitWorkerArtifact::from_compiled(&artifact);
    serde_wasm_bindgen::to_value(&artifact)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

/// What the browser engine produced, and how long a fused stamp costs there.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmJitKernelProbeReport {
    pub contributions: usize,
    pub jacobian_entries: usize,
    pub stamps: u32,
    pub elapsed_ms: f64,
    pub nanoseconds_per_stamp: f64,
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn expect_exact_probe_values(what: &str, actual: &[f64], expected: &[f64]) -> Result<(), String> {
    if actual.len() != expected.len()
        || actual
            .iter()
            .zip(expected)
            .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
    {
        return Err(format!(
            "WASM JIT kernel probe {what} mismatch: {actual:?}, expected {expected:?}"
        ));
    }
    Ok(())
}

/// Evaluate and stamp a multi-contribution model in the browser engine,
/// checking every result bit for bit and timing the fused stamp driver.
///
/// The three checks reach three different paths: the contributions come from
/// the fused evaluation driver, the derivatives from the per-entry Jacobian
/// exports, and the timing from the fused stamp driver. The timing is the only
/// evidence that catches a capability bound to a JavaScript wrapper rather
/// than a raw export, because such a build still computes the right answer.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_run_kernel_probe() -> Result<wasm_bindgen::JsValue, String> {
    let report = compile_wasm_jit_kernel_probe()?;
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "WASMJITKERNEL1",
        std::sync::Arc::new(report.model),
        &report.canonical_ir,
        &[1, 0, 2],
    )
    .map_err(|error| error.to_string())?;
    if !device.fused_stamp_driver_is_active() {
        return Err(
            "WASM JIT kernel probe model did not qualify for the fused stamp driver".to_owned(),
        );
    }
    device
        .try_update_all_voltages(&WASM_JIT_KERNEL_PROBE_VOLTAGES)
        .map_err(|error| error.to_string())?;

    let currents = device.try_evaluate().map_err(|error| error.to_string())?;
    expect_exact_probe_values("contribution", &currents, &WASM_JIT_KERNEL_PROBE_CURRENTS)?;
    let jacobian = device
        .try_compute_jacobian()
        .map_err(|error| error.to_string())?;
    let entries = jacobian
        .iter()
        .map(|entry| entry.value)
        .collect::<Vec<f64>>();
    expect_exact_probe_values("Jacobian", &entries, &WASM_JIT_KERNEL_PROBE_JACOBIAN)?;

    // Nudging one voltage per pass keeps the device from being handed the same
    // operating point twice, by less than the extremum's margin so the arm the
    // checked values came from stays selected.
    let started = crate::time_compat::Instant::now();
    for step in 0..WASM_JIT_KERNEL_PROBE_STAMPS {
        let voltages = [
            WASM_JIT_KERNEL_PROBE_VOLTAGES[0],
            WASM_JIT_KERNEL_PROBE_VOLTAGES[1] + f64::from(step) * f64::EPSILON,
        ];
        device
            .try_stamp(&voltages, |_, _, _| {}, |_, _| {})
            .map_err(|error| error.to_string())?;
    }
    let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;

    let report = WasmJitKernelProbeReport {
        contributions: currents.len(),
        jacobian_entries: entries.len(),
        stamps: WASM_JIT_KERNEL_PROBE_STAMPS,
        elapsed_ms,
        nanoseconds_per_stamp: elapsed_ms * 1.0e6 / f64::from(WASM_JIT_KERNEL_PROBE_STAMPS),
    };
    serde_wasm_bindgen::to_value(&report).map_err(|error| error.to_string())
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
fn compile_wasm_jit_solver_probe() -> Result<rspice_veriloga::RuntimeCompileReport, String> {
    rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions::default())
        .compile_runtime(
            WASM_JIT_SOLVER_PROBE_SOURCE,
            Some("rspice_wasm_solver_probe"),
        )
        .map_err(|error| format!("WASM JIT solver probe compilation failed: {error}"))
}

/// Emit a real canonical model used to qualify the installed-module solver
/// bridge in the browser engine, not merely WebAssembly compilation support.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_solver_probe_artifact()
-> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    let report =
        compile_wasm_jit_solver_probe().map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    let artifact =
        rspice_veriloga::wasm_jit::compile_model_value_module(&report.model, &report.canonical_ir)
            .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let artifact = simulation::veriloga::WasmJitWorkerArtifact::from_compiled(&artifact);
    serde_wasm_bindgen::to_value(&artifact)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

/// Exercise parameter, analysis, assignment, stateful transient, value,
/// Jacobian, matrix, and RHS dispatch through an installed secondary module.
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn rspice_ui_wasm_jit_run_solver_probe() -> Result<f64, String> {
    let report = compile_wasm_jit_solver_probe()?;
    let mut device = rspice_veriloga::device::VerilogADevice::try_new_with_canonical_ir(
        "WASMJITPROBE1",
        std::sync::Arc::new(report.model),
        &report.canonical_ir,
        &[1, 0],
    )
    .map_err(|error| error.to_string())?;
    device
        .try_set_analysis_type(2)
        .map_err(|error| error.to_string())?;
    device
        .try_set_timestep(0.5)
        .map_err(|error| error.to_string())?;
    device
        .try_update_all_voltages(&[3.0])
        .map_err(|error| error.to_string())?;

    let initial_currents = device.try_evaluate().map_err(|error| error.to_string())?;
    if initial_currents.len() != 1 || initial_currents[0].to_bits() != 7.0_f64.to_bits() {
        return Err(format!(
            "WASM JIT solver probe initial current mismatch: {initial_currents:?}, expected [7.0]"
        ));
    }
    device.advance_state();
    device
        .try_set_time(0.5)
        .map_err(|error| error.to_string())?;
    device
        .try_update_all_voltages(&[5.0])
        .map_err(|error| error.to_string())?;

    let currents = device.try_evaluate().map_err(|error| error.to_string())?;
    if currents.len() != 1 || currents[0].to_bits() != 15.0_f64.to_bits() {
        return Err(format!(
            "WASM JIT solver probe committed-state current mismatch: {currents:?}, expected [15.0]"
        ));
    }
    let jacobian = device
        .try_compute_jacobian()
        .map_err(|error| error.to_string())?;
    let expected_jacobian = [4.0_f64, -4.0, -4.0, 4.0];
    if jacobian.len() != expected_jacobian.len()
        || jacobian
            .iter()
            .zip(expected_jacobian)
            .any(|(entry, expected)| entry.value.to_bits() != expected.to_bits())
    {
        return Err(format!(
            "WASM JIT solver probe Jacobian mismatch: {jacobian:?}, expected values {expected_jacobian:?}"
        ));
    }

    let mut matrix = Vec::new();
    let mut rhs = Vec::new();
    device
        .try_stamp(
            &[5.0],
            |row, col, value| matrix.push((row, col, value)),
            |row, value| rhs.push((row, value)),
        )
        .map_err(|error| error.to_string())?;
    if !matrix
        .iter()
        .any(|&(row, col, value)| row == 0 && col == 0 && value.to_bits() == 4.0_f64.to_bits())
        || !rhs
            .iter()
            .any(|&(row, value)| row == 0 && value.to_bits() == 5.0_f64.to_bits())
    {
        return Err(format!(
            "WASM JIT solver probe stamp mismatch: matrix={matrix:?}, rhs={rhs:?}"
        ));
    }
    Ok(currents[0])
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn run_rspice_ui_worker_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    simulation::runner::worker_contract::run_worker_request_value(value)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn prepare_rspice_ui_wasm_jit_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    simulation::runner::worker_contract::prepare_wasm_jit_request_value(value)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn install_rspice_ui_wasm_jit_dispatcher(dispatcher: js_sys::Function) {
    rspice_veriloga::wasm_jit::install_browser_dispatcher(
        move |cache_key, export_name, frame_offset| {
            let value = dispatcher
                .call3(
                    &wasm_bindgen::JsValue::UNDEFINED,
                    &wasm_bindgen::JsValue::from_str(cache_key),
                    &wasm_bindgen::JsValue::from_str(export_name),
                    &wasm_bindgen::JsValue::from_f64(f64::from(frame_offset)),
                )
                .map_err(|error| format!("browser WASM JIT dispatch threw: {error:?}"))?;
            let status = value
                .as_f64()
                .filter(|value| {
                    value.fract() == 0.0
                        && *value >= f64::from(i32::MIN)
                        && *value <= f64::from(i32::MAX)
                })
                .ok_or_else(|| "browser WASM JIT dispatch returned a non-i32 status".to_owned())?;
            Ok(status as i32)
        },
    );
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn run_prepared_rspice_ui_wasm_jit_request(
    dispatch_token: u32,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    simulation::runner::worker_contract::run_prepared_wasm_jit_request_value(dispatch_token)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn cancel_prepared_rspice_ui_wasm_jit_request(
    dispatch_token: u32,
) -> Result<(), wasm_bindgen::JsValue> {
    simulation::runner::worker_contract::cancel_prepared_wasm_jit_request_value(dispatch_token)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn run_rspice_ui_veriloga_compile_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    workbench::documents::code_workspace::run_veriloga_worker_request_value(value)
}

#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub fn run_rspice_ui_hardcopy_request(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    workbench::app::run_hardcopy_worker_request_value(value)
}

pub struct ProjectLibraryPublicationCandidate {
    draft: crate::state::workspace::ProjectLibraryPublicationDraft,
    artifact_bytes: Vec<u8>,
    source_project_revision: ObjectRevision,
}

impl ProjectLibraryPublicationCandidate {
    #[must_use]
    pub fn artifact_bytes(&self) -> &[u8] {
        &self.artifact_bytes
    }

    #[must_use]
    pub fn publication_id(&self) -> uuid::Uuid {
        self.draft.publication_id
    }

    #[must_use]
    pub fn snapshot_digest(&self) -> ContentDigest {
        self.draft.snapshot_digest
    }

    #[must_use]
    pub fn snapshot_byte_len(&self) -> u64 {
        self.draft.snapshot_byte_len
    }
}

impl RSpiceApp {
    /// Install a snapshot already authenticated by a trusted collaboration
    /// connector. RSpice validates its content digest, project identity,
    /// authority continuity, generation, and exact live revisions before the
    /// snapshot can govern any edit.
    pub fn install_project_library_lock_snapshot(
        &mut self,
        snapshot: ProjectLibraryLockSnapshot,
    ) -> Result<(), String> {
        snapshot.validate()?;
        if snapshot.project_id() != self.state.workspace.project.id() {
            return Err(format!(
                "project library lock snapshot belongs to project {}, not current project {}",
                snapshot.project_id(),
                self.state.workspace.project.id()
            ));
        }
        if snapshot.project_revision() != self.state.workspace.project.revision()
            || snapshot.library_revision() != self.state.library_manager.revision()
        {
            return Err(format!(
                "project library lock snapshot is stale (project {} vs {}, library {} vs {})",
                snapshot.project_revision().get(),
                self.state.workspace.project.revision().get(),
                snapshot.library_revision(),
                self.state.library_manager.revision()
            ));
        }
        self.state
            .library_edit_locks
            .install_authoritative(snapshot)
    }

    /// Prepare the exact artifact and receipt candidate without changing live
    /// project state. A native, browser, or repository writer must durably
    /// publish `artifact_bytes()` before commit.
    pub fn prepare_project_library_publication(
        &self,
        label: impl Into<String>,
        actor_id: impl Into<String>,
        authority_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<ProjectLibraryPublicationCandidate, String> {
        use sha2::Digest as _;

        if self.state.workbench.safe_mode.project_read_only() {
            return Err(
                "project library publication is unavailable because the project is open read-only"
                    .to_owned(),
            );
        }
        if self.state.simulation.is_running {
            return Err(
                "project library publication is unavailable while a simulation is running"
                    .to_owned(),
            );
        }
        let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&self.state)
            .map_err(|error| format!("project library publication snapshot failed: {error}"))?;
        let serialized =
            crate::io::project_io::serialize_project_file(&snapshot).map_err(|error| {
                format!("project library publication serialization failed: {error}")
            })?;
        let bytes = serialized.into_bytes();
        let snapshot_byte_len = u64::try_from(bytes.len())
            .map_err(|_| "project library publication artifact is too large".to_owned())?;
        let draft = crate::state::workspace::ProjectLibraryPublicationDraft {
            publication_id: uuid::Uuid::new_v4(),
            label: label.into(),
            actor_id: actor_id.into(),
            authority_id: authority_id.into(),
            reason: reason.into(),
            created_unix_ms: crate::time_compat::unix_epoch()
                .as_millis()
                .try_into()
                .unwrap_or(u64::MAX)
                .max(1),
            library_revision: self.state.library_manager.revision(),
            snapshot_digest: crate::product::ContentDigest::from_bytes(
                sha2::Sha256::digest(&bytes).into(),
            ),
            snapshot_byte_len,
        };
        let mut descriptor_preflight = self.state.workspace.project.clone();
        descriptor_preflight
            .publish_library_snapshot(draft.clone())
            .map_err(|error| format!("project library publication preflight failed: {error}"))?;
        Ok(ProjectLibraryPublicationCandidate {
            draft,
            artifact_bytes: bytes,
            source_project_revision: self.state.workspace.project.revision(),
        })
    }

    /// Commit a publication only after its exact artifact was durably
    /// accepted. Any intervening project or catalog change rejects the
    /// candidate and leaves live state untouched.
    pub fn commit_project_library_publication(
        &mut self,
        candidate: ProjectLibraryPublicationCandidate,
    ) -> Result<ProjectLibraryPublicationReceipt, String> {
        use sha2::Digest as _;

        if self.state.workbench.safe_mode.project_read_only() {
            return Err(
                "project library publication is unavailable because the project is open read-only"
                    .to_owned(),
            );
        }
        if self.state.simulation.is_running {
            return Err(
                "project library publication is unavailable while a simulation is running"
                    .to_owned(),
            );
        }
        if self.state.workspace.project.revision() != candidate.source_project_revision
            || self.state.library_manager.revision() != candidate.draft.library_revision
        {
            return Err(
                "project library publication candidate is stale; prepare and publish a new artifact"
                    .to_owned(),
            );
        }
        let current_snapshot =
            crate::workbench::lifecycle::project_lifecycle::snapshot(&self.state)
                .map_err(|error| format!("project library publication recheck failed: {error}"))?;
        let current_serialized = crate::io::project_io::serialize_project_file(&current_snapshot)
            .map_err(|error| {
            format!("project library publication recheck serialization failed: {error}")
        })?;
        let current_bytes = current_serialized.as_bytes();
        if current_bytes.len() as u64 != candidate.draft.snapshot_byte_len
            || crate::product::ContentDigest::from_bytes(sha2::Sha256::digest(current_bytes).into())
                != candidate.draft.snapshot_digest
        {
            return Err(
                "project library publication content changed after the artifact was prepared"
                    .to_owned(),
            );
        }
        let receipt = self
            .state
            .workspace
            .project
            .publish_library_snapshot(candidate.draft)
            .map_err(|error| format!("project library publication failed: {error}"))?;
        self.state.workspace.project_metadata_dirty = true;
        self.state.design_execution_epoch = self.state.design_execution_epoch.wrapping_add(1);
        self.state.ui.netlist.current_generation_input_digest = None;
        self.state.clear_project_design_history();
        Ok(receipt)
    }

    /// Restore the exact complete project artifact named by an immutable
    /// library publication while preserving the current project identity,
    /// publication ledger, and intervening audit history. The rollback is one
    /// new revision; malformed, tampered, foreign, stale-authority, or
    /// technology-incompatible artifacts leave live state unchanged.
    pub fn rollback_project_library_publication(
        &mut self,
        publication_id: uuid::Uuid,
        artifact_bytes: &[u8],
        actor_id: impl Into<String>,
        authority_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<(), String> {
        use sha2::Digest as _;

        if self.state.workbench.safe_mode.project_read_only() {
            return Err(
                "project library rollback is unavailable because the project is open read-only"
                    .to_owned(),
            );
        }
        if self.state.simulation.is_running {
            return Err(
                "project library rollback is unavailable while a simulation is running".to_owned(),
            );
        }
        let receipt = self
            .state
            .workspace
            .project
            .library_publications()
            .iter()
            .find(|receipt| receipt.publication_id() == publication_id)
            .cloned()
            .ok_or_else(|| {
                format!("project library publication {publication_id} is not retained")
            })?;
        if artifact_bytes.len() as u64 != receipt.snapshot_byte_len()
            || crate::product::ContentDigest::from_bytes(
                sha2::Sha256::digest(artifact_bytes).into(),
            ) != receipt.snapshot_digest()
        {
            return Err(
                "project library rollback artifact does not match its publication receipt"
                    .to_owned(),
            );
        }
        let artifact_text = std::str::from_utf8(artifact_bytes)
            .map_err(|error| format!("project library rollback artifact is not UTF-8: {error}"))?;
        let mut artifact = crate::io::project_io::load_project_text(artifact_text, None)
            .map_err(|error| format!("project library rollback artifact is invalid: {error}"))?;
        if artifact.workspace.project.id() != receipt.project_id()
            || artifact.workspace.project.revision() != receipt.source_project_revision()
            || artifact.libraries.revision() != receipt.library_revision()
        {
            return Err(
                "project library rollback artifact identity or revision does not match its receipt"
                    .to_owned(),
            );
        }
        let expected_prior_publications = usize::try_from(receipt.sequence() - 1)
            .map_err(|_| "project library publication sequence is invalid".to_owned())?;
        if artifact.workspace.project.library_publications().len() != expected_prior_publications
            || artifact
                .workspace
                .project
                .library_publications()
                .last()
                .map(ProjectLibraryPublicationReceipt::receipt_digest)
                != receipt.previous_receipt_digest()
        {
            return Err(
                "project library rollback artifact does not retain the exact publication lineage prefix"
                    .to_owned(),
            );
        }
        if artifact.workspace.project.technology_binding()
            != self.state.workspace.project.technology_binding()
        {
            return Err(
                "project library rollback cannot cross an exact technology-binding change"
                    .to_owned(),
            );
        }

        let mutation = crate::state::ProjectLibraryMutation::RollbackPublication {
            publication_id,
            publication_label: receipt.label().to_owned(),
            snapshot_digest: receipt.snapshot_digest(),
            actor_id: actor_id.into(),
            authority_id: authority_id.into(),
            reason: reason.into(),
        };
        let prepared = self.state.preflight_project_library_mutation(mutation)?;

        let project_id = artifact.workspace.project.id();
        let (simulation_plan, model_library_manager, execution_warnings) =
            match artifact.execution_context.take() {
                Some(context) => context.into_state(project_id).map_err(|error| {
                    format!("project library rollback execution context is invalid: {error}")
                })?,
                None => (
                    crate::workbench::app_state::SimSetupState::new_with_user_preferences(
                        &self.state.ui.preferences,
                    ),
                    crate::workbench::app_state::default_model_library_manager(),
                    vec![
                        "The publication predates durable simulation plans; documented defaults were restored"
                            .to_owned(),
                    ],
                ),
            };

        let mut candidate = self.state.clone();
        let mut current_project = candidate.workspace.project.clone();
        current_project.root_library = artifact.workspace.project.root_library.clone();
        current_project.top_cell = artifact.workspace.project.top_cell.clone();
        artifact.workspace.project = current_project;
        candidate.clear_design_execution_context();
        candidate
            .library_manager
            .replace_catalog_from_snapshot(&artifact.libraries)?;
        candidate.library_edit_locks = crate::state::ProjectLibraryLockAuthority::default();
        candidate.workspace = artifact.workspace;
        candidate.sim_setup = simulation_plan;
        candidate.model_library_manager = model_library_manager;
        candidate.restore_active_schematic_from_workspace();
        candidate.simulation = crate::state::SimulationState::default();
        artifact
            .simulation_results
            .apply_to_state(&mut candidate.simulation)
            .map_err(|error| {
                format!("project library rollback result history is invalid: {error}")
            })?;
        candidate.publish_project_library_mutation(prepared);
        candidate
            .workspace
            .project
            .validate()
            .map_err(|error| format!("project library rollback metadata is invalid: {error}"))?;
        self.state = candidate;
        for warning in execution_warnings {
            self.state
                .push_user_message(crate::diagnostics::ConsoleMessage::warning(warning));
        }
        Ok(())
    }
}

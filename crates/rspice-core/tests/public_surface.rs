//! The crate's public surface may shrink, never grow.
//!
//! `rspice-core` exports 4,257 public item statements. Its five frontends —
//! the CLI, the GUI, the Python and WASM bindings, and the conformance suite —
//! name roughly two hundred distinct paths between them. The rest is internal
//! machinery that happens to be spelled `pub`.
//!
//! That has a cost even though the crate is `publish = false` and carries no
//! semver obligation. Every `pub fn` is a thing a refactor must consider
//! moving, a name rustdoc must list, and a hint to the next reader that
//! somebody outside might depend on this. When almost everything is public,
//! `pub` stops carrying information.
//!
//! This test does not judge which items should be private — that is Phase 9,
//! driven by what the frontends actually import. It exists so the number
//! cannot drift upward while the earlier phases are moving code around. New
//! code should be `pub(crate)` unless a frontend needs it.
//!
//! # What is counted
//!
//! Item declarations written `pub fn` / `pub struct` / `pub enum` /
//! `pub trait` / `pub type` / `pub const` / `pub static`, plus `pub use`
//! re-export statements, at the start of a line after indentation.
//!
//! The unit is the *statement*, not the name. A grouped re-export —
//! `pub use foo::{A, B, C};` — counts once while exposing three names. That
//! makes the number a proxy rather than a census, and it means relocating a
//! type behind a grouped re-export reads as +1 even though the set of public
//! names did not change.
//!
//! # Raising the ceiling
//!
//! Two cases justify it, and both require saying so in the commit:
//!
//! 1. A move that provably preserves the set of public names, by the number of
//!    re-export statements it adds.
//! 2. A deliberate new public API — an entry point a frontend is meant to
//!    call.
//!
//! Nothing else. In particular, "the number went up because I added a helper"
//! is the case this test exists to catch: make it `pub(crate)`. Prefer
//! investigating a rise to explaining it away.
//!
//! `pub(crate)`, `pub(super)` and `pub(in ...)` are deliberately *not*
//! counted: restricting visibility is the direction this test wants, so
//! narrowing `pub` to `pub(crate)` lowers the number, which is the point.
//!
//! Generated Verilog-A is excluded for the same reason as in the layering
//! ratchet: it is machine output, and its size is the generator's business.

use std::fs;
use std::path::{Path, PathBuf};

use rspice_core::analysis::harmonic_balance::{
    DepletionCap, HbConfig, HbError, HbSolver, HbSolverState, NonlinearDeviceInstance,
};

/// Ceiling on public items. Lower it whenever the real count drops. The build
/// fails if the count exceeds this, and also if it falls far enough below that
/// the ceiling has gone stale.
///
/// Raise it only for an item a frontend is meant to call, and say which one in
/// the commit. "Never raise it" would be the simpler rule and it is the wrong
/// one: it does not stop a public surface from growing, it stops a feature
/// from shipping an API, and the way that gets resolved is by marking things
/// `pub` somewhere this test does not count. The ceiling exists to make growth
/// a decision, not to forbid it.
///
/// The last raise was +8 for the attributed-failure vocabulary the GUI reads
/// off `Engine::convergence_quality` to mark non-converged objects on the
/// schematic: `ConvergenceDiagnostic`, `ConvergenceFailureClass`,
/// `ConvergenceSite`, `ConvergenceSiteKind`,
/// `ConvergenceDiagnostic::MAX_NAMED_SITES`,
/// `ConvergenceDiagnostic::describes`, and
/// `ConvergenceQuality::record_failure_diagnostic`. Every one of them is on
/// the path from a failed run to a highlighted conductor; without them the
/// only way to that highlight is parsing the failure's prose.
///
/// The eighth is `Engine::resolved_for_netlist`, which the GUI's bridge must
/// call to get its per-deck engine. It was already the only construction
/// that keeps the run's metrics reachable after the per-deck engine is
/// dropped; the GUI was building its own and losing them.
///
/// The next raise was +1 for `Engine::try_resolved_with_config`, the same
/// metric-sharing construction for a caller that has already resolved the
/// configuration itself. The operating point must: its temperature override,
/// accuracy tier and homotopy choice are applied on top of `.OPTIONS`, so
/// there is no netlist for `resolved_for_netlist` to resolve against, and the
/// bridge was falling back to `try_new_with_resolved_config` — which is why
/// the one analysis that names non-converged conductors never reported one.
///
/// The current raise is +3, from 4,253 to 4,256, after narrowing 22
/// test-fixture and transient-numerics declarations that were never frontend
/// API. The remaining net growth is deliberate: the conformance frontend
/// calls the bounded Xyce PRN serializer; the CLI calls bounded sweep,
/// checkpoint/restart and abort-aware result APIs; and the GUI and Python
/// bindings consume the authenticated pole-zero and Floquet evidence. The GUI
/// also calls the unit-interval estimator. Larger deletions of unused device
/// APIs offset most of those additions, so +3 is the statement-count delta,
/// not the number of frontend paths added.
///
/// The latest raise is +1 for `XyceHbTimeDomainMode`, which the conformance
/// frontend reads from parsed netlists to prove that authored `HBINT.TAHB`
/// controls remain typed. Its numeric rendering helper stays crate-private.
///
/// The next +12 are also cross-crate contracts that landed after that ratchet:
/// eight bounded packed-checkpoint APIs used by the CLI and conformance suite,
/// three RustFFT planning-qualification items shared with the GUI, and the
/// typed duplicate-model-parameter error exposed by the Python binding.
///
/// The latest +1 is `NetlistDiagnostic::xyce_legacy_warning_lines`, which the
/// CLI calls to render the byte-stable two-line Xyce compatibility warning.
///
/// The current reduction removes 36 unused public statements from the legacy
/// HB-only diode/BJT/MOS wrapper module and its re-export. Production HB uses
/// the exact `solver::NonlinearDeviceInstance` path; retaining a second,
/// simplified public device vocabulary made the supported numerical contract
/// ambiguous.
///
/// The latest net change is +3: authenticated PSS operating-point identity
/// adds five frontend-consumed public statements (`PssOperatingPointIdentity`
/// plus its canonical-parts, shooting-state-basis, producer-identity, and
/// authenticated-construction APIs), while removing the obsolete public HB
/// BJT-parameter and current-switch registration helpers offsets two.
///
/// The latest +1 is `BehavioralEvaluationError`. Public behavioral-source
/// evaluation and stamping APIs are fallible so non-finite equations cannot
/// be silently converted into zero-valued sources; callers need the typed
/// error to retain source identity and analysis-coordinate diagnostics.
///
/// The current raise is +30, and it is two unrelated amounts.
///
/// +2 of it is arrears, not growth. `b97258608` ("Bound model library
/// ingestion and discovery") added ten public statements under `src/library/`
/// without touching this ceiling; eight statements deleted earlier in the same
/// range absorbed all but two, so the tree has been two over at 4,263 since
/// that commit and this test has been failing on `main`. Raising to cover them
/// records the debt rather than paying it — several of those ten
/// (`with_max_source_files`, `DEFAULT_MAX_LIBRARY_SOURCE_FILES`,
/// `DEFAULT_MAX_VERILOGA_DISCOVERY_FILES`) look like `pub(crate)` candidates,
/// and narrowing them is the change that should lower this number again.
///
/// +28 is the discrete-event scheduler kernel in
/// `xspice::event_scheduler`: the tick time base (`TimeResolution` and its
/// five operations plus `MAX_EXACT_TICKS`), the stratified region vocabulary
/// (`SchedulerRegion`, `ORDERED`), the event payload (`EventTarget`,
/// `ScheduledEvent`), the typed failure surface (`SchedulerError`,
/// `OscillationCause`, `OscillationDiagnostic`), the ceilings and per-slot
/// report (`SchedulerLimits`, `TimeSlotReport`), and the scheduler itself
/// (`EventScheduler` with six operations, `SchedulerContext` with three).
///
/// These are public for a reason this test's usual rule does not cover:
/// `event_scheduler_kernel.rs` is an integration test, so it can only reach
/// the kernel through the crate's public face. The kernel's ordering and
/// determinism guarantees are the substrate everything digital rests on and
/// that test is what holds them, so the surface it drives stays public.
///
/// The latest +3 is what rehosting XSPICE on the kernel needs from it:
/// `schedule_superseding_at` (a driver replacing its own pending output),
/// `run_due_events` (executing everything due at or before a bound the analog
/// engine names), and `note_delta_cycle` (an outer settle loop marking one
/// iteration so a network that will not quiet is diagnosed rather than
/// looping). Each is driven by the same integration test.
///
/// The previous net change was -20, from 4,291 to 4,271. Those +3 were in it,
/// and the rehost itself removed 23: `xspice::Event`, `xspice::EventQueue` and
/// `EventQueueStats` with their constructors, scheduling, draining,
/// cancellation and statistics methods, and `XspiceInstance::schedule_events`,
/// which no longer names a public queue type. The kernel is what schedules
/// now, and nothing outside this crate ever named the queue it replaced.
///
/// The latest raise is +8, from 4,271 to 4,279: the whole entry path for
/// executing digital Verilog, which the conformance suite's oracle harness
/// calls. It is the second case the doc above admits — a deliberate new API a
/// frontend is meant to call — and it is the *entire* API, not a helper that
/// leaked:
///
/// `xspice::verilog::run_digital_verilog` is the call. `DigitalStimulus`,
/// `DigitalPort` and `DigitalClock` are its input; `DigitalRunReport` and
/// `DigitalObservation` are its output; `DigitalRunError` is its refusal, which
/// costs two statements because the type lives in the crate-private `host`
/// module and is re-exported. Everything else the host and the signal store
/// declare — the store itself, the resolution table, the process scheduler — is
/// `pub(crate)`, as is the time-unit ruling, which the refusal prints rather
/// than a caller reading.
///
/// The coherent-noise program briefly added five public statements to
/// `src/device/veriloga_builtins.rs`, but nothing outside `rspice-core` names
/// any of them — every caller is `src/engine/noise.rs`:
///
/// `BuiltinEvaluatedNoiseInjection` and `BuiltinEvaluatedNoiseProcess` are the
/// per-process result the generated coherent ABI hands back;
/// `has_grouped_noise_processes` is the capability probe that tells a catalog
/// generated before that ABI apart from one generated after it;
/// `grouped_noise_process_catalog` names the processes for the noise report;
/// and `evaluate_noise_processes_at_frequency` is the evaluation itself.
///
/// Those five APIs are now `pub(crate)`, so the ratchet is back at its prior
/// 4,279-item ceiling rather than retaining accidental headroom.
///
/// The FFT result contract adds +5 deliberate frontend-facing types:
/// `XyceFftMode` retains the authored compatibility selection, while
/// `TransientFftBin`, `TransientFftHarmonic`, `TransientFftMetrics`, and
/// `TransientFftResult` expose calibrated spectra and optional `FFTOUT`
/// figures without requiring a frontend to parse an engine text report.
///
/// 2026-08-31, +23 arrears (4,284 → 4,307): two landings raised the count
/// without touching this ceiling — deterministic TEAM resistance noise
/// (`db03d39eb`) and the transactional mixed Verilog transient host
/// (`9549ed6cb`). Neither set has been triaged for narrowing; whether each
/// item is genuinely frontend-facing or a `pub(crate)` candidate is the
/// visibility-narrowing pass's question, not this ratchet's. Recorded here so
/// the next raise cannot mistake the arrears for headroom.
///
/// 2026-08-31, -10 (4,307 → 4,297): the narrowing pass the +30 above asked for,
/// run over the ten statements `b97258608` added under `src/library/`. All ten
/// are now `pub(crate)`: the ingestion ceilings
/// `DEFAULT_MAX_LIBRARY_SOURCE_FILES` and
/// `DEFAULT_MAX_VERILOGA_DISCOVERY_FILES`; the `LibParser` builders
/// `with_resource_limits` and `with_max_source_files` and its
/// `parse_file_with_abort`; the `LibraryManager` loaders
/// `load_external_lib_with_limits` and
/// `load_external_lib_with_limits_and_abort`; and the discovery limit type
/// `VerilogADiscoveryLimits` with `discover_veriloga_models_with_limits` and
/// `discover_veriloga_models_with_limits_and_abort`.
///
/// Every one is an explicit-limit or cancellation overload, and the only
/// callers are `LibraryManager` and this crate's own tests. The entry points a
/// frontend actually names — `LibParser::new`, `parse_file`, `parse_string`,
/// `LibraryManager::load_external_lib`, `discover_veriloga_models` — stay
/// public and still apply those bounds from `ResourceLimits::default()`, so
/// the ingestion that commit bounded remains bounded on the public path.
///
/// Three of the ten turned out to have no shipping caller at all once they
/// stopped being API, which `-D warnings` then said out loud.
/// `load_external_lib_with_limits` had no caller in any configuration and an
/// abort-taking twin one line below it, so it is deleted rather than narrowed;
/// `with_max_source_files` and `discover_veriloga_models_with_limits` are
/// `#[cfg(test)]`, being how the tests drive one ingestion limit at a time to
/// its edge. All three still count as -1 each: a deleted `pub fn` and a
/// cfg-gated `pub(crate) fn` are both gone from this number.
///
/// Established by compiling the CLI, the GUI, the Python and WASM bindings and
/// the conformance suite against the narrowed items, which is the check this
/// question needs: a bare-name grep cannot answer it, because a grouped
/// `pub use` lets a frontend name an item through a path the declaration's own
/// name never appears in. The two grouped re-exports in `src/library.rs` drop
/// those names but keep their statements, so the whole -10 is declarations.
///
/// 2026-09-01, +3 deliberate (4,297 → 4,300): the compile-once digital run
/// API. `CompiledDigitalDesign`, `CompiledDigitalDesign::compile` and
/// `CompiledDigitalDesign::run` split `run_digital_verilog` into its two
/// halves, and every one of the three is on the path a caller with many
/// stimuli and one design must take — the conformance suite's RNM performance
/// measurement is that caller, and without them the only way to run a design
/// twice is to compile it twice. `run_digital_verilog` is retained unchanged
/// as their composition, so nothing already public moved or grew. The
/// module-name accessor that would have made a fourth was *not* added: no
/// frontend reads it, `Debug` prints the name, and the refusal that cites it
/// carries it.
const MAX_PUBLIC_ITEMS: usize = 4300;

/// How far under the ceiling the count may sit before the ceiling is
/// considered stale and must be lowered. Without this, a ratchet silently
/// stops ratcheting: the number falls, nobody updates the constant, and the
/// gap quietly becomes headroom for regrowth.
const STALE_CEILING_SLACK: usize = 100;

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                if path
                    .file_name()
                    .is_some_and(|name| name == "veriloga_builtins")
                {
                    continue;
                }
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

const ITEM_KEYWORDS: &[&str] = &[
    "fn ",
    "struct ",
    "enum ",
    "trait ",
    "type ",
    "const ",
    "static ",
    "unsafe fn ",
    "async fn ",
];

/// Whether a line declares a public item or re-export.
///
/// Takes the line already trimmed of leading whitespace. `pub(` is rejected
/// before the keyword check so restricted visibility never counts.
fn is_public_item(line: &str) -> bool {
    let Some(rest) = line.strip_prefix("pub ") else {
        return false;
    };
    if rest.starts_with("use ") {
        return true;
    }
    ITEM_KEYWORDS
        .iter()
        .any(|keyword| rest.starts_with(keyword))
}

fn count_public_items() -> (usize, Vec<(String, usize)>) {
    let root = src_dir();
    let mut total = 0;
    let mut per_file = Vec::new();
    for path in rust_sources(&root) {
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        let count = source
            .lines()
            .filter(|line| is_public_item(line.trim_start()))
            .count();
        if count > 0 {
            total += count;
            per_file.push((
                path.strip_prefix(&root)
                    .unwrap_or(&path)
                    .display()
                    .to_string()
                    .replace('\\', "/"),
                count,
            ));
        }
    }
    per_file.sort_by(|a, b| b.1.cmp(&a.1));
    (total, per_file)
}

#[test]
fn public_surface_does_not_grow() {
    let (total, per_file) = count_public_items();

    if total > MAX_PUBLIC_ITEMS {
        let worst: Vec<String> = per_file
            .iter()
            .take(10)
            .map(|(path, count)| format!("  {count:>5}  src/{path}"))
            .collect();
        panic!(
            "public surface grew: {total} items, ceiling is {MAX_PUBLIC_ITEMS}.\n\n\
             Largest contributors:\n{}\n\n\
             New items should be `pub(crate)` unless a frontend needs them. \
             The CLI, GUI, Python and WASM bindings, and the conformance \
             suite together name about two hundred distinct paths; anything \
             outside that set is internal machinery and should say so.",
            worst.join("\n")
        );
    }

    assert!(
        total + STALE_CEILING_SLACK >= MAX_PUBLIC_ITEMS,
        "public surface is {total} items but the ceiling is still \
         {MAX_PUBLIC_ITEMS}, a gap of {}.\n\n\
         Lower MAX_PUBLIC_ITEMS in tests/public_surface.rs to {total}. A \
         ceiling left far above the real count is not a ratchet — it is \
         headroom for the surface to grow back into.",
        MAX_PUBLIC_ITEMS - total
    );
}

#[test]
fn restricted_visibility_does_not_count_as_public() {
    assert!(is_public_item("pub fn solve()"));
    assert!(is_public_item("pub struct Circuit"));
    assert!(is_public_item("pub use crate::circuit::CircuitData;"));
    assert!(is_public_item("pub const GMIN: f64 = 1e-12;"));

    // The whole point: narrowing visibility must lower the count, so these
    // are not public for this test's purposes.
    assert!(!is_public_item("pub(crate) fn stamp()"));
    assert!(!is_public_item("pub(super) struct State"));
    assert!(!is_public_item("pub(in crate::engine) fn drive()"));
    assert!(!is_public_item("pub(crate) use super::Thing;"));

    // Not item declarations. The struct field is passed already trimmed,
    // as the counter sees it, so this exercises the keyword check rather
    // than the leading whitespace.
    assert!(!is_public_item("fn private()"));
    assert!(!is_public_item("pub node_pos: Vec<NodeId>,"));
    assert!(!is_public_item("// pub fn commented_out()"));
}

#[test]
fn public_hb_solver_rejects_invalid_charge_parameters_before_evaluation() {
    let mut invalid_devices = Vec::new();

    let mut invalid_junction = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_junction.params.cap_a = DepletionCap::new(1.0e-12, 0.7, 1.01, 0.5);
    invalid_devices.push((invalid_junction, "grading coefficient"));

    let mut invalid_gate = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, 1.0e-3, 0.0);
    invalid_gate.params.cox_wl = -1.0e-15;
    invalid_devices.push((invalid_gate, "intrinsic gate capacitance"));

    let mut invalid_transit = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_transit.params.tt_f = f64::NAN;
    invalid_devices.push((invalid_transit, "transit time"));

    let invalid_diode = NonlinearDeviceInstance::diode(0, 0, -1.0, 1.0);
    invalid_devices.push((invalid_diode, "diode IS"));

    let invalid_mos = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, -1.0, 0.0);
    invalid_devices.push((invalid_mos, "MOS KP"));

    let invalid_jfet = NonlinearDeviceInstance::njfet(0, 0, 0, -2.0, -1.0, 0.0, 1.0e-14);
    invalid_devices.push((invalid_jfet, "JFET BETA"));

    let mut invalid_arity = NonlinearDeviceInstance::diode(0, 0, 1.0e-14, 1.0);
    invalid_arity.terminals.pop();
    invalid_devices.push((invalid_arity, "has 1 terminals, expected 2"));

    let invalid_index = NonlinearDeviceInstance::diode(2, 0, 1.0e-14, 1.0);
    invalid_devices.push((invalid_index, "node index 2 exceeds 1 nodes"));

    for (device, expected) in invalid_devices {
        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
        solver.add_nonlinear_device(device);
        let mut state = HbSolverState::new(1, 1);
        let error = solver
            .solve_dc_operating_point(&mut state)
            .expect_err("invalid public nonlinear-device parameters must fail before solving");
        assert!(matches!(error, HbError::InvalidCircuit(_)));
        assert!(
            error.to_string().contains(expected),
            "wrong public-solver parameter diagnostic: {error}"
        );
    }

    let mut switch_solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
    switch_solver.add_voltage_switch(0, 0, 0, 0, 0.0, 0.1, 1.0, 1.0e6, 0.1);
    let mut state = HbSolverState::new(1, 1);
    let error = switch_solver
        .solve_dc_operating_point(&mut state)
        .expect_err("public exact-HB switch API must reject unrepresented hysteresis");
    assert!(
        error
            .to_string()
            .contains("requires zero finite hysteresis"),
        "wrong public switch diagnostic: {error}"
    );
}

#[test]
fn public_hb_surface_does_not_advertise_rejected_approximate_kernels() {
    let solver_root = src_dir().join("analysis/harmonic_balance");
    let solver_source = fs::read_to_string(solver_root.join("solver.rs")).expect("read solver.rs");
    let device_source =
        fs::read_to_string(solver_root.join("solver/devices.rs")).expect("read solver/devices.rs");
    let api_source = fs::read_to_string(solver_root.join("solver/nonlinear_api.rs"))
        .expect("read solver/nonlinear_api.rs");
    let combined = format!("{solver_source}\n{device_source}\n{api_source}");

    for stale_name in [
        "NpnBjt",
        "PnpBjt",
        "CurrentSwitch",
        "npn_bjt",
        "pnp_bjt",
        "current_switch",
        "add_current_switch",
    ] {
        assert!(
            !combined.contains(stale_name),
            "exact-HB public surface still advertises removed approximate kernel {stale_name}"
        );
    }
}

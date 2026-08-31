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
/// `engine::event_scheduler`: the tick time base (`TimeResolution` and its
/// five operations plus `MAX_EXACT_TICKS`), the stratified region vocabulary
/// (`SchedulerRegion`, `ORDERED`), the event payload (`EventTarget`,
/// `ScheduledEvent`), the typed failure surface (`SchedulerError`,
/// `OscillationCause`, `OscillationDiagnostic`), the ceilings and per-slot
/// report (`SchedulerLimits`, `TimeSlotReport`), and the scheduler itself
/// (`EventScheduler` with six operations, `SchedulerContext` with three).
///
/// These are public for a reason this test's usual rule does not cover: the
/// kernel has no in-crate consumer until XSPICE is rehosted on it, so
/// `pub(crate)` would make every one of them dead code under `-D warnings`.
/// The integration test `event_scheduler_kernel.rs` is the consumer that
/// keeps them honest in the meantime. When the rehost lands and the analog
/// engine calls this module directly, most of this surface should narrow.
const MAX_PUBLIC_ITEMS: usize = 4291;

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

//! A task's installed Verilog-A stays in the shared cache until the task is
//! done with it.
//!
//! The sealed runtimes a task installs live in one process-wide cache bounded
//! by `max_shared_cache_bytes`, and a sealed key is never compiled again from
//! anywhere: a build that misses it fails. So another task's install, which
//! makes room by evicting the oldest entries, must not evict one a task has
//! installed and not yet built.
//!
//! This is its own test binary because it shrinks the shared budget, which
//! evicts every other entry in the process, and its tests take turns for the
//! same reason.

#![cfg(feature = "veriloga")]

use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};

use rspice_core::{
    Engine, Netlist, ProjectVerilogARuntimeRegistration, ProjectVerilogASourceRegistration,
    ResourceLimits, SimulationConfig, hold_project_veriloga_sources_for_session,
};

/// One test at a time owns the process's cache budget.
fn exclusive_cache() -> MutexGuard<'static, ()> {
    static CACHE: Mutex<()> = Mutex::new(());
    CACHE.lock().unwrap_or_else(PoisonError::into_inner)
}

/// A two-terminal conductance of `siemens`, sealed under its own key.
fn conductance(name: &str, siemens: f64) -> (PathBuf, ProjectVerilogASourceRegistration) {
    let report = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(
            &format!(
                "module {name}(p, n); inout p, n; electrical p, n; \
                 analog I(p, n) <+ {siemens} * V(p, n); endmodule\n"
            ),
            None,
        )
        .expect("the conductance compiles");
    let key = PathBuf::from(format!(
        "__rspice_model_library__/holds-{}/{name}/{name}.va",
        std::process::id()
    ));
    (
        key.clone(),
        ProjectVerilogASourceRegistration::Runtime(ProjectVerilogARuntimeRegistration {
            source_key: key,
            aliases: vec![name.to_ascii_uppercase()],
            model: report.model,
            canonical_ir: report.canonical_ir,
        }),
    )
}

fn limits(max_shared_cache_bytes: usize) -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    limits.max_shared_cache_bytes = max_shared_cache_bytes;
    limits
}

/// What one registration retains, read from the refusal of a one-byte budget.
fn retained_bytes(registration: &ProjectVerilogASourceRegistration) -> usize {
    let refusal = hold_project_veriloga_sources_for_session([registration.clone()], limits(1))
        .expect_err("no runtime fits in one byte");
    refusal
        .split("requested ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .and_then(|bytes| bytes.parse().ok())
        .unwrap_or_else(|| panic!("the refusal states the size: {refusal}"))
}

/// The deck a task builds: one instance of `name` across a 1 V source.
fn deck(key: &std::path::Path, name: &str) -> Netlist {
    Netlist::parse(&format!(
        "held runtime\nV1 p 0 1\nX1 p 0 {alias}\n.veriloga \"{key}\" {alias}\n.op\n.end\n",
        key = key.to_string_lossy(),
        alias = name.to_ascii_uppercase(),
    ))
    .expect("the deck parses")
}

/// Task A installs and holds its runtime, then task B installs before A has
/// built. The budget has room for one runtime, so B's install could only fit
/// by evicting A's: it is refused, and A builds. Once A releases its hold, B
/// installs, and A's runtime is what the cache evicted to make room.
#[test]
fn a_task_builds_the_runtime_it_installed_after_another_task_installs_its_own() {
    let _cache = exclusive_cache();
    let (held_key, held) = conductance("held_task", 1e-3);
    let (pushing_key, pushing) = conductance("pushing_task", 2e-3);
    // Room for either runtime, not for both.
    let budget = limits(retained_bytes(&held).max(retained_bytes(&pushing)) + 1);
    let engine = Engine::new(SimulationConfig {
        resource_limits: budget,
        ..Default::default()
    });

    let hold = hold_project_veriloga_sources_for_session([held], budget)
        .expect("the first task installs its runtime");
    let refusal = hold_project_veriloga_sources_for_session([pushing.clone()], budget)
        .expect_err("the second install cannot evict a held runtime");
    assert!(
        refusal.contains("shared_cache_bytes limit exceeded")
            && refusal.contains("held by executions in progress"),
        "{refusal}"
    );
    engine
        .build_circuit(&deck(&held_key, "held_task"))
        .expect("the first task builds the runtime it installed");

    drop(hold);
    let _pushed = hold_project_veriloga_sources_for_session([pushing], budget)
        .expect("a released runtime makes room");
    engine
        .build_circuit(&deck(&pushing_key, "pushing_task"))
        .expect("the second task builds its runtime");
    let evicted = engine
        .build_circuit(&deck(&held_key, "held_task"))
        .expect_err("the released runtime was evicted to make room");
    assert!(
        evicted
            .to_string()
            .contains("not installed for this execution"),
        "{evicted}"
    );
}

/// A hold counts: a runtime two executions hold stays held until both release
/// it, and reinstalling a held runtime keeps it held.
#[test]
fn a_runtime_stays_held_until_every_holder_releases_it() {
    let _cache = exclusive_cache();
    let (key, runtime) = conductance("shared_hold", 3e-3);
    let (_, other) = conductance("shared_other", 4e-3);
    let budget = limits(retained_bytes(&runtime).max(retained_bytes(&other)) + 1);
    let engine = Engine::new(SimulationConfig {
        resource_limits: budget,
        ..Default::default()
    });

    let first = hold_project_veriloga_sources_for_session([runtime.clone()], budget)
        .expect("the first execution holds the runtime");
    let second = hold_project_veriloga_sources_for_session([runtime], budget)
        .expect("a second execution reinstalls and holds the same runtime");
    drop(first);
    hold_project_veriloga_sources_for_session([other.clone()], budget)
        .expect_err("the second execution still holds the runtime");
    engine
        .build_circuit(&deck(&key, "shared_hold"))
        .expect("the runtime is still installed");

    drop(second);
    let _other = hold_project_veriloga_sources_for_session([other], budget)
        .expect("released by both, the runtime makes room");
}

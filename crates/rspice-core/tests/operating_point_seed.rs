//! The linear solve that seeds an operating point's Newton iteration.
//!
//! It stamps only the linear devices, so a behavioral voltage source, whose
//! branch equation is its expression, leaves its branch row empty there. The
//! seed then starts from zero without asking the sparse solver to factor a
//! system that has no solution, which would otherwise report that the solver
//! could not factor it.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;
use std::cell::RefCell;
use std::sync::Once;

static LOG_INIT: Once = Once::new();

thread_local! {
    static CAPTURED: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
}

struct CapturingLogger;

impl log::Log for CapturingLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            CAPTURED.with(|captured| {
                if let Some(lines) = captured.borrow_mut().as_mut() {
                    lines.push(record.args().to_string());
                }
            });
        }
    }

    fn flush(&self) {}
}

static LOGGER: CapturingLogger = CapturingLogger;

/// Run `body` with the warnings it logs on this thread collected.
fn warnings_of<T>(body: impl FnOnce() -> T) -> (T, Vec<String>) {
    LOG_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("the capture logger installs");
        log::set_max_level(log::LevelFilter::Warn);
    });
    CAPTURED.with(|captured| *captured.borrow_mut() = Some(Vec::new()));
    let value = body();
    let lines = CAPTURED.with(|captured| captured.borrow_mut().take().unwrap_or_default());
    (value, lines)
}

// LTspice 26.1.1 and QSPICE 2026 batch: V(y) = 2 V exactly.
#[test]
fn a_behavioral_voltage_source_seeds_newton_without_a_failed_factorization() {
    let netlist = Netlist::parse(
        "behavioral source seed\n\
         V1 a 0 1\n\
         R0 a 0 1k\n\
         B1 y 0 V=2*V(a)\n\
         R1 y 0 1k\n\
         .op\n\
         .end\n",
    )
    .expect("deck parses");
    let (result, warnings) = warnings_of(|| Engine::default().run_dc_op(&netlist));
    let result = result.expect("operating point solves");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("y"))
        .expect("V(y) is solved");
    let value = result.node_voltages[index];
    assert!((value - 2.0).abs() <= 1e-12, "V(y) = {value}");
    assert!(
        !warnings
            .iter()
            .any(|line| line.contains("could not factor")),
        "{warnings:?}"
    );
}

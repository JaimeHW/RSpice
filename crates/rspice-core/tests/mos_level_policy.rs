//! Build-time policy for MOS model levels without a native implementation.
//!
//! BSIM-class cards without a native port (LEVEL=53, ...) must be rejected
//! with a remediation message instead of silently running the simplified
//! short-channel approximation, which honors only a handful of parameters
//! and produces plausible-looking but wrong currents. The
//! `.options allow_simplified_mos` opt-in downgrades the rejection to a
//! warning; LEVEL=3 routes natively as Berkeley MOS3; LEVEL=8/49 route to
//! the native BSIM3v3.3 port and LEVEL=14/54 to the native BSIM4 v4.8 port.

use rspice_core::circuit::DeviceOpReport;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex, Once,
};

static LOG_INIT: Once = Once::new();
static LOG_CAPTURE_ACTIVE: AtomicBool = AtomicBool::new(false);
static LOG_CAPTURE_LOCK: Mutex<()> = Mutex::new(());
static CAPTURED_WARNINGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct CapturingLogger;

impl log::Log for CapturingLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            CAPTURED_WARNINGS
                .lock()
                .expect("warning log capture lock")
                .push(record.args().to_string());
        }
    }

    fn flush(&self) {}
}

static LOGGER: CapturingLogger = CapturingLogger;

fn init_log_capture() {
    LOG_INIT.call_once(|| {
        log::set_logger(&LOGGER).expect("mos_level_policy warning capture logger must install");
        log::set_max_level(log::LevelFilter::Warn);
        LOG_CAPTURE_ACTIVE.store(true, Ordering::SeqCst);
    });
    assert!(
        LOG_CAPTURE_ACTIVE.load(Ordering::SeqCst),
        "mos_level_policy warning capture logger is not active"
    );
}

fn op_deck(model_line: &str, options_line: &str) -> String {
    format!(
        "* mos level policy\n\
         vdd d 0 dc 1.8\n\
         vg g 0 dc 1.0\n\
         m1 d g 0 0 nmod w=1u l=0.1u\n\
         {model_line}\n\
         {options_line}\n\
         .op\n\
         .end\n"
    )
}

fn run_report(deck: &str) -> Result<DeviceOpReport, String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_dc_op_with_report(&netlist)
        .map(|(_, report)| report)
        .map_err(|err| err.to_string())
}

fn run(deck: &str) -> Result<(), String> {
    run_report(deck).map(|_| ())
}

#[test]
fn bsim_level_without_native_model_is_rejected() {
    // LEVEL=53 (BSIM3v3.2-class) has no native port; the rejection must
    // name the family, list the natively supported BSIM levels, and offer
    // the opt-in.
    let deck = op_deck(".model nmod NMOS (LEVEL=53 VTH0=0.5)", "");
    let message = run(&deck).expect_err("LEVEL=53 must not silently run the approximation");
    assert!(
        message.contains("BSIM3v3"),
        "error names the model family: {message}"
    );
    assert!(
        message.contains("allow_simplified_mos"),
        "error names the opt-in: {message}"
    );
    assert!(
        message.contains("14/54 (BSIM4"),
        "error lists the native BSIM4 levels: {message}"
    );
}

#[test]
fn bsim3_levels_run_natively() {
    // LEVEL=49 and LEVEL=8 route to the native BSIM3v3.3 port — no
    // allow_simplified_mos opt-in, no rejection.
    for level in [49, 8] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level} VTH0=0.5 TOX=4.1n NCH=2.35e17)"),
            "",
        );
        run(&deck).unwrap_or_else(|err| panic!("LEVEL={level} must run natively: {err}"));
    }
}

#[test]
fn bsim4_levels_run_natively() {
    // LEVEL=54 and LEVEL=14 route to the native BSIM4 v4.8 port — no
    // allow_simplified_mos opt-in, no rejection.
    for level in [54, 14] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level} VTH0=0.5 TOXE=1.4n NDEP=3e18)"),
            "",
        );
        run(&deck).unwrap_or_else(|err| panic!("LEVEL={level} must run natively: {err}"));
    }
}

#[test]
fn simplified_mos_opt_in_accepts_the_deck() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=53 VTH0=0.5)",
        ".options allow_simplified_mos=1",
    );
    run(&deck).expect("explicit opt-in runs the simplified approximation");
}

#[test]
fn simplified_mos_opt_in_zero_still_rejects() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=53 VTH0=0.5)",
        ".options allow_simplified_mos=0",
    );
    run(&deck).expect_err("allow_simplified_mos=0 keeps the rejection");
}

#[test]
fn level3_runs_without_fallback_warning() {
    let _guard = LOG_CAPTURE_LOCK.lock().expect("serialize log capture");
    init_log_capture();
    CAPTURED_WARNINGS
        .lock()
        .expect("warning log capture lock")
        .clear();

    let deck = "* mos level policy\n\
                vdd d 0 dc 1.8\n\
                vg g 0 dc 1.0\n\
                m1 d g 0 0 n3native w=1u l=0.1u\n\
                .model n3native NMOS (LEVEL=3 VTO=0.6 KP=100u)\n\
                .op\n\
                .end\n";
    let report = run_report(deck).expect("LEVEL=3 must run as native Berkeley MOS3");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 op entry");
    assert!(!entry.device_kind.is_empty(), "m1 op entry has a device kind");
    let warnings = CAPTURED_WARNINGS
        .lock()
        .expect("warning log capture lock")
        .clone();
    assert!(
        !warnings.iter().any(|message| {
            message.contains("no native MOS3")
                || message.contains("MOS3-specific parameters")
        }),
        "LEVEL=3 must not emit the old simplified fallback warning; warnings={warnings:?}"
    );
}

#[test]
fn native_levels_unaffected() {
    for model_line in [
        ".model nmod NMOS (LEVEL=1 VTO=0.6 KP=100u)",
        ".model nmod NMOS (LEVEL=2 VTO=0.6 KP=100u)",
        ".model nmod NMOS (LEVEL=6 VTO=0.6 KC=100u)",
    ] {
        let deck = op_deck(model_line, "");
        run(&deck).unwrap_or_else(|err| panic!("{model_line} must build: {err}"));
    }
}

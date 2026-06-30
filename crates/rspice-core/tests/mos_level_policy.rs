//! Build-time policy for MOS model levels without a native implementation.
//!
//! BSIM-class cards without a native port (LEVEL=53, ...) must be rejected
//! with a remediation message instead of silently running the simplified
//! short-channel approximation, which honors only a handful of parameters
//! and produces plausible-looking but wrong currents. Legacy
//! `.options allow_simplified_mos` input must not downgrade that rejection;
//! LEVEL=3 routes natively as Berkeley MOS3; LEVEL=9 routes either
//! to ngspice MOS9 or Xyce BSIM3 by dialect/parameter surface; LEVEL=8/49
//! route to the native BSIM3v3.3 port, LEVEL=14/54 to the native BSIM4 v4.8
//! port, LEVEL=260 to EKV 2.6, LEVEL=301 to the native EKV3 NMOS150 slice,
//! and CMC/HiSIM levels are named fail-closed advanced families until
//! Verilog-A-to-Rust codegen lands.

use rspice_core::circuit::DeviceOpReport;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::sync::{
    Mutex, Once,
    atomic::{AtomicBool, Ordering},
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
fn compact_three_terminal_mos_rejects_non_vdmos_model() {
    let deck = "* compact mos syntax policy\n\
                vdd d 0 dc 1.8\n\
                vg g 0 dc 1.0\n\
                m1 d g 0 nmod w=1u l=0.1u\n\
                .model nmod NMOS (LEVEL=1 VTO=0.6 KP=100u)\n\
                .op\n\
                .end\n";
    let message = run(deck).expect_err("ordinary MOS must keep an explicit bulk node");
    assert!(
        message.contains("bulk") || message.contains("four-node") || message.contains("VDMOS"),
        "error should explain compact MOS syntax is VDMOS-only, got: {message}"
    );
}

#[test]
fn compact_known_advanced_mos_level_fails_closed_before_bulk_syntax_policy() {
    let deck = "* compact MVS MOS level policy\n\
                vdd d 0 dc 1.8\n\
                vg g 0 dc 1.0\n\
                m1 d g 0 nmod w=1u l=0.1u\n\
                .model nmod NMOS (LEVEL=2000)\n\
                .op\n\
                .end\n";
    let message = run(deck).expect_err("MVS LEVEL=2000 must fail closed as unsupported");
    assert!(
        message.contains("LEVEL=2000") && message.contains("MVS"),
        "error should identify the unsupported MVS level, got: {message}"
    );
    assert!(
        message.contains("Verilog-A-to-Rust") && message.contains("codegen"),
        "error should describe the exact generated-model implementation path, got: {message}"
    );
    assert!(
        !message.contains("bulk") && !message.contains("VDMOS"),
        "advanced unsupported level policy must not be masked by compact MOS syntax wording: {message}"
    );
}

#[test]
fn fractional_mos_level_is_rejected_instead_of_truncated() {
    for level in [14.9, 18.5] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level} VTH0=0.5 TOXE=1.4n NDEP=3e18)"),
            "",
        );
        let message = run(&deck).expect_err("fractional MOS LEVEL must be rejected");

        assert!(
            message.contains(&format!("LEVEL={level}")) && message.contains("integer"),
            "error should explain the invalid fractional level, got: {message}"
        );
    }
}

#[test]
fn unresolved_mos_level_selector_fails_closed() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL={native_level} VTH0=0.5 TOXE=1.4n NDEP=3e18)",
        "",
    );
    let message = run(&deck).expect_err("unresolved MOS LEVEL selector must not fall back to MOS1");

    assert!(
        message.contains("MOSFET") && message.contains("LEVEL"),
        "error should identify the MOS LEVEL selector, got: {message}"
    );
    assert!(
        message.contains("unresolved") && message.contains("finite integer"),
        "error should explain unresolved selectors must be finite integers, got: {message}"
    );
}

#[test]
fn non_numeric_mos_level_selector_fails_closed() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=\"54\" VTH0=0.5 TOXE=1.4n NDEP=3e18)",
        "",
    );
    let message =
        run(&deck).expect_err("non-numeric MOS LEVEL selector must not fall back to MOS1");

    assert!(
        message.contains("MOSFET") && message.contains("LEVEL"),
        "error should identify the MOS LEVEL selector, got: {message}"
    );
    assert!(
        message.contains("non-numeric") && message.contains("finite integer"),
        "error should explain string selectors must be finite integers, got: {message}"
    );
}

#[test]
fn bsim_level_without_native_model_is_rejected() {
    // LEVEL=53 (BSIM3v3.2-class) has no native port; the rejection must
    // name the family and list the natively supported BSIM levels.
    let deck = op_deck(".model nmod NMOS (LEVEL=53 VTH0=0.5)", "");
    let message = run(&deck).expect_err("LEVEL=53 must not silently run the approximation");
    assert!(
        message.contains("BSIM3v3"),
        "error names the model family: {message}"
    );
    assert!(
        !message.contains("allow_simplified_mos"),
        "error must not advertise a simplified fallback: {message}"
    );
    assert!(
        message.contains("14/54 (BSIM4"),
        "error lists the native BSIM4 levels: {message}"
    );
}

#[test]
fn bsim3_levels_run_natively() {
    // LEVEL=49/8 are ngspice-compatible BSIM3 aliases; LEVEL=9 is Xyce's
    // BSIM3 front when the card has a BSIM3 parameter surface. All route to
    // the native port: no allow_simplified_mos opt-in, no rejection.
    for level in [49, 8, 9] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level} VTH0=0.5 TOX=4.1n NCH=2.35e17)"),
            "",
        );
        run(&deck).unwrap_or_else(|err| panic!("LEVEL={level} must run natively: {err}"));
    }
}

#[test]
fn level9_bsim3_only_selectors_route_to_bsim3_not_mos9() {
    for selector in ["ACM=1", "CALCACM=1", "BINUNIT=1", "PARAMCHK=0"] {
        let deck = op_deck(&format!(".model nmod NMOS (LEVEL=9 {selector})"), "");
        let report = run_report(&deck).unwrap_or_else(|err| {
            panic!("LEVEL=9 with BSIM3-only selector {selector} should run as BSIM3: {err}")
        });
        let entry = report
            .entries
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
            .expect("m1 OP entry");
        assert_eq!(
            entry.device_kind, "BSIM3",
            "LEVEL=9 {selector} must not silently fall through to MOS9"
        );
    }
}

#[test]
fn level9_deferred_bsim3_signature_fails_closed_instead_of_mos9() {
    let deck = op_deck(".model nmod NMOS (LEVEL=9 VERSION={bsim_version})", "");
    let message =
        run(&deck).expect_err("deferred BSIM3-shaped LEVEL=9 card must not fall through to MOS9");
    assert!(
        message.contains("native BSIM3")
            && message.contains("VERSION=bsim_version")
            && message.contains("finite numeric literals"),
        "deferred VERSION should fail closed through the BSIM3 selector path: {message}"
    );
}

#[test]
fn level9_string_bsim3_version_routes_to_bsim3_not_mos9() {
    let deck = op_deck(".model nmod NMOS (LEVEL=9 VERSION=\"3.2.2\")", "");
    let report =
        run_report(&deck).expect("string BSIM3 VERSION metadata should route to native BSIM3");
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");
    assert_eq!(
        entry.device_kind, "BSIM3",
        "LEVEL=9 VERSION=\"3.2.2\" must not silently fall through to MOS9"
    );
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
fn hisim_levels_fail_closed_as_named_advanced_models() {
    for (level, family) in [
        (61, "HiSIM2"),
        (68, "HiSIM2"),
        (62, "HiSIM_HV"),
        (73, "HiSIM_HV"),
    ] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level})"),
            ".options allow_simplified_mos=1",
        );
        let message = run(&deck).expect_err("HiSIM-family MOS level must fail closed");
        assert!(
            message.contains(&format!("LEVEL={level}")) && message.contains(family),
            "LEVEL={level} error should identify the unsupported {family} family: {message}"
        );
        assert!(
            message.contains("simplified MOS approximation"),
            "LEVEL={level} must refuse simplified fallback for {family}: {message}"
        );
        assert!(
            !message.contains("allow_simplified_mos"),
            "LEVEL={level} must not suggest or honor simplified MOS fallback: {message}"
        );
    }
}

#[test]
fn cmc_mos_levels_fail_closed_as_veriloga_codegen_targets() {
    for (level, family) in [
        (58, "B4SOI"),
        (70, "B4SOI"),
        (77, "BSIM6"),
        (102, "PSP102"),
        (103, "PSP103"),
        (107, "BSIM-CMG"),
        (109, "BSIM-CMG"),
        (111, "BSIM-CMG"),
        (1031, "PSP103"),
        (2000, "MVS"),
        (2001, "MVS"),
        (70450, "B4SOI"),
        (70470, "B4SOI"),
        (10240, "L-UTSOI"),
    ] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level})"),
            ".options allow_simplified_mos=1",
        );
        let message = run(&deck).expect_err("CMC MOS level must fail closed");

        assert!(
            message.contains(&format!("LEVEL={level}")) && message.contains(family),
            "LEVEL={level} error should identify {family}: {message}"
        );
        assert!(
            message.contains("Verilog-A-to-Rust") && message.contains("codegen"),
            "LEVEL={level} error should describe the codegen path: {message}"
        );
        assert!(
            !message.contains("allow_simplified_mos"),
            "LEVEL={level} must not suggest or use simplified MOS fallback: {message}"
        );
    }
}

#[test]
fn ekv26_accepts_native_junction_storage_params() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=260 AVTO=0 XD_MJ=0.5 XD_MJSW=0.4 XD_MJSWG=0.3 \
         XD_PB=0.8 XD_PBSW=0.6 XD_PBSWG=0.55 XD_CJ=2e-3 XD_CJSW=3e-10 \
         XD_CJSWG=4e-10 TP_CJ=1e-4 TP_CJSW=2e-4 TP_CJSWG=3e-4 \
         TP_PB=1e-4 TP_PBSW=2e-4 TP_PBSWG=3e-4)",
        "",
    );
    let report = run_report(&deck)
        .unwrap_or_else(|err| panic!("EKV26 junction storage params must run natively: {err}"));
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");
    assert_eq!(
        entry.device_kind, "EKV26",
        "LEVEL=260 with junction storage params must route to native EKV26"
    );
}

#[test]
fn ekv26_level260_runs_natively_and_ignores_simplified_escape_hatch() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=260 AVTO=0 VTO=570.6m TCV=1.194m COX=4.379m XJ=22.53n)",
        ".options allow_simplified_mos=1",
    );
    let report =
        run_report(&deck).unwrap_or_else(|err| panic!("LEVEL=260 must run natively: {err}"));
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case("m1"))
        .expect("m1 OP entry");
    assert_eq!(
        entry.device_kind, "EKV26",
        "LEVEL=260 must route to native EKV26, not simplified MOS"
    );
}

#[test]
fn unsupported_mos_levels_reject_even_with_simplified_mos_opt_in() {
    for (level, family) in [(53, "BSIM3v3"), (999, "LEVEL=999")] {
        let deck = op_deck(
            &format!(".model nmod NMOS (LEVEL={level} VTH0=0.5)"),
            ".options allow_simplified_mos=1",
        );
        let message = match run(&deck) {
            Ok(()) => panic!("LEVEL={level} must reject despite opt-in"),
            Err(message) => message,
        };

        assert!(
            message.contains(family) || message.contains(&format!("LEVEL={level}")),
            "unsupported LEVEL={level} error should identify the family/level: {message}"
        );
        assert!(
            message.contains("no native implementation")
                || message.contains("not implemented natively"),
            "unsupported LEVEL={level} error should require native support: {message}"
        );
        assert!(
            !message.contains("allow_simplified_mos"),
            "unsupported LEVEL={level} must not suggest or honor simplified fallback: {message}"
        );
    }
}

#[test]
fn unsupported_ekv3_cards_ignore_simplified_mos_escape_hatch() {
    let (level, family) = (301, "EKV3");
    let deck = op_deck(
        &format!(".model nmod NMOS (LEVEL={level} VTH0=0.5 TOXE=1.4n NDEP=3e18)"),
        ".options allow_simplified_mos=1",
    );
    let message = match run(&deck) {
        Ok(()) => panic!("{family} LEVEL={level} unsupported card must remain fail-closed"),
        Err(message) => message,
    };

    assert!(
        message.contains(&format!("LEVEL={level}")) || message.contains(family),
        "{family} error should identify the known advanced level: {message}"
    );
    assert!(
        message.contains("unsupported EKV3")
            || message.contains("does not support model parameter")
            || message.contains("remain fail-closed"),
        "{family} error should describe unsupported native slice coverage: {message}"
    );
    assert!(
        !message.contains("allow_simplified_mos"),
        "{family} must not suggest or use the simplified MOS escape hatch: {message}"
    );
}

#[test]
fn xyce_ekv3_level301_incomplete_card_fails_closed_outside_native_slice() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=301 VTO=0.4 COX=8.58m KP=390u)",
        ".options allow_simplified_mos=1",
    );
    let message = run(&deck).expect_err("incomplete Xyce EKV3 LEVEL=301 card must reject");

    assert!(
        message.contains("EKV3") && message.contains("NMOS150 slice"),
        "EKV3 LEVEL=301 error should identify the native slice boundary: {message}"
    );
    assert!(
        message.contains("requires model parameter") || message.contains("remain fail-closed"),
        "EKV3 LEVEL=301 error should describe unsupported native slice coverage: {message}"
    );
    assert!(
        !message.contains("allow_simplified_mos"),
        "EKV3 LEVEL=301 must not suggest or use the simplified MOS escape hatch: {message}"
    );
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
    assert!(
        !entry.device_kind.is_empty(),
        "m1 op entry has a device kind"
    );
    let warnings = CAPTURED_WARNINGS
        .lock()
        .expect("warning log capture lock")
        .clone();
    assert!(
        !warnings.iter().any(|message| {
            message.contains("no native MOS3") || message.contains("MOS3-specific parameters")
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

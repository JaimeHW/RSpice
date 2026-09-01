//! Exact-MNA contracts for nonlinear harmonic balance.
//!
//! A nonlinear device must not change ideal voltage sources or inductors into
//! approximate nodal elements.  These regressions deliberately select the HB
//! Newton path with an otherwise unloaded level-1 MOSFET and then pin the
//! voltage law, source-current state, inconsistent-source handling, and the
//! resource cost of the additional branch spectra.

use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig, SpiceDialect};
use rspice_core::netlist::Netlist;
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits, SimulationError};

const F0: f64 = 1.0e6;
const MOS_MODEL: &str = ".model NSELECT NMOS LEVEL=1 VTO=0.7 KP=20u TOX=20n CGSO=0 CGDO=0 CGBO=0";

fn run(deck: &str, use_krylov: bool, harmonics: usize) -> HbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("HB exact-MNA fixture parses");
    let mut config = HbConfig::new(F0).with_harmonics(harmonics);
    config.use_krylov = use_krylov;
    Engine::new(SimulationConfig::default())
        .run_hb(&netlist, config)
        .expect("nonlinear HB exact-MNA fixture converges")
}

fn coefficient(result: &HbAnalysisResult, node: &str, harmonic: usize) -> Complex64 {
    result
        .result
        .spectral_voltages
        .iter()
        .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing HB node '{node}'"))
        .coefficients[harmonic]
}

fn limits_with(update: impl FnOnce(&mut ResourceLimits)) -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    update(&mut limits);
    limits
}

fn assert_ideal_mos_gate_clamp(use_krylov: bool) {
    // This is intentionally hostile to a fixed 1 uOhm Norton replacement:
    // its resistance equals RLOAD, so that approximation produces 0.5 V.
    // Exact MNA must retain VPUMP's authored 1 V constraint independently of
    // the one-megaampere branch current it must supply.
    let deck = format!(
        "* ideal source driving a nonlinear MOS-gate circuit\n\
         VPUMP pump 0 DC 1\n\
         RLOAD pump 0 1u\n\
         MSELECT 0 pump 0 0 NSELECT L=1u W=1u\n\
         {MOS_MODEL}\n\
         .end\n"
    );

    let result = run(&deck, use_krylov, 1);
    let pump = coefficient(&result, "pump", 0);
    assert!(result.converged, "HB must converge (Krylov={use_krylov})");
    assert!(
        (pump - Complex64::new(1.0, 0.0)).norm() <= 1.0e-9,
        "ideal-source KVL must hold in nonlinear HB (Krylov={use_krylov}); \
         V(pump)={pump}, expected 1 V"
    );
}

#[test]
fn dense_newton_preserves_an_ideal_mos_gate_clamp() {
    assert_ideal_mos_gate_clamp(false);
}

#[test]
fn forced_krylov_newton_preserves_an_ideal_mos_gate_clamp() {
    assert_ideal_mos_gate_clamp(true);
}

#[test]
fn nonlinear_hb_retains_voltage_source_current_and_satisfies_kcl() {
    let deck = format!(
        "* nonlinear HB source-current state\n\
         VDRIVE out 0 SIN(0 1 {F0})\n\
         RLOAD out 0 1k\n\
         RGATE gate 0 1\n\
         MSELECT 0 gate 0 0 NSELECT L=1u W=1u\n\
         {MOS_MODEL}\n\
         .end\n"
    );
    let result = run(&deck, false, 2);
    let source = result
        .result
        .mna_branch_currents
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("VDRIVE"))
        .expect("nonlinear HB retains VDRIVE's actual MNA branch spectrum");

    assert_eq!(source.coefficients.len(), 3);
    let voltage = coefficient(&result, "out", 1);
    let kcl = source.coefficients[1] + voltage / 1.0e3;
    assert!(
        (voltage - Complex64::new(0.0, -1.0)).norm() <= 1.0e-9,
        "the cosine-reference phasor of an authored unit sine must be -j V: {voltage}"
    );
    assert!(
        kcl.norm() <= 1.0e-12,
        "source current must use positive-to-negative orientation and close KCL; residual={kcl}"
    );
}

#[test]
fn nonlinear_hb_rejects_conflicting_ideal_voltage_sources() {
    let deck = format!(
        "* inconsistent ideal source constraints\n\
         VONE out 0 DC 1\n\
         VTWO out 0 DC 2\n\
         RLOAD out 0 1k\n\
         MSELECT 0 out 0 0 NSELECT L=1u W=1u\n\
         {MOS_MODEL}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("conflicting-source fixture parses");
    let error = Engine::new(SimulationConfig::default())
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(1))
        .expect_err("inconsistent ideal voltage constraints must fail closed");
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains("singular")
            || message.contains("conflict")
            || message.contains("inconsistent"),
        "error must identify an inconsistent or singular ideal-source system: {error}"
    );
}

#[test]
fn nonlinear_hb_result_budget_counts_retained_mna_branch_spectra() {
    let deck = format!(
        "* nonlinear HB retained-result accounting\n\
         VDRIVE out 0 DC 1\n\
         RLOAD out 0 1k\n\
         MSELECT 0 out 0 0 NSELECT L=1u W=1u\n\
         {MOS_MODEL}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("result-budget fixture parses");
    let config = SimulationConfig {
        // Two one-sided coefficients times one node and one MNA current,
        // with real and imaginary components counted independently.
        resource_limits: limits_with(|limits| limits.max_result_values = 7),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).run_hb(&netlist, HbConfig::new(F0).with_harmonics(1)),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::ResultValues,
            requested: 8,
            limit: 7,
        }))
    ));
}

#[test]
fn nonlinear_hb_admits_level_one_breakdown_and_closes_source_kcl() {
    let deck = "\
* exact Level-1 reverse-breakdown branch; IBV is below IS*BV/VT so BV is the knee
VDRIVE out 0 DC -5.1
D1 out 0 DMOD
.model DMOD D (IS=1u N=1.48 BV=5 IBV=1u)
.end
";
    let netlist = Netlist::parse(deck).expect("breakdown HB fixture parses");
    let config = SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..SimulationConfig::default()
    };
    let result = Engine::new(config)
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(1))
        .expect("canonical Level-1 breakdown is supported by exact HB");
    assert!(result.converged);
    assert!((coefficient(&result, "out", 0).re + 5.1).abs() <= 1.0e-12);

    let source = result
        .result
        .mna_branch_currents
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("VDRIVE"))
        .expect("exact HB retains the ideal-source branch current");

    // Independent Xyce Level-1 oracle: omitted NBV defaults to 1, and the low
    // IBV above leaves the matched knee at the authored 5 V.
    let xyce_vt: f64 = (1.3806226e-23 / 1.6021918e-19) * 300.15;
    let expected_source_current = 1.0e-6 * (0.1 / xyce_vt).exp();
    let actual_source_current = source.coefficients[0].re;
    assert!(
        (actual_source_current - expected_source_current).abs()
            <= 1.0e-7 * expected_source_current.abs().max(1.0e-18),
        "source/diode KCL must use the canonical breakdown current: got {actual_source_current:.9e}, expected {expected_source_current:.9e}"
    );
    assert!(actual_source_current > 1.0e-5);
    assert!(source.coefficients[1].norm() <= 1.0e-12);
}

#[test]
fn nonlinear_hb_matrix_budget_counts_branch_spectrum_unknowns() {
    let deck = format!(
        "* nonlinear HB Newton-state accounting\n\
         VDRIVE out 0 DC 1\n\
         RLOAD out 0 1k\n\
         MSELECT 0 out 0 0 NSELECT L=1u W=1u\n\
         {MOS_MODEL}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("matrix-budget fixture parses");
    let config = SimulationConfig {
        // The H=1 realified Newton state has three scalar components for each
        // of the node-voltage and branch-current spectra.
        resource_limits: limits_with(|limits| limits.max_matrix_unknowns = 5),
        ..SimulationConfig::default()
    };

    assert!(matches!(
        Engine::new(config).run_hb(&netlist, HbConfig::new(F0).with_harmonics(1)),
        Err(SimulationError::ResourceLimit(ResourceLimitError {
            resource: ResourceKind::MatrixUnknowns,
            requested: 6,
            limit: 5,
        }))
    ));
}

#[test]
fn nonlinear_hb_fails_closed_before_solving_unrepresented_periodic_families() {
    let cases = [
        (
            "\
* solution-dependent capacitance requires periodic charge linearization
iin 0 out dc 0
vctrl ctrl 0 dc 0.5
c1 out 0 C={1p*(1+V(ctrl))}
r1 out 0 1k
.end
",
            "solution-dependent capacitor",
        ),
    ];

    for (deck, expected_family) in cases {
        let netlist = Netlist::parse(deck).expect("unsupported HB deck still parses");
        let error = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(F0).with_harmonics(1))
            .expect_err("an unrepresented periodic family must fail before exact HB solving");
        let message = error.to_string();
        assert!(
            message.contains("exact HB MNA is unavailable"),
            "failure must identify the exact-MNA preflight: {message}"
        );
        assert!(
            message.contains(expected_family),
            "failure must identify unsupported family '{expected_family}': {message}"
        );
    }
}

#[test]
fn nonlinear_hb_rejects_every_current_switch_before_surrogate_stamping() {
    let netlist = Netlist::parse(
        "\
* even a static control source needs its exact branch-current spectrum
iin 0 out dc 0
vctrl ctrl 0 dc 0
w1 out 0 vctrl csw
.model csw iswitch (ron=1 roff=1meg ion=1 ioff=0)
r1 out 0 1k
.end
",
    )
    .expect("current-switch rejection fixture parses");
    let error = Engine::new(SimulationConfig::default())
        .run_hb(&netlist, HbConfig::new(F0).with_harmonics(1))
        .expect_err("exact HB must not replace control current with a Norton voltage surrogate");
    let message = error.to_string();
    assert!(
        message
            .contains("current-controlled switches requiring exact control-branch current spectra"),
        "rejection must identify the missing exact current spectrum: {message}"
    );
}

#[test]
fn nonlinear_hb_rejects_device_models_it_would_otherwise_downgrade() {
    let cases = [
        (
            "\
* high injection cannot be reduced to the base Level-1 junction
v1 out 0 dc 1
d1 out 0 dmod
.model dmod d (is=1e-14 ikf=1m)
.end
",
            "diodes requiring high-injection",
        ),
        (
            "\
* complete Gummel-Poon is not the reduced HB Ebers-Moll kernel
vcc c 0 dc 5
vb b 0 dc 0.7
q1 c b 0 qmod
.model qmod npn (is=1e-16 bf=100 ikf=1m)
.end
",
            "native BJT/VBIC",
        ),
        (
            "\
* invalid diode grading must not be projected into the retained HB law
v1 out 0 dc 1
d1 out 0 dmod
.model dmod d (is=1e-14 m=1.1)
.end
",
            "invalid or nonrepresentable exact-HB junction parameters",
        ),
        (
            "\
* MOS2 equations cannot be routed through the HB MOS1 kernel
vd d 0 dc 1
vg g 0 dc 1
m1 d g 0 0 mmod l=1u w=10u
.model mmod nmos (level=2 vto=0.5 kp=1m)
.end
",
            "non-LEVEL=1",
        ),
        (
            "\
* Parker-Skellern JFET2 cannot be routed through Shichman-Hodges
vd d 0 dc 1
vg g 0 dc 0
j1 d g 0 jmod
.model jmod njf (level=2 vto=-1 beta=1m)
.end
",
            "non-Shichman-Hodges",
        ),
        (
            "\
* stateful switch hysteresis has no exact HB state evolution
v1 out 0 dc 1
vc ctrl 0 dc 0
s1 out 0 ctrl 0 smod
.model smod sw (vt=0 vh=0.1 ron=1 roff=1meg)
.end
",
            "requiring hysteresis",
        ),
    ];

    for (deck, expected_capability) in cases {
        let netlist = Netlist::parse(deck).expect("unsupported nonlinear model deck parses");
        let error = Engine::new(SimulationConfig::default())
            .run_hb(&netlist, HbConfig::new(F0).with_harmonics(1))
            .expect_err("HB must not silently downgrade a nonlinear device model");
        let message = error.to_string();
        assert!(
            message.contains(expected_capability),
            "failure must identify missing capability '{expected_capability}': {message}"
        );
    }
}

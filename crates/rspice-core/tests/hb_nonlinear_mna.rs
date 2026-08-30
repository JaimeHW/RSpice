//! Exact-MNA contracts for nonlinear harmonic balance.
//!
//! A nonlinear device must not change ideal voltage sources or inductors into
//! approximate nodal elements.  These regressions deliberately select the HB
//! Newton path with an otherwise unloaded level-1 MOSFET and then pin the
//! voltage law, source-current state, inconsistent-source handling, and the
//! resource cost of the additional branch spectra.

use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
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

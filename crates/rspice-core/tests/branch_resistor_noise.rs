use rspice_core::analysis::NoiseSourceType;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::constants::K_BOLTZMANN;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const TEMPERATURE: f64 = 300.15;

fn parse_resistor(resistance: &str, controls: &str, branch_form: bool) -> Netlist {
    let tolerance = if branch_form { "1" } else { "0" };
    Netlist::parse(&format!(
        "branch-form resistor noise\n\
         RAuth out 0 {resistance} {controls}\n\
         .options device zeroresistancetol={tolerance}\n\
         .end\n"
    ))
    .expect("resistor noise deck parses")
}

fn ordinary(netlist: &Netlist, frequency: f64) -> rspice_core::analysis::NoiseResult {
    let engine = Engine::new(SimulationConfig::default());
    let output = engine
        .build_circuit(netlist)
        .expect("noise circuit builds")
        .get_node_by_name("out")
        .expect("output node exists");
    engine
        .run_noise_ports(netlist, output, None, &[frequency], TEMPERATURE)
        .expect("ordinary noise solves")
        .remove(0)
}

fn pnoise(netlist: &Netlist, frequency: f64) -> rspice_core::engine::PnoiseAnalysisResult {
    Engine::new(SimulationConfig::default())
        .run_pnoise(netlist, 1.0e6, &[frequency], "out", None, None, 0)
        .expect("driven pnoise solves")
}

fn assert_relative(actual: f64, expected: f64, tolerance: f64, label: &str) {
    assert!(actual.is_finite(), "{label} is non-finite: {actual:e}");
    let relative = ((actual - expected) / expected).abs();
    assert!(
        relative <= tolerance,
        "{label}: actual={actual:.17e}, expected={expected:.17e}, relative={relative:.3e}"
    );
}

#[test]
fn branch_form_thermal_noise_matches_nodal_and_preserves_authored_identity() {
    let resistance = 0.6;
    let branch = parse_resistor("0.6", "", true);
    let nodal = parse_resistor("0.6", "", false);
    let expected = 4.0 * K_BOLTZMANN * TEMPERATURE * resistance;

    let ordinary_branch = ordinary(&branch, 1.0e3);
    let ordinary_nodal = ordinary(&nodal, 1.0e3);
    assert_relative(
        ordinary_branch.output_noise_density,
        expected,
        2.0e-12,
        "branch ordinary 4kTR",
    );
    assert_eq!(
        ordinary_branch.output_noise_density.to_bits(),
        ordinary_nodal.output_noise_density.to_bits(),
        "equivalent nodal and branch topologies must publish identical thermal noise"
    );
    let contributor = ordinary_branch
        .contributions
        .iter()
        .find(|source| source.identity.device.eq_ignore_ascii_case("RAuth"))
        .expect("authored branch resistor contributor is retained");
    assert_eq!(contributor.identity.mechanism, None);
    assert_eq!(contributor.noise_type, NoiseSourceType::Thermal);

    let periodic_branch = pnoise(&branch, 1.0e3);
    let periodic_nodal = pnoise(&nodal, 1.0e3);
    assert_relative(
        periodic_branch.output_noise[0],
        expected,
        2.0e-12,
        "branch pnoise 4kTR",
    );
    assert_relative(
        periodic_branch.output_noise[0],
        periodic_nodal.output_noise[0],
        2.0e-12,
        "branch/nodal pnoise parity",
    );
    assert!(
        periodic_branch
            .contributors
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("RAuth thermal"))
    );
}

#[test]
fn exact_zero_ohm_branch_is_an_ideal_constraint_with_no_noise_source() {
    let netlist = parse_resistor("0", "NOISY=1", true);
    let ordinary = ordinary(&netlist, 1.0e3);
    assert_eq!(ordinary.output_noise_density, 0.0);
    assert!(
        ordinary
            .contribution_catalog
            .iter()
            .all(|identity| !identity.device.eq_ignore_ascii_case("RAuth")),
        "an ideal zero-ohm branch has no finite resistor-noise mechanism"
    );
    assert!(ordinary.contributions.is_empty());

    let periodic = pnoise(&netlist, 1.0e3);
    assert_eq!(periodic.output_noise[0], 0.0);
    assert!(periodic.contributors.is_empty());
    assert!(periodic.converged);
}

#[test]
fn exact_resistor_branch_current_orientation_matches_vpos_minus_vneg_over_r() {
    let netlist = Netlist::parse(
        "exact resistor branch orientation\n\
         VBIAS in 0 1\n\
         RAuth in 0 0.5\n\
         CSTATE in 0 1p\n\
         D1 in 0 DM\n\
         .model DM D (IS=1e-12)\n\
         .options device zeroresistancetol=1\n\
         .end\n",
    )
    .expect("HB branch-orientation deck parses");
    let analysis = Engine::new(SimulationConfig::default())
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(1))
        .expect("nonlinear exact-MNA HB solves");
    let branch = analysis
        .result
        .mna_branch_currents
        .iter()
        .find(|branch| branch.device_name.eq_ignore_ascii_case("RAuth"))
        .expect("HB retains the resistor branch current");
    assert_eq!(branch.coefficients[0].im, 0.0);
    assert_relative(
        branch.coefficients[0].re,
        2.0,
        2.0e-12,
        "positive-terminal resistor branch current",
    );
}

#[test]
fn branch_and_nodal_pnoise_use_the_same_ac_override_for_source_and_operator() {
    let deck = |branch_form| {
        let tolerance = if branch_form { "1" } else { "0" };
        Netlist::parse(&format!(
            "resistor AC override noise\n\
             RAuth out 0 0.6 AC=1.2\n\
             .options device zeroresistancetol={tolerance}\n\
             .end\n"
        ))
        .expect("AC override deck parses")
    };
    let expected = 4.0 * K_BOLTZMANN * TEMPERATURE * 1.2;
    let nodal = deck(false);
    let branch = deck(true);
    assert_relative(
        ordinary(&nodal, 1.0e3).output_noise_density,
        expected,
        2.0e-12,
        "nodal ordinary AC override",
    );
    assert_relative(
        ordinary(&branch, 1.0e3).output_noise_density,
        expected,
        2.0e-12,
        "branch ordinary AC override",
    );
    let nodal_pnoise = pnoise(&nodal, 1.0e3).output_noise[0];
    let branch_pnoise = pnoise(&branch, 1.0e3).output_noise[0];
    assert_relative(nodal_pnoise, expected, 2.0e-12, "nodal pnoise AC override");
    assert_eq!(
        branch_pnoise.to_bits(),
        nodal_pnoise.to_bits(),
        "branch and nodal PNoise must use AC resistance in both source law and lifted operator"
    );
}

#[test]
fn branch_form_noise_honors_quiet_dtemp_and_absolute_temp_precedence() {
    for branch_form in [false, true] {
        let quiet = parse_resistor("0.6", "NOISY=0", branch_form);
        let ordinary_quiet = ordinary(&quiet, 1.0e3);
        assert_eq!(ordinary_quiet.output_noise_density, 0.0);
        assert!(ordinary_quiet.contributions.is_empty());
        let periodic_quiet = pnoise(&quiet, 1.0e3);
        assert_eq!(periodic_quiet.output_noise[0], 0.0);
        assert!(periodic_quiet.contributors.is_empty());

        let dtemp = parse_resistor("0.6", "DTEMP=150", branch_form);
        let expected_dtemp = 4.0 * K_BOLTZMANN * (TEMPERATURE + 150.0) * 0.6;
        assert_relative(
            ordinary(&dtemp, 1.0e3).output_noise_density,
            expected_dtemp,
            2.0e-12,
            "ordinary DTEMP",
        );
        assert_relative(
            pnoise(&dtemp, 1.0e3).output_noise[0],
            expected_dtemp,
            2.0e-12,
            "pnoise DTEMP",
        );
    }

    let make_absolute = |branch_form| {
        let tolerance = if branch_form { "1" } else { "0" };
        Netlist::parse(&format!(
            "absolute branch resistor temperature\n\
             RAuth out 0 RM 0.6 TEMP=27 DTEMP=900\n\
             .model RM R (TNOM=100)\n\
             .options device zeroresistancetol={tolerance}\n\
             .end\n"
        ))
        .expect("absolute TEMP deck parses")
    };
    let expected_absolute = 4.0 * K_BOLTZMANN * (TEMPERATURE + 100.0) * 0.6;
    for branch_form in [false, true] {
        let netlist = make_absolute(branch_form);
        assert_relative(
            ordinary(&netlist, 1.0e3).output_noise_density,
            expected_absolute,
            2.0e-12,
            "ordinary absolute TEMP",
        );
        assert_relative(
            pnoise(&netlist, 1.0e3).output_noise[0],
            expected_absolute,
            2.0e-12,
            "pnoise absolute TEMP",
        );
    }
}

#[test]
fn branch_form_ordinary_flicker_uses_exact_branch_current_and_matches_nodal() {
    let deck = |branch_form| {
        let tolerance = if branch_form { "1" } else { "0" };
        Netlist::parse(&format!(
            "branch resistor flicker parity\n\
             VBIAS in 0 1\n\
             RAuth in out RM 0.6\n\
             RLOAD out 0 10\n\
             .model RM R (KF=1e-12 AF=1.3 EF=0.8)\n\
             .options device zeroresistancetol={tolerance}\n\
             .end\n"
        ))
        .expect("flicker parity deck parses")
    };
    let branch = ordinary(&deck(true), 10.0);
    let nodal = ordinary(&deck(false), 10.0);
    let flicker = |result: &rspice_core::analysis::NoiseResult| {
        result
            .contributions
            .iter()
            .find(|source| {
                source.identity.device.eq_ignore_ascii_case("RAuth")
                    && source
                        .identity
                        .mechanism
                        .as_deref()
                        .is_some_and(|name| name.eq_ignore_ascii_case("FN"))
            })
            .expect("resistor FN contributor exists")
            .output_contribution
    };
    assert_relative(
        flicker(&branch),
        flicker(&nodal),
        2.0e-12,
        "branch/nodal resistor flicker",
    );
}

#[test]
fn branch_form_extreme_resistance_preserves_representable_subnormal_output_noise() {
    let resistance = 1.0e-300;
    let expected = 4.0 * K_BOLTZMANN * TEMPERATURE * resistance;
    assert!(expected > 0.0 && expected < f64::MIN_POSITIVE);
    for branch_form in [false, true] {
        let tolerance = if branch_form { "1e-299" } else { "0" };
        let netlist = Netlist::parse(&format!(
            "extreme branch resistor noise\n\
             RAuth out 0 1e-300\n\
             .options device zeroresistancetol={tolerance}\n\
             .end\n"
        ))
        .expect("extreme deck parses");
        assert_relative(
            ordinary(&netlist, 1.0e3).output_noise_density,
            expected,
            5.0e-4,
            "ordinary subnormal 4kTR",
        );
        assert_relative(
            pnoise(&netlist, 1.0e3).output_noise[0],
            expected,
            5.0e-4,
            "pnoise subnormal 4kTR",
        );
    }
}

#[test]
fn pnoise_scales_an_overflowing_elementary_density_while_ordinary_noise_refuses_precisely() {
    let temperature = 1.0e20;
    let resistance = 1.0e-320;
    let expected = 4.0 * K_BOLTZMANN * temperature * resistance;
    assert!(expected > 0.0 && expected.is_finite());
    assert!((4.0 * K_BOLTZMANN * temperature / resistance).is_infinite());
    let netlist = Netlist::parse(
        "overflowing elementary branch density\n\
         RAuth out 0 1e-320\n\
         .options device zeroresistancetol=1e-319\n\
         .end\n",
    )
    .expect("overflow-density deck parses");
    let engine = Engine::new(SimulationConfig {
        temperature,
        ..SimulationConfig::default()
    });
    let output = engine
        .build_circuit(&netlist)
        .expect("overflow-density circuit builds")
        .get_node_by_name("out")
        .expect("output node exists");
    let error = engine
        .run_noise_ports(&netlist, output, None, &[1.0e3], temperature)
        .expect_err("ordinary NoiseSource density cannot materialize infinity");
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains("rauth")
            && message.contains("spectral density")
            && message.contains("finite"),
        "ordinary overflow refusal must identify the source and failed contract: {message}"
    );

    let periodic = engine
        .run_pnoise(&netlist, 1.0e6, &[1.0e3], "out", None, None, 0)
        .expect("PNoise binary scaling preserves the transferred result");
    assert_relative(
        periodic.output_noise[0],
        expected,
        0.1,
        "scaled overflowing-density PNoise result",
    );
}

#[test]
fn noisy_negative_branch_resistance_fails_closed_but_quiet_branch_is_allowed() {
    let noisy = parse_resistor("-0.6", "NOISY=1", true);
    let engine = Engine::new(SimulationConfig::default());
    let output = engine
        .build_circuit(&noisy)
        .expect("negative branch circuit builds")
        .get_node_by_name("out")
        .expect("output node exists");
    let error = engine
        .run_noise_ports(&noisy, output, None, &[1.0e3], TEMPERATURE)
        .expect_err("negative noisy resistance must fail closed");
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains("rauth") && message.contains("finite and nonnegative"),
        "unexpected ordinary negative-resistance diagnostic: {message}"
    );
    let error = engine
        .run_pnoise(&noisy, 1.0e6, &[1.0e3], "out", None, None, 0)
        .expect_err("negative pnoise resistance must fail closed");
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains("rauth") && message.contains("invalid noise resistance"),
        "unexpected pnoise negative-resistance diagnostic: {message}"
    );

    let quiet = parse_resistor("-0.6", "NOISY=0", true);
    assert_eq!(ordinary(&quiet, 1.0e3).output_noise_density, 0.0);
    assert_eq!(pnoise(&quiet, 1.0e3).output_noise[0], 0.0);
}

#[test]
fn active_branch_flicker_is_the_only_branch_noise_contract_pnoise_rejects() {
    let netlist = Netlist::parse(
        "active branch flicker\n\
         RAuth out 0 RM 0.6\n\
         .model RM R (KF=1e-12 AF=1 EF=1)\n\
         .options device zeroresistancetol=1\n\
         .end\n",
    )
    .expect("active branch flicker deck parses");
    let error = Engine::new(SimulationConfig::default())
        .run_pnoise(&netlist, 1.0e6, &[1.0e3], "out", None, None, 0)
        .expect_err("cyclostationary branch flicker must fail closed");
    let message = error.to_string();
    let normalized = message.to_ascii_lowercase();
    assert!(
        message.contains("cyclostationary colored-noise")
            && normalized.contains("branch-form resistor 'rauth'")
            && message.contains("cyclostationary flicker noise"),
        "unexpected branch flicker diagnostic: {message}"
    );
}

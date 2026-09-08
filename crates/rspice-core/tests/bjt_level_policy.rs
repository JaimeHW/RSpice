//! Build-time policy for BJT model levels.
//!
//! Native GP (including ngspice LEVEL=2) and VBIC decks should keep running,
//! but advanced BJT families without native implementations must not be
//! silently evaluated as VBIC or legacy GP.

use rspice_core::engine::{ConvergenceConfig, Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{ElementKind, Netlist};

const VBIC13_TEST_VBBE: f64 = 2.0;
const VBIC13_TEST_NBBE: f64 = 5.0;
const VBIC13_TEST_IBBE: f64 = 1e-9;
const VBIC13_TEST_VBE: f64 = -2.4;

fn op_deck(model_line: &str) -> String {
    format!(
        "* bjt level policy\n\
         vc c 0 dc 1.0\n\
         vb b 0 dc 0.7\n\
         q1 c b 0 qmod\n\
         {model_line}\n\
         .op\n\
         .end\n"
    )
}

fn inert_vbic13_params() -> &'static str {
    "VBBE=0 NBBE=1 IBBE=1e-6 TVBBE1=0 TVBBE2=0 TNBBE=0 EBBE=0"
}

fn active_vbic13_wbe1_breakdown_deck(level: i32, vbe: f64) -> String {
    active_vbic13_wbe1_breakdown_deck_with_extra(level, vbe, "")
}

fn active_vbic13_wbe1_breakdown_deck_with_options(
    level: i32,
    vbe: f64,
    options: &str,
    extra_params: &str,
) -> String {
    let options = if options.is_empty() {
        String::new()
    } else {
        format!(" {options}")
    };
    format!(
        "* VBIC13 reverse B-E breakdown, WBE=1 Xyce/ngspice agreement path\n\
         .options gmin=0 reltol=1e-9 abstol=1e-15 vntol=1e-12{options}\n\
         vc c 0 dc 0\n\
         vb b 0 dc {vbe}\n\
         q1 c b 0 qmod\n\
         .model qmod NPN (LEVEL={level} IS=1e-30 ISRR=0 IBEI=0 IBEN=0 IBCI=0 IBCN=0 \
         RCX=0 RCI=0 RBX=0 RBI=0 RE=0 WBE=1 VBBE={VBIC13_TEST_VBBE} \
         NBBE={VBIC13_TEST_NBBE} IBBE={VBIC13_TEST_IBBE} {extra_params})\n\
         .op\n\
         .end\n"
    )
}

fn active_vbic13_wbe1_breakdown_deck_with_extra(
    level: i32,
    vbe: f64,
    extra_params: &str,
) -> String {
    active_vbic13_wbe1_breakdown_deck_with_options(level, vbe, "", extra_params)
}

fn vbic13_wbe1_reverse_be_source_current(vbe: f64) -> f64 {
    vbic13_wbe1_reverse_be_source_current_at(
        vbe,
        rspice_core::constants::TEMP_REFERENCE,
        0.0,
        0.0,
        0.0,
    )
}

fn vbic13_wbe1_reverse_be_source_current_at(
    vbe: f64,
    temp_k: f64,
    tvbbe1: f64,
    tvbbe2: f64,
    tnbbe: f64,
) -> f64 {
    let delta_t = temp_k - rspice_core::constants::TEMP_REFERENCE;
    // VBIC13's model-defined Vtv (ngspice vbicload.c and Xyce ADMS VBIC13).
    let vt = 1.380662e-23 * temp_k / 1.602189e-19;
    let vbbe = VBIC13_TEST_VBBE * (1.0 + delta_t * (tvbbe1 + delta_t * tvbbe2));
    let nbbe = VBIC13_TEST_NBBE * (1.0 + delta_t * tnbbe);
    let exponent = (-vbbe - vbe) / (nbbe * vt);
    let ebbe = (-vbbe / (nbbe * vt)).exp();
    VBIC13_TEST_IBBE * (exponent.exp() - ebbe)
}

fn xyce_vbic_oracle_deck(level: i32) -> String {
    xyce_vbic_oracle_deck_with_extra_params(level, "")
}

fn xyce_vbic_oracle_deck_with_extra_params(level: i32, extra_params: &str) -> String {
    let substrate = if level == 12 {
        "vs s 0 dc 0\nq1 c b e s qmod\n"
    } else {
        "q1 c b e qmod\n"
    };
    format!(
        "* Xyce 7.10 VBIC LEVEL={level} operating-point oracle\n\
         vc c 0 dc 1.2\n\
         vb b 0 dc 0.78\n\
         ve e 0 dc 0\n\
         {substrate}\
         .model qmod npn level={level} IS=1e-16 IBEI=1e-18 IBEN=5e-15 IBCI=2e-17 IBCN=5e-15 \
         ISP=1e-15 RCX=1 RCI=2 RBX=1 RBI=2 RE=1 RS=0 RBP=1 VEF=10 VER=4 IKF=2e-3 \
         ITF=8e-2 XTF=20 IKR=2e-4 IKP=2e-4 CJE=1e-13 CJC=2e-14 CJEP=1e-13 \
         CJCP=4e-13 VO=2 GAMM=2e-11 HRCF=2 QCO=1e-12 AVC1=2 AVC2=15 TF=10e-12 \
         TR=100e-12 TD=0 RTH=0 GMIN=0.0 {extra_params}\n\
         .op\n\
         .end\n"
    )
}

fn run(deck: &str) -> Result<(), String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn branch_current(deck: &str, branch: &str) -> f64 {
    branch_current_with_config(deck, branch, SimulationConfig::default())
}

fn branch_current_with_config(deck: &str, branch: &str, config: SimulationConfig) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(config)
        .run_dc_op(&netlist)
        .expect("op converges");
    result
        .branch_current_named(branch)
        .unwrap_or_else(|| panic!("missing {branch} branch in {:?}", result.branch_names))
}

fn op_result(deck: &str) -> rspice_core::solver::SimulationResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("op failed: {error}\n{deck}"))
}

fn voltage(result: &rspice_core::solver::SimulationResult, node: &str) -> f64 {
    result
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("missing {node} node in {:?}", result.node_names))
}

fn assert_rel_close(label: &str, got: f64, expected: f64, rel_tol: f64) {
    let rel = (got - expected).abs() / expected.abs().max(1e-30);
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.9e}, expected {expected:.9e}, rel {rel:.3e} > {rel_tol:.3e}"
    );
}

#[test]
fn legacy_gummel_poon_does_not_stamp_vbic_parasitic_gmin_branches() {
    let deck = "* legacy GP BJT GMIN topology\n\
                VCC 4 0 5\n\
                VIN 1 0 0\n\
                RB 1 2 10K\n\
                Q1 3 2 0 Q1\n\
                RC 3 4 1K\n\
                .MODEL Q1 NPN (BF=20 RB=100 TF=1e-10 CJC=2e-12)\n\
                .OP\n\
                .END\n";
    let netlist = Netlist::parse(deck).expect("legacy GP regression deck parses");
    let defaults = SimulationConfig::default();
    let mut convergence_config = ConvergenceConfig::robust();
    convergence_config.voltage_reltol = 1.0e-4;
    let config = SimulationConfig {
        max_iterations: defaults.max_iterations.max(1200),
        convergence_config,
        spice_dialect: SpiceDialect::Xyce,
        temperature: 300.15,
        ..defaults
    };
    let result = Engine::new(config)
        .run_dc_op(&netlist)
        .expect("legacy GP regression deck converges");

    assert_rel_close(
        "legacy GP VIN startup current",
        result
            .branch_current_named("VIN")
            .expect("VIN branch is retained"),
        5.0001e-12,
        1.0e-6,
    );
    assert_rel_close(
        "legacy GP VCC startup current",
        result
            .branch_current_named("VCC")
            .expect("VCC branch is retained"),
        -1.0000e-11,
        2.0e-3,
    );
}

fn build(deck: &str) -> Result<rspice_core::CircuitData, String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    Engine::new(SimulationConfig::default())
        .build_circuit(&netlist)
        .map_err(|err| err.to_string())
}

fn ac_branch_current(deck: &str, branch: &str, frequency: f64) -> rspice_core::Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("AC converges")
        .pop()
        .expect("one AC point");
    let index = result
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(branch))
        .unwrap_or_else(|| panic!("missing {branch} branch in {:?}", result.branch_names));
    result.currents[index]
}

#[test]
fn five_terminal_bjt_syntax_keeps_extra_nodes_and_model_name() {
    let deck = "* five-terminal BJT syntax\n\
                q1 coll base emit subs aux qmod\n\
                .model qmod NPN (IS=1e-16 BF=100)\n\
                .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let q1 = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("q1"))
        .expect("q1 element");

    assert_eq!(
        q1.nodes,
        vec![
            "COLL".to_string(),
            "BASE".to_string(),
            "EMIT".to_string(),
            "SUBS".to_string(),
            "AUX".to_string()
        ]
    );
    match &q1.kind {
        ElementKind::Bjt { model, .. } => assert!(
            model.eq_ignore_ascii_case("qmod"),
            "expected BJT model name qmod, got {model}"
        ),
        other => panic!("expected BJT element, got {other:?}"),
    }
}

#[test]
fn bracketed_bjt_substrate_node_matches_unbracketed_ngspice_form() {
    let unbracketed = xyce_vbic_oracle_deck(12);
    let bracketed = unbracketed.replace("q1 c b e s qmod", "q1 c b e [s] qmod");
    let netlist = Netlist::parse(&bracketed).expect("bracketed substrate BJT deck parses");
    let q1 = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("q1"))
        .expect("q1 element");
    assert_eq!(
        q1.nodes,
        vec![
            "C".to_string(),
            "B".to_string(),
            "E".to_string(),
            "S".to_string()
        ]
    );

    for branch in ["vc", "vb", "ve"] {
        assert_rel_close(
            &format!("bracketed BJT substrate {branch} current"),
            branch_current(&bracketed, branch),
            branch_current(&unbracketed, branch),
            1.0e-12,
        );
    }
}

#[test]
fn bracketed_bjt_ground_substrate_is_accepted_as_ground_node() {
    let deck = xyce_vbic_oracle_deck(11).replace("q1 c b e qmod", "q1 c b e [0] qmod");
    let netlist = Netlist::parse(&deck).expect("bracketed ground substrate BJT deck parses");
    let q1 = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case("q1"))
        .expect("q1 element");
    assert_eq!(
        q1.nodes,
        vec![
            "C".to_string(),
            "B".to_string(),
            "E".to_string(),
            "0".to_string()
        ]
    );
    run(&deck).expect("bracketed ground-substrate BJT deck runs");
}

#[test]
fn cmc_bjt_levels_fail_closed_as_veriloga_codegen_targets() {
    for (level, family) in [
        (8, "HICUM"),
        (230, "HICUM"),
        (234, "HICUM"),
        (504, "MEXTRAM"),
        (505, "MEXTRAM"),
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} VERSION=2.40 IS=1e-16 BF=100)"
        ));
        let message = run(&deck).expect_err("CMC BJT level must fail closed");

        assert!(
            message.contains(&format!("LEVEL={level}")) && message.contains(family),
            "LEVEL={level} error should identify {family}: {message}"
        );
        assert!(
            message.contains("Verilog-A-to-Rust") && message.contains("codegen"),
            "LEVEL={level} error should describe the codegen path: {message}"
        );
        assert!(
            message.contains("must not fall back"),
            "LEVEL={level} error should forbid fallback routing: {message}"
        );
    }
}

#[test]
fn unsupported_advanced_bjt_level_is_rejected() {
    let level = 235;
    let deck = op_deck(&format!(
        ".model qmod NPN (LEVEL={level} VERSION=2.34 IS=1e-16 BF=100)"
    ));
    let message = run(&deck).expect_err(&format!("unsupported LEVEL={level} must not run as VBIC"));
    assert!(
        message.contains("BJT"),
        "error identifies the device family: {message}"
    );
    assert!(
        message.contains(&format!("LEVEL={level}")),
        "error names the unsupported level: {message}"
    );
    assert!(
        message.contains("unsupported") || message.contains("no native implementation"),
        "error explains the unsupported routing: {message}"
    );
    assert!(
        message.contains("generated from Verilog-A") || message.contains("no native"),
        "error should point to future generated support instead of fallback routing: {message}"
    );
}

#[test]
fn xyce_hbt_x_level23_fails_closed_by_name() {
    let deck = op_deck(".model qmod NPN (LEVEL=23)");
    let message = run(&deck).expect_err("Xyce HBT_X LEVEL=23 must fail closed by name");

    assert!(
        message.contains("Xyce")
            && message.contains("HBT_X")
            && message.contains("LEVEL=23")
            && message.contains("native"),
        "HBT_X error should identify the unsupported Xyce native model family: {message}"
    );
    assert!(
        message.contains("Verilog-A"),
        "HBT_X error should make the no-Verilog-A-fallback rule explicit: {message}"
    );
}

#[test]
fn ngspice_bjt_level2_runs_as_legacy_gummel_poon_alias() {
    let deck = op_deck(".model qmod NPN (LEVEL=2 IS=1e-16 BF=100 VAF=50)");
    run(&deck).expect("ngspice BJT LEVEL=2 alias must run on the native GP path");

    // ngspice 46 maps LEVEL=2 to its legacy BJT device for this card.
    assert_rel_close(
        "LEVEL=2 I(VC)",
        branch_current(&deck, "vc"),
        -5.704_37e-5,
        1.0e-5,
    );
    assert_rel_close(
        "LEVEL=2 I(VB)",
        branch_current(&deck, "vb"),
        -5.670_35e-7,
        1.2e-5,
    );
}

#[test]
fn level1_gummel_poon_bias_dependent_transit_time_parameters_stay_legacy() {
    let deck_for = |transit_params: &str| {
        format!(
            "* SPICE Gummel-Poon bias-dependent forward transit time\n\
             vc c 0 dc 5\n\
             vb b 0 dc 0.7 ac 1\n\
             q1 c b 0 qmod\n\
             .model qmod NPN (IS=1e-16 BF=100 ISE=1e-16 NE=1.5 \
             VAF=50 TF=1n {transit_params})\n\
             .op\n\
             .end\n"
        )
    };
    let ordinary = deck_for("");
    let bias_dependent = deck_for("XTF=3 VTF=10 ITF=0");

    // XTF/VTF/ITF alter only stored charge. An implicit level-1 card carrying
    // them must retain the same Gummel-Poon DC transport and base-current model.
    for branch in ["vc", "vb"] {
        assert_rel_close(
            &format!("level-1 GP {branch} DC current"),
            branch_current(&bias_dependent, branch),
            branch_current(&ordinary, branch),
            1.0e-12,
        );
    }

    // The parameters must still participate in the legacy dynamic-charge
    // equation: with ITF=0, XTF scales TF by the VBC-dependent factor.
    let frequency = 1.0e8;
    let ordinary_input = ac_branch_current(&ordinary, "vb", frequency);
    let bias_dependent_input = ac_branch_current(&bias_dependent, "vb", frequency);
    assert!(
        bias_dependent_input.im.abs() > ordinary_input.im.abs() * 1.5,
        "XTF/VTF/ITF should increase the level-1 GP input charge current: \
         ordinary={ordinary_input:?}, bias-dependent={bias_dependent_input:?}"
    );
}

#[test]
fn legacy_bjt_ac_preserves_negative_transport_charge_derivative() {
    // The GP law is Qbe = TF * IS * (exp(Vbe/Vt) - 1) * (1 - Vbe/VAR).
    // Near VAR, its derivative is negative. ngspice 46 gives I(VB).im =
    // +4.467787e-7 A at 1 MHz for this card; clipping the slope gives zero.
    let frequency = 1e6;
    let vt = rspice_core::analysis::temperature::thermal_voltage(300.15);
    let exponential = (0.71 / vt).exp();
    let capbe =
        1e-9 * 1e-16 * (exponential / vt * (1.0 - 0.71 / 0.72) - (exponential - 1.0) / 0.72);
    assert!(capbe < 0.0);
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let bias = 0.71 * polarity;
        let deck = format!(
            "* Signed GP transport charge\n\
             .options gmin=0\n\
             vc c 0 {bias}\n\
             vb b 0 {bias} ac 1\n\
             q1 c b 0 qm\n\
             .model qm {kind} (LEVEL=1 IS=1e-16 TF=1n VAR=0.72)\n\
             .end\n"
        );
        assert_rel_close(
            kind,
            ac_branch_current(&deck, "vb", frequency).im,
            -std::f64::consts::TAU * frequency * capbe,
            1e-6,
        );
    }
}

#[test]
fn legacy_bjt_transient_current_integrates_charge_with_negative_slope() {
    let vt = rspice_core::analysis::temperature::thermal_voltage(300.15);
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let deck = format!(
            "* GP current through a decreasing charge segment\n\
             .options gmin=0 reltol=1e-6 abstol=1e-15 vntol=1e-12\n\
             vc c 0 {}\n\
             vb b 0 PWL(0 {} 20n {})\n\
             q1 c b 0 qm\n\
             .model qm {kind} (LEVEL=1 IS=1e-16 TF=1n VAR=0.72 BF=100)\n\
             .tran 1n 20n\n.end\n",
            polarity * 0.71,
            polarity * 0.70,
            polarity * 0.71,
        );
        let netlist = Netlist::parse(&deck).expect("charge ramp parses");
        let config = rspice_core::engine::resolve_simulation_config(
            &SimulationConfig {
                integration_method:
                    rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
                ..SimulationConfig::default()
            },
            Some(&netlist.options),
            &Default::default(),
        );
        let result = Engine::new(config)
            .run_tran(&netlist, 20e-9, 1e-9)
            .expect("charge ramp solves");
        let voltage = result
            .try_voltage_waveform_named("b")
            .expect("base voltage");
        let current = result
            .try_branch_current_waveform_named("vb")
            .expect("base source current");
        let charge = |v: f64| {
            polarity
                * 1e-9
                * 1e-16
                * ((polarity * v / vt).exp() - 1.0)
                * (1.0 - polarity * v / 0.72)
        };
        assert!(result.time.len() > 3);
        for index in 1..result.time.len() {
            let vbe = polarity * voltage[index];
            let vbc = vbe - 0.71;
            let static_base =
                polarity * 1e-16 * (((vbe / vt).exp() - 1.0) / 100.0 + (vbc / vt).exp() - 1.0);
            let dynamic_base = (charge(voltage[index]) - charge(voltage[index - 1]))
                / (result.time[index] - result.time[index - 1]);
            assert!(
                (current[index] + static_base + dynamic_base).abs() < 1e-11,
                "{kind} at {}: source={}, static={static_base}, dynamic={dynamic_base}",
                result.time[index],
                current[index],
            );
        }
    }
}

#[test]
fn xyce_dialect_uses_xyce710_bjt_thermal_voltage_constants() {
    let deck = op_deck(".model qmod NPN (IS=1e-16 BF=100)").replace(
        "* bjt level policy\n",
        "* bjt level policy\n.options gmin=0\n",
    );
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
    let base_source_current = branch_current_with_config(&deck, "vb", config);

    // Xyce 7.10 N_DEV_Const.h defines k and q for native devices. At the
    // fixed 0.7 V base bias this is the level-1 ideal base-junction current;
    // the reverse B-C term is below the asserted relative precision.
    let vt: f64 = 300.15 * 1.380_622_6e-23 / 1.602_191_8e-19;
    let expected = -(1.0e-16 / 100.0) * (0.7 / vt).exp_m1();
    assert_rel_close(
        "Xyce-dialect level-1 BJT I(VB)",
        base_source_current,
        expected,
        1.0e-8,
    );

    // The compatibility selection is isolated: native best-available and
    // explicit ngspice operation retain current SI/CODATA k and q.
    let modern_vt: f64 = 300.15 * 1.380_649e-23 / 1.602_176_634e-19;
    let modern_expected = -(1.0e-16 / 100.0) * (0.7 / modern_vt).exp_m1();
    for (label, config) in [
        ("best-available", SimulationConfig::default()),
        (
            "ngspice",
            SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice),
        ),
    ] {
        assert_rel_close(
            &format!("{label} level-1 BJT I(VB)"),
            branch_current_with_config(&deck, "vb", config),
            modern_expected,
            1.0e-8,
        );
    }
}

#[test]
fn ngspice_lpnp_model_type_routes_to_native_lateral_pnp() {
    let deck_for = |model_type: &str| {
        format!(
            "* ngspice {model_type} legacy BJT alias\n\
             vc c 0 dc -1.0\n\
             vb b 0 dc -0.7\n\
             q1 c b 0 qmod\n\
             .model qmod {model_type} (IS=1e-16 BF=100 VAF=50)\n\
             .op\n\
             .end\n"
        )
    };
    let lpnp = deck_for("LPNP");
    let pnp = deck_for("PNP");

    // ngspice rewrites LPNP to PNP with lateral substrate semantics; native
    // PNP already defaults to the same lateral topology in RSpice.
    assert_rel_close(
        "LPNP I(VC)",
        branch_current(&lpnp, "vc"),
        branch_current(&pnp, "vc"),
        1.0e-12,
    );
    assert_rel_close(
        "LPNP I(VB)",
        branch_current(&lpnp, "vb"),
        branch_current(&pnp, "vb"),
        1.0e-12,
    );
}

#[test]
fn ngspice_lpnp_alias_rejects_nonlegacy_bjt_levels_until_oracled() {
    let deck = op_deck(".model qmod LPNP (LEVEL=4 IS=1e-16 IBEI=1e-18 IBCI=1e-18)");
    let message =
        run(&deck).expect_err("LPNP must not silently select non-legacy native BJT levels");

    assert!(
        message.contains("LPNP") && message.contains("LEVEL=4") && message.contains("legacy"),
        "LPNP advanced-level error should name the legacy alias boundary: {message}"
    );
}

#[test]
fn unresolved_bjt_level_selector_fails_closed() {
    let deck = op_deck(".model qmod NPN (LEVEL={native_level} IS=1e-16 BF=100)");
    let message =
        run(&deck).expect_err("unresolved BJT LEVEL selector must not fall back to legacy GP");

    assert!(
        message.contains("BJT") && message.contains("LEVEL"),
        "error should identify the BJT LEVEL selector: {message}"
    );
    assert!(
        message.contains("unresolved") && message.contains("finite numeric literal"),
        "error should explain unresolved selectors must be finite numeric literals: {message}"
    );
}

#[test]
fn non_numeric_bjt_level_selector_fails_closed() {
    let deck = op_deck(".model qmod NPN (LEVEL=\"504\" IS=1e-16 BF=100)");
    let message =
        run(&deck).expect_err("non-numeric BJT LEVEL selector must not fall back to legacy GP");

    assert!(
        message.contains("BJT") && message.contains("LEVEL"),
        "error should identify the BJT LEVEL selector: {message}"
    );
    assert!(
        message.contains("non-numeric") && message.contains("finite numeric literal"),
        "error should explain string selectors must be finite numeric literals: {message}"
    );
}

#[test]
fn vbic_level11_rejects_unresolved_native_model_params() {
    let deck = op_deck(".model qmod NPN (LEVEL=11 IS={is_native} IBEI=1e-18 IBCI=1e-18 RCX=1)");
    let message = run(&deck).expect_err("unresolved VBIC model parameter must fail closed");

    assert!(
        message.contains("VBIC") && message.contains("IS"),
        "error should identify the unresolved VBIC model parameter: {message}"
    );
    assert!(
        message.contains("unresolved") && message.contains("finite numeric literal"),
        "error should explain native VBIC model parameters must be numeric: {message}"
    );
}

#[test]
fn vbic_level4_still_runs_natively() {
    let deck = op_deck(".model qmod NPN (LEVEL=4 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1)");
    run(&deck).expect("LEVEL=4 remains the native VBIC selector");
}

#[test]
fn xyce_vbic_levels_11_and_12_run_natively() {
    for level in [11, 12] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2)"
        ));
        run(&deck).unwrap_or_else(|err| panic!("Xyce VBIC LEVEL={level} must run natively: {err}"));
    }
}

#[test]
fn xyce_vbic_levels_11_and_12_use_vbic_internal_topology() {
    for level in [11, 12] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2 RE=1)"
        ));
        let circuit = build(&deck)
            .unwrap_or_else(|err| panic!("Xyce VBIC LEVEL={level} circuit builds: {err}"));
        for node in [
            "q1.__cx.internal",
            "q1.__ci.internal",
            "q1.__bx.internal",
            "q1.__bi.internal",
            "q1.__ei.internal",
        ] {
            assert!(
                circuit.get_node_by_name(node).is_some(),
                "LEVEL={level} should allocate VBIC internal node {node}"
            );
        }
    }
}

#[test]
fn vbic13_self_heating_switch_matches_xyce710_and_grounded_thermal_pins() {
    // Live Xyce 7.10, 2026-09-08: this LEVEL=11 deck gives the following
    // collector currents at 27 C with RTH=1000. SW_ET defaults to one.
    for level in [11, 12] {
        for (kind, sign) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            let deck = |thermal: &str, control: &str| {
                format!(
                    "VBIC13 self-heating oracle\nVc c 0 {}\nVb b 0 {}\n\
                 Q1 c b 0{substrate}{thermal} vm {control}\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0 RTH=1000 TNOM=27)\n.temp 27\n.end\n",
                    1.2 * sign,
                    0.7 * sign
                )
            };
            for (control, current) in [
                ("", -5.69505259e-5),
                ("SW_ET=1", -5.69505259e-5),
                ("SW_ET=0", -5.67002151e-5),
            ] {
                let result = op_result(&deck("", control));
                assert_rel_close(
                    "VBIC13 collector current",
                    result.branch_current_named("vc").unwrap(),
                    sign * current,
                    1e-5,
                );
                let rise = voltage(&result, "q1.__rth.internal");
                if control == "SW_ET=0" {
                    assert!(rise.abs() < 1e-10);
                } else {
                    assert!(rise > 0.06 && rise < 0.08, "{rise}");
                }
            }
            let grounded = op_result(&deck(" 0", "SW_ET=1"));
            assert!(
                !grounded
                    .node_names
                    .iter()
                    .any(|name| name.contains(".__rth."))
            );
            assert_rel_close(
                "grounded thermal pin",
                grounded.branch_current_named("vc").unwrap(),
                sign * -5.67002151e-5,
                1e-5,
            );
            let floor = op_result(&deck("", "").replace("RTH=1000", "RTH=0"));
            let rise = voltage(&floor, "q1.__rth.internal");
            assert!(
                rise > 6e-8 && rise < 8e-8,
                "RTH=0 keeps Xyce's 1 mK/W floor: {rise}"
            );
        }
    }
}

#[test]
fn vbic_parasitic_high_injection_scaling_matches_xyce710() {
    for (multiplier, expected) in [(1, 8.470381112623551e-6), (3, 1.7782090999422936e-5)] {
        let netlist = Netlist::parse(&format!(
            "VBIC parasitic knee-current scaling\nVcc vcc 0 0.1\nRc vcc c 1k\nVb drive 0 0.7\nRb drive b 1k\nVss supply 0 -0.2\nRsub supply s 1k\nVth th 0 0\nQ1 c b 0 s th vm SW_ET=0 M={multiplier}\n\
             .model vm NPN(LEVEL=12 IS=1e-40 IBEI=0 IBCI=0 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=30 RS=50 GMIN=0 IBEIP=0 ISP=1e-15 TNOM=27 WSP=0.6 IKP=1e-3 CJCP=1p)\n.temp 27\n.options gmin=0\n.end\n"
        )).unwrap();
        let result = Engine::default().run_dc_op(&netlist).unwrap();
        let actual = result.branch_current_named("Vss").unwrap();
        assert!(
            (actual - expected).abs() < 2e-7 * expected,
            "M={multiplier}: {actual:e} != {expected:e}"
        );
    }
}

#[test]
fn vbic_noise_parameters_require_finite_values_in_the_model_domain() {
    for level in [4, 9, 11, 12, 13] {
        for parameter in ["KFN=-1", "KFN=\"noisy\"", "AFN={1/0}"] {
            let error = build(&op_deck(&format!(
                ".model qmod NPN(LEVEL={level} {parameter})"
            )))
            .expect_err("invalid noise parameters must not silently select defaults");
            assert!(
                error
                    .to_string()
                    .contains(parameter.split('=').next().unwrap())
            );
        }
        build(&op_deck(&format!(
            ".model qmod NPN(LEVEL={level} KFN=0 AFN=1e-15 BFN=1e-15)"
        )))
        .unwrap();
    }
    for level in [4, 9, 13] {
        build(&op_deck(&format!(
            ".model qmod NPN(LEVEL={level} KFN=1e-20 AFN=0 BFN=-0.5)"
        )))
        .unwrap();
    }
    for level in [11, 12] {
        for parameter in ["AFN=0", "BFN=-0.5"] {
            build(&op_deck(&format!(
                ".model qmod NPN(LEVEL={level} {parameter})"
            )))
            .expect_err("VBIC 1.3 requires positive noise exponents");
        }
    }
    for level in [0, 1, 2] {
        let error = build(&op_deck(&format!(
            ".model qmod NPN(LEVEL={level} KFN=1e-8)"
        )))
        .unwrap_err();
        assert!(error.to_string().contains("native VBIC"));
    }
}

#[test]
fn vbic13_early_voltage_temperature_coefficients_match_xyce710() {
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            ..Default::default()
        },
        ..Default::default()
    });
    for (parameters, expected) in [
        ("", [-0.00016897815020348446, 9.283364676043807e-9]),
        (
            "TCVEF=0.05",
            [-0.00015660535070869053, 8.788299217636993e-9],
        ),
        (
            "TCVER=0.05",
            [-0.00019616883263713254, 1.0369929843899738e-8],
        ),
        (
            "TCVEF=-0.1 TCVER=-0.1",
            [-0.00019170092234187925, 1.0191519968825945e-8],
        ),
    ] {
        let netlist = Netlist::parse(&format!(
            "VBIC13 Early-voltage temperature oracle\nVc c 0 1.8\nVb b 0 0.7\nQ1 c b 0 vm SW_ET=0 TRISE=20\n\
             .model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 VEF=5 VER=3 {parameters} GMIN=1u TNOM=27)\n.temp 27\n.end\n"
        )).unwrap();
        let result = engine.run_dc_op(&netlist).unwrap();
        for (branch, expected) in ["vc", "vb"].into_iter().zip(expected) {
            let actual = result.branch_current_named(branch).unwrap();
            assert!(
                (actual - expected).abs() < 2e-6 * expected.abs(),
                "{parameters} {branch}: {actual:e} != {expected:e}"
            );
        }
    }
}

#[test]
fn vbic13_avalanche_and_pushout_currents_match_xyce710() {
    // Independent Xyce 7.10 vbic_1p3.va DC references. Its PNP Igcx
    // polarity depends on the physical collector-resistor current.
    for (level, kind, control, vc, parameters, expected) in [
        (
            11,
            "NPN",
            "",
            1.8,
            "AVCX1=0.05",
            [-6.251124606622771e-5, 3.931202464614644e-6],
        ),
        (
            11,
            "PNP",
            "",
            1.8,
            "AVCX1=0.05",
            [5.7467919611412814e-5, 1.057088131301366e-6],
        ),
        (
            12,
            "PNP",
            "",
            1.8,
            "AVCX1=0.05",
            [5.919472910615865e-5, 1.1288947081972223e-6],
        ),
        (
            12,
            "NPN",
            "M=3",
            1.8,
            "AVCX1=0.05",
            [-0.0001931683165093576, 1.2027535570122183e-5],
        ),
        (
            12,
            "NPN",
            "TRISE=40",
            1.8,
            "AVCX1=0.05 TAVCX=0.01",
            [-0.0005742348152047687, 1.7613976968633497e-5],
        ),
        (
            12,
            "NPN",
            "TRISE=20",
            1.8,
            "AVCX1=0.05 TAVCX=-0.06",
            [-0.00020588580959210147, 1.1986467093786921e-5],
        ),
        (
            12,
            "NPN",
            "",
            1.8,
            "AVCX1=0.05 MCX=1",
            [-6.433674528014668e-5, 3.957059748605426e-6],
        ),
        (
            12,
            "NPN",
            "",
            1.8,
            "AVCX1=0.05 MAXEXP=0.1",
            [-6.273847715935601e-5, 2.3762228343031284e-6],
        ),
        (
            12,
            "NPN",
            "TRISE=20",
            1.8,
            "AVC1=0.05 AVC2=0.3 TAVC=-0.06",
            [-0.00021240015017831354, 1.7926908619094553e-5],
        ),
        (
            12,
            "NPN",
            "",
            0.68,
            "GAMM=2 QCO=1p MAXEXP=0.1",
            [-5.715927094855637e-5, -2.0270641147215227e-6],
        ),
    ] {
        let polarity = if kind == "PNP" { -1.0 } else { 1.0 };
        let substrate = if level == 12 { " 0" } else { "" };
        let deck = format!(
            "VBIC13 avalanche oracle\nVc c 0 {}\nVb b 0 {}\nQ1 c b 0{substrate} vm SW_ET=0 {control}\n\
             .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 AVCX2=0.3 {parameters} GMIN=1e-6 TNOM=27)\n.temp 27\n.end\n",
            polarity * vc,
            polarity * 0.7,
        );
        let result = op_result(&deck);
        for (branch, expected) in ["vc", "vb"].into_iter().zip(expected) {
            let actual = result.branch_current_named(branch).unwrap();
            assert!(
                (actual - expected).abs() < 2e-7 * expected.abs(),
                "{level} {kind} {control} {parameters} {branch}: {actual:e} != {expected:e}"
            );
        }
    }
}

#[test]
fn vbic13_junction_leakage_obeys_kcl_without_generating_heat() {
    for level in [11, 12] {
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            let deck = format!(
                "VBIC13 leakage and heat\nVc c 0 {}\nVb b 0 {}\nQ1 c b 0{substrate} vm M=3\n\
                 .model vm {kind}(LEVEL={level} IS=1e-40 IBEI=0 IBCI=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=1m RTH=1000)\n.end\n",
                1.2 * polarity,
                0.5 * polarity,
            );
            let result = op_result(&deck);
            // The authored numerical parallels are BE, BEX, BC, BEP,
            // BXCX, and (for LEVEL=12) BCP. The collapsed nodes make
            // their independent KCL sum an elementary resistor network.
            let collector = if level == 12 { 3.3 } else { 2.1 };
            assert!(
                (result.branch_current_named("vc").unwrap() + polarity * 0.003 * collector).abs()
                    < 1e-11,
                "level={level} kind={kind} Ic={:?} Ib={:?}",
                result.branch_current_named("vc"),
                result.branch_current_named("vb")
            );
            assert!(
                (result.branch_current_named("vb").unwrap() - polarity * 0.003 * 1.1).abs() < 1e-11
            );
            assert!(
                voltage(&result, "q1.__rth.internal").abs() < 1e-10,
                "numerical junction conductance must not heat the device"
            );
        }
    }
}

#[test]
fn vbic13_temperature_dependent_self_heating_matches_xyce710() {
    // Live Xyce 7.10, 2026-09-08, at 27 C plus instance TRISE=20 K.
    for level in [11, 12] {
        for (kind, sign) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            for (coefficient, current) in [(0.0, -0.000192271208), (0.05, -0.000194956546)] {
                let deck = format!(
                    "VBIC13 TCRTH oracle\nVc c 0 {}\nVb b 0 {}\nQ1 c b 0{substrate} vm TRISE=20\n\
                     .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0 RTH=1000 TCRTH={coefficient} TNOM=27)\n.temp 27\n.end\n",
                    1.2 * sign,
                    0.7 * sign,
                );
                assert_rel_close(
                    "TCRTH collector current",
                    branch_current(&deck, "vc"),
                    sign * current,
                    1e-5,
                );
            }
        }
    }
}

#[test]
fn vbic13_clips_the_combined_ambient_offset_and_external_thermal_node() {
    for level in [11, 12] {
        let substrate = if level == 12 { " 0" } else { "" };
        for (rise, offset, expected_c) in [
            (-78.0, 0.0, -50.0 + (-2.0_f64).exp()),
            (74.0, 0.0, 100.0 - (-2.0_f64).exp()),
            (-1000.0, 0.0, -50.0),
            (1000.0, 0.0, 100.0),
            (400.0, -400.0, 27.0),
        ] {
            let deck = |thermal, offset, temperature| {
                format!(
                    "VBIC13 effective temperature\nVc c 0 1.2\nVb b 0 0.5\nVth th 0 {thermal}\n\
                 Q1 c b 0{substrate} th vm SW_ET=0 TRISE={offset}\n\
                 .model vm NPN(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0 RTH=1000 TCRTH=0.001 TMINCLIP=-50 TMAXCLIP=100 TNOM=27)\n.temp {temperature}\n.end\n"
                )
            };
            let result = op_result(&deck(rise, offset, 27.0));
            // Use wide limits so the already mapped comparison temperature
            // passes through unchanged, including the exponential tails.
            let equivalent = deck(0.0, 0.0, expected_c)
                .replace("TMINCLIP=-50 TMAXCLIP=100", "TMINCLIP=-100 TMAXCLIP=500");
            for branch in ["vc", "vb"] {
                let actual = result.branch_current_named(branch).unwrap();
                let expected = branch_current(&equivalent, branch);
                assert!(
                    (actual - expected).abs() < 1e-12 + 1e-7 * expected.abs(),
                    "LEVEL={level} rise={rise} offset={offset} {branch}: {actual} vs {expected}"
                );
            }
            assert_rel_close(
                "thermal sink current",
                result.branch_current_named("vth").unwrap(),
                -rise / (1000.0 * (1.0 + 0.001 * (expected_c - 27.0))),
                1e-9,
            );
        }
    }
}

#[test]
fn vbic13_temperature_and_avalanche_parameters_reject_invalid_values_and_model_families() {
    for level in [11, 12] {
        for parameter in [
            "TMINCLIP=-251",
            "TMINCLIP=28",
            "TMAXCLIP=26",
            "TMAXCLIP=1001",
            "TCRTH={1/0}",
            "TMINCLIP=\"cold\"",
            "AVCX1=-0.1",
            "AVCX2=-0.1",
            "TAVCX={1/0}",
            "MCX=0",
            "MCX=1.01",
            "MAXEXP=0",
            "TCVEF=\"warm\"",
            "TCVER={1/0}",
        ] {
            let error = build(&op_deck(&format!(
                ".model qmod NPN(LEVEL={level} {parameter})"
            )))
            .expect_err("invalid thermal model parameter must fail closed");
            assert!(
                error.contains(parameter.split('=').next().unwrap()),
                "{error}"
            );
        }
        for parameters in [
            "TMINCLIP=-250 TMAXCLIP=1000",
            "TMINCLIP=27 TMAXCLIP=27",
            "TCRTH=-0.05",
            "AVCX1=0 AVCX2=0 TAVCX=-0.05 MCX=1 MAXEXP=0.1",
            "TCVEF=0.05 TCVER=-0.1",
        ] {
            build(&op_deck(&format!(
                ".model qmod NPN(LEVEL={level} {parameters})"
            )))
            .unwrap();
        }
    }
    for level in [1, 4, 9, 13] {
        for parameter in [
            "TCRTH=0",
            "TMINCLIP=-100",
            "TMAXCLIP=500",
            "AVCX1=0",
            "AVCX2=0",
            "TAVCX=0",
            "MCX=0.33",
            "MAXEXP=1e22",
            "TCVEF=0",
            "TCVER=0",
        ] {
            let error = build(&op_deck(&format!(
                ".model qmod NPN(LEVEL={level} {parameter})"
            )))
            .expect_err("unimplemented thermal parameter must not be silently ignored");
            assert!(error.contains("LEVEL=11 or LEVEL=12"), "{error}");
        }
    }
}

#[test]
fn native_bjt_vector_parameters_are_rejected_instead_of_ignored() {
    for level in [1, 4, 9, 11, 12] {
        for parameter in [
            "AVCX1=[0.1 0.2]",
            "LEVEL=[11 12]",
            "TCRTH=[{1+2} 0]",
            "IS=[\"small\" \"large\"]",
        ] {
            let error = build(&op_deck(&format!(
                ".model qmod NPN(LEVEL={level} {parameter})"
            )))
            .expect_err("native BJT vectors cannot be silently ignored");
            assert!(error.contains("vector parameters"), "{error}");
        }
    }
}

#[test]
fn vbic_temperature_offset_aliases_and_switch_validation_are_effective() {
    let deck = "VBIC offset aliases\nVc c 0 1.2\nVb b 0 0.7\nQ1 c b 0 vm SW_ET=0\n\
        .model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCI=0 RBI=0)\n.temp 27\n.end\n";
    let warmer = branch_current(&deck.replace(".temp 27", ".temp 47"), "vc");
    for alias in ["TRISE", "DTA", "DTEMP"] {
        let variant = deck.replace("SW_ET=0", &format!("SW_ET=0 {alias}=20"));
        assert_rel_close(alias, branch_current(&variant, "vc"), warmer, 1e-10);
    }
    for control in [
        "SW_ET=0.5",
        "SW_ET=2",
        "SW_NOISE=-1",
        "TRISE=1 DTA=2",
        "DTEMP=1 TRISE=2",
    ] {
        let error =
            build(&deck.replace("SW_ET=0", control)).expect_err("invalid control must fail closed");
        assert!(error.contains("BJT"), "{error}");
    }
}

#[test]
fn vbic_noise_switch_removes_device_noise_without_changing_the_operating_point() {
    for level in [4, 11, 12] {
        let deck = format!(
            "VBIC noise switch\nVcc supply 0 3.3\nVb b 0 0.7 AC 1\nRc supply c 1k\nQ1 c b 0 vm SW_NOISE=1\n.model vm NPN(LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=10 RCI=20 RBX=10 RBI=40 RE=1 KFN=1e-14)\n.end\n"
        );
        let quiet = deck.replace("SW_NOISE=1", "SW_NOISE=0");
        assert_eq!(
            op_result(&deck).node_voltages,
            op_result(&quiet).node_voltages
        );
        for (text, enabled) in [(&deck, true), (&quiet, false)] {
            let points = Engine::default()
                .run_noise_named_with_input_source(
                    &Netlist::parse(text).unwrap(),
                    "c",
                    None,
                    "Vb",
                    &[1e3],
                    300.15,
                )
                .unwrap();
            let device = points[0]
                .contributions
                .iter()
                .filter(|entry| entry.identity.device.eq_ignore_ascii_case("Q1"));
            assert_eq!(
                device.clone().any(|entry| entry.output_contribution > 0.0),
                enabled
            );
            if !enabled {
                assert_eq!(device.count(), 0);
            }
        }
    }
}

#[test]
fn three_terminal_vbic_fourth_pin_is_thermal_in_every_native_dialect() {
    let deck = "VBIC three-terminal external temperature\n\
        Vc c 0 1.2\nVb b 0 0.7\nVdt dt 0 20\nQ1 c b 0 dt vm SW_ET=0\n\
        .model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18\n\
        + RCI=0 RBI=0 RTH=100 TNOM=27)\n.temp 27\n.end\n";
    let warmer = deck
        .replace("Q1 c b 0 dt vm", "Q1 c b 0 vm")
        .replace(".temp 27", ".temp 47");
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let config = SimulationConfig::default().with_spice_dialect(dialect);
        assert_rel_close(
            &format!("{dialect:?} external temperature collector current"),
            branch_current_with_config(deck, "vc", config.clone()),
            branch_current_with_config(&warmer, "vc", config),
            1e-9,
        );
    }
    let extra = deck.replace("Q1 c b 0 dt vm", "Q1 c b 0 0 dt vm");
    let error = build(&extra).expect_err("LEVEL=11 must reject a fifth terminal");
    assert!(
        error.contains("LEVEL=11") && error.contains("fifth terminal"),
        "{error}"
    );
}

#[test]
fn three_terminal_vbic_ignores_substrate_parameters_in_dc_ac_and_thermal_power() {
    // Xyce vbic_1p3.va compiles Ibcp/Iccp/Irs/Qbcp and their power terms
    // only under fourTerminal. Even nonzero model parameters must not create
    // an SI node or perturb the three-terminal electrical/thermal solution.
    for (kind, sign) in [("NPN", 1.0), ("PNP", -1.0)] {
        for rbp in [0.0, 10.0] {
            let deck = |substrate: &str| {
                format!(
                    "three-terminal VBIC substrate isolation\n\
                 Vc c 0 {}\nVb b 0 {} AC 1\nQ1 c b 0 vm\n\
                 .model vm {kind}(LEVEL=11 IS=1e-14 IBEI=1e-16 IBCI=1e-16\n\
                 + RCX=10 RCI=20 RBX=10 RBI=40 RE=1 RBP={rbp} ISP=1e-15 IKP=1u\n\
                 + CJE=10p CJC=5p CJEP=3p TF=10n TR=2n SELFT=1 RTH=300 CTH=1p\n\
                 + {substrate})\n.end\n",
                    0.5 * sign,
                    0.65 * sign
                )
            };
            let baseline = deck("RS=0 CJCP=0 CCSO=0 IBCIP=0 IBCNP=0");
            let unused = deck("RS=123 CJCP=1u CCSO=2u IBCIP=1m IBCNP=2m");
            let expected = op_result(&baseline);
            let actual = op_result(&unused);
            assert_eq!(expected.node_names, actual.node_names);
            assert!(!actual.node_names.iter().any(|name| name.contains(".__si.")));
            assert_eq!(
                actual.node_names.iter().any(|name| name.contains(".__bp.")),
                rbp > 0.0,
                "ignored substrate diodes must not prevent BP collapse"
            );
            assert!(voltage(&actual, "q1.__rth.internal") > 1e-4);
            assert_eq!(expected.node_voltages, actual.node_voltages);
            assert_eq!(expected.branch_currents, actual.branch_currents);
            assert_eq!(
                ac_branch_current(&baseline, "vb", 1e6),
                ac_branch_current(&unused, "vb", 1e6),
            );
        }
    }
}

#[test]
fn ngspice_vbic_level9_uses_vbic_internal_topology() {
    let deck = op_deck(
        ".model qmod NPN (LEVEL=9 IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2 RE=1)",
    );
    let circuit = build(&deck).expect("ngspice VBIC LEVEL=9 alias circuit builds natively");
    for node in [
        "q1.__cx.internal",
        "q1.__ci.internal",
        "q1.__bx.internal",
        "q1.__bi.internal",
        "q1.__ei.internal",
    ] {
        assert!(
            circuit.get_node_by_name(node).is_some(),
            "LEVEL=9 should allocate VBIC internal node {node}"
        );
    }
}

#[test]
fn xyce_vbic_levels_11_12_and_13_match_xyce710_dc_op_oracle() {
    // XyceNF 7.10, same decks with `.print dc V(c) V(b) V(e) I(vc) I(vb) I(ve)`.
    for level in [11, 12, 13] {
        let deck = xyce_vbic_oracle_deck(level);
        assert_rel_close(
            &format!("LEVEL={level} I(VC)"),
            branch_current(&deck, "vc"),
            -7.468_978_34e-4,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} I(VB)"),
            branch_current(&deck, "vb"),
            -1.213_920_45e-5,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} I(VE)"),
            branch_current(&deck, "ve"),
            7.590_370_38e-4,
            1.0e-4,
        );
    }
}

#[test]
fn inert_vbic13_zero_breakdown_params_run_on_native_vbic_levels() {
    for level in [4, 9, 11, 12, 13] {
        let base = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2)"
        ));
        let inert = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 RCX=1 RCI=2 RBX=1 RBI=2 {})",
            inert_vbic13_params()
        ));

        run(&inert).unwrap_or_else(|err| {
            panic!("inert VBIC13 zero-breakdown params should run on LEVEL={level}: {err}")
        });
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 I(VC)"),
            branch_current(&inert, "vc"),
            branch_current(&base, "vc"),
            1.0e-10,
        );
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 I(VB)"),
            branch_current(&inert, "vb"),
            branch_current(&base, "vb"),
            1.0e-10,
        );
    }

    for level in [11, 12, 13] {
        let deck = xyce_vbic_oracle_deck_with_extra_params(level, inert_vbic13_params());
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 Xyce-oracle I(VC)"),
            branch_current(&deck, "vc"),
            -7.468_978_34e-4,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 Xyce-oracle I(VB)"),
            branch_current(&deck, "vb"),
            -1.213_920_45e-5,
            1.0e-4,
        );
        assert_rel_close(
            &format!("LEVEL={level} inert VBIC13 Xyce-oracle I(VE)"),
            branch_current(&deck, "ve"),
            7.590_370_38e-4,
            1.0e-4,
        );
    }
}

#[test]
fn active_vbic13_wbe1_reverse_be_breakdown_matches_xyce_ngspice_equation() {
    let expected = vbic13_wbe1_reverse_be_source_current(VBIC13_TEST_VBE);
    for level in [4, 11, 12, 13] {
        let deck = active_vbic13_wbe1_breakdown_deck(level, VBIC13_TEST_VBE);
        assert_rel_close(
            &format!("LEVEL={level} active VBIC13 WBE=1 I(VB)"),
            branch_current(&deck, "vb"),
            expected,
            2.0e-5,
        );
    }
}

#[test]
fn vbic13_ebbe_parameter_is_recomputed_from_vbbe_nbbe() {
    let base = active_vbic13_wbe1_breakdown_deck(11, VBIC13_TEST_VBE);
    let with_legacy_ebbe =
        active_vbic13_wbe1_breakdown_deck_with_extra(11, VBIC13_TEST_VBE, "EBBE=0.25");

    assert_rel_close(
        "finite user EBBE should not perturb recomputed VBIC13 breakdown",
        branch_current(&with_legacy_ebbe, "vb"),
        branch_current(&base, "vb"),
        1.0e-12,
    );
}

fn active_vbic13_wbe_split_breakdown_deck(wbe: f64, rbx: f64, rbi: f64, vbase: f64) -> String {
    format!(
        "* VBIC13 reverse B-E breakdown, Xyce-primary WBE split path\n\
         .options gmin=0 reltol=1e-10 abstol=1e-15 vntol=1e-12\n\
         vc c 0 dc 0\n\
         vb b 0 dc {vbase}\n\
         q1 c b 0 qmod\n\
         .model qmod NPN (LEVEL=11 IS=1e-30 ISRR=0 IBEI=0 IBEN=0 IBCI=0 IBCN=0 \
         RCX=0 RCI=0 RBX={rbx} RBI={rbi} RE=0 RBP=0 WBE={wbe} \
         VBBE={VBIC13_TEST_VBBE} NBBE={VBIC13_TEST_NBBE} IBBE={VBIC13_TEST_IBBE})\n\
         .op\n\
         .end\n"
    )
}

fn vbic13_reverse_be_breakdown_source_current(vbe: f64) -> f64 {
    vbic13_wbe1_reverse_be_source_current(vbe)
}

fn solve_xyce_split_vbi(vbase: f64, wbe: f64, rbx: f64, rbi: f64) -> f64 {
    let resistance = rbx + wbe * rbi;
    let mut lo = vbase;
    let mut hi = vbase + 0.5;
    for _ in 0..160 {
        let mid = 0.5 * (lo + hi);
        let residual = mid - vbase - resistance * vbic13_reverse_be_breakdown_source_current(mid);
        if residual > 0.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

fn ordinary_vbic_wbe_split_deck(wbe: f64, rbx: f64, rbi: f64, vbase: f64) -> String {
    format!(
        "* VBIC ordinary B-E current, Xyce WBE split path\n\
         .options gmin=0 reltol=1e-10 abstol=1e-15 vntol=1e-12\n\
         vc c 0 dc 0\n\
         vb b 0 dc {vbase}\n\
         q1 c b 0 qmod\n\
         .model qmod NPN (LEVEL=11 IS=1e-30 ISRR=0 IBEI=1e-18 IBEN=0 IBCI=0 IBCN=0 \
         RCX=0 RCI=0 RBX={rbx} RBI={rbi} RE=0 RBP=0 WBE={wbe} VBBE=0)\n\
         .op\n\
         .end\n"
    )
}

fn vbic_ordinary_ibe_current(vbe: f64) -> (f64, f64) {
    let vt = 1.380662e-23 * rspice_core::constants::TEMP_REFERENCE / 1.602189e-19;
    let exp_vbe = (vbe / vt).exp();
    (1.0e-18 * (exp_vbe - 1.0), 1.0e-18 * exp_vbe / vt)
}

fn solve_xyce_ordinary_wbe_split(vbase: f64, wbe: f64, rbx: f64, rbi: f64) -> (f64, f64, f64) {
    let mut vbx = vbase;
    let mut vbi = vbase;
    for _ in 0..80 {
        let (ibe_bx, gbe_bx) = vbic_ordinary_ibe_current(vbx);
        let (ibe_bi, gbe_bi) = vbic_ordinary_ibe_current(vbi);
        let f1 = (vbase - vbx) / rbx - wbe * ibe_bi - (1.0 - wbe) * ibe_bx;
        let f2 = (vbx - vbi) / rbi - wbe * ibe_bi;
        if f1.abs().max(f2.abs()) < 1.0e-18 {
            break;
        }

        let a = -1.0 / rbx - (1.0 - wbe) * gbe_bx;
        let b = -wbe * gbe_bi;
        let c = 1.0 / rbi;
        let d = -1.0 / rbi - wbe * gbe_bi;
        let det = a * d - b * c;
        assert!(
            det.abs() > 1.0e-30,
            "ordinary WBE split oracle Jacobian singular"
        );
        let delta_vbx = (-f1 * d + b * f2) / det;
        let delta_vbi = (c * f1 - a * f2) / det;
        vbx += delta_vbx.clamp(-0.05, 0.05);
        vbi += delta_vbi.clamp(-0.05, 0.05);
    }

    let (ibe_bx, _) = vbic_ordinary_ibe_current(vbx);
    let (ibe_bi, _) = vbic_ordinary_ibe_current(vbi);
    let total = wbe * ibe_bi + (1.0 - wbe) * ibe_bx;
    (vbx, vbi, total)
}

#[test]
fn active_vbic13_wbe_split_collapsed_topology_preserves_total_breakdown_current() {
    let deck = active_vbic13_wbe_split_breakdown_deck(0.5, 0.0, 0.0, VBIC13_TEST_VBE);

    assert_rel_close(
        "active VBIC13 split WBE collapsed I(VB)",
        branch_current(&deck, "vb"),
        vbic13_wbe1_reverse_be_source_current(VBIC13_TEST_VBE),
        2.0e-5,
    );
}

#[test]
fn ordinary_vbic_wbe_split_uses_vbx_for_extrinsic_be_current() {
    let vbase = 0.62;
    let wbe = 0.25;
    let rbx = 1.0e6;
    let rbi = 1.0e6;
    let deck = ordinary_vbic_wbe_split_deck(wbe, rbx, rbi, vbase);
    let result = op_result(&deck);
    let (expected_vbx, expected_vbi, expected_current) =
        solve_xyce_ordinary_wbe_split(vbase, wbe, rbx, rbi);

    assert_rel_close(
        "ordinary VBIC WBE split I(VB)",
        result
            .branch_current_named("vb")
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names)),
        -expected_current,
        2.0e-4,
    );
    assert_rel_close(
        "ordinary VBIC WBE split bx voltage",
        voltage(&result, "q1.__bx.internal"),
        expected_vbx,
        2.0e-5,
    );
    assert_rel_close(
        "ordinary VBIC WBE split bi voltage",
        voltage(&result, "q1.__bi.internal"),
        expected_vbi,
        2.0e-5,
    );
    assert!(
        (expected_vbx - expected_vbi).abs() > 1.0e-3,
        "test must separate bx from bi enough to prove ordinary WBE split placement"
    );
}

#[test]
fn ordinary_vbic_wbe_uses_literal_out_of_range_value_like_xyce() {
    let vbase = 0.62;
    let wbe = 1.25;
    let rbx = 1.0e6;
    let rbi = 1.0e6;
    let deck = ordinary_vbic_wbe_split_deck(wbe, rbx, rbi, vbase);
    let result = op_result(&deck);
    let (expected_vbx, expected_vbi, expected_current) =
        solve_xyce_ordinary_wbe_split(vbase, wbe, rbx, rbi);

    assert_rel_close(
        "ordinary VBIC literal out-of-range WBE I(VB)",
        result
            .branch_current_named("vb")
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names)),
        -expected_current,
        2.0e-4,
    );
    assert_rel_close(
        "ordinary VBIC literal out-of-range WBE bx voltage",
        voltage(&result, "q1.__bx.internal"),
        expected_vbx,
        2.0e-5,
    );
    assert_rel_close(
        "ordinary VBIC literal out-of-range WBE bi voltage",
        voltage(&result, "q1.__bi.internal"),
        expected_vbi,
        2.0e-5,
    );
}

#[test]
fn active_vbic13_wbe_split_uses_bx_to_ei_injection_controlled_by_vbi() {
    let vbase = -2.5;
    let wbe = 0.25;
    let rbx = 1.0e6;
    let rbi = 1.0e6;
    let deck = active_vbic13_wbe_split_breakdown_deck(wbe, rbx, rbi, vbase);
    let result = op_result(&deck);
    let expected_vbi = solve_xyce_split_vbi(vbase, wbe, rbx, rbi);
    let expected_current = vbic13_reverse_be_breakdown_source_current(expected_vbi);
    let expected_vbx = vbase + rbx * expected_current;

    assert_rel_close(
        "active VBIC13 split WBE non-collapsed I(VB)",
        result
            .branch_current_named("vb")
            .unwrap_or_else(|| panic!("missing vb branch in {:?}", result.branch_names)),
        expected_current,
        2.0e-4,
    );
    assert_rel_close(
        "active VBIC13 split WBE bx voltage",
        voltage(&result, "q1.__bx.internal"),
        expected_vbx,
        2.0e-5,
    );
    assert_rel_close(
        "active VBIC13 split WBE bi control voltage",
        voltage(&result, "q1.__bi.internal"),
        expected_vbi,
        2.0e-5,
    );
    assert!(
        (voltage(&result, "q1.__bi.internal") - voltage(&result, "q1.__bx.internal")).abs()
            > 1.0e-3,
        "test must separate bx from bi enough to prove control-node placement"
    );
}

#[test]
fn active_vbic13_temperature_coefficients_follow_xyce_ngspice_formula() {
    let temp_c = 75.0;
    let temp_k = temp_c + 273.15;
    let vbe = -2.5;
    let tvbbe1 = 1.0e-3;
    let tvbbe2 = 1.0e-6;
    let tnbbe = 5.0e-4;
    let deck = active_vbic13_wbe1_breakdown_deck_with_options(
        11,
        vbe,
        &format!("temp={temp_c}"),
        &format!("TVBBE1={tvbbe1} TVBBE2={tvbbe2} TNBBE={tnbbe}"),
    );

    assert_rel_close(
        "active VBIC13 temperature-scaled I(VB)",
        branch_current(&deck, "vb"),
        vbic13_wbe1_reverse_be_source_current_at(vbe, temp_k, tvbbe1, tvbbe2, tnbbe),
        2.0e-5,
    );
}

#[test]
fn vbic13_specific_parameters_reject_fail_closed_when_not_inert_or_not_native_vbic() {
    for level_prefix in ["", "LEVEL=1"] {
        let separator = if level_prefix.is_empty() { "" } else { " " };
        let deck = op_deck(&format!(
            ".model qmod NPN ({level_prefix}{separator}IS=1e-16 IBEI=1e-18 IBCI=1e-18 {})",
            inert_vbic13_params()
        ));
        let message =
            run(&deck).expect_err("VBIC13-only params must not be accepted on legacy GP cards");
        assert!(
            message.contains("VBIC13") || message.contains("VBIC 1.3"),
            "error names the VBIC13 family: {message}"
        );
        for param in ["VBBE", "NBBE", "IBBE", "TVBBE1", "TVBBE2", "TNBBE", "EBBE"] {
            assert!(
                message.contains(param),
                "error lists unsupported VBIC13 parameter {param}: {message}"
            );
        }
        assert!(
            message.contains("not implemented") || message.contains("unsupported"),
            "error explains fail-closed routing: {message}"
        );
        assert!(
            !message.contains("selects native VBIC"),
            "error should not imply legacy/no-level cards selected native VBIC: {message}"
        );
    }

    for (param, value) in [
        ("VBBE", "-1.0"),
        ("NBBE", "0"),
        ("NBBE", "-1"),
        ("IBBE", "0"),
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 {param}={value})"
        ));
        let message =
            run(&deck).expect_err("invalid VBIC13 reverse B-E breakdown params must reject");
        assert!(
            message.contains("VBIC13") || message.contains("VBIC 1.3"),
            "error names the VBIC13 family: {message}"
        );
        assert!(
            message.contains(param),
            "error names the active unsupported parameter {param}: {message}"
        );
        assert!(
            message.contains("breakdown") || message.contains("not implemented"),
            "error explains active physics is unavailable: {message}"
        );
    }

    let deck = op_deck(&format!(
        ".model qmod NPN (LEVEL=234 IS=1e-16 IBEI=1e-18 IBCI=1e-18 {})",
        inert_vbic13_params()
    ));
    let message = run(&deck).expect_err("VBIC13 params must reject on unsupported BJT levels");
    assert!(
        message.contains("LEVEL=4")
            && message.contains("LEVEL=9")
            && message.contains("LEVEL=11")
            && message.contains("LEVEL=12")
            && message.contains("LEVEL=13"),
        "error should name the native VBIC-only acceptance scope: {message}"
    );
}

#[test]
fn vbic13_specific_parameters_reject_when_not_finite_numeric_literals() {
    for extra in [
        "VBBE={VBREAK}",
        "IBBE={IBBE_SCALE}",
        "VBBE=\"active\"",
        "NBBE=\"1\"",
        "VBBE={1/0}",
        "VBBE=2 NBBE=5 IBBE=1e-9 WBE={SPLIT}",
        "VBBE=2 NBBE=5 IBBE=1e-9 WBE=\"0.5\"",
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 {extra})"
        ));
        let message =
            run(&deck).expect_err("VBIC13 params must be finite numeric literals to be accepted");
        assert!(
            message.contains("VBIC13") || message.contains("VBIC 1.3"),
            "error names the VBIC13 family: {message}"
        );
        assert!(
            message.contains("finite")
                || message.contains("numeric")
                || message.contains("unresolved"),
            "error explains finite numeric requirement: {message}"
        );
    }
}

#[test]
fn vbic13_inert_params_require_exact_native_vbic_level_selectors() {
    for level in [
        "4.0000000005",
        "11.0000000005",
        "12.0000000005",
        "13.0000000005",
    ] {
        let deck = op_deck(&format!(
            ".model qmod NPN (LEVEL={level} IS=1e-16 IBEI=1e-18 IBCI=1e-18 {})",
            inert_vbic13_params()
        ));
        let message =
            run(&deck).expect_err("near-integer BJT levels must not select native VBIC routing");
        assert!(
            message.contains("VBIC13") || message.contains("LEVEL"),
            "error explains rejected selector or VBIC13 routing: {message}"
        );
    }
}

#[test]
fn legacy_bjt_levels_still_run() {
    for model_line in [
        ".model qmod NPN (IS=1e-16 BF=100)",
        ".model qmod NPN (LEVEL=0 IS=1e-16 BF=100)",
        ".model qmod NPN (LEVEL=1 IS=1e-16 BF=100)",
    ] {
        let deck = op_deck(model_line);
        run(&deck).unwrap_or_else(|err| panic!("{model_line} must remain legacy GP: {err}"));
    }
}

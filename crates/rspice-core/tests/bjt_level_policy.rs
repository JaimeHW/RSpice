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

#[test]
fn vbic_nonpositive_activation_energies_match_xyce710_dc_and_ac() {
    use rspice_core::Complex64;
    // Xyce 7.10, separately captured DC and 100 MHz thermal-drive AC.
    // All eight nonpositive energies change both saturation-current and
    // depletion-potential temperature laws, including the substrate network.
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            ..Default::default()
        },
        ..Default::default()
    });
    for (level, energy, dc, ac) in [
        (
            11,
            0.0,
            [-1.8760646788795922e-05, -2.1527723643669423e-07, 0.0],
            [
                Complex64::new(7.516482992238466e-07, -7.827908174974698e-06),
                Complex64::new(2.5718191767056676e-07, 1.1571599175275925e-05),
                Complex64::new(0.0, 0.0),
            ],
        ),
        (
            12,
            0.0,
            [
                -1.8759381199128295e-05,
                -5.974425036580707e-06,
                5.759162012220747e-06,
            ],
            [
                Complex64::new(7.475710679968615e-07, -7.2352565019688054e-06),
                Complex64::new(5.679107136074598e-07, 1.1568126579812277e-05),
                Complex64::new(-3.0678023251495346e-07, -5.895892422355515e-07),
            ],
        ),
        (
            11,
            -0.1,
            [-1.3203668688194347e-05, -1.5289766582438167e-07, 0.0],
            [
                Complex64::new(5.763336343616682e-07, -8.370708593818644e-06),
                Complex64::new(2.6448041761346957e-07, 1.2429299779533612e-05),
                Complex64::new(0.0, 0.0),
            ],
        ),
        (
            12,
            -0.1,
            [
                -1.3203041813337315e-05,
                -4.205786531991028e-06,
                4.052895950577105e-06,
            ],
            [
                Complex64::new(5.723237566711971e-07, -7.6979104375324e-06),
                Complex64::new(5.244002563240605e-07, 1.2426635278632646e-05),
                Complex64::new(-2.559863313225597e-07, -6.704596659558985e-07),
            ],
        ),
    ] {
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " s" } else { "" };
            let netlist = Netlist::parse(&format!(
                "* VBIC activation energies\nVc c 0 {}\nVb b 0 {}\nVs s 0 {}\nVth th 0 DC 30 AC 1\nQ1 c b 0{substrate} th vm SW_ET=0 M=3\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=1e-18 IBEN=1e-14 IBCI=1e-18 IBCN=1e-14 ISP=1e-15 IBEIP=1e-18 IBENP=1e-14 IBCIP=1e-16 IBCNP=1e-14 RCX=1 RCI=1 RBX=1 RBI=5 RE=1 RBP=5 RS=1 RTH=1000 CTH=1p CJE=1p CJC=2p CJEP=1p CJCP=1p TF=1n TR=2n TD=1n GMIN=0 TNOM=27 EA={energy} EAIE={energy} EAIC={energy} EAIS={energy} EANE={energy} EANC={energy} EANS={energy} EAP={energy})\n.temp 27\n.options gmin=0\n.end\n",
                 p*0.1, p*0.7, p*(-0.4)
            )).unwrap();
            let operating = engine.run_dc_op(&netlist).unwrap();
            let small_signal = engine.run_ac(&netlist, &[1e8]).unwrap();
            for (index, name) in ["Vc", "Vb", "Vs"].into_iter().enumerate() {
                let actual = operating.branch_current_named(name).unwrap();
                assert!(
                    (actual - p * dc[index]).abs() < 2e-6 * dc[index].abs().max(1e-12),
                    "{level} {kind} EA={energy} I({name})={actual:e}"
                );
                let column = small_signal[0]
                    .branch_names
                    .iter()
                    .position(|branch| branch.eq_ignore_ascii_case(name))
                    .unwrap();
                let actual = small_signal[0].currents[column];
                assert!(
                    (actual - p * ac[index]).norm() < 2e-6 * ac[index].norm().max(1e-12),
                    "{level} {kind} EA={energy} AC I({name})={actual:?}"
                );
            }
        }
    }
}

#[test]
fn vbic13_pnjmaxi_matches_xyce_and_model_overrides_global_option() {
    // Xyce 7.10 LEVEL=11/12, all five active series resistors = 1 ohm.
    for level in [11, 12] {
        let substrate = if level == 12 { " 0" } else { "" };
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            for (model, option, expected) in [
                ("PNJMAXI=1u", "", -8.903669004624935e-6),
                ("", ".options PNJMAXI=1u", -8.903669004624935e-6),
                ("", ".options DEVICE PNJMAXI=1u", -8.903669004624935e-6),
                ("PNJMAXI=1", ".options PNJMAXI=1u", -0.002462218732912948),
            ] {
                let deck = format!(
                    "PNJMAXI precedence\nVc c 0 {p}\nVb b 0 {}\nQ1 c b 0{substrate} vm SW_ET=0\n\
                     .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 {model} GMIN=0 TNOM=27)\n\
                     .temp 27\n.options gmin=0\n{option}\n.op\n.end\n",
                    0.8 * p,
                );
                assert_rel_close(
                    &format!("LEVEL={level} {kind} {model} {option}"),
                    branch_current(&deck, "vc"),
                    p * expected,
                    1e-7,
                );
            }
        }
    }
}

#[test]
fn pnjmaxi_rejects_invalid_values_and_unsupported_model_families() {
    for value in ["0", "-1", "{1/0}", "\"invalid\""] {
        for scope in ["", "DEVICE "] {
            let deck = format!("PNJMAXI validation\n.options {scope}PNJMAXI={value}\n.end\n");
            assert!(Netlist::parse(&deck).is_err(), "{deck}");
        }
        let deck = op_deck(&format!(".model qmod NPN(LEVEL=11 PNJMAXI={value})"));
        assert!(run(&deck).is_err(), "{deck}");
    }
    for level in [1, 4, 9, 13] {
        let deck = op_deck(&format!(".model qmod NPN(LEVEL={level} PNJMAXI=1u)"));
        let error = run(&deck).expect_err("PNJMAXI must not silently affect another model family");
        assert!(error.contains("PNJMAXI"), "{error}");
    }
    let mut netlist = Netlist::parse(&op_deck(".model qmod NPN(LEVEL=11)")).unwrap();
    for value in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        netlist.options.device_pnjmaxi = Some(value);
        assert!(Engine::default().build_circuit(&netlist).is_err());
    }
}

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
fn legacy_itf_scaling_matches_ac_and_transient_charge_conservation() {
    use rspice_core::numerics::integration::IntegrationMethod;
    let vt = rspice_core::analysis::temperature::thermal_voltage(300.15);
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            voltage_reltol: 1e-9,
            current_abstol: 1e-16,
            voltage_abstol: 1e-12,
            ..Default::default()
        },
        integration_method: IntegrationMethod::BackwardEuler,
        // This checks each backward-Euler charge balance, independent of LTE
        // step selection; a prescribed grid keeps the regression inexpensive.
        locked_time_grid: Some(std::sync::Arc::new(
            (0..=20).map(|index| f64::from(index) * 1e-9).collect(),
        )),
        ..Default::default()
    });
    let charge = |vbe: f64| {
        let forward = 1e-16 * (vbe / vt).exp_m1();
        let fraction = forward / (forward + 1e-4);
        1e-9 * forward * (1.0 + 3.0 * ((vbe - 0.72) / 14.4).exp() * fraction.powi(2))
    };
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for (devices, reverse_scale) in [
            ("Q1 c b 0 qm M=3", 3.0),
            ("Q1 c b 0 qm AREA=3", 9.0),
            ("Q1 c b 0 qm AREA=1.5 M=2", 4.5),
            ("Q1 c b 0 qm\nQ2 c b 0 qm\nQ3 c b 0 qm", 3.0),
        ] {
            let netlist = Netlist::parse(&format!(
                "* Legacy ITF charge conservation\nVc c 0 {}\nVb b 0 PWL(0 {} 20n {}) AC 1 DC {}\n{devices}\n\
                 .model qm {kind}(LEVEL=1 IS=1e-16 BF=100 TF=1n XTF=3 VTF=10 ITF=100u)\n.options gmin=0\n.end\n",
                p*0.72, p*0.65, p*0.7, p*0.65
            )).unwrap();
            let ac = engine.run_ac(&netlist, &[1e8]).unwrap();
            let vb = ac[0]
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("Vb"))
                .unwrap();
            let forward = 1e-16 * (0.65 / vt).exp_m1();
            let conductance = 1e-16 / vt * (0.65 / vt).exp();
            let fraction = forward / (forward + 1e-4);
            let extra = 3.0 * ((0.65 - 0.72) / 14.4_f64).exp() * fraction.powi(2);
            let capacitance = 1e-9
                * (conductance * (1.0 + extra * (3.0 - 2.0 * fraction)) + forward * extra / 14.4);
            let expected = rspice_core::Complex64::new(
                -3.0 * conductance / 100.0
                    - reverse_scale * 1e-16 / vt * ((0.65 - 0.72) / vt).exp(),
                -3.0 * std::f64::consts::TAU * 1e8 * capacitance,
            );
            assert!(
                (ac[0].currents[vb] - expected).norm() < 1e-11 * expected.norm(),
                "{kind} {devices}: {:?} != {expected:?}",
                ac[0].currents[vb]
            );
            let result = engine.run_tran(&netlist, 20e-9, 1e-9).unwrap();
            let base = result.try_voltage_waveform_named("b").unwrap();
            let current = result.try_branch_current_waveform_named("Vb").unwrap();
            assert!(result.time.len() > 3 && *result.time.last().unwrap() >= 20e-9);
            for index in 1..result.time.len() {
                let vbe = p * base[index];
                let static_base = p
                    * 1e-16
                    * (3.0 * (vbe / vt).exp_m1() / 100.0
                        + reverse_scale * ((vbe - 0.72) / vt).exp_m1());
                let dynamic_base = 3.0 * p * (charge(vbe) - charge(p * base[index - 1]))
                    / (result.time[index] - result.time[index - 1]);
                assert!(
                    (current[index] + static_base + dynamic_base).abs() < 1e-11,
                    "{kind} {devices} t={}: I={} static={static_base} dynamic={dynamic_base}",
                    result.time[index],
                    current[index]
                );
            }
        }
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
fn vbic_tnf_offsets_and_thermal_ports_match_xyce710() {
    for level in [11, 12] {
        let substrate = if level == 12 { " 0" } else { "" };
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            for (offset, rise, expected) in [
                (20, 0, -2.7689488024311438e-5),
                (0, 20, -2.7689488024311438e-5),
                (20, 20, -5.0209303851805147e-5),
            ] {
                let netlist = Netlist::parse(&format!(
                    "VBIC nominal emission coefficient scaling\nVc c 0 {}\nVb b 0 {}\nVth th 0 {rise}\nQ1 c b 0{substrate} th vm SW_ET=0 M=3 TRISE={offset}\n\
                     .model vm {kind}(LEVEL={level} IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=10 RCI=2 RBX=5 RBI=3 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27)\n.temp 27\n.options gmin=0\n.end\n", p*1.8,p*0.7
                )).unwrap();
                let result = Engine::default().run_dc_op(&netlist).unwrap();
                let actual = result.branch_current_named("Vc").unwrap();
                assert!(
                    (actual - p * expected).abs() < 2e-7 * expected.abs(),
                    "LEVEL={level} {kind} TRISE={offset} thermal={rise}: {actual:e} != {:e}",
                    p * expected
                );
            }
        }
    }
}

#[test]
fn vbic13_signed_reverse_transport_matches_xyce710() {
    // Independent Xyce 7.10 LEVEL=11/12 runs with both junctions reverse
    // biased. Clipping Ifi/Iri to zero incorrectly makes I(Vc) vanish.
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            ..Default::default()
        },
        ..Default::default()
    });
    for level in [11, 12] {
        let substrate = if level == 12 { " 0" } else { "" };
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            for (m, expected) in [(1, -2.0937090356027423e-10), (3, -6.281127106808194e-10)] {
                let netlist = Netlist::parse(&format!(
                    "VBIC reverse transport\nVc c 0 {p}\nVb b 0 {}\nQ1 c b 0{substrate} vm SW_ET=0 M={m}\n\
                     .model vm {kind}(LEVEL={level} IS=1e-8 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27)\n.temp 27\n.options gmin=0\n.end\n",
                    -0.1 * p
                )).unwrap();
                let result = engine.run_dc_op(&netlist).unwrap();
                let actual = result.branch_current_named("Vc").unwrap();
                assert!(
                    (actual - p * expected).abs() < 2e-6 * expected.abs(),
                    "LEVEL={level} {kind} M={m}: {actual:e} != {:e}",
                    p * expected
                );
            }
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
fn vbic13_junction_leakage_obeys_kcl_and_only_heats_series_resistances() {
    // Independent 60-digit solution of the passive leakage/resistance network.
    for (level, collector, base, heat) in [
        (
            11,
            -0.006_299_966_400_242_4,
            0.0032999652003276,
            7.639915280834608e-6,
        ),
        (
            12,
            -0.009899932800544497,
            0.0032999508005375976,
            1.8679743602997705e-5,
        ),
    ] {
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
            // BXCX, and (for LEVEL=12) BCP. They do not generate heat
            // themselves; their current through the 1mOhm physical series
            // resistances does. The thermal resistance is RTH/M.
            assert!(
                (result.branch_current_named("vc").unwrap() - polarity * collector).abs() < 1e-11,
                "level={level} kind={kind} Ic={:?} Ib={:?}",
                result.branch_current_named("vc"),
                result.branch_current_named("vb")
            );
            assert!((result.branch_current_named("vb").unwrap() - polarity * base).abs() < 1e-11);
            assert!(
                (voltage(&result, "q1.__rth.internal") - heat).abs() < 1e-10,
                "heat must include only the physical series resistances"
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
            assert!(
                actual.node_names.iter().any(|name| name.contains(".__bp.")),
                "VBIC 1.3 retains BP through the resistance floor"
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

#[test]
fn vbic13_zero_resistance_floor_matches_xyce710() {
    // Xyce keeps the full resistance network even with every value authored 0.
    for level in [11, 12] {
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            let deck = format!(
                "VBIC resistance floors\nVc c 0 {p}\nVb b 0 {}\nQ1 c b 0{substrate} vm SW_ET=0\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 IBEI=0 IBCI=0 IBEIP=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TNOM=27)\n.temp 27\n.op\n.end\n",
                0.8 * p,
            );
            assert_rel_close(
                "VBIC resistance floor I(Vc)",
                branch_current(&deck, "Vc"),
                p * -0.0027078454711413234,
                1e-7,
            );
        }
    }
}

#[test]
fn vbic13_resistance_floor_preserves_small_current_precision() {
    // A 70-digit Decimal solution of the VBIC equations agrees with Xyce
    // to 4e-22 A; 1mOhm series branches must not inject picoampere errors.
    for level in [11, 12] {
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let substrate = if level == 12 { " 0" } else { "" };
            let deck = format!(
                "VBIC small currents through resistance floors\nVc c 0 {}\nVb b 0 {}\nVth th 0 20\nQ1 c b 0{substrate} th vm SW_ET=1 M=3 TRISE=20\n\
                 .model vm {kind}(LEVEL={level} IS=1e-16 NF=1.1 NR=1.2 ISRR=0.7 TNF=0.001 PNJMAXI=1n XISR=1.8 DEAR=0.1 IBEI=1e-18 IBCI=1e-18 IBEIP=0 ISP=0 RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p TD=1n TF=1n TR=2n)\n.temp 27\n.options gmin=0\n.op\n.end\n",
                1.8 * p,
                0.65 * p,
            );
            let netlist = Netlist::parse(&deck).unwrap();
            let mut unconditioned = SimulationConfig::default();
            // The independent reference has no nodal shunts. Deck GMIN
            // controls junction leakage, separately from this solver floor.
            unconditioned.convergence_config.gmin_target = 0.0;
            let result = Engine::new(unconditioned).run_dc_op(&netlist).unwrap();
            let conditioned = op_result(&deck);
            for (branch, expected) in [
                ("Vc", -7.045_220_149_350_812e-8),
                ("Vb", -8.783310826230572e-8),
            ] {
                let actual = result.branch_current_named(branch).unwrap();
                assert!(
                    (actual - p * expected).abs() < 1e-7 * expected.abs(),
                    "LEVEL={level} {kind} {branch}: {actual:e} != {:e}",
                    p * expected
                );
                // The default 1 fS conditioning is below the ULP of a
                // 3 kS parasitic diagonal, but its current must survive:
                // four collector nodes at 1.8 V, three base nodes at 0.65 V.
                let shunt_current = if branch == "Vc" { 7.2e-15 } else { 1.95e-15 };
                assert!(
                    (conditioned.branch_current_named(branch).unwrap() - actual
                        + p * shunt_current)
                        .abs()
                        < 1e-20,
                    "LEVEL={level} {kind} {branch}: conditioning current was lost",
                );
            }
        }
    }
}

#[test]
fn stationary_zero_bjt_bias_leaves_the_startup_junction_state() {
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for (kind, isat) in [
            ("NPN", 1e-14),
            ("PNP", 1e-14),
            ("NPN", 1.0),
            ("PNP", 1.0),
            ("NPN", 1e20),
            ("PNP", 1e20),
        ] {
            for (terminals, sources) in [("0 0 0", ""), ("c b e", "Vc c 0 0\nVb b 0 0\nVe e 0 0\n")]
            {
                let netlist = Netlist::parse(&format!(
                    "stationary BJT bias\nI1 0 out DC 1 AC 1\nR1 out 0 1\n{sources}Q1 {terminals} qm\n.model qm {kind}(IS={isat})\n.end\n"
                )).unwrap();
                let dc = engine.run_dc_op(&netlist).unwrap_or_else(|error| {
                    panic!("{dialect:?}, {kind}, IS={isat}, {terminals}: {error}")
                });
                assert!((dc.try_voltage_named("out").unwrap() - 1.0).abs() < 1e-12);
                for current in dc.branch_currents {
                    assert!(current.abs() < 1e-14, "zero-bias source current {current}");
                }
                let tran = engine.run_tran(&netlist, 2e-9, 1e-9).unwrap();
                assert!(
                    tran.try_voltage_waveform_named("out")
                        .unwrap()
                        .iter()
                        .all(|v| (v - 1.0).abs() < 1e-12)
                );
                assert_eq!(engine.convergence_quality().force_accepted_points, 0);
            }
        }
    }
}

#[test]
fn small_gummel_poon_instances_scale_dc_and_ac_at_temperature() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for temperature in [280.15, 300.15, 340.15] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.temperature = temperature;
            config.convergence_config.gmin_target = 0.0;
            config.convergence_config.junction_gmin_target = 0.0;
            let engine = Engine::new(config);
            for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
                for (vc, vb) in [(1.0, 0.1), (2.0, 0.7), (0.1, 0.7), (-0.2, -0.4)] {
                    let make = |parameter: &str, scale: f64, mult: &str| {
                        Netlist::parse(&format!(
                        "Small GP instance\nVC c 0 {}\nVB b 0 DC {} AC 1\nQ1 c b 0 mm {parameter}={scale} {mult}\n.model mm {kind}(LEVEL=1 IS=1e-14 BF=100 BR=2 IKF=1e-3 IKR=2e-3 VAF=40 VAR=20 CJE=2p CJC=1p TF=1n TR=2n)\n.options GMIN=0\n.end\n",p*vc,p*vb)).unwrap()
                    };
                    let unit = make("M", 1.0, "");
                    let dc = engine.run_dc_op(&unit).unwrap();
                    let ac = engine.run_ac(&unit, &[1e6]).unwrap();
                    for parameter in ["M", "AREA"] {
                        for scale in [1e-200, 1e-30, 1e-18, 0.25] {
                            let deck = make(parameter, scale, "");
                            // Ngspice AREA also sets the default BC geometry,
                            // so reverse transport is not linear in AREA.
                            // Normalize that same geometry by multiplicity;
                            // retain the full tiny-instance precision check.
                            let (dc, ac) = if parameter == "AREA"
                                && dialect == SpiceDialect::Ngspice
                            {
                                let normalized = make("AREA", scale, &format!("M={}", 1.0 / scale));
                                (
                                    engine.run_dc_op(&normalized).unwrap(),
                                    engine.run_ac(&normalized, &[1e6]).unwrap(),
                                )
                            } else {
                                (dc.clone(), ac.clone())
                            };
                            let actual_dc = engine.run_dc_op(&deck).unwrap();
                            let actual_ac = engine.run_ac(&deck, &[1e6]).unwrap();
                            assert_eq!(actual_dc.branch_names, dc.branch_names);
                            assert_eq!(actual_ac[0].branch_names, ac[0].branch_names);
                            for (&actual, &expected) in
                                actual_dc.branch_currents.iter().zip(&dc.branch_currents)
                            {
                                assert!(
                                    (actual / scale - expected).abs() < expected.abs() * 3e-10,
                                    "{dialect:?} {kind} T={temperature} ({vc},{vb}) {parameter}={scale:e} DC: {actual:e} vs {expected:e}*scale"
                                );
                            }
                            for (actual, expected) in
                                actual_ac[0].currents.iter().zip(&ac[0].currents)
                            {
                                for (actual, expected) in
                                    [(actual.re, expected.re), (actual.im, expected.im)]
                                {
                                    assert!(
                                        (actual / scale - expected).abs() <= expected.abs() * 3e-10,
                                        "{dialect:?} {kind} T={temperature} ({vc},{vb}) {parameter}={scale:e} AC: {actual:e} vs {expected:e}*scale"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn zero_gummel_poon_saturation_current_stays_disabled() {
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    let engine = Engine::new(config);
    for vb in [-0.1, 0.0, 0.7] {
        let deck = Netlist::parse(&format!("Disabled BJT current\nVC c 0 1\nVB b 0 {vb}\nQ1 c b 0 mm\n.model mm NPN(IS=0)\n.options GMIN=0\n.end\n")).unwrap();
        let result = engine.run_dc_op(&deck).unwrap();
        assert!(result.branch_currents.iter().all(|&current| current == 0.0));
    }
}

#[test]
fn legacy_temperature_controls_and_cryogenic_currents_preserve_the_equations() {
    use rspice_core::Complex64;
    // Warning-free ngspice 46 and XyceNF 7.10 DC captures. Xyce clips its
    // junction exponent at 100; the cryogenic Xyce-dialect case instead
    // checks the physical equations with Xyce's k/q, as AC must retain the
    // derivative of the actual junction current rather than a capped value.
    for (dialect, exponent, energy, temperature, vb, vc, reference) in [
        (
            SpiceDialect::Ngspice,
            0.0,
            1.11,
            70.0,
            0.3,
            0.2,
            Some([-3.298800527212516e-07, -4.328755440063333e-09]),
        ),
        (
            SpiceDialect::Xyce,
            0.0,
            1.11,
            70.0,
            0.3,
            0.2,
            Some([-3.305294819137328e-07, -4.1953901686865675e-09]),
        ),
        (
            SpiceDialect::Ngspice,
            -2.0,
            1.11,
            -40.0,
            0.3,
            0.2,
            Some([-1.3377300385225144e-12, -1.6504605776079342e-14]),
        ),
        (
            SpiceDialect::Xyce,
            -2.0,
            1.11,
            -40.0,
            0.3,
            0.2,
            Some([-1.3379090610888149e-12, -1.64880767456844e-14]),
        ),
        (
            SpiceDialect::Ngspice,
            3.0,
            0.0,
            70.0,
            0.3,
            0.2,
            Some([-2.276604405974342e-09, -3.048365269752723e-11]),
        ),
        (
            SpiceDialect::Xyce,
            3.0,
            0.0,
            70.0,
            0.3,
            0.2,
            Some([-2.280739832495972e-09, -2.955897989074215e-11]),
        ),
        (
            SpiceDialect::Ngspice,
            0.0,
            0.0,
            -40.0,
            0.3,
            0.2,
            Some([-1.8319384057959526e-07, -1.7212242983473143e-09]),
        ),
        (
            SpiceDialect::Xyce,
            0.0,
            0.0,
            -40.0,
            0.3,
            0.2,
            Some([-1.8328219526921212e-07, -1.7195327171945165e-09]),
        ),
        (
            SpiceDialect::Ngspice,
            -1.0,
            -0.2,
            70.0,
            0.3,
            0.2,
            Some([-5.056987224693705e-10, -6.8808140483814914e-12]),
        ),
        (
            SpiceDialect::Xyce,
            -1.0,
            -0.2,
            70.0,
            0.3,
            0.2,
            Some([-5.066034549811124e-10, -6.675257679790909e-12]),
        ),
        (
            SpiceDialect::Ngspice,
            3.0,
            1.11,
            -196.15,
            1.0,
            0.9,
            Some([-0.0002778366276213612, -1.191758046411879e-06]),
        ),
        (SpiceDialect::Xyce, 3.0, 1.11, -196.15, 1.0, 0.9, None),
    ] {
        let temp = temperature + 273.15;
        let vt = if dialect == SpiceDialect::Xyce {
            rspice_core::constants::XYCE_K_BOLTZMANN * temp
                / rspice_core::constants::XYCE_Q_ELECTRON
        } else {
            rspice_core::constants::thermal_voltage(temp)
        };
        let ratio = temp / 300.15;
        let log_factor = (ratio - 1.0) * energy / vt + exponent * ratio.ln();
        let beta = ratio.powf(-0.7);
        let diode = |nominal: f64, bias: f64, emission: f64, log_scale: f64| {
            let saturation = nominal * 6.0 * log_scale.exp();
            (
                saturation * (bias / (emission * vt)).exp_m1(),
                saturation * (bias / (emission * vt)).exp() / (emission * vt),
            )
        };
        let (forward, gf) = diode(1e-14, vb, 1.0, log_factor);
        let reverse_area = if dialect == SpiceDialect::Ngspice {
            2.0
        } else {
            1.0
        };
        let (reverse, gr) = diode(reverse_area * 1e-14, vb - vc, 1.0, log_factor);
        let (leak_be, gle) = diode(1e-16, vb, 1.5, log_factor / 1.5 - beta.ln());
        let (leak_bc, glc) = diode(2e-16, vb - vc, 2.0, log_factor / 2.0 - beta.ln());
        let bf = 90.0 * beta;
        let br = 3.0 * beta;
        let expected_dc = [
            -forward + (1.0 + 1.0 / br) * reverse + leak_bc,
            -forward / bf - reverse / br - leak_be - leak_bc,
        ];
        let omega = std::f64::consts::TAU * 1e6;
        let expected_ac = [
            Complex64::new(-gf + (1.0 + 1.0 / br) * gr + glc, omega * 2e-9 * gr),
            Complex64::new(
                -gf / bf - gr / br - gle - glc,
                -omega * (1e-9 * gf + 2e-9 * gr),
            ),
        ];
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            temperature: temp,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                voltage_reltol: 1e-10,
                voltage_abstol: 1e-12,
                current_abstol: 1e-22,
                ..Default::default()
            },
            ..Default::default()
        });
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            for (alias, nested) in [("XTI", false), ("PT", true)] {
                let device = format!(
                    "Q1 c b 0 qm AREA=2 M=3\n.model qm {kind}(IS=1e-14 BF=90 BR=3 ISE=1e-16 NE=1.5 ISC=2e-16 NC=2 NF=1 NR=1 XTB=-.7 {alias}={{x}} EG={{gap}} TF=1n TR=2n)"
                );
                let device = if nested {
                    format!(
                        "X1 c b cell x={exponent} gap={energy}\n.subckt cell c b x=19 gap=2\n{device}\n.ends"
                    )
                } else {
                    device
                };
                let deck=Netlist::parse(&format!("Temperature mapping\n.param x={exponent} gap={energy}\nVC c 0 {}\nVB b 0 DC {} AC 1\n{device}\n.end",polarity*vc,polarity*vb)).unwrap();
                let dc = engine.run_dc_op(&deck).unwrap();
                let ac = engine.run_ac(&deck, &[1e6]).unwrap();
                for (i, name) in ["VC", "VB"].iter().enumerate() {
                    let current = dc.branch_current_named(name).unwrap();
                    let label = format!(
                        "{dialect:?} {kind} {alias}={exponent} EG={energy} T={temperature} {name}"
                    );
                    if let Some(reference) = reference {
                        // Account for the known older ngspice k/q constants.
                        let tolerance = if dialect == SpiceDialect::Ngspice {
                            2e-5
                        } else {
                            1e-8
                        };
                        assert_rel_close(&label, current, polarity * reference[i], tolerance);
                    }
                    assert!(
                        (current - polarity * expected_dc[i]).abs()
                            < expected_dc[i].abs() * 1e-9 + 1e-25,
                        "{label}: {current:e} != {:e}",
                        polarity * expected_dc[i]
                    );
                    let branch = ac[0]
                        .branch_names
                        .iter()
                        .position(|key| key.eq_ignore_ascii_case(name))
                        .unwrap();
                    let actual = ac[0].currents[branch];
                    assert!(
                        (actual - expected_ac[i]).norm() < expected_ac[i].norm() * 1e-9 + 1e-24,
                        "{label}: {actual:?} != {:?}",
                        expected_ac[i]
                    );
                }
            }
        }
    }
    for name in ["XTI", "PT", "EG"] {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut deck =
                Netlist::parse("Invalid temperature law\nQ1 0 0 0 qm\n.model qm NPN\n.end")
                    .unwrap();
            deck.models[0].params.push((name.into(), value));
            assert!(
                Engine::new(SimulationConfig::default())
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
    }
}

#[test]
fn legacy_junction_exponentials_preserve_tiny_and_large_finite_currents() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let vt = if dialect == SpiceDialect::Xyce {
            rspice_core::constants::XYCE_K_BOLTZMANN * 300.15
                / rspice_core::constants::XYCE_Q_ELECTRON
        } else {
            rspice_core::constants::thermal_voltage(300.15)
        };
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                voltage_reltol: 1e-10,
                voltage_abstol: 1e-30,
                current_abstol: 1e-40,
                ..Default::default()
            },
            ..Default::default()
        });
        for (argument, isat) in [(-1e-16, 1e-14), (1e-16, 1e-14), (720.0, 1e-300)] {
            let expected = if argument < 1.0 {
                isat * f64::exp_m1(argument)
            } else {
                f64::exp(f64::ln(isat) + argument) - isat
            };
            let slope = if argument < 1.0 {
                isat * f64::exp(argument) / vt
            } else {
                f64::exp(f64::ln(isat) + argument - f64::ln(vt))
            };
            let deck=Netlist::parse(&format!("Finite junction range\nVC c 0 0\nVB b 0 DC {} AC 1\nQ1 c b 0 qm\n.model qm NPN(IS={isat} BF=100 BR=1)\n.end",argument*vt)).unwrap();
            let dc = engine.run_dc_op(&deck).unwrap();
            let actual = dc.branch_current_named("VC").unwrap();
            assert!(
                (actual - expected).abs() < expected.abs() * 1e-11,
                "{dialect:?} arg={argument}: {actual:e} != {expected:e}"
            );
            let ac = engine.run_ac(&deck, &[1.0]).unwrap();
            let branch = ac[0]
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("VC"))
                .unwrap();
            assert!((ac[0].currents[branch].re - slope).abs() < slope.abs() * 1e-11);
        }
    }
}

#[test]
fn cryogenic_bjt_coefficients_reach_terminal_currents_and_diffusion_charge() {
    use rspice_core::Complex64;
    for (dialect, split) in [
        (SpiceDialect::Ngspice, false),
        (SpiceDialect::Ngspice, true),
        (SpiceDialect::Xyce, false),
    ] {
        let temperature = 10.0;
        let vt = if dialect == SpiceDialect::Xyce {
            rspice_core::constants::XYCE_K_BOLTZMANN * temperature
                / rspice_core::constants::XYCE_Q_ELECTRON
        } else {
            rspice_core::constants::thermal_voltage(temperature)
        };
        let ratio = temperature / 300.15;
        let thermal = (ratio - 1.0) * 1.11 / vt + 3.0 * ratio.ln();
        let beta = ratio.powf(-0.7);
        let bf = 100.0 * beta;
        let br = 2.0 * beta;
        let bc_area = if dialect == SpiceDialect::Xyce {
            2.0
        } else {
            3.0
        };
        // Direct log-domain equations, independently anchored by 65-digit
        // Decimal evaluations below; no stored f64 saturation coefficient.
        let branches = |vb: f64| {
            [
                (if split { 2e-14 } else { 1e-14 }, 2.0, 1.0, vb, false),
                (
                    if split { 7e-14 } else { 1e-14 },
                    if split {
                        bc_area
                    } else if dialect == SpiceDialect::Xyce {
                        2.0
                    } else {
                        2.0 * bc_area
                    },
                    1.0,
                    vb - 0.003,
                    false,
                ),
                (1e-16, 2.0, 1.2, vb, true),
                (2e-17, bc_area, 1.0, vb - 0.003, true),
                (3e-16, bc_area, 1.4, vb - 0.003, true),
                (5e-15, if split { 5.0 } else { 2.0 }, 1.0, 1.1, false),
            ]
            .map(
                |(nominal, area, n, voltage, leakage): (f64, f64, f64, f64, bool)| {
                    let log_is = (nominal * area * 3.0).ln() + thermal / n
                        - if leakage { beta.ln() } else { 0.0 };
                    let exponential = (log_is + voltage / (n * vt)).exp();
                    (exponential - log_is.exp(), exponential / (n * vt))
                },
            )
        };
        if dialect == SpiceDialect::Ngspice {
            let golden = if split {
                [
                    0.00017589292587624807,
                    2.8410195418056978e-5,
                    2.4121464932853505e-9,
                    7.503594100408108e-10,
                    7.319227689842334e-11,
                    0.00010993307867265505,
                ]
            } else {
                [
                    8.794646293812403e-5,
                    8.117198690873422e-6,
                    2.4121464932853505e-9,
                    7.503594100408108e-10,
                    7.319227689842334e-11,
                    4.397323146906202e-5,
                ]
            };
            for ((current, _), expected) in branches(1.1).into_iter().zip(golden) {
                assert!((current / expected - 1.0).abs() < 2e-12);
            }
        }
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            temperature,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                voltage_reltol: 1e-10,
                current_abstol: 1e-18,
                ..Default::default()
            },
            integration_method:
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(
                (0..=4).map(|i| f64::from(i) * 1e-8).collect(),
            )),
            ..Default::default()
        });
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let ng = dialect == SpiceDialect::Ngspice;
            let response = |vb| {
                let [
                    (f, gf),
                    (r, gr),
                    (be, gbe),
                    (bci, gbci),
                    (bcn, gbcn),
                    (sub, _),
                ] = branches(vb);
                let sub = if ng { sub } else { 0.0 };
                let dc = [
                    p * (-f + r * (1.0 + 1.0 / br) + bci + bcn + sub),
                    -p * (f / bf + r / br + be + bci + bcn),
                    -p * sub,
                ];
                let omega = core::f64::consts::TAU * 1e6;
                let ac = [
                    Complex64::new(-gf + gr * (1.0 + 1.0 / br) + gbci + gbcn, omega * 2e-9 * gr),
                    Complex64::new(
                        -gf / bf - gr / br - gbe - gbci - gbcn,
                        -omega * (1e-9 * gf + 2e-9 * gr),
                    ),
                    Complex64::new(0.0, 0.0),
                ];
                (dc, ac, [p * 1e-9 * f, p * 2e-9 * r])
            };
            let geometry = if ng { "AREAB=3 AREAC=5" } else { "" };
            let substrate = if ng { "ISS=5e-15 NS=1 SUBS=1" } else { "" };
            let split_fields = if split { "IBE=2e-14 IBC=7e-14" } else { "" };
            let deck=Netlist::parse(&format!("Cryogenic junction range\nVC c 0 {}\nVB b 0 PWL(0 {} 40n {}) DC {} AC 1\nVS s 0 {}\nQ1 c b 0 s qm AREA=2 M=3 {geometry}\n.model qm {kind}(IS=1e-14 BF=100 BR=2 XTB=-0.7 ISE=1e-16 NE=1.2 IBCI=2e-17 ISC=3e-16 NC=1.4 TF=1n TR=2n {substrate} {split_fields})\n.end",p*0.003,p*1.1,p*1.1005,p*1.1,p*1.103)).unwrap();
            let dc = engine.run_dc_op(&deck).unwrap();
            let ac = engine.run_ac(&deck, &[1e6]).unwrap();
            let (expected_dc, expected_ac, _) = response(1.1);
            for (i, name) in ["VC", "VB", "VS"].iter().enumerate() {
                let current = dc.branch_current_named(name).unwrap();
                assert!(
                    (current - expected_dc[i]).abs() < expected_dc[i].abs() * 1e-9 + 1e-18,
                    "{dialect:?} split={split} {kind} {name}: {current:e} != {:e}",
                    expected_dc[i]
                );
                let branch = ac[0]
                    .branch_names
                    .iter()
                    .position(|key| key.eq_ignore_ascii_case(name))
                    .unwrap();
                assert!(
                    (ac[0].currents[branch] - expected_ac[i]).norm()
                        < expected_ac[i].norm() * 1e-9 + 1e-18,
                    "{dialect:?} split={split} {kind} {name} AC: {:?} != {:?}",
                    ac[0].currents[branch],
                    expected_ac[i]
                );
            }
            if split && p == 1.0 {
                let result = engine.run_tran(&deck, 40e-9, 10e-9).unwrap();
                let voltage = result.try_voltage_waveform_named("b").unwrap();
                let currents = ["VC", "VB"]
                    .map(|name| result.try_branch_current_waveform_named(name).unwrap());
                for i in 1..result.time.len() {
                    let (mut expected, _, q) = response(voltage[i]);
                    let (_, _, previous) = response(voltage[i - 1]);
                    let dt = result.time[i] - result.time[i - 1];
                    expected[0] += (q[1] - previous[1]) / dt;
                    expected[1] -= (q[0] - previous[0] + q[1] - previous[1]) / dt;
                    for terminal in 0..2 {
                        assert!(
                            (currents[terminal][i] - expected[terminal]).abs()
                                < expected[terminal].abs() * 1e-9 + 1e-18
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_split_currents_reach_dc_ac_and_transient() {
    use rspice_core::Complex64;
    // Independent ngspice 46 DC measurements. AC is checked against the
    // derivative of the diode laws: bjtacld.c omits substrate conductance.
    for (temperature, subs, reference) in [
        (
            27.0,
            1,
            [
                3.165937836533358e-06,
                -1.0344045040664362e-06,
                -6.277506987883802e-08,
            ],
        ),
        (
            27.0,
            -1,
            [
                5.171970789437013e-06,
                -1.729405840116699e-06,
                5.398661789424142e-09,
            ],
        ),
        (
            70.0,
            1,
            [
                7.688199823184137e-05,
                -2.4835816379930253e-05,
                -2.3821803046106603e-06,
            ],
        ),
        (
            70.0,
            -1,
            [
                0.0001241713025082066,
                -4.16523084041259e-05,
                2.59330497203629e-07,
            ],
        ),
    ] {
        let temp = temperature + 273.15;
        let vt = rspice_core::constants::thermal_voltage(temp);
        let ratio = temp / 300.15;
        let log_factor = (ratio - 1.0) * 1.11 / vt + 3.0 * ratio.ln();
        let bc_area = if subs == 1 { 3.0 } else { 5.0 };
        let substrate_area = if subs == 1 { 5.0 } else { 3.0 };
        let is_be = 2e-12 * 2.0 * 4.0 * (log_factor / 1.1).exp();
        let is_bc = 7e-12 * bc_area * 4.0 * (log_factor / 1.3).exp();
        let is_sub = 5e-12 * substrate_area * 4.0 * (log_factor / 1.3).exp();
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            temperature: temp,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                voltage_reltol: 1e-9,
                voltage_abstol: 1e-12,
                current_abstol: 1e-18,
                ..Default::default()
            },
            integration_method:
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(
                (0..=20).map(|i| f64::from(i) * 1e-8).collect(),
            )),
            ..Default::default()
        });
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let polarity = p * f64::from(subs);
            // No Early effect, rolloff or depletion capacitance: transport
            // and stored charge have closed-form independent solutions.
            let response = |vb: f64, drive: &str| {
                let diode = |isat: f64, voltage: f64, n: f64| {
                    (
                        isat * (voltage / (n * vt)).exp_m1(),
                        isat * (voltage / (n * vt)).exp() / (n * vt),
                    )
                };
                let (forward, gf) = diode(is_be, vb, 1.1);
                let (reverse, gr) = diode(is_bc, vb + 0.3, 1.3);
                let vsub = if subs == 1 { 0.2 } else { vb + 0.1 };
                let (substrate, gs) = diode(is_sub, vsub, 1.2);
                let (dc_c, dc_b) = if subs == 1 {
                    (
                        p * (-forward + 1.5 * reverse) + polarity * substrate,
                        p * (-forward / 100.0 - reverse / 2.0),
                    )
                } else {
                    (
                        p * (-forward + 1.5 * reverse),
                        p * (-forward / 100.0 - reverse / 2.0) + polarity * substrate,
                    )
                };
                let dvbe = if drive == "VB" { p } else { 0.0 };
                let dvbc = dvbe;
                let dvsub = polarity
                    * if drive == "VS" {
                        1.0
                    } else if subs == -1 {
                        -1.0
                    } else {
                        0.0
                    };
                let substrate_g = polarity * gs * dvsub;
                let re_c =
                    p * (-gf * dvbe + 1.5 * gr * dvbc) + if subs == 1 { substrate_g } else { 0.0 };
                let re_b = p * (-gf * dvbe / 100.0 - gr * dvbc / 2.0)
                    + if subs == -1 { substrate_g } else { 0.0 };
                let omega = std::f64::consts::TAU * 1e6;
                (
                    [dc_c, dc_b, -polarity * substrate],
                    [
                        Complex64::new(re_c, p * omega * 2e-9 * gr * dvbc),
                        Complex64::new(re_b, -p * omega * (1e-9 * gf * dvbe + 2e-9 * gr * dvbc)),
                        Complex64::new(-substrate_g, 0.0),
                    ],
                    [p * 1e-9 * forward, p * 2e-9 * reverse],
                )
            };
            for nested in [false, true] {
                let make = |drive: &str| {
                    let device = format!(
                        "Q1 c b e s qm AREA=2 AREAB=3 AREAC=5 M=4\n.model qm {kind}(IS=3e-12 IBE={{be}} IBC={{bc}} ISS={{sub}} NS=1.2 BF=100 BR=2 NF=1.1 NR=1.3 TF=1n TR=2n SUBS={subs})"
                    );
                    let device = if nested {
                        format!(
                            "X1 c b e s cell be=2p bc=7p sub=5p\n.subckt cell c b e s be=13p bc=17p sub=19p\n{device}\n.ends"
                        )
                    } else {
                        device
                    };
                    Netlist::parse(&format!("Split junctions\n.param be=2p bc=7p sub=5p\nVC c 0 {}\nVB b 0 PWL(0 {} 200n {}) DC {} AC {}\nVE e 0 0\nVS s 0 DC {} AC {}\n{device}\n.end",
                        -0.3*p, 0.04*p, 0.08*p, 0.04*p, u8::from(drive=="VB"), -0.1*p, u8::from(drive=="VS"))).unwrap()
                };
                let deck = make("VB");
                let dc = engine.run_dc_op(&deck).unwrap();
                let (expected_dc, _, _) = response(0.04, "VB");
                for (index, name) in ["VC", "VB", "VS"].iter().enumerate() {
                    let actual = dc.branch_current_named(name).unwrap();
                    // The known SI/older ngspice k/q difference is <8 ppm.
                    assert_rel_close(
                        &format!("{kind} SUBS={subs} T={temperature} nested={nested} {name}"),
                        actual,
                        p * reference[index],
                        8e-6,
                    );
                    assert!(
                        (actual - expected_dc[index]).abs()
                            < 1e-9 * expected_dc[index].abs() + 1e-18
                    );
                }
                for drive in ["VB", "VS"] {
                    let result = engine.run_ac(&make(drive), &[1e6]).unwrap();
                    let (_, expected, _) = response(0.04, drive);
                    for (index, name) in ["VC", "VB", "VS"].iter().enumerate() {
                        let branch = result[0]
                            .branch_names
                            .iter()
                            .position(|key| key.eq_ignore_ascii_case(name))
                            .unwrap();
                        let actual = result[0].currents[branch];
                        assert!(
                            (actual - expected[index]).norm()
                                < 1e-9 * expected[index].norm() + 1e-17,
                            "{kind} SUBS={subs} T={temperature} nested={nested} drive={drive} {name}: {actual:?} != {:?}",
                            expected[index]
                        );
                    }
                }
                let result = engine.run_tran(&deck, 200e-9, 10e-9).unwrap();
                let voltages = result.try_voltage_waveform_named("b").unwrap();
                let currents = ["VC", "VB", "VS"]
                    .map(|name| result.try_branch_current_waveform_named(name).unwrap());
                for i in 1..result.time.len() {
                    let (mut expected, _, charge) = response(p * voltages[i], "VB");
                    let (_, _, previous) = response(p * voltages[i - 1], "VB");
                    let dt = result.time[i] - result.time[i - 1];
                    expected[0] += (charge[1] - previous[1]) / dt;
                    expected[1] -= (charge[0] - previous[0] + charge[1] - previous[1]) / dt;
                    for terminal in 0..3 {
                        assert!(
                            (currents[terminal][i] - expected[terminal]).abs()
                                < 2e-9 * expected[terminal].abs() + 1e-17,
                            "{kind} SUBS={subs} T={temperature} nested={nested} transient {terminal}: {} != {}",
                            currents[terminal][i],
                            expected[terminal]
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_substrate_current_and_charge_flow_through_the_intrinsic_lead() {
    use rspice_core::Complex64;
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            voltage_reltol: 1e-9,
            voltage_abstol: 1e-12,
            current_abstol: 1e-18,
            ..Default::default()
        },
        integration_method: rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
        locked_time_grid: Some(std::sync::Arc::new(
            (0..=20).map(|i| f64::from(i) * 1e-8).collect(),
        )),
        ..Default::default()
    });
    let nvt = 1.2 * rspice_core::constants::thermal_voltage(300.15);
    let isat = 8e-12; // Unsplit ISS uses AREA*M, independently of AREAB/C.
    let resistance = 1000.0; // Authored 8 kohm / (AREA=2 * M=4).
    for (subs, lead, capacitance) in [(1, "RC=8k", 40e-12), (-1, "RB=8k RBM=1k", 24e-12)] {
        // The scalar R-diode-C circuit has a monotone implicit equation;
        // bisection is independent of the engine's Newton implementation.
        let solve = |drive: f64, previous: f64, dt: f64| {
            let mut low = 0.0;
            let mut high = drive;
            for _ in 0..80 {
                let voltage = 0.5 * (low + high);
                let current = isat * (voltage / nvt).exp_m1()
                    + if dt > 0.0 {
                        capacitance * (voltage - previous) / dt
                    } else {
                        0.0
                    };
                if voltage + resistance * current > drive {
                    high = voltage;
                } else {
                    low = voltage;
                }
            }
            0.5 * (low + high)
        };
        let operating_voltage = solve(0.5, 0.0, 0.0);
        let current = isat * (operating_voltage / nvt).exp_m1();
        let admittance = Complex64::new(
            isat * (operating_voltage / nvt).exp() / nvt,
            std::f64::consts::TAU * 1e6 * capacitance,
        );
        let ac_expected = -admittance / (Complex64::new(1.0, 0.0) + resistance * admittance);
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let polarity = p * f64::from(subs);
            let deck = Netlist::parse(&format!("Substrate lead\nVC c 0 0\nVB b 0 0\nVS s 0 PWL(0 {} 200n {}) DC {} AC 1\nQ1 c b 0 s qm AREA=2 AREAB=3 AREAC=5 M=4\n.model qm {kind}(IS=0 ISS=1p NS=1.2 {lead} CJS=2p MJS=0 SUBS={subs})\n.end",polarity*0.5,polarity*0.7,polarity*0.5)).unwrap();
            let dc = engine.run_dc_op(&deck).unwrap();
            assert_rel_close(
                "substrate DC",
                dc.branch_current_named("VS").unwrap(),
                -polarity * current,
                1e-8,
            );
            let ac = engine.run_ac(&deck, &[1e6]).unwrap();
            let branch = ac[0]
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("VS"))
                .unwrap();
            assert!((ac[0].currents[branch] - ac_expected).norm() < ac_expected.norm() * 1e-8);
            let result = engine.run_tran(&deck, 200e-9, 10e-9).unwrap();
            let currents = result.try_branch_current_waveform_named("VS").unwrap();
            let mut previous = operating_voltage;
            for (i, &current) in currents.iter().enumerate().skip(1) {
                let drive = 0.5 + 0.2 * (result.time[i] / 200e-9);
                let voltage = solve(drive, previous, result.time[i] - result.time[i - 1]);
                let expected = -polarity * (drive - voltage) / resistance;
                assert!(
                    (current - expected).abs() < 2e-7 * expected.abs() + 1e-13,
                    "{kind} SUBS={subs} t={}: {} != {expected:e}",
                    result.time[i],
                    current
                );
                previous = voltage;
            }
            assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        }
    }
}

#[test]
fn legacy_tlev_current_laws_match_ngspice_and_dynamic_equations() {
    use rspice_core::Complex64;
    // Independent, isolated ngspice 46 models avoid its function-local
    // bfactor carrying a previous model's value into TLEV=3.
    for (celsius, law, split, overrides, reference) in [
        (
            -40.0,
            0,
            false,
            false,
            [
                -8.493252766471543e-11,
                -1.103849536058198e-12,
                -9.940788323765207e-13,
            ],
        ),
        (
            70.0,
            0,
            true,
            true,
            [
                -2.2173739521600132e-5,
                -9.361498354605891e-7,
                3.0167221726167253e-7,
            ],
        ),
        (
            -40.0,
            1,
            true,
            false,
            [
                -5.158663697463515e-10,
                -6.17223420033702e-11,
                3.0580605488986184e-11,
            ],
        ),
        (
            70.0,
            1,
            false,
            true,
            [
                -1.822490746694588e-5,
                -4.546219106333904e-7,
                -7.632402221885548e-7,
            ],
        ),
        (
            -40.0,
            3,
            false,
            false,
            [
                -0.00026993014910731,
                -3.098803876574745e-6,
                -1.7179210838732885e-7,
            ],
        ),
        (
            70.0,
            3,
            true,
            true,
            [
                -2.4542758845987297e-8,
                -7.453704259823823e-9,
                5.866059314880829e-9,
            ],
        ),
        (
            70.0,
            3,
            false,
            false,
            [
                -9.120406855725484e-9,
                -4.5055650192679e-10,
                -3.910706209920567e-9,
            ],
        ),
        (
            27.0,
            3,
            true,
            true,
            [
                -8.352723017911185e-7,
                -4.415171648529216e-8,
                1.7789689855209342e-8,
            ],
        ),
    ] {
        let t = celsius + 273.15;
        let dt = celsius - 27.0;
        let vt = rspice_core::constants::thermal_voltage(t);
        let ratio = t / 300.15;
        let factlog = (ratio - 1.0) * 1.11 / vt + 3.0 * ratio.ln();
        let xtb = if law == 1 { 0.005 } else { -0.7 };
        let beta = match law {
            1 => 1.0 + xtb * dt,
            3 => 1.0,
            _ => ratio.powf(xtb),
        };
        let bf = 100.0
            * if overrides {
                1.0 + 0.002 * dt + 3e-6 * dt * dt
            } else {
                beta
            };
        let br = 2.0
            * if overrides {
                1.0 - 0.001 * dt + 2e-6 * dt * dt
            } else {
                beta
            };
        let bc_area = if split { 5.0 } else { 3.0 };
        let current = |nominal: f64, area: f64, exponent: f64, index: usize| {
            if law == 3 {
                let coefficients = [
                    (1e-3, 2e-6),
                    (-1.5e-3, 3e-6),
                    (0.7e-3, -1e-6),
                    (-0.4e-3, 1e-6),
                ];
                let (first, second) = coefficients[index];
                nominal.powf(1.0 + first * dt + second * dt * dt) * area * 3.0
            } else {
                nominal * area * 3.0 * exponent.exp()
            }
        };
        let saturation = [
            current(
                if split { 2e-14 } else { 1e-14 },
                2.0,
                if split { factlog / 1.1 } else { factlog },
                0,
            ),
            current(
                if split { 7e-14 } else { 1e-14 },
                if split { bc_area } else { 2.0 * bc_area },
                if split { factlog / 1.3 } else { factlog },
                0,
            ),
            current(1e-16, 2.0, factlog / 1.5 - beta.ln(), 1),
            current(3e-16, bc_area, factlog / 1.7 - beta.ln(), 2),
            current(
                5e-15,
                if split { 3.0 } else { 2.0 },
                if split { factlog / 1.3 } else { factlog },
                3,
            ),
        ];
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            temperature: t,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                current_abstol: 1e-24,
                voltage_reltol: 1e-10,
                ..Default::default()
            },
            integration_method:
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(
                (0..=4).map(|i| f64::from(i) * 1e-8).collect(),
            )),
            ..Default::default()
        });
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let response = |vb: f64| {
                let vsub = if split { vb - 0.05 } else { 0.4 };
                let [(f, gf), (r, gr), (be, gbe), (bc, gbc), (sub, gs)] =
                    core::array::from_fn(|i| {
                        let nvt = [1.1, 1.3, 1.5, 1.7, 1.2][i] * vt;
                        let v = [vb, vb - 0.1, vb, vb - 0.1, vsub][i];
                        (
                            saturation[i] * (v / nvt).exp_m1(),
                            saturation[i] * (v / nvt).exp() / nvt,
                        )
                    });
                let dc = [
                    p * (-f + (1.0 + 1.0 / br) * r + bc + if split { 0.0 } else { sub }),
                    -p * (f / bf + r / br + be + bc + if split { sub } else { 0.0 }),
                    p * if split { sub } else { -sub },
                ];
                let omega = core::f64::consts::TAU * 1e6;
                let ac = [
                    Complex64::new(-gf + (1.0 + 1.0 / br) * gr + gbc, omega * 2e-9 * gr),
                    Complex64::new(
                        -gf / bf - gr / br - gbe - gbc - if split { gs } else { 0.0 },
                        -omega * (1e-9 * gf + 2e-9 * gr),
                    ),
                    Complex64::new(if split { gs } else { 0.0 }, 0.0),
                ];
                (dc, ac, [p * 1e-9 * f, p * 2e-9 * r])
            };
            let split_fields = if split { "IBE=2e-14 IBC=7e-14" } else { "" };
            let gain_fields = if overrides {
                "TBF1=2m TBF2=3u TBR1=-1m TBR2=2u"
            } else {
                ""
            };
            let subs = if split { -1 } else { 1 };
            let device = format!(
                "Q1 c b 0 s qm AREA=2 AREAB=3 AREAC=5 M=3\n.model qm {kind}(IS=1e-14 {split_fields} ISS=5e-15 NS=1.2 BF=100 BR=2 NF=1.1 NR=1.3 ISE=1e-16 NE=1.5 ISC=3e-16 NC=1.7 XTB={xtb} TF=1n TR=2n SUBS={subs} TLEV={{mode}} TIS1=1m TIS2=2u TISE1=-1.5m TISE2=3u TISC1=.7m TISC2=-1u TISS1=-.4m TISS2=1u {gain_fields})"
            );
            let device = if split {
                format!("X1 c b s cell mode={law}\n.subckt cell c b s mode=2\n{device}\n.ends")
            } else {
                format!(".param mode={law}\n{device}")
            };
            let deck=Netlist::parse(&format!("TLEV currents\nVC c 0 {}\nVB b 0 PWL(0 {} 40n {}) DC {} AC 1\nVS s 0 {}\n{device}\n.end",p*0.1,p*0.45,p*0.4505,p*0.45,p*if split {0.05} else {0.5})).unwrap();
            let dc = engine.run_dc_op(&deck).unwrap();
            let ac = engine.run_ac(&deck, &[1e6]).unwrap();
            let (expected_dc, expected_ac, _) = response(0.45);
            for (i, name) in ["VC", "VB", "VS"].iter().enumerate() {
                let actual = dc.branch_current_named(name).unwrap();
                let label = format!(
                    "T={celsius} TLEV={law} split={split} overrides={overrides} {kind} {name}"
                );
                assert_rel_close(&label, actual, p * reference[i], 1e-5);
                assert!(
                    (actual - expected_dc[i]).abs() < expected_dc[i].abs() * 1e-9 + 1e-24,
                    "{label}: {actual:e} != {:e}",
                    expected_dc[i]
                );
                let branch = ac[0]
                    .branch_names
                    .iter()
                    .position(|key| key.eq_ignore_ascii_case(name))
                    .unwrap();
                assert!(
                    (ac[0].currents[branch] - expected_ac[i]).norm()
                        < expected_ac[i].norm() * 1e-9 + 1e-23,
                    "{label} AC"
                );
            }
            if celsius == 70.0 && law == 3 && split && p == 1.0 {
                let result = engine.run_tran(&deck, 40e-9, 10e-9).unwrap();
                let voltage = result.try_voltage_waveform_named("b").unwrap();
                let currents = ["VC", "VB", "VS"]
                    .map(|name| result.try_branch_current_waveform_named(name).unwrap());
                for i in 1..result.time.len() {
                    let (mut expected, _, q) = response(voltage[i]);
                    let (_, _, previous) = response(voltage[i - 1]);
                    let step = result.time[i] - result.time[i - 1];
                    expected[0] += (q[1] - previous[1]) / step;
                    expected[1] -= (q[0] - previous[0] + q[1] - previous[1]) / step;
                    for terminal in 0..3 {
                        assert!(
                            (currents[terminal][i] - expected[terminal]).abs()
                                < expected[terminal].abs() * 1e-9 + 1e-22
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_tlev_validates_domains_and_preserves_power_limits() {
    let config = SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let engine = Engine::new(config.clone());
    for value in [-1.0, 2.0, 4.0, 0.5, f64::NAN, f64::INFINITY] {
        let mut deck = Netlist::parse("Invalid TLEV\nQ1 0 0 0 qm\n.model qm NPN\n.end").unwrap();
        deck.models[0].params.push(("TLEV".into(), value));
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains("TLEV")
        );
    }
    for name in [
        "TBF1", "TBF2", "TBR1", "TBR2", "TIS1", "TIS2", "TISE1", "TISE2", "TISC1", "TISC2",
        "TISS1", "TISS2",
    ] {
        let mut deck =
            Netlist::parse("Invalid current coefficient\nQ1 0 0 0 qm\n.model qm NPN\n.end")
                .unwrap();
        deck.models[0].params.push((name.into(), f64::NAN));
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
        for (dialect, family) in [
            (SpiceDialect::Xyce, "LEVEL=1"),
            (SpiceDialect::Ngspice, "TNF=0"),
        ] {
            let deck = Netlist::parse(&format!(
                "Wrong temperature family\nQ1 0 0 0 qm\n.model qm NPN({family} {name}=0)\n.end"
            ))
            .unwrap();
            assert!(
                Engine::new(config.clone().with_spice_dialect(dialect))
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
    }
    for (fields, expected) in [
        ("TLEV=1 XTB=-2", "positive"),
        ("TBF1=-2", "TBF"),
        ("TBR2=-2", "TBR"),
        ("TLEV=3 IS=0 TIS1=-2", "TIS"),
        ("TLEV=3 ISS=0 TISS2=-2", "TISS"),
        ("TLEV=3 ISE=0 TISE1=-2", "TISE"),
        ("TLEV=3 ISC=0 TISC2=-2", "TISC"),
    ] {
        let deck = Netlist::parse(&format!(
            "Invalid mapped temperature\nQ1 0 0 0 qm TEMP=28\n.model qm NPN({fields})\n.end"
        ))
        .unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(expected)
        );
    }
    // Zero nominal values follow pow's zero-exponent rule, but an absent
    // ISS never creates a substrate diode, even for a zero or negative power.
    let vt = rspice_core::constants::thermal_voltage(301.15);
    for (substrate, power, current) in [
        ("", -1.0, 0.0),
        ("", -2.0, 0.0),
        ("ISS=0", -1.0, -(1e-8 / vt).exp_m1()),
    ] {
        let deck=Netlist::parse(&format!("Zero power\nVC c 0 0\nVB b 0 1e-8\nVS s 0 1e-8\nQ1 c b 0 s qm TEMP=28\n.model qm NPN(TLEV=3 IS=0 TIS1=-1 TISS1={power} {substrate})\n.end")).unwrap();
        let dc = engine.run_dc_op(&deck).unwrap();
        let actual = dc.branch_current_named("VS").unwrap();
        assert!((actual - current).abs() < current.abs() * 1e-10 + 1e-22);
        assert!(dc.branch_current_named("VB").unwrap().abs() > 1e-7);
    }
    let deck=Netlist::parse("Zero beta override\nVC c 0 .1\nVB b 0 .45\nQ1 c b 0 qm TEMP=70\n.model qm NPN(TLEV=1 XTB=5m TBF1=0 TBR2=0 IS=1e-14)\n.end").unwrap();
    let reference = Netlist::parse(
        "Nominal beta\nVC c 0 .1\nVB b 0 .45\nQ1 c b 0 qm TEMP=70\n.model qm NPN(IS=1e-14)\n.end",
    )
    .unwrap();
    assert_eq!(
        engine.run_dc_op(&deck).unwrap().branch_current_named("VB"),
        engine
            .run_dc_op(&reference)
            .unwrap()
            .branch_current_named("VB")
    );
    // IS^2 cannot be stored in f64; the bias exponential restores a finite
    // current and derivative. This exercises the shared retained-scale map.
    let bias = 1400.0 * vt;
    let deck=Netlist::parse(&format!("Power-law range\nVC c 0 0\nVB b 0 {bias} AC 1\nQ1 c b 0 qm TEMP=28\n.model qm NPN(TLEV=3 IS=1e-300 TIS1=1 BR=1)\n.end")).unwrap();
    let expected = (2.0 * 1e-300_f64.ln() + 1400.0).exp();
    let dc = engine.run_dc_op(&deck).unwrap();
    assert!((dc.branch_current_named("VC").unwrap() / expected - 1.0).abs() < 1e-10);
    let ac = engine.run_ac(&deck, &[1.0]).unwrap();
    let branch = ac[0]
        .branch_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("VC"))
        .unwrap();
    assert!((ac[0].currents[branch].re / (expected / vt) - 1.0).abs() < 1e-10);
}

#[test]
fn legacy_emission_temperature_controls_match_ngspice_and_dynamic_equations() {
    use rspice_core::Complex64;
    // ngspice 46 DC measurements, with all ten temperature coefficients.
    for (celsius, split, subs, reference) in [
        (
            -40.0,
            false,
            1,
            [
                -2.9925472958073084e-10,
                -4.503503995608123e-12,
                -3.1295715077895627e-12,
            ],
        ),
        (
            -40.0,
            false,
            -1,
            [
                -3.0231067632677105e-10,
                -7.66089833194704e-12,
                3.1295715077895675e-12,
            ],
        ),
        (
            -40.0,
            true,
            1,
            [
                -1.8212370887476451e-9,
                -2.0670103702046962e-11,
                -1.6045707931178532e-10,
            ],
        ),
        (
            -40.0,
            true,
            -1,
            [
                -1.9770204707795798e-9,
                -1.1833039651373654e-10,
                9.627424758707138e-11,
            ],
        ),
        (
            70.0,
            false,
            1,
            [
                -8.797857401786394e-6,
                -6.464716998513668e-7,
                -3.8303392575241673e-7,
            ],
        ),
        (
            70.0,
            false,
            -1,
            [
                -8.186289954942817e-6,
                -1.3821188090943843e-6,
                3.8303392575241736e-7,
            ],
        ),
        (
            70.0,
            true,
            1,
            [
                -1.0993852425834656e-5,
                -6.267683687268213e-7,
                -2.5232486948176016e-7,
            ],
        ),
        (
            70.0,
            true,
            -1,
            [
                -1.0328895276546684e-5,
                -1.103368719652946e-6,
                1.5139492168905626e-7,
            ],
        ),
    ] {
        let temperature = celsius + 273.15;
        let delta = celsius - 27.0;
        let vt = rspice_core::constants::thermal_voltage(temperature);
        let ratio = temperature / 300.15;
        let factlog = (ratio - 1.0) * 1.11 / vt + 3.0 * ratio.ln();
        let beta = ratio.powf(-0.7);
        let bf = 100.0 * beta;
        let br = 2.0 * beta;
        let bc_area = if subs == 1 { 3.0 } else { 5.0 };
        let sub_area = if subs == 1 { 5.0 } else { 3.0 };
        let nominal_n = [1.1, 1.3, 1.5, 1.7, 1.2];
        let coefficients = [
            (1e-3, 2e-6),
            (-1.5e-3, 1e-6),
            (2e-3, 3e-6),
            (-1e-3, 4e-6),
            (1.3e-3, 5e-6),
        ];
        let operating_n: [f64; 5] = core::array::from_fn(|i| {
            nominal_n[i] * (1.0 + coefficients[i].0 * delta + coefficients[i].1 * delta * delta)
        });
        // Nominal N belongs to saturation scaling; operating N belongs only
        // to the diode slope. Confusing them changes every result below.
        let saturation = [
            if split {
                2e-14 * 6.0 * (factlog / nominal_n[0]).exp()
            } else {
                1e-14 * 6.0 * factlog.exp()
            },
            if split {
                7e-14 * bc_area * 3.0 * (factlog / nominal_n[1]).exp()
            } else {
                1e-14 * 6.0 * bc_area * factlog.exp()
            },
            1e-16 * 6.0 * (factlog / nominal_n[2]).exp() / beta,
            3e-16 * bc_area * 3.0 * (factlog / nominal_n[3]).exp() / beta,
            if split {
                5e-15 * sub_area * 3.0 * (factlog / nominal_n[1]).exp()
            } else {
                5e-15 * 6.0 * factlog.exp()
            },
        ];
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            temperature,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                voltage_reltol: 1e-10,
                current_abstol: 1e-24,
                ..Default::default()
            },
            integration_method:
                rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(
                (0..=4).map(|i| f64::from(i) * 1e-8).collect(),
            )),
            ..Default::default()
        });
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            let response = |vb: f64| {
                let vsub = if subs == 1 { 0.4 } else { vb - 0.05 };
                let [(f, gf), (r, gr), (be, gbe), (bc, gbc), (sub, gs)] =
                    core::array::from_fn(|i| {
                        let v = [vb, vb - 0.1, vb, vb - 0.1, vsub][i];
                        let nvt = operating_n[i] * vt;
                        (
                            saturation[i] * (v / nvt).exp_m1(),
                            saturation[i] * (v / nvt).exp() / nvt,
                        )
                    });
                let dc = [
                    p * (-f + (1.0 + 1.0 / br) * r + bc + if subs == 1 { sub } else { 0.0 }),
                    -p * (f / bf + r / br + be + bc + if subs == -1 { sub } else { 0.0 }),
                    -p * f64::from(subs) * sub,
                ];
                let omega = core::f64::consts::TAU * 1e6;
                let ac = [
                    Complex64::new(-gf + (1.0 + 1.0 / br) * gr + gbc, omega * 2e-9 * gr),
                    Complex64::new(
                        -gf / bf - gr / br - gbe - gbc - if subs == -1 { gs } else { 0.0 },
                        -omega * (1e-9 * gf + 2e-9 * gr),
                    ),
                    Complex64::new(if subs == -1 { gs } else { 0.0 }, 0.0),
                ];
                (dc, ac, [p * 1e-9 * f, p * 2e-9 * r])
            };
            let split_fields = if split { "IBE=2e-14 IBC=7e-14" } else { "" };
            let device = format!(
                "Q1 c b 0 s qm AREA=2 AREAB=3 AREAC=5 M=3\n.model qm {kind}(IS=1e-14 {split_fields} ISS=5e-15 NS=1.2 BF=100 BR=2 NF=1.1 NR=1.3 ISE=1e-16 NE=1.5 ISC=3e-16 NC=1.7 XTB=-.7 TF=1n TR=2n SUBS={subs} TNF1={{a*1m}} TNF2={{a*2u}} TNR1={{a*-1.5m}} TNR2={{a*1u}} TNE1={{a*2m}} TNE2={{a*3u}} TNC1={{a*-1m}} TNC2={{a*4u}} TNS1={{a*1.3m}} TNS2={{a*5u}})"
            );
            let device = if split {
                format!("X1 c b s cell a=1\n.subckt cell c b s a=9\n{device}\n.ends")
            } else {
                format!(".param a=1\n{device}")
            };
            let deck = Netlist::parse(&format!("Emission temperature\nVC c 0 {}\nVB b 0 PWL(0 {} 40n {}) DC {} AC 1\nVS s 0 {}\n{device}\n.end", p*0.1,p*0.45,p*0.4505,p*0.45,p*if subs==1 {0.5} else {0.05})).unwrap();
            let dc = engine.run_dc_op(&deck).unwrap();
            let ac = engine.run_ac(&deck, &[1e6]).unwrap();
            let (expected_dc, expected_ac, _) = response(0.45);
            for (i, name) in ["VC", "VB", "VS"].iter().enumerate() {
                let actual = dc.branch_current_named(name).unwrap();
                let label = format!("T={celsius} split={split} SUBS={subs} {kind} {name}");
                // ngspice's older k/q constants cause a few ppm of drift.
                assert_rel_close(&label, actual, p * reference[i], 1e-5);
                assert!(
                    (actual - expected_dc[i]).abs() < expected_dc[i].abs() * 1e-9 + 1e-24,
                    "{label}: {actual:e} != {:e}",
                    expected_dc[i]
                );
                let branch = ac[0]
                    .branch_names
                    .iter()
                    .position(|key| key.eq_ignore_ascii_case(name))
                    .unwrap();
                assert!(
                    (ac[0].currents[branch] - expected_ac[i]).norm()
                        < expected_ac[i].norm() * 1e-9 + 1e-23,
                    "{label} AC"
                );
            }
            if celsius == 70.0 && split && subs == -1 && p == 1.0 {
                let result = engine.run_tran(&deck, 40e-9, 10e-9).unwrap();
                let voltage = result.try_voltage_waveform_named("b").unwrap();
                let currents = ["VC", "VB", "VS"]
                    .map(|name| result.try_branch_current_waveform_named(name).unwrap());
                for i in 1..result.time.len() {
                    let (mut expected, _, q) = response(voltage[i]);
                    let (_, _, previous) = response(voltage[i - 1]);
                    let dt = result.time[i] - result.time[i - 1];
                    expected[0] += (q[1] - previous[1]) / dt;
                    expected[1] -= (q[0] - previous[0] + q[1] - previous[1]) / dt;
                    for terminal in 0..3 {
                        assert!(
                            (currents[terminal][i] - expected[terminal]).abs()
                                < expected[terminal].abs() * 1e-9 + 1e-22
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_emission_temperature_controls_validate_family_and_operating_domain() {
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let engine = Engine::new(config.clone());
    let names = [
        "TNF1", "TNF2", "TNR1", "TNR2", "TNE1", "TNE2", "TNC1", "TNC2", "TNS1", "TNS2",
    ];
    for name in names {
        for invalid in [f64::NAN, f64::INFINITY] {
            let mut deck =
                Netlist::parse("Invalid coefficient\nQ1 0 0 0 qm\n.model qm NPN\n.end").unwrap();
            deck.models[0].params.push((name.into(), invalid));
            assert!(
                engine
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
        for (dialect, family) in [
            (SpiceDialect::Xyce, "LEVEL=1"),
            (SpiceDialect::Ngspice, "LEVEL=4"),
            (SpiceDialect::Ngspice, "TNF=0"),
        ] {
            let deck = Netlist::parse(&format!(
                "Wrong emission family\nQ1 0 0 0 qm\n.model qm NPN({family} {name}=0)\n.end"
            ))
            .unwrap();
            assert!(
                Engine::new(config.clone().with_spice_dialect(dialect))
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
        // Finite coefficients can still yield a nonpositive operating N.
        let deck = Netlist::parse(&format!(
            "Invalid operating emission\nQ1 0 0 0 qm TEMP=28\n.model qm NPN({name}=-2)\n.end"
        ))
        .unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
    }
    for name in ["NF", "NR", "NE", "NC", "NS"] {
        let deck = Netlist::parse(&format!(
            "Invalid nominal emission\nQ1 0 0 0 qm\n.model qm NPN(TNF1=0 {name}=0)\n.end"
        ))
        .unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
    }
    // An explicit zero polynomial overrides shared TNF for only that axis.
    let control = Netlist::parse("Zero emission polynomial\nVC c 0 .1\nVB b 0 .45\nQ1 c b 0 qm TEMP=70\n.model qm NPN(LEVEL=1 IS=1e-14 TNF=1m TNF1=0 TNR1=0)\n.end").unwrap();
    let nominal = Netlist::parse("Zero emission reference\nVC c 0 .1\nVB b 0 .45\nQ1 c b 0 qm TEMP=70\n.model qm NPN(IS=1e-14)\n.end").unwrap();
    let actual = engine.run_dc_op(&control).unwrap();
    let expected = engine.run_dc_op(&nominal).unwrap();
    for name in ["VC", "VB"] {
        assert_eq!(
            actual.branch_current_named(name),
            expected.branch_current_named(name)
        );
    }
}

#[test]
fn legacy_split_current_presence_and_validation_follow_the_model_family() {
    let config = SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            ..Default::default()
        },
        ..Default::default()
    };
    let engine = Engine::new(config);
    let vt = rspice_core::constants::thermal_voltage(300.15);
    for (fields, be, bc) in [
        ("IBE=0 IBC=7p", 0.0, 7e-12),
        ("IBE=2p IBC=0", 2e-12, 0.0),
        ("IBE=0 IBC=0", 0.0, 0.0),
        ("IBE=9p", 3e-12, 6e-12),
        ("IBC=9p", 3e-12, 6e-12),
    ] {
        let deck = Netlist::parse(&format!("Split presence\nVC c 0 -.3\nVB b 0 .04\nQ1 c b 0 qm AREA=2 AREAB=3 M=4\n.model qm NPN(IS=3p {fields} BF=100 BR=2)\n.end")).unwrap();
        let result = engine.run_dc_op(&deck).unwrap();
        let forward = be * 8.0 * (0.04 / vt).exp_m1();
        let reverse = bc * 12.0 * (0.34 / vt).exp_m1();
        for (name, expected) in [
            ("VC", -forward + 1.5 * reverse),
            ("VB", -forward / 100.0 - reverse / 2.0),
        ] {
            let actual = result.branch_current_named(name).unwrap();
            assert!(
                (actual - expected).abs() <= expected.abs() * 1e-9 + 1e-18,
                "{fields} {name}: {actual:e} != {expected:e}"
            );
        }
    }
    for name in ["IBE", "IBC", "ISS", "NS"] {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut deck =
                Netlist::parse("Invalid junction\nQ1 0 0 0 qm\n.model qm NPN\n.end").unwrap();
            deck.models[0].params.push((name.into(), value));
            assert!(
                engine
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
        for (dialect, family) in [
            (SpiceDialect::Xyce, "LEVEL=1"),
            (SpiceDialect::Ngspice, "LEVEL=4"),
            (SpiceDialect::Ngspice, "TNF=0"),
        ] {
            let deck = Netlist::parse(&format!(
                "Wrong junction family\nQ1 0 0 0 qm\n.model qm NPN({family} {name}=1)\n.end"
            ))
            .unwrap();
            assert!(
                Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
    }
    let deck =
        Netlist::parse("Invalid emission\nQ1 0 0 0 qm\n.model qm NPN(ISS=1p NS=0)\n.end").unwrap();
    assert!(
        engine
            .run_dc_op(&deck)
            .unwrap_err()
            .to_string()
            .contains("NS")
    );
    for name in ["NF", "NR"] {
        let deck = Netlist::parse(&format!(
            "Invalid split emission\nQ1 0 0 0 qm\n.model qm NPN(IBE=1p IBC=2p {name}=0)\n.end"
        ))
        .unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
    }
}

#[test]
fn legacy_junction_areas_match_ngspice_currents_and_stored_charge() {
    // Independently captured ngspice 46 DC and 1 MHz AC currents at C/B/S.
    // Both polarities were measured; DC changes sign and AC is identical.
    // ngspice const.h uses k=1.38064852e-23, q=1.6021766208e-19;
    // RSpice retains current SI constants. Evaluating the exponential
    // junction laws with each pair gives 4.46 ppm DC drift at 27 C and
    // 5.73 ppm at 70 C. Allow 8 ppm for that documented reference
    // difference; the charge-balance and multiplicity checks stay strict.
    for (area, explicit, subs, temperature, dc_reference, ac_reference) in [
        (
            1,
            false,
            1,
            27.0,
            [3.072291281861438e-06, -1.026072606073128e-06, 0.0],
            [
                (-0.0001187566203542553, -0.0002253863750083927),
                (3.9644139641568696e-05, 9.302060453714276e-05),
                (0.0, 0.00013236577047124993),
            ],
        ),
        (
            2,
            false,
            1,
            27.0,
            [1.2283283807314812e-05, -4.098378960010237e-06, 0.0],
            [
                (-0.0004748506828465704, -0.0004527610635224993),
                (0.000158400759995824, 0.0001880295225799994),
                (0.0, 0.00026473154094249987),
            ],
        ),
        (
            2,
            true,
            1,
            27.0,
            [1.8424940490939344e-05, -6.1475681479067665e-06, 0.0],
            [
                (-0.0007122760242698556, -0.0009438731362262488),
                (0.000237601139993736, 0.0002820442838699992),
                (0.0, 0.0006618288523562496),
            ],
        ),
        (
            2,
            true,
            -1,
            70.0,
            [0.0019128605970037404, -0.0006378592895541525, 0.0],
            [
                (0.06468622631986501, 0.0010358099162726597),
                (-0.021568345999745282, -0.0015444077190210586),
                (0.0, 0.000401990629497553),
            ],
        ),
    ] {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            temperature: temperature + 273.15,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                voltage_reltol: 1e-9,
                voltage_abstol: 1e-12,
                current_abstol: 1e-18,
                ..Default::default()
            },
            ..Default::default()
        });
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            for nested in [false, true] {
                let fields = if explicit {
                    "AREAB={ab} AREAC={ac}"
                } else {
                    ""
                };
                let device = format!("Q1 c b e s qm AREA={area} {fields} M=4");
                let device = if nested {
                    format!(
                        "X1 c b e s cell ab=3 ac=5\n.subckt cell c b e s ab=19 ac=29\n{device}\n.ends"
                    )
                } else {
                    device
                };
                let (ac_c, ac_b) = if subs == 1 { (1, 0) } else { (0, 1) };
                let deck = Netlist::parse(&format!(
                    "Junction areas\n.param ab=3 ac=5\n\
                     VC c 0 DC {} AC {ac_c}\nVB b 0 DC {} AC {ac_b}\nVE e 0 0\nVS s 0 {}\n\
                     {device}\n.model qm {kind}(IS=1e-12 BF=100 BR=2 ISE=2e-14 NE=1.5 ISC=3e-14 NC=1.3 CJE=2p CJC=3p CJS=5p MJS=.2 TF=1n TR=2n SUBS={subs})\n.options gmin=0\n.end",
                    -0.3*polarity, 0.04*polarity, -0.1*polarity
                )).unwrap();
                let dc = engine.run_dc_op(&deck).unwrap();
                let ac = engine.run_ac(&deck, &[1e6]).unwrap();
                for (index, name) in ["VC", "VB", "VS"].iter().enumerate() {
                    let branch = dc
                        .branch_names
                        .iter()
                        .position(|branch| branch.eq_ignore_ascii_case(name))
                        .unwrap();
                    assert_rel_close(
                        &format!(
                            "{kind} SUBS={subs} T={temperature} AREA={area} explicit={explicit} nested={nested} {name}"
                        ),
                        dc.branch_currents[branch],
                        polarity * dc_reference[index],
                        8e-6,
                    );
                    let branch = ac[0]
                        .branch_names
                        .iter()
                        .position(|branch| branch.eq_ignore_ascii_case(name))
                        .unwrap();
                    let (re, im) = ac_reference[index];
                    let expected = rspice_core::Complex64::new(re, im);
                    assert!(
                        (ac[0].currents[branch] - expected).norm() < 8e-6 * expected.norm() + 1e-16,
                        "{kind} SUBS={subs} AREA={area} explicit={explicit} nested={nested} {name}: {:?} != {expected:?}",
                        ac[0].currents[branch]
                    );
                }
            }
        }
    }

    // With zero grading and transport, I = C*dV/dt provides an independent
    // transient charge-balance check for both substrate connections.
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            ..Default::default()
        },
        integration_method: rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
        locked_time_grid: Some(std::sync::Arc::new(
            (0..=20).map(|index| f64::from(index) * 1e-8).collect(),
        )),
        ..Default::default()
    });
    for (subs, drive, capacitance) in [(1, "c", 136e-12), (-1, "b", 120e-12)] {
        let ground = if subs == 1 { "b" } else { "c" };
        let deck = Netlist::parse(&format!(
            "Junction charge ramp\nVdrive {drive} 0 PWL(0 0 200n .02)\nVground {ground} 0 0\n\
             Q1 c b 0 0 qm AREA=2 AREAB=3 AREAC=5 M=4\n.model qm NPN(IS=0 CJC=3p CJS=5p MJC=0 MJS=0 SUBS={subs})\n.end"
        )).unwrap();
        let result = engine.run_tran(&deck, 200e-9, 10e-9).unwrap();
        let currents = result.try_branch_current_waveform_named("Vdrive").unwrap();
        assert!(currents.len() > 3);
        for &current in currents.iter().skip(1) {
            assert!(
                (current + capacitance * 1e5).abs() < 1e-13,
                "SUBS={subs}: {current:e}"
            );
        }
    }
    for name in ["AREAB", "AREAC"] {
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            let mut deck = Netlist::parse(
                "Invalid area\nVC c 0 1\nVB b 0 0\nQ1 c b 0 qm\n.model qm NPN\n.end",
            )
            .unwrap();
            let ElementKind::Bjt {
                instance_params, ..
            } = &mut deck.elements[2].kind
            else {
                unreachable!()
            };
            instance_params.push((name.into(), value));
            assert!(
                engine
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
        for (dialect, level) in [(SpiceDialect::Xyce, 1), (SpiceDialect::Ngspice, 4)] {
            let deck = Netlist::parse(&format!("Wrong area family\nVC c 0 1\nVB b 0 0\nQ1 c b 0 qm {name}=2\n.model qm NPN(LEVEL={level})\n.end")).unwrap();
            assert!(
                Engine::new(SimulationConfig::default().with_spice_dialect(dialect))
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
    }
}

#[test]
fn legacy_transport_temperature_controls_match_ngspice_and_equivalent_models() {
    // Independently measured ngspice 46 DC/AC data, including IRB, AREA/M,
    // both Early effects, both high-current knees and all transit terms.
    let parameters = [
        ("VAF", 40.0, "TVAF1", 0.002, "TVAF2", 1e-05),
        ("VAR", 15.0, "TVAR1", -0.001, "TVAR2", 2e-05),
        ("IKF", 0.002, "TIKF1", 0.003, "TIKF2", 2e-05),
        ("IKR", 0.003, "TIKR1", -0.002, "TIKR2", 1e-05),
        ("IRB", 1e-05, "TIRB1", 0.004, "TIRB2", 1e-05),
        ("TF", 2e-09, "TTF1", 0.005, "TTF2", -1e-05),
        ("TR", 3e-09, "TTR1", -0.003, "TTR2", 2e-05),
        ("ITF", 0.0005, "TITF1", 0.002, "TITF2", 3e-05),
        ("RC", 8.0, "TRC1", 0.003, "TRC2", 2e-05),
        ("RE", 3.0, "TRE1", -0.001, "TRE2", 1e-05),
    ];
    let config = SimulationConfig {
        spice_dialect: SpiceDialect::Ngspice,
        convergence_config: ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            voltage_reltol: 1e-10,
            voltage_abstol: 1e-12,
            current_abstol: 1e-18,
            ..Default::default()
        },
        ..Default::default()
    };
    let engine = Engine::new(config.clone());
    for (temperature, vc, dc_reference, ac_reference) in [
        (
            -40.0,
            1.0,
            [
                -1.3503851267415712e-05,
                -1.738607070256304e-07,
                1.3677711974472674e-05,
            ],
            [
                [-0.0005357174275035237, 1.1739495899857801e-05],
                [-7.23911367254191e-06, -0.00015406705145988282],
                [0.000542959163483292, 8.589855355657624e-05],
            ],
        ),
        (
            27.0,
            1.0,
            [
                -0.003360575118428022,
                -5.6129418738205816e-05,
                0.003416704537166235,
            ],
            [
                [-0.07823013540264195, 0.0030095050813166717],
                [-0.0016981378707962079, -0.002428060364399574],
                [0.0799290296445625, -0.0006183320071880909],
            ],
        ),
        (
            70.0,
            1.0,
            [
                -0.018087233404887493,
                -0.0005484493185713563,
                0.018635682723458814,
            ],
            [
                [-0.22970250378263787, 0.024084042476336234],
                [-0.012119262435395909, -0.00952795367967403],
                [0.24182882387173937, -0.014545324814211277],
            ],
        ),
        (
            70.0,
            0.2,
            [
                -0.01735061496240968,
                -0.0006553152103060955,
                0.01800593017271576,
            ],
            [
                [-0.21180005567077803, 0.022484544062144753],
                [-0.015344926111985483, -0.009688793514697164],
                [0.22715157071507736, -0.012790232747375665],
            ],
        ),
    ] {
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            let mut controls = String::new();
            let mut mapped = String::new();
            let dt = temperature - 27.0;
            for (name, value, first, c1, second, c2) in parameters {
                // Exercise native RC/RE coefficient aliases and a scoped
                // expression, without changing any other token's spelling.
                let first = match first {
                    "TRC1" => "TRC",
                    "TRE1" => "TRE",
                    _ => first,
                };
                let coefficient = if first == "TIKF1" {
                    "{knee_tc}".to_string()
                } else {
                    c1.to_string()
                };
                controls.push_str(&format!(
                    " {name}={value} {first}={coefficient} {second}={c2}"
                ));
                mapped.push_str(&format!(
                    " {name}={}",
                    value * (1.0 + c1 * dt + c2 * dt * dt)
                ));
            }
            let make = |fields: &str| {
                format!(
                    "GP transport temperature\n.param knee_tc=9\nVC c 0 {} AC .3\nVB b 0 {} AC 1\nVE e 0 0 AC .2\nX1 c b e cell knee_tc=.003\n.subckt cell c b e knee_tc=7\nQ1 c b e 0 qm AREA=2 M=3\n.model qm {kind}(IS=1e-14 BF=80 BR=3 SUBS=1 RB=120 RBM=20 CJE=2p CJC=3p CJS=5p XCJC=.35 XTF=2 VTF=5 {fields})\n.ends\n.temp {temperature}\n.end",
                    polarity * vc,
                    polarity * 0.65
                )
            };
            let actual_text = make(&controls);
            let mapped_text = make(&mapped);
            let actual_deck = Netlist::parse(&actual_text).unwrap();
            let mapped_deck = Netlist::parse(&mapped_text).unwrap();
            let actual_dc = engine.run_dc_op(&actual_deck).unwrap();
            let mapped_dc = engine.run_dc_op(&mapped_deck).unwrap();
            let actual_ac = engine.run_ac(&actual_deck, &[1e6]).unwrap();
            let mapped_ac = engine.run_ac(&mapped_deck, &[1e6]).unwrap();
            for (index, name) in ["VC", "VB", "VE"].into_iter().enumerate() {
                let actual = actual_dc.branch_current_named(name).unwrap();
                let equivalent = mapped_dc.branch_current_named(name).unwrap();
                assert!((actual - equivalent).abs() <= equivalent.abs() * 1e-9 + 1e-16);
                let reference = polarity * dc_reference[index];
                // The independent simulator uses older k/q constants.
                assert!(
                    (actual - reference).abs() < reference.abs() * 3e-5,
                    "{kind} T={temperature} VC={vc} DC {name}: {actual:e} vs {reference:e}"
                );
                let column = actual_ac[0]
                    .branch_names
                    .iter()
                    .position(|branch| branch.eq_ignore_ascii_case(name))
                    .unwrap();
                let actual = actual_ac[0].currents[column];
                let equivalent = mapped_ac[0].currents[column];
                assert!((actual - equivalent).norm() <= equivalent.norm() * 1e-9 + 1e-16);
                let reference =
                    rspice_core::Complex64::new(ac_reference[index][0], ac_reference[index][1]);
                assert!(
                    (actual - reference).norm() < reference.norm() * 3e-5,
                    "{kind} T={temperature} VC={vc} AC {name}: {actual:?} vs {reference:?}"
                );
            }
            if temperature == 70.0 && vc == 0.2 {
                let mut config = config.clone();
                config.integration_method =
                    rspice_core::numerics::integration::IntegrationMethod::BackwardEuler;
                config.locked_time_grid = Some(std::sync::Arc::new(
                    (0..=20).map(|i| f64::from(i) * 1e-9).collect(),
                ));
                let engine = Engine::new(config);
                let ramp = |text: &str| {
                    Netlist::parse(&text.replace(
                        &format!("VB b 0 {} AC 1", polarity * 0.65),
                        &format!("VB b 0 PWL(0 {} 20n {})", polarity * 0.65, polarity * 0.67),
                    ))
                    .unwrap()
                };
                let actual = engine.run_tran(&ramp(&actual_text), 20e-9, 1e-9).unwrap();
                let mapped = engine.run_tran(&ramp(&mapped_text), 20e-9, 1e-9).unwrap();
                assert_eq!(actual.time, mapped.time);
                for name in ["VC", "VB", "VE"] {
                    let a = actual.try_branch_current_waveform_named(name).unwrap();
                    let b = mapped.try_branch_current_waveform_named(name).unwrap();
                    for (a, b) in a.iter().zip(b) {
                        assert!((a - b).abs() <= b.abs() * 1e-8 + 1e-14);
                    }
                }
            }
        }
    }
    // The IRB fix also applies without temperature coefficients, under both
    // dialects. Parallel copies must leave internal bias and knee ratios fixed.
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let engine = Engine::new(config.clone().with_spice_dialect(dialect));
        let deck = |m: f64| {
            Netlist::parse(&format!("IRB multiplicity\nVC c 0 1\nVB b 0 .68 AC 1\nQ1 c b 0 qm AREA=2 M={m}\n.model qm NPN(IS=1e-14 BF=80 RB=120 RBM=20 IRB=1e-5)\n.end")).unwrap()
        };
        let unit = engine.run_ac(&deck(1.0), &[1e6]).unwrap();
        let parallel = engine.run_ac(&deck(4.0), &[1e6]).unwrap();
        for (one, four) in unit[0].currents.iter().zip(&parallel[0].currents) {
            assert!((four - one * 4.0).norm() <= one.norm() * 4e-9 + 1e-15);
        }
    }
}

#[test]
fn legacy_external_bc_charge_matches_capacitor_terminals_and_checkpoint_history() {
    use rspice_core::engine::{
        TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
    };
    use rspice_core::numerics::integration::IntegrationMethod;
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
        ] {
            let config = SimulationConfig {
                spice_dialect: dialect,
                integration_method: method,
                convergence_config: ConvergenceConfig {
                    gmin_target: 0.0,
                    junction_gmin_target: 0.0,
                    voltage_reltol: 1e-10,
                    voltage_abstol: 1e-12,
                    current_abstol: 1e-18,
                    ..Default::default()
                },
                locked_time_grid: Some(std::sync::Arc::new(
                    (0..=20).map(|i| f64::from(i) * 5e-9).collect(),
                )),
                ..Default::default()
            };
            let engine = Engine::new(config);
            for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
                for rc in [0.0, 100.0] {
                    let sources = format!(
                        "External BC storage\nVC c 0 0\nVB b 0 PWL(0 0 100n {}) AC 1\n",
                        p * 0.1
                    );
                    // Both branches share one order/timestep controller. Ngspice
                    // applies different truncation coverage to C1 and BJT XCJC.
                    let equivalent = if rc == 0.0 {
                        "C1 br cr 6n".into()
                    } else {
                        format!("C1 br ci 6n\nR1 ci cr {}", rc / 6.0)
                    };
                    let actual = Netlist::parse(&format!("{sources}Q1 c b 0 qm AREA=3 M=2\n.model qm {kind}(IS=0 RB=50 RC={rc} CJC=1n MJC=0 XCJC=0)\nVCref cr 0 0\nVBref br 0 PWL(0 0 100n {}) AC 1\n{equivalent}\n.save @Q1[ib] @Q1[ic] I(VB) I(VC) I(VBref) I(VCref)\n.end", p * 0.1)).unwrap();
                    let ac = engine.run_ac(&actual, &[1e6]).unwrap();
                    let current = |name: &str| {
                        ac[0].currents[ac[0]
                            .branch_names
                            .iter()
                            .position(|n| n.eq_ignore_ascii_case(name))
                            .unwrap()]
                    };
                    for (name, reference) in [("VB", "VBref"), ("VC", "VCref")] {
                        let a = current(name);
                        let b = current(reference);
                        assert!((a - b).norm() < b.norm() * 1e-11);
                    }
                    let check_resume = dialect == SpiceDialect::Ngspice
                        && method == IntegrationMethod::BackwardEuler
                        && p == 1.0
                        && rc == 100.0;
                    let (a, scheduled) = if check_resume {
                        engine
                            .run_tran_checkpoint_schedule_with_startup_mode(
                                &actual,
                                100e-9,
                                5e-9,
                                TransientStartupMode::OperatingPoint,
                                &[50e-9],
                            )
                            .unwrap()
                    } else {
                        (engine.run_tran(&actual, 100e-9, 5e-9).unwrap(), Vec::new())
                    };
                    for (source, reference, parameter) in
                        [("VB", "VBref", "IB"), ("VC", "VCref", "IC")]
                    {
                        let actual = a.try_branch_current_waveform_named(source).unwrap();
                        let expected = a.try_branch_current_waveform_named(reference).unwrap();
                        let device = a.try_device_op_waveform_named("Q1", parameter).unwrap();
                        for ((actual, expected), device) in actual.iter().zip(expected).zip(device)
                        {
                            assert!(
                                (actual - expected).abs() < expected.abs() * 1e-8 + 1e-13,
                                "{dialect:?} {method:?} {kind} RC={rc} {source}: {actual:e} vs {expected:e}"
                            );
                            assert!(
                                (device + actual).abs() < actual.abs() * 1e-8 + 1e-13,
                                "{dialect:?} {method:?} {kind} RC={rc} {parameter}: {device:e} vs source {actual:e}"
                            );
                        }
                    }
                    if check_resume {
                        let checkpoint = &scheduled[0].checkpoint;
                        let checkpoint = TransientCheckpoint::from_bytes(
                            &checkpoint
                                .to_bytes(TransientCheckpointEncoding::Packed)
                                .unwrap(),
                        )
                        .unwrap();
                        let (resumed, _) = engine
                            .run_tran_resume(&actual, &checkpoint, 100e-9, 5e-9)
                            .unwrap();
                        let seam = a.time.iter().position(|t| *t == resumed.time[0]).unwrap();
                        assert_eq!(resumed.time, a.time[seam..]);
                        for parameter in ["IB", "IC"] {
                            let continued = resumed
                                .try_device_op_waveform_named("Q1", parameter)
                                .unwrap();
                            let full = a.try_device_op_waveform_named("Q1", parameter).unwrap();
                            assert_eq!(continued, &full[seam..]);
                        }
                        // Terminal checkpoints deliberately reset integration order,
                        // but must preserve the accepted current at the seam exactly.
                        let (prefix, checkpoint) =
                            engine.run_tran_checkpointed(&actual, 50e-9, 5e-9).unwrap();
                        let checkpoint = TransientCheckpoint::from_bytes(
                            &checkpoint
                                .to_bytes(TransientCheckpointEncoding::Packed)
                                .unwrap(),
                        )
                        .unwrap();
                        let (resumed, _) = engine
                            .run_tran_resume(&actual, &checkpoint, 100e-9, 5e-9)
                            .unwrap();
                        for parameter in ["IB", "IC"] {
                            assert_eq!(
                                resumed
                                    .try_device_op_waveform_named("Q1", parameter)
                                    .unwrap()[0],
                                *prefix
                                    .try_device_op_waveform_named("Q1", parameter)
                                    .unwrap()
                                    .last()
                                    .unwrap(),
                            );
                        }
                    }
                }
            }
        }
    }
    for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            integration_method: IntegrationMethod::BackwardEuler,
            locked_time_grid: Some(std::sync::Arc::new(vec![0.0, 5e-9, 10e-9])),
            ..Default::default()
        });
        let netlist = Netlist::parse(&format!(
            "External BC initial charge\nVB b 0 0\nVC c 0 0\nQ1 c b 0 qm AREA=3 M=2 IC={},0\n.model qm {kind}(IS=0 RB=50 CJC=1n MJC=0 XCJC=0)\n.save @Q1[ib] I(VB)\n.end", polarity * 0.2,
        )).unwrap();
        let result = engine
            .run_tran_with_startup_mode(&netlist, 10e-9, 5e-9, TransientStartupMode::Uic)
            .unwrap();
        let current = result.try_branch_current_waveform_named("VB").unwrap()[1];
        // C=6n, V(0)=+/-0.2, V(5ns)=0 => dQ/dt=-/+0.24 A.
        assert!(
            (current - polarity * 0.24).abs() < 1e-10,
            "{kind} initial external BC charge: {current:e}"
        );
        assert!(
            (result.try_device_op_waveform_named("Q1", "IB").unwrap()[1] + current).abs() < 1e-10
        );
    }
}

#[test]
fn legacy_transport_temperature_controls_validate_domains() {
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let engine = Engine::new(config.clone());
    for name in [
        "TVAF1", "TVAF2", "TVAR1", "TVAR2", "TIKF1", "TIKF2", "TIKR1", "TIKR2", "TIRB1", "TIRB2",
        "TTF1", "TTF2", "TTR1", "TTR2", "TITF1", "TITF2", "TRC1", "TRC2", "TRC", "TRE1", "TRE2",
        "TRE", "TRB", "TRB1", "TRB2", "TRM1", "TRM2",
    ] {
        let mut deck =
            Netlist::parse("Invalid transport coefficient\nQ1 0 0 0 qm\n.model qm NPN\n.end")
                .unwrap();
        deck.models[0].params.push((name.into(), f64::NAN));
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
        for (dialect, family) in [
            (SpiceDialect::Xyce, "LEVEL=1"),
            (SpiceDialect::Ngspice, "TNF=0"),
        ] {
            let deck = Netlist::parse(&format!(
                "Wrong transport family\nQ1 0 0 0 qm\n.model qm NPN({family} {name}=0)\n.end"
            ))
            .unwrap();
            assert!(
                Engine::new(config.clone().with_spice_dialect(dialect))
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
    }
    for (name, field) in [
        ("VAF", "TVAF1"),
        ("VAR", "TVAR1"),
        ("IKF", "TIKF1"),
        ("IKR", "TIKR1"),
        ("IRB", "TIRB1"),
        ("TF", "TTF1"),
        ("TR", "TTR1"),
        ("ITF", "TITF1"),
        ("RC", "TRC1"),
        ("RE", "TRE1"),
        ("RB", "TRB1"),
        ("RBM", "TRM1"),
    ] {
        let base = if name == "RBM" { "RB=2" } else { "" };
        for coefficient in [-2.0, 1e308] {
            let deck = Netlist::parse(&format!("Invalid mapped transport\nQ1 0 0 0 qm TEMP=29\n.model qm NPN({base} {name}=1 {field}={coefficient})\n.end")).unwrap();
            assert!(
                engine
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(field)
            );
        }
        let deck = Netlist::parse(&format!(
            "Invalid nominal transport\nQ1 0 0 0 qm\n.model qm NPN({base} {name}=-1 {field}=0)\n.end"
        ))
        .unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
    }
    for name in ["VAF", "VAR", "IKF", "IKR"] {
        let coefficient = format!("T{name}1");
        let deck = Netlist::parse(&format!("Zero inverse parameter\nQ1 0 0 0 qm TEMP=28\n.model qm NPN({name}=1 {coefficient}=-1)\n.end")).unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(&coefficient)
        );
    }
    // Omitted/explicit-zero mechanisms remain disabled, even if a coefficient
    // would be outside its domain for a nonzero nominal value.
    let deck = Netlist::parse("Disabled mechanisms\nQ1 0 0 0 qm TEMP=29\n.model qm NPN(TVAF1=-1 TVAR1=-1 TIKF1=-1 TIKR1=-1 TIRB1=-1 TTF1=-1 TTR1=-1 TITF1=-1 TRC=-1 TRE=-1)\n.end").unwrap();
    engine.run_dc_op(&deck).unwrap();
}

#[test]
fn legacy_capacitance_temperature_controls_match_ngspice_and_stored_charge() {
    // Independent ngspice 46 AC measurements, with all twelve CT/TVJ/TMJ
    // coefficients active. TLEVC=0 deliberately ignores the CT/TVJ values.
    for (law, temperature, bias, expected) in [
        (
            0.0,
            -40.0,
            -0.4,
            [
                1.6765259396807914e-12,
                2.140391394373884e-12,
                4.657460127387347e-12,
            ],
        ),
        (
            0.0,
            70.0,
            0.6,
            [
                3.0290270824033776e-12,
                5.138709323537502e-12,
                5.8680016169128165e-12,
            ],
        ),
        (
            1.0,
            -40.0,
            -0.4,
            [
                1.6136286810736238e-12,
                2.4816763819689538e-12,
                3.896123393697486e-12,
            ],
        ),
        (
            1.0,
            50.0,
            0.1,
            [
                2.097294565936023e-12,
                3.202170347382517e-12,
                5.126373626373625e-12,
            ],
        ),
        (
            1.0,
            70.0,
            0.6,
            [
                3.030128265617304e-12,
                4.791337291014488e-12,
                6.0476348993288594e-12,
            ],
        ),
        (
            1.0,
            125.0,
            0.1,
            [2.290754052646454e-12, 3.029901559798528e-12, 5.93184375e-12],
        ),
    ] {
        let config = SimulationConfig {
            spice_dialect: SpiceDialect::Ngspice,
            convergence_config: ConvergenceConfig {
                gmin_target: 0.0,
                junction_gmin_target: 0.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let engine = Engine::new(config.clone());
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            for subs in [1, -1] {
                let common = format!(
                    "IS=0 SUBS={subs} TNOM=50 FC=.4 TLEVC={law} CTE={{ct}} CTC=-.0007 CTS=.002 TVJE=.001 TVJC=-.0005 TVJS=.0008 TMJE1=.002 TMJE2=.00001 TMJC1=-.001 TMJC2=.00002 TMJS1=.003 TMJS2=-.00001"
                );
                let models = format!(
                    ".model me {kind}({common} CJE=2p VJE=.83 MJE=.37)\n\
                     .model mc {kind}({common} CJC=3p VJC=.68 MJC=.41)\n\
                     .model ms {kind}({common} CJS=5p VJS=.91 MJS=.23)\n"
                );
                let devices = "QBE 0 be 0 0 me AREA=2 AREAB=3 AREAC=5 M=4\nQBC 0 bc 0 0 mc AREA=2 AREAB=3 AREAC=5 M=4\nQSC 0 0 0 sc ms AREA=2 AREAB=3 AREAC=5 M=4\n";
                let body = if subs == -1 {
                    // Scoped coefficients and nominal parameter aliases must
                    // reach the same physical model after expansion.
                    let models = models
                        .replace(" VJE=", " PE=")
                        .replace(" VJC=", " PC=")
                        .replace(" VJS=", " PSUB=")
                        .replace(" MJE=", " ME=")
                        .replace(" MJC=", " MC=")
                        .replace(" MJS=", " ESUB=")
                        .replace(" CJS=", " CSUB=");
                    format!(
                        ".param ct=9\nX1 be bc sc cell ct=.001\n.subckt cell be bc sc ct=7\n{devices}{models}.ends\n"
                    )
                } else {
                    format!(".param ct=.001\n{devices}{models}")
                };
                let voltage = polarity * bias;
                let substrate_voltage = f64::from(subs) * voltage;
                let text = format!(
                    "GP capacitance controls\nVBE be 0 DC {voltage} AC 1\nVBC bc 0 DC {voltage} AC 1\nVSC sc 0 DC {substrate_voltage} AC 1\n{body}.temp {temperature}\n.end"
                );
                let ac = engine
                    .run_ac(&Netlist::parse(&text).unwrap(), &[1e6])
                    .unwrap();
                let geometry = if subs == 1 {
                    [8.0, 12.0, 20.0]
                } else {
                    [8.0, 20.0, 12.0]
                };
                for ((name, expected), scale) in ["VBE", "VBC", "VSC"]
                    .into_iter()
                    .zip(expected)
                    .zip(geometry)
                {
                    let column = ac[0]
                        .branch_names
                        .iter()
                        .position(|branch| branch.eq_ignore_ascii_case(name))
                        .unwrap();
                    let current = ac[0].currents[column];
                    let actual = -current.im / (std::f64::consts::TAU * 1e6 * scale);
                    // Only the physical law uses ngspice's older k/q constants.
                    let tolerance = if law == 0.0 { 3e-8 } else { 2e-12 };
                    assert!(
                        (actual - expected).abs() < expected * tolerance,
                        "{kind} SUBS={subs} TLEVC={law} TEMP={temperature} {name}: {actual:e} vs {expected:e}"
                    );
                    assert_eq!(current.re, 0.0);
                }
                if law == 1.0 && temperature == 70.0 && polarity == 1.0 {
                    let mut transient_config = config.clone();
                    transient_config.integration_method =
                        rspice_core::numerics::integration::IntegrationMethod::BackwardEuler;
                    transient_config.locked_time_grid = Some(std::sync::Arc::new(
                        (0..=20).map(|i| f64::from(i) * 1e-8).collect(),
                    ));
                    let ramp = text
                        .replace(
                            &format!("VSC sc 0 DC {substrate_voltage} AC 1"),
                            &format!(
                                "VSC sc 0 PWL(0 {} 200n {})",
                                -0.4 * f64::from(subs),
                                0.6 * f64::from(subs)
                            ),
                        )
                        .replace(&format!("DC {voltage} AC 1"), "PWL(0 -.4 200n .6)");
                    let result = Engine::new(transient_config)
                        .run_tran(&Netlist::parse(&ramp).unwrap(), 200e-9, 10e-9)
                        .unwrap();
                    // Integrate terminal current, then compare to the analytic
                    // depletion-charge integral across both continuation joins.
                    for (((name, scale), cap), (potential, grading, fc)) in ["VBE", "VBC", "VSC"]
                        .into_iter()
                        .zip(geometry)
                        .zip([2.04e-12, 2.958e-12, 5.2e-12])
                        .zip([
                            (0.81_f64, 0.38628_f64, 0.4),
                            (0.69, 0.40508, 0.4),
                            (0.894, 0.24288, 0.0),
                        ])
                    {
                        let charge = |v: f64| {
                            let join = fc * potential;
                            let x = v.min(join);
                            let q =
                                cap * potential * (1.0 - (1.0 - x / potential).powf(1.0 - grading))
                                    / (1.0 - grading);
                            let overdrive = (v - join).max(0.0);
                            q + cap
                                * (1.0 - fc).powf(-grading)
                                * overdrive
                                * (1.0 + 0.5 * grading * overdrive / (potential * (1.0 - fc)))
                        };
                        let currents = result.try_branch_current_waveform_named(name).unwrap();
                        let integral: f64 = result
                            .time
                            .windows(2)
                            .zip(currents.iter().skip(1))
                            .map(|(t, i)| -i * (t[1] - t[0]) / scale)
                            .sum();
                        let integral = integral * if name == "VSC" { f64::from(subs) } else { 1.0 };
                        let expected = charge(0.6) - charge(-0.4);
                        assert!(
                            (integral - expected).abs() < expected * 1e-8,
                            "SUBS={subs} {name}: integrated charge {integral:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_capacitance_temperature_controls_validate_operating_domains() {
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let engine = Engine::new(config.clone());
    for value in [-1.0, 2.0, 0.5, f64::NAN, f64::INFINITY] {
        let mut deck =
            Netlist::parse("Invalid capacitance law\nQ1 0 0 0 qm\n.model qm NPN\n.end").unwrap();
        deck.models[0].params.push(("TLEVC".into(), value));
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains("TLEVC")
        );
    }
    for name in [
        "TLEVC", "CTE", "CTC", "CTS", "TVJE", "TVJC", "TVJS", "TMJE1", "TMJE2", "TMJC1", "TMJC2",
        "TMJS1", "TMJS2",
    ] {
        let mut deck =
            Netlist::parse("Invalid charge coefficient\nQ1 0 0 0 qm\n.model qm NPN\n.end").unwrap();
        deck.models[0].params.push((name.into(), f64::NAN));
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(name)
        );
        for (dialect, family) in [
            (SpiceDialect::Xyce, "LEVEL=1"),
            (SpiceDialect::Ngspice, "TNF=0"),
        ] {
            let deck = Netlist::parse(&format!(
                "Wrong charge family\nQ1 0 0 0 qm\n.model qm NPN({family} {name}=0)\n.end"
            ))
            .unwrap();
            assert!(
                Engine::new(config.clone().with_spice_dialect(dialect))
                    .run_dc_op(&deck)
                    .unwrap_err()
                    .to_string()
                    .contains(name)
            );
        }
    }
    for (fields, expected) in [
        ("CJE=1p CTE=-1", "CTE"),
        ("CJC=1p CTC=-1", "CTC"),
        ("CJS=1p CTS=-1", "CTS"),
        ("CJE=1p TVJE=1", "TVJE"),
        ("CJC=1p TVJC=1", "TVJC"),
        ("CJS=1p TVJS=1", "TVJS"),
        ("TMJE2=1e308", "TMJE"),
        ("TMJC2=1e308", "TMJC"),
        ("TMJS2=1e308", "TMJS"),
        ("VJE=0", "VJE"),
        ("PC=-1", "PC"),
        ("PSUB=0", "PSUB"),
        ("CJE=-1", "CJE"),
        ("CSUB=-1", "CSUB"),
    ] {
        let deck = Netlist::parse(&format!(
            "Invalid mapped charge\nQ1 0 0 0 qm TEMP=29\n.model qm NPN(TLEVC=1 {fields})\n.end"
        ))
        .unwrap();
        assert!(
            engine
                .run_dc_op(&deck)
                .unwrap_err()
                .to_string()
                .contains(expected),
            "{fields}"
        );
    }
    // Zero coefficients select constant C/P; finite large grading follows
    // ngspice's 0.999 limit, including the substrate (default MJS is zero).
    let make = |fields: &str| {
        Netlist::parse(&format!("Grading limit\nVB b 0 .1 AC 1\nVS s 0 -.2 AC 1\nQ1 0 b 0 s qm\n.model qm NPN(IS=0 CJE=2p CJC=3p CJS=5p {fields})\n.end")).unwrap()
    };
    let mapped = engine
        .run_ac(&make("TLEVC=1 MJE=1 MJC=2 MJS=3"), &[1e6])
        .unwrap();
    let reference = engine
        .run_ac(&make("MJE=.999 MJC=.999 MJS=.999"), &[1e6])
        .unwrap();
    for (actual, expected) in mapped[0].currents.iter().zip(&reference[0].currents) {
        assert!((actual - expected).norm() <= expected.norm() * 1e-12);
    }
}

#[test]
fn legacy_junction_capacitance_temperature_matches_spice_references() {
    use SpiceDialect::{Ngspice, Xyce};
    // Independent ngspice 46 AC measurements (IS=TF=TR=0), in farads.
    // Xyce values are derived from N_DEV_BJT.C in the 7.10 source:
    // operating pbfact is also used in the nominal inversion, and CJS/VJS
    // remain nominal. Biases avoid its inconsistent nominal BC join.
    for (dialect, tnom, temperature, bias, expected) in [
        (
            Ngspice,
            27.0,
            -40.0,
            -0.4,
            [
                1.6592826985669354e-12,
                2.3187629982831146e-12,
                4.504768623867699e-12,
            ],
        ),
        (
            Xyce,
            27.0,
            -40.0,
            -0.4,
            [
                1.8462098259680166e-12,
                2.7014861553409653e-12,
                4.598086389211355e-12,
            ],
        ),
        (Ngspice, 27.0, 27.0, 0.0, [2e-12, 3e-12, 5e-12]),
        (Xyce, 27.0, 27.0, 0.0, [2e-12, 3e-12, 5e-12]),
        (
            Ngspice,
            27.0,
            70.0,
            0.1,
            [
                2.1805869065275923e-12,
                3.4267504027453357e-12,
                5.223725774666841e-12,
            ],
        ),
        (
            Xyce,
            27.0,
            70.0,
            0.1,
            [
                1.9906878749375348e-12,
                3.017356268296936e-12,
                5.126373626373625e-12,
            ],
        ),
        (
            Ngspice,
            27.0,
            125.0,
            0.6,
            [
                3.3938210663884363e-12,
                6.647798973415988e-12,
                6.11777066335303e-12,
            ],
        ),
        (
            Xyce,
            27.0,
            125.0,
            0.6,
            [
                2.30151257189782e-12,
                3.724231648825904e-12,
                5.758241758241757e-12,
            ],
        ),
        (
            Ngspice,
            50.0,
            27.0,
            -0.4,
            [
                1.707690473743761e-12,
                2.4362058346267156e-12,
                4.568111350869261e-12,
            ],
        ),
        (
            Xyce,
            50.0,
            27.0,
            -0.4,
            [
                1.7572808847689595e-12,
                2.523123224854252e-12,
                4.598086389211355e-12,
            ],
        ),
        (
            Ngspice,
            50.0,
            -40.0,
            0.0,
            [
                1.8724352634309237e-12,
                2.6888696309844536e-12,
                4.839825827878507e-12,
            ],
        ),
        (
            Xyce,
            50.0,
            -40.0,
            0.0,
            [2.2530437202648688e-12, 3.490055296764453e-12, 5e-12],
        ),
        (
            Ngspice,
            50.0,
            70.0,
            0.6,
            [
                2.978859243920323e-12,
                5.169633039750461e-12,
                5.821664385549351e-12,
            ],
        ),
        (
            Xyce,
            50.0,
            70.0,
            0.6,
            [
                2.763039693056152e-12,
                4.648400104490727e-12,
                5.758241758241757e-12,
            ],
        ),
        (
            Ngspice,
            50.0,
            50.0,
            0.1,
            [
                2.097294565936023e-12,
                3.202170347382517e-12,
                5.126373626373625e-12,
            ],
        ),
        (
            Xyce,
            50.0,
            50.0,
            0.1,
            [
                2.097294565936024e-12,
                3.202170347382517e-12,
                5.126373626373625e-12,
            ],
        ),
    ] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            for m in [2.0, 2e-20] {
                let voltage = p * bias;
                let deck = Netlist::parse(&format!(
                    "GP junction temperature\nVBE be 0 DC {voltage} AC 1\nVBC bc 0 DC {voltage} AC 1\nVSC sc 0 DC {voltage} AC 1\n\
                     QBE 0 be 0 0 me AREA=3 M={m}\nQBC 0 bc 0 0 mc AREA=3 M={m}\nQSC 0 0 0 sc ms AREA=3 M={m}\n\
                     .model me {kind}(IS=0 SUBS=1 CJE=2p VJE=.83 MJE=.37 FC=.4 TNOM={tnom})\n\
                     .model mc {kind}(IS=0 SUBS=1 CJC=3p VJC=.68 MJC=.41 FC=.4 TNOM={tnom})\n\
                     .model ms {kind}(IS=0 SUBS=1 CJS=5p VJS=.91 MJS=.23 FC=.4 TNOM={tnom})\n\
                     .temp {temperature}\n.options GMIN=0\n.end\n"
                )).unwrap();
                let ac = engine.run_ac(&deck, &[1e6]).unwrap();
                for (name, expected) in ["VBE", "VBC", "VSC"].into_iter().zip(expected) {
                    let column = ac[0]
                        .branch_names
                        .iter()
                        .position(|branch| branch.eq_ignore_ascii_case(name))
                        .unwrap();
                    let current = ac[0].currents[column];
                    let actual = -current.im / (std::f64::consts::TAU * 1e6 * 3.0 * m);
                    // Ngspice uses older k/q constants; RSpice intentionally
                    // uses current SI constants outside the Xyce dialect.
                    let tolerance = if dialect == Ngspice { 3e-8 } else { 2e-12 };
                    assert!(
                        (actual - expected).abs() < expected * tolerance,
                        "{dialect:?} {kind} {name} TNOM={tnom} TEMP={temperature} V={bias} M={m}: {actual:e} vs {expected:e}"
                    );
                    assert_eq!(current.re, 0.0);
                }
            }
        }
    }
}

#[test]
fn legacy_gmin_placement_and_multiplicity_match_spice_dialects() {
    for dialect in [
        SpiceDialect::BestAvailable,
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
    ] {
        for gmin in [1e-12, 1e-3] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.convergence_config.gmin_target = 0.0;
            config.convergence_config.junction_gmin_target = gmin;
            let engine = Engine::new(config);
            for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
                for area in [1.0, 5.0] {
                    for m in [1e-20, 0.5, 3.0] {
                        // Tie the substrate to its collector connection to
                        // isolate BE/BC GMIN. IS=0 eliminates physical current.
                        let deck = Netlist::parse(&format!(
                            "GP GMIN\nVC c 0 {}\nVB b 0 DC {} AC 1\nQ1 c b 0 c mm AREA={area} M={m}\n.model mm {kind}(IS=0 SUBS=1 BF=100 BR=2 TF=1n TR=2n)\n.end\n",p,p*0.1)).unwrap();
                        let dc = engine.run_dc_op(&deck).unwrap();
                        let ac = engine.run_ac(&deck, &[1e6]).unwrap();
                        let g = gmin * m;
                        let omega = std::f64::consts::TAU * 1e6;
                        // Ngspice 46 binary captures: leakage parallels.
                        // Xyce 7.10 N_DEV_BJT.C: GMIN enters transport and
                        // diffusion charge, then BF/BR divide base currents.
                        let expected = if dialect == SpiceDialect::Xyce {
                            [
                                ("VB", 0.449 * g, -0.51 * g, -omega * 3e-9 * g),
                                ("VC", -1.45 * g, 0.5 * g, omega * 2e-9 * g),
                            ]
                        } else {
                            [("VB", 0.8 * g, -2.0 * g, 0.0), ("VC", -0.9 * g, g, 0.0)]
                        };
                        for (branch, current, re, im) in expected {
                            let actual = dc.branch_current_named(branch).unwrap();
                            assert!(
                                (actual - p * current).abs() < g * 2e-11,
                                "{dialect:?} {kind} AREA={area} M={m} GMIN={gmin} DC {branch}: {actual:e} vs {:e}",
                                p * current
                            );
                            let index = ac[0]
                                .branch_names
                                .iter()
                                .position(|name| name.eq_ignore_ascii_case(branch))
                                .unwrap();
                            let actual = ac[0].currents[index];
                            assert!(
                                (actual.re - re).abs() < g * 2e-11
                                    && (actual.im - im).abs() < g * 2e-11,
                                "{dialect:?} {kind} AREA={area} M={m} GMIN={gmin} AC {branch}: {actual:?} vs ({re:e},{im:e})"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn ngspice_gp_gmin_substrate_and_series_network_match_explicit_resistors() {
    let gmin = 1e-3;
    let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = gmin;
    let engine = Engine::new(config);
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        for subs in [1, -1] {
            for series in [false, true] {
                for (area, m) in [(1.0, 1.0), (5.0, 0.25), (2.0, 3.0)] {
                    let (rc, rb, re, rbm) = if series {
                        (20.0, 30.0, 10.0, 10.0)
                    } else {
                        (0.0, 0.0, 0.0, 0.0)
                    };
                    let sources = |vb: f64| {
                        format!(
                            "GP resistor equivalent\nVC c 0 {} AC .3\nVB b 0 DC {vb} AC 1\nVE e 0 {} AC .2\nVS s 0 {} AC .4\n",
                            p,
                            p * (-0.05),
                            p * (-0.2)
                        )
                    };
                    let actual = format!(
                        "{}Q1 c b e s mm AREA={area} M={m}\n.model mm {kind}(IS=0 SUBS={subs} TF=1n TR=2n RC={rc} RB={rb} RBM={rbm} RE={re})\n.end\n",
                        sources(p * 0.1)
                    );
                    let (ci, bi, ei) = if series {
                        ("ci", "bi", "ei")
                    } else {
                        ("c", "b", "e")
                    };
                    let connection = if subs == 1 { ci } else { bi };
                    let rg = 1.0 / (gmin * m);
                    let mut resistors = format!(
                        "Rbe {bi} {ei} {rg}\nRbc {bi} {ci} {rg}\nRsub s {connection} {rg}\n"
                    );
                    if series {
                        resistors.push_str(&format!(
                            "Rc c ci {}\nRb b bi {}\nRe e ei {}\n",
                            rc / (area * m),
                            rb / (area * m),
                            re / (area * m)
                        ));
                    }
                    let reference =
                        Netlist::parse(&format!("{}{resistors}.end\n", sources(p * 0.1))).unwrap();
                    let deck = Netlist::parse(&actual).unwrap();
                    let expected_dc = engine.run_dc_op(&reference).unwrap();
                    let actual_dc = engine.run_dc_op(&deck).unwrap();
                    let expected_ac = engine.run_ac(&reference, &[1e6]).unwrap();
                    let actual_ac = engine.run_ac(&deck, &[1e6]).unwrap();
                    for branch in ["VC", "VB", "VE", "VS"] {
                        let a = actual_dc.branch_current_named(branch).unwrap();
                        let b = expected_dc.branch_current_named(branch).unwrap();
                        assert!(
                            (a - b).abs() < 1e-11 * m,
                            "{kind} SUBS={subs} series={series} AREA={area} M={m} DC {branch}: {a:e} vs {b:e}"
                        );
                        let current = |ac: &rspice_core::analysis::ac::AcResult| {
                            ac.currents[ac
                                .branch_names
                                .iter()
                                .position(|name| name.eq_ignore_ascii_case(branch))
                                .unwrap()]
                        };
                        let a = current(&actual_ac[0]);
                        let b = current(&expected_ac[0]);
                        assert!(
                            (a - b).norm() < 1e-11 * m,
                            "{kind} SUBS={subs} series={series} AREA={area} M={m} AC {branch}: {a:?} vs {b:?}"
                        );
                    }
                    if series && area == 2.0 && p == 1.0 {
                        // With IS=0, GMIN must not create a transient storage
                        // term. A linear ramp follows the resistive endpoints.
                        let final_reference =
                            Netlist::parse(&format!("{}{resistors}.end\n", sources(p * 0.3)))
                                .unwrap();
                        let final_dc = engine.run_dc_op(&final_reference).unwrap();
                        let ramp = Netlist::parse(
                            &actual.replace("DC 0.1 AC 1", "PWL(0 0.1 20n 0.3) AC 1"),
                        )
                        .unwrap();
                        let tran = engine.run_tran(&ramp, 20e-9, 1e-9).unwrap();
                        assert!(tran.time.len() > 3);
                        for branch in ["VC", "VB", "VE", "VS"] {
                            let start = expected_dc.branch_current_named(branch).unwrap();
                            let end = final_dc.branch_current_named(branch).unwrap();
                            let values = tran.try_branch_current_waveform_named(branch).unwrap();
                            for (&time, &actual) in tran.time.iter().zip(values) {
                                let expected =
                                    start + (end - start) * (time / 20e-9).clamp(0.0, 1.0);
                                assert!(
                                    (actual - expected).abs() < 1e-10 * m,
                                    "SUBS={subs} transient {branch} at {time:e}: {actual:e} vs {expected:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_private_base_ac_reduction_matches_explicit_rc_network() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        for kind in ["NPN", "PNP"] {
            for subs in [1, -1] {
                for (area, m) in [(1.0, 1.0), (5.0, 0.25), (1.0, 1e-20), (5.0, 1e-200)] {
                    // Grading zero makes these pure capacitors. RBM<RB keeps
                    // a private base node; IS=0 and no Early/knee terms make
                    // the base resistance constant, so the RC oracle is exact.
                    let sources = "Private BJT AC\nVC c 0 0 AC .3\nVB b 0 0 AC 1\nVE e 0 0 AC .2\nVS s 0 0 AC .4\n";
                    let actual = Netlist::parse(&format!("{sources}Q1 c b e s mm AREA={area} M={m}\n.model mm {kind}(IS=0 SUBS={subs} RC=2k RB=5k RBM=1k RE=1k CJE=1p CJC=2p CJS=3p MJE=0 MJC=0 MJS=0)\n.end\n")).unwrap();
                    let scale = area * m;
                    let connection = if subs == 1 { "ci" } else { "bi" };
                    let reference = Netlist::parse(&format!("{sources}RC c ci {}\nRB b bi {}\nRE e ei {}\nCBE bi ei {}\nCBC bi ci {}\nCS s {connection} {}\n.end\n",2e3/scale,5e3/scale,1e3/scale,1e-12*scale,2e-12*scale,3e-12*scale)).unwrap();
                    let frequencies = [1e3, 1e6, 1e9];
                    let expected = engine.run_ac(&reference, &frequencies).unwrap();
                    let actual = engine.run_ac(&actual, &frequencies).unwrap();
                    for (point, (actual, expected)) in actual.iter().zip(&expected).enumerate() {
                        for branch in ["VC", "VB", "VE", "VS"] {
                            let get = |result: &rspice_core::analysis::ac::AcResult| {
                                result.currents[result
                                    .branch_names
                                    .iter()
                                    .position(|name| name.eq_ignore_ascii_case(branch))
                                    .unwrap()]
                            };
                            let a = get(actual) / scale;
                            let b = get(expected) / scale;
                            assert!(
                                (a - b).norm() < 2e-11 * b.norm().max(1e-12),
                                "{dialect:?} {kind} SUBS={subs} AREA={area} M={m} f={} {branch}: {a:?} vs {b:?}",
                                frequencies[point]
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_private_base_ac_reduction_conserves_nonlinear_terminal_current() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for gmin in [0.0, 1e-3] {
            let mut config = SimulationConfig::default().with_spice_dialect(dialect);
            config.convergence_config.gmin_target = 0.0;
            config.convergence_config.junction_gmin_target = gmin;
            let engine = Engine::new(config);
            for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
                for subs in [1, -1] {
                    let deck=Netlist::parse(&format!("Private BJT KCL\nVC c 0 {p}\nVB b 0 DC {} AC 1\nVE e 0 0\nVS s 0 {}\nQ1 c b e s mm AREA=5 M=3\n.model mm {kind}(IS=1e-14 BF=100 BR=2 VAF=40 VAR=20 IKF=1m IKR=2m SUBS={subs} RC=20 RB=30 RBM=10 RE=10 CJE=1p CJC=2p CJS=3p TF=1n TR=2n)\n.end\n",p*0.65,p*(-0.2))).unwrap();
                    for ac in engine.run_ac(&deck, &[1.0, 1e3, 1e6, 1e9]).unwrap() {
                        // Sum the four terminal probes, excluding independent
                        // currents inside the promoted device (such as RBI).
                        let terminals = ["VC", "VB", "VE", "VS"].map(|name| {
                            let index = ac
                                .branch_names
                                .iter()
                                .position(|branch| branch.eq_ignore_ascii_case(name))
                                .unwrap();
                            ac.currents[index]
                        });
                        let sum = terminals.iter().copied().sum::<rspice_core::Complex64>();
                        let scale = terminals.iter().map(|i| i.norm()).sum::<f64>();
                        assert!(
                            sum.norm() < 2e-11 * scale,
                            "{dialect:?} {kind} SUBS={subs} GMIN={gmin}: terminal sum={sum:?}, scale={scale}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn private_bjt_small_instance_dc_and_ac_match_parallel_scaling() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        let make = |m| {
            Netlist::parse(&format!("Private BJT scaling\nVC c 0 1\nVB b 0 DC .7 AC 1\nQ1 c b 0 mm M={m}\n.model mm NPN(IS=1e-14 BF=100 RB=5k RBM=1k CJE=1p CJC=2p TF=1n)\n.end\n")).unwrap()
        };
        let reference = make(1.0);
        let reference_dc = engine.run_dc_op(&reference).unwrap();
        let reference_ac = engine.run_ac(&reference, &[1e6]).unwrap();
        for m in [1e-20, 1e-100, 1e-200] {
            let deck = make(m);
            let dc = engine.run_dc_op(&deck).unwrap();
            let ac = engine.run_ac(&deck, &[1e6]).unwrap();
            for branch in ["VC", "VB"] {
                let a = dc.branch_current_named(branch).unwrap() / m;
                let b = reference_dc.branch_current_named(branch).unwrap();
                // Xyce's outer DC update uses its fixed 1e-3 relative gate.
                // The private-voltage unit separately requires 2e-12 V;
                // do not mistake the returned MNA iterate for that root.
                let dc_tolerance = if dialect == SpiceDialect::Xyce {
                    1e-6
                } else {
                    2e-8
                };
                assert!(
                    (a - b).abs() < dc_tolerance * b.abs(),
                    "{dialect:?} M={m} {branch} DC {a:e} vs {b:e}"
                );
                let index = ac[0]
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(branch))
                    .unwrap();
                let a = ac[0].currents[index] / m;
                let b = reference_ac[0].currents[index];
                assert!(
                    (a - b).norm() < 2e-8 * b.norm(),
                    "{dialect:?} M={m} {branch} AC {a:?} vs {b:?}"
                );
            }
        }
    }
}

#[test]
fn private_bjt_ac_reduction_errors_instead_of_dropping_invalid_charge() {
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    let engine = Engine::new(config);
    let deck=Netlist::parse("Invalid private BJT AC\nVC c 0 0\nVB b 0 0 AC 1\nQ1 c b 0 mm\n.model mm NPN(IS=0 RB=1k RBM=100 CJE=1e308 MJE=0)\n.end\n").unwrap();
    let error = engine.run_ac(&deck, &[1e6]).unwrap_err().to_string();
    assert!(
        error.contains("Q1") && error.contains("AC charge"),
        "{error}"
    );
}

#[test]
fn private_bjt_transient_small_instances_preserve_rc_response() {
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let mut config = SimulationConfig::default().with_spice_dialect(dialect);
        config.convergence_config.gmin_target = 0.0;
        config.convergence_config.junction_gmin_target = 0.0;
        let engine = Engine::new(config);
        for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
            for parameter in ["M", "AREA"] {
                for scale in [1.0, 1e-20, 1e-200] {
                    let deck = Netlist::parse(&format!("Private BJT ramp\nVB b 0 PWL(0 0 50n {})\nQ1 0 b 0 mm {parameter}={scale}\n.model mm {kind}(IS=0 RB=5k RBM=1k CJE=1p CJC=2p MJE=0 MJC=0)\n.end\n",p*0.001)).unwrap();
                    let tran = engine.run_tran(&deck, 50e-9, 0.05e-9).unwrap();
                    assert!(tran.time.len() > 3);
                    assert_eq!(*tran.time.last().unwrap(), 50e-9);
                    let currents = tran.try_branch_current_waveform_named("VB").unwrap();
                    for (&time, &current) in tran.time.iter().zip(currents) {
                        // R=5k/scale and C=3p*scale: tau=15ns, independent
                        // of instance size. Ramp slope is +/-20,000 V/s.
                        let expected = p * 6e-8 * (-time / 15e-9).exp_m1();
                        let actual = current / scale;
                        assert!(
                            (actual - expected).abs() < 6e-12,
                            "{dialect:?} {kind} {parameter}={scale:e} t={time:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn private_bjt_charge_solver_cannot_be_disabled_by_obsolete_flags() {
    let executable = std::env::current_exe().expect("integration test executable");
    for flag in [
        "RSPICE_LEGACY_BJT_BACKEND",
        "RSPICE_EXPERIMENTAL_NGSPICE_BJT",
    ] {
        // A separate process isolates the environment and the old OnceLock.
        // Reuse the physical RC curve across dialects, polarities and sizes.
        let output = std::process::Command::new(&executable)
            .args([
                "--exact",
                "private_bjt_transient_small_instances_preserve_rc_response",
                "--nocapture",
            ])
            .env_remove("RSPICE_LEGACY_BJT_BACKEND")
            .env_remove("RSPICE_EXPERIMENTAL_NGSPICE_BJT")
            .env(flag, "0")
            .output()
            .expect("run isolated private-charge regression");
        assert!(
            output.status.success()
                && String::from_utf8_lossy(&output.stdout).contains("test result: ok. 1 passed"),
            "{flag}=0 changed the physical RC response:\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}

#[test]
fn xyce_private_bjt_transient_matches_explicit_base_resistor() {
    let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    config.convergence_config.voltage_abstol = 1e-12;
    config.convergence_config.voltage_reltol = 1e-9;
    config.convergence_config.current_abstol = 1e-220;
    config.convergence_config.residual_reltol = 1e-9;
    config.transient_nonlinear_abstol = Some(1e-12);
    config.transient_nonlinear_reltol = Some(1e-9);
    // The promoted RBI constitutive equation is in volts. RHSTOL below its
    // floating-point rounding error demands accidental exact cancellation;
    // voltage/update tolerances and the physical current assertions below
    // qualify accuracy independently of instance size.
    config.transient_nonlinear_rhstol = Some(1e-12);
    for nox in [false, true] {
        config.transient_nonlinear_nox = Some(nox);
        let engine = Engine::new(config.clone());
        for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
            // Start at exact zero bias: this qualifies transient integration,
            // independently of Xyce's intentionally coarser DC stopping rule.
            let sources = format!(
                "Private nonlinear BJT\nVC c 0 {polarity}\nVB b 0 PWL(0 {} 50n {})\n",
                0.0,
                polarity * 0.7
            );
            // With no Early or high-injection effects, RB remains 5k even
            // though RBM<RB selects a private base. Diffusion and depletion
            // charge still depend nonlinearly on the evolving private voltage.
            let model = "IS=1e-14 BF=100 CJE=1p CJC=2p TF=1n";
            // Use the independently resolved unit RC/transistor circuit for every
            // scale, so its outer MNA current weights do not change with the
            // private instance being qualified.
            let explicit = Netlist::parse(&format!(
                "{sources}RB b bi 5k\nQ1 c bi 0 mm\n.model mm {kind}({model})\n.end\n"
            ))
            .unwrap();
            let expected = engine.run_tran(&explicit, 100e-9, 0.05e-9).unwrap();
            for scale in [1.0, 1e-20, 1e-200] {
                let private = Netlist::parse(&format!(
                    "{sources}Q1 c b 0 mm M={scale}\n.model mm {kind}({model} RB=5k RBM=1k)\n.end\n"
                ))
                .unwrap();
                let resistance = 5e3 / scale;
                let scaled_explicit = Netlist::parse(&format!(
                    "{sources}RB b bi {resistance}\nQ1 c bi 0 mm M={scale}\n.model mm {kind}({model})\n.end\n"
                ))
                .unwrap();
                for (form, deck) in [("private", &private), ("explicit", &scaled_explicit)] {
                    let actual = engine
                        .run_tran(deck, 100e-9, 0.05e-9)
                        .unwrap_or_else(|e| panic!("NOX={nox} {form} {kind} M={scale:e}: {e}"));
                    assert_eq!(actual.time, expected.time);
                    for branch in ["VB", "VC"] {
                        let a = actual.try_branch_current_waveform_named(branch).unwrap();
                        let b = expected.try_branch_current_waveform_named(branch).unwrap();
                        for ((&time, &a), &b) in actual.time.iter().zip(a).zip(b) {
                            assert!(
                                (a / scale - b).abs() < 1e-11,
                                "NOX={nox} {form} {kind} M={scale:e} {branch} t={time:e}: {a:e} vs {b:e}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn private_bjt_transient_reports_unresolvable_charge_instead_of_using_dc() {
    let mut config = SimulationConfig::default();
    config.convergence_config.gmin_target = 0.0;
    config.convergence_config.junction_gmin_target = 0.0;
    let deck = Netlist::parse("Invalid private BJT transient\nVB b 0 PWL(0 0 1n .001)\nQ1 0 b 0 mm\n.model mm NPN(IS=0 RB=1k RBM=100 CJE=1e308 MJE=0)\n.end\n").unwrap();
    let error = Engine::new(config)
        .run_tran(&deck, 1e-9, 1e-12)
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Q1") && error.contains("transient charge"),
        "{error}"
    );
}

#[test]
fn legacy_base_resistance_temperature_matches_ngspice_dc_ac_noise_and_transient() {
    use rspice_core::Complex64;
    // Warning-free ngspice46 outputs at -40/70 C. Columns: DC IC/IB,
    // complex V(base)/IC at 1MHz, total/RB noise amplitudes at 1MHz.
    let references = [
        [
            [
                -1.3547813045592424e-05,
                -1.7377877205900644e-07,
                0.9986674889676806,
                -0.0207220433540703,
                -0.000670235368586839,
                0.00011074645695757605,
                1.1344844652024594e-09,
                7.04951921840306e-12,
            ],
            [
                -0.022818273769924347,
                -0.00029202644306478703,
                0.4293406586213592,
                -0.18184025655053856,
                -0.29263787403366176,
                0.15199161269456854,
                1.0851571084413812e-09,
                2.417345982809464e-10,
            ],
        ],
        [
            [
                -1.3546713703094801e-05,
                -1.7376465033872024e-07,
                0.9986478063388414,
                -0.020718643808600924,
                -0.0006700424333023116,
                0.00011165757418738491,
                1.1344731437934336e-09,
                8.687764628421081e-12,
            ],
            [
                -0.021670576304192835,
                -0.00027730790788946073,
                0.4721158522445638,
                -0.16329299723213767,
                -0.2684373737785993,
                0.14281121345086936,
                1.1047494408739524e-09,
                3.3843614937214943e-10,
            ],
        ],
        [
            [
                -1.35475633298367e-05,
                -1.737755643115707e-07,
                0.9986630172813276,
                -0.02072127586528824,
                -0.0006701917607594112,
                0.00011095346181328234,
                1.1344818932804378e-09,
                7.453353285997418e-12,
            ],
            [
                -0.021642327862481103,
                -0.0002769456590465359,
                0.4731510221759345,
                -0.1628536962950831,
                -0.2678554476398883,
                0.1425852027934405,
                1.1052278106816368e-09,
                3.40251313789768e-10,
            ],
        ],
    ];
    let cases = [
        (
            "RB=120 RBM=20 TRB1=.002 TRB2=1e-5 TRM1=-.001 TRM2=1e-5 IRB=1e-5",
            20.0,
            [0.002, 1e-5],
            [-0.001, 1e-5],
            1e-5,
        ),
        (
            "RB=120 TRB=-.005 TRB2=1e-5 TRM1=.002 TRM2=-5e-6 IRB=1e-5",
            120.0,
            [-0.005, 1e-5],
            [0.002, -5e-6],
            1e-5,
        ),
        ("RB=120 RBM=180", 180.0, [0.0, 0.0], [0.0, 0.0], 0.0),
    ];
    let config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice);
    let engine = Engine::new(config.clone());
    for ((fields, rbm, rb_tc, rbm_tc, irb), reference) in cases.into_iter().zip(references) {
        for (temperature, reference) in [-40.0, 70.0].into_iter().zip(reference) {
            let dt = temperature - 27.0;
            let rb = 120.0 * (1.0 + dt * (rb_tc[0] + dt * rb_tc[1]));
            let rbm = rbm * (1.0 + dt * (rbm_tc[0] + dt * rbm_tc[1]));
            let mapped = format!("RB={rb} RBM={rbm} IRB={irb}");
            for (kind, polarity) in [("NPN", 1.0), ("PNP", -1.0)] {
                let make = |fields: &str| {
                    format!(
                        "Whole base resistance temperature\nVC c 0 {}\nVD drive 0 {} AC 1\nRIN drive b 100\nQ1 c b 0 qm AREA=2 M=3\n.model qm {kind}(IS=1e-14 BF=80 VAF=50 VAR=20 CJE=2p CJC=3p XCJC=.35 TF=2n KF=1e-12 AF=1 SUBS=1 {fields})\n.temp {temperature}\n.options GMIN=0 RELTOL=1e-10 ABSTOL=1e-18\n.end",
                        polarity,
                        polarity * 0.65,
                    )
                };
                let text = make(fields);
                let deck = Netlist::parse(&text).unwrap();
                let dc = engine.run_dc_op(&deck).unwrap();
                for (index, name) in ["VC", "VD"].into_iter().enumerate() {
                    let actual = dc.branch_current_named(name).unwrap();
                    let expected = polarity * reference[index];
                    assert!(
                        (actual / expected - 1.0).abs() < 5e-5,
                        "{kind} {fields} T={temperature} DC {name}: {actual:e} vs {expected:e}"
                    );
                }
                let ac = engine.run_ac(&deck, &[1e6]).unwrap();
                let base = ac[0]
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("b"))
                    .unwrap();
                let collector = ac[0]
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("VC"))
                    .unwrap();
                for (actual, pair) in [
                    (ac[0].voltages[base], &reference[2..4]),
                    (ac[0].currents[collector], &reference[4..6]),
                ] {
                    let expected = Complex64::new(pair[0], pair[1]);
                    assert!(
                        (actual - expected).norm() < expected.norm() * 5e-5,
                        "{kind} {fields} T={temperature} AC: {actual:?} vs {expected:?}"
                    );
                }
                let noise = engine
                    .run_noise_named_with_input_source(
                        &deck,
                        "b",
                        None,
                        "VD",
                        &[1e6],
                        temperature + 273.15,
                    )
                    .unwrap();
                let rb_noise: f64 = noise[0]
                    .contributions
                    .iter()
                    .filter(|s| {
                        s.identity.device.eq_ignore_ascii_case("Q1")
                            && s.identity.mechanism.as_deref() == Some("RB")
                    })
                    .map(|s| s.output_contribution)
                    .sum();
                for (actual, expected) in [
                    (noise[0].output_noise_density, reference[6].powi(2)),
                    (rb_noise, reference[7].powi(2)),
                ] {
                    assert!(
                        (actual / expected - 1.0).abs() < 5e-5,
                        "{kind} {fields} T={temperature} noise: {actual:e} vs {expected:e}"
                    );
                }
                if temperature == 70.0 {
                    let mut config = config.clone();
                    config.integration_method =
                        rspice_core::numerics::integration::IntegrationMethod::BackwardEuler;
                    config.locked_time_grid = Some(std::sync::Arc::new(
                        (0..=20).map(|i| f64::from(i) * 1e-9).collect(),
                    ));
                    let engine = Engine::new(config);
                    let ramp = |text: &str| {
                        Netlist::parse(&text.replace(
                            &format!("VD drive 0 {} AC 1", polarity * 0.65),
                            &format!(
                                "VD drive 0 PWL(0 {} 20n {})",
                                polarity * 0.65,
                                polarity * 0.67
                            ),
                        ))
                        .unwrap()
                    };
                    let actual = engine.run_tran(&ramp(&text), 20e-9, 1e-9).unwrap();
                    let equivalent = engine.run_tran(&ramp(&make(&mapped)), 20e-9, 1e-9).unwrap();
                    assert_eq!(actual.time, equivalent.time);
                    for name in ["VC", "VD"] {
                        for (a, b) in actual
                            .try_branch_current_waveform_named(name)
                            .unwrap()
                            .iter()
                            .zip(equivalent.try_branch_current_waveform_named(name).unwrap())
                        {
                            assert!((a - b).abs() <= b.abs() * 1e-8 + 1e-14);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn legacy_private_collector_preserves_external_bc_storage_and_checkpoint() {
    use rspice_core::engine::{
        TransientCheckpoint, TransientCheckpointEncoding, TransientStartupMode,
    };
    use rspice_core::numerics::integration::IntegrationMethod;
    let times: Vec<_> = (0..=20).map(|i| f64::from(i) * 5e-9).collect();
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
        ] {
            let engine = Engine::new(SimulationConfig {
                spice_dialect: dialect,
                integration_method: method,
                convergence_config: ConvergenceConfig {
                    gmin_target: 0.0,
                    junction_gmin_target: 0.0,
                    voltage_reltol: 1e-10,
                    voltage_abstol: 1e-12,
                    current_abstol: 1e-18,
                    ..Default::default()
                },
                locked_time_grid: Some(std::sync::Arc::new(times.clone())),
                ..Default::default()
            });
            for (kind, polarity, subs) in [("NPN", 1.0, 1), ("PNP", -1.0, -1)] {
                for (base, rb) in [("RB=120", 20.0), ("RBX=50 RBI=70", 20.0)] {
                    let sources = |suffix: &str| {
                        format!(
                            "VC{suffix} c{suffix} 0 PWL(0 0 100n {}) AC .3\nVB{suffix} b{suffix} 0 PWL(0 0 100n {}) AC 1\nVE{suffix} e{suffix} 0 0 AC .2\nVS{suffix} s{suffix} 0 PWL(0 0 100n {}) AC .4\n",
                            0.03 * polarity,
                            0.1 * polarity,
                            -0.02 * polarity,
                        )
                    };
                    // Zero grading and IS make an independent linear R/C oracle.
                    // The external BC branch bypasses RB, while CBE and CBC do not.
                    let substrate_connection = if subs == 1 { "ci" } else { "bi" };
                    let deck = Netlist::parse(&format!(
                        "Private collector charge\n{}{}Q1 c b e s qm AREA=2 M=3\n.model qm {kind}(LEVEL=1 IS=0 {base} RCX=60 RCI=120 RE=30 RS=90 CJE=1n CJC=2n CJS=3n MJE=0 MJC=0 MJS=0 XCJC=.25 SUBS={subs})\n\
                         RCref cr ci 30\nRBref br bi {rb}\nREref er ei 5\nRSref sr si 15\nCBE bi ei 6n\nCBC bi ci 3n\nCBX br ci 9n\nCS si {substrate_connection} 18n\n\
                         .save @Q1[ib] @Q1[ic] @Q1[ie] @Q1[is] I(VB) I(VC) I(VE) I(VS) I(VBr) I(VCr) I(VEr) I(VSr)\n.end",
                        sources(""), sources("r"),
                    )).unwrap();
                    let ac = engine.run_ac(&deck, &[1e3, 1e6, 1e9]).unwrap();
                    for point in &ac {
                        let current = |name: &str| {
                            point.currents[point
                                .branch_names
                                .iter()
                                .position(|n| n.eq_ignore_ascii_case(name))
                                .unwrap()]
                        };
                        for source in ["VB", "VC", "VE", "VS"] {
                            let actual = current(source);
                            let expected = current(&format!("{source}r"));
                            assert!(
                                (actual - expected).norm() < 1e-10 * expected.norm() + 1e-15,
                                "{dialect:?} {kind} {base} f={} {source}: {actual:?} vs {expected:?}",
                                point.frequency
                            );
                        }
                    }
                    let check_resume = dialect == SpiceDialect::Ngspice
                        && method == IntegrationMethod::Trapezoidal
                        && kind == "NPN"
                        && base == "RBX=50 RBI=70";
                    let (tran, checkpoints) = if check_resume {
                        engine
                            .run_tran_checkpoint_schedule_with_startup_mode(
                                &deck,
                                100e-9,
                                5e-9,
                                TransientStartupMode::OperatingPoint,
                                &[50e-9],
                            )
                            .unwrap()
                    } else {
                        (engine.run_tran(&deck, 100e-9, 5e-9).unwrap(), Vec::new())
                    };
                    for (source, parameter) in
                        [("VB", "IB"), ("VC", "IC"), ("VE", "IE"), ("VS", "IS")]
                    {
                        let actual = tran.try_branch_current_waveform_named(source).unwrap();
                        let expected = tran
                            .try_branch_current_waveform_named(&format!("{source}r"))
                            .unwrap();
                        let device = tran.try_device_op_waveform_named("Q1", parameter).unwrap();
                        for ((a, b), q) in actual.iter().zip(expected).zip(device) {
                            assert!(
                                (a - b).abs() < 1e-8 * b.abs() + 1e-12,
                                "{dialect:?} {method:?} {kind} {base} {source}: {a:e} vs {b:e}"
                            );
                            assert!(
                                (q + a).abs() < 1e-8 * a.abs() + 1e-12,
                                "{dialect:?} {method:?} {kind} {base} {parameter}: {q:e} vs source {a:e}"
                            );
                        }
                    }
                    if check_resume {
                        let checkpoint = TransientCheckpoint::from_bytes(
                            &checkpoints[0]
                                .checkpoint
                                .to_bytes(TransientCheckpointEncoding::Packed)
                                .unwrap(),
                        )
                        .unwrap();
                        let (resumed, _) = engine
                            .run_tran_resume(&deck, &checkpoint, 100e-9, 5e-9)
                            .unwrap();
                        let seam = tran
                            .time
                            .iter()
                            .position(|t| *t == resumed.time[0])
                            .unwrap();
                        assert_eq!(resumed.time, tran.time[seam..]);
                        for parameter in ["IB", "IC", "IE", "IS"] {
                            assert_eq!(
                                resumed
                                    .try_device_op_waveform_named("Q1", parameter)
                                    .unwrap(),
                                &tran.try_device_op_waveform_named("Q1", parameter).unwrap()
                                    [seam..]
                            );
                        }
                    }
                }
            }
        }
    }
}

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
        for devices in [
            "Q1 c b 0 qm M=3",
            "Q1 c b 0 qm AREA=3",
            "Q1 c b 0 qm AREA=1.5 M=2",
            "Q1 c b 0 qm\nQ2 c b 0 qm\nQ3 c b 0 qm",
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
                -3.0 * (conductance / 100.0 + 1e-16 / vt * ((0.65 - 0.72) / vt).exp()),
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
                let static_base =
                    3.0 * p * 1e-16 * ((vbe / vt).exp_m1() / 100.0 + ((vbe - 0.72) / vt).exp_m1());
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
                    let make = |parameter, scale| {
                        Netlist::parse(&format!(
                        "Small GP instance\nVC c 0 {}\nVB b 0 DC {} AC 1\nQ1 c b 0 mm {parameter}={scale}\n.model mm {kind}(LEVEL=1 IS=1e-14 BF=100 BR=2 IKF=1e-3 IKR=2e-3 VAF=40 VAR=20 CJE=2p CJC=1p TF=1n TR=2n)\n.options GMIN=0\n.end\n",p*vc,p*vb)).unwrap()
                    };
                    let unit = make("M", 1.0);
                    let dc = engine.run_dc_op(&unit).unwrap();
                    let ac = engine.run_ac(&unit, &[1e6]).unwrap();
                    for parameter in ["M", "AREA"] {
                        for scale in [1e-200, 1e-30, 1e-18, 0.25] {
                            let deck = make(parameter, scale);
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
                        let sum = ac.currents.iter().copied().sum::<rspice_core::Complex64>();
                        let scale = ac.currents.iter().map(|i| i.norm()).sum::<f64>();
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
        error.contains("Q1") && error.contains("AC private-state reduction"),
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
                let mut reference = None;
                for scale in [1.0, 1e-20, 1e-200] {
                    let deck = Netlist::parse(&format!("Private BJT ramp\nVB b 0 PWL(0 0 50n {})\nQ1 0 b 0 mm {parameter}={scale}\n.model mm {kind}(IS=0 RB=5k RBM=1k CJE=1p CJC=2p MJE=0 MJC=0)\n.end\n",p*0.001)).unwrap();
                    let tran = engine.run_tran(&deck, 50e-9, 0.05e-9).unwrap();
                    assert!(tran.time.len() > 3);
                    assert_eq!(*tran.time.last().unwrap(), 50e-9);
                    let currents = tran.try_branch_current_waveform_named("VB").unwrap();
                    if scale == 1.0 {
                        reference = Some((tran.time.clone(), currents.to_vec()));
                    }
                    let (reference_time, reference_current) = reference.as_ref().unwrap();
                    assert_eq!(&tran.time, reference_time);
                    for ((&time, &current), &unit_current) in
                        tran.time.iter().zip(currents).zip(reference_current)
                    {
                        // R=5k/scale and C=3p*scale: tau=15ns, independent
                        // of instance size. Ramp slope is +/-20,000 V/s.
                        let expected = if dialect == SpiceDialect::Ngspice {
                            p * 6e-8 * (-time / 15e-9).exp_m1()
                        } else {
                            // Isolate multiplicity/area scaling from Xyce's
                            // separate startup-history qualification.
                            unit_current
                        };
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

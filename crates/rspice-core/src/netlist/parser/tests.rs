use super::*;
use crate::netlist::lexer::parse_spice_value;

#[test]
fn test_parse_value() {
    let v = parse_spice_value("1k").unwrap();
    assert!((v - 1000.0).abs() < 1e-10);

    let v = parse_spice_value("1u").unwrap();
    assert!((v - 1e-6).abs() < 1e-20);

    let v = parse_spice_value("1MEG").unwrap();
    assert!((v - 1e6).abs() < 1e-10);
}

#[test]
fn test_parse_simple_netlist() {
    let netlist = r#"Simple RC Circuit
R1 1 2 1k
C1 2 0 1u
V1 1 0 DC 5
.OP
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.title, "Simple RC Circuit");
    assert_eq!(result.elements.len(), 3);
    assert_eq!(result.analyses.len(), 1);
}

#[test]
fn test_parse_options_tolerances_and_method() {
    let netlist = r#"Options Test
.OPTIONS RELTOL=2e-4 VNTOL=3e-6 ABSTOL=8e-13 IABSTOL=4e-12 RESIDUAL_RELTOL=5e-4 METHOD=GEAR ITL1=120 ITL4=9
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.options.reltol, Some(2e-4));
    assert_eq!(result.options.vntol, Some(3e-6));
    assert_eq!(result.options.abstol, Some(8e-13));
    assert_eq!(result.options.iabstol, Some(4e-12));
    assert_eq!(result.options.residual_reltol, Some(5e-4));
    assert_eq!(result.options.method.as_deref(), Some("GEAR"));
    assert_eq!(result.options.itl1, Some(120));
    assert_eq!(result.options.itl4, Some(9));
}

#[test]
fn test_parse_options_continuation_and_merge() {
    let netlist = r#"Options Merge Test
.OPTIONS RELTOL=1e-3
+ IABSTOL=2e-12 RESRELTOL=7e-4
.OPTIONS RELTOL=5e-4
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.options.reltol, Some(5e-4));
    assert_eq!(result.options.iabstol, Some(2e-12));
    assert_eq!(result.options.residual_reltol, Some(7e-4));
}

#[test]
fn test_parse_options_unknown_flag_does_not_consume_next_option() {
    let netlist = r#"Options Unknown Flag Test
.OPTIONS NOPAGE RELTOL=4e-4
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.options.reltol, Some(4e-4));
}

#[test]
fn test_parse_with_commas() {
    let netlist = r#"Comma Test
R1 1 0 1k, temp=27
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.elements.len(), 1);
    match &result.elements[0].kind {
        ElementKind::Resistor { value, .. } => {
            assert!((value - 1000.0).abs() < 1e-10);
        }
        _ => panic!("Expected Resistor"),
    }
}

#[test]
fn test_parse_model_based_resistor_preserves_model_and_instance_params() {
    let netlist = r#"Model-based Resistor
R1 in out RMOD L=10u W=2u M=2
.MODEL RMOD R (RSH=120)
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Resistor {
            value,
            model,
            instance_params,
            ..
        } => {
            assert!(
                value.is_nan(),
                "model-based resistor should not use placeholder value"
            );
            assert_eq!(model.as_deref(), Some("RMOD"));

            let params: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((params["L"] - 10e-6).abs() < 1e-18);
            assert!((params["W"] - 2e-6).abs() < 1e-18);
            assert!((params["M"] - 2.0).abs() < 1e-12);
        }
        other => panic!("Expected resistor, got {:?}", other),
    }
}

#[test]
fn test_parse_model_based_resistor_with_explicit_r_param_sets_value() {
    let netlist = r#"Model-based Resistor with explicit R
R1 in out RMOD R=2k L=10u W=2u
.MODEL RMOD R (RSH=120)
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Resistor {
            value,
            model,
            instance_params,
            ..
        } => {
            assert_eq!(model.as_deref(), Some("RMOD"));
            assert!((value - 2000.0).abs() < 1e-12);

            let params: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((params["R"] - 2000.0).abs() < 1e-12);
            assert!((params["L"] - 10e-6).abs() < 1e-18);
            assert!((params["W"] - 2e-6).abs() < 1e-18);
        }
        other => panic!("Expected resistor, got {:?}", other),
    }
}

#[test]
fn test_parse_resistor_preserves_parameter_expression_for_late_resolution() {
    let netlist = r#"Parametric Resistor
.PARAM FOO=2k
R1 in out '3*foo'
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Resistor {
            value,
            value_expr,
            model,
            ..
        } => {
            assert!(
                value.is_nan(),
                "deferred resistor value should stay unresolved"
            );
            assert_eq!(model, &None);
            assert_eq!(value_expr.as_deref(), Some("3*foo"));
        }
        other => panic!("Expected resistor, got {:?}", other),
    }
}

#[test]
fn test_parse_resistor_ohms_suffix_not_treated_as_model() {
    let netlist = r#"Resistor Unit Suffix
R1 1 0 1Ohms
R2 1 0 1.019524e+9Ohms
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    assert_eq!(result.elements.len(), 2);

    match &result.elements[0].kind {
        ElementKind::Resistor { value, model, .. } => {
            assert!((*value - 1.0).abs() < 1e-12);
            assert!(model.is_none());
        }
        other => panic!("Expected resistor, got {:?}", other),
    }
    match &result.elements[1].kind {
        ElementKind::Resistor { value, model, .. } => {
            assert!((*value - 1.019_524e9).abs() < 1e-3);
            assert!(model.is_none());
        }
        other => panic!("Expected resistor, got {:?}", other),
    }
}

#[test]
fn test_parse_pulse() {
    let netlist = r#"Pulse Test
V1 1 0 PULSE(0 5 0 1n 1n 1u 2u)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Pulse {
            v1,
            v2,
            delay,
            rise,
            fall,
            width,
            period,
        }) => {
            assert!((*v1 - 0.0).abs() < 1e-10);
            assert!((*v2 - 5.0).abs() < 1e-10);
            assert!((*delay - 0.0).abs() < 1e-10);
            assert!((*rise - 1e-9).abs() < 1e-20);
            assert!((*fall - 1e-9).abs() < 1e-20);
            assert!((*width - 1e-6).abs() < 1e-15);
            assert!((*period - 2e-6).abs() < 1e-15);
        }
        _ => panic!("Expected Pulse source"),
    }
}

#[test]
fn test_parse_pulse_with_minimal_args_uses_step_like_defaults() {
    let netlist = r#"Pulse Defaults
V1 1 0 PULSE(0 1)
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Pulse {
            v1,
            v2,
            delay,
            rise,
            fall,
            width,
            period,
        }) => {
            assert!((*v1 - 0.0).abs() < 1e-12);
            assert!((*v2 - 1.0).abs() < 1e-12);
            assert!((*delay - 0.0).abs() < 1e-12);
            assert!(rise.is_nan());
            assert!(fall.is_nan());
            assert!(width.is_nan());
            assert!(period.is_nan());
        }
        other => panic!("Expected pulse source, got {:?}", other),
    }
}

#[test]
fn test_parse_sin() {
    let netlist = r#"Sin Test
V1 1 0 SIN(0 1 1k)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Sin {
            offset,
            amplitude,
            frequency,
            ..
        }) => {
            assert!((*offset - 0.0).abs() < 1e-10);
            assert!((*amplitude - 1.0).abs() < 1e-10);
            assert!((*frequency - 1000.0).abs() < 1e-10);
        }
        _ => panic!("Expected Sin source"),
    }
}

#[test]
fn test_parse_ac_phase_is_degrees_converted_to_radians() {
    let netlist = r#"AC Phase Test
V1 1 0 AC 1 90
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Ac { magnitude, phase }) => {
            assert!((*magnitude - 1.0).abs() < 1e-12);
            assert!((*phase - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
        }
        _ => panic!("Expected AC source"),
    }
}

#[test]
fn test_parse_dcac_phase_is_degrees_converted_to_radians() {
    let netlist = r#"DC AC Phase Test
I1 1 0 DC 1m AC 2 180
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::CurrentSource(SourceSpec::DcAc {
            dc_value,
            ac_magnitude,
            ac_phase,
        }) => {
            assert!((*dc_value - 1e-3).abs() < 1e-15);
            assert!((*ac_magnitude - 2.0).abs() < 1e-12);
            assert!((*ac_phase - std::f64::consts::PI).abs() < 1e-12);
        }
        _ => panic!("Expected DC+AC source"),
    }
}

#[test]
fn test_parse_ac_dc_sine_combination_into_dc_ac_transient() {
    let netlist = r#"AC DC SIN Combo
V1 1 0 AC 1 DC 0 Sine(0 10m 10Meg 0 0)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::DcAcTransient {
            dc_value,
            ac_magnitude,
            ac_phase,
            transient,
        }) => {
            assert!((*dc_value - 0.0).abs() < 1e-15);
            assert!((*ac_magnitude - 1.0).abs() < 1e-12);
            assert!((*ac_phase - 0.0).abs() < 1e-12);
            match transient.as_ref() {
                SourceSpec::Sin {
                    offset,
                    amplitude,
                    frequency,
                    ..
                } => {
                    assert!((*offset - 0.0).abs() < 1e-15);
                    assert!((*amplitude - 10e-3).abs() < 1e-15);
                    assert!((*frequency - 10e6).abs() < 1e-3);
                }
                other => panic!("Expected SIN transient payload, got {:?}", other),
            }
        }
        other => panic!("Expected DC+AC+transient source, got {:?}", other),
    }
}

#[test]
fn test_parse_sine_keyword_alias() {
    let netlist = r#"SINE Alias
V1 1 0 SINE(0 2 1k)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Sin {
            offset,
            amplitude,
            frequency,
            ..
        }) => {
            assert!((*offset - 0.0).abs() < 1e-15);
            assert!((*amplitude - 2.0).abs() < 1e-12);
            assert!((*frequency - 1e3).abs() < 1e-9);
        }
        other => panic!("Expected SIN source via SINE alias, got {:?}", other),
    }
}

#[test]
fn test_parse_dc_plus_pwl_source() {
    let netlist = r#"DC + PWL Test
V1 1 0 DC 0.7 PWL(0 0 2n 5)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::DcTransient {
            dc_value,
            transient,
        }) => {
            assert!((*dc_value - 0.7).abs() < 1e-12);
            match transient.as_ref() {
                SourceSpec::Pwl { points } => {
                    assert_eq!(points.len(), 2);
                    assert!((points[0].0 - 0.0).abs() < 1e-18);
                    assert!((points[0].1 - 0.0).abs() < 1e-18);
                    assert!((points[1].0 - 2e-9).abs() < 1e-21);
                    assert!((points[1].1 - 5.0).abs() < 1e-18);
                }
                other => panic!("Expected PWL transient source, got {:?}", other),
            }
        }
        other => panic!("Expected DC+transient source, got {:?}", other),
    }
}

#[test]
fn test_parse_dc_plus_pulse_source() {
    let netlist = r#"DC + PULSE Test
I1 1 0 DC 1m PULSE(0 2m 1n 2n 2n 10n 20n)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::CurrentSource(SourceSpec::DcTransient {
            dc_value,
            transient,
        }) => {
            assert!((*dc_value - 1e-3).abs() < 1e-15);
            match transient.as_ref() {
                SourceSpec::Pulse {
                    v1,
                    v2,
                    delay,
                    rise,
                    fall,
                    width,
                    period,
                } => {
                    assert!((*v1 - 0.0).abs() < 1e-15);
                    assert!((*v2 - 2e-3).abs() < 1e-15);
                    assert!((*delay - 1e-9).abs() < 1e-21);
                    assert!((*rise - 2e-9).abs() < 1e-21);
                    assert!((*fall - 2e-9).abs() < 1e-21);
                    assert!((*width - 10e-9).abs() < 1e-21);
                    assert!((*period - 20e-9).abs() < 1e-21);
                }
                other => panic!("Expected PULSE transient source, got {:?}", other),
            }
        }
        other => panic!("Expected DC+transient source, got {:?}", other),
    }
}

#[test]
fn test_parse_sin_phase_is_degrees_converted_to_radians() {
    let netlist = r#"Sin Phase Test
V1 1 0 SIN(0 1 1k 0 0 90)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Sin { phase, .. }) => {
            assert!((*phase - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
        }
        _ => panic!("Expected Sin source"),
    }
}

#[test]
fn test_parse_param() {
    let netlist = r#"Param Test
.PARAM R=1k
R1 1 0 {R}
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert!(result.params.get("R").is_some());
    assert!((result.params.get("R").unwrap() - 1000.0).abs() < 1e-10);

    match &result.elements[0].kind {
        ElementKind::Resistor { value, .. } => {
            assert!((value - 1000.0).abs() < 1e-10);
        }
        _ => panic!("Expected Resistor"),
    }
}

#[test]
fn test_parse_param_expression() {
    let netlist = r#"Param Expression Test
.PARAM R1=1k R2=500
.PARAM RTOTAL={R1+R2}
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert!((result.params.get("RTOTAL").unwrap() - 1500.0).abs() < 1e-10);
}

#[test]
fn test_parse_csparam() {
    let netlist = r#"CsParam Test
.CSPARAM R=2k
R1 1 0 {R}
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert!(result.params.get("R").is_some());
    assert!((result.params.get("R").unwrap() - 2000.0).abs() < 1e-10);

    match &result.elements[0].kind {
        ElementKind::Resistor { value, .. } => {
            assert!((value - 2000.0).abs() < 1e-10);
        }
        _ => panic!("Expected Resistor"),
    }
}

#[test]
fn test_parse_csparam_expression_with_param_reference() {
    let netlist = r#"CsParam Expression Test
.PARAM RBASE=1k
.CSPARAM RLOAD={RBASE*2}
R1 1 0 {RLOAD}
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert!((result.params.get("RLOAD").unwrap() - 2000.0).abs() < 1e-10);

    match &result.elements[0].kind {
        ElementKind::Resistor { value, .. } => {
            assert!((*value - 2000.0).abs() < 1e-10);
        }
        _ => panic!("Expected Resistor"),
    }
}

#[test]
fn test_parse_diode() {
    let netlist = r#"Diode Test
D1 1 0 1N4148
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.elements.len(), 1);
    match &result.elements[0].kind {
        ElementKind::Diode { model } => {
            assert_eq!(model, "1N4148");
        }
        _ => panic!("Expected Diode element"),
    }
}

#[test]
fn test_parse_bjt() {
    let netlist = r#"BJT Test
Q1 2 1 0 2N2222
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::Bjt { model, .. } => {
            assert_eq!(model, "2N2222");
        }
        _ => panic!("Expected Bjt element"),
    }
}

#[test]
fn test_parse_bjt_with_off_keyword_keeps_model_name() {
    let netlist = r#"BJT OFF
Q1 3 2 4 QSTD OFF
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Bjt { model, .. } => {
            assert_eq!(model, "QSTD");
        }
        other => panic!("Expected BJT element, got {:?}", other),
    }
}

#[test]
fn test_parse_bjt_with_instance_params() {
    let netlist = r#"BJT Instance Params
Q1 c b e qmod m=2 area=3 temp=85
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Bjt {
            model,
            instance_params,
            ..
        } => {
            assert!(model.eq_ignore_ascii_case("qmod"));
            let map: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((map["M"] - 2.0).abs() < 1e-18);
            assert!((map["AREA"] - 3.0).abs() < 1e-18);
            assert!((map["TEMP"] - 85.0).abs() < 1e-18);
        }
        other => panic!("Expected BJT element, got {:?}", other),
    }
}

#[test]
fn test_parse_mosfet() {
    let netlist = r#"MOSFET Test
M1 3 2 1 0 NMOS
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::Mosfet {
            model,
            instance_params,
            ..
        } => {
            assert_eq!(model, "NMOS");
            assert!(instance_params.is_empty());
        }
        _ => panic!("Expected Mosfet element"),
    }
}

#[test]
fn test_parse_mosfet_with_instance_params() {
    let netlist = r#"MOSFET Instance Params
M1 d g s b nmod w=10u l=0.25u m=2 nf=4
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Mosfet {
            model,
            instance_params,
            ..
        } => {
            assert!(model.eq_ignore_ascii_case("nmod"));
            assert_eq!(instance_params.len(), 4);
            let map: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((map["W"] - 10e-6).abs() < 1e-18);
            assert!((map["L"] - 0.25e-6).abs() < 1e-18);
            assert!((map["M"] - 2.0).abs() < 1e-18);
            assert!((map["NF"] - 4.0).abs() < 1e-18);
        }
        other => panic!("Expected MOSFET element, got {:?}", other),
    }
}

#[test]
fn test_parse_five_node_mosfet_model_disambiguation() {
    let netlist = r#"Five Node MOS
M1 d g s e b n1 w=10u l=0.25u
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    assert_eq!(result.elements.len(), 1);
    assert_eq!(result.elements[0].nodes.len(), 5);
    assert!(result.elements[0].nodes[3].eq_ignore_ascii_case("e"));
    assert!(result.elements[0].nodes[4].eq_ignore_ascii_case("b"));
    match &result.elements[0].kind {
        ElementKind::Mosfet {
            model,
            instance_params,
            ..
        } => {
            assert!(model.eq_ignore_ascii_case("n1"));
            let map: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((map["W"] - 10e-6).abs() < 1e-18);
            assert!((map["L"] - 0.25e-6).abs() < 1e-18);
        }
        other => panic!("Expected MOSFET element, got {:?}", other),
    }
}

#[test]
fn test_parse_bsimsoi_optional_node_sequence() {
    let netlist = r#"BSIMSOI Optional Nodes
M1 d g s e p b t n1 w=10u l=0.25u
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    assert_eq!(result.elements.len(), 1);
    assert_eq!(result.elements[0].nodes.len(), 7);
    assert!(result.elements[0].nodes[3].eq_ignore_ascii_case("e"));
    assert!(result.elements[0].nodes[4].eq_ignore_ascii_case("p"));
    assert!(result.elements[0].nodes[5].eq_ignore_ascii_case("b"));
    assert!(result.elements[0].nodes[6].eq_ignore_ascii_case("t"));
    match &result.elements[0].kind {
        ElementKind::Mosfet {
            model,
            instance_params,
            ..
        } => {
            assert!(model.eq_ignore_ascii_case("n1"));
            let map: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((map["W"] - 10e-6).abs() < 1e-18);
            assert!((map["L"] - 0.25e-6).abs() < 1e-18);
        }
        other => panic!("Expected MOSFET element, got {:?}", other),
    }
}

#[test]
fn test_parse_jfet_with_instance_params() {
    let netlist = r#"JFET Instance Params
J1 d g s jmod area=2 m=3
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Jfet {
            model,
            instance_params,
            ..
        } => {
            assert!(model.eq_ignore_ascii_case("jmod"));
            let map: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((map["AREA"] - 2.0).abs() < 1e-18);
            assert!((map["M"] - 3.0).abs() < 1e-18);
        }
        other => panic!("Expected JFET element, got {:?}", other),
    }
}

#[test]
fn test_parse_mesfet_with_instance_geometry_params() {
    let netlist = r#"MESFET Instance Params
Z1 d g s zmod l=0.7u w=20u
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    match &result.elements[0].kind {
        ElementKind::Mesfet {
            model,
            instance_params,
            ..
        } => {
            assert!(model.eq_ignore_ascii_case("zmod"));
            let map: std::collections::HashMap<String, Value> =
                instance_params.iter().cloned().collect();
            assert!((map["L"] - 0.7e-6).abs() < 1e-18);
            assert!((map["W"] - 20e-6).abs() < 1e-18);
        }
        other => panic!("Expected MESFET element, got {:?}", other),
    }
}

#[test]
fn test_parse_subcircuit() {
    let netlist = r#"Subcircuit Test
.SUBCKT INVERTER IN OUT VDD VSS
M1 OUT IN VDD VDD PMOS
M2 OUT IN VSS VSS NMOS
.ENDS INVERTER
X1 A B VCC GND INVERTER
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.subcircuits.len(), 1);
    assert_eq!(result.subcircuits[0].name, "INVERTER");
    assert_eq!(result.subcircuits[0].ports, vec!["IN", "OUT", "VDD", "VSS"]);
    assert_eq!(result.subcircuits[0].elements.len(), 2);
    assert_eq!(result.elements.len(), 1);
}

#[test]
fn test_parse_model() {
    let netlist = r#"Model Test
.MODEL NMOS NMOS (VTO=0.7 KP=110u)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.models.len(), 1);
    assert_eq!(result.models[0].name, "NMOS");
}

#[test]
fn test_parse_model_with_inline_semicolon_comments() {
    let netlist = r#"Model Inline Comment Test
.MODEL 2N2222 NPN (
+ IS=14.34E-15 ; Saturation current
+ BF=255.9 ; Forward beta
+ VAF=74.03 ; Early voltage
+)
Q1 2 1 0 2N2222
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.models.len(), 1);
    assert_eq!(result.models[0].name, "2N2222");

    let params: std::collections::HashMap<String, Value> =
        result.models[0].params.iter().cloned().collect();
    assert!((params.get("IS").unwrap() - 14.34e-15).abs() < 1e-24);
    assert!((params.get("BF").unwrap() - 255.9).abs() < 1e-12);
    assert!((params.get("VAF").unwrap() - 74.03).abs() < 1e-12);
}

#[test]
fn test_parse_mos6_reference_model_types() {
    let netlist = include_str!("../../../../../tests/mos6/mos6inv.cir");
    let result = parse_netlist(netlist).expect("mos6inv deck should parse");

    let n10l5 = result
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case("N10L5"))
        .expect("missing N10L5 model");
    assert!(
        n10l5.model_type.eq_ignore_ascii_case("NMOS"),
        "expected N10L5 model type NMOS, got '{}'",
        n10l5.model_type
    );

    let p12l5 = result
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case("P12L5"))
        .expect("missing P12L5 model");
    assert!(
        p12l5.model_type.eq_ignore_ascii_case("PMOS"),
        "expected P12L5 model type PMOS, got '{}'",
        p12l5.model_type
    );
}

#[test]
fn test_parse_element_with_inline_semicolon_comment() {
    let netlist = r#"Inline Element Comment Test
V1 1 0 5 ; DC supply
R1 1 0 1k ; load resistor
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.elements.len(), 2);
    match &result.elements[1].kind {
        ElementKind::Resistor { value, .. } => assert!((*value - 1000.0).abs() < 1e-10),
        _ => panic!("Expected resistor"),
    }
}

#[test]
fn test_parse_element_with_inline_dollar_comment() {
    let netlist = r#"Inline Dollar Comment Test
V1 1 0 5 $ DC supply
R1 1 0 1k $ load resistor
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.elements.len(), 2);
    match &result.elements[1].kind {
        ElementKind::Resistor { value, .. } => assert!((*value - 1000.0).abs() < 1e-10),
        _ => panic!("Expected resistor"),
    }
}

#[test]
fn test_strip_inline_semicolon_comment_preserves_quoted_semicolons() {
    let line = r#"V1 1 0 PWL FILE="stim;ulus.csv" ; trailing comment"#;
    let stripped = strip_inline_semicolon_comment(line);
    assert_eq!(stripped, r#"V1 1 0 PWL FILE="stim;ulus.csv" "#);
}

#[test]
fn test_strip_inline_semicolon_comment_preserves_single_quoted_semicolons() {
    let line = r#".VERILOGA 'models;rf.va' mod1 ; trailing comment"#;
    let stripped = strip_inline_semicolon_comment(line);
    assert_eq!(stripped, r#".VERILOGA 'models;rf.va' mod1 "#);
}

#[test]
fn test_strip_inline_semicolon_comment_handles_escaped_quotes() {
    let line = ".PARAM A=\"quoted \\\";\\\" token\" ; trailing";
    let stripped = strip_inline_semicolon_comment(line);
    assert_eq!(stripped, ".PARAM A=\"quoted \\\";\\\" token\" ");
}

#[test]
fn test_strip_inline_semicolon_comment_preserves_quoted_dollar_signs() {
    let line = r#"V1 1 0 PWL FILE="stim$ulus.csv" $ trailing comment"#;
    let stripped = strip_inline_semicolon_comment(line);
    assert_eq!(stripped, r#"V1 1 0 PWL FILE="stim$ulus.csv" "#);
}

#[test]
fn test_parse_pwl() {
    let netlist = r#"PWL Test
V1 1 0 PWL(0 0 1u 5 2u 0)
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::Pwl { points }) => {
            assert_eq!(points.len(), 3);
            assert!((points[0].0 - 0.0).abs() < 1e-10);
            assert!((points[0].1 - 0.0).abs() < 1e-10);
            assert!((points[1].0 - 1e-6).abs() < 1e-15);
            assert!((points[1].1 - 5.0).abs() < 1e-10);
        }
        _ => panic!("Expected PWL source"),
    }
}

#[test]
fn test_parse_pwl_file_source() {
    let netlist = r#"PWL FILE Test
V1 1 0 PWL FILE="stimulus.csv" TSCALE=1e-3 VSCALE=2 TOFFSET=1u VOFFSET=0.25
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
        }) => {
            assert_eq!(path, "stimulus.csv");
            assert!((*time_scale - 1e-3).abs() < 1e-15);
            assert!((*value_scale - 2.0).abs() < 1e-12);
            assert!((*time_offset - 1e-6).abs() < 1e-15);
            assert!((*value_offset - 0.25).abs() < 1e-12);
        }
        _ => panic!("Expected PWL file source"),
    }
}

#[test]
fn test_parse_pwl_file_source_defaults() {
    let netlist = r#"PWL FILE Defaults Test
I1 1 0 PWL(FILE="wave.csv")
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::CurrentSource(SourceSpec::PwlFile {
            path,
            time_scale,
            value_scale,
            time_offset,
            value_offset,
        }) => {
            assert_eq!(path, "wave.csv");
            assert!((*time_scale - 1.0).abs() < 1e-12);
            assert!((*value_scale - 1.0).abs() < 1e-12);
            assert!((*time_offset - 0.0).abs() < 1e-12);
            assert!((*value_offset - 0.0).abs() < 1e-12);
        }
        _ => panic!("Expected PWL file source"),
    }
}

// =========================================================================
// New Element Type Tests
// =========================================================================

#[test]
fn test_parse_coupling() {
    let netlist = r#"Coupling Test
L1 1 2 1m
L2 3 4 4m
K1 L1 L2 0.99
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.elements.len(), 3);

    match &result.elements[2].kind {
        ElementKind::Coupling {
            inductors,
            coefficient,
        } => {
            assert_eq!(inductors.len(), 2);
            assert_eq!(inductors[0], "L1");
            assert_eq!(inductors[1], "L2");
            assert!((*coefficient - 0.99).abs() < 1e-10);
        }
        _ => panic!("Expected Coupling element"),
    }
}

#[test]
fn test_parse_vswitch() {
    let netlist = r#"VSwitch Test
S1 out 0 ctrl 0 SW1 OFF
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::VSwitch {
            control_pos,
            control_neg,
            model,
            initial_state,
        } => {
            // Note: lexer uppercases identifiers
            assert!(control_pos.eq_ignore_ascii_case("ctrl"));
            assert_eq!(control_neg, "0");
            assert!(model.eq_ignore_ascii_case("SW1"));
            assert_eq!(*initial_state, Some(SwitchState::Off));
        }
        _ => panic!("Expected VSwitch element"),
    }
}

#[test]
fn test_parse_iswitch() {
    let netlist = r#"ISwitch Test
W1 out 0 V1 CSW ON
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::ISwitch {
            control_element,
            model,
            initial_state,
        } => {
            assert_eq!(control_element, "V1");
            assert_eq!(model, "CSW");
            assert_eq!(*initial_state, Some(SwitchState::On));
        }
        _ => panic!("Expected ISwitch element"),
    }
}

#[test]
fn test_parse_transmission_line() {
    let netlist = r#"TLine Test
T1 in 0 out 0 Z0=50 TD=1n
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    match &result.elements[0].kind {
        ElementKind::TransmissionLine {
            z0,
            td,
            freq,
            nl,
            model,
        } => {
            assert_eq!(*z0, Some(50.0));
            assert!(td.is_some());
            assert!((td.unwrap() - 1e-9).abs() < 1e-20);
            assert!(freq.is_none());
            assert!(nl.is_none());
            assert!(model.is_none());
        }
        _ => panic!("Expected TransmissionLine element"),
    }
    assert_eq!(result.elements[0].nodes.len(), 4);
}

#[test]
fn test_parse_lossless_tline_model_reference() {
    let netlist = r#"O-Line Model Test
O1 1 0 2 0 LLINE
.END
"#;
    let result = parse_netlist(netlist).expect("O-line with model should parse");
    match &result.elements[0].kind {
        ElementKind::TransmissionLine {
            z0,
            td,
            freq,
            nl,
            model,
        } => {
            assert!(z0.is_none());
            assert!(td.is_none());
            assert!(freq.is_none());
            assert!(nl.is_none());
            assert_eq!(model.as_deref(), Some("LLINE"));
        }
        _ => panic!("Expected TransmissionLine element"),
    }
}

#[test]
fn test_parse_lossy_tline_model_with_inline_overrides() {
    let netlist = r#"Y-Line Model Test
Y1 1 0 2 0 YMOD Z0=75 TD=2N
.END
"#;
    let result = parse_netlist(netlist).expect("Y-line with model and overrides should parse");
    match &result.elements[0].kind {
        ElementKind::TransmissionLine {
            z0,
            td,
            freq,
            nl,
            model,
        } => {
            assert_eq!(*z0, Some(75.0));
            assert_eq!(*td, Some(2e-9));
            assert!(freq.is_none());
            assert!(nl.is_none());
            assert_eq!(model.as_deref(), Some("YMOD"));
        }
        _ => panic!("Expected TransmissionLine element"),
    }
}

#[test]
fn test_parse_lossless_tline_requires_model_or_z0() {
    let netlist = r#"Invalid O-Line
O1 1 0 2 0
.END
"#;
    let err = parse_netlist(netlist).expect_err("O-line without model/z0 should fail");
    match err {
        ParseError::Syntax { message, .. } => {
            assert!(message.contains("requires MODEL name or Z0"));
        }
        _ => panic!("Expected syntax error"),
    }
}

// =========================================================================
// New Analysis Command Tests
// =========================================================================

#[test]
fn test_parse_step_linear() {
    let netlist = r#"Step Test
R1 1 0 1k
.STEP PARAM RL 100 1k 100
.END
"#;
    let result = parse_netlist(netlist).unwrap();
    assert_eq!(result.analyses.len(), 1);

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Param);
            assert_eq!(cmd.name, "RL");
            match &cmd.sweep {
                StepSweep::Linear { start, stop, step } => {
                    assert!((*start - 100.0).abs() < 1e-10);
                    assert!((*stop - 1000.0).abs() < 1e-10);
                    assert!((*step - 100.0).abs() < 1e-10);
                }
                _ => panic!("Expected Linear sweep"),
            }
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_list() {
    let netlist = r#"Step List Test
.STEP PARAM C1 LIST 1n 10n 100n
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => match &cmd.sweep {
            StepSweep::List(values) => {
                assert_eq!(values.len(), 3);
                assert!((values[0] - 1e-9).abs() < 1e-20);
                assert!((values[1] - 10e-9).abs() < 1e-18);
                assert!((values[2] - 100e-9).abs() < 1e-17);
            }
            _ => panic!("Expected List sweep"),
        },
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_device_parenthesized_param_name() {
    let netlist = r#"Step Device Param
.STEP R1(VALUE) 1k 5k 1k
.END
"#;
    let result = parse_netlist(netlist).expect(".STEP device(param) should parse");

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Device);
            assert_eq!(cmd.name, "R1");
            assert_eq!(cmd.param_name.as_deref(), Some("VALUE"));
            match &cmd.sweep {
                StepSweep::Linear { start, stop, step } => {
                    assert!((*start - 1e3).abs() < 1e-10);
                    assert!((*stop - 5e3).abs() < 1e-10);
                    assert!((*step - 1e3).abs() < 1e-10);
                }
                _ => panic!("Expected Linear sweep"),
            }
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_device_named_param_name() {
    let netlist = r#"Step Device Named Param
.STEP R1 VALUE 1k 3k 1k
.END
"#;
    let result = parse_netlist(netlist).expect(".STEP device param should parse");

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Device);
            assert_eq!(cmd.name, "R1");
            assert_eq!(cmd.param_name.as_deref(), Some("VALUE"));
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_device_named_param_with_list() {
    let netlist = r#"Step Device Named Param List
.STEP R1 VALUE LIST 1k 2k 5k
.END
"#;
    let result = parse_netlist(netlist).expect(".STEP device param LIST should parse");

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Device);
            assert_eq!(cmd.name, "R1");
            assert_eq!(cmd.param_name.as_deref(), Some("VALUE"));
            match &cmd.sweep {
                StepSweep::List(values) => assert_eq!(values, &vec![1e3, 2e3, 5e3]),
                _ => panic!("Expected List sweep"),
            }
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_model_param_name() {
    let netlist = r#"Step Model Param
.STEP MODEL NMOS VTO -0.5 0.5 0.25
.END
"#;
    let result = parse_netlist(netlist).expect(".STEP MODEL should parse");

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Model);
            assert_eq!(cmd.name, "NMOS");
            assert_eq!(cmd.param_name.as_deref(), Some("VTO"));
            match &cmd.sweep {
                StepSweep::Linear { start, stop, step } => {
                    assert!((*start - -0.5).abs() < 1e-12);
                    assert!((*stop - 0.5).abs() < 1e-12);
                    assert!((*step - 0.25).abs() < 1e-12);
                }
                _ => panic!("Expected Linear sweep"),
            }
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_temp_linear_without_name() {
    let netlist = r#"Step Temp Linear
.STEP TEMP -40 125 55
.END
"#;
    let result = parse_netlist(netlist).expect(".STEP TEMP linear should parse");

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Temp);
            assert_eq!(cmd.name, "TEMP");
            match &cmd.sweep {
                StepSweep::Linear { start, stop, step } => {
                    assert!((*start - -40.0).abs() < 1e-10);
                    assert!((*stop - 125.0).abs() < 1e-10);
                    assert!((*step - 55.0).abs() < 1e-10);
                }
                _ => panic!("Expected Linear sweep"),
            }
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_step_temp_list_without_name() {
    let netlist = r#"Step Temp List
.STEP TEMP LIST -40 27 85 125
.END
"#;
    let result = parse_netlist(netlist).expect(".STEP TEMP LIST should parse");

    match &result.analyses[0] {
        AnalysisCommand::Step(cmd) => {
            assert_eq!(cmd.target, StepTarget::Temp);
            assert_eq!(cmd.name, "TEMP");
            match &cmd.sweep {
                StepSweep::List(values) => {
                    assert_eq!(values, &vec![-40.0, 27.0, 85.0, 125.0]);
                }
                _ => panic!("Expected List sweep"),
            }
        }
        _ => panic!("Expected Step command"),
    }
}

#[test]
fn test_parse_temp() {
    let netlist = r#"Temp Test
.TEMP -40 27 85 125
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Temp { temperatures } => {
            assert_eq!(temperatures.len(), 4);
            assert!((*temperatures.first().unwrap() - -40.0).abs() < 1e-10);
            assert!((*temperatures.get(1).unwrap() - 27.0).abs() < 1e-10);
            assert!((*temperatures.get(2).unwrap() - 85.0).abs() < 1e-10);
            assert!((*temperatures.get(3).unwrap() - 125.0).abs() < 1e-10);
        }
        _ => panic!("Expected Temp command"),
    }
}

#[test]
fn test_parse_four() {
    // Use simpler node names without parentheses
    let netlist = r#"Fourier Test
.FOUR 1k OUT
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Four {
            fundamental,
            outputs,
            ..
        } => {
            assert!((*fundamental - 1000.0).abs() < 1e-10);
            assert!(!outputs.is_empty());
        }
        _ => panic!("Expected Four command"),
    }
}

#[test]
fn test_parse_noise() {
    // Use simpler output specification without V() wrapper
    let netlist = r#"Noise Test
.NOISE OUT V1 DEC 10 1 1MEG
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Noise {
            output_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
            ..
        } => {
            assert_eq!(output_node, "OUT");
            assert_eq!(input_source, "V1");
            assert_eq!(*variation, FreqVariation::Dec);
            assert_eq!(*points, 10);
            assert!((*start_freq - 1.0).abs() < 1e-10);
            assert!((*stop_freq - 1e6).abs() < 1e-3);
        }
        _ => panic!("Expected Noise command"),
    }
}

#[test]
fn test_parse_disto_with_f2_ratio() {
    let netlist = r#"Disto Test
.DISTO DEC 12 10 1MEG 1.5
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            assert_eq!(*variation, FreqVariation::Dec);
            assert_eq!(*points, 12);
            assert!((*start_freq - 10.0).abs() < 1e-12);
            assert!((*stop_freq - 1e6).abs() < 1e-3);
            assert_eq!(*f2_over_f1, Some(1.5));
        }
        _ => panic!("Expected Disto command"),
    }
}

#[test]
fn test_parse_disto_without_f2_ratio() {
    let netlist = r#"Disto Test
.DISTO LIN 50 1k 10k
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            assert_eq!(*variation, FreqVariation::Lin);
            assert_eq!(*points, 50);
            assert!((*start_freq - 1e3).abs() < 1e-9);
            assert!((*stop_freq - 1e4).abs() < 1e-6);
            assert!(f2_over_f1.is_none());
        }
        _ => panic!("Expected Disto command"),
    }
}

#[test]
fn test_parse_disto_invalid_variation() {
    let netlist = r#"Disto Invalid
.DISTO BAD 10 1 1MEG
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected invalid .DISTO variation");
    assert!(
        err.to_string()
            .contains("Invalid .DISTO frequency variation")
    );
}

#[test]
fn test_parse_mc_defaults() {
    let netlist = r#"Monte Carlo Default
.MC 128
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::MonteCarlo(cmd) => {
            assert_eq!(cmd.runs, 128);
            assert_eq!(cmd.seed, None);
            assert_eq!(cmd.distribution, MonteCarloDistribution::Gaussian);
            assert!((cmd.relative_spread - 0.01).abs() < 1e-12);
            assert!(cmd.params.is_empty());
        }
        _ => panic!("Expected MonteCarlo command"),
    }
}

#[test]
fn test_parse_mc_with_full_options() {
    let netlist = r#"Monte Carlo Full
.MC 200 SEED 77 DIST UNIFORM SPREAD 0.05 PARAMS RVAL CVAL
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::MonteCarlo(cmd) => {
            assert_eq!(cmd.runs, 200);
            assert_eq!(cmd.seed, Some(77));
            assert_eq!(cmd.distribution, MonteCarloDistribution::Uniform);
            assert!((cmd.relative_spread - 0.05).abs() < 1e-12);
            assert_eq!(cmd.params, vec!["RVAL".to_string(), "CVAL".to_string()]);
        }
        _ => panic!("Expected MonteCarlo command"),
    }
}

#[test]
fn test_parse_mc_shorthand_gaussian() {
    let netlist = r#"Monte Carlo Shorthand
.MC 64 GAUSS 0.02 PARAMS RVAL
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::MonteCarlo(cmd) => {
            assert_eq!(cmd.runs, 64);
            assert_eq!(cmd.distribution, MonteCarloDistribution::Gaussian);
            assert!((cmd.relative_spread - 0.02).abs() < 1e-12);
            assert_eq!(cmd.params, vec!["RVAL".to_string()]);
        }
        _ => panic!("Expected MonteCarlo command"),
    }
}

#[test]
fn test_parse_mc_with_worst_case_distribution() {
    let netlist = r#"Monte Carlo Worst Case
.MC 32 DIST WORSTCASE SPREAD 0.03 PARAMS RVAL
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::MonteCarlo(cmd) => {
            assert_eq!(cmd.runs, 32);
            assert_eq!(cmd.distribution, MonteCarloDistribution::WorstCase);
            assert!((cmd.relative_spread - 0.03).abs() < 1e-12);
            assert_eq!(cmd.params, vec!["RVAL".to_string()]);
        }
        _ => panic!("Expected MonteCarlo command"),
    }
}

#[test]
fn test_parse_mc_invalid_distribution() {
    let netlist = r#"Monte Carlo Invalid Dist
.MC 16 DIST BAD
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected .MC distribution parse error");
    assert!(
        err.to_string()
            .contains("expected GAUSS, UNIFORM, or WORSTCASE")
    );
}

#[test]
fn test_parse_mc_invalid_runs() {
    let netlist = r#"Monte Carlo Invalid Runs
.MC 0
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected .MC run count parse error");
    assert!(err.to_string().contains("positive integer"));
}

#[test]
fn test_parse_sens_dc() {
    let netlist = r#"Sensitivity Test
.SENS V(out)
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            ac_sweep,
        } => {
            assert_eq!(output_node, "OUT");
            assert!(reference_node.is_none());
            assert!(ac_sweep.is_none());
        }
        _ => panic!("Expected Sensitivity command"),
    }
}

#[test]
fn test_parse_sens_ac_with_reference() {
    let netlist = r#"Sensitivity AC Test
.SENS V(out,ref) AC DEC 10 1 1MEG
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            ac_sweep,
        } => {
            assert_eq!(output_node, "OUT");
            assert_eq!(reference_node.as_deref(), Some("REF"));
            let ac = ac_sweep.expect("expected AC sweep");
            assert_eq!(ac.variation, FreqVariation::Dec);
            assert_eq!(ac.points, 10);
            assert!((ac.start_freq - 1.0).abs() < 1e-12);
            assert!((ac.stop_freq - 1e6).abs() < 1e-3);
        }
        _ => panic!("Expected Sensitivity command"),
    }
}

#[test]
fn test_parse_sens_invalid_mode() {
    let netlist = r#"Sensitivity Invalid
.SENS V(out) BAD
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected invalid .SENS mode");
    assert!(err.to_string().contains("expected AC or end-of-line"));
}

#[test]
fn test_parse_sens_invalid_variation() {
    let netlist = r#"Sensitivity Invalid AC
.SENS V(out) AC BAD 10 1 1MEG
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected invalid .SENS AC variation");
    assert!(err.to_string().contains("expected LIN, OCT, or DEC"));
}

#[test]
fn test_parse_pz_voltage_pole_zero() {
    let netlist = r#"PZ Test
.PZ in 0 out 0 VOL PZ
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            assert_eq!(input_pos, "IN");
            assert_eq!(input_neg, "0");
            assert_eq!(output_pos, "OUT");
            assert_eq!(output_neg, "0");
            assert_eq!(*transfer_type, PoleZeroTransferType::Voltage);
            assert_eq!(*analysis_type, PoleZeroAnalysisType::PoleZero);
        }
        _ => panic!("Expected PoleZero command"),
    }
}

#[test]
fn test_parse_pz_current_poles_only() {
    let netlist = r#"PZ Poles Test
.PZ 1 0 2 0 CUR POL
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.analyses[0] {
        AnalysisCommand::PoleZero {
            transfer_type,
            analysis_type,
            ..
        } => {
            assert_eq!(*transfer_type, PoleZeroTransferType::Current);
            assert_eq!(*analysis_type, PoleZeroAnalysisType::PolesOnly);
        }
        _ => panic!("Expected PoleZero command"),
    }
}

#[test]
fn test_parse_pz_invalid_transfer_type() {
    let netlist = r#"PZ Invalid Transfer
.PZ in 0 out 0 BAD PZ
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected .PZ transfer type error");
    assert!(err.to_string().contains("VOL or CUR"));
}

#[test]
fn test_parse_pz_invalid_analysis_type() {
    let netlist = r#"PZ Invalid Type
.PZ in 0 out 0 VOL BAD
.END
"#;
    let err = parse_netlist(netlist).expect_err("expected .PZ analysis type error");
    assert!(err.to_string().contains("PZ, POL, or ZER"));
}

#[test]
fn test_parse_ic() {
    let netlist = r#"IC Test
.IC N1=5 N2=2.5
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.initial_conditions.len(), 2);
    assert_eq!(result.initial_conditions[0].node, "N1");
    assert!((result.initial_conditions[0].voltage - 5.0).abs() < 1e-10);
    assert_eq!(result.initial_conditions[1].node, "N2");
    assert!((result.initial_conditions[1].voltage - 2.5).abs() < 1e-10);
}

#[test]
fn test_parse_ic_with_voltage_function_syntax() {
    let netlist = r#"IC V() Syntax
.IC V(out)=1.2 V(mid,0)=0.45
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    assert_eq!(result.initial_conditions.len(), 2);
    assert_eq!(result.initial_conditions[0].node, "OUT");
    assert!((result.initial_conditions[0].voltage - 1.2).abs() < 1e-12);
    assert_eq!(result.initial_conditions[1].node, "MID");
    assert!((result.initial_conditions[1].voltage - 0.45).abs() < 1e-12);
}

#[test]
fn test_parse_nodeset() {
    let netlist = r#"NODESET Test
.NODESET N1=1.0 V(N2,0)=2.5
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    assert_eq!(result.node_sets.len(), 2);
    assert_eq!(result.node_sets[0].node, "N1");
    assert!((result.node_sets[0].voltage - 1.0).abs() < 1e-12);
    assert_eq!(result.node_sets[1].node, "N2");
    assert!((result.node_sets[1].voltage - 2.5).abs() < 1e-12);
}

#[test]
fn test_parse_behavioral_expression_quoted_preserves_inner_expression_text() {
    let netlist = r#"Behavioral Quote
B1 out 0 V='1+2'
R1 out 0 1k
.END
"#;
    let parsed = parse_netlist(netlist).expect("netlist should parse");
    let b1 = parsed
        .elements
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("B1"))
        .expect("missing B1");

    match &b1.kind {
        ElementKind::BehavioralVoltage {
            expression,
            tc1,
            tc2,
        } => {
            assert_eq!(expression, "1+2");
            assert_eq!((*tc1, *tc2), (0.0, 0.0));
        }
        other => panic!("expected behavioral voltage source, got {:?}", other),
    }
}

#[test]
fn test_parse_behavioral_expression_braced_preserves_inner_expression_text() {
    let netlist = r#"Behavioral Braces
B1 out 0 V={1+2}
R1 out 0 1k
.END
"#;
    let parsed = parse_netlist(netlist).expect("netlist should parse");
    let b1 = parsed
        .elements
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("B1"))
        .expect("missing B1");

    match &b1.kind {
        ElementKind::BehavioralVoltage {
            expression,
            tc1,
            tc2,
        } => {
            assert_eq!(expression, "1+2");
            assert_eq!((*tc1, *tc2), (0.0, 0.0));
        }
        other => panic!("expected behavioral voltage source, got {:?}", other),
    }
}

#[test]
fn test_parse_behavioral_expression_stops_before_temperature_coefficients() {
    let netlist = r#"Behavioral Temp Coeff
B1 out 0 I=V(in) tc1=1m tc2=2u
V1 in 0 1
.END
"#;
    let parsed = parse_netlist(netlist).expect("netlist should parse");
    let b1 = parsed
        .elements
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case("B1"))
        .expect("missing B1");

    match &b1.kind {
        ElementKind::BehavioralCurrent {
            expression,
            tc1,
            tc2,
        } => {
            assert_eq!(expression.to_ascii_uppercase(), "V ( IN )");
            assert!((*tc1 - 1e-3).abs() < 1e-15);
            assert!((*tc2 - 2e-6).abs() < 1e-18);
        }
        other => panic!("expected behavioral current source, got {:?}", other),
    }
}

#[test]
fn test_parse_params_named_like_ic_do_not_create_initial_conditions() {
    let netlist = r#"IC Empty Suffix Test
.PARAM IC_=5
.PARAM IC_NODE=2.5
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");
    assert!(result.initial_conditions.is_empty());
    assert!(result.node_sets.is_empty());
    assert!((result.params.get("IC_NODE").unwrap() - 2.5).abs() < 1e-10);
}

#[test]
fn test_parse_veriloga_directive() {
    let netlist = r#"Verilog-A Test
.VERILOGA resistor.va
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_str().unwrap(),
        "resistor.va"
    );
    assert!(result.veriloga_includes[0].model_name.is_none());
}

#[test]
fn test_parse_veriloga_with_model_name() {
    let netlist = r#"Verilog-A Model Override
.VERILOGA diode.va MyDiode
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_str().unwrap(),
        "diode.va"
    );
    assert_eq!(
        result.veriloga_includes[0].model_name.as_deref(),
        Some("MyDiode")
    );
}

#[test]
fn test_parse_va_shorthand() {
    let netlist = r#"VA Shorthand
.VA capacitor.va
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_str().unwrap(),
        "capacitor.va"
    );
}

#[test]
fn test_parse_multiple_veriloga() {
    let netlist = r#"Multiple VA
.VERILOGA resistor.va
.VERILOGA diode.va Diode1N4148
.VA capacitor.va
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 3);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_str().unwrap(),
        "resistor.va"
    );
    assert_eq!(
        result.veriloga_includes[1].file_path.to_str().unwrap(),
        "diode.va"
    );
    assert_eq!(
        result.veriloga_includes[1].model_name.as_deref(),
        Some("Diode1N4148")
    );
    assert_eq!(
        result.veriloga_includes[2].file_path.to_str().unwrap(),
        "capacitor.va"
    );
}

#[test]
fn test_parse_veriloga_with_path() {
    let netlist = r#"VA with Path
.VERILOGA models/varactor.va
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_str().unwrap(),
        "models/varactor.va"
    );
}

#[test]
fn test_parse_veriloga_with_quoted_path_and_model() {
    let netlist = r#"VA Quoted Path
.VERILOGA "models/custom device.va" custom_device
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_string_lossy(),
        "models/custom device.va"
    );
    assert_eq!(
        result.veriloga_includes[0].model_name.as_deref(),
        Some("custom_device")
    );
}

#[test]
fn test_parse_veriloga_with_semicolon_in_quoted_path() {
    let netlist = r#"VA Semicolon Path
.VERILOGA "models/rf;mixer.va" mixer ; keep path semicolon
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_string_lossy(),
        "models/rf;mixer.va"
    );
    assert_eq!(
        result.veriloga_includes[0].model_name.as_deref(),
        Some("mixer")
    );
}

#[test]
fn test_parse_veriloga_with_single_quoted_path() {
    let netlist = r#"VA Single Quote Path
.VA 'pdk/models/va mos.va'
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    assert_eq!(result.veriloga_includes.len(), 1);
    assert_eq!(
        result.veriloga_includes[0].file_path.to_string_lossy(),
        "pdk/models/va mos.va"
    );
    assert!(result.veriloga_includes[0].model_name.is_none());
}

#[test]
fn test_parse_pwl_file_with_semicolon_in_quoted_path() {
    let netlist = r#"PWL FILE Semicolon Path Test
V1 1 0 PWL FILE="stim;ulus.csv" ; trailing comment
.END
"#;
    let result = parse_netlist(netlist).unwrap();

    match &result.elements[0].kind {
        ElementKind::VoltageSource(SourceSpec::PwlFile { path, .. }) => {
            assert_eq!(path, "stim;ulus.csv");
        }
        other => panic!("Expected PWL file source, got {:?}", other),
    }
}

#[test]
fn test_parse_global_directive_collects_nodes() {
    let netlist = r#"Global Node Test
.GLOBAL VDD gnd 17
V1 VDD 0 1.8
R1 VDD gnd 1k
.END
"#;
    let result = parse_netlist(netlist).expect("netlist should parse");

    assert!(result.is_global("VDD"));
    assert!(result.is_global("gnd"));
    assert!(result.is_global("17"));
    assert_eq!(result.global_nodes.len(), 3);
}

#[test]
fn test_parse_global_directive_requires_nodes() {
    let netlist = r#"Global Empty Test
.GLOBAL
.END
"#;

    let err = parse_netlist(netlist).expect_err("empty .GLOBAL should fail");
    let message = err.to_string();
    assert!(
        message.contains(".GLOBAL requires at least one node name"),
        "unexpected error: {}",
        message
    );
}

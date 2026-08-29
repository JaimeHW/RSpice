use rspice_core::config::ExpressionDialect;
use rspice_core::netlist::{
    DiagnosticSeverity, Netlist, NetlistParseOptions, NetlistSourceLocation,
};

fn xyce_options() -> NetlistParseOptions {
    NetlistParseOptions {
        expression_dialect: ExpressionDialect::Xyce,
        ..NetlistParseOptions::default()
    }
}

#[test]
fn xyce_unknown_diode_expression_parameter_is_preserved_and_warned() {
    let source = "BUG45 warning contract\n\
V1 1 0 1\n\
D1 1 0 DLEG\n\
.MODEL DLEG D(IS=4E-10 N=1.48 RS=.105\n\
+ BOGOPARAM={1+2})\n\
.OP\n\
.END\n";

    let netlist = Netlist::parse_with_options(source, xyce_options()).expect("deck parses");
    let model = netlist.models.first().expect("diode model is retained");
    assert!(
        model
            .params
            .iter()
            .any(|(name, value)| name.eq_ignore_ascii_case("BOGOPARAM") && *value == 3.0),
        "the ignored parameter remains available to tooling and audit"
    );

    assert_eq!(netlist.diagnostics.len(), 1);
    let diagnostic = &netlist.diagnostics[0];
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
    assert_eq!(diagnostic.code, "xyce-unknown-diode-model-parameter");
    assert_eq!(diagnostic.line, 4);
    assert_eq!(diagnostic.origin, Some(NetlistSourceLocation::in_memory(4)));
    assert_eq!(
        diagnostic.message,
        "No model parameter BOGOPARAM found for model DLEG of type D, parameter ignored."
    );
}

#[test]
fn xyce_release_710_legacy_diode_parameter_namespace_does_not_emit_false_warnings() {
    let source = "Xyce 7.10 legacy diode parameter namespace\n\
V1 1 0 1\n\
D1 1 0 DLEG\n\
.MODEL DLEG D(LEVEL=1 IS=1p JS=1p JSW=0 RS=0 N=1 NS=1 ISR=0 NR=2 IKF=1\n\
+ TT=1n CJO=1p CJ=1p CJ0=1p VJ=.7 M=.5 CJSW=0 CJP=0 PHP=1 VJSW=1\n\
+ MJSW=.33 EG=1.11 XTI=3 TIKF=0 TBV1=0 TBV2=0 TRS1=0 TRS=0 TRS2=0\n\
+ FC=.5 FCS=.5 BV=5 VB=5 IBV=1u NBV=1 IBVL=0 NBVL=1 TNOM=27 KF=0 AF=1)\n\
.OP\n\
.END\n";

    let netlist = Netlist::parse_with_options(source, xyce_options()).expect("deck parses");
    assert!(
        netlist.diagnostics.is_empty(),
        "native diode parameters must not be mislabeled: {:?}",
        netlist.diagnostics
    );
}

#[test]
fn non_xyce_unknown_diode_parameter_retains_existing_dialect_policy() {
    let source = "ngspice compatibility\n\
V1 1 0 1\n\
D1 1 0 DLEG\n\
.MODEL DLEG D(BOGOPARAM={1+2})\n\
.OP\n\
.END\n";

    let netlist = Netlist::parse(source).expect("deck parses");
    assert!(netlist.diagnostics.is_empty());
}

#[test]
fn xyce_legacy_diode_diagnoses_non_xyce_model_card_names() {
    let source = "non-Xyce legacy diode model names\n\
D1 1 0 DLEG\n\
.MODEL DLEG D(AREA=2 PJ=3 TLEV=1)\n\
.END\n";
    let netlist = Netlist::parse_with_options(source, xyce_options()).expect("deck parses");
    let messages = netlist
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.message.as_str())
        .collect::<Vec<_>>();
    assert_eq!(messages.len(), 3);
    for name in ["AREA", "PJ", "TLEV"] {
        assert!(
            messages.iter().any(|message| message.contains(name)),
            "missing Xyce unknown-model-parameter warning for {name}: {messages:?}"
        );
    }
}

#[test]
fn xyce_unknown_diode_warning_has_exact_legacy_wrapper_rendering() {
    let diagnostic = rspice_core::netlist::ParseDiagnostic::warning_at(
        NetlistSourceLocation::in_file("some/path/diode.cir", 29),
        "xyce-unknown-diode-model-parameter",
        "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter ignored.",
    );
    assert_eq!(
        diagnostic.xyce_legacy_warning_lines(),
        Some([
            "Netlist warning in file diode.cir at or near line 29".to_string(),
            "No model parameter BOGOPARAM found for model D1N3940 of type D, parameter ignored."
                .to_string(),
        ])
    );
}

#[test]
fn xyce_unknown_diode_warnings_preserve_authored_order_across_value_kinds() {
    let source = "authored warning order\n\
D1 1 0 DLEG\n\
.MODEL DLEG D(BOGO_STRING=\"first\" BOGO_NUMERIC=2 BOGO_EXPR={TEMP})\n\
.END\n";
    let netlist = Netlist::parse_with_options(source, xyce_options()).expect("deck parses");
    let names = netlist
        .diagnostics
        .iter()
        .map(|diagnostic| {
            diagnostic
                .message
                .split_whitespace()
                .nth(3)
                .expect("warning names its parameter")
        })
        .collect::<Vec<_>>();
    assert_eq!(names, ["BOGO_STRING", "BOGO_NUMERIC", "BOGO_EXPR"]);
    assert!(
        netlist
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.message.ends_with("parameter ignored."))
    );
}

#[test]
fn xyce_subcircuit_deferred_level_warns_once_when_any_instance_is_native() {
    let source = "instance-scoped diode namespace\n\
.SUBCKT CELL A K PARAMS: LV=1\n\
.MODEL DLOCAL D(LEVEL={LV} TYPE=1)\n\
D1 A K DLOCAL\n\
.ENDS CELL\n\
XN 1 0 CELL LV=1\n\
XG 2 0 CELL LV=200\n\
.END\n";
    let netlist = Netlist::parse_with_options(source, xyce_options()).expect("deck parses");
    assert_eq!(netlist.diagnostics.len(), 1, "{:#?}", netlist.diagnostics);
    assert_eq!(
        netlist.diagnostics[0].message,
        "No model parameter TYPE found for model DLOCAL of type D, parameter ignored."
    );
}

#[test]
fn xyce_subcircuit_deferred_level_does_not_apply_legacy_namespace_to_generated_routes() {
    let source = "instance-scoped generated diode namespace\n\
.SUBCKT CELL A K PARAMS: LV=200\n\
.MODEL DLOCAL D(LEVEL={LV} TYPE=1)\n\
D1 A K DLOCAL\n\
.ENDS CELL\n\
X1 1 0 CELL LV=200\n\
X2 2 0 CELL LV=200\n\
.END\n";
    let netlist = Netlist::parse_with_options(source, xyce_options()).expect("deck parses");
    assert!(netlist.diagnostics.is_empty(), "{:#?}", netlist.diagnostics);
}

#[test]
fn xyce_deferred_level_warning_survives_unrelated_invalid_hierarchy_siblings() {
    for source in [
        "top-level invalid sibling\n\
.SUBCKT CELL A K PARAMS: LV=1\n\
.MODEL DLOCAL D(LEVEL={LV} TYPE=1)\n\
D1 A K DLOCAL\n\
.ENDS CELL\n\
XGOOD 1 0 CELL LV=1\n\
XBAD 2 0 UNDEFINED_CELL\n\
.END\n",
        "nested invalid sibling\n\
.SUBCKT CELL A K PARAMS: LV=1\n\
.MODEL DLOCAL D(LEVEL={LV} TYPE=1)\n\
D1 A K DLOCAL\n\
.ENDS CELL\n\
.SUBCKT OUTER A K\n\
XGOOD A K CELL LV=1\n\
XBAD A K UNDEFINED_CELL\n\
.ENDS OUTER\n\
XOUT 1 0 OUTER\n\
.END\n",
    ] {
        let netlist = Netlist::parse_with_options(source, xyce_options())
            .expect("ordinary hierarchy failures remain circuit-construction errors");
        assert_eq!(netlist.diagnostics.len(), 1, "{:#?}", netlist.diagnostics);
        assert_eq!(
            netlist.diagnostics[0].message,
            "No model parameter TYPE found for model DLOCAL of type D, parameter ignored."
        );
    }
}

#[test]
fn xyce_generated_diode_levels_use_their_canonical_model_namespaces() {
    for (level, generated_parameter) in [
        ("200", "TYPE=1"),
        ("200.0000000001", "TYPE=1"),
        ("2002", "VERSION=2.1"),
    ] {
        let source = format!(
            "generated diode namespace\nD1 1 0 DGEN\n.MODEL DGEN D(LEVEL={level} {generated_parameter})\n.END\n"
        );
        let netlist =
            Netlist::parse_with_options(&source, xyce_options()).expect("generated diode parses");
        assert!(
            netlist.diagnostics.is_empty(),
            "D LEVEL={level} parameters belong to the generated model descriptor: {:?}",
            netlist.diagnostics
        );
    }
}

#[test]
fn xyce_deferred_and_inherited_generated_diode_levels_do_not_use_legacy_namespace() {
    for source in [
        "forward generated diode namespace\nD1 1 0 DGEN\n.MODEL DGEN D(LEVEL={LV} TYPE=1)\n.PARAM LV=200\n.END\n",
        "AKO generated diode namespace\nD1 1 0 DERIVED\n.MODEL BASE D(LEVEL=200)\n.MODEL DERIVED AKO:BASE (TYPE=1)\n.END\n",
        "AKO CMC diode namespace\nD1 1 0 DERIVED\n.MODEL BASE D(LEVEL=2002)\n.MODEL DERIVED AKO:BASE (VERSION=2.1)\n.END\n",
    ] {
        let netlist =
            Netlist::parse_with_options(source, xyce_options()).expect("generated diode parses");
        assert!(
            netlist.diagnostics.is_empty(),
            "the effective generated route owns its model namespace: {:?}",
            netlist.diagnostics
        );
    }
}

#[test]
fn xyce_final_native_diode_route_diagnoses_effective_unknown_parameters() {
    for (source, expected_model, expected_parameter) in [
        (
            "forward native diode namespace\nD1 1 0 DLEG\n.MODEL DLEG D(LEVEL={LV} BOGOPARAM=3)\n.PARAM LV=1\n.END\n",
            "DLEG",
            "BOGOPARAM",
        ),
        (
            "AKO native diode namespace\nD1 1 0 DERIVED\n.MODEL BASE D(LEVEL=200 TYPE=1)\n.MODEL DERIVED AKO:BASE (LEVEL=1)\n.END\n",
            "DERIVED",
            "TYPE",
        ),
    ] {
        let netlist =
            Netlist::parse_with_options(source, xyce_options()).expect("native diode parses");
        assert_eq!(
            netlist.diagnostics.len(),
            1,
            "the final native route must diagnose its effective namespace: {:?}",
            netlist.diagnostics
        );
        let diagnostic = &netlist.diagnostics[0];
        assert_eq!(diagnostic.code, "xyce-unknown-diode-model-parameter");
        assert!(diagnostic.message.contains(expected_model));
        assert!(diagnostic.message.contains(expected_parameter));
    }
}

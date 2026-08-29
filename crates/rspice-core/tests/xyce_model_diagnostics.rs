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
        "No model parameter BOGOPARAM found for model DLEG of type D, parameter ignored"
    );
}

#[test]
fn xyce_native_diode_parameter_namespace_does_not_emit_false_warnings() {
    let source = "native diode parameter namespace\n\
V1 1 0 1\n\
D1 1 0 DLEG\n\
.MODEL DLEG D(IS=1p JS=1p N=1 RS=0 KF=0 AF=1 BV=5 VB=5 VRB=5 VAR=5\n\
+ IBV=1u IB=1u IKF=1 IK=1 IKR=1 ISR=0 NR=2 CJO=1p CJ0=1p CJ=1p\n\
+ VJ=.7 PB=.7 M=.5 MJ=.5 TT=1n FC=.5 JSW=0 ISW=0 IKP=1 NS=1\n\
+ CJSW=0 CJP=0 PHP=1 VJSW=1 MJSW=.33 FCS=.5 NBV=1 NZ=1 XTI=3 EG=1.11\n\
+ TNOM=27 TREF=27 T_MEASURED=27 JTUN=0 JTUNSW=0 NTUN=20 XTITUN=3 KEG=1\n\
+ TLEV=0 TLEVC=0 GAP1=1 GAP2=1 TCV=0 TPB=0 TVJ=0 TPHP=0 CTA=0 CTC=0\n\
+ CTP=0 TRS=0 TRS1=0 TRS2=0 TM1=0 TM2=0 TTT1=0 TTT2=0 TBV1=0 TBV2=0\n\
+ AREA=1 PJ=0 XW=0 WM=0 LM=0 WP=0 LP=0 XOM=1 XOI=1 XM=0 XP=0)\n\
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

//! SPICE cells a Verilog-AMS design instantiates as leaves.

use rspice_veriloga::canonical_ir::{HirExternalLeaf, HirExternalNet, PortId};
use rspice_veriloga::{CompilerOptions, ExternalCell, VerilogACompiler};

fn cell(name: &str, ports: &[&str], parameters: &[&str]) -> ExternalCell {
    ExternalCell {
        name: name.to_owned(),
        ports: ports.iter().map(|port| (*port).to_owned()).collect(),
        parameters: parameters
            .iter()
            .map(|parameter| (*parameter).to_owned())
            .collect(),
    }
}

fn compiler(cells: Option<Vec<ExternalCell>>) -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        external_cells: cells,
        ..CompilerOptions::default()
    })
}

fn deck_cells() -> Vec<ExternalCell> {
    vec![cell("LDO", &["vin", "vout", "trim"], &["gain"])]
}

fn design(instances: &str) -> String {
    format!(
        "`include \"disciplines.vams\"\n\
         module top(a, b);\n\
         \x20 inout a, b; electrical a, b;\n\
         \x20 electrical m; electrical [1:0] bus;\n\
         \x20 ground g;\n\
         {instances}\n\
         endmodule\n"
    )
}

fn leaves(instances: &str) -> Vec<HirExternalLeaf> {
    compiler(Some(deck_cells()))
        .compile_canonical_ir_module(&design(instances), Some("top"))
        .expect("the design compiles with its leaves")
        .hir
        .external_leaves
}

fn refusal(instances: &str) -> String {
    compiler(Some(deck_cells()))
        .compile_canonical_ir_module(&design(instances), Some("top"))
        .expect_err("the design is refused")
        .to_string()
}

#[test]
fn a_subcircuit_and_a_resistor_compile_as_leaves() {
    let leaves = leaves(
        "  ldo #(.GAIN(2.5)) xl(a, m, b);\n\
         \x20 resistor #(1k) r1(.n(g), .p(m));",
    );
    assert_eq!(
        leaves,
        [
            HirExternalLeaf {
                cell: "LDO".into(),
                instance: "xl".into(),
                connections: vec![
                    HirExternalNet::Port(PortId::from(0)),
                    HirExternalNet::Net("m".into()),
                    HirExternalNet::Port(PortId::from(1)),
                ],
                parameters: vec![("gain".into(), 2.5)],
            },
            HirExternalLeaf {
                cell: "resistor".into(),
                instance: "r1".into(),
                connections: vec![HirExternalNet::Net("m".into()), HirExternalNet::Ground],
                parameters: vec![("r".into(), 1e3)],
            },
        ]
    );
}

#[test]
fn a_leaf_below_the_root_takes_its_occurrence_path() {
    let source = "`include \"disciplines.vams\"\n\
                  module half(x, y); inout x, y; electrical x, y;\n\
                  \x20 resistor #(.r(2e3)) r(x, y);\n\
                  endmodule\n\
                  module top(a, b); inout a, b; electrical a, b;\n\
                  \x20 half h(a, b);\n\
                  endmodule\n";
    let artifact = compiler(Some(Vec::new()))
        .compile_canonical_ir_module(source, Some("top"))
        .expect("a leaf inside a child compiles");
    assert_eq!(
        artifact.hir.external_leaves,
        [HirExternalLeaf {
            cell: "resistor".into(),
            instance: "h.r".into(),
            connections: vec![
                HirExternalNet::Port(PortId::from(0)),
                HirExternalNet::Port(PortId::from(1)),
            ],
            parameters: vec![("r".into(), 2e3)],
        }]
    );
}

#[test]
fn a_host_without_leaves_still_refuses_an_undefined_module() {
    let error = compiler(None)
        .compile_canonical_ir_module(&design("  resistor r1(a, b);"), Some("top"))
        .expect_err("no leaves without a host that builds them")
        .to_string();
    assert!(error.contains("Undefined module: 'resistor'"), "{error}");
    let error = refusal("  regulator xr(a, b);");
    assert!(error.contains("Undefined module: 'regulator'"), "{error}");
}

#[test]
fn each_unsupported_leaf_is_refused_by_name() {
    for (instances, expected) in [
        (
            "  diode d1(a, b);",
            "names the Verilog-AMS SPICE primitive `diode`",
        ),
        (
            "  ldo xl(a, m);",
            "leaves port `trim` unconnected; it has 3 ports",
        ),
        ("  ldo xl(a, m, b, g);", "has 4 ordered connections"),
        (
            "  ldo xl(.vin(a), .vout(m), .trim());",
            "leaves port `trim` unconnected; every port",
        ),
        (
            "  ldo xl(.vin(a), .vout(m), .sense(b));",
            "connects `sense`, which is not one of its ports",
        ),
        (
            "  ldo xl(a, bus, b);",
            "connects vector `bus`; a SPICE cell port is one scalar net",
        ),
        (
            "  ldo #(.vref(1)) xl(a, m, b);",
            "overrides `vref`, which is not one of its parameters",
        ),
        (
            "  ldo #(.gain(a)) xl(a, m, b);",
            "is not a numeric constant",
        ),
    ] {
        let error = refusal(instances);
        assert!(error.contains(expected), "{instances}: {error}");
    }
}

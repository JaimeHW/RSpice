use rspice_veriloga::{CompilerOptions, NoPipelineControl, VerilogACompiler};
use rspice_veriloga::{
    lexer::Lexer, parser::Parser, semantic::SemanticAnalyzer, source::SourceMap,
};

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
}

#[test]
fn physical_ranges_and_probe_selectors_replay_with_parameters() {
    let source = r#"
module top(p);
 parameter integer BASE=2, PICK=2;
 parameter real R=1000;
 inout [BASE:BASE+1] p; electrical [BASE:BASE+1] p;
 analog begin I(p[BASE])<+V(p[PICK])/R; I(p[BASE+1])<+V(p[BASE+1])/R; end
endmodule
"#;
    let tokens = Lexer::new(source, SourceMap::new().add_source("physical.vams", source))
        .collect_tokens()
        .unwrap();
    let ast = Parser::new(&tokens).parse().unwrap();
    let analyzed = SemanticAnalyzer::new().analyze(&ast).unwrap();
    let module = &analyzed.modules["top"];
    assert_eq!(
        module
            .ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>(),
        ["p[2]", "p[3]"]
    );
    for name in ["BASE", "PICK"] {
        assert_eq!(
            module
                .parameters
                .iter()
                .find(|p| p.name == name)
                .unwrap()
                .elaboration_value,
            Some(2.0)
        );
    }
    assert_eq!(
        module
            .parameters
            .iter()
            .find(|p| p.name == "R")
            .unwrap()
            .elaboration_value,
        None
    );
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    let assigned = compiler
        .specialize_mixed_runtime(
            &compiled.canonical_ir,
            &[("BASE", 5.0), ("PICK", 6.0)],
            &NoPipelineControl,
        )
        .unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap();
    let rebuilt = replay.compile_runtime(None).unwrap();
    assert_eq!(
        assigned.canonical_ir.runtime_source_identity(),
        rebuilt.canonical_ir.runtime_source_identity()
    );
}

#[test]
fn physical_vector_errors_do_not_silently_collapse_topology() {
    let cases = [
        (
            "module top(p); parameter [3:0] k=2; inout [0:1] p; electrical [0:1] p; real sample; initial begin : local_scope reg [3:0] k; k=1; sample=V(p[k[0]]); end endmodule",
            "process-local storage",
        ),
        (
            "module top(p); inout p; electrical p; genvar i; analog for(i=0;i<1;i=i+1) begin i=2; I(p)<+0; end endmodule",
            "cannot be written in the loop body",
        ),
        (
            "module top(p); inout p; electrical p; genvar i; analog V(p)<+i; endmodule",
            "requires an active analog for loop",
        ),
        (
            "module top(p); parameter integer k=2; inout [2:3] p; electrical [2:3] p; real sample; initial begin : local_scope integer k; k=3; sample=V(p[k]); end endmodule",
            "process-local storage",
        ),
        (
            "module top(p); input [2:3] p; logic [3:2] p; endmodule",
            "inconsistent vector ranges",
        ),
        (
            "module top(p); input [0:1] p; electrical [1:0] p; endmodule",
            "inconsistent vector ranges",
        ),
        (
            "module top(p); input p; electrical [1:0] p; endmodule",
            "inconsistent vector ranges",
        ),
        (
            "module top(p); inout [2:3] p; electrical [2:3] p; analog I(p[1])<+0; endmodule",
            "outside [2:3]",
        ),
        (
            "module top(p); inout p; electrical p; analog I(p[0])<+0; endmodule",
            "not a physical vector",
        ),
        (
            "module top(p); inout [2:3] p; electrical [2:3] p; integer k; analog I(p[k])<+0; endmodule",
            "integer at elaboration",
        ),
        (
            "module top(p); inout [2:3] p; electrical [2:3] p; analog I(p)<+0; endmodule",
            "requires a scalar coordinate",
        ),
        (
            "module top(p); inout [65536:0] p; electrical [65536:0] p; endmodule",
            "width exceeds",
        ),
        (
            "module leaf(a); input [4:3] a; electrical [4:3] a; endmodule module top(p); inout p; electrical p; leaf x(p); endmodule",
            "requires 2 lanes",
        ),
    ];
    for (source, expected) in cases {
        let result = compiler().compile_runtime(source, Some("top"));
        let error = match result {
            Ok(_) => panic!("accepted {source}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "expected {expected}: {error}");
    }
}

#[test]
fn selected_custom_nature_probes_and_escaped_names_remain_distinct() {
    let source = r#"
nature Heat; access=Temp; units="K"; abstol=1e-6; endnature
nature Flux; access=Pwr; units="W"; abstol=1e-12; endnature
discipline thermal; potential Heat; flow Flux; enddiscipline
module top(p,q);
 inout p,q; electrical p,q;
 thermal [0:1] t;
 thermal \t[0] ;
 analog begin
  Pwr(t[0])<+(Temp(t[0])-2.0);
  Pwr(t[1])<+(Temp(t[1])-3.0);
  Pwr(\t[0] )<+(Temp(\t[0] )-7.0);
  V(p)<+Temp(t[0])+Temp(t[1]);
  V(q)<+Temp(\t[0] );
 end
endmodule
"#;
    compiler().compile_runtime(source, Some("top")).unwrap();
}

#[test]
fn mixed_declaration_lists_and_static_loop_probes_keep_their_shapes() {
    let source = r#"
module top(a,d,p);
 parameter integer LAST=5;
 input [2:3] a,d;
 electrical [2:3] a;
 logic d;
 output p; electrical p;
 electrical [4:5] internal;
 genvar i;
 analog begin
  for (i=4;i<=LAST;i=i+1) I(internal[i])<+V(internal[i])-V(a[i-2]);
  V(p)<+V(internal[4],internal[5])+(d[2] ? 1.0 : 0.0)+(d[3] ? 2.0 : 0.0);
 end
endmodule
"#;
    let tokens = Lexer::new(source, SourceMap::new().add_source("mixed.vams", source))
        .collect_tokens()
        .unwrap();
    let ast = Parser::new(&tokens).parse().unwrap();
    let analyzed = SemanticAnalyzer::new().analyze(&ast).unwrap();
    let module = &analyzed.modules["top"];
    let digital = module
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == "d")
        .unwrap();
    assert_eq!(digital.width, 2);
    assert_eq!(digital.range.unwrap().msb, 2);
    assert_eq!(module.internal_nodes.len(), 2);
    assert_eq!(
        module
            .parameters
            .iter()
            .find(|parameter| parameter.name == "LAST")
            .unwrap()
            .elaboration_value,
        Some(5.0)
    );
    compiler().compile_runtime(source, Some("top")).unwrap();
}

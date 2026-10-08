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
            "module leaf(a); input [1:0] a; electrical [1:0] a; endmodule module top; reg [1:0] words[3:2][-1:0]; leaf l(words[3]); endmodule",
            "requires 2 unpacked coordinates",
        ),
        (
            "module leaf(a); input [1:0] a; electrical [1:0] a; endmodule module top; reg [1:0] words[3:2][-1:0]; leaf l(words); endmodule",
            "requires 2 unpacked coordinates",
        ),
        (
            "module leaf(a); input [1:0] a; electrical [1:0] a; endmodule module top; reg [-2:-1] words[3:2]; leaf l(words[3][-1:-2]); endmodule",
            "packed part-select direction disagrees",
        ),
        (
            "module leaf(a); input [1:0] a; electrical [1:0] a; endmodule module top; reg [-2:-1] word; leaf l(word[-1:-2]); endmodule",
            "packed part-select direction disagrees",
        ),
        (
            "module leaf(a); input [1:0] a; electrical [1:0] a; endmodule module top; reg [1:0] words[3:2]; integer pick; leaf l(words[pick]); endmodule",
            "integer at elaboration",
        ),
        (
            "module leaf(a); inout a; wire a; endmodule module top; wire [3:2] bus; leaf l(bus[1]); endmodule",
            "requires an in-range wire selection",
        ),
        (
            "module leaf(a); inout [1:0] a; wire [1:0] a; endmodule module top; wire [3:2] bus; leaf l(bus[2:3]); endmodule",
            "requires an in-range wire selection",
        ),
        (
            "module leaf(a); inout a; wire a; endmodule module top; reg [3:2] bus; leaf l(bus[2]); endmodule",
            "variable connected to the `inout` port",
        ),
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

#[test]
fn bidirectional_selections_keep_wire_identity_and_validate_artifacts() {
    use rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan;
    let source = r#"
module leaf(io);
 inout [6:7] io; wire [6:7] io;
 assign io=2'bzz;
endmodule
module middle(io);
 parameter integer PICK=8;
 inout [9:6] io; wire [9:6] io;
 leaf child(io[PICK:PICK-1]);
endmodule
module top(bus);
 inout [2:5] bus; wire [2:5] bus;
 middle m(bus);
endmodule
"#;
    let artifact = compiler()
        .compile_canonical_ir_module(source, Some("top"))
        .unwrap();
    let plan = &artifact.digital;
    assert_eq!(plan.bit_aliases.len(), 2);
    assert_eq!(
        plan.drivers.len(),
        1,
        "a wire identity must not synthesize a feedback driver"
    );
    for (alias, (left, right)) in plan.bit_aliases.iter().zip([(1, 2), (0, 1)]) {
        assert_eq!(plan.signal(alias.left.signal).unwrap().name, "m.child.io");
        assert_eq!(plan.signal(alias.right.signal).unwrap().name, "bus");
        assert_eq!((alias.left.bit, alias.right.bit), (left, right));
    }
    let encoded = serde_json::to_vec(plan).unwrap();
    let decoded: CanonicalDigitalPlan = serde_json::from_slice(&encoded).unwrap();
    decoded.validate().unwrap();
    let mut changed = decoded.clone();
    changed.bit_aliases[0].right.bit = 0;
    assert!(
        changed
            .validate()
            .unwrap_err()
            .iter()
            .any(|d| d.to_string().contains("identity is stale"))
    );
    let mut invalid = decoded;
    invalid.bit_aliases[0].right.bit = 4;
    assert!(
        invalid
            .validate()
            .unwrap_err()
            .iter()
            .any(|d| d.to_string().contains("in-range four-state wire bit"))
    );
}

#[test]
fn physical_to_packed_buses_retain_merged_aliases_through_specialization_and_replay() {
    use std::collections::HashSet;
    for (mode, drivers) in [("merged", 2), ("split", 4)] {
        let source = format!(
            r#"
module receiver(d);
 input [2:3] d; logic [2:3] d;
endmodule
module top(p);
 parameter integer BASE=7;
 inout [BASE:BASE+1] p; electrical [BASE:BASE+1] p;
 receiver first(p);
 receiver second(p[BASE:BASE+1]);
 analog begin I(p[BASE])<+V(p[BASE])/1000; I(p[BASE+1])<+V(p[BASE+1])/1000; end
endmodule
connectmodule sample(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #1 d=V(a)>0.5;
endmodule
connectrules selected; connect sample {mode}; endconnectrules
"#
        );
        let compiler = compiler();
        let compiled = compiler.compile_runtime(&source, Some("top")).unwrap();
        let check = |ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact| {
            assert_eq!(ir.digital.bit_aliases.len(), 4);
            assert_eq!(
                ir.digital
                    .bit_aliases
                    .iter()
                    .map(|alias| alias.left.signal)
                    .collect::<HashSet<_>>()
                    .len(),
                drivers
            );
            ir.validate().unwrap();
        };
        check(&compiled.canonical_ir);
        let assigned = compiler
            .specialize_mixed_runtime(
                &compiled.canonical_ir,
                &[("BASE", -3.0)],
                &NoPipelineControl,
            )
            .unwrap();
        check(&assigned.canonical_ir);
        let prepared = compiler
            .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
            .unwrap();
        let replayed = prepared.compile_runtime(None).unwrap();
        check(&replayed.canonical_ir);
        assert_eq!(
            assigned.canonical_ir.digital.content_identity,
            replayed.canonical_ir.digital.content_identity
        );
    }
}

#[test]
fn packed_array_words_specialize_and_replay_physical_connections() {
    let source = r#"
module load(a,p);
 input [4:3] a; electrical [4:3] a;
 output p; electrical p;
 analog V(p)<+V(a[4])+2.0*V(a[3]);
endmodule
module top(p,q);
 parameter integer ROW=3, COLUMN=0;
 parameter real R=1000;
 output p,q; electrical p,q;
 reg [-2:-1] words[3:2][-1:0];
 initial begin words[3][0]=2'b10; words[2][-1]=2'b01; end
 load first(words[$param_given(ROW) ? ROW : 3][COLUMN],p);
 load second(words[ROW][COLUMN][-2:-1],q);
 analog I(p)<+V(p)/R;
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectrules selected; connect drive; endconnectrules
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    for (name, expected) in [("ROW", Some(3.0)), ("COLUMN", Some(0.0)), ("R", None)] {
        assert_eq!(
            compiled
                .canonical_ir
                .hir
                .parameters
                .iter()
                .find(|p| p.name == name)
                .unwrap()
                .elaboration_value,
            expected,
            "{name}",
        );
    }
    assert_eq!(
        compiled
            .canonical_ir
            .hir
            .parameters
            .iter()
            .find(|p| p.name == "ROW")
            .unwrap()
            .elaboration_given,
        Some(false)
    );
    let assigned = compiler
        .specialize_mixed_runtime(
            &compiled.canonical_ir,
            &[("ROW", 2.0), ("COLUMN", -1.0)],
            &NoPipelineControl,
        )
        .unwrap();
    assert_ne!(
        compiled.canonical_ir.digital.content_identity,
        assigned.canonical_ir.digital.content_identity
    );
    let prepared = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap();
    let replayed = prepared.compile_runtime(None).unwrap();
    assert_eq!(
        assigned.canonical_ir.digital.content_identity,
        replayed.canonical_ir.digital.content_identity
    );
    // An absent digital element remains an X-valued read, not a nonexistent physical node.
    let out_of_range = source.replace("ROW=3", "ROW=7");
    compiler
        .compile_runtime(&out_of_range, Some("top"))
        .unwrap();
}

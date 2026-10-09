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
fn continuous_wire_declarations_share_physical_storage_and_replay() {
    let source = r#"
module source(output electrical wire a);
 analog I(a)<+(V(a)-2.5)/1000;
endmodule
module load(input electrical wire [1:0] a,output electrical wire p);
 analog begin I(a[1])<+V(a[1])/1000; I(a[0])<+V(a[0])/1000; V(p)<+V(a[1])+V(a[0]); end
endmodule
module top(output electrical wire p,q);
 parameter integer BASE=-2;
 wire [5:4] bus; electrical [5:4] bus;
 source first(bus[5]),second(bus[4]); load packed_load(bus,p);
 generate begin : channels
   tri electrical [BASE:BASE+1] cells[3:3];
   source first(cells[3][BASE]),second(cells[3][BASE+1]);
   load array_load(cells[3],q);
 end endgenerate
endmodule
"#;
    let compiler = VerilogACompiler::default();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("BASE", 7.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        assert!(report.canonical_ir.digital.signals.is_empty());
        assert!(report.canonical_ir.digital.processes.is_empty());
        assert!(report.canonical_ir.digital.drivers.is_empty());
        let replay = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            replay.canonical_ir.runtime_source_identity()
        );
    }
}

#[test]
fn continuous_wire_normalization_preserves_discrete_siblings_and_probe_reads() {
    let source = r#"
`timescale 1ns/1ps
module top(output electrical p);
 wire physical,discrete; electrical physical;
 real sampled;
 assign discrete=1'b1;
 initial #1 sampled=V(physical);
 analog begin V(physical)<+2.5; V(p)<+sampled+discrete; end
endmodule
"#;
    let artifact = compiler().compile_runtime(source, Some("top")).unwrap();
    let names: Vec<_> = artifact
        .canonical_ir
        .digital
        .signals
        .iter()
        .map(|signal| signal.name.as_str())
        .collect();
    assert!(names.contains(&"discrete"));
    assert!(names.contains(&"sampled"));
    assert!(!names.contains(&"physical"));
}

#[test]
fn continuous_wire_declarations_reject_conflicting_shapes_and_digital_drivers() {
    for (body, expected) in [
        (
            "wire electrical a=1;",
            "cannot be driven by a digital net declaration assignment",
        ),
        (
            "wire a=1; electrical a;",
            "cannot be driven by a digital net declaration assignment",
        ),
        (
            "wire [2:1] a; electrical [1:2] a;",
            "inconsistent wire and discipline shapes",
        ),
        (
            "wire a[2:1]; electrical a[1:2];",
            "inconsistent wire and discipline shapes",
        ),
        (
            "wire a[2:1]; electrical a;",
            "inconsistent wire and discipline shapes",
        ),
        (
            "wire a; wire a; electrical a;",
            "duplicate net type declaration",
        ),
        (
            "wreal electrical [1:0] a;",
            "requires a discrete discipline",
        ),
        ("reg electrical [1:0] a;", "requires a discrete discipline"),
    ] {
        let source = format!("module top; {body} endmodule");
        let error = compiler()
            .compile_runtime(&source, Some("top"))
            .unwrap_err()
            .to_string();
        assert!(error.contains(expected), "{body}: {error}");
    }
    for body in ["assign a=1'b1;", "initial a=1'b1;"] {
        let source = format!("module top; wire electrical a; {body} endmodule");
        assert!(
            compiler().compile_runtime(&source, Some("top")).is_err(),
            "{body}"
        );
    }
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
    assert_eq!(plan.bit_aliases.len(), 6);
    assert_eq!(
        plan.drivers.len(),
        1,
        "a wire identity must not synthesize a feedback driver"
    );
    let mut aliases = plan
        .bit_aliases
        .iter()
        .map(|alias| {
            (
                plan.signal(alias.left.signal).unwrap().name.as_str(),
                alias.left.bit,
                plan.signal(alias.right.signal).unwrap().name.as_str(),
                alias.right.bit,
            )
        })
        .collect::<Vec<_>>();
    aliases.sort_unstable();
    assert_eq!(
        aliases,
        [
            ("m.child.io", 0, "m.io", 1),
            ("m.child.io", 1, "m.io", 2),
            ("m.io", 0, "bus", 0),
            ("m.io", 1, "bus", 1),
            ("m.io", 2, "bus", 2),
            ("m.io", 3, "bus", 3),
        ]
    );
    let encoded = serde_json::to_vec(plan).unwrap();
    let decoded: CanonicalDigitalPlan = serde_json::from_slice(&encoded).unwrap();
    decoded.validate().unwrap();
    let mut changed = decoded.clone();
    changed.bit_aliases[0].right.bit ^= 1;
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

#[test]
fn mixed_concatenated_wire_aliases_specialize_and_replay() {
    let source = r#"
module leaf(d);
 inout [-2:-1] d; logic [-2:-1] d;
 assign d=2'bzz;
endmodule
module top(a);
 parameter integer PICK=5;
 inout a; electrical a;
 wire [5:4] bus;
 leaf child({a,bus[PICK]});
 analog I(a)<+V(a)/1000;
endmodule
connectmodule bidirectional(d,a);
 inout d; logic d;
 inout a; electrical a;
 analog I(a)<+V(a)/1000;
endmodule
connectrules selected; connect bidirectional; endconnectrules
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    let check = |ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact, selected| {
        ir.validate().unwrap();
        assert_eq!(ir.digital.bit_aliases.len(), 2);
        let alias = ir
            .digital
            .bit_aliases
            .iter()
            .find(|alias| ir.digital.signal(alias.right.signal).unwrap().name == "bus")
            .unwrap();
        assert_eq!((alias.left.bit, alias.right.bit), (0, selected));
        assert_eq!(
            ir.digital.drivers.len(),
            1,
            "wire aliases must not create feedback drivers"
        );
    };
    check(&compiled.canonical_ir, 1);
    assert_eq!(
        compiled
            .canonical_ir
            .hir
            .parameters
            .iter()
            .find(|p| p.name == "PICK")
            .unwrap()
            .elaboration_value,
        Some(5.0)
    );
    let assigned = compiler
        .specialize_mixed_runtime(&compiled.canonical_ir, &[("PICK", 4.0)], &NoPipelineControl)
        .unwrap();
    check(&assigned.canonical_ir, 0);
    let replayed = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    check(&replayed.canonical_ir, 0);
    assert_eq!(
        assigned.canonical_ir.digital.content_identity,
        replayed.canonical_ir.digital.content_identity
    );
}

#[test]
fn mixed_concatenations_refuse_invalid_drive_targets() {
    let cases = [
        (
            "output",
            "reg d;",
            "{a,d}",
            "requires a net, not a variable",
        ),
        ("inout", "reg d;", "{a,d}", "requires a net, not a variable"),
        (
            "output",
            "input d; logic d;",
            "{a,d}",
            "cannot drive a parent input port",
        ),
        (
            "inout",
            "wire [5:4] d;",
            "{a,d[3]}",
            "requires an in-range wire bit",
        ),
        (
            "input",
            "wreal d;",
            "{a,d}",
            "requires scalar physical or four-state digital lanes",
        ),
        (
            "output",
            "wire d;",
            "{a,{1{d}}}",
            "replicated concatenations cannot connect",
        ),
        (
            "inout",
            "wire d;",
            "{a,{1{d}}}",
            "replicated concatenations cannot connect",
        ),
    ];
    for (direction, declaration, connection, expected) in cases {
        let ports = if declaration.starts_with("input") {
            "a,d"
        } else {
            "a"
        };
        let source = format!(
            r#"
module leaf(d); {direction} [1:0] d; logic [1:0] d; endmodule
module top({ports}); inout a; electrical a; {declaration} leaf child({connection}); endmodule
connectmodule bidirectional(d,a);
 inout d; logic d;
 inout a; electrical a;
 analog I(a)<+V(a)/1000;
endmodule
connectrules selected; connect bidirectional; endconnectrules
"#
        );
        let result = compiler().compile_runtime(&source, Some("top"));
        let error = match result {
            Ok(_) => panic!("accepted {source}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "expected {expected}: {error}");
    }
}

#[test]
fn computed_input_connections_specialize_and_replay_without_connect_rules() {
    let source = r#"
module leaf(d,p);
 parameter integer K=99;
 input [7:0] d; wire [7:0] d;
 output p; electrical p;
 analog V(p)<+d;
endmodule
module top(p);
 parameter integer K=2;
 output p; electrical p;
 reg [3:0] value;
 initial value=4'd15;
 leaf child(value+K,p);
endmodule
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    assert_eq!(
        compiled
            .canonical_ir
            .hir
            .parameters
            .iter()
            .find(|p| p.name == "K")
            .unwrap()
            .elaboration_value,
        Some(2.0)
    );
    let assigned = compiler
        .specialize_mixed_runtime(&compiled.canonical_ir, &[("K", 5.0)], &NoPipelineControl)
        .unwrap();
    assert_ne!(
        compiled.canonical_ir.digital.content_identity,
        assigned.canonical_ir.digital.content_identity
    );
    let replayed = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        assigned.canonical_ir.digital.content_identity,
        replayed.canonical_ir.digital.content_identity
    );
    replayed.canonical_ir.validate().unwrap();
}


#[test]
fn computed_mixed_connections_replay_shapes_and_preserve_selector_guards() {
    let source = r#"
module leaf(d);
 parameter integer W=12;
 input [W-1:0] d; logic [W-1:0] d;
endmodule
module top(a);
 parameter integer BASE=2, ROW=3, COUNT=2;
 inout [2:5] a; electrical [2:5] a;
 reg [3:0] words[3:2];
 leaf #(.W(2+5*COUNT)) child({a[BASE],{COUNT{a[BASE+1],words[ROW]+4'd1}},1'b1});
 analog I(a[2])<+V(a[2])/1000;
endmodule
connectmodule sample(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
endmodule
connectrules selected; connect sample split; endconnectrules
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    for (name, expected) in [("BASE", 2.0), ("ROW", 3.0), ("COUNT", 2.0)] {
        assert_eq!(
            compiled
                .canonical_ir
                .hir
                .parameters
                .iter()
                .find(|p| p.name == name)
                .unwrap()
                .elaboration_value,
            Some(expected),
            "{name}"
        );
    }
    assert_eq!(compiled.canonical_ir.digital.bit_aliases.len(), 3);
    for count in [1.0, 0.0] {
        let assigned = compiler
            .specialize_mixed_runtime(
                &compiled.canonical_ir,
                &[("BASE", 4.0), ("ROW", 2.0), ("COUNT", count)],
                &NoPipelineControl,
            )
            .unwrap();
        assigned.canonical_ir.validate().unwrap();
        assert_eq!(
            assigned.canonical_ir.digital.bit_aliases.len(),
            1 + count as usize
        );
        assert_ne!(
            compiled.canonical_ir.digital.content_identity,
            assigned.canonical_ir.digital.content_identity
        );
        let replayed = compiler
            .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            assigned.canonical_ir.digital.content_identity,
            replayed.canonical_ir.digital.content_identity
        );
    }
    let mismatched = source.replace("W(2+5*COUNT)", "W(3+5*COUNT)");
    let error = compiler
        .compile_runtime(&mismatched, Some("top"))
        .err()
        .expect("mismatched mixed width")
        .to_string();
    assert!(
        error.contains("requires 13 lanes") && error.contains("supplies 12"),
        "{error}"
    );
}

#[test]
fn vector_branches_retain_terminal_order_ranges_and_specialization() {
    let source = r#"
module top(p,n,q);
 parameter integer BASE=2, PICK=2, FIRST=-2;
 inout [BASE:BASE+1] p; electrical [BASE:BASE+1] p;
 inout n,q; electrical n,q;
 branch(p,n) load[FIRST:FIRST+1], spare;
 branch(n,p[BASE:BASE+1]) reverse[8:7];
 branch(p[PICK],0) chosen;
 branch(<p>) probe;
 real sampled;
 analog begin
  I(load[FIRST])<+V(load[FIRST])/1000;
  I(load[FIRST+1])<+V(load[FIRST+1])/2000;
  V(q)<+1000*(I(probe[0])+I(<p[BASE+1]>))+sampled;
 end
 initial begin #1; sampled=I(load[FIRST+1])+I(probe[1])+I(<p[BASE]>); end
endmodule
"#;
    let tokens = Lexer::new(source, SourceMap::new().add_source("branches.vams", source))
        .collect_tokens()
        .unwrap();
    let ast = Parser::new(&tokens).parse().unwrap();
    let analyzed = SemanticAnalyzer::new().analyze(&ast).unwrap();
    let module = &analyzed.modules["top"];
    let endpoints = |name: &str| {
        let branch = module
            .branches
            .iter()
            .find(|branch| branch.name == name)
            .unwrap();
        (branch.pos_node.as_str(), branch.neg_node.as_str())
    };
    assert_eq!(endpoints("load[-2]"), ("p[2]", "n"));
    assert_eq!(endpoints("load[-1]"), ("p[3]", "n"));
    assert_eq!(endpoints("spare[0]"), ("p[2]", "n"));
    assert_eq!(endpoints("spare[1]"), ("p[3]", "n"));
    assert_eq!(endpoints("reverse[8]"), ("n", "p[2]"));
    assert_eq!(endpoints("reverse[7]"), ("n", "p[3]"));
    assert_eq!(endpoints("chosen"), ("p[2]", "0"));
    assert!(
        !module
            .branches
            .iter()
            .any(|branch| branch.name.starts_with("probe"))
    );
    for (name, value) in [("BASE", 2.0), ("PICK", 2.0), ("FIRST", -2.0)] {
        assert_eq!(
            module
                .parameters
                .iter()
                .find(|p| p.name == name)
                .unwrap()
                .elaboration_value,
            Some(value)
        );
    }
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    let assigned = compiler
        .specialize_mixed_runtime(
            &compiled.canonical_ir,
            &[("BASE", 5.0), ("PICK", 6.0), ("FIRST", 8.0)],
            &NoPipelineControl,
        )
        .unwrap();
    assigned.canonical_ir.validate().unwrap();
    assert_ne!(
        compiled.canonical_ir.runtime_source_identity(),
        assigned.canonical_ir.runtime_source_identity()
    );
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
fn branch_shapes_and_port_probe_misuse_are_rejected() {
    let cases = [
        (
            "electrical [1:0] a; electrical [2:0] b; branch(a,b) x;",
            "equal widths",
        ),
        (
            "electrical [0:0] a; electrical [2:0] b; branch(a,b) x;",
            "equal widths",
        ),
        (
            "electrical [1:0] a; branch(a) x[4:2];",
            "terminals require 2",
        ),
        (
            "electrical [1:0] a; branch(a[0:1]) x;",
            "direction disagrees",
        ),
        ("electrical [1:0] a; branch(a[2]) x;", "outside [1:0]"),
        ("electrical a; branch(a) x; branch(x) y;", "continuous nets"),
        ("electrical a; branch(<a>) x;", "declared port"),
        ("branch(<p>) x; analog V(q)<+V(x[0]);", "flow-probed port"),
        (
            "branch(<p>) x; analog I(x[0])<+1;",
            "cannot be a contribution target",
        ),
        ("analog I(<p[0]>)<+1;", "cannot be a contribution target"),
        ("analog V(q)<+V(<p[0]>);", "flow-probed port"),
        ("integer k; initial k=I(<p[k]>);", "integer at elaboration"),
        (
            "branch(p) x; analog V(q)<+I(x);",
            "requires a scalar coordinate",
        ),
        ("branch(p) x,x;", "duplicate branch declaration"),
        ("branch(p) p;", "duplicate branch declaration"),
        (
            "branch(<p>) x; real result; initial begin : local_scope integer x=0; result=I(x[0]); end",
            "process-local storage",
        ),
        (
            "real result; initial begin : local_scope integer p=0; result=I(<p[0]>); end",
            "process-local storage",
        ),
    ];
    for (body, expected) in cases {
        let source = format!(
            "module top(p,q); inout [0:1] p; electrical [0:1] p; inout q; electrical q; {body} endmodule"
        );
        let error = match compiler().compile_runtime(&source, Some("top")) {
            Ok(_) => panic!("accepted {body}"),
            Err(error) => error.to_string(),
        };
        assert!(
            error.contains(expected),
            "{body}: expected {expected}, got {error}"
        );
    }
}

#[test]
fn wire_array_connections_replay_element_identity_and_reject_invalid_drives() {
    let source = r#"
module drive(p); inout [0:1] p; wire [0:1] p; assign p=2'b10; endmodule
module top(p);
 parameter integer BASE=4, PICK=4;
 output p; electrical p;
 wire [5:4] cells[BASE:BASE-1][-1:0];
 wire [1:0] selected;
 drive driver(cells[PICK][-1]);
 assign cells[BASE][0]=2'b01;
 assign selected=cells[PICK][-1];
 analog V(p)<+selected;
endmodule
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    let check = |ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact| {
        ir.validate().unwrap();
        let array = ir
            .digital
            .arrays
            .iter()
            .find(|array| array.name == "cells")
            .unwrap();
        assert_eq!(array.storage.len, 4);
        for offset in 0..array.storage.len {
            assert!(
                !ir.digital.signals[array.storage.base.index() as usize + offset as usize]
                    .procedurally_assignable
            );
        }
        assert!(ir.digital.bit_aliases.len() >= 2);
    };
    check(&compiled.canonical_ir);
    let assigned = compiler
        .specialize_mixed_runtime(
            &compiled.canonical_ir,
            &[("BASE", 8.0), ("PICK", 7.0)],
            &NoPipelineControl,
        )
        .unwrap();
    check(&assigned.canonical_ir);
    assert_ne!(
        compiled.canonical_ir.digital.content_identity,
        assigned.canonical_ir.digital.content_identity
    );
    let replay = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        assigned.canonical_ir.digital.content_identity,
        replay.canonical_ir.digital.content_identity
    );
    for (body, expected) in [
        ("initial cells[0]=2'b10;", "procedural"),
        ("integer pick; assign cells[pick]=2'b10;", "constant"),
        ("assign cells[2]=2'b10;", "in-range unpacked"),
    ] {
        let source = format!("module top; wire [1:0] cells[0:1]; {body} endmodule");
        let error = match compiler.compile_runtime(&source, Some("top")) {
            Ok(_) => panic!("accepted {body}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "{body}: {error}");
    }
}


#[test]
fn real_net_array_connections_replay_shared_element_identity() {
    let source = r#"
module drive(p); inout p; wrealsum p; assign p=2.0; endmodule
module top(p);
 parameter integer BASE=4, PICK=3;
 output p; electrical p;
 wrealsum cells[BASE:BASE-1][-1:0];
 drive first(cells[PICK][-1]);
 drive second(cells[PICK][-1]);
 assign cells[BASE-1][-1]=1.0;
 analog V(p)<+cells[PICK][-1];
endmodule
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    let check = |ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact, first_word: bool| {
        ir.validate().unwrap();
        let array = ir
            .digital
            .arrays
            .iter()
            .find(|array| array.name == "cells")
            .unwrap();
        assert_eq!(array.storage.len, 4);
        // Both inout ports and the compiler's scalar view share existing words.
        assert_eq!(ir.digital.signals.len(), 4);
        assert!(ir.digital.bit_aliases.is_empty());
        let target = array.storage.base.index() + if first_word { 0 } else { 2 };
        assert_eq!(
            ir.digital
                .drivers
                .iter()
                .filter(|driver| driver.target.signal.index() == target)
                .count(),
            if first_word { 3 } else { 2 }
        );
    };
    check(&compiled.canonical_ir, true);
    let assigned = compiler
        .specialize_mixed_runtime(
            &compiled.canonical_ir,
            &[("BASE", 8.0), ("PICK", 8.0)],
            &NoPipelineControl,
        )
        .unwrap();
    check(&assigned.canonical_ir, false);
    assert_ne!(
        compiled.canonical_ir.digital.content_identity,
        assigned.canonical_ir.digital.content_identity
    );
    let replay = compiler
        .prepare_artifact_runtime_source(&assigned.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        assigned.canonical_ir.digital.content_identity,
        replay.canonical_ir.digital.content_identity
    );
    for body in [
        "initial cells[0]=1.0;",
        "assign cells[0][0]=1'b1;",
        "drive bad(cells[0][0]);",
        "drive bad(cells[2]);",
    ] {
        let source = format!(
            "module drive(p); inout p; wrealsum p; assign p=2.0; endmodule module top; wrealsum cells[0:1]; {body} endmodule"
        );
        assert!(
            compiler.compile_runtime(&source, Some("top")).is_err(),
            "accepted {body}"
        );
    }
}


#[test]
fn real_net_disciplines_survive_array_specialization_and_source_replay() {
    let source = r#"
discipline sense; domain discrete; enddiscipline
module reader(input sense wreal value, output electrical p);
 analog V(p)<+value;
endmodule
module top(p);
 parameter integer BASE=-2;
 output p; electrical p;
 wreal sense cells[BASE:BASE+1];
 assign cells[BASE]=1.25;
 reader selected(cells[BASE],p);
endmodule
"#;
    let compiler = compiler();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&compiled.canonical_ir, &[("BASE", 5.0)], &NoPipelineControl)
        .unwrap();
    specialized.canonical_ir.validate().unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        specialized.canonical_ir.digital.content_identity,
        replay.canonical_ir.digital.content_identity
    );
    assert!(
        replay
            .canonical_ir
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "cells[5]")
    );
}

#[test]
fn real_net_alias_artifacts_preserve_identity_validate_and_link_transitively() {
    use rspice_veriloga::canonical_ir::digital::{
        DigitalRealAlias, DigitalRealResolution, DigitalSignalKind,
    };
    use rspice_veriloga::canonical_ir::digital_link::{
        DigitalLinkDirection, DigitalLinkInstance, DigitalLinkNet, DigitalLinkPort,
        link_digital_plans,
    };
    let artifact = compiler()
        .compile_runtime(
            "module top(a,b); inout a,b; wrealsum a,b; assign a=2.0; endmodule",
            Some("top"),
        )
        .unwrap();
    let plan = artifact.canonical_ir.digital.clone();
    let a = plan.signals.iter().find(|s| s.name == "a").unwrap().id;
    let b = plan.signals.iter().find(|s| s.name == "b").unwrap().id;
    let alias = DigitalRealAlias {
        left: a,
        right: b,
        span: plan.signal(a).unwrap().span,
    };
    let plan = plan.with_real_aliases([alias]).unwrap();
    assert_ne!(
        artifact.canonical_ir.digital.content_identity,
        plan.content_identity
    );
    let encoded = serde_json::to_vec(&plan).unwrap();
    let decoded: rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan =
        serde_json::from_slice(&encoded).unwrap();
    decoded.validate().unwrap();
    assert_eq!(decoded, plan);
    let ports = [DigitalLinkPort {
        name: "b".into(),
        signal: b,
        direction: DigitalLinkDirection::Inout,
    }];
    let nets = [DigitalLinkNet {
        name: "joined".into(),
        ports: vec![("first".into(), "b".into()), ("second".into(), "b".into())],
    }];
    let linked = link_digital_plans(
        &[
            DigitalLinkInstance {
                name: "first",
                plan: &plan,
                ports: &ports,
            },
            DigitalLinkInstance {
                name: "second",
                plan: &plan,
                ports: &ports,
            },
        ],
        &nets,
        &NoPipelineControl,
    )
    .unwrap();
    let representatives = linked.plan.real_net_representatives().unwrap();
    assert!(
        representatives
            .iter()
            .all(|representative| *representative == representatives[0])
    );
    assert_eq!(linked.plan.drivers.len(), 2);
    for invalid in 0..3 {
        let mut changed = plan.clone();
        match invalid {
            0 => {
                changed.real_aliases[0].right =
                    rspice_veriloga::canonical_ir::DigitalSignalId::new(999)
            }
            1 => changed.signals[usize::from(b)].procedurally_assignable = true,
            _ => {
                changed.signals[usize::from(b)].kind =
                    DigitalSignalKind::Real(DigitalRealResolution::Average)
            }
        }
        assert!(changed.validate().is_err());
    }
    let single = compiler()
        .compile_runtime(
            "module top(a,b); inout a,b; wreal a,b; assign a=2.0; endmodule",
            Some("top"),
        )
        .unwrap()
        .canonical_ir
        .digital;
    let alias = DigitalRealAlias {
        left: a,
        right: b,
        span: single.signal(a).unwrap().span,
    };
    let single = single.with_real_aliases([alias]).unwrap();
    assert!(
        link_digital_plans(
            &[
                DigitalLinkInstance {
                    name: "first",
                    plan: &single,
                    ports: &ports
                },
                DigitalLinkInstance {
                    name: "second",
                    plan: &single,
                    ports: &ports
                },
            ],
            &nets,
            &NoPipelineControl
        )
        .is_err()
    );
}


#[test]
fn real_buses_preserve_local_ranges_driver_groups_and_specialized_replay() {
    let source = r#"
module leaf(inout logic wrealsum [5:4] r);
 assign r[5]=2.0; assign r[4]=4.0;
endmodule
module middle(inout logic wrealsum [-1:0] q);
 leaf inner(q);
endmodule
module top(p);
 parameter integer BASE=3;
 output p; electrical p;
 wrealsum logic [BASE:BASE+1] bus;
 assign bus[BASE]=1.0; assign bus[BASE+1]=10.0;
 middle nested(bus[BASE:BASE+1]);
 analog V(p)<+bus[BASE]+bus[BASE+1];
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let check = |ir: &rspice_veriloga::canonical_ir::CanonicalIrArtifact, base: i64| {
        ir.validate().unwrap();
        let plan = &ir.digital;
        assert_eq!(plan.arrays.len(), 3);
        assert_eq!(plan.signals.len(), 6);
        assert_eq!(plan.drivers.len(), 4);
        assert_eq!(
            plan.arrays
                .iter()
                .find(|array| array.name == "bus")
                .unwrap()
                .bounds,
            (base, base + 1)
        );
        let representatives = plan.real_net_representatives().unwrap();
        let mut groups = std::collections::BTreeMap::new();
        for representative in &representatives {
            *groups.entry(*representative).or_insert(0usize) += 1;
        }
        assert_eq!(groups.len(), 2);
        assert!(groups.values().all(|count| *count == 3));
        for representative in groups.keys() {
            assert_eq!(
                plan.drivers
                    .iter()
                    .filter(
                        |driver| representatives[usize::from(driver.id.signal)] == *representative
                    )
                    .count(),
                2
            );
        }
    };
    check(&artifact.canonical_ir, 3);
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("BASE", 8.0)], &NoPipelineControl)
        .unwrap();
    check(&specialized.canonical_ir, 8);
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    check(&replay.canonical_ir, 8);
    for (bad, expected) in [
        (
            source.replace(
                "leaf(inout logic wrealsum [5:4]",
                "leaf(inout logic wrealavg [5:4]",
            ),
            "same real resolution",
        ),
        (source.replace("[-1:0]", "[-1:1]"), "requires 3 lanes"),
        (
            source.replace("bus[BASE:BASE+1]);", "bus[BASE+1:BASE]);"),
            "declared direction",
        ),
        (
            source.replace("inout logic wrealsum [5:4]", "input logic wrealsum [5:4]"),
            "input",
        ),
        (
            source.replace("wrealsum", "wreal"),
            "multiple independent drivers",
        ),
    ] {
        let error = match compiler.compile_runtime(&bad, Some("top")) {
            Ok(_) => panic!("accepted {expected}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "{expected}: {error}");
    }
}

#[test]
fn multidimensional_real_bus_words_and_parts_keep_element_coordinates() {
    let source = r#"
module leaf(inout logic wrealsum [7:6] p);
 assign p[7]=2.0; assign p[6]=3.0;
endmodule
module top(p);
 output p; electrical p;
 wrealsum logic [4:2] bank[-2:-1][8:9];
 leaf first(bank[-1][8][4:3]);
 leaf second({bank[-2][9][2],bank[-1][9][4]});
 analog V(p)<+bank[-1][8][4]+bank[-2][9][2];
endmodule
"#;
    let ir = compiler()
        .compile_runtime(source, Some("top"))
        .unwrap()
        .canonical_ir;
    ir.validate().unwrap();
    assert_eq!(
        ir.digital
            .arrays
            .iter()
            .find(|array| array.name == "bank")
            .unwrap()
            .storage
            .len,
        12
    );
    assert_eq!(ir.digital.drivers.len(), 4);
    assert!(ir.digital.bit_aliases.is_empty());
    assert_eq!(ir.digital.real_aliases.len(), 4);
}


#[test]
fn conservative_arrays_preserve_custom_probes_branches_and_specialized_replay() {
    let source = r#"
nature Effort; units="V"; access=U; abstol=1e-6; endnature
nature Rate; units="A"; access=J; abstol=1e-12; endnature
discipline custom; potential Effort; flow Rate; enddiscipline
module top(p);
 parameter integer BASE=3;
 output p; electrical p;
 custom [2:1] grid[BASE:BASE+1][-2:-1];
 ground custom [2:1] reference[BASE:BASE+1][-2:-1];
 branch(grid[BASE][-1],reference[BASE][-1]) leg[4:5];
 analog begin
  J(leg[4])<+U(leg[4])/1000;
  J(leg[5])<+U(leg[5])/2000;
  V(p)<+U(grid[BASE][-1][2],reference[BASE][-1][2]);
 end
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    artifact.canonical_ir.validate().unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact.canonical_ir,
            &[("BASE", -5.0)],
            &NoPipelineControl,
        )
        .unwrap();
    specialized.canonical_ir.validate().unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        specialized.canonical_ir.runtime_source_identity(),
        replay.canonical_ir.runtime_source_identity()
    );
    for (bad, expected) in [
        (
            source.replace("U(grid[BASE][-1][2]", "U(grid[BASE][-1][3]"),
            "in-range constant coordinate",
        ),
        (
            source.replace("U(grid[BASE][-1][2]", "U(grid[BASE][-1]"),
            "every dimension",
        ),
        (
            source.replace("branch(grid[BASE][-1]", "branch(grid[BASE][-1][1:2]"),
            "declared bounds or direction",
        ),
        (
            source.replace(
                " ground custom",
                " custom [2:1] grid[BASE:BASE+2][-2:-1];\n ground custom",
            ),
            "inconsistent declarations",
        ),
    ] {
        let error = match compiler.compile_runtime(&bad, Some("top")) {
            Ok(_) => panic!("accepted {expected}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "{expected}: {error}");
    }
}

#[test]
fn conservative_array_lanes_and_ranged_grounds_keep_distinct_source_identities() {
    let source = r#"
module sink(inout electrical [8:9] p); endmodule
module scalar(inout electrical p); endmodule
module top(p);
 output p; electrical p;
 electrical [3:2] cells[-2:-1][4:5], vector;
 electrical scalar_cells[1:0], \cells[-2][4][3] ;
 ground [3:2] references;
 ground electrical array_ground[1:0];
 sink bus(cells[-1][5][3:2]);
 sink joined({cells[-2][4][3],cells[-1][4][2]});
 scalar one(scalar_cells[1]);
 analog begin
  I(cells[-2][4][3])<+V(cells[-2][4][3]);
  I(\cells[-2][4][3] )<+V(\cells[-2][4][3] );
  V(p)<+V(scalar_cells[0],array_ground[1])+V(references[2]);
 end
endmodule
"#;
    let tokens = Lexer::new(source, SourceMap::new().add_source("arrays.vams", source))
        .collect_tokens()
        .unwrap();
    let ast = Parser::new(&tokens).parse().unwrap();
    let analyzed = SemanticAnalyzer::new().analyze(&ast).unwrap();
    let module = &analyzed.modules["top"];
    let names: std::collections::HashSet<_> = module
        .internal_nodes
        .iter()
        .map(|node| node.name.as_str())
        .collect();
    assert!(names.contains("cells[-2][4][3]"));
    assert!(names.contains("cells[-2][4][3]_"));
    assert!(names.contains("cells[-1][5][2]"));
    assert!(names.contains("scalar_cells[0]"));
    assert!(names.contains("vector[3]"));
    for name in [
        "references[3]",
        "references[2]",
        "array_ground[1]",
        "array_ground[0]",
    ] {
        assert!(
            module.ground_nodes.iter().any(|node| node == name),
            "missing {name}"
        );
    }
    compiler()
        .compile_runtime(source, Some("top"))
        .unwrap()
        .canonical_ir
        .validate()
        .unwrap();
}


#[test]
fn real_net_types_propagate_per_occurrence_and_replay_after_specialization() {
    let source = r#"
module source(output wreal value);
 parameter real LEVEL=2.75;
 assign value=LEVEL;
endmodule
module bridge(output wire value);
 parameter real LEVEL=2.75;
 source #(.LEVEL(LEVEL)) nested(value);
endmodule
module tap(inout tri value,output electrical p);
 analog V(p)<+value;
endmodule
module top(p,q);
 parameter real LEVEL=2.75;
 output p,q; electrical p,q;
 tri real_path,logic_path;
 bridge #(.LEVEL(LEVEL)) producer(real_path);
 assign logic_path=1'b1;
 tap real_use(real_path,p);
 tap logic_use(logic_path,q);
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let check = |artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact| {
        artifact.validate().unwrap();
        let signals = &artifact.digital.signals;
        let real = signals
            .iter()
            .find(|signal| signal.name == "real_path")
            .unwrap();
        let logic = signals
            .iter()
            .find(|signal| signal.name == "logic_path")
            .unwrap();
        assert!(matches!(
            real.kind,
            rspice_veriloga::canonical_ir::digital::DigitalSignalKind::Real(_)
        ));
        assert!(!matches!(
            logic.kind,
            rspice_veriloga::canonical_ir::digital::DigitalSignalKind::Real(_)
        ));
        assert_eq!(
            artifact
                .digital
                .drivers
                .iter()
                .filter(|driver| driver.id.signal == real.id)
                .count(),
            1
        );
    };
    check(&artifact.canonical_ir);
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact.canonical_ir,
            &[("LEVEL", 4.25)],
            &NoPipelineControl,
        )
        .unwrap();
    check(&specialized.canonical_ir);
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    check(&replay.canonical_ir);
    assert_eq!(
        specialized.canonical_ir.runtime_source_identity(),
        replay.canonical_ir.runtime_source_identity()
    );
    for (extra, expected) in [
        ("assign real_path=1.0;", "2 drivers"),
        ("initial real_path=1;", "procedural assignment"),
    ] {
        let invalid = source.replace(
            "assign logic_path=1'b1;",
            &format!("{extra} assign logic_path=1'b1;"),
        );
        let error = match compiler.compile_runtime(&invalid, Some("top")) {
            Ok(_) => panic!("accepted {extra}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn whole_wire_buses_acquire_real_types_without_changing_local_coordinates() {
    let source = r#"
module source(output logic wrealsum [2:3] value);
 assign value[2]=1.25; assign value[3]=2.5;
endmodule
module sink(inout tri [5:4] value,output electrical p);
 analog V(p)<+value[5]+value[4];
endmodule
module top(p,q);
 output p,q; electrical p,q;
 wire [8:9] bus;
 source producer(bus);
 sink consumer(bus,p);
 sink selected(bus[8:9],q);
endmodule
"#;
    // A complete vector join resolves the source declaration before parts and
    // local body selections are lowered as real bus coordinates.
    let artifact = compiler().compile_runtime(source, Some("top")).unwrap();
    artifact.canonical_ir.validate().unwrap();
    assert!(
        artifact
            .canonical_ir
            .digital
            .arrays
            .iter()
            .any(|array| array.name == "bus")
    );
    let bad = source.replace("module sink(inout tri", "module sink(inout wrealavg");
    assert!(compiler().compile_runtime(&bad, Some("top")).is_err());
}


#[test]
fn concatenated_interconnect_types_preserve_separate_real_driver_groups() {
    let source = r#"
module leaf(inout logic wrealsum [1:0] value);
 assign value[1]=2.5; assign value[0]=4.75;
endmodule
module monitor(input tri [5:4] value,output electrical p);
 analog V(p)<+10*value[5]+value[4];
endmodule
module top(p,q);
 parameter integer COPIES=2;
 output p,q; electrical p,q;
 wire a,b;
 assign a=0.25; assign b=0.5;
 leaf drive({a,{b}});
 monitor readback({a,b},p);
 monitor repeated({COPIES{a}},q);
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let check = |artifact: &rspice_veriloga::canonical_ir::CanonicalIrArtifact| {
        artifact.validate().unwrap();
        let plan = &artifact.digital;
        let representatives = plan.real_net_representatives().unwrap();
        let a = plan
            .signals
            .iter()
            .find(|signal| signal.name == "a")
            .unwrap();
        let b = plan
            .signals
            .iter()
            .find(|signal| signal.name == "b")
            .unwrap();
        assert!(matches!(
            a.kind,
            rspice_veriloga::canonical_ir::digital::DigitalSignalKind::Real(_)
        ));
        assert!(matches!(
            b.kind,
            rspice_veriloga::canonical_ir::digital::DigitalSignalKind::Real(_)
        ));
        let a = representatives[usize::from(a.id)];
        let b = representatives[usize::from(b.id)];
        assert_ne!(a, b);
        for representative in [a, b] {
            assert_eq!(
                plan.drivers
                    .iter()
                    .filter(
                        |driver| representatives[usize::from(driver.id.signal)] == representative
                    )
                    .count(),
                2
            );
        }
    };
    check(&artifact.canonical_ir);
    let replay = compiler
        .prepare_artifact_runtime_source(&artifact.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    check(&replay.canonical_ir);
    for (bad, expected) in [
        (
            source.replace("wire a,b;", "wrealsum a; wrealavg b;"),
            "same real resolution",
        ),
        (
            source.replace("leaf drive({a,{b}});", "leaf drive({2{a}});"),
            "replicated concatenations",
        ),
    ] {
        let error = match compiler.compile_runtime(&bad, Some("top")) {
            Ok(_) => panic!("accepted {expected}"),
            Err(error) => error.to_string(),
        };
        assert!(error.contains(expected), "{expected}: {error}");
    }
    let oversized = source.replace(
        "parameter integer COPIES=2;",
        "parameter integer COPIES=65537;",
    );
    assert!(compiler.compile_runtime(&oversized, Some("top")).is_err());
}

#[test]
fn real_array_cell_concatenations_resolve_wire_formals_through_nested_views() {
    let source = r#"
module word(inout tri [4:3] x,output electrical p);
 analog V(p)<+x[4]+2*x[3];
endmodule
module top(p,q);
 output p,q; electrical p,q;
 wrealsum cells[-2:-1];
 wrealsum [5:4] bus;
 assign cells[-2]=1.25; assign cells[-1]=2.5;
 assign bus[5]=3.75; assign bus[4]=4.25;
 word first({cells[-1],cells[-2]},p);
 word second({bus[4],bus[5]},q);
endmodule
"#;
    let artifact = compiler().compile_runtime(source, Some("top")).unwrap();
    artifact.canonical_ir.validate().unwrap();
    for name in ["first.x", "second.x"] {
        assert!(
            artifact
                .canonical_ir
                .digital
                .arrays
                .iter()
                .any(|array| array.name == name),
            "missing {name}"
        );
    }
}

const COMPATIBLE_DISCIPLINES: &str = r#"
nature LocalVoltage; units="V"; access=U; abstol=1e-8; endnature
nature LocalCurrent; units="A"; access=J; abstol=1e-14; endnature
nature DerivedVoltage : Voltage; abstol=1e-9; endnature
discipline local_electrical; potential LocalVoltage; flow LocalCurrent; enddiscipline
discipline derived_electrical; potential DerivedVoltage; flow Current; enddiscipline
discipline cool; domain discrete; potential Voltage; enddiscipline
discipline warm; domain discrete; potential LocalVoltage; enddiscipline
discipline thermal_data; domain discrete; potential Temperature; enddiscipline
"#;

#[test]
fn compatible_hierarchy_preserves_local_branch_disciplines_and_replay() {
    let source = format!(
        r#"{COMPATIBLE_DISCIPLINES}
module leaf(p,n);
 inout p,n; local_electrical p,n;
 analog J(p,n)<+U(p,n)/1000;
endmodule
module middle(inout derived_electrical p);
 leaf child(p,0);
endmodule
module top(inout electrical p);
 middle nested(p);
endmodule
"#
    );
    let compiler = compiler();
    let compiled = compiler.compile_runtime(&source, Some("top")).unwrap();
    let local: Vec<_> = compiled
        .canonical_ir
        .hir
        .branches
        .iter()
        .filter(|branch| branch.discipline == "local_electrical")
        .collect();
    assert_eq!(local.len(), 1, "child branch lost its authored discipline");
    // Unnamed branches use canonical endpoint order; the contribution carries its sign.
    assert_eq!(local[0].pos_node, "0");
    assert_eq!(local[0].neg_node, "p");
    let replay = compiler
        .prepare_artifact_runtime_source(&compiled.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        compiled.canonical_ir.hir.branches,
        replay.canonical_ir.hir.branches
    );
    for bad in [
        source
            .replace(
                "potential LocalVoltage; flow LocalCurrent",
                "potential Temperature; flow LocalCurrent",
            )
            .replace("J(p,n)<+U(p,n)/1000", "J(p,n)<+Temp(p,n)/1000"),
        source
            .replace(
                "potential LocalVoltage; flow LocalCurrent",
                "potential LocalVoltage; flow Power",
            )
            .replace("J(p,n)", "Pwr(p,n)"),
    ] {
        let error = compiler
            .compile_runtime(&bad, Some("top"))
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("nested.child") && error.contains("incompatible disciplines"),
            "{error}"
        );
    }
}

#[test]
fn discrete_port_compatibility_checks_authored_net_operands_before_preparation() {
    let compiler = compiler();
    for (formal, declaration, actual) in [
        ("input cool wire a", "wire ACTUAL bus;", "bus"),
        ("output cool wire a", "wire ACTUAL bus;", "bus"),
        ("inout cool wire a", "wire ACTUAL [3:2] bus;", "bus[2]"),
        ("input cool wire a", "wire ACTUAL [3:2] bus;", "bus[2]"),
        (
            "input cool wire [1:0] a",
            "wire ACTUAL [3:2] bus;",
            "bus[3:2]",
        ),
        (
            "input cool wire [1:0] a",
            "wire ACTUAL [3:2] bus;",
            "{bus[3],bus[2]}",
        ),
        ("input cool wire [1:0] a", "wire ACTUAL bus;", "{2{bus}}"),
        ("input cool wire a", "wire ACTUAL cells[3:2];", "cells[2]"),
        ("input cool wreal a", "wreal ACTUAL bus;", "bus"),
        ("input cool wreal a", "wreal ACTUAL cells[3:2];", "cells[2]"),
        ("input cool wreal [1:0] a", "wreal ACTUAL [3:2] bus;", "bus"),
    ] {
        for (discipline, accepted) in [("warm", true), ("thermal_data", false)] {
            let declaration = declaration.replace("ACTUAL", discipline);
            let source = format!(
                "{COMPATIBLE_DISCIPLINES}\nmodule sink({formal}); endmodule\nmodule top; {declaration} sink nested({actual}); endmodule\n"
            );
            let outcome = compiler.compile_runtime(&source, Some("top"));
            if accepted {
                outcome.unwrap_or_else(|error| panic!("{declaration} / {actual}: {error}"));
            } else {
                let error = outcome
                    .err()
                    .unwrap_or_else(|| panic!("accepted {declaration} / {actual}"))
                    .to_string();
                assert!(
                    error.contains("instance 'nested'")
                        && error.contains("incompatible disciplines"),
                    "{declaration} / {actual}: {error}"
                );
            }
        }
    }
    // A computed value is an assignment. Its read operands are not port-connected nets.
    let computed = format!(
        r#"{COMPATIBLE_DISCIPLINES}
module sink(input cool wire [1:0] a); endmodule
module top;
 wire thermal_data [1:0] bus;
 sink nested({{bus[1]+1'b0,bus[0]+1'b0}});
endmodule
"#
    );
    compiler.compile_runtime(&computed, Some("top")).unwrap();
}

#[test]
fn explicit_connections_obey_exclusions_without_converter_rules() {
    for (parent, child, declaration) in [
        ("electrical", "local_electrical", "inout"),
        ("cool", "warm", "inout wire"),
    ] {
        let source = format!(
            r#"{COMPATIBLE_DISCIPLINES}
module leaf(p); {declaration} p; {child} p; endmodule
module top(p); {declaration} p; {parent} p; leaf nested(p); endmodule
connectrules supply_limits; connect {parent}, {child} resolveto exclude; endconnectrules
"#
        );
        let error = compiler()
            .compile_runtime(&source, Some("top"))
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("nested") && error.contains("resolveto exclude"),
            "{error}"
        );
        // The standalone planner must make the same decision for explicitly typed segments.
        use rspice_veriloga::connect::{
            NetSegment, PortLink, ResolutionMode, Signal, plan_connect_modules, resolve_disciplines,
        };
        let tokens = Lexer::new(
            &source,
            SourceMap::new().add_source("exclusion.vams", &source),
        )
        .collect_tokens()
        .unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let analyzed = SemanticAnalyzer::new().analyze(&ast).unwrap();
        let mut signal = Signal::default();
        let lower = signal.push(NetSegment::new("lower").declared(child));
        signal.push(
            NetSegment::new("upper")
                .declared(parent)
                .with_child(PortLink::new(
                    lower,
                    rspice_veriloga::ast::PortDirection::Inout,
                    "nested",
                    "p",
                )),
        );
        let resolved = resolve_disciplines(
            &signal,
            &analyzed.connect_rules,
            &analyzed.disciplines,
            None,
            ResolutionMode::Basic,
        )
        .unwrap();
        let error = plan_connect_modules(
            &signal,
            &resolved,
            &analyzed.connect_rules,
            &analyzed.disciplines,
        )
        .unwrap_err();
        assert!(error.to_string().contains("resolveto exclude"), "{error}");
    }
}

const INHERITED_DISCIPLINES: &str = r#"
discipline low; domain discrete; potential Voltage; enddiscipline
discipline high; domain discrete; potential Temperature; enddiscipline
`default_discipline low
module low_source(output wreal value);
 assign value=2.5;
endmodule
`default_discipline high
module high_source(output wreal value);
 assign value=3.5;
endmodule
module selected(output wire value);
 parameter integer MODE=0;
 generate if (MODE==0) begin : low_arm
  low_source nested(value);
 end else begin : high_arm
  high_source nested(value);
 end endgenerate
endmodule
module load(input electrical a, output electrical p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module top(output electrical p,q);
 parameter integer MODE=0;
 wire x,y;
 selected #(.MODE(MODE)) first(x);
 selected #(.MODE(1-MODE)) second(y);
 load left(x,p),right(y,q);
endmodule
connectmodule low_gain(input low wreal value, output electrical a);
 analog I(a)<+(V(a)-2*value)/1000;
endmodule
connectmodule high_gain(input high wreal value, output electrical a);
 analog I(a)<+(V(a)-3*value)/1000;
endmodule
connectrules chosen; connect low_gain; connect high_gain; endconnectrules
"#;

#[test]
fn inherited_disciplines_follow_generated_occurrences_defaults_and_replay() {
    let compiler = compiler();
    let artifact = compiler
        .compile_runtime(INHERITED_DISCIPLINES, Some("top"))
        .unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("MODE", 1.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        let rebuilt = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            rebuilt.canonical_ir.runtime_source_identity()
        );
    }
    // A local declaration wins over inherited resolution and exposes the conflict.
    let explicit = INHERITED_DISCIPLINES.replace("wire x,y;", "wire high x; wire y;");
    let error = compiler
        .compile_runtime(&explicit, Some("top"))
        .err()
        .unwrap()
        .to_string();
    assert!(
        error.contains("incompatible disciplines") && error.contains("first"),
        "{error}"
    );
}

#[test]
fn inherited_resolution_rules_keep_exclusions_unknowns_and_warnings() {
    let source = r#"
discipline first; domain discrete; potential Voltage; enddiscipline
discipline second; domain discrete; potential Voltage; enddiscipline
module one(output first wreal value); endmodule
module two(output second wreal value); endmodule
module load(input electrical a, output electrical p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module top(output electrical p);
 wreal net;
 one left(net); two right(net); load receiver(net,p);
endmodule
connectmodule converter(input first wreal value, output electrical a);
 analog I(a)<+(V(a)-value)/1000;
endmodule
connectrules chosen;
 connect first,second resolveto first;
 connect first,second resolveto second;
 connect converter;
endconnectrules
"#;
    let compiler = compiler();
    let report = compiler.compile_runtime(source, Some("top")).unwrap();
    let warnings: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "VA-SEM-DISCIPLINE-RESOLUTION")
        .collect();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("net 'net'") && warnings[0].message.contains("first"));
    let replay = compiler
        .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(replay.diagnostics.len(), report.diagnostics.len());
    for (original, replayed) in report.diagnostics.iter().zip(&replay.diagnostics) {
        assert_eq!(original.code, replayed.code);
        assert_eq!(original.message, replayed.message);
        let span = original.span.as_ref().unwrap();
        assert_eq!(
            replayed.byte_start.map(|value| value as u64),
            Some(u64::from(span.byte_start))
        );
        assert_eq!(
            replayed.line.map(|value| value as u64),
            span.start.as_ref().map(|position| position.line as u64)
        );
    }
    let explicit = source.replace("wreal net;", "wreal first net;");
    let explicit = compiler.compile_runtime(&explicit, Some("top")).unwrap();
    assert!(
        explicit
            .diagnostics
            .iter()
            .all(|warning| warning.code != "VA-SEM-DISCIPLINE-RESOLUTION")
    );
    let single_rule = source.replace(" connect first,second resolveto second;", "");
    for (bad, expected) in [
        (
            single_rule.replace("resolveto first", "resolveto exclude"),
            "resolveto exclude",
        ),
        (
            single_rule.replace(" connect first,second resolveto first;", ""),
            "no resolved discipline",
        ),
        (
            single_rule.replace(
                "discipline second; domain discrete; potential Voltage",
                "discipline second; domain discrete; potential Temperature",
            ),
            "incompatible disciplines",
        ),
    ] {
        let error = compiler
            .compile_runtime(&bad, Some("top"))
            .err()
            .unwrap()
            .to_string();
        assert!(error.contains(expected), "{expected}: {error}");
    }
}

#[test]
fn structural_wires_resolve_to_physical_shapes_without_a_digital_runtime() {
    let source = r#"
module source(output electrical a);
 analog I(a)<+(V(a)-2.5)/1000;
endmodule
module pair(output wire [1:0] a);
 source first(a[1]),second(a[0]);
endmodule
module bank(output wire [3:2] a);
 parameter integer UNUSED=2;
 pair nested(a);
endmodule
module load(input electrical [0:1] a,output electrical p);
 analog begin I(a[0])<+V(a[0])/1000; I(a[1])<+V(a[1])/1000; V(p)<+V(a[0])+V(a[1]); end
endmodule
module top(output electrical p,q);
 parameter integer BASE=-2;
 wire [5:4] bus;
 tri words[BASE:BASE+1];
 bank nested(bus);
 source first(words[BASE]),second(words[BASE+1]);
 load packed_load(bus,p),array_load({words[BASE],words[BASE+1]},q);
endmodule
"#;
    // Pure physical connectivity must also compile with AMS execution disabled.
    let compiler = VerilogACompiler::default();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("BASE", 7.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        assert!(report.canonical_ir.digital.signals.is_empty());
        assert!(report.canonical_ir.digital.processes.is_empty());
        assert!(report.canonical_ir.digital.drivers.is_empty());
        let replay = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            replay.canonical_ir.runtime_source_identity()
        );
    }
}

#[test]
fn behavioral_wire_uses_remain_discrete_but_lexical_shadows_do_not() {
    let template = r#"
`timescale 1ns/1ps
module source(output electrical a);
 analog I(a)<+(V(a)-2.5)/1000;
endmodule
module receiver(input logic d); endmodule
module top(output electrical p);
 wire x;
 source producer(x);
 BODY
endmodule
connectmodule sense(input electrical a,output logic d);
 reg d;
 initial d=0;
 always #0.1 d=V(a)>1;
endmodule
connectrules chosen; connect sense; endconnectrules
"#;
    for (body, discrete) in [
        ("reg sample; initial sample=x;", true),
        ("initial begin : local_init reg sample=x; end", true),
        ("reg [1:0] bits; initial bits[x]=1;", true),
        ("wire [1:0] words; receiver observer(words[x]);", true),
        ("reg sample; always @(x) sample=1;", true),
        ("wire copy; assign copy=x;", true),
        ("wire copy=x;", true),
        ("assign x=1'b1;", true),
        ("analog V(p)<+x;", true),
        ("receiver observer(x+1'b0);", true),
        (
            "initial begin : local_scope reg x; reg sample; x=1; sample=x; end",
            false,
        ),
        ("receiver observer(x);", false),
    ] {
        let source = template.replace("BODY", body);
        let report = compiler()
            .compile_runtime(&source, Some("top"))
            .unwrap_or_else(|error| panic!("{body}: {error}"));
        let has_digital_x = report
            .canonical_ir
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "x");
        assert_eq!(has_digital_x, discrete, "{body}");
    }
    let explicit = template
        .replace("wire x;", "wire logic x;")
        .replace("BODY", "");
    let report = compiler().compile_runtime(&explicit, Some("top")).unwrap();
    assert!(
        report
            .canonical_ir
            .digital
            .signals
            .iter()
            .any(|signal| signal.name == "x")
    );
}

#[test]
fn implicit_physical_nets_in_generated_scopes_specialize_and_replay() {
    let source = r#"
module source(output electrical a);
 parameter real LEVEL=2.5;
 analog I(a)<+(V(a)-LEVEL)/1000;
endmodule
module load(input electrical a,output electrical p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module top(output electrical [BASE:BASE+1] p);
 parameter integer BASE=-2;
 genvar i;
 generate for(i=BASE;i<BASE+2;i=i+1) begin
   source #(.LEVEL(i+7.5)) producer(link);
   load consumer(link,p[i]);
 end endgenerate
endmodule
"#;
    let compiler = VerilogACompiler::default();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("BASE", 4.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        assert!(report.canonical_ir.digital.signals.is_empty());
        let replay = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            replay.canonical_ir.runtime_source_identity()
        );
    }
}

#[test]
fn implicit_net_defaults_follow_source_position_macros_and_reset() {
    let source = r#"
module leaf(input wire a); endmodule
`define NET_POLICY none
`default_nettype `NET_POLICY
`ifdef INACTIVE
`default_nettype wire
`endif
module explicit(input wire a); leaf instance(a); endmodule
`resetall
module top;
 leaf first(created);
`default_nettype none
 leaf second(created);
endmodule
"#;
    let compiler = compiler();
    let report = compiler.compile_runtime(source, Some("top")).unwrap();
    assert_eq!(
        report
            .canonical_ir
            .digital
            .signals
            .iter()
            .filter(|signal| signal.name == "created")
            .count(),
        1
    );
    let replay = compiler
        .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        report.canonical_ir.runtime_source_identity(),
        replay.canonical_ir.runtime_source_identity()
    );
    for (bad, name) in [
        (
            source.replace(
                "leaf second(created);",
                "leaf second(missing);\n`default_nettype wire",
            ),
            "missing",
        ),
        (
            source.replace("input wire a); leaf instance", "input a); leaf instance"),
            "a",
        ),
    ] {
        let error = compiler
            .compile_runtime(&bad, Some("top"))
            .err()
            .unwrap()
            .to_string();
        assert!(
            error.contains("default_nettype none") && error.contains(name),
            "{error}"
        );
    }
    // A behavioral read alone does not declare a net, and implicit physical
    // connectivity does not supply declared natures to analog behavioral code.
    for bad in [
        "module top; reg q; initial q=missing; endmodule",
        "module top;\n`UNKNOWN discard_this\nendmodule",
        "module leaf(input electrical a); endmodule module top(output electrical p); leaf l(x); analog V(p)<+V(x); endmodule",
    ] {
        assert!(compiler.compile_runtime(bad, Some("top")).is_err(), "{bad}");
    }
}


#[test]
fn foreign_physical_shapes_share_storage_and_replay() {
    let source = r#"
module leaf(output electrical [3:2] p);
 parameter integer BASE=-2;
 electrical [BASE:BASE+1] nodes[4:5];
 branch(nodes[4][BASE],nodes[5][BASE+1]) sense;
 branch(p) legs[7:6];
 analog begin
   V(nodes[4][BASE])<+1; V(nodes[4][BASE+1])<+2;
   V(nodes[5][BASE])<+3; V(nodes[5][BASE+1])<+4;
   V(legs[7])<+5; V(legs[6])<+6;
 end
endmodule
module top(output electrical [3:2] p,output electrical q);
 parameter integer BASE=-2;
 leaf #(.BASE(BASE)) a(p);
 analog V(q)<+V(a.nodes[4][BASE])+V(a.sense)+I(a.legs[7]);
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("BASE", 7.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        assert_eq!(report.canonical_ir.hir.ports.len(), 3);
        let replay = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            replay.canonical_ir.runtime_source_identity()
        );
        assert_eq!(
            report.canonical_ir.hir.branches,
            replay.canonical_ir.hir.branches
        );
    }
}

#[test]
fn foreign_physical_references_reject_variables_and_bad_coordinates() {
    for expression in ["V(a.x)", "V(a.nodes[3])", "I(<a.nodes[0]>)"] {
        let source = format!(
            "module leaf; real x; electrical [1:0] nodes; analog begin V(nodes[0])<+1; V(nodes[1])<+2; end endmodule module top(output electrical p); leaf a(); analog V(p)<+{expression}; endmodule"
        );
        assert!(
            compiler().compile_runtime(&source, Some("top")).is_err(),
            "{expression}"
        );
    }
}


#[test]
fn foreign_explicit_branch_selectors_specialize_in_caller_scope_and_replay() {
    let source = r#"
module leaf(inout electrical [2:1] p);
 parameter integer IDX=999;
 analog begin I(p[2])<+(V(p[2])-4)/1000; I(p[1])<+(V(p[1])-2)/1000; end
endmodule
module top(output electrical [2:1] p,output electrical q);
 parameter integer IDX=1;
 genvar j;
 generate for(j=0;j<2;j=j+1) begin : cells
   leaf a(p);
 end endgenerate
 analog V(q)<+V(cells[1].a.branch(p[IDX]))+I(cells[0].a.branch(0,p[2]))+I(cells[1].a.branch(<p[IDX]>));
endmodule
"#;
    let compiler = VerilogACompiler::default();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("IDX", 2.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        assert_eq!(report.canonical_ir.hir.ports.len(), 3);
        let replay = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            replay.canonical_ir.runtime_source_identity()
        );
        assert_eq!(
            report.canonical_ir.hir.branches,
            replay.canonical_ir.hir.branches
        );
    }
}

#[test]
fn foreign_explicit_branches_reject_missing_or_incompatible_targets() {
    for (declarations, body, usage, diagnostic) in [
        (
            "branch(p) b;",
            "I(b)<+V(b)/1000;",
            "V(q)<+I(a.branch(p));",
            "no existing unnamed branch",
        ),
        (
            "branch(p) b;",
            "I(b)<+V(b)/1000;",
            "V(q)<+I(a.branch(b));",
            "terminals must be nets",
        ),
        (
            "electrical n;",
            "I(p)<+V(p)/1000;",
            "V(q)<+I(a.branch(<n>));",
            "must name a port",
        ),
        (
            "",
            "I(p)<+V(p)/1000;",
            "V(a.branch(p))<+1;",
            "switch branch",
        ),
        (
            "branch(p) b;",
            "I(b)<+V(b)/1000;",
            "V(a.b)<+1;",
            "switch branch",
        ),
        (
            "",
            "V(p):V(p)==1;",
            "I(a.branch(p))<+1m;",
            "indirectly constrained",
        ),
        (
            "",
            "I(p)<+V(p)/1000;",
            "V(a.branch(p)):V(p)==1;",
            "hierarchical indirect",
        ),
    ] {
        let source = format!(
            "module leaf(inout electrical p); {declarations} analog {body} endmodule module top(inout electrical p,output electrical q); leaf a(p); analog begin {usage} end endmodule"
        );
        let error = VerilogACompiler::default()
            .compile_runtime(&source, Some("top"))
            .err()
            .expect("invalid target must fail")
            .to_string();
        assert!(error.contains(diagnostic), "{usage}: {error}");
    }
}


#[test]
fn foreign_explicit_branch_terminals_reject_nested_branch_calls() {
    let expression = format!("{}p{}", "a.branch(".repeat(64), ")".repeat(64));
    let source = format!(
        "module leaf(inout electrical p); analog I(p)<+V(p)/1000; endmodule module top(output electrical p,q); leaf a(p); analog V(q)<+I({expression}); endmodule"
    );
    assert!(
        VerilogACompiler::default()
            .compile_runtime(&source, Some("top"))
            .is_err()
    );
}

#[test]
fn foreign_upward_scopes_preserve_shadowing_generate_context_and_replay() {
    let source = r#"
module leaf(output electrical p);
 analog V(p)<+top.K+$root.top.BASE+peer.K;
endmodule
module constants;
 parameter integer K=99;
endmodule
module wrapper(output electrical p);
 parameter integer K=3;
 constants #(.K(K)) top();
 generate begin : group
   constants #(.K(K+1)) peer();
   leaf l(p);
 end endgenerate
endmodule
module top(output electrical p,q);
 parameter integer BASE=2;
 wrapper #(.K(BASE)) a(p);
 wrapper b(q);
endmodule
module unused;
 analog begin $display($root.other.VALUE); end
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("top")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("BASE", 7.0)], &NoPipelineControl)
        .unwrap();
    for report in [&artifact, &specialized] {
        report.canonical_ir.validate().unwrap();
        let replay = compiler
            .prepare_artifact_runtime_source(&report.canonical_ir, &NoPipelineControl)
            .unwrap()
            .compile_runtime(None)
            .unwrap();
        assert_eq!(
            report.canonical_ir.runtime_source_identity(),
            replay.canonical_ir.runtime_source_identity()
        );
        assert_eq!(
            report.canonical_ir.hir.branches,
            replay.canonical_ir.hir.branches
        );
    }
    // Both executable frontends receive the selected design, including bytecode.
    compiler.compile_module(source, Some("top")).unwrap();
}

#[test]
fn foreign_upward_resolution_rejects_shadowed_paths_wrong_roots_and_cycles() {
    for (source, diagnostic) in [
        (
            "module leaf(output electrical p); analog V(p)<+$root.other.P; endmodule module top(output electrical p); leaf a(p); endmodule",
            "selected top-level",
        ),
        (
            "module leaf(output electrical p); parameter real top=2; analog V(p)<+top.P; endmodule module top(output electrical p); parameter P=1; leaf a(p); endmodule",
            "not a module",
        ),
        (
            "module leaf(output electrical p); parameter P=top.P; analog V(p)<+P; endmodule module top(output electrical p); parameter P=1; leaf a(p); endmodule",
            "parameter declaration cannot reference outside",
        ),
        (
            "module leaf; parameter P=1; endmodule module top; leaf #(.P(b.P)) a(); leaf #(.P(a.P)) b(); endmodule",
            "cyclic parameter",
        ),
        (
            "module leaf; leaf a(); analog $display($root.top.P); endmodule module top; parameter P=1; leaf a(); endmodule",
            "recursive module hierarchy",
        ),
    ] {
        let error = compiler()
            .compile_runtime(source, Some("top"))
            .err()
            .expect("invalid selected hierarchy")
            .to_string();
        assert!(
            error.contains(diagnostic),
            "expected {diagnostic}: {error} for {source}"
        );
    }
}

#[test]
fn foreign_root_overrides_do_not_replace_recursive_instance_defaults() {
    let source = r#"
module tree(inout electrical p);
 parameter integer N=0;
 generate if(N>0) begin : nested
   tree a(p);
 end else begin : terminal
   analog I(p)<+V(p)/1000;
 end endgenerate
 analog $display($root.tree.N);
endmodule
"#;
    let compiler = compiler();
    let artifact = compiler.compile_runtime(source, Some("tree")).unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&artifact.canonical_ir, &[("N", 1.0)], &NoPipelineControl)
        .unwrap();
    specialized.canonical_ir.validate().unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        specialized.canonical_ir.hir.branches,
        replay.canonical_ir.hir.branches
    );
}

#[test]
fn foreign_inactive_generate_references_bind_after_virtual_module_selection() {
    use rspice_veriloga::{VirtualCompileLimits, VirtualSourceBundle, VirtualSourceFile};
    let source = r#"
module leaf(output electrical p);
 parameter integer ENABLE=0;
 generate if(ENABLE) begin : active
   analog V(p)<+$root.top.G;
 end else begin : inactive
   analog V(p)<+0;
 end endgenerate
endmodule
module top(output electrical p);
 parameter real G=3;
 leaf #(.ENABLE(1)) a(p);
endmodule
"#;
    let bundle =
        VirtualSourceBundle::new("top.vams", [VirtualSourceFile::new("top.vams", source)]).unwrap();
    let compiler = compiler();
    let prepared = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap();
    assert_eq!(prepared.module_names().collect::<Vec<_>>(), ["leaf", "top"]);
    assert!(prepared.connect_specification().declares_module);
    let report = prepared.compile_runtime("top").unwrap();
    report.runtime.canonical_ir.validate().unwrap();
    let direct = compiler.compile_runtime(source, Some("top")).unwrap();
    assert_eq!(
        direct.canonical_ir.hir.branches,
        report.runtime.canonical_ir.hir.branches
    );
    let library = VirtualSourceBundle::from_sources(
        "rules.vams",
        [(
            "rules.vams",
            r#"
connectmodule drive(input logic d,output electrical a);
 analog V(a)<+d;
endmodule
connectrules selected; connect drive; endconnectrules
"#,
        )],
    )
    .unwrap();
    let library = compiler
        .prepare_virtual_runtime_source(&library, VirtualCompileLimits::default())
        .unwrap();
    let configuration = library.connection_configuration("selected").unwrap();
    let configured = prepared
        .compile_runtime_with_connections("top", &configuration, &NoPipelineControl)
        .unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(
            &configured.runtime.canonical_ir,
            &[("G", 6.0)],
            &NoPipelineControl,
        )
        .unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        specialized.canonical_ir.runtime_source_identity(),
        replay.canonical_ir.runtime_source_identity()
    );
}

#[test]
fn foreign_selected_connect_module_can_reference_its_root() {
    let source = r#"
discipline logic; domain discrete; enddiscipline
connectmodule drive(input logic d,output electrical a);
 parameter real G=2;
 analog V(a)<+($root.drive.G+d);
endmodule
"#;
    let artifact = compiler().compile_runtime(source, Some("drive")).unwrap();
    artifact.canonical_ir.validate().unwrap();
}

#[test]
fn foreign_inserted_connect_helpers_survive_virtual_configuration_and_replay() {
    use rspice_veriloga::{VirtualCompileLimits, VirtualSourceBundle};
    let source = r#"
module source(output logic q); assign q=1; endmodule
module group(inout electrical p);
 parameter real G=2;
 electrical vdd;
 analog V(vdd)<+$root.top.BASE+G;
 source first(p);
endmodule
module top(inout electrical p,q);
 parameter real BASE=1;
 group a(p);
 group #(.G(5)) b(q);
endmodule
"#;
    let library = r#"
module stage(input logic d,output electrical a);
 analog I(a)<+(V(a)-d*(V(group.vdd)+drive.GAIN))/1000;
endmodule
connectmodule drive(input logic d,output electrical a);
 parameter integer N=1;
 parameter real GAIN=1;
 generate if(N==2) begin : enabled
   stage helper(d,a);
 end else begin : disabled
   analog I(a)<+V(a)/1000;
 end endgenerate
endmodule
connectrules selected; connect drive #(.N(2),.GAIN(3)); endconnectrules
"#;
    let compiler = compiler();
    let device =
        VirtualSourceBundle::from_sources("device.vams", [("device.vams", source)]).unwrap();
    let device = compiler
        .prepare_virtual_runtime_source(&device, VirtualCompileLimits::default())
        .unwrap();
    let bundle =
        VirtualSourceBundle::from_sources("rules.vams", [("rules.vams", library)]).unwrap();
    let library = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap();
    let configuration = library.connection_configuration("selected").unwrap();
    let artifact = device
        .compile_runtime_with_connections("top", &configuration, &NoPipelineControl)
        .unwrap();
    artifact.runtime.canonical_ir.validate().unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(
            &artifact.runtime.canonical_ir,
            &[("BASE", 4.0)],
            &NoPipelineControl,
        )
        .unwrap();
    let replay = compiler
        .prepare_artifact_runtime_source(&specialized.canonical_ir, &NoPipelineControl)
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        specialized.canonical_ir.hir.branches,
        replay.canonical_ir.hir.branches
    );
    assert_eq!(
        specialized.canonical_ir.runtime_source_identity(),
        replay.canonical_ir.runtime_source_identity()
    );
    let invalid = source
        .replace("module group(", "module renamed(")
        .replace("group a(", "renamed a(")
        .replace("group #(", "renamed #(");
    let invalid =
        VirtualSourceBundle::from_sources("device.vams", [("device.vams", invalid)]).unwrap();
    let invalid = compiler
        .prepare_virtual_runtime_source(&invalid, VirtualCompileLimits::default())
        .unwrap();
    let error = invalid
        .compile_runtime_with_connections("top", &configuration, &NoPipelineControl)
        .err()
        .expect("missing insertion ancestor must be diagnosed")
        .to_string();
    assert!(
        error.contains("group") && error.contains("scope"),
        "{error}"
    );
}

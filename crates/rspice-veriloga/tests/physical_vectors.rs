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

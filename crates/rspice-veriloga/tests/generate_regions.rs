//! Generate regions, IEEE 1364-2005 section 12.4.
//!
//! A generate region contributes no run-time behaviour of its own: it
//! contributes module items, decided once, from constants. So every test here
//! asserts on the *elaborated* result — the signals, drivers, and processes the
//! region produced — rather than on a syntax tree. A pass that parsed a region
//! faithfully and unrolled it wrongly would satisfy a tree assertion and fail
//! every one of these.
//!
//! The identity claims are the ones worth reading. Section 12.4.2 names an
//! iteration's items by the generate block and the genvar's value, so eight
//! iterations of one instance are eight instances a scheduler can resume
//! individually and a resolver can tell apart — not one instance elaborated
//! eight times.

use rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

/// Compile one digital module and return its elaborated plan.
fn plan(source: &str) -> CanonicalDigitalPlan {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(source, Some("top"))
        .unwrap_or_else(|error| panic!("fixture must compile: {error}"))
        .digital
}

fn compile_error(source: &str) -> String {
    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(source, Some("top"))
        .expect_err("fixture must be refused")
        .to_string()
}

/// A one-input, one-output child, and a `top` that varies.
///
/// `top` writes the section 12.3.4 two-declaration form for its own ports —
/// `output [3:0] y; wire [3:0] y;` — rather than leaving them implicit. That is
/// not decoration: a module whose *only* discrete-domain content is instances
/// is not currently recognized as a digital module at all, because whether an
/// instance is digital is a question about the instantiated module and the
/// analyzer looks at one module at a time. Every design in this file therefore
/// declares its own nets, as the corpus's own structural designs do.
fn design(top_body: &str) -> String {
    format!(
        "module leaf(y, a);\n\
         \x20   output y;\n\
         \x20   input a;\n\
         \x20   wire y, a;\n\
         \x20   assign y = ~a;\n\
         endmodule\n\
         module top(a, y);\n\
         \x20   input [3:0] a;\n\
         \x20   output [3:0] y;\n\
         \x20   wire [3:0] a, y;\n\
         {top_body}\n\
         endmodule\n"
    )
}

fn signal_names(plan: &CanonicalDigitalPlan) -> Vec<String> {
    plan.signals
        .iter()
        .map(|signal| signal.name.to_string())
        .collect()
}

// ===========================================================================
// Unrolling and identity
// ===========================================================================

/// A generate loop over a genvar with constant bounds produces one copy of its
/// block per iteration, and section 12.4.2 names each copy by the block and the
/// index.
///
/// The names are the assertion. `gen[0].u` and `gen[3].u` are different
/// instances of one module, so their ports are different signals; a pass that
/// unrolled the *items* but shared the *names* would produce four instances
/// writing one net.
#[test]
fn a_generate_loop_names_each_iteration_by_its_index() {
    let plan = plan(&design(
        "    genvar i;\n\
     \x20   generate\n\
     \x20     for (i = 0; i < 4; i = i + 1) begin : gen\n\
     \x20       leaf u (y[i], a[i]);\n\
     \x20     end\n\
     \x20   endgenerate",
    ));

    let names = signal_names(&plan);
    for index in 0..4 {
        for port in ["y", "a"] {
            let expected = format!("gen[{index}].u.{port}");
            assert!(
                names.contains(&expected),
                "no elaborated signal `{expected}`; got {names:?}"
            );
        }
    }

    // Four instances, so four drivers on `y` — one per bit — and four
    // processes for the four `assign y = ~a;` bodies.
    let y = plan
        .signals
        .iter()
        .find(|signal| signal.name == "y")
        .expect("`y` is declared");
    let indices: Vec<u32> = plan
        .drivers_of(y.id)
        .map(|driver| driver.id.index)
        .collect();
    assert_eq!(indices, vec![0, 1, 2, 3], "one driver per generated bit");

    // Every process is a separate identity. Two iterations sharing one would be
    // one thing a scheduler resumes for both.
    let mut identities: Vec<_> = plan.processes.iter().map(|process| process.id).collect();
    identities.sort();
    let count = identities.len();
    identities.dedup();
    assert_eq!(identities.len(), count, "two processes share an identity");
}

/// The genvar is substituted, not merely bound: `a[i]` in iteration 3 reads
/// bit 3, and `carry[i + 1]` reads bit 4.
///
/// Checked through the drivers each iteration produces, because a substitution
/// that got the arithmetic wrong would still produce four drivers and four
/// signals with the right names.
#[test]
fn the_genvar_is_substituted_into_indices_and_arithmetic() {
    let plan = plan(&design(
        "    wire [4:0] carry;\n\
     \x20   genvar i;\n\
     \x20   generate\n\
     \x20     for (i = 0; i < 4; i = i + 1) begin : gen\n\
     \x20       assign carry[i + 1] = a[i];\n\
     \x20     end\n\
     \x20   endgenerate\n\
     \x20   assign y = carry[4:1];",
    ));

    let carry = plan
        .signals
        .iter()
        .find(|signal| signal.name == "carry")
        .expect("`carry` is declared");
    let mut bits: Vec<i64> = plan
        .drivers_of(carry.id)
        .map(|driver| match driver.target.select {
            rspice_veriloga::canonical_ir::digital::DigitalWriteSelect::Bit(position) => position,
            ref other => panic!("expected a bit select, got {other:?}"),
        })
        .collect();
    bits.sort_unstable();
    assert_eq!(
        bits,
        vec![1, 2, 3, 4],
        "`carry[i + 1]` must fold to bits 1 through 4, not to `i + 1` unresolved"
    );
}

/// The loop bound may name the module's own parameter, which section 12.2
/// fixes at elaboration.
#[test]
fn a_generate_loop_bound_may_be_a_parameter() {
    let plan = plan(&design(
        "    parameter WIDTH = 3;\n\
     \x20   genvar i;\n\
     \x20   generate\n\
     \x20     for (i = 0; i < WIDTH; i = i + 1) begin : gen\n\
     \x20       leaf u (y[i], a[i]);\n\
     \x20     end\n\
     \x20   endgenerate\n\
     \x20   assign y[3] = 1'b0;",
    ));

    let names = signal_names(&plan);
    assert!(names.contains(&"gen[2].u.y".to_string()));
    assert!(
        !names.contains(&"gen[3].u.y".to_string()),
        "the loop ran past its parameter bound"
    );
}

/// A generate-if with a constant condition keeps one arm and drops the other
/// entirely — the dropped arm contributes no signal, no driver, and no process.
#[test]
fn a_generate_if_keeps_exactly_one_arm() {
    let source = |condition: &str| {
        design(&format!(
            "    genvar i;\n\
         \x20   generate\n\
         \x20     if ({condition}) begin : taken\n\
         \x20       leaf u (y[0], a[0]);\n\
         \x20     end else begin : untaken\n\
         \x20       leaf u (y[1], a[1]);\n\
         \x20     end\n\
         \x20   endgenerate\n\
         \x20   assign y[3:2] = a[3:2];"
        ))
    };

    let names = signal_names(&plan(&source("1")));
    assert!(names.contains(&"taken.u.y".to_string()));
    assert!(!names.iter().any(|name| name.starts_with("untaken.")));

    let names = signal_names(&plan(&source("0")));
    assert!(names.contains(&"untaken.u.y".to_string()));
    assert!(!names.iter().any(|name| name.starts_with("taken.")));
}

/// A generate-case selects the first arm whose label equals the selector, and
/// falls to `default` when none does.
#[test]
fn a_generate_case_selects_one_arm_or_the_default() {
    let source = |width: &str| {
        design(&format!(
            "    parameter WIDTH = {width};\n\
         \x20   generate\n\
         \x20     case (WIDTH)\n\
         \x20       1: begin : one   leaf u (y[0], a[0]); end\n\
         \x20       2: begin : two   leaf u (y[1], a[1]); end\n\
         \x20       default: begin : other leaf u (y[2], a[2]); end\n\
         \x20     endcase\n\
         \x20   endgenerate\n\
         \x20   assign y[3] = a[3];"
        ))
    };

    for (width, block) in [("1", "one"), ("2", "two"), ("9", "other")] {
        let names = signal_names(&plan(&source(width)));
        assert!(
            names.contains(&format!("{block}.u.y")),
            "WIDTH={width} must select `{block}`; got {names:?}"
        );
        for other in ["one", "two", "other"] {
            if other != block {
                assert!(
                    !names
                        .iter()
                        .any(|name| name.starts_with(&format!("{other}."))),
                    "WIDTH={width} also produced `{other}`"
                );
            }
        }
    }
}

/// Nested loops unroll to the product of their bounds, and each iteration's
/// name carries both indices — which is what keeps `outer[1].inner[0]` and
/// `outer[0].inner[1]` apart.
#[test]
fn nested_generate_loops_carry_both_indices() {
    let plan = plan(&design(
        "    genvar i, j;\n\
     \x20   generate\n\
     \x20     for (i = 0; i < 2; i = i + 1) begin : outer\n\
     \x20       for (j = 0; j < 2; j = j + 1) begin : inner\n\
     \x20         assign y[i * 2 + j] = a[i * 2 + j];\n\
     \x20       end\n\
     \x20     end\n\
     \x20   endgenerate",
    ));

    let y = plan
        .signals
        .iter()
        .find(|signal| signal.name == "y")
        .expect("`y` is declared");
    let mut bits: Vec<i64> = plan
        .drivers_of(y.id)
        .map(|driver| match driver.target.select {
            rspice_veriloga::canonical_ir::digital::DigitalWriteSelect::Bit(position) => position,
            ref other => panic!("expected a bit select, got {other:?}"),
        })
        .collect();
    bits.sort_unstable();
    assert_eq!(
        bits,
        vec![0, 1, 2, 3],
        "each (i, j) pair drives its own bit"
    );
}

/// A process inside a generate loop is copied once per iteration, and each copy
/// gets its own identity: two copies of one `always` block are two things a
/// scheduler resumes, not one resumed twice.
#[test]
fn a_process_inside_a_loop_is_copied_with_a_fresh_identity() {
    let plan = plan(&design(
        "    reg [3:0] q;\n\
     \x20   genvar i;\n\
     \x20   generate\n\
     \x20     for (i = 0; i < 4; i = i + 1) begin : gen\n\
     \x20       always @(a) q[i] = ~a[i];\n\
     \x20     end\n\
     \x20   endgenerate\n\
     \x20   assign y = q;",
    ));

    let processes: Vec<_> = plan
        .processes
        .iter()
        .filter(|process| {
            process.kind == rspice_veriloga::canonical_ir::digital::DigitalProcessKind::Always
        })
        .collect();
    assert_eq!(processes.len(), 4, "one `always` per iteration");
    let mut identities: Vec<_> = processes.iter().map(|process| process.id).collect();
    identities.sort();
    identities.dedup();
    assert_eq!(identities.len(), 4, "two iterations share a process id");
    for (index, process) in plan.processes.iter().enumerate() {
        assert_eq!(
            usize::from(process.id),
            index,
            "generated processes have dense IDs"
        );
        assert_eq!(plan.process(process.id), Some(process));
    }
}

/// Regenerating is deterministic: the same source produces the same plan, so a
/// loop's iteration order and the identities it hands out are a function of the
/// source and not of a hash iteration order.
#[test]
fn unrolling_is_deterministic() {
    let source = design(
        "    genvar i;\n\
     \x20   generate\n\
     \x20     for (i = 0; i < 4; i = i + 1) begin : gen\n\
     \x20       leaf u (y[i], a[i]);\n\
     \x20     end\n\
     \x20   endgenerate",
    );
    assert_eq!(plan(&source), plan(&source));
}

// ===========================================================================
// What is refused, with its clause
// ===========================================================================

/// Every generate construct outside the subset refuses by name, and names the
/// clause that governs it. Nothing is dropped, and nothing is guessed at.
#[test]
fn unelaborated_generate_constructs_refuse_by_name() {
    let cases: Vec<(&str, String, Vec<&str>)> = vec![
        (
            "a non-constant loop bound",
            design(
                "    genvar i;\n\
             \x20   generate\n\
             \x20     for (i = 0; i < a; i = i + 1) begin : gen\n\
             \x20       leaf u (y[i], a[i]);\n\
             \x20     end\n\
             \x20   endgenerate",
            ),
            vec!["generate for condition", "12.4"],
        ),
        (
            "a non-constant conditional",
            design(
                "    generate\n\
             \x20     if (a) begin : gen\n\
             \x20       leaf u (y[0], a[0]);\n\
             \x20     end\n\
             \x20   endgenerate\n\
             \x20   assign y[3:1] = a[3:1];",
            ),
            vec!["generate if condition", "12.4"],
        ),
        (
            "a loop over an undeclared genvar",
            design(
                "    generate\n\
             \x20     for (i = 0; i < 4; i = i + 1) begin : gen\n\
             \x20       leaf u (y[i], a[i]);\n\
             \x20     end\n\
             \x20   endgenerate",
            ),
            vec!["not declared `genvar`", "12.1.3.2"],
        ),
        (
            "a header that advances the wrong name",
            design(
                "    genvar i, k;\n\
             \x20   generate\n\
             \x20     for (i = 0; i < 4; k = k + 1) begin : gen\n\
             \x20       leaf u (y[i], a[i]);\n\
             \x20     end\n\
             \x20   endgenerate",
            ),
            vec!["an update of `k`", "12.4.1"],
        ),
        (
            "a nested loop reusing one genvar",
            design(
                "    genvar i;\n\
             \x20   generate\n\
             \x20     for (i = 0; i < 2; i = i + 1) begin : outer\n\
             \x20       for (i = 0; i < 2; i = i + 1) begin : inner\n\
             \x20         leaf u (y[i], a[i]);\n\
             \x20       end\n\
             \x20     end\n\
             \x20   endgenerate",
            ),
            vec!["reusing the genvar `i`"],
        ),
        (
            "a reopened region inside a region",
            design(
                "    generate\n\
             \x20     generate\n\
             \x20     endgenerate\n\
             \x20   endgenerate\n\
             \x20   assign y = a;",
            ),
            vec!["without reopening a region", "12.4"],
        ),
    ];

    for (label, source, fragments) in cases {
        let message = compile_error(&source);
        for fragment in fragments {
            assert!(
                message.contains(fragment),
                "refusing {label} must mention {fragment:?}, got {message:?}"
            );
        }
    }
}

/// A `genvar` declaration and an empty region are legal and contribute nothing.
///
/// Worth pinning because both used to be refused by name, and "contributes
/// nothing" is a different claim from "is ignored": the module still has to
/// produce the same signals, the same drivers, and the same process identities
/// it would have produced without them. Only the source spans differ, because
/// the text they point into does.
#[test]
fn an_empty_region_and_a_bare_genvar_contribute_nothing() {
    let shape = |plan: &CanonicalDigitalPlan| {
        (
            signal_names(plan),
            plan.drivers
                .iter()
                .map(|driver| (driver.id, driver.target.clone(), driver.process))
                .collect::<Vec<_>>(),
            plan.processes
                .iter()
                .map(|process| (process.id, process.kind))
                .collect::<Vec<_>>(),
        )
    };

    let plain = plan(&design("    assign y = ~a;"));
    let decorated = plan(&design(
        "    genvar i, j;\n\
     \x20   generate\n\
     \x20   endgenerate\n\
     \x20   assign y = ~a;",
    ));
    assert_eq!(shape(&plain), shape(&decorated));
}

#[test]
fn scoped_generated_nets_keep_default_names_and_process_shadows() {
    let source = r#"
module leaf(input wire a,output wire y); assign y=a; endmodule
module top(input wire [3:0] a,output wire [3:0] y);
 parameter integer genblk1=0;
 genvar i;
 generate for(i=0;i<4;i=i+1) begin
   wire t=a[i];
   leaf first(t,implicit_link);
   leaf second(implicit_link,y[i]);
   initial begin : locals
     reg t; integer i;
     t=0; i=3;
   end
 end endgenerate
endmodule
"#;
    let plan = plan(source);
    for index in 0..4 {
        for name in ["t", "implicit_link"] {
            let name = format!("genblk01[{index}].{name}");
            assert!(
                plan.signals.iter().any(|signal| signal.name == name),
                "{name}"
            );
        }
    }
    assert_eq!(plan.processes.len(), 20); // Also includes each selected output-port binding.
}

#[test]
fn plain_generate_region_items_keep_the_surrounding_scope() {
    let plan = plan(
        r#"
module top(output wire result);
 generate
   assign middle=1'b1;
   assign result=middle;
 endgenerate
endmodule
"#,
    );
    assert_eq!(plan.signals.len(), 2);
    assert!(plan.signals.iter().any(|signal| signal.name == "middle"));
    assert!(plan.signals.iter().any(|signal| signal.name == "result"));
}


#[test]
fn generated_storage_and_local_constants_preserve_each_scope() {
    let source = r#"
module top;
 parameter integer BASE=-1;
 localparam integer WIDTH=12;
 genvar i;
 generate for(i=BASE;i<BASE+2;i=i+1) begin : channel
   localparam integer WIDTH=i-BASE+2;
   reg [WIDTH-1:0] state[WIDTH:WIDTH];
   integer count=WIDTH;
   real level=WIDTH+0.25;
   initial begin
     reg [WIDTH-1:0] scratch;
     scratch=WIDTH;
     state[WIDTH]=scratch;
     #1 count=count+1;
     level=level+count;
   end
   if(WIDTH==2) begin : small
     reg enabled=1;
   end else begin : large
     reg enabled=0;
   end
   if(1) begin : nested
     genvar j;
     localparam integer SIZE=WIDTH-1;
     for(j=0;j<SIZE;j=j+1) begin : bits
       reg bit_value=1;
     end
   end
 end endgenerate
endmodule
"#;
    let compiled = plan(source);
    for (index, width) in [(-1, 2), (0, 3)] {
        let name = format!("channel[{index}].state[{width}]");
        let signal = compiled
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap();
        assert_eq!(signal.width, width);
        for variable in ["count", "level"] {
            assert!(
                compiled
                    .signals
                    .iter()
                    .any(|signal| signal.name == format!("channel[{index}].{variable}"))
            );
        }
    }
    let names = signal_names(&compiled);
    assert!(names.iter().any(|name| name == "channel[-1].small.enabled"));
    assert!(names.iter().any(|name| name == "channel[0].large.enabled"));
    assert_eq!(
        names
            .iter()
            .filter(|name| name.ends_with(".bit_value"))
            .count(),
        3
    );
}

#[test]
fn generated_analog_functions_branches_and_local_shadowing_compile() {
    let source = r#"
module top(output electrical [1:0] p);
 parameter real SCALE=1.0;
 genvar i;
 generate for(i=0;i<2;i=i+1) begin : channel
   localparam real LEVEL=i+2.5;
   localparam integer N=2;
   electrical n;
   branch(n) drive;
   real offset;
   analog function real shifted;
     input i; real i;
     real temporary;
     begin temporary=i+LEVEL; shifted=temporary; end
   endfunction
   analog initial offset=0.25;
   analog begin : behavior
     real LEVEL;
     LEVEL=shifted(0.5);
     I(drive)<+(V(drive)-SCALE*LEVEL-offset)/1000;
     I(n)<+V(n)/1000+white_noise(1e-18,"thermal");
     V(p[i])<+V(n);
   end
 end endgenerate
endmodule
"#;
    let compiler = VerilogACompiler::default();
    let compiled = compiler.compile_runtime(source, Some("top")).unwrap();
    assert!(compiled.canonical_ir.digital.signals.is_empty());
    compiled.canonical_ir.validate().unwrap();
    let noise_names: Vec<_> = compiled
        .model
        .noise_sources
        .iter()
        .map(|source| source.name.as_deref().unwrap())
        .collect();
    assert_eq!(noise_names.len(), 2);
    assert!(
        noise_names
            .iter()
            .any(|name| name.contains("channel[0].thermal"))
    );
    assert!(
        noise_names
            .iter()
            .any(|name| name.contains("channel[1].thermal"))
    );
    let rebuilt = compiler
        .prepare_artifact_runtime_source(
            &compiled.canonical_ir,
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap()
        .compile_runtime(None)
        .unwrap();
    assert_eq!(
        compiled.canonical_ir.runtime_source_identity(),
        rebuilt.canonical_ir.runtime_source_identity()
    );
}

#[test]
fn generated_declarations_reject_invalid_interface_and_cyclic_schemes() {
    for (body, expected) in [
        (
            "if(0) begin : bad parameter integer N=2; end",
            "parameter declaration",
        ),
        (
            "if(1) begin : bad parameter integer N=2; end",
            "parameter declaration",
        ),
        ("if(1) begin : bad input p; end", "port declaration"),
        (
            "for(i=0;i<2;i=1-i) begin : bad wire a; end",
            "repeats genvar value",
        ),
        (
            "if(1) begin : bad localparam integer N=M; localparam integer M=N; if(N) wire a; end",
            "cannot fold to a constant",
        ),
    ] {
        let message = compile_error(&format!(
            "module top; genvar i; generate {body} endgenerate endmodule"
        ));
        assert!(message.contains(expected), "{body}: {message}");
    }
}

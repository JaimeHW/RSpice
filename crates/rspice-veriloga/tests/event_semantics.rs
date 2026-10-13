use rspice_veriloga::device::VerilogADevice;

mod support;

use support::DeviceFixture;

fn evaluate(device: &mut VerilogADevice, time: f64, voltage: f64) -> f64 {
    device.set_analysis_type(2);
    device.set_time(time);
    device.set_timestep(time.max(0.0));
    device.update_voltages(&[voltage]);
    device.try_evaluate().expect("event evaluation succeeds")[0]
}

#[test]
fn cross_is_newton_idempotent_and_does_not_treat_tolerance_as_a_crossing() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module tolerant_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 1, 0.0, 0.1, 1.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    assert_eq!(
        evaluate(&mut device, 1.0, -0.05).to_bits(),
        0.0_f64.to_bits(),
        "entering the expression tolerance on the same side is not a crossing"
    );
    assert_eq!(
        evaluate(&mut device, 1.0, -0.05).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 2.0, 1.0).to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        evaluate(&mut device, 2.0, 1.0).to_bits(),
        1.0_f64.to_bits(),
        "repeated Newton evaluations must preserve the crossing event"
    );
}

#[test]
fn cross_enable_disables_events_without_freezing_history() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module disabled_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 0, 0.0, 0.0, 0.0);
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(
        evaluate(&mut device, 0.0, -1.0).to_bits(),
        0.0_f64.to_bits()
    );
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn above_fires_initially_and_only_on_subsequent_rising_events() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module stateful_above(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ above(V(p, n) - 1.5, 0.0, 0.0, 1.0);
endmodule
"#,
    );

    let mut initially_positive = model.device("A1", &[1, 0]);
    assert_eq!(
        evaluate(&mut initially_positive, 0.0, 2.0).to_bits(),
        1.0_f64.to_bits(),
        "above must trigger during initialization when already positive"
    );

    let mut crossing = model.device("A2", &[1, 0]);
    assert_eq!(
        evaluate(&mut crossing, 0.0, 1.0).to_bits(),
        0.0_f64.to_bits()
    );
    crossing.advance_state();
    assert_eq!(
        evaluate(&mut crossing, 1.0, 2.0).to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        evaluate(&mut crossing, 1.0, 2.0).to_bits(),
        1.0_f64.to_bits()
    );
    crossing.advance_state();
    assert_eq!(
        evaluate(&mut crossing, 2.0, 2.5).to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn above_fires_on_same_time_dc_sweep_crossings() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module dc_sweep_above(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ above(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(0);
    device.set_time(0.0);

    device.update_voltages(&[-1.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
    device.advance_state();

    device.update_voltages(&[1.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    device.advance_state();

    device.update_voltages(&[2.0]);
    assert_eq!(device.try_evaluate().unwrap()[0], 0.0);
}

#[test]
fn full_event_control_argument_lists_compile() {
    DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module event_argument_lists(p, n);
    inout p, n;
    electrical p, n;
    real enabled, state;
    analog begin
        enabled = 1.0;
        @(cross(V(p, n), 1, 1.0e-12, 1.0e-6, enabled)) state = 1.0;
        @(above(V(p, n), 1.0e-12, 1.0e-6, enabled)) state = 2.0;
        @(cross(V(p, n), 1, , , enabled)) state = 3.0;
        @(above(V(p, n), , , enabled)) state = 4.0;
        I(p, n) <+ state;
    end
endmodule
"#,
    );
}

#[test]
fn null_event_optional_slots_use_runtime_defaults() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module null_event_arguments(p, n);
    inout p, n;
    electrical p, n;
    real enabled;
    analog begin
        enabled = 1.0;
        I(p, n) <+ cross(V(p, n), 1, , , enabled)
                   + above(V(p, n), , , enabled);
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 1.0, 1.0),
        2.0,
        "null tolerances must select defaults without disabling either rising event"
    );
}

#[test]
fn cross_event_direction_is_evaluated_at_runtime() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module dynamic_cross_direction(p, n);
    inout p, n;
    electrical p, n;
    real direction, count;
    analog begin
        direction = V(p, n) >= 0.0 ? 1.0 : -1.0;
        @(cross(V(p, n), direction)) count = count + 1.0;
        I(p, n) <+ count;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();
    assert_eq!(evaluate(&mut device, 1.0, 1.0), 1.0);
    device.advance_state();
    assert_eq!(
        evaluate(&mut device, 2.0, -1.0),
        2.0,
        "a falling crossing must fire after the runtime direction changes to -1"
    );
}

#[test]
fn event_controlled_variables_are_newton_transactional_and_resume_exactly() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module transactional_event_counter(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(cross(V(p, n), +1)) count = count + 1.0;
        I(p, n) <+ count;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);

    assert_eq!(evaluate(&mut device, 0.0, -1.0), 0.0);
    device.advance_state();

    assert_eq!(evaluate(&mut device, 1.0, 1.0), 1.0);
    assert_eq!(
        evaluate(&mut device, 1.0, 1.0),
        1.0,
        "a repeated Newton pass must replay from accepted procedural state"
    );
    assert_eq!(
        evaluate(&mut device, 1.0, -0.5),
        0.0,
        "a retreating Newton pass must discard the rejected crossing update"
    );
    device.advance_state();

    assert_eq!(evaluate(&mut device, 2.0, 1.0), 1.0);
    device.advance_state();
    let checkpoint = device.checkpoint_state().unwrap();

    let mut restored = model.device("A1", &[1, 0]);
    restored.validate_checkpoint_state(&checkpoint).unwrap();
    restored.apply_validated_checkpoint_state(&checkpoint);
    assert_eq!(evaluate(&mut restored, 3.0, -1.0), 1.0);
    restored.advance_state();
    assert_eq!(evaluate(&mut restored, 4.0, 1.0), 2.0);
    assert_eq!(evaluate(&mut restored, 4.0, 1.0), 2.0);
}

#[test]
fn checkpoint_before_acceptance_excludes_step_event_variable_candidates() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module transactional_initial_step(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("tran")) count = count + 1.0;
        I(p, n) <+ count;
    end
endmodule
"#,
    );
    let count_index = model
        .variable_names
        .iter()
        .position(|name| name == "count")
        .expect("counter variable is present");
    let mut device = model.device("A1", &[1, 0]);
    device.set_analysis_type(2);
    device.set_analysis_step(true, false);
    device.update_voltages(&[0.0]);

    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    assert_eq!(device.try_evaluate().unwrap()[0], 1.0);
    assert_eq!(
        device.checkpoint_state().unwrap().accepted.variables[count_index],
        0.0,
        "a pre-accept checkpoint must retain the prior accepted counter"
    );

    device.advance_state();
    assert_eq!(
        device.checkpoint_state().unwrap().accepted.variables[count_index],
        1.0
    );
}

#[test]
fn step_events_follow_phase_and_analysis_filters() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module step_events(p, n);
    inout p, n;
    electrical p, n;
    real event_value;
    analog begin
        event_value = 0.0;
        @(initial_step("tran", "vendor_extension")) event_value = event_value + 1.0;
        @(final_step("tran")) event_value = event_value + 2.0;
        I(p, n) <+ event_value;
    end
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[0.0]);

    device.set_analysis_type(2);
    device.set_analysis_step(false, false);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        0.0_f64.to_bits()
    );

    device.set_analysis_step(true, false);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        1.0_f64.to_bits()
    );

    device.set_analysis_step(false, true);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        2.0_f64.to_bits()
    );

    device.set_analysis_step(true, true);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        3.0_f64.to_bits()
    );

    device.set_analysis_type(0);
    assert_eq!(
        device.try_evaluate().unwrap()[0].to_bits(),
        0.0_f64.to_bits()
    );
    assert!(device.final_step_feeds_equations());
}

/// The generated route solves a built-in's stop time without asking whether
/// its `@(final_step)` body feeds the equations, which is exact only while no
/// shipped built-in has such a body. Every shipped source is preprocessed as
/// the generator preprocesses it; a module whose stream still names
/// `final_step` is compiled and its body must feed no contribution.
///
/// HiSIM_HV is the one shipped model whose source names the event: its
/// degradation report runs `$fopen`/`$fdisplay` under `_AGING_`, which no
/// compile profile defines, so its stream holds no event at all.
///
/// Source discovery and the compile profiles live in the Rust code
/// generator, so the scan runs wherever that generator is built.
#[cfg(feature = "rust-codegen")]
#[test]
fn generated_builtin_final_step_bodies_feed_no_equations() {
    use rspice_veriloga::preprocessor::MacroDef;
    use rspice_veriloga::rust_backend::{
        VERILOGA_COMPILE_PROFILE_FILE_NAME, discover_veriloga_sources,
    };
    use rspice_veriloga::{CompilerOptions, Preprocessor, VerilogACompiler};
    use std::collections::BTreeSet;
    use std::path::Path;

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .canonicalize()
        .expect("the model tree is checked in");
    let profile = std::fs::read_to_string(root.join(VERILOGA_COMPILE_PROFILE_FILE_NAME))
        .expect("the model root declares a compile profile");
    let shipped: BTreeSet<(String, String)> = profile
        .lines()
        .filter_map(|line| line.trim().strip_prefix("namespace "))
        .map(|arguments| {
            let (selector, _) = arguments.split_once('=').expect("SOURCE::MODULE=HEX_ID");
            let (source, module) = selector.rsplit_once("::").expect("SOURCE::MODULE");
            (source.to_string(), module.to_string())
        })
        .collect();

    let mut preprocessed_sources = 0usize;
    let mut named = Vec::new();
    for candidate in discover_veriloga_sources(&root).expect("the generator's discovery") {
        let relative = candidate
            .path
            .strip_prefix(&root)
            .expect("discovery stays under its root")
            .to_string_lossy()
            .replace('\\', "/");
        let modules: Vec<&String> = candidate
            .modules
            .iter()
            .filter(|module| shipped.contains(&(relative.clone(), (*module).clone())))
            .collect();
        if modules.is_empty() {
            continue;
        }
        let mut preprocessor = Preprocessor::new();
        preprocessor.add_include_path(&root);
        for name in &candidate.compile_profile.undefines {
            preprocessor.undefine(name);
        }
        for (name, value) in &candidate.compile_profile.defines {
            preprocessor.define(name, MacroDef::simple(value.as_deref().unwrap_or_default()));
        }
        let stream = preprocessor
            .preprocess_file(&candidate.path)
            .expect("a shipped source preprocesses");
        preprocessed_sources += 1;
        if !stream.contains("final_step") {
            continue;
        }
        let mut options = CompilerOptions::default();
        options.include_paths.push(root.clone());
        options.defines = candidate.compile_profile.defines.clone();
        options.undefines = candidate.compile_profile.undefines.clone();
        let compiler = VerilogACompiler::new(options);
        for module in modules {
            let compiled = compiler
                .compile_file_runtime_with_metadata(&candidate.path, Some(module.as_str()))
                .expect("a shipped module compiles");
            let nodes: Vec<usize> = (1..=compiled.model.num_terminals).collect();
            let device = VerilogADevice::try_new_with_canonical_ir(
                module.as_str(),
                compiled.model,
                &compiled.canonical_ir,
                &nodes,
            )
            .expect("a shipped module builds a device");
            assert!(
                !device.final_step_feeds_equations(),
                "{relative}::{module}: a final_step body feeds a contribution, so the generated \
                 route would solve its stop time differently from the .va route"
            );
            named.push(format!("{relative}::{module}"));
        }
    }
    // Every shipped device's source was read, so a missing one fails here
    // rather than passing unexamined.
    let shipped_sources: BTreeSet<&String> = shipped.iter().map(|(source, _)| source).collect();
    assert_eq!(preprocessed_sources, shipped_sources.len());
    let hisim = root
        .join("cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va")
        .join("HSMHV_module.inc");
    assert!(
        std::fs::read_to_string(hisim)
            .expect("HiSIM_HV's module body is checked in")
            .contains("@(final_step(\"tran\"))"),
        "the scan's one known source of the event moved"
    );
    eprintln!(
        "{preprocessed_sources} shipped sources; modules whose stream names final_step: {named:?}"
    );
}

#[test]
fn step_events_reject_non_string_analysis_filters() {
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let error = compiler
        .compile(
            r#"
`include "disciplines.vams"
module invalid_step_event(p, n);
    inout p, n;
    electrical p, n;
    analog @(initial_step("tran", 1.0)) I(p, n) <+ 1.0;
endmodule
"#,
        )
        .expect_err("non-string step-event filter must be rejected");
    assert!(error.to_string().contains("string literals"), "{error}");
}

/// `@(posedge expr)` has no digital solver behind it, so it compiles to the
/// analog `cross` operator — a continuous zero-crossing detector on the value
/// of the operand, which is not edge detection.
///
/// That reading is the conventional Verilog-A one and is kept, but it changes
/// what the source means, so it is said out loud on the compile report rather
/// than applied in silence.
#[test]
fn edge_events_on_continuous_signals_report_their_cross_interpretation() {
    let report = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(
            r#"
`include "disciplines.vams"
module edge_on_continuous(p, n);
    inout p, n;
    electrical p, n;
    real hits;
    analog begin
        hits = 0.0;
        @(posedge V(p, n)) hits = 1.0;
        @(negedge V(p, n)) hits = 2.0;
        I(p, n) <+ hits * V(p, n);
    end
endmodule
"#,
            Some("edge_on_continuous"),
        )
        .expect("an edge event on a continuous signal still compiles");

    let edge_diagnostics: Vec<_> = report
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "VA-SEM-EDGE-EVENT-AS-CROSS")
        .collect();
    assert_eq!(
        edge_diagnostics.len(),
        2,
        "both edge events must be reported: {:?}",
        report.diagnostics
    );
    assert!(edge_diagnostics[0].message.contains("posedge"));
    assert!(edge_diagnostics[0].message.contains("cross"));
    assert!(edge_diagnostics[1].message.contains("negedge"));
    for diagnostic in edge_diagnostics {
        assert!(
            diagnostic.span.is_some(),
            "an edge event is pinned to its source position"
        );
    }
}

/// On a discrete-discipline signal the `cross` reading is simply wrong: a
/// digital net carries no continuous value to cross zero, so the event would
/// never fire.
///
/// The construct is legal — Verilog-AMS LRM 2.4 section 7.3.4's
/// `analog_event_expression` lists `posedge expression` beside the analog event
/// functions, and its worked example declares the operand a `wire`; section
/// 7.3.6.2 runs the guarded statement "at the time corresponding to a real
/// promotion of the digital time". It lowers to a digital edge event the mixed
/// host raises, and is never a `cross`: the continuous-only compiler refuses
/// the module for its discrete content, and the mixed compile carries the edge.
#[test]
fn edge_events_on_discrete_signals_are_digital_edge_events() {
    for (keyword, edge) in [
        (
            "posedge",
            rspice_veriloga::canonical_ir::digital::DigitalEdge::Posedge,
        ),
        (
            "negedge",
            rspice_veriloga::canonical_ir::digital::DigitalEdge::Negedge,
        ),
    ] {
        let source = format!(
            r#"
`include "disciplines.vams"
discipline digital_wire
    domain discrete;
enddiscipline
module edge_on_discrete(p, n);
    inout p, n;
    electrical p, n;
    digital_wire clk;
    real hits;
    analog begin
        hits = 0.0;
        @({keyword} clk) hits = 1.0;
        I(p, n) <+ hits * V(p, n);
    end
endmodule
"#
        );
        assert_digital_edge(&source, "edge_on_discrete", edge);
    }
}

/// The built-in `logic` discipline is discrete too, and its edge is the same
/// digital edge event without any user discipline declaration.
#[test]
fn edge_events_on_the_builtin_logic_discipline_are_digital_edge_events() {
    assert_digital_edge(
        r#"
`include "disciplines.vams"
module edge_on_logic(p, n);
    inout p, n;
    electrical p, n;
    logic clk;
    real hits;
    analog begin
        hits = 0.0;
        @(posedge clk) hits = 1.0;
        I(p, n) <+ hits * V(p, n);
    end
endmodule
"#,
        "edge_on_logic",
        rspice_veriloga::canonical_ir::digital::DigitalEdge::Posedge,
    );
}

/// The continuous-only compiler refuses the module for its discrete content,
/// with no `cross` reading reported; the mixed compile carries one edge on
/// `clk`.
fn assert_digital_edge(
    source: &str,
    module: &str,
    edge: rspice_veriloga::canonical_ir::digital::DigitalEdge,
) {
    let error = rspice_veriloga::VerilogACompiler::default()
        .compile_module(source, Some(module))
        .expect_err("a digital edge event must not compile to a cross detector");
    assert!(
        error.to_string().contains("no executable form"),
        "the continuous-only compiler refuses the discrete content: {error}"
    );
    let runtime = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(source, Some(module))
    .unwrap_or_else(|error| panic!("the mixed compile carries the edge: {error}"));
    assert!(
        runtime
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "VA-SEM-EDGE-EVENT-AS-CROSS"),
        "{:?}",
        runtime.diagnostics
    );
    let edges: Vec<_> = runtime
        .canonical_ir
        .hir
        .discrete_edges
        .iter()
        .map(|discrete| (discrete.signal.as_str(), discrete.edge))
        .collect();
    assert_eq!(edges, [("clk", edge)]);
}

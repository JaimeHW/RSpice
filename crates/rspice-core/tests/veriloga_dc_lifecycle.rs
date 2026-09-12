//! End-to-end DC lifecycle pins for runtime-compiled Verilog-A devices.
#![cfg(feature = "veriloga")]

use rspice_core::engine::DcSweepRange;
use rspice_core::{Engine, Netlist, NoAbort};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn write_model(name: &str, source: &str) -> PathBuf {
    let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rspice_dc_lifecycle_{name}_{}_{sequence}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn node_voltage(result: &rspice_core::solver::SimulationResult, name: &str) -> f64 {
    let index = result
        .node_names
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("node {name} is absent from {:?}", result.node_names));
    result.node_voltages[index]
}

#[test]
fn switch_branches_preserve_dc_ac_noise_and_multiplicity() {
    let model = write_model("switched_source", "module switched(p,q); inout p,q; electrical p,q;
        parameter integer mode=0; real process;
        analog begin
            process=white_noise(1,\"shared\");
            I(q)<+I(p);
            if(mode==0) I(p)<+3*V(p)+ddt(2*V(p))+process;
            else V(p)<+2*I(p)+ddt(4*I(p))+process;
            if(mode==2) begin I(p)<+5*V(p)+2*process; V(p)<+3*I(p)+3*process; V(p)<+4*I(p)+4*process; end
        end endmodule");
    for mode in 0..3 {
        for multiplicity in [1.0, 4.0] {
            let netlist=Netlist::parse_validated(&format!("* switched source\nI1 0 out DC 1 AC 1\nR1 probe 0 1\nX1 out probe switched mode={mode} m={multiplicity}\n.va \"{}\" switched\n.end\n",deck_path(&model))).unwrap();
            let engine = Engine::default();
            let resistance = match mode {
                0 => 1.0 / 3.0,
                1 => 2.0,
                _ => 7.0,
            };
            let dc = engine.run_dc_op(&netlist).unwrap();
            assert!(
                (node_voltage(&dc, "out") - resistance / multiplicity).abs() < 1e-8,
                "mode={mode}, m={multiplicity}: {dc:?}"
            );
            assert!((node_voltage(&dc, "probe") + 1.0).abs() < 1e-8);
            let frequency = 1.0 / std::f64::consts::TAU;
            let ac = engine.run_ac(&netlist, &[frequency]).unwrap();
            let out = ac[0]
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let expected = match mode {
                0 => num_complex::Complex64::new(3.0, 2.0).inv(),
                1 => num_complex::Complex64::new(2.0, 4.0),
                _ => num_complex::Complex64::new(7.0, 0.0),
            } / multiplicity;
            assert!(
                (ac[0].voltages[out] - expected).norm() < 1e-8,
                "mode={mode}, m={multiplicity}: {:?}, expected {expected}",
                ac[0].voltages[out]
            );
            let circuit = engine.build_circuit(&netlist).unwrap();
            let output = circuit.get_node_by_name("out").unwrap();
            let noise = engine
                .run_noise(&netlist, output, &[frequency], 300.15)
                .unwrap();
            let expected = match mode {
                0 => 1.0 / 13.0,
                1 => 1.0,
                _ => 49.0,
            } / multiplicity;
            assert!(
                (noise[0].output_noise_density / expected - 1.0).abs() < 1e-8,
                "mode={mode}, m={multiplicity}: {:?}, expected {expected}",
                noise[0]
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn parallel_potential_branches_preserve_dc_ac_and_independent_noise() {
    for (name, source, resistance, noise) in [
        (
            "named_parallel",
            "module top(p); inout p; electrical p; branch(p) a,b; analog begin V(a)<+2*I(a)+white_noise(4,\"a\"); V(b)<+3*I(b)+white_noise(9,\"b\"); end endmodule",
            1.2,
            2.88,
        ),
        (
            "same_branch",
            "module top(p); inout p; electrical p; branch(p) a; analog begin V(a)<+2*I(a)+white_noise(4,\"a\"); V(a)<+3*I(a)+white_noise(9,\"b\"); end endmodule",
            5.0,
            13.0,
        ),
        (
            "instance_parallel",
            "module resistor(p); inout p; electrical p; parameter real r=2; analog V(p)<+r*I(p)+white_noise(r*r,\"thermal\"); endmodule module top(p); inout p; electrical p; resistor #(.r(2)) a(p); resistor #(.r(3)) b(p); endmodule",
            1.2,
            2.88,
        ),
    ] {
        let model = write_model(name, source);
        let netlist=Netlist::parse_validated(&format!("* parallel potential branches\nI1 0 out DC 1 AC 1\nX1 out top\n.va \"{}\" top module=top\n.end\n",deck_path(&model))).unwrap();
        let engine = Engine::default();
        let dc = engine.run_dc_op(&netlist).unwrap();
        assert!(
            (node_voltage(&dc, "out") - resistance).abs() < 1e-8,
            "{name}: {dc:?}"
        );
        for point in engine.run_ac(&netlist, &[0.0, 1.0, 1e6]).unwrap() {
            let output = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let value = point.voltages[output];
            assert!(
                (value.re - resistance).abs() < 1e-8 && value.im.abs() < 1e-12,
                "{name}: {value:?}"
            );
        }
        let circuit = engine.build_circuit(&netlist).unwrap();
        let output = circuit.get_node_by_name("out").unwrap();
        for point in engine
            .run_noise(&netlist, output, &[1.0, 1e6], 300.15)
            .unwrap()
        {
            assert!(
                (point.output_noise_density / noise - 1.0).abs() < 1e-8,
                "{name}: {point:?}"
            );
        }
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn hierarchy_port_currents_preserve_dc_ac_and_independent_noise() {
    let model = write_model(
        "hierarchical_port_currents",
        "module leaf(p,n); inout p,n; electrical p,n; parameter real gain=1;
         analog I(p,n)<+gain*V(p,n)+white_noise(1,\"shot\"); endmodule
         module child(p,n,q); inout p,n,q; electrical p,n,q; parameter real gain=1;
         leaf #(.gain(gain)) inner(p,n); analog I(q,n)<+3*I(<p>); endmodule
         module top(p,n,q); inout p,n,q; electrical p,n,q;
         child #(.gain(1)) a(p,n,q); child #(.gain(2)) b(p,n,q); endmodule",
    );
    for (options, resistance) in [("", 1.0), (".options rshunt=2\n", 2.0 / 3.0)] {
        let netlist = Netlist::parse_validated(&format!(
            "* hierarchical branch currents\n{options}V1 in 0 DC 1 AC 1\nR1 out 0 1\nX1 in 0 out top\n.va \"{}\" top module=top\n.end\n", deck_path(&model)
        )).unwrap();
        let engine = Engine::default();
        let expected = -9.0 * resistance;
        let dc = engine.run_dc_op(&netlist).unwrap();
        assert!((node_voltage(&dc, "out") - expected).abs() < 1e-8);
        for point in engine.run_ac(&netlist, &[0.0, 1.0, 1e6]).unwrap() {
            let output = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let value = point.voltages[output];
            assert!((value.re - expected).abs() < 1e-8 && value.im.abs() < 1e-12);
        }
        let circuit = engine.build_circuit(&netlist).unwrap();
        let output = circuit.get_node_by_name("out").unwrap();
        let expected_noise = 18.0 * resistance * resistance;
        for point in engine
            .run_noise(&netlist, output, &[1.0, 1e6], 300.15)
            .unwrap()
        {
            assert!(
                (point.output_noise_density / expected_noise - 1.0).abs() < 1e-8,
                "{point:?}"
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn flow_probe_feedback_preserves_dc_ac_noise_and_private_state_equations() {
    for feedback in [0.0, 0.1] {
        let model = write_model(
            "flow_probe_feedback",
            &format!(
                "module flow_probe_feedback(p,n,q); inout p,n,q; electrical p,n,q; analog begin
             I(q,n)<+3*I(p,n);
             I(p,n)<+2*V(p,n)+{feedback}*I(p,n)+white_noise(1,\"shot\");
             end endmodule"
            ),
        );
        for (options, resistance) in [("", 1.0), (".options rshunt=2\n", 2.0 / 3.0)] {
            let netlist = Netlist::parse_validated(&format!(
                "* simultaneous branch currents\n{options}V1 in 0 DC 1 AC 1\nR1 out 0 1\nX1 in 0 out flow_probe_feedback\n.va \"{}\" flow_probe_feedback\n.end\n", deck_path(&model)
            )).unwrap();
            let engine = Engine::default();
            let expected = -6.0 * resistance / (1.0 - feedback);
            let dc = engine.run_dc_op(&netlist).unwrap();
            assert!(
                (node_voltage(&dc, "out") - expected).abs() < 1e-8,
                "feedback={feedback}; {options}"
            );
            for point in engine.run_ac(&netlist, &[0.0, 1.0, 1e6]).unwrap() {
                let output = point
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                let value = point.voltages[output];
                assert!(
                    (value.re - expected).abs() < 1e-8 && value.im.abs() < 1e-12,
                    "{value}"
                );
            }
            let circuit = engine.build_circuit(&netlist).unwrap();
            let output = circuit.get_node_by_name("out").unwrap();
            let expected_noise = (3.0 * resistance / (1.0 - feedback)).powi(2);
            for point in engine
                .run_noise(&netlist, output, &[1.0, 1e6], 300.15)
                .unwrap()
            {
                assert!(
                    (point.output_noise_density / expected_noise - 1.0).abs() < 1e-8,
                    "{point:?}"
                );
            }
        }
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn disk_includes_select_modules_independently_of_model_aliases() {
    let model = write_model(
        "module_selection",
        r#"
module first_load(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule
module second_load(p,n); inout p,n; electrical p,n; analog I(p,n)<+2.0*V(p,n); endmodule
"#,
    );
    for reversed in [false, true, false] {
        let first = format!(
            ".va \"{}\" first_alias module=first_load",
            deck_path(&model)
        );
        let second = format!(
            ".va \"{}\" second_alias module=second_load",
            deck_path(&model)
        );
        let includes = if reversed {
            format!("{second}\n{first}")
        } else {
            format!("{first}\n{second}")
        };
        let deck = format!(
            "* Module selection\n{includes}\nI1 0 a 1\nI2 0 b 1\nX1 a 0 first_alias\nX2 b 0 second_alias\n.op\n.end\n"
        );
        let netlist = Netlist::parse_validated(&deck).unwrap();
        let engine = Engine::default();
        let result = engine.run_dc_op(&netlist).unwrap();
        assert!((node_voltage(&result, "a") - 1.0).abs() < 1e-8);
        assert!((node_voltage(&result, "b") - 0.5).abs() < 1e-8);
    }
    for selector in ["", " module=missing", " module=FIRST_LOAD"] {
        let deck = format!(
            "* Invalid selection must not reuse another module\n.va \"{}\" alias{selector}\nI1 0 a 1\nX1 a 0 alias\n.end\n",
            deck_path(&model)
        );
        let netlist = Netlist::parse_validated(&deck).unwrap();
        let error = Engine::default()
            .run_dc_op(&netlist)
            .expect_err("missing, ambiguous and case-mismatched selectors must fail");
        assert!(error.to_string().contains("Module selection"), "{error}");
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn selected_veriloga_module_needs_no_separate_alias() {
    let model = write_model(
        "no_alias",
        "module Chosen(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule Other(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n",
    );
    let deck = format!(
        "* Module selection without an alias\n.va \"{}\" module=Chosen\nI1 0 a 1\nX1 a 0 chosen\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse_validated(&deck).unwrap();
    assert!(netlist.lint_unknown_references().is_empty());
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    assert!((node_voltage(&result, "a") - 1.0).abs() < 1e-8);
    let _ = std::fs::remove_file(model);
}

#[test]
fn inferred_veriloga_module_name_survives_hierarchy_flattening() {
    let model = write_model(
        "different_filename",
        "module ActualDevice(p,n); inout p,n; electrical p,n; parameter real g=1; analog I(p,n)<+g*V(p,n); endmodule\n",
    );
    for alias in ["", " user_alias"] {
        let deck = format!(
            "* Resolve the compiled module name\n.va \"{}\"{alias}\n.subckt wrapper p n scale=2\nXmodel p n actualdevice g={{scale}}\n.ends\nI1 0 a 1\nX1 a 0 wrapper\n.end\n",
            deck_path(&model)
        );
        let netlist = Netlist::parse_validated(&deck).unwrap();
        assert!(netlist.lint_unknown_references().is_empty());
        let result = Engine::default()
            .run_dc_op(&netlist)
            .expect("the compiled module name is an external device inside a subcircuit");
        assert!((node_voltage(&result, "a") - 0.5).abs() < 1e-8);
        let transient = Engine::default().run_tran(&netlist, 1e-6, 1e-7).unwrap();
        let index = transient
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("a"))
            .unwrap();
        assert!(
            transient.voltages[index]
                .iter()
                .all(|voltage| (voltage - 0.5).abs() < 1e-8)
        );
        rspice_core::execution::topology_fingerprint(&Engine::default(), &netlist)
            .expect("execution planning uses the same external leaf semantics");

        let misspelled =
            Netlist::parse_validated(&deck.replace("actualdevice", "misspelled")).unwrap();
        let error = Engine::default()
            .run_dc_op(&misspelled)
            .expect_err("unresolved leaves must fail at model binding");
        let detail = error.to_string().to_ascii_lowercase();
        assert!(
            detail.contains("x1.xmodel") && detail.contains("misspelled"),
            "{detail}"
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn ambiguous_veriloga_bindings_never_select_the_first_loaded_module() {
    let model = write_model(
        "ambiguous_bindings",
        "module First(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule FIRST(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\nmodule Second(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n",
    );
    let stem = model.file_stem().unwrap().to_str().unwrap();
    for (first, second, binding) in [
        ("Shared module=First", "SHARED module=Second", "shared"),
        ("module=First", "module=FIRST", "First"),
        ("a_model module=First", "b_model module=Second", stem),
    ] {
        for reversed in [false, true] {
            let (first, second) = if reversed {
                (second, first)
            } else {
                (first, second)
            };
            let deck = format!(
                "* Ambiguous bindings must be refused\n.va \"{path}\" {first}\n.va \"{path}\" {second}\nI1 0 a 1\nX1 a 0 {binding}\n.end\n",
                path = deck_path(&model)
            );
            let error = Engine::default()
                .run_dc_op(&Netlist::parse_validated(&deck).unwrap())
                .expect_err("an ambiguous alias must not choose a model by include order");
            assert!(
                error.to_string().contains("ambiguous Verilog-A model"),
                "{error}"
            );
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn authored_veriloga_aliases_take_precedence_over_implicit_names() {
    let model = write_model(
        "alias_precedence",
        "module First(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule Second(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n",
    );
    for (first_alias, second_alias) in [
        ("Second", "First"),
        (model.file_stem().unwrap().to_str().unwrap(), "other_model"),
    ] {
        for reversed in [false, true] {
            let first = format!(".va \"{}\" {first_alias} module=First", deck_path(&model));
            let second = format!(".va \"{}\" {second_alias} module=Second", deck_path(&model));
            let includes = if reversed {
                format!("{second}\n{first}")
            } else {
                format!("{first}\n{second}")
            };
            let deck = format!(
                "* Explicit aliases precede implicit names\n{includes}\nI1 0 a 1\nI2 0 b 1\nX1 a 0 {first_alias}\nX2 b 0 {second_alias}\n.end\n"
            );
            let result = Engine::default()
                .run_dc_op(&Netlist::parse_validated(&deck).unwrap())
                .unwrap();
            assert!((node_voltage(&result, "a") - 1.0).abs() < 1e-8);
            assert!((node_voltage(&result, "b") - 0.5).abs() < 1e-8);
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn identical_veriloga_bindings_allow_default_and_explicit_module_selection() {
    let model = write_model(
        "identical_bindings",
        "module Same(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    );
    for reversed in [false, true] {
        let default = format!(".va \"{}\" shared", deck_path(&model));
        let explicit = format!(".va \"{}\" SHARED module=Same", deck_path(&model));
        let includes = if reversed {
            format!("{explicit}\n{default}")
        } else {
            format!("{default}\n{explicit}")
        };
        let deck = format!(
            "* Identical artifacts may share an alias\n{includes}\nI1 0 a 1\nX1 a 0 shared\n.end\n"
        );
        let result = Engine::default()
            .run_dc_op(&Netlist::parse_validated(&deck).unwrap())
            .unwrap();
        assert!((node_voltage(&result, "a") - 1.0).abs() < 1e-8);
    }
    let _ = std::fs::remove_file(model);
}

#[cfg(feature = "veriloga-model-diode-cmc")]
#[test]
fn authored_veriloga_aliases_take_precedence_over_generated_builtins() {
    let builtin_deck = Netlist::parse_validated(
        "* Built-in fallback remains available\nI1 0 a 1\nX1 a 0 DIODE_CMC\n.end\n",
    )
    .unwrap();
    assert!(
        Engine::default()
            .build_circuit(&builtin_deck)
            .unwrap()
            .has_generated_veriloga_devices()
    );
    let model = write_model(
        "builtin_alias",
        "module Authored(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\n",
    );
    let deck = format!(
        "* Authored source must be executed\n.va \"{}\" DIODE_CMC\nI1 0 a 1\nX1 a 0 DIODE_CMC\n.end\n",
        deck_path(&model)
    );
    let result = Engine::default()
        .run_dc_op(&Netlist::parse_validated(&deck).unwrap())
        .unwrap();
    assert!(
        (node_voltage(&result, "a") - 1.0).abs() < 1e-8,
        "the authored one-siemens model must be used: {}",
        node_voltage(&result, "a")
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn module_selection_reads_connect_rules_once_and_requires_a_device() {
    let mut source = String::new();
    for (_, module) in rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES {
        source.push_str(module);
    }
    source.push_str("connectrules deck; connect a2d; connect d2a; endconnectrules\n");
    let connect_only = write_model("selected_connect_library", &source);
    let deck = |path: &std::path::Path, selector: &str| {
        format!(
            "* Connect library selection\n.va \"{}\"{selector}\nV1 a 0 1\nR1 a 0 1k\n.end\n",
            deck_path(path)
        )
    };
    Engine::default()
        .run_dc_op(&Netlist::parse_validated(&deck(&connect_only, "")).unwrap())
        .expect("a connect-only library needs no device selection");
    let error = Engine::default()
        .run_dc_op(&Netlist::parse_validated(&deck(&connect_only, " module=missing")).unwrap())
        .expect_err("an explicit device selection must not be silently ignored");
    assert!(error.to_string().contains("no device module"), "{error}");

    source.push_str("module one(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule two(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n");
    let devices = write_model("selected_modules_with_connect_rules", &source);
    let deck = format!(
        "* Selected modules share one connect specification\n.va \"{path}\" first module=one\n.va \"{path}\" second module=two\nI1 0 a 1\nI2 0 b 1\nX1 a 0 first\nX2 b 0 second\n.end\n",
        path = deck_path(&devices)
    );
    let result = Engine::default()
        .run_dc_op(&Netlist::parse_validated(&deck).unwrap())
        .expect("two modules from one source must register its connect rules only once");
    assert!((node_voltage(&result, "a") - 1.0).abs() < 1e-8);
    assert!((node_voltage(&result, "b") - 0.5).abs() < 1e-8);
    let _ = std::fs::remove_file(connect_only);
    let _ = std::fs::remove_file(devices);
}

#[test]
fn implicit_integrator_initial_condition_is_determined_by_feedback() {
    for (body, enabled, expected) in [
        ("V(output_node)<+idt(V(input_node,output_node));", 1, None),
        (
            "V(output_node)<+idt(1e-15*V(input_node,output_node));",
            1,
            None,
        ),
        (
            "V(output_node)<+idt(1e15*V(input_node,output_node));",
            1,
            None,
        ),
        (
            "if(enabled>0) begin case(enabled) 1: result=idt(V(input_node,output_node)); default: result=0.25; endcase end else result=0.25; V(output_node)<+result;",
            1,
            None,
        ),
        (
            "if(enabled>0) begin case(enabled) 1: result=idt(V(input_node,output_node)); default: result=0.25; endcase end else result=0.25; V(output_node)<+result;",
            2,
            Some(0.25),
        ),
        (
            "error=V(input_node,output_node); result=idt(error); error=error+7; V(output_node)<+result;",
            1,
            None,
        ),
        (
            "V(output_node)<+idt(idt(V(input_node,output_node))-V(output_node));",
            1,
            None,
        ),
        (
            "if (enabled) result=idt(V(input_node,output_node)); else result=0.25; V(output_node)<+result;",
            1,
            None,
        ),
        (
            "if (enabled) result=idt(V(input_node,output_node)); else result=0.25; V(output_node)<+result;",
            0,
            Some(0.25),
        ),
        (
            "V(output_node)<+(enabled ? idt(V(input_node,output_node)) : 0.25);",
            1,
            None,
        ),
        (
            "V(output_node)<+(enabled ? idt(V(input_node,output_node)) : 0.25);",
            0,
            Some(0.25),
        ),
        (
            "V(output_node)<+idt(V(input_node,output_node),0.5);",
            1,
            Some(0.5),
        ),
        (
            "V(output_node)<+idtmod(V(input_node,output_node));",
            1,
            Some(0.0),
        ),
    ] {
        let model = write_model(
            "implicit_integrator_feedback",
            &format!(
                r#"module implicit_integrator_feedback(input_node,output_node);
inout input_node,output_node; electrical input_node,output_node;
parameter integer enabled={enabled};
real error,result,__idt_out1,__idt_input1;
analog begin {body} end
endmodule"#
            ),
        );
        for (bias, options) in [(-2.5, ""), (1.75, ".options rshunt=1e3\n")] {
            let expected = expected.unwrap_or(bias);
            let netlist = Netlist::parse(&format!(
            "* feedback determines the integration constant\n{options}V1 in 0 {bias}\nX1 in out implicit_integrator_feedback\nR1 out 0 1k\n.va \"{}\" implicit_integrator_feedback\n.end\n",
            deck_path(&model),
        )).unwrap();
            let engine = Engine::default();
            let result = engine.run_dc_op(&netlist).unwrap();
            assert!(
                (node_voltage(&result, "out") - expected).abs() < 1e-9,
                "bias={bias}, enabled={enabled}, {body}: {:?}",
                result.node_voltages,
            );
            // Explicit initial conditions may evolve away from their DC value.
            if expected != bias {
                continue;
            }
            let result = engine.run_tran(&netlist, 1e-3, 1e-4).unwrap();
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            assert!(
                result.voltages[output]
                    .iter()
                    .all(|value| (*value - bias).abs() < 1e-9),
                "bias={bias}: {:?}",
                result.voltages[output]
            );
        }
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn implicit_integrator_feedback_has_a_finite_dc_small_signal_limit() {
    let model = write_model(
        "implicit_integrator_ac",
        r#"
module implicit_integrator_ac(input_node,output_node);
inout input_node,output_node; electrical input_node,output_node;
analog V(output_node)<+idt(1000*V(input_node,output_node)+white_noise(1,"input"));
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* first-order feedback\nV1 in 0 DC 2 AC 1\nX1 in out implicit_integrator_ac\nR1 out 0 1k\n.va \"{}\" implicit_integrator_ac\n.end\n", deck_path(&model)
    )).unwrap();
    let frequencies = [0.0, 1.0, 100.0, 1e3, 1e6];
    let points = Engine::default().run_ac(&netlist, &frequencies).unwrap();
    for (frequency, point) in frequencies.into_iter().zip(points) {
        let output = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .unwrap();
        let ratio = std::f64::consts::TAU * frequency / 1000.0;
        let actual = point.voltages[output];
        let denominator = 1.0 + ratio * ratio;
        assert!(
            (actual.re - 1.0 / denominator).abs() < 1e-10
                && (actual.im + ratio / denominator).abs() < 1e-10,
            "f={frequency}: {actual}"
        );
    }
    let engine = Engine::default();
    let circuit = engine.build_circuit(&netlist).unwrap();
    let output = circuit.get_node_by_name("out").unwrap();
    let noise = engine
        .run_noise(&netlist, output, &frequencies[1..], 300.15)
        .unwrap();
    for (frequency, point) in frequencies.into_iter().skip(1).zip(noise) {
        let expected = 1.0 / (1e6 + (std::f64::consts::TAU * frequency).powi(2));
        assert!(
            (point.output_noise_density / expected - 1.0).abs() < 1e-9,
            "f={frequency}: {} vs {expected}",
            point.output_noise_density
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn implicit_integrator_hierarchy_keeps_independent_solver_unknowns() {
    let model = write_model(
        "implicit_integrator_pair",
        r#"
module implicit_integrator_leaf(p,n);
inout p,n; electrical p,n; parameter real gain=1;
analog V(n)<+idt(gain*V(p,n));
endmodule
module implicit_integrator_pair(p,a,b);
inout p,a,b; electrical p,a,b;
implicit_integrator_leaf #(.gain(1000)) first(p,a);
implicit_integrator_leaf #(.gain(2000)) second(p,b);
endmodule"#,
    );
    let source_key = PathBuf::from("__rspice_project__/implicit-integrator/feedback/model.va");
    let netlist = Netlist::parse(&format!(
        "* separate hierarchy sites\nV1 in 0 2\nX1 in a b implicit_integrator_pair\nR1 a 0 1k\nR2 b 0 2k\n.va \"{}\" implicit_integrator_pair\n.end\n", deck_path(&source_key)
    )).unwrap();
    let engine = Engine::default();
    // The project runtime API supports selecting a module from a hierarchy.
    let runtime = rspice_veriloga::VerilogACompiler::default()
        .compile_runtime(
            &std::fs::read_to_string(&model).unwrap(),
            Some("implicit_integrator_pair"),
        )
        .unwrap();
    rspice_core::register_project_veriloga_runtime_for_session(
        &source_key,
        runtime.model,
        runtime.canonical_ir,
    )
    .unwrap();
    let point = engine.run_dc_op(&netlist).unwrap();
    for node in ["a", "b"] {
        assert!((node_voltage(&point, node) - 2.0).abs() < 1e-9);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn implicit_integrator_history_tracks_a_feedback_step() {
    let model = write_model(
        "implicit_integrator_step",
        r#"
module implicit_integrator_step(input_node,output_node);
inout input_node,output_node; electrical input_node,output_node;
analog V(output_node)<+idt(1000*V(input_node,output_node));
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* first-order feedback step\nV1 in 0 PWL(0 0 100u 0 100.001u 1)\nX1 in out implicit_integrator_step\nR1 out 0 1k\n.va \"{}\" implicit_integrator_step\n.end\n", deck_path(&model)
    )).unwrap();
    let result = Engine::default().run_tran(&netlist, 3e-3, 1e-5).unwrap();
    let output = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    for (time, value) in result.time.iter().zip(&result.voltages[output]) {
        let expected = 1.0 - (-1000.0 * (time - 100.0005e-6).max(0.0)).exp();
        assert!(
            (value - expected).abs() < 2e-3,
            "t={time}: {value}, expected {expected}"
        );
    }
    let _ = std::fs::remove_file(model);
}

const REBUILT_LIFECYCLE_MODEL: &str = r#"
`include "disciplines.vams"
module va_dc_rebuild_lifecycle(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0;
    real count;
    analog begin
        @(initial_step("dc")) count = count + 1.0;
        @(final_step("dc")) count = count + 10.0;
        V(p, n) <+ count + 0.0 * gain;
    end
endmodule
"#;

const EARLY_SWEEP_FINISH_MODEL: &str = r#"
module early_sweep_finish(sense,out);
inout sense,out; electrical sense,out;
parameter real threshold=2;
real count;
analog begin
  @(initial_step("dc")) count=count+1;
  @(final_step("dc")) count=count+10;
  if (V(sense)>=threshold && count<10) $finish(1);
  V(out)<+count;
end
endmodule
"#;

#[test]
fn simparam_environment_reaches_analog_and_mixed_parameter_defaults() {
    for mixed in [false, true] {
        let model = write_model(
            "parameter_environment",
            &format!(
                r#"module parameter_environment(out);
inout out; electrical out;
parameter real gain=$simparam("tnom")+50+1000*$simparam("pnjmaxi");
{}
analog V(out)<+gain+$simparam("tnom")+50;
endmodule"#,
                if mixed {
                    "initial begin integer digital; digital=1; end"
                } else {
                    ""
                }
            ),
        );
        for junction_limit in [None, Some(0.003)] {
            let mut netlist = Netlist::parse(&format!(
                "* simulator-owned defaults\nX1 out parameter_environment\n.va \"{}\" parameter_environment\n.end\n",
                deck_path(&model)
            ))
            .unwrap();
            netlist.options.tnom = Some(-40.0);
            netlist.options.device_pnjmaxi = junction_limit;
            let engine = Engine::default();
            if junction_limit.is_none() {
                let error = engine.build_circuit(&netlist).unwrap_err();
                assert!(
                    error.to_string().contains("pnjmaxi"),
                    "mixed={mixed}: {error}"
                );
                continue;
            }
            if !mixed {
                let result = engine.run_dc_op(&netlist).unwrap();
                assert!((node_voltage(&result, "out") - 23.0).abs() < 1e-10);
            }
            let result = engine.run_tran(&netlist, 1e-6, 1e-7).unwrap();
            let output = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap_or_else(|| panic!("node out is absent from {:?}", result.node_names));
            assert!(
                result.voltages[output]
                    .iter()
                    .all(|value| (*value - 23.0).abs() < 1e-10),
                "mixed={mixed}: {:?}",
                result.voltages[output]
            );
        }
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn model_defined_nodeset_selects_an_equilibrium_without_a_netlist_hint() {
    let model = write_model(
        "intrinsic_nodeset",
        r#"module intrinsic_nodeset(out);
inout out; electrical out;
real target;
analog begin
  while (analysis("nodeset") && target<1) target=target+1;
  if (analysis("nodeset")) V(out)<+target;
  else I(out)<+V(out)-(V(out)>0.5 ? 1 : -1);
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* model-defined nodeset\nX1 out intrinsic_nodeset\n.va \"{}\" intrinsic_nodeset\n.end\n",
        deck_path(&model)
    ))
    .unwrap();
    let result = Engine::default().run_dc_op(&netlist).unwrap();
    assert!(
        (node_voltage(&result, "out") - 1.0).abs() < 1e-8,
        "{:?}",
        result.node_voltages
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn model_defined_nodesets_work_with_or_without_external_hints() {
    // The unconstrained model has two equilibria, at -1 V and +1 V.
    // Only the nodeset phase selects +1 V; optional authored hints constrain a
    // different node. This observes the actual intermediate solve through
    // its final equilibrium, without relying on speculative event state.
    for mixed in [false, true] {
        let source = format!(
            r#"module nodeset_choice(out);
inout out; electrical out;
{}
analog I(out)<+V(out)-(analysis("nodeset") || V(out)>0.5 ? 1 : -1);
endmodule"#,
            if mixed {
                "initial begin integer digital; digital=1; end"
            } else {
                ""
            }
        );
        let model = write_model("nodeset_choice", &source);
        for directives in [
            "",
            ".nodeset V(hint)=0.25",
            ".ic V(hint)=0.25",
            ".nodeset V(hint)=0.25\n.ic V(hint)=0.5",
            ".nodeset V(hint,hint)=0",
        ] {
            let netlist = Netlist::parse(&format!(
                "* nodeset phase selection\nR1 hint 0 1k\nX1 out nodeset_choice\n{directives}\n.va \"{}\" nodeset_choice\n.end\n",
                deck_path(&model)
            ))
            .unwrap();
            let engine = Engine::default();
            if !mixed {
                let dc = engine.run_dc_op(&netlist).unwrap();
                assert!(
                    (node_voltage(&dc, "out") - 1.0).abs() < 1e-8,
                    "{directives}: {:?}",
                    dc.node_voltages
                );
                if directives.contains(".ic") {
                    let (forced, _) = engine
                        .run_dc_op_forced_ic_with_report_and_abort(&netlist, &NoAbort)
                        .unwrap();
                    assert!((node_voltage(&forced, "out") + 1.0).abs() < 1e-8);
                }
            }
            let tran = engine.run_tran(&netlist, 1e-5, 1e-6).unwrap();
            let output = tran
                .node_names
                .iter()
                .position(|node| node.eq_ignore_ascii_case("out"))
                .unwrap();
            assert!(
                tran.voltages[output].iter().all(|v| (v - 1.0).abs() < 1e-8),
                "mixed={mixed}, {directives}: {:?}",
                tran.voltages[output]
            );
        }
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn nodeset_model_evaluation_errors_are_not_discarded_as_startup_nonconvergence() {
    let model = write_model(
        "nodeset_error",
        r#"module nodeset_error(out);
inout out; electrical out;
analog begin
  if (analysis("nodeset")) I(out)<+V(out)+sqrt(-1-abs(V(out)));
  else I(out)<+V(out);
end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!(
        "* nodeset error\nR1 hint 0 1k\nX1 out nodeset_error\n.nodeset V(hint)=0.25\n.va \"{}\" nodeset_error\n.end\n",
        deck_path(&model)
    ))
    .unwrap();
    let error = Engine::default().run_dc_op(&netlist).unwrap_err();
    assert!(
        matches!(error, rspice_core::SimulationError::Circuit(_)),
        "{error}"
    );
    let error = Engine::default()
        .run_tran(&netlist, 1e-5, 1e-6)
        .unwrap_err();
    assert!(
        matches!(error, rspice_core::SimulationError::Circuit(_)),
        "{error}"
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn nodesets_only_apply_to_the_first_point_of_rebuilt_and_nested_dc_sweeps() {
    let model = write_model(
        "nodeset_sweep",
        r#"module nodeset_sweep(sense,gate,out);
inout sense,gate,out; electrical sense,gate,out;
parameter real gain=0;
analog begin
  if (analysis("nodeset") && (gain>0.5 || V(sense)>0.5 || V(gate)>0.5 || $temperature>300.5))
    I(out)<+sqrt(-1-abs(V(out)));
  else I(out)<+V(out)-1;
end
endmodule"#,
    );
    for directives in ["", ".nodeset V(hint)=0.25"] {
        let netlist = Netlist::parse(&format!(
        "* first sweep point only\n.param GAIN=0\n.temp 27\nVS sense 0 0\nVG gate 0 0\nR1 hint 0 1k\nX1 sense gate out nodeset_sweep gain={{GAIN}}\n{directives}\n.va \"{}\" nodeset_sweep\n.end\n",
        deck_path(&model)
    )).unwrap();
        for route in ["VS", "GAIN", "TEMP", "nested"] {
            let engine = Engine::default();
            let points = match route {
                "TEMP" => engine.run_dc_sweep(&netlist, "TEMP", 27.0, 28.0, 1.0),
                "nested" => engine.run_dc_sweep2_with_abort(
                    &netlist,
                    "VS",
                    DcSweepRange {
                        start: 0.0,
                        stop: 1.0,
                        step: 1.0,
                    },
                    Some(&rspice_core::netlist::DcSecondSweep::linear(
                        "VG".into(),
                        0.0,
                        1.0,
                        1.0,
                    )),
                    &NoAbort,
                ),
                _ => engine.run_dc_sweep(&netlist, route, 0.0, 1.0, 1.0),
            }
            .unwrap_or_else(|error| panic!("{route}: {error}"));
            assert_eq!(points.len(), if route == "nested" { 4 } else { 2 });
            for (_, result) in points {
                assert!((node_voltage(&result, "out") - 1.0).abs() < 1e-8, "{route}");
            }
        }
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn early_sweep_finish_solves_final_step_before_committing_the_endpoint() {
    use rspice_core::{ModelFinishPoint, SimulationOutcome};
    for route in ["source", "parameter", "temperature", "nested"] {
        let source = match route {
            "temperature" => {
                EARLY_SWEEP_FINISH_MODEL.replace("V(sense)>=threshold", "$temperature>=303.0")
            }
            "nested" => EARLY_SWEEP_FINISH_MODEL.replace(
                "V(sense)>=threshold",
                "$temperature>300.5 && V(sense)>=threshold",
            ),
            _ => EARLY_SWEEP_FINISH_MODEL.into(),
        };
        let model = write_model(&format!("early_finish_{route}"), &source);
        let deck = format!(
            "* early sweep finish\n.param GAIN=4\nVSW sense 0 2\nX1 sense out early_sweep_finish {}\n.va \"{}\" early_sweep_finish\n.end\n",
            if route == "parameter" {
                "threshold={GAIN}"
            } else {
                ""
            },
            deck_path(&model)
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let outcome = Engine::default()
            .run_with_outcome(&NoAbort, |engine, signal| match route {
                "parameter" => {
                    engine.run_dc_sweep_with_abort(&netlist, "GAIN", 4.0, 0.0, -1.0, signal)
                }
                "temperature" => {
                    engine.run_dc_sweep_with_abort(&netlist, "TEMP", 27.0, 32.0, 1.0, signal)
                }
                "nested" => engine.run_dc_sweep2_with_abort(
                    &netlist,
                    "VSW",
                    DcSweepRange {
                        start: 0.0,
                        stop: 4.0,
                        step: 1.0,
                    },
                    Some(&rspice_core::netlist::DcSecondSweep::linear(
                        "TEMP".into(),
                        27.0,
                        28.0,
                        1.0,
                    )),
                    signal,
                ),
                _ => engine.run_dc_sweep_with_abort(&netlist, "VSW", 0.0, 4.0, 1.0, signal),
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: Some(points),
            finish,
        } = outcome
        else {
            panic!("{route}: an early finish must retain the accepted partial sweep");
        };
        let expected_count = match route {
            "temperature" => 4,
            "nested" => 8,
            _ => 3,
        };
        assert_eq!(points.len(), expected_count, "{route}");
        let (value, last) = points.last().unwrap();
        assert_eq!(finish.point, ModelFinishPoint::DcSweep { value: *value });
        assert_eq!(finish.instance, "X1");
        assert_eq!(finish.diagnostic_level, 1);
        assert_eq!(
            node_voltage(last, "out"),
            11.0,
            "{route}: the final_step equation must be solved, with each event assignment accepted once"
        );
        for (_, point) in &points[..points.len() - 1] {
            assert_eq!(node_voltage(point, "out"), 1.0, "{route}");
        }
        let _ = std::fs::remove_file(model);
    }
}

#[test]
fn branch_unknowns_must_be_solved_before_an_ordinary_model_can_finish() {
    let model = write_model(
        "branch_only_finish",
        "module branch_only_finish; analog $finish(0); endmodule",
    );
    let netlist = Netlist::parse(&format!("* branch unknown with no non-ground nodes\nVbad 0 0 1\nX1 branch_only_finish\n.va \"{}\" branch_only_finish\n.end\n", deck_path(&model))).unwrap();
    for sweep in [false, true] {
        let error = Engine::default().run_with_outcome(&NoAbort, |engine, signal| {
            if sweep {
                engine.run_dc_sweep_with_abort(&netlist, "Vbad", 1.0, 2.0, 1.0, signal).map(|_| ())
            } else {
                engine.run_dc_op_with_abort(&netlist, signal).map(|_| ())
            }
        }).expect_err("zero node voltages do not eliminate the unsatisfied voltage-source branch equation");
        assert!(
            !matches!(error, rspice_core::SimulationError::ModelFinished(_)),
            "{error}"
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn a_portless_model_can_finish_an_accepted_sweep_point() {
    use rspice_core::{ModelFinishPoint, SimulationOutcome};
    let model = write_model(
        "portless_sweep_finish",
        r#"module portless_sweep_finish;
parameter real gain=0;
analog begin if (gain>=0) $finish(0); @(final_step("dc")) $finish(2); end
endmodule"#,
    );
    let netlist = Netlist::parse(&format!("* portless accepted control\n.param GAIN=0\nX1 portless_sweep_finish gain={{GAIN}}\n.va \"{}\" portless_sweep_finish\n.end\n", deck_path(&model))).unwrap();
    let operating_point = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_dc_op_with_abort(&netlist, signal)
        })
        .unwrap();
    assert!(
        matches!(operating_point, SimulationOutcome::Finished { result: Some(_), finish } if finish.point == ModelFinishPoint::OperatingPoint)
    );
    let outcome = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_dc_sweep_with_abort(&netlist, "GAIN", 0.0, 4.0, 1.0, signal)
        })
        .unwrap();
    let SimulationOutcome::Finished {
        result: Some(points),
        finish,
    } = outcome
    else {
        panic!("a portless ordinary analog body must execute at the public sweep point");
    };
    assert_eq!(points.len(), 1);
    assert_eq!(finish.point, ModelFinishPoint::DcSweep { value: 0.0 });
    assert_eq!(finish.diagnostic_level, 0);
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilt_sweep_points_refresh_static_guards_from_the_continued_state() {
    let model = write_model(
        "continued_static_guard",
        r#"module continued_static_guard(p,n);
inout p,n; electrical p,n;
real initial_voltage;
analog initial initial_voltage=2.0;
analog if ($temperature<301.0) V(p,n)<+initial_voltage;
endmodule"#,
    );
    let netlist = Netlist::parse(&format!("* reconstructed static branch activation\nX1 out 0 continued_static_guard\nR1 out 0 1k\n.va \"{}\" continued_static_guard\n.end\n", deck_path(&model))).unwrap();
    let points = Engine::default()
        .run_dc_sweep(&netlist, "TEMP", 27.0, 29.0, 1.0)
        .expect("continued static guards must reflect the resolved temperature");
    assert_eq!(points.len(), 3);
    for ((temperature, point), expected) in points.into_iter().zip([2.0, 0.0, 0.0]) {
        assert!(
            (node_voltage(&point, "out") - expected).abs() < 1e-10,
            "temperature {temperature}: expected {expected}, got {}",
            node_voltage(&point, "out")
        );
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilding_a_sweep_point_does_not_execute_an_initializer_again() {
    let model = write_model(
        "initializer_failure_rebuild",
        r#"module initializer_failure_rebuild(p,n);
inout p,n; electrical p,n;
real initial_voltage;
analog initial initial_voltage=sqrt(301.0-$temperature);
analog V(p,n)<+initial_voltage;
endmodule"#,
    );
    let netlist = Netlist::parse(&format!("* initialization executes once for the analysis\nX1 out 0 initializer_failure_rebuild\n.va \"{}\" initializer_failure_rebuild\n.end\n", deck_path(&model))).unwrap();
    let points = Engine::default()
        .run_dc_sweep(&netlist, "TEMP", 27.0, 29.0, 1.0)
        .expect("later sweep points must not execute the now-invalid initializer");
    assert_eq!(points.len(), 3);
    for (_, point) in points {
        assert!((node_voltage(&point, "out") - 0.85f64.sqrt()).abs() < 1e-10);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilding_a_sweep_point_does_not_replay_analog_initial_control() {
    use rspice_core::SimulationOutcome;
    let source = EARLY_SWEEP_FINISH_MODEL
        .replace("V(sense)>=threshold", "0")
        .replace(
            "analog begin",
            "analog initial if ($temperature>301.0) $finish(2);\nanalog begin",
        );
    let model = write_model("initial_control_rebuild", &source);
    let netlist = Netlist::parse(&format!("* one initialization for a rebuilt sweep\nVSW sense 0 0\nX1 sense out early_sweep_finish\n.va \"{}\" early_sweep_finish\n.end\n", deck_path(&model))).unwrap();
    let outcome = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_dc_sweep_with_abort(&netlist, "TEMP", 27.0, 29.0, 1.0, signal)
        })
        .unwrap();
    let SimulationOutcome::Completed(points) = outcome else {
        panic!("reconstruction must not deliver an analog initializer again: {outcome:?}");
    };
    assert_eq!(points.len(), 3);
    assert_eq!(node_voltage(&points[2].1, "out"), 11.0);
    let _ = std::fs::remove_file(model);
}

#[test]
fn a_failed_early_sweep_final_solution_is_not_reported_as_normal_completion() {
    let source = EARLY_SWEEP_FINISH_MODEL.replace("V(out)<+count", "V(out)<+1.0/(11.0-count)");
    let model = write_model("finish_final_failure", &source);
    let netlist = Netlist::parse(&format!("* invalid final solution\nVSW sense 0 0\nX1 sense out early_sweep_finish\n.va \"{}\" early_sweep_finish\n.end\n", deck_path(&model))).unwrap();
    let error = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_dc_sweep_with_abort(&netlist, "VSW", 0.0, 4.0, 1.0, signal)
        })
        .unwrap_err();
    assert!(
        !matches!(error, rspice_core::SimulationError::ModelFinished(_)),
        "{error}"
    );
    let _ = std::fs::remove_file(model);
}

#[test]
fn analog_initial_is_not_replayed_by_dc_newton_or_final_step_evaluations() {
    let model = write_model(
        "analog_initial",
        r#"module va_initial_once(p,n);
inout p,n; electrical p,n;
integer launches;
analog initial launches=launches+1;
analog V(p,n)<+launches;
endmodule"#,
    );
    let deck = format!(
        "* pre-simulation lifecycle\nX1 out 0 va_initial_once\n.va \"{}\" va_initial_once\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    for _ in 0..2 {
        let result = engine
            .run_dc_op(&netlist)
            .expect("initialized DC source converges");
        assert_eq!(node_voltage(&result, "out"), 1.0);
    }
    let _ = std::fs::remove_file(model);
}

#[test]
fn initialization_finish_returns_no_solution_and_does_not_cancel_or_poison_the_engine() {
    use rspice_core::{AbortSignal, AtomicAbort, ModelFinishPoint, SimulationOutcome};
    let model = write_model(
        "finish_initialization",
        r#"
module finish_initialization(p,n);
inout p,n; electrical p,n;
analog initial $finish(0);
analog I(p,n)<+V(p,n);
endmodule"#,
    );
    let deck = format!(
        "* initialization finish\nV1 out 0 2\nX1 out 0 finish_initialization\n.va \"{}\" finish_initialization\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    let abort = AtomicAbort::new();
    for _ in 0..2 {
        let outcome = engine
            .run_with_outcome(&abort, |engine, signal| {
                engine.run_dc_op_with_abort(&netlist, signal)
            })
            .unwrap();
        let SimulationOutcome::Finished {
            result: None,
            finish,
        } = outcome
        else {
            panic!("initialization must not manufacture an operating point: {outcome:?}");
        };
        assert_eq!(finish.model, "finish_initialization");
        assert_eq!(finish.instance, "X1");
        assert_eq!(finish.point, ModelFinishPoint::Initialization);
        assert_eq!(finish.diagnostic_level, 0);
        assert!(!abort.is_aborted());
    }
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine
                .run_tran_with_abort(&netlist, 1e-6, 1e-7, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine.run_ac_with_abort(
                &netlist,
                &[1e3, 1e4],
                signal
            ))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine.run_noise_with_abort(
                &netlist,
                1,
                &[1e3, 1e4],
                300.15,
                signal
            ))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&abort, |engine, signal| engine
                .run_dc_sweep_with_report_and_abort(&netlist, "V1", 0.0, 2.0, 1.0, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(!abort.is_aborted());
    let plain = Netlist::parse("* independent run\nV1 out 0 3\nR1 out 0 1k\n.end\n").unwrap();
    let SimulationOutcome::Completed(result) = engine
        .run_with_outcome(&abort, |engine, signal| {
            engine.run_dc_op_with_abort(&plain, signal)
        })
        .unwrap()
    else {
        panic!("a later run inherited another run's finish state");
    };
    assert_eq!(node_voltage(&result, "out"), 3.0);
    let _ = std::fs::remove_file(model);
}

#[test]
fn accepted_operating_point_finish_keeps_its_solution_and_final_step() {
    use rspice_core::{ModelFinishPoint, SimulationOutcome};
    let model = write_model(
        "finish_dc",
        r#"
module finish_dc(p,n);
inout p,n; electrical p,n;
integer final_count;
analog begin
  @(final_step("dc")) begin final_count=final_count+1; $finish(1); end
  V(p,n)<+final_count;
end
endmodule"#,
    );
    let deck = format!(
        "* accepted finish\nX1 out 0 finish_dc\n.va \"{}\" finish_dc\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let outcome = Engine::default()
        .run_with_outcome(&NoAbort, |engine, signal| {
            engine.run_dc_op_with_abort(&netlist, signal)
        })
        .unwrap();
    let SimulationOutcome::Finished {
        result: Some(result),
        finish,
    } = outcome
    else {
        panic!("an accepted finish must retain its operating point: {outcome:?}");
    };
    assert_eq!(finish.point, ModelFinishPoint::OperatingPoint);
    assert_eq!(finish.diagnostic_level, 1);
    assert_eq!(node_voltage(&result, "out"), 1.0);
    let _ = std::fs::remove_file(model);
}

#[test]
fn a_portless_model_can_finish_before_empty_analysis_shortcuts() {
    use rspice_core::SimulationOutcome;
    let model = write_model(
        "finish_empty",
        "module finish_empty; analog initial $finish(0); endmodule",
    );
    let deck = format!(
        "* empty analog finish\nX1 finish_empty\n.va \"{}\" finish_empty\n.end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).unwrap();
    let engine = Engine::default();
    assert!(matches!(
        engine
            .run_with_outcome(&NoAbort, |engine, signal| engine
                .run_dc_op_with_abort(&netlist, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&NoAbort, |engine, signal| engine
                .run_tran_with_abort(&netlist, 1e-6, 1e-7, signal))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    assert!(matches!(
        engine
            .run_with_outcome(&NoAbort, |engine, signal| engine.run_ac_with_abort(
                &netlist,
                &[1e3],
                signal
            ))
            .unwrap(),
        SimulationOutcome::Finished { result: None, .. }
    ));
    let _ = std::fs::remove_file(model);
}

fn assert_rebuilt_lifecycle_values(
    points: &[(f64, rspice_core::solver::SimulationResult)],
    expected: &[f64],
) {
    assert_eq!(points.len(), expected.len());
    for (index, ((coordinate, result), expected)) in points.iter().zip(expected).enumerate() {
        let actual = node_voltage(result, "out");
        assert!(
            (actual - expected).abs() < 1.0e-12,
            "unexpected rebuilt lifecycle value at point {index} ({coordinate}): actual={actual}, expected={expected}"
        );
    }
}

#[test]
fn forced_initial_conditions_expose_ic_identity_and_boundaries() {
    let model = write_model(
        "forced_ic_identity",
        r#"
`include "disciplines.vams"
module va_forced_ic_identity(anchor, out);
    inout anchor, out;
    electrical anchor, out;
    real level;
    analog begin
        level = 0.0;
        if (analysis("ic")) level = level + 1.0;
        if (analysis("dc")) level = level + 100.0;
        @(initial_step("ic")) level = level + 2.0;
        @(final_step("ic")) level = level + 4.0;
        V(out) <+ level;
    end
endmodule
"#,
    );
    let deck = format!(
        "* forced-IC physical analysis identity\n\
         R1 anchor 0 1g\n\
         X1 anchor out va_forced_ic_identity\n\
         .ic V(anchor)=1\n\
         .va \"{}\" va_forced_ic_identity\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse forced-IC lifecycle deck");
    let ordinary = Engine::default()
        .run_dc_op(&netlist)
        .expect("ordinary DC operating point runs");
    assert!((node_voltage(&ordinary, "out") - 100.0).abs() < 1.0e-12);

    let (forced, _) = Engine::default()
        .run_dc_op_forced_ic_with_report_and_abort(&netlist, &NoAbort)
        .expect("forced-IC operating point runs");
    assert!((node_voltage(&forced, "anchor") - 1.0).abs() < 1.0e-12);
    assert!((node_voltage(&forced, "out") - 7.0).abs() < 1.0e-12);

    let _ = std::fs::remove_file(model);
}

#[test]
fn dc_operating_point_exposes_initial_and_final_step_together() {
    let model = write_model(
        "single_point",
        r#"
`include "disciplines.vams"
module va_dc_single_point(p, n);
    inout p, n;
    electrical p, n;
    real level;
    analog begin
        level = 0.0;
        @(initial_step("dc")) level = level + 1.0;
        @(final_step("dc")) level = level + 2.0;
        V(p, n) <+ level;
    end
endmodule
"#,
    );
    let deck = format!(
        "* one-point DC lifecycle\n\
         X1 out 0 va_dc_single_point\n\
         .va \"{}\" va_dc_single_point\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let result = Engine::default().run_dc_op(&netlist).expect("DC OP runs");
    assert!((node_voltage(&result, "out") - 3.0).abs() < 1.0e-12);

    let _ = std::fs::remove_file(model);
}

#[test]
fn initial_and_final_step_state_is_committed_at_the_public_sweep_endpoints() {
    let model = write_model(
        "sweep_initial",
        r#"
`include "disciplines.vams"
module va_dc_sweep_initial(p, n);
    inout p, n;
    electrical p, n;
    real count;
    analog begin
        @(initial_step("dc")) count = count + 1.0;
        @(final_step("dc")) count = count + 10.0;
        V(p, n) <+ count;
    end
endmodule
"#,
    );
    let deck = format!(
        "* persistent DC sweep lifecycle\n\
         VSW sense 0 0\n\
         X1 out 0 va_dc_sweep_initial\n\
         .va \"{}\" va_dc_sweep_initial\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "VSW", -1.0, 2.0, 1.0)
        .expect("DC source sweep runs");
    assert_eq!(points.len(), 4);
    for (point_index, (coordinate, result)) in points.into_iter().enumerate() {
        let expected = if point_index == 3 { 11.0 } else { 1.0 };
        assert!(
            (node_voltage(&result, "out") - expected).abs() < 1.0e-12,
            "unexpected lifecycle value at sweep coordinate {coordinate}: expected {expected}"
        );
    }

    let _ = std::fs::remove_file(model);
}

#[test]
fn above_crossing_state_is_committed_between_source_sweep_points() {
    let model = write_model(
        "sweep_above",
        r#"
`include "disciplines.vams"
module va_dc_sweep_above(sense, out);
    input sense;
    output out;
    electrical sense, out;
    real latched;
    analog begin
        @(above(V(sense))) latched = 1.0;
        V(out) <+ latched;
    end
endmodule
"#,
    );
    let deck = format!(
        "* same-time DC above lifecycle\n\
         VSW sense 0 0\n\
         X1 sense out va_dc_sweep_above\n\
         .va \"{}\" va_dc_sweep_above\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "VSW", -1.0, 3.0, 2.0)
        .expect("DC source sweep runs");
    let observed = points
        .iter()
        .map(|(_, result)| node_voltage(result, "out"))
        .collect::<Vec<_>>();
    assert_eq!(observed.len(), 3);
    assert!(observed[0].abs() < 1.0e-12, "{observed:?}");
    assert!((observed[1] - 1.0).abs() < 1.0e-12, "{observed:?}");
    assert!((observed[2] - 1.0).abs() < 1.0e-12, "{observed:?}");

    let _ = std::fs::remove_file(model);
}

#[test]
fn two_rebuilt_temperature_circuits_continue_one_accepted_lifecycle() {
    let model = write_model("rebuilt_temp", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* rebuilt TEMP sweep lifecycle\n\
         X1 out 0 va_dc_rebuild_lifecycle\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse rebuilt TEMP lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "TEMP", 25.0, 26.0, 1.0)
        .expect("rebuilt TEMP sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilt_global_parameter_sweep_continues_accepted_veriloga_state() {
    let model = write_model("rebuilt_global_param", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* rebuilt global parameter lifecycle\n\
         .param GAIN=1\n\
         X1 out 0 va_dc_rebuild_lifecycle gain={{GAIN}}\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse rebuilt parameter lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "gain", 1.0, 3.0, 1.0)
        .expect("rebuilt global parameter sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

#[test]
fn rebuilt_device_parameter_sweep_continues_accepted_veriloga_state() {
    let model = write_model("rebuilt_device_param", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* rebuilt device parameter lifecycle\n\
         RLOAD sense 0 1k\n\
         X1 out 0 va_dc_rebuild_lifecycle\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse rebuilt device lifecycle deck");
    let points = Engine::default()
        .run_dc_sweep(&netlist, "rload:r", 1.0e3, 3.0e3, 1.0e3)
        .expect("rebuilt device parameter sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

#[test]
fn nested_rebuilt_sweep_uses_flattened_public_point_boundaries() {
    let model = write_model("rebuilt_nested", REBUILT_LIFECYCLE_MODEL);
    let deck = format!(
        "* nested rebuilt lifecycle\n\
         VSW sense 0 0\n\
         X1 out 0 va_dc_rebuild_lifecycle\n\
         .va \"{}\" va_dc_rebuild_lifecycle\n\
         .end\n",
        deck_path(&model)
    );
    let netlist = Netlist::parse(&deck).expect("parse nested rebuilt lifecycle deck");
    let outer = rspice_core::netlist::DcSecondSweep::linear("TEMP".to_string(), 25.0, 26.0, 1.0);
    let points = Engine::default()
        .run_dc_sweep2_with_abort(
            &netlist,
            "VSW",
            DcSweepRange {
                start: 0.0,
                stop: 1.0,
                step: 1.0,
            },
            Some(&outer),
            &NoAbort,
        )
        .expect("nested rebuilt sweep runs");
    assert_rebuilt_lifecycle_values(&points, &[1.0, 1.0, 1.0, 11.0]);
    let _ = std::fs::remove_file(model);
}

/// A hoisted inner-`if` snapshot does not leak across Newton evaluations.
///
/// The frontend snapshots every non-trivial `if` condition into an
/// unconditional `__guardN` assignment; inside an untaken block the snapshot
/// reads whatever the block did not write, which on the bytecode route is the
/// previous evaluation's value of `t` (3.0, so `t > 0.1` reads true from the
/// second evaluation on) and on the JIT route is zero. Its only consumer is
/// ANDed with the block's own condition, so `g` stays 1.0 on both routes and
/// the operating point is the same whichever route this build selects:
/// `--features veriloga` evaluates the bytecode, `--features veriloga-native`
/// the x64 JIT. The finite oracle in `rspice-veriloga` pins the snapshot
/// itself; this pins that the engine never sees it.
#[test]
fn a_hoisted_condition_snapshot_does_not_leak_across_evaluations() {
    let model = write_model(
        "condition_snapshot",
        r#"
`include "disciplines.vams"
module va_condition_snapshot(p, n);
    inout p, n;
    electrical p, n;
    parameter real vth = 0.5;
    real g, t;
    analog begin
        g = 1.0;
        if (V(p, n) > vth) begin
            t = V(p, n) - vth;
            if (t > 0.1) g = 2.0;
        end
        t = 3.0;
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
    );
    // V(in) = 0.3 keeps V(p, n) below vth on every iteration, so the block is
    // never taken and the device is a 1 S conductance: V(out) = 0.3 / (1 + 1e-3).
    let deck = format!(
        "* hoisted condition snapshot across evaluations\n\
         V1 in 0 0.3\n\
         X1 in out va_condition_snapshot\n\
         R1 out 0 1k\n\
         .va \"{}\" va_condition_snapshot\n\
         .end\n",
        deck_path(&model)
    );

    let netlist = Netlist::parse(&deck).expect("parse snapshot deck");
    let result = Engine::default().run_dc_op(&netlist).expect("DC OP runs");
    let expected = 0.3 / (1.0 + 1.0e-3);
    let actual = node_voltage(&result, "out");
    assert!(
        (actual - expected).abs() < 1.0e-9,
        "V(out) = {actual}, expected {expected}: g must stay 1.0 on the second evaluation on"
    );

    let _ = std::fs::remove_file(model);
}

/// Plain Newton from the zero guess overshoots `ln()`'s domain on its second
/// iterate here. That is a rejected iterate, not a refused circuit: source
/// stepping walks in from a reachable bias and the operating point exists.
#[test]
fn a_nonfinite_verilog_a_dc_trial_is_rejected_and_source_stepped() {
    let model = write_model(
        "nonfinite_dc_trial",
        "module nonfinite_dc_trial(p,n); inout p,n; electrical p,n;\n\
             analog I(p,n) <+ 1.0e-3*ln(V(p,n) + 0.1);\n\
         endmodule\n",
    );
    let netlist = Netlist::parse_validated(&format!(
        "* plain Newton overshoots a log singularity from the zero guess\n\
         V1 in 0 DC -5\n\
         R1 in p 1k\n\
         X1 p 0 nonfinite_dc_trial\n\
         .va \"{}\" nonfinite_dc_trial\n\
         .end\n",
        deck_path(&model)
    ))
    .unwrap();
    let dc = Engine::default()
        .run_dc_op(&netlist)
        .expect("a non-finite trial iterate must reject the iterate, not the run");
    let voltage = node_voltage(&dc, "p");
    assert!(
        voltage > -0.1,
        "V(p) = {voltage} is outside the model's domain"
    );
    let residual = (voltage + 5.0) * 1.0e-3 + 1.0e-3 * (voltage + 0.1).ln();
    assert!(
        residual.abs() < 1.0e-6,
        "V(p) = {voltage} leaves KCL residual {residual}"
    );

    let _ = std::fs::remove_file(model);
}

/// When no retry can produce a finite evaluation the run still fails - and the
/// failure has to name the instance, the failing contribution and the iterate
/// the device was handed, not just an iteration count.
#[test]
fn an_unreachable_verilog_a_domain_names_the_instance_and_its_iterate() {
    let model = write_model(
        "always_nonfinite",
        "module always_nonfinite(p,n); inout p,n; electrical p,n;\n\
             analog I(p,n) <+ 1.0e-3*ln(-1.0 - V(p,n)*V(p,n));\n\
         endmodule\n",
    );
    let netlist = Netlist::parse_validated(&format!(
        "* the module's argument is negative at every real bias\n\
         V1 in 0 DC 1\n\
         R1 in p 1k\n\
         X1 p 0 always_nonfinite\n\
         .va \"{}\" always_nonfinite\n\
         .end\n",
        deck_path(&model)
    ))
    .unwrap();
    let error = Engine::default()
        .run_dc_op(&netlist)
        .expect_err("no retry can make this module evaluate finitely");
    let message = error.to_string();
    let lowered = message.to_lowercase();
    assert!(lowered.contains("x1"), "{message}");
    assert!(
        message.contains("non-finite value at a trial iterate"),
        "{message}"
    );
    assert!(message.contains("contribution"), "{message}");
    assert!(message.contains("p="), "{message}");

    let _ = std::fs::remove_file(model);
}

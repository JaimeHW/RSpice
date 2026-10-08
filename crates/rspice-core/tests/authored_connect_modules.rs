#![cfg(feature = "veriloga")]
//! Selected connection bodies execute in the same circuit as HDL and XSPICE.
use rspice_core::{Engine, Netlist};
use rspice_core::engine::TransientResult;

struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let id = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-authored-connect-{}-{id}.va",
            std::process::id()
        ));
        std::fs::write(&path, text).unwrap();
        Self(path)
    }
    fn path(&self) -> String {
        self.0.to_string_lossy().replace('\\', "/")
    }
}
impl Drop for Source {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}
fn voltage(result: &TransientResult, name: &str, time: f64) -> f64 {
    let node = result
        .node_names
        .iter()
        .position(|node| node.eq_ignore_ascii_case(name))
        .unwrap();
    let sample = result.time.iter().position(|value| *value >= time).unwrap();
    result.voltage_waveform(node + 1)[sample]
}

const DAC: &str = r#"
module output_stage(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1.7, resistance=40;
 analog I(a) <+ (V(a)-transition(d ? level : 0.0,0,20p,20p))/resistance;
endmodule
connectmodule d2a(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1.7, resistance=40;
 output_stage #(.level(level),.resistance(resistance)) stage(d,a);
endmodule
"#;

#[test]
fn hierarchy_connect_insertion_preserves_merged_split_loading_and_sampling() {
    for (mode, high) in [("merged", 1.5), ("split", 2.0)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module source(q);
 parameter integer WIDTH=2;
 output [WIDTH-1:0] q; logic q; reg [WIDTH-1:0] q;
 initial begin q=0; #1 q=1; #1 q=0; end
endmodule
module observer(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 1.0 : 0.0))/100;
endmodule
module group(p,m);
 inout p,m; electrical p,m;
 parameter integer N=1;
 source #(.WIDTH(1)) first(p);
 generate if (N==2) begin : extra
   source #(.WIDTH(1)) second(p);
 end endgenerate
 observer monitor(p,m);
endmodule
module top(p,m);
 inout p,m; electrical p,m;
 group #(.N(2)) nested(p,m);
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1, resistance=100;
 analog I(a)<+(V(a)-(d ? level : 0.0))/resistance;
endmodule
connectmodule sense(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 parameter real threshold=0.5;
 initial d=0;
 always #0.1 d=V(a)>threshold;
endmodule
connectrules selected;
 connect drive {mode} #(.level(3.0),.resistance(1000));
 connect sense #(.threshold(1.3));
endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!(
            "* implicit hierarchical converters\nX1 p m top\nRp p 0 1k\nRm m 0 1k\n.va \"{}\" top module=top\n.end\n", source.path(),
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 2.5e-9, 50e-12).unwrap();
        for (time, expected) in [(0.5e-9, 0.0), (1.5e-9, high), (2.4e-9, 0.0)] {
            assert!(
                (voltage(&result, "p", time) - expected).abs() < 1e-7,
                "{mode}, t={time}"
            );
        }
        assert!(
            (voltage(&result, "m", 1.5e-9) - 1.0 / 1.1).abs() < 1e-7,
            "{mode}"
        );
        assert!(voltage(&result, "m", 2.4e-9).abs() < 1e-7, "{mode}");
        if mode == "merged" {
            let foreign = Source::new(
                r#"
`timescale 1ns/1ps
connectmodule replacement_driver(a,d);
 output a; electrical a;
 input d; logic d;
 analog V(a)<+7.0;
endmodule
connectmodule replacement_sense(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>6;
endmodule
connectrules replacement;
 connect replacement_driver;
 connect replacement_sense;
endconnectrules
"#,
            );
            let override_deck=Netlist::parse(&format!(
                "* external configuration re-elaborates internal connections\nX1 p m top\nRp p 0 1k\nRm m 0 1k\n.va \"{}\" top module=top\n.va \"{}\" replacement\n.options connectrules=replacement\n.end\n", source.path(),foreign.path(),
            )).unwrap();
            let overridden = Engine::default().run_tran(&override_deck, 2.5e-9, 50e-12).unwrap();
            assert!((voltage(&overridden, "p", 1.5e-9)-7.0).abs()<1e-7);
            assert!((voltage(&overridden, "m", 1.5e-9)-1.0/1.1).abs()<1e-7);
            let restored = Engine::default().run_tran(&deck, 2.5e-9, 50e-12).unwrap();
            assert!((voltage(&restored, "p", 1.5e-9)-high).abs()<1e-7);
        }
    }
}

#[test]
fn hierarchy_connect_insertion_binds_real_parent_to_loaded_analog_child() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module amplifier(a,p);
 input a; electrical a;
 output p; electrical p;
 analog begin
   I(a)<+V(a)/1000;
   I(p)<+(V(p)-2.0*V(a))/50;
 end
endmodule
module top(p);
 inout p; electrical p;
 wreal value;
 real level=1.5;
 initial #1 level=3.0;
 assign value=level;
 amplifier child(.a(value),.p(p));
endmodule
connectmodule real_drive(a,r);
 output a; electrical a;
 input r; logic r; wreal r;
 parameter real resistance=100;
 analog I(a)<+(V(a)-r)/resistance;
endmodule
connectrules selected;
 connect real_drive;
endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* reverse hierarchy domain crossing\nX1 p top\nRp p 0 1k\n.va \"{}\" top module=top\n.end\n",
        source.path(),
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
    for (time, level) in [(0.5e-9, 1.5), (1.4e-9, 3.0)] {
        let expected = 2.0 * level / 1.1 / 1.05;
        assert!(
            (voltage(&result, "p", time) - expected).abs() < 1e-7,
            "t={time}"
        );
    }
}

#[test]
fn authored_d2a_body_parameters_hierarchy_and_loading_execute() {
    let source = Source::new(&format!(
        r#"
`timescale 1ns/1ps
module source(q);
 output q; reg q;
 initial begin q=0; #1 q=1; #1 q=0; end
endmodule
{DAC}
connectrules selected;
 connect d2a #(.level(2.4),.resistance(50));
endconnectrules
"#
    ));
    let deck = Netlist::parse(&format!(
        "* authored body shares the builtin name\nX1 out source\nRload out 0 1k\n.va \"{}\" source module=source\n.end\n", source.path()
    )).unwrap();
    let engine = Engine::default();
    for _ in 0..2 {
        let result = engine.run_tran(&deck, 2.5e-9, 100e-12).unwrap();
        assert!(voltage(&result, "out", 0.5e-9).abs() < 1e-8);
        assert!((voltage(&result, "out", 1.5e-9) - 2.4 / 1.05).abs() < 1e-6);
        assert!(voltage(&result, "out", 2.4e-9).abs() < 1e-8);
    }
}

#[test]
fn authored_adc_and_dac_join_xspice_with_independent_sampled_state() {
    let source = Source::new(&format!(
        r#"
`timescale 1ps/1ps
module sampler(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 parameter real threshold=0.2;
 branch (a) sense;
 real levels[0:1];
 analog begin
   I(sense) <+ V(a)/1e12;
   levels[0]=V(a);
   levels[1]=0.5*levels[0];
 end
 initial d=0;
 always #100 d=(V(a)>threshold && I(sense)>threshold/1e12
                  && levels[0]>threshold && levels[1]>0.5*threshold);
endmodule
connectmodule sampled_adc(a,d);
 input a; electrical a;
 output d; logic d;
 parameter real threshold=0.2;
 sampler #(.threshold(threshold)) sample(a,d);
endmodule
{DAC}
connectrules selected;
 connect sampled_adc #(.threshold(0.65));
 connect d2a;
endconnectrules
"#
    ));
    let deck = Netlist::parse(&format!(
        "* authored sample timing with XSPICE\nV1 din 0 pwl(0 0 1n 1)\nA1 din dout buffer\n.model buffer d_buffer(rise_delay=17p fall_delay=17p)\nRload dout 0 1k\nV2 quiet_in 0 0.25\nA2 quiet_in quiet_out buffer\nRquiet quiet_out 0 1k\n.va \"{}\" helper module=output_stage\n.end\n", source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1.1e-9, 70e-12).unwrap();
    assert!(voltage(&result, "dout", 0.65e-9).abs() < 1e-8);
    assert!((voltage(&result, "dout", 0.8e-9) - 1.7 / 1.04).abs() < 1e-6);
    assert!(voltage(&result, "quiet_out", 0.8e-9).abs() < 1e-8);
    // The authored sampler's clock, not an inferred comparator crossing,
    // produces the event. The downstream XSPICE delay remains physical time.
    let trace = &result
        .digital_traces
        .iter()
        .find(|trace| {
            trace
                .node_name
                .eq_ignore_ascii_case("DOUT__D2A__LOGIC__EVENT")
        })
        .unwrap_or_else(|| {
            panic!(
                "missing DAC trace: {:?}",
                result
                    .digital_traces
                    .iter()
                    .map(|trace| &trace.node_name)
                    .collect::<Vec<_>>()
            )
        })
        .points;
    let rise = trace
        .iter()
        .find(|point| point.value.state == rspice_core::xspice::DigitalState::One)
        .unwrap();
    assert!((rise.time - 717e-12).abs() < 2e-20, "{trace:?}");
}

#[test]
fn authored_real_inout_uses_its_loading_law_and_preserves_single_driver_nets() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module real_source(q);
 inout q; wreal q;
 real level=1.5;
 initial #1 level=3.0;
 assign q=level;
endmodule
connectmodule real_boundary(a,r);
 inout a; electrical a;
 inout r; logic r; wreal r;
 parameter real resistance=100;
 analog I(a)<+(V(a)-r)/resistance;
endmodule
connectmodule logic_boundary(a,d);
 inout a; electrical a;
 inout d; logic d;
 analog I(a)<+V(a);
endmodule
connectrules chosen;
 connect logic_boundary;
 connect real_boundary #(.resistance(500));
endconnectrules
"#,
    );
    let deck=Netlist::parse(&format!(
        "* explicit real inout loading\nX1 physical real_source\nRload physical 0 1k\n.va \"{}\" real_source\n.end\n",source.path()
    )).unwrap();
    let engine = Engine::default();
    for _ in 0..2 {
        let result = engine.run_tran(&deck, 2e-9, 100e-12).unwrap();
        assert!((voltage(&result, "physical", 0.5e-9) - 1.0).abs() < 1e-7);
        assert!((voltage(&result, "physical", 1.5e-9) - 2.0).abs() < 1e-7);
        assert!(result.event_only_node_kind("physical").is_none());
    }
}

#[test]
fn authored_real_input_output_bodies_preserve_xspice_gain_and_propagation() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module unused(p); inout p; electrical p; analog I(p)<+V(p); endmodule
connectmodule sense(a,r);
 input a; electrical a;
 output r; logic r; wreal r;
 real sample=0;
 always #100 sample=V(a);
 assign r=sample;
endmodule
connectmodule drive(a,r);
 output a; electrical a;
 input r; logic r; wreal r;
 analog I(a)<+(V(a)-r)/200;
endmodule
connectrules chosen;
 connect sense;
 connect drive;
endconnectrules
"#,
    );
    let deck=Netlist::parse(&format!(
        "* authored real XSPICE conversion\nV1 input 0 pwl(0 0 1n 1)\nA1 input output g\n.model g real_gain(gain=2 delay=17p)\nRload output 0 1k\n.va \"{}\" unused\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1.1e-9, 70e-12).unwrap();
    assert!((voltage(&result, "output", 1.02e-9) - 2.0 / 1.2).abs() < 1e-7);
    let trace = result
        .real_trace_named("OUTPUT__drive__logic__event")
        .expect("real drive trace");
    let step = trace
        .iter()
        .find(|point| (point.value - 1.4).abs() < 1e-10)
        .unwrap();
    assert!((step.time - 717e-12).abs() < 2e-20, "{trace:?}");
}

#[test]
fn authored_real_inout_resolves_independent_drivers_in_loaded_feedback() {
    let model = r#"
`timescale 1ps/1ps
module real_source(q);
 inout q; wrealsum q;
 real level=1.5;
 initial #1000 level=3.0;
 assign q=level;
endmodule
connectmodule feedback(a,r);
 inout a; electrical a;
 inout r; logic r; wrealsum r;
 real sample=0, returned=0;
 always begin #100 sample=0.25*V(a); #10 returned=sample; end
 assign r=returned;
 analog I(a)<+(V(a)-r)/500;
endmodule
connectrules chosen;
 connect feedback;
endconnectrules
"#;
    for resolved in [true, false] {
        let source = Source::new(&if resolved {
            model.to_string()
        } else {
            model.replace("wrealsum", "wreal")
        });
        let deck = Netlist::parse(&format!(
            "* independent drive and return feedback\nX1 physical real_source\nRload physical 0 1k\n.va \"{}\" real_source\n.end\n", source.path()
        )).unwrap();
        let outcome = Engine::default().run_tran(&deck, 1.8e-9, 25e-12);
        if !resolved {
            let error = outcome.unwrap_err().to_string();
            assert!(
                error.contains("driver") && error.contains("resolution"),
                "{error}"
            );
            continue;
        }
        let result = outcome.unwrap();
        // Rload and the authored 500-ohm source give V=(level+sample)/1.5.
        // Each clock feeds a quarter of the physical voltage back after 10 ps
        // as a distinct RNM driver: V[n+1]=level/1.5+V[n]/6, with equilibrium
        // level/1.25. The delay separates sampling from its physical consequence.
        for (time, expected) in [(50e-12, 1.0), (150e-12, 7.0 / 6.0), (250e-12, 43.0 / 36.0)] {
            let actual = voltage(&result, "physical", time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "t={time}: {actual} != {expected}"
            );
        }
        assert!((voltage(&result, "physical", 1.75e-9) - 2.4).abs() < 1e-5);
    }
}

#[test]
fn hierarchy_connect_insertion_keeps_bidirectional_real_drivers_and_loading() {
    for (mode, initial) in [("merged", 2.0), ("split", 1.2)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module driver(r);
 inout r; wrealsum r;
 parameter real drive=1;
 real value=drive;
 initial #1 value=2*drive;
 assign r=value;
endmodule
module top(p);
 inout p; electrical p;
 driver #(.drive(1.0)) first(p);
 driver #(.drive(2.0)) second(p);
endmodule
connectmodule bidir(a,r);
 inout a; electrical a;
 inout r; logic r; wrealsum r;
 analog I(a)<+(V(a)-r)/500;
endmodule
connectrules chosen;
 connect bidir {mode};
endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!(
            "* internal real driver segregation\nX1 p top\nRp p 0 1k\n.va \"{}\" top module=top\n.end\n", source.path(),
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
        for (time, expected) in [(0.5e-9, initial), (1.4e-9, 2.0 * initial)] {
            assert!(
                (voltage(&result, "p", time) - expected).abs() < 1e-7,
                "{mode}, t={time}"
            );
        }
    }
}


#[test]
fn configured_hierarchy_replays_library_and_block_after_transport_and_specialization() {
    use rspice_veriloga::{CompilerOptions, NoPipelineControl, VerilogACompiler};
    const DEVICE: &str = r#"
`timescale 1ns/1ps
`default_transition 9n
module source(q);
 output q; logic q; reg q;
 initial begin q=0; #0.1 q=1; end
endmodule
module top(p);
 inout p; electrical p;
 parameter integer N=1;
 source first(p);
 generate if (N==2) begin : extra
  source second(p);
 end endgenerate
endmodule
"#;
    const LIBRARY: &str = r#"
module stage(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1;
 analog I(a)<+(V(a)-transition(d ? level : 0.0))/1000;
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1;
 stage #(.level(level)) body(d,a);
endmodule
connectrules low;
 connect drive split #(.level(1.0));
endconnectrules
connectrules high;
 connect drive split #(.level(3.0));
endconnectrules
"#;
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    for external in [false, true] {
        let device = Source::new(&if external {
            DEVICE.to_owned()
        } else {
            format!("{DEVICE}\n`default_transition 1n\n{LIBRARY}")
        });
        let library = Source::new(LIBRARY);
        let prepared = compiler.prepare_file_runtime_source(&device.0).unwrap();
        let library_prepared = compiler.prepare_file_runtime_source(&library.0).unwrap();
        let mut identities = Vec::new();
        for (block, expected) in [("low", 2.0 / 3.0), ("high", 2.0)] {
            let configuration = if external {
                &library_prepared
            } else {
                &prepared
            }
            .connection_configuration(block)
            .unwrap();
            let compiled = prepared
                .compile_runtime_with_connections(Some("top"), &configuration, &NoPipelineControl)
                .unwrap();
            identities.push(compiled.canonical_ir.connection_identity);
            let serialized = serde_json::to_vec(&compiled.canonical_ir).unwrap();
            let artifact: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
                serde_json::from_slice(&serialized).unwrap();
            artifact.validate().unwrap();
            assert_eq!(artifact.connections.configuration(), Some(&configuration));
            // The global registration consumes a real cache slot. Instance N=2
            // then forces a source specialization of the transported selection.
            rspice_core::register_precompiled_veriloga_runtime_with_dependencies(
                &device.0,
                &[],
                compiled.model,
                artifact,
            )
            .unwrap();
            let library_import = if external {
                format!(".va \"{}\" converters module=stage\n", library.path())
            } else {
                String::new()
            };
            let deck = Netlist::parse(&format!(
                "* selected hierarchy configuration\n.options connectrules={block}\nX1 p top N=2\nRp p 0 1k\n.va \"{}\" top\n{library_import}.end\n", device.path(),
            )).unwrap();
            let result = Engine::default().run_tran(&deck, 1.6e-9, 50e-12).unwrap();
            let actual = voltage(&result, "p", 1.5e-9);
            assert!(
                (actual - expected).abs() < 1e-7,
                "external={external}, block={block}: {actual} != {expected}"
            );
            let mut tampered: serde_json::Value = serde_json::from_slice(&serialized).unwrap();
            tampered["connections"]["Configured"]["configuration"]["block"] = "different".into();
            let altered: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
                serde_json::from_value(tampered).unwrap();
            assert!(
                altered.validate().is_err(),
                "changing the selected block must invalidate the artifact"
            );
        }
        assert_ne!(identities[0], identities[1]);
    }
}

#[test]
fn configured_library_conflicts_keep_external_source_coordinates() {
    use rspice_veriloga::{CompilerOptions, NoPipelineControl, VerilogACompiler};
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    for (device_prefix, library_prefix, expected) in [
        (
            "",
            "module top(p); inout p; electrical p; analog I(p)<+V(p); endmodule",
            "ordinary module 'top'",
        ),
        (
            "nature Shared; units=\"V\"; access=VS; abstol=1u; endnature",
            "nature Shared; units=\"A\"; access=IS; abstol=1u; endnature",
            "nature 'Shared'",
        ),
    ] {
        let device = Source::new(&format!(
            "{device_prefix}\nmodule top(p); inout p; electrical p; analog I(p)<+V(p); endmodule"
        ));
        let library = Source::new(&format!(
            r#"{library_prefix}
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 1.0 : 0.0))/1000;
endmodule
connectrules chosen; connect drive; endconnectrules
"#
        ));
        let prepared = compiler.prepare_file_runtime_source(&device.0).unwrap();
        let library_prepared = compiler.prepare_file_runtime_source(&library.0).unwrap();
        assert!(
            library_prepared
                .connection_configuration("missing")
                .is_err()
        );
        let configuration = library_prepared.connection_configuration("chosen").unwrap();
        let error = prepared
            .compile_runtime_with_connections(Some("top"), &configuration, &NoPipelineControl)
            .unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        let diagnostics = prepared.diagnostics_for_error_with_connections(&configuration, &error);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].path.as_deref(),
            Some(
                format!(
                    "{} (preprocessed)",
                    configuration.library().source_package()
                )
                .as_str()
            )
        );
        assert!(diagnostics[0].line.is_some());
        assert!(
            prepared.diagnostics_for_error(&error)[0].path.is_none(),
            "external bytes must never be mapped to the device file"
        );
    }
}


const CONFIGURED_DEVICE: &str = r#"
module source(q);
 output q; logic q; reg q;
 initial q=1;
endmodule
module top(p);
 parameter integer N=1;
 inout p; electrical p;
 source first(p);
 generate if (N==2) begin : extra
  source second(p);
 end endgenerate
endmodule
"#;
const CONFIGURED_LIBRARY: &str = r#"
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 parameter real level=1;
 analog I(a)<+(V(a)-(d ? level : 0.0))/1000;
endmodule
connectrules low; connect drive split #(.level(1.0)); endconnectrules
connectrules high; connect drive split #(.level(3.0)); endconnectrules
"#;

#[test]
fn deck_selects_external_rules_before_cold_hierarchy_and_keeps_cached_variants() {
    let device = Source::new(CONFIGURED_DEVICE);
    let library = Source::new(CONFIGURED_LIBRARY);
    for (block, expected, reverse) in [
        ("low", 2.0 / 3.0, false),
        ("high", 2.0, true),
        ("low", 2.0 / 3.0, true),
    ] {
        let imports = [
            format!(".va \"{}\" top module=top\n", device.path()),
            format!(".va \"{}\" library\n", library.path()),
        ];
        let imports = if reverse {
            format!("{}{}", imports[1], imports[0])
        } else {
            imports.concat()
        };
        let deck=Netlist::parse(&format!("* external selected rules\n.options connectrules={block} connectrules_source=library\nX1 p top N=2\nRp p 0 1k\n{imports}.end\n")).unwrap();
        let result = Engine::default().run_tran(&deck, 0.2e-9, 50e-12).unwrap();
        assert!(
            (voltage(&result, "p", 0.1e-9) - expected).abs() < 1e-7,
            "{block}, reverse={reverse}"
        );
    }
}

#[test]
fn deck_reconfigures_registered_specializations_without_resetting_root_assignments() {
    use rspice_veriloga::{CompilerOptions, NoPipelineControl, VerilogACompiler};
    let source = Source::new(&format!("{CONFIGURED_DEVICE}\n{CONFIGURED_LIBRARY}"));
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let prepared = compiler.prepare_file_runtime_source(&source.0).unwrap();
    let low = prepared.connection_configuration("low").unwrap();
    let original = prepared
        .compile_runtime_with_connections(Some("top"), &low, &NoPipelineControl)
        .unwrap();
    let specialized = compiler
        .specialize_mixed_runtime(&original.canonical_ir, &[("N", 2.0)], &NoPipelineControl)
        .unwrap();
    let serialized = serde_json::to_vec(&specialized.canonical_ir).unwrap();
    let specialized_artifact = serde_json::from_slice(&serialized).unwrap();
    rspice_core::register_precompiled_veriloga_runtime_with_dependencies(
        &source.0,
        &[],
        specialized.model,
        specialized_artifact,
    )
    .unwrap();
    for (block, overrides, expected) in [
        ("high", "", 2.0),
        ("low", "", 2.0 / 3.0),
        ("high", "N=1", 1.5),
    ] {
        let deck=Netlist::parse(&format!("* retain source assignments\n.options connectrules={block}\n.va \"{}\" top\nX1 p top {overrides}\nRp p 0 1k\n.end\n",source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 0.2e-9, 50e-12).unwrap();
        assert!(
            (voltage(&result, "p", 0.1e-9) - expected).abs() < 1e-7,
            "{block}, {overrides}"
        );
    }
    // Replacing a base registration at the same path cannot reuse the derived
    // runtime belonging to its previous root assignments.
    rspice_core::register_precompiled_veriloga_runtime_with_dependencies(
        &source.0,
        &[],
        original.model,
        original.canonical_ir,
    )
    .unwrap();
    let deck=Netlist::parse(&format!("* replaced source assignment\n.options connectrules=high\n.va \"{}\" top\nX1 p top\nRp p 0 1k\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 0.2e-9, 50e-12).unwrap();
    assert!((voltage(&result, "p", 0.1e-9) - 1.5).abs() < 1e-7);
    let mut tampered: serde_json::Value = serde_json::from_slice(&serialized).unwrap();
    tampered["source_specialization"] = serde_json::json!([]);
    let tampered: rspice_veriloga::canonical_ir::CanonicalIrArtifact =
        serde_json::from_value(tampered).unwrap();
    assert!(tampered.validate().is_err());
}

#[test]
fn sealed_hierarchy_rebinds_selected_libraries_without_filesystem_sources() {
    use rspice_core::{
        ProjectVerilogAConnectionLibraryRegistration, ProjectVerilogARuntimeRegistration,
        ProjectVerilogASourceRegistration, register_project_veriloga_sources_for_session,
    };
    use rspice_veriloga::{
        CompilerOptions, NoPipelineControl, VerilogACompiler, VirtualCompileLimits,
        VirtualSourceBundle, VirtualSourceFile,
    };
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let bundle = VirtualSourceBundle::new(
        "rules.vams",
        [VirtualSourceFile::new("rules.vams", CONFIGURED_LIBRARY)],
    )
    .unwrap();
    let prepared_library = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap();
    let low = prepared_library.connection_configuration("low").unwrap();
    let library = prepared_library.connection_artifact().unwrap();
    let bundle = VirtualSourceBundle::new(
        "top.vams",
        [VirtualSourceFile::new("top.vams", CONFIGURED_DEVICE)],
    )
    .unwrap();
    let mut original = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap()
        .compile_runtime_with_connections("top", &low, &NoPipelineControl)
        .unwrap();
    original.runtime =
        serde_json::from_slice(&serde_json::to_vec(&original.runtime).unwrap()).unwrap();
    original.validate_integrity().unwrap();
    let runtime = compiler
        .specialize_mixed_runtime(
            &original.runtime.canonical_ir,
            &[("N", 2.0)],
            &NoPipelineControl,
        )
        .unwrap();
    let prefix = format!(
        "__rspice_project__/configured-hierarchy-{}",
        std::process::id()
    );
    let device_key = std::path::PathBuf::from(format!("{prefix}/top.vams"));
    let library_key = std::path::PathBuf::from(format!("{prefix}/rules.vams"));
    assert!(!device_key.exists() && !library_key.exists());
    register_project_veriloga_sources_for_session(vec![
        ProjectVerilogASourceRegistration::Runtime(ProjectVerilogARuntimeRegistration {
            source_key: device_key.clone(),
            aliases: vec!["top".into()],
            model: runtime.model,
            canonical_ir: runtime.canonical_ir,
        }),
        ProjectVerilogASourceRegistration::Connections(
            ProjectVerilogAConnectionLibraryRegistration {
                source_key: library_key.clone(),
                aliases: vec!["rules".into()],
                artifact: library,
            },
        ),
    ])
    .unwrap();
    for (block, expected) in [("high", 2.0), ("low", 2.0 / 3.0), ("high", 2.0)] {
        let deck=Netlist::parse(&format!("* sealed hierarchy\n.options connectrules={block} connectrules_source=rules\n.va \"{}\" top\n.va \"{}\" rules\nX1 p top\nRp p 0 1k\n.end\n",device_key.display(),library_key.display())).unwrap();
        let result = Engine::default().run_tran(&deck, 0.2e-9, 50e-12).unwrap();
        assert!(
            (voltage(&result, "p", 0.1e-9) - expected).abs() < 1e-7,
            "{block}"
        );
    }
}


#[test]
fn selected_bus_lanes_preserve_grouping_loading_and_adc_driver_bits() {
    for (mode, expected) in [("merged", 0.5), ("split", 0.75)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module load(a,p);
 input a; electrical a;
 output p; electrical p;
 analog begin
  I(a)<+V(a)/1000;
  I(p)<+(V(p)-V(a))/1000;
 end
endmodule
module stimulus(a);
 output a; electrical a;
 parameter real level=0;
 analog V(a)<+level;
endmodule
module bank(p,q,r,m,s,t);
 parameter integer INDEX=4;
 output p,q,r,m,s,t; electrical p,q,r,m,s,t;
 reg [2:5] bus;
 reg [8:8] single;
 wire [2:5] sampled;
 reg valid;
 initial begin bus=4'b1001; single=1; valid=0; #1 bus=4'b0100; single=0; end
 load first(bus[INDEX],p);
 load second(.a(bus[2:2]),.p(q));
 load third(bus[INDEX+1],r);
 load fourth(single,s);
 load fifth(single[8],t);
 stimulus #(.level(2.0)) high(sampled[INDEX]);
 stimulus #(.level(0.0)) low(sampled[5:5]);
 always @(sampled) valid = (sampled[2] === 1'b1) && (sampled[5] === 1'b0)
                           && (sampled[3] === 1'bz) && (sampled[4] === 1'bz);
 analog V(m)<+(valid ? 1.0 : 0.0);
endmodule
module top(p,q,r,m,s,t);
 inout p,q,r,m,s,t; electrical p,q,r,m,s,t;
 bank #(.INDEX(2)) child(p,q,r,m,s,t);
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectmodule sense(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>1.0;
endmodule
connectrules chosen;
 connect drive {mode};
 connect sense;
endconnectrules
"#
        ));
        let deck=Netlist::parse(&format!("* selected bus lanes\nX1 p q r m s t top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\nRt t 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
        for node in ["p", "q", "s", "t"] {
            assert!(
                (voltage(&result, node, 0.5e-9) - expected).abs() < 1e-7,
                "{mode}, {node}"
            );
            assert!(
                voltage(&result, node, 1.4e-9).abs() < 1e-7,
                "{mode}, {node} after edge"
            );
        }
        assert!(voltage(&result, "r", 0.5e-9).abs() < 1e-7);
        assert!((voltage(&result, "r", 1.4e-9) - 0.75).abs() < 1e-7);
        assert!(
            (voltage(&result, "m", 0.5e-9) - 1.0).abs() < 1e-7,
            "ADC must drive exactly its selected bits"
        );
    }
}

#[test]
fn mixed_connections_read_real_and_packed_elements_of_multidimensional_arrays() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module load(a,p);
 input a; electrical a;
 output p; electrical p;
 analog begin
  I(a)<+V(a)/1000;
  I(p)<+(V(p)-V(a))/1000;
 end
endmodule
module bank(p,q,r);
 parameter integer ROW=5, COLUMN=1;
 output p,q,r; electrical p,q,r;
 real levels[5:4][2:1];
 reg [6:3] words[2:1][0:1];
 initial begin
  levels[5][1]=2.0; levels[4][2]=9.0;
  words[2][1]=4'b1000; words[1][0]=4'b0010;
  #1 levels[5][1]=4.0; words[2][1]=0; words[1][0]=0;
 end
 load first(levels[ROW][COLUMN],p);
 load second(words[2][1][6],q);
 load third(words[1][0][4:4],r);
endmodule
module top(p,q,r);
 inout p,q,r; electrical p,q,r;
 bank child(p,q,r);
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectmodule real_drive(a,r);
 output a; electrical a;
 input r; logic r; wreal r;
 analog I(a)<+(V(a)-r)/1000;
endmodule
connectrules chosen;
 connect drive;
 connect real_drive;
endconnectrules
"#,
    );
    let deck=Netlist::parse(&format!("* selected array elements\nX1 p q r top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
    assert!((voltage(&result, "p", 0.5e-9) - 0.5).abs() < 1e-7);
    assert!((voltage(&result, "p", 1.4e-9) - 1.0).abs() < 1e-7);
    for node in ["q", "r"] {
        assert!(
            (voltage(&result, node, 0.5e-9) - 0.75).abs() < 1e-7,
            "{node}"
        );
        assert!(
            voltage(&result, node, 1.4e-9).abs() < 1e-7,
            "{node} after edge"
        );
    }
}


#[test]
fn whole_physical_vectors_preserve_lane_order_and_mixed_converter_loading() {
    for (mode, high) in [("merged", 0.5), ("split", 0.75)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module sample(a,p);
 input a; logic a;
 output p; electrical p;
 analog V(p)<+(a ? 1.0 : 0.0);
endmodule
module amplifier(a,p);
 parameter integer BASE=6;
 input [BASE:BASE+1] a; electrical [BASE:BASE+1] a;
 output [9:8] p; electrical [9:8] p;
 analog begin
  I(a[BASE])<+V(a[BASE])/1000;
  I(a[BASE+1])<+V(a[BASE+1])/1000;
  I(p[9])<+(V(p[9])-V(a[BASE]))/1000;
  I(p[8])<+(V(p[8])-V(a[BASE+1]))/1000;
 end
endmodule
module bank(x,y);
 input [4:3] x; electrical [4:3] x;
 output [8:9] y; electrical [8:9] y;
 amplifier #(.BASE(2)) nested(.a(x),.p(y));
endmodule
module top(p,q,r,s);
 inout p,q,r,s; electrical p,q,r,s;
 electrical [7:8] sense_input;
 electrical sense_output;
 analog begin V(sense_input[7])<+2; V(sense_input[8])<+0; I(p)<+(1.0-V(sense_output))/1000; end
 sample selected(sense_input[7],sense_output);
 reg [2:3] data;
 initial begin data=2'b10; #1 data=2'b01; end
 bank first(.x(data),.y({{p,q}}));
 bank second(data[2:3],{{r,s}});
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectmodule sense(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>1.0;
endmodule
connectrules selected; connect drive {mode}; connect sense; endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!("* whole vector mixed hierarchy\nX1 p q r s top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
        for (time, first, second) in [(0.5e-9, high, 0.0), (1.5e-9, 0.0, high)] {
            for (node, expected) in [("p", first), ("q", second), ("r", first), ("s", second)] {
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{mode} {node} at {time}: {actual}, expected {expected}"
                );
            }
        }
    }
}

#[test]
fn bit_alias_connections_keep_driver_handoff_and_analog_loading() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module load(a,p);
 inout a; electrical a;
 output p; electrical p;
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module top(p,q,m);
 output p,q,m; electrical p,q,m;
 wire [5:4] bus;
 reg [1:0] value;
 integer code;
 initial begin value=2'b10; #1 value=2'bzz; #1 value=2'b01; #1 value=2'bzz; end
 assign bus=value;
 load first(bus[5],p);
 load second(bus[4],q);
 always @(bus) begin
  if(bus===2'b10) code=1;
  else if(bus===2'b11) code=2;
  else if(bus===2'b0x) code=3;
  else if(bus===2'bzz) code=4;
  else code=9;
 end
 analog V(m)<+code;
endmodule
connectmodule bidirectional(d,a);
 inout d; logic d;
 inout a; electrical a;
 reg drive;
 wire high, low;
 initial begin drive=1'bz; #1 drive=1; #1 drive=0; #1 drive=1'bz; end
 assign d=drive;
 assign high=(d===1'b1);
 assign low=(d===1'b0);
 analog I(a)<+(high*(V(a)-3.0)+low*V(a))/1000;
endmodule
connectrules selected; connect bidirectional; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!("* selected bidirectional lanes\nX1 p q m top\nRp p 0 1k\nRq q 0 1k\nRm m 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 3.8e-9, 50e-12).unwrap();
    let observed: Vec<_> = [0.5e-9, 1.5e-9, 2.5e-9, 3.5e-9]
        .into_iter()
        .map(|time| {
            (
                time,
                voltage(&result, "m", time),
                voltage(&result, "p", time),
                voltage(&result, "q", time),
            )
        })
        .collect();
    for (time, code, p, q) in [
        (0.5e-9, 1.0, 1.5, 0.0),
        (1.5e-9, 2.0, 1.5, 1.5),
        (2.5e-9, 3.0, 0.0, 0.0),
        (3.5e-9, 4.0, 0.0, 0.0),
    ] {
        for (node, expected) in [("m", code), ("p", p), ("q", q)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node} at {time}: {actual}, expected {expected}; full trace {observed:?}"
            );
        }
    }
}

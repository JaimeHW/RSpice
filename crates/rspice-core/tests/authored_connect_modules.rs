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
fn foreign_inserted_connect_hierarchy_uses_each_parent_supply_and_parameters() {
    for (mode, count) in [("merged", 1.0), ("split", 2.0)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module source(output logic q);
 reg q;
 initial begin q=0; #0.2 q=1; end
endmodule
module group(inout electrical p);
 parameter real G=1;
 electrical vdd;
 analog V(vdd)<+$root.top.BASE+G+1e9*$abstime;
 source first(p);
 source second(p);
endmodule
module top(inout electrical p,q);
 parameter real BASE=2, EXTRA=0.5;
 group a(p);
 group #(.G(4)) b(q);
endmodule
module output_stage(input logic d,output electrical a);
 analog I(a)<+(V(a)-(d ? V(group.vdd)+group.G+drive.S+$root.top.EXTRA : 0))/1000;
endmodule
connectmodule drive(input logic d,output electrical a);
 parameter real S=1, R=9;
 output_stage stage(d,a);
 analog I(a)<+V(a)/(1000*drive.R);
endmodule
connectrules selected; connect drive {mode} #(.S(3),.R(1)); endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!(
            "* occurrence-bound inserted helpers\nX1 p q top BASE=2\nX2 r s top BASE=5\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\n.va \"{}\" top module=top\n.end\n", source.path(),
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
        for (node, base, gain) in [
            ("p", 2.0, 1.0),
            ("q", 2.0, 4.0),
            ("r", 5.0, 1.0),
            ("s", 5.0, 4.0),
        ] {
            assert!(
                voltage(&result, node, 0.1e-9).abs() < 1e-7,
                "{mode} {node}: off"
            );
            for time in [0.5e-9, 0.8e-9] {
                let index = result.time.iter().position(|value| *value >= time).unwrap();
                let target = base + 2.0 * gain + 1e9 * result.time[index] + 3.0 + 0.5;
                let expected = count * target / (2.0 * count + 1.0);
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{mode} {node} t={time}: {actual} != {expected}"
                );
            }
        }
    }
}

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

#[test]
fn physical_buses_feed_packed_inputs_with_merged_loading_across_hierarchy() {
    for (mode, peak) in [("merged", 1.0), ("split", 0.75)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module receiver(d,p);
 parameter integer BASE=0;
 input [BASE:BASE+1] d; logic [BASE:BASE+1] d;
 output p; electrical p;
 analog V(p)<+(d[BASE] ? 1.0 : 0.0)+(d[BASE+1] ? 2.0 : 0.0);
endmodule
module bank(a,p,q);
 parameter integer BASE=3;
 input [BASE:BASE-1] a; electrical [BASE:BASE-1] a;
 output p,q; electrical p,q;
 receiver #(.BASE(-2)) first(a,p);
 receiver #(.BASE(10)) second(.d(a[BASE:BASE-1]),.p(q));
endmodule
module top(p,q,r,s,t);
 output p,q,r,s,t; electrical p,q,r,s,t;
 electrical [7:8] v;
 reg phase;
 initial begin phase=0; #1 phase=1; end
 analog begin
  I(v[7])<+(V(v[7])-(phase ? 0.0 : 3.0))/1000;
  I(v[8])<+(V(v[8])-(phase ? 3.0 : 0.0))/1000;
  V(s)<+V(v[7]); V(t)<+V(v[8]);
 end
 bank #(.BASE(5)) nested(.a(v),.p(p),.q(q));
 receiver root_observer({{v[7],v[8]}},r);
endmodule
connectmodule sample(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>0.5;
 analog I(a)<+V(a)/1000;
endmodule
connectrules selected; connect sample {mode}; endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!("* physical buses into packed inputs\nX1 p q r s t top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\nRt t 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
        for (time, code, first, second) in [(0.5e-9, 1.0, peak, 0.0), (1.5e-9, 2.0, 0.0, peak)] {
            for (node, expected) in [
                ("p", code),
                ("q", code),
                ("r", code),
                ("s", first),
                ("t", second),
            ] {
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
fn packed_variable_outputs_drive_whole_physical_buses() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module driver(d);
 output [2:3] d; logic [2:3] d; reg [2:3] d;
 initial begin d=2'b10; #1 d=2'b01; end
endmodule
module top(p,q);
 output p,q; electrical p,q;
 electrical [8:7] v;
 driver source(v);
 analog begin
  I(v[8])<+V(v[8])/1000; I(v[7])<+V(v[7])/1000;
  V(p)<+V(v[8]); V(q)<+V(v[7]);
 end
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectrules selected; connect drive; endconnectrules
"#,
    );
    let deck=Netlist::parse(&format!("* variable packed outputs into physical bus\nX1 p q top\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
    for (time, p, q) in [(0.5e-9, 1.5, 0.0), (1.5e-9, 0.0, 1.5)] {
        for (node, expected) in [("p", p), ("q", q)] {
            assert!(
                (voltage(&result, node, time) - expected).abs() < 1e-7,
                "{node} at {time}"
            );
        }
    }
}

#[test]
fn physical_buses_join_packed_inouts_with_distinct_merged_and_split_drivers() {
    for (mode, initial_voltage, active_voltage, first_code, second_code) in [
        ("merged", 0.0, 1.5, 9.0, 9.0),
        ("split", 1.0, 2.0, 1.0, 2.0),
    ] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module driver(d,p);
 parameter [1:0] PATTERN=2'b10;
 inout [4:5] d; logic [4:5] d;
 output p; electrical p;
 reg [1:0] drive;
 integer code;
 initial begin drive=PATTERN; #1 drive=2'bzz; #1 drive=PATTERN; #1 drive=2'bzz; end
 assign d=drive;
 always @(d) begin
  if(d===2'b10) code=1;
  else if(d===2'b01) code=2;
  else if(d===2'b11) code=3;
  else if(d===2'bzz) code=4;
  else code=9;
 end
 analog V(p)<+code;
endmodule
module top(p,q,s,t);
 output p,q,s,t; electrical p,q,s,t;
 electrical [7:8] v;
 driver #(.PATTERN(2'b10)) first(v,p);
 driver #(.PATTERN(2'b01)) second(v[7:8],q);
 analog begin
  I(v[7])<+V(v[7])/1000; I(v[8])<+V(v[8])/1000;
  V(s)<+V(v[7]); V(t)<+V(v[8]);
 end
endmodule
connectmodule bidirectional(d,a);
 inout d; logic d;
 inout a; electrical a;
 reg drive;
 wire high, low;
 initial begin drive=1'bz; #1 drive=1; #1 drive=0; #1 drive=1'bz; end
 assign d=drive;
 assign high=(d===1'b1); assign low=(d===1'b0);
 analog I(a)<+(high*(V(a)-3.0)+low*V(a))/1000;
endmodule
connectrules selected; connect bidirectional {mode}; endconnectrules
"#
        ));
        let deck=Netlist::parse(&format!("* physical bus with packed inout drivers\nX1 p q s t top\nRp p 0 1k\nRq q 0 1k\nRs s 0 1k\nRt t 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 3.8e-9, 50e-12).unwrap();
        for (time, p, q, physical) in [
            (0.5e-9, first_code, second_code, initial_voltage),
            (1.5e-9, 3.0, 3.0, active_voltage),
            (2.5e-9, 9.0, 9.0, 0.0),
            (3.5e-9, 4.0, 4.0, 0.0),
        ] {
            for (node, expected) in [("p", p), ("q", q), ("s", physical), ("t", physical)] {
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
fn packed_array_words_drive_physical_vectors_with_shared_lane_loading() {
    for (mode, high) in [("merged", 0.5), ("split", 0.75)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module amplifier(a,p);
 input [4:3] a; electrical [4:3] a;
 output [7:8] p; electrical [7:8] p;
 analog begin
  I(a[4])<+V(a[4])/1000; I(a[3])<+V(a[3])/1000;
  I(p[7])<+(V(p[7])-V(a[4]))/1000;
  I(p[8])<+(V(p[8])-V(a[3]))/1000;
 end
endmodule
module bank(p,q,r,s,t,u,v,w);
 parameter integer ROW=2, COLUMN=-1;
 output p,q,r,s,t,u,v,w; electrical p,q,r,s,t,u,v,w;
 reg [-2:-1] words[3:2][-1:0];
 reg [5:6] singles[-3:-2];
 initial begin
  words[3][0]=2'b10; words[2][-1]=2'b01; singles[-3]=2'b01;
  #1 words[3][0]=2'b01; words[2][-1]=2'b10; singles[-3]=2'b10;
 end
 amplifier first(words[ROW][COLUMN],{{p,q}});
 amplifier second(words[ROW][COLUMN][-2:-1],{{r,s}});
 amplifier third(words[2][-1],{{t,u}});
 amplifier fourth(singles[-3],{{v,w}});
endmodule
module top(p,q,r,s,t,u,v,w);
 output p,q,r,s,t,u,v,w; electrical p,q,r,s,t,u,v,w;
 bank #(.ROW(3),.COLUMN(0)) child(p,q,r,s,t,u,v,w);
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectrules selected; connect drive {mode}; endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!("* packed array words into physical buses\nX1 p q r s t u v w top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\nRt t 0 1k\nRu u 0 1k\nRv v 0 1k\nRw w 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
        for (time, first, second, other_first, other_second) in [
            (0.5e-9, high, 0.0, 0.0, 0.75),
            (1.5e-9, 0.0, high, 0.75, 0.0),
        ] {
            for (node, expected) in [
                ("p", first),
                ("q", second),
                ("r", first),
                ("s", second),
                ("t", other_first),
                ("u", other_second),
                ("v", other_first),
                ("w", other_second),
            ] {
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
fn mixed_concatenated_inputs_preserve_array_lanes_and_converter_loading() {
    for (mode, loaded) in [("merged", 1.5), ("split", 1.0)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module receiver(d,p);
 input [-2:1] d; logic [-2:1] d;
 output p; electrical p;
 analog V(p)<+(d[-2] ? 8.0 : 0.0)+(d[-1] ? 4.0 : 0.0)+(d[0] ? 2.0 : 0.0)+(d[1] ? 1.0 : 0.0);
endmodule
module bank(a,p,q);
 input a; electrical a;
 output p,q; electrical p,q;
 reg [5:6] words[3:2];
 reg flag;
 wire bit_value;
 assign bit_value=flag;
 initial begin words[3]=2'b01; flag=1; #1 words[3]=2'b10; flag=0; end
 receiver first({{a,words[3],bit_value}},p);
 receiver second({{a,{{1{{words[3][5:6]}}}},bit_value}},q);
endmodule
module top(p,q,r);
 output p,q,r; electrical p,q,r;
 electrical a;
 analog begin I(a)<+(V(a)-3.0)/1000; V(r)<+V(a); end
 bank child(a,p,q);
endmodule
connectmodule sample(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>0.5;
 analog I(a)<+V(a)/1000;
endmodule
connectrules selected; connect sample {mode}; endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!("* mixed physical/digital input concatenation\nX1 p q r top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
        let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
        for (time, code) in [(0.5e-9, 11.0), (1.5e-9, 12.0)] {
            for (node, expected) in [("p", code), ("q", code), ("r", loaded)] {
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
fn packed_outputs_drive_mixed_physical_and_digital_concatenations() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module driver(d);
 output [4:1] d; logic [4:1] d; reg [4:1] d;
 initial begin d=4'b1010; #1 d=4'b0101; end
endmodule
module top(p,q);
 output p,q; electrical p,q;
 wire [6:7] bus;
 wire flag;
 driver child({q,bus[6:7],flag});
 analog V(p)<+(bus[6] ? 4.0 : 0.0)+(bus[7] ? 2.0 : 0.0)+(flag ? 1.0 : 0.0);
endmodule
connectmodule drive(d,a);
 input d; logic d;
 output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectrules selected; connect drive; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!("* mixed physical/digital output concatenation\nX1 p q top\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
    for (time, code, loaded) in [(0.5e-9, 2.0, 1.5), (1.5e-9, 5.0, 0.0)] {
        for (node, expected) in [("p", code), ("q", loaded)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node} at {time}: {actual}, expected {expected}"
            );
        }
    }
}

#[test]
fn mixed_concatenated_inouts_keep_independent_drivers_and_release() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module driver(d);
 inout [-1:0] d; logic [-1:0] d;
 reg [1:0] drive;
 initial begin drive=2'b10; #1 drive=2'bzz; #1 drive=2'b01; #1 drive=2'bzz; end
 assign d=drive;
endmodule
module bank(p,m);
 inout p; electrical p;
 output m; electrical m;
 wire [5:6] bus;
 reg [1:0] external;
 integer code;
 initial begin external=2'bzz; #1 external=2'b1z; #1 external=2'b0z; #1 external=2'bzz; end
 assign bus=external;
 driver child({p,bus[5]});
 always @(bus) begin
  if(bus===2'b0z) code=1;
  else if(bus===2'b1z) code=2;
  else if(bus===2'bxz) code=3;
  else if(bus===2'bzz) code=4;
  else code=9;
 end
 analog V(m)<+code;
endmodule
module top(p,m);
 inout p; electrical p;
 output m; electrical m;
 bank child(p,m);
endmodule
connectmodule bidirectional(d,a);
 inout d; logic d;
 inout a; electrical a;
 wire high, low;
 assign d=1'bz;
 assign high=(d===1'b1); assign low=(d===1'b0);
 analog I(a)<+(high*(V(a)-3.0)+low*V(a))/1000;
endmodule
connectrules selected; connect bidirectional; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!("* mixed physical/digital inout concatenation\nX1 p m top\nRp p 0 1k\nRm m 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 3.8e-9, 50e-12).unwrap();
    for (time, code, loaded) in [
        (0.5e-9, 1.0, 1.5),
        (1.5e-9, 2.0, 0.0),
        (2.5e-9, 3.0, 0.0),
        (3.5e-9, 4.0, 0.0),
    ] {
        for (node, expected) in [("m", code), ("p", loaded)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node} at {time}: {actual}, expected {expected}"
            );
        }
    }
}

#[test]
fn computed_inputs_keep_parent_types_arrays_parameters_and_time_scope() {
    let source = Source::new(
        r#"
`timescale 10ps/1ps
module receiver(bits,sample,p);
 parameter real K=99;
 input signed [7:0] bits; wire signed [7:0] bits;
 input sample; wreal sample;
 output p; electrical p;
 wire negative;
 assign negative=(bits<0) && bits[7];
 // Table 7-1 zero-extends a packed read in analog; decode its sign digitally.
 analog V(p)<+bits-(negative ? 256.0 : 0.0)+sample;
endmodule
module decode(d,p);
 input [1:0] d; wire [1:0] d;
 output p; electrical p;
 wire expected;
 assign expected=(d===2'bxz);
 analog V(p)<+(expected ? 7.0 : 9.0);
endmodule
`timescale 1ns/1ps
module bank(p,q,r,s,t,u);
 parameter real K=2;
 output p,q,r,s,t,u; electrical p,q,r,s,t,u;
 reg [3:0] values[2:1];
 reg [8:15] same_width;
 integer pick;
 real level;
 initial begin
  pick=2; values[2]=4'd15; values[1]=4'd2; level=1.5; same_width=8'hff;
  #1 pick=1; level=2.5; same_width=8'h80;
 end
 receiver arithmetic(values[pick]+4'd1,level+K+$realtime,p);
 receiver constant(8'shff,0.25,q);
 receiver concatenated({values[pick][3:2],2'b01,4'b0011},0.5,r);
 receiver resized(values[pick],-0.5,s);
 decode unknown(2'bxz,t);
 receiver converted_type(same_width,0.0,u);
endmodule
module top(p,q,r,s,t,u);
 output p,q,r,s,t,u; electrical p,q,r,s,t,u;
 bank #(.K(3)) child(p,q,r,s,t,u);
endmodule
"#,
    );
    let deck = Netlist::parse(&format!("* parent-scoped digital input expressions\nX1 p q r s t u top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\nRt t 0 1k\nRu u 0 1k\n.va \"{}\" top module=top\n.end\n", source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
    for (time, p, r, s, u) in [
        (0.5e-9, 20.5, -44.5, 14.5, -1.0),
        (1.5e-9, 9.5, 19.5, 1.5, -128.0),
    ] {
        for (node, expected) in [
            ("p", p),
            ("q", -0.75),
            ("r", r),
            ("s", s),
            ("t", 7.0),
            ("u", u),
        ] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node} at {time}: {actual}, expected {expected}"
            );
        }
    }
}


#[test]
fn computed_mixed_inputs_preserve_expression_widths_replication_and_loading() {
    for (mode, first_load, repeated_load) in [("merged", 1.5, 1.5), ("split", 1.0, 0.6)] {
        let source = Source::new(&format!(
            r#"
`timescale 1ns/1ps
module receiver(d,p);
 input [13:0] d; logic [13:0] d;
 output p; electrical p;
 wire [11:0] clean;
 wire valid;
 assign clean={{d[13:9],d[6:0]}};
 assign valid=(d[8:7]===2'bxz);
 analog V(p)<+(valid ? clean : -1.0);
endmodule
module bank(a,p,q);
 parameter integer BASE=7;
 input [BASE:BASE+1] a; electrical [BASE:BASE+1] a;
 output p,q; electrical p,q;
 reg [3:0] words[3:2][-1:0];
 reg [5:4] flags;
 integer row,column,index;
 initial begin
  words[3][0]=4'd15; words[2][-1]=4'd2; flags=2'b10;
  row=3; column=0; index=5;
  #1 row=2; column=-1; index=4;
 end
 receiver first({{a[BASE],words[row][column]+4'd1,2'bxz,
                  {{2{{a[BASE+1],flags[index]^1'b1}}}},
                  flags[index] ? 3'b101 : 3'b010}},p);
 receiver second(.p(q),.d({{a[BASE],words[row][column]+4'd1,2'bxz,
                           {{2{{a[BASE+1],~flags[index]}}}},
                           flags[index] ? 3'b101 : 3'b010}}));
endmodule
module top(p,q,r,s);
 output p,q,r,s; electrical p,q,r,s;
 electrical [9:8] a;
 bank #(.BASE(2)) child(a,p,q);
 analog begin
  I(a[9])<+(V(a[9])-3.0)/1000; I(a[8])<+(V(a[8])-3.0)/1000;
  V(r)<+V(a[9]); V(s)<+V(a[8]);
 end
endmodule
connectmodule sample(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>0.5;
 analog I(a)<+V(a)/1000;
endmodule
connectrules selected; connect sample {mode}; endconnectrules
"#
        ));
        let deck = Netlist::parse(&format!(
            "* computed mixed input connections\nX1 p q r s top\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\n.va \"{}\" top module=top\n.end\n",
            source.path()
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
        for (time, code) in [(0.5e-9, 2133.0), (1.5e-9, 2554.0)] {
            for (node, expected) in [
                ("p", code),
                ("q", code),
                ("r", first_load),
                ("s", repeated_load),
            ] {
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
fn vector_branch_and_port_currents_execute_through_tied_hierarchy() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
nature Heat; access=Temp; units="K"; abstol=1e-6; endnature
nature Flux; access=Pwr; units="W"; abstol=1e-12; endnature
discipline thermal; potential Heat; flow Flux; enddiscipline
module electrical_load(p,n,a,b,c,d);
 inout [3:2] p; electrical [3:2] p;
 inout n,a,b,c,d; electrical n,a,b,c,d;
 branch(p,n) load[8:9], extra[1:0];
 branch(p[3:2],n) shunt;
 branch(p[3],0) ground_load;
 branch(<p>) port_probe[-2:-1];
 genvar k;
 real sample0,sample1;
 analog begin
  for(k=0;k<2;k=k+1) begin
   I(load[8+k])<+V(load[8+k])/(1000*(k+1));
   I(extra[1-k])<+V(extra[1-k])/4000;
   I(shunt[k])<+V(shunt[k])/4000;
  end
  I(ground_load)<+V(ground_load)/2000;
  V(a)<+1000*I(port_probe[-2]);
  V(b)<+1000*I(<p[2]>);
  V(c)<+sample0+10*sample1;
  V(d)<+1000*(I(load[8])+I(<shunt[1]>)+I(ground_load));
 end
 always #0.2 begin
  sample0=1000*I(<p[3]>); sample1=1000*I(port_probe[-1]);
 end
endmodule
module thermal_load(t,a,b);
 inout [4:5] t; thermal [4:5] t;
 inout a,b; electrical a,b;
 branch(t) loss[9:8];
 branch(<t>) port_probe;
 real sample;
 analog begin
  Pwr(loss[9])<+Temp(loss[9])/1000;
  Pwr(loss[8])<+Temp(loss[8])/2000;
  V(a)<+1000*(Pwr(port_probe[0])+Pwr(<t[5]>));
  V(b)<+sample;
 end
 always #0.2 sample=1000*(Pwr(<t[4]>)+Pwr(port_probe[1]));
endmodule
module middle(p0,p1,a,b,c,d,e,f);
 inout p0,p1,a,b,c,d,e,f; electrical p0,p1,a,b,c,d,e,f;
 thermal [4:5] t;
 electrical_load nested({p0,p1},0,a,b,c,d);
 thermal_load heater(t,e,f);
 analog begin Temp(t[4])<+2; Temp(t[5])<+3; end
endmodule
module top(p0,p1,a,b,c,d,e,f);
 inout p0,p1,a,b,c,d,e,f; electrical p0,p1,a,b,c,d,e,f;
 middle parent(p0,p1,a,b,c,d,e,f);
endmodule
"#,
    );
    for tied in [false, true] {
        let second = if tied { "p0" } else { "p1" };
        let deck = format!(
            "* vector branch/port flows
.va \"{}\" top module=top
X1 p0 {second} a b c d e f top
V0 p0 0 PWL(0 2 1n 2 1.1n 4 3n 4)
V1 p1 0 3
Ra a 0 1k
Rb b 0 1k
Rc c 0 1k
Rd d 0 1k
Re e 0 1k
Rf f 0 1k
.end
",
            source.path()
        );
        let netlist = Netlist::parse(&deck).unwrap();
        let result = Engine::default().run_tran(&netlist, 3e-9, 0.05e-9).unwrap();
        for (time, p0) in [(0.6e-9, 2.0), (2.1e-9, 4.0)] {
            let p1 = if tied { p0 } else { 3.0 };
            for (node, expected) in [
                ("a", 2.0 * p0),
                ("b", p1),
                ("c", 2.0 * p0 + 10.0 * p1),
                ("d", 1.5 * p0 + 0.25 * p1),
                ("e", 3.5),
                ("f", 3.5),
            ] {
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "tied={tied}, {node}@{time}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn wire_array_words_preserve_hierarchical_drivers_and_dynamic_reads() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module observer(d,p);
 input [9:8] d; wire [9:8] d;
 output p; electrical p;
 wire [3:0] code;
 assign code=(d===2'b01) ? 1 : (d===2'b1x) ? 2 : (d===2'bzz) ? 3 : (d===2'b10) ? 4 : 9;
 analog V(p)<+code;
endmodule
module driver(p);
 inout [0:1] p; wire [0:1] p;
 reg [1:0] word;
 initial begin word=2'b01; #1 word=2'b10; #1 word=2'bzz; end
 assign p=word;
endmodule
module bank(p,q,r);
 parameter integer BASE=3;
 output p,q,r; electrical p,q,r;
 wire [5:4] cells[BASE:BASE-1][-1:0];
 reg [1:0] data; reg enable;
 integer row,column;
 initial begin
  enable=1; data=2'b01; row=BASE; column=-1;
  #1 data=2'b11; row=BASE-1;
  #1 enable=0; row=BASE;
  #1 enable=1; data=2'b10; row=BASE-1; column=0;
 end
 assign cells[BASE][-1]=enable ? data : 2'bzz;
 assign {cells[BASE-1][-1][5:4],cells[BASE][0]}=4'b1001;
 assign cells[BASE-1][0][5]=1'b0;
 assign cells[BASE-1][0][4]=1'b1;
 driver nested(cells[BASE][-1]);
 observer whole(cells[BASE][-1],p);
 observer bits({cells[BASE][-1][5],cells[BASE][-1][4]},q);
 observer dynamic(cells[row][column],r);
endmodule
module top(p,q,r);
 output p,q,r; electrical p,q,r;
 bank #(.BASE(7)) group(p,q,r);
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* net array resolution
X1 p q r top
Rp p 0 1k
Rq q 0 1k
Rr r 0 1k
.va \"{}\" top module=top
.end
",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 3.8e-9, 50e-12).unwrap();
    for (time, code, dynamic) in [
        (0.5e-9, 1.0, 1.0),
        (1.5e-9, 2.0, 4.0),
        (2.5e-9, 3.0, 3.0),
        (3.5e-9, 4.0, 1.0),
    ] {
        for (node, expected) in [("p", code), ("q", code), ("r", dynamic)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn wire_array_elements_connect_both_directions_through_analog_buses() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module receiver(a,p,q);
 input [9:8] a; electrical [9:8] a;
 output p,q; electrical p,q;
 analog begin
  I(a[9])<+V(a[9])/1000; I(a[8])<+V(a[8])/1000;
  V(p)<+V(a[9]); V(q)<+V(a[8]);
 end
endmodule
module producer(a);
 output [9:8] a; electrical [9:8] a;
 analog begin V(a[9])<+0.3; V(a[8])<+1.7; end
endmodule
module bank(p,q,r);
 parameter integer BASE=-2;
 output p,q,r; electrical p,q,r;
 wire [2:3] words[BASE:BASE+1];
 reg [1:0] data;
 initial begin data=2'b10; #1 data=2'b01; end
 assign words[BASE]=data;
 receiver load(words[BASE],p,q);
 producer source(words[BASE+1]);
 analog V(r)<+words[BASE+1];
endmodule
module top(p,q,r);
 output p,q,r; electrical p,q,r;
 bank #(.BASE(4)) nested(p,q,r);
endmodule
connectmodule dac(d,a);
 input d; logic d; output a; electrical a;
 analog I(a)<+(V(a)-(d ? 3.0 : 0.0))/1000;
endmodule
connectmodule adc(a,d);
 input a; electrical a; output d; logic d; reg d;
 initial d=0;
 always #0.1 d=V(a)>1.0;
endmodule
connectrules selected; connect dac merged; connect adc merged; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* net array mixed connections
X1 p q r top
Rp p 0 1k
Rq q 0 1k
Rr r 0 1k
.va \"{}\" top module=top
.end
",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.8e-9, 50e-12).unwrap();
    for (time, p, q) in [(0.5e-9, 1.5, 0.0), (1.5e-9, 0.0, 1.5)] {
        for (node, expected) in [("p", p), ("q", q), ("r", 1.0)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}


#[test]
fn real_net_array_elements_share_resolved_drivers_through_hierarchy() {
    let model = r#"
`timescale 1ns/1ps
module driver(q,p);
 parameter real DRIVE=2;
 inout q; wrealsum q;
 output p; electrical p;
 assign q=DRIVE;
 analog V(p)<+q;
endmodule
module producer(q);
 output real q;
 initial begin q=6; #1 q=10; end
endmodule
module observer(q,p);
 input q; wrealsum q;
 output p; electrical p;
 analog V(p)<+q;
endmodule
module bank(p,q,r,s,t);
 parameter integer BASE=-2;
 output p,q,r,s,t; electrical p,q,r,s,t;
 wrealsum cells[BASE:BASE-1][-1:0];
 real bias=1;
 integer row=BASE-1, column=-1;
 initial begin #1 bias=2; #1 row=BASE; column=0; end
 assign cells[BASE-1][-1]=bias;
 assign cells[BASE-1][0]=0.25;
 driver #(.DRIVE(2)) first(cells[BASE-1][-1],p);
 driver #(.DRIVE(4)) second(cells[BASE-1][-1],q);
 producer source(cells[BASE][0]);
 observer selected(cells[row][column],r);
 observer fixed(cells[BASE-1][-1],s);
 analog V(t)<+cells[BASE][0];
endmodule
module top(p,q,r,s,t);
 output p,q,r,s,t; electrical p,q,r,s,t;
 bank #(.BASE(4)) nested(p,q,r,s,t);
endmodule
"#;
    for (kind, before, after) in [
        ("wrealsum", 7.0, 8.0),
        ("wrealavg", 7.0 / 3.0, 8.0 / 3.0),
        ("wrealmin", 1.0, 2.0),
        ("wrealmax", 4.0, 4.0),
        ("wreal", 0.0, 0.0),
    ] {
        let source = Source::new(&model.replace("wrealsum", kind));
        let deck = Netlist::parse(&format!(
            "* real array identities\nX1 p q r s t top\n.va \"{}\" top module=top\n.end\n",
            source.path()
        ))
        .unwrap();
        let outcome = Engine::default().run_tran(&deck, 2.5e-9, 50e-12);
        if kind == "wreal" {
            let error = outcome.unwrap_err().to_string();
            assert!(
                error.contains("driver") && error.contains("resolution"),
                "{error}"
            );
            continue;
        }
        let result = outcome.unwrap();
        for (time, resolved, produced) in [
            (0.5e-9, before, 6.0),
            (1.5e-9, after, 10.0),
            (2.4e-9, after, 10.0),
        ] {
            for node in ["p", "q", "s"] {
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - resolved).abs() < 1e-7,
                    "{kind} {node}@{time}: {actual} != {resolved}"
                );
            }
            let expected = if time > 2e-9 { produced } else { resolved };
            assert!(
                (voltage(&result, "r", time) - expected).abs() < 1e-7,
                "{kind} dynamic@{time}"
            );
            assert!(
                (voltage(&result, "t", time) - produced).abs() < 1e-7,
                "{kind} output@{time}"
            );
        }
    }
}

#[test]
fn real_net_array_inout_converters_preserve_feedback_and_physical_loading() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module load(a,p);
 inout a; electrical a;
 output p; electrical p;
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module bank(p,q);
 parameter integer BASE=-2;
 output p,q; electrical p,q;
 logic cells; wrealsum cells[BASE:BASE+1];
 real level=1.5;
 initial #1000 level=3.0;
 assign cells[BASE]=level;
 load receiver(cells[BASE],p);
 analog V(q)<+cells[BASE];
endmodule
module top(p,q);
 output p,q; electrical p,q;
 bank #(.BASE(3)) nested(p,q);
endmodule
connectmodule feedback(a,r);
 inout a; electrical a;
 inout r; logic r; wrealsum r;
 real sample=0, returned=0;
 always begin #100 sample=0.25*V(a); #10 returned=sample; end
 assign r=returned;
 analog I(a)<+(V(a)-r)/500;
endmodule
connectrules chosen; connect feedback merged; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* resolved array feedback\nX1 p q top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.8e-9, 25e-12).unwrap();
    // The selected real net sums the source and sampled return driver. The
    // converter's 500 ohms and the child's 1k load produce V = resolved / 1.5.
    for (time, physical) in [(50e-12, 1.0), (150e-12, 7.0 / 6.0), (250e-12, 43.0 / 36.0)] {
        assert!(
            (voltage(&result, "p", time) - physical).abs() < 1e-7,
            "physical@{time}"
        );
        assert!(
            (voltage(&result, "q", time) - 1.5 * physical).abs() < 1e-7,
            "resolved@{time}"
        );
    }
    assert!((voltage(&result, "p", 1.75e-9) - 2.4).abs() < 1e-5);
}


#[test]
fn inline_real_net_disciplines_select_the_correct_loaded_converters() {
    let model = r#"
`timescale 1ns/1ps
discipline low; domain discrete; potential Voltage; enddiscipline
discipline high; domain discrete; potential Temperature; enddiscipline
module receiver(input electrical a, output electrical p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module bank(p,q,r);
 parameter integer BASE=-2;
 output p,q,r; electrical p,q,r;
 wreal low cells[BASE:BASE+1];
 wreal high upper;
 real level=2;
 initial #1 level=4;
 assign cells[BASE]=level;
 assign upper=level;
 receiver first(cells[BASE],p);
 receiver second(upper,q);
 analog V(r)<+cells[BASE]+upper;
endmodule
module top(p,q,r);
 output p,q,r; electrical p,q,r;
 bank #(.BASE(3)) nested(p,q,r);
endmodule
connectmodule low_gain(input low wreal value, output electrical a);
 analog I(a)<+(V(a)-2*value)/1000;
endmodule
connectmodule high_gain(value,a);
 input high wreal value;
 output a; electrical a;
 analog I(a)<+(V(a)-3*value)/1000;
endmodule
connectrules chosen; connect low_gain; connect high_gain; endconnectrules
"#;
    for ambiguous in [false, true] {
        let source = Source::new(&if ambiguous {
            model.replace("potential Temperature", "potential Voltage")
        } else {
            model.to_owned()
        });
        let deck = Netlist::parse(&format!(
            "* explicit real-net disciplines\nX1 p q r top\n.va \"{}\" top module=top\n.end\n",
            source.path()
        ))
        .unwrap();
        let outcome = Engine::default().run_tran(&deck, 1.5e-9, 50e-12);
        if ambiguous {
            let error = outcome.unwrap_err().to_string();
            assert!(error.contains("match both connect module"), "{error}");
            continue;
        }
        let result = outcome.unwrap();
        for (time, level) in [(0.5e-9, 2.0), (1.4e-9, 4.0)] {
            for (node, expected) in [("p", level), ("q", 1.5 * level), ("r", 2.0 * level)] {
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{node}@{time}: {actual} != {expected}"
                );
            }
        }
    }
}


#[test]
fn real_bus_ranges_resolve_original_drivers_under_loaded_analog_connections() {
    let model = r#"
`timescale 1ns/1ps
module leaf(inout logic KIND [5:4] r);
 assign r[5]=4.0; assign r[4]=6.0;
endmodule
module middle(inout logic KIND [-1:0] q);
 leaf inner(q);
endmodule
module load(input electrical [1:0] a, output electrical p,q);
 analog begin
  I(a[1])<+V(a[1])/1000; I(a[0])<+V(a[0])/2000;
  V(p)<+V(a[1]); V(q)<+V(a[0]);
 end
endmodule
module top(p,q,r);
 output p,q,r; electrical p,q,r;
 KIND logic [3:4] bus;
 real level=1.0, sampled=0.0;
 integer index=3;
 initial #1 level=3.0;
 initial begin #0.4 sampled=bus[index]; #0.7 index=4; sampled=bus[index]; end
 assign bus[3]=level; assign bus[4]=2.0*level;
 middle nested(bus);
 load receiver(bus,p,q);
 analog V(r)<+sampled;
endmodule
connectmodule dac(input logic KIND value, output electrical a);
 analog I(a)<+(V(a)-value)/1000;
endmodule
connectrules chosen; connect dac; endconnectrules
"#;
    for (kind, before, after) in [
        ("wrealsum", [5.0, 8.0], [7.0, 12.0]),
        ("wrealavg", [2.5, 4.0], [3.5, 6.0]),
        ("wrealmin", [1.0, 2.0], [3.0, 6.0]),
        ("wrealmax", [4.0, 6.0], [4.0, 6.0]),
    ] {
        let source = Source::new(&model.replace("KIND", kind));
        let deck = Netlist::parse(&format!(
            "* real bus driver identity\nX1 p q r top\n.va \"{}\" top module=top\n.end\n",
            source.path()
        ))
        .unwrap();
        let result = Engine::default().run_tran(&deck, 1.5e-9, 25e-12).unwrap();
        for (time, values, sample) in [(0.5e-9, before, before[0]), (1.4e-9, after, after[1])] {
            for (node, expected) in [
                ("p", values[0] / 2.0),
                ("q", values[1] * 2.0 / 3.0),
                ("r", sample),
            ] {
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{kind} {node}@{time}: {actual} != {expected}"
                );
            }
        }
    }
}


#[test]
fn multidimensional_physical_arrays_share_loaded_branches_and_sampled_values() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module word(output logic [1:0] q);
 reg [1:0] q;
 initial begin q=2'b10; #1 q=2'b01; end
endmodule
module load(inout electrical [8:9] x);
 analog begin I(x[8])<+V(x[8])/1000; I(x[9])<+V(x[9])/2000; end
endmodule
module top(p,q,r,s);
 output p,q,r,s; electrical p,q,r,s;
 electrical [3:2] a[-2:-1][5:5],g[-2:-1][5:5];
 ground [3:2] g[-2:-1][5:5];
 branch(a[-1][5],g[-1][5]) leg[0:1];
 word source(a[-1][5]);
 load active(a[-1][5]);
 real sampled=0;
 always begin #0.2 sampled=V(a[-1][5][3]); end
 analog begin
  V(a[-2][5][3])<+0; V(a[-2][5][2])<+0;
  I(leg[0])<+V(leg[0])/1000;
  I(leg[1])<+V(leg[1])/2000;
  V(p)<+V(a[-1][5][3]);
  V(q)<+V(a[-1][5][2]);
  V(r)<+sampled;
  V(s)<+1000*I(leg[0]);
 end
endmodule
connectmodule dac(input logic d,output electrical a);
 analog I(a)<+(V(a)-3*d)/1000;
endmodule
connectrules chosen; connect dac; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* multidimensional physical connectivity\nX1 p q r s top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 25e-12).unwrap();
    // Each selected physical node has both a child load and a branch load.
    // The 1k source therefore produces 1V into two 1k loads, or 1.5V into two 2k loads.
    for (time, expected) in [
        (0.5e-9, [1.0, 0.0, 1.0, 1.0]),
        (1.4e-9, [0.0, 1.5, 0.0, 0.0]),
    ] {
        for (node, expected) in ["p", "q", "r", "s"].into_iter().zip(expected) {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}


#[test]
fn promoted_wire_hierarchies_keep_real_values_and_select_loaded_real_converters() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module source(output wreal value);
 real level=2.75;
 initial #1 level=4.25;
 assign value=level;
endmodule
module bridge(output tri value); source nested(value); endmodule
module tap(inout wire value,output electrical p); analog V(p)<+value; endmodule
module load(input electrical a,output electrical p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module helper(input wire value,output electrical a);
 analog I(a)<+(V(a)-value)/1000;
endmodule
module bus_source(output logic wrealsum [2:3] values);
 assign values[2]=1.25; assign values[3]=2.5;
endmodule
module bus_tap(inout tri [5:4] values,output electrical p);
 analog V(p)<+10*values[5]+values[4];
endmodule
module top(p,q,r,s,t);
 output p,q,r,s,t; electrical p,q,r,s,t;
 wire [8:9] values;
 bus_source bus_producer(values);
 bus_tap bus_consumer(values,s);
 tap selected_lane(values[9],t);
 // Keep the RNM interconnect discrete across the analog load boundary.
 tri logic real_path; tri logic_path;
 bridge producer(real_path);
 assign logic_path=1'b1;
 tap real_use(real_path,p);
 tap logic_use(logic_path,q);
 load loaded(real_path,r);
endmodule
connectmodule dac(input logic wreal value,output electrical a);
 helper body(value,a);
endmodule
connectrules chosen; connect dac; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* promoted real interconnect\nX1 p q r s t top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 25e-12).unwrap();
    for (time, level) in [(0.5e-9, 2.75), (1.4e-9, 4.25)] {
        for (node, expected) in [
            ("p", level),
            ("q", 1.0),
            ("r", level / 2.0),
            ("s", 15.0),
            ("t", 2.5),
        ] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}


#[test]
fn real_concatenations_preserve_loaded_lanes_and_repeated_input_driver_identity() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module leaf(inout logic wrealsum [1:0] value);
 assign value[1]=2.5; assign value[0]=4.75;
endmodule
module monitor(input tri [5:4] value,output electrical p);
 analog V(p)<+10*value[5]+value[4];
endmodule
module load(input electrical [0:1] a,output electrical p,q);
 analog begin
  I(a[0])<+V(a[0])/1000; I(a[1])<+V(a[1])/2000;
  V(p)<+V(a[0]); V(q)<+V(a[1]);
 end
endmodule
module top(p,q,r,s);
 output p,q,r,s; electrical p,q,r,s;
 wire a,b;
 real level=0.25;
 initial #1 level=0.75;
 assign a=level; assign b=0.5;
 leaf drive({a,{b}});
 monitor readback({a,b},r);
 monitor repeated({2{a}},s);
 load loaded({a,b},p,q);
endmodule
connectmodule dac(input logic wrealsum value,output electrical a);
 analog I(a)<+(V(a)-value)/1000;
endmodule
connectrules chosen; connect dac; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* real concatenated interconnect\nX1 p q r s top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 25e-12).unwrap();
    for (time, a) in [(0.5e-9, 2.75), (1.4e-9, 3.25)] {
        for (node, expected) in [
            ("p", a / 2.0),
            ("q", 3.5),
            ("r", 10.0 * a + 5.25),
            ("s", 11.0 * a),
        ] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn compatible_disciplines_preserve_loaded_connect_body_physics() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
nature LocalVoltage; units="V"; access=U; abstol=1e-8; endnature
nature LocalCurrent; units="A"; access=J; abstol=1e-14; endnature
discipline local_electrical; potential LocalVoltage; flow LocalCurrent; enddiscipline
discipline drive_data; domain discrete; potential Voltage; enddiscipline
discipline receive_data; domain discrete; potential LocalVoltage; enddiscipline
module stimulus(output drive_data wreal q);
 real level=1.25;
 initial #1 level=2.75;
 assign q=level;
endmodule
module stage(input drive_data wreal d, output local_electrical p);
 analog J(p)<+(U(p)-d)/1000;
endmodule
connectmodule drive(input receive_data wreal d, output electrical p);
 stage nested(d,p);
endmodule
module load(input local_electrical a, output electrical p);
 analog begin J(a)<+U(a)/1000; V(p)<+U(a); end
endmodule
module top(output electrical p);
 wreal receive_data value;
 stimulus source(value);
 load receiver(value,p);
endmodule
connectrules chosen; connect drive; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* compatible mixed disciplines\nX1 p top\n.va \"{}\" top module=top\n.end\n",
        source.path(),
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
    for (time, expected) in [(0.5e-9, 0.625), (1.4e-9, 1.375)] {
        let actual = voltage(&result, "p", time);
        assert!(
            (actual - expected).abs() < 1e-7,
            "{time}: {actual} != {expected}"
        );
    }
}

#[test]
fn inherited_digital_disciplines_select_loaded_converters_across_arrays_and_hierarchy() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
discipline low; domain discrete; potential Voltage; enddiscipline
discipline high; domain discrete; potential Temperature; enddiscipline
`default_discipline low
module low_source(output wreal value);
 real level=2.5; initial #1 level=4.5; assign value=level;
endmodule
`default_discipline high
module high_source(output wreal value);
 real level=3.5; initial #1 level=5.5; assign value=level;
endmodule
module bank(output wreal [3:2] a,b);
 parameter integer BASE=7;
 wreal first[BASE:BASE+1],second[BASE:BASE+1];
 low_source low_a(first[BASE]),low_b(first[BASE+1]);
 high_source high_a(second[BASE]),high_b(second[BASE+1]);
 assign a[3]=first[BASE]; assign a[2]=first[BASE+1];
 assign b[3]=second[BASE]; assign b[2]=second[BASE+1];
 // Inputs are structural connections so the bus acquires its discipline.
 low_observer l({a[3],a[2]}); high_observer h({b[3],b[2]});
endmodule
module low_observer(input low wreal [1:0] values); endmodule
module high_observer(input high wreal [1:0] values); endmodule
module load(input electrical [1:0] a,output electrical p);
 analog begin I(a[1])<+V(a[1])/1000; I(a[0])<+V(a[0])/1000; V(p)<+V(a[1])+V(a[0]); end
endmodule
module top(output electrical p,q);
 wire [1:0] x,y;
 bank #(.BASE(-3)) nested(x,y);
 load left(x,p),right(y,q);
endmodule
connectmodule low_gain(input low wreal value,output electrical a);
 analog I(a)<+(V(a)-2*value)/1000;
endmodule
connectmodule high_gain(input high wreal value,output electrical a);
 analog I(a)<+(V(a)-3*value)/1000;
endmodule
connectrules chosen; connect low_gain; connect high_gain; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* inherited disciplines\nX1 p q top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
    for (time, p, q) in [(0.5e-9, 5.0, 10.5), (1.4e-9, 9.0, 16.5)] {
        for (node, expected) in [("p", p), ("q", q)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn structural_wire_domains_preserve_physical_loading_and_real_conversion_boundaries() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module source(output electrical a);
 parameter real LEVEL=2.5;
 analog I(a)<+(V(a)-LEVEL)/1000;
endmodule
module pair(output wire [1:0] a);
 source #(.LEVEL(2.5)) first(a[1]);
 source #(.LEVEL(4.5)) second(a[0]);
endmodule
module bank(output wire [3:2] a);
 pair nested(a);
endmodule
module real_source(output logic wreal value);
 real level=1.75; initial #1 level=2.75; assign value=level;
endmodule
module load(input electrical [1:0] a,output electrical p);
 analog begin I(a[1])<+V(a[1])/1000; I(a[0])<+V(a[0])/1000; V(p)<+V(a[1])+V(a[0]); end
endmodule
module observer(input logic d,output electrical p);
 analog V(p)<+d;
endmodule
module top(output electrical p,q,r);
 wire [5:4] bus;
 tri words[-2:-1];
 bank nested(bus);
 source #(.LEVEL(6.5)) first(words[-2]);
 real_source second(words[-1]);
 load packed_load(bus,p),array_load({words[-2],words[-1]},q);
 observer monitor(bus[5],r);
endmodule
connectmodule gain(input logic wreal value,output electrical a);
 analog I(a)<+(V(a)-2*value)/1000;
endmodule
connectmodule sense(input electrical a,output logic d);
 reg d;
 initial d=0; always #0.1 d=V(a)>1;
endmodule
connectrules chosen; connect gain; connect sense; endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* structural wires retain loading\nX1 p q r top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
    for (time, q) in [(0.5e-9, 5.0), (1.4e-9, 6.0)] {
        for (node, expected) in [("p", 3.5), ("q", q), ("r", 1.0)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn implicit_and_generated_nets_keep_independent_loaded_mixed_channels() {
    let policy = Source::new("`default_nettype none\n");
    let source = Source::new(&format!(
        r#"
`timescale 1ns/1ps
`include "{}"
module analog_source(output electrical a);
 parameter real LEVEL=2.5;
 analog I(a)<+(V(a)-LEVEL)/1000;
endmodule
module load(input electrical a,output electrical p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module real_source(output logic wreal value);
 real level=1.75; initial #1 level=2.75; assign value=level;
endmodule
module real_reader(input logic wreal value,output electrical p);
 analog V(p)<+value;
endmodule
`default_nettype tri
module top(output electrical [1:0] p,q,output electrical r);
 genvar i;
 generate for(i=0;i<2;i=i+1) begin : channels
   // Separate implicit analog nets in each iteration.
   analog_source #(.LEVEL(i+2.5)) a(middle);
   load b(middle,p[i]);
   // Explicit generated arrays must retain their local shape and discipline.
   electrical local_nodes[i:i];
   analog_source #(.LEVEL(i+5.5)) c(local_nodes[i]);
   load d(local_nodes[i],q[i]);
 end endgenerate
 real_source digital_source(real_link);
 real_reader digital_load(real_link,r);
endmodule
"#,
        policy.path()
    ));
    let deck = Netlist::parse(&format!(
        "* scoped implicit interconnect\nX1 p1 p0 q1 q0 r top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1.5e-9, 50e-12).unwrap();
    for (time, r) in [(0.5e-9, 1.75), (1.4e-9, 2.75)] {
        for (node, expected) in [
            ("p0", 1.25),
            ("p1", 1.75),
            ("q0", 2.75),
            ("q1", 3.25),
            ("r", r),
        ] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn explicitly_continuous_wires_preserve_loading_and_digital_sampling() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module source(output electrical wire a);
 parameter real LEVEL=2.5;
 analog I(a)<+(V(a)-LEVEL)/1000;
endmodule
module load(input electrical wire a,output electrical wire p);
 analog begin I(a)<+V(a)/1000; V(p)<+V(a); end
endmodule
module pair(input electrical wire [1:0] a,output electrical wire p);
 analog begin I(a[1])<+V(a[1])/1000; I(a[0])<+V(a[0])/1000; V(p)<+V(a[1])+V(a[0]); end
endmodule
module top(output electrical wire p,output electrical wire [1:0] q,output electrical wire r,s);
 wire [3:2] bus; electrical [3:2] bus;
 source #(.LEVEL(4.5)) first(bus[3]); source #(.LEVEL(6.5)) second(bus[2]);
 pair packed_load(bus,p);
 genvar i;
 generate for(i=0;i<2;i=i+1) begin : channels
   tri electrical local_nodes[i:i];
   source #(.LEVEL(i+2.5)) producer(local_nodes[i]);
   load consumer(local_nodes[i],q[i]);
 end endgenerate
 wire scalar,discrete; electrical scalar;
 real sampled;
 source producer(scalar); load consumer(scalar,r);
 assign discrete=1'b1;
 always #0.1 sampled=V(scalar);
 analog V(s)<+sampled+discrete;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* explicit continuous wire storage\nX1 p q1 q0 r s top\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 5.5),
        ("q0", 1.25),
        ("q1", 1.75),
        ("r", 1.25),
        ("s", 2.25),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-7,
            "{node}: {actual} != {expected}"
        );
    }
}


#[test]
fn generated_mixed_bodies_keep_local_state_and_branch_loading_independent() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module top(output electrical [BASE:BASE+1] p,q);
 parameter integer BASE=0;
 genvar i;
 generate for(i=BASE;i<BASE+2;i=i+1) begin : channel
   localparam integer DELAY=i-BASE+1;
   localparam real LEVEL=i-BASE+2.5;
   electrical n;
   branch(n) drive;
   reg enabled=0;
   real sampled=0,offset;
   genvar j;
   analog function real shifted;
     input i; real i;
     begin shifted=i+LEVEL; end
   endfunction
   initial #DELAY enabled=1;
   always #0.1 sampled=V(n);
   analog initial offset=0.25;
   analog begin
     I(drive)<+(V(drive)-shifted(enabled ? 0.5 : 0.0)-offset)/1000;
     for(j=0;j<1;j=j+1) I(n)<+V(n)/1000;
     V(p[i])<+V(n);
     V(q[i])<+sampled;
   end
 end endgenerate
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* generated mixed channels\nX1 p0 p1 q0 q1 top BASE=-2\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 2.6e-9, 50e-12).unwrap();
    for (time, first, second) in [
        (0.5e-9, 1.375, 1.875),
        (1.5e-9, 1.625, 1.875),
        (2.5e-9, 1.625, 2.125),
    ] {
        for (node, expected) in [("p0", first), ("q0", first), ("p1", second), ("q1", second)] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}


#[test]
fn nested_generated_analog_bodies_follow_elaboration_order() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module top(output electrical p,q,r);
 parameter integer COUNT=2;
 real value,seed,sampled;
 genvar i,j;
 analog initial seed=0;
 analog value=0;
 generate for(i=0;i<COUNT;i=i+1) begin : outer
   analog value=value*10+1;
   analog initial seed=seed*10+5;
   for(j=0;j<2;j=j+1) begin : inner
     analog value=value*10+j+2;
     analog initial seed=seed*10+j+6;
   end
   analog value=value*10+4;
   analog initial seed=seed*10+8;
 end endgenerate
 analog initial seed=seed*10+9;
 analog begin value=value*10+9; V(p)<+value*1e-8; V(q)<+seed*1e-8; V(r)<+sampled; end
 initial #0.5 sampled=V(p);
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* analog concatenation after unrolling\nX1 p q r top\nX2 a b c top COUNT=1\n.va \"{}\" top module=top\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 1.23412349),
        ("q", 5.67856789),
        ("r", 1.23412349),
        ("a", 0.00012349),
        ("b", 0.00056789),
        ("c", 0.00012349),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-9,
            "{node}: {actual} != {expected}"
        );
    }
}

#[test]
fn generated_hierarchical_references_execute_in_loaded_specialized_circuits() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module top(output electrical p,q,r,s,t);
 parameter integer BASE=0;
 genvar i;
 wire \lane[0].code ;
 assign \lane[0].code =1'b0;
 generate for(i=BASE;i<BASE+2;i=i+1) begin : lane
   electrical n;
   branch(n) drive;
   reg [2:0] code=0;
   analog function real shifted;
     input x; real x;
     begin shifted=x+i-BASE+1; end
   endfunction
   analog I(drive)<+(V(drive)-(i-BASE+2))/1000;
   if(1) begin : inner
     wire [2:0] copied;
     assign copied=lane[BASE+1].code;
   end
 end endgenerate
 initial begin lane[BASE].code=3; lane[BASE+1].code[2:0]=5; end
 real sampled=0;
 initial #0.5 sampled=V(lane[BASE].n);
 analog begin
   I(lane[BASE].drive)<+V(lane[BASE].n)/1000;
   V(p)<+lane[BASE].code+lane[BASE+1].code;
   V(q)<+lane[BASE].shifted(lane[BASE+1].i)-BASE;
   V(r)<+V(lane[BASE].n);
   V(s)<+sampled+lane[BASE].inner.copied;
   V(t)<+\lane[0].code ;
 end
endmodule
"#,
    );
    let deck=Netlist::parse(&format!(
        "* generated hierarchical bindings\nX1 p q r s t top\nX2 a b c d e top BASE=-2\n.va \"{}\" top module=top\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 8.0),
        ("q", 2.0),
        ("r", 1.0),
        ("s", 6.0),
        ("t", 0.0),
        ("a", 8.0),
        ("b", 2.0),
        ("c", 1.0),
        ("d", 6.0),
        ("e", 0.0),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}

#[test]
fn foreign_parameter_reads_follow_nested_overrides_in_loaded_circuits() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module leaf(output electrical p);
 parameter real G=2;
 aliasparam gain=G;
 parameter integer STEP=2;
 parameter [65:0] BITS=66'h20000000000000001;
 localparam real L=G+0.5;
 genvar j;
 generate for(j=0;j<2;j=j+1) begin : cells
   localparam integer CODE=STEP+j;
 end endgenerate
 analog I(p)<+(V(p)-G)/1000;
endmodule
module middle(output electrical p);
 parameter real LEVEL=3;
 leaf #(.gain(LEVEL)) inner(p);
endmodule
module top(output electrical p,q,r,s,t);
 parameter real BASE=3;
 middle #(.LEVEL(BASE)) a(p);
 leaf #(.gain(BASE+2)) b(q);
 leaf #(.G(b.G+1)) c(r);
 real sampled=0;
 initial #0.5 sampled=a.inner.L+b.G+a.inner.cells[1].CODE+a.inner.BITS[65];
 analog begin
   I(p)<+V(p)/1000; I(q)<+V(q)/1000; I(r)<+V(r)/1000;
   V(s)<+sampled; V(t)<+a.inner.G/b.G;
 end
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* cross-module parameters\nX1 p q r s t top\nX2 a b c d e top BASE=5\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 1.5),
        ("q", 2.5),
        ("r", 3.0),
        ("s", 12.5),
        ("t", 0.6),
        ("a", 2.5),
        ("b", 3.5),
        ("c", 4.0),
        ("d", 16.5),
        ("e", 5.0 / 7.0),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}

#[test]
fn foreign_analog_functions_keep_target_scope_outputs_and_newton_derivatives() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module leaf;
 parameter real G=2;
 localparam real B=G+0.5;
 analog function real base; input x; real x; base=G*x+B; endfunction
 analog function real shaped;
   input x; inout acc; real x,acc;
   begin acc=acc+G; shaped=base(x)+sin(x)+$param_given(G); end
 endfunction
 genvar j;
 generate for(j=0;j<2;j=j+1) begin : cells
   analog function real scaled; input x; real x; scaled=base(x)+j; endfunction
 end endgenerate
endmodule
module middle;
 parameter real M=2;
 leaf #(.G(M)) inner();
endmodule
module top(output electrical p,q,r,s,t);
 parameter real BASE=2;
 middle #(.M(BASE)) a();
 leaf #(.G(BASE+2)) b();
 real acc, sampled=0;
 analog function real base; input x; real x; base=-99; endfunction
 analog function real sin; input x; real x; sin=10; endfunction
 initial #0.5 sampled=V(q);
 analog begin
   acc=1;
   I(p)<+a.inner.base(V(p))/1000;
   V(q)<+a.inner.shaped(0,acc);
   V(r)<+acc+sin(0);
   V(s)<+a.inner.cells[1].scaled(2)+b.base(0);
   V(t)<+sampled;
 end
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* foreign function scope\nI1 0 p 1m\nI2 0 a 1m\nX1 p q r s t top\nX2 a b c d e top BASE=4\n.va \"{}\" top module=top\n.end\n",
        source.path(),
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", -0.75),
        ("q", 3.5),
        ("r", 13.0),
        ("s", 12.0),
        ("t", 3.5),
        ("a", -0.875),
        ("b", 5.5),
        ("c", 15.0),
        ("d", 20.0),
        ("e", 5.5),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}


#[test]
fn foreign_physical_storage_preserves_nested_loading_and_port_currents() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module leaf(inout electrical p, output electrical q);
 parameter real G=2;
 electrical n;
 branch(p) drive;
 branch(<p>) supply;
 analog begin I(drive)<+(V(p)-G)/1000; I(n)<+(V(n)-6)/1000; V(q)<+V(n); end
endmodule
module middle(inout electrical p, output electrical q,r);
 parameter real G=2;
 leaf #(.G(G)) inner(p,q);
 analog begin I(inner.p)<+V(inner.p)/1000; V(r)<+1000*I(inner.supply); end
endmodule
module top(output electrical p,q,r,s,t);
 parameter real BASE=2;
 middle #(.G(BASE)) a(p,q,r);
 real sampled=0;
 initial #0.5 sampled=V(a.inner.n);
 analog begin
   I(a.inner.drive)<+1m;
   I(a.inner.n)<+V(a.inner.n)/1000;
   V(s)<+1000*I(<a.p>);
   V(t)<+sampled;
 end
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* shared foreign storage\nX1 p q r s t top\nX2 a b c d e top BASE=4\nR1 p 0 1k\nR2 a 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 1.0 / 3.0),
        ("q", 3.0),
        ("r", -2.0 / 3.0),
        ("s", -1.0 / 3.0),
        ("t", 3.0),
        ("a", 1.0),
        ("b", 3.0),
        ("c", -2.0),
        ("d", -1.0),
        ("e", 3.0),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}

#[test]
fn foreign_node_probes_keep_the_callers_unnamed_branch() {
    let source = Source::new(
        r#"
module leaf(inout electrical p);
 analog I(p)<+(V(p)-4)/1000;
endmodule
module middle(inout electrical p, output electrical q);
 leaf a(p);
 analog begin I(a.p)<+V(a.p)/2000; V(q)<+1000*I(a.p); end
endmodule
module top(output electrical p,q);
 middle a(p,q);
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* caller branch ownership\nX1 p q top\nR1 p 0 1k\n.va \"{}\" top module=top\n.end\n",
        source.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [("p", 1.6), ("q", 0.8)] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}


#[test]
fn foreign_explicit_branches_preserve_sources_direction_derivatives_and_boundaries() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module leaf(inout electrical p);
 parameter real G=4;
 analog I(p)<+(V(p)-G)/1000;
endmodule
module middle(inout electrical p,output electrical q,r,s,t);
 parameter real G=4;
 leaf #(.G(G)) a(p);
 analog begin
   I(a.p)<+V(a.p)/2000;
   V(q)<+1000*I(a.p);
   V(r)<+1000*I(a.branch(p));
   V(s)<+1000*I(a.branch(<p>));
   V(t)<+ddx(V(a.branch(0,p))*V(a.branch(0,p)),V(a.branch(0,p)));
 end
endmodule
module top(output electrical p,q,r,s,t,u,v,w,x);
 parameter real BASE=4;
 middle #(.G(BASE)) m(p,q,r,s,t);
 real sampled=0;
 initial #0.5 sampled=1000*I(m.a.branch(0,p));
 analog begin
   I(m.a.branch(p))<+1m;
   I(m.branch(a.p))<+0.5m;
   V(u)<+1000*I(m.branch(a.p));
   V(v)<+1000*I(m.branch(0,a.p));
   V(w)<+sampled;
   V(x)<+1000*I(m.branch(<p>));
 end
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* explicit hierarchy branches\nX1 p q r s t u v w x top\nX2 a b c d e f g h i top BASE=8\nR1 p 0 1k\nR2 a 0 1k\n.va \"{}\" top module=top\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 1.0),
        ("q", 1.0),
        ("r", -2.0),
        ("s", -2.0),
        ("t", -2.0),
        ("u", 1.0),
        ("v", -1.0),
        ("w", 2.0),
        ("x", -1.0),
        ("a", 2.6),
        ("b", 1.8),
        ("c", -4.4),
        ("d", -4.4),
        ("e", -5.2),
        ("f", 1.8),
        ("g", -1.8),
        ("h", 4.4),
        ("i", -2.6),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}

#[test]
fn foreign_upward_values_use_each_occurrence_and_the_selected_root() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module constants;
 parameter real K=99;
endmodule
module leaf(output electrical p,q,r,s);
 real sampled=0;
 initial #0.5 sampled=parent.G+sibling.K+$root.top.BASE;
 analog begin
   V(p)<+parent.G;
   V(q)<+sibling.K;
   V(r)<+parent.scale(2);
   V(s)<+sampled;
 end
endmodule
module parent(output electrical p,q,r,s);
 parameter real G=5;
 analog function real scale;
   input x; real x;
   begin scale=x+G; end
 endfunction
 constants #(.K(G+1)) sibling();
 leaf l(p,q,r,s);
endmodule
module top(output electrical p,q,r,s,t,u,v,w);
 parameter real BASE=2;
 parent #(.G(BASE)) a(p,q,r,s);
 parent b(t,u,v,w);
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* occurrence-relative values\nX1 p q r s t u v w top\nX2 a b c d e f g h top BASE=4\n.va \"{}\" top module=top\n.end\n", source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 2.0),
        ("q", 3.0),
        ("r", 4.0),
        ("s", 7.0),
        ("t", 5.0),
        ("u", 6.0),
        ("v", 7.0),
        ("w", 13.0),
        ("a", 4.0),
        ("b", 5.0),
        ("c", 6.0),
        ("d", 13.0),
        ("e", 5.0),
        ("f", 6.0),
        ("g", 7.0),
        ("h", 15.0),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}

#[test]
fn foreign_upward_physical_references_share_root_branches_and_boundary_current() {
    let source = Source::new(
        r#"
`timescale 1ns/1ps
module observer(output electrical q,r,s,t,u);
 real sampled=0;
 initial #0.5 sampled=1000*I($root.top.branch(p));
 analog begin
   I(top.branch(p))<+0.5m;
   I(top.supply)<+1m;
   V(q)<+1000*I(top.supply);
   V(r)<+1000*I($root.top.branch(0,p));
   V(s)<+1000*I(<top.p>);
   V(t)<+sampled;
   V(u)<+ddx(V(top.branch(0,p))*V(top.branch(0,p)),V(top.branch(0,p)));
 end
endmodule
module top(output electrical p,q,r,s,t,u);
 parameter real BASE=4;
 branch(p) supply;
 observer o(q,r,s,t,u);
 analog begin I(p)<+(V(p)-BASE)/1000; I(supply)<+V(p)/2000; end
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* upward shared physical state\nX1 p q r s t u top\nX2 a b c d e f top BASE=8\nR1 p 0 1k\nR2 a 0 1k\n.va \"{}\" top module=top\n.end\n", source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1e-9, 50e-12).unwrap();
    for (node, expected) in [
        ("p", 1.0),
        ("q", 1.5),
        ("r", 2.5),
        ("s", -1.0),
        ("t", -2.5),
        ("u", -2.0),
        ("a", 2.6),
        ("b", 2.3),
        ("c", 4.9),
        ("d", -2.6),
        ("e", -4.9),
        ("f", -5.2),
    ] {
        let actual = voltage(&result, node, 0.8e-9);
        assert!(
            (actual - expected).abs() < 1e-8,
            "{node}: {actual} != {expected}"
        );
    }
}

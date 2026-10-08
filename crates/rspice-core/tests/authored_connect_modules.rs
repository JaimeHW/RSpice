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

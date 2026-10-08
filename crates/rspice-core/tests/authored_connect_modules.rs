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

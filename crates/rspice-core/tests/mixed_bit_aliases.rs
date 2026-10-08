#![cfg(feature = "veriloga")]
//! Driver ownership across selected HDL inouts and linked design instances.
use rspice_core::xspice::verilog::{CompiledDigitalDesign, DigitalPort, DigitalStimulus};
use rspice_veriloga::canonical_ir::digital_link::DigitalLinkNet;

const SOURCE: &str = r#"
`timescale 1ns/1ps
module leaf(io);
 inout [6:7] io; wire [6:7] io;
 reg [1:0] value;
 initial begin value=2'b10; #1 value=2'bzz; #1 value=2'b01; #1 value=2'bzz; end
 assign io=value;
endmodule
module middle(io);
 parameter integer PICK=8;
 inout [9:6] io; wire [9:6] io;
 leaf child(io[PICK:PICK-1]);
endmodule
module top(bus);
 inout [2:5] bus; wire [2:5] bus;
 reg [1:0] value;
 middle m(bus);
 initial begin value=2'bzz; #1 value=2'b01; #1 value=2'b10; #1 value=2'bzz; end
 assign bus[3:4]=value;
endmodule
"#;

fn trace(design: &CompiledDigitalDesign, name: &str) -> Vec<String> {
    design
        .run(&DigitalStimulus {
            module: None,
            inputs: Vec::new(),
            outputs: vec![DigitalPort {
                name: name.into(),
                width: 4,
            }],
            clock: None,
            step: 1000,
            settle: 0,
            vectors: vec![Vec::new(); 4],
        })
        .unwrap()
        .observations
        .into_iter()
        .map(|row| row.values[0].1.clone())
        .collect()
}

#[test]
fn selected_inouts_preserve_contention_release_and_positional_linking() {
    let design = CompiledDigitalDesign::compile(SOURCE, Some("top")).unwrap();
    let expected = ["z10z", "z01z", "zxxz", "zzzz"];
    assert_eq!(trace(&design, "bus"), expected);
    // A fresh execution must not retain the last resolved values as drivers.
    assert_eq!(trace(&design, "bus"), expected);
    let linked = CompiledDigitalDesign::link(
        "linked",
        &[("dut", &design)],
        &[DigitalLinkNet {
            name: "net".into(),
            ports: vec![("dut".into(), "bus".into())],
        }],
    )
    .unwrap();
    assert_eq!(trace(&linked, "net"), expected);
    let nested = CompiledDigitalDesign::link(
        "nested",
        &[("outer", &linked)],
        &[DigitalLinkNet {
            name: "net".into(),
            ports: vec![("outer".into(), "net".into())],
        }],
    )
    .unwrap();
    assert_eq!(trace(&nested, "net"), expected);
}

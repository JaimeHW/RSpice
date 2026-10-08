#![cfg(feature = "veriloga")]
//! Local net views and driver ownership across HDL ports and linked instances.
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

fn check_whole_net_coordinates(direction: &str) {
    let source = format!(
        r#"
`timescale 1ns/1ps
module leaf(io);
 {direction} [6:9] io; wire [6:9] io;
 reg [3:0] value;
 initial begin value=4'b1010; #1 value=4'bzzzz; #1 value=4'b0101; #1 value=4'bzzzz; end
 assign io[6:7]=value[3:2];
 assign io[8:9]=value[1:0];
endmodule
module middle(io,seen);
 {direction} [-2:1] io; wire [-2:1] io;
 output [3:0] seen; wire [3:0] seen;
 leaf child(io);
 assign seen={{io[-2:-1],io[0:1]}};
endmodule
module top(bus,seen);
 inout [3:0] bus; wire [3:0] bus;
 output [3:0] seen; wire [3:0] seen;
 reg [3:0] value;
 middle m(bus,seen);
 initial begin value=4'bzzzz; #1 value=4'b0101; #1 value=4'b1010; #1 value=4'bzzzz; end
 assign bus=value;
endmodule
"#
    );
    let design = CompiledDigitalDesign::compile(&source, Some("top")).unwrap();
    let expected = ["1010", "0101", "xxxx", "zzzz"];
    assert_eq!(trace(&design, "bus"), expected, "{direction} writes");
    assert_eq!(trace(&design, "seen"), expected, "{direction} reads");
    let linked = CompiledDigitalDesign::link(
        "linked",
        &[("dut", &design)],
        &[DigitalLinkNet {
            name: "net".into(),
            ports: vec![("dut".into(), "bus".into())],
        }],
    )
    .unwrap();
    assert_eq!(trace(&linked, "net"), expected, "linked {direction}");
}

#[test]
fn whole_net_output_keeps_local_packed_coordinates() {
    check_whole_net_coordinates("output");
}

#[test]
fn whole_net_inout_keeps_local_packed_coordinates() {
    check_whole_net_coordinates("inout");
}

#[test]
fn whole_net_ports_keep_local_signedness() {
    for direction in ["output", "inout"] {
        for (child_sign, parent_sign, expected) in [
            ("signed", "", ["1110", "0001", "1011", "0000"]),
            ("", "signed", ["0010", "0001", "0011", "0000"]),
        ] {
            let source = format!(
                r#"
`timescale 1ns/1ps
module leaf(io,flags);
 {direction} {child_sign} [3:0] io; wire {child_sign} [3:0] io;
 output [3:0] flags; wire [3:0] flags;
 reg [3:0] value;
 initial begin value=4'b1000; #1 value=4'b0111; #1 value=4'b1111; #1 value=4'b0000; end
 assign io=value;
 assign flags={{io<0,(io>>>1)==4'sb1100,io[3],io[0]}};
endmodule
module top(bus,flags);
 inout {parent_sign} [3:0] bus; wire {parent_sign} [3:0] bus;
 output [3:0] flags; wire [3:0] flags;
 leaf child(bus,flags);
endmodule
"#
            );
            let design = CompiledDigitalDesign::compile(&source, Some("top")).unwrap();
            assert_eq!(
                trace(&design, "flags"),
                expected,
                "{direction} {child_sign}"
            );
        }
    }
}

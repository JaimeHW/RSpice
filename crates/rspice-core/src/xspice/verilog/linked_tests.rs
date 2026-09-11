//! Separately compiled instances executing in one resolved scheduling domain.
use super::*;
use rspice_veriloga::canonical_ir::digital_link::{
    DigitalLinkDirection, DigitalLinkInstance, DigitalLinkPort, link_digital_plans,
};

fn net(name: &str, ports: &[(&str, &str)]) -> DigitalLinkNet {
    DigitalLinkNet {
        name: name.into(),
        ports: ports
            .iter()
            .map(|(instance, port)| (instance.to_string(), port.to_string()))
            .collect(),
    }
}

fn stimulus(
    inputs: &[(&str, u32)],
    outputs: &[(&str, u32)],
    step: u64,
    rows: &[&[&str]],
) -> DigitalStimulus {
    let port = |&(name, width): &(&str, u32)| DigitalPort {
        name: name.into(),
        width,
    };
    DigitalStimulus {
        module: None,
        inputs: inputs.iter().map(port).collect(),
        outputs: outputs.iter().map(port).collect(),
        clock: None,
        step,
        settle: 0,
        vectors: rows
            .iter()
            .map(|row| row.iter().map(|value| value.to_string()).collect())
            .collect(),
    }
}

fn values(report: DigitalRunReport) -> Vec<Vec<String>> {
    report
        .observations
        .into_iter()
        .map(|row| row.values.into_iter().map(|(_, value)| value).collect())
        .collect()
}

#[test]
fn linked_digital_instances_sample_before_any_nonblocking_update() {
    let make = |initial| {
        CompiledDigitalDesign::compile(
            &format!(
                r#"
        module sampler(clk, d, q); input clk, d; output q; reg q;
        initial q={initial}; always @(posedge clk) q<=d; endmodule
    "#
            ),
            None,
        )
        .unwrap()
    };
    let a = make(0);
    let b = make(1);
    let nets = vec![
        net("clk", &[("a", "clk"), ("b", "clk")]),
        net("qa", &[("a", "q"), ("b", "d")]),
        net("qb", &[("b", "q"), ("a", "d")]),
    ];
    let linked = CompiledDigitalDesign::link("swap", &[("a", &a), ("b", &b)], &nets).unwrap();
    let mut reversed_nets = nets.clone();
    reversed_nets.reverse();
    for net in &mut reversed_nets {
        net.ports.reverse();
    }
    let reversed =
        CompiledDigitalDesign::link("swap", &[("b", &b), ("a", &a)], &reversed_nets).unwrap();
    assert_eq!(linked.plan.content_identity, reversed.plan.content_identity);
    let vectors = stimulus(
        &[("clk", 1)],
        &[("qa", 1), ("qb", 1), ("a.q", 1)],
        1,
        &[&["0"], &["1"], &["0"], &["1"], &["0"], &["1"]],
    );
    let result = values(linked.run(&vectors).unwrap());
    assert_eq!(
        result,
        vec![
            vec!["0", "1", "0"],
            vec!["1", "0", "1"],
            vec!["1", "0", "1"],
            vec!["0", "1", "0"],
            vec!["0", "1", "0"],
            vec!["1", "0", "1"]
        ]
    );
    assert_eq!(result, values(reversed.run(&vectors).unwrap()));
    // A linked design can itself be instantiated. Collapsed names still
    // resolve to the same underlying net rather than gaining a delta delay.
    let nested = CompiledDigitalDesign::link(
        "nested",
        &[("cell", &linked)],
        &[
            net("clock", &[("cell", "clk")]),
            net("out", &[("cell", "qa")]),
        ],
    )
    .unwrap();
    let vectors = stimulus(
        &[("clock", 1)],
        &[("out", 1), ("cell.a.q", 1)],
        1,
        &[&["0"], &["1"], &["0"], &["1"]],
    );
    assert_eq!(
        values(nested.run(&vectors).unwrap()),
        vec![
            vec!["0", "0"],
            vec!["1", "1"],
            vec!["1", "1"],
            vec!["0", "0"]
        ]
    );
}

#[test]
fn linked_digital_delays_use_module_units_and_share_rollback() {
    let clock = CompiledDigitalDesign::compile(
        r#"
        `timescale 1ns/100ps
        module clock(q); output q; reg q; initial q=0;
        always #0.5 q=~q; endmodule
    "#,
        None,
    )
    .unwrap();
    let delayed = CompiledDigitalDesign::compile(
        r#"
        `timescale 10ps/1ps
        module delayed(clk,q); input clk; output q; reg q; initial q=0;
        always @(posedge clk) q<=#25 ~q; endmodule
    "#,
        None,
    )
    .unwrap();
    let linked = CompiledDigitalDesign::link(
        "timed",
        &[("clock", &clock), ("delayed", &delayed)],
        &[
            net("clk", &[("clock", "q"), ("delayed", "clk")]),
            net("q", &[("delayed", "q")]),
        ],
    )
    .unwrap();
    assert_eq!(linked.time_resolution(), TimeResolution::new(-12).unwrap());
    let mut host = DigitalHost::from_plan(
        Arc::clone(&linked.plan),
        linked.resolution,
        SchedulerLimits::default(),
    );
    host.start().unwrap();
    let q = linked.signal_aliases["q"];
    host.advance_to(500).unwrap();
    assert_eq!(host.read(q).unwrap().spelling(), "0");
    let accepted = host.clone();
    host.advance_to(1000).unwrap();
    assert_eq!(host.read(q).unwrap().spelling(), "1");
    host = accepted;
    host.advance_to(749).unwrap();
    assert_eq!(host.read(q).unwrap().spelling(), "0");
    host.advance_to(750).unwrap();
    assert_eq!(host.read(q).unwrap().spelling(), "1");
    host.advance_to(1750).unwrap();
    assert_eq!(host.read(q).unwrap().spelling(), "0");
}

#[test]
fn linked_digital_resolvers_keep_partial_drivers_and_real_contributions() {
    let driver = CompiledDigitalDesign::compile(
        r#"
        module driver(en,d,bus); input en; input [4:7] d; output [7:4] bus;
        assign bus[7:6]=en?d[4:5]:2'bzz;
        assign bus[5:4]=en?d[6:7]:2'bzz; endmodule
    "#,
        None,
    )
    .unwrap();
    let monitor = CompiledDigitalDesign::compile(
        r#"
        module monitor(bus,q); input [10:13] bus; output [3:0] q;
        assign q=bus; endmodule
    "#,
        None,
    )
    .unwrap();
    let linked = CompiledDigitalDesign::link(
        "bus",
        &[("a", &driver), ("b", &driver), ("m", &monitor)],
        &[
            net("bus", &[("a", "bus"), ("b", "bus"), ("m", "bus")]),
            net("q", &[("m", "q")]),
        ],
    )
    .unwrap();
    let vectors = stimulus(
        &[("a.en", 1), ("a.d", 4), ("b.en", 1), ("b.d", 4)],
        &[("bus", 4), ("m.q", 4)],
        1,
        &[
            &["1", "1010", "0", "0101"],
            &["1", "1010", "1", "0101"],
            &["0", "1010", "0", "0101"],
            &["0", "1010", "1", "0101"],
        ],
    );
    assert_eq!(
        values(linked.run(&vectors).unwrap()),
        vec![
            vec!["1010", "1010"],
            vec!["xxxx", "xxxx"],
            vec!["zzzz", "zzzz"],
            vec!["0101", "0101"]
        ]
    );
    let make_real = |number| {
        CompiledDigitalDesign::compile(
            &format!("module source(r); output r; wrealsum r; assign r={number}; endmodule"),
            None,
        )
        .unwrap()
    };
    let first = make_real(1.25);
    let second = make_real(2.75);
    let linked = CompiledDigitalDesign::link(
        "sum",
        &[("a", &first), ("b", &second)],
        &[net("sum", &[("a", "r"), ("b", "r")])],
    )
    .unwrap();
    assert_eq!(
        values(
            linked
                .run(&stimulus(&[], &[("sum", 0)], 1, &[&[]]))
                .unwrap()
        ),
        vec![vec!["4.0"]]
    );
}

#[test]
fn linked_digital_computed_and_counted_event_controls_relocate_dependencies() {
    let source = CompiledDigitalDesign::compile(
        r#"
        module source(d,out); input [4:3] d; output [1:0] out; assign out=d; endmodule
    "#,
        None,
    )
    .unwrap();
    let sink = CompiledDigitalDesign::compile(
        r#"
        module sink(clk,d,q); input clk; input [4:3] d; output [1:0] q; reg [1:0] q;
        initial q=0;
        always @(posedge (d[4]^d[3])) q<=repeat(2) @(posedge clk) d;
        endmodule
    "#,
        None,
    )
    .unwrap();
    let linked = CompiledDigitalDesign::link(
        "events",
        &[("a", &source), ("b", &sink)],
        &[
            net("data", &[("a", "out"), ("b", "d")]),
            net("q", &[("b", "q")]),
        ],
    )
    .unwrap();
    let vectors = stimulus(
        &[("a.d", 2), ("b.clk", 1)],
        &[("q", 2)],
        1,
        &[
            &["00", "0"],
            &["10", "0"],
            &["11", "1"],
            &["11", "0"],
            &["11", "1"],
        ],
    );
    assert_eq!(
        values(linked.run(&vectors).unwrap()),
        vec![vec!["00"], vec!["00"], vec!["00"], vec!["00"], vec!["10"]]
    );
}

#[test]
fn linked_digital_analog_probes_and_refusals_preserve_instance_identity() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let artifact = compiler
        .compile_canonical_ir_module(
            r#"
        module probe(p,q); input p; electrical p; output q; reg q;
        initial q=(V(p)>0.5); endmodule
    "#,
            None,
        )
        .unwrap();
    let q = artifact
        .digital
        .signals
        .iter()
        .find(|signal| signal.name == "q")
        .unwrap()
        .id;
    let ports = [DigitalLinkPort {
        name: "q".into(),
        signal: q,
        direction: DigitalLinkDirection::Output,
    }];
    let inputs = [
        DigitalLinkInstance {
            name: "a",
            plan: &artifact.digital,
            ports: &ports,
        },
        DigitalLinkInstance {
            name: "b",
            plan: &artifact.digital,
            ports: &ports,
        },
    ];
    let linked = link_digital_plans(&inputs, &[], &rspice_veriloga::NoPipelineControl).unwrap();
    assert_ne!(
        linked.instances[0].source_files,
        linked.instances[1].source_files
    );
    let mut probes = vec![None; linked.plan.analog_probes.len()];
    probes[usize::from(linked.instances[0].analog_probes[0])] = Some(0.25);
    probes[usize::from(linked.instances[1].analog_probes[0])] = Some(0.75);
    let mut host = DigitalHost::new(
        &linked.plan,
        TimeResolution::new(linked.plan.timing.precision_exponent).unwrap(),
        SchedulerLimits::default(),
    );
    host.sample_analog_probes(&probes);
    host.start().unwrap();
    assert_eq!(
        host.read(linked.signal_names["a.q"]).unwrap().spelling(),
        "0"
    );
    assert_eq!(
        host.read(linked.signal_names["b.q"]).unwrap().spelling(),
        "1"
    );
    let single = CompiledDigitalDesign::compile(
        "module single(r); output r; wreal r; assign r=1.0; endmodule",
        None,
    )
    .unwrap();
    let error = CompiledDigitalDesign::link(
        "invalid",
        &[("a", &single), ("b", &single)],
        &[net("r", &[("a", "r"), ("b", "r")])],
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("resolution"), "{error}");
    let error = CompiledDigitalDesign::link(
        "invalid",
        &[("a", &single)],
        &[net("r", &[("a", "missing")])],
    )
    .unwrap_err()
    .to_string();
    assert!(error.contains("a.missing"), "{error}");
    struct Cancelled;
    impl rspice_veriloga::PipelineControl for Cancelled {
        fn is_cancelled(&self) -> bool {
            true
        }
    }
    let error =
        CompiledDigitalDesign::link_with_control("cancelled", &[("a", &single)], &[], &Cancelled)
            .unwrap_err()
            .to_string();
    assert!(error.contains("cancelled"), "{error}");
}

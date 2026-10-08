#![cfg(feature = "veriloga")]
//! Analog occurrences wake digital processes in the unified circuit transaction.
use rspice_core::{Engine, Netlist};
use rspice_core::engine::TransientResult;
struct Source(std::path::PathBuf);
impl Source {
    fn new(text: &str) -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "rspice-analog-events-{}-{}.va",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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
        .position(|n| n.eq_ignore_ascii_case(name))
        .unwrap();
    let sample = result.time.iter().position(|t| *t >= time).unwrap();
    result.voltage_waveform(node + 1)[sample]
}

#[test]
fn analog_cross_and_timer_subscriptions_survive_hierarchy_and_independent_clocks() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module detector(a,p,t);
 input a; electrical a;
 inout p,t; electrical p,t;
 parameter real threshold=0.3573;
 integer crossings=0, ticks=0, future=0, early=0;
 real last=0;
 initial #358 future=1;
 always @(cross(V(a)-threshold,1,0.1p,1u) or cross(V(a)-threshold,-1,0.1p,1u)) begin crossings=crossings+1; last=$abstime; if ($abstime<358p) early=future; end
 always @(timer(210p,330p)) ticks=ticks+1;
 analog begin I(p)<+(V(p)-(crossings+10*ticks))/1000; I(t)<+(V(t)-(last*1e9+10*early))/1000; end
endmodule
module wrapper(a,p,q,tp,tq);
 input a; electrical a;
 inout p,q,tp,tq; electrical p,q,tp,tq;
 detector first(a,p,tp);
 detector #(.threshold(0.65)) second(a,q,tq);
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* event subscriptions in mixed children\nV1 a 0 pwl(0 0 1n 1 2n 0)\nX1 a p q tp tq wrapper\nRp p 0 1k\nRq q 0 1k\nRtp tp 0 1k\nRtq tq 0 1k\nVaux aux 0 pulse(0 1 0 1p 1p 500p 1n)\nRload aux load 100\nCload load 0 0.5p\n.va \"{}\" wrapper module=wrapper\n.end\n",source.path())).unwrap();
    for step in [70e-12, 23e-12] {
        let engine = Engine::default();
        let result = engine.run_tran(&deck, 1.8e-9, step).unwrap();
        if step == 70e-12 {
            assert!(
                engine.convergence_quality().timestep_reductions > 0,
                "exercise rejected candidates"
            );
        }
        assert!((voltage(&result, "tp", 0.42e-9) - 0.3573 / 2.0).abs() < 2e-4);
        let fallen = voltage(&result, "tp", 1.7e-9);
        assert!((fallen - 1.6427 / 2.0).abs() < 2e-4, "fall stamp={fallen}");
        for (time, p, q) in [
            (0.1e-9, 0.0, 0.0),
            (0.25e-9, 5.0, 5.0),
            (0.42e-9, 5.5, 5.0),
            (0.7e-9, 10.5, 10.5),
            (1.7e-9, 26.0, 26.0),
        ] {
            for (name, expected) in [("p", p), ("q", q)] {
                let actual = voltage(&result, name, time);
                assert!(
                    (actual - expected).abs() < 1e-6,
                    "step={step}, {name}@{time}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn unchanged_analog_assignments_wake_repeat_controls_and_mixed_event_lists() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module sampled(a,p);
 input a; electrical a;
 inout p; electrical p;
 real sample=0;
 integer seen=0, repeated=0, started=0, combined=0;
 reg clock=0;
 initial #450 clock=1;
 analog @(timer(150p,300p)) sample=V(a);
 always @(sample) seen=seen+1;
 initial begin repeated=repeat(3) @(sample) 1; end
 always @(above(V(a)-0.5)) started=started+1;
 always @(sample or posedge clock) combined=combined+1;
 analog I(p)<+(V(p)-(seen+10*repeated+100*started+1000*combined))/1000;
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* identical sampled assignments remain events\nV1 a 0 1\nX1 a p sampled\nRp p 0 1k\n.va \"{}\" sampled\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1.1e-9, 70e-12).unwrap();
    // above fires at initialization; three identical assignments at 150,450,
    // 750 ps satisfy the repeat wait. The clock at 450 ps is another event.
    assert!((voltage(&result, "p", 0.1e-9) - 50.0).abs() < 1e-6);
    assert!((voltage(&result, "p", 0.25e-9) - 550.5).abs() < 1e-6);
    let final_value = voltage(&result, "p", 0.85e-9);
    assert!((final_value - 2056.5).abs() < 1e-6, "{final_value}");
}

#[test]
fn standalone_analog_occurrences_roll_back_with_waiting_processes() {
    use rspice_core::xspice::verilog::MixedSignalHost;
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_veriloga::vm::IntegrationCoefficients;
    let source = r#"
`timescale 1ps/1ps
module counter(a);
 inout a; electrical a;
 integer count=0;
 always @(timer(100p,100p)) count=count+1;
 analog I(a)<+V(a)/1000;
endmodule
"#;
    let mut host =
        MixedSignalHost::compile(source, None, "counter", &[1], SchedulerLimits::default())
            .unwrap();
    let evaluate = |host: &mut MixedSignalHost, time: f64, step: f64| {
        host.begin_trial(
            time,
            step,
            IntegrationCoefficients::inactive(),
            time == 0.0,
            false,
        )
        .unwrap();
        for _ in 0..8 {
            host.stamp(&[0.0, 0.0], |_, _, _| {}, |_, _| {}).unwrap();
            if !host.settle_analog_bridges(&[0.0, 0.0]).unwrap() {
                return;
            }
        }
        panic!("analog occurrence did not settle");
    };
    evaluate(&mut host, 0.0, 0.0);
    host.accept_trial().unwrap();
    evaluate(&mut host, 100e-12, 100e-12);
    assert_eq!(host.read_digital("count").unwrap(), format!("{:032b}", 1));
    host.reject_trial().unwrap();
    assert_eq!(host.read_digital("count").unwrap(), format!("{:032b}", 0));
    evaluate(&mut host, 100e-12, 100e-12);
    host.accept_trial().unwrap();
    assert_eq!(host.read_digital("count").unwrap(), format!("{:032b}", 1));
    evaluate(&mut host, 200e-12, 100e-12);
    host.accept_trial().unwrap();
    assert_eq!(host.read_digital("count").unwrap(), format!("{:032b}", 2));
}

#[test]
fn authored_threshold_subscription_drives_xspice_at_its_physical_crossing() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module helper(a);
 inout a; electrical a;
 analog I(a)<+V(a)/1e12;
endmodule
connectmodule threshold_adc(a,d);
 input a; electrical a;
 output d; logic d; reg d;
 parameter real threshold=0.65;
 initial d=0;
 always @(above(V(a)-threshold) or above(threshold-V(a))) d=(V(a)>=threshold);
endmodule
connectrules selected;
 connect threshold_adc;
endconnectrules
"#,
    );
    let deck = Netlist::parse(&format!(
        "* authored analog event with independent XSPICE propagation\nV1 din 0 pwl(0 0 1n 1 2n 0)\nA1 din dout buffer\n.model buffer d_buffer(rise_delay=17p fall_delay=17p)\n.va \"{}\" helper module=helper\n.end\n", source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 1.6e-9, 70e-12).unwrap();
    let trace = &result
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case("dout"))
        .unwrap()
        .points;
    use rspice_core::xspice::DigitalState;
    let rise = trace
        .iter()
        .find(|point| point.value.state == DigitalState::One)
        .unwrap();
    let fall = trace
        .iter()
        .find(|point| point.time > rise.time && point.value.state == DigitalState::Zero)
        .unwrap();
    assert!((rise.time - 667e-12).abs() < 1e-16, "{trace:?}");
    assert!((fall.time - 1367e-12).abs() < 1e-16, "{trace:?}");
}

#[test]
fn event_assigned_values_refresh_continuous_real_and_logic_drivers() {
    // These resistively loaded outputs hold between analog timer events. Read
    // the latest accepted point before the requested time, not a later event.
    let held_voltage = |result: &TransientResult, name: &str, time: f64| {
        let node = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap();
        let sample = result.time.iter().rposition(|t| *t <= time).unwrap();
        result.voltage_waveform(node + 1)[sample]
    };
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module held(a,p,q);
 input a; electrical a;
 inout p,q; electrical p,q;
 parameter real gain=1, start=157.3p;
 real sample=0.25, bias=0.1, unassigned;
 wreal held_value, offset;
 wire high=(sample>0.4);
 assign offset=bias+unassigned;
 assign held_value=gain*sample+offset;
 analog begin
   @(timer(start,200p)) sample=V(a);
   I(p)<+(V(p)-held_value)/1000;
   I(q)<+(V(q)-(high ? 1.0 : 0.0))/1000;
 end
endmodule
module pair(a,p,q,p2,q2);
 input a; electrical a;
 inout p,q,p2,q2; electrical p,q,p2,q2;
 held first(a,p,q);
 held #(.gain(2.0),.start(317.3p)) second(a,p2,q2);
endmodule
"#,
    );
    for (module, pins, extra) in [
        ("held", "a p q", ""),
        ("pair", "a p q p2 q2", "Rp2 p2 0 1k\nRq2 q2 0 1k\n"),
    ] {
        let deck = Netlist::parse(&format!(
            "* event-driven continuous assignments\nV1 a 0 pwl(0 0 1n 1)\nX1 {pins} {module}\nRp p 0 1k\nRq q 0 1k\n{extra}.va \"{}\" {module} module={module}\n.end\n", source.path()
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 0.9e-9, 70e-12).unwrap();
        for (time, p, q) in [
            (0.1e-9, 0.175, 0.0),
            (0.2e-9, 0.2573 / 2.0, 0.0),
            (0.45e-9, 0.4573 / 2.0, 0.0),
            (0.65e-9, 0.6573 / 2.0, 0.5),
        ] {
            for (node, expected) in [("p", p), ("q", q)] {
                let actual = held_voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{module} {node}@{time}: {actual} != {expected}"
                );
            }
        }
        if module == "pair" {
            for (time, expected) in [(0.1e-9, 0.30), (0.4e-9, 0.3673), (0.65e-9, 0.5673)] {
                assert!((held_voltage(&result, "p2", time) - expected).abs() < 1e-7);
            }
        }
    }
}

#[test]
fn event_loop_control_assignments_each_wake_a_waiting_process() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module loop_events(p,q);
 inout p,q; electrical p,q;
 integer k=0, seen=0, ready=0, zero=9, zero_events=0;
 analog @(timer(125p,500p)) for (k=0;k<3;k=k+1) begin end
 analog @(timer(125p,500p)) for(zero=5;zero<2;zero=zero+1) begin end
 always @(zero) zero_events=zero_events+1;
 always @(k) seen=seen+1;
 initial begin ready=repeat(4) @(k) 1; end
 analog I(p)<+(V(p)-(seen+10*ready+100*k))/1000;
 analog I(q)<+(V(q)-(zero+10*zero_events))/1000;
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* loop assignment occurrences\nX1 p q loop_events\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" loop_events\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 0.8e-9, 70e-12).unwrap();
    assert!(voltage(&result, "p", 0.05e-9).abs() < 1e-8);
    assert!((voltage(&result, "q", 0.05e-9) - 4.5).abs() < 1e-7);
    assert!((voltage(&result, "q", 0.2e-9) - 7.5).abs() < 1e-7);
    assert!((voltage(&result, "q", 0.7e-9) - 12.5).abs() < 1e-7);
    assert!(
        (voltage(&result, "p", 0.2e-9) - 157.0).abs() < 1e-7,
        "p={}",
        voltage(&result, "p", 0.2e-9)
    );
    assert!((voltage(&result, "p", 0.7e-9) - 159.0).abs() < 1e-7);
}

#[test]
fn continuously_evaluated_variables_cannot_silently_freeze_a_digital_driver() {
    use rspice_core::xspice::verilog::MixedSignalHost;
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    for assignment in [
        "analog sample=V(a);",
        "analog initial sample=0.125; analog @(timer(1n)) sample=V(a);",
        r#"real unused;
        analog function real relay;
            output result;
            input value;
            real result, value;
            begin result=value; relay=0; end
        endfunction
        analog unused=relay(sample,V(a));"#,
        r#"real unused;
        analog function real relay;
            output result; input value;
            real result,value;
            begin result=value; relay=0; end
        endfunction
        analog @(timer(1n)) sample=0.5;
        analog unused=relay(sample,V(a));"#,
        r#"real unused;
        analog function real relay;
            output result; input value;
            real result,value;
            begin result=value; relay=0; end
        endfunction
        analog @(timer(1n)) unused=relay(sample,0.5);
        analog sample=V(a);"#,
    ] {
        let source = format!(
            r#"
module invalid(a);
 inout a; electrical a;
 real sample;
 wreal held_value;
 assign held_value=sample;
 {assignment}
endmodule
"#
        );
        let error =
            MixedSignalHost::compile(&source, None, "invalid", &[1], SchedulerLimits::default())
                .err()
                .expect("an unclocked analog variable is not an event-driven source");
        assert!(
            error
                .to_string()
                .contains("not assigned exclusively in analog event statements"),
            "{error}"
        );
    }
}

#[test]
fn function_copy_outs_publish_guarded_assignment_events_without_local_shadows() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module function_events(p,q);
 inout p,q; electrical p,q;
 real sample=0.25, accum=0, unused;
 integer calls=0, seen=0, ready=0, accum_seen=0;
 wreal held_value;
 analog function real relay;
   output result; input value;
   real result,value;
   begin result=value; relay=0; end
 endfunction
 analog function real add;
   inout result; input value;
   real result,value;
   begin result=result+value; add=0; end
 endfunction
 analog @(timer(125p,250p)) begin
   calls=calls+1;
   unused=(calls<3) ? relay(sample,0.75) : 0;
   unused=(calls==2) ? add(accum,0.5) : 0;
   begin : shadow
     real sample;
     sample=12;
     unused=relay(sample,14);
   end
 end
 always @(sample) seen=seen+1;
 initial ready=repeat(2) @(sample) 1;
 always @(accum) accum_seen=accum_seen+1;
 assign held_value=sample;
 analog I(p)<+(V(p)-(seen+10*ready+100*held_value))/1000;
 analog I(q)<+(V(q)-(accum+10*accum_seen))/1000;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* function assignment occurrences\nX1 p q function_events\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" function_events\n.end\n", source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 0.8e-9, 70e-12).unwrap();
    for (time, p, q) in [
        (0.05e-9, 12.5, 0.0),
        (0.2e-9, 38.0, 0.0),
        (0.4e-9, 43.5, 5.25),
        (0.7e-9, 43.5, 5.25),
    ] {
        for (name, expected) in [("p", p), ("q", q)] {
            let actual = voltage(&result, name, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{name}@{time}: {actual} != {expected}"
            );
        }
    }
}

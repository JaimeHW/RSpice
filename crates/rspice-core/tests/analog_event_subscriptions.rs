#![cfg(feature = "veriloga")]
//! Analog occurrences wake digital processes in the unified circuit transaction.
use rspice_core::{Engine, Netlist};
use rspice_core::engine::TransientResult;
#[derive(Default)]
struct AcceptedEvents(
    std::sync::Mutex<Vec<(String, rspice_core::abort_signal::TransientEventChange)>>,
);
impl rspice_core::abort_signal::AbortSignal for AcceptedEvents {
    fn is_aborted(&self) -> bool {
        false
    }
    fn observe_transient_sample(&self, sample: rspice_core::abort_signal::TransientSample<'_>) {
        let changes = sample
            .event_changes
            .expect("engine provides accepted event history");
        let mut events = self.0.lock().unwrap();
        for &change in changes {
            assert!(change.time <= *sample.time.last().unwrap());
            events.push((sample.node_names[change.node - 1].clone(), change));
        }
    }
}
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
    let array_source=source.replace("always @(timer(100p,100p)) count=count+1;", "integer data[-1:1]; integer selected=-1; initial #150 selected=0; analog @(timer(100p,100p)) data[selected]=0; always @(data[selected]) count=count+1;");
    for source in [source, array_source.as_str()] {
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
        evaluate(&mut host, 150e-12, 50e-12);
        host.accept_trial().unwrap();
        assert_eq!(host.read_digital("count").unwrap(), format!("{:032b}", 1));
        evaluate(&mut host, 200e-12, 50e-12);
        host.accept_trial().unwrap();
        assert_eq!(host.read_digital("count").unwrap(), format!("{:032b}", 2));
    }
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
        for source in [
            source.clone(),
            source.replace(
                "assign held_value=sample;",
                "reg tick=0; real observed; always @* observed=sample+(tick ? 1 : 0);",
            ),
        ] {
            let error = MixedSignalHost::compile(
                &source,
                None,
                "invalid",
                &[1],
                SchedulerLimits::default(),
            )
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

#[test]
fn implicit_sensitivity_subscribes_to_analog_assignments_with_lexical_scope() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module implicit_events(a,p,q,h);
 input a; electrical a;
 inout p,q,h; electrical p,q,h;
 real sample=0.25, analog_shadow, observed=0, captured=0;
 integer seen=0, shadow_value=0;
 reg tick=0;
 initial begin #100 tick=1; #300 tick=0; end
 initial captured=repeat(2) @* sample;
 always @* begin observed=sample; if (sample>0.5) seen=seen+1; end
 always @* begin : lexical
   real analog_shadow;
   analog_shadow=tick ? 2 : 4;
   shadow_value=analog_shadow;
 end
 analog begin
   analog_shadow=V(a);
   @(timer(125p,250p)) sample=0.75;
   I(p)<+(V(p)-(observed+10*seen))/1000;
   I(q)<+(V(q)-shadow_value)/1000;
   I(h)<+(V(h)-captured)/1000;
 end
endmodule
module wrapper(a,p,q,h);
 input a; electrical a;
 inout p,q,h; electrical p,q,h;
 implicit_events child(a,p,q,h);
endmodule
"#,
    );
    for module in ["implicit_events", "wrapper"] {
        let deck = Netlist::parse(&format!(
            "* implicit analog subscriptions\nV1 a 0 1\nX1 a p q h {module}\nRp p 0 1k\nRq q 0 1k\nRh h 0 1k\n.va \"{}\" {module} module={module}\n.end\n",source.path()
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 0.8e-9, 70e-12).unwrap();
        for (time, p, q, h) in [
            (0.2e-9, 5.375, 1.0, 0.0),
            (0.45e-9, 10.375, 2.0, 0.125),
            (0.7e-9, 15.375, 2.0, 0.125),
        ] {
            for (node, expected) in [("p", p), ("q", q), ("h", h)] {
                let actual = voltage(&result, node, time);
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{module} {node}@{time}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn loop_body_and_function_writes_control_the_actual_iteration_count() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module changing_loops(p,q);
 inout p,q; electrical p,q;
 integer k=0,j=0,k_seen=0,j_seen=0;
 real unused;
 analog function real advance;
   inout value; integer value;
   begin value=value+1; advance=0; end
 endfunction
 analog @(timer(125p,500p)) begin
   for(k=0;k<5;k=k+1) k=k+1;
   for(j=0;j<5;j=j+1) unused=advance(j);
 end
 always @(k) k_seen=k_seen+1;
 always @(j) j_seen=j_seen+1;
 analog I(p)<+(V(p)-(100*k+k_seen))/1000;
 analog I(q)<+(V(q)-(100*j+j_seen))/1000;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* loop body mutates its counter\nX1 p q changing_loops\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" changing_loops\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 0.8e-9, 70e-12).unwrap();
    for (time, expected) in [(0.2e-9, 303.5), (0.7e-9, 307.0)] {
        for node in ["p", "q"] {
            let actual = voltage(&result, node, time);
            assert!(
                (actual - expected).abs() < 1e-7,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn event_wait_rearming_preserves_nonblocking_and_continuous_feedback() {
    for feedback in [
        "reg q=0; initial #100 q=1; always @(q) q<=~q;",
        "reg seed=0; wire q; initial #100 seed=1; assign q=seed ? ((q===1'b1) ? 1'b0 : 1'b1) : 1'b0;",
    ] {
        let source = Source::new(&format!(
            r#"
`timescale 1ps/1ps
module feedback(p);
 inout p; electrical p;
 {feedback}
 analog I(p)<+V(p)/1000;
endmodule
"#
        ));
        let deck = Netlist::parse(&format!(
            "* genuine event feedback\nX1 p feedback\nRp p 0 1k\n.va \"{}\" feedback\n.end\n",
            source.path()
        ))
        .unwrap();
        let error = Engine::default()
            .run_tran(&deck, 0.2e-9, 70e-12)
            .unwrap_err();
        assert!(
            error.to_string().contains("did not settle"),
            "{feedback}: {error}"
        );
    }
}

#[test]
fn constant_loop_event_sites_keep_independent_subscriptions() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module event_sites(p);
 inout p; electrical p;
 integer k, sample=0, seen=0;
 analog for(k=0;k<2;k=k+1) @(timer((k+1)*125p)) sample=sample+1;
 always @(sample) seen=seen+1;
 analog I(p)<+(V(p)-(sample+10*seen))/1000;
endmodule
"#,
    );
    let deck=Netlist::parse(&format!(
        "* independent unrolled event sites\nX1 p event_sites\nRp p 0 1k\n.va \"{}\" event_sites\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 0.4e-9, 40e-12).unwrap();
    for (time, expected) in [(0.18e-9, 5.5), (0.3e-9, 11.0)] {
        let actual = voltage(&result, "p", time);
        assert!(
            (actual - expected).abs() < 1e-7,
            "p@{time}: {actual} != {expected}"
        );
    }
}

#[test]
fn array_assignment_events_select_cells_and_retain_continuous_driver_values() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module array_events(p,q,r,s);
 inout p,q,r,s; electrical p,q,r,s;
 real samples[1:-1]='{0.0,0.0,0.0};
 integer writer=-1, selected=-1, negative_seen=0, selected_seen=0, all_seen=0;
 real implicit_value=0;
 wreal held_value;
 initial begin #160 selected=0; #50 selected=-1; #100 selected=1; #200 selected=0; #100 selected=1; end
 analog @(timer(125p,250p)) begin
   samples[writer]=0.75;
   samples[writer]=0.75;
   writer=writer+1;
   if (writer>1) writer=-1;
 end
 always @(samples[-1]) negative_seen=negative_seen+1;
 always @(samples[selected]) selected_seen=selected_seen+1;
 always @(samples) all_seen=all_seen+1;
 always @* implicit_value=samples[selected];
 assign held_value=samples[selected];
 analog I(p)<+(V(p)-held_value)/1000;
 analog I(q)<+(V(q)-(negative_seen+10*selected_seen))/1000;
 analog I(r)<+(V(r)-all_seen)/1000;
 analog I(s)<+(V(s)-implicit_value)/1000;
endmodule
module wrapper(p,q,r,s);
 inout p,q,r,s; electrical p,q,r,s;
 array_events child(p,q,r,s);
endmodule
"#,
    );
    for module in ["array_events", "wrapper"] {
        let deck=Netlist::parse(&format!(
            "* array occurrences and held values\nX1 p q r s {module}\nRp p 0 1k\nRq q 0 1k\nRr r 0 1k\nRs s 0 1k\n.va \"{}\" {module} module={module}\n.end\n",source.path()
        )).unwrap();
        let result = Engine::default().run_tran(&deck, 0.8e-9, 23e-12).unwrap();
        for (time, held, q, r) in [
            (0.14e-9, 0.375, 11.0, 1.0),
            (0.18e-9, 0.0, 11.0, 1.0),
            (0.25e-9, 0.375, 11.0, 1.0),
            (0.4e-9, 0.0, 11.0, 2.0),
            (0.55e-9, 0.375, 11.0, 2.0),
            (0.7e-9, 0.375, 21.0, 3.0),
        ] {
            for (node, expected) in [("p", held), ("q", q), ("r", r), ("s", held)] {
                let index = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(node))
                    .unwrap();
                let point = result
                    .time
                    .iter()
                    .rposition(|value| *value <= time)
                    .unwrap();
                let actual = result.voltage_waveform(index + 1)[point];
                assert!(
                    (actual - expected).abs() < 1e-7,
                    "{module} {node}@{time}: {actual} != {expected}"
                );
            }
        }
    }
}

#[test]
fn array_write_occurrence_uses_the_address_before_the_write_changes_it() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module indexed_write(p,q);
 inout p,q; electrical p,q;
 integer address[0:1]='{0,0}; integer first=0,second=0;
 analog @(timer(125p,250p)) address[address[0]]=1;
 always @(address[0]) first=first+1;
 always @(address[1]) second=second+1;
 analog I(p)<+(V(p)-first)/1000;
 analog I(q)<+(V(q)-second)/1000;
endmodule
"#,
    );
    let deck=Netlist::parse(&format!(
        "* stable assignment address\nX1 p q indexed_write\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" indexed_write\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 0.8e-9, 50e-12).unwrap();
    for (time, q) in [(0.2e-9, 0.0), (0.45e-9, 0.5), (0.7e-9, 1.0)] {
        assert!((voltage(&result, "p", time) - 0.5).abs() < 1e-7);
        assert!((voltage(&result, "q", time) - q).abs() < 1e-7);
    }
}

#[test]
fn analog_array_event_ownership_is_checked_per_selected_element() {
    use rspice_core::xspice::verilog::MixedSignalHost;
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    let source = r#"
`timescale 1ps/1ps
module cells(a,p);
 input a; electrical a;
 inout p; electrical p;
 real data[0:1]='{0.0,0.0}; integer seen=0;
 wreal held;
 analog begin data[1]=V(a); @(timer(125p)) data[0]=0.75; I(p)<+(V(p)-(held+seen))/1000; end
 always @(data[0]) seen=seen+1;
 assign held=data[0];
endmodule
"#;
    let input = Source::new(source);
    let deck = Netlist::parse(&format!(
        "* independent cell ownership\nV1 a 0 1\nX1 a p cells\nRp p 0 1k\n.va \"{}\" cells\n.end\n",
        input.path()
    ))
    .unwrap();
    let result = Engine::default().run_tran(&deck, 0.3e-9, 40e-12).unwrap();
    assert!((voltage(&result, "p", 0.2e-9) - 0.875).abs() < 1e-7);
    for invalid in [
        source.replace("@(data[0])", "@(data[1])"),
        source.replace("assign held=data[0]", "assign held=data[1]"),
        source.replace(
            "always @(data[0]) seen=seen+1;",
            "real observed; reg tick=0; always @* observed=data[1]+(tick?1:0);",
        ),
    ] {
        let error = MixedSignalHost::compile(
            &invalid,
            None,
            "invalid",
            &[1, 2],
            SchedulerLimits::default(),
        )
        .err()
        .expect("continuous cell must not acquire an assignment-event dependency");
        assert!(
            error.to_string().contains("not assigned exclusively"),
            "{error}"
        );
    }
}

#[test]
fn absdelta_operand_plans_survive_hierarchy_and_circuit_linking() {
    use rspice_veriloga::{CompilerOptions, NoPipelineControl, VerilogACompiler};
    use rspice_veriloga::canonical_ir::digital::{DigitalAnalogProbeTarget, DigitalAnalogQuantity};
    use rspice_veriloga::canonical_ir::digital_link::{DigitalLinkInstance, link_digital_plans};
    let source = r#"
`timescale 1ns/1ps
module observer(a);
 input a; electrical a;
 parameter real increment=0.1;
 real sampled=0;
 always @(absdelta(V(a),increment,,1u)) sampled=V(a);
endmodule
module wrapper(a,b);
 input a,b; electrical a,b;
 observer #(.increment(0.25)) first(a);
 observer #(.increment(0.5)) second(b);
endmodule
"#;
    let artifact = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(source, Some("wrapper"))
        .unwrap();
    let plan = &artifact.digital;
    assert_eq!(plan.absdelta.len(), 2);
    let linked = link_digital_plans(
        &[
            DigitalLinkInstance {
                name: "left",
                plan,
                ports: &[],
            },
            DigitalLinkInstance {
                name: "right",
                plan,
                ports: &[],
            },
        ],
        &[],
        &NoPipelineControl,
    )
    .unwrap();
    assert_eq!(linked.plan.absdelta.len(), 4);
    let mut signals = std::collections::BTreeSet::new();
    let mut operands = std::collections::BTreeSet::new();
    for observer in &linked.plan.absdelta {
        assert!(signals.insert(observer.signal));
        for id in observer.operands {
            assert!(operands.insert(id));
            let probe = linked.plan.analog_probe(id).unwrap();
            assert_eq!(probe.quantity, DigitalAnalogQuantity::RealVariable);
            assert!(!probe.retained && probe.event_signal.is_none());
            let DigitalAnalogProbeTarget::Variable { name } = &probe.target else {
                panic!("operand must name evaluated analog storage")
            };
            assert!(name.starts_with("left.") || name.starts_with("right."));
        }
    }
    linked.plan.validate().unwrap();
    let mut corrupted = linked.plan.clone();
    corrupted.absdelta[0].operands[0] =
        rspice_veriloga::canonical_ir::ids::DigitalAnalogProbeId::new(u32::MAX);
    assert!(corrupted.validate().is_err());
}

#[test]
fn absdelta_invalid_source_is_explicit() {
    use rspice_veriloga::{CompilerOptions, VerilogACompiler};
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_core::xspice::verilog::MixedSignalHost;
    let source = "module observer(a); input a; electrical a; real sampled=0; always @(absdelta(V(a),0.1)) sampled=V(a); endmodule";
    MixedSignalHost::compile(source, None, "observer", &[1], SchedulerLimits::default())
        .expect("standalone observers use the shared coordinator");
    for event in [
        "absdelta(V(a))",
        "absdelta(,0.1)",
        "absdelta(V(a),)",
        "absdelta(V(a),0.1,0,0,1,2)",
        "posedge absdelta(V(a),0.1)",
    ] {
        let invalid = source.replace("absdelta(V(a),0.1)", event);
        assert!(
            VerilogACompiler::new(CompilerOptions::default())
                .compile_canonical_ir(&invalid)
                .is_err(),
            "accepted {event}"
        );
    }
}

#[test]
fn absdelta_shared_circuit_samples_interpolated_values_and_resolves_feedback() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module observer(a,p,t);
 input a; electrical a;
 inout p,t; electrical p,t;
 parameter real delta=0.25;
 real sampled=0,last=0; integer count=0,clock=0,seen_clock=0;
 initial #650 clock=1;
 always @(absdelta(V(a),delta,1p,1u)) begin
   sampled=V(a); last=$abstime; count=count+1; seen_clock=clock;
 end
 analog begin I(p)<+(V(p)-(10*count+sampled))/1000; I(t)<+(V(t)-(last*1e9+10*seen_clock))/1000; end
endmodule
module wrapper(a,p,q,t,u);
 input a; electrical a;
 inout p,q,t,u; electrical p,q,t,u;
 observer #(.delta(0.25)) first(a,p,t);
 observer #(.delta(0.4)) second(a,q,u);
endmodule
"#,
    );
    let deck=Netlist::parse(&format!("* interpolated observers and feedback\nV1 a 0 PWL(0 0 1n 1)\nX1 a p q t u wrapper\nRp p 0 1k\nRq q 0 1k\nRt t 0 1k\nRu u 0 1k\n.va \"{}\" wrapper module=wrapper\n.end\n",source.path())).unwrap();
    let result = Engine::default().run_tran(&deck, 1.1e-9, 170e-12).unwrap();
    for (time, p, q, t, u) in [
        (0.2e-9, 5.0, 5.0, 0.0, 0.0),
        (0.3e-9, 10.125, 5.0, 0.125, 0.0),
        (0.55e-9, 15.25, 10.2, 0.25, 0.2),
        (0.85e-9, 20.375, 15.4, 5.375, 5.4),
        (1.05e-9, 25.5, 15.4, 5.5, 5.4),
    ] {
        for (node, expected) in [("p", p), ("q", q), ("t", t), ("u", u)] {
            let index = result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(node))
                .unwrap();
            let point = result
                .time
                .iter()
                .rposition(|value| *value <= time)
                .unwrap();
            let actual = result.voltage_waveform(index + 1)[point];
            assert!(
                (actual - expected).abs() < 1e-4,
                "{node}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn absdelta_digital_only_observation_does_not_force_analog_sampling_steps() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module observer(a,ok,done,r);
 input a; electrical a;
 output ok,done; reg ok=1,done=0;
 output r; wreal r;
 assign r=sampled;
 real derived,sampled,observed; integer count=0;
 analog derived=2*V(a);
 always @(absdelta(V(a),0.125,1p,1u)) begin
   sampled=V(a); observed=derived; count=count+1;
   if (sampled-$abstime*1e9>1u || $abstime*1e9-sampled>1u) ok=0;
   if (observed-2*$abstime*1e9>2u || 2*$abstime*1e9-observed>2u) ok=0;
   if (count==9) done=1;
   if (count>9) ok=0;
 end
endmodule
module flag_reader(a,ok,done,r);
 input a; electrical a;
 input ok,done; input r; wreal r;
 integer flags=0;
 always @(ok or done) flags=ok+2*done;
 analog I(a)<+0;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* pure observer\nV1 a 0 PWL(0 0 1n 1)\nX1 a ok done r observer\nX2 a ok done r flag_reader\n.va \"{}\" observer module=observer\n.va \"{}\" flag_reader module=flag_reader\n.end\n",
        source.path(), source.path()
    ))
    .unwrap();
    let captured = AcceptedEvents::default();
    let result = Engine::default()
        .run_tran_with_abort(&deck, 1.1e-9, 400e-12, &captured)
        .unwrap();
    assert!(
        result
            .time
            .windows(2)
            .any(|pair| pair[1] - pair[0] > 150e-12),
        "observer must leave intervals spanning multiple sample events: {:?}",
        result.time
    );
    let real = result.real_trace_named("r").unwrap();
    let published = captured.0.lock().unwrap();
    let live: Vec<_> = published
        .iter()
        .filter_map(|(name, change)| {
            if !name.eq_ignore_ascii_case("r") {
                return None;
            }
            match change.value {
                rspice_core::abort_signal::TransientEventValue::Real(value) => {
                    Some((change.time, value))
                }
                _ => None,
            }
        })
        .collect();
    assert_eq!(
        live,
        real.iter()
            .map(|point| (point.time, point.value))
            .collect::<Vec<_>>()
    );
    let changes: Vec<_> = real.iter().filter(|point| point.time > 20e-12).collect();
    assert_eq!(changes.len(), 8, "{real:?}");
    for (index, point) in changes.iter().enumerate() {
        let expected = (index + 1) as f64 * 0.125;
        assert!((point.time - expected * 1e-9).abs() < 1e-15, "{real:?}");
        assert!((point.value - expected).abs() < 1e-6, "{real:?}");
    }
    for node in ["ok", "done"] {
        let trace = result
            .digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("missing {node} trace: {:?}", result.digital_traces));
        assert_eq!(
            trace.points.last().unwrap().value.state,
            rspice_core::xspice::DigitalState::One,
            "{node}: {:?}",
            trace.points
        );
    }
}

#[test]
fn absdelta_xspice_delays_interleave_and_physical_fanout_refines() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module sampler(a,q,valid);
 input a; electrical a;
 input valid; integer seen_valid=0;
 always @(valid) seen_valid=valid;
 output q; reg q=0;
 always @(absdelta(V(a),0.125,1p,1u)) q=~q;
endmodule
module receiver(a,b,c,valid);
 input a; electrical a;
 input b,c; output valid; reg valid=0;
 parameter real ADC_DELAY=0;
 integer count=0,adc_count=0,bad=0; real last=0,last_adc=0,sampled=0;
 always @(c) if ($abstime>20p) begin adc_count=adc_count+1; last_adc=$abstime; end
 analog I(a)<+0;
 always @(b) if ($abstime>20p) begin
   count=count+1; last=$abstime; sampled=V(a);
   if ($abstime<1n && (sampled-$abstime*1e9>1u || $abstime*1e9-sampled>1u)) bad=1;
 end
 initial begin
   #1080;
   valid=(count==8 && adc_count==8 && bad==0 && last-1017p<=1f && 1017p-last<=1f
      && last_adc-(1017p+ADC_DELAY)<=1f && (1017p+ADC_DELAY)-last_adc<=1f);
 end
endmodule
"#,
    );
    for physical in [false, true] {
        let dac = if physical {
            "Adac [b] [out] dac\n.model dac dac_bridge(out_low=0 out_high=1 t_rise=1p t_fall=1p)\nRload out 0 1k\nAadc [out] [returned] adc\n.model adc adc_bridge(in_low=0.5 in_high=0.5 rise_delay=1p fall_delay=1p)\n"
        } else {
            ""
        };
        let (returned, adc_delay) = if physical {
            ("returned", "1.5p")
        } else {
            ("b", "0")
        };
        let deck = Netlist::parse(&format!(
            "* interpolated HDL/XSPICE timeline\nV1 a 0 PWL(0 0 1n 1)\nXsample a q valid sampler\nAbuf q b buffer\nXreceive a b {returned} valid receiver ADC_DELAY={adc_delay}\n.model buffer d_buffer(rise_delay=17p fall_delay=17p)\n{dac}.va \"{}\" sampler module=sampler\n.va \"{}\" receiver module=receiver\n.end\n",
            source.path(),source.path()
        )).unwrap();
        let captured = AcceptedEvents::default();
        let result = Engine::default()
            .run_tran_with_abort(&deck, 1.1e-9, 400e-12, &captured)
            .unwrap_or_else(|error| panic!("physical={physical}: {error}"));
        let published = captured.0.lock().unwrap();
        for trace in &result.digital_traces {
            let live: Vec<_> = published
                .iter()
                .filter_map(|(name, change)| {
                    if name != &trace.node_name {
                        return None;
                    }
                    match change.value {
                        rspice_core::abort_signal::TransientEventValue::Digital(code) => {
                            Some((change.time, code.0))
                        }
                        _ => None,
                    }
                })
                .collect();
            let retained: Vec<_> = trace
                .points
                .iter()
                .map(|point| (point.time, point.value.event_code()))
                .collect();
            assert_eq!(live, retained, "physical={physical}, {}", trace.node_name);
        }
        let valid = result
            .digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case("valid"))
            .unwrap();
        assert_eq!(
            valid.points.last().unwrap().value.state,
            rspice_core::xspice::DigitalState::One,
            "physical={physical}: {:?}",
            result.digital_traces
        );
        for (node, delay) in [
            ("q", 0.0),
            ("b", 17e-12),
            (returned, 17e-12 + if physical { 1.5e-12 } else { 0.0 }),
        ] {
            let trace = result.digital_trace_named(node).unwrap();
            let changes: Vec<_> = trace.iter().filter(|point| point.time > 20e-12).collect();
            assert_eq!(changes.len(), 8, "physical={physical}, {node}: {trace:?}");
            for (index, point) in changes.iter().enumerate() {
                let expected_time = (index + 1) as f64 * 125e-12 + delay;
                let expected_state = if index % 2 == 0 {
                    rspice_core::xspice::DigitalState::Zero
                } else {
                    rspice_core::xspice::DigitalState::One
                };
                assert!(
                    (point.time - expected_time).abs() < 1e-15,
                    "physical={physical}, {node}: {trace:?}"
                );
                assert_eq!(
                    point.value.state, expected_state,
                    "physical={physical}, {node}: {trace:?}"
                );
            }
        }
        if physical {
            for (time, expected) in [(0.2e-9, 0.0), (0.3e-9, 1.0), (0.45e-9, 0.0), (0.57e-9, 1.0)] {
                let node = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("out"))
                    .unwrap();
                let point = result
                    .time
                    .iter()
                    .rposition(|value| *value <= time)
                    .unwrap();
                let actual = result.voltage_waveform(node + 1)[point];
                assert!(
                    (actual - expected).abs() < 1e-4,
                    "out@{time}: {actual} != {expected}"
                );
            }
        } else {
            assert!(
                result
                    .time
                    .windows(2)
                    .any(|pair| pair[1] - pair[0] > 150e-12),
                "digital propagation must not force a sample grid: {:?}",
                result.time
            );
        }
    }
}

#[test]
fn explicit_adc_roots_attach_to_real_only_mixed_domains() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module real_source(r);
 output r; wreal r;
 real level=0;
 initial begin #125 level=1; #125 level=0; end
 assign r=level;
endmodule
"#,
    );
    let deck=Netlist::parse(&format!(
        "* real-only mixed domain with explicit converter\nXreal r real_source\nAconvert r converted rv\n.model rv real_to_v(transition_time=1p)\nRload converted 0 1k\nAadc [converted] [d] adc\n.model adc adc_bridge(in_low=0.5 in_high=0.5 rise_delay=1p fall_delay=1p)\n.va \"{}\" real_source\n.end\n",source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 300e-12, 70e-12).unwrap();
    let trace = result.digital_trace_named("d").unwrap();
    let rise = trace
        .iter()
        .find(|point| point.value.state == rspice_core::xspice::DigitalState::One)
        .unwrap();
    let fall = trace
        .iter()
        .find(|point| {
            point.time > rise.time && point.value.state == rspice_core::xspice::DigitalState::Zero
        })
        .unwrap();
    assert!((rise.time - 126.5e-12).abs() < 1e-15, "{trace:?}");
    assert!((fall.time - 251.5e-12).abs() < 1e-15, "{trace:?}");
}


#[test]
fn absdelta_interval_budget_reaches_engine_execution_as_a_resource_error() {
    let source = Source::new(
        r#"
module observer(a);
 input a; electrical a; integer count=0;
 always @(absdelta(V(a),0.125)) count=count+1;
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* interval budget\nV1 a 0 PWL(0 0 1n 1)\nX1 a observer\n.va \"{}\" observer module=observer\n.end\n", source.path()
    )).unwrap();
    let mut config = rspice_core::SimulationConfig::default();
    config.resource_limits.max_mixed_interval_events = 0;
    let error = Engine::new(config)
        .run_tran(&deck, 1e-9, 400e-12)
        .unwrap_err();
    assert!(
        matches!(error, rspice_core::SimulationError::ResourceLimit(error)
        if error.resource == rspice_core::ResourceKind::MixedIntervalEvents
            && error.limit == 0 && error.requested == 1),
        "{error}"
    );
}

fn settle_standalone_observer(
    host: &mut rspice_core::xspice::verilog::MixedSignalHost,
    solution: &[f64],
) {
    for _ in 0..8 {
        host.stamp(solution, |_, _, _| {}, |_, _| {}).unwrap();
        if !host.settle_analog_bridges(solution).unwrap() {
            return;
        }
    }
    panic!("standalone observer did not settle");
}

#[test]
fn absdelta_standalone_interpolation_inputs_rollback_and_checkpoint() {
    use rspice_core::abort_signal::CountingAbort;
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_core::xspice::verilog::{MixedSignalHost, MixedSignalError};
    use rspice_veriloga::vm::IntegrationCoefficients;
    let source = r#"
`timescale 1ps/1ps
module observer(a,drive);
 input a; electrical a; input drive; wire drive;
 integer count=0, valid=1, early=0, edges=0, future=0;
 real sampled;
 initial #1500 future=1;
 always @(posedge drive) edges=edges+1;
 always @(absdelta(V(a),0.125,1p,1u)) begin
   count=count+1; sampled=V(a);
   if (sampled-$abstime*1e9>1u || $abstime*1e9-sampled>1u) valid=0;
   if (drive===1'b1 && $abstime<999p) early=early+1;
 end
endmodule
"#;
    let compile = || {
        MixedSignalHost::compile(source, None, "observer", &[1], SchedulerLimits::default())
            .unwrap()
    };
    let count = |host: &MixedSignalHost, name| {
        u32::from_str_radix(&host.read_digital(name).unwrap(), 2).unwrap()
    };
    let begin = |host: &mut MixedSignalHost, time, dt| {
        host.begin_trial(
            time,
            dt,
            IntegrationCoefficients::inactive(),
            time == 0.0,
            false,
        )
        .unwrap()
    };
    let mut host = compile();
    begin(&mut host, 0.0, 0.0);
    host.force_digital(&[("drive", "0")]).unwrap();
    settle_standalone_observer(&mut host, &[0.0]);
    host.accept_trial().unwrap();
    assert_eq!(count(&host, "count"), 1);
    let initial = host.checkpoint().unwrap();
    assert_eq!(host.next_event_time().unwrap(), Some(1.5e-9));

    begin(&mut host, 1e-9, 1e-9);
    host.force_digital(&[("drive", "1")]).unwrap();
    host.force_digital(&[("drive", "0")]).unwrap();
    host.force_digital(&[("drive", "1")]).unwrap();
    settle_standalone_observer(&mut host, &[1.0]);
    assert_eq!(count(&host, "count"), 9);
    assert_eq!(count(&host, "valid"), 1);
    assert_eq!(
        count(&host, "early"),
        0,
        "endpoint drives cannot leak into earlier observations"
    );
    assert_eq!(
        count(&host, "edges"),
        2,
        "successive same-time input banks keep both edges"
    );
    host.reject_trial().unwrap();
    assert_eq!(count(&host, "count"), 1);
    assert_eq!(count(&host, "edges"), 0);
    assert_eq!(host.next_event_time().unwrap(), Some(1.5e-9));

    begin(&mut host, 1e-9, 1e-9);
    let abort = CountingAbort::new(4);
    let error = host
        .settle_analog_bridges_with_abort(&[1.0], &abort)
        .unwrap_err();
    assert!(matches!(error, MixedSignalError::Aborted), "{error}");
    assert_eq!(abort.polls_after_abort(), 0);
    host.reject_trial().unwrap();
    assert_eq!(count(&host, "count"), 1);

    host.set_observer_interval_event_limit(2).unwrap();
    begin(&mut host, 1e-9, 1e-9);
    let error = host.settle_analog_bridges(&[1.0]).unwrap_err();
    assert!(
        matches!(error,MixedSignalError::ResourceLimit(error) if error.limit==2),
        "{error}"
    );
    host.reject_trial().unwrap();
    host.set_observer_interval_event_limit(100).unwrap();
    begin(&mut host, 1e-9, 1e-9);
    host.force_digital(&[("drive", "1")]).unwrap();
    settle_standalone_observer(&mut host, &[1.0]);
    host.accept_trial().unwrap();
    let accepted = host.checkpoint().unwrap();
    let mut restored = compile();
    restored.restore(&accepted).unwrap();
    for candidate in [&mut host, &mut restored] {
        begin(candidate, 1.5e-9, 0.5e-9);
        settle_standalone_observer(candidate, &[1.5]);
        candidate.accept_trial().unwrap();
        for (name, expected) in [
            ("count", 13),
            ("valid", 1),
            ("early", 0),
            ("edges", 1),
            ("future", 1),
        ] {
            assert_eq!(count(candidate, name), expected, "{name}");
        }
        assert_eq!(candidate.next_event_time().unwrap(), None);
    }
    restored.set_observer_interval_event_limit(0).unwrap();
    restored.restore(&initial).unwrap();
    assert_eq!(count(&restored, "count"), 1);
    assert_eq!(restored.next_event_time().unwrap(), Some(1.5e-9));
    begin(&mut restored, 125e-12, 125e-12);
    let error = restored.settle_analog_bridges(&[0.125]).unwrap_err();
    assert!(
        matches!(error,MixedSignalError::ResourceLimit(error) if error.limit==0),
        "restore must preserve the receiver's resource policy: {error}"
    );
    restored.reject_trial().unwrap();
}

#[test]
fn absdelta_standalone_feedback_reports_a_refinement_before_acceptance() {
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_core::xspice::verilog::MixedSignalHost;
    use rspice_veriloga::vm::IntegrationCoefficients;
    let source = r#"
`timescale 1ps/1ps
module observer(a,q);
 input a; electrical a; output q; reg q=0;
 always @(absdelta(V(a),0.125,1p,1u)) if ($abstime>0) q=~q;
endmodule
"#;
    let mut host =
        MixedSignalHost::compile(source, None, "observer", &[1], SchedulerLimits::default())
            .unwrap();
    host.add_dac_bridge("q", 0, (2, 0), 0.0, 1.0, 1000.0)
        .unwrap();
    let begin = |host: &mut MixedSignalHost, time, dt| {
        host.begin_trial(
            time,
            dt,
            IntegrationCoefficients::inactive(),
            time == 0.0,
            false,
        )
        .unwrap()
    };
    begin(&mut host, 0.0, 0.0);
    settle_standalone_observer(&mut host, &[0.0, 0.0]);
    host.accept_trial().unwrap();
    begin(&mut host, 1e-9, 1e-9);
    settle_standalone_observer(&mut host, &[1.0, 1.0]);
    let root = host.trial_boundary_refinement_time(1e-15).unwrap().unwrap();
    assert!((root - 125e-12).abs() < 1e-18, "{root}");
    assert!(
        host.accept_trial().is_err(),
        "an interior analog change requires a new solve"
    );
    host.reject_trial().unwrap();
    assert_eq!(host.read_digital("q").unwrap(), "0");
    begin(&mut host, root, root);
    settle_standalone_observer(&mut host, &[0.125, 1.0]);
    assert_eq!(host.trial_boundary_refinement_time(1e-15).unwrap(), None);
    let mut rhs = 0.0;
    host.stamp(
        &[0.125, 1.0],
        |_, _, _| {},
        |node, value| {
            if node == 1 {
                rhs += value;
            }
        },
    )
    .unwrap();
    assert!((rhs - 1e-3).abs() < 1e-15, "{rhs}");
    host.accept_trial().unwrap();
    assert_eq!(host.read_digital("q").unwrap(), "1");
}

#[test]
fn local_event_operands_cross_timer_and_scope_survive_hierarchy() {
    let source = Source::new(
        r#"
`timescale 1ps/1ps
module local_detector(a,out);
 input a; electrical a; inout out; electrical out;
 parameter real BASE=0.125;
 real threshold=99;
 integer hits=0, nested_hits=0, ticks=0;
 initial begin : watch_input
   real threshold=BASE;
   integer direction=1;
   real tolerance=1p;
   reg [1:0] enabled=1;
   repeat (2) begin
     @(cross(V(a)-threshold,direction,tolerance,1u,enabled[0])) begin
       hits=hits+1; threshold=threshold+0.25;
     end
   end
   begin : nested
     real threshold=BASE+0.625;
     @(cross(V(a)-threshold,1,1p,1u)) nested_hits=nested_hits+1;
   end
 end
 initial begin : clocked
   real start=125p, period=500p;
   repeat (3) @(timer(start,period,1p,1)) ticks=ticks+1;
 end
 analog I(out)<+(V(out)-(hits+10*nested_hits+100*ticks))/1000;
endmodule
module local_wrapper(a,p,q);
 input a; electrical a; inout p,q; electrical p,q;
 local_detector first(a,p);
 local_detector #(.BASE(0.2)) second(a,q);
endmodule
"#,
    );
    let deck = Netlist::parse(&format!(
        "* local event operand bindings\nV1 a 0 pwl(0 0 1n 1)\nX1 a p q local_wrapper\nRp p 0 1k\nRq q 0 1k\n.va \"{}\" local_wrapper module=local_wrapper\n.end\n", source.path()
    )).unwrap();
    let result = Engine::default().run_tran(&deck, 0.95e-9, 70e-12).unwrap();
    for (name, expected_times) in [
        ("p", vec![0.0, 125e-12, 375e-12, 625e-12, 750e-12]),
        ("q", vec![0.0, 125e-12, 200e-12, 450e-12, 625e-12, 825e-12]),
    ] {
        let node = result
            .node_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap();
        let wave = result.voltage_waveform(node + 1);
        let times: Vec<_> = result
            .time
            .iter()
            .zip(wave.iter())
            .enumerate()
            .filter(|(i, (_, v))| *i == 0 || (**v - wave[*i - 1]).abs() > 0.25)
            .map(|(_, (&t, _))| t)
            .collect();
        assert_eq!(times.len(), expected_times.len(), "{name}: {times:?}");
        for (&actual, expected) in times.iter().zip(expected_times) {
            assert!(
                (actual - expected).abs() < 2e-15,
                "{name}: {actual} != {expected}"
            );
        }
    }
    for (time, p, q) in [
        (0.15e-9, 50.5, 50.0),
        (0.4e-9, 51.0, 50.5),
        (0.65e-9, 101.0, 101.0),
        (0.79e-9, 106.0, 101.0),
        (0.88e-9, 106.0, 106.0),
    ] {
        for (name, expected) in [("p", p), ("q", q)] {
            let node = result
                .node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .unwrap();
            let sample = result.time.iter().rposition(|t| *t <= time).unwrap();
            let actual = result.voltage_waveform(node + 1)[sample];
            assert!(
                (actual - expected).abs() < 1e-5,
                "{name}@{time}: {actual} != {expected}"
            );
        }
    }
}

#[test]
fn local_event_operands_absdelta_arrays_restore_and_resume() {
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_core::xspice::verilog::MixedSignalHost;
    use rspice_veriloga::vm::IntegrationCoefficients;
    let source = r#"
`timescale 1ps/1ps
module local_observer(a);
 input a; electrical a;
 integer count=0;
 initial begin : observe
   real step[3:2]='{0.25,0.5};
   integer index=3;
   reg [1:0] enabled=1;
   forever begin
     @(absdelta(V(a),step[index],1p,1u,enabled[0])) begin
       count=count+1;
       if (count==3) index<=2;
     end
   end
 end
endmodule
"#;
    let compile = || {
        MixedSignalHost::compile(
            source,
            None,
            "local_observer",
            &[1],
            SchedulerLimits::default(),
        )
        .unwrap()
    };
    let mut host = compile();
    let begin = |host: &mut MixedSignalHost, time, dt| {
        host.begin_trial(
            time,
            dt,
            IntegrationCoefficients::inactive(),
            time == 0.0,
            false,
        )
        .unwrap()
    };
    let count = |host: &MixedSignalHost| {
        u32::from_str_radix(&host.read_digital("count").unwrap(), 2).unwrap()
    };
    begin(&mut host, 0.0, 0.0);
    settle_standalone_observer(&mut host, &[0.0]);
    host.accept_trial().unwrap();
    assert_eq!(count(&host), 1);
    begin(&mut host, 0.5e-9, 0.5e-9);
    settle_standalone_observer(&mut host, &[0.5]);
    assert_eq!(count(&host), 3);
    host.reject_trial().unwrap();
    assert_eq!(count(&host), 1);
    begin(&mut host, 0.5e-9, 0.5e-9);
    settle_standalone_observer(&mut host, &[0.5]);
    host.accept_trial().unwrap();
    assert_eq!(count(&host), 3);
    let checkpoint = host.checkpoint().unwrap();
    let mut resumed = compile();
    resumed.restore(&checkpoint).unwrap();
    for candidate in [&mut host, &mut resumed] {
        begin(candidate, 1e-9, 0.5e-9);
        settle_standalone_observer(candidate, &[1.0]);
        candidate.accept_trial().unwrap();
        assert_eq!(count(candidate), 4);
    }
}

#[test]
fn local_event_operands_do_not_capture_shadowed_electrical_nodes() {
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_core::xspice::verilog::MixedSignalHost;
    for function in ["cross(V(a),1)", "absdelta(V(a),0.1)"] {
        let source = format!(
            "module local_shadow(a); input a; electrical a; initial begin real a=0; @({function}) a=1; end endmodule"
        );
        let error = MixedSignalHost::compile(
            &source,
            None,
            "local_shadow",
            &[1],
            SchedulerLimits::default(),
        )
        .err()
        .expect("a local cannot name a probe node")
        .to_string();
        assert!(
            error.contains("analog access names process-local storage"),
            "{error}"
        );
    }
}

#[test]
fn local_event_operands_retract_counter_without_replaying_and_survive_rejection() {
    use rspice_core::xspice::event_scheduler::SchedulerLimits;
    use rspice_core::xspice::verilog::MixedSignalHost;
    use rspice_veriloga::vm::IntegrationCoefficients;
    let source = r#"
`timescale 1ps/1ps
module local_above(a);
 input a; electrical a; integer count=0;
 initial begin
   real threshold=0.25;
   repeat(2) @(above(V(a)-threshold)) begin
     count=count+1; threshold=threshold+0.25;
   end
 end
endmodule
"#;
    let mut host = MixedSignalHost::compile(
        source,
        None,
        "local_above",
        &[1],
        SchedulerLimits::default(),
    )
    .unwrap();
    let begin = |host: &mut MixedSignalHost, time, dt| {
        host.begin_trial(
            time,
            dt,
            IntegrationCoefficients::inactive(),
            time == 0.0,
            false,
        )
        .unwrap()
    };
    let count = |host: &MixedSignalHost| {
        u32::from_str_radix(&host.read_digital("count").unwrap(), 2).unwrap()
    };
    begin(&mut host, 0.0, 0.0);
    settle_standalone_observer(&mut host, &[0.0]);
    host.accept_trial().unwrap();
    for reject in [true, false] {
        begin(&mut host, 1e-9, 1e-9);
        settle_standalone_observer(&mut host, &[0.25]);
        assert_eq!(count(&host), 1);
        if reject {
            host.reject_trial().unwrap();
            assert_eq!(count(&host), 0);
        } else {
            host.accept_trial().unwrap();
        }
    }
    let checkpoint = host.checkpoint().unwrap();
    for replay in 0..2 {
        if replay == 1 {
            host.restore(&checkpoint).unwrap();
        }
        begin(&mut host, 2e-9, 1e-9);
        settle_standalone_observer(&mut host, &[0.5]);
        host.accept_trial().unwrap();
        assert_eq!(count(&host), 2);
    }
}

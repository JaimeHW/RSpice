use rspice_veriloga::canonical_ir::CanonicalIrArtifact;
use rspice_veriloga::{
    CompilerOptions, VerilogACompiler, VirtualCompileLimits, VirtualSourceBundle, VirtualSourceFile,
};
mod support;

#[test]
fn module_time_queries_preserve_scope_defaults_types_and_unused_fallbacks() {
    use rspice_veriloga_runtime::{GeneratedSimulationParameters, SimulationParameter};
    let compiler = VerilogACompiler::default();
    let report = compiler
        .compile_runtime(include_str!("testdata/module_time_queries.va"), Some("top"))
        .unwrap();
    let fixture = support::DeviceFixture {
        model: report.model,
        canonical_ir: report.canonical_ir,
    };
    let mut device = fixture.device("timing", &[1]);
    let mut environment = GeneratedSimulationParameters::default();
    environment
        .try_set(SimulationParameter::TimeUnit, Some(17.0))
        .unwrap();
    environment
        .try_set(SimulationParameter::TimePrecision, Some(23.0))
        .unwrap();
    device.set_simulation_parameters(environment);
    for time in [0.0, 2e-9, 7.25e-9] {
        device.try_set_time(time).unwrap();
        let mut conductance = 0.0;
        let mut rhs = 0.0;
        device
            .try_stamp(
                &[1.0],
                |row, column, value| {
                    assert_eq!((row, column), (0, 0));
                    conductance += value;
                },
                |row, value| {
                    assert_eq!(row, 0);
                    rhs += value;
                },
            )
            .unwrap();
        let expected = 4.0 + time / 1e-9 + time / 1e-8;
        assert!(
            (conductance - expected).abs() < 1e-12,
            "t={time:e}: {conductance}, expected {expected}"
        );
        assert!(
            rhs.abs() < 1e-12,
            "a linear conductance has no companion offset"
        );
    }
    // A seconds-sized query is still real-valued, so division must not become
    // integer division merely because its resolved value is a whole number.
    let model = support::DeviceFixture::compile(
        "`timescale 1s/1ms\nmodule whole(p); inout p; electrical p; analog I(p)<+$simparam(\"timeUnit\")/2; endmodule",
    );
    assert_eq!(model.device("whole", &[1]).try_evaluate().unwrap(), [0.5]);
    for expression in [
        "$realtime(1)",
        "$simparam(\"timeUnit\",0,1)",
        "$simparam(\"timePrecision\",missing)",
        "$simparam(\"timePrecision\",1 ? 0 : missing+1)",
        "$simparam(\"timePrecision\",sin())",
        "$simparam(\"timeUnit\",\"bad\")",
    ] {
        let source = format!(
            "module invalid(p); inout p; electrical p; analog I(p)<+{expression}; endmodule"
        );
        assert!(
            compiler.compile_runtime(&source, None).is_err(),
            "{expression}"
        );
    }
}

fn delay_ticks(artifact: &CanonicalIrArtifact) -> Vec<i64> {
    use rspice_veriloga::canonical_ir::{
        DigitalSignalId, digital_eval::*, digital_value::FourStateValue, ids::DigitalAnalogProbeId,
    };
    #[derive(Default)]
    struct Store(std::collections::BTreeMap<DigitalSignalId, FourStateValue>);
    impl DigitalEnvironment for Store {
        fn read_clock(&self) -> Option<DigitalClock> {
            None
        }
        fn read_signal(&self, id: DigitalSignalId) -> Option<FourStateValue> {
            self.0.get(&id).cloned()
        }
        fn write_signal(&mut self, id: DigitalSignalId, value: FourStateValue) {
            self.0.insert(id, value);
        }
        fn defer_update(&mut self, _: DigitalDeferredUpdate) {
            panic!("fixture must not defer writes")
        }
        fn read_real_signal(&self, _: DigitalSignalId) -> Option<f64> {
            None
        }
        fn write_real_signal(&mut self, _: DigitalSignalId, _: f64) {
            panic!("fixture must not write reals")
        }
        fn read_analog_potential(&self, _: DigitalAnalogProbeId) -> Option<f64> {
            None
        }
        fn drive_real_signal(&mut self, _: DigitalRealDrive) {
            panic!("fixture must not drive nets")
        }
        fn drive_signal(&mut self, _: DigitalDrive) {
            panic!("fixture must not drive nets")
        }
    }
    let plan = &artifact.digital;
    let mut store = Store::default();
    for signal in &plan.signals {
        store
            .0
            .insert(signal.id, FourStateValue::from_u64(signal.width, 0));
    }
    let mut delays = Vec::new();
    // These fixtures put delays in initial processes. Port drivers belong to
    // the host's resolver and are not needed to observe those suspensions.
    for process in plan.processes.iter().filter(|process| {
        matches!(
            process.kind,
            rspice_veriloga::canonical_ir::DigitalProcessKind::Initial
        )
    }) {
        let mut outcome = start(plan, process, &mut store).unwrap();
        for _ in 0..128 {
            let DigitalProcessOutcome::Suspended(suspension) = outcome else {
                break;
            };
            let DigitalWaitRequest::Delay(ticks) = suspension.wait() else {
                panic!("expected delay")
            };
            delays.push(*ticks);
            outcome = resume(plan, process, suspension.resume_state(), &mut store).unwrap();
        }
        assert!(matches!(outcome, DigitalProcessOutcome::Finished));
    }
    delays
}

#[test]
fn timescale_include_scope_rounding_hierarchy_and_transport_share_one_contract() {
    let child = "`timescale 10ns/100ps\nmodule child(q); output q; reg q; initial begin q=0; #0.125 q=1; end endmodule\n";
    let root = "`include \"child.vh\"\n`ifdef NEVER_ACTIVE\n`timescale invalid\n`endif\n`timescale 1ns/1ps\nmodule top(q,c); output q; reg q; output c; wire c; child u(c); parameter real D=0.4505; initial begin q=0; #D q=1; end endmodule\n`resetall\nmodule legacy(q); output q; reg q; initial #1 q=1; endmodule\n";
    let bundle = VirtualSourceBundle::new(
        "top.va",
        [
            VirtualSourceFile::new("top.va", root),
            VirtualSourceFile::new("child.vh", child),
        ],
    )
    .unwrap();
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..Default::default()
    });
    let prepared = compiler
        .prepare_virtual_runtime_source(&bundle, VirtualCompileLimits::default())
        .unwrap();
    let top = prepared.compile_runtime("top").unwrap();
    let artifact = &top.runtime.canonical_ir;
    assert_eq!(artifact.digital.timing.root.unit_exponent(), -9);
    assert_eq!(artifact.digital.timing.precision_exponent, -12);
    assert_eq!(delay_ticks(artifact), [451, 1300]);
    assert_eq!(artifact.digital.processes[1].time_scale.unit_exponent(), -8);
    let legacy = prepared.compile_runtime("legacy").unwrap();
    assert_eq!(
        legacy
            .runtime
            .canonical_ir
            .digital
            .timing
            .precision_exponent,
        -9
    );
    assert_eq!(delay_ticks(&legacy.runtime.canonical_ir), [1]);

    let decoded: CanonicalIrArtifact =
        serde_json::from_slice(&serde_json::to_vec(artifact).unwrap()).unwrap();
    decoded.digital.validate().unwrap();
    assert_eq!(decoded.digital, artifact.digital);
    let mut changed = decoded.digital.clone();
    changed.timing.precision_exponent = -13;
    assert!(
        changed.validate().is_err(),
        "timing participates in content identity"
    );
    let mut missing = serde_json::to_value(&decoded.digital).unwrap();
    missing.as_object_mut().unwrap().remove("timing");
    assert!(
        serde_json::from_value::<rspice_veriloga::canonical_ir::CanonicalDigitalPlan>(missing)
            .is_err()
    );
}

#[test]
fn timescale_delays_round_before_rescaling_and_never_clamp() {
    let compiler = VerilogACompiler::default();
    for (scale, delay, expected) in [
        ("1ns/100ps", "0.049", 0),
        ("1ns/100ps", "0.05", 1),
        ("1ns/100ps", "0.15", 2),
        ("1ns/1ps", "1.25", 1250),
        ("100s/10s", "1.25", 13),
        ("1ns/1ns", "2147483648", 2147483648),
        ("1ns/1ns", "9007199254740992+1", 9007199254740993),
    ] {
        let source = format!(
            "`timescale {scale}\nmodule timed(q); output q; reg q; initial #({delay}) q=1; endmodule"
        );
        let artifact = compiler.compile_canonical_ir(&source).unwrap();
        assert_eq!(delay_ticks(&artifact), [expected], "{scale}, {delay}");
    }
    for (scale, delay) in [("2ns/1ps", "1"), ("1ns/1us", "1"), ("1ns/1ps junk", "1")] {
        let source = format!(
            "`timescale {scale}\nmodule timed(q); output q; reg q; initial #({delay}) q=1; endmodule"
        );
        assert!(
            compiler.compile_canonical_ir(&source).is_err(),
            "{scale}, {delay}"
        );
    }
}

/// `$realtime` is one rewrite, not four lowerings.
///
/// Semantic analysis rewrites `$realtime` into `$abstime / <the module's time
/// unit>` while that module's own `` `timescale `` is in scope
/// (`SemanticAnalyzer::lower_module_time_function`). Everything below it — the
/// bytecode the interpreter executes, the canonical CFG the JITs compile, and
/// the direct generated Rust — therefore only ever sees `$abstime`, and none
/// of them carries a time unit to scale by.
///
/// This is the probe that makes that a measurement rather than a reading. The
/// module declares a `1us` time unit, so a route that took `$realtime` for
/// plain seconds is wrong by a factor of a million, and it reads `$realtime`
/// from the places a rewrite could plausibly miss: a contribution, a
/// statically unrolled loop body, a guarded branch, and the body of an analog
/// function that is inlined into the block. Every route is measured against
/// the same module with the division spelled out by hand, which is the only
/// oracle that separates "scaled" from "scaled the same way twice".
#[test]
fn every_lowering_route_reads_realtime_through_the_module_time_unit() {
    use rspice_veriloga::canonical_ir::{CfgEvalInputs, CfgModel, evaluate_cfg};
    use rspice_veriloga::rust_backend::RustTranspiler;

    /// The module's own time unit, in seconds.
    const UNIT: f64 = 1.0e-6;

    fn probe(time: &str) -> String {
        format!(
            "`timescale 1us/1ns\n\
             module realtime_routes(p, n);\n\
             inout p, n; electrical p, n;\n\
             real held; integer i;\n\
             analog function real elapsed;\n\
             input k; real k;\n\
             elapsed = k + {time};\n\
             endfunction\n\
             analog begin\n\
             held = 0.0;\n\
             for (i = 0; i < 2; i = i + 1) held = held + {time};\n\
             if ({time} >= 0.0) held = held + elapsed(1.0);\n\
             I(p, n) <+ V(p, n) * held;\n\
             end\n\
             endmodule\n"
        )
    }

    fn artifact_of(source: &str) -> CanonicalIrArtifact {
        VerilogACompiler::default()
            .compile_canonical_ir(source)
            .expect("the probe module compiles to canonical IR")
    }

    /// The Jacobian entry the bytecode route stamps, which is `held`: the
    /// contribution is linear in `V(p, n)`, and `held` reads only the clock.
    fn bytecode_held(fixture: &support::DeviceFixture, time: f64) -> f64 {
        let mut device = fixture.device("routes", &[1, 2]);
        device.try_set_time(time).unwrap();
        let mut conductance = 0.0;
        device
            .try_stamp(
                &[1.0, 0.0],
                |row, column, value| {
                    if (row, column) == (0, 0) {
                        conductance += value;
                    }
                },
                |_row, _value| {},
            )
            .unwrap();
        conductance
    }

    /// The canonical route's own answer: the residual of the one contribution
    /// at `V(p, n) = 1`, which is `held` up to the sign the node ordering
    /// gives it.
    fn canonical_held(artifact: &CanonicalIrArtifact, time: f64) -> f64 {
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
            .unwrap_or_else(|diagnostics| panic!("the probe module lowers: {diagnostics:?}"));
        let inputs = CfgEvalInputs {
            parameters: artifact
                .mir
                .parameters
                .iter()
                .map(|parameter| parameter.default.unwrap_or(0.0))
                .collect(),
            parameter_given: vec![false; artifact.mir.parameters.len()],
            port_connected: vec![true; artifact.hir.ports.len()],
            node_potentials: (0..artifact.mir.nodes.len())
                .map(|node| f64::from(u8::from(node == 0)))
                .collect(),
            branch_flows: vec![0.0; artifact.mir.branches.len()],
            branch_unknown_flows: vec![0.0; artifact.mir.branch_unknowns.len()],
            multiplicity: 1.0,
            time,
            ..Default::default()
        };
        let snapshot =
            evaluate_cfg(&cfg.function, &inputs).expect("the probe module evaluates at this time");
        snapshot
            .value(cfg.residuals[0])
            .expect("the contribution has a residual")
    }

    /// Every emitted line that reads the clock. This backend has no
    /// interpreter to run here, so its answer is the code it wrote.
    fn emitted_clock_reads(artifact: &CanonicalIrArtifact) -> Vec<String> {
        RustTranspiler::default()
            .transpile(artifact)
            .expect("the probe module is within the direct generated-Rust backend")
            .files
            .iter()
            .flat_map(|file| file.contents.lines())
            .filter(|line| line.contains("self.time"))
            .map(|line| line.trim().to_string())
            .collect()
    }

    fn agree(what: &str, measured: f64, reference: f64) {
        assert!(
            (measured - reference).abs() <= 1.0e-9 * reference.abs().max(1.0),
            "{what}: {measured}, expected {reference}"
        );
    }

    let scaled = probe("$realtime");
    let spelled = probe("($abstime / 1e-6)");
    let scaled_fixture = support::DeviceFixture::compile(&scaled);
    let spelled_fixture = support::DeviceFixture::compile(&spelled);
    let scaled_artifact = artifact_of(&scaled);
    let spelled_artifact = artifact_of(&spelled);

    for time in [0.0, 2.0e-6, 7.25e-6] {
        // Two reads in the unrolled loop, one through the inlined function,
        // and that function's own `+ 1.0`.
        let held = 3.0 * (time / UNIT) + 1.0;

        let bytecode = bytecode_held(&scaled_fixture, time);
        agree("bytecode $realtime", bytecode, held);
        agree(
            "bytecode $abstime/1e-6",
            bytecode_held(&spelled_fixture, time),
            bytecode,
        );

        let canonical = canonical_held(&scaled_artifact, time);
        agree("canonical $realtime", canonical.abs(), held);
        agree(
            "canonical $abstime/1e-6",
            canonical_held(&spelled_artifact, time),
            canonical,
        );
    }

    let emitted = emitted_clock_reads(&scaled_artifact);
    assert!(
        !emitted.is_empty(),
        "the generated device must read the clock at all"
    );
    assert_eq!(
        emitted,
        emitted_clock_reads(&spelled_artifact),
        "the generated Rust for `$realtime` must be the generated Rust for the division it means"
    );

    // The same question for an operator's argument, which reaches the lowering
    // through the operator rather than through the expression tree it sits in.
    // Compiling is the whole assertion: a route that kept its own `$realtime`
    // spelling would answer in plain seconds, and a route that has none would
    // refuse.
    let inside_operator = "`timescale 1us/1ns\n\
         module realtime_in_operator(p, n);\n\
         inout p, n; electrical p, n;\n\
         analog I(p, n) <+ V(p, n) * 1.0e-3 + ddt($realtime);\n\
         endmodule\n";
    support::DeviceFixture::compile(inside_operator);
    let artifact = artifact_of(inside_operator);
    CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("`ddt($realtime)` lowers: {diagnostics:?}"));
    RustTranspiler::default()
        .transpile(&artifact)
        .expect("`ddt($realtime)` is within the direct generated-Rust backend");
    support::DeviceFixture::compile(
        "`timescale 1us/1ns\n\
         module realtime_in_filter(p, n);\n\
         inout p, n; electrical p, n;\n\
         analog I(p, n) <+ laplace_nd($realtime * V(p, n), '{1.0, 0.5}, '{1.0, 0.25});\n\
         endmodule\n",
    );
}

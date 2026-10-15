//! `force`, `release`, procedural `assign` and `deassign` (IEEE 1364-2005
//! section 9.3) at the interpreter: each statement reaches the environment as
//! its own site, and a site's right-hand side evaluates against the signals
//! it reads, at the targets' total width. The kernel's override semantics are
//! pinned in `rspice-core`'s HDL host tests.

use super::*;
use rspice_veriloga::ast::ProceduralContinuousKind;
use rspice_veriloga::canonical_ir::digital::DigitalOverrideId;
use rspice_veriloga::canonical_ir::digital_eval::{DigitalEvalScratch, evaluate_override};

/// A store that answers each override statement by evaluating its site now.
struct Overrides<'a> {
    plan: &'a CanonicalDigitalPlan,
    values: Vec<FourStateValue>,
    reals: Vec<f64>,
    executed: Vec<(DigitalOverrideId, Option<DigitalScalar>)>,
}

impl DigitalEnvironment for Overrides<'_> {
    fn read_clock(&self) -> Option<rspice_veriloga::canonical_ir::DigitalClock> {
        None
    }
    fn read_signal(&self, signal: DigitalSignalId) -> Option<FourStateValue> {
        self.values.get(usize::from(signal)).cloned()
    }
    fn write_signal(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        self.values[usize::from(signal)] = value;
    }
    fn defer_update(&mut self, _: DigitalDeferredUpdate) {
        panic!("no deferred update in these fixtures");
    }
    fn write_real_signal(&mut self, signal: DigitalSignalId, value: f64) {
        self.reals[usize::from(signal)] = value;
    }
    fn read_real_signal(&self, signal: DigitalSignalId) -> Option<f64> {
        self.reals.get(usize::from(signal)).copied()
    }
    fn read_analog_potential(&self, _: DigitalAnalogProbeId) -> Option<f64> {
        None
    }
    fn drive_real_signal(&mut self, _: DigitalRealDrive) {}
    fn drive_signal(&mut self, _: DigitalDrive) {}
    fn procedural_override(&mut self, site: DigitalOverrideId) -> Result<(), DigitalEvalError> {
        let plan = self.plan;
        let value = if plan.overrides[site.index()].value.is_some() {
            Some(evaluate_override(
                plan,
                site,
                self,
                &mut DigitalEvalScratch::new(),
            )?)
        } else {
            None
        };
        self.executed.push((site, value));
        Ok(())
    }
}

fn run(
    source: &str,
) -> (
    CanonicalDigitalPlan,
    Vec<(DigitalOverrideId, Option<DigitalScalar>)>,
) {
    let plan = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir_module(source, None)
        .expect("fixture must lower")
        .digital;
    let mut environment = Overrides {
        plan: &plan,
        values: plan
            .signals
            .iter()
            .map(|signal| FourStateValue::splat(signal.width, FourStateBit::Unknown))
            .collect(),
        reals: vec![0.0; plan.signals.len()],
        executed: Vec::new(),
    };
    let process = plan
        .processes
        .iter()
        .find(|process| process.kind == DigitalProcessKind::Initial)
        .expect("one initial process");
    expect_finished(start(&plan, process, &mut environment).expect("process runs"));
    let executed = environment.executed;
    (plan, executed)
}

/// Each statement is one site, in source order, naming its targets most
/// significant first; the right-hand side fills the concatenation's total
/// width, zero-extended per section 5.2.1 as an assignment would be.
#[test]
fn override_statements_name_their_sites_and_evaluate_at_the_target_width() {
    let (plan, executed) = run("module t;\n  wire [3:0] w;\n  reg q;\n  reg [7:0] a;\n  \
         initial begin\n    a = 8'h0d;\n    force {w[2:1], q} = a;\n    release w[2];\n    \
         assign q = a[0];\n    deassign q;\n  end\nendmodule\n");
    let kinds: Vec<_> = plan.overrides.iter().map(|site| site.kind).collect();
    assert_eq!(
        kinds,
        [
            ProceduralContinuousKind::Force,
            ProceduralContinuousKind::Release,
            ProceduralContinuousKind::Assign,
            ProceduralContinuousKind::Deassign
        ]
    );
    let force = &plan.overrides[0];
    assert_eq!(
        force
            .targets
            .iter()
            .map(|target| (
                plan.signals[usize::from(target.target.signal)]
                    .name
                    .as_str(),
                target.width
            ))
            .collect::<Vec<_>>(),
        [("w", 2), ("q", 1)]
    );
    assert_eq!(force.width(), 3);
    let a = plan
        .signals
        .iter()
        .find(|signal| signal.name == "a")
        .unwrap()
        .id;
    assert_eq!(force.value.as_ref().unwrap().dependencies, [a]);
    let sites: Vec<u32> = executed.iter().map(|(site, _)| site.0).collect();
    assert_eq!(sites, [0, 1, 2, 3]);
    assert_eq!(
        executed[0].1,
        Some(DigitalScalar::FourState(parse_value("101")))
    );
    assert_eq!(executed[1].1, None);
    assert_eq!(
        executed[2].1,
        Some(DigitalScalar::FourState(parse_value("1")))
    );
}

/// A real variable is forced with a real value, and an integral right-hand
/// side converts to it (section 4.8.1).
#[test]
fn a_real_target_takes_a_real_right_hand_side() {
    let (_, executed) = run("module t;\n  real r;\n  integer i;\n  \
         initial begin\n    i = 3;\n    force r = i * 2;\n  end\nendmodule\n");
    assert_eq!(
        executed,
        [(DigitalOverrideId(0), Some(DigitalScalar::Real(6.0)))]
    );
}

/// A user function's body reads module signals its arguments do not name;
/// the right-hand side depends on them through every nested call, and not on
/// the functions' own static argument and result storage (section 9.3).
#[test]
fn a_function_call_subscribes_the_right_hand_side_to_what_the_function_reads() {
    let (plan, _) = run("module t;\n  reg [3:0] g, h, a, v;\n  \
         function [3:0] inner;\n    input [3:0] x;\n    inner = x + h;\n  endfunction\n  \
         function [3:0] outer;\n    input [3:0] x;\n    outer = inner(x) + g;\n  endfunction\n  \
         initial begin\n    g = 1; h = 2; a = 3;\n    force v = outer(a);\n  end\nendmodule\n");
    let id = |name: &str| {
        plan.signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap_or_else(|| panic!("signal {name}"))
            .id
    };
    let mut expected = vec![id("g"), id("h"), id("a")];
    expected.sort_unstable();
    assert_eq!(
        plan.overrides[0].value.as_ref().unwrap().dependencies,
        expected
    );
}

//! Route parity, one row per Verilog-A construct.
//!
//! RSpice lowers a module twice. The bytecode lowering is what the interpreter
//! executes (and, under `native-bytecode-contract-tests`, what the bytecode
//! backend compiles); the canonical lowering is the CFG the production native
//! and browser backends compile. Every other test in this crate pins one of
//! them against hand-derived physics. None of them states, per construct,
//! whether the two *agree* — so a construct whose routes diverge is found by
//! whichever model happens to use it.
//!
//! This file is that statement: one `#[test]` per construct, one small module
//! exercising exactly that construct, evaluated on both routes at the same
//! operating points and time steps, compared entry for entry. Bit-identical is
//! the default; a row that needs slack says why in its own doc comment and
//! passes a ULP budget.
//!
//! Gating follows `native_contract.rs`: the comparison is only real where the
//! two lowerings execute differently, which is a `native` build on x86-64.
//! Without `native` both constructors run the same bytecode and the comparison
//! would be vacuous; `try_new` is reachable at all only under the contract-test
//! feature, because production native callers must supply canonical IR.
#![cfg(all(feature = "native-bytecode-contract-tests", target_arch = "x86_64"))]

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::vm::VmError;
use std::collections::BTreeMap;

mod support;

use support::DeviceFixture;

/// One observation, named so a mismatch says *what* disagreed.
///
/// `text` carries the outcomes that are not numbers — construction, and any
/// runtime diagnostic — so a route that refuses a construct is compared
/// against the other route's refusal rather than crashing the row.
#[derive(Debug, Clone, PartialEq)]
struct Entry {
    key: String,
    value: f64,
    text: Option<String>,
}

/// The device under observation plus the trace it is writing.
struct Probe<'a> {
    device: &'a mut VerilogADevice,
    fixture: &'a DeviceFixture,
    entries: Vec<Entry>,
}

impl Probe<'_> {
    fn num(&mut self, key: impl Into<String>, value: f64) {
        self.entries.push(Entry {
            key: key.into(),
            value,
            text: None,
        });
    }

    fn text(&mut self, key: impl Into<String>, text: impl Into<String>) {
        self.entries.push(Entry {
            key: key.into(),
            value: 0.0,
            text: Some(text.into()),
        });
    }

    /// A DC operating point.
    fn dc(&mut self, label: &str, voltages: &[f64]) {
        self.point(label, 0, 0.0, 0.0, voltages);
    }

    /// A transient point at `time` reached with step `dt`.
    fn tran(&mut self, label: &str, time: f64, dt: f64, voltages: &[f64]) {
        self.point(label, 2, time, dt, voltages);
    }

    /// Evaluate, stamp, and read the per-evaluation stepper controls.
    ///
    /// Evaluation and stamping both run at the same point, as Newton does; the
    /// event operators are required to be idempotent across those repeats, and
    /// a route that is not is exactly what this file is looking for.
    fn point(&mut self, label: &str, analysis: u8, time: f64, dt: f64, voltages: &[f64]) {
        if let Err(err) = self.device.try_set_analysis_type(analysis) {
            self.text(format!("{label}.analysis"), err.to_string());
            return;
        }
        if let Err(err) = self.device.try_set_time(time) {
            self.text(format!("{label}.time"), err.to_string());
            return;
        }
        if let Err(err) = self.device.try_set_timestep(dt) {
            self.text(format!("{label}.timestep"), err.to_string());
            return;
        }
        if let Err(err) = self.device.try_update_voltages(voltages) {
            self.text(format!("{label}.voltages"), err.to_string());
            return;
        }
        let evaluated = self.device.try_evaluate();
        match evaluated {
            Ok(currents) => {
                for (index, current) in currents.into_iter().enumerate() {
                    self.num(format!("{label}.i[{index}]"), current);
                }
            }
            Err(err) => self.text(format!("{label}.i"), err.to_string()),
        }
        let mut matrix: BTreeMap<(usize, usize), f64> = BTreeMap::new();
        let mut rhs: BTreeMap<usize, f64> = BTreeMap::new();
        let stamped = self.device.try_stamp(
            voltages,
            |row, column, value| *matrix.entry((row, column)).or_insert(0.0) += value,
            |row, value| *rhs.entry(row).or_insert(0.0) += value,
        );
        match stamped {
            Ok(()) => {
                for ((row, column), value) in matrix {
                    self.num(format!("{label}.g[{row},{column}]"), value);
                }
                for (row, value) in rhs {
                    self.num(format!("{label}.rhs[{row}]"), value);
                }
            }
            Err(err) => self.text(format!("{label}.stamp"), err.to_string()),
        }
        let bound_step = self.device.transient_bound_step().unwrap_or(f64::NAN);
        self.num(format!("{label}.bound_step"), bound_step);
        let pending = f64::from(u8::from(self.device.discontinuity_pending()));
        self.num(format!("{label}.discontinuity"), pending);
        let rising = f64::from(u8::from(self.device.discontinuity_rising()));
        self.num(format!("{label}.rising"), rising);
    }

    /// Reactive (charge) stamping at the current point.
    fn reactive(&mut self, label: &str, voltages: &[f64]) {
        let mut matrix: BTreeMap<(usize, usize), f64> = BTreeMap::new();
        let stamped = self
            .device
            .try_stamp_reactive(voltages, |row, column, value| {
                *matrix.entry((row, column)).or_insert(0.0) += value
            });
        match stamped {
            Ok(()) => {
                for ((row, column), value) in matrix {
                    self.num(format!("{label}.c[{row},{column}]"), value);
                }
            }
            Err(err) => self.text(format!("{label}.reactive"), err.to_string()),
        }
    }

    /// The noise-source table the device emits at this operating point.
    fn noise(&mut self, label: &str, voltages: &[f64]) {
        if let Err(err) = self.device.try_set_analysis_type(3) {
            self.text(format!("{label}.noise_analysis"), err.to_string());
            return;
        }
        let evaluated = self.device.try_noise_sources(voltages);
        match evaluated {
            Ok(sources) => {
                self.num(format!("{label}.noise.count"), sources.len() as f64);
                for (index, source) in sources.iter().enumerate() {
                    self.text(format!("{label}.noise[{index}].name"), source.name.clone());
                    self.num(
                        format!("{label}.noise[{index}].node_pos"),
                        source.node_pos as f64,
                    );
                    self.num(
                        format!("{label}.noise[{index}].node_neg"),
                        source.node_neg as f64,
                    );
                    self.num(format!("{label}.noise[{index}].psd"), source.psd);
                    match source.exponent {
                        Some(exponent) => {
                            self.num(format!("{label}.noise[{index}].exponent"), exponent)
                        }
                        None => self.text(format!("{label}.noise[{index}].exponent"), "white"),
                    }
                    match &source.table {
                        Some((points, logarithmic)) => {
                            self.text(
                                format!("{label}.noise[{index}].table_log"),
                                logarithmic.to_string(),
                            );
                            for (point, (frequency, density)) in points.iter().enumerate() {
                                self.num(
                                    format!("{label}.noise[{index}].table[{point}].f"),
                                    *frequency,
                                );
                                self.num(
                                    format!("{label}.noise[{index}].table[{point}].p"),
                                    *density,
                                );
                            }
                        }
                        None => self.text(format!("{label}.noise[{index}].table"), "none"),
                    }
                }
            }
            Err(err) => self.text(format!("{label}.noise"), err.to_string()),
        }
    }

    /// Publish the named variables and read them back.
    fn observe(&mut self, label: &str, names: &[&str]) {
        let published = self.device.observe_variables(&self.fixture.canonical_ir);
        if let Err(err) = published {
            self.text(format!("{label}.observe"), err.to_string());
            return;
        }
        for name in names {
            let value = self.device.variable(name);
            match value {
                Some(value) => self.num(format!("{label}.var.{name}"), value),
                None => self.text(format!("{label}.var.{name}"), "absent"),
            }
        }
    }

    /// Accept the point: the state operators commit their histories here.
    fn accept(&mut self, label: &str) {
        let advanced = self.device.try_advance_state();
        match advanced {
            Ok(()) => self.text(format!("{label}.accept"), "ok"),
            Err(err) => self.text(format!("{label}.accept"), err.to_string()),
        }
    }

    /// Install an ambient temperature, recording whether the route took it.
    fn temperature(&mut self, label: &str, kelvin: f64) {
        let applied = self.device.try_set_temperature(kelvin);
        match applied {
            Ok(()) => self.text(format!("{label}.temperature"), "ok"),
            Err(err) => self.text(format!("{label}.temperature"), err.to_string()),
        }
    }
}

fn trace(
    fixture: &DeviceFixture,
    constructed: Result<VerilogADevice, VmError>,
    script: &dyn Fn(&mut Probe),
) -> Vec<Entry> {
    match constructed {
        Err(err) => vec![Entry {
            key: "construct".to_string(),
            value: 0.0,
            text: Some(err.to_string()),
        }],
        Ok(mut device) => {
            let mut probe = Probe {
                device: &mut device,
                fixture,
                entries: vec![Entry {
                    key: "construct".to_string(),
                    value: 0.0,
                    text: Some("ok".to_string()),
                }],
            };
            script(&mut probe);
            probe.entries
        }
    }
}

/// Two floats agree within `ulps` representable steps.
///
/// Zero is the default and means bit-identical, including the sign of zero and
/// the payload of a NaN: a route that returns a different NaN has taken a
/// different branch, which is a finding, not a rounding difference.
fn within_ulps(bytecode: f64, canonical: f64, ulps: u64) -> bool {
    if bytecode.to_bits() == canonical.to_bits() {
        return true;
    }
    if ulps == 0 || bytecode.is_nan() || canonical.is_nan() {
        return false;
    }
    if bytecode.is_sign_negative() != canonical.is_sign_negative() {
        // Across zero the bit distance is meaningless; count both sides.
        let distance = bytecode.to_bits() & !(1u64 << 63);
        let other = canonical.to_bits() & !(1u64 << 63);
        return distance.saturating_add(other) <= ulps;
    }
    bytecode.to_bits().abs_diff(canonical.to_bits()) <= ulps
}

fn assert_traces_agree(bytecode: &[Entry], canonical: &[Entry], ulps: u64) {
    let mut differences = Vec::new();
    for index in 0..bytecode.len().max(canonical.len()) {
        match (bytecode.get(index), canonical.get(index)) {
            (Some(left), Some(right)) if left.key != right.key => differences.push(format!(
                "[{index}] observation order: bytecode '{}', canonical '{}'",
                left.key, right.key
            )),
            (Some(left), Some(right)) if left.text != right.text => differences.push(format!(
                "{}: bytecode {:?}, canonical {:?}",
                left.key, left.text, right.text
            )),
            (Some(left), Some(right))
                if left.text.is_none() && !within_ulps(left.value, right.value, ulps) =>
            {
                differences.push(format!(
                    "{}: bytecode {:.17e} ({:016x}), canonical {:.17e} ({:016x})",
                    left.key,
                    left.value,
                    left.value.to_bits(),
                    right.value,
                    right.value.to_bits()
                ));
            }
            (Some(_), Some(_)) => {}
            (Some(left), None) => differences.push(format!(
                "[{index}] '{}' only on the bytecode route",
                left.key
            )),
            (None, Some(right)) => differences.push(format!(
                "[{index}] '{}' only on the canonical route",
                right.key
            )),
            (None, None) => unreachable!("index is below one of the two lengths"),
        }
    }
    assert!(
        differences.is_empty(),
        "{} route difference(s) between the bytecode and canonical lowerings \
         ({} bytecode observations, {} canonical):\n{}",
        differences.len(),
        bytecode.len(),
        canonical.len(),
        differences
            .iter()
            .take(16)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Run one script on both lowerings of one compilation and compare it.
fn compare_routes(source: &str, nodes: &[usize], ulps: u64, script: impl Fn(&mut Probe)) {
    let fixture = DeviceFixture::compile(source);
    let bytecode = trace(
        &fixture,
        fixture.try_bytecode_device("ROUTE_BYTECODE", nodes),
        &script,
    );
    let canonical = trace(
        &fixture,
        fixture.try_device("ROUTE_CANONICAL", nodes),
        &script,
    );
    assert_traces_agree(&bytecode, &canonical, ulps);
}

/// The scripts most rows use: a DC point, then three accepted transient steps.
fn transient_walk(probe: &mut Probe) {
    probe.dc("dc", &[0.0]);
    probe.accept("dc");
    for (label, time, dt, bias) in [
        ("t1", 1.0e-9, 1.0e-9, 1.0),
        ("t2", 2.0e-9, 1.0e-9, 0.5),
        ("t3", 4.0e-9, 2.0e-9, -0.25),
    ] {
        // The order a transient step actually has: a nonlinear value pass, a
        // small-signal probe of the same point, then the pass that decides what
        // is accepted. A reactive stamp is an observation and must not be the
        // last thing a step does, on either route.
        probe.tran(label, time, dt, &[bias]);
        probe.reactive(label, &[bias]);
        probe.tran(&format!("{label}n"), time, dt, &[bias]);
        probe.accept(label);
    }
    probe.tran("t4", 5.0e-9, 1.0e-9, &[-0.25]);
    probe.reactive("t4", &[-0.25]);
}

/// A bias sweep with no time: the rows whose construct is not stateful.
fn bias_sweep(probe: &mut Probe) {
    for (index, bias) in [-1.0, -0.25, 0.0, 0.25, 0.75, 2.0].into_iter().enumerate() {
        probe.dc(&format!("dc{index}"), &[bias]);
    }
}

// ---------------------------------------------------------------------------
// Analog operators with state
// ---------------------------------------------------------------------------

#[test]
fn ddt_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_ddt(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-9;
    analog I(p, n) <+ c * ddt(V(p, n)) + V(p, n) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

/// A small-signal probe taken *after* the value pass and before acceptance
/// leaves both routes with the same integrator history.
///
/// Found while writing `transient_walk`, which originally ended each step with
/// `try_stamp_reactive`. With that ordering the bytecode route accepted the
/// pre-step integrator history and the canonical route the stepped one, so
/// every point after the first differed by a whole step: on the `ddt` fixture
/// at t=2 ns the bytecode route returned 5.00499999999999945e-1 where the
/// canonical route returned -4.99499999999999944e-1 (exactly G*V against
/// G*V - 1), its companion RHS stayed at zero against the canonical route's
/// 9.99999999999999889e-1, and its accepted charge never left the operating
/// point for the rest of the walk. The canonical route is the one the
/// integrator contract names: an observation reads the accepted history and
/// the trial charge, and `advance_state` rotates the candidate the value pass
/// published — so an observation may not reopen that candidate, because the
/// bytecode lowering's reactive Jacobian is `dQ/dx` alone and republishes
/// nothing. `transient_walk` covers the ordering the engine uses; this row
/// covers the one it does not.
#[test]
fn reactive_probe_before_acceptance_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_reactive_before_accept(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-9;
    analog I(p, n) <+ c * ddt(V(p, n)) + V(p, n) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.dc("dc", &[0.0]);
            probe.accept("dc");
            for (label, time, dt, bias) in [
                ("t1", 1.0e-9, 1.0e-9, 1.0),
                ("t2", 2.0e-9, 1.0e-9, 0.5),
                ("t3", 4.0e-9, 2.0e-9, -0.25),
            ] {
                probe.tran(label, time, dt, &[bias]);
                probe.reactive(label, &[bias]);
                probe.accept(label);
            }
            probe.tran("t4", 5.0e-9, 1.0e-9, &[-0.25]);
        },
    );
}

#[test]
fn idt_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_idt(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 0.5) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn idtmod_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_idtmod(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idtmod(V(p, n), 0.25, 2.0, 0.0) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn ddx_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_ddx(p);
    inout p;
    electrical p;
    analog I(p) <+ ddx(3.0 * V(p) * V(p), V(p)) * 1.0e-3;
endmodule
"#,
        &[1],
        0,
        bias_sweep,
    );
}

#[test]
fn transition_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_transition(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n) > 0.5 ? 1.0e-3 : 0.0, 1.0e-9, 2.0e-9, 3.0e-9);
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn slew_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_slew(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ slew(V(p, n), 1.0e6, -2.0e6) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn absdelay_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_absdelay(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ absdelay(V(p, n), 1.0e-9, 1.0e-6) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn laplace_nd_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_laplace(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ laplace_nd(V(p, n), '{1.0, 0.5}, '{1.0, 0.25}) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn zi_nd_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_zi(p, n);
    inout p, n;
    electrical p, n;
    real y;
    analog begin
        y = zi_nd(V(p, n), '{0.25}, '{1.0, -0.75}, 1.0e-9, 0.0);
        I(p, n) <+ y * 1.0e-3;
    end
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            transient_walk(probe);
            probe.observe("t4", &["y"]);
        },
    );
}

// ---------------------------------------------------------------------------
// Event operators
// ---------------------------------------------------------------------------

/// The event rows walk seconds, not nanoseconds: `timer` and `cross` schedule
/// against absolute time and a nanosecond walk never reaches their events.
fn event_walk(probe: &mut Probe) {
    probe.dc("dc", &[-1.0]);
    probe.accept("dc");
    probe.tran("t0", 0.0, 0.0, &[-1.0]);
    probe.accept("t0");
    probe.tran("t1", 1.0, 1.0, &[3.0]);
    probe.tran("t1r", 1.0, 1.0, &[3.0]);
    probe.accept("t1");
    probe.tran("t2", 1.5, 0.5, &[0.25]);
    probe.accept("t2");
    probe.tran("t3", 2.0, 0.5, &[-1.0]);
    probe.accept("t3");
}

#[test]
fn cross_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_cross(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 0, 0.0, 0.1, 1.0);
endmodule
"#,
        &[1, 0],
        0,
        event_walk,
    );
}

#[test]
fn above_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_above(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ above(V(p, n) - 0.5, 0.0, 0.0, 1.0);
endmodule
"#,
        &[1, 0],
        0,
        event_walk,
    );
}

#[test]
fn timer_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_timer(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ timer(1.0, 0.5, 0.0, 1.0);
endmodule
"#,
        &[1, 0],
        0,
        event_walk,
    );
}

// ---------------------------------------------------------------------------
// Simulator control tasks
// ---------------------------------------------------------------------------

#[test]
fn bound_step_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_bound_step(p, n);
    inout p, n;
    electrical p, n;
    parameter real cap = 1.0e-9;
    analog begin
        $bound_step(1.0e-6);
        if (V(p, n) > 0.5)
            $bound_step(cap);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.tran("below", 0.0, 1.0e-9, &[0.2]);
            probe.accept("below");
            probe.tran("above", 1.0e-9, 1.0e-9, &[0.8]);
            probe.accept("above");
            probe.tran("back", 2.0e-9, 1.0e-9, &[0.2]);
        },
    );
}

#[test]
fn discontinuity_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_discontinuity(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (V(p, n) > 1.0)
            $discontinuity(0);
        I(p, n) <+ V(p, n) * 1.0e-3;
    end
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.tran("quiet", 0.0, 1.0e-9, &[0.5]);
            probe.accept("quiet");
            probe.tran("edge", 1.0e-9, 1.0e-9, &[1.5]);
            probe.accept("edge");
            probe.tran("level", 2.0e-9, 1.0e-9, &[1.6]);
            probe.accept("level");
            probe.tran("clear", 3.0e-9, 1.0e-9, &[0.4]);
        },
    );
}

/// `$limit` with a named user limiter does not exist on both routes at all.
///
/// Measured on this fixture: the canonical route compiles and evaluates it (29
/// observations), while the bytecode route refuses construction outright —
/// "model route_limit: native JIT does not support canonical-only named
/// limiter metadata in a bytecode entry; no interpreter fallback" — leaving one
/// observation. That is a capability gap, not a numeric one, so the row states
/// the comparison and stays ignored until the gap is closed or declared.
#[test]
#[ignore = "R4.x-triage: the bytecode lowering refuses a canonical-only named \
            limiter, so $limit has no bytecode route to compare (1 observation \
            against the canonical route's 29)"]
fn limit_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_limit(p, n);
    inout p, n;
    electrical p, n;
    parameter real is_sat = 1.0e-14;
    parameter real vth = 0.025;
    parameter real vmax = 0.4;

    analog function real clampv;
        input proposed, previous, ceiling;
        real proposed, previous, ceiling;
        begin
            clampv = proposed > ceiling ? ceiling : proposed;
        end
    endfunction

    real vd;
    analog begin
        vd = $limit(V(p, n), clampv, vmax);
        I(p, n) <+ is_sat * (exp(vd / vth) - 1.0);
    end
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            for (index, bias) in [0.1, 0.2, 0.4, 0.7].into_iter().enumerate() {
                probe.dc(&format!("dc{index}"), &[bias]);
                probe.observe(&format!("dc{index}"), &["vd"]);
            }
        },
    );
}

// ---------------------------------------------------------------------------
// Environment and analysis queries
// ---------------------------------------------------------------------------

#[test]
fn abstime_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_abstime(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (1.0 + $abstime) * V(p, n) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        transient_walk,
    );
}

#[test]
fn temperature_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_temperature(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ $temperature * V(p, n) * 1.0e-6;
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            for (index, kelvin) in [200.0, 300.15, 450.0].into_iter().enumerate() {
                probe.temperature(&format!("t{index}"), kelvin);
                probe.dc(&format!("dc{index}"), &[0.75]);
            }
        },
    );
}

#[test]
fn thermal_voltage_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_vt(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ $vt() * V(p, n) * 1.0e-3 + $vt(V(p, n) + 400.0) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.temperature("ambient", 330.0);
            bias_sweep(probe);
        },
    );
}

#[test]
fn analysis_query_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_analysis(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (analysis("tran") + 2.0 * analysis("ac") + 4.0 * analysis("dc")
                       + 8.0 * analysis("noise")) * V(p, n) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            for analysis in 0u8..=4 {
                probe.point(&format!("a{analysis}"), analysis, 0.0, 0.0, &[1.25]);
            }
        },
    );
}

#[test]
fn simparam_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_simparam(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ($simparam("gmin", 7.0) + $simparam("imax", 5.0)) * V(p, n);
endmodule
"#,
        &[1, 0],
        0,
        bias_sweep,
    );
}

#[test]
fn port_connected_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_port_connected(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ($port_connected(p) + 2.0 * $port_connected(n)) * V(p, n) * 1.0e-3;
endmodule
"#,
        &[1, 0],
        0,
        bias_sweep,
    );
}

// ---------------------------------------------------------------------------
// Noise
// ---------------------------------------------------------------------------

/// No noise *analysis* runs at this level, so the parity statement is the
/// emitted source table — identity, injection pair, PSD and shape — which is
/// the whole of what a noise analysis consumes from a device.
fn noise_walk(probe: &mut Probe) {
    probe.dc("dc", &[0.3]);
    probe.noise("dc", &[0.3]);
    probe.dc("bias", &[0.9]);
    probe.noise("bias", &[0.9]);
}

#[test]
fn white_noise_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_white_noise(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0e-3 + white_noise(1.0e-18, "wn");
endmodule
"#,
        &[1, 0],
        0,
        noise_walk,
    );
}

#[test]
fn flicker_noise_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_flicker_noise(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0e-3 + flicker_noise(1.0e-18, 2.0, "fl");
endmodule
"#,
        &[1, 0],
        0,
        noise_walk,
    );
}

#[test]
fn noise_table_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_noise_table(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) * 1.0e-3
                    + noise_table('{1.0, 2.0e-18, 10.0, 4.0e-18}, "tbl");
endmodule
"#,
        &[1, 0],
        0,
        noise_walk,
    );
}

// ---------------------------------------------------------------------------
// Structural constructs
// ---------------------------------------------------------------------------

#[test]
fn table_model_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_table_model(p, n);
    inout p, n;
    electrical p, n;
    real gain;
    analog begin
        gain = $table_model(V(p, n), 0.0, 0.0, 1.0, 2.0, 2.0, 8.0);
        I(p, n) <+ gain * 1.0e-3;
    end
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            bias_sweep(probe);
            probe.observe("dc5", &["gain"]);
        },
    );
}

/// A switch branch alternates the branch's own nature, so the row's unknown
/// exists on one side and not the other. The internal index mirrors
/// `timestep_control.rs`, which is the harness that pins this shape.
#[test]
fn switch_branch_agrees_across_lowerings() {
    compare_routes(
        "module route_switch(p, n); inout p, n; electrical p, n;
         analog if (V(p, n) > 0.5) V(p, n) <+ 0.0; else I(p, n) <+ V(p, n) * 1.0e-3; endmodule",
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.device.set_internal_node_indices(&[2]);
            for (index, bias) in [-1.0, 0.25, 1.0, 2.0, 0.25].into_iter().enumerate() {
                probe.dc(&format!("dc{index}"), &[bias, 0.0]);
                probe.accept(&format!("dc{index}"));
            }
        },
    );
}

/// The retained value of a switch branch is `CheckedValue(value, ...)`, and a
/// charge contributed through one arm puts that value's *derivative* in the
/// reactive stamp. The two lowerings differentiate it in different places — the
/// bytecode one while interleaving shadows (`ir.rs`, `BinaryOp::CheckedValue =>
/// dr`), the canonical one in the native backend's own expression lowering —
/// so this row is where those two rules meet. The canonical route refused the
/// operator outright until the backend carried it, which is the shape that kept
/// DIODE_CMC off the native route.
#[test]
fn reactive_switch_branch_agrees_across_lowerings() {
    compare_routes(
        "module route_switch_charge(p, n); inout p, n; electrical p, n;
         parameter real c = 0.5; parameter real nqs = 1.0; real qn;
         analog begin qn = c * V(p, n) * V(p, n);
           if (nqs > 0.0) I(p, n) <+ ddt(qn);
           else V(p, n) <+ 0.0; end endmodule",
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.device.set_internal_node_indices(&[2]);
            probe.dc("dc", &[0.0, 0.0]);
            probe.accept("dc");
            // The charge only reaches a stamp on a transient step, so the three
            // biases the capacitance is read at are transient points.
            for (index, bias) in [-0.75, 0.5, 1.5].into_iter().enumerate() {
                let label = format!("t{index}");
                let time = 1.0e-9 * (index as f64 + 1.0);
                probe.tran(&label, time, 1.0e-9, &[bias, 0.0]);
                probe.reactive(&label, &[bias, 0.0]);
                probe.accept(&label);
            }
        },
    );
}

#[test]
fn indirect_source_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_indirect(p, n);
    inout p, n;
    electrical p, n;
    analog V(p, n): I(p, n) == 0.0;
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            probe.device.set_branch_current_indices(&[2]);
            for (index, bias) in [[0.25, 0.0], [3.0, -0.00275], [-1.0, 0.5]]
                .into_iter()
                .enumerate()
            {
                probe.dc(&format!("dc{index}"), &bias);
            }
        },
    );
}

#[test]
fn user_analog_function_agrees_across_lowerings() {
    compare_routes(
        r#"
`include "disciplines.vams"
module route_function(p, n);
    inout p, n;
    electrical p, n;

    analog function real blended;
        input x, k;
        real x, k;
        real half;
        real square;
        begin
            half = x * 0.5;
            square = half * half;
            blended = square * k + half;
        end
    endfunction

    real out;
    analog begin
        out = blended(V(p, n), 3.0);
        I(p, n) <+ out * 1.0e-3;
    end
endmodule
"#,
        &[1, 0],
        0,
        |probe: &mut Probe| {
            bias_sweep(probe);
            probe.observe("dc5", &["out"]);
        },
    );
}

/// LRM 4.4: a module-scope variable is not reinitialized between evaluations,
/// so a read before assignment yields the value the previous *accepted*
/// evaluation left. The bytecode lowering executes every assignment step of
/// the body on every evaluation and therefore carries it; the CFG lowering
/// treats the variable as block-local and starts from zero. The row asserts
/// the LRM answer — `seen` must equal the previous step's `held` — and the
/// route comparison beneath it is what shows which lowering is wrong.
#[test]
#[ignore = "R4.1: read-before-assign persistence differs by route"]
fn read_before_assign_persistence_agrees_across_lowerings() {
    const SOURCE: &str = r#"
`include "disciplines.vams"
module route_read_before_assign(p, n);
    inout p, n;
    electrical p, n;
    real held;
    real seen;
    analog begin
        seen = held;
        held = V(p, n) * 2.0;
        I(p, n) <+ (seen + 1.0e-3) * V(p, n);
    end
endmodule
"#;

    // The LRM answer, stated independently of either route: the third accepted
    // evaluation must see what the second one stored.
    let fixture = DeviceFixture::compile(SOURCE);
    for (route, constructed) in [
        (
            "bytecode",
            fixture.try_bytecode_device("LRM_BYTECODE", &[1, 0]),
        ),
        ("canonical", fixture.try_device("LRM_CANONICAL", &[1, 0])),
    ] {
        let mut device = constructed.unwrap_or_else(|err| panic!("{route}: {err}"));
        for bias in [0.25, 0.5] {
            device.try_update_voltages(&[bias]).unwrap();
            device.try_evaluate().unwrap();
            device.try_advance_state().unwrap();
        }
        device.try_update_voltages(&[0.75]).unwrap();
        device.try_evaluate().unwrap();
        fixture.observe(&mut device);
        assert_eq!(
            device.variable("seen"),
            Some(1.0),
            "{route}: a read before assignment must carry 2 * 0.5 from the previous evaluation"
        );
    }

    compare_routes(SOURCE, &[1, 0], 0, |probe| {
        for (index, bias) in [0.25, 0.5, 0.75].into_iter().enumerate() {
            probe.dc(&format!("dc{index}"), &[bias]);
            probe.observe(&format!("dc{index}"), &["seen", "held"]);
            probe.accept(&format!("dc{index}"));
        }
    });
}

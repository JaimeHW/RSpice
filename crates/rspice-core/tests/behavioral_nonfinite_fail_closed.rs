//! Behavioral B sources must never turn non-finite equations into zero-valued
//! sources. Every public analysis boundary reports the authored source and the
//! analysis coordinates that produced the invalid value.

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).expect("behavioral non-finite regression deck parses")
}

fn assert_behavioral_error(error: impl std::fmt::Display, kind: &str, name: &str) {
    let message = error.to_string().to_ascii_lowercase();
    assert!(
        message.contains(&format!("behavioral {kind} source '{name}'")),
        "error must identify the behavioral source: {message}"
    );
    assert!(
        message.contains("non-finite expression value"),
        "error must identify the invalid expression value: {message}"
    );
    assert!(
        message.contains("time") && message.contains("frequency"),
        "error must carry analysis coordinates: {message}"
    );
}

fn source_cases(expression: &str) -> [(String, &'static str, &'static str); 2] {
    [
        (
            format!("BVERR out 0 V={{{expression}}}"),
            "voltage",
            "bverr",
        ),
        (
            format!("BIERR out 0 I={{{expression}}}"),
            "current",
            "bierr",
        ),
    ]
}

/// A `B` source whose expression is finite at its operating point and
/// overflows at an overshooting Newton iterate is a rejected iterate, not a
/// refused circuit.
///
/// `exp` is the shape used here rather than `ln(v(a)+0.1)`, the expression
/// R1.4 pinned on the Verilog-A route, because it overflows on the way up
/// rather than at a domain edge: `exp(5/0.002)` overflows while
/// `exp(0.0539/0.002)` is an ordinary number, and plain Newton from the zero
/// guess proposes the former on its way to the latter. The logarithm has its
/// own fixture below, on what the clamp used to do to it.
///
/// Before R1.14 the behavioral route's non-finite refusal ended the run at
/// that iterate while the runtime Verilog-A route retried the same shape. The
/// two answer the same way now in DC, which is this fixture, and in transient,
/// which is the one below it.
#[test]
fn a_behavioral_domain_edge_at_a_trial_iterate_is_rejected_and_source_stepped() {
    let deck = "behavioral overflow at an overshooting iterate\n\
                V1 in 0 DC 5\n\
                R1 in a 1k\n\
                B1 a 0 I={1.0e-14*exp(v(a)/0.002)}\n\
                .OP\n\
                .END\n";
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&parse(deck))
        .expect("a non-finite trial iterate must reject the iterate, not the run");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("node a is absent from the operating point");
    let voltage = result.node_voltages[index];
    let residual = (voltage - 5.0) * 1.0e-3 + 1.0e-14 * (voltage / 0.002).exp();
    assert!(
        residual.abs() < 1.0e-7,
        "V(a) = {voltage} leaves KCL residual {residual}"
    );
}

/// The transient twin of the fixture above, on the same domain edge.
///
/// The two Newton loops meet a behavioural overflow in different places. DC
/// only ever sees it at the stamp. Transient additionally asks
/// `behavioral_linearizations_converged` at the candidate the linear solve
/// just proposed — before anything stamps that candidate — and a 1 fs source
/// edge inside the first offered step makes the first such candidate
/// V(a) ~ 5 V, where `exp(5/0.002)` overflows. The check sat outside the
/// rejection macro, so the same deck that DC retried ended the run in
/// transient.
#[test]
fn a_behavioral_domain_edge_at_a_transient_trial_is_rejected_and_the_step_retried() {
    let deck = "behavioral overflow at an overshooting transient iterate\n\
                V1 in 0 PULSE(0 5 0 1e-15 1e-15 1 2)\n\
                R1 in a 1k\n\
                B1 a 0 I={1.0e-14*exp(v(a)/0.002)}\n\
                .TRAN 1n 3n\n\
                .END\n";
    // The configured first step spans the whole 1 fs source edge, so the first
    // Newton candidate carries the entire 5 V swing onto the exponential.
    let engine = Engine::new(SimulationConfig {
        transient_initial_timestep: Some(1.0e-9),
        ..Default::default()
    });
    let result = engine
        .run_tran(&parse(deck), 3.0e-9, 1.0e-9)
        .expect("a non-finite trial iterate must reject the iterate, not the run");

    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("node a is absent from the transient result");
    let output = &result.voltages[index];
    let source = |time: f64| -> f64 {
        if time >= 1.0e-15 {
            5.0
        } else {
            5.0 * time / 1.0e-15
        }
    };
    // The first point the stepper accepts is finer than the edge it just
    // failed on: that cut is what the rejected trial asked for.
    assert!(
        result.time.get(1).copied().unwrap_or(1.0) < 5.0e-16,
        "the rejected trial must have cut the step: {:?}",
        &result.time[..result.time.len().min(4)]
    );
    // The deck is resistive, so every accepted point solves
    // (V - Vin)/1k + 1e-14*exp(V/0.002) = 0. The branch is checked in volts
    // rather than in amps: the junction's slope at the solution is ~2.5 S, so
    // the solver's own 1 uV voltage tolerance is already a ~2 uA residual and
    // an amp-scale bound would be measuring `vntol`, not the branch.
    for (&time, &voltage) in result.time.iter().zip(output) {
        let residual = (voltage - source(time)) * 1.0e-3 + 1.0e-14 * (voltage / 0.002).exp();
        let slope = 1.0e-3 + 1.0e-14 * (voltage / 0.002).exp() / 0.002;
        let voltage_error = residual / slope;
        assert!(
            voltage_error.abs() < 1.0e-5,
            "t={time}: V(a)={voltage} is {voltage_error} V off the KCL branch (residual {residual})"
        );
    }
    assert!(
        result.time.last().copied().unwrap_or(0.0) >= 3.0e-9 - 1.0e-18,
        "the run must reach tstop: {:?}",
        result.time.last()
    );
}

/// A `B` source's logarithm is a circuit equation, so an iterate outside its
/// domain is rejected instead of clamped.
///
/// The deck has one operating point, V(a) ~ 0.894 V, where
/// `(V+5)/1k + ln(V+0.1) = 0`. It is not where the solver used to land. The
/// first linear solve puts V(a) at the source, -5 V, and `ln(-4.9)` is
/// undefined there — so `expr/vm.rs` clamped the argument to
/// `LOGARITHM_MIN_ARGUMENT = 1e-38` and returned `ln(1e-38) = -87.5`, while
/// the analytic slope beside it returned `1/1e-38 = 1e38`. A 1e38 conductance
/// on that node makes the next Newton update ~5e-6 V, the voltage tolerance
/// calls that converged, and the solve reported V(a) = -4.999995 V — a point
/// that misses KCL by 87 A and is outside the deck's own expression's domain,
/// with no diagnostic of any kind.
///
/// The clamp survives at zero, where the domain has a removable boundary and a
/// finite stand-in costs nothing. Below zero the value is NaN, which R1.14's
/// `StampError::NonFiniteTrial` turns into a rejected iterate: the solver
/// steps the source and follows the branch from V1 = 0, where V(a) = 0 and the
/// argument is 0.1, to the real answer.
#[test]
fn a_behavioral_logarithm_outside_its_domain_is_rejected_not_clamped() {
    let deck = "behavioral logarithm at an iterate outside its domain\n\
                V1 in 0 DC -5\n\
                R1 in a 1k\n\
                B1 a 0 I={ln(v(a)+0.1)}\n\
                .OP\n\
                .END\n";
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&parse(deck))
        .expect("the deck has an operating point inside the logarithm's domain");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("node a is absent from the operating point");
    let voltage = result.node_voltages[index];
    assert!(
        voltage + 0.1 > 0.0,
        "the reported operating point must be inside ln's domain, got V(a) = {voltage}"
    );
    let residual = (voltage + 5.0) * 1.0e-3 + (voltage + 0.1).ln();
    assert!(
        residual.abs() < 1.0e-7,
        "V(a) = {voltage} leaves KCL residual {residual}"
    );
}

#[test]
fn dc_op_rejects_nonfinite_voltage_and_current_source_values() {
    for (source, kind, name) in source_cases("1e308*1e308") {
        let deck =
            format!("behavioral {kind} DC non-finite value\n{source}\nRLOAD out 0 1k\n.OP\n.END\n");
        let error = Engine::new(SimulationConfig::default())
            .run_dc_op(&parse(&deck))
            .expect_err("a non-finite B-source DC equation must fail closed");
        assert_behavioral_error(error, kind, name);
    }
}

#[test]
fn ac_and_noise_reject_frequency_activated_nonfinite_voltage_and_current_sources() {
    for (source, kind, name) in source_cases("FREQ*1e308*1e308") {
        let deck = format!(
            "behavioral {kind} small-signal non-finite value\n{source}\nRLOAD out 0 1k\n.AC LIN 1 1k 1k\n.END\n"
        );
        let netlist = parse(&deck);
        let engine = Engine::new(SimulationConfig::default());

        let ac_error = engine
            .run_ac(&netlist, &[1.0e3])
            .expect_err("AC must reject a frequency-activated non-finite B source");
        assert_behavioral_error(ac_error, kind, name);

        let noise_error = engine
            .run_noise(&netlist, 1, &[1.0e3], 300.15)
            .expect_err("noise must reject a frequency-activated non-finite B source");
        assert_behavioral_error(noise_error, kind, name);
    }
}

#[test]
fn transient_rejects_time_activated_nonfinite_voltage_and_current_sources() {
    for (source, kind, name) in source_cases("TIME*1e308*1e308") {
        let deck = format!(
            "behavioral {kind} transient non-finite value\n{source}\nRLOAD out 0 1k\n.TRAN 1n 2n\n.END\n"
        );
        let error = Engine::new(SimulationConfig::default())
            .run_tran(&parse(&deck), 2.0e-9, 1.0e-9)
            .expect_err("transient must reject a time-activated non-finite B source");
        assert_behavioral_error(error, kind, name);
    }
}

#[test]
fn pss_rejects_time_activated_nonfinite_voltage_and_current_sources() {
    // A ramp is already invalid at the source-period contract. The sinusoid
    // passes that contract and must still fail during numerical evaluation;
    // periodicity does not certify finite expression values.
    for expression in ["TIME*1e308*1e308", "sin(2*pi*1meg*TIME)*1e308*1e308"] {
        for (source, kind, name) in source_cases(expression) {
            let deck = format!(
                "behavioral {kind} PSS non-finite value\n{source}\nRLOAD out 0 1k\nCLOAD out 0 1p\n.END\n"
            );
            let error = Engine::new(SimulationConfig::default())
                .run_pss(
                    &parse(&deck),
                    PssConfig::new(1.0e6)
                        .with_harmonics(2)
                        .with_points_per_period(8)
                        .with_tstab_periods(0),
                )
                .expect_err("PSS must reject a time-activated non-finite B source");
            if expression.starts_with("TIME") {
                let message = error.to_string().to_ascii_lowercase();
                assert!(
                    message.contains("analysis.pss.driven_source_waveform"),
                    "{message}"
                );
                assert!(message.contains(name), "{message}");
            } else {
                assert_behavioral_error(error, kind, name);
            }
        }
    }
}

/// The logarithm's guard at exactly zero is a convergence convenience, and it
/// is measured rather than assumed.
///
/// `ln(0)` is the boundary of the domain approached from inside it, not a
/// point outside it, and the zero initial guess puts an offset-free
/// `ln(v(a))` there on iteration 0 — as does every rung of the
/// source-stepping ladder, which restarts from the same guess. IEEE at zero
/// (−inf, hence a rejected iterate) would therefore refuse this deck outright
/// after the whole ladder, so the clamp stays and this fixture is what says
/// the cost is bounded: Newton climbs out of the guess to the deck's real
/// operating point, V(a) = 1.0040039906467049, where
/// `(V-5)/1k + ln(V) = -1.1e-15`.
///
/// ngspice makes the same exception for the same stated reason — `PTlog`
/// returns −1e99 at exactly zero because "arg 0 may happen, when starting
/// iteration for op or dc simulation" (ptfuncs.c) — while a *negative*
/// argument gets no stand-in there either. That asymmetry is the whole of
/// RSpice's policy for a circuit equation;
/// `a_behavioral_logarithm_outside_its_domain_is_rejected_not_clamped` pins
/// its other half.
#[test]
fn a_logarithm_at_exactly_zero_still_converges_from_the_zero_guess() {
    let deck = "logarithm whose argument is exactly zero at the initial guess\n\
                V1 in 0 DC 5\n\
                R1 in a 1k\n\
                B1 a 0 I={ln(v(a))}\n\
                .OP\n\
                .END\n";
    let result = Engine::new(SimulationConfig::default())
        .run_dc_op(&parse(deck))
        .expect("the stand-in at zero must leave this deck solvable");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("node a is absent from the operating point");
    let voltage = result.node_voltages[index];
    assert!(
        voltage > 0.0,
        "the reported operating point must be inside ln's domain, got V(a) = {voltage}"
    );
    assert!(
        (voltage - 1.0040039906467049).abs() < 1.0e-6,
        "Newton must climb out of the zero guess to the deck's own root, got V(a) = {voltage}"
    );
    let residual = (voltage - 5.0) * 1.0e-3 + voltage.ln();
    assert!(
        residual.abs() < 1.0e-9,
        "V(a) = {voltage} leaves KCL residual {residual}"
    );
}

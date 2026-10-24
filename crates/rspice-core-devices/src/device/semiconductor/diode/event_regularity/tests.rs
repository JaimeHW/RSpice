use super::*;
use std::collections::HashMap;

fn model(dialect: usize) -> Diode {
    let mut diode = Diode::spice_defaults("d".into(), 1, 0).with_model_params(&HashMap::from([
        ("IS".into(), 1e-16),
        ("N".into(), 1.2),
        ("BV".into(), 3.0),
        ("NBV".into(), 1.1),
        ("IBV".into(), 1e-6),
        ("IKF".into(), 1e-3),
        ("IKR".into(), 1e-4),
        ("ISR".into(), 1e-15),
        ("NR".into(), 2.0),
        ("JSW".into(), 2e-16),
        ("NS".into(), 1.4),
        ("IKP".into(), 1e-3),
        ("JTUN".into(), 1e-20),
        ("JTUNSW".into(), 2e-20),
        ("NTUN".into(), 2.0),
        ("CJO".into(), 2e-12),
        ("VJ".into(), 0.8),
        ("M".into(), 0.4),
        ("CJP".into(), 3e-13),
        ("PHP".into(), 0.9),
        ("MJSW".into(), 0.3),
        ("TT".into(), 3e-10),
    ]));
    diode.set_sidewall_perimeter(2.0);
    diode.set_ngspice_compatibility(dialect == 1);
    diode.set_xyce_compatibility(dialect == 2);
    diode
}

#[test]
fn diode_event_smooth_composite_regions_retain_canonical_f_q_and_tangents() {
    for dialect in 0..3 {
        let mut diode = model(dialect);
        for temperature in [250.15, 300.15, 350.15] {
            diode.set_temperature(temperature, REFTEMP);
            for v in [-3.1, -0.2, -0.04, 0.05, 0.25, 0.65] {
                assert!(
                    diode.physical_event_locally_c2(v),
                    "dialect={dialect} T={temperature} V={v}"
                );
                let before = diode.nonlinear_state_snapshot();
                let h = diode.vt * 1e-4;
                let (i, g) = diode.current_and_conductance(v);
                let (q, c) = diode.junction_charge_and_capacitance(v);
                let finite_g = (diode.current(v + h) - diode.current(v - h)) / (2.0 * h);
                let finite_c = (diode.junction_charge_and_capacitance(v + h).0
                    - diode.junction_charge_and_capacitance(v - h).0)
                    / (2.0 * h);
                assert!(
                    (g - finite_g).abs() <= 2e-6 * g.abs().max(finite_g.abs()) + 1e-24,
                    "g: {dialect}/{temperature}/{v}: {g:e} vs {finite_g:e}, I={i:e}"
                );
                assert!(
                    (c - finite_c).abs() <= 2e-6 * c.abs().max(finite_c.abs()) + 1e-24,
                    "c: {dialect}/{temperature}/{v}: {c:e} vs {finite_c:e}, Q={q:e}"
                );
                assert_eq!(before, diode.nonlinear_state_snapshot());
            }
        }
    }
}

#[test]
fn diode_event_regularity_retains_constitutive_joins() {
    let mut diode = Diode::spice_defaults("d".into(), 1, 0);
    // Binary-exact voltage scales make the represented joins unambiguous.
    diode.vt = 0.125;
    diode.n = 1.0;
    diode.bv = Some(3.0);
    diode.temperature_breakdown_voltage = Some(3.0);
    diode.breakdown_emission_coefficient = 1.0;
    diode.cj0 = 1e-12;
    diode.vj = 1.0;
    diode.fc = 0.5;
    for join in [-0.375, -3.0, 12.5, -15.5, 0.5] {
        assert!(!diode.physical_event_locally_c2(join), "join {join}");
        for delta in [-1e-6, 1e-6] {
            assert!(
                diode.physical_event_locally_c2(join + delta),
                "side {}",
                join + delta
            );
        }
    }
    // The forward/reverse join is only C1. Its two conductance slopes
    // independently recover the known 4/3 curvature ratio.
    let join = -3.0 * diode.vt;
    let h = diode.vt * 1e-5;
    let left = diode.current_and_conductance(join - h).1;
    let center = diode.current_and_conductance(join).1;
    let right = diode.current_and_conductance(join + h).1;
    assert!(((center - left) / (right - center) - 4.0 / 3.0).abs() < 3e-5);
    diode.sidewall_current_given = true;
    diode.sidewall_saturation_current = 1e-16;
    diode.sidewall_perimeter = 1.0;
    diode.sidewall_emission_given = true;
    diode.sidewall_emission_coefficient = 2.0;
    assert!(!diode.physical_event_locally_c2(-0.75));
    assert!(!diode.physical_event_locally_c2(25.0));
    diode.tunneling.bottom_given = true;
    diode.tunneling.bottom = 1e-20;
    diode.tunneling.emission = 1.0;
    assert!(!diode.physical_event_locally_c2(-12.5));
    diode.tunneling.bottom_given = false;
    diode.sidewall_current_given = false;
    diode.recombination_saturation_current = 1e-16;
    diode.recombination_emission_coefficient = 2.0;
    assert!(!diode.physical_event_locally_c2(25.0));
    for mode in 0..3 {
        diode.set_ngspice_compatibility(mode == 1);
        diode.set_xyce_compatibility(mode == 2);
        assert!(!diode.physical_event_locally_c2(-0.375));
    }
    diode.sidewall_cj0 = 1e-12;
    diode.sidewall_perimeter = 1.0;
    diode.sidewall_vj = 1.5;
    diode.sidewall_fc = 0.5;
    assert!(!diode.physical_event_locally_c2(0.75));
}

#[test]
fn diode_event_injection_switches_use_the_actual_summed_current() {
    for forward in [false, true] {
        let sign = if forward { 1.0 } else { -1.0 };
        let threshold = sign * 1e-18;
        assert!(!knee_locally_c2(threshold, 1e-18, forward));
        assert!(knee_locally_c2(
            sign * (1e-18_f64).next_down(),
            1e-18,
            forward
        ));
        assert!(knee_locally_c2(
            sign * (1e-18_f64).next_up(),
            1e-18,
            forward
        ));
        let evaluate = if forward {
            Diode::apply_forward_knee
        } else {
            Diode::apply_reverse_knee
        };
        let left = evaluate(sign * (1e-18_f64).next_down(), 1.0, 1e-18);
        let right = evaluate(sign * (1e-18_f64).next_up(), 1.0, 1e-18);
        assert!((left.0 - right.0).abs() > 4.9e-19);
    }
    // Recombination joins the knee input only in ngspice. Exercise the shared
    // owner at a real composite bias, not a proxy Shockley-only current.
    for dialect in 0..3 {
        let diode = model(dialect);
        let components = diode.current_components_before_knees(0.25, 0.0);
        let ordinary = diode
            .exponential_current_and_conductance(0.25, diode.bottom_saturation_current(), diode.n)
            .0
            + diode
                .tunnel_current_and_conductance(0.25, diode.tunnel_bottom())
                .0;
        let expected = ordinary + if dialect == 1 { components[2].0 } else { 0.0 };
        assert_eq!(components[0].0.to_bits(), expected.to_bits());
    }
}

#[test]
fn diode_event_regularity_refuses_invalid_or_clamped_charge_charts() {
    let diode = model(0);
    for v in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
        assert!(!diode.physical_event_locally_c2(v));
    }
    for case in 0..5 {
        let mut invalid = diode.clone();
        match case {
            0 => invalid.vt = 0.0,
            1 => invalid.forward_knee_current = Value::INFINITY,
            2 => invalid.recombination_emission_coefficient = 0.0,
            3 => invalid.m = 1000.0,
            _ => invalid.cj0 = Value::INFINITY,
        }
        assert!(!invalid.physical_event_locally_c2(0.25), "case {case}");
    }
}

#[test]
fn xyce_injection_keeps_the_zero_current_chart_and_checks_its_domain() {
    let mut diode = Diode::spice_defaults("d".into(), 1, 0);
    diode.set_xyce_compatibility(true);
    diode.vt = 0.125;
    diode.is = 2.0;
    diode.forward_knee_current = 1.0;
    for v in [-0.01, 0.0, 0.01] {
        assert!(diode.physical_event_locally_c2(v));
        let h = 1e-7;
        let g = diode.current_and_conductance(v).1;
        let finite = (diode.current(v + h) - diode.current(v - h)) / (2.0 * h);
        assert!((g - finite).abs() < 1e-8 * g.abs());
    }
    let boundary = diode.vt * 0.5_f64.ln();
    assert!(!diode.physical_event_locally_c2(boundary));
    assert!(!diode.physical_event_locally_c2(boundary - 1e-6));
    assert!(diode.physical_event_locally_c2(boundary + 1e-6));
    // An overflowing intermediate current ratio must not erase a finite
    // current or tangent in the high-injection asymptote.
    let (i, g) = Diode::apply_xyce_forward_knee(1e200, 1e200, 1e-200);
    assert!((i - 1.0).abs() < 1e-14);
    assert!((g - 0.5).abs() < 1e-14);
}

#[test]
fn xyce_injection_gmin_changes_do_not_reuse_an_incorrect_candidate_or_charge() {
    let mut diode = Diode::spice_defaults("d".into(), 1, 0);
    diode.set_xyce_compatibility(true);
    diode.is = 1e-14;
    diode.forward_knee_current = 1e-3;
    diode.tt = 2e-9;
    let v: f64 = 0.65;
    diode.update(&[v]);
    let model_current = diode.candidate_current_and_conductance(v);
    for gmin in [0.0, 1e-3, 2e-3, 0.0] {
        diode.set_junction_gmin(gmin);
        let raw = diode.is * (v / diode.vt).exp_m1() + gmin * v;
        let raw_g = diode.is * (v / diode.vt).exp() / diode.vt + gmin;
        let r = raw / diode.forward_knee_current;
        let expected_i = raw / (1.0 + r).sqrt();
        let expected_g = raw_g * (1.0 + 0.5 * r) / (1.0 + r).powf(1.5);
        let before = diode.nonlinear_state_snapshot();
        let actual = diode.candidate_stamped_current_and_conductance(v);
        assert!((actual.0 - expected_i).abs() < 1e-14 * expected_i.abs());
        assert!((actual.1 - expected_g).abs() < 1e-14 * expected_g.abs());
        let charge = diode.junction_charge_and_capacitance(v);
        assert!((charge.0 - 2e-9 * expected_i).abs() < 1e-23);
        assert!((charge.1 - 2e-9 * expected_g).abs() < 1e-22);
        assert_eq!(diode.candidate_current_and_conductance(v), model_current);
        assert_eq!(before, diode.nonlinear_state_snapshot());
        diode.seed_accepted_periodic_bias(v);
        let (_, i, g) = diode.limited_linearization(v);
        assert_eq!((i, g), actual);
        assert!(
            diode.linearized_current_matches_candidate(NonlinearConvergenceCriteria::default())
        );
        let (_, id, gd, cd) = diode.op_values();
        assert_eq!((id, gd, cd), (actual.0, actual.1, charge.1));
    }
}

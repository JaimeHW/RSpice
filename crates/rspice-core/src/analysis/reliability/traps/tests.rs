use super::*;
use crate::abort_signal::{ImmediateAbort, NoAbort};

fn model() -> AgingModel {
    AgingModel {
        id: "synthetic-traps".into(),
        mechanism: AgingMechanism::Nbti,
        applicability: "Analytical kinetic fixture, not a process fit".into(),
        validity: AgingValidity {
            gate_source_v: AgingRange {
                min: -1.0,
                max: 0.0,
            },
            drain_source_v: AgingRange {
                min: -1.0,
                max: 1.0,
            },
            temperature_k: AgingRange {
                min: 300.0,
                max: 600.0,
            },
            current_density_a_per_m2: AgingRange { min: 0.0, max: 0.0 },
            max_equivalent_seconds: 1e30,
        },
        law: AgingLaw::TabulatedTwoState {
            table: AgingTrapTable {
                gate_source_v: vec![-1.0, 0.0],
                drain_source_v: vec![-1.0, 1.0],
                temperature_k: vec![300.0, 600.0],
                interpolation: AgingRateInterpolation::Linear,
                traps: vec![AgingTrap {
                    id: "oxide".into(),
                    initial_occupancy: 0.0,
                    capture_rates_per_s: vec![2.0, 0.0, 2.0, 0.0, 2.0, 0.0, 2.0, 0.0],
                    emission_rates_per_s: vec![0.0, 3.0, 0.0, 3.0, 0.0, 3.0, 0.0, 3.0],
                    parameters: vec![AgingTrapParameter {
                        parameter: "VTO".into(),
                        update: AgingParameterUpdate::Additive,
                        shift_per_occupancy: -0.1,
                    }],
                }],
            },
        },
    }
}

fn stress(gate_source_v: f64) -> AgingStress {
    AgingStress {
        gate_source_v,
        drain_source_v: 0.0,
        temperature_k: 400.0,
        current_density_a_per_m2: 0.0,
    }
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 5e-14 * b.abs().max(1e-15), "{a} != {b}");
}

#[test]
fn reliability_trapping_recovers_and_repeats_chronological_histories_without_losing_slow_rates() {
    let model = model();
    let mut cycle = AgingClock::new(&model).unwrap();
    cycle.advance(0.4, stress(-1.0), &NoAbort).unwrap();
    near(
        cycle.evaluate().unwrap().parameters[0].shift,
        -0.1 * (1.0 - (-0.8f64).exp()),
    );
    cycle.advance(0.2, stress(-0.0), &NoAbort).unwrap();
    let b = (1.0 - (-0.8f64).exp()) * (-0.6f64).exp();
    near(cycle.evaluate().unwrap().trap_occupancies[0].occupancy, b);
    let mut repeated = AgingClock::new(&model).unwrap();
    repeated
        .append_repeated_history(&cycle, 7.0, &NoAbort)
        .unwrap();
    near(
        repeated.evaluate().unwrap().trap_occupancies[0].occupancy,
        b * (1.0 - (-1.4f64 * 7.0).exp()) / (1.0 - (-1.4f64).exp()),
    );
    let mut explicit = AgingClock::new(&model).unwrap();
    for _ in 0..7 {
        explicit.advance(0.4, stress(-1.0), &NoAbort).unwrap();
        explicit.advance(0.2, stress(0.0), &NoAbort).unwrap();
    }
    near(
        explicit.evaluate().unwrap().parameters[0].shift,
        repeated.evaluate().unwrap().parameters[0].shift,
    );
    // A threshold suppresses capture but must not freeze existing trapped charge.
    repeated
        .advance_with_activity(0.5, stress(-0.5), false, &NoAbort)
        .unwrap();
    near(
        repeated.evaluate().unwrap().trap_occupancies[0].occupancy,
        explicit.evaluate().unwrap().trap_occupancies[0].occupancy * (-0.75f64).exp(),
    );
    let before = repeated.evaluate().unwrap();
    assert_eq!(
        repeated.append_repeated_history(&cycle, 1.0, &ImmediateAbort),
        Err(AgingError::Aborted)
    );
    assert!(repeated.advance(1.0, stress(-2.0), &NoAbort).is_err());
    assert!(
        repeated
            .append_repeated_history(&cycle, 0.5, &NoAbort)
            .is_err()
    );
    assert_eq!(before, repeated.evaluate().unwrap());

    let mut slow = model.clone();
    let AgingLaw::TabulatedTwoState { table } = &mut slow.law else {
        unreachable!()
    };
    table.traps[0].capture_rates_per_s.fill(1e-25);
    table.traps[0].emission_rates_per_s.fill(0.0);
    let mut one = AgingClock::new(&slow).unwrap();
    one.advance(1.0, stress(-1.0), &NoAbort).unwrap();
    let mut many = AgingClock::new(&slow).unwrap();
    many.append_repeated_history(&one, 1e25, &NoAbort).unwrap();
    near(
        many.evaluate().unwrap().trap_occupancies[0].occupancy,
        1.0 - (-1.0f64).exp(),
    );
    // Long histories reach saturation without a non-finite occupancy.
    many.advance(1e29, stress(-1.0), &NoAbort).unwrap();
    near(many.evaluate().unwrap().trap_occupancies[0].occupancy, 1.0);
}

#[test]
fn reliability_trapping_tables_interpolate_voltage_and_arrhenius_rates_and_validate_domains() {
    let mut model = model();
    let AgingLaw::TabulatedTwoState { table } = &mut model.law else {
        unreachable!()
    };
    table.interpolation = AgingRateInterpolation::Logarithmic;
    // log(c) is affine in Vgs, Vds and 1/T: interpolation must be exact.
    table.traps[0].capture_rates_per_s = table
        .temperature_k
        .iter()
        .flat_map(|t| {
            table.drain_source_v.iter().flat_map(move |d| {
                [-1.0, 0.0]
                    .into_iter()
                    .map(move |g| (g + 0.2 * d - 300.0 / t).exp())
            })
        })
        .collect();
    table.traps[0].emission_rates_per_s.fill(0.25);
    table.traps[0].initial_occupancy = 0.3;
    let mut clock = AgingClock::new(&model).unwrap();
    let s = AgingStress {
        drain_source_v: 0.6,
        ..stress(-0.4)
    };
    let capture = (-0.4 + 0.2 * 0.6 - 300.0 / 400.0f64).exp();
    let equilibrium = capture / (capture + 0.25);
    clock.advance(2.0, s, &NoAbort).unwrap();
    let occupancy = equilibrium + (0.3 - equilibrium) * (-2.0 * (capture + 0.25)).exp();
    near(
        clock.evaluate().unwrap().trap_occupancies[0].occupancy,
        occupancy,
    );
    near(
        clock.evaluate().unwrap().parameters[0].shift,
        -0.1 * (occupancy - 0.3),
    );
    let mut bad = model.clone();
    let AgingLaw::TabulatedTwoState { table } = &mut bad.law else {
        unreachable!()
    };
    table.traps[0].capture_rates_per_s[0] = 0.0;
    assert!(bad.validate().is_err());
    let mut limited = model.clone();
    limited.validity.max_equivalent_seconds = 1.0;
    let mut clock = AgingClock::new(&limited).unwrap();
    let before = clock.evaluate().unwrap();
    assert!(clock.advance(2.0, s, &NoAbort).is_err());
    assert_eq!(clock.evaluate().unwrap(), before);
    // Imported kinetics and signed couplings retain their exact contract.
    assert_eq!(
        serde_json::from_str::<AgingModel>(&serde_json::to_string(&model).unwrap()).unwrap(),
        model
    );
}

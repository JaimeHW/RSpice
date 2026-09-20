//! Synthetic analytical fixtures; none are distributed as process calibration.

use super::*;
use crate::abort_signal::NoAbort;

fn model() -> AgingModel {
    AgingModel {
        id: "synthetic-nbti".into(),
        mechanism: AgingMechanism::Nbti,
        applicability: "test fixture only".into(),
        validity: AgingValidity {
            gate_source_v: AgingRange {
                min: -4.0,
                max: 4.0,
            },
            drain_source_v: AgingRange {
                min: -4.0,
                max: 4.0,
            },
            temperature_k: AgingRange {
                min: 200.0,
                max: 500.0,
            },
            current_density_a_per_m2: AgingRange {
                min: 0.0,
                max: 1e12,
            },
            max_equivalent_seconds: 1e9,
        },
        law: AgingLaw::EquivalentTimePower {
            reference_time_s: 100.0,
            reference_gate_magnitude_v: 1.0,
            reference_drain_magnitude_v: 1.0,
            reference_temperature_k: 300.0,
            gate_polarity: AgingGatePolarity::Negative,
            clock_gate_exponent: 2.0,
            clock_drain_exponent: 0.0,
            clock_activation_energy_ev: 0.0,
            time_exponent: 0.5,
            parameters: vec![AgingParameterScale {
                parameter: "VTO".into(),
                update: AgingParameterUpdate::Additive,
                scale_at_reference_time: -0.01,
            }],
        },
    }
}

fn stress() -> AgingStress {
    AgingStress {
        gate_source_v: -1.0,
        drain_source_v: 0.0,
        temperature_k: 300.0,
        current_density_a_per_m2: 1e10,
    }
}

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= expected.abs().max(1e-10) * 1e-12,
        "{actual} != {expected}"
    );
}

#[test]
fn normalized_clock_preserves_reference_scale_and_piecewise_stress_history() {
    let model = model();
    let mut clock = AgingClock::new(&model).unwrap();
    clock.advance(100.0, stress(), &NoAbort).unwrap();
    near(clock.evaluate().unwrap().parameters[0].shift, -0.01);
    // Four times the aging clock at twice the gate magnitude.
    clock
        .advance(
            75.0,
            AgingStress {
                gate_source_v: -2.0,
                ..stress()
            },
            &NoAbort,
        )
        .unwrap();
    let result = clock.evaluate().unwrap();
    near(result.equivalent_seconds, 400.0);
    near(result.parameters[0].shift, -0.02);
    // Reversed gate stress adds elapsed time, without inventing recovery.
    clock
        .advance(
            20.0,
            AgingStress {
                gate_source_v: 1.0,
                ..stress()
            },
            &NoAbort,
        )
        .unwrap();
    near(clock.evaluate().unwrap().parameters[0].shift, -0.02);
    near(clock.evaluate().unwrap().elapsed_seconds, 195.0);
}

#[test]
fn black_current_and_temperature_acceleration_retains_lifetime_not_resistance() {
    let mut model = model();
    model.mechanism = AgingMechanism::Electromigration;
    model.law = AgingLaw::BlackElectromigration {
        reference_lifetime_s: 1000.0,
        reference_current_density_a_per_m2: 1e10,
        reference_temperature_k: 300.0,
        current_exponent: 2.0,
        activation_energy_ev: 0.5,
    };
    let mut clock = AgingClock::new(&model).unwrap();
    clock
        .advance(
            250.0,
            AgingStress {
                current_density_a_per_m2: 2e10,
                ..stress()
            },
            &NoAbort,
        )
        .unwrap();
    let result = clock.evaluate().unwrap();
    near(result.electromigration_lifetime_fraction.unwrap(), 1.0);
    assert!(result.parameters.is_empty());
    // Select a temperature that analytically doubles the Arrhenius rate.
    let k = 1.380649e-23 / 1.602176634e-19;
    let temperature_k = 1.0 / (1.0 / 300.0 - k / 0.5 * 2.0_f64.ln());
    clock
        .advance(
            500.0,
            AgingStress {
                temperature_k,
                ..stress()
            },
            &NoAbort,
        )
        .unwrap();
    near(
        clock
            .evaluate()
            .unwrap()
            .electromigration_lifetime_fraction
            .unwrap(),
        2.0,
    );
}

#[test]
fn invalid_out_of_domain_overflow_and_cancelled_intervals_do_not_change_clock() {
    let model = model();
    let mut clock = AgingClock::new(&model).unwrap();
    let fresh = clock.evaluate().unwrap();
    assert!(matches!(
        clock.advance(
            1.0,
            AgingStress {
                temperature_k: 501.0,
                ..stress()
            },
            &NoAbort
        ),
        Err(AgingError::OutsideCalibration(_))
    ));
    assert!(matches!(
        clock.advance(1e10, stress(), &NoAbort),
        Err(AgingError::OutsideCalibration(_))
    ));
    assert!(clock.advance(f64::NAN, stress(), &NoAbort).is_err());
    let abort = crate::abort_signal::ImmediateAbort;
    assert_eq!(
        clock.advance(1.0, stress(), &abort),
        Err(AgingError::Aborted)
    );
    assert_eq!(clock.evaluate().unwrap(), fresh);
    let mut extreme = model.clone();
    if let AgingLaw::EquivalentTimePower {
        clock_gate_exponent,
        ..
    } = &mut extreme.law
    {
        *clock_gate_exponent = f64::MAX;
    }
    let mut clock = AgingClock::new(&extreme).unwrap();
    assert!(matches!(
        clock.advance(
            1.0,
            AgingStress {
                gate_source_v: -4.0,
                ..stress()
            },
            &NoAbort
        ),
        Err(AgingError::Numeric(_))
    ));
    assert_eq!(clock.evaluate().unwrap().equivalent_seconds, 0.0);
}

#[test]
fn pack_loading_refuses_unknown_units_missing_provenance_duplicate_names_and_wrong_laws() {
    let pack = AgingModelPack {
        schema_version: 1,
        id: "synthetic".into(),
        process: "test only".into(),
        qualification: AgingQualification::UserCharacterized,
        source: "analytical test fixture".into(),
        license: "test fixture".into(),
        characterization: "not a physical calibration".into(),
        models: vec![model()],
    };
    let json = serde_json::to_string(&pack).unwrap();
    assert_eq!(AgingModelPack::from_json(&json).unwrap(), pack);
    assert!(
        AgingModelPack::from_json(
            &json.replace("reference_temperature_k", "reference_temperature_c")
        )
        .is_err()
    );
    let mut bad = pack.clone();
    bad.source.clear();
    assert!(bad.validate().is_err());
    let mut bad = pack.clone();
    bad.models.push(model());
    assert!(bad.validate().is_err());
    let mut bad = pack;
    bad.models[0].mechanism = AgingMechanism::Electromigration;
    assert!(bad.validate().is_err());
}

#[test]
fn reference_exposure_reaches_exact_calibration_boundary_and_late_abort_is_atomic() {
    let mut model = model();
    model.validity.max_equivalent_seconds = 100.0;
    if let AgingLaw::EquivalentTimePower {
        clock_activation_energy_ev,
        ..
    } = &mut model.law
    {
        *clock_activation_energy_ev = f64::MAX;
    }
    let mut clock = AgingClock::new(&model).unwrap();
    let abort = crate::abort_signal::CountingAbort::new(1);
    assert_eq!(
        clock.advance(100.0, stress(), &abort),
        Err(AgingError::Aborted)
    );
    assert_eq!(clock.evaluate().unwrap().equivalent_seconds, 0.0);
    clock.advance(100.0, stress(), &NoAbort).unwrap();
    assert_eq!(clock.evaluate().unwrap().equivalent_seconds, 100.0);
}

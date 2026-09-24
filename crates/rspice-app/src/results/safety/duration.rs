//! Engine-facing adapter for portable SOA duration qualification.
#[cfg(test)]
use super::{SoARuleVerdict, SoaThresholds};
use super::{SoaDurationMode, SoaLimitTrace};
use rspice_core::{SimulationError, abort_signal::AbortSignal};
use rspice_results::safety::{SoaDurationScan, SoaDurationScanError, scan_soa_duration_with_mode};

/// Qualifications are retrospective: the complete excursion's width decides
/// whether its above-limit samples are violations. Observation-window edges
/// clip the width; no duration outside that window is inferred.
#[cfg(test)]
fn qualify_soa_duration(
    time: &[f64],
    stress: &[f64],
    limits: SoaLimitTrace<'_>,
    minimum_duration_s: f64,
    abort: &dyn AbortSignal,
) -> Result<SoaDurationScan, SimulationError> {
    qualify_soa_duration_with_mode(
        time,
        stress,
        limits,
        minimum_duration_s,
        SoaDurationMode::PerExcursion,
        abort,
    )
}

/// Preserve engine cancellation and error semantics at the execution boundary.
pub fn qualify_soa_duration_with_mode(
    time: &[f64],
    stress: &[f64],
    limits: SoaLimitTrace<'_>,
    minimum_duration_s: f64,
    mode: SoaDurationMode,
    abort: &dyn AbortSignal,
) -> Result<SoaDurationScan, SimulationError> {
    scan_soa_duration_with_mode(time, stress, limits, minimum_duration_s, mode, || {
        abort.is_aborted()
    })
    .map_err(|error| match error {
        SoaDurationScanError::InvalidMinimumDuration(value) => {
            rspice_core::config::SimulationConfigError::InvalidValue {
                field: "soa.minimum_duration_s",
                value,
                requirement: "finite and positive",
            }
            .into()
        }
        SoaDurationScanError::InvalidInput(message) => SimulationError::Circuit(message),
        SoaDurationScanError::Aborted => SimulationError::Aborted,
    })
}

#[cfg(test)]
#[test]
fn soa_duration_cumulative_exposure_recovers_only_between_excursions() {
    use rspice_core::abort_signal::NoAbort;
    // Excursions [0.25, 1.75] and [2.5, 5.5]: widths 1.5 and 3, gap 0.75.
    let time = [0., 1., 2., 3., 4., 5., 6.];
    let stress = [0., 4., 0., 2., 2., 2., 0.];
    for (recovery_time_s, qualified) in [(None, 1), (Some(0.1), 0), (Some(3.), 1)] {
        let scan = qualify_soa_duration_with_mode(
            &time,
            &stress,
            SoaLimitTrace::Constant(1.),
            4.,
            SoaDurationMode::Cumulative { recovery_time_s },
            &NoAbort,
        )
        .unwrap();
        let evidence = scan.evidence.cumulative.unwrap();
        let peak = recovery_time_s.map_or(4.5, |tau| 3. + 1.5 * (-0.75 / tau).exp());
        let final_exposure = recovery_time_s.map_or(peak, |tau| peak * (-0.5 / tau).exp());
        assert!((evidence.peak_exposure_s - peak).abs() < 1e-14);
        assert!((evidence.final_exposure_s - final_exposure).abs() < 1e-14);
        assert_eq!(scan.evidence.qualified_excursions, qualified);
        assert_eq!(scan.evidence.rejected_excursions, 2 - qualified);
        assert!(
            !scan.qualified_samples[1],
            "later exposure must not reclassify the first pulse"
        );
        assert_eq!(scan.qualified_samples[4], qualified == 1);
    }
    // Starting inside an excursion cannot infer earlier stress, and a long gap
    // lets a later excursion fall below threshold even after a qualified one.
    let clipped = qualify_soa_duration_with_mode(
        &[5., 6., 7., 8., 20., 21.],
        &[2., 2., 0., 0., 0., 2.],
        SoaLimitTrace::Constant(1.),
        1.,
        SoaDurationMode::Cumulative {
            recovery_time_s: Some(1.),
        },
        &NoAbort,
    )
    .unwrap();
    assert_eq!(clipped.evidence.clipped_excursions, 2);
    assert!(clipped.excursions[0].qualified);
    assert!(!clipped.excursions[1].qualified);
    assert!(
        (clipped.evidence.cumulative.unwrap().final_exposure_s - (0.5 + 1.5 * (-14_f64).exp()))
            .abs()
            < 1e-14
    );
    let no_stress = qualify_soa_duration_with_mode(
        &[0., 1.],
        &[0., 0.],
        SoaLimitTrace::Constant(1.),
        1.,
        SoaDurationMode::Cumulative {
            recovery_time_s: None,
        },
        &NoAbort,
    )
    .unwrap();
    assert_eq!(no_stress.evidence.cumulative.unwrap().peak_exposure_s, 0.);
    for recovery_time_s in [Some(0.), Some(-1.), Some(f64::INFINITY), Some(f64::NAN)] {
        assert!(
            SoaDurationMode::Cumulative { recovery_time_s }
                .validate(Some(1.))
                .is_err()
        );
    }
    assert!(
        SoaDurationMode::Cumulative {
            recovery_time_s: None
        }
        .validate(None)
        .is_err()
    );
    struct Abort;
    impl AbortSignal for Abort {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        qualify_soa_duration_with_mode(
            &time,
            &stress,
            SoaLimitTrace::Constant(1.),
            4.,
            SoaDurationMode::Cumulative {
                recovery_time_s: None
            },
            &Abort,
        ),
        Err(SimulationError::Aborted)
    ));
    assert!(matches!(
        qualify_soa_duration_with_mode(
            &time,
            &stress,
            SoaLimitTrace::Constant(1.),
            0.,
            SoaDurationMode::PerExcursion,
            &NoAbort,
        ),
        Err(SimulationError::Configuration(
            rspice_core::config::SimulationConfigError::InvalidValue {
                field: "soa.minimum_duration_s",
                ..
            }
        ))
    ));
    assert!(matches!(
        qualify_soa_duration_with_mode(
            &time,
            &stress[..2],
            SoaLimitTrace::Constant(1.),
            1.,
            SoaDurationMode::PerExcursion,
            &NoAbort,
        ),
        Err(SimulationError::Circuit(message))
            if message == "SOA duration traces have incomplete sample coverage"
    ));
}

#[cfg(test)]
#[test]
fn soa_duration_qualifies_interpolated_excursions_and_reclassifies_worst_point() {
    use super::{SoADefinition, SoALimit, SoAManager, SoAParameter};
    use rspice_core::abort_signal::NoAbort;
    let time = [0., 1., 2., 3., 4., 5., 6.];
    let stress = [0., 4., 0., 2., 2., 2., 0.];
    let scan =
        qualify_soa_duration(&time, &stress, SoaLimitTrace::Constant(1.), 2., &NoAbort).unwrap();
    assert_eq!(scan.evidence.total_exceedance_s, 4.5);
    assert_eq!(scan.evidence.longest_excursion_s, 3.0);
    assert_eq!(scan.evidence.qualified_excursions, 1);
    assert_eq!(scan.evidence.rejected_excursions, 1);
    assert_eq!(scan.evidence.clipped_excursions, 0);
    assert_eq!(
        scan.qualified_samples,
        vec![false, false, false, true, true, true, false]
    );
    assert_eq!(
        (scan.excursions[0].start_s, scan.excursions[0].end_s),
        (0.25, 1.75)
    );
    assert_eq!(
        (scan.excursions[1].start_s, scan.excursions[1].end_s),
        (2.5, 5.5)
    );
    for warning in [Some(0.9), None] {
        let mut manager = SoAManager::with_thresholds(SoaThresholds {
            warning_fraction: warning,
            ..Default::default()
        })
        .unwrap();
        manager
            .register_device(
                "M1",
                SoADefinition {
                    limits: vec![SoALimit {
                        duration_mode: Default::default(),
                        minimum_duration_s: Some(2.),
                        current_envelope: None,
                        power_derating: None,
                        voltage_basis: Default::default(),
                        parameter: SoAParameter::Vds,
                        max_value: 1.,
                        unit: "V".into(),
                        description: "Drain voltage".into(),
                    }],
                },
            )
            .unwrap();
        for (t, actual) in time.into_iter().zip(stress) {
            manager
                .check_point(
                    t,
                    &std::collections::HashMap::from([(
                        "M1".into(),
                        std::collections::HashMap::from([(SoAParameter::Vds, actual)]),
                    )]),
                )
                .unwrap();
        }
        assert_eq!(manager.evaluations().next().unwrap().worst_actual_value, 4.);
        manager.finalize_durations(&time, &NoAbort).unwrap();
        let evaluation = manager.evaluations().next().unwrap();
        assert_eq!(evaluation.worst_actual_value, 2.);
        assert_eq!(evaluation.worst_time, 3.);
        assert_eq!(evaluation.verdict, SoARuleVerdict::Critical);
        assert_eq!(evaluation.duration, Some(scan.evidence));
        assert_eq!(
            manager.violations().len(),
            if warning.is_some() { 4 } else { 3 }
        );
    }
    let varying = qualify_soa_duration(
        &[0., 1., 2., 3.],
        &[0.5; 4],
        SoaLimitTrace::Samples(&[1., 0., 0., 1.]),
        2.,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(varying.evidence.longest_excursion_s, 2.);
    assert_eq!(varying.qualified_samples, vec![false, true, true, false]);
    let extreme = qualify_soa_duration(
        &[0., 1., 2.],
        &[0., f64::MAX, 0.],
        SoaLimitTrace::Samples(&[f64::MAX, 0., f64::MAX]),
        1.,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(
        (extreme.excursions[0].start_s, extreme.excursions[0].end_s),
        (0.5, 1.5)
    );
    let clipped = qualify_soa_duration(
        &[0., 1., 2.],
        &[2., 1., 2.],
        SoaLimitTrace::Constant(1.),
        1.5,
        &NoAbort,
    )
    .unwrap();
    assert_eq!(clipped.evidence.rejected_excursions, 2);
    assert_eq!(clipped.evidence.clipped_excursions, 2);
    assert_eq!(clipped.evidence.total_exceedance_s, 2.);
    assert_eq!(clipped.evidence.longest_excursion_s, 1.);
    struct Abort;
    impl AbortSignal for Abort {
        fn is_aborted(&self) -> bool {
            true
        }
    }
    assert!(matches!(
        qualify_soa_duration(&time, &stress, SoaLimitTrace::Constant(1.), 2., &Abort),
        Err(SimulationError::Aborted)
    ));
}

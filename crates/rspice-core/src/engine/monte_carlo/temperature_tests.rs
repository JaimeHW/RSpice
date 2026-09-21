//! A physical trial temperature must precede expression and conditional replay.
use super::*;

#[test]
fn monte_carlo_temperature_precedes_replay_and_preserves_trial_state() {
    let engine = Engine::default();
    for variation in [
        MonteCarloVariationSource::ParameterTolerance,
        MonteCarloVariationSource::DeckStatistics,
    ] {
        let r = if variation == MonteCarloVariationSource::DeckStatistics {
            "{aunif(1000,200)}"
        } else {
            "1000"
        };
        let source = format!(
            "MC thermal replay\n.param R={r} OFFSET=10\nV1 in 0 DC 1\nRS in out {{R*(1+TEMP/100)+OFFSET}}\n.if (TEMP>30)\nRHOT out 0 2k\n.else\nRCOLD out 0 1k\n.endif\nVKEEP keep 0 1\nRKEEP keep 0 1k\n.options TEMP=12 GMIN=0\n.end\n"
        );
        let mut nominal = Netlist::parse(&source).unwrap();
        nominal.ast_overlay.parameters.insert("OFFSET".into(), 15.0);
        nominal
            .ast_overlay
            .device_parameters
            .insert(("RKEEP".into(), "R".into()), 1234.0);
        nominal.spectre_statistical_coordinate =
            Some(crate::netlist::SpectreStatisticalCoordinate {
                axes: vec![("outer".into(), 7.0)],
                ..Default::default()
            });
        let replay = engine
            .step_temperature_netlist(&nominal, 37.0, &NoAbort)
            .unwrap()
            .0;
        let coordinate = replay.spectre_statistical_coordinate.as_ref().unwrap();
        assert_eq!(coordinate.axes, [("outer".into(), 7.0)]);
        assert_eq!(coordinate.temperature_celsius, 37.0);
        let mut draws = Vec::new();
        for temperature in [17.0, 37.0] {
            let mut study = MonteCarloStudyConfig::new(3, 31, vec!["draw".into()]);
            study.first_trial = 2;
            study.distribution = Distribution::Uniform { tolerance: 0.2 };
            study.variation_source = variation;
            if variation == MonteCarloVariationSource::ParameterTolerance {
                study.parameter_filter = vec!["R".into()];
            }
            study.environment = Some(MonteCarloEnvironment {
                temperature_celsius: temperature,
                supply_voltage: Some(2.0),
                nominal_supply_voltage: Some(1.0),
                supply_source_names: vec!["V1".into()],
            });
            let result = engine
                .run_monte_carlo_measurements_with_abort(
                    &nominal,
                    &study,
                    &NoAbort,
                    |engine, trial, _, abort| {
                        assert_eq!(trial.params.get("TEMP"), Some(temperature));
                        assert_eq!(trial.params.get("TEMPER"), Some(temperature));
                        assert!(
                            (trial.params.get("VT").unwrap()
                                - crate::constants::thermal_voltage(temperature + 273.15))
                            .abs()
                                < 1e-15
                        );
                        assert_eq!(trial.params.get("OFFSET"), Some(15.0));
                        assert_eq!(
                            trial
                                .elements
                                .iter()
                                .any(|element| element.name.eq_ignore_ascii_case("RHOT")),
                            temperature > 30.0
                        );
                        let r = trial.params.get("R").unwrap();
                        let point = engine.run_dc_op_with_abort(trial, abort)?;
                        let index = point
                            .node_names
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case("out"))
                            .unwrap();
                        let load = if temperature > 30.0 { 2000.0 } else { 1000.0 };
                        let expected = 2.0 * load / (load + r * (1.0 + temperature / 100.0) + 15.0);
                        assert!((point.node_voltages[index] - expected).abs() < 1e-9);
                        let branch = point
                            .branch_names
                            .iter()
                            .position(|name| name.eq_ignore_ascii_case("VKEEP"))
                            .unwrap();
                        assert!((point.branch_currents[branch] + 1.0 / 1234.0).abs() < 1e-12);
                        Ok(vec![r])
                    },
                )
                .unwrap();
            assert_eq!(result.num_failures, 0);
            assert_eq!(
                result.successful_trial_indices.as_deref(),
                Some(&[2, 3, 4][..])
            );
            draws.push(result.variables["draw"].samples.clone());
        }
        assert_eq!(
            draws[0], draws[1],
            "temperature must not redraw the original random coordinate"
        );
        assert!(draws[0].windows(2).any(|pair| pair[0] != pair[1]));
    }
    let nominal =
        Netlist::parse("fixed environment\n.param R=1000\nV1 out 0 1\nR1 out 0 {R}\n.end\n")
            .unwrap();
    let mut study = MonteCarloStudyConfig::new(2, 1, vec!["temperature".into()]);
    study.environment = Some(MonteCarloEnvironment {
        temperature_celsius: 37.0,
        supply_voltage: None,
        nominal_supply_voltage: None,
        supply_source_names: vec![],
    });
    let result = engine
        .run_monte_carlo_measurements_with_abort(&nominal, &study, &NoAbort, |_, trial, _, _| {
            Ok(vec![trial.params.get("TEMP").unwrap()])
        })
        .unwrap();
    assert_eq!(result.variables["temperature"].samples, [37.0, 37.0]);
    study.parameter_filter = vec!["TEMP".into()];
    assert!(
        engine
            .run_monte_carlo_measurements_with_abort(&nominal, &study, &NoAbort, |_, _, _, _| Ok(
                vec![0.0]
            ))
            .is_err()
    );
    assert!(matches!(
        engine.step_temperature_netlist(&nominal, 37.0, &crate::abort_signal::ImmediateAbort),
        Err(SimulationError::Aborted)
    ));
    assert!(
        engine
            .step_temperature_netlist(&nominal, -273.15, &NoAbort)
            .is_err()
    );
}

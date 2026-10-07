//! Completion and allocation contracts of the shared row driver.
use super::*;
use crate::{ModelFinish, ModelFinishPoint, NoAbort};

fn table() -> Netlist {
    Netlist::parse("Finish table\n.data points FREQ\n1\n2\n3\n.enddata\n.end\n").unwrap()
}

fn options() -> FrequencyDataOptions {
    FrequencyDataOptions {
        analysis: ".AC",
        positive_frequency: false,
        retain_netlists: false,
        default_temperature: None,
    }
}

fn finish() -> ModelFinish {
    ModelFinish {
        instance: "model_instance".into(),
        model: "model".into(),
        site: 1,
        point: ModelFinishPoint::Frequency { frequency: 2.0 },
        diagnostic_level: 0,
    }
}

#[test]
fn accepted_finish_retains_coordinates_and_identifies_the_requested_prefix() {
    let engine = Engine::default();
    let (_, result) = engine
        .run_frequency_data(
            &table(),
            "points",
            options(),
            &NoAbort,
            |_, _, frequency, abort| {
                if frequency == 2.0 {
                    abort.model_control().unwrap().request_finish(finish());
                }
                Ok(vec![frequency])
            },
            |_| 1,
        )
        .unwrap();
    assert_eq!(result.points, [1.0, 2.0]);
    assert_eq!(result.columns[0].values, result.points);
    assert_eq!(result.requested_rows, 3);
    assert_eq!(result.finish, Some(finish()));
    // A table scope cannot poison later runs on the same engine.
    let (_, next) = engine
        .run_frequency_data(
            &table(),
            "points",
            options(),
            &NoAbort,
            |_, _, frequency, _| Ok(vec![frequency]),
            |_| 1,
        )
        .unwrap();
    assert_eq!(next.points, [1.0, 2.0, 3.0]);
    assert!(next.finish.is_none());
}

#[test]
fn finish_before_a_new_row_excludes_that_rows_coordinates() {
    for stop in [1.0, 2.0] {
        let outcome = Engine::default().run_frequency_data(
            &table(),
            "points",
            options(),
            &NoAbort,
            |_, _, frequency, _| {
                if frequency == stop {
                    Err(SimulationError::ModelFinished(Box::new(finish())))
                } else {
                    Ok(vec![frequency])
                }
            },
            |_| 1,
        );
        if stop == 1.0 {
            assert!(matches!(outcome, Err(SimulationError::ModelFinished(_))));
        } else {
            let (_, result) = outcome.unwrap();
            assert_eq!(result.points, [1.0]);
            assert_eq!(result.columns[0].values, [1.0]);
            assert_eq!(result.requested_rows, 3);
            assert_eq!(result.finish, Some(finish()));
        }
    }
}

#[test]
fn allocator_refusal_preserves_its_typed_cause() {
    let error = reserve(
        &mut Vec::<u8>::new(),
        usize::MAX,
        "frequency table coordinates",
    )
    .unwrap_err();
    assert!(matches!(error, SimulationError::Allocation { .. }));
}

#[test]
fn temperature_coordinate_preserves_unrelated_resolved_policy() {
    let netlist = Netlist::parse("Policy\n.options temp=27 itl1=99 reltol=.01\n.data points FREQ TEMP\n1 27\n2 127\n.enddata\n.end\n").unwrap();
    let base = Engine::default();
    let mut config = base.config().clone();
    config.temperature = 350.0;
    config.max_iterations = 17;
    config.convergence_config.voltage_reltol = 1e-5;
    let engine = base.try_resolved_with_config(config).unwrap();
    let (_, result) = engine
        .run_frequency_data(
            &netlist,
            "points",
            options(),
            &NoAbort,
            |row_engine, _, _, _| {
                let config = row_engine.config();
                assert_eq!(config.max_iterations, 17);
                assert_eq!(config.convergence_config.voltage_reltol, 1e-5);
                Ok(vec![config.temperature])
            },
            |_| 1,
        )
        .unwrap();
    assert_eq!(result.points, [300.15, 400.15]);
}

#[test]
fn row_parameter_snapshots_preserve_complex_dependencies_and_accepted_prefixes() {
    let netlist = Netlist::parse("Table parameters\n.param P=1 Q={2*P} C={P*1j}\n.data points FREQ P\n10 1\n20 5\n10 7\n.enddata\n.end\n").unwrap();
    for stop_before_third in [false, true] {
        let (_, result) = Engine::default()
            .run_frequency_data(
                &netlist,
                "points",
                options(),
                &NoAbort,
                |_, row, frequency, _| {
                    if stop_before_third && row.params.get("P") == Some(7.0) {
                        return Err(SimulationError::ModelFinished(Box::new(finish())));
                    }
                    Ok(vec![frequency])
                },
                |_| 1,
            )
            .unwrap();
        let count = if stop_before_third { 2 } else { 3 };
        assert_eq!(result.parameters.point_count(), count);
        for (row, expected) in [1.0, 5.0, 7.0].into_iter().take(count).enumerate() {
            assert_eq!(
                result.parameters.resolve("P", row).unwrap().unwrap().re,
                expected
            );
            assert_eq!(
                result.parameters.resolve("q", row).unwrap().unwrap().re,
                2.0 * expected
            );
            assert_eq!(
                result.parameters.resolve("C", row).unwrap().unwrap().im,
                expected
            );
        }
        assert_eq!(result.points.len(), count);
        assert_eq!(result.finish.is_some(), stop_before_third);
    }
}

#[test]
fn row_parameter_storage_is_charged_before_solver_execution() {
    let netlist = Netlist::parse(
        "Budget\n.param P=1 Q={2*P} C={P*1j}\n.data points FREQ P\n10 5\n.enddata\n.end\n",
    )
    .unwrap();
    let base = Engine::default();
    let mut config = base.config().clone();
    // Two coordinates fit, but the resolved parameter columns do not.
    config.resource_limits.max_result_values = 2;
    let error = base
        .try_resolved_with_config(config)
        .unwrap()
        .run_frequency_data(
            &netlist,
            "points",
            options(),
            &NoAbort,
            |_, _, _, _| panic!("over-budget row must not reach solver"),
            |_: &f64| 1,
        )
        .unwrap_err();
    assert!(
        matches!(error, SimulationError::ResourceLimit(error) if error.resource == ResourceKind::ResultValues && error.limit == 2)
    );
}

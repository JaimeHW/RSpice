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

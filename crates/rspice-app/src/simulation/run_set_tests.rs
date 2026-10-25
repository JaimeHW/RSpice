//! Application corner fixtures and run-space executor integration.

use rspice_simulation_contract::corner_config::CornerBaseAnalysis;
use rspice_simulation_contract::run_set::{
    ReferencePoint, RunSetAction, RunSetDimensionKind, RunSetState, compose, dispatch, resolve,
};
use rspice_vocabulary::product::ProcessCorner;

fn reference() -> ReferencePoint {
    ReferencePoint {
        process: ProcessCorner::TT,
        temperature_celsius: 27.0,
    }
}

fn bound_default() -> RunSetState {
    let mut state = RunSetState::default();
    state
        .dimensions
        .iter_mut()
        .find(|dimension| dimension.kind == RunSetDimensionKind::Supply)
        .expect("the default run set declares a supply axis")
        .source = "netlist-source:VDD".to_owned();
    state
}

fn set_values(state: &mut RunSetState, kind: RunSetDimensionKind, text: &str) {
    let id = state
        .dimensions
        .iter()
        .find(|dimension| dimension.kind == kind)
        .expect("the default run set declares every kind")
        .id
        .clone();
    let transaction = dispatch(
        state,
        RunSetAction::SetValues {
            id,
            text: text.to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
}

fn key_of(state: &RunSetState, index: usize) -> String {
    compose(state)
        .expect("the space composes")
        .get(index)
        .expect("the composed space reaches this point")
        .point_key()
}

fn exclude(state: &mut RunSetState, key: &str) {
    let transaction = dispatch(
        state,
        RunSetAction::ExcludePoint {
            key: key.to_owned(),
        },
        1,
    );
    assert!(transaction.was_adopted(), "{:?}", transaction.receipt);
}

#[test]
fn a_filtered_space_reaches_the_executor_as_the_points_it_resolved() {
    let mut state = bound_default();
    set_values(&mut state, RunSetDimensionKind::ProcessSection, "TT\nSS");
    set_values(&mut state, RunSetDimensionKind::Supply, "0.9\n1.1");
    set_values(&mut state, RunSetDimensionKind::Temperature, "-40\n125");
    let key = key_of(&state, 5);
    exclude(&mut state, &key);

    let config = state
        .to_corner_config(CornerBaseAnalysis::Op, reference())
        .expect("a filtered space is executable");
    assert_eq!(config.points.len(), 7);
    assert_eq!(config.num_corners(), 7);

    // Same count, same coordinates, same order as the run set resolved.
    let resolved = resolve(&state).expect("the filtered space resolves");
    assert_eq!(config.points.len(), resolved.len());
    for (point, spec) in resolved.iter().zip(&config.points) {
        assert_eq!(
            point.label(),
            format!(
                "{} · {} V · {} °C",
                spec.process.short_name(),
                spec.voltage,
                spec.temperature_celsius
            )
        );
    }

    // The expansion the executor and operating-point preparation share must
    // return exactly that list, not the matrix the axes would rebuild.
    let expanded = rspice_simulation::sweeps::expand_corner_pvt_points(
        &rspice_simulation::sweeps::CornerRunConfig {
            process_corners: vec![
                rspice_vocabulary::product::ProcessCorner::TT,
                rspice_vocabulary::product::ProcessCorner::SS,
            ],
            voltages: config.voltages.clone(),
            temperatures_c: config.temperatures.clone(),
            full_matrix: true,
            points: config
                .points
                .iter()
                .map(|point| rspice_simulation::sweeps::CornerPoint {
                    process: point.process,
                    voltage: point.voltage,
                    temperature_c: point.temperature_celsius,
                })
                .collect(),
            ..Default::default()
        },
    )
    .expect("an explicit list expands");
    assert_eq!(expanded.len(), 7);
    for (index, (_, voltage, temperature)) in expanded.iter().enumerate() {
        assert_eq!(*voltage, config.points[index].voltage);
        assert_eq!(*temperature, config.points[index].temperature_celsius);
    }
}

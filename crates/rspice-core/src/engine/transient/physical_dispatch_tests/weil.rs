use super::*;

fn check_ngspice_reference(source: &str, reference: &str) {
    let rows: Vec<[Value; 3]> = reference
        .lines()
        .skip(1)
        .map(|line| {
            let values: Vec<Value> = line
                .split_whitespace()
                .map(|value| value.parse().unwrap())
                .collect();
            values.try_into().unwrap()
        })
        .collect();
    let grid = std::sync::Arc::new(rows.iter().map(|row| row[0]).collect::<Vec<_>>());
    let stop = *grid.last().unwrap();
    let checkpoint_time = grid[grid.len() / 2];
    let config = SimulationConfig {
        gp_transient_phase_model: crate::GpTransientPhaseModel::NgspiceWeil,
        integration_method: IntegrationMethod::Trapezoidal,
        locked_time_grid: Some(grid.clone()),
        ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice)
    };
    let (result, checkpoints) = run_with_configuration(
        source,
        stop,
        4e-12,
        None,
        &[checkpoint_time],
        config.clone(),
    );
    assert!(
        result.time == *grid,
        "legacy comparison must retain every recorded ngspice time"
    );
    let current = result.try_branch_current_waveform_named("vc").unwrap();
    let base = result.try_voltage_waveform_named("b").unwrap();
    let minimum = rows
        .iter()
        .map(|row| row[1])
        .fold(Value::INFINITY, Value::min);
    let maximum = rows
        .iter()
        .map(|row| row[1])
        .fold(Value::NEG_INFINITY, Value::max);
    let budget = 1e-14 + 2e-6 * (maximum - minimum) / 2.0;
    let error = rows
        .iter()
        .zip(current)
        .map(|(row, value)| (row[1] - value).abs())
        .fold(0.0, Value::max);
    eprintln!(
        "GP Weil actual ngspice: points={}, current_error={error:e}, budget={budget:e}",
        rows.len()
    );
    let initial_offset = current[0] - rows[0][1];
    let (worst_index, dynamic_error) = rows
        .iter()
        .zip(current)
        .enumerate()
        .map(|(index, (row, value))| (index, (value - row[1] - initial_offset).abs()))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap();
    eprintln!(
        "initial={} reference={} offset={initial_offset:e}; dynamic_error={dynamic_error:e} at {}",
        current[0], rows[0][1], rows[worst_index][0]
    );
    assert!(
        error < budget,
        "legacy current error {error:e} exceeds {budget:e}"
    );
    for (row, value) in rows.iter().zip(base) {
        assert!((row[2] - value).abs() < 1e-12);
    }
    let checkpoint = &checkpoints[0].checkpoint;
    let history = &checkpoint.accepted_junction_transient_history().bjt_history;
    assert!(history.phase[0].is_none());
    let weil = history.weil_phase[0].as_ref().unwrap();
    assert_eq!(weil.time, checkpoint_time);
    assert!(weil.previous_dt > 0.0);
    // The packed image carries accepted filter outputs, input, own step size
    // and nominal delay. A configuration change cannot reuse this image.
    let restored = TransientCheckpoint::from_bytes(
        &checkpoint
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        restored
            .accepted_junction_transient_history()
            .bjt_history
            .weil_phase,
        history.weil_phase
    );
    let parsed = Netlist::parse(source).unwrap();
    let resolved = Engine::new(config.clone()).resolved_for_netlist(&parsed);
    let mut changed = resolved.config.clone();
    changed.gp_transient_phase_model = crate::GpTransientPhaseModel::ExactDelay;
    assert!(
        restored
            .validate_for_with_config(&parsed, &changed)
            .is_err()
    );
    let (resumed, _) = run_with_configuration(source, stop, 4e-12, Some(&restored), &[], config);
    let seam = result
        .time
        .iter()
        .position(|&time| time == checkpoint_time)
        .unwrap();
    assert!(resumed.time == result.time[seam..]);
    for (resumed, full) in resumed
        .voltages
        .iter()
        .chain(&resumed.branch_currents)
        .zip(result.voltages.iter().chain(&result.branch_currents))
    {
        assert!(
            resumed == &full[seam..],
            "Weil checkpoint changed accepted samples"
        );
    }
}

#[test]
fn gp_weil_matches_recorded_ngspice_ptf21_and_resume() {
    check_ngspice_reference(
        include_str!("../../../../tests/testdata/gp_weil_p21_ngspice46.cir"),
        include_str!("../../../../tests/testdata/gp_weil_p21_ngspice46.tsv"),
    );
}

#[test]
fn gp_weil_matches_recorded_ngspice_ptf90_and_resume() {
    check_ngspice_reference(
        include_str!("../../../../tests/testdata/gp_weil_p90_ngspice46.cir"),
        include_str!("../../../../tests/testdata/gp_weil_p90_ngspice46.tsv"),
    );
}

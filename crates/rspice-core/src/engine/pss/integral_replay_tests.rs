use super::*;

#[test]
fn integral_replay_matches_accepted_nested_and_branch_controlled_history() {
    for rate in [1.0e3, 1.0e9] {
        let engine = Engine::default();
        let netlist = Netlist::parse(&format!(
            "Integral replay\nvin in 0 sin(.7 .2 {rate})\nrin in 0 1k\n\
             bvol out 0 v=.2+{rate}*sdt(v(in)-v(out))\nrout out 0 1k\n\
             bnested nested 0 v={rate}*sdt(v(in)-v(nested)-{rate}*sdt(v(nested)))\nrnested nested 0 1k\n\
             bcur current 0 i=-{rate}*sdt(-i(vin)-v(current)/1k)\nrcurrent current 0 1k\n.end\n"
        )).unwrap();
        let original = engine.build_circuit(&netlist).unwrap();
        let initial = vec![0.3 / rate, 0.2 / rate, 0.1 / rate, 0.4e-3 / rate];
        let mut circuit = PssCircuit::new(original.clone()).unwrap();
        assert_eq!(circuit.state_dimension(), initial.len());
        circuit.set_state(&initial).unwrap();
        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let seed = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        let mut trace = PssStateTrace::default();
        let waveform = engine
            .pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed,
                PssTraversal {
                    tstop: rate.recip(),
                    max_step: rate.recip() / 64.0,
                    fixed_grid: true,
                    integration_method: Some(IntegrationMethod::Trapezoidal),
                    retain_waveform: true,
                },
                Some(&mut trace),
                &NoAbort,
            )
            .unwrap()
            .unwrap();
        let replay = |engine: &Engine, times: &[Value], abort: &dyn AbortSignal| {
            engine.hb_replay_integral_samples(
                &original,
                &initial,
                times,
                waveform
                    .node_names
                    .iter()
                    .zip(&waveform.voltages)
                    .map(|(name, values)| (name.as_str(), values.as_slice())),
                waveform
                    .branch_names
                    .iter()
                    .zip(&waveform.branch_currents)
                    .map(|(name, values)| (name.as_str(), values.as_slice())),
                abort,
            )
        };
        let integrals = replay(&engine, &waveform.time, &NoAbort).unwrap();
        assert_eq!(trace.times, waveform.time);
        assert_eq!(integrals.len(), initial.len());
        for (row, values) in integrals.iter().enumerate() {
            for (sample, value) in values.iter().enumerate() {
                assert_eq!(
                    value.to_bits(),
                    trace.states[sample][row].to_bits(),
                    "rate={rate}, coordinate={row}, sample={sample}"
                );
            }
        }
        assert!(matches!(
            replay(
                &engine,
                &waveform.time,
                &crate::abort_signal::CountingAbort::new(2)
            ),
            Err(SimulationError::Aborted)
        ));
        let mut bad_grid = waveform.time.clone();
        bad_grid[0] = rate.recip() / 256.0;
        assert!(replay(&engine, &bad_grid, &NoAbort).is_err());
        let mut limits = engine.config.clone();
        limits.resource_limits.max_result_values = 1;
        assert!(matches!(
            replay(&Engine::new(limits), &waveform.time, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
    }
}

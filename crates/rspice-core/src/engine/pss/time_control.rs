//! Time limits shared by stabilization and the certified shooting grid.

use super::*;

impl Engine {
    pub(super) fn pss_lte_abstol(&self, physical_abstol: Value) -> Value {
        let scale = (self.transient_lte_abstol() / self.voltage_abstol()).min(1.0);
        (physical_abstol * scale).max(Value::MIN_POSITIVE)
    }

    pub(super) fn pss_step_ceiling(&self) -> Value {
        [
            Some(self.config.max_timestep),
            self.config.transient_timeint_max_timestep,
        ]
        .into_iter()
        .flatten()
        .filter(|value| value.is_finite() && *value > 0.0)
        .fold(Value::INFINITY, Value::min)
    }

    pub(super) fn pss_minimum_grid_steps(&self, period: Value) -> Result<usize, SimulationError> {
        let count = (period / self.pss_step_ceiling()).ceil().max(1.0);
        if !count.is_finite() || count >= usize::MAX as Value {
            return Err(PssError::InvalidConfig(
                "PSS maximum timestep requires an unrepresentable integration grid".into(),
            )
            .into());
        }
        let count = count as usize;
        self.ensure_analysis_points(count)?;
        Ok(count)
    }

    pub(super) fn pss_waveform_within_step_ceiling(&self, waveform: &TransientResult) -> bool {
        // Subtraction of adjacent clocks can round slightly above the nominal
        // interval even when the uniform mesh was formed with ceil(period/dt).
        let ceiling = self.pss_step_ceiling() * (1.0 + 128.0 * Value::EPSILON);
        waveform
            .time
            .windows(2)
            .all(|pair| pair[1] - pair[0] <= ceiling)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pss_nox_selection_uses_the_selected_solver_update_quantity() {
        // The mathematical correction is smaller than an ULP of this large
        // bias. DampedNewton must retain it when applying DELTAXTOL; NOX
        // deliberately tests the rounded candidate difference instead.
        for nox in [false, true] {
            let netlist = Netlist::parse(&format!(
                "PSS Newton selection\nI1 0 out 1e16\nR1 out 0 3\n.options RSPICE_DIALECT=XYCE ABSTOL=10 VNTOL=10\n.options NONLIN-TRAN RELTOL=1e-30 ABSTOL=1e-30 DELTAXTOL=1 RHSTOL=10 MAXSTEP=3 NOX={}\n.end\n", usize::from(nox)
            )).unwrap();
            let engine = Engine::default().resolved_for_netlist(&netlist);
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let start = vec![3e16; circuit.matrix_size()];
            let coeff = CompanionCoefficients::backward_euler();
            let result = engine
                .pss_newton_trial(
                    &mut circuit,
                    &mut matrix,
                    PssCompanionStep {
                        coeff: &coeff,
                        t_next: 1e-6,
                        dt: 1e-6,
                        initialization: false,
                    },
                    &start,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(result.is_some(), nox, "NOX={nox}: {result:?}");
        }
    }

    #[test]
    fn pss_xyce_newton_tolerances_each_change_actual_acceptance() {
        let solve = |controls: &str| {
            let netlist = Netlist::parse(&format!(
                "PSS Newton tolerances\nB1 out 0 I={{V(out)^2-2}}\nR1 out 0 1e12\n.options RSPICE_DIALECT=XYCE RELTOL=0.01 VNTOL=1m ABSTOL=0.01\n.options NONLIN-TRAN MAXSTEP=20 ENFORCEDEVICECONV=0 {controls}\n.end\n"
            )).unwrap();
            let engine = Engine::default().resolved_for_netlist(&netlist);
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let start = vec![1.0; circuit.matrix_size()];
            let coeff = CompanionCoefficients::backward_euler();
            let polls = crate::abort_signal::CountingAbort::new(1000);
            let solution = engine
                .pss_newton_trial(
                    &mut circuit,
                    &mut matrix,
                    PssCompanionStep {
                        coeff: &coeff,
                        t_next: 1e-6,
                        dt: 1e-6,
                        initialization: false,
                    },
                    &start,
                    &polls,
                )
                .unwrap()
                .unwrap();
            (solution[0], polls.count())
        };
        for (loose, strict) in [
            (
                "RELTOL=1 ABSTOL=1e-14 DELTAXTOL=1 RHSTOL=1",
                "RELTOL=1e-10 ABSTOL=1e-14 DELTAXTOL=1 RHSTOL=1",
            ),
            (
                "RELTOL=1e-14 ABSTOL=1 DELTAXTOL=1 RHSTOL=1",
                "RELTOL=1e-14 ABSTOL=1e-10 DELTAXTOL=1 RHSTOL=1",
            ),
            (
                "RELTOL=1 ABSTOL=1 DELTAXTOL=1 RHSTOL=1",
                "RELTOL=1 ABSTOL=1 DELTAXTOL=1e-12 RHSTOL=1",
            ),
            (
                "RELTOL=1 ABSTOL=1 DELTAXTOL=1 RHSTOL=1",
                "RELTOL=1 ABSTOL=1 DELTAXTOL=1 RHSTOL=1e-12",
            ),
        ] {
            let loose_result = solve(loose);
            let strict_result = solve(strict);
            assert!(
                strict_result.1 > loose_result.1,
                "{strict}: {loose_result:?} -> {strict_result:?}"
            );
            assert!((strict_result.0 - 2.0_f64.sqrt()).abs() < 1e-10);
            assert!(
                (strict_result.0 - 2.0_f64.sqrt()).abs() < (loose_result.0 - 2.0_f64.sqrt()).abs()
            );
        }
    }

    #[test]
    fn pss_xyce_rhstol_preserves_physical_prescribed_current_windings() {
        let netlist = Netlist::parse(
            "PSS winding residual\nI1 0 a SIN(1000 1m 1meg 0 0 37)\nL1 a b 100u\nR1 b 0 0.0001\n.options RSPICE_DIALECT=XYCE\n.options NONLIN-TRAN RHSTOL=1n\n.end\n"
        ).unwrap();
        let engine = Engine::default().resolved_for_netlist(&netlist);
        let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
        circuit.set_state(&[]).unwrap();
        let seed = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        let mut matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        let result = engine
            .pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed,
                PssTraversal {
                    tstop: 10e-9,
                    max_step: 2e-9,
                    fixed_grid: true,
                    integration_method: Some(IntegrationMethod::Trapezoidal),
                    retain_waveform: true,
                },
                None,
                &NoAbort,
            )
            .unwrap()
            .unwrap();
        let winding = result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("L1"))
            .unwrap();
        for (&time, &current) in result.time.iter().zip(&result.branch_currents[winding]) {
            let phase = std::f64::consts::TAU * 1e6 * time + 37.0_f64.to_radians();
            assert!((current - (1000.0 + 1e-3 * phase.sin())).abs() < 1e-10);
        }
    }

    #[test]
    fn pss_timepoint_budget_uses_transient_options_in_each_dialect() {
        // A staircase control requires successive Newton updates; its local
        // derivative is zero. The DC limit deliberately permits every case,
        // so borrowing that limit for a timepoint would hide both failures.
        for (dialect, options, target, converges) in [
            (
                crate::SpiceDialect::Ngspice,
                ".options ITL4=100",
                150,
                false,
            ),
            (crate::SpiceDialect::Ngspice, ".options ITL4=200", 150, true),
            (
                crate::SpiceDialect::Xyce,
                ".options NONLIN-TRAN MAXSTEP=2",
                5,
                false,
            ),
            (
                crate::SpiceDialect::Xyce,
                ".options NONLIN-TRAN MAXSTEP=10",
                5,
                true,
            ),
        ] {
            let netlist = Netlist::parse(&format!(
                "PSS timestep budget\nB1 out 0 V={{min(floor(V(out))+1,{target})}}\nR1 out 0 1k\n.options ITL1=500\n{options}\n.end\n"
            )).unwrap();
            let engine = Engine::new(crate::SimulationConfig {
                spice_dialect: dialect,
                ..Default::default()
            })
            .resolved_for_netlist(&netlist);
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let start = vec![0.0; circuit.matrix_size()];
            let coeff = CompanionCoefficients::backward_euler();
            let result = engine
                .pss_newton_trial(
                    &mut circuit,
                    &mut matrix,
                    PssCompanionStep {
                        coeff: &coeff,
                        t_next: 1e-6,
                        dt: 1e-6,
                        initialization: false,
                    },
                    &start,
                    &NoAbort,
                )
                .unwrap();
            assert_eq!(result.is_some(), converges, "{dialect:?}: {options}");
            if let Some(solution) = result {
                assert!((solution[0] - target as Value).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn pss_timeint_tolerances_control_stabilization_and_orbit_accuracy() {
        let run = |reltol: Value, abstol: Value| {
            let netlist = Netlist::parse(&format!(
                "PSS LTE control\nV1 in 0 SIN(0 1 1k)\nR1 in out 1k\nC1 out 0 159.15494309189535n\n.options TIMEINT RELTOL={reltol:e} ABSTOL={abstol:e}\n.end\n"
            )).unwrap();
            let engine = Engine::default().resolved_for_netlist(&netlist);
            let mut config = PssConfig::new(1e3)
                .with_harmonics(4)
                .with_points_per_period(16)
                .with_tolerance(1e-9);
            config.tstab_periods = 2;
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let seed = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            engine.pss_initialize_reactive_state(&mut circuit, &seed);
            let (stabilized, _) = engine
                .pss_run_stabilization(&mut circuit, &mut matrix, &seed, &config, &NoAbort)
                .unwrap();
            let orbit = engine.run_pss(&netlist, config).unwrap();
            let output = orbit
                .result
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("out"))
                .unwrap();
            let error = orbit
                .result
                .time
                .iter()
                .zip(&orbit.result.waveforms[output].values)
                .map(|(&time, &voltage)| {
                    let phase = std::f64::consts::TAU * 1e3 * time;
                    (voltage - 0.5 * (phase.sin() - phase.cos())).abs()
                })
                .fold(0.0, Value::max);
            (stabilized.time.len(), orbit.result.time.len(), error)
        };
        let loose = run(1e-3, 1e-6);
        let strict = run(1e-6, 1e-9);
        assert!(strict.0 > loose.0, "stabilization: {loose:?} -> {strict:?}");
        assert!(strict.1 > loose.1, "shooting grid: {loose:?} -> {strict:?}");
        assert!(
            strict.2 < loose.2 * 0.1,
            "analytic RC error: {loose:?} -> {strict:?}"
        );
    }

    #[test]
    fn pss_step_ceiling_reaches_stabilization_and_the_retained_orbit() {
        for (options, ceiling) in [
            (".options MAXTIMESTEP=7u", 7e-6),
            (".options MAXTIMESTEP=20u\n.options TIMEINT DELMAX=7u", 7e-6),
            (".options MAXTIMESTEP=5u\n.options TIMEINT DELMAX=7u", 5e-6),
        ] {
            let netlist = Netlist::parse(&format!(
                "PSS time limit\nV1 in 0 SIN(0 1 1k)\nR1 in 0 1k\n{options}\n.end\n"
            ))
            .unwrap();
            let engine = Engine::default().resolved_for_netlist(&netlist);
            let mut config = PssConfig::new(1e3)
                .with_harmonics(4)
                .with_points_per_period(16);
            config.tstab_periods = 1;
            let orbit = engine.run_pss(&netlist, config.clone()).unwrap();
            assert!(orbit.result.time.len() > config.points_per_period + 1);
            for pair in orbit.result.time.windows(2) {
                assert!(pair[1] - pair[0] <= ceiling * (1.0 + 1e-12));
            }
            let mut circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let seed = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            engine.pss_initialize_reactive_state(&mut circuit, &seed);
            let (waveform, _) = engine
                .pss_run_stabilization(&mut circuit, &mut matrix, &seed, &config, &NoAbort)
                .unwrap();
            assert_eq!(waveform.time.last(), Some(&config.effective_tstab()));
            for pair in waveform.time.windows(2) {
                assert!(pair[1] - pair[0] <= ceiling * (1.0 + 1e-12));
            }
        }
    }

    #[test]
    fn pss_step_ceiling_rejects_unbounded_grid_allocation() {
        let mut config = crate::SimulationConfig::default();
        config.transient_timeint_max_timestep = Some(1e-300);
        let engine = Engine::new(config);
        assert!(engine.pss_minimum_grid_steps(1.0).is_err());
    }
}

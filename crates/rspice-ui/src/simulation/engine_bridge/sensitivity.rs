//! Sensitivity analysis over the engine bridge.

use std::collections::HashMap;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::sensitivity::{
    AcSensitivityOutput, SensitivityUnavailability, SensitivityValue,
};

use super::{EngineBridge, ensure_not_aborted};
use crate::output_spec::{
    OutputSpec, ac_output_value, collect_sensitivity_parameters, dc_output_value,
    parse_output_spec, resolve_sensitivity_ac_frequency, validate_sensitivity_output_spec,
};
use crate::simulation::config::SensitivityConfig;
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run sensitivity analysis.
    pub(super) fn run_sensitivity(
        &self,
        netlist: &rspice_core::Netlist,
        config: &SensitivityConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let engine = self.engine_for_netlist(netlist);
        let ac_frequency = resolve_sensitivity_ac_frequency(config.ac_mode, config.frequency);
        ensure_not_aborted(abort)?;
        let ac_frequency = ac_frequency.map_err(SimulationError::InvalidConfig)?;

        let parameters = collect_sensitivity_parameters(netlist);
        ensure_not_aborted(abort)?;
        if parameters.is_empty() {
            return Err(SimulationError::InvalidConfig(
                "Sensitivity analysis found no eligible design parameters".to_owned(),
            ));
        }
        // Nominal probes and adaptive trials all consume one study budget.
        // The core driver checks every additional trial as it refines a result.
        let mut runs = 1 + usize::from(ac_frequency.is_some());
        let requested_runs = parameters.len().saturating_add(runs);
        engine
            .ensure_batch_runs(requested_runs)
            .map_err(|error| self.translate_error(error))?;

        let dc_result = engine
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;
        let output_spec = parse_output_spec(&config.output_var, &dc_result.node_names, &circuit);
        ensure_not_aborted(abort)?;
        let output_spec = output_spec.ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "Sensitivity output '{}' could not be resolved to a node or branch",
                config.output_var
            ))
        })?;
        let output_validation = validate_sensitivity_output_spec(&output_spec);
        ensure_not_aborted(abort)?;
        output_validation.map_err(SimulationError::InvalidConfig)?;

        let nominal_value = if let Some(freq) = ac_frequency {
            self.run_ac_output_at_frequency_with_abort(&engine, netlist, &output_spec, freq, abort)?
                .norm()
        } else {
            let output = dc_output_value(&dc_result, &output_spec);
            ensure_not_aborted(abort)?;
            output.map_err(SimulationError::InvalidConfig)?
        };
        if !nominal_value.is_finite() {
            return Err(SimulationError::SolverError(
                "Sensitivity nominal output is non-finite".to_owned(),
            ));
        }
        ensure_not_aborted(abort)?;

        let probe = match &output_spec {
            OutputSpec::Voltage(voltage) => AcSensitivityOutput::Voltage {
                positive: voltage.pos,
                negative: voltage.neg,
            },
            OutputSpec::BranchCurrent { branch_name, .. } => {
                AcSensitivityOutput::BranchCurrent(branch_name.clone())
            }
        };

        let mut sensitivities = HashMap::new();
        let mut normalized = HashMap::new();

        for (param_name, param_value) in parameters {
            ensure_not_aborted(abort)?;
            if !param_value.is_finite() {
                return Err(SimulationError::InvalidConfig(format!(
                    "Sensitivity parameter '{param_name}' is non-finite"
                )));
            }

            let sensitivity = if let Some(freq) = ac_frequency {
                let values = engine
                    .run_output_sensitivity_ac_with_abort(
                        netlist,
                        probe.clone(),
                        &param_name,
                        param_value,
                        &[freq],
                        None,
                        &mut runs,
                        abort,
                    )
                    .map_err(|error| self.translate_error(error))?;
                *values.first().ok_or_else(|| {
                    SimulationError::SolverError(format!(
                        "Sensitivity parameter '{param_name}' produced no AC derivative"
                    ))
                })?
            } else {
                SensitivityValue::Available(
                    engine
                        .run_output_sensitivity_with_abort(
                            netlist,
                            probe.clone(),
                            &param_name,
                            param_value,
                            None,
                            &mut runs,
                            abort,
                        )
                        .map_err(|error| self.translate_error(error))?,
                )
            };

            ensure_not_aborted(abort)?;
            if sensitivity.value().is_some_and(|value| !value.is_finite()) {
                return Err(SimulationError::SolverError(format!(
                    "Sensitivity parameter '{param_name}' produced a non-finite derivative"
                )));
            }
            sensitivities.insert(param_name.clone(), sensitivity);
            let value = if nominal_value == 0.0 {
                SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput)
            } else {
                match sensitivity {
                    SensitivityValue::Available(raw) => {
                        SensitivityValue::normalized(param_value, raw, nominal_value)
                    }
                    unavailable => unavailable,
                }
            };
            normalized.insert(param_name, value);
        }

        ensure_not_aborted(abort)?;
        Ok(SimulationResult::Sensitivity {
            output: config.output_var.trim().to_owned(),
            ac_mode: config.ac_mode,
            frequency_hz: ac_frequency,
            sensitivities,
            normalized,
        })
    }

    fn run_ac_output_at_frequency_with_abort(
        &self,
        engine: &rspice_core::Engine,
        netlist: &rspice_core::Netlist,
        output_spec: &OutputSpec,
        frequency: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Complex64, SimulationError> {
        ensure_not_aborted(abort)?;
        let ac_results = engine
            .run_ac_with_abort(netlist, &[frequency], abort)
            .map_err(|error| self.translate_error(error))?;
        ensure_not_aborted(abort)?;
        let point = ac_results.first().ok_or_else(|| {
            SimulationError::InvalidConfig(format!(
                "AC analysis produced no data at {frequency} Hz"
            ))
        })?;
        let output = ac_output_value(point, output_spec);
        ensure_not_aborted(abort)?;
        output.map_err(SimulationError::InvalidConfig)
    }
}

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    use super::*;

    fn parameter_result(source: &str, output: &str, ac_mode: bool) -> SimulationResult {
        let netlist = rspice_core::Netlist::parse(source).unwrap();
        let original = netlist.clone();
        let result = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: output.to_owned(),
                    ac_mode,
                    frequency: ac_mode.then_some(1.0),
                },
                &NoAbort,
            )
            .unwrap();
        assert_eq!(netlist.source_text, original.source_text);
        assert_eq!(netlist.params.all_params(), original.params.all_params());
        result
    }

    #[test]
    fn sensitivity_replays_authored_ac_parameters_and_projects_the_phasor() {
        for gain in [0.5_f64, -0.5, 5e-13, -5e-13] {
            let source = format!(
                "AC parameter sensitivity\n.param gain={gain}\nV1 in 0 AC 1 60\nE1 out 0 in 0 {{gain}}\n.end\n"
            );
            let SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } = parameter_result(&source, "V(out)", true)
            else {
                panic!("expected sensitivity data")
            };
            assert!(
                (sensitivities["GAIN"].value().unwrap() - gain.signum()).abs() < 1e-9,
                "gain={gain}: {sensitivities:?}"
            );
            assert!((normalized["GAIN"].value().unwrap() - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn sensitivity_replays_authored_dc_branch_parameters() {
        let SimulationResult::Sensitivity {
            sensitivities,
            normalized,
            ..
        } = parameter_result(
            "DC branch sensitivity\n.param drive=2\nV1 out 0 {drive}\nR1 out 0 2\n.end\n",
            "I(V1)",
            false,
        )
        else {
            panic!("expected sensitivity data")
        };
        assert!(
            (sensitivities["DRIVE"].value().unwrap() + 0.5).abs() < 1e-9,
            "{sensitivities:?}"
        );
        assert!((normalized["DRIVE"].value().unwrap() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn sensitivity_normalizes_finite_outputs_below_the_former_epsilon() {
        for drive in [1e-20, 1e-300] {
            let source =
                format!("Tiny DC sensitivity\n.param drive={drive}\nV1 out 0 {{drive}}\n.end\n");
            let SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } = parameter_result(&source, "V(out)", false)
            else {
                panic!("expected sensitivity data")
            };
            assert!((sensitivities["DRIVE"].value().unwrap() - 1.0).abs() < 1e-9);
            assert!(
                normalized.get("DRIVE").is_some_and(|value| value
                    .value()
                    .is_some_and(|value| (value - 1.0).abs() < 1e-9)),
                "drive={drive}: {normalized:?}"
            );
        }
    }

    #[test]
    fn sensitivity_projects_ac_at_the_nominal_output_near_a_null() {
        for gain in [0.9999_f64, 1.0001] {
            let source = format!(
                "AC null crossing\n.param gain={gain}\nV1 in 0 AC 1 60\nE1 out 0 in 0 {{gain-1}}\n.end\n"
            );
            let SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } = parameter_result(&source, "V(out)", true)
            else {
                panic!("expected sensitivity data")
            };
            let expected = (gain - 1.0).signum();
            assert!((sensitivities["GAIN"].value().unwrap() - expected).abs() < 1e-9);
            assert!(
                (normalized["GAIN"].value().unwrap() / (gain / (gain - 1.0)) - 1.0).abs() < 1e-9
            );
        }
    }

    #[test]
    fn sensitivity_normalization_avoids_intermediate_overflow() {
        let SimulationResult::Sensitivity {
            sensitivities,
            normalized,
            ..
        } = parameter_result(
            "Wide normalization\n.param drive=1e300\nV1 out 0 {1e-300*(drive-1e300)+1e-200}\n.end\n",
            "V(out)",
            false,
        )
        else {
            panic!("expected sensitivity data")
        };
        assert!(
            (sensitivities["DRIVE"].value().unwrap() / 1e-300 - 1.0).abs() < 1e-9,
            "{sensitivities:?}"
        );
        assert!(
            (normalized["DRIVE"].value().unwrap() / 1e200 - 1.0).abs() < 1e-9,
            "{normalized:?}"
        );
    }

    #[test]
    fn sensitivity_resolves_differential_voltage_and_ac_branch_probes() {
        for (source, output, ac, expected) in [
            (
                "DC differential\n.param drive=2\nV1 a 0 {drive}\nV2 b 0 {drive/4}\n.end\n",
                "V(a,b)",
                false,
                0.75,
            ),
            (
                "AC differential\n.param drive=2\nV1 in 0 AC 1 60\nE1 a 0 in 0 {drive}\nE2 b 0 in 0 {drive/4}\n.end\n",
                "V(a,b)",
                true,
                0.75,
            ),
            (
                "AC branch\n.param drive=2\nV1 out 0 AC {drive} 60\nR1 out 0 2\n.end\n",
                "I(V1)",
                true,
                0.5,
            ),
        ] {
            let SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } = parameter_result(source, output, ac)
            else {
                panic!("expected sensitivity data")
            };
            assert!(
                (sensitivities["DRIVE"].value().unwrap() - expected).abs() < 1e-9,
                "{source}: {sensitivities:?}"
            );
            assert!(
                (normalized["DRIVE"].value().unwrap() - 1.0).abs() < 1e-9,
                "{source}: {normalized:?}"
            );
        }
    }

    #[test]
    fn sensitivity_uses_a_relative_step_for_small_physical_parameters() {
        let netlist = rspice_core::Netlist::parse(
            "Small capacitor\n.param cap=1e-15\nV1 in 0 AC 1\nR1 in out 1e6\nC1 out 0 {cap}\n.end\n"
        ).unwrap();
        let SimulationResult::Sensitivity {
            sensitivities,
            normalized,
            ..
        } = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: "V(out)".to_owned(),
                    ac_mode: true,
                    frequency: Some(1.0 / (std::f64::consts::TAU * 1e-9)),
                },
                &NoAbort,
            )
            .unwrap()
        else {
            panic!("expected sensitivity data")
        };
        let expected = -1.0 / (2.0_f64.sqrt() * 2e-15);
        assert!(
            (sensitivities["CAP"].value().unwrap() / expected - 1.0).abs() < 2e-6,
            "{sensitivities:?}"
        );
        assert!(
            (normalized["CAP"].value().unwrap() + 0.5).abs() < 2e-6,
            "{normalized:?}"
        );
    }

    #[test]
    fn sensitivity_retains_valid_rows_and_explicit_unavailable_quantities() {
        let netlist = rspice_core::Netlist::parse(
            "Zero output\n.param gain=0 scale=1\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {gain*scale}\n.end\n"
        ).unwrap();
        for ac_mode in [false, true] {
            let SimulationResult::Sensitivity {
                sensitivities,
                normalized,
                ..
            } = EngineBridge::new()
                .run_sensitivity(
                    &netlist,
                    &SensitivityConfig {
                        ac_mode,
                        frequency: ac_mode.then_some(1.0),
                        ..SensitivityConfig::default()
                    },
                    &NoAbort,
                )
                .unwrap()
            else {
                panic!("expected retained sensitivity data")
            };
            assert_eq!(sensitivities.len(), 2);
            assert_eq!(normalized.len(), 2);
            assert_eq!(sensitivities["SCALE"], SensitivityValue::Available(0.0));
            if ac_mode {
                assert_eq!(
                    sensitivities["GAIN"],
                    SensitivityValue::unavailable(
                        SensitivityUnavailability::NondifferentiableMagnitude
                    )
                );
            } else {
                assert!((sensitivities["GAIN"].value().unwrap() - 1.0).abs() < 1e-9);
            }
            assert!(normalized.values().all(|value| *value
                == SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput)));
        }
    }

    #[test]
    fn sensitivity_checks_the_whole_study_budget() {
        for (source, parameter_count) in [
            (
                "Study budget\n.param drive=2\nV1 out 0 DC {drive} AC {drive}\nR1 out 0 2\n.end\n",
                1,
            ),
            (
                "Study budget\n.param drive=2 load=2\nV1 out 0 DC {drive} AC {drive}\nR1 out 0 {load}\n.end\n",
                2,
            ),
        ] {
            let netlist = rspice_core::Netlist::parse(source).unwrap();
            for ac_mode in [false, true] {
                let required = 1 + usize::from(ac_mode) + 5 * parameter_count;
                for limit in [required - 1, required] {
                    let mut engine_config = rspice_core::SimulationConfig::default();
                    engine_config.resource_limits.max_batch_runs = limit;
                    let result = EngineBridge::try_with_config(engine_config)
                        .unwrap()
                        .run_sensitivity(
                            &netlist,
                            &SensitivityConfig {
                                output_var: "I(V1)".to_owned(),
                                ac_mode,
                                frequency: ac_mode.then_some(1.0),
                            },
                            &NoAbort,
                        );
                    if limit < required {
                        assert!(
                            matches!(result, Err(SimulationError::ResourceLimit { .. })),
                            "{result:?}"
                        );
                    } else {
                        assert!(result.is_ok(), "{result:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn sensitivity_preserves_typed_cancellation_during_parameter_replay_and_solves() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct CancelAfter {
            polls: AtomicUsize,
            limit: usize,
        }
        impl AbortSignal for CancelAfter {
            fn is_aborted(&self) -> bool {
                self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
            }
        }
        let netlist = rspice_core::Netlist::parse(
            "Cancelled study\n.param drive=2\nV1 out 0 DC {drive} AC {drive}\n.end\n",
        )
        .unwrap();
        let original = netlist.source_text.clone();
        for ac_mode in [false, true] {
            let config = SensitivityConfig {
                ac_mode,
                frequency: ac_mode.then_some(1.0),
                ..SensitivityConfig::default()
            };
            let baseline = CancelAfter {
                polls: AtomicUsize::new(0),
                limit: usize::MAX,
            };
            EngineBridge::new()
                .run_sensitivity(&netlist, &config, &baseline)
                .unwrap();
            let total = baseline.polls.load(Ordering::Relaxed);
            for limit in [total / 3, total * 2 / 3] {
                let abort = CancelAfter {
                    polls: AtomicUsize::new(0),
                    limit,
                };
                assert!(matches!(
                    EngineBridge::new().run_sensitivity(&netlist, &config, &abort),
                    Err(SimulationError::Aborted)
                ));
                assert_eq!(netlist.source_text, original);
                assert_eq!(netlist.params.get("DRIVE"), Some(2.0));
            }
        }
    }

    #[test]
    fn sensitivity_preserves_a_typed_abort_before_solving() {
        let netlist = rspice_core::Netlist::parse(
            "Aborted sensitivity\n.param drive=2\nV1 out 0 {drive}\n.end\n",
        )
        .unwrap();
        assert!(matches!(
            EngineBridge::new().run_sensitivity(
                &netlist,
                &SensitivityConfig::default(),
                &ImmediateAbort,
            ),
            Err(SimulationError::Aborted)
        ));
    }
}

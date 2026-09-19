//! Sensitivity analysis over the engine bridge.
//!
//! The Studio runs the same two entries every other surface runs —
//! `Engine::run_sensitivity_dc_complete_with_abort` and
//! `…_ac_complete_with_abort` — with the card's own filter list and the
//! card's own frequency grid. One deck therefore has one answer here, in
//! `rspice run`, in Python and in the bindings, under one set of names.
//!
//! What this file does not do is compute anything. It resolves the probe,
//! builds the grid with the engine's own sweep function, hands both to the
//! engine, and copies the answer into retained evidence. The nominal output
//! every normalized column divides by is the engine's, not a second solve of
//! this bridge's own.

use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::sensitivity::{AcSensitivityOutput, SensitivityValue};

use super::{EngineBridge, ensure_not_aborted};
use crate::output_spec::{OutputSpec, parse_output_spec, validate_sensitivity_output_spec};
use crate::simulation::config::SensitivityConfig;
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::SimulationError;
use crate::state::{
    ComplexResultValue, SensitivityBasisEvidence, SensitivityStudyEvidence, SensitivityStudyRow,
};

impl EngineBridge {
    /// Run sensitivity analysis.
    pub(super) fn run_sensitivity(
        &self,
        netlist: &rspice_core::Netlist,
        config: &SensitivityConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        config
            .validate()
            .map_err(|errors| SimulationError::InvalidConfig(errors.join("; ")))?;
        let engine = self.engine_for_netlist(netlist);
        let frequencies = self.sensitivity_frequencies(config, abort)?;
        ensure_not_aborted(abort)?;

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
        validate_sensitivity_output_spec(&output_spec).map_err(SimulationError::InvalidConfig)?;
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
        // The canonical filter is one space-separated line; the card grammar
        // splits it on whitespace and so does this.
        let filters: Vec<String> = config
            .filter
            .split_whitespace()
            .map(str::to_owned)
            .collect();

        let evidence = match frequencies {
            Some(frequencies) => {
                let result = engine
                    .run_sensitivity_ac_complete_with_abort(
                        netlist,
                        probe,
                        &frequencies,
                        &filters,
                        abort,
                    )
                    .map_err(|error| self.translate_error(error))?;
                ensure_not_aborted(abort)?;
                let mut rows: Vec<SensitivityStudyRow> = result
                    .sensitivities
                    .iter()
                    .map(|trace| SensitivityStudyRow {
                        parameter: trace.vector_name.clone(),
                        nominal_value: trace.nominal_value,
                        raw: trace.magnitude.clone(),
                        normalized: trace
                            .normalized
                            .iter()
                            .map(|value| value.map(|value| value.re))
                            .collect(),
                        phase: trace.phase.clone(),
                    })
                    .collect();
                rows.sort_by(|left, right| left.parameter.cmp(&right.parameter));
                SensitivityStudyEvidence {
                    output: config.output_var.trim().to_owned(),
                    filter: config.filter.trim().to_owned(),
                    basis: SensitivityBasisEvidence::Ac {
                        frequencies_hz: result.frequencies.clone(),
                        output: result
                            .output_values
                            .iter()
                            .map(|value| ComplexResultValue {
                                real: value.re,
                                imaginary: value.im,
                            })
                            .collect(),
                    },
                    rows,
                }
            }
            None => {
                let result = engine
                    .run_sensitivity_dc_complete_with_abort(netlist, probe, &filters, abort)
                    .map_err(|error| self.translate_error(error))?;
                ensure_not_aborted(abort)?;
                let mut rows: Vec<SensitivityStudyRow> = result
                    .sensitivities
                    .iter()
                    .map(|entry| SensitivityStudyRow {
                        parameter: entry.vector_name.clone(),
                        nominal_value: entry.nominal_value,
                        raw: vec![SensitivityValue::Available(entry.absolute)],
                        normalized: vec![entry.normalized],
                        phase: Vec::new(),
                    })
                    .collect();
                rows.sort_by(|left, right| left.parameter.cmp(&right.parameter));
                SensitivityStudyEvidence {
                    output: config.output_var.trim().to_owned(),
                    filter: config.filter.trim().to_owned(),
                    basis: SensitivityBasisEvidence::Dc {
                        output: result.output_value,
                    },
                    rows,
                }
            }
        };

        ensure_not_aborted(abort)?;
        // Refused here rather than at the sheet: a study that cannot be
        // validated is not evidence, and the run is what knows why.
        evidence.validate().map_err(SimulationError::SolverError)?;
        Ok(SimulationResult::SensitivityStudy {
            evidence: std::sync::Arc::new(evidence),
        })
    }

    /// The grid the card asks for, built by the engine's own sweep function.
    ///
    /// `None` is a DC study. A single frequency goes through the same
    /// function as a sweep, as `DEC 1 f f`, so the Studio and the command
    /// line cannot disagree about what one point means.
    fn sensitivity_frequencies(
        &self,
        config: &SensitivityConfig,
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        if !config.ac_mode {
            return Ok(None);
        }
        let Some(start) = config.frequency else {
            return Err(SimulationError::InvalidConfig(
                "AC sensitivity requires a start frequency".to_owned(),
            ));
        };
        let (variation, points, stop) = match config.sweep {
            Some(sweep) => (
                sweep.variation.freq_variation(),
                sweep.points as usize,
                sweep.stop_frequency,
            ),
            None => (rspice_core::netlist::FreqVariation::Dec, 1, start),
        };
        rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
            variation, points, start, stop, abort,
        )
        .map(Some)
        // The engine's own sentence, verbatim: the grid this refuses is the
        // grid `rspice run` would refuse on the same card, and a Studio
        // paraphrase would make the two disagree about why.
        .map_err(|error| match error {
            rspice_core::analysis::FrequencyGridError::Aborted => SimulationError::Aborted,
            error => SimulationError::InvalidConfig(error.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    use super::*;
    use crate::simulation::config::{AcSweepType, DESIGN_PARAMETERS_FILTER, SensitivitySweep};

    fn study(source: &str, output: &str, ac_mode: bool) -> SensitivityStudyEvidence {
        let netlist = rspice_core::Netlist::parse(source).unwrap();
        let original = netlist.clone();
        let result = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: output.to_owned(),
                    ac_mode,
                    frequency: ac_mode.then_some(1.0),
                    filter: DESIGN_PARAMETERS_FILTER.to_owned(),
                    sweep: None,
                },
                &NoAbort,
            )
            .unwrap();
        assert_eq!(netlist.source_text, original.source_text);
        assert_eq!(netlist.params.all_params(), original.params.all_params());
        let SimulationResult::SensitivityStudy { evidence } = result else {
            panic!("expected sensitivity study evidence")
        };
        std::sync::Arc::unwrap_or_clone(evidence)
    }

    fn row<'a>(evidence: &'a SensitivityStudyEvidence, name: &str) -> &'a SensitivityStudyRow {
        evidence
            .rows
            .iter()
            .find(|row| row.parameter == name)
            .unwrap_or_else(|| panic!("no row named {name} in {:?}", evidence.rows))
    }

    fn value(evidence: &SensitivityStudyEvidence, name: &str, point: usize) -> f64 {
        row(evidence, name).raw[point].value().unwrap()
    }

    fn normalized(evidence: &SensitivityStudyEvidence, name: &str, point: usize) -> f64 {
        row(evidence, name).normalized[point].value().unwrap()
    }

    /// The expectations of the tests this route replaced, unchanged: the
    /// design parameters the Studio used to loop are the ones `PARAM:*`
    /// selects, under the engine's names, to the same numbers.
    #[test]
    fn sensitivity_replays_authored_ac_parameters_and_projects_the_phasor() {
        for gain in [0.5_f64, -0.5, 5e-13, -5e-13] {
            let source = format!(
                "AC parameter sensitivity\n.param gain={gain}\nV1 in 0 AC 1 60\nE1 out 0 in 0 {{gain}}\n.end\n"
            );
            let evidence = study(&source, "V(out)", true);
            assert!(
                (value(&evidence, "PARAM:GAIN", 0) - gain.signum()).abs() < 1e-9,
                "gain={gain}: {:?}",
                evidence.rows
            );
            assert!((normalized(&evidence, "PARAM:GAIN", 0) - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn sensitivity_replays_authored_dc_branch_parameters() {
        let evidence = study(
            "DC branch sensitivity\n.param drive=2\nV1 out 0 {drive}\nR1 out 0 2\n.end\n",
            "I(V1)",
            false,
        );
        assert!(
            (value(&evidence, "PARAM:DRIVE", 0) + 0.5).abs() < 1e-9,
            "{:?}",
            evidence.rows
        );
        assert!((normalized(&evidence, "PARAM:DRIVE", 0) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn sensitivity_normalizes_finite_outputs_below_the_former_epsilon() {
        for drive in [1e-20, 1e-300] {
            let source =
                format!("Tiny DC sensitivity\n.param drive={drive}\nV1 out 0 {{drive}}\n.end\n");
            let evidence = study(&source, "V(out)", false);
            assert!((value(&evidence, "PARAM:DRIVE", 0) - 1.0).abs() < 1e-9);
            assert!((normalized(&evidence, "PARAM:DRIVE", 0) - 1.0).abs() < 1e-9);
        }
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
            let evidence = study(source, output, ac);
            assert!(
                (value(&evidence, "PARAM:DRIVE", 0) - expected).abs() < 1e-9,
                "{source}: {:?}",
                evidence.rows
            );
            assert!(
                (normalized(&evidence, "PARAM:DRIVE", 0) - 1.0).abs() < 1e-9,
                "{source}: {:?}",
                evidence.rows
            );
        }
    }

    #[test]
    fn sensitivity_uses_a_relative_step_for_small_physical_parameters() {
        let netlist = rspice_core::Netlist::parse(
            "Small capacitor\n.param cap=1e-15\nV1 in 0 AC 1\nR1 in out 1e6\nC1 out 0 {cap}\n.end\n"
        ).unwrap();
        let SimulationResult::SensitivityStudy { evidence } = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: "V(out)".to_owned(),
                    ac_mode: true,
                    frequency: Some(1.0 / (std::f64::consts::TAU * 1e-9)),
                    filter: DESIGN_PARAMETERS_FILTER.to_owned(),
                    sweep: None,
                },
                &NoAbort,
            )
            .unwrap()
        else {
            panic!("expected sensitivity study evidence")
        };
        let expected = -1.0 / (2.0_f64.sqrt() * 2e-15);
        assert!(
            (value(&evidence, "PARAM:CAP", 0) / expected - 1.0).abs() < 2e-6,
            "{:?}",
            evidence.rows
        );
        assert!((normalized(&evidence, "PARAM:CAP", 0) + 0.5).abs() < 2e-6);
    }

    #[test]
    fn sensitivity_retains_valid_rows_and_explicit_unavailable_quantities() {
        use rspice_core::analysis::sensitivity::SensitivityUnavailability;
        let netlist = rspice_core::Netlist::parse(
            "Zero output\n.param gain=0 scale=1\nV1 in 0 DC 1 AC 1\nE1 out 0 in 0 {gain*scale}\n.end\n"
        ).unwrap();
        for ac_mode in [false, true] {
            let SimulationResult::SensitivityStudy { evidence } = EngineBridge::new()
                .run_sensitivity(
                    &netlist,
                    &SensitivityConfig {
                        ac_mode,
                        frequency: ac_mode.then_some(1.0),
                        filter: DESIGN_PARAMETERS_FILTER.to_owned(),
                        ..SensitivityConfig::default()
                    },
                    &NoAbort,
                )
                .unwrap()
            else {
                panic!("expected retained sensitivity study evidence")
            };
            assert_eq!(evidence.rows.len(), 2);
            assert_eq!(
                row(&evidence, "PARAM:SCALE").raw[0],
                SensitivityValue::Available(0.0)
            );
            if ac_mode {
                assert_eq!(
                    row(&evidence, "PARAM:GAIN").raw[0],
                    SensitivityValue::unavailable(
                        SensitivityUnavailability::NondifferentiableMagnitude
                    )
                );
            } else {
                assert!((value(&evidence, "PARAM:GAIN", 0) - 1.0).abs() < 1e-9);
            }
            // The engine's zero-output rule reaches every normalized column,
            // and the evidence validator insists it agrees with the nominal.
            assert!(evidence.rows.iter().all(
                |row| row.normalized[0].reason() == Some(SensitivityUnavailability::ZeroOutput)
            ));
        }
    }

    /// The Studio's filter is the engine's filter: an empty one selects the
    /// device and model variables and no design parameter, exactly as a bare
    /// `.sens V(out)` read by `rspice run` does.
    #[test]
    fn an_empty_filter_selects_the_engines_own_default_set() {
        let netlist = rspice_core::Netlist::parse(
            "Divider\n.param rload=2\nV1 out 0 4\nR1 out 0 {rload}\n.end\n",
        )
        .unwrap();
        for (filter, expect_design, expect_device) in [
            ("", false, true),
            ("PARAM:*", true, false),
            ("R1 PARAM:*", true, true),
        ] {
            let SimulationResult::SensitivityStudy { evidence } = EngineBridge::new()
                .run_sensitivity(
                    &netlist,
                    &SensitivityConfig {
                        output_var: "I(V1)".to_owned(),
                        filter: filter.to_owned(),
                        ..SensitivityConfig::default()
                    },
                    &NoAbort,
                )
                .unwrap()
            else {
                panic!("expected sensitivity study evidence")
            };
            assert_eq!(evidence.filter, filter);
            let names: Vec<&str> = evidence
                .rows
                .iter()
                .map(|row| row.parameter.as_str())
                .collect();
            assert_eq!(
                names.contains(&"PARAM:RLOAD"),
                expect_design,
                "filter {filter:?}: {names:?}"
            );
            assert_eq!(
                names.iter().any(|name| *name == "R1"),
                expect_device,
                "filter {filter:?}: {names:?}"
            );
            // Whatever it selected, the rows arrive sorted and unique.
            let mut sorted = names.clone();
            sorted.sort_unstable();
            assert_eq!(names, sorted);
        }
    }

    /// Every frequency the card asked for is retained, not just the first.
    #[test]
    fn a_swept_sensitivity_keeps_every_frequency_it_solved() {
        let netlist = rspice_core::Netlist::parse(
            "RC low pass\n.param c=1n\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 {c}\n.end\n",
        )
        .unwrap();
        let SimulationResult::SensitivityStudy { evidence } = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: "V(out)".to_owned(),
                    ac_mode: true,
                    frequency: Some(1.0e3),
                    filter: "R1 C1 PARAM:*".to_owned(),
                    sweep: Some(SensitivitySweep {
                        stop_frequency: 1.0e6,
                        points: 4,
                        variation: AcSweepType::Decade,
                    }),
                },
                &NoAbort,
            )
            .unwrap()
        else {
            panic!("expected sensitivity study evidence")
        };
        assert_eq!(evidence.point_count(), 13);
        assert!(evidence.is_swept());
        assert_eq!(evidence.frequency_at(0), Some(1.0e3));
        assert!(
            (evidence.frequency_at(12).unwrap() / 1.0e6 - 1.0).abs() < 1e-12,
            "{:?}",
            evidence.frequency_at(12)
        );
        // `PARAM:C` differentiates the same quantity `C1` does, so the two
        // columns agree at every point of the band.
        for point in 0..evidence.point_count() {
            let design = value(&evidence, "PARAM:C", point);
            let device = value(&evidence, "C1", point);
            assert!(
                (design - device).abs() <= 1e-6 * device.abs().max(1e-30),
                "point {point}: PARAM:C {design} vs C1 {device}"
            );
        }
    }

    /// The closed form of `H = 1/(1 + j w R C)`, at every solved point.
    #[test]
    fn an_rc_low_pass_sweep_matches_its_closed_form_through_the_studio_bridge() {
        let resistance = 1.0e3_f64;
        let capacitance = 1.0e-9_f64;
        let corner = 1.0 / (std::f64::consts::TAU * resistance * capacitance);
        let netlist = rspice_core::Netlist::parse(
            "RC low pass\n.param c=1n\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 {c}\n.end\n",
        )
        .unwrap();
        let SimulationResult::SensitivityStudy { evidence } = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: "V(out)".to_owned(),
                    ac_mode: true,
                    frequency: Some(corner / 100.0),
                    filter: "R1 C1".to_owned(),
                    sweep: Some(SensitivitySweep {
                        stop_frequency: corner * 100.0,
                        points: 5,
                        variation: AcSweepType::Decade,
                    }),
                },
                &NoAbort,
            )
            .unwrap()
        else {
            panic!("expected sensitivity study evidence")
        };
        for point in 0..evidence.point_count() {
            let frequency = evidence.frequency_at(point).unwrap();
            let omega = std::f64::consts::TAU * frequency;
            let x = omega * resistance * capacitance;
            let scale = (1.0 + x * x).powf(-1.5);
            // d|H|/dR and d|H|/dC, and the phase derivatives beside them.
            for (name, own, magnitude, phase) in [
                (
                    "R1",
                    resistance,
                    -omega * omega * resistance * capacitance * capacitance * scale,
                    -omega * capacitance / (1.0 + x * x),
                ),
                (
                    "C1",
                    capacitance,
                    -omega * omega * resistance * resistance * capacitance * scale,
                    -omega * resistance / (1.0 + x * x),
                ),
            ] {
                let measured = value(&evidence, name, point);
                assert!(
                    (measured - magnitude).abs() <= 1e-5 * magnitude.abs().max(1e-18),
                    "{name} d|H|/dp at {frequency} Hz: {measured} vs {magnitude}"
                );
                let measured_phase = row(&evidence, name).phase[point].value().unwrap();
                assert!(
                    (measured_phase - phase).abs() <= 1e-5 * phase.abs().max(1e-18),
                    "{name} phase derivative at {frequency} Hz: {measured_phase} vs {phase}"
                );
                // Normalized is the same derivative scaled by p / |H|.
                let expected = own * magnitude * (1.0 + x * x).sqrt();
                let measured_normalized = normalized(&evidence, name, point);
                assert!(
                    (measured_normalized - expected).abs() <= 1e-5 * expected.abs().max(1e-18),
                    "{name} normalized at {frequency} Hz: {measured_normalized} vs {expected}"
                );
            }
        }
    }

    /// The card the Studio writes, read by the engine's own card runner,
    /// produces the rows this bridge produced — to the bit.
    #[test]
    fn the_studio_and_the_command_line_agree_on_one_sens_deck() {
        let deck = "Agreement\n.param drive=2\nV1 in 0 DC 4 AC 1\nR1 in out {drive}\nR2 out 0 3\nC1 out 0 1n\n.end\n";
        let netlist = rspice_core::Netlist::parse(deck).unwrap();
        for sweep in [
            None,
            Some(SensitivitySweep {
                stop_frequency: 1.0e5,
                points: 3,
                variation: AcSweepType::Decade,
            }),
        ] {
            for ac_mode in [false, true] {
                if sweep.is_some() && !ac_mode {
                    continue;
                }
                let config = SensitivityConfig {
                    output_var: "V(out)".to_owned(),
                    ac_mode,
                    frequency: ac_mode.then_some(1.0e3),
                    filter: "R* PARAM:*".to_owned(),
                    sweep,
                };
                let SimulationResult::SensitivityStudy { evidence } = EngineBridge::new()
                    .run_sensitivity(&netlist, &config, &NoAbort)
                    .unwrap()
                else {
                    panic!("expected sensitivity study evidence")
                };

                // The card the Studio wrote, spliced into the deck ahead of
                // `.end` exactly as preparation splices it, then parsed and
                // run by the engine.
                let card_deck = deck.replace(".end\n", &format!("{}\n.end\n", config.to_spice()));
                let card_netlist = rspice_core::Netlist::parse(&card_deck).unwrap();
                let card = card_netlist
                    .analyses
                    .iter()
                    .find(|analysis| {
                        matches!(
                            analysis,
                            rspice_core::netlist::AnalysisCommand::Sensitivity { .. }
                        )
                    })
                    .expect("the Studio's card parses as a .SENS card")
                    .clone();
                let engine = EngineBridge::new().engine_for_netlist(&card_netlist);
                let from_card = engine
                    .run_sensitivity_from_card_with_abort(&card_netlist, &card, &NoAbort)
                    .expect("the engine runs its own card");

                match from_card {
                    rspice_core::engine::SensitivityCardResult::Dc(result) => {
                        assert_eq!(evidence.point_count(), 1);
                        let mut names: Vec<&str> = result
                            .sensitivities
                            .iter()
                            .map(|entry| entry.vector_name.as_str())
                            .collect();
                        names.sort_unstable();
                        let studio: Vec<&str> = evidence
                            .rows
                            .iter()
                            .map(|row| row.parameter.as_str())
                            .collect();
                        assert_eq!(studio, names, "{deck}");
                        for entry in &result.sensitivities {
                            let row = row(&evidence, &entry.vector_name);
                            assert_eq!(row.raw[0], SensitivityValue::Available(entry.absolute));
                            assert_eq!(row.normalized[0], entry.normalized);
                            assert_eq!(row.nominal_value, entry.nominal_value);
                        }
                    }
                    rspice_core::engine::SensitivityCardResult::Ac(result) => {
                        assert_eq!(evidence.point_count(), result.frequencies.len());
                        for (point, frequency) in result.frequencies.iter().enumerate() {
                            assert_eq!(evidence.frequency_at(point), Some(*frequency));
                        }
                        let mut names: Vec<&str> = result
                            .sensitivities
                            .iter()
                            .map(|trace| trace.vector_name.as_str())
                            .collect();
                        names.sort_unstable();
                        let studio: Vec<&str> = evidence
                            .rows
                            .iter()
                            .map(|row| row.parameter.as_str())
                            .collect();
                        assert_eq!(studio, names, "{deck}");
                        for trace in &result.sensitivities {
                            let row = row(&evidence, &trace.vector_name);
                            assert_eq!(row.raw, trace.magnitude);
                            assert_eq!(row.phase, trace.phase);
                            assert_eq!(row.nominal_value, trace.nominal_value);
                            let normalized: Vec<SensitivityValue<f64>> = trace
                                .normalized
                                .iter()
                                .map(|value| value.map(|value| value.re))
                                .collect();
                            assert_eq!(row.normalized, normalized);
                        }
                    }
                }
            }
        }
    }

    /// One point per decade from `f` to `f` is exactly `f`, which is what the
    /// single-frequency card means and why it goes through the same function.
    #[test]
    fn one_point_per_decade_from_f_to_f_is_exactly_f() {
        for frequency in [1.0_f64, 60.0, 1.0e6, 2.5e9] {
            let grid = rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
                rspice_core::netlist::FreqVariation::Dec,
                1,
                frequency,
                frequency,
                &NoAbort,
            )
            .expect("a one-point decade sweep is a valid grid");
            assert_eq!(grid, vec![frequency]);
        }
    }

    /// A sweep the engine refuses is refused here, in the engine's words.
    #[test]
    fn a_sweep_the_engine_refuses_is_refused_in_its_words() {
        let netlist =
            rspice_core::Netlist::parse("Refused\nV1 out 0 AC 1\nR1 out 0 1k\n.end\n").unwrap();
        let error = EngineBridge::new()
            .run_sensitivity(
                &netlist,
                &SensitivityConfig {
                    output_var: "V(out)".to_owned(),
                    ac_mode: true,
                    frequency: Some(1.0e6),
                    filter: String::new(),
                    sweep: Some(SensitivitySweep {
                        stop_frequency: 1.0,
                        points: 10,
                        variation: AcSweepType::Decade,
                    }),
                },
                &NoAbort,
            )
            .expect_err("a descending decade sweep is not a grid");
        let sentence = rspice_core::analysis::ac::try_ac_sweep_frequencies_with_abort(
            rspice_core::netlist::FreqVariation::Dec,
            10,
            1.0e6,
            1.0,
            &NoAbort,
        )
        .expect_err("the engine refuses the same grid")
        .to_string();
        let SimulationError::InvalidConfig(message) = &error else {
            panic!("a refused grid is an invalid configuration: {error:?}");
        };
        assert_eq!(message, &sentence);
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
                filter: DESIGN_PARAMETERS_FILTER.to_owned(),
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

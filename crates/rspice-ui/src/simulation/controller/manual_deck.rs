use std::path::Path;

use rspice_core::netlist::{
    AnalysisCommand, FreqVariation, Netlist, PoleZeroAnalysisType, PoleZeroTransferType,
};

use super::*;

impl SimulationController {
    pub(super) fn build_manual_analysis_queue_from_source(
        source: &str,
        source_path: Option<&Path>,
    ) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
        let parsed = match source_path {
            Some(path) => Netlist::parse_with_path(source, path),
            None => Netlist::parse(source),
        }
        .map_err(|err| vec![format!("Manual netlist parse error: {}", err)])?;

        Self::build_manual_analysis_queue(&parsed.analyses)
    }

    fn build_manual_analysis_queue(
        analyses: &[AnalysisCommand],
    ) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
        let mut queue = Vec::new();
        let mut errors = Vec::new();

        for analysis in analyses {
            match Self::manual_analysis_to_queue_entries(analysis, analyses) {
                Ok(entries) => {
                    for entry in entries {
                        if let Some(config) = &entry.config {
                            if let Err(errs) = config.validate() {
                                errors.push(format!(
                                    "{} config is invalid: {}",
                                    entry.spec.run_type().display_name(),
                                    errs.join(", ")
                                ));
                                continue;
                            }
                        }
                        queue.push(entry);
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(queue)
        } else {
            Err(errors)
        }
    }

    fn manual_analysis_to_queue_entries(
        analysis: &AnalysisCommand,
        all_analyses: &[AnalysisCommand],
    ) -> Result<Vec<QueuedAnalysis>, String> {
        let entry = |spec: AnalysisSpec,
                     config: Option<AnalysisConfig>,
                     spec_options: SpecExecutionOptions,
                     analysis_line: String| {
            QueuedAnalysis {
                spec,
                config,
                spec_options,
                analysis_line,
            }
        };

        let entries = match analysis {
            AnalysisCommand::Op => vec![entry(
                AnalysisSpec::DcOp,
                Some(AnalysisConfig::DcOp),
                SpecExecutionOptions::default(),
                ".op".to_string(),
            )],
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                sweep2,
            } => {
                let (source2, start2, stop2, step2) = if let Some(sweep) = sweep2 {
                    (
                        Some(sweep.source.clone()),
                        Some(sweep.start),
                        Some(sweep.stop),
                        Some(sweep.step),
                    )
                } else {
                    (None, None, None, None)
                };
                let spec = AnalysisSpec::DcSweep {
                    source_name: source.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2: source2.clone(),
                    start2,
                    stop2,
                    step2,
                };
                let config = AnalysisConfig::DcSweep(DcSweepConfig {
                    source: source.clone(),
                    start: *start,
                    stop: *stop,
                    step: *step,
                    source2,
                    start2,
                    stop2,
                    step2,
                });
                vec![entry(
                    spec,
                    Some(config.clone()),
                    SpecExecutionOptions::default(),
                    config.to_spice(),
                )]
            }
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => {
                let sweep = Self::manual_frequency_sweep(*variation);
                let spec = AnalysisSpec::Ac {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points,
                    sweep,
                };
                let config = AnalysisConfig::Ac(AcAnalysisConfig {
                    sweep_type: Self::manual_ac_sweep_type(*variation),
                    num_points: *points,
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                });
                vec![entry(
                    spec,
                    Some(config.clone()),
                    SpecExecutionOptions::default(),
                    config.to_spice(),
                )]
            }
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                uic,
            } => {
                let start_time = start.unwrap_or(0.0);
                let spec = AnalysisSpec::Transient {
                    stop_time: *stop,
                    step_time: *step,
                    start_time,
                    max_timestep: *max_step,
                    uic: *uic,
                };
                let config = AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: *stop,
                    step_time: *step,
                    start_time,
                    max_timestep: *max_step,
                    uic: *uic,
                });
                vec![entry(
                    spec,
                    Some(config.clone()),
                    SpecExecutionOptions::default(),
                    config.to_spice(),
                )]
            }
            AnalysisCommand::Noise {
                output_node,
                reference_node,
                input_source,
                variation,
                points,
                start_freq,
                stop_freq,
            } => {
                let spec = AnalysisSpec::Noise {
                    output_node: output_node.clone(),
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_decade: *points,
                    temperature: 300.0,
                };
                let config = AnalysisConfig::Noise(NoiseAnalysisConfig {
                    output_node: output_node.clone(),
                    reference_node: reference_node.clone().unwrap_or_else(|| "0".to_string()),
                    input_source: input_source.clone(),
                    sweep_type: Self::manual_ac_sweep_type(*variation),
                    num_points: *points,
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                });
                vec![entry(
                    spec,
                    Some(config.clone()),
                    SpecExecutionOptions::default(),
                    config.to_spice(),
                )]
            }
            AnalysisCommand::PoleZero {
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                transfer_type,
                analysis_type,
            } => {
                let transfer_type = Self::manual_pz_transfer_type(*transfer_type).to_string();
                let analysis_type = Self::manual_pz_analysis_type(*analysis_type).to_string();
                let spec = AnalysisSpec::PoleZero {
                    input_node: input_pos.clone(),
                    input_ref: input_neg.clone(),
                    output_node: output_pos.clone(),
                    output_ref: output_neg.clone(),
                    transfer_type: transfer_type.clone(),
                    analysis_type: analysis_type.clone(),
                };
                let config = AnalysisConfig::PoleZero(PoleZeroConfig {
                    input_node: input_pos.clone(),
                    input_ref: input_neg.clone(),
                    output_node: output_pos.clone(),
                    output_ref: output_neg.clone(),
                    transfer_type,
                    analysis_type: match *analysis_type {
                        ref mode if mode == "POL" => PzAnalysisType::PolesOnly,
                        ref mode if mode == "ZER" => PzAnalysisType::ZerosOnly,
                        _ => PzAnalysisType::PoleZero,
                    },
                });
                vec![entry(
                    spec,
                    Some(config.clone()),
                    SpecExecutionOptions::default(),
                    config.to_spice(),
                )]
            }
            AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                ac_sweep,
            } => {
                let output_var = Self::manual_voltage_probe(output_node, reference_node.as_deref());
                let spec = AnalysisSpec::Sensitivity {
                    output_var: output_var.clone(),
                    ac_mode: ac_sweep.is_some(),
                    frequency: ac_sweep.map(|sweep| sweep.start_freq),
                };
                let config = AnalysisConfig::Sensitivity(SensitivityConfig {
                    output_var,
                    ac_mode: ac_sweep.is_some(),
                    frequency: ac_sweep.map(|sweep| sweep.start_freq),
                });
                vec![entry(
                    spec,
                    Some(config.clone()),
                    SpecExecutionOptions::default(),
                    config.to_spice(),
                )]
            }
            AnalysisCommand::Disto {
                variation,
                points,
                start_freq,
                stop_freq,
                f2_over_f1,
            } => {
                let spec = AnalysisSpec::Disto {
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    points_per_unit: *points,
                    sweep: Self::manual_frequency_sweep(*variation),
                    f2_over_f1: *f2_over_f1,
                };
                vec![entry(
                    spec,
                    None,
                    SpecExecutionOptions::default(),
                    Self::manual_disto_line(
                        *variation,
                        *points,
                        *start_freq,
                        *stop_freq,
                        *f2_over_f1,
                    ),
                )]
            }
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => {
                if *output_is_current {
                    return Err(
                        "Transfer Function: manual .tf current probes are not supported by the UI runner yet"
                            .to_string(),
                    );
                }
                let mut spec_options = SpecExecutionOptions::default();
                spec_options.tf = Some(crate::services::simulation_runner::TfRunConfig {
                    input_source: input_source.clone(),
                    output_node: output_node.clone(),
                    output_ref: reference_node
                        .clone()
                        .filter(|node| !node.trim().is_empty()),
                    ..crate::services::simulation_runner::TfRunConfig::default()
                });
                vec![entry(
                    AnalysisSpec::Tf,
                    None,
                    spec_options,
                    format!(
                        ".tf {} {}",
                        Self::manual_voltage_probe(output_node, reference_node.as_deref()),
                        input_source
                    ),
                )]
            }
            AnalysisCommand::Four {
                fundamental,
                outputs,
                num_harmonics,
            } => {
                let (start_time, stop_time) = Self::manual_fourier_window(all_analyses)?;
                let mut entries = Vec::with_capacity(outputs.len());
                for output in outputs {
                    let (output_node, output_ref) = Self::manual_fourier_output(output);
                    let spec = AnalysisSpec::Fourier {
                        fundamental_freq: *fundamental,
                        num_harmonics: *num_harmonics,
                        output_node,
                        output_ref,
                        start_time,
                        stop_time,
                    };
                    entries.push(entry(
                        spec,
                        None,
                        SpecExecutionOptions::default(),
                        format!(".four {} {}", fundamental, output),
                    ));
                }
                entries
            }
            AnalysisCommand::Stb {
                variation,
                points,
                start_freq,
                stop_freq,
                probe,
            } => {
                let spec = AnalysisSpec::Stb {
                    probe_node: probe.clone(),
                    start_freq: *start_freq,
                    stop_freq: *stop_freq,
                    sweep: Self::manual_frequency_sweep(*variation),
                    points_per_decade: *points,
                };
                vec![entry(
                    spec,
                    None,
                    SpecExecutionOptions::default(),
                    Self::manual_stb_line(*variation, *points, *start_freq, *stop_freq, probe),
                )]
            }
            AnalysisCommand::MonteCarlo(_)
            | AnalysisCommand::Step(_)
            | AnalysisCommand::Temp { .. } => Vec::new(),
        };

        Ok(entries)
    }

    fn manual_frequency_sweep(variation: FreqVariation) -> FrequencySweep {
        match variation {
            FreqVariation::Dec => FrequencySweep::Decade,
            FreqVariation::Oct => FrequencySweep::Octave,
            FreqVariation::Lin => FrequencySweep::Linear,
        }
    }

    fn manual_ac_sweep_type(variation: FreqVariation) -> AcSweepType {
        match variation {
            FreqVariation::Dec => AcSweepType::Decade,
            FreqVariation::Oct => AcSweepType::Octave,
            FreqVariation::Lin => AcSweepType::Linear,
        }
    }

    fn manual_sweep_keyword(variation: FreqVariation) -> &'static str {
        match variation {
            FreqVariation::Dec => "dec",
            FreqVariation::Oct => "oct",
            FreqVariation::Lin => "lin",
        }
    }

    fn manual_pz_transfer_type(transfer_type: PoleZeroTransferType) -> &'static str {
        match transfer_type {
            PoleZeroTransferType::Voltage => "VOL",
            PoleZeroTransferType::Current => "CUR",
        }
    }

    fn manual_pz_analysis_type(analysis_type: PoleZeroAnalysisType) -> &'static str {
        match analysis_type {
            PoleZeroAnalysisType::PoleZero => "PZ",
            PoleZeroAnalysisType::PolesOnly => "POL",
            PoleZeroAnalysisType::ZerosOnly => "ZER",
        }
    }

    fn manual_voltage_probe(node: &str, reference: Option<&str>) -> String {
        match reference.map(str::trim).filter(|node| !node.is_empty()) {
            Some(reference) => format!("V({},{})", node, reference),
            None => format!("V({})", node),
        }
    }

    fn manual_disto_line(
        variation: FreqVariation,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
        f2_over_f1: Option<f64>,
    ) -> String {
        let mut line = format!(
            ".disto {} {} {} {}",
            Self::manual_sweep_keyword(variation),
            points,
            start_freq,
            stop_freq
        );
        if let Some(ratio) = f2_over_f1 {
            line.push_str(&format!(" {}", ratio));
        }
        line
    }

    fn manual_stb_line(
        variation: FreqVariation,
        points: usize,
        start_freq: f64,
        stop_freq: f64,
        probe: &str,
    ) -> String {
        format!(
            ".stb {} {} {} {} probe={}",
            Self::manual_sweep_keyword(variation),
            points,
            start_freq,
            stop_freq,
            probe
        )
    }

    fn manual_fourier_window(analyses: &[AnalysisCommand]) -> Result<(f64, f64), String> {
        for analysis in analyses {
            if let AnalysisCommand::Tran { stop, start, .. } = analysis {
                return Ok((start.unwrap_or(0.0), *stop));
            }
        }
        Err("Fourier: .four requires .tran in the manual deck".to_string())
    }

    fn manual_fourier_output(output: &str) -> (String, String) {
        let trimmed = output.trim();
        let inner = trimmed
            .strip_prefix(['V', 'v'])
            .and_then(|s| s.strip_prefix('('))
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(trimmed);
        let mut parts = inner.split(',').map(str::trim);
        let node = parts.next().unwrap_or(inner).to_string();
        let reference = parts.next().unwrap_or("0").to_string();
        (node, reference)
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn manual_queue_uses_parsed_analysis_commands() {
        let source = "manual analyses\n\
             V1 in 0 DC 0 AC 1\n\
             V2 bias 0 DC 0\n\
             Vloop loop 0 0\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             .op\n\
             .ac dec 5 1 1e6\n\
             .tran 1n 10n 2n 500p uic\n\
             .dc V1 0 5 1 V2 0 1 0.5\n\
             .noise V(out,0) V1 oct 4 10 1k\n\
             .pz in 0 out 0 vol pol\n\
             .sens V(out,0) ac lin 3 100 300\n\
             .disto lin 7 1k 10k 1.2\n\
             .tf V(out,0) V1\n\
             .four 1k V(out)\n\
             .stb dec 12 10 1e6 probe=Vloop\n\
             .end\n";

        let queue =
            SimulationController::build_manual_analysis_queue_from_source(source, None).unwrap();

        assert_eq!(queue.len(), 11);
        assert!(matches!(queue[0].spec, AnalysisSpec::DcOp));
        assert!(matches!(
            queue[1].config,
            Some(AnalysisConfig::Ac(AcAnalysisConfig {
                sweep_type: AcSweepType::Decade,
                num_points: 5,
                start_freq: 1.0,
                stop_freq: 1e6,
            }))
        ));
        assert!(matches!(
            queue[2].config,
            Some(AnalysisConfig::Transient(TransientAnalysisConfig {
                step_time,
                stop_time,
                start_time,
                max_timestep: Some(max_timestep),
                uic: true,
            })) if step_time == 1e-9
                && stop_time == 1e-8
                && start_time == 2e-9
                && max_timestep == 5e-10
        ));
        assert!(matches!(
            queue[3].config,
            Some(AnalysisConfig::DcSweep(DcSweepConfig {
                ref source,
                start: 0.0,
                stop: 5.0,
                step: 1.0,
                ref source2,
                start2: Some(0.0),
                stop2: Some(1.0),
                step2: Some(0.5),
            })) if source == "V1" && source2.as_deref() == Some("V2")
        ));
        assert!(matches!(
            queue[4].config,
            Some(AnalysisConfig::Noise(NoiseAnalysisConfig {
                ref output_node,
                ref reference_node,
                ref input_source,
                sweep_type: AcSweepType::Octave,
                num_points: 4,
                start_freq: 10.0,
                stop_freq: 1000.0,
            })) if output_node.eq_ignore_ascii_case("out")
                && reference_node == "0"
                && input_source.eq_ignore_ascii_case("V1")
        ));
        assert!(matches!(
            queue[5].spec,
            AnalysisSpec::PoleZero {
                ref transfer_type,
                ref analysis_type,
                ..
            } if transfer_type == "VOL" && analysis_type == "POL"
        ));
        assert!(matches!(
            queue[6].config,
            Some(AnalysisConfig::Sensitivity(SensitivityConfig {
                ref output_var,
                ac_mode: true,
                frequency: Some(100.0),
            })) if output_var.eq_ignore_ascii_case("V(out,0)")
        ));
        assert!(matches!(
            queue[7].spec,
            AnalysisSpec::Disto {
                sweep: FrequencySweep::Linear,
                points_per_unit: 7,
                start_freq,
                stop_freq,
                f2_over_f1: Some(1.2),
            } if start_freq == 1000.0 && stop_freq == 10000.0
        ));
        assert!(matches!(queue[8].spec, AnalysisSpec::Tf));
        assert!(queue[8].spec_options.tf.is_some());
        assert!(matches!(
            queue[9].spec,
            AnalysisSpec::Fourier {
                fundamental_freq: 1000.0,
                num_harmonics: 9,
                ref output_node,
                ref output_ref,
                start_time,
                stop_time,
            } if output_node.eq_ignore_ascii_case("out")
                && output_ref == "0"
                && start_time == 2e-9
                && stop_time == 1e-8
        ));
        assert!(matches!(
            queue[10].spec,
            AnalysisSpec::Stb {
                ref probe_node,
                start_freq: 10.0,
                stop_freq: 1e6,
                points_per_decade: 12,
                ..
            } if probe_node == "VLOOP"
        ));
    }

    #[test]
    fn manual_netlist_composition_does_not_append_generated_analysis_lines() {
        let source = "deck\nR1 in 0 1k\n.op\n.end\n";
        let merged =
            SimulationController::compose_manual_netlist(source, &[".tran 1e-9 1e-6".to_string()]);

        assert_eq!(merged, "deck\nR1 in 0 1k\n.op\n.end\n");
    }

    #[test]
    fn manual_stb_preserves_non_decade_sweep_type() {
        let source = "manual stb\nVloop loop 0 0\n.stb lin 12 10 1e6 probe=Vloop\n.end\n";

        let queue =
            SimulationController::build_manual_analysis_queue_from_source(source, None).unwrap();

        assert_eq!(queue.len(), 1);
        assert!(matches!(
            queue[0].spec,
            AnalysisSpec::Stb {
                sweep: FrequencySweep::Linear,
                points_per_decade: 12,
                ..
            }
        ));
    }

    #[test]
    fn manual_fourier_uses_parsed_tran_window() {
        let source = "manual fourier\n\
            V1 out 0 sin(0 1 1k)\n\
            .tran 1n 20n 5n\n\
            .four 1k V(out)\n\
            .end\n";

        let queue =
            SimulationController::build_manual_analysis_queue_from_source(source, None).unwrap();

        let fourier = queue
            .iter()
            .find_map(|entry| match &entry.spec {
                AnalysisSpec::Fourier {
                    start_time,
                    stop_time,
                    ..
                } => Some((*start_time, *stop_time)),
                _ => None,
            })
            .expect("fourier entry exists");
        assert_eq!(fourier, (5e-9, 20e-9));
    }

    #[test]
    fn manual_fourier_requires_transient_card() {
        let source = "manual fourier\nV1 out 0 sin(0 1 1k)\n.four 1k V(out)\n.end\n";

        let errors = SimulationController::build_manual_analysis_queue_from_source(source, None)
            .expect_err("fourier without .tran should be rejected");

        assert!(
            errors
                .iter()
                .any(|err| err.contains(".four requires .tran"))
        );
    }
}

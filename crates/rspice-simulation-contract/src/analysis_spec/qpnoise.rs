//! Complete QPNOISE specification and native-card interchange with legacy defaults.
use super::types::AnalysisSpec;
use crate::config::FrequencySweep;
pub use crate::quasi_periodic_controls::QpnoiseControls;
use rspice_core::engine::{
    QpnoiseInput, QpnoiseIntegration, QpnoiseIntegrationMethod, QpnoiseLattices,
    QpnoiseObservation, QpnoiseOutput, QpnoiseRequest,
};
use rspice_core::netlist::{FreqVariation, PeriodicSweep, QpacSweep, QpnoiseCard};

impl AnalysisSpec {
    pub fn qpnoise_card(&self) -> Result<QpnoiseCard, String> {
        let Self::Qpnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            output_node,
            output_ref,
            input_source,
            lattice_min,
            lattice_max,
            integrated_noise,
            contributor_ranking,
            controls,
        } = self
        else {
            return Err("QPNOISE configuration requested for another analysis".into());
        };
        let mut outputs = vec![QpnoiseOutput {
            observation: match &controls.branch_current {
                Some(branch) => QpnoiseObservation::BranchCurrent {
                    branch: branch.clone(),
                },
                None => QpnoiseObservation::Voltage {
                    positive: output_node.clone(),
                    negative: output_ref.clone(),
                },
            },
            lattice: controls.output_lattice.clone(),
        }];
        outputs.extend(controls.additional_outputs.clone());
        let request = QpnoiseRequest {
            frequencies_hz: vec![0.0],
            frequency_axis: controls.frequency_axis,
            outputs,
            input: controls.input_referral.then(|| QpnoiseInput {
                source: input_source.clone(),
                lattice: controls.input_lattice.clone(),
            }),
            input_lattices: controls.noise_lattices.clone().unwrap_or_else(|| {
                QpnoiseLattices::Range {
                    minimum: lattice_min.clone(),
                    maximum: lattice_max.clone(),
                }
            }),
            sources: controls.sources.clone(),
            integration: integrated_noise.then(|| QpnoiseIntegration {
                band_hz: controls.integration_band,
                method: controls.integration_method,
            }),
            contributor_ranking: *contributor_ranking,
            noise_figure: controls.noise_figure.clone(),
            linear: controls.solver.clone(),
        };
        let mut card = request.to_qpnoise_card();
        card.sweep = match &controls.explicit_frequencies {
            Some(values) => QpacSweep::Explicit(values.clone()),
            None => QpacSweep::Generated(PeriodicSweep {
                start_freq: *start_freq,
                stop_freq: *stop_freq,
                points: *points_per_unit,
                variation: match sweep {
                    FrequencySweep::Linear => FreqVariation::Lin,
                    FrequencySweep::Decade => FreqVariation::Dec,
                    FrequencySweep::Octave => FreqVariation::Oct,
                },
            }),
        };
        QpnoiseRequest::validate_qpnoise_card(&card, &rspice_core::ResourceLimits::default())
            .map_err(|e| e.to_string())?;
        Ok(card)
    }
    pub fn from_qpnoise_card(card: &QpnoiseCard) -> Result<Self, String> {
        let mut request = QpnoiseRequest::from_qpnoise_card(card).map_err(|e| e.to_string())?;
        let (start_freq, stop_freq, points_per_unit, sweep, explicit_frequencies) =
            match &card.sweep {
                QpacSweep::Generated(s) => (
                    s.start_freq,
                    s.stop_freq,
                    s.points,
                    match s.variation {
                        FreqVariation::Lin => FrequencySweep::Linear,
                        FreqVariation::Dec => FrequencySweep::Decade,
                        FreqVariation::Oct => FrequencySweep::Octave,
                    },
                    None,
                ),
                QpacSweep::Explicit(values) => {
                    (0.0, 0.0, 1, FrequencySweep::Linear, Some(values.clone()))
                }
            };
        let primary = request.outputs.remove(0);
        let (output_node, output_ref, branch_current) = match primary.observation {
            QpnoiseObservation::Voltage { positive, negative } => (positive, negative, None),
            QpnoiseObservation::BranchCurrent { branch } => {
                (String::new(), String::new(), Some(branch))
            }
        };
        let (lattice_min, lattice_max, noise_lattices) = match request.input_lattices {
            QpnoiseLattices::Range { minimum, maximum } => (minimum, maximum, None),
            selection => (Vec::new(), Vec::new(), Some(selection)),
        };
        let input_referral = request.input.is_some();
        let (input_source, input_lattice) = request
            .input
            .map(|i| (i.source, i.lattice))
            .unwrap_or_else(|| (String::new(), vec![0; primary.lattice.len()]));
        Ok(Self::Qpnoise {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            output_node,
            output_ref,
            input_source,
            lattice_min,
            lattice_max,
            integrated_noise: request.integration.is_some(),
            contributor_ranking: request.contributor_ranking,
            controls: QpnoiseControls {
                frequency_axis: request.frequency_axis,
                explicit_frequencies,
                input_referral,
                input_lattice,
                output_lattice: primary.lattice,
                branch_current,
                additional_outputs: request.outputs,
                noise_lattices,
                sources: request.sources,
                integration_band: request.integration.as_ref().and_then(|i| i.band_hz),
                integration_method: request
                    .integration
                    .map_or(QpnoiseIntegrationMethod::Linear, |i| i.method),
                noise_figure: request.noise_figure,
                solver: request.linear,
            },
        })
    }
}

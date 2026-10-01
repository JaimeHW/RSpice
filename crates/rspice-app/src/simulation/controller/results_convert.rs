//! App result adoption: choose labels, supply the host clock and decorate exact data.

use super::*;
#[cfg(test)]
use crate::state::{FloquetSpectrumEvidence, PeriodicNoiseOutputQuantity};
#[cfg(test)]
use std::sync::Arc;

impl SimulationController {
    pub(super) fn retain_periodic_noise_result_metadata(&self, result: &mut AnalysisResult) {
        rspice_simulation::result_conversion::retain_periodic_noise_metadata(
            &mut result.data,
            self.current_spec_options
                .as_ref()
                .and_then(|options| options.pnoise.as_ref()),
            self.current_periodic_carrier_hz,
        );
    }

    pub(super) fn convert_to_analysis_result_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();
        self.convert_to_analysis_result_with_metadata_owned(sim_result, analysis_type, &label)
    }

    pub(in crate::simulation) fn convert_to_analysis_result_with_metadata_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        let retained =
            rspice_simulation::result_conversion::convert(sim_result, analysis_type, label, || {
                crate::time_compat::unix_epoch().as_secs_f64()
            });
        let mut index = 0;
        AnalysisResult {
            data: retained.map_waveforms(|data| {
                let waveform = crate::state::WaveformData {
                    data,
                    color: Self::color_for_index(index),
                    visible: true,
                    display_cache: None,
                };
                index += 1;
                waveform
            }),
        }
    }

    pub(super) fn color_for_index(idx: usize) -> String {
        const COLORS: &[&str] = &[
            "#3B82F6", // Blue
            "#10B981", // Green
            "#F97316", // Orange
            "#8B5CF6", // Purple
            "#EC4899", // Pink
            "#EAB308", // Yellow
            "#14B8A6", // Teal
            "#EF4444", // Red
        ];
        COLORS[idx % COLORS.len()].to_string()
    }
}

#[cfg(test)]
mod operating_point_conversion_tests {
    use super::*;

    #[test]
    fn branch_current_is_wrapped_exactly_once_in_retained_results() {
        let sim_result = crate::simulation::SimulationResult::DcOp(Box::new(
            rspice_simulation::results::DcOpResult {
                configuration: crate::simulation::dialog::OpConfig::default(),
                branch_currents: HashMap::from([("V1".to_owned(), -1.0e-3)]),
                ..Default::default()
            },
        ));
        let result = SimulationController::new()
            .convert_to_analysis_result_owned(sim_result, &AnalysisConfig::dc_op());
        let currents = &result.dc_op.as_ref().unwrap().branch_currents;
        assert_eq!(currents.len(), 1);
        assert_eq!(currents[0].name, "I(V1)");
    }
}

#[cfg(test)]
mod floquet_payload_conversion_tests {
    use super::*;
    use crate::simulation::SimulationResult;
    use num_complex::Complex64;

    fn pss_operating_point(multipliers: Vec<Complex64>) -> rspice_core::engine::PssOperatingPoint {
        let config = rspice_core::analysis::PssConfig::new(1.0)
            .with_harmonics(4)
            .with_points_per_period(16);
        let time = (0..=16)
            .map(|index| index as f64 / 16.0)
            .collect::<Vec<_>>();
        let waveform = time
            .iter()
            .map(|time| (2.0 * std::f64::consts::PI * time).sin())
            .collect();
        let order = multipliers.len();
        let floquet_evidence = if order == 0 {
            rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes
        } else {
            let certificate = rspice_core::analysis::FloquetSpectrumCertificate::new(
                order,
                0.0,
                rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(
                    order,
                ),
            )
            .unwrap();
            rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate }
        };
        let monodromy = multipliers
            .iter()
            .enumerate()
            .map(|(row, _)| {
                multipliers
                    .iter()
                    .enumerate()
                    .map(
                        |(column, multiplier)| {
                            if row == column { multiplier.re } else { 0.0 }
                        },
                    )
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let result = rspice_core::analysis::pss::PssResult {
            period: 1.0,
            frequency: 1.0,
            iterations: 2,
            residual_norm: 1.0e-10,
            time,
            waveforms: vec![rspice_core::analysis::pss::PeriodicWaveform::from_values(
                waveform,
            )],
            node_names: vec!["out".to_owned()],
            branch_names: Vec::new(),
            branch_waveforms: Vec::new(),
            period_detected: false,
            floquet_multipliers: multipliers.clone(),
            floquet_evidence,
            floquet_orbit_kind: rspice_core::analysis::FloquetOrbitKind::Driven,
            trivial_floquet_multiplier_index: None,
        };
        rspice_core::engine::PssOperatingPoint::try_from_parts(
            config,
            rspice_core::engine::PssAnalysisResult {
                is_stable: result.is_stable(),
                result,
                iterations: 2,
                final_residual: 1.0e-10,
                period: 1.0,
                monodromy,
                floquet_multipliers: multipliers,
            },
            vec![0.0; order],
        )
        .unwrap()
    }

    fn pss_result(multipliers: Vec<Complex64>) -> SimulationResult {
        let display = rspice_simulation::results::WaveformData::new_time_domain(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 0.0],
        );
        SimulationResult::Transient {
            spectra: Vec::new(),
            time: vec![0.0, 1.0],
            waveforms: HashMap::from([("V(out)".to_owned(), display)]),
            measurements: Vec::new(),
            periodic_state: Some(Arc::new(pss_operating_point(multipliers))),
            convergence: Default::default(),
            events: Default::default(),
        }
    }

    fn pstb_result(multipliers: &[f64], display_count: usize) -> SimulationResult {
        let period = 2.0;
        let modes = multipliers
            .iter()
            .enumerate()
            .map(
                |(index, multiplier)| rspice_simulation::results::PstbFloquetMode {
                    multiplier: (*multiplier, 0.0),
                    exponent: (multiplier.ln() / period, 0.0),
                    probe_participation: (index + 1) as f64 / multipliers.len().max(1) as f64,
                    is_unstable: false,
                    is_trivial: false,
                    subharmonic_order: None,
                },
            )
            .collect::<Vec<_>>();
        let floquet_evidence = if modes.is_empty() {
            rspice_core::analysis::FloquetSpectrumEvidence::NoDynamicModes
        } else {
            let order = modes.len();
            let certificate = rspice_core::analysis::FloquetSpectrumCertificate::new(
                order,
                0.0,
                rspice_core::analysis::FloquetSpectrumCertificate::canonical_qualification_tolerance(
                    order,
                ),
            )
            .unwrap();
            rspice_core::analysis::FloquetSpectrumEvidence::Qualified { certificate }
        };
        let mode_indices = (0..display_count)
            .map(|index| index as f64 + 1.0)
            .collect::<Vec<_>>();
        let waveforms = if mode_indices.is_empty() {
            HashMap::new()
        } else {
            let waveform = rspice_simulation::results::WaveformData::new_time_domain_in_unit(
                "Floquet |lambda|",
                mode_indices.clone(),
                multipliers[..display_count].to_vec(),
                "",
            );
            HashMap::from([("Floquet |lambda|".to_owned(), waveform)])
        };
        let max_multiplier_magnitude = multipliers.first().copied().unwrap_or(0.0);
        SimulationResult::Pstb {
            period,
            fundamental_frequency: 1.0 / period,
            modes,
            floquet_evidence,
            orbit_kind: rspice_core::analysis::FloquetOrbitKind::Driven,
            stability_threshold: 1.0 + 1.0e-6,
            probe_instance: "LPROBE".to_owned(),
            detect_subharmonics: false,
            trivial_multiplier_index: None,
            stability_verdict: rspice_core::analysis::FloquetStabilityVerdict::Stable,
            stability_classification: rspice_core::analysis::pstb::StabilityType::Stable,
            min_stability_margin_db: multipliers
                .first()
                .map(|multiplier| -20.0 * multiplier.log10()),
            max_multiplier_magnitude,
            num_unstable: 0,
            subharmonics: Vec::new(),
            converged: true,
            iterations: 0,
            mode_indices,
            waveforms,
        }
    }

    #[test]
    fn pss_retains_three_authenticated_modes_beside_one_display_curve_only() {
        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            pss_result(vec![
                Complex64::new(0.75, 0.0),
                Complex64::new(0.5, 0.0),
                Complex64::new(0.25, 0.0),
            ]),
            AnalysisType::Pss,
            "PSS",
        );

        assert!(result.success, "{:?}", result.error_message);
        assert_eq!(
            result.scalar_evidence("pss.period")[0]
                .value_in_unit("ms")
                .unwrap(),
            Some(1000.0)
        );
        assert_eq!(
            result.scalar_evidence("pss_mode_count")[0]
                .value_in_unit("count")
                .unwrap(),
            Some(3.0)
        );
        assert!(
            result.scalar_evidence("pss_mode_count")[0]
                .value_in_unit("V")
                .is_err()
        );

        assert_eq!(result.waveforms.len(), 1);
        let Some(AnalysisResultPayload::PssFloquet {
            multipliers,
            floquet_evidence,
            ..
        }) = &result.result_payload
        else {
            panic!("missing PSS Floquet payload");
        };
        assert_eq!(multipliers.len(), 3);
        assert!(matches!(
            floquet_evidence,
            FloquetSpectrumEvidence::Qualified { certificate }
                if certificate.problem_order == 3
        ));
        let json = serde_json::to_string(result.result_payload.as_ref().unwrap()).unwrap();
        assert!(!json.contains("monodromy"));
        assert!(!json.contains("shooting"));
        assert!(!json.contains("config"));
    }

    #[test]
    fn pss_zero_dynamic_modes_remains_a_viewable_authenticated_result() {
        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            pss_result(Vec::new()),
            AnalysisType::Pss,
            "PSS",
        );

        assert!(result.success, "{:?}", result.error_message);
        assert!(result.has_data());
        assert!(matches!(
            result.result_payload,
            Some(AnalysisResultPayload::PssFloquet {
                ref multipliers,
                floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
                ..
            }) if multipliers.is_empty()
        ));
    }

    #[test]
    fn pstb_retains_three_authenticated_modes_beside_one_display_curve_only() {
        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            pstb_result(&[0.75, 0.5, 0.25], 1),
            AnalysisType::Pstb,
            "PSTB",
        );

        assert!(result.success, "{:?}", result.error_message);
        assert_eq!(
            result.scalar_evidence("pstb.period")[0]
                .value_in_unit("ms")
                .unwrap(),
            Some(2000.0)
        );
        assert_eq!(
            result.scalar_evidence("pstb_mode_count")[0]
                .value_in_unit("count")
                .unwrap(),
            Some(3.0)
        );
        assert!(
            result.scalar_evidence("pstb_mode_count")[0]
                .value_in_unit("V")
                .is_err()
        );

        assert_eq!(result.waveforms.len(), 1);
        let Some(AnalysisResultPayload::Pstb {
            modes,
            stability_threshold,
            probe_instance,
            detect_subharmonics,
            ..
        }) = &result.result_payload
        else {
            panic!("missing PSTB Floquet payload");
        };
        assert_eq!(modes.len(), 3);
        assert_eq!(*stability_threshold, Some(1.0 + 1.0e-6));
        assert_eq!(probe_instance.as_deref(), Some("LPROBE"));
        assert_eq!(*detect_subharmonics, Some(false));
    }

    #[test]
    fn pstb_zero_dynamic_modes_needs_no_display_curve() {
        let result = SimulationController::new().convert_to_analysis_result_with_metadata_owned(
            pstb_result(&[], 0),
            AnalysisType::Pstb,
            "PSTB",
        );

        assert!(result.success, "{:?}", result.error_message);
        assert!(result.waveforms.is_empty());
        assert!(result.has_data());
        assert!(matches!(
            result.result_payload,
            Some(AnalysisResultPayload::Pstb {
                ref modes,
                floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
                min_stability_margin_db: None,
                ..
            }) if modes.is_empty()
        ));
    }
}

#[cfg(test)]
mod noise_conversion_tests;

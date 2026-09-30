//! Touchstone export after an S-parameter run.
//!
//! Builds the port-ordered network dataset and writes it in the requested
//! format when the run configuration asked for an export.

use super::*;
use crate::diagnostics::ConsoleMessage;
use rspice_formats::{WaveformDataset, WaveformFormat, WaveformWriter};

pub(super) struct PreparedTouchstoneExport {
    path: std::path::PathBuf,
    contents: String,
}

impl SimulationController {
    /// Build the exact export payload without changing the filesystem.
    ///
    /// Completion orchestration prepares this while the raw S-parameter
    /// result is available, then commits it only after the immutable result
    /// has been retained successfully. This prevents an exported file from
    /// claiming success for a run the Studio rejected at its retention
    /// boundary.
    pub(super) fn prepare_touchstone_export(
        &self,
        result: &crate::simulation::SimulationResult,
        run_id: u64,
    ) -> Result<Option<PreparedTouchstoneExport>, String> {
        let Some(crate::simulation::multi_run::AnalysisSpec::SParameter { z0, ports, .. }) =
            self.current_spec.as_ref()
        else {
            return Ok(None);
        };
        let Some(touchstone_version) = self.touchstone_export_policy.version() else {
            return Ok(None);
        };

        let z0_by_port: Vec<f64> = ports.iter().map(|port| port.z0.unwrap_or(*z0)).collect();
        let dataset =
            Self::build_touchstone_dataset(result, *z0, &z0_by_port, touchstone_version as usize)?;
        let num_ports = dataset
            .metadata
            .get("num_ports")
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(2);
        let Some(path) =
            self.touchstone_export_policy
                .output_path(run_id, self.current_analysis_idx, num_ports)
        else {
            return Err(
                "prepared output policy is unavailable for the completed analysis".to_owned(),
            );
        };

        let writer = WaveformWriter::new(WaveformFormat::Touchstone);
        let contents = writer
            .write_text(&dataset)
            .map_err(|error| error.to_string())?;
        Ok(Some(PreparedTouchstoneExport { path, contents }))
    }

    pub(super) fn commit_touchstone_export(
        state: &mut AppState,
        export_io: &(impl crate::workbench::workflows::export_workflow::ExportWorkflowIo + ?Sized),
        prepared: PreparedTouchstoneExport,
    ) {
        match export_io.write_new_text_file(&prepared.path, &prepared.contents) {
            Ok(()) => state.push_sim_message(ConsoleMessage::info(
                Self::touchstone_export_completed_message(&prepared.path),
            )),
            Err(e) => state.push_sim_message(ConsoleMessage::warning(format!(
                "Touchstone export failed: {}",
                e
            ))),
        }
    }

    pub(super) fn build_touchstone_dataset(
        result: &crate::simulation::SimulationResult,
        z0: f64,
        z0_by_port: &[f64],
        touchstone_version: usize,
    ) -> Result<WaveformDataset, String> {
        let (frequencies, waveforms, references, noise_temperature) = match result {
            crate::simulation::SimulationResult::Ac {
                frequencies,
                waveforms,
                reference_impedances_ohm,
                noise_reference_temperature_kelvin,
                ..
            } => (
                frequencies,
                waveforms,
                reference_impedances_ohm.as_deref(),
                *noise_reference_temperature_kelvin,
            ),
            _ => return Err("result is not frequency-domain S-parameter data".to_string()),
        };
        rspice_formats::waveform_io::result::project_sparameter_waveforms(
            frequencies,
            waveforms,
            references,
            noise_temperature,
            z0,
            z0_by_port,
            touchstone_version,
        )
        .map_err(|error| error.to_string())
    }

    pub(super) fn touchstone_export_completed_message(path: &std::path::Path) -> String {
        #[cfg(target_arch = "wasm32")]
        {
            format!(
                "Touchstone download started: {} (confirm the browser accepted the download)",
                path.display()
            )
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            format!("Exported Touchstone: {}", path.display())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_simulation::results::WaveformData;

    /// An N-port S-parameter result, as the runner assembles one.
    fn matrix_result(num_ports: usize) -> crate::simulation::SimulationResult {
        let frequencies = vec![1.0e9, 2.0e9];
        let mut waveforms = std::collections::HashMap::new();
        for row in 1..=num_ports {
            for col in 1..=num_ports {
                let name = if num_ports <= 9 {
                    format!("S{row}{col}")
                } else {
                    format!("S{row}_{col}")
                };
                waveforms.insert(
                    name.clone(),
                    WaveformData {
                        name,
                        x_values: frequencies.clone(),
                        y_values: vec![0.5, 0.4],
                        y_unit: String::new(),
                        is_complex: true,
                        y_imag: Some(vec![0.0, 0.0]),
                    },
                );
            }
        }
        crate::simulation::SimulationResult::Ac {
            convergence: None,
            noise_reference_temperature_kelvin: None,
            reference_impedances_ohm: None,
            frequencies,
            waveforms,
            measurements: Vec::new(),
        }
    }

    /// Legacy results without retained references must validate their fallback table.
    #[test]
    fn touchstone_noise_automatic_export_uses_retained_temperature_and_ports() {
        let mut result = matrix_result(2);
        let crate::simulation::SimulationResult::Ac {
            frequencies,
            waveforms,
            reference_impedances_ohm,
            noise_reference_temperature_kelvin,
            ..
        } = &mut result
        else {
            unreachable!()
        };
        *reference_impedances_ohm = Some(vec![75.0, 100.0]);
        *noise_reference_temperature_kelvin = Some(580.0);
        for (name, value, imaginary) in [
            ("Fmin", 2.0, None),
            ("Rn", 15.0, None),
            ("Sopt", 0.0, Some(vec![-0.5; 2])),
        ] {
            waveforms.insert(
                name.into(),
                WaveformData {
                    name: name.into(),
                    x_values: frequencies.clone(),
                    y_values: vec![value; 2],
                    y_unit: if name == "Rn" { "Ω" } else { "1" }.into(),
                    is_complex: imaginary.is_some(),
                    y_imag: imaginary,
                },
            );
        }
        let dataset =
            SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 50.0], 2)
                .unwrap();
        let text = WaveformWriter::new(WaveformFormat::Touchstone)
            .write_text(&dataset)
            .unwrap();
        assert!(text.contains("[Number of Noise Frequencies] 2"));
        let imported = rspice_formats::read_touchstone_bytes("noise.ts", text.as_bytes()).unwrap();
        assert_eq!(imported.metadata["z0_ports"], "75,100");
        assert_eq!(imported.get_signal("Rn").unwrap().data, [30.0; 2]);
        assert!((imported.get_signal("Fmin").unwrap().data[0] - 3.0).abs() < 1e-14);
    }

    #[test]
    fn a_port_table_shorter_than_the_solved_matrix_refuses_the_export() {
        let error = SimulationController::build_touchstone_dataset(
            &matrix_result(3),
            50.0,
            // Two entries, which is what the S-parameter form holds by
            // default, against a three-port design.
            &[50.0, 50.0],
            2,
        )
        .expect_err("a two-entry table cannot describe a three-port matrix");

        assert_eq!(error, "expected 3 per-port reference values, got 2");
    }

    /// A legacy result may use an explicitly supplied complete reference table.
    #[test]
    fn a_port_table_that_fits_supplies_the_exported_reference_impedances() {
        let dataset = SimulationController::build_touchstone_dataset(
            &matrix_result(2),
            50.0,
            &[50.0, 75.0],
            2,
        )
        .expect("a two-entry table describes a two-port matrix");

        assert_eq!(
            dataset.metadata.get("z0_ports").map(String::as_str),
            Some("50,75")
        );
    }

    #[test]
    fn solved_references_override_stale_configuration_including_single_port_exports() {
        for count in [1, 3] {
            let mut result = matrix_result(count);
            let references = (0..count)
                .map(|index| 75.0 + 25.0 * index as f64)
                .collect::<Vec<_>>();
            let crate::simulation::SimulationResult::Ac {
                reference_impedances_ohm,
                ..
            } = &mut result
            else {
                unreachable!()
            };
            *reference_impedances_ohm = Some(references.clone());
            let dataset =
                SimulationController::build_touchstone_dataset(&result, 50.0, &[50.0, 50.0], 2)
                    .unwrap();
            assert_eq!(dataset.metadata["num_ports"], count.to_string());
            assert_eq!(
                dataset.metadata["z0_ports"],
                references
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            );
            WaveformWriter::new(WaveformFormat::Touchstone)
                .write_text(&dataset)
                .expect("the retained port matrix exports");
        }
    }
}

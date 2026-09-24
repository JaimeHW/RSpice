//! Export every displayed noise analysis without dropping typed measurements.

use super::{PreparedTypedResultCsv, prepare_typed_result_csv};
use crate::state::{AnalysisResultPayload, SimulationRun};
use rspice_formats::table::CsvTableMerger;

pub(super) fn prepare(
    run: &SimulationRun,
    indices: &[usize],
) -> Result<PreparedTypedResultCsv, String> {
    let mut merger = CsvTableMerger::new();
    for &index in indices {
        let analysis = run
            .analyses
            .get(index)
            .ok_or("Noise analysis is no longer retained")?;
        analysis.validate_retained_evidence()?;
        let contents = if matches!(
            analysis.result_payload,
            Some(AnalysisResultPayload::Qpnoise { .. })
        ) {
            prepare_typed_result_csv(analysis)
                .ok_or("QPNOISE evidence cannot be exported")?
                .contents
        } else {
            crate::workbench::documents::result_document::export_noise_contribution_csv(
                run,
                &[index],
            )
            .ok_or("Noise spectrum cannot be exported")?
            .contents
        };
        merger.push(analysis.id.to_string(), &analysis.label, &contents)?;
    }
    let (contents, count) = merger.finish()?;
    Ok(PreparedTypedResultCsv {
        default_name: "rspice-noise-results.csv",
        contents,
        detail: format!(
            "{count} noise spectrum and evidence rows across {} analyses",
            indices.len()
        ),
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn qpnoise_result_noise_sheet_export_preserves_all_displayed_analyses() {
        use crate::state::{AnalysisResult, AnalysisType, WaveformData};
        let qpnoise = crate::simulation::SimulationResult::qpnoise_retained_test_fixture();
        let mut state = crate::workbench::AppState::default();
        state.simulation.start_run().add_analysis(qpnoise);
        state.simulation.active_analysis_idx = Some(0);
        let mut ordinary = AnalysisResult::new(2, AnalysisType::Noise, "Ordinary noise");
        ordinary.success = true;
        ordinary.waveforms.push(
            WaveformData::new("onoise", vec![100.0, 300.0, 700.0], vec![1e-18; 3], "#fff")
                .with_unit("V²/Hz"),
        );
        state
            .simulation
            .active_run_mut()
            .unwrap()
            .add_analysis(ordinary);
        let dataset = state.simulation.active_run().unwrap().dataset_id;
        state.workbench.documents.activate(
            crate::workbench::state::WorkspaceDocumentId::ResultDataset(dataset),
        );
        state.ui.results.viewer = crate::workbench::ResultViewer::NoiseContrib;
        let displayed = crate::workbench::documents::result_document::view_context::resolve_displayed_result_view(&state).unwrap();
        assert_eq!(displayed.analysis_indices.len(), 2);
        let (export, _) = super::super::prepare_displayed_table(&state, &displayed)
            .unwrap()
            .unwrap();
        let mut reader = csv::Reader::from_reader(export.contents.as_bytes());
        let headers = reader.headers().unwrap().clone();
        let column = |name| headers.iter().position(|c| c == name).unwrap();
        let rows = reader.records().map(Result::unwrap).collect::<Vec<_>>();
        assert!(rows.iter().any(|r| &r[column("kind")] == "metadata"));
        assert!(
            rows.iter()
                .any(|r| &r[column("kind")] == "covariance" && &r[column("source")] != "total")
        );
        assert!(
            rows.iter()
                .any(|r| &r[column("status")] == "zero_input_transfer")
        );
        assert!(rows.iter().any(|r| &r[column("record")] == "spectrum"
            && &r[column("analysis_label")] == "Ordinary noise"));
        assert!(
            rows.iter()
                .all(|r| !r[column("analysis_sequence")].is_empty())
        );
    }
}

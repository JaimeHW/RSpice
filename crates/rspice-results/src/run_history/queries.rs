//! Exact retained operating-point lookup across newest-first run history.

use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::AnalysisResult;
use crate::analysis_type::AnalysisType;
use crate::operating_point::OpPreviousState;
use crate::result_digest::ResultDigestEncoding;
use crate::run::SimulationRun;
use crate::waveform::RetainedWaveform;
use rspice_app_types::product::ObjectRevision;

/// Newest complete accepted OP, optionally admitting earlier source revisions.
pub fn newest_retained_op_state<
    'a,
    A: AsRef<AnalysisResult<W>> + 'a,
    W: AsRef<RetainedWaveform> + 'a,
>(
    runs: impl Iterator<Item = &'a SimulationRun<A>>,
    project_revision: ObjectRevision,
    allow_changed_revision: bool,
) -> Option<OpPreviousState> {
    let analysis = retained_op_result(runs, project_revision, allow_changed_revision)?;
    let provenance = analysis.provenance()?;
    let AnalysisResultPayload::OperatingPoint {
        mna_node_names,
        mna_branch_names,
        mna_solution,
        effective_source_content_digest: Some(source_content_digest),
        ..
    } = analysis.result_payload.as_ref()?
    else {
        return None;
    };
    Some(OpPreviousState {
        source_content_digest: *source_content_digest,
        producer_snapshot_digest: provenance.prepared_snapshot_digest(),
        producer_result_digest: analysis
            .result_data_ref()
            .digest(ResultDigestEncoding::CURRENT),
        node_names: mna_node_names.clone(),
        branch_names: mna_branch_names.clone(),
        solution: mna_solution.clone(),
    })
}

/// Whether complete OP evidence is available without cloning or hashing it.
pub fn has_retained_op_state<
    'a,
    A: AsRef<AnalysisResult<W>> + 'a,
    W: AsRef<RetainedWaveform> + 'a,
>(
    runs: impl Iterator<Item = &'a SimulationRun<A>>,
    project_revision: ObjectRevision,
    allow_changed_revision: bool,
) -> bool {
    retained_op_result(runs, project_revision, allow_changed_revision).is_some()
}

fn retained_op_result<'a, A: AsRef<AnalysisResult<W>> + 'a, W: AsRef<RetainedWaveform> + 'a>(
    mut runs: impl Iterator<Item = &'a SimulationRun<A>>,
    project_revision: ObjectRevision,
    allow_changed_revision: bool,
) -> Option<&'a AnalysisResult<W>> {
    runs.find_map(|run| {
        let receipt = run.prepared_receipt()?;
        if !allow_changed_revision && receipt.project_revision() != project_revision {
            return None;
        }
        run.analyses
            .iter()
            .map(AsRef::as_ref)
            .rev()
            .find(|analysis| {
                if !analysis.success
                    || analysis.analysis_type != AnalysisType::DcOp
                    || analysis.provenance().is_none()
                {
                    return false;
                }
                let Some(AnalysisResultPayload::OperatingPoint {
                    mna_node_names,
                    mna_branch_names,
                    mna_solution,
                    effective_source_content_digest: Some(_),
                    ..
                }) = analysis.result_payload.as_ref()
                else {
                    return false;
                };
                !mna_solution.is_empty()
                    && mna_node_names.len().saturating_add(mna_branch_names.len())
                        == mna_solution.len()
            })
    })
}

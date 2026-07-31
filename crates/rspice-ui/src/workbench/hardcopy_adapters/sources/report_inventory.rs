//! Exact application-owned authority for linked report references.
//!
//! A report may retain links to immutable result documents, datasets, and
//! verification evidence. Publication is allowed only when every link resolves
//! uniquely to the same identity, revision, digest, and dataset bindings that
//! the report captured. This module deliberately contains no fallback to an
//! active viewer, latest run, or display cache.

use super::{
    ReportHardcopySource, ResolvedHardcopyDocument, RetainedHardcopySourceAvailability,
    resolve_report_source,
};
use crate::hardcopy::HardcopyScope;
use crate::hardcopy::sources::HardcopySourceError;
use crate::product::DatasetBinding;
use crate::results::report_document::{
    ReportDocument, ReportReferenceInventory, ReportReferenceInventoryEntry, ReportReferenceMode,
    ReportReferenceSnapshot, ReportSourceId,
};
use crate::workbench::AppState;

pub(super) fn availability(
    state: &AppState,
    document: &ReportDocument,
) -> RetainedHardcopySourceAvailability {
    if document.pages().is_empty() {
        return RetainedHardcopySourceAvailability::Unavailable {
            reason: "report has no authored pages".to_owned(),
        };
    }
    match resolve(state, document, HardcopyScope::CompleteReport) {
        Ok(_) => RetainedHardcopySourceAvailability::Available,
        Err(error) => RetainedHardcopySourceAvailability::Unavailable {
            reason: error.to_string(),
        },
    }
}

pub(super) fn resolve(
    state: &AppState,
    document: &ReportDocument,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let inventory = reference_inventory(state, document)?;
    resolve_report_source(ReportHardcopySource {
        source_key: format!(
            "project:{}:report:{}",
            state.workspace.project.id().as_uuid(),
            document.id()
        ),
        document,
        reference_inventory: Some(&inventory),
        scope,
    })
}

pub(super) fn reference_inventory(
    state: &AppState,
    document: &ReportDocument,
) -> Result<ReportReferenceInventory, HardcopySourceError> {
    let mut available_datasets = Vec::new();
    for run in &state.simulation.runs {
        let binding = DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
        if !available_datasets
            .iter()
            .any(|candidate: &DatasetBinding| candidate.dataset_id == binding.dataset_id)
        {
            available_datasets.push(binding);
        }
    }

    let mut sources = Vec::new();
    for reference in document
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
        .filter_map(|block| block.kind().reference())
        .filter(|reference| matches!(reference, ReportReferenceMode::Linked { .. }))
    {
        let captured = reference.snapshot();
        if sources
            .iter()
            .any(|candidate: &ReportReferenceInventoryEntry| candidate.source == captured.source)
        {
            continue;
        }
        let entry = match &captured.source {
            ReportSourceId::VisualizationDocument { document_id } => {
                let matches = state
                    .workspace
                    .visualization_documents
                    .iter()
                    .filter(|document| document.id() == *document_id)
                    .collect::<Vec<_>>();
                let [source_document] = matches.as_slice() else {
                    return Err(HardcopySourceError::InvalidReportSource(
                        if matches.is_empty() {
                            format!("linked visualization document {document_id} is not retained")
                        } else {
                            format!(
                                "linked visualization document {document_id} resolves ambiguously"
                            )
                        },
                    ));
                };
                let current_digest = source_document.content_digest().map_err(|error| {
                    HardcopySourceError::InvalidVisualizationSource(error.to_string())
                })?;
                let current = ReportReferenceSnapshot::new(
                    captured.source.clone(),
                    Some(source_document.revision()),
                    current_digest,
                    source_document
                        .datasets()
                        .iter()
                        .map(|dataset| dataset.binding())
                        .collect(),
                )
                .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
                ReportReferenceInventoryEntry::new(
                    current.source,
                    current.source_revision,
                    current.content_digest,
                    current.dataset_bindings,
                )
            }
            ReportSourceId::Dataset { dataset_id } => {
                let run = unique_run_for_dataset(state, *dataset_id, "linked report dataset")?;
                let binding = DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
                ReportReferenceInventoryEntry::new(
                    captured.source.clone(),
                    None,
                    binding.content_digest,
                    vec![binding],
                )
            }
            ReportSourceId::VerificationEvidence { .. } => {
                if captured.dataset_bindings.len() != 1 {
                    return Err(HardcopySourceError::InvalidReportSource(
                        "linked verification evidence must name exactly one retained dataset"
                            .to_owned(),
                    ));
                }
                let run = unique_run_for_dataset(
                    state,
                    captured.dataset_bindings[0].dataset_id,
                    "verification evidence dataset",
                )?;
                let binding = DatasetBinding::new(run.dataset_id, run.dataset_content_digest());
                ReportReferenceInventoryEntry::new(
                    captured.source.clone(),
                    None,
                    binding.content_digest,
                    vec![binding],
                )
            }
            ReportSourceId::ExternalRecord { namespace, key } => {
                return Err(HardcopySourceError::InvalidReportSource(format!(
                    "external report source {namespace}:{key} has no retained application authority"
                )));
            }
        }
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
        sources.push(entry);
    }

    let inventory = ReportReferenceInventory {
        sources,
        available_datasets,
    };
    inventory
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    Ok(inventory)
}

fn unique_run_for_dataset<'a>(
    state: &'a AppState,
    dataset_id: crate::product::DatasetId,
    label: &str,
) -> Result<&'a crate::state::SimulationRun, HardcopySourceError> {
    let mut matching = state
        .simulation
        .runs
        .iter()
        .filter(|run| run.dataset_id == dataset_id);
    let Some(run) = matching.next() else {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "{label} {dataset_id}"
        )));
    };
    if matching.next().is_some() {
        return Err(HardcopySourceError::AmbiguousRetainedDataset(
            dataset_id.to_string(),
        ));
    }
    Ok(run)
}

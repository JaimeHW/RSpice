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
    PlotFigureBlock, ReportBlockId, ReportBlockKind, ReportDocument, ReportReferenceFigureArtifact,
    ReportReferenceInventory, ReportReferenceInventoryEntry, ReportReferenceMode,
    ReportReferenceSnapshot, ReportSourceId,
};
use crate::results::visualization_raster::{
    VisualizationRasterProfile, render_visualization_report_figure,
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
        retain_available_dataset(&mut available_datasets, binding)?;
    }

    let mut sources = Vec::new();
    let mut figure_artifacts = Vec::new();
    for block in document
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
    {
        let Some(reference) = block.kind().reference() else {
            continue;
        };
        if !matches!(reference, ReportReferenceMode::Linked { .. }) {
            continue;
        }
        let captured = reference.snapshot();
        if !sources
            .iter()
            .any(|candidate: &ReportReferenceInventoryEntry| candidate.source == captured.source)
        {
            let entry = resolve_inventory_entry(state, captured)?;
            for binding in &entry.dataset_bindings {
                retain_available_dataset(&mut available_datasets, *binding)?;
            }
            sources.push(entry);
        }
        if let ReportBlockKind::PlotFigure(figure) = block.kind() {
            figure_artifacts.push(resolve_linked_figure_artifact(
                state,
                block.id(),
                figure,
                captured,
            )?);
        }
    }

    let inventory = ReportReferenceInventory {
        sources,
        available_datasets,
        figure_artifacts,
    };
    inventory
        .validate()
        .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))?;
    Ok(inventory)
}

fn retain_available_dataset(
    available: &mut Vec<DatasetBinding>,
    binding: DatasetBinding,
) -> Result<(), HardcopySourceError> {
    if let Some(existing) = available
        .iter()
        .find(|candidate| candidate.dataset_id == binding.dataset_id)
    {
        if existing.content_digest != binding.content_digest {
            return Err(HardcopySourceError::InvalidReportSource(format!(
                "dataset {} resolves to conflicting content digests across retained authorities",
                binding.dataset_id
            )));
        }
        return Ok(());
    }
    available.push(binding);
    Ok(())
}

fn resolve_inventory_entry(
    state: &AppState,
    captured: &ReportReferenceSnapshot,
) -> Result<ReportReferenceInventoryEntry, HardcopySourceError> {
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
                        format!("linked visualization document {document_id} resolves ambiguously")
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
    Ok(entry)
}

fn resolve_linked_figure_artifact(
    state: &AppState,
    block_id: ReportBlockId,
    figure: &PlotFigureBlock,
    captured: &ReportReferenceSnapshot,
) -> Result<ReportReferenceFigureArtifact, HardcopySourceError> {
    let ReportSourceId::VisualizationDocument { document_id } = &captured.source else {
        return Err(HardcopySourceError::InvalidReportSource(format!(
            "linked plot block {block_id} is not bound to a visualization document"
        )));
    };
    let locator = figure.source_locator.clone().ok_or_else(|| {
        HardcopySourceError::InvalidReportSource(format!(
            "linked plot block {block_id} has no exact page and pane locator"
        ))
    })?;
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
                format!("linked visualization document {document_id} resolves ambiguously")
            },
        ));
    };
    let page = source_document
        .pages()
        .iter()
        .find(|page| page.id.get() == locator.page_id)
        .ok_or_else(|| {
            HardcopySourceError::InvalidReportSource(format!(
                "linked plot block {block_id} page {} is not retained",
                locator.page_id
            ))
        })?;
    let pane = source_document
        .panes()
        .iter()
        .find(|pane| pane.id.get() == locator.pane_id && pane.page_id == page.id)
        .ok_or_else(|| {
            HardcopySourceError::InvalidReportSource(format!(
                "linked plot block {block_id} pane {} is not retained on page {}",
                locator.pane_id, locator.page_id
            ))
        })?;
    let raster = render_visualization_report_figure(
        source_document,
        captured,
        page.id,
        pane.id,
        &VisualizationRasterProfile::default(),
    )
    .map_err(|error| HardcopySourceError::InvalidVisualizationSource(error.to_string()))?;
    ReportReferenceFigureArtifact::new(
        block_id,
        captured.clone(),
        locator,
        raster.artifact().clone(),
    )
    .map_err(|error| HardcopySourceError::InvalidReportSource(error.to_string()))
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

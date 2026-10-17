//! Slice, pivot, encode and filter a sweep, corner or Monte Carlo family.
//!
//! Over a project result document the policy is kept with the selected pane
//! and the renderer applies it whenever that pane draws the waveform view.
//! Over a quick view it is kept for the session, per analysis.

use std::collections::HashSet;

use rspice_results::family_projection::{FamilyManifest, FamilyValueKind, SourceSampleSelection};
use rspice_results_ui::document_dialogs::family::{FamilyChoices, FamilyDraft};

use super::*;
use crate::results::visualization_document::{
    AccessibleColorPalette, ColumnRole, FamilyAggregationMethod, FamilyAggregationPolicy,
    FamilyDimension as DocumentFamilyDimension, FamilyEncodingMap, FamilyPresentationPolicy,
    FamilyXDimension, FamilyXOrdering, MissingPointPolicy, Pane, ValueType,
};

const NO_FAMILY: &str = "The result holds no sweep, corner or Monte Carlo family.";

/// Where the reader's family policy is kept.
#[derive(Clone, Copy)]
enum Scope {
    Pane(ResultDocumentId, PaneId),
    QuickView(super::super::AnalysisPresentationKey),
}

fn manifest(analysis: &AnalysisResult) -> Result<FamilyManifest, String> {
    FamilyManifest::from_metadata(analysis.analysis_type, analysis.family_metadata.as_ref())?
        .ok_or_else(|| NO_FAMILY.to_owned())
}

/// The scope a policy is kept in, with the result it slices.
fn scope(state: &AppState) -> Result<(Scope, &SimulationRun, &AnalysisResult), String> {
    let (run, analysis) = reading_analysis(state)?;
    let scope = match active_document_id(state) {
        Some(document_id) => {
            let (_, pane_id) =
                super::super::persistent_document::reading_position(state, document_id)
                    .ok_or_else(|| "The result document has no pane.".to_owned())?;
            Scope::Pane(document_id, pane_id)
        }
        None => Scope::QuickView(super::super::AnalysisPresentationKey::new(
            run.dataset_id,
            analysis,
        )),
    };
    Ok((scope, run, analysis))
}

fn current_policy(state: &AppState, scope: Scope) -> Option<FamilyPresentationPolicy> {
    match scope {
        Scope::Pane(document_id, pane_id) => state
            .workspace
            .content
            .visualization_document(document_id)?
            .panes()
            .iter()
            .find(|pane| pane.id == pane_id)?
            .family_policy
            .clone(),
        Scope::QuickView(key) => state.ui.results.session.family_policies.get(&key).cloned(),
    }
}

/// Whether the result the reader is looking at holds a family to slice.
pub(crate) fn family_available(state: &AppState) -> bool {
    reading_analysis(state).is_ok_and(|(_, analysis)| manifest(analysis).is_ok())
}

/// Open Family slicing on the result the reader is looking at, seeded with
/// the policy already in force or, without one, the family's defaults.
pub(crate) fn open_family(state: &mut AppState) {
    let draft = scope(state).and_then(|(scope, _, analysis)| {
        let manifest = manifest(analysis)?;
        Ok(initial_draft(
            &manifest,
            current_policy(state, scope).as_ref(),
        ))
    });
    match draft {
        Ok(draft) => {
            state.workbench.result_document_dialog = Some(ResultDocumentDialog::Family(draft));
        }
        Err(refusal) => state.prompt(refusal),
    }
}

/// Narrow the open Family slicing form to `filter`, as a hop that carries a
/// point into the family does.
pub(crate) fn set_family_filter(state: &mut AppState, filter: String) {
    if let Some(ResultDocumentDialog::Family(draft)) = &mut state.workbench.result_document_dialog {
        draft.filter = filter;
    }
}

/// Resolve the form's choices against the family's manifest.
pub(super) fn choices(state: &AppState, draft: &FamilyDraft) -> FamilyChoices {
    let refused = |refusal: String| FamilyChoices {
        scope: String::new(),
        x_dimensions: Vec::new(),
        dimensions: Vec::new(),
        points: Ok(String::new()),
        refusal: Some(refusal),
    };
    let (scope, run, analysis) = match scope(state) {
        Ok(found) => found,
        Err(refusal) => return refused(refusal),
    };
    let manifest = match manifest(analysis) {
        Ok(manifest) => manifest,
        Err(refusal) => return refused(refusal),
    };
    let label = |dimension: &rspice_results::family_projection::FamilyDimension| {
        dimension.unit.as_ref().map_or_else(
            || dimension.label.clone(),
            |unit| format!("{} ({unit})", dimension.label),
        )
    };
    let dimensions = manifest
        .dimensions
        .iter()
        .filter(|dimension| dimension.id != "status")
        .collect::<Vec<_>>();
    let x_dimensions = dimensions
        .iter()
        .filter(|dimension| {
            matches!(
                dimension.kind,
                FamilyValueKind::Number | FamilyValueKind::Integer
            )
        })
        .map(|dimension| (dimension.id.clone(), label(dimension)))
        .collect();
    let points = manifest
        .matching_source_indices(&draft.filter)
        .map(|indices| {
            let mut points = format!("{} of {} points", indices.len(), manifest.points.len());
            if manifest.omitted_points > 0 {
                points.push_str(&format!(" · {} not run", manifest.omitted_points));
            }
            points
        });
    let refusal = if state.ui.results.session.viewer == ResultViewer::Waves {
        policy(&manifest, draft)
            .and_then(|policy| {
                selection(run, analysis, &manifest, &policy)?;
                match scope {
                    Scope::Pane(document_id, pane_id) => {
                        undeclared_dimension(state, document_id, pane_id, &policy, &manifest)
                    }
                    Scope::QuickView(_) => Ok(()),
                }
            })
            .err()
    } else {
        Some("Family slicing applies to the waveform view.".to_owned())
    };
    let scope = match scope {
        Scope::Pane(document_id, pane_id) => state
            .workspace
            .content
            .visualization_document(document_id)
            .and_then(|document| {
                let pane = document.panes().iter().find(|pane| pane.id == pane_id)?;
                Some(format!("{} · {}", document.title(), pane.title))
            })
            .unwrap_or_default(),
        Scope::QuickView(_) => format!("Run {} · {} · {}", run.id, run.label, analysis.label),
    };
    FamilyChoices {
        scope,
        x_dimensions,
        dimensions: dimensions
            .iter()
            .map(|dimension| (dimension.id.clone(), label(dimension)))
            .collect(),
        points,
        refusal,
    }
}

/// Keep the draft's policy with the selected pane, or with the analysis for
/// the session over a quick view. The form closes once it lands.
pub(super) fn commit(
    state: &mut AppState,
    draft: &FamilyDraft,
) -> Result<Option<ResultDocumentDialog>, String> {
    let (scope, policy) = {
        let (scope, run, analysis) = scope(state)?;
        let manifest = manifest(analysis)?;
        let policy = policy(&manifest, draft)?;
        selection(run, analysis, &manifest, &policy)?;
        (scope, policy)
    };
    match scope {
        Scope::Pane(document_id, pane_id) => {
            transact(
                state,
                document_id,
                vec![DocumentEdit::SetPaneFamilyPresentation {
                    pane_id,
                    policy: Some(policy),
                }],
            )?;
        }
        Scope::QuickView(key) => {
            state.ui.results.session.family_policies.insert(key, policy);
        }
    }
    Ok(None)
}

/// The rows a pane's kept family policy shows. Only the waveform view draws
/// a family slice, so any other sheet shows every row.
pub(in super::super) fn pane_selection(
    state: &AppState,
    pane: &Pane,
    viewer: ResultViewer,
) -> Result<Option<SourceSampleSelection>, String> {
    let Some(policy) = &pane.family_policy else {
        return Ok(None);
    };
    if viewer != ResultViewer::Waves {
        return Ok(None);
    }
    let binding = pane
        .binding
        .ok_or_else(|| "The pane shows no dataset to slice.".to_owned())?;
    let (run, analysis) = super::super::persistent_document::resolve_binding(state, binding)?;
    let run = &state.simulation.retained.runs[run];
    let analysis = &run.analyses[analysis];
    selection(run, analysis, &manifest(analysis)?, policy).map(Some)
}

/// The rows the quick view's family policy shows for the active analysis.
/// A policy the retained data no longer satisfies shows every row.
pub(in super::super) fn quick_view_selection(state: &AppState) -> Option<SourceSampleSelection> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let key = super::super::AnalysisPresentationKey::new(run.dataset_id, analysis);
    let policy = state.ui.results.session.family_policies.get(&key)?;
    selection(run, analysis, &manifest(analysis).ok()?, policy).ok()
}

/// The exact rows `policy` keeps, grouped and styled as it says.
fn selection(
    run: &SimulationRun,
    analysis: &AnalysisResult,
    manifest: &FamilyManifest,
    policy: &FamilyPresentationPolicy,
) -> Result<SourceSampleSelection, String> {
    for waveform in &analysis.waveforms {
        manifest.compatible_waveform_len(waveform.x.len())?;
        if waveform.x.len() != waveform.y.len() {
            return Err(format!(
                "Waveform '{}' has mismatched X and Y sample counts.",
                waveform.name
            ));
        }
    }
    let indices = manifest.matching_source_indices_for_filter(policy.filter.as_ref())?;
    if indices.is_empty() {
        return Err("The filter keeps no points.".to_owned());
    }
    SourceSampleSelection::new(run.dataset_id, analysis.id, indices)?
        .with_family_presentation(manifest, policy)
}

/// A document keeps a family policy only over dimensions its copy of the
/// result declares, so a slice it cannot keep is refused before Apply.
fn undeclared_dimension(
    state: &AppState,
    document_id: ResultDocumentId,
    pane_id: PaneId,
    policy: &FamilyPresentationPolicy,
    manifest: &FamilyManifest,
) -> Result<(), String> {
    let document = project_document(state, document_id)?;
    let binding = document
        .panes()
        .iter()
        .find(|pane| pane.id == pane_id)
        .and_then(|pane| pane.binding)
        .ok_or_else(|| "The selected pane shows no dataset.".to_owned())?;
    let dataset = document
        .datasets()
        .iter()
        .find(|dataset| dataset.binding() == binding.dataset)
        .ok_or_else(|| "The document no longer holds the pane's dataset.".to_owned())?;
    for dimension in std::iter::once(&policy.x_dimension.dimension).chain(&policy.family_dimensions)
    {
        let declared = dataset.columns().iter().any(|column| {
            column.key() == dimension.key
                && column.role() == ColumnRole::Coordinate
                && column.value_type() == dimension.value_type
        });
        if !declared {
            let name = manifest
                .dimension(&dimension.key)
                .map_or(dimension.key.as_str(), |found| found.label.as_str());
            return Err(format!(
                "The document keeps no {name} values for this pane to slice by."
            ));
        }
    }
    Ok(())
}

/// The draft a family opens with: the policy in force, or the family's
/// defaults.
fn initial_draft(
    manifest: &FamilyManifest,
    policy: Option<&FamilyPresentationPolicy>,
) -> FamilyDraft {
    let preferred_x = manifest
        .dimensions
        .iter()
        .find(|dimension| {
            dimension.kind == FamilyValueKind::Number
                && !matches!(dimension.id.as_str(), "temperature" | "sample")
        })
        .or_else(|| {
            manifest.dimensions.iter().find(|dimension| {
                matches!(
                    dimension.kind,
                    FamilyValueKind::Number | FamilyValueKind::Integer
                ) && dimension.id != "temperature"
            })
        })
        .map_or_else(|| "sample".to_owned(), |dimension| dimension.id.clone());
    let x = policy.map_or(preferred_x, |policy| {
        policy.x_dimension.dimension.key.clone()
    });
    let preferred_family = manifest
        .dimension("process")
        .or_else(|| {
            manifest.dimensions.iter().find(|dimension| {
                dimension.id != x && !matches!(dimension.id.as_str(), "sample" | "status")
            })
        })
        .or_else(|| manifest.dimension("sample"))
        .map(|dimension| dimension.id.clone())
        .unwrap_or_default();
    let family = policy
        .and_then(|policy| policy.family_dimensions.first())
        .map_or(preferred_family, |dimension| dimension.key.clone());
    let encoding = |predicate: fn(&FamilyEncodingMap) -> bool| {
        policy.and_then(|policy| {
            policy
                .encodings
                .iter()
                .find(|encoding| predicate(encoding))
                .map(|encoding| encoding.dimension().key.clone())
        })
    };
    let color = encoding(|encoding| matches!(encoding, FamilyEncodingMap::Color { .. }))
        .unwrap_or_else(|| family.clone());
    let dash = encoding(|encoding| matches!(encoding, FamilyEncodingMap::Dash { .. }))
        .or_else(|| {
            manifest
                .dimension("temperature")
                .map(|dimension| dimension.id.clone())
        })
        .unwrap_or_default();
    let marker = encoding(|encoding| matches!(encoding, FamilyEncodingMap::Marker { .. }))
        .unwrap_or_else(|| color.clone());
    let filter = policy
        .and_then(|policy| policy.filter.as_ref())
        .map(|filter| filter.source.clone())
        .unwrap_or_else(|| {
            if manifest.dimension("temperature").is_some() {
                "temperature in {27,125} and status != not-run".to_owned()
            } else {
                "status != not-run".to_owned()
            }
        });
    FamilyDraft {
        x,
        family,
        color,
        dash,
        marker,
        filter,
        omit_missing: policy.is_some_and(|policy| {
            policy.missing_points == MissingPointPolicy::ExcludeWithOmissionRecord
        }),
    }
}

fn document_dimension(
    manifest: &FamilyManifest,
    id: &str,
) -> Result<DocumentFamilyDimension, String> {
    let dimension = manifest
        .dimension(id)
        .ok_or_else(|| format!("Unknown family dimension '{id}'."))?;
    DocumentFamilyDimension::new(
        dimension.id.clone(),
        match dimension.kind {
            FamilyValueKind::Number => ValueType::Real,
            FamilyValueKind::Integer => ValueType::Integer,
            FamilyValueKind::Text | FamilyValueKind::Status => ValueType::Text,
        },
    )
    .map_err(|error| error.to_string())
}

/// The policy the draft describes. Every family dimension is told apart by
/// an encoding: one left without a visual cue is labelled, and two are
/// refused.
fn policy(
    manifest: &FamilyManifest,
    draft: &FamilyDraft,
) -> Result<FamilyPresentationPolicy, String> {
    let x_dimension = document_dimension(manifest, &draft.x)?;
    let mut family_dimension_ids = vec![draft.family.clone()];
    for dimension in [&draft.color, &draft.dash, &draft.marker] {
        if !dimension.is_empty()
            && *dimension != x_dimension.key
            && !family_dimension_ids.contains(dimension)
        {
            family_dimension_ids.push(dimension.clone());
        }
    }
    if draft.filter.contains("status")
        && x_dimension.key != "status"
        && !family_dimension_ids.iter().any(|id| id == "status")
    {
        family_dimension_ids.push("status".to_owned());
    }
    family_dimension_ids.retain(|id| !id.is_empty() && *id != x_dimension.key);
    let family_dimensions = family_dimension_ids
        .iter()
        .map(|id| document_dimension(manifest, id))
        .collect::<Result<Vec<_>, _>>()?;
    if family_dimensions.is_empty() {
        return Err("Choose a family dimension other than the X axis.".to_owned());
    }

    let mut encodings = Vec::new();
    if !draft.color.is_empty() {
        encodings.push(FamilyEncodingMap::Color {
            dimension: document_dimension(manifest, &draft.color)?,
            palette: AccessibleColorPalette::OkabeItoCategorical,
        });
    }
    if !draft.dash.is_empty() {
        encodings.push(FamilyEncodingMap::Dash {
            dimension: document_dimension(manifest, &draft.dash)?,
        });
    }
    if !draft.marker.is_empty() {
        encodings.push(FamilyEncodingMap::Marker {
            dimension: document_dimension(manifest, &draft.marker)?,
        });
    }
    let encoded: HashSet<_> = encodings
        .iter()
        .map(|encoding| encoding.dimension().key.as_str())
        .collect();
    let unencoded: Vec<_> = family_dimensions
        .iter()
        .filter(|dimension| !encoded.contains(dimension.key.as_str()))
        .cloned()
        .collect();
    if unencoded.len() > 1 {
        return Err(format!(
            "Give {} a color, dash or marker each.",
            unencoded
                .iter()
                .map(|dimension| dimension.key.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if let Some(dimension) = unencoded.into_iter().next() {
        encodings.push(FamilyEncodingMap::Label {
            dimension,
            prefix: None,
        });
    }
    let color_dimensions: Vec<_> = encodings
        .iter()
        .filter_map(|encoding| match encoding {
            FamilyEncodingMap::Color { dimension, .. } => Some(dimension.clone()),
            _ => None,
        })
        .collect();
    for color_dimension in color_dimensions {
        if !encodings.iter().any(|encoding| {
            encoding.dimension() == &color_dimension
                && matches!(
                    encoding,
                    FamilyEncodingMap::Dash { .. }
                        | FamilyEncodingMap::Marker { .. }
                        | FamilyEncodingMap::Label { .. }
                )
        }) {
            if encodings
                .iter()
                .any(|encoding| matches!(encoding, FamilyEncodingMap::Label { .. }))
            {
                return Err(format!(
                    "Give {} a dash or marker as well as its color.",
                    color_dimension.key
                ));
            }
            encodings.push(FamilyEncodingMap::Label {
                dimension: color_dimension,
                prefix: None,
            });
        }
    }

    let policy = FamilyPresentationPolicy {
        x_dimension: FamilyXDimension {
            dimension: x_dimension,
            ordering: FamilyXOrdering::Source,
        },
        family_dimensions,
        facet_layout: None,
        aggregation: FamilyAggregationPolicy {
            method: FamilyAggregationMethod::None,
            over_dimensions: Vec::new(),
        },
        filter: manifest.compile_filter(&draft.filter)?,
        missing_points: if draft.omit_missing {
            MissingPointPolicy::ExcludeWithOmissionRecord
        } else {
            MissingPointPolicy::PreserveAsNotRun
        },
        encodings,
    };
    policy.validate().map_err(|error| error.to_string())?;
    Ok(policy)
}

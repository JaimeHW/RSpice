//! Semantic Studio publication over borrowed presentation and retained runs.

use super::*;
use rspice_app_types::product::{DatasetId, ObjectRevision, ProjectId, RunId};
use rspice_hardcopy_contract::sources::MAX_HARDCOPY_SOURCE_SET_MEMBERS;
use rspice_results::{
    analysis_result::AnalysisResult,
    harmonic_spectrum, phase_noise,
    run::SimulationRun,
    studio_presentation::{
        VisualizationAnnotation as StudioAnnotation, VisualizationAutoscale,
        VisualizationMarker as StudioMarker, VisualizationPane as StudioPane,
    },
    visualization_document::FamilyPresentationPolicy,
    waveform::RetainedWaveform,
};
use serde::Serialize;
use std::collections::BTreeMap;
use uuid::Uuid;

/// Authored presentation needed by hardcopy, borrowed without editor caches or gestures.
#[derive(Clone, Copy)]
pub struct StudioHardcopyPresentation<'a> {
    pub revision: u64,
    pub panes: &'a [StudioPane],
    pub markers: &'a [StudioMarker],
    pub annotations: &'a [StudioAnnotation],
    pub pane_x_ranges: &'a BTreeMap<u64, (f64, f64)>,
    pub family_policies: &'a BTreeMap<u64, FamilyPresentationPolicy>,
    pub autoscale: VisualizationAutoscale,
}

impl<'a> From<&'a rspice_results::studio_presentation::VisualizationStudioPresentation>
    for StudioHardcopyPresentation<'a>
{
    fn from(
        studio: &'a rspice_results::studio_presentation::VisualizationStudioPresentation,
    ) -> Self {
        Self {
            revision: studio.revision,
            panes: &studio.panes,
            markers: &studio.markers,
            annotations: &studio.annotations,
            pane_x_ranges: &studio.pane_x_ranges,
            family_policies: &studio.family_policies,
            autoscale: studio.autoscale,
        }
    }
}

/// Waveform presentation participates in the existing Studio content identity.
pub struct StudioWaveformStyle<'a> {
    pub color: &'a str,
    pub visible: bool,
}

/// Retained data and presentation inputs; resolution grants no publication permission.
pub struct StudioHardcopySource<'a, R, W> {
    pub project_id: ProjectId,
    pub studio: StudioHardcopyPresentation<'a>,
    pub runs: &'a [R],
    pub waveform_style: fn(&W) -> StudioWaveformStyle<'_>,
}

/// Whether Studio can render this curve viewer from its retained waveforms.
pub const fn studio_curve_viewer_is_supported(viewer: ResultViewer) -> bool {
    matches!(
        viewer,
        ResultViewer::Waves
            | ResultViewer::DcSweep
            | ResultViewer::NoiseContrib
            | ResultViewer::HarmonicBalance
            | ResultViewer::PhaseNoise
    )
}

pub fn resolve_studio_document<R, A, W>(
    source: &StudioHardcopySource<'_, R, W>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError>
where
    R: AsRef<SimulationRun<A>>,
    A: AsRef<AnalysisResult<W>>,
    W: AsRef<RetainedWaveform>,
{
    let project_id = source.project_id;
    let studio = &source.studio;
    if studio.panes.is_empty() || studio.panes.len() > MAX_HARDCOPY_SOURCE_SET_MEMBERS {
        return Err(HardcopySourceError::InvalidSourceSet(
            "all-panes scope requires a bounded non-empty retained pane set".to_owned(),
        ));
    }
    let mut resolved_panes = Vec::with_capacity(studio.panes.len());
    for pane in studio.panes {
        resolved_panes.push(resolve_studio_pane(
            source,
            format!(
                "project:{}:visualization-pane:{}",
                project_id.as_uuid(),
                pane.id
            ),
            pane.id,
            HardcopyScope::ActivePlotDocument,
        )?);
    }
    let members = resolved_panes
        .iter()
        .map(source_set_member_from_resolved)
        .collect::<Result<Vec<_>, _>>()?;
    let source_set = HardcopySourceSet::try_new(
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(
            &project_id.as_uuid(),
            b"rspice-hardcopy-all-visualization-panes-v1",
        ))
        .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(studio.revision)
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        "All visualization panes",
        HardcopyDocumentKind::PlotOrWorksheet,
        HardcopyScope::AllSheetsOrPanes,
        members,
    )?;
    let mut resolved_panes = resolved_panes.into_iter();
    resolve_hardcopy_source_set_with(&source_set, |expected| {
        let actual = resolved_panes.next().ok_or_else(|| {
            HardcopySourceError::SourceNotRetained(expected.source_key().to_owned())
        })?;
        if actual.source_key() != expected.source_key() {
            return Err(HardcopySourceError::StaleSourceSetMember {
                source_key: expected.source_key().to_owned(),
            });
        }
        Ok(actual)
    })
}

/// Resolve the selected Studio pane from its exact retained dataset and analysis.
pub fn resolve_studio_pane<R, A, W>(
    source: &StudioHardcopySource<'_, R, W>,
    source_key: String,
    pane_id: u64,
    scope: HardcopyScope,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError>
where
    R: AsRef<SimulationRun<A>>,
    A: AsRef<AnalysisResult<W>>,
    W: AsRef<RetainedWaveform>,
{
    validate_label("source key", &source_key, SOURCE_KEY_LIMIT)?;
    if !matches!(
        &scope,
        HardcopyScope::ActivePlotDocument | HardcopyScope::ActiveDocument
    ) {
        return Err(HardcopySourceError::UnsupportedScope(scope));
    }
    let panes = source
        .studio
        .panes
        .iter()
        .filter(|pane| pane.id == pane_id)
        .collect::<Vec<_>>();
    let [pane] = panes.as_slice() else {
        return if panes.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "active pane {pane_id} is not retained"
            )))
        } else {
            Err(HardcopySourceError::AmbiguousActiveSource(format!(
                "visualization pane {pane_id}"
            )))
        };
    };
    if is_curve_viewer(pane.viewer) && !studio_curve_viewer_is_supported(pane.viewer) {
        return Err(HardcopySourceError::UnsupportedVisualizationViewer(
            format!(
                "{} has no faithful semantic Studio figure writer",
                pane.viewer.label(),
            ),
        ));
    }
    if is_curve_viewer(pane.viewer) && source.studio.family_policies.contains_key(&pane.id) {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "active family presentation requires its exact resolved family slice".to_owned(),
        ));
    }
    if is_curve_viewer(pane.viewer)
        && source.studio.autoscale == VisualizationAutoscale::SpecificationBounds
    {
        return Err(HardcopySourceError::InvalidVisualizationSource(
            "specification-bound autoscale requires the active project specification authority"
                .to_owned(),
        ));
    }

    let runs = source
        .runs
        .iter()
        .map(AsRef::as_ref)
        .filter(|run| run.dataset_id == pane.dataset_id)
        .collect::<Vec<_>>();
    let [run] = runs.as_slice() else {
        return if runs.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "dataset {} is not retained",
                pane.dataset_id
            )))
        } else {
            Err(HardcopySourceError::AmbiguousRetainedDataset(
                pane.dataset_id.to_string(),
            ))
        };
    };
    if !run.lifecycle.is_terminal() {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "dataset {} belongs to a non-terminal run",
            pane.dataset_id
        )));
    }
    let analyses = run
        .analyses
        .iter()
        .map(AsRef::as_ref)
        .filter(|analysis| analysis.id == pane.analysis_sequence)
        .collect::<Vec<_>>();
    let [analysis] = analyses.as_slice() else {
        return if analyses.is_empty() {
            Err(HardcopySourceError::UnretainedResult(format!(
                "analysis {} is not retained in dataset {}",
                pane.analysis_sequence, pane.dataset_id
            )))
        } else {
            Err(HardcopySourceError::AmbiguousRetainedAnalysis(
                pane.analysis_sequence,
            ))
        };
    };
    if !analysis.success {
        return Err(HardcopySourceError::UnretainedResult(format!(
            "analysis {} did not complete successfully",
            analysis.id
        )));
    }
    if !is_curve_viewer(pane.viewer) {
        return resolve_studio_result_summary(
            source,
            &source_key,
            scope,
            pane,
            run.run_id,
            analysis,
        );
    }
    analysis
        .validate_retained_evidence()
        .map_err(HardcopySourceError::InvalidVisualizationSource)?;
    let viewer_accepts_analysis = match pane.viewer {
        ResultViewer::HarmonicBalance => harmonic_spectrum::analysis_is_renderable(
            analysis.success,
            analysis.analysis_type,
            &analysis.waveforms,
            || {},
        ),
        ResultViewer::PhaseNoise => phase_noise::phase_noise_is_renderable(
            analysis.success,
            analysis.analysis_type,
            analysis.family_metadata.as_ref(),
            &analysis.waveforms,
            || {},
        ),
        _ => true,
    };
    if !viewer_accepts_analysis {
        return Err(HardcopySourceError::MissingViewerEvidence(
            match pane.viewer {
                ResultViewer::HarmonicBalance => "harmonic-balance spectrum",
                ResultViewer::PhaseNoise => "phase-noise spectrum",
                _ => unreachable!("only specialist curve viewers are validated here"),
            },
        ));
    }
    let visible = analysis
        .waveforms
        .iter()
        .filter(|waveform| {
            let style = (source.waveform_style)(waveform);
            let waveform = (*waveform).as_ref();
            style.visible
                && match pane.viewer {
                    ResultViewer::HarmonicBalance => {
                        harmonic_spectrum::spectrum_trace_is_renderable(waveform, || {})
                    }
                    ResultViewer::PhaseNoise => {
                        phase_noise::phase_noise_waveform_is_renderable(waveform, || {})
                    }
                    _ => true,
                }
        })
        .collect::<Vec<_>>();
    if visible.is_empty() {
        return Err(HardcopySourceError::UnretainedResult(
            "the active pane has no visible retained waveform".to_owned(),
        ));
    }
    for waveform in &visible {
        let waveform = (*waveform).as_ref();
        if waveform.x.is_empty()
            || waveform.x.len() != waveform.y.len()
            || waveform
                .x
                .iter()
                .chain(waveform.y.iter())
                .any(|value| !value.is_finite())
        {
            return Err(HardcopySourceError::InvalidRetainedWaveform(
                waveform.name.clone(),
            ));
        }
    }

    let source_x_minimum = visible
        .iter()
        .flat_map(|waveform| (*waveform).as_ref().x.iter().copied())
        .min_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no X samples".to_owned()))?;
    let source_x_maximum = visible
        .iter()
        .flat_map(|waveform| (*waveform).as_ref().x.iter().copied())
        .max_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no X samples".to_owned()))?;
    let (x_minimum, x_maximum) = source
        .studio
        .pane_x_ranges
        .get(&pane.id)
        .copied()
        .filter(|(minimum, maximum)| {
            minimum.is_finite() && maximum.is_finite() && minimum < maximum
        })
        .unwrap_or_else(|| nondegenerate_range(source_x_minimum, source_x_maximum));
    let source_y_minimum = visible
        .iter()
        .flat_map(|waveform| (*waveform).as_ref().y.iter().copied())
        .min_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no Y samples".to_owned()))?;
    let source_y_maximum = visible
        .iter()
        .flat_map(|waveform| (*waveform).as_ref().y.iter().copied())
        .max_by(f64::total_cmp)
        .ok_or_else(|| HardcopySourceError::UnretainedResult("no Y samples".to_owned()))?;
    let (mut y_minimum, mut y_maximum) = nondegenerate_range(source_y_minimum, source_y_maximum);
    if source.studio.autoscale == VisualizationAutoscale::RobustVisible {
        let padding = ((y_maximum - y_minimum) * 0.05).max(f64::EPSILON);
        y_minimum -= padding;
        y_maximum += padding;
    }

    let plot_width = PLOT_WIDTH_UM - 2 * PLOT_INSET_UM;
    let plot_height = PLOT_HEIGHT_UM - 2 * PLOT_INSET_UM;
    let mut traces = Vec::with_capacity(visible.len());
    let mut trace_ids = std::collections::HashSet::new();
    for waveform in &visible {
        let waveform = (*waveform).as_ref();
        let trace_id = stable_trace_id(pane.dataset_id, analysis.id, &waveform.name);
        if !trace_ids.insert(trace_id) {
            return Err(HardcopySourceError::DuplicateStableTraceIdentity(trace_id));
        }
        let source_points = waveform
            .x
            .iter()
            .copied()
            .zip(waveform.y.iter().copied())
            .collect::<Vec<_>>();
        traces.push(SemanticPlotTrace {
            trace_id,
            label: waveform.name.clone(),
            paths: clipped_plot_paths(
                &source_points,
                x_minimum,
                x_maximum,
                y_minimum,
                y_maximum,
                plot_width,
                plot_height,
            )?,
            source_samples: source_points
                .iter()
                .map(|(x, y)| (x.to_bits(), y.to_bits()))
                .collect(),
        });
    }
    let markers = source
        .studio
        .markers
        .iter()
        .filter(|marker| {
            marker.dataset_id == pane.dataset_id
                && marker.analysis_sequence == pane.analysis_sequence
                && visible
                    .iter()
                    .any(|waveform| (*waveform).as_ref().name == marker.waveform_name)
        })
        .map(|marker| {
            Ok(SemanticPlotMarker {
                marker_id: marker.id,
                label: marker.label.clone(),
                trace_id: Some(stable_trace_id(
                    marker.dataset_id,
                    marker.analysis_sequence,
                    &marker.waveform_name,
                )),
                source_x_bits: Some(marker.x.to_bits()),
                source_y_bits: Some(marker.y.to_bits()),
                position: Some(map_plot_point(
                    (
                        marker.x.clamp(x_minimum, x_maximum),
                        marker.y.clamp(y_minimum, y_maximum),
                    ),
                    x_minimum,
                    y_minimum,
                    x_maximum - x_minimum,
                    y_maximum - y_minimum,
                    plot_width,
                    plot_height,
                )?),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let annotations = source
        .studio
        .annotations
        .iter()
        .filter(|annotation| {
            annotation.dataset_id == pane.dataset_id
                && annotation.analysis_sequence == pane.analysis_sequence
        })
        .map(|annotation| {
            Ok(SemanticPlotAnnotation {
                annotation_id: annotation.id,
                text: annotation.text.clone(),
                trace_id: None,
                source_x_bits: Some(annotation.x.to_bits()),
                source_y_bits: None,
                position: Some(map_plot_point(
                    (annotation.x.clamp(x_minimum, x_maximum), y_maximum),
                    x_minimum,
                    y_minimum,
                    x_maximum - x_minimum,
                    y_maximum - y_minimum,
                    plot_width,
                    plot_height,
                )?),
            })
        })
        .collect::<Result<Vec<_>, HardcopySourceError>>()?;
    let semantic = SemanticPlot {
        viewer: pane.viewer,
        page_id: stable_page_id(&pane.page),
        pane_id: pane.id,
        // The studio pane's own axis declarations are not projected into this
        // adapter yet, and a scale it has not been told is linear.
        x_scale: AxisScale::Linear,
        y_scale: AxisScale::Linear,
        axis_ticks: Vec::new(),
        traces,
        cursors: Vec::new(),
        markers,
        annotations,
        captions: Vec::new(),
    };
    let digest = studio_pane_digest(
        source,
        pane,
        run.run_id,
        analysis.id,
        &visible,
        &source
            .studio
            .markers
            .iter()
            .filter(|marker| {
                marker.dataset_id == pane.dataset_id
                    && marker.analysis_sequence == pane.analysis_sequence
                    && visible
                        .iter()
                        .any(|waveform| (*waveform).as_ref().name == marker.waveform_name)
            })
            .collect::<Vec<_>>(),
        &source
            .studio
            .annotations
            .iter()
            .filter(|annotation| {
                annotation.dataset_id == pane.dataset_id
                    && annotation.analysis_sequence == pane.analysis_sequence
            })
            .collect::<Vec<_>>(),
    )?;
    let identity =
        studio_source_identity(&source_key, source.project_id, source.studio.revision, pane)?;
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        HardcopySemanticDocument::Plot(semantic),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(PLOT_WIDTH_UM, PLOT_HEIGHT_UM),
        )?,
    )
}

fn resolve_studio_result_summary<R, W: AsRef<RetainedWaveform>>(
    source: &StudioHardcopySource<'_, R, W>,
    source_key: &str,
    scope: HardcopyScope,
    pane: &StudioPane,
    run_id: RunId,
    analysis: &AnalysisResult<W>,
) -> Result<ResolvedHardcopyDocument, HardcopySourceError> {
    let summary = semantic_result_summary(pane.viewer, analysis)?;
    let digest = canonical_digest(
        b"rspice-hardcopy-studio-result-summary-v1",
        &(source.studio.revision, pane, run_id, analysis.id, &summary),
    )?;
    let identity =
        studio_source_identity(source_key, source.project_id, source.studio.revision, pane)?;
    resolve_semantic_source(
        identity,
        digest,
        HardcopyDocumentKind::PlotOrWorksheet,
        scope,
        HardcopySemanticDocument::ResultSummary(Box::new(summary)),
        SemanticBounds::try_new(
            SemanticPoint::new(0, 0),
            SemanticPoint::new(REPORT_PAGE_WIDTH_UM, REPORT_PAGE_HEIGHT_UM),
        )?,
    )
}

pub fn studio_source_identity(
    source_key: &str,
    project_id: ProjectId,
    studio_revision: u64,
    pane: &StudioPane,
) -> Result<HardcopySourceIdentity, HardcopySourceError> {
    let mut identity_name = Vec::with_capacity(24);
    identity_name.extend_from_slice(pane.dataset_id.as_uuid().as_bytes());
    identity_name.extend_from_slice(&pane.id.to_be_bytes());
    HardcopySourceIdentity::try_new(
        source_key,
        HardcopyDocumentId::try_from_uuid(Uuid::new_v5(&project_id.as_uuid(), &identity_name))
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        ObjectRevision::new(studio_revision)
            .map_err(|error| HardcopySourceError::HardcopyContract(error.to_string()))?,
        format!("{} · {}", pane.page, pane.viewer.label()),
    )
}

fn stable_trace_id(dataset_id: DatasetId, analysis_sequence: u64, name: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-studio-trace-id-v1");
    hasher.update(dataset_id.as_uuid().as_bytes());
    hasher.update(analysis_sequence.to_be_bytes());
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0_u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes).max(1)
}

#[derive(Serialize)]
struct StudioWaveformDigestMaterial<'a> {
    name: &'a str,
    color: &'a str,
    visible: bool,
    x_bits: Vec<u64>,
    y_bits: Vec<u64>,
}

#[derive(Serialize)]
struct StudioPaneDigestMaterial<'a> {
    studio_revision: u64,
    pane: &'a StudioPane,
    run_id: RunId,
    analysis_sequence: u64,
    waveforms: Vec<StudioWaveformDigestMaterial<'a>>,
    markers: &'a [&'a StudioMarker],
    annotations: &'a [&'a StudioAnnotation],
}

fn studio_pane_digest<R, W: AsRef<RetainedWaveform>>(
    source: &StudioHardcopySource<'_, R, W>,
    pane: &StudioPane,
    run_id: RunId,
    analysis_sequence: u64,
    waveforms: &[&W],
    markers: &[&StudioMarker],
    annotations: &[&StudioAnnotation],
) -> Result<ContentDigest, HardcopySourceError> {
    let waveforms = waveforms
        .iter()
        .map(|waveform| {
            let style = (source.waveform_style)(waveform);
            let waveform = (*waveform).as_ref();
            StudioWaveformDigestMaterial {
                name: &waveform.name,
                color: style.color,
                visible: style.visible,
                x_bits: waveform.x.iter().map(|value| value.to_bits()).collect(),
                y_bits: waveform.y.iter().map(|value| value.to_bits()).collect(),
            }
        })
        .collect();
    canonical_digest(
        b"rspice-hardcopy-studio-pane-v1",
        &StudioPaneDigestMaterial {
            studio_revision: source.studio.revision,
            pane,
            run_id,
            analysis_sequence,
            waveforms,
            markers,
            annotations,
        },
    )
}

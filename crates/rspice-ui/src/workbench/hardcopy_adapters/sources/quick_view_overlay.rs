//! What the reader had on the sheet, frozen with the rest of the capture.
//!
//! A quick-view hardcopy is resolved from retained samples, which is what
//! makes it reproducible — but the sheet the reader pressed Print on is not
//! only its samples. Traces they hid, cursors they placed, markers they
//! anchored: those are the reading, and a page without them is a different
//! document from the one that was reviewed.
//!
//! None of it is read from the screen. The overlay is captured from the same
//! session state the sheet draws from, through the same read paths, on the UI
//! thread, and travels with the presentation into the worker.

use super::*;

use crate::ui::plot::{SampleInterpolation, sample_at_with};
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, MarkerKind, MarkerView, SourceWaveformPresentationKey,
};
use crate::workbench::preferences::CursorInterpolation;

/// A strip carrying more markers than this is not a reading any more, and a
/// prepared snapshot must not carry an unbounded list across the boundary.
const MAX_RETAINED_OVERLAY_MARKERS: usize = 512;
/// The same bound over strips. A run with more analyses than this cannot be
/// printed as a stack either.
const MAX_RETAINED_OVERLAY_STRIPS: usize = 512;

/// How the sheet resamples a marker's Y off its anchored trace.
///
/// Its own enum rather than `SampleInterpolation` for the reason every other
/// prepared control has one: this travels to the worker as data, so the shape
/// that crosses the boundary is declared here and validated on the way back.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum RetainedCursorInterpolation {
    #[default]
    MonotoneCubic,
    Linear,
    Nearest,
}

impl RetainedCursorInterpolation {
    const fn sampler(self) -> SampleInterpolation {
        match self {
            Self::MonotoneCubic => SampleInterpolation::MonotoneCubic,
            Self::Linear => SampleInterpolation::Linear,
            Self::Nearest => SampleInterpolation::Nearest,
        }
    }
}

/// One marker as the sheet drew it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RetainedQuickMarker {
    /// The tag text: the display id, and the reader's note when there is one.
    pub label: String,
    pub kind: MarkerKind,
    /// Anchor position in the strip's X data space.
    pub x: f64,
    /// The retained trace the marker rides, by the name the sheet lists it
    /// under. A spec limit has none: it constrains the axis position, which
    /// every pane of the strip shares, rather than one curve.
    pub trace_name: Option<String>,
}

/// The transient reading state of one captured strip.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RetainedQuickViewOverlay {
    /// The trace names the sheet is showing, after the reader's per-trace
    /// overrides. `None` means nothing was captured and the dataset's own
    /// flags stand.
    #[serde(default)]
    visible_traces: Option<Vec<String>>,
    #[serde(default)]
    pub cursor_a: Option<f64>,
    #[serde(default)]
    pub cursor_b: Option<f64>,
    /// Both marker stores, in placement order, exactly as `strip_markers`
    /// serves them to the renderer and the readout.
    #[serde(default)]
    pub markers: Vec<RetainedQuickMarker>,
    #[serde(default)]
    interpolation: RetainedCursorInterpolation,
}

/// One strip's reading, keyed by the analysis it belongs to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RetainedStripOverlay {
    /// The run-local analysis sequence, which is what a quick-view page's own
    /// identity is derived from.
    analysis_id: u64,
    overlay: RetainedQuickViewOverlay,
}

/// The reading of every strip in the captured run.
///
/// Plural because a stacked wave view resolves one page per analysis, each
/// through the same frozen presentation: a single overlay would put the first
/// strip's markers on all of them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub(super) struct RetainedQuickViewOverlays(Vec<RetainedStripOverlay>);

impl RetainedQuickViewOverlays {
    pub(super) fn capture(state: &AppState, run: &SimulationRun) -> Self {
        Self(
            run.analyses
                .iter()
                .map(|analysis| RetainedStripOverlay {
                    analysis_id: analysis.id,
                    overlay: RetainedQuickViewOverlay::capture(state, run, analysis),
                })
                .collect(),
        )
    }

    /// The reading for one strip, or an empty one for a page whose analysis
    /// was not captured — a page without annotation, never another strip's.
    pub(super) fn for_analysis(&self, analysis_id: u64) -> RetainedQuickViewOverlay {
        self.0
            .iter()
            .find(|entry| entry.analysis_id == analysis_id)
            .map(|entry| entry.overlay.clone())
            .unwrap_or_default()
    }

    pub(super) fn validate(&self) -> Result<(), HardcopySourceError> {
        if self.0.len() > MAX_RETAINED_OVERLAY_STRIPS {
            return Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(format!(
                "prepared overlay carries {} strips, above the governed limit",
                self.0.len()
            )));
        }
        for entry in &self.0 {
            entry.overlay.validate()?;
        }
        Ok(())
    }
}

impl RetainedQuickViewOverlay {
    /// One reading, stated rather than captured.
    ///
    /// A document marker only reaches `strip_markers` while the persistent
    /// pane context it belongs to is entered, and entering one is private to
    /// the result document. This exists so the resolution half — a retained
    /// `D` marker travelling from the frozen capture onto the page — can be
    /// stated without standing up a whole visualization document.
    #[cfg(test)]
    pub(super) fn for_test(
        cursor_a: Option<f64>,
        cursor_b: Option<f64>,
        markers: Vec<RetainedQuickMarker>,
    ) -> Self {
        Self {
            visible_traces: None,
            cursor_a,
            cursor_b,
            markers,
            interpolation: RetainedCursorInterpolation::default(),
        }
    }

    /// Freeze the reading state of the strip a quick-view page will show.
    fn capture(state: &AppState, run: &SimulationRun, analysis: &AnalysisResult) -> Self {
        let results = &state.ui.results;
        let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        let visible_traces = analysis
            .waveforms
            .iter()
            .filter(|waveform| {
                results.waveform_visibility(
                    &SourceWaveformPresentationKey::new(analysis_key, &waveform.name),
                    waveform.visible,
                )
            })
            .map(|waveform| waveform.name.clone())
            .collect();
        // One read path with the screen: `strip_markers` unions the quick and
        // document stores and verifies the overlay against the live pane
        // context, so a stale projection cannot put another document's
        // markers on this page.
        let markers = results
            .strip_markers(analysis_key)
            .map(|marker| RetainedQuickMarker {
                label: marker_tag(marker),
                kind: marker.kind(),
                x: marker.x(),
                trace_name: (marker.kind() != MarkerKind::Spec)
                    .then(|| marker.trace_name().to_owned()),
            })
            .collect();
        Self {
            visible_traces: Some(visible_traces),
            cursor_a: results.cursors.a,
            cursor_b: results.cursors.b,
            markers,
            interpolation: match state
                .ui
                .preferences
                .result_presentation_policy()
                .cursor_interpolation()
            {
                CursorInterpolation::MonotoneCubicWhereValid => {
                    RetainedCursorInterpolation::MonotoneCubic
                }
                CursorInterpolation::Linear => RetainedCursorInterpolation::Linear,
                CursorInterpolation::NearestAcceptedPoint => RetainedCursorInterpolation::Nearest,
            },
        }
    }

    /// Reject a capture that cannot describe a page before it crosses the
    /// worker boundary, in the same shape as every other prepared control.
    fn validate(&self) -> Result<(), HardcopySourceError> {
        let invalid =
            |detail: String| Err(HardcopySourceError::InvalidPreparedWorkerSnapshot(detail));
        let optional_label = |field, value: Option<&str>| {
            value.map_or(Ok(()), |value| {
                validate_label(field, value, DISPLAY_NAME_LIMIT)
            })
        };
        if self.markers.len() > MAX_RETAINED_OVERLAY_MARKERS {
            return invalid(format!(
                "prepared overlay carries {} markers, above the governed limit",
                self.markers.len()
            ));
        }
        for cursor in [self.cursor_a, self.cursor_b].into_iter().flatten() {
            if !cursor.is_finite() {
                return invalid("prepared overlay cursor is not finite".to_owned());
            }
        }
        for marker in &self.markers {
            validate_label("prepared overlay marker", &marker.label, DISPLAY_NAME_LIMIT)?;
            optional_label(
                "prepared overlay marker trace",
                marker.trace_name.as_deref(),
            )?;
            if !marker.x.is_finite() {
                return invalid(format!(
                    "prepared overlay marker {} has a non-finite position",
                    marker.label
                ));
            }
        }
        if let Some(names) = &self.visible_traces {
            for name in names {
                validate_label("prepared overlay trace", name, DISPLAY_NAME_LIMIT)?;
            }
        }
        Ok(())
    }

    /// Whether the captured sheet was showing this trace.
    ///
    /// The dataset's own flag is the default and the reader's override wins
    /// over it — the printed page carried every retained trace regardless,
    /// so hiding a trace changed the sheet and not the page taken from it.
    pub(super) fn trace_is_visible(&self, name: &str, dataset_default: bool) -> bool {
        self.visible_traces
            .as_ref()
            .map_or(dataset_default, |names| {
                names.iter().any(|visible| visible == name)
            })
    }

    /// The Y a marker's tag sits at, resampled off its anchored trace exactly
    /// as the sheet resamples it every frame.
    pub(super) fn marker_y(&self, points: &[(f64, f64)], x: f64) -> f64 {
        let (xs, ys): (Vec<f64>, Vec<f64>) = points.iter().copied().unzip();
        sample_at_with(&xs, &ys, x, self.interpolation.sampler())
    }
}

/// The tag text the sheet draws: the display id, plus the note when the
/// reader wrote one. Mirrors `waves::readout::marker_label`, which is
/// `pub(super)` to `result_document` and cannot be called from here.
fn marker_tag(marker: MarkerView<'_>) -> String {
    let id = marker.display_id();
    let note = marker.note().trim();
    if note.is_empty() {
        id
    } else {
        format!("{id} · {note}")
    }
}
/// The axis rectangle one plot's overlay is mapped into.
pub(super) struct PlotFrame {
    pub x_minimum: f64,
    pub x_maximum: f64,
    pub y_minimum: f64,
    pub y_maximum: f64,
    pub x_span: f64,
    pub y_span: f64,
    pub plot_width: i64,
    pub plot_height: i64,
}

/// Place the captured cursors and markers on the page.
///
/// Every position is derived from the source coordinate through the same
/// mapping the traces went through, so a marker sits on its curve on the page
/// for the same reason it sits on it on screen. A cursor, and a spec marker,
/// are full-height lines: a spec constrains the axis position rather than one
/// curve, which is exactly what the semantic cursor primitive expresses, so
/// it travels there rather than being flattened into a point.
pub(super) fn resolved_overlay_geometry(
    viewer: ResultViewer,
    overlay: &RetainedQuickViewOverlay,
    series: &[QuickResultSeries],
    frame: PlotFrame,
) -> Result<(Vec<SemanticPlotCursor>, Vec<SemanticPlotMarker>), HardcopySourceError> {
    let mut cursors = Vec::new();
    let mut markers = Vec::new();
    let mut vertical = |label: &str, x: f64| -> Result<(), HardcopySourceError> {
        if !x.is_finite() || x < frame.x_minimum || x > frame.x_maximum {
            return Ok(());
        }
        let point = |y| {
            map_plot_point(
                x,
                y,
                frame.x_minimum,
                frame.y_minimum,
                frame.x_span,
                frame.y_span,
                frame.plot_width,
                frame.plot_height,
            )
        };
        cursors.push(SemanticPlotCursor {
            cursor_id: stable_overlay_id(viewer, "cursor", label),
            label: label.to_owned(),
            source_x_bits: x.to_bits(),
            start: point(frame.y_minimum)?,
            end: point(frame.y_maximum)?,
        });
        Ok(())
    };
    if let Some(x) = overlay.cursor_a {
        vertical("A", x)?;
    }
    if let Some(x) = overlay.cursor_b {
        vertical("B", x)?;
    }
    for marker in &overlay.markers {
        let Some(trace_name) = marker.trace_name.as_deref() else {
            vertical(&marker.label, marker.x)?;
            continue;
        };
        // A marker whose anchored trace is not on this page has nothing to
        // ride, exactly as on screen: the sheet skips it rather than pinning
        // it to a neighbouring curve.
        let Some((index, source)) = series
            .iter()
            .enumerate()
            .find(|(_, source)| source.label == trace_name)
        else {
            continue;
        };
        let y = overlay.marker_y(&source.points, marker.x);
        if !marker.x.is_finite()
            || marker.x < frame.x_minimum
            || marker.x > frame.x_maximum
            || !y.is_finite()
            || y < frame.y_minimum
            || y > frame.y_maximum
        {
            continue;
        }
        markers.push(SemanticPlotMarker {
            marker_id: stable_overlay_id(viewer, "marker", &marker.label),
            label: marker.label.clone(),
            trace_id: Some(stable_quick_trace_id(viewer, index, &source.identity)),
            source_x_bits: Some(marker.x.to_bits()),
            source_y_bits: Some(y.to_bits()),
            position: Some(map_plot_point(
                marker.x,
                y,
                frame.x_minimum,
                frame.y_minimum,
                frame.x_span,
                frame.y_span,
                frame.plot_width,
                frame.plot_height,
            )?),
        });
    }
    Ok((cursors, markers))
}

/// A stable id for one overlay entity, in the same shape as the trace ids so
/// a print mapping can name a cursor or a marker across two runs of the same
/// page.
fn stable_overlay_id(viewer: ResultViewer, kind: &str, label: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(b"rspice-hardcopy-results-overlay-v1");
    hasher.update(viewer.label().as_bytes());
    hasher.update(kind.as_bytes());
    hasher.update(label.as_bytes());
    let bytes: [u8; 8] = hasher.finalize()[..8]
        .try_into()
        .expect("sha256 yields at least eight bytes");
    u64::from_be_bytes(bytes)
}

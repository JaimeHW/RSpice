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
    /// The window the reader had pinned on the sheet, in the axes' source
    /// space. `None` on either axis means that axis was showing the data's
    /// own extents, which is what the page rules when nothing is captured.
    #[serde(default)]
    viewport: Option<RetainedQuickViewport>,
}

/// One captured zoom, in the source space of the axes it was pinned on.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RetainedQuickViewport {
    #[serde(default)]
    x: Option<(f64, f64)>,
    #[serde(default)]
    y: Option<(f64, f64)>,
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
            viewport: None,
        }
    }

    /// The page's own bounds, narrowed to whatever the reader had pinned.
    ///
    /// The bounds arrive in the axes' projected space, and so does the
    /// capture: both went through `project`, so a logarithmic abscissa is
    /// compared in decades at both ends. A captured edge that the projection
    /// cannot place — a non-positive frequency on a log axis — leaves that
    /// axis showing the data, rather than pinning the page to a coordinate
    /// the axis has no position for.
    pub(super) fn framed_bounds(
        &self,
        x_scale: AxisScale,
        y_scale: AxisScale,
        bounds: (f64, f64, f64, f64),
    ) -> (f64, f64, f64, f64) {
        let (x_minimum, x_maximum, y_minimum, y_maximum) = bounds;
        let Some(viewport) = self.viewport else {
            return bounds;
        };
        let pinned = |scale, range: Option<(f64, f64)>, fallback: (f64, f64)| {
            range
                .and_then(|(low, high)| Some((project(scale, low)?, project(scale, high)?)))
                .filter(|(low, high)| low < high)
                .unwrap_or(fallback)
        };
        let (x_minimum, x_maximum) = pinned(x_scale, viewport.x, (x_minimum, x_maximum));
        let (y_minimum, y_maximum) = pinned(y_scale, viewport.y, (y_minimum, y_maximum));
        (x_minimum, x_maximum, y_minimum, y_maximum)
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
            viewport: crate::workbench::documents::result_document::captured_viewport(
                state,
                analysis_key,
            )
            .map(|(x, y)| RetainedQuickViewport { x, y }),
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
        if let Some(viewport) = self.viewport {
            for (low, high) in [viewport.x, viewport.y].into_iter().flatten() {
                if !low.is_finite() || !high.is_finite() || low >= high {
                    return invalid(format!(
                        "prepared overlay viewport bound {low} .. {high} is not an interval"
                    ));
                }
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
/// Whether one printed series carries the retained trace a marker rides.
///
/// The label is what the sheet writes on the legend, and several viewers
/// decorate it: the Bode page lists `|V(out)| (dB)`, which is never equal to
/// the `V(out)` a marker is anchored to, so every marker on a printed Bode
/// page was dropped without a word. The identity is the other half of the
/// same series and is built from the retained name — `dataset:run:analysis`
/// and then the name, colon-delimited, with a derivation suffix where the
/// series is one — so the anchor is resolved there instead. The first three
/// components are skipped rather than searched: an analysis ordinal of `3`
/// would otherwise answer to a net called `3`.
fn series_carries_trace(series: &QuickResultSeries, trace_name: &str) -> bool {
    if series.label == trace_name {
        return true;
    }
    series
        .identity
        .splitn(4, ':')
        .nth(3)
        .is_some_and(|retained| {
            retained == trace_name
                || retained
                    .strip_prefix(trace_name)
                    .is_some_and(|suffix| suffix.starts_with(':'))
        })
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
    x_scale: AxisScale,
    y_scale: AxisScale,
    frame: &PlotFrame,
) -> Result<(Vec<SemanticPlotCursor>, Vec<SemanticPlotMarker>), HardcopySourceError> {
    let mut cursors = Vec::new();
    let mut markers = Vec::new();
    let mut vertical = |label: &str, source_x: f64| -> Result<(), HardcopySourceError> {
        // The frame is in the axes' own space, so the source coordinate is
        // projected before it is placed — and the exact source value still
        // travels beside the geometry.
        let Some(x) = project(x_scale, source_x) else {
            return Ok(());
        };
        if x < frame.x_minimum || x > frame.x_maximum {
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
            source_x_bits: source_x.to_bits(),
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
            .find(|(_, source)| series_carries_trace(source, trace_name))
        else {
            continue;
        };
        let source_y = overlay.marker_y(&source.points, marker.x);
        let (Some(x), Some(y)) = (project(x_scale, marker.x), project(y_scale, source_y)) else {
            continue;
        };
        if x < frame.x_minimum || x > frame.x_maximum || y < frame.y_minimum || y > frame.y_maximum
        {
            continue;
        }
        markers.push(SemanticPlotMarker {
            marker_id: stable_overlay_id(viewer, "marker", &marker.label),
            label: marker.label.clone(),
            trace_id: Some(stable_quick_trace_id(viewer, index, &source.identity)),
            source_x_bits: Some(marker.x.to_bits()),
            source_y_bits: Some(source_y.to_bits()),
            position: Some(map_plot_point(
                x,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> PlotFrame {
        PlotFrame {
            x_minimum: 1.0,
            x_maximum: 6.0,
            y_minimum: -40.0,
            y_maximum: 40.0,
            x_span: 5.0,
            y_span: 80.0,
            plot_width: 100_000,
            plot_height: 100_000,
        }
    }

    fn anchored_marker(trace_name: &str) -> RetainedQuickViewOverlay {
        RetainedQuickViewOverlay::for_test(
            None,
            None,
            vec![RetainedQuickMarker {
                label: "D1".to_owned(),
                kind: MarkerKind::Note,
                x: 3.0,
                trace_name: Some(trace_name.to_owned()),
            }],
        )
    }

    /// A marker rides its retained trace, whatever the page calls it.
    ///
    /// Markers were matched against the series' display label, and the Bode
    /// page decorates its labels — `|V(out)| (dB)` is never equal to the
    /// `V(out)` a marker is anchored to. Every marker on a printed Bode page
    /// was therefore dropped without a word, on a page whose whole reason for
    /// carrying the overlay is that the annotations are the reading.
    #[test]
    fn a_bode_marker_finds_its_trace_through_the_decorated_label() {
        let series = vec![QuickResultSeries {
            identity: "dataset:run:1:V(out):magnitude-db".to_owned(),
            label: "|V(out)| (dB)".to_owned(),
            points: vec![(1.0, 40.0), (6.0, -40.0)],
        }];

        let (_, markers) = resolved_overlay_geometry(
            ResultViewer::Bode,
            &anchored_marker("V(out)"),
            &series,
            AxisScale::Linear,
            AxisScale::Linear,
            &frame(),
        )
        .expect("the marker maps inside the frame");

        assert_eq!(
            markers.len(),
            1,
            "the marker was dropped because the page decorates its label"
        );
        assert_eq!(markers[0].label, "D1");
        assert_eq!(
            markers[0].trace_id,
            Some(stable_quick_trace_id(
                ResultViewer::Bode,
                0,
                "dataset:run:1:V(out):magnitude-db"
            ))
        );
    }

    /// And a marker whose trace is not on this page still has nothing to
    /// ride: the match is on the retained name, not on any substring of it.
    #[test]
    fn a_marker_for_another_trace_is_still_dropped() {
        let series = vec![QuickResultSeries {
            identity: "dataset:run:1:V(out):magnitude-db".to_owned(),
            label: "|V(out)| (dB)".to_owned(),
            points: vec![(1.0, 40.0), (6.0, -40.0)],
        }];

        let (_, markers) = resolved_overlay_geometry(
            ResultViewer::Bode,
            &anchored_marker("V(in)"),
            &series,
            AxisScale::Linear,
            AxisScale::Linear,
            &frame(),
        )
        .expect("the overlay resolves");

        assert!(markers.is_empty());
    }

    /// The undecorated sheets keep matching on the name they already show.
    #[test]
    fn a_waveform_marker_still_rides_its_own_series() {
        let series = vec![QuickResultSeries {
            identity: "dataset:run:1:V(out)".to_owned(),
            label: "V(out)".to_owned(),
            points: vec![(1.0, 40.0), (6.0, -40.0)],
        }];

        let (_, markers) = resolved_overlay_geometry(
            ResultViewer::Waves,
            &anchored_marker("V(out)"),
            &series,
            AxisScale::Linear,
            AxisScale::Linear,
            &frame(),
        )
        .expect("the marker maps inside the frame");

        assert_eq!(markers.len(), 1);
    }
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

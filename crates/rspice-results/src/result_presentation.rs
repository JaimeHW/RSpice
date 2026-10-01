//! Durable presentation identities and annotations for retained results.
//!
//! Shared by project I/O and the workbench. Rendering, session caches, and
//! editor state remain in the workbench; none can redefine this wire contract.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[cfg(feature = "engine-evidence")]
use crate::{analysis_result::AnalysisResult, run::SimulationRun, waveform::RetainedWaveform};
use rspice_app_types::product::{AnalysisInstanceId, DatasetId};

/// The result viewers, in tab order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ResultViewer {
    /// Stacked waveform strips, one per analysis.
    #[default]
    Waves,
    /// Swept-source or swept-parameter DC transfer curves.
    DcSweep,
    /// Loop-gain stability view with margin markers.
    Bode,
    /// Spectrum with harmonic markers.
    Fft,
    /// Retained complex harmonic-balance coefficient spectrum.
    HarmonicBalance,
    /// Periodic phase-noise spectrum versus offset frequency.
    PhaseNoise,
    /// Eye diagram with compliance mask.
    Eye,
    /// Monte-Carlo distribution.
    Hist,
    /// Per-device operating-point inspector (Spectre-style OP info).
    Op,
    /// Ordinary-noise root spectral density with contributor evidence.
    NoiseContrib,
    /// Ranked signed parameter-sensitivity contributions.
    Contribution,
    /// Scalar DC transfer gain and input/output resistances.
    TransferFunction,
    /// Measurements × runs matrix against spec bounds.
    Specs,
    /// The retained samples of one analysis, as rows.
    Table,
    /// Nyquist loop-gain stability surface.
    Nyquist,
    /// Smith-chart RF/network surface.
    Smith,
    /// One retained complex response on the polar plane.
    Polar,
    /// Complex-plane pole-zero surface.
    PoleZero,
    /// Two measured Monte-Carlo columns read as a correlation.
    Scatter,
    /// The shape of a retained population against its requirement.
    BoxViolin,
    /// Committed XSPICE digital and real-valued event history.
    Events,
    /// Safe-operating-area rule evidence with per-rule stress history.
    Soa,
    /// Optimizer cost convergence and the candidate history behind it.
    Optimization,
    /// Immutable task and retained-value inventory for the active dataset.
    ///
    /// This is dataset-native and deliberately has no Visualization Studio
    /// viewer-document identity.
    Manifest,
    /// Exact single-ended and mixed-mode power-wave matrices.
    NetworkMatrix,
}

impl ResultViewer {
    /// Compact command/status label.
    pub fn label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "WAVES",
            ResultViewer::DcSweep => "DC",
            ResultViewer::Bode => "BODE",
            ResultViewer::Fft => "FFT",
            ResultViewer::HarmonicBalance => "HB",
            ResultViewer::PhaseNoise => "PNOISE",
            ResultViewer::Eye => "EYE",
            ResultViewer::Hist => "HIST",
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "NOISE",
            ResultViewer::Contribution => "SENS",
            ResultViewer::TransferFunction => "XF",
            ResultViewer::Specs => "SPECS",
            ResultViewer::Table => "TABLE",
            ResultViewer::Nyquist => "NYQ",
            ResultViewer::Smith => "SMITH",
            ResultViewer::Polar => "POLAR",
            ResultViewer::NetworkMatrix => "NETWORK",
            ResultViewer::PoleZero => "PZ",
            ResultViewer::Scatter => "SCATTER",
            ResultViewer::BoxViolin => "DIST",
            ResultViewer::Events => "EVENTS",
            ResultViewer::Soa => "SOA",
            ResultViewer::Optimization => "OPT",
            ResultViewer::Manifest => "MANIFEST",
        }
    }

    const PRIMARY: [ResultViewer; 24] = [
        ResultViewer::Waves,
        ResultViewer::DcSweep,
        ResultViewer::Bode,
        ResultViewer::NoiseContrib,
        ResultViewer::Nyquist,
        ResultViewer::Fft,
        ResultViewer::HarmonicBalance,
        ResultViewer::PhaseNoise,
        ResultViewer::Smith,
        ResultViewer::Polar,
        ResultViewer::NetworkMatrix,
        ResultViewer::TransferFunction,
        ResultViewer::Contribution,
        ResultViewer::Op,
        ResultViewer::Specs,
        ResultViewer::Table,
        ResultViewer::Hist,
        ResultViewer::Scatter,
        ResultViewer::BoxViolin,
        ResultViewer::Eye,
        ResultViewer::PoleZero,
        // Specialist sheets last: each needs evidence an ordinary run does not
        // produce — XSPICE event nodes, or a whole campaign analysis kind — so
        // leading with them would push the everyday sheets rightward for the
        // sake of tabs that are usually dim.
        ResultViewer::Events,
        ResultViewer::Soa,
        ResultViewer::Optimization,
    ];
    const DATASET_NATIVE: [ResultViewer; 1] = [ResultViewer::Manifest];

    /// Every result presentation, in the established tab order.
    pub fn all() -> impl Iterator<Item = ResultViewer> {
        Self::PRIMARY.into_iter().chain(Self::DATASET_NATIVE)
    }

    /// Human-readable document-tab label from the upgraded Results mockup.
    pub const fn tab_label(self) -> &'static str {
        match self {
            ResultViewer::Waves => "Waves",
            ResultViewer::DcSweep => "DC Sweep",
            ResultViewer::Bode => "Bode",
            ResultViewer::Fft => "FFT",
            ResultViewer::HarmonicBalance => "HB Tones",
            ResultViewer::PhaseNoise => "Phase Noise",
            ResultViewer::Eye => "Eye",
            ResultViewer::Hist => "Histogram",
            ResultViewer::Op => "OP",
            ResultViewer::NoiseContrib => "Noise",
            ResultViewer::Contribution => "Sensitivity",
            ResultViewer::TransferFunction => "XF",
            ResultViewer::Specs => "Specs",
            ResultViewer::Table => "Table",
            ResultViewer::Nyquist => "Nyquist",
            ResultViewer::Smith => "Smith",
            ResultViewer::Polar => "Polar",
            ResultViewer::NetworkMatrix => "Network Matrix",
            ResultViewer::PoleZero => "PZ",
            ResultViewer::Scatter => "Scatter",
            ResultViewer::BoxViolin => "Distribution",
            ResultViewer::Events => "Events",
            ResultViewer::Soa => "SOA",
            ResultViewer::Optimization => "Optimization",
            ResultViewer::Manifest => "Manifest",
        }
    }

    /// The Visualization Studio viewer document this sheet renders, if any.
    ///
    /// `None` means the sheet is dataset-native: it reads the retained result
    /// directly and has no place in the Studio's document catalog, so binding
    /// a page to it would name a renderer that cannot draw the page.
    ///
    /// One owner on purpose. Three copies of this map existed — in the
    /// persistent-document layer, the Studio, and the workbench state — and
    /// they disagreed about the noise sheet: two said `viewer-bode`, the
    /// Studio's own said `viewer-spectrum`, which the catalog rejects for a
    /// `noise` analysis. Whether a noise sheet could be pinned into a
    /// document depended on which path asked.
    pub const fn viewer_document_id(self) -> Option<&'static str> {
        Some(match self {
            ResultViewer::Manifest => return None,
            ResultViewer::Waves | ResultViewer::DcSweep => "viewer-waveform",
            ResultViewer::Bode | ResultViewer::Nyquist | ResultViewer::NoiseContrib => {
                "viewer-bode"
            }
            ResultViewer::Fft | ResultViewer::HarmonicBalance => "viewer-spectrum",
            ResultViewer::PhaseNoise => "viewer-phase-noise",
            ResultViewer::Eye => "eye-viewer",
            ResultViewer::Hist => "viewer-histogram",
            ResultViewer::Op | ResultViewer::Specs | ResultViewer::Table => "viewer-table",
            ResultViewer::Contribution => "viewer-contribution",
            ResultViewer::TransferFunction => "viewer-transfer-function",
            ResultViewer::Smith => "viewer-smith",
            ResultViewer::Polar => "viewer-polar",
            ResultViewer::NetworkMatrix => "viewer-mixed-mode-network",
            ResultViewer::PoleZero => "viewer-pz",
            ResultViewer::Scatter => "viewer-scatter",
            ResultViewer::BoxViolin => "viewer-box-violin",
            ResultViewer::Events => "viewer-digital-events",
            ResultViewer::Soa => "viewer-soa",
            ResultViewer::Optimization => "viewer-optimization",
        })
    }

    /// The sheet that draws a retained pane bound to this viewer document.
    ///
    /// The inverse of [`Self::viewer_document_id`] is not one-to-one — three
    /// sheets render `viewer-table` — so the choice has to be made once, here.
    /// It was made twice instead, and the two answers differed: the persistent
    /// document layer drew the Table sheet and the Studio drew Specs, so the
    /// same pane showed exact samples in one surface and a specification matrix
    /// in the other. Table is the truthful answer, because
    /// `renderer_supports_analysis` admits a `viewer-table` pane on retained
    /// waveforms — which is what Table reads and what Specs does not.
    pub fn from_viewer_document_id(id: &str) -> Option<Self> {
        Some(match id {
            "viewer-waveform" => ResultViewer::Waves,
            "viewer-bode" => ResultViewer::Bode,
            "viewer-spectrum" => ResultViewer::Fft,
            "viewer-phase-noise" => ResultViewer::PhaseNoise,
            "viewer-smith" => ResultViewer::Smith,
            "viewer-polar" => ResultViewer::Polar,
            "viewer-mixed-mode-network" => ResultViewer::NetworkMatrix,
            "viewer-table" => ResultViewer::Table,
            "viewer-histogram" => ResultViewer::Hist,
            "viewer-scatter" => ResultViewer::Scatter,
            "viewer-box-violin" => ResultViewer::BoxViolin,
            "eye-viewer" => ResultViewer::Eye,
            "viewer-pz" => ResultViewer::PoleZero,
            "viewer-contribution" => ResultViewer::Contribution,
            "viewer-transfer-function" => ResultViewer::TransferFunction,
            "viewer-digital-events" => ResultViewer::Events,
            "viewer-soa" => ResultViewer::Soa,
            "viewer-optimization" => ResultViewer::Optimization,
            _ => return None,
        })
    }
}

/// Stable identity of one retained analysis within one immutable dataset.
///
/// Current results use the exact prepared-task identity. A legacy result has
/// no such provenance, so its run-local analysis id is safe only when paired
/// with the immutable dataset id. Neither representation depends on vector
/// position, which prevents presentation state from moving to another
/// analysis when retained results are reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AnalysisPresentationKey {
    dataset_id: DatasetId,
    source: AnalysisPresentationSource,
}

/// Which authored analysis a result came from, independent of the dataset it
/// was solved into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalysisPresentationSource {
    Prepared(AnalysisInstanceId),
    Legacy(u64),
}

impl AnalysisPresentationKey {
    #[cfg(feature = "engine-evidence")]
    pub fn new<W: AsRef<RetainedWaveform>>(
        dataset_id: DatasetId,
        analysis: &AnalysisResult<W>,
    ) -> Self {
        let source = analysis.provenance().map_or(
            AnalysisPresentationSource::Legacy(analysis.id),
            |provenance| AnalysisPresentationSource::Prepared(provenance.source_instance_id()),
        );
        Self { dataset_id, source }
    }

    pub const fn dataset_id(self) -> DatasetId {
        self.dataset_id
    }

    /// The authored analysis this key names, without the dataset one run of
    /// it produced.
    ///
    /// Presentation decisions the reader makes about "the transient" are
    /// about the analysis, not about the one solve of it that happened to be
    /// on screen when they made them.
    pub const fn authored(self) -> AnalysisPresentationSource {
        self.source
    }

    /// Analysis identity used by retained visualization-document bindings.
    pub fn retained_instance_id(self) -> AnalysisInstanceId {
        match self.source {
            AnalysisPresentationSource::Prepared(id) => id,
            AnalysisPresentationSource::Legacy(id) => AnalysisInstanceId::from_namespace(
                self.dataset_id.as_uuid(),
                format!("legacy-analysis-v1/{id}").as_bytes(),
            ),
        }
    }

    pub fn order_key(self) -> (uuid::Uuid, u8, [u8; 16]) {
        let (kind, source) = match self.source {
            AnalysisPresentationSource::Prepared(id) => (0, *id.as_uuid().as_bytes()),
            AnalysisPresentationSource::Legacy(id) => {
                let mut bytes = [0_u8; 16];
                bytes[8..].copy_from_slice(&id.to_be_bytes());
                (1, bytes)
            }
        };
        (self.dataset_id.as_uuid(), kind, source)
    }

    #[cfg(feature = "engine-evidence")]
    pub fn resolve<A: AsRef<AnalysisResult<W>>, W: AsRef<RetainedWaveform>>(
        self,
        run: &SimulationRun<A>,
    ) -> Option<(usize, &A)> {
        (run.dataset_id == self.dataset_id)
            .then(|| {
                run.analyses
                    .iter()
                    .enumerate()
                    .find(|(_, analysis)| Self::new(run.dataset_id, analysis.as_ref()) == self)
            })
            .flatten()
    }
}

/// Stable identity of one source waveform representation inside an analysis.
///
/// `source_name` follows the retained waveform through reordering. `kind` and
/// `family_group` distinguish real/imaginary, magnitude/phase, and projected
/// family traces that intentionally share the same source waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TracePresentationKey {
    pub source_name: String,
    pub kind: u8,
    pub family_group: u64,
}

/// Fully dataset-bound identity of one presented waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WaveformPresentationKey {
    pub analysis: AnalysisPresentationKey,
    pub trace: TracePresentationKey,
}

/// What a result marker asserts, and therefore how it draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MarkerKind {
    /// A freeform annotation on a sample.
    #[default]
    Note,
    /// A called-out extremum or feature of the curve.
    Peak,
    /// A limit the design is measured against. Drawn as a limit line,
    /// because a spec constrains the axis position, not one curve.
    Spec,
}

impl MarkerKind {
    /// Every kind a marker may be given.
    pub const ALL: [MarkerKind; 3] = [MarkerKind::Note, MarkerKind::Peak, MarkerKind::Spec];

    /// Short label used on the chip and in the marker list.
    pub const fn label(self) -> &'static str {
        match self {
            MarkerKind::Note => "note",
            MarkerKind::Peak => "peak",
            MarkerKind::Spec => "spec",
        }
    }

    /// What choosing this kind asserts, spelled out in the edit dialog.
    pub const fn dialog_label(self) -> &'static str {
        match self {
            MarkerKind::Note => "Note — a remark about this point on the curve",
            MarkerKind::Peak => "Peak — a called-out extremum or feature",
            MarkerKind::Spec => "Spec — a limit line the design is measured against",
        }
    }

    /// A spec marker constrains the X position alone and so carries no
    /// trace value in the readout.
    pub const fn rides_a_trace(self) -> bool {
        !matches!(self, MarkerKind::Spec)
    }
}

/// A user-placed marker on a waveform strip.
///
/// The anchor is a *signal*, not one solve of it: `y` is resampled from the
/// trace every frame, so a marker survives zoom, pan, and retained-vector
/// reordering without drifting onto a different dataset or curve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultMarker {
    /// Stable per-project id. Renders as `M{id}`.
    pub id: u32,
    /// Dataset-bound identity of the strip this marker lives on.
    pub analysis: AnalysisPresentationKey,
    /// Dataset-bound identity of the trace the marker rides.
    pub anchor: WaveformPresentationKey,
    /// Display name of the anchored trace, for the marker list.
    pub trace_name: String,
    /// Anchor position in the strip's X data space.
    pub x: f64,
    pub kind: MarkerKind,
    /// Free text shown after the id on the tag. May be empty.
    pub note: String,
}

impl ResultMarker {
    pub fn validate_placement(
        analysis: AnalysisPresentationKey,
        anchor: &WaveformPresentationKey,
        x: f64,
    ) -> Result<(), String> {
        if !x.is_finite() {
            return Err("marker position must be finite".to_owned());
        }
        if anchor.analysis != analysis {
            return Err("marker anchor belongs to a different analysis or dataset".to_owned());
        }
        Ok(())
    }
}

/// Stable identity of one unit-scoped waveform pane.
///
/// The analysis key retains the exact dataset identity; the unit is the pane
/// grouping contract and remains independent of transient pane ordinals.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WavePanePresentationKey {
    pub analysis: AnalysisPresentationKey,
    pub unit: String,
}

/// One user expression trace on a waves strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExprTrace {
    /// The calculator expression as typed ("V(out)/V(in)").
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "crate::saved_output::ComplexExpressionPolicy::is_legacy"
    )]
    pub complex_policy: crate::saved_output::ComplexExpressionPolicy,
    /// Legend-chip visibility.
    #[serde(default = "default_visible")]
    pub visible: bool,
}

fn default_visible() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultExpressionGroup {
    pub analysis: AnalysisPresentationKey,
    pub traces: Vec<ExprTrace>,
}

/// Authored quick-view annotations about retained datasets. These are never
/// part of immutable solver evidence. One value crosses snapshot, save,
/// acceptance, and restoration boundaries so they cannot omit a field.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResultPresentation {
    #[serde(
        default,
        rename = "result_markers",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub markers: Vec<ResultMarker>,
    /// Highest allocated quick-marker ID, including deleted markers. Absence
    /// identifies older writers that also mixed document projections into this
    /// list; current writers own quick markers separately and always publish it.
    #[serde(
        default,
        rename = "result_marker_id_high_water",
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_marker_id_high_water"
    )]
    pub marker_id_high_water: Option<u32>,
    #[serde(
        default,
        rename = "result_log_y_panes",
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "deserialize_log_y_panes"
    )]
    pub log_y_panes: Vec<WavePanePresentationKey>,
    #[serde(
        default,
        rename = "result_expression_groups",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub expression_groups: Vec<ResultExpressionGroup>,
}

fn deserialize_marker_id_high_water<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u32>, D::Error> {
    // Missing is legacy; explicit null cannot erase allocation history or
    // opt a current marker list back into the old projection migration.
    u32::deserialize(deserializer).map(Some)
}

/// Borrowed compatibility fields and the additional durable allocation limit.
pub struct ResultFingerprintFields<'a> {
    pub markers: &'a [ResultMarker],
    pub log_y_panes: &'a [WavePanePresentationKey],
    pub expression_groups: &'a [ResultExpressionGroup],
    pub marker_history: Option<u32>,
}

/// Pane choices are a set, but project files and content fingerprints encode
/// an array. Normalize both captured and loaded content to the same order.
pub fn canonicalize_log_y_panes(panes: &mut Vec<WavePanePresentationKey>) {
    panes.sort_by(|left, right| {
        left.analysis
            .order_key()
            .cmp(&right.analysis.order_key())
            .then_with(|| left.unit.cmp(&right.unit))
    });
    panes.dedup();
}

fn deserialize_log_y_panes<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<WavePanePresentationKey>, D::Error> {
    let mut panes = Vec::deserialize(deserializer)?;
    canonicalize_log_y_panes(&mut panes);
    Ok(panes)
}

impl ResultPresentation {
    pub fn validate_markers(&self) -> Result<(), String> {
        self.marker_allocation_history()?;
        let mut ids = HashSet::new();
        for marker in &self.markers {
            ResultMarker::validate_placement(marker.analysis, &marker.anchor, marker.x)?;
            if !ids.insert(marker.id) {
                return Err(format!(
                    "result markers contain duplicate marker ID {}",
                    marker.id
                ));
            }
        }
        Ok(())
    }

    /// Older project writers could mix document projections and quick markers
    /// in one list. Keep the first occurrence of each identity and every
    /// annotation's exact payload, assigning unused IDs only to duplicates.
    /// No persisted object refers to quick IDs; their edit selectors are runtime
    /// state. The loader reports this deterministic repair to the reader.
    pub fn repair_duplicate_marker_ids(&mut self) -> Result<usize, String> {
        self.marker_allocation_history()?;
        let mut reserved = self
            .markers
            .iter()
            .map(|marker| marker.id)
            .collect::<HashSet<_>>();
        if reserved.len() == self.markers.len() {
            return Ok(0);
        }
        let mut seen = HashSet::new();
        let mut next = self
            .marker_id_high_water
            .map_or(Some(1), |highest| highest.checked_add(1));
        let mut repaired = 0;
        for marker in &mut self.markers {
            if seen.insert(marker.id) {
                continue;
            }
            let mut id = next.ok_or("result marker identity space is exhausted")?;
            while reserved.contains(&id) {
                id = id
                    .checked_add(1)
                    .ok_or("result marker identity space is exhausted")?;
            }
            marker.id = id;
            reserved.insert(id);
            next = id.checked_add(1);
            if let Some(highest) = &mut self.marker_id_high_water {
                *highest = id;
            }
            repaired += 1;
        }
        Ok(repaired)
    }

    /// Only allocation history beyond the retained markers adds new logical
    /// content. A legacy list already implies its highest retained ID, so
    /// adopting the explicit field must not make an unchanged project dirty.
    fn marker_allocation_history(&self) -> Result<Option<u32>, String> {
        let retained = self
            .markers
            .iter()
            .map(|marker| marker.id)
            .max()
            .unwrap_or(0);
        match self.marker_id_high_water {
            Some(highest) if highest < retained => {
                Err("result marker allocation history is below a retained marker ID".to_owned())
            }
            Some(highest) if highest > retained => Ok(Some(highest)),
            _ => Ok(None),
        }
    }

    /// The three annotation slices retain their existing fingerprint encoding.
    /// Only additional allocation history needs a versioned digest extension;
    /// adopting a legacy list's implied maximum does not change its identity.
    /// Exhaustive destructuring makes a new durable field require an explicit
    /// fingerprint/migration decision instead of silently bypassing dirty state.
    pub fn fingerprint_fields(&self) -> Result<ResultFingerprintFields<'_>, String> {
        let Self {
            markers,
            marker_id_high_water: _,
            log_y_panes,
            expression_groups,
        } = self;
        Ok(ResultFingerprintFields {
            markers,
            log_y_panes,
            expression_groups,
            marker_history: self.marker_allocation_history()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `viewer_document_id` and `from_viewer_document_id` are one map read in
    /// two directions, so every answer either gives must agree with the other.
    /// Three sheets share `viewer-table`, which is exactly where the second
    /// copy of the inverse had drifted onto a different one.
    #[test]
    fn the_viewer_document_map_agrees_with_itself_in_both_directions() {
        use crate::viewer_catalog::VIEWER_DOCUMENTS;

        for viewer in ResultViewer::all() {
            let Some(document_id) = viewer.viewer_document_id() else {
                assert_eq!(
                    ResultViewer::from_viewer_document_id(viewer.label()),
                    None,
                    "{viewer:?} is dataset-native and must not answer to a document id"
                );
                continue;
            };
            assert!(
                VIEWER_DOCUMENTS
                    .iter()
                    .any(|document| document.id == document_id),
                "{viewer:?} claims {document_id}, which the catalog does not publish"
            );
            let drawn_by = ResultViewer::from_viewer_document_id(document_id)
                .unwrap_or_else(|| panic!("{document_id} has no sheet"));
            assert_eq!(
                drawn_by.viewer_document_id(),
                Some(document_id),
                "{document_id} resolves to {drawn_by:?}, which renders something else"
            );
        }
    }

    #[test]
    fn marker_history_field_is_optional_but_not_nullable_or_truncated() {
        let legacy: ResultPresentation = serde_json::from_str("{}").unwrap();
        assert_eq!(legacy.marker_id_high_water, None);
        assert_eq!(serde_json::to_value(legacy).unwrap(), serde_json::json!({}));
        for bad in ["null", "-1", "4294967296", "1.5", "\"7\""] {
            let text = format!("{{\"result_marker_id_high_water\":{bad}}}");
            assert!(
                serde_json::from_str::<ResultPresentation>(&text).is_err(),
                "{bad}"
            );
        }
        let exhausted: ResultPresentation =
            serde_json::from_str("{\"result_marker_id_high_water\":4294967295}").unwrap();
        assert_eq!(exhausted.marker_id_high_water, Some(u32::MAX));
        exhausted.validate_markers().unwrap();
        assert_eq!(
            serde_json::to_value(exhausted).unwrap(),
            serde_json::json!({"result_marker_id_high_water": u32::MAX})
        );
    }

    #[test]
    fn duplicate_marker_recovery_respects_published_allocation_history() {
        let key = serde_json::json!({
            "dataset_id": "b3c6b2be-c997-4f5d-a06e-714071283df5", "source": {"Legacy": 7}
        });
        let marker = serde_json::json!({
            "id": 3, "analysis": key,
            "anchor": {"analysis": key, "trace": {"source_name": "V(out)", "kind": 0, "family_group": 0}},
            "trace_name": "V(out)", "x": 0.5, "kind": "Note", "note": "Retained"
        });
        let mut presentation: ResultPresentation = serde_json::from_value(serde_json::json!({
            "result_markers": [marker.clone(), marker], "result_marker_id_high_water": 10
        }))
        .unwrap();
        let mut inconsistent = presentation.clone();
        inconsistent.marker_id_high_water = Some(2);
        assert!(inconsistent.validate_markers().is_err());
        assert!(inconsistent.repair_duplicate_marker_ids().is_err());
        let mut exhausted = presentation.clone();
        exhausted.marker_id_high_water = Some(u32::MAX);
        assert!(exhausted.repair_duplicate_marker_ids().is_err());
        assert_eq!(exhausted.markers[1].id, 3);
        assert_eq!(presentation.repair_duplicate_marker_ids().unwrap(), 1);
        assert_eq!(
            presentation
                .markers
                .iter()
                .map(|marker| marker.id)
                .collect::<Vec<_>>(),
            [3, 11]
        );
        assert_eq!(presentation.marker_id_high_water, Some(11));
        presentation.validate_markers().unwrap();
        assert_eq!(presentation.repair_duplicate_marker_ids().unwrap(), 0);
    }

    #[test]
    fn legacy_presentation_wire_contract_and_expression_defaults_are_preserved() {
        let key = serde_json::json!({
            "dataset_id": "b3c6b2be-c997-4f5d-a06e-714071283df5",
            "source": {"Legacy": 7}
        });
        let mut wire = serde_json::json!({
            "result_markers": [{
                "id": 3, "analysis": key,
                "anchor": {"analysis": key, "trace": {
                    "source_name": "V(out)", "kind": 0, "family_group": 0
                }},
                "trace_name": "V(out)", "x": 0.125, "kind": "Peak", "note": "Settling"
            }],
            "result_log_y_panes": [{"analysis": key, "unit": "V"}],
            "result_expression_groups": [{"analysis": key, "traces": [{"text": "V(out)*2"}]}]
        });
        let presentation: ResultPresentation = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(
            presentation.markers[0].analysis.authored(),
            AnalysisPresentationSource::Legacy(7)
        );
        assert!(presentation.expression_groups[0].traces[0].visible);
        wire["result_expression_groups"][0]["traces"][0]["visible"] = true.into();
        assert_eq!(serde_json::to_value(&presentation).unwrap(), wire);

        // Older projects wrote this set in hash iteration order. Normalize
        // the accepted file as well as the working snapshot on restoration.
        wire["result_log_y_panes"] = serde_json::json!([
            {"analysis": key, "unit": "V"},
            {"analysis": key, "unit": "A"},
            {"analysis": key, "unit": "V"}
        ]);
        let restored: ResultPresentation = serde_json::from_value(wire).unwrap();
        assert_eq!(
            restored
                .log_y_panes
                .iter()
                .map(|pane| pane.unit.as_str())
                .collect::<Vec<_>>(),
            ["A", "V"]
        );

        let prepared = serde_json::json!({
            "dataset_id": key["dataset_id"],
            "source": {"Prepared": "ab428908-f301-423c-bdb7-4cb4b1feef6f"}
        });
        let identity: AnalysisPresentationKey = serde_json::from_value(prepared.clone()).unwrap();
        assert_eq!(serde_json::to_value(identity).unwrap(), prepared);
        let empty: ResultPresentation = serde_json::from_str("{}").unwrap();
        assert_eq!(serde_json::to_value(empty).unwrap(), serde_json::json!({}));
    }
}

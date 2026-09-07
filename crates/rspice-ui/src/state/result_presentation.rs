//! Durable presentation identities and annotations for retained results.
//!
//! Shared by project I/O and the workbench. Rendering, session caches, and
//! editor state remain in the workbench; none can redefine this wire contract.

use serde::{Deserialize, Serialize};

use super::{AnalysisResult, SimulationRun};
use crate::product::{AnalysisInstanceId, DatasetId};

/// Stable identity of one retained analysis within one immutable dataset.
///
/// Current results use the exact prepared-task identity. A legacy result has
/// no such provenance, so its run-local analysis id is safe only when paired
/// with the immutable dataset id. Neither representation depends on vector
/// position, which prevents presentation state from moving to another
/// analysis when retained results are reordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct AnalysisPresentationKey {
    dataset_id: DatasetId,
    source: AnalysisPresentationSource,
}

/// Which authored analysis a result came from, independent of the dataset it
/// was solved into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum AnalysisPresentationSource {
    Prepared(AnalysisInstanceId),
    Legacy(u64),
}

impl AnalysisPresentationKey {
    pub(crate) fn new(dataset_id: DatasetId, analysis: &AnalysisResult) -> Self {
        let source = analysis.provenance().map_or(
            AnalysisPresentationSource::Legacy(analysis.id),
            |provenance| AnalysisPresentationSource::Prepared(provenance.source_instance_id()),
        );
        Self { dataset_id, source }
    }

    pub(crate) const fn dataset_id(self) -> DatasetId {
        self.dataset_id
    }

    /// The authored analysis this key names, without the dataset one run of
    /// it produced.
    ///
    /// Presentation decisions the reader makes about "the transient" are
    /// about the analysis, not about the one solve of it that happened to be
    /// on screen when they made them.
    pub(crate) const fn authored(self) -> AnalysisPresentationSource {
        self.source
    }

    /// Analysis identity used by retained visualization-document bindings.
    pub(crate) fn retained_instance_id(self) -> AnalysisInstanceId {
        match self.source {
            AnalysisPresentationSource::Prepared(id) => id,
            AnalysisPresentationSource::Legacy(id) => AnalysisInstanceId::from_namespace(
                self.dataset_id.as_uuid(),
                format!("legacy-analysis-v1/{id}").as_bytes(),
            ),
        }
    }

    pub(crate) fn order_key(self) -> (uuid::Uuid, u8, [u8; 16]) {
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

    pub(crate) fn resolve(self, run: &SimulationRun) -> Option<(usize, &AnalysisResult)> {
        (run.dataset_id == self.dataset_id)
            .then(|| {
                run.analyses
                    .iter()
                    .enumerate()
                    .find(|(_, analysis)| Self::new(run.dataset_id, analysis) == self)
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
pub(crate) struct TracePresentationKey {
    pub(crate) source_name: String,
    pub(crate) kind: u8,
    pub(crate) family_group: u64,
}

/// Fully dataset-bound identity of one presented waveform.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct WaveformPresentationKey {
    pub(crate) analysis: AnalysisPresentationKey,
    pub(crate) trace: TracePresentationKey,
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
    /// Legend-chip visibility.
    #[serde(default = "default_visible")]
    pub visible: bool,
}

fn default_visible() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ResultExpressionGroup {
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
    pub(crate) expression_groups: Vec<ResultExpressionGroup>,
}

/// Pane choices are a set, but project files and content fingerprints encode
/// an array. Normalize both captured and loaded content to the same order.
pub(crate) fn canonicalize_log_y_panes(panes: &mut Vec<WavePanePresentationKey>) {
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
    /// Existing lifecycle fingerprints encode these as three tuple elements.
    /// Preserve that encoding: generated-input identities also use the registry.
    /// Exhaustive destructuring makes a new durable field require an explicit
    /// fingerprint/migration decision instead of silently bypassing dirty state.
    pub(crate) fn fingerprint_fields(
        &self,
    ) -> (
        &[ResultMarker],
        &[WavePanePresentationKey],
        &[ResultExpressionGroup],
    ) {
        let Self {
            markers,
            log_y_panes,
            expression_groups,
        } = self;
        (markers, log_y_panes, expression_groups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

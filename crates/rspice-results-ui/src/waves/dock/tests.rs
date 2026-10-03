//! Cursor dock geometry and marker-label regressions.
use super::*;
use crate::session::DocumentMarker;
use crate::waves::TraceKind;
use rspice_app_types::product::{DatasetId, ResultDocumentId};
use rspice_results::{
    analysis_result::AnalysisResult,
    analysis_type::AnalysisType,
    result_presentation::{ResultMarker, TracePresentationKey},
    waveform::RetainedWaveform,
};
fn retained_entity_id<T: serde::de::DeserializeOwned>(serial: u64) -> T {
    T::deserialize(serde::de::value::U64Deserializer::<serde::de::value::Error>::new(serial))
        .expect("a non-zero entity serial")
}

#[test]
fn cursor_and_marker_columns_split_at_normal_desktop_widths() {
    assert!(!readout_columns_side_by_side(679.0, true, true));
    assert!(readout_columns_side_by_side(680.0, true, true));
    assert!(!readout_columns_side_by_side(900.0, true, false));
    assert!(!readout_columns_side_by_side(900.0, false, true));
}

#[test]
fn a_marker_tag_names_the_note_only_when_there_is_one() {
    let analysis_result =
        AnalysisResult::<RetainedWaveform>::new(1, AnalysisType::Transient, "marker analysis", 0.0);
    let analysis = AnalysisPresentationKey::new(DatasetId::new(), &analysis_result);
    let mut marker = ResultMarker {
        id: 3,
        analysis,
        anchor: WaveformPresentationKey {
            analysis,
            trace: TracePresentationKey {
                source_name: "V(out)".to_owned(),
                kind: TraceKind::Value as u8,
                family_group: 0,
            },
        },
        trace_name: "V(out)".to_owned(),
        x: 0.0,
        kind: MarkerKind::Note,
        note: String::new(),
    };
    assert_eq!(marker_label(MarkerView::Quick(&marker)), "M3");

    marker.note = "  settling  ".to_owned();
    assert_eq!(marker_label(MarkerView::Quick(&marker)), "M3 · settling");

    // The two stores allocate independently, so a retained marker's tag can
    // never be mistaken for the quick marker that happens to share its number.
    let retained = DocumentMarker {
        document_id: ResultDocumentId::new(),
        pane_id: retained_entity_id(1),
        retained_id: retained_entity_id(3),
        analysis,
        anchor: marker.anchor.clone(),
        trace_name: "V(out)".to_owned(),
        x: 0.0,
        kind: MarkerKind::Note,
        note: "  ringing  ".to_owned(),
    };
    assert_eq!(
        marker_label(MarkerView::Document(&retained)),
        "D3 · ringing"
    );
}

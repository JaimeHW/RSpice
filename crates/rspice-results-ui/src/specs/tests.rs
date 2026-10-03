//! Specification geometry, accessible readouts and summary-band rendering.

#[cfg(not(target_arch = "wasm32"))]
use super::test_support::painted_spans;
use super::*;
use rspice_results::{
    analysis_type::AnalysisType,
    run::ExecutionTarget,
    specification::{SpecPointScope, report::result_row},
};
type AnalysisResult =
    rspice_results::analysis_result::AnalysisResult<crate::waveform::WaveformData>;
type Run = SimulationRun<AnalysisResult>;

/// The pane widths this document is drawn in, from the narrowest it will fit in
/// at all to the widest window.
///
/// Pane widths, not viewport widths. The results workspace keeps rails either
/// side of this document, so at the 1000-point gate it is given around 720
/// points and a claim measured against 1000 is a claim about a surface the
/// table is never drawn on. Sweeping the range rather than naming the two gates
/// means no measurement of that inset can go stale here: whatever the rails
/// take, the pane that is left is in this sweep.
///
/// [`super::table_minimum_width`] is the floor because below it the table's own
/// columns no longer fit, which the document refuses on its own terms.
#[cfg(not(target_arch = "wasm32"))]
fn drawn_pane_widths() -> Vec<f32> {
    let floor = super::table_minimum_width();
    let mut widths = Vec::new();
    let mut width = floor;
    while width <= 1600.0 {
        widths.push(width);
        width += 37.0;
    }
    widths.push(1600.0);
    widths
}

/// The workspace with one immutable run whose two bounded limits went
/// unmeasured, on the specification viewer.
///
/// That is the state the band was clipped in — `0 / 2 pass · 2 unavailable ·
/// immutable · dataset …`, the longest verdict the band has — and it is reached
/// by a run that retains no measurement at all rather than by a contrived
/// string.
#[cfg(not(target_arch = "wasm32"))]
fn two_unmeasured_limits_on_an_immutable_run() -> (Run, Vec<SpecEntry>) {
    let mut run = Run::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.success = true;
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "ac", 0.0));
    let specs = vec![
        SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(40.0),
            max: None,
            unit: "dB".to_owned(),
            scope: SpecPointScope::AllPoints,
        },
        SpecEntry {
            measurement: "bandwidth_3db".to_owned(),
            expression: "bw V(out)".to_owned(),
            min: Some(1.0e6),
            max: None,
            unit: "Hz".to_owned(),
            scope: SpecPointScope::AllPoints,
        },
    ];
    (run, specs)
}

/// The document drawn into a pane exactly `pane` points wide.
#[cfg(not(target_arch = "wasm32"))]
fn document_in_pane(pane: f32) -> Vec<(String, egui::Rect, egui::Rect)> {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let (run, specs) = two_unmeasured_limits_on_an_immutable_run();
    let mut output = None;
    // Two passes: the document resolves its content width against the scrollbar
    // track it reserves, which it only knows on a second pass.
    for _ in 0..2 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(pane, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        super::show(ui, Some(&run), &specs, None);
                    });
            },
        ));
    }
    painted_spans(&output.expect("the document drew a frame"))
}

#[test]
fn matrix_rows_follow_desktop_and_touch_control_contracts() {
    assert_eq!(spec_table_row_height(25.0), 28.0);
    assert_eq!(spec_table_row_height(32.0), 32.0);
    assert_eq!(spec_table_row_height(44.0), 44.0);
    assert_eq!(spec_table_row_height(48.0), 48.0);
}

#[test]
fn seven_column_geometry_is_stable_and_does_not_depend_on_row_state() {
    assert_eq!(super::SPEC_COLUMNS.len(), 7);
    assert_eq!(table_width(), 1042.0);
    // And the table can be laid out in a pane narrower than that, which at
    // both gate widths is what the pane is.
    assert!(super::table_minimum_width() < 1042.0);
}

#[test]
fn row_accessibility_carries_every_visible_engineering_value() {
    let mut run = Run::new(9, 0.0, ExecutionTarget::LocalDesktop);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.5)]),
    );
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(1.0),
        max: Some(2.0),
        unit: "V/V".to_owned(),
        scope: SpecPointScope::AllPoints,
    };
    let row = result_row(&run, "gain".to_owned(), Some(&spec));

    let label = row_accessibility_label(&row);

    assert!(label.contains("expression max V(out)"));
    assert!(label.contains(&format!("value {}", super::value_text(&row))));
    assert!(label.contains(&format!("limit {}", row.limit)));
    assert!(label.contains(&format!("margin {}", super::margin_text(&row))));
    assert!(label.contains("status pass"));
}

/// The band states its verdict head first, inside the room it is given.
///
/// The verdict was right-aligned into whatever room the title left, so in the
/// pane this document actually gets at the 1000-point gate — around 720 points,
/// not 1000 — `0 / 2 pass · 2 unavailable · immutable · dataset <uuid>` began
/// off the left edge of that room and arrived as `/ 2 pass · …`. The pass count
/// is the one thing the band exists to state and it was the first thing cut.
///
/// Three claims, each of which fails on its own: the verdict starts where its
/// room starts, nothing is lost from its head, and the whole line fits. The
/// first two fail on the alignment; the third fails on the length, which is why
/// the dataset identity is elided to the eight characters every other surface
/// elides it to.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_summary_band_states_its_verdict_head_first_inside_its_pane() {
    for pane in drawn_pane_widths() {
        let spans = document_in_pane(pane);
        let (text, rect, clip) = spans
            .iter()
            .find(|(text, _, _)| text.starts_with("0 / 2 pass"))
            .unwrap_or_else(|| {
                panic!(
                    "the band paints its verdict in a {pane:.0}-point pane; it painted {:?}",
                    spans
                        .iter()
                        .map(|(text, _, _)| text.as_str())
                        .filter(|text| text.contains("pass"))
                        .collect::<Vec<_>>()
                )
            });
        assert!(
            text.contains("2 unavailable") && text.contains("immutable"),
            "the band's longest verdict is the case: {text:?}"
        );
        assert!(
            rect.left() >= clip.left() - 0.5,
            "in a {pane:.0}-point pane the verdict starts {:.0} points left of the room it is \
             given, so the pass count is cut off: {text:?}",
            clip.left() - rect.left()
        );
        assert!(
            rect.left() <= clip.left() + 0.5,
            "in a {pane:.0}-point pane the verdict starts {:.0} points into the room it is given \
             rather than at the start of it: set from the right, what overflows is lost from the \
             head, and the head is the count: {text:?}",
            rect.left() - clip.left()
        );
        assert!(
            rect.right() <= clip.right() + 0.5,
            "in a {pane:.0}-point pane the verdict runs {:.0} points past the room it is given: \
             {text:?}",
            rect.right() - clip.right()
        );
    }
}

/// Every column is inside the pane the document is drawn in.
///
/// This fed `spec_columns` the *gate* width — 1000 and 1600 — and the document
/// is never that wide: the results workspace keeps rails either side of it, so
/// at the 1000-point gate the table has around 720 points and was being
/// measured against 1000. The sweep is over pane widths now, from the narrowest
/// the table fits in at all up to the widest window, so no measurement of what
/// the rails take can go stale here.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn every_column_is_inside_the_pane_the_document_is_drawn_in() {
    for pane in drawn_pane_widths() {
        let widths = super::spec_columns(pane);
        let total: f32 = widths.iter().sum();
        assert!(
            total <= pane + 0.5,
            "in a {pane:.0}-point pane the columns take {total:.0}: {widths:?}"
        );
        // The verdict never gives up a point: it is one word, it does not
        // elide, and it is the answer. The prose columns shrink instead.
        assert!(
            (widths[6] - super::SPEC_COLUMNS[6].0).abs() < 0.5,
            "the status column keeps its width in a {pane:.0}-point pane"
        );
        for (width, (_, floor)) in widths.iter().zip(super::SPEC_COLUMNS) {
            assert!(
                *width >= floor - 0.5,
                "a column shrank to {width:.0}, below the {floor:.0} it can be read at"
            );
        }
    }
    // And where nothing squeezes them, every column has what it wants.
    let ample = super::spec_columns(super::table_width() + 200.0);
    assert_eq!(ample, super::SPEC_COLUMNS.map(|(want, _)| want));
}

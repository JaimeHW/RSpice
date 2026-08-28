//! Tests for the specification table: its rows, its geometry, and the band
//! that heads it.
//!
//! Split from `specs.rs` into the sibling file the crate uses for a module's
//! own tests, so the table and the evidence that it holds can each grow.

use super::AppState;
use super::{
    SpecDraft, SpecResultStatus, apply_drafts, result_row, result_rows, row_accessibility_label,
    signed_margin, spec_table_row_height, summarize_rows, table_width,
};
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisResultProvenance,
    AnalysisType, FloquetOrbitKindEvidence, FloquetSpectrumEvidence,
    FloquetStabilityVerdictEvidence, SimulationRun, SpecEntry,
};

/// A hop from the studio's Requirements page names one limit. This table
/// and that page read the same selection, so arriving here has to mark
/// the row the reader asked for rather than leaving them to find it.
#[test]
fn the_table_marks_the_limit_a_hop_carried_into_it() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "ac").with_measurements(vec![
            rspice_core::MeasureResult::success("gain_dc", 44.0),
            rspice_core::MeasureResult::success("bandwidth_3db", 1.0e6),
        ]),
    );
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.workspace.specs = vec![
        SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: "db20(V(out))".to_owned(),
            min: Some(40.0),
            max: None,
            unit: "dB".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        SpecEntry {
            measurement: "bandwidth_3db".to_owned(),
            expression: "bw(V(out))".to_owned(),
            min: Some(1.0),
            max: None,
            unit: "Hz".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
    ];
    // Matched the way the studio matches: a measurement name is one
    // identity however it was typed.
    state.workbench.selected_specification = Some("BANDWIDTH_3DB".to_owned());

    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let nodes = ctx
        .run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1400.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| super::show(ui, &mut state));
            },
        )
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    let selected: Vec<&str> = nodes
        .iter()
        .filter(|(_, node)| {
            node.role() == egui::accesskit::Role::Row && node.is_selected() == Some(true)
        })
        .filter_map(|(_, node)| node.label())
        .collect();
    assert_eq!(
        selected.len(),
        1,
        "exactly one row is the one the hop carried"
    );
    assert!(
        selected[0].starts_with("Measurement bandwidth_3db;"),
        "and it is the limit the studio named, not the first row: {selected:?}"
    );
}

#[test]
fn matrix_rows_follow_desktop_and_touch_control_contracts() {
    assert_eq!(spec_table_row_height(25.0), 28.0);
    assert_eq!(spec_table_row_height(32.0), 32.0);
    assert_eq!(spec_table_row_height(44.0), 44.0);
    assert_eq!(spec_table_row_height(48.0), 48.0);
}

#[test]
fn legacy_specification_deserialization_does_not_invent_an_expression() {
    let spec: SpecEntry =
        serde_json::from_str(r#"{"measurement":"gain","min":1.0,"max":2.0,"unit":"V/V"}"#)
            .expect("legacy specification remains readable");

    assert!(spec.expression.is_empty());
    assert!(
        !serde_json::to_string(&spec)
            .expect("migrated specification serializes")
            .contains("expression")
    );
}

#[test]
fn signed_margin_is_positive_inside_and_negative_outside_each_bound_shape() {
    let two_sided = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(10.0),
        max: Some(20.0),
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    assert_eq!(signed_margin(&two_sided, 12.0), Some(2.0));
    assert_eq!(signed_margin(&two_sided, 22.5), Some(-2.5));

    let minimum = SpecEntry {
        max: None,
        ..two_sided.clone()
    };
    assert_eq!(signed_margin(&minimum, 13.0), Some(3.0));
    let maximum = SpecEntry {
        min: None,
        max: Some(20.0),
        ..two_sided
    };
    assert_eq!(signed_margin(&maximum, 18.0), Some(2.0));
}

#[test]
fn bounded_row_uses_the_exact_worst_retained_source_and_corner() {
    let mut run = SimulationRun::new(4);
    let source = AnalysisInstanceId::new();
    for (value, corner) in [(0.8, "TT · 27 °C"), (1.2, "SS · 125 °C")] {
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Corner, corner)
                .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
                    member_measurements: Vec::new(),
                    x_values: vec![1.0],
                    x_label: "corner".to_owned(),
                    x_unit: String::new(),
                    temperatures_c: vec![27.0],
                    corner_labels: vec![corner.to_owned()],
                    failed_corners: 0,
                })
                .with_provenance(
                    AnalysisResultProvenance::new(
                        source,
                        ObjectRevision::INITIAL,
                        ContentDigest::from_bytes([0x51; 32]),
                        Vec::new(),
                    )
                    .expect("corner provenance is valid"),
                )
                .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)]),
        );
    }
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(0.0),
        max: Some(1.0),
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    let row = result_row(&run, "gain".to_owned(), Some(&spec));

    assert_eq!(row.value, Some(1.2));
    assert!(
        row.margin
            .is_some_and(|margin| (margin + 0.2).abs() < 1.0e-12)
    );
    assert_eq!(row.status, SpecResultStatus::Fail);
    assert_eq!(row.source_analysis_index, Some(1));
    assert_eq!(row.worst_corner.as_deref(), Some("SS · 125 °C"));
}

#[test]
fn ambiguous_unbound_measurement_never_selects_an_arbitrary_value() {
    let mut run = SimulationRun::new(5);
    for value in [1.0, 2.0] {
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_measurements(vec![rspice_core::MeasureResult::success("delay", value)]),
        );
    }

    let row = result_row(&run, "delay".to_owned(), None);

    assert_eq!(row.status, SpecResultStatus::Invalid);
    assert_eq!(row.value, None);
    assert_eq!(row.source_analysis_index, None);
    assert!(row.detail.contains("2 retained analyses"));
}

#[test]
fn result_rows_consume_durable_pss_scalars_without_a_measurement_copy() {
    let payload = AnalysisResultPayload::PssFloquet {
        period_s: Some(2.0),
        fundamental_frequency_hz: Some(0.5),
        iterations: Some(2),
        residual_norm: Some(1.0e-12),
        multipliers: Vec::new(),
        floquet_evidence: FloquetSpectrumEvidence::NoDynamicModes,
        orbit_kind: FloquetOrbitKindEvidence::Driven,
        trivial_multiplier_index: None,
        stability_verdict: FloquetStabilityVerdictEvidence::Stable,
    };
    let mut run = SimulationRun::new(6);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Pss, "PSS").with_result_payload(payload));
    let spec = SpecEntry {
        measurement: "pss_mode_count".to_owned(),
        expression: "authenticated Floquet spectrum order".to_owned(),
        min: Some(0.0),
        max: Some(0.0),
        unit: "count".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    let row = result_row(&run, spec.measurement.clone(), Some(&spec));

    assert_eq!(row.value, Some(0.0));
    assert_eq!(row.status, SpecResultStatus::Pass);
    assert_eq!(row.source_analysis_index, Some(0));
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
fn bounded_missing_measurements_remain_in_the_requirement_denominator() {
    let run = SimulationRun::new(6);
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(1.0),
        max: None,
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    let rows = result_rows(&run, &[spec]);
    let summary = summarize_rows(&rows);

    assert_eq!(summary.bounded, 1);
    assert_eq!(summary.passing, 0);
    assert_eq!(summary.failures, 0);
    assert_eq!(summary.unavailable, 1);
    assert_eq!(rows[0].status, SpecResultStatus::Missing);
}

#[test]
fn bounded_cross_analysis_name_collision_fails_closed_without_lineage() {
    let mut run = SimulationRun::new(7);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 2.0)]),
    );
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(0.0),
        max: Some(3.0),
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    let row = result_row(&run, "gain".to_owned(), Some(&spec));

    assert_eq!(row.status, SpecResultStatus::Invalid);
    assert_eq!(row.value, None);
    assert_eq!(row.source_analysis_index, None);
    assert!(row.detail.contains("different or unproven source lineages"));
}

#[test]
fn rows_preserve_declared_contract_order_then_first_retained_order() {
    let mut run = SimulationRun::new(8);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_measurements(vec![
            rspice_core::MeasureResult::success("zeta", 1.0),
            rspice_core::MeasureResult::success("beta", 2.0),
            rspice_core::MeasureResult::success("alpha", 3.0),
        ]),
    );
    let specs = [
        SpecEntry {
            measurement: "zeta".to_owned(),
            expression: "max V(z)".to_owned(),
            min: None,
            max: Some(2.0),
            unit: "V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        SpecEntry {
            measurement: "alpha".to_owned(),
            expression: "max V(a)".to_owned(),
            min: None,
            max: Some(4.0),
            unit: "V".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
    ];

    let names: Vec<_> = result_rows(&run, &specs)
        .into_iter()
        .map(|row| row.measurement)
        .collect();

    assert_eq!(names, ["zeta", "alpha", "beta"]);
}

#[test]
fn row_accessibility_carries_every_visible_engineering_value() {
    let mut run = SimulationRun::new(9);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.5)]),
    );
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "max V(out)".to_owned(),
        min: Some(1.0),
        max: Some(2.0),
        unit: "V/V".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    let row = result_row(&run, "gain".to_owned(), Some(&spec));

    let label = row_accessibility_label(&row);

    assert!(label.contains("expression max V(out)"));
    assert!(label.contains(&format!("value {}", super::value_text(&row))));
    assert!(label.contains(&format!("limit {}", row.limit)));
    assert!(label.contains(&format!("margin {}", super::margin_text(&row))));
    assert!(label.contains("status pass"));
}

#[test]
fn applying_drafts_commits_the_active_plan_owned_specification() {
    let mut state = crate::workbench::AppState::default();
    let plan_id = state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source_revision = state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .revision();
    state.ui.results.spec_drafts = Some(vec![SpecDraft {
        requirement_key: "REQ-GAIN-001".to_owned(),
        requirement_name: "Closed-loop gain window".to_owned(),
        measurement: "gain_db".to_owned(),
        expression: "max db(V(out))".to_owned(),
        comparison: super::ComparisonDraftKind::Range,
        primary_limit: "20".to_owned(),
        secondary_limit: "40".to_owned(),
        unit: "dB".to_owned(),
        ..Default::default()
    }]);

    assert!(apply_drafts(&mut state));

    let owned = state
        .workspace
        .plan_data(plan_id)
        .expect("active plan payload");
    assert_eq!(owned.specs.len(), 1);
    assert_eq!(owned.specs[0].measurement, "gain_db");
    assert_eq!(owned.specs[0].expression, "max db(V(out))");
    assert_eq!(owned.specification_definitions.len(), 1);
    assert_eq!(
        owned.specification_definitions[0].requirement_key,
        "REQ-GAIN-001"
    );
    assert_eq!(
        owned.specification_definitions[0].comparison,
        crate::state::SpecificationComparison::Range {
            minimum: 20.0,
            maximum: 40.0,
        }
    );
    assert_eq!(state.workspace.specs, owned.specs);
    assert!(
        state
            .sim_setup
            .stable_analysis_plan()
            .expect("default plan")
            .revision()
            > source_revision
    );
    assert!(state.ui.results.spec_drafts.is_none());
}

#[test]
fn governed_editor_preserves_identity_source_waiver_producer_and_equality_kind() {
    use crate::state::workspace::{SpecificationSource, SpecificationWaiver};

    let mut state = crate::workbench::AppState::default();
    let plan = state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan");
    let plan_id = plan.id();
    let producer = plan.instances()[0].id();
    let entry = SpecEntry {
        measurement: "offset".to_owned(),
        expression: "avg V(out)".to_owned(),
        min: Some(-0.1),
        max: Some(0.1),
        unit: "V".to_owned(),
        scope: crate::state::SpecPointScope::Nominal,
    };
    let mut definition = crate::state::SpecificationDefinition::from_legacy(plan_id, 0, &entry);
    definition.requirement_key = "REQ-OFFSET-1".to_owned();
    definition.comparison = crate::state::SpecificationComparison::EqualWithin {
        target: 0.0,
        tolerance: 0.1,
    };
    definition.guard_band = Some(0.01);
    definition.role = crate::state::SpecificationRole::Review;
    definition.producing_analysis = Some(producer);
    definition.source = Some(SpecificationSource {
        logical_path: "requirements/analog.csv".to_owned(),
        row: 12,
        imported_revision: "req-19".to_owned(),
        source_digest: ContentDigest::from_bytes([0x71; 32]),
    });
    definition.waiver = Some(SpecificationWaiver {
        reference: "WVR-7".to_owned(),
        owner: "Analog lead".to_owned(),
        rationale: "Characterization disposition".to_owned(),
    });
    let original = definition.clone();
    state
        .workspace
        .replace_active_specification_definitions(plan_id, vec![definition]);

    super::open_editor(&mut state);
    state.ui.results.spec_drafts.as_mut().unwrap()[0].requirement_name =
        "Input-referred offset".to_owned();
    assert!(apply_drafts(&mut state));

    let retained = &state
        .workspace
        .plan_data(plan_id)
        .unwrap()
        .specification_definitions[0];
    assert_eq!(retained.id, original.id);
    assert_eq!(retained.requirement_name, "Input-referred offset");
    assert_eq!(retained.comparison, original.comparison);
    assert_eq!(retained.source, original.source);
    assert_eq!(retained.waiver, original.waiver);
    assert_eq!(retained.producing_analysis, Some(producer));
    assert_eq!(retained.scope, crate::state::SpecPointScope::Nominal);
}

/// One spelling of a bound, wherever it is printed.
///
/// This table formatted its own limits with the plot-axis formatter, so a
/// megahertz requirement read `≥ 1.000 M Hz` here while the studio page
/// that authored it read `≥ 1M Hz` — one number, two spellings, and the one
/// here is not how a limit is written beside a unit anywhere in the field.
///
/// The row is asserted, not only the spelling: pinning `limit_text` alone
/// proves the studio's spelling exists, not that this table prints it, and the
/// defect was that the table did not. Both the measured row and the row for a
/// measurement the dataset never retained carry it, because a bound is a bound
/// whether or not anything met it.
#[test]
fn a_limit_is_spelled_the_same_here_as_on_the_page_that_authored_it() {
    let spec = crate::state::SpecEntry {
        measurement: "bandwidth_3db".to_owned(),
        expression: String::new(),
        min: Some(1.0e6),
        max: None,
        unit: "Hz".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };
    assert_eq!(spec.limit_text(), "\u{2265} 1M Hz");
    assert!(
        !spec.limit_text().contains("Meg"),
        "`Meg` is the deck's prefix and belongs in a deck, not beside a unit"
    );

    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "ac").with_measurements(vec![
            rspice_core::MeasureResult::success("bandwidth_3db", 2.0e6),
        ]),
    );
    let measured = result_row(&run, "bandwidth_3db".to_owned(), Some(&spec));
    assert_eq!(
        measured.limit,
        spec.limit_text(),
        "the row prints the bound the page that authored it prints"
    );
    let unmeasured = result_row(&run, "slew_rate".to_owned(), Some(&spec));
    assert_eq!(
        unmeasured.status,
        SpecResultStatus::Missing,
        "no retained analysis evaluated it"
    );
    assert_eq!(unmeasured.limit, spec.limit_text());
}

// ---------------------------------------------------- the band over the table

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
fn two_unmeasured_limits_on_an_immutable_run() -> AppState {
    use crate::state::SimulationRunLifecycle;

    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.success = true;
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Ac, "ac"));
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.workspace.specs = vec![
        SpecEntry {
            measurement: "gain_dc".to_owned(),
            expression: "max V(out)".to_owned(),
            min: Some(40.0),
            max: None,
            unit: "dB".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        SpecEntry {
            measurement: "bandwidth_3db".to_owned(),
            expression: "bw V(out)".to_owned(),
            min: Some(1.0e6),
            max: None,
            unit: "Hz".to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
    ];
    state
}

/// Every painted text span, as `(text, its own rectangle, the clip it was
/// painted into)`.
#[cfg(not(target_arch = "wasm32"))]
fn painted_spans(output: &egui::FullOutput) -> Vec<(String, egui::Rect, egui::Rect)> {
    fn walk(
        shape: &egui::epaint::Shape,
        clip: egui::Rect,
        out: &mut Vec<(String, egui::Rect, egui::Rect)>,
    ) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.job.text.clone(),
                egui::Rect::from_min_size(text.pos, text.galley.size()),
                clip,
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, out);
                }
            }
            _ => {}
        }
    }
    let mut spans = Vec::new();
    for shape in &output.shapes {
        walk(&shape.shape, shape.clip_rect, &mut spans);
    }
    spans
}

/// The document drawn into a pane exactly `pane` points wide.
#[cfg(not(target_arch = "wasm32"))]
fn document_in_pane(pane: f32) -> Vec<(String, egui::Rect, egui::Rect)> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = two_unmeasured_limits_on_an_immutable_run();
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
                    .show(ctx, |ui| super::show(ui, &mut state));
            },
        ));
    }
    painted_spans(&output.expect("the document drew a frame"))
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

// ------------------------------------------- one judge, one resolved contract

use crate::product::SimulationPlanId;
use crate::state::{
    AnalysisResultPvtPoint, AnalysisResultSourceDomain, PreparedRunReceipt, PreparedRunTaskReceipt,
    PreparedSourceCheckReceipt, PreparedSpecification, SimulationRunLifecycle, SpecPointScope,
    SpecificationComparison, SpecificationDefinition,
};

/// A governed requirement, spelled the way the studio's editor spells one.
fn requirement(
    measurement: &str,
    comparison: SpecificationComparison,
    guard_band: Option<f64>,
    producing_analysis: Option<AnalysisInstanceId>,
    scope: SpecPointScope,
) -> SpecificationDefinition {
    let projection = SpecEntry {
        measurement: measurement.to_owned(),
        expression: format!("param={measurement}"),
        min: None,
        max: None,
        unit: "dB".to_owned(),
        scope: scope.clone(),
    };
    let mut definition =
        SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, &projection);
    definition.requirement_key = "REQ-GAIN-1".to_owned();
    definition.comparison = comparison;
    definition.guard_band = guard_band;
    definition.producing_analysis = producing_analysis;
    definition.scope = scope;
    definition
}

/// A dispatched run whose receipt froze `definitions`, plus the producing task
/// identity every attributed analysis in it is authored by.
fn prepared_run(definitions: Vec<SpecificationDefinition>) -> (SimulationRun, AnalysisInstanceId) {
    let task_id = AnalysisInstanceId::new();
    let task = PreparedRunTaskReceipt::new(
        task_id,
        ObjectRevision::INITIAL,
        Vec::new(),
        2,
        ContentDigest::from_bytes([0x72; 32]),
    )
    .expect("task receipt");
    let receipt = PreparedRunReceipt::new_with_project_model_sources_and_specifications(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([0x71; 32]),
        ContentDigest::from_bytes([0x73; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0x74; 32])),
        Vec::new(),
        definitions
            .into_iter()
            .map(|definition| {
                PreparedSpecification::from_definition(definition).expect("prepared requirement")
            })
            .collect(),
        vec![task],
    )
    .expect("prepared receipt");
    (SimulationRun::new_prepared(1, receipt), task_id)
}

/// One measured analysis attributed to `producer` at the named PVT point.
fn attributed(
    run: &SimulationRun,
    id: u64,
    producer: AnalysisInstanceId,
    process: &str,
    nominal: bool,
    value: f64,
) -> AnalysisResult {
    let snapshot = run
        .prepared_receipt()
        .expect("prepared receipt")
        .prepared_snapshot_digest();
    AnalysisResult::new(id, AnalysisType::Ac, format!("AC {process}"))
        .with_measurements(vec![rspice_core::MeasureResult::success("gain", value)])
        .with_provenance(
            AnalysisResultProvenance::new(producer, ObjectRevision::INITIAL, snapshot, Vec::new())
                .expect("prepared provenance")
                .with_pvt_point(Some(
                    AnalysisResultPvtPoint::new(process, None, 27.0, None, nominal)
                        .expect("attributed point"),
                )),
        )
}

/// The specifications the run froze, which is what every surface reading it
/// presents. Spelled from the receipt so the fixture states the contract
/// rather than trusting the code under test to resolve it.
fn frozen_specs(run: &SimulationRun) -> Vec<SpecEntry> {
    run.prepared_receipt()
        .expect("prepared receipt")
        .specifications()
        .iter()
        .map(|specification| specification.entry().clone())
        .collect()
}

/// A verdict does not change at the moment the run reaches its terminal state.
///
/// The sheet judged a streaming run with a projection of its own — no guard
/// band, no point scope, no producing-analysis binding — and then replaced
/// every row with the sealed verdict the instant the run completed. On this
/// dataset the two disagree in all three ways at once, so the row visibly
/// flipped under a reader who had changed nothing.
#[test]
fn a_streaming_verdict_is_the_verdict_the_seal_will_state() {
    let producer = AnalysisInstanceId::new();
    let (mut run, _task_id) = prepared_run(vec![requirement(
        "gain",
        SpecificationComparison::Maximum { limit: 10.0 },
        Some(1.0),
        Some(producer),
        SpecPointScope::Nominal,
    )]);
    let unrelated = AnalysisInstanceId::new();
    // 20 dB from a producer the requirement is not bound to, 15 dB from the
    // bound producer at a corner the scope does not admit, and 9.5 dB from the
    // bound producer at nominal — the only candidate the requirement admits,
    // and one the 1 dB guard band puts out of bound.
    for (id, producer, process, nominal, value) in [
        (1_u64, unrelated, "TT", true, 20.0),
        (2, producer, "SS", false, 15.0),
        (3, producer, "TT", true, 9.5),
    ] {
        let analysis = attributed(&run, id, producer, process, nominal, value);
        run.add_analysis(analysis);
    }
    run.mark_running().expect("the engine accepted the run");
    let specs = frozen_specs(&run);

    let live = result_rows(&run, &specs);

    assert_eq!(live.len(), 1);
    assert_eq!(
        live[0].status,
        SpecResultStatus::Fail,
        "the guard-banded 10 dB maximum is broken by the one admitted candidate: {:?}",
        live[0]
    );
    assert_eq!(live[0].value, Some(9.5), "{:?}", live[0]);
    assert_eq!(live[0].margin, Some(-0.5), "{:?}", live[0]);
    assert!(
        live[0].detail.contains("Provisional"),
        "a streaming judgment is not a sealed one and has to say so: {:?}",
        live[0].detail
    );

    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("the controller seals the run");
    let sealed = result_rows(&run, &specs);

    assert_eq!(
        sealed[0].status, live[0].status,
        "the verdict changed at the terminality boundary although the evidence did not"
    );
    assert_eq!(sealed[0].value, live[0].value);
    assert_eq!(sealed[0].margin, live[0].margin);
    assert_eq!(
        sealed[0].source_analysis_index,
        live[0].source_analysis_index
    );
    assert!(
        sealed[0].detail.contains("Immutable terminal verdict"),
        "{:?}",
        sealed[0].detail
    );
}

/// Two attributed producers are not an ambiguous lineage.
///
/// The streaming projection refused any measurement published by more than one
/// source instance, so an ordinary corner sweep — every point a separate
/// prepared task — read `invalid`, with no value and no margin, until the run
/// completed. The sealed judge takes the worst of the admitted evidence, and
/// that is the only judgment either surface may show.
#[test]
fn attributed_evidence_from_several_producers_is_judged_not_refused() {
    let (mut run, task_id) = prepared_run(vec![requirement(
        "gain",
        SpecificationComparison::Minimum { limit: 10.0 },
        None,
        None,
        SpecPointScope::AllPoints,
    )]);
    let second_task = AnalysisInstanceId::new();
    for (id, producer, process, value) in
        [(1_u64, task_id, "TT", 12.0), (2, second_task, "SS", 8.0)]
    {
        let analysis = attributed(&run, id, producer, process, process == "TT", value);
        run.add_analysis(analysis);
    }
    run.mark_running().expect("the engine accepted the run");

    let rows = result_rows(&run, &frozen_specs(&run));

    assert_eq!(
        rows[0].status,
        SpecResultStatus::Fail,
        "the worst admitted corner is 8 dB against a 10 dB floor: {:?}",
        rows[0]
    );
    assert_eq!(rows[0].value, Some(8.0));
    assert!(
        !rows[0]
            .detail
            .contains("different or unproven source lineages"),
        "two attributed prepared tasks are two proven lineages, not an ambiguity: {:?}",
        rows[0].detail
    );
}

/// The sheet, the CSV export and the print capture state one bound.
///
/// The run seals the requirement it was dispatched with. The export arm and
/// the hardcopy capture re-resolved theirs against the workspace's live
/// contract instead, so editing a limit after the run left the exported row
/// carrying a bound the run was never judged against beside the verdict it
/// actually earned.
#[test]
fn the_sheet_the_export_and_the_print_state_the_bound_the_run_was_judged_against() {
    let (mut run, task_id) = prepared_run(vec![requirement(
        "gain",
        SpecificationComparison::Minimum { limit: 10.0 },
        None,
        None,
        SpecPointScope::AllPoints,
    )]);
    let analysis = attributed(&run, 1, task_id, "TT", true, 12.0);
    run.add_analysis(analysis);
    run.mark_running().expect("the engine accepted the run");
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("the controller seals the run");

    // The reader edits the limit after the run: the workspace now asks for
    // 20 dB, and the retained run was never judged against it.
    let workspace_specs = vec![SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param=gain".to_owned(),
        min: Some(20.0),
        max: None,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    }];
    let frozen = frozen_specs(&run);
    assert_eq!(
        frozen[0].min,
        Some(10.0),
        "the receipt froze the 10 dB floor"
    );

    let sheet = result_rows(&run, &frozen);
    assert_eq!(sheet[0].status, SpecResultStatus::Pass);
    assert_eq!(sheet[0].limit, frozen[0].limit_text());

    let csv = super::export_csv(&run, &workspace_specs);
    let exported = csv
        .contents
        .lines()
        .find(|line| line.starts_with("gain,"))
        .expect("the export carries the requirement row")
        .to_owned();
    let fields: Vec<&str> = exported.split(',').collect();
    assert_eq!(
        fields[3],
        format!("{:.17e}", 10.0),
        "the export states the bound the run was judged against, not the one \
         the workspace now holds: {exported}"
    );
    assert!(
        exported.contains(&frozen[0].limit_text()),
        "and spells it the way the sheet does: {exported}"
    );

    let printed = super::hardcopy_table(&run, &workspace_specs);
    assert_eq!(
        printed.rows[0][3],
        format!("value >= {:.17e}", 10.0),
        "the printed document states the same bound the sheet showed: {:?}",
        printed.rows[0]
    );
    assert_eq!(printed.rows[0][8], sheet[0].status.label());
}

/// A workspace holding one long requirement contract and two runs that both
/// measured every limit in it, with the last limit carried in from the studio.
#[cfg(not(target_arch = "wasm32"))]
fn two_runs_over_a_long_contract() -> AppState {
    let names: Vec<String> = (0..60).map(|index| format!("m{index:02}")).collect();
    let mut state = AppState::default();
    for run_number in 1..=2 {
        let mut run = SimulationRun::new(run_number);
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Ac, "ac").with_measurements(
                names
                    .iter()
                    .enumerate()
                    .map(|(index, name)| {
                        rspice_core::MeasureResult::success(name, 1.0 + index as f64)
                    })
                    .collect(),
            ),
        );
        state.simulation.runs.push(run);
    }
    state.simulation.active_run_idx = Some(0);
    state.workspace.specs = names
        .iter()
        .map(|name| SpecEntry {
            measurement: name.clone(),
            expression: format!("max V({name})"),
            min: Some(0.0),
            max: None,
            unit: "V".to_owned(),
            scope: SpecPointScope::AllPoints,
        })
        .collect();
    state.workbench.selected_specification = Some("m59".to_owned());
    state
}

/// Where the carried row is painted, after the table has settled.
///
/// Forty frames because the scroll area animates towards a requested target
/// rather than jumping to it, and this is the position the reader ends up
/// looking at.
#[cfg(not(target_arch = "wasm32"))]
fn carried_row_position(
    ctx: &egui::Context,
    state: &mut AppState,
    screen: egui::Rect,
) -> egui::Rect {
    let mut spans = Vec::new();
    for _ in 0..40 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(screen),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::show(ui, state));
            },
        );
        spans = painted_spans(&output);
    }
    spans
        .into_iter()
        .find(|(text, _, _)| text == "m59")
        .map(|(_, rect, _)| rect)
        .expect("the carried requirement is one of the rows the table paints")
}

/// Hopping to another run scrolls the carried limit into view again.
///
/// The table remembered only which measurement it had last scrolled to, so
/// after a hop carried `m59` into run #1 and the reader selected run #2 — a
/// different dataset, a table drawn from the top — the memo still named `m59`
/// and the request was suppressed. The selected row was marked sixty rows
/// below the fold, in a table the reader had no reason to think had scrolled.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_carried_limit_is_scrolled_to_again_in_the_next_run() {
    let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1400.0, 420.0));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut state = two_runs_over_a_long_contract();

    let first = carried_row_position(&ctx, &mut state, screen);
    assert!(
        screen.contains(first.center()),
        "the hop into the first run puts its limit on screen: {first:?}"
    );

    state.simulation.active_run_idx = Some(1);
    let second = carried_row_position(&ctx, &mut state, screen);

    assert!(
        screen.contains(second.center()),
        "the same limit in the next run is {:.0} points below the fold, because the \
         table's scroll memo was never told the dataset changed: {second:?}",
        second.center().y - screen.bottom()
    );
}

/// The source button opens the sheet the shared map names for that analysis.
///
/// This module kept an analysis-type-to-viewer table of its own, and asked
/// whether the answer was available of the *previously* selected analysis
/// rather than the one the reader clicked. An optimizer's cost history is the
/// plain case: the shared compatibility predicate names the optimization
/// sheet, and the private table answered `Waves`, found it unavailable for the
/// old selection, and dropped the reader on the manifest.
#[test]
fn the_source_button_opens_the_viewer_the_shared_map_names() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC")
            .with_measurements(vec![rspice_core::MeasureResult::success("cost", 1.0)]),
    );
    run.add_analysis(
        AnalysisResult::new(2, AnalysisType::Optimization, "Optimize").with_family_metadata(
            AnalysisResultFamilyMetadata::Optimization {
                iterations: vec![4.0, 2.0, 1.0],
                best_cost: 1.0,
                best_variables: std::collections::BTreeMap::new(),
                converged: true,
            },
        ),
    );
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);

    let optimization = state.simulation.runs[0].analyses[1].clone();

    assert_eq!(
        super::source_viewer(&optimization),
        crate::workbench::ResultViewer::Optimization,
        "the retained optimizer history is what the shared map answers with"
    );
}

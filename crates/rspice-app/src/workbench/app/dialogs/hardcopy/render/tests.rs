//! The hardcopy studio decides once, then renders what it decided.
//!
//! Two rules carry most of these: a summary reports a decision only where
//! there is one to make, and the preview worker is bounded — two rotating
//! texture slots, no launch over an active worker, and a cancel that
//! completes before the dialog can reopen. The fit cases hold the studio to
//! every viewport without scrolling, which is the constraint the section
//! layout exists to satisfy.

use super::*;

/// A source with no governed drawing sheet has no sheet furniture to
/// configure, so the rail must not offer the section — and a selection
/// left pointing at it must land somewhere real rather than on a blank
/// editor.
#[test]
fn a_source_without_an_authored_sheet_offers_no_sheet_section() {
    let draft = HardcopyDialogState::default();
    let sections = available_sections(&draft);

    assert!(!sections.contains(&HardcopySection::DrawingSheet));
    assert_eq!(sections.first(), Some(&HardcopySection::Source));
    assert!(sections.contains(&HardcopySection::Output));
}

/// The rail summarises the sections nobody is looking at, so a summary that
/// names a decision the document does not pose is worse than none.
#[test]
fn the_sheet_summary_only_reports_a_decision_there_is_one_to_make() {
    let mut draft = HardcopyDialogState::default();
    draft.outside_sheet_content = OutsideSheetContentPolicy::Ask;
    // No resolved document: nothing crosses a sheet that does not exist.
    assert_eq!(
        section_detail(&draft, HardcopySection::DrawingSheet),
        "authored · undecided"
    );
    draft.schematic_extent = SchematicHardcopyExtent::CompleteSchematicContent;
    assert_eq!(
        section_detail(&draft, HardcopySection::DrawingSheet),
        "complete content"
    );
}

/// A raster target is meaningless without its resolution, and the rail is
/// where it is read while another section is open.
#[test]
fn the_output_summary_carries_the_resolution_of_a_raster_target() {
    assert_eq!(
        output_summary(OutputFormat::Png { dpi: 300 }),
        "PNG · 300 dpi"
    );
    assert_eq!(
        output_summary(OutputFormat::Tiff { dpi: 1_200 }),
        "TIFF · 1200 dpi"
    );
    assert_eq!(output_summary(OutputFormat::PdfA), "PDF/A · vector");
}

/// The field must refuse what the publisher would refuse, and for the
/// same reason. A ceiling of `None` means the plan has not compiled yet,
/// so only the contract's own range is known.
#[test]
fn a_resolution_outside_the_offered_range_is_refused() {
    assert_eq!(parse_raster_dpi("300", Some(600)), Ok(300));
    assert_eq!(parse_raster_dpi("  600 ", Some(600)), Ok(600));
    assert!(parse_raster_dpi("601", Some(600)).is_err());
    assert!(parse_raster_dpi("71", Some(600)).is_err());
    assert!(parse_raster_dpi("", Some(600)).is_err());
    assert!(parse_raster_dpi("600dpi", Some(600)).is_err());
    assert!(parse_raster_dpi("-300", Some(600)).is_err());
    // Without a compiled plan the contract is the only authority.
    assert_eq!(parse_raster_dpi("9600", None), Ok(MAX_RASTER_DPI));
    assert!(parse_raster_dpi("9601", None).is_err());
}

#[test]
fn preview_texture_cache_is_bounded_to_two_rotating_slots() {
    assert_eq!(preview_texture_slot_id(0), preview_texture_slot_id(0));
    assert_eq!(preview_texture_slot_id(1), preview_texture_slot_id(8));
    assert_ne!(preview_texture_slot_id(0), preview_texture_slot_id(1));
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn rapid_preview_invalidation_never_launches_over_an_active_worker() {
    assert_eq!(
        preview_schedule_decision(None, 10),
        PreviewScheduleDecision::Launch
    );
    assert_eq!(
        preview_schedule_decision(Some(10), 10),
        PreviewScheduleDecision::Wait
    );
    for generation in 11..1_000 {
        assert_eq!(
            preview_schedule_decision(Some(10), generation),
            PreviewScheduleDecision::CancelAndWait
        );
    }
    assert_eq!(
        preview_schedule_decision(None, 999),
        PreviewScheduleDecision::Launch
    );
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn closing_preview_cancels_and_clears_worker_before_reopen() {
    let (_sender, receiver) = std::sync::mpsc::sync_channel(1);
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    PREVIEW_WORKER.with(|runtime| {
        runtime.borrow_mut().active = Some(ActivePreviewWorker {
            generation: 41,
            receiver,
            cancelled: cancelled.clone(),
        });
    });

    cancel_preview_worker();

    assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
    PREVIEW_WORKER.with(|runtime| {
        assert!(runtime.borrow().active.is_none());
    });
    assert_eq!(
        preview_schedule_decision(None, 42),
        PreviewScheduleDecision::Launch
    );
}

/// Whatever the track, the editor takes its measure and the page desk takes
/// the rest — and a column too short for a stacked form buys its height
/// back with the width the desk would have had.
#[test]
fn the_page_desk_takes_every_point_the_form_does_not_need() {
    let tall = STACKED_FORM_HEIGHT;
    for track in [1_050.0_f32, 888.0, 782.0, 768.0] {
        let (editor, preview) = column_widths(track, tall);
        assert_eq!(editor, EDITOR_STACKED_MEASURE);
        assert!((editor + preview - track).abs() < 0.5);
    }
    let (editor, preview) = column_widths(1_050.0, STACKED_FORM_HEIGHT - 1.0);
    assert_eq!(editor, EDITOR_PAIRED_MEASURE);
    assert!((editor + preview - 1_050.0).abs() < 0.5);
    // A track that cannot pay for both gives way from the editor first: a
    // page too small to read is not worth a wider form.
    let (editor, preview) = column_widths(EDITOR_MIN + PREVIEW_MIN, tall);
    assert_eq!((editor, preview), (EDITOR_MIN, PREVIEW_MIN));
}

/// The toolbar's band is measured, not assumed. The desk must give up the
/// exact band the toolbar draws at both wide and constrained widths.
#[test]
fn the_preview_toolbar_reserves_exactly_the_band_it_draws() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(44.0));
    let mut bands = Vec::new();
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            for width in [620.0_f32, 340.0] {
                let view = PreviewView { page: 0, zoom: 100 };
                let reserved = preview_toolbar_height(ui, &view, 4, width);
                let drawn = ui
                    .allocate_ui_with_layout(
                        vec2(width, 400.0),
                        Layout::top_down(Align::Min),
                        |ui| preview_toolbar(ui, &mut { view }, 4, width),
                    )
                    .inner;
                bands.push((width, reserved, drawn));
            }
        });
    });
    for (width, reserved, drawn) in &bands {
        assert!(
            (reserved - drawn).abs() < 0.5,
            "at {width:.0} pt the desk gave up {reserved:.0} pt for a toolbar that drew \
                 {drawn:.0}"
        );
    }
    let row = 44.0 + 12.0;
    assert!(
        bands[0].1 < row + 6.0,
        "a 620 pt column holds the toolbar in one row, but {:.0} pt was reserved",
        bands[0].1
    );
}

/// The tablet width the studio ships to is edge to edge and touch-sized,
/// and the page desk keeps only the column the form does not need. The
/// toolbar has to hold that column in one row: wrapping there costs the
/// page a whole row and buys a strip with one control stranded under six.
#[test]
fn the_preview_toolbar_holds_one_row_at_the_tablet_width() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply_responsive_metrics_with_target(&ctx, Some(44.0));
    let (_, desk) = column_widths(820.0, STACKED_FORM_HEIGHT);
    let mut band = 0.0;
    let _ = ctx.run_ui(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let view = PreviewView { page: 0, zoom: 100 };
            band = preview_toolbar_height(ui, &view, 4, desk);
        });
    });
    assert!(
        band < 44.0 + 12.0 + 6.0,
        "the toolbar took {band:.0} pt of a {desk:.0} pt column, which is more than one row"
    );
}

#[test]
fn document_type_choice_prefers_active_extent_over_selection() {
    use crate::workbench::hardcopy_adapters::sources::{
        RetainedHardcopySourceAvailability, RetainedHardcopySourceDescriptor,
    };
    use rspice_hardcopy_contract::{HardcopyDocumentKind, HardcopyScope};

    let candidate = RetainedHardcopySourceDescriptor {
        source_key: "project:test:schematic".to_owned(),
        display_name: "top / schematic".to_owned(),
        document_kind: HardcopyDocumentKind::SchematicOrSymbol,
        allowed_scopes: vec![
            HardcopyScope::Selection,
            HardcopyScope::CurrentSheet,
            HardcopyScope::AllSheetsOrPanes,
        ],
        availability: RetainedHardcopySourceAvailability::Available,
    };
    let candidates = vec![candidate];
    let choice = source_choice_for_active_extent(&candidates, candidates.first()).unwrap();
    assert_eq!(choice.0, "project:test:schematic");
    assert_eq!(choice.1, HardcopyScope::CurrentSheet);
}

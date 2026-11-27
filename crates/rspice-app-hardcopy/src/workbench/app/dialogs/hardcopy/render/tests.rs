//! The Print and Export dialog decides once, then renders what it decided.
//!
//! The preview worker is bounded: two rotating texture slots, no launch over
//! an active worker, and a cancel that completes before the dialog can
//! reopen.

use super::*;

/// A raster target is meaningless without its resolution.
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

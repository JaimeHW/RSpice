use super::*;
use egui::Rect;

/// The well the Results workspace hands a blocking state on a desktop.
const WELL: egui::Vec2 = egui::vec2(1280.0, 720.0);

/// Lay one blocking card out headlessly and report the well it was given
/// alongside the rectangle it actually took.
fn centered_card(state: ResultOperationalState, well_size: egui::Vec2) -> (Rect, Rect) {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let status = ResultOperationalStatus::canonical(state, true);
    let mut well = Rect::NOTHING;
    let mut card = Rect::NOTHING;
    // Fonts build on the first pass and sizing settles on the second, so
    // the geometry asserted on is the third pass's.
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, well_size)),
            ..Default::default()
        };
        let _ = ctx.run_ui(input, |ui| {
            well = ui.available_rect_before_wrap();
            card = OperationalCard {
                status: &status,
                accent: Tokens::get(ui.ctx()).color.info,
                offer: None,
                floating: true,
            }
            .show_centered(ui, &mut OperationalResponse::default())
            .rect;
        });
    }
    (well, card)
}

/// A state that owns the well is centred in it, on both axes. The offset
/// this replaced was computed from a guessed card height against the
/// well's left edge, so the card landed neither centred nor predictably.
#[test]
fn a_blocking_card_is_centred_in_the_well_it_owns() {
    let (well, card) = centered_card(ResultOperationalState::Corrupted, WELL);

    assert!(
        (card.center().x - well.center().x).abs() <= 1.0,
        "card centre {:?} is not the well centre {:?}",
        card.center(),
        well.center()
    );
    assert!(
        (card.center().y - well.center().y).abs() <= 1.0,
        "card centre {:?} is not the well centre {:?}",
        card.center(),
        well.center()
    );
}

/// Every state gets the same bounded column. Width used to be whatever
/// the state's longest sentence happened to measure, so each state drew a
/// differently sized card.
#[test]
fn card_width_is_the_bounded_column_and_not_the_length_of_the_sentence() {
    let (_, corrupted) = centered_card(ResultOperationalState::Corrupted, WELL);
    let (_, partial) = centered_card(ResultOperationalState::Partial, WELL);
    let (_, offline) = centered_card(ResultOperationalState::Offline, WELL);

    assert_eq!(corrupted.width(), OPERATIONAL_CARD_MAX_WIDTH);
    assert_eq!(partial.width(), corrupted.width());
    assert_eq!(offline.width(), corrupted.width());
}

/// Below the bounded column the card keeps its gutter rather than
/// touching, or overflowing, the well's edges.
#[test]
fn a_narrow_well_keeps_the_card_inside_its_gutter() {
    let narrow = egui::vec2(360.0, 540.0);
    let (well, card) = centered_card(ResultOperationalState::Corrupted, narrow);

    assert_eq!(card.width(), well.width() - 2.0 * OPERATIONAL_CARD_GUTTER);
    assert!(
        well.contains_rect(card),
        "{card:?} escaped the well {well:?}"
    );
}

/// A card taller than its well is pinned to the top of it, so the copy
/// that explains the state is the part that stays on screen.
#[test]
fn a_card_taller_than_its_well_starts_at_the_top() {
    let short = egui::vec2(420.0, 90.0);
    let (well, card) = centered_card(ResultOperationalState::Corrupted, short);

    assert!(card.height() > well.height(), "the case under test");
    assert_eq!(card.top(), (well.top() + OPERATIONAL_CARD_GUTTER).round());
}

/// Lay one empty-state hint out headlessly and report the well, the
/// rectangle its text took, and every shape the pass painted.
fn empty_hint(
    state: ResultOperationalState,
    well_size: egui::Vec2,
) -> (Rect, Rect, Vec<egui::epaint::ClippedShape>) {
    let ctx = egui::Context::default();
    rspice_ui_kit::Theme::default().apply(&ctx);
    let status = ResultOperationalStatus::canonical(state, true);
    assert_eq!(state.category(), ResultOperationalCategory::Empty);
    let mut well = Rect::NOTHING;
    let mut text = Rect::NOTHING;
    let mut shapes = Vec::new();
    for _ in 0..3 {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, well_size)),
            ..Default::default()
        };
        shapes = ctx
            .run_ui(input, |ui| {
                well = ui.available_rect_before_wrap();
                text = show_empty_hint(ui, &status).rect;
            })
            .shapes;
    }
    (well, text, shapes)
}

/// An empty well reads like an empty schematic sheet: centred text with
/// nothing painted behind it.
#[test]
fn an_empty_state_is_centred_text_with_no_card() {
    for state in [
        ResultOperationalState::NoDataset,
        ResultOperationalState::NoProject,
    ] {
        let (well, text, shapes) = empty_hint(state, WELL);

        assert_eq!(shapes.len(), 3, "{state:?}: title, message, recovery");
        assert!(
            shapes
                .iter()
                .all(|clipped| matches!(clipped.shape, egui::Shape::Text(_))),
            "{state:?} painted something other than text: {shapes:?}"
        );
        assert!(
            (text.center().x - well.center().x).abs() <= 1.0,
            "{state:?}: text centre {:?} is not the well centre {:?}",
            text.center(),
            well.center()
        );
        assert!(
            (text.center().y - well.center().y).abs() <= 1.0,
            "{state:?}: text centre {:?} is not the well centre {:?}",
            text.center(),
            well.center()
        );
    }
}

/// On a phone-width well the sentences wrap inside the gutter instead of
/// running past the canvas edges.
#[test]
fn a_narrow_well_wraps_the_hint_inside_its_gutter() {
    let narrow = egui::vec2(360.0, 540.0);
    let (well, text, _) = empty_hint(ResultOperationalState::NoDataset, narrow);

    assert!(
        well.shrink(OPERATIONAL_CARD_GUTTER - 0.5)
            .contains_rect(text),
        "{text:?} escaped the gutter of {well:?}"
    );
}

/// A hint taller than its well is pinned to the top, so the title stays
/// on screen.
#[test]
fn a_hint_taller_than_its_well_starts_at_the_top() {
    let short = egui::vec2(420.0, 60.0);
    let (well, text, _) = empty_hint(ResultOperationalState::NoDataset, short);

    assert!(text.height() > well.height(), "the case under test");
    assert_eq!(text.top(), (well.top() + OPERATIONAL_CARD_GUTTER).round());
}
#[test]
fn canonical_operational_registry_matches_the_result_contract() {
    assert_eq!(
        ResultOperationalState::ALL.map(ResultOperationalState::id),
        [
            "complete",
            "no-project",
            "no-dataset",
            "loading",
            "streaming",
            "partial",
            "stale",
            "failed",
            "corrupted",
            "unsupported",
            "offline",
            "storage-denied",
            "low-memory",
            "renderer-loss",
            "interrupted-operation",
            "recovered",
        ]
    );
    for state in ResultOperationalState::ALL {
        assert!(!state.label().is_empty());
        assert!(!state.message().is_empty());
        assert!(!state.recovery().is_empty());
    }
}

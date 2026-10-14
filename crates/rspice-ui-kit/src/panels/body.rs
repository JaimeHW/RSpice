//! The panel body: the one inset and the one vertical rhythm every panel
//! primitive lays itself out on.
//!
//! A panel's content sits in a body that starts [`PANEL_INSET`] in from each
//! edge. Anything placed in it — a property row, a note, a button row, a bare
//! label, a field under a stacked label — starts on the inset because the body
//! put it there, so no caller has to remember to. A section header's band and
//! a row's background still reach the panel's edges: they paint across
//! [`panel_bleed`] rather than across their own allocation.
//!
//! The rhythm is owned the same way. Inside a body a section header leaves
//! [`PANEL_SECTION_GAP`] above itself unless it opens the body, and
//! [`PANEL_HEADER_GAP`] below; rows are [`PANEL_ROW_H`] tall and stack with no
//! spacing between them; the body closes with one more section gap.

use egui::{Rect, Ui, UiBuilder, UiStackInfo, Vec2};

/// Where every element in a panel starts, measured from the panel's left edge,
/// and where it stops short of the right one.
pub const PANEL_INSET: f32 = 10.0;
/// A panel row's height at compact density: a property row, a note's line, a
/// problem's line.
pub const PANEL_ROW_H: f32 = 24.0;
/// Space between a section header and the first row under it.
pub const PANEL_HEADER_GAP: f32 = 4.0;
/// Space between one section's last row and the next section's header, and
/// between two blocks of one section's body.
pub const PANEL_SECTION_GAP: f32 = 8.0;

/// A panel row is this much shorter than a standalone control at the same
/// density: its field gives up the padding a free-standing control keeps.
const PANEL_ROW_TRIM: f32 = 4.0;
/// Space between a row's edge and the field set in it, above and below.
pub(crate) const PANEL_FIELD_INSET_Y: f32 = 1.0;

/// The tag a body's `Ui` carries in the [`egui::UiStack`].
const PANEL_BODY_TAG: &str = "rspice.panel-body";

/// What a body records about itself for the primitives placed in it.
#[derive(Debug, Clone, Copy)]
struct PanelBody {
    /// The panel's own horizontal extent: where a band or a row background
    /// reaches.
    outer: egui::Rangef,
}

/// The nearest panel body around `ui`, as its tag and its inner rect.
fn enclosing_body(ui: &Ui) -> Option<(PanelBody, Rect, egui::Id)> {
    ui.stack().iter().find_map(|node| {
        node.tags()
            .get_downcast::<PanelBody>(PANEL_BODY_TAG)
            .map(|body| (*body, node.max_rect, node.id))
    })
}

/// Lay `add` out in a panel body: inset [`PANEL_INSET`] from both edges of the
/// space it is given, with rows stacked on the panel rhythm and every text
/// field and select spanning the body unless a row gives it a column.
///
/// Inside a body already, `add` runs in it as it is: a body never nests.
pub fn panel_body<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    if enclosing_body(ui).is_some() {
        return add(ui);
    }
    let outer = ui.available_rect_before_wrap();
    let inner = Rect::from_x_y_ranges(
        (outer.left() + PANEL_INSET)
            ..=(outer.right() - PANEL_INSET).max(outer.left() + PANEL_INSET),
        outer.y_range(),
    );
    let info = UiStackInfo::default().with_tag_value(
        PANEL_BODY_TAG,
        PanelBody {
            outer: outer.x_range(),
        },
    );
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(inner)
            .layout(egui::Layout::top_down(egui::Align::Min))
            .ui_stack_info(info),
        |ui| {
            ui.set_min_width(inner.width());
            let spacing = ui.spacing_mut();
            spacing.item_spacing.y = 0.0;
            // A field's own width is a guess; the body's is the column.
            // `TextEdit` never takes more than the space it is given, so a
            // field a row lays out beside a label still fits its column.
            spacing.text_edit_width = f32::INFINITY;
            let top = ui.cursor().top();
            let inner = add(ui);
            if ui.cursor().top() > top + 0.5 {
                ui.add_space(PANEL_SECTION_GAP);
            }
            let body = ui.stack().id;
            ui.data_mut(|data| data.remove_temp::<f32>(header_bottom_id(body)));
            inner
        },
    )
    .inner
}

/// Whether `left` is the left edge of the body around `ui`: the body has
/// already inset whatever starts there.
fn on_body_edge(ui: &Ui, left: f32) -> Option<PanelBody> {
    enclosing_body(ui)
        .filter(|(_, inner, _)| (left - inner.left()).abs() < 0.5)
        .map(|(body, _, _)| body)
}

/// How far in from the space a row is given its text starts: nothing on a
/// panel body's edge, where the body has inset it already, and [`PANEL_INSET`]
/// anywhere else.
///
/// A primitive that paints its own row reads this rather than restating the
/// inset, so it lines up with the rows around it in a body and out of one.
pub fn panel_inset(ui: &Ui) -> f32 {
    if on_body_edge(ui, ui.available_rect_before_wrap().left()).is_some() {
        0.0
    } else {
        PANEL_INSET
    }
}

/// `rect` widened to the panel's edges when it sits on a panel body's inset:
/// where a band, a rule or a row's background is painted, so it reaches the
/// panel's edges while the row's text stays on the inset.
pub fn panel_bleed(ui: &Ui, rect: Rect) -> Rect {
    match on_body_edge(ui, rect.left()) {
        Some(body) => Rect::from_x_y_ranges(
            body.outer.min.min(rect.left())..=body.outer.max.max(rect.right()),
            rect.y_range(),
        ),
        None => rect,
    }
}

/// How tall one panel row is under the active density: [`PANEL_ROW_H`] when
/// compact, the control height less the field padding a row gives up when
/// relaxed, and the whole touch target on a coarse pointer.
///
/// A surface that lays rows into a grid has to know a row's height before it
/// paints one, to size the band the grid sits in.
pub fn property_row_height(ui: &Ui) -> f32 {
    let metrics = crate::tokens::Tokens::get(ui.ctx()).metrics;
    if metrics.is_touch() {
        metrics.ctl_h
    } else {
        PANEL_ROW_H.max(metrics.ctl_h - PANEL_ROW_TRIM)
    }
}

/// Where the last section header in the body around `ui` left the cursor.
fn header_bottom_id(body: egui::Id) -> egui::Id {
    body.with("rspice.panel-body.header-bottom")
}

/// Space a section header leaves above itself: one section gap, unless it is
/// the first thing in its body. Outside a body a header leaves none; the
/// surface it sits on owns that space.
pub(super) fn before_header(ui: &mut Ui) {
    if let Some((_, inner, _)) = enclosing_body(ui)
        && ui.cursor().top() > inner.top() + 0.5
    {
        ui.add_space(PANEL_SECTION_GAP);
    }
}

/// Space a section header leaves under itself, recorded so the body's first
/// block can tell that it is first.
pub(super) fn after_header(ui: &mut Ui) {
    let Some((_, _, body)) = enclosing_body(ui) else {
        return;
    };
    ui.add_space(PANEL_HEADER_GAP);
    let bottom = ui.cursor().top();
    ui.data_mut(|data| data.insert_temp(header_bottom_id(body), bottom));
}

/// One section gap between two blocks of the same section body — a list of
/// rows and the button row under it, say. The block right under a header
/// takes none: the header's own gap already sits above it.
pub fn section_block_gap(ui: &mut Ui) {
    let header_bottom = enclosing_body(ui)
        .and_then(|(_, _, body)| ui.data(|data| data.get_temp::<f32>(header_bottom_id(body))));
    if header_bottom.is_none_or(|bottom| ui.cursor().top() > bottom + 0.5) {
        ui.add_space(PANEL_SECTION_GAP);
    }
}

/// The rect a field takes in a row: the row's value column, a pixel in from
/// its top and bottom.
pub(crate) fn row_field(value: Rect) -> Rect {
    value.shrink2(Vec2::new(0.0, PANEL_FIELD_INSET_Y))
}

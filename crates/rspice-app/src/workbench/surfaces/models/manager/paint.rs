//! The painters every page of this workspace draws with.
//!
//! One card, one property row, one table header, one selectable data row, one
//! empty state. They live together because the pages have to agree: a table
//! whose header measured its columns differently from the rows under it puts
//! every cell in the wrong place, and two spellings of "this page has nothing
//! on it" is two designs.
//!
//! Nothing here reads or writes application state. Every function takes what
//! it paints and returns either nothing or the response the caller senses,
//! which is what lets a page compose them without any of them reaching for the
//! project.
//!
//! # Painted text publishes no accessibility node
//!
//! Most of this draws through `ui.painter()`, which is what keeps a table of
//! two hundred rows affordable and is also why a painted cell is invisible to
//! a screen reader. Every painter here either publishes a node itself — see
//! [`page_empty_state`] — or is used by a caller that declares one on the
//! response it hands back. A painter that did neither would be legible only to
//! readers who can see it.

use super::*;

/// The page bar's content height, from the mockup's `.model-section-bar`
/// (`min-height: 40px`).
pub(super) const SECTION_BAR_CONTENT_H: f32 = 40.0;
/// Vertical padding the bar's frame adds above and below that content
/// (`padding: 5px 12px`).
const SECTION_BAR_PAD_Y: i8 = 5;
/// The whole bar, frame included — the band the surface test measures its
/// page actions against.
#[cfg(test)]
pub(in crate::workbench::surfaces::models) const SECTION_BAR_H: f32 =
    SECTION_BAR_CONTENT_H + SECTION_BAR_PAD_Y as f32 * 2.0;

/// One page bar: title and live subtitle on the left, the page's own actions
/// hard against the right.
///
/// The actions are laid out **right to left**, so a caller writes its accent
/// primary *first* and the secondaries after it — the same grammar the
/// Simulate pages use, and the same order the mockup renders.
///
/// `allocate_ui_with_layout` advances the parent by the *content* width, not
/// the width it was handed, which is why the band and the title inside it both
/// state a minimum: without one, a page whose actions are narrow lets the bar
/// shrink to them and the title truncates against a band that is mostly empty.
pub(super) fn section_title(
    ui: &mut Ui,
    title: &str,
    subtitle: &str,
    actions: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let bar = egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, SECTION_BAR_PAD_Y))
        .show(ui, |ui| {
            let track = ui.available_width();
            ui.allocate_ui_with_layout(
                egui::vec2(track, SECTION_BAR_CONTENT_H),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.set_min_width(track);
                    ui.set_min_height(SECTION_BAR_CONTENT_H);
                    ui.spacing_mut().item_spacing.x = 6.0;
                    actions(ui);
                    // The mockup's `gap: 6px 12px` between the meta and the
                    // action cluster.
                    ui.add_space(6.0);
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        // A title is one line of a page bar. Truncating is what
                        // keeps a long subtitle from wrapping the band open and
                        // pushing the body down.
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Truncate);
                        ui.spacing_mut().item_spacing.x = 12.0;
                        ui.label(
                            RichText::new(title)
                                .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                                .color(t.color.text),
                        );
                        ui.label(
                            RichText::new(subtitle)
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                    });
                },
            );
        });
    // The mockup's `border-bottom` on the bar itself: the full width of the
    // document column, not the width inside the bar's own padding.
    ui.painter().hline(
        bar.response.rect.x_range(),
        bar.response.rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
}

pub(super) fn card(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(7))
        .show(ui, |ui| {
            // Cards are structural panes, not shrink-to-fit labels.
            ui.set_min_width(ui.available_width().max(1.0));
            content(ui);
        });
}

pub(super) fn detail_pane(
    ui: &mut Ui,
    title: &str,
    meta: Option<&str>,
    content: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::ZERO)
        .show(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| card_title(ui, title, meta));
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, content);
        });
}

/// The shortest a detail pane is ever drawn, from the mockup's
/// `grid-auto-rows: minmax(170px, auto)`.
pub(super) const DETAIL_PANE_MIN_H: f32 = 170.0;

/// A detail pane that fills the track it was handed.
///
/// The mockup's `.model-detail-body` is one grid row of `minmax(0, 1fr)`
/// columns: every pane in the row is the same height and the row reaches the
/// panel's bottom edge. A pane that stopped at its own content left the
/// container's hairline colour showing through below it, which reads as a
/// block that failed to render rather than as room. Overflowing rows scroll
/// inside the pane — `.model-pane-scroll { overflow: auto }` — so a card with
/// three hundred parameters never grows the row.
pub(super) fn filled_detail_pane(
    ui: &mut Ui,
    title: &str,
    meta: Option<&str>,
    height: f32,
    scroll_salt: &str,
    content: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::ZERO)
        .show(ui, |ui| {
            // The caller measures the track it has; the pane's own border is
            // drawn inside that, so the content box is the track less the two
            // hairlines. Without the subtraction a row of panes overshoots the
            // panel by exactly its border, every time.
            let height = (height - 2.0).max(DETAIL_PANE_MIN_H);
            ui.set_min_width(ui.available_width().max(1.0));
            ui.set_min_height(height);
            ui.set_max_height(height);
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| card_title(ui, title, meta));
            ScrollArea::vertical()
                .id_salt(scroll_salt)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Frame::NONE
                        .inner_margin(egui::Margin::symmetric(12, 8))
                        .show(ui, content);
                });
        });
}

pub(super) fn card_title(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text_dim),
        );
        if let Some(meta) = meta {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(meta)
                        .small()
                        .monospace()
                        .color(t.color.text_faint),
                );
            });
        }
    });
    ui.separator();
}

pub(super) fn property(ui: &mut Ui, name: &str, value: &str, origin: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 22.0), Sense::hover());
    let name_width = rect.width() * 0.30;
    let value_width = rect.width() * 0.34;
    let origin_width = (rect.width() - name_width - value_width).max(1.0);
    let inset = 3.0;

    let name = elide(ui, name, (name_width - inset * 2.0).max(1.0), false);
    let value = elide(ui, value, (value_width - inset * 2.0).max(1.0), true);
    let origin = elide(ui, origin, (origin_width - inset * 2.0).max(1.0), false);
    ui.painter().text(
        egui::pos2(rect.left() + inset, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.left() + name_width + inset, rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.right() - inset, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        origin,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

pub(super) fn empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong().color(t.color.text_dim));
        ui.label(RichText::new(detail).small().color(t.color.text_faint));
    });
    ui.add_space(10.0);
}

/// A page with nothing on it, and the reason there is nothing.
///
/// Every glyph here is painter text, which publishes no accessibility node at
/// all — so this used to be a whole page a screen reader found empty *and
/// silent*, including the pages that carry a refusal: a store that would not
/// open, and a catalog past the instant its publisher signed. The node is
/// declared outright, carrying both sentences whole. The painted detail elides
/// to the panel's width; what is announced does not, for the same reason
/// `announced_widget` next door announces the full sentence.
pub(super) fn page_empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let size = egui::vec2(
        ui.available_width().max(1.0),
        ui.available_height().max(180.0),
    );
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    let announcement = format!("{title}. {detail}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(announcement.clone());
    });
    let panel = rect.shrink2(egui::vec2(12.0, 12.0));
    ui.painter().rect(
        panel,
        3.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let accent = egui::Rect::from_center_size(
        egui::pos2(panel.center().x, panel.center().y - 34.0),
        egui::vec2(34.0, 3.0),
    );
    ui.painter().rect_filled(accent, 2.0, t.color.accent);
    ui.painter().text(
        egui::pos2(panel.center().x, panel.center().y - 12.0),
        egui::Align2::CENTER_CENTER,
        title,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(panel.center().x, panel.center().y + 14.0),
        egui::Align2::CENTER_CENTER,
        elide(ui, detail, (panel.width() - 48.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

pub(super) fn table_header(ui: &mut Ui, columns: &[(&str, f32)]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), HEADER_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    let mut x = rect.left() + 5.0;
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_faint,
        );
        x += width;
    }
}

/// One column of a sortable table header.
///
/// The label is authored at the call site as a literal, which is what lets the
/// control ratchet see a header cell for the control it is — a click on one
/// reorders the table the reader is looking at.
pub(super) struct SortColumn<K> {
    label: &'static str,
    fraction: f32,
    key: K,
}

/// Declare a sortable column: what it says, how wide it is, and what clicking
/// it orders by.
pub(super) const fn sort_column<K>(label: &'static str, fraction: f32, key: K) -> SortColumn<K> {
    SortColumn {
        label,
        fraction,
        key,
    }
}

/// A table header whose cells order the rows beneath them.
///
/// Returns the key of the cell activated this frame, if any; the caller owns
/// what that does to its order. The active column carries a painted direction
/// mark rather than a font glyph — the bundled face is missing several arrows,
/// and a header that renders a tofu box is worse than one with no mark at all.
///
/// Each cell is a real control: it publishes a button role with the name a
/// reader hears, takes keyboard focus, and activates on Enter and Space the
/// same as it does on a click.
pub(super) fn sortable_table_header<K: Copy + PartialEq>(
    ui: &mut Ui,
    id_salt: &str,
    columns: &[SortColumn<K>],
    active: K,
    descending: bool,
) -> Option<K> {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), HEADER_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    let mut activated = None;
    let mut x = rect.left();
    for (index, column) in columns.iter().enumerate() {
        let width = rect.width() * column.fraction;
        let cell = egui::Rect::from_min_max(
            egui::pos2(x, rect.top()),
            egui::pos2((x + width).min(rect.right()), rect.bottom()),
        );
        x += width;
        let response = ui.interact(
            cell,
            ui.make_persistent_id((id_salt, index)),
            Sense::click(),
        );
        let sorted = column.key == active;
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!("sort by {}", column.label),
            )
        });
        if response.clicked() {
            activated = Some(column.key);
        }
        if response.hovered() {
            ui.painter().rect_filled(cell, 0.0, t.color.bg_hover);
        }
        let text_color = if sorted {
            t.color.text
        } else {
            t.color.text_faint
        };
        // The mark sits after the label, inside the cell, so a narrow column
        // clips the label rather than losing the direction it is ordered by.
        let mark_room = if sorted { 13.0 } else { 0.0 };
        let label = elide(
            ui,
            column.label,
            (cell.width() - 10.0 - mark_room).max(1.0),
            false,
        );
        let label_width = ui
            .painter()
            .layout_no_wrap(
                label.clone(),
                theme::sans(tokens::FS_0, FontWeight::SemiBold),
                text_color,
            )
            .size()
            .x;
        ui.painter().text(
            egui::pos2(cell.left() + 5.0, cell.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            text_color,
        );
        if sorted {
            paint_sort_direction(
                ui,
                egui::pos2(cell.left() + 5.0 + label_width + 6.0, cell.center().y),
                descending,
                t.color.accent,
            );
        }
        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        theme::paint_focus_ring(ui, &response, cell);
    }
    activated
}

/// The direction mark on the ordered column: a filled triangle, drawn.
fn paint_sort_direction(ui: &Ui, center: egui::Pos2, descending: bool, color: egui::Color32) {
    let half_width = 3.5;
    let half_height = 2.5;
    let points = if descending {
        vec![
            egui::pos2(center.x - half_width, center.y - half_height),
            egui::pos2(center.x + half_width, center.y - half_height),
            egui::pos2(center.x, center.y + half_height),
        ]
    } else {
        vec![
            egui::pos2(center.x - half_width, center.y + half_height),
            egui::pos2(center.x + half_width, center.y + half_height),
            egui::pos2(center.x, center.y - half_height),
        ]
    };
    ui.painter().add(egui::epaint::Shape::convex_polygon(
        points,
        color,
        Stroke::NONE,
    ));
}

pub(super) fn selectable_data_row(
    ui: &mut Ui,
    selected: bool,
    columns: &[(&str, f32, bool)],
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    paint_columns(ui, rect, columns);
    // The first column is the row's identifier in every caller, so it is the
    // name a screen reader should announce for the selection.
    let row_label = columns
        .first()
        .map(|(value, _, _)| (*value).to_owned())
        .unwrap_or_default();
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
    response
}

pub(super) fn paint_columns(ui: &Ui, rect: egui::Rect, columns: &[(&str, f32, bool)]) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left() + 5.0;
    for (value, fraction, mono) in columns {
        let width = rect.width() * fraction;
        let clipped = elide(ui, value, width - 9.0, *mono);
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            clipped,
            if *mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            t.color.text_dim,
        );
        x += width;
    }
}

/// How far a re-parsed display spelling may sit from the value it was made
/// from before the raw form is shown instead.
///
/// The same relative tolerance the netlist document's value hover uses: a
/// decade rescale is not exact in binary, so `480m` re-parsed is not bit-equal
/// to `0.48` and demanding that it were would print raw digits for every value
/// on every surface.
const ENGINEERING_ROUND_TRIP_TOLERANCE: f64 = 1e-12;

/// A number spelled the way this product spells numbers — but only when its
/// own parser reads that spelling back to the number it was made from.
///
/// The round trip is the whole guard. Engineering notation drops digits, and a
/// display form that no longer names the value behind it is a claim the reader
/// has no way to check; a card parameter that reads `1.5p` when the source says
/// `1.4999e-12` has quietly edited the model. When the spelling does not
/// survive, the raw form is shown.
///
/// The check runs against the *deck* spelling, because that is the one
/// [`crate::quantity::parse_engineering_value`] reads: `M` on a surface means
/// mega and in a deck means milli. Only a spelling that survives it is then
/// retyped for a reader (`Meg` → `M`, `u` → `µ`), and the digits are the same
/// ones, derived rather than recomputed.
pub(super) fn engineering_value(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let deck = crate::quantity::format_engineering_value(value);
    let round_trips = crate::quantity::parse_engineering_value(&deck).is_ok_and(|parsed| {
        (parsed - value).abs() <= value.abs() * ENGINEERING_ROUND_TRIP_TOLERANCE
    });
    if round_trips {
        crate::state::property_types::format_engineering_display_with(
            value,
            crate::quantity::EngineeringPrecision::Adaptive,
        )
    } else {
        value.to_string()
    }
}

/// [`engineering_value`] carrying its unit, with the decade prefix bound to the
/// unit rather than to the mantissa: `25nm`, `1.8V`, `2pF`.
pub(super) fn engineering_quantity(value: f64, unit: &str) -> String {
    format!("{}{unit}", engineering_value(value))
}

/// Clip a cell's text to its column.
///
/// This module carried its own copy that dropped one character at a time and
/// laid the whole string out again after each, so a name that had to lose
/// thirty characters cost thirty text layouts — paid per cell, per row, on a
/// table the size of the corpus. The design system's owner bisects instead,
/// and cuts on grapheme boundaries rather than `char`s.
pub(super) fn elide(ui: &Ui, value: &str, max_width: f32, mono: bool) -> String {
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    crate::workbench::design_system::elide_text(ui, value, &font, max_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The display spelling is offered only when the product's own parser reads
    /// it back to the value it was made from.
    ///
    /// A card parameter is engineering data, not decoration: a value shown as
    /// `1.5p` when the source says `1.4999e-12` has quietly edited the model,
    /// and nothing on the surface would say so.
    #[test]
    fn a_value_is_spelled_in_engineering_notation_only_when_it_survives_the_round_trip() {
        for (value, spelling) in [
            (0.48, "480m"),
            (1.8, "1.8"),
            (2.5e-8, "25n"),
            (1.0e-12, "1p"),
            (0.0, "0"),
            (1.5e6, "1.5M"),
        ] {
            assert_eq!(engineering_value(value), spelling, "{value}");
        }

        // The suffix a deck reads and the suffix a reader reads differ, and
        // the guard runs against the deck one: `M` here is mega, and in a deck
        // it is milli.
        assert_eq!(
            crate::quantity::parse_engineering_value("1.5Meg").expect("deck spelling parses"),
            1.5e6
        );

        // A value the notation cannot name lands raw rather than rounded.
        let unrepresentable = 1.234_567_890_123e-12;
        assert_eq!(
            engineering_value(unrepresentable),
            unrepresentable.to_string(),
            "a value the three-decimal ladder would round is shown as it is"
        );
        assert_eq!(engineering_value(f64::NAN), "NaN");
    }

    #[test]
    fn a_quantity_binds_its_decade_prefix_to_its_unit() {
        assert_eq!(engineering_quantity(2.5e-8, "m"), "25nm");
        assert_eq!(engineering_quantity(1.8, "V"), "1.8V");
        assert_eq!(engineering_quantity(27.0, " °C"), "27 °C");
    }

    fn header_output(active: ModelsCatalogSortKey, descending: bool) -> egui::FullOutput {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 200.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let _ = sortable_table_header(
                        ui,
                        "paint-test-header",
                        &[
                            sort_column("MODEL", 0.5, ModelsCatalogSortKey::Model),
                            sort_column("STATUS", 0.5, ModelsCatalogSortKey::Status),
                        ],
                        active,
                        descending,
                    );
                });
            },
        )
    }

    /// A header cell a reader can order the table with is a control, and is
    /// reachable as one.
    #[test]
    fn every_sortable_header_cell_publishes_a_button_a_reader_can_reach() {
        let nodes = header_output(ModelsCatalogSortKey::Model, false)
            .platform_output
            .accesskit_update
            .expect("the header publishes an access tree")
            .nodes;
        for name in ["sort by MODEL", "sort by STATUS"] {
            assert!(
                nodes.iter().any(|(_, node)| {
                    node.role() == egui::accesskit::Role::Button && node.label() == Some(name)
                }),
                "no reachable control named {name:?}"
            );
        }
    }

    /// The direction is drawn, never typed.
    ///
    /// The bundled face is missing several arrow glyphs, and a header that
    /// renders a tofu box says less than one with no mark at all — this app has
    /// shipped that defect before. The mark is a filled triangle, and it
    /// changes with the direction.
    #[test]
    fn the_ordered_column_carries_a_painted_direction_mark_rather_than_a_glyph() {
        fn marks(output: &egui::FullOutput) -> Vec<Vec<egui::Pos2>> {
            fn walk(shape: &egui::epaint::Shape, out: &mut Vec<Vec<egui::Pos2>>) {
                match shape {
                    egui::epaint::Shape::Path(path) if path.points.len() == 3 => {
                        out.push(path.points.clone());
                    }
                    egui::epaint::Shape::Vec(shapes) => {
                        for shape in shapes {
                            walk(shape, out);
                        }
                    }
                    _ => {}
                }
            }
            let mut found = Vec::new();
            for clipped in &output.shapes {
                walk(&clipped.shape, &mut found);
            }
            found
        }

        let ascending = marks(&header_output(ModelsCatalogSortKey::Model, false));
        let descending = marks(&header_output(ModelsCatalogSortKey::Model, true));
        assert_eq!(
            ascending.len(),
            1,
            "exactly one column is ordered, so exactly one mark is drawn"
        );
        assert_eq!(descending.len(), 1);
        assert_ne!(
            ascending, descending,
            "the mark points the other way when the order reverses"
        );

        // Nothing was typed: no painted text in the header says anything but
        // the two column names.
        let mut painted = Vec::new();
        for clipped in &header_output(ModelsCatalogSortKey::Model, false).shapes {
            if let egui::epaint::Shape::Text(text) = &clipped.shape {
                painted.push(text.galley.text().to_owned());
            }
        }
        assert_eq!(painted, ["MODEL", "STATUS"]);
    }
}

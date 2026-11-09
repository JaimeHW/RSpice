//! Form primitives — inspector rows, key-value rows, checkboxes, and the
//! mono value input.

use egui::{Response, TextEdit, Ui, vec2};

use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// Label column width of inspector form grids.
pub(crate) const LABEL_COL: f32 = 92.0;

/// Announce `name` as a control's accessible name.
///
/// Every label in this design system is painted rather than laid out as a
/// widget, so there is no label node for a control to be `labelled_by`. A
/// screen reader that reaches an input therefore hears whatever the widget
/// publishes for itself, which for [`TextEdit`] is nothing at all and for
/// [`egui::Checkbox`] is the word it happens to be showing — "Enabled" as the
/// name of the control rather than as its state.
///
/// Naming the node is the only route that survives that, and it belongs to the
/// constructors that use it so a call site cannot forget it: `mono_input` takes
/// the row's own label and there is no spelling of it that omits one.
pub fn name_control(ui: &Ui, response: &Response, name: &str) {
    debug_assert!(
        !name.trim().is_empty(),
        "a control announces the label its row states; an unnamed one is unreachable"
    );
    let name = name.to_owned();
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_label(name));
}

/// A monospace value input filling `width`, styled as an inset well.
///
/// `label` is what the row states beside or above the well, and becomes the
/// control's accessible name — see [`name_control`].
pub fn mono_input(ui: &mut Ui, label: &str, value: &mut String, width: f32) -> Response {
    let t = Tokens::get(ui.ctx());
    let response = ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        TextEdit::singleline(value)
            .font(theme::field_mono())
            .vertical_align(egui::Align::Center)
            .margin(egui::Margin::symmetric(8, 4)),
    );
    name_control(ui, &response, label);
    response
}

/// A single-line text field `width` wide at the theme's control height, with
/// its text centred in it.
///
/// egui sizes a bare [`TextEdit`] from its font's row and a two-point margin,
/// so a search or filter field stood 21 points tall beside 28-point selects
/// and buttons, and beside 44-point ones under a finger. The control height is
/// the one the theme raises to the touch target on a touch screen, so a field
/// laid out here is as tall as the controls beside it on either. The caller
/// keeps its own font, hint and margin.
pub fn text_field(ui: &mut Ui, edit: TextEdit<'_>, width: f32) -> Response {
    let height = Tokens::get(ui.ctx()).metrics.ctl_h;
    ui.add_sized(
        vec2(width, height),
        edit.vertical_align(egui::Align::Center),
    )
}

/// Inset between a well's trailing edge and the unit inside it.
const UNIT_INSET: f32 = 8.0;
/// Least air between a value's text and its unit.
const UNIT_GAP: f32 = 4.0;

/// [`mono_input`] with the value's unit inside the well's trailing edge, as
/// the inspector's property rows set theirs: in the sans face, which carries
/// `Ω` where the bundled mono face does not, and in the faint tone. The text
/// stops short of the unit, so a well keeps the width of the wells beside it
/// whether or not it has one, and the unit never sits on the label's line.
pub fn mono_input_with_unit(
    ui: &mut Ui,
    label: &str,
    value: &mut String,
    width: f32,
    unit: Option<&str>,
) -> Response {
    let Some(unit) = unit.filter(|unit| !unit.trim().is_empty()) else {
        return mono_input(ui, label, value, width);
    };
    let t = Tokens::get(ui.ctx());
    let value_font = theme::field_mono();
    let unit_galley = ui.painter().layout_no_wrap(
        unit.to_owned(),
        theme::sans(value_font.size, FontWeight::Regular),
        t.color.text_faint,
    );
    let trailing = (UNIT_INSET + unit_galley.size().x + UNIT_GAP).min(f32::from(i8::MAX));
    let margin = egui::Margin {
        left: 8,
        right: trailing as i8,
        top: 4,
        bottom: 4,
    };
    let response = ui.add_sized(
        vec2(width, t.metrics.ctl_h),
        TextEdit::singleline(value)
            .font(value_font.clone())
            .margin(margin),
    );
    name_control(ui, &response, label);
    // On the value's baseline: the well sets its text at its top margin, and
    // the two faces sit differently in their rows.
    let probe = ui
        .painter()
        .layout_no_wrap("0".to_owned(), value_font, t.color.text);
    let baseline = response.rect.top() + f32::from(margin.top) + first_baseline(&probe);
    ui.painter().with_clip_rect(response.rect).galley(
        egui::pos2(
            response.rect.right() - UNIT_INSET - unit_galley.size().x,
            baseline - first_baseline(&unit_galley),
        ),
        unit_galley,
        t.color.text_faint,
    );
    response
}

/// Where a laid-out line's baseline sits below the top of its galley.
fn first_baseline(galley: &egui::Galley) -> f32 {
    galley.rows.first().map_or(0.0, |placed| {
        placed.pos.y
            + placed
                .row
                .glyphs
                .first()
                .map_or(placed.row.size.y, |glyph| glyph.pos.y)
    })
}

/// An inspector form row: dimmed label in a fixed column, editable mono
/// value filling the remainder. Returns the edit response.
pub fn input_row(ui: &mut Ui, label: &str, value: &mut String) -> Response {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_COL, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            mono_input(ui, label, value, ui.available_width())
        },
    )
    .inner
}

/// A key-value row: dimmed key left, mono value right, at the size a field
/// beside it holds its value.
pub fn kv_row(ui: &mut Ui, key: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, 22.0), egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter();
    painter.text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        key,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        c.text_dim,
    );
    painter.text(
        egui::pos2(rect.right(), rect.center().y),
        egui::Align2::RIGHT_CENTER,
        value,
        theme::field_mono(),
        c.text,
    );
}

/// A read-only row of a form: dimmed key in the input rows' label column, mono
/// value where their fields start.
pub fn value_row(ui: &mut Ui, key: &str, value: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            let (key_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_COL, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(key_rect.left(), key_rect.center().y),
                egui::Align2::LEFT_CENTER,
                key,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            let (value_rect, response) =
                ui.allocate_exact_size(vec2(ui.available_width(), row_h), egui::Sense::hover());
            let font = theme::field_mono();
            let shown = crate::panels::elide_text(ui, value, &font, value_rect.width());
            ui.painter().text(
                egui::pos2(value_rect.left(), value_rect.center().y),
                egui::Align2::LEFT_CENTER,
                shown,
                font,
                c.text,
            );
            crate::panels::announce_row(ui, &response, key, value);
        },
    );
}

/// A labelled on/off row. Returns `true` if the value changed.
///
/// The control is this design system's switch, carried on a tree row so the
/// label column lines up with the input rows beside it. It painted a tick box
/// until the row it delegates to stopped: every boolean in the product reads as
/// the same control now, including the ones a narrow editor column falls back
/// to here for.
pub fn switch_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let row = super::tree::TreeRow::new(label)
        .switch(value)
        .height(t.metrics.row_h);
    row.show(ui).switch_changed
}

/// A labeled chip-choice row for small enums (sweep type, distribution).
/// Returns `true` if the selection changed.
pub fn choice_row(ui: &mut Ui, label: &str, options: &[&str], value: &mut usize) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let row_h = t.metrics.row_h;
    let mut changed = false;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), row_h),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            let (label_rect, _) =
                ui.allocate_exact_size(vec2(LABEL_COL + 4.0, row_h), egui::Sense::hover());
            ui.painter().text(
                egui::pos2(label_rect.left(), label_rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                theme::sans(tokens::FS_1, FontWeight::Regular),
                c.text_dim,
            );
            for (idx, option) in options.iter().enumerate() {
                if super::chip(ui, option, *value == idx).clicked() {
                    *value = idx;
                    changed = true;
                }
            }
        },
    );
    changed
}

/// A stacked label above its field, painted as written (sentence case).
///
/// Sans, like a property row's label: a mono label over a mono value reads
/// as one run of code once case no longer sets the label apart.
pub fn field_label<R>(ui: &mut Ui, label: &str, content: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
    content(ui)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A text field is as tall as the controls beside it: the pointer control
    /// height, and the touch control height under a finger.
    #[test]
    fn a_text_field_takes_the_control_height_under_a_pointer_and_a_finger() {
        for (target, height) in [(None, 28.0), (Some(44.0), 44.0)] {
            let ctx = egui::Context::default();
            crate::Theme::default().apply_responsive_metrics_with_target(&ctx, target);
            let mut query = String::new();
            let mut field = egui::Rect::NOTHING;
            let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    field = text_field(ui, TextEdit::singleline(&mut query), 170.0).rect;
                });
            });
            assert_eq!(Tokens::get(&ctx).metrics.ctl_h, height);
            assert!(
                (field.height() - height).abs() < 0.5 && (field.width() - 170.0).abs() < 0.5,
                "touch target {target:?}: the field is {field:?}"
            );
        }
    }

    /// A fact stated beside a field reads at the size the field holds its
    /// value: the row's value and the typed value are one size.
    #[test]
    fn a_fact_beside_a_field_reads_at_the_fields_size() {
        fn walk(shape: &egui::Shape, into: &mut Vec<(String, egui::FontId)>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
                egui::Shape::Text(text) => into.push((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.clone(),
                )),
                _ => {}
            }
        }
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        let mut name = "amp_core_copy".to_owned();
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                kv_row(ui, "Cell", "amp_core");
                let _ = input_row(ui, "New name", &mut name);
            });
        });
        let mut texts = Vec::new();
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut texts);
        }
        let font_of = |value: &str| {
            texts
                .iter()
                .find(|(text, _)| text == value)
                .map(|(_, font)| font.clone())
                .unwrap_or_else(|| panic!("{value:?} is painted"))
        };
        assert_eq!(font_of("amp_core"), theme::field_mono(), "the fact");
        assert_eq!(font_of("amp_core_copy"), theme::field_mono(), "the field");
    }
}

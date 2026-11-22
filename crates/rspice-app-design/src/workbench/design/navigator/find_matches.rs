//! Find's hits, at the top of the Navigator while its search box holds
//! text: one row per hit, the current one marked, each a click from being
//! shown on its sheet. The band counts them and says which one Enter showed
//! last ("3 of 17"). Unfolded, the Replace row sits under the band, every
//! hit reads as the text now and the text it becomes, and the tick in front
//! of it says whether Replace changes it.

use egui::{Ui, vec2};
use rspice_design::schematic::find_replace::{FindField, FindMatch};

use super::{DesignNavigatorSection, nav_row_indented_mono_response, navigator_section_header};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Button;
use crate::workbench::app::dialogs::find_replace::{
    DocumentMatches, FindState, find_hits, hits, replace_refusal, replace_ticked, replaced_summary,
    select_all_hits, show_hit,
};
use crate::workbench::design::DesignFeature;
use crate::workbench::design_system::{WorkbenchIcon, icon_button};
use crate::workbench::docks::navigator::empty_navigator_row;
use crate::workbench::framework::feature::Cx;

/// More rows than this are summed up in one, so a pattern that matches
/// everything cannot bury the rails under it.
const MAX_ROWS: usize = 200;
/// Air at the ends of the band's own rows, as the search box above has.
const ROW_MARGIN_X: f32 = 10.0;
/// The tick's box, in the column a disclosure caret takes on other rows.
const TICK_SIZE: f32 = 11.0;
const TICK_CENTER_X: f32 = 14.0;
/// How far from a row's left edge a click lands on its tick.
const TICK_HIT_W: f32 = 24.0;

pub(super) fn find_section(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>) {
    let find = cx.wb.navigator_filter().trim().to_owned();
    if find.is_empty() && !cx.state.find_replace.replacing {
        return;
    }
    let found = if find.is_empty() {
        Err("Type the text to find in the search box.".to_owned())
    } else {
        find_hits(&find, &cx.state.find_replace, cx.model)
    };
    let total = found.as_ref().map_or(0, |found| hits(found).count());
    let position = found
        .as_ref()
        .ok()
        .and_then(|found| cx.state.find_replace.current_position(found));
    let count = match (&found, position) {
        (Err(_), _) => "\u{2014}".to_owned(),
        (Ok(_), Some(at)) => format!("{} of {total}", at + 1),
        (Ok(_), None) => total.to_string(),
    };
    if !navigator_section_header(ui, DesignNavigatorSection::Find, &count) {
        return;
    }
    options_row(ui, &mut cx.state.find_replace);
    if cx.state.find_replace.replacing {
        let edit_in_progress = cx.state.schematic_edit_in_progress(cx.model);
        let refusal = replace_refusal(&cx.state.find_replace, &found, edit_in_progress, cx.model);
        if replace_row(ui, &mut cx.state.find_replace.replace, refusal.as_deref()) {
            let outcome = replace_ticked(
                &mut cx.state.find_replace,
                &find,
                edit_in_progress,
                cx.model,
                cx.wb,
            );
            cx.wb.prompt(match outcome {
                Ok((texts, documents)) => replaced_summary(texts, documents),
                Err(reason) => reason,
            });
            return;
        }
    }
    let found = match found {
        Ok(found) => found,
        Err(reason) => {
            empty_navigator_row(ui, &reason);
            return;
        }
    };
    if total == 0 {
        empty_navigator_row(ui, "No matches");
        return;
    }
    if let Some((document, hit)) = hit_rows(ui, &cx.state.find_replace, &found, position) {
        match hit {
            Pick::Show(at) => {
                let Some((_, found_hit)) = hits(&found).nth(at) else {
                    return;
                };
                cx.state.find_replace.current = Some(document.hit_key(found_hit));
                show_hit(cx.state, cx.model, cx.wb, cx.runs, &document, found_hit);
            }
            Pick::Tick(key) => cx.state.find_replace.toggle_tick(key),
        }
    }
    let response = nav_row_indented_mono_response(
        ui,
        WorkbenchIcon::SelectBox,
        "Select all hits",
        false,
        None,
        0,
    );
    if response.clicked() {
        select_all_hits(cx.model, cx.wb, &found);
    }
}

/// What a press on a hit row asks for.
enum Pick {
    /// Show the hit at this place in the stepping order.
    Show(usize),
    /// Turn the hit's tick over.
    Tick(crate::workbench::app::dialogs::find_replace::HitKey),
}

/// The hit rows, under a row naming each cell view where more than one has
/// hits. Returns what a press asked for, with the cell view it is in.
fn hit_rows(
    ui: &mut Ui,
    form: &FindState,
    found: &[DocumentMatches],
    current: Option<usize>,
) -> Option<(DocumentMatches, Pick)> {
    let several = found.len() > 1;
    let total = hits(found).count();
    let mut shown = 0;
    let mut picked = None;
    for document in found {
        if several {
            let count = document.matches.len().to_string();
            let meta = if document.read_only {
                format!("read-only \u{00b7} {count}")
            } else {
                count
            };
            nav_row_indented_mono_response(
                ui,
                WorkbenchIcon::Design,
                &document.path,
                false,
                Some(&meta),
                0,
            );
        }
        for found in &document.matches {
            if shown == MAX_ROWS {
                let rest = format!("{} more", total - shown);
                nav_row_indented_mono_response(ui, WorkbenchIcon::Search, &rest, false, None, 1);
                return picked;
            }
            let at = shown;
            shown += 1;
            let label = if form.replacing {
                format!("{} \u{2192} {}", found.before, found.after)
            } else {
                found.before.clone()
            };
            let level = usize::from(several);
            let response = nav_row_indented_mono_response(
                ui,
                icon(&found.field),
                &label,
                current == Some(at),
                Some(&meta(found)),
                level,
            );
            if form.replacing {
                paint_tick(
                    ui,
                    &response,
                    level,
                    form.ticks(document, found),
                    !document.read_only,
                );
            }
            if response.clicked() {
                let on_tick = form.replacing
                    && !document.read_only
                    && response.interact_pointer_pos().is_some_and(|pointer| {
                        pointer.x < response.rect.left() + TICK_HIT_W + 14.0 * level as f32
                    });
                let pick = if on_tick {
                    Pick::Tick(document.hit_key(found))
                } else {
                    Pick::Show(at)
                };
                picked = Some((document.clone(), pick));
            }
        }
    }
    picked
}

/// The tick in front of a hit: filled where Replace changes it, empty where
/// the reader unticked it, faint where its cell view cannot change.
fn paint_tick(ui: &Ui, row: &egui::Response, level: usize, ticked: bool, editable: bool) {
    let t = Tokens::get(ui.ctx());
    let center = egui::pos2(
        row.rect.left() + TICK_CENTER_X + 14.0 * level as f32,
        row.rect.center().y,
    );
    let rect = egui::Rect::from_center_size(center, vec2(TICK_SIZE, TICK_SIZE));
    let ink = if editable {
        t.color.text_dim
    } else {
        t.color.text_faint
    };
    if ticked {
        ui.painter().rect_filled(rect, 2.0, t.color.accent);
        WorkbenchIcon::Check.paint(ui.painter(), rect.shrink(1.0), t.color.bg_panel);
    } else {
        ui.painter().rect_stroke(
            rect,
            2.0,
            egui::Stroke::new(1.0, ink),
            egui::StrokeKind::Inside,
        );
    }
}

/// The Replace row's fold, then how the text is matched.
fn options_row(ui: &mut Ui, form: &mut FindState) {
    let t = Tokens::get(ui.ctx());
    let square = vec2(t.metrics.ctl_h, t.metrics.ctl_h);
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        ui.add_space(ROW_MARGIN_X);
        let (icon, label) = if form.replacing {
            (WorkbenchIcon::ChevronDown, "Hide Replace")
        } else {
            (WorkbenchIcon::ChevronRight, "Replace")
        };
        if icon_button(ui, icon, label, false, square)
            .on_hover_text(label)
            .clicked()
        {
            form.replacing = !form.replacing;
        }
        for (on, icon, label) in [
            (
                &mut form.options.match_case,
                WorkbenchIcon::MatchCase,
                "Match case",
            ),
            (
                &mut form.options.whole_word,
                WorkbenchIcon::WholeWord,
                "Whole word",
            ),
            (
                &mut form.options.regex,
                WorkbenchIcon::RegularExpression,
                "Regular expression",
            ),
        ] {
            if icon_button(ui, icon, label, *on, square)
                .on_hover_text(label)
                .clicked()
            {
                *on = !*on;
            }
        }
    });
    ui.add_space(4.0);
}

/// The replacement and the button that makes it, with the reason it is
/// refused under them. Returns whether Replace was pressed.
fn replace_row(ui: &mut Ui, replace: &mut String, refusal: Option<&str>) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut pressed = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.add_space(ROW_MARGIN_X);
        let button = Button::new("Replace").compact().enabled(refusal.is_none());
        let button_width = button.measured_width(ui);
        let field_width = (ui.available_width() - button_width - 6.0 - ROW_MARGIN_X).max(60.0);
        ui.add_sized(
            [field_width, t.metrics.ctl_h],
            egui::TextEdit::singleline(replace).hint_text("Replace with"),
        );
        pressed = button.show(ui).clicked();
    });
    if let Some(refusal) = refusal {
        egui::Frame::new()
            .inner_margin(egui::Margin::symmetric(ROW_MARGIN_X as i8, 2))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(refusal)
                        .size(crate::ui::tokens::FS_1)
                        .color(t.color.warn),
                );
            });
    }
    ui.add_space(4.0);
    pressed
}

fn icon(field: &FindField) -> WorkbenchIcon {
    match field {
        FindField::InstanceName => WorkbenchIcon::Component,
        FindField::NetLabel => WorkbenchIcon::Label,
        FindField::Value | FindField::Parameter(_) => WorkbenchIcon::Sliders,
    }
}

/// What the row's text is: an instance's own name, or which of its fields.
fn meta(found: &FindMatch) -> String {
    match found.field {
        FindField::InstanceName | FindField::NetLabel => found.field.label(),
        _ => format!("{} {}", found.owner, found.field.label()),
    }
}

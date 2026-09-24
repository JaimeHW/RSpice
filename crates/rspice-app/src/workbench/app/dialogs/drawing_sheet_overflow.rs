//! Live review of printable content outside the authored drawing sheet.
//!
//! This is an advisory surface. It never mutates schematic content itself;
//! navigation selects an exact stable target, while the three resolution
//! actions hand off to existing view, Page Setup, and hardcopy authorities.

use egui::{Align, Context, Frame, Label, Layout, RichText, Stroke, Ui, vec2};

use crate::schematic::view::SchematicSymbolContext;
use crate::schematic::view::drawing_sheet::{
    ActiveDrawingSheet, DrawingSheetOverflowItem, DrawingSheetOverflowSummary,
    DrawingSheetOverflowTarget, drawing_sheet_overflow_summary, show_drawing_sheet_overflow_target,
};
use crate::state::{DrawingSheetDisplayUnit, ViewType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::app::{HardcopyWorkflow, RSpiceApp};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "DRAWING SHEET \u{00b7} ADVISORY \u{00b7} NO BLOCKING EFFECT";
const TITLE: &str = "Content outside the drawing sheet";
const DESCRIPTION: &str = "Every object that falls outside the authored drawing area of this sheet, with its sheet coordinates and a way to reach it.";

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum OverflowReviewAction {
    #[default]
    None,
    Show(DrawingSheetOverflowTarget),
    FitContent,
    PageSetup,
    Hardcopy,
}

/// Open the live, itemized outside-sheet review from the inspector.
pub(crate) fn open_drawing_sheet_overflow_review(state: &mut AppState) -> bool {
    if state.dialogs.drawing_sheet_overflow_open
        || !matches!(
            state.workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        )
    {
        return false;
    }
    state.dialogs.drawing_sheet_overflow_open = true;
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_overflow_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.drawing_sheet_overflow_open {
            return;
        }

        let (report, format, display_unit) = {
            let symbol_context = SchematicSymbolContext::from_state(&self.state);
            let sheet = ActiveDrawingSheet::resolve(&self.state);
            let format = sheet.format_label();
            (
                drawing_sheet_overflow_summary(&self.state, &symbol_context, &sheet),
                format,
                sheet.format.display_unit,
            )
        };
        let mut action = OverflowReviewAction::None;
        let choice = Dialog::new(EYEBROW, TITLE, "Close")
            .description(DESCRIPTION)
            .size(DialogSize::DrawingSheetWorkflow)
            .initial_height(610.0)
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::Primary)
            .show(ctx, |ui| {
                action = overflow_review_body(ui, &report, &format, display_unit);
            });

        match choice {
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_overflow_open = false;
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
        match action {
            OverflowReviewAction::None => {}
            OverflowReviewAction::Show(target) => {
                self.state.dialogs.drawing_sheet_overflow_open = false;
                show_drawing_sheet_overflow_target(&mut self.state, target);
            }
            OverflowReviewAction::FitContent => {
                self.state.dialogs.drawing_sheet_overflow_open = false;
                self.state.schematic.needs_fit = true;
                self.state.schematic.needs_drawing_sheet_fit = false;
            }
            OverflowReviewAction::PageSetup => {
                self.state.dialogs.drawing_sheet_overflow_open = false;
                crate::workbench::app::open_drawing_sheet_setup(self);
            }
            OverflowReviewAction::Hardcopy => {
                self.state.dialogs.drawing_sheet_overflow_open = false;
                crate::workbench::app::open_hardcopy_workflow(self, HardcopyWorkflow::Print);
            }
        }
    }
}

fn overflow_review_body(
    ui: &mut Ui,
    report: &DrawingSheetOverflowSummary,
    format: &str,
    display_unit: DrawingSheetDisplayUnit,
) -> OverflowReviewAction {
    let mut action = OverflowReviewAction::None;
    advisory_banner(ui, report, format);
    context_facts(ui, report);
    ui.add_space(8.0);
    detection_contract(ui);
    ui.add_space(10.0);
    if !report.items.is_empty() {
        overflow_table(ui, &report.items, display_unit, &mut action);
        ui.add_space(12.0);
        resolution_actions(ui, &mut action);
    }
    action
}

fn detection_contract(ui: &mut Ui) {
    let tokens = Tokens::get(ui.ctx());
    let response = Frame::NONE
        .fill(tokens.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.label(
                "Detection is geometric and re-runs after a move, a paste, a format change or an orientation change, so no separate invalidation can leave the canvas, the inspector and Page Setup disagreeing. A conductor crossing the border counts once. Durable labels, junctions, design notes and documentation geometry are printable sheet content and are counted. Probe flags and transient interaction markers are view state and are never printed, so they are never counted. A title-block overlap is on the page but will print underneath the identity block, which is why it is reported apart from the rest.",
            );
        })
        .response;
    ui.painter().vline(
        response.rect.left(),
        response.rect.y_range(),
        Stroke::new(2.0, tokens.color.accent),
    );
}

fn advisory_banner(ui: &mut Ui, report: &DrawingSheetOverflowSummary, format: &str) {
    let tokens = Tokens::get(ui.ctx());
    let accent = if report.is_clear() {
        tokens.color.ok
    } else {
        tokens.color.warn
    };
    Frame::NONE
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(1.0, tokens.color.border_strong))
        .inner_margin(egui::Margin::symmetric(12, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let (marker, _) = ui.allocate_exact_size(vec2(3.0, 42.0), egui::Sense::hover());
                ui.painter().rect_filled(marker, 0.0, accent);
                ui.vertical(|ui| {
                    let heading = if report.is_clear() {
                        "Every printable object is inside the drawing area.".to_owned()
                    } else {
                        format!(
                            "{} {} outside the drawing area of {format}.",
                            report.finding_count(),
                            if report.finding_count() == 1 {
                                "object is"
                            } else {
                                "objects are"
                            }
                        )
                    };
                    ui.label(RichText::new(heading).strong());
                    ui.label(
                        "This is advisory. The sheet saves, netlists, checks and simulates exactly as it would with everything inside. Print and export ask whether output clips to the sheet or extends the media.",
                    );
                });
            });
        });
}

fn context_facts(ui: &mut Ui, report: &DrawingSheetOverflowSummary) {
    let tokens = Tokens::get(ui.ctx());
    ui.columns(4, |columns| {
        fact(&mut columns[0], "Detected", "on every redraw", &tokens);
        fact(
            &mut columns[1],
            "Counted",
            "instances \u{00b7} conductors \u{00b7} labels \u{00b7} documentation",
            &tokens,
        );
        fact(
            &mut columns[2],
            "Not counted",
            "probe flags \u{00b7} transient markers",
            &tokens,
        );
        fact(
            &mut columns[3],
            "Title-block overlaps",
            if report.title_block_collisions == 0 {
                "counted separately \u{00b7} none"
            } else {
                "counted separately"
            },
            &tokens,
        );
    });
}

fn fact(ui: &mut Ui, label: &str, value: &str, tokens: &Tokens) {
    ui.vertical(|ui| {
        ui.label(RichText::new(label).color(tokens.color.text_faint));
        ui.add(
            Label::new(
                RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(tokens.color.text),
            )
            .wrap(),
        );
    });
}

fn overflow_table(
    ui: &mut Ui,
    items: &[DrawingSheetOverflowItem],
    display_unit: DrawingSheetDisplayUnit,
    action: &mut OverflowReviewAction,
) {
    let tokens = Tokens::get(ui.ctx());
    let available = (ui.available_width() - 2.0).max(720.0);
    let widths = [
        available * 0.17,
        available * 0.09,
        available * 0.21,
        available * 0.20,
        available * 0.11,
        available * 0.22,
    ];
    Frame::NONE
        .stroke(Stroke::new(1.0, tokens.color.border_strong))
        .show(ui, |ui| {
            table_row(ui, widths, None, true, display_unit, action);
            for item in items {
                table_row(ui, widths, Some(item), false, display_unit, action);
            }
        });
}

fn table_row(
    ui: &mut Ui,
    widths: [f32; 6],
    item: Option<&DrawingSheetOverflowItem>,
    header: bool,
    display_unit: DrawingSheetDisplayUnit,
    action: &mut OverflowReviewAction,
) {
    let tokens = Tokens::get(ui.ctx());
    let row_height = if header { 28.0 } else { 36.0 };
    let response = Frame::NONE
        .fill(if header {
            tokens.color.bg_elevated
        } else {
            tokens.color.bg_app
        })
        .show(ui, |ui| {
            ui.set_height(row_height);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                if header {
                    for (width, text) in widths.into_iter().zip([
                        "OBJECT",
                        "KIND",
                        "WHERE",
                        "SHEET COORDINATES",
                        "ZONE",
                        "ACTION",
                    ]) {
                        table_cell(ui, width, row_height, text, true, false);
                    }
                    return;
                }
                let Some(item) = item else {
                    return;
                };
                table_cell(ui, widths[0], row_height, &item.identity, false, true);
                table_cell(ui, widths[1], row_height, item.kind, false, false);
                table_status_cell(ui, widths[2], row_height, item);
                table_cell(
                    ui,
                    widths[3],
                    row_height,
                    &format_sheet_coordinates(item.sheet_coordinates_mm, display_unit),
                    false,
                    true,
                );
                table_cell(
                    ui,
                    widths[4],
                    row_height,
                    item.zone.as_deref().unwrap_or("Off sheet"),
                    false,
                    true,
                );
                ui.allocate_ui_with_layout(
                    vec2(widths[5], row_height),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        if Button::new("Show on sheet").ghost().show(ui).clicked() {
                            *action = OverflowReviewAction::Show(item.target);
                        }
                    },
                );
            });
        })
        .response;
    ui.painter().hline(
        response.rect.x_range(),
        response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
}

fn format_sheet_coordinates(
    coordinates_mm: (f64, f64),
    display_unit: DrawingSheetDisplayUnit,
) -> String {
    let (scale, suffix) = match display_unit {
        DrawingSheetDisplayUnit::Millimetres => (1.0, "mm"),
        DrawingSheetDisplayUnit::Centimetres => (10.0, "cm"),
        DrawingSheetDisplayUnit::Inches => (25.4, "in"),
    };
    format!(
        "{:.3}, {:.3} {suffix}",
        coordinates_mm.0 / scale,
        coordinates_mm.1 / scale
    )
}

fn table_cell(ui: &mut Ui, width: f32, height: f32, text: &str, header: bool, mono: bool) {
    let tokens = Tokens::get(ui.ctx());
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(
            tokens::FS_0,
            if header {
                FontWeight::SemiBold
            } else {
                FontWeight::Regular
            },
        )
    };
    ui.add_sized(
        [width, height],
        Label::new(RichText::new(text).font(font).color(if header {
            tokens.color.text_faint
        } else {
            tokens.color.text
        }))
        .truncate(),
    );
}

fn table_status_cell(ui: &mut Ui, width: f32, height: f32, item: &DrawingSheetOverflowItem) {
    let tokens = Tokens::get(ui.ctx());
    let color = if item.off_paper {
        tokens.color.err
    } else {
        tokens.color.warn
    };
    ui.add_sized(
        [width, height],
        Label::new(
            RichText::new(item.severity.label())
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(color),
        )
        .truncate(),
    );
}

fn resolution_actions(ui: &mut Ui, action: &mut OverflowReviewAction) {
    ui.label(RichText::new("Resolutions").strong());
    ui.add_space(6.0);
    ui.columns(3, |columns| {
        resolution_card(
            &mut columns[0],
            "Move the content",
            "Select the objects and drag them inside the drawing area, or use Fit content to see everything at once first.",
            "Fit content",
            OverflowReviewAction::FitContent,
            action,
        );
        resolution_card(
            &mut columns[1],
            "Grow the sheet",
            "A larger format or the other orientation may be the honest answer; Page Setup previews the result before applying.",
            "Page setup\u{2026}",
            OverflowReviewAction::PageSetup,
            action,
        );
        resolution_card(
            &mut columns[2],
            "Leave it and decide at output",
            "Print and export state exactly what happens to content outside the sheet, per output, and never decide silently.",
            "Print and hardcopy\u{2026}",
            OverflowReviewAction::Hardcopy,
            action,
        );
    });
}

fn resolution_card(
    ui: &mut Ui,
    title: &str,
    description: &str,
    button: &str,
    next: OverflowReviewAction,
    action: &mut OverflowReviewAction,
) {
    let tokens = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_min_height(112.0);
            ui.label(RichText::new(title).strong());
            ui.label(description);
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                if Button::new(button).show(ui).clicked() {
                    *action = next;
                }
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_opener_is_modal_and_idempotent_for_schematic_views() {
        let mut state = AppState::default();
        assert!(open_drawing_sheet_overflow_review(&mut state));
        assert!(!open_drawing_sheet_overflow_review(&mut state));
        assert!(state.dialogs.drawing_sheet_overflow_open);
        assert!(state.dialogs.application_modal_open());
    }
}

//! The project's design-rule deck: every rule the checker can report, what it
//! is reported as, and what it is for.
//!
//! The severities themselves live on the project — see
//! [`rspice_design::drc::DesignRuleDeck`] — because which findings block a
//! sign-off is a decision a team shares. This page is where that decision is
//! made, and it is a page rather than a modal because a reader setting one
//! rule is usually reading the rest of the deck at the same time.
//!
//! A rule's default severity is stated beside the control rather than replaced
//! by it: a deck is only auditable if the reader can see both what RSpice
//! reports a finding as and what this project decided instead.

use std::sync::Arc;

use egui::{Galley, Stroke, Ui, WidgetInfo, WidgetType};

use rspice_design::drc::{DesignRuleDeck, DrcRuleCategory, DrcSeverity, DrcViolationType};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::select;

use super::AppState;
use super::preferences_shell::{content_inset, filtering, row_passes_filter};

const COLUMNS: usize = 5;

/// Column widths as fractions of the table. The rule's own name and the two
/// prose columns take the width; the default severity needs only a word.
const COLUMN_FRACTIONS: [f32; COLUMNS] = [0.20, 0.09, 0.155, 0.2775, 0.2775];

const HEADINGS: [&str; COLUMNS] = [
    "Rule",
    "Default",
    "Reported as",
    "What it finds",
    "Remediation",
];

/// The narrowest the table is laid out at. Below it the page scrolls
/// horizontally rather than wrapping five columns into unreadable slivers,
/// which is what the command-binding table in this same dialog does.
const TABLE_MIN_WIDTH: f32 = 1_000.0;
/// Horizontal padding inside a cell, each side.
const CELL_PAD: f32 = 8.0;
/// Vertical padding above and below a row's tallest cell.
const ROW_PAD: f32 = 6.0;
const HEADER_HEIGHT: f32 = 27.0;
const BAND_HEIGHT: f32 = 24.0;
/// Width of the gutter mark on a rule this project has decided for itself.
const OVERRIDE_MARK_WIDTH: f32 = 2.0;

pub(super) fn render(ui: &mut Ui, state: &mut AppState) {
    let banded = !filtering(ui);
    let rules = visible_rules(ui);
    if rules.is_empty() {
        return;
    }
    if banded {
        deck_status(ui, &state.workspace.content.design_rules);
    }
    let read_only = state.workbench.safe_mode.project_read_only();
    let deck = state.workspace.content.design_rules.clone();
    if let Some((rule, severity)) = table(ui, &deck, &rules, banded, read_only) {
        // Nothing is written unless the pick actually moves the deck, so
        // reselecting the severity a rule already has does not mark the project
        // changed. A pick equal to the rule's own default removes the entry:
        // see `DesignRuleDeck::set_severity`.
        if state
            .workspace
            .content
            .design_rules
            .set_severity(rule, severity)
        {
            state.workspace.content.project_metadata_dirty = true;
        }
    }
}

/// Every rule still on the page under the shell's search, in deck order.
fn visible_rules(ui: &mut Ui) -> Vec<DrcViolationType> {
    DrcViolationType::ALL
        .into_iter()
        .filter(|rule| row_passes_filter(ui, rule.label(), &searchable_text(*rule)))
        .collect()
}

/// What a query is tested against besides the rule's name: its published code,
/// so a reader who has a finding in front of them can search the code they were
/// shown, and both prose columns.
fn searchable_text(rule: DrcViolationType) -> String {
    format!(
        "{} {} {}",
        rule.rule_code(),
        rule.description(),
        rule.suggested_fix()
    )
}

/// Where the deck is stored, and how much of it this project has decided.
///
/// One line, because the page is read for the table under it: a reader opening
/// a settings page needs to know that these severities travel with the project
/// and not with their install, and a reviewer needs to know whether the deck
/// deviates from the shipped defaults at all.
fn deck_status(ui: &mut Ui, deck: &DesignRuleDeck) {
    let t = Tokens::get(ui.ctx());
    let inset = content_inset(ui);
    let text = format!(
        "Saved with the project \u{00b7} {} of {} rules set here",
        deck.len(),
        DrcViolationType::ALL.len()
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let galley = ui
        .painter()
        .layout_no_wrap(text.clone(), font, t.color.text_dim);
    let height = galley.size().y + 2.0 * ROW_PAD;
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), height), egui::Sense::hover());
    response.widget_info(|| {
        let mut info = WidgetInfo::new(WidgetType::Label);
        info.value = Some(text.clone().into());
        info
    });
    if ui.is_rect_visible(rect) {
        ui.painter().galley(
            egui::pos2(rect.left() + inset, rect.center().y - galley.size().y * 0.5),
            galley,
            t.color.text_dim,
        );
    }
}

/// The exact left edge and width of each column, in the row's own space.
fn column_geometry(table_width: f32, inset: f32) -> [(f32, f32); COLUMNS] {
    let mut columns = [(0.0, 0.0); COLUMNS];
    let mut left = inset;
    for (index, fraction) in COLUMN_FRACTIONS.iter().enumerate() {
        let width = if index + 1 == COLUMNS {
            // The last column closes the table exactly, so rounding the
            // others down cannot leave a sliver of unpainted header.
            (inset + table_width - left).max(1.0)
        } else {
            (table_width * fraction).floor()
        };
        columns[index] = (left, width);
        left += width;
    }
    columns
}

fn table(
    ui: &mut Ui,
    deck: &DesignRuleDeck,
    rules: &[DrcViolationType],
    banded: bool,
    read_only: bool,
) -> Option<(DrcViolationType, DrcSeverity)> {
    let inset = content_inset(ui);
    let table_width = (ui.available_width() - 2.0 * inset).max(TABLE_MIN_WIDTH);
    let columns = column_geometry(table_width, inset);
    let full_width = table_width + 2.0 * inset;
    let mut picked = None;
    egui::ScrollArea::horizontal()
        .id_salt("preferences.design-rules.table")
        .show(ui, |ui| {
            ui.set_min_width(full_width);
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            let table = ui
                .vertical(|ui| {
                    header_row(ui, full_width, &columns);
                    let mut band = None;
                    for rule in rules.iter().copied() {
                        if banded && band != Some(rule.category()) {
                            band = Some(rule.category());
                            category_band(ui, full_width, inset, rule.category());
                        }
                        if let Some(severity) =
                            rule_row(ui, deck, rule, full_width, &columns, read_only)
                        {
                            picked = Some((rule, severity));
                        }
                    }
                })
                .response;
            table.widget_info(|| {
                WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), "Design rules")
            });
            ui.ctx().accesskit_node_builder(table.id, |node| {
                node.set_role(egui::accesskit::Role::Table);
                node.set_label("Design rules");
            });
        });
    picked
}

fn header_row(ui: &mut Ui, full_width: f32, columns: &[(f32, f32); COLUMNS]) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(full_width, HEADER_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), "Design rule columns")
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label("Design rule columns");
    });
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, t.color.bg_panel);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    for (heading, (left, width)) in HEADINGS.iter().zip(columns) {
        let mut job = egui::text::LayoutJob::default();
        job.wrap.max_width = (width - 2.0 * CELL_PAD).max(1.0);
        job.append(
            &heading.to_uppercase(),
            0.0,
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_0, FontWeight::Medium),
                color: t.color.text_faint,
                extra_letter_spacing: 0.04 * tokens::FS_0,
                ..Default::default()
            },
        );
        let galley = painter.layout_job(job);
        painter.galley(
            egui::pos2(
                rect.left() + left,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            t.color.text_faint,
        );
    }
}

/// A band naming the run of rules under it.
///
/// The categories are the checker's own grouping, so a reader looking for a bus
/// rule reads one band of fourteen rather than scanning twenty-eight.
fn category_band(ui: &mut Ui, full_width: f32, inset: f32, category: DrcRuleCategory) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(full_width, BAND_HEIGHT), egui::Sense::hover());
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), category.label())
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::RowHeader);
        node.set_label(category.label());
    });
    if !ui.is_rect_visible(rect) {
        return;
    }
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let galley = painter.layout_no_wrap(
        category.label().to_owned(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    painter.galley(
        egui::pos2(
            rect.left() + inset,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text_dim,
    );
}

/// The wrapped text of one rule's row, laid out before the row is allocated so
/// the row is as tall as its tallest cell rather than clipping it.
struct RowText {
    name: Arc<Galley>,
    code: Arc<Galley>,
    default: Arc<Galley>,
    finds: Arc<Galley>,
    remediation: Arc<Galley>,
}

impl RowText {
    fn layout(ui: &Ui, rule: DrcViolationType, columns: &[(f32, f32); COLUMNS]) -> Self {
        let t = Tokens::get(ui.ctx());
        let painter = ui.painter();
        let text_width = |index: usize| (columns[index].1 - 2.0 * CELL_PAD).max(1.0);
        Self {
            name: painter.layout(
                rule.label().to_owned(),
                theme::sans(tokens::FS_1, FontWeight::Medium),
                t.color.text,
                text_width(0),
            ),
            code: painter.layout(
                rule.rule_code(),
                theme::mono(tokens::FS_MICRO, FontWeight::Regular),
                t.color.text_faint,
                text_width(0),
            ),
            default: painter.layout_no_wrap(
                rule.default_severity().display_name().to_owned(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
            ),
            finds: painter.layout(
                rule.description().to_owned(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
                text_width(3),
            ),
            remediation: painter.layout(
                rule.suggested_fix().to_owned(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
                text_width(4),
            ),
        }
    }

    fn height(&self, control_height: f32) -> f32 {
        let tallest = (self.name.size().y + self.code.size().y)
            .max(self.default.size().y)
            .max(self.finds.size().y)
            .max(self.remediation.size().y);
        (tallest + 2.0 * ROW_PAD).max(control_height + 2.0 * ROW_PAD)
    }
}

/// The option text for one severity on one rule.
///
/// The rule's own default is marked in the list, so the closed control states
/// whether this project has decided anything about the rule without the reader
/// having to compare two columns.
fn option_label(rule: DrcViolationType, severity: DrcSeverity) -> String {
    if severity == rule.default_severity() {
        format!("{} \u{00b7} default", severity.display_name())
    } else {
        severity.display_name().to_owned()
    }
}

fn rule_row(
    ui: &mut Ui,
    deck: &DesignRuleDeck,
    rule: DrcViolationType,
    full_width: f32,
    columns: &[(f32, f32); COLUMNS],
    read_only: bool,
) -> Option<DrcSeverity> {
    let t = Tokens::get(ui.ctx());
    let effective = deck.effective_severity(rule);
    let overridden = deck.severity(rule).is_some();
    let text = RowText::layout(ui, rule, columns);
    let height = text.height(t.metrics.ctl_h);
    let row_label = format!(
        "{} \u{00b7} default {} \u{00b7} reported as {}",
        rule.label(),
        rule.default_severity().display_name(),
        effective.display_name()
    );

    let outer = ui.cursor().min;
    let rect = egui::Rect::from_min_size(outer, egui::vec2(full_width, height));
    let response = ui.allocate_rect(rect, egui::Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), &row_label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(row_label.as_str());
    });

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            Stroke::new(1.0, t.color.border),
        );
        if overridden {
            // A rule this project decided carries a gutter mark, so a reviewer
            // can see which rows deviate without reading every severity.
            painter.rect_filled(
                egui::Rect::from_min_max(
                    rect.left_top(),
                    egui::pos2(rect.left() + OVERRIDE_MARK_WIDTH, rect.bottom() - 1.0),
                ),
                0.0,
                t.color.accent,
            );
        }
        let name_top = rect.top() + ROW_PAD;
        painter.galley(
            egui::pos2(rect.left() + columns[0].0, name_top),
            text.name.clone(),
            t.color.text,
        );
        painter.galley(
            egui::pos2(
                rect.left() + columns[0].0,
                name_top + text.name.size().y,
            ),
            text.code.clone(),
            t.color.text_faint,
        );
        let centered = |galley: &Arc<Galley>, column: usize| {
            egui::pos2(
                rect.left() + columns[column].0,
                rect.center().y - galley.size().y * 0.5,
            )
        };
        painter.galley(
            centered(&text.default, 1),
            text.default.clone(),
            if overridden {
                t.color.text_faint
            } else {
                t.color.text_dim
            },
        );
        painter.galley(
            egui::pos2(rect.left() + columns[3].0, rect.top() + ROW_PAD),
            text.finds.clone(),
            t.color.text_dim,
        );
        painter.galley(
            egui::pos2(rect.left() + columns[4].0, rect.top() + ROW_PAD),
            text.remediation.clone(),
            t.color.text_dim,
        );
    }

    let control_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.left() + columns[2].0,
            rect.center().y - t.metrics.ctl_h * 0.5,
        ),
        egui::vec2(
            (columns[2].1 - CELL_PAD).max(1.0),
            t.metrics.ctl_h,
        ),
    );
    let mut picked = None;
    let mut control = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(control_rect)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    if read_only {
        let galley = control.painter().layout_no_wrap(
            effective.display_name().to_owned(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        control.painter().galley(
            egui::pos2(
                control_rect.left(),
                control_rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            t.color.text_dim,
        );
    } else {
        let options = DrcSeverity::ALL
            .iter()
            .map(|severity| option_label(rule, *severity))
            .collect::<Vec<_>>();
        if let Some(index) = select(
            &mut control,
            &format!("preferences.design-rules.{}", rule.rule_code()),
            rule.label(),
            &option_label(rule, effective),
            &options,
            control_rect.width(),
        ) {
            picked = Some(DrcSeverity::ALL[index]);
        }
    }
    picked
}

/// Put every rule back on its own default.
///
/// The deck is the only thing this page stores, so the footer's reset clears
/// it outright rather than restoring a preference key. It takes the project
/// workspace rather than the whole session because the caller is holding a
/// borrow of the preference store while it resets each page.
pub(super) fn restore_defaults(project: &mut rspice_project::ProjectWorkspace) {
    if project.design_rules.is_empty() {
        return;
    }
    project.design_rules.clear_all();
    project.project_metadata_dirty = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::diagnostic_store::design_rule_code;

    /// The code a stored deck keys a rule by is the code a published finding
    /// carries. Two spellings of the same rule identity is how a deck comes to
    /// set a severity for a rule nobody can find on the Problems page.
    #[test]
    fn a_rule_is_named_the_same_way_in_the_deck_and_on_a_finding() {
        for rule in DrcViolationType::ALL {
            assert_eq!(rule.rule_code(), design_rule_code(rule));
        }
    }

    #[test]
    fn every_rule_the_checker_reports_is_on_the_page() {
        // The page walks the checker's own inventory, so this is a check that
        // the inventory is exhaustive rather than a hand-maintained subset.
        assert_eq!(DrcViolationType::ALL.len(), 28);
        for rule in DrcViolationType::ALL {
            assert!(!rule.label().is_empty());
            assert!(!rule.description().is_empty());
            assert!(!rule.suggested_fix().is_empty());
        }
    }

    #[test]
    fn the_default_option_is_marked_and_nothing_else_is() {
        let rule = DrcViolationType::DanglingWire;
        assert_eq!(rule.default_severity(), DrcSeverity::Warning);
        assert_eq!(option_label(rule, DrcSeverity::Warning), "Warning · default");
        assert_eq!(option_label(rule, DrcSeverity::Critical), "Critical");
    }

    #[test]
    fn the_columns_close_the_table_exactly() {
        for width in [1_000.0_f32, 1_337.0, 2_048.0] {
            let columns = column_geometry(width, 16.0);
            assert_eq!(columns[0].0, 16.0, "the first column starts at the inset");
            for pair in columns.windows(2) {
                assert_eq!(
                    pair[0].0 + pair[0].1,
                    pair[1].0,
                    "columns leave no gap at width {width}"
                );
            }
            let (last_left, last_width) = columns[COLUMNS - 1];
            assert!(
                (last_left + last_width - (16.0 + width)).abs() < f32::EPSILON,
                "the last column closes the table at width {width}"
            );
        }
    }

    /// Restoring the page clears the deck and nothing else.
    #[test]
    fn restoring_the_page_puts_every_rule_back_on_its_default() {
        let mut state = AppState::default();
        state
            .workspace
            .content
            .design_rules
            .set_severity(DrcViolationType::UnloadedOutput, DrcSeverity::Critical);
        assert_eq!(state.workspace.content.design_rules.len(), 1);
        restore_defaults(&mut state.workspace.content);
        assert!(state.workspace.content.design_rules.is_empty());
        assert!(state.workspace.content.project_metadata_dirty);
        assert_eq!(
            DrcViolationType::UnloadedOutput.default_severity(),
            DrcSeverity::Warning,
            "the shipped default is not what the deck changed"
        );
    }
}

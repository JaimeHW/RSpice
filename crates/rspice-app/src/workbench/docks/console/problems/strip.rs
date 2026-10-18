//! The filter strip: what the grid is narrowed to, and what each control
//! would list if the reader moved it.
//!
//! Every count printed here is the count of rows that control would list with
//! every *other* control exactly where it stands. That is the only definition
//! under which the numbers stay true as the reader works: a severity count
//! taken over the whole list would promise rows that the producer filter beside
//! it then withholds.
//!
//! A severity with nothing in it and nothing hidden earns no control. One that
//! is switched off keeps its control whatever its count, because a reader who
//! hid a rung has to be able to find it again.

use egui::{Align, Layout, Ui};

use super::super::{CONSOLE_COLUMN_GAP, CONSOLE_SELECT_CHROME};
use super::arrange::Tally;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{chip, select_mono_with_response};
use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
use crate::workbench::app_state::problems_view::{ProblemGrouping, ProblemsView};

/// What the producer control says when the grid is narrowed to none.
const EVERY_PRODUCER: &str = "All producers";

/// What the text filter says while it is empty: what it matches.
const FILTER_HINT: &str = "rule, subject, message";

/// The text filter's margin around its text: egui's own, stated, because the
/// field's width is reckoned from it.
const FILTER_MARGIN: egui::Margin = egui::Margin::symmetric(4, 2);

/// The space between two lines of the strip, when it wraps.
const LINE_GAP: f32 = 4.0;

/// Draw the strip, editing the reading position it is drawn from.
///
/// Every control is as wide as the longest thing it can say — a select its
/// longest choice, the text filter its hint — in the face it says it in, so
/// none prints a cut word and picking a choice never moves the controls
/// beside it.
pub(super) fn filter_strip(ui: &mut Ui, view: &mut ProblemsView, tally: &Tally) {
    let advance = character_width(ui);
    let select_width = |options: &[String]| {
        let widest = options
            .iter()
            .map(|option| option.chars().count())
            .max()
            .unwrap_or(0);
        advance * widest as f32 + CONSOLE_SELECT_CHROME
    };
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        ui.spacing_mut().item_spacing.y = LINE_GAP;
        for level in DiagnosticLevel::ALL {
            let count = tally.severity(level);
            let listed = view.severities.lists(level);
            if count == 0 && listed {
                continue;
            }
            if chip(ui, &format!("{} {count}", level.display_name()), listed).clicked() {
                view.severities.toggle(level);
            }
        }
        if (tally.waived > 0 || !view.waived)
            && chip(ui, &format!("Waived {}", tally.waived), view.waived).clicked()
        {
            view.waived = !view.waived;
        }

        let mut options = Vec::with_capacity(tally.producers.len() + 1);
        options.push(EVERY_PRODUCER.to_owned());
        options.extend(
            tally
                .producers
                .iter()
                .map(|(producer, count)| format!("{producer} {count}")),
        );
        // The field shows the option the reader picked, spelled the way the
        // list spells it, so the open list has something to match it against.
        let selected = view.producer.as_deref().map_or_else(
            || EVERY_PRODUCER.to_owned(),
            |producer| {
                tally
                    .producers
                    .iter()
                    .find(|(listed, _)| *listed == producer)
                    .map_or_else(
                        || producer.to_owned(),
                        |(listed, count)| format!("{listed} {count}"),
                    )
            },
        );
        let picked = select_mono_with_response(
            ui,
            "workbench.problems.producer",
            "Producer",
            &selected,
            &options,
            select_width(&options),
        )
        .picked;
        if let Some(picked) = picked {
            view.producer = picked
                .checked_sub(1)
                .and_then(|index| tally.producers.get(index))
                .map(|(producer, _)| producer.clone());
        }

        // The hint whole, inside the field's margins, with a character to
        // spare for the caret, in the face and well of the Console's find
        // field.
        let response = ui.add_sized(
            [
                advance * (FILTER_HINT.chars().count() as f32 + 1.0) + FILTER_MARGIN.sum().x,
                Tokens::get(ui.ctx()).metrics.ctl_h,
            ],
            egui::TextEdit::singleline(&mut view.text)
                .id(egui::Id::new("workbench.problems.text"))
                .hint_text(FILTER_HINT)
                .margin(FILTER_MARGIN)
                .vertical_align(egui::Align::Center)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_label("Filter findings by text");
        });

        let grouping_options: Vec<String> = ProblemGrouping::ALL
            .iter()
            .map(|grouping| grouping.label().to_owned())
            .collect();
        let picked = select_mono_with_response(
            ui,
            "workbench.problems.grouping",
            "Grouping",
            view.grouping.label(),
            &grouping_options,
            select_width(&grouping_options),
        )
        .picked;
        if let Some(picked) = picked
            && let Some(grouping) = ProblemGrouping::ALL.get(picked)
        {
            view.grouping = *grouping;
        }
    });
}

/// What the strip says about a list the filters have emptied, and the control
/// that undoes it. A page that said only "no findings" here would be claiming
/// the design is clean when the reader had hidden what is wrong with it.
///
/// It is the toolbar's last line, as far under the controls as their own
/// lines are apart.
pub(super) fn filtered_out(ui: &mut Ui, listed: usize, total: usize) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let mut cleared = false;
    ui.add_space(LINE_GAP);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = CONSOLE_COLUMN_GAP;
        ui.label(
            egui::RichText::new(format!(
                "{listed} of {total} finding{} listed",
                if total == 1 { "" } else { "s" }
            ))
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(tokens.color.text_dim),
        );
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let clear = ui.button("List everything");
            clear.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::Button,
                    ui.is_enabled(),
                    "Clear every Problems filter",
                )
            });
            cleared = clear.clicked();
        });
    });
    cleared
}

/// One character of the face the strip's controls print in, which is what
/// they are sized in rather than in pixels somebody typed.
fn character_width(ui: &Ui) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                "M".to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                egui::Color32::WHITE,
            )
            .size()
            .x
    })
    .max(1.0)
}

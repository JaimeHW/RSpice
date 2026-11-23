//! Simulate for a project whose simulation is its deck.
//!
//! A deck runs as written, so Simulate has no plan to edit for it. In the
//! plan editor's place the page is a bar, "Deck setup" and how many
//! analyses and settings the deck states, over one dense table of the
//! deck's own setup, as a simulation environment lists a setup: its
//! analysis, option, temperature and parameter statements in the order the
//! deck states them, each at its line, read-only and as the deck writes
//! them. A row opens the Netlist workspace at its line, where the deck is
//! changed. The toolbar carries Open in Netlist and Run deck; the
//! inspector, with nothing selected, the deck's facts.

use egui::{Align2, Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2};

use crate::state::{NetlistSourceIndex, OutlineEntryKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::ui::widgets::table::{Cell, Column, DataTable, TableState};
use crate::workbench::code::CodeFeature;
use crate::workbench::code::netlist_first::{self, NetlistFirstAction, deck_name};
use crate::workbench::design_system::{PANEL_INSET, WorkbenchIcon, empty_state_with_actions};
use crate::workbench::documents::netlist_document::root_deck_source_index;
use crate::workbench::framework::feature::Cx;
use crate::workbench::layout::document_bar_height;
use crate::workbench::{MessageCatalog, MessageId};

/// The line column's width, and the statement column's; the arguments take
/// what is left.
const LINE_WIDTH: f32 = 64.0;
const STATEMENT_WIDTH: f32 = 112.0;
const ARGUMENTS_LEAST_WIDTH: f32 = 240.0;

/// The kinds of statement a deck sets its simulation up with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupSection {
    Analyses,
    Options,
    Temperature,
    Parameters,
}

/// One statement of the deck's setup: its kind, the line it starts on, and
/// its text as the deck writes it, one entry per line it spans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupCard {
    pub section: SetupSection,
    pub line: usize,
    pub lines: Vec<String>,
}

impl SetupCard {
    /// The statement's first line.
    pub fn head(&self) -> &str {
        self.lines.first().map_or("", |line| line.trim())
    }

    /// The statement's keyword, as written: `.tran`, `.options`, `.param`.
    pub fn statement(&self) -> &str {
        self.head().split_whitespace().next().unwrap_or_default()
    }

    /// What follows the keyword, as written. A statement continued on
    /// further lines reads on in one row, each continuation without its `+`.
    pub fn arguments(&self) -> String {
        let head = self.head();
        let first = head
            .get(self.statement().len()..)
            .unwrap_or_default()
            .trim();
        std::iter::once(first)
            .chain(
                self.lines
                    .iter()
                    .skip(1)
                    .map(|line| line.trim().trim_start_matches('+').trim()),
            )
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// The deck's own setup statements, from the index of its first file: every
/// analysis, the option and `.temp` statements, and the parameter
/// statements written at the deck's top level (a subcircuit's parameters
/// are its own), in the order the deck states them. A statement that sets
/// several parameters is one row.
pub fn setup_cards(index: &NetlistSourceIndex) -> Vec<SetupCard> {
    let mut cards: Vec<SetupCard> = Vec::new();
    for entry in index.outline().entries() {
        let section = match entry.kind() {
            OutlineEntryKind::Analysis => SetupSection::Analyses,
            OutlineEntryKind::Option => SetupSection::Options,
            OutlineEntryKind::Parameter if entry.scope().is_none() => SetupSection::Parameters,
            OutlineEntryKind::Control if entry.keyword().eq_ignore_ascii_case(".temp") => {
                SetupSection::Temperature
            }
            _ => continue,
        };
        if cards.iter().any(|card| card.line == entry.line()) {
            continue;
        }
        let lines = (entry.line()..=entry.end_line().max(entry.line()))
            .map(|line| printable_card(index.card(line)))
            .collect();
        cards.push(SetupCard {
            section,
            line: entry.line(),
            lines,
        });
    }
    cards.sort_by_key(|card| card.line);
    cards
}

/// One line of a statement, exactly as painted: no carriage return, tabs
/// widened so the mono grid stays a grid.
fn printable_card(card: &str) -> String {
    card.strip_suffix('\r')
        .unwrap_or(card)
        .replace('\t', "    ")
}

/// The page, in the code feature's context: its bar, then the setup's
/// table, or, for a deck that states no analysis, the empty state that says
/// so.
pub(crate) fn show(cx: &mut Cx<'_, CodeFeature>, ui: &mut Ui) {
    let messages = cx.wb.messages();
    let index = root_deck_source_index(ui.ctx(), cx.model, &mut cx.state.netlist);
    let cards = setup_cards(&index);
    let viewport = ui.ctx().content_rect().size();
    page_bar(
        ui,
        &cards,
        messages,
        document_bar_height(viewport.x, viewport.y, cx.wb.coarse_pointer),
    );
    let mut action = None;
    if cards
        .iter()
        .any(|card| card.section == SetupSection::Analyses)
    {
        let columns = [
            Column::number(messages.text(MessageId::DeckSetupLine), LINE_WIDTH).unsortable(),
            Column::words(
                messages.text(MessageId::DeckSetupStatement),
                STATEMENT_WIDTH,
            )
            .unsortable(),
            Column::name(
                messages.text(MessageId::DeckSetupArguments),
                ARGUMENTS_LEAST_WIDTH,
            )
            .unsortable(),
        ];
        // The table's selection and scroll live between frames beside the
        // page's other egui state; the rows are the deck's, read each frame.
        let id = ui.id().with("workbench.simulate.deck-setup");
        let mut state = ui
            .data(|data| data.get_temp::<TableState>(id))
            .unwrap_or_default();
        let response = DataTable::new("simulate.deck-setup", &columns, cards.len())
            .state(&mut state)
            .max_height(ui.available_height())
            .show(ui, |row| {
                let card = &cards[row];
                let hint = format!(
                    "{}\n{}",
                    card.lines.join("\n"),
                    messages.format(
                        MessageId::DeckSetupOpenAtLine,
                        &[("line", &card.line.to_string())]
                    )
                );
                vec![
                    Cell::number(card.line.to_string()),
                    Cell::number(card.statement()),
                    Cell::number(card.arguments()).hint(hint),
                ]
            });
        ui.data_mut(|data| data.insert_temp(id, state));
        if let Some(card) = response
            .clicked
            .or(response.activate)
            .and_then(|row| cards.get(row))
        {
            action = Some(NetlistFirstAction::OpenAtLine(card.line));
        }
    } else {
        let name = deck_name(cx.model, &cx.state.netlist, messages);
        let open = messages.text(MessageId::NetlistFirstOpenInNetlist);
        empty_state_with_actions(
            ui,
            WorkbenchIcon::Simulate,
            &messages.text(MessageId::DeckSetupNoAnalysisTitle),
            &messages.format(MessageId::DeckSetupNoAnalysis, &[("name", &name)]),
            |ui| {
                if Button::new(&open).accent().show(ui).clicked() {
                    action = Some(NetlistFirstAction::OpenNetlistWorkspace);
                }
            },
        );
    }
    if let Some(action) = action {
        netlist_first::carry_out(cx, action);
    }
}

/// The page's bar, where other pages stand their document bar and as tall
/// as it, so the bands across the top end on one line: the page's name on
/// the left, and on the right, neutral, how many analyses and settings the
/// table lists.
fn page_bar(ui: &mut Ui, cards: &[SetupCard], messages: MessageCatalog, height: f32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    let title = messages.text(MessageId::DeckSetupTitle);
    let summary = setup_summary(cards, messages);
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            format!("{title}, {summary}"),
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(title.as_str());
        node.set_description(summary.as_str());
        node.set_level(2);
    });
    ui.painter().text(
        pos2(rect.left() + PANEL_INSET, rect.center().y),
        Align2::LEFT_CENTER,
        &title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    ui.painter().text(
        pos2(rect.right() - PANEL_INSET, rect.center().y),
        Align2::RIGHT_CENTER,
        &summary,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

/// "2 analyses · 3 settings": the setup's analyses, and its option,
/// temperature and parameter statements.
pub fn setup_summary(cards: &[SetupCard], messages: MessageCatalog) -> String {
    let analyses = cards
        .iter()
        .filter(|card| card.section == SetupSection::Analyses)
        .count();
    let settings = cards.len() - analyses;
    let counted = |count: usize, one: MessageId, many: MessageId| {
        messages.format(
            if count == 1 { one } else { many },
            &[("count", &count.to_string())],
        )
    };
    messages.format(
        MessageId::DeckSetupSummary,
        &[
            (
                "analyses",
                &counted(
                    analyses,
                    MessageId::DeckSetupAnalysisCountSingular,
                    MessageId::DeckSetupAnalysisCount,
                ),
            ),
            (
                "settings",
                &counted(
                    settings,
                    MessageId::DeckSetupSettingCountSingular,
                    MessageId::DeckSetupSettingCount,
                ),
            ),
        ],
    )
}

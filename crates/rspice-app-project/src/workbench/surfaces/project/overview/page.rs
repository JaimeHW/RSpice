//! The Overview's page: a band of facts over the latest run's outputs.
//!
//! The band runs the page's width under the tab strip, its facts in equal
//! columns, as many as the page has room for, read down each column. Every
//! fact is one row, and every column sets its values on one edge, the one
//! the widest label on the page leaves. One rule closes the band. The
//! outputs table fills the rest of the page, flush on all four sides; its
//! head row is its title. Before any run the kit's empty state stands in
//! the table's place and offers the plan's Run; after a run that measured
//! nothing it says where outputs are added and offers Verify.
//!
//! A linked value takes its action on a click or Enter; a row takes its
//! action on Enter or a double-click, and its context menu offers the same
//! action with the same enablement.

use egui::{Align, Layout, Rect, Sense, Stroke, Ui, UiBuilder, Vec2, pos2, vec2};

use crate::ui::panels::{
    FACT_MARK_ROOM, Fact, WorkbenchIcon, elide_text, empty_state_with_actions, fact_label_width,
    fact_rows_on, property_row_height,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use crate::ui::widgets::table::{
    Align as Side, Cell, Column, DataTable, TABLE_EDGE_INSET, TableResponse, TableState,
    Tone as Ink,
};

use super::snapshot::{Action, Outputs, OverviewSnapshot, SpecificationLine, SummaryFact};
use super::{OverviewIntent, Tone};

/// The least width a column of the band is given; the band sets as many
/// columns as the page has room for, up to [`BAND_COLUMNS_MOST`].
const BAND_COLUMN: f32 = 400.0;
const BAND_COLUMNS_MOST: usize = 4;
/// Room above and below the band's rows, so they sit clear of the rules.
const BAND_PAD: f32 = 4.0;
/// The least width of a column of the outputs table. Every column has the
/// same least and the same share of what is over, so the columns stand
/// equal across the page, none stretched alone.
const COLUMN_LEAST: f32 = 96.0;

/// The page, and what the reader asked of it.
pub(super) fn page(
    ui: &mut Ui,
    snapshot: &OverviewSnapshot,
    table: &mut TableState,
) -> Option<OverviewIntent> {
    let page = ui.available_rect_before_wrap();
    let mut asked = Asked::default();
    let band = band(ui, page, &snapshot.facts, &mut asked);
    let body = Rect::from_min_max(pos2(page.left(), band.bottom()), page.max);
    let mut child = ui.new_child(
        UiBuilder::new()
            .id_salt("workbench.project.overview.outputs")
            .max_rect(body)
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(body.intersect(ui.clip_rect()));
    child.spacing_mut().item_spacing = Vec2::ZERO;
    if let Some(text) = &snapshot.note {
        note(&mut child, text);
    }
    match &snapshot.outputs {
        Outputs::Judged { spread, rows } => judged(&mut child, *spread, rows, table, &mut asked),
        Outputs::Measured(rows) => {
            let columns = [words("Output"), numbers("Value")];
            let response = DataTable::new(TABLE, &columns, rows.len())
                .state(table)
                .max_height(child.available_height().max(1.0))
                .show(&mut child, |row| {
                    let line = &rows[row];
                    vec![Cell::text(&line.output), Cell::number(&line.value)]
                });
            asked.rows(&response, |row| {
                rows.get(row).map(|line| line.action.clone())
            });
        }
        Outputs::NoRun(does, run) => empty(
            &mut child,
            (WorkbenchIcon::Run, "No runs yet"),
            does,
            run,
            &mut asked,
        ),
        Outputs::Unmeasured(verify) => empty(
            &mut child,
            (WorkbenchIcon::Verify, "Nothing measured"),
            "The latest run measured nothing. Outputs are added on Verify's Specifications page.",
            verify,
            &mut asked,
        ),
    }
    ui.allocate_rect(page, Sense::hover());
    let from_menu = row_menu(ui, asked.menu.take());
    asked.intent.or(from_menu)
}

const TABLE: &str = "workbench.project.overview.outputs";

/// The band of `facts` across the top of `page`, closed by its rule; the
/// rect it stands in.
fn band(ui: &mut Ui, page: Rect, facts: &[SummaryFact], asked: &mut Asked) -> Rect {
    let fit = (page.width() / BAND_COLUMN).floor() as usize;
    let columns = fit.clamp(1, BAND_COLUMNS_MOST).min(facts.len().max(1));
    let rows = facts.len().div_ceil(columns).max(1);
    let height = rows as f32 * property_row_height(ui) + 2.0 * BAND_PAD;
    let band = Rect::from_min_size(page.min, vec2(page.width(), height));
    // One label column for the whole band, so every value starts on one
    // edge in its column, marked or not, whatever the project's state.
    let label_width = fact_label_width(ui, facts.iter().map(|fact| fact.label)) + FACT_MARK_ROOM;
    let width = (page.width() / columns as f32).floor();
    for (index, column) in facts.chunks(rows).enumerate() {
        let left = page.left() + index as f32 * width;
        // The last column takes what the floor left over.
        let right = if index + 1 == columns {
            page.right()
        } else {
            left + width
        };
        let rect = Rect::from_min_max(
            pos2(left, band.top() + BAND_PAD),
            pos2(right, band.bottom() - BAND_PAD),
        );
        let mut child = ui.new_child(
            UiBuilder::new()
                .id_salt(("workbench.project.overview.band", index))
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        child.spacing_mut().item_spacing = Vec2::ZERO;
        let shown = column
            .iter()
            .map(|fact| {
                let link = fact.link.as_ref().filter(|link| link.enabled);
                Fact::new(fact.label, &fact.value)
                    .marked(fact.tone.problem())
                    .linked(link.map(|link| link.label))
            })
            .collect::<Vec<_>>();
        if let Some(followed) = fact_rows_on(&mut child, &shown, label_width)
            && let Some(link) = &column[followed].link
        {
            asked.take(link);
        }
    }
    let t = Tokens::get(ui.ctx());
    ui.painter().hline(
        page.x_range(),
        band.bottom() - 0.5,
        Stroke::new(1.0, t.color.border),
    );
    band
}

/// Verify's verdicts, one row a specification: a swept run's spread, its
/// least and greatest values and where the worst was, headed by `spread`;
/// else a run's one value.
fn judged(
    ui: &mut Ui,
    spread: Option<&str>,
    rows: &[SpecificationLine],
    table: &mut TableState,
    asked: &mut Asked,
) {
    let columns = match spread {
        Some(worst) => vec![
            words("Output"),
            numbers("Spec"),
            numbers("Min"),
            numbers("Max"),
            words(worst),
            numbers("Margin"),
            words("Verdict"),
        ],
        None => vec![
            words("Output"),
            numbers("Value"),
            numbers("Spec"),
            numbers("Margin"),
            words("Verdict"),
        ],
    };
    let response = DataTable::new(TABLE, &columns, rows.len())
        .state(table)
        .max_height(ui.available_height().max(1.0))
        .show(ui, |row| {
            let line = &rows[row];
            let output = Cell::text(&line.output);
            let output = match &line.measurement {
                Some(measurement) => output.hint(format!("Measurement {measurement}")),
                None => output,
            };
            let number =
                |text: &Option<String>| text.as_ref().map_or_else(Cell::absent, Cell::number);
            let verdict = line.verdict.map_or_else(Cell::absent, |(word, tone)| {
                let cell = Cell::text(word);
                match tone {
                    Tone::Error => cell.toned(Ink::Error),
                    Tone::Warn => cell.toned(Ink::Caution),
                    Tone::Neutral => cell,
                }
            });
            match spread {
                Some(_) => vec![
                    output,
                    number(&line.limit),
                    number(&line.min),
                    number(&line.max),
                    line.worst_at.as_ref().map_or_else(Cell::absent, Cell::text),
                    number(&line.margin),
                    verdict,
                ],
                None => vec![
                    output,
                    number(&line.value),
                    number(&line.limit),
                    number(&line.margin),
                    verdict,
                ],
            }
        });
    asked.rows(&response, |row| {
        rows.get(row).map(|line| line.action.clone())
    });
}

/// A column of words, as wide as every other column of the table.
fn words(head: &str) -> Column {
    Column::name(head, COLUMN_LEAST).unsortable()
}

/// A column of numbers, right-aligned, as wide as every other column.
fn numbers(head: &str) -> Column {
    Column::name(head, COLUMN_LEAST)
        .align(Side::Right)
        .unsortable()
}

/// What the reader asked of the page this frame: an action to take now,
/// and a row whose menu to open.
#[derive(Default)]
struct Asked {
    intent: Option<OverviewIntent>,
    menu: Option<(Action, egui::Pos2)>,
}

impl Asked {
    /// Take `action` when it can be taken.
    fn take(&mut self, action: &Action) {
        if action.enabled && self.intent.is_none() {
            self.intent = Some(action.intent.clone());
        }
    }

    /// What a table's rows asked: a row's action by Enter or a
    /// double-click, or its menu.
    fn rows(&mut self, response: &TableResponse, action_of: impl Fn(usize) -> Option<Action>) {
        if let Some(action) = response.activate.and_then(&action_of) {
            self.take(&action);
        }
        if let Some(request) = response.context_menu
            && let Some(action) = action_of(request.cell.row)
        {
            self.menu = Some((action, request.position));
        }
    }
}

/// The kit's empty state in the table's place: `title` under `icon`, what
/// fills the table, and `action`'s button, faint with its reason while it
/// cannot be taken.
fn empty(
    ui: &mut Ui,
    (icon, title): (WorkbenchIcon, &str),
    description: &str,
    action: &Action,
    asked: &mut Asked,
) {
    let mut pressed = false;
    empty_state_with_actions(ui, icon, title, description, |ui| {
        let response = Button::new(action.label).enabled(action.enabled).show(ui);
        let response = match action.disabled_reason.filter(|_| !action.enabled) {
            Some(reason) => response.on_disabled_hover_text(reason),
            None => response,
        };
        pressed |= response.clicked();
    });
    if pressed {
        asked.take(action);
    }
}

/// A neutral note over the table: what the reader has to know before
/// reading the verdicts.
fn note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, property_row_height(ui)), Sense::hover());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let words = elide_text(ui, text, &font, (width - 2.0 * TABLE_EDGE_INSET).max(0.0));
    ui.painter().text(
        pos2(rect.left() + TABLE_EDGE_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        words,
        font,
        t.color.text_dim,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    response.on_hover_text(text);
}

/// The row menu's popup.
fn menu_id(ui: &Ui) -> egui::Id {
    ui.id().with("workbench.project.overview.row menu")
}

/// A row's context menu, opened where the reader asked for it and kept
/// open until its item is taken or the reader clicks elsewhere: the row's
/// one action, shown disabled with its reason while it cannot be taken.
/// The action taken from it.
fn row_menu(ui: &Ui, request: Option<(Action, egui::Pos2)>) -> Option<OverviewIntent> {
    let id = menu_id(ui);
    let at = id.with("at");
    let opened = request.is_some();
    if let Some(request) = request {
        ui.ctx().data_mut(|data| data.insert_temp(at, request));
    }
    let (action, position) = ui
        .ctx()
        .data(|data| data.get_temp::<(Action, egui::Pos2)>(at))?;
    if !opened && !egui::Popup::is_id_open(ui.ctx(), id) {
        return None;
    }
    let mut taken = None;
    egui::Popup::new(
        id,
        ui.ctx().clone(),
        egui::PopupAnchor::Position(position),
        egui::LayerId::new(egui::Order::Foreground, id),
    )
    .kind(egui::PopupKind::Menu)
    .layout(Layout::top_down_justified(Align::Min))
    .style(egui::containers::menu::menu_style)
    .open_memory(opened.then_some(egui::SetOpenCommand::Bool(true)))
    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
    .show(|ui| {
        ui.set_min_width(160.0);
        let item = ui.add_enabled(action.enabled, egui::Button::new(action.label));
        let item = match action.disabled_reason.filter(|_| !action.enabled) {
            Some(reason) => item.on_disabled_hover_text(reason),
            None => item,
        };
        if item.clicked() {
            taken = Some(action.intent.clone());
            ui.close();
        }
    });
    taken
}

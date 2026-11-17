//! Netlist-first design surface.
//!
//! A deck-driven project has no schematic to draw, so the Design surface
//! states what owns the project — the SPICE deck — and routes to the two
//! things that can change that: the Netlist workspace, and schematic
//! promotion. The page shares the no-project landing's geometry and action
//! idiom so the workbench's landing surfaces read as one product, and every
//! fact on it is live document state: the deck preview, the outline counts,
//! and the publication state all come from the project's own deck, indexed the
//! way the Netlist workspace indexes it. It is the code feature's surface,
//! drawn in that feature's context while the deck owns the project.

use egui::{
    Align, Align2, Color32, Key, Layout, Modifiers, Rect, Response, ScrollArea, Sense, Stroke, Ui,
    UiBuilder, Vec2, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::state::command_catalog::CommandKey;
use crate::state::{DependencyResolution, NetlistSourceIndex, OutlineEntryKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::Model;
use crate::workbench::code::CodeFeature;
use crate::workbench::design_system::{
    Problem, WorkbenchIcon, action_row, centered_content_rect, elide_text, paint_problem_mark,
    property_card, property_row, property_row_problem,
};
use crate::workbench::documents::netlist_document::{
    NetlistDocumentState, OwnedNetlistPublicationState, owned_netlist_publication_state_in,
    root_deck_source_index,
};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::framework::feature::{Cx, Execute};
use crate::workbench::state::WorkbenchSession;

/// Geometry shared with the no-project landing, so the two landing surfaces
/// sit on the same grid.
const HEADER_MIN_HEIGHT: f32 = 116.0;
const CONTENT_MAX_WIDTH: f32 = 1240.0;
const DESKTOP_GUTTER: f32 = 30.0;
const BODY_TOP: f32 = 14.0;
const BODY_BOTTOM: f32 = 26.0;
const RAIL_WIDTH: f32 = 322.0;
const COLUMN_GAP: f32 = 18.0;
const STACK_BREAKPOINT: f32 = 900.0;

/// Side of the publication state's problem mark and the air after it: the
/// kit's own, so it matches every other marked line in the product.
use crate::ui::widgets::{
    PROBLEM_MARK_GAP as PUBLICATION_MARK_GAP, PROBLEM_MARK_SIDE as PUBLICATION_MARK_SIDE,
};
const PREVIEW_TOOLBAR_HEIGHT: f32 = 34.0;
const PREVIEW_LINE_HEIGHT: f32 = 17.0;
const PREVIEW_GUTTER_WIDTH: f32 = 44.0;
const PREVIEW_TOP_PAD: f32 = 7.0;
const PREVIEW_FOOTER_HEIGHT: f32 = 24.0;
const PREVIEW_MIN_HEIGHT: f32 = 180.0;
const STACKED_PREVIEW_HEIGHT: f32 = 320.0;

pub const EYEBROW: &str = "NETLIST-FIRST PROJECT \u{00b7} NO SCHEMATIC";
const DESCRIPTION: &str = "This project is driven by its SPICE deck, edited in the Netlist \
    workspace; simulation, probing, and results work exactly as in schematic projects. \
    Creating a schematic promotes this into a schematic-driven design.";
const PREVIEW_ACTION_LABEL: &str = "Open the deck in the Netlist workspace";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetlistFirstAction {
    OpenNetlistWorkspace,
    CreateSchematic,
}

impl NetlistFirstAction {
    /// The command the action runs.
    fn command_key(self) -> CommandKey {
        match self {
            Self::OpenNetlistWorkspace => ids::NETLIST_WORKSPACE,
            Self::CreateSchematic => ids::NEW_CELL,
        }
    }
}

/// Run the action's command. The shell runs it once the surface has drawn,
/// and the surface draws nothing after the action.
pub fn raise_netlist_first_action(wb: &mut WorkbenchSession, action: NetlistFirstAction) {
    wb.outbox.push(Execute(action.command_key()));
}

/// The surface the code feature registers: the landing, in its context.
pub(crate) fn show(cx: &mut Cx<'_, CodeFeature>, ui: &mut Ui) {
    let mut action = None;
    // The row dispatches this exact command, so it reads the command's own
    // label rather than keeping a second copy of the workspace name.
    let open_label = cx.commands.label(ids::NETLIST_WORKSPACE);
    egui::Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_app)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            header(ui, cx.model.workspace.content.project.name());
            body(ui, cx.model, &mut cx.state.netlist, open_label, &mut action);
        });
    if let Some(action) = action {
        raise_netlist_first_action(cx.wb, action);
    }
}

fn header(ui: &mut Ui, project_name: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let content_width = centered_content_rect(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(width, 1.0)),
        DESKTOP_GUTTER,
        CONTENT_MAX_WIDTH,
    )
    .width();
    // The description is the one line that wraps, so the band takes the height
    // it needs on narrow shells instead of clipping its own copy.
    let description_width = content_width.min(720.0);
    let description_height = ui
        .painter()
        .layout(
            DESCRIPTION.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            Color32::PLACEHOLDER,
            description_width,
        )
        .size()
        .y;
    let height = HEADER_MIN_HEIGHT.max(26.0 + 14.0 + 5.0 + 27.0 + 3.0 + description_height + 14.0);

    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );

    let content = centered_content_rect(rect, DESKTOP_GUTTER, CONTENT_MAX_WIDTH);
    let mut header = ui.new_child(
        UiBuilder::new()
            .max_rect(content)
            .layout(Layout::top_down(Align::Min)),
    );
    header.add_space(26.0);
    header.label(
        egui::RichText::new(EYEBROW)
            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
            .color(t.color.text_faint)
            .extra_letter_spacing(0.09 * tokens::FS_0),
    );
    header.add_space(5.0);
    header.add(
        egui::Label::new(
            egui::RichText::new(project_name)
                .font(theme::sans(20.0, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .truncate(),
    );
    header.add_space(3.0);
    header.scope(|ui| {
        ui.set_max_width(description_width);
        ui.add(
            egui::Label::new(
                egui::RichText::new(DESCRIPTION)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim),
            )
            .wrap(),
        );
    });
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Label,
            ui.is_enabled(),
            format!("Netlist-first project, no schematic. {project_name}"),
        )
    });
}

fn body(
    ui: &mut Ui,
    model: &Model,
    netlist: &mut NetlistDocumentState,
    open_label: &str,
    action: &mut Option<NetlistFirstAction>,
) {
    let index = root_deck_source_index(model, netlist);
    let facts = deck_facts(model, netlist, &index);

    let available = ui.available_rect_before_wrap();
    ui.allocate_rect(available, Sense::hover());
    let mut content = centered_content_rect(available, DESKTOP_GUTTER, CONTENT_MAX_WIDTH);
    content.min.y = (content.top() + BODY_TOP).min(content.bottom());
    content.max.y = (content.bottom() - BODY_BOTTOM).max(content.top());

    if content.width() >= STACK_BREAKPOINT {
        let rail_width = RAIL_WIDTH.min((content.width() * 0.40).max(1.0));
        let preview_width = (content.width() - rail_width - COLUMN_GAP).max(1.0);
        let left_rect = Rect::from_min_size(content.min, vec2(preview_width, content.height()));
        let right_rect = Rect::from_min_max(
            pos2(left_rect.right() + COLUMN_GAP, content.top()),
            content.right_bottom(),
        );

        let mut left = ui.new_child(
            UiBuilder::new()
                .max_rect(left_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        left.set_clip_rect(left_rect);
        deck_preview(&mut left, &index, &facts, left_rect.height(), action);

        let mut right = ui.new_child(
            UiBuilder::new()
                .max_rect(right_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        right.set_clip_rect(right_rect);
        ScrollArea::vertical()
            .id_salt("workbench.design.netlist-first.rail")
            .auto_shrink([false, false])
            .show(&mut right, |ui| rail(ui, &facts, open_label, action));
    } else {
        let mut stacked = ui.new_child(
            UiBuilder::new()
                .max_rect(content)
                .layout(Layout::top_down(Align::Min)),
        );
        stacked.set_clip_rect(content);
        ScrollArea::vertical()
            .id_salt("workbench.design.netlist-first.stacked")
            .auto_shrink([false, false])
            .show(&mut stacked, |ui| {
                ui.set_min_width(content.width());
                deck_preview(ui, &index, &facts, STACKED_PREVIEW_HEIGHT, action);
                ui.add_space(16.0);
                rail(ui, &facts, open_label, action);
            });
    }
}

fn rail(ui: &mut Ui, facts: &DeckFacts, open_label: &str, action: &mut Option<NetlistFirstAction>) {
    for (title, detail, icon, primary, next) in [
        (
            open_label,
            "Deck source \u{00b7} outline \u{00b7} diagnostics \u{00b7} overlay",
            WorkbenchIcon::Code,
            true,
            NetlistFirstAction::OpenNetlistWorkspace,
        ),
        (
            "Create schematic\u{2026}",
            "Promote to a schematic-driven project",
            WorkbenchIcon::Design,
            false,
            NetlistFirstAction::CreateSchematic,
        ),
    ] {
        if action_row(ui, title, detail, icon, primary).clicked() {
            *action = Some(next);
        }
        ui.add_space(5.0);
    }
    ui.add_space(9.0);
    deck_summary_card(ui, facts);
}

fn deck_summary_card(ui: &mut Ui, facts: &DeckFacts) {
    property_card(ui, "Deck summary", |ui| {
        property_row(ui, "Source", &facts.name);
        if let Some(dialect) = facts.dialect {
            property_row(ui, "Dialect", dialect);
        }
        property_row(ui, "Lines", &facts.lines.to_string());
        property_row(ui, "Devices", &facts.devices.to_string());
        property_row(ui, "Analyses", &facts.analyses.to_string());
        property_row(ui, "Models", &facts.models.to_string());
        property_row(ui, "Subcircuits", &facts.subcircuits.to_string());
        if facts.unresolved_dependencies > 0 {
            property_row_problem(
                ui,
                "Dependencies",
                &format!(
                    "{} \u{00b7} {} unresolved",
                    facts.dependencies, facts.unresolved_dependencies
                ),
                Some(Problem::Warning),
            );
        } else {
            property_row(ui, "Dependencies", &facts.dependencies.to_string());
        }
    });
}

/// The read-only deck panel. The whole panel is one control that opens the
/// Netlist workspace — the deck is the document this project continues
/// from, so clicking it behaves like opening any other document.
///
/// The panel takes the height its deck needs up to `max_height`: a short deck
/// ends where its `.end` card does instead of trailing an empty slab, and a
/// long one truncates behind the footer that counts what was left out.
fn deck_preview(
    ui: &mut Ui,
    index: &NetlistSourceIndex,
    facts: &DeckFacts,
    max_height: f32,
    action: &mut Option<NetlistFirstAction>,
) {
    let t = Tokens::get(ui.ctx());
    let needed = PREVIEW_TOOLBAR_HEIGHT
        + PREVIEW_TOP_PAD
        + index.line_count() as f32 * PREVIEW_LINE_HEIGHT
        + 11.0;
    let height = needed.clamp(PREVIEW_MIN_HEIGHT, max_height.max(PREVIEW_MIN_HEIGHT));
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let hovered = response.hovered();
    ui.painter().rect(
        rect,
        t.radius,
        t.color.bg_panel,
        Stroke::new(
            1.0,
            if hovered {
                t.color.accent
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );

    let toolbar = Rect::from_min_size(
        rect.min + vec2(1.0, 1.0),
        vec2(rect.width() - 2.0, PREVIEW_TOOLBAR_HEIGHT),
    );
    let radius = t.radius as u8;
    ui.painter().rect_filled(
        toolbar,
        egui::CornerRadius {
            nw: radius,
            ne: radius,
            sw: 0,
            se: 0,
        },
        t.color.bg_panel_2,
    );
    ui.painter().hline(
        toolbar.x_range(),
        toolbar.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    preview_toolbar_text(ui, toolbar, facts, &t);

    let body = Rect::from_min_max(
        pos2(rect.left() + 1.0, toolbar.bottom() + 1.0),
        rect.max - vec2(1.0, 1.0),
    );
    if hovered {
        ui.painter().rect_filled(
            body,
            egui::CornerRadius {
                nw: 0,
                ne: 0,
                sw: radius,
                se: radius,
            },
            t.color.bg_hover,
        );
    }
    preview_rows(ui, body, index, &t);

    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), PREVIEW_ACTION_LABEL)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_description(format!("{} \u{00b7} {} lines", facts.name, facts.lines));
    });
    theme::paint_focus_ring(ui, &response, rect);
    if activated(ui, &response) {
        *action = Some(NetlistFirstAction::OpenNetlistWorkspace);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand);
}

/// Whether the preview was clicked or activated from the keyboard. The key is
/// consumed here so nothing behind the page also acts on the same press.
fn activated(ui: &Ui, response: &Response) -> bool {
    if response.clicked() {
        return true;
    }
    response.has_focus()
        && ui.input_mut(|input| {
            input.consume_key(Modifiers::NONE, Key::Enter)
                || input.consume_key(Modifiers::NONE, Key::Space)
        })
}

fn preview_toolbar_text(ui: &Ui, toolbar: Rect, facts: &DeckFacts, t: &Tokens) {
    let title_rect = ui.painter().text(
        pos2(toolbar.left() + 11.0, toolbar.center().y),
        Align2::LEFT_CENTER,
        "Deck source",
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
    let state_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let state_left = facts
        .publication
        .and_then(|publication| {
            let (label, problem) = publication_presentation(publication);
            let label = label?;
            let mut left = ui
                .painter()
                .text(
                    pos2(toolbar.right() - 11.0, toolbar.center().y),
                    Align2::RIGHT_CENTER,
                    label,
                    state_font.clone(),
                    t.color.text_dim,
                )
                .left();
            if let Some(problem) = problem {
                left -= PUBLICATION_MARK_GAP;
                paint_problem_mark(
                    ui.painter(),
                    Rect::from_center_size(
                        pos2(left - PUBLICATION_MARK_SIDE / 2.0, toolbar.center().y),
                        vec2(PUBLICATION_MARK_SIDE, PUBLICATION_MARK_SIDE),
                    ),
                    problem,
                );
                left -= PUBLICATION_MARK_SIDE;
            }
            Some(left)
        })
        .unwrap_or(toolbar.right() - 11.0);
    let name_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let name_left = title_rect.right() + 9.0;
    let name_width = (state_left - 9.0 - name_left).max(0.0);
    if name_width > 12.0 {
        ui.painter().text(
            pos2(name_left, toolbar.center().y),
            Align2::LEFT_CENTER,
            elide_text(ui, &facts.name, &name_font, name_width),
            name_font,
            t.color.text_dim,
        );
    }
}

fn preview_rows(ui: &Ui, body: Rect, index: &NetlistSourceIndex, t: &Tokens) {
    let painter = ui.painter().with_clip_rect(body);
    let line_count = index.line_count();
    if line_count == 0 {
        painter.text(
            body.center(),
            Align2::CENTER_CENTER,
            "The deck has no source text.",
            theme::sans(tokens::FS_1, FontWeight::Regular),
            t.color.text_dim,
        );
        return;
    }

    let capacity =
        (((body.height() - PREVIEW_TOP_PAD) / PREVIEW_LINE_HEIGHT).floor()).max(0.0) as usize;
    let (shown, truncated) = if line_count <= capacity {
        (line_count, false)
    } else {
        let with_footer = ((body.height() - PREVIEW_TOP_PAD - PREVIEW_FOOTER_HEIGHT)
            / PREVIEW_LINE_HEIGHT)
            .floor()
            .max(0.0) as usize;
        (with_footer.min(line_count), true)
    };

    let gutter_right = body.left() + PREVIEW_GUTTER_WIDTH;
    let rows_bottom = body.top() + PREVIEW_TOP_PAD + shown as f32 * PREVIEW_LINE_HEIGHT;
    if shown > 0 {
        painter.vline(
            gutter_right,
            egui::Rangef::new(body.top(), rows_bottom.min(body.bottom())),
            Stroke::new(1.0, t.color.border),
        );
    }
    let number_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let card_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let card_painter = painter.with_clip_rect(Rect::from_min_max(
        body.min,
        pos2(body.right() - 9.0, body.max.y),
    ));
    for line in 1..=shown {
        let center_y = body.top() + PREVIEW_TOP_PAD + (line as f32 - 0.5) * PREVIEW_LINE_HEIGHT;
        painter.text(
            pos2(gutter_right - 7.0, center_y),
            Align2::RIGHT_CENTER,
            line.to_string(),
            number_font.clone(),
            t.color.text_faint,
        );
        let card = printable_card(index.card(line));
        card_painter.text(
            pos2(gutter_right + 10.0, center_y),
            Align2::LEFT_CENTER,
            card.as_ref(),
            card_font.clone(),
            card_tone(t, line, &card),
        );
    }

    if truncated {
        let footer = Rect::from_min_max(
            pos2(body.left(), body.bottom() - PREVIEW_FOOTER_HEIGHT),
            body.max,
        );
        painter.hline(
            footer.x_range(),
            footer.top(),
            Stroke::new(1.0, t.color.border),
        );
        painter.text(
            pos2(footer.left() + 11.0, footer.center().y),
            Align2::LEFT_CENTER,
            format!(
                "\u{2026} {} more lines \u{00b7} open in the Netlist workspace",
                line_count - shown
            ),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
}

/// One card, exactly as painted: no carriage return, tabs widened so the mono
/// grid stays a grid.
fn printable_card(card: &str) -> std::borrow::Cow<'_, str> {
    let card = card.strip_suffix('\r').unwrap_or(card);
    if card.contains('\t') {
        std::borrow::Cow::Owned(card.replace('\t', "    "))
    } else {
        std::borrow::Cow::Borrowed(card)
    }
}

/// The preview's three-tone reading of a card: comments recede, the title and
/// dot directives carry full weight, device cards sit between.
fn card_tone(t: &Tokens, line: usize, card: &str) -> Color32 {
    let trimmed = card.trim_start();
    if trimmed.starts_with('*') || trimmed.starts_with(';') {
        t.color.text_faint
    } else if line == 1 || trimmed.starts_with('.') {
        t.color.text
    } else {
        t.color.text_dim
    }
}

/// What the deck toolbar says about the owned source's publication, and
/// whether it is worth a mark.
///
/// Validation and synchronization are states the deck is in, not verdicts on
/// it, so they are stated in the ordinary tone. A change made to the file
/// outside this project is the one the reader has to act on.
const fn publication_presentation(
    publication: OwnedNetlistPublicationState,
) -> (Option<&'static str>, Option<Problem>) {
    (
        publication.label(),
        if publication.external_change_pending {
            Some(Problem::Warning)
        } else {
            None
        },
    )
}

/// Everything the page states about the deck, derived once per frame from the
/// canonical document and the editor's own cached source index.
pub struct DeckFacts {
    pub name: String,
    dialect: Option<&'static str>,
    pub lines: usize,
    pub devices: usize,
    pub analyses: usize,
    pub models: usize,
    pub subcircuits: usize,
    pub dependencies: usize,
    unresolved_dependencies: usize,
    pub publication: Option<OwnedNetlistPublicationState>,
}

pub fn deck_facts(
    model: &Model,
    netlist: &NetlistDocumentState,
    index: &NetlistSourceIndex,
) -> DeckFacts {
    let document = model.workspace.content.netlist_document.as_ref();
    let name = model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .map(|descriptor| descriptor.artifact_name.clone())
        .or_else(|| {
            document.and_then(|document| {
                document
                    .provenance()
                    .imported()
                    .map(|imported| imported.origin().display_name().to_owned())
            })
        })
        .unwrap_or_else(|| "Untitled deck".to_owned());
    let dialect = model
        .workspace
        .content
        .netlist_descriptor
        .as_ref()
        .and_then(|descriptor| descriptor.imported_dialect)
        .map(crate::state::NetlistSourceDialect::label);

    let (mut devices, mut analyses, mut models, mut subcircuits) = (0, 0, 0, 0);
    for entry in index.outline().entries() {
        match entry.kind() {
            OutlineEntryKind::Device => devices += 1,
            OutlineEntryKind::Analysis => analyses += 1,
            OutlineEntryKind::Model => models += 1,
            OutlineEntryKind::Subcircuit => subcircuits += 1,
            _ => {}
        }
    }

    let (dependencies, unresolved_dependencies) = document.map_or((0, 0), |document| {
        let unresolved = document
            .dependencies()
            .iter()
            .filter(|dependency| {
                !matches!(
                    dependency.resolution(),
                    DependencyResolution::Resolved { .. }
                )
            })
            .count();
        (document.dependencies().len(), unresolved)
    });
    let publication = document.map(|document| {
        owned_netlist_publication_state_in(model, netlist, document.content_digest())
    });

    DeckFacts {
        name,
        dialect,
        lines: index.line_count(),
        devices,
        analyses,
        models,
        subcircuits,
        dependencies,
        unresolved_dependencies,
        publication,
    }
}

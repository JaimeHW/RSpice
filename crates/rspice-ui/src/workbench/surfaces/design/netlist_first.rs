//! Netlist-first design surface.
//!
//! A deck-driven project has no schematic to draw, so the Design surface
//! states what owns the project — the SPICE deck — and routes to the two
//! things that can change that: the Netlist & Script Editor, and schematic
//! promotion. The page shares the no-project landing's geometry and action
//! idiom so the workbench's landing surfaces read as one product, and every
//! fact on it is live document state: the deck preview, the outline counts,
//! and the publication state all come from the same source index and
//! projection the Netlist workspace itself renders.

use egui::{
    Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, UiBuilder, Vec2,
    WidgetInfo, WidgetType, pos2, vec2,
};

use crate::state::{DependencyResolution, NetlistSourceIndex, OutlineEntryKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{
    WorkbenchIcon, action_row, centered_content_rect, elide_text, property_card, property_row,
    property_row_toned,
};
use crate::workbench::documents::netlist_document::{
    OwnedNetlistPublicationState, owned_netlist_publication_state, visible_source_index,
};
use crate::workbench::state::Workspace;
use crate::workbench::{AppState, RSpiceApp};

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

const PREVIEW_TOOLBAR_HEIGHT: f32 = 34.0;
const PREVIEW_LINE_HEIGHT: f32 = 17.0;
const PREVIEW_GUTTER_WIDTH: f32 = 44.0;
const PREVIEW_TOP_PAD: f32 = 7.0;
const PREVIEW_FOOTER_HEIGHT: f32 = 24.0;
const PREVIEW_MIN_HEIGHT: f32 = 180.0;
const STACKED_PREVIEW_HEIGHT: f32 = 320.0;

const EYEBROW: &str = "NETLIST-FIRST PROJECT \u{00b7} NO SCHEMATIC";
const DESCRIPTION: &str = "This project is driven by its SPICE deck. The Netlist & Script Editor \
    owns source editing; simulation, probing, and results work exactly as in schematic projects. \
    Creating a schematic promotes this into a schematic-driven design.";
const PREVIEW_ACTION_LABEL: &str = "Open the deck in the Netlist & Script Editor";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NetlistFirstAction {
    OpenNetlistWorkspace,
    CreateSchematic,
}

fn execute_netlist_first_action(app: &mut RSpiceApp, action: NetlistFirstAction) {
    match action {
        NetlistFirstAction::OpenNetlistWorkspace => {
            Command::OpenWorkspace(Workspace::Netlist).execute(app);
        }
        NetlistFirstAction::CreateSchematic => Command::NewCell.execute(app),
    }
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut action = None;
    egui::Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_app)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            header(ui, app.state.workspace.project.name());
            body(ui, &mut app.state, &mut action);
        });
    if let Some(action) = action {
        execute_netlist_first_action(app, action);
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

fn body(ui: &mut Ui, state: &mut AppState, action: &mut Option<NetlistFirstAction>) {
    let index = visible_source_index(state);
    let facts = deck_facts(state, &index);

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
            .show(&mut right, |ui| rail(ui, &facts, action));
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
                rail(ui, &facts, action);
            });
    }
}

fn rail(ui: &mut Ui, facts: &DeckFacts, action: &mut Option<NetlistFirstAction>) {
    for (title, detail, icon, primary, next) in [
        (
            // The row dispatches this exact command, so it reads the command's
            // own label rather than keeping a second copy of the workspace name.
            Command::OpenWorkspace(Workspace::Netlist).spec().label,
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
    let t = Tokens::get(ui.ctx());
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
            property_row_toned(
                ui,
                "Dependencies",
                &format!(
                    "{} \u{00b7} {} unresolved",
                    facts.dependencies, facts.unresolved_dependencies
                ),
                t.color.warn,
            );
        } else {
            property_row(ui, "Dependencies", &facts.dependencies.to_string());
        }
    });
}

/// The read-only deck panel. The whole panel is one control that opens the
/// Netlist & Script Editor — the deck is the document this project continues
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
    if super::activated(ui, &response) {
        *action = Some(NetlistFirstAction::OpenNetlistWorkspace);
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand);
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
        .map_or(toolbar.right() - 11.0, |publication| {
            let (label, tone) = publication_presentation(publication, t);
            ui.painter()
                .text(
                    pos2(toolbar.right() - 11.0, toolbar.center().y),
                    Align2::RIGHT_CENTER,
                    label,
                    state_font.clone(),
                    tone,
                )
                .left()
        });
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
                "\u{2026} {} more lines \u{00b7} open in the Netlist & Script Editor",
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

fn publication_presentation(
    publication: OwnedNetlistPublicationState,
    t: &Tokens,
) -> (&'static str, Color32) {
    let tone = if publication.external_change_pending {
        t.color.warn
    } else if publication.validated {
        t.color.ok
    } else {
        t.color.text_dim
    };
    (publication.label(), tone)
}

/// Everything the page states about the deck, derived once per frame from the
/// canonical document and the editor's own cached source index.
struct DeckFacts {
    name: String,
    dialect: Option<&'static str>,
    lines: usize,
    devices: usize,
    analyses: usize,
    models: usize,
    subcircuits: usize,
    dependencies: usize,
    unresolved_dependencies: usize,
    publication: Option<OwnedNetlistPublicationState>,
}

fn deck_facts(state: &AppState, index: &NetlistSourceIndex) -> DeckFacts {
    let document = state.workspace.netlist_document.as_ref();
    let name = state
        .workspace
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
    let dialect = state
        .workspace
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
    let publication =
        document.map(|document| owned_netlist_publication_state(state, document.content_digest()));

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

#[cfg(test)]
mod tests {
    use super::*;

    const DECK: &str = "Precision front-end bench\n\
        * differential input stage\n\
        V1 in 0 AC 1\n\
        R1 in inn 4.7k\n\
        C1 inn 0 220p\n\
        M1 out inn 0 0 NFET W=10u L=1u\n\
        .model NFET NMOS (LEVEL=1 VTO=0.7)\n\
        .subckt buffer a y\n\
        E1 y 0 a 0 1\n\
        .ends\n\
        .tran 1n 1u\n\
        .op\n\
        .end\n";

    const SUPPORTED_VIEWPORTS: [(&str, Vec2); 3] = [
        ("1440x900", Vec2::new(1440.0, 900.0)),
        ("820x1180", Vec2::new(820.0, 1180.0)),
        ("390x844", Vec2::new(390.0, 844.0)),
    ];
    #[derive(Clone, Copy)]
    struct NetlistFirstBaseline {
        name: &'static str,
        size: Vec2,
        truncated: bool,
        fingerprint: &'static str,
    }

    const NETLIST_FIRST_BASELINES: [NetlistFirstBaseline; 4] = [
        NetlistFirstBaseline {
            name: "design-netlist-first-1440x900",
            size: Vec2::new(1440.0, 900.0),
            truncated: false,
            fingerprint: "b48463edaa1cca3ded701d236f979dedf85234407ebf82800416ded9c08839bf",
        },
        NetlistFirstBaseline {
            name: "design-netlist-first-820x1180",
            size: Vec2::new(820.0, 1180.0),
            truncated: false,
            fingerprint: "56dead751affee02043e750cb3f244cde376346497e2025356b6a0bda3d4f5e5",
        },
        NetlistFirstBaseline {
            name: "design-netlist-first-390x844",
            size: Vec2::new(390.0, 844.0),
            truncated: false,
            fingerprint: "458f6391ca34d31a6c215db6b882bdba1be8372f7d2bb7227eda00e3a2110898",
        },
        NetlistFirstBaseline {
            name: "design-netlist-first-1440x900-truncated",
            size: Vec2::new(1440.0, 900.0),
            truncated: true,
            fingerprint: "93c7e7295618e57d7be01396707e79d9c61ba0f65a837887cfe0e54adadf4605",
        },
    ];

    fn netlist_first_app() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        assert!(
            crate::workbench::workflows::netlist_workflow::apply_imported_netlist(
                &mut app.state,
                DECK.to_owned(),
                None,
                "front_end.sp",
            )
        );
        assert!(app.state.is_netlist_first_without_schematic());
        app
    }

    fn truncated_netlist_first_app() -> RSpiceApp {
        let mut deck = String::from("Long resistor ladder bench\n");
        for element in 0..400 {
            deck.push_str(&format!("R{element} n{element} 0 1k\n"));
        }
        deck.push_str(".op\n.end\n");
        let mut app = RSpiceApp::test_instance();
        assert!(
            crate::workbench::workflows::netlist_workflow::apply_imported_netlist(
                &mut app.state,
                deck,
                None,
                "ladder.sp",
            )
        );
        assert!(app.state.is_netlist_first_without_schematic());
        app
    }

    fn netlist_first_canvas(size: Vec2, truncated: bool) -> crate::ui::raster::Canvas {
        let mut app = if truncated {
            truncated_netlist_first_app()
        } else {
            netlist_first_app()
        };
        crate::ui::raster::render(size, |ui, _| show(ui, &mut app))
    }

    fn regression_height(canvas: &crate::ui::raster::Canvas) -> usize {
        canvas.content_height().max(1)
    }

    #[test]
    fn netlist_first_visual_baselines_cover_supported_viewports_and_truncation_and_are_unique() {
        assert_eq!(NETLIST_FIRST_BASELINES.len(), SUPPORTED_VIEWPORTS.len() + 1);
        assert_eq!(
            NETLIST_FIRST_BASELINES[..SUPPORTED_VIEWPORTS.len()]
                .iter()
                .map(|baseline| baseline.size)
                .collect::<Vec<_>>(),
            SUPPORTED_VIEWPORTS.map(|(_, size)| size),
            "the baseline table must cover every supported netlist-first viewport"
        );
        assert!(
            NETLIST_FIRST_BASELINES[..SUPPORTED_VIEWPORTS.len()]
                .iter()
                .all(|baseline| !baseline.truncated)
        );
        let truncated = NETLIST_FIRST_BASELINES
            .last()
            .expect("truncated netlist-first baseline");
        assert!(truncated.truncated);
        assert_eq!(truncated.size, SUPPORTED_VIEWPORTS[0].1);
        let names = NETLIST_FIRST_BASELINES
            .iter()
            .map(|baseline| baseline.name)
            .collect::<std::collections::HashSet<_>>();
        let fingerprints = NETLIST_FIRST_BASELINES
            .iter()
            .map(|baseline| baseline.fingerprint)
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(names.len(), NETLIST_FIRST_BASELINES.len());
        assert_eq!(
            fingerprints.len(),
            NETLIST_FIRST_BASELINES.len(),
            "every supported viewport/state must have an independently reviewed fingerprint"
        );
    }

    #[test]
    fn supported_netlist_first_landings_match_their_reviewed_visual_baselines() {
        for baseline in NETLIST_FIRST_BASELINES {
            let canvas = netlist_first_canvas(baseline.size, baseline.truncated);
            canvas.assert_regression(
                baseline.name,
                regression_height(&canvas),
                baseline.fingerprint,
            );
        }
    }

    #[test]
    #[ignore = "prints source-ready visual fingerprints after explicit review"]
    fn print_netlist_first_visual_fingerprints_for_review() {
        for baseline in NETLIST_FIRST_BASELINES {
            let canvas = netlist_first_canvas(baseline.size, baseline.truncated);
            let height = regression_height(&canvas);
            eprintln!(
                "{} {}x{} {}",
                baseline.name,
                canvas.width(),
                height,
                canvas.regression_fingerprint(height)
            );
        }
    }

    #[test]
    fn imported_deck_with_only_the_pristine_bootstrap_buffer_is_netlist_first() {
        let mut app = RSpiceApp::test_instance();
        assert!(
            crate::workbench::workflows::netlist_workflow::apply_imported_netlist(
                &mut app.state,
                "V1 out 0 1\n.op\n.end\n".to_owned(),
                None,
                "front_end.sp",
            )
        );

        assert!(app.state.is_netlist_first_without_schematic());

        app.state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(0, 0),
        );
        assert!(!app.state.is_netlist_first_without_schematic());
    }

    #[test]
    fn netlist_first_empty_state_actions_use_the_canonical_commands() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;

        execute_netlist_first_action(&mut app, NetlistFirstAction::OpenNetlistWorkspace);
        assert_eq!(app.state.workbench.workspace, Workspace::Netlist);

        execute_netlist_first_action(&mut app, NetlistFirstAction::CreateSchematic);
        assert!(app.state.dialogs.new_cell_dialog);
        assert!(app.state.dialogs.new_cell_create_schematic);
    }

    #[test]
    fn deck_facts_read_the_live_document_rather_than_placeholders() {
        let mut app = netlist_first_app();
        let index = visible_source_index(&mut app.state);
        let facts = deck_facts(&app.state, &index);

        assert!(facts.name.contains("front_end"), "name: {}", facts.name);
        assert_eq!(facts.lines, 13);
        // V1, R1, C1, M1 at the top level and E1 inside the subcircuit.
        assert_eq!(facts.devices, 5);
        // .tran and .op.
        assert_eq!(facts.analyses, 2);
        assert_eq!(facts.models, 1);
        assert_eq!(facts.subcircuits, 1);
        assert_eq!(facts.dependencies, 0);
        assert!(
            facts.publication.is_some(),
            "an owned deck has a publication state"
        );
    }

    /// Every string the page paints in one pass, so wording is asserted on the
    /// galleys rather than read off pixels.
    fn painted_text(size: Vec2, mut pass: impl FnMut(&mut Ui)) -> Vec<String> {
        fn walk(shape: &egui::epaint::Shape, out: &mut Vec<String>) {
            match shape {
                egui::epaint::Shape::Text(text) => out.push(text.galley.text().to_owned()),
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, out);
                    }
                }
                _ => {}
            }
        }

        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut shapes = Vec::new();
        for _ in 0..2 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| pass(ui));
                    },
                )
                .shapes;
        }
        let mut out = Vec::new();
        for clipped in &shapes {
            walk(&clipped.shape, &mut out);
        }
        out
    }

    #[test]
    fn the_page_names_the_deck_its_actions_and_its_facts() {
        let mut app = netlist_first_app();
        let text = painted_text(vec2(1440.0, 900.0), |ui| show(ui, &mut app)).join("\n");
        for expected in [
            EYEBROW,
            "Open Netlist & Script Editor",
            "Create schematic\u{2026}",
            "Promote to a schematic-driven project",
            "Deck source",
            "front_end",
            "Deck summary",
            "Devices",
            "Analyses",
            "Precision front-end bench",
        ] {
            assert!(text.contains(expected), "missing {expected:?} in:\n{text}");
        }
    }

    #[test]
    fn a_long_deck_truncates_behind_a_footer_that_counts_what_was_left_out() {
        let mut app = truncated_netlist_first_app();

        let text = painted_text(vec2(1440.0, 900.0), |ui| show(ui, &mut app)).join("\n");
        assert!(
            text.contains("more lines"),
            "no truncation footer in:\n{text}"
        );
        assert!(
            !text.contains("R399"),
            "the deck's tail painted past the viewport"
        );
    }

    #[test]
    fn the_page_paints_the_full_landing_at_all_supported_viewports() {
        for ((_, size), minimum_content_height) in
            SUPPORTED_VIEWPORTS.into_iter().zip([500, 780, 740])
        {
            let canvas = netlist_first_canvas(size, false);
            let content_height = canvas.content_height();
            assert!(
                content_height >= minimum_content_height,
                "{}x{}: content stops at row {content_height}",
                size.x,
                size.y
            );
        }
    }

    /// On the two-column layout the deck panel and the rail both carry ink in
    /// their own columns — the failure mode this catches is the old page,
    /// whose body was one empty band.
    #[test]
    fn the_two_column_landing_fills_both_columns() {
        let mut app = netlist_first_app();
        let size = vec2(1440.0, 900.0);
        let canvas = crate::ui::raster::render(size, |ui, _| show(ui, &mut app));

        // Content column: 1240 centered in 1440 leaves 100 on each side. The
        // preview occupies the left column and the rail the last 322 points.
        let preview_probe = Rect::from_min_size(pos2(160.0, 300.0), vec2(300.0, 200.0));
        let rail_probe = Rect::from_min_size(pos2(1040.0, 160.0), vec2(240.0, 60.0));
        for (label, probe) in [("deck preview", preview_probe), ("action rail", rail_probe)] {
            assert!(
                canvas
                    .pixels_in(probe)
                    .any(|pixel| pixel != canvas.background()),
                "{label} painted nothing"
            );
        }
    }

    /// Write the landing at the supported viewports to PNGs so the design can
    /// be reviewed. Read them for layout, not wording — `crate::ui::raster`'s
    /// header says why.
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_the_landing_at_every_supported_viewport() {
        use std::io::Write as _;

        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        let stderr = std::io::stderr();
        let mut report_output = stderr.lock();

        for baseline in NETLIST_FIRST_BASELINES {
            let canvas = netlist_first_canvas(baseline.size, baseline.truncated);
            let height = regression_height(&canvas);
            let path = directory.join(format!("{}.png", baseline.name));
            std::fs::write(&path, canvas.png(height)).expect("write the render");
            writeln!(
                report_output,
                "{} {}x{}",
                path.display(),
                canvas.width(),
                height
            )
            .ok();
        }
    }
}

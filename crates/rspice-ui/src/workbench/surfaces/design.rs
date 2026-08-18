//! Hierarchical design document surface.

mod layout_editor;

use egui::{Align2, Context, Id, Key, Modifiers, Order, Rect, Response, Sense, Stroke, Ui, Vec2};

use crate::state::ViewType;
use crate::state::workspace::DocumentOccurrence;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::state::Workspace;
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::{WorkbenchIcon, empty_state};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.is_netlist_first_without_schematic() {
        netlist_first_empty_state(ui, app);
        return;
    }
    if app.state.active_view_read_only() {
        read_only_banner(ui, app);
    }
    occurrence_scope_banner(ui, app);
    let content_rect = ui.available_rect_before_wrap();
    let canvas_document = matches!(
        app.state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    );
    match app.state.workspace.active_view_type() {
        ViewType::Schematic | ViewType::Testbench => {
            crate::schematic::view::render_schematic_view(
                ui,
                &mut app.state,
                app.symbol_library.as_ref(),
            );
            paint_live_cursors(ui, app, content_rect);
        }
        ViewType::Symbol => crate::schematic::symbol_editor::show(ui, &mut app.state),
        ViewType::Layout => layout_editor::show(ui, app),
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => source_document(ui, app),
        view_type => unsupported_document(ui, app, view_type),
    }
    breadcrumb(ui.ctx(), &mut app.state, content_rect);
    if canvas_document
        && app.state.workbench.current_route().surface_id() == super::super::SurfaceId::Design
    {
        crate::schematic::view::show_mobile_canvas_controls(ui.ctx(), app, content_rect);
    }
}

fn paint_live_cursors(ui: &mut Ui, app: &RSpiceApp, canvas: Rect) {
    use crate::services::live_protocol::CursorLocus;

    let doc = format!("schematic/{}", app.state.workspace.active_key());
    let cursors: Vec<_> = app
        .live_session
        .peers()
        .filter_map(|peer| match peer.cursor.as_ref() {
            Some(CursorLocus::Canvas {
                doc: peer_doc,
                x,
                y,
            }) if peer_doc == &doc && x.is_finite() && y.is_finite() => Some((
                peer.identity,
                app.live_session.display_name(&peer.identity),
                *x,
                *y,
            )),
            _ => None,
        })
        .collect();
    if cursors.is_empty() {
        return;
    }

    let t = Tokens::get(ui.ctx());
    let painter = ui.painter_at(canvas);
    let grid = app.state.schematic.grid_size.max(1) as f32;
    let zoom = app.state.schematic.zoom as f32;
    let pan = egui::vec2(
        app.state.schematic.pan.0 as f32,
        app.state.schematic.pan.1 as f32,
    );
    const COLORS: [egui::Color32; 8] = [
        egui::Color32::from_rgb(0x2F, 0xC7, 0xE5),
        egui::Color32::from_rgb(0xFF, 0x9F, 0x43),
        egui::Color32::from_rgb(0xA7, 0x7B, 0xFF),
        egui::Color32::from_rgb(0x37, 0xD6, 0x7A),
        egui::Color32::from_rgb(0xFF, 0x61, 0x91),
        egui::Color32::from_rgb(0xFF, 0xD1, 0x66),
        egui::Color32::from_rgb(0x52, 0x8B, 0xFF),
        egui::Color32::from_rgb(0xE8, 0x70, 0xFF),
    ];
    let mut accessible = Vec::with_capacity(cursors.len());
    for (identity, name, x, y) in cursors {
        let point = canvas.min + pan + egui::vec2(x * zoom * grid, y * zoom * grid);
        if !canvas.expand(16.0).contains(point) {
            continue;
        }
        let color = COLORS[(identity.principal_id.as_u128() as usize) % COLORS.len()];
        let tip = point;
        painter.add(egui::Shape::convex_polygon(
            vec![
                tip,
                tip + egui::vec2(7.0, 18.0),
                tip + egui::vec2(11.0, 11.0),
            ],
            color,
            egui::Stroke::new(1.0, t.color.bg_app),
        ));
        let galley = painter.layout_no_wrap(
            name.clone(),
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text,
        );
        let desired = tip + egui::vec2(12.0, 13.0);
        let label_size = galley.size() + egui::vec2(10.0, 6.0);
        let label_min = egui::pos2(
            desired
                .x
                .min((canvas.right() - label_size.x).max(canvas.left())),
            desired
                .y
                .min((canvas.bottom() - label_size.y).max(canvas.top())),
        );
        let label = Rect::from_min_size(label_min, label_size);
        painter.rect_filled(label, 4.0, t.color.bg_panel);
        painter.rect_stroke(
            label,
            4.0,
            egui::Stroke::new(1.0, color),
            egui::StrokeKind::Inside,
        );
        painter.galley(label.min + egui::vec2(5.0, 3.0), galley, t.color.text);
        accessible.push(format!("{name} cursor at {x:.1}, {y:.1}"));
    }
    if !accessible.is_empty() {
        let response = ui.interact(
            Rect::from_min_size(canvas.min, Vec2::splat(1.0)),
            Id::new("live-cursor-accessibility"),
            Sense::hover(),
        );
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(format!("Live collaboration: {}", accessible.join("; ")));
        });
    }
}

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

fn netlist_first_empty_state(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let available_width = ui.available_width().max(1.0);
    let project_name = app.state.workspace.project.name().to_owned();

    let (header_rect, _) =
        ui.allocate_exact_size(Vec2::new(available_width, 118.0), Sense::hover());
    ui.painter().rect_filled(header_rect, 0.0, t.color.bg_app);
    ui.painter().line_segment(
        [header_rect.left_bottom(), header_rect.right_bottom()],
        Stroke::new(1.0, t.color.border),
    );
    let header_content = Rect::from_min_max(
        header_rect.min + egui::vec2(30.0, 25.0),
        egui::pos2(
            (header_rect.left() + 750.0).min(header_rect.right() - 30.0),
            header_rect.bottom() - 18.0,
        ),
    );
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(header_content)
            .layout(egui::Layout::top_down(egui::Align::LEFT)),
        |ui| {
            ui.spacing_mut().item_spacing.y = 3.0;
            ui.label(
                egui::RichText::new("NETLIST-FIRST PROJECT \u{00b7} NO SCHEMATIC")
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.text_dim),
            );
            ui.label(
                egui::RichText::new(project_name)
                    .font(theme::sans(tokens::FS_4, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.label(
                egui::RichText::new(
                    "This project is driven by its SPICE deck. The Netlist workspace owns editing; simulation, probing, and results work exactly as in schematic projects. Create a schematic to promote this into a schematic-driven design.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text_dim),
            );
        },
    );

    let columns = if available_width <= 460.0 {
        1
    } else if available_width <= 760.0 {
        2
    } else {
        4
    };
    let rows = 2_usize.div_ceil(columns);
    let row_height = 72.0 * rows as f32;
    let (actions_rect, _) =
        ui.allocate_exact_size(Vec2::new(available_width, row_height), Sense::hover());
    ui.painter()
        .rect_filled(actions_rect, 0.0, t.color.bg_inset);
    ui.painter().line_segment(
        [actions_rect.left_bottom(), actions_rect.right_bottom()],
        Stroke::new(1.0, t.color.border),
    );

    let column_width = actions_rect.width() / columns as f32;
    let actions = [
        (
            NetlistFirstAction::OpenNetlistWorkspace,
            WorkbenchIcon::Code,
            // The card dispatches this exact command, so it reads the command's
            // own label rather than keeping a second copy of the workspace name.
            Command::OpenWorkspace(Workspace::Netlist).spec().label,
            "Deck source \u{00b7} outline \u{00b7} diagnostics \u{00b7} overlay",
            true,
        ),
        (
            NetlistFirstAction::CreateSchematic,
            WorkbenchIcon::Design,
            "Create schematic\u{2026}",
            "Promote to a schematic-driven project",
            false,
        ),
    ];
    let mut invoked = None;
    for (index, (action, icon, title, detail, primary)) in actions.into_iter().enumerate() {
        let row = index / columns;
        let column = index % columns;
        let rect = Rect::from_min_size(
            actions_rect.min + egui::vec2(column as f32 * column_width, row as f32 * 72.0),
            Vec2::new(column_width, 72.0),
        );
        let response = ui.interact(
            rect,
            ui.id().with(("netlist-first-action", index)),
            Sense::click(),
        );
        response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, title));
        if response.hovered() {
            ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
        }
        ui.painter().line_segment(
            [rect.right_top(), rect.right_bottom()],
            Stroke::new(1.0, t.color.border),
        );
        if primary {
            ui.painter().line_segment(
                [
                    egui::pos2(rect.left(), rect.bottom() - 1.0),
                    egui::pos2(rect.right(), rect.bottom() - 1.0),
                ],
                Stroke::new(2.0, t.color.accent),
            );
        }
        icon.paint(
            ui.painter(),
            Rect::from_min_size(rect.min + egui::vec2(15.0, 21.0), Vec2::splat(30.0)),
            if primary {
                t.color.accent
            } else {
                t.color.text_dim
            },
        );
        ui.painter().text(
            rect.min + egui::vec2(53.0, 17.0),
            Align2::LEFT_TOP,
            title,
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
        ui.painter().text(
            rect.min + egui::vec2(53.0, 39.0),
            Align2::LEFT_TOP,
            detail,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        theme::paint_focus_ring_outset(ui, &response, rect);
        if response.clicked() {
            invoked = Some(action);
        }
    }

    if let Some(action) = invoked {
        execute_netlist_first_action(app, action);
    }
}

const CANVAS_BREADCRUMB_FONT_SIZE: f32 = tokens::FS_1;
const CANVAS_BREADCRUMB_SEPARATOR: &str = " / ";
const CANVAS_BREADCRUMB_ELISION: &str = "\u{2026}";

/// One crumb on the canvas breadcrumb: what it reads, and the occurrence level
/// it opens.
///
/// `level` is the argument [`AppState::focus_workspace_breadcrumb`] takes, so a
/// crumb navigates through the occurrence routes rather than by rewriting the
/// derived hierarchy projection. A crumb that names something other than a
/// level — the library the design root lives in, the view that is open, the
/// elision that stands for crumbs there was no room for — carries none and is
/// not a place you can go.
#[derive(Debug, Clone, PartialEq, Eq)]
struct BreadcrumbCrumb {
    label: String,
    level: Option<usize>,
}

impl BreadcrumbCrumb {
    /// A crumb that names something other than an occurrence level.
    fn label_only(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            level: None,
        }
    }
}

fn breadcrumb(ctx: &Context, state: &mut AppState, content_rect: Rect) {
    let t = Tokens::get(ctx);
    let crumbs = hierarchy_breadcrumb_segments(state);
    let maximum_frame_width = (content_rect.width() * 0.5 - 16.0).max(80.0);
    let mut focused_level = None;

    egui::Area::new(Id::new("workbench.design.canvas-breadcrumb"))
        .order(Order::Middle)
        .fixed_pos(content_rect.min + egui::vec2(10.0, 9.0))
        .constrain_to(content_rect)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(with_alpha(t.color.bg_panel, 240))
                .stroke(Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(9, 0))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    let inner_width = (maximum_frame_width - 18.0).max(62.0);
                    ui.set_max_width(inner_width);
                    ui.set_min_height(27.0);
                    ui.spacing_mut().item_spacing = Vec2::ZERO;
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        let shown = crumbs_within(ui, &crumbs, inner_width);
                        let last = shown.len().saturating_sub(1);
                        for (index, crumb) in shown.iter().enumerate() {
                            if index > 0 {
                                breadcrumb_separator(ui, &t);
                            }
                            if breadcrumb_crumb(ui, &t, crumb, index == last)
                                && let Some(level) = crumb.level
                            {
                                focused_level = Some(level);
                            }
                        }
                    });
                });
        });

    if let Some(level) = focused_level {
        state.focus_workspace_breadcrumb(level);
    }
}

fn breadcrumb_separator(ui: &mut Ui, t: &Tokens) {
    ui.add(
        egui::Label::new(
            egui::RichText::new(CANVAS_BREADCRUMB_SEPARATOR)
                .font(theme::sans(
                    CANVAS_BREADCRUMB_FONT_SIZE,
                    FontWeight::Regular,
                ))
                .color(t.color.text_faint),
        )
        .selectable(false),
    );
}

/// Paint one crumb and report whether it was activated.
///
/// A crumb that opens a level is a control: it takes focus, answers the pointer
/// and the keyboard alike, and underlines itself so the affordance is visible
/// before the click rather than only after it.
fn breadcrumb_crumb(ui: &mut Ui, t: &Tokens, crumb: &BreadcrumbCrumb, is_view: bool) -> bool {
    let navigable = crumb.level.is_some();
    let text = egui::RichText::new(crumb.label.as_str())
        .font(theme::sans(
            CANVAS_BREADCRUMB_FONT_SIZE,
            if is_view {
                FontWeight::Regular
            } else {
                FontWeight::Medium
            },
        ))
        .color(if is_view {
            t.color.text_dim
        } else {
            t.color.text
        });
    let mut label = egui::Label::new(text).selectable(false);
    if navigable {
        label = label.sense(Sense::click());
    }
    let response = ui.add(label);
    if !navigable {
        return false;
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Link, true, crumb.label.as_str())
    });
    if response.hovered() || response.has_focus() {
        ui.painter().hline(
            response.rect.x_range(),
            response.rect.bottom() - 1.0,
            Stroke::new(1.0, t.color.accent),
        );
    }
    theme::paint_focus_ring_outset(ui, &response, response.rect);
    activated(ui, &response)
}

/// Whether a focusable crumb was clicked or activated from the keyboard.
///
/// The key is consumed here so the canvas behind the breadcrumb does not also
/// act on the same press.
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

/// The crumbs that fit `available`, eliding from the second position inward.
///
/// The design root and the deepest levels are what say where an edit lands, so
/// they are the last things given up; what disappears first is the middle of a
/// long descent, replaced by one elision crumb.
fn crumbs_within(ui: &Ui, crumbs: &[BreadcrumbCrumb], available: f32) -> Vec<BreadcrumbCrumb> {
    let widths: Vec<f32> = crumbs
        .iter()
        .map(|crumb| crumb_width(ui, &crumb.label, FontWeight::Medium))
        .collect();
    let separator = crumb_width(ui, CANVAS_BREADCRUMB_SEPARATOR, FontWeight::Regular);
    let elision = crumb_width(ui, CANVAS_BREADCRUMB_ELISION, FontWeight::Medium);
    let row = |kept: &[usize], elided: bool| -> f32 {
        let painted = kept.len() + usize::from(elided);
        kept.iter().map(|index| widths[*index]).sum::<f32>()
            + if elided { elision } else { 0.0 }
            + separator * painted.saturating_sub(1) as f32
    };

    let mut kept: Vec<usize> = (0..crumbs.len()).collect();
    let mut elided = false;
    // Position 0 is the outermost crumb and the last two are the deepest level
    // and the view, so the middle is everything from index 1 up to len - 3.
    while row(&kept, elided) > available && kept.len() > 3 {
        kept.remove(1);
        elided = true;
    }

    let mut shown = Vec::with_capacity(kept.len() + 1);
    for (position, index) in kept.into_iter().enumerate() {
        if position == 1 && elided {
            shown.push(BreadcrumbCrumb::label_only(CANVAS_BREADCRUMB_ELISION));
        }
        shown.push(crumbs[index].clone());
    }
    shown
}

fn crumb_width(ui: &Ui, text: &str, weight: FontWeight) -> f32 {
    ui.painter()
        .layout_no_wrap(
            text.to_owned(),
            theme::sans(CANVAS_BREADCRUMB_FONT_SIZE, weight),
            egui::Color32::PLACEHOLDER,
        )
        .size()
        .x
}

fn hierarchy_breadcrumb_segments(state: &AppState) -> Vec<BreadcrumbCrumb> {
    use crate::state::SchematicHierarchyVisibility;

    let active = &state.workspace.active_view;
    let occurrence = state
        .workspace
        .active_occurrence()
        .cloned()
        .unwrap_or_else(|| DocumentOccurrence::rooted(active.clone()));
    let visibility = if matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        state.ui.schematic_visibility.hierarchy
    } else {
        SchematicHierarchyVisibility::FullVisibleHierarchy
    };
    let deepest = occurrence.depth() - 1;
    let mut crumbs = Vec::with_capacity(occurrence.depth() + 2);
    match visibility {
        SchematicHierarchyVisibility::ActiveOnly => {
            crumbs.push(occurrence_level_crumb(&occurrence, deepest));
        }
        SchematicHierarchyVisibility::ActiveAndParent => {
            for level in deepest.saturating_sub(1)..=deepest {
                crumbs.push(occurrence_level_crumb(&occurrence, level));
            }
        }
        // The root cellview is a crumb because the canvas names what is open,
        // but it is not a path segment: the occurrence below it comes from the
        // path type, whose root is implicit.
        SchematicHierarchyVisibility::FullVisibleHierarchy => {
            crumbs.push(BreadcrumbCrumb::label_only(occurrence.root.library.clone()));
            for level in 0..=deepest {
                crumbs.push(occurrence_level_crumb(&occurrence, level));
            }
        }
    }
    crumbs.push(BreadcrumbCrumb::label_only(active.view.clone()));
    crumbs
}

/// One occurrence level as a crumb: the instance stepped through and the master
/// it opened, always both.
///
/// `X1` and `X2` of one master are the same word in every reading of the design
/// that drops the master, and the master alone is the same word at every one of
/// its occurrences — so a crumb that names only one of the two names something
/// other than the level it opens.
fn occurrence_level_crumb(occurrence: &DocumentOccurrence, level: usize) -> BreadcrumbCrumb {
    let label = level
        .checked_sub(1)
        .and_then(|step| occurrence.steps.get(step))
        .map_or_else(
            || occurrence.root.cell.clone(),
            |step| format!("{} \u{00b7} {}", step.instance_name, step.master.cell),
        );
    BreadcrumbCrumb {
        label,
        level: Some(level),
    }
}

fn with_alpha(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

/// Say when an edit to the open document lands on more than one occurrence.
///
/// A master instantiated once is the ordinary case and needs no words. A master
/// instantiated more than once is the case where an edit that looks local is
/// not, so the canvas says so beside the read-only marking — before the edit
/// rather than after it — and names every occurrence it will reach on hover.
///
/// `AppState::master_occurrence_paths` is the one derivation of that scope; the
/// sheet inspector's edit-scope row reads the same function.
fn occurrence_scope_banner(ui: &mut Ui, app: &RSpiceApp) {
    let occurrences = app.state.master_occurrence_paths();
    if occurrences.len() < 2 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.info.gamma_multiply(0.12))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "Edits here apply to all {} occurrences of {}.",
                    occurrences.len(),
                    app.state.workspace.active_view.cell
                ))
                .color(t.color.info),
            )
            .on_hover_text(occurrences.join("\n"));
        });
}

fn read_only_banner(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.warn.gamma_multiply(0.14))
        .inner_margin(egui::Margin::symmetric(12, 6))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} is read only. Create an editable copy before changing this document.",
                    app.state.workspace.active_display_path()
                ))
                .color(t.color.warn),
            );
        });
}

fn source_document(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let reference = app.state.workspace.active_view.clone();
    let contents = app
        .state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .and_then(|cell| cell.get_view(&reference.view))
        .and_then(|view| view.metadata.get("source"))
        .cloned()
        .unwrap_or_default();
    if contents.is_empty() {
        empty_state(
            ui,
            super::super::design_system::WorkbenchIcon::Netlist,
            "No source text stored",
            "Import or compile this behavioral view from the Models workspace.",
        );
        return;
    }
    let mut display = contents;
    egui::Frame::new().fill(t.color.canvas_bg).show(ui, |ui| {
        ui.add_sized(
            ui.available_size(),
            egui::TextEdit::multiline(&mut display)
                .font(egui::TextStyle::Monospace)
                .code_editor()
                .interactive(false),
        );
    });
}

fn unsupported_document(ui: &mut Ui, app: &RSpiceApp, view_type: ViewType) {
    empty_state(
        ui,
        super::super::design_system::WorkbenchIcon::File,
        &format!("{} view", view_type.display_name()),
        &format!(
            "{} is registered in the project and available for downstream integrations.",
            app.state.workspace.active_display_path()
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The canvas every render in this module uses. Wide enough that the crumb
    /// row is measured rather than elided.
    const RASTER_VIEWPORT: Vec2 = egui::vec2(720.0, 60.0);

    fn crumb_labels(state: &AppState) -> Vec<String> {
        hierarchy_breadcrumb_segments(state)
            .into_iter()
            .map(|crumb| crumb.label)
            .collect()
    }

    fn descended(state: &mut AppState, instance: &str, cell: &str) {
        state.workspace.descend_into(
            instance.to_owned(),
            crate::state::CellViewRef::new("user", cell, "schematic"),
            ViewType::Schematic,
        );
    }

    #[test]
    fn canvas_breadcrumb_uses_the_mockup_body_type_size() {
        assert_eq!(CANVAS_BREADCRUMB_FONT_SIZE, 12.0);
    }

    #[test]
    fn hierarchy_visibility_changes_only_the_canvas_context_breadcrumb() {
        let mut state = AppState::default();
        descended(&mut state, "XAMP", "amp");
        descended(&mut state, "XBIAS", "bias");

        state.ui.schematic_visibility.hierarchy =
            crate::state::SchematicHierarchyVisibility::ActiveOnly;
        assert_eq!(
            crumb_labels(&state),
            vec!["XBIAS \u{b7} bias".to_owned(), "schematic".to_owned()]
        );

        state.ui.schematic_visibility.hierarchy =
            crate::state::SchematicHierarchyVisibility::ActiveAndParent;
        assert_eq!(
            crumb_labels(&state),
            vec![
                "XAMP \u{b7} amp".to_owned(),
                "XBIAS \u{b7} bias".to_owned(),
                "schematic".to_owned(),
            ]
        );
    }

    #[test]
    fn the_full_breadcrumb_names_the_root_cellview_then_the_occurrence_below_it() {
        let mut state = AppState::default();
        state.ui.schematic_visibility.hierarchy =
            crate::state::SchematicHierarchyVisibility::FullVisibleHierarchy;
        assert_eq!(
            crumb_labels(&state),
            vec!["user".to_owned(), "top".to_owned(), "schematic".to_owned()],
            "the design root owns no path segment"
        );

        descended(&mut state, "XAFE", "afe_core");
        assert_eq!(
            crumb_labels(&state),
            vec![
                "user".to_owned(),
                "top".to_owned(),
                "XAFE \u{b7} afe_core".to_owned(),
                "schematic".to_owned(),
            ]
        );

        descended(&mut state, "XBIAS", "bias");
        assert_eq!(state.workspace.occurrence_path().to_string(), "/XAFE/XBIAS");
        assert_eq!(
            crumb_labels(&state),
            vec![
                "user".to_owned(),
                "top".to_owned(),
                "XAFE \u{b7} afe_core".to_owned(),
                "XBIAS \u{b7} bias".to_owned(),
                "schematic".to_owned(),
            ]
        );
    }

    /// Every level is a place you can go, and going there opens exactly that
    /// occurrence rather than the deepest one that shares its master.
    #[test]
    fn a_breadcrumb_level_opens_the_occurrence_it_names() {
        let mut app = RSpiceApp::test_instance();
        app.state.ui.schematic_visibility.hierarchy =
            crate::state::SchematicHierarchyVisibility::FullVisibleHierarchy;
        descended(&mut app.state, "XAFE", "afe_core");
        descended(&mut app.state, "XBIAS", "bias");

        let crumbs = hierarchy_breadcrumb_segments(&app.state);
        let levels: Vec<Option<usize>> = crumbs.iter().map(|crumb| crumb.level).collect();
        assert_eq!(
            levels,
            vec![None, Some(0), Some(1), Some(2), None],
            "the library and the view name no level; every occurrence level does"
        );

        let intermediate = crumbs
            .iter()
            .find(|crumb| crumb.label.starts_with("XAFE"))
            .and_then(|crumb| crumb.level)
            .expect("the descended level is a crumb");
        app.state.focus_workspace_breadcrumb(intermediate);

        assert_eq!(app.state.workspace.occurrence_path().to_string(), "/XAFE");
        assert_eq!(app.state.workspace.active_view.cell, "afe_core");

        let root = hierarchy_breadcrumb_segments(&app.state)
            .into_iter()
            .find(|crumb| crumb.label == "top")
            .and_then(|crumb| crumb.level)
            .expect("the design root is a crumb");
        app.state.focus_workspace_breadcrumb(root);

        assert!(app.state.workspace.occurrence_path().is_root());
        assert_eq!(app.state.workspace.active_view.cell, "top");
    }

    /// A crumb states the instance *and* its master. Two occurrences of one
    /// master must not paint the same crumb, and one instance name under two
    /// masters must not either — which is what a crumb truncated to either half
    /// would do.
    #[test]
    fn breadcrumb_segments_render_instance_and_master() {
        fn render(instance: &str, master: &str) -> crate::ui::raster::Canvas {
            let mut app = RSpiceApp::test_instance();
            app.state.ui.schematic_visibility.hierarchy =
                crate::state::SchematicHierarchyVisibility::FullVisibleHierarchy;
            descended(&mut app.state, instance, master);
            crate::ui::raster::render(RASTER_VIEWPORT, |ui, _| {
                breadcrumb(
                    ui.ctx(),
                    &mut app.state,
                    Rect::from_min_size(egui::Pos2::ZERO, RASTER_VIEWPORT),
                );
            })
        }

        let band = Rect::from_min_size(egui::Pos2::ZERO, RASTER_VIEWPORT);
        let ink = |canvas: &crate::ui::raster::Canvas| -> Vec<egui::Color32> {
            canvas.pixels_in(band).collect()
        };

        let first = render("X1", "amp");
        let second = render("X2", "amp");
        let elsewhere = render("X1", "bias");

        assert!(
            ink(&first).iter().any(|pixel| *pixel != first.background()),
            "the breadcrumb painted nothing at all"
        );
        assert_ne!(
            ink(&first),
            ink(&second),
            "a crumb that dropped the instance would paint X1 and X2 identically"
        );
        assert_ne!(
            ink(&first),
            ink(&elsewhere),
            "a crumb that dropped the master would paint amp and bias identically"
        );
    }

    /// A project whose root sheet instantiates one cell twice, with the design
    /// root open. Descending into the shared cell is what puts a document with
    /// two occurrences in front.
    fn app_with_a_twice_instantiated_cell() -> (RSpiceApp, crate::state::CellViewRef) {
        use crate::state::{
            CellViewRef, ComponentType, Library, LibraryCellInstance, Point, SchematicState, View,
        };

        let mut app = RSpiceApp::test_instance();
        let root = app.state.workspace.active_view.clone();
        let shared = CellViewRef::new("user", "pad", "schematic");

        if app
            .state
            .library_manager
            .get_library(&shared.library)
            .is_none()
        {
            app.state
                .library_manager
                .add_library(Library::new(&shared.library));
        }
        let cell = app
            .state
            .library_manager
            .get_library_mut(&shared.library)
            .expect("the project library")
            .get_or_create_cell(&shared.cell);
        if cell.get_view(&shared.view).is_none() {
            cell.add_view(View::new(&shared.view, ViewType::Schematic));
        }

        let mut master = SchematicState::default();
        master.add_component(ComponentType::Resistor, Point::new(30, 0));
        app.state
            .workspace
            .schematic_buffers
            .insert(shared.key(), master);

        let mut binding = LibraryCellInstance::new(&shared.library, &shared.cell, &shared.view);
        binding.bind_interface(&[]);
        let mut root_sheet = SchematicState::default();
        root_sheet.add_library_cell_component(Point::new(400, 400), binding.clone());
        root_sheet.add_library_cell_component(Point::new(600, 400), binding);
        app.state.schematic = root_sheet.clone();
        app.state
            .workspace
            .schematic_buffers
            .insert(root.key(), root_sheet);
        (app, shared)
    }

    /// The strip states a scope, so it appears exactly where the scope is wider
    /// than the sheet in front of the reader: never at a design root, never on a
    /// master placed once, always on one placed more than once.
    #[test]
    fn occurrence_watermark_appears_only_above_one_instance() {
        fn render(app: &RSpiceApp) -> crate::ui::raster::Canvas {
            crate::ui::raster::render(RASTER_VIEWPORT, |ui, background| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(background))
                    .show(ui, |ui| occurrence_scope_banner(ui, app));
            })
        }

        let (mut app, shared) = app_with_a_twice_instantiated_cell();
        assert_eq!(
            app.state.master_occurrence_paths().len(),
            1,
            "the design root is instantiated once"
        );
        assert_eq!(
            render(&app).content_height(),
            0,
            "a design root must carry no occurrence marking"
        );

        let master = app
            .state
            .workspace
            .schematic_buffers
            .get(&shared.key())
            .cloned()
            .expect("the shared master buffer");
        app.state
            .workspace
            .descend_into("X1".to_owned(), shared.clone(), ViewType::Schematic);
        app.state.schematic = master;

        assert_eq!(
            app.state.master_occurrence_paths().len(),
            2,
            "the shared master is instantiated twice"
        );
        assert!(
            render(&app).content_height() > 0,
            "a master placed twice must say so before the edit"
        );
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
}

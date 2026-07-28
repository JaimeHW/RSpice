//! Hierarchical design document surface.

use egui::{Align2, Context, Id, Order, Rect, Sense, Stroke, Ui, Vec2};

use crate::state::ViewType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::commands::Command;
use crate::workbench::state::Workspace;
use crate::workbench::{AppState, RSpiceApp};

use super::super::design_system::{WorkbenchIcon, empty_state};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if is_netlist_first_without_schematic(&app.state) {
        netlist_first_empty_state(ui, app);
        return;
    }
    if app.state.active_view_read_only() {
        read_only_banner(ui, app);
    }
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
        }
        ViewType::Symbol => crate::schematic::symbol_editor::show(ui, &mut app.state),
        ViewType::Spice | ViewType::Verilog | ViewType::VerilogA => source_document(ui, app),
        view_type => unsupported_document(ui, app, view_type),
    }
    breadcrumb(ui.ctx(), app, content_rect);
    if canvas_document {
        canvas_check_note(ui.ctx(), app, content_rect);
        if app.state.workbench.current_route().surface_id() == super::super::SurfaceId::Design {
            crate::schematic::view::show_mobile_canvas_controls(ui.ctx(), app, content_rect);
        }
    }
}

fn is_netlist_first_without_schematic(state: &AppState) -> bool {
    let imported_deck_owns_the_project = state
        .workspace
        .netlist_document
        .as_ref()
        .is_some_and(|document| document.provenance().imported().is_some());
    if !imported_deck_owns_the_project || schematic_has_authored_content(&state.schematic) {
        return false;
    }

    // ProjectWorkspace keeps one bootstrap buffer so all legacy editor and
    // save invariants remain valid. An imported source project is still
    // netlist-first while that sole buffer is pristine. Creating any
    // schematic cell materializes a second buffer (or authored content) and
    // therefore promotes the Design surface without discarding the source
    // deck.
    state.workspace.schematic_buffers.len() <= 1
        && state
            .workspace
            .schematic_buffers
            .values()
            .all(|schematic| !schematic_has_authored_content(schematic))
}

fn schematic_has_authored_content(schematic: &crate::state::SchematicState) -> bool {
    !schematic.components.is_empty()
        || !schematic.wires.is_empty()
        || !schematic.buses.is_empty()
        || !schematic.bus_taps.is_empty()
        || !schematic.design_notes.is_empty()
        || !schematic.documentation_shapes.is_empty()
        || !schematic.probes.is_empty()
        || !schematic.net_labels.is_empty()
        || !schematic.junctions.is_empty()
        || !schematic.connections.is_empty()
        || !schematic.validated_revisions.is_empty()
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
            "Open netlist workspace",
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

fn breadcrumb(ctx: &Context, app: &RSpiceApp, content_rect: Rect) {
    let t = Tokens::get(ctx);
    let segments = hierarchy_breadcrumb_segments(&app.state);
    let mut text = egui::text::LayoutJob::default();
    for (index, segment) in segments.iter().enumerate() {
        if index > 0 {
            text.append(
                " / ",
                0.0,
                egui::TextFormat {
                    font_id: theme::sans(CANVAS_BREADCRUMB_FONT_SIZE, FontWeight::Regular),
                    color: t.color.text_faint,
                    ..Default::default()
                },
            );
        }
        let is_view = index + 1 == segments.len();
        text.append(
            segment,
            0.0,
            egui::TextFormat {
                font_id: theme::sans(
                    CANVAS_BREADCRUMB_FONT_SIZE,
                    if is_view {
                        FontWeight::Regular
                    } else {
                        FontWeight::Medium
                    },
                ),
                color: if is_view {
                    t.color.text_dim
                } else {
                    t.color.text
                },
                ..Default::default()
            },
        );
    }
    let maximum_frame_width = (content_rect.width() * 0.5 - 16.0).max(80.0);

    egui::Area::new(Id::new("workbench.design.canvas-breadcrumb"))
        .order(Order::Middle)
        .fixed_pos(content_rect.min + egui::vec2(10.0, 9.0))
        .constrain_to(content_rect)
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(with_alpha(t.color.bg_panel, 240))
                .stroke(Stroke::new(1.0, t.color.border))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(9, 0))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.set_max_width((maximum_frame_width - 18.0).max(62.0));
                    ui.set_min_height(27.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(text).truncate());
                    });
                });
        });
}

fn hierarchy_breadcrumb_segments(state: &AppState) -> Vec<String> {
    use crate::state::SchematicHierarchyVisibility;

    let active = &state.workspace.active_view;
    let visibility = if matches!(
        state.workspace.active_view_type(),
        ViewType::Schematic | ViewType::Testbench
    ) {
        state.ui.schematic_visibility.hierarchy
    } else {
        SchematicHierarchyVisibility::FullVisibleHierarchy
    };
    match visibility {
        SchematicHierarchyVisibility::ActiveOnly => {
            vec![active.cell.clone(), active.view.clone()]
        }
        SchematicHierarchyVisibility::ActiveAndParent => {
            let mut segments = state
                .workspace
                .hierarchy_stack
                .iter()
                .rev()
                .take(2)
                .map(|reference| reference.cell.clone())
                .collect::<Vec<_>>();
            segments.reverse();
            if segments.last() != Some(&active.cell) {
                segments.push(active.cell.clone());
            }
            segments.push(active.view.clone());
            segments
        }
        SchematicHierarchyVisibility::FullVisibleHierarchy => {
            let root_library = state
                .workspace
                .hierarchy_stack
                .first()
                .map_or(active.library.as_str(), |reference| {
                    reference.library.as_str()
                });
            let mut segments =
                Vec::with_capacity(state.workspace.hierarchy_stack.len().saturating_add(2));
            segments.push(root_library.to_owned());
            segments.extend(state.workspace.occurrence_labels());
            segments.push(active.view.clone());
            segments
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CheckNoteTone {
    Ok,
    Warning,
    Error,
}

const CANVAS_CHECK_NOTE_MIN_WIDTH: f32 = 620.0;

fn canvas_check_note_visible(viewport_width: f32) -> bool {
    viewport_width > CANVAS_CHECK_NOTE_MIN_WIDTH
}

fn canvas_check_note(ctx: &Context, app: &RSpiceApp, content_rect: Rect) {
    // The upgraded mockup keeps the current/stale engineering status visible
    // on tablets and phone landscape, suppressing it only in the narrow
    // portrait composition where it would collide with the breadcrumb.
    if !canvas_check_note_visible(ctx.content_rect().width()) {
        return;
    }
    let (message, tone) = check_note_content(&app.state);
    let t = Tokens::get(ctx);
    let color = match tone {
        CheckNoteTone::Ok => t.color.ok,
        CheckNoteTone::Warning => t.color.warn,
        CheckNoteTone::Error => t.color.err,
    };

    egui::Area::new(Id::new("workbench.design.canvas-check-note"))
        .order(Order::Middle)
        .pivot(Align2::RIGHT_TOP)
        .fixed_pos(content_rect.right_top() + egui::vec2(-11.0, 10.0))
        .constrain_to(content_rect)
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(with_alpha(t.color.bg_panel, 245))
                .stroke(Stroke::new(1.0, color.gamma_multiply(0.55)))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(8, 0))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.set_max_width((content_rect.width() * 0.5 - 32.0).max(80.0));
                    ui.set_min_height(27.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 7.0;
                        let (icon_rect, _) =
                            ui.allocate_exact_size(egui::Vec2::splat(13.0), egui::Sense::hover());
                        match tone {
                            CheckNoteTone::Ok => WorkbenchIcon::Success,
                            CheckNoteTone::Warning | CheckNoteTone::Error => WorkbenchIcon::Warning,
                        }
                        .paint(ui.painter(), icon_rect, color);
                        ui.label(
                            egui::RichText::new(message)
                                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                                .color(color),
                        );
                    });
                });
        });
}

fn with_alpha(color: egui::Color32, alpha: u8) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

fn check_note_content(state: &AppState) -> (String, CheckNoteTone) {
    let Some(result) = state.dialogs.drc_results.as_ref() else {
        return (
            "Schematic checks stale · run schematic checks".to_owned(),
            CheckNoteTone::Warning,
        );
    };
    if state.dialogs.drc_checked_version != state.schematic.topology_version() {
        return (
            "Schematic checks stale · run schematic checks".to_owned(),
            CheckNoteTone::Warning,
        );
    }

    let summary = result.summary();
    let blocking = summary.critical + summary.errors;
    if blocking > 0 {
        return (
            format!("{blocking} blocking schematic findings"),
            CheckNoteTone::Error,
        );
    }
    if summary.warnings > 0 {
        return (
            format!("{} schematic advisories", summary.warnings),
            CheckNoteTone::Warning,
        );
    }
    if let Some(run) = historical_annotation_run(state) {
        return (
            format!("Checks current · Run {run} annotations historical"),
            CheckNoteTone::Warning,
        );
    }
    (
        "Checks and annotations current".to_owned(),
        CheckNoteTone::Ok,
    )
}

/// The run whose retained operating point no longer annotates this drawing.
///
/// Canvas annotations fail closed: the cross-probe point map is rejected as
/// soon as it stops matching the open cell and topology, so the drawing is
/// silently unannotated. Checks can be re-run without re-simulating, which
/// would otherwise leave an all-clear note over a schematic that carries no
/// operating point at all.
fn historical_annotation_run(state: &AppState) -> Option<u64> {
    if state.ui.schematic_visibility.annotations
        == crate::state::SchematicAnnotationVisibility::Hidden
    {
        return None;
    }
    let run = state.simulation.active_run()?;
    let solved = state
        .simulation
        .active_analysis()
        .is_some_and(|analysis| analysis.dc_op.is_some());
    let annotates_this_drawing = state.simulation.cross_probe.is_current_for(
        &state.workspace.active_view,
        state.schematic.topology_version(),
    );
    (solved && !annotates_this_drawing).then_some(run.id)
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

    #[test]
    fn canvas_check_note_never_calls_unrun_checks_current() {
        let mut state = AppState::default();
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Warning);

        state.dialogs.drc_results = Some(crate::services::drc::DrcResult::new());
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Ok);

        state.dialogs.drc_checked_version = state.schematic.topology_version().wrapping_sub(1);
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Warning);
    }

    #[test]
    fn canvas_check_note_matches_the_upgraded_mockup_breakpoint() {
        assert_eq!(CANVAS_CHECK_NOTE_MIN_WIDTH, 620.0);
        assert!(!canvas_check_note_visible(619.0));
        assert!(!canvas_check_note_visible(620.0));
        assert!(canvas_check_note_visible(620.01));
    }

    #[test]
    fn canvas_breadcrumb_uses_the_mockup_body_type_size() {
        assert_eq!(CANVAS_BREADCRUMB_FONT_SIZE, 12.0);
    }

    #[test]
    fn a_solved_run_that_no_longer_annotates_the_drawing_is_never_an_all_clear() {
        let mut state = AppState::default();
        state.dialogs.drc_results = Some(crate::services::drc::DrcResult::new());
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        assert_eq!(check_note_content(&state).1, CheckNoteTone::Ok);

        // A run that solved an operating point, whose cross-probe map was
        // never built for this drawing, must read as historical.
        let mut run = crate::state::SimulationRun::new(41);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::DcOp, "OP")
                .with_dc_op(crate::state::DcOpResult::default()),
        );
        state.simulation.runs.insert(0, run);
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let (message, tone) = check_note_content(&state);
        assert_eq!(tone, CheckNoteTone::Warning);
        assert!(message.contains("Run 41"), "message was {message}");
        assert!(message.contains("historical"), "message was {message}");
    }

    #[test]
    fn hierarchy_visibility_changes_only_the_canvas_context_breadcrumb() {
        let mut state = AppState::default();
        state.workspace.hierarchy_stack = vec![
            crate::state::CellViewRef::new("work", "top", "schematic"),
            crate::state::CellViewRef::new("work", "amp", "schematic"),
            crate::state::CellViewRef::new("work", "bias", "schematic"),
        ];
        state.workspace.active_view = crate::state::CellViewRef::new("work", "bias", "schematic");

        state.ui.schematic_visibility.hierarchy =
            crate::state::SchematicHierarchyVisibility::ActiveOnly;
        assert_eq!(
            hierarchy_breadcrumb_segments(&state),
            vec!["bias".to_owned(), "schematic".to_owned()]
        );

        state.ui.schematic_visibility.hierarchy =
            crate::state::SchematicHierarchyVisibility::ActiveAndParent;
        assert_eq!(
            hierarchy_breadcrumb_segments(&state),
            vec!["amp".to_owned(), "bias".to_owned(), "schematic".to_owned()]
        );
    }

    #[test]
    fn imported_deck_with_only_the_pristine_bootstrap_buffer_is_netlist_first() {
        let mut app = RSpiceApp::test_instance();
        assert!(crate::workbench::netlist_workflow::apply_imported_netlist(
            &mut app.state,
            "V1 out 0 1\n.op\n.end\n".to_owned(),
            None,
            "front_end.sp",
        ));

        assert!(is_netlist_first_without_schematic(&app.state));

        app.state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(0, 0),
        );
        assert!(!is_netlist_first_without_schematic(&app.state));
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

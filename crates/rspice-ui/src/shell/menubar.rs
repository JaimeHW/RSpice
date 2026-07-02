//! Application menu bar — brand mark, the application menus, and the active
//! design context on the right.
//!
//! Rendering is design-system styled; every item dispatches into the existing
//! action layer (`common::menu_bar` dispatchers and direct state methods), so
//! behavior matches the previous chrome exactly.

use egui::{Context, Frame, Sense, TopBottomPanel, Ui, vec2};

use crate::common::RSpiceApp;
use crate::common::menu_bar::{FileMenuAction, dispatch_file_menu_action};
use crate::shell::WorkspaceView;
use crate::state::Tool;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::{Density, Direction, Mode};

/// Menu bar height.
pub const MENUBAR_HEIGHT: f32 = 32.0;
const COMPACT_MENUBAR_REQUIRED_WIDTH: f32 = 142.0;
const FULL_MENUBAR_REQUIRED_WIDTH: f32 = 760.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenubarPresentation {
    Full,
    CompactMenu,
}

fn menubar_presentation_for_width(width: f32) -> MenubarPresentation {
    if width < menubar_required_width(MenubarPresentation::Full, true) {
        MenubarPresentation::CompactMenu
    } else {
        MenubarPresentation::Full
    }
}

fn menubar_required_width(presentation: MenubarPresentation, project_label: bool) -> f32 {
    match presentation {
        MenubarPresentation::CompactMenu => {
            COMPACT_MENUBAR_REQUIRED_WIDTH + if project_label { 88.0 } else { 0.0 }
        }
        MenubarPresentation::Full => {
            FULL_MENUBAR_REQUIRED_WIDTH + if project_label { 180.0 } else { 0.0 }
        }
    }
}

/// Render the menu bar panel.
pub fn show(ctx: &Context, app: &mut RSpiceApp) {
    let t = Tokens::get(ctx);
    let c = t.color;

    TopBottomPanel::top("volta.menubar")
        .exact_height(MENUBAR_HEIGHT)
        .frame(Frame::NONE.fill(c.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let panel_rect = ui.max_rect();
            ui.painter().hline(
                panel_rect.x_range(),
                panel_rect.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                match menubar_presentation_for_width(panel_rect.width()) {
                    MenubarPresentation::Full => full_menubar(ui, app),
                    MenubarPresentation::CompactMenu => compact_menubar(ui, app),
                }
            });
        });
}

// ---------------------------------------------------------------------------
// brand + plumbing
// ---------------------------------------------------------------------------

fn brand(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let galley = {
        let mut job = egui::text::LayoutJob::default();
        job.append(
            "RSPICE",
            0.0,
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_1, FontWeight::SemiBold),
                color: c.text,
                extra_letter_spacing: 0.06 * tokens::FS_1,
                ..Default::default()
            },
        );
        ui.fonts_mut(|f| f.layout_job(job))
    };
    let width = 12.0 + galley.size().x + 12.0;
    let (rect, _) = ui.allocate_exact_size(vec2(width, MENUBAR_HEIGHT), Sense::hover());
    let painter = ui.painter();
    painter.galley(
        egui::pos2(rect.left() + 12.0, rect.center().y - galley.size().y * 0.5),
        galley,
        c.text,
    );
}

fn full_menubar(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    brand(ui);

    top_menu(ui, "File", |ui| file_menu(ui, app));
    top_menu(ui, "Edit", |ui| edit_menu(ui, &mut app.state));
    top_menu(ui, "View", |ui| view_menu(ui, &mut app.state));
    top_menu(ui, "Create", |ui| create_menu(ui, &mut app.state));
    top_menu(ui, "Check", |ui| check_menu(ui, &mut app.state));
    top_menu(ui, "Simulate", |ui| simulate_menu(ui, &mut app.state));
    top_menu(ui, "Tools", |ui| tools_menu(ui, &mut app.state));
    top_menu(ui, "Window", |ui| window_menu(ui, &mut app.state));
    top_menu(ui, "Help", |ui| help_menu(ui, app));

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add_space(10.0);
        let label = app.state.workspace.project.display_name().to_owned();
        ui.label(
            egui::RichText::new(label)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.text_faint),
        );
        if let Some(info) = &app.state.license {
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new(&info.tier)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(c.accent),
            );
        }
    });
}

fn compact_menubar(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    brand(ui);
    top_menu(ui, "Menu", |ui| {
        submenu(ui, "File", |ui| file_menu(ui, app));
        submenu(ui, "Edit", |ui| edit_menu(ui, &mut app.state));
        submenu(ui, "View", |ui| view_menu(ui, &mut app.state));
        submenu(ui, "Create", |ui| create_menu(ui, &mut app.state));
        submenu(ui, "Check", |ui| check_menu(ui, &mut app.state));
        submenu(ui, "Simulate", |ui| simulate_menu(ui, &mut app.state));
        submenu(ui, "Tools", |ui| tools_menu(ui, &mut app.state));
        submenu(ui, "Window", |ui| window_menu(ui, &mut app.state));
        submenu(ui, "Help", |ui| help_menu(ui, app));
    });

    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add_space(8.0);
        if let Some(info) = &app.state.license {
            ui.label(
                egui::RichText::new(&info.tier)
                    .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                    .color(c.accent),
            );
        } else if let Some(label) = compact_project_label(
            app.state.workspace.project.display_name(),
            ui.available_width(),
        ) {
            ui.label(
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        }
    });
}

fn compact_project_label(label: &str, available_width: f32) -> Option<String> {
    if available_width < 140.0 {
        return None;
    }
    let label = label.trim();
    if label.is_empty() {
        return None;
    }
    let max_chars = ((available_width / 8.5).floor() as usize).clamp(12, 24);
    if label.chars().count() <= max_chars {
        return Some(label.to_owned());
    }
    let keep = max_chars.saturating_sub(3);
    let mut truncated: String = label.chars().take(keep).collect();
    truncated.push_str("...");
    Some(truncated)
}

/// A top-level menu button with flat, square, full-height styling.
///
/// The derived `Style` is cached per theme: `style_mut` would deep-clone
/// the whole style per menu per frame (×9 menus), so the override is built
/// once and shared as an `Arc`.
fn top_menu(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    use std::sync::Arc;

    thread_local! {
        static MENU_STYLE: std::cell::RefCell<Option<(usize, Arc<egui::Style>)>> =
            const { std::cell::RefCell::new(None) };
    }

    let t = Tokens::get(ui.ctx());
    // The token Arc is replaced on theme change, so its address keys the
    // derived style.
    let theme_key = Arc::as_ptr(&t) as usize;
    let style = MENU_STYLE.with(|cache| {
        let mut cache = cache.borrow_mut();
        match cache.as_ref() {
            Some((key, style)) if *key == theme_key => Arc::clone(style),
            _ => {
                let c = t.color;
                let mut style: egui::Style = (*ui.ctx().style()).clone();
                for w in [
                    &mut style.visuals.widgets.inactive,
                    &mut style.visuals.widgets.hovered,
                    &mut style.visuals.widgets.active,
                    &mut style.visuals.widgets.open,
                ] {
                    w.corner_radius = egui::CornerRadius::ZERO;
                    w.bg_stroke = egui::Stroke::NONE;
                }
                style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                style.visuals.widgets.inactive.fg_stroke.color = c.text_dim;
                style.visuals.widgets.hovered.weak_bg_fill = c.bg_hover;
                style.visuals.widgets.open.weak_bg_fill = c.bg_active;
                style.spacing.button_padding = vec2(10.0, (MENUBAR_HEIGHT - 18.0) * 0.5);
                let style = Arc::new(style);
                *cache = Some((theme_key, Arc::clone(&style)));
                style
            }
        }
    });

    ui.scope(|ui| {
        ui.set_style(style);
        ui.menu_button(title, |ui| {
            ui.set_min_width(230.0);
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            add_contents(ui);
        });
    });
}

/// One menu row: label left, optional shortcut right. Returns `true` when
/// activated (and closes the menu).
pub(crate) fn item(ui: &mut Ui, label: &str, kbd: Option<&str>) -> bool {
    item_impl(ui, label, kbd, true).0
}

/// A non-interactive (disabled) menu row.
pub(crate) fn item_disabled(ui: &mut Ui, label: &str, kbd: Option<&str>) {
    let _ = item_impl(ui, label, kbd, false);
}

/// A disabled menu row with explanatory hover text.
pub(crate) fn item_disabled_with_hint(ui: &mut Ui, label: &str, kbd: Option<&str>, hint: &str) {
    let (_, response) = item_impl(ui, label, kbd, false);
    response.on_hover_text(hint);
}

fn item_impl(ui: &mut Ui, label: &str, kbd: Option<&str>, enabled: bool) -> (bool, egui::Response) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let width = ui.available_width().max(220.0);
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, 26.0),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if !ui.is_rect_visible(rect) {
        return (false, response);
    }

    let painter = ui.painter();
    if enabled && response.hovered() {
        painter.rect_filled(rect, t.radius, c.bg_hover);
    }
    painter.text(
        egui::pos2(rect.left() + 9.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_2, FontWeight::Regular),
        if enabled { c.text } else { c.text_faint },
    );
    if let Some(kbd) = kbd {
        painter.text(
            egui::pos2(rect.right() - 9.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            kbd,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            c.text_faint,
        );
    }

    if enabled && response.clicked() {
        ui.close();
        return (true, response);
    }
    (false, response)
}

/// Hairline separator between item groups.
pub(crate) fn separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 11.0), Sense::hover());
    ui.painter().hline(
        egui::Rangef::new(rect.left() + 4.0, rect.right() - 4.0),
        rect.center().y,
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// A submenu row. egui's stock `menu_button` carries its own widget
/// styling (dim text, different padding/height), which made these rows
/// look disabled next to [`item`] rows — restyle it to the same chrome:
/// 26 px row, FS_2 sans in `text`, `bg_hover` fill, 9 px inset.
fn submenu(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.scope(|ui| {
        let style = ui.style_mut();
        style.spacing.button_padding = vec2(9.0, 0.0);
        style.spacing.interact_size.y = 26.0;
        let corner_radius = egui::CornerRadius::same(t.radius.round() as u8);
        for visuals in [
            &mut style.visuals.widgets.inactive,
            &mut style.visuals.widgets.hovered,
            &mut style.visuals.widgets.active,
            &mut style.visuals.widgets.open,
        ] {
            visuals.fg_stroke.color = c.text;
            visuals.bg_stroke = egui::Stroke::NONE;
            visuals.corner_radius = corner_radius;
        }
        style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
        style.visuals.widgets.hovered.weak_bg_fill = c.bg_hover;
        style.visuals.widgets.active.weak_bg_fill = c.bg_hover;
        style.visuals.widgets.open.weak_bg_fill = c.bg_hover;

        ui.menu_button(
            egui::RichText::new(title).font(theme::sans(tokens::FS_2, FontWeight::Regular)),
            |ui| {
                ui.set_min_width(210.0);
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                add_contents(ui);
            },
        );
    });
}

// ---------------------------------------------------------------------------
// menus
// ---------------------------------------------------------------------------

fn file_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    let mut action: Option<FileMenuAction> = None;
    let mut set = |a: FileMenuAction, picked: bool| {
        if picked {
            action = Some(a);
        }
    };

    set(FileMenuAction::New, item(ui, "New cell…", Some("Ctrl+N")));
    set(FileMenuAction::Open, item(ui, "Open…", Some("Ctrl+O")));
    open_recent_submenu(ui, app);
    set(FileMenuAction::Save, item(ui, "Save", Some("Ctrl+S")));
    set(
        FileMenuAction::SaveAs,
        item(ui, "Save as…", Some("Ctrl+Shift+S")),
    );
    separator(ui);
    set(FileMenuAction::NewProject, item(ui, "New project", None));
    set(FileMenuAction::OpenProject, item(ui, "Open project…", None));
    set(FileMenuAction::SaveProject, item(ui, "Save project", None));
    set(
        FileMenuAction::SaveProjectAs,
        item(ui, "Save project as…", None),
    );
    separator(ui);
    submenu(ui, "Open example", |ui| {
        for example in crate::common::examples::EXAMPLES {
            if item(ui, example.name, None)
                && crate::common::menu_bar::request_load_named_example(&mut app.state, example.name)
                && !app.state.dialogs.confirmation_dialog.visible
            {
                app.state.shell.view = WorkspaceView::Schematic;
            }
        }
    });
    separator(ui);
    set(
        FileMenuAction::ImportVerilogA,
        item(ui, "Import Verilog-A…", None),
    );
    set(
        FileMenuAction::ImportNetlist,
        item(ui, "Import SPICE deck...", None),
    );
    submenu(ui, "Export netlist", |ui| {
        for (label, format) in [
            ("SPICE…", crate::io::NetlistFormat::Spice),
            ("Spectre…", crate::io::NetlistFormat::Spectre),
            ("HSPICE…", crate::io::NetlistFormat::Hspice),
            ("Xyce…", crate::io::NetlistFormat::Xyce),
        ] {
            if item(ui, label, None) {
                crate::common::menu_bar::action_export_netlist_with_io(
                    &mut app.state,
                    format,
                    app.export_workflow_io.as_ref(),
                );
            }
        }
    });
    set(
        FileMenuAction::ExportSvg,
        item(ui, "Export schematic SVG…", None),
    );
    set(
        FileMenuAction::ExportCsvWaveforms,
        item(ui, "Export waveforms CSV…", None),
    );
    separator(ui);
    set(FileMenuAction::Exit, item(ui, "Exit", None));

    if let Some(action) = action {
        dispatch_file_menu_action(
            &mut app.state,
            action,
            app.file_workflow_io.as_ref(),
            app.export_workflow_io.as_ref(),
        );
    }
}

/// File ▸ Open recent — the last 8 opened/saved schematics and projects,
/// most recent first, plus a clear action. Grayed out while the list is
/// empty so the menu shape stays stable.
fn open_recent_submenu(ui: &mut Ui, app: &mut RSpiceApp) {
    if app.state.recent_files.is_empty() {
        item_disabled(ui, "Open recent", None);
        return;
    }

    let recents = app.state.recent_files.clone();
    let mut clicked: Option<crate::common::app::RecentFile> = None;
    let mut clear = false;

    submenu(ui, "Open recent", |ui| {
        for recent in &recents {
            let label = recent
                .path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| recent.path.display().to_string());
            if item(ui, &label, None) {
                clicked = Some(recent.clone());
            }
        }
        separator(ui);
        if item(ui, "Clear recent files", None) {
            clear = true;
        }
    });

    if let Some(recent) = clicked {
        app.open_recent_file(recent);
    }
    if clear {
        app.state.recent_files.clear();
    }
}

fn edit_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    let undo_label = match state.schematic.undo_description() {
        Some(desc) => format!("Undo {desc}"),
        None => "Undo".to_owned(),
    };
    if state.schematic.can_undo() {
        if item(ui, &undo_label, Some("Ctrl+Z")) {
            state.schematic.undo();
        }
    } else {
        item_disabled(ui, "Undo", Some("Ctrl+Z"));
    }
    if state.schematic.can_redo() {
        if item(ui, "Redo", Some("Ctrl+Shift+Z")) {
            state.schematic.redo();
        }
    } else {
        item_disabled(ui, "Redo", Some("Ctrl+Shift+Z"));
    }
    separator(ui);
    let has_selection = !state.schematic.selection.is_empty();
    if has_selection {
        if item(ui, "Cut", Some("Ctrl+X")) {
            state.schematic.copy_selection();
            state.schematic.delete_selection();
        }
        if item(ui, "Copy", Some("Ctrl+C")) {
            state.schematic.copy_selection();
        }
    } else {
        item_disabled(ui, "Cut", Some("Ctrl+X"));
        item_disabled(ui, "Copy", Some("Ctrl+C"));
    }
    if item(ui, "Paste", Some("Ctrl+V")) {
        let anchor = state.schematic_paste_anchor();
        state.schematic.paste_at(anchor);
    }
    if has_selection {
        if item(ui, "Delete", Some("Del")) {
            state.schematic.delete_selection();
        }
    } else {
        item_disabled(ui, "Delete", Some("Del"));
    }
    separator(ui);
    if item(ui, "Select all", Some("Ctrl+A")) {
        let component_ids: Vec<u64> = state.schematic.components.iter().map(|c| c.id).collect();
        let wire_ids: Vec<u64> = state.schematic.wires.iter().map(|w| w.id).collect();
        for id in component_ids {
            state.schematic.selection.select_component(id);
        }
        for id in wire_ids {
            state.schematic.selection.select_wire(id);
        }
    }
}

fn view_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    if item(ui, "Zoom in", Some("Ctrl++")) {
        state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
    }
    if item(ui, "Zoom out", Some("Ctrl+-")) {
        state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
    }
    if item(ui, "Zoom to fit", Some("F")) {
        state.schematic.needs_fit = true;
    }
    if item(ui, "Zoom 100 %", Some("Ctrl+0")) {
        state.schematic.zoom = 1.0;
    }
    separator(ui);
    submenu(ui, "Grid", |ui| {
        for style in crate::shell::GridStyle::ALL {
            let label = if state.shell.grid == style {
                format!("●  {}", style.label())
            } else {
                format!("    {}", style.label())
            };
            if item(ui, &label, None) {
                state.shell.grid = style;
            }
        }
    });
    let console_label = if state.shell.console.collapsed {
        "Show console"
    } else {
        "Hide console"
    };
    if item(ui, console_label, Some("Ctrl+J")) {
        state.shell.console.collapsed = !state.shell.console.collapsed;
    }
    let panels_label = if state.shell.panels_hidden {
        "Show side panels"
    } else {
        "Hide side panels"
    };
    if item(ui, panels_label, Some("Ctrl+B")) {
        state.shell.panels_hidden = !state.shell.panels_hidden;
    }
    separator(ui);
    submenu(ui, "Theme", |ui| {
        for direction in Direction::ALL {
            let label = if state.shell.theme.direction == direction {
                format!("●  {}", direction.label())
            } else {
                format!("    {}", direction.label())
            };
            if item(ui, &label, None) {
                state.shell.theme.direction = direction;
            }
        }
        separator(ui);
        for mode in Mode::ALL {
            let label = if state.shell.theme.mode == mode {
                format!("●  {}", mode.label())
            } else {
                format!("    {}", mode.label())
            };
            if item(ui, &label, None) {
                state.shell.theme.mode = mode;
            }
        }
        separator(ui);
        for density in Density::ALL {
            let label = if state.shell.theme.density == density {
                format!("●  {}", density.label())
            } else {
                format!("    {}", density.label())
            };
            if item(ui, &label, None) {
                state.shell.theme.density = density;
            }
        }
    });
}

fn create_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    if item(ui, "Instance…", Some("Shift+I")) {
        state.shell.view = WorkspaceView::Schematic;
        state.shell.focus_cell_search = true;
    }
    if item(ui, "Wire", Some("W")) {
        state.shell.view = WorkspaceView::Schematic;
        state.schematic.tool = Tool::Wire;
    }
    if item(ui, "Net label", Some("N")) {
        state.shell.view = WorkspaceView::Schematic;
        state.schematic.tool = Tool::Label;
    }
    separator(ui);
    for section in crate::schematic::component_palette() {
        submenu(ui, section.title, |ui| {
            for entry in section.entries {
                if item(ui, entry.label, None) {
                    state.shell.view = WorkspaceView::Schematic;
                    state.schematic.tool = Tool::Place(entry.kind);
                }
            }
        });
    }
}

fn check_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    if item(ui, "Run design checks", Some("Ctrl+E")) {
        if state.workspace.active_view_type() == crate::state::ViewType::Symbol {
            state.run_active_symbol_pin_checks();
        } else {
            crate::common::menu_bar::run_design_rule_check(state);
        }
    }
    separator(ui);
    if state.dialogs.drc_results.is_some() {
        if item(ui, "View violations", None) {
            // Badges live on the canvas, the rows in the console — show both,
            // with the console tab opened on the worst severity present.
            state.shell.view = WorkspaceView::Schematic;
            state.shell.console.collapsed = false;
            state.shell.console.filter = match state.dialogs.drc_results.as_ref() {
                Some(result) if !result.errors().is_empty() => {
                    crate::shell::state::ConsoleFilter::Errors
                }
                Some(result) if !result.warnings().is_empty() => {
                    crate::shell::state::ConsoleFilter::Warnings
                }
                _ => crate::shell::state::ConsoleFilter::All,
            };
        }
        if item(ui, "Clear violations", None) {
            state.dialogs.drc_results = None;
        }
    } else {
        item_disabled(ui, "View violations", None);
        item_disabled(ui, "Clear violations", None);
    }
}

fn simulate_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    let run_block_reason = state.simulation_run_block_reason();
    if run_block_reason.is_none() {
        if item(ui, "Run", Some("F5")) {
            state.request_run_set_simulation();
            state.shell.view = WorkspaceView::Simulate;
        }
    } else if let Some(reason) = run_block_reason.as_deref() {
        item_disabled_with_hint(ui, "Run", Some("F5"), reason);
    }
    if state.simulation.is_running {
        if item(ui, "Stop", Some("Shift+F5")) {
            state.simulation.trigger_abort = true;
        }
    } else {
        item_disabled(ui, "Stop", Some("Shift+F5"));
    }
    separator(ui);
    if item(ui, "Edit analyses…", None) {
        state.shell.view = WorkspaceView::Simulate;
    }
    if item(ui, "Simulation options…", None) {
        crate::common::menu_bar::open_simulation_options(state);
    }
    if item(ui, "Generate netlist", None) {
        crate::common::menu_bar::action_view_netlist(state);
        state.shell.view = WorkspaceView::Netlist;
    }
    separator(ui);
    if state.simulation.has_results() {
        if item(ui, "Clear results", None) {
            state.clear_simulation_results();
        }
    } else {
        item_disabled(ui, "Clear results", None);
    }
}

fn tools_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    if item(ui, "Waveform calculator", None) {
        state.dialogs.waveform_calculator_dialog = true;
    }
    if item(ui, "Model browser", None) {
        state.model_browser_state.open = true;
    }
    if item(ui, "Library path editor…", None) {
        state.pdk_settings_dialog.open(state.pdk_config.clone());
    }
    if item(ui, "Automation console", None) {
        state.shell.console.collapsed = false;
        state.shell.console.filter = crate::shell::state::ConsoleFilter::Script;
    }
    separator(ui);
    if item(ui, "Command palette", Some("Ctrl+K")) {
        state.dialogs.command_palette.open();
    }
    if item(ui, "Preferences…", Some("Ctrl+,")) {
        state.dialogs.preferences_open = true;
    }
    separator(ui);
    if item(ui, "Compile Verilog-A…", None) {
        state.dialogs.veriloga_dialog.open();
    }
    submenu(ui, "Verilog-A cache", |ui| {
        if item(ui, "Status", None) {
            crate::common::menu_bar::action_veriloga_cache_status(state);
        }
        if item(ui, "List entries", None) {
            crate::common::menu_bar::action_veriloga_cache_list_entries(state);
        }
        if item(ui, "Recompile library", None) {
            crate::common::menu_bar::action_veriloga_recompile_library(state);
        }
        separator(ui);
        if item(ui, "Prune stale entries", None) {
            crate::common::menu_bar::action_veriloga_cache_prune(state);
        }
        if item(ui, "Clear cache", None) {
            crate::common::menu_bar::action_veriloga_cache_clear(state);
        }
    });
}

fn window_menu(ui: &mut Ui, state: &mut crate::common::AppState) {
    if item(ui, "Next workspace tab", Some("Ctrl+Tab")) {
        state.shell.cycle_view();
    }
    let console_label = if state.shell.console.collapsed {
        "Show console"
    } else {
        "Hide console"
    };
    if item(ui, console_label, Some("Ctrl+J")) {
        state.shell.console.collapsed = !state.shell.console.collapsed;
    }
    separator(ui);
    if item(ui, "Reset layout", None) {
        state.shell.reset_layout();
    }
}

fn help_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    submenu(ui, "Documentation", |ui| {
        for (title, path) in crate::common::menu_bar::DOC_REFERENCES {
            if item(ui, title, None) {
                crate::common::menu_bar::open_documentation_reference(&mut app.state, title, path);
            }
        }
    });
    if item(ui, "Keyboard shortcuts", Some("F1")) {
        app.state.dialogs.shortcuts_help = true;
    }
    separator(ui);
    let license_label = match &app.state.license {
        Some(info) => format!("License — {} until {}", info.tier, info.updates_until),
        None => "Enter license key…".to_owned(),
    };
    if item(ui, &license_label, None) {
        app.open_license_dialog();
    }
    separator(ui);
    if item(ui, "About RSpice", None) {
        app.state.dialogs.about = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_width_menubar_uses_compact_menu_that_fits() {
        let presentation = menubar_presentation_for_width(390.0);

        assert_eq!(presentation, MenubarPresentation::CompactMenu);
        assert!(
            menubar_required_width(presentation, true) <= 390.0,
            "compact menu bar should fit a phone viewport"
        );
    }

    #[test]
    fn desktop_menubar_keeps_top_level_menus_and_project_label() {
        let presentation = menubar_presentation_for_width(1280.0);

        assert_eq!(presentation, MenubarPresentation::Full);
    }

    #[test]
    fn compact_project_label_is_bounded_for_long_names() {
        let label =
            compact_project_label("precision_frontend_with_long_corner_sweep_project", 220.0)
                .expect("label has enough room");

        assert!(label.chars().count() <= 24);
        assert!(label.ends_with("..."));
    }

    #[test]
    fn menubar_stays_compact_until_full_estimated_width_fits() {
        let full_required = menubar_required_width(MenubarPresentation::Full, true);

        assert_eq!(
            menubar_presentation_for_width(full_required - 1.0),
            MenubarPresentation::CompactMenu
        );
        assert_eq!(
            menubar_presentation_for_width(full_required),
            MenubarPresentation::Full
        );
    }
}

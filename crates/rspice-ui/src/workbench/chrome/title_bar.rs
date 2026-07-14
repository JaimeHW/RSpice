//! Application title bar and complete implemented menu taxonomy.

use egui::{Align, Context, Frame, Layout, Sense, TopBottomPanel, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::{TITLE_BAR_H, WorkbenchIcon};
use super::super::layout::LayoutSpec;
use super::super::state::{ModelsPage, VerificationPage, Workspace};

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::top("workbench.title_bar")
        .exact_height(TITLE_BAR_H)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                brand(ui, app);
                if layout.show_title_menus {
                    menus(ui, app);
                } else {
                    compact_menu(ui, app);
                }

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(6.0);
                    account_button(ui);
                    notification_button(ui);
                    if icon_action(ui, WorkbenchIcon::Settings, "Open preferences") {
                        Command::Preferences.execute(app);
                    }
                    if search_button(ui, layout, app) {
                        Command::CommandPalette.execute(app);
                    }
                    if ui.available_width() > 190.0 {
                        active_project(ui, app);
                    }
                });
            });
        });
}

fn brand(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let show_wordmark = ui.available_width() > 520.0;
    let width = if show_wordmark { 86.0 } else { 38.0 };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, TITLE_BAR_H), Sense::click());
    WorkbenchIcon::Brand.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 18.0, rect.center().y),
            Vec2::splat(22.0),
        ),
        t.color.accent,
    );
    if show_wordmark {
        ui.painter().text(
            egui::Pos2::new(rect.left() + 34.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "RSpice",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
    }
    if response.clicked() {
        Command::ProjectLauncher.execute(app);
    }
    response.on_hover_text("Open Project workspace");
}

fn menus(ui: &mut Ui, app: &mut RSpiceApp) {
    top_menu(ui, "File", |ui| file_menu(ui, app));
    top_menu(ui, "Edit", |ui| edit_menu(ui, app));
    top_menu(ui, "View", |ui| view_menu(ui, app));
    top_menu(ui, "Design", |ui| design_menu(ui, app));
    top_menu(ui, "Simulate", |ui| simulate_menu(ui, app));
    top_menu(ui, "Results", |ui| results_menu(ui, app));
    top_menu(ui, "Verify", |ui| verify_menu(ui, app));
    top_menu(ui, "Models", |ui| models_menu(ui, app));
    top_menu(ui, "Automation", |ui| automation_menu(ui, app));
    top_menu(ui, "Window", |ui| window_menu(ui, app));
    top_menu(ui, "Help", |ui| help_menu(ui, app));
}

fn compact_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    top_menu(ui, "Menu", |ui| {
        submenu(ui, "File", |ui| file_menu(ui, app));
        submenu(ui, "Edit", |ui| edit_menu(ui, app));
        submenu(ui, "View", |ui| view_menu(ui, app));
        submenu(ui, "Design", |ui| design_menu(ui, app));
        submenu(ui, "Simulate", |ui| simulate_menu(ui, app));
        submenu(ui, "Results", |ui| results_menu(ui, app));
        submenu(ui, "Verify", |ui| verify_menu(ui, app));
        submenu(ui, "Models", |ui| models_menu(ui, app));
        submenu(ui, "Automation", |ui| automation_menu(ui, app));
        submenu(ui, "Window", |ui| window_menu(ui, app));
        submenu(ui, "Help", |ui| help_menu(ui, app));
    });
}

fn top_menu(ui: &mut Ui, label: &str, contents: impl FnOnce(&mut Ui)) {
    ui.menu_button(
        egui::RichText::new(label).font(theme::sans(tokens::FS_1, FontWeight::Regular)),
        |ui| {
            ui.set_min_width(286.0);
            contents(ui);
        },
    );
}

fn submenu(ui: &mut Ui, label: &str, contents: impl FnOnce(&mut Ui)) {
    ui.menu_button(label, |ui| {
        ui.set_min_width(286.0);
        contents(ui);
    });
}

fn command_item(ui: &mut Ui, app: &mut RSpiceApp, command: Command) {
    let spec = command.spec();
    command_item_as(ui, app, command, spec.label);
}

fn command_item_as(ui: &mut Ui, app: &mut RSpiceApp, command: Command, label: &str) {
    let enabled = command.is_enabled(app);
    let shortcut = command.spec().shortcut;
    let text = if shortcut.is_empty() {
        label.to_owned()
    } else {
        format!("{label}\t{shortcut}")
    };
    if ui
        .add_enabled(enabled, egui::Button::new(text).frame(false))
        .clicked()
    {
        command.execute(app);
        ui.close();
    }
}

fn file_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ProjectLauncher);
    ui.separator();
    command_item(ui, app, Command::OpenProject);
    command_item(ui, app, Command::NewProject);
    command_item(ui, app, Command::Save);
    command_item(ui, app, Command::SaveAs);
    ui.separator();
    command_item(ui, app, Command::NewCell);
    command_item(ui, app, Command::OpenDocument);
    submenu(ui, "Open example", |ui| {
        for example in crate::common::examples::EXAMPLES {
            if ui.button(example.name).clicked() {
                if crate::common::menu_bar::request_load_named_example(&mut app.state, example.name)
                    && !app.state.dialogs.confirmation_dialog.visible
                {
                    Command::OpenWorkspace(Workspace::Design).execute(app);
                }
                ui.close();
            }
        }
    });
    ui.separator();
    command_item(ui, app, Command::ImportNetlist);
    command_item(ui, app, Command::ImportVerilogA);
    submenu(ui, "Export netlist", |ui| {
        for (label, format) in [
            ("SPICE…", crate::io::NetlistFormat::Spice),
            ("Spectre…", crate::io::NetlistFormat::Spectre),
            ("HSPICE…", crate::io::NetlistFormat::Hspice),
            ("Xyce…", crate::io::NetlistFormat::Xyce),
        ] {
            command_item_as(ui, app, Command::ExportNetlist(format), label);
        }
    });
    command_item(ui, app, Command::ExportSchematicSvg);
    command_item(ui, app, Command::ExportWaveformsCsv);
    ui.separator();
    command_item(ui, app, Command::Exit);
}

fn edit_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in [Command::Undo, Command::Redo] {
        command_item(ui, app, command);
    }
    ui.separator();
    for command in [
        Command::Cut,
        Command::Copy,
        Command::Paste,
        Command::Duplicate,
        Command::Delete,
        Command::SelectAll,
    ] {
        command_item(ui, app, command);
    }
    ui.separator();
    command_item(ui, app, Command::ObjectProperties);
    command_item(ui, app, Command::FindInDesign);
    ui.separator();
    command_item(ui, app, Command::Preferences);
}

fn view_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in [
        Command::ZoomIn,
        Command::ZoomOut,
        Command::ZoomFit,
        Command::ZoomOneToOne,
        Command::CycleGrid,
    ] {
        command_item(ui, app, command);
    }
    ui.separator();
    command_item(ui, app, Command::ToggleFocusMode);
    command_item(ui, app, Command::ResetLayout);
}

fn design_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::OpenWorkspace(Workspace::Design));
    command_item(ui, app, Command::AscendHierarchy);
    command_item(ui, app, Command::DescendHierarchy);
    ui.separator();
    for command in [
        Command::PlaceInstance,
        Command::PlaceWire,
        Command::PlaceLabel,
        Command::PlaceProbe,
    ] {
        command_item(ui, app, command);
    }
    submenu(ui, "Place component", |ui| {
        for section in crate::schematic::component_palette() {
            submenu(ui, section.title, |ui| {
                for entry in section.entries {
                    command_item_as(ui, app, Command::Place(entry.kind), entry.label);
                }
            });
        }
    });
    ui.separator();
    command_item(ui, app, Command::RotateSelection);
    command_item(ui, app, Command::MirrorSelectionHorizontal);
    ui.separator();
    command_item(ui, app, Command::RunChecks);
    command_item(ui, app, Command::ClearChecks);
}

fn simulate_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::RunSimulation);
    command_item(ui, app, Command::StopSimulation);
    ui.separator();
    command_item(ui, app, Command::OpenWorkspace(Workspace::Simulate));
    command_item(ui, app, Command::SimulationOptions);
    command_item(ui, app, Command::GenerateNetlist);
}

fn results_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::OpenWorkspace(Workspace::Results));
    ui.separator();
    for (label, viewer) in [
        ("Waveforms", crate::shell::ResultViewer::Waves),
        ("Bode and stability", crate::shell::ResultViewer::Bode),
        ("Spectrum / FFT", crate::shell::ResultViewer::Fft),
        ("Eye diagram", crate::shell::ResultViewer::Eye),
        ("Histogram", crate::shell::ResultViewer::Hist),
        ("Operating point", crate::shell::ResultViewer::Op),
        (
            "Noise contributors",
            crate::shell::ResultViewer::NoiseContrib,
        ),
        ("Specification matrix", crate::shell::ResultViewer::Specs),
        ("Nyquist", crate::shell::ResultViewer::Nyquist),
        ("Smith chart", crate::shell::ResultViewer::Smith),
        ("Pole-zero", crate::shell::ResultViewer::PoleZero),
    ] {
        command_item_as(ui, app, Command::ResultViewer(viewer), label);
    }
    ui.separator();
    command_item(ui, app, Command::WaveformCalculator);
    command_item(ui, app, Command::ExportWaveformsCsv);
    command_item(ui, app, Command::ClearResults);
}

fn verify_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::OpenWorkspace(Workspace::Verify));
    ui.separator();
    for page in VerificationPage::ALL {
        command_item_as(ui, app, Command::VerificationPage(page), page.label());
    }
    ui.separator();
    command_item(ui, app, Command::RunChecks);
    command_item(ui, app, Command::EditSpecifications);
}

fn models_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::OpenWorkspace(Workspace::Models));
    ui.separator();
    for page in ModelsPage::ALL {
        command_item_as(ui, app, Command::ModelsPage(page), page.label());
    }
    ui.separator();
    command_item(ui, app, Command::ModelBrowser);
    command_item(ui, app, Command::PdkSettings);
    command_item(ui, app, Command::CompileVerilogA);
}

fn automation_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::OpenWorkspace(Workspace::Netlist));
    command_item(ui, app, Command::AutomationConsole);
    command_item(ui, app, Command::ImportNetlist);
    command_item(ui, app, Command::GenerateNetlist);
}

fn window_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ToggleNavigator);
    command_item(ui, app, Command::ToggleInspector);
    command_item(ui, app, Command::ToggleConsole);
    command_item(ui, app, Command::ToggleFocusMode);
    ui.separator();
    command_item(ui, app, Command::PreviousWorkspace);
    command_item(ui, app, Command::NextWorkspace);
    ui.separator();
    command_item(ui, app, Command::ResetLayout);
}

fn help_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    submenu(ui, "Documentation", |ui| {
        for (title, path) in crate::common::menu_bar::DOC_REFERENCES {
            command_item_as(ui, app, Command::Documentation(title, path), title);
        }
    });
    command_item(ui, app, Command::KeyboardShortcuts);
    command_item(ui, app, Command::License);
    ui.separator();
    command_item(ui, app, Command::About);
}

fn active_project(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        let (dot, _) = ui.allocate_exact_size(Vec2::splat(8.0), Sense::hover());
        ui.painter().circle_filled(
            dot.center(),
            3.0,
            if app.state.schematic.is_dirty || app.state.workspace.any_dirty() {
                t.color.warn
            } else {
                t.color.ok
            },
        );
        ui.label(
            egui::RichText::new(app.state.workspace.project.display_name())
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text),
        );
    });
}

fn search_button(ui: &mut Ui, layout: LayoutSpec, app: &RSpiceApp) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = if matches!(layout.width_class, super::super::state::WidthClass::Wide) {
        246.0
    } else if layout.show_title_menus {
        170.0
    } else {
        38.0
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 26.0), Sense::click());
    ui.painter().rect_filled(rect, t.radius, t.color.bg_inset);
    ui.painter().rect_stroke(
        rect,
        t.radius,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Search.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 15.0, rect.center().y),
            Vec2::splat(14.0),
        ),
        t.color.text_dim,
    );
    if width > 60.0 {
        ui.painter().text(
            egui::Pos2::new(rect.left() + 29.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Search or run a command",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        ui.painter().text(
            egui::Pos2::new(rect.right() - 7.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            "Ctrl K",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    let _ = app;
    response
        .on_hover_text("Search and run a command (Ctrl+K)")
        .clicked()
}

fn icon_action(ui: &mut Ui, icon: WorkbenchIcon, label: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::click());
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    icon.paint(ui.painter(), rect.shrink(6.0), t.color.text_dim);
    response.on_hover_text(label).clicked()
}

fn notification_button(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
    WorkbenchIcon::Bell.paint(ui.painter(), rect.shrink(6.0), t.color.text_dim);
    ui.painter().circle_filled(
        rect.right_top() + egui::vec2(-5.0, 5.0),
        5.0,
        t.color.accent,
    );
    ui.painter().text(
        rect.right_top() + egui::vec2(-5.0, 5.0),
        egui::Align2::CENTER_CENTER,
        "2",
        theme::mono(8.0, FontWeight::SemiBold),
        t.color.accent_ink,
    );
    response.on_hover_text("Notifications and activity · 2 unread");
}

fn account_button(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
    ui.painter().circle_stroke(
        rect.center(),
        11.0,
        egui::Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "JM",
        theme::mono(8.5, FontWeight::SemiBold),
        t.color.text_dim,
    );
    response.on_hover_text("Account and organization");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_menu_commands_are_all_real_dispatch_commands() {
        let commands = [
            Command::OpenProject,
            Command::Save,
            Command::Undo,
            Command::PlaceWire,
            Command::RunSimulation,
            Command::WaveformCalculator,
            Command::PdkSettings,
            Command::AutomationConsole,
        ];
        assert!(commands.iter().all(|command| !command.spec().id.is_empty()));
    }
}

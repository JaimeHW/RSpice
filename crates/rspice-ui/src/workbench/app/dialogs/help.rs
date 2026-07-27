//! Help dialogs on the modal primitive: About, keyboard shortcuts, and the
//! waveform calculator host.

use egui::{Context, Grid, RichText, ScrollArea};

use crate::workbench::app::{HelpCenterPage, RSpiceApp, session::shortcuts::ShortcutCategory};
use crate::workbench::export_workflow::SaveDialogConfig;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row};
use crate::workbench::ShortcutPreferences;
use crate::workbench::commands::{Command, CommandPlatform};

const PRODUCT_LICENSE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../LICENSE"));

#[derive(Debug, Clone, Copy)]
enum HelpAction {
    Command(Command),
}

fn shortcut_help_row(
    shortcuts: &ShortcutPreferences,
    command: Command,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> (String, &'static str) {
    let shortcut = shortcuts.resolved_label(command, platform, operating_system);
    (
        if shortcut.is_empty() {
            "\u{2014}".to_owned()
        } else {
            shortcut
        },
        command.spec().label,
    )
}

fn shortcut_row_matches(
    shortcuts: &ShortcutPreferences,
    command: Command,
    filter: &str,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> bool {
    filter.is_empty()
        || command.spec().label.to_lowercase().contains(filter)
        || command.stable_id().contains(filter)
        || shortcuts
            .resolved_label(command, platform, operating_system)
            .to_lowercase()
            .contains(filter)
}

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_help_center_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.help_center.open {
            return;
        }

        let mut page = self.state.dialogs.help_center.page;
        let mut query = self.state.dialogs.help_center.query.clone();
        let mut include_session_log = self.state.dialogs.help_center.include_session_log;
        let mut disclosure_reviewed = self.state.dialogs.help_center.disclosure_reviewed;
        let mut action = None;

        let mut dialog = Dialog::new(
            "HELP \u{00b7} VERSIONED \u{00b7} PLATFORM AWARE",
            page.label(),
            "Close",
        )
        .description(
            "Browse product guidance, compatibility changes, migration requirements, diagnostics, support disclosure, and notices.",
        )
        .size(DialogSize::Manager);
        if page == HelpCenterPage::SupportBundle {
            dialog = dialog.ghost("Create support bundle\u{2026}");
        }
        let choice = dialog.show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for candidate in HelpCenterPage::ALL {
                    if ui
                        .selectable_label(page == candidate, candidate.label())
                        .clicked()
                    {
                        page = candidate;
                    }
                }
            });
            ui.separator();
            match page {
                HelpCenterPage::Help => render_help_page(ui, &mut query, &mut action),
                HelpCenterPage::ReleaseNotes => render_release_notes(ui),
                HelpCenterPage::MigrationGuide => render_migration_guide(ui),
                HelpCenterPage::Diagnostics => render_diagnostics(ui, self),
                HelpCenterPage::SupportBundle => render_support_bundle(
                    ui,
                    self,
                    &mut include_session_log,
                    &mut disclosure_reviewed,
                ),
                HelpCenterPage::LegalPrivacy => render_legal_privacy(ui),
            }
        });

        self.state.dialogs.help_center.page = page;
        self.state.dialogs.help_center.query = query;
        self.state.dialogs.help_center.include_session_log = include_session_log;
        self.state.dialogs.help_center.disclosure_reviewed = disclosure_reviewed;

        match choice {
            DialogChoice::None => {}
            DialogChoice::Ghost if page == HelpCenterPage::SupportBundle => {
                if disclosure_reviewed {
                    export_support_bundle(self, ctx, include_session_log);
                } else {
                    self.state.ui.toasts.warn_with_title(
                        ctx,
                        "Disclosure review required",
                        "Review and acknowledge the exact support-bundle contents before export.",
                    );
                }
            }
            _ => {
                self.state.dialogs.help_center.close();
                return;
            }
        }

        if let Some(HelpAction::Command(command)) = action {
            self.state.dialogs.help_center.close();
            command.execute(self);
        }
    }

    pub(in crate::workbench::app) fn render_about_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.about {
            return;
        }

        let license_line = match &self.state.license {
            Some(info) => format!("{} · until {}", info.tier, info.updates_until),
            None => "—".to_owned(),
        };

        let license_for_body = license_line.clone();
        let choice = Dialog::new("RSpice", "About", "Close")
            .description(
                "Review the RSpice version, engine, matrix package, and local license status, or copy diagnostics.",
            )
            .size(DialogSize::Transaction)
            .ghost("Copy diagnostics")
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                let c = t.color;
                ui.label(
                    egui::RichText::new("RSpice")
                        .font(theme::sans(tokens::FS_4, FontWeight::SemiBold))
                        .color(c.text),
                );
                ui.label(
                    egui::RichText::new("Analog circuit design and simulation")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_dim),
                );
                ui.add_space(10.0);
                ui.spacing_mut().item_spacing.y = 0.0;
                kv_row(
                    ui,
                    "Version",
                    concat!(env!("CARGO_PKG_VERSION"), " · ", env!("RSPICE_BUILD_HASH")),
                );
                kv_row(
                    ui,
                    "Engine",
                    concat!("rspice-core ", env!("CARGO_PKG_VERSION")),
                );
                kv_row(ui, "Matrix pkg", "faer sparse");
                kv_row(ui, "License", &license_for_body);
            });

        match choice {
            DialogChoice::None => {}
            DialogChoice::Ghost => {
                // The first thing support asks for, one click to produce.
                let diagnostics = format!(
                    "RSpice {} ({}) · rspice-core {} · faer sparse · license {} · {}-{}",
                    env!("CARGO_PKG_VERSION"),
                    env!("RSPICE_BUILD_HASH"),
                    env!("CARGO_PKG_VERSION"),
                    license_line,
                    std::env::consts::OS,
                    std::env::consts::ARCH,
                );
                ctx.copy_text(diagnostics);
                self.state.ui.toasts.success(
                    ctx,
                    "Diagnostics copied",
                    "Application and platform diagnostics were copied to the clipboard.",
                );
            }
            _ => self.state.dialogs.about = false,
        }
    }

    pub(in crate::workbench::app) fn render_shortcuts_help_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.shortcuts_help {
            return;
        }

        // Count against the filter as entered last frame — immediate mode
        // corrects the hint one frame after each keystroke.
        let shortcuts = self.state.ui.preferences.shortcuts().clone();
        let platform = crate::workbench::app::runtime_command_platform(ctx);
        let operating_system = ctx.os();
        let filter = self.state.dialogs.shortcuts_filter.trim().to_lowercase();
        let total: usize = ShortcutCategory::ALL
            .iter()
            .flat_map(|category| category.commands())
            .count();
        let shown: usize = ShortcutCategory::ALL
            .iter()
            .flat_map(|c| c.commands())
            .filter(|c| shortcut_row_matches(&shortcuts, *c, &filter, platform, operating_system))
            .count();
        let hint = format!("{shown} of {total} commands");

        let filter_text = &mut self.state.dialogs.shortcuts_filter;
        let choice = Dialog::new("Help", "Command reference", "Close")
            .description(
                "Search every discoverable command by name, stable identity, category, or shortcut.",
            )
            .size(DialogSize::Manager)
            .hint(&hint)
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                ui.add(
                    egui::TextEdit::singleline(filter_text)
                        .hint_text("Filter — try 'project' or 'zoom'")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .desired_width(f32::INFINITY),
                );
                let filter = filter_text.trim().to_lowercase();

                ui.spacing_mut().item_spacing.y = 0.0;
                let mut any_shown = false;

                for category in ShortcutCategory::ALL {
                    let commands: Vec<_> = category
                        .commands()
                        .filter(|c| {
                            shortcut_row_matches(
                                &shortcuts,
                                *c,
                                &filter,
                                platform,
                                operating_system,
                            )
                        })
                        .collect();
                    if commands.is_empty() {
                        continue;
                    }
                    any_shown = true;

                    ui.add_space(8.0);
                    let mut header = egui::text::LayoutJob::default();
                    header.append(
                        &category.display_name().to_uppercase(),
                        0.0,
                        egui::TextFormat {
                            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                            color: c.text_faint,
                            extra_letter_spacing: 0.08 * tokens::FS_0,
                            ..Default::default()
                        },
                    );
                    ui.label(header);
                    ui.add_space(2.0);

                    for command in commands {
                        let (shortcut, display_name) =
                            shortcut_help_row(&shortcuts, command, platform, operating_system);
                        // Description left, key chord right in mono — the
                        // kv pattern with the value rendered as a key cap.
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(ui.available_width(), 24.0),
                            egui::Sense::hover(),
                        );
                        let painter = ui.painter();
                        painter.text(
                            egui::pos2(rect.left(), rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            display_name,
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        let key_galley = ui.fonts_mut(|f| {
                            f.layout_no_wrap(
                                shortcut.to_owned(),
                                theme::mono(tokens::FS_0, FontWeight::Regular),
                                c.text,
                            )
                        });
                        let pad = egui::vec2(7.0, 3.0);
                        let key_rect = egui::Rect::from_min_size(
                            egui::pos2(
                                rect.right() - key_galley.size().x - 2.0 * pad.x,
                                rect.center().y - key_galley.size().y * 0.5 - pad.y,
                            ),
                            key_galley.size() + 2.0 * pad,
                        );
                        painter.rect(
                            key_rect,
                            t.radius,
                            c.bg_inset,
                            egui::Stroke::new(1.0, c.border_strong),
                            egui::StrokeKind::Inside,
                        );
                        painter.galley(key_rect.min + pad, key_galley, c.text);
                    }
                }

                if !any_shown {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("nothing matches — clear the filter")
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                }
            });

        if choice != DialogChoice::None {
            self.state.dialogs.shortcuts_help = false;
            self.state.dialogs.shortcuts_filter.clear();
        }
    }
}

fn render_help_page(ui: &mut egui::Ui, query: &mut String, action: &mut Option<HelpAction>) {
    ui.add(
        egui::TextEdit::singleline(query)
            .hint_text("Search tasks, commands, analysis, errors, or compatibility")
            .desired_width(f32::INFINITY),
    );
    ui.add_space(8.0);
    let filter = query.trim().to_ascii_lowercase();
    let topics = [
        (
            "Schematic authoring",
            "Place and edit components, wires, buses, ports, hierarchy, annotations, and design checks.",
        ),
        (
            "Simulation setup",
            "Configure analyses, dependencies, outputs, PVT, models, solvers, preflight, and execution.",
        ),
        (
            "Results and measurements",
            "Inspect retained datasets, plots, cursors, expressions, measurements, comparisons, and exports.",
        ),
        (
            "Project recovery",
            "Review accepted baselines, autosave checkpoints, project revisions, and non-destructive recovery.",
        ),
        (
            "Automation and netlists",
            "Open owned or generated decks, compile Verilog-A, validate sources, and run reproducible workflows.",
        ),
    ];
    for (title, detail) in topics {
        if !filter.is_empty()
            && !title.to_ascii_lowercase().contains(&filter)
            && !detail.to_ascii_lowercase().contains(&filter)
        {
            continue;
        }
        ui.group(|ui| {
            ui.strong(title);
            ui.label(detail);
        });
    }
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        if ui.button("Command reference").clicked() {
            *action = Some(HelpAction::Command(Command::KeyboardShortcuts));
        }
        if ui.button("Platform capability matrix").clicked() {
            *action = Some(HelpAction::Command(Command::FeatureAvailability));
        }
        if ui.button("Interoperability matrix").clicked() {
            *action = Some(HelpAction::Command(Command::InteroperabilityMatrix));
        }
    });
}

fn render_release_notes(ui: &mut egui::Ui) {
    section_heading(ui, "CURRENT BUILD");
    Grid::new("help.release-build")
        .num_columns(2)
        .spacing([24.0, 7.0])
        .show(ui, |ui| {
            ui.label("RSpice");
            ui.label(env!("CARGO_PKG_VERSION"));
            ui.end_row();
            ui.label("Build");
            ui.label(env!("RSPICE_BUILD_HASH"));
            ui.end_row();
            ui.label("Target");
            ui.label(format!(
                "{} / {}",
                std::env::consts::OS,
                std::env::consts::ARCH
            ));
            ui.end_row();
        });
    ui.add_space(12.0);
    section_heading(ui, "COMPATIBILITY CHANGES");
    ui.label(
        "\u{2022} Project execution contexts are migrated through schema 9 before they can authorize simulation.",
    );
    ui.label(
        "\u{2022} Project descriptors, technology bindings, model-bound symbols, and source registries retain explicit schema identities.",
    );
    ui.label(
        "\u{2022} Unsupported future schemas fail closed; they are never silently rewritten by this build.",
    );
    ui.label(
        "\u{2022} Browser and native persistence bindings remain separate authorities and cannot be inferred from a pathname alone.",
    );
}

fn render_migration_guide(ui: &mut egui::Ui) {
    section_heading(ui, "SUPPORTED PROJECT CONTRACTS");
    Grid::new("help.migration-contracts")
        .num_columns(2)
        .spacing([24.0, 7.0])
        .show(ui, |ui| {
            ui.label("Project descriptor");
            ui.label(crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION.to_string());
            ui.end_row();
            ui.label("Technology binding");
            ui.label(crate::state::PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION.to_string());
            ui.end_row();
            ui.label("Execution context");
            ui.label(crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION.to_string());
            ui.end_row();
            ui.label("Model-bound symbol");
            ui.label(crate::state::MODEL_BOUND_SYMBOL_SCHEMA_VERSION.to_string());
            ui.end_row();
            ui.label("Project source registry");
            ui.label(crate::state::PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION.to_string());
            ui.end_row();
        });
    ui.add_space(12.0);
    section_heading(ui, "SAFE MIGRATION WORKFLOW");
    ui.label("1. Preserve the original project and all externally referenced model/source files.");
    ui.label("2. Open a project copy; RSpice validates every supported schema before mutation.");
    ui.label("3. Review migration diagnostics, source bindings, model revisions, and analysis dependencies.");
    ui.label("4. Run schematic checks and simulation preflight against the migrated copy.");
    ui.label("5. Save only after the accepted baseline and generated artifacts match the intended project.");
}

fn render_diagnostics(ui: &mut egui::Ui, app: &RSpiceApp) {
    let config = dirs::config_dir()
        .map(|path| path.join("rspice").display().to_string())
        .unwrap_or_else(|| "Host configuration directory unavailable".to_owned());
    section_heading(ui, "SYSTEM DIAGNOSTICS");
    Grid::new("help.system-diagnostics")
        .num_columns(2)
        .spacing([24.0, 7.0])
        .show(ui, |ui| {
            ui.label("Version");
            ui.label(concat!(
                env!("CARGO_PKG_VERSION"),
                " / ",
                env!("RSPICE_BUILD_HASH")
            ));
            ui.end_row();
            ui.label("Platform");
            ui.label(format!(
                "{} / {}",
                std::env::consts::OS,
                std::env::consts::ARCH
            ));
            ui.end_row();
            ui.label("Host");
            ui.label(if cfg!(target_arch = "wasm32") {
                "Browser / WebAssembly"
            } else {
                "Native desktop / wgpu"
            });
            ui.end_row();
            ui.label("Project");
            ui.label(if app.state.project_lifecycle.project_open {
                app.state.workspace.project.display_name()
            } else {
                "No project open"
            });
            ui.end_row();
            ui.label("Workspace");
            ui.label(app.state.workbench.workspace.label());
            ui.end_row();
            ui.label("Session log");
            ui.label(format!(
                "{} entries in the in-memory console ring buffer",
                app.state.log_buffer.len()
            ));
            ui.end_row();
            ui.label("Configuration");
            ui.label(config);
            ui.end_row();
            ui.label("Persistent log file");
            ui.label("Not configured; native logging is emitted to the launch host");
            ui.end_row();
        });
}

fn render_support_bundle(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    include_session_log: &mut bool,
    disclosure_reviewed: &mut bool,
) {
    section_heading(ui, "DISCLOSURE PREVIEW");
    ui.label("Always included:");
    ui.label(
        "\u{2022} product version, build identity, operating system, architecture, and host class",
    );
    ui.label("\u{2022} active workspace, project-open state, document counts, and schema versions");
    ui.checkbox(
        include_session_log,
        format!(
            "Include {} session-log entries after home-path redaction",
            app.state.log_buffer.len()
        ),
    );
    ui.add_space(8.0);
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(
            "Never included: project files, schematic/netlist/model contents, waveforms, license keys, authentication material, clipboard contents, or autosave bytes.",
        )
        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
        .color(t.color.text_dim),
    );
    ui.add_space(10.0);
    ui.checkbox(
        disclosure_reviewed,
        "I reviewed the exact data classes above and approve this local export",
    );
}

fn render_legal_privacy(ui: &mut egui::Ui) {
    section_heading(ui, "PRIVACY AND SUPPORT DATA");
    ui.label(
        "RSpice support export is explicit and local. The disclosure preview controls whether the redacted in-memory session log is included.",
    );
    ui.label(
        "Project content, models, results, credentials, license keys, and recovery checkpoints are excluded from the support artifact.",
    );
    ui.add_space(10.0);
    section_heading(ui, "PRODUCT LICENSE");
    ScrollArea::vertical()
        .id_salt("help.legal-license")
        .max_height(300.0)
        .show(ui, |ui| {
            ui.label(
                RichText::new(PRODUCT_LICENSE).font(theme::mono(tokens::FS_0, FontWeight::Regular)),
            );
        });
}

fn export_support_bundle(app: &mut RSpiceApp, ctx: &Context, include_session_log: bool) {
    let documents = crate::workbench::chrome::document_bar::document_descriptors(&app.state);
    let home = dirs::home_dir().map(|path| path.display().to_string());
    let logs = if include_session_log {
        app.state
            .log_buffer
            .entries()
            .map(|entry| {
                serde_json::json!({
                    "id": entry.id,
                    "timestamp": entry.format_timestamp(),
                    "severity": format!("{:?}", entry.severity),
                    "source": format!("{:?}", entry.source),
                    "message": redact_support_text(&entry.message, home.as_deref()),
                    "context": entry.context.as_deref().map(|value| redact_support_text(value, home.as_deref())),
                })
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let artifact = serde_json::json!({
        "schema": "rspice-support-bundle/v1",
        "disclosure": {
            "session_log_included": include_session_log,
            "project_content_included": false,
            "credentials_included": false,
            "recovery_bytes_included": false,
        },
        "product": {
            "version": env!("CARGO_PKG_VERSION"),
            "build": env!("RSPICE_BUILD_HASH"),
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "host": if cfg!(target_arch = "wasm32") { "browser" } else { "desktop" },
        },
        "session": {
            "project_open": app.state.project_lifecycle.project_open,
            "workspace": app.state.workbench.workspace.label(),
            "open_documents": documents.iter().filter(|document| document.open).count(),
            "dirty_documents": documents.iter().filter(|document| document.dirty).count(),
            "autosave_minutes": app.state.ui.autosave_minutes,
        },
        "schemas": {
            "project_descriptor": crate::state::PROJECT_DESCRIPTOR_SCHEMA_VERSION,
            "technology_binding": crate::state::PROJECT_TECHNOLOGY_BINDING_SCHEMA_VERSION,
            "execution_context": crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
            "model_bound_symbol": crate::state::MODEL_BOUND_SYMBOL_SCHEMA_VERSION,
            "project_source_registry": crate::state::PROJECT_SOURCE_REGISTRY_SCHEMA_VERSION,
        },
        "session_log": logs,
    });
    let contents = match serde_json::to_string_pretty(&artifact) {
        Ok(contents) => contents,
        Err(error) => {
            app.state.ui.toasts.error_with_title(
                ctx,
                "Support bundle failed",
                format!("The disclosure artifact could not be encoded: {error}"),
            );
            return;
        }
    };
    let picked = app.export_workflow_io.show_save_dialog(SaveDialogConfig {
        title: "Create RSpice Support Bundle",
        default_name: "rspice-support-bundle.json",
        filter_name: "RSpice Support Bundle",
        filter_extensions: &["json"],
    });
    let Some(mut path) = (match picked {
        Ok(path) => path,
        Err(error) => {
            app.state.ui.toasts.error_with_title(
                ctx,
                "Support bundle failed",
                format!("The export destination could not be selected: {error}"),
            );
            return;
        }
    }) else {
        return;
    };
    crate::workbench::file_actions::ensure_file_extension(&mut path, "json");
    let result = app
        .export_workflow_io
        .observe_destination(&path)
        .and_then(|destination| {
            app.export_workflow_io
                .write_text_file_observed(&destination, &contents)
        });
    match result {
        Ok(()) => app.state.ui.toasts.success(
            ctx,
            "Support bundle created",
            "The reviewed, redacted support artifact was exported locally.",
        ),
        Err(error) => app.state.ui.toasts.error_with_title(
            ctx,
            "Support bundle failed",
            format!("The support artifact was not published: {error}"),
        ),
    }
}

fn redact_support_text(value: &str, home: Option<&str>) -> String {
    let mut redacted = home
        .filter(|home| !home.is_empty())
        .map_or_else(|| value.to_owned(), |home| value.replace(home, "<HOME>"));
    const MAX_SUPPORT_TEXT_BYTES: usize = 2_048;
    if redacted.len() > MAX_SUPPORT_TEXT_BYTES {
        let mut boundary = MAX_SUPPORT_TEXT_BYTES;
        while !redacted.is_char_boundary(boundary) {
            boundary -= 1;
        }
        redacted.truncate(boundary);
        redacted.push_str("\u{2026}<truncated>");
    }
    redacted
}

fn section_heading(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
    ui.add_space(4.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_text_redacts_home_and_preserves_utf8_boundaries_when_truncated() {
        let text = format!(r"C:\Users\James\project\deck.sp {}", "μ".repeat(2_100));
        let redacted = redact_support_text(&text, Some(r"C:\Users\James"));
        assert!(redacted.starts_with(r"<HOME>\project\deck.sp"));
        assert!(redacted.ends_with("\u{2026}<truncated>"));
        assert!(redacted.is_char_boundary(redacted.len()));
    }
}

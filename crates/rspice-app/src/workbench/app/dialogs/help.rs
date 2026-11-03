//! Help dialogs on the modal primitive: the Help Center, Report a problem,
//! About, and the keyboard shortcuts.

use egui::{Context, RichText};

use super::fact_grid;
use crate::services::license::LicenseInfo;
use crate::state::command_catalog::{CommandKey, CommandPlatform};
use crate::state::shortcuts::ShortcutPreferences;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::{HelpCenterPage, RSpiceApp};
use crate::workbench::app_state::session::shortcuts::ShortcutCategory;
use crate::workbench::commands::vocabulary::{Command, CommandRegistry, command_registry};
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::workflows::export_workflow::SaveDialogConfig;

pub(in crate::workbench) mod report_problem;
pub(in crate::workbench) mod site;

#[derive(Debug, Clone)]
pub(crate) struct HelpCenterDialogState {
    pub(crate) open: bool,
    pub(crate) page: HelpCenterPage,
    pub(crate) include_session_log: bool,
    pub(crate) disclosure_reviewed: bool,
    /// Whether this dialog owns the canonical `?surface=help-center` route.
    pub(crate) route_owned: bool,
}

impl Default for HelpCenterDialogState {
    fn default() -> Self {
        Self {
            open: false,
            page: HelpCenterPage::LegalPrivacy,
            include_session_log: true,
            disclosure_reviewed: false,
            route_owned: false,
        }
    }
}

impl HelpCenterDialogState {
    pub(crate) fn open(&mut self, page: HelpCenterPage) {
        self.open = true;
        self.page = page;
        if page != HelpCenterPage::SupportBundle {
            self.disclosure_reviewed = false;
        }
    }

    pub(crate) fn open_routed(&mut self, page: HelpCenterPage) {
        self.open(page);
        self.route_owned = true;
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.disclosure_reviewed = false;
        self.route_owned = false;
    }
}

const PRODUCT_LICENSE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../LICENSE"));
const THIRD_PARTY_NOTICES: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../NOTICE"));

/// This build's version and the commit it was built from.
const VERSION_LINE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (build ",
    env!("RSPICE_BUILD_HASH"),
    ")"
);
/// The simulation core's version. rspice-core takes the workspace version,
/// as this crate does, so the two cannot differ within one build.
const ENGINE_LINE: &str = concat!("rspice-core ", env!("CARGO_PKG_VERSION"));
/// The product mark beside the line under About's title.
const MARK_SIDE: f32 = 32.0;

/// One command reference row: the command's shortcut, empty for a command
/// with none, and its name.
fn shortcut_help_row(
    shortcuts: &ShortcutPreferences,
    command: CommandKey,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> (String, &'static str) {
    (
        shortcuts.resolved_label(&CommandRegistry, command, platform, operating_system),
        command_registry().label(command),
    )
}

/// The license this machine holds, in words.
fn license_state(license: Option<&LicenseInfo>) -> String {
    match license {
        None => "Not activated".to_owned(),
        Some(info) if info.updates_expired => {
            format!("{}, updates ended {}", info.tier, info.updates_until)
        }
        Some(info) => format!("{}, updates until {}", info.tier, info.updates_until),
    }
}

/// Who holds the copyright, as the shipped license states it.
fn copyright_holder() -> &'static str {
    PRODUCT_LICENSE
        .lines()
        .find_map(|line| line.trim().strip_prefix("Copyright (c) "))
        .map_or("RSpice Contributors", |holder| {
            holder.split_once(". ").map_or(holder, |(holder, _)| holder)
        })
}

fn shortcut_row_matches(
    shortcuts: &ShortcutPreferences,
    command: CommandKey,
    filter: &str,
    platform: CommandPlatform,
    operating_system: egui::os::OperatingSystem,
) -> bool {
    filter.is_empty()
        || command_registry()
            .label(command)
            .to_lowercase()
            .contains(filter)
        || command.as_str().contains(filter)
        || shortcuts
            .resolved_label(&CommandRegistry, command, platform, operating_system)
            .to_lowercase()
            .contains(filter)
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_help_center_dialog(&mut self, ctx: &Context) {
        if !self.state.shell.help_center.open {
            return;
        }

        // System diagnostics and Create support bundle both open Report a
        // problem, which is its own dialog rather than two of these pages.
        if report_problem::is_report_page(self.state.shell.help_center.page) {
            self.render_report_problem_dialog(ctx);
            return;
        }

        // Legal & privacy is the one page left to draw here: Help and
        // Release notes open rspice.app, and Report a problem has its own
        // dialog.
        let choice = Dialog::new(HelpCenterPage::LegalPrivacy.label(), "Close")
            .description("Read how support data is handled, the product license, and the third-party notices.")
            .size(DialogSize::Manager)
            .show(ctx, render_legal_privacy);

        if choice != DialogChoice::None {
            self.close_help_center_route();
        }
    }

    fn close_help_center_route(&mut self) {
        let route_owned = self.state.shell.help_center.route_owned;
        self.state.shell.help_center.close();
        if route_owned {
            self.state.close_route_owned_transient(
                crate::workbench::SurfaceId::HelpCenter,
                "Legal & privacy",
            );
        }
    }

    pub(in crate::workbench) fn render_about_dialog(&mut self, ctx: &Context) {
        if !self.state.shell.about {
            return;
        }

        let license = license_state(self.state.cloud.license.as_ref());
        let copyright = format!("\u{a9} {}", copyright_holder());
        let mut notices = false;
        let choice = Dialog::new("About RSpice", "Close")
            .description(
                "The RSpice version and build, its simulation engine, this machine's license, and \
                 the notices it ships with.",
            )
            .size(DialogSize::CommandForm)
            .overlay_scrollbar()
            .ghost("Copy diagnostics")
            .show(ctx, |ui| {
                let t = Tokens::get(ui.ctx());
                // The mark stands as far below the body's edge as the last
                // row's words stand above it.
                ui.add_space(fact_grid::row_air(ui));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = tokens::SP_5;
                    let (mark, _) =
                        ui.allocate_exact_size(egui::Vec2::splat(MARK_SIDE), egui::Sense::hover());
                    WorkbenchIcon::Brand.paint(ui.painter(), mark, t.color.accent);
                    ui.label(
                        RichText::new("Analog circuit design and simulation")
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                });
                ui.add_space(tokens::SP_5);
                fact_grid::show(ui, |rows| {
                    rows.mono("Version", VERSION_LINE);
                    rows.mono("Engine", ENGINE_LINE);
                    rows.text("License", &license);
                    rows.text("Copyright", &copyright);
                    rows.row("Notices", |ui| {
                        notices = ui.link("Third-party notices").clicked();
                    });
                });
            });

        if notices {
            self.state.shell.about = false;
            Command::LegalPrivacy.execute(self);
            return;
        }
        match choice {
            DialogChoice::None => {}
            DialogChoice::Ghost => {
                // The first thing support asks for, one click to produce.
                let diagnostics = format!(
                    "RSpice {VERSION_LINE} \u{b7} {ENGINE_LINE} \u{b7} license {license} \u{b7} {}-{}",
                    std::env::consts::OS,
                    std::env::consts::ARCH,
                );
                ctx.copy_text(diagnostics);
                self.state
                    .prompt("Application and platform diagnostics were copied to the clipboard.");
            }
            _ => self.state.shell.about = false,
        }
    }

    pub(in crate::workbench) fn render_shortcuts_help_dialog(&mut self, ctx: &Context) {
        if !self.state.shell.shortcuts_help {
            return;
        }

        // Count against the filter as entered last frame — immediate mode
        // corrects the hint one frame after each keystroke.
        let shortcuts = self.state.model.preferences.shortcuts().clone();
        let platform = crate::workbench::app_state::runtime_command_platform(ctx);
        let operating_system = ctx.os();
        let filter = self.state.shell.shortcuts_filter.trim().to_lowercase();
        let total: usize = ShortcutCategory::ALL
            .iter()
            .flat_map(|category| category.commands(command_registry()))
            .count();
        let shown: usize = ShortcutCategory::ALL
            .iter()
            .flat_map(|c| c.commands(command_registry()))
            .filter(|c| shortcut_row_matches(&shortcuts, *c, &filter, platform, operating_system))
            .count();
        let hint = format!("{shown} of {total} commands");

        let filter_text = &mut self.state.shell.shortcuts_filter;
        let choice = Dialog::new("Keyboard shortcuts", "Close")
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
                        .hint_text("Filter by name or shortcut")
                        .desired_width(f32::INFINITY),
                );
                let filter = filter_text.trim().to_lowercase();

                ui.spacing_mut().item_spacing.y = 0.0;
                let mut any_shown = false;

                for category in ShortcutCategory::ALL {
                    let commands: Vec<_> = category
                        .commands(command_registry())
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
                    section_heading(ui, category.display_name());

                    for command in commands {
                        let (shortcut, display_name) =
                            shortcut_help_row(&shortcuts, command, platform, operating_system);
                        // Name left, key chord right in mono as a key cap;
                        // a command with no shortcut leaves the cell blank.
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
                        if shortcut.is_empty() {
                            continue;
                        }
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
                        egui::RichText::new("No command matches the filter")
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_faint),
                    );
                }
            });

        if choice != DialogChoice::None {
            self.state.shell.shortcuts_help = false;
            self.state.shell.shortcuts_filter.clear();
        }
    }
}

fn render_legal_privacy(ui: &mut egui::Ui) {
    section_heading(ui, "Privacy and support data");
    let t = Tokens::get(ui.ctx());
    for paragraph in [
        "A support bundle is made only when you create one from Report a problem, and saved \
         where you choose. You decide whether it carries the session log, with your home \
         folder removed.",
        "Project content, models, results, credentials, license keys, and recovery checkpoints \
         are excluded from the support artifact.",
    ] {
        ui.add(
            egui::Label::new(
                RichText::new(paragraph)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text),
            )
            .wrap(),
        );
    }
    ui.add_space(10.0);
    // Both documents read in the dialog's own scrolling body: a second
    // scroller inside it left the notices with nowhere to go.
    section_heading(ui, "Product license");
    legal_document(ui, PRODUCT_LICENSE);
    ui.add_space(10.0);
    section_heading(ui, "Third-party notices");
    legal_document(ui, THIRD_PARTY_NOTICES);
}

/// A shipped legal file, wrapped once, at the dialog's width.
///
/// Each paragraph and list item is laid as one block: the file's own hard
/// line breaks would otherwise break every line twice, at 79 columns and
/// again at the dialog's edge, and an item's wrapped lines fell back to the
/// left margin under its marker. An item keeps its indent and its marker,
/// and its further lines hang under the text after the marker.
fn legal_document(ui: &mut egui::Ui, source: &'static str) {
    let t = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let column = ui.fonts_mut(|fonts| fonts.glyph_width(&font, ' '));
    let line = ui.fonts_mut(|fonts| fonts.row_height(&font));
    let text = |text: &str| RichText::new(text).font(font.clone()).color(t.color.text);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
        for block in legal_blocks(&legal_text(source)) {
            let Some(block) = block else {
                ui.add_space(line);
                continue;
            };
            ui.horizontal_top(|ui| {
                ui.add_space(block.indent as f32 * column);
                if !block.marker.is_empty() {
                    ui.label(text(&block.marker));
                }
                ui.vertical(|ui| {
                    ui.add(egui::Label::new(text(&block.text)).wrap());
                });
            });
        }
    });
}

/// One paragraph or list item of a legal file: its indent in columns, its
/// list marker with the space after it, and its words on one line.
#[derive(Debug, PartialEq)]
struct LegalBlock {
    indent: usize,
    marker: String,
    text: String,
}

/// A line at least this long ended where the file's author wrapped it, so
/// the line after it continues the same paragraph. Headings and list items
/// that end short stand alone.
const HARD_WRAP_COLUMN: usize = 60;

/// `source` as its paragraphs and list items, a blank line as `None`.
fn legal_blocks(source: &str) -> Vec<Option<LegalBlock>> {
    let mut blocks: Vec<Option<LegalBlock>> = Vec::new();
    let mut previous_wrapped = false;
    for raw in source.lines() {
        let line = raw.trim_end();
        if line.is_empty() {
            blocks.push(None);
            previous_wrapped = false;
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        let body = line.trim_start();
        let rule = is_rule(body);
        let marker = if rule { None } else { list_marker(body) };
        if previous_wrapped
            && !rule
            && marker.is_none()
            && let Some(Some(open)) = blocks.last_mut()
            && indent == open.indent + open.marker.len()
        {
            open.text.push(' ');
            open.text.push_str(body);
        } else {
            let (marker, text) = marker.map_or(("", body), |marker| body.split_at(marker));
            blocks.push(Some(LegalBlock {
                indent,
                marker: marker.to_owned(),
                text: text.to_owned(),
            }));
        }
        previous_wrapped = !rule && line.chars().count() >= HARD_WRAP_COLUMN;
    }
    blocks
}

/// A line of one repeated `=` or `-`: a heading's underline or a divider.
fn is_rule(body: &str) -> bool {
    let mut chars = body.chars();
    chars
        .next()
        .is_some_and(|first| (first == '=' || first == '-') && chars.all(|c| c == first))
        && body.len() >= 3
}

/// The length of the list marker `body` opens with, with its trailing space:
/// "- ", "* ", "1. " or "(a) ".
fn list_marker(body: &str) -> Option<usize> {
    if body.starts_with("- ") || body.starts_with("* ") {
        return Some(2);
    }
    let (open, close) = if body.starts_with('(') {
        (1, ')')
    } else {
        (0, '.')
    };
    let label = &body[open..];
    let end = label.find(close)?;
    let name = &label[..end];
    let numbered = !name.is_empty()
        && name.len() <= 4
        && name.chars().all(|c| c.is_ascii_alphanumeric())
        && (open == 1 || name.chars().all(|c| c.is_ascii_digit()));
    (numbered && label[end + 1..].starts_with(' ')).then_some(open + end + 2)
}

fn legal_text(source: &'static str) -> std::borrow::Cow<'static, str> {
    if source.contains('\r') {
        std::borrow::Cow::Owned(source.replace("\r\n", "\n"))
    } else {
        std::borrow::Cow::Borrowed(source)
    }
}

fn export_support_bundle(app: &mut RSpiceApp, include_session_log: bool) {
    let documents = crate::workbench::chrome::document_bar::document_descriptors(&app.state);
    let logs = if include_session_log {
        support_bundle_log(&app.state, crate::diagnostics::home_directory().as_deref())
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
            "project_open": app.state.platform.project_lifecycle.is_open(),
            "workspace": app.state.workbench.workspace.label(),
            "open_documents": documents.iter().filter(|document| document.open).count(),
            "dirty_documents": documents.iter().filter(|document| document.dirty).count(),
            "autosave_minutes": app.state.platform.autosave_minutes,
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
            app.state.report_failure(
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
            app.state.report_failure(
                "Support bundle failed",
                format!("The export destination could not be selected: {error}"),
            );
            return;
        }
    }) else {
        return;
    };
    crate::io::file_exchange::ensure_file_extension(&mut path, "json");
    let result = app
        .export_workflow_io
        .observe_destination(&path)
        .and_then(|destination| {
            app.export_workflow_io
                .write_text_file_observed(&destination, &contents)
        });
    match result {
        Ok(()) => {
            let message = crate::workbench::workflows::export_workflow::export_completion_message(
                "support bundle",
                &path,
                None,
                app.export_workflow_io.as_ref(),
            );
            app.state.prompt(message);
        }
        Err(error) => app.state.report_failure(
            "Support bundle failed",
            format!("The support artifact was not published: {error}"),
        ),
    }
}

/// The session log as the support bundle carries it, each message redacted.
fn support_bundle_log(
    state: &crate::workbench::app_state::AppState,
    home: Option<&str>,
) -> Vec<serde_json::Value> {
    state
        .model
        .log_buffer
        .entries()
        .map(|entry| {
            serde_json::json!({
                "id": entry.id,
                "timestamp": entry.format_timestamp(),
                "severity": format!("{:?}", entry.severity),
                "source": format!("{:?}", entry.source),
                "message": crate::diagnostics::redact_log_message(&entry.message, home),
            })
        })
        .collect()
}

/// A caption over one part of a Help page, in the face the kit titles a
/// panel section in, and a heading to assistive technology.
fn section_heading(ui: &mut egui::Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let response = ui.label(
        RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    let label = text.to_owned();
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(label.clone());
    });
    ui.add_space(4.0);
}

#[cfg(test)]
mod tests {
    /// The notices' hard-wrapped paragraphs read as one block each, and a
    /// list item keeps its marker and its indent with its wrapped lines
    /// joined behind them.
    #[test]
    fn legal_files_reflow_into_paragraphs_and_hanging_items() {
        let blocks = super::legal_blocks(&super::legal_text(super::THIRD_PARTY_NOTICES));
        let paragraph = blocks
            .iter()
            .flatten()
            .find(|block| block.text.starts_with("Portions of RSpice's device models"))
            .expect("the ngspice paragraph");
        assert!(
            paragraph.text.contains("version 46. All such files"),
            "the paragraph stops at its file's line break: {:?}",
            paragraph.text
        );
        let item = blocks
            .iter()
            .flatten()
            .find(|block| block.text.starts_with("BSIM3SOI DD/FD/PD"))
            .expect("the BSIM3SOI item");
        assert_eq!((item.indent, item.marker.as_str()), (2, "- "));
        assert!(item.text.contains("ported from ngspice"), "{:?}", item.text);

        let blocks = super::legal_blocks(&super::legal_text(super::PRODUCT_LICENSE));
        let heading = blocks
            .iter()
            .flatten()
            .find(|block| block.text == "GRANT OF LICENSE")
            .expect("a short heading stands alone");
        assert_eq!(heading.marker, "2. ");
        let definition = blocks
            .iter()
            .flatten()
            .find(|block| block.text.starts_with("\"Software\" means"))
            .expect("the first definition");
        assert_eq!((definition.indent, definition.marker.as_str()), (3, "(a) "));
        assert!(
            blocks
                .iter()
                .flatten()
                .all(|block| !block.text.contains('\n')),
            "a block keeps a line break"
        );
        // Reflowing loses no word and moves none.
        for source in [super::PRODUCT_LICENSE, super::THIRD_PARTY_NOTICES] {
            let words = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
            let blocks = super::legal_blocks(&super::legal_text(source))
                .into_iter()
                .flatten()
                .map(|block| format!("{}{}", block.marker, block.text))
                .collect::<Vec<_>>()
                .join(" ");
            assert_eq!(words(&blocks), words(source));
        }
    }

    use super::*;

    #[test]
    fn the_support_bundle_log_carries_no_home_directory() {
        let mut state = crate::workbench::app_state::AppState::default();
        state.record(
            crate::diagnostics::LogSeverity::Info,
            crate::diagnostics::LogSource::Studio,
            r"Opened project amp (C:\Users\James\designs\amp.rspiceproj).",
        );

        let log = support_bundle_log(&state, Some(r"C:\Users\James"));

        let message = log
            .last()
            .and_then(|entry| entry["message"].as_str())
            .expect("the line is carried");
        assert_eq!(
            message,
            r"Opened project amp (<HOME>\designs\amp.rspiceproj)."
        );
    }

    /// One node the frame published for a reader: what it says, what it is,
    /// and where it was drawn.
    struct Shown {
        text: String,
        role: egui::accesskit::Role,
        bounds: Option<egui::accesskit::Rect>,
    }

    /// Every node `render` published for a reader, at desktop size. Generic
    /// over the app so that a test helper is not counted as a handler that
    /// takes the whole application.
    fn shown_nodes<App>(app: &mut App, render: fn(&mut App, &Context)) -> Vec<Shown> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut update = None;
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..Default::default()
                },
                |ui| render(app, ui.ctx()),
            );
            update = output.platform_output.accesskit_update;
        }
        update
            .expect("the frame published its accessibility tree")
            .nodes
            .iter()
            .filter_map(|(_, node)| {
                Some(Shown {
                    text: node.value().or_else(|| node.label())?.to_owned(),
                    role: node.role(),
                    bounds: node.bounds(),
                })
            })
            .collect()
    }

    /// Every text the Help Center published for a reader to hear.
    fn shown_texts(mut app: RSpiceApp) -> Vec<String> {
        shown_nodes(&mut app, RSpiceApp::render_help_center_dialog)
            .into_iter()
            .map(|shown| shown.text)
            .collect()
    }

    /// Examples… is a Help menu row: it opens the Examples dialog on its
    /// first circuit, and the Help Center no longer offers a button of its
    /// own.
    #[test]
    fn examples_open_from_the_help_menu_and_not_from_the_help_center() {
        let mut app = RSpiceApp::test_instance();
        Command::Examples.execute(&mut app);
        assert_eq!(app.state.project.examples_selection, Some(0));

        let mut app = RSpiceApp::test_instance();
        app.state.shell.help_center.open(HelpCenterPage::default());
        let shown = shown_texts(app);
        assert!(
            !shown.iter().any(|text| text == "Examples\u{2026}"),
            "{shown:?}"
        );
    }

    /// The Help Center draws the legal and privacy notices and nothing else:
    /// its page of six topics restated rspice.app/docs, which RSpice Help
    /// opens, so there is no topic, search field or page strip.
    #[test]
    fn the_help_center_offers_the_legal_notices_only() {
        assert_eq!(HelpCenterPage::default(), HelpCenterPage::LegalPrivacy);
        let mut app = RSpiceApp::test_instance();
        app.state.shell.help_center.open(HelpCenterPage::default());
        let shown = shown_nodes(&mut app, RSpiceApp::render_help_center_dialog);
        assert!(
            shown.iter().any(|shown| shown.text == "Legal & privacy"),
            "the dialog is titled Legal & privacy"
        );
        for gone in [
            "Help center",
            "Schematic authoring",
            "Simulation setup",
            "Keyboard shortcuts",
        ] {
            assert!(
                !shown.iter().any(|shown| shown.text.starts_with(gone)),
                "{gone:?} is still offered"
            );
        }
        assert!(
            !shown
                .iter()
                .any(|shown| shown.role == egui::accesskit::Role::TextInput),
            "the topic search field is gone"
        );
        assert!(
            !shown.iter().any(|shown| shown.text == "Legal & privacy"
                && matches!(
                    shown.role,
                    egui::accesskit::Role::Button | egui::accesskit::Role::Tab
                )),
            "the title alone names the page; no one-tab strip repeats it"
        );
    }

    /// About says what the build, its engine and this machine's license are
    /// in words, beside their labels: no dash for "no license", no
    /// abbreviation, and the copyright the shipped license states.
    #[test]
    fn about_states_the_build_engine_and_license_in_words() {
        let mut app = RSpiceApp::test_instance();
        assert!(app.state.cloud.license.is_none());
        Command::About.execute(&mut app);
        let shown = shown_nodes(&mut app, RSpiceApp::render_about_dialog);
        let texts: Vec<&str> = shown.iter().map(|shown| shown.text.as_str()).collect();
        for expected in [
            "About RSpice",
            VERSION_LINE,
            ENGINE_LINE,
            "Not activated",
            "\u{a9} 2026 RSpice Contributors",
            "Third-party notices",
        ] {
            assert!(
                texts.contains(&expected),
                "About shows {expected:?}: {texts:#?}"
            );
        }
        assert!(PRODUCT_LICENSE.contains(&format!("Copyright (c) {}.", copyright_holder())));
        for text in &texts {
            assert!(!text.contains('\u{2014}'), "{text:?} has an em dash");
            assert!(!text.contains("pkg"), "{text:?} abbreviates");
        }
        // Each value starts at one x, beside its label rather than at the
        // dialog's far edge.
        let value_left: Vec<f64> = shown
            .iter()
            .filter(|shown| {
                [VERSION_LINE, ENGINE_LINE, "Not activated"].contains(&shown.text.as_str())
            })
            .filter_map(|shown| shown.bounds.map(|bounds| bounds.x0.round()))
            .collect();
        // A selectable label publishes its words twice: as the label and as
        // its text run, at one place.
        assert!(value_left.len() >= 3, "{value_left:?}");
        assert!(
            value_left.windows(2).all(|pair| pair[0] == pair[1]),
            "{value_left:?}"
        );
    }

    /// A command with no shortcut leaves its cell blank rather than marking
    /// it with a dash.
    #[test]
    fn an_unbound_command_leaves_its_shortcut_cell_blank() {
        let shortcuts = ShortcutPreferences::default();
        let os = egui::os::OperatingSystem::Windows;
        let unbound = ShortcutCategory::ALL
            .iter()
            .flat_map(|category| category.commands(command_registry()))
            .find(|command| {
                shortcuts
                    .resolved_label(&CommandRegistry, *command, CommandPlatform::Desktop, os)
                    .is_empty()
            })
            .expect("some listed command has no shortcut");
        let (shortcut, name) = shortcut_help_row(&shortcuts, unbound, CommandPlatform::Desktop, os);
        assert_eq!(shortcut, "", "{name} shows a blank shortcut cell");
    }

    /// The legal page's captions are headings in the kit's section face.
    #[test]
    fn the_legal_page_captions_are_headings() {
        let mut app = RSpiceApp::test_instance();
        Command::LegalPrivacy.execute(&mut app);
        let shown = shown_nodes(&mut app, RSpiceApp::render_help_center_dialog);
        for caption in [
            "Privacy and support data",
            "Product license",
            "Third-party notices",
        ] {
            assert!(
                shown
                    .iter()
                    .any(|shown| shown.text == caption
                        && shown.role == egui::accesskit::Role::Heading),
                "{caption:?} is a heading"
            );
        }
    }

    /// The legal page shows the license and the third-party notices the
    /// repository ships, each whole: every paragraph and item of each file.
    #[test]
    fn the_legal_page_shows_the_shipped_license_and_third_party_notices() {
        let mut app = RSpiceApp::test_instance();
        Command::LegalPrivacy.execute(&mut app);
        let shown = shown_texts(app);
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        for name in ["LICENSE", "NOTICE"] {
            let file = std::fs::read_to_string(root.join(name))
                .expect("the repository ships the file")
                .replace("\r\n", "\n");
            for block in super::legal_blocks(&file).into_iter().flatten() {
                assert!(
                    shown.contains(&block.text),
                    "the legal page leaves out {:?} from {name}",
                    block.text
                );
            }
        }
    }

    /// Both rows that each opened half of it open Report a problem, which
    /// names the file the session log is written to and where to send a
    /// report; the Help Center stops offering the two pages it replaced.
    #[test]
    fn report_a_problem_replaces_the_diagnostics_and_support_bundle_pages() {
        for command in [Command::SystemDiagnostics, Command::SupportBundle] {
            let mut app = RSpiceApp::test_instance();
            command.execute(&mut app);
            let shown = shown_texts(app);
            for expected in ["Report a problem", report_problem::SUPPORT_ADDRESS] {
                assert!(
                    shown.iter().any(|text| text == expected),
                    "{command:?} shows {expected:?}"
                );
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let log = crate::workbench::lifecycle::recovery::diagnostics_directory()
                    .expect("the test host has an application-data directory")
                    .join("current-session.log")
                    .display()
                    .to_string();
                assert!(
                    shown.contains(&log),
                    "{command:?} names the session log file {log}"
                );
            }
        }
        let mut app = RSpiceApp::test_instance();
        app.state.shell.help_center.open(HelpCenterPage::default());
        let shown = shown_texts(app);
        assert!(
            !shown
                .iter()
                .any(|text| text == "Diagnostics" || text == "Support bundle"),
            "the Help Center no longer offers the pages Report a problem replaced"
        );
    }

    /// Writes nothing, and fails the write when told to.
    struct BundleIo {
        write_error: Option<&'static str>,
    }

    impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for BundleIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<std::path::PathBuf>, String> {
            Ok(Some(std::path::PathBuf::from("bundle.json")))
        }

        fn write_text_file(&self, _path: &std::path::Path, _contents: &str) -> Result<(), String> {
            self.write_error
                .map_or(Ok(()), |error| Err(error.to_owned()))
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &std::path::Path,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    /// The bundle the user just exported is named on the status bar; one
    /// that could not be written is said now.
    #[test]
    fn a_support_bundle_reports_its_file_and_toasts_only_a_failure() {
        let ctx = egui::Context::default();
        let mut app = RSpiceApp::test_instance();
        app.export_workflow_io = Box::new(BundleIo { write_error: None });
        export_support_bundle(&mut app, false);
        app.state.drain_notices(&ctx);
        assert!(
            app.state
                .prompt_text()
                .is_some_and(|prompt| prompt.contains("bundle.json")),
            "{:?}",
            app.state.prompt_text()
        );
        assert!(app.state.workbench.toasts.activity().is_empty());

        app.export_workflow_io = Box::new(BundleIo {
            write_error: Some("disk full"),
        });
        export_support_bundle(&mut app, false);
        app.state.drain_notices(&ctx);
        let activity = app.state.workbench.toasts.activity();
        assert_eq!(activity.len(), 1);
        assert_eq!(activity[0].title(), "Support bundle failed");
    }
}

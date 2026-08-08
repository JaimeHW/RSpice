//! Publish to web — the immutable-snapshot publication transaction.
//!
//! Compose shows exactly what leaves this machine: the scope inventory comes
//! from a real snapshot build at open time, and the payload figure is the
//! canonical byte count against the contract's hard cap. Submit rebuilds the
//! snapshot with the confirmed page fields so the published bytes are always
//! the bytes reviewed here, then hands one typed request to the cloud
//! session executor; progress renders from its owned snapshot.

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::services::cloud_account::{
    CLOUD_PUBLISHING_FEATURE, CloudAccountAvailability, PublishRequest, PublishState, PublishTarget,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::publication_snapshot::{
    PublicationDraft, build_publication_snapshot, publication_timestamp_utc,
};

/// What the open-time snapshot build proved about the publish scope.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct PublishScope {
    /// Schematic sheet names, in page order.
    pub sheets: Vec<String>,
    /// Plot figure titles, in page order.
    pub plots: Vec<String>,
    /// SPICE title line of the effective deck, when a deck publishes.
    pub netlist_title: Option<String>,
    pub analyses: usize,
    pub datasets: usize,
    pub measurements: usize,
    /// Canonical `.rspicepub` byte count at review time.
    pub payload_bytes: usize,
}

/// Publish-to-web dialog state. `scope` holds the reviewed build (or the
/// builder's refusal); `submitted` means this dialog handed a request to the
/// executor and now renders its progress.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct PublishWebDialogState {
    pub open: bool,
    pub title: String,
    pub description: String,
    pub author_credit: String,
    /// First-publish page visibility; the service pins it on the circuit.
    pub public_visibility: bool,
    /// Content license sealed into this version's snapshot.
    pub license: rspice_publication_contract::ContentLicense,
    /// Selected workspace for a first publish (index into the session list).
    pub workspace_index: usize,
    pub scope: Option<Result<PublishScope, String>>,
    pub submitted: bool,
}

impl RSpiceApp {
    /// Open the dialog with a fresh review build and the signed-in identity
    /// as the default author credit. No-op unless this build can publish.
    pub(crate) fn open_publish_web_dialog(&mut self) {
        if !matches!(
            self.cloud_account.availability(),
            CloudAccountAvailability::Native
        ) || !self
            .cloud_account
            .snapshot()
            .cloud_feature_enabled(CLOUD_PUBLISHING_FEATURE)
        {
            return;
        }
        let author = self
            .cloud_account
            .snapshot()
            .principal
            .as_ref()
            .and_then(|principal| {
                principal
                    .display_name
                    .clone()
                    .or_else(|| principal.email.clone())
            })
            .unwrap_or_default();
        let project_name = self.state.workspace.project.name().trim().to_owned();
        let title = if project_name.is_empty() {
            "Untitled project".to_owned()
        } else {
            project_name
        };
        let scope = build_scope(&self.state, &title, &author);
        self.state.dialogs.publish_web = PublishWebDialogState {
            open: true,
            title,
            author_credit: author,
            scope: Some(scope),
            ..Default::default()
        };
        if let Some(binding) = self.state.workspace.project.cloud_publication() {
            self.cloud_account
                .refresh_publications(binding.circuit_id().to_owned());
        }
    }

    pub(in crate::workbench) fn render_publish_web_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.publish_web.open {
            return;
        }

        // Server-side facts, cloned before the body borrows app state.
        let publish_state = self.cloud_account.snapshot().publish.clone();
        let publications = self.cloud_account.snapshot().publications.clone();
        let workspaces = self.cloud_account.snapshot().workspaces.clone();
        let binding = self
            .state
            .workspace
            .project
            .cloud_publication()
            .map(|binding| {
                (
                    binding.workspace_id().to_owned(),
                    binding.circuit_id().to_owned(),
                )
            });
        let live_publication = binding.as_ref().and_then(|_| {
            publications
                .iter()
                .find(|publication| {
                    publication.unpublished_at.is_none() && publication.page_status == "live"
                })
                .cloned()
        });
        let version_count = if binding.is_some() {
            publications.len()
        } else {
            0
        };

        let submitted = self.state.dialogs.publish_web.submitted;
        let in_flight = submitted
            && matches!(
                publish_state,
                None | Some(PublishState::Publishing) | Some(PublishState::AwaitingPage { .. })
            );
        let succeeded = submitted && matches!(publish_state, Some(PublishState::Live { .. }));
        let failed_message = if submitted {
            match &publish_state {
                Some(PublishState::Failed { message }) => Some(message.clone()),
                _ => None,
            }
        } else {
            None
        };

        let scope_ok = matches!(self.state.dialogs.publish_web.scope, Some(Ok(_)));
        let compose_ready = scope_ok
            && !self.state.dialogs.publish_web.title.trim().is_empty()
            && (binding.is_some() || !workspaces.is_empty());

        let publish_label = if version_count > 0 {
            format!("Publish version {}", version_count + 1)
        } else {
            "Publish".to_owned()
        };
        let (primary, primary_enabled): (String, bool) = if succeeded {
            ("Close".to_owned(), true)
        } else if in_flight {
            ("Publishing…".to_owned(), false)
        } else {
            (publish_label, compose_ready)
        };
        let hint = if let Some((_, circuit)) = &binding {
            format!("immutable versions · circuit {circuit}")
        } else {
            "immutable snapshot · earlier versions keep their URLs".to_owned()
        };

        let mut dialog = Dialog::new("Publication", "Publish to web", &primary)
            .description(
                "Review the exact snapshot that leaves this machine, then publish it as an \
                 immutable page version.",
            )
            .size(DialogSize::Transaction)
            .ghost(if submitted { "Close" } else { "Cancel" })
            .hint(&hint)
            .primary_enabled(primary_enabled)
            .primary_on_enter(false);
        let secondary_unpublishes = !submitted && live_publication.is_some();
        if secondary_unpublishes {
            dialog = dialog.secondary("Unpublish…");
        } else if succeeded {
            dialog = dialog.secondary("Copy URL");
        }

        let dialog_state = &mut self.state.dialogs.publish_web;
        let choice = dialog.show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            let c = t.color;

            if submitted {
                render_progress(ui, publish_state.as_ref());
                return;
            }

            if let Some(message) = &failed_message {
                note(ui, message, c.err);
                ui.add_space(tokens::SP_4);
            }
            if let Some(live) = &live_publication {
                note(
                    ui,
                    &format!(
                        "Version {version_count} is live at {}. Publishing creates version {} \
                         from the scope below; every earlier version stays resolvable at its \
                         pinned URL.",
                        live.url_path,
                        version_count + 1
                    ),
                    c.text_dim,
                );
                ui.add_space(tokens::SP_4);
            }

            match dialog_state.scope.clone() {
                Some(Err(reason)) => {
                    note(ui, &format!("Nothing can be published: {reason}"), c.err);
                }
                Some(Ok(scope)) => {
                    ui.columns(2, |columns| {
                        render_scope_column(&mut columns[0], &scope);
                        render_page_column(
                            &mut columns[1],
                            dialog_state,
                            binding.is_some(),
                            &workspaces,
                            live_publication.as_ref().map(|live| live.url_path.as_str()),
                        );
                    });
                    ui.add_space(tokens::SP_4);
                    render_leaves_machine(ui, &scope);
                    ui.add_space(tokens::SP_4);
                    note(
                        ui,
                        "Everything listed becomes publicly retrievable at the page URL. \
                         Publishing is immutable: a new version never rewrites an earlier one.",
                        c.text_faint,
                    );
                }
                None => {}
            }
        });

        match choice {
            DialogChoice::None => {}
            DialogChoice::Cancelled | DialogChoice::Ghost => {
                // An in-flight publish keeps running on the executor; the
                // receipt still binds the project and reports to the console.
                self.state.dialogs.publish_web.open = false;
            }
            DialogChoice::Secondary => {
                if secondary_unpublishes {
                    if let (Some((_, circuit_id)), Some(live)) = (&binding, &live_publication) {
                        self.state.dialogs.publish_web.open = false;
                        self.open_unpublish_web_dialog(
                            circuit_id.clone(),
                            live.id.clone(),
                            live.url_path.clone(),
                            version_count,
                        );
                    }
                } else if succeeded && let Some(PublishState::Live { url_path }) = &publish_state {
                    ctx.copy_text(url_path.clone());
                }
            }
            DialogChoice::Primary => {
                if succeeded {
                    self.state.dialogs.publish_web.open = false;
                } else if !in_flight {
                    self.submit_publish(binding, &workspaces);
                }
            }
        }
    }

    /// Drain the executor's publish receipt: bind the project to its page
    /// lineage and report the URL. Runs every frame from the app loop so a
    /// closed dialog never loses the binding.
    pub(in crate::workbench) fn record_publish_receipt(&mut self) {
        let Some(receipt) = self.cloud_account.take_publish_receipt() else {
            return;
        };
        match crate::state::ProjectCloudPublicationBinding::new(
            receipt.workspace_id.clone(),
            receipt.circuit_id.clone(),
        ) {
            Ok(binding) => {
                if let Err(error) = self.state.workspace.bind_cloud_publication(binding) {
                    self.state
                        .push_user_message(ConsoleMessage::warning(format!(
                            "Published, but the project could not record its page lineage: {error}"
                        )));
                }
            }
            Err(error) => {
                self.state
                    .push_user_message(ConsoleMessage::warning(format!(
                        "Published, but the returned identifiers were not usable: {error}"
                    )));
            }
        }
        self.state.push_user_message(ConsoleMessage::info(format!(
            "Published {} — the page is being prepared.",
            receipt.url_path
        )));
    }

    /// Rebuild the snapshot with the confirmed page fields and hand the
    /// publish request to the executor.
    fn submit_publish(
        &mut self,
        binding: Option<(String, String)>,
        workspaces: &[crate::services::cloud_account::WorkspaceSummary],
    ) {
        let dialog = &self.state.dialogs.publish_web;
        let title = dialog.title.trim().to_owned();
        let description = dialog.description.trim().to_owned();
        let author = {
            let credit = dialog.author_credit.trim();
            if credit.is_empty() {
                "Anonymous".to_owned()
            } else {
                credit.to_owned()
            }
        };
        let public = dialog.public_visibility;
        let workspace_index = dialog.workspace_index;

        let target = match resolve_target(binding, workspaces, workspace_index, public) {
            Ok(target) => target,
            Err(message) => {
                self.state.dialogs.publish_web.scope = Some(Err(message));
                return;
            }
        };
        let draft = PublicationDraft {
            title: title.clone(),
            description: description.clone(),
            author_display: author,
            created_utc: publication_timestamp_utc(),
            license: dialog.license,
        };
        let built = build_publication_snapshot(&self.state, &draft)
            .map_err(|error| error.to_string())
            .and_then(|snapshot| {
                snapshot
                    .canonical_bytes()
                    .map_err(|error| error.to_string())
            });
        match built {
            Ok(snapshot_bytes) => {
                self.cloud_account.publish_snapshot(PublishRequest {
                    target,
                    title,
                    description,
                    snapshot_bytes,
                });
                self.state.dialogs.publish_web.submitted = true;
            }
            Err(message) => {
                self.state.dialogs.publish_web.scope = Some(Err(message));
                self.state.push_user_message(ConsoleMessage::error(
                    "Publish stopped: the snapshot no longer builds from the current project \
                     state.",
                ));
            }
        }
    }
}

/// Resolve where this publish lands: the bound circuit, or a new circuit in
/// the chosen workspace.
fn resolve_target(
    binding: Option<(String, String)>,
    workspaces: &[crate::services::cloud_account::WorkspaceSummary],
    workspace_index: usize,
    public: bool,
) -> Result<PublishTarget, String> {
    if let Some((workspace_id, circuit_id)) = binding {
        let workspace_id = uuid::Uuid::parse_str(&workspace_id)
            .map_err(|_| "the recorded publication binding is not a valid identifier".to_owned())?;
        let circuit_id = uuid::Uuid::parse_str(&circuit_id)
            .map_err(|_| "the recorded publication binding is not a valid identifier".to_owned())?;
        return Ok(PublishTarget::ExistingCircuit {
            workspace_id,
            circuit_id,
        });
    }
    let workspace = workspaces
        .get(workspace_index)
        .or_else(|| workspaces.first())
        .ok_or_else(|| "the signed-in account has no workspace to publish into".to_owned())?;
    let workspace_id = uuid::Uuid::parse_str(&workspace.id)
        .map_err(|_| "the session workspace list is not usable".to_owned())?;
    Ok(PublishTarget::NewCircuit {
        workspace_id,
        public,
    })
}

/// Build the reviewed scope: a real snapshot with the current defaults.
fn build_scope(state: &AppState, title: &str, author: &str) -> Result<PublishScope, String> {
    let draft = PublicationDraft {
        title: title.to_owned(),
        description: String::new(),
        author_display: if author.is_empty() {
            "Anonymous".to_owned()
        } else {
            author.to_owned()
        },
        created_utc: publication_timestamp_utc(),
        license: rspice_publication_contract::ContentLicense::default(),
    };
    let snapshot = build_publication_snapshot(state, &draft).map_err(|error| error.to_string())?;
    let payload_bytes = snapshot
        .canonical_bytes()
        .map_err(|error| error.to_string())?
        .len();
    let sheets = snapshot
        .schematic
        .as_ref()
        .map(|section| {
            section
                .sheets
                .iter()
                .map(|sheet| sheet.name.clone())
                .collect()
        })
        .unwrap_or_default();
    let plots = snapshot
        .figures
        .iter()
        .filter(|figure| {
            matches!(
                figure.content,
                rspice_publication_contract::FigureContent::Plot(_)
            )
        })
        .map(|figure| figure.title.clone())
        .collect();
    let (analyses, datasets, measurements) = snapshot
        .results
        .as_ref()
        .map(|results| {
            (
                results.analyses.len(),
                results.datasets.len(),
                results.measurements.len(),
            )
        })
        .unwrap_or((0, 0, 0));
    Ok(PublishScope {
        sheets,
        plots,
        netlist_title: snapshot
            .netlist
            .as_ref()
            .map(|netlist| netlist.title().to_owned()),
        analyses,
        datasets,
        measurements,
        payload_bytes,
    })
}

fn render_scope_column(ui: &mut egui::Ui, scope: &PublishScope) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    section_head(ui, "Scope");
    for sheet in &scope.sheets {
        scope_row(ui, sheet, "schematic · SVG scene");
    }
    for plot in &scope.plots {
        scope_row(ui, plot, "plot · SVG scene");
    }
    if let Some(title) = &scope.netlist_title {
        scope_row(ui, "Generated netlist", title);
    }
    if scope.analyses + scope.datasets + scope.measurements > 0 {
        scope_row(
            ui,
            "Results",
            &format!(
                "{} analyses · {} datasets · {} measurements",
                scope.analyses, scope.datasets, scope.measurements
            ),
        );
    }
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(format!(
            "Payload {} of {}",
            format_bytes(scope.payload_bytes),
            format_bytes(rspice_publication_contract::MAX_PUBLICATION_SNAPSHOT_BYTES),
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
        .color(c.text),
    );
}

fn render_page_column(
    ui: &mut egui::Ui,
    dialog: &mut PublishWebDialogState,
    bound: bool,
    workspaces: &[crate::services::cloud_account::WorkspaceSummary],
    live_url: Option<&str>,
) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    section_head(ui, "Page");

    field_label(ui, "Title");
    ui.add(
        egui::TextEdit::singleline(&mut dialog.title)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .desired_width(f32::INFINITY),
    );
    field_label(ui, "Author credit");
    ui.add(
        egui::TextEdit::singleline(&mut dialog.author_credit)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .desired_width(f32::INFINITY)
            .hint_text("shown on the page · leave empty for Anonymous"),
    );
    field_label(ui, "Description");
    ui.add(
        egui::TextEdit::multiline(&mut dialog.description)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .desired_rows(2)
            .desired_width(f32::INFINITY),
    );

    field_label(ui, "Content license");
    {
        use rspice_publication_contract::ContentLicense;
        egui::ComboBox::from_id_salt("publish-web-license")
            .width(ui.available_width())
            .selected_text(dialog.license.display_name())
            .show_ui(ui, |ui| {
                for license in [
                    ContentLicense::CernOhlP2,
                    ContentLicense::CcBy40,
                    ContentLicense::AllRightsReserved,
                ] {
                    ui.selectable_value(&mut dialog.license, license, license.display_name());
                }
            });
    }

    if !bound {
        field_label(ui, "Visibility");
        egui::ComboBox::from_id_salt("publish-web-visibility")
            .width(ui.available_width())
            .selected_text(visibility_label(dialog.public_visibility))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut dialog.public_visibility,
                    false,
                    visibility_label(false),
                );
                ui.selectable_value(&mut dialog.public_visibility, true, visibility_label(true));
            });
        if workspaces.len() > 1 {
            field_label(ui, "Workspace");
            let selected = workspaces
                .get(dialog.workspace_index)
                .or_else(|| workspaces.first())
                .map(|workspace| workspace.name.clone())
                .unwrap_or_default();
            egui::ComboBox::from_id_salt("publish-web-workspace")
                .width(ui.available_width())
                .selected_text(selected)
                .show_ui(ui, |ui| {
                    for (index, workspace) in workspaces.iter().enumerate() {
                        ui.selectable_value(
                            &mut dialog.workspace_index,
                            index,
                            workspace.name.clone(),
                        );
                    }
                });
        }
    }

    ui.add_space(6.0);
    let url_value = live_url.unwrap_or("assigned at publish");
    ui.label(
        egui::RichText::new(format!("URL  {url_value}"))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(if live_url.is_some() {
                c.text
            } else {
                c.text_faint
            }),
    );
}

fn render_leaves_machine(ui: &mut egui::Ui, scope: &PublishScope) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    section_head(ui, "Leaves this machine");
    let documents = scope.sheets.len() + scope.plots.len();
    kv_row(
        ui,
        &format!(
            "{documents} scenes · {}netlist deck text · result data as listed",
            if scope.netlist_title.is_some() {
                ""
            } else {
                "no "
            }
        ),
        &format_bytes(scope.payload_bytes),
        c.text,
    );
    kv_row(
        ui,
        "Page title, description, author credit",
        "included",
        c.text,
    );
    kv_row(ui, "Local file paths · account email", "not included", c.ok);
}

fn render_progress(ui: &mut egui::Ui, publish_state: Option<&PublishState>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    match publish_state {
        None | Some(PublishState::Publishing) => {
            ui.horizontal(|ui| {
                ui.add(egui::Spinner::new().size(14.0));
                ui.label(
                    egui::RichText::new("Uploading the snapshot and sealing the circuit revision…")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text),
                );
            });
        }
        Some(PublishState::AwaitingPage { url_path }) => {
            ui.horizontal(|ui| {
                ui.add(egui::Spinner::new().size(14.0));
                ui.label(
                    egui::RichText::new("Publication created — the page render is preparing.")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text),
                );
            });
            ui.add_space(4.0);
            mono_row(ui, url_path, c.text_dim);
        }
        Some(PublishState::Live { url_path }) => {
            ui.label(
                egui::RichText::new("The page is live.")
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(c.ok),
            );
            ui.add_space(4.0);
            mono_row(ui, url_path, c.text);
        }
        Some(PublishState::Failed { message }) => {
            // Rendered by the compose pass; kept for exhaustiveness if the
            // executor reports failure between frames.
            note(ui, message, c.err);
        }
    }
}

fn resolve_note_font() -> egui::FontId {
    theme::sans(tokens::FS_0, FontWeight::Regular)
}

fn note(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    ui.label(
        egui::RichText::new(text)
            .font(resolve_note_font())
            .color(color),
    );
}

fn section_head(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_dim),
    );
    ui.add_space(4.0);
}

fn field_label(ui: &mut egui::Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

fn scope_row(ui: &mut egui::Ui, name: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(name)
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(c.text),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(detail)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(c.text_faint),
            );
        });
    });
}

fn kv_row(ui: &mut egui::Ui, key: &str, value: &str, value_color: egui::Color32) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(key)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.text_dim),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(value_color),
            );
        });
    });
}

fn mono_row(ui: &mut egui::Ui, text: &str, color: egui::Color32) {
    ui.label(
        egui::RichText::new(text)
            .font(theme::mono(tokens::FS_1, FontWeight::Regular))
            .color(color),
    );
}

fn visibility_label(public: bool) -> &'static str {
    if public {
        "Public · listed and indexable"
    } else {
        "Unlisted · anyone with the link"
    }
}

/// Binary-scaled size with one decimal, matching the contract's MiB budget.
fn format_bytes(bytes: usize) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    let bytes = bytes as f64;
    if bytes >= MIB {
        format!("{:.1} MiB", bytes / MIB)
    } else if bytes >= KIB {
        format!("{:.1} KiB", bytes / KIB)
    } else {
        format!("{bytes:.0} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_formatting_is_binary_scaled() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(14 * 1024 + 205), "14.2 KiB");
        assert_eq!(format_bytes(64 * 1024 * 1024), "64.0 MiB");
    }

    #[test]
    fn target_resolution_prefers_the_recorded_binding() {
        let workspace = uuid::Uuid::from_u128(7);
        let circuit = uuid::Uuid::from_u128(9);
        let target = resolve_target(
            Some((workspace.to_string(), circuit.to_string())),
            &[],
            0,
            true,
        )
        .expect("bound target resolves");
        assert_eq!(
            target,
            PublishTarget::ExistingCircuit {
                workspace_id: workspace,
                circuit_id: circuit,
            }
        );
    }

    #[test]
    fn first_publish_needs_a_workspace_and_honors_visibility() {
        assert!(resolve_target(None, &[], 0, false).is_err());
        let workspace = uuid::Uuid::from_u128(11);
        let list = vec![crate::services::cloud_account::WorkspaceSummary {
            id: workspace.to_string(),
            name: "Personal".to_owned(),
        }];
        let target = resolve_target(None, &list, 0, true).expect("workspace target resolves");
        assert_eq!(
            target,
            PublishTarget::NewCircuit {
                workspace_id: workspace,
                public: true,
            }
        );
    }

    #[test]
    fn empty_project_scope_reports_the_builder_refusal() {
        let state = AppState::default();
        let scope = build_scope(&state, "Untitled project", "");
        let reason = scope.expect_err("empty project cannot publish");
        assert!(reason.contains("no schematic"), "{reason}");
    }
}

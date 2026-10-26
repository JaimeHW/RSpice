//! Publish to web — the immutable-snapshot publication transaction.
//!
//! Compose shows exactly what leaves this machine: the scope inventory comes
//! from a real snapshot build at open time, and the payload figure is the
//! canonical byte count against the contract's hard cap. Submit rebuilds the
//! snapshot with the confirmed page fields so the published bytes are always
//! the bytes reviewed here, then hands one typed request to the cloud
//! session executor; progress renders from its owned snapshot.

use egui::Context;

use super::live_session::{field_label, section_head};
use crate::diagnostics::{LogSeverity, LogSource};
use crate::services::cloud_account::{
    CLOUD_PUBLISHING_FEATURE, CloudAccountAvailability, CloudAccountService, PublishRequest,
    PublishState, PublishTarget,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, Problem};
use crate::workbench::app::dialogs::review_primitives::problem_note;
use crate::workbench::app::dialogs::unpublish_web::{
    UnpublishWebDialogState, open_unpublish_web_dialog,
};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::{
    HardcopyPort, PublicationDraft, publication_timestamp_utc,
};
use crate::workbench::framework::ports::ResultsPort;
use crate::workbench::state::WorkbenchSession;
use rspice_app_runs::workbench::slots::runs::RunsState;
use rspice_publication_contract::PublicationSnapshot;

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
    /// Retained imported model sources that may contribute text to the
    /// published effective deck.
    pub imported_model_sources: usize,
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
    /// Fixed when the composer opens so the reviewed canonical bytes do not
    /// change underneath the user while they edit page fields.
    pub created_utc: String,
    pub overview_narrative: String,
    pub specification_label: String,
    pub specification_value: String,
    pub specification_unit: String,
    pub include_schematic: bool,
    pub include_results: bool,
    pub include_netlist: bool,
    /// Explicit confirmation required when the published deck may contain
    /// retained imported model source text.
    pub imported_models_acknowledged: bool,
    pub activate_featured: bool,
    /// Selected workspace for a first publish (index into the session list).
    pub workspace_index: usize,
    pub scope: Option<Result<PublishScope, String>>,
    pub submitted: bool,
}

/// What a review build reads: the open project, and the hardcopy feature's
/// port with the parts it builds the snapshot from beside its own state.
#[derive(Clone, Copy)]
pub(crate) struct SnapshotSource<'a> {
    pub(crate) hardcopy: &'a dyn HardcopyPort,
    pub(crate) model: &'a Model,
    pub(crate) workbench: &'a WorkbenchSession,
    pub(crate) results: &'a dyn ResultsPort,
    pub(crate) runs: &'a RunsState,
}

impl<'a> SnapshotSource<'a> {
    /// The snapshot `draft` describes, as the hardcopy feature builds it.
    fn build(self, draft: &PublicationDraft) -> Result<PublicationSnapshot, String> {
        self.hardcopy.build_publication_snapshot(
            self.model,
            self.workbench,
            self.results,
            self.runs,
            draft,
        )
    }
}

/// Open `dialog` with a fresh review build of `source` and the identity
/// `account` has signed in as the default author credit. No-op unless this
/// build can publish.
pub(crate) fn open_publish_web_dialog(
    dialog: &mut PublishWebDialogState,
    account: &mut CloudAccountService,
    source: SnapshotSource<'_>,
) {
    if !matches!(account.availability(), CloudAccountAvailability::Native)
        || !account
            .snapshot()
            .cloud_feature_enabled(CLOUD_PUBLISHING_FEATURE)
    {
        return;
    }
    let author = account
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
    let project = &source.model.workspace.content.project;
    let project_name = project.name().trim().to_owned();
    let title = if project_name.is_empty() {
        "Untitled project".to_owned()
    } else {
        project_name
    };
    let mut opened = PublishWebDialogState {
        open: true,
        title,
        author_credit: author,
        created_utc: publication_timestamp_utc(),
        include_schematic: true,
        include_results: true,
        include_netlist: true,
        ..Default::default()
    };
    opened.scope = Some(build_scope(source, &publication_draft(&opened)));
    *dialog = opened;
    if let Some(binding) = project.cloud_publication() {
        account.refresh_publications(binding.circuit_id().to_owned());
    }
}

/// What the publish dialog reads and writes: its own fields, the unpublish
/// dialog it hands a live page to, the account service that publishes, the
/// model it builds from and logs to, the session it prompts in, and what the
/// hardcopy feature, the results workspace and the run tier lend the build.
pub(crate) struct PublishWebParts<'a> {
    pub dialog: &'a mut PublishWebDialogState,
    pub unpublish: &'a mut UnpublishWebDialogState,
    pub account: &'a mut CloudAccountService,
    pub hardcopy: &'a dyn HardcopyPort,
    pub model: &'a mut Model,
    pub workbench: &'a mut WorkbenchSession,
    pub results: &'a dyn ResultsPort,
    pub runs: &'a RunsState,
}

impl PublishWebParts<'_> {
    /// What a review build reads, lent from these parts.
    fn source(&self) -> SnapshotSource<'_> {
        SnapshotSource {
            hardcopy: self.hardcopy,
            model: &*self.model,
            workbench: &*self.workbench,
            results: self.results,
            runs: self.runs,
        }
    }
}

/// Draw the publish dialog for this frame, when it is open, and carry out
/// what the reader chose in it.
pub(crate) fn render_publish_web_dialog(parts: &mut PublishWebParts<'_>, ctx: &Context) {
    if !parts.dialog.open {
        return;
    }
    if !parts.dialog.submitted && parts.dialog.scope.is_none() {
        let draft = publication_draft(parts.dialog);
        parts.dialog.scope = Some(build_scope(parts.source(), &draft));
    }

    // Server-side facts, cloned before the body borrows the dialog's state.
    let publish_state = parts.account.snapshot().publish.clone();
    let publications = parts.account.snapshot().publications.clone();
    let workspaces = parts.account.snapshot().workspaces.clone();
    let binding = parts
        .model
        .workspace
        .content
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

    let submitted = parts.dialog.submitted;
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

    let scope_ok = matches!(parts.dialog.scope, Some(Ok(_)));
    let imported_models_confirmed = parts
        .dialog
        .scope
        .as_ref()
        .and_then(|scope| scope.as_ref().ok())
        .is_none_or(|scope| {
            scope.imported_model_sources == 0 || parts.dialog.imported_models_acknowledged
        });
    let compose_ready = scope_ok
        && imported_models_confirmed
        && !parts.dialog.title.trim().is_empty()
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
        format!("published versions never change · circuit {circuit}")
    } else {
        "published versions never change · earlier versions keep their URLs".to_owned()
    };

    let mut dialog = Dialog::new("Publish to web", &primary)
        .description(
            "Review the snapshot that leaves this machine, then publish it as a new page \
             version.",
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

    let dialog_state = &mut *parts.dialog;
    let choice = dialog.show(ctx, |ui| {
        let t = Tokens::get(ui.ctx());
        let c = t.color;

        if submitted {
            render_progress(ui, publish_state.as_ref());
            return;
        }

        if let Some(message) = &failed_message {
            problem_note(ui, Problem::Failure, message);
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
                ui.columns(2, |columns| {
                    section_head(&mut columns[0], "Scope");
                    problem_note(
                        &mut columns[0],
                        Problem::Failure,
                        &format!("Nothing can be published: {reason}"),
                    );
                    render_page_column(
                        &mut columns[1],
                        dialog_state,
                        binding.is_some(),
                        &workspaces,
                        live_publication.as_ref().map(|live| live.url_path.as_str()),
                    );
                });
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
                render_leaves_machine(ui, &scope, dialog_state);
                ui.add_space(tokens::SP_4);
                note(
                    ui,
                    "Everything listed becomes publicly retrievable at the page URL. \
                     Each publish adds a new version and never rewrites an earlier one.",
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
            // receipt still binds the project and reports to the console,
            // and a failure is reported once this dialog is not there to
            // show it. A publish that already ended was shown here.
            parts.dialog.open = false;
            parts.dialog.submitted = in_flight;
        }
        DialogChoice::Secondary => {
            if secondary_unpublishes {
                if let (Some((_, circuit_id)), Some(live)) = (&binding, &live_publication) {
                    parts.dialog.open = false;
                    open_unpublish_web_dialog(
                        parts.unpublish,
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
                parts.dialog.open = false;
            } else if !in_flight {
                submit_publish(parts, binding, &workspaces);
            }
        }
    }
}

/// Drain the executor's publish receipt: bind the project to its page
/// lineage and report the URL. Runs every frame from the app loop so a
/// closed dialog never loses the binding, or the failure.
pub(crate) fn record_publish_receipt(
    dialog: &mut PublishWebDialogState,
    account: &mut CloudAccountService,
    model: &mut Model,
) {
    report_unwatched_publish_failure(dialog, account, model);
    let Some(receipt) = account.take_publish_receipt() else {
        return;
    };
    match crate::state::ProjectCloudPublicationBinding::new(
        receipt.workspace_id.clone(),
        receipt.circuit_id.clone(),
    ) {
        Ok(binding) => {
            if let Err(error) = model.workspace.content.bind_cloud_publication(binding) {
                model.record(
                    LogSeverity::Warning,
                    LogSource::Studio,
                    format!(
                        "Published, but the project could not record which page it was \
                         published to: {error}"
                    ),
                );
            }
        }
        Err(error) => {
            model.record(
                LogSeverity::Warning,
                LogSource::Studio,
                format!("Published, but the page the service returned is unusable: {error}"),
            );
        }
    }
    model.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "Published {}; the page is being prepared.",
            receipt.url_path
        ),
    );
}

/// Say that a publish failed after its dialog was closed.
///
/// The dialog shows a failure while it is open. Once it is closed nothing
/// else would, and a publish the user asked for must not fail silently.
fn report_unwatched_publish_failure(
    dialog: &mut PublishWebDialogState,
    account: &CloudAccountService,
    model: &mut Model,
) {
    if dialog.open || !dialog.submitted {
        return;
    }
    let Some(PublishState::Failed { message }) = &account.snapshot().publish else {
        return;
    };
    let message = message.clone();
    dialog.submitted = false;
    model.report_failure("Publish failed", message);
}

/// Rebuild the snapshot with the confirmed page fields and hand the
/// publish request to the executor.
fn submit_publish(
    parts: &mut PublishWebParts<'_>,
    binding: Option<(String, String)>,
    workspaces: &[crate::services::cloud_account::WorkspaceSummary],
) {
    let include_netlist = parts.dialog.include_netlist;
    let imported_models_acknowledged = parts.dialog.imported_models_acknowledged;
    let imported_model_sources = imported_model_source_count(parts.model, include_netlist);
    if imported_model_sources > 0 && !imported_models_acknowledged {
        parts
            .workbench
            .prompt("Publish stopped: confirm that imported model source text may be uploaded.");
        return;
    }
    let dialog = &*parts.dialog;
    let title = dialog.title.trim().to_owned();
    let description = dialog.description.trim().to_owned();
    let public = dialog.public_visibility;
    let workspace_index = dialog.workspace_index;

    let target = match resolve_target(binding, workspaces, workspace_index, public) {
        Ok(target) => target,
        Err(message) => {
            parts.dialog.scope = Some(Err(message));
            return;
        }
    };
    let draft = publication_draft(dialog);
    let built = parts.source().build(&draft).and_then(|snapshot| {
        snapshot
            .canonical_bytes()
            .map_err(|error| error.to_string())
    });
    match built {
        Ok(snapshot_bytes) => {
            parts.account.publish_snapshot(PublishRequest {
                target,
                title,
                description,
                snapshot_bytes,
            });
            parts.dialog.submitted = true;
        }
        Err(message) => {
            parts.dialog.scope = Some(Err(message));
            parts.model.record(
                LogSeverity::Error,
                LogSource::Studio,
                "Publish stopped: the project could not be packaged for the page.",
            );
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

pub(crate) fn publication_draft(dialog: &PublishWebDialogState) -> PublicationDraft {
    PublicationDraft {
        title: dialog.title.trim().to_owned(),
        description: dialog.description.trim().to_owned(),
        author_display: if dialog.author_credit.trim().is_empty() {
            "Anonymous".to_owned()
        } else {
            dialog.author_credit.trim().to_owned()
        },
        created_utc: dialog.created_utc.clone(),
        license: dialog.license,
        overview_narrative: dialog.overview_narrative.trim().to_owned(),
        specification_label: dialog.specification_label.trim().to_owned(),
        specification_value: dialog.specification_value.trim().to_owned(),
        specification_unit: dialog.specification_unit.trim().to_owned(),
        include_schematic: dialog.include_schematic,
        include_results: dialog.include_results,
        include_netlist: dialog.include_netlist,
        activate_featured: dialog.activate_featured,
    }
}

/// Build the reviewed scope from the same draft the final submission uses.
pub(crate) fn build_scope(
    source: SnapshotSource<'_>,
    draft: &PublicationDraft,
) -> Result<PublishScope, String> {
    let snapshot = source.build(draft)?;
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
        imported_model_sources: imported_model_source_count(
            source.model,
            snapshot.netlist.is_some(),
        ),
        payload_bytes,
    })
}

pub(crate) fn imported_model_source_count(model: &Model, include_netlist: bool) -> usize {
    if !include_netlist {
        return 0;
    }
    model
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter(|library| {
            matches!(
                library.source_authority,
                crate::state::model_library::ModelSourceAuthority::RetainedImport { .. }
            )
        })
        .count()
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
    let reviewed_draft = publication_draft(dialog);
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

    field_label(ui, "Published content");
    ui.horizontal_wrapped(|ui| {
        ui.checkbox(&mut dialog.include_schematic, "Schematic");
        ui.checkbox(&mut dialog.include_results, "Results and plots");
        ui.checkbox(&mut dialog.include_netlist, "Netlist");
    });

    field_label(ui, "Overview narrative");
    ui.add(
        egui::TextEdit::multiline(&mut dialog.overview_narrative)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .desired_rows(3)
            .desired_width(f32::INFINITY)
            .hint_text("What this circuit does, its design intent, and key conclusions"),
    );

    field_label(ui, "Featured specification");
    ui.columns(3, |columns| {
        columns[0].add(
            egui::TextEdit::singleline(&mut dialog.specification_label)
                .desired_width(f32::INFINITY)
                .hint_text("Label"),
        );
        columns[1].add(
            egui::TextEdit::singleline(&mut dialog.specification_value)
                .desired_width(f32::INFINITY)
                .hint_text("Value"),
        );
        columns[2].add(
            egui::TextEdit::singleline(&mut dialog.specification_unit)
                .desired_width(f32::INFINITY)
                .hint_text("Unit"),
        );
    });

    ui.checkbox(
        &mut dialog.activate_featured,
        "Open the featured figure in interactive mode",
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

    if publication_draft(dialog) != reviewed_draft {
        dialog.scope = None;
    }
}

fn render_leaves_machine(
    ui: &mut egui::Ui,
    scope: &PublishScope,
    dialog: &mut PublishWebDialogState,
) {
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
    );
    kv_row(
        ui,
        "Page copy, specification, author credit, and presentation choices",
        "included",
    );
    kv_row(ui, "Local file paths · account email", "not included");
    if scope.imported_model_sources > 0 {
        ui.add_space(tokens::SP_4);
        problem_note(
            ui,
            Problem::Warning,
            &format!(
                "The netlist may contain text from {} imported model source{}.",
                scope.imported_model_sources,
                if scope.imported_model_sources == 1 {
                    ""
                } else {
                    "s"
                }
            ),
        );
        ui.checkbox(
            &mut dialog.imported_models_acknowledged,
            "I confirm I may upload and publish that model source text",
        );
    }
}

fn render_progress(ui: &mut egui::Ui, publish_state: Option<&PublishState>) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    match publish_state {
        None | Some(PublishState::Publishing) => {
            ui.horizontal(|ui| {
                crate::ui::widgets::busy_mark(ui, 14.0);
                ui.label(
                    egui::RichText::new("Uploading the snapshot")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text),
                );
            });
        }
        Some(PublishState::AwaitingPage { url_path }) => {
            ui.horizontal(|ui| {
                crate::ui::widgets::busy_mark(ui, 14.0);
                ui.label(
                    egui::RichText::new("Rendering the page")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text),
                );
            });
            ui.add_space(4.0);
            mono_row(ui, url_path, c.text_dim);
        }
        Some(PublishState::Live { url_path }) => {
            // The URL is the fact the reader came for; "live" is the state it
            // is in, and neither needs a tone to say so.
            ui.label(
                egui::RichText::new("The page is live.")
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(c.text),
            );
            ui.add_space(4.0);
            mono_row(ui, url_path, c.text);
        }
        Some(PublishState::Failed { message }) => {
            // Rendered by the compose pass; kept for exhaustiveness if the
            // executor reports failure between frames.
            problem_note(ui, Problem::Failure, message);
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

fn kv_row(ui: &mut egui::Ui, key: &str, value: &str) {
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
                    .color(c.text),
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

/// The one public byte spelling, over the contract's `usize` counts.
///
/// Delegates so the payload gauge and every storage figure elsewhere in the
/// product describe the same byte count with the same number.
fn format_bytes(bytes: usize) -> String {
    rspice_simulation_contract::run_set::format_bytes(bytes as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_formatting_is_binary_scaled() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(14 * 1024 + 205), "14.20 KiB");
        assert_eq!(format_bytes(64 * 1024 * 1024), "64.00 MiB");
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
}

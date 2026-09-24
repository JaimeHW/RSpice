//! Signed PDK technology-package administration.
//!
//! This surface projects only validated package state. Persisted archives do
//! not regain runtime authority after restart until their signatures,
//! artifacts, contracts, trust roots, and audit chain have all been checked.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use egui::{Align, Color32, Frame, Grid, Layout, RichText, ScrollArea, Sense, Stroke, Ui};
#[cfg(not(target_arch = "wasm32"))]
use std::collections::VecDeque;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Mutex, OnceLock};

use crate::diagnostics::ConsoleMessage;
use crate::state::pdk_config::{
    MAX_PDK_ARCHIVE_BYTES, PdkAdministrativeAuthority, PdkDisplayFillStyle,
    PdkDisplayProfileBinding, PdkDisplayProfileDraft, PdkDisplayProfileScope, PdkExecutionTarget,
    PdkExtractionContract, PdkExtractionQuantity, PdkLayerAlias, PdkLayerKind, PdkLayerPurposeRef,
    PdkRecognitionContract, PdkRecognitionQualificationVector, PdkRecognitionTerminal,
    PdkStreamMapEntry, PdkTechnologyArtifactKind, PdkTechnologyAuditAction, PdkTechnologyBinding,
    PdkTechnologyDiffArea, PdkTechnologyDiffEntry, PdkTechnologyDiffError, PdkTechnologyDiffImpact,
    PdkTechnologyDiffKind, PdkTechnologyDraft, PdkTechnologyLayer, PdkTechnologyManifest,
    PdkTechnologyRevisionDiff, PdkTrustAuditAction, PdkViaDefinition, ProjectPdkCallbackReceipt,
    TrustedPdkPublisherKey, ValidatedPdkTechnologyPackage,
};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

const VIEW_STATE_ID: &str = "pdk-technology-admin-view-state";
const COMPACT_BREAKPOINT: f32 = 760.0;
const MAX_TRUST_KEY_BASE64_INPUT: usize = 256;

#[cfg(target_arch = "wasm32")]
const BROWSER_PDK_IMPORT_PROTOCOL_VERSION: u16 = 1;

#[cfg(target_arch = "wasm32")]
type BrowserPackageImport = Result<Option<BrowserPackageImportCandidate>, String>;

#[cfg(target_arch = "wasm32")]
struct BrowserPackageImportCandidate {
    base: crate::state::pdk_config::PdkConfig,
    payload: BrowserPdkImportPayload,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkImportMetadata {
    protocol_version: u16,
    config: crate::state::pdk_config::PdkConfig,
    authority: PdkAdministrativeAuthority,
    reason: String,
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkImportPayload {
    protocol_version: u16,
    config: crate::state::pdk_config::PdkConfig,
    validated_packages: Vec<ValidatedPdkTechnologyPackage>,
    package_id: String,
    revision: String,
    sequence: u64,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PACKAGE_IMPORTS:
        std::cell::RefCell<std::collections::VecDeque<BrowserPackageImport>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

#[cfg(not(target_arch = "wasm32"))]
struct NativePackageImport {
    base: crate::state::pdk_config::PdkConfig,
    result: Result<NativePackageImportCandidate, String>,
}

#[cfg(not(target_arch = "wasm32"))]
struct NativePackageImportCandidate {
    config: crate::state::pdk_config::PdkConfig,
    package_id: String,
    revision: String,
    sequence: u64,
}

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_PACKAGE_IMPORTS: OnceLock<Mutex<VecDeque<NativePackageImport>>> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
fn native_package_imports() -> &'static Mutex<VecDeque<NativePackageImport>> {
    NATIVE_PACKAGE_IMPORTS.get_or_init(|| Mutex::new(VecDeque::new()))
}

#[cfg(not(target_arch = "wasm32"))]
fn prepare_native_package_import(
    base: &crate::state::pdk_config::PdkConfig,
    bytes: &[u8],
    authority: &PdkAdministrativeAuthority,
    reason: &str,
) -> Result<NativePackageImportCandidate, String> {
    let mut config = base.clone();
    let receipt = config
        .technology_registry
        .install_archive_bytes(bytes, &config.publisher_trust_store, authority, reason)
        .map_err(|error| error.to_string())?;
    Ok(NativePackageImportCandidate {
        config,
        package_id: receipt.target.package_id,
        revision: receipt.target.revision,
        sequence: receipt.sequence,
    })
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum AdminSection {
    #[default]
    Package,
    Compare,
    Layers,
    Display,
    StreamMaps,
    Connectivity,
    Recognition,
    Extraction,
    Resources,
    TrustAudit,
}

impl AdminSection {
    const ALL: [Self; 10] = [
        Self::Package,
        Self::Compare,
        Self::Layers,
        Self::Display,
        Self::StreamMaps,
        Self::Connectivity,
        Self::Recognition,
        Self::Extraction,
        Self::Resources,
        Self::TrustAudit,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Package => "Package",
            Self::Compare => "Compare",
            Self::Layers => "Layers",
            Self::Display => "Display",
            Self::StreamMaps => "Stream maps",
            Self::Connectivity => "Vias",
            Self::Recognition => "Recognition",
            Self::Extraction => "Extraction",
            Self::Resources => "Resources",
            Self::TrustAudit => "Trust & audit",
        }
    }
}

#[derive(Debug, Clone, Default)]
struct AdminViewState {
    section: AdminSection,
    selected: Option<(String, String)>,
    compare_against: Option<(String, String)>,
    actor_id: String,
    authority_id: String,
    reason: String,
    trust_publisher_id: String,
    trust_key_id: String,
    trust_key_base64: String,
    selected_display_profile: Option<(String, u64)>,
    display_draft: Option<PdkDisplayProfileDraft>,
    display_draft_dirty: bool,
    technology_draft: Option<PdkTechnologyDraft>,
    technology_draft_dirty: bool,
    discard_technology_draft_armed: bool,
    technology_draft_error: Option<String>,
    package_import_in_progress: bool,
}

enum AdminAction {
    ChooseImport,
    ReportError(String),
    Revalidate,
    SaveTechnologyDraft(PdkTechnologyDraft),
    DiscardTechnologyDraft,
    ExportTechnologyDraft(PdkTechnologyDraft),
    Activate {
        binding: PdkTechnologyBinding,
        rollback: bool,
    },
    ProvisionTrustKey(TrustedPdkPublisherKey),
    RevokeTrustKey {
        publisher_id: String,
        key_id: String,
    },
    PublishDisplayProfile(PdkDisplayProfileDraft),
    ActivateDisplayProfile {
        binding: PdkDisplayProfileBinding,
        rollback: bool,
    },
    RunProjectCallback {
        callback_id: String,
    },
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
enum AdminCommitEffect {
    None,
    ClearTrustDraft {
        publisher_id: String,
        key_id: String,
    },
    SelectDisplayProfile {
        profile_id: String,
        revision: u64,
    },
    TechnologyDraftSaved,
    TechnologyDraftDiscarded,
}

#[derive(Clone)]
struct RegistrySnapshot {
    packages: Vec<ValidatedPdkTechnologyPackage>,
    active: Option<PdkTechnologyBinding>,
    audit: Vec<crate::state::pdk_config::PdkTechnologyAuditReceipt>,
    validation_errors: Vec<String>,
    installed_count: usize,
    trust_keys: Vec<crate::state::pdk_config::TrustedPdkPublisherKey>,
    trust_audit: Vec<crate::state::pdk_config::PdkTrustAuditReceipt>,
    display_profiles: Vec<crate::state::pdk_config::PdkDisplayProfileRevision>,
    active_display_profile: Option<PdkDisplayProfileBinding>,
    display_audit: Vec<crate::state::pdk_config::PdkDisplayProfileAuditReceipt>,
    display_validation_error: Option<String>,
    runtime_display_profile_valid: bool,
    runtime_ready: bool,
    active_trusted: bool,
    project_signed_package: Option<(PdkTechnologyBinding, crate::product::ContentDigest)>,
    project_callback_blocker: Option<String>,
    project_callback_receipts: Vec<ProjectPdkCallbackReceipt>,
    technology_draft: Option<PdkTechnologyDraft>,
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    #[cfg(not(target_arch = "wasm32"))]
    poll_native_package_imports(ui.ctx(), app);
    #[cfg(target_arch = "wasm32")]
    poll_browser_package_imports(ui.ctx(), app);

    let mut view = ui
        .ctx()
        .data(|data| data.get_temp::<AdminViewState>(egui::Id::new(VIEW_STATE_ID)))
        .unwrap_or_default();
    if view.technology_draft.is_none()
        && app
            .state
            .workbench
            .pdk_technology_authoring
            .working_draft
            .is_some()
    {
        view.technology_draft = app
            .state
            .workbench
            .pdk_technology_authoring
            .working_draft
            .clone();
        view.technology_draft_dirty = app.state.workbench.pdk_technology_authoring.dirty;
    }
    let snapshot = registry_snapshot(app);
    reconcile_selection(&mut view, &snapshot);
    reconcile_technology_draft(&mut view, &snapshot);

    let tokens = Tokens::get(ui.ctx());
    let size = ui.available_size().max(egui::Vec2::splat(1.0));
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_app);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            "PDK technology administration",
        )
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label("PDK technology administration");
    });

    let mut root = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::top_down(Align::Min)),
    );
    #[cfg(target_arch = "wasm32")]
    let persistence_ready = app.browser_pdk_persistence_ready();
    #[cfg(not(target_arch = "wasm32"))]
    let persistence_ready = true;
    root.set_clip_rect(rect);
    root.spacing_mut().item_spacing.y = 0.0;

    header(&mut root, &snapshot);
    metrics(&mut root, &snapshot);
    #[cfg(target_arch = "wasm32")]
    browser_pdk_storage_status(&mut root, app);

    let mut requested_action = None;
    ScrollArea::vertical()
        .id_salt("pdk-technology-admin-scroll")
        .auto_shrink([false, false])
        .show(&mut root, |ui| {
            if !persistence_ready {
                ui.disable();
            }
            ui.add_space(12.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 4.0;
                for section in AdminSection::ALL {
                    if ui
                        .selectable_label(view.section == section, section.label())
                        .clicked()
                    {
                        view.section = section;
                    }
                }
            });
            ui.add_space(12.0);

            if ui.available_width() < COMPACT_BREAKPOINT {
                package_selector(ui, &snapshot, &mut view);
                ui.add_space(10.0);
                administration_controls(ui, &snapshot, &mut view, &mut requested_action);
                ui.add_space(10.0);
                section_contents(ui, &snapshot, &mut view, &mut requested_action);
            } else {
                ui.columns(2, |columns| {
                    columns[0].set_width((columns[0].available_width() * 0.82).max(260.0));
                    package_selector(&mut columns[0], &snapshot, &mut view);
                    columns[0].add_space(10.0);
                    administration_controls(
                        &mut columns[0],
                        &snapshot,
                        &mut view,
                        &mut requested_action,
                    );
                    section_contents(&mut columns[1], &snapshot, &mut view, &mut requested_action);
                });
            }
            ui.add_space(16.0);
        });

    ui.ctx()
        .data_mut(|data| data.insert_temp(egui::Id::new(VIEW_STATE_ID), view.clone()));
    if let Some(action) = requested_action {
        apply_action(ui.ctx(), app, &mut view, action);
        ui.ctx()
            .data_mut(|data| data.insert_temp(egui::Id::new(VIEW_STATE_ID), view.clone()));
    }
    app.state.workbench.pdk_technology_authoring.working_draft = view.technology_draft.clone();
    app.state.workbench.pdk_technology_authoring.dirty = view.technology_draft_dirty;
}

#[cfg(target_arch = "wasm32")]
fn browser_pdk_storage_status(ui: &mut Ui, app: &mut RSpiceApp) {
    let ready = app.browser_pdk_persistence_ready();
    let retryable = app.browser_pdk_persistence_retryable();
    let degraded = app.browser_pdk_persistence_degraded();
    let tokens = Tokens::get(ui.ctx());
    let color = if ready && !degraded {
        tokens.color.ok
    } else if retryable {
        tokens.color.err
    } else {
        tokens.color.warn
    };
    let response = Frame::new()
        .fill(tokens.color.bg_inset)
        .stroke(Stroke::new(1.0, color))
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.colored_label(
                    color,
                    RichText::new(app.browser_pdk_persistence_status_message()).strong(),
                );
                if retryable
                    && Button::new("Retry storage recovery")
                        .accessible_label("Retry browser PDK storage recovery")
                        .show(ui)
                        .clicked()
                {
                    match app.retry_browser_pdk_persistence(ui.ctx()) {
                        Ok(()) => {
                            app.state.push_user_message(ConsoleMessage::info(
                                "Retrying browser PDK storage recovery.".to_owned(),
                            ));
                            app.state.ui.toasts.info(
                                ui.ctx(),
                                "PDK storage recovery: reloading and validating authoritative browser PDK state.",
                            );
                        }
                        Err(error) => {
                            let message =
                                format!("Browser PDK storage recovery could not start: {error}");
                            app.state
                                .push_user_message(ConsoleMessage::error(message.clone()));
                            app.state.ui.toasts.error_with_title(
                                ui.ctx(),
                                "PDK storage recovery blocked",
                                message,
                            );
                        }
                    }
                }
            });
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(app.browser_pdk_persistence_status_message());
    });
}

fn registry_snapshot(app: &RSpiceApp) -> RegistrySnapshot {
    let registry = &app.state.pdk_config.technology_registry;
    let display_registry = &app.state.pdk_config.display_profile_registry;
    let runtime_display_profile_valid = display_registry.active_binding().is_none()
        || registry
            .active_package()
            .and_then(|package| display_registry.active_for_package(package))
            .is_some();
    let project_signed_package = app
        .state
        .workspace
        .project
        .technology_binding()
        .and_then(|binding| binding.signed_package())
        .map(|pin| {
            (
                PdkTechnologyBinding {
                    package_id: pin.package_id().to_owned(),
                    revision: pin.revision().to_owned(),
                    manifest_digest: pin.manifest_digest(),
                },
                pin.archive_digest(),
            )
        });
    let project_callback_blocker = app.state.validate_project_technology_contract().err();
    RegistrySnapshot {
        packages: registry.validated_packages().to_vec(),
        active: registry.active_binding().cloned(),
        audit: registry.audit().to_vec(),
        validation_errors: registry.validation_errors().to_vec(),
        installed_count: registry.archives().len(),
        trust_keys: app.state.pdk_config.publisher_trust_store.keys.clone(),
        trust_audit: app.state.pdk_config.publisher_trust_store.audit().to_vec(),
        display_profiles: display_registry.revisions().to_vec(),
        active_display_profile: display_registry.active_binding().cloned(),
        display_audit: display_registry.audit().to_vec(),
        display_validation_error: display_registry
            .validate_audit_chain()
            .err()
            .map(|error| error.to_string()),
        runtime_display_profile_valid,
        runtime_ready: registry.runtime_ready(),
        active_trusted: registry.active_package().is_some(),
        project_signed_package,
        project_callback_blocker,
        project_callback_receipts: app.state.workspace.pdk_callback_receipts().to_vec(),
        technology_draft: app.state.pdk_config.technology_draft.clone(),
    }
}

fn reconcile_technology_draft(view: &mut AdminViewState, snapshot: &RegistrySnapshot) {
    if view.technology_draft_dirty {
        return;
    }
    if view.technology_draft != snapshot.technology_draft {
        view.technology_draft.clone_from(&snapshot.technology_draft);
        view.discard_technology_draft_armed = false;
    }
}

fn reconcile_selection(view: &mut AdminViewState, snapshot: &RegistrySnapshot) {
    let selected_exists = view
        .selected
        .as_ref()
        .is_some_and(|(package_id, revision)| {
            snapshot.packages.iter().any(|package| {
                package
                    .manifest()
                    .package_id
                    .eq_ignore_ascii_case(package_id)
                    && package.manifest().revision == *revision
            })
        });
    if !selected_exists {
        view.selected = snapshot
            .active
            .as_ref()
            .map(|binding| (binding.package_id.clone(), binding.revision.clone()))
            .or_else(|| {
                snapshot.packages.first().map(|package| {
                    (
                        package.manifest().package_id.clone(),
                        package.manifest().revision.clone(),
                    )
                })
            });
    }

    let selected = view.selected.as_ref();
    let comparison_exists = view
        .compare_against
        .as_ref()
        .is_some_and(|(package_id, revision)| {
            selected.is_none_or(|(selected_id, selected_revision)| {
                !selected_id.eq_ignore_ascii_case(package_id) || selected_revision != revision
            }) && snapshot.packages.iter().any(|package| {
                package
                    .manifest()
                    .package_id
                    .eq_ignore_ascii_case(package_id)
                    && package.manifest().revision == *revision
            })
        });
    if comparison_exists {
        return;
    }
    view.compare_against = snapshot
        .active
        .as_ref()
        .filter(|binding| {
            selected.is_none_or(|(package_id, revision)| {
                !binding.package_id.eq_ignore_ascii_case(package_id)
                    || binding.revision != *revision
            })
        })
        .map(|binding| (binding.package_id.clone(), binding.revision.clone()))
        .or_else(|| {
            snapshot.packages.iter().rev().find_map(|package| {
                let identity = (
                    package.manifest().package_id.clone(),
                    package.manifest().revision.clone(),
                );
                (selected != Some(&identity)).then_some(identity)
            })
        });
}

fn selected_package<'a>(
    snapshot: &'a RegistrySnapshot,
    view: &AdminViewState,
) -> Option<&'a ValidatedPdkTechnologyPackage> {
    package_by_identity(snapshot, view.selected.as_ref()?)
}

fn package_by_identity<'a>(
    snapshot: &'a RegistrySnapshot,
    identity: &(String, String),
) -> Option<&'a ValidatedPdkTechnologyPackage> {
    let (package_id, revision) = identity;
    snapshot.packages.iter().find(|package| {
        package
            .manifest()
            .package_id
            .eq_ignore_ascii_case(package_id)
            && package.manifest().revision == *revision
    })
}

fn header(ui: &mut Ui, snapshot: &RegistrySnapshot) {
    let tokens = Tokens::get(ui.ctx());
    let compact = ui.available_width() < COMPACT_BREAKPOINT;
    let (status_label, status_color) = if snapshot.runtime_ready {
        if snapshot.active_trusted {
            ("ACTIVE · TRUSTED", tokens.color.ok)
        } else {
            ("NO ACTIVE BINDING", tokens.color.warn)
        }
    } else {
        ("TRUST BLOCKED", tokens.color.err)
    };
    Frame::new()
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .inner_margin(egui::Margin::symmetric(16, 12))
        .show(ui, |ui| {
            if compact {
                header_identity(ui, &tokens);
                ui.add_space(4.0);
                header_runtime_status(ui, status_label, status_color);
            } else {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| header_identity(ui, &tokens));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        header_runtime_status(ui, status_label, status_color);
                    });
                });
            }
        });
}

fn header_identity(ui: &mut Ui, tokens: &Tokens) {
    ui.label(
        RichText::new("PDK · TECHNOLOGY FILE · STREAM MAP · TRUST")
            .small()
            .color(tokens.color.text_faint),
    );
    ui.heading("PDK technology administration");
    const DESCRIPTION: &str = "Validate signed technology packages, inspect exact physical resources, and govern activation with durable audit receipts.";
    let description = ui.label(DESCRIPTION);
    ui.ctx().accesskit_node_builder(description.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(DESCRIPTION);
    });
}

fn header_runtime_status(ui: &mut Ui, label: &str, color: Color32) {
    let response = ui.colored_label(color, RichText::new(label).strong());
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_label(label);
    });
}

fn metrics(ui: &mut Ui, snapshot: &RegistrySnapshot) {
    let active = snapshot.active.as_ref().map_or_else(
        || "None".to_owned(),
        |binding| format!("{} {}", binding.package_id, binding.revision),
    );
    let purposes = snapshot
        .packages
        .iter()
        .map(|package| {
            package
                .manifest()
                .layers
                .iter()
                .map(|layer| layer.purposes.len())
                .sum::<usize>()
        })
        .sum::<usize>();
    let maps = snapshot
        .packages
        .iter()
        .map(|package| package.manifest().stream_map.len())
        .sum::<usize>();
    let values = [
        ("Active binding", active),
        (
            "Installed / trusted",
            format!("{} / {}", snapshot.installed_count, snapshot.packages.len()),
        ),
        ("Layer purposes", purposes.to_string()),
        (
            "Stream maps / trust keys",
            format!("{maps} / {}", snapshot.trust_keys.len()),
        ),
    ];
    let tokens = Tokens::get(ui.ctx());
    Frame::new()
        .fill(tokens.color.bg_app)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .inner_margin(egui::Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (label, value) in values {
                    ui.vertical(|ui| {
                        ui.small(label);
                        ui.label(RichText::new(value).strong().monospace());
                    });
                    ui.separator();
                }
            });
        });
}

fn package_selector(ui: &mut Ui, snapshot: &RegistrySnapshot, view: &mut AdminViewState) {
    panel(ui, "Installed signed revisions", |ui| {
        if !snapshot.validation_errors.is_empty() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                "Runtime trust is blocked. No installed package can execute.",
            );
            for error in &snapshot.validation_errors {
                ui.label(RichText::new(error).small().monospace());
            }
            ui.separator();
        }
        if snapshot.packages.is_empty() {
            ui.label("No currently trusted technology revision is available.");
            if snapshot.installed_count > 0 {
                ui.small(format!(
                    "{} persisted archive(s) remain quarantined.",
                    snapshot.installed_count
                ));
            }
            return;
        }
        for package in &snapshot.packages {
            let manifest = package.manifest();
            let selected = view.selected.as_ref().is_some_and(|(id, revision)| {
                manifest.package_id.eq_ignore_ascii_case(id) && manifest.revision == *revision
            });
            let active = snapshot.active.as_ref().is_some_and(|binding| {
                binding
                    .package_id
                    .eq_ignore_ascii_case(&manifest.package_id)
                    && binding.revision == manifest.revision
                    && binding.manifest_digest == package.manifest_digest()
            });
            let label = if active {
                format!("● {} {} · active", manifest.package_id, manifest.revision)
            } else {
                format!("{} {}", manifest.package_id, manifest.revision)
            };
            if ui.selectable_label(selected, label).clicked() {
                let next = (manifest.package_id.clone(), manifest.revision.clone());
                let previous = view.selected.replace(next.clone());
                if view.compare_against.as_ref() == Some(&next) {
                    view.compare_against = previous.filter(|identity| identity != &next);
                }
            }
            ui.small(format!(
                "{} · {} nm · {} · signer {}/{}",
                manifest.technology_name,
                manifest.process_node_nm,
                manifest.stack_name,
                manifest.publisher_id,
                manifest.signing_key_id
            ));
        }
    });
}

fn administration_controls(
    ui: &mut Ui,
    snapshot: &RegistrySnapshot,
    view: &mut AdminViewState,
    action: &mut Option<AdminAction>,
) {
    panel(ui, "Administrative transaction", |ui| {
        ui.label("Identity and reason are recorded in the immutable hash-chained receipt.");
        ui.add(
            egui::TextEdit::singleline(&mut view.actor_id)
                .hint_text("Actor ID")
                .desired_width(f32::INFINITY),
        );
        ui.add(
            egui::TextEdit::singleline(&mut view.authority_id)
                .hint_text("Authority ID")
                .desired_width(f32::INFINITY),
        );
        ui.add(
            egui::TextEdit::multiline(&mut view.reason)
                .hint_text("Change reason")
                .desired_rows(2)
                .desired_width(f32::INFINITY),
        );
        let has_authority = administrative_fields_ready(view);

        ui.horizontal_wrapped(|ui| {
            let import = Button::new("Import signed package…")
                .accent()
                .enabled(
                    has_authority
                        && !snapshot.trust_keys.is_empty()
                        && !view.package_import_in_progress,
                )
                .accessible_label(if snapshot.trust_keys.is_empty() {
                    "Import signed package. Unavailable: no publisher trust keys are provisioned"
                } else if !has_authority {
                    "Import signed package. Unavailable: actor, authority, and reason are required"
                } else {
                    "Import signed package"
                })
                .show(ui)
                .on_disabled_hover_text(if snapshot.trust_keys.is_empty() {
                    "Provision organization-approved publisher verification keys in PDK configuration first."
                } else {
                    "Actor, authority, and reason are required."
                });
            if import.clicked() {
                *action = Some(AdminAction::ChooseImport);
            }
            if Button::new("Revalidate installed packages").show(ui).clicked() {
                *action = Some(AdminAction::Revalidate);
            }
            let can_compare = view.selected.as_ref().is_some_and(|selected| {
                snapshot.packages.iter().any(|package| {
                    !package
                        .manifest()
                        .package_id
                        .eq_ignore_ascii_case(&selected.0)
                        || package.manifest().revision != selected.1
                })
            });
            let compare = Button::new("Compare signed revisions…")
                .enabled(can_compare)
                .accessible_label(if can_compare {
                    "Compare selected signed PDK revision"
                } else {
                    "Compare selected signed PDK revision. Unavailable: install another trusted revision"
                })
                .show(ui)
                .on_disabled_hover_text("Install another currently trusted revision to compare.");
            if compare.clicked() {
                view.section = AdminSection::Compare;
            }
        });

        let Some(package) = selected_package(snapshot, view) else {
            return;
        };
        let binding = package.binding();
        let active = snapshot.active.as_ref() == Some(&binding);
        let appeared_before = snapshot.audit.iter().any(|receipt| {
            receipt.after_active.as_ref().is_some_and(|candidate| {
                candidate
                    .package_id
                    .eq_ignore_ascii_case(&binding.package_id)
                    && candidate.revision == binding.revision
                    && candidate.manifest_digest == binding.manifest_digest
            })
        });
        let label = if appeared_before {
            "Roll back to selected revision"
        } else {
            "Activate selected revision"
        };
        let runtime_compatibility = package.runtime_compatibility();
        let target_allowed = runtime_compatibility.is_ok();
        let enabled =
            has_authority && !active && snapshot.validation_errors.is_empty() && target_allowed;
        let response = Button::new(label)
            .enabled(enabled)
            .accessible_label(if active {
                "Activate selected revision. Unavailable: this exact revision is already active"
            } else if !has_authority {
                "Activate selected revision. Unavailable: actor, authority, and reason are required"
            } else if !snapshot.validation_errors.is_empty() {
                "Activate selected revision. Unavailable: package trust validation is blocked"
            } else if !target_allowed {
                "Activate selected revision. Unavailable: this package is incompatible with the current runtime"
            } else {
                label
            })
            .show(ui);
        if let Err(error) = runtime_compatibility {
            ui.small(error);
        }
        if response.clicked() {
            *action = Some(AdminAction::Activate {
                binding,
                rollback: appeared_before,
            });
        }
        ui.separator();
        technology_draft_controls(ui, package, view, action);
    });
}

fn technology_draft_controls(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    view: &mut AdminViewState,
    action: &mut Option<AdminAction>,
) {
    ui.label(RichText::new("Unsigned technology authoring").strong());
    let binding = package.binding();
    let draft_matches = view
        .technology_draft
        .as_ref()
        .is_some_and(|draft| draft.baseline.binding == binding);
    if view.technology_draft.is_none() {
        ui.small(
            "Fork the selected immutable revision into one persisted unsigned draft. Activation still requires an externally signed package import.",
        );
        if Button::new("Fork selected revision into draft")
            .show(ui)
            .clicked()
        {
            view.technology_draft = Some(PdkTechnologyDraft::from_package(package));
            view.technology_draft_dirty = true;
            view.discard_technology_draft_armed = false;
        }
        return;
    }
    if !draft_matches {
        let draft = view
            .technology_draft
            .as_ref()
            .expect("draft presence checked above");
        ui.colored_label(
            Tokens::get(ui.ctx()).color.warn,
            format!(
                "Draft '{}' is based on {} {}. Select that exact source revision to edit it.",
                draft.draft_id, draft.baseline.binding.package_id, draft.baseline.binding.revision
            ),
        );
        return;
    }

    let draft = view
        .technology_draft
        .as_mut()
        .expect("matching draft exists");
    ui.horizontal(|ui| {
        ui.label("Candidate revision");
        let mut revision = draft.manifest.revision.clone();
        if ui
            .add(
                egui::TextEdit::singleline(&mut revision)
                    .desired_width(140.0)
                    .hint_text("semantic version"),
            )
            .changed()
        {
            draft.set_revision(revision);
            view.technology_draft_dirty = true;
            view.discard_technology_draft_armed = false;
        }
    });
    let validation = draft.validate_candidate(package);
    match &validation {
        Ok(()) => {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.ok,
                "Candidate manifest is valid.",
            );
        }
        Err(error) => {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                format!("Draft validation: {error}"),
            );
        }
    }
    ui.horizontal_wrapped(|ui| {
        if Button::new("Save draft")
            .accent()
            .enabled(view.technology_draft_dirty)
            .show(ui)
            .on_disabled_hover_text("The working draft has no unsaved changes.")
            .clicked()
        {
            *action = Some(AdminAction::SaveTechnologyDraft(draft.clone()));
        }
        if Button::new("Export for signing…")
            .enabled(validation.is_ok())
            .show(ui)
            .on_disabled_hover_text("Resolve the displayed validation error before export.")
            .clicked()
        {
            *action = Some(AdminAction::ExportTechnologyDraft(draft.clone()));
        }
        let discard_label = if view.discard_technology_draft_armed {
            "Confirm discard draft"
        } else {
            "Discard draft…"
        };
        if Button::new(discard_label).show(ui).clicked() {
            if view.discard_technology_draft_armed {
                *action = Some(AdminAction::DiscardTechnologyDraft);
            } else {
                view.discard_technology_draft_armed = true;
            }
        }
    });
}

fn section_contents(
    ui: &mut Ui,
    snapshot: &RegistrySnapshot,
    view: &mut AdminViewState,
    action: &mut Option<AdminAction>,
) {
    if view.section == AdminSection::TrustAudit {
        trust_audit_section(ui, selected_package(snapshot, view), snapshot, view, action);
        return;
    }
    let Some(package) = selected_package(snapshot, view) else {
        panel(ui, view.section.label(), |ui| {
            ui.label("Select a trusted installed revision to inspect its exact contents.");
        });
        return;
    };
    match view.section {
        AdminSection::Package => package_section(ui, package, snapshot),
        AdminSection::Compare => compare_section(ui, package, snapshot, view),
        AdminSection::Layers => layers_section(ui, package, view),
        AdminSection::Display => display_section(ui, package, snapshot, view, action),
        AdminSection::StreamMaps => stream_section(ui, package, view),
        AdminSection::Connectivity => connectivity_section(ui, package, view),
        AdminSection::Recognition => recognition_section(ui, package, view),
        AdminSection::Extraction => extraction_section(ui, package, view),
        AdminSection::Resources => resources_section(ui, package, snapshot, view, action),
        AdminSection::TrustAudit => unreachable!("trust section handled before package selection"),
    }
}

fn compare_section(
    ui: &mut Ui,
    candidate: &ValidatedPdkTechnologyPackage,
    snapshot: &RegistrySnapshot,
    view: &mut AdminViewState,
) {
    panel(ui, "Exact signed-revision comparison", |ui| {
        ui.label(
            "Compare authenticated package contracts before explicit project migration. No project binding or design data is changed here.",
        );
        let alternatives = snapshot
            .packages
            .iter()
            .filter(|package| package.binding() != candidate.binding())
            .collect::<Vec<_>>();
        if alternatives.is_empty() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "No second currently trusted revision is available for comparison.",
            );
            ui.small(
                "Import and revalidate another signed revision. Quarantined or untrusted archives are never offered as a baseline.",
            );
            return;
        }

        let selected_text = view.compare_against.as_ref().map_or_else(
            || "Select baseline".to_owned(),
            |(package_id, revision)| format!("{package_id} {revision}"),
        );
        let combo = egui::ComboBox::from_id_salt("pdk-revision-diff-baseline")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for package in &alternatives {
                    let manifest = package.manifest();
                    ui.selectable_value(
                        &mut view.compare_against,
                        Some((manifest.package_id.clone(), manifest.revision.clone())),
                        format!("{} {}", manifest.package_id, manifest.revision),
                    );
                }
            });
        ui.ctx().accesskit_node_builder(combo.response.id, |node| {
            node.set_label("Baseline signed PDK revision");
        });

        let Some(baseline) = view
            .compare_against
            .as_ref()
            .and_then(|identity| package_by_identity(snapshot, identity))
            .filter(|package| package.binding() != candidate.binding())
        else {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                "The selected comparison baseline is no longer runtime trusted.",
            );
            return;
        };
        let diff_result: Result<PdkTechnologyRevisionDiff, PdkTechnologyDiffError> =
            PdkTechnologyRevisionDiff::between(baseline, candidate);
        let diff = match diff_result {
            Ok(diff) => diff,
            Err(error) => {
                ui.colored_label(
                    Tokens::get(ui.ctx()).color.err,
                    format!("Comparison failed closed: {error}"),
                );
                return;
            }
        };

        ui.separator();
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(format!(
                    "{} {} → {} {}",
                    diff.baseline.package_id,
                    diff.baseline.revision,
                    diff.candidate.package_id,
                    diff.candidate.revision
                ))
                .strong()
                .monospace(),
            );
            diff_metric(
                ui,
                "Breaking",
                diff.count(PdkTechnologyDiffImpact::Breaking),
                Tokens::get(ui.ctx()).color.err,
            );
            diff_metric(
                ui,
                "Review",
                diff.count(PdkTechnologyDiffImpact::ReviewRequired),
                Tokens::get(ui.ctx()).color.warn,
            );
            diff_metric(
                ui,
                "Information",
                diff.count(PdkTechnologyDiffImpact::Informational),
                Tokens::get(ui.ctx()).color.info,
            );
        });
        if !diff.same_package_lineage {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                "Package IDs differ. This is a cross-technology comparison, not a revision migration.",
            );
        } else if diff.has_breaking_changes() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.err,
                "Breaking contracts changed. Automatic migration is prohibited; every affected project object requires an explicit migration plan and validation.",
            );
        } else if diff.migration_requires_review() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "No structural breaking change was detected, but the signed revision still requires explicit review and project revalidation.",
            );
        } else {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.ok,
                "The exact validated packages are identical.",
            );
        }
        ui.small(
            RichText::new(format!(
                "Baseline manifest {} · archive {}",
                diff.baseline.manifest_digest, diff.baseline_archive_digest
            ))
            .monospace(),
        );
        ui.small(
            RichText::new(format!(
                "Candidate manifest {} · archive {}",
                diff.candidate.manifest_digest, diff.candidate_archive_digest
            ))
            .monospace(),
        );
        match serde_json::to_string_pretty(&diff) {
            Ok(json) => {
                if Button::new("Copy exact comparison JSON")
                    .accessible_label("Copy exact signed PDK revision comparison as JSON")
                    .show(ui)
                    .clicked()
                {
                    ui.ctx().copy_text(json);
                }
            }
            Err(error) => {
                ui.colored_label(
                    Tokens::get(ui.ctx()).color.err,
                    format!("Comparison export failed closed: {error}"),
                );
            }
        }

        if diff.entries.is_empty() {
            return;
        }
        ui.separator();
        let table = ScrollArea::both()
            .id_salt("pdk-revision-diff-table")
            .max_height(520.0)
            .show(ui, |ui| {
                ui.set_min_width(DIFF_TABLE_WIDTH);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    diff_table_header(ui);
                    for (index, entry) in diff.entries.iter().enumerate() {
                        diff_entry_row(ui, entry, index);
                    }
                })
                .response
            });
        ui.ctx().accesskit_node_builder(table.inner.id, |node| {
            node.set_role(egui::accesskit::Role::Table);
            node.set_label("Signed PDK revision differences");
        });
    });
}

fn diff_metric(ui: &mut Ui, label: &str, count: usize, color: Color32) {
    ui.colored_label(color, RichText::new(format!("{label} {count}")).strong());
}

const DIFF_TABLE_WIDTH: f32 = 1_480.0;
const DIFF_TABLE_ROW_HEIGHT: f32 = 30.0;
const DIFF_TABLE_COLUMN_WIDTHS: [f32; 6] = [100.0, 112.0, 92.0, 196.0, 490.0, 490.0];

fn diff_table_header(ui: &mut Ui) {
    let response = Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_panel_2)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                for (index, header) in ["Impact", "Area", "Change", "Identity", "Before", "After"]
                    .iter()
                    .enumerate()
                {
                    diff_table_cell(
                        ui,
                        header,
                        DIFF_TABLE_COLUMN_WIDTHS[index],
                        true,
                        Tokens::get(ui.ctx()).color.text,
                        false,
                    );
                }
            });
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label("PDK revision comparison headers");
    });
}

fn diff_entry_row(ui: &mut Ui, entry: &PdkTechnologyDiffEntry, index: usize) {
    let before = entry.before.as_deref().unwrap_or("—");
    let after = entry.after.as_deref().unwrap_or("—");
    let response = Frame::new()
        .fill(if index.is_multiple_of(2) {
            Tokens::get(ui.ctx()).color.bg_inset
        } else {
            Tokens::get(ui.ctx()).color.bg_panel
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let cells = [
                    (
                        diff_impact_label(entry.impact),
                        diff_impact_color(ui, entry.impact),
                        false,
                    ),
                    (
                        diff_area_label(entry.area),
                        Tokens::get(ui.ctx()).color.text,
                        false,
                    ),
                    (
                        diff_kind_label(entry.kind),
                        Tokens::get(ui.ctx()).color.text,
                        false,
                    ),
                    (
                        entry.identity.as_str(),
                        Tokens::get(ui.ctx()).color.text,
                        true,
                    ),
                    (before, Tokens::get(ui.ctx()).color.text_dim, true),
                    (after, Tokens::get(ui.ctx()).color.text_dim, true),
                ];
                for (index, (text, color, monospace)) in cells.into_iter().enumerate() {
                    diff_table_cell(
                        ui,
                        text,
                        DIFF_TABLE_COLUMN_WIDTHS[index],
                        false,
                        color,
                        monospace,
                    );
                }
            });
        })
        .response;
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
        node.set_label(format!(
            "{} {} {}",
            diff_impact_label(entry.impact),
            diff_area_label(entry.area),
            entry.identity
        ));
    });
}

fn diff_table_cell(
    ui: &mut Ui,
    text: &str,
    width: f32,
    header: bool,
    color: Color32,
    monospace: bool,
) {
    let mut rich = RichText::new(if header {
        text.to_ascii_uppercase()
    } else {
        text.to_owned()
    })
    .color(color);
    if header {
        rich = rich.strong();
    }
    if monospace {
        rich = rich.monospace();
    }
    let response = ui
        .add_sized(
            [width, DIFF_TABLE_ROW_HEIGHT],
            egui::Label::new(rich).truncate().selectable(!header),
        )
        .on_hover_text(text);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(if header {
            egui::accesskit::Role::ColumnHeader
        } else {
            egui::accesskit::Role::Cell
        });
        node.set_label(text);
    });
}

const fn diff_area_label(area: PdkTechnologyDiffArea) -> &'static str {
    match area {
        PdkTechnologyDiffArea::Identity => "Identity",
        PdkTechnologyDiffArea::Compatibility => "Compatibility",
        PdkTechnologyDiffArea::ModelSource => "Model source",
        PdkTechnologyDiffArea::Symbol => "Symbol",
        PdkTechnologyDiffArea::Layer => "Layer",
        PdkTechnologyDiffArea::StreamMap => "Stream map",
        PdkTechnologyDiffArea::Connectivity => "Connectivity",
        PdkTechnologyDiffArea::Recognition => "Recognition",
        PdkTechnologyDiffArea::Extraction => "Extraction",
        PdkTechnologyDiffArea::Callback => "Callback",
        PdkTechnologyDiffArea::Artifact => "Artifact",
    }
}

const fn diff_kind_label(kind: PdkTechnologyDiffKind) -> &'static str {
    match kind {
        PdkTechnologyDiffKind::Added => "Added",
        PdkTechnologyDiffKind::Removed => "Removed",
        PdkTechnologyDiffKind::Changed => "Changed",
    }
}

const fn diff_impact_label(impact: PdkTechnologyDiffImpact) -> &'static str {
    match impact {
        PdkTechnologyDiffImpact::Informational => "Information",
        PdkTechnologyDiffImpact::ReviewRequired => "Review",
        PdkTechnologyDiffImpact::Breaking => "Breaking",
    }
}

fn diff_impact_color(ui: &Ui, impact: PdkTechnologyDiffImpact) -> Color32 {
    let tokens = Tokens::get(ui.ctx());
    match impact {
        PdkTechnologyDiffImpact::Informational => tokens.color.info,
        PdkTechnologyDiffImpact::ReviewRequired => tokens.color.warn,
        PdkTechnologyDiffImpact::Breaking => tokens.color.err,
    }
}

fn display_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    snapshot: &RegistrySnapshot,
    view: &mut AdminViewState,
    action: &mut Option<AdminAction>,
) {
    reconcile_display_draft(package, snapshot, view);
    panel(
        ui,
        "Versioned display resources and visibility sets",
        |ui| {
            ui.label(
            "Presentation-only overrides are bound to this exact signed technology manifest. Layer identity, connectivity, stream mapping, recognition, extraction, and package bytes remain immutable.",
        );
            if let Some(error) = &snapshot.display_validation_error {
                ui.colored_label(
                    Tokens::get(ui.ctx()).color.err,
                    format!("Display-profile audit is corrupted and fails closed: {error}"),
                );
                return;
            }
            if !snapshot.runtime_display_profile_valid {
                ui.colored_label(
                    Tokens::get(ui.ctx()).color.warn,
                    "The stored active display profile is not consumable by the active trusted technology. Rendering falls back to signed package defaults.",
                );
            }

            let package_binding = package.binding();
            let package_profiles = snapshot
                .display_profiles
                .iter()
                .filter(|profile| profile.technology == package_binding)
                .collect::<Vec<_>>();
            let active = snapshot
                .active_display_profile
                .as_ref()
                .filter(|binding| binding.technology_manifest_digest == package.manifest_digest());
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("{} immutable revision(s)", package_profiles.len()));
                if let Some(active) = active {
                    ui.colored_label(
                        Tokens::get(ui.ctx()).color.ok,
                        format!("active: {} r{}", active.profile_id, active.revision),
                    );
                } else {
                    ui.colored_label(
                        Tokens::get(ui.ctx()).color.warn,
                        "No validated active profile for this technology",
                    );
                }
            });

            if !package_profiles.is_empty() {
                egui::ComboBox::from_label("Stored revision")
                    .selected_text(view.selected_display_profile.as_ref().map_or_else(
                        || "Select revision".to_owned(),
                        |(id, revision)| format!("{id} r{revision}"),
                    ))
                    .show_ui(ui, |ui| {
                        for profile in &package_profiles {
                            let identity = (profile.profile_id.clone(), profile.revision);
                            if ui
                                .selectable_value(
                                    &mut view.selected_display_profile,
                                    Some(identity.clone()),
                                    format!(
                                        "{} r{} \u{00b7} {}",
                                        profile.profile_id, profile.revision, profile.label
                                    ),
                                )
                                .changed()
                            {
                                view.display_draft = Some(profile.draft());
                                view.display_draft_dirty = false;
                            }
                        }
                    });
            }

            ui.horizontal_wrapped(|ui| {
                if Button::new("New from signed defaults").show(ui).clicked() {
                    let next = snapshot
                        .display_profiles
                        .iter()
                        .map(|profile| profile.profile_id.as_str())
                        .filter_map(|id| id.strip_prefix("layout-profile-"))
                        .filter_map(|suffix| suffix.parse::<u64>().ok())
                        .max()
                        .unwrap_or(0)
                        .saturating_add(1);
                    view.display_draft = Some(PdkDisplayProfileDraft::signed_defaults(
                        package,
                        format!("layout-profile-{next:02}"),
                        format!("Layout profile {next:02}"),
                    ));
                    view.selected_display_profile = None;
                    view.display_draft_dirty = true;
                }
                if Button::new("Reset draft to signed colors")
                    .enabled(view.display_draft.is_some())
                    .show(ui)
                    .clicked()
                {
                    let (id, label, scope) = view.display_draft.as_ref().map_or_else(
                        || {
                            (
                                "layout-profile-01".to_owned(),
                                "Layout profile 01".to_owned(),
                                PdkDisplayProfileScope::PersonalDevice,
                            )
                        },
                        |draft| (draft.profile_id.clone(), draft.label.clone(), draft.scope),
                    );
                    let mut reset = PdkDisplayProfileDraft::signed_defaults(package, id, label);
                    reset.scope = scope;
                    view.display_draft = Some(reset);
                    view.display_draft_dirty = true;
                }
            });

            let administrative_ready = administrative_fields_ready(view);
            let Some(draft) = view.display_draft.as_mut() else {
                ui.label("Create or select a display profile to edit.");
                return;
            };
            ui.separator();
            let before_header = (
                draft.profile_id.clone(),
                draft.label.clone(),
                draft.scope,
                draft.dim_unrelated,
                draft.hidden_objects_pickable,
                draft.selection_rgba,
            );
            ui.horizontal_wrapped(|ui| {
                ui.label("Profile ID");
                let profile_id = ui.add(
                    egui::TextEdit::singleline(&mut draft.profile_id)
                        .char_limit(128)
                        .desired_width(180.0),
                );
                ui.ctx()
                    .accesskit_node_builder(profile_id.id, |node| node.set_label("Profile ID"));
                ui.label("Label");
                let profile_label = ui.add(
                    egui::TextEdit::singleline(&mut draft.label)
                        .char_limit(128)
                        .desired_width(220.0),
                );
                ui.ctx().accesskit_node_builder(profile_label.id, |node| {
                    node.set_label("Profile label");
                });
            });
            ui.horizontal_wrapped(|ui| {
                egui::ComboBox::from_label("Scope")
                    .selected_text(display_scope_label(draft.scope))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut draft.scope,
                            PdkDisplayProfileScope::PersonalDevice,
                            display_scope_label(PdkDisplayProfileScope::PersonalDevice),
                        );
                        ui.add_enabled(false, egui::Label::new("Project"))
                            .on_disabled_hover_text(
                                "Project scope requires a project-owned profile repository.",
                            );
                        ui.add_enabled(false, egui::Label::new("Organization"))
                            .on_disabled_hover_text(
                            "Organization scope requires connected identity and policy services.",
                        );
                    });
                ui.checkbox(&mut draft.dim_unrelated, "Dim unrelated geometry");
                ui.checkbox(
                    &mut draft.hidden_objects_pickable,
                    "Hidden objects pickable in ghost mode",
                );
                let mut selection = Color32::from_rgba_unmultiplied(
                    draft.selection_rgba[0],
                    draft.selection_rgba[1],
                    draft.selection_rgba[2],
                    draft.selection_rgba[3],
                );
                ui.label("Selection");
                let selection_color = ui.color_edit_button_srgba(&mut selection);
                ui.ctx().accesskit_node_builder(selection_color.id, |node| {
                    node.set_label("Selection color");
                });
                if selection_color.changed() {
                    draft.selection_rgba = selection.to_array();
                }
            });
            if before_header
                != (
                    draft.profile_id.clone(),
                    draft.label.clone(),
                    draft.scope,
                    draft.dim_unrelated,
                    draft.hidden_objects_pickable,
                    draft.selection_rgba,
                )
            {
                view.display_draft_dirty = true;
            }

            ui.separator();
            ScrollArea::vertical()
                .id_salt("pdk-display-profile-editor")
                .max_height(520.0)
                .show(ui, |ui| {
                    for entry in &mut draft.entries {
                        let before = entry.clone();
                        let identity = format!("{}/{}", entry.layer, entry.purpose);
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.monospace(&identity);
                                let visible = ui.checkbox(&mut entry.visible, "Visible");
                                ui.ctx().accesskit_node_builder(visible.id, |node| {
                                    node.set_label(format!("Visible {identity}"));
                                });
                                let selectable = ui.checkbox(&mut entry.selectable, "Selectable");
                                ui.ctx().accesskit_node_builder(selectable.id, |node| {
                                    node.set_label(format!("Selectable {identity}"));
                                });
                                let mut opacity = entry.screen_rgba[3];
                                let opacity_response = ui.add(
                                    egui::Slider::new(&mut opacity, 0..=255)
                                        .text("Opacity")
                                        .custom_formatter(|value, _| {
                                            format!("{:.0}%", value * 100.0 / 255.0)
                                        }),
                                );
                                ui.ctx()
                                    .accesskit_node_builder(opacity_response.id, |node| {
                                        node.set_label(format!("Opacity for {identity}"))
                                    });
                                entry.screen_rgba[3] = opacity;
                            });
                            ui.horizontal_wrapped(|ui| {
                                let mut screen = Color32::from_rgba_unmultiplied(
                                    entry.screen_rgba[0],
                                    entry.screen_rgba[1],
                                    entry.screen_rgba[2],
                                    entry.screen_rgba[3],
                                );
                                ui.label("Screen");
                                let screen_color = ui.color_edit_button_srgba(&mut screen);
                                ui.ctx().accesskit_node_builder(screen_color.id, |node| {
                                    node.set_label(format!("Screen color for {identity}"));
                                });
                                if screen_color.changed() {
                                    entry.screen_rgba = screen.to_array();
                                }
                                display_fill_combo(
                                    ui,
                                    &format!("screen-fill-{}-{}", entry.layer, entry.purpose),
                                    &format!("Screen fill for {identity}"),
                                    &mut entry.screen_fill,
                                );
                                let mut print = Color32::from_rgba_unmultiplied(
                                    entry.print_rgba[0],
                                    entry.print_rgba[1],
                                    entry.print_rgba[2],
                                    entry.print_rgba[3],
                                );
                                ui.label("Print");
                                let print_color = ui.color_edit_button_srgba(&mut print);
                                ui.ctx().accesskit_node_builder(print_color.id, |node| {
                                    node.set_label(format!("Print color for {identity}"));
                                });
                                if print_color.changed() {
                                    entry.print_rgba = print.to_array();
                                }
                                display_fill_combo(
                                    ui,
                                    &format!("print-fill-{}-{}", entry.layer, entry.purpose),
                                    &format!("Print fill for {identity}"),
                                    &mut entry.print_fill,
                                );
                                let outline_response = ui.add(
                                    egui::Slider::new(
                                        &mut entry.outline_width_milli_px,
                                        0..=16_000,
                                    )
                                    .text("Outline")
                                    .custom_formatter(
                                        |value, _| format!("{:.2} px", value / 1_000.0),
                                    ),
                                );
                                ui.ctx()
                                    .accesskit_node_builder(outline_response.id, |node| {
                                        node.set_label(format!("Outline for {identity}"))
                                    });
                            });
                        });
                        if *entry != before {
                            view.display_draft_dirty = true;
                        }
                    }
                });

            ui.horizontal_wrapped(|ui| {
                let publish = Button::new("Save & activate immutable revision")
                    .accent()
                    .enabled(administrative_ready && view.display_draft_dirty)
                    .accessible_label(if !administrative_ready {
                    "Save and activate display profile. Unavailable: actor, authority, and reason are required"
                } else if !view.display_draft_dirty {
                    "Save and activate display profile. Unavailable: the draft has no changes"
                } else {
                    "Save and activate immutable display-profile revision"
                })
                .show(ui);
            if publish.clicked() {
                *action = Some(AdminAction::PublishDisplayProfile(draft.clone()));
            }

            if let Some((profile_id, revision)) = &view.selected_display_profile
                && let Some(profile) = package_profiles.iter().find(|profile| {
                    profile.profile_id.eq_ignore_ascii_case(profile_id)
                        && profile.revision == *revision
                }) {
                    let binding = PdkDisplayProfileBinding {
                        profile_id: profile.profile_id.clone(),
                        revision: profile.revision,
                        technology_manifest_digest: profile.technology.manifest_digest,
                        profile_digest: profile.content_digest,
                    };
                    let is_active = active == Some(&binding);
                    let appeared_before = snapshot
                        .display_audit
                        .iter()
                        .any(|receipt| receipt.after_active == binding);
                    let label = if appeared_before {
                        "Roll back to selected revision"
                    } else {
                        "Activate selected revision"
                    };
                    let activate = Button::new(label)
                        .enabled(administrative_ready && !is_active)
                        .accessible_label(if !administrative_ready {
                            "Activate selected display profile. Unavailable: actor, authority, and reason are required"
                        } else if is_active {
                            "Activate selected display profile. Unavailable: this exact revision is already active"
                        } else {
                            label
                        })
                        .show(ui);
                    if activate.clicked() {
                        *action = Some(AdminAction::ActivateDisplayProfile {
                            binding,
                            rollback: appeared_before,
                        });
                    }
                }
        });
            ui.small(
            "Actor and authority are operator-provided local audit fields. This surface does not claim connected organization authentication or remote authorization.",
        );
        },
    );
}

fn reconcile_display_draft(
    package: &ValidatedPdkTechnologyPackage,
    snapshot: &RegistrySnapshot,
    view: &mut AdminViewState,
) {
    if view
        .display_draft
        .as_ref()
        .is_some_and(|draft| draft.technology == package.binding())
    {
        return;
    }
    let selected = snapshot
        .active_display_profile
        .as_ref()
        .filter(|binding| binding.technology_manifest_digest == package.manifest_digest())
        .and_then(|binding| {
            snapshot.display_profiles.iter().find(|profile| {
                profile.profile_id.eq_ignore_ascii_case(&binding.profile_id)
                    && profile.revision == binding.revision
                    && profile.content_digest == binding.profile_digest
            })
        })
        .or_else(|| {
            snapshot
                .display_profiles
                .iter()
                .rev()
                .find(|profile| profile.technology == package.binding())
        });
    if let Some(profile) = selected {
        view.selected_display_profile = Some((profile.profile_id.clone(), profile.revision));
        view.display_draft = Some(profile.draft());
        view.display_draft_dirty = false;
    } else {
        view.selected_display_profile = None;
        view.display_draft = Some(PdkDisplayProfileDraft::signed_defaults(
            package,
            "layout-profile-01",
            "Layout profile 01",
        ));
        view.display_draft_dirty = true;
    }
}

fn display_fill_combo(ui: &mut Ui, id: &str, label: &str, value: &mut PdkDisplayFillStyle) {
    let response = egui::ComboBox::from_id_salt(id)
        .selected_text(display_fill_label(*value))
        .show_ui(ui, |ui| {
            for fill in [
                PdkDisplayFillStyle::Solid,
                PdkDisplayFillStyle::Diagonal,
                PdkDisplayFillStyle::Crosshatch,
                PdkDisplayFillStyle::Dots,
                PdkDisplayFillStyle::Hollow,
            ] {
                ui.selectable_value(value, fill, display_fill_label(fill));
            }
        })
        .response
        .on_hover_text(label);
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_label(label));
}

const fn display_fill_label(fill: PdkDisplayFillStyle) -> &'static str {
    match fill {
        PdkDisplayFillStyle::Solid => "Solid",
        PdkDisplayFillStyle::Diagonal => "Diagonal",
        PdkDisplayFillStyle::Crosshatch => "Crosshatch",
        PdkDisplayFillStyle::Dots => "Dots",
        PdkDisplayFillStyle::Hollow => "Hollow",
    }
}

const fn display_scope_label(scope: PdkDisplayProfileScope) -> &'static str {
    match scope {
        PdkDisplayProfileScope::PersonalDevice => "Personal device",
        PdkDisplayProfileScope::Project => "Project",
        PdkDisplayProfileScope::Organization => "Organization",
    }
}

fn package_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    snapshot: &RegistrySnapshot,
) {
    let manifest = package.manifest();
    panel(ui, "Technology package identity and compatibility", |ui| {
        property(ui, "Package", &manifest.package_id);
        property(ui, "Technology", &manifest.technology_name);
        property(ui, "Revision", &manifest.revision);
        property(ui, "Process", &format!("{} nm", manifest.process_node_nm));
        property(ui, "Stack", &manifest.stack_name);
        property(ui, "License", &manifest.license_spdx);
        property(
            ui,
            "Database unit",
            &format!("{:.12e} m", manifest.database_unit_meters),
        );
        property(
            ui,
            "Publisher",
            &format!("{}/{}", manifest.publisher_id, manifest.signing_key_id),
        );
        property(
            ui,
            "Manifest digest",
            &short_digest(package.manifest_digest()),
        );
        property(
            ui,
            "Archive digest",
            &short_digest(package.archive_digest()),
        );
        property(
            ui,
            "Compatibility",
            &format!(
                "engine ≥ {} · viewer ≥ {} · {}",
                manifest.compatibility.minimum_engine_version,
                manifest.compatibility.minimum_viewer_version,
                manifest
                    .compatibility
                    .targets
                    .iter()
                    .map(|target| execution_target_label(*target))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        );
        let active = snapshot.active.as_ref() == Some(&package.binding());
        ui.colored_label(
            if active {
                Tokens::get(ui.ctx()).color.ok
            } else {
                Tokens::get(ui.ctx()).color.text_faint
            },
            if active {
                "This exact signed revision is the active environment binding."
            } else {
                "This revision is installed and trusted but does not mutate any pinned project."
            },
        );
    });
}

fn layers_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage, view: &mut AdminViewState) {
    if draft_matches_package(view, package) {
        editable_layers_section(ui, package, view);
        return;
    }
    panel(
        ui,
        "Layer, purpose, display, and electrical-role dictionary",
        |ui| {
            ui.label(format!(
                "{} layer identities · {} purposes",
                package.manifest().layers.len(),
                package
                    .manifest()
                    .layers
                    .iter()
                    .map(|layer| layer.purposes.len())
                    .sum::<usize>()
            ));
            ScrollArea::vertical()
                .id_salt("pdk-layer-table")
                .max_height(520.0)
                .show(ui, |ui| {
                    Grid::new("pdk-layer-grid")
                        .striped(true)
                        .num_columns(5)
                        .show(ui, |ui| {
                            table_headers(ui, &["Layer", "Order", "Purposes", "Role", "Display"]);
                            for layer in &package.manifest().layers {
                                layer_row(ui, layer);
                            }
                        });
                });
        },
    );
}

fn editable_layers_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    view: &mut AdminViewState,
) {
    panel(ui, "Draft layer, purpose, and alias dictionary", |ui| {
        ui.small(
            "Edits apply only to the unsigned working draft. Layer renames cascade through stream, via, recognition, extraction, and alias references.",
        );
        if let Some(error) = &view.technology_draft_error {
            ui.colored_label(Tokens::get(ui.ctx()).color.err, error);
        }
        let mut changed = false;
        let mut remove_index = None;
        let mut purpose_edit_error = None;
        let draft = view
            .technology_draft
            .as_mut()
            .expect("matching draft checked by caller");
        ScrollArea::vertical()
            .id_salt("pdk-draft-layer-table")
            .max_height(420.0)
            .show(ui, |ui| {
                for index in 0..draft.manifest.layers.len() {
                    let old_name = draft.manifest.layers[index].name.clone();
                    Frame::NONE
                        .fill(Tokens::get(ui.ctx()).color.bg_inset)
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                changed |= ui
                                    .add(
                                        egui::TextEdit::singleline(
                                            &mut draft.manifest.layers[index].name,
                                        )
                                        .desired_width(120.0),
                                    )
                                    .changed();
                                changed |= ui
                                    .add(
                                        egui::DragValue::new(
                                            &mut draft.manifest.layers[index].order,
                                        )
                                        .range(0..=u16::MAX)
                                        .prefix("order "),
                                    )
                                    .changed();
                                let selected_kind = draft.manifest.layers[index].kind;
                                egui::ComboBox::from_id_salt(("draft-layer-kind", index))
                                    .selected_text(layer_kind_label(selected_kind))
                                    .show_ui(ui, |ui| {
                                        for kind in all_layer_kinds() {
                                            changed |= ui
                                                .selectable_value(
                                                    &mut draft.manifest.layers[index].kind,
                                                    kind,
                                                    layer_kind_label(kind),
                                                )
                                                .changed();
                                        }
                                    });
                                if Button::new("Remove").show(ui).clicked() {
                                    remove_index = Some(index);
                                }
                            });
                            ui.horizontal_wrapped(|ui| {
                                ui.label("Purposes");
                                let mut purposes = draft.manifest.layers[index].purposes.join(", ");
                                if ui
                                    .add(
                                        egui::TextEdit::singleline(&mut purposes)
                                            .desired_width(220.0)
                                            .hint_text("drawing, pin, label"),
                                    )
                                    .changed()
                                {
                                    let candidate = purposes
                                        .split(',')
                                        .map(str::trim)
                                        .filter(|value| !value.is_empty())
                                        .map(str::to_owned)
                                        .collect::<Vec<_>>();
                                    match apply_layer_purpose_edit(
                                        &mut draft.manifest,
                                        index,
                                        candidate,
                                    ) {
                                        Ok(()) => changed = true,
                                        Err(error) => purpose_edit_error = Some(error),
                                    }
                                }
                                ui.label("Role");
                                changed |= ui
                                    .add(
                                        egui::TextEdit::singleline(
                                            &mut draft.manifest.layers[index].role,
                                        )
                                        .desired_width(180.0),
                                    )
                                    .changed();
                                let mut color = Color32::from_rgba_unmultiplied(
                                    draft.manifest.layers[index].display_rgba[0],
                                    draft.manifest.layers[index].display_rgba[1],
                                    draft.manifest.layers[index].display_rgba[2],
                                    draft.manifest.layers[index].display_rgba[3],
                                );
                                if ui.color_edit_button_srgba(&mut color).changed() {
                                    draft.manifest.layers[index].display_rgba = color.to_array();
                                    changed = true;
                                }
                            });
                        });
                    if old_name != draft.manifest.layers[index].name {
                        cascade_layer_rename(&mut draft.manifest, &old_name, index);
                    }
                    ui.add_space(6.0);
                }
            });
        if let Some(error) = purpose_edit_error {
            view.technology_draft_error = Some(error);
        }
        if let Some(index) = remove_index {
            match remove_draft_layer(&mut draft.manifest, index) {
                Ok(()) => {
                    changed = true;
                    view.technology_draft_error = None;
                }
                Err(error) => view.technology_draft_error = Some(error),
            }
        }
        if Button::new("Add layer").show(ui).clicked() {
            add_draft_layer(&mut draft.manifest);
            changed = true;
        }
        ui.separator();
        ui.label(RichText::new("Layer aliases").strong());
        let layer_choices = layer_purpose_choices(&draft.manifest);
        let mut remove_alias = None;
        for (index, alias) in draft.manifest.layer_aliases.iter_mut().enumerate() {
            ui.horizontal_wrapped(|ui| {
                changed |= ui
                    .add(
                        egui::TextEdit::singleline(&mut alias.alias)
                            .desired_width(140.0)
                            .hint_text("portable alias"),
                    )
                    .changed();
                layer_purpose_combo(
                    ui,
                    ("draft-layer-alias-target", index),
                    &layer_choices,
                    &mut alias.layer,
                    &mut alias.purpose,
                    &mut changed,
                );
                if Button::new("Remove").show(ui).clicked() {
                    remove_alias = Some(index);
                }
            });
        }
        if let Some(index) = remove_alias {
            draft.manifest.layer_aliases.remove(index);
            changed = true;
        }
        if Button::new("Add alias")
            .enabled(!layer_choices.is_empty())
            .show(ui)
            .clicked()
        {
            let (layer, purpose) = layer_choices[0].clone();
            draft.manifest.layer_aliases.push(PdkLayerAlias {
                alias: next_alias_name(&draft.manifest),
                layer,
                purpose,
            });
            changed = true;
        }
        if changed {
            view.technology_draft_dirty = true;
            view.discard_technology_draft_armed = false;
            view.technology_draft_error = None;
        }
        let _ = package;
    });
}

fn layer_row(ui: &mut Ui, layer: &PdkTechnologyLayer) {
    let color = Color32::from_rgba_unmultiplied(
        layer.display_rgba[0],
        layer.display_rgba[1],
        layer.display_rgba[2],
        layer.display_rgba[3],
    );
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(12.0, 12.0), Sense::hover());
        ui.painter().rect_filled(rect, 2.0, color);
        ui.monospace(&layer.name);
    });
    ui.monospace(layer.order.to_string());
    ui.label(layer.purposes.join(" · "));
    ui.label(&layer.role);
    ui.monospace(format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        layer.display_rgba[0], layer.display_rgba[1], layer.display_rgba[2], layer.display_rgba[3]
    ));
    ui.end_row();
}

fn draft_matches_package(view: &AdminViewState, package: &ValidatedPdkTechnologyPackage) -> bool {
    view.technology_draft
        .as_ref()
        .is_some_and(|draft| draft.baseline.binding == package.binding())
}

const fn all_layer_kinds() -> [PdkLayerKind; 9] {
    [
        PdkLayerKind::Substrate,
        PdkLayerKind::Well,
        PdkLayerKind::Active,
        PdkLayerKind::Poly,
        PdkLayerKind::Metal,
        PdkLayerKind::Via,
        PdkLayerKind::Cut,
        PdkLayerKind::Marker,
        PdkLayerKind::Other,
    ]
}

const fn layer_kind_label(kind: PdkLayerKind) -> &'static str {
    match kind {
        PdkLayerKind::Substrate => "substrate",
        PdkLayerKind::Well => "well",
        PdkLayerKind::Active => "active",
        PdkLayerKind::Poly => "poly",
        PdkLayerKind::Metal => "metal",
        PdkLayerKind::Via => "via",
        PdkLayerKind::Cut => "cut",
        PdkLayerKind::Marker => "marker",
        PdkLayerKind::Other => "other",
    }
}

fn layer_purpose_choices(manifest: &PdkTechnologyManifest) -> Vec<(String, String)> {
    manifest
        .layers
        .iter()
        .flat_map(|layer| {
            layer
                .purposes
                .iter()
                .map(|purpose| (layer.name.clone(), purpose.clone()))
        })
        .collect()
}

fn layer_purpose_combo(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    choices: &[(String, String)],
    layer: &mut String,
    purpose: &mut String,
    changed: &mut bool,
) {
    egui::ComboBox::from_id_salt(id)
        .selected_text(format!("{layer}:{purpose}"))
        .show_ui(ui, |ui| {
            for (candidate_layer, candidate_purpose) in choices {
                let selected = layer.eq_ignore_ascii_case(candidate_layer)
                    && purpose.eq_ignore_ascii_case(candidate_purpose);
                if ui
                    .selectable_label(selected, format!("{candidate_layer}:{candidate_purpose}"))
                    .clicked()
                    && !selected
                {
                    layer.clone_from(candidate_layer);
                    purpose.clone_from(candidate_purpose);
                    *changed = true;
                }
            }
        });
}

fn cascade_layer_rename(manifest: &mut PdkTechnologyManifest, old_name: &str, index: usize) {
    let new_name = manifest.layers[index].name.clone();
    let replace = |value: &mut String| {
        if value.eq_ignore_ascii_case(old_name) {
            value.clone_from(&new_name);
        }
    };
    for alias in &mut manifest.layer_aliases {
        replace(&mut alias.layer);
    }
    for mapping in &mut manifest.stream_map {
        replace(&mut mapping.layer);
    }
    for edge in &mut manifest.connectivity {
        replace(&mut edge.from_layer);
        replace(&mut edge.through_layer);
        replace(&mut edge.to_layer);
    }
    for via in &mut manifest.vias {
        replace(&mut via.lower_layer);
        replace(&mut via.cut_layer);
        replace(&mut via.upper_layer);
    }
    for contract in &mut manifest.recognition {
        for terminal in &mut contract.terminals {
            replace(&mut terminal.layer);
        }
    }
    for contract in &mut manifest.extraction {
        for reference in &mut contract.layer_purposes {
            replace(&mut reference.layer);
        }
    }
}

fn sync_layer_stream_mappings(manifest: &mut PdkTechnologyManifest, index: usize) {
    let layer = manifest.layers[index].name.clone();
    let purposes = manifest.layers[index].purposes.clone();
    manifest.stream_map.retain(|mapping| {
        !mapping.layer.eq_ignore_ascii_case(&layer)
            || purposes
                .iter()
                .any(|purpose| purpose.eq_ignore_ascii_case(&mapping.purpose))
    });
    for purpose in purposes {
        if manifest.stream_map.iter().any(|mapping| {
            mapping.layer.eq_ignore_ascii_case(&layer)
                && mapping.purpose.eq_ignore_ascii_case(&purpose)
        }) {
            continue;
        }
        let (stream_layer, stream_datatype) = next_stream_identity(manifest);
        manifest.stream_map.push(PdkStreamMapEntry {
            layer: layer.clone(),
            purpose,
            stream_layer,
            stream_datatype,
        });
    }
}

fn apply_layer_purpose_edit(
    manifest: &mut PdkTechnologyManifest,
    index: usize,
    purposes: Vec<String>,
) -> Result<(), String> {
    if purposes.is_empty() {
        return Err("A technology layer must retain at least one purpose.".to_owned());
    }
    let mut identities = std::collections::BTreeSet::new();
    for purpose in &purposes {
        if !identities.insert(purpose.to_ascii_lowercase()) {
            return Err(format!(
                "Layer '{}' repeats purpose '{purpose}' ignoring case.",
                manifest.layers[index].name
            ));
        }
    }

    let layer = manifest.layers[index].name.clone();
    let previous = manifest.layers[index].purposes.clone();
    let mut removed = previous
        .iter()
        .filter(|old| !purposes.iter().any(|new| new.eq_ignore_ascii_case(old)))
        .cloned()
        .collect::<Vec<_>>();
    let added = purposes
        .iter()
        .filter(|new| !previous.iter().any(|old| old.eq_ignore_ascii_case(new)))
        .cloned()
        .collect::<Vec<_>>();

    if removed.len() == 1 && added.len() == 1 {
        cascade_layer_purpose_rename(manifest, &layer, &removed[0], &added[0]);
        removed.clear();
    }

    for purpose in &removed {
        let referenced_by_alias = manifest.layer_aliases.iter().any(|alias| {
            alias.layer.eq_ignore_ascii_case(&layer) && alias.purpose.eq_ignore_ascii_case(purpose)
        });
        let referenced_by_recognition = manifest.recognition.iter().any(|contract| {
            contract.terminals.iter().any(|terminal| {
                terminal.layer.eq_ignore_ascii_case(&layer)
                    && terminal.purpose.eq_ignore_ascii_case(purpose)
            })
        });
        let referenced_by_extraction = manifest.extraction.iter().any(|contract| {
            contract.layer_purposes.iter().any(|reference| {
                reference.layer.eq_ignore_ascii_case(&layer)
                    && reference.purpose.eq_ignore_ascii_case(purpose)
            })
        });
        if referenced_by_alias || referenced_by_recognition || referenced_by_extraction {
            return Err(format!(
                "Purpose '{layer}:{purpose}' is still referenced by an alias, recognition terminal, or extraction contract. Rename it one-for-one or remove those references first."
            ));
        }
    }

    for old in &previous {
        if let Some(new) = purposes
            .iter()
            .find(|new| new.eq_ignore_ascii_case(old) && *new != old)
        {
            cascade_layer_purpose_rename(manifest, &layer, old, new);
        }
    }

    manifest.layers[index].purposes = purposes;
    sync_layer_stream_mappings(manifest, index);
    Ok(())
}

fn cascade_layer_purpose_rename(
    manifest: &mut PdkTechnologyManifest,
    layer: &str,
    old_purpose: &str,
    new_purpose: &str,
) {
    let replace = |reference_layer: &str, purpose: &mut String| {
        if reference_layer.eq_ignore_ascii_case(layer) && purpose.eq_ignore_ascii_case(old_purpose)
        {
            purpose.clone_from(&new_purpose.to_owned());
        }
    };
    for mapping in &mut manifest.stream_map {
        replace(&mapping.layer, &mut mapping.purpose);
    }
    for alias in &mut manifest.layer_aliases {
        replace(&alias.layer, &mut alias.purpose);
    }
    for contract in &mut manifest.recognition {
        for terminal in &mut contract.terminals {
            replace(&terminal.layer, &mut terminal.purpose);
        }
    }
    for contract in &mut manifest.extraction {
        for reference in &mut contract.layer_purposes {
            replace(&reference.layer, &mut reference.purpose);
        }
    }
}

fn next_stream_identity(manifest: &PdkTechnologyManifest) -> (u16, u16) {
    for layer in 0..=u16::MAX {
        for datatype in 0..=u16::MAX {
            if !manifest
                .stream_map
                .iter()
                .any(|mapping| mapping.stream_layer == layer && mapping.stream_datatype == datatype)
            {
                return (layer, datatype);
            }
        }
    }
    (u16::MAX, u16::MAX)
}

fn add_draft_layer(manifest: &mut PdkTechnologyManifest) {
    let mut suffix = manifest.layers.len().saturating_add(1);
    let name = loop {
        let candidate = format!("layer_{suffix}");
        if !manifest
            .layers
            .iter()
            .any(|layer| layer.name.eq_ignore_ascii_case(&candidate))
        {
            break candidate;
        }
        suffix = suffix.saturating_add(1);
    };
    let order = manifest
        .layers
        .iter()
        .map(|layer| layer.order)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let (stream_layer, stream_datatype) = next_stream_identity(manifest);
    manifest.layers.push(PdkTechnologyLayer {
        name: name.clone(),
        order,
        kind: PdkLayerKind::Other,
        purposes: vec!["drawing".to_owned()],
        role: "unassigned".to_owned(),
        display_rgba: [128, 128, 128, 255],
    });
    manifest.stream_map.push(PdkStreamMapEntry {
        layer: name,
        purpose: "drawing".to_owned(),
        stream_layer,
        stream_datatype,
    });
}

fn remove_draft_layer(manifest: &mut PdkTechnologyManifest, index: usize) -> Result<(), String> {
    if manifest.layers.len() <= 1 {
        return Err("A technology manifest must retain at least one layer.".to_owned());
    }
    let name = manifest.layers[index].name.clone();
    let recognition_references = manifest.recognition.iter().any(|contract| {
        contract
            .terminals
            .iter()
            .any(|terminal| terminal.layer.eq_ignore_ascii_case(&name))
    });
    let extraction_references = manifest.extraction.iter().any(|contract| {
        contract
            .layer_purposes
            .iter()
            .any(|reference| reference.layer.eq_ignore_ascii_case(&name))
    });
    if recognition_references || extraction_references {
        return Err(format!(
            "Layer '{name}' is still referenced by recognition or extraction. Remove those references first."
        ));
    }
    manifest.layers.remove(index);
    manifest
        .layer_aliases
        .retain(|alias| !alias.layer.eq_ignore_ascii_case(&name));
    manifest
        .stream_map
        .retain(|mapping| !mapping.layer.eq_ignore_ascii_case(&name));
    manifest.connectivity.retain(|edge| {
        !edge.from_layer.eq_ignore_ascii_case(&name)
            && !edge.through_layer.eq_ignore_ascii_case(&name)
            && !edge.to_layer.eq_ignore_ascii_case(&name)
    });
    manifest.vias.retain(|via| {
        !via.lower_layer.eq_ignore_ascii_case(&name)
            && !via.cut_layer.eq_ignore_ascii_case(&name)
            && !via.upper_layer.eq_ignore_ascii_case(&name)
    });
    Ok(())
}

fn next_alias_name(manifest: &PdkTechnologyManifest) -> String {
    let mut suffix = manifest.layer_aliases.len().saturating_add(1);
    loop {
        let candidate = format!("alias_{suffix}");
        if !manifest
            .layer_aliases
            .iter()
            .any(|alias| alias.alias.eq_ignore_ascii_case(&candidate))
            && !manifest
                .layers
                .iter()
                .any(|layer| layer.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

fn stream_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage, view: &mut AdminViewState) {
    if draft_matches_package(view, package) {
        panel(ui, "Draft GDSII and OASIS stream mapping", |ui| {
            ui.small(
                "Every canonical layer-purpose remains explicitly mapped. Layer-purpose edits add and remove rows here deterministically.",
            );
            let draft = view
                .technology_draft
                .as_mut()
                .expect("matching draft checked by caller");
            let mut changed = false;
            Grid::new("pdk-draft-stream-grid")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    table_headers(ui, &["Layer", "Purpose", "Stream layer", "Datatype"]);
                    for mapping in &mut draft.manifest.stream_map {
                        ui.monospace(&mapping.layer);
                        ui.monospace(&mapping.purpose);
                        changed |= ui
                            .add(
                                egui::DragValue::new(&mut mapping.stream_layer).range(0..=u16::MAX),
                            )
                            .changed();
                        changed |= ui
                            .add(
                                egui::DragValue::new(&mut mapping.stream_datatype)
                                    .range(0..=u16::MAX),
                            )
                            .changed();
                        ui.end_row();
                    }
                });
            if changed {
                view.technology_draft_dirty = true;
                view.discard_technology_draft_armed = false;
            }
        });
        return;
    }
    panel(ui, "GDSII and OASIS stream mapping", |ui| {
        ui.label("Every declared layer purpose has exactly one validated stream identity.");
        ScrollArea::vertical()
            .id_salt("pdk-stream-table")
            .max_height(540.0)
            .show(ui, |ui| {
                Grid::new("pdk-stream-grid")
                    .striped(true)
                    .num_columns(4)
                    .show(ui, |ui| {
                        table_headers(ui, &["Layer", "Purpose", "Layer / datatype", "Status"]);
                        for mapping in &package.manifest().stream_map {
                            ui.monospace(&mapping.layer);
                            ui.monospace(&mapping.purpose);
                            ui.monospace(format!(
                                "{} / {}",
                                mapping.stream_layer, mapping.stream_datatype
                            ));
                            ui.colored_label(Tokens::get(ui.ctx()).color.ok, "mapped");
                            ui.end_row();
                        }
                    });
            });
    });
}

fn connectivity_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    view: &mut AdminViewState,
) {
    if draft_matches_package(view, package) {
        editable_vias_section(ui, view);
        return;
    }
    panel(ui, "Validated connectivity graph", |ui| {
        if package.manifest().connectivity.is_empty() {
            ui.label("This signed revision declares no conductive transitions.");
            return;
        }
        Grid::new("pdk-connectivity-grid")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                table_headers(ui, &["From", "Through", "To", "Status"]);
                for edge in &package.manifest().connectivity {
                    ui.monospace(&edge.from_layer);
                    ui.monospace(&edge.through_layer);
                    ui.monospace(&edge.to_layer);
                    ui.colored_label(Tokens::get(ui.ctx()).color.ok, "resolved");
                    ui.end_row();
                }
            });
    });
}

fn editable_vias_section(ui: &mut Ui, view: &mut AdminViewState) {
    panel(ui, "Draft via definitions and connectivity", |ui| {
        ui.small(
            "Generator dimensions are stored in metres. Every via definition must match one explicit lower → cut → upper connectivity edge.",
        );
        let draft = view
            .technology_draft
            .as_mut()
            .expect("matching draft checked by caller");
        let conductor_layers = draft
            .manifest
            .layers
            .iter()
            .filter(|layer| via_endpoint_layer_kind(layer.kind))
            .map(|layer| layer.name.clone())
            .collect::<Vec<_>>();
        let cut_layers = draft
            .manifest
            .layers
            .iter()
            .filter(|layer| via_cut_layer_kind(layer.kind))
            .map(|layer| layer.name.clone())
            .collect::<Vec<_>>();
        let mut changed = false;
        let mut remove_via = None;
        for (index, via) in draft.manifest.vias.iter_mut().enumerate() {
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_inset)
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        changed |= ui
                            .add(egui::TextEdit::singleline(&mut via.via_id).desired_width(150.0))
                            .changed();
                        layer_combo(
                            ui,
                            ("draft-via-lower", index),
                            &conductor_layers,
                            &mut via.lower_layer,
                            &mut changed,
                        );
                        ui.label("→");
                        layer_combo(
                            ui,
                            ("draft-via-cut", index),
                            &cut_layers,
                            &mut via.cut_layer,
                            &mut changed,
                        );
                        ui.label("→");
                        layer_combo(
                            ui,
                            ("draft-via-upper", index),
                            &conductor_layers,
                            &mut via.upper_layer,
                            &mut changed,
                        );
                    });
                    Grid::new(("draft-via-geometry", index))
                        .num_columns(4)
                        .show(ui, |ui| {
                            ui.label("Cut width (m)");
                            changed |= ui
                                .add(egui::DragValue::new(&mut via.cut_width_meters).speed(1.0e-9))
                                .changed();
                            ui.label("Cut height (m)");
                            changed |= ui
                                .add(egui::DragValue::new(&mut via.cut_height_meters).speed(1.0e-9))
                                .changed();
                            ui.end_row();
                            ui.label("Lower enclosure (m)");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut via.lower_enclosure_meters)
                                        .speed(1.0e-9),
                                )
                                .changed();
                            ui.label("Upper enclosure (m)");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut via.upper_enclosure_meters)
                                        .speed(1.0e-9),
                                )
                                .changed();
                            ui.end_row();
                            ui.label("Maximum rows");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut via.maximum_rows).range(1..=u16::MAX),
                                )
                                .changed();
                            ui.label("Maximum columns");
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut via.maximum_columns)
                                        .range(1..=u16::MAX),
                                )
                                .changed();
                            ui.end_row();
                        });
                    let mut has_current = via.maximum_rms_current_per_cut_amperes.is_some();
                    if ui
                        .checkbox(&mut has_current, "Specify RMS current per cut")
                        .changed()
                    {
                        via.maximum_rms_current_per_cut_amperes = has_current.then_some(1.0e-3);
                        changed = true;
                    }
                    if let Some(current) = via.maximum_rms_current_per_cut_amperes.as_mut() {
                        changed |= ui
                            .add(egui::DragValue::new(current).speed(1.0e-3).prefix("A/cut "))
                            .changed();
                    }
                    if Button::new("Remove generator definition")
                        .show(ui)
                        .clicked()
                    {
                        remove_via = Some(index);
                    }
                });
            ui.add_space(6.0);
        }
        if let Some(index) = remove_via {
            draft.manifest.vias.remove(index);
            changed = true;
        }

        let bare_edges = draft
            .manifest
            .connectivity
            .iter()
            .filter(|edge| {
                !draft.manifest.vias.iter().any(|via| {
                    via.lower_layer.eq_ignore_ascii_case(&edge.from_layer)
                        && via.cut_layer.eq_ignore_ascii_case(&edge.through_layer)
                        && via.upper_layer.eq_ignore_ascii_case(&edge.to_layer)
                })
            })
            .cloned()
            .collect::<Vec<_>>();
        if !bare_edges.is_empty() {
            ui.separator();
            ui.label(RichText::new("Connectivity-only transitions").strong());
            for edge in &bare_edges {
                ui.horizontal_wrapped(|ui| {
                    ui.monospace(format!(
                        "{} → {} → {}",
                        edge.from_layer, edge.through_layer, edge.to_layer
                    ));
                    if Button::new("Add generator definition").show(ui).clicked() {
                        draft
                            .manifest
                            .vias
                            .push(default_via_for_edge(&draft.manifest, edge));
                        changed = true;
                    }
                    if Button::new("Remove transition").show(ui).clicked()
                        && let Some(position) =
                            draft.manifest.connectivity.iter().position(|item| {
                                item.from_layer.eq_ignore_ascii_case(&edge.from_layer)
                                    && item.through_layer.eq_ignore_ascii_case(&edge.through_layer)
                                    && item.to_layer.eq_ignore_ascii_case(&edge.to_layer)
                            })
                    {
                        draft.manifest.connectivity.remove(position);
                        changed = true;
                    }
                });
            }
        }
        if Button::new("Add via transition")
            .enabled(default_via_layers(&draft.manifest).is_some())
            .show(ui)
            .on_disabled_hover_text(
                "Declare at least one cut/via layer and two conductor layers first.",
            )
            .clicked()
            && let Some((lower, cut, upper)) = default_via_layers(&draft.manifest)
        {
            let edge = crate::state::pdk_config::PdkConnectivityEdge {
                from_layer: lower,
                through_layer: cut,
                to_layer: upper,
            };
            let via = default_via_for_edge(&draft.manifest, &edge);
            draft.manifest.connectivity.push(edge);
            draft.manifest.vias.push(via);
            changed = true;
        }
        if changed {
            synchronize_via_connectivity(&mut draft.manifest);
            view.technology_draft_dirty = true;
            view.discard_technology_draft_armed = false;
        }
    });
}

fn layer_combo(
    ui: &mut Ui,
    id: (&'static str, usize),
    choices: &[String],
    value: &mut String,
    changed: &mut bool,
) {
    egui::ComboBox::from_id_salt(id)
        .selected_text(value.as_str())
        .show_ui(ui, |ui| {
            for choice in choices {
                if ui.selectable_value(value, choice.clone(), choice).changed() {
                    *changed = true;
                }
            }
        });
}

fn default_via_layers(manifest: &PdkTechnologyManifest) -> Option<(String, String, String)> {
    let cut = manifest
        .layers
        .iter()
        .find(|layer| via_cut_layer_kind(layer.kind))?;
    let conductors = manifest
        .layers
        .iter()
        .filter(|layer| via_endpoint_layer_kind(layer.kind))
        .take(2)
        .collect::<Vec<_>>();
    (conductors.len() == 2).then(|| {
        (
            conductors[0].name.clone(),
            cut.name.clone(),
            conductors[1].name.clone(),
        )
    })
}

const fn via_cut_layer_kind(kind: PdkLayerKind) -> bool {
    matches!(kind, PdkLayerKind::Cut | PdkLayerKind::Via)
}

const fn via_endpoint_layer_kind(kind: PdkLayerKind) -> bool {
    !matches!(
        kind,
        PdkLayerKind::Cut | PdkLayerKind::Via | PdkLayerKind::Marker
    )
}

fn next_via_id(manifest: &PdkTechnologyManifest) -> String {
    let mut suffix = manifest.vias.len().saturating_add(1);
    loop {
        let candidate = format!("via_{suffix}");
        if !manifest
            .vias
            .iter()
            .any(|via| via.via_id.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

fn default_via_for_edge(
    manifest: &PdkTechnologyManifest,
    edge: &crate::state::pdk_config::PdkConnectivityEdge,
) -> PdkViaDefinition {
    PdkViaDefinition {
        via_id: next_via_id(manifest),
        lower_layer: edge.from_layer.clone(),
        cut_layer: edge.through_layer.clone(),
        upper_layer: edge.to_layer.clone(),
        cut_width_meters: 1.0e-9,
        cut_height_meters: 1.0e-9,
        lower_enclosure_meters: 1.0e-9,
        upper_enclosure_meters: 1.0e-9,
        maximum_rows: 1,
        maximum_columns: 1,
        maximum_rms_current_per_cut_amperes: None,
    }
}

fn synchronize_via_connectivity(manifest: &mut PdkTechnologyManifest) {
    for via in &manifest.vias {
        if !manifest.connectivity.iter().any(|edge| {
            edge.from_layer.eq_ignore_ascii_case(&via.lower_layer)
                && edge.through_layer.eq_ignore_ascii_case(&via.cut_layer)
                && edge.to_layer.eq_ignore_ascii_case(&via.upper_layer)
        }) {
            manifest
                .connectivity
                .push(crate::state::pdk_config::PdkConnectivityEdge {
                    from_layer: via.lower_layer.clone(),
                    through_layer: via.cut_layer.clone(),
                    to_layer: via.upper_layer.clone(),
                });
        }
    }
}

fn recognition_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    view: &mut AdminViewState,
) {
    if draft_matches_package(view, package) {
        editable_recognition_section(ui, view);
        return;
    }
    panel(ui, "Signed device-recognition contracts", |ui| {
        ui.label(
            "Every displayed rule, terminal layer-purpose, and qualification vector was validated against this package’s exact signed artifact closure.",
        );
        if package.manifest().recognition.is_empty() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "This package declares no typed recognition contract. No device-recognition capability is claimed.",
            );
            return;
        }
        ScrollArea::vertical()
            .id_salt("pdk-recognition-contracts")
            .max_height(520.0)
            .show(ui, |ui| {
                for contract in &package.manifest().recognition {
                    Frame::NONE
                        .fill(Tokens::get(ui.ctx()).color.bg_inset)
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.monospace(&contract.contract_id);
                                ui.colored_label(Tokens::get(ui.ctx()).color.ok, "source-bound");
                            });
                            property(ui, "Device class", &contract.device_class);
                            property(ui, "Rule artifact", &contract.rule_artifact_path);
                            property(
                                ui,
                                "Terminals",
                                &contract
                                    .terminals
                                    .iter()
                                    .map(|terminal| {
                                        format!(
                                            "{}={} : {}",
                                            terminal.terminal_name,
                                            terminal.layer,
                                            terminal.purpose
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(" · "),
                            );
                            for vector in &contract.qualification_vectors {
                                property(
                                    ui,
                                    &format!("Vector {}", vector.vector_id),
                                    &format!(
                                        "{} · expected instances {}",
                                        vector.layout_artifact_path, vector.expected_instance_count
                                    ),
                                );
                            }
                        });
                    ui.add_space(6.0);
                }
            });
        ui.small(
            "Package validation proves contract completeness and artifact identity; it does not claim that a layout-recognition engine or foundry qualification is available.",
        );
    });
}

fn editable_recognition_section(ui: &mut Ui, view: &mut AdminViewState) {
    panel(ui, "Draft device-recognition contracts", |ui| {
        ui.small(
            "Contracts bind editable device and terminal mappings to immutable source-package rule and qualification artifacts.",
        );
        let draft = view
            .technology_draft
            .as_mut()
            .expect("matching draft checked by caller");
        let rule_artifacts =
            artifact_paths(&draft.manifest, PdkTechnologyArtifactKind::RecognitionMap);
        let vector_artifacts = artifact_paths(
            &draft.manifest,
            PdkTechnologyArtifactKind::QualificationVector,
        );
        let layer_choices = layer_purpose_choices(&draft.manifest);
        let mut reserved_vector_ids = draft
            .manifest
            .recognition
            .iter()
            .flat_map(|contract| {
                contract
                    .qualification_vectors
                    .iter()
                    .map(|vector| vector.vector_id.to_ascii_lowercase())
            })
            .chain(draft.manifest.extraction.iter().flat_map(|contract| {
                contract
                    .qualification_vectors
                    .iter()
                    .map(|vector| vector.vector_id.to_ascii_lowercase())
            }))
            .collect::<std::collections::BTreeSet<_>>();
        let mut changed = false;
        let mut remove_contract = None;
        let mut remove_terminal = None;
        let mut remove_vector = None;
        for (contract_index, contract) in draft.manifest.recognition.iter_mut().enumerate() {
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_inset)
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Contract");
                        changed |= ui
                            .add(
                                egui::TextEdit::singleline(&mut contract.contract_id)
                                    .desired_width(140.0),
                            )
                            .changed();
                        ui.label("Device class");
                        changed |= ui
                            .add(
                                egui::TextEdit::singleline(&mut contract.device_class)
                                    .desired_width(140.0),
                            )
                            .changed();
                        if Button::new("Remove contract").show(ui).clicked() {
                            remove_contract = Some(contract_index);
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Rule artifact");
                        artifact_combo(
                            ui,
                            ("draft-recognition-rule", contract_index, 0),
                            &rule_artifacts,
                            &mut contract.rule_artifact_path,
                            &mut changed,
                        );
                    });
                    ui.label(RichText::new("Terminals").strong());
                    for (terminal_index, terminal) in contract.terminals.iter_mut().enumerate() {
                        ui.horizontal_wrapped(|ui| {
                            changed |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut terminal.terminal_name)
                                        .desired_width(100.0),
                                )
                                .changed();
                            layer_purpose_combo(
                                ui,
                                ("draft-recognition-terminal", contract_index, terminal_index),
                                &layer_choices,
                                &mut terminal.layer,
                                &mut terminal.purpose,
                                &mut changed,
                            );
                            if Button::new("Remove").show(ui).clicked() {
                                remove_terminal = Some((contract_index, terminal_index));
                            }
                        });
                    }
                    if Button::new("Add terminal")
                        .enabled(!layer_choices.is_empty())
                        .show(ui)
                        .clicked()
                    {
                        let (layer, purpose) = layer_choices[0].clone();
                        let terminal_name = next_terminal_name(contract);
                        contract.terminals.push(PdkRecognitionTerminal {
                            terminal_name,
                            layer,
                            purpose,
                        });
                        changed = true;
                    }
                    ui.label(RichText::new("Qualification vectors").strong());
                    for (vector_index, vector) in
                        contract.qualification_vectors.iter_mut().enumerate()
                    {
                        ui.horizontal_wrapped(|ui| {
                            changed |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut vector.vector_id)
                                        .desired_width(120.0),
                                )
                                .changed();
                            artifact_combo(
                                ui,
                                ("draft-recognition-vector", contract_index, vector_index),
                                &vector_artifacts,
                                &mut vector.layout_artifact_path,
                                &mut changed,
                            );
                            changed |= ui
                                .add(
                                    egui::DragValue::new(&mut vector.expected_instance_count)
                                        .range(0..=1_000_000)
                                        .prefix("expected "),
                                )
                                .changed();
                            if Button::new("Remove").show(ui).clicked() {
                                remove_vector = Some((contract_index, vector_index));
                            }
                        });
                    }
                    if Button::new("Add qualification vector")
                        .enabled(!vector_artifacts.is_empty())
                        .show(ui)
                        .clicked()
                    {
                        contract
                            .qualification_vectors
                            .push(PdkRecognitionQualificationVector {
                                vector_id: next_unique_identifier(
                                    &mut reserved_vector_ids,
                                    "recognition_vector",
                                ),
                                layout_artifact_path: vector_artifacts[0].clone(),
                                expected_instance_count: 1,
                            });
                        changed = true;
                    }
                });
            ui.add_space(6.0);
        }
        if let Some((contract, terminal)) = remove_terminal {
            draft.manifest.recognition[contract]
                .terminals
                .remove(terminal);
            changed = true;
        }
        if let Some((contract, vector)) = remove_vector {
            draft.manifest.recognition[contract]
                .qualification_vectors
                .remove(vector);
            changed = true;
        }
        if let Some(index) = remove_contract {
            draft.manifest.recognition.remove(index);
            changed = true;
        }
        let can_add =
            !rule_artifacts.is_empty() && !vector_artifacts.is_empty() && !layer_choices.is_empty();
        if Button::new("Add recognition contract")
            .enabled(can_add)
            .show(ui)
            .on_disabled_hover_text(
                "The source package must contain recognition-map and qualification-vector artifacts and at least one layer-purpose.",
            )
            .clicked()
        {
            let (layer, purpose) = layer_choices[0].clone();
            let contract_id = next_contract_id(&draft.manifest, "recognition");
            draft.manifest.recognition.push(PdkRecognitionContract {
                contract_id,
                device_class: "device".to_owned(),
                rule_artifact_path: rule_artifacts[0].clone(),
                terminals: vec![PdkRecognitionTerminal {
                    terminal_name: "terminal_1".to_owned(),
                    layer,
                    purpose,
                }],
                qualification_vectors: vec![PdkRecognitionQualificationVector {
                    vector_id: next_unique_identifier(
                        &mut reserved_vector_ids,
                        "recognition_vector",
                    ),
                    layout_artifact_path: vector_artifacts[0].clone(),
                    expected_instance_count: 1,
                }],
            });
            changed = true;
        }
        if changed {
            view.technology_draft_dirty = true;
            view.discard_technology_draft_armed = false;
        }
    });
}

fn extraction_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    view: &mut AdminViewState,
) {
    if draft_matches_package(view, package) {
        editable_extraction_section(ui, view);
        return;
    }
    panel(ui, "Signed parasitic-extraction contracts", |ui| {
        ui.label(
            "Rule artifacts, covered layer-purposes, requested quantities, and independent reference vectors are exact members of the signed archive.",
        );
        if package.manifest().extraction.is_empty() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "This package declares no typed extraction contract. No parasitic-extraction capability is claimed.",
            );
            return;
        }
        ScrollArea::vertical()
            .id_salt("pdk-extraction-contracts")
            .max_height(520.0)
            .show(ui, |ui| {
                for contract in &package.manifest().extraction {
                    Frame::NONE
                        .fill(Tokens::get(ui.ctx()).color.bg_inset)
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.monospace(&contract.contract_id);
                                ui.colored_label(Tokens::get(ui.ctx()).color.ok, "source-bound");
                            });
                            property(ui, "Rule artifact", &contract.rule_artifact_path);
                            property(
                                ui,
                                "Quantities",
                                &contract
                                    .quantities
                                    .iter()
                                    .map(|quantity| extraction_quantity_label(*quantity))
                                    .collect::<Vec<_>>()
                                    .join(" · "),
                            );
                            property(
                                ui,
                                "Layer purposes",
                                &contract
                                    .layer_purposes
                                    .iter()
                                    .map(|reference| {
                                        format!("{}:{}", reference.layer, reference.purpose)
                                    })
                                    .collect::<Vec<_>>()
                                    .join(" · "),
                            );
                            for vector in &contract.qualification_vectors {
                                property(
                                    ui,
                                    &format!("Vector {}", vector.vector_id),
                                    &format!(
                                        "layout {} · reference {}",
                                        vector.layout_artifact_path, vector.reference_artifact_path
                                    ),
                                );
                            }
                        });
                    ui.add_space(6.0);
                }
            });
        ui.small(
            "Package validation does not execute or qualify extraction. Release and sign-off remain blocked until a registered producer supplies source-bound evidence.",
        );
    });
}

fn editable_extraction_section(ui: &mut Ui, view: &mut AdminViewState) {
    panel(ui, "Draft parasitic-extraction contracts", |ui| {
        ui.small(
            "Mappings select exact immutable rule, layout-vector, and reference artifacts from the source package.",
        );
        let draft = view
            .technology_draft
            .as_mut()
            .expect("matching draft checked by caller");
        let rule_artifacts =
            artifact_paths(&draft.manifest, PdkTechnologyArtifactKind::ExtractionRule);
        let vector_artifacts = artifact_paths(
            &draft.manifest,
            PdkTechnologyArtifactKind::QualificationVector,
        );
        let reference_artifacts = artifact_paths(
            &draft.manifest,
            PdkTechnologyArtifactKind::QualificationReference,
        );
        let layer_choices = layer_purpose_choices(&draft.manifest);
        let mut reserved_vector_ids = draft
            .manifest
            .recognition
            .iter()
            .flat_map(|contract| {
                contract
                    .qualification_vectors
                    .iter()
                    .map(|vector| vector.vector_id.to_ascii_lowercase())
            })
            .chain(draft.manifest.extraction.iter().flat_map(|contract| {
                contract
                    .qualification_vectors
                    .iter()
                    .map(|vector| vector.vector_id.to_ascii_lowercase())
            }))
            .collect::<std::collections::BTreeSet<_>>();
        let mut changed = false;
        let mut remove_contract = None;
        let mut remove_layer_purpose = None;
        let mut remove_vector = None;
        for (contract_index, contract) in draft.manifest.extraction.iter_mut().enumerate() {
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_inset)
                .inner_margin(egui::Margin::same(8))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Contract");
                        changed |= ui
                            .add(
                                egui::TextEdit::singleline(&mut contract.contract_id)
                                    .desired_width(150.0),
                            )
                            .changed();
                        ui.label("Rule artifact");
                        artifact_combo(
                            ui,
                            ("draft-extraction-rule", contract_index, 0),
                            &rule_artifacts,
                            &mut contract.rule_artifact_path,
                            &mut changed,
                        );
                        if Button::new("Remove contract").show(ui).clicked() {
                            remove_contract = Some(contract_index);
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Quantities");
                        for quantity in all_extraction_quantities() {
                            let mut selected = contract.quantities.contains(&quantity);
                            if ui
                                .checkbox(&mut selected, extraction_quantity_label(quantity))
                                .changed()
                            {
                                if selected {
                                    contract.quantities.push(quantity);
                                } else {
                                    contract.quantities.retain(|item| *item != quantity);
                                }
                                changed = true;
                            }
                        }
                    });
                    ui.label(RichText::new("Covered layer-purposes").strong());
                    for (reference_index, reference) in
                        contract.layer_purposes.iter_mut().enumerate()
                    {
                        ui.horizontal_wrapped(|ui| {
                            layer_purpose_combo(
                                ui,
                                (
                                    "draft-extraction-layer-purpose",
                                    contract_index,
                                    reference_index,
                                ),
                                &layer_choices,
                                &mut reference.layer,
                                &mut reference.purpose,
                                &mut changed,
                            );
                            if Button::new("Remove").show(ui).clicked() {
                                remove_layer_purpose = Some((contract_index, reference_index));
                            }
                        });
                    }
                    if Button::new("Add layer-purpose")
                        .enabled(!layer_choices.is_empty())
                        .show(ui)
                        .clicked()
                    {
                        let (layer, purpose) = layer_choices[0].clone();
                        contract
                            .layer_purposes
                            .push(PdkLayerPurposeRef { layer, purpose });
                        changed = true;
                    }
                    ui.label(RichText::new("Qualification vectors").strong());
                    for (vector_index, vector) in
                        contract.qualification_vectors.iter_mut().enumerate()
                    {
                        ui.horizontal_wrapped(|ui| {
                            changed |= ui
                                .add(
                                    egui::TextEdit::singleline(&mut vector.vector_id)
                                        .desired_width(120.0),
                                )
                                .changed();
                            artifact_combo(
                                ui,
                                ("draft-extraction-layout", contract_index, vector_index),
                                &vector_artifacts,
                                &mut vector.layout_artifact_path,
                                &mut changed,
                            );
                            artifact_combo(
                                ui,
                                ("draft-extraction-reference", contract_index, vector_index),
                                &reference_artifacts,
                                &mut vector.reference_artifact_path,
                                &mut changed,
                            );
                            if Button::new("Remove").show(ui).clicked() {
                                remove_vector = Some((contract_index, vector_index));
                            }
                        });
                    }
                    if Button::new("Add qualification vector")
                        .enabled(!vector_artifacts.is_empty() && !reference_artifacts.is_empty())
                        .show(ui)
                        .clicked()
                    {
                        contract.qualification_vectors.push(
                            crate::state::pdk_config::PdkExtractionQualificationVector {
                                vector_id: next_unique_identifier(
                                    &mut reserved_vector_ids,
                                    "extraction_vector",
                                ),
                                layout_artifact_path: vector_artifacts[0].clone(),
                                reference_artifact_path: reference_artifacts[0].clone(),
                            },
                        );
                        changed = true;
                    }
                });
            ui.add_space(6.0);
        }
        if let Some((contract, reference)) = remove_layer_purpose {
            draft.manifest.extraction[contract]
                .layer_purposes
                .remove(reference);
            changed = true;
        }
        if let Some((contract, vector)) = remove_vector {
            draft.manifest.extraction[contract]
                .qualification_vectors
                .remove(vector);
            changed = true;
        }
        if let Some(index) = remove_contract {
            draft.manifest.extraction.remove(index);
            changed = true;
        }
        let can_add = !rule_artifacts.is_empty()
            && !vector_artifacts.is_empty()
            && !reference_artifacts.is_empty()
            && !layer_choices.is_empty();
        if Button::new("Add extraction contract")
            .enabled(can_add)
            .show(ui)
            .on_disabled_hover_text(
                "The source package must contain extraction-rule, qualification-vector, and qualification-reference artifacts and at least one layer-purpose.",
            )
            .clicked()
        {
            let (layer, purpose) = layer_choices[0].clone();
            let contract_id = next_contract_id(&draft.manifest, "extraction");
            draft.manifest.extraction.push(PdkExtractionContract {
                contract_id,
                rule_artifact_path: rule_artifacts[0].clone(),
                quantities: vec![PdkExtractionQuantity::Resistance],
                layer_purposes: vec![PdkLayerPurposeRef { layer, purpose }],
                qualification_vectors: vec![
                    crate::state::pdk_config::PdkExtractionQualificationVector {
                        vector_id: next_unique_identifier(
                            &mut reserved_vector_ids,
                            "extraction_vector",
                        ),
                        layout_artifact_path: vector_artifacts[0].clone(),
                        reference_artifact_path: reference_artifacts[0].clone(),
                    },
                ],
            });
            changed = true;
        }
        if changed {
            view.technology_draft_dirty = true;
            view.discard_technology_draft_armed = false;
        }
    });
}

const fn all_extraction_quantities() -> [PdkExtractionQuantity; 5] {
    [
        PdkExtractionQuantity::Resistance,
        PdkExtractionQuantity::Capacitance,
        PdkExtractionQuantity::CouplingCapacitance,
        PdkExtractionQuantity::Inductance,
        PdkExtractionQuantity::DeviceParameter,
    ]
}

fn artifact_paths(
    manifest: &PdkTechnologyManifest,
    kind: PdkTechnologyArtifactKind,
) -> Vec<String> {
    manifest
        .artifacts
        .iter()
        .filter(|artifact| artifact.kind == kind)
        .map(|artifact| artifact.path.clone())
        .collect()
}

fn artifact_combo(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    choices: &[String],
    value: &mut String,
    changed: &mut bool,
) {
    egui::ComboBox::from_id_salt(id)
        .selected_text(value.as_str())
        .show_ui(ui, |ui| {
            for choice in choices {
                if ui.selectable_value(value, choice.clone(), choice).changed() {
                    *changed = true;
                }
            }
        });
}

fn next_terminal_name(contract: &PdkRecognitionContract) -> String {
    let mut suffix = contract.terminals.len().saturating_add(1);
    loop {
        let candidate = format!("terminal_{suffix}");
        if !contract
            .terminals
            .iter()
            .any(|terminal| terminal.terminal_name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

fn next_contract_id(manifest: &PdkTechnologyManifest, prefix: &str) -> String {
    let mut suffix = manifest
        .recognition
        .len()
        .saturating_add(manifest.extraction.len())
        .saturating_add(1);
    loop {
        let candidate = format!("{prefix}_{suffix}");
        if !manifest
            .recognition
            .iter()
            .any(|contract| contract.contract_id.eq_ignore_ascii_case(&candidate))
            && !manifest
                .extraction
                .iter()
                .any(|contract| contract.contract_id.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

fn next_unique_identifier(
    reserved: &mut std::collections::BTreeSet<String>,
    prefix: &str,
) -> String {
    let mut suffix = reserved.len().saturating_add(1);
    loop {
        let candidate = format!("{prefix}_{suffix}");
        if reserved.insert(candidate.to_ascii_lowercase()) {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

const fn extraction_quantity_label(quantity: PdkExtractionQuantity) -> &'static str {
    match quantity {
        PdkExtractionQuantity::Resistance => "resistance",
        PdkExtractionQuantity::Capacitance => "capacitance",
        PdkExtractionQuantity::CouplingCapacitance => "coupling capacitance",
        PdkExtractionQuantity::Inductance => "inductance",
        PdkExtractionQuantity::DeviceParameter => "device parameters",
    }
}

fn resources_section(
    ui: &mut Ui,
    package: &ValidatedPdkTechnologyPackage,
    snapshot: &RegistrySnapshot,
    view: &AdminViewState,
    action: &mut Option<AdminAction>,
) {
    panel(
        ui,
        "Content-addressed resources and callback permissions",
        |ui| {
            ui.label(format!(
                "{} / {} artifact digests verified",
                package.artifact_digests().len(),
                package.manifest().artifacts.len()
            ));
            Grid::new("pdk-artifact-grid")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    table_headers(ui, &["Artifact", "Kind", "Bytes", "SHA-256"]);
                    for artifact in &package.manifest().artifacts {
                        ui.monospace(&artifact.path);
                        ui.label(artifact_kind_label(artifact.kind));
                        ui.monospace(artifact.size_bytes.to_string());
                        ui.monospace(short_digest(artifact.sha256));
                        ui.end_row();
                    }
                });
            ui.separator();
            if package.manifest().model_sources.is_empty() {
                ui.label("No executable SPICE model-source contract is declared.");
            } else {
                ui.label(RichText::new("Executable model-source contract").strong());
                ui.small(
                    "Simulation resolves these exact signed artifacts and package-relative dependencies through an in-memory content-addressed bundle. No model search path is inferred.",
                );
                Grid::new("pdk-model-source-grid")
                    .striped(true)
                    .num_columns(5)
                    .show(ui, |ui| {
                        table_headers(
                            ui,
                            &["Process", "Domain", "Source ID", "Artifact", "Section"],
                        );
                        for contract in &package.manifest().model_sources {
                            for source in &contract.sources {
                                ui.monospace(contract.process.keyword());
                                ui.label(source.domain.label());
                                ui.monospace(&source.source_id);
                                ui.monospace(&source.artifact_path);
                                ui.monospace(
                                    source.section.as_deref().unwrap_or("complete source"),
                                );
                                ui.end_row();
                            }
                        }
                    });
                ui.separator();
            }
            if package.manifest().callbacks.is_empty() {
                ui.label("No executable callbacks are declared.");
            } else {
                for callback in &package.manifest().callbacks {
                    ui.label(
                        RichText::new(format!(
                            "{} · {} · ABI {} / {}() -> i32 / {}",
                            callback.callback_id,
                            callback.artifact_path,
                            callback.abi_version,
                            callback.entrypoint,
                            callback
                                .capabilities
                                .iter()
                                .map(|capability| format!("{capability:?}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ))
                        .monospace(),
                    );
                    let selected_is_project_pin = snapshot
                        .project_signed_package
                        .as_ref()
                        .is_some_and(|(binding, archive_digest)| {
                            *binding == package.binding()
                                && *archive_digest == package.archive_digest()
                        });
                    let ready = selected_is_project_pin
                        && snapshot.project_callback_blocker.is_none()
                        && administrative_fields_ready(view);
                    let accessible_label = if !selected_is_project_pin {
                        format!(
                            "Run callback {} for the attached project. Unavailable: the selected signed revision is not the project's exact pin",
                            callback.callback_id
                        )
                    } else if let Some(blocker) = &snapshot.project_callback_blocker {
                        format!(
                            "Run callback {} for the attached project. Unavailable: {blocker}",
                            callback.callback_id
                        )
                    } else if !administrative_fields_ready(view) {
                        format!(
                            "Run callback {} for the attached project. Unavailable: actor, authority, and reason are required",
                            callback.callback_id
                        )
                    } else {
                        format!(
                            "Run callback {} for the exact attached project revision",
                            callback.callback_id
                        )
                    };
                    if Button::new("Run for attached project")
                        .enabled(ready)
                        .accessible_label(&accessible_label)
                        .show(ui)
                        .clicked()
                    {
                        *action = Some(AdminAction::RunProjectCallback {
                            callback_id: callback.callback_id.clone(),
                        });
                    }
                }
                ui.small(
                    "Each signed module is ABI-validated at installation. Project execution uses only the exact project pin and canonical active-plan variables, is deterministic, fuel-metered and memory-bounded, exposes only declared RSpice host capabilities, and commits derived metadata in a project-owned receipt. WASI and network access are absent.",
                );
            }
            ui.separator();
            ui.label(RichText::new("Project callback evidence").strong());
            let receipts = snapshot
                .project_callback_receipts
                .iter()
                .filter(|receipt| {
                    receipt.execution.package_binding == package.binding()
                        && receipt.execution.archive_digest == package.archive_digest()
                })
                .collect::<Vec<_>>();
            if receipts.is_empty() {
                ui.small("No callback execution receipt for this exact signed revision is retained by the project.");
            } else {
                ScrollArea::vertical()
                    .id_salt("pdk-project-callback-receipts")
                    .max_height(420.0)
                    .show(ui, |ui| {
                        for receipt in receipts.into_iter().rev() {
                            Frame::NONE
                                .fill(Tokens::get(ui.ctx()).color.bg_inset)
                                .inner_margin(egui::Margin::same(8))
                                .show(ui, |ui| {
                                    ui.horizontal_wrapped(|ui| {
                                        ui.label(
                                            RichText::new(format!(
                                                "#{} {}",
                                                receipt.sequence, receipt.execution.callback_id
                                            ))
                                            .strong()
                                            .monospace(),
                                        );
                                        let verified = ui.colored_label(
                                            Tokens::get(ui.ctx()).color.ok,
                                            "receipt verified",
                                        );
                                        ui.ctx().accesskit_node_builder(verified.id, |node| {
                                            node.set_role(egui::accesskit::Role::Status);
                                            node.set_label("receipt verified");
                                        });
                                    });
                                    property(
                                        ui,
                                        "Project revision",
                                        &format!(
                                            "{} -> {}",
                                            receipt.from_project_revision.get(),
                                            receipt.to_project_revision.get()
                                        ),
                                    );
                                    property(
                                        ui,
                                        "Plan",
                                        &format!(
                                            "{} at revision {}",
                                            receipt.plan_id,
                                            receipt.plan_revision.get()
                                        ),
                                    );
                                    property(
                                        ui,
                                        "Operator authority",
                                        &format!("{} / {}", receipt.actor_id, receipt.authority_id),
                                    );
                                    property(ui, "Reason", &receipt.reason);
                                    property(
                                        ui,
                                        "Sandbox",
                                        &format!(
                                            "{} fuel / {} target / {}",
                                            receipt.execution.fuel_consumed,
                                            execution_target_label(
                                                receipt.execution.execution_target
                                            ),
                                            short_digest(receipt.execution.receipt_digest)
                                        ),
                                    );
                                    if receipt.execution.derived_metadata.is_empty() {
                                        property(ui, "Derived metadata", "none");
                                    } else {
                                        for (key, value) in &receipt.execution.derived_metadata {
                                            property(ui, &format!("Derived {key}"), value);
                                        }
                                    }
                                    let copy_label = format!(
                                        "Copy exact project callback receipt {} as JSON",
                                        receipt.sequence
                                    );
                                    if Button::new("Copy exact callback receipt as JSON")
                                        .accessible_label(&copy_label)
                                        .show(ui)
                                        .clicked()
                                    {
                                        match serde_json::to_string_pretty(receipt) {
                                            Ok(json) => ui.ctx().copy_text(json),
                                            Err(error) => ui.ctx().copy_text(format!(
                                                "Callback receipt serialization failed: {error}"
                                            )),
                                        }
                                    }
                                });
                            ui.add_space(6.0);
                        }
                    });
            }
        },
    );
}

fn trust_audit_section(
    ui: &mut Ui,
    package: Option<&ValidatedPdkTechnologyPackage>,
    snapshot: &RegistrySnapshot,
    view: &mut AdminViewState,
    action: &mut Option<AdminAction>,
) {
    panel(ui, "Publisher trust-root lifecycle", |ui| {
        ui.label(
            "Provision only organization-approved Ed25519 public verification keys. Private signing material is never accepted.",
        );
        ui.add(
            egui::TextEdit::singleline(&mut view.trust_publisher_id)
                .hint_text("Publisher ID")
                .desired_width(f32::INFINITY),
        );
        ui.add(
            egui::TextEdit::singleline(&mut view.trust_key_id)
                .hint_text("Signing key ID")
                .desired_width(f32::INFINITY),
        );
        ui.add(
            egui::TextEdit::multiline(&mut view.trust_key_base64)
                .hint_text("Ed25519 public key · base64 · exactly 32 decoded bytes")
                .char_limit(MAX_TRUST_KEY_BASE64_INPUT)
                .desired_rows(2)
                .desired_width(f32::INFINITY),
        );
        let key_fields_ready = !view.trust_publisher_id.trim().is_empty()
            && !view.trust_key_id.trim().is_empty()
            && !view.trust_key_base64.trim().is_empty();
        let provision = Button::new("Provision publisher key")
            .accent()
            .enabled(administrative_fields_ready(view) && key_fields_ready)
            .accessible_label(if !administrative_fields_ready(view) {
                "Provision publisher key. Unavailable: actor, authority, and reason are required"
            } else if !key_fields_ready {
                "Provision publisher key. Unavailable: publisher, key ID, and base64 public key are required"
            } else {
                "Provision publisher key"
            })
            .show(ui);
        if provision.clicked() {
            let encoded = view.trust_key_base64.trim();
            *action = Some(if encoded.len() > MAX_TRUST_KEY_BASE64_INPUT {
                AdminAction::ReportError(format!(
                    "Publisher verification key input exceeds {MAX_TRUST_KEY_BASE64_INPUT} bytes"
                ))
            } else {
                match STANDARD.decode(encoded) {
                    Ok(bytes) => match <[u8; 32]>::try_from(bytes) {
                        Ok(verifying_key) => {
                            AdminAction::ProvisionTrustKey(TrustedPdkPublisherKey {
                                publisher_id: view.trust_publisher_id.trim().to_owned(),
                                key_id: view.trust_key_id.trim().to_owned(),
                                verifying_key,
                                revoked: false,
                            })
                        }
                        Err(bytes) => AdminAction::ReportError(format!(
                            "Publisher verification key decodes to {} bytes; exactly 32 are required",
                            bytes.len()
                        )),
                    },
                    Err(error) => AdminAction::ReportError(format!(
                        "Publisher verification key is not valid base64: {error}"
                    )),
                }
            });
        }

        ui.separator();
        ui.label("Provisioned Ed25519 verification keys");
        for key in &snapshot.trust_keys {
            let encoded = STANDARD.encode(key.verifying_key);
            ui.horizontal_wrapped(|ui| {
                ui.monospace(format!("{}/{}", key.publisher_id, key.key_id));
                ui.colored_label(
                    if key.revoked {
                        Tokens::get(ui.ctx()).color.err
                    } else {
                        Tokens::get(ui.ctx()).color.ok
                    },
                    if key.revoked { "revoked" } else { "trusted" },
                );
                ui.monospace(format!("{}…", &encoded[..encoded.len().min(12)]));
                let revoke = Button::new("Revoke")
                    .enabled(!key.revoked && administrative_fields_ready(view))
                    .accessible_label(if key.revoked {
                        "Revoke publisher key. Unavailable: this key is already irrevocably revoked"
                    } else if !administrative_fields_ready(view) {
                        "Revoke publisher key. Unavailable: actor, authority, and reason are required"
                    } else {
                        "Revoke publisher key"
                    })
                    .show(ui);
                if revoke.clicked() {
                    *action = Some(AdminAction::RevokeTrustKey {
                        publisher_id: key.publisher_id.clone(),
                        key_id: key.key_id.clone(),
                    });
                }
            });
        }
        if snapshot.trust_keys.is_empty() {
            ui.colored_label(
                Tokens::get(ui.ctx()).color.warn,
                "No publisher verification key is provisioned.",
            );
        }
        if let Some(package) = package {
            ui.separator();
            ui.label(format!(
                "Selected package signer: {}/{}",
                package.manifest().publisher_id,
                package.manifest().signing_key_id
            ));
        }
    });

    ui.add_space(10.0);
    panel(
        ui,
        "Immutable trust and package transaction history",
        |ui| {
            ui.label("Trust-root receipts");
            for receipt in snapshot.trust_audit.iter().rev() {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("#{}", receipt.sequence)).strong());
                        ui.label(trust_audit_action_label(receipt.action));
                        ui.monospace(format!("{}/{}", receipt.publisher_id, receipt.key_id));
                    });
                    ui.small(format!(
                        "{} · {} · {}",
                        receipt.actor_id, receipt.authority_id, receipt.reason
                    ));
                    ui.monospace(short_digest(receipt.receipt_digest));
                });
            }
            if snapshot.trust_audit.is_empty() {
                ui.label("No governed trust-root transaction has been recorded.");
            }
            ui.separator();
            ui.label("Technology package receipts");
            ScrollArea::vertical()
                .id_salt("pdk-audit-table")
                .max_height(360.0)
                .show(ui, |ui| {
                    for receipt in snapshot.audit.iter().rev() {
                        ui.group(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new(format!("#{}", receipt.sequence)).strong());
                                ui.label(audit_action_label(receipt.action));
                                ui.monospace(format!(
                                    "{} {}",
                                    receipt.target.package_id, receipt.target.revision
                                ));
                            });
                            ui.small(format!(
                                "{} · {} · {}",
                                receipt.actor_id, receipt.authority_id, receipt.reason
                            ));
                            ui.monospace(short_digest(receipt.receipt_digest));
                        });
                    }
                });
            if snapshot.audit.is_empty() {
                ui.label("No package transaction has been recorded.");
            }
            ui.separator();
            ui.label("Display-profile receipts");
            for receipt in snapshot.display_audit.iter().rev() {
                ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new(format!("#{}", receipt.sequence)).strong());
                        ui.label(display_audit_action_label(receipt.action));
                        ui.monospace(format!(
                            "{} r{}",
                            receipt.target.profile_id, receipt.target.revision
                        ));
                    });
                    ui.small(format!(
                        "{} \u{00b7} {} \u{00b7} {}",
                        receipt.actor_id, receipt.authority_id, receipt.reason
                    ));
                    ui.monospace(short_digest(receipt.receipt_digest));
                });
            }
            if snapshot.display_audit.is_empty() {
                ui.label("No display-profile transaction has been recorded.");
            }
            if Button::new("Copy complete audit JSON").show(ui).clicked() {
                let value = serde_json::json!({
                    "trust_root_receipts": snapshot.trust_audit,
                    "technology_package_receipts": snapshot.audit,
                    "display_profile_receipts": snapshot.display_audit,
                });
                if let Ok(json) = serde_json::to_string_pretty(&value) {
                    ui.ctx().copy_text(json);
                }
            }
        },
    );
}

fn panel(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) {
    let tokens = Tokens::get(ui.ctx());
    Frame::new()
        .fill(tokens.color.bg_panel)
        .stroke(Stroke::new(1.0, tokens.color.border))
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.label(RichText::new(title).strong());
            ui.separator();
            body(ui);
        });
}

fn property(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(format!("{label}:"));
        ui.monospace(value);
    });
}

fn table_headers(ui: &mut Ui, headers: &[&str]) {
    for header in headers {
        ui.label(RichText::new(*header).strong());
    }
    ui.end_row();
}

fn short_digest(digest: crate::product::ContentDigest) -> String {
    let value = digest.to_string();
    format!("{}…{}", &value[..12], &value[value.len() - 8..])
}

const fn execution_target_label(target: PdkExecutionTarget) -> &'static str {
    match target {
        PdkExecutionTarget::Desktop => "desktop",
        PdkExecutionTarget::WebAssembly => "browser",
        PdkExecutionTarget::Mobile => "mobile",
    }
}

const fn artifact_kind_label(kind: PdkTechnologyArtifactKind) -> &'static str {
    match kind {
        PdkTechnologyArtifactKind::Model => "model",
        PdkTechnologyArtifactKind::VerilogASource => "Verilog-A source",
        PdkTechnologyArtifactKind::RuleDeck => "rule deck",
        PdkTechnologyArtifactKind::DisplayResource => "display",
        PdkTechnologyArtifactKind::StreamMap => "stream map",
        PdkTechnologyArtifactKind::RecognitionMap => "recognition",
        PdkTechnologyArtifactKind::ExtractionRule => "extraction",
        PdkTechnologyArtifactKind::QualificationVector => "qualification vector",
        PdkTechnologyArtifactKind::QualificationReference => "qualification reference",
        PdkTechnologyArtifactKind::Callback => "callback",
        PdkTechnologyArtifactKind::Documentation => "documentation",
    }
}

const fn audit_action_label(action: PdkTechnologyAuditAction) -> &'static str {
    match action {
        PdkTechnologyAuditAction::Install => "installed",
        PdkTechnologyAuditAction::Activate => "activated",
        PdkTechnologyAuditAction::Rollback => "rolled back",
    }
}

const fn trust_audit_action_label(action: PdkTrustAuditAction) -> &'static str {
    match action {
        PdkTrustAuditAction::Provision => "provisioned",
        PdkTrustAuditAction::Revoke => "revoked",
    }
}

const fn display_audit_action_label(
    action: crate::state::pdk_config::PdkDisplayProfileAuditAction,
) -> &'static str {
    match action {
        crate::state::pdk_config::PdkDisplayProfileAuditAction::PublishAndActivate => {
            "published and activated"
        }
        crate::state::pdk_config::PdkDisplayProfileAuditAction::Activate => "activated",
        crate::state::pdk_config::PdkDisplayProfileAuditAction::Rollback => "rolled back",
    }
}

fn administrative_fields_ready(view: &AdminViewState) -> bool {
    !view.actor_id.trim().is_empty()
        && !view.authority_id.trim().is_empty()
        && !view.reason.trim().is_empty()
}

fn authority(view: &AdminViewState) -> PdkAdministrativeAuthority {
    PdkAdministrativeAuthority {
        actor_id: view.actor_id.trim().to_owned(),
        authority_id: view.authority_id.trim().to_owned(),
    }
}

fn apply_action(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    view: &mut AdminViewState,
    action: AdminAction,
) {
    if matches!(action, AdminAction::ChooseImport) {
        request_package_import(ctx, app, view);
        return;
    }
    let result = match action {
        AdminAction::ChooseImport => unreachable!("handled before transaction dispatch"),
        AdminAction::Revalidate => {
            let mut candidate = app.state.pdk_config.clone();
            let trust_store = candidate.publisher_trust_store.clone();
            match candidate
                .technology_registry
                .revalidate_installed(&trust_store)
            {
                Ok(count) => persist_candidate(
                    ctx,
                    app,
                    view,
                    candidate,
                    format!("Revalidated {count} installed signed package revision(s)."),
                    AdminCommitEffect::None,
                ),
                Err(errors) => {
                    let message = errors.join("; ");
                    // Persist no failed runtime cache: the current in-memory
                    // registry is still replaced so consumers fail closed.
                    app.state.pdk_config.technology_registry = candidate.technology_registry;
                    Err(message)
                }
            }
        }
        AdminAction::SaveTechnologyDraft(draft) => {
            let mut candidate = app.state.pdk_config.clone();
            candidate.technology_draft = Some(draft.clone());
            persist_candidate(
                ctx,
                app,
                view,
                candidate,
                format!(
                    "Saved unsigned technology draft '{}' for baseline {} {}.",
                    draft.draft_id,
                    draft.baseline.binding.package_id,
                    draft.baseline.binding.revision
                ),
                AdminCommitEffect::TechnologyDraftSaved,
            )
        }
        AdminAction::DiscardTechnologyDraft => {
            let mut candidate = app.state.pdk_config.clone();
            candidate.technology_draft = None;
            persist_candidate(
                ctx,
                app,
                view,
                candidate,
                "Discarded the unsigned technology draft. Installed signed packages were unchanged."
                    .to_owned(),
                AdminCommitEffect::TechnologyDraftDiscarded,
            )
        }
        AdminAction::ExportTechnologyDraft(draft) => {
            let registry = &app.state.pdk_config.technology_registry;
            let package = registry
                .validated_packages()
                .iter()
                .find(|package| package.binding() == draft.baseline.binding)
                .ok_or_else(|| {
                    "The draft baseline no longer resolves to a currently trusted package."
                        .to_owned()
                });
            package.and_then(|package| {
                let archive = registry.archive_for_package(package).ok_or_else(|| {
                    "The exact source archive for this draft is unavailable.".to_owned()
                })?;
                let bundle = draft
                    .authoring_bundle(package, archive)
                    .map_err(|error| error.to_string())?;
                let bytes = serde_json::to_vec_pretty(&bundle)
                    .map_err(|error| format!("Could not serialize authoring bundle: {error}"))?;
                crate::workbench::workflows::export_workflow::publish_generated_bytes(
                    "unsigned PDK technology authoring bundle",
                    crate::workbench::workflows::export_workflow::SaveDialogConfig {
                        title: "Export unsigned PDK authoring bundle",
                        default_name: "rspice-pdk-authoring.json",
                        filter_name: "RSpice PDK authoring bundle",
                        filter_extensions: &["json"],
                    },
                    &bytes,
                    "application/json",
                )
            })
        }
        AdminAction::Activate { binding, rollback } => {
            let mut candidate = app.state.pdk_config.clone();
            let selected = candidate
                .technology_registry
                .validated_packages()
                .iter()
                .find(|package| package.binding() == binding)
                .cloned()
                .ok_or_else(|| {
                    "The selected revision no longer has current runtime trust.".to_owned()
                });
            selected.and_then(|selected| {
                let receipt = if rollback {
                    candidate.technology_registry.rollback_to(
                        &binding.package_id,
                        &binding.revision,
                        &authority(view),
                        view.reason.trim(),
                    )
                } else {
                    candidate.technology_registry.activate(
                        &binding.package_id,
                        &binding.revision,
                        &authority(view),
                        view.reason.trim(),
                    )
                }
                .map_err(|error| error.to_string())?;
                candidate.layout_database_unit = crate::quantity::LayoutDatabaseUnit::from_metres(
                    selected.manifest().database_unit_meters,
                )
                .map(Some)
                .map_err(|error| error.to_string())?;
                persist_candidate(
                    ctx,
                    app,
                    view,
                    candidate,
                    format!(
                        "{} {} {} as audit receipt #{}.",
                        if rollback {
                            "Rolled back to"
                        } else {
                            "Activated"
                        },
                        binding.package_id,
                        binding.revision,
                        receipt.sequence
                    ),
                    AdminCommitEffect::None,
                )
            })
        }
        AdminAction::ProvisionTrustKey(key) => {
            let mut candidate = app.state.pdk_config.clone();
            let receipt = candidate
                .publisher_trust_store
                .provision_key(key, &authority(view), view.reason.trim())
                .map_err(|error| error.to_string());
            receipt.and_then(|receipt| {
                let validation_errors = revalidate_after_trust_change(&mut candidate);
                let message = if validation_errors.is_empty() {
                    format!(
                        "Provisioned publisher key {}/{} as trust receipt #{} and revalidated installed packages.",
                        receipt.publisher_id, receipt.key_id, receipt.sequence
                    )
                } else {
                    format!(
                        "Provisioned publisher key {}/{} as trust receipt #{}. {} installed package validation error(s) remain quarantined.",
                        receipt.publisher_id,
                        receipt.key_id,
                        receipt.sequence,
                        validation_errors.len()
                    )
                };
                #[cfg(not(target_arch = "wasm32"))]
                {
                    view.trust_publisher_id.clear();
                    view.trust_key_id.clear();
                    view.trust_key_base64.clear();
                }
                persist_candidate(
                    ctx,
                    app,
                    view,
                    candidate,
                    message,
                    AdminCommitEffect::ClearTrustDraft {
                        publisher_id: receipt.publisher_id,
                        key_id: receipt.key_id,
                    },
                )
            })
        }
        AdminAction::RevokeTrustKey {
            publisher_id,
            key_id,
        } => {
            let mut candidate = app.state.pdk_config.clone();
            let receipt = candidate
                .publisher_trust_store
                .revoke_key(&publisher_id, &key_id, &authority(view), view.reason.trim())
                .map_err(|error| error.to_string());
            receipt.and_then(|receipt| {
                let validation_errors = revalidate_after_trust_change(&mut candidate);
                persist_candidate(
                    ctx,
                    app,
                    view,
                    candidate,
                    format!(
                        "Irrevocably revoked publisher key {}/{} as trust receipt #{}. {} installed package validation error(s) now fail closed.",
                        receipt.publisher_id,
                        receipt.key_id,
                        receipt.sequence,
                        validation_errors.len()
                    ),
                    AdminCommitEffect::None,
                )
            })
        }
        AdminAction::PublishDisplayProfile(draft) => {
            let mut candidate = app.state.pdk_config.clone();
            let package = candidate
                .technology_registry
                .validated_packages()
                .iter()
                .find(|package| package.binding() == draft.technology)
                .cloned()
                .ok_or_else(|| {
                    "The display draft no longer resolves to an exact currently trusted technology package."
                        .to_owned()
                });
            package.and_then(|package| {
                let receipt = candidate
                    .display_profile_registry
                    .publish_and_activate(&package, draft, &authority(view), view.reason.trim())
                    .map_err(|error| error.to_string())?;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    view.selected_display_profile =
                        Some((receipt.target.profile_id.clone(), receipt.target.revision));
                    view.display_draft_dirty = false;
                }
                persist_candidate(
                    ctx,
                    app,
                    view,
                    candidate,
                    format!(
                        "Published and activated display profile {} revision {} as audit receipt #{}.",
                        receipt.target.profile_id, receipt.target.revision, receipt.sequence
                    ),
                    AdminCommitEffect::SelectDisplayProfile {
                        profile_id: receipt.target.profile_id,
                        revision: receipt.target.revision,
                    },
                )
            })
        }
        AdminAction::ActivateDisplayProfile { binding, rollback } => {
            let mut candidate = app.state.pdk_config.clone();
            let package = candidate
                .technology_registry
                .validated_packages()
                .iter()
                .find(|package| package.manifest_digest() == binding.technology_manifest_digest)
                .cloned()
                .ok_or_else(|| {
                    "The selected display revision no longer resolves to an exact currently trusted technology package."
                        .to_owned()
                });
            package.and_then(|package| {
                let receipt = if rollback {
                    candidate.display_profile_registry.rollback_to(
                        &package,
                        &binding.profile_id,
                        binding.revision,
                        &authority(view),
                        view.reason.trim(),
                    )
                } else {
                    candidate.display_profile_registry.activate(
                        &package,
                        &binding.profile_id,
                        binding.revision,
                        &authority(view),
                        view.reason.trim(),
                    )
                }
                .map_err(|error| error.to_string())?;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    view.selected_display_profile =
                        Some((receipt.target.profile_id.clone(), receipt.target.revision));
                    view.display_draft_dirty = false;
                }
                persist_candidate(
                    ctx,
                    app,
                    view,
                    candidate,
                    format!(
                        "{} display profile {} revision {} as audit receipt #{}.",
                        if rollback {
                            "Rolled back to"
                        } else {
                            "Activated"
                        },
                        receipt.target.profile_id,
                        receipt.target.revision,
                        receipt.sequence
                    ),
                    AdminCommitEffect::SelectDisplayProfile {
                        profile_id: receipt.target.profile_id,
                        revision: receipt.target.revision,
                    },
                )
            })
        }
        AdminAction::RunProjectCallback { callback_id } => app
            .state
            .execute_project_pdk_callback(&callback_id, &authority(view), view.reason.trim())
            .map(|receipt| {
                Some(format!(
                    "Executed signed callback {} for exact package {} {} and committed project receipt #{} at revision {}.",
                    receipt.execution.callback_id,
                    receipt.execution.package_binding.package_id,
                    receipt.execution.package_binding.revision,
                    receipt.sequence,
                    receipt.to_project_revision.get()
                ))
            }),
        AdminAction::ReportError(error) => Err(error),
    };

    match result {
        Ok(Some(message)) => {
            app.state
                .push_user_message(ConsoleMessage::info(message.clone()));
            app.state
                .ui
                .toasts
                .success(ctx, "PDK technology operation completed", message);
        }
        Ok(None) => {}
        Err(error) => {
            app.state
                .push_user_message(ConsoleMessage::error(error.clone()));
            app.state
                .ui
                .toasts
                .error_with_title(ctx, "PDK technology operation blocked", error);
        }
    }
}

fn revalidate_after_trust_change(
    candidate: &mut crate::state::pdk_config::PdkConfig,
) -> Vec<String> {
    let trust_store = candidate.publisher_trust_store.clone();
    match candidate
        .technology_registry
        .revalidate_installed(&trust_store)
    {
        Ok(_) => Vec::new(),
        Err(errors) => errors,
    }
}

fn persist_candidate(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    view: &mut AdminViewState,
    candidate: crate::state::pdk_config::PdkConfig,
    message: String,
    effect: AdminCommitEffect,
) -> Result<Option<String>, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ctx;
        candidate.save().map_err(|error| error.to_string())?;
        app.state.pdk_config = candidate;
        apply_admin_commit_effect(view, effect);
        Ok(Some(message))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = view;
        app.start_browser_pdk_administration_publication(
            ctx,
            candidate,
            "PDK technology updated",
            message,
            move |ctx| {
                ctx.data_mut(|data| {
                    let id = egui::Id::new(VIEW_STATE_ID);
                    let mut view = data.get_temp::<AdminViewState>(id).unwrap_or_default();
                    match effect {
                        AdminCommitEffect::None => {}
                        AdminCommitEffect::ClearTrustDraft {
                            publisher_id,
                            key_id,
                        } => {
                            if view.trust_publisher_id.trim() == publisher_id
                                && view.trust_key_id.trim() == key_id
                            {
                                view.trust_publisher_id.clear();
                                view.trust_key_id.clear();
                                view.trust_key_base64.clear();
                            }
                        }
                        AdminCommitEffect::SelectDisplayProfile {
                            profile_id,
                            revision,
                        } => {
                            view.selected_display_profile = Some((profile_id, revision));
                            view.display_draft_dirty = false;
                        }
                        AdminCommitEffect::TechnologyDraftSaved => {
                            view.technology_draft_dirty = false;
                            view.discard_technology_draft_armed = false;
                            view.technology_draft_error = None;
                        }
                        AdminCommitEffect::TechnologyDraftDiscarded => {
                            view.technology_draft = None;
                            view.technology_draft_dirty = false;
                            view.discard_technology_draft_armed = false;
                            view.technology_draft_error = None;
                        }
                    }
                    data.insert_temp(id, view);
                });
            },
        )?;
        Ok(None)
    }
}

fn apply_admin_commit_effect(view: &mut AdminViewState, effect: AdminCommitEffect) {
    match effect {
        AdminCommitEffect::None => {}
        AdminCommitEffect::ClearTrustDraft {
            publisher_id,
            key_id,
        } => {
            if view.trust_publisher_id.trim() == publisher_id && view.trust_key_id.trim() == key_id
            {
                view.trust_publisher_id.clear();
                view.trust_key_id.clear();
                view.trust_key_base64.clear();
            }
        }
        AdminCommitEffect::SelectDisplayProfile {
            profile_id,
            revision,
        } => {
            view.selected_display_profile = Some((profile_id, revision));
            view.display_draft_dirty = false;
        }
        AdminCommitEffect::TechnologyDraftSaved => {
            view.technology_draft_dirty = false;
            view.discard_technology_draft_armed = false;
            view.technology_draft_error = None;
        }
        AdminCommitEffect::TechnologyDraftDiscarded => {
            view.technology_draft = None;
            view.technology_draft_dirty = false;
            view.discard_technology_draft_armed = false;
            view.technology_draft_error = None;
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn request_package_import(ctx: &egui::Context, app: &mut RSpiceApp, view: &mut AdminViewState) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Signed RSpice PDK package", &["rspdk", "json"])
        .pick_file()
    else {
        return;
    };
    let base = app.state.pdk_config.clone();
    let authority = authority(view);
    let reason = view.reason.trim().to_owned();
    let repaint = ctx.clone();
    view.package_import_in_progress = true;
    std::thread::spawn(move || {
        let result = (|| {
            let metadata = std::fs::metadata(&path).map_err(|error| error.to_string())?;
            if metadata.len() > MAX_PDK_ARCHIVE_BYTES as u64 {
                return Err(format!(
                    "{} exceeds the {}-byte package limit",
                    path.display(),
                    MAX_PDK_ARCHIVE_BYTES
                ));
            }
            let bytes = std::fs::read(&path).map_err(|error| error.to_string())?;
            if bytes.len() > MAX_PDK_ARCHIVE_BYTES {
                return Err(format!(
                    "{} grew beyond the {}-byte package limit while it was being read",
                    path.display(),
                    MAX_PDK_ARCHIVE_BYTES
                ));
            }
            prepare_native_package_import(&base, &bytes, &authority, &reason)
        })();
        native_package_imports()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push_back(NativePackageImport { base, result });
        repaint.request_repaint();
    });
}

#[cfg(target_arch = "wasm32")]
fn request_package_import(ctx: &egui::Context, app: &mut RSpiceApp, view: &mut AdminViewState) {
    let repaint = ctx.clone();
    let base = app.state.pdk_config.clone();
    let metadata = BrowserPdkImportMetadata {
        protocol_version: BROWSER_PDK_IMPORT_PROTOCOL_VERSION,
        config: base.clone(),
        authority: authority(view),
        reason: view.reason.trim().to_owned(),
    };
    view.package_import_in_progress = true;
    wasm_bindgen_futures::spawn_local(async move {
        let picked = rfd::AsyncFileDialog::new()
            .add_filter("Signed RSpice PDK package", &["rspdk", "json"])
            .pick_file()
            .await;
        match picked {
            None => {
                BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().push_back(Ok(None)));
            }
            Some(file) => {
                let size = file.inner().size();
                if !size.is_finite() || size < 0.0 || size > MAX_PDK_ARCHIVE_BYTES as f64 {
                    BROWSER_PACKAGE_IMPORTS.with(|queue| {
                        queue.borrow_mut().push_back(Err(format!(
                            "Selected package exceeds the {MAX_PDK_ARCHIVE_BYTES}-byte limit"
                        )))
                    });
                } else {
                    let bytes = file.read().await;
                    if bytes.len() > MAX_PDK_ARCHIVE_BYTES {
                        BROWSER_PACKAGE_IMPORTS.with(|queue| {
                            queue.borrow_mut().push_back(Err(format!(
                                "Selected package grew beyond the {MAX_PDK_ARCHIVE_BYTES}-byte limit"
                            )))
                        });
                    } else if let Err(error) =
                        browser_pdk_import_worker::start(metadata, bytes, base, repaint.clone())
                    {
                        BROWSER_PACKAGE_IMPORTS
                            .with(|queue| queue.borrow_mut().push_back(Err(error)));
                    }
                }
            }
        }
        repaint.request_repaint();
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn poll_native_package_imports(ctx: &egui::Context, app: &mut RSpiceApp) {
    let completions = native_package_imports()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .drain(..)
        .collect::<Vec<_>>();
    for completion in completions {
        let mut view = ctx
            .data(|data| data.get_temp::<AdminViewState>(egui::Id::new(VIEW_STATE_ID)))
            .unwrap_or_default();
        view.package_import_in_progress = false;
        if app.state.pdk_config != completion.base {
            apply_action(
                ctx,
                app,
                &mut view,
                AdminAction::ReportError(
                    "PDK configuration changed while the signed package was being validated; the stale candidate was discarded without mutation."
                        .to_owned(),
                ),
            );
        } else {
            match completion.result {
                Ok(candidate) => {
                    view.selected =
                        Some((candidate.package_id.clone(), candidate.revision.clone()));
                    let result = persist_candidate(
                        ctx,
                        app,
                        &mut view,
                        candidate.config,
                        format!(
                            "Installed trusted package {} {} as audit receipt #{}.",
                            candidate.package_id, candidate.revision, candidate.sequence
                        ),
                        AdminCommitEffect::None,
                    );
                    if let Err(error) = result {
                        apply_action(ctx, app, &mut view, AdminAction::ReportError(error));
                    }
                }
                Err(error) => apply_action(ctx, app, &mut view, AdminAction::ReportError(error)),
            }
        }
        ctx.data_mut(|data| {
            data.insert_temp(egui::Id::new(VIEW_STATE_ID), view);
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_package_imports(ctx: &egui::Context, app: &mut RSpiceApp) {
    let completions =
        BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    if !completions.is_empty() {
        browser_pdk_import_worker::finish();
    }
    for completion in completions {
        match completion {
            Ok(Some(candidate)) => {
                let mut view = ctx
                    .data(|data| data.get_temp::<AdminViewState>(egui::Id::new(VIEW_STATE_ID)))
                    .unwrap_or_default();
                view.package_import_in_progress = false;
                if app.state.pdk_config != candidate.base {
                    apply_action(
                        ctx,
                        app,
                        &mut view,
                        AdminAction::ReportError(
                            "PDK configuration changed while the browser worker was validating the signed package; the stale candidate was discarded without mutation."
                                .to_owned(),
                        ),
                    );
                } else {
                    let BrowserPdkImportPayload {
                        protocol_version,
                        mut config,
                        validated_packages,
                        package_id,
                        revision,
                        sequence,
                    } = candidate.payload;
                    let restore = if protocol_version == BROWSER_PDK_IMPORT_PROTOCOL_VERSION {
                        config
                            .technology_registry
                            .restore_worker_validated_packages(validated_packages)
                            .map_err(|error| error.to_string())
                    } else {
                        Err(format!(
                            "Unsupported browser PDK import response protocol {protocol_version}."
                        ))
                    };
                    match restore {
                        Ok(()) => {
                            view.selected = Some((package_id.clone(), revision.clone()));
                            if let Err(error) = persist_candidate(
                                ctx,
                                app,
                                &mut view,
                                config,
                                format!(
                                    "Installed trusted package {package_id} {revision} as audit receipt #{sequence}."
                                ),
                                AdminCommitEffect::None,
                            ) {
                                apply_action(ctx, app, &mut view, AdminAction::ReportError(error));
                            }
                        }
                        Err(error) => {
                            apply_action(ctx, app, &mut view, AdminAction::ReportError(error))
                        }
                    }
                }
                ctx.data_mut(|data| {
                    data.insert_temp(egui::Id::new(VIEW_STATE_ID), view);
                });
            }
            Ok(None) => {
                ctx.data_mut(|data| {
                    let id = egui::Id::new(VIEW_STATE_ID);
                    let mut view = data.get_temp::<AdminViewState>(id).unwrap_or_default();
                    view.package_import_in_progress = false;
                    data.insert_temp(id, view);
                });
            }
            Err(error) => {
                let mut view = ctx
                    .data(|data| data.get_temp::<AdminViewState>(egui::Id::new(VIEW_STATE_ID)))
                    .unwrap_or_default();
                view.package_import_in_progress = false;
                apply_action(ctx, app, &mut view, AdminAction::ReportError(error));
                ctx.data_mut(|data| {
                    data.insert_temp(egui::Id::new(VIEW_STATE_ID), view);
                });
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod browser_pdk_import_worker {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use js_sys::{Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::*;

    const MAX_RESPONSE_BYTES: usize = 256 * 1024 * 1024;

    struct ActiveWorker {
        worker: web_sys::Worker,
        _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
        _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
    }

    impl Drop for ActiveWorker {
        fn drop(&mut self) {
            self.worker.set_onmessage(None);
            self.worker.set_onerror(None);
            self.worker.set_onmessageerror(None);
            self.worker.terminate();
        }
    }

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveWorker>> = const { RefCell::new(None) };
    }

    pub(super) fn start(
        metadata: BrowserPdkImportMetadata,
        archive: Vec<u8>,
        base: crate::state::pdk_config::PdkConfig,
        repaint: egui::Context,
    ) -> Result<(), String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser PDK package validator is already active.".to_owned());
        }
        let metadata = serde_json::to_vec(&metadata)
            .map_err(|error| format!("Could not encode PDK import metadata: {error}"))?;
        let id = NEXT_REQUEST_ID.with(|next| {
            let id = next.get().wrapping_add(1).max(1);
            next.set(id);
            id
        });
        let metadata = transferred_array(&metadata)?;
        let archive = transferred_array(&archive)?;
        let request = Object::new();
        Reflect::set(&request, &JsValue::from_str("metadataBytes"), &metadata)
            .map_err(js_error_message)?;
        Reflect::set(&request, &JsValue::from_str("archiveBytes"), &archive)
            .map_err(js_error_message)?;

        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker = web_sys::Worker::new_with_options(&worker_url()?, &options)
            .map_err(js_error_message)?;
        let completed = Rc::new(Cell::new(false));

        let success_base = base.clone();
        let success_repaint = repaint.clone();
        let success_completed = Rc::clone(&completed);
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                if numeric_property(&data, "id") != Some(id) {
                    return;
                }
                let result = match string_property(&data, "type").as_deref() {
                    Some("pdk-import-result") => {
                        Reflect::get(&data, &JsValue::from_str("response"))
                            .map_err(js_error_message)
                            .and_then(|response| decode_response(&response))
                            .map(|payload| {
                                Some(BrowserPackageImportCandidate {
                                    base: success_base.clone(),
                                    payload,
                                })
                            })
                    }
                    Some("pdk-import-error") | Some("error") => {
                        Err(string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| "Browser PDK package validator failed.".to_owned()))
                    }
                    _ => return,
                };
                complete_once(&success_completed, &success_repaint, result);
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let error_repaint = repaint.clone();
        let error_completed = Rc::clone(&completed);
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                complete_once(
                    &error_completed,
                    &error_repaint,
                    Err(if event.message().is_empty() {
                        "Browser PDK package validator failed.".to_owned()
                    } else {
                        event.message()
                    }),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_repaint = repaint;
        let message_completed = completed;
        let onmessageerror =
            Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(move |_event| {
                complete_once(
                    &message_completed,
                    &message_repaint,
                    Err("Browser PDK package validator returned an unreadable message.".to_owned()),
                );
            }));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run-pdk-import"),
        )
        .map_err(js_error_message)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(f64::from(id)),
        )
        .map_err(js_error_message)?;
        Reflect::set(&message, &JsValue::from_str("request"), &request)
            .map_err(js_error_message)?;
        let transfer = js_sys::Array::of2(&metadata.buffer(), &archive.buffer());
        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveWorker {
                worker: worker.clone(),
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
            });
        });
        if let Err(error) = worker.post_message_with_transfer(&message, &transfer) {
            finish();
            return Err(format!(
                "Could not dispatch browser PDK package validation: {}",
                js_error_message(error)
            ));
        }
        Ok(())
    }

    fn complete_once(
        completed: &Cell<bool>,
        repaint: &egui::Context,
        result: BrowserPackageImport,
    ) {
        if completed.replace(true) {
            return;
        }
        BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().push_back(result));
        repaint.request_repaint();
    }

    pub(super) fn finish() {
        ACTIVE_WORKER.with(|active| {
            active.borrow_mut().take();
        });
    }

    fn transferred_array(bytes: &[u8]) -> Result<Uint8Array, String> {
        let length = u32::try_from(bytes.len())
            .map_err(|_| "PDK package worker input exceeds browser array limits.".to_owned())?;
        let view = Uint8Array::new_with_length(length);
        view.copy_from(bytes);
        Ok(view)
    }

    fn decode_response(value: &JsValue) -> Result<BrowserPdkImportPayload, String> {
        let protocol = numeric_property(value, "protocolVersion")
            .ok_or_else(|| "PDK import worker response has no protocol version.".to_owned())?;
        if protocol != u32::from(BROWSER_PDK_IMPORT_PROTOCOL_VERSION) {
            return Err(format!(
                "Unsupported PDK import worker protocol {protocol}."
            ));
        }
        let bytes =
            Reflect::get(value, &JsValue::from_str("payloadBytes")).map_err(js_error_message)?;
        let bytes = Uint8Array::new(&bytes);
        let length = usize::try_from(bytes.length())
            .map_err(|_| "PDK import worker response exceeds host limits.".to_owned())?;
        if length == 0 || length > MAX_RESPONSE_BYTES {
            return Err(format!(
                "PDK import worker response contains {length} bytes; the supported range is 1..={MAX_RESPONSE_BYTES}."
            ));
        }
        let mut encoded = vec![0; length];
        bytes.copy_to(&mut encoded);
        serde_json::from_slice(&encoded)
            .map_err(|error| format!("PDK import worker returned invalid candidate data: {error}"))
    }

    fn worker_url() -> Result<String, String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .map_err(js_error_message)?
        .as_string()
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "Browser PDK package validator URL is unavailable.".to_owned())
    }

    fn string_property(value: &JsValue, property: &str) -> Option<String> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_string())
    }

    fn numeric_property(value: &JsValue, property: &str) -> Option<u32> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_f64())
            .filter(|value| {
                value.is_finite()
                    && *value >= 0.0
                    && *value <= f64::from(u32::MAX)
                    && value.fract() == 0.0
            })
            .map(|value| value as u32)
    }

    fn js_error_message(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| {
                Reflect::get(&error, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|message| message.as_string())
            })
            .unwrap_or_else(|| "unknown JavaScript error".to_owned())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn run_pdk_import_worker_request_value(
    request: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Object, Reflect, Uint8Array};
    use wasm_bindgen::JsValue;

    let metadata = Uint8Array::new(&Reflect::get(
        &request,
        &JsValue::from_str("metadataBytes"),
    )?);
    let mut metadata_bytes = vec![
        0;
        usize::try_from(metadata.length()).map_err(|_| {
            JsValue::from_str("PDK import metadata exceeds host limits.")
        })?
    ];
    metadata.copy_to(&mut metadata_bytes);
    let metadata: BrowserPdkImportMetadata = serde_json::from_slice(&metadata_bytes)
        .map_err(|error| JsValue::from_str(&format!("Invalid PDK import metadata: {error}")))?;
    if metadata.protocol_version != BROWSER_PDK_IMPORT_PROTOCOL_VERSION {
        return Err(JsValue::from_str(&format!(
            "Unsupported PDK import protocol {}.",
            metadata.protocol_version
        )));
    }
    let archive = Uint8Array::new(&Reflect::get(&request, &JsValue::from_str("archiveBytes"))?);
    let archive_length = usize::try_from(archive.length())
        .map_err(|_| JsValue::from_str("PDK package exceeds host limits."))?;
    if archive_length == 0 || archive_length > MAX_PDK_ARCHIVE_BYTES {
        return Err(JsValue::from_str(
            "PDK package size is outside supported limits.",
        ));
    }
    let mut archive_bytes = vec![0; archive_length];
    archive.copy_to(&mut archive_bytes);

    let mut config = metadata.config;
    if !config.technology_registry.archives().is_empty() {
        let trust = config.publisher_trust_store.clone();
        config
            .technology_registry
            .revalidate_installed(&trust)
            .map_err(|errors| JsValue::from_str(&errors.join("; ")))?;
    }
    let receipt = config
        .technology_registry
        .install_archive_bytes(
            &archive_bytes,
            &config.publisher_trust_store,
            &metadata.authority,
            &metadata.reason,
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let validated_packages = config.technology_registry.take_worker_validated_packages();
    let payload = BrowserPdkImportPayload {
        protocol_version: BROWSER_PDK_IMPORT_PROTOCOL_VERSION,
        config,
        validated_packages,
        package_id: receipt.target.package_id,
        revision: receipt.target.revision,
        sequence: receipt.sequence,
    };
    let encoded = serde_json::to_vec(&payload)
        .map_err(|error| JsValue::from_str(&format!("Could not encode PDK candidate: {error}")))?;
    if encoded.is_empty() || encoded.len() > 256 * 1024 * 1024 {
        return Err(JsValue::from_str(
            "Validated PDK candidate exceeds the browser worker response limit.",
        ));
    }
    let bytes = Uint8Array::new_with_length(
        u32::try_from(encoded.len())
            .map_err(|_| JsValue::from_str("PDK candidate exceeds browser array limits."))?,
    );
    bytes.copy_from(&encoded);
    let response = Object::new();
    Reflect::set(
        &response,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(BROWSER_PDK_IMPORT_PROTOCOL_VERSION)),
    )?;
    Reflect::set(&response, &JsValue::from_str("payloadBytes"), &bytes)?;
    Ok(response.into())
}

#[cfg(test)]
mod tests;

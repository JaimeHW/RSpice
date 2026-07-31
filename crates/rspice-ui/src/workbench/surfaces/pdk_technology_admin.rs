//! Signed PDK technology-package administration.
//!
//! This surface projects only validated package state. Persisted archives do
//! not regain runtime authority after restart until their signatures,
//! artifacts, contracts, trust roots, and audit chain have all been checked.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use egui::{Align, Color32, Frame, Grid, Layout, RichText, ScrollArea, Sense, Stroke, Ui};

use crate::diagnostics::ConsoleMessage;
use crate::state::pdk_config::{
    MAX_PDK_ARCHIVE_BYTES, PdkAdministrativeAuthority, PdkDisplayFillStyle,
    PdkDisplayProfileBinding, PdkDisplayProfileDraft, PdkDisplayProfileScope, PdkExecutionTarget,
    PdkExtractionQuantity, PdkTechnologyArtifactKind, PdkTechnologyAuditAction,
    PdkTechnologyBinding, PdkTechnologyDiffArea, PdkTechnologyDiffEntry, PdkTechnologyDiffError,
    PdkTechnologyDiffImpact, PdkTechnologyDiffKind, PdkTechnologyLayer, PdkTechnologyRevisionDiff,
    PdkTrustAuditAction, ProjectPdkCallbackReceipt, TrustedPdkPublisherKey,
    ValidatedPdkTechnologyPackage,
};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::Button;
use crate::workbench::RSpiceApp;

const VIEW_STATE_ID: &str = "pdk-technology-admin-view-state";
const COMPACT_BREAKPOINT: f32 = 760.0;
const MAX_TRUST_KEY_BASE64_INPUT: usize = 256;

#[cfg(target_arch = "wasm32")]
type BrowserPackageImport = Result<Option<Vec<u8>>, String>;

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PACKAGE_IMPORTS:
        std::cell::RefCell<std::collections::VecDeque<BrowserPackageImport>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
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
            Self::Connectivity => "Connectivity",
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
}

enum AdminAction {
    Import(Vec<u8>),
    ReportError(String),
    Revalidate,
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
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    #[cfg(target_arch = "wasm32")]
    poll_browser_package_imports(ui.ctx(), app);

    let mut view = ui
        .ctx()
        .data(|data| data.get_temp::<AdminViewState>(egui::Id::new(VIEW_STATE_ID)))
        .unwrap_or_default();
    let snapshot = registry_snapshot(app);
    reconcile_selection(&mut view, &snapshot);

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
            .data_mut(|data| data.insert_temp(egui::Id::new(VIEW_STATE_ID), view));
    }
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
                .enabled(has_authority && !snapshot.trust_keys.is_empty())
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
                request_package_import(ui.ctx(), action);
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
        AdminSection::Layers => layers_section(ui, package),
        AdminSection::Display => display_section(ui, package, snapshot, view, action),
        AdminSection::StreamMaps => stream_section(ui, package),
        AdminSection::Connectivity => connectivity_section(ui, package),
        AdminSection::Recognition => recognition_section(ui, package),
        AdminSection::Extraction => extraction_section(ui, package),
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
        .fill(if index % 2 == 0 {
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

            if let Some((profile_id, revision)) = &view.selected_display_profile {
                if let Some(profile) = package_profiles.iter().find(|profile| {
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

fn layers_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage) {
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

fn stream_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage) {
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

fn connectivity_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage) {
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

fn recognition_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage) {
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

fn extraction_section(ui: &mut Ui, package: &ValidatedPdkTechnologyPackage) {
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
                            "{} · {} · {}",
                            callback.callback_id,
                            callback.artifact_path,
                            format!(
                                "ABI {} / {}() -> i32 / {}",
                                callback.abi_version,
                                callback.entrypoint,
                                callback
                                    .capabilities
                                    .iter()
                                    .map(|capability| format!("{capability:?}"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
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
    let result = match action {
        AdminAction::Import(bytes) => {
            let mut candidate = app.state.pdk_config.clone();
            match candidate.technology_registry.install_archive_bytes(
                &bytes,
                &candidate.publisher_trust_store,
                &authority(view),
                view.reason.trim(),
            ) {
                Ok(receipt) => {
                    view.selected = Some((
                        receipt.target.package_id.clone(),
                        receipt.target.revision.clone(),
                    ));
                    persist_candidate(
                        ctx,
                        app,
                        candidate,
                        format!(
                            "Installed trusted package {} {} as audit receipt #{}.",
                            receipt.target.package_id, receipt.target.revision, receipt.sequence
                        ),
                        AdminCommitEffect::None,
                    )
                }
                Err(error) => Err(error.to_string()),
            }
        }
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
    _ctx: &egui::Context,
    app: &mut RSpiceApp,
    candidate: crate::state::pdk_config::PdkConfig,
    message: String,
    _effect: AdminCommitEffect,
) -> Result<Option<String>, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        candidate.save().map_err(|error| error.to_string())?;
        app.state.pdk_config = candidate;
        Ok(Some(message))
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.start_browser_pdk_administration_publication(
            _ctx,
            candidate,
            "PDK technology updated",
            message,
            move |ctx| {
                ctx.data_mut(|data| {
                    let id = egui::Id::new(VIEW_STATE_ID);
                    let mut view = data.get_temp::<AdminViewState>(id).unwrap_or_default();
                    match _effect {
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
                    }
                    data.insert_temp(id, view);
                });
            },
        )?;
        Ok(None)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn request_package_import(_ctx: &egui::Context, action: &mut Option<AdminAction>) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Signed RSpice PDK package", &["rspdk", "json"])
        .pick_file()
    else {
        return;
    };
    let bytes = std::fs::metadata(&path)
        .map_err(|error| error.to_string())
        .and_then(|metadata| {
            if metadata.len() > MAX_PDK_ARCHIVE_BYTES as u64 {
                Err(format!(
                    "{} exceeds the {}-byte package limit",
                    path.display(),
                    MAX_PDK_ARCHIVE_BYTES
                ))
            } else {
                std::fs::read(&path).map_err(|error| error.to_string())
            }
        });
    match bytes {
        Ok(bytes) if bytes.len() <= MAX_PDK_ARCHIVE_BYTES => {
            *action = Some(AdminAction::Import(bytes));
        }
        Ok(_) => {
            *action = Some(AdminAction::ReportError(format!(
                "{} grew beyond the {}-byte package limit while it was being read",
                path.display(),
                MAX_PDK_ARCHIVE_BYTES
            )));
        }
        Err(error) => {
            *action = Some(AdminAction::ReportError(error));
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn request_package_import(ctx: &egui::Context, _action: &mut Option<AdminAction>) {
    let repaint = ctx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let picked = rfd::AsyncFileDialog::new()
            .add_filter("Signed RSpice PDK package", &["rspdk", "json"])
            .pick_file()
            .await;
        let result = match picked {
            None => Ok(None),
            Some(file) => {
                let size = file.inner().size();
                if !size.is_finite() || size < 0.0 || size > MAX_PDK_ARCHIVE_BYTES as f64 {
                    Err(format!(
                        "Selected package exceeds the {MAX_PDK_ARCHIVE_BYTES}-byte limit"
                    ))
                } else {
                    let bytes = file.read().await;
                    if bytes.len() > MAX_PDK_ARCHIVE_BYTES {
                        Err(format!(
                            "Selected package grew beyond the {MAX_PDK_ARCHIVE_BYTES}-byte limit"
                        ))
                    } else {
                        Ok(Some(bytes))
                    }
                }
            }
        };
        BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().push_back(result));
        repaint.request_repaint();
    });
}

#[cfg(target_arch = "wasm32")]
fn poll_browser_package_imports(ctx: &egui::Context, app: &mut RSpiceApp) {
    let completions =
        BROWSER_PACKAGE_IMPORTS.with(|queue| queue.borrow_mut().drain(..).collect::<Vec<_>>());
    for completion in completions {
        match completion {
            Ok(Some(bytes)) => {
                let mut view = ctx
                    .data(|data| data.get_temp::<AdminViewState>(egui::Id::new(VIEW_STATE_ID)))
                    .unwrap_or_default();
                apply_action(ctx, app, &mut view, AdminAction::Import(bytes));
                ctx.data_mut(|data| {
                    data.insert_temp(egui::Id::new(VIEW_STATE_ID), view);
                });
            }
            Ok(None) => {}
            Err(error) => {
                app.state
                    .ui
                    .toasts
                    .error_with_title(ctx, "PDK package import blocked", error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_surface_exposes_real_actions_and_honest_trust_boundary() {
        let mut app = RSpiceApp::test_instance();
        app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_100.0, 760.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("PDK accessibility tree")
            .nodes;
        let labels = nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();
        assert!(labels.contains(&"PDK technology administration"));
        assert!(labels.iter().any(|label| {
            label.contains("Import signed package") && label.contains("no publisher trust keys")
        }));
        assert!(labels.contains(&"Revalidate installed packages"));
        assert!(labels.iter().any(|label| {
            label.contains("Compare selected signed PDK revision")
                && label.contains("install another trusted revision")
        }));
    }

    #[test]
    fn phone_header_stacks_runtime_status_below_description_without_overlap() {
        let mut app = RSpiceApp::test_instance();
        app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(390.0, 844.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let nodes = output
            .platform_output
            .accesskit_update
            .expect("PDK phone accessibility tree")
            .nodes;
        let description = nodes
            .iter()
            .find(|(_, node)| {
                node.label().is_some_and(|label| {
                    label.contains(
                        "Validate signed technology packages, inspect exact physical resources",
                    )
                })
            })
            .and_then(|(_, node)| node.bounds())
            .unwrap_or_else(|| {
                panic!(
                    "PDK header description bounds; labels={:?}",
                    nodes
                        .iter()
                        .filter_map(|(_, node)| node.label())
                        .collect::<Vec<_>>()
                )
            });
        let status = nodes
            .iter()
            .find(|(_, node)| node.label() == Some("NO ACTIVE BINDING"))
            .and_then(|(_, node)| node.bounds())
            .expect("PDK runtime status bounds");

        assert!(
            status.y0 >= description.y1 - 1.0,
            "compact runtime status overlaps the header description: description={description:?}, status={status:?}"
        );
    }

    #[test]
    fn empty_registry_still_exposes_accessible_trust_root_provisioning() {
        let mut app = RSpiceApp::test_instance();
        app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.data_mut(|data| {
            data.insert_temp(
                egui::Id::new(VIEW_STATE_ID),
                AdminViewState {
                    section: AdminSection::TrustAudit,
                    ..AdminViewState::default()
                },
            );
        });
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_100.0, 760.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let accessibility = output
            .platform_output
            .accesskit_update
            .expect("PDK accessibility tree");
        let labels = accessibility
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();
        assert!(labels.iter().any(|label| {
            label.contains("Provision publisher key")
                && label.contains("actor, authority, and reason")
        }));
        assert!(labels.contains(&"Copy complete audit JSON"));
    }

    #[test]
    fn display_profile_editor_is_accessible_at_phone_width_and_package_bound() {
        let mut app = RSpiceApp::test_instance();
        let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
        app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
        app.state.pdk_config.publisher_trust_store = trust;
        app.state
            .pdk_config
            .technology_registry
            .install_archive_bytes(
                &bytes,
                &app.state.pdk_config.publisher_trust_store,
                &authority,
                "install display editor fixture",
            )
            .expect("install package");
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.data_mut(|data| {
            data.insert_temp(
                egui::Id::new(VIEW_STATE_ID),
                AdminViewState {
                    section: AdminSection::Display,
                    actor_id: authority.actor_id,
                    authority_id: authority.authority_id,
                    reason: "publish reviewed display profile".to_owned(),
                    ..AdminViewState::default()
                },
            );
        });

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(430.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let accessibility = output
            .platform_output
            .accesskit_update
            .expect("PDK display accessibility tree");
        let labels = accessibility
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();

        assert!(labels.contains(&"PDK technology administration"));
        assert!(labels.contains(&"New from signed defaults"));
        assert!(labels.contains(&"Reset draft to signed colors"));
        assert!(labels.contains(&"Save and activate immutable display-profile revision"));
        assert!(labels.iter().any(|label| label.starts_with("Visible ")));
        assert!(labels.iter().any(|label| label.starts_with("Selectable ")));
        assert!(
            labels
                .iter()
                .any(|label| label.starts_with("Screen color for "))
        );
        assert!(
            labels
                .iter()
                .any(|label| label.starts_with("Print fill for "))
        );
    }

    #[test]
    fn revision_comparison_fails_closed_at_phone_width_without_a_second_trusted_revision() {
        let mut app = RSpiceApp::test_instance();
        let (bytes, trust, authority) = crate::state::pdk_config::signed_technology_test_fixture();
        app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
        app.state.pdk_config.publisher_trust_store = trust;
        app.state
            .pdk_config
            .technology_registry
            .install_archive_bytes(
                &bytes,
                &app.state.pdk_config.publisher_trust_store,
                &authority,
                "install comparison fixture",
            )
            .expect("install package");
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.data_mut(|data| {
            data.insert_temp(
                egui::Id::new(VIEW_STATE_ID),
                AdminViewState {
                    section: AdminSection::Compare,
                    ..AdminViewState::default()
                },
            );
        });

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(430.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let accessibility = output
            .platform_output
            .accesskit_update
            .expect("PDK comparison accessibility tree");
        let labels = accessibility
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();

        assert!(labels.iter().any(|label| {
            label.contains("Compare selected signed PDK revision")
                && label.contains("install another trusted revision")
        }));
    }

    #[test]
    fn revision_comparison_table_is_complete_and_accessible_at_phone_width() {
        let mut app = RSpiceApp::test_instance();
        let (baseline, candidate, trust, authority) =
            crate::state::pdk_config::signed_technology_diff_test_fixture();
        app.state.pdk_config = crate::state::pdk_config::PdkConfig::default();
        app.state.pdk_config.publisher_trust_store = trust;
        app.state
            .pdk_config
            .technology_registry
            .install_archive_bytes(
                &baseline,
                &app.state.pdk_config.publisher_trust_store,
                &authority,
                "install comparison baseline",
            )
            .expect("install baseline");
        app.state
            .pdk_config
            .technology_registry
            .install_archive_bytes(
                &candidate,
                &app.state.pdk_config.publisher_trust_store,
                &authority,
                "install comparison candidate",
            )
            .expect("install candidate");
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.data_mut(|data| {
            data.insert_temp(
                egui::Id::new(VIEW_STATE_ID),
                AdminViewState {
                    section: AdminSection::Compare,
                    selected: Some(("demo180".to_owned(), "2.4.0".to_owned())),
                    compare_against: Some(("demo180".to_owned(), "2.3.1".to_owned())),
                    ..AdminViewState::default()
                },
            );
        });

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(430.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let accessibility = output
            .platform_output
            .accesskit_update
            .expect("PDK comparison accessibility tree");
        let nodes = &accessibility.nodes;
        let labels = nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();

        assert!(nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Table
                && node.label() == Some("Signed PDK revision differences")
        }));
        assert_eq!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::ColumnHeader)
                .count(),
            6
        );
        assert!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Row)
                .count()
                >= 4
        );
        assert!(
            nodes
                .iter()
                .filter(|(_, node)| node.role() == egui::accesskit::Role::Cell)
                .count()
                >= 18
        );
        assert!(labels.contains(&"Copy exact signed PDK revision comparison as JSON"));
        assert!(labels.contains(&"active"));
        assert!(labels.contains(&"signed archive digest"));
    }

    #[test]
    fn project_callback_workflow_executes_exact_pin_and_exposes_durable_receipt() {
        let mut app = RSpiceApp::test_instance();
        app.state.provision_test_project_technology_contract();
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut view = AdminViewState {
            section: AdminSection::Resources,
            actor_id: "callback-operator@rspice.invalid".to_owned(),
            authority_id: "test:project-callback-ui".to_owned(),
            reason: "Execute and retain exact project callback evidence".to_owned(),
            ..AdminViewState::default()
        };

        apply_action(
            &ctx,
            &mut app,
            &mut view,
            AdminAction::RunProjectCallback {
                callback_id: "derive-device".to_owned(),
            },
        );
        assert_eq!(app.state.workspace.pdk_callback_receipts().len(), 1);
        app.state
            .workspace
            .validate_pdk_callback_receipts()
            .expect("committed callback ledger validates");
        ctx.data_mut(|data| {
            data.insert_temp(egui::Id::new(VIEW_STATE_ID), view);
        });

        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(430.0, 3_000.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let accessibility = output
            .platform_output
            .accesskit_update
            .expect("PDK callback accessibility tree");
        let labels = accessibility
            .nodes
            .iter()
            .filter_map(|(_, node)| node.label())
            .collect::<Vec<_>>();

        assert!(labels.iter().any(|label| {
            label.contains("Run callback derive-device")
                && label.contains("exact attached project revision")
        }));
        assert!(
            labels
                .iter()
                .any(|label| label.contains("receipt verified")),
            "callback receipt status is absent from accessibility labels: {labels:?}"
        );
        assert!(
            labels
                .iter()
                .any(|label| { label.contains("Copy exact project callback receipt 1 as JSON") })
        );
    }
}

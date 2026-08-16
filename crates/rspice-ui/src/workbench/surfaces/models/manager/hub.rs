//! Distributed model packs, and the shipped corpus beside them.
//!
//! The Models workspace has one packs scope and two things to say in it: which
//! signed releases the catalog publishes and which of them this machine holds,
//! and — separately — what the shipped corpus tree on this installation
//! contains. Those are different distribution mechanisms with different
//! evidence, so they are two tables rather than one merged list that would have
//! to lie about where a row came from. The corpus table renders only when a
//! corpus is actually installed.
//!
//! # The hub table is one list, not two
//!
//! Installed and available releases are the same question asked once: what can
//! this project use, and what would it cost to get there. Splitting them into
//! "installed" and "browse" tabs makes a user ask it twice and makes an
//! available update invisible until they think to look. So every release the
//! catalog publishes appears once, carrying the state that says what can be
//! done with it now — and a release this build cannot run appears too, saying
//! exactly which capability it needs, because hiding it would turn a plain
//! answer into a mystery.

use super::*;

use crate::services::model_hub::ModelHubService;
use crate::state::model_hub::missing_capabilities;
use crate::workbench::app::ModelHubRequest;
use crate::workbench::state::{ModelHubFacet, PackReleaseConfirmation};

/// One published release, as the workspace lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HubPackRow {
    pub pack_id: String,
    pub name: String,
    pub category: String,
    pub version: String,
    pub state: HubPackState,
    pub spdx: String,
    pub archive_length: u64,
    pub parts: usize,
    pub capabilities: Vec<String>,
}

/// What can be done with one published release right now.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum HubPackState {
    Installed,
    Available,
    /// A newer release than the one installed. Both versions are named because
    /// the action is "replace this with that", not "get the latest".
    UpdateAvailable {
        installed: String,
    },
    /// This engine build does not offer what the release requires.
    Incompatible {
        missing: Vec<String>,
    },
}

impl HubPackState {
    pub(super) const fn pill(&self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Available => "available",
            Self::UpdateAvailable { .. } => "update",
            Self::Incompatible { .. } => "incompatible",
        }
    }

    const fn matches(&self, facet: ModelHubFacet) -> bool {
        match facet {
            ModelHubFacet::All => true,
            ModelHubFacet::Installed => matches!(self, Self::Installed),
            ModelHubFacet::Available => matches!(self, Self::Available),
            ModelHubFacet::Updatable => matches!(self, Self::UpdateAvailable { .. }),
            ModelHubFacet::Incompatible => matches!(self, Self::Incompatible { .. }),
        }
    }
}

/// Everything the packs scope reads about distributed releases in one frame.
///
/// It is built once per frame from the session hub rather than queried per row.
/// The projection is small, and computing it in one place is what keeps the
/// table, the detail pane, and the freshness line from disagreeing about which
/// release is current.
#[derive(Debug, Default, Clone)]
pub(super) struct HubCatalog {
    pub rows: Vec<HubPackRow>,
    /// Whole days since the cached catalog was signed.
    pub age_days: Option<u64>,
    /// Why there is no hub at all, in words a user can act on.
    pub unavailable: Option<String>,
    pub stale: bool,
}

/// Projects the session hub into what this workspace lists.
pub(super) fn hub_catalog(service: &ModelHubService) -> HubCatalog {
    let mut catalog = HubCatalog {
        age_days: service.catalog_age_days(),
        unavailable: service.unavailable_reason().map(str::to_owned),
        stale: service.catalog_is_stale(),
        ..HubCatalog::default()
    };
    let Some(hub) = service.hub() else {
        return catalog;
    };
    let installed = hub.installed();
    if let Some(snapshot) = hub.snapshot() {
        for pack in &snapshot.packs {
            let held = installed
                .iter()
                .find(|candidate| candidate.pack_id() == pack.id)
                .map(|candidate| candidate.version().to_owned());
            let newest = pack
                .releases
                .iter()
                .map(|release| release.version.as_str())
                .max();
            for release in &pack.releases {
                let missing = missing_capabilities(&release.capabilities);
                let state = if !missing.is_empty() {
                    HubPackState::Incompatible { missing }
                } else if held.as_deref() == Some(release.version.as_str()) {
                    HubPackState::Installed
                } else if newest == Some(release.version.as_str())
                    && held
                        .as_deref()
                        .is_some_and(|held| held < release.version.as_str())
                {
                    // Only the newest listed release offers an update. An older
                    // one is history, and offering it would make every
                    // superseded version look like an action.
                    HubPackState::UpdateAvailable {
                        installed: held.clone().unwrap_or_default(),
                    }
                } else {
                    HubPackState::Available
                };
                catalog.rows.push(HubPackRow {
                    pack_id: pack.id.clone(),
                    name: pack.name.clone(),
                    category: pack.category.clone(),
                    version: release.version.clone(),
                    state,
                    spdx: release.spdx.clone(),
                    archive_length: release.archive_length,
                    parts: release.parts.len(),
                    capabilities: release.capabilities.clone(),
                });
            }
        }
    }
    // A release installed from a catalog that has since dropped it is still on
    // this machine and still usable, so it is listed from its own manifest
    // rather than vanishing when the catalog forgets it.
    for pack in installed {
        if catalog
            .rows
            .iter()
            .any(|row| row.pack_id == pack.pack_id() && row.version == pack.version())
        {
            continue;
        }
        catalog.rows.push(HubPackRow {
            pack_id: pack.pack_id().to_owned(),
            name: pack.manifest.pack.name.clone(),
            category: pack.manifest.pack.category.clone(),
            version: pack.version().to_owned(),
            state: HubPackState::Installed,
            spdx: pack.manifest.license.spdx.clone(),
            archive_length: 0,
            parts: pack.manifest.parts.len(),
            capabilities: pack.manifest.requires.capabilities.clone(),
        });
    }
    catalog.rows.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| right.version.cmp(&left.version))
    });
    catalog
}

/// A byte count a person reads rather than counts.
pub(super) fn byte_size(bytes: u64) -> String {
    const UNITS: [(&str, u64); 3] = [("MB", 1024 * 1024), ("kB", 1024), ("B", 1)];
    if bytes == 0 {
        return "—".to_owned();
    }
    for (unit, scale) in UNITS {
        if bytes >= scale {
            let value = bytes as f64 / scale as f64;
            return if value >= 100.0 || scale == 1 {
                format!("{value:.0} {unit}")
            } else {
                format!("{value:.1} {unit}")
            };
        }
    }
    "—".to_owned()
}

/// The packs scope: distributed releases, then the shipped corpus if present.
pub(super) fn packs_page(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    catalog_freshness(ui, app, hub);
    hub_table(ui, app, hub);
    let packs = app
        .state
        .model_library_manager
        .spice_packs()
        .map(|index| index.packs().to_vec())
        .unwrap_or_default();
    if !packs.is_empty() {
        pack_catalog(ui, app, &packs);
    }
}

/// How current the cached catalog is, in one phrase.
///
/// "Never fetched" is a real state and says so: a client that has not asked
/// yet knows nothing about what is published, which is different from knowing
/// that nothing is.
pub(super) fn catalog_summary(age_days: Option<u64>) -> String {
    match age_days {
        Some(0) => "catalog: signed today".to_owned(),
        Some(1) => "catalog: 1 d old".to_owned(),
        Some(days) => format!("catalog: {days} d old"),
        None => "catalog: never fetched".to_owned(),
    }
}

/// One line saying how current the catalog is, and how to make it current.
///
/// It is a line rather than a card because it answers one question and is
/// wrong the moment it is acted on. A hub that could not open says so here
/// instead, since "the catalog is 3 days old" would be a claim about nothing.
fn catalog_freshness(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        if let Some(reason) = hub.unavailable.as_deref() {
            ui.label(RichText::new(reason).small().color(t.color.text_dim));
            return;
        }
        let summary = catalog_summary(hub.age_days);
        ui.label(RichText::new(summary).small().color(if hub.stale {
            t.color.warn
        } else {
            t.color.text_dim
        }));
        let idle = !app.state.workbench.models_view.model_import_in_progress;
        if ui
            .add_enabled(idle, compact_button("Refresh catalog"))
            .on_disabled_hover_text("Another model-source operation is still running.")
            .clicked()
        {
            app.queue_model_hub(ModelHubRequest::FetchSnapshot);
        }
        if let Some(fraction) = app.state.workbench.models_view.model_import_progress {
            ui.add(
                egui::ProgressBar::new(fraction)
                    .desired_width(120.0)
                    .show_percentage(),
            );
        }
    });
}

/// The distributed-release table and the detail pane under it.
fn hub_table(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    if hub.unavailable.is_some() {
        page_empty_state(
            ui,
            "Distributed model packs are unavailable",
            "RSpice could not open its pack store, so nothing can be installed on this machine. \
             Projects that already retained a part keep working.",
        );
        return;
    }
    if hub.rows.is_empty() {
        page_empty_state(
            ui,
            "No published model pack has been fetched",
            "Refresh the catalog to list the signed releases this build can install.",
        );
        return;
    }
    let facet = app.state.workbench.models_view.hub_facet;
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        for candidate in ModelHubFacet::ALL {
            let count = hub
                .rows
                .iter()
                .filter(|row| row.state.matches(candidate))
                .count();
            if facet_button(ui, facet == candidate, candidate.label(), Some(count)).clicked() {
                app.state.workbench.models_view.hub_facet = candidate;
            }
        }
    });
    let visible = hub
        .rows
        .iter()
        .filter(|row| {
            row.state.matches(facet)
                && (query.is_empty()
                    || format!("{} {} {} {}", row.pack_id, row.name, row.category, row.spdx)
                        .to_ascii_lowercase()
                        .contains(&query))
        })
        .collect::<Vec<_>>();
    let table_h = (ui.available_height() * 0.34).max(110.0);
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
            table_header(
                ui,
                &[
                    ("PACK", 0.28),
                    ("VERSION", 0.12),
                    ("PARTS", 0.10),
                    ("SIZE", 0.11),
                    ("LICENSE", 0.20),
                    ("STATE", 0.19),
                ],
            );
            ScrollArea::vertical()
                .id_salt("models-hub-pack-table")
                .max_height(table_h)
                .show(ui, |ui| {
                    if visible.is_empty() {
                        empty_state(
                            ui,
                            "No published release matches this facet.",
                            "Facets derive from the signed catalog and what this machine holds.",
                        );
                        if ui.button("Clear release filter").clicked() {
                            app.state.workbench.models_view.hub_facet = ModelHubFacet::All;
                            app.state.workbench.models_view.catalog_query.clear();
                        }
                    }
                    for row in &visible {
                        let key = format!("{}@{}", row.pack_id, row.version);
                        let selected =
                            app.state.workbench.models_view.selected_pack.as_deref() == Some(&key);
                        selectable_data_row(
                            ui,
                            selected,
                            &[
                                (&row.name, 0.28, false),
                                (&row.version, 0.12, true),
                                (&row.parts.to_string(), 0.10, true),
                                (&byte_size(row.archive_length), 0.11, true),
                                (&row.spdx, 0.20, true),
                                (row.state.pill(), 0.19, true),
                            ],
                        )
                        .clicked()
                        .then(|| {
                            app.state.workbench.models_view.selected_pack = Some(key.clone());
                        });
                    }
                });
        });
    catalog_footer(
        ui,
        visible.len(),
        hub.rows.len(),
        hub.rows
            .iter()
            .filter(|row| matches!(row.state, HubPackState::Incompatible { .. }))
            .count(),
        "published releases",
    );
    hub_detail(ui, app, hub);
}

/// What one selected release is, and what can be done with it.
fn hub_detail(ui: &mut Ui, app: &mut ManagerRenderContext<'_>, hub: &HubCatalog) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_pack
        .as_deref()
        .and_then(|key| {
            hub.rows
                .iter()
                .find(|row| format!("{}@{}", row.pack_id, row.version) == key)
        })
        .or_else(|| hub.rows.first());
    let Some(row) = selected.cloned() else {
        return;
    };
    let idle = !app.state.workbench.models_view.model_import_in_progress;
    ui.horizontal_wrapped(|ui| {
        ui.add_space(12.0);
        ui.label(RichText::new(&row.name).strong());
        ui.label(RichText::new(&row.pack_id).monospace().small());
        match &row.state {
            HubPackState::Available => {
                if ui
                    .add_enabled(idle, compact_button("Install"))
                    .on_disabled_hover_text("Another model-source operation is still running.")
                    .clicked()
                {
                    app.state.workbench.models_view.dialog = Some(confirmation(&row, None));
                }
            }
            HubPackState::UpdateAvailable { installed } => {
                if ui
                    .add_enabled(idle, compact_button("Update"))
                    .on_disabled_hover_text("Another model-source operation is still running.")
                    .clicked()
                {
                    app.state.workbench.models_view.dialog =
                        Some(confirmation(&row, Some(installed.clone())));
                }
            }
            HubPackState::Installed => {
                if ui
                    .add_enabled(idle, compact_button("Remove"))
                    .on_disabled_hover_text("Another model-source operation is still running.")
                    .clicked()
                {
                    app.queue_model_hub(ModelHubRequest::RemovePack {
                        pack_id: row.pack_id.clone(),
                        version: row.version.clone(),
                    });
                }
            }
            HubPackState::Incompatible { missing } => {
                ui.add_enabled(false, compact_button("Install"))
                    .on_disabled_hover_text(format!(
                        "This build does not offer {}.",
                        plain_list(missing)
                    ));
            }
        }
    });
    card(ui, |ui| {
        card_title(ui, "RELEASE", Some(&row.category));
        property(ui, "Version", &row.version, "signed catalog");
        property(ui, "License", &row.spdx, "signed manifest");
        property(
            ui,
            "Download",
            &byte_size(row.archive_length),
            "verified end to end before install",
        );
        property(ui, "Parts", &row.parts.to_string(), "addressable models");
        property(
            ui,
            "Requires",
            &if row.capabilities.is_empty() {
                "nothing beyond the core engine".to_owned()
            } else {
                row.capabilities.join(", ")
            },
            match &row.state {
                HubPackState::Incompatible { .. } => "unsatisfied",
                _ => "satisfied by this build",
            },
        );
        if let HubPackState::Incompatible { missing } = &row.state {
            property(ui, "Missing", &plain_list(missing), "engine capability");
        }
        if let HubPackState::UpdateAvailable { installed } = &row.state {
            property(ui, "Installed", installed, "this machine");
        }
    });
}

/// Builds the confirmation this release's action needs.
fn confirmation(row: &HubPackRow, replaces: Option<String>) -> ModelsWorkbenchDialog {
    ModelsWorkbenchDialog::ConfirmPack {
        pack_id: row.pack_id.clone(),
        attach: true,
        release: Some(Box::new(PackReleaseConfirmation {
            name: row.name.clone(),
            version: row.version.clone(),
            spdx: row.spdx.clone(),
            archive_length: row.archive_length,
            parts: row.parts,
            capabilities: row.capabilities.clone(),
            missing: match &row.state {
                HubPackState::Incompatible { missing } => missing.clone(),
                _ => Vec::new(),
            },
            part: None,
            replaces,
        })),
    }
}

/// Renders the release half of the pack confirmation dialog.
///
/// It states everything the action costs and everything it commits to before
/// it happens: which release, under which licence, how many bytes, how many
/// parts, and whether this engine can run it. The primary action is disabled —
/// with the reason — when it cannot.
pub(super) fn release_confirmation(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    pack_id: &str,
    release: &PackReleaseConfirmation,
) -> Option<bool> {
    let mut decision = None;
    ui.label(if release.replaces.is_some() {
        "RSpice will download and prove the newer release, then remove the older copy. A project \
         that already added a part keeps the exact bytes it was built against."
    } else if release.part.is_some() {
        "RSpice will download and prove this release, retain the chosen part's source into the \
         project, and arm it for placement."
    } else {
        "RSpice will download this release and verify it end to end against the compiled-in \
         release key before anything reaches disk."
    });
    ui.label(
        RichText::new(format!("{} {}", release.name, release.version))
            .monospace()
            .strong(),
    );
    card(ui, |ui| {
        card_title(ui, "WHAT WILL BE INSTALLED", Some(pack_id));
        property(ui, "License", &release.spdx, "signed manifest");
        property(
            ui,
            "Download",
            &byte_size(release.archive_length),
            "exact signed length",
        );
        property(
            ui,
            "Parts",
            &release.parts.to_string(),
            "addressable models",
        );
        property(
            ui,
            "Capability check",
            &if release.missing.is_empty() {
                "this build runs every part".to_owned()
            } else {
                format!("missing {}", plain_list(&release.missing))
            },
            if release.missing.is_empty() {
                "passed"
            } else {
                "failed"
            },
        );
        if let Some(part) = release.part.as_deref() {
            property(ui, "Part", part, "retained into the project");
        }
        if let Some(replaces) = release.replaces.as_deref() {
            property(ui, "Replaces", replaces, "removed after success");
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Cancel").clicked() {
            decision = Some(false);
        }
        let installable =
            release.missing.is_empty() && !app.state.workbench.models_view.model_import_in_progress;
        if ui
            .add_enabled(installable, egui::Button::new("Install pack"))
            .on_disabled_hover_text(if release.missing.is_empty() {
                "Another model-source operation is still running.".to_owned()
            } else {
                format!(
                    "This build does not offer {}.",
                    plain_list(&release.missing)
                )
            })
            .clicked()
        {
            decision = Some(true);
        }
    });
    decision
}

/// Turns the request this confirmation describes into hub work.
pub(super) fn release_request(pack_id: &str, release: &PackReleaseConfirmation) -> ModelHubRequest {
    match release.replaces.as_deref() {
        Some(installed) => ModelHubRequest::UpdatePack {
            pack_id: pack_id.to_owned(),
            installed: installed.to_owned(),
            latest: release.version.clone(),
        },
        None => ModelHubRequest::InstallPack {
            pack_id: pack_id.to_owned(),
            version: release.version.clone(),
            part: release.part.clone(),
        },
    }
}

/// "a, b and c" — a list a sentence can contain.
pub(super) fn plain_list(values: &[String]) -> String {
    match values {
        [] => "nothing".to_owned(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// The shipped model-corpus table, when an installation carries one.
///
/// This is the pre-distribution mechanism: a versioned tree on disk, attached
/// into a project wholesale. It stays because an installation that has one
/// must still be able to see and attach it, and it renders only when that tree
/// is actually present — so a build that ships no corpus shows one table, not
/// an empty second one.
fn pack_catalog(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    packs: &[rspice_core::library::SpicePack],
) {
    let facet = app.state.workbench.models_view.pack_facet;
    let query = app
        .state
        .workbench
        .models_view
        .catalog_query
        .trim()
        .to_ascii_lowercase();
    let visible = packs
        .iter()
        .filter(|pack| {
            let attached = !attached_libraries_for_pack(app, &pack.id).is_empty();
            let source_available = app
                .state
                .model_library_manager
                .spice_pack_entry_available(&pack.id);
            let facet_match = match facet {
                ModelPackFacet::All => true,
                ModelPackFacet::NeedsAttention => {
                    pack.entry.is_none() || !pack.redistributable || !source_available
                }
                ModelPackFacet::Attached => attached,
                ModelPackFacet::Foundry => pack.category.eq_ignore_ascii_case("foundry"),
                ModelPackFacet::Vendor => pack.category.eq_ignore_ascii_case("vendor"),
                ModelPackFacet::Community => pack.category.eq_ignore_ascii_case("community"),
                ModelPackFacet::Redistributable => pack.redistributable,
            };
            let haystack = format!("{} {} {} {}", pack.id, pack.name, pack.category, pack.spdx)
                .to_ascii_lowercase();
            facet_match && (query.is_empty() || haystack.contains(&query))
        })
        .cloned()
        .collect::<Vec<_>>();
    let table_h = (ui.available_height() * 0.42).max(120.0);
    egui::Frame::NONE
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .show(ui, |ui| {
        table_header(
            ui,
            &[
                ("PACK", 0.25),
                ("CONTENTS", 0.18),
                ("ORIGIN", 0.12),
                ("PARTS", 0.11),
                ("LICENSE", 0.17),
                ("STATE", 0.17),
            ],
        );
        ScrollArea::vertical()
            .id_salt("models-pack-table")
            .max_height(table_h)
            .show(ui, |ui| {
                if visible.is_empty() {
                    empty_state(
                        ui,
                        "No pack matches this facet.",
                        "Facets derive from the installed corpus manifest and live project attachments.",
                    );
                    if ui.button("Clear filter").clicked() {
                        app.state.workbench.models_view.pack_facet = ModelPackFacet::All;
                        app.state.workbench.models_view.catalog_query.clear();
                    }
                }
                for pack in &visible {
                    let selected = app.state.workbench.models_view.selected_pack.as_deref()
                        == Some(pack.id.as_str());
                    let attached = !attached_libraries_for_pack(app, &pack.id).is_empty();
                    let built_in = is_builtin_pack(app, &pack.id);
                    let source_available = app
                        .state
                        .model_library_manager
                        .spice_pack_entry_available(&pack.id);
                    let state = if built_in {
                        "built in"
                    } else if attached {
                        "attached"
                    } else if pack.entry.is_none() {
                        "no entry"
                    } else if !pack.redistributable {
                        "license review"
                    } else if !source_available {
                        "source required"
                    } else {
                        "available"
                    };
                    selectable_data_row(
                        ui,
                        selected,
                        &[
                            (&pack.name, 0.25, false),
                            (
                                &format!("{} models · {} subckts", pack.models, pack.subcircuits),
                                0.18,
                                false,
                            ),
                            (&pack.category, 0.12, false),
                            (
                                &(pack.models_top + pack.subcircuits_top).to_string(),
                                0.11,
                                true,
                            ),
                            (&pack.spdx, 0.17, true),
                            (state, 0.17, true),
                        ],
                    )
                    .clicked()
                    .then(|| {
                        app.state.workbench.models_view.selected_pack = Some(pack.id.clone())
                    });
                }
            });
        });
    catalog_footer(
        ui,
        visible.len(),
        packs.len(),
        visible
            .iter()
            .filter(|pack| {
                pack.entry.is_none()
                    || !pack.redistributable
                    || !app
                        .state
                        .model_library_manager
                        .spice_pack_entry_available(&pack.id)
            })
            .count(),
        "installed packs",
    );
    pack_detail(ui, app, packs);
}

pub(super) fn pack_facet_count(
    app: &ManagerRenderContext<'_>,
    packs: &[rspice_core::library::SpicePack],
    facet: ModelPackFacet,
) -> usize {
    packs
        .iter()
        .filter(|pack| match facet {
            ModelPackFacet::All => true,
            ModelPackFacet::NeedsAttention => {
                pack.entry.is_none()
                    || !pack.redistributable
                    || !app
                        .state
                        .model_library_manager
                        .spice_pack_entry_available(&pack.id)
            }
            ModelPackFacet::Attached => !attached_libraries_for_pack(app, &pack.id).is_empty(),
            ModelPackFacet::Foundry => pack.category.eq_ignore_ascii_case("foundry"),
            ModelPackFacet::Vendor => pack.category.eq_ignore_ascii_case("vendor"),
            ModelPackFacet::Community => pack.category.eq_ignore_ascii_case("community"),
            ModelPackFacet::Redistributable => pack.redistributable,
        })
        .count()
}

fn pack_detail(
    ui: &mut Ui,
    app: &mut ManagerRenderContext<'_>,
    packs: &[rspice_core::library::SpicePack],
) {
    let selected = app
        .state
        .workbench
        .models_view
        .selected_pack
        .as_deref()
        .and_then(|id| packs.iter().find(|pack| pack.id == id))
        .cloned()
        .or_else(|| packs.first().cloned());
    let Some(pack) = selected else {
        empty_state(
            ui,
            "No shipped model corpus is installed.",
            "Set RSPICE_MODELS_DIR or install the versioned model-pack tree, then rescan.",
        );
        return;
    };
    app.state.workbench.models_view.selected_pack = Some(pack.id.clone());
    let attached = attached_libraries_for_pack(app, &pack.id);
    let built_in = is_builtin_pack(app, &pack.id);
    let catalog_source_available = app
        .state
        .model_library_manager
        .spice_pack_entry_available(&pack.id);
    let model_source_job_idle = !app.state.workbench.models_view.model_import_in_progress;
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(&pack.name).strong());
        ui.label(RichText::new(&pack.id).monospace().small());
        if ui.button("Browse parts").clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::RSpiceLibrary;
            app.state.workbench.models_view.catalog_query.clear();
            app.state.workbench.models_view.selected_pack = Some(pack.id.clone());
            app.state.workbench.models_view.part_catalog_offset = 0;
            app.state.workbench.models_view.selected_part = None;
        }
        if built_in {
            ui.label(RichText::new("Built in").small());
        } else if let Some(library) = attached.first() {
            if ui
                .add_enabled(
                    catalog_source_available && model_source_job_idle,
                    egui::Button::new("Refresh snapshot"),
                )
                .on_disabled_hover_text(
                    if !model_source_job_idle {
                        "Another model-source operation is still running."
                    } else {
                        "Refreshing requires the installed corpus source; the retained project snapshot remains executable."
                    },
                )
                .clicked()
                && let Some(library) = app
                    .state
                    .model_library_manager
                    .get_library(library)
                    .cloned()
            {
                refresh_library(app, &library);
            }
            if ui.button("Detach…").clicked() {
                app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
                    pack_id: pack.id.clone(),
                    release: None,
                    attach: false,
                });
            }
        } else if ui
            .add_enabled(
                pack.entry.is_some()
                    && pack.redistributable
                    && catalog_source_available
                    && model_source_job_idle,
                egui::Button::new("Attach…"),
            )
            .on_disabled_hover_text(if !pack.redistributable {
                "This pack has no established redistribution grant."
            } else if pack.entry.is_none() {
                "The pack manifest has no attachable entry file."
            } else if !catalog_source_available {
                "The shipped source is not installed. Reinstall RSpice or rescan the model library."
            } else {
                "This pack cannot be attached."
            })
            .clicked()
        {
            app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
                pack_id: pack.id.clone(),
                attach: true,
                release: None,
            });
        }
    });
    card(ui, |ui| {
        card_title(ui, "PACK CONTRACT", Some(&pack.category));
        property(
            ui,
            "Contents",
            &format!(
                "{} addressable · {} total definitions · {} files",
                pack.models_top + pack.subcircuits_top,
                pack.models + pack.subcircuits,
                pack.files
            ),
            "manifest",
        );
        property(ui, "License", &pack.spdx, pack.tier.display_name());
        property(
            ui,
            "Redistributable",
            if pack.redistributable { "yes" } else { "no" },
            "enforced before project embedding",
        );
        property(
            ui,
            "Attachment",
            &if built_in {
                "built into RSpice".to_owned()
            } else if attached.is_empty() {
                "not attached".to_owned()
            } else {
                attached.join(", ")
            },
            if built_in {
                "embedded foundation"
            } else if !attached.is_empty() {
                "authenticated project source"
            } else {
                "corpus only"
            },
        );
        property(
            ui,
            "Executable source",
            if catalog_source_available || !attached.is_empty() {
                "available"
            } else {
                "import required"
            },
            "attach gate",
        );
        property(
            ui,
            "Entry",
            pack.entry
                .as_deref()
                .map(path_label)
                .as_deref()
                .unwrap_or("not declared"),
            "pack manifest",
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::workbench::app_state::AppState;

    /// Renders one models-workspace fragment and returns its access tree.
    fn accessibility_nodes(
        state: &mut AppState,
        size: egui::Vec2,
        render: impl FnOnce(&mut Ui, &mut ManagerRenderContext<'_>),
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut pending = Vec::new();
        // `run_ui` takes an `FnMut`, so the one-shot renderer is parked where
        // the closure can take it rather than consuming a captured value.
        let mut render = Some(render);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..egui::RawInput::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut context = ManagerRenderContext {
                        state,
                        pending_actions: &mut pending,
                    };
                    if let Some(render) = render.take() {
                        render(ui, &mut context);
                    }
                });
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("the models workspace publishes an access tree")
            .nodes
    }

    fn button(
        nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
        label: &str,
    ) -> Option<egui::accesskit::Node> {
        nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .map(|(_, node)| node.clone())
    }

    fn labelled(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], label: &str) -> bool {
        nodes
            .iter()
            .any(|(_, node)| node.label().is_some_and(|found| found == label))
    }

    fn release(missing: &[&str]) -> PackReleaseConfirmation {
        PackReleaseConfirmation {
            name: "RSpice proving pack".to_owned(),
            version: "1.0.0".to_owned(),
            spdx: "LicenseRef-RSpice-Models".to_owned(),
            archive_length: 2 * 1024 * 1024,
            parts: 12,
            capabilities: vec!["subckt".to_owned(), "resistor".to_owned()],
            missing: missing.iter().map(|value| (*value).to_owned()).collect(),
            part: None,
            replaces: None,
        }
    }

    fn row(name: &str, version: &str, state: HubPackState) -> HubPackRow {
        HubPackRow {
            pack_id: format!("rspice-{}", name.to_ascii_lowercase()),
            name: name.to_owned(),
            category: "proving".to_owned(),
            version: version.to_owned(),
            state,
            spdx: "LicenseRef-RSpice-Models".to_owned(),
            archive_length: 1024 * 1024,
            parts: 4,
            capabilities: vec!["subckt".to_owned()],
        }
    }

    #[test]
    fn the_release_confirmation_states_its_cost_and_exposes_both_actions() {
        let mut state = AppState::default();
        let confirmation = release(&[]);
        let nodes = accessibility_nodes(&mut state, egui::vec2(640.0, 520.0), |ui, app| {
            release_confirmation(ui, app, "rspice-proving", &confirmation);
        });
        assert!(button(&nodes, "Cancel").is_some(), "cancel is reachable");
        let install = button(&nodes, "Install pack").expect("the primary action is reachable");
        assert!(
            !install.is_disabled(),
            "a compatible release is installable"
        );
        // The cost the dialog paints comes from the confirmation it captured,
        // which is what the request it dispatches is built from too. Asserting
        // the projection rather than the painted glyphs is deliberate: this
        // workspace's property rows are painter text and expose no
        // accessibility node, so a tree assertion here would prove nothing.
        assert_eq!(byte_size(confirmation.archive_length), "2.0 MB");
        assert_eq!(confirmation.spdx, "LicenseRef-RSpice-Models");
        assert_eq!(confirmation.parts, 12);
        assert_eq!(
            release_request("rspice-proving", &confirmation),
            ModelHubRequest::InstallPack {
                pack_id: "rspice-proving".to_owned(),
                version: "1.0.0".to_owned(),
                part: None,
            }
        );
    }

    #[test]
    fn a_confirmed_update_replaces_the_release_it_names() {
        let mut confirmation = release(&[]);
        confirmation.version = "1.1.0".to_owned();
        confirmation.replaces = Some("1.0.0".to_owned());
        assert_eq!(
            release_request("rspice-proving", &confirmation),
            ModelHubRequest::UpdatePack {
                pack_id: "rspice-proving".to_owned(),
                installed: "1.0.0".to_owned(),
                latest: "1.1.0".to_owned(),
            }
        );
    }

    #[test]
    fn an_incompatible_release_is_described_and_refused_rather_than_hidden() {
        let mut state = AppState::default();
        let confirmation = release(&["nonexistent-capability"]);
        let nodes = accessibility_nodes(&mut state, egui::vec2(640.0, 520.0), |ui, app| {
            release_confirmation(ui, app, "rspice-proving", &confirmation);
        });
        let install = button(&nodes, "Install pack").expect("the action is still present");
        assert!(
            install.is_disabled(),
            "an incompatible release cannot be installed"
        );
        assert_eq!(
            plain_list(&confirmation.missing),
            "nonexistent-capability",
            "the reason offered is the plain capability name"
        );
    }

    #[test]
    fn all_four_release_states_are_reachable_and_offer_the_right_action() {
        let states = [
            (
                HubPackState::Installed,
                "Remove",
                "Installed release",
                "1.0.0",
            ),
            (
                HubPackState::Available,
                "Install",
                "Available release",
                "1.0.0",
            ),
            (
                HubPackState::UpdateAvailable {
                    installed: "1.0.0".to_owned(),
                },
                "Update",
                "Updatable release",
                "1.1.0",
            ),
            (
                HubPackState::Incompatible {
                    missing: vec!["nonexistent-capability".to_owned()],
                },
                "Install",
                "Incompatible release",
                "1.0.0",
            ),
        ];
        let catalog = HubCatalog {
            rows: states
                .iter()
                .map(|(state, _, name, version)| row(name, version, state.clone()))
                .collect(),
            age_days: Some(2),
            unavailable: None,
            stale: false,
        };

        for (state, action, name, version) in &states {
            let mut app_state = AppState::default();
            app_state.workbench.models_view.selected_pack =
                Some(format!("rspice-{}@{version}", name.to_ascii_lowercase()));
            let hub = catalog.clone();
            let nodes =
                accessibility_nodes(&mut app_state, egui::vec2(1100.0, 760.0), move |ui, app| {
                    packs_page(ui, app, &hub);
                });
            assert!(
                labelled(&nodes, name),
                "the {} row is reachable",
                state.pill()
            );
            let action_node = button(&nodes, action)
                .unwrap_or_else(|| panic!("the {} state offers {action}", state.pill()));
            assert_eq!(
                action_node.is_disabled(),
                matches!(state, HubPackState::Incompatible { .. }),
                "only an incompatible release refuses its action"
            );
        }
    }

    #[test]
    fn the_freshness_line_says_how_old_the_catalog_is_and_offers_one_refresh() {
        let mut state = AppState::default();
        let catalog = HubCatalog {
            rows: vec![row("Proving", "1.0.0", HubPackState::Available)],
            age_days: Some(9),
            unavailable: None,
            stale: true,
        };
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &catalog);
        });
        assert_eq!(catalog_summary(Some(9)), "catalog: 9 d old");
        assert_eq!(catalog_summary(Some(1)), "catalog: 1 d old");
        assert_eq!(catalog_summary(Some(0)), "catalog: signed today");
        assert_eq!(catalog_summary(None), "catalog: never fetched");
        assert!(button(&nodes, "Refresh catalog").is_some());

        // A hub that could not open says that instead of a fresh-looking age.
        let mut state = AppState::default();
        let unavailable = HubCatalog {
            unavailable: Some("RSpice could not open its model-pack storage.".to_owned()),
            ..HubCatalog::default()
        };
        let nodes = accessibility_nodes(&mut state, egui::vec2(1100.0, 760.0), move |ui, app| {
            packs_page(ui, app, &unavailable);
        });
        assert!(
            button(&nodes, "Refresh catalog").is_none(),
            "there is nothing to refresh"
        );
        assert!(
            button(&nodes, "Install").is_none(),
            "and nothing to install"
        );
    }

    #[test]
    fn a_byte_count_reads_as_a_size_rather_than_a_number() {
        assert_eq!(byte_size(0), "—");
        assert_eq!(byte_size(512), "512 B");
        assert_eq!(byte_size(2048), "2.0 kB");
        assert_eq!(byte_size(3 * 1024 * 1024), "3.0 MB");
        assert_eq!(byte_size(150 * 1024 * 1024), "150 MB");
    }
}

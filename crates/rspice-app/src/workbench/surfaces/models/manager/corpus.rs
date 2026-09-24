//! The shipped model corpus, which predates signed distribution.
//!
//! A versioned tree on disk, attached into a project wholesale — a different
//! mechanism from the Model Hub's signed releases, with different evidence
//! behind every claim it makes. It stays because an installation that carries
//! such a tree must still be able to see and attach it, and it renders only
//! when the tree is actually present, so a build that ships no corpus shows one
//! table rather than an empty second one.
//!
//! Merging it into the ledger beside it would mean a row that cannot honestly
//! say where it came from: "installed" means proved under the release key on
//! one side of that merge and "present in a directory" on the other.

use super::*;

/// The shipped model-corpus table, when an installation carries one.
pub(super) fn pack_catalog(
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
    // This table's own chips, beside this table. The bar above carries the
    // ledger's rail, and a second rail up there would leave a reader guessing
    // which of the two tables below it was narrowing.
    ui.horizontal(|ui| {
        ui.add_space(12.0);
        ui.spacing_mut().item_spacing.x = 4.0;
        for candidate in ModelPackFacet::ALL {
            let count = pack_facet_count(app, packs, candidate);
            if facet_button(ui, facet == candidate, candidate.label(), Some(count)).clicked() {
                app.state.workbench.models_view.pack_facet = candidate;
            }
        }
    });
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
                    if Button::new("Clear filter").show(ui).clicked() {
                        app.state.workbench.models_view.pack_facet = ModelPackFacet::All;
                        app.state.workbench.models_view.catalog_query.clear();
                    }
                }
                for pack in &visible {
                    let selected = app.state.workbench.models_view.selected_corpus_pack.as_deref()
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
                        app.state.workbench.models_view.selected_corpus_pack = Some(pack.id.clone())
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

fn pack_facet_count(
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
        .selected_corpus_pack
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
    app.state.workbench.models_view.selected_corpus_pack = Some(pack.id.clone());
    let attached = attached_libraries_for_pack(app, &pack.id);
    let built_in = is_builtin_pack(app, &pack.id);
    let catalog_source_available = app
        .state
        .model_library_manager
        .spice_pack_entry_available(&pack.id);
    let model_source_job_idle = !app.state.workbench.models_view.model_import_in_progress;
    ui.horizontal_wrapped(|ui| {
        // The workspace zeroes item spacing to paint itself as one document;
        // a line of separate phrases asks for its own gaps back.
        ui.spacing_mut().item_spacing.x = 10.0;
        ui.add_space(12.0);
        ui.label(RichText::new(&pack.name).strong());
        ui.label(RichText::new(&pack.id).monospace().small());
        if Button::new("Browse parts").show(ui).clicked() {
            app.state.workbench.models_view.catalog_scope = ModelsCatalogScope::RSpiceLibrary;
            app.state.workbench.models_view.catalog_query.clear();
            app.state.workbench.models_view.selected_pack = Some(pack.id.clone());
            app.state.workbench.models_view.part_catalog_offset = 0;
            app.state.workbench.models_view.selected_part = None;
        }
        if built_in {
            ui.label(RichText::new("Built in").small());
        } else if let Some(library) = attached.first() {
            if Button::new("Refresh snapshot")
                .enabled(catalog_source_available && model_source_job_idle)
                .show(ui)
                .on_disabled_hover_text(
                    if !model_source_job_idle {
                        "Another model-source operation is still running."
                    } else {
                        "Refreshing requires the installed corpus source; the retained project snapshot remains executable."
                    },
                )
                .clicked()
                && app
                    .state
                    .model_library_manager
                    .get_library(library)
                    .is_some()
            {
                refresh_library(app, library);
            }
            if Button::new("Detach…").show(ui).clicked() {
                app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
                    pack_id: pack.id.clone(),
                    release: None,
                    attach: false,
                });
            }
        } else if Button::new("Attach…")
            .enabled(
                pack.entry.is_some()
                    && pack.redistributable
                    && catalog_source_available
                    && model_source_job_idle,
            )
            .show(ui)
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
    // The last block on the packs page whenever an installation carries a
    // shipped corpus tree, so it is the one that reaches the panel's bottom
    // edge. A shrink-to-fit card here left the surface ending in the
    // container's hairline colour, which reads as a pane that failed to render
    // rather than as room.
    let region_h = ui.available_height().max(1.0);
    filled_detail_pane(
        ui,
        "PACK CONTRACT",
        Some(&pack.category),
        region_h,
        "models-corpus-contract",
        |ui| {
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
        },
    );
}

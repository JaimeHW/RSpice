//! Offscreen renders of the Model Hub, so its design can be looked at rather
//! than only asserted about.
//!
//! The contract tests next door read the accessibility tree; nothing in them
//! sees where a cell lands, whether the Attention column has room for its
//! phrase, or whether the inspector's two panes actually split the width. This
//! composes the ledger, the shelf and the held-catalog card into a full-width
//! panel and hands each to [`crate::ui::raster`], which runs it headless and
//! rasterizes in software — no GPU, no window, no wasm build.
//!
//! Read the renders for layout, not for wording; the rasterizer's own header
//! says why.
//!
//! Run with `--ignored`; the renders go to `RSPICE_RASTER_DIR` (default: the
//! system temp directory).

use super::*;

use crate::state::model_hub::ArchiveEvidence;
use crate::ui::raster::Canvas;

/// Width of the workspace column in the shell at a 1680 px window.
const PAGE_WIDTH: f32 = 1180.0;
/// Tall enough that the ledger, its footer, the inspector and the shipped
/// corpus table below them all fit; the render is cropped to its content.
const PAGE_HEIGHT: f32 = 1600.0;

/// Render one catalog scope over a hand-built projection.
///
/// The projection is supplied rather than read from a store because a signed
/// hub is not something a render should have to stand up: what is being looked
/// at here is the composition, and the composition is a function of exactly
/// this value.
fn raster(
    scope: ModelsCatalogScope,
    hub: hub::HubCatalog,
    seed: impl FnOnce(&mut AppState),
) -> Canvas {
    let mut state = AppState::default();
    state.workbench.models_view.catalog_scope = scope;
    seed(&mut state);
    let mut pending = Vec::new();
    crate::ui::raster::render(egui::vec2(PAGE_WIDTH, PAGE_HEIGHT), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::ZERO;
                let mut context = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                page_tabs(ui, &mut context);
                // No hub store stands behind a render, so no release is
                // installed in one. Every shelf state a render is for — the
                // class facets, the part detail, the disabled Place — is a
                // function of the project and the shipped index instead.
                catalog_page(ui, &mut context, &hub, &[]);
            });
    })
}

/// Render one dialog on its own, with no page behind it.
///
/// The rasterizer fills triangles in the order the tessellator emits them and
/// does not re-sort by layer, so a modal drawn over a page comes out with the
/// page's text lying across it — an artifact of this harness rather than of the
/// dialog, and one that makes the card impossible to read. A modal is a
/// self-contained composition anyway, so it is rendered as one.
fn raster_dialog(hub: hub::HubCatalog, dialog: ModelsWorkbenchDialog) -> Canvas {
    let mut state = AppState::default();
    state.workbench.models_view.dialog = Some(dialog);
    let mut pending = Vec::new();
    crate::ui::raster::render(egui::vec2(760.0, 900.0), |ui, background| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(background))
            .show(ui, |ui| {
                let mut context = ManagerRenderContext {
                    state: &mut state,
                    pending_actions: &mut pending,
                };
                dialogs::render_dialog(ui, &mut context, &hub);
            });
    })
}

/// One published release of one pack.
fn release(pack_id: &str, name: &str, version: &str, state: hub::HubPackState) -> hub::HubPackRow {
    hub::HubPackRow {
        pack_id: pack_id.to_owned(),
        name: name.to_owned(),
        category: "discrete".to_owned(),
        version: version.to_owned(),
        state,
        spdx: "LicenseRef-RSpice-Models".to_owned(),
        archive_length: 1_900_000,
        parts: 412,
        capabilities: vec!["diode".to_owned()],
        archive: None,
    }
}

/// A catalog holding a handful of packs in the states a real machine reaches.
fn catalog(attention: bool) -> hub::HubCatalog {
    let held = |version: &str, evidence| hub::InstalledRelease {
        version: version.to_owned(),
        archive: Some(evidence),
        archive_sha256: "9f2c".repeat(16),
    };
    let pin = |version: &str, archive: &str| crate::state::model_library::PackPartPin {
        pack_id: "rspice-opamps".to_owned(),
        pack_version: version.to_owned(),
        archive_sha256: archive.to_owned(),
        part_id: "OPA2340".to_owned(),
    };
    let mut packs = vec![
        hub::ledger_row(
            "rspice-discrete-diodes",
            vec![release(
                "rspice-discrete-diodes",
                "Discrete diodes, rectifiers, Schottky and Zener",
                "1.1.0",
                hub::HubPackState::Installed,
            )],
            Some(held("1.1.0", ArchiveEvidence::MatchesCatalog)),
            &[],
            None,
        ),
        hub::ledger_row(
            "rspice-opamps",
            vec![release(
                "rspice-opamps",
                "Operational amplifier macromodels",
                "2.0.0",
                hub::HubPackState::Installed,
            )],
            Some(held("2.0.0", ArchiveEvidence::MatchesCatalog)),
            &[
                pin("2.0.0", &"9f2c".repeat(16)),
                pin("2.0.0", &"9f2c".repeat(16)),
            ],
            None,
        ),
        hub::ledger_row(
            "rspice-comparators",
            vec![release(
                "rspice-comparators",
                "Comparator macromodels",
                "1.0.0",
                hub::HubPackState::Available,
            )],
            None,
            &[],
            None,
        ),
    ];
    if attention {
        packs.push(hub::ledger_row(
            "rspice-regulators",
            vec![release(
                "rspice-regulators",
                "Linear regulators and voltage references",
                "3.1.0",
                hub::HubPackState::Installed,
            )],
            Some(held("3.1.0", ArchiveEvidence::DiffersFromCatalog)),
            &[],
            None,
        ));
    }
    hub::HubCatalog {
        packs,
        age_days: Some(2),
        signed: Some("2026-08-18".to_owned()),
        unavailable: None,
        stale: false,
        expired: None,
        cache_discarded: false,
        identity: Some(identity()),
        signing_key: "7ce1fddbb60d7a3ba6a09d5bf669087cd59104fe9fbaad72bbf42e41762f957a".to_owned(),
        licences: vec!["LicenseRef-RSpice-Models".to_owned()],
        host: browser::Host::default(),
        storage: crate::state::model_hub::durable::PackStorageStanding::NotApplicable,
    }
}

/// The held catalog's identity, as a refresh would have settled it.
fn identity() -> crate::state::model_hub::CatalogIdentity {
    crate::state::model_hub::CatalogIdentity {
        generation: Some(41),
        digest: "c1d9".repeat(16),
        schema: 2,
        serial: 41,
        generated_at: "2026-08-18T04:12:09Z".to_owned(),
        expires_at: "2026-09-17T04:12:09Z".to_owned(),
        expires_at_seconds: crate::state::model_hub::rfc3339_seconds("2026-09-17T04:12:09Z"),
    }
}

/// A catalog past the horizon its publisher signed.
///
/// The ledger keeps the installed packs — that is the whole claim of D-D — and
/// loses the one nothing here holds, because that row was an offer and offers
/// are what an expiry withdraws.
fn expired_catalog() -> hub::HubCatalog {
    let mut catalog = catalog(false);
    catalog.packs.retain(|row| row.installed.is_some());
    catalog.stale = true;
    catalog.expired = Some("2026-08-19T04:12:09Z".to_owned());
    catalog.identity = Some(crate::state::model_hub::CatalogIdentity {
        expires_at: "2026-08-19T04:12:09Z".to_owned(),
        expires_at_seconds: crate::state::model_hub::rfc3339_seconds("2026-08-19T04:12:09Z"),
        ..identity()
    });
    catalog
}

/// A catalog recalling the release this machine is running.
///
/// The recalled release stays on the ledger — it is installed, and a recall
/// removes nothing from a machine — and the project's two pins stay pinned to
/// it, which is exactly the row the `revoked` rung has to be legible on.
fn recalled_catalog() -> hub::HubCatalog {
    let mut catalog = catalog(false);
    let pin = |version: &str| crate::state::model_library::PackPartPin {
        pack_id: "rspice-opamps".to_owned(),
        pack_version: version.to_owned(),
        archive_sha256: "9f2c".repeat(16),
        part_id: "OPA2340".to_owned(),
    };
    catalog.packs = catalog
        .packs
        .into_iter()
        .map(|row| {
            if row.pack_id != "rspice-opamps" {
                return row;
            }
            hub::ledger_row(
                &row.pack_id,
                row.releases.clone(),
                row.installed.clone(),
                &[pin("2.0.0"), pin("2.0.0")],
                Some(hub::Recalled {
                    version: "2.0.0".to_owned(),
                    reason: "the output stage mismodels saturation above 85 C.".to_owned(),
                }),
            )
        })
        .collect();
    catalog
}

/// A library of pinned bytes the project retained, for the shelf renders.
///
/// Two definitions rather than one, so a render shows the "in project" word
/// beside rows that do not carry it — an exception-only state is only legible
/// next to the ordinary case.
fn retained_fixture() -> crate::state::model_library::ModelLibrary {
    use crate::state::model_library::{DeviceModel, ModelSubcircuitInterface, ModelType};

    let mut library = crate::state::model_library::ModelLibrary::new("proving_parts");
    library.pdk_name = "RSpice proving parts".to_owned();
    library.source_authority = crate::state::model_library::ModelSourceAuthority::External;
    library.root_path = Some(PathBuf::from("/retained/proving.lib"));
    library.subcircuits.insert(
        "PROVING_DIV".to_owned(),
        ModelSubcircuitInterface {
            name: "PROVING_DIV".to_owned(),
            ports: vec!["IN".to_owned(), "OUT".to_owned()],
            parameter_defaults: BTreeMap::new(),
            description: None,
            file_path: None,
            source_line: Some(2),
            section: None,
        },
    );
    let mut zener = DeviceModel::new("RSPICE_ZENER", ModelType::Diode);
    zener.spice_type = Some("D".to_owned());
    library.add_model(zener);
    library
}

/// One pack with an update on offer, three parts pinned to what is held, and
/// the projection the inspector would have computed for that pair of releases.
///
/// The three pins are deliberately in the three states adoption distinguishes:
/// one the newer release lists differently, one it does not publish at all, and
/// one it re-lists unchanged. A render that only ever showed the easy case
/// would say nothing about whether the refusal has room for its reason.
fn offered() -> (hub::HubCatalog, crate::state::model_hub::ReleaseDiff) {
    use crate::state::model_hub::{ChangedPart, PartFact, ReleaseDiff, ReleaseDiffKey};

    let pin = |part: &str| crate::state::model_library::PackPartPin {
        pack_id: "rspice-opamps".to_owned(),
        pack_version: "2.0.0".to_owned(),
        archive_sha256: "9f2c".repeat(16),
        part_id: part.to_owned(),
    };
    let mut catalog = catalog(false);
    catalog.packs = vec![hub::ledger_row(
        "rspice-opamps",
        vec![
            release(
                "rspice-opamps",
                "Operational amplifier macromodels",
                "2.1.0",
                hub::HubPackState::UpdateAvailable {
                    installed: "2.0.0".to_owned(),
                },
            ),
            release(
                "rspice-opamps",
                "Operational amplifier macromodels",
                "2.0.0",
                hub::HubPackState::Installed,
            ),
        ],
        Some(hub::InstalledRelease {
            version: "2.0.0".to_owned(),
            archive: Some(ArchiveEvidence::MatchesCatalog),
            archive_sha256: "9f2c".repeat(16),
        }),
        &[pin("OPA2340"), pin("OPA2333"), pin("OPA2277")],
        None,
    )];
    let diff = ReleaseDiff {
        key: ReleaseDiffKey {
            catalog_digest: "c1d9".repeat(16),
            pack_id: "rspice-opamps".to_owned(),
            from: "2.0.0".to_owned(),
            to: "2.1.0".to_owned(),
        },
        added: vec!["OPA2350".to_owned()],
        removed: vec!["OPA2333".to_owned()],
        changed: vec![
            ChangedPart {
                part_id: "OPA2340".to_owned(),
                facts: vec![PartFact::Terminals {
                    from: ["INP", "INN", "OUT"].map(str::to_owned).to_vec(),
                    to: ["INP", "INN", "VCC", "OUT"].map(str::to_owned).to_vec(),
                }],
            },
            // The two facts schema 2 added, on a second part, so the render
            // shows how a description and a specification read beside a
            // terminal change rather than only on their own.
            ChangedPart {
                part_id: "OPA2277".to_owned(),
                facts: vec![
                    PartFact::Description {
                        from: Some("Precision op-amp".to_owned()),
                        to: Some("Precision low-drift op-amp".to_owned()),
                    },
                    PartFact::Spec {
                        key: "GBW".to_owned(),
                        from: Some("1 MHz".to_owned()),
                        to: Some("1.2 MHz".to_owned()),
                    },
                    PartFact::Spec {
                        key: "Iq".to_owned(),
                        from: None,
                        to: Some("800 uA".to_owned()),
                    },
                ],
            },
        ],
        relisted: 409,
        capabilities_added: vec!["veriloga".to_owned()],
        capabilities_removed: Vec::new(),
        licence: None,
        archive_differs: true,
    };
    (catalog, diff)
}

/// Write every Model Hub state to a PNG so its design can be reviewed.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_model_hub_state() {
    use crate::workbench::state::{ModelsAttemptedOperation, ModelsOperationalState};
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report_output = stderr.lock();

    let renders: Vec<(&str, Canvas)> = vec![
        (
            "ledger-healthy",
            raster(
                ModelsCatalogScope::InstalledPacks,
                catalog(false),
                |state| {
                    state.workbench.models_view.selected_pack =
                        Some("rspice-discrete-diodes".to_owned());
                },
            ),
        ),
        (
            "ledger-attention",
            raster(ModelsCatalogScope::InstalledPacks, catalog(true), |state| {
                state.workbench.models_view.selected_pack = Some("rspice-regulators".to_owned());
            }),
        ),
        (
            "ledger-exception-banner",
            raster(ModelsCatalogScope::InstalledPacks, catalog(true), |state| {
                state.workbench.models_view.operational_state = ModelsOperationalState::Offline;
                state.workbench.models_view.action_receipt =
                    Some(Err("the model hub could not be reached".to_owned()));
                state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
                    label: "model-catalog refresh".to_owned(),
                    reissuable: true,
                    landing_pack: None,
                });
            }),
        ),
        // The three trust refusals, each on the page a reader meets it on.
        //
        // A rollback is a *refused refresh*: the held catalog is untouched, so
        // the ledger behind the banner is the healthy one and the whole story
        // is in the banner. That is the composition being checked — that the
        // two read as one statement rather than as a warning over a page that
        // contradicts it.
        (
            "ledger-catalog-rollback-refused",
            raster(
                ModelsCatalogScope::InstalledPacks,
                catalog(false),
                |state| {
                    state.workbench.models_view.selected_pack =
                        Some("rspice-discrete-diodes".to_owned());
                    state.workbench.models_view.operational_state = ModelsOperationalState::Stale;
                    state.workbench.models_view.action_receipt = Some(Err(
                        "the model hub offered catalog serial 40, which is stale beside serial 41 \
                     this machine has already accepted; the held catalog was kept"
                            .to_owned(),
                    ));
                    state.workbench.models_view.attempted_operation =
                        Some(ModelsAttemptedOperation {
                            label: "model-catalog refresh".to_owned(),
                            reissuable: true,
                            landing_pack: None,
                        });
                },
            ),
        ),
        // An expired catalog keeps every installed pack on the ledger and
        // withdraws the offers. The status line carries the instant, so the
        // render is where to check that the line still fits beside the two
        // controls it shares a row with.
        (
            "ledger-catalog-expired",
            raster(
                ModelsCatalogScope::InstalledPacks,
                expired_catalog(),
                |state| {
                    state.workbench.models_view.selected_pack = Some("rspice-opamps".to_owned());
                },
            ),
        ),
        // A recalled release that is installed and pinned. The Attention cell
        // is the narrowest column on the ledger, so "revoked" sharing it with
        // the publisher's reason in the hover is exactly the pairing to look at.
        (
            "ledger-release-revoked",
            raster(
                ModelsCatalogScope::InstalledPacks,
                recalled_catalog(),
                |state| {
                    state.workbench.models_view.selected_pack = Some("rspice-opamps".to_owned());
                },
            ),
        ),
        // And the refusal a recall produces when somebody acts on it anyway.
        (
            "ledger-revoked-update-refused",
            raster(
                ModelsCatalogScope::InstalledPacks,
                recalled_catalog(),
                |state| {
                    state.workbench.models_view.selected_pack = Some("rspice-opamps".to_owned());
                    state.workbench.models_view.operational_state =
                        ModelsOperationalState::Recalled;
                    state.workbench.models_view.action_receipt = Some(Err(
                        "rspice-opamps 2.1.0 was recalled by its publisher: the 2.0.0 output \
                         stage mismodels saturation above 85 C."
                            .to_owned(),
                    ));
                    state.workbench.models_view.attempted_operation =
                        Some(ModelsAttemptedOperation {
                            label: "model-pack update of 'rspice-opamps' to 2.1.0".to_owned(),
                            reissuable: false,
                            landing_pack: Some("rspice-opamps".to_owned()),
                        });
                },
            ),
        ),
        (
            "held-catalog-card-expired",
            raster_dialog(expired_catalog(), ModelsWorkbenchDialog::HeldCatalog),
        ),
        (
            "ledger-facet-needs-attention",
            raster(ModelsCatalogScope::InstalledPacks, catalog(true), |state| {
                state.workbench.models_view.hub_facet = ModelHubFacet::NeedsAttention;
            }),
        ),
        ("ledger-release-diff", {
            let (catalog, diff) = offered();
            raster(ModelsCatalogScope::InstalledPacks, catalog, |state| {
                state.workbench.models_view.selected_pack = Some("rspice-opamps".to_owned());
                state.workbench.models_view.release_diff = Some(diff);
            })
        }),
        (
            "shelf-class-facet",
            raster(ModelsCatalogScope::RSpiceLibrary, catalog(false), |state| {
                state.workbench.models_view.part_facet = RSpicePartFacet::Diode;
            }),
        ),
        // The part detail with placement refused, so the disabled control and
        // the row it belongs to can be read together.
        (
            "shelf-detail-place-refused",
            raster(ModelsCatalogScope::RSpiceLibrary, catalog(false), |state| {
                state.workbench.safe_mode.activate(
                    crate::workbench::state::LocalSafeModeOptions {
                        open_project_read_only: true,
                        ..Default::default()
                    },
                    "render".to_owned(),
                );
            }),
        ),
        // A shelf carrying both halves: the shipped index's rows and a library
        // the project retained, with the "in project" word on the ones it holds.
        (
            "shelf-retained-in-project",
            raster(ModelsCatalogScope::RSpiceLibrary, catalog(false), |state| {
                state.model_library_manager.add_library(retained_fixture());
            }),
        ),
        // The browser projection, composed on a desktop build. The host is a
        // value the projection carries rather than something read from the
        // platform at paint time, so the browser host can be handed to the
        // same composition and looked at here — which is the only place the
        // wasm status line can be seen at all without a browser.
        (
            "ledger-browser-session",
            raster(
                ModelsCatalogScope::InstalledPacks,
                hub::HubCatalog {
                    host: browser::Host::Browser,
                    ..catalog(false)
                },
                |state| {
                    state.workbench.models_view.selected_pack =
                        Some("rspice-discrete-diodes".to_owned());
                },
            ),
        ),
        // A transfer in flight, on the row it is landing on. The percentage
        // shares the Attention column with the exceptions, so the two have to
        // be looked at in the same width: a phrase that fits at 40% and
        // truncates at 100% would be a meter that lies at the end.
        (
            "ledger-installing",
            raster(ModelsCatalogScope::InstalledPacks, catalog(true), |state| {
                state.workbench.models_view.selected_pack = Some("rspice-comparators".to_owned());
                state.workbench.models_view.model_import_in_progress = true;
                state.workbench.models_view.model_import_progress = Some(0.62);
                state.workbench.models_view.attempted_operation = Some(ModelsAttemptedOperation {
                    label: "model-pack install of 'rspice-comparators 1.0.0'".to_owned(),
                    reissuable: false,
                    landing_pack: Some("rspice-comparators".to_owned()),
                });
            }),
        ),
        (
            "held-catalog-card",
            raster_dialog(catalog(false), ModelsWorkbenchDialog::HeldCatalog),
        ),
        (
            "held-catalog-card-browser-session",
            raster_dialog(
                hub::HubCatalog {
                    host: browser::Host::Browser,
                    ..catalog(false)
                },
                ModelsWorkbenchDialog::HeldCatalog,
            ),
        ),
        (
            "held-catalog-card-discarded",
            raster_dialog(
                hub::HubCatalog {
                    cache_discarded: true,
                    stale: true,
                    ..hub::HubCatalog::default()
                },
                ModelsWorkbenchDialog::HeldCatalog,
            ),
        ),
    ];

    for (name, canvas) in renders {
        let height = canvas.content_height().max(1);
        let bytes = canvas.png(height);
        let path = directory.join(format!("models-{name}.png"));
        std::fs::write(&path, &bytes).expect("write hub render");
        writeln!(
            report_output,
            "{} {}x{} {} bytes",
            path.display(),
            canvas.width(),
            height,
            bytes.len()
        )
        .expect("write raster qualification report");
    }
}

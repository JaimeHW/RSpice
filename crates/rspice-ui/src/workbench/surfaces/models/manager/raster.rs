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
                catalog_page(ui, &mut context, &hub);
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
        ));
    }
    hub::HubCatalog {
        packs,
        age_days: Some(2),
        signed: Some("2026-08-18".to_owned()),
        unavailable: None,
        stale: false,
        cache_discarded: false,
        identity: Some(crate::state::model_hub::CatalogIdentity {
            generation: Some(41),
            digest: "c1d9".repeat(16),
            schema: 2,
            generated_at: "2026-08-18T04:12:09Z".to_owned(),
        }),
        signing_key: "7ce1fddbb60d7a3ba6a09d5bf669087cd59104fe9fbaad72bbf42e41762f957a".to_owned(),
        licences: vec!["LicenseRef-RSpice-Models".to_owned()],
    }
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
                });
            }),
        ),
        (
            "ledger-facet-needs-attention",
            raster(ModelsCatalogScope::InstalledPacks, catalog(true), |state| {
                state.workbench.models_view.hub_facet = ModelHubFacet::NeedsAttention;
            }),
        ),
        (
            "shelf-class-facet",
            raster(ModelsCatalogScope::RSpiceLibrary, catalog(false), |state| {
                state.workbench.models_view.part_facet = RSpicePartFacet::Diode;
            }),
        ),
        (
            "held-catalog-card",
            raster_dialog(catalog(false), ModelsWorkbenchDialog::HeldCatalog),
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

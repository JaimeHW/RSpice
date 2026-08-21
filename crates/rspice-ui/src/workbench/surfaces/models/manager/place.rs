//! What placing one shelf row would actually do.
//!
//! Browsing parts a schematic cannot receive is a catalogue, not a chooser, so
//! the shelf offers Place — and Place has to mean the same thing whichever of
//! the three places on this machine a part came from.
//!
//! # One route, entered from two doors
//!
//! There is exactly one way a part reaches the cursor:
//! [`crate::state::schematic::SchematicState::arm_pack_part`], taking a
//! [`PartPlacement`]. The Model Hub already drives it — `InstallPack { part }`
//! retains the bytes, publishes the project revision, and arms the cursor last,
//! in that order, so a placement is never armed against a part the project does
//! not yet hold. This module does not add a second route; it decides which of
//! the two doors a row goes through.
//!
//! - A definition the project already holds — compiled in, or retained — is
//!   armed directly. Nothing needs downloading, proving, or publishing.
//! - A part of an installed release the project has *not* adopted needs its
//!   bytes retained first, so it is enqueued as the same `InstallPack` request
//!   the pack confirmation raises. The install step is skipped for a release
//!   already on this machine, so the round trip is a retention and an arming.
//!
//! # Refusal is a sentence, not a missing button
//!
//! A control that vanishes teaches nothing. Every reason placement is
//! impossible is a string here, and the shelf renders the control disabled
//! carrying it.

use super::*;

use crate::state::model_hub::{InstalledPack, PartPlacement, plan_library_placement};

use held_parts::{PartOrigin, ShelfRow};

/// How one shelf row reaches the cursor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PlaceRoute {
    /// Arm now: the definition is already in this project's catalog.
    Arm(Box<PartPlacement>),
    /// Retain the release's part into the project first. The hub operation
    /// arms the cursor when it completes, which is why nothing is armed here.
    Retain {
        pack_id: String,
        version: String,
        part: String,
    },
}

/// The route this row would take, or why it has none.
///
/// The error is the sentence a reader sees on the disabled control, so it says
/// what about *this part* cannot be drawn rather than that something failed.
pub(super) fn place_route(
    manager: &ModelLibraryManager,
    row: &ShelfRow,
) -> Result<PlaceRoute, String> {
    match &row.origin {
        PartOrigin::Foundation { library } | PartOrigin::ProjectRetained { library } => {
            let library = manager.get_library(library).ok_or_else(|| {
                format!("Model library '{library}' is no longer loaded in this session.")
            })?;
            plan_library_placement(library, &row.hit.name)
                .map(|placement| PlaceRoute::Arm(Box::new(placement)))
                .map_err(sentence)
        }
        PartOrigin::InstalledPack { pack_id, version } => Ok(PlaceRoute::Retain {
            pack_id: pack_id.clone(),
            version: version.clone(),
            part: row.hit.name.clone(),
        }),
        // The corpus tree is a directory of source files, not a project
        // closure. A row still classified as one is a definition no loaded
        // library holds — so there is nothing in this project to place, and
        // adding the part is what authenticates and retains its bytes. The
        // shelf offers exactly that, beside this, on the same row.
        PartOrigin::Corpus => Err(format!(
            "'{}' is a shipped-corpus definition this project has not added. Add it to the \
             project first — an instance of source the project does not hold would leave a \
             schematic that cannot be simulated anywhere else.",
            row.hit.name
        )),
    }
}

/// Why this session cannot place anything at all right now.
///
/// Separate from [`place_route`] because these are facts about the session, not
/// about the part: they change with which document is in front and whether the
/// project may be written, and they apply to every row on the page at once.
pub(super) fn session_block_reason(state: &AppState, route: &PlaceRoute) -> Option<&'static str> {
    // A placement is armed on a sheet's cursor, so there has to be a sheet.
    // The active view having no schematic buffer is exactly that condition:
    // a symbol view is in front, and a device has nowhere to land on one.
    if state.workspace.active_schematic().is_none() {
        return Some(
            "The view in front is not a schematic, so a placed part would have nowhere to land. Open a schematic first.",
        );
    }
    if state.workbench.safe_mode.project_read_only() {
        return Some(
            "This project is open read-only in safe mode, so it cannot receive an instance.",
        );
    }
    if matches!(route, PlaceRoute::Retain { .. })
        && state.workbench.models_view.model_import_in_progress
    {
        return Some("Another model-source operation is still running.");
    }
    None
}

/// One refusal, ended as a sentence.
///
/// The placement planner writes clause-shaped diagnostics — "part 'X' declares
/// 3 terminals but Diode has 2" — because its other readers are receipts. On a
/// disabled control it is read as prose, so it is capitalized and stopped.
fn sentence(reason: String) -> String {
    let mut characters = reason.chars();
    let mut sentence = match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect::<String>(),
        None => return "This part cannot be placed.".to_owned(),
    };
    if !sentence.ends_with('.') {
        sentence.push('.');
    }
    sentence
}

/// Arms the cursor with one planned placement, and hands over the canvas.
///
/// Runs after the frame, from the workspace's own action queue, because a
/// placement ends in three things a render borrow cannot reach: a toast, the
/// canvas focus, and the workspace receipt. It is the same completion the Model
/// Hub performs when a retention lands, one operation later.
pub(super) fn arm(state: &mut AppState, ctx: &egui::Context, placement: PartPlacement) {
    let armed = state.schematic.arm_pack_part(placement);
    crate::schematic::view::request_schematic_canvas_focus(ctx);
    state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{armed} will snap to the schematic grid."),
    );
    apply_receipt(state, Ok(format!("{armed} is armed for placement.")));
}

/// Everything the shelf needs to offer Place on one row.
///
/// Assembled once per painted detail rather than per control so the route is
/// planned exactly once for the row a reader is looking at.
pub(super) struct PlaceOffer {
    pub(super) route: Option<PlaceRoute>,
    pub(super) blocked: Option<String>,
}

impl PlaceOffer {
    pub(super) fn for_row(state: &AppState, installed: &[InstalledPack], row: &ShelfRow) -> Self {
        let manager = &state.model_library_manager;
        // A retention names a release; if the store no longer holds it, the
        // route is a promise about something that is gone.
        if let PartOrigin::InstalledPack { pack_id, version } = &row.origin
            && !installed
                .iter()
                .any(|pack| pack.pack_id() == pack_id && pack.version() == version)
        {
            return Self {
                route: None,
                blocked: Some(format!(
                    "{pack_id} {version} is no longer installed on this machine."
                )),
            };
        }
        match place_route(manager, row) {
            Err(reason) => Self {
                route: None,
                blocked: Some(reason),
            },
            Ok(route) => {
                let blocked = session_block_reason(state, &route).map(str::to_owned);
                Self {
                    route: Some(route),
                    blocked,
                }
            }
        }
    }
}

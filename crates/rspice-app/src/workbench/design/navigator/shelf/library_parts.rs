//! The shelf's library parts: every part the indexed model libraries offer
//! this shelf, and where a click on one goes: armed now, or the pack
//! confirmation first.

use super::*;

/// Complete one click on a library-part row.
pub(super) fn apply_library_part_row(
    app: &mut RSpiceApp,
    row: LibraryPartRow,
    ctx: &egui::Context,
) {
    match row.action {
        LibraryPartAction::Arm(placement) => arm_library_part(&mut app.state, *placement, ctx),
        LibraryPartAction::Review {
            pack_id, version, ..
        } => request_library_part(app, row.part_id, pack_id, version),
        // A refused row renders disabled, so its click never arrives.
        LibraryPartAction::Refused(_) => {}
    }
}

/// One indexed part the shelf can offer, with what a click would do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench::design::navigator) struct LibraryPartRow {
    pub(in crate::workbench::design::navigator) part_id: String,
    /// The part's device class, in the unified index's vocabulary.
    pub(in crate::workbench::design::navigator) device: String,
    /// The part's address: its pack, or the project library holding it.
    pub(in crate::workbench::design::navigator) source: String,
    /// The meta column: the part's device class, and where it stands.
    pub(in crate::workbench::design::navigator) meta: String,
    pub(in crate::workbench::design::navigator) action: LibraryPartAction,
}

/// Where one library-part click goes.
///
/// The same fork the Models workspace shelf decides in its `place` module: a
/// definition the project already holds is armed directly, a release it has
/// not adopted is reviewed first, and a part that cannot be drawn is refused
/// with the sentence the disabled row carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench::design::navigator) enum LibraryPartAction {
    /// Arm now: the definition is already in this project's catalog —
    /// compiled in, or retained when the part was added.
    Arm(Box<PartPlacement>),
    /// Raise the pack confirmation in the Models workspace. Only right for a
    /// release that still needs installing or retaining; the hub operation
    /// arms the cursor when the retention lands.
    Review {
        pack_id: String,
        version: String,
        pack_name: String,
    },
    /// No placement exists for this part; the sentence the disabled control
    /// carries.
    Refused(String),
}

/// Every part the unified index can offer this shelf.
///
/// A part the project retained — and a compiled-in foundation part — is
/// listed here and arms directly. The "Project library" section below reads
/// the symbol-cell library, not the model-library manager, so this section is
/// the only door such a definition has onto the canvas. Pack releases the
/// project has not adopted sit beside them and go through the pack
/// confirmation instead.
pub(in crate::workbench::design::navigator) fn library_part_rows(
    app: &RSpiceApp,
    query: &ShelfQuery,
) -> Vec<LibraryPartRow> {
    let libraries = app.state.model.model_library_manager.libraries_sorted();
    let index = app.model_hub.part_index(&libraries);
    shelf_rows(&index, &libraries, query)
}

/// Decides each indexed row's door, dropping the rows that have none.
///
/// Separated from [`library_part_rows`] so a test can feed it an index
/// without opening a hub over a store.
pub(in crate::workbench::design::navigator) fn shelf_rows(
    index: &[ModelHubPartRow],
    libraries: &[&ModelLibrary],
    query: &ShelfQuery,
) -> Vec<LibraryPartRow> {
    /// Rows the shelf lists before asking the user to narrow the search.
    ///
    /// A published catalog is unbounded; a navigator column is not. The cap is
    /// a rendering decision, and the footer says when it bit.
    const SHELF_ROWS: usize = 200;

    // A part the project adopted is the retained row's to place: its
    // installed-pack row is dropped rather than offering to review and add
    // something that is already added — the same rule the Models workspace
    // shelf applies to its own two halves.
    let retained = index
        .iter()
        .filter(|row| matches!(row.provenance, PartProvenance::ProjectRetained { .. }))
        .map(|row| row.part_id.as_str())
        .collect::<HashSet<_>>();
    index
        .iter()
        .filter(|row| match &row.provenance {
            PartProvenance::InstalledPack { .. } => !retained.contains(row.part_id.as_str()),
            // A section-scoped subcircuit key cannot be referenced by its
            // bare name, so it is not a part a reader picks.
            PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => {
                !row.part_id.contains('\u{1f}')
            }
            PartProvenance::RemoteRelease { .. } => true,
        })
        .filter(|row| {
            library_part_rank(
                query,
                &row.part_id,
                &row.device,
                part_source(&row.provenance),
            )
            .is_some()
        })
        .take(SHELF_ROWS)
        .filter_map(|row| library_part_row(libraries, row))
        .collect()
}

/// The address a part row is searched by besides its number: the pack for a
/// pack row, the holding library for a project one — the same column the
/// Models workspace shelf searches.
fn part_source(provenance: &PartProvenance) -> &str {
    match provenance {
        PartProvenance::InstalledPack { pack_id, .. }
        | PartProvenance::RemoteRelease { pack_id, .. } => pack_id.as_str(),
        PartProvenance::ProjectRetained { library } => library.as_str(),
        PartProvenance::Foundation => "",
    }
}

/// How one library part answers a query. Its number and address are part
/// numbers and its device class supplies the words; it has no label beyond
/// its number.
pub(super) fn library_part_rank(
    query: &ShelfQuery,
    part_id: &str,
    device: &str,
    source: &str,
) -> Option<MatchRank> {
    query.rank(&ShelfTerms {
        label: "",
        keywords: device_class_keywords(device),
        part_numbers: &[part_id, source],
    })
}

/// One shelf row, with the route its click takes.
fn library_part_row(libraries: &[&ModelLibrary], row: &ModelHubPartRow) -> Option<LibraryPartRow> {
    let (meta, action) = match &row.provenance {
        PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => {
            let holder = holding_library(libraries, &row.provenance, &row.part_id)?;
            let action = match plan_library_placement(holder, &row.part_id) {
                Ok(placement) => LibraryPartAction::Arm(Box::new(placement)),
                Err(reason) => LibraryPartAction::Refused(refusal_sentence(reason)),
            };
            // A part that is here and placeable is the normal case; where
            // it came from is the Models workspace's business.
            (row.device.clone(), action)
        }
        PartProvenance::InstalledPack { pack_id, version }
        | PartProvenance::RemoteRelease { pack_id, version } => {
            let pack_name = row.pack_name.clone().unwrap_or_else(|| pack_id.clone());
            let meta = match &row.state {
                PartState::Installed => row.device.clone(),
                PartState::Available => format!("{} · available", row.device),
                PartState::UpdateAvailable { latest, .. } => {
                    format!("{} · update {latest}", row.device)
                }
                PartState::Incompatible { missing } => {
                    format!("{} · needs {}", row.device, missing.join(", "))
                }
            };
            let action = match &row.state {
                // The row stays searchable and stays readable; only the
                // action is refused, and the refusal says why in the same
                // words the pack manifest used.
                PartState::Incompatible { missing } => LibraryPartAction::Refused(format!(
                    "This build of RSpice does not offer {}, which {} requires.",
                    missing.join(", "),
                    pack_name
                )),
                _ => LibraryPartAction::Review {
                    pack_id: pack_id.clone(),
                    version: version.clone(),
                    pack_name,
                },
            };
            (meta, action)
        }
    };
    Some(LibraryPartRow {
        part_id: row.part_id.clone(),
        device: row.device.clone(),
        source: part_source(&row.provenance).to_owned(),
        meta,
        action,
    })
}

/// The loaded library whose definition a foundation or retained row names.
///
/// Both come straight out of the same library set the index was built from in
/// this frame, so a miss means the row and the set disagree — the row is
/// dropped rather than offered against bytes that are not there.
fn holding_library<'a>(
    libraries: &[&'a ModelLibrary],
    provenance: &PartProvenance,
    part: &str,
) -> Option<&'a ModelLibrary> {
    libraries.iter().copied().find(|library| match provenance {
        PartProvenance::ProjectRetained { library: name } => library.name == *name,
        PartProvenance::Foundation => {
            matches!(library.source_authority, ModelSourceAuthority::BuiltIn)
                && (library.models.contains_key(part)
                    || library.top_level_models.contains_key(part)
                    || library.subcircuits.contains_key(part))
        }
        PartProvenance::InstalledPack { .. } | PartProvenance::RemoteRelease { .. } => false,
    })
}

/// The shelf section for the parts the unified model index lists.
pub(super) fn library_parts_section(
    ui: &mut Ui,
    state: &mut AppState,
    rows: &[LibraryPartRow],
) -> Option<LibraryPartRow> {
    if rows.is_empty() {
        return None;
    }
    if !catalog_group_row(
        ui,
        "component-shelf-library-parts",
        ShelfGlyph::Icon(WorkbenchIcon::Models),
        "Library parts",
        rows.len(),
        false,
    ) {
        return None;
    }

    let mut requested = None;
    for row in rows {
        let placeable = !matches!(row.action, LibraryPartAction::Refused(_));
        let offered = ui
            .add_enabled_ui(placeable, |ui| {
                let response = nav_row_indented_response(
                    ui,
                    WorkbenchIcon::Models,
                    &row.part_id,
                    false,
                    Some(&row.meta),
                    2,
                );
                match &row.action {
                    // The row stays searchable and stays readable; only the
                    // action is refused, and the refusal says why.
                    LibraryPartAction::Refused(reason) => {
                        response.on_disabled_hover_text(reason.as_str());
                        None
                    }
                    LibraryPartAction::Arm(_) => {
                        Some(response.on_hover_text(format!("Click to arm {}", row.part_id)))
                    }
                    LibraryPartAction::Review {
                        pack_name, version, ..
                    } => Some(response.on_hover_text(format!(
                        "Review and add {} from {} {}",
                        row.part_id, pack_name, version
                    ))),
                }
            })
            .inner;
        // A refused part has no placement to pin, so it carries no pin menu
        // either: the rail must not fill with doors that lead nowhere.
        let Some(response) = offered else {
            continue;
        };
        if response.clicked() {
            requested = Some(row.clone());
        }
        shelf_pin_context_menu(
            &response,
            state,
            &ShelfEntry::LibraryPart(row.part_id.clone()),
        );
    }
    requested
}

/// Arms the cursor with a definition the project already holds.
///
/// The same completion the Models workspace shelf performs when it arms a
/// held part, minus that workspace's operation receipt: this click happened
/// on the canvas side, so the armed cursor is the whole story.
fn arm_library_part(
    state: &mut crate::workbench::app_state::AppState,
    placement: PartPlacement,
    ctx: &egui::Context,
) {
    let _ = crate::workbench::app::arm_pack_part(&mut state.model, placement);
    finish_shelf_placement(state, ctx);
}

/// Raises the pack confirmation for one shelf part.
///
/// The decision is shown in the Models workspace rather than over the canvas:
/// it commits the project to a licence, a download, and a capability claim,
/// and those are exactly what that workspace is for. The placement is armed
/// on the cursor when the install completes, so the round trip ends where the
/// user started.
fn request_library_part(app: &mut RSpiceApp, part_id: String, pack_id: String, version: String) {
    app.state.workbench.outbox.push(ConfirmPackAttach {
        part_id,
        pack_id,
        version,
    });
}

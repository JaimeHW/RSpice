//! Exporting a plan to a portable package and importing one back.
//!
//! One of the five routes the plan-manager shell dispatches to. Both directions
//! share this file because they share the package format: the version and format
//! tags below are the compatibility contract, and a reader that accepted a
//! package it did not recognise would import a plan it could not honour.
//!
//! An import mints a new local identity and retains the source lineage. It does
//! not copy result references — a run's receipt names the plan it was dispatched
//! from, and importing a plan elsewhere cannot manufacture runs of it.
//!
//! **Neither direction paints the package.** A package is a file the reader
//! chooses, not text they copy between two windows, and a route that painted its
//! JSON put the one thing it exists to carry below the fold of a dialog that is
//! not allowed to scroll. What each direction shows instead is what the package
//! *is*: its envelope, the plan it holds, and the boundary of the transaction.

use super::*;

use crate::workbench::design_system::property_row_input;

/// The package's compatibility contract, and the only version this build reads.
const SIMULATION_PLAN_PACKAGE_VERSION: u16 = 1;
const SIMULATION_PLAN_PACKAGE_FORMAT: &str = "rspice.simulation-plan";

/// The authored extension for a package on disk, and the label the chooser
/// files it under.
///
/// The contents are JSON, and the import filter accepts `json` as well so a
/// package saved or received under the generic extension still opens.
const PACKAGE_EXTENSION: &str = "rspice-plan";
const PACKAGE_JSON_EXTENSION: &str = "json";
const PACKAGE_FILTER_LABEL: &str = "RSpice simulation-plan package";

/// Longest file-name stem this route proposes.
///
/// The destination the reader picks is not the only path the write needs: the
/// durable publication stages a sidecar beside it, so a plan name long enough to
/// be legal here can produce a sidecar path that is not. The cap is on what this
/// route suggests, never on what the reader then types.
const PACKAGE_STEM_LIMIT: usize = 64;

/// The largest package this route will read.
///
/// A plan package is a document, not a dataset: the whole catalog projection is
/// counts and identities. A file past this size is a file that was not produced
/// by the export beside it, and reading it into the draft would be the failure.
const MAX_PACKAGE_BYTES: usize = 4 * 1024 * 1024;

/// The whole wire form. Unknown fields are refused rather than dropped, so a
/// package written by a build that knows a field this one does not is rejected
/// with that field named instead of imported with it silently missing.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct PortableSimulationPlanPackage {
    format: String,
    version: u16,
    plan: crate::workbench::app_state::SimulationPlanImportDocument,
    payload: crate::state::SimulationPlanPayload,
}

/// What the package carries, what it does not, and what it is not.
///
/// Every clause is read off the two structures the package is made of.
/// `SimulationPlanImportDocument` carries the source identity and revision, the
/// name, the analysis plan, the reference PVT corner, the run set, the model
/// bindings, the save policy and the solver options.
/// `SimulationPlanPayload` carries the design variables, saved outputs,
/// specifications and their policy, the regression tolerances, and the pinned
/// baseline run.
///
/// The exclusion states an absence, and it is stated because a portable package
/// is exactly the artifact a reader is entitled to ask that question about. The
/// authored reference offers a digest inventory and a signature verification
/// stage; RSpice signs nothing and has nothing to verify a signature against, so
/// saying so is the honest form of that claim.
///
/// It is the authored pair — Included beside Excluded — rather than the three
/// boxes this held. The third was headed "Not authenticated" and listed what the
/// package does not carry, which is what Excluded is: an exclusion is an
/// exclusion whether it is a dataset or a digest, and a third heading for the
/// same kind of claim spent a third of the row saying so twice.
///
/// The authored surface also offers a package-format choice, a plan-revision
/// choice, a reference policy and a redaction policy. There is one format, the
/// export is of the plan as the catalog holds it, nothing is embedded by
/// reference, and there is no classification to redact — so all four are omitted
/// rather than rendered as a control with one reachable option.
const EXPORT_NOTES: [(&str, &str); 2] = [
    (
        "Included",
        "The plan's analysis instances, its reference PVT corner, its declared \
         run set, its model bindings, its save policy and solver options, and \
         the plan-owned design variables, saved outputs, specifications and \
         regression tolerances. The format tag and the version are the whole \
         compatibility contract, and a field this build does not know is \
         refused on import rather than ignored.",
    ),
    (
        "Excluded",
        "Result samples and datasets. A run's authenticated receipt names the \
         plan it was dispatched from and cannot travel with it, and the pinned \
         regression baseline the package carries is dropped rather than \
         recreated when the package is imported. No digest and no signature \
         either: this build has nothing to verify one against.",
    ),
];

/// The authored validation table's three columns.
const IMPORT_STAGE_HEADINGS: [&str; 3] =
    ["Validation stage", "Required outcome", "Failure behavior"];

/// The stages [`commit_import_simulation_plan`] runs, in the order it runs them,
/// each with the outcome it demands and what a refusal there leaves behind.
///
/// The two halves of the authored table that RSpice cannot own are absent rather
/// than restated: nothing here checks a digest or a signature, and nothing
/// classifies a capability or an entitlement.
///
/// The three rows are three *failure behaviors*, which is why they are three
/// rows and not seven. Everything up to the model-binding check runs before any
/// copy of the project exists, so a refusal there touches nothing at all; the
/// four stages after it run against clones of the setup and the workspace that
/// replace the real ones only once every stage has passed, so a refusal in any
/// of them discards the candidate and leaves the current plan active. A column
/// repeating one of those two sentences four times is what the authored table
/// spends a third of its width on.
const IMPORT_STAGES: [[&str; 3]; 3] = [
    [
        "Envelope format and version",
        "a package this build's reader accepts",
        "package rejected · no project mutation",
    ],
    [
        "Model bindings",
        "every binding resolves in the library",
        "import blocked until bindings are reviewed",
    ],
    [
        "Catalog, payload, configuration, context",
        "each applies to a copy of the project",
        "candidate discarded · plan stays active",
    ],
];

/// What the envelope row says before a package has been chosen.
const NO_PACKAGE_CHOSEN: &str = "no package chosen";

/// [`IMPORT_STAGES`] as the authored table, or as one group per stage where the
/// track cannot hold three columns.
///
/// The columns are measured from the cells rather than authored, because a
/// three-column table of clauses is exactly the shape that elides silently: a
/// stage shortened to `Catalog, payload, config…` sits inside its clip rect and
/// passes every fit gate this module has.
fn import_stage_table(ui: &mut Ui) {
    let columns = IMPORT_STAGE_HEADINGS
        .iter()
        .enumerate()
        .map(|(index, heading)| {
            let width = IMPORT_STAGES
                .iter()
                .map(|stage| kit::prose_column_width(ui, heading, stage[index]))
                .fold(0.0_f32, f32::max);
            TableColumn {
                heading,
                track: if index + 1 == IMPORT_STAGE_HEADINGS.len() {
                    ColumnTrack::Elastic(width)
                } else {
                    ColumnTrack::Fixed(width)
                },
            }
        })
        .collect::<Vec<_>>();
    if ui.available_width() >= kit::table_minimum_width(&columns) {
        let rows = IMPORT_STAGES
            .iter()
            .map(|stage| TableRow {
                selected: false,
                announced: format!("{}: {} · on failure, {}", stage[0], stage[1], stage[2]),
            })
            .collect::<Vec<_>>();
        kit::read_only_records_table(
            ui,
            "simulation.plan-manager.import.stages",
            &columns,
            &rows,
            |ui, row, column| {
                let t = Tokens::get(ui.ctx());
                kit::cell_prose(ui, IMPORT_STAGES[row][column], t.color.text);
            },
        );
        return;
    }
    // Too narrow for three tracks. Each stage keeps its two claims, under its
    // own name, where the values have the whole width instead of a third of it.
    for stage in IMPORT_STAGES {
        kit::section_head(ui, stage[0], None);
        property_row(ui, IMPORT_STAGE_HEADINGS[1], stage[1]);
        property_row(ui, IMPORT_STAGE_HEADINGS[2], stage[2]);
    }
}

/// The route's dialog. See the shell's child-dialog signature contract.
pub(super) fn dialog(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    if draft.mode == SimulationPlanManagerMode::Import {
        return import(ctx, draft);
    }
    export(ctx, draft, records)
}

/// What the export states about the plan it is about to write out.
///
/// It reads the catalog projection rather than the package text, for the reason
/// the shell's contract gives: every count a surface shows comes from one owner,
/// so the number here and the number in the same plan's table row cannot
/// disagree. The one fact that is the package's own — how large it came out — is
/// measured off the serialized text the shell already put in the draft.
fn export(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
    records: &[PlanCatalogRecord],
) -> Option<PlanManagerAction> {
    // A save the browser completed between frames closes the route exactly as a
    // save the desktop completed within one does. Painting a dialog that is
    // closing this frame would only be work the shell throws away.
    if poll_saved_package(ctx, draft) {
        return Some(PlanManagerAction::CancelInline);
    }
    let selected = records
        .iter()
        .find(|record| record.id == draft.selected_plan_id);
    let mut action = None;
    let mut refusal = None;
    let choice = Dialog::new(
        "SIMULATION · EXPORT PLAN · PORTABLE PACKAGE",
        "Export simulation plan",
        "Save package…",
    )
    .description(
        "The selected plan's document and its plan-owned payload, written as one portable package. Exporting reads the catalog and changes nothing.",
    )
    .size(DialogSize::SimulationWorkflow)
    .secondary("Copy JSON")
    .ghost("Close")
    .show(ctx, |ui| {
        kit::section_head(
            ui,
            "Exported plan",
            selected.map(|record| HeadStatus {
                label: record.lifecycle_label(),
                tone: lifecycle_tone(record),
            }),
        );
        if let Some(record) = selected {
            property_row(ui, "Name", &record.name);
            property_row(
                ui,
                "Source identity",
                &format!("{} · revision {}", record.id, record.revision),
            );
            property_row(
                ui,
                "Declares",
                &format!(
                    "{} of {} analyses enabled · {} model binding{}",
                    record.enabled,
                    record.analyses,
                    record.model_bindings,
                    plural_suffix(record.model_bindings)
                ),
            );
            property_row(
                ui,
                "Plan-owned records",
                &format!(
                    "{} variable{} · {} saved output{} · {} specification{}",
                    record.design_variables,
                    plural_suffix(record.design_variables),
                    record.saved_outputs,
                    plural_suffix(record.saved_outputs),
                    record.specifications,
                    plural_suffix(record.specifications)
                ),
            );
        }
        property_row(ui, "Package", &envelope_label(draft.exchange_text.len()));
        ui.add_space(8.0);
        kit::note_grid(ui, &EXPORT_NOTES);
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    match choice {
        // Writing the file is the whole transaction, so completing it returns to
        // the catalog the same way every other route's commit does. The reader
        // named the destination in the chooser a moment ago; repeating it here
        // would need a receipt this route has nowhere to keep.
        DialogChoice::Primary => {
            match save_exported_package(ctx, &package_file_name(selected), &draft.exchange_text) {
                Ok(true) => action = Some(PlanManagerAction::CancelInline),
                Ok(false) => {}
                Err(error) => refusal = Some(error),
            }
        }
        DialogChoice::Secondary => action = Some(PlanManagerAction::CopyExport),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None => {}
    }
    // A refusal is reported instead of an action, never alongside one: the shell
    // clears this field on `CancelInline` and on every successful commit, so a
    // refusal returned together with either would be erased before it was ever
    // painted. Returning nothing leaves the route open on the same package with
    // the reason under it.
    if let Some(error) = refusal {
        draft.validation_error = Some(error);
    }
    action
}

/// What the import states about the package it holds.
///
/// The envelope stage is evaluated here, because it is pure — the same
/// deserialization and the same two tag checks the commit performs, through the
/// same function. Nothing else is: every later stage needs the catalog and the
/// model library, which a route deliberately cannot reach, so those are named as
/// what the import will check rather than pre-marked as passed. A column of
/// green marks for checks that have not run is the defect this route is not
/// allowed to ship.
fn import(
    ctx: &egui::Context,
    draft: &mut SimulationPlanManagerDraft,
) -> Option<PlanManagerAction> {
    poll_chosen_package(ctx, draft);
    accept_pasted_package(ctx, draft);

    let loaded = !draft.exchange_text.trim().is_empty();
    let inspected = loaded.then(|| inspect_package(&draft.exchange_text));
    let envelope_error = match &inspected {
        Some(Err(error)) => Some(error.clone()),
        _ => None,
    };
    let mut action = None;
    let mut chooser = false;
    let choice = Dialog::new(
        "SIMULATION · IMPORT PLAN · NEW LOCAL IDENTITY",
        "Import simulation plan",
        "Import plan",
    )
    .description(
        "An imported plan is created under a new local identity and activated. Its source lineage is retained; no result reference is copied.",
    )
    .size(DialogSize::SimulationWorkflow)
    .ghost("Cancel")
    .hint("Choose a package file, or paste one with Ctrl+V")
    .primary_enabled(!draft.name.trim().is_empty() && matches!(inspected, Some(Ok(_))))
    .show(ctx, |ui| {
        let t = Tokens::get(ui.ctx());
        // No status word on this head. Whether a package is loaded, and whether
        // it is one this build accepts, are the same question, and the envelope
        // row below answers it with a mark. A second answer here would either
        // repeat it or, worse, disagree with it — and it would have to borrow
        // the lifecycle vocabulary, which states what a *plan* is, to do so.
        kit::section_head(ui, "Source package", None);
        ui.horizontal(|ui| {
            if Button::new("Choose file…").show(ui).clicked() {
                chooser = true;
            }
        });
        match &inspected {
            Some(Ok(package)) => {
                property_row_status(
                    ui,
                    "Package envelope",
                    &envelope_label(draft.exchange_text.len()),
                    t.color.ok,
                    StatusMark::Success,
                );
                property_row(
                    ui,
                    "Source plan",
                    &format!(
                        "{} · revision {}",
                        package.plan.source_plan_id,
                        package.plan.source_revision.get()
                    ),
                );
                property_row(
                    ui,
                    "Declares",
                    &format!(
                        "{} analysis instance{} · {} model binding{}",
                        package.plan.analysis_plan.instances().len(),
                        plural_suffix(package.plan.analysis_plan.instances().len()),
                        package.plan.model_bindings.len(),
                        plural_suffix(package.plan.model_bindings.len())
                    ),
                );
                property_row(
                    ui,
                    "Plan-owned records",
                    &format!(
                        "{} variable{} · {} saved output{} · {} specification{}",
                        package.payload.design_variables.len(),
                        plural_suffix(package.payload.design_variables.len()),
                        package.payload.saved_outputs.len(),
                        plural_suffix(package.payload.saved_outputs.len()),
                        specification_count(&package.payload),
                        plural_suffix(specification_count(&package.payload))
                    ),
                );
            }
            // The row states the verdict and the band below states the reason,
            // because a deserializer's reason is a sentence and a property row
            // is one elided line.
            Some(Err(_)) => {
                property_row_status(
                    ui,
                    "Package envelope",
                    "not a package this build accepts",
                    t.color.err,
                    StatusMark::Failure,
                );
            }
            None => {
                property_row_status(
                    ui,
                    "Package envelope",
                    NO_PACKAGE_CHOSEN,
                    t.color.text_dim,
                    StatusMark::Neutral,
                );
            }
        }
        // No gap before the head: it draws its own closing rule, which is the
        // separator, and a dialog that may not scroll does not spend height on
        // a second one.
        kit::section_head(ui, "Imported plan", None);
        // The editable twin of the rows above, so the name lines up with the
        // facts it is being given rather than sitting in its own arrangement.
        // Empty is the one state that disables the primary, and outlining the
        // input is what says which control the refusal is about.
        let unnamed = draft.name.trim().is_empty();
        property_row_input(ui, "Name", &mut draft.name, unnamed);
        ui.add_space(8.0);
        import_stage_table(ui);
        // The commit's refusal wins: it is the later and more specific of the
        // two, and it is the one the reader just pressed a button to get.
        workflow_validation_message(
            ui,
            draft
                .validation_error
                .as_deref()
                .or(envelope_error.as_deref()),
        );
    });
    if chooser {
        choose_import_package(ctx, draft);
    }
    match choice {
        DialogChoice::Primary => action = Some(PlanManagerAction::ApplyImport),
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            action = Some(PlanManagerAction::CancelInline);
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
    action
}

/// The envelope, as both directions state it: the contract, then the size.
fn envelope_label(bytes: usize) -> String {
    format!(
        "{SIMULATION_PLAN_PACKAGE_FORMAT} · version {SIMULATION_PLAN_PACKAGE_VERSION} · {}",
        crate::simulation::run_set::format_bytes(bytes as u64)
    )
}

/// How many specifications the payload really carries.
///
/// It mirrors `Workspace::import_plan_data`, which migrates the legacy `specs`
/// list into governed definitions exactly when the governed list is empty. A
/// count taken off either field alone would state zero for one of the two shapes
/// a package can legitimately have.
fn specification_count(payload: &crate::state::SimulationPlanPayload) -> usize {
    if payload.specification_definitions.is_empty() {
        payload.specs.len()
    } else {
        payload.specification_definitions.len()
    }
}

/// Read a package, and refuse it for exactly the reasons the commit would.
///
/// One owner for the envelope stage. The dialog previews with this and
/// [`commit_import_simulation_plan`] admits with it, so a package the preview
/// shows as sound cannot be refused by the commit on envelope grounds, and one
/// the preview refuses is refused there with the identical sentence.
fn inspect_package(json: &str) -> Result<PortableSimulationPlanPackage, String> {
    let package: PortableSimulationPlanPackage = serde_json::from_str(json)
        .map_err(|error| format!("Simulation-plan package JSON is invalid: {error}"))?;
    if package.format != SIMULATION_PLAN_PACKAGE_FORMAT {
        return Err(format!(
            "Unsupported simulation-plan package format '{}'.",
            package.format
        ));
    }
    if package.version != SIMULATION_PLAN_PACKAGE_VERSION {
        return Err(format!(
            "Unsupported simulation-plan package version {}; this build accepts version {}.",
            package.version, SIMULATION_PLAN_PACKAGE_VERSION
        ));
    }
    Ok(package)
}

/// The file name the save chooser opens with.
///
/// Derived from the plan's own name so a catalog of exports is readable on disk,
/// reduced to characters every file system accepts, and capped at
/// [`PACKAGE_STEM_LIMIT`].
fn package_file_name(selected: Option<&PlanCatalogRecord>) -> String {
    let source = selected.map_or("simulation plan", |record| record.name.as_str());
    let mut stem = String::new();
    for character in source.chars() {
        if stem.len() >= PACKAGE_STEM_LIMIT {
            break;
        }
        if character.is_ascii_alphanumeric() {
            stem.push(character.to_ascii_lowercase());
        } else if !stem.ends_with('-') && !stem.is_empty() {
            stem.push('-');
        }
    }
    let stem = stem.trim_matches('-');
    let stem = if stem.is_empty() {
        "simulation-plan"
    } else {
        stem
    };
    format!("{stem}.{PACKAGE_EXTENSION}")
}

/// Accept a package the reader pastes.
///
/// The chooser is the discoverable route in and this is the one they already
/// have: a package that arrived in a message is on the clipboard, not on disk.
/// It is ignored while any control holds focus, so pasting into the name field
/// still pastes into the name field.
fn accept_pasted_package(ctx: &egui::Context, draft: &mut SimulationPlanManagerDraft) {
    if ctx.memory(|memory| memory.focused()).is_some() {
        return;
    }
    let pasted = ctx.input(|input| {
        input.events.iter().rev().find_map(|event| match event {
            egui::Event::Paste(text) if !text.trim().is_empty() => Some(text.clone()),
            _ => None,
        })
    });
    if let Some(text) = pasted {
        draft.exchange_text = text;
        draft.validation_error = None;
    }
}

/// Bound and decode what a chooser handed back.
///
/// Shared by both platforms so the limit and the encoding rule are one rule
/// rather than a desktop one and a browser one free to disagree.
fn bounded_package_text(bytes: Vec<u8>) -> Result<String, String> {
    if bytes.len() > MAX_PACKAGE_BYTES {
        return Err(format!(
            "The simulation-plan package exceeds the {MAX_PACKAGE_BYTES} byte import limit."
        ));
    }
    String::from_utf8(bytes).map_err(|_| "A simulation-plan package must be UTF-8 JSON.".to_owned())
}

/// Write the package to a destination the reader chooses.
///
/// `Ok(true)` means the package is on disk. `Ok(false)` is a chooser that has
/// not produced one — cancelled here, and on the browser also *not yet*, since
/// there the chooser is asynchronous and [`poll_saved_package`] reports it.
/// Neither is a failure, and both leave the route open on the same package.
#[cfg(not(target_arch = "wasm32"))]
fn save_exported_package(_ctx: &egui::Context, name: &str, source: &str) -> Result<bool, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter(
            PACKAGE_FILTER_LABEL,
            &[PACKAGE_EXTENSION, PACKAGE_JSON_EXTENSION],
        )
        .set_file_name(name)
        .save_file()
    else {
        return Ok(false);
    };
    // The same compare-and-exchange publication every other export in the
    // product uses: an existing destination is never overwritten on the strength
    // of a stale observation, and a partially written package is never visible
    // at the path.
    let expected = crate::io::durable_file::observe_expected_content(&path)
        .map_err(|error| format!("Could not authorize the package destination: {error}"))?;
    crate::io::durable_file::compare_exchange_bytes(&path, expected, source.as_bytes())
        .map_err(|error| format!("Could not write the simulation-plan package: {error}"))?;
    Ok(true)
}

/// The browser's save chooser is asynchronous like its open chooser, so the
/// press starts it and a later frame learns whether it produced a destination.
///
/// It goes through `rfd`, not through the workbench's own download helper. The
/// helper synthesizes an anchor click, which hands the file to whatever the user
/// agent does with downloads rather than to a destination the reader picked —
/// and reaching it from a surface would extend a module edge this crate is
/// retiring. One chooser on both platforms is also one behaviour to describe.
#[cfg(target_arch = "wasm32")]
fn save_exported_package(ctx: &egui::Context, name: &str, source: &str) -> Result<bool, String> {
    let id = save_mailbox_id();
    if ctx.data(|data| data.get_temp::<SaveMailbox>(id)).is_some() {
        return Ok(false);
    }
    let mailbox: SaveMailbox = std::sync::Arc::new(std::sync::Mutex::new(None));
    ctx.data_mut(|data| data.insert_temp(id, mailbox.clone()));
    let repaint = ctx.clone();
    let name = name.to_owned();
    let bytes = source.as_bytes().to_vec();
    wasm_bindgen_futures::spawn_local(async move {
        let picked = rfd::AsyncFileDialog::new()
            .add_filter(
                PACKAGE_FILTER_LABEL,
                &[PACKAGE_EXTENSION, PACKAGE_JSON_EXTENSION],
            )
            .set_file_name(&name)
            .save_file()
            .await;
        let result = match picked {
            Some(file) => match file.write(&bytes).await {
                Ok(()) => Ok(true),
                Err(error) => Err(format!(
                    "Could not write the simulation-plan package: {error}"
                )),
            },
            None => Ok(false),
        };
        if let Ok(mut slot) = mailbox.lock() {
            *slot = Some(result);
        }
        repaint.request_repaint();
    });
    Ok(false)
}

#[cfg(target_arch = "wasm32")]
type SaveMailbox = std::sync::Arc<std::sync::Mutex<Option<Result<bool, String>>>>;

#[cfg(target_arch = "wasm32")]
fn save_mailbox_id() -> egui::Id {
    egui::Id::new("simulation.plan-manager.export-package-mailbox")
}

/// Whether the browser's save chooser has completed a write since the last
/// frame. A refusal is reported on the draft; only a completed write closes the
/// route, which is what the desktop's blocking chooser does in one frame.
#[cfg(target_arch = "wasm32")]
fn poll_saved_package(ctx: &egui::Context, draft: &mut SimulationPlanManagerDraft) -> bool {
    let id = save_mailbox_id();
    let Some(mailbox) = ctx.data(|data| data.get_temp::<SaveMailbox>(id)) else {
        return false;
    };
    let Some(result) = mailbox.lock().ok().and_then(|mut slot| slot.take()) else {
        return false;
    };
    ctx.data_mut(|data| data.remove::<SaveMailbox>(id));
    match result {
        Ok(saved) => saved,
        Err(error) => {
            draft.validation_error = Some(error);
            false
        }
    }
}

/// The desktop chooser blocks, so a save is never still in flight.
#[cfg(not(target_arch = "wasm32"))]
const fn poll_saved_package(_ctx: &egui::Context, _draft: &mut SimulationPlanManagerDraft) -> bool {
    false
}

/// Choose a package to import.
///
/// Blocking, because the platform's chooser is: it returns inside the frame the
/// button was pressed in, and the draft the shell re-arms already carries the
/// result.
#[cfg(not(target_arch = "wasm32"))]
fn choose_import_package(_ctx: &egui::Context, draft: &mut SimulationPlanManagerDraft) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter(
            PACKAGE_FILTER_LABEL,
            &[PACKAGE_EXTENSION, PACKAGE_JSON_EXTENSION],
        )
        .pick_file()
    else {
        return;
    };
    match read_package_file(&path) {
        Ok(source) => {
            draft.exchange_text = source;
            draft.validation_error = None;
        }
        Err(error) => draft.validation_error = Some(error),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_package_file(path: &std::path::Path) -> Result<String, String> {
    use std::io::Read as _;

    let file = std::fs::File::open(path)
        .map_err(|error| format!("Could not open the simulation-plan package: {error}"))?;
    let mut bytes = Vec::new();
    file.take((MAX_PACKAGE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("Could not read the simulation-plan package: {error}"))?;
    bounded_package_text(bytes)
}

/// The browser's chooser is asynchronous and user-gestured, so the click starts
/// it and a later frame collects it.
///
/// The handle lives in the context's own frame-to-frame store rather than in the
/// draft: the draft is the plan-manager's serialized state, shared by every
/// route, and a file handle in flight is neither state nor this route's to add
/// there. The handle is removed the moment its result is taken, so a second
/// chooser can be opened and a cancelled one leaves nothing behind.
#[cfg(target_arch = "wasm32")]
type ImportMailbox = std::sync::Arc<std::sync::Mutex<Option<Result<Option<String>, String>>>>;

#[cfg(target_arch = "wasm32")]
fn import_mailbox_id() -> egui::Id {
    egui::Id::new("simulation.plan-manager.import-package-mailbox")
}

#[cfg(target_arch = "wasm32")]
fn choose_import_package(ctx: &egui::Context, _draft: &mut SimulationPlanManagerDraft) {
    let id = import_mailbox_id();
    if ctx
        .data(|data| data.get_temp::<ImportMailbox>(id))
        .is_some()
    {
        return;
    }
    let mailbox: ImportMailbox = std::sync::Arc::new(std::sync::Mutex::new(None));
    ctx.data_mut(|data| data.insert_temp(id, mailbox.clone()));
    let repaint = ctx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let picked = rfd::AsyncFileDialog::new()
            .add_filter(
                PACKAGE_FILTER_LABEL,
                &[PACKAGE_EXTENSION, PACKAGE_JSON_EXTENSION],
            )
            .pick_file()
            .await;
        let result = match picked {
            Some(file) => bounded_package_text(file.read().await).map(Some),
            None => Ok(None),
        };
        if let Ok(mut slot) = mailbox.lock() {
            *slot = Some(result);
        }
        repaint.request_repaint();
    });
}

/// Take the browser chooser's result, if it has produced one.
#[cfg(target_arch = "wasm32")]
fn poll_chosen_package(ctx: &egui::Context, draft: &mut SimulationPlanManagerDraft) {
    let id = import_mailbox_id();
    let Some(mailbox) = ctx.data(|data| data.get_temp::<ImportMailbox>(id)) else {
        return;
    };
    let Some(result) = mailbox.lock().ok().and_then(|mut slot| slot.take()) else {
        return;
    };
    ctx.data_mut(|data| data.remove::<ImportMailbox>(id));
    match result {
        Ok(Some(source)) => {
            draft.exchange_text = source;
            draft.validation_error = None;
        }
        Ok(None) => {}
        Err(error) => draft.validation_error = Some(error),
    }
}

/// The desktop chooser blocks, so there is never a result in flight to collect.
#[cfg(not(target_arch = "wasm32"))]
fn poll_chosen_package(_ctx: &egui::Context, _draft: &mut SimulationPlanManagerDraft) {}

pub(in crate::workbench::surfaces::simulate) fn export_simulation_plan_package(
    app: &RSpiceApp,
    id: SimulationPlanId,
) -> Result<String, String> {
    let plan = app
        .state
        .sim_setup
        .export_plan(id)
        .map_err(|error| error.to_string())?;
    let payload = app
        .state
        .workspace
        .plan_data(id)
        .cloned()
        .ok_or_else(|| format!("Simulation plan {id} has no plan-owned payload to export."))?;
    serde_json::to_string_pretty(&PortableSimulationPlanPackage {
        format: SIMULATION_PLAN_PACKAGE_FORMAT.to_owned(),
        version: SIMULATION_PLAN_PACKAGE_VERSION,
        plan,
        payload,
    })
    .map_err(|error| format!("Could not serialize simulation plan {id}: {error}"))
}

pub(in crate::workbench::surfaces::simulate) fn commit_import_simulation_plan(
    app: &mut RSpiceApp,
    json: &str,
    name: &str,
) -> Result<(SimulationPlanId, String), String> {
    if json.trim().is_empty() {
        return Err(
            "Choose a portable RSpice simulation-plan package before importing.".to_owned(),
        );
    }
    let mut package = inspect_package(json)?;
    package.plan.name = crate::workbench::app_state::SimulationPlanName::new(name.to_owned())
        .map_err(|error| error.to_string())?;
    app.state
        .model_library_manager
        .validate_simulation_plan_bindings(&package.plan.model_bindings)
        .map_err(|error| format!("Imported model bindings require review: {error}"))?;

    let mut setup = app.state.sim_setup.clone();
    let mut workspace = app.state.workspace.clone();
    let current_id = setup.stable_analysis_plan()?.id();
    workspace.migrate_active_plan_data(current_id);
    let outcome = setup
        .import_plan(package.plan)
        .map_err(|error| error.to_string())?;
    workspace
        .import_plan_data(
            outcome.cloned_plan_id,
            &package.payload,
            &outcome.analysis_identity_map,
        )
        .map_err(|error| error.to_string())?;
    workspace
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    crate::io::ProjectExecutionContext::from_state(
        workspace.project.id(),
        &setup,
        &app.state.model_library_manager,
    )?;

    let first_instance = setup
        .stable_analysis_plan()?
        .instances()
        .first()
        .map(|instance| instance.id());
    let id = outcome.cloned_plan_id;
    let imported_name = setup.active_plan_name().to_string();
    app.state.sim_setup = setup;
    app.state.workspace = workspace;
    app.state.workbench.active_analysis_instance = first_instance;
    app.invalidate_simulation_preflight();
    Ok((
        id,
        format!(
            "Imported and activated '{imported_name}' as new local identity {id}; source lineage was retained and result references were not copied."
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A real export of a real plan, which is the only package these tests use.
    /// A hand-authored JSON fixture would drift from the wire form the moment a
    /// field moved, and would prove nothing about the format that ships.
    fn exported_package() -> String {
        let app = RSpiceApp::test_instance();
        let id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        export_simulation_plan_package(&app, id).expect("the fixture's plan exports")
    }

    /// An unknown field is refused rather than dropped. This is the behaviour
    /// the export's third note claims, and it is `deny_unknown_fields` doing it:
    /// without that attribute serde would ignore the field and import a package
    /// whose extra content silently went missing.
    #[test]
    fn a_package_carrying_an_unknown_field_is_refused_by_name() {
        let json = exported_package();
        let mut value: serde_json::Value = serde_json::from_str(&json).expect("the export is JSON");
        value["governance_authority"] = serde_json::Value::String("release".to_owned());
        let doctored = serde_json::to_string(&value).expect("the doctored package serializes");

        let error = inspect_package(&doctored).expect_err("an unknown field is refused");
        assert!(
            error.contains("governance_authority"),
            "the refusal has to name the field it did not know: {error}"
        );
        assert!(
            inspect_package(&json).is_ok(),
            "the same package without the extra field is accepted"
        );
    }

    /// The tag and the version are the whole compatibility contract, and both
    /// halves of it are enforced.
    #[test]
    fn the_envelope_admits_only_this_format_and_this_version() {
        let json = exported_package();
        let mut value: serde_json::Value = serde_json::from_str(&json).expect("the export is JSON");
        assert_eq!(value["format"], SIMULATION_PLAN_PACKAGE_FORMAT);
        assert_eq!(value["version"], SIMULATION_PLAN_PACKAGE_VERSION);

        value["version"] = serde_json::json!(SIMULATION_PLAN_PACKAGE_VERSION + 1);
        let future = serde_json::to_string(&value).expect("the future package serializes");
        assert!(
            inspect_package(&future)
                .expect_err("a later version is refused")
                .contains("version"),
            "a version refusal has to say so"
        );

        value["version"] = serde_json::json!(SIMULATION_PLAN_PACKAGE_VERSION);
        value["format"] = serde_json::Value::String("rspice.result-document".to_owned());
        let foreign = serde_json::to_string(&value).expect("the foreign package serializes");
        assert!(
            inspect_package(&foreign)
                .expect_err("a foreign format is refused")
                .contains("rspice.result-document"),
            "a format refusal has to name the format it was handed"
        );
    }

    /// The preview and the commit refuse the same package with the same
    /// sentence, because they are one function. A second envelope check would
    /// let the dialog show a package as sound that the commit then rejects.
    #[test]
    fn the_previewed_refusal_is_the_committed_refusal() {
        let mut app = RSpiceApp::test_instance();
        let mut value: serde_json::Value =
            serde_json::from_str(&exported_package()).expect("the export is JSON");
        value["version"] = serde_json::json!(SIMULATION_PLAN_PACKAGE_VERSION + 1);
        let future = serde_json::to_string(&value).expect("the future package serializes");

        let previewed = inspect_package(&future).expect_err("the preview refuses it");
        let committed = commit_import_simulation_plan(&mut app, &future, "Imported plan")
            .expect_err("the commit refuses it");
        assert_eq!(previewed, committed);
    }

    /// An empty draft is refused with the instruction that matches the route:
    /// the reader chooses a file or pastes one, and no longer types into a
    /// field, so a message telling them to paste one would name a control that
    /// is not there.
    #[test]
    fn an_empty_draft_is_refused_before_anything_is_parsed() {
        let mut app = RSpiceApp::test_instance();
        let error = commit_import_simulation_plan(&mut app, "   ", "Imported plan")
            .expect_err("an empty draft is refused");
        assert!(error.contains("Choose"), "{error}");
    }

    /// The proposed file name is derived from the plan and is safe to hand a
    /// file system: one extension, no separators, and bounded.
    #[test]
    fn the_proposed_file_name_is_derived_bounded_and_portable() {
        let app = RSpiceApp::test_instance();
        let records = plan_catalog_records(&app);
        let record = records.first().expect("the fixture projects a plan");
        let derived = package_file_name(Some(record));
        assert!(
            derived.ends_with(PACKAGE_EXTENSION),
            "the proposal carries the package extension: {derived}"
        );
        assert!(
            derived
                .chars()
                .all(|character| character.is_ascii_alphanumeric()
                    || character == '-'
                    || character == '.'),
            "a proposed name may only use characters every file system accepts: {derived}"
        );
        assert!(derived.len() <= PACKAGE_STEM_LIMIT + PACKAGE_EXTENSION.len() + 1);

        // No plan selected, and a name with nothing usable in it, both fall back
        // rather than proposing a file called ".rspice-plan".
        assert_eq!(package_file_name(None), "simulation-plan.rspice-plan");
    }

    /// The number the dialog previews is the number the import creates.
    ///
    /// Checked against the real transaction rather than against
    /// `specification_count`'s own arithmetic, because the claim is not that the
    /// helper adds up — it is that it mirrors the migration
    /// `Workspace::import_plan_data` performs. A payload carrying only the
    /// legacy list becomes that many governed definitions, and a count taken off
    /// the governed field alone would have previewed zero of them.
    #[test]
    fn the_previewed_specification_count_is_what_the_import_creates() {
        let mut app = RSpiceApp::test_instance();
        app.state.provision_test_project_technology_contract();
        let id = app
            .state
            .sim_setup
            .stable_analysis_plan()
            .expect("the fixture has a stable plan")
            .id();
        let json = export_simulation_plan_package(&app, id).expect("the fixture's plan exports");
        let previewed = specification_count(
            &inspect_package(&json)
                .expect("its own export is accepted")
                .payload,
        );

        let (imported_id, _) =
            commit_import_simulation_plan(&mut app, &json, "Imported specification count")
                .expect("the fixture's own package imports");
        let owned = app
            .state
            .workspace
            .plan_data(imported_id)
            .expect("the imported plan owns a payload")
            .specification_definitions
            .len();
        assert_eq!(
            previewed, owned,
            "the import created {owned} specification(s) where the dialog \
             previewed {previewed}"
        );
    }

    /// The envelope line is one owner, so the export's package row and the
    /// import's envelope row cannot come to state different contracts.
    #[test]
    fn the_envelope_label_states_the_format_and_the_version() {
        let label = envelope_label(exported_package().len());
        assert!(label.contains(SIMULATION_PLAN_PACKAGE_FORMAT), "{label}");
        assert!(
            label.contains(&SIMULATION_PLAN_PACKAGE_VERSION.to_string()),
            "{label}"
        );
    }
}

//! The Stimulus Library's verbs.
//!
//! Every one of them acts on the project's library and the workspace's open
//! drafts, and nothing else; none of them takes the application. They are here
//! rather than inside the stage because the same verbs are reached from the
//! toolbar, the command palette and a keyboard chord, and a verb that only
//! existed inside a painted button would be a verb the palette could not find.
//!
//! Editing is always through [`DefinitionDraft`]: the working record changes,
//! the history gets one entry, and the library document is untouched until
//! Apply publishes. That is what makes Ctrl+Z on this workspace mean "step my
//! edit back" rather than "undo whatever the application did last".

use std::collections::HashMap;

use crate::diagnostics::ConsoleMessage;
use crate::state::stimulus_library::definition::{
    RetainedPwlFile, StimulusDefinition, StimulusFamily, StimulusKind,
};
use crate::state::stimulus_library::draft::DefinitionDraft;
use crate::state::stimulus_library::provenance::ProvenanceState;
use crate::ui::accessibility::counted;
use crate::workbench::app_state::AppState;
use crate::workbench::state::write_field;

/// The family a new definition opens on.
///
/// A pulse, because it is the shape with the most authored fields and the one
/// whose defaults produce a visible waveform immediately; the identity band
/// retargets family and kind in one click.
const NEW_DEFINITION_FAMILY: crate::state::ComponentType =
    crate::state::ComponentType::VoltageSourcePulse;

/// Create a definition, select it, and leave the reader in its name field.
pub(crate) fn new_definition(state: &mut AppState) {
    match state
        .workspace
        .stimulus_library
        .new_definition(NEW_DEFINITION_FAMILY)
    {
        Ok(definition) => {
            let name = definition.name().to_owned();
            state.workbench.selected_stimulus_definition = Some(name.clone());
            state.workbench.stimulus_editor.focus_name();
            state.push_user_message(ConsoleMessage::info(format!(
                "Stimulus definition '{name}' created at r1"
            )));
        }
        Err(error) => state.push_user_message(ConsoleMessage::warning(error.to_string())),
    }
}

/// Copy the selected definition into an editable duplicate.
pub(crate) fn duplicate_definition(state: &mut AppState) {
    let Some(name) = selected(state) else {
        return;
    };
    match state.workspace.stimulus_library.duplicate(&name) {
        Ok(copy) => {
            let copy = copy.name().to_owned();
            state.workbench.selected_stimulus_definition = Some(copy.clone());
            state.push_user_message(ConsoleMessage::info(format!(
                "Stimulus definition '{name}' duplicated as '{copy}' at r1"
            )));
        }
        Err(error) => state.push_user_message(ConsoleMessage::warning(error.to_string())),
    }
}

/// Remove the selected definition. Adopters keep the cards they copied.
pub(crate) fn delete_definition(state: &mut AppState) {
    let Some(name) = selected(state) else {
        return;
    };
    let adopters = design_adopters(state, &name).len();
    if state.workspace.stimulus_library.delete(&name).is_none() {
        return;
    }
    state.workbench.stimulus_editor.close(&name);
    state.workbench.selected_stimulus_definition = state
        .workspace
        .stimulus_library
        .definitions()
        .first()
        .map(|definition| definition.name().to_owned());
    state.push_user_message(ConsoleMessage::info(if adopters == 0 {
        format!("Stimulus definition '{name}' deleted")
    } else {
        format!(
            "Stimulus definition '{name}' deleted; {} and now {} 'definition removed'",
            counted(
                adopters,
                "placed source keeps its card",
                "placed sources keep their cards"
            ),
            if adopters == 1 { "reads" } else { "read" }
        )
    }));
}

/// Whether deleting the selection needs to be confirmed first.
///
/// Two things make a delete worth a question: work that has not been applied,
/// and instances that will start reading "definition removed" because of it.
#[must_use]
pub(crate) fn delete_needs_confirmation(state: &AppState) -> Option<String> {
    let name = state.workbench.selected_stimulus_definition.as_deref()?;
    let adopters = design_adopters(state, name).len();
    let dirty = state.workbench.stimulus_editor.is_dirty(name);
    match (adopters, dirty) {
        (0, false) => None,
        (0, true) => Some(format!(
            "'{name}' has a draft that has not been applied. Deleting the definition discards it."
        )),
        (count, false) => Some(format!(
            "{} adopted '{name}'. {} and will read 'definition removed'.",
            counted(count, "placed source", "placed sources"),
            if count == 1 {
                "It keeps its card"
            } else {
                "They keep their cards"
            }
        )),
        (count, true) => Some(format!(
            "'{name}' has a draft that has not been applied, and {} adopted it. The draft is \
             discarded; {} and will read 'definition removed'.",
            counted(count, "placed source", "placed sources"),
            if count == 1 {
                "the instance keeps its card"
            } else {
                "the instances keep their cards"
            }
        )),
    }
}

/// Publish the draft as the next revision.
///
/// A rename is published here too, as a rename of the library's own record.
/// Nothing reaches into the design to repoint adopters: an instance that
/// adopted before the rename is on whatever sheet it is on, and the definition
/// keeps answering to the name that instance copied.
pub(crate) fn apply_draft(state: &mut AppState) {
    let Some(name) = selected(state) else {
        return;
    };
    let Some(saved) = state.workspace.stimulus_library.get(&name).cloned() else {
        return;
    };
    let draft = state.workbench.stimulus_editor.draft_for(&saved);
    if !draft.is_dirty() {
        return;
    }
    let renamed = draft.working().name().to_owned();
    let mut published = draft.clone();
    if !renamed.eq_ignore_ascii_case(&name)
        && let Err(error) = state.workspace.stimulus_library.rename(&name, &renamed)
    {
        state.push_user_message(ConsoleMessage::warning(error.to_string()));
        return;
    }
    let revision = state.workspace.stimulus_library.apply(&mut published);
    if !renamed.eq_ignore_ascii_case(&name) {
        state.workbench.stimulus_editor.rename_key(&name, &renamed);
    }
    // The draft the editor holds is the one this publish came from, so it is
    // still the pre-publish record. Replacing it keeps the reader's editing
    // session open on the revision they just published rather than silently
    // reopening it on the next frame.
    if let Some(record) = state.workspace.stimulus_library.get(&renamed).cloned() {
        *state.workbench.stimulus_editor.draft_for(&record) = published;
    }
    state.workbench.selected_stimulus_definition = Some(renamed.clone());
    let behind = design_adopters(state, &renamed).len();
    state.push_user_message(ConsoleMessage::info(if behind == 0 {
        format!("Stimulus definition '{renamed}' published at r{revision}; no adopter")
    } else {
        format!(
            "Stimulus definition '{renamed}' published at r{revision}; {} behind until re-adopted",
            counted(behind, "placed source now reads", "placed sources now read")
        )
    }));
}

/// Throw the draft away, as one undoable step.
pub(crate) fn revert_draft(state: &mut AppState) {
    with_draft(state, DefinitionDraft::revert);
}

/// Step the selected definition's draft back, if it has one.
pub(crate) fn undo_draft(state: &mut AppState) -> bool {
    let mut stepped = false;
    with_draft(state, |draft| stepped = draft.undo());
    stepped
}

/// Step the selected definition's draft forward, if it has one.
pub(crate) fn redo_draft(state: &mut AppState) -> bool {
    let mut stepped = false;
    with_draft(state, |draft| stepped = draft.redo());
    stepped
}

/// Rename the working record.
pub(crate) fn edit_name(state: &mut AppState, name: &str) {
    with_draft(state, |draft| {
        draft.edit(|working| {
            let _ = working.rename(name.trim());
        });
    });
}

/// Switch the waveform shape. The shape parameters reset; undo restores them.
pub(crate) fn edit_family(state: &mut AppState, family: StimulusFamily) {
    with_draft(state, |draft| {
        draft.edit(|working| *working = working.with_family(family));
    });
}

/// Switch the driven quantity, keeping the card.
pub(crate) fn edit_kind(state: &mut AppState, kind: StimulusKind) {
    with_draft(state, |draft| {
        draft.edit(|working| *working = working.with_kind(kind));
    });
}

/// Write one waveform field, in the instance's own spelling.
pub(crate) fn edit_field(state: &mut AppState, field: &str, value: &str) {
    with_draft(state, |draft| {
        draft.edit(|working| write_field(working, field, value));
    });
}

/// Import a data file and retain its bytes in the definition.
///
/// The card names the file and the project keeps a copy of it, which is what
/// lets a project that has been zipped and mailed still describe the same run.
pub(crate) fn import_data_file(state: &mut AppState) {
    let data_root = state
        .workspace
        .project
        .data_root()
        .map(std::path::Path::to_path_buf);
    let picked = crate::properties::tabbed_dialog::attach_data_file(data_root.as_deref());
    let reference = match picked {
        Ok(Some(reference)) => reference,
        Ok(None) => return,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(error));
            return;
        }
    };
    let absolute = match data_root.as_deref() {
        Some(root) if !std::path::Path::new(&reference).is_absolute() => root.join(&reference),
        _ => std::path::PathBuf::from(&reference),
    };
    let bytes = match std::fs::read(&absolute) {
        Ok(bytes) => bytes,
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Cannot read '{}': {error}",
                absolute.display()
            )));
            return;
        }
    };
    let name = absolute
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| reference.clone());
    let retained = match retained_table(
        &name,
        bytes,
        u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX),
    ) {
        Ok(retained) => retained,
        Err(refusal) => {
            state.push_user_message(ConsoleMessage::warning(refusal));
            return;
        }
    };
    let retained_bytes = retained.contents.len();
    with_draft(state, |draft| {
        draft.edit(|working| {
            write_field(working, "file", &reference);
            working.pwl_file = Some(retained.clone());
        });
    });
    state.push_user_message(ConsoleMessage::info(format!(
        "Retained '{name}' ({retained_bytes} B) in the stimulus definition"
    )));
}

/// One picked file as a definition's retained table, or why it is not one.
///
/// The picker offers CSV and WAV, because the engine's `PWL FILE` loader reads
/// both — but a definition retains the *text* of a table, and a WAV is samples
/// in a binary container. Reading one as text surfaces as an operating-system
/// decoding error naming a byte offset, which tells the reader nothing about
/// what they picked, so the refusal is stated here in the product's own terms
/// and the draft is left exactly as it was.
pub(crate) fn retained_table(
    file_name: &str,
    bytes: Vec<u8>,
    imported_at_unix_ms: u64,
) -> Result<RetainedPwlFile, String> {
    let contents = String::from_utf8(bytes).map_err(|_| {
        format!(
            "'{file_name}' is not a text PWL table; a PWL FILE definition retains time/value text"
        )
    })?;
    Ok(RetainedPwlFile::new(
        file_name,
        contents,
        imported_at_unix_ms,
    ))
}

/// Copy the library's current revision of `definition` onto one adopter, and
/// say what happened where the reader is looking.
///
/// The transaction is [`crate::workbench::app::commit_readoption`] — the one
/// the link dialog, the Studio's Excitations rows and the component editor all
/// commit through — and this adds only the console sentence it hands back. The
/// instrument used to own a second re-adoption with its own undo group and its
/// own wording, so the same verb on two surfaces produced two different
/// receipts for one edit.
pub(crate) fn readopt_adopter(state: &mut AppState, component_id: u64, definition: &str) {
    let outcome = crate::workbench::app::commit_readoption(state, component_id, definition);
    state.push_user_message(match outcome {
        Ok(line) => ConsoleMessage::info(line),
        Err(refusal) => ConsoleMessage::warning(refusal),
    });
}

/// Arm the next placement with the selected definition's saved revision.
///
/// A definition with an unapplied draft is armed anyway, and the console says
/// which revision the pointer is carrying. Refusing would be the wrong trade:
/// the reader asked to place the definition, the library's published revision
/// is the only thing anything can adopt, and a verb that declines rather than
/// stating what it did sends them looking for a setting.
pub(crate) fn place_selected_definition(state: &mut AppState) {
    let Some(name) = selected(state) else {
        return;
    };
    let saved = state
        .workspace
        .stimulus_library
        .get(&name)
        .map_or(0, StimulusDefinition::revision);
    if state.workbench.stimulus_editor.is_dirty(&name) {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Placement copies the saved r{saved} of '{name}'; apply the draft first to place what \
             you are editing"
        )));
    }
    if let Err(refusal) = crate::workbench::app::arm_placement_from_definition(state, &name) {
        state.push_user_message(ConsoleMessage::warning(refusal));
    }
}

/// Audit every definition in the library and report the totals.
pub(crate) fn validate_library(state: &mut AppState) {
    let definitions = state.workspace.stimulus_library.definitions().to_vec();
    let mut errors = 0_usize;
    let mut advisories = 0_usize;
    for definition in &definitions {
        let record = state
            .workbench
            .stimulus_editor
            .draft(definition.name())
            .map_or_else(|| definition.clone(), |draft| draft.working().clone());
        if record.card_text(["p", "n"]).is_err() {
            errors += 1;
        }
        for finding in contract_findings(state, &record) {
            if finding.strength == crate::state::ContractStrength::Refusal {
                errors += 1;
            } else {
                advisories += 1;
            }
        }
    }
    state.push_user_message(ConsoleMessage::info(format!(
        "Stimulus library checked: {}, {}, {}",
        counted(definitions.len(), "definition", "definitions"),
        counted(errors, "error", "errors"),
        counted(advisories, "advisory", "advisories")
    )));
}

/// What the engine contract says about one record.
///
/// The audit strip and the library-wide check read the same function, so the
/// tally a console line reports cannot disagree with the verdict the
/// instrument is showing.
pub(crate) fn contract_findings(
    state: &AppState,
    record: &StimulusDefinition,
) -> Vec<crate::state::SourceContractFinding> {
    let component = record.transient_component();
    let Some(sheet) = state.property_registry.get(component.kind) else {
        return Vec::new();
    };
    let values = crate::properties::property_bridge::collect_properties_from_component(
        &component,
        &state.property_registry,
    );
    crate::properties::property_bridge::component_source_contract(&component, &values, sheet)
}

/// Every placed source in the whole design that adopted one definition.
///
/// The design rather than the open sheet, because that is what the run builds:
/// a definition adopted inside a child master is a real adopter, and a
/// definition two instances of one master reach is adopted twice. The walk is
/// [`crate::simulation::placed_sources::whole_design_excitations`] — the same
/// reading the Studio's Excitations page lists — so the instrument's count, the
/// inspector's cards and that ledger cannot disagree about who is carrying a
/// copy.
///
/// Matching is by what the library resolves the receipt to, never by comparing
/// the receipt's spelling with the browser's: a definition that has been
/// renamed still answers to the name its adopters copied, and a hand-written
/// comparison would report every one of them as an adopter of nothing.
#[must_use]
pub(crate) fn design_adopters(state: &AppState, name: &str) -> Vec<StimulusAdopter> {
    let library = &state.workspace.stimulus_library;
    let Some(held) = library.get(name).map(StimulusDefinition::name) else {
        return Vec::new();
    };
    let (sources, _) = crate::simulation::placed_sources::whole_design_excitations(
        &state.library_manager,
        &state.workspace,
        &state.schematic,
        state.sim_setup.analysis_plan.as_ref(),
    );
    sources
        .into_iter()
        .filter(|source| {
            source
                .definition
                .as_deref()
                .and_then(|adopted| library.get(adopted))
                .is_some_and(|resolved| resolved.name() == held)
        })
        .map(|source| StimulusAdopter {
            on_this_sheet: crate::workbench::app::actions::reveal::reaches(
                state,
                source.occurrence.as_ref(),
            ),
            occurrence: source.occurrence_label(),
            chip: source.provenance.label(),
            offers_readoption: source.provenance.offers_readoption(),
            behind: matches!(
                source.provenance,
                ProvenanceState::Behind { .. } | ProvenanceState::ModifiedBehind { .. }
            ),
            modified: matches!(
                source.provenance,
                ProvenanceState::Modified { .. } | ProvenanceState::ModifiedBehind { .. }
            ),
            source,
        })
        .collect()
}

/// One placed source that adopted the definition a surface is showing.
///
/// It carries the walk's own row rather than a copy of its fields, so a surface
/// that needs the terminals, the key figure or the analyses reading it takes
/// them from the list the run was built from.
#[derive(Debug, Clone)]
pub(crate) struct StimulusAdopter {
    /// The row [`design_adopters`] resolved this from.
    pub source: crate::simulation::placed_sources::PlacedSource,
    /// Whether the buffer on screen is the one this instance is drawn in, and
    /// therefore whether the verbs that edit it can reach it at all.
    pub on_this_sheet: bool,
    /// The occurrence the run reaches it through, as a column states it.
    pub occurrence: String,
    /// The lifecycle chip, in the provenance vocabulary.
    pub chip: String,
    /// Whether re-adopting would change anything.
    pub offers_readoption: bool,
    /// Whether the library has published past this instance's copy.
    pub behind: bool,
    /// Whether the instance's card has been edited away from its copy.
    pub modified: bool,
}

/// How many placed sources carry each definition, and how many of those the
/// library has published past.
///
/// One walk for the whole library rather than [`design_adopters`] per row: a
/// browser listing fourteen definitions would otherwise resolve the design
/// fourteen times for two numbers per row.
#[must_use]
pub(crate) fn design_adopter_tally(state: &AppState) -> HashMap<String, AdopterTally> {
    let library = &state.workspace.stimulus_library;
    let (sources, _) = crate::simulation::placed_sources::whole_design_excitations(
        &state.library_manager,
        &state.workspace,
        &state.schematic,
        state.sim_setup.analysis_plan.as_ref(),
    );
    let mut tally: HashMap<String, AdopterTally> = HashMap::new();
    for source in &sources {
        let Some(held) = source
            .definition
            .as_deref()
            .and_then(|adopted| library.get(adopted))
        else {
            continue;
        };
        let entry = tally.entry(held.name().to_owned()).or_default();
        entry.adopters += 1;
        entry.references.push(source.reference.clone());
        if matches!(
            source.provenance,
            ProvenanceState::Behind { .. } | ProvenanceState::ModifiedBehind { .. }
        ) {
            entry.behind += 1;
        }
    }
    tally
}

/// What one definition's adopters amount to, for a row that has space for two
/// numbers and a filter that searches for a third thing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct AdopterTally {
    /// How many placed sources carry a copy of this definition.
    pub adopters: usize,
    /// How many of them the library has published past.
    pub behind: usize,
    /// The deck spelling of each adopter, so a browser can answer a filter
    /// naming an instance without walking the design once per row.
    pub references: Vec<String>,
}

/// Whether anything in the design has adopted the selected definition.
#[must_use]
pub(crate) fn selected_definition_has_adopter(state: &AppState) -> bool {
    state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .is_some_and(|name| !design_adopters(state, name).is_empty())
}

/// Show the first adopter of the selected definition on the drawing.
pub(crate) fn show_adopter_on_schematic(state: &mut AppState) {
    let Some(name) = selected(state) else {
        return;
    };
    let Some(first) = design_adopters(state, &name).first().cloned() else {
        return;
    };
    crate::workbench::app::actions::reveal::placed_instance(
        state,
        first.source.occurrence.as_ref(),
        first.source.component_id,
    );
}

/// Whether the selected definition has an unapplied draft.
#[must_use]
pub(crate) fn selected_draft_is_dirty(state: &AppState) -> bool {
    state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .is_some_and(|name| state.workbench.stimulus_editor.is_dirty(name))
}

/// Run one edit against the selected definition's draft.
fn with_draft(state: &mut AppState, edit: impl FnOnce(&mut DefinitionDraft)) {
    let Some(name) = selected(state) else {
        return;
    };
    let Some(saved) = state.workspace.stimulus_library.get(&name).cloned() else {
        return;
    };
    edit(state.workbench.stimulus_editor.draft_for(&saved));
}

/// The definition the library browser is on, by the name the library spells.
fn selected(state: &AppState) -> Option<String> {
    let name = state.workbench.selected_stimulus_definition.as_deref()?;
    state
        .workspace
        .stimulus_library
        .get(name)
        .map(|definition| definition.name().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shared picker offers WAV as well as CSV, so a binary pick is a
    /// thing a reader can do. It is refused by what it is rather than by the
    /// byte offset a decoder happened to stop at.
    #[test]
    fn a_binary_pick_is_refused_as_not_a_text_table() {
        let refusal = retained_table("bridge_capture.wav", vec![0x52, 0x49, 0xff, 0xfe, 0x00], 7)
            .expect_err("a WAV is not a PWL table");
        assert_eq!(
            refusal,
            "'bridge_capture.wav' is not a text PWL table; a PWL FILE definition retains \
             time/value text"
        );
    }

    #[test]
    fn a_text_table_is_retained_with_its_own_bytes() {
        let retained = retained_table("step.csv", b"0 0\n1e-9 1\n".to_vec(), 7).expect("retained");
        assert_eq!(retained.file_name, "step.csv");
        assert_eq!(retained.contents, "0 0\n1e-9 1\n");
        assert_eq!(retained.imported_at_unix_ms, 7);
    }
}

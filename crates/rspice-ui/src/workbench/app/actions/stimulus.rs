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

use crate::diagnostics::ConsoleMessage;
use crate::state::stimulus_library::definition::{
    RetainedPwlFile, StimulusDefinition, StimulusFamily, StimulusKind,
};
use crate::state::stimulus_library::draft::DefinitionDraft;
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
    let adopters = state
        .workspace
        .stimulus_library
        .adopters(&state.schematic.components)
        .remove(&name)
        .map_or(0, |adopters| adopters.len());
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
            "Stimulus definition '{name}' deleted; {adopters} placed source(s) keep their cards \
             and now read 'definition removed'"
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
    let adopters = state
        .workspace
        .stimulus_library
        .adopters(&state.schematic.components)
        .remove(name)
        .map_or(0, |adopters| adopters.len());
    let dirty = state.workbench.stimulus_editor.is_dirty(name);
    match (adopters, dirty) {
        (0, false) => None,
        (0, true) => Some(format!(
            "'{name}' has a draft that has not been applied. Deleting the definition discards it."
        )),
        (count, false) => Some(format!(
            "{count} placed source(s) adopted '{name}'. They keep their cards and will read \
             'definition removed'."
        )),
        (count, true) => Some(format!(
            "'{name}' has a draft that has not been applied, and {count} placed source(s) adopted \
             it. The draft is discarded; the instances keep their cards and will read 'definition \
             removed'."
        )),
    }
}

/// Publish the draft as the next revision.
///
/// A rename is published here too: every placed source whose provenance names
/// the old definition is rewritten to the new name, because a rename is not a
/// deletion and an adopter that suddenly read "definition removed" because
/// someone corrected a spelling would be a lie about what it adopted.
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
        rewrite_adopters(state, &name, &renamed);
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
    let behind = state
        .workspace
        .stimulus_library
        .adopters(&state.schematic.components)
        .remove(&renamed)
        .map_or(0, |adopters| adopters.len());
    state.push_user_message(ConsoleMessage::info(if behind == 0 {
        format!("Stimulus definition '{renamed}' published at r{revision}; no adopter")
    } else {
        format!(
            "Stimulus definition '{renamed}' published at r{revision}; {behind} placed source(s) \
             now read behind until re-adopted"
        )
    }));
}

/// Point every adopter of `old` at `new`.
pub(crate) fn rewrite_adopters(state: &mut AppState, old: &str, new: &str) {
    for component in &mut state.schematic.components {
        if let Some(provenance) = component.stimulus_provenance.as_mut()
            && provenance.definition.eq_ignore_ascii_case(old)
        {
            provenance.definition = new.to_owned();
        }
    }
    state.sync_active_schematic_to_workspace();
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
    let contents = match std::fs::read_to_string(&absolute) {
        Ok(contents) => contents,
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
    let bytes = contents.len();
    let retained = RetainedPwlFile::new(
        name.clone(),
        contents,
        u64::try_from(crate::time_compat::unix_epoch().as_millis()).unwrap_or(u64::MAX),
    );
    with_draft(state, |draft| {
        draft.edit(|working| {
            write_field(working, "file", &reference);
            working.pwl_file = Some(retained.clone());
        });
    });
    state.push_user_message(ConsoleMessage::info(format!(
        "Retained '{name}' ({bytes} B) in the stimulus definition"
    )));
}

/// Copy the definition's current revision onto one placed adopter.
pub(crate) fn readopt_instance(state: &mut AppState, component_id: u64) {
    let Some(name) = selected(state) else {
        return;
    };
    let Some(definition) = state.workspace.stimulus_library.get(&name).cloned() else {
        return;
    };
    let revision = definition.revision();
    let mut outcome = None;
    state.schematic.with_undo("Re-adopt stimulus definition", |schematic| {
        if let Some(component) = schematic
            .components
            .iter_mut()
            .find(|component| component.id == component_id)
        {
            let instance = component.name.clone();
            outcome = Some(definition.readopt_onto(component).map(|()| instance));
        }
    });
    state.sync_active_schematic_to_workspace();
    match outcome {
        Some(Ok(instance)) => state.push_user_message(ConsoleMessage::info(format!(
            "{instance} re-adopted '{name}' at r{revision}"
        ))),
        Some(Err(error)) => state.push_user_message(ConsoleMessage::warning(error)),
        None => {}
    }
}

/// Open Component Properties on one placed adopter.
pub(crate) fn open_instance_properties(state: &mut AppState, component_id: u64) {
    state.schematic.selection.clear();
    state.schematic.selection.select_component(component_id);
    crate::workbench::app::actions::property_edit::open_selected_object_properties(state);
}

/// Audit every definition in the library and report the totals.
pub(crate) fn validate_library(state: &mut AppState) {
    let definitions = state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .cloned()
        .collect::<Vec<_>>();
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
        "Stimulus library checked: {} definition(s), {errors} error(s), {advisories} advisory(ies)",
        definitions.len()
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

/// The first placed source that adopted the selected definition.
#[must_use]
pub(crate) fn first_adopter(state: &AppState) -> Option<u64> {
    let name = state.workbench.selected_stimulus_definition.as_deref()?;
    state
        .workspace
        .stimulus_library
        .adopters(&state.schematic.components)
        .remove(name)?
        .first()
        .map(|component| component.id)
}

/// Select the first adopter on the sheet, so the design workspace opens on it.
pub(crate) fn show_adopter_on_schematic(state: &mut AppState) {
    let Some(id) = first_adopter(state) else {
        return;
    };
    state.schematic.selection.clear();
    state.schematic.selection.select_component(id);
    state.schematic.center_request = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.pos);
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

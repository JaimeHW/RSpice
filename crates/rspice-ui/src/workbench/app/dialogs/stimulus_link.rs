//! Linking a placed source to the project's stimulus library.
//!
//! Two directions, one transaction, one shell. Adopting copies a definition
//! onto the instance; saving publishes the instance's card as a definition and
//! points the instance at it. Both are one edit of one component against one
//! library, and both have to show the reader the same three things before they
//! commit — which instance, which card, and what the card becomes — so the
//! header, the pane split and the fact rows live in [`shell`] and are called
//! twice. The two bodies differ enough to be their own files: one is a library
//! to choose from, the other a record to author.
//!
//! Nothing here decides a lifecycle word or a refusal. `AdoptionFit`,
//! `kind_refusal`, `adopt_onto` and `extract_from` are the model's, the card
//! comes from the netlist generator through `stimulus_realize`, and every curve
//! is drawn by [`crate::properties::source_preview`] — the same painter the
//! component editor's evidence pane uses, because this dialog draws the same
//! source.
//!
//! **Adopting across families re-places the instance.** Family is waveform
//! shape and it is spelled as a `ComponentType`, so a `SIN` definition on a
//! `PULSE` source is a new component type, symbol and property sheet. It is
//! performed as the narrowest possible re-type — the component's `kind` is
//! changed and the definition copied onto it in one component transaction —
//! rather than through the replace-instance machinery, which resolves a
//! library master and maps a pin contract that these 24 types do not have.
//! Every independent source shares one terminal geometry
//! (`component_type::geometry`: `+` at `(0, -20)`, `-` at `(0, 20)`), so both
//! terminals stay exactly where they were and every attached conductor keeps
//! its net; undo restores the old type and the old card together because the
//! whole thing is one entry.

mod adopt;
mod extract;
mod shell;

use egui::Context;

use crate::simulation::stimulus_realize::{self, PreviewTiming};
use crate::state::Component;
use crate::state::stimulus_library::definition::{
    StimulusDefinition, StimulusDefinitionError, StimulusFamily,
};
use crate::state::stimulus_library::provenance::{AdoptionFit, ProvenanceState};
use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;

use shell::MiniCache;

/// Which direction the dialog is open in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum StimulusLinkMode {
    /// Copy a definition onto the instance.
    #[default]
    Adopt,
    /// Publish the instance's card as a definition.
    Extract,
}

/// One open link transaction over one placed source.
///
/// The instance's identity, card, nets and library standing are captured when
/// the dialog opens rather than resolved per frame: resolving the nets walks
/// the sheet, and the dialog is modal, so nothing can move underneath it while
/// it is up.
#[derive(Debug, Clone)]
pub(crate) struct StimulusLinkDialogState {
    pub(crate) open: bool,
    mode: StimulusLinkMode,
    component_id: u64,
    reference: String,
    chip: String,
    provenance: ProvenanceState,
    card: String,
    nets: [String; 2],
    timing: PreviewTiming,
    /// Adopt: the definition the reader has picked.
    pick: Option<String>,
    /// Adopt: what the reader has typed into the list filter.
    filter: String,
    /// Extract: the name and purpose being authored.
    name: String,
    purpose: String,
    /// Extract: whether the generated name is still the untouched suggestion
    /// and should be offered whole to the first keystroke.
    select_name: bool,
    /// The refusal the last commit attempt produced.
    error: Option<String>,
    /// Every definition's list mini, evaluated once each while this dialog is
    /// open. See [`adopt::ensure_minis`].
    minis: MiniCache,
}

impl Default for StimulusLinkDialogState {
    fn default() -> Self {
        Self {
            open: false,
            mode: StimulusLinkMode::default(),
            component_id: 0,
            reference: String::new(),
            chip: String::new(),
            provenance: ProvenanceState::FromSchematic,
            card: String::new(),
            nets: [String::new(), String::new()],
            timing: PreviewTiming::default(),
            pick: None,
            filter: String::new(),
            name: String::new(),
            purpose: String::new(),
            select_name: false,
            error: None,
            minis: MiniCache::new(),
        }
    }
}

impl StimulusLinkDialogState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

/// Whether one instance can be linked at all: an editable independent source.
fn stimulus_link_target(state: &AppState, component_id: u64) -> Option<&Component> {
    if state.schematic_edit_read_only() {
        return None;
    }
    state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .filter(|component| stimulus_realize::is_independent_source(component.kind))
}

/// Open the dialog over one placed source.
///
/// Returns the sentence to show the reader when it refuses, so the surface
/// that asked states it where they are looking.
pub(crate) fn open_stimulus_link(
    state: &mut AppState,
    component_id: u64,
    mode: StimulusLinkMode,
) -> Result<(), String> {
    let Some(component) = stimulus_link_target(state, component_id) else {
        return Err(
            "Select one independent source on an editable sheet to link it to the stimulus \
             library."
                .to_owned(),
        );
    };
    if mode == StimulusLinkMode::Adopt && state.workspace.stimulus_library.is_empty() {
        return Err(
            "This project holds no stimulus definitions yet. Save this source as one first, or \
             author one in the Stimulus Library workspace."
                .to_owned(),
        );
    }
    let component = component.clone();
    let nets = instance_nets(state, component_id);
    let card = stimulus_realize::source_card_text(&component, [nets[0].as_str(), nets[1].as_str()])
        .unwrap_or_else(|errors| errors.join("; "));
    let reference = component.spice_instance_name();
    let name = format!(
        "{}_{}",
        reference.to_lowercase(),
        family_stem(&component).unwrap_or("stimulus")
    );
    // The definition the instance already adopted, when the library still
    // holds it; otherwise the first one whose card this instance already
    // carries, which is what a reader saving and re-adopting almost always
    // means.
    let pick = component
        .stimulus_provenance
        .as_ref()
        .map(|provenance| provenance.definition.clone())
        .filter(|name| state.workspace.stimulus_library.get(name).is_some())
        .or_else(|| first_equal_definition(state, &component));
    let provenance = state
        .workspace
        .stimulus_library
        .provenance_state(&component);

    state.dialogs.stimulus_link = StimulusLinkDialogState {
        open: true,
        mode,
        component_id,
        chip: provenance.label(),
        provenance,
        reference,
        card,
        nets,
        timing: crate::workbench::app::actions::property_edit::stimulus_preview_timing(state),
        pick,
        name,
        select_name: true,
        ..StimulusLinkDialogState::default()
    };
    Ok(())
}

/// The definition an instance's own card already equals, if the library holds
/// one. It is the pick a reader almost always wants when nothing was adopted.
fn first_equal_definition(state: &AppState, component: &Component) -> Option<String> {
    state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .find(|definition| {
            use crate::state::stimulus_library::definition::normalize_params;

            definition.adoption_fit(component) == AdoptionFit::Same
                && definition.value.trim() == component.value.trim()
                && normalize_params(&definition.params) == normalize_params(&component.params)
        })
        .map(|definition| definition.name().to_owned())
}

/// The family's name fragment, taken from the keyword the card carries so the
/// generated name and the card cannot disagree about what this source is.
fn family_stem(component: &Component) -> Option<&'static str> {
    Some(match StimulusFamily::of(component.kind)? {
        StimulusFamily::Dc => "dc",
        StimulusFamily::Ac => "ac",
        StimulusFamily::Pulse => "pulse",
        StimulusFamily::Sin => "sin",
        StimulusFamily::Pwl => "pwl",
        StimulusFamily::PwlFile => "pwl_file",
        StimulusFamily::Exp => "exp",
        StimulusFamily::Sffm => "sffm",
        StimulusFamily::Am => "am",
        StimulusFamily::Pat => "pat",
        StimulusFamily::Trnoise => "trnoise",
        StimulusFamily::Trrandom => "trrandom",
    })
}

/// The two nets this instance sits across, as the deck spells them.
///
/// Read through the same walk the Excitations page reads, so the card in the
/// dialog header is the card in the deck. A source whose terminals are not
/// both wired has no net names to show, and the detached pair the preview
/// bridge uses stands in — the card's shape is what the reader is here for.
fn instance_nets(state: &AppState, component_id: u64) -> [String; 2] {
    let detached = || {
        [
            stimulus_realize::DETACHED_NETS[0].to_owned(),
            stimulus_realize::DETACHED_NETS[1].to_owned(),
        ]
    };
    let sources = crate::simulation::placed_sources::placed_sources(
        &state.schematic,
        &state.workspace.stimulus_library,
        None,
    );
    let Some(source) = sources
        .iter()
        .find(|source| source.component_id == component_id)
    else {
        return detached();
    };
    match source.nets.as_slice() {
        [positive, negative, ..] if !positive.is_empty() && !negative.is_empty() => {
            [positive.clone(), negative.clone()]
        }
        _ => detached(),
    }
}

/// The instance as it would be once `definition` is on it, including the
/// re-type a family change performs. `None` when the definition cannot be
/// adopted onto it at all.
fn realized(component: &Component, definition: &StimulusDefinition) -> Option<Component> {
    if definition.adoption_fit(component) == AdoptionFit::Kind {
        return None;
    }
    let mut candidate = component.clone();
    candidate.kind = definition.component_type();
    definition.adopt_onto(&mut candidate).ok()?;
    Some(candidate)
}

/// What confirming would do, in the one line a footer has for it.
///
/// Deliberately shorter than `AdoptionFit::replace_warning`, which is the
/// model's full second-level warning and is what `readopt_onto` refuses with,
/// where there is a console line's worth of room. A footer note sits beside two
/// buttons: a sentence that has to elide to fit is a sentence a reader does not
/// finish, so this one names the instance, the verb, and the way back.
fn consequence(component: &Component, definition: &StimulusDefinition) -> String {
    let reference = component.spice_instance_name();
    match definition.adoption_fit(component) {
        AdoptionFit::Same => format!(
            "Copies {} r{} onto {reference}. Later edits in Component Properties read modified.",
            definition.name(),
            definition.revision(),
        ),
        AdoptionFit::Replace { from, to } => {
            let from = StimulusFamily::of(from).map_or("this", StimulusFamily::label);
            let to = StimulusFamily::of(to).map_or("that", StimulusFamily::label);
            format!(
                "Re-places {reference} as a {to} source. One undo restores the {from} instance."
            )
        }
        AdoptionFit::Kind => definition.kind_refusal(component),
    }
}

/// Why this name cannot be saved, in the model's own words.
fn extract_refusal(state: &AppState, component: &Component, name: &str) -> Option<String> {
    if let Err(error) = StimulusDefinition::new(name, component.kind) {
        return Some(error.to_string());
    }
    state
        .workspace
        .stimulus_library
        .get(name)
        .map(|held| StimulusDefinitionError::DuplicateName(held.name().to_owned()).to_string())
}

/// `PULSE · voltage` — the two facts a definition's identity is made of.
fn family_line(component: &Component) -> String {
    match (
        StimulusFamily::of(component.kind),
        crate::state::stimulus_library::definition::StimulusKind::of(component.kind),
    ) {
        (Some(family), Some(kind)) => format!("{} \u{00b7} {}", family.label(), kind.word()),
        _ => component.kind.display_name().to_owned(),
    }
}

/// Render the open link transaction.
///
/// The minis are lifted out of the session for the duration of the render so
/// the per-frame clone of the session does not copy every evaluated curve, and
/// are only put back while the dialog is still open: a closed dialog holds no
/// samples for a library it is no longer showing.
pub(crate) fn render_stimulus_link_dialog(ctx: &Context, state: &mut AppState) {
    if !state.dialogs.stimulus_link.open {
        return;
    }
    let mut minis = std::mem::take(&mut state.dialogs.stimulus_link.minis);
    let session = state.dialogs.stimulus_link.clone();
    let Some(component) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == session.component_id)
        .cloned()
    else {
        state.dialogs.stimulus_link.close();
        return;
    };

    match session.mode {
        StimulusLinkMode::Adopt => adopt::render(ctx, state, &session, &component, &mut minis),
        StimulusLinkMode::Extract => extract::render(ctx, state, &session, &component),
    }
    if state.dialogs.stimulus_link.open {
        state.dialogs.stimulus_link.minis = minis;
    }
}

/// Copy one definition onto the instance, re-typing it first when the family
/// differs. One component transaction, so one undo restores the type and the
/// card together.
pub(crate) fn commit_adoption(
    state: &mut AppState,
    component_id: u64,
    definition: &StimulusDefinition,
) -> Result<String, String> {
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
    let re_placed = match definition.adoption_fit(&expected) {
        AdoptionFit::Same => false,
        AdoptionFit::Replace { .. } => true,
        AdoptionFit::Kind => return Err(definition.kind_refusal(&expected)),
    };
    let candidate =
        realized(&expected, definition).ok_or_else(|| definition.kind_refusal(&expected))?;
    let reference = candidate.spice_instance_name();
    if re_placed {
        // Not a property edit, and the property-edit transaction says so:
        // `prepare_component_edit` refuses any candidate whose type differs
        // from the one on the sheet, because a typed parameter commit must
        // never be able to change what a component *is*. A family change is
        // exactly that change, so it is written as its own operation — one
        // undo entry, the instance replaced in place, and the topology version
        // bumped because the netlister must re-read the card.
        if state.schematic_edit_read_only() {
            return Err(format!(
                "{reference} was not re-placed: the active schematic view is read-only."
            ));
        }
        let description = format!("adopt {} onto {reference}", definition.name());
        let replaced = state.schematic.with_undo(description, |schematic| {
            if let Some(held) = schematic
                .components
                .iter_mut()
                .find(|component| component.id == component_id)
            {
                *held = candidate;
            }
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        });
        if !replaced {
            return Err(format!(
                "{reference} was not re-placed: the active schematic view is read-only."
            ));
        }
    } else {
        state.edit_component_transaction(&expected, candidate, "adopt stimulus definition")?;
    }
    Ok(if re_placed {
        format!(
            "{reference} was re-placed as {} and adopted {} r{}; both terminals kept their nets.",
            definition.component_type().display_name(),
            definition.name(),
            definition.revision(),
        )
    } else {
        format!(
            "{reference} adopted {} r{}.",
            definition.name(),
            definition.revision(),
        )
    })
}

/// Copy the library's current revision of `definition` back onto one instance
/// that already adopted it.
///
/// Narrower than [`commit_adoption`] on purpose, because the model is: an
/// instance naming some other definition is adopting, not re-adopting, and
/// `readopt_onto` is where that refusal is written. Every surface that offers
/// the verb — the Studio's Excitations rows, the component editor's evidence
/// pane, the schematic's own menu — comes through here, so one re-adoption is
/// one transaction and one console sentence wherever it is asked for.
pub(crate) fn commit_readoption(
    state: &mut AppState,
    component_id: u64,
    definition: &str,
) -> Result<String, String> {
    let held = state
        .workspace
        .stimulus_library
        .get(definition)
        .cloned()
        .ok_or_else(|| format!("This project defines no '{definition}' to re-adopt."))?;
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
    let mut candidate = expected.clone();
    held.readopt_onto(&mut candidate)?;
    let reference = candidate.spice_instance_name();
    state.edit_component_transaction(&expected, candidate, "re-adopt stimulus definition")?;
    Ok(format!(
        "{reference} re-adopted {definition} r{}; its card is the library's again.",
        held.revision()
    ))
}

/// Publish the instance's card as a new `r1` definition and point the instance
/// at it.
fn commit_extraction(state: &mut AppState, component_id: u64) -> Result<String, String> {
    let session = state.dialogs.stimulus_link.clone();
    let expected = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "That instance is no longer on this sheet.".to_owned())?;
    if let Some(refusal) = extract_refusal(state, &expected, &session.name) {
        return Err(refusal);
    }
    let mut candidate = expected.clone();
    let definition =
        StimulusDefinition::extract_from(&mut candidate, &session.name, &session.purpose)
            .map_err(|error| error.to_string())?;
    let name = definition.name().to_owned();
    // The library first, then the instance, and the library back out if the
    // instance refuses. The other order leaves a receipt naming a definition
    // the project does not hold — which every surface reads as `definition
    // removed`, over an extraction the reader was told had failed.
    state
        .workspace
        .stimulus_library
        .insert(definition)
        .map_err(|error| error.to_string())?;
    if let Err(refusal) =
        state.edit_component_transaction(&expected, candidate, "save stimulus definition")
    {
        let _ = state.workspace.stimulus_library.delete(&name);
        return Err(refusal);
    }
    state.workbench.selected_stimulus_definition = Some(name.clone());
    Ok(format!(
        "{} saved {name} as r1 in this project's stimulus library and now reads adopted \u{00b7} \
         r1.",
        session.reference
    ))
}

/// Open the Stimulus Library workspace on one definition.
pub(crate) fn open_stimulus_definition(state: &mut AppState, name: &str) {
    state.workbench.selected_stimulus_definition = Some(name.to_owned());
    state.workbench.activate(Workspace::Stimulus);
}

#[cfg(test)]
mod tests;

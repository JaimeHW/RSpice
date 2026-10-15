//! Live display evidence and interaction results for the component editor.

/// Live, non-editable evidence shown under a component's typed parameters.
///
/// The property draft remains owned by [`super::ComponentPropertyDraft`]. This
/// projection is rebuilt from authoritative application state every frame, so
/// model provenance and stimulus provenance can never become a second source
/// of truth.
///
/// It carries only what this editor is for, which is changing the instance.
/// Where its pins land and what a retained run measured on it are readings of
/// the instance in place, and the design inspector states both beside the
/// schematic, where a reader can act on them without first dismissing a modal.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComponentEditorContext {
    pub glyph: String,
    pub subtitle: String,
    pub instance_path: String,
    pub library_cell: String,
    pub family: String,
    pub model: Option<ComponentModelContext>,
    /// Where this instance stands with the project's stimulus library.
    /// `None` for anything that is not an independent source, which is every
    /// component with no waveform to adopt.
    pub stimulus: Option<StimulusEditorContext>,
}

/// What the editor shows about one source's link to the stimulus library.
///
/// Every field is read from the library and the instance when the frame is
/// built; nothing here is retained, so the chip in the header, the section in
/// the evidence pane and the Excitations page cannot describe one instance
/// differently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StimulusEditorContext {
    /// The lifecycle word, from the model that computes it.
    pub state: rspice_design::stimulus_library::provenance::ProvenanceState,
    /// The definition the receipt names, if it names one.
    pub definition: Option<String>,
    /// The revision the library holds of that definition now, if it still
    /// holds it. `None` is what makes "open it" an offer the editor withholds.
    pub library_revision: Option<u32>,
    /// Whether the project has authored any definition at all: with none,
    /// there is nothing to adopt and the verb is not offered.
    pub library_is_empty: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComponentModelContext {
    pub name: String,
    /// Exact catalog library identity when the binding was resolved from a
    /// model library. Model names alone are not globally unique.
    pub library: Option<String>,
    pub source: String,
    pub section: String,
    pub status: String,
    pub can_open: bool,
    pub can_qualify: bool,
}

/// Result of the component editor interaction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ComponentPropertyDialogResult {
    /// No action taken
    #[default]
    None,
    /// User clicked Apply - commit and retain the editor.
    Applied,
    /// User clicked OK - commit and close after the host accepts the change.
    AppliedAndClose,
    /// Open the selected model's source/catalog record.
    OpenModel,
    /// Open qualification evidence for the selected model.
    OpenQualification,
    /// Close the editor and open the adopt half of the stimulus link dialog.
    AdoptStimulus,
    /// Close the editor and open the save half of the stimulus link dialog.
    ExtractStimulus,
    /// Close the editor and show the adopted definition in the Stimulus
    /// Library workspace.
    OpenStimulusDefinition,
    /// Copy the library's current revision onto this instance now, and reload
    /// the editor from what that leaves behind.
    ReadoptStimulus,
    /// User clicked Cancel - changes discarded
    Cancelled,
}

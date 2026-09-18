//! What the Stimulus Library instrument is in the middle of editing.
//!
//! A draft belongs to the definition it edits, not to the workspace: switching
//! to another definition and back has to find the same unapplied working
//! record and the same undo history, or the browser beside the stage becomes a
//! way to lose work by clicking on it. So the editor keeps one
//! [`DefinitionDraft`] per definition, keyed the way the library compares
//! names — case-insensitively, because SPICE reads `VDD` and `vdd` as one
//! instance name.
//!
//! None of this is the project. A draft is unapplied by definition, the
//! library document holds only what Apply published, and the whole record is
//! runtime-only: restoring a half-typed pulse width into a project someone
//! else has since republished would show a revision nobody wrote. Closing the
//! project, reverting the library document and resetting the workspace view
//! therefore clear it outright.
//!
//! The preview cache lives here for the same reason the drafts do: the proof
//! surface is the engine's own evaluator, and asking it for four hundred
//! samples of a modulated carrier on every frame is real work. It is keyed on
//! the three things that can change what the engine would answer — the working
//! record, the span the surface is looking through, and the transient the plan
//! resolves omitted fields against — so a frame that changed none of them
//! repaints the samples it already had.

use std::collections::HashMap;

use crate::simulation::stimulus_realize::{PreviewTiming, StimulusRealization};
use crate::state::stimulus_library::definition::StimulusDefinition;
use crate::state::stimulus_library::draft::DefinitionDraft;

/// Which span the proof surface looks through.
///
/// Three spans, because a waveform answers three different questions: what
/// shape is this, what does one cycle look like, and what will the run see.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PreviewSpan {
    /// The natural span of the shape, derived from its own resolved timing.
    #[default]
    Fit,
    /// One fundamental period, for the families that have one.
    Period,
    /// The plan's own transient window, which is what a run would show.
    Transient,
}

impl PreviewSpan {
    /// The three spans, in the order the strip offers them.
    pub const ALL: [Self; 3] = [Self::Fit, Self::Period, Self::Transient];

    /// The word on the button.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fit => "Fit",
            Self::Period => "Period",
            Self::Transient => "Transient",
        }
    }
}

/// One definition's open editing session.
#[derive(Debug, Clone)]
struct EditorEntry {
    draft: DefinitionDraft,
    /// Which PWL row is selected, as a row index into the authored table.
    /// The plot's markers and the table's rows read the same slot, so
    /// clicking either one selects in both.
    selected_point: Option<usize>,
}

/// What the preview was last asked, and what the engine answered.
#[derive(Debug, Clone)]
struct PreviewCache {
    record: StimulusDefinition,
    span: PreviewSpan,
    timing: PreviewTiming,
    realization: StimulusRealization,
}

/// The field a reader is typing into, before it becomes an edit.
///
/// A draft's history is one entry per edit, and a keystroke is not an edit: a
/// pulse width typed digit by digit would bury the previous state under six
/// undo steps. So the in-progress text is held here and committed to the draft
/// once, when the field loses focus or Enter closes it. The proof surface
/// reads the record with this text laid over it, so the curve still follows
/// the typing.
#[derive(Debug, Clone)]
struct FieldEdit {
    definition: String,
    field: String,
    text: String,
}

/// Every definition the instrument has open, and the preview it last drew.
#[derive(Debug, Clone, Default)]
pub struct StimulusEditorState {
    entries: HashMap<String, EditorEntry>,
    /// Which span the proof surface is looking through. One choice for the
    /// instrument rather than one per definition: it is a way of looking, and
    /// a reader who chose "Transient" means it for the next definition too.
    pub span: PreviewSpan,
    cache: Option<PreviewCache>,
    field: Option<FieldEdit>,
    nets: Option<(u64, HashMap<(u64, String), String>)>,
    focus_name: bool,
    pending_delete: Option<String>,
    /// How many times the engine has been asked to evaluate a waveform for
    /// this instrument. The cache is a claim about per-frame cost, and this
    /// is what lets a test hold it to that claim.
    evaluations: u64,
}

impl StimulusEditorState {
    /// The draft for this definition, opened from the library's record if the
    /// instrument has not opened it yet.
    ///
    /// A clean draft whose saved record no longer matches the library's is
    /// reopened: the library moved under it, through an Apply somewhere else
    /// or a revert of the document, and a clean draft has nothing to lose. A
    /// dirty one is kept, because its working record is unapplied work and
    /// discarding it here would be exactly the silent loss the workspace's
    /// close guard exists to prevent.
    pub fn draft_for(&mut self, saved: &StimulusDefinition) -> &mut DefinitionDraft {
        let key = Self::key(saved.name());
        let entry = self.entries.entry(key).or_insert_with(|| EditorEntry {
            draft: DefinitionDraft::new(saved.clone()),
            selected_point: None,
        });
        if !entry.draft.is_dirty() && entry.draft.saved().normalized() != saved.normalized() {
            entry.draft = DefinitionDraft::new(saved.clone());
        }
        &mut entry.draft
    }

    /// The draft for this definition, if one is open.
    #[must_use]
    pub fn draft(&self, name: &str) -> Option<&DefinitionDraft> {
        self.entries.get(&Self::key(name)).map(|entry| &entry.draft)
    }

    /// Whether this definition has unapplied edits.
    #[must_use]
    pub fn is_dirty(&self, name: &str) -> bool {
        self.draft(name).is_some_and(DefinitionDraft::is_dirty)
    }

    /// Every definition with unapplied edits, by the name its draft carries,
    /// sorted so two readers of the same editor list them the same way.
    #[must_use]
    pub fn dirty_definitions(&self) -> Vec<String> {
        let mut names = self
            .entries
            .values()
            .filter(|entry| entry.draft.is_dirty())
            .map(|entry| entry.draft.working().name().to_owned())
            .collect::<Vec<_>>();
        names.sort();
        names
    }

    /// Forget this definition's draft and its point selection.
    pub fn close(&mut self, name: &str) {
        self.entries.remove(&Self::key(name));
        self.cache = None;
    }

    /// Follow a rename, so the draft the reader is typing into stays theirs.
    pub fn rename_key(&mut self, old: &str, new: &str) {
        let old_key = Self::key(old);
        let new_key = Self::key(new);
        if old_key == new_key {
            return;
        }
        if let Some(entry) = self.entries.remove(&old_key) {
            self.entries.insert(new_key, entry);
        }
    }

    /// Forget everything. The project closed, or its library was reverted.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.span = PreviewSpan::default();
        self.cache = None;
        self.field = None;
        self.pending_delete = None;
    }

    /// Which PWL point this definition's table and plot have selected.
    #[must_use]
    pub fn selected_point(&self, name: &str) -> Option<usize> {
        self.entries
            .get(&Self::key(name))
            .and_then(|entry| entry.selected_point)
    }

    /// Select a PWL point, by row index, for this definition.
    pub fn select_point(&mut self, name: &str, point: Option<usize>) {
        if let Some(entry) = self.entries.get_mut(&Self::key(name)) {
            entry.selected_point = point;
        }
    }

    /// The engine's reading of this record over this span, evaluated only when
    /// one of the three things it depends on has changed.
    pub fn realization(
        &mut self,
        record: &StimulusDefinition,
        span: PreviewSpan,
        timing: PreviewTiming,
    ) -> &StimulusRealization {
        let normalized = record.normalized();
        let reusable = self.cache.as_ref().is_some_and(|cache| {
            cache.span == span && cache.timing == timing && cache.record == normalized
        });
        if !reusable {
            self.evaluations = self.evaluations.saturating_add(1);
            self.cache = Some(PreviewCache {
                record: normalized,
                span,
                timing,
                realization: StimulusRealization::of(record, span_seconds(span), timing),
            });
        }
        // The branch above has just written it when it was not there.
        &self
            .cache
            .as_ref()
            .expect("the preview cache was filled above")
            .realization
    }

    /// How many times this editor has asked the engine for a waveform.
    #[must_use]
    pub const fn evaluations(&self) -> u64 {
        self.evaluations
    }

    /// Ask the identity band to put the caret in the name field.
    ///
    /// A definition created from a verb opens under a generated name, and the
    /// first thing anyone does with it is give it a real one.
    pub fn focus_name(&mut self) {
        self.focus_name = true;
    }

    /// Whether this frame is the one that should take the caret.
    pub fn take_focus_name(&mut self) -> bool {
        std::mem::take(&mut self.focus_name)
    }

    /// Which field of this definition is being typed into, if any.
    #[must_use]
    pub fn editing_field(&self, definition: &str) -> Option<&str> {
        self.field
            .as_ref()
            .filter(|edit| edit.definition == Self::key(definition))
            .map(|edit| edit.field.as_str())
    }

    /// The in-progress text of one field, if that field is the one being
    /// typed into.
    #[must_use]
    pub fn field_text(&self, definition: &str, field: &str) -> Option<&str> {
        self.field
            .as_ref()
            .filter(|edit| edit.definition == Self::key(definition) && edit.field == field)
            .map(|edit| edit.text.as_str())
    }

    /// Start, or continue, typing into one field.
    pub fn set_field_text(&mut self, definition: &str, field: &str, text: String) {
        self.field = Some(FieldEdit {
            definition: Self::key(definition),
            field: field.to_owned(),
            text,
        });
    }

    /// Which definition a delete is waiting on an answer about.
    #[must_use]
    pub fn pending_delete(&self) -> Option<&str> {
        self.pending_delete.as_deref()
    }

    /// Ask before deleting this definition.
    pub fn ask_before_deleting(&mut self, definition: &str) {
        self.pending_delete = Some(definition.to_owned());
    }

    /// Take the definition a delete was waiting on, answered either way.
    pub fn take_pending_delete(&mut self) -> Option<String> {
        self.pending_delete.take()
    }

    /// Abandon the field being typed into without committing it.
    pub fn clear_field(&mut self) {
        self.field = None;
    }

    /// The working record with the in-progress field laid over it.
    ///
    /// This is what the proof surface, the realization band and the audit
    /// strip all read, so a half-typed frequency is previewed, realized and
    /// audited as one record rather than three.
    #[must_use]
    pub fn live_record(
        &self,
        definition: &str,
        working: &StimulusDefinition,
    ) -> StimulusDefinition {
        let Some(edit) = self
            .field
            .as_ref()
            .filter(|edit| edit.definition == Self::key(definition))
        else {
            return working.clone();
        };
        let mut live = working.clone();
        write_field(&mut live, &edit.field, &edit.text);
        live
    }

    /// The net every placed terminal sits on, extracted once per schematic
    /// revision.
    ///
    /// Naming the nets of a placed adopter means resolving the whole sheet's
    /// connectivity, which is not something to do on every frame of an
    /// instrument that is not the schematic. The design's own epoch is the
    /// key: it advances whenever the active buffer changes, which is exactly
    /// when a net name can.
    pub fn instance_nets(
        &mut self,
        epoch: u64,
        schematic: &crate::state::SchematicState,
    ) -> &HashMap<(u64, String), String> {
        if self
            .nets
            .as_ref()
            .is_none_or(|(cached, _)| *cached != epoch)
        {
            let nets = crate::simulation::netlist_gen::design_nets(schematic)
                .into_iter()
                .flat_map(|net| {
                    let name = net.name.clone();
                    net.terminals
                        .into_iter()
                        .map(move |terminal| ((terminal.component_id, terminal.pin), name.clone()))
                })
                .collect();
            self.nets = Some((epoch, nets));
        }
        // The branch above has just written it when it was not there.
        &self
            .nets
            .as_ref()
            .expect("the net cache was filled above")
            .1
    }

    fn key(name: &str) -> String {
        name.trim().to_lowercase()
    }
}

/// The field name a definition's own name is filed under while it is being
/// typed. It is not a property of the sheet, and `name` is the spelling every
/// source sheet already uses for an instance's own name.
pub const NAME_FIELD: &str = "name";

/// Write one field into a record, in the instance's own spelling.
///
/// The primary field is what `Component::value` carries and everything else
/// lives in the parameter string, so which of the two a field belongs to is
/// the property bridge's answer and not this module's. A name that is not an
/// identifier is left unwritten: the audit strip is where a reader learns
/// that, and a record that refused the name still previews the waveform.
pub fn write_field(record: &mut StimulusDefinition, field: &str, value: &str) {
    if field == NAME_FIELD {
        let _ = record.rename(value.trim());
        return;
    }
    let primary =
        crate::properties::property_bridge::get_primary_property_name(record.component_type());
    if field == primary {
        record.value = value.trim().to_owned();
        return;
    }
    let mut params = crate::state::parse_params_string(&record.params);
    let value = value.trim();
    if value.is_empty() {
        params.remove(field);
    } else {
        params.insert(field.to_owned(), value.to_owned());
    }
    record.params = crate::state::format_params_string(&params);
}

/// What a record currently says for one field, in its own spelling.
#[must_use]
pub fn read_field(record: &StimulusDefinition, field: &str) -> String {
    if field == NAME_FIELD {
        return record.name().to_owned();
    }
    let primary =
        crate::properties::property_bridge::get_primary_property_name(record.component_type());
    if field == primary {
        return record.value.clone();
    }
    crate::state::parse_params_string(&record.params)
        .get(field)
        .cloned()
        .unwrap_or_default()
}

/// Which span the realization is asked for.
///
/// The span choice is the instrument's, and the arithmetic that turns it into
/// seconds is the engine bridge's; this is the one word that crosses between
/// them, so the two never grow separate notions of "Period".
const fn span_seconds(span: PreviewSpan) -> crate::simulation::stimulus_realize::SpanChoice {
    use crate::simulation::stimulus_realize::SpanChoice;

    match span {
        PreviewSpan::Fit => SpanChoice::Fit,
        PreviewSpan::Period => SpanChoice::Period,
        PreviewSpan::Transient => SpanChoice::Transient,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;

    fn sine(name: &str) -> StimulusDefinition {
        let mut definition = StimulusDefinition::new(name, ComponentType::VoltageSourceSin)
            .expect("a sine source is placeable");
        definition.value = "0".to_owned();
        definition.params = "va=3m freq=1k".to_owned();
        definition
    }

    #[test]
    fn a_dirty_draft_survives_a_switch_to_another_definition_and_back() {
        let mut editor = StimulusEditorState::default();
        let first = sine("sensor_diff_1k");
        let second = sine("bridge_drive");
        editor
            .draft_for(&first)
            .edit(|working| working.params = "va=6m freq=1k".to_owned());
        editor.draft_for(&second);

        assert!(editor.is_dirty("sensor_diff_1k"));
        assert!(!editor.is_dirty("bridge_drive"));
        let reopened = editor.draft_for(&first);
        assert_eq!(reopened.working().params, "va=6m freq=1k");
        assert!(reopened.can_undo(), "the undo history came back with it");
    }

    #[test]
    fn names_are_matched_the_way_the_library_matches_them() {
        let mut editor = StimulusEditorState::default();
        editor
            .draft_for(&sine("VDD_OPERATE"))
            .edit(|working| working.value = "1.8".to_owned());
        assert!(editor.is_dirty("vdd_operate"));
        assert!(editor.draft("Vdd_Operate").is_some());
    }

    #[test]
    fn closing_one_definition_forgets_it_and_leaves_the_others() {
        let mut editor = StimulusEditorState::default();
        editor
            .draft_for(&sine("one"))
            .edit(|working| working.value = "1".to_owned());
        editor
            .draft_for(&sine("two"))
            .edit(|working| working.value = "2".to_owned());

        editor.close("one");
        assert!(editor.draft("one").is_none());
        assert!(editor.is_dirty("two"));
        assert_eq!(editor.dirty_definitions(), vec!["two".to_owned()]);
    }

    #[test]
    fn clearing_the_editor_forgets_every_draft() {
        let mut editor = StimulusEditorState::default();
        editor.span = PreviewSpan::Transient;
        editor
            .draft_for(&sine("one"))
            .edit(|working| working.value = "1".to_owned());
        editor.clear();

        assert!(editor.dirty_definitions().is_empty());
        assert!(editor.draft("one").is_none());
        assert_eq!(editor.span, PreviewSpan::Fit);
    }

    #[test]
    fn a_rename_carries_the_draft_to_the_new_name() {
        let mut editor = StimulusEditorState::default();
        editor
            .draft_for(&sine("old_name"))
            .edit(|working| working.value = "9".to_owned());
        editor.rename_key("old_name", "new_name");

        assert!(editor.draft("old_name").is_none());
        assert_eq!(
            editor
                .draft("new_name")
                .map(|draft| draft.working().value.as_str()),
            Some("9")
        );
    }

    /// The library can move under a draft — another Apply, or a revert of the
    /// document. A clean draft follows it; a dirty one keeps the work.
    #[test]
    fn a_clean_draft_reopens_on_the_library_record_and_a_dirty_one_does_not() {
        let mut editor = StimulusEditorState::default();
        editor.draft_for(&sine("moved"));
        let mut republished = sine("moved");
        republished.params = "va=9m freq=1k".to_owned();
        assert_eq!(
            editor.draft_for(&republished).working().params,
            "va=9m freq=1k"
        );

        editor
            .draft_for(&republished)
            .edit(|working| working.value = "1".to_owned());
        let mut again = sine("moved");
        again.params = "va=12m freq=1k".to_owned();
        assert_eq!(editor.draft_for(&again).working().params, "va=9m freq=1k");
    }

    #[test]
    fn the_engine_is_asked_once_for_a_frame_that_changed_nothing() {
        let mut editor = StimulusEditorState::default();
        let definition = sine("sensor_diff_1k");
        let timing = PreviewTiming::default();

        editor.realization(&definition, PreviewSpan::Fit, timing);
        let after_first = editor.evaluations();
        editor.realization(&definition, PreviewSpan::Fit, timing);
        editor.realization(&definition, PreviewSpan::Fit, timing);
        assert_eq!(editor.evaluations(), after_first, "nothing changed");

        editor.realization(&definition, PreviewSpan::Transient, timing);
        assert_eq!(editor.evaluations(), after_first + 1, "the span changed");

        let mut edited = definition.clone();
        edited.params = "va=6m freq=1k".to_owned();
        editor.realization(&edited, PreviewSpan::Transient, timing);
        assert_eq!(editor.evaluations(), after_first + 2, "the record changed");
    }
}

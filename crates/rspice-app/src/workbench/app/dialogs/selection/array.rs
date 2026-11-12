//! Create array: three questions, then the canvas takes the pitch.
//!
//! The three controls are the whole design decision. Placement is acquired
//! from the canvas after the draft has been parsed and the immutable
//! schematic authority has been revalidated.
//!
//! The model prints a count as `8 × 1` and a naming range as
//! `U4…U11 · DATA[0]…DATA[7]`. None of `×`, `…` or `·` is on a keyboard, so
//! the two fields take `x`, `...` and `,` and rewrite them as they are typed:
//! the reader types what they can reach and reads back what the design calls
//! it.

use egui::{Context, Id};

use crate::state::{
    Point, SchematicArrayCount, SchematicArrayKind, SchematicArrayNaming, SchematicArrayPlacement,
    SchematicArrayPlan, Tool,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine, FormRows};

use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app::dialogs::selection::move_selection::{
    MoveSelectionDialogState, cancel_armed_move_selection_in,
};
use crate::workbench::app::dialogs::selection::stretch::{
    StretchSelectionDialogState, cancel_armed_stretch_selection_in,
};
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::state::WorkbenchSession;

/// Isolated authority, typed draft, and canvas interaction state for the
/// mockup-owned Create object array transaction. The complete replica set is
/// validated as one candidate and the live document remains unchanged until
/// the canvas commit boundary succeeds.
#[derive(Debug, Clone)]
pub(crate) struct ArraySelectionPreviewCache {
    pub(crate) plan: crate::state::SchematicArrayPlan,
    pub(crate) library_revision: u64,
    pub(crate) symbol_revision: u64,
    pub(crate) identity_cursor: u64,
    pub(crate) preview: Result<crate::state::SchematicArrayPreview, String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ArraySelectionDialogState {
    pub(crate) open: bool,
    pub(crate) armed: bool,
    pub(crate) kind: crate::state::SchematicArrayKind,
    pub(crate) count: String,
    pub(crate) naming: String,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) canvas: rspice_schematic_editor::session::transform::TransformCanvasSession,
    pub(crate) generation: u64,
    pub(crate) preview_cache: Option<ArraySelectionPreviewCache>,
    pub(crate) initial_kind: crate::state::SchematicArrayKind,
    pub(crate) initial_count: String,
    pub(crate) initial_naming: String,
    pub(crate) dirty: bool,
}

impl ArraySelectionDialogState {
    pub(crate) fn open(
        &mut self,
        authority: SchematicEditAuthority,
        count: String,
        naming: String,
    ) {
        let initial_count = count.clone();
        let initial_naming = naming.clone();
        *self = Self {
            open: true,
            armed: false,
            kind: crate::state::SchematicArrayKind::default(),
            count,
            naming,
            authority: Some(authority),
            canvas: Default::default(),
            generation: self.generation.wrapping_add(1),
            preview_cache: None,
            initial_kind: crate::state::SchematicArrayKind::default(),
            initial_count,
            initial_naming,
            dirty: false,
        };
    }

    pub(crate) fn arm(&mut self) {
        self.open = false;
        self.armed = true;
        self.canvas = Default::default();
        self.generation = self.generation.wrapping_add(1);
        self.preview_cache = None;
        self.dirty = false;
    }

    pub(crate) fn close(&mut self) {
        *self = Self {
            generation: self.generation.wrapping_add(1),
            ..Self::default()
        };
    }

    pub(crate) fn mark_edited(&mut self) {
        self.dirty = self.kind != self.initial_kind
            || !equivalent_array_count(&self.count, &self.initial_count)
            || !equivalent_array_naming(&self.naming, &self.initial_naming);
        self.preview_cache = None;
    }
}

fn equivalent_array_count(left: &str, right: &str) -> bool {
    left == right
        || crate::state::SchematicArrayCount::parse(left)
            .ok()
            .zip(crate::state::SchematicArrayCount::parse(right).ok())
            .is_some_and(|(left, right)| left == right)
}

fn equivalent_array_naming(left: &str, right: &str) -> bool {
    left == right
        || crate::state::SchematicArrayNaming::parse(left)
            .ok()
            .zip(crate::state::SchematicArrayNaming::parse(right).ok())
            .is_some_and(|(left, right)| left == right)
}

const TITLE: &str = "Create array";
const PRIMARY: &str = "Create";
const DEFAULT_COUNT: &str = "8 \u{00d7} 1";
const COUNT_HINT: &str = "8 x 1";

/// `×` is the only separator the count has, and it is not on a keyboard.
pub(crate) fn canonical_count(typed: &str) -> String {
    typed.replace(['x', 'X', '*'], "\u{00d7}")
}

#[derive(Debug)]
enum DraftValidation {
    Invalid(DraftErrors),
    Valid,
}

#[derive(Debug, Default)]
struct DraftErrors {
    messages: Vec<String>,
    array: Option<String>,
    count: Option<String>,
    naming: Option<String>,
}

impl DraftErrors {
    fn push_array(&mut self, message: String) {
        append_field_error(&mut self.array, &message);
        self.messages.push(message);
    }

    fn push_count(&mut self, message: String) {
        append_field_error(&mut self.count, &message);
        self.messages.push(message);
    }

    fn push_naming(&mut self, message: String) {
        append_field_error(&mut self.naming, &message);
        self.messages.push(message);
    }
}

fn append_field_error(field: &mut Option<String>, message: &str) {
    match field {
        Some(existing) => {
            existing.push(' ');
            existing.push_str(message);
        }
        None => *field = Some(message.to_owned()),
    }
}

impl DraftValidation {
    fn can_commit(&self) -> bool {
        matches!(self, Self::Valid)
    }

    #[cfg(test)]
    fn field_errors(&self) -> (bool, bool, bool) {
        match self {
            Self::Invalid(errors) => (
                errors.array.is_some(),
                errors.count.is_some(),
                errors.naming.is_some(),
            ),
            Self::Valid => (false, false, false),
        }
    }

    fn field_error_details(&self) -> (Option<&str>, Option<&str>, Option<&str>) {
        match self {
            Self::Invalid(errors) => (
                errors.array.as_deref(),
                errors.count.as_deref(),
                errors.naming.as_deref(),
            ),
            Self::Valid => (None, None, None),
        }
    }

    /// A refusal that belongs to no field — a stale baseline, a plan the
    /// three fields are each legal for but that cannot be built.
    fn form_refusal(&self) -> Option<String> {
        let Self::Invalid(errors) = self else {
            return None;
        };
        let unattributed = errors
            .messages
            .iter()
            .filter(|message| {
                ![&errors.array, &errors.count, &errors.naming]
                    .into_iter()
                    .flatten()
                    .any(|field| field.contains(message.as_str()))
            })
            .cloned()
            .collect::<Vec<_>>();
        (!unattributed.is_empty()).then(|| unattributed.join(" "))
    }
}

/// [`open_array_selection_dialog_in`] over the whole state, the shape the
/// canvas tests open it through.
#[cfg(test)]
pub(crate) fn open_array_selection_dialog(state: &mut AppState) {
    let design = &mut state.design;
    open_array_selection_dialog_in(
        &mut design.array_selection,
        &mut design.move_selection,
        &mut design.stretch_selection,
        &mut state.model,
        &mut state.workbench,
    );
}

/// Open Create array over the three canvas tools' drafts, the model and the
/// session: opening it stops an armed Move or Stretch.
pub(crate) fn open_array_selection_dialog_in(
    dialog: &mut ArraySelectionDialogState,
    move_draft: &mut MoveSelectionDialogState,
    stretch: &mut StretchSelectionDialogState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
) {
    // Create array is greyed on a read-only schematic and on a selection it
    // cannot repeat, so those two return without a word.
    if model.schematic_edit_read_only()
        || model.schematic.validate_array_source_selection().is_err()
    {
        return;
    }
    let count = SchematicArrayCount::parse(DEFAULT_COUNT)
        .expect("the mockup-owned default array count is valid");
    let naming = match model.schematic.default_array_naming(count) {
        Ok(naming) => naming.to_string(),
        Err(error) => {
            wb.prompt(format!("Create array could not find unused names: {error}"));
            return;
        }
    };
    if move_draft.armed {
        cancel_armed_move_selection_in(move_draft, model);
    }
    if stretch.armed {
        cancel_armed_stretch_selection_in(stretch, model);
    }
    dialog.open(
        SchematicEditAuthority::capture(&*model),
        count.to_string(),
        naming,
    );
}

fn field_id(stable: &str) -> Id {
    Id::new(("array-selection", stable))
}

pub(in crate::workbench) fn render_array_selection_dialog(
    dialog: &mut ArraySelectionDialogState,
    model: &mut Model,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }
    derive_naming_from_count(dialog, model);
    let validation = validate_draft(dialog, model);
    let (array_error, _, naming_error) = validation.field_error_details();
    // The count's refusal stands under the count. Everything else — the
    // array kind, the names the count derives, a stale baseline — belongs
    // to no field the reader can type into, so the footer states it.
    let form_refusal = array_error
        .or(naming_error)
        .map(str::to_owned)
        .or_else(|| validation.form_refusal());
    let choice = CommandForm::new(TITLE, PRIMARY)
        .primary_enabled(validation.can_commit())
        .status(match form_refusal.as_deref() {
            Some(message) => Err(message),
            None => Ok(None),
        })
        .show(ctx, |rows| array_form(rows, &validation, &mut *dialog));
    match choice {
        DialogChoice::Primary => {
            if validate_draft(dialog, model).can_commit() {
                let radial_center = if dialog.kind == SchematicArrayKind::RadialDocumentation {
                    radial_documentation_center(model)
                } else {
                    None
                };
                dialog.arm();
                dialog.canvas.anchor = radial_center;
                model.schematic.arm_tool(Tool::ArraySelection);
                rspice_schematic_editor::view::canvas::request_focus(ctx);
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.close();
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

fn validate_draft(draft: &ArraySelectionDialogState, model: &Model) -> DraftValidation {
    let mut errors = DraftErrors::default();
    let Some(authority) = draft.authority.as_ref() else {
        errors.messages.push(
            "The retained design baseline is unavailable. Close and reopen Create array."
                .to_owned(),
        );
        return DraftValidation::Invalid(errors);
    };
    if let Err(message) = authority.validate(model, TITLE) {
        errors.messages.push(message);
    }
    if draft.kind == SchematicArrayKind::RadialDocumentation {
        let selection = &model.schematic.session.editor.selection;
        let documentation_count = selection
            .documentation_shapes
            .len()
            .saturating_add(selection.design_notes.len());
        if documentation_count == 0 || selection.count() != documentation_count {
            errors.push_array(
                "Radial documentation requires only complete design notes or documentation shapes."
                    .to_owned(),
            );
        }
    }
    let count = match SchematicArrayCount::parse(&draft.count) {
        Ok(count) => Some(count),
        Err(error) => {
            errors.push_count(format!("Count: {error}"));
            None
        }
    };
    let naming = match SchematicArrayNaming::parse(&draft.naming) {
        Ok(naming) => Some(naming),
        Err(error) => {
            errors.push_naming(format!("Naming: {error}"));
            None
        }
    };
    if let Some(count) = count
        && let Err(error) = count.validate_for(draft.kind)
    {
        errors.push_count(format!("Count: {error}"));
    }
    if let (Some(count), Some(naming)) = (count, naming.as_ref())
        && errors.count.is_none()
        && let Err(error) = naming.clone().normalized_for_members(count.member_count())
    {
        errors.push_naming(format!("Naming: {error}"));
    }
    if let (Some(count), Some(naming)) = (count, naming.as_ref())
        && errors.count.is_none()
        && errors.naming.is_none()
    {
        let placement = match draft.kind {
            SchematicArrayKind::RadialDocumentation => {
                SchematicArrayPlacement::Center(Point::origin())
            }
            SchematicArrayKind::Linear => SchematicArrayPlacement::Pitch(Point::new(
                model.schematic.document().grid_size.max(1),
                model.schematic.document().grid_size.max(1),
            )),
            SchematicArrayKind::Rectangular => SchematicArrayPlacement::Pitch(Point::new(
                model.schematic.document().grid_size.max(1),
                model.schematic.document().grid_size.max(1),
            )),
        };
        if let Err(error) = SchematicArrayPlan::new(draft.kind, count, naming.clone(), placement) {
            errors.messages.push(error.to_string());
        }
    }
    if errors.messages.is_empty() {
        DraftValidation::Valid
    } else {
        DraftValidation::Invalid(errors)
    }
}

pub(crate) fn armed_array_selection_authority(state: &AppState) -> Result<(), String> {
    if !state.design.array_selection.armed
        || state.model.schematic.session.editor.tool != Tool::ArraySelection
    {
        return Err("Create array is not armed.".to_owned());
    }
    match validate_draft(&state.design.array_selection, &state.model) {
        DraftValidation::Valid => Ok(()),
        DraftValidation::Invalid(errors) => Err(errors.messages.join(" ")),
    }
}

pub(crate) fn armed_array_selection_plan(
    state: &AppState,
    placement: SchematicArrayPlacement,
) -> Result<SchematicArrayPlan, String> {
    armed_array_selection_authority(state)?;
    let draft = &state.design.array_selection;
    SchematicArrayPlan::parse(draft.kind, &draft.count, &draft.naming, placement)
        .map_err(|error| error.to_string())
}

pub(crate) fn cancel_armed_array_selection(state: &mut AppState) {
    cancel_armed_array_selection_in(&mut state.design.array_selection, &mut state.model);
}

/// [`cancel_armed_array_selection`] over the dialog and the model.
pub(crate) fn cancel_armed_array_selection_in(
    dialog: &mut ArraySelectionDialogState,
    model: &mut Model,
) {
    dialog.close();
    if model.schematic.session.editor.tool == Tool::ArraySelection {
        model.schematic.cancel_tool();
    }
}

/// The names a component array takes follow from its count: the first unused
/// run of the selection's own prefix, as many as the count makes. They are
/// derived, not typed, so they are worked out again whenever the count
/// parses, and a count that does not parse leaves the last ones standing.
fn derive_naming_from_count(dialog: &mut ArraySelectionDialogState, model: &Model) {
    let Ok(count) = SchematicArrayCount::parse(&dialog.count) else {
        return;
    };
    if let Ok(naming) = model.schematic.default_array_naming(count) {
        dialog.naming = naming.to_string();
    }
}

/// The two rows: the kind of array, and its count with the names the count
/// derives read off under it, or the count's refusal in their place. Returns
/// the count field, which takes focus.
fn array_form(
    rows: &mut FormRows<'_>,
    validation: &DraftValidation,
    draft: &mut ArraySelectionDialogState,
) -> Option<Id> {
    let (_, count_error, _) = validation.field_error_details();

    let labels = SchematicArrayKind::ALL.map(|kind| kind.label().to_owned());
    if let Some(index) = rows.select("Array", "array-selection-kind", draft.kind.label(), &labels) {
        let next = SchematicArrayKind::ALL[index];
        if next != draft.kind {
            draft.kind = next;
            draft.mark_edited();
        }
    }

    let count_id = field_id("count");
    if rows
        .text("Count", count_id, &mut draft.count, COUNT_HINT)
        .changed()
    {
        draft.count = canonical_count(&draft.count);
        draft.mark_edited();
    }
    let names = naming_line(&draft.naming);
    rows.field_line(
        Some(count_id),
        match count_error {
            Some(error) => FieldLine::Error(error),
            None if names.is_empty() => FieldLine::Blank,
            None => FieldLine::Derived(&names),
        },
    );
    Some(count_id)
}

/// The derived names as the line under Count states them.
fn naming_line(naming: &str) -> String {
    if naming.is_empty() {
        String::new()
    } else {
        format!("Names {naming}")
    }
}

fn radial_documentation_center(model: &Model) -> Option<Point> {
    let selection = &model.schematic.session.editor.selection;
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut include = |point: Point| {
        min_x = min_x.min(point.x);
        min_y = min_y.min(point.y);
        max_x = max_x.max(point.x);
        max_y = max_y.max(point.y);
    };
    for note in &model.schematic.document().design_notes {
        if selection.has_design_note(note.id) {
            include(note.pos);
        }
    }
    for shape in &model.schematic.document().documentation_shapes {
        if selection.has_documentation_shape(shape.id) {
            for point in shape.geometry.points() {
                include(point);
            }
        }
    }
    (min_x != i32::MAX).then(|| {
        let center_x = i64::from(min_x) + (i64::from(max_x) - i64::from(min_x)) / 2;
        let center_y = i64::from(min_y) + (i64::from(max_y) - i64::from(min_y)) / 2;
        Point::new(center_x as i32, center_y as i32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, DesignNote, DesignNoteKind};

    /// The draft's validation, over what the form reads.
    fn validate_draft(state: &AppState) -> DraftValidation {
        super::validate_draft(&state.design.array_selection, &state.model)
    }

    /// The names the count derives, over what the form reads.
    fn derive_naming_from_count(state: &mut AppState) {
        super::derive_naming_from_count(&mut state.design.array_selection, &state.model);
    }

    /// The radial array's centre, over the model it reads.
    fn radial_documentation_center(state: &AppState) -> Option<Point> {
        super::radial_documentation_center(&state.model)
    }

    #[test]
    fn the_form_offers_every_array_kind_under_its_own_name() {
        assert_eq!(
            SchematicArrayKind::ALL.map(SchematicArrayKind::label),
            ["Linear", "Rectangular", "Radial documentation"]
        );
    }

    /// `×` is not on a keyboard. What a reader can type has to reach the
    /// model, and what it reaches has to be what the model prints.
    #[test]
    fn the_count_separator_is_typed_in_ascii_and_read_back_canonical() {
        assert_eq!(canonical_count("8x1"), "8\u{00d7}1");
        assert_eq!(canonical_count("2 X 4"), "2 \u{00d7} 4");
        assert_eq!(canonical_count("3*3"), "3\u{00d7}3");
        assert!(SchematicArrayCount::parse(&canonical_count("8 x 1")).is_ok());
    }

    /// The names are derived from the count, not typed: a new count derives
    /// a new run of names, and the line under Count reads them off.
    #[test]
    fn the_names_follow_the_count() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                Component::new(4, ComponentType::Resistor, Point::origin())
                    .with_name_value("R1", "1k"),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        let eight = state.design.array_selection.naming.clone();
        state.design.array_selection.count = "4 \u{00d7} 1".to_owned();
        derive_naming_from_count(&mut state);
        let four = state.design.array_selection.naming.clone();
        assert_ne!(four, eight);
        let expected = state
            .model
            .schematic
            .default_array_naming(SchematicArrayCount::parse("4 \u{00d7} 1").unwrap())
            .unwrap()
            .to_string();
        assert_eq!(four, expected);
        assert!(validate_draft(&state).can_commit());
        assert_eq!(naming_line(&four), format!("Names {four}"));
        assert_eq!(naming_line(""), "");
    }

    #[test]
    fn open_uses_exact_default_count_and_collision_free_naming() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                Component::new(4, ComponentType::Resistor, Point::origin())
                    .with_name_value("R1", "1k"),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);

        open_array_selection_dialog(&mut state);

        assert!(state.design.array_selection.open);
        assert_eq!(state.design.array_selection.count, DEFAULT_COUNT);
        assert!(!state.design.array_selection.naming.is_empty());
        assert!(state.design.array_selection.authority.is_some());
    }

    #[test]
    fn reverting_array_fields_to_the_opened_baseline_clears_dirty_state() {
        let mut draft = ArraySelectionDialogState {
            open: true,
            count: "8 \u{00d7} 1".to_owned(),
            initial_count: "8 \u{00d7} 1".to_owned(),
            ..ArraySelectionDialogState::default()
        };
        draft.count = "9 \u{00d7} 1".to_owned();
        draft.mark_edited();
        assert!(draft.dirty);

        draft.count = "8 \u{00d7} 1".to_owned();
        draft.mark_edited();
        assert!(!draft.dirty);
        draft.count = "8\u{00d7}1".to_owned();
        draft.mark_edited();
        assert!(!draft.dirty, "canonical-equivalent text is not a change");
    }

    #[test]
    fn radial_documentation_accepts_notes_and_starts_at_the_selection_center() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .design_notes
            .push(
                DesignNote::new(
                    11,
                    Point::new(120, -40),
                    DesignNoteKind::PlainText,
                    "review boundary",
                )
                .unwrap(),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_design_note(11);

        open_array_selection_dialog(&mut state);
        state.design.array_selection.kind = SchematicArrayKind::RadialDocumentation;

        assert!(validate_draft(&state).can_commit());
        assert_eq!(
            radial_documentation_center(&state),
            Some(Point::new(120, -40))
        );
    }

    #[test]
    fn rectangular_array_highlights_only_the_incompatible_count_field() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                Component::new(4, ComponentType::Resistor, Point::origin())
                    .with_name_value("R1", "1k"),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.design.array_selection.kind = SchematicArrayKind::Rectangular;

        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, true, false));
        // The refusal is the count row's, so nothing is said twice at the
        // foot of the form.
        assert_eq!(validation.form_refusal(), None);
    }

    #[test]
    fn malformed_naming_highlights_only_the_naming_field() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                Component::new(4, ComponentType::Resistor, Point::origin())
                    .with_name_value("R1", "1k"),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.design.array_selection.naming = "R1 to R8".to_owned();

        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, false, true));
        assert!(
            validation
                .field_error_details()
                .2
                .is_some_and(|detail| detail.contains("Naming:"))
        );
        assert!(!validation.can_commit());
    }

    #[test]
    fn stale_operation_is_not_misrepresented_as_a_field_error() {
        let mut state = AppState::default();
        state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .push(
                Component::new(4, ComponentType::Resistor, Point::origin())
                    .with_name_value("R1", "1k"),
            );
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.design.array_selection.authority = None;

        // No field is at fault, so the refusal stands at the foot of the form
        // rather than being pinned on a value the reader typed correctly.
        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, false, false));
        assert!(validation.form_refusal().is_some());
    }
}

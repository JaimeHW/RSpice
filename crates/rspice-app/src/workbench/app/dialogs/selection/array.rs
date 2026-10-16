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

use crate::diagnostics::ConsoleMessage;
use crate::state::{
    Point, SchematicArrayCount, SchematicArrayKind, SchematicArrayNaming, SchematicArrayPlacement,
    SchematicArrayPlan, Tool,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine, FormRows};

use crate::workbench::app::{ArraySelectionDialogState, RSpiceApp, SchematicEditAuthority};
use crate::workbench::app_state::AppState;

const TITLE: &str = "Create array";
const PRIMARY: &str = "Create";
const DEFAULT_COUNT: &str = "8 \u{00d7} 1";
const COUNT_HINT: &str = "8 x 1";
const NAMING_HINT: &str = "U4...U11, DATA[0]...DATA[7]";

/// `×` is the only separator the count has, and it is not on a keyboard.
pub(crate) fn canonical_count(typed: &str) -> String {
    typed.replace(['x', 'X', '*'], "\u{00d7}")
}

/// `…` separates a naming range's endpoints and `·` separates two ranges.
/// A reference endpoint can itself contain `x`, so the count's rewrite does
/// not apply here.
pub(crate) fn canonical_naming(typed: &str) -> String {
    typed
        .replace("...", "\u{2026}")
        .replace(", ", " \u{00b7} ")
        .replace(',', " \u{00b7} ")
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

pub(crate) fn open_array_selection_dialog(state: &mut AppState) {
    if state.schematic_edit_read_only() {
        state.push_user_message(ConsoleMessage::warning(
            "Create array is unavailable because the active schematic is read-only.".to_owned(),
        ));
        return;
    }
    if let Err(error) = state.schematic.validate_array_source_selection() {
        state.push_user_message(ConsoleMessage::warning(format!(
            "Create array requires one complete eligible selection: {error}"
        )));
        return;
    }
    let count = SchematicArrayCount::parse(DEFAULT_COUNT)
        .expect("the mockup-owned default array count is valid");
    let naming = match state.schematic.default_array_naming(count) {
        Ok(naming) => naming.to_string(),
        Err(error) => {
            state.push_user_message(ConsoleMessage::warning(format!(
                "Create array could not derive a collision-free naming policy: {error}"
            )));
            return;
        }
    };
    if state.dialogs.move_selection.armed {
        crate::workbench::app::cancel_armed_move_selection(state);
    }
    if state.dialogs.stretch_selection.armed {
        crate::workbench::app::cancel_armed_stretch_selection(state);
    }
    state.dialogs.array_selection.open(
        SchematicEditAuthority::capture(state),
        count.to_string(),
        naming,
    );
}

fn field_id(stable: &str) -> Id {
    Id::new(("array-selection", stable))
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_array_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.array_selection.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let form_refusal = validation.form_refusal();
        let choice = CommandForm::new(TITLE, PRIMARY)
            .primary_enabled(validation.can_commit())
            .status(match form_refusal.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .show(ctx, |rows| {
                array_form(rows, &validation, &mut self.state.dialogs.array_selection)
            });
        match choice {
            DialogChoice::Primary => {
                if validate_draft(&self.state).can_commit() {
                    let radial_center = if self.state.dialogs.array_selection.kind
                        == SchematicArrayKind::RadialDocumentation
                    {
                        radial_documentation_center(&self.state)
                    } else {
                        None
                    };
                    self.state.dialogs.array_selection.arm();
                    self.state.dialogs.array_selection.canvas.anchor = radial_center;
                    self.state.schematic.arm_tool(Tool::ArraySelection);
                    rspice_schematic_editor::view::canvas::request_focus(ctx);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.array_selection.close();
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn validate_draft(state: &AppState) -> DraftValidation {
    let draft = &state.dialogs.array_selection;
    let mut errors = DraftErrors::default();
    let Some(authority) = draft.authority.as_ref() else {
        errors.messages.push(
            "The retained design baseline is unavailable. Close and reopen Create array."
                .to_owned(),
        );
        return DraftValidation::Invalid(errors);
    };
    if let Err(message) = authority.validate(state, TITLE) {
        errors.messages.push(message);
    }
    if draft.kind == SchematicArrayKind::RadialDocumentation {
        let selection = &state.schematic.session.editor.selection;
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
                state.schematic.document().grid_size.max(1),
                state.schematic.document().grid_size.max(1),
            )),
            SchematicArrayKind::Rectangular => SchematicArrayPlacement::Pitch(Point::new(
                state.schematic.document().grid_size.max(1),
                state.schematic.document().grid_size.max(1),
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
    if !state.dialogs.array_selection.armed
        || state.schematic.session.editor.tool != Tool::ArraySelection
    {
        return Err("Create array is not armed.".to_owned());
    }
    match validate_draft(state) {
        DraftValidation::Valid => Ok(()),
        DraftValidation::Invalid(errors) => Err(errors.messages.join(" ")),
    }
}

pub(crate) fn armed_array_selection_plan(
    state: &AppState,
    placement: SchematicArrayPlacement,
) -> Result<SchematicArrayPlan, String> {
    armed_array_selection_authority(state)?;
    let draft = &state.dialogs.array_selection;
    SchematicArrayPlan::parse(draft.kind, &draft.count, &draft.naming, placement)
        .map_err(|error| error.to_string())
}

pub(crate) fn cancel_armed_array_selection(state: &mut AppState) {
    state.dialogs.array_selection.close();
    if state.schematic.session.editor.tool == Tool::ArraySelection {
        state.schematic.cancel_tool();
    }
}

/// The three rows, each with the refusal that names it underneath. Returns
/// the field that takes focus: the first one with something wrong with it,
/// else the one at the top.
fn array_form(
    rows: &mut FormRows<'_>,
    validation: &DraftValidation,
    draft: &mut ArraySelectionDialogState,
) -> Option<Id> {
    let (array_invalid, count_invalid, naming_invalid) = validation.field_errors();
    let (array_error, count_error, naming_error) = validation.field_error_details();

    let labels = SchematicArrayKind::ALL.map(|kind| kind.label().to_owned());
    if let Some(index) = rows.select("Array", "array-selection-kind", draft.kind.label(), &labels) {
        let next = SchematicArrayKind::ALL[index];
        if next != draft.kind {
            draft.kind = next;
            draft.mark_edited();
        }
    }
    rows.field_line(None, error_or_blank(array_error));

    let count_id = field_id("count");
    if rows
        .text("Count", count_id, &mut draft.count, COUNT_HINT)
        .changed()
    {
        draft.count = canonical_count(&draft.count);
        draft.mark_edited();
    }
    rows.field_line(Some(count_id), error_or_blank(count_error));

    let naming_id = field_id("naming");
    if rows
        .text("Naming", naming_id, &mut draft.naming, NAMING_HINT)
        .changed()
    {
        draft.naming = canonical_naming(&draft.naming);
        draft.mark_edited();
    }
    rows.field_line(Some(naming_id), error_or_blank(naming_error));

    if array_invalid {
        None
    } else if count_invalid {
        Some(count_id)
    } else if naming_invalid {
        Some(naming_id)
    } else {
        Some(count_id)
    }
}

fn error_or_blank(error: Option<&str>) -> FieldLine<'_> {
    error.map_or(FieldLine::Blank, FieldLine::Error)
}

fn radial_documentation_center(state: &AppState) -> Option<Point> {
    let selection = &state.schematic.session.editor.selection;
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
    for note in &state.schematic.document().design_notes {
        if selection.has_design_note(note.id) {
            include(note.pos);
        }
    }
    for shape in &state.schematic.document().documentation_shapes {
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

    #[test]
    fn the_form_offers_every_array_kind_under_its_own_name() {
        assert_eq!(DEFAULT_COUNT, "8 \u{00d7} 1");
        assert_eq!(
            SchematicArrayKind::ALL.map(SchematicArrayKind::label),
            ["Linear", "Rectangular", "Radial documentation"]
        );
    }

    /// `×`, `…` and `·` are not on a keyboard. What a reader can type has to
    /// reach the model, and what it reaches has to be what the model prints.
    #[test]
    fn the_two_separators_are_typed_in_ascii_and_read_back_canonical() {
        assert_eq!(canonical_count("8x1"), "8\u{00d7}1");
        assert_eq!(canonical_count("2 X 4"), "2 \u{00d7} 4");
        assert_eq!(canonical_count("3*3"), "3\u{00d7}3");
        assert!(SchematicArrayCount::parse(&canonical_count("8 x 1")).is_ok());

        let typed = canonical_naming("U4...U11, DATA[0]...DATA[7]");
        assert_eq!(typed, "U4\u{2026}U11 \u{00b7} DATA[0]\u{2026}DATA[7]");
        assert!(SchematicArrayNaming::parse(&typed).is_ok());
        // A reference endpoint may contain `x`; only the count rewrites it.
        assert_eq!(canonical_naming("Mx1...Mx8"), "Mx1\u{2026}Mx8");
    }

    #[test]
    fn open_uses_exact_default_count_and_collision_free_naming() {
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);

        open_array_selection_dialog(&mut state);

        assert!(state.dialogs.array_selection.open);
        assert_eq!(state.dialogs.array_selection.count, DEFAULT_COUNT);
        assert!(!state.dialogs.array_selection.naming.is_empty());
        assert!(state.dialogs.array_selection.authority.is_some());
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
        state.schematic.document_mut_for_test().design_notes.push(
            DesignNote::new(
                11,
                Point::new(120, -40),
                DesignNoteKind::PlainText,
                "review boundary",
            )
            .unwrap(),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_design_note(11);

        open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.kind = SchematicArrayKind::RadialDocumentation;

        assert!(validate_draft(&state).can_commit());
        assert_eq!(
            radial_documentation_center(&state),
            Some(Point::new(120, -40))
        );
    }

    #[test]
    fn rectangular_array_highlights_only_the_incompatible_count_field() {
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.kind = SchematicArrayKind::Rectangular;

        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, true, false));
        // The refusal is the count row's, so nothing is said twice at the
        // foot of the form.
        assert_eq!(validation.form_refusal(), None);
    }

    #[test]
    fn malformed_naming_highlights_only_the_naming_field() {
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.naming = "R1 to R8".to_owned();

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
        state.schematic.document_mut_for_test().components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.authority = None;

        // No field is at fault, so the refusal stands at the foot of the form
        // rather than being pinned on a value the reader typed correctly.
        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, false, false));
        assert!(validation.form_refusal().is_some());
    }
}

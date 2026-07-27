//! Mockup-owned Create object array transaction.
//!
//! The three visible controls are the complete design contract. Placement is
//! acquired from the canvas after the retained draft has been parsed and the
//! immutable schematic authority has been revalidated.

use egui::{Align, Context, Frame, Layout, Margin, Ui, vec2};

use crate::state::{
    Point, SchematicArrayCount, SchematicArrayKind, SchematicArrayNaming, SchematicArrayPlacement,
    SchematicArrayPlan, Tool,
};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    select_mono_with_response,
};

use crate::workbench::app::dialogs::review_primitives::{
    BODY_HEIGHT as MOCKUP_BODY_HEIGHT, CONTEXT_WIDTH as MOCKUP_CONTEXT_WIDTH,
    SURFACE_HEIGHT as MOCKUP_SURFACE_HEIGHT, TRANSACTION_HEIGHT as MOCKUP_TRANSACTION_HEIGHT,
    configure_field_validation, field_label, input_field, paint_body_dividers, purpose_line,
    resolved_context,
};
use crate::workbench::app::dialogs::schematic_command::{DISCARD_DETAIL, DISCARD_TITLE};
use crate::workbench::app::{
    AppState, ArraySelectionDialogState, ConsoleMessage, RSpiceApp, SchematicEditAuthority,
};

const EYEBROW: &str = "SCHEMATIC \u{00b7} REPEATED STRUCTURE";
const TITLE: &str = "Create object array";
const PRIMARY: &str = "Create array";
const DESCRIPTION: &str =
    "Create a named linear or rectangular array with reference-designator and bus-index policies.";
const DEFAULT_COUNT: &str = "8 \u{00d7} 1";
const INVALID_DETAIL: &str = "Correct the highlighted values before this operation can continue. No project, result, or governed record has changed.";

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

    fn transaction_title(&self) -> Option<String> {
        let Self::Invalid(errors) = self else {
            return None;
        };
        let fields = usize::from(errors.array.is_some())
            .saturating_add(usize::from(errors.count.is_some()))
            .saturating_add(usize::from(errors.naming.is_some()));
        Some(match fields {
            0 => "Create array cannot continue".to_owned(),
            1 => "1 field requires attention".to_owned(),
            fields => format!("{fields} fields require attention"),
        })
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

    fn field_mask(&self) -> u8 {
        let (array, count, naming) = self.field_errors();
        u8::from(array) | (u8::from(count) << 1) | (u8::from(naming) << 2)
    }
}

const fn should_focus_first_invalid(previous: u8, current: u8) -> bool {
    previous == 0 && current != 0
}

pub(crate) fn open_array_selection_dialog(state: &mut AppState) {
    if state.schematic.read_only || state.active_view_read_only() {
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

impl RSpiceApp {
    pub(in crate::workbench::app) fn render_array_selection_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.array_selection.open {
            return;
        }
        let validation = validate_draft(&self.state);
        let validation_title = validation.transaction_title();
        let project_name = self.state.workspace.project.display_name().to_owned();
        let project_revision = self.state.workspace.project.revision().get().to_string();
        let validation_field_mask = validation.field_mask();
        let request_first_invalid_focus = should_focus_first_invalid(
            self.state.dialogs.array_selection.validation_field_mask,
            validation_field_mask,
        );
        self.state.dialogs.array_selection.validation_field_mask = validation_field_mask;
        let discard_confirm = self.state.dialogs.array_selection.discard_confirm;
        let retain_dirty_cancel = self.state.dialogs.array_selection.dirty && !discard_confirm;
        let has_transaction = discard_confirm || validation_title.is_some();
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .initial_height(
                MOCKUP_SURFACE_HEIGHT
                    + if has_transaction {
                        MOCKUP_TRANSACTION_HEIGHT
                    } else {
                        0.0
                    },
            )
            .flush_body()
            .ghost(if discard_confirm {
                "Discard changes"
            } else {
                "Cancel"
            })
            .primary_enabled(validation.can_commit())
            .primary_on_enter(false)
            .initial_focus(DialogInitialFocus::BodyControl);
        if retain_dirty_cancel {
            dialog = dialog.retain_on_cancel_focus(DialogInitialFocus::Ghost);
        }
        if discard_confirm {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                DISCARD_TITLE,
                DISCARD_DETAIL,
            );
        } else if let Some(title) = validation_title.as_deref() {
            dialog = dialog.transaction_state(DialogTransactionTone::Error, title, INVALID_DETAIL);
        }
        let mut first_invalid_focus = None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            let workflow = array_dialog_body(
                ui,
                &validation,
                &mut self.state.dialogs.array_selection,
                &project_name,
                &project_revision,
            );
            first_invalid_focus = workflow.first_invalid;
            Some(workflow.initial)
        });
        if request_first_invalid_focus
            && choice == DialogChoice::None
            && let Some(field) = first_invalid_focus
        {
            ctx.memory_mut(|memory| memory.request_focus(field));
        }
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
                    self.state.dialogs.array_selection.anchor = radial_center;
                    crate::workbench::commands::arm_schematic_tool(
                        &mut self.state.schematic,
                        Tool::ArraySelection,
                    );
                    crate::schematic::view::request_schematic_canvas_focus(ctx);
                    self.state.push_user_message(ConsoleMessage::info(
                        "Create array armed; choose the exact pitch or center on the schematic canvas."
                            .to_owned(),
                    ));
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.array_selection.attempt_close();
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
        let selection = &state.schematic.selection;
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
                state.schematic.grid_size.max(1),
                state.schematic.grid_size.max(1),
            )),
            SchematicArrayKind::Rectangular => SchematicArrayPlacement::Pitch(Point::new(
                state.schematic.grid_size.max(1),
                state.schematic.grid_size.max(1),
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
    if !state.dialogs.array_selection.armed || state.schematic.tool != Tool::ArraySelection {
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
    if state.schematic.tool == Tool::ArraySelection {
        crate::workbench::commands::cancel_schematic_tool(&mut state.schematic);
    }
}

#[derive(Debug, Clone, Copy)]
struct WorkflowFocus {
    initial: egui::Id,
    first_invalid: Option<egui::Id>,
}

fn array_dialog_body(
    ui: &mut Ui,
    validation: &DraftValidation,
    draft: &mut ArraySelectionDialogState,
    project_name: &str,
    project_revision: &str,
) -> WorkflowFocus {
    purpose_line(ui, DESCRIPTION);
    let (array_invalid, count_invalid, naming_invalid) = validation.field_errors();
    let (array_error, count_error, naming_error) = validation.field_error_details();
    let mut initial = None;
    let mut first_invalid = None;
    let width = ui.available_width();
    let context_width = MOCKUP_CONTEXT_WIDTH.min((width * 0.42).max(220.0));
    let form_width = (width - context_width).max(1.0);
    let body = ui.allocate_ui_with_layout(
        vec2(width, MOCKUP_BODY_HEIGHT),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.allocate_ui_with_layout(
                vec2(form_width, MOCKUP_BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(form_width, MOCKUP_BODY_HEIGHT));
                    Frame::NONE
                        .fill(Tokens::get(ui.ctx()).color.bg_app)
                        .inner_margin(Margin::symmetric(12, 10))
                        .show(ui, |ui| {
                            ui.set_width((form_width - 24.0).max(1.0));
                            let labels =
                                SchematicArrayKind::ALL.map(|kind| kind.label().to_owned());
                            let array = field_label(ui, "Array", |ui| {
                                select_mono_with_response(
                                    ui,
                                    "array-selection-kind",
                                    "Array",
                                    draft.kind.label(),
                                    &labels,
                                    ui.available_width(),
                                )
                            });
                            if let Some(index) = array.picked {
                                let next = SchematicArrayKind::ALL[index];
                                if next != draft.kind {
                                    draft.kind = next;
                                    draft.mark_edited();
                                }
                            }
                            configure_field_validation(
                                ui,
                                &array.response,
                                "Array",
                                array_error,
                                "Array kind is incompatible with the retained selection",
                            );
                            ui.add_space(9.0);
                            let count = input_field(
                                ui,
                                "Count",
                                &mut draft.count,
                                "8 \u{00d7} 1",
                                count_error,
                                "Array columns and rows, including the retained source member",
                            );
                            if count.changed() {
                                draft.mark_edited();
                            }
                            ui.add_space(9.0);
                            let naming = input_field(
                                ui,
                                "Naming",
                                &mut draft.naming,
                                "U4\u{2026}U11 \u{00b7} DATA[0]\u{2026}DATA[7]",
                                naming_error,
                                "Reference-designator and bus-index range policy",
                            );
                            if naming.changed() {
                                draft.mark_edited();
                            }
                            first_invalid = if array_invalid {
                                Some(array.response.id)
                            } else if count_invalid {
                                Some(count.id)
                            } else if naming_invalid {
                                Some(naming.id)
                            } else {
                                None
                            };
                            initial = Some(first_invalid.unwrap_or(array.response.id));
                        });
                },
            );
            ui.allocate_ui_with_layout(
                vec2(context_width, MOCKUP_BODY_HEIGHT),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.set_min_size(vec2(context_width, MOCKUP_BODY_HEIGHT));
                    resolved_context(ui, project_name, project_revision);
                },
            );
        },
    );
    paint_body_dividers(ui, body.response.rect, form_width);
    WorkflowFocus {
        initial: initial.unwrap_or_else(|| ui.id().with("array-selection-kind")),
        first_invalid,
    }
}

fn radial_documentation_center(state: &AppState) -> Option<Point> {
    let selection = &state.schematic.selection;
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
    for note in &state.schematic.design_notes {
        if selection.has_design_note(note.id) {
            include(note.pos);
        }
    }
    for shape in &state.schematic.documentation_shapes {
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
    fn restored_mockup_shell_contract_is_exact() {
        assert_eq!(EYEBROW, "SCHEMATIC \u{00b7} REPEATED STRUCTURE");
        assert_eq!(TITLE, "Create object array");
        assert_eq!(PRIMARY, "Create array");
        assert_eq!(DEFAULT_COUNT, "8 \u{00d7} 1");
        assert_eq!(MOCKUP_SURFACE_HEIGHT, 370.0);
        assert_eq!(
            crate::workbench::app::dialogs::review_primitives::PURPOSE_HEIGHT,
            35.0
        );
        assert_eq!(MOCKUP_BODY_HEIGHT, 230.0);
        assert_eq!(MOCKUP_CONTEXT_WIDTH, 260.0);
        assert_eq!(
            SchematicArrayKind::ALL.map(SchematicArrayKind::label),
            ["Linear", "Rectangular", "Radial documentation"]
        );
    }

    #[test]
    fn open_uses_exact_default_count_and_collision_free_naming() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state.schematic.selection.select_only_component(4);

        open_array_selection_dialog(&mut state);

        assert!(state.dialogs.array_selection.open);
        assert_eq!(state.dialogs.array_selection.count, DEFAULT_COUNT);
        assert!(!state.dialogs.array_selection.naming.is_empty());
        assert!(state.dialogs.array_selection.authority.is_some());
    }

    #[test]
    fn dirty_array_draft_requires_two_close_attempts() {
        let mut draft = ArraySelectionDialogState::default();
        draft.open = true;
        draft.count = "8 \u{00d7} 1".to_owned();
        draft.mark_edited();
        assert!(!draft.attempt_close());
        assert!(draft.discard_confirm);
        assert!(draft.attempt_close());
        assert!(!draft.open);
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
        assert!(draft.attempt_close());
    }

    #[test]
    fn radial_documentation_accepts_notes_and_starts_at_the_selection_center() {
        let mut state = AppState::default();
        state.schematic.design_notes.push(
            DesignNote::new(
                11,
                Point::new(120, -40),
                DesignNoteKind::PlainText,
                "review boundary",
            )
            .unwrap(),
        );
        state.schematic.selection.select_design_note(11);

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
        state.schematic.components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state.schematic.selection.select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.kind = SchematicArrayKind::Rectangular;

        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, true, false));
        assert_eq!(
            validation.transaction_title().as_deref(),
            Some("1 field requires attention")
        );
    }

    #[test]
    fn malformed_naming_highlights_only_the_naming_field() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state.schematic.selection.select_only_component(4);
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
    fn first_invalid_focus_is_requested_once_per_valid_to_invalid_transition() {
        assert!(!should_focus_first_invalid(0, 0));
        assert!(should_focus_first_invalid(0, 0b010));
        assert!(!should_focus_first_invalid(0b010, 0b010));
        assert!(!should_focus_first_invalid(0b010, 0b110));
        assert!(should_focus_first_invalid(0, 0b100));
    }

    #[test]
    fn stale_operation_is_not_misrepresented_as_a_field_error() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(4, ComponentType::Resistor, Point::origin()).with_name_value("R1", "1k"),
        );
        state.schematic.selection.select_only_component(4);
        open_array_selection_dialog(&mut state);
        state.dialogs.array_selection.authority = None;

        let validation = validate_draft(&state);
        assert_eq!(validation.field_errors(), (false, false, false));
        assert_eq!(
            validation.transaction_title().as_deref(),
            Some("Create array cannot continue")
        );
    }
}

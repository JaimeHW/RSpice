//! Display options: what the schematic canvas shows, on this device.
//!
//! One dialog for the schematic's overlays and labels and for the optional
//! drawing-sheet layers. It edits an isolated copy of both and publishes them
//! together on Apply; neither opening, cancelling, nor applying can create a
//! schematic undo record or modify project-owned design data, printing or
//! export. Wire routing is an editing behaviour, not a display one, and is
//! set only in Grid, snap and routing.

use egui::{Context, Id};

use crate::state::WireRoutingMode;
use crate::state::{
    SchematicAnnotationVisibility, SchematicBackAnnotationContent, SchematicNetHighlighting,
    SchematicParameterLabelVisibility, SchematicReviewMarkerVisibility, SchematicVisibilityPolicy,
    SchematicWireRoutingStyle,
};
use crate::ui::widgets::{CommandForm, DialogChoice, FormRows};

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

const TITLE: &str = "Display options";
const PRIMARY: &str = "Apply";
const DESCRIPTION: &str = "Choose what the schematic canvas shows on this device. The design, printing and export are unchanged.";

/// Open Display options. Both the schematic-visibility and the
/// drawing-sheet-layers commands land here; it is one dialog.
pub(crate) fn open_display_options(state: &mut AppState) -> bool {
    if state.dialogs.schematic_visibility.open {
        return false;
    }
    state
        .dialogs
        .schematic_visibility
        .open(state.ui.schematic_visibility, state.ui.drawing_sheet_layers);
    true
}

pub(crate) fn open_schematic_visibility_options(state: &mut AppState) -> bool {
    open_display_options(state)
}

pub(crate) fn open_drawing_sheet_layers_dialog(state: &mut AppState) -> bool {
    open_display_options(state)
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_schematic_visibility_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.schematic_visibility.open {
            return;
        }

        let recovery_available = visibility_recovery_available(&self.state);
        let mut form = CommandForm::new(TITLE, PRIMARY)
            .describe(DESCRIPTION)
            .status_fixed_at_open();
        if recovery_available {
            form = form.secondary("Undo last change");
        }
        let dialog_state = &mut self.state.dialogs.schematic_visibility;
        let choice = form.show(ctx, |rows| {
            Some(display_options_rows(
                rows,
                &mut dialog_state.draft,
                &mut dialog_state.layers_draft,
            ))
        });

        match choice {
            DialogChoice::Primary => apply_display_options(&mut self.state),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.schematic_visibility.close();
            }
            DialogChoice::Secondary => match restore_visibility_recovery(&mut self.state) {
                Ok(()) => {
                    self.state.dialogs.schematic_visibility.close();
                    self.state
                        .prompt("Restored the previous schematic visibility on this device.");
                }
                Err(error) => {
                    self.state.prompt(error);
                }
            },
            DialogChoice::None => {}
        }
    }
}

/// Publish both drafts together and close.
fn apply_display_options(state: &mut AppState) {
    let policy = state.dialogs.schematic_visibility.draft;
    if policy != state.ui.schematic_visibility {
        let recovery = capture_visibility_recovery(state);
        state.ui.schematic_visibility_recovery = Some(recovery);
    }
    publish_visibility_policy(state, policy);
    state.ui.drawing_sheet_layers = state.dialogs.schematic_visibility.layers_draft;
    state.dialogs.schematic_visibility.close();
    state.prompt("Display options applied on this device; the design is unchanged.");
}

fn capture_visibility_recovery(state: &AppState) -> crate::workbench::SchematicVisibilityRecovery {
    crate::workbench::SchematicVisibilityRecovery {
        view_path: state.workspace.content.active_view.display_path(),
        policy: state.ui.schematic_visibility,
        routing_mode: state.ui.schematic_routing_mode,
        net_highlight: state.schematic.session.editor.net_highlight.clone(),
        selection: state.schematic.session.editor.selection.clone(),
    }
}

fn visibility_recovery_available(state: &AppState) -> bool {
    state
        .ui
        .schematic_visibility_recovery
        .as_ref()
        .is_some_and(|recovery| {
            recovery.view_path == state.workspace.content.active_view.display_path()
        })
}

fn restore_visibility_recovery(state: &mut AppState) -> Result<(), String> {
    let recovery = state
        .ui
        .schematic_visibility_recovery
        .clone()
        .ok_or_else(|| "No prior schematic visibility state is available.".to_owned())?;
    if recovery.view_path != state.workspace.content.active_view.display_path() {
        return Err(
            "The active cell/view changed; its prior visibility state was not applied.".to_owned(),
        );
    }
    state.ui.schematic_visibility = recovery.policy;
    state.ui.schematic_routing_mode = recovery.routing_mode;
    state.schematic.session.editor.wire_drawing.routing_mode = recovery.routing_mode;
    state.schematic.session.editor.bus_drawing.routing_mode = recovery.routing_mode;
    state.schematic.session.editor.net_highlight = recovery.net_highlight;
    state.schematic.session.editor.selection = recovery.selection;
    let active_key = state.workspace.content.active_schematic_reference().key();
    if let Some(mut buffer) = state.workspace.schematic_editor_mut(&active_key) {
        buffer.editor.session.editor.selection = state.schematic.session.editor.selection.clone();
    }
    state.ui.schematic_visibility_recovery = None;
    Ok(())
}

pub(crate) fn publish_visibility_policy(state: &mut AppState, policy: SchematicVisibilityPolicy) {
    let current = state.schematic.session.editor.wire_drawing.routing_mode;
    let routing_mode = routing_mode_for(policy.wire_routing, current);
    state.schematic.session.editor.wire_drawing.routing_mode = routing_mode;
    state.schematic.session.editor.bus_drawing.routing_mode = routing_mode;
    state.ui.schematic_routing_mode = routing_mode;
    if !policy.net_highlighting.lights_selected_net() {
        state.schematic.session.editor.net_highlight.clear();
    }
    let selectable_note_ids = state
        .schematic
        .document()
        .design_notes
        .iter()
        .filter(|note| {
            if note.kind != crate::state::DesignNoteKind::ReviewNote {
                return true;
            }
            match policy.review_markers {
                SchematicReviewMarkerVisibility::Hidden => false,
                SchematicReviewMarkerVisibility::All => true,
                SchematicReviewMarkerVisibility::OpenAndAssigned => {
                    note.review.as_ref().is_some_and(|review| {
                        review.state == crate::state::DesignReviewState::Open
                            || review.assignee.is_some()
                    })
                }
            }
        })
        .map(|note| note.id)
        .collect::<std::collections::HashSet<_>>();
    state
        .schematic
        .session
        .editor
        .selection
        .design_notes
        .retain(|id| selectable_note_ids.contains(id));
    state.ui.schematic_visibility = policy;
}

fn routing_mode_for(style: SchematicWireRoutingStyle, current: WireRoutingMode) -> WireRoutingMode {
    match style {
        SchematicWireRoutingStyle::Orthogonal if current.is_orthogonal() => current,
        SchematicWireRoutingStyle::Orthogonal => WireRoutingMode::HorizontalFirst,
        SchematicWireRoutingStyle::FortyFiveDegree => WireRoutingMode::FortyFiveDegree,
        SchematicWireRoutingStyle::FreeAngle => WireRoutingMode::Diagonal,
    }
}

/// The rows of Display options. Returns the first control, which takes the
/// caret when the dialog opens.
fn display_options_rows(
    rows: &mut FormRows<'_>,
    draft: &mut SchematicVisibilityPolicy,
    layers: &mut crate::state::DrawingSheetLayerVisibility,
) -> Id {
    let options = |labels: &[&str]| {
        labels
            .iter()
            .map(|label| (*label).to_owned())
            .collect::<Vec<_>>()
    };

    let annotation = rows.select_with_response(
        "Annotations",
        "display-options-annotations",
        annotation_label(draft.annotations),
        &options(&[
            "OP voltages + selected currents",
            "Violations only",
            "Hidden",
        ]),
    );
    if let Some(index) = annotation.picked {
        draft.annotations = match index {
            1 => SchematicAnnotationVisibility::ViolationsOnly,
            2 => SchematicAnnotationVisibility::Hidden,
            _ => SchematicAnnotationVisibility::OperatingPoint,
        };
    }

    let back_annotation = rows.select(
        "Back-annotation",
        "display-options-back-annotation",
        back_annotation_label(draft.back_annotation),
        &options(&[
            "Net voltages + device currents",
            "Voltages only",
            "Voltages + currents + power",
        ]),
    );
    if let Some(index) = back_annotation {
        draft.back_annotation = match index {
            1 => SchematicBackAnnotationContent::VoltagesOnly,
            2 => SchematicBackAnnotationContent::VoltagesCurrentsAndPower,
            _ => SchematicBackAnnotationContent::NetVoltagesAndDeviceCurrents,
        };
    }

    let parameter_labels = rows.select(
        "Parameter labels",
        "display-options-parameter-labels",
        parameter_label(draft.parameter_labels),
        &options(&["Values only", "Names + values", "Hidden"]),
    );
    if let Some(index) = parameter_labels {
        draft.parameter_labels = match index {
            0 => SchematicParameterLabelVisibility::ValuesOnly,
            2 => SchematicParameterLabelVisibility::Hidden,
            _ => SchematicParameterLabelVisibility::NamesAndValues,
        };
    }

    let highlighting = rows.select(
        "Net highlighting",
        "display-options-net-highlighting",
        net_highlighting_label(draft.net_highlighting),
        &options(&[
            "Selected net",
            "Selected net across hierarchy",
            "Net class colors",
            "Off",
        ]),
    );
    if let Some(index) = highlighting {
        draft.net_highlighting = match index {
            1 => SchematicNetHighlighting::SelectedNetAcrossHierarchy,
            2 => SchematicNetHighlighting::NetClassColors,
            3 => SchematicNetHighlighting::Off,
            _ => SchematicNetHighlighting::SelectedNet,
        };
    }

    let review = rows.select(
        "Review markers",
        "display-options-review-markers",
        review_marker_label(draft.review_markers),
        &options(&["Open + assigned", "All", "Hidden"]),
    );
    if let Some(index) = review {
        draft.review_markers = match index {
            1 => SchematicReviewMarkerVisibility::All,
            2 => SchematicReviewMarkerVisibility::Hidden,
            _ => SchematicReviewMarkerVisibility::OpenAndAssigned,
        };
    }

    // The paper edge is not among them: it is always drawn.
    rows.checks(
        "Sheet layers",
        &mut [
            ("Zone references", &mut layers.zones),
            ("Title block", &mut layers.title_block),
            ("Centring marks", &mut layers.centring_marks),
            (
                "Dimensions at detail zoom",
                &mut layers.dimensions_at_detail_zoom,
            ),
        ],
    );

    annotation.response.id
}

const fn annotation_label(value: SchematicAnnotationVisibility) -> &'static str {
    match value {
        SchematicAnnotationVisibility::OperatingPoint => "OP voltages + selected currents",
        SchematicAnnotationVisibility::ViolationsOnly => "Violations only",
        SchematicAnnotationVisibility::Hidden => "Hidden",
    }
}

const fn back_annotation_label(value: SchematicBackAnnotationContent) -> &'static str {
    match value {
        SchematicBackAnnotationContent::NetVoltagesAndDeviceCurrents => {
            "Net voltages + device currents"
        }
        SchematicBackAnnotationContent::VoltagesOnly => "Voltages only",
        SchematicBackAnnotationContent::VoltagesCurrentsAndPower => "Voltages + currents + power",
    }
}

const fn parameter_label(value: SchematicParameterLabelVisibility) -> &'static str {
    match value {
        SchematicParameterLabelVisibility::ValuesOnly => "Values only",
        SchematicParameterLabelVisibility::NamesAndValues => "Names + values",
        SchematicParameterLabelVisibility::Hidden => "Hidden",
    }
}

const fn net_highlighting_label(value: SchematicNetHighlighting) -> &'static str {
    match value {
        SchematicNetHighlighting::SelectedNet => "Selected net",
        SchematicNetHighlighting::SelectedNetAcrossHierarchy => "Selected net across hierarchy",
        SchematicNetHighlighting::NetClassColors => "Net class colors",
        SchematicNetHighlighting::Off => "Off",
    }
}

const fn review_marker_label(value: SchematicReviewMarkerVisibility) -> &'static str {
    match value {
        SchematicReviewMarkerVisibility::OpenAndAssigned => "Open + assigned",
        SchematicReviewMarkerVisibility::All => "All",
        SchematicReviewMarkerVisibility::Hidden => "Hidden",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The visibility and the drawing-sheet-layers commands open one
    /// dialog, which applies both together and offers no wire routing: that
    /// is set in Grid, snap and routing alone.
    #[test]
    fn one_display_options_dialog_owns_visibility_and_sheet_layers() {
        let mut app = RSpiceApp::test_instance();
        assert!(open_drawing_sheet_layers_dialog(&mut app.state));
        assert!(
            !open_schematic_visibility_options(&mut app.state),
            "both commands open the same dialog"
        );

        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        // A later pass publishes only the nodes that changed, so the tree is
        // what every pass published.
        let mut labels = String::new();
        for _ in 0..2 {
            let output = ctx.run_ui(egui::RawInput::default(), |ui| {
                app.render_schematic_visibility_dialog(ui.ctx());
            });
            if let Some(update) = output.platform_output.accesskit_update {
                for text in update
                    .nodes
                    .iter()
                    .flat_map(|(_, node)| [node.label(), node.value()])
                    .flatten()
                {
                    labels.push_str(&text);
                    labels.push('\n');
                }
            }
        }
        for row in [
            "Annotations",
            "Net highlighting",
            "Zone references",
            "Title block",
        ] {
            assert!(labels.contains(row), "{row} is missing:\n{labels}");
        }
        assert!(
            !labels.contains("Wire routing"),
            "wire routing is not a display option:\n{labels}"
        );

        app.state.dialogs.schematic_visibility.layers_draft.zones = false;
        app.state
            .dialogs
            .schematic_visibility
            .draft
            .parameter_labels = SchematicParameterLabelVisibility::Hidden;
        apply_display_options(&mut app.state);
        assert!(!app.state.ui.drawing_sheet_layers.zones);
        assert_eq!(
            app.state.ui.schematic_visibility.parameter_labels,
            SchematicParameterLabelVisibility::Hidden
        );
        assert!(!app.state.dialogs.schematic_visibility.open);
    }

    #[test]
    fn orthogonal_publication_preserves_the_users_axis_order() {
        assert_eq!(
            routing_mode_for(
                SchematicWireRoutingStyle::Orthogonal,
                WireRoutingMode::VerticalFirst
            ),
            WireRoutingMode::VerticalFirst
        );
    }

    #[test]
    fn non_orthogonal_styles_map_exactly_to_the_wire_engine() {
        assert_eq!(
            routing_mode_for(
                SchematicWireRoutingStyle::FortyFiveDegree,
                WireRoutingMode::HorizontalFirst
            ),
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            routing_mode_for(
                SchematicWireRoutingStyle::FreeAngle,
                WireRoutingMode::HorizontalFirst
            ),
            WireRoutingMode::Diagonal
        );
    }

    #[test]
    fn applying_visibility_changes_only_session_owned_runtime_state() {
        let mut state = AppState::default();
        let design_before =
            serde_json::to_value(&state.schematic).expect("serialize project-owned schematic");
        let topology_before = state.schematic.topology_version();
        let policy = SchematicVisibilityPolicy {
            wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
            net_highlighting: SchematicNetHighlighting::Off,
            parameter_labels: SchematicParameterLabelVisibility::Hidden,
            ..SchematicVisibilityPolicy::default()
        };

        publish_visibility_policy(&mut state, policy);

        assert_eq!(state.ui.schematic_visibility, policy);
        assert_eq!(
            state.schematic.session.editor.wire_drawing.routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            state.schematic.session.editor.bus_drawing.routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(state.schematic.topology_version(), topology_before);
        assert!(!state.schematic.can_undo());
        assert_eq!(
            serde_json::to_value(&state.schematic).expect("serialize schematic after apply"),
            design_before
        );
    }

    #[test]
    fn visibility_recovery_restores_policy_routing_highlight_and_selection() {
        let mut state = AppState::default();
        let selected = state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::origin(),
        );
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(selected);
        state.schematic.session.editor.net_highlight.active = true;
        state
            .schematic
            .session
            .editor
            .net_highlight
            .highlighted_wires
            .insert(91);
        let recovery = capture_visibility_recovery(&state);
        state.ui.schematic_visibility_recovery = Some(recovery);

        publish_visibility_policy(
            &mut state,
            SchematicVisibilityPolicy {
                wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
                net_highlighting: SchematicNetHighlighting::Off,
                ..SchematicVisibilityPolicy::default()
            },
        );
        state.schematic.session.editor.selection.clear();
        restore_visibility_recovery(&mut state).expect("restore device-local view");

        assert_eq!(
            state.ui.schematic_visibility,
            SchematicVisibilityPolicy::default()
        );
        assert_eq!(
            state.schematic.session.editor.wire_drawing.routing_mode,
            WireRoutingMode::HorizontalFirst
        );
        assert!(state.schematic.session.editor.net_highlight.active);
        assert!(
            state
                .schematic
                .session
                .editor
                .net_highlight
                .highlighted_wires
                .contains(&91)
        );
        assert!(
            state
                .schematic
                .session
                .editor
                .selection
                .has_component(selected)
        );
        assert!(state.ui.schematic_visibility_recovery.is_none());
    }

    /// The default highlight lights the selected net in the open view only:
    /// nothing carries it into a child or back up to the parent, so it is not
    /// named for the hierarchy. A preference saved under the old name still
    /// reads as this policy.
    #[test]
    fn the_net_highlight_policy_is_named_for_what_it_lights() {
        let selected = SchematicNetHighlighting::default();
        assert_eq!(net_highlighting_label(selected), "Selected net");
        assert_eq!(
            serde_json::from_str::<SchematicNetHighlighting>("\"selected-across-hierarchy\"")
                .expect("a saved policy reads"),
            selected
        );
        assert_eq!(
            serde_json::to_string(&selected).expect("the policy writes"),
            "\"selected-net\""
        );
    }
}

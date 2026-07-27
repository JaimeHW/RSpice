//! Device-local schematic hierarchy and annotation visibility workflow.
//!
//! The mockup defines this as a display-only transaction. The dialog edits an
//! isolated copy and publishes all seven choices together; neither opening,
//! cancelling, nor applying can create a schematic undo record or modify
//! project-owned design data.

use egui::{Context, Frame, Margin, Stroke, Ui, vec2};

use crate::state::WireRoutingMode;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, select_mono_with_response,
};
use crate::state::{
    SchematicAnnotationVisibility, SchematicBackAnnotationContent, SchematicHierarchyVisibility,
    SchematicNetHighlighting, SchematicParameterLabelVisibility, SchematicReviewMarkerVisibility,
    SchematicVisibilityPolicy, SchematicWireRoutingStyle,
};

use crate::common::app::{AppState, ConsoleMessage, RSpiceApp};

const EYEBROW: &str = "VIEW \u{00b7} DISPLAY ONLY";
const TITLE: &str = "Hierarchy and annotation visibility";
const PRIMARY: &str = "Apply visibility";
const BODY: &str = "Control hierarchy context, operating-point annotations, net names, probes, comments, and physical markers without changing design data.";

pub(crate) fn open_schematic_visibility_options(state: &mut AppState) -> bool {
    if state.dialogs.schematic_visibility.open {
        return false;
    }
    state
        .dialogs
        .schematic_visibility
        .open(state.ui.schematic_visibility);
    true
}

impl RSpiceApp {
    pub(in crate::common::app) fn render_schematic_visibility_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.schematic_visibility.open {
            return;
        }

        let dirty = self.state.dialogs.schematic_visibility.dirty();
        let recovery_available = visibility_recovery_available(&self.state);
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(BODY)
            .size(DialogSize::Transaction)
            .ghost(if dirty { "Discard changes" } else { "Cancel" })
            .initial_focus(DialogInitialFocus::BodyControl);
        if recovery_available {
            dialog = dialog.secondary("Undo last visibility change");
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            Some(visibility_body(
                ui,
                &mut self.state.dialogs.schematic_visibility.draft,
            ))
        });

        match choice {
            DialogChoice::Primary => {
                let policy = self.state.dialogs.schematic_visibility.draft;
                if policy != self.state.ui.schematic_visibility {
                    let recovery = capture_visibility_recovery(&self.state);
                    self.state.ui.schematic_visibility_recovery = Some(recovery);
                }
                publish_visibility_policy(&mut self.state, policy);
                self.state.dialogs.schematic_visibility.close();
                self.state.push_user_message(ConsoleMessage::info(
                    "Schematic visibility was applied to this device session; design data was unchanged."
                        .to_owned(),
                ));
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.schematic_visibility.close();
            }
            DialogChoice::Secondary => match restore_visibility_recovery(&mut self.state) {
                Ok(()) => {
                    self.state.dialogs.schematic_visibility.close();
                    self.state.push_user_message(ConsoleMessage::info(
                        "Restored the prior device-local schematic visibility state.".to_owned(),
                    ));
                }
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::warning(error));
                }
            },
            DialogChoice::None => {}
        }
    }
}

fn capture_visibility_recovery(state: &AppState) -> crate::workbench::SchematicVisibilityRecovery {
    crate::workbench::SchematicVisibilityRecovery {
        view_path: state.workspace.active_view.display_path(),
        policy: state.ui.schematic_visibility,
        routing_mode: state.ui.schematic_routing_mode,
        net_highlight: state.schematic.net_highlight.clone(),
        selection: state.schematic.selection.clone(),
    }
}

fn visibility_recovery_available(state: &AppState) -> bool {
    state
        .ui
        .schematic_visibility_recovery
        .as_ref()
        .is_some_and(|recovery| recovery.view_path == state.workspace.active_view.display_path())
}

fn restore_visibility_recovery(state: &mut AppState) -> Result<(), String> {
    let recovery = state
        .ui
        .schematic_visibility_recovery
        .clone()
        .ok_or_else(|| "No prior schematic visibility state is available.".to_owned())?;
    if recovery.view_path != state.workspace.active_view.display_path() {
        return Err(
            "The active cell/view changed; its prior visibility state was not applied.".to_owned(),
        );
    }
    state.ui.schematic_visibility = recovery.policy;
    state.ui.schematic_routing_mode = recovery.routing_mode;
    state.schematic.wire_drawing.routing_mode = recovery.routing_mode;
    state.schematic.bus_drawing.routing_mode = recovery.routing_mode;
    state.schematic.net_highlight = recovery.net_highlight;
    state.schematic.selection = recovery.selection;
    let active_key = state.workspace.active_schematic_reference().key();
    if let Some(buffer) = state.workspace.schematic_buffers.get_mut(&active_key) {
        buffer.selection = state.schematic.selection.clone();
    }
    state.ui.schematic_visibility_recovery = None;
    Ok(())
}

pub(crate) fn publish_visibility_policy(state: &mut AppState, policy: SchematicVisibilityPolicy) {
    let current = state.schematic.wire_drawing.routing_mode;
    let routing_mode = routing_mode_for(policy.wire_routing, current);
    state.schematic.wire_drawing.routing_mode = routing_mode;
    state.schematic.bus_drawing.routing_mode = routing_mode;
    state.ui.schematic_routing_mode = routing_mode;
    if policy.net_highlighting != SchematicNetHighlighting::SelectedAcrossHierarchy {
        state.schematic.net_highlight.clear();
    }
    let selectable_note_ids = state
        .schematic
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

fn visibility_body(ui: &mut Ui, draft: &mut SchematicVisibilityPolicy) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    let first = enum_row(
        ui,
        "Hierarchy",
        "Visible edit context around the active cell view.",
        "schematic-visibility-hierarchy",
        hierarchy_label(draft.hierarchy),
        &[
            "Active + one parent level",
            "Active only",
            "Full visible hierarchy",
        ],
    );
    if let Some(index) = first.0 {
        draft.hierarchy = match index {
            1 => SchematicHierarchyVisibility::ActiveOnly,
            2 => SchematicHierarchyVisibility::FullVisibleHierarchy,
            _ => SchematicHierarchyVisibility::ActiveAndParent,
        };
    }

    let annotation = enum_row(
        ui,
        "Annotations",
        "Mutually exclusive result or verification overlay.",
        "schematic-visibility-annotations",
        annotation_label(draft.annotations),
        &[
            "OP voltages + selected currents",
            "Violations only",
            "Hidden",
        ],
    );
    if let Some(index) = annotation.0 {
        draft.annotations = match index {
            1 => SchematicAnnotationVisibility::ViolationsOnly,
            2 => SchematicAnnotationVisibility::Hidden,
            _ => SchematicAnnotationVisibility::OperatingPoint,
        };
    }

    let back_annotation = enum_row(
        ui,
        "Back-annotation content",
        "Quantities from the explicitly selected retained run.",
        "schematic-visibility-back-annotation",
        back_annotation_label(draft.back_annotation),
        &[
            "Net voltages + device currents",
            "Voltages only",
            "Voltages + currents + power",
        ],
    );
    if let Some(index) = back_annotation.0 {
        draft.back_annotation = match index {
            1 => SchematicBackAnnotationContent::VoltagesOnly,
            2 => SchematicBackAnnotationContent::VoltagesCurrentsAndPower,
            _ => SchematicBackAnnotationContent::NetVoltagesAndDeviceCurrents,
        };
    }

    let parameter_labels = enum_row(
        ui,
        "Parameter labels",
        "Instance identity and value labels painted on the canvas.",
        "schematic-visibility-parameter-labels",
        parameter_label(draft.parameter_labels),
        &["Values only", "Names + values", "Hidden"],
    );
    if let Some(index) = parameter_labels.0 {
        draft.parameter_labels = match index {
            0 => SchematicParameterLabelVisibility::ValuesOnly,
            2 => SchematicParameterLabelVisibility::Hidden,
            _ => SchematicParameterLabelVisibility::NamesAndValues,
        };
    }

    let routing = enum_row(
        ui,
        "Wire routing style",
        "Shared interactive routing for wires and buses.",
        "schematic-visibility-wire-routing",
        routing_label(draft.wire_routing),
        &["Orthogonal", "45\u{00b0} diagonal", "Free angle"],
    );
    if let Some(index) = routing.0 {
        draft.wire_routing = match index {
            1 => SchematicWireRoutingStyle::FortyFiveDegree,
            2 => SchematicWireRoutingStyle::FreeAngle,
            _ => SchematicWireRoutingStyle::Orthogonal,
        };
    }

    let highlighting = enum_row(
        ui,
        "Net highlighting",
        "Temporary highlighting inside the available hierarchy geometry.",
        "schematic-visibility-net-highlighting",
        net_highlighting_label(draft.net_highlighting),
        &["Selected net across hierarchy", "Net class colors", "Off"],
    );
    if let Some(index) = highlighting.0 {
        draft.net_highlighting = match index {
            1 => SchematicNetHighlighting::NetClassColors,
            2 => SchematicNetHighlighting::Off,
            _ => SchematicNetHighlighting::SelectedAcrossHierarchy,
        };
    }

    let review = enum_row(
        ui,
        "Review markers",
        "Governed review-note markers visible in this session.",
        "schematic-visibility-review-markers",
        review_marker_label(draft.review_markers),
        &["Open + assigned", "All", "Hidden"],
    );
    if let Some(index) = review.0 {
        draft.review_markers = match index {
            1 => SchematicReviewMarkerVisibility::All,
            2 => SchematicReviewMarkerVisibility::Hidden,
            _ => SchematicReviewMarkerVisibility::OpenAndAssigned,
        };
    }

    first.1
}

fn enum_row(
    ui: &mut Ui,
    title: &str,
    detail: &str,
    id: &str,
    selected: &str,
    options: &[&str],
) -> (Option<usize>, egui::Id) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let options = options
        .iter()
        .map(|option| (*option).to_owned())
        .collect::<Vec<_>>();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            let narrow = ui.available_width() < 520.0;
            if narrow {
                setting_copy(ui, title, detail);
                ui.add_space(7.0);
                let control_width = ui.available_width();
                let output =
                    select_mono_with_response(ui, id, title, selected, &options, control_width);
                (output.picked, output.response.id)
            } else {
                ui.horizontal(|ui| {
                    let copy_width = (ui.available_width() * 0.43).clamp(190.0, 270.0);
                    ui.allocate_ui_with_layout(
                        vec2(copy_width, t.metrics.ctl_h),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| setting_copy(ui, title, detail),
                    );
                    ui.add_space(12.0);
                    let control_width = ui.available_width();
                    let output =
                        select_mono_with_response(ui, id, title, selected, &options, control_width);
                    (output.picked, output.response.id)
                })
                .inner
            }
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    row.inner
}

fn setting_copy(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(title)
            .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text),
    );
    ui.label(
        egui::RichText::new(detail)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

const fn hierarchy_label(value: SchematicHierarchyVisibility) -> &'static str {
    match value {
        SchematicHierarchyVisibility::ActiveAndParent => "Active + one parent level",
        SchematicHierarchyVisibility::ActiveOnly => "Active only",
        SchematicHierarchyVisibility::FullVisibleHierarchy => "Full visible hierarchy",
    }
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

const fn routing_label(value: SchematicWireRoutingStyle) -> &'static str {
    match value {
        SchematicWireRoutingStyle::Orthogonal => "Orthogonal",
        SchematicWireRoutingStyle::FortyFiveDegree => "45\u{00b0} diagonal",
        SchematicWireRoutingStyle::FreeAngle => "Free angle",
    }
}

const fn net_highlighting_label(value: SchematicNetHighlighting) -> &'static str {
    match value {
        SchematicNetHighlighting::SelectedAcrossHierarchy => "Selected net across hierarchy",
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
            state.schematic.wire_drawing.routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            state.schematic.bus_drawing.routing_mode,
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
        state.schematic.selection.select_only_component(selected);
        state.schematic.net_highlight.active = true;
        state.schematic.net_highlight.highlighted_wires.insert(91);
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
        state.schematic.selection.clear();
        restore_visibility_recovery(&mut state).expect("restore device-local view");

        assert_eq!(
            state.ui.schematic_visibility,
            SchematicVisibilityPolicy::default()
        );
        assert_eq!(
            state.schematic.wire_drawing.routing_mode,
            WireRoutingMode::HorizontalFirst
        );
        assert!(state.schematic.net_highlight.active);
        assert!(
            state
                .schematic
                .net_highlight
                .highlighted_wires
                .contains(&91)
        );
        assert!(state.schematic.selection.has_component(selected));
        assert!(state.ui.schematic_visibility_recovery.is_none());
    }
}

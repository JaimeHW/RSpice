//! Transactional schematic grid, snap, and wire-routing settings.
//!
//! Grid display is device-local presentation state, snap targets are exact
//! runtime interaction state, and grid pitch is project-owned document policy.
//! Keeping one isolated draft across those owners gives the upgraded mockup a
//! real Apply/Cancel boundary and prevents partially applied canvas settings.

use egui::{Context, Frame, Margin, Stroke, Ui, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::state::{GridStyle, SchematicWireRoutingStyle, WireRoutingMode};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    select_mono_with_response,
};
use crate::workbench::app::{
    GridSnapRoutingDialogState, GridSnapRoutingDraft, GridSnapRoutingFocusTarget,
    GridSnapSpacingChoice, RSpiceApp, SchematicEditAuthority,
};
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "SCHEMATIC \u{00b7} EDIT BEHAVIOUR";
const TITLE: &str = "Grid, snap and wire routing";
const PRIMARY: &str = "Apply canvas settings";
const DESCRIPTION: &str = "Grid display is what you see; snap pitch and snap targets are what the cursor commits to. They are set separately because a coarse grid with fine snapping is a normal way to work.";
const ERROR_TITLE: &str = "Canvas settings were not applied";
const GRID_STYLES: [GridStyle; 3] = [GridStyle::Dots, GridStyle::Lines, GridStyle::Off];

pub(crate) fn open_grid_snap_routing_dialog(state: &mut AppState) -> bool {
    if state.dialogs.grid_snap_routing.open {
        return false;
    }
    if state.schematic.canvas_settings_change_blocked() {
        state.push_user_message(ConsoleMessage::warning(
            "Finish or cancel the active canvas authoring gesture before changing snap or routing settings.",
        ));
        return false;
    }

    let spacing = if state.schematic.snap_engine.enabled {
        GridSnapSpacingChoice::from_pitch(state.schematic.document_policy.grid_pitch)
    } else {
        GridSnapSpacingChoice::Free
    };
    let draft = GridSnapRoutingDraft {
        grid_style: state.ui.grid,
        snap_spacing: spacing,
        snap_engine: state.schematic.snap_engine.clone(),
        wire_routing: state.schematic.wire_drawing.routing_mode,
    };
    let authority = SchematicEditAuthority::capture(state);
    state.dialogs.grid_snap_routing.open(draft, authority);
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_grid_snap_routing_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.grid_snap_routing.open {
            return;
        }

        let dirty = self.state.dialogs.grid_snap_routing.dirty();
        let read_only = self.state.schematic_edit_read_only();
        let validation_error = self
            .state
            .dialogs
            .grid_snap_routing
            .validation_error
            .clone();
        let mut dialog = Dialog::new(EYEBROW, TITLE, PRIMARY)
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .ghost(if dirty { "Discard changes" } else { "Cancel" })
            .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(error) = validation_error.as_deref() {
            dialog = dialog.transaction_state(DialogTransactionTone::Error, ERROR_TITLE, error);
        }
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            Some(grid_snap_routing_body(
                ui,
                &mut self.state.dialogs.grid_snap_routing,
                read_only,
            ))
        });

        match choice {
            DialogChoice::Primary => {
                if let Err(error) = commit_grid_snap_routing(&mut self.state) {
                    self.state.dialogs.grid_snap_routing.validation_error = Some(error.message);
                    self.state.dialogs.grid_snap_routing.focus_target = error.focus_target;
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.grid_snap_routing.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

#[derive(Debug)]
struct GridSnapRoutingCommitError {
    message: String,
    focus_target: Option<GridSnapRoutingFocusTarget>,
}

impl GridSnapRoutingCommitError {
    fn new(message: impl Into<String>, focus_target: Option<GridSnapRoutingFocusTarget>) -> Self {
        Self {
            message: message.into(),
            focus_target,
        }
    }
}

fn commit_grid_snap_routing(state: &mut AppState) -> Result<(), GridSnapRoutingCommitError> {
    let transaction = state.dialogs.grid_snap_routing.clone();
    if state.schematic.canvas_settings_change_blocked() {
        return Err(GridSnapRoutingCommitError::new(
            "Finish or cancel the active canvas authoring gesture before applying snap or routing settings.",
            None,
        ));
    }
    let authority = transaction.authority.as_ref().ok_or_else(|| {
        GridSnapRoutingCommitError::new("The canvas-settings authority is missing.", None)
    })?;
    authority
        .validate_presentation(state, TITLE)
        .map_err(|message| GridSnapRoutingCommitError::new(message, None))?;

    let requested_pitch = transaction.draft.snap_spacing.pitch();
    let pitch_changed =
        requested_pitch.is_some_and(|pitch| pitch != state.schematic.document_policy.grid_pitch);
    if pitch_changed && state.schematic_edit_read_only() {
        return Err(GridSnapRoutingCommitError::new(
            "Snap spacing is project-owned and cannot be changed in this read-only view.",
            Some(GridSnapRoutingFocusTarget::SnapSpacing),
        ));
    }
    if requested_pitch.is_some() && !any_snap_target_enabled(&transaction.draft.snap_engine) {
        return Err(GridSnapRoutingCommitError::new(
            "Enable at least one snap target, or choose Free snap spacing.",
            Some(GridSnapRoutingFocusTarget::SnapTargets),
        ));
    }

    // Finish the complete candidate before mutating any owner. `Free` disables
    // snapping while retaining the governed document pitch for later reuse.
    let mut snap_engine = transaction.draft.snap_engine.clone();
    snap_engine.enabled = requested_pitch.is_some();
    snap_engine.grid_size = requested_pitch
        .unwrap_or(state.schematic.document_policy.grid_pitch)
        .canvas_grid_size();
    let routing_mode = canonical_routing_mode(
        transaction.draft.wire_routing,
        state.schematic.wire_drawing.routing_mode,
    );

    if let Some(pitch) = requested_pitch.filter(|_| pitch_changed) {
        let changed = state
            .schematic
            .with_undo("change schematic grid pitch", |document| {
                document.document_policy.grid_pitch = pitch;
                document.grid_size = pitch.canvas_grid_size();
                document.is_dirty = true;
                document.bump_topology_version();
            });
        if !changed {
            return Err(GridSnapRoutingCommitError::new(
                "The schematic grid pitch could not be changed.",
                Some(GridSnapRoutingFocusTarget::SnapSpacing),
            ));
        }
    }

    state.ui.set_grid_style(transaction.draft.grid_style);
    state.schematic.snap_engine = snap_engine.clone();
    state.ui.schematic_snap = snap_engine;
    state.schematic.wire_drawing.set_routing_mode(routing_mode);
    state.schematic.bus_drawing.routing_mode = routing_mode;
    state.ui.schematic_routing_mode = routing_mode;
    state.ui.schematic_visibility.wire_routing = routing_style(routing_mode);
    if pitch_changed {
        state.sync_active_schematic_to_workspace();
    }
    state.dialogs.grid_snap_routing.close();
    state.push_user_message(ConsoleMessage::info(
        "Canvas grid, snap targets, and wire routing were applied atomically.".to_owned(),
    ));
    Ok(())
}

const fn any_snap_target_enabled(snap: &crate::state::SnapEngine) -> bool {
    snap.snap_to_grid
        || snap.snap_to_terminals
        || snap.snap_to_junctions
        || snap.snap_to_wire_endpoints
        || snap.snap_to_wire_segments
}

fn canonical_routing_mode(requested: WireRoutingMode, current: WireRoutingMode) -> WireRoutingMode {
    if requested.is_orthogonal() && current.is_orthogonal() {
        current
    } else if requested.is_orthogonal() {
        WireRoutingMode::HorizontalFirst
    } else {
        requested
    }
}

const fn routing_style(mode: WireRoutingMode) -> SchematicWireRoutingStyle {
    match mode {
        WireRoutingMode::HorizontalFirst | WireRoutingMode::VerticalFirst => {
            SchematicWireRoutingStyle::Orthogonal
        }
        WireRoutingMode::FortyFiveDegree => SchematicWireRoutingStyle::FortyFiveDegree,
        WireRoutingMode::Diagonal => SchematicWireRoutingStyle::FreeAngle,
    }
}

fn grid_snap_routing_body(
    ui: &mut Ui,
    transaction: &mut GridSnapRoutingDialogState,
    read_only: bool,
) -> egui::Id {
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    let grid = enum_row(
        ui,
        "Grid display",
        "Visual canvas grid; this never changes snapping.",
        "grid-snap-routing-display",
        grid_style_label(transaction.draft.grid_style),
        &["Dots", "Lines", "Off"],
    );
    if let Some(index) = grid.0 {
        transaction.draft.grid_style = GRID_STYLES[index];
        transaction.validation_error = None;
    }

    let spacing = enum_row(
        ui,
        "Snap spacing",
        if read_only {
            "Free is session-only; changing the document pitch is locked in this view."
        } else {
            "Independent interaction pitch; Free disables the snap engine."
        },
        "grid-snap-routing-spacing",
        transaction.draft.snap_spacing.label(),
        &["Free", "25 mil", "50 mil", "Metric \u{00b7} 0.5 mm"],
    );
    if let Some(index) = spacing.0 {
        transaction.draft.snap_spacing = GridSnapSpacingChoice::ALL[index];
        transaction.draft.snap_engine.enabled =
            transaction.draft.snap_spacing != GridSnapSpacingChoice::Free;
        transaction.validation_error = None;
    }

    let (snap_target_id, snap_targets_changed) = snap_targets_row(ui, &mut transaction.draft);
    if snap_targets_changed {
        transaction.validation_error = None;
    }

    let routing = enum_row(
        ui,
        "Wire routing",
        "Shared interactive routing policy for wires and buses.",
        "grid-snap-routing-wire-routing",
        routing_label(transaction.draft.wire_routing),
        &["Orthogonal", "45\u{00b0}", "Free"],
    );
    if let Some(index) = routing.0 {
        transaction.draft.wire_routing = match index {
            1 => WireRoutingMode::FortyFiveDegree,
            2 => WireRoutingMode::Diagonal,
            _ if transaction.draft.wire_routing.is_orthogonal() => transaction.draft.wire_routing,
            _ => WireRoutingMode::HorizontalFirst,
        };
        transaction.validation_error = None;
    }

    if let Some(target) = transaction.focus_target.take() {
        let id = match target {
            GridSnapRoutingFocusTarget::SnapSpacing => spacing.1,
            GridSnapRoutingFocusTarget::SnapTargets => snap_target_id,
        };
        ui.memory_mut(|memory| memory.request_focus(id));
    }
    grid.1
}

const fn grid_style_label(style: GridStyle) -> &'static str {
    match style {
        GridStyle::Dots => "Dots",
        GridStyle::Lines => "Lines",
        GridStyle::Off => "Off",
    }
}

fn snap_targets_row(ui: &mut Ui, draft: &mut GridSnapRoutingDraft) -> (egui::Id, bool) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let row = Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.set_min_width((width - 24.0).max(1.0));
            setting_copy(
                ui,
                "Snap targets",
                "Exact geometry classes considered by the snap engine.",
            );
            ui.add_space(7.0);
            ui.add_enabled_ui(draft.snap_spacing != GridSnapSpacingChoice::Free, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let first =
                        ui.checkbox(&mut draft.snap_engine.snap_to_grid, "Grid intersections");
                    let mut changed = first.changed();
                    changed |= ui
                        .checkbox(&mut draft.snap_engine.snap_to_terminals, "Terminals")
                        .changed();
                    changed |= ui
                        .checkbox(&mut draft.snap_engine.snap_to_junctions, "Junctions")
                        .changed();
                    changed |= ui
                        .checkbox(
                            &mut draft.snap_engine.snap_to_wire_endpoints,
                            "Wire endpoints",
                        )
                        .changed();
                    changed |= ui
                        .checkbox(
                            &mut draft.snap_engine.snap_to_wire_segments,
                            "Wire segments",
                        )
                        .changed();
                    (first.id, changed)
                })
                .inner
            })
            .inner
        });
    ui.painter().hline(
        row.response.rect.x_range(),
        row.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    row.inner
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
            if ui.available_width() < 520.0 {
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

const fn routing_label(mode: WireRoutingMode) -> &'static str {
    match mode {
        WireRoutingMode::HorizontalFirst | WireRoutingMode::VerticalFirst => "Orthogonal",
        WireRoutingMode::FortyFiveDegree => "45\u{00b0}",
        WireRoutingMode::Diagonal => "Free",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{SchematicGridPitch, SchematicWireRoutingStyle, Tool};
    use crate::workbench::state::LocalSafeModeOptions;

    #[test]
    fn open_captures_an_isolated_exact_draft_and_models_free_without_fake_pitch() {
        let mut state = AppState::default();
        state.ui.set_grid_style(GridStyle::Lines);
        state.schematic.snap_engine.enabled = false;
        state.schematic.snap_engine.snap_radius = 7;
        state.schematic.snap_engine.snap_to_wire_segments = false;
        state.schematic.wire_drawing.routing_mode = WireRoutingMode::VerticalFirst;

        assert!(open_grid_snap_routing_dialog(&mut state));
        let draft = &state.dialogs.grid_snap_routing.draft;
        assert_eq!(draft.grid_style, GridStyle::Lines);
        assert_eq!(draft.snap_spacing, GridSnapSpacingChoice::Free);
        assert_eq!(draft.snap_engine.snap_radius, 7);
        assert!(!draft.snap_engine.snap_to_wire_segments);
        assert_eq!(draft.wire_routing, WireRoutingMode::VerticalFirst);

        state.dialogs.grid_snap_routing.draft.grid_style = GridStyle::Off;
        assert_eq!(
            state.ui.grid,
            GridStyle::Lines,
            "editing the draft must not publish live presentation state"
        );
    }

    #[test]
    fn apply_publishes_every_owner_and_records_only_the_document_pitch() {
        let mut state = AppState::default();
        assert!(open_grid_snap_routing_dialog(&mut state));
        let draft = &mut state.dialogs.grid_snap_routing.draft;
        draft.grid_style = GridStyle::Lines;
        draft.snap_spacing = GridSnapSpacingChoice::Mil25;
        draft.snap_engine.snap_radius = 9;
        draft.snap_engine.snap_to_grid = false;
        draft.snap_engine.snap_to_terminals = false;
        draft.snap_engine.snap_to_junctions = false;
        draft.snap_engine.snap_to_wire_endpoints = false;
        draft.snap_engine.snap_to_wire_segments = true;
        draft.wire_routing = WireRoutingMode::FortyFiveDegree;

        assert!(commit_grid_snap_routing(&mut state).is_ok());

        assert_eq!(state.ui.grid, GridStyle::Lines);
        assert_eq!(
            state.schematic.document_policy.grid_pitch,
            SchematicGridPitch::Mil25
        );
        assert_eq!(state.schematic.grid_size, 5);
        assert!(state.schematic.can_undo());
        assert!(state.schematic.snap_engine.enabled);
        assert_eq!(state.schematic.snap_engine.grid_size, 5);
        assert_eq!(state.schematic.snap_engine.snap_radius, 9);
        assert!(!state.schematic.snap_engine.snap_to_grid);
        assert!(!state.schematic.snap_engine.snap_to_terminals);
        assert!(!state.schematic.snap_engine.snap_to_junctions);
        assert!(!state.schematic.snap_engine.snap_to_wire_endpoints);
        assert!(state.schematic.snap_engine.snap_to_wire_segments);
        assert_eq!(state.ui.schematic_snap, state.schematic.snap_engine);
        assert_eq!(
            state.schematic.wire_drawing.routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            state.schematic.bus_drawing.routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            state.ui.schematic_visibility.wire_routing,
            SchematicWireRoutingStyle::FortyFiveDegree
        );
        assert!(!state.dialogs.grid_snap_routing.open);
    }

    #[test]
    fn free_snap_and_session_only_choices_do_not_dirty_document_history() {
        let mut state = AppState::default();
        let topology = state.schematic.topology_version();
        let pitch = state.schematic.document_policy.grid_pitch;
        assert!(open_grid_snap_routing_dialog(&mut state));
        state.dialogs.grid_snap_routing.draft.grid_style = GridStyle::Off;
        state.dialogs.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Free;
        state.dialogs.grid_snap_routing.draft.wire_routing = WireRoutingMode::Diagonal;

        assert!(commit_grid_snap_routing(&mut state).is_ok());

        assert_eq!(state.schematic.document_policy.grid_pitch, pitch);
        assert_eq!(state.schematic.topology_version(), topology);
        assert!(!state.schematic.can_undo());
        assert!(!state.schematic.snap_engine.enabled);
        assert_eq!(
            state.schematic.snap_engine.grid_size,
            pitch.canvas_grid_size(),
            "Free keeps the active document pitch ready without enabling snapping"
        );
        assert_eq!(state.ui.grid, GridStyle::Off);
        assert_eq!(
            state.schematic.wire_drawing.routing_mode,
            WireRoutingMode::Diagonal
        );
    }

    #[test]
    fn read_only_pitch_change_rejects_the_whole_transaction_without_partial_apply() {
        let mut state = AppState::default();
        state.schematic.read_only = true;
        let original_snap = state.schematic.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(&mut state));
        state.dialogs.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state.dialogs.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Mil25;
        state
            .dialogs
            .grid_snap_routing
            .draft
            .snap_engine
            .snap_to_grid = false;

        let error = commit_grid_snap_routing(&mut state).expect_err("read-only pitch must fail");

        assert!(error.message.contains("read-only"));
        assert_eq!(
            error.focus_target,
            Some(GridSnapRoutingFocusTarget::SnapSpacing)
        );
        assert_eq!(state.ui.grid, GridStyle::Dots);
        assert_eq!(state.schematic.snap_engine, original_snap);
        assert_eq!(
            state.schematic.document_policy.grid_pitch,
            SchematicGridPitch::Mil50
        );
        assert!(state.dialogs.grid_snap_routing.open);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn late_safe_mode_activation_rejects_pitch_change_without_partial_apply() {
        let mut state = AppState::default();
        let original_snap = state.schematic.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(&mut state));
        state.dialogs.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state.dialogs.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Mil25;

        state.workbench.safe_mode.activate(
            LocalSafeModeOptions {
                open_project_read_only: true,
                ..LocalSafeModeOptions::default()
            },
            String::new(),
        );

        let error =
            commit_grid_snap_routing(&mut state).expect_err("late read-only policy must win");
        assert!(error.message.contains("read-only"));
        assert_eq!(
            error.focus_target,
            Some(GridSnapRoutingFocusTarget::SnapSpacing)
        );
        assert_eq!(state.ui.grid, GridStyle::Dots);
        assert_eq!(state.schematic.snap_engine, original_snap);
        assert_eq!(
            state.schematic.document_policy.grid_pitch,
            SchematicGridPitch::Mil50
        );
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn stale_authority_rejects_every_candidate_before_any_owner_changes() {
        let mut state = AppState::default();
        assert!(open_grid_snap_routing_dialog(&mut state));
        state.dialogs.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state
            .dialogs
            .grid_snap_routing
            .draft
            .snap_engine
            .snap_to_terminals = false;
        state.schematic.bump_topology_version();

        let error = commit_grid_snap_routing(&mut state).expect_err("stale authority must fail");

        assert!(error.message.contains("topology"));
        assert_eq!(state.ui.grid, GridStyle::Dots);
        assert!(state.schematic.snap_engine.snap_to_terminals);
        assert!(state.dialogs.grid_snap_routing.open);
    }

    #[test]
    fn wire_gesture_started_after_open_rejects_every_candidate_without_cancelling_it() {
        let mut state = AppState::default();
        let original_snap = state.schematic.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(&mut state));
        state.dialogs.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state.dialogs.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Mil25;
        state.schematic.arm_tool(Tool::Wire);
        state.schematic.wire_drawing.active = true;

        let error =
            commit_grid_snap_routing(&mut state).expect_err("started wire must block stale apply");

        assert!(error.message.contains("authoring gesture"));
        assert_eq!(state.schematic.tool, Tool::Wire);
        assert_eq!(state.ui.grid, GridStyle::Dots);
        assert_eq!(state.schematic.snap_engine, original_snap);
        assert_eq!(
            state.schematic.document_policy.grid_pitch,
            SchematicGridPitch::Mil50
        );
        assert!(state.dialogs.grid_snap_routing.open);
        assert!(!state.schematic.can_undo());
    }

    #[test]
    fn every_governed_snap_spacing_drives_find_snap_target_at_that_exact_pitch() {
        use crate::state::Point;

        for (choice, expected_size, expected_position) in [
            (GridSnapSpacingChoice::Mil50, 10, Point::new(10, 10)),
            (GridSnapSpacingChoice::Mil25, 5, Point::new(5, 5)),
            (GridSnapSpacingChoice::Metric, 4, Point::new(8, 8)),
        ] {
            let mut state = AppState::default();
            assert!(open_grid_snap_routing_dialog(&mut state));
            let draft = &mut state.dialogs.grid_snap_routing.draft;
            draft.snap_spacing = choice;
            draft.snap_engine.snap_to_grid = true;
            draft.snap_engine.snap_to_terminals = false;
            draft.snap_engine.snap_to_junctions = false;
            draft.snap_engine.snap_to_wire_endpoints = false;
            draft.snap_engine.snap_to_wire_segments = false;

            commit_grid_snap_routing(&mut state).expect("valid pitch applies");

            assert_eq!(state.schematic.snap_engine.grid_size, expected_size);
            let snapped =
                state
                    .schematic
                    .snap_engine
                    .find_snap_target(Point::new(6, 6), &[], &[], &[]);
            assert_eq!(
                snapped.snapped_position, expected_position,
                "{choice:?} must be the snap engine's real behavior, not display-only state"
            );
        }
    }

    #[test]
    fn enabled_spacing_requires_at_least_one_actionable_snap_target() {
        let mut state = AppState::default();
        let original_snap = state.schematic.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(&mut state));
        let draft = &mut state.dialogs.grid_snap_routing.draft;
        draft.grid_style = GridStyle::Lines;
        draft.snap_spacing = GridSnapSpacingChoice::Mil25;
        draft.snap_engine.snap_to_grid = false;
        draft.snap_engine.snap_to_terminals = false;
        draft.snap_engine.snap_to_junctions = false;
        draft.snap_engine.snap_to_wire_endpoints = false;
        draft.snap_engine.snap_to_wire_segments = false;

        let error =
            commit_grid_snap_routing(&mut state).expect_err("empty target set must fail closed");

        assert!(error.message.contains("at least one snap target"));
        assert_eq!(
            error.focus_target,
            Some(GridSnapRoutingFocusTarget::SnapTargets)
        );
        assert_eq!(state.ui.grid, GridStyle::Dots);
        assert_eq!(state.schematic.snap_engine, original_snap);
        assert!(!state.schematic.can_undo());
        assert!(state.dialogs.grid_snap_routing.open);
    }
}

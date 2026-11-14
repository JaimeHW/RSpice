//! Transactional schematic grid, snap, and wire-routing settings.
//!
//! Grid display is device-local presentation state, snap targets are exact
//! runtime interaction state, and grid pitch is project-owned document policy.
//! Keeping one isolated draft across those owners gives the upgraded mockup a
//! real Apply/Cancel boundary and prevents partially applied canvas settings.

use egui::Context;

use crate::state::{GridStyle, SchematicWireRoutingStyle, WireRoutingMode};
use crate::ui::widgets::{CommandForm, DialogChoice, FormRows};
use crate::workbench::app::SchematicEditAuthority;
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

/// Governed snap-spacing choices shown by the upgraded schematic toolbar
/// transaction. `Free` disables snapping without replacing the document's
/// retained pitch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum GridSnapSpacingChoice {
    #[default]
    Free,
    Mil25,
    Mil50,
    Metric,
}

impl GridSnapSpacingChoice {
    pub(crate) const ALL: [Self; 4] = [Self::Free, Self::Mil25, Self::Mil50, Self::Metric];

    pub(crate) fn label(self) -> String {
        self.pitch().map_or_else(
            || "Free".to_owned(),
            crate::state::policy_labels::grid_pitch_label,
        )
    }

    pub(crate) const fn pitch(self) -> Option<crate::state::SchematicGridPitch> {
        match self {
            Self::Free => None,
            Self::Mil25 => Some(crate::state::SchematicGridPitch::Mil25),
            Self::Mil50 => Some(crate::state::SchematicGridPitch::Mil50),
            Self::Metric => Some(crate::state::SchematicGridPitch::Metric),
        }
    }

    pub(crate) const fn from_pitch(pitch: crate::state::SchematicGridPitch) -> Self {
        match pitch {
            crate::state::SchematicGridPitch::Mil25 => Self::Mil25,
            crate::state::SchematicGridPitch::Mil50 => Self::Mil50,
            crate::state::SchematicGridPitch::Metric => Self::Metric,
        }
    }
}

/// Complete isolated candidate published by Grid, snap and wire routing.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GridSnapRoutingDraft {
    pub(crate) grid_style: crate::state::GridStyle,
    pub(crate) snap_spacing: GridSnapSpacingChoice,
    pub(crate) snap_engine: crate::state::SnapEngine,
    pub(crate) wire_routing: crate::state::WireRoutingMode,
}

impl Default for GridSnapRoutingDraft {
    fn default() -> Self {
        Self {
            grid_style: crate::state::GridStyle::Dots,
            snap_spacing: GridSnapSpacingChoice::Mil50,
            snap_engine: crate::state::SnapEngine::default(),
            wire_routing: crate::state::WireRoutingMode::HorizontalFirst,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GridSnapRoutingFocusTarget {
    SnapSpacing,
    SnapTargets,
}

/// Application-modal transactional owner for the schematic canvas settings
/// popover. The original and draft are kept separately so Cancel is exact.
#[derive(Debug, Clone, Default)]
pub(crate) struct GridSnapRoutingDialogState {
    pub(crate) open: bool,
    pub(crate) draft: GridSnapRoutingDraft,
    pub(crate) authority: Option<SchematicEditAuthority>,
    pub(crate) validation_error: Option<String>,
    pub(crate) focus_target: Option<GridSnapRoutingFocusTarget>,
}

impl GridSnapRoutingDialogState {
    pub(crate) fn open(&mut self, draft: GridSnapRoutingDraft, authority: SchematicEditAuthority) {
        *self = Self {
            open: true,
            draft,
            authority: Some(authority),
            validation_error: None,
            focus_target: None,
        };
    }

    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

pub(crate) const TITLE: &str = "Grid, snap and wire routing";
const PRIMARY: &str = "Apply canvas settings";
const DESCRIPTION: &str = "Grid display is what you see; snap pitch and snap targets are what the cursor commits to. They are set separately because a coarse grid with fine snapping is a normal way to work.";
const GRID_STYLES: [GridStyle; 3] = [GridStyle::Dots, GridStyle::Lines, GridStyle::Off];

pub(crate) fn open_grid_snap_routing_dialog(
    dialog: &mut GridSnapRoutingDialogState,
    grid: GridStyle,
    model: &Model,
    workbench: &mut WorkbenchSession,
) -> bool {
    if dialog.open {
        return false;
    }
    if model.schematic.canvas_settings_change_blocked() {
        workbench
            .prompt("Finish or cancel the drawing in progress before changing snap or routing.");
        return false;
    }

    let spacing = if model.schematic.session.editor.snap_engine.enabled {
        GridSnapSpacingChoice::from_pitch(model.schematic.document().document_policy.grid_pitch)
    } else {
        GridSnapSpacingChoice::Free
    };
    let draft = GridSnapRoutingDraft {
        grid_style: grid,
        snap_spacing: spacing,
        snap_engine: model.schematic.session.editor.snap_engine.clone(),
        // A session left in angled routing opens on Orthogonal where the
        // document cannot take angled wires, so the draft can be applied.
        wire_routing: match model.schematic.session.editor.wire_drawing.routing_mode {
            mode if mode.is_orthogonal() || angled_routing_allowed(model) => mode,
            _ => WireRoutingMode::HorizontalFirst,
        },
    };
    let authority = SchematicEditAuthority::capture(model);
    dialog.open(draft, authority);
    true
}

/// Draw Grid, snap and wire routing over its draft and what the model
/// allows; what the reader chose.
pub(in crate::workbench) fn show_grid_snap_routing_form(
    transaction: &mut GridSnapRoutingDialogState,
    model: &Model,
    ctx: &Context,
) -> DialogChoice {
    let read_only = model.schematic_edit_read_only();
    let angled_routing = angled_routing_allowed(model);
    let validation_error = transaction.validation_error.clone();
    let mut spacing_id = None;
    let choice = CommandForm::new(TITLE, PRIMARY)
        .describe(DESCRIPTION)
        // A refusal is known only once Apply is pressed.
        .status(validation_error.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            let (grid, spacing) =
                grid_snap_routing_rows(rows, transaction, read_only, angled_routing);
            spacing_id = Some(spacing);
            Some(grid)
        });
    if let Some(GridSnapRoutingFocusTarget::SnapSpacing) = transaction.focus_target.take()
        && let Some(id) = spacing_id
    {
        ctx.memory_mut(|memory| memory.request_focus(id));
    }

    choice
}

/// Why Apply refused, and the field the refusal belongs to.
#[derive(Debug)]
pub(crate) struct GridSnapRoutingCommitError {
    pub(crate) message: String,
    pub(crate) focus_target: Option<GridSnapRoutingFocusTarget>,
}

impl GridSnapRoutingCommitError {
    pub(crate) fn new(
        message: impl Into<String>,
        focus_target: Option<GridSnapRoutingFocusTarget>,
    ) -> Self {
        Self {
            message: message.into(),
            focus_target,
        }
    }
}

pub(crate) const fn any_snap_target_enabled(snap: &crate::state::SnapEngine) -> bool {
    snap.snap_to_grid
        || snap.snap_to_terminals
        || snap.snap_to_junctions
        || snap.snap_to_wire_endpoints
        || snap.snap_to_wire_segments
}

/// Why 45° and free routing are unavailable under an orthogonal policy.
pub(crate) const ANGLED_ROUTING_REASON: &str = "45\u{00b0} and free routing need the any-angle junction policy; \
     this document keeps its wiring and junctions orthogonal.";

/// Angled wires are drawn only where the document's junction policy keeps
/// dots at angled joins too.
pub(crate) fn angled_routing_allowed(model: &Model) -> bool {
    model.schematic.document().document_policy.wire_junctions
        == crate::state::WireJunctionPolicy::AnyAngle
}

pub(crate) fn canonical_routing_mode(
    requested: WireRoutingMode,
    current: WireRoutingMode,
) -> WireRoutingMode {
    if requested.is_orthogonal() && current.is_orthogonal() {
        current
    } else if requested.is_orthogonal() {
        WireRoutingMode::HorizontalFirst
    } else {
        requested
    }
}

pub(in crate::workbench) const fn routing_style(
    mode: WireRoutingMode,
) -> SchematicWireRoutingStyle {
    match mode {
        WireRoutingMode::HorizontalFirst | WireRoutingMode::VerticalFirst => {
            SchematicWireRoutingStyle::Orthogonal
        }
        WireRoutingMode::FortyFiveDegree => SchematicWireRoutingStyle::FortyFiveDegree,
        WireRoutingMode::Diagonal => SchematicWireRoutingStyle::FreeAngle,
    }
}

/// The form's rows. Returns the grid display's and the snap spacing's
/// controls: the first takes the caret on open, the second after Apply
/// refused the spacing.
fn grid_snap_routing_rows(
    rows: &mut FormRows<'_>,
    transaction: &mut GridSnapRoutingDialogState,
    read_only: bool,
    angled_routing: bool,
) -> (egui::Id, egui::Id) {
    let grid_options = GRID_STYLES.map(|style| grid_style_label(style).to_owned());
    let grid = rows.select_with_response(
        "Grid display",
        "grid-snap-routing-display",
        grid_style_label(transaction.draft.grid_style),
        &grid_options,
    );
    if let Some(index) = grid.picked {
        transaction.draft.grid_style = GRID_STYLES[index];
        transaction.validation_error = None;
    }

    let spacing_labels = GridSnapSpacingChoice::ALL.map(GridSnapSpacingChoice::label);
    let spacing = rows.select_with_response(
        "Snap spacing",
        "grid-snap-routing-spacing",
        &transaction.draft.snap_spacing.label(),
        &spacing_labels,
    );
    if let Some(index) = spacing.picked {
        transaction.draft.snap_spacing = GridSnapSpacingChoice::ALL[index];
        transaction.draft.snap_engine.enabled =
            transaction.draft.snap_spacing != GridSnapSpacingChoice::Free;
        transaction.validation_error = None;
    }
    if read_only {
        rows.derived("Only Free applies here: this view cannot change the document pitch.");
    }

    let snap = &mut transaction.draft.snap_engine;
    if rows.checks(
        "Snap targets",
        &mut [
            ("Grid intersections", &mut snap.snap_to_grid),
            ("Terminals", &mut snap.snap_to_terminals),
            ("Junctions", &mut snap.snap_to_junctions),
            ("Wire endpoints", &mut snap.snap_to_wire_endpoints),
            ("Wire segments", &mut snap.snap_to_wire_segments),
        ],
    ) {
        transaction.validation_error = None;
    }
    if transaction.draft.snap_spacing == GridSnapSpacingChoice::Free {
        rows.derived("Free spacing turns snapping off; these apply once a pitch is chosen.");
    }

    if angled_routing {
        let routing_options = ["Orthogonal", "45\u{00b0}", "Free"].map(str::to_owned);
        if let Some(index) = rows.select(
            "Wire routing",
            "grid-snap-routing-wire-routing",
            routing_label(transaction.draft.wire_routing),
            &routing_options,
        ) {
            transaction.draft.wire_routing = match index {
                1 => WireRoutingMode::FortyFiveDegree,
                2 => WireRoutingMode::Diagonal,
                _ if transaction.draft.wire_routing.is_orthogonal() => {
                    transaction.draft.wire_routing
                }
                _ => WireRoutingMode::HorizontalFirst,
            };
            transaction.validation_error = None;
        }
    } else {
        rows.value("Wire routing", "Orthogonal");
        rows.derived("45\u{00b0} and free routing need the any-angle junction policy.");
    }
    (grid.response.id, spacing.response.id)
}

const fn grid_style_label(style: GridStyle) -> &'static str {
    match style {
        GridStyle::Dots => "Dots",
        GridStyle::Lines => "Lines",
        GridStyle::Off => "Off",
    }
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
    use crate::state::write_policy::LocalSafeModeOptions;
    use crate::state::{SchematicGridPitch, SchematicWireRoutingStyle, Tool};
    use crate::workbench::app::RSpiceApp;
    use crate::workbench::app_state::AppState;

    /// Apply, over the slot, the model and the session.
    fn commit_grid_snap_routing(state: &mut AppState) -> Result<(), GridSnapRoutingCommitError> {
        state
            .design
            .commit_grid_snap_routing(&mut state.model, &mut state.workbench)
    }

    /// Draw the dialog as its dialog step does.
    fn render(app: &mut RSpiceApp, ctx: &Context) {
        let state = &mut app.state;
        state
            .design
            .render_grid_snap_routing_dialog(&mut state.model, &mut state.workbench, ctx);
    }

    #[test]
    fn open_captures_an_isolated_exact_draft_and_models_free_without_fake_pitch() {
        let mut state = AppState::default();
        state.design.set_grid_style(GridStyle::Lines);
        state.model.schematic.session.editor.snap_engine.enabled = false;
        state.model.schematic.session.editor.snap_engine.snap_radius = 7;
        state
            .model
            .schematic
            .session
            .editor
            .snap_engine
            .snap_to_wire_segments = false;
        state
            .model
            .schematic
            .session
            .editor
            .wire_drawing
            .routing_mode = WireRoutingMode::VerticalFirst;

        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        let draft = &state.design.grid_snap_routing.draft;
        assert_eq!(draft.grid_style, GridStyle::Lines);
        assert_eq!(draft.snap_spacing, GridSnapSpacingChoice::Free);
        assert_eq!(draft.snap_engine.snap_radius, 7);
        assert!(!draft.snap_engine.snap_to_wire_segments);
        assert_eq!(draft.wire_routing, WireRoutingMode::VerticalFirst);

        state.design.grid_snap_routing.draft.grid_style = GridStyle::Off;
        assert_eq!(
            state.design.grid,
            GridStyle::Lines,
            "editing the draft must not publish live presentation state"
        );
    }

    #[test]
    fn apply_publishes_every_owner_and_records_only_the_document_pitch() {
        let mut state = AppState::default();
        // 45° routing is offered only under the any-angle junction policy.
        state
            .model
            .schematic
            .document_mut_for_test()
            .document_policy
            .wire_junctions = crate::state::WireJunctionPolicy::AnyAngle;
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        let draft = &mut state.design.grid_snap_routing.draft;
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

        assert_eq!(state.design.grid, GridStyle::Lines);
        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            SchematicGridPitch::Mil25
        );
        assert_eq!(state.model.schematic.document().grid_size, 5);
        assert!(state.model.schematic.can_undo());
        assert!(state.model.schematic.session.editor.snap_engine.enabled);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.grid_size,
            5
        );
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.snap_radius,
            9
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_grid
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_terminals
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_junctions
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_wire_endpoints
        );
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_wire_segments
        );
        assert_eq!(
            state.design.schematic_snap,
            state.model.schematic.session.editor.snap_engine
        );
        assert_eq!(
            state
                .model
                .schematic
                .session
                .editor
                .wire_drawing
                .routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            state
                .model
                .schematic
                .session
                .editor
                .bus_drawing
                .routing_mode,
            WireRoutingMode::FortyFiveDegree
        );
        assert_eq!(
            state.design.schematic_visibility.wire_routing,
            SchematicWireRoutingStyle::FortyFiveDegree
        );
        assert!(!state.design.grid_snap_routing.open);
    }

    /// Under an orthogonal junction policy nothing maintains dots at angled
    /// crossings the policy promises not to have, so 45° and free routing
    /// are refused there with the reason, and offered under any-angle.
    #[test]
    fn angled_routing_needs_the_any_angle_junction_policy() {
        for mode in [WireRoutingMode::FortyFiveDegree, WireRoutingMode::Diagonal] {
            let mut state = AppState::default();
            assert!(open_grid_snap_routing_dialog(
                &mut state.design.grid_snap_routing,
                state.design.grid,
                &state.model,
                &mut state.workbench
            ));
            state.design.grid_snap_routing.draft.wire_routing = mode;

            let error = commit_grid_snap_routing(&mut state)
                .expect_err("an orthogonal junction policy refuses angled routing");
            assert!(error.message.contains("junction"), "{}", error.message);
            assert_eq!(
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .wire_drawing
                    .routing_mode,
                WireRoutingMode::HorizontalFirst
            );

            let mut state = AppState::default();
            state
                .model
                .schematic
                .document_mut_for_test()
                .document_policy
                .wire_junctions = crate::state::WireJunctionPolicy::AnyAngle;
            assert!(open_grid_snap_routing_dialog(
                &mut state.design.grid_snap_routing,
                state.design.grid,
                &state.model,
                &mut state.workbench
            ));
            state.design.grid_snap_routing.draft.wire_routing = mode;
            assert!(commit_grid_snap_routing(&mut state).is_ok());
            assert_eq!(
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .wire_drawing
                    .routing_mode,
                mode
            );
        }
    }

    #[test]
    fn free_snap_and_session_only_choices_do_not_dirty_document_history() {
        let mut state = AppState::default();
        // Free routing is offered only under the any-angle junction policy.
        state
            .model
            .schematic
            .document_mut_for_test()
            .document_policy
            .wire_junctions = crate::state::WireJunctionPolicy::AnyAngle;
        let topology = state.model.schematic.topology_version();
        let pitch = state.model.schematic.document().document_policy.grid_pitch;
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        state.design.grid_snap_routing.draft.grid_style = GridStyle::Off;
        state.design.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Free;
        state.design.grid_snap_routing.draft.wire_routing = WireRoutingMode::Diagonal;

        assert!(commit_grid_snap_routing(&mut state).is_ok());

        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            pitch
        );
        assert_eq!(state.model.schematic.topology_version(), topology);
        assert!(!state.model.schematic.can_undo());
        assert!(!state.model.schematic.session.editor.snap_engine.enabled);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.grid_size,
            pitch.canvas_grid_size(),
            "Free keeps the active document pitch ready without enabling snapping"
        );
        assert_eq!(state.design.grid, GridStyle::Off);
        assert_eq!(
            state
                .model
                .schematic
                .session
                .editor
                .wire_drawing
                .routing_mode,
            WireRoutingMode::Diagonal
        );
    }

    #[test]
    fn read_only_pitch_change_rejects_the_whole_transaction_without_partial_apply() {
        let mut state = AppState::default();
        state.model.schematic.session.read_only = true;
        let original_snap = state.model.schematic.session.editor.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        state.design.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state.design.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Mil25;
        state
            .design
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
        assert_eq!(state.design.grid, GridStyle::Dots);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine,
            original_snap
        );
        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            SchematicGridPitch::Mil50
        );
        assert!(state.design.grid_snap_routing.open);
        assert!(!state.model.schematic.can_undo());
    }

    #[test]
    fn late_safe_mode_activation_rejects_pitch_change_without_partial_apply() {
        let mut state = AppState::default();
        let original_snap = state.model.schematic.session.editor.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        state.design.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state.design.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Mil25;

        state.model.safe_mode.activate(
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
        assert_eq!(state.design.grid, GridStyle::Dots);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine,
            original_snap
        );
        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            SchematicGridPitch::Mil50
        );
        assert!(!state.model.schematic.can_undo());
    }

    #[test]
    fn stale_authority_rejects_every_candidate_before_any_owner_changes() {
        let mut state = AppState::default();
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        state.design.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state
            .design
            .grid_snap_routing
            .draft
            .snap_engine
            .snap_to_terminals = false;
        state.model.schematic.bump_topology_version();

        let error = commit_grid_snap_routing(&mut state).expect_err("stale authority must fail");

        assert!(error.message.contains("topology"));
        assert_eq!(state.design.grid, GridStyle::Dots);
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_terminals
        );
        assert!(state.design.grid_snap_routing.open);
    }

    #[test]
    fn wire_gesture_started_after_open_rejects_every_candidate_without_cancelling_it() {
        let mut state = AppState::default();
        let original_snap = state.model.schematic.session.editor.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        state.design.grid_snap_routing.draft.grid_style = GridStyle::Lines;
        state.design.grid_snap_routing.draft.snap_spacing = GridSnapSpacingChoice::Mil25;
        state.model.schematic.arm_tool(Tool::Wire);
        state.model.schematic.session.editor.wire_drawing.active = true;

        let error =
            commit_grid_snap_routing(&mut state).expect_err("started wire must block stale apply");

        assert!(error.message.contains("authoring gesture"));
        assert_eq!(state.model.schematic.session.editor.tool, Tool::Wire);
        assert_eq!(state.design.grid, GridStyle::Dots);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine,
            original_snap
        );
        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            SchematicGridPitch::Mil50
        );
        assert!(state.design.grid_snap_routing.open);
        assert!(!state.model.schematic.can_undo());
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
            assert!(open_grid_snap_routing_dialog(
                &mut state.design.grid_snap_routing,
                state.design.grid,
                &state.model,
                &mut state.workbench
            ));
            let draft = &mut state.design.grid_snap_routing.draft;
            draft.snap_spacing = choice;
            draft.snap_engine.snap_to_grid = true;
            draft.snap_engine.snap_to_terminals = false;
            draft.snap_engine.snap_to_junctions = false;
            draft.snap_engine.snap_to_wire_endpoints = false;
            draft.snap_engine.snap_to_wire_segments = false;

            commit_grid_snap_routing(&mut state).expect("valid pitch applies");

            assert_eq!(
                state.model.schematic.session.editor.snap_engine.grid_size,
                expected_size
            );
            let snapped = state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .find_snap_target_resolved(Point::new(6, 6), &[], &[], &[], |_| None);
            assert_eq!(
                snapped.snapped_position, expected_position,
                "{choice:?} must be the snap engine's real behavior, not display-only state"
            );
        }
    }

    #[test]
    fn enabled_spacing_requires_at_least_one_actionable_snap_target() {
        let mut state = AppState::default();
        let original_snap = state.model.schematic.session.editor.snap_engine.clone();
        assert!(open_grid_snap_routing_dialog(
            &mut state.design.grid_snap_routing,
            state.design.grid,
            &state.model,
            &mut state.workbench
        ));
        let draft = &mut state.design.grid_snap_routing.draft;
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
        assert_eq!(state.design.grid, GridStyle::Dots);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine,
            original_snap
        );
        assert!(!state.model.schematic.can_undo());
        assert!(state.design.grid_snap_routing.open);
    }

    /// Grid, snap and wire routing is a command form: four rows whose
    /// controls share one column, not a 760 pt card of tinted rows each
    /// restating what its control is for.
    #[test]
    fn grid_snap_and_routing_is_a_command_form_without_restated_rows() {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        assert!(open_grid_snap_routing_dialog(
            &mut app.state.design.grid_snap_routing,
            app.state.design.grid,
            &app.state.model,
            &mut app.state.workbench
        ));
        // Access updates are incremental: keep the latest of every node.
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..4 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| render(&mut app, ctx),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        // The modal's own node is the whole screen. The surface is centred
        // on it, and the primary ends 12 pt in from its right edge.
        let primary = nodes
            .values()
            .find(|node| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(PRIMARY)
            })
            .and_then(egui::accesskit::Node::bounds)
            .expect("the primary has bounds");
        let width = 2.0 * (primary.x1 + 12.0) - 1440.0;
        assert!(width <= 424.0, "the form is {width} pt wide");
        let says = |text: &str| {
            nodes.values().any(|node| {
                node.label().is_some_and(|label| label.contains(text))
                    || node.value().is_some_and(|value| value.contains(text))
            })
        };
        assert!(
            !says("never changes snapping"),
            "a row restates its control"
        );
        assert!(says("Snap spacing"));
    }

    /// A routing the view cannot change is a read-only fact on the form's
    /// grid, and it reads at the size of the choices above it.
    #[test]
    fn the_routing_fact_reads_at_the_size_of_the_choices_beside_it() {
        fn walk(shape: &egui::Shape, into: &mut Vec<(String, egui::FontId)>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
                egui::Shape::Text(text) => into.push((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.clone(),
                )),
                _ => {}
            }
        }
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = RSpiceApp::test_instance();
        assert!(open_grid_snap_routing_dialog(
            &mut app.state.design.grid_snap_routing,
            app.state.design.grid,
            &app.state.model,
            &mut app.state.workbench
        ));
        let mut shapes = Vec::new();
        for _ in 0..4 {
            shapes = ctx
                .run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1440.0, 900.0),
                        )),
                        ..egui::RawInput::default()
                    },
                    |ctx| render(&mut app, ctx),
                )
                .shapes;
        }
        let mut texts = Vec::new();
        for clipped in &shapes {
            walk(&clipped.shape, &mut texts);
        }
        let font_of = |value: &str| {
            texts
                .iter()
                .find(|(text, _)| text == value)
                .map(|(_, font)| font.clone())
                .unwrap_or_else(|| panic!("{value:?} is painted"))
        };
        let choice = font_of(grid_style_label(app.state.design.grid));
        assert_eq!(font_of("Orthogonal"), choice, "the routing fact");
        assert_eq!(choice, crate::ui::theme::field_sans(), "the field size");
    }
}

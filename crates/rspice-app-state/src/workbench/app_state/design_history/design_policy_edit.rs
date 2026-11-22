//! The project's design policy as steps of the one Undo.
//!
//! The connectivity policy and the rule deck are project state, applied from
//! the Connectivity dialog and from Design rule setup. Every apply is one step
//! holding the policy as it was and as it became and, where the Connectivity
//! dialog's repairs went with it, the drawing as they found it and as they
//! left it. One Undo therefore takes back the repairs and the policy together,
//! where it used to put the wires back and leave the policy changed. A step
//! stands on what its last move left: a change made since to the policy or to
//! that drawing refuses it and says so, rather than overwriting that change.

use super::hierarchy_extraction::{apply_design_snapshot, schematic_matches};
use super::*;
use rspice_design::connectivity_contract::ConnectivityContract;
use rspice_design::drc::DesignRuleDeck;

/// What the project decides about its design: what nets and buses may do,
/// and the severity each design rule is reported at.
#[derive(Debug, Clone, PartialEq)]
pub struct DesignPolicy {
    pub connectivity: ConnectivityContract,
    pub design_rules: DesignRuleDeck,
}

impl DesignPolicy {
    /// The policy the project holds now.
    pub fn of(state: &Model) -> Self {
        Self {
            connectivity: (*state.workspace.content.connectivity).clone(),
            design_rules: (*state.workspace.content.design_rules).clone(),
        }
    }
}

/// The drawing a policy step rewired, as the step found it and left it.
#[derive(Debug, Clone)]
struct RewiredDrawing {
    owner: CellViewRef,
    before: SchematicSnapshot,
    after: SchematicSnapshot,
}

/// One committed change to the project's design policy, with the drawing
/// that changed beside it.
#[derive(Debug, Clone)]
pub struct DesignPolicyEditRecord {
    description: String,
    before: DesignPolicy,
    after: DesignPolicy,
    drawing: Option<RewiredDrawing>,
}

impl Model {
    /// Make `candidate` the project's design policy and, where `drawing` is
    /// given, make it the active schematic's content, as one step of Undo
    /// described by `description`. Refused when neither changes anything, or
    /// when what it writes is read-only.
    pub fn publish_design_policy(
        &mut self,
        candidate: DesignPolicy,
        drawing: Option<SchematicSnapshot>,
        description: impl Into<String>,
    ) -> Result<(), String> {
        if self.safe_mode.project_read_only() {
            return Err("The project is open read-only.".to_owned());
        }
        let before = DesignPolicy::of(self);
        let owner = self.workspace.content.active_schematic_reference();
        let drawing = drawing
            .filter(|after| !after.is_equal_document(self.schematic.document()))
            .map(|after| RewiredDrawing {
                owner: owner.clone(),
                before: SchematicSnapshot::capture(self.schematic.document()),
                after,
            });
        if candidate == before && drawing.is_none() {
            return Err("Nothing to apply: the policy and the drawing are unchanged.".to_owned());
        }
        if drawing.is_some() && document_read_only(self, &owner) {
            return Err(format!("{} is read-only.", owner.display_path()));
        }
        if let Some(drawing) = &drawing {
            apply_design_snapshot(self, &drawing.owner, &drawing.after)?;
            self.schematic.recalculate_runtime_state();
        }
        replace(self, &before, &candidate);
        let header = match &drawing {
            Some(drawing) => RecordHeader::committed(
                vec![DocumentCompensation::naming(drawing.owner.clone())],
                Some(drawing.owner.clone()),
                Some(drawing.owner.clone()),
            ),
            None => RecordHeader::project_only(),
        };
        self.push_project_record(
            header,
            ProjectDesignBody::DesignPolicyEdit(Box::new(DesignPolicyEditRecord {
                description: description.into(),
                before,
                after: candidate,
                drawing,
            })),
        );
        Ok(())
    }
}

impl DesignPolicyEditRecord {
    pub(super) fn description(&self) -> &str {
        &self.description
    }

    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        DesignPolicy::of(state) == self.after
            && self
                .drawing
                .as_ref()
                .is_none_or(|drawing| schematic_matches(state, &drawing.owner, &drawing.after))
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        DesignPolicy::of(state) == self.before
            && self
                .drawing
                .as_ref()
                .is_none_or(|drawing| schematic_matches(state, &drawing.owner, &drawing.before))
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "The design policy cannot be {operation} without an open project."
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "The design policy cannot be {operation} while the project is read-only."
            ));
        }
        if let Some(drawing) = &self.drawing
            && document_read_only(state, &drawing.owner)
        {
            return Err(format!(
                "The repairs cannot be {operation} because {} is read-only.",
                drawing.owner.display_path()
            ));
        }
        Ok(())
    }

    pub(super) fn apply_before(
        &mut self,
        state: &mut Model,
        effects: &mut HistoryEffects,
    ) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "The design policy change cannot be undone because the policy or the drawing changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        self.restore(state, false, effects)
    }

    pub(super) fn apply_after(
        &mut self,
        state: &mut Model,
        effects: &mut HistoryEffects,
    ) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "The design policy change cannot be redone because the policy or the drawing changed since."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        self.restore(state, true, effects)
    }

    /// Put the drawing and the policy on one side of the step.
    fn restore(
        &self,
        state: &mut Model,
        forward: bool,
        effects: &mut HistoryEffects,
    ) -> Result<(), String> {
        let (current, policy) = if forward {
            (&self.before, &self.after)
        } else {
            (&self.after, &self.before)
        };
        if let Some(drawing) = &self.drawing {
            let snapshot = if forward {
                &drawing.after
            } else {
                &drawing.before
            };
            apply_design_snapshot(state, &drawing.owner, snapshot)?;
            if state.workspace.content.active_schematic_reference() == drawing.owner {
                effects.restore_active_schematic = true;
            }
            if let Some(open) = state
                .workspace
                .content
                .open_views
                .iter_mut()
                .find(|open| open.reference == drawing.owner)
            {
                open.dirty = true;
            }
        }
        replace(state, current, policy);
        Ok(())
    }
}

/// Write `policy` over `current`, touching only the half that differs, so a
/// rule-deck step leaves the connectivity contract's stamp, and every report
/// keyed by it, as they were. A new connectivity contract changes which nets
/// a run joins, so the generated netlist's input identity no longer
/// describes it.
fn replace(state: &mut Model, current: &DesignPolicy, policy: &DesignPolicy) {
    if current.connectivity != policy.connectivity {
        *state.workspace.content.connectivity = policy.connectivity.clone();
        state.netlist_artifacts.current_generation_input_digest = None;
    }
    if current.design_rules != policy.design_rules {
        *state.workspace.content.design_rules = policy.design_rules.clone();
    }
    state.workspace.content.project_metadata_dirty = true;
}

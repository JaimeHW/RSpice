//! Adding or editing a capture group, and the preview of the outputs it claims.

use super::*;

// --------------------------------------------------------------- capture group

/// The choice a policy axis offers when the group declines to decide it.
///
/// First in every list, because inheriting is the default a new group opens on
/// and the state most groups stay in — a group usually exists to change one
/// thing, not three.
const INHERIT_CHOICE: &str = "Per output";

/// Open the editor on an existing group's own fields.
///
/// Lives here rather than on the draft because the draft is deliberately
/// ignorant of the capture-group types: it holds indices and text, and this is
/// the one place that knows what those index into.
pub fn group_draft(group: &crate::state::CaptureGroup) -> CaptureGroupDraft {
    // Every rule, in authored order, because that order is the group's own
    // precedence and resolution reads all of them. A ruleless group opens on
    // one empty row so the form has somewhere to type the first rule; an empty
    // row commits as no rule at all.
    let mut rules = group
        .rules
        .iter()
        .map(|rule| CaptureGroupRuleDraft {
            scope: rule
                .scope
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default(),
            kind: rule.kind.and_then(|kind| {
                SavedOutputKind::ALL
                    .iter()
                    .position(|candidate| *candidate == kind)
            }),
        })
        .collect::<Vec<_>>();
    if rules.is_empty() {
        rules.push(CaptureGroupRuleDraft::default());
    }
    CaptureGroupDraft {
        group: Some(group.id),
        name: group.name.clone(),
        rules,
        points: group.points.and_then(|points| {
            SavedOutputPolicy::ALL
                .iter()
                .position(|candidate| *candidate == points)
        }),
        streaming: group.streaming.and_then(|streaming| {
            SavedOutputStreaming::ALL
                .iter()
                .position(|candidate| *candidate == streaming)
        }),
        precision: group.precision.and_then(|precision| {
            SavedOutputPrecision::ALL
                .iter()
                .position(|candidate| *candidate == precision)
        }),
        validation_error: None,
    }
}

/// An optional choice as one select: the inherit option, then the real ones.
///
/// Absent has to be a value in the list rather than a checkbox beside it,
/// because "leave this to each output" is a policy decision of the same kind as
/// the others and reads as one here.
fn optional_choice_field(
    ui: &mut Ui,
    salt: &str,
    label: &str,
    value: &mut Option<usize>,
    inherit: &str,
    options: &[&str],
) {
    let mut choices = vec![inherit];
    choices.extend_from_slice(options);
    let mut index = value.map_or(0, |chosen| chosen + 1);
    workflow_select_field(ui, salt, label, &mut index, &choices);
    *value = index.checked_sub(1);
}

/// Author or edit one capture group.
///
/// The plan is asked what it would refuse while the fields are being typed in,
/// so a name already taken disables the primary control with the reason on
/// screen rather than after the button is pressed. Committing re-asks: the
/// preview is a courtesy and the workspace remains the only authority.
pub(super) fn capture_group_dialog(
    ctx: &egui::Context,
    cx: &mut Cx<'_, SimulateFeature>,
    mut draft: CaptureGroupDraft,
) {
    draft.validation_error = capture_group_from_draft(cx.model, &draft).err();
    let enabled = draft.validation_error.is_none();
    let editing = draft.group.is_some();
    let claimed = capture_group_claim_preview(cx.model, &draft);
    let kind_options = SavedOutputKind::ALL
        .iter()
        .map(|kind| kind.label())
        .collect::<Vec<_>>();
    let points_options = SavedOutputPolicy::ALL
        .iter()
        .map(|policy| policy.label())
        .collect::<Vec<_>>();
    let precision_options = SavedOutputPrecision::ALL
        .iter()
        .map(|precision| precision.label())
        .collect::<Vec<_>>();
    let streaming_options = SavedOutputStreaming::ALL
        .iter()
        .map(|streaming| streaming.label())
        .collect::<Vec<_>>();
    let choice = Dialog::new(
        if editing {
            "Edit capture group"
        } else {
            "Add capture group"
        },
        if editing { "Save group" } else { "Add group" },
    )
    .description(
        "Name one capture policy and the outputs it governs. Each rule claims outputs by scope \
         and kind, and an output matched by any of them is a member; an output named directly by \
         a group outranks every rule.",
    )
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .primary_on_enter(false)
    .show(ctx, |ui| {
        workflow_split(
            ui,
            |ui| {
                workflow_text_field(ui, "Group name", &mut draft.name, false);
                workflow_section_heading(ui, "Membership rules");
                // Every rule the group states, in authored order, with the
                // controls that change that order. A group's rules are a
                // disjunction and resolution reads all of them, so an editor
                // showing one row of several would commit a membership it had
                // never displayed.
                let rule_count = draft.rules.len();
                let mut raise = None;
                let mut remove = None;
                for (index, rule) in draft.rules.iter_mut().enumerate() {
                    if index > 0 {
                        ui.add_space(6.0);
                    }
                    workflow_text_field(
                        ui,
                        &format!("Rule {} \u{b7} instance scope", index + 1),
                        &mut rule.scope,
                        true,
                    );
                    optional_choice_field(
                        ui,
                        &format!("capture-group-kind-{index}"),
                        "Output kind",
                        &mut rule.kind,
                        "Any kind",
                        &kind_options,
                    );
                    ui.horizontal(|ui| {
                        if IconButton::new(Icon::ChevronUp)
                            .enabled(index > 0)
                            .tooltip("Evaluate this rule earlier in the group")
                            .show(ui)
                            .clicked()
                        {
                            raise = Some(index);
                        }
                        if IconButton::new(Icon::Trash)
                            .enabled(rule_count > 1)
                            .tooltip("Remove this rule from the group")
                            .show(ui)
                            .clicked()
                        {
                            remove = Some(index);
                        }
                    });
                }
                if Button::new("Add rule")
                    .icon(Icon::Add)
                    .ghost()
                    .min_width(ui.available_width())
                    .show(ui)
                    .clicked()
                {
                    draft.rules.push(CaptureGroupRuleDraft::default());
                }
                // Applied after the loop: both would invalidate the indices the
                // rows were drawn from.
                if let Some(index) = raise {
                    draft.rules.swap(index - 1, index);
                }
                if let Some(index) = remove {
                    draft.rules.remove(index);
                }
                ui.add_space(8.0);
                // Wrapped, not elided: the sentence is the whole answer, and
                // a single-line row cut it at "match this…", which left the
                // reader unable to tell whether one rule or several had been
                // counted — the one thing the sentence exists to say.
                property_row_wrapped(ui, "Claims now", &claimed);
            },
            |ui| {
                workflow_section_heading(ui, "Capture policy for members");
                optional_choice_field(
                    ui,
                    "capture-group-points",
                    "Points",
                    &mut draft.points,
                    INHERIT_CHOICE,
                    &points_options,
                );
                optional_choice_field(
                    ui,
                    "capture-group-streaming",
                    "Streaming",
                    &mut draft.streaming,
                    INHERIT_CHOICE,
                    &streaming_options,
                );
                optional_choice_field(
                    ui,
                    "capture-group-precision",
                    "Stored precision",
                    &mut draft.precision,
                    INHERIT_CHOICE,
                    &precision_options,
                );
                ui.add_space(8.0);
                property_row_wrapped(
                    ui,
                    "Unset axes",
                    "keep each member's own contract, so a group can change one decision without \
                     restating the other two",
                );
                property_row_wrapped(
                    ui,
                    "Resolution order",
                    "a new group is added last, and takes only outputs no earlier group claims",
                );
            },
        );
        workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    finish_workflow_choice(cx, choice, draft, commit_capture_group);
}

/// Build the candidate group the draft describes, or say why it is not one yet.
fn capture_group_from_draft(
    model: &Model,
    draft: &CaptureGroupDraft,
) -> Result<crate::state::CaptureGroup, String> {
    let name = crate::state::workspace::normalize_capture_group_name(&draft.name)?;
    let mut group = crate::state::CaptureGroup::new(name)?;
    // Every row, in the order the form shows them, because that order is the
    // group's own disjunction and resolution reads all of it. A row stating
    // neither a scope nor a kind is an empty row rather than a rule claiming
    // every output, so it commits as nothing.
    for row in &draft.rules {
        let scope = row.scope.trim();
        let scope = if scope.is_empty() {
            None
        } else {
            Some(crate::state::InstancePath::parse(scope).map_err(|error| error.to_string())?)
        };
        let kind = row
            .kind
            .and_then(|index| SavedOutputKind::ALL.get(index).copied());
        if scope.is_none() && kind.is_none() {
            continue;
        }
        let rule = crate::state::CaptureGroupRule { scope, kind };
        rule.validate()?;
        group.rules.push(rule);
    }
    group.points = draft
        .points
        .and_then(|index| SavedOutputPolicy::ALL.get(index).copied());
    group.streaming = draft
        .streaming
        .and_then(|index| SavedOutputStreaming::ALL.get(index).copied());
    group.precision = draft
        .precision
        .and_then(|index| SavedOutputPrecision::ALL.get(index).copied());
    // Editing keeps the record's identity and the outputs the group names
    // directly: those are membership decisions made elsewhere, and dropping
    // them here would delete work the form never showed. The rules are not in
    // that category any more — the form shows every one of them, so what it
    // commits is what it displayed.
    if let Some(id) = draft.group {
        group.id = id;
        if let Some(existing) = existing_capture_group(model, id) {
            group.members = existing.members.clone();
        }
    }
    let plan_id = model.sim_setup.stable_analysis_plan()?.id();
    if let Some(payload) = model.workspace.content.plan_data(plan_id)
        && let Some(holder) = payload.capture_groups.iter().find(|existing| {
            existing.id != group.id
                && crate::state::workspace::capture_group_collation_key(&existing.name)
                    == crate::state::workspace::capture_group_collation_key(&group.name)
        })
    {
        return Err(format!(
            "capture group name '{}' is already taken by {}",
            group.name, holder.id
        ));
    }
    group.validate()?;
    Ok(group)
}

fn existing_capture_group(
    model: &Model,
    id: crate::product::CaptureGroupId,
) -> Option<&crate::state::CaptureGroup> {
    let plan_id = model.sim_setup.stable_analysis_plan().ok()?.id();
    model
        .workspace
        .content
        .plan_data(plan_id)?
        .capture_groups
        .iter()
        .find(|group| group.id == id)
}

/// How many of the plan's authored outputs this draft's rule would match.
///
/// Counted over the authored registry rather than the effective set, because
/// the effective set depends on probes and automatic fallback that the dialog
/// cannot show and the reader did not ask about. It is stated as "match", not
/// "claim": precedence still decides, and this dialog is not where it is
/// resolved.
fn capture_group_claim_preview(model: &Model, draft: &CaptureGroupDraft) -> String {
    let Ok(group) = capture_group_from_draft(model, draft) else {
        return "not yet a valid group".to_owned();
    };
    if group.rules.is_empty() {
        return "no rule · this group holds only the outputs it names".to_owned();
    }
    let rules = group.rules.len();
    let Ok(plan_id) = model.sim_setup.stable_analysis_plan().map(|plan| plan.id()) else {
        return "no active plan".to_owned();
    };
    let matched = model
        .workspace
        .content
        .plan_data(plan_id)
        .map(|payload| {
            payload
                .saved_outputs
                .iter()
                .filter(|output| group.rules.iter().any(|rule| rule.matches(output)))
                .count()
        })
        .unwrap_or_default();
    claim_preview_sentence(matched, rules)
}

/// How the preview states `matched` authored outputs over `rules` rules.
///
/// The rules are a disjunction, so the count is over all of them together and
/// says so; naming one rule while counting several was the same half-truth the
/// editor itself used to tell. The verb follows the count rather than the noun
/// it was attached to — the row read "1 authored output match this rule", which
/// is the sentence a reader stops trusting before they finish it.
fn claim_preview_sentence(matched: usize, rules: usize) -> String {
    format!(
        "{matched} {} {}",
        if matched == 1 {
            "authored output matches"
        } else {
            "authored outputs match"
        },
        if rules == 1 {
            "this rule".to_owned()
        } else {
            format!("any of these {rules} rules")
        }
    )
}

pub fn commit_capture_group(
    cx: &mut Cx<'_, SimulateFeature>,
    draft: &CaptureGroupDraft,
) -> Result<String, String> {
    let group = capture_group_from_draft(cx.model, draft)?;
    let name = group.name.clone();
    let editing = draft.group.is_some();
    let plan_id = cx.model.sim_setup.stable_analysis_plan()?.id();
    let detail = if editing {
        format!("Updated capture group {name}.")
    } else {
        format!("Added capture group {name}.")
    };
    let committed = commit_plan_change(cx, plan_id, &detail, move |workspace, plan_id| {
        if let Some(id) = draft.group {
            workspace
                .content
                .replace_capture_group(plan_id, id, group)
                .map(|_| ())
                .map_err(|error| error.to_string())
        } else {
            workspace
                .content
                .add_capture_group(plan_id, group)
                .map(|_| ())
                .map_err(|error| error.to_string())
        }
    });
    if !committed {
        // `commit_plan_change` already reported the refusal on the lifecycle
        // channel; the dialog needs it too so it stays open on the field that
        // caused it.
        return Err(cx.model.analysis_lifecycle_status.message().to_owned());
    }
    Ok(format!(
        "{} capture group {name} in plan {}.",
        if editing { "Updated" } else { "Added" },
        cx.model.sim_setup.active_plan_name()
    ))
}

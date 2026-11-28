//! The words a lifecycle outcome is announced in: the line for a committed change,
//! the line for a refused one, and a removal refusal in the names analyses show under.

use super::*;

/// Restate a refusal in the names the analyses show under.
///
/// The backstop refusal is reached when the plan changed underneath a staged
/// review, or when a caller went straight to the transaction — rare, and
/// exactly when a reader has the least context to spend on decoding
/// `Analysis 4f2a1b3c-… is still required by 9d10e2f4-…`. Only the identities
/// are rewritten: the sentence, and the fail-closed guarantee it reports, stay
/// the plan's own.
pub(super) fn removal_refusal_naming_instances(model: &Model, error: &AnalysisPlanError) -> String {
    let AnalysisPlanError::ReferencedBy { target, dependents } = error else {
        return error.to_string();
    };
    let named = |id: AnalysisInstanceId| format!("{} ({id})", analysis_display_name(model, id));
    format!(
        "Analysis {} is still required by {}.",
        named(*target),
        dependents
            .iter()
            .map(|dependent| named(*dependent))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// Both of these only announce an outcome — the lifecycle status line and the
/// Console record of it, through the funnel that writes the pair — so they
/// take the simulate feature's context rather than the application. A handler
/// that took the whole app could mutate every subsystem to announce an outcome.
pub(super) fn record_receipt(cx: &mut Cx<'_, SimulateFeature>, receipt: &AnalysisLifecycleReceipt) {
    cx.model.record_plan_receipt(receipt_line(receipt));
}

pub(crate) fn record_failure(cx: &mut Cx<'_, SimulateFeature>, action: &str, error: &str) {
    record_plan_refusal(cx, failure_line(action, error));
}

/// The line [`record_receipt`] announces for `receipt`.
pub(crate) fn receipt_line(receipt: &AnalysisLifecycleReceipt) -> String {
    let related = receipt
        .related_instance_id()
        .map_or_else(String::new, |id| format!(" · related instance {id}"));
    format!(
        "Receipt #{} · {} committed for instance {}{related} · revision {} to {} · outcome {}. {}",
        receipt.sequence(),
        lifecycle_command_label(receipt.command()),
        receipt.instance_id(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.outcome(),
        receipt.detail(),
    )
}

/// The line [`record_failure`] announces for `action` refused with `error`.
pub(crate) fn failure_line(action: &str, error: &str) -> String {
    format!("{action} rejected fail-closed: {error}. The stable plan is unchanged.")
}

pub(super) const fn lifecycle_command_label(command: AnalysisLifecycleCommand) -> &'static str {
    match command {
        AnalysisLifecycleCommand::Insert => "Insert",
        AnalysisLifecycleCommand::Edit => "Edit",
        AnalysisLifecycleCommand::Rename => "Rename",
        AnalysisLifecycleCommand::Clone => "Clone",
        AnalysisLifecycleCommand::Enable => "Enable",
        AnalysisLifecycleCommand::Disable => "Disable",
        AnalysisLifecycleCommand::Reorder => "Reorder",
        AnalysisLifecycleCommand::Dependency => "Bind dependencies",
        AnalysisLifecycleCommand::Validate => "Validate",
        AnalysisLifecycleCommand::Preflight => "Preflight",
        AnalysisLifecycleCommand::Execute => "Execute",
        AnalysisLifecycleCommand::Remove => "Remove",
    }
}

//! What an analysis instance is called, and what it is called when nobody has
//! said.
//!
//! An instance carries an optional name. Absent means the instance is shown by
//! its kind label, which is what every project written before names existed
//! resolves to and what a freshly inserted analysis starts as. Everything that
//! shows an instance to a person goes through [`AnalysisInstance::display_name`]
//! so the plan, the picker, the receipts, the jobs manager, and the provenance
//! hops cannot disagree about what one analysis is called.

use std::collections::HashSet;

use crate::product::AnalysisInstanceId;

use super::{
    AnalysisInstance, AnalysisLifecycleCommand, AnalysisLifecycleReceipt, AnalysisLifecycleState,
    AnalysisPlanError, FrozenAnalysisInstance, SimulationPlan,
};

/// The longest name an analysis may carry.
///
/// Long enough for a sentence an engineer would actually write ("Startup
/// transient, cold corner"), short enough that a stack row, a jobs row, and a
/// receipt line can all show it whole.
pub(crate) const NAME_LIMIT: usize = 120;

/// Accept a name, or say exactly why it cannot be one.
///
/// Trimming is part of accepting: a trailing space is invisible in the field
/// that produced it, so storing one would create two names that look identical
/// and collate differently.
pub(super) fn normalize(name: &str) -> Result<String, AnalysisPlanError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(AnalysisPlanError::EmptyInstanceName);
    }
    if trimmed.chars().any(char::is_control) {
        return Err(AnalysisPlanError::InstanceNameNotSingleLine);
    }
    if trimmed.chars().count() > NAME_LIMIT {
        return Err(AnalysisPlanError::InstanceNameTooLong { limit: NAME_LIMIT });
    }
    Ok(trimmed.to_owned())
}

/// The form two names are compared in. Collision is case-insensitive because
/// "startup" and "Startup" name the same analysis to everyone but the compiler.
pub(super) fn collation_key(name: &str) -> String {
    name.trim().to_lowercase()
}

impl AnalysisInstance {
    /// The name this instance was given, if it was given one.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// What this instance is called on every surface that shows it.
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or_else(|| self.kind.label())
    }
}

impl FrozenAnalysisInstance {
    /// What this instance is called on every surface that shows it. A frozen
    /// projection answers this identically to the editable instance it came
    /// from, so a dispatched task is reported by the name the plan showed.
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or_else(|| self.kind.label())
    }
}

impl SimulationPlan {
    /// Why this plan would refuse the name, without changing anything.
    ///
    /// The editor asks before offering to commit, so a refusal is stated while
    /// the engineer is still typing rather than after they press a button.
    #[must_use]
    pub fn instance_name_refusal(
        &self,
        id: AnalysisInstanceId,
        name: &str,
    ) -> Option<AnalysisPlanError> {
        self.checked_instance_name(id, name).err()
    }

    /// The name if this plan would take it, or the refusal.
    ///
    /// Collision is checked against what the other instances are *shown as*,
    /// not only against names that were explicitly set. Taking "Transient" while
    /// an unnamed transient analysis is already shown that way would produce two
    /// rows a person cannot tell apart, which is the exact ambiguity naming
    /// exists to remove. Pre-existing kind-label ambiguity between two unnamed
    /// instances is left alone: it is older than this field and refusing to load
    /// it would cost projects.
    fn checked_instance_name(
        &self,
        id: AnalysisInstanceId,
        name: &str,
    ) -> Result<String, AnalysisPlanError> {
        if self.instance(id).is_none() {
            return Err(AnalysisPlanError::InstanceNotFound(id));
        }
        let name = normalize(name)?;
        let key = collation_key(&name);
        if let Some(holder) = self
            .instances()
            .iter()
            .find(|instance| instance.id() != id && collation_key(instance.display_name()) == key)
        {
            return Err(AnalysisPlanError::InstanceNameTaken {
                name,
                holder: holder.id(),
            });
        }
        Ok(name)
    }

    /// Why this plan would refuse to return the analysis to its kind label.
    ///
    /// The dialog asks before it offers the control, so returning to the
    /// default is disabled with the reason stated rather than offered and then
    /// refused.
    #[must_use]
    pub fn instance_name_clear_refusal(&self, id: AnalysisInstanceId) -> Option<AnalysisPlanError> {
        self.checked_instance_name_clear(id).err()
    }

    /// The name being given up, if this plan would take the analysis back to
    /// its kind label.
    ///
    /// This is the mirror of [`Self::checked_instance_name`], and it is
    /// deliberately the *weaker* of the two tests. Setting a name is checked
    /// against what every other instance is shown as; clearing one is checked
    /// only against names another instance was explicitly given. The asymmetry
    /// is the same rule from both ends: an explicit name must stay
    /// unambiguous, and kind-label ambiguity between instances that carry no
    /// name of their own is the state plans have always been allowed to be in.
    /// Refusing to clear because some other transient analysis is also shown as
    /// "Transient" would strand this one under a name it can never give back.
    fn checked_instance_name_clear(
        &self,
        id: AnalysisInstanceId,
    ) -> Result<String, AnalysisPlanError> {
        let instance = self
            .instance(id)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?;
        // Refused rather than treated as a no-op: the receipt would state that
        // a name was cleared when the analysis never had one, and a log that
        // records changes which did not happen is worse than no log.
        let previous = instance
            .name()
            .ok_or(AnalysisPlanError::InstanceNameAlreadyDefault)?
            .to_owned();
        let fallback = collation_key(instance.kind().label());
        if let Some(holder) = self.instances().iter().find(|other| {
            other.id() != id
                && other
                    .name()
                    .is_some_and(|name| collation_key(name) == fallback)
        }) {
            return Err(AnalysisPlanError::InstanceNameTaken {
                name: instance.kind().label().to_owned(),
                holder: holder.id(),
            });
        }
        Ok(previous)
    }

    /// Return one analysis to being shown by its kind label, atomically.
    ///
    /// Clearing is its own intent, not an empty name: the rename path still
    /// refuses an empty field, because a blank box is far more often a
    /// half-finished edit than a decision to give the name up.
    ///
    /// It commits through the same transaction as every other plan command, so
    /// the revision advances and preflight goes stale exactly as a rename does.
    pub fn clear_instance_name(
        &mut self,
        id: AnalysisInstanceId,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let previous = self.checked_instance_name_clear(id)?;
        let kind = self
            .instance(id)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?
            .kind();
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Rename,
            id,
            None,
            AnalysisLifecycleState::SameState,
            format!(
                "\"{previous}\" ({id}) gave up its name and now shows as {}.",
                kind.label()
            ),
            move |candidate, revision| {
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                let instance = &mut candidate.instances[index];
                instance.name = None;
                instance.modified_revision = revision;
                Ok(())
            },
        )?;
        Ok(receipt)
    }

    /// Give one analysis a name, atomically.
    ///
    /// The plan revision advances, which is what makes a rename a plan edit:
    /// preflight artifacts pinned to the previous revision go stale by
    /// construction rather than by anyone remembering to invalidate them. The
    /// receipt names both the old and the new wording, because a receipt that
    /// only said the new one could not be read back as a history.
    pub fn set_instance_name(
        &mut self,
        id: AnalysisInstanceId,
        name: &str,
    ) -> Result<AnalysisLifecycleReceipt, AnalysisPlanError> {
        let name = self.checked_instance_name(id, name)?;
        let previous = self
            .instance(id)
            .ok_or(AnalysisPlanError::InstanceNotFound(id))?
            .display_name()
            .to_owned();
        let detail = if previous == name {
            format!("Analysis {id} is now named \"{name}\" outright.")
        } else {
            format!("Analysis {id} was renamed from \"{previous}\" to \"{name}\".")
        };
        let ((), receipt) = self.transact(
            AnalysisLifecycleCommand::Rename,
            id,
            None,
            AnalysisLifecycleState::SameState,
            detail,
            move |candidate, revision| {
                let index = candidate.index_of(id)?;
                candidate.ensure_editable(index)?;
                let instance = &mut candidate.instances[index];
                instance.name = Some(name);
                instance.modified_revision = revision;
                Ok(())
            },
        )?;
        Ok(receipt)
    }
}

/// The name a clone of this instance starts with.
///
/// An unnamed source produces an unnamed clone: there is nothing to carry, and
/// the kind-label fallback is already what both would show. A named source
/// keeps its wording with a counter appended, because the clone is recognisably
/// the same analysis and losing the name would lose the only thing anyone said
/// about it. The counter is re-derived rather than incremented, so cloning a
/// clone does not accumulate suffixes.
pub(super) fn clone_name_for(plan: &SimulationPlan, source: Option<&str>) -> Option<String> {
    let stem = counter_stem(source?);
    let taken = plan
        .instances()
        .iter()
        .map(|instance| collation_key(instance.display_name()))
        .collect::<HashSet<_>>();

    // Every counted candidate has to fit, so the stem is trimmed once against
    // the widest suffix this loop can produce rather than being retried at
    // lengths that can never be accepted.
    let suffix = " 4294967295".chars().count();
    let stem = truncate_to(stem, NAME_LIMIT.saturating_sub(suffix));
    for counter in 2..=u32::MAX {
        let candidate = format!("{stem} {counter}");
        if !taken.contains(&collation_key(&candidate)) {
            return Some(candidate);
        }
    }
    None
}

/// A name with any trailing " N" counter removed, so "Startup 2" cloned is
/// "Startup 3" rather than "Startup 2 2".
fn counter_stem(name: &str) -> &str {
    match name.rsplit_once(' ') {
        Some((stem, tail)) if tail.parse::<u32>().is_ok() && !stem.trim().is_empty() => {
            stem.trim_end()
        }
        _ => name,
    }
}

/// Keep at most `limit` characters, never splitting one.
fn truncate_to(name: &str, limit: usize) -> &str {
    match name.char_indices().nth(limit) {
        Some((end, _)) => name[..end].trim_end(),
        None => name,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::AnalysisKind;

    /// A plan holding exactly these kinds, in order, none of them named.
    fn plan_with(kinds: &[AnalysisKind]) -> SimulationPlan {
        let mut plan = SimulationPlan::empty();
        for kind in kinds {
            plan.insert(*kind).expect("a dependency-free kind inserts");
        }
        plan
    }

    #[test]
    fn an_unnamed_instance_is_shown_by_its_kind_label() {
        let plan = plan_with(&[AnalysisKind::Transient]);
        let instance = &plan.instances()[0];
        assert_eq!(instance.name(), None);
        assert_eq!(instance.display_name(), AnalysisKind::Transient.label());
    }

    #[test]
    fn a_name_is_trimmed_before_it_is_stored() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        plan.set_instance_name(id, "  Startup transient \t")
            .expect("a name with padding is a name");
        assert_eq!(plan.instance(id).unwrap().name(), Some("Startup transient"));
        assert_eq!(
            plan.instance(id).unwrap().display_name(),
            "Startup transient"
        );
    }

    #[test]
    fn an_empty_or_whitespace_name_is_refused() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        for attempt in ["", "   ", "\t"] {
            assert_eq!(
                plan.set_instance_name(id, attempt).unwrap_err(),
                AnalysisPlanError::EmptyInstanceName,
                "{attempt:?} is not a name"
            );
        }
        assert_eq!(plan.instance(id).unwrap().name(), None);
    }

    #[test]
    fn a_name_that_is_not_one_line_is_refused() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        assert_eq!(
            plan.set_instance_name(id, "Startup\ntransient")
                .unwrap_err(),
            AnalysisPlanError::InstanceNameNotSingleLine
        );
    }

    #[test]
    fn an_overlong_name_is_refused_by_character_count() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        let at_limit = "é".repeat(NAME_LIMIT);
        plan.set_instance_name(id, &at_limit)
            .expect("the limit counts characters, not bytes");
        assert_eq!(
            plan.set_instance_name(id, &"é".repeat(NAME_LIMIT + 1))
                .unwrap_err(),
            AnalysisPlanError::InstanceNameTooLong { limit: NAME_LIMIT }
        );
    }

    #[test]
    fn a_name_collision_is_refused_case_insensitively_and_changes_nothing() {
        let mut plan = plan_with(&[AnalysisKind::Transient, AnalysisKind::Ac]);
        let first = plan.instances()[0].id();
        let second = plan.instances()[1].id();
        plan.set_instance_name(first, "Startup")
            .expect("the first name is free");
        let revision = plan.revision();

        let refusal = plan.set_instance_name(second, "STARTUP").unwrap_err();
        assert_eq!(
            refusal,
            AnalysisPlanError::InstanceNameTaken {
                name: "STARTUP".to_owned(),
                holder: first,
            }
        );
        assert_eq!(plan.revision(), revision, "a refusal is not a plan edit");
        assert_eq!(plan.instance(second).unwrap().name(), None);
        assert_eq!(
            plan.instance_name_refusal(second, "STARTUP"),
            Some(refusal),
            "the editor is told before it offers to commit"
        );
    }

    #[test]
    fn a_kind_label_another_instance_is_shown_as_is_taken() {
        let mut plan = plan_with(&[AnalysisKind::Transient, AnalysisKind::Ac]);
        let ac = plan.instances()[1].id();
        let refusal = plan
            .set_instance_name(ac, AnalysisKind::Transient.label())
            .unwrap_err();
        assert!(
            matches!(refusal, AnalysisPlanError::InstanceNameTaken { .. }),
            "an unnamed instance still holds the name it is shown as: {refusal}"
        );
    }

    #[test]
    fn renaming_an_instance_to_the_name_it_is_already_shown_as_is_allowed() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        plan.set_instance_name(id, AnalysisKind::Transient.label())
            .expect("an instance does not collide with itself");
        assert_eq!(
            plan.instance(id).unwrap().name(),
            Some(AnalysisKind::Transient.label()),
            "the fallback became an explicit choice"
        );
    }

    #[test]
    fn a_rename_advances_the_revision_and_earns_a_receipt_naming_both_wordings() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        let before = plan.revision();
        let receipt = plan
            .set_instance_name(id, "Startup")
            .expect("the name is free");
        assert_eq!(receipt.command(), AnalysisLifecycleCommand::Rename);
        assert_eq!(receipt.instance_id(), id);
        assert_eq!(receipt.source_revision(), before);
        assert_eq!(receipt.committed_revision(), plan.revision());
        assert!(
            receipt.detail().contains("Transient") && receipt.detail().contains("Startup"),
            "the receipt reads back as history: {}",
            receipt.detail()
        );
        assert_eq!(
            plan.instance(id).unwrap().modified_revision(),
            plan.revision(),
            "a rename is a modification of the instance"
        );
    }

    #[test]
    fn a_named_instance_carries_its_name_into_the_frozen_projection() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        plan.set_instance_name(id, "Startup").expect("free name");
        let frozen = plan.freeze().expect("a named plan still freezes");
        assert_eq!(frozen.instances()[0].display_name(), "Startup");
    }

    #[test]
    fn an_unnamed_frozen_instance_falls_back_to_its_kind_label() {
        let plan = plan_with(&[AnalysisKind::Transient]);
        let frozen = plan.freeze().expect("an unnamed plan freezes");
        assert_eq!(
            frozen.instances()[0].display_name(),
            AnalysisKind::Transient.label()
        );
    }

    #[test]
    fn cloning_a_named_instance_keeps_the_wording_and_stays_unique() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        plan.set_instance_name(id, "Startup").expect("free name");
        let (first, _) = plan.clone_instance(id).expect("a clone is permitted");
        assert_eq!(plan.instance(first).unwrap().name(), Some("Startup 2"));
        let (second, _) = plan.clone_instance(first).expect("a clone of a clone");
        assert_eq!(
            plan.instance(second).unwrap().name(),
            Some("Startup 3"),
            "counters are re-derived, never accumulated"
        );
        assert!(plan.validation_issues().iter().all(|issue| !matches!(
            issue,
            super::super::AnalysisPlanIssue::DuplicateInstanceName { .. }
        )));
    }

    #[test]
    fn cloning_an_unnamed_instance_leaves_the_clone_unnamed() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        let (clone, _) = plan.clone_instance(id).expect("a clone is permitted");
        assert_eq!(plan.instance(clone).unwrap().name(), None);
    }

    #[test]
    fn a_name_can_be_given_back_and_the_kind_label_returns() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        plan.set_instance_name(id, "Startup").expect("free name");
        let before = plan.revision();

        let receipt = plan
            .clear_instance_name(id)
            .expect("the name is given back");

        assert_eq!(plan.instance(id).unwrap().name(), None);
        assert_eq!(
            plan.instance(id).unwrap().display_name(),
            AnalysisKind::Transient.label()
        );
        assert_eq!(receipt.command(), AnalysisLifecycleCommand::Rename);
        assert_eq!(receipt.instance_id(), id);
        assert_eq!(receipt.source_revision(), before);
        assert_eq!(receipt.committed_revision(), plan.revision());
        assert!(
            receipt.detail().contains("Startup")
                && receipt.detail().contains(AnalysisKind::Transient.label()),
            "the receipt says what was given up and what is shown now: {}",
            receipt.detail()
        );
        assert_eq!(
            plan.instance(id).unwrap().modified_revision(),
            plan.revision(),
            "giving up a name modifies the instance"
        );
    }

    #[test]
    fn clearing_an_unnamed_analysis_is_refused_and_changes_nothing() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        let before = plan.revision();

        // The dialog disables the control in this state, so this refusal is
        // unreachable in normal use. It exists because the model may not be
        // asked to record that a name was cleared when there was none.
        assert_eq!(
            plan.clear_instance_name(id).unwrap_err(),
            AnalysisPlanError::InstanceNameAlreadyDefault
        );
        assert_eq!(plan.revision(), before, "a refusal is not a plan edit");
        assert_eq!(
            plan.instance_name_clear_refusal(id),
            Some(AnalysisPlanError::InstanceNameAlreadyDefault),
            "the dialog is told before it offers the control"
        );
    }

    #[test]
    fn clearing_is_refused_when_an_explicit_name_holds_the_kind_label() {
        let mut plan = plan_with(&[AnalysisKind::Transient, AnalysisKind::Ac]);
        let transient = plan.instances()[0].id();
        let ac = plan.instances()[1].id();
        plan.set_instance_name(transient, "Startup")
            .expect("free name");
        // Only free once the transient stopped being shown as its kind label.
        plan.set_instance_name(ac, AnalysisKind::Transient.label())
            .expect("no instance is shown that way any more");
        let before = plan.revision();

        let refusal = plan.clear_instance_name(transient).unwrap_err();
        assert_eq!(
            refusal,
            AnalysisPlanError::InstanceNameTaken {
                name: AnalysisKind::Transient.label().to_owned(),
                holder: ac,
            },
            "giving the name back would collide with a name somebody chose"
        );
        assert_eq!(plan.revision(), before, "a refusal is not a plan edit");
        assert_eq!(plan.instance(transient).unwrap().name(), Some("Startup"));
    }

    #[test]
    fn clearing_is_allowed_when_only_an_unnamed_analysis_shares_the_kind_label() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let first = plan.instances()[0].id();
        plan.set_instance_name(first, "Startup").expect("free name");
        let (second, _) = plan.insert(AnalysisKind::Transient).expect("a second one");

        plan.clear_instance_name(first)
            .expect("kind-label ambiguity between unnamed analyses is allowed");

        assert_eq!(
            plan.instance(first).unwrap().display_name(),
            plan.instance(second).unwrap().display_name(),
            "both are shown as their kind, which is the state plans start in"
        );
        assert!(
            plan.structural_issues().is_empty(),
            "{:?}",
            plan.structural_issues()
        );
    }

    #[test]
    fn a_plan_written_before_names_existed_loads_and_falls_back() {
        let mut plan = plan_with(&[AnalysisKind::Transient, AnalysisKind::Ac]);
        let named = plan.instances()[0].id();
        plan.set_instance_name(named, "Startup").expect("free name");
        let json = serde_json::to_string(&plan).expect("a plan serializes");

        // What a project written before the field existed looks like: the key
        // is simply absent, which is also how an unnamed instance is written
        // today, so this is the exact byte shape both produce.
        let older = json.replace("\"name\":\"Startup\",", "");
        assert!(!older.contains("\"name\""), "the field is gone: {older}");
        let restored: SimulationPlan =
            serde_json::from_str(&older).expect("an older plan still loads");

        assert_eq!(restored.instances()[0].name(), None);
        assert_eq!(
            restored.instances()[0].display_name(),
            AnalysisKind::Transient.label()
        );
        assert_eq!(
            restored.instances()[1].display_name(),
            AnalysisKind::Ac.label()
        );
        assert!(
            restored.structural_issues().is_empty(),
            "{:?}",
            restored.structural_issues()
        );
    }

    #[test]
    fn a_plan_of_unnamed_instances_of_one_kind_stays_valid() {
        let mut plan = plan_with(&[AnalysisKind::Transient]);
        let id = plan.instances()[0].id();
        plan.clone_instance(id).expect("a second transient");
        assert!(
            plan.structural_issues().is_empty(),
            "kind-label ambiguity predates naming and must still load: {:?}",
            plan.structural_issues()
        );
    }
}

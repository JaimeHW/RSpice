//! Named capture groups: what a group is, and who owns each output.
//!
//! A saved output states what to capture. A capture group states the *policy*
//! a set of outputs is captured under, so an engineer can say "everything
//! inside `/X1`, at every accepted point" once instead of editing forty rows.
//! Membership is therefore not a label on the output — it is a question asked
//! of the group set, and [`CaptureGroupMembership::resolve`] is the only place
//! it is answered.
//!
//! # One owner per output
//!
//! Every output resolves to exactly one group. That is not a convention the
//! ledger is careful about; it is the shape of the answer.
//! [`CaptureGroupMembership`] is a vector of owners parallel to the outputs it
//! was resolved from, so "this output is in two groups" cannot be written down,
//! and a ledger folding over it cannot double-count.
//!
//! The precedence, in order:
//!
//! 1. **Explicit membership beats rules.** Naming an output is a decision about
//!    that output; a rule is a decision about a shape. When they disagree the
//!    specific one wins, which is also the only order that lets a rule group
//!    have exceptions at all. An output may be explicitly named by at most one
//!    group — [`ProjectWorkspace::add_capture_group`] refuses a second claim —
//!    so this tier is already a partition before rules are consulted.
//! 2. **Among rules, the first group in authored order wins.** Order is the one
//!    tie-break an operator can both see and change: the ledger lists the
//!    groups in exactly this order, and moving a group changes what it takes.
//!    "Most specific rule wins" was the alternative and is worse — the reader
//!    cannot see specificity in the table, so an output would move for reasons
//!    the surface never showed.
//! 3. **Anything unclaimed belongs to the synthesized fallback group.** It is
//!    not authored, not persisted, and carries no policy override, so an output
//!    it holds is captured exactly as its own record says. That is what makes a
//!    project written before capture groups existed load with its forecast and
//!    its execution byte-for-byte unchanged: no groups means one fallback group
//!    holding everything, overriding nothing.
//!
//! # Overrides are per axis
//!
//! `points`, `streaming` and `precision` are each `Option`. `None` means the
//! output's own value survives. A group that could only override all three at
//! once would force an engineer to restate two decisions to change one, and
//! the fallback group would need special-casing in the projection instead of
//! simply being a group that overrides nothing.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::product::{CaptureGroupId, ObjectRevision, RevisionError, SavedOutputId};
use crate::state::{
    InstancePath, ProbeTarget, SavedOutput, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming,
};

/// Why a capture group was refused.
///
/// Owned here rather than spelled out as five variants of the project-wide
/// configuration error, because every one of them is a rule this module
/// states: what a name may be, that a name is unique, that an output is named
/// by at most one group. A caller that wants to know *which* refusal it got
/// matches on this; the workspace wraps it with the plan it happened in.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CaptureGroupError {
    #[error("capture_groups[{index}] is invalid: {message}")]
    Invalid { index: usize, message: String },
    #[error("a capture group named '{name}' already exists")]
    NameConflict { name: String },
    #[error("there is no capture group {group_id}")]
    NotFound { group_id: CaptureGroupId },
    #[error(
        "saved output {output_id} is already named by capture group '{holder}', and an output is \
         named by at most one group"
    )]
    MemberClaimed {
        output_id: SavedOutputId,
        holder: String,
    },
    #[error("capture group identity {id} is already used by plan {first_plan_id}")]
    DuplicateIdentity {
        id: CaptureGroupId,
        first_plan_id: crate::product::SimulationPlanId,
    },
    #[error("capture group {group_id} could not advance its revision: {source}")]
    Revision {
        group_id: CaptureGroupId,
        #[source]
        source: RevisionError,
    },
}

/// Validate one plan's whole group set, accumulating identities across plans.
///
/// A set rather than one group at a time: a colliding name and an output two
/// groups both name are properties of the set, and both would let the ledger
/// show rows a reader cannot tell apart or count one output twice.
pub(super) fn validate_plan_groups(
    plan_id: crate::product::SimulationPlanId,
    groups: &[CaptureGroup],
    identities: &mut HashMap<CaptureGroupId, crate::product::SimulationPlanId>,
) -> Result<(), CaptureGroupError> {
    let mut names = HashMap::<String, usize>::new();
    let mut claimed = HashMap::<SavedOutputId, String>::new();
    for (index, group) in groups.iter().enumerate() {
        group
            .validate()
            .map_err(|message| CaptureGroupError::Invalid { index, message })?;
        if let Some(first_plan_id) = identities.insert(group.id, plan_id) {
            return Err(CaptureGroupError::DuplicateIdentity {
                id: group.id,
                first_plan_id,
            });
        }
        if names.insert(collation_key(&group.name), index).is_some() {
            return Err(CaptureGroupError::NameConflict {
                name: group.name.clone(),
            });
        }
        for member in &group.members {
            if let Some(holder) = claimed.insert(*member, group.name.clone()) {
                return Err(CaptureGroupError::MemberClaimed {
                    output_id: *member,
                    holder,
                });
            }
        }
    }
    Ok(())
}

/// The longest name a capture group may carry.
///
/// The same limit analysis instances use, for the same reason: a ledger row, a
/// receipt line and a dialog caption all have to show it whole.
pub const CAPTURE_GROUP_NAME_LIMIT: usize = 120;

/// What the fallback group is called wherever it is shown.
pub const UNGROUPED_NAME: &str = "Ungrouped";

/// The namespace the fallback group's identity is derived in.
///
/// Fixed, so every project reaches the same identity for it without any
/// project storing one. Versioned by being a distinct constant: deriving it
/// differently later would be a migration, not an edit.
const UNGROUPED_NAMESPACE: Uuid = Uuid::from_u128(0x2f7c_41b9_8ae6_5d02_9b13_c47a_51de_6608);

/// One membership predicate: an output matches when every stated clause holds.
///
/// A clause left `None` is not asked about. Both left `None` would match every
/// output ever authored, which is a group that silently swallows the plan the
/// first time it is listed ahead of another — so [`CaptureGroupRule::validate`]
/// refuses it, and a group that really wants everything says so by scoping to
/// the design root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct CaptureGroupRule {
    /// Instance scope the output's probe must sit at or beneath. The design
    /// root (`/`) matches every resolvable probe.
    #[serde(default)]
    pub scope: Option<InstancePath>,
    #[serde(default)]
    pub kind: Option<SavedOutputKind>,
}

impl CaptureGroupRule {
    #[must_use]
    pub fn for_scope(scope: InstancePath) -> Self {
        Self {
            scope: Some(scope),
            kind: None,
        }
    }

    #[must_use]
    pub const fn for_kind(kind: SavedOutputKind) -> Self {
        Self {
            scope: None,
            kind: Some(kind),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.scope.is_none() && self.kind.is_none() {
            return Err(
                "a membership rule must state a scope, a kind, or both; a rule that states \
                 neither claims every output"
                    .to_owned(),
            );
        }
        Ok(())
    }

    /// Whether `output` satisfies every clause this rule states.
    ///
    /// An output whose expression names no resolvable probe — a derived
    /// expression over several signals, say — has no scope, so a scoped rule
    /// does not match it. Guessing a scope for it would put a signal under an
    /// instance's policy because of how its formula happened to be written.
    #[must_use]
    pub fn matches(&self, output: &SavedOutput) -> bool {
        if let Some(kind) = self.kind
            && output.kind != kind
        {
            return false;
        }
        match &self.scope {
            None => true,
            Some(scope) => probe_scope(&output.source_expression)
                .is_some_and(|actual| actual.starts_with(scope)),
        }
    }

    /// How the rule reads in a ledger cell or a receipt.
    #[must_use]
    pub fn summary(&self) -> String {
        match (&self.scope, self.kind) {
            (Some(scope), Some(kind)) => format!("{scope} · {}", kind.label()),
            (Some(scope), None) => scope.to_string(),
            (None, Some(kind)) => kind.label().to_owned(),
            (None, None) => "states nothing".to_owned(),
        }
    }
}

/// The instance scope of the first probe target in `expression`, if it names
/// one.
///
/// Reads the same grammar saved outputs are validated against, so a scope rule
/// and the output's own validator agree about what `V(x1.n)` refers to. Only
/// the first target is read: an expression spanning two scopes has no single
/// scope, and taking the first would be an arbitrary answer stated as a
/// confident one.
fn probe_scope(expression: &str) -> Option<InstancePath> {
    let expression = expression.trim();
    let token = if let Some(body) = expression.strip_prefix('@') {
        body.split('[').next()?
    } else {
        let open = expression.find('(')?;
        let close = expression.rfind(')')?;
        expression.get(open + 1..close)?.split(',').next()?
    };
    ProbeTarget::parse_legacy(token.trim())
        .ok()
        .map(|target| target.scope)
}

/// A named policy over a set of saved outputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureGroup {
    pub id: CaptureGroupId,
    pub revision: ObjectRevision,
    pub name: String,
    /// Predicates, evaluated as a disjunction: an output matched by any rule is
    /// matched by the group.
    #[serde(default)]
    pub rules: Vec<CaptureGroupRule>,
    /// Outputs named directly, which outrank every rule in every group.
    #[serde(default)]
    pub members: Vec<SavedOutputId>,
    /// `None` on any axis leaves the output's own value in force.
    #[serde(default)]
    pub points: Option<SavedOutputPolicy>,
    #[serde(default)]
    pub streaming: Option<SavedOutputStreaming>,
    #[serde(default)]
    pub precision: Option<SavedOutputPrecision>,
}

impl CaptureGroup {
    pub fn new(name: impl Into<String>) -> Result<Self, String> {
        let group = Self {
            id: CaptureGroupId::new(),
            revision: ObjectRevision::INITIAL,
            name: normalize_name(&name.into())?,
            rules: Vec::new(),
            members: Vec::new(),
            points: None,
            streaming: None,
            precision: None,
        };
        group.validate()?;
        Ok(group)
    }

    /// The identity of the group that holds everything nothing else claims.
    ///
    /// Derived rather than nil because product identities refuse the nil UUID,
    /// and derived rather than minted because two resolutions of the same plan
    /// must attribute the unclaimed outputs to the same group.
    #[must_use]
    pub fn ungrouped_id() -> CaptureGroupId {
        CaptureGroupId::from_namespace(UNGROUPED_NAMESPACE, b"capture-group/ungrouped")
    }

    /// The fallback group as the ledger and the projection see it: named,
    /// ruleless, and overriding nothing.
    #[must_use]
    pub fn ungrouped() -> Self {
        Self {
            id: Self::ungrouped_id(),
            revision: ObjectRevision::INITIAL,
            name: UNGROUPED_NAME.to_owned(),
            rules: Vec::new(),
            members: Vec::new(),
            points: None,
            streaming: None,
            precision: None,
        }
    }

    #[must_use]
    pub fn is_ungrouped(&self) -> bool {
        self.id == Self::ungrouped_id()
    }

    pub fn validate(&self) -> Result<(), String> {
        normalize_name(&self.name)?;
        for rule in &self.rules {
            rule.validate()?;
        }
        let mut seen = std::collections::HashSet::with_capacity(self.members.len());
        if let Some(duplicate) = self.members.iter().find(|member| !seen.insert(**member)) {
            return Err(format!(
                "capture group '{}' names output {duplicate} twice",
                self.name
            ));
        }
        Ok(())
    }

    /// Apply this group's overrides to one of its members.
    ///
    /// Every axis the group leaves unset is left exactly as the output records
    /// it, which is what makes the fallback group a no-op rather than a policy.
    pub fn apply(&self, output: &mut SavedOutput) {
        if let Some(points) = self.points {
            output.save_policy = points;
        }
        if let Some(streaming) = self.streaming {
            output.streaming = streaming;
        }
        if let Some(precision) = self.precision {
            output.stored_precision = precision;
        }
    }

    /// What the group's policy cell says. A group overriding nothing reports
    /// that its members keep their own contracts rather than printing a policy
    /// it does not impose.
    #[must_use]
    pub fn policy_summary(&self) -> String {
        let mut parts = Vec::new();
        if let Some(points) = self.points {
            parts.push(points.label().to_owned());
        }
        if let Some(streaming) = self.streaming {
            parts.push(streaming.label().to_owned());
        }
        if let Some(precision) = self.precision {
            parts.push(precision.label().to_owned());
        }
        if parts.is_empty() {
            "per output".to_owned()
        } else {
            parts.join(" · ")
        }
    }

    /// Rebind this group onto a cloned plan's output identities.
    ///
    /// Rules survive untouched — they name shapes, not records. Explicit
    /// members are remapped through the clone's own output mapping, and a
    /// member with no mapping is dropped rather than carried as a dangling
    /// identity that would silently never resolve.
    #[must_use]
    pub(super) fn cloned_for_new_plan(
        &self,
        output_map: &HashMap<SavedOutputId, SavedOutputId>,
    ) -> Self {
        Self {
            id: CaptureGroupId::new(),
            revision: ObjectRevision::INITIAL,
            name: self.name.clone(),
            rules: self.rules.clone(),
            members: self
                .members
                .iter()
                .filter_map(|member| output_map.get(member).copied())
                .collect(),
            points: self.points,
            streaming: self.streaming,
            precision: self.precision,
        }
    }
}

/// Accept a group name, or say exactly why it cannot be one.
///
/// Trimming is part of accepting, for the reason analysis names give: a
/// trailing space is invisible in the field that produced it, so storing one
/// creates two names that look identical and collate differently.
pub fn normalize_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("capture group name is required".to_owned());
    }
    if trimmed.chars().any(char::is_control) {
        return Err("capture group name must be a single line".to_owned());
    }
    if trimmed.chars().count() > CAPTURE_GROUP_NAME_LIMIT {
        return Err(format!(
            "capture group name exceeds {CAPTURE_GROUP_NAME_LIMIT} characters"
        ));
    }
    if collation_key(trimmed) == collation_key(UNGROUPED_NAME) {
        return Err(format!(
            "'{UNGROUPED_NAME}' names the group that holds every unclaimed output and cannot be \
             taken"
        ));
    }
    Ok(trimmed.to_owned())
}

/// The form two group names are compared in. Case-insensitive, because
/// "rails" and "Rails" name the same group to everyone but the compiler.
#[must_use]
pub fn collation_key(name: &str) -> String {
    name.trim().to_lowercase()
}

/// Who owns each output, resolved once.
///
/// `owners` is parallel to the output slice it was resolved from. There is no
/// constructor that produces anything else, which is what makes "an output
/// counted in two groups" unrepresentable rather than merely avoided.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CaptureGroupMembership {
    owners: Vec<CaptureGroupId>,
}

/// One output that changed hands, as a receipt states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipMove {
    pub output: String,
    /// The group it left, or `None` when the output is new to the plan.
    pub from: Option<String>,
    /// The group it joined, or `None` when the output left the plan.
    pub to: Option<String>,
}

impl MembershipMove {
    /// How one move reads in a receipt line.
    #[must_use]
    pub fn summary(&self) -> String {
        match (&self.from, &self.to) {
            (Some(from), Some(to)) => format!("{} moved {from} → {to}", self.output),
            (None, Some(to)) => format!("{} joined {to}", self.output),
            (Some(from), None) => format!("{} left {from}", self.output),
            (None, None) => self.output.clone(),
        }
    }
}

impl CaptureGroupMembership {
    /// Resolve every output to exactly one owning group.
    ///
    /// The two passes are the precedence: explicit claims first over the whole
    /// output set, then rules in authored order over whatever is still
    /// unclaimed. Doing it in one pass would make a rule in an earlier group
    /// beat an explicit claim in a later one, which is the opposite of the rule
    /// this module states.
    #[must_use]
    pub fn resolve(groups: &[CaptureGroup], outputs: &[SavedOutput]) -> Self {
        let mut owners = vec![CaptureGroup::ungrouped_id(); outputs.len()];
        let mut claimed = vec![false; outputs.len()];
        for group in groups {
            for member in &group.members {
                if let Some(index) = outputs.iter().position(|output| output.id == *member)
                    && !claimed[index]
                {
                    owners[index] = group.id;
                    claimed[index] = true;
                }
            }
        }
        for group in groups {
            for (index, output) in outputs.iter().enumerate() {
                if claimed[index] {
                    continue;
                }
                if group.rules.iter().any(|rule| rule.matches(output)) {
                    owners[index] = group.id;
                    claimed[index] = true;
                }
            }
        }
        Self { owners }
    }

    /// The group owning the output at `index`, or the fallback group when the
    /// index is outside the set this membership was resolved from.
    #[must_use]
    pub fn owner(&self, index: usize) -> CaptureGroupId {
        self.owners
            .get(index)
            .copied()
            .unwrap_or_else(CaptureGroup::ungrouped_id)
    }

    /// What moved between two resolutions of the same plan.
    ///
    /// Outputs are matched by identity rather than position, because a registry
    /// edit is exactly the case where positions shift and nothing moved. The
    /// result is what a receipt names: an output whose owner changed, one that
    /// arrived, or one that left.
    #[must_use]
    pub fn diff(
        previous: (&Self, &[SavedOutput]),
        next: (&Self, &[SavedOutput]),
        naming: &dyn Fn(CaptureGroupId) -> String,
    ) -> Vec<MembershipMove> {
        let (before, before_outputs) = previous;
        let (after, after_outputs) = next;
        let mut moves = Vec::new();
        for (index, output) in after_outputs.iter().enumerate() {
            let to = after.owner(index);
            match before_outputs
                .iter()
                .position(|earlier| earlier.id == output.id)
            {
                Some(earlier) if before.owner(earlier) == to => {}
                Some(earlier) => moves.push(MembershipMove {
                    output: output.name.clone(),
                    from: Some(naming(before.owner(earlier))),
                    to: Some(naming(to)),
                }),
                None => moves.push(MembershipMove {
                    output: output.name.clone(),
                    from: None,
                    to: Some(naming(to)),
                }),
            }
        }
        for (index, output) in before_outputs.iter().enumerate() {
            if !after_outputs.iter().any(|later| later.id == output.id) {
                moves.push(MembershipMove {
                    output: output.name.clone(),
                    from: Some(naming(before.owner(index))),
                    to: None,
                });
            }
        }
        moves
    }
}

/// Name every group in `groups`, with the fallback group's name supplied for
/// any identity the authored set does not hold.
#[must_use]
pub fn group_namer(groups: &[CaptureGroup]) -> impl Fn(CaptureGroupId) -> String + '_ {
    move |id| {
        groups
            .iter()
            .find(|group| group.id == id)
            .map_or_else(|| UNGROUPED_NAME.to_owned(), |group| group.name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SavedOutputCompatibility;

    fn output(name: &str, expression: &str, kind: SavedOutputKind) -> SavedOutput {
        SavedOutput::new(
            kind,
            name,
            expression,
            SavedOutputCompatibility::OpTranAc,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("test output is valid")
    }

    fn scoped(name: &str, scope: &str) -> CaptureGroup {
        let mut group = CaptureGroup::new(name).expect("test group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy(scope).expect("test scope"),
        ));
        group
    }

    #[test]
    fn a_scope_rule_reads_every_spelling_of_the_same_instance() {
        let group = scoped("Core", "/x1");
        for expression in ["V(/X1/n)", "V(x1.n)", "V(x1:n)", "I(x1.r1)"] {
            assert!(
                group.rules[0].matches(&output(
                    "probe",
                    expression,
                    SavedOutputKind::RawVoltageOrCurrent
                )),
                "{expression} sits inside /x1"
            );
        }
        assert!(
            !group.rules[0].matches(&output(
                "probe",
                "V(x2.n)",
                SavedOutputKind::RawVoltageOrCurrent
            )),
            "a sibling instance is not inside /x1"
        );
        assert!(
            group.rules[0].matches(&output(
                "deep",
                "@x1.m1[gm]",
                SavedOutputKind::DeviceOperatingPointQuantity
            )),
            "a device quantity carries the same scope grammar"
        );
    }

    #[test]
    fn an_explicit_member_outranks_every_rule_in_every_group() {
        let outputs = vec![output(
            "core_out",
            "V(x1.n)",
            SavedOutputKind::RawVoltageOrCurrent,
        )];
        let mut rules = scoped("Core", "/x1");
        let mut named = CaptureGroup::new("Watchlist").expect("group");
        named.members.push(outputs[0].id);
        // The rule group is listed first, so only precedence can put the output
        // in the group that named it.
        rules.rules.push(CaptureGroupRule::for_kind(
            SavedOutputKind::RawVoltageOrCurrent,
        ));
        let membership = CaptureGroupMembership::resolve(&[rules, named.clone()], &outputs);

        assert_eq!(membership.owner(0), named.id);
    }

    #[test]
    fn among_rules_the_first_authored_group_takes_the_output() {
        let outputs = vec![output(
            "core_out",
            "V(x1.n)",
            SavedOutputKind::RawVoltageOrCurrent,
        )];
        let first = scoped("First", "/x1");
        let second = scoped("Second", "/x1");
        let forward = CaptureGroupMembership::resolve(&[first.clone(), second.clone()], &outputs);
        let reversed = CaptureGroupMembership::resolve(&[second.clone(), first.clone()], &outputs);

        assert_eq!(forward.owner(0), first.id);
        assert_eq!(
            reversed.owner(0),
            second.id,
            "order is the tie-break, so reordering the groups moves the output"
        );
    }

    #[test]
    fn an_output_no_group_claims_belongs_to_the_fallback_group() {
        let outputs = vec![output(
            "elsewhere",
            "V(x9.n)",
            SavedOutputKind::RawVoltageOrCurrent,
        )];
        let membership = CaptureGroupMembership::resolve(&[scoped("Core", "/x1")], &outputs);

        assert_eq!(membership.owner(0), CaptureGroup::ungrouped_id());
        assert_eq!(
            CaptureGroupMembership::resolve(&[], &outputs).owner(0),
            CaptureGroup::ungrouped_id(),
            "and a plan with no groups at all resolves the same way"
        );
    }

    #[test]
    fn the_fallback_group_overrides_nothing_it_holds() {
        let mut kept = output("keep", "V(n)", SavedOutputKind::RawVoltageOrCurrent);
        let before = kept.clone();
        CaptureGroup::ungrouped().apply(&mut kept);

        assert_eq!(kept, before);
    }

    #[test]
    fn a_rule_that_states_nothing_is_refused() {
        assert!(CaptureGroupRule::default().validate().is_err());
        assert!(
            CaptureGroupRule::for_scope(InstancePath::root())
                .validate()
                .is_ok(),
            "a group that really wants everything says so by scoping to the root"
        );
    }

    #[test]
    fn names_are_trimmed_bounded_and_refuse_the_fallback_name() {
        assert_eq!(normalize_name("  Core rails "), Ok("Core rails".to_owned()));
        assert!(normalize_name("   ").is_err());
        assert!(normalize_name("two\nlines").is_err());
        assert!(normalize_name(&"x".repeat(CAPTURE_GROUP_NAME_LIMIT + 1)).is_err());
        assert!(normalize_name("ungrouped").is_err());
        assert_eq!(collation_key("  Core Rails "), collation_key("core rails"));
    }

    #[test]
    fn the_diff_names_what_moved_rather_than_what_shifted() {
        let first = output("a", "V(x1.n)", SavedOutputKind::RawVoltageOrCurrent);
        let second = output("b", "V(x2.n)", SavedOutputKind::RawVoltageOrCurrent);
        let core = scoped("Core", "/x1");
        let groups = vec![core.clone()];
        let before_outputs = vec![first.clone(), second.clone()];
        let before = CaptureGroupMembership::resolve(&groups, &before_outputs);
        // The registry drops the first output, which shifts the second's index
        // without changing whose it is.
        let after_outputs = vec![second.clone()];
        let after = CaptureGroupMembership::resolve(&groups, &after_outputs);
        let namer = group_namer(&groups);

        let moves = CaptureGroupMembership::diff(
            (&before, &before_outputs),
            (&after, &after_outputs),
            &namer,
        );

        assert_eq!(
            moves,
            vec![MembershipMove {
                output: "a".to_owned(),
                from: Some("Core".to_owned()),
                to: None,
            }]
        );
        assert_eq!(moves[0].summary(), "a left Core");
    }

    #[test]
    fn widening_a_rule_moves_the_outputs_it_now_claims() {
        let outputs = vec![
            output("a", "V(x1.n)", SavedOutputKind::RawVoltageOrCurrent),
            output("b", "V(x2.n)", SavedOutputKind::RawVoltageOrCurrent),
        ];
        let narrow = vec![scoped("Core", "/x1")];
        // The same group, widened — not a second group by the same name, which
        // would move both outputs and for a different reason.
        let mut wide = narrow.clone();
        wide[0].rules[0].scope = Some(InstancePath::root());
        let before = CaptureGroupMembership::resolve(&narrow, &outputs);
        let after = CaptureGroupMembership::resolve(&wide, &outputs);
        let namer = group_namer(&wide);

        let moves = CaptureGroupMembership::diff((&before, &outputs), (&after, &outputs), &namer);

        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].summary(), "b moved Ungrouped → Core");
    }

    // ---------------------------------------- the workspace transactions

    use crate::product::SimulationPlanId;
    use crate::state::workspace::SimulationConfigurationError;
    use crate::state::{
        ProjectWorkspace, SimulationPlanPayload, SimulationPlanPayloadRecord,
    };

    fn capture_output(name: &str, expression: &str) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            SavedOutputCompatibility::OpTranAc,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("saved output")
    }

    /// A plan holding two outputs and one scoped group over the first.
    fn workspace_with_capture_group() -> (ProjectWorkspace, SimulationPlanId, CaptureGroup) {
        let plan_id = SimulationPlanId::new();
        let mut workspace = ProjectWorkspace::default();
        workspace
            .simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id,
                payload: SimulationPlanPayload::default(),
            });
        workspace
            .add_saved_output(plan_id, capture_output("core", "V(x1.n)"))
            .expect("first output");
        workspace
            .add_saved_output(plan_id, capture_output("edge", "V(x2.n)"))
            .expect("second output");
        let mut group = CaptureGroup::new("Core rails").expect("group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy("/x1").expect("scope"),
        ));
        group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
        workspace
            .add_capture_group(plan_id, group.clone())
            .expect("group is added");
        (workspace, plan_id, group)
    }

    #[test]
    fn capture_groups_survive_a_project_round_trip_unchanged() {
        let (workspace, plan_id, group) = workspace_with_capture_group();
        let payload = workspace.plan_data(plan_id).expect("payload").clone();

        let wire = serde_json::to_string(&payload).expect("serialize");
        let restored: SimulationPlanPayload = serde_json::from_str(&wire).expect("deserialize");

        assert_eq!(restored.capture_groups.len(), 1);
        assert_eq!(restored.capture_groups[0].id, group.id);
        assert_eq!(restored.capture_groups[0].name, "Core rails");
        assert_eq!(
            restored.capture_groups[0].points,
            Some(SavedOutputPolicy::EveryAcceptedPoint)
        );
        assert_eq!(
            restored.capture_groups[0].rules[0]
                .scope
                .as_ref()
                .map(ToString::to_string),
            Some("/x1".to_owned())
        );
        assert_eq!(restored, payload, "the whole payload round-trips");
    }

    /// A project written before capture groups existed carries no such field.
    ///
    /// It must load, and every output it holds must land in the fallback group
    /// with its own contract untouched — which is the whole migration: there is
    /// nothing to rewrite, because "no groups" already means what it should.
    #[test]
    fn a_project_written_before_capture_groups_loads_with_everything_ungrouped() {
        let (workspace, plan_id, _) = workspace_with_capture_group();
        let payload = workspace.plan_data(plan_id).expect("payload").clone();
        let mut wire = serde_json::to_value(&payload).expect("serialize");
        wire.as_object_mut()
            .expect("payload object")
            .remove("capture_groups")
            .expect("the field is present before it is removed");

        let restored: SimulationPlanPayload =
            serde_json::from_value(wire).expect("a payload with no capture groups loads");

        assert!(restored.capture_groups.is_empty());
        let membership = CaptureGroupMembership::resolve(&[], &restored.saved_outputs);
        for (index, output) in restored.saved_outputs.iter().enumerate() {
            assert_eq!(
                membership.owner(index),
                CaptureGroup::ungrouped_id(),
                "{} belongs to the fallback group",
                output.name
            );
            assert_eq!(
                output.save_policy,
                SavedOutputPolicy::SelectedAndFinalPoints,
                "and keeps the contract the project stored"
            );
        }
    }

    #[test]
    fn a_capture_group_name_that_collides_is_refused_and_changes_nothing() {
        let (mut workspace, plan_id, _) = workspace_with_capture_group();
        let before = workspace.clone();

        let collision = CaptureGroup::new("core RAILS").expect("name is well formed on its own");
        let refusal = workspace
            .add_capture_group(plan_id, collision)
            .expect_err("a case-insensitive collision is refused");

        assert!(
            matches!(
                refusal,
                SimulationConfigurationError::CaptureGroup {
                    source: CaptureGroupError::NameConflict { .. },
                    ..
                }
            ),
            "{refusal}"
        );
        assert_eq!(
            workspace.plan_data(plan_id),
            before.plan_data(plan_id),
            "a refused add leaves the plan untouched"
        );
    }

    #[test]
    fn an_empty_or_reserved_capture_group_name_is_refused_before_a_group_exists() {
        assert!(CaptureGroup::new("   ").is_err());
        assert!(CaptureGroup::new("line\nbreak").is_err());
        assert!(
            CaptureGroup::new("Ungrouped").is_err(),
            "the fallback group's name is not available to author"
        );
    }

    #[test]
    fn one_output_cannot_be_named_by_two_groups() {
        let (mut workspace, plan_id, first) = workspace_with_capture_group();
        let output_id = workspace.plan_data(plan_id).expect("payload").saved_outputs[0].id;
        workspace
            .set_capture_group_member(plan_id, first.id, output_id, true)
            .expect("the first group names it");
        let mut second = CaptureGroup::new("Watchlist").expect("group");
        second.members.push(output_id);

        let refusal = workspace
            .add_capture_group(plan_id, second)
            .expect_err("a second explicit claim on one output is refused");

        assert!(
            matches!(
                refusal,
                SimulationConfigurationError::CaptureGroup {
                    source: CaptureGroupError::MemberClaimed { .. },
                    ..
                }
            ),
            "{refusal}"
        );
    }

    #[test]
    fn naming_an_output_moves_it_off_whichever_group_held_it() {
        let (mut workspace, plan_id, core) = workspace_with_capture_group();
        let edge_id = workspace.plan_data(plan_id).expect("payload").saved_outputs[1].id;
        let watchlist = CaptureGroup::new("Watchlist").expect("group");
        let watchlist_id = watchlist.id;
        workspace
            .add_capture_group(plan_id, watchlist)
            .expect("second group");
        workspace
            .set_capture_group_member(plan_id, core.id, edge_id, true)
            .expect("core names the sibling-scope output");

        workspace
            .set_capture_group_member(plan_id, watchlist_id, edge_id, true)
            .expect("the watchlist takes it over");

        let payload = workspace.plan_data(plan_id).expect("payload");
        assert!(
            payload.capture_groups[0].members.is_empty(),
            "the previous holder released it rather than both holding it"
        );
        assert_eq!(payload.capture_groups[1].members, vec![edge_id]);
    }

    #[test]
    fn reordering_groups_changes_which_rule_takes_a_contested_output() {
        let (mut workspace, plan_id, core) = workspace_with_capture_group();
        let mut wide = CaptureGroup::new("Everything").expect("group");
        wide.rules.push(CaptureGroupRule::for_kind(
            SavedOutputKind::RawVoltageOrCurrent,
        ));
        let wide_id = wide.id;
        workspace.add_capture_group(plan_id, wide).expect("added");
        let outputs = workspace
            .plan_data(plan_id)
            .expect("payload")
            .saved_outputs
            .clone();
        let before = CaptureGroupMembership::resolve(
            &workspace
                .plan_data(plan_id)
                .expect("payload")
                .capture_groups,
            &outputs,
        );
        assert_eq!(before.owner(0), core.id, "the earlier group takes it first");

        workspace
            .reorder_capture_group(plan_id, wide_id, true)
            .expect("the wide group is raised");

        let after = CaptureGroupMembership::resolve(
            &workspace
                .plan_data(plan_id)
                .expect("payload")
                .capture_groups,
            &outputs,
        );
        assert_eq!(
            after.owner(0),
            wide_id,
            "raising a group is a policy edit: it now takes the contested output"
        );
    }

    #[test]
    fn cloning_a_plan_rebinds_named_members_onto_the_cloned_outputs() {
        let (mut workspace, plan_id, core) = workspace_with_capture_group();
        let source_output = workspace.plan_data(plan_id).expect("payload").saved_outputs[0].id;
        workspace
            .set_capture_group_member(plan_id, core.id, source_output, true)
            .expect("named member");
        let cloned_plan = SimulationPlanId::new();

        workspace
            .clone_plan_data(plan_id, cloned_plan, true, false, &[])
            .expect("clone");

        let cloned = workspace.plan_data(cloned_plan).expect("cloned payload");
        assert_eq!(cloned.capture_groups.len(), 1);
        assert_ne!(
            cloned.capture_groups[0].id, core.id,
            "the clone owns its own group identity"
        );
        assert_eq!(
            cloned.capture_groups[0].members,
            vec![cloned.saved_outputs[0].id],
            "and its named member is the cloned output, not the source one"
        );
    }
}

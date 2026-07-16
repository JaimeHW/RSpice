use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::schema::DecodedShortcutArtifact;
use super::{canonical_json_bytes, hex_digest, sha256};
use crate::workbench::commands::{COMMAND_REGISTRY, Command, CommandPlatform, ShortcutContext};
use crate::workbench::shortcuts::{
    CommandShortcutOverride, ProfileShortcutBinding, ShortcutBindingSlot, ShortcutPreferences,
    ShortcutProfileAudit, ShortcutProfileIssueCode, ShortcutProfileIssueSeverity,
    ShortcutProfileLibrary, ShortcutProfileLibraryError, ShortcutSequence, ShortcutStroke,
    SingleKeyCanvasPolicy, shortcut_context_precedence_rank, shortcut_contexts_overlap,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShortcutMergePolicy {
    MergeNonConflicting,
    ReplaceCurrentUserBindings,
    ImportNamedPreset,
}

impl ShortcutMergePolicy {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::MergeNonConflicting => "Merge non-conflicting bindings",
            Self::ReplaceCurrentUserBindings => "Replace current user bindings",
            Self::ImportNamedPreset => "Import into a named preset",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShortcutConflictPolicy {
    KeepCurrentAndReport,
    UseImportedBinding,
    LeaveBothUnbound,
}

impl ShortcutConflictPolicy {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::KeepCurrentAndReport => "Keep current and report",
            Self::UseImportedBinding => "Use imported binding",
            Self::LeaveBothUnbound => "Leave both unbound",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImportBindingClass {
    Global,
    Schematic,
    Results,
    Simulation,
    Verification,
}

impl ImportBindingClass {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Global => "Global",
            Self::Schematic => "Schematic",
            Self::Results => "Results",
            Self::Simulation => "Simulation",
            Self::Verification => "Verification",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportClassSummary {
    pub binding_class: Option<ImportBindingClass>,
    pub imported: usize,
    pub conflicts: usize,
    pub kept: usize,
    pub replaced: usize,
    pub unbound: usize,
    pub omitted: usize,
}

impl ImportClassSummary {
    #[must_use]
    pub fn label(&self) -> &'static str {
        self.binding_class
            .map_or("Unknown future", ImportBindingClass::label)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutImportConflictKind {
    SameTarget,
    ExactCollision,
    PrefixCollision,
    Policy,
    UnknownCommand,
    ProtectedBinding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutImportDecision {
    Imported,
    KeptCurrent,
    ReplacedCurrent,
    UnboundBoth,
    Omitted,
    RequiresReview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutImportConflict {
    kind: ShortcutImportConflictKind,
    command_id: String,
    conflicting_command_id: Option<String>,
    slot: Option<ShortcutBindingSlot>,
    platform: Option<CommandPlatform>,
    current: Option<String>,
    imported: Option<String>,
    decision: ShortcutImportDecision,
}

impl ShortcutImportConflict {
    #[must_use]
    pub const fn kind(&self) -> ShortcutImportConflictKind {
        self.kind
    }

    #[must_use]
    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    #[must_use]
    pub fn conflicting_command_id(&self) -> Option<&str> {
        self.conflicting_command_id.as_deref()
    }

    #[must_use]
    pub const fn slot(&self) -> Option<ShortcutBindingSlot> {
        self.slot
    }

    #[must_use]
    pub const fn platform(&self) -> Option<CommandPlatform> {
        self.platform
    }

    #[must_use]
    pub fn current(&self) -> Option<&str> {
        self.current.as_deref()
    }

    #[must_use]
    pub fn imported(&self) -> Option<&str> {
        self.imported.as_deref()
    }

    #[must_use]
    pub const fn decision(&self) -> ShortcutImportDecision {
        self.decision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutImportOptions {
    pub merge_policy: ShortcutMergePolicy,
    pub conflict_policy: ShortcutConflictPolicy,
    pub preset_name: Option<String>,
    pub overwrite_existing_preset: bool,
    pub protected_confirmations: BTreeSet<String>,
}

impl Default for ShortcutImportOptions {
    fn default() -> Self {
        Self {
            merge_policy: ShortcutMergePolicy::MergeNonConflicting,
            conflict_policy: ShortcutConflictPolicy::KeepCurrentAndReport,
            preset_name: None,
            overwrite_existing_preset: false,
            protected_confirmations: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShortcutImportPlan {
    source_name: String,
    source_digest: [u8; 32],
    base_revision: u64,
    base_digest: [u8; 32],
    candidate_library: ShortcutProfileLibrary,
    merge_policy: ShortcutMergePolicy,
    conflict_policy: ShortcutConflictPolicy,
    summaries: Vec<ImportClassSummary>,
    conflicts: Vec<ShortcutImportConflict>,
    audit: ShortcutProfileAudit,
    required_protected_confirmations: BTreeSet<String>,
    omitted_envelope_fields: BTreeSet<String>,
    unresolved_internal_conflict: bool,
    preset_name: Option<String>,
}

impl ShortcutImportPlan {
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }
    #[must_use]
    pub const fn source_digest(&self) -> [u8; 32] {
        self.source_digest
    }
    #[must_use]
    pub const fn base_revision(&self) -> u64 {
        self.base_revision
    }
    #[must_use]
    pub const fn base_digest(&self) -> [u8; 32] {
        self.base_digest
    }
    #[must_use]
    pub const fn candidate_library(&self) -> &ShortcutProfileLibrary {
        &self.candidate_library
    }
    #[must_use]
    pub fn summaries(&self) -> &[ImportClassSummary] {
        &self.summaries
    }
    #[must_use]
    pub fn conflicts(&self) -> &[ShortcutImportConflict] {
        &self.conflicts
    }
    #[must_use]
    pub const fn audit(&self) -> &ShortcutProfileAudit {
        &self.audit
    }
    #[must_use]
    pub fn required_protected_confirmations(&self) -> &BTreeSet<String> {
        &self.required_protected_confirmations
    }
    /// Forward envelope metadata is intentionally not persisted into the
    /// profile library. Exposing the exact omitted keys keeps that lossy
    /// boundary visible to import review surfaces and receipts.
    #[must_use]
    pub fn omitted_envelope_fields(&self) -> &BTreeSet<String> {
        &self.omitted_envelope_fields
    }
    #[must_use]
    pub fn can_apply(&self) -> bool {
        self.audit.is_valid()
            && self.required_protected_confirmations.is_empty()
            && !self.unresolved_internal_conflict
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutImportPlanError {
    IncompatibleLibrary,
    InvalidBaseProfile(Vec<String>),
    InvalidImportedProfile(Vec<String>),
    MissingPresetName,
    Preset(ShortcutProfileLibraryError),
    Library(ShortcutProfileLibraryError),
    Profile(String),
    Serialization(String),
}

impl fmt::Display for ShortcutImportPlanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompatibleLibrary => write!(
                formatter,
                "the active shortcut library belongs to an incompatible version"
            ),
            Self::InvalidBaseProfile(issues) => write!(
                formatter,
                "the active shortcut profile is invalid: {}",
                issues.join("; ")
            ),
            Self::InvalidImportedProfile(issues) => write!(
                formatter,
                "the imported shortcut profile is invalid: {}",
                issues.join("; ")
            ),
            Self::MissingPresetName => write!(formatter, "a named-preset import requires a name"),
            Self::Preset(error) | Self::Library(error) => error.fmt(formatter),
            Self::Profile(error) | Self::Serialization(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ShortcutImportPlanError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SliceKey {
    command_id: String,
    slot: ShortcutBindingSlot,
    platform: CommandPlatform,
}

type SliceMap = BTreeMap<SliceKey, Option<ShortcutSequence>>;

pub fn plan_shortcut_import(
    base: &ShortcutProfileLibrary,
    artifact: &DecodedShortcutArtifact,
    options: &ShortcutImportOptions,
) -> Result<ShortcutImportPlan, ShortcutImportPlanError> {
    if !base.is_compatible() {
        return Err(ShortcutImportPlanError::IncompatibleLibrary);
    }
    let base_audit = base.active().audit();
    if !base_audit.is_valid() {
        return Err(ShortcutImportPlanError::InvalidBaseProfile(
            blocking_messages(&base_audit),
        ));
    }
    let imported_audit = artifact.profile().audit();
    let non_resolvable = imported_audit
        .issues()
        .iter()
        .filter(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
        .filter(|issue| {
            !matches!(
                issue.code(),
                ShortcutProfileIssueCode::ExactCollision
                    | ShortcutProfileIssueCode::PrefixCollision
                    | ShortcutProfileIssueCode::MissingBrowserAlternate
            )
        })
        .map(|issue| issue.message().to_owned())
        .collect::<Vec<_>>();
    if !non_resolvable.is_empty() {
        return Err(ShortcutImportPlanError::InvalidImportedProfile(
            non_resolvable,
        ));
    }

    let mut working_profile = match options.merge_policy {
        ShortcutMergePolicy::ImportNamedPreset => ShortcutPreferences::default(),
        ShortcutMergePolicy::MergeNonConflicting
        | ShortcutMergePolicy::ReplaceCurrentUserBindings => base.active().clone(),
    };
    let baseline_profile = working_profile.clone();
    let default_profile = ShortcutPreferences::default();
    let baseline_slices = profile_slices(&baseline_profile);
    let default_slices = profile_slices(&default_profile);
    let imported_slices = profile_slices(artifact.profile());
    let mut working_slices = baseline_slices.clone();
    let mut imported_keys = BTreeSet::new();
    let mut conflicts = Vec::new();
    let mut summaries = summary_map();
    let omitted_envelope_fields = artifact
        .unknown_fields()
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    if !omitted_envelope_fields.is_empty() {
        increment_summary(&mut summaries, None, |summary| {
            summary.omitted += omitted_envelope_fields.len();
        });
    }

    if options.merge_policy == ShortcutMergePolicy::ReplaceCurrentUserBindings {
        for command in COMMAND_REGISTRY.iter().copied() {
            if !artifact
                .manifest()
                .coverage
                .covers_context(command.shortcut_context().label())
            {
                continue;
            }
            for slot in ShortcutBindingSlot::ALL {
                for platform in CommandPlatform::ALL {
                    if !artifact.manifest().coverage.covers_platform(platform) {
                        continue;
                    }
                    let key = slice_key(command, slot, platform);
                    working_slices.insert(
                        key.clone(),
                        default_slices.get(&key).cloned().unwrap_or(None),
                    );
                }
            }
        }
        if artifact.manifest().coverage.policies_included {
            *working_profile.policies_mut() = ShortcutPreferences::default().policies().clone();
        }
    }

    let imported_command_ids = imported_command_ids(artifact);
    // Known records outside the declared context coverage are not part of
    // this transaction. In particular, their forward-compatible extra fields
    // must not leak into an unrelated partial-context Replace.
    let mut commands_to_apply = COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| imported_command_ids.contains(command.stable_id()))
        .filter(|command| {
            artifact
                .manifest()
                .coverage
                .covers_context(command.shortcut_context().label())
        })
        .map(|command| command.stable_id().to_owned())
        .collect::<BTreeSet<_>>();
    if options.merge_policy == ShortcutMergePolicy::ReplaceCurrentUserBindings {
        commands_to_apply.extend(
            COMMAND_REGISTRY
                .iter()
                .copied()
                .filter(|command| {
                    artifact
                        .manifest()
                        .coverage
                        .covers_context(command.shortcut_context().label())
                })
                .filter(|command| baseline_profile.command_override(*command).is_some())
                .map(|command| command.stable_id().to_owned()),
        );
    }
    for command in COMMAND_REGISTRY.iter().copied() {
        if !imported_command_ids.contains(command.stable_id())
            || !artifact
                .manifest()
                .coverage
                .covers_context(command.shortcut_context().label())
        {
            continue;
        }
        let has_current_override = baseline_profile.command_override(command).is_some();
        for slot in ShortcutBindingSlot::ALL {
            for platform in CommandPlatform::ALL {
                if !artifact.manifest().coverage.covers_platform(platform) {
                    continue;
                }
                let key = slice_key(command, slot, platform);
                let current = baseline_slices.get(&key).cloned().unwrap_or(None);
                let imported = imported_slices.get(&key).cloned().unwrap_or(None);
                let class = binding_class(command.shortcut_context());
                if options.merge_policy == ShortcutMergePolicy::MergeNonConflicting
                    && has_current_override
                    && current.is_some()
                    && current != imported
                {
                    increment_summary(&mut summaries, class, |summary| summary.conflicts += 1);
                    let decision = match options.conflict_policy {
                        ShortcutConflictPolicy::KeepCurrentAndReport => {
                            increment_summary(&mut summaries, class, |summary| summary.kept += 1);
                            ShortcutImportDecision::KeptCurrent
                        }
                        ShortcutConflictPolicy::UseImportedBinding => {
                            working_slices.insert(key.clone(), imported.clone());
                            imported_keys.insert(key.clone());
                            increment_summary(&mut summaries, class, |summary| {
                                summary.replaced += 1
                            });
                            ShortcutImportDecision::ReplacedCurrent
                        }
                        ShortcutConflictPolicy::LeaveBothUnbound => {
                            working_slices.insert(key.clone(), None);
                            increment_summary(&mut summaries, class, |summary| {
                                summary.unbound += 1
                            });
                            ShortcutImportDecision::UnboundBoth
                        }
                    };
                    conflicts.push(ShortcutImportConflict {
                        kind: ShortcutImportConflictKind::SameTarget,
                        command_id: command.stable_id().to_owned(),
                        conflicting_command_id: None,
                        slot: Some(slot),
                        platform: Some(platform),
                        current: display_sequence(current.as_ref()),
                        imported: display_sequence(imported.as_ref()),
                        decision,
                    });
                } else {
                    working_slices.insert(key.clone(), imported);
                    imported_keys.insert(key);
                    increment_summary(&mut summaries, class, |summary| summary.imported += 1);
                }
            }
        }
    }

    apply_slice_map(
        &mut working_profile,
        &working_slices,
        &default_slices,
        &commands_to_apply,
    )?;
    restore_known_command_extras(
        &mut working_profile,
        &baseline_profile,
        artifact,
        &commands_to_apply,
        &imported_keys,
    )?;

    if artifact.manifest().coverage.policies_included
        && working_profile.policies() != artifact.profile().policies()
    {
        let conflict = options.merge_policy == ShortcutMergePolicy::MergeNonConflicting
            && baseline_profile.policies() != artifact.profile().policies();
        let decision = if conflict {
            match options.conflict_policy {
                ShortcutConflictPolicy::KeepCurrentAndReport => ShortcutImportDecision::KeptCurrent,
                ShortcutConflictPolicy::UseImportedBinding => {
                    *working_profile.policies_mut() = artifact.profile().policies().clone();
                    ShortcutImportDecision::ReplacedCurrent
                }
                ShortcutConflictPolicy::LeaveBothUnbound => {
                    *working_profile.policies_mut() =
                        ShortcutPreferences::default().policies().clone();
                    ShortcutImportDecision::UnboundBoth
                }
            }
        } else {
            *working_profile.policies_mut() = artifact.profile().policies().clone();
            ShortcutImportDecision::Imported
        };
        if conflict {
            conflicts.push(ShortcutImportConflict {
                kind: ShortcutImportConflictKind::Policy,
                command_id: "execution-policies".to_owned(),
                conflicting_command_id: None,
                slot: None,
                platform: None,
                current: Some("current policy set".to_owned()),
                imported: Some("imported policy set".to_owned()),
                decision,
            });
            increment_summary(
                &mut summaries,
                Some(ImportBindingClass::Global),
                |summary| summary.conflicts += 1,
            );
        }
    }

    merge_unknown_commands(
        &mut working_profile,
        &baseline_profile,
        artifact,
        options.merge_policy,
        options.conflict_policy,
        &mut conflicts,
        &mut summaries,
    )?;
    merge_unknown_profile_fields(
        &mut working_profile,
        &baseline_profile,
        artifact,
        options.merge_policy,
        options.conflict_policy,
        &mut conflicts,
        &mut summaries,
    )?;

    working_slices = profile_slices(&working_profile);
    let mut unresolved_internal_conflict = false;
    let mut seen_collisions = BTreeSet::new();
    for _ in 0..(COMMAND_REGISTRY.len() * 2) {
        let collisions = blocking_collisions(&working_profile);
        let Some(collision) = collisions
            .into_iter()
            .find(|collision| !seen_collisions.contains(&collision.identity()))
        else {
            break;
        };
        seen_collisions.insert(collision.identity());
        let left_imported = imported_keys.contains(&collision.left);
        let right_imported = imported_keys.contains(&collision.right);
        if !left_imported && !right_imported {
            unresolved_internal_conflict = true;
            break;
        }
        let class = Command::from_stable_id(&collision.left.command_id)
            .and_then(|command| binding_class(command.shortcut_context()));
        increment_summary(&mut summaries, class, |summary| summary.conflicts += 1);
        let decision = match options.conflict_policy {
            ShortcutConflictPolicy::KeepCurrentAndReport => {
                for key in [&collision.left, &collision.right] {
                    if imported_keys.remove(key) {
                        working_slices.insert(
                            key.clone(),
                            baseline_slices.get(key).cloned().unwrap_or(None),
                        );
                    }
                }
                increment_summary(&mut summaries, class, |summary| summary.kept += 1);
                ShortcutImportDecision::KeptCurrent
            }
            ShortcutConflictPolicy::UseImportedBinding => {
                if left_imported ^ right_imported {
                    let current_key = if left_imported {
                        &collision.right
                    } else {
                        &collision.left
                    };
                    working_slices.insert(current_key.clone(), None);
                    increment_summary(&mut summaries, class, |summary| summary.replaced += 1);
                    ShortcutImportDecision::ReplacedCurrent
                } else {
                    unresolved_internal_conflict = true;
                    ShortcutImportDecision::RequiresReview
                }
            }
            ShortcutConflictPolicy::LeaveBothUnbound => {
                working_slices.insert(collision.left.clone(), None);
                working_slices.insert(collision.right.clone(), None);
                imported_keys.remove(&collision.left);
                imported_keys.remove(&collision.right);
                increment_summary(&mut summaries, class, |summary| summary.unbound += 2);
                ShortcutImportDecision::UnboundBoth
            }
        };
        conflicts.push(ShortcutImportConflict {
            kind: if collision.prefix {
                ShortcutImportConflictKind::PrefixCollision
            } else {
                ShortcutImportConflictKind::ExactCollision
            },
            command_id: collision.left.command_id.clone(),
            conflicting_command_id: Some(collision.right.command_id.clone()),
            slot: Some(collision.left.slot),
            platform: Some(collision.left.platform),
            current: display_sequence(
                baseline_slices
                    .get(if left_imported {
                        &collision.right
                    } else {
                        &collision.left
                    })
                    .and_then(Option::as_ref),
            ),
            imported: display_sequence(
                working_slices
                    .get(if left_imported {
                        &collision.left
                    } else {
                        &collision.right
                    })
                    .and_then(Option::as_ref),
            ),
            decision,
        });
        let affected_commands = [
            collision.left.command_id.clone(),
            collision.right.command_id.clone(),
        ]
        .into_iter()
        .collect();
        apply_slice_map(
            &mut working_profile,
            &working_slices,
            &default_slices,
            &affected_commands,
        )?;
        restore_known_command_extras(
            &mut working_profile,
            &baseline_profile,
            artifact,
            &affected_commands,
            &imported_keys,
        )?;
        if decision == ShortcutImportDecision::RequiresReview {
            break;
        }
    }

    let mut required_protected_confirmations = BTreeSet::new();
    let pre_confirmation_audit = working_profile.audit();
    for issue in pre_confirmation_audit.issues() {
        if issue.severity() == ShortcutProfileIssueSeverity::Error
            && issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate
            && let Some(command_id) = issue.command_id()
        {
            if options
                .protected_confirmations
                .contains(command_id.as_str())
                && let Some(command) = Command::from_stable_id(command_id.as_str())
            {
                working_profile.acknowledge_protected_override(command);
            } else {
                required_protected_confirmations.insert(command_id.as_str().to_owned());
                conflicts.push(ShortcutImportConflict {
                    kind: ShortcutImportConflictKind::ProtectedBinding,
                    command_id: command_id.as_str().to_owned(),
                    conflicting_command_id: None,
                    slot: issue.slot(),
                    platform: Some(CommandPlatform::Browser),
                    current: None,
                    imported: None,
                    decision: ShortcutImportDecision::RequiresReview,
                });
            }
        }
    }
    let audit = working_profile.audit();

    let mut candidate_library = base.clone();
    let preset_name = match options.merge_policy {
        ShortcutMergePolicy::ImportNamedPreset => {
            let name = options
                .preset_name
                .as_deref()
                .ok_or(ShortcutImportPlanError::MissingPresetName)?;
            candidate_library
                .insert_named_preset(name, working_profile, options.overwrite_existing_preset)
                .map_err(ShortcutImportPlanError::Preset)?;
            Some(name.to_owned())
        }
        ShortcutMergePolicy::MergeNonConflicting
        | ShortcutMergePolicy::ReplaceCurrentUserBindings => {
            candidate_library
                .replace_active(working_profile)
                .map_err(ShortcutImportPlanError::Library)?;
            None
        }
    };

    Ok(ShortcutImportPlan {
        source_name: artifact.source_name().to_owned(),
        source_digest: artifact.source_digest(),
        base_revision: base.revision(),
        base_digest: shortcut_library_digest(base)?,
        candidate_library,
        merge_policy: options.merge_policy,
        conflict_policy: options.conflict_policy,
        summaries: summaries.into_values().collect(),
        conflicts,
        audit,
        required_protected_confirmations,
        omitted_envelope_fields,
        unresolved_internal_conflict,
        preset_name,
    })
}

fn blocking_messages(audit: &ShortcutProfileAudit) -> Vec<String> {
    audit
        .issues()
        .iter()
        .filter(|issue| issue.severity() == ShortcutProfileIssueSeverity::Error)
        .map(|issue| issue.message().to_owned())
        .collect()
}

fn imported_command_ids(artifact: &DecodedShortcutArtifact) -> BTreeSet<String> {
    artifact
        .portable_profile()
        .get("commands")
        .and_then(Value::as_object)
        .map(|commands| commands.keys().cloned().collect())
        .unwrap_or_default()
}

fn profile_slices(profile: &ShortcutPreferences) -> SliceMap {
    let mut slices = BTreeMap::new();
    for command in COMMAND_REGISTRY.iter().copied() {
        let bindings = profile.resolved_bindings(command);
        for slot in ShortcutBindingSlot::ALL {
            for platform in CommandPlatform::ALL {
                let sequence = bindings
                    .iter()
                    .find(|binding| {
                        binding.slot() == slot && binding.platforms().contains(&platform)
                    })
                    .map(|binding| binding.sequence().clone());
                slices.insert(slice_key(command, slot, platform), sequence);
            }
        }
    }
    slices
}

fn slice_key(command: Command, slot: ShortcutBindingSlot, platform: CommandPlatform) -> SliceKey {
    SliceKey {
        command_id: command.stable_id().to_owned(),
        slot,
        platform,
    }
}

fn apply_slice_map(
    profile: &mut ShortcutPreferences,
    slices: &SliceMap,
    defaults: &SliceMap,
    command_ids: &BTreeSet<String>,
) -> Result<(), ShortcutImportPlanError> {
    for command in COMMAND_REGISTRY.iter().copied() {
        if !command_ids.contains(command.stable_id()) {
            continue;
        }
        let command_keys = ShortcutBindingSlot::ALL
            .into_iter()
            .flat_map(|slot| {
                CommandPlatform::ALL
                    .into_iter()
                    .map(move |platform| slice_key(command, slot, platform))
            })
            .collect::<Vec<_>>();
        if command_keys
            .iter()
            .all(|key| slices.get(key) == defaults.get(key))
        {
            profile.reset_command(command);
            continue;
        }
        let mut grouped =
            BTreeMap::<(ShortcutBindingSlot, ShortcutSequence), Vec<CommandPlatform>>::new();
        for key in command_keys {
            if let Some(sequence) = slices.get(&key).cloned().flatten() {
                grouped
                    .entry((key.slot, sequence))
                    .or_default()
                    .push(key.platform);
            }
        }
        let bindings = grouped
            .into_iter()
            .map(|((slot, sequence), platforms)| {
                ProfileShortcutBinding::new(slot, platforms, sequence)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| ShortcutImportPlanError::Profile(error.to_string()))?;
        let command_override = CommandShortcutOverride::new(bindings)
            .map_err(|error| ShortcutImportPlanError::Profile(error.to_string()))?;
        profile
            .set_command_override(command, command_override)
            .map_err(|error| ShortcutImportPlanError::Profile(error.to_string()))?;
    }
    Ok(())
}

fn restore_known_command_extras(
    working: &mut ShortcutPreferences,
    baseline: &ShortcutPreferences,
    artifact: &DecodedShortcutArtifact,
    command_ids: &BTreeSet<String>,
    imported_keys: &BTreeSet<SliceKey>,
) -> Result<(), ShortcutImportPlanError> {
    let mut working_value = serde_json::to_value(&*working)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    let baseline_value = serde_json::to_value(baseline)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    let baseline_commands = baseline_value.get("commands").and_then(Value::as_object);
    let imported_commands = artifact
        .portable_profile()
        .get("commands")
        .and_then(Value::as_object);
    let working_commands = working_value
        .as_object_mut()
        .and_then(|profile| profile.get_mut("commands"))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            ShortcutImportPlanError::Serialization(
                "shortcut profile has no command object".to_owned(),
            )
        })?;

    for id in command_ids {
        let Some(target) = working_commands.get_mut(id).and_then(Value::as_object_mut) else {
            continue;
        };
        for source in [baseline_commands, imported_commands]
            .into_iter()
            .flatten()
            .filter_map(|commands| commands.get(id))
            .filter_map(Value::as_object)
        {
            for (key, value) in source {
                if key != "bindings" {
                    target.insert(key.clone(), value.clone());
                }
            }
        }
        restore_binding_extras(
            id,
            target,
            baseline_commands.and_then(|commands| commands.get(id)),
            imported_commands.and_then(|commands| commands.get(id)),
            imported_keys,
        );
    }
    *working = serde_json::from_value(working_value)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    Ok(())
}

fn restore_binding_extras(
    command_id: &str,
    target: &mut serde_json::Map<String, Value>,
    baseline: Option<&Value>,
    imported: Option<&Value>,
    imported_keys: &BTreeSet<SliceKey>,
) {
    let Some(bindings) = target.get_mut("bindings").and_then(Value::as_array_mut) else {
        return;
    };
    let baseline_bindings = baseline
        .and_then(|command| command.get("bindings"))
        .and_then(Value::as_array);
    let imported_bindings = imported
        .and_then(|command| command.get("bindings"))
        .and_then(Value::as_array);
    let mut restored = Vec::new();
    for binding in std::mem::take(bindings) {
        let Some(binding_object) = binding.as_object() else {
            restored.push(binding);
            continue;
        };
        let Some(platforms) = binding_object.get("platforms").and_then(Value::as_array) else {
            restored.push(binding);
            continue;
        };
        let mut groups = Vec::<(BTreeMap<String, Value>, Vec<Value>)>::new();
        for platform in platforms {
            let Some(platform_id) = platform.as_str() else {
                continue;
            };
            let Ok(platform) = serde_json::from_value::<CommandPlatform>(platform.clone()) else {
                continue;
            };
            let Some(slot) = binding_object
                .get("slot")
                .cloned()
                .and_then(|slot| serde_json::from_value::<ShortcutBindingSlot>(slot).ok())
            else {
                continue;
            };
            let key = SliceKey {
                command_id: command_id.to_owned(),
                slot,
                platform,
            };
            let source = if imported_keys.contains(&key) {
                imported_bindings
            } else {
                baseline_bindings
            };
            let extras = source
                .and_then(|source| source_binding_extras(source, binding_object, platform_id))
                .unwrap_or_default();
            if let Some((_, grouped_platforms)) = groups
                .iter_mut()
                .find(|(candidate, _)| *candidate == extras)
            {
                grouped_platforms.push(Value::String(platform_id.to_owned()));
            } else {
                groups.push((extras, vec![Value::String(platform_id.to_owned())]));
            }
        }
        if groups.is_empty() {
            restored.push(binding);
            continue;
        }
        for (extras, platforms) in groups {
            let mut split = binding_object
                .iter()
                .filter(|(key, _)| matches!(key.as_str(), "slot" | "platforms" | "sequence"))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect::<serde_json::Map<_, _>>();
            split.insert("platforms".to_owned(), Value::Array(platforms));
            split.extend(extras);
            restored.push(Value::Object(split));
        }
    }
    *bindings = restored;
}

fn source_binding_extras(
    bindings: &[Value],
    target: &serde_json::Map<String, Value>,
    platform: &str,
) -> Option<BTreeMap<String, Value>> {
    bindings.iter().find_map(|binding| {
        let binding = binding.as_object()?;
        if binding.get("slot") != target.get("slot")
            || binding.get("sequence") != target.get("sequence")
            || !binding
                .get("platforms")?
                .as_array()?
                .iter()
                .any(|candidate| candidate.as_str() == Some(platform))
        {
            return None;
        }
        Some(
            binding
                .iter()
                .filter(|(key, _)| !matches!(key.as_str(), "slot" | "platforms" | "sequence"))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        )
    })
}

fn merge_unknown_commands(
    working: &mut ShortcutPreferences,
    baseline: &ShortcutPreferences,
    artifact: &DecodedShortcutArtifact,
    merge_policy: ShortcutMergePolicy,
    conflict_policy: ShortcutConflictPolicy,
    conflicts: &mut Vec<ShortcutImportConflict>,
    summaries: &mut BTreeMap<Option<ImportBindingClass>, ImportClassSummary>,
) -> Result<(), ShortcutImportPlanError> {
    let imported_commands = artifact
        .portable_profile()
        .get("commands")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let known = COMMAND_REGISTRY
        .iter()
        .map(|command| command.stable_id())
        .collect::<BTreeSet<_>>();
    let mut working_value = serde_json::to_value(&*working)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    let baseline_value = serde_json::to_value(baseline)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    let working_commands = working_value
        .as_object_mut()
        .and_then(|profile| profile.get_mut("commands"))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            ShortcutImportPlanError::Serialization(
                "shortcut profile has no command object".to_owned(),
            )
        })?;
    let baseline_commands = baseline_value
        .get("commands")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let full_context_coverage = artifact
        .manifest()
        .coverage
        .contexts
        .iter()
        .any(|context| context == "all");
    let full_platform_coverage = CommandPlatform::ALL
        .into_iter()
        .all(|platform| artifact.manifest().coverage.covers_platform(platform));

    if merge_policy == ShortcutMergePolicy::ReplaceCurrentUserBindings
        && full_context_coverage
        && full_platform_coverage
    {
        for id in baseline_commands.keys() {
            if !known.contains(id.as_str()) && !imported_commands.contains_key(id) {
                working_commands.remove(id);
                increment_summary(summaries, None, |summary| summary.replaced += 1);
            }
        }
    }
    for (id, imported) in imported_commands {
        if known.contains(id.as_str()) {
            continue;
        }
        // This build cannot safely project an unknown record by platform or
        // context. Only a complete artifact may replace it atomically.
        if !full_context_coverage || !full_platform_coverage {
            increment_summary(summaries, None, |summary| summary.omitted += 1);
            continue;
        }
        let current = baseline_commands.get(&id);
        let differs = merge_policy == ShortcutMergePolicy::MergeNonConflicting
            && current.is_some_and(|current| current != &imported);
        if differs {
            let decision = match conflict_policy {
                ShortcutConflictPolicy::KeepCurrentAndReport => ShortcutImportDecision::KeptCurrent,
                ShortcutConflictPolicy::UseImportedBinding => {
                    working_commands.insert(id.clone(), imported.clone());
                    ShortcutImportDecision::ReplacedCurrent
                }
                ShortcutConflictPolicy::LeaveBothUnbound => {
                    working_commands.remove(&id);
                    ShortcutImportDecision::UnboundBoth
                }
            };
            conflicts.push(ShortcutImportConflict {
                kind: ShortcutImportConflictKind::UnknownCommand,
                command_id: id,
                conflicting_command_id: None,
                slot: None,
                platform: None,
                current: Some("retained future command record".to_owned()),
                imported: Some("imported future command record".to_owned()),
                decision,
            });
            increment_summary(summaries, None, |summary| {
                summary.conflicts += 1;
                match decision {
                    ShortcutImportDecision::KeptCurrent => summary.kept += 1,
                    ShortcutImportDecision::ReplacedCurrent => summary.replaced += 1,
                    ShortcutImportDecision::UnboundBoth => summary.unbound += 1,
                    _ => {}
                }
            });
        } else {
            working_commands.insert(id, imported);
            increment_summary(summaries, None, |summary| summary.imported += 1);
        }
    }
    *working = serde_json::from_value(working_value)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    Ok(())
}

fn merge_unknown_profile_fields(
    working: &mut ShortcutPreferences,
    baseline: &ShortcutPreferences,
    artifact: &DecodedShortcutArtifact,
    merge_policy: ShortcutMergePolicy,
    conflict_policy: ShortcutConflictPolicy,
    conflicts: &mut Vec<ShortcutImportConflict>,
    summaries: &mut BTreeMap<Option<ImportBindingClass>, ImportClassSummary>,
) -> Result<(), ShortcutImportPlanError> {
    const KNOWN_PROFILE_FIELDS: [&str; 3] = [
        "policies",
        "commands",
        "protected-override-acknowledgements",
    ];
    let mut working_value = serde_json::to_value(&*working)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    let baseline_value = serde_json::to_value(baseline)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    let working_object = working_value.as_object_mut().ok_or_else(|| {
        ShortcutImportPlanError::Serialization("shortcut profile is not an object".to_owned())
    })?;
    let baseline_object = baseline_value.as_object().ok_or_else(|| {
        ShortcutImportPlanError::Serialization("base shortcut profile is not an object".to_owned())
    })?;
    let imported_object = artifact.portable_profile().as_object().ok_or_else(|| {
        ShortcutImportPlanError::Serialization(
            "imported shortcut profile is not an object".to_owned(),
        )
    })?;
    let known = KNOWN_PROFILE_FIELDS.into_iter().collect::<BTreeSet<_>>();
    let imported_extras = imported_object
        .iter()
        .filter(|(key, _)| !known.contains(key.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<BTreeMap<_, _>>();
    let baseline_extras = baseline_object
        .iter()
        .filter(|(key, _)| !known.contains(key.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<BTreeMap<_, _>>();
    let full_coverage = artifact
        .manifest()
        .coverage
        .contexts
        .iter()
        .any(|context| context == "all")
        && CommandPlatform::ALL
            .into_iter()
            .all(|platform| artifact.manifest().coverage.covers_platform(platform));
    if !full_coverage {
        increment_summary(summaries, None, |summary| {
            summary.omitted += imported_extras.len();
        });
        return Ok(());
    }

    if merge_policy == ShortcutMergePolicy::ReplaceCurrentUserBindings {
        for key in baseline_extras.keys() {
            if !imported_extras.contains_key(key) {
                working_object.remove(key);
                increment_summary(summaries, None, |summary| summary.replaced += 1);
            }
        }
    }
    for (key, imported) in imported_extras {
        let current = baseline_extras.get(&key);
        if merge_policy == ShortcutMergePolicy::MergeNonConflicting
            && current.is_some_and(|current| current != &imported)
        {
            let decision = match conflict_policy {
                ShortcutConflictPolicy::KeepCurrentAndReport => ShortcutImportDecision::KeptCurrent,
                ShortcutConflictPolicy::UseImportedBinding => {
                    working_object.insert(key.clone(), imported);
                    ShortcutImportDecision::ReplacedCurrent
                }
                ShortcutConflictPolicy::LeaveBothUnbound => {
                    working_object.remove(&key);
                    ShortcutImportDecision::UnboundBoth
                }
            };
            conflicts.push(ShortcutImportConflict {
                kind: ShortcutImportConflictKind::UnknownCommand,
                command_id: format!("profile.{key}"),
                conflicting_command_id: None,
                slot: None,
                platform: None,
                current: Some("retained future profile field".to_owned()),
                imported: Some("imported future profile field".to_owned()),
                decision,
            });
            increment_summary(summaries, None, |summary| {
                summary.conflicts += 1;
                match decision {
                    ShortcutImportDecision::KeptCurrent => summary.kept += 1,
                    ShortcutImportDecision::ReplacedCurrent => summary.replaced += 1,
                    ShortcutImportDecision::UnboundBoth => summary.unbound += 1,
                    _ => {}
                }
            });
        } else {
            working_object.insert(key, imported);
            increment_summary(summaries, None, |summary| summary.imported += 1);
        }
    }
    *working = serde_json::from_value(working_value)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    Ok(())
}

#[derive(Debug, Clone)]
struct BlockingCollision {
    left: SliceKey,
    right: SliceKey,
    prefix: bool,
}

impl BlockingCollision {
    fn identity(&self) -> String {
        format!(
            "{}:{:?}:{:?}|{}:{:?}:{:?}|{}",
            self.left.command_id,
            self.left.slot,
            self.left.platform,
            self.right.command_id,
            self.right.slot,
            self.right.platform,
            self.prefix
        )
    }
}

fn blocking_collisions(profile: &ShortcutPreferences) -> Vec<BlockingCollision> {
    #[derive(Clone)]
    struct Entry {
        key: SliceKey,
        sequence: ShortcutSequence,
        context: ShortcutContext,
    }
    let mut entries = Vec::new();
    for command in COMMAND_REGISTRY.iter().copied() {
        // `effective_bindings` deliberately fails closed to immutable defaults
        // when a profile is invalid. Collision repair must inspect the actual
        // candidate that made the profile invalid, so use its resolved slices.
        for binding in collision_bindings(profile, command) {
            for platform in binding.platforms() {
                entries.push(Entry {
                    key: slice_key(command, binding.slot(), *platform),
                    sequence: binding.sequence().clone(),
                    context: command.shortcut_context(),
                });
            }
        }
    }
    let mut collisions = Vec::new();
    for left_index in 0..entries.len() {
        for right in &entries[(left_index + 1)..] {
            let left = &entries[left_index];
            if left.key.platform != right.key.platform
                || !shortcut_contexts_overlap(left.context, right.context)
                || shortcut_context_precedence_rank(
                    left.context,
                    profile.policies().context_precedence(),
                ) != shortcut_context_precedence_rank(
                    right.context,
                    profile.policies().context_precedence(),
                )
            {
                continue;
            }
            let exact = left.sequence == right.sequence;
            let prefix = left.sequence.is_prefix_of(&right.sequence)
                || right.sequence.is_prefix_of(&left.sequence);
            if exact || prefix {
                collisions.push(BlockingCollision {
                    left: left.key.clone(),
                    right: right.key.clone(),
                    prefix: !exact && prefix,
                });
            }
        }
    }
    collisions
}

fn collision_bindings(
    profile: &ShortcutPreferences,
    command: Command,
) -> Vec<crate::workbench::shortcuts::ResolvedShortcutBinding> {
    let bindings = profile.resolved_bindings(command);
    if !matches!(
        command.shortcut_context(),
        ShortcutContext::EditContext
            | ShortcutContext::EngineeringCanvas
            | ShortcutContext::SymbolCanvas
    ) {
        return bindings;
    }
    bindings
        .into_iter()
        .filter_map(|binding| {
            let [stroke] = binding.sequence().strokes() else {
                return Some(binding);
            };
            let stroke = *stroke;
            if stroke.primary() || stroke.alt() || stroke.shift() {
                return Some(binding);
            }
            match profile.policies().single_key_canvas() {
                SingleKeyCanvasPolicy::CanvasFocusOnly => Some(binding),
                SingleKeyCanvasPolicy::Disabled => None,
                SingleKeyCanvasPolicy::RequireAlt => Some(binding.with_sequence(
                    ShortcutSequence::single(ShortcutStroke::new(stroke.key(), false, true, false)),
                )),
            }
        })
        .collect()
}

fn binding_class(context: ShortcutContext) -> Option<ImportBindingClass> {
    Some(match context {
        ShortcutContext::Global
        | ShortcutContext::ApplicationChrome
        | ShortcutContext::RunnableProject => ImportBindingClass::Global,
        ShortcutContext::EditContext
        | ShortcutContext::EngineeringCanvas
        | ShortcutContext::SymbolCanvas
        | ShortcutContext::DesignWorkspace
        | ShortcutContext::ViolationNavigation => ImportBindingClass::Schematic,
        ShortcutContext::SimulationWorkspace => ImportBindingClass::Simulation,
        ShortcutContext::ResultsWorkspace => ImportBindingClass::Results,
        ShortcutContext::VerificationWorkspace => ImportBindingClass::Verification,
    })
}

fn summary_map() -> BTreeMap<Option<ImportBindingClass>, ImportClassSummary> {
    let mut summaries = BTreeMap::new();
    for class in [
        ImportBindingClass::Global,
        ImportBindingClass::Schematic,
        ImportBindingClass::Results,
        ImportBindingClass::Simulation,
        ImportBindingClass::Verification,
    ] {
        summaries.insert(
            Some(class),
            ImportClassSummary {
                binding_class: Some(class),
                ..ImportClassSummary::default()
            },
        );
    }
    summaries.insert(None, ImportClassSummary::default());
    summaries
}

fn increment_summary(
    summaries: &mut BTreeMap<Option<ImportBindingClass>, ImportClassSummary>,
    class: Option<ImportBindingClass>,
    increment: impl FnOnce(&mut ImportClassSummary),
) {
    increment(
        summaries
            .entry(class)
            .or_insert_with(|| ImportClassSummary {
                binding_class: class,
                ..ImportClassSummary::default()
            }),
    );
}

fn display_sequence(sequence: Option<&ShortcutSequence>) -> Option<String> {
    Some(sequence.map_or_else(|| "Unbound".to_owned(), ShortcutSequence::display_label))
}

pub fn shortcut_library_digest(
    library: &ShortcutProfileLibrary,
) -> Result<[u8; 32], ShortcutImportPlanError> {
    let mut value = serde_json::to_value(library)
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))?;
    if let Some(object) = value.as_object_mut() {
        object.remove("revision");
    }
    canonical_json_bytes(value)
        .map(|bytes| sha256(&bytes))
        .map_err(|error| ShortcutImportPlanError::Serialization(error.to_string()))
}

#[derive(Debug, Clone)]
pub struct ShortcutImportReceipt {
    id: Uuid,
    source_name: String,
    source_digest: [u8; 32],
    merge_policy: ShortcutMergePolicy,
    conflict_policy: ShortcutConflictPolicy,
    base_revision: u64,
    base_digest: [u8; 32],
    applied_revision: u64,
    applied_digest: [u8; 32],
    preset_name: Option<String>,
    summaries: Vec<ImportClassSummary>,
    omitted_envelope_fields: BTreeSet<String>,
    predecessor: ShortcutProfileLibrary,
}

impl ShortcutImportReceipt {
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }
    #[must_use]
    pub fn source_digest_hex(&self) -> String {
        hex_digest(self.source_digest)
    }
    #[must_use]
    pub const fn merge_policy(&self) -> ShortcutMergePolicy {
        self.merge_policy
    }
    #[must_use]
    pub const fn conflict_policy(&self) -> ShortcutConflictPolicy {
        self.conflict_policy
    }
    #[must_use]
    pub const fn base_revision(&self) -> u64 {
        self.base_revision
    }
    #[must_use]
    pub fn base_digest_hex(&self) -> String {
        hex_digest(self.base_digest)
    }
    #[must_use]
    pub const fn applied_revision(&self) -> u64 {
        self.applied_revision
    }
    #[must_use]
    pub fn preset_name(&self) -> Option<&str> {
        self.preset_name.as_deref()
    }
    #[must_use]
    pub fn summaries(&self) -> &[ImportClassSummary] {
        &self.summaries
    }
    #[must_use]
    pub fn omitted_envelope_fields(&self) -> &BTreeSet<String> {
        &self.omitted_envelope_fields
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutImportApplyError {
    PlanNotReady,
    StaleBase,
    StaleAppliedState,
    Persistence(String),
    Serialization(String),
    Library(ShortcutProfileLibraryError),
}

impl fmt::Display for ShortcutImportApplyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlanNotReady => write!(formatter, "shortcut import plan is not ready to apply"),
            Self::StaleBase => write!(
                formatter,
                "shortcut bindings changed after the import review was prepared"
            ),
            Self::StaleAppliedState => write!(
                formatter,
                "shortcut bindings changed after import; undo would erase later edits"
            ),
            Self::Persistence(error) | Self::Serialization(error) => error.fmt(formatter),
            Self::Library(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ShortcutImportApplyError {}

pub fn apply_shortcut_import(
    library: &mut ShortcutProfileLibrary,
    plan: &ShortcutImportPlan,
    persist: impl FnOnce(&ShortcutProfileLibrary) -> Result<(), String>,
) -> Result<ShortcutImportReceipt, ShortcutImportApplyError> {
    if !plan.can_apply() {
        return Err(ShortcutImportApplyError::PlanNotReady);
    }
    let current_digest = shortcut_library_digest(library)
        .map_err(|error| ShortcutImportApplyError::Serialization(error.to_string()))?;
    if library.revision() != plan.base_revision || current_digest != plan.base_digest {
        return Err(ShortcutImportApplyError::StaleBase);
    }
    let predecessor = library.clone();
    let mut published = library.clone();
    published
        .replace_content_from(&plan.candidate_library)
        .map_err(ShortcutImportApplyError::Library)?;
    persist(&published).map_err(ShortcutImportApplyError::Persistence)?;
    let applied_revision = published.revision();
    let applied_digest = shortcut_library_digest(&published)
        .map_err(|error| ShortcutImportApplyError::Serialization(error.to_string()))?;
    *library = published;
    Ok(ShortcutImportReceipt {
        id: Uuid::new_v4(),
        source_name: plan.source_name.clone(),
        source_digest: plan.source_digest,
        merge_policy: plan.merge_policy,
        conflict_policy: plan.conflict_policy,
        base_revision: plan.base_revision,
        base_digest: plan.base_digest,
        applied_revision,
        applied_digest,
        preset_name: plan.preset_name.clone(),
        summaries: plan.summaries.clone(),
        omitted_envelope_fields: plan.omitted_envelope_fields.clone(),
        predecessor,
    })
}

pub fn rollback_shortcut_import(
    library: &mut ShortcutProfileLibrary,
    receipt: &ShortcutImportReceipt,
    persist: impl FnOnce(&ShortcutProfileLibrary) -> Result<(), String>,
) -> Result<(), ShortcutImportApplyError> {
    let digest = shortcut_library_digest(library)
        .map_err(|error| ShortcutImportApplyError::Serialization(error.to_string()))?;
    if library.revision() != receipt.applied_revision || digest != receipt.applied_digest {
        return Err(ShortcutImportApplyError::StaleAppliedState);
    }
    let mut restored = library.clone();
    restored
        .replace_content_from(&receipt.predecessor)
        .map_err(ShortcutImportApplyError::Library)?;
    persist(&restored).map_err(ShortcutImportApplyError::Persistence)?;
    *library = restored;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::shortcut_artifacts::{
        ShortcutExportRequest, ShortcutExportScope, build_shortcut_reference_model,
        decode_shortcut_artifact_json, serialize_shortcut_reference_json,
    };
    use crate::workbench::shortcuts::{ProtectedShortcutPolicy, ShortcutStroke};
    use egui::Key;
    use egui::os::OperatingSystem;

    fn artifact_from(profile: &ShortcutPreferences) -> DecodedShortcutArtifact {
        let model = build_shortcut_reference_model(
            profile,
            &ShortcutExportRequest {
                scope: ShortcutExportScope::UserOverrides,
                include_platform_mappings: true,
                runtime_platform: CommandPlatform::Desktop,
                operating_system: OperatingSystem::Windows,
                current_contexts: Vec::new(),
            },
        )
        .unwrap();
        decode_shortcut_artifact_json(
            "import.json",
            &serialize_shortcut_reference_json(&model).unwrap(),
        )
        .unwrap()
    }

    fn artifact_with_coverage(
        profile: &ShortcutPreferences,
        contexts: &[&str],
        platforms: &[CommandPlatform],
        policies_included: bool,
    ) -> DecodedShortcutArtifact {
        artifact_value_with_coverage(
            serde_json::to_value(profile).unwrap(),
            contexts,
            platforms,
            policies_included,
        )
    }

    fn artifact_value_with_coverage(
        profile: Value,
        contexts: &[&str],
        platforms: &[CommandPlatform],
        policies_included: bool,
    ) -> DecodedShortcutArtifact {
        let envelope = serde_json::json!({
            "format": "rspice.shortcuts/1",
            "artifact": {
                "schemaVersion": 1,
                "scope": "user-overrides",
                "coverage": {
                    "contexts": contexts,
                    "platforms": platforms,
                    "policiesIncluded": policies_included
                },
                "platformMappingsIncluded": true
            },
            "profile": profile
        });
        decode_shortcut_artifact_json("import.json", &serde_json::to_string(&envelope).unwrap())
            .unwrap()
    }

    fn set(
        profile: &mut ShortcutPreferences,
        command: Command,
        slot: ShortcutBindingSlot,
        platform: CommandPlatform,
        key: Key,
    ) {
        profile
            .set_binding(
                command,
                slot,
                vec![platform],
                Some(ShortcutSequence::single(ShortcutStroke::new(
                    key, false, false, false,
                ))),
            )
            .unwrap();
    }

    fn set_sequence(
        profile: &mut ShortcutPreferences,
        command: Command,
        slot: ShortcutBindingSlot,
        platform: CommandPlatform,
        keys: &[Key],
    ) {
        profile
            .set_binding(
                command,
                slot,
                vec![platform],
                Some(
                    ShortcutSequence::new(
                        keys.iter()
                            .copied()
                            .map(|key| ShortcutStroke::new(key, false, false, false))
                            .collect(),
                    )
                    .unwrap(),
                ),
            )
            .unwrap();
    }

    fn with_command_extra(
        profile: &ShortcutPreferences,
        command: Command,
        key: &str,
        value: Value,
    ) -> ShortcutPreferences {
        let mut raw = serde_json::to_value(profile).unwrap();
        raw["commands"][command.stable_id()][key] = value;
        serde_json::from_value(raw).unwrap()
    }

    fn with_binding_extra(
        profile: &ShortcutPreferences,
        command: Command,
        platform: CommandPlatform,
        key: &str,
        value: Value,
    ) -> ShortcutPreferences {
        let mut raw = serde_json::to_value(profile).unwrap();
        let platform = serde_json::to_value(platform).unwrap();
        let binding = raw["commands"][command.stable_id()]["bindings"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|binding| {
                binding["slot"] == "primary"
                    && binding["platforms"]
                        .as_array()
                        .is_some_and(|platforms| platforms.contains(&platform))
            })
            .unwrap();
        binding[key] = value;
        serde_json::from_value(raw).unwrap()
    }

    #[test]
    fn same_target_conflict_honors_every_resolution_policy() {
        let cases = [
            (
                ShortcutConflictPolicy::KeepCurrentAndReport,
                "F6",
                ShortcutImportDecision::KeptCurrent,
            ),
            (
                ShortcutConflictPolicy::UseImportedBinding,
                "F7",
                ShortcutImportDecision::ReplacedCurrent,
            ),
            (
                ShortcutConflictPolicy::LeaveBothUnbound,
                "",
                ShortcutImportDecision::UnboundBoth,
            ),
        ];
        for (conflict_policy, expected_label, expected_decision) in cases {
            let mut base = ShortcutProfileLibrary::default();
            set(
                base.active_mut().unwrap(),
                Command::ZoomFit,
                ShortcutBindingSlot::Primary,
                CommandPlatform::Desktop,
                Key::F6,
            );
            let mut imported = ShortcutPreferences::default();
            set(
                &mut imported,
                Command::ZoomFit,
                ShortcutBindingSlot::Primary,
                CommandPlatform::Desktop,
                Key::F7,
            );
            let plan = plan_shortcut_import(
                &base,
                &artifact_from(&imported),
                &ShortcutImportOptions {
                    conflict_policy,
                    ..ShortcutImportOptions::default()
                },
            )
            .unwrap();
            assert!(plan.can_apply());
            assert!(plan.conflicts().iter().any(|conflict| {
                conflict.kind() == ShortcutImportConflictKind::SameTarget
                    && conflict.platform() == Some(CommandPlatform::Desktop)
                    && conflict.decision() == expected_decision
            }));
            assert_eq!(
                plan.candidate_library().active().resolved_label(
                    Command::ZoomFit,
                    CommandPlatform::Desktop,
                    OperatingSystem::Windows
                ),
                expected_label
            );
        }
    }

    #[test]
    fn disjoint_platform_slices_of_one_command_merge_without_conflict() {
        let mut base = ShortcutProfileLibrary::default();
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Browser,
            Key::F7,
        );
        let plan = plan_shortcut_import(
            &base,
            &artifact_with_coverage(
                &imported,
                &[Command::ZoomFit.shortcut_context().label()],
                &[CommandPlatform::Browser],
                false,
            ),
            &ShortcutImportOptions::default(),
        )
        .unwrap();

        assert!(plan.can_apply());
        assert!(
            plan.conflicts()
                .iter()
                .all(|conflict| conflict.kind() != ShortcutImportConflictKind::SameTarget)
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            "F6"
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Browser,
                OperatingSystem::Windows
            ),
            "F7"
        );
    }

    #[test]
    fn exact_cross_command_collision_honors_every_resolution_policy() {
        let cases = [
            (
                ShortcutConflictPolicy::KeepCurrentAndReport,
                "F6",
                "S",
                ShortcutImportDecision::KeptCurrent,
            ),
            (
                ShortcutConflictPolicy::UseImportedBinding,
                "",
                "F6",
                ShortcutImportDecision::ReplacedCurrent,
            ),
            (
                ShortcutConflictPolicy::LeaveBothUnbound,
                "",
                "",
                ShortcutImportDecision::UnboundBoth,
            ),
        ];
        for (conflict_policy, zoom_label, select_label, expected_decision) in cases {
            let mut base = ShortcutProfileLibrary::default();
            set(
                base.active_mut().unwrap(),
                Command::ZoomFit,
                ShortcutBindingSlot::Primary,
                CommandPlatform::Desktop,
                Key::F6,
            );
            let mut imported = ShortcutPreferences::default();
            set(
                &mut imported,
                Command::SelectTool,
                ShortcutBindingSlot::Primary,
                CommandPlatform::Desktop,
                Key::F6,
            );
            let plan = plan_shortcut_import(
                &base,
                &artifact_from(&imported),
                &ShortcutImportOptions {
                    conflict_policy,
                    ..ShortcutImportOptions::default()
                },
            )
            .unwrap();

            assert!(plan.can_apply());
            assert!(plan.conflicts().iter().any(|conflict| {
                conflict.kind() == ShortcutImportConflictKind::ExactCollision
                    && conflict.decision() == expected_decision
            }));
            assert_eq!(
                plan.candidate_library().active().resolved_label(
                    Command::ZoomFit,
                    CommandPlatform::Desktop,
                    OperatingSystem::Windows
                ),
                zoom_label
            );
            assert_eq!(
                plan.candidate_library().active().resolved_label(
                    Command::SelectTool,
                    CommandPlatform::Desktop,
                    OperatingSystem::Windows
                ),
                select_label
            );
        }
    }

    #[test]
    fn prefix_collision_moves_a_multistroke_sequence_atomically() {
        let mut base = ShortcutProfileLibrary::default();
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let mut imported = ShortcutPreferences::default();
        set_sequence(
            &mut imported,
            Command::SelectTool,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            &[Key::F6, Key::F7],
        );
        let plan = plan_shortcut_import(
            &base,
            &artifact_from(&imported),
            &ShortcutImportOptions {
                conflict_policy: ShortcutConflictPolicy::UseImportedBinding,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();

        assert!(plan.can_apply());
        assert!(plan.conflicts().iter().any(|conflict| {
            conflict.kind() == ShortcutImportConflictKind::PrefixCollision
                && conflict.decision() == ShortcutImportDecision::ReplacedCurrent
        }));
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            ""
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::SelectTool,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            "F6 F7"
        );
    }

    #[test]
    fn single_platform_replace_preserves_other_platform_slices() {
        let mut base = ShortcutProfileLibrary::default();
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Browser,
            Key::F7,
        );
        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F8,
        );
        let plan = plan_shortcut_import(
            &base,
            &artifact_with_coverage(
                &imported,
                &[Command::ZoomFit.shortcut_context().label()],
                &[CommandPlatform::Desktop],
                false,
            ),
            &ShortcutImportOptions {
                merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();

        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            "F8"
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Browser,
                OperatingSystem::Windows
            ),
            "F7"
        );
    }

    #[test]
    fn covered_unbind_only_removes_the_covered_platform_slice() {
        let mut base = ShortcutProfileLibrary::default();
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        set(
            base.active_mut().unwrap(),
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Browser,
            Key::F7,
        );
        let mut imported = ShortcutPreferences::default();
        imported
            .set_binding(
                Command::ZoomFit,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                None,
            )
            .unwrap();
        let plan = plan_shortcut_import(
            &base,
            &artifact_with_coverage(
                &imported,
                &[Command::ZoomFit.shortcut_context().label()],
                &[CommandPlatform::Desktop],
                false,
            ),
            &ShortcutImportOptions {
                merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();

        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            ""
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Browser,
                OperatingSystem::Windows
            ),
            "F7"
        );
    }

    #[test]
    fn partial_context_replace_does_not_leak_uncovered_records_or_extras() {
        let mut base_profile = ShortcutPreferences::default();
        set(
            &mut base_profile,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        set(
            &mut base_profile,
            Command::ToggleLinkedCursors,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F7,
        );
        let base_profile = with_command_extra(
            &base_profile,
            Command::ToggleLinkedCursors,
            "future-field",
            serde_json::json!({"source":"base"}),
        );
        let mut base_raw = serde_json::to_value(base_profile).unwrap();
        base_raw["commands"]["future-routing-command"] =
            serde_json::json!({"bindings":[], "source":"base"});
        let mut base = ShortcutProfileLibrary::default();
        base.replace_active(serde_json::from_value(base_raw).unwrap())
            .unwrap();

        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F8,
        );
        set(
            &mut imported,
            Command::ToggleLinkedCursors,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F9,
        );
        let imported = with_command_extra(
            &imported,
            Command::ToggleLinkedCursors,
            "future-field",
            serde_json::json!({"source":"imported"}),
        );
        let mut imported_raw = serde_json::to_value(imported).unwrap();
        imported_raw["commands"]["future-routing-command"] =
            serde_json::json!({"bindings":[], "source":"imported"});
        let plan = plan_shortcut_import(
            &base,
            &artifact_value_with_coverage(
                imported_raw,
                &[Command::ZoomFit.shortcut_context().label()],
                &[CommandPlatform::Desktop],
                false,
            ),
            &ShortcutImportOptions {
                merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();
        let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();

        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            "F8"
        );
        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ToggleLinkedCursors,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            "F7"
        );
        assert_eq!(
            candidate["commands"][Command::ToggleLinkedCursors.stable_id()]["future-field"],
            serde_json::json!({"source":"base"})
        );
        assert_eq!(
            candidate["commands"]["future-routing-command"]["source"],
            "base"
        );
    }

    #[test]
    fn known_command_and_binding_extension_fields_follow_their_platform_slice() {
        let mut base_profile = ShortcutPreferences::default();
        set(
            &mut base_profile,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Browser,
            Key::F7,
        );
        let base_profile = with_binding_extra(
            &base_profile,
            Command::ZoomFit,
            CommandPlatform::Browser,
            "future-binding",
            serde_json::json!({"source":"base"}),
        );
        let mut base = ShortcutProfileLibrary::default();
        base.replace_active(base_profile).unwrap();

        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F8,
        );
        let imported = with_command_extra(
            &with_binding_extra(
                &imported,
                Command::ZoomFit,
                CommandPlatform::Desktop,
                "future-binding",
                serde_json::json!({"source":"imported"}),
            ),
            Command::ZoomFit,
            "future-command",
            serde_json::json!({"mode":"portable"}),
        );
        let plan = plan_shortcut_import(
            &base,
            &artifact_with_coverage(
                &imported,
                &[Command::ZoomFit.shortcut_context().label()],
                &[CommandPlatform::Desktop],
                false,
            ),
            &ShortcutImportOptions::default(),
        )
        .unwrap();
        let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();
        let bindings = candidate["commands"][Command::ZoomFit.stable_id()]["bindings"]
            .as_array()
            .unwrap();
        let desktop = bindings
            .iter()
            .find(|binding| binding["platforms"] == serde_json::json!(["desktop"]))
            .unwrap();
        let browser = bindings
            .iter()
            .find(|binding| binding["platforms"] == serde_json::json!(["browser"]))
            .unwrap();

        assert_eq!(
            candidate["commands"][Command::ZoomFit.stable_id()]["future-command"],
            serde_json::json!({"mode":"portable"})
        );
        assert_eq!(
            desktop["future-binding"],
            serde_json::json!({"source":"imported"})
        );
        assert_eq!(
            browser["future-binding"],
            serde_json::json!({"source":"base"})
        );
    }

    #[test]
    fn full_replace_resets_omitted_known_and_unknown_command_overrides() {
        let mut profile = ShortcutPreferences::default();
        set(
            &mut profile,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let mut raw = serde_json::to_value(profile).unwrap();
        raw["commands"]["future-routing-command"] = serde_json::json!({
            "bindings": [],
            "future": 7
        });
        let mut base = ShortcutProfileLibrary::default();
        base.replace_active(serde_json::from_value(raw).unwrap())
            .unwrap();
        let plan = plan_shortcut_import(
            &base,
            &artifact_from(&ShortcutPreferences::default()),
            &ShortcutImportOptions {
                merge_policy: ShortcutMergePolicy::ReplaceCurrentUserBindings,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();
        let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();

        assert_eq!(
            plan.candidate_library().active().resolved_label(
                Command::ZoomFit,
                CommandPlatform::Desktop,
                OperatingSystem::Windows
            ),
            "F"
        );
        assert!(
            candidate["commands"]
                .get("future-routing-command")
                .is_none()
        );
    }

    #[test]
    fn imported_unknown_profile_and_command_records_are_preserved() {
        let mut profile = ShortcutPreferences::default();
        set(
            &mut profile,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let mut raw = serde_json::to_value(profile).unwrap();
        raw["future-profile"] = serde_json::json!({"mode":"portable"});
        raw["commands"]["future-routing-command"] = serde_json::json!({
            "bindings": [],
            "future": {"version": 7}
        });
        let plan = plan_shortcut_import(
            &ShortcutProfileLibrary::default(),
            &artifact_value_with_coverage(raw, &["all"], &CommandPlatform::ALL, true),
            &ShortcutImportOptions::default(),
        )
        .unwrap();
        let candidate = serde_json::to_value(plan.candidate_library().active()).unwrap();

        assert_eq!(candidate["future-profile"]["mode"], "portable");
        assert_eq!(
            candidate["commands"]["future-routing-command"]["future"]["version"],
            7
        );
    }

    #[test]
    fn unknown_envelope_metadata_is_reported_as_intentionally_omitted() {
        let envelope = serde_json::json!({
            "format": "rspice.shortcuts/1",
            "artifact": {
                "schemaVersion": 1,
                "scope": "user-overrides",
                "coverage": {
                    "contexts": ["all"],
                    "platforms": CommandPlatform::ALL,
                    "policiesIncluded": true
                },
                "platformMappingsIncluded": true
            },
            "profile": ShortcutPreferences::default(),
            "vendor-metadata": {"schema": 7}
        });
        let artifact = decode_shortcut_artifact_json(
            "import.json",
            &serde_json::to_string(&envelope).unwrap(),
        )
        .unwrap();
        let plan = plan_shortcut_import(
            &ShortcutProfileLibrary::default(),
            &artifact,
            &ShortcutImportOptions::default(),
        )
        .unwrap();

        assert_eq!(
            plan.omitted_envelope_fields(),
            &BTreeSet::from(["vendor-metadata".to_owned()])
        );
        assert_eq!(
            plan.summaries()
                .iter()
                .find(|summary| summary.binding_class.is_none())
                .unwrap()
                .omitted,
            1
        );
        assert!(
            serde_json::to_value(plan.candidate_library())
                .unwrap()
                .get("vendor-metadata")
                .is_none()
        );
        let mut library = ShortcutProfileLibrary::default();
        let receipt = apply_shortcut_import(&mut library, &plan, |_| Ok(())).unwrap();
        assert_eq!(
            receipt.omitted_envelope_fields(),
            plan.omitted_envelope_fields()
        );
    }

    #[test]
    fn protected_acknowledgements_are_reconfirmed_for_each_import() {
        let mut imported = ShortcutPreferences::default();
        imported
            .policies_mut()
            .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
        imported
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Alternate,
                vec![
                    CommandPlatform::Browser,
                    CommandPlatform::Tablet,
                    CommandPlatform::Phone,
                ],
                None,
            )
            .unwrap();
        imported.acknowledge_protected_override(Command::Save);
        let artifact = artifact_with_coverage(&imported, &["all"], &CommandPlatform::ALL, true);
        assert!(
            !artifact
                .profile()
                .protected_override_acknowledged(Command::Save)
        );
        let options = ShortcutImportOptions {
            conflict_policy: ShortcutConflictPolicy::UseImportedBinding,
            ..ShortcutImportOptions::default()
        };
        let review =
            plan_shortcut_import(&ShortcutProfileLibrary::default(), &artifact, &options).unwrap();

        assert!(!review.can_apply());
        assert_eq!(
            review.required_protected_confirmations(),
            &BTreeSet::from([Command::Save.stable_id().to_owned()])
        );
        let confirmed = plan_shortcut_import(
            &ShortcutProfileLibrary::default(),
            &artifact,
            &ShortcutImportOptions {
                protected_confirmations: BTreeSet::from([Command::Save.stable_id().to_owned()]),
                ..options
            },
        )
        .unwrap();
        assert!(confirmed.can_apply());
        assert!(
            confirmed
                .candidate_library()
                .active()
                .protected_override_acknowledged(Command::Save)
        );
    }

    #[test]
    fn named_preset_import_leaves_active_profile_byte_equivalent() {
        let mut base = ShortcutProfileLibrary::default();
        base.insert_named_preset("RF Review", ShortcutPreferences::default(), false)
            .unwrap();
        let active_before = serde_json::to_vec(base.active()).unwrap();
        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let options = ShortcutImportOptions {
            merge_policy: ShortcutMergePolicy::ImportNamedPreset,
            preset_name: Some("rf review".to_owned()),
            ..ShortcutImportOptions::default()
        };
        assert!(matches!(
            plan_shortcut_import(&base, &artifact_from(&imported), &options),
            Err(ShortcutImportPlanError::Preset(
                ShortcutProfileLibraryError::DuplicateName(_)
            ))
        ));
        let plan = plan_shortcut_import(
            &base,
            &artifact_from(&imported),
            &ShortcutImportOptions {
                overwrite_existing_preset: true,
                ..options
            },
        )
        .unwrap();
        assert_eq!(
            serde_json::to_vec(plan.candidate_library().active()).unwrap(),
            active_before
        );
        assert_eq!(
            plan.candidate_library()
                .named_preset("RF REVIEW")
                .unwrap()
                .profile()
                .resolved_label(
                    Command::ZoomFit,
                    CommandPlatform::Desktop,
                    OperatingSystem::Windows
                ),
            "F6"
        );
    }

    #[test]
    fn apply_persists_before_live_mutation_and_rollback_is_monotonic() {
        let mut library = ShortcutProfileLibrary::default();
        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::Save,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let plan = plan_shortcut_import(
            &library,
            &artifact_from(&imported),
            &ShortcutImportOptions {
                conflict_policy: ShortcutConflictPolicy::UseImportedBinding,
                ..ShortcutImportOptions::default()
            },
        )
        .unwrap();
        let before = library.clone();
        assert!(
            apply_shortcut_import(&mut library, &plan, |_| Err("disk full".to_owned())).is_err()
        );
        assert_eq!(library, before);
        let receipt = apply_shortcut_import(&mut library, &plan, |_| Ok(())).unwrap();
        let applied_revision = library.revision();
        rollback_shortcut_import(&mut library, &receipt, |_| Ok(())).unwrap();
        assert_eq!(library.revision(), applied_revision + 1);
        assert_eq!(
            shortcut_library_digest(&library).unwrap(),
            shortcut_library_digest(&before).unwrap()
        );
    }

    #[test]
    fn rollback_refuses_to_erase_a_later_edit() {
        let mut library = ShortcutProfileLibrary::default();
        let plan = plan_shortcut_import(
            &library,
            &artifact_from(&ShortcutPreferences::default()),
            &ShortcutImportOptions::default(),
        )
        .unwrap();
        let receipt = apply_shortcut_import(&mut library, &plan, |_| Ok(())).unwrap();
        let _ = library.active_mut().unwrap();
        assert_eq!(
            rollback_shortcut_import(&mut library, &receipt, |_| Ok(())).unwrap_err(),
            ShortcutImportApplyError::StaleAppliedState
        );
    }

    #[test]
    fn apply_rejects_a_stale_revision_before_persistence() {
        let mut library = ShortcutProfileLibrary::default();
        let plan = plan_shortcut_import(
            &library,
            &artifact_from(&ShortcutPreferences::default()),
            &ShortcutImportOptions::default(),
        )
        .unwrap();
        let _ = library.active_mut().unwrap();
        let mut persisted = false;

        assert_eq!(
            apply_shortcut_import(&mut library, &plan, |_| {
                persisted = true;
                Ok(())
            })
            .unwrap_err(),
            ShortcutImportApplyError::StaleBase
        );
        assert!(!persisted);
    }

    #[test]
    fn rollback_persistence_failure_leaves_applied_state_untouched() {
        let mut library = ShortcutProfileLibrary::default();
        let mut imported = ShortcutPreferences::default();
        set(
            &mut imported,
            Command::ZoomFit,
            ShortcutBindingSlot::Primary,
            CommandPlatform::Desktop,
            Key::F6,
        );
        let plan = plan_shortcut_import(
            &library,
            &artifact_from(&imported),
            &ShortcutImportOptions::default(),
        )
        .unwrap();
        let receipt = apply_shortcut_import(&mut library, &plan, |_| Ok(())).unwrap();
        let applied = library.clone();

        assert!(matches!(
            rollback_shortcut_import(&mut library, &receipt, |_| Err("disk full".to_owned())),
            Err(ShortcutImportApplyError::Persistence(_))
        ));
        assert_eq!(library, applied);
    }

    #[test]
    fn future_library_is_rejected_without_discarding_raw_state() {
        let source = serde_json::json!({
            "library-version": 99,
            "active": {"commands": {}},
            "named-presets": {"Future": {"commands": {}}},
            "revision": 12,
            "future-library": {"required": true}
        });
        let library: ShortcutProfileLibrary = serde_json::from_value(source.clone()).unwrap();

        assert_eq!(
            plan_shortcut_import(
                &library,
                &artifact_from(&ShortcutPreferences::default()),
                &ShortcutImportOptions::default()
            )
            .unwrap_err(),
            ShortcutImportPlanError::IncompatibleLibrary
        );
        assert_eq!(serde_json::to_value(library).unwrap(), source);
    }
}

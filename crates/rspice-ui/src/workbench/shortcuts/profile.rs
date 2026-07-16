use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use egui::os::OperatingSystem;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use super::{
    CommandShortcutOverride, ContextPrecedencePolicy, ProfileShortcutBinding,
    ProtectedShortcutPolicy, ResolvedShortcutBinding, ShortcutBindingSlot, ShortcutPolicies,
    ShortcutProfileError, ShortcutSequence, ShortcutStroke, SingleKeyCanvasPolicy,
};
use crate::product::CommandId;
use crate::workbench::commands::{
    COMMAND_REGISTRY, Command, CommandPlatform, ShortcutContext, ShortcutKind,
};

/// Persisted user shortcut profile. Command entries remain raw until they are
/// requested so a malformed or future command cannot invalidate the complete
/// preferences/session document.
#[derive(Debug, Clone, Default)]
pub struct ShortcutPreferences {
    policies: ShortcutPolicies,
    command_entries: BTreeMap<CommandId, Value>,
    malformed_command_entries: BTreeMap<String, Value>,
    protected_override_acknowledgements: BTreeSet<String>,
    unknown_fields: BTreeMap<String, Value>,
    malformed_root: Option<Value>,
    validation_cache: Cell<Option<bool>>,
}

impl PartialEq for ShortcutPreferences {
    fn eq(&self, other: &Self) -> bool {
        self.policies == other.policies
            && self.command_entries == other.command_entries
            && self.malformed_command_entries == other.malformed_command_entries
            && self.protected_override_acknowledgements == other.protected_override_acknowledgements
            && self.unknown_fields == other.unknown_fields
            && self.malformed_root == other.malformed_root
    }
}

impl Eq for ShortcutPreferences {}

#[derive(Serialize, Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
struct ShortcutPreferencesWire {
    policies: ShortcutPolicies,
    commands: BTreeMap<String, Value>,
    protected_override_acknowledgements: BTreeSet<String>,
    #[serde(flatten)]
    unknown_fields: BTreeMap<String, Value>,
}

impl Serialize for ShortcutPreferences {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(raw) = &self.malformed_root {
            return raw.serialize(serializer);
        }
        let mut commands = self.malformed_command_entries.clone();
        commands.extend(
            self.command_entries
                .iter()
                .map(|(id, value)| (id.as_str().to_owned(), value.clone())),
        );
        ShortcutPreferencesWire {
            policies: self.policies.clone(),
            commands,
            protected_override_acknowledgements: self.protected_override_acknowledgements.clone(),
            unknown_fields: self.unknown_fields.clone(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShortcutPreferences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        let wire = match serde_json::from_value::<ShortcutPreferencesWire>(raw.clone()) {
            Ok(wire) => wire,
            Err(_) => {
                return Ok(Self {
                    malformed_root: Some(raw),
                    ..Self::default()
                });
            }
        };
        let mut command_entries = BTreeMap::new();
        let mut malformed_command_entries = BTreeMap::new();
        for (raw_id, value) in wire.commands {
            match CommandId::new(raw_id.clone()) {
                Ok(id) => {
                    command_entries.insert(id, value);
                }
                Err(_) => {
                    malformed_command_entries.insert(raw_id, value);
                }
            }
        }
        Ok(Self {
            policies: wire.policies,
            command_entries,
            malformed_command_entries,
            protected_override_acknowledgements: wire.protected_override_acknowledgements,
            unknown_fields: wire.unknown_fields,
            malformed_root: None,
            validation_cache: Cell::new(None),
        })
    }
}

impl ShortcutPreferences {
    #[must_use]
    pub const fn policies(&self) -> &ShortcutPolicies {
        &self.policies
    }

    pub fn policies_mut(&mut self) -> &mut ShortcutPolicies {
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.protected_override_acknowledgements.clear();
        &mut self.policies
    }

    #[must_use]
    pub fn command_override(&self, command: Command) -> Option<CommandShortcutOverride> {
        self.command_entries
            .get(&command_id(command))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    pub fn set_command_override(
        &mut self,
        command: Command,
        command_override: CommandShortcutOverride,
    ) -> Result<(), ShortcutProfileError> {
        command_override.validate()?;
        let value = serde_json::to_value(command_override)
            .map_err(|error| ShortcutProfileError::InvalidBinding(error.to_string()))?;
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.clear_protected_override_acknowledgement(command);
        self.command_entries.insert(command_id(command), value);
        Ok(())
    }

    /// Replace one slot while retaining all other effective bindings. Starting
    /// from the resolved projection is important: the first edit materializes
    /// the complete immutable default profile before changing only this slot.
    pub fn set_binding(
        &mut self,
        command: Command,
        slot: ShortcutBindingSlot,
        platforms: Vec<CommandPlatform>,
        sequence: Option<ShortcutSequence>,
    ) -> Result<(), ShortcutProfileError> {
        let bindings = self
            .resolved_bindings(command)
            .into_iter()
            .map(|binding| {
                ProfileShortcutBinding::new(
                    binding.slot(),
                    binding.platforms().to_vec(),
                    binding.sequence().clone(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut command_override = CommandShortcutOverride::new(bindings)?;
        let replacement = sequence
            .map(|sequence| ProfileShortcutBinding::new(slot, platforms, sequence))
            .transpose()?
            .into_iter()
            .collect();
        command_override.replace_slot(slot, replacement)?;
        self.set_command_override(command, command_override)
    }

    pub fn reset_command(&mut self, command: Command) {
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.command_entries.remove(&command_id(command));
        self.clear_protected_override_acknowledgement(command);
    }

    /// Reset every command understood by this build. Unknown future commands
    /// stay byte-semantically intact when an older build edits the profile.
    pub fn reset_all(&mut self) {
        self.malformed_root = None;
        self.validation_cache.set(None);
        for command in COMMAND_REGISTRY.iter().copied() {
            self.command_entries.remove(&command_id(command));
            self.protected_override_acknowledgements
                .remove(command.stable_id());
        }
    }

    /// Remove only data this build can prove is incompatible. Valid unknown
    /// future commands and extension fields are intentionally preserved.
    /// Returned identifiers are suitable for a user-facing repair receipt.
    pub fn remove_blocking_invalid_entries(&mut self) -> Vec<String> {
        if self.malformed_root.take().is_some() {
            *self = Self::default();
            return vec!["shortcut-profile-root".to_owned()];
        }

        let mut removed = self
            .policies
            .repair_unknown_execution_policies()
            .into_iter()
            .map(|name| format!("policy:{name}"))
            .collect::<Vec<_>>();

        removed.extend(self.malformed_command_entries.keys().cloned());
        self.malformed_command_entries.clear();

        let malformed_known_commands = self
            .command_entries
            .iter()
            .filter_map(|(id, value)| {
                let command = Command::from_stable_id(id.as_str())?;
                let invalid = match serde_json::from_value::<CommandShortcutOverride>(value.clone())
                {
                    Ok(command_override) => command_override.validate().is_err(),
                    Err(_) => true,
                };
                invalid.then_some((id.clone(), command))
            })
            .collect::<Vec<_>>();
        for (id, command) in malformed_known_commands {
            self.command_entries.remove(&id);
            self.protected_override_acknowledgements
                .remove(command.stable_id());
            removed.push(id.as_str().to_owned());
        }

        if !removed.is_empty() {
            self.validation_cache.set(None);
        }
        removed
    }

    pub fn acknowledge_protected_override(&mut self, command: Command) {
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.protected_override_acknowledgements
            .insert(command.stable_id().to_owned());
    }

    #[must_use]
    pub fn protected_override_acknowledged(&self, command: Command) -> bool {
        self.protected_override_acknowledgements
            .contains(command.stable_id())
    }

    pub fn clear_protected_override_acknowledgement(&mut self, command: Command) {
        self.validation_cache.set(None);
        self.protected_override_acknowledgements
            .remove(command.stable_id());
    }

    #[must_use]
    pub fn resolved_bindings(&self, command: Command) -> Vec<ResolvedShortcutBinding> {
        if let Some(command_override) = self.command_override(command) {
            return command_override
                .bindings()
                .iter()
                .map(ResolvedShortcutBinding::from_profile)
                .collect();
        }
        default_bindings(command)
    }

    /// Runtime/display projection. Any profile error fails closed to the
    /// complete immutable registry so execution, visible labels, and
    /// accessibility metadata cannot disagree about a partially valid file.
    #[must_use]
    pub fn effective_bindings(&self, command: Command) -> Vec<ResolvedShortcutBinding> {
        if self.is_effective_profile_valid() {
            self.policy_projected_bindings(command)
        } else {
            default_bindings(command)
        }
    }

    fn policy_projected_bindings(&self, command: Command) -> Vec<ResolvedShortcutBinding> {
        let bindings = self.resolved_bindings(command);
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
                match self.policies.single_key_canvas() {
                    SingleKeyCanvasPolicy::CanvasFocusOnly => Some(binding),
                    SingleKeyCanvasPolicy::Disabled => None,
                    SingleKeyCanvasPolicy::RequireAlt => Some(binding.with_sequence(
                        ShortcutSequence::single(ShortcutStroke::new(
                            stroke.key(),
                            false,
                            true,
                            false,
                        )),
                    )),
                }
            })
            .collect()
    }

    #[must_use]
    pub fn resolved_labels(
        &self,
        command: Command,
        platform: CommandPlatform,
        operating_system: OperatingSystem,
    ) -> Vec<String> {
        self.effective_bindings(command)
            .into_iter()
            .filter(|binding| binding.supports(platform))
            .map(|binding| binding.sequence().display_label_for(operating_system))
            .collect()
    }

    #[must_use]
    pub fn resolved_label(
        &self,
        command: Command,
        platform: CommandPlatform,
        operating_system: OperatingSystem,
    ) -> String {
        self.resolved_labels(command, platform, operating_system)
            .into_iter()
            .next()
            .unwrap_or_default()
    }

    #[must_use]
    pub fn is_effective_profile_valid(&self) -> bool {
        if let Some(valid) = self.validation_cache.get() {
            return valid;
        }
        let valid = self.audit().is_valid();
        self.validation_cache.set(Some(valid));
        valid
    }

    #[must_use]
    pub fn audit(&self) -> ShortcutProfileAudit {
        let mut issues = Vec::new();
        if self.malformed_root.is_some() {
            issues.push(ShortcutProfileIssue::new(
                ShortcutProfileIssueSeverity::Error,
                ShortcutProfileIssueCode::MalformedProfileRoot,
                None,
                None,
                None,
                "shortcut profile root is not understood by this build".to_owned(),
            ));
        }
        for policy_name in self.policies.unknown_execution_policy_names() {
            issues.push(ShortcutProfileIssue::new(
                ShortcutProfileIssueSeverity::Error,
                ShortcutProfileIssueCode::UnknownPolicy,
                None,
                None,
                None,
                format!(
                    "shortcut execution policy '{policy_name}' is not understood by this build"
                ),
            ));
        }
        for raw_id in self.malformed_command_entries.keys() {
            issues.push(ShortcutProfileIssue::new(
                ShortcutProfileIssueSeverity::Error,
                ShortcutProfileIssueCode::MalformedCommandId,
                None,
                None,
                None,
                format!("'{raw_id}' is not a valid command identifier"),
            ));
        }

        for (id, value) in &self.command_entries {
            let Some(command) = Command::from_stable_id(id.as_str()) else {
                issues.push(ShortcutProfileIssue::new(
                    ShortcutProfileIssueSeverity::Warning,
                    ShortcutProfileIssueCode::UnknownCommand,
                    Some(id.clone()),
                    None,
                    None,
                    format!("{} belongs to a command not available in this build", id),
                ));
                continue;
            };
            match serde_json::from_value::<CommandShortcutOverride>(value.clone()) {
                Ok(command_override) => {
                    if let Err(error) = command_override.validate() {
                        issues.push(ShortcutProfileIssue::for_command(
                            ShortcutProfileIssueCode::InvalidBinding,
                            command,
                            None,
                            error.to_string(),
                        ));
                    }
                }
                Err(error) => {
                    let code = if error.to_string().contains("unsupported shortcut key") {
                        ShortcutProfileIssueCode::MalformedKey
                    } else {
                        ShortcutProfileIssueCode::MalformedOverride
                    };
                    issues.push(ShortcutProfileIssue::for_command(
                        code,
                        command,
                        None,
                        error.to_string(),
                    ));
                }
            }
        }

        let effective = COMMAND_REGISTRY
            .iter()
            .copied()
            .flat_map(|command| {
                self.policy_projected_bindings(command)
                    .into_iter()
                    .map(move |binding| EffectiveBinding { command, binding })
            })
            .collect::<Vec<_>>();

        for left_index in 0..effective.len() {
            for right_index in (left_index + 1)..effective.len() {
                let left = &effective[left_index];
                let right = &effective[right_index];
                let Some(platform) = shared_platform(&left.binding, &right.binding) else {
                    continue;
                };
                if !shortcut_contexts_overlap(
                    left.command.shortcut_context(),
                    right.command.shortcut_context(),
                ) {
                    continue;
                }
                let exact = left.binding.sequence() == right.binding.sequence();
                let prefix = left
                    .binding
                    .sequence()
                    .is_prefix_of(right.binding.sequence())
                    || right
                        .binding
                        .sequence()
                        .is_prefix_of(left.binding.sequence());
                if exact || prefix {
                    let code = if exact {
                        ShortcutProfileIssueCode::ExactCollision
                    } else {
                        ShortcutProfileIssueCode::PrefixCollision
                    };
                    let left_rank = shortcut_context_precedence_rank(
                        left.command.shortcut_context(),
                        self.policies.context_precedence(),
                    );
                    let right_rank = shortcut_context_precedence_rank(
                        right.command.shortcut_context(),
                        self.policies.context_precedence(),
                    );
                    let severity = if left_rank == right_rank {
                        ShortcutProfileIssueSeverity::Error
                    } else {
                        ShortcutProfileIssueSeverity::Warning
                    };
                    issues.push(ShortcutProfileIssue::collision(
                        severity, code, left, right, platform,
                    ));
                }
            }
        }

        for command in COMMAND_REGISTRY
            .iter()
            .copied()
            .filter(|command| command.primary_is_reserved_on(CommandPlatform::Browser))
        {
            let has_browser_alternate = self.resolved_bindings(command).iter().any(|binding| {
                binding.slot() == ShortcutBindingSlot::Alternate
                    && binding.supports(CommandPlatform::Browser)
            });
            if !has_browser_alternate {
                let acknowledged = self.policies.protected_shortcuts()
                    == ProtectedShortcutPolicy::AllowWithConfirmation
                    && self.protected_override_acknowledged(command);
                issues.push(ShortcutProfileIssue::for_command_with_severity(
                    if acknowledged {
                        ShortcutProfileIssueSeverity::Warning
                    } else {
                        ShortcutProfileIssueSeverity::Error
                    },
                    ShortcutProfileIssueCode::MissingBrowserAlternate,
                    command,
                    Some(ShortcutBindingSlot::Alternate),
                    "browser-reserved primary requires a browser alternate".to_owned(),
                ));
            }
        }

        let audit = ShortcutProfileAudit {
            binding_count: effective
                .iter()
                .map(|binding| binding.binding.platforms().len())
                .sum(),
            issues,
        };
        self.validation_cache.set(Some(audit.is_valid()));
        audit
    }
}

fn command_id(command: Command) -> CommandId {
    CommandId::new(command.stable_id()).expect("registry command IDs are validated product IDs")
}

fn default_bindings(command: Command) -> Vec<ResolvedShortcutBinding> {
    command
        .shortcut_bindings()
        .iter()
        .map(|binding| {
            let slot = match binding.kind {
                ShortcutKind::Primary => ShortcutBindingSlot::Primary,
                ShortcutKind::Alternate => ShortcutBindingSlot::Alternate,
            };
            ResolvedShortcutBinding::from_default(
                slot,
                binding.platforms.to_vec(),
                ShortcutSequence::single(ShortcutStroke::new(
                    binding.chord.key,
                    binding.chord.primary,
                    binding.chord.alt,
                    binding.chord.shift,
                )),
            )
        })
        .collect()
}

struct EffectiveBinding {
    command: Command,
    binding: ResolvedShortcutBinding,
}

fn shared_platform(
    left: &ResolvedShortcutBinding,
    right: &ResolvedShortcutBinding,
) -> Option<CommandPlatform> {
    CommandPlatform::ALL
        .into_iter()
        .find(|platform| left.supports(*platform) && right.supports(*platform))
}

pub(crate) const fn shortcut_contexts_overlap(
    left: ShortcutContext,
    right: ShortcutContext,
) -> bool {
    context_workspace_mask(left) & context_workspace_mask(right) != 0
        && context_view_mask(left) & context_view_mask(right) != 0
}

#[must_use]
pub const fn shortcut_context_precedence_rank(
    context: ShortcutContext,
    policy: ContextPrecedencePolicy,
) -> u8 {
    let modal = matches!(context, ShortcutContext::ApplicationChrome);
    let editor = matches!(
        context,
        ShortcutContext::EditContext
            | ShortcutContext::EngineeringCanvas
            | ShortcutContext::SymbolCanvas
    );
    if modal {
        return match policy {
            ContextPrecedencePolicy::ModalEditorWorkspaceGlobal => 0,
            ContextPrecedencePolicy::EditorModalWorkspaceGlobal => 1,
        };
    }
    if editor {
        return match policy {
            ContextPrecedencePolicy::ModalEditorWorkspaceGlobal => 1,
            ContextPrecedencePolicy::EditorModalWorkspaceGlobal => 0,
        };
    }
    if matches!(context, ShortcutContext::Global) {
        3
    } else {
        2
    }
}

const fn context_workspace_mask(context: ShortcutContext) -> u8 {
    match context {
        ShortcutContext::Global
        | ShortcutContext::ApplicationChrome
        | ShortcutContext::RunnableProject => 0b1_1111,
        ShortcutContext::EditContext => 0b1_0001,
        ShortcutContext::EngineeringCanvas => 0b0_0001,
        ShortcutContext::DesignWorkspace => 0b0_0001,
        ShortcutContext::SymbolCanvas => 0b1_0001,
        ShortcutContext::SimulationWorkspace => 0b0_0010,
        ShortcutContext::ResultsWorkspace => 0b0_0100,
        ShortcutContext::VerificationWorkspace => 0b0_1000,
        ShortcutContext::ViolationNavigation => 0b0_1001,
    }
}

/// View-family ownership complements the workspace mask so schematic and
/// symbol editors may intentionally reuse conventional single-key tools.
const fn context_view_mask(context: ShortcutContext) -> u8 {
    match context {
        ShortcutContext::EditContext => 0b0111,
        ShortcutContext::EngineeringCanvas => 0b0011,
        ShortcutContext::SymbolCanvas => 0b0100,
        ShortcutContext::Global
        | ShortcutContext::ApplicationChrome
        | ShortcutContext::DesignWorkspace
        | ShortcutContext::SimulationWorkspace
        | ShortcutContext::ResultsWorkspace
        | ShortcutContext::VerificationWorkspace
        | ShortcutContext::ViolationNavigation
        | ShortcutContext::RunnableProject => 0b1111,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutProfileIssueSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutProfileIssueCode {
    MalformedProfileRoot,
    MalformedCommandId,
    UnknownCommand,
    MalformedOverride,
    MalformedKey,
    InvalidBinding,
    ExactCollision,
    PrefixCollision,
    MissingBrowserAlternate,
    UnknownPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutProfileIssue {
    severity: ShortcutProfileIssueSeverity,
    code: ShortcutProfileIssueCode,
    command_id: Option<CommandId>,
    platform: Option<CommandPlatform>,
    slot: Option<ShortcutBindingSlot>,
    message: String,
}

impl ShortcutProfileIssue {
    fn new(
        severity: ShortcutProfileIssueSeverity,
        code: ShortcutProfileIssueCode,
        command_id: Option<CommandId>,
        platform: Option<CommandPlatform>,
        slot: Option<ShortcutBindingSlot>,
        message: String,
    ) -> Self {
        Self {
            severity,
            code,
            command_id,
            platform,
            slot,
            message,
        }
    }

    fn for_command(
        code: ShortcutProfileIssueCode,
        command: Command,
        slot: Option<ShortcutBindingSlot>,
        message: String,
    ) -> Self {
        Self::for_command_with_severity(
            ShortcutProfileIssueSeverity::Error,
            code,
            command,
            slot,
            message,
        )
    }

    fn for_command_with_severity(
        severity: ShortcutProfileIssueSeverity,
        code: ShortcutProfileIssueCode,
        command: Command,
        slot: Option<ShortcutBindingSlot>,
        message: String,
    ) -> Self {
        Self::new(
            severity,
            code,
            Some(command_id(command)),
            None,
            slot,
            message,
        )
    }

    fn collision(
        severity: ShortcutProfileIssueSeverity,
        code: ShortcutProfileIssueCode,
        left: &EffectiveBinding,
        right: &EffectiveBinding,
        platform: CommandPlatform,
    ) -> Self {
        Self::new(
            severity,
            code,
            Some(command_id(left.command)),
            Some(platform),
            Some(left.binding.slot()),
            format!(
                "{} conflicts with {} on {} ({})",
                left.command.stable_id(),
                right.command.stable_id(),
                platform.label(),
                left.binding.display_label()
            ),
        )
    }

    #[must_use]
    pub const fn severity(&self) -> ShortcutProfileIssueSeverity {
        self.severity
    }

    #[must_use]
    pub const fn code(&self) -> ShortcutProfileIssueCode {
        self.code
    }

    #[must_use]
    pub const fn command_id(&self) -> Option<&CommandId> {
        self.command_id.as_ref()
    }

    #[must_use]
    pub const fn platform(&self) -> Option<CommandPlatform> {
        self.platform
    }

    #[must_use]
    pub const fn slot(&self) -> Option<ShortcutBindingSlot> {
        self.slot
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutProfileAudit {
    binding_count: usize,
    issues: Vec<ShortcutProfileIssue>,
}

impl ShortcutProfileAudit {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.issues
            .iter()
            .all(|issue| issue.severity != ShortcutProfileIssueSeverity::Error)
    }

    #[must_use]
    pub const fn binding_count(&self) -> usize {
        self.binding_count
    }

    #[must_use]
    pub fn issues(&self) -> &[ShortcutProfileIssue] {
        &self.issues
    }
}

#[cfg(test)]
mod tests {
    use egui::Key;

    use super::*;
    use crate::state::ComponentType;
    use crate::workbench::state::Workspace;

    fn sequence(key: Key, primary: bool) -> ShortcutSequence {
        ShortcutSequence::single(ShortcutStroke::new(key, primary, false, false))
    }

    #[test]
    fn immutable_workspace_ownership_projects_to_results_and_verify() {
        let preferences = ShortcutPreferences::default();
        assert_eq!(
            preferences.resolved_bindings(Command::OpenWorkspace(Workspace::Results))[0]
                .display_label(),
            "Alt+4"
        );
        assert_eq!(
            preferences.resolved_bindings(Command::OpenWorkspace(Workspace::Verify))[0]
                .display_label(),
            "Alt+5"
        );
        assert!(
            preferences
                .resolved_bindings(Command::ResultViewer(crate::workbench::ResultViewer::Waves))
                .is_empty()
        );
        let audit = preferences.audit();
        assert!(audit.is_valid(), "default shortcut profile: {audit:#?}");
    }

    #[test]
    fn default_profile_is_valid_with_complete_symbol_tool_registry() {
        let preferences = ShortcutPreferences::default();
        let audit = preferences.audit();
        assert!(audit.is_valid(), "default shortcut profile: {audit:#?}");

        for command in [
            Command::SelectTool,
            Command::SymbolPinTool,
            Command::SymbolPolylineTool,
            Command::SymbolCircleTool,
            Command::SymbolArcTool,
            Command::SymbolArrowTool,
            Command::SymbolDotTool,
            Command::ZoomFit,
            Command::Cancel,
        ] {
            assert!(
                !preferences.effective_bindings(command).is_empty(),
                "missing effective symbol-editor binding: {command:?}"
            );
        }
    }

    #[test]
    fn editing_one_slot_preserves_the_other_then_resets_to_defaults() {
        let mut preferences = ShortcutPreferences::default();
        let command = Command::Save;
        let default_alternate = preferences
            .resolved_bindings(command)
            .into_iter()
            .find(|binding| binding.slot() == ShortcutBindingSlot::Alternate)
            .unwrap()
            .display_label();

        preferences
            .set_binding(
                command,
                ShortcutBindingSlot::Primary,
                CommandPlatform::ALL.to_vec(),
                Some(sequence(Key::F6, false)),
            )
            .unwrap();
        let resolved = preferences.resolved_bindings(command);
        assert_eq!(
            resolved
                .iter()
                .find(|binding| binding.slot() == ShortcutBindingSlot::Primary)
                .unwrap()
                .display_label(),
            "F6"
        );
        assert_eq!(
            resolved
                .iter()
                .find(|binding| binding.slot() == ShortcutBindingSlot::Alternate)
                .unwrap()
                .display_label(),
            default_alternate
        );

        preferences.reset_command(command);
        assert!(preferences.command_override(command).is_none());
    }

    #[test]
    fn unknown_commands_fields_and_policy_values_round_trip() {
        let source = r#"{
            "policies":{"single-key-canvas":"future-policy","future-policy-field":7},
            "commands":{"future-command":{"bindings":[],"future-data":true}},
            "future-domain":{"version":2}
        }"#;
        let preferences: ShortcutPreferences = serde_json::from_str(source).unwrap();
        let encoded = serde_json::to_value(preferences).unwrap();
        assert_eq!(encoded["policies"]["single-key-canvas"], "future-policy");
        assert_eq!(encoded["policies"]["future-policy-field"], 7);
        assert_eq!(encoded["commands"]["future-command"]["future-data"], true);
        assert_eq!(encoded["future-domain"]["version"], 2);
    }

    #[test]
    fn malformed_keys_are_isolated_and_audited_without_hiding_defaults() {
        let preferences: ShortcutPreferences = serde_json::from_str(
            r#"{"commands":{"save-project":{"bindings":[{"slot":"primary","platforms":["desktop"],"sequence":[{"key":"DefinitelyNotAKey"}]}]}}}"#,
        )
        .unwrap();
        assert!(!preferences.resolved_bindings(Command::Save).is_empty());
        assert!(preferences.audit().issues().iter().any(|issue| {
            issue.code() == ShortcutProfileIssueCode::MalformedKey
                && issue
                    .command_id()
                    .is_some_and(|id| id.as_str() == "save-project")
        }));
        let encoded = serde_json::to_string(&preferences).unwrap();
        assert!(encoded.contains("DefinitelyNotAKey"));
    }

    #[test]
    fn binding_count_is_expanded_across_supported_platforms() {
        let preferences = ShortcutPreferences::default();
        let expected: usize = COMMAND_REGISTRY
            .iter()
            .copied()
            .flat_map(|command| preferences.resolved_bindings(command))
            .map(|binding| binding.platforms().len())
            .sum();
        assert_eq!(preferences.audit().binding_count(), expected);
    }

    #[test]
    fn audit_finds_context_collisions_and_missing_browser_alternates() {
        let mut preferences = ShortcutPreferences::default();
        preferences
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Primary,
                CommandPlatform::ALL.to_vec(),
                Some(sequence(Key::W, false)),
            )
            .unwrap();
        preferences
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Alternate,
                CommandPlatform::ALL.to_vec(),
                None,
            )
            .unwrap();
        let audit = preferences.audit();
        assert!(
            audit
                .issues()
                .iter()
                .any(|issue| { issue.code() == ShortcutProfileIssueCode::ExactCollision })
        );
        assert!(
            audit
                .issues()
                .iter()
                .any(|issue| { issue.code() == ShortcutProfileIssueCode::MissingBrowserAlternate })
        );
    }

    #[test]
    fn single_key_canvas_policy_is_part_of_the_effective_projection() {
        let mut preferences = ShortcutPreferences::default();
        assert_eq!(
            preferences.resolved_label(
                Command::PlaceWire,
                CommandPlatform::Desktop,
                OperatingSystem::Windows,
            ),
            "W"
        );

        preferences
            .policies_mut()
            .set_single_key_canvas(SingleKeyCanvasPolicy::RequireAlt);
        assert_eq!(
            preferences.resolved_label(
                Command::PlaceWire,
                CommandPlatform::Desktop,
                OperatingSystem::Windows,
            ),
            "Alt+W"
        );

        preferences
            .policies_mut()
            .set_single_key_canvas(SingleKeyCanvasPolicy::Disabled);
        assert!(
            preferences
                .effective_bindings(Command::PlaceWire)
                .is_empty()
        );
    }

    #[test]
    fn context_precedence_warns_across_tiers_but_blocks_same_tier_collisions() {
        let mut across_tiers = ShortcutPreferences::default();
        across_tiers
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Primary,
                CommandPlatform::ALL.to_vec(),
                Some(sequence(Key::W, false)),
            )
            .unwrap();
        let audit = across_tiers.audit();
        assert!(
            audit.is_valid(),
            "precedence-resolved collision: {audit:#?}"
        );
        assert!(audit.issues().iter().any(|issue| {
            issue.code() == ShortcutProfileIssueCode::ExactCollision
                && issue.severity() == ShortcutProfileIssueSeverity::Warning
        }));

        let mut same_tier = ShortcutPreferences::default();
        same_tier
            .set_binding(
                Command::Place(ComponentType::Ground),
                ShortcutBindingSlot::Primary,
                CommandPlatform::ALL.to_vec(),
                Some(sequence(Key::W, false)),
            )
            .unwrap();
        let audit = same_tier.audit();
        assert!(!audit.is_valid());
        assert!(audit.issues().iter().any(|issue| {
            issue.code() == ShortcutProfileIssueCode::ExactCollision
                && issue.severity() == ShortcutProfileIssueSeverity::Error
        }));
    }

    #[test]
    fn protected_override_acknowledgement_is_persisted_and_invalidated_by_edits() {
        let mut preferences = ShortcutPreferences::default();
        preferences
            .policies_mut()
            .set_protected_shortcuts(ProtectedShortcutPolicy::AllowWithConfirmation);
        preferences
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
        assert!(!preferences.audit().is_valid());

        preferences.acknowledge_protected_override(Command::Save);
        assert!(preferences.audit().is_valid());
        let mut round_trip: ShortcutPreferences =
            serde_json::from_value(serde_json::to_value(&preferences).unwrap()).unwrap();
        assert!(round_trip.protected_override_acknowledged(Command::Save));

        round_trip
            .set_binding(
                Command::Save,
                ShortcutBindingSlot::Primary,
                vec![CommandPlatform::Desktop],
                Some(sequence(Key::F6, false)),
            )
            .unwrap();
        assert!(!round_trip.protected_override_acknowledged(Command::Save));
        assert!(!round_trip.audit().is_valid());
    }

    #[test]
    fn unknown_execution_policy_is_preserved_but_fails_closed_until_repaired() {
        let source = r#"{
            "policies":{"single-key-canvas":"future-policy"},
            "commands":{
                "toggle-navigator":{"bindings":[{
                    "slot":"primary",
                    "platforms":["desktop","browser","tablet","phone"],
                    "sequence":[{"key":"F6"}]
                }]},
                "future-command":{"bindings":[],"future-data":true}
            },
            "future-domain":{"version":2}
        }"#;
        let mut preferences: ShortcutPreferences = serde_json::from_str(source).unwrap();
        assert_eq!(
            preferences.resolved_bindings(Command::ToggleNavigator)[0].display_label(),
            "F6"
        );
        let audit = preferences.audit();
        assert!(!audit.is_valid());
        assert!(
            audit
                .issues()
                .iter()
                .any(|issue| issue.code() == ShortcutProfileIssueCode::UnknownPolicy)
        );
        assert_eq!(
            preferences.resolved_label(
                Command::ToggleNavigator,
                CommandPlatform::Desktop,
                OperatingSystem::Windows,
            ),
            "Ctrl+B"
        );
        let encoded = serde_json::to_value(&preferences).unwrap();
        assert_eq!(encoded["policies"]["single-key-canvas"], "future-policy");

        let receipt = preferences.remove_blocking_invalid_entries();
        assert_eq!(receipt, ["policy:single-key-canvas"]);
        assert!(preferences.audit().is_valid());
        assert_eq!(
            preferences.resolved_label(
                Command::ToggleNavigator,
                CommandPlatform::Desktop,
                OperatingSystem::Windows,
            ),
            "F6"
        );
        let repaired = serde_json::to_value(preferences).unwrap();
        assert_eq!(repaired["commands"]["future-command"]["future-data"], true);
        assert_eq!(repaired["future-domain"]["version"], 2);
    }
}

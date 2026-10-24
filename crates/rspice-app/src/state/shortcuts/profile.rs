//! A shortcut profile.
//!
//! The bindings themselves, and the context overlap rules that decide
//! whether two bindings actually conflict — the same chord in two contexts
//! that cannot both be active is not a conflict.

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
use crate::state::command_catalog::{
    CommandCatalog, CommandKey, CommandPlatform, ShortcutContext, ShortcutKind,
};

/// Persisted user shortcut profile. Command entries remain raw until they are
/// requested so a malformed or future command cannot invalidate the complete
/// preferences/session document. What a command ships with, and where its
/// keys act, come from a [`CommandCatalog`].
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
    pub fn command_override(&self, command: CommandKey) -> Option<CommandShortcutOverride> {
        self.command_entries
            .get(&command.id())
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    pub fn set_command_override(
        &mut self,
        command: CommandKey,
        command_override: CommandShortcutOverride,
    ) -> Result<(), ShortcutProfileError> {
        command_override.validate()?;
        let value = serde_json::to_value(command_override)
            .map_err(|error| ShortcutProfileError::InvalidBinding(error.to_string()))?;
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.clear_protected_override_acknowledgement(command);
        self.command_entries.insert(command.id(), value);
        Ok(())
    }

    /// Replace one slot while retaining all other effective bindings. Starting
    /// from the resolved projection is important: the first edit materializes
    /// the complete immutable default profile before changing only this slot.
    pub fn set_binding(
        &mut self,
        catalog: &dyn CommandCatalog,
        command: CommandKey,
        slot: ShortcutBindingSlot,
        platforms: Vec<CommandPlatform>,
        sequence: Option<ShortcutSequence>,
    ) -> Result<(), ShortcutProfileError> {
        let bindings = self
            .resolved_bindings(catalog, command)
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

    pub fn reset_command(&mut self, command: CommandKey) {
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.command_entries.remove(&command.id());
        self.clear_protected_override_acknowledgement(command);
    }

    /// Put every command this build knows back on its shipped keys and every
    /// binding policy back on its shipped value. Commands and fields from a
    /// newer build are kept as they are.
    pub fn restore_shipped(&mut self, catalog: &dyn CommandCatalog) {
        self.reset_all(catalog);
        self.policies_mut().restore_shipped();
    }

    /// Reset every command understood by this build. Unknown future commands
    /// stay byte-semantically intact when an older build edits the profile.
    pub fn reset_all(&mut self, catalog: &dyn CommandCatalog) {
        self.malformed_root = None;
        self.validation_cache.set(None);
        for command in catalog.commands().iter().copied() {
            self.command_entries.remove(&command.id());
            self.protected_override_acknowledgements
                .remove(command.as_str());
        }
    }

    /// Remove only data this build can prove is incompatible. Valid unknown
    /// future commands and extension fields are intentionally preserved.
    /// Returned identifiers are suitable for a user-facing repair receipt.
    pub fn remove_blocking_invalid_entries(&mut self, catalog: &dyn CommandCatalog) -> Vec<String> {
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
                let command = catalog.resolve(id.as_str())?;
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
                .remove(command.as_str());
            removed.push(id.as_str().to_owned());
        }

        if !removed.is_empty() {
            self.validation_cache.set(None);
        }
        removed
    }

    pub fn acknowledge_protected_override(&mut self, command: CommandKey) {
        self.malformed_root = None;
        self.validation_cache.set(None);
        self.protected_override_acknowledgements
            .insert(command.as_str().to_owned());
    }

    #[must_use]
    pub fn protected_override_acknowledged(&self, command: CommandKey) -> bool {
        self.protected_override_acknowledgements
            .contains(command.as_str())
    }

    pub fn clear_protected_override_acknowledgement(&mut self, command: CommandKey) {
        self.validation_cache.set(None);
        self.protected_override_acknowledgements
            .remove(command.as_str());
    }

    #[must_use]
    pub fn resolved_bindings(
        &self,
        catalog: &dyn CommandCatalog,
        command: CommandKey,
    ) -> Vec<ResolvedShortcutBinding> {
        if let Some(command_override) = self.command_override(command) {
            return command_override
                .bindings()
                .iter()
                .map(ResolvedShortcutBinding::from_profile)
                .collect();
        }
        default_bindings(catalog, command)
    }

    /// Runtime/display projection. Any profile error fails closed to the
    /// complete immutable registry so execution, visible labels, and
    /// accessibility metadata cannot disagree about a partially valid file.
    #[must_use]
    pub fn effective_bindings(
        &self,
        catalog: &dyn CommandCatalog,
        command: CommandKey,
    ) -> Vec<ResolvedShortcutBinding> {
        if self.is_effective_profile_valid(catalog) {
            self.policy_projected_bindings(
                catalog.context(command),
                self.resolved_bindings(catalog, command),
            )
        } else {
            default_bindings(catalog, command)
        }
    }

    /// The bindings as the single-key canvas policy lets them act in
    /// `context`.
    fn policy_projected_bindings(
        &self,
        context: Option<ShortcutContext>,
        bindings: Vec<ResolvedShortcutBinding>,
    ) -> Vec<ResolvedShortcutBinding> {
        if !matches!(
            context,
            Some(
                ShortcutContext::EditContext
                    | ShortcutContext::EngineeringCanvas
                    | ShortcutContext::SymbolCanvas
            )
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
        catalog: &dyn CommandCatalog,
        command: CommandKey,
        platform: CommandPlatform,
        operating_system: OperatingSystem,
    ) -> Vec<String> {
        self.effective_bindings(catalog, command)
            .into_iter()
            .filter(|binding| binding.supports(platform))
            .map(|binding| binding.sequence().display_label_for(operating_system))
            .collect()
    }

    #[must_use]
    pub fn resolved_label(
        &self,
        catalog: &dyn CommandCatalog,
        command: CommandKey,
        platform: CommandPlatform,
        operating_system: OperatingSystem,
    ) -> String {
        self.resolved_labels(catalog, command, platform, operating_system)
            .into_iter()
            .next()
            .unwrap_or_default()
    }

    #[must_use]
    pub fn is_effective_profile_valid(&self, catalog: &dyn CommandCatalog) -> bool {
        if let Some(valid) = self.validation_cache.get() {
            return valid;
        }
        let valid = self.audit(catalog).is_valid();
        self.validation_cache.set(Some(valid));
        valid
    }

    #[must_use]
    pub fn audit(&self, catalog: &dyn CommandCatalog) -> ShortcutProfileAudit {
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
            let Some(command) = catalog.resolve(id.as_str()) else {
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

        let effective = catalog
            .commands()
            .iter()
            .copied()
            .filter_map(|command| Some((command, catalog.context(command)?)))
            .flat_map(|(command, context)| {
                self.policy_projected_bindings(
                    Some(context),
                    self.resolved_bindings(catalog, command),
                )
                .into_iter()
                .map(move |binding| EffectiveBinding {
                    command,
                    context,
                    binding,
                })
            })
            .collect::<Vec<_>>();

        for left_index in 0..effective.len() {
            for right_index in (left_index + 1)..effective.len() {
                let left = &effective[left_index];
                let right = &effective[right_index];
                let Some(platform) = shared_platform(&left.binding, &right.binding) else {
                    continue;
                };
                if !shortcut_contexts_overlap(left.context, right.context) {
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
                        left.context,
                        self.policies.context_precedence(),
                    );
                    let right_rank = shortcut_context_precedence_rank(
                        right.context,
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

        for command in
            catalog.commands().iter().copied().filter(|command| {
                catalog.primary_is_reserved_on(*command, CommandPlatform::Browser)
            })
        {
            let has_browser_alternate =
                self.resolved_bindings(catalog, command)
                    .iter()
                    .any(|binding| {
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

fn default_bindings(
    catalog: &dyn CommandCatalog,
    command: CommandKey,
) -> Vec<ResolvedShortcutBinding> {
    catalog
        .default_bindings(command)
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
    command: CommandKey,
    context: ShortcutContext,
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

pub const fn shortcut_contexts_overlap(left: ShortcutContext, right: ShortcutContext) -> bool {
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
            | ShortcutContext::SourceEditor
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
        // Design, Models and Results: the edit context follows the surfaces
        // its commands can act on, and collision detection has to see the
        // Results bit or a Results chord and an edit chord could be declared
        // non-overlapping while both are live on the same sheet.
        ShortcutContext::EditContext => 0b1_0101,
        ShortcutContext::EngineeringCanvas => 0b0_0001,
        ShortcutContext::DesignWorkspace => 0b0_0001,
        ShortcutContext::SymbolCanvas => 0b1_0001,
        ShortcutContext::SimulationWorkspace => 0b0_0010,
        ShortcutContext::ResultsWorkspace => 0b0_0100,
        ShortcutContext::VerificationWorkspace => 0b0_1000,
        ShortcutContext::ViolationNavigation => 0b0_1001,
        // A Verilog-A view is a Design document. The Netlist workspace's
        // pages have no bit of their own; the global contexts that share
        // them are compared here through Design.
        ShortcutContext::SourceEditor => 0b0_0001,
    }
}

/// View-family ownership complements the workspace mask so schematic and
/// symbol editors may intentionally reuse conventional single-key tools.
const fn context_view_mask(context: ShortcutContext) -> u8 {
    match context {
        ShortcutContext::EditContext => 0b0111,
        ShortcutContext::EngineeringCanvas => 0b0011,
        ShortcutContext::SymbolCanvas => 0b0100,
        // Source documents: never a schematic or symbol canvas, so Ctrl+D can
        // be Duplicate line here and Duplicate on a drawing.
        ShortcutContext::SourceEditor => 0b1000,
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
        command: CommandKey,
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
        command: CommandKey,
        slot: Option<ShortcutBindingSlot>,
        message: String,
    ) -> Self {
        Self::new(severity, code, Some(command.id()), None, slot, message)
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
            Some(left.command.id()),
            Some(platform),
            Some(left.binding.slot()),
            format!(
                "{} conflicts with {} on {} ({})",
                left.command.as_str(),
                right.command.as_str(),
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
    #[cfg(test)]
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
    use super::*;

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
}

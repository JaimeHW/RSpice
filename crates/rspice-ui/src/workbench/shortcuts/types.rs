use std::collections::BTreeMap;
use std::fmt;

use egui::Key;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};
use serde_json::Value;

use crate::workbench::commands::CommandPlatform;

pub const MAX_SHORTCUT_SEQUENCE_STROKES: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ShortcutProfileError {
    #[error("shortcut sequence must contain at least one stroke")]
    EmptySequence,
    #[error("shortcut sequence exceeds the {MAX_SHORTCUT_SEQUENCE_STROKES}-stroke limit")]
    SequenceTooLong,
    #[error("shortcut binding must target at least one platform")]
    EmptyPlatforms,
    #[error("shortcut binding repeats platform {0}")]
    DuplicatePlatform(&'static str),
    #[error("shortcut binding is structurally invalid: {0}")]
    InvalidBinding(String),
}

/// Stable serde adapter for egui's logical key identity.
///
/// The wire value is the documented key name rather than Rust's debug output
/// or enum layout, so portable profiles do not depend on an implementation
/// detail of the current egui release.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortcutKey(Key);

impl ShortcutKey {
    #[must_use]
    pub const fn new(key: Key) -> Self {
        Self(key)
    }

    #[must_use]
    pub const fn get(self) -> Key {
        self.0
    }

    #[must_use]
    pub fn name(self) -> &'static str {
        self.0.name()
    }
}

impl Serialize for ShortcutKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.name())
    }
}

impl<'de> Deserialize<'de> for ShortcutKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Key::from_name(&name)
            .map(Self)
            .ok_or_else(|| serde::de::Error::custom(format!("unsupported shortcut key '{name}'")))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ShortcutStroke {
    key: ShortcutKey,
    #[serde(default, skip_serializing_if = "is_false")]
    primary: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    alt: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    shift: bool,
}

const fn is_false(value: &bool) -> bool {
    !*value
}

impl ShortcutStroke {
    #[must_use]
    pub const fn new(key: Key, primary: bool, alt: bool, shift: bool) -> Self {
        Self {
            key: ShortcutKey::new(key),
            primary,
            alt,
            shift,
        }
    }

    #[must_use]
    pub const fn key(self) -> Key {
        self.key.get()
    }

    #[must_use]
    pub const fn primary(self) -> bool {
        self.primary
    }

    #[must_use]
    pub const fn alt(self) -> bool {
        self.alt
    }

    #[must_use]
    pub const fn shift(self) -> bool {
        self.shift
    }

    #[must_use]
    pub fn display_label(self) -> String {
        let mut parts = Vec::with_capacity(4);
        if self.primary {
            parts.push("Ctrl");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.shift {
            parts.push("Shift");
        }
        parts.push(self.key.get().symbol_or_name());
        parts.join("+")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ShortcutSequence(Vec<ShortcutStroke>);

impl ShortcutSequence {
    pub fn new(strokes: Vec<ShortcutStroke>) -> Result<Self, ShortcutProfileError> {
        match strokes.len() {
            0 => Err(ShortcutProfileError::EmptySequence),
            1..=MAX_SHORTCUT_SEQUENCE_STROKES => Ok(Self(strokes)),
            _ => Err(ShortcutProfileError::SequenceTooLong),
        }
    }

    #[must_use]
    pub fn single(stroke: ShortcutStroke) -> Self {
        Self(vec![stroke])
    }

    #[must_use]
    pub fn strokes(&self) -> &[ShortcutStroke] {
        &self.0
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        self.0
            .iter()
            .map(|stroke| stroke.display_label())
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub(crate) fn validate(&self) -> Result<(), ShortcutProfileError> {
        match self.0.len() {
            0 => Err(ShortcutProfileError::EmptySequence),
            1..=MAX_SHORTCUT_SEQUENCE_STROKES => Ok(()),
            _ => Err(ShortcutProfileError::SequenceTooLong),
        }
    }

    pub(crate) fn is_prefix_of(&self, other: &Self) -> bool {
        self.0.len() < other.0.len() && other.0.starts_with(&self.0)
    }
}

impl fmt::Display for ShortcutSequence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_label().fmt(formatter)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShortcutBindingSlot {
    Primary,
    Alternate,
}

impl ShortcutBindingSlot {
    pub const ALL: [Self; 2] = [Self::Primary, Self::Alternate];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Primary => "Primary",
            Self::Alternate => "Alternate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileShortcutBinding {
    slot: ShortcutBindingSlot,
    platforms: Vec<CommandPlatform>,
    sequence: ShortcutSequence,
    #[serde(flatten)]
    unknown_fields: BTreeMap<String, Value>,
}

impl ProfileShortcutBinding {
    pub fn new(
        slot: ShortcutBindingSlot,
        mut platforms: Vec<CommandPlatform>,
        sequence: ShortcutSequence,
    ) -> Result<Self, ShortcutProfileError> {
        if platforms.is_empty() {
            return Err(ShortcutProfileError::EmptyPlatforms);
        }
        platforms.sort_unstable();
        if let Some(duplicate) = platforms.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(ShortcutProfileError::DuplicatePlatform(
                duplicate[0].label(),
            ));
        }
        sequence.validate()?;
        Ok(Self {
            slot,
            platforms,
            sequence,
            unknown_fields: BTreeMap::new(),
        })
    }

    #[must_use]
    pub const fn slot(&self) -> ShortcutBindingSlot {
        self.slot
    }

    #[must_use]
    pub fn platforms(&self) -> &[CommandPlatform] {
        &self.platforms
    }

    #[must_use]
    pub const fn sequence(&self) -> &ShortcutSequence {
        &self.sequence
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        self.sequence.display_label()
    }

    pub(crate) fn validate(&self) -> Result<(), ShortcutProfileError> {
        if self.platforms.is_empty() {
            return Err(ShortcutProfileError::EmptyPlatforms);
        }
        let mut platforms = self.platforms.clone();
        platforms.sort_unstable();
        if let Some(duplicate) = platforms.windows(2).find(|pair| pair[0] == pair[1]) {
            return Err(ShortcutProfileError::DuplicatePlatform(
                duplicate[0].label(),
            ));
        }
        self.sequence.validate()
    }

    pub(crate) fn supports(&self, platform: CommandPlatform) -> bool {
        self.platforms.contains(&platform)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CommandShortcutOverride {
    bindings: Vec<ProfileShortcutBinding>,
    #[serde(flatten)]
    unknown_fields: BTreeMap<String, Value>,
}

impl CommandShortcutOverride {
    pub fn new(bindings: Vec<ProfileShortcutBinding>) -> Result<Self, ShortcutProfileError> {
        let value = Self {
            bindings,
            unknown_fields: BTreeMap::new(),
        };
        value.validate()?;
        Ok(value)
    }

    #[must_use]
    pub fn bindings(&self) -> &[ProfileShortcutBinding] {
        &self.bindings
    }

    pub fn bindings_for_slot(
        &self,
        slot: ShortcutBindingSlot,
    ) -> impl Iterator<Item = &ProfileShortcutBinding> {
        self.bindings
            .iter()
            .filter(move |binding| binding.slot == slot)
    }

    pub fn replace_slot(
        &mut self,
        slot: ShortcutBindingSlot,
        replacements: Vec<ProfileShortcutBinding>,
    ) -> Result<(), ShortcutProfileError> {
        if replacements.iter().any(|binding| binding.slot != slot) {
            return Err(ShortcutProfileError::InvalidBinding(format!(
                "{} replacement contains a {} binding",
                slot.label(),
                replacements
                    .iter()
                    .find(|binding| binding.slot != slot)
                    .map_or("different-slot", |binding| binding.slot.label())
            )));
        }
        for binding in &replacements {
            binding.validate()?;
        }
        let mut candidate = self.clone();
        candidate.bindings.retain(|binding| binding.slot != slot);
        candidate.bindings.extend(replacements);
        candidate.validate()?;
        *self = candidate;
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), ShortcutProfileError> {
        for binding in &self.bindings {
            binding.validate()?;
        }
        for left_index in 0..self.bindings.len() {
            for right in &self.bindings[(left_index + 1)..] {
                let left = &self.bindings[left_index];
                if left.slot == right.slot
                    && let Some(platform) = CommandPlatform::ALL
                        .into_iter()
                        .find(|platform| left.supports(*platform) && right.supports(*platform))
                {
                    return Err(ShortcutProfileError::InvalidBinding(format!(
                        "multiple {} bindings target {}",
                        left.slot.label(),
                        platform.label()
                    )));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutBindingSource {
    Default,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedShortcutBinding {
    slot: ShortcutBindingSlot,
    platforms: Vec<CommandPlatform>,
    sequence: ShortcutSequence,
    source: ShortcutBindingSource,
}

impl ResolvedShortcutBinding {
    pub(crate) fn from_profile(binding: &ProfileShortcutBinding) -> Self {
        Self {
            slot: binding.slot,
            platforms: binding.platforms.clone(),
            sequence: binding.sequence.clone(),
            source: ShortcutBindingSource::User,
        }
    }

    pub(crate) fn from_default(
        slot: ShortcutBindingSlot,
        platforms: Vec<CommandPlatform>,
        sequence: ShortcutSequence,
    ) -> Self {
        Self {
            slot,
            platforms,
            sequence,
            source: ShortcutBindingSource::Default,
        }
    }

    #[must_use]
    pub const fn slot(&self) -> ShortcutBindingSlot {
        self.slot
    }

    #[must_use]
    pub fn platforms(&self) -> &[CommandPlatform] {
        &self.platforms
    }

    #[must_use]
    pub const fn sequence(&self) -> &ShortcutSequence {
        &self.sequence
    }

    #[must_use]
    pub const fn source(&self) -> ShortcutBindingSource {
        self.source
    }

    #[must_use]
    pub fn display_label(&self) -> String {
        self.sequence.display_label()
    }

    pub(crate) fn supports(&self, platform: CommandPlatform) -> bool {
        self.platforms.contains(&platform)
    }
}

macro_rules! policy_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $default:ident => $default_label:literal
            $(, $variant:ident => $label:literal)* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum $name {
            $default,
            $($variant),*
        }

        impl Default for $name {
            fn default() -> Self { Self::$default }
        }

        impl $name {
            pub const ALL: &'static [Self] = &[Self::$default, $(Self::$variant),*];

            #[must_use]
            pub const fn label(self) -> &'static str {
                match self {
                    Self::$default => $default_label,
                    $(Self::$variant => $label),*
                }
            }
        }
    };
}

policy_enum! {
    pub enum SingleKeyCanvasPolicy {
        CanvasFocusOnly => "Enabled only while canvas has focus",
        RequireAlt => "Require Alt modifier",
        Disabled => "Disabled"
    }
}

policy_enum! {
    pub enum ChordTimeoutPolicy {
        OnePointFiveSeconds => "1.5 seconds",
        ThreeSeconds => "3 seconds",
        NoTimeout => "No timeout"
    }
}

impl ChordTimeoutPolicy {
    #[must_use]
    pub const fn seconds(self) -> Option<f64> {
        match self {
            Self::OnePointFiveSeconds => Some(1.5),
            Self::ThreeSeconds => Some(3.0),
            Self::NoTimeout => None,
        }
    }
}

policy_enum! {
    pub enum ProtectedShortcutPolicy {
        RequireConflictFreeAlternate => "Require a conflict-free alternate",
        AllowWithConfirmation => "Allow with confirmation"
    }
}

policy_enum! {
    pub enum ContextPrecedencePolicy {
        ModalEditorWorkspaceGlobal => "Modal → editor → workspace → global",
        EditorModalWorkspaceGlobal => "Editor → modal → workspace → global"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoredPolicy<T> {
    effective: T,
    unknown: Option<Value>,
}

impl<T: Default> Default for StoredPolicy<T> {
    fn default() -> Self {
        Self {
            effective: T::default(),
            unknown: None,
        }
    }
}

impl<T: Copy> StoredPolicy<T> {
    const fn get(&self) -> T {
        self.effective
    }

    fn set(&mut self, value: T) {
        self.effective = value;
        self.unknown = None;
    }
}

impl<T: Serialize> Serialize for StoredPolicy<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.unknown {
            Some(raw) => raw.serialize(serializer),
            None => self.effective.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for StoredPolicy<T>
where
    T: Default + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = Value::deserialize(deserializer)?;
        match serde_json::from_value(raw.clone()) {
            Ok(effective) => Ok(Self {
                effective,
                unknown: None,
            }),
            Err(_) => Ok(Self {
                effective: T::default(),
                unknown: Some(raw),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ShortcutPolicies {
    single_key_canvas: StoredPolicy<SingleKeyCanvasPolicy>,
    chord_timeout: StoredPolicy<ChordTimeoutPolicy>,
    protected_shortcuts: StoredPolicy<ProtectedShortcutPolicy>,
    context_precedence: StoredPolicy<ContextPrecedencePolicy>,
    #[serde(flatten)]
    unknown_fields: BTreeMap<String, Value>,
}

impl ShortcutPolicies {
    #[must_use]
    pub const fn single_key_canvas(&self) -> SingleKeyCanvasPolicy {
        self.single_key_canvas.get()
    }

    pub fn set_single_key_canvas(&mut self, value: SingleKeyCanvasPolicy) {
        self.single_key_canvas.set(value);
    }

    #[must_use]
    pub const fn chord_timeout(&self) -> ChordTimeoutPolicy {
        self.chord_timeout.get()
    }

    pub fn set_chord_timeout(&mut self, value: ChordTimeoutPolicy) {
        self.chord_timeout.set(value);
    }

    #[must_use]
    pub const fn protected_shortcuts(&self) -> ProtectedShortcutPolicy {
        self.protected_shortcuts.get()
    }

    pub fn set_protected_shortcuts(&mut self, value: ProtectedShortcutPolicy) {
        self.protected_shortcuts.set(value);
    }

    #[must_use]
    pub const fn context_precedence(&self) -> ContextPrecedencePolicy {
        self.context_precedence.get()
    }

    pub fn set_context_precedence(&mut self, value: ContextPrecedencePolicy) {
        self.context_precedence.set(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_labels_use_product_key_symbols_but_wire_keys_use_stable_names() {
        let labels = [
            (Key::Plus, "Ctrl++", "Plus"),
            (Key::Minus, "Ctrl+\u{2212}", "Minus"),
            (Key::OpenBracket, "Ctrl+[", "OpenBracket"),
            (Key::CloseBracket, "Ctrl+]", "CloseBracket"),
        ];
        for (key, label, wire_name) in labels {
            let stroke = ShortcutStroke::new(key, true, false, false);
            assert_eq!(stroke.display_label(), label);
            assert_eq!(serde_json::to_value(stroke).unwrap()["key"], wire_name);
        }
    }

    #[test]
    fn one_slot_cannot_have_overlapping_platform_coverage() {
        let first = ProfileShortcutBinding::new(
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop, CommandPlatform::Browser],
            ShortcutSequence::single(ShortcutStroke::new(Key::S, true, false, false)),
        )
        .unwrap();
        let second = ProfileShortcutBinding::new(
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Browser],
            ShortcutSequence::single(ShortcutStroke::new(Key::F6, false, false, false)),
        )
        .unwrap();
        assert!(matches!(
            CommandShortcutOverride::new(vec![first, second]),
            Err(ShortcutProfileError::InvalidBinding(message))
                if message.contains("Primary") && message.contains("Browser")
        ));
    }
}

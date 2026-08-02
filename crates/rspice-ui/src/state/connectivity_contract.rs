//! Durable project-owned connectivity policy and named signal-bundle catalog.
//!
//! Schematic geometry owns wires, indexed buses, and taps. This module owns
//! the project-level rules used when those objects are compared across views,
//! plus named bundle declarations whose members bind to exact qualified nets.
//! Nothing in this catalog is inferred from display text.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

pub const CONNECTIVITY_CONTRACT_SCHEMA_VERSION: u32 = 1;
pub const MAX_NAMED_SIGNAL_BUNDLES: usize = 4_096;
pub const MAX_NAMED_BUNDLE_MEMBERS: usize = 16_384;
pub const MAX_NAMED_BUNDLE_CATALOG_MEMBERS: usize = 65_536;
pub const MAX_CONNECTIVITY_ALIAS_GROUPS: usize = 4_096;
pub const MAX_CONNECTIVITY_ALIASES_PER_GROUP: usize = 256;
pub const MAX_CONNECTIVITY_NAME_BYTES: usize = 256;
pub const MAX_QUALIFIED_NET_BYTES: usize = 1_024;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleExpansionPolicy {
    #[default]
    NamedMemberMapping,
    PositionalMapping,
}

impl BundleExpansionPolicy {
    pub const ALL: [Self; 2] = [Self::NamedMemberMapping, Self::PositionalMapping];

    pub const fn label(self) -> &'static str {
        match self {
            Self::NamedMemberMapping => "Named member mapping",
            Self::PositionalMapping => "Positional mapping",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleIndexOrderPolicy {
    #[default]
    PreserveDeclaration,
    MsbFirst,
    LsbFirst,
}

impl BundleIndexOrderPolicy {
    pub const ALL: [Self; 3] = [Self::PreserveDeclaration, Self::MsbFirst, Self::LsbFirst];

    pub const fn label(self) -> &'static str {
        match self {
            Self::PreserveDeclaration => "Preserve declaration",
            Self::MsbFirst => "MSB first",
            Self::LsbFirst => "LSB first",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleWidthMismatchPolicy {
    #[default]
    BlockConnection,
    ExplicitSliceOrExtend,
}

impl BundleWidthMismatchPolicy {
    pub const ALL: [Self; 2] = [Self::BlockConnection, Self::ExplicitSliceOrExtend];

    pub const fn label(self) -> &'static str {
        match self {
            Self::BlockConnection => "Block connection",
            Self::ExplicitSliceOrExtend => "Explicit slice / extend",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalNetPromotionPolicy {
    #[default]
    ExplicitReviewedDeclaration,
    TechnologyDefinedOnly,
}

impl GlobalNetPromotionPolicy {
    pub const ALL: [Self; 2] = [
        Self::ExplicitReviewedDeclaration,
        Self::TechnologyDefinedOnly,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ExplicitReviewedDeclaration => "Explicit reviewed global declaration",
            Self::TechnologyDefinedOnly => "Technology-defined only",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalAliasComparisonPolicy {
    #[default]
    CaseSensitiveExact,
    DialectCompatibility,
}

impl GlobalAliasComparisonPolicy {
    pub const ALL: [Self; 2] = [Self::CaseSensitiveExact, Self::DialectCompatibility];

    pub const fn label(self) -> &'static str {
        match self {
            Self::CaseSensitiveExact => "Case-sensitive exact",
            Self::DialectCompatibility => "Dialect compatibility",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalGlobalShadowingPolicy {
    #[default]
    BlockAndIdentifyScope,
    AllowExplicitLocalNet,
}

impl LocalGlobalShadowingPolicy {
    pub const ALL: [Self; 2] = [Self::BlockAndIdentifyScope, Self::AllowExplicitLocalNet];

    pub const fn label(self) -> &'static str {
        match self {
            Self::BlockAndIdentifyScope => "Block and identify scope",
            Self::AllowExplicitLocalNet => "Allow explicit local net",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleDirection {
    #[default]
    Input,
    Output,
    InOut,
    Mixed,
}

impl BundleDirection {
    pub const ALL: [Self; 4] = [Self::Input, Self::Output, Self::InOut, Self::Mixed];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
            Self::InOut => "inout",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleDiscipline {
    #[default]
    Electrical,
    Logic,
    Mixed,
}

impl BundleDiscipline {
    pub const ALL: [Self; 3] = [Self::Electrical, Self::Logic, Self::Mixed];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Electrical => "electrical",
            Self::Logic => "logic",
            Self::Mixed => "mixed",
        }
    }
}

/// Exact project net identity. `view_key` is the canonical
/// `library/cell/view` workspace key; `net_name` is the extractor identity in
/// that view. The pair remains unambiguous even when several sheets reuse a
/// local name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualifiedNetReference {
    pub view_key: String,
    pub net_name: String,
}

impl QualifiedNetReference {
    pub fn new(view_key: impl Into<String>, net_name: impl Into<String>) -> Self {
        Self {
            view_key: view_key.into(),
            net_name: net_name.into(),
        }
    }

    pub fn display(&self) -> String {
        format!("{}::{}", self.view_key, self.net_name)
    }

    fn validate(&self) -> Result<(), String> {
        validate_view_key(&self.view_key)?;
        validate_net_name(&self.net_name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedSignalBundleMember {
    pub name: String,
    pub target: QualifiedNetReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NamedSignalBundle {
    pub id: u64,
    pub name: String,
    pub direction: BundleDirection,
    pub discipline: BundleDiscipline,
    pub members: Vec<NamedSignalBundleMember>,
}

impl NamedSignalBundle {
    pub fn validate(&self) -> Result<(), String> {
        if self.id == 0 {
            return Err("bundle identity must be nonzero".to_owned());
        }
        validate_name("bundle name", &self.name, MAX_CONNECTIVITY_NAME_BYTES)?;
        if self.members.is_empty() {
            return Err("bundle must contain at least one named member".to_owned());
        }
        if self.members.len() > MAX_NAMED_BUNDLE_MEMBERS {
            return Err(format!(
                "bundle contains {} members; maximum is {MAX_NAMED_BUNDLE_MEMBERS}",
                self.members.len()
            ));
        }
        let mut names = HashSet::with_capacity(self.members.len());
        let mut targets = HashSet::with_capacity(self.members.len());
        for (index, member) in self.members.iter().enumerate() {
            validate_name(
                "bundle member name",
                &member.name,
                MAX_CONNECTIVITY_NAME_BYTES,
            )
            .map_err(|error| format!("member {index}: {error}"))?;
            if !names.insert(member.name.to_ascii_lowercase()) {
                return Err(format!(
                    "bundle member name '{}' is duplicated",
                    member.name
                ));
            }
            member
                .target
                .validate()
                .map_err(|error| format!("member {index}: {error}"))?;
            if !targets.insert(member.target.clone()) {
                return Err(format!(
                    "qualified net '{}' is bound by more than one member",
                    member.target.display()
                ));
            }
        }
        Ok(())
    }
}

/// One exact canonical net name and the spellings an external authority
/// declares equivalent to it. Matching is deliberately case-sensitive:
/// dialects or technology packages that fold case must enumerate those
/// spellings rather than relying on ambient host behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectivityAliasGroup {
    pub canonical_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

impl ConnectivityAliasGroup {
    fn validate(&self) -> Result<(), String> {
        validate_catalog_net_name(&self.canonical_name)?;
        if self.aliases.len() > MAX_CONNECTIVITY_ALIASES_PER_GROUP {
            return Err(format!(
                "alias group '{}' contains {} aliases; maximum is {MAX_CONNECTIVITY_ALIASES_PER_GROUP}",
                self.canonical_name,
                self.aliases.len()
            ));
        }
        let mut names = HashSet::with_capacity(self.aliases.len() + 1);
        names.insert(self.canonical_name.as_str());
        for alias in &self.aliases {
            validate_catalog_net_name(alias)?;
            if !names.insert(alias.as_str()) {
                return Err(format!(
                    "alias group '{}' repeats exact name '{alias}'",
                    self.canonical_name
                ));
            }
        }
        Ok(())
    }

    fn contains(&self, name: &str) -> bool {
        self.canonical_name == name || self.aliases.iter().any(|alias| alias == name)
    }
}

/// Technology-package authority for nets that may be promoted globally
/// without an authored `!` declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TechnologyGlobalNetCatalog {
    /// Stable project-owned package/profile identity shown in review receipts.
    pub authority: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nets: Vec<ConnectivityAliasGroup>,
}

/// Simulator-dialect authority for exact net-name equivalence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DialectAliasCatalog {
    /// Exact simulator dialect/profile identity.
    pub authority: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<ConnectivityAliasGroup>,
}

fn validate_alias_catalog(
    kind: &str,
    authority: &str,
    groups: &[ConnectivityAliasGroup],
) -> Result<(), String> {
    validate_catalog_authority(kind, authority)?;
    if groups.is_empty() {
        return Err(format!(
            "{kind} catalog must contain at least one alias group"
        ));
    }
    if groups.len() > MAX_CONNECTIVITY_ALIAS_GROUPS {
        return Err(format!(
            "{kind} catalog contains {} groups; maximum is {MAX_CONNECTIVITY_ALIAS_GROUPS}",
            groups.len()
        ));
    }
    let mut owners = HashMap::<&str, usize>::new();
    for (index, group) in groups.iter().enumerate() {
        group
            .validate()
            .map_err(|error| format!("{kind} group {index}: {error}"))?;
        for name in std::iter::once(group.canonical_name.as_str())
            .chain(group.aliases.iter().map(String::as_str))
        {
            if let Some(first) = owners.insert(name, index) {
                return Err(format!(
                    "{kind} name '{name}' is owned by both group {first} and group {index}"
                ));
            }
        }
    }
    Ok(())
}

impl TechnologyGlobalNetCatalog {
    pub fn validate(&self) -> Result<(), String> {
        validate_alias_catalog("technology global-net", &self.authority, &self.nets)
    }

    pub fn canonical_name<'a>(&'a self, name: &str) -> Option<&'a str> {
        let name = name.trim_end_matches('!');
        self.nets
            .iter()
            .find(|group| group.contains(name))
            .map(|group| group.canonical_name.as_str())
    }
}

impl DialectAliasCatalog {
    pub fn validate(&self) -> Result<(), String> {
        validate_alias_catalog("dialect alias", &self.authority, &self.groups)
    }

    pub fn equivalent(&self, left: &str, right: &str) -> bool {
        let left = left.trim_end_matches('!');
        let right = right.trim_end_matches('!');
        left == right
            || matches!(
                (self.canonical_name(left), self.canonical_name(right)),
                (Some(left_canonical), Some(right_canonical)) if left_canonical == right_canonical
            )
    }

    pub fn canonical_name<'a>(&'a self, name: &str) -> Option<&'a str> {
        let name = name.trim_end_matches('!');
        self.groups
            .iter()
            .find(|group| group.contains(name))
            .map(|group| group.canonical_name.as_str())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectivityPolicy {
    #[serde(default)]
    pub expansion: BundleExpansionPolicy,
    #[serde(default)]
    pub index_order: BundleIndexOrderPolicy,
    #[serde(default)]
    pub width_mismatch: BundleWidthMismatchPolicy,
    #[serde(default)]
    pub global_promotion: GlobalNetPromotionPolicy,
    #[serde(default)]
    pub alias_comparison: GlobalAliasComparisonPolicy,
    #[serde(default)]
    pub local_shadowing: LocalGlobalShadowingPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectivityContract {
    #[serde(default = "connectivity_contract_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub policy: ConnectivityPolicy,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub named_bundles: Vec<NamedSignalBundle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technology_global_nets: Option<TechnologyGlobalNetCatalog>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dialect_aliases: Option<DialectAliasCatalog>,
    #[serde(default = "first_connectivity_identity")]
    pub next_identity: u64,
}

impl Default for ConnectivityContract {
    fn default() -> Self {
        Self {
            schema_version: CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
            policy: ConnectivityPolicy::default(),
            named_bundles: Vec::new(),
            technology_global_nets: None,
            dialect_aliases: None,
            next_identity: first_connectivity_identity(),
        }
    }
}

impl ConnectivityContract {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != CONNECTIVITY_CONTRACT_SCHEMA_VERSION {
            return Err(format!(
                "unsupported connectivity contract schema {}; expected {}",
                self.schema_version, CONNECTIVITY_CONTRACT_SCHEMA_VERSION
            ));
        }
        if self.named_bundles.len() > MAX_NAMED_SIGNAL_BUNDLES {
            return Err(format!(
                "named bundle catalog contains {} entries; maximum is {MAX_NAMED_SIGNAL_BUNDLES}",
                self.named_bundles.len()
            ));
        }
        if let Some(catalog) = &self.technology_global_nets {
            catalog
                .validate()
                .map_err(|error| format!("technology_global_nets is invalid: {error}"))?;
        }
        if let Some(catalog) = &self.dialect_aliases {
            catalog
                .validate()
                .map_err(|error| format!("dialect_aliases is invalid: {error}"))?;
        }
        if self.policy.global_promotion == GlobalNetPromotionPolicy::TechnologyDefinedOnly
            && self.technology_global_nets.is_none()
        {
            return Err(
                "technology-defined global promotion has no project-owned technology global-net catalog"
                    .to_owned(),
            );
        }
        if self.policy.alias_comparison == GlobalAliasComparisonPolicy::DialectCompatibility
            && self.dialect_aliases.is_none()
        {
            return Err(
                "dialect-compatible alias comparison has no project-owned dialect alias catalog"
                    .to_owned(),
            );
        }
        if self.next_identity == 0 {
            return Err("next connectivity identity must be nonzero".to_owned());
        }
        let mut ids = HashSet::with_capacity(self.named_bundles.len());
        let mut names = HashMap::<String, usize>::with_capacity(self.named_bundles.len());
        let mut highest_id = 0_u64;
        let mut total_members = 0_usize;
        for (index, bundle) in self.named_bundles.iter().enumerate() {
            bundle
                .validate()
                .map_err(|error| format!("named_bundles[{index}] is invalid: {error}"))?;
            if !ids.insert(bundle.id) {
                return Err(format!("named bundle identity {} is duplicated", bundle.id));
            }
            if let Some(first) = names.insert(bundle.name.to_ascii_lowercase(), index) {
                return Err(format!(
                    "named_bundles[{index}] duplicates the case-insensitive name of named_bundles[{first}]"
                ));
            }
            highest_id = highest_id.max(bundle.id);
            total_members = total_members
                .checked_add(bundle.members.len())
                .ok_or_else(|| "named bundle member count overflowed".to_owned())?;
            if total_members > MAX_NAMED_BUNDLE_CATALOG_MEMBERS {
                return Err(format!(
                    "named bundle catalog contains {total_members} members; maximum is {MAX_NAMED_BUNDLE_CATALOG_MEMBERS}"
                ));
            }
        }
        if self.next_identity <= highest_id {
            return Err(format!(
                "next connectivity identity {} is not greater than allocated identity {highest_id}",
                self.next_identity
            ));
        }
        Ok(())
    }

    pub fn aliases_match(
        &self,
        left: &str,
        right: &str,
        comparison: GlobalAliasComparisonPolicy,
    ) -> bool {
        let left = left.trim_end_matches('!');
        let right = right.trim_end_matches('!');
        match comparison {
            GlobalAliasComparisonPolicy::CaseSensitiveExact => left == right,
            GlobalAliasComparisonPolicy::DialectCompatibility => self
                .dialect_aliases
                .as_ref()
                .is_some_and(|catalog| catalog.equivalent(left, right)),
        }
    }

    pub fn technology_global_canonical_name<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.technology_global_nets
            .as_ref()
            .and_then(|catalog| catalog.canonical_name(name))
    }

    pub fn dialect_canonical_name<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.dialect_aliases
            .as_ref()
            .and_then(|catalog| catalog.canonical_name(name))
    }

    pub fn insert_named_bundle(
        &mut self,
        name: impl Into<String>,
        direction: BundleDirection,
        discipline: BundleDiscipline,
        members: Vec<NamedSignalBundleMember>,
    ) -> Result<u64, String> {
        self.validate()?;
        if self.named_bundles.len() >= MAX_NAMED_SIGNAL_BUNDLES {
            return Err(format!(
                "named bundle catalog is full ({MAX_NAMED_SIGNAL_BUNDLES} entries)"
            ));
        }
        let id = self.next_identity;
        let next_identity = id
            .checked_add(1)
            .ok_or_else(|| "connectivity identity space is exhausted".to_owned())?;
        let candidate = NamedSignalBundle {
            id,
            name: name.into(),
            direction,
            discipline,
            members,
        };
        candidate.validate()?;
        if self
            .named_bundles
            .iter()
            .any(|bundle| bundle.name.eq_ignore_ascii_case(&candidate.name))
        {
            return Err(format!(
                "named signal bundle '{}' already exists",
                candidate.name
            ));
        }
        self.named_bundles.push(candidate);
        self.next_identity = next_identity;
        self.validate()?;
        Ok(id)
    }
}

const fn connectivity_contract_schema_version() -> u32 {
    CONNECTIVITY_CONTRACT_SCHEMA_VERSION
}

const fn first_connectivity_identity() -> u64 {
    1
}

fn validate_name(label: &str, value: &str, max_bytes: usize) -> Result<(), String> {
    if value.trim().is_empty() || value != value.trim() {
        return Err(format!("{label} must be nonempty and trimmed"));
    }
    if value.len() > max_bytes {
        return Err(format!("{label} exceeds {max_bytes} bytes"));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("{label} contains a control character"));
    }
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return Err(format!("{label} is required"));
    };
    if !first.is_ascii_alphabetic() && first != '_' {
        return Err(format!(
            "{label} must begin with an ASCII letter or underscore"
        ));
    }
    if chars.any(|character| !character.is_ascii_alphanumeric() && character != '_') {
        return Err(format!("{label} contains unsupported characters"));
    }
    Ok(())
}

fn validate_catalog_authority(kind: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value != value.trim() {
        return Err(format!("{kind} authority must be nonempty and trimmed"));
    }
    if value.len() > MAX_QUALIFIED_NET_BYTES {
        return Err(format!(
            "{kind} authority exceeds {MAX_QUALIFIED_NET_BYTES} bytes"
        ));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("{kind} authority contains a control character"));
    }
    Ok(())
}

fn validate_catalog_net_name(value: &str) -> Result<(), String> {
    validate_net_name(value)?;
    if value.ends_with('!') {
        return Err(
            "catalog net names must omit the authored global-declaration '!' suffix".to_owned(),
        );
    }
    Ok(())
}

/// Qualified references must admit every exact name that a retained schematic
/// can own, including SPICE ground `0`, punctuation, and relaxed Unicode. The
/// owning schematic's document policy is applied again when the reference is
/// resolved; this structural check only rejects ambiguous/unsafe serialized
/// text.
fn validate_net_name(value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value != value.trim() {
        return Err("net name must be nonempty and trimmed".to_owned());
    }
    if value.len() > MAX_CONNECTIVITY_NAME_BYTES {
        return Err(format!(
            "net name exceeds {MAX_CONNECTIVITY_NAME_BYTES} bytes"
        ));
    }
    if value.chars().any(char::is_whitespace) || value.chars().any(char::is_control) {
        return Err("net name contains whitespace or a control character".to_owned());
    }
    let opens = value
        .chars()
        .filter(|character| matches!(character, '[' | '<'))
        .count();
    let closes = value
        .chars()
        .filter(|character| matches!(character, ']' | '>'))
        .count();
    if opens != closes {
        return Err("net name contains unbalanced bus delimiters".to_owned());
    }
    Ok(())
}

fn validate_view_key(value: &str) -> Result<(), String> {
    if value.trim().is_empty()
        || value != value.trim()
        || value.len() > MAX_QUALIFIED_NET_BYTES
        || value.chars().any(char::is_control)
    {
        return Err(
            "qualified view key is empty, untrimmed, oversized, or contains controls".to_owned(),
        );
    }
    let parts = value.split('/').collect::<Vec<_>>();
    if parts.len() != 3 || parts.iter().any(|part| part.is_empty()) {
        return Err("qualified view key must be library/cell/view".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(name: &str, net: &str) -> NamedSignalBundleMember {
        NamedSignalBundleMember {
            name: name.to_owned(),
            target: QualifiedNetReference::new("user/top/schematic", net),
        }
    }

    #[test]
    fn older_projects_receive_a_strict_bounded_default_contract() {
        let restored: ConnectivityContract = serde_json::from_str("{}").unwrap();
        restored.validate().unwrap();
        assert_eq!(
            restored.policy.width_mismatch,
            BundleWidthMismatchPolicy::BlockConnection
        );
    }

    #[test]
    fn insertion_is_atomic_and_rejects_duplicate_names() {
        let mut contract = ConnectivityContract::default();
        let first = contract
            .insert_named_bundle(
                "SENSE_DIFF",
                BundleDirection::Input,
                BundleDiscipline::Electrical,
                vec![member("p", "SENSE_P"), member("n", "SENSE_N")],
            )
            .unwrap();
        assert_eq!(first, 1);
        let before = contract.clone();
        assert!(
            contract
                .insert_named_bundle(
                    "sense_diff",
                    BundleDirection::Input,
                    BundleDiscipline::Electrical,
                    vec![member("p", "OTHER")]
                )
                .is_err()
        );
        assert_eq!(contract, before);
    }

    #[test]
    fn members_require_distinct_names_and_exact_qualified_targets() {
        let duplicate_name = NamedSignalBundle {
            id: 1,
            name: "SPI".to_owned(),
            direction: BundleDirection::Mixed,
            discipline: BundleDiscipline::Logic,
            members: vec![member("clk", "CLK"), member("CLK", "MISO")],
        };
        assert!(duplicate_name.validate().is_err());

        let duplicate_target = NamedSignalBundle {
            id: 1,
            name: "SPI".to_owned(),
            direction: BundleDirection::Mixed,
            discipline: BundleDiscipline::Logic,
            members: vec![member("clk", "CLK"), member("miso", "CLK")],
        };
        assert!(duplicate_target.validate().is_err());
    }

    #[test]
    fn qualified_targets_admit_retained_spice_and_relaxed_net_names() {
        for net_name in ["0", "afe.out:3", "DATA[7]", "rail-$sense", "unicode_δ"] {
            QualifiedNetReference::new("user/top/schematic", net_name)
                .validate()
                .unwrap();
        }
        assert!(
            QualifiedNetReference::new("user/top/schematic", "two nodes")
                .validate()
                .is_err()
        );
        assert!(
            QualifiedNetReference::new("user/top/schematic", "DATA[7")
                .validate()
                .is_err()
        );
    }

    #[test]
    fn deserialize_rejects_unknown_fields_and_invalid_identity_progression() {
        let unknown = serde_json::json!({
            "schema_version": 1,
            "policy": {},
            "next_identity": 1,
            "unexpected": true
        });
        assert!(serde_json::from_value::<ConnectivityContract>(unknown).is_err());

        let mut contract = ConnectivityContract::default();
        contract
            .insert_named_bundle(
                "ONE",
                BundleDirection::Input,
                BundleDiscipline::Logic,
                vec![member("bit", "BIT")],
            )
            .unwrap();
        contract.next_identity = 1;
        assert!(contract.validate().is_err());
    }

    #[test]
    fn optional_authority_catalogs_are_exact_bounded_and_backward_compatible() {
        let mut contract = ConnectivityContract::default();
        contract.technology_global_nets = Some(TechnologyGlobalNetCatalog {
            authority: "demo-pdk@1.0".to_owned(),
            nets: vec![ConnectivityAliasGroup {
                canonical_name: "VDD".to_owned(),
                aliases: vec!["vdd".to_owned(), "VCC".to_owned()],
            }],
        });
        contract.dialect_aliases = Some(DialectAliasCatalog {
            authority: "commercial-spice-2026".to_owned(),
            groups: vec![ConnectivityAliasGroup {
                canonical_name: "0".to_owned(),
                aliases: vec!["GND".to_owned()],
            }],
        });
        contract.validate().unwrap();
        assert_eq!(
            contract.technology_global_canonical_name("VCC!"),
            Some("VDD")
        );
        assert!(contract.aliases_match(
            "0",
            "GND!",
            GlobalAliasComparisonPolicy::DialectCompatibility
        ));
        assert!(!contract.aliases_match(
            "0",
            "gnd",
            GlobalAliasComparisonPolicy::DialectCompatibility
        ));

        let restored: ConnectivityContract =
            serde_json::from_str(&serde_json::to_string(&contract).unwrap()).unwrap();
        assert_eq!(restored, contract);
    }

    #[test]
    fn authority_catalogs_reject_ambiguous_alias_ownership() {
        let catalog = DialectAliasCatalog {
            authority: "ambiguous".to_owned(),
            groups: vec![
                ConnectivityAliasGroup {
                    canonical_name: "VDD".to_owned(),
                    aliases: vec!["SUPPLY".to_owned()],
                },
                ConnectivityAliasGroup {
                    canonical_name: "VCC".to_owned(),
                    aliases: vec!["SUPPLY".to_owned()],
                },
            ],
        };
        assert!(catalog.validate().is_err());
    }
}

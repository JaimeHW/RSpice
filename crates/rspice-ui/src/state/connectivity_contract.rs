//! Durable project-owned connectivity policy and net-alias authorities.
//!
//! Schematic geometry owns wires, indexed buses, and taps. This module owns
//! the project-level rules used when those objects are compared across views,
//! plus the technology and dialect catalogs that declare which exact net names
//! are equivalent. Nothing in this catalog is inferred from display text.

use std::collections::{HashMap, HashSet};

use serde::de::IgnoredAny;
use serde::{Deserialize, Deserializer, Serialize};

pub const CONNECTIVITY_CONTRACT_SCHEMA_VERSION: u32 = 2;

/// The schema that persisted a named signal-bundle registry beside the policy.
/// Its bundle rows, its identity allocator, and the three policy fields that
/// only steered bundle expansion are dropped while the contract is being
/// deserialized; nothing downstream ever sees them, and no later save rewrites
/// them.
const NAMED_BUNDLE_REGISTRY_SCHEMA_VERSION: u32 = 1;

pub const MAX_CONNECTIVITY_ALIAS_GROUPS: usize = 4_096;
pub const MAX_CONNECTIVITY_ALIASES_PER_GROUP: usize = 256;
pub const MAX_CONNECTIVITY_NAME_BYTES: usize = 256;
pub const MAX_CONNECTIVITY_AUTHORITY_BYTES: usize = 1_024;

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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectivityPolicy {
    pub width_mismatch: BundleWidthMismatchPolicy,
    pub global_promotion: GlobalNetPromotionPolicy,
    pub alias_comparison: GlobalAliasComparisonPolicy,
}

/// The policy as the retiring schema spelled it. The three settings that only
/// steered bundle expansion are accepted and discarded; every other unknown key
/// is still fatal.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConnectivityPolicyWire {
    #[serde(default)]
    width_mismatch: BundleWidthMismatchPolicy,
    #[serde(default)]
    global_promotion: GlobalNetPromotionPolicy,
    #[serde(default)]
    alias_comparison: GlobalAliasComparisonPolicy,
    #[serde(default)]
    expansion: Option<IgnoredAny>,
    #[serde(default)]
    index_order: Option<IgnoredAny>,
    #[serde(default)]
    local_shadowing: Option<IgnoredAny>,
}

impl ConnectivityPolicyWire {
    /// Whether the document spelled any of the retired bundle-expansion
    /// settings, which only the retiring schema was allowed to write.
    const fn carries_retired_settings(&self) -> bool {
        self.expansion.is_some() || self.index_order.is_some() || self.local_shadowing.is_some()
    }
}

impl From<ConnectivityPolicyWire> for ConnectivityPolicy {
    fn from(wire: ConnectivityPolicyWire) -> Self {
        Self {
            width_mismatch: wire.width_mismatch,
            global_promotion: wire.global_promotion,
            alias_comparison: wire.alias_comparison,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConnectivityContract {
    pub schema_version: u32,
    pub policy: ConnectivityPolicy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technology_global_nets: Option<TechnologyGlobalNetCatalog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect_aliases: Option<DialectAliasCatalog>,
}

/// The contract as the retiring schema spelled it. A document that predates the
/// schema tag is that same retiring schema, so an absent tag reads as it rather
/// than as today's.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConnectivityContractWire {
    #[serde(default = "named_bundle_registry_schema_version")]
    schema_version: u32,
    #[serde(default)]
    policy: ConnectivityPolicyWire,
    #[serde(default)]
    technology_global_nets: Option<TechnologyGlobalNetCatalog>,
    #[serde(default)]
    dialect_aliases: Option<DialectAliasCatalog>,
    #[serde(default)]
    named_bundles: Option<IgnoredAny>,
    #[serde(default)]
    next_identity: Option<IgnoredAny>,
}

/// The retired named-bundle registry loads and is dropped, so a project saved
/// before it was deleted still opens. It is tolerated only at the schema that
/// wrote it: at any later tag those names are as unknown as any other, and a
/// contract this crate writes never spells them again.
impl<'de> Deserialize<'de> for ConnectivityContract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ConnectivityContractWire::deserialize(deserializer)?;
        let retiring = wire.schema_version == NAMED_BUNDLE_REGISTRY_SCHEMA_VERSION;
        if !retiring
            && (wire.named_bundles.is_some()
                || wire.next_identity.is_some()
                || wire.policy.carries_retired_settings())
        {
            return Err(serde::de::Error::custom(format!(
                "connectivity contract schema {} must not spell the retired named-bundle registry",
                wire.schema_version
            )));
        }
        Ok(Self {
            schema_version: if retiring {
                CONNECTIVITY_CONTRACT_SCHEMA_VERSION
            } else {
                wire.schema_version
            },
            policy: wire.policy.into(),
            technology_global_nets: wire.technology_global_nets,
            dialect_aliases: wire.dialect_aliases,
        })
    }
}

impl Default for ConnectivityContract {
    fn default() -> Self {
        Self {
            schema_version: CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
            policy: ConnectivityPolicy::default(),
            technology_global_nets: None,
            dialect_aliases: None,
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
}

const fn named_bundle_registry_schema_version() -> u32 {
    NAMED_BUNDLE_REGISTRY_SCHEMA_VERSION
}

fn validate_catalog_authority(kind: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() || value != value.trim() {
        return Err(format!("{kind} authority must be nonempty and trimmed"));
    }
    if value.len() > MAX_CONNECTIVITY_AUTHORITY_BYTES {
        return Err(format!(
            "{kind} authority exceeds {MAX_CONNECTIVITY_AUTHORITY_BYTES} bytes"
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

/// A catalog name must admit every exact name that a retained schematic can
/// own, including SPICE ground `0`, punctuation, and relaxed Unicode. The
/// owning schematic's document policy is applied again when a name is
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn older_projects_receive_a_strict_bounded_default_contract() {
        let restored: ConnectivityContract = serde_json::from_str("{}").unwrap();
        restored.validate().unwrap();
        assert_eq!(
            restored.schema_version,
            CONNECTIVITY_CONTRACT_SCHEMA_VERSION
        );
        assert_eq!(
            restored.policy.width_mismatch,
            BundleWidthMismatchPolicy::BlockConnection
        );
    }

    #[test]
    fn catalog_names_admit_retained_spice_and_relaxed_net_names() {
        for canonical_name in ["0", "afe.out:3", "DATA[7]", "rail-$sense", "unicode_δ"] {
            validate_catalog_net_name(canonical_name).unwrap();
        }
        assert!(validate_catalog_net_name("two nodes").is_err());
        assert!(validate_catalog_net_name("DATA[7").is_err());
        assert!(validate_catalog_net_name("VDD!").is_err());
    }

    #[test]
    fn legacy_contract_with_named_bundles_loads_and_drops_them() {
        let legacy = serde_json::json!({
            "schema_version": NAMED_BUNDLE_REGISTRY_SCHEMA_VERSION,
            "policy": {
                "expansion": "named_member_mapping",
                "index_order": "msb_first",
                "width_mismatch": "explicit_slice_or_extend",
                "global_promotion": "explicit_reviewed_declaration",
                "alias_comparison": "case_sensitive_exact",
                "local_shadowing": "allow_explicit_local_net"
            },
            "named_bundles": [
                {
                    "id": 1,
                    "name": "SENSE_DIFF",
                    "direction": "input",
                    "discipline": "electrical",
                    "members": [{
                        "name": "p",
                        "target": {
                            "view_key": "user/top/schematic",
                            "net_name": "SENSE_P"
                        }
                    }]
                },
                {
                    "id": 2,
                    "name": "SPI",
                    "direction": "mixed",
                    "discipline": "logic",
                    "members": [{
                        "name": "clk",
                        "target": {
                            "view_key": "user/top/schematic",
                            "net_name": "CLK"
                        }
                    }]
                }
            ],
            "next_identity": 3
        });

        let restored: ConnectivityContract = serde_json::from_value(legacy).unwrap();
        restored.validate().unwrap();
        assert_eq!(
            restored.schema_version,
            CONNECTIVITY_CONTRACT_SCHEMA_VERSION
        );
        assert_eq!(
            restored.policy.width_mismatch,
            BundleWidthMismatchPolicy::ExplicitSliceOrExtend
        );

        let reserialized = serde_json::to_string(&restored).unwrap();
        for retired in [
            "named_bundles",
            "next_identity",
            "expansion",
            "index_order",
            "local_shadowing",
        ] {
            assert!(
                !reserialized.contains(retired),
                "re-serialized contract still writes '{retired}': {reserialized}"
            );
        }
    }

    #[test]
    fn unknown_key_is_still_rejected() {
        let unknown_contract_key = serde_json::json!({
            "schema_version": CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
            "policy": {},
            "unexpected": true
        });
        assert!(serde_json::from_value::<ConnectivityContract>(unknown_contract_key).is_err());

        let unknown_policy_key = serde_json::json!({
            "schema_version": CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
            "policy": { "unexpected": "positional_mapping" }
        });
        assert!(serde_json::from_value::<ConnectivityContract>(unknown_policy_key).is_err());
    }

    #[test]
    fn a_retired_key_is_unknown_once_the_schema_moves_past_it() {
        for retired in [
            serde_json::json!({
                "schema_version": CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
                "named_bundles": []
            }),
            serde_json::json!({
                "schema_version": CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
                "next_identity": 1
            }),
            serde_json::json!({
                "schema_version": CONNECTIVITY_CONTRACT_SCHEMA_VERSION,
                "policy": { "expansion": "positional_mapping" }
            }),
        ] {
            assert!(
                serde_json::from_value::<ConnectivityContract>(retired.clone()).is_err(),
                "{retired} loaded at the current schema"
            );
        }
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

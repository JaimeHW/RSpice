use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const TOPOLOGY_FINGERPRINT_DOMAIN: &[u8] = b"rspice.execution.topology-fingerprint";
const TOPOLOGY_FINGERPRINT_VERSION: u32 = 2;

/// Complete structural identity of one elaborated component.
///
/// A component cannot be used for topology fingerprinting without identifying
/// the model implementation and its matrix sparsity. Numeric parameter values
/// remain intentionally absent: only changes that can alter the solver
/// structure belong here.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TopologyComponent {
    kind: String,
    name: String,
    model_identity: String,
    terminals: Vec<String>,
    stamp_sparsity: Vec<(usize, usize)>,
}

impl TopologyComponent {
    pub fn new(
        kind: impl Into<String>,
        name: impl Into<String>,
        model_identity: impl Into<String>,
        terminals: impl IntoIterator<Item = impl Into<String>>,
        stamp_sparsity: impl IntoIterator<Item = (usize, usize)>,
    ) -> Result<Self, TopologyFingerprintError> {
        let kind = normalize_identifier(kind.into());
        let name = normalize_identifier(name.into());
        let model_identity = normalize_identifier(model_identity.into());
        let terminals = terminals
            .into_iter()
            .map(|terminal| normalize_identifier(terminal.into()))
            .collect::<Vec<_>>();
        if kind.is_empty() {
            return Err(TopologyFingerprintError::EmptyComponentKind);
        }
        if name.is_empty() {
            return Err(TopologyFingerprintError::EmptyComponentName);
        }
        if model_identity.is_empty() {
            return Err(TopologyFingerprintError::EmptyModelIdentity { component: name });
        }
        if terminals.iter().any(String::is_empty) {
            return Err(TopologyFingerprintError::EmptyTerminal { component: name });
        }

        // Sparsity is a mathematical set. Canonicalizing it here makes the
        // fingerprint independent of device-stamping traversal order.
        let stamp_sparsity = stamp_sparsity
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Ok(Self {
            kind,
            name,
            model_identity,
            terminals,
            stamp_sparsity,
        })
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn model_identity(&self) -> &str {
        &self.model_identity
    }

    pub fn terminals(&self) -> &[String] {
        &self.terminals
    }

    pub fn stamp_sparsity(&self) -> &[(usize, usize)] {
        &self.stamp_sparsity
    }

    fn identity(&self) -> (&str, &str) {
        (&self.kind, &self.name)
    }
}

/// Collision-resistant identity of a fully elaborated solver topology.
///
/// Numeric values and non-structural model parameters are intentionally
/// absent. The complete node namespace, component/model identity, terminal
/// order, ordered unknown and state layouts, and component stamp sparsity are
/// all required. This prevents a partial connectivity description from being
/// presented as a production-safe topology fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TopologyFingerprint([u8; 32]);

impl TopologyFingerprint {
    pub fn from_materialized(
        nodes: impl IntoIterator<Item = impl AsRef<str>>,
        unknown_layout: impl IntoIterator<Item = impl AsRef<str>>,
        state_layout: impl IntoIterator<Item = impl AsRef<str>>,
        components: impl IntoIterator<Item = TopologyComponent>,
    ) -> Result<Self, TopologyFingerprintError> {
        let descriptor = StructuralTopology::new(nodes, unknown_layout, state_layout, components)?;
        Ok(hash_structural_descriptor(
            &descriptor,
            TOPOLOGY_FINGERPRINT_DOMAIN,
            TOPOLOGY_FINGERPRINT_VERSION,
        ))
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }

    /// Rebuild a fingerprint that this crate previously emitted.
    ///
    /// This is deliberately crate-private: a fingerprint may only enter the
    /// public API from a real elaborated topology or from decoding a document
    /// that a run produced. There is no public path from arbitrary bytes to a
    /// structural identity.
    pub(crate) const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

impl fmt::Display for TopologyFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructuralTopology {
    nodes: Vec<String>,
    unknown_layout: Vec<String>,
    state_layout: Vec<String>,
    components: Vec<TopologyComponent>,
}

impl StructuralTopology {
    fn new(
        nodes: impl IntoIterator<Item = impl AsRef<str>>,
        unknown_layout: impl IntoIterator<Item = impl AsRef<str>>,
        state_layout: impl IntoIterator<Item = impl AsRef<str>>,
        components: impl IntoIterator<Item = TopologyComponent>,
    ) -> Result<Self, TopologyFingerprintError> {
        let mut node_set = BTreeSet::new();
        for node in nodes {
            let node = normalize_identifier(node.as_ref());
            if node.is_empty() {
                return Err(TopologyFingerprintError::EmptyNode);
            }
            if !node_set.insert(node.clone()) {
                return Err(TopologyFingerprintError::DuplicateNode { node });
            }
        }

        let unknown_layout = normalize_layout(unknown_layout, LayoutKind::Unknown)?;
        let state_layout = normalize_layout(state_layout, LayoutKind::State)?;
        let unknown_count = unknown_layout.len();

        let mut components_by_identity = BTreeMap::new();
        for component in components {
            for terminal in &component.terminals {
                if !node_set.contains(terminal) {
                    return Err(TopologyFingerprintError::UnknownTerminal {
                        component: component.name.clone(),
                        terminal: terminal.clone(),
                    });
                }
            }
            for &(row, column) in &component.stamp_sparsity {
                if row >= unknown_count || column >= unknown_count {
                    return Err(TopologyFingerprintError::StampOutsideUnknownLayout {
                        component: component.name.clone(),
                        row,
                        column,
                        unknown_count,
                    });
                }
            }

            let identity = (
                component.identity().0.to_string(),
                component.identity().1.to_string(),
            );
            if components_by_identity.insert(identity, component).is_some() {
                return Err(TopologyFingerprintError::DuplicateComponent);
            }
        }

        Ok(Self {
            nodes: node_set.into_iter().collect(),
            unknown_layout,
            state_layout,
            components: components_by_identity.into_values().collect(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum LayoutKind {
    Unknown,
    State,
}

fn normalize_layout(
    values: impl IntoIterator<Item = impl AsRef<str>>,
    kind: LayoutKind,
) -> Result<Vec<String>, TopologyFingerprintError> {
    let mut layout = Vec::new();
    let mut identities = BTreeSet::new();
    for value in values {
        let value = normalize_identifier(value.as_ref());
        if value.is_empty() {
            return Err(match kind {
                LayoutKind::Unknown => TopologyFingerprintError::EmptyUnknown,
                LayoutKind::State => TopologyFingerprintError::EmptyState,
            });
        }
        if !identities.insert(value.clone()) {
            return Err(match kind {
                LayoutKind::Unknown => {
                    TopologyFingerprintError::DuplicateUnknown { unknown: value }
                }
                LayoutKind::State => TopologyFingerprintError::DuplicateState { state: value },
            });
        }
        layout.push(value);
    }
    Ok(layout)
}

fn hash_structural_descriptor(
    descriptor: &StructuralTopology,
    domain: &[u8],
    version: u32,
) -> TopologyFingerprint {
    let mut bytes = Vec::new();
    append_field(&mut bytes, domain);
    bytes.extend_from_slice(&version.to_le_bytes());

    append_field(&mut bytes, b"nodes");
    bytes.extend_from_slice(&(descriptor.nodes.len() as u64).to_le_bytes());
    for node in &descriptor.nodes {
        append_field(&mut bytes, node.as_bytes());
    }

    append_layout(&mut bytes, b"unknown-layout", &descriptor.unknown_layout);
    append_layout(&mut bytes, b"state-layout", &descriptor.state_layout);

    append_field(&mut bytes, b"components");
    bytes.extend_from_slice(&(descriptor.components.len() as u64).to_le_bytes());
    for component in &descriptor.components {
        append_field(&mut bytes, b"component");
        append_field(&mut bytes, component.kind.as_bytes());
        append_field(&mut bytes, component.name.as_bytes());
        append_field(&mut bytes, component.model_identity.as_bytes());
        bytes.extend_from_slice(&(component.terminals.len() as u64).to_le_bytes());
        for terminal in &component.terminals {
            append_field(&mut bytes, terminal.as_bytes());
        }
        bytes.extend_from_slice(&(component.stamp_sparsity.len() as u64).to_le_bytes());
        for &(row, column) in &component.stamp_sparsity {
            bytes.extend_from_slice(&(row as u64).to_le_bytes());
            bytes.extend_from_slice(&(column as u64).to_le_bytes());
        }
    }
    TopologyFingerprint(*blake3::hash(&bytes).as_bytes())
}

fn append_layout(bytes: &mut Vec<u8>, tag: &[u8], layout: &[String]) {
    append_field(bytes, tag);
    bytes.extend_from_slice(&(layout.len() as u64).to_le_bytes());
    for value in layout {
        append_field(bytes, value.as_bytes());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TopologyFingerprintError {
    EmptyNode,
    DuplicateNode {
        node: String,
    },
    EmptyComponentKind,
    EmptyComponentName,
    EmptyModelIdentity {
        component: String,
    },
    EmptyTerminal {
        component: String,
    },
    UnknownTerminal {
        component: String,
        terminal: String,
    },
    EmptyUnknown,
    DuplicateUnknown {
        unknown: String,
    },
    EmptyState,
    DuplicateState {
        state: String,
    },
    StampOutsideUnknownLayout {
        component: String,
        row: usize,
        column: usize,
        unknown_count: usize,
    },
    DuplicateComponent,
}

impl fmt::Display for TopologyFingerprintError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNode => formatter.write_str("materialized topology contains an empty node"),
            Self::DuplicateNode { node } => {
                write!(
                    formatter,
                    "materialized topology contains duplicate node '{node}'"
                )
            }
            Self::EmptyComponentKind => formatter.write_str("topology component kind is empty"),
            Self::EmptyComponentName => formatter.write_str("topology component name is empty"),
            Self::EmptyModelIdentity { component } => {
                write!(
                    formatter,
                    "topology component '{component}' has no model identity"
                )
            }
            Self::EmptyTerminal { component } => {
                write!(
                    formatter,
                    "topology component '{component}' has an empty terminal"
                )
            }
            Self::UnknownTerminal {
                component,
                terminal,
            } => write!(
                formatter,
                "topology component '{component}' references unknown terminal node '{terminal}'"
            ),
            Self::EmptyUnknown => {
                formatter.write_str("materialized topology contains an empty unknown-layout entry")
            }
            Self::DuplicateUnknown { unknown } => write!(
                formatter,
                "materialized topology contains duplicate unknown-layout entry '{unknown}'"
            ),
            Self::EmptyState => {
                formatter.write_str("materialized topology contains an empty state-layout entry")
            }
            Self::DuplicateState { state } => write!(
                formatter,
                "materialized topology contains duplicate state-layout entry '{state}'"
            ),
            Self::StampOutsideUnknownLayout {
                component,
                row,
                column,
                unknown_count,
            } => write!(
                formatter,
                "topology component '{component}' stamp coordinate ({row}, {column}) is outside the {unknown_count}-entry unknown layout"
            ),
            Self::DuplicateComponent => {
                formatter.write_str("materialized topology contains a duplicate component")
            }
        }
    }
}

impl std::error::Error for TopologyFingerprintError {}

fn normalize_identifier(value: impl AsRef<str>) -> String {
    value.as_ref().trim().to_ascii_lowercase()
}

fn append_field(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component(
        model_identity: &str,
        terminals: [&str; 2],
        stamp_sparsity: &[(usize, usize)],
    ) -> TopologyComponent {
        TopologyComponent::new(
            "R",
            "R1",
            model_identity,
            terminals,
            stamp_sparsity.iter().copied(),
        )
        .expect("component")
    }

    fn fingerprint(
        unknown_layout: &[&str],
        state_layout: &[&str],
        component: TopologyComponent,
    ) -> TopologyFingerprint {
        TopologyFingerprint::from_materialized(
            ["0", "in", "out"],
            unknown_layout.iter().copied(),
            state_layout.iter().copied(),
            [component],
        )
        .expect("topology")
    }

    #[test]
    fn fingerprint_is_order_and_case_independent_but_connectivity_sensitive() {
        let first = TopologyFingerprint::from_materialized(
            ["0", "in", "OUT"],
            ["v(in)", "v(out)"],
            ["r1:history"],
            [
                TopologyComponent::new(
                    "R",
                    "R1",
                    "builtin:resistor:v1",
                    ["in", "out"],
                    [(0, 0), (0, 1), (1, 0), (1, 1)],
                )
                .expect("R1"),
                TopologyComponent::new(
                    "V",
                    "V1",
                    "builtin:voltage-source:v1",
                    ["in", "0"],
                    [(0, 0)],
                )
                .expect("V1"),
            ],
        )
        .expect("first topology");
        let reordered = TopologyFingerprint::from_materialized(
            ["out", "IN", "0"],
            ["V(IN)", "V(OUT)"],
            ["R1:HISTORY"],
            [
                TopologyComponent::new(
                    "v",
                    "v1",
                    "BUILTIN:VOLTAGE-SOURCE:V1",
                    ["IN", "0"],
                    [(0, 0)],
                )
                .expect("V1"),
                TopologyComponent::new(
                    "r",
                    "r1",
                    "BUILTIN:RESISTOR:V1",
                    ["IN", "OUT"],
                    [(1, 1), (0, 1), (1, 0), (0, 0)],
                )
                .expect("R1"),
            ],
        )
        .expect("reordered topology");
        assert_eq!(first, reordered);

        let changed = TopologyFingerprint::from_materialized(
            ["0", "in", "out", "extra"],
            ["v(in)", "v(out)"],
            ["r1:history"],
            [
                TopologyComponent::new(
                    "r",
                    "r1",
                    "builtin:resistor:v1",
                    ["in", "extra"],
                    [(0, 0), (0, 1), (1, 0), (1, 1)],
                )
                .expect("R1"),
                TopologyComponent::new(
                    "v",
                    "v1",
                    "builtin:voltage-source:v1",
                    ["in", "0"],
                    [(0, 0)],
                )
                .expect("V1"),
            ],
        )
        .expect("changed topology");
        assert_ne!(first, changed);
    }

    #[test]
    fn model_layout_state_and_stamp_changes_have_distinct_fingerprints() {
        let base_component = component("builtin:resistor:v1", ["in", "out"], &[(0, 0), (1, 1)]);
        let base = fingerprint(
            &["v(in)", "v(out)"],
            &["r1:history", "r1:charge"],
            base_component,
        );
        let changed_model = fingerprint(
            &["v(in)", "v(out)"],
            &["r1:history", "r1:charge"],
            component("plugin:resistor:v2", ["in", "out"], &[(0, 0), (1, 1)]),
        );
        let changed_unknown_layout = fingerprint(
            &["v(out)", "v(in)"],
            &["r1:history", "r1:charge"],
            component("builtin:resistor:v1", ["in", "out"], &[(0, 0), (1, 1)]),
        );
        let changed_state_layout = fingerprint(
            &["v(in)", "v(out)"],
            &["r1:charge", "r1:history"],
            component("builtin:resistor:v1", ["in", "out"], &[(0, 0), (1, 1)]),
        );
        let changed_stamp = fingerprint(
            &["v(in)", "v(out)"],
            &["r1:history", "r1:charge"],
            component(
                "builtin:resistor:v1",
                ["in", "out"],
                &[(0, 0), (0, 1), (1, 1)],
            ),
        );

        let fingerprints = BTreeSet::from([
            base,
            changed_model,
            changed_unknown_layout,
            changed_state_layout,
            changed_stamp,
        ]);
        assert_eq!(fingerprints.len(), 5);
    }

    #[test]
    fn invalid_terminals_and_stamp_coordinates_cannot_be_fingerprinted() {
        let missing_terminal = component("builtin:resistor:v1", ["in", "missing"], &[(0, 0)]);
        assert!(matches!(
            TopologyFingerprint::from_materialized(
                ["0", "in", "out"],
                ["v(in)", "v(out)"],
                std::iter::empty::<&str>(),
                [missing_terminal]
            ),
            Err(TopologyFingerprintError::UnknownTerminal { component, terminal })
                if component == "r1" && terminal == "missing"
        ));

        let invalid_stamp = component("builtin:resistor:v1", ["in", "out"], &[(0, 2)]);
        assert!(matches!(
            TopologyFingerprint::from_materialized(
                ["0", "in", "out"],
                ["v(in)", "v(out)"],
                std::iter::empty::<&str>(),
                [invalid_stamp]
            ),
            Err(TopologyFingerprintError::StampOutsideUnknownLayout {
                row: 0,
                column: 2,
                unknown_count: 2,
                ..
            })
        ));
    }

    #[test]
    fn invalid_or_duplicate_descriptors_cannot_produce_a_fingerprint() {
        assert!(matches!(
            TopologyComponent::new("r", "", "builtin:resistor:v1", ["a", "b"], [(0, 0)]),
            Err(TopologyFingerprintError::EmptyComponentName)
        ));
        assert!(matches!(
            TopologyComponent::new("r", "r1", "", ["a", "b"], [(0, 0)]),
            Err(TopologyFingerprintError::EmptyModelIdentity { .. })
        ));
        let component =
            TopologyComponent::new("r", "r1", "builtin:resistor:v1", ["a", "b"], [(0, 0)])
                .expect("R1");
        assert!(matches!(
            TopologyFingerprint::from_materialized(
                ["0", "a", "b"],
                ["v(a)", "v(b)"],
                std::iter::empty::<&str>(),
                [component.clone(), component]
            ),
            Err(TopologyFingerprintError::DuplicateComponent)
        ));
        assert!(matches!(
            TopologyFingerprint::from_materialized(
                ["0", "a", "A"],
                ["v(a)", "v(b)"],
                std::iter::empty::<&str>(),
                std::iter::empty()
            ),
            Err(TopologyFingerprintError::DuplicateNode { node }) if node == "a"
        ));
        assert!(matches!(
            TopologyFingerprint::from_materialized(
                ["0", "a", "b"],
                ["v(a)", "V(A)"],
                std::iter::empty::<&str>(),
                std::iter::empty()
            ),
            Err(TopologyFingerprintError::DuplicateUnknown { unknown }) if unknown == "v(a)"
        ));
    }

    #[test]
    fn domain_and_version_are_part_of_the_hash_input() {
        let descriptor = StructuralTopology::new(
            ["0", "in", "out"],
            ["v(in)", "v(out)"],
            std::iter::empty::<&str>(),
            [component(
                "builtin:resistor:v1",
                ["in", "out"],
                &[(0, 0), (1, 1)],
            )],
        )
        .expect("descriptor");
        let production = hash_structural_descriptor(
            &descriptor,
            TOPOLOGY_FINGERPRINT_DOMAIN,
            TOPOLOGY_FINGERPRINT_VERSION,
        );
        let different_domain = hash_structural_descriptor(
            &descriptor,
            b"rspice.execution.other-domain",
            TOPOLOGY_FINGERPRINT_VERSION,
        );
        let different_version = hash_structural_descriptor(
            &descriptor,
            TOPOLOGY_FINGERPRINT_DOMAIN,
            TOPOLOGY_FINGERPRINT_VERSION + 1,
        );
        assert_ne!(production, different_domain);
        assert_ne!(production, different_version);
    }
}

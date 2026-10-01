//! Armed port contracts and placement sequences over the current design revision.

use super::placement_authority::PlacementAuthority;
use rspice_design::schematic::port::{
    PortContract, PortDirectionType, PortDiscipline, PortSignalType,
};
use rspice_design_model::port::PortDirection;

/// Validated one-shot configuration owned by the armed port tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingPortPlacement {
    pub name: String,
    pub contract: PortContract,
    pub expected_topology_version: u64,
    pub expected_netlist_order: usize,
    pub document_authority: Option<PortPlacementAuthority>,
}

/// Application-document authority captured when a validated port draft arms
/// the one-shot canvas tool. Schematic-only callers can leave it absent, but
/// the interactive placement boundary requires an exact match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortPlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
}

/// The names still to place and the contract they share.
///
/// Runtime only, and deliberately so: what reaches the document is one ordinary
/// `Component` per name, with the same `params` string a single placement has
/// always written. Naming several pins at once is a property of the command,
/// not of the file.
///
/// The one-shot payload for each name is built at the click rather than when
/// the sequence is armed — see [`Self::next_placement`] — because the topology
/// version and the next interface order both move as the sequence is consumed,
/// and a payload frozen at arming time would refuse its own second placement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingPortSequence {
    /// Names still to place, in the order they were typed.
    pub names: std::collections::VecDeque<String>,
    /// How many names the sequence started with, for `{k}/{n}`.
    pub total: usize,
    pub direction: PortDirection,
    pub signal_type: PortSignalType,
    pub discipline: PortDiscipline,
    pub authority: Option<PlacementAuthority>,
}

impl PendingPortSequence {
    pub fn new(
        names: impl IntoIterator<Item = String>,
        direction: PortDirection,
        signal_type: PortSignalType,
        discipline: PortDiscipline,
    ) -> Self {
        let names: std::collections::VecDeque<String> = names.into_iter().collect();
        Self {
            total: names.len(),
            names,
            direction,
            signal_type,
            discipline,
            authority: None,
        }
    }

    pub fn with_authority(mut self, authority: PlacementAuthority) -> Self {
        self.authority = Some(authority);
        self
    }

    /// The name the next click will place.
    pub fn next_name(&self) -> Option<&str> {
        self.names.front().map(String::as_str)
    }

    /// One-based position of the next name in the sequence.
    pub fn position(&self) -> usize {
        self.total - self.names.len() + 1
    }

    /// The one-shot payload for the next name, measured against the live
    /// document.
    pub fn next_placement(
        &self,
        topology_version: u64,
        next_interface_order: usize,
    ) -> Option<PendingPortPlacement> {
        let name = self.names.front()?;
        Some(PendingPortPlacement::from_contract(
            name,
            self.direction,
            self.signal_type,
            self.discipline,
            topology_version,
            next_interface_order,
        ))
    }

    /// Drop the name just placed. Returns `false` when nothing is left.
    pub fn advance(&mut self) -> bool {
        self.names.pop_front();
        !self.names.is_empty()
    }
}

impl PendingPortPlacement {
    /// The payload for one name under an already-chosen contract.
    ///
    /// The three contract fields are separate here because the interface they
    /// describe has always had three: [`PortDirectionType`] fuses direction and
    /// signal type into five pairs, and no pair of those five is a logic
    /// output, so no digital block's output can be spelled through it.
    pub fn from_contract(
        name: impl Into<String>,
        direction: PortDirection,
        signal_type: PortSignalType,
        discipline: PortDiscipline,
        expected_topology_version: u64,
        expected_netlist_order: usize,
    ) -> Self {
        let name = name.into();
        let contract = PortContract {
            direction,
            signal_type,
            discipline,
            netlist_order: Some(expected_netlist_order),
            // The sentence [`PortContract::from_component`] would generate for
            // a port carrying no `documentation=` entry, so a pin placed here
            // reads the same as one recovered from a hand-written file.
            documentation: format!(
                "{name} {} {} interface port",
                direction.keyword(),
                discipline.keyword()
            ),
        };
        Self {
            name,
            contract,
            expected_topology_version,
            expected_netlist_order,
            document_authority: None,
        }
    }

    pub fn new(
        name: impl Into<String>,
        direction_type: PortDirectionType,
        discipline: PortDiscipline,
        expected_topology_version: u64,
        expected_netlist_order: usize,
    ) -> Self {
        let name = name.into();
        let contract = PortContract {
            direction: direction_type.direction(),
            signal_type: direction_type.signal_type(),
            discipline,
            netlist_order: Some(expected_netlist_order),
            documentation: format!(
                "{name} {} {} interface port",
                direction_type.label(),
                discipline.keyword()
            ),
        };
        Self {
            name,
            contract,
            expected_topology_version,
            expected_netlist_order,
            document_authority: None,
        }
    }

    pub fn with_document_authority(
        mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: impl Into<String>,
    ) -> Self {
        self.document_authority = Some(PortPlacementAuthority {
            design_execution_epoch,
            active_schematic_epoch,
            view_path: view_path.into(),
        });
        self
    }
}

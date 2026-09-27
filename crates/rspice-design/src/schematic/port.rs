//! Durable interface metadata encoded in component parameter text.

use super::component::Component;
use super::component_type::ComponentType;
pub use rspice_design_model::port::{PortDirection, PortSpec};
use serde::{Deserialize, Serialize};

/// Signal semantics declared by an interface port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PortSignalType {
    Logic,
    #[default]
    Analog,
    Power,
}

impl PortSignalType {
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Logic => "logic",
            Self::Analog => "analog",
            Self::Power => "power",
        }
    }

    fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "logic" | "digital" => Self::Logic,
            "power" | "supply" | "rail" => Self::Power,
            _ => Self::Analog,
        }
    }
}

/// Physical or behavioral discipline carried by an interface port.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PortDiscipline {
    #[default]
    Electrical,
    Logic,
    Wreal,
    Thermal,
}

impl PortDiscipline {
    pub const ALL: [Self; 4] = [Self::Electrical, Self::Logic, Self::Wreal, Self::Thermal];

    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Electrical => "electrical",
            Self::Logic => "logic",
            Self::Wreal => "wreal",
            Self::Thermal => "thermal",
        }
    }

    fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "logic" | "digital" => Self::Logic,
            "wreal" | "real" => Self::Wreal,
            "thermal" | "temperature" | "heat" => Self::Thermal,
            _ => Self::Electrical,
        }
    }
}

/// The exact direction/type combinations an interface pin may declare.
/// Keeping the pair typed prevents a dialog index or translated label from
/// silently producing a different electrical contract.
///
/// [`Self::SupplyPower`] is the rail case, and it is a different electrical
/// contract from [`Self::InOutPower`] rather than a label for it: a supply pin
/// states that the parent feeds this net, which is why the generated symbol
/// puts it on the top/bottom edge and why the topology preflight treats a net
/// fed only by one as reaching a source rather than as floating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PortDirectionType {
    #[default]
    InputLogic,
    InputAnalog,
    OutputAnalog,
    InOutPower,
    SupplyPower,
}

impl PortDirectionType {
    pub const ALL: [Self; 5] = [
        Self::InputLogic,
        Self::InputAnalog,
        Self::OutputAnalog,
        Self::InOutPower,
        Self::SupplyPower,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::InputLogic => "input \u{00b7} logic",
            Self::InputAnalog => "input \u{00b7} analog",
            Self::OutputAnalog => "output \u{00b7} analog",
            Self::InOutPower => "inout \u{00b7} power",
            Self::SupplyPower => "supply \u{00b7} power",
        }
    }

    pub const fn direction(self) -> PortDirection {
        match self {
            Self::InputLogic | Self::InputAnalog => PortDirection::In,
            Self::OutputAnalog => PortDirection::Out,
            Self::InOutPower => PortDirection::InOut,
            Self::SupplyPower => PortDirection::Supply,
        }
    }

    pub const fn signal_type(self) -> PortSignalType {
        match self {
            Self::InputLogic => PortSignalType::Logic,
            Self::InputAnalog | Self::OutputAnalog => PortSignalType::Analog,
            Self::InOutPower | Self::SupplyPower => PortSignalType::Power,
        }
    }
}

/// Durable, typed interface metadata encoded in the component parameter
/// string. SPICE itself consumes the positional port list; hierarchy,
/// generated symbols, documentation, AMS tooling and future connect rules
/// consume this richer contract without maintaining a second source of truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortContract {
    pub direction: PortDirection,
    pub signal_type: PortSignalType,
    pub discipline: PortDiscipline,
    /// One-based positional interface order. Legacy ports derive this from
    /// document order until they are rewritten through the typed editor.
    pub netlist_order: Option<usize>,
    pub documentation: String,
}

impl PortContract {
    fn from_component(component: &Component, name: &str) -> Self {
        let params = crate::parameters::parse_params_string(&component.params);
        let direction = params
            .get("dir")
            .map(|raw| PortDirection::parse(raw))
            .unwrap_or_default();
        let signal_type = params
            .get("signal_type")
            .map(|raw| PortSignalType::parse(raw))
            .unwrap_or_else(|| {
                if direction == PortDirection::Supply {
                    PortSignalType::Power
                } else {
                    PortSignalType::Analog
                }
            });
        let discipline = params
            .get("discipline")
            .map(|raw| PortDiscipline::parse(raw))
            .unwrap_or_default();
        let netlist_order = params
            .get("interface_order")
            .and_then(|raw| parse_interface_order(raw));
        let documentation = params.get("documentation").cloned().unwrap_or_else(|| {
            format!(
                "{name} {} {} interface port",
                direction.keyword(),
                discipline.keyword()
            )
        });
        Self {
            direction,
            signal_type,
            discipline,
            netlist_order,
            documentation,
        }
    }

    pub fn encoded_params(&self) -> String {
        let mut values = std::collections::HashMap::from([
            ("dir".to_owned(), self.direction.keyword().to_owned()),
            (
                "signal_type".to_owned(),
                self.signal_type.keyword().to_owned(),
            ),
            (
                "discipline".to_owned(),
                self.discipline.keyword().to_owned(),
            ),
            ("documentation".to_owned(), self.documentation.clone()),
        ]);
        if let Some(order) = self.netlist_order {
            values.insert("interface_order".to_owned(), order.to_string());
        }
        crate::parameters::format_params_string(&values)
    }
}

/// Read a durable interface position.
///
/// The numeric property editor writes a whole number as `2`, but a value that
/// arrived from a hand-edited file or a future schema may not be a position at
/// all. Anything that is not a positive whole number is no position, and the
/// port falls back to document order — `interface_order_is_well_formed` is what
/// stops that fallback from being silent on an edit.
fn parse_interface_order(raw: &str) -> Option<usize> {
    let value = raw.trim().parse::<f64>().ok()?;
    (value.is_finite() && value > 0.0 && value.fract() == 0.0).then_some(value as usize)
}

impl Component {
    /// The interface pin this component declares, when it is a named port.
    ///
    /// The port's name lives in `value` (it doubles as the net name); the
    /// direction in the `dir=` entry of `params`. An unnamed port declares
    /// nothing — netlist generation reports it instead of guessing.
    pub fn port_spec(&self) -> Option<PortSpec> {
        if self.kind != ComponentType::Port {
            return None;
        }
        let name = self.value.trim();
        if name.is_empty() {
            return None;
        }
        let direction = PortContract::from_component(self, name).direction;
        Some(PortSpec {
            name: name.to_string(),
            direction,
        })
    }

    /// Typed interface metadata for a named port.
    pub fn port_contract(&self) -> Option<PortContract> {
        let spec = self.port_spec()?;
        Some(PortContract::from_component(self, &spec.name))
    }
}

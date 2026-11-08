//! Static instance parameters read by name, the way Xyce's `R1:R` and
//! ngspice's `@m1[w]` read them: the value installed in this circuit, not one
//! reconstructed from the card. Only parameters that cannot move during a
//! solve are answered, so one value stands for every point of an analysis.

use super::*;
use crate::netlist::{OutputOperandKind, SaveSignal, canonical_symbol};

impl CircuitData {
    /// The installed value of one static instance parameter, with the
    /// device's name as this circuit spells it. `None` when no device has the
    /// name, the family has no such parameter, or the value moves during the
    /// solve (a thermal resistor, an expression-valued capacitor).
    pub(crate) fn static_instance_parameter(
        &self,
        device: &str,
        parameter: &str,
    ) -> Option<(String, Value)> {
        let key = canonical_symbol(device);
        let parameter = parameter.trim().to_ascii_uppercase();
        let is = |names: &[&str]| names.contains(&parameter.as_str());
        let position =
            |names: &[String]| names.iter().position(|name| canonical_symbol(name) == key);
        let named =
            |names: &[String], index: usize, value: Value| Some((names[index].clone(), value));

        if let Some(index) = position(&self.resistors.names) {
            let moving = self
                .resistors
                .thermal
                .get(index)
                .is_some_and(Option::is_some);
            return (is(&["R", "RESISTANCE"]) && !moving)
                .then(|| {
                    named(
                        &self.resistors.names,
                        index,
                        self.resistors.reported_resistances[index],
                    )
                })
                .flatten();
        }
        if let Some(index) = position(&self.resistor_branches.names) {
            return is(&["R", "RESISTANCE"])
                .then(|| {
                    named(
                        &self.resistor_branches.names,
                        index,
                        self.resistor_branches.reported_resistances[index],
                    )
                })
                .flatten();
        }
        if let Some(index) = self
            .capacitors
            .names
            .iter()
            .zip(&self.capacitors.internal)
            .position(|(name, internal)| !internal && canonical_symbol(name) == key)
        {
            let moving = self
                .capacitors
                .value_expressions
                .get(index)
                .is_some_and(Option::is_some);
            return (is(&["C", "CAPACITANCE"]) && !moving)
                .then(|| {
                    named(
                        &self.capacitors.names,
                        index,
                        self.capacitors.capacitances[index],
                    )
                })
                .flatten();
        }
        if let Some(index) = position(&self.inductors.names) {
            return is(&["L", "INDUCTANCE"])
                .then(|| {
                    named(
                        &self.inductors.names,
                        index,
                        self.inductors.inductances[index],
                    )
                })
                .flatten();
        }
        if let Some(index) = position(&self.voltage_sources.names) {
            return is(&["DCV0", "DC"])
                .then(|| {
                    named(
                        &self.voltage_sources.names,
                        index,
                        self.voltage_sources.dc_values[index],
                    )
                })
                .flatten();
        }
        if let Some(index) = position(&self.current_sources.names) {
            return is(&["DCV0", "DC"])
                .then(|| {
                    named(
                        &self.current_sources.names,
                        index,
                        self.current_sources.dc_values[index],
                    )
                })
                .flatten();
        }
        let geometry = |name: &String, l: Value, w: Value| {
            (canonical_symbol(name) == key)
                .then(|| match parameter.as_str() {
                    "L" => Some((name.clone(), l)),
                    "W" => Some((name.clone(), w)),
                    _ => None,
                })
                .flatten()
        };
        self.mosfets
            .devices
            .iter()
            .find_map(|device| geometry(&device.name, device.l, device.w))
            .or_else(|| {
                self.bsim3v3.devices.iter().find_map(|device| {
                    geometry(&device.name, device.core.geom.l, device.core.geom.w)
                })
            })
            .or_else(|| {
                self.bsim4v8.devices.iter().find_map(|device| {
                    geometry(&device.name, device.core.geom.l, device.core.geom.w)
                })
            })
    }

    /// The installed values of every static instance parameter an output card
    /// names, as `(device, parameter, value)` with the parameter spelled as
    /// authored.
    pub(crate) fn requested_static_instance_parameters(
        &self,
        netlist: &crate::netlist::Netlist,
    ) -> Vec<(String, String, Value)> {
        requested_instance_parameters(netlist)
            .into_iter()
            .filter_map(|(device, parameter)| {
                self.static_instance_parameter(&device, &parameter)
                    .map(|(device, value)| (device, parameter, value))
            })
            .collect()
    }
}

/// The `(device, parameter)` pairs the deck's output cards name, in Xyce's
/// `device:param` or ngspice's `@device[param]` spelling, directly or inside a
/// braced expression. A pair no device answers is simply never published.
pub(crate) fn requested_instance_parameters(
    netlist: &crate::netlist::Netlist,
) -> Vec<(String, String)> {
    let mut pairs: Vec<(String, String)> = Vec::new();
    let mut push = |device: &str, parameter: &str| {
        let (device, parameter) = (device.trim(), parameter.trim());
        if device.is_empty()
            || parameter.is_empty()
            || pairs.iter().any(|(known_device, known_parameter)| {
                known_device.eq_ignore_ascii_case(device)
                    && known_parameter.eq_ignore_ascii_case(parameter)
            })
        {
            return;
        }
        pairs.push((device.to_owned(), parameter.to_owned()));
    };
    for request in &netlist.output_requests {
        for kind in request.operand_kinds() {
            match kind {
                OutputOperandKind::Probe(SaveSignal::DeviceParam { device, param }) => {
                    push(device, param);
                }
                OutputOperandKind::Probe(SaveSignal::Raw(raw)) => {
                    if let Some((device, parameter)) = raw.rsplit_once(':') {
                        push(device, parameter);
                    }
                }
                OutputOperandKind::Expression { body } => {
                    for (device, parameter) in expression_instance_parameters(body) {
                        push(device, parameter);
                    }
                }
                OutputOperandKind::Probe(_) => {}
            }
        }
    }
    pairs
}

/// The `device:param` and `@device[param]` names inside one expression body.
fn expression_instance_parameters(body: &str) -> Vec<(&str, &str)> {
    let is_name = |character: char| {
        character.is_ascii_alphanumeric()
            || matches!(character, '_' | ':' | '.' | '$' | '!' | '#' | '`')
    };
    let mut found = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find(|character: char| character == '@' || is_name(character)) {
        rest = &rest[start..];
        if let Some(after) = rest.strip_prefix('@') {
            let device_end = after
                .find(|character: char| !is_name(character))
                .unwrap_or(after.len());
            let (device, tail) = after.split_at(device_end);
            if let Some((parameter, remainder)) =
                tail.strip_prefix('[').and_then(|tail| tail.split_once(']'))
            {
                found.push((device, parameter));
                rest = remainder;
            } else {
                rest = tail;
            }
            continue;
        }
        let end = rest
            .find(|character: char| !is_name(character))
            .unwrap_or(rest.len());
        let (name, remainder) = rest.split_at(end);
        if let Some((device, parameter)) = name.rsplit_once(':') {
            found.push((device, parameter));
        }
        rest = remainder;
    }
    found
}

#[cfg(test)]
mod tests {
    use super::expression_instance_parameters;

    #[test]
    fn an_expression_names_its_colon_and_bracket_instance_parameters() {
        assert_eq!(
            expression_instance_parameters(" Xtest1:Xtest:R1:R + 2*@m1[w] - V(out)"),
            [("Xtest1:Xtest:R1", "R"), ("m1", "w")]
        );
        assert!(expression_instance_parameters("V(1)*2+I(V1)").is_empty());
    }
}

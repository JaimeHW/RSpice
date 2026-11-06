//! Static instance parameters read by name, the way Xyce's `R1:R` and
//! ngspice's `@m1[w]` read them: the value installed in this circuit, not one
//! reconstructed from the card. Only parameters that cannot move during a
//! solve are answered, so one value stands for every point of an analysis.
//! A temperature coefficient and a model card's parameter, which no installed
//! device keeps, are read from the deck.

use super::*;
use crate::netlist::{
    Element, ElementKind, FlattenedNetlist, ModelDef, OutputOperandKind, SaveSignal,
    canonical_symbol,
};

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
    /// authored. A pair no installed device answers is read from the deck
    /// ([`authored_static_parameter`]).
    pub(crate) fn requested_static_instance_parameters(
        &self,
        netlist: &crate::netlist::Netlist,
    ) -> Vec<(String, String, Value)> {
        let mut flattened = None;
        requested_instance_parameters(netlist)
            .into_iter()
            .filter_map(|(device, parameter)| {
                self.static_instance_parameter(&device, &parameter)
                    .or_else(|| {
                        authored_static_parameter(netlist, &mut flattened, &device, &parameter)
                    })
                    .map(|(device, value)| (device, parameter, value))
            })
            .collect()
    }
}

/// A `device:param` no installed device answers, read from the deck the way
/// Xyce's `DeviceEntity::getParam` reads a value that cannot move during a
/// solve, in every analysis: the `TC1` or `TC2` of a resistor, capacitor or
/// inductor (the instance's, else its model card's, else 0), and a parameter
/// a top-level model card authors (`DMOD:IS`, `NMOD:VTO`, `RMOD:TC1`). A model parameter
/// left at its family default is not answered, nor is a model scoped inside a
/// subcircuit, which Xyce refuses too. `flattened` caches the hierarchy, which
/// is expanded only for a coefficient of an element no top-level card names.
fn authored_static_parameter(
    netlist: &crate::netlist::Netlist,
    flattened: &mut Option<Option<FlattenedNetlist>>,
    device: &str,
    parameter: &str,
) -> Option<(String, Value)> {
    let key = canonical_symbol(device);
    if ["TC1", "TC2"]
        .iter()
        .any(|name| parameter.trim().eq_ignore_ascii_case(name))
    {
        let coefficient = |elements: &[Element], scoped: &[ModelDef]| {
            elements
                .iter()
                .find(|element| canonical_symbol(&element.name) == key)
                .and_then(|element| {
                    temperature_coefficient(element, netlist.models.iter().chain(scoped), parameter)
                })
        };
        let element = coefficient(&netlist.elements, &[]).or_else(|| {
            flattened
                .get_or_insert_with(|| crate::netlist::flatten_netlist_with_models(netlist).ok())
                .as_ref()
                .and_then(|hierarchy| coefficient(&hierarchy.elements, &hierarchy.scoped_models))
        });
        if element.is_some() {
            return element;
        }
    }
    let model = netlist
        .models
        .iter()
        .find(|model| canonical_symbol(&model.name) == key)?;
    authored(&model.params, parameter).map(|value| (model.name.clone(), value))
}

/// The `TC1` or `TC2` a resistor, capacitor or inductor is built with: the
/// instance's, else its model card's, else 0, as the builder resolves it.
fn temperature_coefficient<'a>(
    element: &Element,
    mut models: impl Iterator<Item = &'a ModelDef>,
    parameter: &str,
) -> Option<(String, Value)> {
    let (ElementKind::Resistor {
        model,
        instance_params,
        ..
    }
    | ElementKind::Capacitor {
        model,
        instance_params,
        ..
    }
    | ElementKind::Inductor {
        model,
        instance_params,
        ..
    }) = &element.kind
    else {
        return None;
    };
    let value = authored(instance_params, parameter)
        .or_else(|| {
            let model = model.as_deref()?;
            let card = models.find(|card| card.name.eq_ignore_ascii_case(model))?;
            authored(&card.params, parameter)
        })
        .unwrap_or(0.0);
    Some((element.name.clone(), value))
}

/// The value a card authors for `parameter`, by name in any case.
fn authored(params: &[(String, Value)], parameter: &str) -> Option<Value> {
    params
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(parameter.trim()))
        .map(|(_, value)| *value)
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

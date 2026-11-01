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
            return is(&["DCV0"])
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
            return is(&["DCV0"])
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
    use crate::abort_signal::NoAbort;
    use crate::analysis::measure_signals::{
        FrontendOutputColumn, evaluate_dc_output_requests_with_abort,
        evaluate_tran_output_requests_with_abort,
    };
    use crate::netlist::Netlist;

    const DEVICES: &str = "\
V1 1 0 DC 1 PWL 0 0 1u 1
R1 1 2 2
C1 2 0 1u
L1 2 3 1m
R2 3 0 1
M1 3 1 0 0 NM L=2u W=10u
.MODEL NM NMOS LEVEL=1
";

    fn xyce_deck(analysis: &str) -> Netlist {
        Netlist::parse_with_options(
            &format!("instance parameters\n{DEVICES}{analysis}.END\n"),
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce deck parses")
    }

    fn xyce_engine() -> crate::Engine {
        crate::Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::config::SpiceDialect::Xyce),
        )
    }

    fn assert_columns(columns: &[FrontendOutputColumn], expected: &[(&str, &[f64])]) {
        for (name, values) in expected {
            let (_, _, column) = columns
                .iter()
                .find(|(column, _, _)| column == name)
                .unwrap_or_else(|| panic!("no column {name}"));
            assert_eq!(column.len(), values.len(), "{name} rows");
            for (actual, expected) in column.iter().zip(values.iter()) {
                assert!(
                    (actual - expected).abs() <= 1e-12 * expected.abs(),
                    "{name}: {actual} != {expected}"
                );
            }
        }
    }

    #[test]
    fn a_transient_prints_each_static_instance_parameter_at_every_point() {
        // Xyce 7.10 prints R1:R 2, C1:C 1u, L1:L 1m, V1:DCV0 1, M1:L 2u and
        // M1:W 10u at each of its 33 time points.
        let netlist = xyce_deck(".TRAN 0 1u\n.PRINT TRAN R1:R C1:C L1:L V1:DCV0 M1:L M1:W\n");
        let result = xyce_engine()
            .run_tran(&netlist, 1e-6, 1e-7)
            .expect("transient runs");
        let columns = evaluate_tran_output_requests_with_abort(
            &netlist,
            &result,
            Default::default(),
            &NoAbort,
        )
        .expect("every instance parameter resolves");
        let points = result.time.len();
        assert!(points > 2);
        let constant = |value: f64| vec![value; points];
        assert_columns(
            &columns,
            &[
                ("R1:R", &constant(2.0)),
                ("C1:C", &constant(1e-6)),
                ("L1:L", &constant(1e-3)),
                ("V1:DCV0", &constant(1.0)),
                ("M1:L", &constant(2e-6)),
                ("M1:W", &constant(1e-5)),
            ],
        );
    }

    #[test]
    fn a_dc_sweep_prints_each_static_instance_parameter_at_its_point() {
        // Xyce 7.10: V1:DCV0 follows the sweep (1, 2); the others are the
        // installed values.
        let netlist = xyce_deck(".DC V1 1 2 1\n.PRINT DC V1:DCV0 C1:C L1:L M1:L M1:W\n");
        let sweep = xyce_engine()
            .run_dc_sweep(&netlist, "V1", 1.0, 2.0, 1.0)
            .expect("sweep runs");
        let columns =
            evaluate_dc_output_requests_with_abort(&netlist, &sweep, Default::default(), &NoAbort)
                .expect("every instance parameter resolves");
        assert_columns(
            &columns,
            &[
                ("V1:DCV0", &[1.0, 2.0]),
                ("C1:C", &[1e-6, 1e-6]),
                ("L1:L", &[1e-3, 1e-3]),
                ("M1:L", &[2e-6, 2e-6]),
                ("M1:W", &[1e-5, 1e-5]),
            ],
        );
    }

    #[test]
    fn an_expression_names_its_colon_and_bracket_instance_parameters() {
        assert_eq!(
            expression_instance_parameters(" Xtest1:Xtest:R1:R + 2*@m1[w] - V(out)"),
            [("Xtest1:Xtest:R1", "R"), ("m1", "w")]
        );
        assert!(expression_instance_parameters("V(1)*2+I(V1)").is_empty());
    }
}

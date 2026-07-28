//! Analytic oracles and per-family run qualification.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn analytic_rc_specification(
        netlist: &Netlist,
        plan: &XyceStaticTranPlan,
        source: &XyceAnalyticRcSourceContract,
    ) -> Result<XyceAnalyticRcSpecification, String> {
        const LABEL: &str = "analytic first-order RC";
        if netlist.title.trim().is_empty()
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} contains model, auxiliary analysis, hierarchy, external-model, or diagnostic state"
            ));
        }
        if !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{LABEL} does not admit parameters or functions"));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }]) {
            return Err(format!("{LABEL} requires exactly one transient analysis"));
        }
        if !Self::analytic_rc_options_match(&netlist.options, source) {
            return Err(format!(
                "{LABEL} parsed options differ from the bounded TIMEINT/RELTOL/ABSTOL source contract"
            ));
        }
        if netlist.elements.len() != 3 {
            return Err(format!(
                "{LABEL} requires exactly three elements, found {}",
                netlist.elements.len()
            ));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::passive_primary_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "element '{}' uses ground alias '{}'; {LABEL} requires literal node 0",
                    element.name, alias
                ));
            }
            let name = Self::normalize_device_instance_name(&element.name);
            if elements.insert(name.clone(), element).is_some() {
                return Err(format!("{LABEL} contains duplicate element name '{name}'"));
            }
        }
        let expected_names = [
            source.capacitor_name.clone(),
            source.resistor_name.clone(),
            source.source_name.clone(),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if elements.keys().cloned().collect::<BTreeSet<_>>() != expected_names {
            return Err(format!(
                "{LABEL} parsed element inventory differs from direct source provenance"
            ));
        }

        let capacitor = elements
            .get(&source.capacitor_name)
            .expect("element inventory was checked");
        let capacitor_nodes = capacitor
            .nodes
            .iter()
            .map(|node| Self::canonical_passive_primary_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::Capacitor {
            value: capacitance,
            value_expr,
            initial_voltage,
            model,
            instance_params,
            deferred_params,
        } = &capacitor.kind
        else {
            return Err(format!(
                "source-qualified capacitor '{}' parsed as another element kind",
                capacitor.name
            ));
        };
        if capacitor_nodes.as_slice() != source.capacitor_nodes
            || !capacitance.is_finite()
            || capacitance.to_bits() != source.capacitance_bits
            || value_expr.is_some()
            || initial_voltage.map(Value::to_bits) != Some(source.initial_voltage_bits)
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "capacitor '{}' differs from the direct finite value/IC source contract",
                capacitor.name
            ));
        }
        let effective_capacitance = Self::effective_capacitor_value(netlist, &capacitor.name)
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| format!("{LABEL} capacitance did not resolve"))?;
        if effective_capacitance.to_bits() != source.capacitance_bits {
            return Err(format!(
                "{LABEL} explicit and effective capacitance values differ"
            ));
        }

        let resistor = elements
            .get(&source.resistor_name)
            .expect("element inventory was checked");
        let resistor_nodes = resistor
            .nodes
            .iter()
            .map(|node| Self::canonical_passive_primary_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::Resistor {
            value: resistance,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } = &resistor.kind
        else {
            return Err(format!(
                "source-qualified resistor '{}' parsed as another element kind",
                resistor.name
            ));
        };
        if resistor_nodes.as_slice() != source.resistor_nodes
            || !resistance.is_finite()
            || resistance.to_bits() != source.resistance_bits
            || value_expr.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "resistor '{}' differs from the direct finite source contract",
                resistor.name
            ));
        }
        let effective_resistance = Self::effective_resistor_value(netlist, &resistor.name)?
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| format!("{LABEL} resistance did not resolve"))?;
        if effective_resistance.to_bits() != source.resistance_bits {
            return Err(format!(
                "{LABEL} explicit and effective resistance values differ"
            ));
        }

        let voltage_source = elements
            .get(&source.source_name)
            .expect("element inventory was checked");
        let source_nodes = voltage_source
            .nodes
            .iter()
            .map(|node| Self::canonical_passive_primary_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(source_value)) =
            &voltage_source.kind
        else {
            return Err(format!(
                "source-qualified voltage source '{}' is not direct DC",
                voltage_source.name
            ));
        };
        if source_nodes.as_slice() != source.source_nodes
            || !source_value.is_finite()
            || source_value.to_bits() != source.source_value_bits
        {
            return Err(format!(
                "voltage source '{}' differs from direct source provenance",
                voltage_source.name
            ));
        }

        let [capacitor_node, capacitor_ground] = &source.capacitor_nodes;
        let [source_node, source_ground] = &source.source_nodes;
        let [resistor_a, resistor_b] = &source.resistor_nodes;
        if capacitor_ground != "0"
            || source_ground != "0"
            || capacitor_node == "0"
            || source_node == "0"
            || capacitor_node == source_node
            || source.probe_node != *capacitor_node
            || !((resistor_a == capacitor_node && resistor_b == source_node)
                || (resistor_b == capacitor_node && resistor_a == source_node))
        {
            return Err(format!(
                "{LABEL} topology must be modeled as C(output,0), R(output,source), and V(source,0)"
            ));
        }
        let initial_voltage = Value::from_bits(source.initial_voltage_bits);
        let initial_delta = initial_voltage - *source_value;
        if !initial_voltage.is_finite() || !initial_delta.is_finite() || initial_delta == 0.0 {
            return Err(format!(
                "{LABEL} requires a finite nontrivial initial-to-source voltage transition"
            ));
        }
        if plan.tran.stop.to_bits() != source.tran_stop_bits
            || plan.tran.step.to_bits() != source.tran_step_bits
        {
            return Err(format!("{LABEL} parsed and planned .TRAN tuples differ"));
        }
        let time_constant = effective_resistance * effective_capacitance;
        if !time_constant.is_finite() || time_constant <= 0.0 {
            return Err(format!(
                "{LABEL} RC time constant is not finite and positive"
            ));
        }
        if *source_value != XYCE_ANALYTIC_RC_ORACLE_FINAL_VALUE
            || initial_voltage.to_bits() != XYCE_ANALYTIC_RC_ORACLE_INITIAL_VALUE.to_bits()
            || time_constant.to_bits() != XYCE_ANALYTIC_RC_ORACLE_TIME_CONSTANT.to_bits()
        {
            return Err(format!(
                "{LABEL} deck must reproduce the generated Release 7.10 oracle with final value {}, initial value {}, and time constant {}",
                XYCE_ANALYTIC_RC_ORACLE_FINAL_VALUE,
                XYCE_ANALYTIC_RC_ORACLE_INITIAL_VALUE,
                XYCE_ANALYTIC_RC_ORACLE_TIME_CONSTANT,
            ));
        }

        Ok(XyceAnalyticRcSpecification {
            output_node: capacitor_node.clone(),
            source_value: *source_value,
            initial_voltage,
            resistance: effective_resistance,
            capacitance: effective_capacitance,
            time_constant,
        })
    }

    pub(super) fn analytic_sinusoidal_rc_specification(
        netlist: &Netlist,
        plan: &XyceStaticTranPlan,
        source: &XyceAnalyticSinusoidalRcSourceContract,
    ) -> Result<XyceAnalyticSinusoidalRcSpecification, String> {
        const LABEL: &str = "analytic sinusoidal first-order RC";
        if netlist.title.trim().is_empty()
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} contains model, hierarchy, auxiliary-analysis, external-model, or diagnostic state"
            ));
        }
        if !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{LABEL} does not admit parameters or functions"));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }]) {
            return Err(format!("{LABEL} requires exactly one transient analysis"));
        }
        if !Self::analytic_sinusoidal_rc_options_match(&netlist.options, source) {
            return Err(format!(
                "{LABEL} parsed options differ from the qualified TIMEINT source contract"
            ));
        }
        if netlist.elements.len() != 3 {
            return Err(format!(
                "{LABEL} requires exactly three elements, found {}",
                netlist.elements.len()
            ));
        }

        let mut elements = BTreeMap::new();
        for element in &netlist.elements {
            if let Some(alias) = element.nodes.iter().find(|node| {
                Self::xyce_ground_alias_name(node)
                    && !Self::passive_primary_name_is_literal_ground(node)
            }) {
                return Err(format!(
                    "element '{}' uses ground alias '{}'; {LABEL} requires literal node 0",
                    element.name, alias
                ));
            }
            let name = Self::normalize_device_instance_name(&element.name);
            if elements.insert(name.clone(), element).is_some() {
                return Err(format!("{LABEL} contains duplicate element '{name}'"));
            }
        }
        let expected_names = [
            source.capacitor_name.clone(),
            source.resistor_name.clone(),
            source.source_name.clone(),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if elements.keys().cloned().collect::<BTreeSet<_>>() != expected_names {
            return Err(format!(
                "{LABEL} parsed element inventory differs from source provenance"
            ));
        }

        let capacitor = elements
            .get(&source.capacitor_name)
            .expect("element inventory was checked");
        let capacitor_nodes = capacitor
            .nodes
            .iter()
            .map(|node| Self::canonical_passive_primary_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::Capacitor {
            value: capacitance,
            value_expr,
            initial_voltage,
            model,
            instance_params,
            deferred_params,
        } = &capacitor.kind
        else {
            return Err(format!(
                "source-qualified capacitor '{}' parsed as another element kind",
                capacitor.name
            ));
        };
        if capacitor_nodes.as_slice() != source.capacitor_nodes
            || !capacitance.is_finite()
            || capacitance.to_bits() != source.capacitance_bits
            || value_expr.is_some()
            || initial_voltage.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "capacitor '{}' differs from the direct value/no-IC source contract",
                capacitor.name
            ));
        }
        let effective_capacitance = Self::effective_capacitor_value(netlist, &capacitor.name)
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| format!("{LABEL} capacitance did not resolve"))?;
        if effective_capacitance.to_bits() != source.capacitance_bits {
            return Err(format!(
                "{LABEL} explicit and effective capacitance values differ"
            ));
        }

        let resistor = elements
            .get(&source.resistor_name)
            .expect("element inventory was checked");
        let resistor_nodes = resistor
            .nodes
            .iter()
            .map(|node| Self::canonical_passive_primary_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::Resistor {
            value: resistance,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } = &resistor.kind
        else {
            return Err(format!(
                "source-qualified resistor '{}' parsed as another element kind",
                resistor.name
            ));
        };
        if resistor_nodes.as_slice() != source.resistor_nodes
            || !resistance.is_finite()
            || resistance.to_bits() != source.resistance_bits
            || value_expr.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "resistor '{}' differs from the direct source contract",
                resistor.name
            ));
        }
        let effective_resistance = Self::effective_resistor_value(netlist, &resistor.name)?
            .filter(|value| value.is_finite() && *value > 0.0)
            .ok_or_else(|| format!("{LABEL} resistance did not resolve"))?;
        if effective_resistance.to_bits() != source.resistance_bits {
            return Err(format!(
                "{LABEL} explicit and effective resistance values differ"
            ));
        }

        let voltage_source = elements
            .get(&source.source_name)
            .expect("element inventory was checked");
        let source_nodes = voltage_source
            .nodes
            .iter()
            .map(|node| Self::canonical_passive_primary_node_name(node))
            .collect::<Vec<_>>();
        let ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Sin {
            offset,
            amplitude,
            frequency,
            delay,
            damping,
            phase,
        }) = &voltage_source.kind
        else {
            return Err(format!(
                "source-qualified voltage source '{}' is not direct SIN",
                voltage_source.name
            ));
        };
        if source_nodes.as_slice() != source.source_nodes
            || offset.to_bits() != source.source_offset_bits
            || amplitude.to_bits() != source.source_amplitude_bits
            || frequency.to_bits() != source.source_frequency_bits
            || delay.to_bits() != source.source_delay_bits
            || damping.to_bits() != source.source_damping_bits
            || phase.to_bits() != 0.0f64.to_bits()
        {
            return Err(format!(
                "voltage source '{}' differs from direct SIN source provenance",
                voltage_source.name
            ));
        }

        let [capacitor_node, capacitor_ground] = &source.capacitor_nodes;
        let [source_node, source_ground] = &source.source_nodes;
        let [resistor_a, resistor_b] = &source.resistor_nodes;
        if capacitor_ground != "0"
            || source_ground != "0"
            || capacitor_node == "0"
            || source_node == "0"
            || capacitor_node == source_node
            || source.probe_node != *capacitor_node
            || !((resistor_a == capacitor_node && resistor_b == source_node)
                || (resistor_b == capacitor_node && resistor_a == source_node))
        {
            return Err(format!(
                "{LABEL} topology must be C(output,0), R(output,source), and SIN V(source,0)"
            ));
        }

        let expression = Self::print_expression_inner(&source.print_expression)
            .ok_or_else(|| format!("{LABEL} print expression lost its braces"))?;
        let prepared = prepare_behavioral_expression(expression, &netlist.params)
            .map_err(|err| format!("could not prepare {LABEL} print expression: {err}"))?;
        let ast = parse_expression_strict(&prepared)
            .map_err(|err| format!("could not parse prepared {LABEL} print expression: {err}"))?;
        let Expr::Binary {
            op: rspice_core::expr::BinaryOp::Add,
            left,
            right,
        } = ast
        else {
            return Err(format!(
                "{LABEL} prepared expression must directly add voltage and offset"
            ));
        };
        let Expr::NodeVoltage(node) = left.as_ref() else {
            return Err(format!(
                "{LABEL} prepared expression left operand is not node voltage"
            ));
        };
        let Expr::Const(print_offset) = right.as_ref() else {
            return Err(format!(
                "{LABEL} prepared expression right operand is not a constant"
            ));
        };
        if Self::canonical_passive_primary_node_name(node) != source.probe_node
            || print_offset.to_bits() != source.print_offset_bits
        {
            return Err(format!(
                "{LABEL} prepared expression changed its voltage node or offset"
            ));
        }
        if plan.tran.step.to_bits() != source.tran_step_bits
            || plan.tran.stop.to_bits() != source.tran_stop_bits
        {
            return Err(format!("{LABEL} parsed and planned .TRAN tuples differ"));
        }

        Ok(XyceAnalyticSinusoidalRcSpecification {
            output_node: capacitor_node.clone(),
            print_expression: source.print_expression.clone(),
            resistance: effective_resistance,
            capacitance: effective_capacitance,
            source_frequency: *frequency,
            print_offset: *print_offset,
        })
    }

    pub(super) fn subckt_parameter_resolution_qualification(
        netlist: &Netlist,
        print: &XycePrintRequest,
        sweep_source: &str,
    ) -> Result<
        (
            XyceSubcktParameterResolutionRepresentation,
            String,
            Option<XyceSubcktParameterResolutionSnapshot>,
        ),
        String,
    > {
        const LABEL: &str = "subcircuit-parameter resolution";
        let source = netlist
            .source_text
            .as_deref()
            .ok_or_else(|| format!("{LABEL} requires original source text"))?;
        Self::validate_subckt_parameter_resolution_source_directives(source)?;
        if !netlist.models.is_empty()
            || !netlist.diagnostics.is_empty()
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc { .. }])
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || netlist.subcircuits.len() != 1
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{LABEL} requires one flat top-level DC harness, one subcircuit definition, and no auxiliary or external model state"
            ));
        }

        let globals = netlist.params.numeric_parameters();
        if globals.len() > 1 || globals.iter().any(|(_, value)| !value.is_finite()) {
            return Err(format!(
                "{LABEL} admits at most one finite global scalar parameter"
            ));
        }
        let global = globals
            .first()
            .map(|(name, value)| (name.to_ascii_lowercase(), *value));

        let [subcircuit] = netlist.subcircuits.as_slice() else {
            return Err(format!("{LABEL} requires exactly one subcircuit"));
        };
        if subcircuit.ports.len() != 2
            || subcircuit.elements.len() != 1
            || subcircuit.params.len() > 1
            || subcircuit
                .params
                .iter()
                .any(|(_, value)| !value.is_finite())
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err(format!(
                "{LABEL} requires one two-port, one-resistor subcircuit with at most one finite formal parameter and no nested state"
            ));
        }
        let formal = subcircuit
            .params
            .first()
            .map(|(name, value)| (name.to_ascii_lowercase(), *value));

        let mut top_instance = None;
        let mut voltage_source = None;
        for element in &netlist.elements {
            if element.nodes.len() != 2
                || element.nodes.iter().any(|node| node.trim().is_empty())
                || element
                    .nodes
                    .iter()
                    .any(|node| Self::xyce_ground_alias_name(node) && node.trim() != "0")
            {
                return Err(format!(
                    "{LABEL} top-level element '{}' must use two explicit nodes and literal ground",
                    element.name
                ));
            }
            match &element.kind {
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } if top_instance.is_none()
                    && subckt_name.eq_ignore_ascii_case(&subcircuit.name)
                    && params.len() <= 1 =>
                {
                    top_instance = Some((element, params));
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if voltage_source.is_none() && value.is_finite() =>
                {
                    voltage_source = Some(element);
                }
                _ => {
                    return Err(format!(
                        "{LABEL} contains unqualified top-level element '{}'",
                        element.name
                    ));
                }
            }
        }
        let (instance_element, instance_params) = top_instance
            .ok_or_else(|| format!("{LABEL} contains no qualified subcircuit instance"))?;
        let voltage_source = voltage_source
            .ok_or_else(|| format!("{LABEL} contains no qualified independent DC source"))?;
        if netlist.elements.len() != 2
            || instance_element.nodes != voltage_source.nodes
            || instance_element.nodes[1].trim() != "0"
            || !voltage_source.name.eq_ignore_ascii_case(sweep_source)
        {
            return Err(format!(
                "{LABEL} requires the swept source directly across the two-port instance"
            ));
        }
        let instance = match instance_params.as_slice() {
            [] => None,
            [(name, ParametricValue::Resolved(value))] if value.is_finite() => {
                Some((name.to_ascii_lowercase(), *value))
            }
            _ => {
                return Err(format!(
                    "{LABEL} instance parameter must be absent or one direct finite scalar"
                ));
            }
        };

        let [resistor] = subcircuit.elements.as_slice() else {
            return Err(format!("{LABEL} subcircuit must contain one resistor"));
        };
        if resistor.nodes != subcircuit.ports
            || resistor.nodes.iter().any(|node| node.trim().is_empty())
        {
            return Err(format!(
                "{LABEL} resistor must connect the two formal ports in order"
            ));
        }
        let (expression_name, literal_value) = match &resistor.kind {
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } if model.is_none() && instance_params.is_empty() && deferred_params.is_empty() => {
                match value_expr {
                    Some(expression) => (
                        Some(
                            Self::bare_subckt_parameter_expression_name(expression).ok_or_else(
                                || {
                                    format!(
                                        "{LABEL} parameterized resistor must use one bare scalar reference"
                                    )
                                },
                            )?,
                        ),
                        None,
                    ),
                    None if value.is_finite() && *value > 0.0 => (None, Some(*value)),
                    _ => {
                        return Err(format!(
                            "{LABEL} resistor value must be one bare parameter reference or one positive finite literal"
                        ));
                    }
                }
            }
            _ => {
                return Err(format!(
                    "{LABEL} subcircuit body contains an unqualified resistor"
                ));
            }
        };

        let mut binding_names = Vec::new();
        binding_names.extend(global.as_ref().map(|(name, _)| name.as_str()));
        binding_names.extend(formal.as_ref().map(|(name, _)| name.as_str()));
        binding_names.extend(instance.as_ref().map(|(name, _)| name.as_str()));
        binding_names.extend(expression_name.as_deref());
        let parameter_name = binding_names
            .first()
            .ok_or_else(|| format!("{LABEL} contains no parameter binding or reference"))?
            .to_string();
        if binding_names
            .iter()
            .any(|name| !name.eq_ignore_ascii_case(&parameter_name))
        {
            return Err(format!(
                "{LABEL} formal, global, instance, and resistor names must identify one scalar parameter"
            ));
        }

        let representation = match (
            global.as_ref(),
            formal.as_ref(),
            instance.as_ref(),
            expression_name.as_ref(),
            literal_value,
        ) {
            (None, Some((_, default)), Some((_, override_value)), Some(_), None)
                if default.to_bits() != override_value.to_bits() =>
            {
                XyceSubcktParameterResolutionRepresentation::FormalDefaultAndInstanceOverride
            }
            (None, None, Some(_), Some(_), None) => {
                XyceSubcktParameterResolutionRepresentation::ImplicitInstanceBinding
            }
            (Some(_), None, None, Some(_), None) => {
                XyceSubcktParameterResolutionRepresentation::GlobalBinding
            }
            (Some((_, global_value)), None, Some((_, instance_value)), Some(_), None)
                if global_value.to_bits() != instance_value.to_bits() =>
            {
                XyceSubcktParameterResolutionRepresentation::InstanceOverridesGlobal
            }
            (Some((_, global_value)), None, Some((_, instance_value)), None, Some(literal))
                if global_value.to_bits() != instance_value.to_bits()
                    && literal.to_bits() == instance_value.to_bits() =>
            {
                XyceSubcktParameterResolutionRepresentation::UnusedInstanceBinding
            }
            (None, None, None, Some(_), None) => {
                XyceSubcktParameterResolutionRepresentation::UndefinedBinding
            }
            _ => {
                return Err(format!(
                    "{LABEL} source does not exercise one recognized parameter-resolution precedence mode"
                ));
            }
        };

        let [voltage_probe_text, current_probe_text] = print.probes.as_slice() else {
            return Err(format!("{LABEL} requires exactly two ordered probes"));
        };
        let voltage_probe = Self::parse_voltage_probe(voltage_probe_text)
            .ok_or_else(|| format!("{LABEL} first output is not a voltage probe"))?;
        let current_probe = Self::parse_current_probe(current_probe_text)
            .ok_or_else(|| format!("{LABEL} second output is not a branch-current probe"))?;
        if voltage_probe.accessor != XyceVoltageAccessor::Value
            || voltage_probe.node_neg.is_some()
            || !voltage_probe
                .node_pos
                .eq_ignore_ascii_case(&instance_element.nodes[0])
            || !current_probe.eq_ignore_ascii_case(&voltage_source.name)
        {
            return Err(format!(
                "{LABEL} outputs must observe the driven two-port node and swept-source current"
            ));
        }

        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist);
        if representation == XyceSubcktParameterResolutionRepresentation::UndefinedBinding {
            match flattened {
                Err(ParseError::UndefinedParameter(name))
                    if name.eq_ignore_ascii_case(&parameter_name) => {}
                Err(other) => {
                    return Err(format!(
                        "{LABEL} undefined binding must fail with typed undefined parameter '{parameter_name}', got {other}"
                    ));
                }
                Ok(_) => {
                    return Err(format!(
                        "{LABEL} undefined binding unexpectedly flattened without an error"
                    ));
                }
            }
            return Ok((representation, parameter_name, None));
        }

        let flattened = flattened.map_err(|error| {
            format!("{LABEL} valid representation failed during flattening: {error}")
        })?;
        if !flattened.scoped_models.is_empty()
            || !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
            || flattened.elements.len() != 2
        {
            return Err(format!(
                "{LABEL} valid representation must flatten to exactly one resistor and one source without scoped auxiliary state"
            ));
        }
        let mut resistor_count = 0usize;
        let mut source_count = 0usize;
        let mut fingerprints = Vec::new();
        for element in &flattened.elements {
            if element.nodes.len() != 2
                || element.nodes.iter().any(|node| node.trim().is_empty())
                || element
                    .nodes
                    .iter()
                    .any(|node| Self::xyce_ground_alias_name(node) && node.trim() != "0")
            {
                return Err(format!(
                    "{LABEL} flattened element '{}' has unqualified nodes",
                    element.name
                ));
            }
            match &element.kind {
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } if value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty() =>
                {
                    resistor_count += 1;
                }
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.is_finite() =>
                {
                    source_count += 1;
                }
                _ => {
                    return Err(format!(
                        "{LABEL} flattened element '{}' is not a direct native resistor/source",
                        element.name
                    ));
                }
            }
            let fingerprint = Self::scoped_model_element_fingerprint(element, &netlist.params)?;
            fingerprints.push(fingerprint);
        }
        if resistor_count != 1 || source_count != 1 {
            return Err(format!(
                "{LABEL} must flatten to one resistor and one independent source"
            ));
        }
        fingerprints.sort();
        Ok((
            representation,
            parameter_name.clone(),
            Some(XyceSubcktParameterResolutionSnapshot {
                representation,
                parameter_name,
                flattened_elements: fingerprints,
            }),
        ))
    }

    pub(super) fn ac_analysis_source_qualification(
        source: &str,
    ) -> Result<(XyceAcAnalysisRepresentation, BTreeMap<String, u64>), String> {
        const LABEL: &str = "AC-analysis expression parity";
        if Self::source_has_comp_directive(source) {
            return Err(format!("{LABEL} does not admit *COMP directives"));
        }
        let lines = Self::logical_netlist_lines(source);
        if lines.is_empty() {
            return Err(format!("{LABEL} requires a nonempty source"));
        }
        let mut parameter_bits = BTreeMap::new();
        let mut ac_fields = None;
        let mut print_count = 0usize;
        let mut option_count = 0usize;
        let mut end_count = 0usize;
        let mut element_count = 0usize;
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".param" => {
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "AC-analysis .PARAM statement",
                        )?;
                        let [_, assignment] = fields.as_slice() else {
                            return Err(format!(
                                "{LABEL} requires one direct '.PARAM name=value' assignment per statement"
                            ));
                        };
                        let Some((name, value)) = assignment.split_once('=') else {
                            return Err(format!(
                                "{LABEL} requires canonical '.PARAM name=value' syntax"
                            ));
                        };
                        let name = name.trim().to_ascii_lowercase();
                        if name.is_empty()
                            || value.trim().is_empty()
                            || value.contains('=')
                            || !name.chars().enumerate().all(|(index, ch)| {
                                ch.is_ascii_alphanumeric()
                                    || ch == '_'
                                    || (index > 0 && matches!(ch, '.' | '$'))
                            })
                            || name.chars().next().is_some_and(|ch| ch.is_ascii_digit())
                        {
                            return Err(format!(
                                "{LABEL} contains invalid parameter assignment '{assignment}'"
                            ));
                        }
                        let value = Self::single_spice_numeric_literal_value(value)?;
                        if !value.is_finite()
                            || parameter_bits
                                .insert(name.clone(), value.to_bits())
                                .is_some()
                        {
                            return Err(format!(
                                "{LABEL} parameter '{name}' must be unique and finite"
                            ));
                        }
                    }
                    ".ac" => {
                        if ac_fields.is_some() {
                            return Err(format!("{LABEL} requires exactly one .AC"));
                        }
                        ac_fields = Some(Self::ac_analysis_value_fields(stripped)?);
                    }
                    ".print" => {
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "AC-analysis .PRINT statement",
                        )?;
                        if fields.len() < 3
                            || !fields[0].eq_ignore_ascii_case(".PRINT")
                            || !fields[1].eq_ignore_ascii_case("AC")
                        {
                            return Err(format!(
                                "{LABEL} requires one nonempty '.PRINT AC probe ...' statement"
                            ));
                        }
                        if fields.iter().skip(2).any(|field| {
                            field.contains('=') || field.eq_ignore_ascii_case("NOINDEX")
                        }) {
                            return Err(format!(
                                "{LABEL} requires default indexed AC PRN formatting without .PRINT options"
                            ));
                        }
                        print_count += 1;
                    }
                    ".options" => {
                        option_count += 1;
                        if !Self::ac_analysis_output_option_is_footer_suppression(stripped)? {
                            return Err(format!(
                                "{LABEL} admits only the data-neutral '.OPTIONS OUTPUT PRINTFOOTER=false' setting"
                            ));
                        }
                    }
                    ".end" => {
                        end_count += 1;
                        if !stripped.eq_ignore_ascii_case(".end") {
                            return Err(format!("{LABEL} requires a bare .END statement"));
                        }
                    }
                    other => return Err(format!("{LABEL} does not admit directive '{other}'")),
                }
                continue;
            }

            element_count += 1;
            if stripped.contains('{') || stripped.contains('}') {
                return Err(format!(
                    "{LABEL} element statements must not consume analysis parameters"
                ));
            }
            let fields =
                Self::split_grouped_whitespace_fields(stripped, "AC-analysis element statement")?;
            let designator = fields
                .first()
                .and_then(|field| field.chars().next())
                .map(|ch| ch.to_ascii_uppercase())
                .ok_or_else(|| format!("{LABEL} contains an empty element statement"))?;
            match designator {
                'R' | 'C'
                    if fields.len() == 4 && Self::is_single_spice_numeric_literal(&fields[3]) => {}
                'V' | 'I' if fields.len() >= 4 => {}
                _ => {
                    return Err(format!(
                        "{LABEL} admits only direct R/C passives and independent V/I sources; got '{command}'"
                    ));
                }
            }
        }
        if print_count != 1 || option_count > 1 || end_count != 1 || element_count == 0 {
            return Err(format!(
                "{LABEL} requires one .PRINT, at most one footer-only .OPTIONS, one .END, and at least one element; found ({print_count}, {option_count}, {end_count}, {element_count})"
            ));
        }
        let ac_fields = ac_fields.ok_or_else(|| format!("{LABEL} contains no .AC"))?;
        if parameter_bits.is_empty() {
            if ac_fields
                .iter()
                .all(|field| Self::is_single_spice_numeric_literal(field))
            {
                return Ok((XyceAcAnalysisRepresentation::DirectNumeric, parameter_bits));
            }
            return Err(format!(
                "{LABEL} numeric baseline requires only direct finite .AC literals"
            ));
        }

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.eq_ignore_ascii_case(".param") || command.eq_ignore_ascii_case(".ac") {
                continue;
            }
            let identifiers = stripped
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$')))
                .filter(|field| {
                    field
                        .chars()
                        .next()
                        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
                });
            if identifiers.into_iter().any(|identifier| {
                parameter_bits
                    .keys()
                    .any(|name| identifier.eq_ignore_ascii_case(name))
            }) {
                return Err(format!(
                    "{LABEL} analysis parameters may be referenced only by .AC fields"
                ));
            }
        }
        if ac_fields.len() != parameter_bits.len() {
            return Err(format!(
                "{LABEL} requires one unique scalar parameter per .AC value field"
            ));
        }
        let mut field_parameters = BTreeSet::new();
        for field in &ac_fields {
            let Some(expression) = field
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
            else {
                return Err(format!(
                    "{LABEL} parameterized .AC fields must each be one braced expression"
                ));
            };
            let expression = rspice_core::netlist::expr::parse_expression(expression)
                .map_err(|err| format!("{LABEL} could not parse '{field}': {err}"))?;
            let mut references = BTreeMap::new();
            Self::collect_analysis_parameter_references(
                &expression,
                &parameter_bits,
                &mut references,
                ".AC",
            )?;
            let reference_entries = references.into_iter().collect::<Vec<_>>();
            let [(reference, count)] = reference_entries.as_slice() else {
                return Err(format!(
                    "{LABEL} each .AC expression must exercise exactly one scalar parameter"
                ));
            };
            if *count != 1 || !field_parameters.insert(reference.clone()) {
                return Err(format!(
                    "{LABEL} each scalar parameter must occur exactly once in exactly one .AC field"
                ));
            }
        }
        if field_parameters != parameter_bits.keys().cloned().collect() {
            return Err(format!(
                "{LABEL} every declared scalar parameter must feed exactly one .AC field"
            ));
        }
        Ok((
            XyceAcAnalysisRepresentation::ParameterExpression,
            parameter_bits,
        ))
    }

    pub(super) fn delimited_expression_source_qualification(
        source: &str,
    ) -> Result<
        (
            XyceDelimitedExpressionRepresentation,
            BTreeMap<String, XyceExpressionAstFingerprint>,
        ),
        String,
    > {
        const LABEL: &str = "delimited-expression parity";
        if Self::source_has_comp_directive(source) {
            return Err(format!("{LABEL} does not admit *COMP"));
        }
        let lines = Self::logical_netlist_lines(source);
        if lines.is_empty() || lines[0].trim().is_empty() {
            return Err(format!("{LABEL} requires one nonempty title line"));
        }
        let mut representation = None;
        let mut sites = BTreeMap::new();
        let mut param_count = 0usize;
        let mut resistor_count = 0usize;
        let mut voltage_count = 0usize;
        let mut dc_count = 0usize;
        let mut print_count = 0usize;
        let mut print_expression_count = 0usize;
        let mut end_count = 0usize;

        let mut record_site = |key: String, token: &str| -> Result<(), String> {
            let (site_representation, inner) = Self::delimited_expression_token(token)?;
            if representation
                .replace(site_representation)
                .is_some_and(|current| current != site_representation)
            {
                return Err(format!(
                    "{LABEL} member mixes braced and single-quoted expression sites"
                ));
            }
            let fingerprint = Self::parse_expression_fingerprint(inner)?;
            if sites.insert(key.clone(), fingerprint).is_some() {
                return Err(format!(
                    "{LABEL} contains duplicate expression site '{key}'"
                ));
            }
            Ok(())
        };

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            if stripped.is_empty() {
                continue;
            }
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".param" => {
                        param_count += 1;
                        let rest = stripped[command.len()..].trim();
                        let Some((name, value)) = rest.split_once('=') else {
                            return Err(format!(
                                "{LABEL} requires canonical '.PARAM name = expression' syntax"
                            ));
                        };
                        let name = name.trim().to_ascii_lowercase();
                        if name.is_empty()
                            || value.contains('=')
                            || !Self::is_single_spice_identifier(&name)
                        {
                            return Err(format!("{LABEL} contains an invalid .PARAM assignment"));
                        }
                        record_site(format!("param:{name}"), value.trim())?;
                    }
                    ".dc" => {
                        dc_count += 1;
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "delimited-expression .DC statement",
                        )?;
                        if fields.len() != 5
                            || fields[1..]
                                .iter()
                                .any(|field| field.contains(['{', '}', '\'']))
                        {
                            return Err(format!(
                                "{LABEL} requires one direct linear '.DC source start stop step'"
                            ));
                        }
                    }
                    ".print" => {
                        print_count += 1;
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "delimited-expression .PRINT statement",
                        )?;
                        if fields.len() != 6
                            || !fields[1].eq_ignore_ascii_case("DC")
                            || fields[2..].iter().any(|field| field.contains('='))
                        {
                            return Err(format!(
                                "{LABEL} requires one default '.PRINT DC' with four probes"
                            ));
                        }
                        for (index, field) in fields[2..].iter().enumerate() {
                            if field.starts_with(['{', '\'']) || field.ends_with(['}', '\'']) {
                                record_site(format!("print:{index}"), field)?;
                                print_expression_count += 1;
                            } else if field.contains(['{', '}', '\'']) {
                                return Err(format!(
                                    "{LABEL} contains a malformed .PRINT expression token"
                                ));
                            }
                        }
                    }
                    ".end" => {
                        end_count += 1;
                        if !stripped.eq_ignore_ascii_case(".end") {
                            return Err(format!("{LABEL} requires a bare .END"));
                        }
                    }
                    other => {
                        return Err(format!("{LABEL} does not admit directive '{other}'"));
                    }
                }
                continue;
            }

            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "delimited-expression element statement",
            )?;
            if fields.len() != 4 {
                return Err(format!(
                    "{LABEL} admits only canonical two-terminal resistor and voltage-source lines"
                ));
            }
            let name = fields[0].trim().to_ascii_lowercase();
            match name.chars().next().map(|ch| ch.to_ascii_uppercase()) {
                Some('R') => {
                    resistor_count += 1;
                    record_site(format!("element:{name}:value"), &fields[3])?;
                }
                Some('V') => {
                    voltage_count += 1;
                    if fields[3].contains(['{', '}', '\'']) {
                        return Err(format!(
                            "{LABEL} voltage source must use a direct finite DC value"
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "{LABEL} admits only one native resistor and one independent voltage source"
                    ));
                }
            }
        }

        if param_count != 1
            || resistor_count != 1
            || voltage_count != 1
            || dc_count != 1
            || print_count != 1
            || print_expression_count != 2
            || end_count != 1
            || sites.len() != 4
        {
            return Err(format!(
                "{LABEL} requires exactly one parameter, resistor, voltage source, DC, PRINT, END and four expression sites"
            ));
        }
        Ok((
            representation.ok_or_else(|| format!("{LABEL} contains no expression sites"))?,
            sites,
        ))
    }

    pub(super) fn dc_analysis_source_qualification(
        source: &str,
    ) -> Result<(XyceDcAnalysisRepresentation, BTreeMap<String, u64>), String> {
        const LABEL: &str = "DC-analysis expression parity";
        if Self::source_has_comp_directive(source) {
            return Err(format!(
                "{LABEL} uses the canonical default verifier and does not admit *COMP"
            ));
        }
        let lines = Self::logical_netlist_lines(source);
        if lines.is_empty() {
            return Err(format!("{LABEL} requires a nonempty source"));
        }
        let mut parameter_bits = BTreeMap::new();
        let mut dc_fields = None;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        let mut model_count = 0usize;
        let mut element_count = 0usize;
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".param" => {
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "DC-analysis .PARAM statement",
                        )?;
                        let [_, assignment] = fields.as_slice() else {
                            return Err(format!(
                                "{LABEL} requires one direct '.PARAM name=value' assignment per statement"
                            ));
                        };
                        let Some((name, value)) = assignment.split_once('=') else {
                            return Err(format!(
                                "{LABEL} requires canonical '.PARAM name=value' syntax"
                            ));
                        };
                        let name = name.trim().to_ascii_lowercase();
                        if name.is_empty()
                            || value.trim().is_empty()
                            || value.contains('=')
                            || !name.chars().enumerate().all(|(index, ch)| {
                                ch.is_ascii_alphanumeric()
                                    || ch == '_'
                                    || (index > 0 && matches!(ch, '.' | '$'))
                            })
                            || name.chars().next().is_some_and(|ch| ch.is_ascii_digit())
                        {
                            return Err(format!(
                                "{LABEL} contains invalid parameter assignment '{assignment}'"
                            ));
                        }
                        let value = Self::single_spice_numeric_literal_value(value)?;
                        if !value.is_finite()
                            || parameter_bits
                                .insert(name.clone(), value.to_bits())
                                .is_some()
                        {
                            return Err(format!(
                                "{LABEL} parameter '{name}' must be unique and finite"
                            ));
                        }
                    }
                    ".dc" => {
                        if dc_fields.is_some() {
                            return Err(format!("{LABEL} requires exactly one .DC"));
                        }
                        dc_fields = Some(Self::dc_analysis_value_fields(stripped)?);
                    }
                    ".print" => {
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "DC-analysis .PRINT statement",
                        )?;
                        if fields.len() < 3
                            || !fields[0].eq_ignore_ascii_case(".PRINT")
                            || !fields[1].eq_ignore_ascii_case("DC")
                        {
                            return Err(format!(
                                "{LABEL} requires a nonempty '.PRINT DC probe ...' statement"
                            ));
                        }
                        print_count += 1;
                    }
                    ".model" => model_count += 1,
                    ".end" => {
                        end_count += 1;
                        if !stripped.eq_ignore_ascii_case(".end") {
                            return Err(format!("{LABEL} requires a bare .END statement"));
                        }
                    }
                    other => {
                        return Err(format!("{LABEL} does not admit directive '{other}'"));
                    }
                }
                continue;
            }

            element_count += 1;
            if stripped.contains('{') || stripped.contains('}') {
                return Err(format!(
                    "{LABEL} element statements must not consume analysis parameters"
                ));
            }
            let designator = command
                .chars()
                .next()
                .map(|ch| ch.to_ascii_uppercase())
                .ok_or_else(|| format!("{LABEL} contains an empty element statement"))?;
            if !matches!(designator, 'R' | 'V' | 'M') {
                return Err(format!(
                    "{LABEL} admits only native resistors, independent voltage sources, and classic MOS elements; got '{command}'"
                ));
            }
        }
        if print_count != 1 || end_count != 1 || element_count == 0 || model_count > 1 {
            return Err(format!(
                "{LABEL} requires one .PRINT, one .END, at least one element, and at most one model; found ({print_count}, {end_count}, {element_count}, {model_count})"
            ));
        }
        let dc_fields = dc_fields.ok_or_else(|| format!("{LABEL} contains no .DC"))?;

        if parameter_bits.is_empty() {
            if dc_fields
                .iter()
                .all(|field| Self::is_single_spice_numeric_literal(field))
            {
                return Ok((XyceDcAnalysisRepresentation::DirectNumeric, parameter_bits));
            }
            return Err(format!(
                "{LABEL} numeric baseline requires only direct finite .DC literals"
            ));
        }

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.eq_ignore_ascii_case(".param") || command.eq_ignore_ascii_case(".dc") {
                continue;
            }
            let identifiers = stripped
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$')))
                .filter(|field| {
                    field
                        .chars()
                        .next()
                        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
                });
            if identifiers.into_iter().any(|identifier| {
                parameter_bits
                    .keys()
                    .any(|name| identifier.eq_ignore_ascii_case(name))
            }) {
                return Err(format!(
                    "{LABEL} analysis parameters may be referenced only by .DC fields"
                ));
            }
        }

        if dc_fields.len() != parameter_bits.len() {
            return Err(format!(
                "{LABEL} requires one unique scalar parameter per .DC value field"
            ));
        }
        let mut field_parameters = BTreeSet::new();
        for field in &dc_fields {
            let Some(expression) = field
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
            else {
                return Err(format!(
                    "{LABEL} parameterized .DC fields must each be one braced expression"
                ));
            };
            let expression = rspice_core::netlist::expr::parse_expression(expression)
                .map_err(|err| format!("{LABEL} could not parse '{field}': {err}"))?;
            let mut references = BTreeMap::new();
            Self::collect_analysis_parameter_references(
                &expression,
                &parameter_bits,
                &mut references,
                ".DC",
            )?;
            let reference_entries = references.into_iter().collect::<Vec<_>>();
            let [(reference, count)] = reference_entries.as_slice() else {
                return Err(format!(
                    "{LABEL} each .DC expression must exercise exactly one scalar parameter"
                ));
            };
            if *count != 1 || !field_parameters.insert(reference.clone()) {
                return Err(format!(
                    "{LABEL} each scalar parameter must occur exactly once in exactly one .DC field"
                ));
            }
        }
        if field_parameters != parameter_bits.keys().cloned().collect() {
            return Err(format!(
                "{LABEL} every declared scalar parameter must feed exactly one .DC field"
            ));
        }
        Ok((
            XyceDcAnalysisRepresentation::ParameterExpression,
            parameter_bits,
        ))
    }

    pub(super) fn transient_analysis_source_qualification(
        source: &str,
    ) -> Result<(XyceTransientAnalysisRepresentation, BTreeMap<String, u64>), String> {
        const LABEL: &str = "transient-analysis expression parity";
        if Self::source_has_comp_directive(source) {
            return Err(format!(
                "{LABEL} uses canonical default xyce_verify tolerances and does not admit *COMP"
            ));
        }
        let lines = Self::logical_netlist_lines(source);
        if lines.is_empty() {
            return Err(format!("{LABEL} requires a nonempty source"));
        }
        let mut parameter_bits = BTreeMap::new();
        let mut tran_fields = None;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        let mut element_count = 0usize;
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".param" => {
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "transient-analysis .PARAM statement",
                        )?;
                        let [_, assignment] = fields.as_slice() else {
                            return Err(format!(
                                "{LABEL} requires one direct '.PARAM name=value' assignment per statement"
                            ));
                        };
                        let Some((name, value)) = assignment.split_once('=') else {
                            return Err(format!(
                                "{LABEL} requires canonical '.PARAM name=value' syntax"
                            ));
                        };
                        let name = name.trim().to_ascii_lowercase();
                        if name.is_empty()
                            || value.trim().is_empty()
                            || value.contains('=')
                            || !name.chars().enumerate().all(|(index, ch)| {
                                ch.is_ascii_alphanumeric()
                                    || ch == '_'
                                    || (index > 0 && matches!(ch, '.' | '$'))
                            })
                            || name.chars().next().is_some_and(|ch| ch.is_ascii_digit())
                        {
                            return Err(format!(
                                "{LABEL} contains invalid parameter assignment '{assignment}'"
                            ));
                        }
                        let value = Self::single_spice_numeric_literal_value(value)?;
                        if !value.is_finite()
                            || parameter_bits
                                .insert(name.clone(), value.to_bits())
                                .is_some()
                        {
                            return Err(format!(
                                "{LABEL} parameter '{name}' must be unique and finite"
                            ));
                        }
                    }
                    ".tran" => {
                        if tran_fields.is_some() {
                            return Err(format!("{LABEL} requires exactly one .TRAN"));
                        }
                        let fields = Self::split_grouped_whitespace_fields(
                            stripped,
                            "transient-analysis .TRAN statement",
                        )?;
                        if !(3..=5).contains(&fields.len()) {
                            return Err(format!("{LABEL} requires two to four .TRAN value fields"));
                        }
                        tran_fields = Some(fields.into_iter().skip(1).collect::<Vec<_>>());
                    }
                    ".print" => print_count += 1,
                    ".options" => {}
                    ".end" => {
                        end_count += 1;
                        if !stripped.eq_ignore_ascii_case(".end") {
                            return Err(format!("{LABEL} requires a bare .END statement"));
                        }
                    }
                    other => {
                        return Err(format!("{LABEL} does not admit directive '{other}'"));
                    }
                }
                continue;
            }

            element_count += 1;
            if stripped.contains('{') || stripped.contains('}') {
                return Err(format!(
                    "{LABEL} element statements must not consume analysis parameters"
                ));
            }
            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "transient-analysis element statement",
            )?;
            let designator = fields
                .first()
                .and_then(|field| field.chars().next())
                .map(|ch| ch.to_ascii_uppercase())
                .ok_or_else(|| format!("{LABEL} contains an empty element statement"))?;
            match designator {
                'R' if fields.len() == 4 && Self::is_single_spice_numeric_literal(&fields[3]) => {}
                'C' if (4..=5).contains(&fields.len())
                    && Self::is_single_spice_numeric_literal(&fields[3]) =>
                {
                    if let Some(initial) = fields.get(4) {
                        let Some((name, value)) = initial.split_once('=') else {
                            return Err(format!(
                                "{LABEL} capacitor optional field must be direct IC=value"
                            ));
                        };
                        if !name.eq_ignore_ascii_case("IC")
                            || !Self::is_single_spice_numeric_literal(value)
                        {
                            return Err(format!(
                                "{LABEL} capacitor optional field must be direct finite IC=value"
                            ));
                        }
                    }
                }
                'V' | 'I' if fields.len() == 4 => {}
                _ => {
                    return Err(format!(
                        "{LABEL} admits only direct R/C and independent V/I element statements; got '{command}'"
                    ));
                }
            }
        }
        if print_count != 1 || end_count != 1 || element_count == 0 {
            return Err(format!(
                "{LABEL} requires one .PRINT, one .END, and at least one element; found ({print_count}, {end_count}, {element_count})"
            ));
        }
        let tran_fields = tran_fields.ok_or_else(|| format!("{LABEL} contains no .TRAN"))?;

        if parameter_bits.is_empty() {
            if tran_fields
                .iter()
                .all(|field| Self::is_single_spice_numeric_literal(field))
            {
                return Ok((
                    XyceTransientAnalysisRepresentation::DirectNumeric,
                    parameter_bits,
                ));
            }
            return Err(format!(
                "{LABEL} numeric baseline requires only direct finite .TRAN literals"
            ));
        }

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.eq_ignore_ascii_case(".param") || command.eq_ignore_ascii_case(".tran") {
                continue;
            }
            let identifiers = stripped
                .split(|ch: char| !(ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$')))
                .filter(|field| {
                    field
                        .chars()
                        .next()
                        .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_')
                });
            if identifiers.into_iter().any(|identifier| {
                parameter_bits
                    .keys()
                    .any(|name| identifier.eq_ignore_ascii_case(name))
            }) {
                return Err(format!(
                    "{LABEL} analysis parameters may be referenced only by .TRAN fields"
                ));
            }
        }

        if tran_fields.len() != parameter_bits.len() {
            return Err(format!(
                "{LABEL} requires one unique scalar parameter per .TRAN field"
            ));
        }
        let mut field_parameters = BTreeSet::new();
        for field in &tran_fields {
            let Some(expression) = field
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
            else {
                return Err(format!(
                    "{LABEL} parameterized .TRAN fields must each be one braced expression"
                ));
            };
            let expression = rspice_core::netlist::expr::parse_expression(expression)
                .map_err(|err| format!("{LABEL} could not parse '{field}': {err}"))?;
            let mut references = BTreeMap::new();
            Self::collect_analysis_parameter_references(
                &expression,
                &parameter_bits,
                &mut references,
                ".TRAN",
            )?;
            let reference_entries = references.into_iter().collect::<Vec<_>>();
            let [(reference, count)] = reference_entries.as_slice() else {
                return Err(format!(
                    "{LABEL} each .TRAN expression must exercise exactly one scalar parameter"
                ));
            };
            if *count != 1 {
                return Err(format!(
                    "{LABEL} each scalar parameter must occur exactly once in its .TRAN field"
                ));
            }
            if !field_parameters.insert(reference.clone()) {
                return Err(format!(
                    "{LABEL} each scalar parameter must belong to exactly one .TRAN field"
                ));
            }
        }
        if field_parameters != parameter_bits.keys().cloned().collect() {
            return Err(format!(
                "{LABEL} every declared scalar parameter must feed exactly one .TRAN field"
            ));
        }
        Ok((
            XyceTransientAnalysisRepresentation::ParameterExpression,
            parameter_bits,
        ))
    }

    pub(super) fn diode_model_alias_source_qualification(
        source: &str,
    ) -> Result<(XyceDiodeModelAliasRepresentation, String), String> {
        const LABEL: &str = "native diode model-parameter alias equivalence";
        const CANONICAL_ORDER: [&str; 12] = [
            "IS", "N", "BV", "IBV", "RS", "CJO", "VJ", "M", "FC", "EG", "XTI", "TT",
        ];
        let alias_canonical = |name: &str| match name.to_ascii_uppercase().as_str() {
            "IS" | "JS" => Some("IS"),
            "BV" | "VB" => Some("BV"),
            "CJO" | "CJ" => Some("CJO"),
            _ => None,
        };
        let mut assignments = Vec::<(String, String)>::new();
        let mut canonical_source = String::with_capacity(source.len());
        let mut in_model = false;
        let mut model_headers = 0usize;
        let mut comp_lines = 0usize;

        for raw_line in source.split_inclusive('\n') {
            let newline = if raw_line.ends_with('\n') { "\n" } else { "" };
            let physical = raw_line
                .strip_suffix('\n')
                .unwrap_or(raw_line)
                .strip_suffix('\r')
                .unwrap_or_else(|| raw_line.strip_suffix('\n').unwrap_or(raw_line));
            let carriage = if raw_line
                .strip_suffix('\n')
                .is_some_and(|line| line.ends_with('\r'))
            {
                "\r"
            } else {
                ""
            };
            let comment_at = physical.find(';').unwrap_or(physical.len());
            let active = &physical[..comment_at];
            let suffix = &physical[comment_at..];
            let trimmed = active.trim_start();
            if trimmed.to_ascii_uppercase().starts_with("*COMP") {
                comp_lines += 1;
                let fields = trimmed.split_whitespace().collect::<Vec<_>>();
                if fields.len() != 4
                    || !fields[0].eq_ignore_ascii_case("*COMP")
                    || !fields[2].to_ascii_lowercase().starts_with("reltol=")
                    || !fields[3].to_ascii_lowercase().starts_with("abstol=")
                    || Self::single_spice_numeric_literal_value(
                        fields[2]
                            .split_once('=')
                            .map(|(_, value)| value)
                            .unwrap_or(""),
                    )
                    .is_err()
                    || Self::single_spice_numeric_literal_value(
                        fields[3]
                            .split_once('=')
                            .map(|(_, value)| value)
                            .unwrap_or(""),
                    )
                    .is_err()
                {
                    return Err(format!(
                        "{LABEL} requires scalar reltol/abstol assignments on each *COMP line"
                    ));
                }
            }
            let is_comment =
                trimmed.starts_with('*') || trimmed.starts_with("//") || trimmed.is_empty();
            let model_start = trimmed
                .split_whitespace()
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case(".MODEL"));
            let continuation = trimmed.starts_with('+');
            if model_start {
                model_headers += 1;
                in_model = true;
                let header = trimmed.split_whitespace().take(3).collect::<Vec<_>>();
                if header.len() != 3 || !header[2].eq_ignore_ascii_case("D") {
                    return Err(format!("{LABEL} requires one plain native D model"));
                }
            } else if !continuation && !is_comment {
                in_model = false;
            }

            if !in_model || is_comment {
                canonical_source.push_str(physical);
                canonical_source.push_str(carriage);
                canonical_source.push_str(newline);
                continue;
            }

            let bytes = active.as_bytes();
            let mut index = 0usize;
            let mut copied = 0usize;
            while index < bytes.len() {
                if !(bytes[index].is_ascii_alphabetic() || bytes[index] == b'_') {
                    index += 1;
                    continue;
                }
                let start = index;
                index += 1;
                while index < bytes.len()
                    && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
                {
                    index += 1;
                }
                let end = index;
                let mut equals = end;
                while equals < bytes.len() && bytes[equals].is_ascii_whitespace() {
                    equals += 1;
                }
                if equals >= bytes.len() || bytes[equals] != b'=' {
                    continue;
                }
                let mut value_start = equals + 1;
                while value_start < bytes.len() && bytes[value_start].is_ascii_whitespace() {
                    value_start += 1;
                }
                let mut value_end = value_start;
                while value_end < bytes.len() && !bytes[value_end].is_ascii_whitespace() {
                    value_end += 1;
                }
                let name = &active[start..end];
                let value = &active[value_start..value_end];
                if value.is_empty() || !Self::is_single_spice_numeric_literal(value) {
                    return Err(format!(
                        "{LABEL} model parameters must be direct scalar numeric assignments"
                    ));
                }
                assignments.push((name.to_ascii_uppercase(), value.to_string()));
                if let Some(canonical) = alias_canonical(name) {
                    canonical_source.push_str(&active[copied..start]);
                    canonical_source.push_str(canonical);
                    copied = end;
                }
                index = value_end;
            }
            canonical_source.push_str(&active[copied..]);
            canonical_source.push_str(suffix);
            canonical_source.push_str(carriage);
            canonical_source.push_str(newline);
        }
        if model_headers != 1 || comp_lines != 3 || assignments.len() != CANONICAL_ORDER.len() {
            return Err(format!(
                "{LABEL} requires one model card, twelve ordered parameters, and three *COMP records"
            ));
        }
        let canonical_names = assignments
            .iter()
            .map(|(name, _)| alias_canonical(name).unwrap_or(name.as_str()))
            .collect::<Vec<_>>();
        if canonical_names != CANONICAL_ORDER {
            return Err(format!(
                "{LABEL} model parameter set/order or alias-group cardinality is ambiguous"
            ));
        }
        let alias_spellings = assignments
            .iter()
            .filter(|(name, _)| matches!(name.as_str(), "JS" | "VB" | "CJ"))
            .count();
        let canonical_spellings = assignments
            .iter()
            .filter(|(name, _)| matches!(name.as_str(), "IS" | "BV" | "CJO"))
            .count();
        let representation = match (canonical_spellings, alias_spellings) {
            (3, 0) => XyceDiodeModelAliasRepresentation::Canonical,
            (0, 3) => XyceDiodeModelAliasRepresentation::Alias,
            _ => {
                return Err(format!(
                    "{LABEL} requires one complete canonical or synonym spelling set"
                ));
            }
        };

        let mut element_counts = BTreeMap::<char, usize>::new();
        let mut directive_counts = BTreeMap::<String, usize>::new();
        let mut logical_lines = Self::logical_netlist_lines(source).into_iter();
        let title = logical_lines
            .next()
            .ok_or_else(|| format!("{LABEL} requires a circuit title"))?;
        let title = Self::strip_netlist_comment(&title).trim();
        if title.is_empty() || title.starts_with('.') || title.starts_with('*') {
            return Err(format!(
                "{LABEL} requires one ordinary non-directive circuit title"
            ));
        }
        for line in logical_lines {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.starts_with('.') {
                *directive_counts
                    .entry(command.to_ascii_lowercase())
                    .or_default() += 1;
                continue;
            }
            let designator = command
                .chars()
                .next()
                .map(|ch| ch.to_ascii_uppercase())
                .ok_or_else(|| format!("{LABEL} contains an empty element name"))?;
            if !matches!(designator, 'V' | 'D' | 'R') {
                return Err(format!("{LABEL} contains an unqualified element statement"));
            }
            *element_counts.entry(designator).or_default() += 1;
        }
        if element_counts != BTreeMap::from([('D', 1), ('R', 1), ('V', 1)])
            || directive_counts
                != BTreeMap::from([
                    (".end".to_string(), 1),
                    (".model".to_string(), 1),
                    (".print".to_string(), 1),
                    (".tran".to_string(), 1),
                ])
        {
            return Err(format!(
                "{LABEL} requires exactly one V, D, R, MODEL, PRINT, TRAN, and END"
            ));
        }
        Ok((representation, canonical_source))
    }

    pub(super) fn switch_state_case_source_qualification(
        source: &str,
    ) -> Result<(XyceSwitchStateCaseRepresentation, String), String> {
        const LABEL: &str = "generic-switch initial-state case equivalence";
        let mut canonical_source = source.to_string();
        let mut state_replacement = None::<(std::ops::Range<usize>, String)>;
        let mut byte_offset = 0usize;
        for raw_line in source.split_inclusive('\n') {
            let physical = raw_line
                .strip_suffix('\n')
                .unwrap_or(raw_line)
                .strip_suffix('\r')
                .unwrap_or_else(|| raw_line.strip_suffix('\n').unwrap_or(raw_line));
            let active = physical
                .split_once(';')
                .map(|(head, _)| head)
                .unwrap_or(physical);
            let trimmed = active.trim_start();
            if trimmed.starts_with('*') || trimmed.starts_with("//") || trimmed.is_empty() {
                byte_offset += raw_line.len();
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                byte_offset += raw_line.len();
                continue;
            };
            if !command.starts_with('.')
                && command
                    .chars()
                    .next()
                    .is_some_and(|ch| ch.eq_ignore_ascii_case(&'S'))
            {
                if state_replacement.is_some() || active.trim_end().ends_with('+') {
                    return Err(format!(
                        "{LABEL} requires exactly one single-line generic switch statement"
                    ));
                }
                let spans = active.char_indices().fold(
                    Vec::<std::ops::Range<usize>>::new(),
                    |mut spans, (index, ch)| {
                        if ch.is_whitespace() {
                            return spans;
                        }
                        if spans.last().is_none_or(|range| range.end != index) {
                            spans.push(index..index + ch.len_utf8());
                        } else if let Some(last) = spans.last_mut() {
                            last.end = index + ch.len_utf8();
                        }
                        spans
                    },
                );
                if spans.len() != 6 {
                    return Err(format!(
                        "{LABEL} switch must use 'Sname n+ n- model ON|OFF CONTROL={{expression}}'"
                    ));
                }
                let token = &active[spans[4].clone()];
                let canonical = match token {
                    "on" => "ON",
                    "off" => "OFF",
                    "ON" | "OFF" => token,
                    _ => {
                        return Err(format!(
                            "{LABEL} initial state must be pure lower- or uppercase ON/OFF"
                        ));
                    }
                };
                let control = &active[spans[5].clone()];
                let Some((name, expression)) = control.split_once('=') else {
                    return Err(format!("{LABEL} switch requires CONTROL={{expression}}"));
                };
                let Some(expression) = Self::print_expression_inner(expression) else {
                    return Err(format!("{LABEL} switch CONTROL must be one braced token"));
                };
                if !name.eq_ignore_ascii_case("CONTROL") || expression.trim().is_empty() {
                    return Err(format!("{LABEL} switch requires CONTROL={{expression}}"));
                }
                Self::parse_expression_fingerprint(expression)?;
                state_replacement = Some((
                    byte_offset + spans[4].start..byte_offset + spans[4].end,
                    canonical.to_string(),
                ));
            }
            byte_offset += raw_line.len();
        }
        let (range, canonical_state) = state_replacement
            .ok_or_else(|| format!("{LABEL} requires exactly one generic switch"))?;
        let original_state = canonical_source[range.clone()].to_string();
        let representation = if original_state == canonical_state {
            XyceSwitchStateCaseRepresentation::Uppercase
        } else {
            XyceSwitchStateCaseRepresentation::Lowercase
        };
        canonical_source.replace_range(range, &canonical_state);

        let mut element_counts = BTreeMap::<char, usize>::new();
        let mut directive_counts = BTreeMap::<String, usize>::new();
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            let fields = Self::split_grouped_whitespace_fields(stripped, LABEL)?;
            if command.starts_with('.') {
                let directive = command.to_ascii_lowercase();
                *directive_counts.entry(directive.clone()).or_default() += 1;
                match directive.as_str() {
                    ".model" => {
                        if fields.len() != 7 || !fields[2].eq_ignore_ascii_case("SWITCH") {
                            return Err(format!(
                                "{LABEL} requires one scalar four-parameter SWITCH model"
                            ));
                        }
                        let mut names = BTreeSet::new();
                        for assignment in &fields[3..] {
                            let Some((name, value)) = assignment.split_once('=') else {
                                return Err(format!(
                                    "{LABEL} model parameters must be assignments"
                                ));
                            };
                            let name = name.to_ascii_uppercase();
                            if !matches!(name.as_str(), "RON" | "ROFF" | "ON" | "OFF")
                                || !names.insert(name)
                                || !Self::is_single_spice_numeric_literal(value)
                            {
                                return Err(format!(
                                    "{LABEL} model requires unique numeric RON, ROFF, ON, and OFF assignments"
                                ));
                            }
                        }
                    }
                    ".tran" => {
                        if fields.len() != 4
                            || !Self::is_single_spice_numeric_literal(&fields[1])
                            || !Self::is_single_spice_numeric_literal(&fields[2])
                            || Self::single_spice_numeric_literal_value(&fields[3])
                                .ok()
                                .map(Value::to_bits)
                                != Some(0.0f64.to_bits())
                        {
                            return Err(format!(
                                "{LABEL} requires direct numeric '.TRAN step stop 0' syntax with positive zero START"
                            ));
                        }
                    }
                    ".print" => {
                        let print_fields = Self::split_print_fields(stripped)?;
                        if print_fields.len() != 4 || !print_fields[1].eq_ignore_ascii_case("TRAN")
                        {
                            return Err(format!(
                                "{LABEL} requires one default two-probe .PRINT TRAN"
                            ));
                        }
                    }
                    ".end" if stripped.eq_ignore_ascii_case(".end") => {}
                    ".end" => return Err(format!("{LABEL} requires a bare .END")),
                    other => return Err(format!("{LABEL} does not admit directive '{other}'")),
                }
                continue;
            }
            let designator = command
                .chars()
                .next()
                .map(|ch| ch.to_ascii_uppercase())
                .ok_or_else(|| format!("{LABEL} contains an empty element name"))?;
            *element_counts.entry(designator).or_default() += 1;
            match designator {
                'S' if fields.len() == 6 => {}
                'R' if fields.len() == 4 && Self::is_single_spice_numeric_literal(&fields[3]) => {}
                'V' if fields.len() == 5
                    && fields[3].eq_ignore_ascii_case("DC")
                    && Self::is_single_spice_numeric_literal(&fields[4]) => {}
                _ => return Err(format!("{LABEL} contains an unqualified element statement")),
            }
        }
        if element_counts != BTreeMap::from([('R', 1), ('S', 1), ('V', 1)])
            || directive_counts
                != BTreeMap::from([
                    (".end".to_string(), 1),
                    (".model".to_string(), 1),
                    (".print".to_string(), 1),
                    (".tran".to_string(), 1),
                ])
        {
            return Err(format!(
                "{LABEL} requires one R, V, generic switch, SWITCH model, TRAN, PRINT, and END"
            ));
        }
        Ok((representation, canonical_source))
    }

    pub(super) fn age_cap_source_qualification(
        source: &str,
    ) -> Result<XyceAgeCapRepresentation, String> {
        const LABEL: &str = "native capacitor AGE/D equivalence";
        let mut capacitor_field = None;
        let mut resistor_count = 0usize;
        let mut voltage_count = 0usize;
        let mut parameters = BTreeMap::<String, rspice_core::netlist::expr::Expr>::new();
        let mut directive_counts = BTreeMap::<String, usize>::new();
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.starts_with('.') {
                let directive = command.to_ascii_lowercase();
                *directive_counts.entry(directive.clone()).or_default() += 1;
                match directive.as_str() {
                    ".param" => {
                        let rest = stripped[command.len()..].trim();
                        let Some((name, expression)) = rest.split_once('=') else {
                            return Err(format!("{LABEL} requires one assignment per .PARAM"));
                        };
                        let name = name.trim().to_ascii_lowercase();
                        let expression = expression.trim();
                        if !Self::is_single_spice_identifier(&name)
                            || expression.is_empty()
                            || expression.contains('=')
                            || parameters.contains_key(&name)
                        {
                            return Err(format!(
                                "{LABEL} contains a malformed or duplicate .PARAM assignment"
                            ));
                        }
                        let inner = Self::print_expression_inner(expression).unwrap_or(expression);
                        let ast =
                            rspice_core::netlist::expr::parse_expression(inner).map_err(|err| {
                                format!("{LABEL} could not parse .PARAM expression: {err}")
                            })?;
                        parameters.insert(name, ast);
                    }
                    ".tran" => {
                        let fields = stripped.split_whitespace().collect::<Vec<_>>();
                        if fields.len() != 3
                            || !Self::is_single_spice_numeric_literal(fields[1])
                            || !Self::is_single_spice_numeric_literal(fields[2])
                        {
                            return Err(format!(
                                "{LABEL} requires direct '.TRAN step stop' syntax"
                            ));
                        }
                    }
                    ".options" => {
                        let fields = stripped.split_whitespace().collect::<Vec<_>>();
                        let valid = fields.len() == 3
                            && fields[1].eq_ignore_ascii_case("TIMEINT")
                            && fields[2]
                                .split_once('=')
                                .filter(|(name, _)| name.eq_ignore_ascii_case("RELTOL"))
                                .and_then(|(_, value)| {
                                    Self::single_spice_numeric_literal_value(value).ok()
                                })
                                .is_some_and(|value| value.is_finite() && value > 0.0);
                        if !valid {
                            return Err(format!(
                                "{LABEL} requires one bounded TIMEINT RELTOL option"
                            ));
                        }
                    }
                    ".print" => {
                        let fields = Self::split_print_fields(stripped)?;
                        if fields.len() != 4 || !fields[1].eq_ignore_ascii_case("TRAN") {
                            return Err(format!(
                                "{LABEL} requires one default two-probe .PRINT TRAN"
                            ));
                        }
                    }
                    ".end" if stripped.eq_ignore_ascii_case(".end") => {}
                    ".end" => return Err(format!("{LABEL} requires a bare .END")),
                    other => return Err(format!("{LABEL} does not admit directive '{other}'")),
                }
                continue;
            }
            let designator = command.chars().next().map(|ch| ch.to_ascii_uppercase());
            match designator {
                Some('C') => {
                    if capacitor_field.is_some() {
                        return Err(format!("{LABEL} requires exactly one capacitor"));
                    }
                    capacitor_field = Some(Self::split_grouped_whitespace_fields(
                        stripped,
                        "AGE/D capacitor statement",
                    )?);
                }
                Some('R') => {
                    let fields = Self::split_grouped_whitespace_fields(
                        stripped,
                        "AGE/D resistor statement",
                    )?;
                    if fields.len() != 4 || !Self::is_single_spice_numeric_literal(&fields[3]) {
                        return Err(format!(
                            "{LABEL} resistors must use direct numeric Rname n+ n- value syntax"
                        ));
                    }
                    resistor_count += 1;
                }
                Some('V') => {
                    let fields = Self::split_grouped_whitespace_fields(
                        stripped,
                        "AGE/D voltage-source statement",
                    )?;
                    if fields.len() != 4
                        || !(Self::is_single_spice_numeric_literal(&fields[3])
                            || Self::age_cap_direct_pulse_field(&fields[3]))
                    {
                        return Err(format!(
                            "{LABEL} voltage sources must use direct numeric DC or direct numeric PULSE syntax"
                        ));
                    }
                    voltage_count += 1;
                }
                _ => {}
            }
        }
        if directive_counts.get(".tran") != Some(&1)
            || directive_counts.get(".options") != Some(&1)
            || directive_counts.get(".print") != Some(&1)
            || directive_counts.get(".end") != Some(&1)
            || directive_counts
                .iter()
                .any(|(name, count)| name != ".param" && *count != 1)
            || resistor_count != 2
            || voltage_count != 2
        {
            return Err(format!(
                "{LABEL} requires exactly one .TRAN, .OPTIONS, .PRINT, and .END"
            ));
        }
        let fields = capacitor_field.ok_or_else(|| format!("{LABEL} has no capacitor"))?;
        if fields.len() == 4 && Self::print_expression_inner(&fields[3]).is_some() {
            if parameters.is_empty() {
                return Err(format!(
                    "{LABEL} expression representation requires parameters"
                ));
            }
            let cap_expression = Self::print_expression_inner(&fields[3]).expect("checked above");
            let cap_ast = rspice_core::netlist::expr::parse_expression(cap_expression)
                .map_err(|err| format!("{LABEL} could not parse capacitance expression: {err}"))?;
            Self::age_cap_parameter_graph(&parameters, &cap_ast)?;
            return Ok(XyceAgeCapRepresentation::ParameterExpression);
        }
        if fields.len() < 5 || !Self::is_single_spice_numeric_literal(&fields[3]) {
            return Err(format!(
                "{LABEL} aged representation requires a direct numeric capacitance"
            ));
        }
        if !parameters.is_empty() {
            return Err(format!(
                "{LABEL} aged representation does not admit parameters"
            ));
        }
        let mut age = None;
        let mut degradation = None;
        for assignment in &fields[4..] {
            let Some((name, value)) = assignment.split_once('=') else {
                return Err(format!("{LABEL} AGE/D fields must be named assignments"));
            };
            let parsed = Self::single_spice_numeric_literal_value(value)?;
            if name.eq_ignore_ascii_case("AGE") && age.replace(parsed).is_none() {
                continue;
            }
            if name.eq_ignore_ascii_case("D") && degradation.replace(parsed).is_none() {
                continue;
            }
            return Err(format!(
                "{LABEL} aged capacitor admits exactly one AGE and at most one D"
            ));
        }
        if age.is_none_or(|value| !value.is_finite() || value <= 1.0)
            || degradation.is_some_and(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} AGE must be finite and greater than one, and D must be finite"
            ));
        }
        Ok(XyceAgeCapRepresentation::NativeAge)
    }
}

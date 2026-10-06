//! Input-path discovery without model initialization or circuit allocation.
use super::*;
use std::path::PathBuf;

impl Engine {
    /// Discover native input candidates for XSPICE data-file models and
    /// behavioral lookup tables in sources, resistors, and capacitors.
    /// Uses the circuit builder's hierarchy and model/instance parameter
    /// resolution, including model defaults. Registered virtual inputs are
    /// excluded. Both `NGSPICE_INPUT_DIR` and fallback paths are reserved so
    /// publishing a new preferred file cannot shadow an existing fallback.
    ///
    /// Discovery does not read data files, initialize models, or advance the
    /// caller's statistical stream. Paths are sorted and deduplicated. Keep
    /// source files, environment, and virtual registrations stable until the
    /// run ends. This inventory covers model data inputs, not arbitrary files
    /// opened by external processes, libraries, or Verilog-A programs.
    pub fn data_file_candidates_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<PathBuf>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_valid_configuration()?;
        check_build_abort(abort)?;

        // Avoid hierarchy work for decks with no data-consuming element kinds.
        let mut definitions: Vec<_> = netlist.subcircuits.iter().collect();
        let mut elements = netlist.elements.as_slice();
        loop {
            let mut found = false;
            for element in elements {
                check_build_abort(abort)?;
                if matches!(element.kind, ElementKind::Xspice { .. })
                    || behavioral_data_expression(&element.kind).is_some()
                {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            let Some(definition) = definitions.pop() else {
                return Ok(Vec::new());
            };
            elements = &definition.elements;
            definitions.extend(&definition.nested_subcircuits);
        }

        let mut isolated = netlist.clone();
        isolated.params = netlist.params.isolated_random_clone();
        let flattened = flatten_netlist_with_models_config_with_abort(
            &isolated,
            FlattenerConfig {
                max_depth: engine.config.resource_limits.max_hierarchy_depth,
                max_elements: engine.config.resource_limits.max_flattened_elements,
                ..FlattenerConfig::default()
            },
            abort,
        )
        .map_err(|error| map_build_parse_error("data-file dependency discovery", error))?;
        isolated.models.extend(flattened.scoped_models);
        let registry = OnceLock::new();
        let mut paths = BTreeSet::new();
        for element in flattened.elements {
            check_build_abort(abort)?;
            if let Some(expression) = behavioral_data_expression(&element.kind) {
                collect_behavioral_data_inputs(&isolated, &element, expression, abort, &mut paths)?;
            }
            let ElementKind::Xspice {
                model,
                params,
                expr_params,
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                real_vector_params,
                real_vector_expr_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            let registry = registry.get_or_init(crate::xspice::CodeModelRegistry::with_builtins);
            let model_type = find_model_def(&isolated, model)
                .map_or(model.as_str(), |definition| definition.model_type.as_str());
            let Some(code_model) = registry.get(model_type) else {
                continue;
            };
            if code_model.input_data_file_parameters().is_empty() {
                continue;
            }
            let resolved = resolve_xspice_model_instance(
                &isolated,
                registry,
                model,
                XspiceInstanceParams {
                    params,
                    expr_params,
                    string_params,
                    string_expr_params,
                    string_vector_params,
                    string_vector_expr_params,
                    real_vector_params,
                    real_vector_expr_params,
                },
            )?;
            for input in code_model.input_data_file_parameters() {
                check_build_abort(abort)?;
                let default = code_model
                    .parameters()
                    .iter()
                    .find(|spec| spec.name.eq_ignore_ascii_case(input.name))
                    .and_then(|spec| spec.string_default.as_deref());
                let path = resolved
                    .string_params
                    .iter()
                    .rev()
                    .find(|(name, _)| name.eq_ignore_ascii_case(input.name))
                    .map(|(_, value)| value.as_str())
                    .filter(|value| !input.empty_uses_default || !value.trim().is_empty())
                    .or(default);
                if let Some(path) = path {
                    let path = if input.trim { path.trim() } else { path };
                    paths.extend(crate::xspice::data_file_input_candidates(path));
                }
            }
        }
        check_build_abort(abort)?;
        Ok(paths.into_iter().collect())
    }
}

fn behavioral_data_expression(kind: &ElementKind) -> Option<&str> {
    match kind {
        ElementKind::BehavioralVoltage { expression, .. }
        | ElementKind::BehavioralCurrent { expression, .. } => Some(expression),
        ElementKind::Resistor { value_expr, .. } | ElementKind::Capacitor { value_expr, .. } => {
            value_expr.as_deref()
        }
        _ => None,
    }
}

fn collect_behavioral_data_inputs(
    netlist: &Netlist,
    element: &Element,
    expression: &str,
    abort: &dyn AbortSignal,
    paths: &mut BTreeSet<PathBuf>,
) -> Result<(), SimulationError> {
    use crate::expr::{Expr, ParseExpressionWithAbortError};
    use crate::netlist::expr::BehavioralPreparationError;

    let error_context = |error| {
        SimulationError::Circuit(format!(
            "Behavioral data-file dependencies for '{}': {error}",
            element.name
        ))
    };
    let params = if matches!(element.kind, ElementKind::Resistor { .. }) {
        base_eval_context(netlist)
    } else {
        netlist.params.clone()
    };
    let prepared =
        crate::netlist::expr::prepare_behavioral_expression_with_abort(expression, &params, abort)
            .map_err(|error| match error {
                BehavioralPreparationError::Aborted => SimulationError::Aborted,
                BehavioralPreparationError::Semantic(message) => error_context(message),
            })?;
    let expression =
        crate::expr::parse_expression_strict_with_abort(&prepared, abort).map_err(|error| {
            match error {
                ParseExpressionWithAbortError::Aborted => SimulationError::Aborted,
                ParseExpressionWithAbortError::Parse(error) => error_context(error.to_string()),
            }
        })?;
    let mut pending = vec![&expression];
    while let Some(node) = pending.pop() {
        check_build_abort(abort)?;
        if let Some(path) =
            crate::expr::file_lookup_dependency(node, netlist.source_path.as_deref())
        {
            paths.insert(path);
        }
        match node {
            Expr::Function { args, .. } => pending.extend(args),
            Expr::Unary { operand, .. } => pending.push(operand),
            Expr::Binary { left, right, .. } => pending.extend([left.as_ref(), right.as_ref()]),
            Expr::LookupTable { input, .. } => pending.push(input),
            _ => {}
        }
    }
    Ok(())
}

//! Input-path discovery without model initialization or circuit allocation.
use super::*;
use std::path::PathBuf;

/// One code-model parameter that names a data file, as the model will read it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CodeModelDataFile {
    /// The flattened instance name.
    pub instance: String,
    /// The parameter, spelled as the model's schema spells it.
    pub parameter: &'static str,
    /// The path the model hands the data-file loader: the instance override
    /// or the schema default, trimmed when the model trims it.
    pub path: String,
}

impl Engine {
    /// Discover native input candidates for XSPICE and PEM data-file models and
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
        let mut paths = BTreeSet::new();
        for file in self.data_inputs_with_abort(netlist, abort, Some(&mut paths))? {
            paths.extend(crate::xspice::data_file_input_candidates(&file.path));
        }
        Ok(paths.into_iter().collect())
    }

    /// Every `.VEC` card's vector file, then every XSPICE code-model
    /// parameter of the flattened deck that names a data file, in hierarchy
    /// order, with the path exactly as the build asks the data-file loader
    /// for it: a host that seals the file registers its bytes under that
    /// path.
    ///
    /// The same resolution as [`Self::data_file_candidates_with_abort`]:
    /// nothing is read, no model is initialized, and the caller's statistical
    /// stream does not advance. An empty path names no file and is left out.
    pub fn code_model_data_files_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<CodeModelDataFile>, SimulationError> {
        self.data_inputs_with_abort(netlist, abort, None)
    }

    /// The code-model data files, and, when `other_inputs` is given, the
    /// native candidates of the PEM tables and behavioral lookups too.
    fn data_inputs_with_abort(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
        mut other_inputs: Option<&mut BTreeSet<PathBuf>>,
    ) -> Result<Vec<CodeModelDataFile>, SimulationError> {
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_valid_configuration()?;
        check_build_abort(abort)?;
        // A `.VEC` card's vector file is read as the card names it, when the
        // circuit is built.
        let mut files: Vec<CodeModelDataFile> = netlist
            .vector_files
            .iter()
            .map(|reference| CodeModelDataFile {
                instance: format!(".VEC at {}", reference.origin),
                parameter: "file",
                path: reference.path.clone(),
            })
            .collect();

        // Avoid hierarchy work for decks with no data-consuming element kinds.
        let other_kinds = other_inputs.is_some();
        let mut definitions: Vec<_> = netlist.subcircuits.iter().collect();
        let mut elements = netlist.elements.as_slice();
        loop {
            let mut found = false;
            for element in elements {
                check_build_abort(abort)?;
                if matches!(element.kind, ElementKind::Xspice { .. })
                    || other_kinds
                        && (matches!(element.kind, ElementKind::XyceMemristor { .. })
                            || behavioral_data_expression(&element.kind).is_some())
                {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
            let Some(definition) = definitions.pop() else {
                return Ok(files);
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
        for element in flattened.elements {
            check_build_abort(abort)?;
            if let Some(paths) = other_inputs.as_deref_mut() {
                if let ElementKind::XyceMemristor { model, .. } = &element.kind {
                    collect_pem_data_inputs(
                        &ModelResolution::new(&isolated, abort),
                        &element.name,
                        model,
                        engine.config.temperature,
                        paths,
                    )?;
                }
                if let Some(expression) = behavioral_data_expression(&element.kind) {
                    collect_behavioral_data_inputs(&isolated, &element, expression, abort, paths)?;
                }
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
                &ModelResolution::new(&isolated, abort),
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
                    if !path.is_empty() {
                        files.push(CodeModelDataFile {
                            instance: element.name.clone(),
                            parameter: input.name,
                            path: path.to_owned(),
                        });
                    }
                }
            }
        }
        check_build_abort(abort)?;
        Ok(files)
    }
}

fn collect_pem_data_inputs(
    netlist: &ModelResolution<'_>,
    element_name: &str,
    model_name: &str,
    temperature: Value,
    paths: &mut BTreeSet<PathBuf>,
) -> Result<(), SimulationError> {
    let model = find_model_def(netlist, model_name).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Xyce memristor '{element_name}' references unknown model '{model_name}'"
        ))
    })?;
    if resolve_native_xyce_memristor_family_with_context(
        netlist,
        model,
        element_name,
        model_name,
        temperature,
    )? != NativeXyceMemristorFamily::Pem
    {
        return Ok(());
    }
    for (parameter, default) in [
        (
            "FXPDATA",
            crate::device::XYCE_PEM_DEFAULT_POSITIVE_TABLE_FILE,
        ),
        (
            "FXMDATA",
            crate::device::XYCE_PEM_DEFAULT_NEGATIVE_TABLE_FILE,
        ),
    ] {
        let path =
            xyce_pem_table_path(netlist, model, element_name, model_name, parameter, default)?;
        paths.extend(crate::xspice::data_file_input_candidates(&path));
    }
    Ok(())
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

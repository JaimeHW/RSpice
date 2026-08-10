//! Parser state containers and line-command contexts.

use super::*;

#[derive(Debug, Clone)]
struct ElementNameOrigin {
    spelling: String,
    line: usize,
}

#[derive(Debug, Clone, Default)]
pub(super) struct ElementNameRegistry {
    origins: HashMap<String, ElementNameOrigin>,
}

impl ElementNameRegistry {
    pub(super) fn register(
        &mut self,
        elements: &[Element],
        authored_name: Option<&str>,
        scope: &str,
        line: usize,
    ) -> Result<(), ParseError> {
        for (index, element) in elements.iter().enumerate() {
            let canonical_name = element.name.to_ascii_uppercase();
            let spelling = authored_name
                .filter(|name| index == 0 && name.eq_ignore_ascii_case(&element.name))
                .unwrap_or(&element.name);
            if let Some(first) = self.origins.get(&canonical_name) {
                return Err(ParseError::DuplicateName {
                    canonical_name,
                    first_name: first.spelling.clone(),
                    duplicate_name: spelling.to_string(),
                    scope: scope.to_string(),
                    first_line: first.line,
                    duplicate_line: line,
                });
            }
            self.origins.insert(
                canonical_name,
                ElementNameOrigin {
                    spelling: spelling.to_string(),
                    line,
                },
            );
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn contains_canonical(&self, name: &str) -> bool {
        self.origins.contains_key(name)
    }
}

#[derive(Debug, Clone)]
pub(super) struct SubcktFrame {
    pub(super) def: SubcircuitDef,
    pub(super) qualified_name: String,
    pub(super) opened_at: NetlistSourceLocation,
    pub(super) local_params: ParamContext,
    pub(super) nested_aliases: HashMap<String, String>,
    pub(super) local_model_aliases: HashMap<String, String>,
    pub(super) element_names: ElementNameRegistry,
}

#[derive(Debug)]
pub(super) struct ParseState {
    pub(super) elements: Vec<Element>,
    pub(super) element_names: ElementNameRegistry,
    pub(super) analyses: Vec<AnalysisCommand>,
    pub(super) lin_analysis: Option<LinAnalysis>,
    pub(super) fft_analyses: Vec<FftAnalysis>,
    pub(super) data_tables: Vec<DataTable>,
    pub(super) models: Vec<ModelDef>,
    ///  parameters written as an unresolvable bare identifier, with
    /// the parameter name, the reference, and the line that wrote it.
    pub(super) model_bare_ident_deferrals: Vec<(String, String, usize)>,
    pub(super) subcircuits: Vec<SubcircuitDef>,
    pub(super) params: ParamContext,
    pub(super) initial_conditions: Vec<InitialCondition>,
    pub(super) device_initial_conditions: Option<DeviceInitialConditionDirective>,
    pub(super) node_sets: Vec<NodeSet>,
    pub(super) startup_directives: Vec<StartupDirectiveRecord>,
    pub(super) global_nodes: HashSet<String>,
    pub(super) veriloga_includes: Vec<VerilogAInclude>,
    pub(super) spef_includes: Vec<String>,
    pub(super) measurements: Vec<MeasureStatement>,
    pub(super) saves: SaveSet,
    pub(super) output_requests: Vec<OutputRequest>,
    pub(super) options: super::SimulationOptions,
    pub(super) diagnostics: Vec<ParseDiagnostic>,
    /// Xyce warns and ignores an unmatched top-level `.ENDS`; other dialects
    /// retain the strict parser error for this malformed subcircuit card.
    pub(super) allow_unmatched_subckt_ends: bool,
    /// Xyce closes the current subcircuit on `.ENDS` without validating an
    /// optional terminator name; other dialects retain strict name matching.
    pub(super) enforce_subckt_end_names: bool,
    pub(super) subckt_stack: Vec<SubcktFrame>,
    /// Open `.if`/`.elseif`/`.else` blocks, innermost last.
    pub(super) conditional_stack: Vec<ConditionalFrame>,
    pub(super) mutual_inductor_records: Vec<MutualInductorSemanticRecord>,
    /// Unknown dot-commands and `.options` keys already warned about, so a
    /// deck repeating one card does not flood the log.
    pub(super) unknown_warned: HashSet<String>,
}

impl ParseState {
    pub(super) fn new() -> Self {
        Self {
            elements: Vec::new(),
            element_names: ElementNameRegistry::default(),
            analyses: Vec::new(),
            lin_analysis: None,
            fft_analyses: Vec::new(),
            data_tables: Vec::new(),
            models: Vec::new(),
            model_bare_ident_deferrals: Vec::new(),
            subcircuits: Vec::new(),
            params: ParamContext::new(),
            initial_conditions: Vec::new(),
            device_initial_conditions: None,
            node_sets: Vec::new(),
            startup_directives: Vec::new(),
            global_nodes: HashSet::new(),
            veriloga_includes: Vec::new(),
            spef_includes: Vec::new(),
            measurements: Vec::new(),
            saves: SaveSet::default(),
            output_requests: Vec::new(),
            options: super::SimulationOptions::default(),
            diagnostics: Vec::new(),
            allow_unmatched_subckt_ends: false,
            enforce_subckt_end_names: true,
            subckt_stack: Vec::new(),
            conditional_stack: Vec::new(),
            mutual_inductor_records: Vec::new(),
            unknown_warned: HashSet::new(),
        }
    }

    pub(super) fn push_veriloga_include(&mut self, include: VerilogAInclude) {
        self.veriloga_includes.push(include);
    }

    pub(super) fn into_netlist(
        self,
        title: String,
        input: &str,
        detected_at: NetlistSourceLocation,
        abort: &dyn AbortSignal,
    ) -> Result<Netlist, ParseWithAbortError> {
        if let Some(frame) = self.subckt_stack.last() {
            return Err(Self::missing_subcircuit_ends_error(
                frame,
                detected_at,
                MissingSubcircuitEndsBoundary::EndOfSource,
            )
            .into());
        }
        let mut params = self.params;
        super::super::expr::finalize_parameter_expressions(&mut params)
            .map_err(ParseError::InvalidValue)
            .map_err(ParseWithAbortError::from)?;

        validate_mutual_inductor_semantic_records_with_abort(&self.mutual_inductor_records, abort)?;

        let mut analyses = self.analyses;
        resolve_implicit_step_targets(&mut analyses, &params);

        let mut options = self.options;
        if options.replace_ground == Some(false) {
            // FALSE is the semantic default; explicit spelling must not split
            // transient checkpoint identity from an omitted directive.
            options.replace_ground = None;
        }
        let mut netlist = Netlist {
            title,
            elements: self.elements,
            analyses,
            lin_analysis: self.lin_analysis,
            fft_analyses: self.fft_analyses,
            data_tables: self.data_tables,
            models: self.models,
            subcircuits: self.subcircuits,
            params,
            initial_conditions: self.initial_conditions,
            device_initial_conditions: self.device_initial_conditions,
            node_sets: self.node_sets,
            startup_directives: self.startup_directives,
            global_nodes: self.global_nodes,
            measurements: self.measurements,
            saves: self.saves,
            output_requests: self.output_requests,
            options,
            veriloga_includes: self.veriloga_includes,
            spef_includes: self.spef_includes,
            diagnostics: self.diagnostics,
            source_text: Some(input.to_string()),
            source_path: None,
        };
        apply_temp_directive_to_options(&mut netlist);
        let ground_policy = netlist.ground_policy();
        netlist.saves.apply_ground_policy(ground_policy);
        for measurement in &mut netlist.measurements {
            measurement.apply_ground_policy(ground_policy);
        }
        for analysis in &mut netlist.analyses {
            if let AnalysisCommand::Four { outputs, .. } = analysis {
                for output in outputs {
                    *output = super::super::apply_ground_policy_to_probe_references(
                        output,
                        ground_policy,
                    );
                }
            }
        }
        validate_startup_directives_with_abort(&mut netlist, abort)?;
        Ok(netlist)
    }

    pub(super) fn missing_subcircuit_ends(
        &self,
        detected_at: NetlistSourceLocation,
        boundary: MissingSubcircuitEndsBoundary,
    ) -> Option<ParseError> {
        self.subckt_stack
            .last()
            .map(|frame| Self::missing_subcircuit_ends_error(frame, detected_at, boundary))
    }

    fn missing_subcircuit_ends_error(
        frame: &SubcktFrame,
        detected_at: NetlistSourceLocation,
        boundary: MissingSubcircuitEndsBoundary,
    ) -> ParseError {
        ParseError::MissingSubcircuitEnds(Box::new(MissingSubcircuitEndsError {
            authored_name: frame.def.name.clone(),
            canonical_name: frame.def.name.to_ascii_uppercase(),
            qualified_name: frame.qualified_name.to_ascii_uppercase(),
            opened_at: frame.opened_at.clone(),
            detected_at,
            boundary,
        }))
    }
}

/// Resolve an unqualified `.STEP name ...` after the full deck is known.
///
/// Xyce records the authored name and resolves it at execution time. Global
/// parameters precede natural device/model parameters in that lookup. RSpice
/// keeps an explicit target in the AST, so it defers only the ambiguous bare
/// name until all `.PARAM` declarations have been parsed. Explicit `PARAM`,
/// `MODEL`, `TEMP`, and `device:param` spellings are already unambiguous.
fn resolve_implicit_step_targets(analyses: &mut [AnalysisCommand], params: &ParamContext) {
    for analysis in analyses {
        let AnalysisCommand::Step(command) = analysis else {
            continue;
        };
        if command.target != StepTarget::Device || command.param_name.is_some() {
            continue;
        }
        if params.has_any_parameter_binding(&command.name) {
            command.target = StepTarget::Param;
        }
    }
}

/// Fold a `.TEMP` card into the run temperature that `.OPTIONS TEMP` names.
///
/// ngspice reads `.temp` out of the whole deck and applies it once the circuit
/// exists (`inp.c`, `inp_spsource`), so the card wins over `.options temp` in
/// either authored order and the last `.temp` is the one that lands. Resolving
/// it here rather than at the callers keeps one source of truth for circuit
/// temperature: `.step temp` and `.dc temp` already drive the same field.
///
/// A multi-valued `.temp` is a temperature sweep, which ngspice does not
/// accept at all and RSpice expands in the runners. Its per-point temperature
/// is theirs to set, so the single-run temperature is left alone here.
fn apply_temp_directive_to_options(netlist: &mut Netlist) {
    if let Some(AnalysisCommand::Temp { temperatures }) = netlist
        .analyses
        .iter()
        .rev()
        .find(|analysis| matches!(analysis, AnalysisCommand::Temp { .. }))
        && let [single] = temperatures.as_slice()
    {
        netlist.options.temp = Some(*single);
    }
}

pub(super) struct ParseLineContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) lin_analysis: &'a mut Option<LinAnalysis>,
    pub(super) fft_analyses: &'a mut Vec<FftAnalysis>,
    pub(super) unknown_warned: &'a mut HashSet<String>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) device_initial_conditions: &'a mut Option<DeviceInitialConditionDirective>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) startup_directives: &'a mut Vec<StartupDirectiveRecord>,
    pub(super) startup_scope: StartupDirectiveScope,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) saves: &'a mut SaveSet,
    pub(super) output_requests: &'a mut Vec<OutputRequest>,
    pub(super) options: &'a mut super::SimulationOptions,
    pub(super) diagnostics: &'a mut Vec<ParseDiagnostic>,
    pub(super) spef_includes: &'a mut Vec<String>,
    pub(super) origin: &'a NetlistSourceLocation,
    pub(super) deferred_body_params: Option<&'a mut Vec<(String, String)>>,
    /// Sink for  parameters written as an unresolvable bare
    /// identifier, carrying the parameter name, the reference, and the line.
    pub(super) model_bare_ident_deferrals: &'a mut Vec<(String, String, usize)>,
}

pub(super) struct ParseCommandContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) lin_analysis: &'a mut Option<LinAnalysis>,
    pub(super) fft_analyses: &'a mut Vec<FftAnalysis>,
    pub(super) unknown_warned: &'a mut HashSet<String>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) params: &'a mut ParamContext,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) device_initial_conditions: &'a mut Option<DeviceInitialConditionDirective>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) startup_directives: &'a mut Vec<StartupDirectiveRecord>,
    pub(super) startup_scope: StartupDirectiveScope,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) measurements: &'a mut Vec<MeasureStatement>,
    pub(super) saves: &'a mut SaveSet,
    pub(super) output_requests: &'a mut Vec<OutputRequest>,
    pub(super) options: &'a mut super::SimulationOptions,
    pub(super) diagnostics: &'a mut Vec<ParseDiagnostic>,
    pub(super) spef_includes: &'a mut Vec<String>,
    pub(super) origin: &'a NetlistSourceLocation,
    pub(super) defer_scoped_values: bool,
    pub(super) deferred_body_params: Option<&'a mut Vec<(String, String)>>,
    /// Sink for `.model` parameters written as an unresolvable bare
    /// identifier, carrying the parameter name, the reference, and the line.
    ///
    /// A bare identifier is a forward reference until the deck ends and a
    /// typo afterwards; only end-of-parse validation can tell which, and it
    /// needs the line to report the second case the way the parser used to.
    pub(super) model_bare_ident_deferrals: &'a mut Vec<(String, String, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn resistor(name: &str) -> Element {
        Element {
            name: name.to_string(),
            kind: ElementKind::Resistor {
                value: 1.0,
                value_expr: None,
                model: None,
                instance_params: Vec::new(),
                deferred_params: Vec::new(),
            },
            nodes: vec!["1".to_string(), "0".to_string()],
            provenance: crate::netlist::ElementProvenance::Authored,
        }
    }

    #[test]
    fn element_registry_tracks_every_name_from_one_synthesized_append_batch() {
        let mut registry = ElementNameRegistry::default();
        registry
            .register(
                &[resistor("P1"), resistor("__RSPICE_P1_Z0")],
                Some("P1"),
                "TOP_LEVEL",
                2,
            )
            .expect("authored RF port and synthesized termination register together");

        let error = registry
            .register(
                &[resistor("__RSPICE_P1_Z0")],
                Some("__rspice_p1_z0"),
                "TOP_LEVEL",
                3,
            )
            .expect_err("later name must collide with synthesized termination");
        match error {
            ParseError::DuplicateName {
                canonical_name,
                first_name,
                duplicate_name,
                scope,
                first_line,
                duplicate_line,
            } => {
                assert_eq!(canonical_name, "__RSPICE_P1_Z0");
                assert_eq!(first_name, "__RSPICE_P1_Z0");
                assert_eq!(duplicate_name, "__rspice_p1_z0");
                assert_eq!(scope, "TOP_LEVEL");
                assert_eq!(first_line, 2);
                assert_eq!(duplicate_line, 3);
            }
            other => panic!("expected synthesized-name collision, got {other:?}"),
        }
    }
}

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
    pub(super) fft_analyses: Vec<FftAnalysis>,
    pub(super) data_tables: Vec<DataTable>,
    pub(super) models: Vec<ModelDef>,
    pub(super) subcircuits: Vec<SubcircuitDef>,
    pub(super) params: ParamContext,
    pub(super) initial_conditions: Vec<InitialCondition>,
    pub(super) node_sets: Vec<NodeSet>,
    pub(super) global_nodes: HashSet<String>,
    pub(super) veriloga_includes: Vec<VerilogAInclude>,
    pub(super) spef_includes: Vec<String>,
    pub(super) measurements: Vec<MeasureStatement>,
    pub(super) saves: SaveSet,
    pub(super) options: super::SimulationOptions,
    pub(super) diagnostics: Vec<ParseDiagnostic>,
    pub(super) subckt_stack: Vec<SubcktFrame>,
    /// Open `.if`/`.elseif`/`.else` blocks, innermost last.
    pub(super) conditional_stack: Vec<ConditionalFrame>,
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
            fft_analyses: Vec::new(),
            data_tables: Vec::new(),
            models: Vec::new(),
            subcircuits: Vec::new(),
            params: ParamContext::new(),
            initial_conditions: Vec::new(),
            node_sets: Vec::new(),
            global_nodes: HashSet::new(),
            veriloga_includes: Vec::new(),
            spef_includes: Vec::new(),
            measurements: Vec::new(),
            saves: SaveSet::default(),
            options: super::SimulationOptions::default(),
            diagnostics: Vec::new(),
            subckt_stack: Vec::new(),
            conditional_stack: Vec::new(),
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
    ) -> Result<Netlist, ParseError> {
        if let Some(frame) = self.subckt_stack.last() {
            return Err(Self::missing_subcircuit_ends_error(
                frame,
                detected_at,
                MissingSubcircuitEndsBoundary::EndOfSource,
            ));
        }
        super::super::expr::validate_global_parameter_expressions(&self.params)
            .map_err(ParseError::InvalidValue)?;

        Ok(Netlist {
            title,
            elements: self.elements,
            analyses: self.analyses,
            fft_analyses: self.fft_analyses,
            data_tables: self.data_tables,
            models: self.models,
            subcircuits: self.subcircuits,
            params: self.params,
            initial_conditions: self.initial_conditions,
            node_sets: self.node_sets,
            global_nodes: self.global_nodes,
            measurements: self.measurements,
            saves: self.saves,
            options: self.options,
            veriloga_includes: self.veriloga_includes,
            spef_includes: self.spef_includes,
            diagnostics: self.diagnostics,
            source_text: Some(input.to_string()),
            source_path: None,
        })
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
        ParseError::MissingSubcircuitEnds {
            authored_name: frame.def.name.clone(),
            canonical_name: frame.def.name.to_ascii_uppercase(),
            qualified_name: frame.qualified_name.to_ascii_uppercase(),
            opened_at: frame.opened_at.clone(),
            detected_at,
            boundary,
        }
    }
}

pub(super) struct ParseLineContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) fft_analyses: &'a mut Vec<FftAnalysis>,
    pub(super) unknown_warned: &'a mut HashSet<String>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) saves: &'a mut SaveSet,
    pub(super) options: &'a mut super::SimulationOptions,
    pub(super) diagnostics: &'a mut Vec<ParseDiagnostic>,
    pub(super) spef_includes: &'a mut Vec<String>,
    pub(super) deferred_body_params: Option<&'a mut Vec<(String, String)>>,
}

pub(super) struct ParseCommandContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) fft_analyses: &'a mut Vec<FftAnalysis>,
    pub(super) unknown_warned: &'a mut HashSet<String>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) params: &'a mut ParamContext,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) measurements: &'a mut Vec<MeasureStatement>,
    pub(super) saves: &'a mut SaveSet,
    pub(super) options: &'a mut super::SimulationOptions,
    pub(super) diagnostics: &'a mut Vec<ParseDiagnostic>,
    pub(super) spef_includes: &'a mut Vec<String>,
    pub(super) defer_scoped_values: bool,
    pub(super) deferred_body_params: Option<&'a mut Vec<(String, String)>>,
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

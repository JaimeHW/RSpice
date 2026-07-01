//! Parser state containers and line-command contexts.

use super::*;

#[derive(Debug, Clone)]
pub(super) struct SubcktFrame {
    pub(super) def: SubcircuitDef,
    pub(super) qualified_name: String,
    pub(super) local_params: ParamContext,
    pub(super) nested_aliases: HashMap<String, String>,
    pub(super) local_model_aliases: HashMap<String, String>,
}

#[derive(Debug)]
pub(super) struct ParseState {
    pub(super) elements: Vec<Element>,
    pub(super) analyses: Vec<AnalysisCommand>,
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
            analyses: Vec::new(),
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
        line_num: usize,
    ) -> Result<Netlist, ParseError> {
        if !self.subckt_stack.is_empty() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Unterminated .SUBCKT block".to_string(),
            });
        }

        Ok(Netlist {
            title,
            elements: self.elements,
            analyses: self.analyses,
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
}

pub(super) struct ParseLineContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) unknown_warned: &'a mut HashSet<String>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) saves: &'a mut SaveSet,
    pub(super) options: &'a mut super::SimulationOptions,
    pub(super) diagnostics: &'a mut Vec<ParseDiagnostic>,
    pub(super) spef_includes: &'a mut Vec<String>,
}

pub(super) struct ParseCommandContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
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
}

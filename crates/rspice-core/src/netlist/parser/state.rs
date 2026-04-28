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
    pub(super) models: Vec<ModelDef>,
    pub(super) subcircuits: Vec<SubcircuitDef>,
    pub(super) params: ParamContext,
    pub(super) initial_conditions: Vec<InitialCondition>,
    pub(super) node_sets: Vec<NodeSet>,
    pub(super) global_nodes: HashSet<String>,
    pub(super) veriloga_includes: Vec<VerilogAInclude>,
    pub(super) measurements: Vec<MeasureStatement>,
    pub(super) options: super::SimulationOptions,
    pub(super) subckt_stack: Vec<SubcktFrame>,
}

impl ParseState {
    pub(super) fn new() -> Self {
        Self {
            elements: Vec::new(),
            analyses: Vec::new(),
            models: Vec::new(),
            subcircuits: Vec::new(),
            params: ParamContext::new(),
            initial_conditions: Vec::new(),
            node_sets: Vec::new(),
            global_nodes: HashSet::new(),
            veriloga_includes: Vec::new(),
            measurements: Vec::new(),
            options: super::SimulationOptions::default(),
            subckt_stack: Vec::new(),
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
            models: self.models,
            subcircuits: self.subcircuits,
            params: self.params,
            initial_conditions: self.initial_conditions,
            node_sets: self.node_sets,
            global_nodes: self.global_nodes,
            measurements: self.measurements,
            options: self.options,
            veriloga_includes: self.veriloga_includes,
            source_text: Some(input.to_string()),
            source_path: None,
        })
    }
}

pub(super) struct ParseLineContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) options: &'a mut super::SimulationOptions,
}

pub(super) struct ParseCommandContext<'a> {
    pub(super) analyses: &'a mut Vec<AnalysisCommand>,
    pub(super) models: &'a mut Vec<ModelDef>,
    pub(super) params: &'a mut ParamContext,
    pub(super) initial_conditions: &'a mut Vec<InitialCondition>,
    pub(super) node_sets: &'a mut Vec<NodeSet>,
    pub(super) global_nodes: &'a mut HashSet<String>,
    pub(super) measurements: &'a mut Vec<MeasureStatement>,
    pub(super) options: &'a mut super::SimulationOptions,
}

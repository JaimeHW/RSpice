//! Lexical definitions and instance references, without expanding the circuit.

use super::elements::ElementDetails;
use super::parameters::{Parameter, instance_parameters, scalar_parameters};
use rspice_core::netlist::{Element, ElementKind, SubcircuitDef};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Serialize)]
pub(super) struct Hierarchy<'a> {
    instances: Vec<Instance<'a>>,
    definitions: Vec<Definition<'a>>,
}

#[derive(Serialize)]
struct Instance<'a> {
    name: &'a str,
    subcircuit: &'a str,
    nodes: &'a [String],
    parameters: Vec<Parameter<'a>>,
}

#[derive(Serialize)]
struct Definition<'a> {
    name: &'a str,
    ports: &'a [String],
    element_count: usize,
    parameters: Vec<Parameter<'a>>,
    body_parameters: Vec<Parameter<'a>>,
    element_details: Option<Vec<ElementDetails<'a>>>,
    #[serde(flatten)]
    contents: Hierarchy<'a>,
}

impl<'a> Hierarchy<'a> {
    pub(super) fn new(
        elements: &'a [Element],
        definitions: &'a [SubcircuitDef],
        detailed: bool,
    ) -> Self {
        // The parser also puts qualified copies of nested definitions in the
        // root lookup table. Show their lexical owner once, not both copies.
        let mut nested_names = HashSet::new();
        let mut pending: Vec<_> = definitions
            .iter()
            .flat_map(|s| &s.nested_subcircuits)
            .collect();
        while let Some(nested) = pending.pop() {
            nested_names.insert(nested.name.to_ascii_uppercase());
            pending.extend(&nested.nested_subcircuits);
        }
        Self {
            instances: elements
                .iter()
                .filter_map(|element| {
                    let ElementKind::Subcircuit {
                        subckt_name,
                        params,
                    } = &element.kind
                    else {
                        return None;
                    };
                    Some(Instance {
                        name: &element.name,
                        subcircuit: subckt_name,
                        nodes: &element.nodes,
                        parameters: instance_parameters(params),
                    })
                })
                .collect(),
            definitions: definitions
                .iter()
                .filter(|definition| !nested_names.contains(&definition.name.to_ascii_uppercase()))
                .map(|definition| Definition {
                    name: &definition.name,
                    ports: &definition.ports,
                    element_count: definition.elements.len(),
                    parameters: scalar_parameters(
                        &definition.params,
                        &definition.expr_params,
                        &definition.string_params,
                    ),
                    body_parameters: scalar_parameters(
                        &definition.body_params,
                        &definition.body_expr_params,
                        &definition.body_string_params,
                    ),
                    element_details: detailed.then(|| {
                        definition
                            .elements
                            .iter()
                            .map(ElementDetails::new)
                            .collect()
                    }),
                    contents: Self::new(
                        &definition.elements,
                        &definition.nested_subcircuits,
                        detailed,
                    ),
                })
                .collect(),
        }
    }

    pub(super) fn write(
        &self,
        out: &mut impl std::io::Write,
        indent: usize,
    ) -> std::io::Result<()> {
        for instance in &self.instances {
            writeln!(
                out,
                "{:indent$}{} -> {} ({})",
                "",
                instance.name,
                instance.subcircuit,
                instance.nodes.join(" ")
            )?;
            for parameter in &instance.parameters {
                writeln!(out, "{:indent$}  {parameter}", "")?;
            }
        }
        for definition in &self.definitions {
            writeln!(
                out,
                "{:indent$}.subckt {} ({}, {} elements)",
                "",
                definition.name,
                definition.ports.join(" "),
                definition.element_count
            )?;
            for parameter in &definition.parameters {
                writeln!(out, "{:indent$}  default {parameter}", "")?;
            }
            for parameter in &definition.body_parameters {
                writeln!(out, "{:indent$}  .param {parameter}", "")?;
            }
            if let Some(elements) = &definition.element_details {
                for element in elements {
                    element.write(out, indent + 2)?;
                }
            }
            definition.contents.write(out, indent + 2)?;
        }
        Ok(())
    }
}

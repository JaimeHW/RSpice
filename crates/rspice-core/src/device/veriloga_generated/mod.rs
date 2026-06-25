//! Runtime ABI for build-time generated Verilog-A devices.
//!
//! Generated device modules call this small surface directly from their
//! hand-emitted Rust stamps. Keep it narrow, deterministic, and free of
//! interpreter concepts.

use crate::Value;
use crate::solver::StaticMatrix;

pub mod builtins {
    include!(concat!(env!("OUT_DIR"), "/veriloga_builtins/registry.rs"));
}

#[derive(Debug, Clone)]
pub struct BuiltinVerilogAInstance {
    pub model_name: &'static str,
    pub instance_name: String,
    pub nodes: Vec<usize>,
    kind: builtins::GeneratedBuiltinKind,
}

#[derive(Debug, Clone, Default)]
pub struct BuiltinVerilogADevices {
    devices: Vec<BuiltinVerilogAInstance>,
}

impl BuiltinVerilogADevices {
    #[inline]
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    #[inline]
    pub fn add(&mut self, device: BuiltinVerilogAInstance) {
        self.devices.push(device);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &BuiltinVerilogAInstance> {
        self.devices.iter()
    }

    pub fn stamp_all(&mut self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        for device in &mut self.devices {
            device.stamp(matrix, rhs, voltages);
        }
    }
}

impl BuiltinVerilogAInstance {
    #[inline]
    pub fn stamp(&mut self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let ctx = GeneratedEvalContext::new(voltages);
        let mut stamper = GeneratedStamper::new(matrix, rhs, voltages);
        self.kind.stamp(&ctx, &mut stamper);
    }
}

pub fn instantiate_builtin(
    model_name: &str,
    instance_name: &str,
    node_names: &[String],
    params: &[(String, crate::netlist::ParametricValue)],
    param_ctx: &crate::netlist::ParamContext,
    circuit: &mut crate::CircuitData,
) -> Result<Option<BuiltinVerilogAInstance>, crate::engine::SimulationError> {
    let Some(descriptor_name) = builtins::builtin_names()
        .iter()
        .find(|name| name.eq_ignore_ascii_case(model_name))
        .copied()
    else {
        return Ok(None);
    };

    let expected_nodes = builtins::node_count(descriptor_name).unwrap_or(0);
    if node_names.len() != expected_nodes {
        return Err(crate::engine::SimulationError::Circuit(format!(
            "Generated Verilog-A instance '{}' expects {} terminals for model '{}', found {}",
            instance_name,
            expected_nodes,
            model_name,
            node_names.len()
        )));
    }

    let mut nodes = Vec::with_capacity(node_names.len());
    for node_name in node_names {
        nodes.push(if node_name.eq_ignore_ascii_case("0") {
            0
        } else {
            circuit.get_or_create_node(node_name)
        });
    }

    let mut resolved = Vec::with_capacity(params.len());
    for (name, value) in params {
        let value = match value {
            crate::netlist::ParametricValue::Resolved(value) => *value,
            crate::netlist::ParametricValue::Expression(expr) => {
                crate::netlist::expr::eval_expression(expr, param_ctx).map_err(|error| {
                    crate::engine::SimulationError::Circuit(format!(
                        "Failed to resolve generated Verilog-A parameter '{}': {}",
                        name, error
                    ))
                })?
            }
        };
        resolved.push((name.clone(), value));
    }

    let Some(kind) =
        builtins::instantiate(descriptor_name, &nodes, &resolved).map_err(|error| {
            crate::engine::SimulationError::Circuit(format!(
                "Failed to instantiate generated Verilog-A instance '{}': {}",
                instance_name, error
            ))
        })?
    else {
        return Ok(None);
    };

    Ok(Some(BuiltinVerilogAInstance {
        model_name: descriptor_name,
        instance_name: instance_name.to_string(),
        nodes,
        kind,
    }))
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value]) -> Self {
        Self { voltages }
    }

    #[inline]
    pub fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }
}

pub struct GeneratedStamper<'a> {
    matrix: &'a mut StaticMatrix,
    rhs: &'a mut [Value],
    voltages: &'a [Value],
}

impl<'a> GeneratedStamper<'a> {
    #[inline]
    pub fn new(matrix: &'a mut StaticMatrix, rhs: &'a mut [Value], voltages: &'a [Value]) -> Self {
        Self {
            matrix,
            rhs,
            voltages,
        }
    }

    #[inline]
    pub fn stamp_current(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[(usize, Value)],
    ) {
        let mut equivalent = value;
        for &(node, derivative) in derivatives {
            equivalent -= derivative * self.node_voltage(node);
        }

        if let Some(row) = pos {
            self.stamp_current_row(row, 1.0, equivalent, derivatives);
        }
        if let Some(row) = neg {
            self.stamp_current_row(row, -1.0, equivalent, derivatives);
        }
    }

    #[inline]
    fn stamp_current_row(
        &mut self,
        row_node: usize,
        row_sign: Value,
        equivalent: Value,
        derivatives: &[(usize, Value)],
    ) {
        if row_node == 0 {
            return;
        }
        let row = row_node - 1;
        for &(col_node, derivative) in derivatives {
            if col_node > 0 {
                self.matrix.add(row, col_node - 1, row_sign * derivative);
            }
        }
        if let Some(slot) = self.rhs.get_mut(row) {
            *slot += -row_sign * equivalent;
        }
    }

    #[inline]
    fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }
}

//! Runtime ABI for build-time generated Verilog-A devices.
//!
//! Generated device modules call this small surface directly from their
//! hand-emitted Rust stamps. Keep it narrow, deterministic, and free of
//! interpreter concepts.

use crate::Value;
use crate::solver::{ComplexMatrix, StaticMatrix};

pub mod builtins {
    include!("registry.rs");
}

#[derive(Debug, Clone)]
pub struct BuiltinVerilogAInstance {
    pub model_name: &'static str,
    pub instance_name: String,
    pub nodes: Vec<usize>,
    pub branches: Vec<usize>,
    temperature: Value,
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

    pub fn stamp_all(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
    ) {
        for device in &mut self.devices {
            device.stamp(matrix, rhs, voltages, num_nodes);
        }
    }

    #[inline]
    pub fn set_temperature(&mut self, temperature: Value) {
        for device in &mut self.devices {
            device.set_temperature(temperature);
        }
    }

    #[inline]
    pub fn set_timepoint(&mut self, time: Value, timestep: Value) {
        for device in &mut self.devices {
            device.set_timepoint(time, timestep);
        }
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        for device in &mut self.devices {
            device.accept_timestep();
        }
    }

    pub fn stamp_ac_real_all(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
    ) {
        for device in &mut self.devices {
            device.stamp_ac_real(matrix, voltages, num_nodes);
        }
    }

    pub fn stamp_reactive_all(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        omega: Value,
    ) {
        for device in &mut self.devices {
            device.stamp_reactive(matrix, voltages, num_nodes, omega);
        }
    }
}

impl BuiltinVerilogAInstance {
    #[inline]
    pub fn stamp(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
    ) {
        let ctx = GeneratedEvalContext::new(voltages, self.temperature, num_nodes);
        let mut stamper = GeneratedStamper::new(matrix, rhs, voltages, num_nodes);
        self.kind.stamp(&ctx, &mut stamper);
    }

    #[inline]
    pub fn set_temperature(&mut self, temperature: Value) {
        if temperature.is_finite() && temperature > 0.0 {
            self.temperature = temperature;
        }
    }

    #[inline]
    pub fn set_timepoint(&mut self, time: Value, timestep: Value) {
        self.kind.set_timepoint(time, timestep);
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        self.kind.accept_timestep();
    }

    #[inline]
    pub fn stamp_ac_real(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
    ) {
        let ctx = GeneratedEvalContext::new(voltages, self.temperature, num_nodes);
        let mut stamper = GeneratedStamper::new_ac_real(matrix, voltages, num_nodes);
        self.kind.stamp(&ctx, &mut stamper);
    }

    #[inline]
    pub fn stamp_reactive(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        omega: Value,
    ) {
        let ctx = GeneratedEvalContext::new(voltages, self.temperature, num_nodes);
        let mut stamper = GeneratedReactiveStamper::new(matrix, num_nodes, omega);
        self.kind.stamp_reactive(&ctx, &mut stamper);
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

    let internal_node_names = builtins::internal_node_names(descriptor_name).unwrap_or(&[]);
    let total_nodes = builtins::total_node_count(descriptor_name)
        .unwrap_or(expected_nodes + internal_node_names.len());

    let mut nodes = Vec::with_capacity(total_nodes);
    for node_name in node_names {
        nodes.push(if node_name.eq_ignore_ascii_case("0") {
            0
        } else {
            circuit.get_or_create_node(node_name)
        });
    }
    for internal_name in internal_node_names {
        let node_name = format!("{instance_name}.__{internal_name}.internal");
        nodes.push(circuit.get_or_create_node(&node_name));
    }
    debug_assert_eq!(
        nodes.len(),
        total_nodes,
        "generated Verilog-A node metadata is internally inconsistent"
    );

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

    let branch_count = builtins::branch_count(descriptor_name).unwrap_or(0);
    let mut branches = Vec::with_capacity(branch_count);
    for _ in 0..branch_count {
        branches.push(circuit.allocate_branch());
    }

    let Some(kind) =
        builtins::instantiate(descriptor_name, &nodes, &branches, &resolved).map_err(|error| {
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
        branches,
        temperature: crate::constants::TEMP_REFERENCE,
        kind,
    }))
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
    temperature: Value,
    num_nodes: usize,
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value], temperature: Value, num_nodes: usize) -> Self {
        Self {
            voltages,
            temperature,
            num_nodes,
        }
    }

    #[inline]
    pub fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn temperature(&self) -> Value {
        self.temperature
    }

    #[inline]
    pub fn thermal_voltage(&self) -> Value {
        crate::constants::thermal_voltage(self.temperature)
    }

    #[inline]
    pub fn branch_current(&self, branch_ordinal: usize) -> Value {
        if branch_ordinal == 0 {
            0.0
        } else {
            self.voltages
                .get(self.num_nodes + branch_ordinal - 1)
                .copied()
                .unwrap_or(0.0)
        }
    }
}

enum GeneratedMatrixTarget<'a> {
    Static(&'a mut StaticMatrix),
    AcReal(&'a mut ComplexMatrix),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDerivative {
    axis: GeneratedDerivativeAxis,
    value: Value,
}

impl GeneratedDerivative {
    #[inline]
    pub const fn node(node: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Node(node),
            value,
        }
    }

    #[inline]
    pub const fn branch(branch_ordinal: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Branch(branch_ordinal),
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedDerivativeAxis {
    Node(usize),
    Branch(usize),
}

pub struct GeneratedStamper<'a> {
    matrix: GeneratedMatrixTarget<'a>,
    rhs: Option<&'a mut [Value]>,
    voltages: &'a [Value],
    num_nodes: usize,
}

impl<'a> GeneratedStamper<'a> {
    #[inline]
    pub fn new(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static(matrix),
            rhs: Some(rhs),
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_ac_real(
        matrix: &'a mut ComplexMatrix,
        voltages: &'a [Value],
        num_nodes: usize,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::AcReal(matrix),
            rhs: None,
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn stamp_current(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        let mut equivalent = value;
        for derivative in derivatives {
            equivalent -= derivative.value * self.axis_value(derivative.axis);
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
        derivatives: &[GeneratedDerivative],
    ) {
        if row_node == 0 {
            return;
        }
        let row = row_node - 1;
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                match &mut self.matrix {
                    GeneratedMatrixTarget::Static(matrix) => {
                        matrix.add(row, col, row_sign * derivative.value);
                    }
                    GeneratedMatrixTarget::AcReal(matrix) => {
                        matrix.add_real(row, col, row_sign * derivative.value);
                    }
                }
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += -row_sign * equivalent;
        }
    }

    #[inline]
    pub fn stamp_potential_branch(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_ordinal: usize,
        multiplicity: Value,
    ) {
        let Some(branch) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(node) = pos.filter(|node| *node > 0) {
            self.add_real(node - 1, branch, multiplicity);
            self.add_real(branch, node - 1, 1.0);
        }
        if let Some(node) = neg.filter(|node| *node > 0) {
            self.add_real(node - 1, branch, -multiplicity);
            self.add_real(branch, node - 1, -1.0);
        }
    }

    #[inline]
    pub fn stamp_potential(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let mut equivalent = value;
        for derivative in derivatives {
            equivalent -= derivative.value * self.axis_value(derivative.axis);
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_real(row, col, -derivative.value);
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    fn axis_value(&self, axis: GeneratedDerivativeAxis) -> Value {
        match axis {
            GeneratedDerivativeAxis::Node(node) => {
                if node == 0 {
                    0.0
                } else {
                    self.voltages.get(node - 1).copied().unwrap_or(0.0)
                }
            }
            GeneratedDerivativeAxis::Branch(branch) => self
                .branch_matrix_index(branch)
                .and_then(|index| self.voltages.get(index).copied())
                .unwrap_or(0.0),
        }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        (branch_ordinal > 0).then_some(self.num_nodes + branch_ordinal - 1)
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => (node > 0).then_some(node - 1),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }

    #[inline]
    fn add_real(&mut self, row: usize, col: usize, value: Value) {
        match &mut self.matrix {
            GeneratedMatrixTarget::Static(matrix) => matrix.add(row, col, value),
            GeneratedMatrixTarget::AcReal(matrix) => matrix.add_real(row, col, value),
        }
    }
}

pub struct GeneratedReactiveStamper<'a> {
    matrix: &'a mut ComplexMatrix,
    num_nodes: usize,
    omega: Value,
}

impl<'a> GeneratedReactiveStamper<'a> {
    #[inline]
    pub fn new(matrix: &'a mut ComplexMatrix, num_nodes: usize, omega: Value) -> Self {
        Self {
            matrix,
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn stamp_current_reactive(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(row) = pos {
            self.stamp_current_reactive_row(row, 1.0, derivatives);
        }
        if let Some(row) = neg {
            self.stamp_current_reactive_row(row, -1.0, derivatives);
        }
    }

    #[inline]
    fn stamp_current_reactive_row(
        &mut self,
        row_node: usize,
        row_sign: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        if row_node == 0 {
            return;
        }
        let row = row_node - 1;
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.matrix
                    .add_imag(row, col, row_sign * self.omega * derivative.value);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive(
        &mut self,
        branch_ordinal: usize,
        derivatives: &[GeneratedDerivative],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.matrix
                    .add_imag(row, col, -self.omega * derivative.value);
            }
        }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        (branch_ordinal > 0).then_some(self.num_nodes + branch_ordinal - 1)
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => (node > 0).then_some(node - 1),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }
}

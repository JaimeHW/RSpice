//! Runtime ABI for build-time generated Verilog-A devices.
//!
//! Generated device modules call this small surface directly from their
//! hand-emitted Rust stamps. Keep it narrow, deterministic, and free of
//! interpreter concepts.

use crate::Value;
use crate::solver::{ComplexMatrix, StaticMatrix};

pub(crate) mod support;

pub mod builtins {
    include!("registry.rs");
}

#[derive(Clone)]
pub struct BuiltinVerilogAInstance {
    pub model_name: &'static str,
    pub instance_name: String,
    pub nodes: Vec<usize>,
    pub branches: Vec<usize>,
    temperature: Value,
    kind: builtins::GeneratedBuiltinKind,
}

impl std::fmt::Debug for BuiltinVerilogAInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltinVerilogAInstance")
            .field("model_name", &self.model_name)
            .field("instance_name", &self.instance_name)
            .field("nodes", &self.nodes)
            .field("branches", &self.branches)
            .field("temperature", &self.temperature)
            .finish_non_exhaustive()
    }
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

    #[inline]
    pub(crate) fn restore_from_snapshot(&mut self, snapshot: Self) {
        if self.devices.len() == snapshot.devices.len() {
            for (active, snapshot) in self.devices.iter_mut().zip(snapshot.devices) {
                active.restore_from_snapshot(snapshot);
            }
        } else {
            *self = snapshot;
        }
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
    pub(crate) fn restore_from_snapshot(&mut self, snapshot: Self) {
        debug_assert_eq!(self.model_name, snapshot.model_name);
        debug_assert_eq!(self.instance_name, snapshot.instance_name);
        debug_assert_eq!(self.nodes, snapshot.nodes);
        debug_assert_eq!(self.branches, snapshot.branches);
        self.temperature = snapshot.temperature;
        self.kind.restore_from_snapshot(snapshot.kind);
    }

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
        let ctx = GeneratedEvalContext::with_analysis(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Ac,
        );
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
        let ctx = GeneratedEvalContext::with_analysis(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Ac,
        );
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
pub enum GeneratedAnalysisKind {
    Dc,
    Ac,
    Tran,
    Noise,
    Ic,
}

impl GeneratedAnalysisKind {
    #[inline]
    fn matches_query(self, query: &str) -> bool {
        match query {
            "dc" | "op" => matches!(self, Self::Dc),
            "ac" => matches!(self, Self::Ac),
            "tran" => matches!(self, Self::Tran),
            "noise" => matches!(self, Self::Noise),
            "ic" => matches!(self, Self::Ic),
            "static" => matches!(self, Self::Dc | Self::Ic),
            "smallsig" => matches!(self, Self::Ac | Self::Noise),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
    temperature: Value,
    num_nodes: usize,
    analysis: GeneratedAnalysisKind,
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value], temperature: Value, num_nodes: usize) -> Self {
        Self::with_analysis(voltages, temperature, num_nodes, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn with_analysis(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
    ) -> Self {
        Self {
            voltages,
            temperature,
            num_nodes,
            analysis,
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

    #[inline]
    pub fn analysis(&self, query: &str) -> bool {
        self.analysis.matches_query(query)
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
    pub fn stamp_current_const(&mut self, pos: Option<usize>, neg: Option<usize>, value: Value) {
        if self.rhs.is_none() {
            return;
        }
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        self.add_current_rhs_pair(pos_row, neg_row, value);
    }

    #[inline]
    pub fn stamp_current_node1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0) - derivative1 * self.node_value(node1)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node3(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value(node0)
                - derivative1 * self.node_value(node1)
                - derivative2 * self.node_value(node2)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if let Some(col) = Self::node_matrix_index(node2) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.branch_value(branch0)
                - derivative1 * self.branch_value(branch1)
        } else {
            0.0
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node1_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0) - derivative1 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value(node0)
                - derivative1 * self.node_value(node1)
                - derivative2 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
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
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for derivative in derivatives {
            if needs_rhs {
                equivalent -= derivative.value * self.axis_value(derivative.axis);
            }
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative.value);
            }
        }

        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_dense(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value(node);
            }
            if let Some(col) = Self::node_matrix_index(node) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value(branch);
            }
            if let Some(col) = self.branch_matrix_index(branch) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    fn add_current_derivative_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        col: usize,
        derivative: Value,
    ) {
        match &mut self.matrix {
            GeneratedMatrixTarget::Static(matrix) => {
                if let Some(row) = pos_row {
                    matrix.add(row, col, derivative);
                }
                if let Some(row) = neg_row {
                    matrix.add(row, col, -derivative);
                }
            }
            GeneratedMatrixTarget::AcReal(matrix) => {
                if let Some(row) = pos_row {
                    matrix.add_real(row, col, derivative);
                }
                if let Some(row) = neg_row {
                    matrix.add_real(row, col, -derivative);
                }
            }
        }
    }

    #[inline]
    fn add_current_rhs_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        equivalent: Value,
    ) {
        if let Some(rhs) = &mut self.rhs {
            if let Some(row) = pos_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot -= equivalent;
            }
            if let Some(row) = neg_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot += equivalent;
            }
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
    pub fn stamp_potential_const(&mut self, branch_ordinal: usize, value: Value) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        self.add_potential_rhs(row, value);
    }

    #[inline]
    pub fn stamp_potential_node1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value - derivative0 * self.node_value(node0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent =
            value - derivative0 * self.node_value(node0) - derivative1 * self.node_value(node1);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value - derivative0 * self.branch_value(branch0);
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative0);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch2(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.branch_value(branch0)
            - derivative1 * self.branch_value(branch1);
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node1_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent =
            value - derivative0 * self.node_value(node0) - derivative1 * self.branch_value(branch0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value(node0)
            - derivative1 * self.node_value(node1)
            - derivative2 * self.branch_value(branch0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_real(row, col, -derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative2);
        }
        self.add_potential_rhs(row, equivalent);
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
    pub fn stamp_potential_dense(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.axis_value(GeneratedDerivativeAxis::Node(node));
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Node(node)) {
                self.add_real(row, col, -derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.axis_value(GeneratedDerivativeAxis::Branch(branch));
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Branch(branch)) {
                self.add_real(row, col, -derivative);
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    fn add_potential_rhs(&mut self, row: usize, equivalent: Value) {
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    fn node_value(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn branch_value(&self, branch: usize) -> Value {
        self.branch_matrix_index(branch)
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn axis_value(&self, axis: GeneratedDerivativeAxis) -> Value {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_value(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_value(branch),
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => Self::node_matrix_index(node),
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
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_current_reactive_derivative_pair(
                    pos_row,
                    neg_row,
                    col,
                    self.omega * derivative.value,
                );
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_dense(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = Self::node_matrix_index(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node3(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = Self::node_matrix_index(node2) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    fn add_current_reactive_derivative_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        col: usize,
        derivative: Value,
    ) {
        if let Some(row) = pos_row {
            self.matrix.add_imag(row, col, derivative);
        }
        if let Some(row) = neg_row {
            self.matrix.add_imag(row, col, -derivative);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.matrix.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.matrix.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.matrix.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch1(
        &mut self,
        branch_ordinal: usize,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.matrix.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch2(
        &mut self,
        branch_ordinal: usize,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.matrix.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.matrix.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_branch1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.matrix.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.matrix.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_branch1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.matrix.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.matrix.add_imag(row, col, -self.omega * derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.matrix.add_imag(row, col, -self.omega * derivative2);
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
    pub fn stamp_potential_reactive_dense(
        &mut self,
        branch_ordinal: usize,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Node(node)) {
                self.matrix.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Branch(branch)) {
                self.matrix.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => Self::node_matrix_index(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }
}

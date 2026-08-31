//! AC Small-Signal Analysis
//!
//! Linearizes the circuit at the DC operating point, then performs
//! frequency-domain analysis at each specified frequency. Supports
//! parallel frequency sweeps when the `parallel` feature is enabled.

#![allow(clippy::needless_range_loop)]

use super::data::{FrequencyDataOverridePlan, materialize_frequency_data_row_with_abort};
use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::ac::AcResult;
use crate::device::semiconductor::{
    BJT_DYNAMIC_CHARGE_COUNT, BJT_EXTERNAL_STATE_DIM, BJT_INTERNAL_STATE_DIM, BjtChargeSnapshot,
};
use crate::device::{MatrixStamper, NonlinearDevice};
use crate::solver::{ComplexMatrix, SolverError, StaticMatrix};
use crate::{CircuitData, Complex64, Netlist, NodeId, Value};
use std::collections::VecDeque;
use std::f64::consts::PI;

const BJT_DELAY_XF1_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 2;
const BJT_DELAY_XF2_BRANCH_INDEX: usize = BJT_DYNAMIC_CHARGE_COUNT - 1;
const AC_CONSTRAINT_BACKWARD_ERROR_FACTOR: Value = 64.0;

/// Physical Verilog-A identity of a frequency-domain small-signal operator.
///
/// AC and noise share the same complex matrix assembly, but they are distinct
/// analyses to a model. Keeping that identity explicit prevents the shared
/// numerical path from making `analysis("ac")` true during noise transfer
/// calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SmallSignalAnalysisKind {
    Ac,
    Noise,
}

impl SmallSignalAnalysisKind {
    #[cfg(feature = "veriloga")]
    #[inline]
    const fn runtime_code(self) -> u8 {
        match self {
            Self::Ac => 1,
            Self::Noise => 3,
        }
    }

    #[cfg(feature = "veriloga-builtins-base")]
    #[inline]
    const fn generated_kind(self) -> crate::device::veriloga_builtins::GeneratedAnalysisKind {
        match self {
            Self::Ac => crate::device::veriloga_builtins::GeneratedAnalysisKind::Ac,
            Self::Noise => crate::device::veriloga_builtins::GeneratedAnalysisKind::Noise,
        }
    }
}

fn try_stamp_behavioral_ac_coefficient(
    matrix: &mut ComplexMatrix,
    row: usize,
    column: usize,
    coefficient: Value,
    source_kind: &str,
    source_name: &str,
    frequency: Value,
) -> Result<(), SimulationError> {
    matrix.try_add_real(row, column, coefficient).map_err(|error| {
        SimulationError::Circuit(format!(
            "behavioral {source_kind} source '{source_name}' stamp failed at time {time:.17e} s and frequency {frequency:.17e} Hz while adding matrix coefficient {coefficient}: {error}",
            time = 0.0,
        ))
    })
}

#[derive(Debug, Clone, Copy)]
struct AcVoltageConstraint {
    node_pos: NodeId,
    node_neg: NodeId,
    target: Complex64,
}

#[derive(Debug)]
struct AcVoltageConstraintComponent {
    grounded: bool,
    /// `(node, V(node) - V(root))` in deterministic forest traversal order.
    nodes: Vec<(NodeId, Complex64)>,
}

/// Immutable, allocation-free-at-evaluation projection for independent AC
/// voltage-source equations.
///
/// Sparse equilibration and unscaling can leave an otherwise certified MNA
/// solution a final ULP away from `V(n+) - V(n-) = Vac`. Those equations are
/// ideal constraints, so publish their exact affine manifold after verifying
/// that the raw residual is only solver-sized. A whole connected component is
/// projected together; pairwise snapping would break earlier constraints in a
/// stack of ideal sources.
#[derive(Debug)]
struct AcVoltageConstraintProjection {
    constraints: Vec<AcVoltageConstraint>,
    components: Vec<AcVoltageConstraintComponent>,
}

impl AcVoltageConstraintProjection {
    fn new(circuit: &CircuitData) -> Result<Self, SimulationError> {
        let num_nodes = circuit.num_nodes();
        let sources = &circuit.voltage_sources;
        let mut parents = (0..=num_nodes).collect::<Vec<_>>();
        let mut ranks = vec![0_u8; num_nodes + 1];
        let mut constraints = Vec::with_capacity(sources.len());
        let mut adjacency = vec![Vec::<(NodeId, Complex64)>::new(); num_nodes + 1];

        fn root(parents: &mut [usize], mut node: usize) -> usize {
            let mut representative = node;
            while parents[representative] != representative {
                representative = parents[representative];
            }
            while parents[node] != node {
                let next = parents[node];
                parents[node] = representative;
                node = next;
            }
            representative
        }

        for index in 0..sources.len() {
            let node_pos = sources.node_pos[index];
            let node_neg = sources.node_neg[index];
            if node_pos > num_nodes || node_neg > num_nodes {
                return Err(SolverError::InvalidCircuit(format!(
                    "AC voltage source '{}' references node outside the solved system",
                    sources.names[index]
                ))
                .into());
            }
            let target = sources.ac_excitation(index);
            if !complex_is_finite(target) {
                return Err(SolverError::Overflow.into());
            }

            let root_pos = root(&mut parents, node_pos);
            let root_neg = root(&mut parents, node_neg);
            if root_pos == root_neg {
                return Err(SolverError::InvalidCircuit(format!(
                    "independent voltage source '{}' closes a singular ideal-source loop; its branch current is not uniquely determined",
                    sources.names[index]
                ))
                .into());
            }
            if ranks[root_pos] < ranks[root_neg] {
                parents[root_pos] = root_neg;
            } else {
                parents[root_neg] = root_pos;
                if ranks[root_pos] == ranks[root_neg] {
                    ranks[root_pos] = ranks[root_pos].saturating_add(1);
                }
            }

            constraints.push(AcVoltageConstraint {
                node_pos,
                node_neg,
                target,
            });
            // V(pos) = V(neg) + target, and conversely.
            adjacency[node_neg].push((node_pos, target));
            adjacency[node_pos].push((node_neg, -target));
        }

        let mut components = Vec::new();
        let mut visited = vec![false; num_nodes + 1];
        let mut relative = vec![Complex64::new(0.0, 0.0); num_nodes + 1];
        for root_node in 0..=num_nodes {
            if visited[root_node] || adjacency[root_node].is_empty() {
                continue;
            }
            let grounded = root_node == 0;
            visited[root_node] = true;
            relative[root_node] = Complex64::new(0.0, 0.0);
            let mut queue = VecDeque::from([root_node]);
            let mut nodes = Vec::new();

            while let Some(node) = queue.pop_front() {
                nodes.push((node, relative[node]));
                for &(neighbor, delta) in &adjacency[node] {
                    if visited[neighbor] {
                        continue;
                    }
                    let neighbor_relative = relative[node] + delta;
                    if !complex_is_finite(neighbor_relative) {
                        return Err(SolverError::Overflow.into());
                    }
                    visited[neighbor] = true;
                    relative[neighbor] = neighbor_relative;
                    queue.push_back(neighbor);
                }
            }
            components.push(AcVoltageConstraintComponent { grounded, nodes });
        }

        Ok(Self {
            constraints,
            components,
        })
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }

    fn project(&self, solution: &mut [Complex64]) -> Result<(), SimulationError> {
        self.validate_raw_constraints(solution)?;

        // Validate every prospective component before publishing any of them,
        // so an overflow or malformed solution cannot leave a partial
        // projection behind.
        for component in &self.components {
            let common_mode = if component.grounded {
                Complex64::new(0.0, 0.0)
            } else {
                compensated_common_mode(solution, &component.nodes)?
            };
            for &(node, relative) in &component.nodes {
                if node > 0 && !complex_is_finite(common_mode + relative) {
                    return Err(SolverError::Overflow.into());
                }
            }
        }
        for component in &self.components {
            let common_mode = if component.grounded {
                Complex64::new(0.0, 0.0)
            } else {
                // The validation pass above established that this identical,
                // deterministic calculation and every projected value are
                // finite. Components are disjoint, so earlier publication
                // cannot change a later component's mean.
                compensated_common_mode(solution, &component.nodes)?
            };
            for &(node, relative) in &component.nodes {
                if node > 0 {
                    solution[node - 1] = common_mode + relative;
                }
            }
        }

        // Arithmetic along a long source stack can round while accumulating
        // relative potentials. Recheck the published equations against the
        // same componentwise bound rather than assuming exact subtraction is
        // representable for every common-mode/target scale combination.
        self.validate_raw_constraints(solution)
    }

    fn validate_raw_constraints(&self, solution: &[Complex64]) -> Result<(), SimulationError> {
        for constraint in &self.constraints {
            let node_voltage = |node: NodeId| {
                if node == 0 {
                    Some(Complex64::new(0.0, 0.0))
                } else {
                    solution.get(node - 1).copied()
                }
            };
            let Some(node_pos) = node_voltage(constraint.node_pos) else {
                return Err(SolverError::InvalidCircuit(
                    "AC voltage-source projection received a truncated solution".to_string(),
                )
                .into());
            };
            let Some(node_neg) = node_voltage(constraint.node_neg) else {
                return Err(SolverError::InvalidCircuit(
                    "AC voltage-source projection received a truncated solution".to_string(),
                )
                .into());
            };
            if !complex_is_finite(node_pos) || !complex_is_finite(node_neg) {
                return Err(SolverError::Overflow.into());
            }

            let residual = complex_max_norm(node_pos - node_neg - constraint.target);
            // A homogeneous source row has a zero target and an exact zero
            // solution, so its purely relative denominator also collapses to
            // the unscaling residue we are trying to classify. Use a one-volt
            // coordinate floor: the resulting bound remains only a few tens
            // of femtovolts, far below SPICE voltage tolerances, while
            // admitting roundoff introduced by matrix equilibration.
            let mut scale = complex_max_norm(node_pos);
            scale = (scale + complex_max_norm(node_neg)).min(Value::MAX);
            scale = (scale + complex_max_norm(constraint.target)).min(Value::MAX);
            scale = scale.max(1.0);
            let row_terms = if constraint.node_pos == 0 || constraint.node_neg == 0 {
                2.0
            } else {
                3.0
            };
            let tolerance =
                AC_CONSTRAINT_BACKWARD_ERROR_FACTOR * Value::EPSILON * row_terms * scale;
            if residual > tolerance || !residual.is_finite() {
                let relative_error = if scale > 0.0 {
                    residual / scale
                } else {
                    residual
                };
                return Err(SolverError::InaccurateSolution(relative_error).into());
            }
        }
        Ok(())
    }
}

#[inline]
fn complex_is_finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

#[inline]
fn complex_max_norm(value: Complex64) -> Value {
    value.re.abs().max(value.im.abs())
}

fn compensated_common_mode(
    solution: &[Complex64],
    nodes: &[(NodeId, Complex64)],
) -> Result<Complex64, SimulationError> {
    let mut scale: Value = 0.0;
    for &(node, relative) in nodes {
        debug_assert!(node > 0);
        let sample = solution[node - 1] - relative;
        if !complex_is_finite(sample) {
            return Err(SolverError::Overflow.into());
        }
        scale = scale.max(complex_max_norm(sample));
    }
    if scale == 0.0 {
        return Ok(Complex64::new(0.0, 0.0));
    }

    // Sum normalized samples with Kahan compensation. Scaling first prevents
    // same-sign finite common modes from overflowing while retaining the
    // minimum-L2 translation to substantially better than a naive mean.
    let mut sum_re = 0.0;
    let mut compensation_re = 0.0;
    let mut sum_im = 0.0;
    let mut compensation_im = 0.0;
    for &(node, relative) in nodes {
        debug_assert!(node > 0);
        let raw = (solution[node - 1] - relative) / scale;

        let adjusted_re = raw.re - compensation_re;
        let next_re = sum_re + adjusted_re;
        compensation_re = (next_re - sum_re) - adjusted_re;
        sum_re = next_re;

        let adjusted_im = raw.im - compensation_im;
        let next_im = sum_im + adjusted_im;
        compensation_im = (next_im - sum_im) - adjusted_im;
        sum_im = next_im;
    }
    let count = nodes.len() as Value;
    let mean = Complex64::new(sum_re / count, sum_im / count) * scale;
    if complex_is_finite(mean) {
        Ok(mean)
    } else {
        Err(SolverError::Overflow.into())
    }
}

struct AcImagStamper<'a> {
    matrix: &'a mut ComplexMatrix,
}

impl MatrixStamper for AcImagStamper<'_> {
    #[inline]
    fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            self.matrix.add_imag(row - 1, col - 1, value);
        }
    }

    #[inline]
    fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
        // Small-signal AC uses only dQ/dx matrix terms.
    }
}

impl Engine {
    /// Align every stateful nonlinear model and behavioral-source Jacobian
    /// with a supplied operating state before building a small-signal
    /// operator. Distortion analysis also uses this at nearby bias states for
    /// circuit-wide directional differentiation.
    pub(super) fn prepare_small_signal_state(
        circuit: &mut CircuitData,
        operating_state: &[Value],
    ) -> Result<(), SimulationError> {
        if circuit.has_nonlinear_devices() {
            for dev in &circuit.b3soi.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_fd.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_pd.devices {
                dev.begin_timestep_iteration();
            }
            circuit.update_nonlinear(operating_state);
        }
        circuit
            .prepare_behavioral_small_signal(operating_state)
            .map_err(SimulationError::Circuit)
    }

    /// Align nonlinear devices and behavioral-source Jacobians with an
    /// arbitrary state at the actual small-signal frequency. This is used by
    /// Volterra perturbations, where both state and frequency may differ from
    /// the preceding operator evaluation.
    pub(super) fn prepare_small_signal_state_at_frequency(
        circuit: &mut CircuitData,
        operating_state: &[Value],
        frequency: Value,
    ) -> Result<(), SimulationError> {
        if circuit.has_nonlinear_devices() {
            for dev in &circuit.b3soi.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_fd.devices {
                dev.begin_timestep_iteration();
            }
            for dev in &circuit.b3soi_pd.devices {
                dev.begin_timestep_iteration();
            }
            circuit.update_nonlinear(operating_state);
        }
        circuit
            .prepare_behavioral_small_signal_state_at_frequency(operating_state, frequency)
            .map_err(SimulationError::Circuit)
    }

    #[inline]
    fn ac_node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn stamp_complex_four_terminal(
        matrix: &mut ComplexMatrix,
        row_pos: usize,
        row_neg: usize,
        col_pos: usize,
        col_neg: usize,
        y: Complex64,
    ) {
        if row_pos > 0 {
            if col_pos > 0 {
                matrix.add(row_pos - 1, col_pos - 1, y);
            }
            if col_neg > 0 {
                matrix.add(row_pos - 1, col_neg - 1, -y);
            }
        }
        if row_neg > 0 {
            if col_pos > 0 {
                matrix.add(row_neg - 1, col_pos - 1, -y);
            }
            if col_neg > 0 {
                matrix.add(row_neg - 1, col_neg - 1, y);
            }
        }
    }

    #[inline]
    /// Exact LTRA small-signal branch load (ngspice ltraacld.c):
    /// `Y0(s)*V1 - I1 = e^{-lambda*l}*(Y0(s)*V2 + I2)` and symmetrically for
    /// port 2, with `Y0 = sqrt((G+sC)/(R+sL))`, `lambda*l =
    /// sqrt((G+sCtot)(R+sLtot))` in total quantities (G = 0 for the native
    /// kernel). Stamped on the branch rows the native topology reserves.
    fn stamp_ltra_branch_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        br1: NodeId,
        br2: NodeId,
        omega: Value,
    ) -> bool {
        let Some((ltot, ctot, rtot)) = tline.ltra_ac_total_rlc() else {
            return false;
        };

        let s_c = Complex64::new(0.0, omega * ctot);
        let z_series = Complex64::new(rtot, omega * ltot);
        let y0 = (s_c / z_series).sqrt();
        let lambda_l = (s_c * z_series).sqrt();
        let explambda = (-lambda_l).exp();
        let y0exp = y0 * explambda;
        if !(y0.re.is_finite() && y0.im.is_finite() && explambda.re.is_finite()) {
            return false;
        }

        let mut add = |row: NodeId, col: NodeId, value: Complex64| {
            if row > 0 && col > 0 {
                matrix.add_real(row - 1, col - 1, value.re);
                matrix.add_imag(row - 1, col - 1, value.im);
            }
        };
        let one = Complex64::new(1.0, 0.0);

        for &(br, (pos_self, neg_self), (pos_far, neg_far), far_br) in &[
            (
                br1,
                (tline.node1_pos, tline.node1_neg),
                (tline.node2_pos, tline.node2_neg),
                br2,
            ),
            (
                br2,
                (tline.node2_pos, tline.node2_neg),
                (tline.node1_pos, tline.node1_neg),
                br1,
            ),
        ] {
            add(br, pos_self, y0);
            add(br, neg_self, -y0);
            add(br, br, -one);
            add(br, pos_far, -y0exp);
            add(br, neg_far, y0exp);
            add(br, far_br, -explambda);
        }
        add(tline.node1_pos, br1, one);
        add(tline.node1_neg, br1, -one);
        add(tline.node2_pos, br2, one);
        add(tline.node2_neg, br2, -one);
        true
    }

    /// Native TXL small-signal load. ngspice registers the regular TXLload
    /// as DEVacLoad, and the AC driver runs it under MODEDC, so the oracle
    /// semantic is the DC resistive two-port: `I1 + I2 = 0` and
    /// `V1 - V2 - R*len*I1 = 0` on the reserved branch rows.
    fn stamp_txl_branch_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        br1: NodeId,
        br2: NodeId,
    ) {
        let r_series = tline.dc_series_resistance();
        let mut add = |row: NodeId, col: NodeId, value: Value| {
            if row > 0 && col > 0 {
                matrix.add_real(row - 1, col - 1, value);
            }
        };

        add(tline.node1_pos, br1, 1.0);
        add(tline.node1_neg, br1, -1.0);
        add(tline.node2_pos, br2, 1.0);
        add(tline.node2_neg, br2, -1.0);

        add(br1, br1, 1.0);
        add(br1, br2, 1.0);

        add(br2, tline.node1_pos, 1.0);
        add(br2, tline.node1_neg, -1.0);
        add(br2, tline.node2_pos, -1.0);
        add(br2, tline.node2_neg, 1.0);
        add(br2, br1, -r_series);
    }

    fn stamp_transmission_line_ac(
        matrix: &mut ComplexMatrix,
        tline: &crate::device::TransmissionLine,
        omega: Value,
    ) {
        // Distributed-line Y-parameters:
        // Y11 = Y22 = Y0 * coth(gamma)
        // Y12 = Y21 = -Y0 * csch(gamma)
        // where gamma = alpha + j*omega*td (dimensionless over one delay length).
        let y0 = 1.0 / tline.z0;
        let attenuation = tline.attenuation().clamp(1e-12, 1.0);
        let alpha = (-attenuation.ln()).max(1e-12); // avoid exact lossless singular poles
        let gamma = Complex64::new(alpha, omega * tline.td);
        let sinh_gamma = gamma.sinh();

        let (y11, y12) = if sinh_gamma.norm() < 1e-12 {
            // Series expansion around gamma=0 for numerical stability.
            let inv_gamma = Complex64::new(1.0, 0.0) / gamma;
            let coth_gamma = inv_gamma + gamma / 3.0;
            let csch_gamma = inv_gamma - gamma / 6.0;
            (
                Complex64::new(y0, 0.0) * coth_gamma,
                -Complex64::new(y0, 0.0) * csch_gamma,
            )
        } else {
            let cosh_gamma = gamma.cosh();
            (
                Complex64::new(y0, 0.0) * (cosh_gamma / sinh_gamma),
                -Complex64::new(y0, 0.0) / sinh_gamma,
            )
        };
        let y21 = y12;
        let y22 = y11;

        // Stamp differential 2-port:
        // i1 = y11*v1 + y12*v2
        // i2 = y21*v1 + y22*v2
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node1_pos,
            tline.node1_neg,
            tline.node1_pos,
            tline.node1_neg,
            y11,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node1_pos,
            tline.node1_neg,
            tline.node2_pos,
            tline.node2_neg,
            y12,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node2_pos,
            tline.node2_neg,
            tline.node1_pos,
            tline.node1_neg,
            y21,
        );
        Self::stamp_complex_four_terminal(
            matrix,
            tline.node2_pos,
            tline.node2_neg,
            tline.node2_pos,
            tline.node2_neg,
            y22,
        );
    }

    #[inline]
    fn stamp_imag_two_terminal(
        matrix: &mut ComplexMatrix,
        node_pos: NodeId,
        node_neg: NodeId,
        susceptance: Value,
    ) {
        if node_pos > 0 {
            matrix.add_imag(node_pos - 1, node_pos - 1, susceptance);
            if node_neg > 0 {
                matrix.add_imag(node_pos - 1, node_neg - 1, -susceptance);
            }
        }
        if node_neg > 0 {
            if node_pos > 0 {
                matrix.add_imag(node_neg - 1, node_pos - 1, -susceptance);
            }
            matrix.add_imag(node_neg - 1, node_neg - 1, susceptance);
        }
    }

    #[inline]
    fn stamp_xspice_ac_control_partial(
        matrix: &mut ComplexMatrix,
        row: usize,
        connection: &crate::xspice::PortConnection,
        partial: Complex64,
        sign: Value,
        num_nodes: usize,
    ) {
        let signed = partial * sign;
        match connection {
            crate::xspice::PortConnection::Analog(node) => {
                if *node > 0 {
                    matrix.add(row, *node - 1, signed);
                }
            }
            crate::xspice::PortConnection::Differential(pos, neg) => {
                if *pos > 0 {
                    matrix.add(row, *pos - 1, signed);
                }
                if *neg > 0 {
                    matrix.add(row, *neg - 1, -signed);
                }
            }
            crate::xspice::PortConnection::CurrentProbe { branch_ordinal, .. }
            | crate::xspice::PortConnection::BranchCurrent { branch_ordinal }
            | crate::xspice::PortConnection::Hybrid { branch_ordinal, .. } => {
                if *branch_ordinal > 0 {
                    matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                }
            }
            crate::xspice::PortConnection::NamedBranchCurrent {
                branch_ordinal: Some(branch_ordinal),
                ..
            } => {
                if *branch_ordinal > 0 {
                    matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                }
            }
            _ => {}
        }
    }

    fn stamp_xspice_ac_vector_control_partial(
        matrix: &mut ComplexMatrix,
        row: usize,
        connection: &crate::xspice::PortConnection,
        index: usize,
        partial: Complex64,
        sign: Value,
        num_nodes: usize,
    ) {
        let signed = partial * sign;
        match connection {
            crate::xspice::PortConnection::AnalogVector(nodes) => {
                if let Some(node) = nodes.get(index)
                    && *node > 0
                {
                    matrix.add(row, *node - 1, signed);
                }
            }
            crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                let Some(element) = elements.get(index) else {
                    return;
                };
                match element {
                    crate::xspice::AnalogInputConnection::Node(node) => {
                        if *node > 0 {
                            matrix.add(row, *node - 1, signed);
                        }
                    }
                    crate::xspice::AnalogInputConnection::Differential(pos, neg) => {
                        if *pos > 0 {
                            matrix.add(row, *pos - 1, signed);
                        }
                        if *neg > 0 {
                            matrix.add(row, *neg - 1, -signed);
                        }
                    }
                    crate::xspice::AnalogInputConnection::CurrentProbe {
                        branch_ordinal, ..
                    }
                    | crate::xspice::AnalogInputConnection::BranchCurrent { branch_ordinal }
                    | crate::xspice::AnalogInputConnection::Hybrid { branch_ordinal, .. } => {
                        if *branch_ordinal > 0 {
                            matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                        }
                    }
                    crate::xspice::AnalogInputConnection::NamedBranchCurrent {
                        branch_ordinal: Some(branch_ordinal),
                        ..
                    } => {
                        if *branch_ordinal > 0 {
                            matrix.add(row, num_nodes + *branch_ordinal - 1, signed);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn is_nonzero_finite_complex(value: Complex64) -> bool {
        value != Complex64::new(0.0, 0.0) && value.re.is_finite() && value.im.is_finite()
    }

    fn stamp_xspice_ac_current_probe(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        pos: usize,
        neg: usize,
        branch_ordinal: usize,
    ) {
        if branch_ordinal == 0 {
            return;
        }
        let br = circuit.get_branch_matrix_index(branch_ordinal);
        let br_idx = br - 1;
        if pos > 0 {
            ac_matrix.add_real(br_idx, pos - 1, 1.0);
            ac_matrix.add_real(pos - 1, br_idx, 1.0);
        }
        if neg > 0 {
            ac_matrix.add_real(br_idx, neg - 1, -1.0);
            ac_matrix.add_real(neg - 1, br_idx, -1.0);
        }
    }

    #[inline]
    fn stamp_xspice_ac_voltage_branch_topology(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        branch_ordinal: usize,
        pos: usize,
        neg: usize,
    ) {
        Self::stamp_xspice_ac_current_probe(circuit, ac_matrix, pos, neg, branch_ordinal);
    }

    #[inline]
    fn xspice_ac_current_output_self_conductance(
        port: &crate::xspice::PortSpec,
        conductance: Value,
    ) -> Value {
        match port.default_type {
            crate::xspice::PortType::Current
            | crate::xspice::PortType::DifferentialCurrent
            | crate::xspice::PortType::Conductance
            | crate::xspice::PortType::DifferentialConductance => conductance,
            _ => 0.0,
        }
    }

    #[inline]
    fn stamp_xspice_ac_current_self_conductance(
        ac_matrix: &mut ComplexMatrix,
        pos: usize,
        neg: usize,
        conductance: Value,
    ) {
        if !conductance.is_finite() || conductance == 0.0 {
            return;
        }
        if pos > 0 {
            ac_matrix.add_real(pos - 1, pos - 1, conductance);
            if neg > 0 {
                ac_matrix.add_real(pos - 1, neg - 1, -conductance);
            }
        }
        if neg > 0 {
            if pos > 0 {
                ac_matrix.add_real(neg - 1, pos - 1, -conductance);
            }
            ac_matrix.add_real(neg - 1, neg - 1, conductance);
        }
    }

    fn stamp_xspice_ac_vector_voltage_controls(
        ac_matrix: &mut ComplexMatrix,
        instance: &crate::xspice::XspiceInstance,
        output_port: &str,
        output_index: usize,
        branch_row: usize,
        frequency_hz: Value,
        num_nodes: usize,
    ) {
        for (control_port, partial) in
            instance.output_vector_input_ac_partials(output_port, output_index, frequency_hz)
        {
            if !Self::is_nonzero_finite_complex(partial) {
                continue;
            }
            if let Some(control_connection) = instance.connection(&control_port) {
                Self::stamp_xspice_ac_control_partial(
                    ac_matrix,
                    branch_row,
                    control_connection,
                    partial,
                    -1.0,
                    num_nodes,
                );
            }
        }
        for (control_port, index, partial) in
            instance.output_vector_input_vector_ac_partials(output_port, output_index, frequency_hz)
        {
            if !Self::is_nonzero_finite_complex(partial) {
                continue;
            }
            if let Some(control_connection) = instance.connection(&control_port) {
                Self::stamp_xspice_ac_vector_control_partial(
                    ac_matrix,
                    branch_row,
                    control_connection,
                    index,
                    partial,
                    -1.0,
                    num_nodes,
                );
            }
        }
    }

    fn stamp_xspice_ac_vector_current_controls(
        ac_matrix: &mut ComplexMatrix,
        instance: &crate::xspice::XspiceInstance,
        output_port: &str,
        output_index: usize,
        pos: usize,
        neg: usize,
        frequency_hz: Value,
        num_nodes: usize,
    ) {
        for (control_port, partial) in
            instance.output_vector_input_ac_partials(output_port, output_index, frequency_hz)
        {
            if !Self::is_nonzero_finite_complex(partial) {
                continue;
            }
            let Some(control_connection) = instance.connection(&control_port) else {
                continue;
            };
            if pos > 0 {
                Self::stamp_xspice_ac_control_partial(
                    ac_matrix,
                    pos - 1,
                    control_connection,
                    partial,
                    1.0,
                    num_nodes,
                );
            }
            if neg > 0 {
                Self::stamp_xspice_ac_control_partial(
                    ac_matrix,
                    neg - 1,
                    control_connection,
                    partial,
                    -1.0,
                    num_nodes,
                );
            }
        }
        for (control_port, index, partial) in
            instance.output_vector_input_vector_ac_partials(output_port, output_index, frequency_hz)
        {
            if !Self::is_nonzero_finite_complex(partial) {
                continue;
            }
            let Some(control_connection) = instance.connection(&control_port) else {
                continue;
            };
            if pos > 0 {
                Self::stamp_xspice_ac_vector_control_partial(
                    ac_matrix,
                    pos - 1,
                    control_connection,
                    index,
                    partial,
                    1.0,
                    num_nodes,
                );
            }
            if neg > 0 {
                Self::stamp_xspice_ac_vector_control_partial(
                    ac_matrix,
                    neg - 1,
                    control_connection,
                    index,
                    partial,
                    -1.0,
                    num_nodes,
                );
            }
        }
    }

    fn stamp_xspice_ac_vector_output_element(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        instance: &crate::xspice::XspiceInstance,
        port: &crate::xspice::PortSpec,
        port_idx: usize,
        output_index: usize,
        pos: usize,
        neg: usize,
        force_current_output: bool,
        frequency_hz: Value,
        num_nodes: usize,
    ) {
        let (conductance, _) =
            instance.analog_vector_small_signal_contribution_at(port_idx, output_index);
        let stamp_as_current_output = force_current_output
            || matches!(
                port.default_type,
                crate::xspice::PortType::Current
                    | crate::xspice::PortType::DifferentialCurrent
                    | crate::xspice::PortType::Conductance
                    | crate::xspice::PortType::DifferentialConductance
            );
        if stamp_as_current_output {
            let self_conductance =
                Self::xspice_ac_current_output_self_conductance(port, conductance);
            Self::stamp_xspice_ac_current_self_conductance(ac_matrix, pos, neg, self_conductance);
            Self::stamp_xspice_ac_vector_current_controls(
                ac_matrix,
                instance,
                &port.name,
                output_index,
                pos,
                neg,
                frequency_hz,
                num_nodes,
            );
            return;
        }
        match port.default_type {
            crate::xspice::PortType::Voltage
            | crate::xspice::PortType::DifferentialVoltage
            | crate::xspice::PortType::Hybrid
            | crate::xspice::PortType::DifferentialHybrid => {
                let Some(branch_ordinal) =
                    instance.branch_vector_output_ordinal(port_idx, output_index)
                else {
                    return;
                };
                Self::stamp_xspice_ac_voltage_branch_topology(
                    circuit,
                    ac_matrix,
                    branch_ordinal,
                    pos,
                    neg,
                );
                let branch = circuit.get_branch_matrix_index(branch_ordinal);
                Self::stamp_xspice_ac_vector_voltage_controls(
                    ac_matrix,
                    instance,
                    &port.name,
                    output_index,
                    branch - 1,
                    frequency_hz,
                    num_nodes,
                );
            }
            _ => {}
        }
    }

    fn stamp_xspice_small_signal_ac(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        frequency_hz: Value,
    ) {
        let num_nodes = circuit.num_nodes();
        for instance in &circuit.xspice_instances {
            let ports = instance.ports();
            for (pos, neg, branch_ordinal) in instance.current_probe_branches() {
                Self::stamp_xspice_ac_current_probe(circuit, ac_matrix, pos, neg, branch_ordinal);
            }
            for (port_idx, connection) in instance.connections().iter().enumerate() {
                let Some(port) = ports.get(port_idx) else {
                    continue;
                };
                if !matches!(
                    port.direction,
                    crate::xspice::PortDirection::Out | crate::xspice::PortDirection::InOut
                ) {
                    continue;
                }

                if instance.has_analog_vector_small_signal_contributions(port_idx) {
                    match connection {
                        crate::xspice::PortConnection::AnalogVector(nodes) => {
                            for (output_index, node) in nodes.iter().copied().enumerate() {
                                Self::stamp_xspice_ac_vector_output_element(
                                    circuit,
                                    ac_matrix,
                                    instance,
                                    port,
                                    port_idx,
                                    output_index,
                                    node,
                                    0,
                                    false,
                                    frequency_hz,
                                    num_nodes,
                                );
                            }
                        }
                        crate::xspice::PortConnection::TypedAnalogVector(elements) => {
                            for (output_index, element) in elements.iter().enumerate() {
                                match element {
                                    crate::xspice::AnalogInputConnection::Node(node) => {
                                        Self::stamp_xspice_ac_vector_output_element(
                                            circuit,
                                            ac_matrix,
                                            instance,
                                            port,
                                            port_idx,
                                            output_index,
                                            *node,
                                            0,
                                            false,
                                            frequency_hz,
                                            num_nodes,
                                        );
                                    }
                                    crate::xspice::AnalogInputConnection::Differential(
                                        pos,
                                        neg,
                                    )
                                    | crate::xspice::AnalogInputConnection::Hybrid {
                                        pos,
                                        neg,
                                        ..
                                    } => {
                                        Self::stamp_xspice_ac_vector_output_element(
                                            circuit,
                                            ac_matrix,
                                            instance,
                                            port,
                                            port_idx,
                                            output_index,
                                            *pos,
                                            *neg,
                                            false,
                                            frequency_hz,
                                            num_nodes,
                                        );
                                    }
                                    crate::xspice::AnalogInputConnection::CurrentOutput {
                                        pos,
                                        neg,
                                    } => {
                                        Self::stamp_xspice_ac_vector_output_element(
                                            circuit,
                                            ac_matrix,
                                            instance,
                                            port,
                                            port_idx,
                                            output_index,
                                            *pos,
                                            *neg,
                                            true,
                                            frequency_hz,
                                            num_nodes,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                if let crate::xspice::PortConnection::CurrentOutput { pos, neg } = connection {
                    if let Some((conductance, _)) = instance.get_analog_contribution(port_idx) {
                        let self_conductance =
                            Self::xspice_ac_current_output_self_conductance(port, conductance);
                        Self::stamp_xspice_ac_current_self_conductance(
                            ac_matrix,
                            *pos,
                            *neg,
                            self_conductance,
                        );
                    }
                    for (control_port, partial) in
                        instance.output_input_ac_partials(&port.name, frequency_hz)
                    {
                        if !Self::is_nonzero_finite_complex(partial) {
                            continue;
                        }
                        let Some(control_connection) = instance.connection(&control_port) else {
                            continue;
                        };
                        if *pos > 0 {
                            Self::stamp_xspice_ac_control_partial(
                                ac_matrix,
                                *pos - 1,
                                control_connection,
                                partial,
                                1.0,
                                num_nodes,
                            );
                        }
                        if *neg > 0 {
                            Self::stamp_xspice_ac_control_partial(
                                ac_matrix,
                                *neg - 1,
                                control_connection,
                                partial,
                                -1.0,
                                num_nodes,
                            );
                        }
                    }
                    for (control_port, index, partial) in
                        instance.output_input_vector_ac_partials(&port.name, frequency_hz)
                    {
                        if !Self::is_nonzero_finite_complex(partial) {
                            continue;
                        }
                        let Some(control_connection) = instance.connection(&control_port) else {
                            continue;
                        };
                        if *pos > 0 {
                            Self::stamp_xspice_ac_vector_control_partial(
                                ac_matrix,
                                *pos - 1,
                                control_connection,
                                index,
                                partial,
                                1.0,
                                num_nodes,
                            );
                        }
                        if *neg > 0 {
                            Self::stamp_xspice_ac_vector_control_partial(
                                ac_matrix,
                                *neg - 1,
                                control_connection,
                                index,
                                partial,
                                -1.0,
                                num_nodes,
                            );
                        }
                    }
                    continue;
                }

                match port.default_type {
                    crate::xspice::PortType::Voltage
                    | crate::xspice::PortType::DifferentialVoltage
                    | crate::xspice::PortType::Hybrid
                    | crate::xspice::PortType::DifferentialHybrid => {
                        let Some(branch_ordinal) = instance.branch_ordinal_at(port_idx) else {
                            continue;
                        };
                        let br = circuit.get_branch_matrix_index(branch_ordinal);
                        let br_idx = br - 1;

                        match connection {
                            crate::xspice::PortConnection::Analog(node) => {
                                if *node > 0 {
                                    ac_matrix.add_real(br_idx, *node - 1, 1.0);
                                    ac_matrix.add_real(*node - 1, br_idx, 1.0);
                                }
                            }
                            crate::xspice::PortConnection::Differential(pos, neg) => {
                                if *pos > 0 {
                                    ac_matrix.add_real(br_idx, *pos - 1, 1.0);
                                    ac_matrix.add_real(*pos - 1, br_idx, 1.0);
                                }
                                if *neg > 0 {
                                    ac_matrix.add_real(br_idx, *neg - 1, -1.0);
                                    ac_matrix.add_real(*neg - 1, br_idx, -1.0);
                                }
                            }
                            crate::xspice::PortConnection::Hybrid { pos, neg, .. } => {
                                Self::stamp_xspice_ac_voltage_branch_topology(
                                    circuit,
                                    ac_matrix,
                                    branch_ordinal,
                                    *pos,
                                    *neg,
                                );
                            }
                            _ => continue,
                        }

                        for (control_port, partial) in
                            instance.output_input_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            if let Some(control_connection) = instance.connection(&control_port) {
                                Self::stamp_xspice_ac_control_partial(
                                    ac_matrix,
                                    br_idx,
                                    control_connection,
                                    partial,
                                    -1.0,
                                    num_nodes,
                                );
                            }
                        }
                        for (control_port, index, partial) in
                            instance.output_input_vector_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            if let Some(control_connection) = instance.connection(&control_port) {
                                Self::stamp_xspice_ac_vector_control_partial(
                                    ac_matrix,
                                    br_idx,
                                    control_connection,
                                    index,
                                    partial,
                                    -1.0,
                                    num_nodes,
                                );
                            }
                        }
                    }
                    crate::xspice::PortType::Current
                    | crate::xspice::PortType::DifferentialCurrent
                    | crate::xspice::PortType::Conductance
                    | crate::xspice::PortType::DifferentialConductance => {
                        if let Some((conductance, _)) = instance.get_analog_contribution(port_idx) {
                            let self_conductance =
                                Self::xspice_ac_current_output_self_conductance(port, conductance);
                            match connection {
                                crate::xspice::PortConnection::Analog(node) => {
                                    Self::stamp_xspice_ac_current_self_conductance(
                                        ac_matrix,
                                        *node,
                                        0,
                                        self_conductance,
                                    );
                                }
                                crate::xspice::PortConnection::Differential(pos, neg)
                                | crate::xspice::PortConnection::CurrentOutput { pos, neg } => {
                                    Self::stamp_xspice_ac_current_self_conductance(
                                        ac_matrix,
                                        *pos,
                                        *neg,
                                        self_conductance,
                                    );
                                }
                                _ => {}
                            }
                        }
                        for (control_port, partial) in
                            instance.output_input_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            let Some(control_connection) = instance.connection(&control_port)
                            else {
                                continue;
                            };
                            match connection {
                                crate::xspice::PortConnection::Analog(node) => {
                                    if *node > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *node - 1,
                                            control_connection,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                crate::xspice::PortConnection::Differential(pos, neg) => {
                                    if *pos > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *pos - 1,
                                            control_connection,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                    if *neg > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *neg - 1,
                                            control_connection,
                                            partial,
                                            -1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                crate::xspice::PortConnection::CurrentOutput { pos, neg } => {
                                    if *pos > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *pos - 1,
                                            control_connection,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                    if *neg > 0 {
                                        Self::stamp_xspice_ac_control_partial(
                                            ac_matrix,
                                            *neg - 1,
                                            control_connection,
                                            partial,
                                            -1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                        for (control_port, index, partial) in
                            instance.output_input_vector_ac_partials(&port.name, frequency_hz)
                        {
                            if !Self::is_nonzero_finite_complex(partial) {
                                continue;
                            }
                            let Some(control_connection) = instance.connection(&control_port)
                            else {
                                continue;
                            };
                            match connection {
                                crate::xspice::PortConnection::Analog(node) => {
                                    if *node > 0 {
                                        Self::stamp_xspice_ac_vector_control_partial(
                                            ac_matrix,
                                            *node - 1,
                                            control_connection,
                                            index,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                crate::xspice::PortConnection::Differential(pos, neg)
                                | crate::xspice::PortConnection::CurrentOutput { pos, neg } => {
                                    if *pos > 0 {
                                        Self::stamp_xspice_ac_vector_control_partial(
                                            ac_matrix,
                                            *pos - 1,
                                            control_connection,
                                            index,
                                            partial,
                                            1.0,
                                            num_nodes,
                                        );
                                    }
                                    if *neg > 0 {
                                        Self::stamp_xspice_ac_vector_control_partial(
                                            ac_matrix,
                                            *neg - 1,
                                            control_connection,
                                            index,
                                            partial,
                                            -1.0,
                                            num_nodes,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn solve_small_dense_complex_system<const N: usize>(
        matrix: &[[Complex64; N]; N],
        rhs: &[Complex64; N],
        dim: usize,
    ) -> Option<[Complex64; N]> {
        if dim == 0 {
            return Some([Complex64::new(0.0, 0.0); N]);
        }

        let mut a = *matrix;
        let mut b = *rhs;

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = a[pivot][pivot].norm();
            for row in (pivot + 1)..dim {
                let value = a[row][pivot].norm();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                a.swap(pivot, best);
                b.swap(pivot, best);
            }

            let pivot_value = a[pivot][pivot];
            for row in (pivot + 1)..dim {
                let factor = a[row][pivot] / pivot_value;
                a[row][pivot] = Complex64::new(0.0, 0.0);
                for col in (pivot + 1)..dim {
                    a[row][col] -= factor * a[pivot][col];
                }
                b[row] -= factor * b[pivot];
            }
        }

        let mut x = [Complex64::new(0.0, 0.0); N];
        for row in (0..dim).rev() {
            let mut sum = b[row];
            for col in (row + 1)..dim {
                sum -= a[row][col] * x[col];
            }
            let diag = a[row][row];
            if diag.norm() < 1e-18 {
                return None;
            }
            x[row] = sum / diag;
        }

        Some(x)
    }

    fn stamp_bjt_dynamic_ac(
        matrix: &mut ComplexMatrix,
        bjt: &crate::device::Bjt,
        op_voltages: &[Value],
        omega: Value,
        include_delay_branches: bool,
    ) {
        if bjt.vbic_mna_promoted() {
            // Promoted BJT: the internal states are matrix unknowns, so each
            // charge branch stamps jw*C directly on its own nodes alongside
            // the promoted static real part - no dense Schur reduction.
            let (branches, _, _) = bjt.vbic_mna_charge_state_at_solution(op_voltages);
            let external_nodes = [
                bjt.node_collector,
                bjt.node_base,
                bjt.node_emitter,
                bjt.node_substrate,
            ];
            for (branch_idx, branch) in branches.iter().enumerate() {
                if !branch.is_active() {
                    continue;
                }
                if !include_delay_branches
                    && (branch_idx == BJT_DELAY_XF1_BRANCH_INDEX
                        || branch_idx == BJT_DELAY_XF2_BRANCH_INDEX)
                {
                    // Without the xf charges the algebraic xf rows pin vxf2
                    // to Itzf and the delayed-transport correction vanishes
                    // (the pre-xf reduced behavior). ngspice-46 keeps these
                    // charges in AC (vbicacld.c XQxf stamps), so every
                    // production caller passes true; the reduced mode
                    // remains for descriptor-based callers that add charge
                    // terms themselves.
                    continue;
                }

                let mut stamp_row = |row: NodeId, sign: Value| {
                    if row == 0 {
                        return;
                    }
                    for col in 0..BJT_INTERNAL_STATE_DIM {
                        let c = branch.d_internal[col];
                        let col_node = bjt.vbic_internal_node(col);
                        if c != 0.0 && col_node > 0 {
                            matrix.add_imag(row - 1, col_node - 1, sign * omega * c);
                        }
                    }
                    for col in 0..BJT_EXTERNAL_STATE_DIM {
                        let c = branch.d_external[col];
                        let col_node = external_nodes[col];
                        if c != 0.0 && col_node > 0 {
                            matrix.add_imag(row - 1, col_node - 1, sign * omega * c);
                        }
                    }
                };

                let pos = branch
                    .pos_internal
                    .map(|idx| bjt.vbic_internal_node(idx))
                    .or_else(|| branch.pos_external.map(|idx| external_nodes[idx]));
                let neg = branch
                    .neg_internal
                    .map(|idx| bjt.vbic_internal_node(idx))
                    .or_else(|| branch.neg_external.map(|idx| external_nodes[idx]));
                if let Some(row) = pos {
                    stamp_row(row, 1.0);
                }
                if let Some(row) = neg {
                    stamp_row(row, -1.0);
                }
            }
            return;
        }

        let [vc, vb, ve, vs] = [
            Self::ac_node_voltage(op_voltages, bjt.node_collector),
            Self::ac_node_voltage(op_voltages, bjt.node_base),
            Self::ac_node_voltage(op_voltages, bjt.node_emitter),
            Self::ac_node_voltage(op_voltages, bjt.node_substrate),
        ];
        let snapshot: BjtChargeSnapshot = bjt.charge_snapshot(vc, vb, ve, vs);
        let mut c_ii = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ie = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        let mut c_ei = [[0.0; BJT_INTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut c_ee = [[0.0; BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        let mut has_dynamic_charge = false;
        for (branch_idx, branch) in snapshot.branches.iter().enumerate() {
            if !branch.is_active() {
                continue;
            }
            if !include_delay_branches
                && (branch_idx == BJT_DELAY_XF1_BRANCH_INDEX
                    || branch_idx == BJT_DELAY_XF2_BRANCH_INDEX)
            {
                // Reduced mode without the xf companion charges (see the
                // promoted arm above); ngspice-46 includes them in AC.
                continue;
            }
            branch.accumulate_derivatives(&mut c_ii, &mut c_ie, &mut c_ei, &mut c_ee);
            has_dynamic_charge = true;
        }
        if !has_dynamic_charge {
            return;
        }

        let s = Complex64::new(0.0, omega);
        let mut internal =
            [[Complex64::new(0.0, 0.0); BJT_INTERNAL_STATE_DIM]; BJT_INTERNAL_STATE_DIM];
        for row in 0..BJT_INTERNAL_STATE_DIM {
            for col in 0..BJT_INTERNAL_STATE_DIM {
                internal[row][col] =
                    Complex64::new(snapshot.reduction.g_ii[row][col], 0.0) + s * c_ii[row][col];
            }
        }

        let mut y_total =
            [[Complex64::new(0.0, 0.0); BJT_EXTERNAL_STATE_DIM]; BJT_EXTERNAL_STATE_DIM];
        for col in 0..BJT_EXTERNAL_STATE_DIM {
            let mut rhs = [Complex64::new(0.0, 0.0); BJT_INTERNAL_STATE_DIM];
            for row in 0..BJT_INTERNAL_STATE_DIM {
                rhs[row] =
                    -(Complex64::new(snapshot.reduction.g_ie[row][col], 0.0) + s * c_ie[row][col]);
            }

            let Some(solution) =
                Self::solve_small_dense_complex_system(&internal, &rhs, BJT_INTERNAL_STATE_DIM)
            else {
                return;
            };

            for row in 0..BJT_EXTERNAL_STATE_DIM {
                let mut value =
                    Complex64::new(snapshot.reduction.g_ee[row][col], 0.0) + s * c_ee[row][col];
                for idx in 0..BJT_INTERNAL_STATE_DIM {
                    value += (Complex64::new(snapshot.reduction.g_ei[row][idx], 0.0)
                        + s * c_ei[row][idx])
                        * solution[idx];
                }
                y_total[row][col] = value;
            }
        }

        let nodes = [
            bjt.node_collector,
            bjt.node_base,
            bjt.node_emitter,
            bjt.node_substrate,
        ];
        for row in 0..BJT_EXTERNAL_STATE_DIM {
            for col in 0..BJT_EXTERNAL_STATE_DIM {
                let delta =
                    y_total[row][col] - Complex64::new(snapshot.reduction.g_reduced[row][col], 0.0);
                if delta.norm() > 0.0 && nodes[row] > 0 && nodes[col] > 0 {
                    matrix.add(nodes[row] - 1, nodes[col] - 1, delta);
                }
            }
        }
    }

    #[inline]
    fn stamp_nonlinear_small_signal_real(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        frequency_hz: Value,
        physical_analysis: SmallSignalAnalysisKind,
    ) -> Result<(), SimulationError> {
        // The analysis identity remains part of this shared stamping contract
        // even when neither Verilog-A backend is compiled into the build.
        let _ = physical_analysis;

        struct AcRealStamper<'a> {
            matrix: &'a mut ComplexMatrix,
        }

        impl MatrixStamper for AcRealStamper<'_> {
            #[inline]
            fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
                if row > 0 && col > 0 {
                    self.matrix.add_real(row - 1, col - 1, value);
                }
            }

            #[inline]
            fn stamp_rhs(&mut self, _index: NodeId, _value: Value) {
                // AC uses only Jacobian matrix terms from nonlinear devices.
            }
        }

        let mut stamper = AcRealStamper { matrix };
        let mut rhs_dummy: [Value; 0] = [];
        circuit
            .diodes
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        for bjt in &circuit.bjts.devices {
            bjt.stamp_small_signal_ac(op_voltages, &mut stamper);
        }
        circuit
            .mosfets
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .b3soi
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .b3soi_fd
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .b3soi_pd
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // BSIM3: the DC linearization at the operating point is the real
        // part of the small-signal admittance (b3acld.c stamps the same
        // gm/gds/gmbs/gbd/gbs/substrate-current groups as the DC load).
        circuit
            .bsim3v3
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // BSIM4: identical discipline (b4acld.c repeats the DC
        // conductance groups, GIDL/GISL included, on the real axis).
        circuit
            .bsim4v8
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // EKV26 native AC uses the DC current Jacobian for real small-signal
        // conductances and the intrinsic terminal-charge Jacobian below.
        circuit
            .ekv26s
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        // EKV3 uses its ekv3_rf external DC derivatives as the low-frequency
        // real small-signal term; the VANOISE fixture applies cancellation and
        // frequency shaping in `stamp_nonlinear_capacitances`.
        circuit
            .ekv3s
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        circuit
            .vdmoses
            .stamp_all(&mut stamper, &mut rhs_dummy, op_voltages);
        for jfet in &circuit.jfets {
            jfet.stamp_small_signal_ac(op_voltages, frequency_hz, &mut stamper)
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "JFET '{}' AC feedback failed at {frequency_hz:.16e} Hz: {error}",
                        jfet.name
                    ))
                })?;
        }
        for sw in &circuit.vswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.iswitches {
            sw.stamp_nonlinear(op_voltages, &mut stamper, &mut rhs_dummy);
        }
        for sw in &circuit.generic_switches {
            sw.stamp_current_conductance(&mut stamper);
        }
        #[cfg(feature = "veriloga")]
        {
            let omega = 2.0 * std::f64::consts::PI * frequency_hz;
            for device in circuit.veriloga_devices().iter() {
                // Small-signal linearization uses Jacobian terms at the
                // operating point. Verilog-A device stamping exposes the
                // Jacobian through matrix callbacks.
                let mut cloned = device.clone();
                let device_name = cloned.name.to_string();
                cloned
                    .try_set_analysis_type(physical_analysis.runtime_code())
                    .map_err(|err| {
                        SimulationError::Circuit(format!(
                            "Verilog-A device '{device_name}' small-signal analysis setup failed: {err}"
                        ))
                    })?;
                cloned
                    .try_stamp(
                        op_voltages,
                        |row, col, value| matrix.add_real(row, col, value),
                        |_index, _value| {},
                    )
                    .map_err(|err| {
                        SimulationError::Circuit(format!(
                            "Verilog-A device '{device_name}' small-signal stamping failed: {err}"
                        ))
                    })?;
                let device_name = cloned.name.to_string();
                // Reactive (ddt charge/flux) part: jw * dQ/dx
                cloned
                    .try_stamp_reactive(op_voltages, |row, col, charge_deriv| {
                        matrix.add_imag(row, col, omega * charge_deriv);
                    })
                    .map_err(|err| {
                        SimulationError::Circuit(format!(
                            "Verilog-A device '{device_name}' small-signal reactive stamping failed: {err}"
                        ))
                    })?;
            }
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if circuit.has_generated_veriloga_devices() {
            let omega = 2.0 * std::f64::consts::PI * frequency_hz;
            let mut generated = circuit.generated_veriloga_devices().clone();
            let num_nodes = circuit.num_nodes();
            let simparams = circuit.generated_simulation_parameters;
            generated.set_timepoint(
                0.0,
                0.0,
                crate::device::veriloga_builtins::GeneratedDdtCoefficients::inactive(),
            );
            generated
                .stamp_small_signal_real_all(
                    matrix,
                    op_voltages,
                    num_nodes,
                    physical_analysis.generated_kind(),
                    simparams,
                )
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
            generated
                .stamp_small_signal_reactive_all(
                    matrix,
                    op_voltages,
                    num_nodes,
                    omega,
                    physical_analysis.generated_kind(),
                    simparams,
                )
                .map_err(|error| SimulationError::Circuit(error.to_string()))?;
        }
        Ok(())
    }

    #[inline]
    fn stamp_nonlinear_capacitances(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        // Diode junction capacitance Cj(Vd) + diffusion capacitance.
        for diode in &circuit.diodes.devices {
            let va = Self::ac_node_voltage(op_voltages, diode.node_anode);
            let vc = Self::ac_node_voltage(op_voltages, diode.node_cathode);
            let c = diode.junction_capacitance(va - vc);
            if c.is_finite() && c > 0.0 {
                Self::stamp_imag_two_terminal(
                    matrix,
                    diode.node_anode,
                    diode.node_cathode,
                    omega * c,
                );
            }
        }

        // JFET gate-source and gate-drain depletion capacitances.
        for jfet in &circuit.jfets {
            let vd = Self::ac_node_voltage(op_voltages, jfet.drain);
            let vg = Self::ac_node_voltage(op_voltages, jfet.gate);
            let vs = Self::ac_node_voltage(op_voltages, jfet.source);
            let (cgs, cgd, cds) =
                jfet.ac_capacitances(vg - vs, vg - vd, jfet.analysis_temperature());

            if cgs.is_finite() && cgs > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.gate, jfet.source, omega * cgs);
            }
            if cgd.is_finite() && cgd > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.gate, jfet.drain, omega * cgd);
            }
            if cds.is_finite() && cds > 0.0 {
                Self::stamp_imag_two_terminal(matrix, jfet.drain, jfet.source, omega * cds);
            }
        }

        // VDMOS gate, drain-source, and body-junction capacitances at the operating point.
        for vdmos in &circuit.vdmoses.devices {
            if vdmos.xyce_level18 {
                continue;
            }
            let drain = vdmos.drain_int.unwrap_or(vdmos.drain);
            let source = vdmos.source_int.unwrap_or(vdmos.source);
            // `Vdmos::capacitances` consumes the polarity-normalized charge
            // biases used by the transient path.  Feeding physical terminal
            // voltages directly happens to be correct for N-channel parts,
            // but reverses the nonlinear Cgd law for P-channel parts and
            // selects CGDMAX under reverse drain bias.  Reuse the canonical
            // charge-coordinate conversion so AC and transient linearize the
            // same physical device.
            let (vgs, _vgd, _vgb, vds) = vdmos.transient_charge_branch_voltages_at(op_voltages);
            let (cgs, cgd, cds) = vdmos.capacitances(vgs, vds);
            let cgb = vdmos.gate_bulk_capacitance();
            let (vbs, vbd) = vdmos.body_charge_branch_voltages_at(op_voltages);
            let (_, cbs) = vdmos.body_source_charge_and_capacitance_at(vbs);
            let (_, cbd) = vdmos.body_drain_charge_and_capacitance_at(vbd);
            let d1_vds = vdmos.d1_charge_branch_voltage_at(op_voltages);
            let (_, cd1) = vdmos.d1_charge_and_capacitance_at(d1_vds);

            if cgs.is_finite() && cgs > 0.0 {
                Self::stamp_imag_two_terminal(matrix, vdmos.gate, source, omega * cgs);
            }
            if cgd.is_finite() && cgd > 0.0 {
                Self::stamp_imag_two_terminal(matrix, vdmos.gate, drain, omega * cgd);
            }
            if cgb.is_finite() && cgb > 0.0 {
                Self::stamp_imag_two_terminal(matrix, vdmos.gate, vdmos.bulk, omega * cgb);
            }
            if cds.is_finite() && cds > 0.0 {
                Self::stamp_imag_two_terminal(matrix, drain, source, omega * cds);
            }
            if cbs.is_finite() && cbs > 0.0 {
                let (pos, neg) = vdmos.body_source_charge_nodes();
                Self::stamp_imag_two_terminal(matrix, pos, neg, omega * cbs);
            }
            if cbd.is_finite() && cbd > 0.0 {
                let (pos, neg) = vdmos.body_drain_charge_nodes();
                Self::stamp_imag_two_terminal(matrix, pos, neg, omega * cbd);
            }
            if cd1.is_finite() && cd1 > 0.0 {
                let (pos, neg) = vdmos.d1_charge_nodes();
                Self::stamp_imag_two_terminal(matrix, pos, neg, omega * cd1);
            }
        }
    }

    #[inline]
    fn stamp_bsim3_ac_nqs_corrections(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        if omega == 0.0 || circuit.bsim3v3.is_empty() {
            return;
        }
        for dev in &circuit.bsim3v3.devices {
            if !dev.uses_ac_nqs() {
                continue;
            }
            let (charge, mode) = dev.charge_at(op_voltages);
            dev.stamp_ac_nqs_correction(&charge, mode, omega, |row, col, value| {
                if row > 0 && col > 0 {
                    matrix.add(row - 1, col - 1, value);
                }
            });
        }
    }

    #[inline]
    fn stamp_bsim4_ac_nqs_corrections(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        omega: Value,
    ) {
        if omega == 0.0 || circuit.bsim4v8.is_empty() {
            return;
        }
        for dev in &circuit.bsim4v8.devices {
            if !dev.uses_ac_nqs() {
                continue;
            }
            let (charge, mode) = dev.charge_at(op_voltages);
            dev.stamp_ac_nqs_correction(&charge, mode, omega, |row, col, value| {
                if row > 0 && col > 0 {
                    matrix.add(row - 1, col - 1, value);
                }
            });
        }
    }

    #[inline]
    fn stamp_bsim4_trnqs_ac_charge_node_anchors(matrix: &mut ComplexMatrix, circuit: &CircuitData) {
        if circuit.bsim4v8.is_empty() {
            return;
        }
        for dev in &circuit.bsim4v8.devices {
            dev.stamp_trnqs_ac_charge_node_anchor_delta(|row, col, value| {
                if row > 0 && col > 0 {
                    matrix.add(row - 1, col - 1, value);
                }
            });
        }
    }

    #[inline]
    fn stamp_imag_matrix_entry(matrix: &mut ComplexMatrix, row: NodeId, col: NodeId, value: Value) {
        if row > 0 && col > 0 {
            matrix.add_imag(row - 1, col - 1, value);
        }
    }

    #[inline]
    fn stamp_jfet_ac_imag_feedback(
        matrix: &mut ComplexMatrix,
        circuit: &CircuitData,
        op_voltages: &[Value],
        frequency_hz: Value,
    ) -> Result<(), SimulationError> {
        for jfet in &circuit.jfets {
            let Some((xgm, xgds)) = jfet
                .ac_imag_feedback_terms_at_frequency(op_voltages, frequency_hz)
                .map_err(|error| {
                    SimulationError::Circuit(format!(
                        "JFET '{}' AC feedback failed at {frequency_hz:.16e} Hz: {error}",
                        jfet.name
                    ))
                })?
            else {
                continue;
            };

            Self::stamp_imag_matrix_entry(matrix, jfet.drain, jfet.drain, xgds);
            Self::stamp_imag_matrix_entry(matrix, jfet.drain, jfet.gate, xgm);
            Self::stamp_imag_matrix_entry(matrix, jfet.drain, jfet.source, -xgds - xgm);
            Self::stamp_imag_matrix_entry(matrix, jfet.source, jfet.drain, -xgds);
            Self::stamp_imag_matrix_entry(matrix, jfet.source, jfet.gate, -xgm);
            Self::stamp_imag_matrix_entry(matrix, jfet.source, jfet.source, xgds + xgm);
        }
        Ok(())
    }

    /// Refill a complex AC workspace in place for one frequency. The
    /// workspace keeps its sparsity pattern and shared symbolic
    /// factorization across calls, so a sweep pays the structure cost once
    /// instead of once per point.
    pub(super) fn try_fill_small_signal_matrix_with_vbic_delay_mode(
        circuit: &CircuitData,
        ac_matrix: &mut ComplexMatrix,
        op_voltages: &[Value],
        omega: Value,
        physical_analysis: SmallSignalAnalysisKind,
        include_vbic_dynamic_stamp: bool,
        include_vbic_delay_branches: bool,
    ) -> Result<(), SimulationError> {
        let has_nonlinear = circuit.has_nonlinear_devices();
        let frequency_hz = omega / (2.0 * PI);
        ac_matrix.clear_values();

        // Stamp resistors (real conductance)
        for (r_idx, stamp) in circuit.resistors.stamps.iter().enumerate() {
            let g = circuit.resistors.small_signal_conductance(r_idx);

            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                ac_matrix.add_real(stamp.pp.row - 1, stamp.pp.col - 1, g);
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                ac_matrix.add_real(stamp.pn.row - 1, stamp.pn.col - 1, -g);
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                ac_matrix.add_real(stamp.np.row - 1, stamp.np.col - 1, -g);
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                ac_matrix.add_real(stamp.nn.row - 1, stamp.nn.col - 1, g);
            }
        }
        if circuit.global_shunt_conductance != 0.0 {
            for index in 0..circuit.num_nodes() {
                if !circuit.is_non_electrical_state_matrix_index(index) {
                    ac_matrix.add_real(index, index, circuit.global_shunt_conductance);
                }
            }
        }

        // Stamp transmission lines. Native LTRA/TXL lines carry branch
        // unknowns whose rows only the branch-form loads can fill -- the
        // nodal Y-parameter stamp would land on absent matrix cells and
        // leave the branch equations singular (dead far port).
        for tline in &circuit.tlines {
            if let Some((br1, br2)) = tline.zero_length_branch_matrix_indices() {
                // Xyce's zero-length RC/RG special cases are exact ideal
                // through connections in small signal as well as transient.
                Self::stamp_txl_branch_ac(ac_matrix, tline, br1, br2);
                continue;
            }
            if let Some((br1, br2)) = tline.ltra_branch_matrix_indices()
                && Self::stamp_ltra_branch_ac(ac_matrix, tline, br1, br2, omega)
            {
                continue;
            }
            if let Some((br1, br2)) = tline.txl_branch_matrix_indices() {
                Self::stamp_txl_branch_ac(ac_matrix, tline, br1, br2);
                continue;
            }
            Self::stamp_transmission_line_ac(ac_matrix, tline, omega);
        }

        // Nonlinear device Jacobian (real part) evaluated at DC operating point.
        if has_nonlinear {
            Self::stamp_nonlinear_small_signal_real(
                ac_matrix,
                circuit,
                op_voltages,
                frequency_hz,
                physical_analysis,
            )?;
            if include_vbic_dynamic_stamp {
                for bjt in &circuit.bjts.devices {
                    Self::stamp_bjt_dynamic_ac(
                        ac_matrix,
                        bjt,
                        op_voltages,
                        omega,
                        include_vbic_delay_branches,
                    );
                }
            }
        }

        // Stamp capacitors: jωC
        for (i, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let c = circuit
                .capacitors
                .capacitances
                .get(i)
                .copied()
                .unwrap_or(0.0);
            let jwc = omega * c;

            if stamp.pp.row > 0 && stamp.pp.col > 0 {
                ac_matrix.add_imag(stamp.pp.row - 1, stamp.pp.col - 1, jwc);
            }
            if stamp.pn.row > 0 && stamp.pn.col > 0 {
                ac_matrix.add_imag(stamp.pn.row - 1, stamp.pn.col - 1, -jwc);
            }
            if stamp.np.row > 0 && stamp.np.col > 0 {
                ac_matrix.add_imag(stamp.np.row - 1, stamp.np.col - 1, -jwc);
            }
            if stamp.nn.row > 0 && stamp.nn.col > 0 {
                ac_matrix.add_imag(stamp.nn.row - 1, stamp.nn.col - 1, jwc);
            }
            if let Some(branch_ordinal) = circuit.capacitors.ic_branch_indices[i] {
                let branch = circuit.get_branch_matrix_index(branch_ordinal) - 1;
                if stamp.pp.row > 0 {
                    ac_matrix.add_imag(branch, stamp.pp.row - 1, -jwc);
                }
                if stamp.nn.row > 0 {
                    ac_matrix.add_imag(branch, stamp.nn.row - 1, jwc);
                }
            }
        }
        // The extra branch allocated for a Xyce capacitor IC becomes the
        // small-signal lead-current observation equation. Its unit diagonal
        // combines with the -jωC/+jωC branch-row terms stamped above.
        for branch_ordinal in circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .flatten()
            .copied()
        {
            let branch = circuit.get_branch_matrix_index(branch_ordinal);
            ac_matrix.add_real(branch - 1, branch - 1, 1.0);
        }

        // Nonlinear semiconductor junction capacitances at the operating point.
        if has_nonlinear {
            Self::stamp_nonlinear_capacitances(ac_matrix, circuit, op_voltages, omega);
            Self::stamp_jfet_ac_imag_feedback(ac_matrix, circuit, op_voltages, frequency_hz)?;
        }

        // Stamp MOSFET capacitances: jωCgs, jωCgd, jωCgb (Meyer model)
        for mos in &circuit.mosfets.devices {
            let (cgs, cgd, cgb) = mos.ac_capacitances();
            let ng = mos.node_gate;
            let nd = mos.node_drain;
            let ns = mos.node_source;
            let nb = mos.node_bulk;

            let jwcgs = omega * cgs;
            if ng > 0 {
                ac_matrix.add_imag(ng - 1, ng - 1, jwcgs);
            }
            if ng > 0 && ns > 0 {
                ac_matrix.add_imag(ng - 1, ns - 1, -jwcgs);
            }
            if ns > 0 && ng > 0 {
                ac_matrix.add_imag(ns - 1, ng - 1, -jwcgs);
            }
            if ns > 0 {
                ac_matrix.add_imag(ns - 1, ns - 1, jwcgs);
            }

            let jwcgd = omega * cgd;
            if ng > 0 {
                ac_matrix.add_imag(ng - 1, ng - 1, jwcgd);
            }
            if ng > 0 && nd > 0 {
                ac_matrix.add_imag(ng - 1, nd - 1, -jwcgd);
            }
            if nd > 0 && ng > 0 {
                ac_matrix.add_imag(nd - 1, ng - 1, -jwcgd);
            }
            if nd > 0 {
                ac_matrix.add_imag(nd - 1, nd - 1, jwcgd);
            }

            let jwcgb = omega * cgb;
            if ng > 0 {
                ac_matrix.add_imag(ng - 1, ng - 1, jwcgb);
            }
            if ng > 0 && nb > 0 {
                ac_matrix.add_imag(ng - 1, nb - 1, -jwcgb);
            }
            if nb > 0 && ng > 0 {
                ac_matrix.add_imag(nb - 1, ng - 1, -jwcgb);
            }
            if nb > 0 {
                ac_matrix.add_imag(nb - 1, nb - 1, jwcgb);
            }

            let (_vgs_eval, vds_eval, vbs_eval) = mos.eval_branch_voltages_at(op_voltages);
            let (_, cbs) = mos.body_source_junction_charge_and_capacitance_at(vbs_eval);
            if cbs.is_finite() && cbs > 0.0 {
                let (pos, neg) = mos.body_source_charge_nodes();
                Self::stamp_imag_two_terminal(ac_matrix, pos, neg, omega * cbs);
            }

            let (_, cbd) = mos.body_drain_junction_charge_and_capacitance_at(vds_eval, vbs_eval);
            if cbd.is_finite() && cbd > 0.0 {
                let (pos, neg) = mos.body_drain_charge_nodes();
                Self::stamp_imag_two_terminal(ac_matrix, pos, neg, omega * cbd);
            }
        }

        // B3SOI/BSIM3/BSIM4 coupled capacitance matrices: the mode-assembled
        // gc** blocks evaluated at the operating point, times jw — exactly the
        // xc*** entries of each model's AC load path (nqsMod = 0 for BSIM4).
        if omega != 0.0
            && (!circuit.b3soi.is_empty()
                || !circuit.b3soi_fd.is_empty()
                || !circuit.b3soi_pd.is_empty()
                || !circuit.bsim3v3.is_empty()
                || !circuit.bsim4v8.is_empty()
                || !circuit.ekv26s.is_empty()
                || !circuit.ekv3s.is_empty())
        {
            let mut stamper = AcImagStamper { matrix: ac_matrix };
            for dev in &circuit.b3soi.devices {
                if dev.charges_suppressed() {
                    continue;
                }
                let charge = dev.charge_at(op_voltages);
                dev.stamp_charge_companion(
                    &charge,
                    omega,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    op_voltages,
                    &mut stamper,
                );
            }
            for dev in &circuit.b3soi_fd.devices {
                if dev.charges_suppressed() {
                    continue;
                }
                let charge = dev.charge_at(op_voltages);
                dev.stamp_charge_companion(
                    &charge,
                    omega,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    op_voltages,
                    &mut stamper,
                );
            }
            for dev in &circuit.b3soi_pd.devices {
                if dev.charges_suppressed() {
                    continue;
                }
                let charge = dev.charge_at(op_voltages);
                dev.stamp_charge_companion(
                    &charge,
                    omega,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    op_voltages,
                    &mut stamper,
                );
            }
            for dev in &circuit.bsim3v3.devices {
                let (charge, mode) = dev.charge_at(op_voltages);
                let gc = crate::device::Bsim3v3Device::charge_matrix(&charge, mode);
                dev.stamp_charge_matrix(&gc, omega, &mut stamper);
            }
            for dev in &circuit.bsim4v8.devices {
                let (charge, mode) = dev.charge_at(op_voltages);
                dev.stamp_ac_charge_matrix(&charge, mode, omega, &mut stamper);
            }
            for dev in &circuit.ekv26s.devices {
                dev.stamp_ac_quasi_static_charge_matrix(op_voltages, omega, &mut stamper);
            }
        }
        if omega != 0.0 && !circuit.ekv3s.is_empty() {
            let frequency_hz = omega / (2.0 * PI);
            for dev in &circuit.ekv3s.devices {
                dev.stamp_ac_transadmittance_delta(frequency_hz, |row, col, value| {
                    ac_matrix.add_real(row - 1, col - 1, value);
                });
            }
        }
        Self::stamp_bsim3_ac_nqs_corrections(ac_matrix, circuit, op_voltages, omega);
        Self::stamp_bsim4_ac_nqs_corrections(ac_matrix, circuit, op_voltages, omega);
        Self::stamp_bsim4_trnqs_ac_charge_node_anchors(ac_matrix, circuit);

        // Voltage sources for AC (MNA branch equations)
        for i in 0..circuit.voltage_sources.len() {
            let np = circuit.voltage_sources.node_pos[i];
            let nn = circuit.voltage_sources.node_neg[i];
            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
        }

        // Branch-form resistors for AC:
        // V(np)-V(nn)-R_ac*I = 0.
        for i in 0..circuit.resistor_branches.len() {
            let np = circuit.resistor_branches.node_pos[i];
            let nn = circuit.resistor_branches.node_neg[i];
            let br_ordinal = circuit.resistor_branches.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let resistance = circuit.resistor_branches.small_signal_resistances[i];

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            ac_matrix.add_real(br - 1, br - 1, -resistance);
        }

        // Inductors for AC:
        // V(np)-V(nn)-jωL*I = 0
        for i in 0..circuit.inductors.len() {
            let np = circuit.inductors.node_pos[i];
            let nn = circuit.inductors.node_neg[i];
            let br_ordinal = circuit.inductors.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let l = circuit.inductors.inductances[i];

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            ac_matrix.add_imag(br - 1, br - 1, -omega * l);
        }

        // Mutual coupling (K elements) for AC: the standalone inductors above
        // carry the self terms; each pair adds the -jwM cross terms.
        for binding in &circuit.coupled_inductor_pairs {
            let br1 = circuit.get_branch_matrix_index(binding.branch1_ordinal);
            let br2 = circuit.get_branch_matrix_index(binding.branch2_ordinal);
            let m = binding.device.m;
            ac_matrix.add_imag(br1 - 1, br2 - 1, -omega * m);
            ac_matrix.add_imag(br2 - 1, br1 - 1, -omega * m);
        }

        // Controlled sources: VCVS
        for i in 0..circuit.vcvs.len() {
            let np = circuit.vcvs.node_pos[i];
            let nn = circuit.vcvs.node_neg[i];
            let cp = circuit.vcvs.ctrl_pos[i];
            let cn = circuit.vcvs.ctrl_neg[i];
            let br_ordinal = circuit.vcvs.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let gain = circuit.vcvs.gains[i];

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            if cp > 0 {
                ac_matrix.add_real(br - 1, cp - 1, -gain);
            }
            if cn > 0 {
                ac_matrix.add_real(br - 1, cn - 1, gain);
            }
        }

        // Controlled sources: VCCS
        for i in 0..circuit.vccs.len() {
            let np = circuit.vccs.node_pos[i];
            let nn = circuit.vccs.node_neg[i];
            let cp = circuit.vccs.ctrl_pos[i];
            let cn = circuit.vccs.ctrl_neg[i];
            let gm = circuit.vccs.transconductances[i];

            if np > 0 && cp > 0 {
                ac_matrix.add_real(np - 1, cp - 1, gm);
            }
            if np > 0 && cn > 0 {
                ac_matrix.add_real(np - 1, cn - 1, -gm);
            }
            if nn > 0 && cp > 0 {
                ac_matrix.add_real(nn - 1, cp - 1, -gm);
            }
            if nn > 0 && cn > 0 {
                ac_matrix.add_real(nn - 1, cn - 1, gm);
            }
        }

        // Controlled sources: CCCS
        for i in 0..circuit.cccs.len() {
            let np = circuit.cccs.node_pos[i];
            let nn = circuit.cccs.node_neg[i];
            let ctrl_branch_ordinal = circuit.cccs.ctrl_branch[i];
            let gain = circuit.cccs.gains[i];
            if ctrl_branch_ordinal == 0 {
                continue;
            }
            let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal);

            if np > 0 {
                ac_matrix.add_real(np - 1, cb - 1, gain);
            }
            if nn > 0 {
                ac_matrix.add_real(nn - 1, cb - 1, -gain);
            }
        }

        // Controlled sources: CCVS
        for i in 0..circuit.ccvs.len() {
            let np = circuit.ccvs.node_pos[i];
            let nn = circuit.ccvs.node_neg[i];
            let br_ordinal = circuit.ccvs.branch_indices[i];
            let ctrl_branch_ordinal = circuit.ccvs.ctrl_branch[i];
            let rm = circuit.ccvs.transresistances[i];
            if br_ordinal == 0 || ctrl_branch_ordinal == 0 {
                continue;
            }
            let br = circuit.get_branch_matrix_index(br_ordinal);
            let cb = circuit.get_branch_matrix_index(ctrl_branch_ordinal);

            if np > 0 {
                ac_matrix.add_real(br - 1, np - 1, 1.0);
                ac_matrix.add_real(np - 1, br - 1, 1.0);
            }
            if nn > 0 {
                ac_matrix.add_real(br - 1, nn - 1, -1.0);
                ac_matrix.add_real(nn - 1, br - 1, -1.0);
            }
            ac_matrix.add_real(br - 1, cb - 1, -rm);
        }

        // Behavioral sources: small-signal linearization at the DC
        // operating point and active frequency. The per-point caller
        // refreshes these partials so Xyce FREQ/HERTZ expressions remain
        // live; sign conventions mirror the DC stamps exactly.
        for source in &circuit.behavioral_sources.voltage_sources {
            let np = source.node_pos;
            let nn = source.node_neg;
            let br = circuit.get_branch_matrix_index(source.branch_ordinal);

            if np > 0 {
                try_stamp_behavioral_ac_coefficient(
                    ac_matrix,
                    br - 1,
                    np - 1,
                    1.0,
                    "voltage",
                    &source.name,
                    frequency_hz,
                )?;
                try_stamp_behavioral_ac_coefficient(
                    ac_matrix,
                    np - 1,
                    br - 1,
                    1.0,
                    "voltage",
                    &source.name,
                    frequency_hz,
                )?;
            }
            if nn > 0 {
                try_stamp_behavioral_ac_coefficient(
                    ac_matrix,
                    br - 1,
                    nn - 1,
                    -1.0,
                    "voltage",
                    &source.name,
                    frequency_hz,
                )?;
                try_stamp_behavioral_ac_coefficient(
                    ac_matrix,
                    nn - 1,
                    br - 1,
                    -1.0,
                    "voltage",
                    &source.name,
                    frequency_hz,
                )?;
            }
            // Branch row: V(np) - V(nn) - Σ (df/dx)·x = 0
            for (global_idx, df) in source.linearized_partials() {
                if df != 0.0 {
                    try_stamp_behavioral_ac_coefficient(
                        ac_matrix,
                        br - 1,
                        global_idx,
                        -df,
                        "voltage",
                        &source.name,
                        frequency_hz,
                    )?;
                }
            }
        }
        for source in &circuit.behavioral_sources.current_sources {
            let np = source.node_pos;
            let nn = source.node_neg;
            // KCL rows: I flows np -> nn, linearized I ≈ Σ (df/dx)·x.
            for (global_idx, df) in source.linearized_partials() {
                if df == 0.0 {
                    continue;
                }
                if np > 0 {
                    try_stamp_behavioral_ac_coefficient(
                        ac_matrix,
                        np - 1,
                        global_idx,
                        df,
                        "current",
                        &source.name,
                        frequency_hz,
                    )?;
                }
                if nn > 0 {
                    try_stamp_behavioral_ac_coefficient(
                        ac_matrix,
                        nn - 1,
                        global_idx,
                        -df,
                        "current",
                        &source.name,
                        frequency_hz,
                    )?;
                }
            }
        }

        Self::stamp_xspice_small_signal_ac(circuit, ac_matrix, frequency_hz);
        Ok(())
    }

    pub(super) fn try_build_small_signal_ac_matrix(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
    ) -> Result<ComplexMatrix, SimulationError> {
        // ngspice-46 includes the VBIC excess-phase network in small-signal
        // analysis: vbicacld.c stamps the full Ixf static coupling and the
        // cqxf1/cqxf2 charges (times omega) onto the xf rows. The delayed
        // transport therefore shapes AC and noise transfers above ~1/TD,
        // and the official binary fails the pre-xf 2005 AC tables by over
        // 1 dB at 10 GHz on the CEamp deck.
        let mut ac_matrix = ComplexMatrix::from_real_structure(matrix);
        Self::try_fill_small_signal_matrix_with_vbic_delay_mode(
            circuit,
            &mut ac_matrix,
            op_voltages,
            omega,
            SmallSignalAnalysisKind::Ac,
            true,
            true,
        )?;
        Ok(ac_matrix)
    }

    /// Build the small-signal operator after reevaluating a private circuit
    /// clone at an arbitrary nearby state. The caller's converged circuit and
    /// all of its device caches remain untouched.
    pub(super) fn try_build_small_signal_ac_matrix_at_state(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        operating_state: &[Value],
        omega: Value,
    ) -> Result<ComplexMatrix, SimulationError> {
        let mut state_circuit = circuit.clone();
        Self::prepare_small_signal_state_at_frequency(
            &mut state_circuit,
            operating_state,
            omega / (2.0 * PI),
        )?;
        Self::try_build_small_signal_ac_matrix(&state_circuit, matrix, operating_state, omega)
    }

    pub(super) fn try_build_small_signal_pz_matrix(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        op_voltages: &[Value],
        omega: Value,
    ) -> Result<ComplexMatrix, SimulationError> {
        // PZ descriptor construction handles VBIC hidden dynamic states
        // explicitly in `engine/advanced/mod.rs`, so keep the base AC
        // linearization free of frequency-dependent VBIC companion reduction.
        let mut ac_matrix = ComplexMatrix::from_real_structure(matrix);
        Self::try_fill_small_signal_matrix_with_vbic_delay_mode(
            circuit,
            &mut ac_matrix,
            op_voltages,
            omega,
            SmallSignalAnalysisKind::Ac,
            false,
            true,
        )?;
        Ok(ac_matrix)
    }

    pub(super) fn build_ac_excitation_rhs(circuit: &CircuitData) -> Vec<Complex64> {
        let size = circuit.matrix_size();
        let mut rhs = vec![Complex64::new(0.0, 0.0); size];

        // Independent voltage sources with AC specification.
        for i in 0..circuit.voltage_sources.len() {
            let excitation = circuit.voltage_sources.ac_excitation(i);
            if excitation == Complex64::new(0.0, 0.0) {
                continue;
            }

            let br_ordinal = circuit.voltage_sources.branch_indices[i];
            let br = circuit.get_branch_matrix_index(br_ordinal);
            rhs[br - 1] = excitation;
        }

        // Independent current sources with AC specification.
        for i in 0..circuit.current_sources.len() {
            let ac_mag = circuit.current_sources.ac_magnitudes[i];
            let ac_phase = circuit.current_sources.ac_phases[i];
            if ac_mag.abs() <= 1e-15 {
                continue;
            }

            let i_ac = Complex64::from_polar(ac_mag, ac_phase);
            let np = circuit.current_sources.node_pos[i];
            let nn = circuit.current_sources.node_neg[i];

            if np > 0 {
                rhs[np - 1] -= i_ac;
            }
            if nn > 0 {
                rhs[nn - 1] += i_ac;
            }
        }

        rhs
    }

    /// Run AC small-signal analysis
    ///
    /// Linearizes circuit at DC operating point, then solves at each frequency.
    /// When the `parallel` feature is enabled and there are many frequency points,
    /// the frequency sweep is parallelized for better performance.
    pub fn run_ac(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
    ) -> Result<Vec<AcResult>, SimulationError> {
        self.run_ac_with_abort(netlist, frequencies, &NoAbort)
    }

    /// Run an AC sweep with cooperative cancellation during the operating
    /// point and between independent frequency solves.
    pub fn run_ac_with_abort(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<AcResult>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        validate_ac_frequencies(frequencies)?;
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(frequencies.len())?;
        let mut circuit = engine.build_circuit_with_abort(netlist, abort)?;
        if circuit.num_nodes() == 0 && circuit.num_branches() == 0 {
            engine.ensure_result_shape(frequencies.len(), 1)?;
            return Ok(frequencies
                .iter()
                .map(|&frequency| AcResult {
                    frequency,
                    node_names: Vec::new(),
                    branch_names: Vec::new(),
                    voltages: Vec::new(),
                    currents: Vec::new(),
                })
                .collect());
        }
        // Coupled multiconductor lines have no small-signal load (ngspice's
        // CPL registers none and its AC solve fails with a singular matrix);
        // refuse explicitly instead of returning silently dead ports.
        if !circuit.coupled_tlines.is_empty() {
            return Err(SimulationError::Circuit(
                "AC analysis does not support coupled multiconductor (CPL) transmission lines"
                    .to_string(),
            ));
        }
        Self::ensure_supported_ac_dynamic_charges(&circuit)?;
        circuit
            .begin_veriloga_equilibrium_analysis(1)
            .map_err(SimulationError::Circuit)?;
        circuit
            .prepare_veriloga_equilibrium_analysis_point(1, true, false)
            .map_err(SimulationError::Circuit)?;
        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let ac_voltage_projection = AcVoltageConstraintProjection::new(&circuit)?;

        // Get DC operating point
        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = if circuit.can_use_zero_bias_for_explicit_xspice_ac() {
            log::debug!(
                "using zero-bias small-signal state for explicit XSPICE transmission-line AC"
            );
            vec![0.0; circuit.matrix_size()]
        } else {
            engine.solve_dc_operating_point_with_abort(netlist, &mut circuit, &mut matrix, abort)?
        };
        if has_nonlinear {
            engine.try_observe_dc_operating_point(&mut circuit, &mut matrix, &dc_solution)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }
        circuit
            .accept_veriloga_analysis_point()
            .map_err(SimulationError::Circuit)?;
        circuit
            .finish_veriloga_equilibrium_operating_point(1)
            .map_err(SimulationError::Circuit)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if has_nonlinear {
            // Align stateful nonlinear models (limited junction voltages,
            // operating region) with the final converged operating point.
            Self::prepare_small_signal_state(&mut circuit, &dc_solution)?;
        } else {
            // Behavioral source caches may still be present on an otherwise
            // linear circuit.
            circuit
                .prepare_behavioral_small_signal(&dc_solution)
                .map_err(SimulationError::Circuit)?;
        }

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        engine.ensure_result_shape(frequencies.len(), size.saturating_mul(2).saturating_add(1))?;
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();
        let ac_solve_denominator_floor = if ac_voltage_projection.is_empty() {
            None
        } else {
            let mut floors = vec![0.0; size];
            for &branch_ordinal in &circuit.voltage_sources.branch_indices {
                let branch = circuit.get_branch_matrix_index(branch_ordinal);
                let row = branch.checked_sub(1).ok_or_else(|| {
                    SolverError::InvalidCircuit(
                        "independent voltage source has no AC equation row".to_string(),
                    )
                })?;
                let Some(floor) = floors.get_mut(row) else {
                    return Err(SolverError::InvalidCircuit(
                        "independent voltage-source AC equation lies outside the solved system"
                            .to_string(),
                    )
                    .into());
                };
                // Homogeneous ideal-source rows can carry only roundoff-scale
                // leakage before their exact post-solve projection. Give
                // those known voltage equations the same one-volt coordinate
                // floor as the projection validator; every other MNA row
                // remains under the strict componentwise solve certificate.
                *floor = 1.0;
            }
            Some(floors)
        };

        // Closure to solve at a single frequency. Takes the circuit as a
        // parameter so the parallel path below can hand each worker its own
        // clone (device-evaluation caches are Cell-based and not Sync).
        let solve_at_freq = |circuit: &mut CircuitData,
                             ac_matrix: &mut ComplexMatrix,
                             freq: Value,
                             final_step: bool|
         -> Result<AcResult, SimulationError> {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let omega = 2.0 * PI * freq;
            circuit
                .prepare_veriloga_frequency_analysis_point(1, final_step)
                .map_err(SimulationError::Circuit)?;
            circuit
                .prepare_behavioral_small_signal_at_frequency(&dc_solution, freq)
                .map_err(SimulationError::Circuit)?;
            Self::try_fill_small_signal_matrix_with_vbic_delay_mode(
                circuit,
                ac_matrix,
                &dc_solution,
                omega,
                SmallSignalAnalysisKind::Ac,
                true,
                true,
            )?;
            let rhs = Self::build_ac_excitation_rhs(circuit);
            let sparse_solution = match ac_solve_denominator_floor.as_deref() {
                Some(floor) => ac_matrix.solve_with_row_denominator_floors(&rhs, floor),
                None => ac_matrix.solve(&rhs),
            };
            let mut solution = match sparse_solution {
                Ok(solution) => solution,
                Err(SolverError::InaccurateSolution(_)) if rhs.len() <= 64 => {
                    log::debug!(
                        "sparse AC solve failed strict backward-error certification; retrying the small complex system with extended precision"
                    );
                    ac_matrix
                        .solve_dense_extended(&rhs)
                        .map_err(SimulationError::Solver)?
                }
                Err(error) => return Err(SimulationError::Solver(error)),
            };
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if !ac_voltage_projection.is_empty() {
                ac_voltage_projection.project(&mut solution)?;
                ac_matrix
                    .certify_solution(&solution, &rhs)
                    .map_err(SimulationError::Solver)?;
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
            }

            let mut currents = if size > num_nodes {
                solution[num_nodes..].to_vec()
            } else {
                Vec::new()
            };
            circuit
                .capacitors
                .project_complex_ic_branch_currents(&solution, &mut currents, omega);

            Ok(AcResult {
                frequency: freq,
                node_names: node_names.clone(),
                branch_names: branch_names.clone(),
                voltages: solution[..num_nodes].to_vec(),
                currents,
            })
        };

        // Parallel sweep: every frequency point shares the same operating
        // point and matrix structure, so points are fully independent.
        // CircuitData is not Sync (Cell-based device-eval caches), so each
        // worker owns an independent clone paired with one contiguous chunk
        // of the sweep — no shared state, no locks, and chunk order
        // preserves output ordering. The caches are pure memoization, so
        // per-point results are identical to the sequential path. Lifecycle
        // state is evaluated only on the private point clone: the final
        // frequency is an observation boundary, not state consumed by a
        // later analysis point, so it is intentionally not accepted.
        #[cfg(feature = "parallel")]
        if frequencies.len() >= 10 {
            use rayon::prelude::*;

            let workers = self.parallel_worker_count(frequencies.len());
            if workers > 1 {
                let chunk_len = frequencies.len().div_ceil(workers);
                let work: Vec<(CircuitData, usize, &[Value])> = frequencies
                    .chunks(chunk_len)
                    .enumerate()
                    .map(|(chunk_index, chunk)| (circuit.clone(), chunk_index * chunk_len, chunk))
                    .collect();
                let chunk_results: Result<Vec<Vec<AcResult>>, SimulationError> = self
                    .install_parallel(|| {
                        work.into_par_iter()
                            .map(|(mut worker_circuit, chunk_start, chunk)| {
                                let mut workspace = ComplexMatrix::from_real_structure(&matrix);
                                chunk
                                    .iter()
                                    .enumerate()
                                    .map(|(chunk_offset, &freq)| {
                                        let final_step =
                                            chunk_start + chunk_offset + 1 == frequencies.len();
                                        solve_at_freq(
                                            &mut worker_circuit,
                                            &mut workspace,
                                            freq,
                                            final_step,
                                        )
                                    })
                                    .collect()
                            })
                            .collect()
                    })?;
                return chunk_results.map(|chunks| chunks.into_iter().flatten().collect());
            }
        }

        let mut workspace = ComplexMatrix::from_real_structure(&matrix);
        frequencies
            .iter()
            .enumerate()
            .map(|(index, &freq)| {
                solve_at_freq(
                    &mut circuit,
                    &mut workspace,
                    freq,
                    index + 1 == frequencies.len(),
                )
            })
            .collect()
    }

    /// Execute a Xyce-style table-driven AC analysis.
    ///
    /// Each validated row from `table_name` is applied to a fresh netlist and
    /// solved at its literal `FREQ`/`HERTZ` value. Returning the materialized
    /// row netlists alongside their results preserves the parameter state used
    /// for each point for callers that evaluate `.PRINT` expressions after
    /// solving.
    pub fn run_ac_data(
        &self,
        netlist: &Netlist,
        table_name: &str,
    ) -> Result<(Vec<Netlist>, Vec<AcResult>), SimulationError> {
        self.run_ac_data_with_abort(netlist, table_name, &NoAbort)
    }

    /// Cancellable variant of [`Engine::run_ac_data`].
    pub fn run_ac_data_with_abort(
        &self,
        netlist: &Netlist,
        table_name: &str,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Netlist>, Vec<AcResult>), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let points = netlist
            .frequency_data_table_points(table_name)
            .map_err(|error| SimulationError::Circuit(format!(".AC DATA {error}")))?;
        self.ensure_analysis_points(points.len())?;
        self.ensure_batch_runs(points.len())?;
        let override_plan = FrequencyDataOverridePlan::resolve(netlist, &points)?;
        let mut row_netlists = Vec::with_capacity(points.len());
        let mut results = Vec::with_capacity(points.len());
        for (row_index, point) in points.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let row_netlist =
                materialize_frequency_data_row_with_abort(netlist, &override_plan, point, abort)?;
            let mut row_results =
                self.run_ac_with_abort(&row_netlist, &[point.frequency], abort)?;
            if row_results.len() != 1 {
                return Err(SimulationError::Circuit(format!(
                    ".AC DATA table '{}' row {} produced {} results, expected one",
                    table_name,
                    row_index + 1,
                    row_results.len()
                )));
            }
            row_netlists.push(row_netlist);
            results.push(row_results.remove(0));
        }
        Ok((row_netlists, results))
    }
}

fn validate_ac_frequencies(frequencies: &[Value]) -> Result<(), SimulationError> {
    if frequencies.is_empty() {
        return Err(SimulationError::Circuit(
            "AC analysis requires at least one frequency point".to_string(),
        ));
    }

    if let Some((index, frequency)) = frequencies
        .iter()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || **frequency < 0.0)
    {
        return Err(SimulationError::Circuit(format!(
            "AC frequency at index {index} must be finite and non-negative, got {frequency}"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voltage_at(point: &AcResult, node_name: &str) -> Complex64 {
        let index = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node_name))
            .unwrap_or_else(|| panic!("missing node '{node_name}' in {:?}", point.node_names));
        point.voltages[index]
    }

    fn ac_deck() -> Netlist {
        Netlist::parse(
            "AC deck\n\
             V1 in 0 DC 0 AC 1\n\
             R1 in out 1k\n\
             C1 out 0 1u\n\
             .end\n",
        )
        .expect("deck parses")
    }

    #[test]
    fn small_signal_matrix_contains_only_authored_physical_terms() {
        let netlist = Netlist::parse(
            "Physical AC operator\n\
             V1 in 0 AC 1\n\
             R1 in 0 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::default();
        let mut circuit = engine.build_circuit(&netlist).expect("circuit builds");
        let matrix = engine.build_matrix(&circuit).expect("matrix builds");
        circuit.link_indices(&matrix);

        let ac_matrix = Engine::try_build_small_signal_ac_matrix(
            &circuit,
            &matrix,
            &vec![0.0; circuit.matrix_size()],
            2.0 * PI * 1.0e3,
        )
        .expect("small-signal matrix builds");
        let conductance = circuit.resistors.small_signal_conductance(0);

        assert_eq!(
            ac_matrix.to_dense_real(),
            vec![vec![conductance, 1.0], vec![1.0, 0.0]],
            "AC assembly must not add hidden node or branch diagonal terms"
        );
        assert_eq!(
            ac_matrix.to_dense_imag(),
            vec![vec![0.0, 0.0], vec![0.0, 0.0]]
        );
    }

    #[test]
    fn ac_rejects_empty_or_invalid_frequency_grid() {
        let netlist = ac_deck();
        let engine = Engine::default();

        let err = engine
            .run_ac(&netlist, &[])
            .expect_err("empty AC sweep must not report success");
        assert!(
            err.to_string().contains("frequency"),
            "unexpected error: {err}"
        );

        for freq in [f64::NAN, f64::INFINITY, -1.0] {
            let err = engine
                .run_ac(&netlist, &[freq])
                .expect_err("invalid AC frequency must not enter the solver");
            assert!(
                err.to_string().contains("finite") || err.to_string().contains("non-negative"),
                "unexpected error for freq={freq:?}: {err}"
            );
        }
    }

    #[test]
    fn ac_preserves_exact_grounded_source_phasors_in_short_and_parallel_sweeps() {
        let netlist = Netlist::parse(
            "Exact AC voltage constraint\n\
             V1 a 0 DC 0 AC 1\n\
             C1 a b 1m\n\
             R1 b 0 2\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::default();
        let expected = Complex64::from_polar(1.0, 0.0);

        for frequencies in [
            vec![1.0, 10.0],
            vec![
                1.0,
                1.5848931924611136,
                2.51188643150958,
                3.9810717055349722,
                6.309573444801933,
                10.0,
                15.848931924611133,
                25.118864315095795,
                39.810717055349734,
                63.09573444801933,
                100.0,
                158.48931924611142,
                251.18864315095797,
                398.1071705534973,
                630.957344480193,
                1000.0,
            ],
        ] {
            let results = engine
                .run_ac(&netlist, &frequencies)
                .expect("AC solve succeeds");
            for point in &results {
                assert_eq!(
                    voltage_at(point, "a"),
                    expected,
                    "ideal source drifted at {} Hz",
                    point.frequency
                );
            }
        }
    }

    #[test]
    fn ac_projects_source_trees_without_breaking_shared_node_constraints() {
        let netlist = Netlist::parse(
            "Stacked AC voltage constraints\n\
             V1 a 0 AC 1\n\
             V2 b a AC 2\n\
             R1 b 0 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let point = Engine::default()
            .run_ac(&netlist, &[1.0e3])
            .expect("AC solve succeeds")
            .remove(0);

        assert_eq!(voltage_at(&point, "a"), Complex64::new(1.0, 0.0));
        assert_eq!(voltage_at(&point, "b"), Complex64::new(3.0, 0.0));
    }

    #[test]
    fn ac_floating_source_projection_preserves_common_mode() {
        let netlist = Netlist::parse(
            "Floating AC voltage constraint\n\
             V1 p n AC 2\n\
             RP p 0 1k\n\
             RN n 0 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let point = Engine::default()
            .run_ac(&netlist, &[1.0e3])
            .expect("AC solve succeeds")
            .remove(0);
        let vp = voltage_at(&point, "p");
        let vn = voltage_at(&point, "n");

        assert_eq!(vp - vn, Complex64::new(2.0, 0.0));
        assert!(
            (vp + vn).norm() <= 16.0 * Value::EPSILON,
            "floating common mode changed: Vp={vp}, Vn={vn}"
        );
    }

    #[test]
    fn ac_zero_volt_probe_remains_exact_while_carrying_current() {
        let netlist = Netlist::parse(
            "Loaded zero-volt AC probe\n\
             I1 0 sense AC 1\n\
             VPROBE sense 0 0\n\
             .END\n",
        )
        .expect("deck parses");
        let point = Engine::default()
            .run_ac(&netlist, &[1.0e3])
            .expect("loaded zero-volt probe solves")
            .remove(0);

        assert_eq!(voltage_at(&point, "sense"), Complex64::new(0.0, 0.0));
        let probe_index = point
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("VPROBE"))
            .expect("probe branch is observable");
        assert!(
            (point.currents[probe_index].norm() - 1.0).abs() <= 16.0 * Value::EPSILON,
            "zero-volt probe did not carry the excitation current: {:?}",
            point.currents[probe_index]
        );
    }

    #[test]
    fn ac_complex_source_stack_preserves_every_constraint() {
        let netlist = Netlist::parse(
            "Complex stacked AC constraints\n\
             V1 a 0 AC 0.75 20\n\
             V2 b a AC 1.25 -35\n\
             R1 b 0 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let point = Engine::default()
            .run_ac(&netlist, &[1.0e3])
            .expect("complex source stack solves")
            .remove(0);
        let first = Complex64::from_polar(0.75, 20.0_f64.to_radians());
        let second = Complex64::from_polar(1.25, (-35.0_f64).to_radians());
        let va = voltage_at(&point, "a");
        let vb = voltage_at(&point, "b");

        assert_eq!(va, first);
        assert!(
            (vb - va - second).norm() <= 16.0 * Value::EPSILON * second.norm().max(1.0),
            "second stacked constraint drifted: Va={va}, Vb={vb}, target={second}"
        );
    }

    #[test]
    fn ac_projection_rejects_a_gross_raw_constraint_error() {
        let projection = AcVoltageConstraintProjection {
            constraints: vec![AcVoltageConstraint {
                node_pos: 1,
                node_neg: 0,
                target: Complex64::new(1.0, 0.0),
            }],
            components: vec![AcVoltageConstraintComponent {
                grounded: true,
                nodes: vec![(0, Complex64::new(0.0, 0.0)), (1, Complex64::new(1.0, 0.0))],
            }],
        };
        let mut solution = [Complex64::new(1.0 + 1.0e-8, 0.0)];

        assert!(matches!(
            projection.project(&mut solution),
            Err(SimulationError::Solver(SolverError::InaccurateSolution(_)))
        ));
        assert_eq!(solution, [Complex64::new(1.0 + 1.0e-8, 0.0)]);
    }

    #[test]
    fn floating_common_mode_mean_stays_finite_near_numeric_limits() {
        let magnitude = Value::MAX / 4.0;
        let solution = [
            Complex64::new(magnitude, magnitude),
            Complex64::new(magnitude, magnitude),
        ];
        let nodes = [(1, Complex64::new(0.0, 0.0)), (2, Complex64::new(0.0, 0.0))];

        assert_eq!(
            compensated_common_mode(&solution, &nodes).expect("finite mean"),
            Complex64::new(magnitude, magnitude)
        );
    }

    #[test]
    fn ac_grounded_phase_source_uses_the_exact_rhs_phasor() {
        let netlist = Netlist::parse(
            "Phased AC voltage constraint\n\
             V1 a 0 AC 2 30\n\
             R1 a 0 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let point = Engine::default()
            .run_ac(&netlist, &[1.0e3])
            .expect("AC solve succeeds")
            .remove(0);

        assert_eq!(
            voltage_at(&point, "a"),
            Complex64::from_polar(2.0, 30.0_f64.to_radians())
        );
    }

    #[test]
    fn ac_rejects_ideal_voltage_source_loops_before_projection() {
        let netlist = Netlist::parse(
            "Parallel ideal AC voltage sources\n\
             V1 a 0 AC 1\n\
             V2 a 0 AC 1\n\
             R1 a 0 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let error = Engine::default()
            .run_ac(&netlist, &[1.0e3])
            .expect_err("an ideal-source loop has no unique branch currents");

        assert!(
            error.to_string().contains("ideal-source loop"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn xyce_capacitor_ic_branch_reports_physical_ac_lead_current() {
        let netlist = Netlist::parse(
            "Xyce capacitor IC AC current\n\
             V1 in 0 DC 0 AC 1\n\
             R1 in out 1k\n\
             C1 out 0 1u IC=0\n\
             .AC LIN 1 1k 1k\n\
             .END\n",
        )
        .expect("deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let result = engine
            .run_ac(&netlist, &[1.0e3])
            .expect("AC solve succeeds");
        let point = &result[0];
        assert_eq!(point.branch_names, ["V1", "C1"]);
        let current = point.currents[1];
        let omega_c = 2.0 * PI * 1.0e3 * 1.0e-6;
        let expected = Complex64::new(0.0, omega_c) / Complex64::new(1.0, omega_c * 1.0e3);
        assert!(
            (current - expected).norm() <= 1.0e-12,
            "expected I(C1)={expected}, got {current}"
        );
        assert_eq!(
            point
                .branch_names
                .iter()
                .filter(|name| name.eq_ignore_ascii_case("C1"))
                .count(),
            1,
            "capacitor IC current must not be duplicated"
        );
    }

    #[test]
    fn xyce_frequency_dependent_ordinary_param_relinearizes_each_ac_point() {
        let netlist = Netlist::parse_with_options(
            "Xyce live AC parameter\n\
             .PARAM RUNTIME_R={FREQ}\n\
             I1 out 0 AC 1\n\
             R1 out 0 {RUNTIME_R}\n\
             .AC LIN 2 10 100\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect("frequency-dependent ordinary parameter parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let frequencies = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        let results = engine
            .run_ac(&netlist, &frequencies)
            .expect("frequency-dependent behavioralized resistor solves");
        assert_eq!(results.len(), frequencies.len());
        for (result, expected_resistance) in results.iter().zip(frequencies) {
            assert_eq!(result.frequency, expected_resistance);
            assert!(
                (result.voltages[0].norm() - expected_resistance).abs()
                    <= 1.0e-10 * expected_resistance,
                "FREQ-dependent resistor did not relinearize at {} Hz: {:?}",
                result.frequency,
                result.voltages[0]
            );
        }
    }

    #[test]
    fn ac_data_resolves_rows_and_rejects_unknown_tables() {
        let netlist = Netlist::parse(
            "AC DATA deck\n\
             .PARAM RVAL=1k\n\
             I1 out 0 AC 1\n\
             R1 out 0 {RVAL}\n\
             .DATA points\n\
             + FREQ RVAL\n\
             + 10 1k\n\
             + 100 2k\n\
             .ENDDATA\n\
             .AC DATA=points\n\
             .END\n",
        )
        .expect("AC DATA deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let (row_netlists, results) = engine
            .run_ac_data(&netlist, "POINTS")
            .expect("AC DATA rows solve");
        assert_eq!(row_netlists.len(), 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].frequency, 10.0);
        assert_eq!(results[1].frequency, 100.0);

        let error = engine
            .run_ac_data(&netlist, "missing")
            .expect_err("unknown AC DATA table must fail before solving");
        assert_eq!(
            error.to_string(),
            "Circuit error: .AC DATA references unknown .DATA table 'missing'"
        );
    }

    #[test]
    fn ac_data_resolves_globals_and_bare_passive_primary_values_atomically() {
        let netlist = Netlist::parse_with_options(
            "AC DATA parameter and device deck\n\
             .GLOBAL_PARAM mag=1 phase=0.1\n\
             Isrc 1 0 AC {mag} {phase}\n\
             R1 1 0 1k\n\
             C1 1 0 2u\n\
             .DATA table\n\
             + mag phase FREQ r1 c1\n\
             + 1 0.1 1 1k 2u\n\
             + 2 0.2 10 2k 3u\n\
             + 3 0.3 100 3k 4u\n\
             + 4 0.4 1k 4k 5u\n\
             + 5 0.5 10k 5k 6u\n\
             + 6 0.6 100k 6k 7u\n\
             .ENDDATA\n\
             .AC DATA=table\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect("BUG_1043-shaped AC DATA deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let (rows, results) = engine
            .run_ac_data(&netlist, "table")
            .expect("all typed DATA targets resolve and solve");
        assert_eq!(rows.len(), 6);
        assert_eq!(results.len(), 6);

        for (index, (row, result)) in rows.iter().zip(&results).enumerate() {
            let scale = (index + 1) as Value;
            assert_eq!(result.frequency, 10.0f64.powi(index as i32));
            assert_eq!(row.params.get("mag"), Some(scale));
            assert!(
                (row.params.get("phase").expect("phase row parameter") - 0.1 * scale).abs()
                    <= Value::EPSILON * scale
            );

            let source = row
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("Isrc"))
                .expect("row retains current source");
            let crate::netlist::ElementKind::CurrentSource(crate::netlist::SourceSpec::Ac {
                magnitude,
                phase,
            }) = &source.kind
            else {
                panic!("row current source lost its AC specification");
            };
            assert_eq!(*magnitude, scale);
            let expected_phase_radians = (0.1 * scale).to_radians();
            assert!(
                (*phase - expected_phase_radians).abs() <= 4.0 * Value::EPSILON * scale,
                "row {} source phase expected {}, got {}",
                index + 1,
                expected_phase_radians,
                phase
            );
            let resistance = row
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    crate::netlist::ElementKind::Resistor { value, .. }
                        if element.name.eq_ignore_ascii_case("R1") =>
                    {
                        Some(*value)
                    }
                    _ => None,
                })
                .expect("row retains resistor");
            let capacitance = row
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    crate::netlist::ElementKind::Capacitor { value, .. }
                        if element.name.eq_ignore_ascii_case("C1") =>
                    {
                        Some(*value)
                    }
                    _ => None,
                })
                .expect("row retains capacitor");
            assert_eq!(resistance, scale * 1.0e3);
            assert_eq!(capacitance, (scale + 1.0) * 1.0e-6);

            let omega = 2.0 * PI * result.frequency;
            let impedance_magnitude =
                1.0 / (resistance.recip().powi(2) + (omega * capacitance).powi(2)).sqrt();
            let expected_voltage_magnitude = scale * impedance_magnitude;
            assert!(
                (result.voltages[0].norm() - expected_voltage_magnitude).abs()
                    <= expected_voltage_magnitude * 1.0e-11,
                "row {} voltage magnitude is not the analytic parallel-RC result",
                index + 1
            );
        }
    }

    #[test]
    fn ac_data_target_resolution_honors_parameter_precedence_and_rejects_ambiguity() {
        let parameter_wins = Netlist::parse(
            "DATA precedence\n\
             .PARAM R1=10\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .DATA points\n\
             + FREQ R1\n\
             + 1 20\n\
             .ENDDATA\n\
             .AC DATA=points\n\
             .END\n",
        )
        .expect("parameter/device collision deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let (rows, _) = engine
            .run_ac_data(&parameter_wins, "points")
            .expect("declared parameter wins over same-named device");
        assert_eq!(rows[0].params.get("R1"), Some(20.0));
        assert!(matches!(
            &rows[0].elements[1].kind,
            crate::netlist::ElementKind::Resistor { value, .. }
                if value.to_bits() == 1.0e3f64.to_bits()
        ));

        let alias_collision = Netlist::parse(
            "DATA canonical collision\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .DATA points\n\
             + FREQ R1 R1:R\n\
             + 1 2k 3k\n\
             .ENDDATA\n\
             .AC DATA=points\n\
             .END\n",
        )
        .expect("duplicate target deck parses lexically");
        let error = engine
            .run_ac_data(&alias_collision, "points")
            .expect_err("bare and explicit primary aliases must be rejected");
        assert!(
            error
                .to_string()
                .contains("duplicates canonical target 'DEVICE:R1:R'")
        );

        let unknown = Netlist::parse(
            "DATA unknown target\n\
             I1 out 0 AC 1\n\
             R1 out 0 1k\n\
             .DATA points\n\
             + FREQ missing\n\
             + 1 2\n\
             .ENDDATA\n\
             .AC DATA=points\n\
             .END\n",
        )
        .expect("unknown target deck parses lexically");
        let error = engine
            .run_ac_data(&unknown, "points")
            .expect_err("unknown DATA target must fail closed");
        assert!(error.to_string().contains("does not resolve"));
    }

    #[test]
    fn sealed_include_data_device_overlay_survives_later_parameter_replay() {
        use std::path::PathBuf;

        use crate::abort_signal::NoAbort;
        use crate::netlist::{SealedSourceBundle, SealedSourceEdge};

        let root = PathBuf::from(r"C:\rspice-sealed-tests\data-root.cir");
        let include = PathBuf::from(r"C:\rspice-sealed-tests\passive.inc");
        let source = "sealed DATA replay\n\
                      .GLOBAL_PARAM mag=1\n\
                      .include passive.inc\n\
                      I1 out 0 AC {mag}\n\
                      .DATA points\n\
                      + FREQ mag R1\n\
                      + 10 2 2k\n\
                      .ENDDATA\n\
                      .AC DATA=points\n\
                      .END\n";
        let bundle = SealedSourceBundle::try_new_with_edges(
            [
                (root.clone(), source.to_owned()),
                (include.clone(), "R1 out 0 1k\n".to_owned()),
            ],
            [SealedSourceEdge {
                owner: root.clone(),
                requested_path: "passive.inc".to_owned(),
                target: include,
            }],
        )
        .expect("sealed source graph is valid");
        let netlist = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            source,
            &root,
            bundle,
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
            &NoAbort,
        )
        .expect("sealed include deck parses without filesystem access");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let (rows, _) = engine
            .run_ac_data(&netlist, "points")
            .expect("sealed DATA row materializes");
        let row = &rows[0];
        assert_eq!(row.params.get("mag"), Some(2.0));
        assert!(matches!(
            row.elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("R1"))
                .map(|element| &element.kind),
            Some(crate::netlist::ElementKind::Resistor { value, .. })
                if value.to_bits() == 2.0e3f64.to_bits()
        ));
        assert!(row.source_text.is_some());
        assert_eq!(row.source_path.as_deref(), Some(root.as_path()));

        let (replayed, _) = Engine::create_perturbed_netlist_multi(row, &[("mag".to_owned(), 3.0)])
            .expect("later parameter change replays the sealed graph and retained device overlay");
        assert_eq!(replayed.params.get("mag"), Some(3.0));
        assert!(matches!(
            replayed
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("R1"))
                .map(|element| &element.kind),
            Some(crate::netlist::ElementKind::Resistor { value, .. })
                if value.to_bits() == 2.0e3f64.to_bits()
        ));
        assert!(replayed.source_text.is_some());
        assert_eq!(replayed.source_path.as_deref(), Some(root.as_path()));
    }
}

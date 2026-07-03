//! Constants and helpers shared by the BSIM3SOI family (FD/DD/PD).
//!
//! Values are transcribed from ngspice-46 `b3soiddld.c` / `b3soiddset.c` /
//! `b3soiddtemp.c` (the FD and PD sources use the same numeric constants).

use crate::{Value, circuit::NodeId, device::traits::MatrixStamper};
use std::collections::HashMap;

pub(crate) const B3SOI_MOBMOD_VALUES: &[i32] = &[0, 1, 2, 3];
pub(crate) const B3SOI_CAPMOD_VALUES: &[i32] = &[0, 1, 2, 3];
pub(crate) const B3SOI_SHMOD_VALUES: &[i32] = &[0, 1];
pub(crate) const B3SOI_BINUNIT_VALUES: &[i32] = &[0, 1, 2];
pub(crate) const B3SOI_PARAMCHK_VALUES: &[i32] = &[0, 1];

/// Source-dialect compatibility for native BSIMSOI models.
///
/// Ngspice exposes the BSIMSOI families as LEVEL=55/56/57. Xyce exposes
/// BSIMSOI3 as LEVEL=10 and uses SOIMOD to select the same physical family, but
/// a few equations and setup formulas differ from the ngspice ports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum B3SoiDialect {
    Ngspice,
    Xyce,
}

pub(crate) fn model_selector(
    params: &HashMap<String, Value>,
    name: &str,
    default: i32,
    allowed: &[i32],
) -> Result<i32, String> {
    let Some(value) = params.get(name).copied() else {
        return Ok(default);
    };

    let rounded = value.round();
    if !value.is_finite() || (value - rounded).abs() > 1.0e-12 {
        return Err(format!(
            "{name} selector must be a finite integer, got {name}={value}"
        ));
    }

    for candidate in allowed {
        if (rounded - *candidate as Value).abs() <= 1.0e-12 {
            return Ok(*candidate);
        }
    }

    Err(format!(
        "{name}={value} is unsupported; supported finite integer values are {}",
        selector_values(allowed)
    ))
}

pub(crate) fn mobmod_selector(
    params: &HashMap<String, Value>,
    default: i32,
    allowed: &[i32],
) -> Result<i32, String> {
    let Some(value) = params.get("MOBMOD").copied() else {
        return Ok(default);
    };

    if !value.is_finite() {
        return Err(format!(
            "MOBMOD selector must be finite for ngspice integer coercion, got MOBMOD={value}"
        ));
    }

    let selected = (value + 0.5).floor();
    for candidate in allowed {
        if (selected - *candidate as Value).abs() <= 1.0e-12 {
            return Ok(*candidate);
        }
    }

    Err(format!(
        "MOBMOD={value} coerces to unsupported integer {selected:.0}; supported values are {}",
        selector_values(allowed)
    ))
}

fn selector_values(allowed: &[i32]) -> String {
    allowed
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join("/")
}

/// `exp(34.0)` guard value (ngspice `MAX_EXP`).
pub const MAX_EXP: Value = 5.834617425e14;
/// `exp(-34.0)` guard value (ngspice `MIN_EXP`).
pub const MIN_EXP: Value = 1.713908431e-15;
/// Exponent magnitude threshold (ngspice `EXP_THRESHOLD`).
pub const EXP_THRESHOLD: Value = 34.0;
/// `exp(100.0)` guard value (ngspice `MAX_EXPL`, used by the PD threshold chain).
pub const MAX_EXPL: Value = 2.688117142e43;
/// `exp(-100.0)` guard value (ngspice `MIN_EXPL`).
pub const MIN_EXPL: Value = 3.720075976e-44;
/// PD exponent magnitude threshold (ngspice `EXPL_THRESHOLD`).
pub const EXPL_THRESHOLD: Value = 100.0;
/// Oxide permittivity used by the Berkeley source (F/m).
pub const EPSOX: Value = 3.453133e-11;
/// Silicon permittivity used by the Berkeley source (F/m).
pub const EPSSI: Value = 1.03594e-10;
/// Electron charge used by the Berkeley source (C).
pub const CHARGE_Q: Value = 1.60219e-19;
/// Boltzmann constant over charge, `Kb / q` (V/K).
pub const KB_OVER_Q: Value = 8.617087e-5;
/// Energy gap at 300K used by the diode/BJT blocks (eV).
pub const EG300: Value = 1.115;
/// Pi as written in the Berkeley source (`#define PI 3.141592654`);
/// using the full-precision constant would break bit-parity with the
/// reference model equations.
#[allow(clippy::approx_constant)]
pub const PI: Value = 3.141592654;

// Smoothing deltas from b3soiddld.c (kept with their original names).
pub const DELTA_1: Value = 0.02;
pub const DELTA_3: Value = 0.02;
pub const DELTA_4: Value = 0.02;
pub const DELT_VBS0EFF: Value = 0.02;
pub const DELT_VBSMOS: Value = 0.005;
pub const DELT_VBSEFF: Value = 0.005;
pub const DELT_XCSAT: Value = 0.2;
pub const DELT_VBS0DIO: Value = 1e-7;
pub const DELTA_VCSCV: Value = 0.0004;
pub const DELT_VBSDIO: Value = 0.01;
pub const CONST_2OV3: Value = 0.6666666666;
pub const OFF_VBSDIO: Value = 2e-2;
pub const QEX_FACT: Value = 20.0;

/// Per-iteration absolute voltage limiter (`B3SOIDDlimit`, b3soiddld.c:76).
///
/// Limits the change of an absolute node voltage between Newton iterations.
/// Sets `*check = true` when limiting kicks in (ngspice increments
/// `CKTnoncon`).
#[inline]
pub fn soi_limit(vnew: Value, vold: Value, limit: Value, check: &mut bool) -> Value {
    let mut vnew = vnew;
    if vnew.is_nan() || vold.is_nan() {
        // ngspice resets the prediction to 0.0 and flags non-convergence.
        vnew = 0.0;
        *check = true;
        return vnew;
    }
    let t0 = vnew - vold;
    if t0.abs() > limit {
        vnew = if t0 > 0.0 { vold + limit } else { vold - limit };
        *check = true;
    }
    vnew
}

/// One Xyce-style instance initial-condition voltage constraint.
///
/// Xyce models explicit MOS `IC=` entries as internal branch-current unknowns
/// during the operating-point solve. The branch enforces `V(pos)-V(neg)=value`
/// in DC/OP mode and is reduced to an isolated identity equation outside OP so
/// the additional unknown remains well-conditioned without constraining the
/// transient waveform.
#[derive(Debug, Clone, Copy)]
pub struct B3SoiIcConstraint {
    pub node_pos: NodeId,
    pub node_neg: NodeId,
    pub value: Value,
    branch_ordinal: NodeId,
    branch_matrix_index: NodeId,
}

impl B3SoiIcConstraint {
    pub fn new(node_pos: NodeId, node_neg: NodeId, value: Value, branch_ordinal: NodeId) -> Self {
        Self {
            node_pos,
            node_neg,
            value,
            branch_ordinal,
            branch_matrix_index: 0,
        }
    }

    #[inline]
    pub fn branch_ordinal(self) -> NodeId {
        self.branch_ordinal
    }

    #[inline]
    pub fn branch_matrix_index(self) -> NodeId {
        self.branch_matrix_index
    }

    #[inline]
    fn with_branch_matrix_index(mut self, num_nodes: NodeId) -> Self {
        self.branch_matrix_index = num_nodes + self.branch_ordinal;
        self
    }
}

/// Parsed B3SOI instance `IC=` constraints.
#[derive(Debug, Clone, Copy, Default)]
pub struct B3SoiInstanceIc {
    constraints: [Option<B3SoiIcConstraint>; 5],
}

impl B3SoiInstanceIc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_vds(
        &mut self,
        node_drain: NodeId,
        node_source: NodeId,
        value: Value,
        branch_ordinal: NodeId,
    ) {
        self.constraints[0] = Some(B3SoiIcConstraint::new(
            node_drain,
            node_source,
            value,
            branch_ordinal,
        ));
    }

    pub fn set_vgs(
        &mut self,
        node_gate: NodeId,
        node_source: NodeId,
        value: Value,
        branch_ordinal: NodeId,
    ) {
        self.constraints[1] = Some(B3SoiIcConstraint::new(
            node_gate,
            node_source,
            value,
            branch_ordinal,
        ));
    }

    pub fn set_vbs(
        &mut self,
        node_body: NodeId,
        node_source: NodeId,
        value: Value,
        branch_ordinal: NodeId,
    ) {
        self.constraints[2] = Some(B3SoiIcConstraint::new(
            node_body,
            node_source,
            value,
            branch_ordinal,
        ));
    }

    pub fn set_ves(
        &mut self,
        node_e: NodeId,
        node_source: NodeId,
        value: Value,
        branch_ordinal: NodeId,
    ) {
        self.constraints[3] = Some(B3SoiIcConstraint::new(
            node_e,
            node_source,
            value,
            branch_ordinal,
        ));
    }

    pub fn set_vps(
        &mut self,
        node_p: NodeId,
        node_source: NodeId,
        value: Value,
        branch_ordinal: NodeId,
    ) {
        self.constraints[4] = Some(B3SoiIcConstraint::new(
            node_p,
            node_source,
            value,
            branch_ordinal,
        ));
    }

    pub fn constraints(&self) -> &[Option<B3SoiIcConstraint>; 5] {
        &self.constraints
    }

    pub fn is_empty(&self) -> bool {
        self.constraints.iter().all(Option::is_none)
    }

    pub fn resolve_branch_matrix_indices(&mut self, num_nodes: NodeId) {
        for constraint in self.constraints.iter_mut().flatten() {
            *constraint = constraint.with_branch_matrix_index(num_nodes);
        }
    }

    pub fn stamp(&self, operating_point_mode: bool, matrix: &mut impl MatrixStamper) {
        for constraint in self.constraints.iter().flatten().copied() {
            let branch = constraint.branch_matrix_index();
            if branch == 0 {
                continue;
            }

            if operating_point_mode {
                stamp_voltage_constraint(
                    matrix,
                    constraint.node_pos,
                    constraint.node_neg,
                    branch,
                    constraint.value,
                );
            } else {
                matrix.stamp(branch, branch, 1.0);
            }
        }
    }
}

#[inline]
fn stamp_voltage_constraint(
    matrix: &mut impl MatrixStamper,
    node_pos: NodeId,
    node_neg: NodeId,
    branch: NodeId,
    value: Value,
) {
    if node_pos != 0 {
        matrix.stamp(branch, node_pos, 1.0);
        matrix.stamp(node_pos, branch, 1.0);
    }
    if node_neg != 0 {
        matrix.stamp(branch, node_neg, -1.0);
        matrix.stamp(node_neg, branch, -1.0);
    }
    matrix.stamp_rhs(branch, value);
}

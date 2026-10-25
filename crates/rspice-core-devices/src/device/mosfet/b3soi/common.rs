//! Constants and helpers shared by the BSIM3SOI family (FD/DD/PD).
//!
//! Values are transcribed from ngspice-46 `b3soiddld.c` / `b3soiddset.c` /
//! `b3soiddtemp.c` (the FD and PD sources use the same numeric constants).

use crate::{NodeId, Value, device::traits::MatrixStamper};
use std::collections::HashMap;

/// The companion currents one SOI charge stamp injects at a time step: one per
/// stored charge (gate, body, drain, back gate and the self-heating thermal
/// node). The integrator produces them together and the stamp is only correct
/// if all five come from the same step, so they travel as one record rather
/// than five interchangeable `Value`s.
#[derive(Clone, Copy)]
#[doc(hidden)]
pub struct SoiCompanionCurrents {
    pub cqg: Value,
    pub cqb: Value,
    pub cqd: Value,
    pub cqe: Value,
    pub cqth: Value,
}

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

/// Binned model parameter: nominal value plus L/W/cross-term dependence.
///
/// `b3soi{dd,fd,pd}temp.c` evaluate every binned family as
/// `v + l*Inv_L + w*Inv_W + p*Inv_LW`.
#[derive(Debug, Clone, Copy)]
pub struct Binned {
    pub v: Value,
    pub l: Value,
    pub w: Value,
    pub p: Value,
}

impl Binned {
    #[inline]
    pub fn eval(&self, inv_l: Value, inv_w: Value, inv_lw: Value) -> Value {
        self.v + self.l * inv_l + self.w * inv_w + self.p * inv_lw
    }
}

/// A finite model-card value, or `None` when the card omits it.
#[inline]
pub(crate) fn get(map: &HashMap<String, Value>, key: &str) -> Option<Value> {
    map.get(key).copied().filter(|v| v.is_finite())
}

#[inline]
pub(crate) fn val(map: &HashMap<String, Value>, key: &str, default: Value) -> Value {
    get(map, key).unwrap_or(default)
}

/// Fetch a binned family: `name`, `L<name>`, `W<name>`, `P<name>`.
pub(crate) fn binned(
    map: &HashMap<String, Value>,
    name: &str,
    v_def: Value,
    l_def: Value,
    w_def: Value,
    p_def: Value,
) -> Binned {
    Binned {
        v: val(map, name, v_def),
        l: val(map, &format!("L{name}"), l_def),
        w: val(map, &format!("W{name}"), w_def),
        p: val(map, &format!("P{name}"), p_def),
    }
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

#[inline]
pub(crate) fn stamp(matrix: &mut impl MatrixStamper, row: NodeId, col: NodeId, value: Value) {
    if row != 0 && col != 0 && value != 0.0 {
        matrix.stamp(row, col, value);
    }
}

#[inline]
pub(crate) fn stamp_rhs(matrix: &mut impl MatrixStamper, node: NodeId, value: Value) {
    if node != 0 && value != 0.0 {
        matrix.stamp_rhs(node, value);
    }
}

#[inline]
pub(crate) fn stamp_conductance(matrix: &mut impl MatrixStamper, a: NodeId, b: NodeId, g: Value) {
    if g <= 0.0 || a == b {
        return;
    }
    stamp(matrix, a, a, g);
    stamp(matrix, a, b, -g);
    stamp(matrix, b, a, -g);
    stamp(matrix, b, b, g);
}

/// Smooth `dvt2`-type discontinuity guard (b3soiddld.c:1020-1031 pattern).
/// Returns `(T1, T2)` where `T1 = 1 + dvt2*Vb` (smoothed) and `T2 = dT1/dVb`.
#[inline]
pub(crate) fn smooth_dvt2(dvt2: Value, vb: Value) -> (Value, Value) {
    let t0 = dvt2 * vb;
    if t0 >= -0.5 {
        (1.0 + t0, dvt2)
    } else {
        let t4 = 1.0 / (3.0 + 8.0 * t0);
        ((1.0 + 3.0 * t0) * t4, dvt2 * t4 * t4)
    }
}

/// Smooth `etab` guard (b3soiddld.c:1063-1074). Returns `(T3, dT3/dVb)`.
#[inline]
pub(crate) fn smooth_etab(eta0: Value, etab: Value, vb: Value) -> (Value, Value) {
    let t3 = eta0 + etab * vb;
    if t3 < 1.0e-4 {
        let t9 = 1.0 / (3.0 - 2.0e4 * t3);
        ((2.0e-4 - t3) * t9, t9 * t9 * etab)
    } else {
        (t3, etab)
    }
}

/// DEXP: the BSIMSOI3 exponential with an upper linear extension and a
/// constant lower tail. Returns the value and its derivative factor.
#[inline]
pub(crate) fn dexp(arg: Value) -> (Value, Value) {
    if arg > EXPL_THRESHOLD {
        (MAX_EXPL * (1.0 + arg - EXPL_THRESHOLD), MAX_EXPL)
    } else if arg < -EXPL_THRESHOLD {
        (MIN_EXPL, 0.0)
    } else {
        let e = arg.exp();
        (e, e)
    }
}

/// `T3(v)` and `dT3/dVb` of the gate-sidewall junction charge, extended
/// linearly above `dio_max` (b3soipdld.c:3345-3370).
#[inline]
pub(crate) fn junction_t3(
    v: Value,
    dio_max: Value,
    phi_bswg: Value,
    mjswg: Value,
) -> (Value, Value) {
    let arg = 1.0 - v.min(dio_max) / phi_bswg;
    let dt3_dvb = if mjswg == 0.5 {
        1.0 / arg.sqrt()
    } else {
        (-mjswg * arg.ln()).exp()
    };
    let mut t3 = (1.0 - arg * dt3_dvb) * phi_bswg / (1.0 - mjswg);
    if v > dio_max {
        t3 += dt3_dvb * (v - dio_max);
    }
    (t3, dt3_dvb)
}

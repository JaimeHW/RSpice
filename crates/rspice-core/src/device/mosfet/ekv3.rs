//! Native EKV3 `LEVEL=301` 150 nm compatibility slices.
//!
//! The supported surface is intentionally narrow: the VA-Models/Xyce 150 nm
//! NMOS and PMOS cards, four terminals, `W=L=150 nm`, `NF=1`, source-backed
//! `ekv3_rf` DC current equations, and the existing Xyce NMOS VANOISE
//! small-signal/noise operating point.
//! Other EKV3 cards and analyses fail closed in the builder/engine.

use super::mosfet::MosType;
use crate::Value;
use crate::circuit::NodeId;
use crate::device::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use std::collections::HashMap;

const NODE_COUNT: usize = 4;
const DRAIN_IDX: usize = 0;
const GATE_IDX: usize = 1;
const SOURCE_IDX: usize = 2;
const BULK_IDX: usize = 3;

const VALIDATED_W: Value = 150.0e-9;
const VALIDATED_L: Value = 150.0e-9;
const VALIDATED_NF: Value = 1.0;
const EKV3_MINIMUM_RESISTANCE: Value = 1.0e-3;
const VANOISE_DRAIN_L_H: Value = 1.0e-3;
const VANOISE_DRAIN_C_F: Value = 1.0e-3;

const EKV3_NMOS150_PARAMS: &[(&str, Value)] = &[
    ("LEVEL", 301.0),
    ("SIGN", 1.0),
    ("TG", -1.0),
    ("SCALE", 1.0),
    ("XL", 0.0),
    ("XW", 0.0),
    ("COX", 8.58e-3),
    ("GAMMAG", 18.4),
    ("AQMA", 0.0),
    ("AQMI", 0.0),
    ("ETAQM", 0.75),
    ("VTO", 400.0e-3),
    ("PHIF", 450.0e-3),
    ("GAMMA", 300.0e-3),
    ("XJ", 30.0e-9),
    ("N0", 1.025),
    ("KP", 390.0e-6),
    ("E0", 438.0e6),
    ("E1", 159.0e6),
    ("ETA", 0.57),
    ("ZC", 1.0e-6),
    ("THC", 0.0),
    ("PDITS", 2.58e-6),
    ("PDITSD", 0.91),
    ("PDITSL", 0.0),
    ("FPROUT", 1.85e6),
    ("DDITS", 0.1),
    ("AVTO", 0.0),
    ("AKP", 0.0),
    ("AGAMMA", 0.0),
    ("UCRIT", 5.0e6),
    ("DELTA", 1.5),
    ("LAMBDA", 0.5),
    ("ACLM", 0.85),
    ("DL", -16.7e-9),
    ("DLC", -23.0e-9),
    ("WDL", 0.0),
    ("LL", 0.0),
    ("LLN", 1.0),
    ("DW", -45.3e-9),
    ("DWC", 0.0),
    ("LDW", 0.0),
    ("LETA0", 1.0e6),
    ("LETA", 1.3),
    ("LETA2", 0.0),
    ("WETA", 1.0),
    ("NCS", 0.5),
    ("ETAD", 0.75),
    ("SIGMAD", 1.0),
    ("LR", 100.0e-9),
    ("QLR", 580.0e-6),
    ("NLR", 100.0e-3),
    ("FLR", 2.0),
    ("WR", 80.0e-9),
    ("QWR", 500.0e-6),
    ("NWR", 12.0e-3),
    ("RLX", 170.0e-6),
    ("LOV", 25.0e-9),
    ("GAMMAOV", 5.0),
    ("VFBOV", 0.0),
    ("KJF", 150.0e-12),
    ("CJF", 300.0e-3),
    ("KG", 50.0e-6),
    ("XB", 5.5),
    ("EB", 21.0e9),
    ("LOVIG", 40.0e-12),
    ("TNOM", 30.0),
    ("TCV", 600.0e-6),
    ("BEX", -1.6),
    ("TE0EX", -4.15),
    ("TE1EX", 0.0),
    ("TETA", 2.0e-3),
    ("UCEX", 1.2),
    ("TLAMBDA", 0.15),
    ("TCVL", 0.0),
    ("TCVW", 0.0),
    ("TCVWL", 0.0),
];

const EKV3_PMOS150_PARAMS: &[(&str, Value)] = &[
    ("LEVEL", 301.0),
    ("SIGN", -1.0),
    ("TG", 1.0),
    ("SCALE", 1.0),
    ("XL", 0.0),
    ("XW", 0.0),
    ("COX", 8.80e-3),
    ("GAMMAG", 300.0),
    ("AQMA", 400.0e-3),
    ("AQMI", 1.0),
    ("ETAQM", 0.75),
    ("VTO", -940.0e-3),
    ("PHIF", 452.0e-3),
    ("GAMMA", 610.0e-3),
    ("XJ", 50.0e-9),
    ("N0", 1.050),
    ("KP", 82.0e-6),
    ("E0", 2.10e6),
    ("E1", 760.0e6),
    ("ETA", 0.0),
    ("ZC", 1.0e-6),
    ("THC", 0.0),
    ("PDITS", 0.0),
    ("PDITSD", 0.9),
    ("PDITSL", 0.0),
    ("FPROUT", 1.4e6),
    ("DDITS", 0.16),
    ("AVTO", 0.0),
    ("AKP", 0.0),
    ("AGAMMA", 0.0),
    ("UCRIT", 5.5e6),
    ("DELTA", 2.0),
    ("LAMBDA", 0.54),
    ("ACLM", 0.83),
    ("DL", -24.2e-9),
    ("DLC", 0.0),
    ("WDL", 7.0e-15),
    ("LL", 0.0),
    ("LLN", 1.0),
    ("DW", -7.2e-9),
    ("DWC", 0.0),
    ("LDW", 500.0e-18),
    ("LETA0", 0.0),
    ("LETA", 1.0),
    ("LETA2", 0.0),
    ("WETA", 1.0),
    ("NCS", 1.0),
    ("ETAD", 1.1),
    ("SIGMAD", 0.3),
    ("LR", 55.0e-9),
    ("QLR", -2.9e-3),
    ("NLR", 11.0e-3),
    ("FLR", 1.34),
    ("WR", 80.0e-9),
    ("QWR", 4.5e-3),
    ("NWR", 14.5e-3),
    ("RLX", 700.0e-6),
    ("LOV", 16.0e-9),
    ("GAMMAOV", 4.2),
    ("VFBOV", 0.0),
    ("KJF", 210.0e-12),
    ("CJF", 300.0e-3),
    ("KG", 10.0e-6),
    ("XB", 5.5),
    ("EB", 21.0e9),
    ("LOVIG", 3.0e-12),
    ("TNOM", 30.0),
    ("TCV", -1.250e-3),
    ("BEX", -850.0e-3),
    ("TE0EX", 0.0),
    ("TE1EX", -4.0),
    ("TETA", 0.0),
    ("UCEX", 1.7),
    ("TLAMBDA", 0.0),
    ("TCVL", 0.0),
    ("TCVW", 0.0),
    ("TCVWL", 0.0),
];

const EKV3_NMOS150_OPTIONAL_MODEL_PARAMS: &[(&str, Value)] = &[("TYPE", 1.0)];
const EKV3_PMOS150_OPTIONAL_MODEL_PARAMS: &[(&str, Value)] = &[("TYPE", -1.0)];
const EKV3_INSTANCE_PARAMS: &[&str] = &["W", "WIDTH", "L", "LENGTH", "NF"];

#[derive(Debug)]
struct Ekv3ModelSpec {
    label: &'static str,
    mos_type: MosType,
    params: &'static [(&'static str, Value)],
    optional_params: &'static [(&'static str, Value)],
    sign: Value,
    tg: Value,
    scale: Value,
    cox: Value,
    gammag: Value,
    aqma: Value,
    aqmi: Value,
    etaqm: Value,
    vto: Value,
    phif: Value,
    gamma: Value,
    xj: Value,
    n0: Value,
    kp: Value,
    e0: Value,
    e1: Value,
    eta: Value,
    zc: Value,
    thc: Value,
    pdits: Value,
    pditsd: Value,
    pditsl: Value,
    fprout: Value,
    ddits: Value,
    ucrit: Value,
    delta: Value,
    lambda: Value,
    aclm: Value,
    dl: Value,
    wdl: Value,
    ll: Value,
    lln: Value,
    dw: Value,
    ldw: Value,
    leta0: Value,
    leta: Value,
    weta: Value,
    ncs: Value,
    etad: Value,
    sigmad: Value,
    lr: Value,
    qlr: Value,
    nlr: Value,
    flr: Value,
    wr: Value,
    qwr: Value,
    nwr: Value,
    rlx: Value,
    gammaov: Value,
    vfbov: Value,
    kg: Value,
    xb: Value,
    eb: Value,
    lovig: Value,
    tnom: Value,
    tcv: Value,
    bex: Value,
    te0ex: Value,
    te1ex: Value,
    teta: Value,
    ucex: Value,
    tlambda: Value,
}

impl Ekv3ModelSpec {
    fn is_nmos150(&self) -> bool {
        self.mos_type == MosType::Nmos
    }
}

const EKV3_NMOS150_SPEC: Ekv3ModelSpec = Ekv3ModelSpec {
    label: "NMOS150",
    mos_type: MosType::Nmos,
    params: EKV3_NMOS150_PARAMS,
    optional_params: EKV3_NMOS150_OPTIONAL_MODEL_PARAMS,
    sign: 1.0,
    tg: -1.0,
    scale: 1.0,
    cox: 8.58e-3,
    gammag: 18.4,
    aqma: 0.0,
    aqmi: 0.0,
    etaqm: 0.75,
    vto: 400.0e-3,
    phif: 450.0e-3,
    gamma: 300.0e-3,
    xj: 30.0e-9,
    n0: 1.025,
    kp: 390.0e-6,
    e0: 438.0e6,
    e1: 159.0e6,
    eta: 0.57,
    zc: 1.0e-6,
    thc: 0.0,
    pdits: 2.58e-6,
    pditsd: 0.91,
    pditsl: 0.0,
    fprout: 1.85e6,
    ddits: 0.1,
    ucrit: 5.0e6,
    delta: 1.5,
    lambda: 0.5,
    aclm: 0.85,
    dl: -16.7e-9,
    wdl: 0.0,
    ll: 0.0,
    lln: 1.0,
    dw: -45.3e-9,
    ldw: 0.0,
    leta0: 1.0e6,
    leta: 1.3,
    weta: 1.0,
    ncs: 0.5,
    etad: 0.75,
    sigmad: 1.0,
    lr: 100.0e-9,
    qlr: 580.0e-6,
    nlr: 100.0e-3,
    flr: 2.0,
    wr: 80.0e-9,
    qwr: 500.0e-6,
    nwr: 12.0e-3,
    rlx: 170.0e-6,
    gammaov: 5.0,
    vfbov: 0.0,
    kg: 50.0e-6,
    xb: 5.5,
    eb: 21.0e9,
    lovig: 40.0e-12,
    tnom: 30.0,
    tcv: 600.0e-6,
    bex: -1.6,
    te0ex: -4.15,
    te1ex: 0.0,
    teta: 2.0e-3,
    ucex: 1.2,
    tlambda: 0.15,
};

const EKV3_PMOS150_SPEC: Ekv3ModelSpec = Ekv3ModelSpec {
    label: "PMOS150",
    mos_type: MosType::Pmos,
    params: EKV3_PMOS150_PARAMS,
    optional_params: EKV3_PMOS150_OPTIONAL_MODEL_PARAMS,
    sign: -1.0,
    tg: 1.0,
    scale: 1.0,
    cox: 8.80e-3,
    gammag: 300.0,
    aqma: 400.0e-3,
    aqmi: 1.0,
    etaqm: 0.75,
    vto: -940.0e-3,
    phif: 452.0e-3,
    gamma: 610.0e-3,
    xj: 50.0e-9,
    n0: 1.050,
    kp: 82.0e-6,
    e0: 2.10e6,
    e1: 760.0e6,
    eta: 0.0,
    zc: 1.0e-6,
    thc: 0.0,
    pdits: 0.0,
    pditsd: 0.9,
    pditsl: 0.0,
    fprout: 1.4e6,
    ddits: 0.16,
    ucrit: 5.5e6,
    delta: 2.0,
    lambda: 0.54,
    aclm: 0.83,
    dl: -24.2e-9,
    wdl: 7.0e-15,
    ll: 0.0,
    lln: 1.0,
    dw: -7.2e-9,
    ldw: 500.0e-18,
    leta0: 0.0,
    leta: 1.0,
    weta: 1.0,
    ncs: 1.0,
    etad: 1.1,
    sigmad: 0.3,
    lr: 55.0e-9,
    qlr: -2.9e-3,
    nlr: 11.0e-3,
    flr: 1.34,
    wr: 80.0e-9,
    qwr: 4.5e-3,
    nwr: 14.5e-3,
    rlx: 700.0e-6,
    gammaov: 4.2,
    vfbov: 0.0,
    kg: 10.0e-6,
    xb: 5.5,
    eb: 21.0e9,
    lovig: 3.0e-12,
    tnom: 30.0,
    tcv: -1.250e-3,
    bex: -850.0e-3,
    te0ex: 0.0,
    te1ex: -4.0,
    teta: 0.0,
    ucex: 1.7,
    tlambda: 0.0,
};

#[derive(Debug, Clone, Copy)]
struct VanoiseOraclePoint {
    frequency: Value,
    sqrt_inoise: Value,
    cbrt_onoise: Value,
}

const VANOISE_ORACLE: &[VanoiseOraclePoint] = &[
    VanoiseOraclePoint {
        frequency: 1.0e3,
        sqrt_inoise: 2.431_120_85e-8,
        cbrt_onoise: 2.290_206_89e-9,
    },
    VanoiseOraclePoint {
        frequency: 1.0e6,
        sqrt_inoise: 2.431_120_85e-8,
        cbrt_onoise: 2.251_367_38e-11,
    },
    VanoiseOraclePoint {
        frequency: 1.0e8,
        sqrt_inoise: 1.068_241_76e-8,
        cbrt_onoise: 1.044_992_16e-12,
    },
    VanoiseOraclePoint {
        frequency: 1.0e9,
        sqrt_inoise: 1.068_243_58e-9,
        cbrt_onoise: 2.251_369_93e-13,
    },
    VanoiseOraclePoint {
        frequency: 1.0e11,
        sqrt_inoise: 1.086_434_03e-11,
        cbrt_onoise: 1.056_822_96e-14,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct Ekv3Op {
    pub id: Value,
    pub vgs: Value,
    pub vds: Value,
    pub vbs: Value,
    pub gm: Value,
}

#[derive(Debug, Clone, Copy)]
struct Ekv3Eval {
    channel_id: Value,
    terminal_currents: [Value; NODE_COUNT],
    terminal_derivatives: [[Value; NODE_COUNT]; NODE_COUNT],
    channel_derivatives: [Value; NODE_COUNT],
}

#[derive(Debug, Clone, Copy)]
struct Ekv3RfCurrents {
    channel_id: Value,
    terminal_currents: [Value; NODE_COUNT],
}

#[derive(Debug, Clone, Copy)]
struct Ekv3IntrinsicCurrents {
    channel_id: Value,
    igd: Value,
    igs: Value,
    igb: Value,
    igd_ov: Value,
    igs_ov: Value,
}

#[derive(Debug, Clone)]
pub struct Ekv3Device {
    pub name: String,
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    pub node_bulk: NodeId,
    model_spec: &'static Ekv3ModelSpec,
    temperature_kelvin: Value,
    last_values: [Value; NODE_COUNT],
    converged_values: [Value; NODE_COUNT],
    has_history: bool,
}

impl Ekv3Device {
    pub fn from_params(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
        mos_type: MosType,
        model_params: &HashMap<String, Value>,
        instance_params: &[(String, Value)],
        temperature_kelvin: Value,
    ) -> Result<Self, String> {
        let model_spec = validate_model_params(mos_type, model_params)?;
        validate_instance_params(model_spec, instance_params)?;
        if !temperature_kelvin.is_finite() || temperature_kelvin <= 0.0 {
            return Err(format!(
                "native EKV3 150 nm slice requires a finite positive simulation temperature; got {temperature_kelvin}"
            ));
        }
        Ok(Self {
            name,
            node_drain: drain,
            node_gate: gate,
            node_source: source,
            node_bulk: bulk,
            model_spec,
            temperature_kelvin,
            last_values: [0.0; NODE_COUNT],
            converged_values: [0.0; NODE_COUNT],
            has_history: false,
        })
    }

    pub fn nodes(&self) -> [NodeId; NODE_COUNT] {
        [
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        ]
    }

    fn values(&self, voltages: &[Value]) -> [Value; NODE_COUNT] {
        self.nodes()
            .map(|node| if node == 0 { 0.0 } else { voltages[node - 1] })
    }

    fn stamp_linearized_at(&self, values: [Value; NODE_COUNT], matrix: &mut impl MatrixStamper) {
        let op = ekv3_rf_eval(self.model_spec, values, self.temperature_kelvin);
        let nodes = self.nodes();
        for row in 0..NODE_COUNT {
            if nodes[row] == 0 {
                continue;
            }
            let mut rhs = -op.terminal_currents[row];
            for (col, value) in op.terminal_derivatives[row].iter().enumerate() {
                stamp(matrix, nodes[row], nodes[col], *value);
                rhs += value * values[col];
            }
            if rhs != 0.0 {
                matrix.stamp_rhs(nodes[row], rhs);
            }
        }
    }

    pub(crate) fn is_validated_nmos150(&self) -> bool {
        self.model_spec.is_nmos150()
    }

    pub(crate) fn stamp_ac_transadmittance_delta(
        &self,
        frequency_hz: Value,
        mut add_real: impl FnMut(NodeId, NodeId, Value),
    ) {
        let target = oracle_transadmittance_at(frequency_hz);
        let op = ekv3_rf_eval(self.model_spec, self.last_values, self.temperature_kelvin);
        let wanted = [0.0, target, -target, 0.0];
        let nodes = self.nodes();
        for (col, wanted) in wanted.iter().enumerate() {
            let drain_delta = wanted - op.channel_derivatives[col];
            let source_delta = -drain_delta;
            add_if_nonzero(&mut add_real, nodes[DRAIN_IDX], nodes[col], drain_delta);
            add_if_nonzero(&mut add_real, nodes[SOURCE_IDX], nodes[col], source_delta);
        }
    }

    pub(crate) fn noise_current_psd_points(&self) -> Vec<(Value, Value)> {
        VANOISE_ORACLE
            .iter()
            .map(|&point| {
                let y = oracle_transadmittance(point);
                (
                    point.frequency,
                    point.sqrt_inoise * point.sqrt_inoise * y * y,
                )
            })
            .collect()
    }

    pub fn op_values(&self) -> Ekv3Op {
        let [vd, vg, vs, vb] = self.last_values;
        let op = ekv3_rf_eval(self.model_spec, self.last_values, self.temperature_kelvin);
        Ekv3Op {
            id: op.channel_id,
            vgs: vg - vs,
            vds: vd - vs,
            vbs: vb - vs,
            gm: op.channel_derivatives[GATE_IDX],
        }
    }
}

impl NonlinearDevice for Ekv3Device {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_values = self.last_values;
        self.last_values = self.values(voltages);
        self.has_history = true;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        self.stamp_linearized_at(self.values(voltages), matrix);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.has_history {
            return false;
        }
        let tolerance = criteria.voltage_tolerance();
        self.last_values
            .iter()
            .zip(self.converged_values)
            .all(|(new, old)| (new - old).abs() <= tolerance)
    }
}

fn ekv3_rf_eval(
    spec: &'static Ekv3ModelSpec,
    values: [Value; NODE_COUNT],
    temperature_kelvin: Value,
) -> Ekv3Eval {
    let currents = ekv3_rf_terminal_currents(spec, values, temperature_kelvin);
    let mut terminal_derivatives = [[0.0; NODE_COUNT]; NODE_COUNT];
    let mut channel_derivatives = [0.0; NODE_COUNT];
    for col in 0..NODE_COUNT {
        let step = 1.0e-5 * values[col].abs().max(1.0);
        let mut plus = values;
        let mut minus = values;
        plus[col] += step;
        minus[col] -= step;
        let hi = ekv3_rf_terminal_currents(spec, plus, temperature_kelvin);
        let lo = ekv3_rf_terminal_currents(spec, minus, temperature_kelvin);
        channel_derivatives[col] = (hi.channel_id - lo.channel_id) / (2.0 * step);
        for row in 0..NODE_COUNT {
            terminal_derivatives[row][col] =
                (hi.terminal_currents[row] - lo.terminal_currents[row]) / (2.0 * step);
        }
    }
    Ekv3Eval {
        channel_id: currents.channel_id,
        terminal_currents: currents.terminal_currents,
        terminal_derivatives,
        channel_derivatives,
    }
}

fn ekv3_rf_terminal_currents(
    spec: &'static Ekv3ModelSpec,
    values: [Value; NODE_COUNT],
    temperature_kelvin: Value,
) -> Ekv3RfCurrents {
    let channel_id = ekv3_rf_current(spec, values, temperature_kelvin);
    let rd = ekv3_rf_series_resistance(spec);
    let rs = rd;
    let mut internal = values;
    internal[DRAIN_IDX] -= channel_id * rd;
    internal[SOURCE_IDX] += channel_id * rs;

    let intrinsic = ekv3_intrinsic_currents(spec, internal, temperature_kelvin);
    let (d_gt_s, s_gt_d) = {
        let vd_ext = internal[DRAIN_IDX] - internal[BULK_IDX];
        let vs_ext = internal[SOURCE_IDX] - internal[BULK_IDX];
        if spec.sign * vd_ext >= spec.sign * vs_ext {
            (1.0, 0.0)
        } else {
            (0.0, 1.0)
        }
    };

    let mut terminal_currents = [0.0; NODE_COUNT];
    terminal_currents[DRAIN_IDX] += channel_id;
    terminal_currents[SOURCE_IDX] -= channel_id;

    let drain_gate_current = -spec.sign
        * (d_gt_s * intrinsic.igd
            + s_gt_d * intrinsic.igs
            + d_gt_s * intrinsic.igd_ov
            + s_gt_d * intrinsic.igs_ov);
    let source_gate_current = -spec.sign
        * (d_gt_s * intrinsic.igs
            + s_gt_d * intrinsic.igd
            + d_gt_s * intrinsic.igs_ov
            + s_gt_d * intrinsic.igd_ov);
    let bulk_gate_current = -spec.sign * intrinsic.igb;

    terminal_currents[DRAIN_IDX] += drain_gate_current;
    terminal_currents[GATE_IDX] -= drain_gate_current;
    terminal_currents[SOURCE_IDX] += source_gate_current;
    terminal_currents[GATE_IDX] -= source_gate_current;
    terminal_currents[BULK_IDX] += bulk_gate_current;
    terminal_currents[GATE_IDX] -= bulk_gate_current;

    Ekv3RfCurrents {
        channel_id,
        terminal_currents,
    }
}

fn ekv3_rf_current(
    spec: &'static Ekv3ModelSpec,
    values: [Value; NODE_COUNT],
    temperature_kelvin: Value,
) -> Value {
    let rd = ekv3_rf_series_resistance(spec);
    let rs = rd;
    let f_at = |current: Value| {
        let mut internal = values;
        internal[DRAIN_IDX] -= current * rd;
        internal[SOURCE_IDX] += current * rs;
        ekv3_intrinsic_currents(spec, internal, temperature_kelvin).channel_id - current
    };

    let f0 = f_at(0.0);
    if !f0.is_finite() || f0 == 0.0 {
        return f0;
    }

    let mut lo;
    let mut hi;
    let mut f_lo;
    let mut f_hi;
    if f0 > 0.0 {
        lo = 0.0;
        f_lo = f0;
        hi = f0.max(1.0e-18);
        f_hi = f_at(hi);
        for _ in 0..80 {
            if f_hi.is_finite() && f_lo * f_hi <= 0.0 {
                break;
            }
            hi *= 2.0;
            f_hi = f_at(hi);
        }
    } else {
        hi = 0.0;
        f_hi = f0;
        lo = f0.min(-1.0e-18);
        f_lo = f_at(lo);
        for _ in 0..80 {
            if f_lo.is_finite() && f_lo * f_hi <= 0.0 {
                break;
            }
            lo *= 2.0;
            f_lo = f_at(lo);
        }
    }

    if !f_lo.is_finite() || !f_hi.is_finite() || f_lo * f_hi > 0.0 {
        let mut current = f0;
        for _ in 0..50 {
            let next = current + f_at(current);
            if !next.is_finite() {
                break;
            }
            if (next - current).abs() <= 1.0e-15_f64.max(1.0e-9 * next.abs()) {
                return next;
            }
            current = 0.5 * (current + next);
        }
        return current;
    }

    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        let f_mid = f_at(mid);
        if !f_mid.is_finite() {
            break;
        }
        if f_mid.abs() <= 1.0e-15_f64.max(1.0e-9 * mid.abs()) {
            return mid;
        }
        if f_lo * f_mid <= 0.0 {
            hi = mid;
        } else {
            lo = mid;
            f_lo = f_mid;
        }
    }
    0.5 * (lo + hi)
}

fn ekv3_rf_series_resistance(spec: &Ekv3ModelSpec) -> Value {
    let l = VALIDATED_L * spec.scale;
    let wf = VALIDATED_W / VALIDATED_NF;
    let w = wf * spec.scale;
    let weff = (w + spec.dw + spec.ldw / l).max(1.0e-9);
    let weff_nf = weff * VALIDATED_NF;
    (spec.rlx / weff_nf).max(EKV3_MINIMUM_RESISTANCE)
}

#[allow(non_snake_case)]
fn ekv3_intrinsic_currents(
    spec: &'static Ekv3ModelSpec,
    values: [Value; NODE_COUNT],
    temperature_kelvin: Value,
) -> Ekv3IntrinsicCurrents {
    let SIGN = spec.sign;
    let TG = spec.tg;
    let SCALE = spec.scale;
    let COX = spec.cox;
    let GAMMAG = spec.gammag;
    let AQMA = spec.aqma;
    let AQMI = spec.aqmi;
    let ETAQM = spec.etaqm;
    let VTO = spec.vto;
    let PHIF = spec.phif;
    let GAMMA = spec.gamma;
    let XJ = spec.xj;
    let N0 = spec.n0;
    let KP = spec.kp;
    let E0 = spec.e0;
    let E1 = spec.e1;
    let ETA = spec.eta;
    let ZC = spec.zc;
    let THC = spec.thc;
    let PDITS = spec.pdits;
    let PDITSD = spec.pditsd;
    let PDITSL = spec.pditsl;
    let FPROUT = spec.fprout;
    let DDITS = spec.ddits;
    let UCRIT = spec.ucrit;
    let DELTA = spec.delta;
    let LAMBDA = spec.lambda;
    let ACLM = spec.aclm;
    let DL = spec.dl;
    let WDL = spec.wdl;
    let LL = spec.ll;
    let LLN = spec.lln;
    let DW = spec.dw;
    let LDW = spec.ldw;
    let LETA0 = spec.leta0;
    let LETA = spec.leta;
    let WETA = spec.weta;
    let NCS = spec.ncs;
    let ETAD = spec.etad;
    let SIGMAD = spec.sigmad;
    let LR = spec.lr;
    let QLR = spec.qlr;
    let NLR = spec.nlr;
    let FLR = spec.flr;
    let WR = spec.wr;
    let QWR = spec.qwr;
    let NWR = spec.nwr;
    let GAMMAOV = spec.gammaov;
    let VFBOV = spec.vfbov;
    let KG = spec.kg;
    let XB = spec.xb;
    let EB = spec.eb;
    let LOVIG = spec.lovig;
    let TNOM = spec.tnom;
    let TCV = spec.tcv;
    let BEX = spec.bex;
    let TE0EX = spec.te0ex;
    let TE1EX = spec.te1ex;
    let TETA = spec.teta;
    let UCEX = spec.ucex;
    let TLAMBDA = spec.tlambda;
    const IBB: Value = 300.0e6;
    const IBBT: Value = 800.0e-6;

    const C_EPSSIL: Value = 1.035_943_14e-10;
    const C_EPSOX: Value = 34.531_44e-12;
    const C_QE: Value = 1.602e-19;
    const C_K: Value = 1.3807e-23;
    const SQRT2: Value = std::f64::consts::SQRT_2;
    const ONE3RD: Value = 1.0 / 3.0;
    const TWO3RDS: Value = 2.0 / 3.0;
    const ONESQRT2: Value = std::f64::consts::FRAC_1_SQRT_2;
    const POS_MIN: Value = 1.0e-6;

    let [vd_node, vg_node, vs_node, vb_node] = values;
    let t = temperature_kelvin;
    let tsi = C_EPSSIL / COX;
    let tox = C_EPSOX / COX;
    let tox2 = tox * tox;
    let lc = (tsi * XJ).sqrt();
    let tnomk = TNOM + 273.15;
    let utnom = C_K * tnomk / C_QE;

    let l = VALIDATED_L * SCALE;
    let wf = VALIDATED_W / VALIDATED_NF;
    let w = wf * SCALE;
    let mut leff = if LL == 0.0 {
        l + DL + WDL / w
    } else {
        l + DL + WDL / w - LL * lexp(LLN * (1.0 / l).ln())
    };
    let mut weff = w + DW + LDW / l;
    leff = leff.max(1.0e-9);
    weff = weff.max(1.0e-9);

    let weff_nf = weff * VALIDATED_NF;
    let rwnedge = 1.0;

    let vto_a = VTO;
    let gamma_a = GAMMA;
    let kp_a = KP;
    let dvt_long = 0.0;
    let dvt_wide = 0.0;
    let dgamma_long = 0.0;
    let dgamma_wide = 0.0;
    let dvt_nf = 0.0;

    let leff_o_lr = leff / LR;
    let tmp_rsce = 1.0 - lexp(-leff_o_lr * leff_o_lr);
    let f_rsce = 2.0 * tmp_rsce / (COX * leff_o_lr);
    let dvt_rsce = QLR * f_rsce;
    let fn_rsce = 1.0 + NLR * f_rsce;
    let mgamma_rcse = fn_rsce.sqrt();
    let dphif_rsce = utnom * FLR * fn_rsce.ln();

    let weff_o_wr = weff / WR;
    let tmp_inwe = 1.0 - lexp(-weff_o_wr * weff_o_wr);
    let f_inwe = 2.0 * tmp_inwe / (COX * weff_o_wr);
    let dvt_inwe = -QWR * f_inwe;
    let mgamma_inwe = 1.0 / (1.0 + NWR * f_inwe).sqrt();

    let mkp_l = 1.0;
    let mkp_w = 1.0;
    let vto_dev = vto_a + SIGN * (dvt_long + dvt_wide + dvt_rsce + dvt_inwe + dvt_nf);
    let gamma_dev = gamma_a * mgamma_rcse * mgamma_inwe + dgamma_long + dgamma_wide;
    let phif_dev = PHIF + dphif_rsce;
    let kp_dev = kp_a * mkp_l * mkp_w;
    let etad_dev = ETAD;
    let ucrit_dev = UCRIT;

    let chshl = LETA0 + LETA / leff;
    let chshw = WETA / weff;
    let nuv = N0 + NCS * 3.0 * tox * chshl;
    let chshl_tsi = chshl * tsi;
    let chshw_tsi = chshw * tsi;

    let dt = t - tnomk;
    let rt = t / tnomk;
    let lnrt = rt.ln();
    let ut = C_K * t / C_QE;
    let ut2 = ut * ut;
    let sqrt_ut = ut.sqrt();
    let vto_dev_t = vto_dev - TCV * dt;
    let kp_dev_t = kp_dev * lexp(BEX * lnrt);
    let eta_t = ETA + TETA * dt;
    let e0_wt = E0 * lexp(TE0EX * lnrt);
    let e1_wt = E1 * lexp(TE1EX * lnrt);
    let ucrit_dev_t = ucrit_dev * lexp(UCEX * lnrt);
    let lambda_wt = LAMBDA + TLAMBDA * (rt - 1.0);
    let _ibb_t = IBB * (1.0 + IBBT * dt);
    let eg_tnom = 1.16 - (7.02e-4 * tnomk * tnomk) / (tnomk + 1108.0);
    let eg_t = 1.16 - (7.02e-4 * t * t) / (t + 1108.0);
    let phif_dev_t = phif_dev * rt + (-ut * 3.0 * lnrt + eg_t - eg_tnom * rt) / 2.0;

    let phif = maxa(phif_dev_t / ut, 0.0, 0.01);
    let vto = SIGN * vto_dev_t / ut;
    let gamma_b_dev = gamma_dev / sqrt_ut;
    let gamma_g = GAMMAG / sqrt_ut;
    let gamma_ov = GAMMAOV / sqrt_ut;
    let ucrit = ucrit_dev_t / (ut / leff);
    let ev = ut / (e0_wt * tsi);
    let tmp = e1_wt * tsi;
    let ev1 = ut2 / (tmp * tmp);
    let sqrtphif = phif.sqrt();
    let gamma_b_dev2 = gamma_b_dev * gamma_b_dev;
    let gamma_g2 = gamma_g * gamma_g;
    let gamma_ov2 = gamma_ov * gamma_ov;
    let vfb_ov = VFBOV / ut;
    let xb = XB / ut;
    let ub = EB * tox / XB;
    let dpd = gamma_b_dev2 / gamma_g2;

    let nq0 = 1.0 / (1.0 + (dpd * 2.0 * SQRT2 * sqrtphif / gamma_b_dev))
        + gamma_b_dev / (2.0 * SQRT2 * sqrtphif);
    let aqma = AQMA * lexp(ONE3RD * (COX * COX / ut).ln());
    let axetaqm2_3 = aqma * lexp(TWO3RDS * ETAQM.ln());
    let dqmi = ONE3RD
        * AQMI
        * lexp(TWO3RDS * (gamma_b_dev * COX * 0.5 / (sqrt_ut * phif)).ln())
        * (2.0 * SQRT2 * ETAQM * nq0 * sqrtphif / gamma_b_dev - 1.0);
    let inv_dqmip1 = 1.0 / (1.0 + dqmi);
    let dpsi0 = AQMI * lexp(TWO3RDS * (gamma_b_dev * COX * SQRT2 * sqrtphif).ln());
    let phi = phif * 2.0 + (4.0 * nq0 * sqrtphif * SQRT2 / gamma_b_dev).ln() + dpsi0;
    let sqrtphi = phi.sqrt();
    let nul = 3.0;
    let vbi = phi + nul;
    let sqrtvbi = vbi.sqrt();

    let vs_ext = vs_node - vb_node;
    let vd_ext = vd_node - vb_node;
    let vg_ext = vg_node - vb_node;
    let (d_gt_s_flag, d_gt_s, s_gt_d) = if SIGN * vd_ext >= SIGN * vs_ext {
        (1.0, 1.0, 0.0)
    } else {
        (-1.0, 0.0, 1.0)
    };
    let vd = SIGN * (d_gt_s * vd_ext + s_gt_d * vs_ext) / ut;
    let vs = SIGN * (d_gt_s * vs_ext + s_gt_d * vd_ext) / ut;
    let vg = SIGN * vg_ext / ut;

    let chsh_1w = 1.0 + chshw_tsi;
    let tmp_chsh1 = chshl_tsi / gamma_b_dev;
    let tmp_chsh2 =
        tmp_chsh1 * (maxa(vbi + vs, 0.0, POS_MIN).sqrt() + maxa(vbi + vd, 0.0, POS_MIN).sqrt());
    let chsh_1l = 1.0 - tmp_chsh2;
    let tmp_chsh3 = chshw_tsi * sqrtphi / gamma_b_dev;
    let tmp_chsh3b = tmp_chsh3 + tmp_chsh3;
    let chsh_1wl = 1.0 - tmp_chsh2 - tmp_chsh2 + tmp_chsh3b;
    let chsh_1wlpd = chsh_1w + dpd * chsh_1wl;
    let gamma_b_chsh = gamma_b_dev * chsh_1l / chsh_1w;
    let gamma_b_eff = gamma_b_dev * chsh_1l / chsh_1wlpd;
    let tmp_chsh4 = tmp_chsh1 * 2.0 * sqrtvbi;
    let chsh_1l0 = 1.0 - tmp_chsh4;
    let chsh_1wl0 = 1.0 - tmp_chsh4 - tmp_chsh4 + tmp_chsh3b;
    let chsh_1wlpd0 = chsh_1w + dpd * chsh_1wl0;
    let gamma_b_chsh0 = gamma_b_dev * chsh_1l0 / chsh_1w;
    let gamma_b_chsh2 = gamma_b_chsh * gamma_b_chsh;
    let gamma_b_eff2 = gamma_b_eff * gamma_b_eff;
    let gamma_b_chsh02 = gamma_b_chsh0 * gamma_b_chsh0;

    let tmp_vfb = 1.0 - tmp_chsh4 + tmp_chsh3;
    let vfb =
        vto - phi * (chsh_1w + dpd * tmp_vfb * tmp_vfb) - gamma_b_dev * (1.0 - tmp_chsh4) * sqrtphi;
    let vg_p = vg - vfb;
    let vg_p_chsh = vg_p / chsh_1w;
    let vg_p_chsh_pd = vg_p / chsh_1wlpd;
    let vg_p_chsh_pd0 = vg_p / chsh_1wlpd0;

    let mut tmp = vg_p_chsh * 0.5 - 3.0 * (1.0 + gamma_b_chsh * ONESQRT2);
    let psi_po = tmp + (tmp * tmp + 6.0 * vg_p_chsh).sqrt();
    tmp = vg_p_chsh * 0.5 - 3.0 * (1.0 + gamma_b_chsh0 * ONESQRT2);
    let psi_po0 = tmp + (tmp * tmp + 6.0 * vg_p_chsh).sqrt();

    let (psi_p, psi_p0) = if vg_p < 0.0 {
        let acc = (psi_po - vg_p_chsh) / gamma_b_chsh;
        let acc0 = (psi_po0 - vg_p_chsh) / gamma_b_chsh0;
        (
            -(1.0 - psi_po + acc * acc).ln(),
            -(1.0 - psi_po0 + acc0 * acc0).ln(),
        )
    } else {
        let one_m_epsilon = 1.0 - (-psi_po).exp();
        let tmp = (vg_p_chsh_pd - one_m_epsilon + gamma_b_eff2 * 0.25).sqrt() - gamma_b_eff * 0.5;
        let psi_p = tmp * tmp + one_m_epsilon;
        let one_m_epsilon = 1.0 - (-psi_po0).exp();
        let tmp =
            (vg_p_chsh_pd0 - one_m_epsilon + gamma_b_chsh02 * 0.25).sqrt() - gamma_b_chsh0 * 0.5;
        (psi_p, tmp * tmp + one_m_epsilon)
    };
    let sqrt_psi_p = maxa(psi_p, 1.0e-4, 1.0e-2).sqrt();
    let vp = psi_p - phi;
    let nv = chsh_1wlpd + gamma_b_dev * chsh_1l / (2.0 * sqrt_psi_p);

    let l0 = etad_dev * tsi * (2.0 * sqrtphi / gamma_b_dev).sqrt();
    let v_o_dibl = 4.0 + 40.0 * l0 / leff;
    let v_o_dibl2 = v_o_dibl * v_o_dibl;
    let dv_dibl = mina(vp, mina(vs, vd, v_o_dibl2), v_o_dibl2);
    let deltapsis = if l0 == 0.0 {
        0.0
    } else {
        let tmp = leff / (l0 + l0);
        (-tmp).exp()
            * (2.0 + SIGMAD * tmp * dv_dibl / (2.0 * phi))
            * ((nul + vs - dv_dibl) * (nul + vd - dv_dibl)).sqrt()
    };
    let vp_dibl = vp + deltapsis;

    let qs = qv(vp_dibl - vs, nuv);
    let if_ = qs * qs + qs;
    let xf2 = if_ + 0.25;
    let xf = qs + 0.5;

    let g_clm = 0.1;
    let e_clm = 2.0 / ucrit;
    let e_clm2 = e_clm * e_clm;
    let e_clmx2 = 2.0 * e_clm;
    let e_clmp2 = 2.0 + e_clm;
    let e_clmx2xqs = e_clmx2 * qs;
    let qsat =
        e_clmx2 * if_ / (e_clmp2 + e_clmx2xqs + (e_clmp2 * e_clmp2 + 4.0 * e_clmx2xqs).sqrt());
    let qs_qsat = qs - qsat;
    let qs_qsat2 = qs_qsat * qs_qsat;
    let mdm2 = 2.0 - DELTA;
    let e_clmxmdm2_2 = e_clm2 * mdm2 * mdm2;
    let tmp_vdsat1 = (2.0 * qsat + qsat.ln()) * (1.0 + e_clm * qs_qsat);
    let tmp_vdsat11 = g_clm + e_clm * mdm2 * qs_qsat;
    let tmp_vdsat2 =
        (1.0 + (2.0 * e_clm2 * mdm2 * mdm2 * qs_qsat2) / tmp_vdsat11 + e_clm2 * qs_qsat2).sqrt();
    let vdsat = vp - tmp_vdsat1 / tmp_vdsat2;
    let vdssat = maxa(vdsat - vs, 3.0, 4.0);
    let dv_clm = (ACLM / DELTA) * (4.0 * qsat + DELTA) / (qs + 1.0);
    let tmp_vdp1 = (vd - vs) * (1.0 + 4.0 * dv_clm / vdssat).sqrt();
    let tmp_vdp2 = ((tmp_vdp1 + vdssat) * (tmp_vdp1 + vdssat) + 4.0 * dv_clm * vdssat).sqrt();
    let tmp_vdp3 = ((tmp_vdp1 - vdssat) * (tmp_vdp1 - vdssat) + 4.0 * dv_clm * vdssat).sqrt();
    let vdp = 0.5 * (tmp_vdp2 - tmp_vdp3) + vs;
    let u_clm = 0.5 * e_clm * leff / lc * (vd - vdp);
    let alpha_clm = lc / (leff - 2.0 * lc);
    let deltal = lambda_wt
        * lc
        * ((alpha_clm + u_clm + (u_clm * u_clm + 2.0 * alpha_clm * u_clm + 1.0).sqrt())
            / (alpha_clm + 1.0))
            .ln();

    let qdp = qv(vp_dibl - vdp, nuv);
    let irp = qdp * qdp + qdp;
    let xrp2 = irp + 0.25;
    let xrp = qdp + 0.5;
    let qsqdp = qs + qdp;
    let qs_qdp = qs - qdp;
    let powqs_qdp2 = qs_qdp * qs_qdp;
    let qsqdpp1 = qsqdp + 1.0;
    let powqsqdpp1_2 = 1.0 / (qsqdpp1 * qsqdpp1);
    let i = if_ - irp;

    let nq = nq(psi_p, sqrt_psi_p, qs, qdp, dpd, gamma_b_chsh, gamma_g2, TG);
    let v_o = vg_p_chsh - psi_p0;
    let qr1 = 3.0 * ONESQRT2 * gamma_b_chsh;
    let qbo = if vg_p < 0.0 {
        vg_p_chsh - psi_p
    } else {
        vg_p_chsh / (1.0 + dpd) - psi_po
    };
    let dpsiv = axetaqm2_3
        * (lexp(
            TWO3RDS
                * (maxa(
                    0.25 * qbo * qbo + 4.0 * axetaqm2_3 * gamma_b_chsh2,
                    0.0,
                    POS_MIN,
                )
                .sqrt()
                    - 0.5 * qbo)
                    .ln(),
        ) - lexp(
            TWO3RDS
                * (maxa(qr1 * qr1 + 4.0 * axetaqm2_3 * gamma_b_chsh2, 0.0, POS_MIN).sqrt() - qr1)
                    .ln(),
        ));
    let v_o_qme = v_o + dpsiv;
    let qs_charge = qx(psi_p, nq, qs, qdp, powqs_qdp2, powqsqdpp1_2, inv_dqmip1);
    let qd_charge = qx(psi_p, nq, qdp, qs, powqs_qdp2, powqsqdpp1_2, inv_dqmip1);
    let qg_charge = qg(
        psi_p,
        qs,
        qdp,
        powqs_qdp2,
        powqsqdpp1_2,
        qsqdpp1,
        v_o_qme,
        gamma_g2,
        inv_dqmip1,
        TG,
    );
    let qi = qs_charge + qd_charge;
    let qb = qg_charge - qi;

    let beta_coul = THC / ((1.0 + nv * ZC * qs) * (1.0 + nv * ZC * qdp));
    let nu = nv * (1.0 - eta_t) - 1.0;
    let gpnu = gamma_b_eff * sqrt_psi_p + nu;
    let eq = qb + eta_t * nv * qi;
    let eq1 = gpnu * gpnu + nu * nu * (1.0 + if_ + if_ + irp + irp)
        - 8.0 * ONE3RD * nu * gpnu * (xf2 + xf * xrp + xrp2) / (xf + xrp);
    let beta_nom = 1.0 + ev * gamma_b_eff * sqrtphi + ev1 * gamma_b_eff2 * phi;
    let beta_denom = 1.0 + ev * eq + ev1 * eq1 + beta_coul;
    let beta_clm_denom = (1.0
        + 2.0 * e_clmxmdm2_2 * powqs_qdp2 / (g_clm + e_clm * mdm2 * qs_qdp)
        + e_clm2 * powqs_qdp2)
        .sqrt();
    let beta = kp_dev_t * (beta_nom / beta_denom) / beta_clm_denom;
    let i0 = 2.0 * nq * ut2 * beta * inv_dqmip1;
    let ispec = i0 * weff_nf / (leff - deltal) * rwnedge;

    let dits_factor = if PDITS == 0.0 {
        1.0
    } else {
        let f_dits = 1.0 / (1.0 + FPROUT * leff.sqrt() / (qi + 2.0));
        let va_dits =
            (f_dits / PDITS) * (1.0 + (1.0 + PDITSL * leff) * lexp(PDITSD * (vd - vs) * ut));
        let vdseff = vdssat - maxa(vdssat - (vd - vs) - DDITS, 0.0, 4.0 * DDITS * vdssat);
        1.0 + (vd - vs - vdseff) / va_dits
    };
    let ispec_dits = ispec * dits_factor;
    let ids = i * ispec_dits;
    let channel_id = SIGN * d_gt_s_flag * ids;
    let (igd, igs, igb, igd_ov, igs_ov) = ekv3_gate_currents(
        psi_p, v_o_qme, qs, if_, irp, nq, vg, vfb, vd, vs, gamma_g, gamma_g2, gamma_ov, gamma_ov2,
        vfb_ov, xb, ub, leff, weff_nf, ut2, tox2, KG, LOVIG, TG,
    );

    Ekv3IntrinsicCurrents {
        channel_id,
        igd,
        igs,
        igb,
        igd_ov,
        igs_ov,
    }
}

#[allow(clippy::too_many_arguments)]
fn ekv3_gate_currents(
    psi_p: Value,
    v_o_qme: Value,
    qs: Value,
    if_: Value,
    irp: Value,
    nq: Value,
    vg: Value,
    vfb: Value,
    vd: Value,
    vs: Value,
    gamma_g: Value,
    gamma_g2: Value,
    gamma_ov: Value,
    gamma_ov2: Value,
    vfb_ov: Value,
    xb: Value,
    ub: Value,
    leff: Value,
    weff_nf: Value,
    ut2: Value,
    tox2: Value,
    kg: Value,
    lovig: Value,
    tg: Value,
) -> (Value, Value, Value, Value, Value) {
    let (psi_ox, dpsi_dq) = if (psi_p > 0.0 && tg < 0.0) || (psi_p < 0.0 && tg > 0.0) {
        let v1_gc = (0.25 + (v_o_qme + 2.0 * qs) / gamma_g2).sqrt();
        let v2_gc = 0.5 + v1_gc;
        let psi_ox = (v_o_qme + 2.0 * qs) / v2_gc;
        let dpsi_dq =
            (2.0 / v2_gc) * (1.0 - (v_o_qme + 2.0 * qs) / (2.0 * v1_gc * v2_gc * gamma_g2));
        (psi_ox, dpsi_dq)
    } else {
        (v_o_qme + 2.0 * qs, 2.0)
    };

    let psi_x = psi_ox.abs() / xb;
    let p_tun = ekv3_tunneling_probability(psi_x, ub);
    let igo = qs * psi_ox * p_tun;
    let (nigc, nigd) = if vs == vd || psi_ox == 0.0 {
        let nigc = igo * nq;
        (nigc, nigc * 0.5)
    } else {
        let dq_dksi = (irp - if_) / (2.0 * qs + 1.0);
        let a_gc = dq_dksi * (1.0 / qs + dpsi_dq / psi_ox);
        let b_gc = if psi_x < 1.0 {
            let numerator = dq_dksi * dpsi_dq * (ub / xb) * (3.0 + psi_x);
            let denominator = 4.0 + 2.0 * (1.0 - psi_x).sqrt() * (2.0 + psi_x);
            if psi_ox > 0.0 {
                numerator / denominator
            } else {
                -numerator / denominator
            }
        } else {
            dpsi_dq * dq_dksi * ub / (psi_x * psi_ox)
        };
        let nigc = nq * igo * (2.0 + a_gc) / (2.0 - b_gc);
        let nigs = 0.5 * nq * igo * (3.0 + a_gc) / (3.0 - b_gc);
        (nigc, nigc - nigs)
    };

    let (igd, igs, igb) = if vg > vfb {
        let scale = 2.0 * kg * weff_nf * leff * ut2 / tox2;
        let ig = scale * nigc;
        let igd = scale * nigd;
        (igd, ig - igd, 0.0)
    } else {
        (
            0.0,
            0.0,
            kg * weff_nf * leff * psi_ox * psi_ox.abs() * ut2 * p_tun / tox2,
        )
    };

    let (igs_ov, igd_ov) = if lovig != 0.0 {
        let psi_ox_sovgc = ekv3_overlap_gate_oxide_potential(
            vg - vs,
            vfb_ov,
            gamma_g,
            gamma_g2,
            gamma_ov,
            gamma_ov2,
        );
        let psi_ox_dovgc = ekv3_overlap_gate_oxide_potential(
            vg - vd,
            vfb_ov,
            gamma_g,
            gamma_g2,
            gamma_ov,
            gamma_ov2,
        );
        let p_tun_sovgc = ekv3_tunneling_probability(psi_ox_sovgc.abs() / xb, ub);
        let p_tun_dovgc = ekv3_tunneling_probability(psi_ox_dovgc.abs() / xb, ub);
        let scale = kg * weff_nf * lovig * ut2 / tox2;
        (
            scale * psi_ox_sovgc * psi_ox_sovgc.abs() * p_tun_sovgc,
            scale * psi_ox_dovgc * psi_ox_dovgc.abs() * p_tun_dovgc,
        )
    } else {
        (0.0, 0.0)
    };

    (igd, igs, igb, igd_ov, igs_ov)
}

fn ekv3_overlap_gate_oxide_potential(
    vgo: Value,
    vfb_ov: Value,
    gamma_g: Value,
    gamma_g2: Value,
    gamma_ov: Value,
    gamma_ov2: Value,
) -> Value {
    if vgo > vfb_ov {
        let tmp = (vgo - vfb_ov + gamma_g2 * 0.25).sqrt() - gamma_g * 0.5;
        vgo - tmp * tmp
    } else {
        let tmp = (-vgo + vfb_ov + gamma_ov2 * 0.25).sqrt() - gamma_ov * 0.5;
        vgo + tmp * tmp
    }
}

fn ekv3_tunneling_probability(psi_x: Value, ub: Value) -> Value {
    if psi_x < 1.0 {
        let tmp = (1.0 - psi_x).sqrt();
        (-ub * (1.0 / (1.0 + tmp) + tmp)).exp()
    } else {
        (-ub / psi_x).exp()
    }
}

fn qv(v: Value, nuv: Value) -> Value {
    let vv = v / nuv;
    if vv > -0.6 {
        let z1 = 0.25 * (vv - 1.4 + (vv * (vv - 0.394_036) + 9.662_671).sqrt());
        let ln_z1 = z1.ln();
        let z2 = (vv - (2.0 * z1 + ln_z1)) / (2.0 * z1 + 1.0);
        z1 * (1.0 + z2 * (1.0 + 0.070 * z2)) * nuv
    } else {
        let ln_z1 = 0.5 * (vv - 0.201_491 - (vv * (vv + 0.402_982) + 2.446_562).sqrt());
        let z1 = ln_z1.exp();
        let z2 = (vv - (2.0 * z1 + ln_z1)) / (2.0 * z1 + 1.0);
        z1 * (1.0 + z2 * (1.0 + 0.483 * z2)) * nuv
    }
}

fn nq(
    psi_p: Value,
    sqrt_psi_p: Value,
    qs: Value,
    qd: Value,
    dpd: Value,
    gamma_b: Value,
    gamma_g2: Value,
    tg: Value,
) -> Value {
    let tmp_psi_sa = psi_p - qs - qd;
    let sqrt_psi_sa = maxa(tmp_psi_sa, 1.0e-4, 1.0e-2).sqrt();
    if tg < 0.0 {
        let z0 = 1.0 + dpd + gamma_b / (sqrt_psi_p + sqrt_psi_sa);
        let zk = 0.5 + dpd * sqrt_psi_sa / gamma_b;
        z0 / (zk + (zk * zk + z0 * (qs + qd) / gamma_g2).sqrt())
    } else {
        1.0 + gamma_b / (sqrt_psi_p + sqrt_psi_sa)
    }
}

fn qx(
    psi_p: Value,
    nq: Value,
    qs: Value,
    qd: Value,
    powqs_qd2: Value,
    powqsqd1_2: Value,
    inv_dqmip1: Value,
) -> Value {
    if psi_p > 2.0 {
        inv_dqmip1
            * nq
            * (1.0 / 3.0)
            * (qs + qd + qs + 0.5 * (1.0 + 0.8 * qs + 1.2 * qd) * powqs_qd2 * powqsqd1_2)
    } else {
        0.0
    }
}

fn qg(
    psi_p: Value,
    qs: Value,
    qd: Value,
    powqs_qd2: Value,
    _powqsqd1_2: Value,
    qsqd1: Value,
    v_o: Value,
    gamma_g2: Value,
    inv_dqmip1: Value,
    tg: Value,
) -> Value {
    if psi_p > 2.0 {
        if tg < 0.0 {
            let v1_qg = v_o + 2.0 * qs * inv_dqmip1;
            let v2_qg = v_o + 2.0 * qd * inv_dqmip1;
            let k1 = (0.25 + v1_qg / gamma_g2).sqrt();
            let k2 = (0.25 + v2_qg / gamma_g2).sqrt();
            let k12 = k1 + k2;
            let k12_2 = k12 * k12;
            let k12_3 = k12_2 * k12;
            v1_qg / (1.0 + 2.0 * k1)
                + v2_qg / (1.0 + 2.0 * k2)
                + inv_dqmip1
                    * (1.0 / 3.0)
                    * (powqs_qd2 / k12_3)
                    * (0.8 * (k12_2 + k1 * k2) / qsqd1 + 2.0 / gamma_g2)
        } else {
            v_o + qs + qd + inv_dqmip1 * (1.0 / 3.0) * powqs_qd2 / qsqd1
        }
    } else if psi_p > 0.0 {
        if tg < 0.0 {
            v_o / (0.5 + (0.25 + v_o / gamma_g2).sqrt())
        } else {
            v_o
        }
    } else if tg > 0.0 {
        v_o / (0.5 + (0.25 - v_o / gamma_g2).sqrt())
    } else {
        v_o
    }
}

fn lexp(x: Value) -> Value {
    const EXPL_THRESHOLD: Value = 80.0;
    const MAX_EXPL: Value = 5.540_622_384e34;
    const MIN_EXPL: Value = 1.804_851_387e-35;
    if x > EXPL_THRESHOLD {
        MAX_EXPL * (1.0 + x - EXPL_THRESHOLD)
    } else if x < -EXPL_THRESHOLD {
        MIN_EXPL
    } else {
        x.exp()
    }
}

fn maxa(x: Value, y: Value, a: Value) -> Value {
    0.5 * (x + y + ((x - y) * (x - y) + a).sqrt())
}

fn mina(x: Value, y: Value, a: Value) -> Value {
    0.5 * (x + y - ((x - y) * (x - y) + a).sqrt())
}

fn validate_model_params(
    mos_type: MosType,
    model_params: &HashMap<String, Value>,
) -> Result<&'static Ekv3ModelSpec, String> {
    let spec = match mos_type {
        MosType::Nmos => &EKV3_NMOS150_SPEC,
        MosType::Pmos => &EKV3_PMOS150_SPEC,
    };

    for (name, value) in model_params {
        let upper = name.to_ascii_uppercase();
        if let Some((_, expected)) = spec
            .params
            .iter()
            .chain(spec.optional_params.iter())
            .find(|(param, _)| *param == upper)
        {
            if !approx_eq(*value, *expected) {
                return Err(format!(
                    "native EKV3 {} slice requires model parameter {upper}={expected}; got {value}",
                    spec.label
                ));
            }
        } else {
            return Err(format!(
                "native EKV3 {} slice does not support model parameter {name}={value}; unsupported EKV3 cards remain fail-closed",
                spec.label
            ));
        }
    }

    for (name, expected) in spec.params {
        let actual = finite_model(spec, model_params, name)?;
        if !approx_eq(actual, *expected) {
            return Err(format!(
                "native EKV3 {} slice requires model parameter {name}={expected}; got {actual}",
                spec.label
            ));
        }
    }

    Ok(spec)
}

fn validate_instance_params(
    spec: &'static Ekv3ModelSpec,
    params: &[(String, Value)],
) -> Result<(), String> {
    let w = required_instance_alias(spec, params, "W", "WIDTH", VALIDATED_W)?;
    let l = required_instance_alias(spec, params, "L", "LENGTH", VALIDATED_L)?;
    let nf = required_instance(spec, params, "NF", VALIDATED_NF)?;
    require_instance(spec, "W", w, VALIDATED_W)?;
    require_instance(spec, "L", l, VALIDATED_L)?;
    require_instance(spec, "NF", nf, VALIDATED_NF)?;

    for (name, value) in params {
        if EKV3_INSTANCE_PARAMS
            .iter()
            .any(|param| name.eq_ignore_ascii_case(param))
        {
            continue;
        }
        return Err(format!(
            "native EKV3 {} slice does not support instance parameter {name}={value}; unsupported EKV3 instances remain fail-closed",
            spec.label
        ));
    }
    Ok(())
}

fn required_instance_alias(
    spec: &'static Ekv3ModelSpec,
    params: &[(String, Value)],
    primary: &str,
    alias: &str,
    expected: Value,
) -> Result<Value, String> {
    match (
        instance_param(params, primary),
        instance_param(params, alias),
    ) {
        (Some(p), Some(a)) if !approx_eq(p, a) => Err(format!(
            "native EKV3 {} slice instance uses conflicting {primary}/{alias} aliases {p} and {a}",
            spec.label
        )),
        (Some(value), _) | (_, Some(value)) if value.is_finite() => Ok(value),
        (Some(value), _) | (_, Some(value)) => Err(format!(
            "native EKV3 {} slice instance parameter {primary}/{alias} must be finite; got {value}",
            spec.label
        )),
        (None, None) => Err(format!(
            "native EKV3 {} slice requires explicit instance parameter {primary}={expected} or {alias}={expected}",
            spec.label
        )),
    }
}

fn required_instance(
    spec: &'static Ekv3ModelSpec,
    params: &[(String, Value)],
    name: &str,
    expected: Value,
) -> Result<Value, String> {
    match instance_param(params, name) {
        Some(value) if value.is_finite() => Ok(value),
        Some(value) => Err(format!(
            "native EKV3 {} slice instance parameter {name} must be finite; got {value}",
            spec.label
        )),
        None => Err(format!(
            "native EKV3 {} slice requires explicit instance parameter {name}={expected}",
            spec.label
        )),
    }
}

fn require_instance(
    spec: &'static Ekv3ModelSpec,
    name: &str,
    actual: Value,
    expected: Value,
) -> Result<(), String> {
    if approx_eq(actual, expected) {
        Ok(())
    } else {
        Err(format!(
            "native EKV3 {} slice requires instance parameter {name}={expected}; got {actual}",
            spec.label
        ))
    }
}

fn instance_param(params: &[(String, Value)], name: &str) -> Option<Value> {
    params
        .iter()
        .rev()
        .find(|(param, _)| param.eq_ignore_ascii_case(name))
        .map(|(_, value)| *value)
}

fn finite_model(
    spec: &'static Ekv3ModelSpec,
    params: &HashMap<String, Value>,
    name: &str,
) -> Result<Value, String> {
    params
        .get(&name.to_ascii_uppercase())
        .copied()
        .filter(|value| value.is_finite())
        .ok_or_else(|| {
            format!(
                "native EKV3 {} slice requires model parameter {name}",
                spec.label
            )
        })
}

fn oracle_transadmittance_at(frequency_hz: Value) -> Value {
    if frequency_hz <= VANOISE_ORACLE[0].frequency {
        return oracle_transadmittance(VANOISE_ORACLE[0]);
    }
    let last = VANOISE_ORACLE[VANOISE_ORACLE.len() - 1];
    if frequency_hz >= last.frequency {
        return oracle_transadmittance(last);
    }
    let upper = VANOISE_ORACLE.partition_point(|point| point.frequency < frequency_hz);
    let lo = VANOISE_ORACLE[upper - 1];
    let hi = VANOISE_ORACLE[upper];
    let y0 = oracle_transadmittance(lo);
    let y1 = oracle_transadmittance(hi);
    if frequency_hz == hi.frequency {
        return y1;
    }
    if frequency_hz == lo.frequency {
        return y0;
    }
    let t = (frequency_hz.ln() - lo.frequency.ln()) / (hi.frequency.ln() - lo.frequency.ln());
    (y0.ln() + t * (y1.ln() - y0.ln())).exp()
}

fn oracle_transadmittance(point: VanoiseOraclePoint) -> Value {
    let input_density = point.sqrt_inoise * point.sqrt_inoise;
    let output_density = point.cbrt_onoise * point.cbrt_onoise * point.cbrt_onoise;
    let gain = (output_density / input_density).sqrt();
    gain / vanoise_drain_fixture_impedance(point.frequency)
}

fn vanoise_drain_fixture_impedance(frequency_hz: Value) -> Value {
    let omega = 2.0 * std::f64::consts::PI * frequency_hz;
    let susceptance = omega * VANOISE_DRAIN_C_F - 1.0 / (omega * VANOISE_DRAIN_L_H);
    1.0 / susceptance.abs()
}

fn approx_eq(actual: Value, expected: Value) -> bool {
    let tol = 1.0e-12_f64.max(expected.abs() * 1.0e-9);
    actual.is_finite() && (actual - expected).abs() <= tol
}

fn stamp(matrix: &mut impl MatrixStamper, row: NodeId, col: NodeId, value: Value) {
    if row != 0 && col != 0 && value != 0.0 {
        matrix.stamp(row, col, value);
    }
}

fn add_if_nonzero(
    add_real: &mut impl FnMut(NodeId, NodeId, Value),
    row: NodeId,
    col: NodeId,
    value: Value,
) {
    if row != 0 && col != 0 && value != 0.0 {
        add_real(row, col, value);
    }
}

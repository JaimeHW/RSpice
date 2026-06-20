use super::{Mextram504Model, Mextram504Nodes, Mextram504Op};
use crate::Value;

#[allow(dead_code)]
const VEXLIM: Value = 400.0;
#[allow(dead_code)]
const NODE_COUNT: usize = 12;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub(super) enum NodeSlot {
    C = 0,
    B = 1,
    E = 2,
    S = 3,
    E1 = 4,
    B1 = 5,
    B2 = 6,
    C1 = 7,
    C2 = 8,
    C3 = 9,
    C4 = 10,
    Noi = 11,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Mextram504BranchCurrents {
    pub ic1c2: Value,
    pub in_: Value,
    pub ib1_s: Value,
    pub ib1: Value,
    pub ib2: Value,
    pub izteb: Value,
    pub isub: Value,
    pub xisub: Value,
    pub isf: Value,
    pub ib1b2: Value,
    pub iavl: Value,
    pub emitter_res: Value,
    pub base_res: Value,
    pub xiex: Value,
    pub collector_contact: Value,
    pub collector_extrinsic: Value,
    pub collector_intrinsic: Value,
    pub ib3: Value,
    pub iex: Value,
    pub noise_ground: Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(super) struct ProbeVoltages {
    pub s_gnd: Value,
    pub b_gnd: Value,
    pub b2_gnd: Value,
    pub b1_gnd: Value,
    pub e_gnd: Value,
    pub c1_gnd: Value,
    pub c2_gnd: Value,
    pub e1_gnd: Value,
    pub s_c: Value,
    pub b_s: Value,
    pub s_e: Value,
    pub c_e: Value,
    pub noi_gnd: Value,
    pub c3_c1: Value,
    pub c3_c4: Value,
    pub c4_c1: Value,
    pub b_c: Value,
    pub b_e: Value,
    pub b_b1: Value,
    pub e_e1: Value,
    pub c1_c2: Value,
    pub s_c1: Value,
    pub b1_b2: Value,
    pub b1_e1: Value,
    pub b2_e1: Value,
    pub b2_c2: Value,
    pub b2_c1: Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Mextram504LimiterMemory {
    pub vb2c1: Value,
    pub vb2c2: Value,
    pub vb1e1: Value,
    pub vb1b2: Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Mextram504Biases {
    pub vb2c1: Value,
    pub vb2c2: Value,
    pub vb2e1: Value,
    pub vb1e1: Value,
    pub vb1b2: Value,
    pub vsc1: Value,
    pub vc1c2: Value,
    pub vee1: Value,
    pub vbb1: Value,
    pub vbe: Value,
    pub vbc: Value,
    pub vc4c1: Value,
    pub vc3c4: Value,
    pub vb1c4: Value,
    pub vcc3: Value,
    pub vbc3: Value,
    pub vsc4: Value,
    pub vsc3: Value,
    pub limiter_applied: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LeadSlot {
    C,
    B,
    E,
    S,
}

pub fn evaluate_dc(
    _model: &Mextram504Model,
    _nodes: Mextram504Nodes,
    _voltages: &[Value],
) -> Mextram504Op {
    panic!(
        "native MEXTRAM 504 evaluator is not complete; LEVEL=504 routing must remain fail-closed"
    )
}

#[inline]
#[allow(dead_code)]
fn exp_lin(x: Value) -> Value {
    if x < VEXLIM {
        x.exp()
    } else {
        VEXLIM.exp() * (1.0 + (x - VEXLIM))
    }
}

#[inline]
#[allow(dead_code)]
fn trunc_ev(val: Value, vprev: Value, vmin: Value, vmax: Value) -> Value {
    let mut result = val;
    if val > vmax {
        if vprev > vmax - 0.05 {
            if val - vprev > 0.05 {
                result = vprev + 0.05;
            }
        } else {
            result = vmax;
        }
    } else if val < vmin {
        if vprev < 0.9 * vmin {
            let reverse_limit = 1.5 * vprev + 0.10 * vmin;
            if val < reverse_limit {
                result = reverse_limit;
            }
        } else {
            result = vmin;
        }
    }
    result
}

#[inline]
#[allow(dead_code)]
fn typed_trunc_ev(val: Value, vprev: Value, type_sign: Value, vmin: Value, vmax: Value) -> Value {
    type_sign * trunc_ev(type_sign * val, type_sign * vprev, vmin, vmax)
}

#[allow(dead_code)]
impl ProbeVoltages {
    pub fn from_solution(nodes: Mextram504Nodes, voltages: &[Value], type_sign: Value) -> Self {
        let c = node_voltage(voltages, nodes.c);
        let b = node_voltage(voltages, nodes.b);
        let e = node_voltage(voltages, nodes.e);
        let s = node_voltage(voltages, nodes.s);
        let e1 = node_voltage(voltages, nodes.e1);
        let b1 = node_voltage(voltages, nodes.b1);
        let b2 = node_voltage(voltages, nodes.b2);
        let c1 = node_voltage(voltages, nodes.c1);
        let c2 = node_voltage(voltages, nodes.c2);
        let c3 = node_voltage(voltages, nodes.c3);
        let c4 = node_voltage(voltages, nodes.c4);
        let noi = node_voltage(voltages, nodes.noi);

        Self {
            s_gnd: s,
            b_gnd: b,
            b2_gnd: b2,
            b1_gnd: b1,
            e_gnd: e,
            c1_gnd: c1,
            c2_gnd: c2,
            e1_gnd: e1,
            s_c: s - c,
            b_s: b - s,
            s_e: s - e,
            c_e: c - e,
            noi_gnd: noi,
            c3_c1: c3 - c1,
            c3_c4: c3 - c4,
            c4_c1: c4 - c1,
            b_c: b - c,
            b_e: b - e,
            b_b1: b - b1,
            e_e1: e - e1,
            c1_c2: c1 - c2,
            s_c1: s - c1,
            b1_b2: type_sign * (b1 - b2),
            b1_e1: type_sign * (b1 - e1),
            b2_e1: b2 - e1,
            b2_c2: type_sign * (b2 - c2),
            b2_c1: type_sign * (b2 - c1),
        }
    }
}

#[allow(dead_code)]
impl Mextram504Biases {
    pub fn from_probes(
        model: &Mextram504Model,
        probes: &ProbeVoltages,
        memory: Mextram504LimiterMemory,
        voltage_limiter_enabled: bool,
    ) -> Self {
        let (vb2c1, vb2c2, vb1e1, vb1b2, limiter_applied) = if voltage_limiter_enabled {
            let vb2c1 = trunc_ev(probes.b2_c1, memory.vb2c1, -model.ver, 0.7);
            let vb2c2 = trunc_ev(probes.b2_c2, memory.vb2c2, -model.ver, 0.7);
            let vb1e1 = trunc_ev(probes.b1_e1, memory.vb1e1, -model.ver, 0.7);
            let vb1b2 = trunc_ev(probes.b1_b2, memory.vb1b2, -model.ver, 0.7);
            let limiter_applied = vb2c1 != probes.b2_c1
                || vb2c2 != probes.b2_c2
                || vb1e1 != probes.b1_e1
                || vb1b2 != probes.b1_b2;
            (vb2c1, vb2c2, vb1e1, vb1b2, limiter_applied)
        } else {
            (
                probes.b2_c1,
                probes.b2_c2,
                probes.b1_e1,
                probes.b1_b2,
                false,
            )
        };

        let type_sign = model.type_;
        let vb2e1 = type_sign * probes.b2_e1;
        let vsc1 = type_sign * probes.s_c1;
        let vc1c2 = type_sign * probes.c1_c2;
        let vee1 = type_sign * probes.e_e1;
        let vbb1 = type_sign * probes.b_b1;
        let vbe = type_sign * probes.b_e;
        let vbc = type_sign * probes.b_c;

        let (vc4c1, vc3c4) = match (model.rcblx > 0.0, model.rcbli > 0.0) {
            (true, true) => (type_sign * probes.c4_c1, type_sign * probes.c3_c4),
            (true, false) => (0.0, type_sign * probes.c3_c1),
            (false, true) => (type_sign * probes.c4_c1, 0.0),
            (false, false) => (0.0, 0.0),
        };

        let vb1c4 = vb1b2 + vb2c2 - vc1c2 - vc4c1;
        let vcc3 = -vbc + vbb1 + vb1c4 - vc3c4;
        let vbc3 = vbc + vcc3;
        let vsc4 = vsc1 - vc4c1;
        let vsc3 = vsc4 - vc3c4;

        Self {
            vb2c1,
            vb2c2,
            vb2e1,
            vb1e1,
            vb1b2,
            vsc1,
            vc1c2,
            vee1,
            vbb1,
            vbe,
            vbc,
            vc4c1,
            vc3c4,
            vb1c4,
            vcc3,
            vbc3,
            vsc4,
            vsc3,
            limiter_applied,
        }
    }
}

#[allow(dead_code)]
pub(super) fn static_contributions(
    branches: &Mextram504BranchCurrents,
    type_sign: Value,
    rcblx: Value,
    rcbli: Value,
) -> [Value; NODE_COUNT] {
    let mut currents = [0.0; NODE_COUNT];

    stamp_branch(
        &mut currents,
        NodeSlot::C1,
        NodeSlot::C2,
        type_sign * branches.ic1c2,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::C2,
        NodeSlot::E1,
        type_sign * branches.in_,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B1,
        NodeSlot::E1,
        type_sign * branches.ib1_s,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B2,
        NodeSlot::E1,
        type_sign * (branches.ib1 + branches.ib2 - branches.izteb),
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B1,
        NodeSlot::S,
        type_sign * branches.isub,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B,
        NodeSlot::S,
        type_sign * branches.xisub,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::S,
        NodeSlot::C1,
        type_sign * branches.isf,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B1,
        NodeSlot::B2,
        type_sign * branches.ib1b2,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B2,
        NodeSlot::C2,
        type_sign * -branches.iavl,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::E,
        NodeSlot::E1,
        type_sign * branches.emitter_res,
    );
    stamp_branch(
        &mut currents,
        NodeSlot::B,
        NodeSlot::B1,
        type_sign * branches.base_res,
    );

    if rcblx > 0.0 {
        stamp_branch(
            &mut currents,
            NodeSlot::B,
            NodeSlot::C3,
            type_sign * branches.xiex,
        );
        stamp_branch(
            &mut currents,
            NodeSlot::C,
            NodeSlot::C3,
            type_sign * branches.collector_contact,
        );
        if rcbli > 0.0 {
            stamp_branch(
                &mut currents,
                NodeSlot::C4,
                NodeSlot::C1,
                type_sign * branches.collector_intrinsic,
            );
            stamp_branch(
                &mut currents,
                NodeSlot::B1,
                NodeSlot::C4,
                type_sign * (branches.ib3 + branches.iex),
            );
            stamp_branch(
                &mut currents,
                NodeSlot::C3,
                NodeSlot::C4,
                type_sign * branches.collector_extrinsic,
            );
        } else {
            stamp_branch(
                &mut currents,
                NodeSlot::B1,
                NodeSlot::C1,
                type_sign * (branches.ib3 + branches.iex),
            );
            stamp_branch(
                &mut currents,
                NodeSlot::C3,
                NodeSlot::C1,
                type_sign * branches.collector_extrinsic,
            );
        }
    } else if rcbli > 0.0 {
        stamp_branch(
            &mut currents,
            NodeSlot::B,
            NodeSlot::C4,
            type_sign * branches.xiex,
        );
        stamp_branch(
            &mut currents,
            NodeSlot::C,
            NodeSlot::C4,
            type_sign * branches.collector_contact,
        );
        stamp_branch(
            &mut currents,
            NodeSlot::C4,
            NodeSlot::C1,
            type_sign * branches.collector_intrinsic,
        );
        stamp_branch(
            &mut currents,
            NodeSlot::B1,
            NodeSlot::C4,
            type_sign * (branches.ib3 + branches.iex),
        );
    } else {
        stamp_branch(
            &mut currents,
            NodeSlot::B,
            NodeSlot::C1,
            type_sign * branches.xiex,
        );
        stamp_branch(
            &mut currents,
            NodeSlot::C,
            NodeSlot::C1,
            type_sign * branches.collector_contact,
        );
        stamp_branch(
            &mut currents,
            NodeSlot::B1,
            NodeSlot::C1,
            type_sign * (branches.ib3 + branches.iex),
        );
        stamp_branch(
            &mut currents,
            NodeSlot::C3,
            NodeSlot::C1,
            type_sign * branches.collector_extrinsic,
        );
    }

    currents[NodeSlot::Noi as usize] += branches.noise_ground;

    currents
}

#[allow(dead_code)]
pub(super) fn lead_currents_from_static_contributions(
    static_contributions: &[Value; NODE_COUNT],
    node_map: &[Option<LeadSlot>; NODE_COUNT],
) -> Mextram504Op {
    let mut op = Mextram504Op::default();
    for (current, lead) in static_contributions.iter().zip(node_map) {
        match lead {
            Some(LeadSlot::C) => op.source_c += current,
            Some(LeadSlot::B) => op.source_b += current,
            Some(LeadSlot::E) => op.source_e += current,
            Some(LeadSlot::S) => op.source_s += current,
            None => {}
        }
    }
    op
}

#[inline]
#[allow(dead_code)]
fn node_voltage(voltages: &[Value], node: usize) -> Value {
    if node == 0 { 0.0 } else { voltages[node - 1] }
}

#[inline]
#[allow(dead_code)]
fn stamp_branch(currents: &mut [Value; NODE_COUNT], pos: NodeSlot, neg: NodeSlot, current: Value) {
    currents[pos as usize] += current;
    currents[neg as usize] -= current;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(got: Value, expected: Value) {
        let scale = expected.abs().max(1.0);
        assert!(
            (got - expected).abs() <= 1.0e-14 * scale,
            "got {got:.17e}, expected {expected:.17e}"
        );
    }

    #[test]
    fn exp_lin_matches_xyce_frontdef_below_and_above_vexlim() {
        assert_close(exp_lin(0.0), 1.0);
        assert_close(exp_lin(399.0), 399.0_f64.exp());

        let at_limit = 400.0_f64.exp();
        assert_close(exp_lin(400.0), at_limit);
        assert_close(exp_lin(402.5), at_limit * 3.5);
    }

    #[test]
    fn trunc_ev_limits_large_forward_steps_like_xyce() {
        assert_close(trunc_ev(0.9, 0.60, -2.5, 0.7), 0.7);
        assert_close(trunc_ev(0.9, 0.68, -2.5, 0.7), 0.73);
        assert_close(trunc_ev(0.72, 0.68, -2.5, 0.7), 0.72);
    }

    #[test]
    fn trunc_ev_limits_large_reverse_steps_like_xyce() {
        assert_close(trunc_ev(-3.0, -1.0, -2.5, 0.7), -2.5);
        assert_close(trunc_ev(-5.0, -3.0, -2.5, 0.7), -4.75);
        assert_close(trunc_ev(-4.6, -3.0, -2.5, 0.7), -4.6);
    }

    #[test]
    fn typed_trunc_ev_applies_polarity_before_and_after_limiting() {
        assert_close(typed_trunc_ev(0.9, 0.60, 1.0, -2.5, 0.7), 0.7);
        assert_close(typed_trunc_ev(-0.9, -0.60, -1.0, -2.5, 0.7), -0.7);
    }

    #[test]
    fn static_contributions_match_xyce_branch_signs_for_collapsed_collector() {
        let branches = Mextram504BranchCurrents {
            ic1c2: 10.0,
            in_: 20.0,
            ib1_s: 30.0,
            ib1: 1.0,
            ib2: 2.0,
            izteb: 0.5,
            isub: 4.0,
            xisub: 5.0,
            isf: 6.0,
            ib1b2: 7.0,
            iavl: 8.0,
            emitter_res: 9.0,
            base_res: 11.0,
            xiex: 12.0,
            collector_contact: 13.0,
            ib3: 14.0,
            iex: 15.0,
            ..Mextram504BranchCurrents::default()
        };

        let got = static_contributions(&branches, 1.0, 0.0, 0.0);

        assert_close(got[NodeSlot::C as usize], 13.0);
        assert_close(got[NodeSlot::B as usize], 28.0);
        assert_close(got[NodeSlot::E as usize], 9.0);
        assert_close(got[NodeSlot::S as usize], -3.0);
        assert_close(got[NodeSlot::E1 as usize], -61.5);
        assert_close(got[NodeSlot::B1 as usize], 59.0);
        assert_close(got[NodeSlot::B2 as usize], -12.5);
        assert_close(got[NodeSlot::C1 as usize], -50.0);
        assert_close(got[NodeSlot::C2 as usize], 18.0);
        assert_close(got[NodeSlot::C3 as usize], 0.0);
        assert_close(got[NodeSlot::C4 as usize], 0.0);
        assert_close(got.iter().sum(), 0.0);
    }

    fn collector_path_fixture() -> Mextram504BranchCurrents {
        Mextram504BranchCurrents {
            xiex: 1.0,
            collector_contact: 2.0,
            collector_extrinsic: 3.0,
            collector_intrinsic: 4.0,
            ib3: 5.0,
            iex: 6.0,
            ..Mextram504BranchCurrents::default()
        }
    }

    #[test]
    fn static_contributions_project_full_distributed_collector_path() {
        let got = static_contributions(&collector_path_fixture(), 1.0, 1.0, 1.0);

        assert_close(got[NodeSlot::C as usize], 2.0);
        assert_close(got[NodeSlot::B as usize], 1.0);
        assert_close(got[NodeSlot::B1 as usize], 11.0);
        assert_close(got[NodeSlot::C1 as usize], -4.0);
        assert_close(got[NodeSlot::C3 as usize], 0.0);
        assert_close(got[NodeSlot::C4 as usize], -10.0);
        assert_close(got.iter().sum(), 0.0);
    }

    #[test]
    fn static_contributions_project_extrinsic_only_collector_path() {
        let got = static_contributions(&collector_path_fixture(), 1.0, 1.0, 0.0);

        assert_close(got[NodeSlot::C as usize], 2.0);
        assert_close(got[NodeSlot::B as usize], 1.0);
        assert_close(got[NodeSlot::B1 as usize], 11.0);
        assert_close(got[NodeSlot::C1 as usize], -14.0);
        assert_close(got[NodeSlot::C3 as usize], 0.0);
        assert_close(got[NodeSlot::C4 as usize], 0.0);
        assert_close(got.iter().sum(), 0.0);
    }

    #[test]
    fn static_contributions_project_intrinsic_only_collector_path() {
        let got = static_contributions(&collector_path_fixture(), 1.0, 0.0, 1.0);

        assert_close(got[NodeSlot::C as usize], 2.0);
        assert_close(got[NodeSlot::B as usize], 1.0);
        assert_close(got[NodeSlot::B1 as usize], 11.0);
        assert_close(got[NodeSlot::C1 as usize], -4.0);
        assert_close(got[NodeSlot::C3 as usize], 0.0);
        assert_close(got[NodeSlot::C4 as usize], -10.0);
        assert_close(got.iter().sum(), 0.0);
    }

    #[test]
    fn static_contributions_apply_pnp_type_sign_to_all_branches() {
        let branches = Mextram504BranchCurrents {
            ic1c2: 10.0,
            xiex: 12.0,
            collector_contact: 13.0,
            ib3: 14.0,
            iex: 15.0,
            ..Mextram504BranchCurrents::default()
        };

        let npn = static_contributions(&branches, 1.0, 0.0, 0.0);
        let pnp = static_contributions(&branches, -1.0, 0.0, 0.0);

        for (npn_value, pnp_value) in npn.into_iter().zip(pnp) {
            assert_close(pnp_value, -npn_value);
        }
    }

    #[test]
    fn static_contributions_include_xyce_noise_node_ground_stamp() {
        let branches = Mextram504BranchCurrents {
            noise_ground: 3.25,
            ..Mextram504BranchCurrents::default()
        };

        let got = static_contributions(&branches, 1.0, 0.0, 0.0);

        assert_close(got[NodeSlot::Noi as usize], 3.25);
        assert_close(got.iter().sum(), 3.25);
    }

    #[test]
    fn probe_voltages_match_xyce_solution_extraction_and_type_premultiply() {
        let nodes = Mextram504Nodes {
            c: 1,
            b: 2,
            e: 3,
            s: 4,
            e1: 5,
            b1: 6,
            b2: 7,
            c1: 8,
            c2: 9,
            c3: 10,
            c4: 11,
            noi: 12,
        };
        let voltages = [
            10.0, 20.0, 30.0, 40.0, 5.0, 6.0, 7.0, 8.0, 9.0, 13.0, 14.0, 15.0,
        ];

        let npn = ProbeVoltages::from_solution(nodes, &voltages, 1.0);
        assert_close(npn.s_gnd, 40.0);
        assert_close(npn.b_gnd, 20.0);
        assert_close(npn.b2_gnd, 7.0);
        assert_close(npn.b1_gnd, 6.0);
        assert_close(npn.e_gnd, 30.0);
        assert_close(npn.c1_gnd, 8.0);
        assert_close(npn.c2_gnd, 9.0);
        assert_close(npn.e1_gnd, 5.0);
        assert_close(npn.s_c, 30.0);
        assert_close(npn.b_s, -20.0);
        assert_close(npn.s_e, 10.0);
        assert_close(npn.c_e, -20.0);
        assert_close(npn.noi_gnd, 15.0);
        assert_close(npn.c3_c1, 5.0);
        assert_close(npn.c3_c4, -1.0);
        assert_close(npn.c4_c1, 6.0);
        assert_close(npn.b_c, 10.0);
        assert_close(npn.b_e, -10.0);
        assert_close(npn.b_b1, 14.0);
        assert_close(npn.e_e1, 25.0);
        assert_close(npn.c1_c2, -1.0);
        assert_close(npn.s_c1, 32.0);
        assert_close(npn.b1_b2, -1.0);
        assert_close(npn.b1_e1, 1.0);
        assert_close(npn.b2_e1, 2.0);
        assert_close(npn.b2_c2, -2.0);
        assert_close(npn.b2_c1, -1.0);

        let pnp = ProbeVoltages::from_solution(nodes, &voltages, -1.0);
        assert_close(pnp.b1_b2, 1.0);
        assert_close(pnp.b1_e1, -1.0);
        assert_close(pnp.b2_e1, 2.0);
        assert_close(pnp.b2_c2, 2.0);
        assert_close(pnp.b2_c1, 1.0);
    }

    #[test]
    fn lead_currents_sum_static_contributions_through_xyce_node_map() {
        let mut static_values = [0.0; NODE_COUNT];
        static_values[NodeSlot::C as usize] = 1.0;
        static_values[NodeSlot::B as usize] = 2.0;
        static_values[NodeSlot::E as usize] = 3.0;
        static_values[NodeSlot::S as usize] = 4.0;
        static_values[NodeSlot::C1 as usize] = 10.0;
        static_values[NodeSlot::C2 as usize] = 20.0;
        static_values[NodeSlot::Noi as usize] = 99.0;
        let node_map = [
            Some(LeadSlot::C),
            Some(LeadSlot::B),
            Some(LeadSlot::E),
            Some(LeadSlot::S),
            None,
            None,
            None,
            Some(LeadSlot::C),
            Some(LeadSlot::B),
            None,
            None,
            None,
        ];

        let op = lead_currents_from_static_contributions(&static_values, &node_map);

        assert_close(op.source_c, 11.0);
        assert_close(op.source_b, 22.0);
        assert_close(op.source_e, 3.0);
        assert_close(op.source_s, 4.0);
    }

    fn probe_fixture(type_sign: Value) -> ProbeVoltages {
        let nodes = Mextram504Nodes {
            c: 1,
            b: 2,
            e: 3,
            s: 4,
            e1: 5,
            b1: 6,
            b2: 7,
            c1: 8,
            c2: 9,
            c3: 10,
            c4: 11,
            noi: 12,
        };
        let voltages = [
            10.0, 20.0, 30.0, 40.0, 5.0, 6.0, 7.0, 8.0, 9.0, 13.0, 14.0, 15.0,
        ];
        ProbeVoltages::from_solution(nodes, &voltages, type_sign)
    }

    #[test]
    fn biases_match_xyce_unlimited_npn_collapsed_collector_algebra() {
        let model = Mextram504Model::from_params(
            &std::collections::HashMap::new(),
            &std::collections::HashMap::new(),
            super::super::Mextram504Polarity::Npn,
        );

        let biases = Mextram504Biases::from_probes(
            &model,
            &probe_fixture(1.0),
            Mextram504LimiterMemory::default(),
            false,
        );

        assert_close(biases.vb2c1, -1.0);
        assert_close(biases.vb2c2, -2.0);
        assert_close(biases.vb2e1, 2.0);
        assert_close(biases.vb1e1, 1.0);
        assert_close(biases.vb1b2, -1.0);
        assert_close(biases.vsc1, 32.0);
        assert_close(biases.vc1c2, -1.0);
        assert_close(biases.vee1, 25.0);
        assert_close(biases.vbb1, 14.0);
        assert_close(biases.vbe, -10.0);
        assert_close(biases.vbc, 10.0);
        assert_close(biases.vc4c1, 0.0);
        assert_close(biases.vc3c4, 0.0);
        assert_close(biases.vb1c4, -2.0);
        assert_close(biases.vcc3, 2.0);
        assert_close(biases.vbc3, 12.0);
        assert_close(biases.vsc4, 32.0);
        assert_close(biases.vsc3, 32.0);
        assert!(!biases.limiter_applied);
    }

    #[test]
    fn biases_apply_xyce_trunc_ev_to_four_limited_probes_only() {
        let model = Mextram504Model::from_params(
            &std::collections::HashMap::new(),
            &std::collections::HashMap::new(),
            super::super::Mextram504Polarity::Npn,
        );
        let probes = ProbeVoltages {
            b2_c1: 0.9,
            b2_c2: -5.0,
            b1_e1: 0.9,
            b1_b2: -5.0,
            b2_e1: 0.9,
            ..probe_fixture(1.0)
        };
        let memory = Mextram504LimiterMemory {
            vb2c1: 0.68,
            vb2c2: -3.0,
            vb1e1: 0.60,
            vb1b2: -1.0,
        };

        let biases = Mextram504Biases::from_probes(&model, &probes, memory, true);

        assert_close(biases.vb2c1, 0.73);
        assert_close(biases.vb2c2, -4.75);
        assert_close(biases.vb1e1, 0.7);
        assert_close(biases.vb1b2, -2.5);
        assert_close(biases.vb2e1, 0.9);
        assert!(biases.limiter_applied);
    }

    #[test]
    fn biases_select_xyce_distributed_collector_voltage_path() {
        let probes = probe_fixture(1.0);
        let model_params = std::collections::HashMap::from([
            ("RCBLX".to_string(), 3.0),
            ("RCBLI".to_string(), 4.0),
        ]);
        let full = Mextram504Model::from_params(
            &model_params,
            &std::collections::HashMap::new(),
            super::super::Mextram504Polarity::Npn,
        );
        let full_biases = Mextram504Biases::from_probes(
            &full,
            &probes,
            Mextram504LimiterMemory::default(),
            false,
        );
        assert_close(full_biases.vc4c1, 6.0);
        assert_close(full_biases.vc3c4, -1.0);

        let extrinsic_only = Mextram504Model::from_params(
            &std::collections::HashMap::from([("RCBLX".to_string(), 3.0)]),
            &std::collections::HashMap::new(),
            super::super::Mextram504Polarity::Npn,
        );
        let extrinsic_biases = Mextram504Biases::from_probes(
            &extrinsic_only,
            &probes,
            Mextram504LimiterMemory::default(),
            false,
        );
        assert_close(extrinsic_biases.vc4c1, 0.0);
        assert_close(extrinsic_biases.vc3c4, 5.0);

        let intrinsic_only = Mextram504Model::from_params(
            &std::collections::HashMap::from([("RCBLI".to_string(), 4.0)]),
            &std::collections::HashMap::new(),
            super::super::Mextram504Polarity::Npn,
        );
        let intrinsic_biases = Mextram504Biases::from_probes(
            &intrinsic_only,
            &probes,
            Mextram504LimiterMemory::default(),
            false,
        );
        assert_close(intrinsic_biases.vc4c1, 6.0);
        assert_close(intrinsic_biases.vc3c4, 0.0);
    }
}

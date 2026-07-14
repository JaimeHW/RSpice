#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq28_e504, eq28_e504_d_n0, eq28_e504_d_n1, eq28_e504_d_n2, eq28_e504_d_n3, eq28_e504_d_n4, eq28_e504_d_n5, eq28_e504_d_n6, eq28_e504_d_n7, eq28_e504_d_n8, eq28_e504_d_n9, eq28_e504_d_n10, eq28_e504_d_n11, eq28_e504_d_n12, eq28_e504_d_b0, eq28_e504_d_b1, eq28_e504_d_b2, eq28_e504_d_b3, eq28_e504_d_b4, eq28_e504_d_b5, eq28_e504_d_b6, eq28_e504_d_b7, eq28_e504_q, eq28_e504_q_d_n0, eq28_e504_q_d_n1, eq28_e504_q_d_n2, eq28_e504_q_d_n3, eq28_e504_q_d_n4, eq28_e504_q_d_n5, eq28_e504_q_d_n6, eq28_e504_q_d_n7, eq28_e504_q_d_n8, eq28_e504_q_d_n9, eq28_e504_q_d_n10, eq28_e504_q_d_n11, eq28_e504_q_d_n12, eq28_e504_q_d_b0, eq28_e504_q_d_b1, eq28_e504_q_d_b2, eq28_e504_q_d_b3, eq28_e504_q_d_b4, eq28_e504_q_d_b5, eq28_e504_q_d_b6, eq28_e504_q_d_b7,) = {
    if s.b[1094] {
        let eq28_e493: f64 = (-s.v[547]);let eq28_e496: f64 = (s.v[516] * (nv4 - 0.0));let eq28_e496_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));let eq28_e496_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));let eq28_e496_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));let eq28_e496_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));let eq28_e496_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);let eq28_e496_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));let eq28_e496_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));let eq28_e496_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));let eq28_e496_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));let eq28_e496_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));let eq28_e496_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));let eq28_e496_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));let eq28_e496_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));let eq28_e496_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));let eq28_e496_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));let eq28_e496_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));let eq28_e496_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));let eq28_e496_d_b4: f64 = (s.db[516][4] * (nv4 - 0.0));let eq28_e496_d_b5: f64 = (s.db[516][5] * (nv4 - 0.0));let eq28_e496_d_b6: f64 = (s.db[516][6] * (nv4 - 0.0));let eq28_e496_d_b7: f64 = (s.db[516][7] * (nv4 - 0.0));let eq28_e497_q: f64 = eq28_e496;let eq28_e498: f64 = (eq28_e493 + eq28_e496);let eq28_e498_d_n0: f64 = ((-s.dn[547][0]) + eq28_e496_d_n0);let eq28_e498_d_n1: f64 = ((-s.dn[547][1]) + eq28_e496_d_n1);let eq28_e498_d_n2: f64 = ((-s.dn[547][2]) + eq28_e496_d_n2);let eq28_e498_d_n3: f64 = ((-s.dn[547][3]) + eq28_e496_d_n3);let eq28_e498_d_n4: f64 = ((-s.dn[547][4]) + eq28_e496_d_n4);let eq28_e498_d_n5: f64 = ((-s.dn[547][5]) + eq28_e496_d_n5);let eq28_e498_d_n6: f64 = ((-s.dn[547][6]) + eq28_e496_d_n6);let eq28_e498_d_n7: f64 = ((-s.dn[547][7]) + eq28_e496_d_n7);let eq28_e498_d_n8: f64 = ((-s.dn[547][8]) + eq28_e496_d_n8);let eq28_e498_d_n9: f64 = ((-s.dn[547][9]) + eq28_e496_d_n9);let eq28_e498_d_n10: f64 = ((-s.dn[547][10]) + eq28_e496_d_n10);let eq28_e498_d_n11: f64 = ((-s.dn[547][11]) + eq28_e496_d_n11);let eq28_e498_d_n12: f64 = ((-s.dn[547][12]) + eq28_e496_d_n12);let eq28_e498_d_b0: f64 = ((-s.db[547][0]) + eq28_e496_d_b0);let eq28_e498_d_b1: f64 = ((-s.db[547][1]) + eq28_e496_d_b1);let eq28_e498_d_b2: f64 = ((-s.db[547][2]) + eq28_e496_d_b2);let eq28_e498_d_b3: f64 = ((-s.db[547][3]) + eq28_e496_d_b3);let eq28_e498_d_b4: f64 = ((-s.db[547][4]) + eq28_e496_d_b4);let eq28_e498_d_b5: f64 = ((-s.db[547][5]) + eq28_e496_d_b5);let eq28_e498_d_b6: f64 = ((-s.db[547][6]) + eq28_e496_d_b6);let eq28_e498_d_b7: f64 = ((-s.db[547][7]) + eq28_e496_d_b7);let eq28_e498_q: f64 = eq28_e497_q;let eq28_e501: f64 = ((nv4 - 0.0) * s.v[557]);let eq28_e501_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);let eq28_e501_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);let eq28_e501_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);let eq28_e501_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);let eq28_e501_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));let eq28_e501_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);let eq28_e501_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);let eq28_e501_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);let eq28_e501_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);let eq28_e501_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);let eq28_e501_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);let eq28_e501_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);let eq28_e501_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);let eq28_e501_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);let eq28_e501_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);let eq28_e501_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);let eq28_e501_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);let eq28_e501_d_b4: f64 = ((nv4 - 0.0) * s.db[557][4]);let eq28_e501_d_b5: f64 = ((nv4 - 0.0) * s.db[557][5]);let eq28_e501_d_b6: f64 = ((nv4 - 0.0) * s.db[557][6]);let eq28_e501_d_b7: f64 = ((nv4 - 0.0) * s.db[557][7]);let eq28_e502: f64 = (eq28_e498 + eq28_e501);let eq28_e502_d_n0: f64 = (eq28_e498_d_n0 + eq28_e501_d_n0);let eq28_e502_d_n1: f64 = (eq28_e498_d_n1 + eq28_e501_d_n1);
        let eq28_e502_d_n2: f64 = (eq28_e498_d_n2 + eq28_e501_d_n2);let eq28_e502_d_n3: f64 = (eq28_e498_d_n3 + eq28_e501_d_n3);let eq28_e502_d_n4: f64 = (eq28_e498_d_n4 + eq28_e501_d_n4);let eq28_e502_d_n5: f64 = (eq28_e498_d_n5 + eq28_e501_d_n5);let eq28_e502_d_n6: f64 = (eq28_e498_d_n6 + eq28_e501_d_n6);let eq28_e502_d_n7: f64 = (eq28_e498_d_n7 + eq28_e501_d_n7);let eq28_e502_d_n8: f64 = (eq28_e498_d_n8 + eq28_e501_d_n8);let eq28_e502_d_n9: f64 = (eq28_e498_d_n9 + eq28_e501_d_n9);let eq28_e502_d_n10: f64 = (eq28_e498_d_n10 + eq28_e501_d_n10);let eq28_e502_d_n11: f64 = (eq28_e498_d_n11 + eq28_e501_d_n11);let eq28_e502_d_n12: f64 = (eq28_e498_d_n12 + eq28_e501_d_n12);let eq28_e502_d_b0: f64 = (eq28_e498_d_b0 + eq28_e501_d_b0);let eq28_e502_d_b1: f64 = (eq28_e498_d_b1 + eq28_e501_d_b1);let eq28_e502_d_b2: f64 = (eq28_e498_d_b2 + eq28_e501_d_b2);let eq28_e502_d_b3: f64 = (eq28_e498_d_b3 + eq28_e501_d_b3);let eq28_e502_d_b4: f64 = (eq28_e498_d_b4 + eq28_e501_d_b4);let eq28_e502_d_b5: f64 = (eq28_e498_d_b5 + eq28_e501_d_b5);let eq28_e502_d_b6: f64 = (eq28_e498_d_b6 + eq28_e501_d_b6);let eq28_e502_d_b7: f64 = (eq28_e498_d_b7 + eq28_e501_d_b7);let eq28_e502_q: f64 = eq28_e498_q;
        (eq28_e502, eq28_e502_d_n0, eq28_e502_d_n1, eq28_e502_d_n2, eq28_e502_d_n3, eq28_e502_d_n4, eq28_e502_d_n5, eq28_e502_d_n6, eq28_e502_d_n7, eq28_e502_d_n8, eq28_e502_d_n9, eq28_e502_d_n10, eq28_e502_d_n11, eq28_e502_d_n12, eq28_e502_d_b0, eq28_e502_d_b1, eq28_e502_d_b2, eq28_e502_d_b3, eq28_e502_d_b4, eq28_e502_d_b5, eq28_e502_d_b6, eq28_e502_d_b7, eq28_e502_q, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3, eq28_e496_d_b4, eq28_e496_d_b5, eq28_e496_d_b6, eq28_e496_d_b7,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e504_q_d_n0, eq28_e504_q_d_n1, eq28_e504_q_d_n2, eq28_e504_q_d_n3, eq28_e504_q_d_n4, eq28_e504_q_d_n5, eq28_e504_q_d_n6, eq28_e504_q_d_n7, eq28_e504_q_d_n8, eq28_e504_q_d_n9, eq28_e504_q_d_n10, eq28_e504_q_d_n11, eq28_e504_q_d_n12];let eq28_reactive_branch_derivatives: [f64; 8] = [eq28_e504_q_d_b0, eq28_e504_q_d_b1, eq28_e504_q_d_b2, eq28_e504_q_d_b3, eq28_e504_q_d_b4, eq28_e504_q_d_b5, eq28_e504_q_d_b6, eq28_e504_q_d_b7];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &eq28_reactive_node_derivatives,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);
        let (eq30_e518, eq30_e518_d_n0, eq30_e518_d_n1, eq30_e518_d_n2, eq30_e518_d_n3, eq30_e518_d_n4, eq30_e518_d_n5, eq30_e518_d_n6, eq30_e518_d_n7, eq30_e518_d_n8, eq30_e518_d_n9, eq30_e518_d_n10, eq30_e518_d_n11, eq30_e518_d_n12, eq30_e518_d_b0, eq30_e518_d_b1, eq30_e518_d_b2, eq30_e518_d_b3, eq30_e518_d_b4, eq30_e518_d_b5, eq30_e518_d_b6, eq30_e518_d_b7, eq30_e518_q, eq30_e518_q_d_n10,) = {
    if s.b[1095] {
        let eq30_e514: f64 = (1e-9 * (nv10 - 0.0));let eq30_e515_q: f64 = eq30_e514;let eq30_e516: f64 = (s.v[558] + eq30_e514);let eq30_e516_d_n10: f64 = (s.dn[558][10] + 1e-9);let eq30_e516_q: f64 = eq30_e515_q;
        (eq30_e516, s.dn[558][0], s.dn[558][1], s.dn[558][2], s.dn[558][3], s.dn[558][4], s.dn[558][5], s.dn[558][6], s.dn[558][7], s.dn[558][8], s.dn[558][9], eq30_e516_d_n10, s.dn[558][11], s.dn[558][12], s.db[558][0], s.db[558][1], s.db[558][2], s.db[558][3], s.db[558][4], s.db[558][5], s.db[558][6], s.db[558][7], eq30_e516_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(10),
            None,
            10,
            multiplicity * (eq30_e518_q_d_n10),
        );
        let (eq32_e532, eq32_e532_d_n0, eq32_e532_d_n1, eq32_e532_d_n2, eq32_e532_d_n3, eq32_e532_d_n4, eq32_e532_d_n5, eq32_e532_d_n6, eq32_e532_d_n7, eq32_e532_d_n8, eq32_e532_d_n9, eq32_e532_d_n10, eq32_e532_d_n11, eq32_e532_d_n12, eq32_e532_d_b0, eq32_e532_d_b1, eq32_e532_d_b2, eq32_e532_d_b3, eq32_e532_d_b4, eq32_e532_d_b5, eq32_e532_d_b6, eq32_e532_d_b7, eq32_e532_q, eq32_e532_q_d_n8,) = {
    if (p.p24 != 0.0) {
        let eq32_e528: f64 = (1e-9 * (nv8 - 0.0));let eq32_e529_q: f64 = eq32_e528;let eq32_e530: f64 = (s.v[549] + eq32_e528);let eq32_e530_d_n8: f64 = (s.dn[549][8] + 1e-9);let eq32_e530_q: f64 = eq32_e529_q;
        (eq32_e530, s.dn[549][0], s.dn[549][1], s.dn[549][2], s.dn[549][3], s.dn[549][4], s.dn[549][5], s.dn[549][6], s.dn[549][7], eq32_e530_d_n8, s.dn[549][9], s.dn[549][10], s.dn[549][11], s.dn[549][12], s.db[549][0], s.db[549][1], s.db[549][2], s.db[549][3], s.db[549][4], s.db[549][5], s.db[549][6], s.db[549][7], eq32_e530_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(8),
            None,
            8,
            multiplicity * (eq32_e532_q_d_n8),
        );
        let (eq33_e541, eq33_e541_d_n0, eq33_e541_d_n1, eq33_e541_d_n2, eq33_e541_d_n3, eq33_e541_d_n4, eq33_e541_d_n5, eq33_e541_d_n6, eq33_e541_d_n7, eq33_e541_d_n8, eq33_e541_d_n9, eq33_e541_d_n10, eq33_e541_d_n11, eq33_e541_d_n12, eq33_e541_d_b0, eq33_e541_d_b1, eq33_e541_d_b2, eq33_e541_d_b3, eq33_e541_d_b4, eq33_e541_d_b5, eq33_e541_d_b6, eq33_e541_d_b7, eq33_e541_q, eq33_e541_q_d_n9,) = {
    if (p.p24 != 0.0) {
        let eq33_e537: f64 = (1e-9 * (nv9 - 0.0));let eq33_e538_q: f64 = eq33_e537;let eq33_e539: f64 = (s.v[550] + eq33_e537);let eq33_e539_d_n9: f64 = (s.dn[550][9] + 1e-9);let eq33_e539_q: f64 = eq33_e538_q;
        (eq33_e539, s.dn[550][0], s.dn[550][1], s.dn[550][2], s.dn[550][3], s.dn[550][4], s.dn[550][5], s.dn[550][6], s.dn[550][7], s.dn[550][8], eq33_e539_d_n9, s.dn[550][10], s.dn[550][11], s.dn[550][12], s.db[550][0], s.db[550][1], s.db[550][2], s.db[550][3], s.db[550][4], s.db[550][5], s.db[550][6], s.db[550][7], eq33_e539_q, 1e-9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(9),
            None,
            9,
            multiplicity * (eq33_e541_q_d_n9),
        );
    }
}

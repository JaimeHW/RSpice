#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);let nv11 = ctx.node_voltage(nodes[11]);let eq58_e1352: f64 = (s.v[767] * (nv4 - 0.0));let eq58_e1352_d_n0: f64 = (s.dn[767][0] * (nv4 - 0.0));let eq58_e1352_d_n1: f64 = (s.dn[767][1] * (nv4 - 0.0));let eq58_e1352_d_n2: f64 = (s.dn[767][2] * (nv4 - 0.0));let eq58_e1352_d_n3: f64 = (s.dn[767][3] * (nv4 - 0.0));let eq58_e1352_d_n4: f64 = ((s.dn[767][4] * (nv4 - 0.0)) + s.v[767]);let eq58_e1352_d_n5: f64 = (s.dn[767][5] * (nv4 - 0.0));let eq58_e1352_d_n6: f64 = (s.dn[767][6] * (nv4 - 0.0));let eq58_e1352_d_n7: f64 = (s.dn[767][7] * (nv4 - 0.0));let eq58_e1352_d_n8: f64 = (s.dn[767][8] * (nv4 - 0.0));let eq58_e1352_d_n9: f64 = (s.dn[767][9] * (nv4 - 0.0));let eq58_e1352_d_n10: f64 = (s.dn[767][10] * (nv4 - 0.0));let eq58_e1352_d_n11: f64 = (s.dn[767][11] * (nv4 - 0.0));let eq58_e1352_d_n12: f64 = (s.dn[767][12] * (nv4 - 0.0));let eq58_e1352_d_n13: f64 = (s.dn[767][13] * (nv4 - 0.0));let eq58_e1352_d_n14: f64 = (s.dn[767][14] * (nv4 - 0.0));let eq58_e1352_d_n15: f64 = (s.dn[767][15] * (nv4 - 0.0));let eq58_e1352_d_n16: f64 = (s.dn[767][16] * (nv4 - 0.0));let eq58_e1352_d_n17: f64 = (s.dn[767][17] * (nv4 - 0.0));let eq58_e1352_d_b0: f64 = (s.db[767][0] * (nv4 - 0.0));let eq58_e1352_d_b1: f64 = (s.db[767][1] * (nv4 - 0.0));let eq58_e1352_d_b2: f64 = (s.db[767][2] * (nv4 - 0.0));let eq58_e1352_d_b3: f64 = (s.db[767][3] * (nv4 - 0.0));let eq58_e1352_d_b4: f64 = (s.db[767][4] * (nv4 - 0.0));let eq58_e1352_d_b5: f64 = (s.db[767][5] * (nv4 - 0.0));let eq58_e1352_d_b6: f64 = (s.db[767][6] * (nv4 - 0.0));let eq58_e1352_d_b7: f64 = (s.db[767][7] * (nv4 - 0.0));let eq58_e1352_d_b8: f64 = (s.db[767][8] * (nv4 - 0.0));let eq58_e1352_d_b9: f64 = (s.db[767][9] * (nv4 - 0.0));let eq58_e1352_d_b10: f64 = (s.db[767][10] * (nv4 - 0.0));let eq58_e1352_d_b11: f64 = (s.db[767][11] * (nv4 - 0.0));let eq58_e1353_q: f64 = eq58_e1352;let eq58_reactive_node_derivatives: [f64; 18] = [eq58_e1352_d_n0, eq58_e1352_d_n1, eq58_e1352_d_n2, eq58_e1352_d_n3, eq58_e1352_d_n4, eq58_e1352_d_n5, eq58_e1352_d_n6, eq58_e1352_d_n7, eq58_e1352_d_n8, eq58_e1352_d_n9, eq58_e1352_d_n10, eq58_e1352_d_n11, eq58_e1352_d_n12, eq58_e1352_d_n13, eq58_e1352_d_n14, eq58_e1352_d_n15, eq58_e1352_d_n16, eq58_e1352_d_n17];let eq58_reactive_branch_derivatives: [f64; 12] = [eq58_e1352_d_b0, eq58_e1352_d_b1, eq58_e1352_d_b2, eq58_e1352_d_b3, eq58_e1352_d_b4, eq58_e1352_d_b5, eq58_e1352_d_b6, eq58_e1352_d_b7, eq58_e1352_d_b8, eq58_e1352_d_b9, eq58_e1352_d_b10, eq58_e1352_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(4),
            None,
            &eq58_reactive_node_derivatives,
            &eq58_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1368, eq61_e1368_d_n0, eq61_e1368_d_n1, eq61_e1368_d_n2, eq61_e1368_d_n3, eq61_e1368_d_n4, eq61_e1368_d_n5, eq61_e1368_d_n6, eq61_e1368_d_n7, eq61_e1368_d_n8, eq61_e1368_d_n9, eq61_e1368_d_n10, eq61_e1368_d_n11, eq61_e1368_d_n12, eq61_e1368_d_n13, eq61_e1368_d_n14, eq61_e1368_d_n15, eq61_e1368_d_n16, eq61_e1368_d_n17, eq61_e1368_d_b0, eq61_e1368_d_b1, eq61_e1368_d_b2, eq61_e1368_d_b3, eq61_e1368_d_b4, eq61_e1368_d_b5, eq61_e1368_d_b6, eq61_e1368_d_b7, eq61_e1368_d_b8, eq61_e1368_d_b9, eq61_e1368_d_b10, eq61_e1368_d_b11, eq61_e1368_q,) = {
    if (p[28] != 0.0) {
        let eq61_e1365: f64 = (s.v[800] * (nv11 - 0.0));let eq61_e1365_d_n0: f64 = (s.dn[800][0] * (nv11 - 0.0));let eq61_e1365_d_n1: f64 = (s.dn[800][1] * (nv11 - 0.0));let eq61_e1365_d_n2: f64 = (s.dn[800][2] * (nv11 - 0.0));let eq61_e1365_d_n3: f64 = (s.dn[800][3] * (nv11 - 0.0));let eq61_e1365_d_n4: f64 = (s.dn[800][4] * (nv11 - 0.0));let eq61_e1365_d_n5: f64 = (s.dn[800][5] * (nv11 - 0.0));let eq61_e1365_d_n6: f64 = (s.dn[800][6] * (nv11 - 0.0));let eq61_e1365_d_n7: f64 = (s.dn[800][7] * (nv11 - 0.0));let eq61_e1365_d_n8: f64 = (s.dn[800][8] * (nv11 - 0.0));let eq61_e1365_d_n9: f64 = (s.dn[800][9] * (nv11 - 0.0));let eq61_e1365_d_n10: f64 = (s.dn[800][10] * (nv11 - 0.0));let eq61_e1365_d_n11: f64 = ((s.dn[800][11] * (nv11 - 0.0)) + s.v[800]);let eq61_e1365_d_n12: f64 = (s.dn[800][12] * (nv11 - 0.0));let eq61_e1365_d_n13: f64 = (s.dn[800][13] * (nv11 - 0.0));let eq61_e1365_d_n14: f64 = (s.dn[800][14] * (nv11 - 0.0));let eq61_e1365_d_n15: f64 = (s.dn[800][15] * (nv11 - 0.0));let eq61_e1365_d_n16: f64 = (s.dn[800][16] * (nv11 - 0.0));let eq61_e1365_d_n17: f64 = (s.dn[800][17] * (nv11 - 0.0));let eq61_e1365_d_b0: f64 = (s.db[800][0] * (nv11 - 0.0));let eq61_e1365_d_b1: f64 = (s.db[800][1] * (nv11 - 0.0));let eq61_e1365_d_b2: f64 = (s.db[800][2] * (nv11 - 0.0));let eq61_e1365_d_b3: f64 = (s.db[800][3] * (nv11 - 0.0));let eq61_e1365_d_b4: f64 = (s.db[800][4] * (nv11 - 0.0));let eq61_e1365_d_b5: f64 = (s.db[800][5] * (nv11 - 0.0));let eq61_e1365_d_b6: f64 = (s.db[800][6] * (nv11 - 0.0));let eq61_e1365_d_b7: f64 = (s.db[800][7] * (nv11 - 0.0));let eq61_e1365_d_b8: f64 = (s.db[800][8] * (nv11 - 0.0));let eq61_e1365_d_b9: f64 = (s.db[800][9] * (nv11 - 0.0));let eq61_e1365_d_b10: f64 = (s.db[800][10] * (nv11 - 0.0));let eq61_e1365_d_b11: f64 = (s.db[800][11] * (nv11 - 0.0));let eq61_e1366_q: f64 = eq61_e1365;
        (eq61_e1365, eq61_e1365_d_n0, eq61_e1365_d_n1, eq61_e1365_d_n2, eq61_e1365_d_n3, eq61_e1365_d_n4, eq61_e1365_d_n5, eq61_e1365_d_n6, eq61_e1365_d_n7, eq61_e1365_d_n8, eq61_e1365_d_n9, eq61_e1365_d_n10, eq61_e1365_d_n11, eq61_e1365_d_n12, eq61_e1365_d_n13, eq61_e1365_d_n14, eq61_e1365_d_n15, eq61_e1365_d_n16, eq61_e1365_d_n17, eq61_e1365_d_b0, eq61_e1365_d_b1, eq61_e1365_d_b2, eq61_e1365_d_b3, eq61_e1365_d_b4, eq61_e1365_d_b5, eq61_e1365_d_b6, eq61_e1365_d_b7, eq61_e1365_d_b8, eq61_e1365_d_b9, eq61_e1365_d_b10, eq61_e1365_d_b11, eq61_e1366_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_reactive_node_derivatives: [f64; 18] = [eq61_e1368_d_n0, eq61_e1368_d_n1, eq61_e1368_d_n2, eq61_e1368_d_n3, eq61_e1368_d_n4, eq61_e1368_d_n5, eq61_e1368_d_n6, eq61_e1368_d_n7, eq61_e1368_d_n8, eq61_e1368_d_n9, eq61_e1368_d_n10, eq61_e1368_d_n11, eq61_e1368_d_n12, eq61_e1368_d_n13, eq61_e1368_d_n14, eq61_e1368_d_n15, eq61_e1368_d_n16, eq61_e1368_d_n17];let eq61_reactive_branch_derivatives: [f64; 12] = [eq61_e1368_d_b0, eq61_e1368_d_b1, eq61_e1368_d_b2, eq61_e1368_d_b3, eq61_e1368_d_b4, eq61_e1368_d_b5, eq61_e1368_d_b6, eq61_e1368_d_b7, eq61_e1368_d_b8, eq61_e1368_d_b9, eq61_e1368_d_b10, eq61_e1368_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(11),
            None,
            &eq61_reactive_node_derivatives,
            &eq61_reactive_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);
        let (eq62_e1375, eq62_e1375_d_n0, eq62_e1375_d_n1, eq62_e1375_d_n2, eq62_e1375_d_n3, eq62_e1375_d_n4, eq62_e1375_d_n5, eq62_e1375_d_n6, eq62_e1375_d_n7, eq62_e1375_d_n8, eq62_e1375_d_n9, eq62_e1375_d_n10, eq62_e1375_d_n11, eq62_e1375_d_n12, eq62_e1375_d_n13, eq62_e1375_d_n14, eq62_e1375_d_n15, eq62_e1375_d_n16, eq62_e1375_d_n17, eq62_e1375_d_b0, eq62_e1375_d_b1, eq62_e1375_d_b2, eq62_e1375_d_b3, eq62_e1375_d_b4, eq62_e1375_d_b5, eq62_e1375_d_b6, eq62_e1375_d_b7, eq62_e1375_d_b8, eq62_e1375_d_b9, eq62_e1375_d_b10, eq62_e1375_d_b11, eq62_e1375_q,) = {
    if (p[28] != 0.0) {
        let eq62_e1372: f64 = (s.v[801] * (nv12 - 0.0));let eq62_e1372_d_n0: f64 = (s.dn[801][0] * (nv12 - 0.0));let eq62_e1372_d_n1: f64 = (s.dn[801][1] * (nv12 - 0.0));let eq62_e1372_d_n2: f64 = (s.dn[801][2] * (nv12 - 0.0));let eq62_e1372_d_n3: f64 = (s.dn[801][3] * (nv12 - 0.0));let eq62_e1372_d_n4: f64 = (s.dn[801][4] * (nv12 - 0.0));let eq62_e1372_d_n5: f64 = (s.dn[801][5] * (nv12 - 0.0));let eq62_e1372_d_n6: f64 = (s.dn[801][6] * (nv12 - 0.0));let eq62_e1372_d_n7: f64 = (s.dn[801][7] * (nv12 - 0.0));let eq62_e1372_d_n8: f64 = (s.dn[801][8] * (nv12 - 0.0));let eq62_e1372_d_n9: f64 = (s.dn[801][9] * (nv12 - 0.0));let eq62_e1372_d_n10: f64 = (s.dn[801][10] * (nv12 - 0.0));let eq62_e1372_d_n11: f64 = (s.dn[801][11] * (nv12 - 0.0));let eq62_e1372_d_n12: f64 = ((s.dn[801][12] * (nv12 - 0.0)) + s.v[801]);let eq62_e1372_d_n13: f64 = (s.dn[801][13] * (nv12 - 0.0));let eq62_e1372_d_n14: f64 = (s.dn[801][14] * (nv12 - 0.0));let eq62_e1372_d_n15: f64 = (s.dn[801][15] * (nv12 - 0.0));let eq62_e1372_d_n16: f64 = (s.dn[801][16] * (nv12 - 0.0));let eq62_e1372_d_n17: f64 = (s.dn[801][17] * (nv12 - 0.0));let eq62_e1372_d_b0: f64 = (s.db[801][0] * (nv12 - 0.0));let eq62_e1372_d_b1: f64 = (s.db[801][1] * (nv12 - 0.0));let eq62_e1372_d_b2: f64 = (s.db[801][2] * (nv12 - 0.0));let eq62_e1372_d_b3: f64 = (s.db[801][3] * (nv12 - 0.0));let eq62_e1372_d_b4: f64 = (s.db[801][4] * (nv12 - 0.0));let eq62_e1372_d_b5: f64 = (s.db[801][5] * (nv12 - 0.0));let eq62_e1372_d_b6: f64 = (s.db[801][6] * (nv12 - 0.0));let eq62_e1372_d_b7: f64 = (s.db[801][7] * (nv12 - 0.0));let eq62_e1372_d_b8: f64 = (s.db[801][8] * (nv12 - 0.0));let eq62_e1372_d_b9: f64 = (s.db[801][9] * (nv12 - 0.0));let eq62_e1372_d_b10: f64 = (s.db[801][10] * (nv12 - 0.0));let eq62_e1372_d_b11: f64 = (s.db[801][11] * (nv12 - 0.0));let eq62_e1373_q: f64 = eq62_e1372;
        (eq62_e1372, eq62_e1372_d_n0, eq62_e1372_d_n1, eq62_e1372_d_n2, eq62_e1372_d_n3, eq62_e1372_d_n4, eq62_e1372_d_n5, eq62_e1372_d_n6, eq62_e1372_d_n7, eq62_e1372_d_n8, eq62_e1372_d_n9, eq62_e1372_d_n10, eq62_e1372_d_n11, eq62_e1372_d_n12, eq62_e1372_d_n13, eq62_e1372_d_n14, eq62_e1372_d_n15, eq62_e1372_d_n16, eq62_e1372_d_n17, eq62_e1372_d_b0, eq62_e1372_d_b1, eq62_e1372_d_b2, eq62_e1372_d_b3, eq62_e1372_d_b4, eq62_e1372_d_b5, eq62_e1372_d_b6, eq62_e1372_d_b7, eq62_e1372_d_b8, eq62_e1372_d_b9, eq62_e1372_d_b10, eq62_e1372_d_b11, eq62_e1373_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_reactive_node_derivatives: [f64; 18] = [eq62_e1375_d_n0, eq62_e1375_d_n1, eq62_e1375_d_n2, eq62_e1375_d_n3, eq62_e1375_d_n4, eq62_e1375_d_n5, eq62_e1375_d_n6, eq62_e1375_d_n7, eq62_e1375_d_n8, eq62_e1375_d_n9, eq62_e1375_d_n10, eq62_e1375_d_n11, eq62_e1375_d_n12, eq62_e1375_d_n13, eq62_e1375_d_n14, eq62_e1375_d_n15, eq62_e1375_d_n16, eq62_e1375_d_n17];let eq62_reactive_branch_derivatives: [f64; 12] = [eq62_e1375_d_b0, eq62_e1375_d_b1, eq62_e1375_d_b2, eq62_e1375_d_b3, eq62_e1375_d_b4, eq62_e1375_d_b5, eq62_e1375_d_b6, eq62_e1375_d_b7, eq62_e1375_d_b8, eq62_e1375_d_b9, eq62_e1375_d_b10, eq62_e1375_d_b11];
        stamper.stamp_current_reactive_dense_local(
            Some(12),
            None,
            &eq62_reactive_node_derivatives,
            &eq62_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq66_e1394, eq66_e1394_d_n13, eq66_e1394_q,) = {
    if (p[29] != 0.0) {
        let eq66_e1392_q: f64 = (nv13 - 0.0);
        ((nv13 - 0.0), 1.0, eq66_e1392_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1_local(
            Some(13),
            None,
            13,
            multiplicity * (eq66_e1394_d_n13),
        );
    }
}

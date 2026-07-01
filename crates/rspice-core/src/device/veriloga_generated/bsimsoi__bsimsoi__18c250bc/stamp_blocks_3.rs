#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq52_e1703, eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13, eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17, eq52_e1703_q,) = {
    if s.b[1553] {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1700: f64 = (eq52_e1698 * s.v[336]);
        let eq52_e1700_d_n3: f64 = (((-p.p33) * s.v[336]) + (eq52_e1698 * s.dn[336][3]));
        let eq52_e1700_d_n10: f64 = ((p.p33 * s.v[336]) + (eq52_e1698 * s.dn[336][10]));
        let eq52_e1701_q: f64 = eq52_e1700;
        (eq52_e1700, (eq52_e1698 * s.dn[336][0]), (eq52_e1698 * s.dn[336][1]), (eq52_e1698 * s.dn[336][2]), eq52_e1700_d_n3, (eq52_e1698 * s.dn[336][4]), (eq52_e1698 * s.dn[336][5]), (eq52_e1698 * s.dn[336][6]), (eq52_e1698 * s.dn[336][7]), (eq52_e1698 * s.dn[336][8]), (eq52_e1698 * s.dn[336][9]), eq52_e1700_d_n10, (eq52_e1698 * s.dn[336][11]), (eq52_e1698 * s.dn[336][12]), (eq52_e1698 * s.dn[336][13]), (eq52_e1698 * s.db[336][0]), (eq52_e1698 * s.db[336][1]), (eq52_e1698 * s.db[336][2]), (eq52_e1698 * s.db[336][3]), (eq52_e1698 * s.db[336][4]), (eq52_e1698 * s.db[336][5]), (eq52_e1698 * s.db[336][6]), (eq52_e1698 * s.db[336][7]), (eq52_e1698 * s.db[336][8]), (eq52_e1698 * s.db[336][9]), (eq52_e1698 * s.db[336][10]), (eq52_e1698 * s.db[336][11]), (eq52_e1698 * s.db[336][12]), (eq52_e1698 * s.db[336][13]), (eq52_e1698 * s.db[336][14]), (eq52_e1698 * s.db[336][15]), (eq52_e1698 * s.db[336][16]), (eq52_e1698 * s.db[336][17]), eq52_e1701_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 14] = [eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13];
        let eq52_reactive_branch_derivatives: [f64; 18] = [eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1713, eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13, eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17, eq53_e1713_q,) = {
    if (!s.b[1553]) {
        let eq53_e1709: f64 = (p.p33 * s.v[895]);
        let eq53_e1710_q: f64 = eq53_e1709;
        let eq53_e1711: f64 = (p.p37 * eq53_e1709);
        let eq53_e1711_d_n0: f64 = (p.p37 * (p.p33 * s.dn[895][0]));
        let eq53_e1711_d_n1: f64 = (p.p37 * (p.p33 * s.dn[895][1]));
        let eq53_e1711_d_n2: f64 = (p.p37 * (p.p33 * s.dn[895][2]));
        let eq53_e1711_d_n3: f64 = (p.p37 * (p.p33 * s.dn[895][3]));
        let eq53_e1711_d_n4: f64 = (p.p37 * (p.p33 * s.dn[895][4]));
        let eq53_e1711_d_n5: f64 = (p.p37 * (p.p33 * s.dn[895][5]));
        let eq53_e1711_d_n6: f64 = (p.p37 * (p.p33 * s.dn[895][6]));
        let eq53_e1711_d_n7: f64 = (p.p37 * (p.p33 * s.dn[895][7]));
        let eq53_e1711_d_n8: f64 = (p.p37 * (p.p33 * s.dn[895][8]));
        let eq53_e1711_d_n9: f64 = (p.p37 * (p.p33 * s.dn[895][9]));
        let eq53_e1711_d_n10: f64 = (p.p37 * (p.p33 * s.dn[895][10]));
        let eq53_e1711_d_n11: f64 = (p.p37 * (p.p33 * s.dn[895][11]));
        let eq53_e1711_d_n12: f64 = (p.p37 * (p.p33 * s.dn[895][12]));
        let eq53_e1711_d_n13: f64 = (p.p37 * (p.p33 * s.dn[895][13]));
        let eq53_e1711_d_b0: f64 = (p.p37 * (p.p33 * s.db[895][0]));
        let eq53_e1711_d_b1: f64 = (p.p37 * (p.p33 * s.db[895][1]));
        let eq53_e1711_d_b2: f64 = (p.p37 * (p.p33 * s.db[895][2]));
        let eq53_e1711_d_b3: f64 = (p.p37 * (p.p33 * s.db[895][3]));
        let eq53_e1711_d_b4: f64 = (p.p37 * (p.p33 * s.db[895][4]));
        let eq53_e1711_d_b5: f64 = (p.p37 * (p.p33 * s.db[895][5]));
        let eq53_e1711_d_b6: f64 = (p.p37 * (p.p33 * s.db[895][6]));
        let eq53_e1711_d_b7: f64 = (p.p37 * (p.p33 * s.db[895][7]));
        let eq53_e1711_d_b8: f64 = (p.p37 * (p.p33 * s.db[895][8]));
        let eq53_e1711_d_b9: f64 = (p.p37 * (p.p33 * s.db[895][9]));
        let eq53_e1711_d_b10: f64 = (p.p37 * (p.p33 * s.db[895][10]));
        let eq53_e1711_d_b11: f64 = (p.p37 * (p.p33 * s.db[895][11]));
        let eq53_e1711_d_b12: f64 = (p.p37 * (p.p33 * s.db[895][12]));
        let eq53_e1711_d_b13: f64 = (p.p37 * (p.p33 * s.db[895][13]));
        let eq53_e1711_d_b14: f64 = (p.p37 * (p.p33 * s.db[895][14]));
        let eq53_e1711_d_b15: f64 = (p.p37 * (p.p33 * s.db[895][15]));
        let eq53_e1711_d_b16: f64 = (p.p37 * (p.p33 * s.db[895][16]));
        let eq53_e1711_d_b17: f64 = (p.p37 * (p.p33 * s.db[895][17]));
        let eq53_e1711_q: f64 = (p.p37 * eq53_e1710_q);
        (eq53_e1711, eq53_e1711_d_n0, eq53_e1711_d_n1, eq53_e1711_d_n2, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_d_n13, eq53_e1711_d_b0, eq53_e1711_d_b1, eq53_e1711_d_b2, eq53_e1711_d_b3, eq53_e1711_d_b4, eq53_e1711_d_b5, eq53_e1711_d_b6, eq53_e1711_d_b7, eq53_e1711_d_b8, eq53_e1711_d_b9, eq53_e1711_d_b10, eq53_e1711_d_b11, eq53_e1711_d_b12, eq53_e1711_d_b13, eq53_e1711_d_b14, eq53_e1711_d_b15, eq53_e1711_d_b16, eq53_e1711_d_b17, eq53_e1711_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 14] = [eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13];
        let eq53_reactive_branch_derivatives: [f64; 18] = [eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e1723, eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13, eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17, eq54_e1723_q,) = {
    if (!s.b[1553]) {
        let eq54_e1719: f64 = (p.p33 * s.v[896]);
        let eq54_e1720_q: f64 = eq54_e1719;
        let eq54_e1721: f64 = (p.p37 * eq54_e1719);
        let eq54_e1721_d_n0: f64 = (p.p37 * (p.p33 * s.dn[896][0]));
        let eq54_e1721_d_n1: f64 = (p.p37 * (p.p33 * s.dn[896][1]));
        let eq54_e1721_d_n2: f64 = (p.p37 * (p.p33 * s.dn[896][2]));
        let eq54_e1721_d_n3: f64 = (p.p37 * (p.p33 * s.dn[896][3]));
        let eq54_e1721_d_n4: f64 = (p.p37 * (p.p33 * s.dn[896][4]));
        let eq54_e1721_d_n5: f64 = (p.p37 * (p.p33 * s.dn[896][5]));
        let eq54_e1721_d_n6: f64 = (p.p37 * (p.p33 * s.dn[896][6]));
        let eq54_e1721_d_n7: f64 = (p.p37 * (p.p33 * s.dn[896][7]));
        let eq54_e1721_d_n8: f64 = (p.p37 * (p.p33 * s.dn[896][8]));
        let eq54_e1721_d_n9: f64 = (p.p37 * (p.p33 * s.dn[896][9]));
        let eq54_e1721_d_n10: f64 = (p.p37 * (p.p33 * s.dn[896][10]));
        let eq54_e1721_d_n11: f64 = (p.p37 * (p.p33 * s.dn[896][11]));
        let eq54_e1721_d_n12: f64 = (p.p37 * (p.p33 * s.dn[896][12]));
        let eq54_e1721_d_n13: f64 = (p.p37 * (p.p33 * s.dn[896][13]));
        let eq54_e1721_d_b0: f64 = (p.p37 * (p.p33 * s.db[896][0]));
        let eq54_e1721_d_b1: f64 = (p.p37 * (p.p33 * s.db[896][1]));
        let eq54_e1721_d_b2: f64 = (p.p37 * (p.p33 * s.db[896][2]));
        let eq54_e1721_d_b3: f64 = (p.p37 * (p.p33 * s.db[896][3]));
        let eq54_e1721_d_b4: f64 = (p.p37 * (p.p33 * s.db[896][4]));
        let eq54_e1721_d_b5: f64 = (p.p37 * (p.p33 * s.db[896][5]));
        let eq54_e1721_d_b6: f64 = (p.p37 * (p.p33 * s.db[896][6]));
        let eq54_e1721_d_b7: f64 = (p.p37 * (p.p33 * s.db[896][7]));
        let eq54_e1721_d_b8: f64 = (p.p37 * (p.p33 * s.db[896][8]));
        let eq54_e1721_d_b9: f64 = (p.p37 * (p.p33 * s.db[896][9]));
        let eq54_e1721_d_b10: f64 = (p.p37 * (p.p33 * s.db[896][10]));
        let eq54_e1721_d_b11: f64 = (p.p37 * (p.p33 * s.db[896][11]));
        let eq54_e1721_d_b12: f64 = (p.p37 * (p.p33 * s.db[896][12]));
        let eq54_e1721_d_b13: f64 = (p.p37 * (p.p33 * s.db[896][13]));
        let eq54_e1721_d_b14: f64 = (p.p37 * (p.p33 * s.db[896][14]));
        let eq54_e1721_d_b15: f64 = (p.p37 * (p.p33 * s.db[896][15]));
        let eq54_e1721_d_b16: f64 = (p.p37 * (p.p33 * s.db[896][16]));
        let eq54_e1721_d_b17: f64 = (p.p37 * (p.p33 * s.db[896][17]));
        let eq54_e1721_q: f64 = (p.p37 * eq54_e1720_q);
        (eq54_e1721, eq54_e1721_d_n0, eq54_e1721_d_n1, eq54_e1721_d_n2, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_d_n13, eq54_e1721_d_b0, eq54_e1721_d_b1, eq54_e1721_d_b2, eq54_e1721_d_b3, eq54_e1721_d_b4, eq54_e1721_d_b5, eq54_e1721_d_b6, eq54_e1721_d_b7, eq54_e1721_d_b8, eq54_e1721_d_b9, eq54_e1721_d_b10, eq54_e1721_d_b11, eq54_e1721_d_b12, eq54_e1721_d_b13, eq54_e1721_d_b14, eq54_e1721_d_b15, eq54_e1721_d_b16, eq54_e1721_d_b17, eq54_e1721_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 14] = [eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13];
        let eq54_reactive_branch_derivatives: [f64; 18] = [eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1733, eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13, eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17, eq55_e1733_q,) = {
    if (!s.b[1553]) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1730: f64 = (eq55_e1728 * s.v[336]);
        let eq55_e1730_d_n3: f64 = (((-p.p33) * s.v[336]) + (eq55_e1728 * s.dn[336][3]));
        let eq55_e1730_d_n9: f64 = ((p.p33 * s.v[336]) + (eq55_e1728 * s.dn[336][9]));
        let eq55_e1731_q: f64 = eq55_e1730;
        (eq55_e1730, (eq55_e1728 * s.dn[336][0]), (eq55_e1728 * s.dn[336][1]), (eq55_e1728 * s.dn[336][2]), eq55_e1730_d_n3, (eq55_e1728 * s.dn[336][4]), (eq55_e1728 * s.dn[336][5]), (eq55_e1728 * s.dn[336][6]), (eq55_e1728 * s.dn[336][7]), (eq55_e1728 * s.dn[336][8]), eq55_e1730_d_n9, (eq55_e1728 * s.dn[336][10]), (eq55_e1728 * s.dn[336][11]), (eq55_e1728 * s.dn[336][12]), (eq55_e1728 * s.dn[336][13]), (eq55_e1728 * s.db[336][0]), (eq55_e1728 * s.db[336][1]), (eq55_e1728 * s.db[336][2]), (eq55_e1728 * s.db[336][3]), (eq55_e1728 * s.db[336][4]), (eq55_e1728 * s.db[336][5]), (eq55_e1728 * s.db[336][6]), (eq55_e1728 * s.db[336][7]), (eq55_e1728 * s.db[336][8]), (eq55_e1728 * s.db[336][9]), (eq55_e1728 * s.db[336][10]), (eq55_e1728 * s.db[336][11]), (eq55_e1728 * s.db[336][12]), (eq55_e1728 * s.db[336][13]), (eq55_e1728 * s.db[336][14]), (eq55_e1728 * s.db[336][15]), (eq55_e1728 * s.db[336][16]), (eq55_e1728 * s.db[336][17]), eq55_e1731_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 14] = [eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13];
        let eq55_reactive_branch_derivatives: [f64; 18] = [eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e1736: f64 = (p.p33 * s.v[87]);
        let eq56_e1737_q: f64 = eq56_e1736;
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &s.dn[87],
            branches,
            &s.db[87],
            (multiplicity) * (p.p33),
        );
        let eq57_e1740: f64 = (p.p33 * s.v[86]);
        let eq57_e1741_q: f64 = eq57_e1740;
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &s.dn[86],
            branches,
            &s.db[86],
            (multiplicity) * (p.p33),
        );
        let (eq71_e1869, eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13, eq71_e1869_d_b0, eq71_e1869_d_b1, eq71_e1869_d_b2, eq71_e1869_d_b3, eq71_e1869_d_b4, eq71_e1869_d_b5, eq71_e1869_d_b6, eq71_e1869_d_b7, eq71_e1869_d_b8, eq71_e1869_d_b9, eq71_e1869_d_b10, eq71_e1869_d_b11, eq71_e1869_d_b12, eq71_e1869_d_b13, eq71_e1869_d_b14, eq71_e1869_d_b15, eq71_e1869_d_b16, eq71_e1869_d_b17, eq71_e1869_q, eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13, eq71_e1869_q_d_b0, eq71_e1869_q_d_b1, eq71_e1869_q_d_b2, eq71_e1869_q_d_b3, eq71_e1869_q_d_b4, eq71_e1869_q_d_b5, eq71_e1869_q_d_b6, eq71_e1869_q_d_b7, eq71_e1869_q_d_b8, eq71_e1869_q_d_b9, eq71_e1869_q_d_b10, eq71_e1869_q_d_b11, eq71_e1869_q_d_b12, eq71_e1869_q_d_b13, eq71_e1869_q_d_b14, eq71_e1869_q_d_b15, eq71_e1869_q_d_b16, eq71_e1869_q_d_b17,) = {
    if ((s.b[1559] && s.b[1560]) && s.b[1561]) {
        let eq71_e1856: f64 = (-s.v[885]);
        let eq71_e1858: f64 = (eq71_e1856 * s.v[822]);
        let eq71_e1858_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq71_e1856 * s.dn[822][0]));
        let eq71_e1858_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq71_e1856 * s.dn[822][1]));
        let eq71_e1858_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq71_e1856 * s.dn[822][2]));
        let eq71_e1858_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq71_e1856 * s.dn[822][3]));
        let eq71_e1858_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq71_e1856 * s.dn[822][4]));
        let eq71_e1858_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq71_e1856 * s.dn[822][5]));
        let eq71_e1858_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq71_e1856 * s.dn[822][6]));
        let eq71_e1858_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq71_e1856 * s.dn[822][7]));
        let eq71_e1858_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq71_e1856 * s.dn[822][8]));
        let eq71_e1858_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq71_e1856 * s.dn[822][9]));
        let eq71_e1858_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq71_e1856 * s.dn[822][10]));
        let eq71_e1858_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq71_e1856 * s.dn[822][11]));
        let eq71_e1858_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq71_e1856 * s.dn[822][12]));
        let eq71_e1858_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq71_e1856 * s.dn[822][13]));
        let eq71_e1858_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq71_e1856 * s.db[822][0]));
        let eq71_e1858_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq71_e1856 * s.db[822][1]));
        let eq71_e1858_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq71_e1856 * s.db[822][2]));
        let eq71_e1858_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq71_e1856 * s.db[822][3]));
        let eq71_e1858_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq71_e1856 * s.db[822][4]));
        let eq71_e1858_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq71_e1856 * s.db[822][5]));
        let eq71_e1858_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq71_e1856 * s.db[822][6]));
        let eq71_e1858_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq71_e1856 * s.db[822][7]));
        let eq71_e1858_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq71_e1856 * s.db[822][8]));
        let eq71_e1858_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq71_e1856 * s.db[822][9]));
        let eq71_e1858_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq71_e1856 * s.db[822][10]));
        let eq71_e1858_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq71_e1856 * s.db[822][11]));
        let eq71_e1858_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq71_e1856 * s.db[822][12]));
        let eq71_e1858_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq71_e1856 * s.db[822][13]));
        let eq71_e1858_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq71_e1856 * s.db[822][14]));
        let eq71_e1858_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq71_e1856 * s.db[822][15]));
        let eq71_e1858_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq71_e1856 * s.db[822][16]));
        let eq71_e1858_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq71_e1856 * s.db[822][17]));
        let eq71_e1861: f64 = (s.v[410] * s.v[158]);
        let eq71_e1862_q: f64 = eq71_e1861;
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1861);
        let eq71_e1863_d_n0: f64 = (eq71_e1858_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq71_e1863_d_n1: f64 = (eq71_e1858_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq71_e1863_d_n2: f64 = (eq71_e1858_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq71_e1863_d_n13: f64 = (eq71_e1858_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq71_e1863_d_b0: f64 = (eq71_e1858_d_b0 + (s.db[410][0] * s.v[158]));
        let eq71_e1863_d_b1: f64 = (eq71_e1858_d_b1 + (s.db[410][1] * s.v[158]));
        let eq71_e1863_d_b2: f64 = (eq71_e1858_d_b2 + (s.db[410][2] * s.v[158]));
        let eq71_e1863_d_b3: f64 = (eq71_e1858_d_b3 + (s.db[410][3] * s.v[158]));
        let eq71_e1863_d_b4: f64 = (eq71_e1858_d_b4 + (s.db[410][4] * s.v[158]));
        let eq71_e1863_d_b5: f64 = (eq71_e1858_d_b5 + (s.db[410][5] * s.v[158]));
        let eq71_e1863_d_b6: f64 = (eq71_e1858_d_b6 + (s.db[410][6] * s.v[158]));
        let eq71_e1863_d_b7: f64 = (eq71_e1858_d_b7 + (s.db[410][7] * s.v[158]));
        let eq71_e1863_d_b8: f64 = (eq71_e1858_d_b8 + (s.db[410][8] * s.v[158]));
        let eq71_e1863_d_b9: f64 = (eq71_e1858_d_b9 + (s.db[410][9] * s.v[158]));
        let eq71_e1863_d_b10: f64 = (eq71_e1858_d_b10 + (s.db[410][10] * s.v[158]));
        let eq71_e1863_d_b11: f64 = (eq71_e1858_d_b11 + (s.db[410][11] * s.v[158]));
        let eq71_e1863_d_b12: f64 = (eq71_e1858_d_b12 + (s.db[410][12] * s.v[158]));
        let eq71_e1863_d_b13: f64 = (eq71_e1858_d_b13 + (s.db[410][13] * s.v[158]));
        let eq71_e1863_d_b14: f64 = (eq71_e1858_d_b14 + (s.db[410][14] * s.v[158]));
        let eq71_e1863_d_b15: f64 = (eq71_e1858_d_b15 + (s.db[410][15] * s.v[158]));
        let eq71_e1863_d_b16: f64 = (eq71_e1858_d_b16 + (s.db[410][16] * s.v[158]));
        let eq71_e1863_d_b17: f64 = (eq71_e1858_d_b17 + (s.db[410][17] * s.v[158]));
        let eq71_e1863_q: f64 = eq71_e1862_q;
        let __rspice_inv_cse_0: f64 = 1.0 / s.v[157];
        let eq71_e1866: f64 = (s.v[410] * __rspice_inv_cse_0);
        let eq71_e1866_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_0);
        let eq71_e1866_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_0);
        let eq71_e1866_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_0);
        let eq71_e1866_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_0);
        let eq71_e1866_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_0);
        let eq71_e1866_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_0);
        let eq71_e1866_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_0);
        let eq71_e1866_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_0);
        let eq71_e1866_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_0);
        let eq71_e1866_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_0);
        let eq71_e1866_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_0);
        let eq71_e1866_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_0);
        let eq71_e1866_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_0);
        let eq71_e1866_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_0);
        let eq71_e1866_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_0);
        let eq71_e1866_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_0);
        let eq71_e1866_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_0);
        let eq71_e1866_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_0);
        let eq71_e1866_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_0);
        let eq71_e1866_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_0);
        let eq71_e1866_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_0);
        let eq71_e1866_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_0);
        let eq71_e1866_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_0);
        let eq71_e1866_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_0);
        let eq71_e1866_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_0);
        let eq71_e1866_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_0);
        let eq71_e1866_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_0);
        let eq71_e1866_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_0);
        let eq71_e1866_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_0);
        let eq71_e1866_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_0);
        let eq71_e1866_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_0);
        let eq71_e1866_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_0);
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n0: f64 = (eq71_e1863_d_n0 + eq71_e1866_d_n0);
        let eq71_e1867_d_n1: f64 = (eq71_e1863_d_n1 + eq71_e1866_d_n1);
        let eq71_e1867_d_n2: f64 = (eq71_e1863_d_n2 + eq71_e1866_d_n2);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        let eq71_e1867_d_n13: f64 = (eq71_e1863_d_n13 + eq71_e1866_d_n13);
        let eq71_e1867_d_b0: f64 = (eq71_e1863_d_b0 + eq71_e1866_d_b0);
        let eq71_e1867_d_b1: f64 = (eq71_e1863_d_b1 + eq71_e1866_d_b1);
        let eq71_e1867_d_b2: f64 = (eq71_e1863_d_b2 + eq71_e1866_d_b2);
        let eq71_e1867_d_b3: f64 = (eq71_e1863_d_b3 + eq71_e1866_d_b3);
        let eq71_e1867_d_b4: f64 = (eq71_e1863_d_b4 + eq71_e1866_d_b4);
        let eq71_e1867_d_b5: f64 = (eq71_e1863_d_b5 + eq71_e1866_d_b5);
        let eq71_e1867_d_b6: f64 = (eq71_e1863_d_b6 + eq71_e1866_d_b6);
        let eq71_e1867_d_b7: f64 = (eq71_e1863_d_b7 + eq71_e1866_d_b7);
        let eq71_e1867_d_b8: f64 = (eq71_e1863_d_b8 + eq71_e1866_d_b8);
        let eq71_e1867_d_b9: f64 = (eq71_e1863_d_b9 + eq71_e1866_d_b9);
        let eq71_e1867_d_b10: f64 = (eq71_e1863_d_b10 + eq71_e1866_d_b10);
        let eq71_e1867_d_b11: f64 = (eq71_e1863_d_b11 + eq71_e1866_d_b11);
        let eq71_e1867_d_b12: f64 = (eq71_e1863_d_b12 + eq71_e1866_d_b12);
        let eq71_e1867_d_b13: f64 = (eq71_e1863_d_b13 + eq71_e1866_d_b13);
        let eq71_e1867_d_b14: f64 = (eq71_e1863_d_b14 + eq71_e1866_d_b14);
        let eq71_e1867_d_b15: f64 = (eq71_e1863_d_b15 + eq71_e1866_d_b15);
        let eq71_e1867_d_b16: f64 = (eq71_e1863_d_b16 + eq71_e1866_d_b16);
        let eq71_e1867_d_b17: f64 = (eq71_e1863_d_b17 + eq71_e1866_d_b17);
        let eq71_e1867_q: f64 = eq71_e1863_q;
        (eq71_e1867, eq71_e1867_d_n0, eq71_e1867_d_n1, eq71_e1867_d_n2, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_d_n13, eq71_e1867_d_b0, eq71_e1867_d_b1, eq71_e1867_d_b2, eq71_e1867_d_b3, eq71_e1867_d_b4, eq71_e1867_d_b5, eq71_e1867_d_b6, eq71_e1867_d_b7, eq71_e1867_d_b8, eq71_e1867_d_b9, eq71_e1867_d_b10, eq71_e1867_d_b11, eq71_e1867_d_b12, eq71_e1867_d_b13, eq71_e1867_d_b14, eq71_e1867_d_b15, eq71_e1867_d_b16, eq71_e1867_d_b17, eq71_e1867_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 14] = [eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13];
        let eq71_reactive_branch_derivatives: [f64; 18] = [eq71_e1869_q_d_b0, eq71_e1869_q_d_b1, eq71_e1869_q_d_b2, eq71_e1869_q_d_b3, eq71_e1869_q_d_b4, eq71_e1869_q_d_b5, eq71_e1869_q_d_b6, eq71_e1869_q_d_b7, eq71_e1869_q_d_b8, eq71_e1869_q_d_b9, eq71_e1869_q_d_b10, eq71_e1869_q_d_b11, eq71_e1869_q_d_b12, eq71_e1869_q_d_b13, eq71_e1869_q_d_b14, eq71_e1869_q_d_b15, eq71_e1869_q_d_b16, eq71_e1869_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1892, eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13, eq72_e1892_d_b0, eq72_e1892_d_b1, eq72_e1892_d_b2, eq72_e1892_d_b3, eq72_e1892_d_b4, eq72_e1892_d_b5, eq72_e1892_d_b6, eq72_e1892_d_b7, eq72_e1892_d_b8, eq72_e1892_d_b9, eq72_e1892_d_b10, eq72_e1892_d_b11, eq72_e1892_d_b12, eq72_e1892_d_b13, eq72_e1892_d_b14, eq72_e1892_d_b15, eq72_e1892_d_b16, eq72_e1892_d_b17, eq72_e1892_q, eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13, eq72_e1892_q_d_b0, eq72_e1892_q_d_b1, eq72_e1892_q_d_b2, eq72_e1892_q_d_b3, eq72_e1892_q_d_b4, eq72_e1892_q_d_b5, eq72_e1892_q_d_b6, eq72_e1892_q_d_b7, eq72_e1892_q_d_b8, eq72_e1892_q_d_b9, eq72_e1892_q_d_b10, eq72_e1892_q_d_b11, eq72_e1892_q_d_b12, eq72_e1892_q_d_b13, eq72_e1892_q_d_b14, eq72_e1892_q_d_b15, eq72_e1892_q_d_b16, eq72_e1892_q_d_b17,) = {
    if (((s.b[1559] && s.b[1560]) && (!s.b[1561])) && s.b[1562]) {
        let eq72_e1879: f64 = (-s.v[885]);
        let eq72_e1881: f64 = (eq72_e1879 * s.v[822]);
        let eq72_e1881_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq72_e1879 * s.dn[822][0]));
        let eq72_e1881_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq72_e1879 * s.dn[822][1]));
        let eq72_e1881_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq72_e1879 * s.dn[822][2]));
        let eq72_e1881_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq72_e1879 * s.dn[822][3]));
        let eq72_e1881_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq72_e1879 * s.dn[822][4]));
        let eq72_e1881_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq72_e1879 * s.dn[822][5]));
        let eq72_e1881_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq72_e1879 * s.dn[822][6]));
        let eq72_e1881_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq72_e1879 * s.dn[822][7]));
        let eq72_e1881_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq72_e1879 * s.dn[822][8]));
        let eq72_e1881_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq72_e1879 * s.dn[822][9]));
        let eq72_e1881_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq72_e1879 * s.dn[822][10]));
        let eq72_e1881_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq72_e1879 * s.dn[822][11]));
        let eq72_e1881_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq72_e1879 * s.dn[822][12]));
        let eq72_e1881_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq72_e1879 * s.dn[822][13]));
        let eq72_e1881_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq72_e1879 * s.db[822][0]));
        let eq72_e1881_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq72_e1879 * s.db[822][1]));
        let eq72_e1881_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq72_e1879 * s.db[822][2]));
        let eq72_e1881_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq72_e1879 * s.db[822][3]));
        let eq72_e1881_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq72_e1879 * s.db[822][4]));
        let eq72_e1881_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq72_e1879 * s.db[822][5]));
        let eq72_e1881_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq72_e1879 * s.db[822][6]));
        let eq72_e1881_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq72_e1879 * s.db[822][7]));
        let eq72_e1881_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq72_e1879 * s.db[822][8]));
        let eq72_e1881_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq72_e1879 * s.db[822][9]));
        let eq72_e1881_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq72_e1879 * s.db[822][10]));
        let eq72_e1881_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq72_e1879 * s.db[822][11]));
        let eq72_e1881_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq72_e1879 * s.db[822][12]));
        let eq72_e1881_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq72_e1879 * s.db[822][13]));
        let eq72_e1881_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq72_e1879 * s.db[822][14]));
        let eq72_e1881_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq72_e1879 * s.db[822][15]));
        let eq72_e1881_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq72_e1879 * s.db[822][16]));
        let eq72_e1881_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq72_e1879 * s.db[822][17]));
        let eq72_e1884: f64 = (s.v[410] * s.v[158]);
        let eq72_e1885_q: f64 = eq72_e1884;
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1884);
        let eq72_e1886_d_n0: f64 = (eq72_e1881_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq72_e1886_d_n1: f64 = (eq72_e1881_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq72_e1886_d_n2: f64 = (eq72_e1881_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq72_e1886_d_n13: f64 = (eq72_e1881_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq72_e1886_d_b0: f64 = (eq72_e1881_d_b0 + (s.db[410][0] * s.v[158]));
        let eq72_e1886_d_b1: f64 = (eq72_e1881_d_b1 + (s.db[410][1] * s.v[158]));
        let eq72_e1886_d_b2: f64 = (eq72_e1881_d_b2 + (s.db[410][2] * s.v[158]));
        let eq72_e1886_d_b3: f64 = (eq72_e1881_d_b3 + (s.db[410][3] * s.v[158]));
        let eq72_e1886_d_b4: f64 = (eq72_e1881_d_b4 + (s.db[410][4] * s.v[158]));
        let eq72_e1886_d_b5: f64 = (eq72_e1881_d_b5 + (s.db[410][5] * s.v[158]));
        let eq72_e1886_d_b6: f64 = (eq72_e1881_d_b6 + (s.db[410][6] * s.v[158]));
        let eq72_e1886_d_b7: f64 = (eq72_e1881_d_b7 + (s.db[410][7] * s.v[158]));
        let eq72_e1886_d_b8: f64 = (eq72_e1881_d_b8 + (s.db[410][8] * s.v[158]));
        let eq72_e1886_d_b9: f64 = (eq72_e1881_d_b9 + (s.db[410][9] * s.v[158]));
        let eq72_e1886_d_b10: f64 = (eq72_e1881_d_b10 + (s.db[410][10] * s.v[158]));
        let eq72_e1886_d_b11: f64 = (eq72_e1881_d_b11 + (s.db[410][11] * s.v[158]));
        let eq72_e1886_d_b12: f64 = (eq72_e1881_d_b12 + (s.db[410][12] * s.v[158]));
        let eq72_e1886_d_b13: f64 = (eq72_e1881_d_b13 + (s.db[410][13] * s.v[158]));
        let eq72_e1886_d_b14: f64 = (eq72_e1881_d_b14 + (s.db[410][14] * s.v[158]));
        let eq72_e1886_d_b15: f64 = (eq72_e1881_d_b15 + (s.db[410][15] * s.v[158]));
        let eq72_e1886_d_b16: f64 = (eq72_e1881_d_b16 + (s.db[410][16] * s.v[158]));
        let eq72_e1886_d_b17: f64 = (eq72_e1881_d_b17 + (s.db[410][17] * s.v[158]));
        let eq72_e1886_q: f64 = eq72_e1885_q;
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq72_e1889: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq72_e1889_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq72_e1889_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq72_e1889_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq72_e1889_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq72_e1889_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq72_e1889_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq72_e1889_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq72_e1889_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq72_e1889_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq72_e1889_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq72_e1889_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq72_e1889_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq72_e1889_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq72_e1889_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq72_e1889_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq72_e1889_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq72_e1889_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq72_e1889_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq72_e1889_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq72_e1889_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq72_e1889_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq72_e1889_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq72_e1889_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq72_e1889_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq72_e1889_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq72_e1889_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq72_e1889_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq72_e1889_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq72_e1889_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq72_e1889_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq72_e1889_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq72_e1889_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n0: f64 = (eq72_e1886_d_n0 + eq72_e1889_d_n0);
        let eq72_e1890_d_n1: f64 = (eq72_e1886_d_n1 + eq72_e1889_d_n1);
        let eq72_e1890_d_n2: f64 = (eq72_e1886_d_n2 + eq72_e1889_d_n2);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        let eq72_e1890_d_n13: f64 = (eq72_e1886_d_n13 + eq72_e1889_d_n13);
        let eq72_e1890_d_b0: f64 = (eq72_e1886_d_b0 + eq72_e1889_d_b0);
        let eq72_e1890_d_b1: f64 = (eq72_e1886_d_b1 + eq72_e1889_d_b1);
        let eq72_e1890_d_b2: f64 = (eq72_e1886_d_b2 + eq72_e1889_d_b2);
        let eq72_e1890_d_b3: f64 = (eq72_e1886_d_b3 + eq72_e1889_d_b3);
        let eq72_e1890_d_b4: f64 = (eq72_e1886_d_b4 + eq72_e1889_d_b4);
        let eq72_e1890_d_b5: f64 = (eq72_e1886_d_b5 + eq72_e1889_d_b5);
        let eq72_e1890_d_b6: f64 = (eq72_e1886_d_b6 + eq72_e1889_d_b6);
        let eq72_e1890_d_b7: f64 = (eq72_e1886_d_b7 + eq72_e1889_d_b7);
        let eq72_e1890_d_b8: f64 = (eq72_e1886_d_b8 + eq72_e1889_d_b8);
        let eq72_e1890_d_b9: f64 = (eq72_e1886_d_b9 + eq72_e1889_d_b9);
        let eq72_e1890_d_b10: f64 = (eq72_e1886_d_b10 + eq72_e1889_d_b10);
        let eq72_e1890_d_b11: f64 = (eq72_e1886_d_b11 + eq72_e1889_d_b11);
        let eq72_e1890_d_b12: f64 = (eq72_e1886_d_b12 + eq72_e1889_d_b12);
        let eq72_e1890_d_b13: f64 = (eq72_e1886_d_b13 + eq72_e1889_d_b13);
        let eq72_e1890_d_b14: f64 = (eq72_e1886_d_b14 + eq72_e1889_d_b14);
        let eq72_e1890_d_b15: f64 = (eq72_e1886_d_b15 + eq72_e1889_d_b15);
        let eq72_e1890_d_b16: f64 = (eq72_e1886_d_b16 + eq72_e1889_d_b16);
        let eq72_e1890_d_b17: f64 = (eq72_e1886_d_b17 + eq72_e1889_d_b17);
        let eq72_e1890_q: f64 = eq72_e1886_q;
        (eq72_e1890, eq72_e1890_d_n0, eq72_e1890_d_n1, eq72_e1890_d_n2, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_d_n13, eq72_e1890_d_b0, eq72_e1890_d_b1, eq72_e1890_d_b2, eq72_e1890_d_b3, eq72_e1890_d_b4, eq72_e1890_d_b5, eq72_e1890_d_b6, eq72_e1890_d_b7, eq72_e1890_d_b8, eq72_e1890_d_b9, eq72_e1890_d_b10, eq72_e1890_d_b11, eq72_e1890_d_b12, eq72_e1890_d_b13, eq72_e1890_d_b14, eq72_e1890_d_b15, eq72_e1890_d_b16, eq72_e1890_d_b17, eq72_e1890_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 14] = [eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13];
        let eq72_reactive_branch_derivatives: [f64; 18] = [eq72_e1892_q_d_b0, eq72_e1892_q_d_b1, eq72_e1892_q_d_b2, eq72_e1892_q_d_b3, eq72_e1892_q_d_b4, eq72_e1892_q_d_b5, eq72_e1892_q_d_b6, eq72_e1892_q_d_b7, eq72_e1892_q_d_b8, eq72_e1892_q_d_b9, eq72_e1892_q_d_b10, eq72_e1892_q_d_b11, eq72_e1892_q_d_b12, eq72_e1892_q_d_b13, eq72_e1892_q_d_b14, eq72_e1892_q_d_b15, eq72_e1892_q_d_b16, eq72_e1892_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq73_e1920, eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13, eq73_e1920_d_b0, eq73_e1920_d_b1, eq73_e1920_d_b2, eq73_e1920_d_b3, eq73_e1920_d_b4, eq73_e1920_d_b5, eq73_e1920_d_b6, eq73_e1920_d_b7, eq73_e1920_d_b8, eq73_e1920_d_b9, eq73_e1920_d_b10, eq73_e1920_d_b11, eq73_e1920_d_b12, eq73_e1920_d_b13, eq73_e1920_d_b14, eq73_e1920_d_b15, eq73_e1920_d_b16, eq73_e1920_d_b17, eq73_e1920_q, eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13, eq73_e1920_q_d_b0, eq73_e1920_q_d_b1, eq73_e1920_q_d_b2, eq73_e1920_q_d_b3, eq73_e1920_q_d_b4, eq73_e1920_q_d_b5, eq73_e1920_q_d_b6, eq73_e1920_q_d_b7, eq73_e1920_q_d_b8, eq73_e1920_q_d_b9, eq73_e1920_q_d_b10, eq73_e1920_q_d_b11, eq73_e1920_q_d_b12, eq73_e1920_q_d_b13, eq73_e1920_q_d_b14, eq73_e1920_q_d_b15, eq73_e1920_q_d_b16, eq73_e1920_q_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq73_e1906: f64 = (s.v[885] * __rspice_inv_cse_0);
        let eq73_e1906_d_n0: f64 = (s.dn[885][0] * __rspice_inv_cse_0);
        let eq73_e1906_d_n1: f64 = (s.dn[885][1] * __rspice_inv_cse_0);
        let eq73_e1906_d_n2: f64 = (s.dn[885][2] * __rspice_inv_cse_0);
        let eq73_e1906_d_n3: f64 = (s.dn[885][3] * __rspice_inv_cse_0);
        let eq73_e1906_d_n4: f64 = (s.dn[885][4] * __rspice_inv_cse_0);
        let eq73_e1906_d_n5: f64 = (s.dn[885][5] * __rspice_inv_cse_0);
        let eq73_e1906_d_n6: f64 = (s.dn[885][6] * __rspice_inv_cse_0);
        let eq73_e1906_d_n7: f64 = (s.dn[885][7] * __rspice_inv_cse_0);
        let eq73_e1906_d_n8: f64 = (s.dn[885][8] * __rspice_inv_cse_0);
        let eq73_e1906_d_n9: f64 = (s.dn[885][9] * __rspice_inv_cse_0);
        let eq73_e1906_d_n10: f64 = (s.dn[885][10] * __rspice_inv_cse_0);
        let eq73_e1906_d_n11: f64 = (s.dn[885][11] * __rspice_inv_cse_0);
        let eq73_e1906_d_n12: f64 = (s.dn[885][12] * __rspice_inv_cse_0);
        let eq73_e1906_d_n13: f64 = (s.dn[885][13] * __rspice_inv_cse_0);
        let eq73_e1906_d_b0: f64 = (s.db[885][0] * __rspice_inv_cse_0);
        let eq73_e1906_d_b1: f64 = (s.db[885][1] * __rspice_inv_cse_0);
        let eq73_e1906_d_b2: f64 = (s.db[885][2] * __rspice_inv_cse_0);
        let eq73_e1906_d_b3: f64 = (s.db[885][3] * __rspice_inv_cse_0);
        let eq73_e1906_d_b4: f64 = (s.db[885][4] * __rspice_inv_cse_0);
        let eq73_e1906_d_b5: f64 = (s.db[885][5] * __rspice_inv_cse_0);
        let eq73_e1906_d_b6: f64 = (s.db[885][6] * __rspice_inv_cse_0);
        let eq73_e1906_d_b7: f64 = (s.db[885][7] * __rspice_inv_cse_0);
        let eq73_e1906_d_b8: f64 = (s.db[885][8] * __rspice_inv_cse_0);
        let eq73_e1906_d_b9: f64 = (s.db[885][9] * __rspice_inv_cse_0);
        let eq73_e1906_d_b10: f64 = (s.db[885][10] * __rspice_inv_cse_0);
        let eq73_e1906_d_b11: f64 = (s.db[885][11] * __rspice_inv_cse_0);
        let eq73_e1906_d_b12: f64 = (s.db[885][12] * __rspice_inv_cse_0);
        let eq73_e1906_d_b13: f64 = (s.db[885][13] * __rspice_inv_cse_0);
        let eq73_e1906_d_b14: f64 = (s.db[885][14] * __rspice_inv_cse_0);
        let eq73_e1906_d_b15: f64 = (s.db[885][15] * __rspice_inv_cse_0);
        let eq73_e1906_d_b16: f64 = (s.db[885][16] * __rspice_inv_cse_0);
        let eq73_e1906_d_b17: f64 = (s.db[885][17] * __rspice_inv_cse_0);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1909: f64 = (eq73_e1907 * s.v[822]);
        let eq73_e1909_d_n0: f64 = (((-eq73_e1906_d_n0) * s.v[822]) + (eq73_e1907 * s.dn[822][0]));
        let eq73_e1909_d_n1: f64 = (((-eq73_e1906_d_n1) * s.v[822]) + (eq73_e1907 * s.dn[822][1]));
        let eq73_e1909_d_n2: f64 = (((-eq73_e1906_d_n2) * s.v[822]) + (eq73_e1907 * s.dn[822][2]));
        let eq73_e1909_d_n3: f64 = (((-eq73_e1906_d_n3) * s.v[822]) + (eq73_e1907 * s.dn[822][3]));
        let eq73_e1909_d_n4: f64 = (((-eq73_e1906_d_n4) * s.v[822]) + (eq73_e1907 * s.dn[822][4]));
        let eq73_e1909_d_n5: f64 = (((-eq73_e1906_d_n5) * s.v[822]) + (eq73_e1907 * s.dn[822][5]));
        let eq73_e1909_d_n6: f64 = (((-eq73_e1906_d_n6) * s.v[822]) + (eq73_e1907 * s.dn[822][6]));
        let eq73_e1909_d_n7: f64 = (((-eq73_e1906_d_n7) * s.v[822]) + (eq73_e1907 * s.dn[822][7]));
        let eq73_e1909_d_n8: f64 = (((-eq73_e1906_d_n8) * s.v[822]) + (eq73_e1907 * s.dn[822][8]));
        let eq73_e1909_d_n9: f64 = (((-eq73_e1906_d_n9) * s.v[822]) + (eq73_e1907 * s.dn[822][9]));
        let eq73_e1909_d_n10: f64 = (((-eq73_e1906_d_n10) * s.v[822]) + (eq73_e1907 * s.dn[822][10]));
        let eq73_e1909_d_n11: f64 = (((-eq73_e1906_d_n11) * s.v[822]) + (eq73_e1907 * s.dn[822][11]));
        let eq73_e1909_d_n12: f64 = (((-eq73_e1906_d_n12) * s.v[822]) + (eq73_e1907 * s.dn[822][12]));
        let eq73_e1909_d_n13: f64 = (((-eq73_e1906_d_n13) * s.v[822]) + (eq73_e1907 * s.dn[822][13]));
        let eq73_e1909_d_b0: f64 = (((-eq73_e1906_d_b0) * s.v[822]) + (eq73_e1907 * s.db[822][0]));
        let eq73_e1909_d_b1: f64 = (((-eq73_e1906_d_b1) * s.v[822]) + (eq73_e1907 * s.db[822][1]));
        let eq73_e1909_d_b2: f64 = (((-eq73_e1906_d_b2) * s.v[822]) + (eq73_e1907 * s.db[822][2]));
        let eq73_e1909_d_b3: f64 = (((-eq73_e1906_d_b3) * s.v[822]) + (eq73_e1907 * s.db[822][3]));
        let eq73_e1909_d_b4: f64 = (((-eq73_e1906_d_b4) * s.v[822]) + (eq73_e1907 * s.db[822][4]));
        let eq73_e1909_d_b5: f64 = (((-eq73_e1906_d_b5) * s.v[822]) + (eq73_e1907 * s.db[822][5]));
        let eq73_e1909_d_b6: f64 = (((-eq73_e1906_d_b6) * s.v[822]) + (eq73_e1907 * s.db[822][6]));
        let eq73_e1909_d_b7: f64 = (((-eq73_e1906_d_b7) * s.v[822]) + (eq73_e1907 * s.db[822][7]));
        let eq73_e1909_d_b8: f64 = (((-eq73_e1906_d_b8) * s.v[822]) + (eq73_e1907 * s.db[822][8]));
        let eq73_e1909_d_b9: f64 = (((-eq73_e1906_d_b9) * s.v[822]) + (eq73_e1907 * s.db[822][9]));
        let eq73_e1909_d_b10: f64 = (((-eq73_e1906_d_b10) * s.v[822]) + (eq73_e1907 * s.db[822][10]));
        let eq73_e1909_d_b11: f64 = (((-eq73_e1906_d_b11) * s.v[822]) + (eq73_e1907 * s.db[822][11]));
        let eq73_e1909_d_b12: f64 = (((-eq73_e1906_d_b12) * s.v[822]) + (eq73_e1907 * s.db[822][12]));
        let eq73_e1909_d_b13: f64 = (((-eq73_e1906_d_b13) * s.v[822]) + (eq73_e1907 * s.db[822][13]));
        let eq73_e1909_d_b14: f64 = (((-eq73_e1906_d_b14) * s.v[822]) + (eq73_e1907 * s.db[822][14]));
        let eq73_e1909_d_b15: f64 = (((-eq73_e1906_d_b15) * s.v[822]) + (eq73_e1907 * s.db[822][15]));
        let eq73_e1909_d_b16: f64 = (((-eq73_e1906_d_b16) * s.v[822]) + (eq73_e1907 * s.db[822][16]));
        let eq73_e1909_d_b17: f64 = (((-eq73_e1906_d_b17) * s.v[822]) + (eq73_e1907 * s.db[822][17]));
        let eq73_e1912: f64 = (s.v[410] * s.v[158]);
        let eq73_e1913_q: f64 = eq73_e1912;
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1912);
        let eq73_e1914_d_n0: f64 = (eq73_e1909_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq73_e1914_d_n1: f64 = (eq73_e1909_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq73_e1914_d_n2: f64 = (eq73_e1909_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq73_e1914_d_n13: f64 = (eq73_e1909_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq73_e1914_d_b0: f64 = (eq73_e1909_d_b0 + (s.db[410][0] * s.v[158]));
        let eq73_e1914_d_b1: f64 = (eq73_e1909_d_b1 + (s.db[410][1] * s.v[158]));
        let eq73_e1914_d_b2: f64 = (eq73_e1909_d_b2 + (s.db[410][2] * s.v[158]));
        let eq73_e1914_d_b3: f64 = (eq73_e1909_d_b3 + (s.db[410][3] * s.v[158]));
        let eq73_e1914_d_b4: f64 = (eq73_e1909_d_b4 + (s.db[410][4] * s.v[158]));
        let eq73_e1914_d_b5: f64 = (eq73_e1909_d_b5 + (s.db[410][5] * s.v[158]));
        let eq73_e1914_d_b6: f64 = (eq73_e1909_d_b6 + (s.db[410][6] * s.v[158]));
        let eq73_e1914_d_b7: f64 = (eq73_e1909_d_b7 + (s.db[410][7] * s.v[158]));
        let eq73_e1914_d_b8: f64 = (eq73_e1909_d_b8 + (s.db[410][8] * s.v[158]));
        let eq73_e1914_d_b9: f64 = (eq73_e1909_d_b9 + (s.db[410][9] * s.v[158]));
        let eq73_e1914_d_b10: f64 = (eq73_e1909_d_b10 + (s.db[410][10] * s.v[158]));
        let eq73_e1914_d_b11: f64 = (eq73_e1909_d_b11 + (s.db[410][11] * s.v[158]));
        let eq73_e1914_d_b12: f64 = (eq73_e1909_d_b12 + (s.db[410][12] * s.v[158]));
        let eq73_e1914_d_b13: f64 = (eq73_e1909_d_b13 + (s.db[410][13] * s.v[158]));
        let eq73_e1914_d_b14: f64 = (eq73_e1909_d_b14 + (s.db[410][14] * s.v[158]));
        let eq73_e1914_d_b15: f64 = (eq73_e1909_d_b15 + (s.db[410][15] * s.v[158]));
        let eq73_e1914_d_b16: f64 = (eq73_e1909_d_b16 + (s.db[410][16] * s.v[158]));
        let eq73_e1914_d_b17: f64 = (eq73_e1909_d_b17 + (s.db[410][17] * s.v[158]));
        let eq73_e1914_q: f64 = eq73_e1913_q;
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq73_e1917: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq73_e1917_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq73_e1917_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq73_e1917_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq73_e1917_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq73_e1917_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq73_e1917_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq73_e1917_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq73_e1917_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq73_e1917_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq73_e1917_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq73_e1917_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq73_e1917_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq73_e1917_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq73_e1917_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq73_e1917_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq73_e1917_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq73_e1917_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq73_e1917_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq73_e1917_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq73_e1917_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq73_e1917_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq73_e1917_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq73_e1917_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq73_e1917_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq73_e1917_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq73_e1917_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq73_e1917_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq73_e1917_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq73_e1917_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq73_e1917_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq73_e1917_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq73_e1917_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n0: f64 = (eq73_e1914_d_n0 + eq73_e1917_d_n0);
        let eq73_e1918_d_n1: f64 = (eq73_e1914_d_n1 + eq73_e1917_d_n1);
        let eq73_e1918_d_n2: f64 = (eq73_e1914_d_n2 + eq73_e1917_d_n2);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        let eq73_e1918_d_n13: f64 = (eq73_e1914_d_n13 + eq73_e1917_d_n13);
        let eq73_e1918_d_b0: f64 = (eq73_e1914_d_b0 + eq73_e1917_d_b0);
        let eq73_e1918_d_b1: f64 = (eq73_e1914_d_b1 + eq73_e1917_d_b1);
        let eq73_e1918_d_b2: f64 = (eq73_e1914_d_b2 + eq73_e1917_d_b2);
        let eq73_e1918_d_b3: f64 = (eq73_e1914_d_b3 + eq73_e1917_d_b3);
        let eq73_e1918_d_b4: f64 = (eq73_e1914_d_b4 + eq73_e1917_d_b4);
        let eq73_e1918_d_b5: f64 = (eq73_e1914_d_b5 + eq73_e1917_d_b5);
        let eq73_e1918_d_b6: f64 = (eq73_e1914_d_b6 + eq73_e1917_d_b6);
        let eq73_e1918_d_b7: f64 = (eq73_e1914_d_b7 + eq73_e1917_d_b7);
        let eq73_e1918_d_b8: f64 = (eq73_e1914_d_b8 + eq73_e1917_d_b8);
        let eq73_e1918_d_b9: f64 = (eq73_e1914_d_b9 + eq73_e1917_d_b9);
        let eq73_e1918_d_b10: f64 = (eq73_e1914_d_b10 + eq73_e1917_d_b10);
        let eq73_e1918_d_b11: f64 = (eq73_e1914_d_b11 + eq73_e1917_d_b11);
        let eq73_e1918_d_b12: f64 = (eq73_e1914_d_b12 + eq73_e1917_d_b12);
        let eq73_e1918_d_b13: f64 = (eq73_e1914_d_b13 + eq73_e1917_d_b13);
        let eq73_e1918_d_b14: f64 = (eq73_e1914_d_b14 + eq73_e1917_d_b14);
        let eq73_e1918_d_b15: f64 = (eq73_e1914_d_b15 + eq73_e1917_d_b15);
        let eq73_e1918_d_b16: f64 = (eq73_e1914_d_b16 + eq73_e1917_d_b16);
        let eq73_e1918_d_b17: f64 = (eq73_e1914_d_b17 + eq73_e1917_d_b17);
        let eq73_e1918_q: f64 = eq73_e1914_q;
        (eq73_e1918, eq73_e1918_d_n0, eq73_e1918_d_n1, eq73_e1918_d_n2, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_d_n13, eq73_e1918_d_b0, eq73_e1918_d_b1, eq73_e1918_d_b2, eq73_e1918_d_b3, eq73_e1918_d_b4, eq73_e1918_d_b5, eq73_e1918_d_b6, eq73_e1918_d_b7, eq73_e1918_d_b8, eq73_e1918_d_b9, eq73_e1918_d_b10, eq73_e1918_d_b11, eq73_e1918_d_b12, eq73_e1918_d_b13, eq73_e1918_d_b14, eq73_e1918_d_b15, eq73_e1918_d_b16, eq73_e1918_d_b17, eq73_e1918_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 14] = [eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13];
        let eq73_reactive_branch_derivatives: [f64; 18] = [eq73_e1920_q_d_b0, eq73_e1920_q_d_b1, eq73_e1920_q_d_b2, eq73_e1920_q_d_b3, eq73_e1920_q_d_b4, eq73_e1920_q_d_b5, eq73_e1920_q_d_b6, eq73_e1920_q_d_b7, eq73_e1920_q_d_b8, eq73_e1920_q_d_b9, eq73_e1920_q_d_b10, eq73_e1920_q_d_b11, eq73_e1920_q_d_b12, eq73_e1920_q_d_b13, eq73_e1920_q_d_b14, eq73_e1920_q_d_b15, eq73_e1920_q_d_b16, eq73_e1920_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1947, eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13, eq74_e1947_d_b0, eq74_e1947_d_b1, eq74_e1947_d_b2, eq74_e1947_d_b3, eq74_e1947_d_b4, eq74_e1947_d_b5, eq74_e1947_d_b6, eq74_e1947_d_b7, eq74_e1947_d_b8, eq74_e1947_d_b9, eq74_e1947_d_b10, eq74_e1947_d_b11, eq74_e1947_d_b12, eq74_e1947_d_b13, eq74_e1947_d_b14, eq74_e1947_d_b15, eq74_e1947_d_b16, eq74_e1947_d_b17, eq74_e1947_q, eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13, eq74_e1947_q_d_b0, eq74_e1947_q_d_b1, eq74_e1947_q_d_b2, eq74_e1947_q_d_b3, eq74_e1947_q_d_b4, eq74_e1947_q_d_b5, eq74_e1947_q_d_b6, eq74_e1947_q_d_b7, eq74_e1947_q_d_b8, eq74_e1947_q_d_b9, eq74_e1947_q_d_b10, eq74_e1947_q_d_b11, eq74_e1947_q_d_b12, eq74_e1947_q_d_b13, eq74_e1947_q_d_b14, eq74_e1947_q_d_b15, eq74_e1947_q_d_b16, eq74_e1947_q_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && (!s.b[1563])) {
        let eq74_e1934: f64 = (-s.v[885]);
        let eq74_e1936: f64 = (eq74_e1934 * s.v[822]);
        let eq74_e1936_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq74_e1934 * s.dn[822][0]));
        let eq74_e1936_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq74_e1934 * s.dn[822][1]));
        let eq74_e1936_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq74_e1934 * s.dn[822][2]));
        let eq74_e1936_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq74_e1934 * s.dn[822][3]));
        let eq74_e1936_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq74_e1934 * s.dn[822][4]));
        let eq74_e1936_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq74_e1934 * s.dn[822][5]));
        let eq74_e1936_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq74_e1934 * s.dn[822][6]));
        let eq74_e1936_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq74_e1934 * s.dn[822][7]));
        let eq74_e1936_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq74_e1934 * s.dn[822][8]));
        let eq74_e1936_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq74_e1934 * s.dn[822][9]));
        let eq74_e1936_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq74_e1934 * s.dn[822][10]));
        let eq74_e1936_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq74_e1934 * s.dn[822][11]));
        let eq74_e1936_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq74_e1934 * s.dn[822][12]));
        let eq74_e1936_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq74_e1934 * s.dn[822][13]));
        let eq74_e1936_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq74_e1934 * s.db[822][0]));
        let eq74_e1936_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq74_e1934 * s.db[822][1]));
        let eq74_e1936_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq74_e1934 * s.db[822][2]));
        let eq74_e1936_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq74_e1934 * s.db[822][3]));
        let eq74_e1936_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq74_e1934 * s.db[822][4]));
        let eq74_e1936_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq74_e1934 * s.db[822][5]));
        let eq74_e1936_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq74_e1934 * s.db[822][6]));
        let eq74_e1936_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq74_e1934 * s.db[822][7]));
        let eq74_e1936_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq74_e1934 * s.db[822][8]));
        let eq74_e1936_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq74_e1934 * s.db[822][9]));
        let eq74_e1936_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq74_e1934 * s.db[822][10]));
        let eq74_e1936_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq74_e1934 * s.db[822][11]));
        let eq74_e1936_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq74_e1934 * s.db[822][12]));
        let eq74_e1936_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq74_e1934 * s.db[822][13]));
        let eq74_e1936_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq74_e1934 * s.db[822][14]));
        let eq74_e1936_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq74_e1934 * s.db[822][15]));
        let eq74_e1936_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq74_e1934 * s.db[822][16]));
        let eq74_e1936_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq74_e1934 * s.db[822][17]));
        let eq74_e1939: f64 = (s.v[410] * s.v[158]);
        let eq74_e1940_q: f64 = eq74_e1939;
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1939);
        let eq74_e1941_d_n0: f64 = (eq74_e1936_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq74_e1941_d_n1: f64 = (eq74_e1936_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq74_e1941_d_n2: f64 = (eq74_e1936_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq74_e1941_d_n13: f64 = (eq74_e1936_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq74_e1941_d_b0: f64 = (eq74_e1936_d_b0 + (s.db[410][0] * s.v[158]));
        let eq74_e1941_d_b1: f64 = (eq74_e1936_d_b1 + (s.db[410][1] * s.v[158]));
        let eq74_e1941_d_b2: f64 = (eq74_e1936_d_b2 + (s.db[410][2] * s.v[158]));
        let eq74_e1941_d_b3: f64 = (eq74_e1936_d_b3 + (s.db[410][3] * s.v[158]));
        let eq74_e1941_d_b4: f64 = (eq74_e1936_d_b4 + (s.db[410][4] * s.v[158]));
        let eq74_e1941_d_b5: f64 = (eq74_e1936_d_b5 + (s.db[410][5] * s.v[158]));
        let eq74_e1941_d_b6: f64 = (eq74_e1936_d_b6 + (s.db[410][6] * s.v[158]));
        let eq74_e1941_d_b7: f64 = (eq74_e1936_d_b7 + (s.db[410][7] * s.v[158]));
        let eq74_e1941_d_b8: f64 = (eq74_e1936_d_b8 + (s.db[410][8] * s.v[158]));
        let eq74_e1941_d_b9: f64 = (eq74_e1936_d_b9 + (s.db[410][9] * s.v[158]));
        let eq74_e1941_d_b10: f64 = (eq74_e1936_d_b10 + (s.db[410][10] * s.v[158]));
        let eq74_e1941_d_b11: f64 = (eq74_e1936_d_b11 + (s.db[410][11] * s.v[158]));
        let eq74_e1941_d_b12: f64 = (eq74_e1936_d_b12 + (s.db[410][12] * s.v[158]));
        let eq74_e1941_d_b13: f64 = (eq74_e1936_d_b13 + (s.db[410][13] * s.v[158]));
        let eq74_e1941_d_b14: f64 = (eq74_e1936_d_b14 + (s.db[410][14] * s.v[158]));
        let eq74_e1941_d_b15: f64 = (eq74_e1936_d_b15 + (s.db[410][15] * s.v[158]));
        let eq74_e1941_d_b16: f64 = (eq74_e1936_d_b16 + (s.db[410][16] * s.v[158]));
        let eq74_e1941_d_b17: f64 = (eq74_e1936_d_b17 + (s.db[410][17] * s.v[158]));
        let eq74_e1941_q: f64 = eq74_e1940_q;
        let __rspice_inv_cse_2: f64 = 1.0 / s.v[157];
        let eq74_e1944: f64 = (s.v[410] * __rspice_inv_cse_2);
        let eq74_e1944_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_2);
        let eq74_e1944_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_2);
        let eq74_e1944_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_2);
        let eq74_e1944_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_2);
        let eq74_e1944_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_2);
        let eq74_e1944_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_2);
        let eq74_e1944_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_2);
        let eq74_e1944_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_2);
        let eq74_e1944_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_2);
        let eq74_e1944_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_2);
        let eq74_e1944_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_2);
        let eq74_e1944_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_2);
        let eq74_e1944_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_2);
        let eq74_e1944_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_2);
        let eq74_e1944_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_2);
        let eq74_e1944_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_2);
        let eq74_e1944_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_2);
        let eq74_e1944_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_2);
        let eq74_e1944_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_2);
        let eq74_e1944_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_2);
        let eq74_e1944_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_2);
        let eq74_e1944_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_2);
        let eq74_e1944_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_2);
        let eq74_e1944_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_2);
        let eq74_e1944_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_2);
        let eq74_e1944_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_2);
        let eq74_e1944_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_2);
        let eq74_e1944_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_2);
        let eq74_e1944_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_2);
        let eq74_e1944_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_2);
        let eq74_e1944_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_2);
        let eq74_e1944_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_2);
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n0: f64 = (eq74_e1941_d_n0 + eq74_e1944_d_n0);
        let eq74_e1945_d_n1: f64 = (eq74_e1941_d_n1 + eq74_e1944_d_n1);
        let eq74_e1945_d_n2: f64 = (eq74_e1941_d_n2 + eq74_e1944_d_n2);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        let eq74_e1945_d_n13: f64 = (eq74_e1941_d_n13 + eq74_e1944_d_n13);
        let eq74_e1945_d_b0: f64 = (eq74_e1941_d_b0 + eq74_e1944_d_b0);
        let eq74_e1945_d_b1: f64 = (eq74_e1941_d_b1 + eq74_e1944_d_b1);
        let eq74_e1945_d_b2: f64 = (eq74_e1941_d_b2 + eq74_e1944_d_b2);
        let eq74_e1945_d_b3: f64 = (eq74_e1941_d_b3 + eq74_e1944_d_b3);
        let eq74_e1945_d_b4: f64 = (eq74_e1941_d_b4 + eq74_e1944_d_b4);
        let eq74_e1945_d_b5: f64 = (eq74_e1941_d_b5 + eq74_e1944_d_b5);
        let eq74_e1945_d_b6: f64 = (eq74_e1941_d_b6 + eq74_e1944_d_b6);
        let eq74_e1945_d_b7: f64 = (eq74_e1941_d_b7 + eq74_e1944_d_b7);
        let eq74_e1945_d_b8: f64 = (eq74_e1941_d_b8 + eq74_e1944_d_b8);
        let eq74_e1945_d_b9: f64 = (eq74_e1941_d_b9 + eq74_e1944_d_b9);
        let eq74_e1945_d_b10: f64 = (eq74_e1941_d_b10 + eq74_e1944_d_b10);
        let eq74_e1945_d_b11: f64 = (eq74_e1941_d_b11 + eq74_e1944_d_b11);
        let eq74_e1945_d_b12: f64 = (eq74_e1941_d_b12 + eq74_e1944_d_b12);
        let eq74_e1945_d_b13: f64 = (eq74_e1941_d_b13 + eq74_e1944_d_b13);
        let eq74_e1945_d_b14: f64 = (eq74_e1941_d_b14 + eq74_e1944_d_b14);
        let eq74_e1945_d_b15: f64 = (eq74_e1941_d_b15 + eq74_e1944_d_b15);
        let eq74_e1945_d_b16: f64 = (eq74_e1941_d_b16 + eq74_e1944_d_b16);
        let eq74_e1945_d_b17: f64 = (eq74_e1941_d_b17 + eq74_e1944_d_b17);
        let eq74_e1945_q: f64 = eq74_e1941_q;
        (eq74_e1945, eq74_e1945_d_n0, eq74_e1945_d_n1, eq74_e1945_d_n2, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_d_n13, eq74_e1945_d_b0, eq74_e1945_d_b1, eq74_e1945_d_b2, eq74_e1945_d_b3, eq74_e1945_d_b4, eq74_e1945_d_b5, eq74_e1945_d_b6, eq74_e1945_d_b7, eq74_e1945_d_b8, eq74_e1945_d_b9, eq74_e1945_d_b10, eq74_e1945_d_b11, eq74_e1945_d_b12, eq74_e1945_d_b13, eq74_e1945_d_b14, eq74_e1945_d_b15, eq74_e1945_d_b16, eq74_e1945_d_b17, eq74_e1945_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 14] = [eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13];
        let eq74_reactive_branch_derivatives: [f64; 18] = [eq74_e1947_q_d_b0, eq74_e1947_q_d_b1, eq74_e1947_q_d_b2, eq74_e1947_q_d_b3, eq74_e1947_q_d_b4, eq74_e1947_q_d_b5, eq74_e1947_q_d_b6, eq74_e1947_q_d_b7, eq74_e1947_q_d_b8, eq74_e1947_q_d_b9, eq74_e1947_q_d_b10, eq74_e1947_q_d_b11, eq74_e1947_q_d_b12, eq74_e1947_q_d_b13, eq74_e1947_q_d_b14, eq74_e1947_q_d_b15, eq74_e1947_q_d_b16, eq74_e1947_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq75_e1970, eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13, eq75_e1970_d_b0, eq75_e1970_d_b1, eq75_e1970_d_b2, eq75_e1970_d_b3, eq75_e1970_d_b4, eq75_e1970_d_b5, eq75_e1970_d_b6, eq75_e1970_d_b7, eq75_e1970_d_b8, eq75_e1970_d_b9, eq75_e1970_d_b10, eq75_e1970_d_b11, eq75_e1970_d_b12, eq75_e1970_d_b13, eq75_e1970_d_b14, eq75_e1970_d_b15, eq75_e1970_d_b16, eq75_e1970_d_b17, eq75_e1970_q, eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13, eq75_e1970_q_d_b0, eq75_e1970_q_d_b1, eq75_e1970_q_d_b2, eq75_e1970_q_d_b3, eq75_e1970_q_d_b4, eq75_e1970_q_d_b5, eq75_e1970_q_d_b6, eq75_e1970_q_d_b7, eq75_e1970_q_d_b8, eq75_e1970_q_d_b9, eq75_e1970_q_d_b10, eq75_e1970_q_d_b11, eq75_e1970_q_d_b12, eq75_e1970_q_d_b13, eq75_e1970_q_d_b14, eq75_e1970_q_d_b15, eq75_e1970_q_d_b16, eq75_e1970_q_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && s.b[1564]) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq75_e1956: f64 = (s.v[885] * __rspice_inv_cse_0);
        let eq75_e1956_d_n0: f64 = (s.dn[885][0] * __rspice_inv_cse_0);
        let eq75_e1956_d_n1: f64 = (s.dn[885][1] * __rspice_inv_cse_0);
        let eq75_e1956_d_n2: f64 = (s.dn[885][2] * __rspice_inv_cse_0);
        let eq75_e1956_d_n3: f64 = (s.dn[885][3] * __rspice_inv_cse_0);
        let eq75_e1956_d_n4: f64 = (s.dn[885][4] * __rspice_inv_cse_0);
        let eq75_e1956_d_n5: f64 = (s.dn[885][5] * __rspice_inv_cse_0);
        let eq75_e1956_d_n6: f64 = (s.dn[885][6] * __rspice_inv_cse_0);
        let eq75_e1956_d_n7: f64 = (s.dn[885][7] * __rspice_inv_cse_0);
        let eq75_e1956_d_n8: f64 = (s.dn[885][8] * __rspice_inv_cse_0);
        let eq75_e1956_d_n9: f64 = (s.dn[885][9] * __rspice_inv_cse_0);
        let eq75_e1956_d_n10: f64 = (s.dn[885][10] * __rspice_inv_cse_0);
        let eq75_e1956_d_n11: f64 = (s.dn[885][11] * __rspice_inv_cse_0);
        let eq75_e1956_d_n12: f64 = (s.dn[885][12] * __rspice_inv_cse_0);
        let eq75_e1956_d_n13: f64 = (s.dn[885][13] * __rspice_inv_cse_0);
        let eq75_e1956_d_b0: f64 = (s.db[885][0] * __rspice_inv_cse_0);
        let eq75_e1956_d_b1: f64 = (s.db[885][1] * __rspice_inv_cse_0);
        let eq75_e1956_d_b2: f64 = (s.db[885][2] * __rspice_inv_cse_0);
        let eq75_e1956_d_b3: f64 = (s.db[885][3] * __rspice_inv_cse_0);
        let eq75_e1956_d_b4: f64 = (s.db[885][4] * __rspice_inv_cse_0);
        let eq75_e1956_d_b5: f64 = (s.db[885][5] * __rspice_inv_cse_0);
        let eq75_e1956_d_b6: f64 = (s.db[885][6] * __rspice_inv_cse_0);
        let eq75_e1956_d_b7: f64 = (s.db[885][7] * __rspice_inv_cse_0);
        let eq75_e1956_d_b8: f64 = (s.db[885][8] * __rspice_inv_cse_0);
        let eq75_e1956_d_b9: f64 = (s.db[885][9] * __rspice_inv_cse_0);
        let eq75_e1956_d_b10: f64 = (s.db[885][10] * __rspice_inv_cse_0);
        let eq75_e1956_d_b11: f64 = (s.db[885][11] * __rspice_inv_cse_0);
        let eq75_e1956_d_b12: f64 = (s.db[885][12] * __rspice_inv_cse_0);
        let eq75_e1956_d_b13: f64 = (s.db[885][13] * __rspice_inv_cse_0);
        let eq75_e1956_d_b14: f64 = (s.db[885][14] * __rspice_inv_cse_0);
        let eq75_e1956_d_b15: f64 = (s.db[885][15] * __rspice_inv_cse_0);
        let eq75_e1956_d_b16: f64 = (s.db[885][16] * __rspice_inv_cse_0);
        let eq75_e1956_d_b17: f64 = (s.db[885][17] * __rspice_inv_cse_0);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1959: f64 = (eq75_e1957 * s.v[822]);
        let eq75_e1959_d_n0: f64 = (((-eq75_e1956_d_n0) * s.v[822]) + (eq75_e1957 * s.dn[822][0]));
        let eq75_e1959_d_n1: f64 = (((-eq75_e1956_d_n1) * s.v[822]) + (eq75_e1957 * s.dn[822][1]));
        let eq75_e1959_d_n2: f64 = (((-eq75_e1956_d_n2) * s.v[822]) + (eq75_e1957 * s.dn[822][2]));
        let eq75_e1959_d_n3: f64 = (((-eq75_e1956_d_n3) * s.v[822]) + (eq75_e1957 * s.dn[822][3]));
        let eq75_e1959_d_n4: f64 = (((-eq75_e1956_d_n4) * s.v[822]) + (eq75_e1957 * s.dn[822][4]));
        let eq75_e1959_d_n5: f64 = (((-eq75_e1956_d_n5) * s.v[822]) + (eq75_e1957 * s.dn[822][5]));
        let eq75_e1959_d_n6: f64 = (((-eq75_e1956_d_n6) * s.v[822]) + (eq75_e1957 * s.dn[822][6]));
        let eq75_e1959_d_n7: f64 = (((-eq75_e1956_d_n7) * s.v[822]) + (eq75_e1957 * s.dn[822][7]));
        let eq75_e1959_d_n8: f64 = (((-eq75_e1956_d_n8) * s.v[822]) + (eq75_e1957 * s.dn[822][8]));
        let eq75_e1959_d_n9: f64 = (((-eq75_e1956_d_n9) * s.v[822]) + (eq75_e1957 * s.dn[822][9]));
        let eq75_e1959_d_n10: f64 = (((-eq75_e1956_d_n10) * s.v[822]) + (eq75_e1957 * s.dn[822][10]));
        let eq75_e1959_d_n11: f64 = (((-eq75_e1956_d_n11) * s.v[822]) + (eq75_e1957 * s.dn[822][11]));
        let eq75_e1959_d_n12: f64 = (((-eq75_e1956_d_n12) * s.v[822]) + (eq75_e1957 * s.dn[822][12]));
        let eq75_e1959_d_n13: f64 = (((-eq75_e1956_d_n13) * s.v[822]) + (eq75_e1957 * s.dn[822][13]));
        let eq75_e1959_d_b0: f64 = (((-eq75_e1956_d_b0) * s.v[822]) + (eq75_e1957 * s.db[822][0]));
        let eq75_e1959_d_b1: f64 = (((-eq75_e1956_d_b1) * s.v[822]) + (eq75_e1957 * s.db[822][1]));
        let eq75_e1959_d_b2: f64 = (((-eq75_e1956_d_b2) * s.v[822]) + (eq75_e1957 * s.db[822][2]));
        let eq75_e1959_d_b3: f64 = (((-eq75_e1956_d_b3) * s.v[822]) + (eq75_e1957 * s.db[822][3]));
        let eq75_e1959_d_b4: f64 = (((-eq75_e1956_d_b4) * s.v[822]) + (eq75_e1957 * s.db[822][4]));
        let eq75_e1959_d_b5: f64 = (((-eq75_e1956_d_b5) * s.v[822]) + (eq75_e1957 * s.db[822][5]));
        let eq75_e1959_d_b6: f64 = (((-eq75_e1956_d_b6) * s.v[822]) + (eq75_e1957 * s.db[822][6]));
        let eq75_e1959_d_b7: f64 = (((-eq75_e1956_d_b7) * s.v[822]) + (eq75_e1957 * s.db[822][7]));
        let eq75_e1959_d_b8: f64 = (((-eq75_e1956_d_b8) * s.v[822]) + (eq75_e1957 * s.db[822][8]));
        let eq75_e1959_d_b9: f64 = (((-eq75_e1956_d_b9) * s.v[822]) + (eq75_e1957 * s.db[822][9]));
        let eq75_e1959_d_b10: f64 = (((-eq75_e1956_d_b10) * s.v[822]) + (eq75_e1957 * s.db[822][10]));
        let eq75_e1959_d_b11: f64 = (((-eq75_e1956_d_b11) * s.v[822]) + (eq75_e1957 * s.db[822][11]));
        let eq75_e1959_d_b12: f64 = (((-eq75_e1956_d_b12) * s.v[822]) + (eq75_e1957 * s.db[822][12]));
        let eq75_e1959_d_b13: f64 = (((-eq75_e1956_d_b13) * s.v[822]) + (eq75_e1957 * s.db[822][13]));
        let eq75_e1959_d_b14: f64 = (((-eq75_e1956_d_b14) * s.v[822]) + (eq75_e1957 * s.db[822][14]));
        let eq75_e1959_d_b15: f64 = (((-eq75_e1956_d_b15) * s.v[822]) + (eq75_e1957 * s.db[822][15]));
        let eq75_e1959_d_b16: f64 = (((-eq75_e1956_d_b16) * s.v[822]) + (eq75_e1957 * s.db[822][16]));
        let eq75_e1959_d_b17: f64 = (((-eq75_e1956_d_b17) * s.v[822]) + (eq75_e1957 * s.db[822][17]));
        let eq75_e1962: f64 = (s.v[410] * s.v[158]);
        let eq75_e1963_q: f64 = eq75_e1962;
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1962);
        let eq75_e1964_d_n0: f64 = (eq75_e1959_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq75_e1964_d_n1: f64 = (eq75_e1959_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq75_e1964_d_n2: f64 = (eq75_e1959_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq75_e1964_d_n13: f64 = (eq75_e1959_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq75_e1964_d_b0: f64 = (eq75_e1959_d_b0 + (s.db[410][0] * s.v[158]));
        let eq75_e1964_d_b1: f64 = (eq75_e1959_d_b1 + (s.db[410][1] * s.v[158]));
        let eq75_e1964_d_b2: f64 = (eq75_e1959_d_b2 + (s.db[410][2] * s.v[158]));
        let eq75_e1964_d_b3: f64 = (eq75_e1959_d_b3 + (s.db[410][3] * s.v[158]));
        let eq75_e1964_d_b4: f64 = (eq75_e1959_d_b4 + (s.db[410][4] * s.v[158]));
        let eq75_e1964_d_b5: f64 = (eq75_e1959_d_b5 + (s.db[410][5] * s.v[158]));
        let eq75_e1964_d_b6: f64 = (eq75_e1959_d_b6 + (s.db[410][6] * s.v[158]));
        let eq75_e1964_d_b7: f64 = (eq75_e1959_d_b7 + (s.db[410][7] * s.v[158]));
        let eq75_e1964_d_b8: f64 = (eq75_e1959_d_b8 + (s.db[410][8] * s.v[158]));
        let eq75_e1964_d_b9: f64 = (eq75_e1959_d_b9 + (s.db[410][9] * s.v[158]));
        let eq75_e1964_d_b10: f64 = (eq75_e1959_d_b10 + (s.db[410][10] * s.v[158]));
        let eq75_e1964_d_b11: f64 = (eq75_e1959_d_b11 + (s.db[410][11] * s.v[158]));
        let eq75_e1964_d_b12: f64 = (eq75_e1959_d_b12 + (s.db[410][12] * s.v[158]));
        let eq75_e1964_d_b13: f64 = (eq75_e1959_d_b13 + (s.db[410][13] * s.v[158]));
        let eq75_e1964_d_b14: f64 = (eq75_e1959_d_b14 + (s.db[410][14] * s.v[158]));
        let eq75_e1964_d_b15: f64 = (eq75_e1959_d_b15 + (s.db[410][15] * s.v[158]));
        let eq75_e1964_d_b16: f64 = (eq75_e1959_d_b16 + (s.db[410][16] * s.v[158]));
        let eq75_e1964_d_b17: f64 = (eq75_e1959_d_b17 + (s.db[410][17] * s.v[158]));
        let eq75_e1964_q: f64 = eq75_e1963_q;
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq75_e1967: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq75_e1967_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq75_e1967_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq75_e1967_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq75_e1967_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq75_e1967_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq75_e1967_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq75_e1967_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq75_e1967_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq75_e1967_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq75_e1967_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq75_e1967_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq75_e1967_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq75_e1967_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq75_e1967_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq75_e1967_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq75_e1967_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq75_e1967_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq75_e1967_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq75_e1967_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq75_e1967_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq75_e1967_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq75_e1967_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq75_e1967_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq75_e1967_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq75_e1967_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq75_e1967_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq75_e1967_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq75_e1967_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq75_e1967_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq75_e1967_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq75_e1967_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq75_e1967_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n0: f64 = (eq75_e1964_d_n0 + eq75_e1967_d_n0);
        let eq75_e1968_d_n1: f64 = (eq75_e1964_d_n1 + eq75_e1967_d_n1);
        let eq75_e1968_d_n2: f64 = (eq75_e1964_d_n2 + eq75_e1967_d_n2);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        let eq75_e1968_d_n13: f64 = (eq75_e1964_d_n13 + eq75_e1967_d_n13);
        let eq75_e1968_d_b0: f64 = (eq75_e1964_d_b0 + eq75_e1967_d_b0);
        let eq75_e1968_d_b1: f64 = (eq75_e1964_d_b1 + eq75_e1967_d_b1);
        let eq75_e1968_d_b2: f64 = (eq75_e1964_d_b2 + eq75_e1967_d_b2);
        let eq75_e1968_d_b3: f64 = (eq75_e1964_d_b3 + eq75_e1967_d_b3);
        let eq75_e1968_d_b4: f64 = (eq75_e1964_d_b4 + eq75_e1967_d_b4);
        let eq75_e1968_d_b5: f64 = (eq75_e1964_d_b5 + eq75_e1967_d_b5);
        let eq75_e1968_d_b6: f64 = (eq75_e1964_d_b6 + eq75_e1967_d_b6);
        let eq75_e1968_d_b7: f64 = (eq75_e1964_d_b7 + eq75_e1967_d_b7);
        let eq75_e1968_d_b8: f64 = (eq75_e1964_d_b8 + eq75_e1967_d_b8);
        let eq75_e1968_d_b9: f64 = (eq75_e1964_d_b9 + eq75_e1967_d_b9);
        let eq75_e1968_d_b10: f64 = (eq75_e1964_d_b10 + eq75_e1967_d_b10);
        let eq75_e1968_d_b11: f64 = (eq75_e1964_d_b11 + eq75_e1967_d_b11);
        let eq75_e1968_d_b12: f64 = (eq75_e1964_d_b12 + eq75_e1967_d_b12);
        let eq75_e1968_d_b13: f64 = (eq75_e1964_d_b13 + eq75_e1967_d_b13);
        let eq75_e1968_d_b14: f64 = (eq75_e1964_d_b14 + eq75_e1967_d_b14);
        let eq75_e1968_d_b15: f64 = (eq75_e1964_d_b15 + eq75_e1967_d_b15);
        let eq75_e1968_d_b16: f64 = (eq75_e1964_d_b16 + eq75_e1967_d_b16);
        let eq75_e1968_d_b17: f64 = (eq75_e1964_d_b17 + eq75_e1967_d_b17);
        let eq75_e1968_q: f64 = eq75_e1964_q;
        (eq75_e1968, eq75_e1968_d_n0, eq75_e1968_d_n1, eq75_e1968_d_n2, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_d_n13, eq75_e1968_d_b0, eq75_e1968_d_b1, eq75_e1968_d_b2, eq75_e1968_d_b3, eq75_e1968_d_b4, eq75_e1968_d_b5, eq75_e1968_d_b6, eq75_e1968_d_b7, eq75_e1968_d_b8, eq75_e1968_d_b9, eq75_e1968_d_b10, eq75_e1968_d_b11, eq75_e1968_d_b12, eq75_e1968_d_b13, eq75_e1968_d_b14, eq75_e1968_d_b15, eq75_e1968_d_b16, eq75_e1968_d_b17, eq75_e1968_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_reactive_node_derivatives: [f64; 14] = [eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13];
        let eq75_reactive_branch_derivatives: [f64; 18] = [eq75_e1970_q_d_b0, eq75_e1970_q_d_b1, eq75_e1970_q_d_b2, eq75_e1970_q_d_b3, eq75_e1970_q_d_b4, eq75_e1970_q_d_b5, eq75_e1970_q_d_b6, eq75_e1970_q_d_b7, eq75_e1970_q_d_b8, eq75_e1970_q_d_b9, eq75_e1970_q_d_b10, eq75_e1970_q_d_b11, eq75_e1970_q_d_b12, eq75_e1970_q_d_b13, eq75_e1970_q_d_b14, eq75_e1970_q_d_b15, eq75_e1970_q_d_b16, eq75_e1970_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq75_reactive_node_derivatives,
            branches,
            &eq75_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1992, eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13, eq76_e1992_d_b0, eq76_e1992_d_b1, eq76_e1992_d_b2, eq76_e1992_d_b3, eq76_e1992_d_b4, eq76_e1992_d_b5, eq76_e1992_d_b6, eq76_e1992_d_b7, eq76_e1992_d_b8, eq76_e1992_d_b9, eq76_e1992_d_b10, eq76_e1992_d_b11, eq76_e1992_d_b12, eq76_e1992_d_b13, eq76_e1992_d_b14, eq76_e1992_d_b15, eq76_e1992_d_b16, eq76_e1992_d_b17, eq76_e1992_q, eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13, eq76_e1992_q_d_b0, eq76_e1992_q_d_b1, eq76_e1992_q_d_b2, eq76_e1992_q_d_b3, eq76_e1992_q_d_b4, eq76_e1992_q_d_b5, eq76_e1992_q_d_b6, eq76_e1992_q_d_b7, eq76_e1992_q_d_b8, eq76_e1992_q_d_b9, eq76_e1992_q_d_b10, eq76_e1992_q_d_b11, eq76_e1992_q_d_b12, eq76_e1992_q_d_b13, eq76_e1992_q_d_b14, eq76_e1992_q_d_b15, eq76_e1992_q_d_b16, eq76_e1992_q_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && (!s.b[1564])) {
        let eq76_e1979: f64 = (-s.v[885]);
        let eq76_e1981: f64 = (eq76_e1979 * s.v[822]);
        let eq76_e1981_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq76_e1979 * s.dn[822][0]));
        let eq76_e1981_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq76_e1979 * s.dn[822][1]));
        let eq76_e1981_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq76_e1979 * s.dn[822][2]));
        let eq76_e1981_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq76_e1979 * s.dn[822][3]));
        let eq76_e1981_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq76_e1979 * s.dn[822][4]));
        let eq76_e1981_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq76_e1979 * s.dn[822][5]));
        let eq76_e1981_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq76_e1979 * s.dn[822][6]));
        let eq76_e1981_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq76_e1979 * s.dn[822][7]));
        let eq76_e1981_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq76_e1979 * s.dn[822][8]));
        let eq76_e1981_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq76_e1979 * s.dn[822][9]));
        let eq76_e1981_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq76_e1979 * s.dn[822][10]));
        let eq76_e1981_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq76_e1979 * s.dn[822][11]));
        let eq76_e1981_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq76_e1979 * s.dn[822][12]));
        let eq76_e1981_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq76_e1979 * s.dn[822][13]));
        let eq76_e1981_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq76_e1979 * s.db[822][0]));
        let eq76_e1981_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq76_e1979 * s.db[822][1]));
        let eq76_e1981_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq76_e1979 * s.db[822][2]));
        let eq76_e1981_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq76_e1979 * s.db[822][3]));
        let eq76_e1981_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq76_e1979 * s.db[822][4]));
        let eq76_e1981_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq76_e1979 * s.db[822][5]));
        let eq76_e1981_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq76_e1979 * s.db[822][6]));
        let eq76_e1981_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq76_e1979 * s.db[822][7]));
        let eq76_e1981_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq76_e1979 * s.db[822][8]));
        let eq76_e1981_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq76_e1979 * s.db[822][9]));
        let eq76_e1981_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq76_e1979 * s.db[822][10]));
        let eq76_e1981_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq76_e1979 * s.db[822][11]));
        let eq76_e1981_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq76_e1979 * s.db[822][12]));
        let eq76_e1981_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq76_e1979 * s.db[822][13]));
        let eq76_e1981_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq76_e1979 * s.db[822][14]));
        let eq76_e1981_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq76_e1979 * s.db[822][15]));
        let eq76_e1981_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq76_e1979 * s.db[822][16]));
        let eq76_e1981_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq76_e1979 * s.db[822][17]));
        let eq76_e1984: f64 = (s.v[410] * s.v[158]);
        let eq76_e1985_q: f64 = eq76_e1984;
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1984);
        let eq76_e1986_d_n0: f64 = (eq76_e1981_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq76_e1986_d_n1: f64 = (eq76_e1981_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq76_e1986_d_n2: f64 = (eq76_e1981_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq76_e1986_d_n13: f64 = (eq76_e1981_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq76_e1986_d_b0: f64 = (eq76_e1981_d_b0 + (s.db[410][0] * s.v[158]));
        let eq76_e1986_d_b1: f64 = (eq76_e1981_d_b1 + (s.db[410][1] * s.v[158]));
        let eq76_e1986_d_b2: f64 = (eq76_e1981_d_b2 + (s.db[410][2] * s.v[158]));
        let eq76_e1986_d_b3: f64 = (eq76_e1981_d_b3 + (s.db[410][3] * s.v[158]));
        let eq76_e1986_d_b4: f64 = (eq76_e1981_d_b4 + (s.db[410][4] * s.v[158]));
        let eq76_e1986_d_b5: f64 = (eq76_e1981_d_b5 + (s.db[410][5] * s.v[158]));
        let eq76_e1986_d_b6: f64 = (eq76_e1981_d_b6 + (s.db[410][6] * s.v[158]));
        let eq76_e1986_d_b7: f64 = (eq76_e1981_d_b7 + (s.db[410][7] * s.v[158]));
        let eq76_e1986_d_b8: f64 = (eq76_e1981_d_b8 + (s.db[410][8] * s.v[158]));
        let eq76_e1986_d_b9: f64 = (eq76_e1981_d_b9 + (s.db[410][9] * s.v[158]));
        let eq76_e1986_d_b10: f64 = (eq76_e1981_d_b10 + (s.db[410][10] * s.v[158]));
        let eq76_e1986_d_b11: f64 = (eq76_e1981_d_b11 + (s.db[410][11] * s.v[158]));
        let eq76_e1986_d_b12: f64 = (eq76_e1981_d_b12 + (s.db[410][12] * s.v[158]));
        let eq76_e1986_d_b13: f64 = (eq76_e1981_d_b13 + (s.db[410][13] * s.v[158]));
        let eq76_e1986_d_b14: f64 = (eq76_e1981_d_b14 + (s.db[410][14] * s.v[158]));
        let eq76_e1986_d_b15: f64 = (eq76_e1981_d_b15 + (s.db[410][15] * s.v[158]));
        let eq76_e1986_d_b16: f64 = (eq76_e1981_d_b16 + (s.db[410][16] * s.v[158]));
        let eq76_e1986_d_b17: f64 = (eq76_e1981_d_b17 + (s.db[410][17] * s.v[158]));
        let eq76_e1986_q: f64 = eq76_e1985_q;
        let __rspice_inv_cse_2: f64 = 1.0 / s.v[157];
        let eq76_e1989: f64 = (s.v[410] * __rspice_inv_cse_2);
        let eq76_e1989_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_2);
        let eq76_e1989_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_2);
        let eq76_e1989_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_2);
        let eq76_e1989_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_2);
        let eq76_e1989_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_2);
        let eq76_e1989_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_2);
        let eq76_e1989_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_2);
        let eq76_e1989_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_2);
        let eq76_e1989_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_2);
        let eq76_e1989_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_2);
        let eq76_e1989_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_2);
        let eq76_e1989_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_2);
        let eq76_e1989_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_2);
        let eq76_e1989_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_2);
        let eq76_e1989_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_2);
        let eq76_e1989_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_2);
        let eq76_e1989_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_2);
        let eq76_e1989_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_2);
        let eq76_e1989_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_2);
        let eq76_e1989_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_2);
        let eq76_e1989_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_2);
        let eq76_e1989_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_2);
        let eq76_e1989_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_2);
        let eq76_e1989_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_2);
        let eq76_e1989_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_2);
        let eq76_e1989_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_2);
        let eq76_e1989_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_2);
        let eq76_e1989_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_2);
        let eq76_e1989_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_2);
        let eq76_e1989_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_2);
        let eq76_e1989_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_2);
        let eq76_e1989_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_2);
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n0: f64 = (eq76_e1986_d_n0 + eq76_e1989_d_n0);
        let eq76_e1990_d_n1: f64 = (eq76_e1986_d_n1 + eq76_e1989_d_n1);
        let eq76_e1990_d_n2: f64 = (eq76_e1986_d_n2 + eq76_e1989_d_n2);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        let eq76_e1990_d_n13: f64 = (eq76_e1986_d_n13 + eq76_e1989_d_n13);
        let eq76_e1990_d_b0: f64 = (eq76_e1986_d_b0 + eq76_e1989_d_b0);
        let eq76_e1990_d_b1: f64 = (eq76_e1986_d_b1 + eq76_e1989_d_b1);
        let eq76_e1990_d_b2: f64 = (eq76_e1986_d_b2 + eq76_e1989_d_b2);
        let eq76_e1990_d_b3: f64 = (eq76_e1986_d_b3 + eq76_e1989_d_b3);
        let eq76_e1990_d_b4: f64 = (eq76_e1986_d_b4 + eq76_e1989_d_b4);
        let eq76_e1990_d_b5: f64 = (eq76_e1986_d_b5 + eq76_e1989_d_b5);
        let eq76_e1990_d_b6: f64 = (eq76_e1986_d_b6 + eq76_e1989_d_b6);
        let eq76_e1990_d_b7: f64 = (eq76_e1986_d_b7 + eq76_e1989_d_b7);
        let eq76_e1990_d_b8: f64 = (eq76_e1986_d_b8 + eq76_e1989_d_b8);
        let eq76_e1990_d_b9: f64 = (eq76_e1986_d_b9 + eq76_e1989_d_b9);
        let eq76_e1990_d_b10: f64 = (eq76_e1986_d_b10 + eq76_e1989_d_b10);
        let eq76_e1990_d_b11: f64 = (eq76_e1986_d_b11 + eq76_e1989_d_b11);
        let eq76_e1990_d_b12: f64 = (eq76_e1986_d_b12 + eq76_e1989_d_b12);
        let eq76_e1990_d_b13: f64 = (eq76_e1986_d_b13 + eq76_e1989_d_b13);
        let eq76_e1990_d_b14: f64 = (eq76_e1986_d_b14 + eq76_e1989_d_b14);
        let eq76_e1990_d_b15: f64 = (eq76_e1986_d_b15 + eq76_e1989_d_b15);
        let eq76_e1990_d_b16: f64 = (eq76_e1986_d_b16 + eq76_e1989_d_b16);
        let eq76_e1990_d_b17: f64 = (eq76_e1986_d_b17 + eq76_e1989_d_b17);
        let eq76_e1990_q: f64 = eq76_e1986_q;
        (eq76_e1990, eq76_e1990_d_n0, eq76_e1990_d_n1, eq76_e1990_d_n2, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_d_n13, eq76_e1990_d_b0, eq76_e1990_d_b1, eq76_e1990_d_b2, eq76_e1990_d_b3, eq76_e1990_d_b4, eq76_e1990_d_b5, eq76_e1990_d_b6, eq76_e1990_d_b7, eq76_e1990_d_b8, eq76_e1990_d_b9, eq76_e1990_d_b10, eq76_e1990_d_b11, eq76_e1990_d_b12, eq76_e1990_d_b13, eq76_e1990_d_b14, eq76_e1990_d_b15, eq76_e1990_d_b16, eq76_e1990_d_b17, eq76_e1990_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 14] = [eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13];
        let eq76_reactive_branch_derivatives: [f64; 18] = [eq76_e1992_q_d_b0, eq76_e1992_q_d_b1, eq76_e1992_q_d_b2, eq76_e1992_q_d_b3, eq76_e1992_q_d_b4, eq76_e1992_q_d_b5, eq76_e1992_q_d_b6, eq76_e1992_q_d_b7, eq76_e1992_q_d_b8, eq76_e1992_q_d_b9, eq76_e1992_q_d_b10, eq76_e1992_q_d_b11, eq76_e1992_q_d_b12, eq76_e1992_q_d_b13, eq76_e1992_q_d_b14, eq76_e1992_q_d_b15, eq76_e1992_q_d_b16, eq76_e1992_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

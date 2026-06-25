#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_51_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq51_e1694, eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13, eq51_e1694_q, eq51_e1694_q_d_n0, eq51_e1694_q_d_n1, eq51_e1694_q_d_n2, eq51_e1694_q_d_n3, eq51_e1694_q_d_n4, eq51_e1694_q_d_n5, eq51_e1694_q_d_n6, eq51_e1694_q_d_n7, eq51_e1694_q_d_n8, eq51_e1694_q_d_n9, eq51_e1694_q_d_n10, eq51_e1694_q_d_n11, eq51_e1694_q_d_n12, eq51_e1694_q_d_n13,) = {
    if (s.v[1553] != 0.0) {
        let eq51_e1690: f64 = (p.p33 * s.v[896]);
        let eq51_e1690_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq51_e1690_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq51_e1690_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq51_e1690_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq51_e1690_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq51_e1690_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq51_e1690_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq51_e1690_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq51_e1690_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq51_e1690_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq51_e1690_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq51_e1690_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq51_e1690_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq51_e1690_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq51_e1691_q: f64 = eq51_e1690;
        let eq51_e1692: f64 = (p.p37 * eq51_e1690);
        let eq51_e1692_d_n0: f64 = (p.p37 * eq51_e1690_d_n0);
        let eq51_e1692_d_n1: f64 = (p.p37 * eq51_e1690_d_n1);
        let eq51_e1692_d_n2: f64 = (p.p37 * eq51_e1690_d_n2);
        let eq51_e1692_d_n3: f64 = (p.p37 * eq51_e1690_d_n3);
        let eq51_e1692_d_n4: f64 = (p.p37 * eq51_e1690_d_n4);
        let eq51_e1692_d_n5: f64 = (p.p37 * eq51_e1690_d_n5);
        let eq51_e1692_d_n6: f64 = (p.p37 * eq51_e1690_d_n6);
        let eq51_e1692_d_n7: f64 = (p.p37 * eq51_e1690_d_n7);
        let eq51_e1692_d_n8: f64 = (p.p37 * eq51_e1690_d_n8);
        let eq51_e1692_d_n9: f64 = (p.p37 * eq51_e1690_d_n9);
        let eq51_e1692_d_n10: f64 = (p.p37 * eq51_e1690_d_n10);
        let eq51_e1692_d_n11: f64 = (p.p37 * eq51_e1690_d_n11);
        let eq51_e1692_d_n12: f64 = (p.p37 * eq51_e1690_d_n12);
        let eq51_e1692_d_n13: f64 = (p.p37 * eq51_e1690_d_n13);
        let eq51_e1692_q: f64 = (p.p37 * eq51_e1691_q);
        let eq51_e1692_q_d_n0: f64 = (p.p37 * eq51_e1690_d_n0);
        let eq51_e1692_q_d_n1: f64 = (p.p37 * eq51_e1690_d_n1);
        let eq51_e1692_q_d_n2: f64 = (p.p37 * eq51_e1690_d_n2);
        let eq51_e1692_q_d_n3: f64 = (p.p37 * eq51_e1690_d_n3);
        let eq51_e1692_q_d_n4: f64 = (p.p37 * eq51_e1690_d_n4);
        let eq51_e1692_q_d_n5: f64 = (p.p37 * eq51_e1690_d_n5);
        let eq51_e1692_q_d_n6: f64 = (p.p37 * eq51_e1690_d_n6);
        let eq51_e1692_q_d_n7: f64 = (p.p37 * eq51_e1690_d_n7);
        let eq51_e1692_q_d_n8: f64 = (p.p37 * eq51_e1690_d_n8);
        let eq51_e1692_q_d_n9: f64 = (p.p37 * eq51_e1690_d_n9);
        let eq51_e1692_q_d_n10: f64 = (p.p37 * eq51_e1690_d_n10);
        let eq51_e1692_q_d_n11: f64 = (p.p37 * eq51_e1690_d_n11);
        let eq51_e1692_q_d_n12: f64 = (p.p37 * eq51_e1690_d_n12);
        let eq51_e1692_q_d_n13: f64 = (p.p37 * eq51_e1690_d_n13);
        (eq51_e1692, eq51_e1692_d_n0, eq51_e1692_d_n1, eq51_e1692_d_n2, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12, eq51_e1692_d_n13, eq51_e1692_q, eq51_e1692_q_d_n0, eq51_e1692_q_d_n1, eq51_e1692_q_d_n2, eq51_e1692_q_d_n3, eq51_e1692_q_d_n4, eq51_e1692_q_d_n5, eq51_e1692_q_d_n6, eq51_e1692_q_d_n7, eq51_e1692_q_d_n8, eq51_e1692_q_d_n9, eq51_e1692_q_d_n10, eq51_e1692_q_d_n11, eq51_e1692_q_d_n12, eq51_e1692_q_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 14] = [eq51_e1694_q_d_n0, eq51_e1694_q_d_n1, eq51_e1694_q_d_n2, eq51_e1694_q_d_n3, eq51_e1694_q_d_n4, eq51_e1694_q_d_n5, eq51_e1694_q_d_n6, eq51_e1694_q_d_n7, eq51_e1694_q_d_n8, eq51_e1694_q_d_n9, eq51_e1694_q_d_n10, eq51_e1694_q_d_n11, eq51_e1694_q_d_n12, eq51_e1694_q_d_n13];
        let eq51_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            &nodes,
            &eq51_reactive_node_derivatives,
            &branches,
            &eq51_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_52_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq52_e1703, eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13, eq52_e1703_q, eq52_e1703_q_d_n0, eq52_e1703_q_d_n1, eq52_e1703_q_d_n2, eq52_e1703_q_d_n3, eq52_e1703_q_d_n4, eq52_e1703_q_d_n5, eq52_e1703_q_d_n6, eq52_e1703_q_d_n7, eq52_e1703_q_d_n8, eq52_e1703_q_d_n9, eq52_e1703_q_d_n10, eq52_e1703_q_d_n11, eq52_e1703_q_d_n12, eq52_e1703_q_d_n13,) = {
    if (s.v[1553] != 0.0) {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1698_d_n3: f64 = (-p.p33);
        let eq52_e1698_d_n10: f64 = p.p33;
        let eq52_e1700: f64 = (eq52_e1698 * s.v[336]);
        let eq52_e1700_d_n0: f64 = (eq52_e1698 * s.dn[336][0]);
        let eq52_e1700_d_n1: f64 = (eq52_e1698 * s.dn[336][1]);
        let eq52_e1700_d_n2: f64 = (eq52_e1698 * s.dn[336][2]);
        let eq52_e1700_d_n3: f64 = ((eq52_e1698_d_n3 * s.v[336]) + (eq52_e1698 * s.dn[336][3]));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * s.dn[336][4]);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * s.dn[336][5]);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * s.dn[336][6]);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * s.dn[336][7]);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * s.dn[336][8]);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * s.dn[336][9]);
        let eq52_e1700_d_n10: f64 = ((eq52_e1698_d_n10 * s.v[336]) + (eq52_e1698 * s.dn[336][10]));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * s.dn[336][11]);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * s.dn[336][12]);
        let eq52_e1700_d_n13: f64 = (eq52_e1698 * s.dn[336][13]);
        let eq52_e1701_q: f64 = eq52_e1700;
        (eq52_e1700, eq52_e1700_d_n0, eq52_e1700_d_n1, eq52_e1700_d_n2, eq52_e1700_d_n3, eq52_e1700_d_n4, eq52_e1700_d_n5, eq52_e1700_d_n6, eq52_e1700_d_n7, eq52_e1700_d_n8, eq52_e1700_d_n9, eq52_e1700_d_n10, eq52_e1700_d_n11, eq52_e1700_d_n12, eq52_e1700_d_n13, eq52_e1701_q, eq52_e1700_d_n0, eq52_e1700_d_n1, eq52_e1700_d_n2, eq52_e1700_d_n3, eq52_e1700_d_n4, eq52_e1700_d_n5, eq52_e1700_d_n6, eq52_e1700_d_n7, eq52_e1700_d_n8, eq52_e1700_d_n9, eq52_e1700_d_n10, eq52_e1700_d_n11, eq52_e1700_d_n12, eq52_e1700_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 14] = [eq52_e1703_q_d_n0, eq52_e1703_q_d_n1, eq52_e1703_q_d_n2, eq52_e1703_q_d_n3, eq52_e1703_q_d_n4, eq52_e1703_q_d_n5, eq52_e1703_q_d_n6, eq52_e1703_q_d_n7, eq52_e1703_q_d_n8, eq52_e1703_q_d_n9, eq52_e1703_q_d_n10, eq52_e1703_q_d_n11, eq52_e1703_q_d_n12, eq52_e1703_q_d_n13];
        let eq52_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            &nodes,
            &eq52_reactive_node_derivatives,
            &branches,
            &eq52_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_53_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq53_e1713, eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13, eq53_e1713_q, eq53_e1713_q_d_n0, eq53_e1713_q_d_n1, eq53_e1713_q_d_n2, eq53_e1713_q_d_n3, eq53_e1713_q_d_n4, eq53_e1713_q_d_n5, eq53_e1713_q_d_n6, eq53_e1713_q_d_n7, eq53_e1713_q_d_n8, eq53_e1713_q_d_n9, eq53_e1713_q_d_n10, eq53_e1713_q_d_n11, eq53_e1713_q_d_n12, eq53_e1713_q_d_n13,) = {
    if (!(s.v[1553] != 0.0)) {
        let eq53_e1709: f64 = (p.p33 * s.v[895]);
        let eq53_e1709_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq53_e1709_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq53_e1709_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq53_e1709_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq53_e1709_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq53_e1709_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq53_e1709_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq53_e1709_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq53_e1709_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq53_e1709_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq53_e1709_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq53_e1709_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq53_e1709_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq53_e1709_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq53_e1710_q: f64 = eq53_e1709;
        let eq53_e1711: f64 = (p.p37 * eq53_e1709);
        let eq53_e1711_d_n0: f64 = (p.p37 * eq53_e1709_d_n0);
        let eq53_e1711_d_n1: f64 = (p.p37 * eq53_e1709_d_n1);
        let eq53_e1711_d_n2: f64 = (p.p37 * eq53_e1709_d_n2);
        let eq53_e1711_d_n3: f64 = (p.p37 * eq53_e1709_d_n3);
        let eq53_e1711_d_n4: f64 = (p.p37 * eq53_e1709_d_n4);
        let eq53_e1711_d_n5: f64 = (p.p37 * eq53_e1709_d_n5);
        let eq53_e1711_d_n6: f64 = (p.p37 * eq53_e1709_d_n6);
        let eq53_e1711_d_n7: f64 = (p.p37 * eq53_e1709_d_n7);
        let eq53_e1711_d_n8: f64 = (p.p37 * eq53_e1709_d_n8);
        let eq53_e1711_d_n9: f64 = (p.p37 * eq53_e1709_d_n9);
        let eq53_e1711_d_n10: f64 = (p.p37 * eq53_e1709_d_n10);
        let eq53_e1711_d_n11: f64 = (p.p37 * eq53_e1709_d_n11);
        let eq53_e1711_d_n12: f64 = (p.p37 * eq53_e1709_d_n12);
        let eq53_e1711_d_n13: f64 = (p.p37 * eq53_e1709_d_n13);
        let eq53_e1711_q: f64 = (p.p37 * eq53_e1710_q);
        let eq53_e1711_q_d_n0: f64 = (p.p37 * eq53_e1709_d_n0);
        let eq53_e1711_q_d_n1: f64 = (p.p37 * eq53_e1709_d_n1);
        let eq53_e1711_q_d_n2: f64 = (p.p37 * eq53_e1709_d_n2);
        let eq53_e1711_q_d_n3: f64 = (p.p37 * eq53_e1709_d_n3);
        let eq53_e1711_q_d_n4: f64 = (p.p37 * eq53_e1709_d_n4);
        let eq53_e1711_q_d_n5: f64 = (p.p37 * eq53_e1709_d_n5);
        let eq53_e1711_q_d_n6: f64 = (p.p37 * eq53_e1709_d_n6);
        let eq53_e1711_q_d_n7: f64 = (p.p37 * eq53_e1709_d_n7);
        let eq53_e1711_q_d_n8: f64 = (p.p37 * eq53_e1709_d_n8);
        let eq53_e1711_q_d_n9: f64 = (p.p37 * eq53_e1709_d_n9);
        let eq53_e1711_q_d_n10: f64 = (p.p37 * eq53_e1709_d_n10);
        let eq53_e1711_q_d_n11: f64 = (p.p37 * eq53_e1709_d_n11);
        let eq53_e1711_q_d_n12: f64 = (p.p37 * eq53_e1709_d_n12);
        let eq53_e1711_q_d_n13: f64 = (p.p37 * eq53_e1709_d_n13);
        (eq53_e1711, eq53_e1711_d_n0, eq53_e1711_d_n1, eq53_e1711_d_n2, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_d_n13, eq53_e1711_q, eq53_e1711_q_d_n0, eq53_e1711_q_d_n1, eq53_e1711_q_d_n2, eq53_e1711_q_d_n3, eq53_e1711_q_d_n4, eq53_e1711_q_d_n5, eq53_e1711_q_d_n6, eq53_e1711_q_d_n7, eq53_e1711_q_d_n8, eq53_e1711_q_d_n9, eq53_e1711_q_d_n10, eq53_e1711_q_d_n11, eq53_e1711_q_d_n12, eq53_e1711_q_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 14] = [eq53_e1713_q_d_n0, eq53_e1713_q_d_n1, eq53_e1713_q_d_n2, eq53_e1713_q_d_n3, eq53_e1713_q_d_n4, eq53_e1713_q_d_n5, eq53_e1713_q_d_n6, eq53_e1713_q_d_n7, eq53_e1713_q_d_n8, eq53_e1713_q_d_n9, eq53_e1713_q_d_n10, eq53_e1713_q_d_n11, eq53_e1713_q_d_n12, eq53_e1713_q_d_n13];
        let eq53_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq53_reactive_node_derivatives,
            &branches,
            &eq53_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_54_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq54_e1723, eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13, eq54_e1723_q, eq54_e1723_q_d_n0, eq54_e1723_q_d_n1, eq54_e1723_q_d_n2, eq54_e1723_q_d_n3, eq54_e1723_q_d_n4, eq54_e1723_q_d_n5, eq54_e1723_q_d_n6, eq54_e1723_q_d_n7, eq54_e1723_q_d_n8, eq54_e1723_q_d_n9, eq54_e1723_q_d_n10, eq54_e1723_q_d_n11, eq54_e1723_q_d_n12, eq54_e1723_q_d_n13,) = {
    if (!(s.v[1553] != 0.0)) {
        let eq54_e1719: f64 = (p.p33 * s.v[896]);
        let eq54_e1719_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq54_e1719_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq54_e1719_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq54_e1719_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq54_e1719_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq54_e1719_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq54_e1719_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq54_e1719_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq54_e1719_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq54_e1719_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq54_e1719_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq54_e1719_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq54_e1719_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq54_e1719_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq54_e1720_q: f64 = eq54_e1719;
        let eq54_e1721: f64 = (p.p37 * eq54_e1719);
        let eq54_e1721_d_n0: f64 = (p.p37 * eq54_e1719_d_n0);
        let eq54_e1721_d_n1: f64 = (p.p37 * eq54_e1719_d_n1);
        let eq54_e1721_d_n2: f64 = (p.p37 * eq54_e1719_d_n2);
        let eq54_e1721_d_n3: f64 = (p.p37 * eq54_e1719_d_n3);
        let eq54_e1721_d_n4: f64 = (p.p37 * eq54_e1719_d_n4);
        let eq54_e1721_d_n5: f64 = (p.p37 * eq54_e1719_d_n5);
        let eq54_e1721_d_n6: f64 = (p.p37 * eq54_e1719_d_n6);
        let eq54_e1721_d_n7: f64 = (p.p37 * eq54_e1719_d_n7);
        let eq54_e1721_d_n8: f64 = (p.p37 * eq54_e1719_d_n8);
        let eq54_e1721_d_n9: f64 = (p.p37 * eq54_e1719_d_n9);
        let eq54_e1721_d_n10: f64 = (p.p37 * eq54_e1719_d_n10);
        let eq54_e1721_d_n11: f64 = (p.p37 * eq54_e1719_d_n11);
        let eq54_e1721_d_n12: f64 = (p.p37 * eq54_e1719_d_n12);
        let eq54_e1721_d_n13: f64 = (p.p37 * eq54_e1719_d_n13);
        let eq54_e1721_q: f64 = (p.p37 * eq54_e1720_q);
        let eq54_e1721_q_d_n0: f64 = (p.p37 * eq54_e1719_d_n0);
        let eq54_e1721_q_d_n1: f64 = (p.p37 * eq54_e1719_d_n1);
        let eq54_e1721_q_d_n2: f64 = (p.p37 * eq54_e1719_d_n2);
        let eq54_e1721_q_d_n3: f64 = (p.p37 * eq54_e1719_d_n3);
        let eq54_e1721_q_d_n4: f64 = (p.p37 * eq54_e1719_d_n4);
        let eq54_e1721_q_d_n5: f64 = (p.p37 * eq54_e1719_d_n5);
        let eq54_e1721_q_d_n6: f64 = (p.p37 * eq54_e1719_d_n6);
        let eq54_e1721_q_d_n7: f64 = (p.p37 * eq54_e1719_d_n7);
        let eq54_e1721_q_d_n8: f64 = (p.p37 * eq54_e1719_d_n8);
        let eq54_e1721_q_d_n9: f64 = (p.p37 * eq54_e1719_d_n9);
        let eq54_e1721_q_d_n10: f64 = (p.p37 * eq54_e1719_d_n10);
        let eq54_e1721_q_d_n11: f64 = (p.p37 * eq54_e1719_d_n11);
        let eq54_e1721_q_d_n12: f64 = (p.p37 * eq54_e1719_d_n12);
        let eq54_e1721_q_d_n13: f64 = (p.p37 * eq54_e1719_d_n13);
        (eq54_e1721, eq54_e1721_d_n0, eq54_e1721_d_n1, eq54_e1721_d_n2, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_d_n13, eq54_e1721_q, eq54_e1721_q_d_n0, eq54_e1721_q_d_n1, eq54_e1721_q_d_n2, eq54_e1721_q_d_n3, eq54_e1721_q_d_n4, eq54_e1721_q_d_n5, eq54_e1721_q_d_n6, eq54_e1721_q_d_n7, eq54_e1721_q_d_n8, eq54_e1721_q_d_n9, eq54_e1721_q_d_n10, eq54_e1721_q_d_n11, eq54_e1721_q_d_n12, eq54_e1721_q_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 14] = [eq54_e1723_q_d_n0, eq54_e1723_q_d_n1, eq54_e1723_q_d_n2, eq54_e1723_q_d_n3, eq54_e1723_q_d_n4, eq54_e1723_q_d_n5, eq54_e1723_q_d_n6, eq54_e1723_q_d_n7, eq54_e1723_q_d_n8, eq54_e1723_q_d_n9, eq54_e1723_q_d_n10, eq54_e1723_q_d_n11, eq54_e1723_q_d_n12, eq54_e1723_q_d_n13];
        let eq54_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            &nodes,
            &eq54_reactive_node_derivatives,
            &branches,
            &eq54_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq55_e1733, eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13, eq55_e1733_q, eq55_e1733_q_d_n0, eq55_e1733_q_d_n1, eq55_e1733_q_d_n2, eq55_e1733_q_d_n3, eq55_e1733_q_d_n4, eq55_e1733_q_d_n5, eq55_e1733_q_d_n6, eq55_e1733_q_d_n7, eq55_e1733_q_d_n8, eq55_e1733_q_d_n9, eq55_e1733_q_d_n10, eq55_e1733_q_d_n11, eq55_e1733_q_d_n12, eq55_e1733_q_d_n13,) = {
    if (!(s.v[1553] != 0.0)) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1728_d_n3: f64 = (-p.p33);
        let eq55_e1728_d_n9: f64 = p.p33;
        let eq55_e1730: f64 = (eq55_e1728 * s.v[336]);
        let eq55_e1730_d_n0: f64 = (eq55_e1728 * s.dn[336][0]);
        let eq55_e1730_d_n1: f64 = (eq55_e1728 * s.dn[336][1]);
        let eq55_e1730_d_n2: f64 = (eq55_e1728 * s.dn[336][2]);
        let eq55_e1730_d_n3: f64 = ((eq55_e1728_d_n3 * s.v[336]) + (eq55_e1728 * s.dn[336][3]));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * s.dn[336][4]);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * s.dn[336][5]);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * s.dn[336][6]);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * s.dn[336][7]);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * s.dn[336][8]);
        let eq55_e1730_d_n9: f64 = ((eq55_e1728_d_n9 * s.v[336]) + (eq55_e1728 * s.dn[336][9]));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * s.dn[336][10]);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * s.dn[336][11]);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * s.dn[336][12]);
        let eq55_e1730_d_n13: f64 = (eq55_e1728 * s.dn[336][13]);
        let eq55_e1731_q: f64 = eq55_e1730;
        (eq55_e1730, eq55_e1730_d_n0, eq55_e1730_d_n1, eq55_e1730_d_n2, eq55_e1730_d_n3, eq55_e1730_d_n4, eq55_e1730_d_n5, eq55_e1730_d_n6, eq55_e1730_d_n7, eq55_e1730_d_n8, eq55_e1730_d_n9, eq55_e1730_d_n10, eq55_e1730_d_n11, eq55_e1730_d_n12, eq55_e1730_d_n13, eq55_e1731_q, eq55_e1730_d_n0, eq55_e1730_d_n1, eq55_e1730_d_n2, eq55_e1730_d_n3, eq55_e1730_d_n4, eq55_e1730_d_n5, eq55_e1730_d_n6, eq55_e1730_d_n7, eq55_e1730_d_n8, eq55_e1730_d_n9, eq55_e1730_d_n10, eq55_e1730_d_n11, eq55_e1730_d_n12, eq55_e1730_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 14] = [eq55_e1733_q_d_n0, eq55_e1733_q_d_n1, eq55_e1733_q_d_n2, eq55_e1733_q_d_n3, eq55_e1733_q_d_n4, eq55_e1733_q_d_n5, eq55_e1733_q_d_n6, eq55_e1733_q_d_n7, eq55_e1733_q_d_n8, eq55_e1733_q_d_n9, eq55_e1733_q_d_n10, eq55_e1733_q_d_n11, eq55_e1733_q_d_n12, eq55_e1733_q_d_n13];
        let eq55_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            &nodes,
            &eq55_reactive_node_derivatives,
            &branches,
            &eq55_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq56_e1736: f64 = (p.p33 * s.v[87]);
        let eq56_e1736_d_n0: f64 = (p.p33 * s.dn[87][0]);
        let eq56_e1736_d_n1: f64 = (p.p33 * s.dn[87][1]);
        let eq56_e1736_d_n2: f64 = (p.p33 * s.dn[87][2]);
        let eq56_e1736_d_n3: f64 = (p.p33 * s.dn[87][3]);
        let eq56_e1736_d_n4: f64 = (p.p33 * s.dn[87][4]);
        let eq56_e1736_d_n5: f64 = (p.p33 * s.dn[87][5]);
        let eq56_e1736_d_n6: f64 = (p.p33 * s.dn[87][6]);
        let eq56_e1736_d_n7: f64 = (p.p33 * s.dn[87][7]);
        let eq56_e1736_d_n8: f64 = (p.p33 * s.dn[87][8]);
        let eq56_e1736_d_n9: f64 = (p.p33 * s.dn[87][9]);
        let eq56_e1736_d_n10: f64 = (p.p33 * s.dn[87][10]);
        let eq56_e1736_d_n11: f64 = (p.p33 * s.dn[87][11]);
        let eq56_e1736_d_n12: f64 = (p.p33 * s.dn[87][12]);
        let eq56_e1736_d_n13: f64 = (p.p33 * s.dn[87][13]);
        let eq56_e1737_q: f64 = eq56_e1736;
        let eq56_reactive_node_derivatives: [f64; 14] = [eq56_e1736_d_n0, eq56_e1736_d_n1, eq56_e1736_d_n2, eq56_e1736_d_n3, eq56_e1736_d_n4, eq56_e1736_d_n5, eq56_e1736_d_n6, eq56_e1736_d_n7, eq56_e1736_d_n8, eq56_e1736_d_n9, eq56_e1736_d_n10, eq56_e1736_d_n11, eq56_e1736_d_n12, eq56_e1736_d_n13];
        let eq56_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            &nodes,
            &eq56_reactive_node_derivatives,
            &branches,
            &eq56_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_57_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq57_e1740: f64 = (p.p33 * s.v[86]);
        let eq57_e1740_d_n0: f64 = (p.p33 * s.dn[86][0]);
        let eq57_e1740_d_n1: f64 = (p.p33 * s.dn[86][1]);
        let eq57_e1740_d_n2: f64 = (p.p33 * s.dn[86][2]);
        let eq57_e1740_d_n3: f64 = (p.p33 * s.dn[86][3]);
        let eq57_e1740_d_n4: f64 = (p.p33 * s.dn[86][4]);
        let eq57_e1740_d_n5: f64 = (p.p33 * s.dn[86][5]);
        let eq57_e1740_d_n6: f64 = (p.p33 * s.dn[86][6]);
        let eq57_e1740_d_n7: f64 = (p.p33 * s.dn[86][7]);
        let eq57_e1740_d_n8: f64 = (p.p33 * s.dn[86][8]);
        let eq57_e1740_d_n9: f64 = (p.p33 * s.dn[86][9]);
        let eq57_e1740_d_n10: f64 = (p.p33 * s.dn[86][10]);
        let eq57_e1740_d_n11: f64 = (p.p33 * s.dn[86][11]);
        let eq57_e1740_d_n12: f64 = (p.p33 * s.dn[86][12]);
        let eq57_e1740_d_n13: f64 = (p.p33 * s.dn[86][13]);
        let eq57_e1741_q: f64 = eq57_e1740;
        let eq57_reactive_node_derivatives: [f64; 14] = [eq57_e1740_d_n0, eq57_e1740_d_n1, eq57_e1740_d_n2, eq57_e1740_d_n3, eq57_e1740_d_n4, eq57_e1740_d_n5, eq57_e1740_d_n6, eq57_e1740_d_n7, eq57_e1740_d_n8, eq57_e1740_d_n9, eq57_e1740_d_n10, eq57_e1740_d_n11, eq57_e1740_d_n12, eq57_e1740_d_n13];
        let eq57_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            &nodes,
            &eq57_reactive_node_derivatives,
            &branches,
            &eq57_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq71_e1869, eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13, eq71_e1869_q, eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13,) = {
    if (((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (s.v[1561] != 0.0)) {
        let eq71_e1856: f64 = (-s.v[885]);
        let eq71_e1856_d_n0: f64 = (-s.dn[885][0]);
        let eq71_e1856_d_n1: f64 = (-s.dn[885][1]);
        let eq71_e1856_d_n2: f64 = (-s.dn[885][2]);
        let eq71_e1856_d_n3: f64 = (-s.dn[885][3]);
        let eq71_e1856_d_n4: f64 = (-s.dn[885][4]);
        let eq71_e1856_d_n5: f64 = (-s.dn[885][5]);
        let eq71_e1856_d_n6: f64 = (-s.dn[885][6]);
        let eq71_e1856_d_n7: f64 = (-s.dn[885][7]);
        let eq71_e1856_d_n8: f64 = (-s.dn[885][8]);
        let eq71_e1856_d_n9: f64 = (-s.dn[885][9]);
        let eq71_e1856_d_n10: f64 = (-s.dn[885][10]);
        let eq71_e1856_d_n11: f64 = (-s.dn[885][11]);
        let eq71_e1856_d_n12: f64 = (-s.dn[885][12]);
        let eq71_e1856_d_n13: f64 = (-s.dn[885][13]);
        let eq71_e1858: f64 = (eq71_e1856 * s.v[822]);
        let eq71_e1858_d_n0: f64 = ((eq71_e1856_d_n0 * s.v[822]) + (eq71_e1856 * s.dn[822][0]));
        let eq71_e1858_d_n1: f64 = ((eq71_e1856_d_n1 * s.v[822]) + (eq71_e1856 * s.dn[822][1]));
        let eq71_e1858_d_n2: f64 = ((eq71_e1856_d_n2 * s.v[822]) + (eq71_e1856 * s.dn[822][2]));
        let eq71_e1858_d_n3: f64 = ((eq71_e1856_d_n3 * s.v[822]) + (eq71_e1856 * s.dn[822][3]));
        let eq71_e1858_d_n4: f64 = ((eq71_e1856_d_n4 * s.v[822]) + (eq71_e1856 * s.dn[822][4]));
        let eq71_e1858_d_n5: f64 = ((eq71_e1856_d_n5 * s.v[822]) + (eq71_e1856 * s.dn[822][5]));
        let eq71_e1858_d_n6: f64 = ((eq71_e1856_d_n6 * s.v[822]) + (eq71_e1856 * s.dn[822][6]));
        let eq71_e1858_d_n7: f64 = ((eq71_e1856_d_n7 * s.v[822]) + (eq71_e1856 * s.dn[822][7]));
        let eq71_e1858_d_n8: f64 = ((eq71_e1856_d_n8 * s.v[822]) + (eq71_e1856 * s.dn[822][8]));
        let eq71_e1858_d_n9: f64 = ((eq71_e1856_d_n9 * s.v[822]) + (eq71_e1856 * s.dn[822][9]));
        let eq71_e1858_d_n10: f64 = ((eq71_e1856_d_n10 * s.v[822]) + (eq71_e1856 * s.dn[822][10]));
        let eq71_e1858_d_n11: f64 = ((eq71_e1856_d_n11 * s.v[822]) + (eq71_e1856 * s.dn[822][11]));
        let eq71_e1858_d_n12: f64 = ((eq71_e1856_d_n12 * s.v[822]) + (eq71_e1856 * s.dn[822][12]));
        let eq71_e1858_d_n13: f64 = ((eq71_e1856_d_n13 * s.v[822]) + (eq71_e1856 * s.dn[822][13]));
        let eq71_e1861: f64 = (s.v[410] * s.v[158]);
        let eq71_e1861_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq71_e1861_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq71_e1861_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq71_e1861_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq71_e1861_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq71_e1861_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq71_e1861_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq71_e1861_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq71_e1861_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq71_e1861_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq71_e1861_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq71_e1861_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq71_e1861_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq71_e1861_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq71_e1862_q: f64 = eq71_e1861;
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1861);
        let eq71_e1863_d_n0: f64 = (eq71_e1858_d_n0 + eq71_e1861_d_n0);
        let eq71_e1863_d_n1: f64 = (eq71_e1858_d_n1 + eq71_e1861_d_n1);
        let eq71_e1863_d_n2: f64 = (eq71_e1858_d_n2 + eq71_e1861_d_n2);
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + eq71_e1861_d_n3);
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + eq71_e1861_d_n4);
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + eq71_e1861_d_n5);
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + eq71_e1861_d_n6);
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + eq71_e1861_d_n7);
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + eq71_e1861_d_n8);
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + eq71_e1861_d_n9);
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + eq71_e1861_d_n10);
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + eq71_e1861_d_n11);
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + eq71_e1861_d_n12);
        let eq71_e1863_d_n13: f64 = (eq71_e1858_d_n13 + eq71_e1861_d_n13);
        let eq71_e1863_q: f64 = eq71_e1862_q;
        let eq71_e1866: f64 = (s.v[410] / s.v[157]);
        let eq71_e1866_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq71_e1866_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq71_e1866_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq71_e1866_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq71_e1866_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq71_e1866_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq71_e1866_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq71_e1866_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq71_e1866_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq71_e1866_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq71_e1866_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq71_e1866_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq71_e1866_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq71_e1866_d_n13: f64 = (s.dn[410][13] / s.v[157]);
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
        let eq71_e1867_q: f64 = eq71_e1863_q;
        (eq71_e1867, eq71_e1867_d_n0, eq71_e1867_d_n1, eq71_e1867_d_n2, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_d_n13, eq71_e1867_q, eq71_e1861_d_n0, eq71_e1861_d_n1, eq71_e1861_d_n2, eq71_e1861_d_n3, eq71_e1861_d_n4, eq71_e1861_d_n5, eq71_e1861_d_n6, eq71_e1861_d_n7, eq71_e1861_d_n8, eq71_e1861_d_n9, eq71_e1861_d_n10, eq71_e1861_d_n11, eq71_e1861_d_n12, eq71_e1861_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 14] = [eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13];
        let eq71_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            &nodes,
            &eq71_reactive_node_derivatives,
            &branches,
            &eq71_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_72_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq72_e1892, eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13, eq72_e1892_q, eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13,) = {
    if ((((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) && (s.v[1562] != 0.0)) {
        let eq72_e1879: f64 = (-s.v[885]);
        let eq72_e1879_d_n0: f64 = (-s.dn[885][0]);
        let eq72_e1879_d_n1: f64 = (-s.dn[885][1]);
        let eq72_e1879_d_n2: f64 = (-s.dn[885][2]);
        let eq72_e1879_d_n3: f64 = (-s.dn[885][3]);
        let eq72_e1879_d_n4: f64 = (-s.dn[885][4]);
        let eq72_e1879_d_n5: f64 = (-s.dn[885][5]);
        let eq72_e1879_d_n6: f64 = (-s.dn[885][6]);
        let eq72_e1879_d_n7: f64 = (-s.dn[885][7]);
        let eq72_e1879_d_n8: f64 = (-s.dn[885][8]);
        let eq72_e1879_d_n9: f64 = (-s.dn[885][9]);
        let eq72_e1879_d_n10: f64 = (-s.dn[885][10]);
        let eq72_e1879_d_n11: f64 = (-s.dn[885][11]);
        let eq72_e1879_d_n12: f64 = (-s.dn[885][12]);
        let eq72_e1879_d_n13: f64 = (-s.dn[885][13]);
        let eq72_e1881: f64 = (eq72_e1879 * s.v[822]);
        let eq72_e1881_d_n0: f64 = ((eq72_e1879_d_n0 * s.v[822]) + (eq72_e1879 * s.dn[822][0]));
        let eq72_e1881_d_n1: f64 = ((eq72_e1879_d_n1 * s.v[822]) + (eq72_e1879 * s.dn[822][1]));
        let eq72_e1881_d_n2: f64 = ((eq72_e1879_d_n2 * s.v[822]) + (eq72_e1879 * s.dn[822][2]));
        let eq72_e1881_d_n3: f64 = ((eq72_e1879_d_n3 * s.v[822]) + (eq72_e1879 * s.dn[822][3]));
        let eq72_e1881_d_n4: f64 = ((eq72_e1879_d_n4 * s.v[822]) + (eq72_e1879 * s.dn[822][4]));
        let eq72_e1881_d_n5: f64 = ((eq72_e1879_d_n5 * s.v[822]) + (eq72_e1879 * s.dn[822][5]));
        let eq72_e1881_d_n6: f64 = ((eq72_e1879_d_n6 * s.v[822]) + (eq72_e1879 * s.dn[822][6]));
        let eq72_e1881_d_n7: f64 = ((eq72_e1879_d_n7 * s.v[822]) + (eq72_e1879 * s.dn[822][7]));
        let eq72_e1881_d_n8: f64 = ((eq72_e1879_d_n8 * s.v[822]) + (eq72_e1879 * s.dn[822][8]));
        let eq72_e1881_d_n9: f64 = ((eq72_e1879_d_n9 * s.v[822]) + (eq72_e1879 * s.dn[822][9]));
        let eq72_e1881_d_n10: f64 = ((eq72_e1879_d_n10 * s.v[822]) + (eq72_e1879 * s.dn[822][10]));
        let eq72_e1881_d_n11: f64 = ((eq72_e1879_d_n11 * s.v[822]) + (eq72_e1879 * s.dn[822][11]));
        let eq72_e1881_d_n12: f64 = ((eq72_e1879_d_n12 * s.v[822]) + (eq72_e1879 * s.dn[822][12]));
        let eq72_e1881_d_n13: f64 = ((eq72_e1879_d_n13 * s.v[822]) + (eq72_e1879 * s.dn[822][13]));
        let eq72_e1884: f64 = (s.v[410] * s.v[158]);
        let eq72_e1884_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq72_e1884_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq72_e1884_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq72_e1884_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq72_e1884_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq72_e1884_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq72_e1884_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq72_e1884_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq72_e1884_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq72_e1884_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq72_e1884_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq72_e1884_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq72_e1884_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq72_e1884_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq72_e1885_q: f64 = eq72_e1884;
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1884);
        let eq72_e1886_d_n0: f64 = (eq72_e1881_d_n0 + eq72_e1884_d_n0);
        let eq72_e1886_d_n1: f64 = (eq72_e1881_d_n1 + eq72_e1884_d_n1);
        let eq72_e1886_d_n2: f64 = (eq72_e1881_d_n2 + eq72_e1884_d_n2);
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + eq72_e1884_d_n3);
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + eq72_e1884_d_n4);
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + eq72_e1884_d_n5);
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + eq72_e1884_d_n6);
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + eq72_e1884_d_n7);
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + eq72_e1884_d_n8);
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + eq72_e1884_d_n9);
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + eq72_e1884_d_n10);
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + eq72_e1884_d_n11);
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + eq72_e1884_d_n12);
        let eq72_e1886_d_n13: f64 = (eq72_e1881_d_n13 + eq72_e1884_d_n13);
        let eq72_e1886_q: f64 = eq72_e1885_q;
        let eq72_e1889: f64 = (s.v[410] / s.v[157]);
        let eq72_e1889_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq72_e1889_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq72_e1889_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq72_e1889_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq72_e1889_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq72_e1889_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq72_e1889_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq72_e1889_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq72_e1889_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq72_e1889_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq72_e1889_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq72_e1889_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq72_e1889_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq72_e1889_d_n13: f64 = (s.dn[410][13] / s.v[157]);
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
        let eq72_e1890_q: f64 = eq72_e1886_q;
        (eq72_e1890, eq72_e1890_d_n0, eq72_e1890_d_n1, eq72_e1890_d_n2, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_d_n13, eq72_e1890_q, eq72_e1884_d_n0, eq72_e1884_d_n1, eq72_e1884_d_n2, eq72_e1884_d_n3, eq72_e1884_d_n4, eq72_e1884_d_n5, eq72_e1884_d_n6, eq72_e1884_d_n7, eq72_e1884_d_n8, eq72_e1884_d_n9, eq72_e1884_d_n10, eq72_e1884_d_n11, eq72_e1884_d_n12, eq72_e1884_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 14] = [eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13];
        let eq72_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq72_reactive_node_derivatives,
            &branches,
            &eq72_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_73_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq73_e1920, eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13, eq73_e1920_q, eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13,) = {
    if (((((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) && (!(s.v[1562] != 0.0))) && (s.v[1563] != 0.0)) {
        let eq73_e1906: f64 = (s.v[885] / p.p30);
        let eq73_e1906_d_n0: f64 = (s.dn[885][0] / p.p30);
        let eq73_e1906_d_n1: f64 = (s.dn[885][1] / p.p30);
        let eq73_e1906_d_n2: f64 = (s.dn[885][2] / p.p30);
        let eq73_e1906_d_n3: f64 = (s.dn[885][3] / p.p30);
        let eq73_e1906_d_n4: f64 = (s.dn[885][4] / p.p30);
        let eq73_e1906_d_n5: f64 = (s.dn[885][5] / p.p30);
        let eq73_e1906_d_n6: f64 = (s.dn[885][6] / p.p30);
        let eq73_e1906_d_n7: f64 = (s.dn[885][7] / p.p30);
        let eq73_e1906_d_n8: f64 = (s.dn[885][8] / p.p30);
        let eq73_e1906_d_n9: f64 = (s.dn[885][9] / p.p30);
        let eq73_e1906_d_n10: f64 = (s.dn[885][10] / p.p30);
        let eq73_e1906_d_n11: f64 = (s.dn[885][11] / p.p30);
        let eq73_e1906_d_n12: f64 = (s.dn[885][12] / p.p30);
        let eq73_e1906_d_n13: f64 = (s.dn[885][13] / p.p30);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1907_d_n0: f64 = (-eq73_e1906_d_n0);
        let eq73_e1907_d_n1: f64 = (-eq73_e1906_d_n1);
        let eq73_e1907_d_n2: f64 = (-eq73_e1906_d_n2);
        let eq73_e1907_d_n3: f64 = (-eq73_e1906_d_n3);
        let eq73_e1907_d_n4: f64 = (-eq73_e1906_d_n4);
        let eq73_e1907_d_n5: f64 = (-eq73_e1906_d_n5);
        let eq73_e1907_d_n6: f64 = (-eq73_e1906_d_n6);
        let eq73_e1907_d_n7: f64 = (-eq73_e1906_d_n7);
        let eq73_e1907_d_n8: f64 = (-eq73_e1906_d_n8);
        let eq73_e1907_d_n9: f64 = (-eq73_e1906_d_n9);
        let eq73_e1907_d_n10: f64 = (-eq73_e1906_d_n10);
        let eq73_e1907_d_n11: f64 = (-eq73_e1906_d_n11);
        let eq73_e1907_d_n12: f64 = (-eq73_e1906_d_n12);
        let eq73_e1907_d_n13: f64 = (-eq73_e1906_d_n13);
        let eq73_e1909: f64 = (eq73_e1907 * s.v[822]);
        let eq73_e1909_d_n0: f64 = ((eq73_e1907_d_n0 * s.v[822]) + (eq73_e1907 * s.dn[822][0]));
        let eq73_e1909_d_n1: f64 = ((eq73_e1907_d_n1 * s.v[822]) + (eq73_e1907 * s.dn[822][1]));
        let eq73_e1909_d_n2: f64 = ((eq73_e1907_d_n2 * s.v[822]) + (eq73_e1907 * s.dn[822][2]));
        let eq73_e1909_d_n3: f64 = ((eq73_e1907_d_n3 * s.v[822]) + (eq73_e1907 * s.dn[822][3]));
        let eq73_e1909_d_n4: f64 = ((eq73_e1907_d_n4 * s.v[822]) + (eq73_e1907 * s.dn[822][4]));
        let eq73_e1909_d_n5: f64 = ((eq73_e1907_d_n5 * s.v[822]) + (eq73_e1907 * s.dn[822][5]));
        let eq73_e1909_d_n6: f64 = ((eq73_e1907_d_n6 * s.v[822]) + (eq73_e1907 * s.dn[822][6]));
        let eq73_e1909_d_n7: f64 = ((eq73_e1907_d_n7 * s.v[822]) + (eq73_e1907 * s.dn[822][7]));
        let eq73_e1909_d_n8: f64 = ((eq73_e1907_d_n8 * s.v[822]) + (eq73_e1907 * s.dn[822][8]));
        let eq73_e1909_d_n9: f64 = ((eq73_e1907_d_n9 * s.v[822]) + (eq73_e1907 * s.dn[822][9]));
        let eq73_e1909_d_n10: f64 = ((eq73_e1907_d_n10 * s.v[822]) + (eq73_e1907 * s.dn[822][10]));
        let eq73_e1909_d_n11: f64 = ((eq73_e1907_d_n11 * s.v[822]) + (eq73_e1907 * s.dn[822][11]));
        let eq73_e1909_d_n12: f64 = ((eq73_e1907_d_n12 * s.v[822]) + (eq73_e1907 * s.dn[822][12]));
        let eq73_e1909_d_n13: f64 = ((eq73_e1907_d_n13 * s.v[822]) + (eq73_e1907 * s.dn[822][13]));
        let eq73_e1912: f64 = (s.v[410] * s.v[158]);
        let eq73_e1912_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq73_e1912_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq73_e1912_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq73_e1912_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq73_e1912_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq73_e1912_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq73_e1912_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq73_e1912_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq73_e1912_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq73_e1912_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq73_e1912_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq73_e1912_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq73_e1912_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq73_e1912_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq73_e1913_q: f64 = eq73_e1912;
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1912);
        let eq73_e1914_d_n0: f64 = (eq73_e1909_d_n0 + eq73_e1912_d_n0);
        let eq73_e1914_d_n1: f64 = (eq73_e1909_d_n1 + eq73_e1912_d_n1);
        let eq73_e1914_d_n2: f64 = (eq73_e1909_d_n2 + eq73_e1912_d_n2);
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + eq73_e1912_d_n3);
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + eq73_e1912_d_n4);
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + eq73_e1912_d_n5);
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + eq73_e1912_d_n6);
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + eq73_e1912_d_n7);
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + eq73_e1912_d_n8);
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + eq73_e1912_d_n9);
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + eq73_e1912_d_n10);
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + eq73_e1912_d_n11);
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + eq73_e1912_d_n12);
        let eq73_e1914_d_n13: f64 = (eq73_e1909_d_n13 + eq73_e1912_d_n13);
        let eq73_e1914_q: f64 = eq73_e1913_q;
        let eq73_e1917: f64 = (s.v[410] / s.v[157]);
        let eq73_e1917_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq73_e1917_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq73_e1917_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq73_e1917_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq73_e1917_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq73_e1917_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq73_e1917_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq73_e1917_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq73_e1917_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq73_e1917_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq73_e1917_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq73_e1917_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq73_e1917_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq73_e1917_d_n13: f64 = (s.dn[410][13] / s.v[157]);
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
        let eq73_e1918_q: f64 = eq73_e1914_q;
        (eq73_e1918, eq73_e1918_d_n0, eq73_e1918_d_n1, eq73_e1918_d_n2, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_d_n13, eq73_e1918_q, eq73_e1912_d_n0, eq73_e1912_d_n1, eq73_e1912_d_n2, eq73_e1912_d_n3, eq73_e1912_d_n4, eq73_e1912_d_n5, eq73_e1912_d_n6, eq73_e1912_d_n7, eq73_e1912_d_n8, eq73_e1912_d_n9, eq73_e1912_d_n10, eq73_e1912_d_n11, eq73_e1912_d_n12, eq73_e1912_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 14] = [eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13];
        let eq73_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            &nodes,
            &eq73_reactive_node_derivatives,
            &branches,
            &eq73_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_74_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq74_e1947, eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13, eq74_e1947_q, eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13,) = {
    if (((((s.v[1559] != 0.0) && (s.v[1560] != 0.0)) && (!(s.v[1561] != 0.0))) && (!(s.v[1562] != 0.0))) && (!(s.v[1563] != 0.0))) {
        let eq74_e1934: f64 = (-s.v[885]);
        let eq74_e1934_d_n0: f64 = (-s.dn[885][0]);
        let eq74_e1934_d_n1: f64 = (-s.dn[885][1]);
        let eq74_e1934_d_n2: f64 = (-s.dn[885][2]);
        let eq74_e1934_d_n3: f64 = (-s.dn[885][3]);
        let eq74_e1934_d_n4: f64 = (-s.dn[885][4]);
        let eq74_e1934_d_n5: f64 = (-s.dn[885][5]);
        let eq74_e1934_d_n6: f64 = (-s.dn[885][6]);
        let eq74_e1934_d_n7: f64 = (-s.dn[885][7]);
        let eq74_e1934_d_n8: f64 = (-s.dn[885][8]);
        let eq74_e1934_d_n9: f64 = (-s.dn[885][9]);
        let eq74_e1934_d_n10: f64 = (-s.dn[885][10]);
        let eq74_e1934_d_n11: f64 = (-s.dn[885][11]);
        let eq74_e1934_d_n12: f64 = (-s.dn[885][12]);
        let eq74_e1934_d_n13: f64 = (-s.dn[885][13]);
        let eq74_e1936: f64 = (eq74_e1934 * s.v[822]);
        let eq74_e1936_d_n0: f64 = ((eq74_e1934_d_n0 * s.v[822]) + (eq74_e1934 * s.dn[822][0]));
        let eq74_e1936_d_n1: f64 = ((eq74_e1934_d_n1 * s.v[822]) + (eq74_e1934 * s.dn[822][1]));
        let eq74_e1936_d_n2: f64 = ((eq74_e1934_d_n2 * s.v[822]) + (eq74_e1934 * s.dn[822][2]));
        let eq74_e1936_d_n3: f64 = ((eq74_e1934_d_n3 * s.v[822]) + (eq74_e1934 * s.dn[822][3]));
        let eq74_e1936_d_n4: f64 = ((eq74_e1934_d_n4 * s.v[822]) + (eq74_e1934 * s.dn[822][4]));
        let eq74_e1936_d_n5: f64 = ((eq74_e1934_d_n5 * s.v[822]) + (eq74_e1934 * s.dn[822][5]));
        let eq74_e1936_d_n6: f64 = ((eq74_e1934_d_n6 * s.v[822]) + (eq74_e1934 * s.dn[822][6]));
        let eq74_e1936_d_n7: f64 = ((eq74_e1934_d_n7 * s.v[822]) + (eq74_e1934 * s.dn[822][7]));
        let eq74_e1936_d_n8: f64 = ((eq74_e1934_d_n8 * s.v[822]) + (eq74_e1934 * s.dn[822][8]));
        let eq74_e1936_d_n9: f64 = ((eq74_e1934_d_n9 * s.v[822]) + (eq74_e1934 * s.dn[822][9]));
        let eq74_e1936_d_n10: f64 = ((eq74_e1934_d_n10 * s.v[822]) + (eq74_e1934 * s.dn[822][10]));
        let eq74_e1936_d_n11: f64 = ((eq74_e1934_d_n11 * s.v[822]) + (eq74_e1934 * s.dn[822][11]));
        let eq74_e1936_d_n12: f64 = ((eq74_e1934_d_n12 * s.v[822]) + (eq74_e1934 * s.dn[822][12]));
        let eq74_e1936_d_n13: f64 = ((eq74_e1934_d_n13 * s.v[822]) + (eq74_e1934 * s.dn[822][13]));
        let eq74_e1939: f64 = (s.v[410] * s.v[158]);
        let eq74_e1939_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq74_e1939_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq74_e1939_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq74_e1939_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq74_e1939_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq74_e1939_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq74_e1939_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq74_e1939_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq74_e1939_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq74_e1939_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq74_e1939_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq74_e1939_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq74_e1939_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq74_e1939_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq74_e1940_q: f64 = eq74_e1939;
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1939);
        let eq74_e1941_d_n0: f64 = (eq74_e1936_d_n0 + eq74_e1939_d_n0);
        let eq74_e1941_d_n1: f64 = (eq74_e1936_d_n1 + eq74_e1939_d_n1);
        let eq74_e1941_d_n2: f64 = (eq74_e1936_d_n2 + eq74_e1939_d_n2);
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + eq74_e1939_d_n3);
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + eq74_e1939_d_n4);
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + eq74_e1939_d_n5);
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + eq74_e1939_d_n6);
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + eq74_e1939_d_n7);
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + eq74_e1939_d_n8);
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + eq74_e1939_d_n9);
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + eq74_e1939_d_n10);
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + eq74_e1939_d_n11);
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + eq74_e1939_d_n12);
        let eq74_e1941_d_n13: f64 = (eq74_e1936_d_n13 + eq74_e1939_d_n13);
        let eq74_e1941_q: f64 = eq74_e1940_q;
        let eq74_e1944: f64 = (s.v[410] / s.v[157]);
        let eq74_e1944_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq74_e1944_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq74_e1944_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq74_e1944_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq74_e1944_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq74_e1944_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq74_e1944_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq74_e1944_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq74_e1944_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq74_e1944_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq74_e1944_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq74_e1944_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq74_e1944_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq74_e1944_d_n13: f64 = (s.dn[410][13] / s.v[157]);
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
        let eq74_e1945_q: f64 = eq74_e1941_q;
        (eq74_e1945, eq74_e1945_d_n0, eq74_e1945_d_n1, eq74_e1945_d_n2, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_d_n13, eq74_e1945_q, eq74_e1939_d_n0, eq74_e1939_d_n1, eq74_e1939_d_n2, eq74_e1939_d_n3, eq74_e1939_d_n4, eq74_e1939_d_n5, eq74_e1939_d_n6, eq74_e1939_d_n7, eq74_e1939_d_n8, eq74_e1939_d_n9, eq74_e1939_d_n10, eq74_e1939_d_n11, eq74_e1939_d_n12, eq74_e1939_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 14] = [eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13];
        let eq74_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            &nodes,
            &eq74_reactive_node_derivatives,
            &branches,
            &eq74_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_75_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq75_e1970, eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13, eq75_e1970_q, eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13,) = {
    if (((s.v[1559] != 0.0) && (!(s.v[1560] != 0.0))) && (s.v[1564] != 0.0)) {
        let eq75_e1956: f64 = (s.v[885] / p.p30);
        let eq75_e1956_d_n0: f64 = (s.dn[885][0] / p.p30);
        let eq75_e1956_d_n1: f64 = (s.dn[885][1] / p.p30);
        let eq75_e1956_d_n2: f64 = (s.dn[885][2] / p.p30);
        let eq75_e1956_d_n3: f64 = (s.dn[885][3] / p.p30);
        let eq75_e1956_d_n4: f64 = (s.dn[885][4] / p.p30);
        let eq75_e1956_d_n5: f64 = (s.dn[885][5] / p.p30);
        let eq75_e1956_d_n6: f64 = (s.dn[885][6] / p.p30);
        let eq75_e1956_d_n7: f64 = (s.dn[885][7] / p.p30);
        let eq75_e1956_d_n8: f64 = (s.dn[885][8] / p.p30);
        let eq75_e1956_d_n9: f64 = (s.dn[885][9] / p.p30);
        let eq75_e1956_d_n10: f64 = (s.dn[885][10] / p.p30);
        let eq75_e1956_d_n11: f64 = (s.dn[885][11] / p.p30);
        let eq75_e1956_d_n12: f64 = (s.dn[885][12] / p.p30);
        let eq75_e1956_d_n13: f64 = (s.dn[885][13] / p.p30);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1957_d_n0: f64 = (-eq75_e1956_d_n0);
        let eq75_e1957_d_n1: f64 = (-eq75_e1956_d_n1);
        let eq75_e1957_d_n2: f64 = (-eq75_e1956_d_n2);
        let eq75_e1957_d_n3: f64 = (-eq75_e1956_d_n3);
        let eq75_e1957_d_n4: f64 = (-eq75_e1956_d_n4);
        let eq75_e1957_d_n5: f64 = (-eq75_e1956_d_n5);
        let eq75_e1957_d_n6: f64 = (-eq75_e1956_d_n6);
        let eq75_e1957_d_n7: f64 = (-eq75_e1956_d_n7);
        let eq75_e1957_d_n8: f64 = (-eq75_e1956_d_n8);
        let eq75_e1957_d_n9: f64 = (-eq75_e1956_d_n9);
        let eq75_e1957_d_n10: f64 = (-eq75_e1956_d_n10);
        let eq75_e1957_d_n11: f64 = (-eq75_e1956_d_n11);
        let eq75_e1957_d_n12: f64 = (-eq75_e1956_d_n12);
        let eq75_e1957_d_n13: f64 = (-eq75_e1956_d_n13);
        let eq75_e1959: f64 = (eq75_e1957 * s.v[822]);
        let eq75_e1959_d_n0: f64 = ((eq75_e1957_d_n0 * s.v[822]) + (eq75_e1957 * s.dn[822][0]));
        let eq75_e1959_d_n1: f64 = ((eq75_e1957_d_n1 * s.v[822]) + (eq75_e1957 * s.dn[822][1]));
        let eq75_e1959_d_n2: f64 = ((eq75_e1957_d_n2 * s.v[822]) + (eq75_e1957 * s.dn[822][2]));
        let eq75_e1959_d_n3: f64 = ((eq75_e1957_d_n3 * s.v[822]) + (eq75_e1957 * s.dn[822][3]));
        let eq75_e1959_d_n4: f64 = ((eq75_e1957_d_n4 * s.v[822]) + (eq75_e1957 * s.dn[822][4]));
        let eq75_e1959_d_n5: f64 = ((eq75_e1957_d_n5 * s.v[822]) + (eq75_e1957 * s.dn[822][5]));
        let eq75_e1959_d_n6: f64 = ((eq75_e1957_d_n6 * s.v[822]) + (eq75_e1957 * s.dn[822][6]));
        let eq75_e1959_d_n7: f64 = ((eq75_e1957_d_n7 * s.v[822]) + (eq75_e1957 * s.dn[822][7]));
        let eq75_e1959_d_n8: f64 = ((eq75_e1957_d_n8 * s.v[822]) + (eq75_e1957 * s.dn[822][8]));
        let eq75_e1959_d_n9: f64 = ((eq75_e1957_d_n9 * s.v[822]) + (eq75_e1957 * s.dn[822][9]));
        let eq75_e1959_d_n10: f64 = ((eq75_e1957_d_n10 * s.v[822]) + (eq75_e1957 * s.dn[822][10]));
        let eq75_e1959_d_n11: f64 = ((eq75_e1957_d_n11 * s.v[822]) + (eq75_e1957 * s.dn[822][11]));
        let eq75_e1959_d_n12: f64 = ((eq75_e1957_d_n12 * s.v[822]) + (eq75_e1957 * s.dn[822][12]));
        let eq75_e1959_d_n13: f64 = ((eq75_e1957_d_n13 * s.v[822]) + (eq75_e1957 * s.dn[822][13]));
        let eq75_e1962: f64 = (s.v[410] * s.v[158]);
        let eq75_e1962_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq75_e1962_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq75_e1962_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq75_e1962_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq75_e1962_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq75_e1962_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq75_e1962_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq75_e1962_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq75_e1962_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq75_e1962_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq75_e1962_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq75_e1962_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq75_e1962_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq75_e1962_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq75_e1963_q: f64 = eq75_e1962;
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1962);
        let eq75_e1964_d_n0: f64 = (eq75_e1959_d_n0 + eq75_e1962_d_n0);
        let eq75_e1964_d_n1: f64 = (eq75_e1959_d_n1 + eq75_e1962_d_n1);
        let eq75_e1964_d_n2: f64 = (eq75_e1959_d_n2 + eq75_e1962_d_n2);
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + eq75_e1962_d_n3);
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + eq75_e1962_d_n4);
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + eq75_e1962_d_n5);
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + eq75_e1962_d_n6);
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + eq75_e1962_d_n7);
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + eq75_e1962_d_n8);
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + eq75_e1962_d_n9);
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + eq75_e1962_d_n10);
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + eq75_e1962_d_n11);
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + eq75_e1962_d_n12);
        let eq75_e1964_d_n13: f64 = (eq75_e1959_d_n13 + eq75_e1962_d_n13);
        let eq75_e1964_q: f64 = eq75_e1963_q;
        let eq75_e1967: f64 = (s.v[410] / s.v[157]);
        let eq75_e1967_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq75_e1967_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq75_e1967_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq75_e1967_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq75_e1967_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq75_e1967_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq75_e1967_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq75_e1967_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq75_e1967_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq75_e1967_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq75_e1967_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq75_e1967_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq75_e1967_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq75_e1967_d_n13: f64 = (s.dn[410][13] / s.v[157]);
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
        let eq75_e1968_q: f64 = eq75_e1964_q;
        (eq75_e1968, eq75_e1968_d_n0, eq75_e1968_d_n1, eq75_e1968_d_n2, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_d_n13, eq75_e1968_q, eq75_e1962_d_n0, eq75_e1962_d_n1, eq75_e1962_d_n2, eq75_e1962_d_n3, eq75_e1962_d_n4, eq75_e1962_d_n5, eq75_e1962_d_n6, eq75_e1962_d_n7, eq75_e1962_d_n8, eq75_e1962_d_n9, eq75_e1962_d_n10, eq75_e1962_d_n11, eq75_e1962_d_n12, eq75_e1962_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_reactive_node_derivatives: [f64; 14] = [eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13];
        let eq75_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            &nodes,
            &eq75_reactive_node_derivatives,
            &branches,
            &eq75_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_76_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq76_e1992, eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13, eq76_e1992_q, eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13,) = {
    if (((s.v[1559] != 0.0) && (!(s.v[1560] != 0.0))) && (!(s.v[1564] != 0.0))) {
        let eq76_e1979: f64 = (-s.v[885]);
        let eq76_e1979_d_n0: f64 = (-s.dn[885][0]);
        let eq76_e1979_d_n1: f64 = (-s.dn[885][1]);
        let eq76_e1979_d_n2: f64 = (-s.dn[885][2]);
        let eq76_e1979_d_n3: f64 = (-s.dn[885][3]);
        let eq76_e1979_d_n4: f64 = (-s.dn[885][4]);
        let eq76_e1979_d_n5: f64 = (-s.dn[885][5]);
        let eq76_e1979_d_n6: f64 = (-s.dn[885][6]);
        let eq76_e1979_d_n7: f64 = (-s.dn[885][7]);
        let eq76_e1979_d_n8: f64 = (-s.dn[885][8]);
        let eq76_e1979_d_n9: f64 = (-s.dn[885][9]);
        let eq76_e1979_d_n10: f64 = (-s.dn[885][10]);
        let eq76_e1979_d_n11: f64 = (-s.dn[885][11]);
        let eq76_e1979_d_n12: f64 = (-s.dn[885][12]);
        let eq76_e1979_d_n13: f64 = (-s.dn[885][13]);
        let eq76_e1981: f64 = (eq76_e1979 * s.v[822]);
        let eq76_e1981_d_n0: f64 = ((eq76_e1979_d_n0 * s.v[822]) + (eq76_e1979 * s.dn[822][0]));
        let eq76_e1981_d_n1: f64 = ((eq76_e1979_d_n1 * s.v[822]) + (eq76_e1979 * s.dn[822][1]));
        let eq76_e1981_d_n2: f64 = ((eq76_e1979_d_n2 * s.v[822]) + (eq76_e1979 * s.dn[822][2]));
        let eq76_e1981_d_n3: f64 = ((eq76_e1979_d_n3 * s.v[822]) + (eq76_e1979 * s.dn[822][3]));
        let eq76_e1981_d_n4: f64 = ((eq76_e1979_d_n4 * s.v[822]) + (eq76_e1979 * s.dn[822][4]));
        let eq76_e1981_d_n5: f64 = ((eq76_e1979_d_n5 * s.v[822]) + (eq76_e1979 * s.dn[822][5]));
        let eq76_e1981_d_n6: f64 = ((eq76_e1979_d_n6 * s.v[822]) + (eq76_e1979 * s.dn[822][6]));
        let eq76_e1981_d_n7: f64 = ((eq76_e1979_d_n7 * s.v[822]) + (eq76_e1979 * s.dn[822][7]));
        let eq76_e1981_d_n8: f64 = ((eq76_e1979_d_n8 * s.v[822]) + (eq76_e1979 * s.dn[822][8]));
        let eq76_e1981_d_n9: f64 = ((eq76_e1979_d_n9 * s.v[822]) + (eq76_e1979 * s.dn[822][9]));
        let eq76_e1981_d_n10: f64 = ((eq76_e1979_d_n10 * s.v[822]) + (eq76_e1979 * s.dn[822][10]));
        let eq76_e1981_d_n11: f64 = ((eq76_e1979_d_n11 * s.v[822]) + (eq76_e1979 * s.dn[822][11]));
        let eq76_e1981_d_n12: f64 = ((eq76_e1979_d_n12 * s.v[822]) + (eq76_e1979 * s.dn[822][12]));
        let eq76_e1981_d_n13: f64 = ((eq76_e1979_d_n13 * s.v[822]) + (eq76_e1979 * s.dn[822][13]));
        let eq76_e1984: f64 = (s.v[410] * s.v[158]);
        let eq76_e1984_d_n0: f64 = (s.dn[410][0] * s.v[158]);
        let eq76_e1984_d_n1: f64 = (s.dn[410][1] * s.v[158]);
        let eq76_e1984_d_n2: f64 = (s.dn[410][2] * s.v[158]);
        let eq76_e1984_d_n3: f64 = (s.dn[410][3] * s.v[158]);
        let eq76_e1984_d_n4: f64 = (s.dn[410][4] * s.v[158]);
        let eq76_e1984_d_n5: f64 = (s.dn[410][5] * s.v[158]);
        let eq76_e1984_d_n6: f64 = (s.dn[410][6] * s.v[158]);
        let eq76_e1984_d_n7: f64 = (s.dn[410][7] * s.v[158]);
        let eq76_e1984_d_n8: f64 = (s.dn[410][8] * s.v[158]);
        let eq76_e1984_d_n9: f64 = (s.dn[410][9] * s.v[158]);
        let eq76_e1984_d_n10: f64 = (s.dn[410][10] * s.v[158]);
        let eq76_e1984_d_n11: f64 = (s.dn[410][11] * s.v[158]);
        let eq76_e1984_d_n12: f64 = (s.dn[410][12] * s.v[158]);
        let eq76_e1984_d_n13: f64 = (s.dn[410][13] * s.v[158]);
        let eq76_e1985_q: f64 = eq76_e1984;
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1984);
        let eq76_e1986_d_n0: f64 = (eq76_e1981_d_n0 + eq76_e1984_d_n0);
        let eq76_e1986_d_n1: f64 = (eq76_e1981_d_n1 + eq76_e1984_d_n1);
        let eq76_e1986_d_n2: f64 = (eq76_e1981_d_n2 + eq76_e1984_d_n2);
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + eq76_e1984_d_n3);
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + eq76_e1984_d_n4);
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + eq76_e1984_d_n5);
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + eq76_e1984_d_n6);
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + eq76_e1984_d_n7);
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + eq76_e1984_d_n8);
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + eq76_e1984_d_n9);
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + eq76_e1984_d_n10);
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + eq76_e1984_d_n11);
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + eq76_e1984_d_n12);
        let eq76_e1986_d_n13: f64 = (eq76_e1981_d_n13 + eq76_e1984_d_n13);
        let eq76_e1986_q: f64 = eq76_e1985_q;
        let eq76_e1989: f64 = (s.v[410] / s.v[157]);
        let eq76_e1989_d_n0: f64 = (s.dn[410][0] / s.v[157]);
        let eq76_e1989_d_n1: f64 = (s.dn[410][1] / s.v[157]);
        let eq76_e1989_d_n2: f64 = (s.dn[410][2] / s.v[157]);
        let eq76_e1989_d_n3: f64 = (s.dn[410][3] / s.v[157]);
        let eq76_e1989_d_n4: f64 = (s.dn[410][4] / s.v[157]);
        let eq76_e1989_d_n5: f64 = (s.dn[410][5] / s.v[157]);
        let eq76_e1989_d_n6: f64 = (s.dn[410][6] / s.v[157]);
        let eq76_e1989_d_n7: f64 = (s.dn[410][7] / s.v[157]);
        let eq76_e1989_d_n8: f64 = (s.dn[410][8] / s.v[157]);
        let eq76_e1989_d_n9: f64 = (s.dn[410][9] / s.v[157]);
        let eq76_e1989_d_n10: f64 = (s.dn[410][10] / s.v[157]);
        let eq76_e1989_d_n11: f64 = (s.dn[410][11] / s.v[157]);
        let eq76_e1989_d_n12: f64 = (s.dn[410][12] / s.v[157]);
        let eq76_e1989_d_n13: f64 = (s.dn[410][13] / s.v[157]);
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
        let eq76_e1990_q: f64 = eq76_e1986_q;
        (eq76_e1990, eq76_e1990_d_n0, eq76_e1990_d_n1, eq76_e1990_d_n2, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_d_n13, eq76_e1990_q, eq76_e1984_d_n0, eq76_e1984_d_n1, eq76_e1984_d_n2, eq76_e1984_d_n3, eq76_e1984_d_n4, eq76_e1984_d_n5, eq76_e1984_d_n6, eq76_e1984_d_n7, eq76_e1984_d_n8, eq76_e1984_d_n9, eq76_e1984_d_n10, eq76_e1984_d_n11, eq76_e1984_d_n12, eq76_e1984_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 14] = [eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13];
        let eq76_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            &nodes,
            &eq76_reactive_node_derivatives,
            &branches,
            &eq76_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

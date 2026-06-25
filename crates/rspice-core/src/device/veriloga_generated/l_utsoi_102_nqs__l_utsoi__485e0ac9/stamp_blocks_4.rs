#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq3_e525: f64 = (p.p14 * s.v[366]);
        let eq3_e525_d_n0: f64 = (p.p14 * s.dn[366][0]);
        let eq3_e525_d_n1: f64 = (p.p14 * s.dn[366][1]);
        let eq3_e525_d_n2: f64 = (p.p14 * s.dn[366][2]);
        let eq3_e525_d_n3: f64 = (p.p14 * s.dn[366][3]);
        let eq3_e525_d_n4: f64 = (p.p14 * s.dn[366][4]);
        let eq3_e525_d_n5: f64 = (p.p14 * s.dn[366][5]);
        let eq3_e525_d_n6: f64 = (p.p14 * s.dn[366][6]);
        let eq3_e525_d_n7: f64 = (p.p14 * s.dn[366][7]);
        let eq3_e525_d_n8: f64 = (p.p14 * s.dn[366][8]);
        let eq3_e525_d_n9: f64 = (p.p14 * s.dn[366][9]);
        let eq3_e525_d_n10: f64 = (p.p14 * s.dn[366][10]);
        let eq3_e525_d_n11: f64 = (p.p14 * s.dn[366][11]);
        let eq3_e525_d_n12: f64 = (p.p14 * s.dn[366][12]);
        let eq3_e525_d_n13: f64 = (p.p14 * s.dn[366][13]);
        let eq3_value: f64 = eq3_e525;
        let eq3_node_derivatives: [f64; 14] = [eq3_e525_d_n0, eq3_e525_d_n1, eq3_e525_d_n2, eq3_e525_d_n3, eq3_e525_d_n4, eq3_e525_d_n5, eq3_e525_d_n6, eq3_e525_d_n7, eq3_e525_d_n8, eq3_e525_d_n9, eq3_e525_d_n10, eq3_e525_d_n11, eq3_e525_d_n12, eq3_e525_d_n13];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            self.multiplicity * (eq3_value),
            &nodes,
            &eq3_node_derivatives,
            &branches,
            &eq3_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq4_e528: f64 = (p.p14 * s.v[367]);
        let eq4_e528_d_n0: f64 = (p.p14 * s.dn[367][0]);
        let eq4_e528_d_n1: f64 = (p.p14 * s.dn[367][1]);
        let eq4_e528_d_n2: f64 = (p.p14 * s.dn[367][2]);
        let eq4_e528_d_n3: f64 = (p.p14 * s.dn[367][3]);
        let eq4_e528_d_n4: f64 = (p.p14 * s.dn[367][4]);
        let eq4_e528_d_n5: f64 = (p.p14 * s.dn[367][5]);
        let eq4_e528_d_n6: f64 = (p.p14 * s.dn[367][6]);
        let eq4_e528_d_n7: f64 = (p.p14 * s.dn[367][7]);
        let eq4_e528_d_n8: f64 = (p.p14 * s.dn[367][8]);
        let eq4_e528_d_n9: f64 = (p.p14 * s.dn[367][9]);
        let eq4_e528_d_n10: f64 = (p.p14 * s.dn[367][10]);
        let eq4_e528_d_n11: f64 = (p.p14 * s.dn[367][11]);
        let eq4_e528_d_n12: f64 = (p.p14 * s.dn[367][12]);
        let eq4_e528_d_n13: f64 = (p.p14 * s.dn[367][13]);
        let eq4_value: f64 = eq4_e528;
        let eq4_node_derivatives: [f64; 14] = [eq4_e528_d_n0, eq4_e528_d_n1, eq4_e528_d_n2, eq4_e528_d_n3, eq4_e528_d_n4, eq4_e528_d_n5, eq4_e528_d_n6, eq4_e528_d_n7, eq4_e528_d_n8, eq4_e528_d_n9, eq4_e528_d_n10, eq4_e528_d_n11, eq4_e528_d_n12, eq4_e528_d_n13];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq4_value),
            &nodes,
            &eq4_node_derivatives,
            &branches,
            &eq4_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq5_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq5_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_6_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq6_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq6_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq7_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq7_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_8_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq8_e534: f64 = (p.p31 * s.v[475]);
        let eq8_e536: f64 = (eq8_e534 * (nv7 - nv6));
        let eq8_e536_d_n6: f64 = (-eq8_e534);
        let eq8_value: f64 = eq8_e536;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq8_value),
            &[
                GeneratedDerivative::node(nodes[6], self.multiplicity * eq8_e536_d_n6),
                GeneratedDerivative::node(nodes[7], self.multiplicity * eq8_e534),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_9_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq9_value: f64 = s.v[1765];
        let eq9_node_derivatives: [f64; 14] = [s.dn[1765][0], s.dn[1765][1], s.dn[1765][2], s.dn[1765][3], s.dn[1765][4], s.dn[1765][5], s.dn[1765][6], s.dn[1765][7], s.dn[1765][8], s.dn[1765][9], s.dn[1765][10], s.dn[1765][11], s.dn[1765][12], s.dn[1765][13]];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq9_value),
            &nodes,
            &eq9_node_derivatives,
            &branches,
            &eq9_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_value: f64 = s.v[1766];
        let eq10_node_derivatives: [f64; 14] = [s.dn[1766][0], s.dn[1766][1], s.dn[1766][2], s.dn[1766][3], s.dn[1766][4], s.dn[1766][5], s.dn[1766][6], s.dn[1766][7], s.dn[1766][8], s.dn[1766][9], s.dn[1766][10], s.dn[1766][11], s.dn[1766][12], s.dn[1766][13]];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq11_e548, eq11_e548_d_n0, eq11_e548_d_n1, eq11_e548_d_n2, eq11_e548_d_n3, eq11_e548_d_n4, eq11_e548_d_n5, eq11_e548_d_n6, eq11_e548_d_n7, eq11_e548_d_n8, eq11_e548_d_n9, eq11_e548_d_n10, eq11_e548_d_n11, eq11_e548_d_n12, eq11_e548_d_n13,) = {
    if (s.v[1768] != 0.0) {
        let eq11_e542: f64 = (p.p31 * s.v[13]);
        let eq11_e542_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq11_e542_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq11_e542_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq11_e542_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq11_e542_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq11_e542_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq11_e542_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq11_e542_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq11_e542_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq11_e542_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq11_e542_d_n10: f64 = (p.p31 * s.dn[13][10]);
        let eq11_e542_d_n11: f64 = (p.p31 * s.dn[13][11]);
        let eq11_e542_d_n12: f64 = (p.p31 * s.dn[13][12]);
        let eq11_e542_d_n13: f64 = (p.p31 * s.dn[13][13]);
        let eq11_e544: f64 = (eq11_e542 * s.v[316]);
        let eq11_e544_d_n0: f64 = ((eq11_e542_d_n0 * s.v[316]) + (eq11_e542 * s.dn[316][0]));
        let eq11_e544_d_n1: f64 = ((eq11_e542_d_n1 * s.v[316]) + (eq11_e542 * s.dn[316][1]));
        let eq11_e544_d_n2: f64 = ((eq11_e542_d_n2 * s.v[316]) + (eq11_e542 * s.dn[316][2]));
        let eq11_e544_d_n3: f64 = ((eq11_e542_d_n3 * s.v[316]) + (eq11_e542 * s.dn[316][3]));
        let eq11_e544_d_n4: f64 = ((eq11_e542_d_n4 * s.v[316]) + (eq11_e542 * s.dn[316][4]));
        let eq11_e544_d_n5: f64 = ((eq11_e542_d_n5 * s.v[316]) + (eq11_e542 * s.dn[316][5]));
        let eq11_e544_d_n6: f64 = ((eq11_e542_d_n6 * s.v[316]) + (eq11_e542 * s.dn[316][6]));
        let eq11_e544_d_n7: f64 = ((eq11_e542_d_n7 * s.v[316]) + (eq11_e542 * s.dn[316][7]));
        let eq11_e544_d_n8: f64 = ((eq11_e542_d_n8 * s.v[316]) + (eq11_e542 * s.dn[316][8]));
        let eq11_e544_d_n9: f64 = ((eq11_e542_d_n9 * s.v[316]) + (eq11_e542 * s.dn[316][9]));
        let eq11_e544_d_n10: f64 = ((eq11_e542_d_n10 * s.v[316]) + (eq11_e542 * s.dn[316][10]));
        let eq11_e544_d_n11: f64 = ((eq11_e542_d_n11 * s.v[316]) + (eq11_e542 * s.dn[316][11]));
        let eq11_e544_d_n12: f64 = ((eq11_e542_d_n12 * s.v[316]) + (eq11_e542 * s.dn[316][12]));
        let eq11_e544_d_n13: f64 = ((eq11_e542_d_n13 * s.v[316]) + (eq11_e542 * s.dn[316][13]));
        let eq11_e546: f64 = (eq11_e544 * (nv1 - nv9));
        let eq11_e546_d_n0: f64 = (eq11_e544_d_n0 * (nv1 - nv9));
        let eq11_e546_d_n1: f64 = ((eq11_e544_d_n1 * (nv1 - nv9)) + eq11_e544);
        let eq11_e546_d_n2: f64 = (eq11_e544_d_n2 * (nv1 - nv9));
        let eq11_e546_d_n3: f64 = (eq11_e544_d_n3 * (nv1 - nv9));
        let eq11_e546_d_n4: f64 = (eq11_e544_d_n4 * (nv1 - nv9));
        let eq11_e546_d_n5: f64 = (eq11_e544_d_n5 * (nv1 - nv9));
        let eq11_e546_d_n6: f64 = (eq11_e544_d_n6 * (nv1 - nv9));
        let eq11_e546_d_n7: f64 = (eq11_e544_d_n7 * (nv1 - nv9));
        let eq11_e546_d_n8: f64 = (eq11_e544_d_n8 * (nv1 - nv9));
        let eq11_e546_d_n9: f64 = ((eq11_e544_d_n9 * (nv1 - nv9)) + (-eq11_e544));
        let eq11_e546_d_n10: f64 = (eq11_e544_d_n10 * (nv1 - nv9));
        let eq11_e546_d_n11: f64 = (eq11_e544_d_n11 * (nv1 - nv9));
        let eq11_e546_d_n12: f64 = (eq11_e544_d_n12 * (nv1 - nv9));
        let eq11_e546_d_n13: f64 = (eq11_e544_d_n13 * (nv1 - nv9));
        (eq11_e546, eq11_e546_d_n0, eq11_e546_d_n1, eq11_e546_d_n2, eq11_e546_d_n3, eq11_e546_d_n4, eq11_e546_d_n5, eq11_e546_d_n6, eq11_e546_d_n7, eq11_e546_d_n8, eq11_e546_d_n9, eq11_e546_d_n10, eq11_e546_d_n11, eq11_e546_d_n12, eq11_e546_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e548;
        let eq11_node_derivatives: [f64; 14] = [eq11_e548_d_n0, eq11_e548_d_n1, eq11_e548_d_n2, eq11_e548_d_n3, eq11_e548_d_n4, eq11_e548_d_n5, eq11_e548_d_n6, eq11_e548_d_n7, eq11_e548_d_n8, eq11_e548_d_n9, eq11_e548_d_n10, eq11_e548_d_n11, eq11_e548_d_n12, eq11_e548_d_n13];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_12_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq12_e558,) = {
    if (s.v[1768] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e558;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[9]),
            self.multiplicity * (eq12_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq13_e563,) = {
    if (!(s.v[1768] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e563;
        stamper.stamp_potential(
            branches[0],
            eq13_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_14_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq14_e573, eq14_e573_d_n0, eq14_e573_d_n1, eq14_e573_d_n2, eq14_e573_d_n3, eq14_e573_d_n4, eq14_e573_d_n5, eq14_e573_d_n6, eq14_e573_d_n7, eq14_e573_d_n8, eq14_e573_d_n9, eq14_e573_d_n10, eq14_e573_d_n11, eq14_e573_d_n12, eq14_e573_d_n13,) = {
    if (s.v[1769] != 0.0) {
        let eq14_e567: f64 = (p.p31 * s.v[13]);
        let eq14_e567_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq14_e567_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq14_e567_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq14_e567_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq14_e567_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq14_e567_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq14_e567_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq14_e567_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq14_e567_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq14_e567_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq14_e567_d_n10: f64 = (p.p31 * s.dn[13][10]);
        let eq14_e567_d_n11: f64 = (p.p31 * s.dn[13][11]);
        let eq14_e567_d_n12: f64 = (p.p31 * s.dn[13][12]);
        let eq14_e567_d_n13: f64 = (p.p31 * s.dn[13][13]);
        let eq14_e569: f64 = (eq14_e567 * s.v[320]);
        let eq14_e569_d_n0: f64 = ((eq14_e567_d_n0 * s.v[320]) + (eq14_e567 * s.dn[320][0]));
        let eq14_e569_d_n1: f64 = ((eq14_e567_d_n1 * s.v[320]) + (eq14_e567 * s.dn[320][1]));
        let eq14_e569_d_n2: f64 = ((eq14_e567_d_n2 * s.v[320]) + (eq14_e567 * s.dn[320][2]));
        let eq14_e569_d_n3: f64 = ((eq14_e567_d_n3 * s.v[320]) + (eq14_e567 * s.dn[320][3]));
        let eq14_e569_d_n4: f64 = ((eq14_e567_d_n4 * s.v[320]) + (eq14_e567 * s.dn[320][4]));
        let eq14_e569_d_n5: f64 = ((eq14_e567_d_n5 * s.v[320]) + (eq14_e567 * s.dn[320][5]));
        let eq14_e569_d_n6: f64 = ((eq14_e567_d_n6 * s.v[320]) + (eq14_e567 * s.dn[320][6]));
        let eq14_e569_d_n7: f64 = ((eq14_e567_d_n7 * s.v[320]) + (eq14_e567 * s.dn[320][7]));
        let eq14_e569_d_n8: f64 = ((eq14_e567_d_n8 * s.v[320]) + (eq14_e567 * s.dn[320][8]));
        let eq14_e569_d_n9: f64 = ((eq14_e567_d_n9 * s.v[320]) + (eq14_e567 * s.dn[320][9]));
        let eq14_e569_d_n10: f64 = ((eq14_e567_d_n10 * s.v[320]) + (eq14_e567 * s.dn[320][10]));
        let eq14_e569_d_n11: f64 = ((eq14_e567_d_n11 * s.v[320]) + (eq14_e567 * s.dn[320][11]));
        let eq14_e569_d_n12: f64 = ((eq14_e567_d_n12 * s.v[320]) + (eq14_e567 * s.dn[320][12]));
        let eq14_e569_d_n13: f64 = ((eq14_e567_d_n13 * s.v[320]) + (eq14_e567 * s.dn[320][13]));
        let eq14_e571: f64 = (eq14_e569 * (nv2 - nv6));
        let eq14_e571_d_n0: f64 = (eq14_e569_d_n0 * (nv2 - nv6));
        let eq14_e571_d_n1: f64 = (eq14_e569_d_n1 * (nv2 - nv6));
        let eq14_e571_d_n2: f64 = ((eq14_e569_d_n2 * (nv2 - nv6)) + eq14_e569);
        let eq14_e571_d_n3: f64 = (eq14_e569_d_n3 * (nv2 - nv6));
        let eq14_e571_d_n4: f64 = (eq14_e569_d_n4 * (nv2 - nv6));
        let eq14_e571_d_n5: f64 = (eq14_e569_d_n5 * (nv2 - nv6));
        let eq14_e571_d_n6: f64 = ((eq14_e569_d_n6 * (nv2 - nv6)) + (-eq14_e569));
        let eq14_e571_d_n7: f64 = (eq14_e569_d_n7 * (nv2 - nv6));
        let eq14_e571_d_n8: f64 = (eq14_e569_d_n8 * (nv2 - nv6));
        let eq14_e571_d_n9: f64 = (eq14_e569_d_n9 * (nv2 - nv6));
        let eq14_e571_d_n10: f64 = (eq14_e569_d_n10 * (nv2 - nv6));
        let eq14_e571_d_n11: f64 = (eq14_e569_d_n11 * (nv2 - nv6));
        let eq14_e571_d_n12: f64 = (eq14_e569_d_n12 * (nv2 - nv6));
        let eq14_e571_d_n13: f64 = (eq14_e569_d_n13 * (nv2 - nv6));
        (eq14_e571, eq14_e571_d_n0, eq14_e571_d_n1, eq14_e571_d_n2, eq14_e571_d_n3, eq14_e571_d_n4, eq14_e571_d_n5, eq14_e571_d_n6, eq14_e571_d_n7, eq14_e571_d_n8, eq14_e571_d_n9, eq14_e571_d_n10, eq14_e571_d_n11, eq14_e571_d_n12, eq14_e571_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e573;
        let eq14_node_derivatives: [f64; 14] = [eq14_e573_d_n0, eq14_e573_d_n1, eq14_e573_d_n2, eq14_e573_d_n3, eq14_e573_d_n4, eq14_e573_d_n5, eq14_e573_d_n6, eq14_e573_d_n7, eq14_e573_d_n8, eq14_e573_d_n9, eq14_e573_d_n10, eq14_e573_d_n11, eq14_e573_d_n12, eq14_e573_d_n13];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq15_e583,) = {
    if (s.v[1769] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e583;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[6]),
            self.multiplicity * (eq15_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq16_e588,) = {
    if (!(s.v[1769] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e588;
        stamper.stamp_potential(
            branches[1],
            eq16_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq17_e598, eq17_e598_d_n0, eq17_e598_d_n1, eq17_e598_d_n2, eq17_e598_d_n3, eq17_e598_d_n4, eq17_e598_d_n5, eq17_e598_d_n6, eq17_e598_d_n7, eq17_e598_d_n8, eq17_e598_d_n9, eq17_e598_d_n10, eq17_e598_d_n11, eq17_e598_d_n12, eq17_e598_d_n13,) = {
    if (s.v[1770] != 0.0) {
        let eq17_e592: f64 = (p.p31 * s.v[13]);
        let eq17_e592_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq17_e592_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq17_e592_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq17_e592_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq17_e592_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq17_e592_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq17_e592_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq17_e592_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq17_e592_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq17_e592_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq17_e592_d_n10: f64 = (p.p31 * s.dn[13][10]);
        let eq17_e592_d_n11: f64 = (p.p31 * s.dn[13][11]);
        let eq17_e592_d_n12: f64 = (p.p31 * s.dn[13][12]);
        let eq17_e592_d_n13: f64 = (p.p31 * s.dn[13][13]);
        let eq17_e594: f64 = (eq17_e592 * s.v[324]);
        let eq17_e594_d_n0: f64 = ((eq17_e592_d_n0 * s.v[324]) + (eq17_e592 * s.dn[324][0]));
        let eq17_e594_d_n1: f64 = ((eq17_e592_d_n1 * s.v[324]) + (eq17_e592 * s.dn[324][1]));
        let eq17_e594_d_n2: f64 = ((eq17_e592_d_n2 * s.v[324]) + (eq17_e592 * s.dn[324][2]));
        let eq17_e594_d_n3: f64 = ((eq17_e592_d_n3 * s.v[324]) + (eq17_e592 * s.dn[324][3]));
        let eq17_e594_d_n4: f64 = ((eq17_e592_d_n4 * s.v[324]) + (eq17_e592 * s.dn[324][4]));
        let eq17_e594_d_n5: f64 = ((eq17_e592_d_n5 * s.v[324]) + (eq17_e592 * s.dn[324][5]));
        let eq17_e594_d_n6: f64 = ((eq17_e592_d_n6 * s.v[324]) + (eq17_e592 * s.dn[324][6]));
        let eq17_e594_d_n7: f64 = ((eq17_e592_d_n7 * s.v[324]) + (eq17_e592 * s.dn[324][7]));
        let eq17_e594_d_n8: f64 = ((eq17_e592_d_n8 * s.v[324]) + (eq17_e592 * s.dn[324][8]));
        let eq17_e594_d_n9: f64 = ((eq17_e592_d_n9 * s.v[324]) + (eq17_e592 * s.dn[324][9]));
        let eq17_e594_d_n10: f64 = ((eq17_e592_d_n10 * s.v[324]) + (eq17_e592 * s.dn[324][10]));
        let eq17_e594_d_n11: f64 = ((eq17_e592_d_n11 * s.v[324]) + (eq17_e592 * s.dn[324][11]));
        let eq17_e594_d_n12: f64 = ((eq17_e592_d_n12 * s.v[324]) + (eq17_e592 * s.dn[324][12]));
        let eq17_e594_d_n13: f64 = ((eq17_e592_d_n13 * s.v[324]) + (eq17_e592 * s.dn[324][13]));
        let eq17_e596: f64 = (eq17_e594 * (nv0 - nv7));
        let eq17_e596_d_n0: f64 = ((eq17_e594_d_n0 * (nv0 - nv7)) + eq17_e594);
        let eq17_e596_d_n1: f64 = (eq17_e594_d_n1 * (nv0 - nv7));
        let eq17_e596_d_n2: f64 = (eq17_e594_d_n2 * (nv0 - nv7));
        let eq17_e596_d_n3: f64 = (eq17_e594_d_n3 * (nv0 - nv7));
        let eq17_e596_d_n4: f64 = (eq17_e594_d_n4 * (nv0 - nv7));
        let eq17_e596_d_n5: f64 = (eq17_e594_d_n5 * (nv0 - nv7));
        let eq17_e596_d_n6: f64 = (eq17_e594_d_n6 * (nv0 - nv7));
        let eq17_e596_d_n7: f64 = ((eq17_e594_d_n7 * (nv0 - nv7)) + (-eq17_e594));
        let eq17_e596_d_n8: f64 = (eq17_e594_d_n8 * (nv0 - nv7));
        let eq17_e596_d_n9: f64 = (eq17_e594_d_n9 * (nv0 - nv7));
        let eq17_e596_d_n10: f64 = (eq17_e594_d_n10 * (nv0 - nv7));
        let eq17_e596_d_n11: f64 = (eq17_e594_d_n11 * (nv0 - nv7));
        let eq17_e596_d_n12: f64 = (eq17_e594_d_n12 * (nv0 - nv7));
        let eq17_e596_d_n13: f64 = (eq17_e594_d_n13 * (nv0 - nv7));
        (eq17_e596, eq17_e596_d_n0, eq17_e596_d_n1, eq17_e596_d_n2, eq17_e596_d_n3, eq17_e596_d_n4, eq17_e596_d_n5, eq17_e596_d_n6, eq17_e596_d_n7, eq17_e596_d_n8, eq17_e596_d_n9, eq17_e596_d_n10, eq17_e596_d_n11, eq17_e596_d_n12, eq17_e596_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e598;
        let eq17_node_derivatives: [f64; 14] = [eq17_e598_d_n0, eq17_e598_d_n1, eq17_e598_d_n2, eq17_e598_d_n3, eq17_e598_d_n4, eq17_e598_d_n5, eq17_e598_d_n6, eq17_e598_d_n7, eq17_e598_d_n8, eq17_e598_d_n9, eq17_e598_d_n10, eq17_e598_d_n11, eq17_e598_d_n12, eq17_e598_d_n13];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq17_value),
            &nodes,
            &eq17_node_derivatives,
            &branches,
            &eq17_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq18_e608,) = {
    if (s.v[1770] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e608;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq18_value),
            &[
            ],
        );
    }
}

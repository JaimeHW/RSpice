#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_49_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq49_e634,) = {
    if ((s.v[1851] != 0.0) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e634;
        stamper.stamp_potential(
            branches[9],
            eq49_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_50_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq50_e641,) = {
    if ((s.v[1851] != 0.0) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq50_value: f64 = eq50_e641;
        stamper.stamp_potential(
            branches[10],
            eq50_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_51_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq51_e647, eq51_e647_d_n0, eq51_e647_d_n1, eq51_e647_d_n2, eq51_e647_d_n3, eq51_e647_d_n4, eq51_e647_d_n5, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n8, eq51_e647_d_n9, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n13, eq51_e647_d_n14, eq51_e647_d_n15, eq51_e647_d_n16, eq51_e647_d_n17, eq51_e647_d_n18, eq51_e647_d_b0, eq51_e647_d_b1, eq51_e647_d_b2, eq51_e647_d_b3, eq51_e647_d_b4, eq51_e647_d_b5, eq51_e647_d_b6, eq51_e647_d_b7, eq51_e647_d_b8, eq51_e647_d_b9, eq51_e647_d_b10, eq51_e647_d_b11, eq51_e647_d_b12,) = {
    if ((s.v[1851] != 0.0) && (s.v[1852] != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e647;
        let eq51_node_derivatives: [f64; 19] = [eq51_e647_d_n0, eq51_e647_d_n1, eq51_e647_d_n2, eq51_e647_d_n3, eq51_e647_d_n4, eq51_e647_d_n5, eq51_e647_d_n6, eq51_e647_d_n7, eq51_e647_d_n8, eq51_e647_d_n9, eq51_e647_d_n10, eq51_e647_d_n11, eq51_e647_d_n12, eq51_e647_d_n13, eq51_e647_d_n14, eq51_e647_d_n15, eq51_e647_d_n16, eq51_e647_d_n17, eq51_e647_d_n18];
        let eq51_branch_derivatives: [f64; 13] = [eq51_e647_d_b0, eq51_e647_d_b1, eq51_e647_d_b2, eq51_e647_d_b3, eq51_e647_d_b4, eq51_e647_d_b5, eq51_e647_d_b6, eq51_e647_d_b7, eq51_e647_d_b8, eq51_e647_d_b9, eq51_e647_d_b10, eq51_e647_d_b11, eq51_e647_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq51_value),
            &nodes,
            &eq51_node_derivatives,
            &branches,
            &eq51_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_52_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq52_e655, eq52_e655_d_n17,) = {
    if ((s.v[1851] != 0.0) && (s.v[1852] != 0.0)) {
        let eq52_e653: f64 = ((nv17 - 0.0) * 1e-12);
        let eq52_e653_d_n17: f64 = 1e-12;
        (eq52_e653, eq52_e653_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e655;
        stamper.stamp_current(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq52_value),
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * eq52_e655_d_n17),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_53_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq53_e666, eq53_e666_d_n0, eq53_e666_d_n1, eq53_e666_d_n2, eq53_e666_d_n3, eq53_e666_d_n4, eq53_e666_d_n5, eq53_e666_d_n6, eq53_e666_d_n7, eq53_e666_d_n8, eq53_e666_d_n9, eq53_e666_d_n10, eq53_e666_d_n11, eq53_e666_d_n12, eq53_e666_d_n13, eq53_e666_d_n14, eq53_e666_d_n15, eq53_e666_d_n16, eq53_e666_d_n17, eq53_e666_d_n18, eq53_e666_d_b0, eq53_e666_d_b1, eq53_e666_d_b2, eq53_e666_d_b3, eq53_e666_d_b4, eq53_e666_d_b5, eq53_e666_d_b6, eq53_e666_d_b7, eq53_e666_d_b8, eq53_e666_d_b9, eq53_e666_d_b10, eq53_e666_d_b11, eq53_e666_d_b12,) = {
    if ((s.v[1851] != 0.0) && (s.v[1852] != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664: f64 = self.eval_ddt(10, eq53_e663);
        let eq53_e664_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_n17: f64 = self.ddt_jacobian(eq53_e661);
        let eq53_e664_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b11: f64 = self.ddt_jacobian(0.0);
        let eq53_e664_d_b12: f64 = self.ddt_jacobian(0.0);
        (eq53_e664, eq53_e664_d_n0, eq53_e664_d_n1, eq53_e664_d_n2, eq53_e664_d_n3, eq53_e664_d_n4, eq53_e664_d_n5, eq53_e664_d_n6, eq53_e664_d_n7, eq53_e664_d_n8, eq53_e664_d_n9, eq53_e664_d_n10, eq53_e664_d_n11, eq53_e664_d_n12, eq53_e664_d_n13, eq53_e664_d_n14, eq53_e664_d_n15, eq53_e664_d_n16, eq53_e664_d_n17, eq53_e664_d_n18, eq53_e664_d_b0, eq53_e664_d_b1, eq53_e664_d_b2, eq53_e664_d_b3, eq53_e664_d_b4, eq53_e664_d_b5, eq53_e664_d_b6, eq53_e664_d_b7, eq53_e664_d_b8, eq53_e664_d_b9, eq53_e664_d_b10, eq53_e664_d_b11, eq53_e664_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e666;
        let eq53_node_derivatives: [f64; 19] = [eq53_e666_d_n0, eq53_e666_d_n1, eq53_e666_d_n2, eq53_e666_d_n3, eq53_e666_d_n4, eq53_e666_d_n5, eq53_e666_d_n6, eq53_e666_d_n7, eq53_e666_d_n8, eq53_e666_d_n9, eq53_e666_d_n10, eq53_e666_d_n11, eq53_e666_d_n12, eq53_e666_d_n13, eq53_e666_d_n14, eq53_e666_d_n15, eq53_e666_d_n16, eq53_e666_d_n17, eq53_e666_d_n18];
        let eq53_branch_derivatives: [f64; 13] = [eq53_e666_d_b0, eq53_e666_d_b1, eq53_e666_d_b2, eq53_e666_d_b3, eq53_e666_d_b4, eq53_e666_d_b5, eq53_e666_d_b6, eq53_e666_d_b7, eq53_e666_d_b8, eq53_e666_d_b9, eq53_e666_d_b10, eq53_e666_d_b11, eq53_e666_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq53_value),
            &nodes,
            &eq53_node_derivatives,
            &branches,
            &eq53_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_54_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq54_e673,) = {
    if ((s.v[1851] != 0.0) && (!(s.v[1852] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq54_value: f64 = eq54_e673;
        stamper.stamp_potential(
            branches[11],
            eq54_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_55_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq55_e682, eq55_e682_d_n0, eq55_e682_d_n1, eq55_e682_d_n2, eq55_e682_d_n3, eq55_e682_d_n4, eq55_e682_d_n5, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n8, eq55_e682_d_n9, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n13, eq55_e682_d_n14, eq55_e682_d_n15, eq55_e682_d_n16, eq55_e682_d_n17, eq55_e682_d_n18, eq55_e682_d_b0, eq55_e682_d_b1, eq55_e682_d_b2, eq55_e682_d_b3, eq55_e682_d_b4, eq55_e682_d_b5, eq55_e682_d_b6, eq55_e682_d_b7, eq55_e682_d_b8, eq55_e682_d_b9, eq55_e682_d_b10, eq55_e682_d_b11, eq55_e682_d_b12,) = {
    if (!(s.v[1851] != 0.0)) {
        let eq55_e679: f64 = (s.v[311] + s.v[263]);
        let eq55_e679_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq55_e679_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq55_e679_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq55_e679_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq55_e679_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq55_e679_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq55_e679_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq55_e679_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq55_e679_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq55_e679_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq55_e679_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq55_e679_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq55_e679_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq55_e679_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq55_e679_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq55_e679_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq55_e679_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq55_e679_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq55_e679_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq55_e679_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq55_e679_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq55_e679_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq55_e679_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq55_e679_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq55_e679_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq55_e679_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq55_e679_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq55_e679_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq55_e679_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq55_e679_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq55_e679_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq55_e679_d_b12: f64 = (s.db[311][12] + s.db[263][12]);
        let eq55_e680: f64 = (p.p50 * eq55_e679);
        let eq55_e680_d_n0: f64 = (p.p50 * eq55_e679_d_n0);
        let eq55_e680_d_n1: f64 = (p.p50 * eq55_e679_d_n1);
        let eq55_e680_d_n2: f64 = (p.p50 * eq55_e679_d_n2);
        let eq55_e680_d_n3: f64 = (p.p50 * eq55_e679_d_n3);
        let eq55_e680_d_n4: f64 = (p.p50 * eq55_e679_d_n4);
        let eq55_e680_d_n5: f64 = (p.p50 * eq55_e679_d_n5);
        let eq55_e680_d_n6: f64 = (p.p50 * eq55_e679_d_n6);
        let eq55_e680_d_n7: f64 = (p.p50 * eq55_e679_d_n7);
        let eq55_e680_d_n8: f64 = (p.p50 * eq55_e679_d_n8);
        let eq55_e680_d_n9: f64 = (p.p50 * eq55_e679_d_n9);
        let eq55_e680_d_n10: f64 = (p.p50 * eq55_e679_d_n10);
        let eq55_e680_d_n11: f64 = (p.p50 * eq55_e679_d_n11);
        let eq55_e680_d_n12: f64 = (p.p50 * eq55_e679_d_n12);
        let eq55_e680_d_n13: f64 = (p.p50 * eq55_e679_d_n13);
        let eq55_e680_d_n14: f64 = (p.p50 * eq55_e679_d_n14);
        let eq55_e680_d_n15: f64 = (p.p50 * eq55_e679_d_n15);
        let eq55_e680_d_n16: f64 = (p.p50 * eq55_e679_d_n16);
        let eq55_e680_d_n17: f64 = (p.p50 * eq55_e679_d_n17);
        let eq55_e680_d_n18: f64 = (p.p50 * eq55_e679_d_n18);
        let eq55_e680_d_b0: f64 = (p.p50 * eq55_e679_d_b0);
        let eq55_e680_d_b1: f64 = (p.p50 * eq55_e679_d_b1);
        let eq55_e680_d_b2: f64 = (p.p50 * eq55_e679_d_b2);
        let eq55_e680_d_b3: f64 = (p.p50 * eq55_e679_d_b3);
        let eq55_e680_d_b4: f64 = (p.p50 * eq55_e679_d_b4);
        let eq55_e680_d_b5: f64 = (p.p50 * eq55_e679_d_b5);
        let eq55_e680_d_b6: f64 = (p.p50 * eq55_e679_d_b6);
        let eq55_e680_d_b7: f64 = (p.p50 * eq55_e679_d_b7);
        let eq55_e680_d_b8: f64 = (p.p50 * eq55_e679_d_b8);
        let eq55_e680_d_b9: f64 = (p.p50 * eq55_e679_d_b9);
        let eq55_e680_d_b10: f64 = (p.p50 * eq55_e679_d_b10);
        let eq55_e680_d_b11: f64 = (p.p50 * eq55_e679_d_b11);
        let eq55_e680_d_b12: f64 = (p.p50 * eq55_e679_d_b12);
        (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n1, eq55_e680_d_n2, eq55_e680_d_n3, eq55_e680_d_n4, eq55_e680_d_n5, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n8, eq55_e680_d_n9, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n13, eq55_e680_d_n14, eq55_e680_d_n15, eq55_e680_d_n16, eq55_e680_d_n17, eq55_e680_d_n18, eq55_e680_d_b0, eq55_e680_d_b1, eq55_e680_d_b2, eq55_e680_d_b3, eq55_e680_d_b4, eq55_e680_d_b5, eq55_e680_d_b6, eq55_e680_d_b7, eq55_e680_d_b8, eq55_e680_d_b9, eq55_e680_d_b10, eq55_e680_d_b11, eq55_e680_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e682;
        let eq55_node_derivatives: [f64; 19] = [eq55_e682_d_n0, eq55_e682_d_n1, eq55_e682_d_n2, eq55_e682_d_n3, eq55_e682_d_n4, eq55_e682_d_n5, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n8, eq55_e682_d_n9, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n13, eq55_e682_d_n14, eq55_e682_d_n15, eq55_e682_d_n16, eq55_e682_d_n17, eq55_e682_d_n18];
        let eq55_branch_derivatives: [f64; 13] = [eq55_e682_d_b0, eq55_e682_d_b1, eq55_e682_d_b2, eq55_e682_d_b3, eq55_e682_d_b4, eq55_e682_d_b5, eq55_e682_d_b6, eq55_e682_d_b7, eq55_e682_d_b8, eq55_e682_d_b9, eq55_e682_d_b10, eq55_e682_d_b11, eq55_e682_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq55_value),
            &nodes,
            &eq55_node_derivatives,
            &branches,
            &eq55_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq56_e691, eq56_e691_d_n0, eq56_e691_d_n1, eq56_e691_d_n2, eq56_e691_d_n3, eq56_e691_d_n4, eq56_e691_d_n5, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n8, eq56_e691_d_n9, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n13, eq56_e691_d_n14, eq56_e691_d_n15, eq56_e691_d_n16, eq56_e691_d_n17, eq56_e691_d_n18, eq56_e691_d_b0, eq56_e691_d_b1, eq56_e691_d_b2, eq56_e691_d_b3, eq56_e691_d_b4, eq56_e691_d_b5, eq56_e691_d_b6, eq56_e691_d_b7, eq56_e691_d_b8, eq56_e691_d_b9, eq56_e691_d_b10, eq56_e691_d_b11, eq56_e691_d_b12,) = {
    if (!(s.v[1851] != 0.0)) {
        let eq56_e688: f64 = (s.v[312] + s.v[573]);
        let eq56_e688_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq56_e688_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq56_e688_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq56_e688_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq56_e688_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq56_e688_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq56_e688_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq56_e688_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq56_e688_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq56_e688_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq56_e688_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq56_e688_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq56_e688_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq56_e688_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq56_e688_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq56_e688_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq56_e688_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq56_e688_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq56_e688_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq56_e688_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq56_e688_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq56_e688_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq56_e688_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq56_e688_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq56_e688_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq56_e688_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq56_e688_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq56_e688_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq56_e688_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq56_e688_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq56_e688_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq56_e688_d_b12: f64 = (s.db[312][12] + s.db[573][12]);
        let eq56_e689: f64 = (p.p50 * eq56_e688);
        let eq56_e689_d_n0: f64 = (p.p50 * eq56_e688_d_n0);
        let eq56_e689_d_n1: f64 = (p.p50 * eq56_e688_d_n1);
        let eq56_e689_d_n2: f64 = (p.p50 * eq56_e688_d_n2);
        let eq56_e689_d_n3: f64 = (p.p50 * eq56_e688_d_n3);
        let eq56_e689_d_n4: f64 = (p.p50 * eq56_e688_d_n4);
        let eq56_e689_d_n5: f64 = (p.p50 * eq56_e688_d_n5);
        let eq56_e689_d_n6: f64 = (p.p50 * eq56_e688_d_n6);
        let eq56_e689_d_n7: f64 = (p.p50 * eq56_e688_d_n7);
        let eq56_e689_d_n8: f64 = (p.p50 * eq56_e688_d_n8);
        let eq56_e689_d_n9: f64 = (p.p50 * eq56_e688_d_n9);
        let eq56_e689_d_n10: f64 = (p.p50 * eq56_e688_d_n10);
        let eq56_e689_d_n11: f64 = (p.p50 * eq56_e688_d_n11);
        let eq56_e689_d_n12: f64 = (p.p50 * eq56_e688_d_n12);
        let eq56_e689_d_n13: f64 = (p.p50 * eq56_e688_d_n13);
        let eq56_e689_d_n14: f64 = (p.p50 * eq56_e688_d_n14);
        let eq56_e689_d_n15: f64 = (p.p50 * eq56_e688_d_n15);
        let eq56_e689_d_n16: f64 = (p.p50 * eq56_e688_d_n16);
        let eq56_e689_d_n17: f64 = (p.p50 * eq56_e688_d_n17);
        let eq56_e689_d_n18: f64 = (p.p50 * eq56_e688_d_n18);
        let eq56_e689_d_b0: f64 = (p.p50 * eq56_e688_d_b0);
        let eq56_e689_d_b1: f64 = (p.p50 * eq56_e688_d_b1);
        let eq56_e689_d_b2: f64 = (p.p50 * eq56_e688_d_b2);
        let eq56_e689_d_b3: f64 = (p.p50 * eq56_e688_d_b3);
        let eq56_e689_d_b4: f64 = (p.p50 * eq56_e688_d_b4);
        let eq56_e689_d_b5: f64 = (p.p50 * eq56_e688_d_b5);
        let eq56_e689_d_b6: f64 = (p.p50 * eq56_e688_d_b6);
        let eq56_e689_d_b7: f64 = (p.p50 * eq56_e688_d_b7);
        let eq56_e689_d_b8: f64 = (p.p50 * eq56_e688_d_b8);
        let eq56_e689_d_b9: f64 = (p.p50 * eq56_e688_d_b9);
        let eq56_e689_d_b10: f64 = (p.p50 * eq56_e688_d_b10);
        let eq56_e689_d_b11: f64 = (p.p50 * eq56_e688_d_b11);
        let eq56_e689_d_b12: f64 = (p.p50 * eq56_e688_d_b12);
        (eq56_e689, eq56_e689_d_n0, eq56_e689_d_n1, eq56_e689_d_n2, eq56_e689_d_n3, eq56_e689_d_n4, eq56_e689_d_n5, eq56_e689_d_n6, eq56_e689_d_n7, eq56_e689_d_n8, eq56_e689_d_n9, eq56_e689_d_n10, eq56_e689_d_n11, eq56_e689_d_n12, eq56_e689_d_n13, eq56_e689_d_n14, eq56_e689_d_n15, eq56_e689_d_n16, eq56_e689_d_n17, eq56_e689_d_n18, eq56_e689_d_b0, eq56_e689_d_b1, eq56_e689_d_b2, eq56_e689_d_b3, eq56_e689_d_b4, eq56_e689_d_b5, eq56_e689_d_b6, eq56_e689_d_b7, eq56_e689_d_b8, eq56_e689_d_b9, eq56_e689_d_b10, eq56_e689_d_b11, eq56_e689_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e691;
        let eq56_node_derivatives: [f64; 19] = [eq56_e691_d_n0, eq56_e691_d_n1, eq56_e691_d_n2, eq56_e691_d_n3, eq56_e691_d_n4, eq56_e691_d_n5, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n8, eq56_e691_d_n9, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n13, eq56_e691_d_n14, eq56_e691_d_n15, eq56_e691_d_n16, eq56_e691_d_n17, eq56_e691_d_n18];
        let eq56_branch_derivatives: [f64; 13] = [eq56_e691_d_b0, eq56_e691_d_b1, eq56_e691_d_b2, eq56_e691_d_b3, eq56_e691_d_b4, eq56_e691_d_b5, eq56_e691_d_b6, eq56_e691_d_b7, eq56_e691_d_b8, eq56_e691_d_b9, eq56_e691_d_b10, eq56_e691_d_b11, eq56_e691_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq56_value),
            &nodes,
            &eq56_node_derivatives,
            &branches,
            &eq56_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_57_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq57_e696,) = {
    if (!(s.v[1851] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq57_value: f64 = eq57_e696;
        stamper.stamp_potential(
            branches[12],
            eq57_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_58_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq58_e703, eq58_e703_d_n0, eq58_e703_d_n1, eq58_e703_d_n2, eq58_e703_d_n3, eq58_e703_d_n4, eq58_e703_d_n5, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n8, eq58_e703_d_n9, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n13, eq58_e703_d_n14, eq58_e703_d_n15, eq58_e703_d_n16, eq58_e703_d_n17, eq58_e703_d_n18, eq58_e703_d_b0, eq58_e703_d_b1, eq58_e703_d_b2, eq58_e703_d_b3, eq58_e703_d_b4, eq58_e703_d_b5, eq58_e703_d_b6, eq58_e703_d_b7, eq58_e703_d_b8, eq58_e703_d_b9, eq58_e703_d_b10, eq58_e703_d_b11, eq58_e703_d_b12,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11], s.db[592][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e703;
        let eq58_node_derivatives: [f64; 19] = [eq58_e703_d_n0, eq58_e703_d_n1, eq58_e703_d_n2, eq58_e703_d_n3, eq58_e703_d_n4, eq58_e703_d_n5, eq58_e703_d_n6, eq58_e703_d_n7, eq58_e703_d_n8, eq58_e703_d_n9, eq58_e703_d_n10, eq58_e703_d_n11, eq58_e703_d_n12, eq58_e703_d_n13, eq58_e703_d_n14, eq58_e703_d_n15, eq58_e703_d_n16, eq58_e703_d_n17, eq58_e703_d_n18];
        let eq58_branch_derivatives: [f64; 13] = [eq58_e703_d_b0, eq58_e703_d_b1, eq58_e703_d_b2, eq58_e703_d_b3, eq58_e703_d_b4, eq58_e703_d_b5, eq58_e703_d_b6, eq58_e703_d_b7, eq58_e703_d_b8, eq58_e703_d_b9, eq58_e703_d_b10, eq58_e703_d_b11, eq58_e703_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq58_value),
            &nodes,
            &eq58_node_derivatives,
            &branches,
            &eq58_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_59_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq59_e712, eq59_e712_d_n17,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p37 != 0.0)) {
        let eq59_e710: f64 = ((nv17 - 0.0) * 1e-12);
        let eq59_e710_d_n17: f64 = 1e-12;
        (eq59_e710, eq59_e710_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e712;
        stamper.stamp_current(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq59_value),
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * eq59_e712_d_n17),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_60_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq60_e724, eq60_e724_d_n0, eq60_e724_d_n1, eq60_e724_d_n2, eq60_e724_d_n3, eq60_e724_d_n4, eq60_e724_d_n5, eq60_e724_d_n6, eq60_e724_d_n7, eq60_e724_d_n8, eq60_e724_d_n9, eq60_e724_d_n10, eq60_e724_d_n11, eq60_e724_d_n12, eq60_e724_d_n13, eq60_e724_d_n14, eq60_e724_d_n15, eq60_e724_d_n16, eq60_e724_d_n17, eq60_e724_d_n18, eq60_e724_d_b0, eq60_e724_d_b1, eq60_e724_d_b2, eq60_e724_d_b3, eq60_e724_d_b4, eq60_e724_d_b5, eq60_e724_d_b6, eq60_e724_d_b7, eq60_e724_d_b8, eq60_e724_d_b9, eq60_e724_d_b10, eq60_e724_d_b11, eq60_e724_d_b12,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722: f64 = self.eval_ddt(11, eq60_e721);
        let eq60_e722_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_n17: f64 = self.ddt_jacobian(eq60_e719);
        let eq60_e722_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b11: f64 = self.ddt_jacobian(0.0);
        let eq60_e722_d_b12: f64 = self.ddt_jacobian(0.0);
        (eq60_e722, eq60_e722_d_n0, eq60_e722_d_n1, eq60_e722_d_n2, eq60_e722_d_n3, eq60_e722_d_n4, eq60_e722_d_n5, eq60_e722_d_n6, eq60_e722_d_n7, eq60_e722_d_n8, eq60_e722_d_n9, eq60_e722_d_n10, eq60_e722_d_n11, eq60_e722_d_n12, eq60_e722_d_n13, eq60_e722_d_n14, eq60_e722_d_n15, eq60_e722_d_n16, eq60_e722_d_n17, eq60_e722_d_n18, eq60_e722_d_b0, eq60_e722_d_b1, eq60_e722_d_b2, eq60_e722_d_b3, eq60_e722_d_b4, eq60_e722_d_b5, eq60_e722_d_b6, eq60_e722_d_b7, eq60_e722_d_b8, eq60_e722_d_b9, eq60_e722_d_b10, eq60_e722_d_b11, eq60_e722_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        let eq60_node_derivatives: [f64; 19] = [eq60_e724_d_n0, eq60_e724_d_n1, eq60_e724_d_n2, eq60_e724_d_n3, eq60_e724_d_n4, eq60_e724_d_n5, eq60_e724_d_n6, eq60_e724_d_n7, eq60_e724_d_n8, eq60_e724_d_n9, eq60_e724_d_n10, eq60_e724_d_n11, eq60_e724_d_n12, eq60_e724_d_n13, eq60_e724_d_n14, eq60_e724_d_n15, eq60_e724_d_n16, eq60_e724_d_n17, eq60_e724_d_n18];
        let eq60_branch_derivatives: [f64; 13] = [eq60_e724_d_b0, eq60_e724_d_b1, eq60_e724_d_b2, eq60_e724_d_b3, eq60_e724_d_b4, eq60_e724_d_b5, eq60_e724_d_b6, eq60_e724_d_b7, eq60_e724_d_b8, eq60_e724_d_b9, eq60_e724_d_b10, eq60_e724_d_b11, eq60_e724_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq60_value),
            &nodes,
            &eq60_node_derivatives,
            &branches,
            &eq60_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_61_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq61_e732,) = {
    if ((!(s.v[1851] != 0.0)) && (!(p.p37 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e732;
        stamper.stamp_potential(
            branches[13],
            eq61_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_62_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq62_e739, eq62_e739_d_n0, eq62_e739_d_n1, eq62_e739_d_n2, eq62_e739_d_n3, eq62_e739_d_n4, eq62_e739_d_n5, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n8, eq62_e739_d_n9, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n14, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18, eq62_e739_d_b0, eq62_e739_d_b1, eq62_e739_d_b2, eq62_e739_d_b3, eq62_e739_d_b4, eq62_e739_d_b5, eq62_e739_d_b6, eq62_e739_d_b7, eq62_e739_d_b8, eq62_e739_d_b9, eq62_e739_d_b10, eq62_e739_d_b11, eq62_e739_d_b12,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11], s.db[574][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e739;
        let eq62_node_derivatives: [f64; 19] = [eq62_e739_d_n0, eq62_e739_d_n1, eq62_e739_d_n2, eq62_e739_d_n3, eq62_e739_d_n4, eq62_e739_d_n5, eq62_e739_d_n6, eq62_e739_d_n7, eq62_e739_d_n8, eq62_e739_d_n9, eq62_e739_d_n10, eq62_e739_d_n11, eq62_e739_d_n12, eq62_e739_d_n13, eq62_e739_d_n14, eq62_e739_d_n15, eq62_e739_d_n16, eq62_e739_d_n17, eq62_e739_d_n18];
        let eq62_branch_derivatives: [f64; 13] = [eq62_e739_d_b0, eq62_e739_d_b1, eq62_e739_d_b2, eq62_e739_d_b3, eq62_e739_d_b4, eq62_e739_d_b5, eq62_e739_d_b6, eq62_e739_d_b7, eq62_e739_d_b8, eq62_e739_d_b9, eq62_e739_d_b10, eq62_e739_d_b11, eq62_e739_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq62_value),
            &nodes,
            &eq62_node_derivatives,
            &branches,
            &eq62_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_63_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq63_e746, eq63_e746_d_n0, eq63_e746_d_n1, eq63_e746_d_n2, eq63_e746_d_n3, eq63_e746_d_n4, eq63_e746_d_n5, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n8, eq63_e746_d_n9, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n14, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18, eq63_e746_d_b0, eq63_e746_d_b1, eq63_e746_d_b2, eq63_e746_d_b3, eq63_e746_d_b4, eq63_e746_d_b5, eq63_e746_d_b6, eq63_e746_d_b7, eq63_e746_d_b8, eq63_e746_d_b9, eq63_e746_d_b10, eq63_e746_d_b11, eq63_e746_d_b12,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11], s.db[575][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e746;
        let eq63_node_derivatives: [f64; 19] = [eq63_e746_d_n0, eq63_e746_d_n1, eq63_e746_d_n2, eq63_e746_d_n3, eq63_e746_d_n4, eq63_e746_d_n5, eq63_e746_d_n6, eq63_e746_d_n7, eq63_e746_d_n8, eq63_e746_d_n9, eq63_e746_d_n10, eq63_e746_d_n11, eq63_e746_d_n12, eq63_e746_d_n13, eq63_e746_d_n14, eq63_e746_d_n15, eq63_e746_d_n16, eq63_e746_d_n17, eq63_e746_d_n18];
        let eq63_branch_derivatives: [f64; 13] = [eq63_e746_d_b0, eq63_e746_d_b1, eq63_e746_d_b2, eq63_e746_d_b3, eq63_e746_d_b4, eq63_e746_d_b5, eq63_e746_d_b6, eq63_e746_d_b7, eq63_e746_d_b8, eq63_e746_d_b9, eq63_e746_d_b10, eq63_e746_d_b11, eq63_e746_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            None,
            self.multiplicity * (eq63_value),
            &nodes,
            &eq63_node_derivatives,
            &branches,
            &eq63_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_64_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq64_e753, eq64_e753_d_n0, eq64_e753_d_n1, eq64_e753_d_n2, eq64_e753_d_n3, eq64_e753_d_n4, eq64_e753_d_n5, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n8, eq64_e753_d_n9, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n14, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18, eq64_e753_d_b0, eq64_e753_d_b1, eq64_e753_d_b2, eq64_e753_d_b3, eq64_e753_d_b4, eq64_e753_d_b5, eq64_e753_d_b6, eq64_e753_d_b7, eq64_e753_d_b8, eq64_e753_d_b9, eq64_e753_d_b10, eq64_e753_d_b11, eq64_e753_d_b12,) = {
    if ((!(s.v[1851] != 0.0)) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11], s.db[583][12],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e753;
        let eq64_node_derivatives: [f64; 19] = [eq64_e753_d_n0, eq64_e753_d_n1, eq64_e753_d_n2, eq64_e753_d_n3, eq64_e753_d_n4, eq64_e753_d_n5, eq64_e753_d_n6, eq64_e753_d_n7, eq64_e753_d_n8, eq64_e753_d_n9, eq64_e753_d_n10, eq64_e753_d_n11, eq64_e753_d_n12, eq64_e753_d_n13, eq64_e753_d_n14, eq64_e753_d_n15, eq64_e753_d_n16, eq64_e753_d_n17, eq64_e753_d_n18];
        let eq64_branch_derivatives: [f64; 13] = [eq64_e753_d_b0, eq64_e753_d_b1, eq64_e753_d_b2, eq64_e753_d_b3, eq64_e753_d_b4, eq64_e753_d_b5, eq64_e753_d_b6, eq64_e753_d_b7, eq64_e753_d_b8, eq64_e753_d_b9, eq64_e753_d_b10, eq64_e753_d_b11, eq64_e753_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq64_value),
            &nodes,
            &eq64_node_derivatives,
            &branches,
            &eq64_branch_derivatives,
            self.multiplicity,
        );
    }
}

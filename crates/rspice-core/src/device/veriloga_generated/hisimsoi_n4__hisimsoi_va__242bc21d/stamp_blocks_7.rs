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
        let (eq49_e630,) = {
    if ((s.v[1847] != 0.0) && (!(p.p34 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e630;
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
        let (eq50_e636, eq50_e636_d_n0, eq50_e636_d_n1, eq50_e636_d_n2, eq50_e636_d_n3, eq50_e636_d_n4, eq50_e636_d_n5, eq50_e636_d_n6, eq50_e636_d_n7, eq50_e636_d_n8, eq50_e636_d_n9, eq50_e636_d_n10, eq50_e636_d_n11, eq50_e636_d_n12, eq50_e636_d_n13, eq50_e636_d_n14, eq50_e636_d_n15, eq50_e636_d_n16, eq50_e636_d_n17, eq50_e636_d_n18, eq50_e636_d_b0, eq50_e636_d_b1, eq50_e636_d_b2, eq50_e636_d_b3, eq50_e636_d_b4, eq50_e636_d_b5, eq50_e636_d_b6, eq50_e636_d_b7, eq50_e636_d_b8, eq50_e636_d_b9, eq50_e636_d_b10, eq50_e636_d_b11,) = {
    if ((s.v[1847] != 0.0) && (s.v[1848] != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e636;
        let eq50_node_derivatives: [f64; 19] = [eq50_e636_d_n0, eq50_e636_d_n1, eq50_e636_d_n2, eq50_e636_d_n3, eq50_e636_d_n4, eq50_e636_d_n5, eq50_e636_d_n6, eq50_e636_d_n7, eq50_e636_d_n8, eq50_e636_d_n9, eq50_e636_d_n10, eq50_e636_d_n11, eq50_e636_d_n12, eq50_e636_d_n13, eq50_e636_d_n14, eq50_e636_d_n15, eq50_e636_d_n16, eq50_e636_d_n17, eq50_e636_d_n18];
        let eq50_branch_derivatives: [f64; 12] = [eq50_e636_d_b0, eq50_e636_d_b1, eq50_e636_d_b2, eq50_e636_d_b3, eq50_e636_d_b4, eq50_e636_d_b5, eq50_e636_d_b6, eq50_e636_d_b7, eq50_e636_d_b8, eq50_e636_d_b9, eq50_e636_d_b10, eq50_e636_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq50_value),
            &nodes,
            &eq50_node_derivatives,
            &branches,
            &eq50_branch_derivatives,
            self.multiplicity,
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
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq51_e644, eq51_e644_d_n17,) = {
    if ((s.v[1847] != 0.0) && (s.v[1848] != 0.0)) {
        let eq51_e642: f64 = ((nv17 - 0.0) * 1e-12);
        let eq51_e642_d_n17: f64 = 1e-12;
        (eq51_e642, eq51_e642_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e644;
        stamper.stamp_current(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq51_value),
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * eq51_e644_d_n17),
            ],
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
        let (eq52_e655, eq52_e655_d_n0, eq52_e655_d_n1, eq52_e655_d_n2, eq52_e655_d_n3, eq52_e655_d_n4, eq52_e655_d_n5, eq52_e655_d_n6, eq52_e655_d_n7, eq52_e655_d_n8, eq52_e655_d_n9, eq52_e655_d_n10, eq52_e655_d_n11, eq52_e655_d_n12, eq52_e655_d_n13, eq52_e655_d_n14, eq52_e655_d_n15, eq52_e655_d_n16, eq52_e655_d_n17, eq52_e655_d_n18, eq52_e655_d_b0, eq52_e655_d_b1, eq52_e655_d_b2, eq52_e655_d_b3, eq52_e655_d_b4, eq52_e655_d_b5, eq52_e655_d_b6, eq52_e655_d_b7, eq52_e655_d_b8, eq52_e655_d_b9, eq52_e655_d_b10, eq52_e655_d_b11,) = {
    if ((s.v[1847] != 0.0) && (s.v[1848] != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653: f64 = self.eval_ddt(10, eq52_e652);
        let eq52_e653_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_n17: f64 = self.ddt_jacobian(eq52_e650);
        let eq52_e653_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq52_e653_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq52_e653, eq52_e653_d_n0, eq52_e653_d_n1, eq52_e653_d_n2, eq52_e653_d_n3, eq52_e653_d_n4, eq52_e653_d_n5, eq52_e653_d_n6, eq52_e653_d_n7, eq52_e653_d_n8, eq52_e653_d_n9, eq52_e653_d_n10, eq52_e653_d_n11, eq52_e653_d_n12, eq52_e653_d_n13, eq52_e653_d_n14, eq52_e653_d_n15, eq52_e653_d_n16, eq52_e653_d_n17, eq52_e653_d_n18, eq52_e653_d_b0, eq52_e653_d_b1, eq52_e653_d_b2, eq52_e653_d_b3, eq52_e653_d_b4, eq52_e653_d_b5, eq52_e653_d_b6, eq52_e653_d_b7, eq52_e653_d_b8, eq52_e653_d_b9, eq52_e653_d_b10, eq52_e653_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e655;
        let eq52_node_derivatives: [f64; 19] = [eq52_e655_d_n0, eq52_e655_d_n1, eq52_e655_d_n2, eq52_e655_d_n3, eq52_e655_d_n4, eq52_e655_d_n5, eq52_e655_d_n6, eq52_e655_d_n7, eq52_e655_d_n8, eq52_e655_d_n9, eq52_e655_d_n10, eq52_e655_d_n11, eq52_e655_d_n12, eq52_e655_d_n13, eq52_e655_d_n14, eq52_e655_d_n15, eq52_e655_d_n16, eq52_e655_d_n17, eq52_e655_d_n18];
        let eq52_branch_derivatives: [f64; 12] = [eq52_e655_d_b0, eq52_e655_d_b1, eq52_e655_d_b2, eq52_e655_d_b3, eq52_e655_d_b4, eq52_e655_d_b5, eq52_e655_d_b6, eq52_e655_d_b7, eq52_e655_d_b8, eq52_e655_d_b9, eq52_e655_d_b10, eq52_e655_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq52_value),
            &nodes,
            &eq52_node_derivatives,
            &branches,
            &eq52_branch_derivatives,
            self.multiplicity,
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
        let (eq53_e662,) = {
    if ((s.v[1847] != 0.0) && (!(s.v[1848] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq53_value: f64 = eq53_e662;
        stamper.stamp_potential(
            branches[10],
            eq53_value,
            &[
            ],
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
        let (eq54_e671, eq54_e671_d_n0, eq54_e671_d_n1, eq54_e671_d_n2, eq54_e671_d_n3, eq54_e671_d_n4, eq54_e671_d_n5, eq54_e671_d_n6, eq54_e671_d_n7, eq54_e671_d_n8, eq54_e671_d_n9, eq54_e671_d_n10, eq54_e671_d_n11, eq54_e671_d_n12, eq54_e671_d_n13, eq54_e671_d_n14, eq54_e671_d_n15, eq54_e671_d_n16, eq54_e671_d_n17, eq54_e671_d_n18, eq54_e671_d_b0, eq54_e671_d_b1, eq54_e671_d_b2, eq54_e671_d_b3, eq54_e671_d_b4, eq54_e671_d_b5, eq54_e671_d_b6, eq54_e671_d_b7, eq54_e671_d_b8, eq54_e671_d_b9, eq54_e671_d_b10, eq54_e671_d_b11,) = {
    if (!(s.v[1847] != 0.0)) {
        let eq54_e668: f64 = (s.v[311] + s.v[263]);
        let eq54_e668_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq54_e668_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq54_e668_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq54_e668_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq54_e668_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq54_e668_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq54_e668_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq54_e668_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq54_e668_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq54_e668_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq54_e668_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq54_e668_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq54_e668_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq54_e668_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq54_e668_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq54_e668_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq54_e668_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq54_e668_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq54_e668_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq54_e668_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq54_e668_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq54_e668_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq54_e668_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq54_e668_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq54_e668_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq54_e668_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq54_e668_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq54_e668_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq54_e668_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq54_e668_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq54_e668_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq54_e669: f64 = (p.p50 * eq54_e668);
        let eq54_e669_d_n0: f64 = (p.p50 * eq54_e668_d_n0);
        let eq54_e669_d_n1: f64 = (p.p50 * eq54_e668_d_n1);
        let eq54_e669_d_n2: f64 = (p.p50 * eq54_e668_d_n2);
        let eq54_e669_d_n3: f64 = (p.p50 * eq54_e668_d_n3);
        let eq54_e669_d_n4: f64 = (p.p50 * eq54_e668_d_n4);
        let eq54_e669_d_n5: f64 = (p.p50 * eq54_e668_d_n5);
        let eq54_e669_d_n6: f64 = (p.p50 * eq54_e668_d_n6);
        let eq54_e669_d_n7: f64 = (p.p50 * eq54_e668_d_n7);
        let eq54_e669_d_n8: f64 = (p.p50 * eq54_e668_d_n8);
        let eq54_e669_d_n9: f64 = (p.p50 * eq54_e668_d_n9);
        let eq54_e669_d_n10: f64 = (p.p50 * eq54_e668_d_n10);
        let eq54_e669_d_n11: f64 = (p.p50 * eq54_e668_d_n11);
        let eq54_e669_d_n12: f64 = (p.p50 * eq54_e668_d_n12);
        let eq54_e669_d_n13: f64 = (p.p50 * eq54_e668_d_n13);
        let eq54_e669_d_n14: f64 = (p.p50 * eq54_e668_d_n14);
        let eq54_e669_d_n15: f64 = (p.p50 * eq54_e668_d_n15);
        let eq54_e669_d_n16: f64 = (p.p50 * eq54_e668_d_n16);
        let eq54_e669_d_n17: f64 = (p.p50 * eq54_e668_d_n17);
        let eq54_e669_d_n18: f64 = (p.p50 * eq54_e668_d_n18);
        let eq54_e669_d_b0: f64 = (p.p50 * eq54_e668_d_b0);
        let eq54_e669_d_b1: f64 = (p.p50 * eq54_e668_d_b1);
        let eq54_e669_d_b2: f64 = (p.p50 * eq54_e668_d_b2);
        let eq54_e669_d_b3: f64 = (p.p50 * eq54_e668_d_b3);
        let eq54_e669_d_b4: f64 = (p.p50 * eq54_e668_d_b4);
        let eq54_e669_d_b5: f64 = (p.p50 * eq54_e668_d_b5);
        let eq54_e669_d_b6: f64 = (p.p50 * eq54_e668_d_b6);
        let eq54_e669_d_b7: f64 = (p.p50 * eq54_e668_d_b7);
        let eq54_e669_d_b8: f64 = (p.p50 * eq54_e668_d_b8);
        let eq54_e669_d_b9: f64 = (p.p50 * eq54_e668_d_b9);
        let eq54_e669_d_b10: f64 = (p.p50 * eq54_e668_d_b10);
        let eq54_e669_d_b11: f64 = (p.p50 * eq54_e668_d_b11);
        (eq54_e669, eq54_e669_d_n0, eq54_e669_d_n1, eq54_e669_d_n2, eq54_e669_d_n3, eq54_e669_d_n4, eq54_e669_d_n5, eq54_e669_d_n6, eq54_e669_d_n7, eq54_e669_d_n8, eq54_e669_d_n9, eq54_e669_d_n10, eq54_e669_d_n11, eq54_e669_d_n12, eq54_e669_d_n13, eq54_e669_d_n14, eq54_e669_d_n15, eq54_e669_d_n16, eq54_e669_d_n17, eq54_e669_d_n18, eq54_e669_d_b0, eq54_e669_d_b1, eq54_e669_d_b2, eq54_e669_d_b3, eq54_e669_d_b4, eq54_e669_d_b5, eq54_e669_d_b6, eq54_e669_d_b7, eq54_e669_d_b8, eq54_e669_d_b9, eq54_e669_d_b10, eq54_e669_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e671;
        let eq54_node_derivatives: [f64; 19] = [eq54_e671_d_n0, eq54_e671_d_n1, eq54_e671_d_n2, eq54_e671_d_n3, eq54_e671_d_n4, eq54_e671_d_n5, eq54_e671_d_n6, eq54_e671_d_n7, eq54_e671_d_n8, eq54_e671_d_n9, eq54_e671_d_n10, eq54_e671_d_n11, eq54_e671_d_n12, eq54_e671_d_n13, eq54_e671_d_n14, eq54_e671_d_n15, eq54_e671_d_n16, eq54_e671_d_n17, eq54_e671_d_n18];
        let eq54_branch_derivatives: [f64; 12] = [eq54_e671_d_b0, eq54_e671_d_b1, eq54_e671_d_b2, eq54_e671_d_b3, eq54_e671_d_b4, eq54_e671_d_b5, eq54_e671_d_b6, eq54_e671_d_b7, eq54_e671_d_b8, eq54_e671_d_b9, eq54_e671_d_b10, eq54_e671_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq54_value),
            &nodes,
            &eq54_node_derivatives,
            &branches,
            &eq54_branch_derivatives,
            self.multiplicity,
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
        let (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n1, eq55_e680_d_n2, eq55_e680_d_n3, eq55_e680_d_n4, eq55_e680_d_n5, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n8, eq55_e680_d_n9, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n13, eq55_e680_d_n14, eq55_e680_d_n15, eq55_e680_d_n16, eq55_e680_d_n17, eq55_e680_d_n18, eq55_e680_d_b0, eq55_e680_d_b1, eq55_e680_d_b2, eq55_e680_d_b3, eq55_e680_d_b4, eq55_e680_d_b5, eq55_e680_d_b6, eq55_e680_d_b7, eq55_e680_d_b8, eq55_e680_d_b9, eq55_e680_d_b10, eq55_e680_d_b11,) = {
    if (!(s.v[1847] != 0.0)) {
        let eq55_e677: f64 = (s.v[312] + s.v[573]);
        let eq55_e677_d_n0: f64 = (s.dn[312][0] + s.dn[573][0]);
        let eq55_e677_d_n1: f64 = (s.dn[312][1] + s.dn[573][1]);
        let eq55_e677_d_n2: f64 = (s.dn[312][2] + s.dn[573][2]);
        let eq55_e677_d_n3: f64 = (s.dn[312][3] + s.dn[573][3]);
        let eq55_e677_d_n4: f64 = (s.dn[312][4] + s.dn[573][4]);
        let eq55_e677_d_n5: f64 = (s.dn[312][5] + s.dn[573][5]);
        let eq55_e677_d_n6: f64 = (s.dn[312][6] + s.dn[573][6]);
        let eq55_e677_d_n7: f64 = (s.dn[312][7] + s.dn[573][7]);
        let eq55_e677_d_n8: f64 = (s.dn[312][8] + s.dn[573][8]);
        let eq55_e677_d_n9: f64 = (s.dn[312][9] + s.dn[573][9]);
        let eq55_e677_d_n10: f64 = (s.dn[312][10] + s.dn[573][10]);
        let eq55_e677_d_n11: f64 = (s.dn[312][11] + s.dn[573][11]);
        let eq55_e677_d_n12: f64 = (s.dn[312][12] + s.dn[573][12]);
        let eq55_e677_d_n13: f64 = (s.dn[312][13] + s.dn[573][13]);
        let eq55_e677_d_n14: f64 = (s.dn[312][14] + s.dn[573][14]);
        let eq55_e677_d_n15: f64 = (s.dn[312][15] + s.dn[573][15]);
        let eq55_e677_d_n16: f64 = (s.dn[312][16] + s.dn[573][16]);
        let eq55_e677_d_n17: f64 = (s.dn[312][17] + s.dn[573][17]);
        let eq55_e677_d_n18: f64 = (s.dn[312][18] + s.dn[573][18]);
        let eq55_e677_d_b0: f64 = (s.db[312][0] + s.db[573][0]);
        let eq55_e677_d_b1: f64 = (s.db[312][1] + s.db[573][1]);
        let eq55_e677_d_b2: f64 = (s.db[312][2] + s.db[573][2]);
        let eq55_e677_d_b3: f64 = (s.db[312][3] + s.db[573][3]);
        let eq55_e677_d_b4: f64 = (s.db[312][4] + s.db[573][4]);
        let eq55_e677_d_b5: f64 = (s.db[312][5] + s.db[573][5]);
        let eq55_e677_d_b6: f64 = (s.db[312][6] + s.db[573][6]);
        let eq55_e677_d_b7: f64 = (s.db[312][7] + s.db[573][7]);
        let eq55_e677_d_b8: f64 = (s.db[312][8] + s.db[573][8]);
        let eq55_e677_d_b9: f64 = (s.db[312][9] + s.db[573][9]);
        let eq55_e677_d_b10: f64 = (s.db[312][10] + s.db[573][10]);
        let eq55_e677_d_b11: f64 = (s.db[312][11] + s.db[573][11]);
        let eq55_e678: f64 = (p.p50 * eq55_e677);
        let eq55_e678_d_n0: f64 = (p.p50 * eq55_e677_d_n0);
        let eq55_e678_d_n1: f64 = (p.p50 * eq55_e677_d_n1);
        let eq55_e678_d_n2: f64 = (p.p50 * eq55_e677_d_n2);
        let eq55_e678_d_n3: f64 = (p.p50 * eq55_e677_d_n3);
        let eq55_e678_d_n4: f64 = (p.p50 * eq55_e677_d_n4);
        let eq55_e678_d_n5: f64 = (p.p50 * eq55_e677_d_n5);
        let eq55_e678_d_n6: f64 = (p.p50 * eq55_e677_d_n6);
        let eq55_e678_d_n7: f64 = (p.p50 * eq55_e677_d_n7);
        let eq55_e678_d_n8: f64 = (p.p50 * eq55_e677_d_n8);
        let eq55_e678_d_n9: f64 = (p.p50 * eq55_e677_d_n9);
        let eq55_e678_d_n10: f64 = (p.p50 * eq55_e677_d_n10);
        let eq55_e678_d_n11: f64 = (p.p50 * eq55_e677_d_n11);
        let eq55_e678_d_n12: f64 = (p.p50 * eq55_e677_d_n12);
        let eq55_e678_d_n13: f64 = (p.p50 * eq55_e677_d_n13);
        let eq55_e678_d_n14: f64 = (p.p50 * eq55_e677_d_n14);
        let eq55_e678_d_n15: f64 = (p.p50 * eq55_e677_d_n15);
        let eq55_e678_d_n16: f64 = (p.p50 * eq55_e677_d_n16);
        let eq55_e678_d_n17: f64 = (p.p50 * eq55_e677_d_n17);
        let eq55_e678_d_n18: f64 = (p.p50 * eq55_e677_d_n18);
        let eq55_e678_d_b0: f64 = (p.p50 * eq55_e677_d_b0);
        let eq55_e678_d_b1: f64 = (p.p50 * eq55_e677_d_b1);
        let eq55_e678_d_b2: f64 = (p.p50 * eq55_e677_d_b2);
        let eq55_e678_d_b3: f64 = (p.p50 * eq55_e677_d_b3);
        let eq55_e678_d_b4: f64 = (p.p50 * eq55_e677_d_b4);
        let eq55_e678_d_b5: f64 = (p.p50 * eq55_e677_d_b5);
        let eq55_e678_d_b6: f64 = (p.p50 * eq55_e677_d_b6);
        let eq55_e678_d_b7: f64 = (p.p50 * eq55_e677_d_b7);
        let eq55_e678_d_b8: f64 = (p.p50 * eq55_e677_d_b8);
        let eq55_e678_d_b9: f64 = (p.p50 * eq55_e677_d_b9);
        let eq55_e678_d_b10: f64 = (p.p50 * eq55_e677_d_b10);
        let eq55_e678_d_b11: f64 = (p.p50 * eq55_e677_d_b11);
        (eq55_e678, eq55_e678_d_n0, eq55_e678_d_n1, eq55_e678_d_n2, eq55_e678_d_n3, eq55_e678_d_n4, eq55_e678_d_n5, eq55_e678_d_n6, eq55_e678_d_n7, eq55_e678_d_n8, eq55_e678_d_n9, eq55_e678_d_n10, eq55_e678_d_n11, eq55_e678_d_n12, eq55_e678_d_n13, eq55_e678_d_n14, eq55_e678_d_n15, eq55_e678_d_n16, eq55_e678_d_n17, eq55_e678_d_n18, eq55_e678_d_b0, eq55_e678_d_b1, eq55_e678_d_b2, eq55_e678_d_b3, eq55_e678_d_b4, eq55_e678_d_b5, eq55_e678_d_b6, eq55_e678_d_b7, eq55_e678_d_b8, eq55_e678_d_b9, eq55_e678_d_b10, eq55_e678_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e680;
        let eq55_node_derivatives: [f64; 19] = [eq55_e680_d_n0, eq55_e680_d_n1, eq55_e680_d_n2, eq55_e680_d_n3, eq55_e680_d_n4, eq55_e680_d_n5, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n8, eq55_e680_d_n9, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n13, eq55_e680_d_n14, eq55_e680_d_n15, eq55_e680_d_n16, eq55_e680_d_n17, eq55_e680_d_n18];
        let eq55_branch_derivatives: [f64; 12] = [eq55_e680_d_b0, eq55_e680_d_b1, eq55_e680_d_b2, eq55_e680_d_b3, eq55_e680_d_b4, eq55_e680_d_b5, eq55_e680_d_b6, eq55_e680_d_b7, eq55_e680_d_b8, eq55_e680_d_b9, eq55_e680_d_b10, eq55_e680_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
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
        let (eq56_e685,) = {
    if (!(s.v[1847] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq56_value: f64 = eq56_e685;
        stamper.stamp_potential(
            branches[11],
            eq56_value,
            &[
            ],
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
        let (eq57_e692, eq57_e692_d_n0, eq57_e692_d_n1, eq57_e692_d_n2, eq57_e692_d_n3, eq57_e692_d_n4, eq57_e692_d_n5, eq57_e692_d_n6, eq57_e692_d_n7, eq57_e692_d_n8, eq57_e692_d_n9, eq57_e692_d_n10, eq57_e692_d_n11, eq57_e692_d_n12, eq57_e692_d_n13, eq57_e692_d_n14, eq57_e692_d_n15, eq57_e692_d_n16, eq57_e692_d_n17, eq57_e692_d_n18, eq57_e692_d_b0, eq57_e692_d_b1, eq57_e692_d_b2, eq57_e692_d_b3, eq57_e692_d_b4, eq57_e692_d_b5, eq57_e692_d_b6, eq57_e692_d_b7, eq57_e692_d_b8, eq57_e692_d_b9, eq57_e692_d_b10, eq57_e692_d_b11,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p37 != 0.0)) {
        (s.v[592], s.dn[592][0], s.dn[592][1], s.dn[592][2], s.dn[592][3], s.dn[592][4], s.dn[592][5], s.dn[592][6], s.dn[592][7], s.dn[592][8], s.dn[592][9], s.dn[592][10], s.dn[592][11], s.dn[592][12], s.dn[592][13], s.dn[592][14], s.dn[592][15], s.dn[592][16], s.dn[592][17], s.dn[592][18], s.db[592][0], s.db[592][1], s.db[592][2], s.db[592][3], s.db[592][4], s.db[592][5], s.db[592][6], s.db[592][7], s.db[592][8], s.db[592][9], s.db[592][10], s.db[592][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e692;
        let eq57_node_derivatives: [f64; 19] = [eq57_e692_d_n0, eq57_e692_d_n1, eq57_e692_d_n2, eq57_e692_d_n3, eq57_e692_d_n4, eq57_e692_d_n5, eq57_e692_d_n6, eq57_e692_d_n7, eq57_e692_d_n8, eq57_e692_d_n9, eq57_e692_d_n10, eq57_e692_d_n11, eq57_e692_d_n12, eq57_e692_d_n13, eq57_e692_d_n14, eq57_e692_d_n15, eq57_e692_d_n16, eq57_e692_d_n17, eq57_e692_d_n18];
        let eq57_branch_derivatives: [f64; 12] = [eq57_e692_d_b0, eq57_e692_d_b1, eq57_e692_d_b2, eq57_e692_d_b3, eq57_e692_d_b4, eq57_e692_d_b5, eq57_e692_d_b6, eq57_e692_d_b7, eq57_e692_d_b8, eq57_e692_d_b9, eq57_e692_d_b10, eq57_e692_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq57_value),
            &nodes,
            &eq57_node_derivatives,
            &branches,
            &eq57_branch_derivatives,
            self.multiplicity,
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
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq58_e701, eq58_e701_d_n17,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p37 != 0.0)) {
        let eq58_e699: f64 = ((nv17 - 0.0) * 1e-12);
        let eq58_e699_d_n17: f64 = 1e-12;
        (eq58_e699, eq58_e699_d_n17,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e701;
        stamper.stamp_current(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq58_value),
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * eq58_e701_d_n17),
            ],
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
        let (eq59_e713, eq59_e713_d_n0, eq59_e713_d_n1, eq59_e713_d_n2, eq59_e713_d_n3, eq59_e713_d_n4, eq59_e713_d_n5, eq59_e713_d_n6, eq59_e713_d_n7, eq59_e713_d_n8, eq59_e713_d_n9, eq59_e713_d_n10, eq59_e713_d_n11, eq59_e713_d_n12, eq59_e713_d_n13, eq59_e713_d_n14, eq59_e713_d_n15, eq59_e713_d_n16, eq59_e713_d_n17, eq59_e713_d_n18, eq59_e713_d_b0, eq59_e713_d_b1, eq59_e713_d_b2, eq59_e713_d_b3, eq59_e713_d_b4, eq59_e713_d_b5, eq59_e713_d_b6, eq59_e713_d_b7, eq59_e713_d_b8, eq59_e713_d_b9, eq59_e713_d_b10, eq59_e713_d_b11,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711: f64 = self.eval_ddt(11, eq59_e710);
        let eq59_e711_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_n17: f64 = self.ddt_jacobian(eq59_e708);
        let eq59_e711_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b6: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b7: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b8: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b9: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b10: f64 = self.ddt_jacobian(0.0);
        let eq59_e711_d_b11: f64 = self.ddt_jacobian(0.0);
        (eq59_e711, eq59_e711_d_n0, eq59_e711_d_n1, eq59_e711_d_n2, eq59_e711_d_n3, eq59_e711_d_n4, eq59_e711_d_n5, eq59_e711_d_n6, eq59_e711_d_n7, eq59_e711_d_n8, eq59_e711_d_n9, eq59_e711_d_n10, eq59_e711_d_n11, eq59_e711_d_n12, eq59_e711_d_n13, eq59_e711_d_n14, eq59_e711_d_n15, eq59_e711_d_n16, eq59_e711_d_n17, eq59_e711_d_n18, eq59_e711_d_b0, eq59_e711_d_b1, eq59_e711_d_b2, eq59_e711_d_b3, eq59_e711_d_b4, eq59_e711_d_b5, eq59_e711_d_b6, eq59_e711_d_b7, eq59_e711_d_b8, eq59_e711_d_b9, eq59_e711_d_b10, eq59_e711_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e713;
        let eq59_node_derivatives: [f64; 19] = [eq59_e713_d_n0, eq59_e713_d_n1, eq59_e713_d_n2, eq59_e713_d_n3, eq59_e713_d_n4, eq59_e713_d_n5, eq59_e713_d_n6, eq59_e713_d_n7, eq59_e713_d_n8, eq59_e713_d_n9, eq59_e713_d_n10, eq59_e713_d_n11, eq59_e713_d_n12, eq59_e713_d_n13, eq59_e713_d_n14, eq59_e713_d_n15, eq59_e713_d_n16, eq59_e713_d_n17, eq59_e713_d_n18];
        let eq59_branch_derivatives: [f64; 12] = [eq59_e713_d_b0, eq59_e713_d_b1, eq59_e713_d_b2, eq59_e713_d_b3, eq59_e713_d_b4, eq59_e713_d_b5, eq59_e713_d_b6, eq59_e713_d_b7, eq59_e713_d_b8, eq59_e713_d_b9, eq59_e713_d_b10, eq59_e713_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
            self.multiplicity * (eq59_value),
            &nodes,
            &eq59_node_derivatives,
            &branches,
            &eq59_branch_derivatives,
            self.multiplicity,
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
        let (eq60_e721,) = {
    if ((!(s.v[1847] != 0.0)) && (!(p.p37 != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e721;
        stamper.stamp_potential(
            branches[12],
            eq60_value,
            &[
            ],
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
        let (eq61_e728, eq61_e728_d_n0, eq61_e728_d_n1, eq61_e728_d_n2, eq61_e728_d_n3, eq61_e728_d_n4, eq61_e728_d_n5, eq61_e728_d_n6, eq61_e728_d_n7, eq61_e728_d_n8, eq61_e728_d_n9, eq61_e728_d_n10, eq61_e728_d_n11, eq61_e728_d_n12, eq61_e728_d_n13, eq61_e728_d_n14, eq61_e728_d_n15, eq61_e728_d_n16, eq61_e728_d_n17, eq61_e728_d_n18, eq61_e728_d_b0, eq61_e728_d_b1, eq61_e728_d_b2, eq61_e728_d_b3, eq61_e728_d_b4, eq61_e728_d_b5, eq61_e728_d_b6, eq61_e728_d_b7, eq61_e728_d_b8, eq61_e728_d_b9, eq61_e728_d_b10, eq61_e728_d_b11,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p34 != 0.0)) {
        (s.v[574], s.dn[574][0], s.dn[574][1], s.dn[574][2], s.dn[574][3], s.dn[574][4], s.dn[574][5], s.dn[574][6], s.dn[574][7], s.dn[574][8], s.dn[574][9], s.dn[574][10], s.dn[574][11], s.dn[574][12], s.dn[574][13], s.dn[574][14], s.dn[574][15], s.dn[574][16], s.dn[574][17], s.dn[574][18], s.db[574][0], s.db[574][1], s.db[574][2], s.db[574][3], s.db[574][4], s.db[574][5], s.db[574][6], s.db[574][7], s.db[574][8], s.db[574][9], s.db[574][10], s.db[574][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e728;
        let eq61_node_derivatives: [f64; 19] = [eq61_e728_d_n0, eq61_e728_d_n1, eq61_e728_d_n2, eq61_e728_d_n3, eq61_e728_d_n4, eq61_e728_d_n5, eq61_e728_d_n6, eq61_e728_d_n7, eq61_e728_d_n8, eq61_e728_d_n9, eq61_e728_d_n10, eq61_e728_d_n11, eq61_e728_d_n12, eq61_e728_d_n13, eq61_e728_d_n14, eq61_e728_d_n15, eq61_e728_d_n16, eq61_e728_d_n17, eq61_e728_d_n18];
        let eq61_branch_derivatives: [f64; 12] = [eq61_e728_d_b0, eq61_e728_d_b1, eq61_e728_d_b2, eq61_e728_d_b3, eq61_e728_d_b4, eq61_e728_d_b5, eq61_e728_d_b6, eq61_e728_d_b7, eq61_e728_d_b8, eq61_e728_d_b9, eq61_e728_d_b10, eq61_e728_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq61_value),
            &nodes,
            &eq61_node_derivatives,
            &branches,
            &eq61_branch_derivatives,
            self.multiplicity,
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
        let (eq62_e735, eq62_e735_d_n0, eq62_e735_d_n1, eq62_e735_d_n2, eq62_e735_d_n3, eq62_e735_d_n4, eq62_e735_d_n5, eq62_e735_d_n6, eq62_e735_d_n7, eq62_e735_d_n8, eq62_e735_d_n9, eq62_e735_d_n10, eq62_e735_d_n11, eq62_e735_d_n12, eq62_e735_d_n13, eq62_e735_d_n14, eq62_e735_d_n15, eq62_e735_d_n16, eq62_e735_d_n17, eq62_e735_d_n18, eq62_e735_d_b0, eq62_e735_d_b1, eq62_e735_d_b2, eq62_e735_d_b3, eq62_e735_d_b4, eq62_e735_d_b5, eq62_e735_d_b6, eq62_e735_d_b7, eq62_e735_d_b8, eq62_e735_d_b9, eq62_e735_d_b10, eq62_e735_d_b11,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p34 != 0.0)) {
        (s.v[575], s.dn[575][0], s.dn[575][1], s.dn[575][2], s.dn[575][3], s.dn[575][4], s.dn[575][5], s.dn[575][6], s.dn[575][7], s.dn[575][8], s.dn[575][9], s.dn[575][10], s.dn[575][11], s.dn[575][12], s.dn[575][13], s.dn[575][14], s.dn[575][15], s.dn[575][16], s.dn[575][17], s.dn[575][18], s.db[575][0], s.db[575][1], s.db[575][2], s.db[575][3], s.db[575][4], s.db[575][5], s.db[575][6], s.db[575][7], s.db[575][8], s.db[575][9], s.db[575][10], s.db[575][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e735;
        let eq62_node_derivatives: [f64; 19] = [eq62_e735_d_n0, eq62_e735_d_n1, eq62_e735_d_n2, eq62_e735_d_n3, eq62_e735_d_n4, eq62_e735_d_n5, eq62_e735_d_n6, eq62_e735_d_n7, eq62_e735_d_n8, eq62_e735_d_n9, eq62_e735_d_n10, eq62_e735_d_n11, eq62_e735_d_n12, eq62_e735_d_n13, eq62_e735_d_n14, eq62_e735_d_n15, eq62_e735_d_n16, eq62_e735_d_n17, eq62_e735_d_n18];
        let eq62_branch_derivatives: [f64; 12] = [eq62_e735_d_b0, eq62_e735_d_b1, eq62_e735_d_b2, eq62_e735_d_b3, eq62_e735_d_b4, eq62_e735_d_b5, eq62_e735_d_b6, eq62_e735_d_b7, eq62_e735_d_b8, eq62_e735_d_b9, eq62_e735_d_b10, eq62_e735_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[16]),
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
        let (eq63_e742, eq63_e742_d_n0, eq63_e742_d_n1, eq63_e742_d_n2, eq63_e742_d_n3, eq63_e742_d_n4, eq63_e742_d_n5, eq63_e742_d_n6, eq63_e742_d_n7, eq63_e742_d_n8, eq63_e742_d_n9, eq63_e742_d_n10, eq63_e742_d_n11, eq63_e742_d_n12, eq63_e742_d_n13, eq63_e742_d_n14, eq63_e742_d_n15, eq63_e742_d_n16, eq63_e742_d_n17, eq63_e742_d_n18, eq63_e742_d_b0, eq63_e742_d_b1, eq63_e742_d_b2, eq63_e742_d_b3, eq63_e742_d_b4, eq63_e742_d_b5, eq63_e742_d_b6, eq63_e742_d_b7, eq63_e742_d_b8, eq63_e742_d_b9, eq63_e742_d_b10, eq63_e742_d_b11,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p34 != 0.0)) {
        (s.v[583], s.dn[583][0], s.dn[583][1], s.dn[583][2], s.dn[583][3], s.dn[583][4], s.dn[583][5], s.dn[583][6], s.dn[583][7], s.dn[583][8], s.dn[583][9], s.dn[583][10], s.dn[583][11], s.dn[583][12], s.dn[583][13], s.dn[583][14], s.dn[583][15], s.dn[583][16], s.dn[583][17], s.dn[583][18], s.db[583][0], s.db[583][1], s.db[583][2], s.db[583][3], s.db[583][4], s.db[583][5], s.db[583][6], s.db[583][7], s.db[583][8], s.db[583][9], s.db[583][10], s.db[583][11],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e742;
        let eq63_node_derivatives: [f64; 19] = [eq63_e742_d_n0, eq63_e742_d_n1, eq63_e742_d_n2, eq63_e742_d_n3, eq63_e742_d_n4, eq63_e742_d_n5, eq63_e742_d_n6, eq63_e742_d_n7, eq63_e742_d_n8, eq63_e742_d_n9, eq63_e742_d_n10, eq63_e742_d_n11, eq63_e742_d_n12, eq63_e742_d_n13, eq63_e742_d_n14, eq63_e742_d_n15, eq63_e742_d_n16, eq63_e742_d_n17, eq63_e742_d_n18];
        let eq63_branch_derivatives: [f64; 12] = [eq63_e742_d_b0, eq63_e742_d_b1, eq63_e742_d_b2, eq63_e742_d_b3, eq63_e742_d_b4, eq63_e742_d_b5, eq63_e742_d_b6, eq63_e742_d_b7, eq63_e742_d_b8, eq63_e742_d_b9, eq63_e742_d_b10, eq63_e742_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[13]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq64_e751, eq64_e751_d_n15,) = {
    if ((!(s.v[1847] != 0.0)) && (p.p34 != 0.0)) {
        let eq64_e749: f64 = ((nv15 - 0.0) * 1e-12);
        let eq64_e749_d_n15: f64 = 1e-12;
        (eq64_e749, eq64_e749_d_n15,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e751;
        stamper.stamp_current(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq64_value),
            &[
                GeneratedDerivative::node(nodes[15], self.multiplicity * eq64_e751_d_n15),
            ],
        );
    }
}

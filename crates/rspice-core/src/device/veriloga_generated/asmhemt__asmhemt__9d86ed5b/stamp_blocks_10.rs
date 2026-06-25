#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_103_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq103_e1440, eq103_e1440_d_n0, eq103_e1440_d_n1, eq103_e1440_d_n2, eq103_e1440_d_n3, eq103_e1440_d_n4, eq103_e1440_d_n5, eq103_e1440_d_n6, eq103_e1440_d_n7, eq103_e1440_d_n8, eq103_e1440_d_n9, eq103_e1440_d_n10, eq103_e1440_d_n11, eq103_e1440_d_n12, eq103_e1440_d_n13, eq103_e1440_d_n14, eq103_e1440_d_n15, eq103_e1440_d_n16, eq103_e1440_d_n17, eq103_e1440_d_n18, eq103_e1440_d_n19, eq103_e1440_d_n20, eq103_e1440_d_n21, eq103_e1440_d_n22,) = {
    if ((!(s.v[553] != 0.0)) && (s.v[555] != 0.0)) {
        let eq103_e1436: f64 = (p.p6 * s.v[320]);
        let eq103_e1436_d_n0: f64 = (p.p6 * s.dn[320][0]);
        let eq103_e1436_d_n1: f64 = (p.p6 * s.dn[320][1]);
        let eq103_e1436_d_n2: f64 = (p.p6 * s.dn[320][2]);
        let eq103_e1436_d_n3: f64 = (p.p6 * s.dn[320][3]);
        let eq103_e1436_d_n4: f64 = (p.p6 * s.dn[320][4]);
        let eq103_e1436_d_n5: f64 = (p.p6 * s.dn[320][5]);
        let eq103_e1436_d_n6: f64 = (p.p6 * s.dn[320][6]);
        let eq103_e1436_d_n7: f64 = (p.p6 * s.dn[320][7]);
        let eq103_e1436_d_n8: f64 = (p.p6 * s.dn[320][8]);
        let eq103_e1436_d_n9: f64 = (p.p6 * s.dn[320][9]);
        let eq103_e1436_d_n10: f64 = (p.p6 * s.dn[320][10]);
        let eq103_e1436_d_n11: f64 = (p.p6 * s.dn[320][11]);
        let eq103_e1436_d_n12: f64 = (p.p6 * s.dn[320][12]);
        let eq103_e1436_d_n13: f64 = (p.p6 * s.dn[320][13]);
        let eq103_e1436_d_n14: f64 = (p.p6 * s.dn[320][14]);
        let eq103_e1436_d_n15: f64 = (p.p6 * s.dn[320][15]);
        let eq103_e1436_d_n16: f64 = (p.p6 * s.dn[320][16]);
        let eq103_e1436_d_n17: f64 = (p.p6 * s.dn[320][17]);
        let eq103_e1436_d_n18: f64 = (p.p6 * s.dn[320][18]);
        let eq103_e1436_d_n19: f64 = (p.p6 * s.dn[320][19]);
        let eq103_e1436_d_n20: f64 = (p.p6 * s.dn[320][20]);
        let eq103_e1436_d_n21: f64 = (p.p6 * s.dn[320][21]);
        let eq103_e1436_d_n22: f64 = (p.p6 * s.dn[320][22]);
        let eq103_e1438: f64 = (eq103_e1436 * (nv10 - nv9));
        let eq103_e1438_d_n0: f64 = (eq103_e1436_d_n0 * (nv10 - nv9));
        let eq103_e1438_d_n1: f64 = (eq103_e1436_d_n1 * (nv10 - nv9));
        let eq103_e1438_d_n2: f64 = (eq103_e1436_d_n2 * (nv10 - nv9));
        let eq103_e1438_d_n3: f64 = (eq103_e1436_d_n3 * (nv10 - nv9));
        let eq103_e1438_d_n4: f64 = (eq103_e1436_d_n4 * (nv10 - nv9));
        let eq103_e1438_d_n5: f64 = (eq103_e1436_d_n5 * (nv10 - nv9));
        let eq103_e1438_d_n6: f64 = (eq103_e1436_d_n6 * (nv10 - nv9));
        let eq103_e1438_d_n7: f64 = (eq103_e1436_d_n7 * (nv10 - nv9));
        let eq103_e1438_d_n8: f64 = (eq103_e1436_d_n8 * (nv10 - nv9));
        let eq103_e1438_d_n9: f64 = ((eq103_e1436_d_n9 * (nv10 - nv9)) + (-eq103_e1436));
        let eq103_e1438_d_n10: f64 = ((eq103_e1436_d_n10 * (nv10 - nv9)) + eq103_e1436);
        let eq103_e1438_d_n11: f64 = (eq103_e1436_d_n11 * (nv10 - nv9));
        let eq103_e1438_d_n12: f64 = (eq103_e1436_d_n12 * (nv10 - nv9));
        let eq103_e1438_d_n13: f64 = (eq103_e1436_d_n13 * (nv10 - nv9));
        let eq103_e1438_d_n14: f64 = (eq103_e1436_d_n14 * (nv10 - nv9));
        let eq103_e1438_d_n15: f64 = (eq103_e1436_d_n15 * (nv10 - nv9));
        let eq103_e1438_d_n16: f64 = (eq103_e1436_d_n16 * (nv10 - nv9));
        let eq103_e1438_d_n17: f64 = (eq103_e1436_d_n17 * (nv10 - nv9));
        let eq103_e1438_d_n18: f64 = (eq103_e1436_d_n18 * (nv10 - nv9));
        let eq103_e1438_d_n19: f64 = (eq103_e1436_d_n19 * (nv10 - nv9));
        let eq103_e1438_d_n20: f64 = (eq103_e1436_d_n20 * (nv10 - nv9));
        let eq103_e1438_d_n21: f64 = (eq103_e1436_d_n21 * (nv10 - nv9));
        let eq103_e1438_d_n22: f64 = (eq103_e1436_d_n22 * (nv10 - nv9));
        (eq103_e1438, eq103_e1438_d_n0, eq103_e1438_d_n1, eq103_e1438_d_n2, eq103_e1438_d_n3, eq103_e1438_d_n4, eq103_e1438_d_n5, eq103_e1438_d_n6, eq103_e1438_d_n7, eq103_e1438_d_n8, eq103_e1438_d_n9, eq103_e1438_d_n10, eq103_e1438_d_n11, eq103_e1438_d_n12, eq103_e1438_d_n13, eq103_e1438_d_n14, eq103_e1438_d_n15, eq103_e1438_d_n16, eq103_e1438_d_n17, eq103_e1438_d_n18, eq103_e1438_d_n19, eq103_e1438_d_n20, eq103_e1438_d_n21, eq103_e1438_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1440;
        let eq103_node_derivatives: [f64; 23] = [eq103_e1440_d_n0, eq103_e1440_d_n1, eq103_e1440_d_n2, eq103_e1440_d_n3, eq103_e1440_d_n4, eq103_e1440_d_n5, eq103_e1440_d_n6, eq103_e1440_d_n7, eq103_e1440_d_n8, eq103_e1440_d_n9, eq103_e1440_d_n10, eq103_e1440_d_n11, eq103_e1440_d_n12, eq103_e1440_d_n13, eq103_e1440_d_n14, eq103_e1440_d_n15, eq103_e1440_d_n16, eq103_e1440_d_n17, eq103_e1440_d_n18, eq103_e1440_d_n19, eq103_e1440_d_n20, eq103_e1440_d_n21, eq103_e1440_d_n22];
        let eq103_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[9]),
            self.multiplicity * (eq103_value),
            &nodes,
            &eq103_node_derivatives,
            &branches,
            &eq103_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_104_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq104_e1448,) = {
    if ((!(s.v[553] != 0.0)) && (!(s.v[555] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq104_value: f64 = eq104_e1448;
        stamper.stamp_potential(
            branches[54],
            eq104_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_105_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq105_e1456,) = {
    if ((!(s.v[553] != 0.0)) && (!(s.v[555] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq105_value: f64 = eq105_e1456;
        stamper.stamp_potential(
            branches[55],
            eq105_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_106_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq106_e1459: f64 = (p.p6 * s.v[369]);
        let eq106_e1459_d_n0: f64 = (p.p6 * s.dn[369][0]);
        let eq106_e1459_d_n1: f64 = (p.p6 * s.dn[369][1]);
        let eq106_e1459_d_n2: f64 = (p.p6 * s.dn[369][2]);
        let eq106_e1459_d_n3: f64 = (p.p6 * s.dn[369][3]);
        let eq106_e1459_d_n4: f64 = (p.p6 * s.dn[369][4]);
        let eq106_e1459_d_n5: f64 = (p.p6 * s.dn[369][5]);
        let eq106_e1459_d_n6: f64 = (p.p6 * s.dn[369][6]);
        let eq106_e1459_d_n7: f64 = (p.p6 * s.dn[369][7]);
        let eq106_e1459_d_n8: f64 = (p.p6 * s.dn[369][8]);
        let eq106_e1459_d_n9: f64 = (p.p6 * s.dn[369][9]);
        let eq106_e1459_d_n10: f64 = (p.p6 * s.dn[369][10]);
        let eq106_e1459_d_n11: f64 = (p.p6 * s.dn[369][11]);
        let eq106_e1459_d_n12: f64 = (p.p6 * s.dn[369][12]);
        let eq106_e1459_d_n13: f64 = (p.p6 * s.dn[369][13]);
        let eq106_e1459_d_n14: f64 = (p.p6 * s.dn[369][14]);
        let eq106_e1459_d_n15: f64 = (p.p6 * s.dn[369][15]);
        let eq106_e1459_d_n16: f64 = (p.p6 * s.dn[369][16]);
        let eq106_e1459_d_n17: f64 = (p.p6 * s.dn[369][17]);
        let eq106_e1459_d_n18: f64 = (p.p6 * s.dn[369][18]);
        let eq106_e1459_d_n19: f64 = (p.p6 * s.dn[369][19]);
        let eq106_e1459_d_n20: f64 = (p.p6 * s.dn[369][20]);
        let eq106_e1459_d_n21: f64 = (p.p6 * s.dn[369][21]);
        let eq106_e1459_d_n22: f64 = (p.p6 * s.dn[369][22]);
        let eq106_value: f64 = eq106_e1459;
        let eq106_node_derivatives: [f64; 23] = [eq106_e1459_d_n0, eq106_e1459_d_n1, eq106_e1459_d_n2, eq106_e1459_d_n3, eq106_e1459_d_n4, eq106_e1459_d_n5, eq106_e1459_d_n6, eq106_e1459_d_n7, eq106_e1459_d_n8, eq106_e1459_d_n9, eq106_e1459_d_n10, eq106_e1459_d_n11, eq106_e1459_d_n12, eq106_e1459_d_n13, eq106_e1459_d_n14, eq106_e1459_d_n15, eq106_e1459_d_n16, eq106_e1459_d_n17, eq106_e1459_d_n18, eq106_e1459_d_n19, eq106_e1459_d_n20, eq106_e1459_d_n21, eq106_e1459_d_n22];
        let eq106_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            self.multiplicity * (eq106_value),
            &nodes,
            &eq106_node_derivatives,
            &branches,
            &eq106_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_107_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq107_e1462: f64 = (p.p6 * s.v[370]);
        let eq107_e1462_d_n0: f64 = (p.p6 * s.dn[370][0]);
        let eq107_e1462_d_n1: f64 = (p.p6 * s.dn[370][1]);
        let eq107_e1462_d_n2: f64 = (p.p6 * s.dn[370][2]);
        let eq107_e1462_d_n3: f64 = (p.p6 * s.dn[370][3]);
        let eq107_e1462_d_n4: f64 = (p.p6 * s.dn[370][4]);
        let eq107_e1462_d_n5: f64 = (p.p6 * s.dn[370][5]);
        let eq107_e1462_d_n6: f64 = (p.p6 * s.dn[370][6]);
        let eq107_e1462_d_n7: f64 = (p.p6 * s.dn[370][7]);
        let eq107_e1462_d_n8: f64 = (p.p6 * s.dn[370][8]);
        let eq107_e1462_d_n9: f64 = (p.p6 * s.dn[370][9]);
        let eq107_e1462_d_n10: f64 = (p.p6 * s.dn[370][10]);
        let eq107_e1462_d_n11: f64 = (p.p6 * s.dn[370][11]);
        let eq107_e1462_d_n12: f64 = (p.p6 * s.dn[370][12]);
        let eq107_e1462_d_n13: f64 = (p.p6 * s.dn[370][13]);
        let eq107_e1462_d_n14: f64 = (p.p6 * s.dn[370][14]);
        let eq107_e1462_d_n15: f64 = (p.p6 * s.dn[370][15]);
        let eq107_e1462_d_n16: f64 = (p.p6 * s.dn[370][16]);
        let eq107_e1462_d_n17: f64 = (p.p6 * s.dn[370][17]);
        let eq107_e1462_d_n18: f64 = (p.p6 * s.dn[370][18]);
        let eq107_e1462_d_n19: f64 = (p.p6 * s.dn[370][19]);
        let eq107_e1462_d_n20: f64 = (p.p6 * s.dn[370][20]);
        let eq107_e1462_d_n21: f64 = (p.p6 * s.dn[370][21]);
        let eq107_e1462_d_n22: f64 = (p.p6 * s.dn[370][22]);
        let eq107_value: f64 = eq107_e1462;
        let eq107_node_derivatives: [f64; 23] = [eq107_e1462_d_n0, eq107_e1462_d_n1, eq107_e1462_d_n2, eq107_e1462_d_n3, eq107_e1462_d_n4, eq107_e1462_d_n5, eq107_e1462_d_n6, eq107_e1462_d_n7, eq107_e1462_d_n8, eq107_e1462_d_n9, eq107_e1462_d_n10, eq107_e1462_d_n11, eq107_e1462_d_n12, eq107_e1462_d_n13, eq107_e1462_d_n14, eq107_e1462_d_n15, eq107_e1462_d_n16, eq107_e1462_d_n17, eq107_e1462_d_n18, eq107_e1462_d_n19, eq107_e1462_d_n20, eq107_e1462_d_n21, eq107_e1462_d_n22];
        let eq107_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            self.multiplicity * (eq107_value),
            &nodes,
            &eq107_node_derivatives,
            &branches,
            &eq107_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_108_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq108_e1471,) = {
    if (s.v[567] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq108_value: f64 = eq108_e1471;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq108_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_109_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq109_e1474: f64 = self.eval_ddt(8, s.v[165]);
        let eq109_e1474_d_n0: f64 = self.ddt_jacobian(s.dn[165][0]);
        let eq109_e1474_d_n1: f64 = self.ddt_jacobian(s.dn[165][1]);
        let eq109_e1474_d_n2: f64 = self.ddt_jacobian(s.dn[165][2]);
        let eq109_e1474_d_n3: f64 = self.ddt_jacobian(s.dn[165][3]);
        let eq109_e1474_d_n4: f64 = self.ddt_jacobian(s.dn[165][4]);
        let eq109_e1474_d_n5: f64 = self.ddt_jacobian(s.dn[165][5]);
        let eq109_e1474_d_n6: f64 = self.ddt_jacobian(s.dn[165][6]);
        let eq109_e1474_d_n7: f64 = self.ddt_jacobian(s.dn[165][7]);
        let eq109_e1474_d_n8: f64 = self.ddt_jacobian(s.dn[165][8]);
        let eq109_e1474_d_n9: f64 = self.ddt_jacobian(s.dn[165][9]);
        let eq109_e1474_d_n10: f64 = self.ddt_jacobian(s.dn[165][10]);
        let eq109_e1474_d_n11: f64 = self.ddt_jacobian(s.dn[165][11]);
        let eq109_e1474_d_n12: f64 = self.ddt_jacobian(s.dn[165][12]);
        let eq109_e1474_d_n13: f64 = self.ddt_jacobian(s.dn[165][13]);
        let eq109_e1474_d_n14: f64 = self.ddt_jacobian(s.dn[165][14]);
        let eq109_e1474_d_n15: f64 = self.ddt_jacobian(s.dn[165][15]);
        let eq109_e1474_d_n16: f64 = self.ddt_jacobian(s.dn[165][16]);
        let eq109_e1474_d_n17: f64 = self.ddt_jacobian(s.dn[165][17]);
        let eq109_e1474_d_n18: f64 = self.ddt_jacobian(s.dn[165][18]);
        let eq109_e1474_d_n19: f64 = self.ddt_jacobian(s.dn[165][19]);
        let eq109_e1474_d_n20: f64 = self.ddt_jacobian(s.dn[165][20]);
        let eq109_e1474_d_n21: f64 = self.ddt_jacobian(s.dn[165][21]);
        let eq109_e1474_d_n22: f64 = self.ddt_jacobian(s.dn[165][22]);
        let eq109_e1475: f64 = (p.p7 * eq109_e1474);
        let eq109_e1475_d_n0: f64 = (p.p7 * eq109_e1474_d_n0);
        let eq109_e1475_d_n1: f64 = (p.p7 * eq109_e1474_d_n1);
        let eq109_e1475_d_n2: f64 = (p.p7 * eq109_e1474_d_n2);
        let eq109_e1475_d_n3: f64 = (p.p7 * eq109_e1474_d_n3);
        let eq109_e1475_d_n4: f64 = (p.p7 * eq109_e1474_d_n4);
        let eq109_e1475_d_n5: f64 = (p.p7 * eq109_e1474_d_n5);
        let eq109_e1475_d_n6: f64 = (p.p7 * eq109_e1474_d_n6);
        let eq109_e1475_d_n7: f64 = (p.p7 * eq109_e1474_d_n7);
        let eq109_e1475_d_n8: f64 = (p.p7 * eq109_e1474_d_n8);
        let eq109_e1475_d_n9: f64 = (p.p7 * eq109_e1474_d_n9);
        let eq109_e1475_d_n10: f64 = (p.p7 * eq109_e1474_d_n10);
        let eq109_e1475_d_n11: f64 = (p.p7 * eq109_e1474_d_n11);
        let eq109_e1475_d_n12: f64 = (p.p7 * eq109_e1474_d_n12);
        let eq109_e1475_d_n13: f64 = (p.p7 * eq109_e1474_d_n13);
        let eq109_e1475_d_n14: f64 = (p.p7 * eq109_e1474_d_n14);
        let eq109_e1475_d_n15: f64 = (p.p7 * eq109_e1474_d_n15);
        let eq109_e1475_d_n16: f64 = (p.p7 * eq109_e1474_d_n16);
        let eq109_e1475_d_n17: f64 = (p.p7 * eq109_e1474_d_n17);
        let eq109_e1475_d_n18: f64 = (p.p7 * eq109_e1474_d_n18);
        let eq109_e1475_d_n19: f64 = (p.p7 * eq109_e1474_d_n19);
        let eq109_e1475_d_n20: f64 = (p.p7 * eq109_e1474_d_n20);
        let eq109_e1475_d_n21: f64 = (p.p7 * eq109_e1474_d_n21);
        let eq109_e1475_d_n22: f64 = (p.p7 * eq109_e1474_d_n22);
        let eq109_value: f64 = eq109_e1475;
        let eq109_node_derivatives: [f64; 23] = [eq109_e1475_d_n0, eq109_e1475_d_n1, eq109_e1475_d_n2, eq109_e1475_d_n3, eq109_e1475_d_n4, eq109_e1475_d_n5, eq109_e1475_d_n6, eq109_e1475_d_n7, eq109_e1475_d_n8, eq109_e1475_d_n9, eq109_e1475_d_n10, eq109_e1475_d_n11, eq109_e1475_d_n12, eq109_e1475_d_n13, eq109_e1475_d_n14, eq109_e1475_d_n15, eq109_e1475_d_n16, eq109_e1475_d_n17, eq109_e1475_d_n18, eq109_e1475_d_n19, eq109_e1475_d_n20, eq109_e1475_d_n21, eq109_e1475_d_n22];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq109_value),
            &nodes,
            &eq109_node_derivatives,
            &branches,
            &eq109_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_110_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq110_e1478: f64 = self.eval_ddt(9, s.v[161]);
        let eq110_e1478_d_n0: f64 = self.ddt_jacobian(s.dn[161][0]);
        let eq110_e1478_d_n1: f64 = self.ddt_jacobian(s.dn[161][1]);
        let eq110_e1478_d_n2: f64 = self.ddt_jacobian(s.dn[161][2]);
        let eq110_e1478_d_n3: f64 = self.ddt_jacobian(s.dn[161][3]);
        let eq110_e1478_d_n4: f64 = self.ddt_jacobian(s.dn[161][4]);
        let eq110_e1478_d_n5: f64 = self.ddt_jacobian(s.dn[161][5]);
        let eq110_e1478_d_n6: f64 = self.ddt_jacobian(s.dn[161][6]);
        let eq110_e1478_d_n7: f64 = self.ddt_jacobian(s.dn[161][7]);
        let eq110_e1478_d_n8: f64 = self.ddt_jacobian(s.dn[161][8]);
        let eq110_e1478_d_n9: f64 = self.ddt_jacobian(s.dn[161][9]);
        let eq110_e1478_d_n10: f64 = self.ddt_jacobian(s.dn[161][10]);
        let eq110_e1478_d_n11: f64 = self.ddt_jacobian(s.dn[161][11]);
        let eq110_e1478_d_n12: f64 = self.ddt_jacobian(s.dn[161][12]);
        let eq110_e1478_d_n13: f64 = self.ddt_jacobian(s.dn[161][13]);
        let eq110_e1478_d_n14: f64 = self.ddt_jacobian(s.dn[161][14]);
        let eq110_e1478_d_n15: f64 = self.ddt_jacobian(s.dn[161][15]);
        let eq110_e1478_d_n16: f64 = self.ddt_jacobian(s.dn[161][16]);
        let eq110_e1478_d_n17: f64 = self.ddt_jacobian(s.dn[161][17]);
        let eq110_e1478_d_n18: f64 = self.ddt_jacobian(s.dn[161][18]);
        let eq110_e1478_d_n19: f64 = self.ddt_jacobian(s.dn[161][19]);
        let eq110_e1478_d_n20: f64 = self.ddt_jacobian(s.dn[161][20]);
        let eq110_e1478_d_n21: f64 = self.ddt_jacobian(s.dn[161][21]);
        let eq110_e1478_d_n22: f64 = self.ddt_jacobian(s.dn[161][22]);
        let eq110_e1479: f64 = (p.p7 * eq110_e1478);
        let eq110_e1479_d_n0: f64 = (p.p7 * eq110_e1478_d_n0);
        let eq110_e1479_d_n1: f64 = (p.p7 * eq110_e1478_d_n1);
        let eq110_e1479_d_n2: f64 = (p.p7 * eq110_e1478_d_n2);
        let eq110_e1479_d_n3: f64 = (p.p7 * eq110_e1478_d_n3);
        let eq110_e1479_d_n4: f64 = (p.p7 * eq110_e1478_d_n4);
        let eq110_e1479_d_n5: f64 = (p.p7 * eq110_e1478_d_n5);
        let eq110_e1479_d_n6: f64 = (p.p7 * eq110_e1478_d_n6);
        let eq110_e1479_d_n7: f64 = (p.p7 * eq110_e1478_d_n7);
        let eq110_e1479_d_n8: f64 = (p.p7 * eq110_e1478_d_n8);
        let eq110_e1479_d_n9: f64 = (p.p7 * eq110_e1478_d_n9);
        let eq110_e1479_d_n10: f64 = (p.p7 * eq110_e1478_d_n10);
        let eq110_e1479_d_n11: f64 = (p.p7 * eq110_e1478_d_n11);
        let eq110_e1479_d_n12: f64 = (p.p7 * eq110_e1478_d_n12);
        let eq110_e1479_d_n13: f64 = (p.p7 * eq110_e1478_d_n13);
        let eq110_e1479_d_n14: f64 = (p.p7 * eq110_e1478_d_n14);
        let eq110_e1479_d_n15: f64 = (p.p7 * eq110_e1478_d_n15);
        let eq110_e1479_d_n16: f64 = (p.p7 * eq110_e1478_d_n16);
        let eq110_e1479_d_n17: f64 = (p.p7 * eq110_e1478_d_n17);
        let eq110_e1479_d_n18: f64 = (p.p7 * eq110_e1478_d_n18);
        let eq110_e1479_d_n19: f64 = (p.p7 * eq110_e1478_d_n19);
        let eq110_e1479_d_n20: f64 = (p.p7 * eq110_e1478_d_n20);
        let eq110_e1479_d_n21: f64 = (p.p7 * eq110_e1478_d_n21);
        let eq110_e1479_d_n22: f64 = (p.p7 * eq110_e1478_d_n22);
        let eq110_value: f64 = eq110_e1479;
        let eq110_node_derivatives: [f64; 23] = [eq110_e1479_d_n0, eq110_e1479_d_n1, eq110_e1479_d_n2, eq110_e1479_d_n3, eq110_e1479_d_n4, eq110_e1479_d_n5, eq110_e1479_d_n6, eq110_e1479_d_n7, eq110_e1479_d_n8, eq110_e1479_d_n9, eq110_e1479_d_n10, eq110_e1479_d_n11, eq110_e1479_d_n12, eq110_e1479_d_n13, eq110_e1479_d_n14, eq110_e1479_d_n15, eq110_e1479_d_n16, eq110_e1479_d_n17, eq110_e1479_d_n18, eq110_e1479_d_n19, eq110_e1479_d_n20, eq110_e1479_d_n21, eq110_e1479_d_n22];
        let eq110_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq110_value),
            &nodes,
            &eq110_node_derivatives,
            &branches,
            &eq110_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_111_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq111_e1486, eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22,) = {
    if (s.v[569] != 0.0) {
        let eq111_e1483: f64 = self.eval_ddt(10, s.v[162]);
        let eq111_e1483_d_n0: f64 = self.ddt_jacobian(s.dn[162][0]);
        let eq111_e1483_d_n1: f64 = self.ddt_jacobian(s.dn[162][1]);
        let eq111_e1483_d_n2: f64 = self.ddt_jacobian(s.dn[162][2]);
        let eq111_e1483_d_n3: f64 = self.ddt_jacobian(s.dn[162][3]);
        let eq111_e1483_d_n4: f64 = self.ddt_jacobian(s.dn[162][4]);
        let eq111_e1483_d_n5: f64 = self.ddt_jacobian(s.dn[162][5]);
        let eq111_e1483_d_n6: f64 = self.ddt_jacobian(s.dn[162][6]);
        let eq111_e1483_d_n7: f64 = self.ddt_jacobian(s.dn[162][7]);
        let eq111_e1483_d_n8: f64 = self.ddt_jacobian(s.dn[162][8]);
        let eq111_e1483_d_n9: f64 = self.ddt_jacobian(s.dn[162][9]);
        let eq111_e1483_d_n10: f64 = self.ddt_jacobian(s.dn[162][10]);
        let eq111_e1483_d_n11: f64 = self.ddt_jacobian(s.dn[162][11]);
        let eq111_e1483_d_n12: f64 = self.ddt_jacobian(s.dn[162][12]);
        let eq111_e1483_d_n13: f64 = self.ddt_jacobian(s.dn[162][13]);
        let eq111_e1483_d_n14: f64 = self.ddt_jacobian(s.dn[162][14]);
        let eq111_e1483_d_n15: f64 = self.ddt_jacobian(s.dn[162][15]);
        let eq111_e1483_d_n16: f64 = self.ddt_jacobian(s.dn[162][16]);
        let eq111_e1483_d_n17: f64 = self.ddt_jacobian(s.dn[162][17]);
        let eq111_e1483_d_n18: f64 = self.ddt_jacobian(s.dn[162][18]);
        let eq111_e1483_d_n19: f64 = self.ddt_jacobian(s.dn[162][19]);
        let eq111_e1483_d_n20: f64 = self.ddt_jacobian(s.dn[162][20]);
        let eq111_e1483_d_n21: f64 = self.ddt_jacobian(s.dn[162][21]);
        let eq111_e1483_d_n22: f64 = self.ddt_jacobian(s.dn[162][22]);
        let eq111_e1484: f64 = (p.p7 * eq111_e1483);
        let eq111_e1484_d_n0: f64 = (p.p7 * eq111_e1483_d_n0);
        let eq111_e1484_d_n1: f64 = (p.p7 * eq111_e1483_d_n1);
        let eq111_e1484_d_n2: f64 = (p.p7 * eq111_e1483_d_n2);
        let eq111_e1484_d_n3: f64 = (p.p7 * eq111_e1483_d_n3);
        let eq111_e1484_d_n4: f64 = (p.p7 * eq111_e1483_d_n4);
        let eq111_e1484_d_n5: f64 = (p.p7 * eq111_e1483_d_n5);
        let eq111_e1484_d_n6: f64 = (p.p7 * eq111_e1483_d_n6);
        let eq111_e1484_d_n7: f64 = (p.p7 * eq111_e1483_d_n7);
        let eq111_e1484_d_n8: f64 = (p.p7 * eq111_e1483_d_n8);
        let eq111_e1484_d_n9: f64 = (p.p7 * eq111_e1483_d_n9);
        let eq111_e1484_d_n10: f64 = (p.p7 * eq111_e1483_d_n10);
        let eq111_e1484_d_n11: f64 = (p.p7 * eq111_e1483_d_n11);
        let eq111_e1484_d_n12: f64 = (p.p7 * eq111_e1483_d_n12);
        let eq111_e1484_d_n13: f64 = (p.p7 * eq111_e1483_d_n13);
        let eq111_e1484_d_n14: f64 = (p.p7 * eq111_e1483_d_n14);
        let eq111_e1484_d_n15: f64 = (p.p7 * eq111_e1483_d_n15);
        let eq111_e1484_d_n16: f64 = (p.p7 * eq111_e1483_d_n16);
        let eq111_e1484_d_n17: f64 = (p.p7 * eq111_e1483_d_n17);
        let eq111_e1484_d_n18: f64 = (p.p7 * eq111_e1483_d_n18);
        let eq111_e1484_d_n19: f64 = (p.p7 * eq111_e1483_d_n19);
        let eq111_e1484_d_n20: f64 = (p.p7 * eq111_e1483_d_n20);
        let eq111_e1484_d_n21: f64 = (p.p7 * eq111_e1483_d_n21);
        let eq111_e1484_d_n22: f64 = (p.p7 * eq111_e1483_d_n22);
        (eq111_e1484, eq111_e1484_d_n0, eq111_e1484_d_n1, eq111_e1484_d_n2, eq111_e1484_d_n3, eq111_e1484_d_n4, eq111_e1484_d_n5, eq111_e1484_d_n6, eq111_e1484_d_n7, eq111_e1484_d_n8, eq111_e1484_d_n9, eq111_e1484_d_n10, eq111_e1484_d_n11, eq111_e1484_d_n12, eq111_e1484_d_n13, eq111_e1484_d_n14, eq111_e1484_d_n15, eq111_e1484_d_n16, eq111_e1484_d_n17, eq111_e1484_d_n18, eq111_e1484_d_n19, eq111_e1484_d_n20, eq111_e1484_d_n21, eq111_e1484_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1486;
        let eq111_node_derivatives: [f64; 23] = [eq111_e1486_d_n0, eq111_e1486_d_n1, eq111_e1486_d_n2, eq111_e1486_d_n3, eq111_e1486_d_n4, eq111_e1486_d_n5, eq111_e1486_d_n6, eq111_e1486_d_n7, eq111_e1486_d_n8, eq111_e1486_d_n9, eq111_e1486_d_n10, eq111_e1486_d_n11, eq111_e1486_d_n12, eq111_e1486_d_n13, eq111_e1486_d_n14, eq111_e1486_d_n15, eq111_e1486_d_n16, eq111_e1486_d_n17, eq111_e1486_d_n18, eq111_e1486_d_n19, eq111_e1486_d_n20, eq111_e1486_d_n21, eq111_e1486_d_n22];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            self.multiplicity * (eq111_value),
            &nodes,
            &eq111_node_derivatives,
            &branches,
            &eq111_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_112_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq112_e1493, eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22,) = {
    if (s.v[569] != 0.0) {
        let eq112_e1490: f64 = self.eval_ddt(11, s.v[163]);
        let eq112_e1490_d_n0: f64 = self.ddt_jacobian(s.dn[163][0]);
        let eq112_e1490_d_n1: f64 = self.ddt_jacobian(s.dn[163][1]);
        let eq112_e1490_d_n2: f64 = self.ddt_jacobian(s.dn[163][2]);
        let eq112_e1490_d_n3: f64 = self.ddt_jacobian(s.dn[163][3]);
        let eq112_e1490_d_n4: f64 = self.ddt_jacobian(s.dn[163][4]);
        let eq112_e1490_d_n5: f64 = self.ddt_jacobian(s.dn[163][5]);
        let eq112_e1490_d_n6: f64 = self.ddt_jacobian(s.dn[163][6]);
        let eq112_e1490_d_n7: f64 = self.ddt_jacobian(s.dn[163][7]);
        let eq112_e1490_d_n8: f64 = self.ddt_jacobian(s.dn[163][8]);
        let eq112_e1490_d_n9: f64 = self.ddt_jacobian(s.dn[163][9]);
        let eq112_e1490_d_n10: f64 = self.ddt_jacobian(s.dn[163][10]);
        let eq112_e1490_d_n11: f64 = self.ddt_jacobian(s.dn[163][11]);
        let eq112_e1490_d_n12: f64 = self.ddt_jacobian(s.dn[163][12]);
        let eq112_e1490_d_n13: f64 = self.ddt_jacobian(s.dn[163][13]);
        let eq112_e1490_d_n14: f64 = self.ddt_jacobian(s.dn[163][14]);
        let eq112_e1490_d_n15: f64 = self.ddt_jacobian(s.dn[163][15]);
        let eq112_e1490_d_n16: f64 = self.ddt_jacobian(s.dn[163][16]);
        let eq112_e1490_d_n17: f64 = self.ddt_jacobian(s.dn[163][17]);
        let eq112_e1490_d_n18: f64 = self.ddt_jacobian(s.dn[163][18]);
        let eq112_e1490_d_n19: f64 = self.ddt_jacobian(s.dn[163][19]);
        let eq112_e1490_d_n20: f64 = self.ddt_jacobian(s.dn[163][20]);
        let eq112_e1490_d_n21: f64 = self.ddt_jacobian(s.dn[163][21]);
        let eq112_e1490_d_n22: f64 = self.ddt_jacobian(s.dn[163][22]);
        let eq112_e1491: f64 = (p.p7 * eq112_e1490);
        let eq112_e1491_d_n0: f64 = (p.p7 * eq112_e1490_d_n0);
        let eq112_e1491_d_n1: f64 = (p.p7 * eq112_e1490_d_n1);
        let eq112_e1491_d_n2: f64 = (p.p7 * eq112_e1490_d_n2);
        let eq112_e1491_d_n3: f64 = (p.p7 * eq112_e1490_d_n3);
        let eq112_e1491_d_n4: f64 = (p.p7 * eq112_e1490_d_n4);
        let eq112_e1491_d_n5: f64 = (p.p7 * eq112_e1490_d_n5);
        let eq112_e1491_d_n6: f64 = (p.p7 * eq112_e1490_d_n6);
        let eq112_e1491_d_n7: f64 = (p.p7 * eq112_e1490_d_n7);
        let eq112_e1491_d_n8: f64 = (p.p7 * eq112_e1490_d_n8);
        let eq112_e1491_d_n9: f64 = (p.p7 * eq112_e1490_d_n9);
        let eq112_e1491_d_n10: f64 = (p.p7 * eq112_e1490_d_n10);
        let eq112_e1491_d_n11: f64 = (p.p7 * eq112_e1490_d_n11);
        let eq112_e1491_d_n12: f64 = (p.p7 * eq112_e1490_d_n12);
        let eq112_e1491_d_n13: f64 = (p.p7 * eq112_e1490_d_n13);
        let eq112_e1491_d_n14: f64 = (p.p7 * eq112_e1490_d_n14);
        let eq112_e1491_d_n15: f64 = (p.p7 * eq112_e1490_d_n15);
        let eq112_e1491_d_n16: f64 = (p.p7 * eq112_e1490_d_n16);
        let eq112_e1491_d_n17: f64 = (p.p7 * eq112_e1490_d_n17);
        let eq112_e1491_d_n18: f64 = (p.p7 * eq112_e1490_d_n18);
        let eq112_e1491_d_n19: f64 = (p.p7 * eq112_e1490_d_n19);
        let eq112_e1491_d_n20: f64 = (p.p7 * eq112_e1490_d_n20);
        let eq112_e1491_d_n21: f64 = (p.p7 * eq112_e1490_d_n21);
        let eq112_e1491_d_n22: f64 = (p.p7 * eq112_e1490_d_n22);
        (eq112_e1491, eq112_e1491_d_n0, eq112_e1491_d_n1, eq112_e1491_d_n2, eq112_e1491_d_n3, eq112_e1491_d_n4, eq112_e1491_d_n5, eq112_e1491_d_n6, eq112_e1491_d_n7, eq112_e1491_d_n8, eq112_e1491_d_n9, eq112_e1491_d_n10, eq112_e1491_d_n11, eq112_e1491_d_n12, eq112_e1491_d_n13, eq112_e1491_d_n14, eq112_e1491_d_n15, eq112_e1491_d_n16, eq112_e1491_d_n17, eq112_e1491_d_n18, eq112_e1491_d_n19, eq112_e1491_d_n20, eq112_e1491_d_n21, eq112_e1491_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1493;
        let eq112_node_derivatives: [f64; 23] = [eq112_e1493_d_n0, eq112_e1493_d_n1, eq112_e1493_d_n2, eq112_e1493_d_n3, eq112_e1493_d_n4, eq112_e1493_d_n5, eq112_e1493_d_n6, eq112_e1493_d_n7, eq112_e1493_d_n8, eq112_e1493_d_n9, eq112_e1493_d_n10, eq112_e1493_d_n11, eq112_e1493_d_n12, eq112_e1493_d_n13, eq112_e1493_d_n14, eq112_e1493_d_n15, eq112_e1493_d_n16, eq112_e1493_d_n17, eq112_e1493_d_n18, eq112_e1493_d_n19, eq112_e1493_d_n20, eq112_e1493_d_n21, eq112_e1493_d_n22];
        let eq112_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            self.multiplicity * (eq112_value),
            &nodes,
            &eq112_node_derivatives,
            &branches,
            &eq112_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_113_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq113_e1501, eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22,) = {
    if (!(s.v[569] != 0.0)) {
        let eq113_e1498: f64 = self.eval_ddt(12, s.v[162]);
        let eq113_e1498_d_n0: f64 = self.ddt_jacobian(s.dn[162][0]);
        let eq113_e1498_d_n1: f64 = self.ddt_jacobian(s.dn[162][1]);
        let eq113_e1498_d_n2: f64 = self.ddt_jacobian(s.dn[162][2]);
        let eq113_e1498_d_n3: f64 = self.ddt_jacobian(s.dn[162][3]);
        let eq113_e1498_d_n4: f64 = self.ddt_jacobian(s.dn[162][4]);
        let eq113_e1498_d_n5: f64 = self.ddt_jacobian(s.dn[162][5]);
        let eq113_e1498_d_n6: f64 = self.ddt_jacobian(s.dn[162][6]);
        let eq113_e1498_d_n7: f64 = self.ddt_jacobian(s.dn[162][7]);
        let eq113_e1498_d_n8: f64 = self.ddt_jacobian(s.dn[162][8]);
        let eq113_e1498_d_n9: f64 = self.ddt_jacobian(s.dn[162][9]);
        let eq113_e1498_d_n10: f64 = self.ddt_jacobian(s.dn[162][10]);
        let eq113_e1498_d_n11: f64 = self.ddt_jacobian(s.dn[162][11]);
        let eq113_e1498_d_n12: f64 = self.ddt_jacobian(s.dn[162][12]);
        let eq113_e1498_d_n13: f64 = self.ddt_jacobian(s.dn[162][13]);
        let eq113_e1498_d_n14: f64 = self.ddt_jacobian(s.dn[162][14]);
        let eq113_e1498_d_n15: f64 = self.ddt_jacobian(s.dn[162][15]);
        let eq113_e1498_d_n16: f64 = self.ddt_jacobian(s.dn[162][16]);
        let eq113_e1498_d_n17: f64 = self.ddt_jacobian(s.dn[162][17]);
        let eq113_e1498_d_n18: f64 = self.ddt_jacobian(s.dn[162][18]);
        let eq113_e1498_d_n19: f64 = self.ddt_jacobian(s.dn[162][19]);
        let eq113_e1498_d_n20: f64 = self.ddt_jacobian(s.dn[162][20]);
        let eq113_e1498_d_n21: f64 = self.ddt_jacobian(s.dn[162][21]);
        let eq113_e1498_d_n22: f64 = self.ddt_jacobian(s.dn[162][22]);
        let eq113_e1499: f64 = (p.p7 * eq113_e1498);
        let eq113_e1499_d_n0: f64 = (p.p7 * eq113_e1498_d_n0);
        let eq113_e1499_d_n1: f64 = (p.p7 * eq113_e1498_d_n1);
        let eq113_e1499_d_n2: f64 = (p.p7 * eq113_e1498_d_n2);
        let eq113_e1499_d_n3: f64 = (p.p7 * eq113_e1498_d_n3);
        let eq113_e1499_d_n4: f64 = (p.p7 * eq113_e1498_d_n4);
        let eq113_e1499_d_n5: f64 = (p.p7 * eq113_e1498_d_n5);
        let eq113_e1499_d_n6: f64 = (p.p7 * eq113_e1498_d_n6);
        let eq113_e1499_d_n7: f64 = (p.p7 * eq113_e1498_d_n7);
        let eq113_e1499_d_n8: f64 = (p.p7 * eq113_e1498_d_n8);
        let eq113_e1499_d_n9: f64 = (p.p7 * eq113_e1498_d_n9);
        let eq113_e1499_d_n10: f64 = (p.p7 * eq113_e1498_d_n10);
        let eq113_e1499_d_n11: f64 = (p.p7 * eq113_e1498_d_n11);
        let eq113_e1499_d_n12: f64 = (p.p7 * eq113_e1498_d_n12);
        let eq113_e1499_d_n13: f64 = (p.p7 * eq113_e1498_d_n13);
        let eq113_e1499_d_n14: f64 = (p.p7 * eq113_e1498_d_n14);
        let eq113_e1499_d_n15: f64 = (p.p7 * eq113_e1498_d_n15);
        let eq113_e1499_d_n16: f64 = (p.p7 * eq113_e1498_d_n16);
        let eq113_e1499_d_n17: f64 = (p.p7 * eq113_e1498_d_n17);
        let eq113_e1499_d_n18: f64 = (p.p7 * eq113_e1498_d_n18);
        let eq113_e1499_d_n19: f64 = (p.p7 * eq113_e1498_d_n19);
        let eq113_e1499_d_n20: f64 = (p.p7 * eq113_e1498_d_n20);
        let eq113_e1499_d_n21: f64 = (p.p7 * eq113_e1498_d_n21);
        let eq113_e1499_d_n22: f64 = (p.p7 * eq113_e1498_d_n22);
        (eq113_e1499, eq113_e1499_d_n0, eq113_e1499_d_n1, eq113_e1499_d_n2, eq113_e1499_d_n3, eq113_e1499_d_n4, eq113_e1499_d_n5, eq113_e1499_d_n6, eq113_e1499_d_n7, eq113_e1499_d_n8, eq113_e1499_d_n9, eq113_e1499_d_n10, eq113_e1499_d_n11, eq113_e1499_d_n12, eq113_e1499_d_n13, eq113_e1499_d_n14, eq113_e1499_d_n15, eq113_e1499_d_n16, eq113_e1499_d_n17, eq113_e1499_d_n18, eq113_e1499_d_n19, eq113_e1499_d_n20, eq113_e1499_d_n21, eq113_e1499_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1501;
        let eq113_node_derivatives: [f64; 23] = [eq113_e1501_d_n0, eq113_e1501_d_n1, eq113_e1501_d_n2, eq113_e1501_d_n3, eq113_e1501_d_n4, eq113_e1501_d_n5, eq113_e1501_d_n6, eq113_e1501_d_n7, eq113_e1501_d_n8, eq113_e1501_d_n9, eq113_e1501_d_n10, eq113_e1501_d_n11, eq113_e1501_d_n12, eq113_e1501_d_n13, eq113_e1501_d_n14, eq113_e1501_d_n15, eq113_e1501_d_n16, eq113_e1501_d_n17, eq113_e1501_d_n18, eq113_e1501_d_n19, eq113_e1501_d_n20, eq113_e1501_d_n21, eq113_e1501_d_n22];
        let eq113_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            self.multiplicity * (eq113_value),
            &nodes,
            &eq113_node_derivatives,
            &branches,
            &eq113_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_114_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq114_e1509, eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22,) = {
    if (!(s.v[569] != 0.0)) {
        let eq114_e1506: f64 = self.eval_ddt(13, s.v[163]);
        let eq114_e1506_d_n0: f64 = self.ddt_jacobian(s.dn[163][0]);
        let eq114_e1506_d_n1: f64 = self.ddt_jacobian(s.dn[163][1]);
        let eq114_e1506_d_n2: f64 = self.ddt_jacobian(s.dn[163][2]);
        let eq114_e1506_d_n3: f64 = self.ddt_jacobian(s.dn[163][3]);
        let eq114_e1506_d_n4: f64 = self.ddt_jacobian(s.dn[163][4]);
        let eq114_e1506_d_n5: f64 = self.ddt_jacobian(s.dn[163][5]);
        let eq114_e1506_d_n6: f64 = self.ddt_jacobian(s.dn[163][6]);
        let eq114_e1506_d_n7: f64 = self.ddt_jacobian(s.dn[163][7]);
        let eq114_e1506_d_n8: f64 = self.ddt_jacobian(s.dn[163][8]);
        let eq114_e1506_d_n9: f64 = self.ddt_jacobian(s.dn[163][9]);
        let eq114_e1506_d_n10: f64 = self.ddt_jacobian(s.dn[163][10]);
        let eq114_e1506_d_n11: f64 = self.ddt_jacobian(s.dn[163][11]);
        let eq114_e1506_d_n12: f64 = self.ddt_jacobian(s.dn[163][12]);
        let eq114_e1506_d_n13: f64 = self.ddt_jacobian(s.dn[163][13]);
        let eq114_e1506_d_n14: f64 = self.ddt_jacobian(s.dn[163][14]);
        let eq114_e1506_d_n15: f64 = self.ddt_jacobian(s.dn[163][15]);
        let eq114_e1506_d_n16: f64 = self.ddt_jacobian(s.dn[163][16]);
        let eq114_e1506_d_n17: f64 = self.ddt_jacobian(s.dn[163][17]);
        let eq114_e1506_d_n18: f64 = self.ddt_jacobian(s.dn[163][18]);
        let eq114_e1506_d_n19: f64 = self.ddt_jacobian(s.dn[163][19]);
        let eq114_e1506_d_n20: f64 = self.ddt_jacobian(s.dn[163][20]);
        let eq114_e1506_d_n21: f64 = self.ddt_jacobian(s.dn[163][21]);
        let eq114_e1506_d_n22: f64 = self.ddt_jacobian(s.dn[163][22]);
        let eq114_e1507: f64 = (p.p7 * eq114_e1506);
        let eq114_e1507_d_n0: f64 = (p.p7 * eq114_e1506_d_n0);
        let eq114_e1507_d_n1: f64 = (p.p7 * eq114_e1506_d_n1);
        let eq114_e1507_d_n2: f64 = (p.p7 * eq114_e1506_d_n2);
        let eq114_e1507_d_n3: f64 = (p.p7 * eq114_e1506_d_n3);
        let eq114_e1507_d_n4: f64 = (p.p7 * eq114_e1506_d_n4);
        let eq114_e1507_d_n5: f64 = (p.p7 * eq114_e1506_d_n5);
        let eq114_e1507_d_n6: f64 = (p.p7 * eq114_e1506_d_n6);
        let eq114_e1507_d_n7: f64 = (p.p7 * eq114_e1506_d_n7);
        let eq114_e1507_d_n8: f64 = (p.p7 * eq114_e1506_d_n8);
        let eq114_e1507_d_n9: f64 = (p.p7 * eq114_e1506_d_n9);
        let eq114_e1507_d_n10: f64 = (p.p7 * eq114_e1506_d_n10);
        let eq114_e1507_d_n11: f64 = (p.p7 * eq114_e1506_d_n11);
        let eq114_e1507_d_n12: f64 = (p.p7 * eq114_e1506_d_n12);
        let eq114_e1507_d_n13: f64 = (p.p7 * eq114_e1506_d_n13);
        let eq114_e1507_d_n14: f64 = (p.p7 * eq114_e1506_d_n14);
        let eq114_e1507_d_n15: f64 = (p.p7 * eq114_e1506_d_n15);
        let eq114_e1507_d_n16: f64 = (p.p7 * eq114_e1506_d_n16);
        let eq114_e1507_d_n17: f64 = (p.p7 * eq114_e1506_d_n17);
        let eq114_e1507_d_n18: f64 = (p.p7 * eq114_e1506_d_n18);
        let eq114_e1507_d_n19: f64 = (p.p7 * eq114_e1506_d_n19);
        let eq114_e1507_d_n20: f64 = (p.p7 * eq114_e1506_d_n20);
        let eq114_e1507_d_n21: f64 = (p.p7 * eq114_e1506_d_n21);
        let eq114_e1507_d_n22: f64 = (p.p7 * eq114_e1506_d_n22);
        (eq114_e1507, eq114_e1507_d_n0, eq114_e1507_d_n1, eq114_e1507_d_n2, eq114_e1507_d_n3, eq114_e1507_d_n4, eq114_e1507_d_n5, eq114_e1507_d_n6, eq114_e1507_d_n7, eq114_e1507_d_n8, eq114_e1507_d_n9, eq114_e1507_d_n10, eq114_e1507_d_n11, eq114_e1507_d_n12, eq114_e1507_d_n13, eq114_e1507_d_n14, eq114_e1507_d_n15, eq114_e1507_d_n16, eq114_e1507_d_n17, eq114_e1507_d_n18, eq114_e1507_d_n19, eq114_e1507_d_n20, eq114_e1507_d_n21, eq114_e1507_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq114_value: f64 = eq114_e1509;
        let eq114_node_derivatives: [f64; 23] = [eq114_e1509_d_n0, eq114_e1509_d_n1, eq114_e1509_d_n2, eq114_e1509_d_n3, eq114_e1509_d_n4, eq114_e1509_d_n5, eq114_e1509_d_n6, eq114_e1509_d_n7, eq114_e1509_d_n8, eq114_e1509_d_n9, eq114_e1509_d_n10, eq114_e1509_d_n11, eq114_e1509_d_n12, eq114_e1509_d_n13, eq114_e1509_d_n14, eq114_e1509_d_n15, eq114_e1509_d_n16, eq114_e1509_d_n17, eq114_e1509_d_n18, eq114_e1509_d_n19, eq114_e1509_d_n20, eq114_e1509_d_n21, eq114_e1509_d_n22];
        let eq114_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            self.multiplicity * (eq114_value),
            &nodes,
            &eq114_node_derivatives,
            &branches,
            &eq114_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_115_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq115_e1512: f64 = self.eval_ddt(14, s.v[164]);
        let eq115_e1512_d_n0: f64 = self.ddt_jacobian(s.dn[164][0]);
        let eq115_e1512_d_n1: f64 = self.ddt_jacobian(s.dn[164][1]);
        let eq115_e1512_d_n2: f64 = self.ddt_jacobian(s.dn[164][2]);
        let eq115_e1512_d_n3: f64 = self.ddt_jacobian(s.dn[164][3]);
        let eq115_e1512_d_n4: f64 = self.ddt_jacobian(s.dn[164][4]);
        let eq115_e1512_d_n5: f64 = self.ddt_jacobian(s.dn[164][5]);
        let eq115_e1512_d_n6: f64 = self.ddt_jacobian(s.dn[164][6]);
        let eq115_e1512_d_n7: f64 = self.ddt_jacobian(s.dn[164][7]);
        let eq115_e1512_d_n8: f64 = self.ddt_jacobian(s.dn[164][8]);
        let eq115_e1512_d_n9: f64 = self.ddt_jacobian(s.dn[164][9]);
        let eq115_e1512_d_n10: f64 = self.ddt_jacobian(s.dn[164][10]);
        let eq115_e1512_d_n11: f64 = self.ddt_jacobian(s.dn[164][11]);
        let eq115_e1512_d_n12: f64 = self.ddt_jacobian(s.dn[164][12]);
        let eq115_e1512_d_n13: f64 = self.ddt_jacobian(s.dn[164][13]);
        let eq115_e1512_d_n14: f64 = self.ddt_jacobian(s.dn[164][14]);
        let eq115_e1512_d_n15: f64 = self.ddt_jacobian(s.dn[164][15]);
        let eq115_e1512_d_n16: f64 = self.ddt_jacobian(s.dn[164][16]);
        let eq115_e1512_d_n17: f64 = self.ddt_jacobian(s.dn[164][17]);
        let eq115_e1512_d_n18: f64 = self.ddt_jacobian(s.dn[164][18]);
        let eq115_e1512_d_n19: f64 = self.ddt_jacobian(s.dn[164][19]);
        let eq115_e1512_d_n20: f64 = self.ddt_jacobian(s.dn[164][20]);
        let eq115_e1512_d_n21: f64 = self.ddt_jacobian(s.dn[164][21]);
        let eq115_e1512_d_n22: f64 = self.ddt_jacobian(s.dn[164][22]);
        let eq115_e1513: f64 = (p.p7 * eq115_e1512);
        let eq115_e1513_d_n0: f64 = (p.p7 * eq115_e1512_d_n0);
        let eq115_e1513_d_n1: f64 = (p.p7 * eq115_e1512_d_n1);
        let eq115_e1513_d_n2: f64 = (p.p7 * eq115_e1512_d_n2);
        let eq115_e1513_d_n3: f64 = (p.p7 * eq115_e1512_d_n3);
        let eq115_e1513_d_n4: f64 = (p.p7 * eq115_e1512_d_n4);
        let eq115_e1513_d_n5: f64 = (p.p7 * eq115_e1512_d_n5);
        let eq115_e1513_d_n6: f64 = (p.p7 * eq115_e1512_d_n6);
        let eq115_e1513_d_n7: f64 = (p.p7 * eq115_e1512_d_n7);
        let eq115_e1513_d_n8: f64 = (p.p7 * eq115_e1512_d_n8);
        let eq115_e1513_d_n9: f64 = (p.p7 * eq115_e1512_d_n9);
        let eq115_e1513_d_n10: f64 = (p.p7 * eq115_e1512_d_n10);
        let eq115_e1513_d_n11: f64 = (p.p7 * eq115_e1512_d_n11);
        let eq115_e1513_d_n12: f64 = (p.p7 * eq115_e1512_d_n12);
        let eq115_e1513_d_n13: f64 = (p.p7 * eq115_e1512_d_n13);
        let eq115_e1513_d_n14: f64 = (p.p7 * eq115_e1512_d_n14);
        let eq115_e1513_d_n15: f64 = (p.p7 * eq115_e1512_d_n15);
        let eq115_e1513_d_n16: f64 = (p.p7 * eq115_e1512_d_n16);
        let eq115_e1513_d_n17: f64 = (p.p7 * eq115_e1512_d_n17);
        let eq115_e1513_d_n18: f64 = (p.p7 * eq115_e1512_d_n18);
        let eq115_e1513_d_n19: f64 = (p.p7 * eq115_e1512_d_n19);
        let eq115_e1513_d_n20: f64 = (p.p7 * eq115_e1512_d_n20);
        let eq115_e1513_d_n21: f64 = (p.p7 * eq115_e1512_d_n21);
        let eq115_e1513_d_n22: f64 = (p.p7 * eq115_e1512_d_n22);
        let eq115_value: f64 = eq115_e1513;
        let eq115_node_derivatives: [f64; 23] = [eq115_e1513_d_n0, eq115_e1513_d_n1, eq115_e1513_d_n2, eq115_e1513_d_n3, eq115_e1513_d_n4, eq115_e1513_d_n5, eq115_e1513_d_n6, eq115_e1513_d_n7, eq115_e1513_d_n8, eq115_e1513_d_n9, eq115_e1513_d_n10, eq115_e1513_d_n11, eq115_e1513_d_n12, eq115_e1513_d_n13, eq115_e1513_d_n14, eq115_e1513_d_n15, eq115_e1513_d_n16, eq115_e1513_d_n17, eq115_e1513_d_n18, eq115_e1513_d_n19, eq115_e1513_d_n20, eq115_e1513_d_n21, eq115_e1513_d_n22];
        let eq115_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq115_value),
            &nodes,
            &eq115_node_derivatives,
            &branches,
            &eq115_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_116_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq116_e1516: f64 = self.eval_ddt(15, s.v[219]);
        let eq116_e1516_d_n0: f64 = self.ddt_jacobian(s.dn[219][0]);
        let eq116_e1516_d_n1: f64 = self.ddt_jacobian(s.dn[219][1]);
        let eq116_e1516_d_n2: f64 = self.ddt_jacobian(s.dn[219][2]);
        let eq116_e1516_d_n3: f64 = self.ddt_jacobian(s.dn[219][3]);
        let eq116_e1516_d_n4: f64 = self.ddt_jacobian(s.dn[219][4]);
        let eq116_e1516_d_n5: f64 = self.ddt_jacobian(s.dn[219][5]);
        let eq116_e1516_d_n6: f64 = self.ddt_jacobian(s.dn[219][6]);
        let eq116_e1516_d_n7: f64 = self.ddt_jacobian(s.dn[219][7]);
        let eq116_e1516_d_n8: f64 = self.ddt_jacobian(s.dn[219][8]);
        let eq116_e1516_d_n9: f64 = self.ddt_jacobian(s.dn[219][9]);
        let eq116_e1516_d_n10: f64 = self.ddt_jacobian(s.dn[219][10]);
        let eq116_e1516_d_n11: f64 = self.ddt_jacobian(s.dn[219][11]);
        let eq116_e1516_d_n12: f64 = self.ddt_jacobian(s.dn[219][12]);
        let eq116_e1516_d_n13: f64 = self.ddt_jacobian(s.dn[219][13]);
        let eq116_e1516_d_n14: f64 = self.ddt_jacobian(s.dn[219][14]);
        let eq116_e1516_d_n15: f64 = self.ddt_jacobian(s.dn[219][15]);
        let eq116_e1516_d_n16: f64 = self.ddt_jacobian(s.dn[219][16]);
        let eq116_e1516_d_n17: f64 = self.ddt_jacobian(s.dn[219][17]);
        let eq116_e1516_d_n18: f64 = self.ddt_jacobian(s.dn[219][18]);
        let eq116_e1516_d_n19: f64 = self.ddt_jacobian(s.dn[219][19]);
        let eq116_e1516_d_n20: f64 = self.ddt_jacobian(s.dn[219][20]);
        let eq116_e1516_d_n21: f64 = self.ddt_jacobian(s.dn[219][21]);
        let eq116_e1516_d_n22: f64 = self.ddt_jacobian(s.dn[219][22]);
        let eq116_e1517: f64 = (p.p7 * eq116_e1516);
        let eq116_e1517_d_n0: f64 = (p.p7 * eq116_e1516_d_n0);
        let eq116_e1517_d_n1: f64 = (p.p7 * eq116_e1516_d_n1);
        let eq116_e1517_d_n2: f64 = (p.p7 * eq116_e1516_d_n2);
        let eq116_e1517_d_n3: f64 = (p.p7 * eq116_e1516_d_n3);
        let eq116_e1517_d_n4: f64 = (p.p7 * eq116_e1516_d_n4);
        let eq116_e1517_d_n5: f64 = (p.p7 * eq116_e1516_d_n5);
        let eq116_e1517_d_n6: f64 = (p.p7 * eq116_e1516_d_n6);
        let eq116_e1517_d_n7: f64 = (p.p7 * eq116_e1516_d_n7);
        let eq116_e1517_d_n8: f64 = (p.p7 * eq116_e1516_d_n8);
        let eq116_e1517_d_n9: f64 = (p.p7 * eq116_e1516_d_n9);
        let eq116_e1517_d_n10: f64 = (p.p7 * eq116_e1516_d_n10);
        let eq116_e1517_d_n11: f64 = (p.p7 * eq116_e1516_d_n11);
        let eq116_e1517_d_n12: f64 = (p.p7 * eq116_e1516_d_n12);
        let eq116_e1517_d_n13: f64 = (p.p7 * eq116_e1516_d_n13);
        let eq116_e1517_d_n14: f64 = (p.p7 * eq116_e1516_d_n14);
        let eq116_e1517_d_n15: f64 = (p.p7 * eq116_e1516_d_n15);
        let eq116_e1517_d_n16: f64 = (p.p7 * eq116_e1516_d_n16);
        let eq116_e1517_d_n17: f64 = (p.p7 * eq116_e1516_d_n17);
        let eq116_e1517_d_n18: f64 = (p.p7 * eq116_e1516_d_n18);
        let eq116_e1517_d_n19: f64 = (p.p7 * eq116_e1516_d_n19);
        let eq116_e1517_d_n20: f64 = (p.p7 * eq116_e1516_d_n20);
        let eq116_e1517_d_n21: f64 = (p.p7 * eq116_e1516_d_n21);
        let eq116_e1517_d_n22: f64 = (p.p7 * eq116_e1516_d_n22);
        let eq116_value: f64 = eq116_e1517;
        let eq116_node_derivatives: [f64; 23] = [eq116_e1517_d_n0, eq116_e1517_d_n1, eq116_e1517_d_n2, eq116_e1517_d_n3, eq116_e1517_d_n4, eq116_e1517_d_n5, eq116_e1517_d_n6, eq116_e1517_d_n7, eq116_e1517_d_n8, eq116_e1517_d_n9, eq116_e1517_d_n10, eq116_e1517_d_n11, eq116_e1517_d_n12, eq116_e1517_d_n13, eq116_e1517_d_n14, eq116_e1517_d_n15, eq116_e1517_d_n16, eq116_e1517_d_n17, eq116_e1517_d_n18, eq116_e1517_d_n19, eq116_e1517_d_n20, eq116_e1517_d_n21, eq116_e1517_d_n22];
        let eq116_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            self.multiplicity * (eq116_value),
            &nodes,
            &eq116_node_derivatives,
            &branches,
            &eq116_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_117_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq117_e1520: f64 = self.eval_ddt(16, s.v[220]);
        let eq117_e1520_d_n0: f64 = self.ddt_jacobian(s.dn[220][0]);
        let eq117_e1520_d_n1: f64 = self.ddt_jacobian(s.dn[220][1]);
        let eq117_e1520_d_n2: f64 = self.ddt_jacobian(s.dn[220][2]);
        let eq117_e1520_d_n3: f64 = self.ddt_jacobian(s.dn[220][3]);
        let eq117_e1520_d_n4: f64 = self.ddt_jacobian(s.dn[220][4]);
        let eq117_e1520_d_n5: f64 = self.ddt_jacobian(s.dn[220][5]);
        let eq117_e1520_d_n6: f64 = self.ddt_jacobian(s.dn[220][6]);
        let eq117_e1520_d_n7: f64 = self.ddt_jacobian(s.dn[220][7]);
        let eq117_e1520_d_n8: f64 = self.ddt_jacobian(s.dn[220][8]);
        let eq117_e1520_d_n9: f64 = self.ddt_jacobian(s.dn[220][9]);
        let eq117_e1520_d_n10: f64 = self.ddt_jacobian(s.dn[220][10]);
        let eq117_e1520_d_n11: f64 = self.ddt_jacobian(s.dn[220][11]);
        let eq117_e1520_d_n12: f64 = self.ddt_jacobian(s.dn[220][12]);
        let eq117_e1520_d_n13: f64 = self.ddt_jacobian(s.dn[220][13]);
        let eq117_e1520_d_n14: f64 = self.ddt_jacobian(s.dn[220][14]);
        let eq117_e1520_d_n15: f64 = self.ddt_jacobian(s.dn[220][15]);
        let eq117_e1520_d_n16: f64 = self.ddt_jacobian(s.dn[220][16]);
        let eq117_e1520_d_n17: f64 = self.ddt_jacobian(s.dn[220][17]);
        let eq117_e1520_d_n18: f64 = self.ddt_jacobian(s.dn[220][18]);
        let eq117_e1520_d_n19: f64 = self.ddt_jacobian(s.dn[220][19]);
        let eq117_e1520_d_n20: f64 = self.ddt_jacobian(s.dn[220][20]);
        let eq117_e1520_d_n21: f64 = self.ddt_jacobian(s.dn[220][21]);
        let eq117_e1520_d_n22: f64 = self.ddt_jacobian(s.dn[220][22]);
        let eq117_e1521: f64 = (p.p7 * eq117_e1520);
        let eq117_e1521_d_n0: f64 = (p.p7 * eq117_e1520_d_n0);
        let eq117_e1521_d_n1: f64 = (p.p7 * eq117_e1520_d_n1);
        let eq117_e1521_d_n2: f64 = (p.p7 * eq117_e1520_d_n2);
        let eq117_e1521_d_n3: f64 = (p.p7 * eq117_e1520_d_n3);
        let eq117_e1521_d_n4: f64 = (p.p7 * eq117_e1520_d_n4);
        let eq117_e1521_d_n5: f64 = (p.p7 * eq117_e1520_d_n5);
        let eq117_e1521_d_n6: f64 = (p.p7 * eq117_e1520_d_n6);
        let eq117_e1521_d_n7: f64 = (p.p7 * eq117_e1520_d_n7);
        let eq117_e1521_d_n8: f64 = (p.p7 * eq117_e1520_d_n8);
        let eq117_e1521_d_n9: f64 = (p.p7 * eq117_e1520_d_n9);
        let eq117_e1521_d_n10: f64 = (p.p7 * eq117_e1520_d_n10);
        let eq117_e1521_d_n11: f64 = (p.p7 * eq117_e1520_d_n11);
        let eq117_e1521_d_n12: f64 = (p.p7 * eq117_e1520_d_n12);
        let eq117_e1521_d_n13: f64 = (p.p7 * eq117_e1520_d_n13);
        let eq117_e1521_d_n14: f64 = (p.p7 * eq117_e1520_d_n14);
        let eq117_e1521_d_n15: f64 = (p.p7 * eq117_e1520_d_n15);
        let eq117_e1521_d_n16: f64 = (p.p7 * eq117_e1520_d_n16);
        let eq117_e1521_d_n17: f64 = (p.p7 * eq117_e1520_d_n17);
        let eq117_e1521_d_n18: f64 = (p.p7 * eq117_e1520_d_n18);
        let eq117_e1521_d_n19: f64 = (p.p7 * eq117_e1520_d_n19);
        let eq117_e1521_d_n20: f64 = (p.p7 * eq117_e1520_d_n20);
        let eq117_e1521_d_n21: f64 = (p.p7 * eq117_e1520_d_n21);
        let eq117_e1521_d_n22: f64 = (p.p7 * eq117_e1520_d_n22);
        let eq117_value: f64 = eq117_e1521;
        let eq117_node_derivatives: [f64; 23] = [eq117_e1521_d_n0, eq117_e1521_d_n1, eq117_e1521_d_n2, eq117_e1521_d_n3, eq117_e1521_d_n4, eq117_e1521_d_n5, eq117_e1521_d_n6, eq117_e1521_d_n7, eq117_e1521_d_n8, eq117_e1521_d_n9, eq117_e1521_d_n10, eq117_e1521_d_n11, eq117_e1521_d_n12, eq117_e1521_d_n13, eq117_e1521_d_n14, eq117_e1521_d_n15, eq117_e1521_d_n16, eq117_e1521_d_n17, eq117_e1521_d_n18, eq117_e1521_d_n19, eq117_e1521_d_n20, eq117_e1521_d_n21, eq117_e1521_d_n22];
        let eq117_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[2]),
            self.multiplicity * (eq117_value),
            &nodes,
            &eq117_node_derivatives,
            &branches,
            &eq117_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_118_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq118_e1524: f64 = self.eval_ddt(17, s.v[221]);
        let eq118_e1524_d_n0: f64 = self.ddt_jacobian(s.dn[221][0]);
        let eq118_e1524_d_n1: f64 = self.ddt_jacobian(s.dn[221][1]);
        let eq118_e1524_d_n2: f64 = self.ddt_jacobian(s.dn[221][2]);
        let eq118_e1524_d_n3: f64 = self.ddt_jacobian(s.dn[221][3]);
        let eq118_e1524_d_n4: f64 = self.ddt_jacobian(s.dn[221][4]);
        let eq118_e1524_d_n5: f64 = self.ddt_jacobian(s.dn[221][5]);
        let eq118_e1524_d_n6: f64 = self.ddt_jacobian(s.dn[221][6]);
        let eq118_e1524_d_n7: f64 = self.ddt_jacobian(s.dn[221][7]);
        let eq118_e1524_d_n8: f64 = self.ddt_jacobian(s.dn[221][8]);
        let eq118_e1524_d_n9: f64 = self.ddt_jacobian(s.dn[221][9]);
        let eq118_e1524_d_n10: f64 = self.ddt_jacobian(s.dn[221][10]);
        let eq118_e1524_d_n11: f64 = self.ddt_jacobian(s.dn[221][11]);
        let eq118_e1524_d_n12: f64 = self.ddt_jacobian(s.dn[221][12]);
        let eq118_e1524_d_n13: f64 = self.ddt_jacobian(s.dn[221][13]);
        let eq118_e1524_d_n14: f64 = self.ddt_jacobian(s.dn[221][14]);
        let eq118_e1524_d_n15: f64 = self.ddt_jacobian(s.dn[221][15]);
        let eq118_e1524_d_n16: f64 = self.ddt_jacobian(s.dn[221][16]);
        let eq118_e1524_d_n17: f64 = self.ddt_jacobian(s.dn[221][17]);
        let eq118_e1524_d_n18: f64 = self.ddt_jacobian(s.dn[221][18]);
        let eq118_e1524_d_n19: f64 = self.ddt_jacobian(s.dn[221][19]);
        let eq118_e1524_d_n20: f64 = self.ddt_jacobian(s.dn[221][20]);
        let eq118_e1524_d_n21: f64 = self.ddt_jacobian(s.dn[221][21]);
        let eq118_e1524_d_n22: f64 = self.ddt_jacobian(s.dn[221][22]);
        let eq118_e1525: f64 = (p.p7 * eq118_e1524);
        let eq118_e1525_d_n0: f64 = (p.p7 * eq118_e1524_d_n0);
        let eq118_e1525_d_n1: f64 = (p.p7 * eq118_e1524_d_n1);
        let eq118_e1525_d_n2: f64 = (p.p7 * eq118_e1524_d_n2);
        let eq118_e1525_d_n3: f64 = (p.p7 * eq118_e1524_d_n3);
        let eq118_e1525_d_n4: f64 = (p.p7 * eq118_e1524_d_n4);
        let eq118_e1525_d_n5: f64 = (p.p7 * eq118_e1524_d_n5);
        let eq118_e1525_d_n6: f64 = (p.p7 * eq118_e1524_d_n6);
        let eq118_e1525_d_n7: f64 = (p.p7 * eq118_e1524_d_n7);
        let eq118_e1525_d_n8: f64 = (p.p7 * eq118_e1524_d_n8);
        let eq118_e1525_d_n9: f64 = (p.p7 * eq118_e1524_d_n9);
        let eq118_e1525_d_n10: f64 = (p.p7 * eq118_e1524_d_n10);
        let eq118_e1525_d_n11: f64 = (p.p7 * eq118_e1524_d_n11);
        let eq118_e1525_d_n12: f64 = (p.p7 * eq118_e1524_d_n12);
        let eq118_e1525_d_n13: f64 = (p.p7 * eq118_e1524_d_n13);
        let eq118_e1525_d_n14: f64 = (p.p7 * eq118_e1524_d_n14);
        let eq118_e1525_d_n15: f64 = (p.p7 * eq118_e1524_d_n15);
        let eq118_e1525_d_n16: f64 = (p.p7 * eq118_e1524_d_n16);
        let eq118_e1525_d_n17: f64 = (p.p7 * eq118_e1524_d_n17);
        let eq118_e1525_d_n18: f64 = (p.p7 * eq118_e1524_d_n18);
        let eq118_e1525_d_n19: f64 = (p.p7 * eq118_e1524_d_n19);
        let eq118_e1525_d_n20: f64 = (p.p7 * eq118_e1524_d_n20);
        let eq118_e1525_d_n21: f64 = (p.p7 * eq118_e1524_d_n21);
        let eq118_e1525_d_n22: f64 = (p.p7 * eq118_e1524_d_n22);
        let eq118_value: f64 = eq118_e1525;
        let eq118_node_derivatives: [f64; 23] = [eq118_e1525_d_n0, eq118_e1525_d_n1, eq118_e1525_d_n2, eq118_e1525_d_n3, eq118_e1525_d_n4, eq118_e1525_d_n5, eq118_e1525_d_n6, eq118_e1525_d_n7, eq118_e1525_d_n8, eq118_e1525_d_n9, eq118_e1525_d_n10, eq118_e1525_d_n11, eq118_e1525_d_n12, eq118_e1525_d_n13, eq118_e1525_d_n14, eq118_e1525_d_n15, eq118_e1525_d_n16, eq118_e1525_d_n17, eq118_e1525_d_n18, eq118_e1525_d_n19, eq118_e1525_d_n20, eq118_e1525_d_n21, eq118_e1525_d_n22];
        let eq118_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[1]),
            self.multiplicity * (eq118_value),
            &nodes,
            &eq118_node_derivatives,
            &branches,
            &eq118_branch_derivatives,
            self.multiplicity,
        );
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
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
        let (eq103_e2818,) = {
    if ((s.v[1734] != 0.0) && (!(s.v[1735] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq103_value: f64 = eq103_e2818;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq103_value),
            &[
            ],
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
        let (eq104_e2832,) = {
    if ((s.v[1734] != 0.0) && (!(s.v[1735] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq104_value: f64 = eq104_e2832;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[5]),
            self.multiplicity * (eq104_value),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq105_e2843, eq105_e2843_d_n0, eq105_e2843_d_n1, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n12, eq105_e2843_d_n13, eq105_e2843_d_n14, eq105_e2843_d_n15, eq105_e2843_d_n16,) = {
    if (s.v[1736] != 0.0) {
        let eq105_e2836: f64 = (s.v[114] * s.v[128]);
        let eq105_e2836_d_n0: f64 = ((s.dn[114][0] * s.v[128]) + (s.v[114] * s.dn[128][0]));
        let eq105_e2836_d_n1: f64 = ((s.dn[114][1] * s.v[128]) + (s.v[114] * s.dn[128][1]));
        let eq105_e2836_d_n2: f64 = ((s.dn[114][2] * s.v[128]) + (s.v[114] * s.dn[128][2]));
        let eq105_e2836_d_n3: f64 = ((s.dn[114][3] * s.v[128]) + (s.v[114] * s.dn[128][3]));
        let eq105_e2836_d_n4: f64 = ((s.dn[114][4] * s.v[128]) + (s.v[114] * s.dn[128][4]));
        let eq105_e2836_d_n5: f64 = ((s.dn[114][5] * s.v[128]) + (s.v[114] * s.dn[128][5]));
        let eq105_e2836_d_n6: f64 = ((s.dn[114][6] * s.v[128]) + (s.v[114] * s.dn[128][6]));
        let eq105_e2836_d_n7: f64 = ((s.dn[114][7] * s.v[128]) + (s.v[114] * s.dn[128][7]));
        let eq105_e2836_d_n8: f64 = ((s.dn[114][8] * s.v[128]) + (s.v[114] * s.dn[128][8]));
        let eq105_e2836_d_n9: f64 = ((s.dn[114][9] * s.v[128]) + (s.v[114] * s.dn[128][9]));
        let eq105_e2836_d_n10: f64 = ((s.dn[114][10] * s.v[128]) + (s.v[114] * s.dn[128][10]));
        let eq105_e2836_d_n11: f64 = ((s.dn[114][11] * s.v[128]) + (s.v[114] * s.dn[128][11]));
        let eq105_e2836_d_n12: f64 = ((s.dn[114][12] * s.v[128]) + (s.v[114] * s.dn[128][12]));
        let eq105_e2836_d_n13: f64 = ((s.dn[114][13] * s.v[128]) + (s.v[114] * s.dn[128][13]));
        let eq105_e2836_d_n14: f64 = ((s.dn[114][14] * s.v[128]) + (s.v[114] * s.dn[128][14]));
        let eq105_e2836_d_n15: f64 = ((s.dn[114][15] * s.v[128]) + (s.v[114] * s.dn[128][15]));
        let eq105_e2836_d_n16: f64 = ((s.dn[114][16] * s.v[128]) + (s.v[114] * s.dn[128][16]));
        let eq105_e2838: f64 = (eq105_e2836 * (nv5 - nv6));
        let eq105_e2838_d_n0: f64 = (eq105_e2836_d_n0 * (nv5 - nv6));
        let eq105_e2838_d_n1: f64 = (eq105_e2836_d_n1 * (nv5 - nv6));
        let eq105_e2838_d_n2: f64 = (eq105_e2836_d_n2 * (nv5 - nv6));
        let eq105_e2838_d_n3: f64 = (eq105_e2836_d_n3 * (nv5 - nv6));
        let eq105_e2838_d_n4: f64 = (eq105_e2836_d_n4 * (nv5 - nv6));
        let eq105_e2838_d_n5: f64 = ((eq105_e2836_d_n5 * (nv5 - nv6)) + eq105_e2836);
        let eq105_e2838_d_n6: f64 = ((eq105_e2836_d_n6 * (nv5 - nv6)) + (-eq105_e2836));
        let eq105_e2838_d_n7: f64 = (eq105_e2836_d_n7 * (nv5 - nv6));
        let eq105_e2838_d_n8: f64 = (eq105_e2836_d_n8 * (nv5 - nv6));
        let eq105_e2838_d_n9: f64 = (eq105_e2836_d_n9 * (nv5 - nv6));
        let eq105_e2838_d_n10: f64 = (eq105_e2836_d_n10 * (nv5 - nv6));
        let eq105_e2838_d_n11: f64 = (eq105_e2836_d_n11 * (nv5 - nv6));
        let eq105_e2838_d_n12: f64 = (eq105_e2836_d_n12 * (nv5 - nv6));
        let eq105_e2838_d_n13: f64 = (eq105_e2836_d_n13 * (nv5 - nv6));
        let eq105_e2838_d_n14: f64 = (eq105_e2836_d_n14 * (nv5 - nv6));
        let eq105_e2838_d_n15: f64 = (eq105_e2836_d_n15 * (nv5 - nv6));
        let eq105_e2838_d_n16: f64 = (eq105_e2836_d_n16 * (nv5 - nv6));
        let eq105_e2840: f64 = (eq105_e2838 * s.v[124]);
        let eq105_e2840_d_n0: f64 = ((eq105_e2838_d_n0 * s.v[124]) + (eq105_e2838 * s.dn[124][0]));
        let eq105_e2840_d_n1: f64 = ((eq105_e2838_d_n1 * s.v[124]) + (eq105_e2838 * s.dn[124][1]));
        let eq105_e2840_d_n2: f64 = ((eq105_e2838_d_n2 * s.v[124]) + (eq105_e2838 * s.dn[124][2]));
        let eq105_e2840_d_n3: f64 = ((eq105_e2838_d_n3 * s.v[124]) + (eq105_e2838 * s.dn[124][3]));
        let eq105_e2840_d_n4: f64 = ((eq105_e2838_d_n4 * s.v[124]) + (eq105_e2838 * s.dn[124][4]));
        let eq105_e2840_d_n5: f64 = ((eq105_e2838_d_n5 * s.v[124]) + (eq105_e2838 * s.dn[124][5]));
        let eq105_e2840_d_n6: f64 = ((eq105_e2838_d_n6 * s.v[124]) + (eq105_e2838 * s.dn[124][6]));
        let eq105_e2840_d_n7: f64 = ((eq105_e2838_d_n7 * s.v[124]) + (eq105_e2838 * s.dn[124][7]));
        let eq105_e2840_d_n8: f64 = ((eq105_e2838_d_n8 * s.v[124]) + (eq105_e2838 * s.dn[124][8]));
        let eq105_e2840_d_n9: f64 = ((eq105_e2838_d_n9 * s.v[124]) + (eq105_e2838 * s.dn[124][9]));
        let eq105_e2840_d_n10: f64 = ((eq105_e2838_d_n10 * s.v[124]) + (eq105_e2838 * s.dn[124][10]));
        let eq105_e2840_d_n11: f64 = ((eq105_e2838_d_n11 * s.v[124]) + (eq105_e2838 * s.dn[124][11]));
        let eq105_e2840_d_n12: f64 = ((eq105_e2838_d_n12 * s.v[124]) + (eq105_e2838 * s.dn[124][12]));
        let eq105_e2840_d_n13: f64 = ((eq105_e2838_d_n13 * s.v[124]) + (eq105_e2838 * s.dn[124][13]));
        let eq105_e2840_d_n14: f64 = ((eq105_e2838_d_n14 * s.v[124]) + (eq105_e2838 * s.dn[124][14]));
        let eq105_e2840_d_n15: f64 = ((eq105_e2838_d_n15 * s.v[124]) + (eq105_e2838 * s.dn[124][15]));
        let eq105_e2840_d_n16: f64 = ((eq105_e2838_d_n16 * s.v[124]) + (eq105_e2838 * s.dn[124][16]));
        let eq105_e2841: f64 = (-eq105_e2840);
        let eq105_e2841_d_n0: f64 = (-eq105_e2840_d_n0);
        let eq105_e2841_d_n1: f64 = (-eq105_e2840_d_n1);
        let eq105_e2841_d_n2: f64 = (-eq105_e2840_d_n2);
        let eq105_e2841_d_n3: f64 = (-eq105_e2840_d_n3);
        let eq105_e2841_d_n4: f64 = (-eq105_e2840_d_n4);
        let eq105_e2841_d_n5: f64 = (-eq105_e2840_d_n5);
        let eq105_e2841_d_n6: f64 = (-eq105_e2840_d_n6);
        let eq105_e2841_d_n7: f64 = (-eq105_e2840_d_n7);
        let eq105_e2841_d_n8: f64 = (-eq105_e2840_d_n8);
        let eq105_e2841_d_n9: f64 = (-eq105_e2840_d_n9);
        let eq105_e2841_d_n10: f64 = (-eq105_e2840_d_n10);
        let eq105_e2841_d_n11: f64 = (-eq105_e2840_d_n11);
        let eq105_e2841_d_n12: f64 = (-eq105_e2840_d_n12);
        let eq105_e2841_d_n13: f64 = (-eq105_e2840_d_n13);
        let eq105_e2841_d_n14: f64 = (-eq105_e2840_d_n14);
        let eq105_e2841_d_n15: f64 = (-eq105_e2840_d_n15);
        let eq105_e2841_d_n16: f64 = (-eq105_e2840_d_n16);
        (eq105_e2841, eq105_e2841_d_n0, eq105_e2841_d_n1, eq105_e2841_d_n2, eq105_e2841_d_n3, eq105_e2841_d_n4, eq105_e2841_d_n5, eq105_e2841_d_n6, eq105_e2841_d_n7, eq105_e2841_d_n8, eq105_e2841_d_n9, eq105_e2841_d_n10, eq105_e2841_d_n11, eq105_e2841_d_n12, eq105_e2841_d_n13, eq105_e2841_d_n14, eq105_e2841_d_n15, eq105_e2841_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e2843;
        let eq105_node_derivatives: [f64; 17] = [eq105_e2843_d_n0, eq105_e2843_d_n1, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n12, eq105_e2843_d_n13, eq105_e2843_d_n14, eq105_e2843_d_n15, eq105_e2843_d_n16];
        let eq105_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq105_value),
            &nodes,
            &eq105_node_derivatives,
            &branches,
            &eq105_branch_derivatives,
            self.multiplicity,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq106_e2854, eq106_e2854_d_n0, eq106_e2854_d_n1, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n12, eq106_e2854_d_n13, eq106_e2854_d_n14, eq106_e2854_d_n15, eq106_e2854_d_n16,) = {
    if ((s.v[1736] != 0.0) && (s.v[1737] != 0.0)) {
        let eq106_e2848: f64 = (-(nv0 - nv9));
        let eq106_e2848_d_n0: f64 = (-1.0);
        let eq106_e2850: f64 = (eq106_e2848 * (nv0 - nv9));
        let eq106_e2850_d_n0: f64 = ((eq106_e2848_d_n0 * (nv0 - nv9)) + eq106_e2848);
        let eq106_e2850_d_n9: f64 = ((nv0 - nv9) + (-eq106_e2848));
        let eq106_e2852: f64 = (eq106_e2850 * s.v[596]);
        let eq106_e2852_d_n0: f64 = ((eq106_e2850_d_n0 * s.v[596]) + (eq106_e2850 * s.dn[596][0]));
        let eq106_e2852_d_n1: f64 = (eq106_e2850 * s.dn[596][1]);
        let eq106_e2852_d_n2: f64 = (eq106_e2850 * s.dn[596][2]);
        let eq106_e2852_d_n3: f64 = (eq106_e2850 * s.dn[596][3]);
        let eq106_e2852_d_n4: f64 = (eq106_e2850 * s.dn[596][4]);
        let eq106_e2852_d_n5: f64 = (eq106_e2850 * s.dn[596][5]);
        let eq106_e2852_d_n6: f64 = (eq106_e2850 * s.dn[596][6]);
        let eq106_e2852_d_n7: f64 = (eq106_e2850 * s.dn[596][7]);
        let eq106_e2852_d_n8: f64 = (eq106_e2850 * s.dn[596][8]);
        let eq106_e2852_d_n9: f64 = ((eq106_e2850_d_n9 * s.v[596]) + (eq106_e2850 * s.dn[596][9]));
        let eq106_e2852_d_n10: f64 = (eq106_e2850 * s.dn[596][10]);
        let eq106_e2852_d_n11: f64 = (eq106_e2850 * s.dn[596][11]);
        let eq106_e2852_d_n12: f64 = (eq106_e2850 * s.dn[596][12]);
        let eq106_e2852_d_n13: f64 = (eq106_e2850 * s.dn[596][13]);
        let eq106_e2852_d_n14: f64 = (eq106_e2850 * s.dn[596][14]);
        let eq106_e2852_d_n15: f64 = (eq106_e2850 * s.dn[596][15]);
        let eq106_e2852_d_n16: f64 = (eq106_e2850 * s.dn[596][16]);
        (eq106_e2852, eq106_e2852_d_n0, eq106_e2852_d_n1, eq106_e2852_d_n2, eq106_e2852_d_n3, eq106_e2852_d_n4, eq106_e2852_d_n5, eq106_e2852_d_n6, eq106_e2852_d_n7, eq106_e2852_d_n8, eq106_e2852_d_n9, eq106_e2852_d_n10, eq106_e2852_d_n11, eq106_e2852_d_n12, eq106_e2852_d_n13, eq106_e2852_d_n14, eq106_e2852_d_n15, eq106_e2852_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq106_value: f64 = eq106_e2854;
        let eq106_node_derivatives: [f64; 17] = [eq106_e2854_d_n0, eq106_e2854_d_n1, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n12, eq106_e2854_d_n13, eq106_e2854_d_n14, eq106_e2854_d_n15, eq106_e2854_d_n16];
        let eq106_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq107_e2867, eq107_e2867_d_n0, eq107_e2867_d_n1, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n12, eq107_e2867_d_n13, eq107_e2867_d_n14, eq107_e2867_d_n15, eq107_e2867_d_n16,) = {
    if (((s.v[1736] != 0.0) && (s.v[1737] != 0.0)) && (s.v[1738] != 0.0)) {
        let eq107_e2861: f64 = (-(nv9 - nv7));
        let eq107_e2861_d_n9: f64 = (-1.0);
        let eq107_e2863: f64 = (eq107_e2861 * (nv9 - nv7));
        let eq107_e2863_d_n7: f64 = ((nv9 - nv7) + (-eq107_e2861));
        let eq107_e2863_d_n9: f64 = ((eq107_e2861_d_n9 * (nv9 - nv7)) + eq107_e2861);
        let eq107_e2865: f64 = (eq107_e2863 * s.v[1042]);
        let eq107_e2865_d_n0: f64 = (eq107_e2863 * s.dn[1042][0]);
        let eq107_e2865_d_n1: f64 = (eq107_e2863 * s.dn[1042][1]);
        let eq107_e2865_d_n2: f64 = (eq107_e2863 * s.dn[1042][2]);
        let eq107_e2865_d_n3: f64 = (eq107_e2863 * s.dn[1042][3]);
        let eq107_e2865_d_n4: f64 = (eq107_e2863 * s.dn[1042][4]);
        let eq107_e2865_d_n5: f64 = (eq107_e2863 * s.dn[1042][5]);
        let eq107_e2865_d_n6: f64 = (eq107_e2863 * s.dn[1042][6]);
        let eq107_e2865_d_n7: f64 = ((eq107_e2863_d_n7 * s.v[1042]) + (eq107_e2863 * s.dn[1042][7]));
        let eq107_e2865_d_n8: f64 = (eq107_e2863 * s.dn[1042][8]);
        let eq107_e2865_d_n9: f64 = ((eq107_e2863_d_n9 * s.v[1042]) + (eq107_e2863 * s.dn[1042][9]));
        let eq107_e2865_d_n10: f64 = (eq107_e2863 * s.dn[1042][10]);
        let eq107_e2865_d_n11: f64 = (eq107_e2863 * s.dn[1042][11]);
        let eq107_e2865_d_n12: f64 = (eq107_e2863 * s.dn[1042][12]);
        let eq107_e2865_d_n13: f64 = (eq107_e2863 * s.dn[1042][13]);
        let eq107_e2865_d_n14: f64 = (eq107_e2863 * s.dn[1042][14]);
        let eq107_e2865_d_n15: f64 = (eq107_e2863 * s.dn[1042][15]);
        let eq107_e2865_d_n16: f64 = (eq107_e2863 * s.dn[1042][16]);
        (eq107_e2865, eq107_e2865_d_n0, eq107_e2865_d_n1, eq107_e2865_d_n2, eq107_e2865_d_n3, eq107_e2865_d_n4, eq107_e2865_d_n5, eq107_e2865_d_n6, eq107_e2865_d_n7, eq107_e2865_d_n8, eq107_e2865_d_n9, eq107_e2865_d_n10, eq107_e2865_d_n11, eq107_e2865_d_n12, eq107_e2865_d_n13, eq107_e2865_d_n14, eq107_e2865_d_n15, eq107_e2865_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq107_value: f64 = eq107_e2867;
        let eq107_node_derivatives: [f64; 17] = [eq107_e2867_d_n0, eq107_e2867_d_n1, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n12, eq107_e2867_d_n13, eq107_e2867_d_n14, eq107_e2867_d_n15, eq107_e2867_d_n16];
        let eq107_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq108_e2878, eq108_e2878_d_n0, eq108_e2878_d_n1, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n12, eq108_e2878_d_n13, eq108_e2878_d_n14, eq108_e2878_d_n15, eq108_e2878_d_n16,) = {
    if ((s.v[1736] != 0.0) && (s.v[1739] != 0.0)) {
        let eq108_e2872: f64 = (-(nv2 - nv8));
        let eq108_e2872_d_n2: f64 = (-1.0);
        let eq108_e2874: f64 = (eq108_e2872 * (nv2 - nv8));
        let eq108_e2874_d_n2: f64 = ((eq108_e2872_d_n2 * (nv2 - nv8)) + eq108_e2872);
        let eq108_e2874_d_n8: f64 = ((nv2 - nv8) + (-eq108_e2872));
        let eq108_e2876: f64 = (eq108_e2874 * s.v[595]);
        let eq108_e2876_d_n0: f64 = (eq108_e2874 * s.dn[595][0]);
        let eq108_e2876_d_n1: f64 = (eq108_e2874 * s.dn[595][1]);
        let eq108_e2876_d_n2: f64 = ((eq108_e2874_d_n2 * s.v[595]) + (eq108_e2874 * s.dn[595][2]));
        let eq108_e2876_d_n3: f64 = (eq108_e2874 * s.dn[595][3]);
        let eq108_e2876_d_n4: f64 = (eq108_e2874 * s.dn[595][4]);
        let eq108_e2876_d_n5: f64 = (eq108_e2874 * s.dn[595][5]);
        let eq108_e2876_d_n6: f64 = (eq108_e2874 * s.dn[595][6]);
        let eq108_e2876_d_n7: f64 = (eq108_e2874 * s.dn[595][7]);
        let eq108_e2876_d_n8: f64 = ((eq108_e2874_d_n8 * s.v[595]) + (eq108_e2874 * s.dn[595][8]));
        let eq108_e2876_d_n9: f64 = (eq108_e2874 * s.dn[595][9]);
        let eq108_e2876_d_n10: f64 = (eq108_e2874 * s.dn[595][10]);
        let eq108_e2876_d_n11: f64 = (eq108_e2874 * s.dn[595][11]);
        let eq108_e2876_d_n12: f64 = (eq108_e2874 * s.dn[595][12]);
        let eq108_e2876_d_n13: f64 = (eq108_e2874 * s.dn[595][13]);
        let eq108_e2876_d_n14: f64 = (eq108_e2874 * s.dn[595][14]);
        let eq108_e2876_d_n15: f64 = (eq108_e2874 * s.dn[595][15]);
        let eq108_e2876_d_n16: f64 = (eq108_e2874 * s.dn[595][16]);
        (eq108_e2876, eq108_e2876_d_n0, eq108_e2876_d_n1, eq108_e2876_d_n2, eq108_e2876_d_n3, eq108_e2876_d_n4, eq108_e2876_d_n5, eq108_e2876_d_n6, eq108_e2876_d_n7, eq108_e2876_d_n8, eq108_e2876_d_n9, eq108_e2876_d_n10, eq108_e2876_d_n11, eq108_e2876_d_n12, eq108_e2876_d_n13, eq108_e2876_d_n14, eq108_e2876_d_n15, eq108_e2876_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq108_value: f64 = eq108_e2878;
        let eq108_node_derivatives: [f64; 17] = [eq108_e2878_d_n0, eq108_e2878_d_n1, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n12, eq108_e2878_d_n13, eq108_e2878_d_n14, eq108_e2878_d_n15, eq108_e2878_d_n16];
        let eq108_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq108_value),
            &nodes,
            &eq108_node_derivatives,
            &branches,
            &eq108_branch_derivatives,
            self.multiplicity,
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq109_e2891, eq109_e2891_d_n0, eq109_e2891_d_n1, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n12, eq109_e2891_d_n13, eq109_e2891_d_n14, eq109_e2891_d_n15, eq109_e2891_d_n16,) = {
    if (((s.v[1736] != 0.0) && (s.v[1739] != 0.0)) && (s.v[1740] != 0.0)) {
        let eq109_e2885: f64 = (-(nv8 - nv6));
        let eq109_e2885_d_n8: f64 = (-1.0);
        let eq109_e2887: f64 = (eq109_e2885 * (nv8 - nv6));
        let eq109_e2887_d_n6: f64 = ((nv8 - nv6) + (-eq109_e2885));
        let eq109_e2887_d_n8: f64 = ((eq109_e2885_d_n8 * (nv8 - nv6)) + eq109_e2885);
        let eq109_e2889: f64 = (eq109_e2887 * s.v[1043]);
        let eq109_e2889_d_n0: f64 = (eq109_e2887 * s.dn[1043][0]);
        let eq109_e2889_d_n1: f64 = (eq109_e2887 * s.dn[1043][1]);
        let eq109_e2889_d_n2: f64 = (eq109_e2887 * s.dn[1043][2]);
        let eq109_e2889_d_n3: f64 = (eq109_e2887 * s.dn[1043][3]);
        let eq109_e2889_d_n4: f64 = (eq109_e2887 * s.dn[1043][4]);
        let eq109_e2889_d_n5: f64 = (eq109_e2887 * s.dn[1043][5]);
        let eq109_e2889_d_n6: f64 = ((eq109_e2887_d_n6 * s.v[1043]) + (eq109_e2887 * s.dn[1043][6]));
        let eq109_e2889_d_n7: f64 = (eq109_e2887 * s.dn[1043][7]);
        let eq109_e2889_d_n8: f64 = ((eq109_e2887_d_n8 * s.v[1043]) + (eq109_e2887 * s.dn[1043][8]));
        let eq109_e2889_d_n9: f64 = (eq109_e2887 * s.dn[1043][9]);
        let eq109_e2889_d_n10: f64 = (eq109_e2887 * s.dn[1043][10]);
        let eq109_e2889_d_n11: f64 = (eq109_e2887 * s.dn[1043][11]);
        let eq109_e2889_d_n12: f64 = (eq109_e2887 * s.dn[1043][12]);
        let eq109_e2889_d_n13: f64 = (eq109_e2887 * s.dn[1043][13]);
        let eq109_e2889_d_n14: f64 = (eq109_e2887 * s.dn[1043][14]);
        let eq109_e2889_d_n15: f64 = (eq109_e2887 * s.dn[1043][15]);
        let eq109_e2889_d_n16: f64 = (eq109_e2887 * s.dn[1043][16]);
        (eq109_e2889, eq109_e2889_d_n0, eq109_e2889_d_n1, eq109_e2889_d_n2, eq109_e2889_d_n3, eq109_e2889_d_n4, eq109_e2889_d_n5, eq109_e2889_d_n6, eq109_e2889_d_n7, eq109_e2889_d_n8, eq109_e2889_d_n9, eq109_e2889_d_n10, eq109_e2889_d_n11, eq109_e2889_d_n12, eq109_e2889_d_n13, eq109_e2889_d_n14, eq109_e2889_d_n15, eq109_e2889_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e2891;
        let eq109_node_derivatives: [f64; 17] = [eq109_e2891_d_n0, eq109_e2891_d_n1, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n12, eq109_e2891_d_n13, eq109_e2891_d_n14, eq109_e2891_d_n15, eq109_e2891_d_n16];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq110_e2897, eq110_e2897_d_n0, eq110_e2897_d_n1, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n12, eq110_e2897_d_n13, eq110_e2897_d_n14, eq110_e2897_d_n15, eq110_e2897_d_n16,) = {
    if (s.v[1736] != 0.0) {
        let eq110_e2895: f64 = ((nv4 - 0.0) * s.v[633]);
        let eq110_e2895_d_n0: f64 = ((nv4 - 0.0) * s.dn[633][0]);
        let eq110_e2895_d_n1: f64 = ((nv4 - 0.0) * s.dn[633][1]);
        let eq110_e2895_d_n2: f64 = ((nv4 - 0.0) * s.dn[633][2]);
        let eq110_e2895_d_n3: f64 = ((nv4 - 0.0) * s.dn[633][3]);
        let eq110_e2895_d_n4: f64 = (s.v[633] + ((nv4 - 0.0) * s.dn[633][4]));
        let eq110_e2895_d_n5: f64 = ((nv4 - 0.0) * s.dn[633][5]);
        let eq110_e2895_d_n6: f64 = ((nv4 - 0.0) * s.dn[633][6]);
        let eq110_e2895_d_n7: f64 = ((nv4 - 0.0) * s.dn[633][7]);
        let eq110_e2895_d_n8: f64 = ((nv4 - 0.0) * s.dn[633][8]);
        let eq110_e2895_d_n9: f64 = ((nv4 - 0.0) * s.dn[633][9]);
        let eq110_e2895_d_n10: f64 = ((nv4 - 0.0) * s.dn[633][10]);
        let eq110_e2895_d_n11: f64 = ((nv4 - 0.0) * s.dn[633][11]);
        let eq110_e2895_d_n12: f64 = ((nv4 - 0.0) * s.dn[633][12]);
        let eq110_e2895_d_n13: f64 = ((nv4 - 0.0) * s.dn[633][13]);
        let eq110_e2895_d_n14: f64 = ((nv4 - 0.0) * s.dn[633][14]);
        let eq110_e2895_d_n15: f64 = ((nv4 - 0.0) * s.dn[633][15]);
        let eq110_e2895_d_n16: f64 = ((nv4 - 0.0) * s.dn[633][16]);
        (eq110_e2895, eq110_e2895_d_n0, eq110_e2895_d_n1, eq110_e2895_d_n2, eq110_e2895_d_n3, eq110_e2895_d_n4, eq110_e2895_d_n5, eq110_e2895_d_n6, eq110_e2895_d_n7, eq110_e2895_d_n8, eq110_e2895_d_n9, eq110_e2895_d_n10, eq110_e2895_d_n11, eq110_e2895_d_n12, eq110_e2895_d_n13, eq110_e2895_d_n14, eq110_e2895_d_n15, eq110_e2895_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq110_value: f64 = eq110_e2897;
        let eq110_node_derivatives: [f64; 17] = [eq110_e2897_d_n0, eq110_e2897_d_n1, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n12, eq110_e2897_d_n13, eq110_e2897_d_n14, eq110_e2897_d_n15, eq110_e2897_d_n16];
        let eq110_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16,) = {
    if (s.v[1736] != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * s.v[634]);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * s.dn[634][0]);
        let eq111_e2901_d_n1: f64 = ((nv4 - 0.0) * s.dn[634][1]);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * s.dn[634][2]);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * s.dn[634][3]);
        let eq111_e2901_d_n4: f64 = (s.v[634] + ((nv4 - 0.0) * s.dn[634][4]));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * s.dn[634][5]);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * s.dn[634][6]);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * s.dn[634][7]);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * s.dn[634][8]);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * s.dn[634][9]);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * s.dn[634][10]);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * s.dn[634][11]);
        let eq111_e2901_d_n12: f64 = ((nv4 - 0.0) * s.dn[634][12]);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * s.dn[634][13]);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * s.dn[634][14]);
        let eq111_e2901_d_n15: f64 = ((nv4 - 0.0) * s.dn[634][15]);
        let eq111_e2901_d_n16: f64 = ((nv4 - 0.0) * s.dn[634][16]);
        let eq111_e2902: f64 = self.eval_ddt(27, eq111_e2901);
        let eq111_e2902_d_n0: f64 = self.ddt_jacobian(eq111_e2901_d_n0);
        let eq111_e2902_d_n1: f64 = self.ddt_jacobian(eq111_e2901_d_n1);
        let eq111_e2902_d_n2: f64 = self.ddt_jacobian(eq111_e2901_d_n2);
        let eq111_e2902_d_n3: f64 = self.ddt_jacobian(eq111_e2901_d_n3);
        let eq111_e2902_d_n4: f64 = self.ddt_jacobian(eq111_e2901_d_n4);
        let eq111_e2902_d_n5: f64 = self.ddt_jacobian(eq111_e2901_d_n5);
        let eq111_e2902_d_n6: f64 = self.ddt_jacobian(eq111_e2901_d_n6);
        let eq111_e2902_d_n7: f64 = self.ddt_jacobian(eq111_e2901_d_n7);
        let eq111_e2902_d_n8: f64 = self.ddt_jacobian(eq111_e2901_d_n8);
        let eq111_e2902_d_n9: f64 = self.ddt_jacobian(eq111_e2901_d_n9);
        let eq111_e2902_d_n10: f64 = self.ddt_jacobian(eq111_e2901_d_n10);
        let eq111_e2902_d_n11: f64 = self.ddt_jacobian(eq111_e2901_d_n11);
        let eq111_e2902_d_n12: f64 = self.ddt_jacobian(eq111_e2901_d_n12);
        let eq111_e2902_d_n13: f64 = self.ddt_jacobian(eq111_e2901_d_n13);
        let eq111_e2902_d_n14: f64 = self.ddt_jacobian(eq111_e2901_d_n14);
        let eq111_e2902_d_n15: f64 = self.ddt_jacobian(eq111_e2901_d_n15);
        let eq111_e2902_d_n16: f64 = self.ddt_jacobian(eq111_e2901_d_n16);
        (eq111_e2902, eq111_e2902_d_n0, eq111_e2902_d_n1, eq111_e2902_d_n2, eq111_e2902_d_n3, eq111_e2902_d_n4, eq111_e2902_d_n5, eq111_e2902_d_n6, eq111_e2902_d_n7, eq111_e2902_d_n8, eq111_e2902_d_n9, eq111_e2902_d_n10, eq111_e2902_d_n11, eq111_e2902_d_n12, eq111_e2902_d_n13, eq111_e2902_d_n14, eq111_e2902_d_n15, eq111_e2902_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e2904;
        let eq111_node_derivatives: [f64; 17] = [eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
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
        let (eq112_e2909,) = {
    if (!(s.v[1736] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq112_value: f64 = eq112_e2909;
        stamper.stamp_potential(
            branches[17],
            eq112_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_4_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n1, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n12, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_d_n15, eq4_e1979_d_n16, eq4_e1979_q, eq4_e1979_q_d_n0, eq4_e1979_q_d_n1, eq4_e1979_q_d_n2, eq4_e1979_q_d_n3, eq4_e1979_q_d_n4, eq4_e1979_q_d_n5, eq4_e1979_q_d_n6, eq4_e1979_q_d_n7, eq4_e1979_q_d_n8, eq4_e1979_q_d_n9, eq4_e1979_q_d_n10, eq4_e1979_q_d_n11, eq4_e1979_q_d_n12, eq4_e1979_q_d_n13, eq4_e1979_q_d_n14, eq4_e1979_q_d_n15, eq4_e1979_q_d_n16,) = {
    if (!(s.v[1696] != 0.0)) {
        let eq4_e1976_q: f64 = s.v[137];
        let eq4_e1977: f64 = (s.v[114] * s.v[137]);
        let eq4_e1977_d_n0: f64 = ((s.dn[114][0] * s.v[137]) + (s.v[114] * s.dn[137][0]));
        let eq4_e1977_d_n1: f64 = ((s.dn[114][1] * s.v[137]) + (s.v[114] * s.dn[137][1]));
        let eq4_e1977_d_n2: f64 = ((s.dn[114][2] * s.v[137]) + (s.v[114] * s.dn[137][2]));
        let eq4_e1977_d_n3: f64 = ((s.dn[114][3] * s.v[137]) + (s.v[114] * s.dn[137][3]));
        let eq4_e1977_d_n4: f64 = ((s.dn[114][4] * s.v[137]) + (s.v[114] * s.dn[137][4]));
        let eq4_e1977_d_n5: f64 = ((s.dn[114][5] * s.v[137]) + (s.v[114] * s.dn[137][5]));
        let eq4_e1977_d_n6: f64 = ((s.dn[114][6] * s.v[137]) + (s.v[114] * s.dn[137][6]));
        let eq4_e1977_d_n7: f64 = ((s.dn[114][7] * s.v[137]) + (s.v[114] * s.dn[137][7]));
        let eq4_e1977_d_n8: f64 = ((s.dn[114][8] * s.v[137]) + (s.v[114] * s.dn[137][8]));
        let eq4_e1977_d_n9: f64 = ((s.dn[114][9] * s.v[137]) + (s.v[114] * s.dn[137][9]));
        let eq4_e1977_d_n10: f64 = ((s.dn[114][10] * s.v[137]) + (s.v[114] * s.dn[137][10]));
        let eq4_e1977_d_n11: f64 = ((s.dn[114][11] * s.v[137]) + (s.v[114] * s.dn[137][11]));
        let eq4_e1977_d_n12: f64 = ((s.dn[114][12] * s.v[137]) + (s.v[114] * s.dn[137][12]));
        let eq4_e1977_d_n13: f64 = ((s.dn[114][13] * s.v[137]) + (s.v[114] * s.dn[137][13]));
        let eq4_e1977_d_n14: f64 = ((s.dn[114][14] * s.v[137]) + (s.v[114] * s.dn[137][14]));
        let eq4_e1977_d_n15: f64 = ((s.dn[114][15] * s.v[137]) + (s.v[114] * s.dn[137][15]));
        let eq4_e1977_d_n16: f64 = ((s.dn[114][16] * s.v[137]) + (s.v[114] * s.dn[137][16]));
        let eq4_e1977_q: f64 = (s.v[114] * eq4_e1976_q);
        let eq4_e1977_q_d_n0: f64 = ((s.dn[114][0] * eq4_e1976_q) + (s.v[114] * s.dn[137][0]));
        let eq4_e1977_q_d_n1: f64 = ((s.dn[114][1] * eq4_e1976_q) + (s.v[114] * s.dn[137][1]));
        let eq4_e1977_q_d_n2: f64 = ((s.dn[114][2] * eq4_e1976_q) + (s.v[114] * s.dn[137][2]));
        let eq4_e1977_q_d_n3: f64 = ((s.dn[114][3] * eq4_e1976_q) + (s.v[114] * s.dn[137][3]));
        let eq4_e1977_q_d_n4: f64 = ((s.dn[114][4] * eq4_e1976_q) + (s.v[114] * s.dn[137][4]));
        let eq4_e1977_q_d_n5: f64 = ((s.dn[114][5] * eq4_e1976_q) + (s.v[114] * s.dn[137][5]));
        let eq4_e1977_q_d_n6: f64 = ((s.dn[114][6] * eq4_e1976_q) + (s.v[114] * s.dn[137][6]));
        let eq4_e1977_q_d_n7: f64 = ((s.dn[114][7] * eq4_e1976_q) + (s.v[114] * s.dn[137][7]));
        let eq4_e1977_q_d_n8: f64 = ((s.dn[114][8] * eq4_e1976_q) + (s.v[114] * s.dn[137][8]));
        let eq4_e1977_q_d_n9: f64 = ((s.dn[114][9] * eq4_e1976_q) + (s.v[114] * s.dn[137][9]));
        let eq4_e1977_q_d_n10: f64 = ((s.dn[114][10] * eq4_e1976_q) + (s.v[114] * s.dn[137][10]));
        let eq4_e1977_q_d_n11: f64 = ((s.dn[114][11] * eq4_e1976_q) + (s.v[114] * s.dn[137][11]));
        let eq4_e1977_q_d_n12: f64 = ((s.dn[114][12] * eq4_e1976_q) + (s.v[114] * s.dn[137][12]));
        let eq4_e1977_q_d_n13: f64 = ((s.dn[114][13] * eq4_e1976_q) + (s.v[114] * s.dn[137][13]));
        let eq4_e1977_q_d_n14: f64 = ((s.dn[114][14] * eq4_e1976_q) + (s.v[114] * s.dn[137][14]));
        let eq4_e1977_q_d_n15: f64 = ((s.dn[114][15] * eq4_e1976_q) + (s.v[114] * s.dn[137][15]));
        let eq4_e1977_q_d_n16: f64 = ((s.dn[114][16] * eq4_e1976_q) + (s.v[114] * s.dn[137][16]));
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n1, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n12, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_d_n15, eq4_e1977_d_n16, eq4_e1977_q, eq4_e1977_q_d_n0, eq4_e1977_q_d_n1, eq4_e1977_q_d_n2, eq4_e1977_q_d_n3, eq4_e1977_q_d_n4, eq4_e1977_q_d_n5, eq4_e1977_q_d_n6, eq4_e1977_q_d_n7, eq4_e1977_q_d_n8, eq4_e1977_q_d_n9, eq4_e1977_q_d_n10, eq4_e1977_q_d_n11, eq4_e1977_q_d_n12, eq4_e1977_q_d_n13, eq4_e1977_q_d_n14, eq4_e1977_q_d_n15, eq4_e1977_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 17] = [eq4_e1979_q_d_n0, eq4_e1979_q_d_n1, eq4_e1979_q_d_n2, eq4_e1979_q_d_n3, eq4_e1979_q_d_n4, eq4_e1979_q_d_n5, eq4_e1979_q_d_n6, eq4_e1979_q_d_n7, eq4_e1979_q_d_n8, eq4_e1979_q_d_n9, eq4_e1979_q_d_n10, eq4_e1979_q_d_n11, eq4_e1979_q_d_n12, eq4_e1979_q_d_n13, eq4_e1979_q_d_n14, eq4_e1979_q_d_n15, eq4_e1979_q_d_n16];
        let eq4_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &nodes,
            &eq4_reactive_node_derivatives,
            &branches,
            &eq4_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_5_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n1, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n12, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_d_n15, eq5_e1987_d_n16, eq5_e1987_q, eq5_e1987_q_d_n0, eq5_e1987_q_d_n1, eq5_e1987_q_d_n2, eq5_e1987_q_d_n3, eq5_e1987_q_d_n4, eq5_e1987_q_d_n5, eq5_e1987_q_d_n6, eq5_e1987_q_d_n7, eq5_e1987_q_d_n8, eq5_e1987_q_d_n9, eq5_e1987_q_d_n10, eq5_e1987_q_d_n11, eq5_e1987_q_d_n12, eq5_e1987_q_d_n13, eq5_e1987_q_d_n14, eq5_e1987_q_d_n15, eq5_e1987_q_d_n16,) = {
    if (!(s.v[1696] != 0.0)) {
        let eq5_e1984_q: f64 = s.v[138];
        let eq5_e1985: f64 = (s.v[114] * s.v[138]);
        let eq5_e1985_d_n0: f64 = ((s.dn[114][0] * s.v[138]) + (s.v[114] * s.dn[138][0]));
        let eq5_e1985_d_n1: f64 = ((s.dn[114][1] * s.v[138]) + (s.v[114] * s.dn[138][1]));
        let eq5_e1985_d_n2: f64 = ((s.dn[114][2] * s.v[138]) + (s.v[114] * s.dn[138][2]));
        let eq5_e1985_d_n3: f64 = ((s.dn[114][3] * s.v[138]) + (s.v[114] * s.dn[138][3]));
        let eq5_e1985_d_n4: f64 = ((s.dn[114][4] * s.v[138]) + (s.v[114] * s.dn[138][4]));
        let eq5_e1985_d_n5: f64 = ((s.dn[114][5] * s.v[138]) + (s.v[114] * s.dn[138][5]));
        let eq5_e1985_d_n6: f64 = ((s.dn[114][6] * s.v[138]) + (s.v[114] * s.dn[138][6]));
        let eq5_e1985_d_n7: f64 = ((s.dn[114][7] * s.v[138]) + (s.v[114] * s.dn[138][7]));
        let eq5_e1985_d_n8: f64 = ((s.dn[114][8] * s.v[138]) + (s.v[114] * s.dn[138][8]));
        let eq5_e1985_d_n9: f64 = ((s.dn[114][9] * s.v[138]) + (s.v[114] * s.dn[138][9]));
        let eq5_e1985_d_n10: f64 = ((s.dn[114][10] * s.v[138]) + (s.v[114] * s.dn[138][10]));
        let eq5_e1985_d_n11: f64 = ((s.dn[114][11] * s.v[138]) + (s.v[114] * s.dn[138][11]));
        let eq5_e1985_d_n12: f64 = ((s.dn[114][12] * s.v[138]) + (s.v[114] * s.dn[138][12]));
        let eq5_e1985_d_n13: f64 = ((s.dn[114][13] * s.v[138]) + (s.v[114] * s.dn[138][13]));
        let eq5_e1985_d_n14: f64 = ((s.dn[114][14] * s.v[138]) + (s.v[114] * s.dn[138][14]));
        let eq5_e1985_d_n15: f64 = ((s.dn[114][15] * s.v[138]) + (s.v[114] * s.dn[138][15]));
        let eq5_e1985_d_n16: f64 = ((s.dn[114][16] * s.v[138]) + (s.v[114] * s.dn[138][16]));
        let eq5_e1985_q: f64 = (s.v[114] * eq5_e1984_q);
        let eq5_e1985_q_d_n0: f64 = ((s.dn[114][0] * eq5_e1984_q) + (s.v[114] * s.dn[138][0]));
        let eq5_e1985_q_d_n1: f64 = ((s.dn[114][1] * eq5_e1984_q) + (s.v[114] * s.dn[138][1]));
        let eq5_e1985_q_d_n2: f64 = ((s.dn[114][2] * eq5_e1984_q) + (s.v[114] * s.dn[138][2]));
        let eq5_e1985_q_d_n3: f64 = ((s.dn[114][3] * eq5_e1984_q) + (s.v[114] * s.dn[138][3]));
        let eq5_e1985_q_d_n4: f64 = ((s.dn[114][4] * eq5_e1984_q) + (s.v[114] * s.dn[138][4]));
        let eq5_e1985_q_d_n5: f64 = ((s.dn[114][5] * eq5_e1984_q) + (s.v[114] * s.dn[138][5]));
        let eq5_e1985_q_d_n6: f64 = ((s.dn[114][6] * eq5_e1984_q) + (s.v[114] * s.dn[138][6]));
        let eq5_e1985_q_d_n7: f64 = ((s.dn[114][7] * eq5_e1984_q) + (s.v[114] * s.dn[138][7]));
        let eq5_e1985_q_d_n8: f64 = ((s.dn[114][8] * eq5_e1984_q) + (s.v[114] * s.dn[138][8]));
        let eq5_e1985_q_d_n9: f64 = ((s.dn[114][9] * eq5_e1984_q) + (s.v[114] * s.dn[138][9]));
        let eq5_e1985_q_d_n10: f64 = ((s.dn[114][10] * eq5_e1984_q) + (s.v[114] * s.dn[138][10]));
        let eq5_e1985_q_d_n11: f64 = ((s.dn[114][11] * eq5_e1984_q) + (s.v[114] * s.dn[138][11]));
        let eq5_e1985_q_d_n12: f64 = ((s.dn[114][12] * eq5_e1984_q) + (s.v[114] * s.dn[138][12]));
        let eq5_e1985_q_d_n13: f64 = ((s.dn[114][13] * eq5_e1984_q) + (s.v[114] * s.dn[138][13]));
        let eq5_e1985_q_d_n14: f64 = ((s.dn[114][14] * eq5_e1984_q) + (s.v[114] * s.dn[138][14]));
        let eq5_e1985_q_d_n15: f64 = ((s.dn[114][15] * eq5_e1984_q) + (s.v[114] * s.dn[138][15]));
        let eq5_e1985_q_d_n16: f64 = ((s.dn[114][16] * eq5_e1984_q) + (s.v[114] * s.dn[138][16]));
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n1, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n12, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_d_n15, eq5_e1985_d_n16, eq5_e1985_q, eq5_e1985_q_d_n0, eq5_e1985_q_d_n1, eq5_e1985_q_d_n2, eq5_e1985_q_d_n3, eq5_e1985_q_d_n4, eq5_e1985_q_d_n5, eq5_e1985_q_d_n6, eq5_e1985_q_d_n7, eq5_e1985_q_d_n8, eq5_e1985_q_d_n9, eq5_e1985_q_d_n10, eq5_e1985_q_d_n11, eq5_e1985_q_d_n12, eq5_e1985_q_d_n13, eq5_e1985_q_d_n14, eq5_e1985_q_d_n15, eq5_e1985_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 17] = [eq5_e1987_q_d_n0, eq5_e1987_q_d_n1, eq5_e1987_q_d_n2, eq5_e1987_q_d_n3, eq5_e1987_q_d_n4, eq5_e1987_q_d_n5, eq5_e1987_q_d_n6, eq5_e1987_q_d_n7, eq5_e1987_q_d_n8, eq5_e1987_q_d_n9, eq5_e1987_q_d_n10, eq5_e1987_q_d_n11, eq5_e1987_q_d_n12, eq5_e1987_q_d_n13, eq5_e1987_q_d_n14, eq5_e1987_q_d_n15, eq5_e1987_q_d_n16];
        let eq5_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            &nodes,
            &eq5_reactive_node_derivatives,
            &branches,
            &eq5_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq36_e2281_q: f64 = s.v[507];
        let eq36_e2282: f64 = (s.v[114] * s.v[507]);
        let eq36_e2282_d_n0: f64 = ((s.dn[114][0] * s.v[507]) + (s.v[114] * s.dn[507][0]));
        let eq36_e2282_d_n1: f64 = ((s.dn[114][1] * s.v[507]) + (s.v[114] * s.dn[507][1]));
        let eq36_e2282_d_n2: f64 = ((s.dn[114][2] * s.v[507]) + (s.v[114] * s.dn[507][2]));
        let eq36_e2282_d_n3: f64 = ((s.dn[114][3] * s.v[507]) + (s.v[114] * s.dn[507][3]));
        let eq36_e2282_d_n4: f64 = ((s.dn[114][4] * s.v[507]) + (s.v[114] * s.dn[507][4]));
        let eq36_e2282_d_n5: f64 = ((s.dn[114][5] * s.v[507]) + (s.v[114] * s.dn[507][5]));
        let eq36_e2282_d_n6: f64 = ((s.dn[114][6] * s.v[507]) + (s.v[114] * s.dn[507][6]));
        let eq36_e2282_d_n7: f64 = ((s.dn[114][7] * s.v[507]) + (s.v[114] * s.dn[507][7]));
        let eq36_e2282_d_n8: f64 = ((s.dn[114][8] * s.v[507]) + (s.v[114] * s.dn[507][8]));
        let eq36_e2282_d_n9: f64 = ((s.dn[114][9] * s.v[507]) + (s.v[114] * s.dn[507][9]));
        let eq36_e2282_d_n10: f64 = ((s.dn[114][10] * s.v[507]) + (s.v[114] * s.dn[507][10]));
        let eq36_e2282_d_n11: f64 = ((s.dn[114][11] * s.v[507]) + (s.v[114] * s.dn[507][11]));
        let eq36_e2282_d_n12: f64 = ((s.dn[114][12] * s.v[507]) + (s.v[114] * s.dn[507][12]));
        let eq36_e2282_d_n13: f64 = ((s.dn[114][13] * s.v[507]) + (s.v[114] * s.dn[507][13]));
        let eq36_e2282_d_n14: f64 = ((s.dn[114][14] * s.v[507]) + (s.v[114] * s.dn[507][14]));
        let eq36_e2282_d_n15: f64 = ((s.dn[114][15] * s.v[507]) + (s.v[114] * s.dn[507][15]));
        let eq36_e2282_d_n16: f64 = ((s.dn[114][16] * s.v[507]) + (s.v[114] * s.dn[507][16]));
        let eq36_e2282_q: f64 = (s.v[114] * eq36_e2281_q);
        let eq36_e2282_q_d_n0: f64 = ((s.dn[114][0] * eq36_e2281_q) + (s.v[114] * s.dn[507][0]));
        let eq36_e2282_q_d_n1: f64 = ((s.dn[114][1] * eq36_e2281_q) + (s.v[114] * s.dn[507][1]));
        let eq36_e2282_q_d_n2: f64 = ((s.dn[114][2] * eq36_e2281_q) + (s.v[114] * s.dn[507][2]));
        let eq36_e2282_q_d_n3: f64 = ((s.dn[114][3] * eq36_e2281_q) + (s.v[114] * s.dn[507][3]));
        let eq36_e2282_q_d_n4: f64 = ((s.dn[114][4] * eq36_e2281_q) + (s.v[114] * s.dn[507][4]));
        let eq36_e2282_q_d_n5: f64 = ((s.dn[114][5] * eq36_e2281_q) + (s.v[114] * s.dn[507][5]));
        let eq36_e2282_q_d_n6: f64 = ((s.dn[114][6] * eq36_e2281_q) + (s.v[114] * s.dn[507][6]));
        let eq36_e2282_q_d_n7: f64 = ((s.dn[114][7] * eq36_e2281_q) + (s.v[114] * s.dn[507][7]));
        let eq36_e2282_q_d_n8: f64 = ((s.dn[114][8] * eq36_e2281_q) + (s.v[114] * s.dn[507][8]));
        let eq36_e2282_q_d_n9: f64 = ((s.dn[114][9] * eq36_e2281_q) + (s.v[114] * s.dn[507][9]));
        let eq36_e2282_q_d_n10: f64 = ((s.dn[114][10] * eq36_e2281_q) + (s.v[114] * s.dn[507][10]));
        let eq36_e2282_q_d_n11: f64 = ((s.dn[114][11] * eq36_e2281_q) + (s.v[114] * s.dn[507][11]));
        let eq36_e2282_q_d_n12: f64 = ((s.dn[114][12] * eq36_e2281_q) + (s.v[114] * s.dn[507][12]));
        let eq36_e2282_q_d_n13: f64 = ((s.dn[114][13] * eq36_e2281_q) + (s.v[114] * s.dn[507][13]));
        let eq36_e2282_q_d_n14: f64 = ((s.dn[114][14] * eq36_e2281_q) + (s.v[114] * s.dn[507][14]));
        let eq36_e2282_q_d_n15: f64 = ((s.dn[114][15] * eq36_e2281_q) + (s.v[114] * s.dn[507][15]));
        let eq36_e2282_q_d_n16: f64 = ((s.dn[114][16] * eq36_e2281_q) + (s.v[114] * s.dn[507][16]));
        let eq36_reactive_node_derivatives: [f64; 17] = [eq36_e2282_q_d_n0, eq36_e2282_q_d_n1, eq36_e2282_q_d_n2, eq36_e2282_q_d_n3, eq36_e2282_q_d_n4, eq36_e2282_q_d_n5, eq36_e2282_q_d_n6, eq36_e2282_q_d_n7, eq36_e2282_q_d_n8, eq36_e2282_q_d_n9, eq36_e2282_q_d_n10, eq36_e2282_q_d_n11, eq36_e2282_q_d_n12, eq36_e2282_q_d_n13, eq36_e2282_q_d_n14, eq36_e2282_q_d_n15, eq36_e2282_q_d_n16];
        let eq36_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            &nodes,
            &eq36_reactive_node_derivatives,
            &branches,
            &eq36_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq37_e2285_q: f64 = s.v[508];
        let eq37_e2286: f64 = (s.v[114] * s.v[508]);
        let eq37_e2286_d_n0: f64 = ((s.dn[114][0] * s.v[508]) + (s.v[114] * s.dn[508][0]));
        let eq37_e2286_d_n1: f64 = ((s.dn[114][1] * s.v[508]) + (s.v[114] * s.dn[508][1]));
        let eq37_e2286_d_n2: f64 = ((s.dn[114][2] * s.v[508]) + (s.v[114] * s.dn[508][2]));
        let eq37_e2286_d_n3: f64 = ((s.dn[114][3] * s.v[508]) + (s.v[114] * s.dn[508][3]));
        let eq37_e2286_d_n4: f64 = ((s.dn[114][4] * s.v[508]) + (s.v[114] * s.dn[508][4]));
        let eq37_e2286_d_n5: f64 = ((s.dn[114][5] * s.v[508]) + (s.v[114] * s.dn[508][5]));
        let eq37_e2286_d_n6: f64 = ((s.dn[114][6] * s.v[508]) + (s.v[114] * s.dn[508][6]));
        let eq37_e2286_d_n7: f64 = ((s.dn[114][7] * s.v[508]) + (s.v[114] * s.dn[508][7]));
        let eq37_e2286_d_n8: f64 = ((s.dn[114][8] * s.v[508]) + (s.v[114] * s.dn[508][8]));
        let eq37_e2286_d_n9: f64 = ((s.dn[114][9] * s.v[508]) + (s.v[114] * s.dn[508][9]));
        let eq37_e2286_d_n10: f64 = ((s.dn[114][10] * s.v[508]) + (s.v[114] * s.dn[508][10]));
        let eq37_e2286_d_n11: f64 = ((s.dn[114][11] * s.v[508]) + (s.v[114] * s.dn[508][11]));
        let eq37_e2286_d_n12: f64 = ((s.dn[114][12] * s.v[508]) + (s.v[114] * s.dn[508][12]));
        let eq37_e2286_d_n13: f64 = ((s.dn[114][13] * s.v[508]) + (s.v[114] * s.dn[508][13]));
        let eq37_e2286_d_n14: f64 = ((s.dn[114][14] * s.v[508]) + (s.v[114] * s.dn[508][14]));
        let eq37_e2286_d_n15: f64 = ((s.dn[114][15] * s.v[508]) + (s.v[114] * s.dn[508][15]));
        let eq37_e2286_d_n16: f64 = ((s.dn[114][16] * s.v[508]) + (s.v[114] * s.dn[508][16]));
        let eq37_e2286_q: f64 = (s.v[114] * eq37_e2285_q);
        let eq37_e2286_q_d_n0: f64 = ((s.dn[114][0] * eq37_e2285_q) + (s.v[114] * s.dn[508][0]));
        let eq37_e2286_q_d_n1: f64 = ((s.dn[114][1] * eq37_e2285_q) + (s.v[114] * s.dn[508][1]));
        let eq37_e2286_q_d_n2: f64 = ((s.dn[114][2] * eq37_e2285_q) + (s.v[114] * s.dn[508][2]));
        let eq37_e2286_q_d_n3: f64 = ((s.dn[114][3] * eq37_e2285_q) + (s.v[114] * s.dn[508][3]));
        let eq37_e2286_q_d_n4: f64 = ((s.dn[114][4] * eq37_e2285_q) + (s.v[114] * s.dn[508][4]));
        let eq37_e2286_q_d_n5: f64 = ((s.dn[114][5] * eq37_e2285_q) + (s.v[114] * s.dn[508][5]));
        let eq37_e2286_q_d_n6: f64 = ((s.dn[114][6] * eq37_e2285_q) + (s.v[114] * s.dn[508][6]));
        let eq37_e2286_q_d_n7: f64 = ((s.dn[114][7] * eq37_e2285_q) + (s.v[114] * s.dn[508][7]));
        let eq37_e2286_q_d_n8: f64 = ((s.dn[114][8] * eq37_e2285_q) + (s.v[114] * s.dn[508][8]));
        let eq37_e2286_q_d_n9: f64 = ((s.dn[114][9] * eq37_e2285_q) + (s.v[114] * s.dn[508][9]));
        let eq37_e2286_q_d_n10: f64 = ((s.dn[114][10] * eq37_e2285_q) + (s.v[114] * s.dn[508][10]));
        let eq37_e2286_q_d_n11: f64 = ((s.dn[114][11] * eq37_e2285_q) + (s.v[114] * s.dn[508][11]));
        let eq37_e2286_q_d_n12: f64 = ((s.dn[114][12] * eq37_e2285_q) + (s.v[114] * s.dn[508][12]));
        let eq37_e2286_q_d_n13: f64 = ((s.dn[114][13] * eq37_e2285_q) + (s.v[114] * s.dn[508][13]));
        let eq37_e2286_q_d_n14: f64 = ((s.dn[114][14] * eq37_e2285_q) + (s.v[114] * s.dn[508][14]));
        let eq37_e2286_q_d_n15: f64 = ((s.dn[114][15] * eq37_e2285_q) + (s.v[114] * s.dn[508][15]));
        let eq37_e2286_q_d_n16: f64 = ((s.dn[114][16] * eq37_e2285_q) + (s.v[114] * s.dn[508][16]));
        let eq37_reactive_node_derivatives: [f64; 17] = [eq37_e2286_q_d_n0, eq37_e2286_q_d_n1, eq37_e2286_q_d_n2, eq37_e2286_q_d_n3, eq37_e2286_q_d_n4, eq37_e2286_q_d_n5, eq37_e2286_q_d_n6, eq37_e2286_q_d_n7, eq37_e2286_q_d_n8, eq37_e2286_q_d_n9, eq37_e2286_q_d_n10, eq37_e2286_q_d_n11, eq37_e2286_q_d_n12, eq37_e2286_q_d_n13, eq37_e2286_q_d_n14, eq37_e2286_q_d_n15, eq37_e2286_q_d_n16];
        let eq37_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            &nodes,
            &eq37_reactive_node_derivatives,
            &branches,
            &eq37_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_e2289_q: f64 = s.v[509];
        let eq38_e2290: f64 = (s.v[114] * s.v[509]);
        let eq38_e2290_d_n0: f64 = ((s.dn[114][0] * s.v[509]) + (s.v[114] * s.dn[509][0]));
        let eq38_e2290_d_n1: f64 = ((s.dn[114][1] * s.v[509]) + (s.v[114] * s.dn[509][1]));
        let eq38_e2290_d_n2: f64 = ((s.dn[114][2] * s.v[509]) + (s.v[114] * s.dn[509][2]));
        let eq38_e2290_d_n3: f64 = ((s.dn[114][3] * s.v[509]) + (s.v[114] * s.dn[509][3]));
        let eq38_e2290_d_n4: f64 = ((s.dn[114][4] * s.v[509]) + (s.v[114] * s.dn[509][4]));
        let eq38_e2290_d_n5: f64 = ((s.dn[114][5] * s.v[509]) + (s.v[114] * s.dn[509][5]));
        let eq38_e2290_d_n6: f64 = ((s.dn[114][6] * s.v[509]) + (s.v[114] * s.dn[509][6]));
        let eq38_e2290_d_n7: f64 = ((s.dn[114][7] * s.v[509]) + (s.v[114] * s.dn[509][7]));
        let eq38_e2290_d_n8: f64 = ((s.dn[114][8] * s.v[509]) + (s.v[114] * s.dn[509][8]));
        let eq38_e2290_d_n9: f64 = ((s.dn[114][9] * s.v[509]) + (s.v[114] * s.dn[509][9]));
        let eq38_e2290_d_n10: f64 = ((s.dn[114][10] * s.v[509]) + (s.v[114] * s.dn[509][10]));
        let eq38_e2290_d_n11: f64 = ((s.dn[114][11] * s.v[509]) + (s.v[114] * s.dn[509][11]));
        let eq38_e2290_d_n12: f64 = ((s.dn[114][12] * s.v[509]) + (s.v[114] * s.dn[509][12]));
        let eq38_e2290_d_n13: f64 = ((s.dn[114][13] * s.v[509]) + (s.v[114] * s.dn[509][13]));
        let eq38_e2290_d_n14: f64 = ((s.dn[114][14] * s.v[509]) + (s.v[114] * s.dn[509][14]));
        let eq38_e2290_d_n15: f64 = ((s.dn[114][15] * s.v[509]) + (s.v[114] * s.dn[509][15]));
        let eq38_e2290_d_n16: f64 = ((s.dn[114][16] * s.v[509]) + (s.v[114] * s.dn[509][16]));
        let eq38_e2290_q: f64 = (s.v[114] * eq38_e2289_q);
        let eq38_e2290_q_d_n0: f64 = ((s.dn[114][0] * eq38_e2289_q) + (s.v[114] * s.dn[509][0]));
        let eq38_e2290_q_d_n1: f64 = ((s.dn[114][1] * eq38_e2289_q) + (s.v[114] * s.dn[509][1]));
        let eq38_e2290_q_d_n2: f64 = ((s.dn[114][2] * eq38_e2289_q) + (s.v[114] * s.dn[509][2]));
        let eq38_e2290_q_d_n3: f64 = ((s.dn[114][3] * eq38_e2289_q) + (s.v[114] * s.dn[509][3]));
        let eq38_e2290_q_d_n4: f64 = ((s.dn[114][4] * eq38_e2289_q) + (s.v[114] * s.dn[509][4]));
        let eq38_e2290_q_d_n5: f64 = ((s.dn[114][5] * eq38_e2289_q) + (s.v[114] * s.dn[509][5]));
        let eq38_e2290_q_d_n6: f64 = ((s.dn[114][6] * eq38_e2289_q) + (s.v[114] * s.dn[509][6]));
        let eq38_e2290_q_d_n7: f64 = ((s.dn[114][7] * eq38_e2289_q) + (s.v[114] * s.dn[509][7]));
        let eq38_e2290_q_d_n8: f64 = ((s.dn[114][8] * eq38_e2289_q) + (s.v[114] * s.dn[509][8]));
        let eq38_e2290_q_d_n9: f64 = ((s.dn[114][9] * eq38_e2289_q) + (s.v[114] * s.dn[509][9]));
        let eq38_e2290_q_d_n10: f64 = ((s.dn[114][10] * eq38_e2289_q) + (s.v[114] * s.dn[509][10]));
        let eq38_e2290_q_d_n11: f64 = ((s.dn[114][11] * eq38_e2289_q) + (s.v[114] * s.dn[509][11]));
        let eq38_e2290_q_d_n12: f64 = ((s.dn[114][12] * eq38_e2289_q) + (s.v[114] * s.dn[509][12]));
        let eq38_e2290_q_d_n13: f64 = ((s.dn[114][13] * eq38_e2289_q) + (s.v[114] * s.dn[509][13]));
        let eq38_e2290_q_d_n14: f64 = ((s.dn[114][14] * eq38_e2289_q) + (s.v[114] * s.dn[509][14]));
        let eq38_e2290_q_d_n15: f64 = ((s.dn[114][15] * eq38_e2289_q) + (s.v[114] * s.dn[509][15]));
        let eq38_e2290_q_d_n16: f64 = ((s.dn[114][16] * eq38_e2289_q) + (s.v[114] * s.dn[509][16]));
        let eq38_reactive_node_derivatives: [f64; 17] = [eq38_e2290_q_d_n0, eq38_e2290_q_d_n1, eq38_e2290_q_d_n2, eq38_e2290_q_d_n3, eq38_e2290_q_d_n4, eq38_e2290_q_d_n5, eq38_e2290_q_d_n6, eq38_e2290_q_d_n7, eq38_e2290_q_d_n8, eq38_e2290_q_d_n9, eq38_e2290_q_d_n10, eq38_e2290_q_d_n11, eq38_e2290_q_d_n12, eq38_e2290_q_d_n13, eq38_e2290_q_d_n14, eq38_e2290_q_d_n15, eq38_e2290_q_d_n16];
        let eq38_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            &nodes,
            &eq38_reactive_node_derivatives,
            &branches,
            &eq38_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16, eq39_e2295_q, eq39_e2295_q_d_n0, eq39_e2295_q_d_n1, eq39_e2295_q_d_n2, eq39_e2295_q_d_n3, eq39_e2295_q_d_n4, eq39_e2295_q_d_n5, eq39_e2295_q_d_n6, eq39_e2295_q_d_n7, eq39_e2295_q_d_n8, eq39_e2295_q_d_n9, eq39_e2295_q_d_n10, eq39_e2295_q_d_n11, eq39_e2295_q_d_n12, eq39_e2295_q_d_n13, eq39_e2295_q_d_n14, eq39_e2295_q_d_n15, eq39_e2295_q_d_n16,) = {
    if (s.v[1705] != 0.0) {
        let eq39_e2293_q: f64 = s.v[505];
        (s.v[505], s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], eq39_e2293_q, s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 17] = [eq39_e2295_q_d_n0, eq39_e2295_q_d_n1, eq39_e2295_q_d_n2, eq39_e2295_q_d_n3, eq39_e2295_q_d_n4, eq39_e2295_q_d_n5, eq39_e2295_q_d_n6, eq39_e2295_q_d_n7, eq39_e2295_q_d_n8, eq39_e2295_q_d_n9, eq39_e2295_q_d_n10, eq39_e2295_q_d_n11, eq39_e2295_q_d_n12, eq39_e2295_q_d_n13, eq39_e2295_q_d_n14, eq39_e2295_q_d_n15, eq39_e2295_q_d_n16];
        let eq39_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            &nodes,
            &eq39_reactive_node_derivatives,
            &branches,
            &eq39_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

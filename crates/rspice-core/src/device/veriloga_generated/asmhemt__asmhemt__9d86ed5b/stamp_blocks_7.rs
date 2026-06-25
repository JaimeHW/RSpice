#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq55_e957, eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22,) = {
    if (!(s.v[423] != 0.0)) {
        let eq55_e951: f64 = 0.0;
        let eq55_e953: f64 = (eq55_e951 * (nv9 - nv8));
        let eq55_e953_d_n8: f64 = (-eq55_e951);
        let eq55_e954: f64 = (s.v[206] + eq55_e953);
        let eq55_e954_d_n8: f64 = (s.dn[206][8] + eq55_e953_d_n8);
        let eq55_e954_d_n9: f64 = (s.dn[206][9] + eq55_e951);
        let eq55_e955: f64 = (p.p6 * eq55_e954);
        let eq55_e955_d_n0: f64 = (p.p6 * s.dn[206][0]);
        let eq55_e955_d_n1: f64 = (p.p6 * s.dn[206][1]);
        let eq55_e955_d_n2: f64 = (p.p6 * s.dn[206][2]);
        let eq55_e955_d_n3: f64 = (p.p6 * s.dn[206][3]);
        let eq55_e955_d_n4: f64 = (p.p6 * s.dn[206][4]);
        let eq55_e955_d_n5: f64 = (p.p6 * s.dn[206][5]);
        let eq55_e955_d_n6: f64 = (p.p6 * s.dn[206][6]);
        let eq55_e955_d_n7: f64 = (p.p6 * s.dn[206][7]);
        let eq55_e955_d_n8: f64 = (p.p6 * eq55_e954_d_n8);
        let eq55_e955_d_n9: f64 = (p.p6 * eq55_e954_d_n9);
        let eq55_e955_d_n10: f64 = (p.p6 * s.dn[206][10]);
        let eq55_e955_d_n11: f64 = (p.p6 * s.dn[206][11]);
        let eq55_e955_d_n12: f64 = (p.p6 * s.dn[206][12]);
        let eq55_e955_d_n13: f64 = (p.p6 * s.dn[206][13]);
        let eq55_e955_d_n14: f64 = (p.p6 * s.dn[206][14]);
        let eq55_e955_d_n15: f64 = (p.p6 * s.dn[206][15]);
        let eq55_e955_d_n16: f64 = (p.p6 * s.dn[206][16]);
        let eq55_e955_d_n17: f64 = (p.p6 * s.dn[206][17]);
        let eq55_e955_d_n18: f64 = (p.p6 * s.dn[206][18]);
        let eq55_e955_d_n19: f64 = (p.p6 * s.dn[206][19]);
        let eq55_e955_d_n20: f64 = (p.p6 * s.dn[206][20]);
        let eq55_e955_d_n21: f64 = (p.p6 * s.dn[206][21]);
        let eq55_e955_d_n22: f64 = (p.p6 * s.dn[206][22]);
        (eq55_e955, eq55_e955_d_n0, eq55_e955_d_n1, eq55_e955_d_n2, eq55_e955_d_n3, eq55_e955_d_n4, eq55_e955_d_n5, eq55_e955_d_n6, eq55_e955_d_n7, eq55_e955_d_n8, eq55_e955_d_n9, eq55_e955_d_n10, eq55_e955_d_n11, eq55_e955_d_n12, eq55_e955_d_n13, eq55_e955_d_n14, eq55_e955_d_n15, eq55_e955_d_n16, eq55_e955_d_n17, eq55_e955_d_n18, eq55_e955_d_n19, eq55_e955_d_n20, eq55_e955_d_n21, eq55_e955_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e957;
        let eq55_node_derivatives: [f64; 23] = [eq55_e957_d_n0, eq55_e957_d_n1, eq55_e957_d_n2, eq55_e957_d_n3, eq55_e957_d_n4, eq55_e957_d_n5, eq55_e957_d_n6, eq55_e957_d_n7, eq55_e957_d_n8, eq55_e957_d_n9, eq55_e957_d_n10, eq55_e957_d_n11, eq55_e957_d_n12, eq55_e957_d_n13, eq55_e957_d_n14, eq55_e957_d_n15, eq55_e957_d_n16, eq55_e957_d_n17, eq55_e957_d_n18, eq55_e957_d_n19, eq55_e957_d_n20, eq55_e957_d_n21, eq55_e957_d_n22];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq56_e970, eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n10, eq56_e970_d_n11, eq56_e970_d_n12, eq56_e970_d_n13, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22,) = {
    if (!(s.v[423] != 0.0)) {
        let eq56_e964: f64 = 0.0;
        let eq56_e966: f64 = (eq56_e964 * (nv9 - nv7));
        let eq56_e966_d_n7: f64 = (-eq56_e964);
        let eq56_e967: f64 = (s.v[207] + eq56_e966);
        let eq56_e967_d_n7: f64 = (s.dn[207][7] + eq56_e966_d_n7);
        let eq56_e967_d_n9: f64 = (s.dn[207][9] + eq56_e964);
        let eq56_e968: f64 = (p.p6 * eq56_e967);
        let eq56_e968_d_n0: f64 = (p.p6 * s.dn[207][0]);
        let eq56_e968_d_n1: f64 = (p.p6 * s.dn[207][1]);
        let eq56_e968_d_n2: f64 = (p.p6 * s.dn[207][2]);
        let eq56_e968_d_n3: f64 = (p.p6 * s.dn[207][3]);
        let eq56_e968_d_n4: f64 = (p.p6 * s.dn[207][4]);
        let eq56_e968_d_n5: f64 = (p.p6 * s.dn[207][5]);
        let eq56_e968_d_n6: f64 = (p.p6 * s.dn[207][6]);
        let eq56_e968_d_n7: f64 = (p.p6 * eq56_e967_d_n7);
        let eq56_e968_d_n8: f64 = (p.p6 * s.dn[207][8]);
        let eq56_e968_d_n9: f64 = (p.p6 * eq56_e967_d_n9);
        let eq56_e968_d_n10: f64 = (p.p6 * s.dn[207][10]);
        let eq56_e968_d_n11: f64 = (p.p6 * s.dn[207][11]);
        let eq56_e968_d_n12: f64 = (p.p6 * s.dn[207][12]);
        let eq56_e968_d_n13: f64 = (p.p6 * s.dn[207][13]);
        let eq56_e968_d_n14: f64 = (p.p6 * s.dn[207][14]);
        let eq56_e968_d_n15: f64 = (p.p6 * s.dn[207][15]);
        let eq56_e968_d_n16: f64 = (p.p6 * s.dn[207][16]);
        let eq56_e968_d_n17: f64 = (p.p6 * s.dn[207][17]);
        let eq56_e968_d_n18: f64 = (p.p6 * s.dn[207][18]);
        let eq56_e968_d_n19: f64 = (p.p6 * s.dn[207][19]);
        let eq56_e968_d_n20: f64 = (p.p6 * s.dn[207][20]);
        let eq56_e968_d_n21: f64 = (p.p6 * s.dn[207][21]);
        let eq56_e968_d_n22: f64 = (p.p6 * s.dn[207][22]);
        (eq56_e968, eq56_e968_d_n0, eq56_e968_d_n1, eq56_e968_d_n2, eq56_e968_d_n3, eq56_e968_d_n4, eq56_e968_d_n5, eq56_e968_d_n6, eq56_e968_d_n7, eq56_e968_d_n8, eq56_e968_d_n9, eq56_e968_d_n10, eq56_e968_d_n11, eq56_e968_d_n12, eq56_e968_d_n13, eq56_e968_d_n14, eq56_e968_d_n15, eq56_e968_d_n16, eq56_e968_d_n17, eq56_e968_d_n18, eq56_e968_d_n19, eq56_e968_d_n20, eq56_e968_d_n21, eq56_e968_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e970;
        let eq56_node_derivatives: [f64; 23] = [eq56_e970_d_n0, eq56_e970_d_n1, eq56_e970_d_n2, eq56_e970_d_n3, eq56_e970_d_n4, eq56_e970_d_n5, eq56_e970_d_n6, eq56_e970_d_n7, eq56_e970_d_n8, eq56_e970_d_n9, eq56_e970_d_n10, eq56_e970_d_n11, eq56_e970_d_n12, eq56_e970_d_n13, eq56_e970_d_n14, eq56_e970_d_n15, eq56_e970_d_n16, eq56_e970_d_n17, eq56_e970_d_n18, eq56_e970_d_n19, eq56_e970_d_n20, eq56_e970_d_n21, eq56_e970_d_n22];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq57_e980, eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n10, eq57_e980_d_n11, eq57_e980_d_n12, eq57_e980_d_n13, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22,) = {
    if ((s.v[424] != 0.0) && (s.v[427] != 0.0)) {
        let eq57_e976: f64 = (p.p6 * s.v[142]);
        let eq57_e976_d_n0: f64 = (p.p6 * s.dn[142][0]);
        let eq57_e976_d_n1: f64 = (p.p6 * s.dn[142][1]);
        let eq57_e976_d_n2: f64 = (p.p6 * s.dn[142][2]);
        let eq57_e976_d_n3: f64 = (p.p6 * s.dn[142][3]);
        let eq57_e976_d_n4: f64 = (p.p6 * s.dn[142][4]);
        let eq57_e976_d_n5: f64 = (p.p6 * s.dn[142][5]);
        let eq57_e976_d_n6: f64 = (p.p6 * s.dn[142][6]);
        let eq57_e976_d_n7: f64 = (p.p6 * s.dn[142][7]);
        let eq57_e976_d_n8: f64 = (p.p6 * s.dn[142][8]);
        let eq57_e976_d_n9: f64 = (p.p6 * s.dn[142][9]);
        let eq57_e976_d_n10: f64 = (p.p6 * s.dn[142][10]);
        let eq57_e976_d_n11: f64 = (p.p6 * s.dn[142][11]);
        let eq57_e976_d_n12: f64 = (p.p6 * s.dn[142][12]);
        let eq57_e976_d_n13: f64 = (p.p6 * s.dn[142][13]);
        let eq57_e976_d_n14: f64 = (p.p6 * s.dn[142][14]);
        let eq57_e976_d_n15: f64 = (p.p6 * s.dn[142][15]);
        let eq57_e976_d_n16: f64 = (p.p6 * s.dn[142][16]);
        let eq57_e976_d_n17: f64 = (p.p6 * s.dn[142][17]);
        let eq57_e976_d_n18: f64 = (p.p6 * s.dn[142][18]);
        let eq57_e976_d_n19: f64 = (p.p6 * s.dn[142][19]);
        let eq57_e976_d_n20: f64 = (p.p6 * s.dn[142][20]);
        let eq57_e976_d_n21: f64 = (p.p6 * s.dn[142][21]);
        let eq57_e976_d_n22: f64 = (p.p6 * s.dn[142][22]);
        let eq57_e978: f64 = (eq57_e976 * (nv0 - nv18));
        let eq57_e978_d_n0: f64 = ((eq57_e976_d_n0 * (nv0 - nv18)) + eq57_e976);
        let eq57_e978_d_n1: f64 = (eq57_e976_d_n1 * (nv0 - nv18));
        let eq57_e978_d_n2: f64 = (eq57_e976_d_n2 * (nv0 - nv18));
        let eq57_e978_d_n3: f64 = (eq57_e976_d_n3 * (nv0 - nv18));
        let eq57_e978_d_n4: f64 = (eq57_e976_d_n4 * (nv0 - nv18));
        let eq57_e978_d_n5: f64 = (eq57_e976_d_n5 * (nv0 - nv18));
        let eq57_e978_d_n6: f64 = (eq57_e976_d_n6 * (nv0 - nv18));
        let eq57_e978_d_n7: f64 = (eq57_e976_d_n7 * (nv0 - nv18));
        let eq57_e978_d_n8: f64 = (eq57_e976_d_n8 * (nv0 - nv18));
        let eq57_e978_d_n9: f64 = (eq57_e976_d_n9 * (nv0 - nv18));
        let eq57_e978_d_n10: f64 = (eq57_e976_d_n10 * (nv0 - nv18));
        let eq57_e978_d_n11: f64 = (eq57_e976_d_n11 * (nv0 - nv18));
        let eq57_e978_d_n12: f64 = (eq57_e976_d_n12 * (nv0 - nv18));
        let eq57_e978_d_n13: f64 = (eq57_e976_d_n13 * (nv0 - nv18));
        let eq57_e978_d_n14: f64 = (eq57_e976_d_n14 * (nv0 - nv18));
        let eq57_e978_d_n15: f64 = (eq57_e976_d_n15 * (nv0 - nv18));
        let eq57_e978_d_n16: f64 = (eq57_e976_d_n16 * (nv0 - nv18));
        let eq57_e978_d_n17: f64 = (eq57_e976_d_n17 * (nv0 - nv18));
        let eq57_e978_d_n18: f64 = ((eq57_e976_d_n18 * (nv0 - nv18)) + (-eq57_e976));
        let eq57_e978_d_n19: f64 = (eq57_e976_d_n19 * (nv0 - nv18));
        let eq57_e978_d_n20: f64 = (eq57_e976_d_n20 * (nv0 - nv18));
        let eq57_e978_d_n21: f64 = (eq57_e976_d_n21 * (nv0 - nv18));
        let eq57_e978_d_n22: f64 = (eq57_e976_d_n22 * (nv0 - nv18));
        (eq57_e978, eq57_e978_d_n0, eq57_e978_d_n1, eq57_e978_d_n2, eq57_e978_d_n3, eq57_e978_d_n4, eq57_e978_d_n5, eq57_e978_d_n6, eq57_e978_d_n7, eq57_e978_d_n8, eq57_e978_d_n9, eq57_e978_d_n10, eq57_e978_d_n11, eq57_e978_d_n12, eq57_e978_d_n13, eq57_e978_d_n14, eq57_e978_d_n15, eq57_e978_d_n16, eq57_e978_d_n17, eq57_e978_d_n18, eq57_e978_d_n19, eq57_e978_d_n20, eq57_e978_d_n21, eq57_e978_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e980;
        let eq57_node_derivatives: [f64; 23] = [eq57_e980_d_n0, eq57_e980_d_n1, eq57_e980_d_n2, eq57_e980_d_n3, eq57_e980_d_n4, eq57_e980_d_n5, eq57_e980_d_n6, eq57_e980_d_n7, eq57_e980_d_n8, eq57_e980_d_n9, eq57_e980_d_n10, eq57_e980_d_n11, eq57_e980_d_n12, eq57_e980_d_n13, eq57_e980_d_n14, eq57_e980_d_n15, eq57_e980_d_n16, eq57_e980_d_n17, eq57_e980_d_n18, eq57_e980_d_n19, eq57_e980_d_n20, eq57_e980_d_n21, eq57_e980_d_n22];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[18]),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv22 = ctx.node_voltage(nodes[22]);
        let (eq58_e990, eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22,) = {
    if ((s.v[424] != 0.0) && (s.v[427] != 0.0)) {
        let eq58_e986: f64 = (p.p6 * s.v[143]);
        let eq58_e986_d_n0: f64 = (p.p6 * s.dn[143][0]);
        let eq58_e986_d_n1: f64 = (p.p6 * s.dn[143][1]);
        let eq58_e986_d_n2: f64 = (p.p6 * s.dn[143][2]);
        let eq58_e986_d_n3: f64 = (p.p6 * s.dn[143][3]);
        let eq58_e986_d_n4: f64 = (p.p6 * s.dn[143][4]);
        let eq58_e986_d_n5: f64 = (p.p6 * s.dn[143][5]);
        let eq58_e986_d_n6: f64 = (p.p6 * s.dn[143][6]);
        let eq58_e986_d_n7: f64 = (p.p6 * s.dn[143][7]);
        let eq58_e986_d_n8: f64 = (p.p6 * s.dn[143][8]);
        let eq58_e986_d_n9: f64 = (p.p6 * s.dn[143][9]);
        let eq58_e986_d_n10: f64 = (p.p6 * s.dn[143][10]);
        let eq58_e986_d_n11: f64 = (p.p6 * s.dn[143][11]);
        let eq58_e986_d_n12: f64 = (p.p6 * s.dn[143][12]);
        let eq58_e986_d_n13: f64 = (p.p6 * s.dn[143][13]);
        let eq58_e986_d_n14: f64 = (p.p6 * s.dn[143][14]);
        let eq58_e986_d_n15: f64 = (p.p6 * s.dn[143][15]);
        let eq58_e986_d_n16: f64 = (p.p6 * s.dn[143][16]);
        let eq58_e986_d_n17: f64 = (p.p6 * s.dn[143][17]);
        let eq58_e986_d_n18: f64 = (p.p6 * s.dn[143][18]);
        let eq58_e986_d_n19: f64 = (p.p6 * s.dn[143][19]);
        let eq58_e986_d_n20: f64 = (p.p6 * s.dn[143][20]);
        let eq58_e986_d_n21: f64 = (p.p6 * s.dn[143][21]);
        let eq58_e986_d_n22: f64 = (p.p6 * s.dn[143][22]);
        let eq58_e988: f64 = (eq58_e986 * (nv22 - nv2));
        let eq58_e988_d_n0: f64 = (eq58_e986_d_n0 * (nv22 - nv2));
        let eq58_e988_d_n1: f64 = (eq58_e986_d_n1 * (nv22 - nv2));
        let eq58_e988_d_n2: f64 = ((eq58_e986_d_n2 * (nv22 - nv2)) + (-eq58_e986));
        let eq58_e988_d_n3: f64 = (eq58_e986_d_n3 * (nv22 - nv2));
        let eq58_e988_d_n4: f64 = (eq58_e986_d_n4 * (nv22 - nv2));
        let eq58_e988_d_n5: f64 = (eq58_e986_d_n5 * (nv22 - nv2));
        let eq58_e988_d_n6: f64 = (eq58_e986_d_n6 * (nv22 - nv2));
        let eq58_e988_d_n7: f64 = (eq58_e986_d_n7 * (nv22 - nv2));
        let eq58_e988_d_n8: f64 = (eq58_e986_d_n8 * (nv22 - nv2));
        let eq58_e988_d_n9: f64 = (eq58_e986_d_n9 * (nv22 - nv2));
        let eq58_e988_d_n10: f64 = (eq58_e986_d_n10 * (nv22 - nv2));
        let eq58_e988_d_n11: f64 = (eq58_e986_d_n11 * (nv22 - nv2));
        let eq58_e988_d_n12: f64 = (eq58_e986_d_n12 * (nv22 - nv2));
        let eq58_e988_d_n13: f64 = (eq58_e986_d_n13 * (nv22 - nv2));
        let eq58_e988_d_n14: f64 = (eq58_e986_d_n14 * (nv22 - nv2));
        let eq58_e988_d_n15: f64 = (eq58_e986_d_n15 * (nv22 - nv2));
        let eq58_e988_d_n16: f64 = (eq58_e986_d_n16 * (nv22 - nv2));
        let eq58_e988_d_n17: f64 = (eq58_e986_d_n17 * (nv22 - nv2));
        let eq58_e988_d_n18: f64 = (eq58_e986_d_n18 * (nv22 - nv2));
        let eq58_e988_d_n19: f64 = (eq58_e986_d_n19 * (nv22 - nv2));
        let eq58_e988_d_n20: f64 = (eq58_e986_d_n20 * (nv22 - nv2));
        let eq58_e988_d_n21: f64 = (eq58_e986_d_n21 * (nv22 - nv2));
        let eq58_e988_d_n22: f64 = ((eq58_e986_d_n22 * (nv22 - nv2)) + eq58_e986);
        (eq58_e988, eq58_e988_d_n0, eq58_e988_d_n1, eq58_e988_d_n2, eq58_e988_d_n3, eq58_e988_d_n4, eq58_e988_d_n5, eq58_e988_d_n6, eq58_e988_d_n7, eq58_e988_d_n8, eq58_e988_d_n9, eq58_e988_d_n10, eq58_e988_d_n11, eq58_e988_d_n12, eq58_e988_d_n13, eq58_e988_d_n14, eq58_e988_d_n15, eq58_e988_d_n16, eq58_e988_d_n17, eq58_e988_d_n18, eq58_e988_d_n19, eq58_e988_d_n20, eq58_e988_d_n21, eq58_e988_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e990;
        let eq58_node_derivatives: [f64; 23] = [eq58_e990_d_n0, eq58_e990_d_n1, eq58_e990_d_n2, eq58_e990_d_n3, eq58_e990_d_n4, eq58_e990_d_n5, eq58_e990_d_n6, eq58_e990_d_n7, eq58_e990_d_n8, eq58_e990_d_n9, eq58_e990_d_n10, eq58_e990_d_n11, eq58_e990_d_n12, eq58_e990_d_n13, eq58_e990_d_n14, eq58_e990_d_n15, eq58_e990_d_n16, eq58_e990_d_n17, eq58_e990_d_n18, eq58_e990_d_n19, eq58_e990_d_n20, eq58_e990_d_n21, eq58_e990_d_n22];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[22]),
            Some(nodes[2]),
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq59_e1001, eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n10, eq59_e1001_d_n11, eq59_e1001_d_n12, eq59_e1001_d_n13, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22,) = {
    if ((s.v[424] != 0.0) && (!(s.v[427] != 0.0))) {
        let eq59_e997: f64 = (p.p6 * s.v[142]);
        let eq59_e997_d_n0: f64 = (p.p6 * s.dn[142][0]);
        let eq59_e997_d_n1: f64 = (p.p6 * s.dn[142][1]);
        let eq59_e997_d_n2: f64 = (p.p6 * s.dn[142][2]);
        let eq59_e997_d_n3: f64 = (p.p6 * s.dn[142][3]);
        let eq59_e997_d_n4: f64 = (p.p6 * s.dn[142][4]);
        let eq59_e997_d_n5: f64 = (p.p6 * s.dn[142][5]);
        let eq59_e997_d_n6: f64 = (p.p6 * s.dn[142][6]);
        let eq59_e997_d_n7: f64 = (p.p6 * s.dn[142][7]);
        let eq59_e997_d_n8: f64 = (p.p6 * s.dn[142][8]);
        let eq59_e997_d_n9: f64 = (p.p6 * s.dn[142][9]);
        let eq59_e997_d_n10: f64 = (p.p6 * s.dn[142][10]);
        let eq59_e997_d_n11: f64 = (p.p6 * s.dn[142][11]);
        let eq59_e997_d_n12: f64 = (p.p6 * s.dn[142][12]);
        let eq59_e997_d_n13: f64 = (p.p6 * s.dn[142][13]);
        let eq59_e997_d_n14: f64 = (p.p6 * s.dn[142][14]);
        let eq59_e997_d_n15: f64 = (p.p6 * s.dn[142][15]);
        let eq59_e997_d_n16: f64 = (p.p6 * s.dn[142][16]);
        let eq59_e997_d_n17: f64 = (p.p6 * s.dn[142][17]);
        let eq59_e997_d_n18: f64 = (p.p6 * s.dn[142][18]);
        let eq59_e997_d_n19: f64 = (p.p6 * s.dn[142][19]);
        let eq59_e997_d_n20: f64 = (p.p6 * s.dn[142][20]);
        let eq59_e997_d_n21: f64 = (p.p6 * s.dn[142][21]);
        let eq59_e997_d_n22: f64 = (p.p6 * s.dn[142][22]);
        let eq59_e999: f64 = (eq59_e997 * (nv0 - nv7));
        let eq59_e999_d_n0: f64 = ((eq59_e997_d_n0 * (nv0 - nv7)) + eq59_e997);
        let eq59_e999_d_n1: f64 = (eq59_e997_d_n1 * (nv0 - nv7));
        let eq59_e999_d_n2: f64 = (eq59_e997_d_n2 * (nv0 - nv7));
        let eq59_e999_d_n3: f64 = (eq59_e997_d_n3 * (nv0 - nv7));
        let eq59_e999_d_n4: f64 = (eq59_e997_d_n4 * (nv0 - nv7));
        let eq59_e999_d_n5: f64 = (eq59_e997_d_n5 * (nv0 - nv7));
        let eq59_e999_d_n6: f64 = (eq59_e997_d_n6 * (nv0 - nv7));
        let eq59_e999_d_n7: f64 = ((eq59_e997_d_n7 * (nv0 - nv7)) + (-eq59_e997));
        let eq59_e999_d_n8: f64 = (eq59_e997_d_n8 * (nv0 - nv7));
        let eq59_e999_d_n9: f64 = (eq59_e997_d_n9 * (nv0 - nv7));
        let eq59_e999_d_n10: f64 = (eq59_e997_d_n10 * (nv0 - nv7));
        let eq59_e999_d_n11: f64 = (eq59_e997_d_n11 * (nv0 - nv7));
        let eq59_e999_d_n12: f64 = (eq59_e997_d_n12 * (nv0 - nv7));
        let eq59_e999_d_n13: f64 = (eq59_e997_d_n13 * (nv0 - nv7));
        let eq59_e999_d_n14: f64 = (eq59_e997_d_n14 * (nv0 - nv7));
        let eq59_e999_d_n15: f64 = (eq59_e997_d_n15 * (nv0 - nv7));
        let eq59_e999_d_n16: f64 = (eq59_e997_d_n16 * (nv0 - nv7));
        let eq59_e999_d_n17: f64 = (eq59_e997_d_n17 * (nv0 - nv7));
        let eq59_e999_d_n18: f64 = (eq59_e997_d_n18 * (nv0 - nv7));
        let eq59_e999_d_n19: f64 = (eq59_e997_d_n19 * (nv0 - nv7));
        let eq59_e999_d_n20: f64 = (eq59_e997_d_n20 * (nv0 - nv7));
        let eq59_e999_d_n21: f64 = (eq59_e997_d_n21 * (nv0 - nv7));
        let eq59_e999_d_n22: f64 = (eq59_e997_d_n22 * (nv0 - nv7));
        (eq59_e999, eq59_e999_d_n0, eq59_e999_d_n1, eq59_e999_d_n2, eq59_e999_d_n3, eq59_e999_d_n4, eq59_e999_d_n5, eq59_e999_d_n6, eq59_e999_d_n7, eq59_e999_d_n8, eq59_e999_d_n9, eq59_e999_d_n10, eq59_e999_d_n11, eq59_e999_d_n12, eq59_e999_d_n13, eq59_e999_d_n14, eq59_e999_d_n15, eq59_e999_d_n16, eq59_e999_d_n17, eq59_e999_d_n18, eq59_e999_d_n19, eq59_e999_d_n20, eq59_e999_d_n21, eq59_e999_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e1001;
        let eq59_node_derivatives: [f64; 23] = [eq59_e1001_d_n0, eq59_e1001_d_n1, eq59_e1001_d_n2, eq59_e1001_d_n3, eq59_e1001_d_n4, eq59_e1001_d_n5, eq59_e1001_d_n6, eq59_e1001_d_n7, eq59_e1001_d_n8, eq59_e1001_d_n9, eq59_e1001_d_n10, eq59_e1001_d_n11, eq59_e1001_d_n12, eq59_e1001_d_n13, eq59_e1001_d_n14, eq59_e1001_d_n15, eq59_e1001_d_n16, eq59_e1001_d_n17, eq59_e1001_d_n18, eq59_e1001_d_n19, eq59_e1001_d_n20, eq59_e1001_d_n21, eq59_e1001_d_n22];
        let eq59_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq60_e1012, eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22,) = {
    if ((s.v[424] != 0.0) && (!(s.v[427] != 0.0))) {
        let eq60_e1008: f64 = (p.p6 * s.v[143]);
        let eq60_e1008_d_n0: f64 = (p.p6 * s.dn[143][0]);
        let eq60_e1008_d_n1: f64 = (p.p6 * s.dn[143][1]);
        let eq60_e1008_d_n2: f64 = (p.p6 * s.dn[143][2]);
        let eq60_e1008_d_n3: f64 = (p.p6 * s.dn[143][3]);
        let eq60_e1008_d_n4: f64 = (p.p6 * s.dn[143][4]);
        let eq60_e1008_d_n5: f64 = (p.p6 * s.dn[143][5]);
        let eq60_e1008_d_n6: f64 = (p.p6 * s.dn[143][6]);
        let eq60_e1008_d_n7: f64 = (p.p6 * s.dn[143][7]);
        let eq60_e1008_d_n8: f64 = (p.p6 * s.dn[143][8]);
        let eq60_e1008_d_n9: f64 = (p.p6 * s.dn[143][9]);
        let eq60_e1008_d_n10: f64 = (p.p6 * s.dn[143][10]);
        let eq60_e1008_d_n11: f64 = (p.p6 * s.dn[143][11]);
        let eq60_e1008_d_n12: f64 = (p.p6 * s.dn[143][12]);
        let eq60_e1008_d_n13: f64 = (p.p6 * s.dn[143][13]);
        let eq60_e1008_d_n14: f64 = (p.p6 * s.dn[143][14]);
        let eq60_e1008_d_n15: f64 = (p.p6 * s.dn[143][15]);
        let eq60_e1008_d_n16: f64 = (p.p6 * s.dn[143][16]);
        let eq60_e1008_d_n17: f64 = (p.p6 * s.dn[143][17]);
        let eq60_e1008_d_n18: f64 = (p.p6 * s.dn[143][18]);
        let eq60_e1008_d_n19: f64 = (p.p6 * s.dn[143][19]);
        let eq60_e1008_d_n20: f64 = (p.p6 * s.dn[143][20]);
        let eq60_e1008_d_n21: f64 = (p.p6 * s.dn[143][21]);
        let eq60_e1008_d_n22: f64 = (p.p6 * s.dn[143][22]);
        let eq60_e1010: f64 = (eq60_e1008 * (nv8 - nv2));
        let eq60_e1010_d_n0: f64 = (eq60_e1008_d_n0 * (nv8 - nv2));
        let eq60_e1010_d_n1: f64 = (eq60_e1008_d_n1 * (nv8 - nv2));
        let eq60_e1010_d_n2: f64 = ((eq60_e1008_d_n2 * (nv8 - nv2)) + (-eq60_e1008));
        let eq60_e1010_d_n3: f64 = (eq60_e1008_d_n3 * (nv8 - nv2));
        let eq60_e1010_d_n4: f64 = (eq60_e1008_d_n4 * (nv8 - nv2));
        let eq60_e1010_d_n5: f64 = (eq60_e1008_d_n5 * (nv8 - nv2));
        let eq60_e1010_d_n6: f64 = (eq60_e1008_d_n6 * (nv8 - nv2));
        let eq60_e1010_d_n7: f64 = (eq60_e1008_d_n7 * (nv8 - nv2));
        let eq60_e1010_d_n8: f64 = ((eq60_e1008_d_n8 * (nv8 - nv2)) + eq60_e1008);
        let eq60_e1010_d_n9: f64 = (eq60_e1008_d_n9 * (nv8 - nv2));
        let eq60_e1010_d_n10: f64 = (eq60_e1008_d_n10 * (nv8 - nv2));
        let eq60_e1010_d_n11: f64 = (eq60_e1008_d_n11 * (nv8 - nv2));
        let eq60_e1010_d_n12: f64 = (eq60_e1008_d_n12 * (nv8 - nv2));
        let eq60_e1010_d_n13: f64 = (eq60_e1008_d_n13 * (nv8 - nv2));
        let eq60_e1010_d_n14: f64 = (eq60_e1008_d_n14 * (nv8 - nv2));
        let eq60_e1010_d_n15: f64 = (eq60_e1008_d_n15 * (nv8 - nv2));
        let eq60_e1010_d_n16: f64 = (eq60_e1008_d_n16 * (nv8 - nv2));
        let eq60_e1010_d_n17: f64 = (eq60_e1008_d_n17 * (nv8 - nv2));
        let eq60_e1010_d_n18: f64 = (eq60_e1008_d_n18 * (nv8 - nv2));
        let eq60_e1010_d_n19: f64 = (eq60_e1008_d_n19 * (nv8 - nv2));
        let eq60_e1010_d_n20: f64 = (eq60_e1008_d_n20 * (nv8 - nv2));
        let eq60_e1010_d_n21: f64 = (eq60_e1008_d_n21 * (nv8 - nv2));
        let eq60_e1010_d_n22: f64 = (eq60_e1008_d_n22 * (nv8 - nv2));
        (eq60_e1010, eq60_e1010_d_n0, eq60_e1010_d_n1, eq60_e1010_d_n2, eq60_e1010_d_n3, eq60_e1010_d_n4, eq60_e1010_d_n5, eq60_e1010_d_n6, eq60_e1010_d_n7, eq60_e1010_d_n8, eq60_e1010_d_n9, eq60_e1010_d_n10, eq60_e1010_d_n11, eq60_e1010_d_n12, eq60_e1010_d_n13, eq60_e1010_d_n14, eq60_e1010_d_n15, eq60_e1010_d_n16, eq60_e1010_d_n17, eq60_e1010_d_n18, eq60_e1010_d_n19, eq60_e1010_d_n20, eq60_e1010_d_n21, eq60_e1010_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1012;
        let eq60_node_derivatives: [f64; 23] = [eq60_e1012_d_n0, eq60_e1012_d_n1, eq60_e1012_d_n2, eq60_e1012_d_n3, eq60_e1012_d_n4, eq60_e1012_d_n5, eq60_e1012_d_n6, eq60_e1012_d_n7, eq60_e1012_d_n8, eq60_e1012_d_n9, eq60_e1012_d_n10, eq60_e1012_d_n11, eq60_e1012_d_n12, eq60_e1012_d_n13, eq60_e1012_d_n14, eq60_e1012_d_n15, eq60_e1012_d_n16, eq60_e1012_d_n17, eq60_e1012_d_n18, eq60_e1012_d_n19, eq60_e1012_d_n20, eq60_e1012_d_n21, eq60_e1012_d_n22];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
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
        let (eq61_e1019,) = {
    if ((!(s.v[424] != 0.0)) && (s.v[428] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e1019;
        stamper.stamp_potential(
            branches[29],
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
        let (eq62_e1026,) = {
    if ((!(s.v[424] != 0.0)) && (s.v[428] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq62_value: f64 = eq62_e1026;
        stamper.stamp_potential(
            branches[30],
            eq62_value,
            &[
            ],
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
        let (eq63_e1034,) = {
    if ((!(s.v[424] != 0.0)) && (!(s.v[428] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq63_value: f64 = eq63_e1034;
        stamper.stamp_potential(
            branches[31],
            eq63_value,
            &[
            ],
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
        let (eq64_e1042,) = {
    if ((!(s.v[424] != 0.0)) && (!(s.v[428] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e1042;
        stamper.stamp_potential(
            branches[32],
            eq64_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_65_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq65_e1050,) = {
    if (s.v[429] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e1050;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[8]),
            self.multiplicity * (eq65_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_66_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq66_e1068,) = {
    if (((s.v[429] != 0.0) && (s.v[430] != 0.0)) && (s.v[431] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e1068;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[18]),
            self.multiplicity * (eq66_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq67_e1086,) = {
    if (((s.v[429] != 0.0) && (s.v[430] != 0.0)) && (s.v[431] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e1086;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[22]),
            self.multiplicity * (eq67_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq68_e1105,) = {
    if (((s.v[429] != 0.0) && (s.v[430] != 0.0)) && (!(s.v[431] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq68_value: f64 = eq68_e1105;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq68_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq69_e1124,) = {
    if (((s.v[429] != 0.0) && (s.v[430] != 0.0)) && (!(s.v[431] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq69_value: f64 = eq69_e1124;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq69_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_70_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq70_e1137,) = {
    if (s.v[432] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e1137;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq70_value),
            &[
            ],
        );
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16, eq40_e2302_q, eq40_e2302_q_d_n0, eq40_e2302_q_d_n1, eq40_e2302_q_d_n2, eq40_e2302_q_d_n3, eq40_e2302_q_d_n4, eq40_e2302_q_d_n5, eq40_e2302_q_d_n6, eq40_e2302_q_d_n7, eq40_e2302_q_d_n8, eq40_e2302_q_d_n9, eq40_e2302_q_d_n10, eq40_e2302_q_d_n11, eq40_e2302_q_d_n12, eq40_e2302_q_d_n13, eq40_e2302_q_d_n14, eq40_e2302_q_d_n15, eq40_e2302_q_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1706] != 0.0)) {
        let eq40_e2300_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], eq40_e2300_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 17] = [eq40_e2302_q_d_n0, eq40_e2302_q_d_n1, eq40_e2302_q_d_n2, eq40_e2302_q_d_n3, eq40_e2302_q_d_n4, eq40_e2302_q_d_n5, eq40_e2302_q_d_n6, eq40_e2302_q_d_n7, eq40_e2302_q_d_n8, eq40_e2302_q_d_n9, eq40_e2302_q_d_n10, eq40_e2302_q_d_n11, eq40_e2302_q_d_n12, eq40_e2302_q_d_n13, eq40_e2302_q_d_n14, eq40_e2302_q_d_n15, eq40_e2302_q_d_n16];
        let eq40_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            &nodes,
            &eq40_reactive_node_derivatives,
            &branches,
            &eq40_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16, eq41_e2311_q, eq41_e2311_q_d_n0, eq41_e2311_q_d_n1, eq41_e2311_q_d_n2, eq41_e2311_q_d_n3, eq41_e2311_q_d_n4, eq41_e2311_q_d_n5, eq41_e2311_q_d_n6, eq41_e2311_q_d_n7, eq41_e2311_q_d_n8, eq41_e2311_q_d_n9, eq41_e2311_q_d_n10, eq41_e2311_q_d_n11, eq41_e2311_q_d_n12, eq41_e2311_q_d_n13, eq41_e2311_q_d_n14, eq41_e2311_q_d_n15, eq41_e2311_q_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1706] != 0.0)) {
        let eq41_e2308_q: f64 = s.v[503];
        let eq41_e2309: f64 = (s.v[114] * s.v[503]);
        let eq41_e2309_d_n0: f64 = ((s.dn[114][0] * s.v[503]) + (s.v[114] * s.dn[503][0]));
        let eq41_e2309_d_n1: f64 = ((s.dn[114][1] * s.v[503]) + (s.v[114] * s.dn[503][1]));
        let eq41_e2309_d_n2: f64 = ((s.dn[114][2] * s.v[503]) + (s.v[114] * s.dn[503][2]));
        let eq41_e2309_d_n3: f64 = ((s.dn[114][3] * s.v[503]) + (s.v[114] * s.dn[503][3]));
        let eq41_e2309_d_n4: f64 = ((s.dn[114][4] * s.v[503]) + (s.v[114] * s.dn[503][4]));
        let eq41_e2309_d_n5: f64 = ((s.dn[114][5] * s.v[503]) + (s.v[114] * s.dn[503][5]));
        let eq41_e2309_d_n6: f64 = ((s.dn[114][6] * s.v[503]) + (s.v[114] * s.dn[503][6]));
        let eq41_e2309_d_n7: f64 = ((s.dn[114][7] * s.v[503]) + (s.v[114] * s.dn[503][7]));
        let eq41_e2309_d_n8: f64 = ((s.dn[114][8] * s.v[503]) + (s.v[114] * s.dn[503][8]));
        let eq41_e2309_d_n9: f64 = ((s.dn[114][9] * s.v[503]) + (s.v[114] * s.dn[503][9]));
        let eq41_e2309_d_n10: f64 = ((s.dn[114][10] * s.v[503]) + (s.v[114] * s.dn[503][10]));
        let eq41_e2309_d_n11: f64 = ((s.dn[114][11] * s.v[503]) + (s.v[114] * s.dn[503][11]));
        let eq41_e2309_d_n12: f64 = ((s.dn[114][12] * s.v[503]) + (s.v[114] * s.dn[503][12]));
        let eq41_e2309_d_n13: f64 = ((s.dn[114][13] * s.v[503]) + (s.v[114] * s.dn[503][13]));
        let eq41_e2309_d_n14: f64 = ((s.dn[114][14] * s.v[503]) + (s.v[114] * s.dn[503][14]));
        let eq41_e2309_d_n15: f64 = ((s.dn[114][15] * s.v[503]) + (s.v[114] * s.dn[503][15]));
        let eq41_e2309_d_n16: f64 = ((s.dn[114][16] * s.v[503]) + (s.v[114] * s.dn[503][16]));
        let eq41_e2309_q: f64 = (s.v[114] * eq41_e2308_q);
        let eq41_e2309_q_d_n0: f64 = ((s.dn[114][0] * eq41_e2308_q) + (s.v[114] * s.dn[503][0]));
        let eq41_e2309_q_d_n1: f64 = ((s.dn[114][1] * eq41_e2308_q) + (s.v[114] * s.dn[503][1]));
        let eq41_e2309_q_d_n2: f64 = ((s.dn[114][2] * eq41_e2308_q) + (s.v[114] * s.dn[503][2]));
        let eq41_e2309_q_d_n3: f64 = ((s.dn[114][3] * eq41_e2308_q) + (s.v[114] * s.dn[503][3]));
        let eq41_e2309_q_d_n4: f64 = ((s.dn[114][4] * eq41_e2308_q) + (s.v[114] * s.dn[503][4]));
        let eq41_e2309_q_d_n5: f64 = ((s.dn[114][5] * eq41_e2308_q) + (s.v[114] * s.dn[503][5]));
        let eq41_e2309_q_d_n6: f64 = ((s.dn[114][6] * eq41_e2308_q) + (s.v[114] * s.dn[503][6]));
        let eq41_e2309_q_d_n7: f64 = ((s.dn[114][7] * eq41_e2308_q) + (s.v[114] * s.dn[503][7]));
        let eq41_e2309_q_d_n8: f64 = ((s.dn[114][8] * eq41_e2308_q) + (s.v[114] * s.dn[503][8]));
        let eq41_e2309_q_d_n9: f64 = ((s.dn[114][9] * eq41_e2308_q) + (s.v[114] * s.dn[503][9]));
        let eq41_e2309_q_d_n10: f64 = ((s.dn[114][10] * eq41_e2308_q) + (s.v[114] * s.dn[503][10]));
        let eq41_e2309_q_d_n11: f64 = ((s.dn[114][11] * eq41_e2308_q) + (s.v[114] * s.dn[503][11]));
        let eq41_e2309_q_d_n12: f64 = ((s.dn[114][12] * eq41_e2308_q) + (s.v[114] * s.dn[503][12]));
        let eq41_e2309_q_d_n13: f64 = ((s.dn[114][13] * eq41_e2308_q) + (s.v[114] * s.dn[503][13]));
        let eq41_e2309_q_d_n14: f64 = ((s.dn[114][14] * eq41_e2308_q) + (s.v[114] * s.dn[503][14]));
        let eq41_e2309_q_d_n15: f64 = ((s.dn[114][15] * eq41_e2308_q) + (s.v[114] * s.dn[503][15]));
        let eq41_e2309_q_d_n16: f64 = ((s.dn[114][16] * eq41_e2308_q) + (s.v[114] * s.dn[503][16]));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n1, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n12, eq41_e2309_d_n13, eq41_e2309_d_n14, eq41_e2309_d_n15, eq41_e2309_d_n16, eq41_e2309_q, eq41_e2309_q_d_n0, eq41_e2309_q_d_n1, eq41_e2309_q_d_n2, eq41_e2309_q_d_n3, eq41_e2309_q_d_n4, eq41_e2309_q_d_n5, eq41_e2309_q_d_n6, eq41_e2309_q_d_n7, eq41_e2309_q_d_n8, eq41_e2309_q_d_n9, eq41_e2309_q_d_n10, eq41_e2309_q_d_n11, eq41_e2309_q_d_n12, eq41_e2309_q_d_n13, eq41_e2309_q_d_n14, eq41_e2309_q_d_n15, eq41_e2309_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 17] = [eq41_e2311_q_d_n0, eq41_e2311_q_d_n1, eq41_e2311_q_d_n2, eq41_e2311_q_d_n3, eq41_e2311_q_d_n4, eq41_e2311_q_d_n5, eq41_e2311_q_d_n6, eq41_e2311_q_d_n7, eq41_e2311_q_d_n8, eq41_e2311_q_d_n9, eq41_e2311_q_d_n10, eq41_e2311_q_d_n11, eq41_e2311_q_d_n12, eq41_e2311_q_d_n13, eq41_e2311_q_d_n14, eq41_e2311_q_d_n15, eq41_e2311_q_d_n16];
        let eq41_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            &nodes,
            &eq41_reactive_node_derivatives,
            &branches,
            &eq41_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16, eq42_e2320_q, eq42_e2320_q_d_n0, eq42_e2320_q_d_n1, eq42_e2320_q_d_n2, eq42_e2320_q_d_n3, eq42_e2320_q_d_n4, eq42_e2320_q_d_n5, eq42_e2320_q_d_n6, eq42_e2320_q_d_n7, eq42_e2320_q_d_n8, eq42_e2320_q_d_n9, eq42_e2320_q_d_n10, eq42_e2320_q_d_n11, eq42_e2320_q_d_n12, eq42_e2320_q_d_n13, eq42_e2320_q_d_n14, eq42_e2320_q_d_n15, eq42_e2320_q_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1706] != 0.0)) {
        let eq42_e2317_q: f64 = s.v[504];
        let eq42_e2318: f64 = (s.v[114] * s.v[504]);
        let eq42_e2318_d_n0: f64 = ((s.dn[114][0] * s.v[504]) + (s.v[114] * s.dn[504][0]));
        let eq42_e2318_d_n1: f64 = ((s.dn[114][1] * s.v[504]) + (s.v[114] * s.dn[504][1]));
        let eq42_e2318_d_n2: f64 = ((s.dn[114][2] * s.v[504]) + (s.v[114] * s.dn[504][2]));
        let eq42_e2318_d_n3: f64 = ((s.dn[114][3] * s.v[504]) + (s.v[114] * s.dn[504][3]));
        let eq42_e2318_d_n4: f64 = ((s.dn[114][4] * s.v[504]) + (s.v[114] * s.dn[504][4]));
        let eq42_e2318_d_n5: f64 = ((s.dn[114][5] * s.v[504]) + (s.v[114] * s.dn[504][5]));
        let eq42_e2318_d_n6: f64 = ((s.dn[114][6] * s.v[504]) + (s.v[114] * s.dn[504][6]));
        let eq42_e2318_d_n7: f64 = ((s.dn[114][7] * s.v[504]) + (s.v[114] * s.dn[504][7]));
        let eq42_e2318_d_n8: f64 = ((s.dn[114][8] * s.v[504]) + (s.v[114] * s.dn[504][8]));
        let eq42_e2318_d_n9: f64 = ((s.dn[114][9] * s.v[504]) + (s.v[114] * s.dn[504][9]));
        let eq42_e2318_d_n10: f64 = ((s.dn[114][10] * s.v[504]) + (s.v[114] * s.dn[504][10]));
        let eq42_e2318_d_n11: f64 = ((s.dn[114][11] * s.v[504]) + (s.v[114] * s.dn[504][11]));
        let eq42_e2318_d_n12: f64 = ((s.dn[114][12] * s.v[504]) + (s.v[114] * s.dn[504][12]));
        let eq42_e2318_d_n13: f64 = ((s.dn[114][13] * s.v[504]) + (s.v[114] * s.dn[504][13]));
        let eq42_e2318_d_n14: f64 = ((s.dn[114][14] * s.v[504]) + (s.v[114] * s.dn[504][14]));
        let eq42_e2318_d_n15: f64 = ((s.dn[114][15] * s.v[504]) + (s.v[114] * s.dn[504][15]));
        let eq42_e2318_d_n16: f64 = ((s.dn[114][16] * s.v[504]) + (s.v[114] * s.dn[504][16]));
        let eq42_e2318_q: f64 = (s.v[114] * eq42_e2317_q);
        let eq42_e2318_q_d_n0: f64 = ((s.dn[114][0] * eq42_e2317_q) + (s.v[114] * s.dn[504][0]));
        let eq42_e2318_q_d_n1: f64 = ((s.dn[114][1] * eq42_e2317_q) + (s.v[114] * s.dn[504][1]));
        let eq42_e2318_q_d_n2: f64 = ((s.dn[114][2] * eq42_e2317_q) + (s.v[114] * s.dn[504][2]));
        let eq42_e2318_q_d_n3: f64 = ((s.dn[114][3] * eq42_e2317_q) + (s.v[114] * s.dn[504][3]));
        let eq42_e2318_q_d_n4: f64 = ((s.dn[114][4] * eq42_e2317_q) + (s.v[114] * s.dn[504][4]));
        let eq42_e2318_q_d_n5: f64 = ((s.dn[114][5] * eq42_e2317_q) + (s.v[114] * s.dn[504][5]));
        let eq42_e2318_q_d_n6: f64 = ((s.dn[114][6] * eq42_e2317_q) + (s.v[114] * s.dn[504][6]));
        let eq42_e2318_q_d_n7: f64 = ((s.dn[114][7] * eq42_e2317_q) + (s.v[114] * s.dn[504][7]));
        let eq42_e2318_q_d_n8: f64 = ((s.dn[114][8] * eq42_e2317_q) + (s.v[114] * s.dn[504][8]));
        let eq42_e2318_q_d_n9: f64 = ((s.dn[114][9] * eq42_e2317_q) + (s.v[114] * s.dn[504][9]));
        let eq42_e2318_q_d_n10: f64 = ((s.dn[114][10] * eq42_e2317_q) + (s.v[114] * s.dn[504][10]));
        let eq42_e2318_q_d_n11: f64 = ((s.dn[114][11] * eq42_e2317_q) + (s.v[114] * s.dn[504][11]));
        let eq42_e2318_q_d_n12: f64 = ((s.dn[114][12] * eq42_e2317_q) + (s.v[114] * s.dn[504][12]));
        let eq42_e2318_q_d_n13: f64 = ((s.dn[114][13] * eq42_e2317_q) + (s.v[114] * s.dn[504][13]));
        let eq42_e2318_q_d_n14: f64 = ((s.dn[114][14] * eq42_e2317_q) + (s.v[114] * s.dn[504][14]));
        let eq42_e2318_q_d_n15: f64 = ((s.dn[114][15] * eq42_e2317_q) + (s.v[114] * s.dn[504][15]));
        let eq42_e2318_q_d_n16: f64 = ((s.dn[114][16] * eq42_e2317_q) + (s.v[114] * s.dn[504][16]));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n1, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n12, eq42_e2318_d_n13, eq42_e2318_d_n14, eq42_e2318_d_n15, eq42_e2318_d_n16, eq42_e2318_q, eq42_e2318_q_d_n0, eq42_e2318_q_d_n1, eq42_e2318_q_d_n2, eq42_e2318_q_d_n3, eq42_e2318_q_d_n4, eq42_e2318_q_d_n5, eq42_e2318_q_d_n6, eq42_e2318_q_d_n7, eq42_e2318_q_d_n8, eq42_e2318_q_d_n9, eq42_e2318_q_d_n10, eq42_e2318_q_d_n11, eq42_e2318_q_d_n12, eq42_e2318_q_d_n13, eq42_e2318_q_d_n14, eq42_e2318_q_d_n15, eq42_e2318_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_reactive_node_derivatives: [f64; 17] = [eq42_e2320_q_d_n0, eq42_e2320_q_d_n1, eq42_e2320_q_d_n2, eq42_e2320_q_d_n3, eq42_e2320_q_d_n4, eq42_e2320_q_d_n5, eq42_e2320_q_d_n6, eq42_e2320_q_d_n7, eq42_e2320_q_d_n8, eq42_e2320_q_d_n9, eq42_e2320_q_d_n10, eq42_e2320_q_d_n11, eq42_e2320_q_d_n12, eq42_e2320_q_d_n13, eq42_e2320_q_d_n14, eq42_e2320_q_d_n15, eq42_e2320_q_d_n16];
        let eq42_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &nodes,
            &eq42_reactive_node_derivatives,
            &branches,
            &eq42_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16, eq43_e2328_q, eq43_e2328_q_d_n0, eq43_e2328_q_d_n1, eq43_e2328_q_d_n2, eq43_e2328_q_d_n3, eq43_e2328_q_d_n4, eq43_e2328_q_d_n5, eq43_e2328_q_d_n6, eq43_e2328_q_d_n7, eq43_e2328_q_d_n8, eq43_e2328_q_d_n9, eq43_e2328_q_d_n10, eq43_e2328_q_d_n11, eq43_e2328_q_d_n12, eq43_e2328_q_d_n13, eq43_e2328_q_d_n14, eq43_e2328_q_d_n15, eq43_e2328_q_d_n16,) = {
    if ((s.v[1705] != 0.0) && (!(s.v[1706] != 0.0))) {
        let eq43_e2326_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], eq43_e2326_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_reactive_node_derivatives: [f64; 17] = [eq43_e2328_q_d_n0, eq43_e2328_q_d_n1, eq43_e2328_q_d_n2, eq43_e2328_q_d_n3, eq43_e2328_q_d_n4, eq43_e2328_q_d_n5, eq43_e2328_q_d_n6, eq43_e2328_q_d_n7, eq43_e2328_q_d_n8, eq43_e2328_q_d_n9, eq43_e2328_q_d_n10, eq43_e2328_q_d_n11, eq43_e2328_q_d_n12, eq43_e2328_q_d_n13, eq43_e2328_q_d_n14, eq43_e2328_q_d_n15, eq43_e2328_q_d_n16];
        let eq43_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            &nodes,
            &eq43_reactive_node_derivatives,
            &branches,
            &eq43_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16, eq44_e2333_q, eq44_e2333_q_d_n0, eq44_e2333_q_d_n1, eq44_e2333_q_d_n2, eq44_e2333_q_d_n3, eq44_e2333_q_d_n4, eq44_e2333_q_d_n5, eq44_e2333_q_d_n6, eq44_e2333_q_d_n7, eq44_e2333_q_d_n8, eq44_e2333_q_d_n9, eq44_e2333_q_d_n10, eq44_e2333_q_d_n11, eq44_e2333_q_d_n12, eq44_e2333_q_d_n13, eq44_e2333_q_d_n14, eq44_e2333_q_d_n15, eq44_e2333_q_d_n16,) = {
    if (s.v[1705] != 0.0) {
        let eq44_e2331_q: f64 = s.v[502];
        (s.v[502], s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], eq44_e2331_q, s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_reactive_node_derivatives: [f64; 17] = [eq44_e2333_q_d_n0, eq44_e2333_q_d_n1, eq44_e2333_q_d_n2, eq44_e2333_q_d_n3, eq44_e2333_q_d_n4, eq44_e2333_q_d_n5, eq44_e2333_q_d_n6, eq44_e2333_q_d_n7, eq44_e2333_q_d_n8, eq44_e2333_q_d_n9, eq44_e2333_q_d_n10, eq44_e2333_q_d_n11, eq44_e2333_q_d_n12, eq44_e2333_q_d_n13, eq44_e2333_q_d_n14, eq44_e2333_q_d_n15, eq44_e2333_q_d_n16];
        let eq44_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            &nodes,
            &eq44_reactive_node_derivatives,
            &branches,
            &eq44_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16, eq45_e2340_q, eq45_e2340_q_d_n0, eq45_e2340_q_d_n1, eq45_e2340_q_d_n2, eq45_e2340_q_d_n3, eq45_e2340_q_d_n4, eq45_e2340_q_d_n5, eq45_e2340_q_d_n6, eq45_e2340_q_d_n7, eq45_e2340_q_d_n8, eq45_e2340_q_d_n9, eq45_e2340_q_d_n10, eq45_e2340_q_d_n11, eq45_e2340_q_d_n12, eq45_e2340_q_d_n13, eq45_e2340_q_d_n14, eq45_e2340_q_d_n15, eq45_e2340_q_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1707] != 0.0)) {
        let eq45_e2338_q: f64 = s.v[500];
        (s.v[500], s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], eq45_e2338_q, s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_reactive_node_derivatives: [f64; 17] = [eq45_e2340_q_d_n0, eq45_e2340_q_d_n1, eq45_e2340_q_d_n2, eq45_e2340_q_d_n3, eq45_e2340_q_d_n4, eq45_e2340_q_d_n5, eq45_e2340_q_d_n6, eq45_e2340_q_d_n7, eq45_e2340_q_d_n8, eq45_e2340_q_d_n9, eq45_e2340_q_d_n10, eq45_e2340_q_d_n11, eq45_e2340_q_d_n12, eq45_e2340_q_d_n13, eq45_e2340_q_d_n14, eq45_e2340_q_d_n15, eq45_e2340_q_d_n16];
        let eq45_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            &nodes,
            &eq45_reactive_node_derivatives,
            &branches,
            &eq45_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16, eq46_e2347_q, eq46_e2347_q_d_n0, eq46_e2347_q_d_n1, eq46_e2347_q_d_n2, eq46_e2347_q_d_n3, eq46_e2347_q_d_n4, eq46_e2347_q_d_n5, eq46_e2347_q_d_n6, eq46_e2347_q_d_n7, eq46_e2347_q_d_n8, eq46_e2347_q_d_n9, eq46_e2347_q_d_n10, eq46_e2347_q_d_n11, eq46_e2347_q_d_n12, eq46_e2347_q_d_n13, eq46_e2347_q_d_n14, eq46_e2347_q_d_n15, eq46_e2347_q_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1707] != 0.0)) {
        let eq46_e2345_q: f64 = s.v[501];
        (s.v[501], s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], eq46_e2345_q, s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 17] = [eq46_e2347_q_d_n0, eq46_e2347_q_d_n1, eq46_e2347_q_d_n2, eq46_e2347_q_d_n3, eq46_e2347_q_d_n4, eq46_e2347_q_d_n5, eq46_e2347_q_d_n6, eq46_e2347_q_d_n7, eq46_e2347_q_d_n8, eq46_e2347_q_d_n9, eq46_e2347_q_d_n10, eq46_e2347_q_d_n11, eq46_e2347_q_d_n12, eq46_e2347_q_d_n13, eq46_e2347_q_d_n14, eq46_e2347_q_d_n15, eq46_e2347_q_d_n16];
        let eq46_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            &nodes,
            &eq46_reactive_node_derivatives,
            &branches,
            &eq46_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16, eq47_e2353_q, eq47_e2353_q_d_n0, eq47_e2353_q_d_n1, eq47_e2353_q_d_n2, eq47_e2353_q_d_n3, eq47_e2353_q_d_n4, eq47_e2353_q_d_n5, eq47_e2353_q_d_n6, eq47_e2353_q_d_n7, eq47_e2353_q_d_n8, eq47_e2353_q_d_n9, eq47_e2353_q_d_n10, eq47_e2353_q_d_n11, eq47_e2353_q_d_n12, eq47_e2353_q_d_n13, eq47_e2353_q_d_n14, eq47_e2353_q_d_n15, eq47_e2353_q_d_n16,) = {
    if (!(s.v[1705] != 0.0)) {
        let eq47_e2351_q: f64 = s.v[505];
        (s.v[505], s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16], eq47_e2351_q, s.dn[505][0], s.dn[505][1], s.dn[505][2], s.dn[505][3], s.dn[505][4], s.dn[505][5], s.dn[505][6], s.dn[505][7], s.dn[505][8], s.dn[505][9], s.dn[505][10], s.dn[505][11], s.dn[505][12], s.dn[505][13], s.dn[505][14], s.dn[505][15], s.dn[505][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 17] = [eq47_e2353_q_d_n0, eq47_e2353_q_d_n1, eq47_e2353_q_d_n2, eq47_e2353_q_d_n3, eq47_e2353_q_d_n4, eq47_e2353_q_d_n5, eq47_e2353_q_d_n6, eq47_e2353_q_d_n7, eq47_e2353_q_d_n8, eq47_e2353_q_d_n9, eq47_e2353_q_d_n10, eq47_e2353_q_d_n11, eq47_e2353_q_d_n12, eq47_e2353_q_d_n13, eq47_e2353_q_d_n14, eq47_e2353_q_d_n15, eq47_e2353_q_d_n16];
        let eq47_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            &nodes,
            &eq47_reactive_node_derivatives,
            &branches,
            &eq47_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16, eq48_e2361_q, eq48_e2361_q_d_n0, eq48_e2361_q_d_n1, eq48_e2361_q_d_n2, eq48_e2361_q_d_n3, eq48_e2361_q_d_n4, eq48_e2361_q_d_n5, eq48_e2361_q_d_n6, eq48_e2361_q_d_n7, eq48_e2361_q_d_n8, eq48_e2361_q_d_n9, eq48_e2361_q_d_n10, eq48_e2361_q_d_n11, eq48_e2361_q_d_n12, eq48_e2361_q_d_n13, eq48_e2361_q_d_n14, eq48_e2361_q_d_n15, eq48_e2361_q_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
        let eq48_e2359_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], eq48_e2359_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 17] = [eq48_e2361_q_d_n0, eq48_e2361_q_d_n1, eq48_e2361_q_d_n2, eq48_e2361_q_d_n3, eq48_e2361_q_d_n4, eq48_e2361_q_d_n5, eq48_e2361_q_d_n6, eq48_e2361_q_d_n7, eq48_e2361_q_d_n8, eq48_e2361_q_d_n9, eq48_e2361_q_d_n10, eq48_e2361_q_d_n11, eq48_e2361_q_d_n12, eq48_e2361_q_d_n13, eq48_e2361_q_d_n14, eq48_e2361_q_d_n15, eq48_e2361_q_d_n16];
        let eq48_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            &nodes,
            &eq48_reactive_node_derivatives,
            &branches,
            &eq48_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_49_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16, eq49_e2371_q, eq49_e2371_q_d_n0, eq49_e2371_q_d_n1, eq49_e2371_q_d_n2, eq49_e2371_q_d_n3, eq49_e2371_q_d_n4, eq49_e2371_q_d_n5, eq49_e2371_q_d_n6, eq49_e2371_q_d_n7, eq49_e2371_q_d_n8, eq49_e2371_q_d_n9, eq49_e2371_q_d_n10, eq49_e2371_q_d_n11, eq49_e2371_q_d_n12, eq49_e2371_q_d_n13, eq49_e2371_q_d_n14, eq49_e2371_q_d_n15, eq49_e2371_q_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
        let eq49_e2368_q: f64 = s.v[503];
        let eq49_e2369: f64 = (s.v[114] * s.v[503]);
        let eq49_e2369_d_n0: f64 = ((s.dn[114][0] * s.v[503]) + (s.v[114] * s.dn[503][0]));
        let eq49_e2369_d_n1: f64 = ((s.dn[114][1] * s.v[503]) + (s.v[114] * s.dn[503][1]));
        let eq49_e2369_d_n2: f64 = ((s.dn[114][2] * s.v[503]) + (s.v[114] * s.dn[503][2]));
        let eq49_e2369_d_n3: f64 = ((s.dn[114][3] * s.v[503]) + (s.v[114] * s.dn[503][3]));
        let eq49_e2369_d_n4: f64 = ((s.dn[114][4] * s.v[503]) + (s.v[114] * s.dn[503][4]));
        let eq49_e2369_d_n5: f64 = ((s.dn[114][5] * s.v[503]) + (s.v[114] * s.dn[503][5]));
        let eq49_e2369_d_n6: f64 = ((s.dn[114][6] * s.v[503]) + (s.v[114] * s.dn[503][6]));
        let eq49_e2369_d_n7: f64 = ((s.dn[114][7] * s.v[503]) + (s.v[114] * s.dn[503][7]));
        let eq49_e2369_d_n8: f64 = ((s.dn[114][8] * s.v[503]) + (s.v[114] * s.dn[503][8]));
        let eq49_e2369_d_n9: f64 = ((s.dn[114][9] * s.v[503]) + (s.v[114] * s.dn[503][9]));
        let eq49_e2369_d_n10: f64 = ((s.dn[114][10] * s.v[503]) + (s.v[114] * s.dn[503][10]));
        let eq49_e2369_d_n11: f64 = ((s.dn[114][11] * s.v[503]) + (s.v[114] * s.dn[503][11]));
        let eq49_e2369_d_n12: f64 = ((s.dn[114][12] * s.v[503]) + (s.v[114] * s.dn[503][12]));
        let eq49_e2369_d_n13: f64 = ((s.dn[114][13] * s.v[503]) + (s.v[114] * s.dn[503][13]));
        let eq49_e2369_d_n14: f64 = ((s.dn[114][14] * s.v[503]) + (s.v[114] * s.dn[503][14]));
        let eq49_e2369_d_n15: f64 = ((s.dn[114][15] * s.v[503]) + (s.v[114] * s.dn[503][15]));
        let eq49_e2369_d_n16: f64 = ((s.dn[114][16] * s.v[503]) + (s.v[114] * s.dn[503][16]));
        let eq49_e2369_q: f64 = (s.v[114] * eq49_e2368_q);
        let eq49_e2369_q_d_n0: f64 = ((s.dn[114][0] * eq49_e2368_q) + (s.v[114] * s.dn[503][0]));
        let eq49_e2369_q_d_n1: f64 = ((s.dn[114][1] * eq49_e2368_q) + (s.v[114] * s.dn[503][1]));
        let eq49_e2369_q_d_n2: f64 = ((s.dn[114][2] * eq49_e2368_q) + (s.v[114] * s.dn[503][2]));
        let eq49_e2369_q_d_n3: f64 = ((s.dn[114][3] * eq49_e2368_q) + (s.v[114] * s.dn[503][3]));
        let eq49_e2369_q_d_n4: f64 = ((s.dn[114][4] * eq49_e2368_q) + (s.v[114] * s.dn[503][4]));
        let eq49_e2369_q_d_n5: f64 = ((s.dn[114][5] * eq49_e2368_q) + (s.v[114] * s.dn[503][5]));
        let eq49_e2369_q_d_n6: f64 = ((s.dn[114][6] * eq49_e2368_q) + (s.v[114] * s.dn[503][6]));
        let eq49_e2369_q_d_n7: f64 = ((s.dn[114][7] * eq49_e2368_q) + (s.v[114] * s.dn[503][7]));
        let eq49_e2369_q_d_n8: f64 = ((s.dn[114][8] * eq49_e2368_q) + (s.v[114] * s.dn[503][8]));
        let eq49_e2369_q_d_n9: f64 = ((s.dn[114][9] * eq49_e2368_q) + (s.v[114] * s.dn[503][9]));
        let eq49_e2369_q_d_n10: f64 = ((s.dn[114][10] * eq49_e2368_q) + (s.v[114] * s.dn[503][10]));
        let eq49_e2369_q_d_n11: f64 = ((s.dn[114][11] * eq49_e2368_q) + (s.v[114] * s.dn[503][11]));
        let eq49_e2369_q_d_n12: f64 = ((s.dn[114][12] * eq49_e2368_q) + (s.v[114] * s.dn[503][12]));
        let eq49_e2369_q_d_n13: f64 = ((s.dn[114][13] * eq49_e2368_q) + (s.v[114] * s.dn[503][13]));
        let eq49_e2369_q_d_n14: f64 = ((s.dn[114][14] * eq49_e2368_q) + (s.v[114] * s.dn[503][14]));
        let eq49_e2369_q_d_n15: f64 = ((s.dn[114][15] * eq49_e2368_q) + (s.v[114] * s.dn[503][15]));
        let eq49_e2369_q_d_n16: f64 = ((s.dn[114][16] * eq49_e2368_q) + (s.v[114] * s.dn[503][16]));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n1, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n12, eq49_e2369_d_n13, eq49_e2369_d_n14, eq49_e2369_d_n15, eq49_e2369_d_n16, eq49_e2369_q, eq49_e2369_q_d_n0, eq49_e2369_q_d_n1, eq49_e2369_q_d_n2, eq49_e2369_q_d_n3, eq49_e2369_q_d_n4, eq49_e2369_q_d_n5, eq49_e2369_q_d_n6, eq49_e2369_q_d_n7, eq49_e2369_q_d_n8, eq49_e2369_q_d_n9, eq49_e2369_q_d_n10, eq49_e2369_q_d_n11, eq49_e2369_q_d_n12, eq49_e2369_q_d_n13, eq49_e2369_q_d_n14, eq49_e2369_q_d_n15, eq49_e2369_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_reactive_node_derivatives: [f64; 17] = [eq49_e2371_q_d_n0, eq49_e2371_q_d_n1, eq49_e2371_q_d_n2, eq49_e2371_q_d_n3, eq49_e2371_q_d_n4, eq49_e2371_q_d_n5, eq49_e2371_q_d_n6, eq49_e2371_q_d_n7, eq49_e2371_q_d_n8, eq49_e2371_q_d_n9, eq49_e2371_q_d_n10, eq49_e2371_q_d_n11, eq49_e2371_q_d_n12, eq49_e2371_q_d_n13, eq49_e2371_q_d_n14, eq49_e2371_q_d_n15, eq49_e2371_q_d_n16];
        let eq49_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            &nodes,
            &eq49_reactive_node_derivatives,
            &branches,
            &eq49_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_50_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16, eq50_e2381_q, eq50_e2381_q_d_n0, eq50_e2381_q_d_n1, eq50_e2381_q_d_n2, eq50_e2381_q_d_n3, eq50_e2381_q_d_n4, eq50_e2381_q_d_n5, eq50_e2381_q_d_n6, eq50_e2381_q_d_n7, eq50_e2381_q_d_n8, eq50_e2381_q_d_n9, eq50_e2381_q_d_n10, eq50_e2381_q_d_n11, eq50_e2381_q_d_n12, eq50_e2381_q_d_n13, eq50_e2381_q_d_n14, eq50_e2381_q_d_n15, eq50_e2381_q_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
        let eq50_e2378_q: f64 = s.v[504];
        let eq50_e2379: f64 = (s.v[114] * s.v[504]);
        let eq50_e2379_d_n0: f64 = ((s.dn[114][0] * s.v[504]) + (s.v[114] * s.dn[504][0]));
        let eq50_e2379_d_n1: f64 = ((s.dn[114][1] * s.v[504]) + (s.v[114] * s.dn[504][1]));
        let eq50_e2379_d_n2: f64 = ((s.dn[114][2] * s.v[504]) + (s.v[114] * s.dn[504][2]));
        let eq50_e2379_d_n3: f64 = ((s.dn[114][3] * s.v[504]) + (s.v[114] * s.dn[504][3]));
        let eq50_e2379_d_n4: f64 = ((s.dn[114][4] * s.v[504]) + (s.v[114] * s.dn[504][4]));
        let eq50_e2379_d_n5: f64 = ((s.dn[114][5] * s.v[504]) + (s.v[114] * s.dn[504][5]));
        let eq50_e2379_d_n6: f64 = ((s.dn[114][6] * s.v[504]) + (s.v[114] * s.dn[504][6]));
        let eq50_e2379_d_n7: f64 = ((s.dn[114][7] * s.v[504]) + (s.v[114] * s.dn[504][7]));
        let eq50_e2379_d_n8: f64 = ((s.dn[114][8] * s.v[504]) + (s.v[114] * s.dn[504][8]));
        let eq50_e2379_d_n9: f64 = ((s.dn[114][9] * s.v[504]) + (s.v[114] * s.dn[504][9]));
        let eq50_e2379_d_n10: f64 = ((s.dn[114][10] * s.v[504]) + (s.v[114] * s.dn[504][10]));
        let eq50_e2379_d_n11: f64 = ((s.dn[114][11] * s.v[504]) + (s.v[114] * s.dn[504][11]));
        let eq50_e2379_d_n12: f64 = ((s.dn[114][12] * s.v[504]) + (s.v[114] * s.dn[504][12]));
        let eq50_e2379_d_n13: f64 = ((s.dn[114][13] * s.v[504]) + (s.v[114] * s.dn[504][13]));
        let eq50_e2379_d_n14: f64 = ((s.dn[114][14] * s.v[504]) + (s.v[114] * s.dn[504][14]));
        let eq50_e2379_d_n15: f64 = ((s.dn[114][15] * s.v[504]) + (s.v[114] * s.dn[504][15]));
        let eq50_e2379_d_n16: f64 = ((s.dn[114][16] * s.v[504]) + (s.v[114] * s.dn[504][16]));
        let eq50_e2379_q: f64 = (s.v[114] * eq50_e2378_q);
        let eq50_e2379_q_d_n0: f64 = ((s.dn[114][0] * eq50_e2378_q) + (s.v[114] * s.dn[504][0]));
        let eq50_e2379_q_d_n1: f64 = ((s.dn[114][1] * eq50_e2378_q) + (s.v[114] * s.dn[504][1]));
        let eq50_e2379_q_d_n2: f64 = ((s.dn[114][2] * eq50_e2378_q) + (s.v[114] * s.dn[504][2]));
        let eq50_e2379_q_d_n3: f64 = ((s.dn[114][3] * eq50_e2378_q) + (s.v[114] * s.dn[504][3]));
        let eq50_e2379_q_d_n4: f64 = ((s.dn[114][4] * eq50_e2378_q) + (s.v[114] * s.dn[504][4]));
        let eq50_e2379_q_d_n5: f64 = ((s.dn[114][5] * eq50_e2378_q) + (s.v[114] * s.dn[504][5]));
        let eq50_e2379_q_d_n6: f64 = ((s.dn[114][6] * eq50_e2378_q) + (s.v[114] * s.dn[504][6]));
        let eq50_e2379_q_d_n7: f64 = ((s.dn[114][7] * eq50_e2378_q) + (s.v[114] * s.dn[504][7]));
        let eq50_e2379_q_d_n8: f64 = ((s.dn[114][8] * eq50_e2378_q) + (s.v[114] * s.dn[504][8]));
        let eq50_e2379_q_d_n9: f64 = ((s.dn[114][9] * eq50_e2378_q) + (s.v[114] * s.dn[504][9]));
        let eq50_e2379_q_d_n10: f64 = ((s.dn[114][10] * eq50_e2378_q) + (s.v[114] * s.dn[504][10]));
        let eq50_e2379_q_d_n11: f64 = ((s.dn[114][11] * eq50_e2378_q) + (s.v[114] * s.dn[504][11]));
        let eq50_e2379_q_d_n12: f64 = ((s.dn[114][12] * eq50_e2378_q) + (s.v[114] * s.dn[504][12]));
        let eq50_e2379_q_d_n13: f64 = ((s.dn[114][13] * eq50_e2378_q) + (s.v[114] * s.dn[504][13]));
        let eq50_e2379_q_d_n14: f64 = ((s.dn[114][14] * eq50_e2378_q) + (s.v[114] * s.dn[504][14]));
        let eq50_e2379_q_d_n15: f64 = ((s.dn[114][15] * eq50_e2378_q) + (s.v[114] * s.dn[504][15]));
        let eq50_e2379_q_d_n16: f64 = ((s.dn[114][16] * eq50_e2378_q) + (s.v[114] * s.dn[504][16]));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n1, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n12, eq50_e2379_d_n13, eq50_e2379_d_n14, eq50_e2379_d_n15, eq50_e2379_d_n16, eq50_e2379_q, eq50_e2379_q_d_n0, eq50_e2379_q_d_n1, eq50_e2379_q_d_n2, eq50_e2379_q_d_n3, eq50_e2379_q_d_n4, eq50_e2379_q_d_n5, eq50_e2379_q_d_n6, eq50_e2379_q_d_n7, eq50_e2379_q_d_n8, eq50_e2379_q_d_n9, eq50_e2379_q_d_n10, eq50_e2379_q_d_n11, eq50_e2379_q_d_n12, eq50_e2379_q_d_n13, eq50_e2379_q_d_n14, eq50_e2379_q_d_n15, eq50_e2379_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 17] = [eq50_e2381_q_d_n0, eq50_e2381_q_d_n1, eq50_e2381_q_d_n2, eq50_e2381_q_d_n3, eq50_e2381_q_d_n4, eq50_e2381_q_d_n5, eq50_e2381_q_d_n6, eq50_e2381_q_d_n7, eq50_e2381_q_d_n8, eq50_e2381_q_d_n9, eq50_e2381_q_d_n10, eq50_e2381_q_d_n11, eq50_e2381_q_d_n12, eq50_e2381_q_d_n13, eq50_e2381_q_d_n14, eq50_e2381_q_d_n15, eq50_e2381_q_d_n16];
        let eq50_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            &nodes,
            &eq50_reactive_node_derivatives,
            &branches,
            &eq50_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16, eq51_e2390_q, eq51_e2390_q_d_n0, eq51_e2390_q_d_n1, eq51_e2390_q_d_n2, eq51_e2390_q_d_n3, eq51_e2390_q_d_n4, eq51_e2390_q_d_n5, eq51_e2390_q_d_n6, eq51_e2390_q_d_n7, eq51_e2390_q_d_n8, eq51_e2390_q_d_n9, eq51_e2390_q_d_n10, eq51_e2390_q_d_n11, eq51_e2390_q_d_n12, eq51_e2390_q_d_n13, eq51_e2390_q_d_n14, eq51_e2390_q_d_n15, eq51_e2390_q_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (!(s.v[1708] != 0.0))) {
        let eq51_e2388_q: f64 = s.v[506];
        (s.v[506], s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16], eq51_e2388_q, s.dn[506][0], s.dn[506][1], s.dn[506][2], s.dn[506][3], s.dn[506][4], s.dn[506][5], s.dn[506][6], s.dn[506][7], s.dn[506][8], s.dn[506][9], s.dn[506][10], s.dn[506][11], s.dn[506][12], s.dn[506][13], s.dn[506][14], s.dn[506][15], s.dn[506][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 17] = [eq51_e2390_q_d_n0, eq51_e2390_q_d_n1, eq51_e2390_q_d_n2, eq51_e2390_q_d_n3, eq51_e2390_q_d_n4, eq51_e2390_q_d_n5, eq51_e2390_q_d_n6, eq51_e2390_q_d_n7, eq51_e2390_q_d_n8, eq51_e2390_q_d_n9, eq51_e2390_q_d_n10, eq51_e2390_q_d_n11, eq51_e2390_q_d_n12, eq51_e2390_q_d_n13, eq51_e2390_q_d_n14, eq51_e2390_q_d_n15, eq51_e2390_q_d_n16];
        let eq51_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
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
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16, eq52_e2396_q, eq52_e2396_q_d_n0, eq52_e2396_q_d_n1, eq52_e2396_q_d_n2, eq52_e2396_q_d_n3, eq52_e2396_q_d_n4, eq52_e2396_q_d_n5, eq52_e2396_q_d_n6, eq52_e2396_q_d_n7, eq52_e2396_q_d_n8, eq52_e2396_q_d_n9, eq52_e2396_q_d_n10, eq52_e2396_q_d_n11, eq52_e2396_q_d_n12, eq52_e2396_q_d_n13, eq52_e2396_q_d_n14, eq52_e2396_q_d_n15, eq52_e2396_q_d_n16,) = {
    if (!(s.v[1705] != 0.0)) {
        let eq52_e2394_q: f64 = s.v[502];
        (s.v[502], s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16], eq52_e2394_q, s.dn[502][0], s.dn[502][1], s.dn[502][2], s.dn[502][3], s.dn[502][4], s.dn[502][5], s.dn[502][6], s.dn[502][7], s.dn[502][8], s.dn[502][9], s.dn[502][10], s.dn[502][11], s.dn[502][12], s.dn[502][13], s.dn[502][14], s.dn[502][15], s.dn[502][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 17] = [eq52_e2396_q_d_n0, eq52_e2396_q_d_n1, eq52_e2396_q_d_n2, eq52_e2396_q_d_n3, eq52_e2396_q_d_n4, eq52_e2396_q_d_n5, eq52_e2396_q_d_n6, eq52_e2396_q_d_n7, eq52_e2396_q_d_n8, eq52_e2396_q_d_n9, eq52_e2396_q_d_n10, eq52_e2396_q_d_n11, eq52_e2396_q_d_n12, eq52_e2396_q_d_n13, eq52_e2396_q_d_n14, eq52_e2396_q_d_n15, eq52_e2396_q_d_n16];
        let eq52_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
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
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16, eq53_e2404_q, eq53_e2404_q_d_n0, eq53_e2404_q_d_n1, eq53_e2404_q_d_n2, eq53_e2404_q_d_n3, eq53_e2404_q_d_n4, eq53_e2404_q_d_n5, eq53_e2404_q_d_n6, eq53_e2404_q_d_n7, eq53_e2404_q_d_n8, eq53_e2404_q_d_n9, eq53_e2404_q_d_n10, eq53_e2404_q_d_n11, eq53_e2404_q_d_n12, eq53_e2404_q_d_n13, eq53_e2404_q_d_n14, eq53_e2404_q_d_n15, eq53_e2404_q_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1709] != 0.0)) {
        let eq53_e2402_q: f64 = s.v[500];
        (s.v[500], s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16], eq53_e2402_q, s.dn[500][0], s.dn[500][1], s.dn[500][2], s.dn[500][3], s.dn[500][4], s.dn[500][5], s.dn[500][6], s.dn[500][7], s.dn[500][8], s.dn[500][9], s.dn[500][10], s.dn[500][11], s.dn[500][12], s.dn[500][13], s.dn[500][14], s.dn[500][15], s.dn[500][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 17] = [eq53_e2404_q_d_n0, eq53_e2404_q_d_n1, eq53_e2404_q_d_n2, eq53_e2404_q_d_n3, eq53_e2404_q_d_n4, eq53_e2404_q_d_n5, eq53_e2404_q_d_n6, eq53_e2404_q_d_n7, eq53_e2404_q_d_n8, eq53_e2404_q_d_n9, eq53_e2404_q_d_n10, eq53_e2404_q_d_n11, eq53_e2404_q_d_n12, eq53_e2404_q_d_n13, eq53_e2404_q_d_n14, eq53_e2404_q_d_n15, eq53_e2404_q_d_n16];
        let eq53_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[2]),
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
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16, eq54_e2412_q, eq54_e2412_q_d_n0, eq54_e2412_q_d_n1, eq54_e2412_q_d_n2, eq54_e2412_q_d_n3, eq54_e2412_q_d_n4, eq54_e2412_q_d_n5, eq54_e2412_q_d_n6, eq54_e2412_q_d_n7, eq54_e2412_q_d_n8, eq54_e2412_q_d_n9, eq54_e2412_q_d_n10, eq54_e2412_q_d_n11, eq54_e2412_q_d_n12, eq54_e2412_q_d_n13, eq54_e2412_q_d_n14, eq54_e2412_q_d_n15, eq54_e2412_q_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1709] != 0.0)) {
        let eq54_e2410_q: f64 = s.v[501];
        (s.v[501], s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16], eq54_e2410_q, s.dn[501][0], s.dn[501][1], s.dn[501][2], s.dn[501][3], s.dn[501][4], s.dn[501][5], s.dn[501][6], s.dn[501][7], s.dn[501][8], s.dn[501][9], s.dn[501][10], s.dn[501][11], s.dn[501][12], s.dn[501][13], s.dn[501][14], s.dn[501][15], s.dn[501][16],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 17] = [eq54_e2412_q_d_n0, eq54_e2412_q_d_n1, eq54_e2412_q_d_n2, eq54_e2412_q_d_n3, eq54_e2412_q_d_n4, eq54_e2412_q_d_n5, eq54_e2412_q_d_n6, eq54_e2412_q_d_n7, eq54_e2412_q_d_n8, eq54_e2412_q_d_n9, eq54_e2412_q_d_n10, eq54_e2412_q_d_n11, eq54_e2412_q_d_n12, eq54_e2412_q_d_n13, eq54_e2412_q_d_n14, eq54_e2412_q_d_n15, eq54_e2412_q_d_n16];
        let eq54_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[0]),
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
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n1, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n12, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_d_n15, eq55_e2419_d_n16, eq55_e2419_q, eq55_e2419_q_d_n0, eq55_e2419_q_d_n1, eq55_e2419_q_d_n2, eq55_e2419_q_d_n3, eq55_e2419_q_d_n4, eq55_e2419_q_d_n5, eq55_e2419_q_d_n6, eq55_e2419_q_d_n7, eq55_e2419_q_d_n8, eq55_e2419_q_d_n9, eq55_e2419_q_d_n10, eq55_e2419_q_d_n11, eq55_e2419_q_d_n12, eq55_e2419_q_d_n13, eq55_e2419_q_d_n14, eq55_e2419_q_d_n15, eq55_e2419_q_d_n16,) = {
    if (s.v[1710] != 0.0) {
        let eq55_e2416_q: f64 = s.v[495];
        let eq55_e2417: f64 = (s.v[114] * s.v[495]);
        let eq55_e2417_d_n0: f64 = ((s.dn[114][0] * s.v[495]) + (s.v[114] * s.dn[495][0]));
        let eq55_e2417_d_n1: f64 = ((s.dn[114][1] * s.v[495]) + (s.v[114] * s.dn[495][1]));
        let eq55_e2417_d_n2: f64 = ((s.dn[114][2] * s.v[495]) + (s.v[114] * s.dn[495][2]));
        let eq55_e2417_d_n3: f64 = ((s.dn[114][3] * s.v[495]) + (s.v[114] * s.dn[495][3]));
        let eq55_e2417_d_n4: f64 = ((s.dn[114][4] * s.v[495]) + (s.v[114] * s.dn[495][4]));
        let eq55_e2417_d_n5: f64 = ((s.dn[114][5] * s.v[495]) + (s.v[114] * s.dn[495][5]));
        let eq55_e2417_d_n6: f64 = ((s.dn[114][6] * s.v[495]) + (s.v[114] * s.dn[495][6]));
        let eq55_e2417_d_n7: f64 = ((s.dn[114][7] * s.v[495]) + (s.v[114] * s.dn[495][7]));
        let eq55_e2417_d_n8: f64 = ((s.dn[114][8] * s.v[495]) + (s.v[114] * s.dn[495][8]));
        let eq55_e2417_d_n9: f64 = ((s.dn[114][9] * s.v[495]) + (s.v[114] * s.dn[495][9]));
        let eq55_e2417_d_n10: f64 = ((s.dn[114][10] * s.v[495]) + (s.v[114] * s.dn[495][10]));
        let eq55_e2417_d_n11: f64 = ((s.dn[114][11] * s.v[495]) + (s.v[114] * s.dn[495][11]));
        let eq55_e2417_d_n12: f64 = ((s.dn[114][12] * s.v[495]) + (s.v[114] * s.dn[495][12]));
        let eq55_e2417_d_n13: f64 = ((s.dn[114][13] * s.v[495]) + (s.v[114] * s.dn[495][13]));
        let eq55_e2417_d_n14: f64 = ((s.dn[114][14] * s.v[495]) + (s.v[114] * s.dn[495][14]));
        let eq55_e2417_d_n15: f64 = ((s.dn[114][15] * s.v[495]) + (s.v[114] * s.dn[495][15]));
        let eq55_e2417_d_n16: f64 = ((s.dn[114][16] * s.v[495]) + (s.v[114] * s.dn[495][16]));
        let eq55_e2417_q: f64 = (s.v[114] * eq55_e2416_q);
        let eq55_e2417_q_d_n0: f64 = ((s.dn[114][0] * eq55_e2416_q) + (s.v[114] * s.dn[495][0]));
        let eq55_e2417_q_d_n1: f64 = ((s.dn[114][1] * eq55_e2416_q) + (s.v[114] * s.dn[495][1]));
        let eq55_e2417_q_d_n2: f64 = ((s.dn[114][2] * eq55_e2416_q) + (s.v[114] * s.dn[495][2]));
        let eq55_e2417_q_d_n3: f64 = ((s.dn[114][3] * eq55_e2416_q) + (s.v[114] * s.dn[495][3]));
        let eq55_e2417_q_d_n4: f64 = ((s.dn[114][4] * eq55_e2416_q) + (s.v[114] * s.dn[495][4]));
        let eq55_e2417_q_d_n5: f64 = ((s.dn[114][5] * eq55_e2416_q) + (s.v[114] * s.dn[495][5]));
        let eq55_e2417_q_d_n6: f64 = ((s.dn[114][6] * eq55_e2416_q) + (s.v[114] * s.dn[495][6]));
        let eq55_e2417_q_d_n7: f64 = ((s.dn[114][7] * eq55_e2416_q) + (s.v[114] * s.dn[495][7]));
        let eq55_e2417_q_d_n8: f64 = ((s.dn[114][8] * eq55_e2416_q) + (s.v[114] * s.dn[495][8]));
        let eq55_e2417_q_d_n9: f64 = ((s.dn[114][9] * eq55_e2416_q) + (s.v[114] * s.dn[495][9]));
        let eq55_e2417_q_d_n10: f64 = ((s.dn[114][10] * eq55_e2416_q) + (s.v[114] * s.dn[495][10]));
        let eq55_e2417_q_d_n11: f64 = ((s.dn[114][11] * eq55_e2416_q) + (s.v[114] * s.dn[495][11]));
        let eq55_e2417_q_d_n12: f64 = ((s.dn[114][12] * eq55_e2416_q) + (s.v[114] * s.dn[495][12]));
        let eq55_e2417_q_d_n13: f64 = ((s.dn[114][13] * eq55_e2416_q) + (s.v[114] * s.dn[495][13]));
        let eq55_e2417_q_d_n14: f64 = ((s.dn[114][14] * eq55_e2416_q) + (s.v[114] * s.dn[495][14]));
        let eq55_e2417_q_d_n15: f64 = ((s.dn[114][15] * eq55_e2416_q) + (s.v[114] * s.dn[495][15]));
        let eq55_e2417_q_d_n16: f64 = ((s.dn[114][16] * eq55_e2416_q) + (s.v[114] * s.dn[495][16]));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n1, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n12, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_d_n15, eq55_e2417_d_n16, eq55_e2417_q, eq55_e2417_q_d_n0, eq55_e2417_q_d_n1, eq55_e2417_q_d_n2, eq55_e2417_q_d_n3, eq55_e2417_q_d_n4, eq55_e2417_q_d_n5, eq55_e2417_q_d_n6, eq55_e2417_q_d_n7, eq55_e2417_q_d_n8, eq55_e2417_q_d_n9, eq55_e2417_q_d_n10, eq55_e2417_q_d_n11, eq55_e2417_q_d_n12, eq55_e2417_q_d_n13, eq55_e2417_q_d_n14, eq55_e2417_q_d_n15, eq55_e2417_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e2419_q_d_n0, eq55_e2419_q_d_n1, eq55_e2419_q_d_n2, eq55_e2419_q_d_n3, eq55_e2419_q_d_n4, eq55_e2419_q_d_n5, eq55_e2419_q_d_n6, eq55_e2419_q_d_n7, eq55_e2419_q_d_n8, eq55_e2419_q_d_n9, eq55_e2419_q_d_n10, eq55_e2419_q_d_n11, eq55_e2419_q_d_n12, eq55_e2419_q_d_n13, eq55_e2419_q_d_n14, eq55_e2419_q_d_n15, eq55_e2419_q_d_n16];
        let eq55_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            &nodes,
            &eq55_reactive_node_derivatives,
            &branches,
            &eq55_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

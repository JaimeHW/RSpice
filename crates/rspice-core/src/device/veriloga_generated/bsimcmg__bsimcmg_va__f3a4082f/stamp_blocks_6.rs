#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_39_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16,) = {
    if (s.v[1705] != 0.0) {
        let eq39_e2293: f64 = self.eval_ddt(5, s.v[505]);
        let eq39_e2293_d_n0: f64 = self.ddt_jacobian(s.dn[505][0]);
        let eq39_e2293_d_n1: f64 = self.ddt_jacobian(s.dn[505][1]);
        let eq39_e2293_d_n2: f64 = self.ddt_jacobian(s.dn[505][2]);
        let eq39_e2293_d_n3: f64 = self.ddt_jacobian(s.dn[505][3]);
        let eq39_e2293_d_n4: f64 = self.ddt_jacobian(s.dn[505][4]);
        let eq39_e2293_d_n5: f64 = self.ddt_jacobian(s.dn[505][5]);
        let eq39_e2293_d_n6: f64 = self.ddt_jacobian(s.dn[505][6]);
        let eq39_e2293_d_n7: f64 = self.ddt_jacobian(s.dn[505][7]);
        let eq39_e2293_d_n8: f64 = self.ddt_jacobian(s.dn[505][8]);
        let eq39_e2293_d_n9: f64 = self.ddt_jacobian(s.dn[505][9]);
        let eq39_e2293_d_n10: f64 = self.ddt_jacobian(s.dn[505][10]);
        let eq39_e2293_d_n11: f64 = self.ddt_jacobian(s.dn[505][11]);
        let eq39_e2293_d_n12: f64 = self.ddt_jacobian(s.dn[505][12]);
        let eq39_e2293_d_n13: f64 = self.ddt_jacobian(s.dn[505][13]);
        let eq39_e2293_d_n14: f64 = self.ddt_jacobian(s.dn[505][14]);
        let eq39_e2293_d_n15: f64 = self.ddt_jacobian(s.dn[505][15]);
        let eq39_e2293_d_n16: f64 = self.ddt_jacobian(s.dn[505][16]);
        (eq39_e2293, eq39_e2293_d_n0, eq39_e2293_d_n1, eq39_e2293_d_n2, eq39_e2293_d_n3, eq39_e2293_d_n4, eq39_e2293_d_n5, eq39_e2293_d_n6, eq39_e2293_d_n7, eq39_e2293_d_n8, eq39_e2293_d_n9, eq39_e2293_d_n10, eq39_e2293_d_n11, eq39_e2293_d_n12, eq39_e2293_d_n13, eq39_e2293_d_n14, eq39_e2293_d_n15, eq39_e2293_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e2295;
        let eq39_node_derivatives: [f64; 17] = [eq39_e2295_d_n0, eq39_e2295_d_n1, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n12, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_d_n15, eq39_e2295_d_n16];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            self.multiplicity * (eq39_value),
            &nodes,
            &eq39_node_derivatives,
            &branches,
            &eq39_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_40_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1706] != 0.0)) {
        let eq40_e2300: f64 = self.eval_ddt(6, s.v[506]);
        let eq40_e2300_d_n0: f64 = self.ddt_jacobian(s.dn[506][0]);
        let eq40_e2300_d_n1: f64 = self.ddt_jacobian(s.dn[506][1]);
        let eq40_e2300_d_n2: f64 = self.ddt_jacobian(s.dn[506][2]);
        let eq40_e2300_d_n3: f64 = self.ddt_jacobian(s.dn[506][3]);
        let eq40_e2300_d_n4: f64 = self.ddt_jacobian(s.dn[506][4]);
        let eq40_e2300_d_n5: f64 = self.ddt_jacobian(s.dn[506][5]);
        let eq40_e2300_d_n6: f64 = self.ddt_jacobian(s.dn[506][6]);
        let eq40_e2300_d_n7: f64 = self.ddt_jacobian(s.dn[506][7]);
        let eq40_e2300_d_n8: f64 = self.ddt_jacobian(s.dn[506][8]);
        let eq40_e2300_d_n9: f64 = self.ddt_jacobian(s.dn[506][9]);
        let eq40_e2300_d_n10: f64 = self.ddt_jacobian(s.dn[506][10]);
        let eq40_e2300_d_n11: f64 = self.ddt_jacobian(s.dn[506][11]);
        let eq40_e2300_d_n12: f64 = self.ddt_jacobian(s.dn[506][12]);
        let eq40_e2300_d_n13: f64 = self.ddt_jacobian(s.dn[506][13]);
        let eq40_e2300_d_n14: f64 = self.ddt_jacobian(s.dn[506][14]);
        let eq40_e2300_d_n15: f64 = self.ddt_jacobian(s.dn[506][15]);
        let eq40_e2300_d_n16: f64 = self.ddt_jacobian(s.dn[506][16]);
        (eq40_e2300, eq40_e2300_d_n0, eq40_e2300_d_n1, eq40_e2300_d_n2, eq40_e2300_d_n3, eq40_e2300_d_n4, eq40_e2300_d_n5, eq40_e2300_d_n6, eq40_e2300_d_n7, eq40_e2300_d_n8, eq40_e2300_d_n9, eq40_e2300_d_n10, eq40_e2300_d_n11, eq40_e2300_d_n12, eq40_e2300_d_n13, eq40_e2300_d_n14, eq40_e2300_d_n15, eq40_e2300_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e2302;
        let eq40_node_derivatives: [f64; 17] = [eq40_e2302_d_n0, eq40_e2302_d_n1, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n12, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_d_n15, eq40_e2302_d_n16];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq40_value),
            &nodes,
            &eq40_node_derivatives,
            &branches,
            &eq40_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_41_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1706] != 0.0)) {
        let eq41_e2308: f64 = self.eval_ddt(7, s.v[503]);
        let eq41_e2308_d_n0: f64 = self.ddt_jacobian(s.dn[503][0]);
        let eq41_e2308_d_n1: f64 = self.ddt_jacobian(s.dn[503][1]);
        let eq41_e2308_d_n2: f64 = self.ddt_jacobian(s.dn[503][2]);
        let eq41_e2308_d_n3: f64 = self.ddt_jacobian(s.dn[503][3]);
        let eq41_e2308_d_n4: f64 = self.ddt_jacobian(s.dn[503][4]);
        let eq41_e2308_d_n5: f64 = self.ddt_jacobian(s.dn[503][5]);
        let eq41_e2308_d_n6: f64 = self.ddt_jacobian(s.dn[503][6]);
        let eq41_e2308_d_n7: f64 = self.ddt_jacobian(s.dn[503][7]);
        let eq41_e2308_d_n8: f64 = self.ddt_jacobian(s.dn[503][8]);
        let eq41_e2308_d_n9: f64 = self.ddt_jacobian(s.dn[503][9]);
        let eq41_e2308_d_n10: f64 = self.ddt_jacobian(s.dn[503][10]);
        let eq41_e2308_d_n11: f64 = self.ddt_jacobian(s.dn[503][11]);
        let eq41_e2308_d_n12: f64 = self.ddt_jacobian(s.dn[503][12]);
        let eq41_e2308_d_n13: f64 = self.ddt_jacobian(s.dn[503][13]);
        let eq41_e2308_d_n14: f64 = self.ddt_jacobian(s.dn[503][14]);
        let eq41_e2308_d_n15: f64 = self.ddt_jacobian(s.dn[503][15]);
        let eq41_e2308_d_n16: f64 = self.ddt_jacobian(s.dn[503][16]);
        let eq41_e2309: f64 = (s.v[114] * eq41_e2308);
        let eq41_e2309_d_n0: f64 = ((s.dn[114][0] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n0));
        let eq41_e2309_d_n1: f64 = ((s.dn[114][1] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n1));
        let eq41_e2309_d_n2: f64 = ((s.dn[114][2] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n2));
        let eq41_e2309_d_n3: f64 = ((s.dn[114][3] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n3));
        let eq41_e2309_d_n4: f64 = ((s.dn[114][4] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n4));
        let eq41_e2309_d_n5: f64 = ((s.dn[114][5] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n5));
        let eq41_e2309_d_n6: f64 = ((s.dn[114][6] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n6));
        let eq41_e2309_d_n7: f64 = ((s.dn[114][7] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n7));
        let eq41_e2309_d_n8: f64 = ((s.dn[114][8] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n8));
        let eq41_e2309_d_n9: f64 = ((s.dn[114][9] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n9));
        let eq41_e2309_d_n10: f64 = ((s.dn[114][10] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n10));
        let eq41_e2309_d_n11: f64 = ((s.dn[114][11] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n11));
        let eq41_e2309_d_n12: f64 = ((s.dn[114][12] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n12));
        let eq41_e2309_d_n13: f64 = ((s.dn[114][13] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n13));
        let eq41_e2309_d_n14: f64 = ((s.dn[114][14] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n14));
        let eq41_e2309_d_n15: f64 = ((s.dn[114][15] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n15));
        let eq41_e2309_d_n16: f64 = ((s.dn[114][16] * eq41_e2308) + (s.v[114] * eq41_e2308_d_n16));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n1, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n12, eq41_e2309_d_n13, eq41_e2309_d_n14, eq41_e2309_d_n15, eq41_e2309_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e2311;
        let eq41_node_derivatives: [f64; 17] = [eq41_e2311_d_n0, eq41_e2311_d_n1, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n12, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_d_n15, eq41_e2311_d_n16];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            self.multiplicity * (eq41_value),
            &nodes,
            &eq41_node_derivatives,
            &branches,
            &eq41_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1706] != 0.0)) {
        let eq42_e2317: f64 = self.eval_ddt(8, s.v[504]);
        let eq42_e2317_d_n0: f64 = self.ddt_jacobian(s.dn[504][0]);
        let eq42_e2317_d_n1: f64 = self.ddt_jacobian(s.dn[504][1]);
        let eq42_e2317_d_n2: f64 = self.ddt_jacobian(s.dn[504][2]);
        let eq42_e2317_d_n3: f64 = self.ddt_jacobian(s.dn[504][3]);
        let eq42_e2317_d_n4: f64 = self.ddt_jacobian(s.dn[504][4]);
        let eq42_e2317_d_n5: f64 = self.ddt_jacobian(s.dn[504][5]);
        let eq42_e2317_d_n6: f64 = self.ddt_jacobian(s.dn[504][6]);
        let eq42_e2317_d_n7: f64 = self.ddt_jacobian(s.dn[504][7]);
        let eq42_e2317_d_n8: f64 = self.ddt_jacobian(s.dn[504][8]);
        let eq42_e2317_d_n9: f64 = self.ddt_jacobian(s.dn[504][9]);
        let eq42_e2317_d_n10: f64 = self.ddt_jacobian(s.dn[504][10]);
        let eq42_e2317_d_n11: f64 = self.ddt_jacobian(s.dn[504][11]);
        let eq42_e2317_d_n12: f64 = self.ddt_jacobian(s.dn[504][12]);
        let eq42_e2317_d_n13: f64 = self.ddt_jacobian(s.dn[504][13]);
        let eq42_e2317_d_n14: f64 = self.ddt_jacobian(s.dn[504][14]);
        let eq42_e2317_d_n15: f64 = self.ddt_jacobian(s.dn[504][15]);
        let eq42_e2317_d_n16: f64 = self.ddt_jacobian(s.dn[504][16]);
        let eq42_e2318: f64 = (s.v[114] * eq42_e2317);
        let eq42_e2318_d_n0: f64 = ((s.dn[114][0] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n0));
        let eq42_e2318_d_n1: f64 = ((s.dn[114][1] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n1));
        let eq42_e2318_d_n2: f64 = ((s.dn[114][2] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n2));
        let eq42_e2318_d_n3: f64 = ((s.dn[114][3] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n3));
        let eq42_e2318_d_n4: f64 = ((s.dn[114][4] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n4));
        let eq42_e2318_d_n5: f64 = ((s.dn[114][5] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n5));
        let eq42_e2318_d_n6: f64 = ((s.dn[114][6] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n6));
        let eq42_e2318_d_n7: f64 = ((s.dn[114][7] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n7));
        let eq42_e2318_d_n8: f64 = ((s.dn[114][8] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n8));
        let eq42_e2318_d_n9: f64 = ((s.dn[114][9] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n9));
        let eq42_e2318_d_n10: f64 = ((s.dn[114][10] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n10));
        let eq42_e2318_d_n11: f64 = ((s.dn[114][11] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n11));
        let eq42_e2318_d_n12: f64 = ((s.dn[114][12] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n12));
        let eq42_e2318_d_n13: f64 = ((s.dn[114][13] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n13));
        let eq42_e2318_d_n14: f64 = ((s.dn[114][14] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n14));
        let eq42_e2318_d_n15: f64 = ((s.dn[114][15] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n15));
        let eq42_e2318_d_n16: f64 = ((s.dn[114][16] * eq42_e2317) + (s.v[114] * eq42_e2317_d_n16));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n1, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n12, eq42_e2318_d_n13, eq42_e2318_d_n14, eq42_e2318_d_n15, eq42_e2318_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e2320;
        let eq42_node_derivatives: [f64; 17] = [eq42_e2320_d_n0, eq42_e2320_d_n1, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n12, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_d_n15, eq42_e2320_d_n16];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            self.multiplicity * (eq42_value),
            &nodes,
            &eq42_node_derivatives,
            &branches,
            &eq42_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16,) = {
    if ((s.v[1705] != 0.0) && (!(s.v[1706] != 0.0))) {
        let eq43_e2326: f64 = self.eval_ddt(9, s.v[506]);
        let eq43_e2326_d_n0: f64 = self.ddt_jacobian(s.dn[506][0]);
        let eq43_e2326_d_n1: f64 = self.ddt_jacobian(s.dn[506][1]);
        let eq43_e2326_d_n2: f64 = self.ddt_jacobian(s.dn[506][2]);
        let eq43_e2326_d_n3: f64 = self.ddt_jacobian(s.dn[506][3]);
        let eq43_e2326_d_n4: f64 = self.ddt_jacobian(s.dn[506][4]);
        let eq43_e2326_d_n5: f64 = self.ddt_jacobian(s.dn[506][5]);
        let eq43_e2326_d_n6: f64 = self.ddt_jacobian(s.dn[506][6]);
        let eq43_e2326_d_n7: f64 = self.ddt_jacobian(s.dn[506][7]);
        let eq43_e2326_d_n8: f64 = self.ddt_jacobian(s.dn[506][8]);
        let eq43_e2326_d_n9: f64 = self.ddt_jacobian(s.dn[506][9]);
        let eq43_e2326_d_n10: f64 = self.ddt_jacobian(s.dn[506][10]);
        let eq43_e2326_d_n11: f64 = self.ddt_jacobian(s.dn[506][11]);
        let eq43_e2326_d_n12: f64 = self.ddt_jacobian(s.dn[506][12]);
        let eq43_e2326_d_n13: f64 = self.ddt_jacobian(s.dn[506][13]);
        let eq43_e2326_d_n14: f64 = self.ddt_jacobian(s.dn[506][14]);
        let eq43_e2326_d_n15: f64 = self.ddt_jacobian(s.dn[506][15]);
        let eq43_e2326_d_n16: f64 = self.ddt_jacobian(s.dn[506][16]);
        (eq43_e2326, eq43_e2326_d_n0, eq43_e2326_d_n1, eq43_e2326_d_n2, eq43_e2326_d_n3, eq43_e2326_d_n4, eq43_e2326_d_n5, eq43_e2326_d_n6, eq43_e2326_d_n7, eq43_e2326_d_n8, eq43_e2326_d_n9, eq43_e2326_d_n10, eq43_e2326_d_n11, eq43_e2326_d_n12, eq43_e2326_d_n13, eq43_e2326_d_n14, eq43_e2326_d_n15, eq43_e2326_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e2328;
        let eq43_node_derivatives: [f64; 17] = [eq43_e2328_d_n0, eq43_e2328_d_n1, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n12, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_d_n15, eq43_e2328_d_n16];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            self.multiplicity * (eq43_value),
            &nodes,
            &eq43_node_derivatives,
            &branches,
            &eq43_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16,) = {
    if (s.v[1705] != 0.0) {
        let eq44_e2331: f64 = self.eval_ddt(10, s.v[502]);
        let eq44_e2331_d_n0: f64 = self.ddt_jacobian(s.dn[502][0]);
        let eq44_e2331_d_n1: f64 = self.ddt_jacobian(s.dn[502][1]);
        let eq44_e2331_d_n2: f64 = self.ddt_jacobian(s.dn[502][2]);
        let eq44_e2331_d_n3: f64 = self.ddt_jacobian(s.dn[502][3]);
        let eq44_e2331_d_n4: f64 = self.ddt_jacobian(s.dn[502][4]);
        let eq44_e2331_d_n5: f64 = self.ddt_jacobian(s.dn[502][5]);
        let eq44_e2331_d_n6: f64 = self.ddt_jacobian(s.dn[502][6]);
        let eq44_e2331_d_n7: f64 = self.ddt_jacobian(s.dn[502][7]);
        let eq44_e2331_d_n8: f64 = self.ddt_jacobian(s.dn[502][8]);
        let eq44_e2331_d_n9: f64 = self.ddt_jacobian(s.dn[502][9]);
        let eq44_e2331_d_n10: f64 = self.ddt_jacobian(s.dn[502][10]);
        let eq44_e2331_d_n11: f64 = self.ddt_jacobian(s.dn[502][11]);
        let eq44_e2331_d_n12: f64 = self.ddt_jacobian(s.dn[502][12]);
        let eq44_e2331_d_n13: f64 = self.ddt_jacobian(s.dn[502][13]);
        let eq44_e2331_d_n14: f64 = self.ddt_jacobian(s.dn[502][14]);
        let eq44_e2331_d_n15: f64 = self.ddt_jacobian(s.dn[502][15]);
        let eq44_e2331_d_n16: f64 = self.ddt_jacobian(s.dn[502][16]);
        (eq44_e2331, eq44_e2331_d_n0, eq44_e2331_d_n1, eq44_e2331_d_n2, eq44_e2331_d_n3, eq44_e2331_d_n4, eq44_e2331_d_n5, eq44_e2331_d_n6, eq44_e2331_d_n7, eq44_e2331_d_n8, eq44_e2331_d_n9, eq44_e2331_d_n10, eq44_e2331_d_n11, eq44_e2331_d_n12, eq44_e2331_d_n13, eq44_e2331_d_n14, eq44_e2331_d_n15, eq44_e2331_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e2333;
        let eq44_node_derivatives: [f64; 17] = [eq44_e2333_d_n0, eq44_e2333_d_n1, eq44_e2333_d_n2, eq44_e2333_d_n3, eq44_e2333_d_n4, eq44_e2333_d_n5, eq44_e2333_d_n6, eq44_e2333_d_n7, eq44_e2333_d_n8, eq44_e2333_d_n9, eq44_e2333_d_n10, eq44_e2333_d_n11, eq44_e2333_d_n12, eq44_e2333_d_n13, eq44_e2333_d_n14, eq44_e2333_d_n15, eq44_e2333_d_n16];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq44_value),
            &nodes,
            &eq44_node_derivatives,
            &branches,
            &eq44_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1707] != 0.0)) {
        let eq45_e2338: f64 = self.eval_ddt(11, s.v[500]);
        let eq45_e2338_d_n0: f64 = self.ddt_jacobian(s.dn[500][0]);
        let eq45_e2338_d_n1: f64 = self.ddt_jacobian(s.dn[500][1]);
        let eq45_e2338_d_n2: f64 = self.ddt_jacobian(s.dn[500][2]);
        let eq45_e2338_d_n3: f64 = self.ddt_jacobian(s.dn[500][3]);
        let eq45_e2338_d_n4: f64 = self.ddt_jacobian(s.dn[500][4]);
        let eq45_e2338_d_n5: f64 = self.ddt_jacobian(s.dn[500][5]);
        let eq45_e2338_d_n6: f64 = self.ddt_jacobian(s.dn[500][6]);
        let eq45_e2338_d_n7: f64 = self.ddt_jacobian(s.dn[500][7]);
        let eq45_e2338_d_n8: f64 = self.ddt_jacobian(s.dn[500][8]);
        let eq45_e2338_d_n9: f64 = self.ddt_jacobian(s.dn[500][9]);
        let eq45_e2338_d_n10: f64 = self.ddt_jacobian(s.dn[500][10]);
        let eq45_e2338_d_n11: f64 = self.ddt_jacobian(s.dn[500][11]);
        let eq45_e2338_d_n12: f64 = self.ddt_jacobian(s.dn[500][12]);
        let eq45_e2338_d_n13: f64 = self.ddt_jacobian(s.dn[500][13]);
        let eq45_e2338_d_n14: f64 = self.ddt_jacobian(s.dn[500][14]);
        let eq45_e2338_d_n15: f64 = self.ddt_jacobian(s.dn[500][15]);
        let eq45_e2338_d_n16: f64 = self.ddt_jacobian(s.dn[500][16]);
        (eq45_e2338, eq45_e2338_d_n0, eq45_e2338_d_n1, eq45_e2338_d_n2, eq45_e2338_d_n3, eq45_e2338_d_n4, eq45_e2338_d_n5, eq45_e2338_d_n6, eq45_e2338_d_n7, eq45_e2338_d_n8, eq45_e2338_d_n9, eq45_e2338_d_n10, eq45_e2338_d_n11, eq45_e2338_d_n12, eq45_e2338_d_n13, eq45_e2338_d_n14, eq45_e2338_d_n15, eq45_e2338_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e2340;
        let eq45_node_derivatives: [f64; 17] = [eq45_e2340_d_n0, eq45_e2340_d_n1, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n12, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_d_n15, eq45_e2340_d_n16];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            self.multiplicity * (eq45_value),
            &nodes,
            &eq45_node_derivatives,
            &branches,
            &eq45_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16,) = {
    if ((s.v[1705] != 0.0) && (s.v[1707] != 0.0)) {
        let eq46_e2345: f64 = self.eval_ddt(12, s.v[501]);
        let eq46_e2345_d_n0: f64 = self.ddt_jacobian(s.dn[501][0]);
        let eq46_e2345_d_n1: f64 = self.ddt_jacobian(s.dn[501][1]);
        let eq46_e2345_d_n2: f64 = self.ddt_jacobian(s.dn[501][2]);
        let eq46_e2345_d_n3: f64 = self.ddt_jacobian(s.dn[501][3]);
        let eq46_e2345_d_n4: f64 = self.ddt_jacobian(s.dn[501][4]);
        let eq46_e2345_d_n5: f64 = self.ddt_jacobian(s.dn[501][5]);
        let eq46_e2345_d_n6: f64 = self.ddt_jacobian(s.dn[501][6]);
        let eq46_e2345_d_n7: f64 = self.ddt_jacobian(s.dn[501][7]);
        let eq46_e2345_d_n8: f64 = self.ddt_jacobian(s.dn[501][8]);
        let eq46_e2345_d_n9: f64 = self.ddt_jacobian(s.dn[501][9]);
        let eq46_e2345_d_n10: f64 = self.ddt_jacobian(s.dn[501][10]);
        let eq46_e2345_d_n11: f64 = self.ddt_jacobian(s.dn[501][11]);
        let eq46_e2345_d_n12: f64 = self.ddt_jacobian(s.dn[501][12]);
        let eq46_e2345_d_n13: f64 = self.ddt_jacobian(s.dn[501][13]);
        let eq46_e2345_d_n14: f64 = self.ddt_jacobian(s.dn[501][14]);
        let eq46_e2345_d_n15: f64 = self.ddt_jacobian(s.dn[501][15]);
        let eq46_e2345_d_n16: f64 = self.ddt_jacobian(s.dn[501][16]);
        (eq46_e2345, eq46_e2345_d_n0, eq46_e2345_d_n1, eq46_e2345_d_n2, eq46_e2345_d_n3, eq46_e2345_d_n4, eq46_e2345_d_n5, eq46_e2345_d_n6, eq46_e2345_d_n7, eq46_e2345_d_n8, eq46_e2345_d_n9, eq46_e2345_d_n10, eq46_e2345_d_n11, eq46_e2345_d_n12, eq46_e2345_d_n13, eq46_e2345_d_n14, eq46_e2345_d_n15, eq46_e2345_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e2347;
        let eq46_node_derivatives: [f64; 17] = [eq46_e2347_d_n0, eq46_e2347_d_n1, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n12, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_d_n15, eq46_e2347_d_n16];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            self.multiplicity * (eq46_value),
            &nodes,
            &eq46_node_derivatives,
            &branches,
            &eq46_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16,) = {
    if (!(s.v[1705] != 0.0)) {
        let eq47_e2351: f64 = self.eval_ddt(13, s.v[505]);
        let eq47_e2351_d_n0: f64 = self.ddt_jacobian(s.dn[505][0]);
        let eq47_e2351_d_n1: f64 = self.ddt_jacobian(s.dn[505][1]);
        let eq47_e2351_d_n2: f64 = self.ddt_jacobian(s.dn[505][2]);
        let eq47_e2351_d_n3: f64 = self.ddt_jacobian(s.dn[505][3]);
        let eq47_e2351_d_n4: f64 = self.ddt_jacobian(s.dn[505][4]);
        let eq47_e2351_d_n5: f64 = self.ddt_jacobian(s.dn[505][5]);
        let eq47_e2351_d_n6: f64 = self.ddt_jacobian(s.dn[505][6]);
        let eq47_e2351_d_n7: f64 = self.ddt_jacobian(s.dn[505][7]);
        let eq47_e2351_d_n8: f64 = self.ddt_jacobian(s.dn[505][8]);
        let eq47_e2351_d_n9: f64 = self.ddt_jacobian(s.dn[505][9]);
        let eq47_e2351_d_n10: f64 = self.ddt_jacobian(s.dn[505][10]);
        let eq47_e2351_d_n11: f64 = self.ddt_jacobian(s.dn[505][11]);
        let eq47_e2351_d_n12: f64 = self.ddt_jacobian(s.dn[505][12]);
        let eq47_e2351_d_n13: f64 = self.ddt_jacobian(s.dn[505][13]);
        let eq47_e2351_d_n14: f64 = self.ddt_jacobian(s.dn[505][14]);
        let eq47_e2351_d_n15: f64 = self.ddt_jacobian(s.dn[505][15]);
        let eq47_e2351_d_n16: f64 = self.ddt_jacobian(s.dn[505][16]);
        (eq47_e2351, eq47_e2351_d_n0, eq47_e2351_d_n1, eq47_e2351_d_n2, eq47_e2351_d_n3, eq47_e2351_d_n4, eq47_e2351_d_n5, eq47_e2351_d_n6, eq47_e2351_d_n7, eq47_e2351_d_n8, eq47_e2351_d_n9, eq47_e2351_d_n10, eq47_e2351_d_n11, eq47_e2351_d_n12, eq47_e2351_d_n13, eq47_e2351_d_n14, eq47_e2351_d_n15, eq47_e2351_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e2353;
        let eq47_node_derivatives: [f64; 17] = [eq47_e2353_d_n0, eq47_e2353_d_n1, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n12, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_d_n15, eq47_e2353_d_n16];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            self.multiplicity * (eq47_value),
            &nodes,
            &eq47_node_derivatives,
            &branches,
            &eq47_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
        let eq48_e2359: f64 = self.eval_ddt(14, s.v[506]);
        let eq48_e2359_d_n0: f64 = self.ddt_jacobian(s.dn[506][0]);
        let eq48_e2359_d_n1: f64 = self.ddt_jacobian(s.dn[506][1]);
        let eq48_e2359_d_n2: f64 = self.ddt_jacobian(s.dn[506][2]);
        let eq48_e2359_d_n3: f64 = self.ddt_jacobian(s.dn[506][3]);
        let eq48_e2359_d_n4: f64 = self.ddt_jacobian(s.dn[506][4]);
        let eq48_e2359_d_n5: f64 = self.ddt_jacobian(s.dn[506][5]);
        let eq48_e2359_d_n6: f64 = self.ddt_jacobian(s.dn[506][6]);
        let eq48_e2359_d_n7: f64 = self.ddt_jacobian(s.dn[506][7]);
        let eq48_e2359_d_n8: f64 = self.ddt_jacobian(s.dn[506][8]);
        let eq48_e2359_d_n9: f64 = self.ddt_jacobian(s.dn[506][9]);
        let eq48_e2359_d_n10: f64 = self.ddt_jacobian(s.dn[506][10]);
        let eq48_e2359_d_n11: f64 = self.ddt_jacobian(s.dn[506][11]);
        let eq48_e2359_d_n12: f64 = self.ddt_jacobian(s.dn[506][12]);
        let eq48_e2359_d_n13: f64 = self.ddt_jacobian(s.dn[506][13]);
        let eq48_e2359_d_n14: f64 = self.ddt_jacobian(s.dn[506][14]);
        let eq48_e2359_d_n15: f64 = self.ddt_jacobian(s.dn[506][15]);
        let eq48_e2359_d_n16: f64 = self.ddt_jacobian(s.dn[506][16]);
        (eq48_e2359, eq48_e2359_d_n0, eq48_e2359_d_n1, eq48_e2359_d_n2, eq48_e2359_d_n3, eq48_e2359_d_n4, eq48_e2359_d_n5, eq48_e2359_d_n6, eq48_e2359_d_n7, eq48_e2359_d_n8, eq48_e2359_d_n9, eq48_e2359_d_n10, eq48_e2359_d_n11, eq48_e2359_d_n12, eq48_e2359_d_n13, eq48_e2359_d_n14, eq48_e2359_d_n15, eq48_e2359_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e2361;
        let eq48_node_derivatives: [f64; 17] = [eq48_e2361_d_n0, eq48_e2361_d_n1, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n12, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_d_n15, eq48_e2361_d_n16];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            self.multiplicity * (eq48_value),
            &nodes,
            &eq48_node_derivatives,
            &branches,
            &eq48_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
        let eq49_e2368: f64 = self.eval_ddt(15, s.v[503]);
        let eq49_e2368_d_n0: f64 = self.ddt_jacobian(s.dn[503][0]);
        let eq49_e2368_d_n1: f64 = self.ddt_jacobian(s.dn[503][1]);
        let eq49_e2368_d_n2: f64 = self.ddt_jacobian(s.dn[503][2]);
        let eq49_e2368_d_n3: f64 = self.ddt_jacobian(s.dn[503][3]);
        let eq49_e2368_d_n4: f64 = self.ddt_jacobian(s.dn[503][4]);
        let eq49_e2368_d_n5: f64 = self.ddt_jacobian(s.dn[503][5]);
        let eq49_e2368_d_n6: f64 = self.ddt_jacobian(s.dn[503][6]);
        let eq49_e2368_d_n7: f64 = self.ddt_jacobian(s.dn[503][7]);
        let eq49_e2368_d_n8: f64 = self.ddt_jacobian(s.dn[503][8]);
        let eq49_e2368_d_n9: f64 = self.ddt_jacobian(s.dn[503][9]);
        let eq49_e2368_d_n10: f64 = self.ddt_jacobian(s.dn[503][10]);
        let eq49_e2368_d_n11: f64 = self.ddt_jacobian(s.dn[503][11]);
        let eq49_e2368_d_n12: f64 = self.ddt_jacobian(s.dn[503][12]);
        let eq49_e2368_d_n13: f64 = self.ddt_jacobian(s.dn[503][13]);
        let eq49_e2368_d_n14: f64 = self.ddt_jacobian(s.dn[503][14]);
        let eq49_e2368_d_n15: f64 = self.ddt_jacobian(s.dn[503][15]);
        let eq49_e2368_d_n16: f64 = self.ddt_jacobian(s.dn[503][16]);
        let eq49_e2369: f64 = (s.v[114] * eq49_e2368);
        let eq49_e2369_d_n0: f64 = ((s.dn[114][0] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n0));
        let eq49_e2369_d_n1: f64 = ((s.dn[114][1] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n1));
        let eq49_e2369_d_n2: f64 = ((s.dn[114][2] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n2));
        let eq49_e2369_d_n3: f64 = ((s.dn[114][3] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n3));
        let eq49_e2369_d_n4: f64 = ((s.dn[114][4] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n4));
        let eq49_e2369_d_n5: f64 = ((s.dn[114][5] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n5));
        let eq49_e2369_d_n6: f64 = ((s.dn[114][6] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n6));
        let eq49_e2369_d_n7: f64 = ((s.dn[114][7] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n7));
        let eq49_e2369_d_n8: f64 = ((s.dn[114][8] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n8));
        let eq49_e2369_d_n9: f64 = ((s.dn[114][9] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n9));
        let eq49_e2369_d_n10: f64 = ((s.dn[114][10] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n10));
        let eq49_e2369_d_n11: f64 = ((s.dn[114][11] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n11));
        let eq49_e2369_d_n12: f64 = ((s.dn[114][12] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n12));
        let eq49_e2369_d_n13: f64 = ((s.dn[114][13] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n13));
        let eq49_e2369_d_n14: f64 = ((s.dn[114][14] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n14));
        let eq49_e2369_d_n15: f64 = ((s.dn[114][15] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n15));
        let eq49_e2369_d_n16: f64 = ((s.dn[114][16] * eq49_e2368) + (s.v[114] * eq49_e2368_d_n16));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n1, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n12, eq49_e2369_d_n13, eq49_e2369_d_n14, eq49_e2369_d_n15, eq49_e2369_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e2371;
        let eq49_node_derivatives: [f64; 17] = [eq49_e2371_d_n0, eq49_e2371_d_n1, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n12, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_d_n15, eq49_e2371_d_n16];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            self.multiplicity * (eq49_value),
            &nodes,
            &eq49_node_derivatives,
            &branches,
            &eq49_branch_derivatives,
            self.multiplicity,
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
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1708] != 0.0)) {
        let eq50_e2378: f64 = self.eval_ddt(16, s.v[504]);
        let eq50_e2378_d_n0: f64 = self.ddt_jacobian(s.dn[504][0]);
        let eq50_e2378_d_n1: f64 = self.ddt_jacobian(s.dn[504][1]);
        let eq50_e2378_d_n2: f64 = self.ddt_jacobian(s.dn[504][2]);
        let eq50_e2378_d_n3: f64 = self.ddt_jacobian(s.dn[504][3]);
        let eq50_e2378_d_n4: f64 = self.ddt_jacobian(s.dn[504][4]);
        let eq50_e2378_d_n5: f64 = self.ddt_jacobian(s.dn[504][5]);
        let eq50_e2378_d_n6: f64 = self.ddt_jacobian(s.dn[504][6]);
        let eq50_e2378_d_n7: f64 = self.ddt_jacobian(s.dn[504][7]);
        let eq50_e2378_d_n8: f64 = self.ddt_jacobian(s.dn[504][8]);
        let eq50_e2378_d_n9: f64 = self.ddt_jacobian(s.dn[504][9]);
        let eq50_e2378_d_n10: f64 = self.ddt_jacobian(s.dn[504][10]);
        let eq50_e2378_d_n11: f64 = self.ddt_jacobian(s.dn[504][11]);
        let eq50_e2378_d_n12: f64 = self.ddt_jacobian(s.dn[504][12]);
        let eq50_e2378_d_n13: f64 = self.ddt_jacobian(s.dn[504][13]);
        let eq50_e2378_d_n14: f64 = self.ddt_jacobian(s.dn[504][14]);
        let eq50_e2378_d_n15: f64 = self.ddt_jacobian(s.dn[504][15]);
        let eq50_e2378_d_n16: f64 = self.ddt_jacobian(s.dn[504][16]);
        let eq50_e2379: f64 = (s.v[114] * eq50_e2378);
        let eq50_e2379_d_n0: f64 = ((s.dn[114][0] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n0));
        let eq50_e2379_d_n1: f64 = ((s.dn[114][1] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n1));
        let eq50_e2379_d_n2: f64 = ((s.dn[114][2] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n2));
        let eq50_e2379_d_n3: f64 = ((s.dn[114][3] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n3));
        let eq50_e2379_d_n4: f64 = ((s.dn[114][4] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n4));
        let eq50_e2379_d_n5: f64 = ((s.dn[114][5] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n5));
        let eq50_e2379_d_n6: f64 = ((s.dn[114][6] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n6));
        let eq50_e2379_d_n7: f64 = ((s.dn[114][7] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n7));
        let eq50_e2379_d_n8: f64 = ((s.dn[114][8] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n8));
        let eq50_e2379_d_n9: f64 = ((s.dn[114][9] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n9));
        let eq50_e2379_d_n10: f64 = ((s.dn[114][10] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n10));
        let eq50_e2379_d_n11: f64 = ((s.dn[114][11] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n11));
        let eq50_e2379_d_n12: f64 = ((s.dn[114][12] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n12));
        let eq50_e2379_d_n13: f64 = ((s.dn[114][13] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n13));
        let eq50_e2379_d_n14: f64 = ((s.dn[114][14] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n14));
        let eq50_e2379_d_n15: f64 = ((s.dn[114][15] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n15));
        let eq50_e2379_d_n16: f64 = ((s.dn[114][16] * eq50_e2378) + (s.v[114] * eq50_e2378_d_n16));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n1, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n12, eq50_e2379_d_n13, eq50_e2379_d_n14, eq50_e2379_d_n15, eq50_e2379_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e2381;
        let eq50_node_derivatives: [f64; 17] = [eq50_e2381_d_n0, eq50_e2381_d_n1, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n12, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_d_n15, eq50_e2381_d_n16];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[5]),
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
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (!(s.v[1708] != 0.0))) {
        let eq51_e2388: f64 = self.eval_ddt(17, s.v[506]);
        let eq51_e2388_d_n0: f64 = self.ddt_jacobian(s.dn[506][0]);
        let eq51_e2388_d_n1: f64 = self.ddt_jacobian(s.dn[506][1]);
        let eq51_e2388_d_n2: f64 = self.ddt_jacobian(s.dn[506][2]);
        let eq51_e2388_d_n3: f64 = self.ddt_jacobian(s.dn[506][3]);
        let eq51_e2388_d_n4: f64 = self.ddt_jacobian(s.dn[506][4]);
        let eq51_e2388_d_n5: f64 = self.ddt_jacobian(s.dn[506][5]);
        let eq51_e2388_d_n6: f64 = self.ddt_jacobian(s.dn[506][6]);
        let eq51_e2388_d_n7: f64 = self.ddt_jacobian(s.dn[506][7]);
        let eq51_e2388_d_n8: f64 = self.ddt_jacobian(s.dn[506][8]);
        let eq51_e2388_d_n9: f64 = self.ddt_jacobian(s.dn[506][9]);
        let eq51_e2388_d_n10: f64 = self.ddt_jacobian(s.dn[506][10]);
        let eq51_e2388_d_n11: f64 = self.ddt_jacobian(s.dn[506][11]);
        let eq51_e2388_d_n12: f64 = self.ddt_jacobian(s.dn[506][12]);
        let eq51_e2388_d_n13: f64 = self.ddt_jacobian(s.dn[506][13]);
        let eq51_e2388_d_n14: f64 = self.ddt_jacobian(s.dn[506][14]);
        let eq51_e2388_d_n15: f64 = self.ddt_jacobian(s.dn[506][15]);
        let eq51_e2388_d_n16: f64 = self.ddt_jacobian(s.dn[506][16]);
        (eq51_e2388, eq51_e2388_d_n0, eq51_e2388_d_n1, eq51_e2388_d_n2, eq51_e2388_d_n3, eq51_e2388_d_n4, eq51_e2388_d_n5, eq51_e2388_d_n6, eq51_e2388_d_n7, eq51_e2388_d_n8, eq51_e2388_d_n9, eq51_e2388_d_n10, eq51_e2388_d_n11, eq51_e2388_d_n12, eq51_e2388_d_n13, eq51_e2388_d_n14, eq51_e2388_d_n15, eq51_e2388_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2390;
        let eq51_node_derivatives: [f64; 17] = [eq51_e2390_d_n0, eq51_e2390_d_n1, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n12, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_d_n15, eq51_e2390_d_n16];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[5]),
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
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16,) = {
    if (!(s.v[1705] != 0.0)) {
        let eq52_e2394: f64 = self.eval_ddt(18, s.v[502]);
        let eq52_e2394_d_n0: f64 = self.ddt_jacobian(s.dn[502][0]);
        let eq52_e2394_d_n1: f64 = self.ddt_jacobian(s.dn[502][1]);
        let eq52_e2394_d_n2: f64 = self.ddt_jacobian(s.dn[502][2]);
        let eq52_e2394_d_n3: f64 = self.ddt_jacobian(s.dn[502][3]);
        let eq52_e2394_d_n4: f64 = self.ddt_jacobian(s.dn[502][4]);
        let eq52_e2394_d_n5: f64 = self.ddt_jacobian(s.dn[502][5]);
        let eq52_e2394_d_n6: f64 = self.ddt_jacobian(s.dn[502][6]);
        let eq52_e2394_d_n7: f64 = self.ddt_jacobian(s.dn[502][7]);
        let eq52_e2394_d_n8: f64 = self.ddt_jacobian(s.dn[502][8]);
        let eq52_e2394_d_n9: f64 = self.ddt_jacobian(s.dn[502][9]);
        let eq52_e2394_d_n10: f64 = self.ddt_jacobian(s.dn[502][10]);
        let eq52_e2394_d_n11: f64 = self.ddt_jacobian(s.dn[502][11]);
        let eq52_e2394_d_n12: f64 = self.ddt_jacobian(s.dn[502][12]);
        let eq52_e2394_d_n13: f64 = self.ddt_jacobian(s.dn[502][13]);
        let eq52_e2394_d_n14: f64 = self.ddt_jacobian(s.dn[502][14]);
        let eq52_e2394_d_n15: f64 = self.ddt_jacobian(s.dn[502][15]);
        let eq52_e2394_d_n16: f64 = self.ddt_jacobian(s.dn[502][16]);
        (eq52_e2394, eq52_e2394_d_n0, eq52_e2394_d_n1, eq52_e2394_d_n2, eq52_e2394_d_n3, eq52_e2394_d_n4, eq52_e2394_d_n5, eq52_e2394_d_n6, eq52_e2394_d_n7, eq52_e2394_d_n8, eq52_e2394_d_n9, eq52_e2394_d_n10, eq52_e2394_d_n11, eq52_e2394_d_n12, eq52_e2394_d_n13, eq52_e2394_d_n14, eq52_e2394_d_n15, eq52_e2394_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2396;
        let eq52_node_derivatives: [f64; 17] = [eq52_e2396_d_n0, eq52_e2396_d_n1, eq52_e2396_d_n2, eq52_e2396_d_n3, eq52_e2396_d_n4, eq52_e2396_d_n5, eq52_e2396_d_n6, eq52_e2396_d_n7, eq52_e2396_d_n8, eq52_e2396_d_n9, eq52_e2396_d_n10, eq52_e2396_d_n11, eq52_e2396_d_n12, eq52_e2396_d_n13, eq52_e2396_d_n14, eq52_e2396_d_n15, eq52_e2396_d_n16];
        let eq52_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
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
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1709] != 0.0)) {
        let eq53_e2402: f64 = self.eval_ddt(19, s.v[500]);
        let eq53_e2402_d_n0: f64 = self.ddt_jacobian(s.dn[500][0]);
        let eq53_e2402_d_n1: f64 = self.ddt_jacobian(s.dn[500][1]);
        let eq53_e2402_d_n2: f64 = self.ddt_jacobian(s.dn[500][2]);
        let eq53_e2402_d_n3: f64 = self.ddt_jacobian(s.dn[500][3]);
        let eq53_e2402_d_n4: f64 = self.ddt_jacobian(s.dn[500][4]);
        let eq53_e2402_d_n5: f64 = self.ddt_jacobian(s.dn[500][5]);
        let eq53_e2402_d_n6: f64 = self.ddt_jacobian(s.dn[500][6]);
        let eq53_e2402_d_n7: f64 = self.ddt_jacobian(s.dn[500][7]);
        let eq53_e2402_d_n8: f64 = self.ddt_jacobian(s.dn[500][8]);
        let eq53_e2402_d_n9: f64 = self.ddt_jacobian(s.dn[500][9]);
        let eq53_e2402_d_n10: f64 = self.ddt_jacobian(s.dn[500][10]);
        let eq53_e2402_d_n11: f64 = self.ddt_jacobian(s.dn[500][11]);
        let eq53_e2402_d_n12: f64 = self.ddt_jacobian(s.dn[500][12]);
        let eq53_e2402_d_n13: f64 = self.ddt_jacobian(s.dn[500][13]);
        let eq53_e2402_d_n14: f64 = self.ddt_jacobian(s.dn[500][14]);
        let eq53_e2402_d_n15: f64 = self.ddt_jacobian(s.dn[500][15]);
        let eq53_e2402_d_n16: f64 = self.ddt_jacobian(s.dn[500][16]);
        (eq53_e2402, eq53_e2402_d_n0, eq53_e2402_d_n1, eq53_e2402_d_n2, eq53_e2402_d_n3, eq53_e2402_d_n4, eq53_e2402_d_n5, eq53_e2402_d_n6, eq53_e2402_d_n7, eq53_e2402_d_n8, eq53_e2402_d_n9, eq53_e2402_d_n10, eq53_e2402_d_n11, eq53_e2402_d_n12, eq53_e2402_d_n13, eq53_e2402_d_n14, eq53_e2402_d_n15, eq53_e2402_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2404;
        let eq53_node_derivatives: [f64; 17] = [eq53_e2404_d_n0, eq53_e2404_d_n1, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n12, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_d_n15, eq53_e2404_d_n16];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[2]),
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
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16,) = {
    if ((!(s.v[1705] != 0.0)) && (s.v[1709] != 0.0)) {
        let eq54_e2410: f64 = self.eval_ddt(20, s.v[501]);
        let eq54_e2410_d_n0: f64 = self.ddt_jacobian(s.dn[501][0]);
        let eq54_e2410_d_n1: f64 = self.ddt_jacobian(s.dn[501][1]);
        let eq54_e2410_d_n2: f64 = self.ddt_jacobian(s.dn[501][2]);
        let eq54_e2410_d_n3: f64 = self.ddt_jacobian(s.dn[501][3]);
        let eq54_e2410_d_n4: f64 = self.ddt_jacobian(s.dn[501][4]);
        let eq54_e2410_d_n5: f64 = self.ddt_jacobian(s.dn[501][5]);
        let eq54_e2410_d_n6: f64 = self.ddt_jacobian(s.dn[501][6]);
        let eq54_e2410_d_n7: f64 = self.ddt_jacobian(s.dn[501][7]);
        let eq54_e2410_d_n8: f64 = self.ddt_jacobian(s.dn[501][8]);
        let eq54_e2410_d_n9: f64 = self.ddt_jacobian(s.dn[501][9]);
        let eq54_e2410_d_n10: f64 = self.ddt_jacobian(s.dn[501][10]);
        let eq54_e2410_d_n11: f64 = self.ddt_jacobian(s.dn[501][11]);
        let eq54_e2410_d_n12: f64 = self.ddt_jacobian(s.dn[501][12]);
        let eq54_e2410_d_n13: f64 = self.ddt_jacobian(s.dn[501][13]);
        let eq54_e2410_d_n14: f64 = self.ddt_jacobian(s.dn[501][14]);
        let eq54_e2410_d_n15: f64 = self.ddt_jacobian(s.dn[501][15]);
        let eq54_e2410_d_n16: f64 = self.ddt_jacobian(s.dn[501][16]);
        (eq54_e2410, eq54_e2410_d_n0, eq54_e2410_d_n1, eq54_e2410_d_n2, eq54_e2410_d_n3, eq54_e2410_d_n4, eq54_e2410_d_n5, eq54_e2410_d_n6, eq54_e2410_d_n7, eq54_e2410_d_n8, eq54_e2410_d_n9, eq54_e2410_d_n10, eq54_e2410_d_n11, eq54_e2410_d_n12, eq54_e2410_d_n13, eq54_e2410_d_n14, eq54_e2410_d_n15, eq54_e2410_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2412;
        let eq54_node_derivatives: [f64; 17] = [eq54_e2412_d_n0, eq54_e2412_d_n1, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n12, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_d_n15, eq54_e2412_d_n16];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            self.multiplicity * (eq54_value),
            &nodes,
            &eq54_node_derivatives,
            &branches,
            &eq54_branch_derivatives,
            self.multiplicity,
        );
    }
}

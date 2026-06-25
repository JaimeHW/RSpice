#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq1_e317,) = {
    if (!(s.v[625] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e317;
        stamper.stamp_potential(
            branches[1],
            eq1_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_2_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq2_e321,) = {
    if (s.v[629] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e321;
        stamper.stamp_potential(
            branches[2],
            eq2_value,
            &[
            ],
        );
    }

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
        let eq3_e324: f64 = (p.p50 * s.v[199]);
        let eq3_e324_d_n0: f64 = (p.p50 * s.dn[199][0]);
        let eq3_e324_d_n1: f64 = (p.p50 * s.dn[199][1]);
        let eq3_e324_d_n2: f64 = (p.p50 * s.dn[199][2]);
        let eq3_e324_d_n3: f64 = (p.p50 * s.dn[199][3]);
        let eq3_e324_d_n4: f64 = (p.p50 * s.dn[199][4]);
        let eq3_e324_d_n5: f64 = (p.p50 * s.dn[199][5]);
        let eq3_e324_d_n6: f64 = (p.p50 * s.dn[199][6]);
        let eq3_e324_d_n7: f64 = (p.p50 * s.dn[199][7]);
        let eq3_e324_d_n8: f64 = (p.p50 * s.dn[199][8]);
        let eq3_e324_d_n9: f64 = (p.p50 * s.dn[199][9]);
        let eq3_e324_d_n10: f64 = (p.p50 * s.dn[199][10]);
        let eq3_e324_d_n11: f64 = (p.p50 * s.dn[199][11]);
        let eq3_e324_d_n12: f64 = (p.p50 * s.dn[199][12]);
        let eq3_e324_d_n13: f64 = (p.p50 * s.dn[199][13]);
        let eq3_e324_d_n14: f64 = (p.p50 * s.dn[199][14]);
        let eq3_e324_d_n15: f64 = (p.p50 * s.dn[199][15]);
        let eq3_e324_d_n16: f64 = (p.p50 * s.dn[199][16]);
        let eq3_e324_d_n17: f64 = (p.p50 * s.dn[199][17]);
        let eq3_e324_d_n18: f64 = (p.p50 * s.dn[199][18]);
        let eq3_e324_d_b0: f64 = (p.p50 * s.db[199][0]);
        let eq3_e324_d_b1: f64 = (p.p50 * s.db[199][1]);
        let eq3_e324_d_b2: f64 = (p.p50 * s.db[199][2]);
        let eq3_e324_d_b3: f64 = (p.p50 * s.db[199][3]);
        let eq3_e324_d_b4: f64 = (p.p50 * s.db[199][4]);
        let eq3_e324_d_b5: f64 = (p.p50 * s.db[199][5]);
        let eq3_e324_d_b6: f64 = (p.p50 * s.db[199][6]);
        let eq3_e324_d_b7: f64 = (p.p50 * s.db[199][7]);
        let eq3_e324_d_b8: f64 = (p.p50 * s.db[199][8]);
        let eq3_e324_d_b9: f64 = (p.p50 * s.db[199][9]);
        let eq3_e324_d_b10: f64 = (p.p50 * s.db[199][10]);
        let eq3_e324_d_b11: f64 = (p.p50 * s.db[199][11]);
        let eq3_e324_d_b12: f64 = (p.p50 * s.db[199][12]);
        let eq3_value: f64 = eq3_e324;
        let eq3_node_derivatives: [f64; 19] = [eq3_e324_d_n0, eq3_e324_d_n1, eq3_e324_d_n2, eq3_e324_d_n3, eq3_e324_d_n4, eq3_e324_d_n5, eq3_e324_d_n6, eq3_e324_d_n7, eq3_e324_d_n8, eq3_e324_d_n9, eq3_e324_d_n10, eq3_e324_d_n11, eq3_e324_d_n12, eq3_e324_d_n13, eq3_e324_d_n14, eq3_e324_d_n15, eq3_e324_d_n16, eq3_e324_d_n17, eq3_e324_d_n18];
        let eq3_branch_derivatives: [f64; 13] = [eq3_e324_d_b0, eq3_e324_d_b1, eq3_e324_d_b2, eq3_e324_d_b3, eq3_e324_d_b4, eq3_e324_d_b5, eq3_e324_d_b6, eq3_e324_d_b7, eq3_e324_d_b8, eq3_e324_d_b9, eq3_e324_d_b10, eq3_e324_d_b11, eq3_e324_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let (eq4_e330, eq4_e330_d_n0, eq4_e330_d_n1, eq4_e330_d_n2, eq4_e330_d_n3, eq4_e330_d_n4, eq4_e330_d_n5, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n8, eq4_e330_d_n9, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n13, eq4_e330_d_n14, eq4_e330_d_n15, eq4_e330_d_n16, eq4_e330_d_n17, eq4_e330_d_n18, eq4_e330_d_b0, eq4_e330_d_b1, eq4_e330_d_b2, eq4_e330_d_b3, eq4_e330_d_b4, eq4_e330_d_b5, eq4_e330_d_b6, eq4_e330_d_b7, eq4_e330_d_b8, eq4_e330_d_b9, eq4_e330_d_b10, eq4_e330_d_b11, eq4_e330_d_b12,) = {
    if (s.v[1848] != 0.0) {
        let eq4_e328: f64 = (p.p50 * s.v[306]);
        let eq4_e328_d_n0: f64 = (p.p50 * s.dn[306][0]);
        let eq4_e328_d_n1: f64 = (p.p50 * s.dn[306][1]);
        let eq4_e328_d_n2: f64 = (p.p50 * s.dn[306][2]);
        let eq4_e328_d_n3: f64 = (p.p50 * s.dn[306][3]);
        let eq4_e328_d_n4: f64 = (p.p50 * s.dn[306][4]);
        let eq4_e328_d_n5: f64 = (p.p50 * s.dn[306][5]);
        let eq4_e328_d_n6: f64 = (p.p50 * s.dn[306][6]);
        let eq4_e328_d_n7: f64 = (p.p50 * s.dn[306][7]);
        let eq4_e328_d_n8: f64 = (p.p50 * s.dn[306][8]);
        let eq4_e328_d_n9: f64 = (p.p50 * s.dn[306][9]);
        let eq4_e328_d_n10: f64 = (p.p50 * s.dn[306][10]);
        let eq4_e328_d_n11: f64 = (p.p50 * s.dn[306][11]);
        let eq4_e328_d_n12: f64 = (p.p50 * s.dn[306][12]);
        let eq4_e328_d_n13: f64 = (p.p50 * s.dn[306][13]);
        let eq4_e328_d_n14: f64 = (p.p50 * s.dn[306][14]);
        let eq4_e328_d_n15: f64 = (p.p50 * s.dn[306][15]);
        let eq4_e328_d_n16: f64 = (p.p50 * s.dn[306][16]);
        let eq4_e328_d_n17: f64 = (p.p50 * s.dn[306][17]);
        let eq4_e328_d_n18: f64 = (p.p50 * s.dn[306][18]);
        let eq4_e328_d_b0: f64 = (p.p50 * s.db[306][0]);
        let eq4_e328_d_b1: f64 = (p.p50 * s.db[306][1]);
        let eq4_e328_d_b2: f64 = (p.p50 * s.db[306][2]);
        let eq4_e328_d_b3: f64 = (p.p50 * s.db[306][3]);
        let eq4_e328_d_b4: f64 = (p.p50 * s.db[306][4]);
        let eq4_e328_d_b5: f64 = (p.p50 * s.db[306][5]);
        let eq4_e328_d_b6: f64 = (p.p50 * s.db[306][6]);
        let eq4_e328_d_b7: f64 = (p.p50 * s.db[306][7]);
        let eq4_e328_d_b8: f64 = (p.p50 * s.db[306][8]);
        let eq4_e328_d_b9: f64 = (p.p50 * s.db[306][9]);
        let eq4_e328_d_b10: f64 = (p.p50 * s.db[306][10]);
        let eq4_e328_d_b11: f64 = (p.p50 * s.db[306][11]);
        let eq4_e328_d_b12: f64 = (p.p50 * s.db[306][12]);
        (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18, eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11, eq4_e328_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e330;
        let eq4_node_derivatives: [f64; 19] = [eq4_e330_d_n0, eq4_e330_d_n1, eq4_e330_d_n2, eq4_e330_d_n3, eq4_e330_d_n4, eq4_e330_d_n5, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n8, eq4_e330_d_n9, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n13, eq4_e330_d_n14, eq4_e330_d_n15, eq4_e330_d_n16, eq4_e330_d_n17, eq4_e330_d_n18];
        let eq4_branch_derivatives: [f64; 13] = [eq4_e330_d_b0, eq4_e330_d_b1, eq4_e330_d_b2, eq4_e330_d_b3, eq4_e330_d_b4, eq4_e330_d_b5, eq4_e330_d_b6, eq4_e330_d_b7, eq4_e330_d_b8, eq4_e330_d_b9, eq4_e330_d_b10, eq4_e330_d_b11, eq4_e330_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[11]),
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
        let (eq5_e336, eq5_e336_d_n0, eq5_e336_d_n1, eq5_e336_d_n2, eq5_e336_d_n3, eq5_e336_d_n4, eq5_e336_d_n5, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n8, eq5_e336_d_n9, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n13, eq5_e336_d_n14, eq5_e336_d_n15, eq5_e336_d_n16, eq5_e336_d_n17, eq5_e336_d_n18, eq5_e336_d_b0, eq5_e336_d_b1, eq5_e336_d_b2, eq5_e336_d_b3, eq5_e336_d_b4, eq5_e336_d_b5, eq5_e336_d_b6, eq5_e336_d_b7, eq5_e336_d_b8, eq5_e336_d_b9, eq5_e336_d_b10, eq5_e336_d_b11, eq5_e336_d_b12,) = {
    if (s.v[1848] != 0.0) {
        let eq5_e334: f64 = (p.p50 * s.v[307]);
        let eq5_e334_d_n0: f64 = (p.p50 * s.dn[307][0]);
        let eq5_e334_d_n1: f64 = (p.p50 * s.dn[307][1]);
        let eq5_e334_d_n2: f64 = (p.p50 * s.dn[307][2]);
        let eq5_e334_d_n3: f64 = (p.p50 * s.dn[307][3]);
        let eq5_e334_d_n4: f64 = (p.p50 * s.dn[307][4]);
        let eq5_e334_d_n5: f64 = (p.p50 * s.dn[307][5]);
        let eq5_e334_d_n6: f64 = (p.p50 * s.dn[307][6]);
        let eq5_e334_d_n7: f64 = (p.p50 * s.dn[307][7]);
        let eq5_e334_d_n8: f64 = (p.p50 * s.dn[307][8]);
        let eq5_e334_d_n9: f64 = (p.p50 * s.dn[307][9]);
        let eq5_e334_d_n10: f64 = (p.p50 * s.dn[307][10]);
        let eq5_e334_d_n11: f64 = (p.p50 * s.dn[307][11]);
        let eq5_e334_d_n12: f64 = (p.p50 * s.dn[307][12]);
        let eq5_e334_d_n13: f64 = (p.p50 * s.dn[307][13]);
        let eq5_e334_d_n14: f64 = (p.p50 * s.dn[307][14]);
        let eq5_e334_d_n15: f64 = (p.p50 * s.dn[307][15]);
        let eq5_e334_d_n16: f64 = (p.p50 * s.dn[307][16]);
        let eq5_e334_d_n17: f64 = (p.p50 * s.dn[307][17]);
        let eq5_e334_d_n18: f64 = (p.p50 * s.dn[307][18]);
        let eq5_e334_d_b0: f64 = (p.p50 * s.db[307][0]);
        let eq5_e334_d_b1: f64 = (p.p50 * s.db[307][1]);
        let eq5_e334_d_b2: f64 = (p.p50 * s.db[307][2]);
        let eq5_e334_d_b3: f64 = (p.p50 * s.db[307][3]);
        let eq5_e334_d_b4: f64 = (p.p50 * s.db[307][4]);
        let eq5_e334_d_b5: f64 = (p.p50 * s.db[307][5]);
        let eq5_e334_d_b6: f64 = (p.p50 * s.db[307][6]);
        let eq5_e334_d_b7: f64 = (p.p50 * s.db[307][7]);
        let eq5_e334_d_b8: f64 = (p.p50 * s.db[307][8]);
        let eq5_e334_d_b9: f64 = (p.p50 * s.db[307][9]);
        let eq5_e334_d_b10: f64 = (p.p50 * s.db[307][10]);
        let eq5_e334_d_b11: f64 = (p.p50 * s.db[307][11]);
        let eq5_e334_d_b12: f64 = (p.p50 * s.db[307][12]);
        (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18, eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11, eq5_e334_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e336;
        let eq5_node_derivatives: [f64; 19] = [eq5_e336_d_n0, eq5_e336_d_n1, eq5_e336_d_n2, eq5_e336_d_n3, eq5_e336_d_n4, eq5_e336_d_n5, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n8, eq5_e336_d_n9, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n13, eq5_e336_d_n14, eq5_e336_d_n15, eq5_e336_d_n16, eq5_e336_d_n17, eq5_e336_d_n18];
        let eq5_branch_derivatives: [f64; 13] = [eq5_e336_d_b0, eq5_e336_d_b1, eq5_e336_d_b2, eq5_e336_d_b3, eq5_e336_d_b4, eq5_e336_d_b5, eq5_e336_d_b6, eq5_e336_d_b7, eq5_e336_d_b8, eq5_e336_d_b9, eq5_e336_d_b10, eq5_e336_d_b11, eq5_e336_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq5_value),
            &nodes,
            &eq5_node_derivatives,
            &branches,
            &eq5_branch_derivatives,
            self.multiplicity,
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
        let (eq6_e342, eq6_e342_d_n0, eq6_e342_d_n1, eq6_e342_d_n2, eq6_e342_d_n3, eq6_e342_d_n4, eq6_e342_d_n5, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n8, eq6_e342_d_n9, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n13, eq6_e342_d_n14, eq6_e342_d_n15, eq6_e342_d_n16, eq6_e342_d_n17, eq6_e342_d_n18, eq6_e342_d_b0, eq6_e342_d_b1, eq6_e342_d_b2, eq6_e342_d_b3, eq6_e342_d_b4, eq6_e342_d_b5, eq6_e342_d_b6, eq6_e342_d_b7, eq6_e342_d_b8, eq6_e342_d_b9, eq6_e342_d_b10, eq6_e342_d_b11, eq6_e342_d_b12,) = {
    if (s.v[1848] != 0.0) {
        let eq6_e340: f64 = (p.p50 * s.v[308]);
        let eq6_e340_d_n0: f64 = (p.p50 * s.dn[308][0]);
        let eq6_e340_d_n1: f64 = (p.p50 * s.dn[308][1]);
        let eq6_e340_d_n2: f64 = (p.p50 * s.dn[308][2]);
        let eq6_e340_d_n3: f64 = (p.p50 * s.dn[308][3]);
        let eq6_e340_d_n4: f64 = (p.p50 * s.dn[308][4]);
        let eq6_e340_d_n5: f64 = (p.p50 * s.dn[308][5]);
        let eq6_e340_d_n6: f64 = (p.p50 * s.dn[308][6]);
        let eq6_e340_d_n7: f64 = (p.p50 * s.dn[308][7]);
        let eq6_e340_d_n8: f64 = (p.p50 * s.dn[308][8]);
        let eq6_e340_d_n9: f64 = (p.p50 * s.dn[308][9]);
        let eq6_e340_d_n10: f64 = (p.p50 * s.dn[308][10]);
        let eq6_e340_d_n11: f64 = (p.p50 * s.dn[308][11]);
        let eq6_e340_d_n12: f64 = (p.p50 * s.dn[308][12]);
        let eq6_e340_d_n13: f64 = (p.p50 * s.dn[308][13]);
        let eq6_e340_d_n14: f64 = (p.p50 * s.dn[308][14]);
        let eq6_e340_d_n15: f64 = (p.p50 * s.dn[308][15]);
        let eq6_e340_d_n16: f64 = (p.p50 * s.dn[308][16]);
        let eq6_e340_d_n17: f64 = (p.p50 * s.dn[308][17]);
        let eq6_e340_d_n18: f64 = (p.p50 * s.dn[308][18]);
        let eq6_e340_d_b0: f64 = (p.p50 * s.db[308][0]);
        let eq6_e340_d_b1: f64 = (p.p50 * s.db[308][1]);
        let eq6_e340_d_b2: f64 = (p.p50 * s.db[308][2]);
        let eq6_e340_d_b3: f64 = (p.p50 * s.db[308][3]);
        let eq6_e340_d_b4: f64 = (p.p50 * s.db[308][4]);
        let eq6_e340_d_b5: f64 = (p.p50 * s.db[308][5]);
        let eq6_e340_d_b6: f64 = (p.p50 * s.db[308][6]);
        let eq6_e340_d_b7: f64 = (p.p50 * s.db[308][7]);
        let eq6_e340_d_b8: f64 = (p.p50 * s.db[308][8]);
        let eq6_e340_d_b9: f64 = (p.p50 * s.db[308][9]);
        let eq6_e340_d_b10: f64 = (p.p50 * s.db[308][10]);
        let eq6_e340_d_b11: f64 = (p.p50 * s.db[308][11]);
        let eq6_e340_d_b12: f64 = (p.p50 * s.db[308][12]);
        (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18, eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11, eq6_e340_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e342;
        let eq6_node_derivatives: [f64; 19] = [eq6_e342_d_n0, eq6_e342_d_n1, eq6_e342_d_n2, eq6_e342_d_n3, eq6_e342_d_n4, eq6_e342_d_n5, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n8, eq6_e342_d_n9, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n13, eq6_e342_d_n14, eq6_e342_d_n15, eq6_e342_d_n16, eq6_e342_d_n17, eq6_e342_d_n18];
        let eq6_branch_derivatives: [f64; 13] = [eq6_e342_d_b0, eq6_e342_d_b1, eq6_e342_d_b2, eq6_e342_d_b3, eq6_e342_d_b4, eq6_e342_d_b5, eq6_e342_d_b6, eq6_e342_d_b7, eq6_e342_d_b8, eq6_e342_d_b9, eq6_e342_d_b10, eq6_e342_d_b11, eq6_e342_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq6_value),
            &nodes,
            &eq6_node_derivatives,
            &branches,
            &eq6_branch_derivatives,
            self.multiplicity,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq7_e348, eq7_e348_d_n0, eq7_e348_d_n1, eq7_e348_d_n2, eq7_e348_d_n3, eq7_e348_d_n4, eq7_e348_d_n5, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n8, eq7_e348_d_n9, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n13, eq7_e348_d_n14, eq7_e348_d_n15, eq7_e348_d_n16, eq7_e348_d_n17, eq7_e348_d_n18, eq7_e348_d_b0, eq7_e348_d_b1, eq7_e348_d_b2, eq7_e348_d_b3, eq7_e348_d_b4, eq7_e348_d_b5, eq7_e348_d_b6, eq7_e348_d_b7, eq7_e348_d_b8, eq7_e348_d_b9, eq7_e348_d_b10, eq7_e348_d_b11, eq7_e348_d_b12,) = {
    if (p.p259 != 0.0) {
        let eq7_e346: f64 = ((nv7 - nv2) / s.v[1]);
        let eq7_e346_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));
        let eq7_e346_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));
        let eq7_e346_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));
        let eq7_e346_d_b12: f64 = (-(((nv7 - nv2) * s.db[1][12]) / (s.v[1] * s.v[1])));
        (eq7_e346, eq7_e346_d_n0, eq7_e346_d_n1, eq7_e346_d_n2, eq7_e346_d_n3, eq7_e346_d_n4, eq7_e346_d_n5, eq7_e346_d_n6, eq7_e346_d_n7, eq7_e346_d_n8, eq7_e346_d_n9, eq7_e346_d_n10, eq7_e346_d_n11, eq7_e346_d_n12, eq7_e346_d_n13, eq7_e346_d_n14, eq7_e346_d_n15, eq7_e346_d_n16, eq7_e346_d_n17, eq7_e346_d_n18, eq7_e346_d_b0, eq7_e346_d_b1, eq7_e346_d_b2, eq7_e346_d_b3, eq7_e346_d_b4, eq7_e346_d_b5, eq7_e346_d_b6, eq7_e346_d_b7, eq7_e346_d_b8, eq7_e346_d_b9, eq7_e346_d_b10, eq7_e346_d_b11, eq7_e346_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e348;
        let eq7_node_derivatives: [f64; 19] = [eq7_e348_d_n0, eq7_e348_d_n1, eq7_e348_d_n2, eq7_e348_d_n3, eq7_e348_d_n4, eq7_e348_d_n5, eq7_e348_d_n6, eq7_e348_d_n7, eq7_e348_d_n8, eq7_e348_d_n9, eq7_e348_d_n10, eq7_e348_d_n11, eq7_e348_d_n12, eq7_e348_d_n13, eq7_e348_d_n14, eq7_e348_d_n15, eq7_e348_d_n16, eq7_e348_d_n17, eq7_e348_d_n18];
        let eq7_branch_derivatives: [f64; 13] = [eq7_e348_d_b0, eq7_e348_d_b1, eq7_e348_d_b2, eq7_e348_d_b3, eq7_e348_d_b4, eq7_e348_d_b5, eq7_e348_d_b6, eq7_e348_d_b7, eq7_e348_d_b8, eq7_e348_d_b9, eq7_e348_d_b10, eq7_e348_d_b11, eq7_e348_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq7_value),
            &nodes,
            &eq7_node_derivatives,
            &branches,
            &eq7_branch_derivatives,
            self.multiplicity,
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
        let (eq8_e353,) = {
    if (!(p.p259 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e353;
        stamper.stamp_potential(
            branches[3],
            eq8_value,
            &[
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq9_e359, eq9_e359_d_n0, eq9_e359_d_n1, eq9_e359_d_n2, eq9_e359_d_n3, eq9_e359_d_n4, eq9_e359_d_n5, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n8, eq9_e359_d_n9, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n13, eq9_e359_d_n14, eq9_e359_d_n15, eq9_e359_d_n16, eq9_e359_d_n17, eq9_e359_d_n18, eq9_e359_d_b0, eq9_e359_d_b1, eq9_e359_d_b2, eq9_e359_d_b3, eq9_e359_d_b4, eq9_e359_d_b5, eq9_e359_d_b6, eq9_e359_d_b7, eq9_e359_d_b8, eq9_e359_d_b9, eq9_e359_d_b10, eq9_e359_d_b11, eq9_e359_d_b12,) = {
    if (p.p260 != 0.0) {
        let eq9_e357: f64 = ((nv0 - nv6) / s.v[0]);
        let eq9_e357_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));
        let eq9_e357_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));
        let eq9_e357_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));
        let eq9_e357_d_b12: f64 = (-(((nv0 - nv6) * s.db[0][12]) / (s.v[0] * s.v[0])));
        (eq9_e357, eq9_e357_d_n0, eq9_e357_d_n1, eq9_e357_d_n2, eq9_e357_d_n3, eq9_e357_d_n4, eq9_e357_d_n5, eq9_e357_d_n6, eq9_e357_d_n7, eq9_e357_d_n8, eq9_e357_d_n9, eq9_e357_d_n10, eq9_e357_d_n11, eq9_e357_d_n12, eq9_e357_d_n13, eq9_e357_d_n14, eq9_e357_d_n15, eq9_e357_d_n16, eq9_e357_d_n17, eq9_e357_d_n18, eq9_e357_d_b0, eq9_e357_d_b1, eq9_e357_d_b2, eq9_e357_d_b3, eq9_e357_d_b4, eq9_e357_d_b5, eq9_e357_d_b6, eq9_e357_d_b7, eq9_e357_d_b8, eq9_e357_d_b9, eq9_e357_d_b10, eq9_e357_d_b11, eq9_e357_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e359;
        let eq9_node_derivatives: [f64; 19] = [eq9_e359_d_n0, eq9_e359_d_n1, eq9_e359_d_n2, eq9_e359_d_n3, eq9_e359_d_n4, eq9_e359_d_n5, eq9_e359_d_n6, eq9_e359_d_n7, eq9_e359_d_n8, eq9_e359_d_n9, eq9_e359_d_n10, eq9_e359_d_n11, eq9_e359_d_n12, eq9_e359_d_n13, eq9_e359_d_n14, eq9_e359_d_n15, eq9_e359_d_n16, eq9_e359_d_n17, eq9_e359_d_n18];
        let eq9_branch_derivatives: [f64; 13] = [eq9_e359_d_b0, eq9_e359_d_b1, eq9_e359_d_b2, eq9_e359_d_b3, eq9_e359_d_b4, eq9_e359_d_b5, eq9_e359_d_b6, eq9_e359_d_b7, eq9_e359_d_b8, eq9_e359_d_b9, eq9_e359_d_b10, eq9_e359_d_b11, eq9_e359_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[6]),
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
        let (eq10_e364,) = {
    if (!(p.p260 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq10_value: f64 = eq10_e364;
        stamper.stamp_potential(
            branches[4],
            eq10_value,
            &[
            ],
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
        let eq11_e367: f64 = self.eval_ddt(0, s.v[594]);
        let eq11_e367_d_n0: f64 = self.ddt_jacobian(s.dn[594][0]);
        let eq11_e367_d_n1: f64 = self.ddt_jacobian(s.dn[594][1]);
        let eq11_e367_d_n2: f64 = self.ddt_jacobian(s.dn[594][2]);
        let eq11_e367_d_n3: f64 = self.ddt_jacobian(s.dn[594][3]);
        let eq11_e367_d_n4: f64 = self.ddt_jacobian(s.dn[594][4]);
        let eq11_e367_d_n5: f64 = self.ddt_jacobian(s.dn[594][5]);
        let eq11_e367_d_n6: f64 = self.ddt_jacobian(s.dn[594][6]);
        let eq11_e367_d_n7: f64 = self.ddt_jacobian(s.dn[594][7]);
        let eq11_e367_d_n8: f64 = self.ddt_jacobian(s.dn[594][8]);
        let eq11_e367_d_n9: f64 = self.ddt_jacobian(s.dn[594][9]);
        let eq11_e367_d_n10: f64 = self.ddt_jacobian(s.dn[594][10]);
        let eq11_e367_d_n11: f64 = self.ddt_jacobian(s.dn[594][11]);
        let eq11_e367_d_n12: f64 = self.ddt_jacobian(s.dn[594][12]);
        let eq11_e367_d_n13: f64 = self.ddt_jacobian(s.dn[594][13]);
        let eq11_e367_d_n14: f64 = self.ddt_jacobian(s.dn[594][14]);
        let eq11_e367_d_n15: f64 = self.ddt_jacobian(s.dn[594][15]);
        let eq11_e367_d_n16: f64 = self.ddt_jacobian(s.dn[594][16]);
        let eq11_e367_d_n17: f64 = self.ddt_jacobian(s.dn[594][17]);
        let eq11_e367_d_n18: f64 = self.ddt_jacobian(s.dn[594][18]);
        let eq11_e367_d_b0: f64 = self.ddt_jacobian(s.db[594][0]);
        let eq11_e367_d_b1: f64 = self.ddt_jacobian(s.db[594][1]);
        let eq11_e367_d_b2: f64 = self.ddt_jacobian(s.db[594][2]);
        let eq11_e367_d_b3: f64 = self.ddt_jacobian(s.db[594][3]);
        let eq11_e367_d_b4: f64 = self.ddt_jacobian(s.db[594][4]);
        let eq11_e367_d_b5: f64 = self.ddt_jacobian(s.db[594][5]);
        let eq11_e367_d_b6: f64 = self.ddt_jacobian(s.db[594][6]);
        let eq11_e367_d_b7: f64 = self.ddt_jacobian(s.db[594][7]);
        let eq11_e367_d_b8: f64 = self.ddt_jacobian(s.db[594][8]);
        let eq11_e367_d_b9: f64 = self.ddt_jacobian(s.db[594][9]);
        let eq11_e367_d_b10: f64 = self.ddt_jacobian(s.db[594][10]);
        let eq11_e367_d_b11: f64 = self.ddt_jacobian(s.db[594][11]);
        let eq11_e367_d_b12: f64 = self.ddt_jacobian(s.db[594][12]);
        let eq11_e368: f64 = (p.p50 * eq11_e367);
        let eq11_e368_d_n0: f64 = (p.p50 * eq11_e367_d_n0);
        let eq11_e368_d_n1: f64 = (p.p50 * eq11_e367_d_n1);
        let eq11_e368_d_n2: f64 = (p.p50 * eq11_e367_d_n2);
        let eq11_e368_d_n3: f64 = (p.p50 * eq11_e367_d_n3);
        let eq11_e368_d_n4: f64 = (p.p50 * eq11_e367_d_n4);
        let eq11_e368_d_n5: f64 = (p.p50 * eq11_e367_d_n5);
        let eq11_e368_d_n6: f64 = (p.p50 * eq11_e367_d_n6);
        let eq11_e368_d_n7: f64 = (p.p50 * eq11_e367_d_n7);
        let eq11_e368_d_n8: f64 = (p.p50 * eq11_e367_d_n8);
        let eq11_e368_d_n9: f64 = (p.p50 * eq11_e367_d_n9);
        let eq11_e368_d_n10: f64 = (p.p50 * eq11_e367_d_n10);
        let eq11_e368_d_n11: f64 = (p.p50 * eq11_e367_d_n11);
        let eq11_e368_d_n12: f64 = (p.p50 * eq11_e367_d_n12);
        let eq11_e368_d_n13: f64 = (p.p50 * eq11_e367_d_n13);
        let eq11_e368_d_n14: f64 = (p.p50 * eq11_e367_d_n14);
        let eq11_e368_d_n15: f64 = (p.p50 * eq11_e367_d_n15);
        let eq11_e368_d_n16: f64 = (p.p50 * eq11_e367_d_n16);
        let eq11_e368_d_n17: f64 = (p.p50 * eq11_e367_d_n17);
        let eq11_e368_d_n18: f64 = (p.p50 * eq11_e367_d_n18);
        let eq11_e368_d_b0: f64 = (p.p50 * eq11_e367_d_b0);
        let eq11_e368_d_b1: f64 = (p.p50 * eq11_e367_d_b1);
        let eq11_e368_d_b2: f64 = (p.p50 * eq11_e367_d_b2);
        let eq11_e368_d_b3: f64 = (p.p50 * eq11_e367_d_b3);
        let eq11_e368_d_b4: f64 = (p.p50 * eq11_e367_d_b4);
        let eq11_e368_d_b5: f64 = (p.p50 * eq11_e367_d_b5);
        let eq11_e368_d_b6: f64 = (p.p50 * eq11_e367_d_b6);
        let eq11_e368_d_b7: f64 = (p.p50 * eq11_e367_d_b7);
        let eq11_e368_d_b8: f64 = (p.p50 * eq11_e367_d_b8);
        let eq11_e368_d_b9: f64 = (p.p50 * eq11_e367_d_b9);
        let eq11_e368_d_b10: f64 = (p.p50 * eq11_e367_d_b10);
        let eq11_e368_d_b11: f64 = (p.p50 * eq11_e367_d_b11);
        let eq11_e368_d_b12: f64 = (p.p50 * eq11_e367_d_b12);
        let eq11_value: f64 = eq11_e368;
        let eq11_node_derivatives: [f64; 19] = [eq11_e368_d_n0, eq11_e368_d_n1, eq11_e368_d_n2, eq11_e368_d_n3, eq11_e368_d_n4, eq11_e368_d_n5, eq11_e368_d_n6, eq11_e368_d_n7, eq11_e368_d_n8, eq11_e368_d_n9, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, eq11_e368_d_n14, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_branch_derivatives: [f64; 13] = [eq11_e368_d_b0, eq11_e368_d_b1, eq11_e368_d_b2, eq11_e368_d_b3, eq11_e368_d_b4, eq11_e368_d_b5, eq11_e368_d_b6, eq11_e368_d_b7, eq11_e368_d_b8, eq11_e368_d_b9, eq11_e368_d_b10, eq11_e368_d_b11, eq11_e368_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
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
        let eq12_e371: f64 = self.eval_ddt(1, s.v[198]);
        let eq12_e371_d_n0: f64 = self.ddt_jacobian(s.dn[198][0]);
        let eq12_e371_d_n1: f64 = self.ddt_jacobian(s.dn[198][1]);
        let eq12_e371_d_n2: f64 = self.ddt_jacobian(s.dn[198][2]);
        let eq12_e371_d_n3: f64 = self.ddt_jacobian(s.dn[198][3]);
        let eq12_e371_d_n4: f64 = self.ddt_jacobian(s.dn[198][4]);
        let eq12_e371_d_n5: f64 = self.ddt_jacobian(s.dn[198][5]);
        let eq12_e371_d_n6: f64 = self.ddt_jacobian(s.dn[198][6]);
        let eq12_e371_d_n7: f64 = self.ddt_jacobian(s.dn[198][7]);
        let eq12_e371_d_n8: f64 = self.ddt_jacobian(s.dn[198][8]);
        let eq12_e371_d_n9: f64 = self.ddt_jacobian(s.dn[198][9]);
        let eq12_e371_d_n10: f64 = self.ddt_jacobian(s.dn[198][10]);
        let eq12_e371_d_n11: f64 = self.ddt_jacobian(s.dn[198][11]);
        let eq12_e371_d_n12: f64 = self.ddt_jacobian(s.dn[198][12]);
        let eq12_e371_d_n13: f64 = self.ddt_jacobian(s.dn[198][13]);
        let eq12_e371_d_n14: f64 = self.ddt_jacobian(s.dn[198][14]);
        let eq12_e371_d_n15: f64 = self.ddt_jacobian(s.dn[198][15]);
        let eq12_e371_d_n16: f64 = self.ddt_jacobian(s.dn[198][16]);
        let eq12_e371_d_n17: f64 = self.ddt_jacobian(s.dn[198][17]);
        let eq12_e371_d_n18: f64 = self.ddt_jacobian(s.dn[198][18]);
        let eq12_e371_d_b0: f64 = self.ddt_jacobian(s.db[198][0]);
        let eq12_e371_d_b1: f64 = self.ddt_jacobian(s.db[198][1]);
        let eq12_e371_d_b2: f64 = self.ddt_jacobian(s.db[198][2]);
        let eq12_e371_d_b3: f64 = self.ddt_jacobian(s.db[198][3]);
        let eq12_e371_d_b4: f64 = self.ddt_jacobian(s.db[198][4]);
        let eq12_e371_d_b5: f64 = self.ddt_jacobian(s.db[198][5]);
        let eq12_e371_d_b6: f64 = self.ddt_jacobian(s.db[198][6]);
        let eq12_e371_d_b7: f64 = self.ddt_jacobian(s.db[198][7]);
        let eq12_e371_d_b8: f64 = self.ddt_jacobian(s.db[198][8]);
        let eq12_e371_d_b9: f64 = self.ddt_jacobian(s.db[198][9]);
        let eq12_e371_d_b10: f64 = self.ddt_jacobian(s.db[198][10]);
        let eq12_e371_d_b11: f64 = self.ddt_jacobian(s.db[198][11]);
        let eq12_e371_d_b12: f64 = self.ddt_jacobian(s.db[198][12]);
        let eq12_e372: f64 = (p.p50 * eq12_e371);
        let eq12_e372_d_n0: f64 = (p.p50 * eq12_e371_d_n0);
        let eq12_e372_d_n1: f64 = (p.p50 * eq12_e371_d_n1);
        let eq12_e372_d_n2: f64 = (p.p50 * eq12_e371_d_n2);
        let eq12_e372_d_n3: f64 = (p.p50 * eq12_e371_d_n3);
        let eq12_e372_d_n4: f64 = (p.p50 * eq12_e371_d_n4);
        let eq12_e372_d_n5: f64 = (p.p50 * eq12_e371_d_n5);
        let eq12_e372_d_n6: f64 = (p.p50 * eq12_e371_d_n6);
        let eq12_e372_d_n7: f64 = (p.p50 * eq12_e371_d_n7);
        let eq12_e372_d_n8: f64 = (p.p50 * eq12_e371_d_n8);
        let eq12_e372_d_n9: f64 = (p.p50 * eq12_e371_d_n9);
        let eq12_e372_d_n10: f64 = (p.p50 * eq12_e371_d_n10);
        let eq12_e372_d_n11: f64 = (p.p50 * eq12_e371_d_n11);
        let eq12_e372_d_n12: f64 = (p.p50 * eq12_e371_d_n12);
        let eq12_e372_d_n13: f64 = (p.p50 * eq12_e371_d_n13);
        let eq12_e372_d_n14: f64 = (p.p50 * eq12_e371_d_n14);
        let eq12_e372_d_n15: f64 = (p.p50 * eq12_e371_d_n15);
        let eq12_e372_d_n16: f64 = (p.p50 * eq12_e371_d_n16);
        let eq12_e372_d_n17: f64 = (p.p50 * eq12_e371_d_n17);
        let eq12_e372_d_n18: f64 = (p.p50 * eq12_e371_d_n18);
        let eq12_e372_d_b0: f64 = (p.p50 * eq12_e371_d_b0);
        let eq12_e372_d_b1: f64 = (p.p50 * eq12_e371_d_b1);
        let eq12_e372_d_b2: f64 = (p.p50 * eq12_e371_d_b2);
        let eq12_e372_d_b3: f64 = (p.p50 * eq12_e371_d_b3);
        let eq12_e372_d_b4: f64 = (p.p50 * eq12_e371_d_b4);
        let eq12_e372_d_b5: f64 = (p.p50 * eq12_e371_d_b5);
        let eq12_e372_d_b6: f64 = (p.p50 * eq12_e371_d_b6);
        let eq12_e372_d_b7: f64 = (p.p50 * eq12_e371_d_b7);
        let eq12_e372_d_b8: f64 = (p.p50 * eq12_e371_d_b8);
        let eq12_e372_d_b9: f64 = (p.p50 * eq12_e371_d_b9);
        let eq12_e372_d_b10: f64 = (p.p50 * eq12_e371_d_b10);
        let eq12_e372_d_b11: f64 = (p.p50 * eq12_e371_d_b11);
        let eq12_e372_d_b12: f64 = (p.p50 * eq12_e371_d_b12);
        let eq12_value: f64 = eq12_e372;
        let eq12_node_derivatives: [f64; 19] = [eq12_e372_d_n0, eq12_e372_d_n1, eq12_e372_d_n2, eq12_e372_d_n3, eq12_e372_d_n4, eq12_e372_d_n5, eq12_e372_d_n6, eq12_e372_d_n7, eq12_e372_d_n8, eq12_e372_d_n9, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, eq12_e372_d_n14, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_branch_derivatives: [f64; 13] = [eq12_e372_d_b0, eq12_e372_d_b1, eq12_e372_d_b2, eq12_e372_d_b3, eq12_e372_d_b4, eq12_e372_d_b5, eq12_e372_d_b6, eq12_e372_d_b7, eq12_e372_d_b8, eq12_e372_d_b9, eq12_e372_d_b10, eq12_e372_d_b11, eq12_e372_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq12_value),
            &nodes,
            &eq12_node_derivatives,
            &branches,
            &eq12_branch_derivatives,
            self.multiplicity,
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
        let eq13_e375: f64 = self.eval_ddt(2, s.v[196]);
        let eq13_e375_d_n0: f64 = self.ddt_jacobian(s.dn[196][0]);
        let eq13_e375_d_n1: f64 = self.ddt_jacobian(s.dn[196][1]);
        let eq13_e375_d_n2: f64 = self.ddt_jacobian(s.dn[196][2]);
        let eq13_e375_d_n3: f64 = self.ddt_jacobian(s.dn[196][3]);
        let eq13_e375_d_n4: f64 = self.ddt_jacobian(s.dn[196][4]);
        let eq13_e375_d_n5: f64 = self.ddt_jacobian(s.dn[196][5]);
        let eq13_e375_d_n6: f64 = self.ddt_jacobian(s.dn[196][6]);
        let eq13_e375_d_n7: f64 = self.ddt_jacobian(s.dn[196][7]);
        let eq13_e375_d_n8: f64 = self.ddt_jacobian(s.dn[196][8]);
        let eq13_e375_d_n9: f64 = self.ddt_jacobian(s.dn[196][9]);
        let eq13_e375_d_n10: f64 = self.ddt_jacobian(s.dn[196][10]);
        let eq13_e375_d_n11: f64 = self.ddt_jacobian(s.dn[196][11]);
        let eq13_e375_d_n12: f64 = self.ddt_jacobian(s.dn[196][12]);
        let eq13_e375_d_n13: f64 = self.ddt_jacobian(s.dn[196][13]);
        let eq13_e375_d_n14: f64 = self.ddt_jacobian(s.dn[196][14]);
        let eq13_e375_d_n15: f64 = self.ddt_jacobian(s.dn[196][15]);
        let eq13_e375_d_n16: f64 = self.ddt_jacobian(s.dn[196][16]);
        let eq13_e375_d_n17: f64 = self.ddt_jacobian(s.dn[196][17]);
        let eq13_e375_d_n18: f64 = self.ddt_jacobian(s.dn[196][18]);
        let eq13_e375_d_b0: f64 = self.ddt_jacobian(s.db[196][0]);
        let eq13_e375_d_b1: f64 = self.ddt_jacobian(s.db[196][1]);
        let eq13_e375_d_b2: f64 = self.ddt_jacobian(s.db[196][2]);
        let eq13_e375_d_b3: f64 = self.ddt_jacobian(s.db[196][3]);
        let eq13_e375_d_b4: f64 = self.ddt_jacobian(s.db[196][4]);
        let eq13_e375_d_b5: f64 = self.ddt_jacobian(s.db[196][5]);
        let eq13_e375_d_b6: f64 = self.ddt_jacobian(s.db[196][6]);
        let eq13_e375_d_b7: f64 = self.ddt_jacobian(s.db[196][7]);
        let eq13_e375_d_b8: f64 = self.ddt_jacobian(s.db[196][8]);
        let eq13_e375_d_b9: f64 = self.ddt_jacobian(s.db[196][9]);
        let eq13_e375_d_b10: f64 = self.ddt_jacobian(s.db[196][10]);
        let eq13_e375_d_b11: f64 = self.ddt_jacobian(s.db[196][11]);
        let eq13_e375_d_b12: f64 = self.ddt_jacobian(s.db[196][12]);
        let eq13_e376: f64 = (p.p50 * eq13_e375);
        let eq13_e376_d_n0: f64 = (p.p50 * eq13_e375_d_n0);
        let eq13_e376_d_n1: f64 = (p.p50 * eq13_e375_d_n1);
        let eq13_e376_d_n2: f64 = (p.p50 * eq13_e375_d_n2);
        let eq13_e376_d_n3: f64 = (p.p50 * eq13_e375_d_n3);
        let eq13_e376_d_n4: f64 = (p.p50 * eq13_e375_d_n4);
        let eq13_e376_d_n5: f64 = (p.p50 * eq13_e375_d_n5);
        let eq13_e376_d_n6: f64 = (p.p50 * eq13_e375_d_n6);
        let eq13_e376_d_n7: f64 = (p.p50 * eq13_e375_d_n7);
        let eq13_e376_d_n8: f64 = (p.p50 * eq13_e375_d_n8);
        let eq13_e376_d_n9: f64 = (p.p50 * eq13_e375_d_n9);
        let eq13_e376_d_n10: f64 = (p.p50 * eq13_e375_d_n10);
        let eq13_e376_d_n11: f64 = (p.p50 * eq13_e375_d_n11);
        let eq13_e376_d_n12: f64 = (p.p50 * eq13_e375_d_n12);
        let eq13_e376_d_n13: f64 = (p.p50 * eq13_e375_d_n13);
        let eq13_e376_d_n14: f64 = (p.p50 * eq13_e375_d_n14);
        let eq13_e376_d_n15: f64 = (p.p50 * eq13_e375_d_n15);
        let eq13_e376_d_n16: f64 = (p.p50 * eq13_e375_d_n16);
        let eq13_e376_d_n17: f64 = (p.p50 * eq13_e375_d_n17);
        let eq13_e376_d_n18: f64 = (p.p50 * eq13_e375_d_n18);
        let eq13_e376_d_b0: f64 = (p.p50 * eq13_e375_d_b0);
        let eq13_e376_d_b1: f64 = (p.p50 * eq13_e375_d_b1);
        let eq13_e376_d_b2: f64 = (p.p50 * eq13_e375_d_b2);
        let eq13_e376_d_b3: f64 = (p.p50 * eq13_e375_d_b3);
        let eq13_e376_d_b4: f64 = (p.p50 * eq13_e375_d_b4);
        let eq13_e376_d_b5: f64 = (p.p50 * eq13_e375_d_b5);
        let eq13_e376_d_b6: f64 = (p.p50 * eq13_e375_d_b6);
        let eq13_e376_d_b7: f64 = (p.p50 * eq13_e375_d_b7);
        let eq13_e376_d_b8: f64 = (p.p50 * eq13_e375_d_b8);
        let eq13_e376_d_b9: f64 = (p.p50 * eq13_e375_d_b9);
        let eq13_e376_d_b10: f64 = (p.p50 * eq13_e375_d_b10);
        let eq13_e376_d_b11: f64 = (p.p50 * eq13_e375_d_b11);
        let eq13_e376_d_b12: f64 = (p.p50 * eq13_e375_d_b12);
        let eq13_value: f64 = eq13_e376;
        let eq13_node_derivatives: [f64; 19] = [eq13_e376_d_n0, eq13_e376_d_n1, eq13_e376_d_n2, eq13_e376_d_n3, eq13_e376_d_n4, eq13_e376_d_n5, eq13_e376_d_n6, eq13_e376_d_n7, eq13_e376_d_n8, eq13_e376_d_n9, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, eq13_e376_d_n14, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_branch_derivatives: [f64; 13] = [eq13_e376_d_b0, eq13_e376_d_b1, eq13_e376_d_b2, eq13_e376_d_b3, eq13_e376_d_b4, eq13_e376_d_b5, eq13_e376_d_b6, eq13_e376_d_b7, eq13_e376_d_b8, eq13_e376_d_b9, eq13_e376_d_b10, eq13_e376_d_b11, eq13_e376_d_b12];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            self.multiplicity * (eq13_value),
            &nodes,
            &eq13_node_derivatives,
            &branches,
            &eq13_branch_derivatives,
            self.multiplicity,
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
        let eq14_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq14_value),
            &[
            ],
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq15_e387: f64 = (nv14 - 0.0);
        let eq15_value: f64 = eq15_e387;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq15_value),
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * 1.0),
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
        let eq16_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq16_value),
            &[
            ],
        );
    }
}

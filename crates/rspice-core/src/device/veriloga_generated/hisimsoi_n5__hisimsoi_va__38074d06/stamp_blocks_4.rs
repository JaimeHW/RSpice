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
        let (eq1_e313,) = {
    if (s.v[627] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e313;
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
        let eq2_e316: f64 = (p.p50 * s.v[199]);
        let eq2_e316_d_n0: f64 = (p.p50 * s.dn[199][0]);
        let eq2_e316_d_n1: f64 = (p.p50 * s.dn[199][1]);
        let eq2_e316_d_n2: f64 = (p.p50 * s.dn[199][2]);
        let eq2_e316_d_n3: f64 = (p.p50 * s.dn[199][3]);
        let eq2_e316_d_n4: f64 = (p.p50 * s.dn[199][4]);
        let eq2_e316_d_n5: f64 = (p.p50 * s.dn[199][5]);
        let eq2_e316_d_n6: f64 = (p.p50 * s.dn[199][6]);
        let eq2_e316_d_n7: f64 = (p.p50 * s.dn[199][7]);
        let eq2_e316_d_n8: f64 = (p.p50 * s.dn[199][8]);
        let eq2_e316_d_n9: f64 = (p.p50 * s.dn[199][9]);
        let eq2_e316_d_n10: f64 = (p.p50 * s.dn[199][10]);
        let eq2_e316_d_n11: f64 = (p.p50 * s.dn[199][11]);
        let eq2_e316_d_n12: f64 = (p.p50 * s.dn[199][12]);
        let eq2_e316_d_n13: f64 = (p.p50 * s.dn[199][13]);
        let eq2_e316_d_n14: f64 = (p.p50 * s.dn[199][14]);
        let eq2_e316_d_n15: f64 = (p.p50 * s.dn[199][15]);
        let eq2_e316_d_n16: f64 = (p.p50 * s.dn[199][16]);
        let eq2_e316_d_n17: f64 = (p.p50 * s.dn[199][17]);
        let eq2_e316_d_n18: f64 = (p.p50 * s.dn[199][18]);
        let eq2_e316_d_b0: f64 = (p.p50 * s.db[199][0]);
        let eq2_e316_d_b1: f64 = (p.p50 * s.db[199][1]);
        let eq2_e316_d_b2: f64 = (p.p50 * s.db[199][2]);
        let eq2_e316_d_b3: f64 = (p.p50 * s.db[199][3]);
        let eq2_e316_d_b4: f64 = (p.p50 * s.db[199][4]);
        let eq2_e316_d_b5: f64 = (p.p50 * s.db[199][5]);
        let eq2_e316_d_b6: f64 = (p.p50 * s.db[199][6]);
        let eq2_e316_d_b7: f64 = (p.p50 * s.db[199][7]);
        let eq2_e316_d_b8: f64 = (p.p50 * s.db[199][8]);
        let eq2_e316_d_b9: f64 = (p.p50 * s.db[199][9]);
        let eq2_e316_d_b10: f64 = (p.p50 * s.db[199][10]);
        let eq2_e316_d_b11: f64 = (p.p50 * s.db[199][11]);
        let eq2_value: f64 = eq2_e316;
        let eq2_node_derivatives: [f64; 19] = [eq2_e316_d_n0, eq2_e316_d_n1, eq2_e316_d_n2, eq2_e316_d_n3, eq2_e316_d_n4, eq2_e316_d_n5, eq2_e316_d_n6, eq2_e316_d_n7, eq2_e316_d_n8, eq2_e316_d_n9, eq2_e316_d_n10, eq2_e316_d_n11, eq2_e316_d_n12, eq2_e316_d_n13, eq2_e316_d_n14, eq2_e316_d_n15, eq2_e316_d_n16, eq2_e316_d_n17, eq2_e316_d_n18];
        let eq2_branch_derivatives: [f64; 12] = [eq2_e316_d_b0, eq2_e316_d_b1, eq2_e316_d_b2, eq2_e316_d_b3, eq2_e316_d_b4, eq2_e316_d_b5, eq2_e316_d_b6, eq2_e316_d_b7, eq2_e316_d_b8, eq2_e316_d_b9, eq2_e316_d_b10, eq2_e316_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq2_value),
            &nodes,
            &eq2_node_derivatives,
            &branches,
            &eq2_branch_derivatives,
            self.multiplicity,
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
        let (eq3_e322, eq3_e322_d_n0, eq3_e322_d_n1, eq3_e322_d_n2, eq3_e322_d_n3, eq3_e322_d_n4, eq3_e322_d_n5, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n8, eq3_e322_d_n9, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n13, eq3_e322_d_n14, eq3_e322_d_n15, eq3_e322_d_n16, eq3_e322_d_n17, eq3_e322_d_n18, eq3_e322_d_b0, eq3_e322_d_b1, eq3_e322_d_b2, eq3_e322_d_b3, eq3_e322_d_b4, eq3_e322_d_b5, eq3_e322_d_b6, eq3_e322_d_b7, eq3_e322_d_b8, eq3_e322_d_b9, eq3_e322_d_b10, eq3_e322_d_b11,) = {
    if (s.v[1846] != 0.0) {
        let eq3_e320: f64 = (p.p50 * s.v[306]);
        let eq3_e320_d_n0: f64 = (p.p50 * s.dn[306][0]);
        let eq3_e320_d_n1: f64 = (p.p50 * s.dn[306][1]);
        let eq3_e320_d_n2: f64 = (p.p50 * s.dn[306][2]);
        let eq3_e320_d_n3: f64 = (p.p50 * s.dn[306][3]);
        let eq3_e320_d_n4: f64 = (p.p50 * s.dn[306][4]);
        let eq3_e320_d_n5: f64 = (p.p50 * s.dn[306][5]);
        let eq3_e320_d_n6: f64 = (p.p50 * s.dn[306][6]);
        let eq3_e320_d_n7: f64 = (p.p50 * s.dn[306][7]);
        let eq3_e320_d_n8: f64 = (p.p50 * s.dn[306][8]);
        let eq3_e320_d_n9: f64 = (p.p50 * s.dn[306][9]);
        let eq3_e320_d_n10: f64 = (p.p50 * s.dn[306][10]);
        let eq3_e320_d_n11: f64 = (p.p50 * s.dn[306][11]);
        let eq3_e320_d_n12: f64 = (p.p50 * s.dn[306][12]);
        let eq3_e320_d_n13: f64 = (p.p50 * s.dn[306][13]);
        let eq3_e320_d_n14: f64 = (p.p50 * s.dn[306][14]);
        let eq3_e320_d_n15: f64 = (p.p50 * s.dn[306][15]);
        let eq3_e320_d_n16: f64 = (p.p50 * s.dn[306][16]);
        let eq3_e320_d_n17: f64 = (p.p50 * s.dn[306][17]);
        let eq3_e320_d_n18: f64 = (p.p50 * s.dn[306][18]);
        let eq3_e320_d_b0: f64 = (p.p50 * s.db[306][0]);
        let eq3_e320_d_b1: f64 = (p.p50 * s.db[306][1]);
        let eq3_e320_d_b2: f64 = (p.p50 * s.db[306][2]);
        let eq3_e320_d_b3: f64 = (p.p50 * s.db[306][3]);
        let eq3_e320_d_b4: f64 = (p.p50 * s.db[306][4]);
        let eq3_e320_d_b5: f64 = (p.p50 * s.db[306][5]);
        let eq3_e320_d_b6: f64 = (p.p50 * s.db[306][6]);
        let eq3_e320_d_b7: f64 = (p.p50 * s.db[306][7]);
        let eq3_e320_d_b8: f64 = (p.p50 * s.db[306][8]);
        let eq3_e320_d_b9: f64 = (p.p50 * s.db[306][9]);
        let eq3_e320_d_b10: f64 = (p.p50 * s.db[306][10]);
        let eq3_e320_d_b11: f64 = (p.p50 * s.db[306][11]);
        (eq3_e320, eq3_e320_d_n0, eq3_e320_d_n1, eq3_e320_d_n2, eq3_e320_d_n3, eq3_e320_d_n4, eq3_e320_d_n5, eq3_e320_d_n6, eq3_e320_d_n7, eq3_e320_d_n8, eq3_e320_d_n9, eq3_e320_d_n10, eq3_e320_d_n11, eq3_e320_d_n12, eq3_e320_d_n13, eq3_e320_d_n14, eq3_e320_d_n15, eq3_e320_d_n16, eq3_e320_d_n17, eq3_e320_d_n18, eq3_e320_d_b0, eq3_e320_d_b1, eq3_e320_d_b2, eq3_e320_d_b3, eq3_e320_d_b4, eq3_e320_d_b5, eq3_e320_d_b6, eq3_e320_d_b7, eq3_e320_d_b8, eq3_e320_d_b9, eq3_e320_d_b10, eq3_e320_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e322;
        let eq3_node_derivatives: [f64; 19] = [eq3_e322_d_n0, eq3_e322_d_n1, eq3_e322_d_n2, eq3_e322_d_n3, eq3_e322_d_n4, eq3_e322_d_n5, eq3_e322_d_n6, eq3_e322_d_n7, eq3_e322_d_n8, eq3_e322_d_n9, eq3_e322_d_n10, eq3_e322_d_n11, eq3_e322_d_n12, eq3_e322_d_n13, eq3_e322_d_n14, eq3_e322_d_n15, eq3_e322_d_n16, eq3_e322_d_n17, eq3_e322_d_n18];
        let eq3_branch_derivatives: [f64; 12] = [eq3_e322_d_b0, eq3_e322_d_b1, eq3_e322_d_b2, eq3_e322_d_b3, eq3_e322_d_b4, eq3_e322_d_b5, eq3_e322_d_b6, eq3_e322_d_b7, eq3_e322_d_b8, eq3_e322_d_b9, eq3_e322_d_b10, eq3_e322_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[11]),
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
        let (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18, eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11,) = {
    if (s.v[1846] != 0.0) {
        let eq4_e326: f64 = (p.p50 * s.v[307]);
        let eq4_e326_d_n0: f64 = (p.p50 * s.dn[307][0]);
        let eq4_e326_d_n1: f64 = (p.p50 * s.dn[307][1]);
        let eq4_e326_d_n2: f64 = (p.p50 * s.dn[307][2]);
        let eq4_e326_d_n3: f64 = (p.p50 * s.dn[307][3]);
        let eq4_e326_d_n4: f64 = (p.p50 * s.dn[307][4]);
        let eq4_e326_d_n5: f64 = (p.p50 * s.dn[307][5]);
        let eq4_e326_d_n6: f64 = (p.p50 * s.dn[307][6]);
        let eq4_e326_d_n7: f64 = (p.p50 * s.dn[307][7]);
        let eq4_e326_d_n8: f64 = (p.p50 * s.dn[307][8]);
        let eq4_e326_d_n9: f64 = (p.p50 * s.dn[307][9]);
        let eq4_e326_d_n10: f64 = (p.p50 * s.dn[307][10]);
        let eq4_e326_d_n11: f64 = (p.p50 * s.dn[307][11]);
        let eq4_e326_d_n12: f64 = (p.p50 * s.dn[307][12]);
        let eq4_e326_d_n13: f64 = (p.p50 * s.dn[307][13]);
        let eq4_e326_d_n14: f64 = (p.p50 * s.dn[307][14]);
        let eq4_e326_d_n15: f64 = (p.p50 * s.dn[307][15]);
        let eq4_e326_d_n16: f64 = (p.p50 * s.dn[307][16]);
        let eq4_e326_d_n17: f64 = (p.p50 * s.dn[307][17]);
        let eq4_e326_d_n18: f64 = (p.p50 * s.dn[307][18]);
        let eq4_e326_d_b0: f64 = (p.p50 * s.db[307][0]);
        let eq4_e326_d_b1: f64 = (p.p50 * s.db[307][1]);
        let eq4_e326_d_b2: f64 = (p.p50 * s.db[307][2]);
        let eq4_e326_d_b3: f64 = (p.p50 * s.db[307][3]);
        let eq4_e326_d_b4: f64 = (p.p50 * s.db[307][4]);
        let eq4_e326_d_b5: f64 = (p.p50 * s.db[307][5]);
        let eq4_e326_d_b6: f64 = (p.p50 * s.db[307][6]);
        let eq4_e326_d_b7: f64 = (p.p50 * s.db[307][7]);
        let eq4_e326_d_b8: f64 = (p.p50 * s.db[307][8]);
        let eq4_e326_d_b9: f64 = (p.p50 * s.db[307][9]);
        let eq4_e326_d_b10: f64 = (p.p50 * s.db[307][10]);
        let eq4_e326_d_b11: f64 = (p.p50 * s.db[307][11]);
        (eq4_e326, eq4_e326_d_n0, eq4_e326_d_n1, eq4_e326_d_n2, eq4_e326_d_n3, eq4_e326_d_n4, eq4_e326_d_n5, eq4_e326_d_n6, eq4_e326_d_n7, eq4_e326_d_n8, eq4_e326_d_n9, eq4_e326_d_n10, eq4_e326_d_n11, eq4_e326_d_n12, eq4_e326_d_n13, eq4_e326_d_n14, eq4_e326_d_n15, eq4_e326_d_n16, eq4_e326_d_n17, eq4_e326_d_n18, eq4_e326_d_b0, eq4_e326_d_b1, eq4_e326_d_b2, eq4_e326_d_b3, eq4_e326_d_b4, eq4_e326_d_b5, eq4_e326_d_b6, eq4_e326_d_b7, eq4_e326_d_b8, eq4_e326_d_b9, eq4_e326_d_b10, eq4_e326_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e328;
        let eq4_node_derivatives: [f64; 19] = [eq4_e328_d_n0, eq4_e328_d_n1, eq4_e328_d_n2, eq4_e328_d_n3, eq4_e328_d_n4, eq4_e328_d_n5, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n8, eq4_e328_d_n9, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n13, eq4_e328_d_n14, eq4_e328_d_n15, eq4_e328_d_n16, eq4_e328_d_n17, eq4_e328_d_n18];
        let eq4_branch_derivatives: [f64; 12] = [eq4_e328_d_b0, eq4_e328_d_b1, eq4_e328_d_b2, eq4_e328_d_b3, eq4_e328_d_b4, eq4_e328_d_b5, eq4_e328_d_b6, eq4_e328_d_b7, eq4_e328_d_b8, eq4_e328_d_b9, eq4_e328_d_b10, eq4_e328_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
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
        let (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18, eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11,) = {
    if (s.v[1846] != 0.0) {
        let eq5_e332: f64 = (p.p50 * s.v[308]);
        let eq5_e332_d_n0: f64 = (p.p50 * s.dn[308][0]);
        let eq5_e332_d_n1: f64 = (p.p50 * s.dn[308][1]);
        let eq5_e332_d_n2: f64 = (p.p50 * s.dn[308][2]);
        let eq5_e332_d_n3: f64 = (p.p50 * s.dn[308][3]);
        let eq5_e332_d_n4: f64 = (p.p50 * s.dn[308][4]);
        let eq5_e332_d_n5: f64 = (p.p50 * s.dn[308][5]);
        let eq5_e332_d_n6: f64 = (p.p50 * s.dn[308][6]);
        let eq5_e332_d_n7: f64 = (p.p50 * s.dn[308][7]);
        let eq5_e332_d_n8: f64 = (p.p50 * s.dn[308][8]);
        let eq5_e332_d_n9: f64 = (p.p50 * s.dn[308][9]);
        let eq5_e332_d_n10: f64 = (p.p50 * s.dn[308][10]);
        let eq5_e332_d_n11: f64 = (p.p50 * s.dn[308][11]);
        let eq5_e332_d_n12: f64 = (p.p50 * s.dn[308][12]);
        let eq5_e332_d_n13: f64 = (p.p50 * s.dn[308][13]);
        let eq5_e332_d_n14: f64 = (p.p50 * s.dn[308][14]);
        let eq5_e332_d_n15: f64 = (p.p50 * s.dn[308][15]);
        let eq5_e332_d_n16: f64 = (p.p50 * s.dn[308][16]);
        let eq5_e332_d_n17: f64 = (p.p50 * s.dn[308][17]);
        let eq5_e332_d_n18: f64 = (p.p50 * s.dn[308][18]);
        let eq5_e332_d_b0: f64 = (p.p50 * s.db[308][0]);
        let eq5_e332_d_b1: f64 = (p.p50 * s.db[308][1]);
        let eq5_e332_d_b2: f64 = (p.p50 * s.db[308][2]);
        let eq5_e332_d_b3: f64 = (p.p50 * s.db[308][3]);
        let eq5_e332_d_b4: f64 = (p.p50 * s.db[308][4]);
        let eq5_e332_d_b5: f64 = (p.p50 * s.db[308][5]);
        let eq5_e332_d_b6: f64 = (p.p50 * s.db[308][6]);
        let eq5_e332_d_b7: f64 = (p.p50 * s.db[308][7]);
        let eq5_e332_d_b8: f64 = (p.p50 * s.db[308][8]);
        let eq5_e332_d_b9: f64 = (p.p50 * s.db[308][9]);
        let eq5_e332_d_b10: f64 = (p.p50 * s.db[308][10]);
        let eq5_e332_d_b11: f64 = (p.p50 * s.db[308][11]);
        (eq5_e332, eq5_e332_d_n0, eq5_e332_d_n1, eq5_e332_d_n2, eq5_e332_d_n3, eq5_e332_d_n4, eq5_e332_d_n5, eq5_e332_d_n6, eq5_e332_d_n7, eq5_e332_d_n8, eq5_e332_d_n9, eq5_e332_d_n10, eq5_e332_d_n11, eq5_e332_d_n12, eq5_e332_d_n13, eq5_e332_d_n14, eq5_e332_d_n15, eq5_e332_d_n16, eq5_e332_d_n17, eq5_e332_d_n18, eq5_e332_d_b0, eq5_e332_d_b1, eq5_e332_d_b2, eq5_e332_d_b3, eq5_e332_d_b4, eq5_e332_d_b5, eq5_e332_d_b6, eq5_e332_d_b7, eq5_e332_d_b8, eq5_e332_d_b9, eq5_e332_d_b10, eq5_e332_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e334;
        let eq5_node_derivatives: [f64; 19] = [eq5_e334_d_n0, eq5_e334_d_n1, eq5_e334_d_n2, eq5_e334_d_n3, eq5_e334_d_n4, eq5_e334_d_n5, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n8, eq5_e334_d_n9, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n13, eq5_e334_d_n14, eq5_e334_d_n15, eq5_e334_d_n16, eq5_e334_d_n17, eq5_e334_d_n18];
        let eq5_branch_derivatives: [f64; 12] = [eq5_e334_d_b0, eq5_e334_d_b1, eq5_e334_d_b2, eq5_e334_d_b3, eq5_e334_d_b4, eq5_e334_d_b5, eq5_e334_d_b6, eq5_e334_d_b7, eq5_e334_d_b8, eq5_e334_d_b9, eq5_e334_d_b10, eq5_e334_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[12]),
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18, eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11,) = {
    if (p.p259 != 0.0) {
        let eq6_e338: f64 = ((nv7 - nv2) / s.v[1]);
        let eq6_e338_d_n0: f64 = (-(((nv7 - nv2) * s.dn[1][0]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n1: f64 = (-(((nv7 - nv2) * s.dn[1][1]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n2: f64 = (((-s.v[1]) - ((nv7 - nv2) * s.dn[1][2])) / (s.v[1] * s.v[1]));
        let eq6_e338_d_n3: f64 = (-(((nv7 - nv2) * s.dn[1][3]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n4: f64 = (-(((nv7 - nv2) * s.dn[1][4]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n5: f64 = (-(((nv7 - nv2) * s.dn[1][5]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n6: f64 = (-(((nv7 - nv2) * s.dn[1][6]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n7: f64 = ((s.v[1] - ((nv7 - nv2) * s.dn[1][7])) / (s.v[1] * s.v[1]));
        let eq6_e338_d_n8: f64 = (-(((nv7 - nv2) * s.dn[1][8]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n9: f64 = (-(((nv7 - nv2) * s.dn[1][9]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n10: f64 = (-(((nv7 - nv2) * s.dn[1][10]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n11: f64 = (-(((nv7 - nv2) * s.dn[1][11]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n12: f64 = (-(((nv7 - nv2) * s.dn[1][12]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n13: f64 = (-(((nv7 - nv2) * s.dn[1][13]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n14: f64 = (-(((nv7 - nv2) * s.dn[1][14]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n15: f64 = (-(((nv7 - nv2) * s.dn[1][15]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n16: f64 = (-(((nv7 - nv2) * s.dn[1][16]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n17: f64 = (-(((nv7 - nv2) * s.dn[1][17]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_n18: f64 = (-(((nv7 - nv2) * s.dn[1][18]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b0: f64 = (-(((nv7 - nv2) * s.db[1][0]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b1: f64 = (-(((nv7 - nv2) * s.db[1][1]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b2: f64 = (-(((nv7 - nv2) * s.db[1][2]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b3: f64 = (-(((nv7 - nv2) * s.db[1][3]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b4: f64 = (-(((nv7 - nv2) * s.db[1][4]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b5: f64 = (-(((nv7 - nv2) * s.db[1][5]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b6: f64 = (-(((nv7 - nv2) * s.db[1][6]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b7: f64 = (-(((nv7 - nv2) * s.db[1][7]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b8: f64 = (-(((nv7 - nv2) * s.db[1][8]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b9: f64 = (-(((nv7 - nv2) * s.db[1][9]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b10: f64 = (-(((nv7 - nv2) * s.db[1][10]) / (s.v[1] * s.v[1])));
        let eq6_e338_d_b11: f64 = (-(((nv7 - nv2) * s.db[1][11]) / (s.v[1] * s.v[1])));
        (eq6_e338, eq6_e338_d_n0, eq6_e338_d_n1, eq6_e338_d_n2, eq6_e338_d_n3, eq6_e338_d_n4, eq6_e338_d_n5, eq6_e338_d_n6, eq6_e338_d_n7, eq6_e338_d_n8, eq6_e338_d_n9, eq6_e338_d_n10, eq6_e338_d_n11, eq6_e338_d_n12, eq6_e338_d_n13, eq6_e338_d_n14, eq6_e338_d_n15, eq6_e338_d_n16, eq6_e338_d_n17, eq6_e338_d_n18, eq6_e338_d_b0, eq6_e338_d_b1, eq6_e338_d_b2, eq6_e338_d_b3, eq6_e338_d_b4, eq6_e338_d_b5, eq6_e338_d_b6, eq6_e338_d_b7, eq6_e338_d_b8, eq6_e338_d_b9, eq6_e338_d_b10, eq6_e338_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e340;
        let eq6_node_derivatives: [f64; 19] = [eq6_e340_d_n0, eq6_e340_d_n1, eq6_e340_d_n2, eq6_e340_d_n3, eq6_e340_d_n4, eq6_e340_d_n5, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n8, eq6_e340_d_n9, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n13, eq6_e340_d_n14, eq6_e340_d_n15, eq6_e340_d_n16, eq6_e340_d_n17, eq6_e340_d_n18];
        let eq6_branch_derivatives: [f64; 12] = [eq6_e340_d_b0, eq6_e340_d_b1, eq6_e340_d_b2, eq6_e340_d_b3, eq6_e340_d_b4, eq6_e340_d_b5, eq6_e340_d_b6, eq6_e340_d_b7, eq6_e340_d_b8, eq6_e340_d_b9, eq6_e340_d_b10, eq6_e340_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[2]),
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
        let (eq7_e345,) = {
    if (!(p.p259 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq7_value: f64 = eq7_e345;
        stamper.stamp_potential(
            branches[2],
            eq7_value,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq8_e351, eq8_e351_d_n0, eq8_e351_d_n1, eq8_e351_d_n2, eq8_e351_d_n3, eq8_e351_d_n4, eq8_e351_d_n5, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n8, eq8_e351_d_n9, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n13, eq8_e351_d_n14, eq8_e351_d_n15, eq8_e351_d_n16, eq8_e351_d_n17, eq8_e351_d_n18, eq8_e351_d_b0, eq8_e351_d_b1, eq8_e351_d_b2, eq8_e351_d_b3, eq8_e351_d_b4, eq8_e351_d_b5, eq8_e351_d_b6, eq8_e351_d_b7, eq8_e351_d_b8, eq8_e351_d_b9, eq8_e351_d_b10, eq8_e351_d_b11,) = {
    if (p.p260 != 0.0) {
        let eq8_e349: f64 = ((nv0 - nv6) / s.v[0]);
        let eq8_e349_d_n0: f64 = ((s.v[0] - ((nv0 - nv6) * s.dn[0][0])) / (s.v[0] * s.v[0]));
        let eq8_e349_d_n1: f64 = (-(((nv0 - nv6) * s.dn[0][1]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n2: f64 = (-(((nv0 - nv6) * s.dn[0][2]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n3: f64 = (-(((nv0 - nv6) * s.dn[0][3]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n4: f64 = (-(((nv0 - nv6) * s.dn[0][4]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n5: f64 = (-(((nv0 - nv6) * s.dn[0][5]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n6: f64 = (((-s.v[0]) - ((nv0 - nv6) * s.dn[0][6])) / (s.v[0] * s.v[0]));
        let eq8_e349_d_n7: f64 = (-(((nv0 - nv6) * s.dn[0][7]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n8: f64 = (-(((nv0 - nv6) * s.dn[0][8]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n9: f64 = (-(((nv0 - nv6) * s.dn[0][9]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n10: f64 = (-(((nv0 - nv6) * s.dn[0][10]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n11: f64 = (-(((nv0 - nv6) * s.dn[0][11]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n12: f64 = (-(((nv0 - nv6) * s.dn[0][12]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n13: f64 = (-(((nv0 - nv6) * s.dn[0][13]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n14: f64 = (-(((nv0 - nv6) * s.dn[0][14]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n15: f64 = (-(((nv0 - nv6) * s.dn[0][15]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n16: f64 = (-(((nv0 - nv6) * s.dn[0][16]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n17: f64 = (-(((nv0 - nv6) * s.dn[0][17]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_n18: f64 = (-(((nv0 - nv6) * s.dn[0][18]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b0: f64 = (-(((nv0 - nv6) * s.db[0][0]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b1: f64 = (-(((nv0 - nv6) * s.db[0][1]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b2: f64 = (-(((nv0 - nv6) * s.db[0][2]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b3: f64 = (-(((nv0 - nv6) * s.db[0][3]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b4: f64 = (-(((nv0 - nv6) * s.db[0][4]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b5: f64 = (-(((nv0 - nv6) * s.db[0][5]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b6: f64 = (-(((nv0 - nv6) * s.db[0][6]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b7: f64 = (-(((nv0 - nv6) * s.db[0][7]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b8: f64 = (-(((nv0 - nv6) * s.db[0][8]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b9: f64 = (-(((nv0 - nv6) * s.db[0][9]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b10: f64 = (-(((nv0 - nv6) * s.db[0][10]) / (s.v[0] * s.v[0])));
        let eq8_e349_d_b11: f64 = (-(((nv0 - nv6) * s.db[0][11]) / (s.v[0] * s.v[0])));
        (eq8_e349, eq8_e349_d_n0, eq8_e349_d_n1, eq8_e349_d_n2, eq8_e349_d_n3, eq8_e349_d_n4, eq8_e349_d_n5, eq8_e349_d_n6, eq8_e349_d_n7, eq8_e349_d_n8, eq8_e349_d_n9, eq8_e349_d_n10, eq8_e349_d_n11, eq8_e349_d_n12, eq8_e349_d_n13, eq8_e349_d_n14, eq8_e349_d_n15, eq8_e349_d_n16, eq8_e349_d_n17, eq8_e349_d_n18, eq8_e349_d_b0, eq8_e349_d_b1, eq8_e349_d_b2, eq8_e349_d_b3, eq8_e349_d_b4, eq8_e349_d_b5, eq8_e349_d_b6, eq8_e349_d_b7, eq8_e349_d_b8, eq8_e349_d_b9, eq8_e349_d_b10, eq8_e349_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e351;
        let eq8_node_derivatives: [f64; 19] = [eq8_e351_d_n0, eq8_e351_d_n1, eq8_e351_d_n2, eq8_e351_d_n3, eq8_e351_d_n4, eq8_e351_d_n5, eq8_e351_d_n6, eq8_e351_d_n7, eq8_e351_d_n8, eq8_e351_d_n9, eq8_e351_d_n10, eq8_e351_d_n11, eq8_e351_d_n12, eq8_e351_d_n13, eq8_e351_d_n14, eq8_e351_d_n15, eq8_e351_d_n16, eq8_e351_d_n17, eq8_e351_d_n18];
        let eq8_branch_derivatives: [f64; 12] = [eq8_e351_d_b0, eq8_e351_d_b1, eq8_e351_d_b2, eq8_e351_d_b3, eq8_e351_d_b4, eq8_e351_d_b5, eq8_e351_d_b6, eq8_e351_d_b7, eq8_e351_d_b8, eq8_e351_d_b9, eq8_e351_d_b10, eq8_e351_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq8_value),
            &nodes,
            &eq8_node_derivatives,
            &branches,
            &eq8_branch_derivatives,
            self.multiplicity,
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
        let (eq9_e356,) = {
    if (!(p.p260 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e356;
        stamper.stamp_potential(
            branches[3],
            eq9_value,
            &[
            ],
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
        let eq10_e359: f64 = self.eval_ddt(0, s.v[594]);
        let eq10_e359_d_n0: f64 = self.ddt_jacobian(s.dn[594][0]);
        let eq10_e359_d_n1: f64 = self.ddt_jacobian(s.dn[594][1]);
        let eq10_e359_d_n2: f64 = self.ddt_jacobian(s.dn[594][2]);
        let eq10_e359_d_n3: f64 = self.ddt_jacobian(s.dn[594][3]);
        let eq10_e359_d_n4: f64 = self.ddt_jacobian(s.dn[594][4]);
        let eq10_e359_d_n5: f64 = self.ddt_jacobian(s.dn[594][5]);
        let eq10_e359_d_n6: f64 = self.ddt_jacobian(s.dn[594][6]);
        let eq10_e359_d_n7: f64 = self.ddt_jacobian(s.dn[594][7]);
        let eq10_e359_d_n8: f64 = self.ddt_jacobian(s.dn[594][8]);
        let eq10_e359_d_n9: f64 = self.ddt_jacobian(s.dn[594][9]);
        let eq10_e359_d_n10: f64 = self.ddt_jacobian(s.dn[594][10]);
        let eq10_e359_d_n11: f64 = self.ddt_jacobian(s.dn[594][11]);
        let eq10_e359_d_n12: f64 = self.ddt_jacobian(s.dn[594][12]);
        let eq10_e359_d_n13: f64 = self.ddt_jacobian(s.dn[594][13]);
        let eq10_e359_d_n14: f64 = self.ddt_jacobian(s.dn[594][14]);
        let eq10_e359_d_n15: f64 = self.ddt_jacobian(s.dn[594][15]);
        let eq10_e359_d_n16: f64 = self.ddt_jacobian(s.dn[594][16]);
        let eq10_e359_d_n17: f64 = self.ddt_jacobian(s.dn[594][17]);
        let eq10_e359_d_n18: f64 = self.ddt_jacobian(s.dn[594][18]);
        let eq10_e359_d_b0: f64 = self.ddt_jacobian(s.db[594][0]);
        let eq10_e359_d_b1: f64 = self.ddt_jacobian(s.db[594][1]);
        let eq10_e359_d_b2: f64 = self.ddt_jacobian(s.db[594][2]);
        let eq10_e359_d_b3: f64 = self.ddt_jacobian(s.db[594][3]);
        let eq10_e359_d_b4: f64 = self.ddt_jacobian(s.db[594][4]);
        let eq10_e359_d_b5: f64 = self.ddt_jacobian(s.db[594][5]);
        let eq10_e359_d_b6: f64 = self.ddt_jacobian(s.db[594][6]);
        let eq10_e359_d_b7: f64 = self.ddt_jacobian(s.db[594][7]);
        let eq10_e359_d_b8: f64 = self.ddt_jacobian(s.db[594][8]);
        let eq10_e359_d_b9: f64 = self.ddt_jacobian(s.db[594][9]);
        let eq10_e359_d_b10: f64 = self.ddt_jacobian(s.db[594][10]);
        let eq10_e359_d_b11: f64 = self.ddt_jacobian(s.db[594][11]);
        let eq10_e360: f64 = (p.p50 * eq10_e359);
        let eq10_e360_d_n0: f64 = (p.p50 * eq10_e359_d_n0);
        let eq10_e360_d_n1: f64 = (p.p50 * eq10_e359_d_n1);
        let eq10_e360_d_n2: f64 = (p.p50 * eq10_e359_d_n2);
        let eq10_e360_d_n3: f64 = (p.p50 * eq10_e359_d_n3);
        let eq10_e360_d_n4: f64 = (p.p50 * eq10_e359_d_n4);
        let eq10_e360_d_n5: f64 = (p.p50 * eq10_e359_d_n5);
        let eq10_e360_d_n6: f64 = (p.p50 * eq10_e359_d_n6);
        let eq10_e360_d_n7: f64 = (p.p50 * eq10_e359_d_n7);
        let eq10_e360_d_n8: f64 = (p.p50 * eq10_e359_d_n8);
        let eq10_e360_d_n9: f64 = (p.p50 * eq10_e359_d_n9);
        let eq10_e360_d_n10: f64 = (p.p50 * eq10_e359_d_n10);
        let eq10_e360_d_n11: f64 = (p.p50 * eq10_e359_d_n11);
        let eq10_e360_d_n12: f64 = (p.p50 * eq10_e359_d_n12);
        let eq10_e360_d_n13: f64 = (p.p50 * eq10_e359_d_n13);
        let eq10_e360_d_n14: f64 = (p.p50 * eq10_e359_d_n14);
        let eq10_e360_d_n15: f64 = (p.p50 * eq10_e359_d_n15);
        let eq10_e360_d_n16: f64 = (p.p50 * eq10_e359_d_n16);
        let eq10_e360_d_n17: f64 = (p.p50 * eq10_e359_d_n17);
        let eq10_e360_d_n18: f64 = (p.p50 * eq10_e359_d_n18);
        let eq10_e360_d_b0: f64 = (p.p50 * eq10_e359_d_b0);
        let eq10_e360_d_b1: f64 = (p.p50 * eq10_e359_d_b1);
        let eq10_e360_d_b2: f64 = (p.p50 * eq10_e359_d_b2);
        let eq10_e360_d_b3: f64 = (p.p50 * eq10_e359_d_b3);
        let eq10_e360_d_b4: f64 = (p.p50 * eq10_e359_d_b4);
        let eq10_e360_d_b5: f64 = (p.p50 * eq10_e359_d_b5);
        let eq10_e360_d_b6: f64 = (p.p50 * eq10_e359_d_b6);
        let eq10_e360_d_b7: f64 = (p.p50 * eq10_e359_d_b7);
        let eq10_e360_d_b8: f64 = (p.p50 * eq10_e359_d_b8);
        let eq10_e360_d_b9: f64 = (p.p50 * eq10_e359_d_b9);
        let eq10_e360_d_b10: f64 = (p.p50 * eq10_e359_d_b10);
        let eq10_e360_d_b11: f64 = (p.p50 * eq10_e359_d_b11);
        let eq10_value: f64 = eq10_e360;
        let eq10_node_derivatives: [f64; 19] = [eq10_e360_d_n0, eq10_e360_d_n1, eq10_e360_d_n2, eq10_e360_d_n3, eq10_e360_d_n4, eq10_e360_d_n5, eq10_e360_d_n6, eq10_e360_d_n7, eq10_e360_d_n8, eq10_e360_d_n9, eq10_e360_d_n10, eq10_e360_d_n11, eq10_e360_d_n12, eq10_e360_d_n13, eq10_e360_d_n14, eq10_e360_d_n15, eq10_e360_d_n16, eq10_e360_d_n17, eq10_e360_d_n18];
        let eq10_branch_derivatives: [f64; 12] = [eq10_e360_d_b0, eq10_e360_d_b1, eq10_e360_d_b2, eq10_e360_d_b3, eq10_e360_d_b4, eq10_e360_d_b5, eq10_e360_d_b6, eq10_e360_d_b7, eq10_e360_d_b8, eq10_e360_d_b9, eq10_e360_d_b10, eq10_e360_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[7]),
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
        let eq11_e363: f64 = self.eval_ddt(1, s.v[198]);
        let eq11_e363_d_n0: f64 = self.ddt_jacobian(s.dn[198][0]);
        let eq11_e363_d_n1: f64 = self.ddt_jacobian(s.dn[198][1]);
        let eq11_e363_d_n2: f64 = self.ddt_jacobian(s.dn[198][2]);
        let eq11_e363_d_n3: f64 = self.ddt_jacobian(s.dn[198][3]);
        let eq11_e363_d_n4: f64 = self.ddt_jacobian(s.dn[198][4]);
        let eq11_e363_d_n5: f64 = self.ddt_jacobian(s.dn[198][5]);
        let eq11_e363_d_n6: f64 = self.ddt_jacobian(s.dn[198][6]);
        let eq11_e363_d_n7: f64 = self.ddt_jacobian(s.dn[198][7]);
        let eq11_e363_d_n8: f64 = self.ddt_jacobian(s.dn[198][8]);
        let eq11_e363_d_n9: f64 = self.ddt_jacobian(s.dn[198][9]);
        let eq11_e363_d_n10: f64 = self.ddt_jacobian(s.dn[198][10]);
        let eq11_e363_d_n11: f64 = self.ddt_jacobian(s.dn[198][11]);
        let eq11_e363_d_n12: f64 = self.ddt_jacobian(s.dn[198][12]);
        let eq11_e363_d_n13: f64 = self.ddt_jacobian(s.dn[198][13]);
        let eq11_e363_d_n14: f64 = self.ddt_jacobian(s.dn[198][14]);
        let eq11_e363_d_n15: f64 = self.ddt_jacobian(s.dn[198][15]);
        let eq11_e363_d_n16: f64 = self.ddt_jacobian(s.dn[198][16]);
        let eq11_e363_d_n17: f64 = self.ddt_jacobian(s.dn[198][17]);
        let eq11_e363_d_n18: f64 = self.ddt_jacobian(s.dn[198][18]);
        let eq11_e363_d_b0: f64 = self.ddt_jacobian(s.db[198][0]);
        let eq11_e363_d_b1: f64 = self.ddt_jacobian(s.db[198][1]);
        let eq11_e363_d_b2: f64 = self.ddt_jacobian(s.db[198][2]);
        let eq11_e363_d_b3: f64 = self.ddt_jacobian(s.db[198][3]);
        let eq11_e363_d_b4: f64 = self.ddt_jacobian(s.db[198][4]);
        let eq11_e363_d_b5: f64 = self.ddt_jacobian(s.db[198][5]);
        let eq11_e363_d_b6: f64 = self.ddt_jacobian(s.db[198][6]);
        let eq11_e363_d_b7: f64 = self.ddt_jacobian(s.db[198][7]);
        let eq11_e363_d_b8: f64 = self.ddt_jacobian(s.db[198][8]);
        let eq11_e363_d_b9: f64 = self.ddt_jacobian(s.db[198][9]);
        let eq11_e363_d_b10: f64 = self.ddt_jacobian(s.db[198][10]);
        let eq11_e363_d_b11: f64 = self.ddt_jacobian(s.db[198][11]);
        let eq11_e364: f64 = (p.p50 * eq11_e363);
        let eq11_e364_d_n0: f64 = (p.p50 * eq11_e363_d_n0);
        let eq11_e364_d_n1: f64 = (p.p50 * eq11_e363_d_n1);
        let eq11_e364_d_n2: f64 = (p.p50 * eq11_e363_d_n2);
        let eq11_e364_d_n3: f64 = (p.p50 * eq11_e363_d_n3);
        let eq11_e364_d_n4: f64 = (p.p50 * eq11_e363_d_n4);
        let eq11_e364_d_n5: f64 = (p.p50 * eq11_e363_d_n5);
        let eq11_e364_d_n6: f64 = (p.p50 * eq11_e363_d_n6);
        let eq11_e364_d_n7: f64 = (p.p50 * eq11_e363_d_n7);
        let eq11_e364_d_n8: f64 = (p.p50 * eq11_e363_d_n8);
        let eq11_e364_d_n9: f64 = (p.p50 * eq11_e363_d_n9);
        let eq11_e364_d_n10: f64 = (p.p50 * eq11_e363_d_n10);
        let eq11_e364_d_n11: f64 = (p.p50 * eq11_e363_d_n11);
        let eq11_e364_d_n12: f64 = (p.p50 * eq11_e363_d_n12);
        let eq11_e364_d_n13: f64 = (p.p50 * eq11_e363_d_n13);
        let eq11_e364_d_n14: f64 = (p.p50 * eq11_e363_d_n14);
        let eq11_e364_d_n15: f64 = (p.p50 * eq11_e363_d_n15);
        let eq11_e364_d_n16: f64 = (p.p50 * eq11_e363_d_n16);
        let eq11_e364_d_n17: f64 = (p.p50 * eq11_e363_d_n17);
        let eq11_e364_d_n18: f64 = (p.p50 * eq11_e363_d_n18);
        let eq11_e364_d_b0: f64 = (p.p50 * eq11_e363_d_b0);
        let eq11_e364_d_b1: f64 = (p.p50 * eq11_e363_d_b1);
        let eq11_e364_d_b2: f64 = (p.p50 * eq11_e363_d_b2);
        let eq11_e364_d_b3: f64 = (p.p50 * eq11_e363_d_b3);
        let eq11_e364_d_b4: f64 = (p.p50 * eq11_e363_d_b4);
        let eq11_e364_d_b5: f64 = (p.p50 * eq11_e363_d_b5);
        let eq11_e364_d_b6: f64 = (p.p50 * eq11_e363_d_b6);
        let eq11_e364_d_b7: f64 = (p.p50 * eq11_e363_d_b7);
        let eq11_e364_d_b8: f64 = (p.p50 * eq11_e363_d_b8);
        let eq11_e364_d_b9: f64 = (p.p50 * eq11_e363_d_b9);
        let eq11_e364_d_b10: f64 = (p.p50 * eq11_e363_d_b10);
        let eq11_e364_d_b11: f64 = (p.p50 * eq11_e363_d_b11);
        let eq11_value: f64 = eq11_e364;
        let eq11_node_derivatives: [f64; 19] = [eq11_e364_d_n0, eq11_e364_d_n1, eq11_e364_d_n2, eq11_e364_d_n3, eq11_e364_d_n4, eq11_e364_d_n5, eq11_e364_d_n6, eq11_e364_d_n7, eq11_e364_d_n8, eq11_e364_d_n9, eq11_e364_d_n10, eq11_e364_d_n11, eq11_e364_d_n12, eq11_e364_d_n13, eq11_e364_d_n14, eq11_e364_d_n15, eq11_e364_d_n16, eq11_e364_d_n17, eq11_e364_d_n18];
        let eq11_branch_derivatives: [f64; 12] = [eq11_e364_d_b0, eq11_e364_d_b1, eq11_e364_d_b2, eq11_e364_d_b3, eq11_e364_d_b4, eq11_e364_d_b5, eq11_e364_d_b6, eq11_e364_d_b7, eq11_e364_d_b8, eq11_e364_d_b9, eq11_e364_d_b10, eq11_e364_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let eq12_e367: f64 = self.eval_ddt(2, s.v[196]);
        let eq12_e367_d_n0: f64 = self.ddt_jacobian(s.dn[196][0]);
        let eq12_e367_d_n1: f64 = self.ddt_jacobian(s.dn[196][1]);
        let eq12_e367_d_n2: f64 = self.ddt_jacobian(s.dn[196][2]);
        let eq12_e367_d_n3: f64 = self.ddt_jacobian(s.dn[196][3]);
        let eq12_e367_d_n4: f64 = self.ddt_jacobian(s.dn[196][4]);
        let eq12_e367_d_n5: f64 = self.ddt_jacobian(s.dn[196][5]);
        let eq12_e367_d_n6: f64 = self.ddt_jacobian(s.dn[196][6]);
        let eq12_e367_d_n7: f64 = self.ddt_jacobian(s.dn[196][7]);
        let eq12_e367_d_n8: f64 = self.ddt_jacobian(s.dn[196][8]);
        let eq12_e367_d_n9: f64 = self.ddt_jacobian(s.dn[196][9]);
        let eq12_e367_d_n10: f64 = self.ddt_jacobian(s.dn[196][10]);
        let eq12_e367_d_n11: f64 = self.ddt_jacobian(s.dn[196][11]);
        let eq12_e367_d_n12: f64 = self.ddt_jacobian(s.dn[196][12]);
        let eq12_e367_d_n13: f64 = self.ddt_jacobian(s.dn[196][13]);
        let eq12_e367_d_n14: f64 = self.ddt_jacobian(s.dn[196][14]);
        let eq12_e367_d_n15: f64 = self.ddt_jacobian(s.dn[196][15]);
        let eq12_e367_d_n16: f64 = self.ddt_jacobian(s.dn[196][16]);
        let eq12_e367_d_n17: f64 = self.ddt_jacobian(s.dn[196][17]);
        let eq12_e367_d_n18: f64 = self.ddt_jacobian(s.dn[196][18]);
        let eq12_e367_d_b0: f64 = self.ddt_jacobian(s.db[196][0]);
        let eq12_e367_d_b1: f64 = self.ddt_jacobian(s.db[196][1]);
        let eq12_e367_d_b2: f64 = self.ddt_jacobian(s.db[196][2]);
        let eq12_e367_d_b3: f64 = self.ddt_jacobian(s.db[196][3]);
        let eq12_e367_d_b4: f64 = self.ddt_jacobian(s.db[196][4]);
        let eq12_e367_d_b5: f64 = self.ddt_jacobian(s.db[196][5]);
        let eq12_e367_d_b6: f64 = self.ddt_jacobian(s.db[196][6]);
        let eq12_e367_d_b7: f64 = self.ddt_jacobian(s.db[196][7]);
        let eq12_e367_d_b8: f64 = self.ddt_jacobian(s.db[196][8]);
        let eq12_e367_d_b9: f64 = self.ddt_jacobian(s.db[196][9]);
        let eq12_e367_d_b10: f64 = self.ddt_jacobian(s.db[196][10]);
        let eq12_e367_d_b11: f64 = self.ddt_jacobian(s.db[196][11]);
        let eq12_e368: f64 = (p.p50 * eq12_e367);
        let eq12_e368_d_n0: f64 = (p.p50 * eq12_e367_d_n0);
        let eq12_e368_d_n1: f64 = (p.p50 * eq12_e367_d_n1);
        let eq12_e368_d_n2: f64 = (p.p50 * eq12_e367_d_n2);
        let eq12_e368_d_n3: f64 = (p.p50 * eq12_e367_d_n3);
        let eq12_e368_d_n4: f64 = (p.p50 * eq12_e367_d_n4);
        let eq12_e368_d_n5: f64 = (p.p50 * eq12_e367_d_n5);
        let eq12_e368_d_n6: f64 = (p.p50 * eq12_e367_d_n6);
        let eq12_e368_d_n7: f64 = (p.p50 * eq12_e367_d_n7);
        let eq12_e368_d_n8: f64 = (p.p50 * eq12_e367_d_n8);
        let eq12_e368_d_n9: f64 = (p.p50 * eq12_e367_d_n9);
        let eq12_e368_d_n10: f64 = (p.p50 * eq12_e367_d_n10);
        let eq12_e368_d_n11: f64 = (p.p50 * eq12_e367_d_n11);
        let eq12_e368_d_n12: f64 = (p.p50 * eq12_e367_d_n12);
        let eq12_e368_d_n13: f64 = (p.p50 * eq12_e367_d_n13);
        let eq12_e368_d_n14: f64 = (p.p50 * eq12_e367_d_n14);
        let eq12_e368_d_n15: f64 = (p.p50 * eq12_e367_d_n15);
        let eq12_e368_d_n16: f64 = (p.p50 * eq12_e367_d_n16);
        let eq12_e368_d_n17: f64 = (p.p50 * eq12_e367_d_n17);
        let eq12_e368_d_n18: f64 = (p.p50 * eq12_e367_d_n18);
        let eq12_e368_d_b0: f64 = (p.p50 * eq12_e367_d_b0);
        let eq12_e368_d_b1: f64 = (p.p50 * eq12_e367_d_b1);
        let eq12_e368_d_b2: f64 = (p.p50 * eq12_e367_d_b2);
        let eq12_e368_d_b3: f64 = (p.p50 * eq12_e367_d_b3);
        let eq12_e368_d_b4: f64 = (p.p50 * eq12_e367_d_b4);
        let eq12_e368_d_b5: f64 = (p.p50 * eq12_e367_d_b5);
        let eq12_e368_d_b6: f64 = (p.p50 * eq12_e367_d_b6);
        let eq12_e368_d_b7: f64 = (p.p50 * eq12_e367_d_b7);
        let eq12_e368_d_b8: f64 = (p.p50 * eq12_e367_d_b8);
        let eq12_e368_d_b9: f64 = (p.p50 * eq12_e367_d_b9);
        let eq12_e368_d_b10: f64 = (p.p50 * eq12_e367_d_b10);
        let eq12_e368_d_b11: f64 = (p.p50 * eq12_e367_d_b11);
        let eq12_value: f64 = eq12_e368;
        let eq12_node_derivatives: [f64; 19] = [eq12_e368_d_n0, eq12_e368_d_n1, eq12_e368_d_n2, eq12_e368_d_n3, eq12_e368_d_n4, eq12_e368_d_n5, eq12_e368_d_n6, eq12_e368_d_n7, eq12_e368_d_n8, eq12_e368_d_n9, eq12_e368_d_n10, eq12_e368_d_n11, eq12_e368_d_n12, eq12_e368_d_n13, eq12_e368_d_n14, eq12_e368_d_n15, eq12_e368_d_n16, eq12_e368_d_n17, eq12_e368_d_n18];
        let eq12_branch_derivatives: [f64; 12] = [eq12_e368_d_b0, eq12_e368_d_b1, eq12_e368_d_b2, eq12_e368_d_b3, eq12_e368_d_b4, eq12_e368_d_b5, eq12_e368_d_b6, eq12_e368_d_b7, eq12_e368_d_b8, eq12_e368_d_b9, eq12_e368_d_b10, eq12_e368_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[12]),
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
        let eq13_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq13_value),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq14_e379: f64 = (nv14 - 0.0);
        let eq14_value: f64 = eq14_e379;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq14_value),
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * 1.0),
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
        let eq15_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
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
        let eq16_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq16_value),
            &[
            ],
        );
    }
}

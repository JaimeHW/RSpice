#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq2_e1030,) = {
    if (!(s.v[3305] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1030;
        stamper.stamp_potential(
            branches[0],
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
        let (eq3_e1035,) = {
    if (!(s.v[3305] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1035;
        stamper.stamp_potential(
            branches[1],
            eq3_value,
            &[
            ],
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
        let (eq4_e1042, eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17, eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8,) = {
    if (s.v[3306] != 0.0) {
        let eq4_e1039: f64 = self.eval_ddt(2, s.v[931]);
        let eq4_e1039_d_n0: f64 = self.ddt_jacobian(s.dn[931][0]);
        let eq4_e1039_d_n1: f64 = self.ddt_jacobian(s.dn[931][1]);
        let eq4_e1039_d_n2: f64 = self.ddt_jacobian(s.dn[931][2]);
        let eq4_e1039_d_n3: f64 = self.ddt_jacobian(s.dn[931][3]);
        let eq4_e1039_d_n4: f64 = self.ddt_jacobian(s.dn[931][4]);
        let eq4_e1039_d_n5: f64 = self.ddt_jacobian(s.dn[931][5]);
        let eq4_e1039_d_n6: f64 = self.ddt_jacobian(s.dn[931][6]);
        let eq4_e1039_d_n7: f64 = self.ddt_jacobian(s.dn[931][7]);
        let eq4_e1039_d_n8: f64 = self.ddt_jacobian(s.dn[931][8]);
        let eq4_e1039_d_n9: f64 = self.ddt_jacobian(s.dn[931][9]);
        let eq4_e1039_d_n10: f64 = self.ddt_jacobian(s.dn[931][10]);
        let eq4_e1039_d_n11: f64 = self.ddt_jacobian(s.dn[931][11]);
        let eq4_e1039_d_n12: f64 = self.ddt_jacobian(s.dn[931][12]);
        let eq4_e1039_d_n13: f64 = self.ddt_jacobian(s.dn[931][13]);
        let eq4_e1039_d_n14: f64 = self.ddt_jacobian(s.dn[931][14]);
        let eq4_e1039_d_n15: f64 = self.ddt_jacobian(s.dn[931][15]);
        let eq4_e1039_d_n16: f64 = self.ddt_jacobian(s.dn[931][16]);
        let eq4_e1039_d_n17: f64 = self.ddt_jacobian(s.dn[931][17]);
        let eq4_e1039_d_b0: f64 = self.ddt_jacobian(s.db[931][0]);
        let eq4_e1039_d_b1: f64 = self.ddt_jacobian(s.db[931][1]);
        let eq4_e1039_d_b2: f64 = self.ddt_jacobian(s.db[931][2]);
        let eq4_e1039_d_b3: f64 = self.ddt_jacobian(s.db[931][3]);
        let eq4_e1039_d_b4: f64 = self.ddt_jacobian(s.db[931][4]);
        let eq4_e1039_d_b5: f64 = self.ddt_jacobian(s.db[931][5]);
        let eq4_e1039_d_b6: f64 = self.ddt_jacobian(s.db[931][6]);
        let eq4_e1039_d_b7: f64 = self.ddt_jacobian(s.db[931][7]);
        let eq4_e1039_d_b8: f64 = self.ddt_jacobian(s.db[931][8]);
        let eq4_e1040: f64 = (s.v[932] + eq4_e1039);
        let eq4_e1040_d_n0: f64 = (s.dn[932][0] + eq4_e1039_d_n0);
        let eq4_e1040_d_n1: f64 = (s.dn[932][1] + eq4_e1039_d_n1);
        let eq4_e1040_d_n2: f64 = (s.dn[932][2] + eq4_e1039_d_n2);
        let eq4_e1040_d_n3: f64 = (s.dn[932][3] + eq4_e1039_d_n3);
        let eq4_e1040_d_n4: f64 = (s.dn[932][4] + eq4_e1039_d_n4);
        let eq4_e1040_d_n5: f64 = (s.dn[932][5] + eq4_e1039_d_n5);
        let eq4_e1040_d_n6: f64 = (s.dn[932][6] + eq4_e1039_d_n6);
        let eq4_e1040_d_n7: f64 = (s.dn[932][7] + eq4_e1039_d_n7);
        let eq4_e1040_d_n8: f64 = (s.dn[932][8] + eq4_e1039_d_n8);
        let eq4_e1040_d_n9: f64 = (s.dn[932][9] + eq4_e1039_d_n9);
        let eq4_e1040_d_n10: f64 = (s.dn[932][10] + eq4_e1039_d_n10);
        let eq4_e1040_d_n11: f64 = (s.dn[932][11] + eq4_e1039_d_n11);
        let eq4_e1040_d_n12: f64 = (s.dn[932][12] + eq4_e1039_d_n12);
        let eq4_e1040_d_n13: f64 = (s.dn[932][13] + eq4_e1039_d_n13);
        let eq4_e1040_d_n14: f64 = (s.dn[932][14] + eq4_e1039_d_n14);
        let eq4_e1040_d_n15: f64 = (s.dn[932][15] + eq4_e1039_d_n15);
        let eq4_e1040_d_n16: f64 = (s.dn[932][16] + eq4_e1039_d_n16);
        let eq4_e1040_d_n17: f64 = (s.dn[932][17] + eq4_e1039_d_n17);
        let eq4_e1040_d_b0: f64 = (s.db[932][0] + eq4_e1039_d_b0);
        let eq4_e1040_d_b1: f64 = (s.db[932][1] + eq4_e1039_d_b1);
        let eq4_e1040_d_b2: f64 = (s.db[932][2] + eq4_e1039_d_b2);
        let eq4_e1040_d_b3: f64 = (s.db[932][3] + eq4_e1039_d_b3);
        let eq4_e1040_d_b4: f64 = (s.db[932][4] + eq4_e1039_d_b4);
        let eq4_e1040_d_b5: f64 = (s.db[932][5] + eq4_e1039_d_b5);
        let eq4_e1040_d_b6: f64 = (s.db[932][6] + eq4_e1039_d_b6);
        let eq4_e1040_d_b7: f64 = (s.db[932][7] + eq4_e1039_d_b7);
        let eq4_e1040_d_b8: f64 = (s.db[932][8] + eq4_e1039_d_b8);
        (eq4_e1040, eq4_e1040_d_n0, eq4_e1040_d_n1, eq4_e1040_d_n2, eq4_e1040_d_n3, eq4_e1040_d_n4, eq4_e1040_d_n5, eq4_e1040_d_n6, eq4_e1040_d_n7, eq4_e1040_d_n8, eq4_e1040_d_n9, eq4_e1040_d_n10, eq4_e1040_d_n11, eq4_e1040_d_n12, eq4_e1040_d_n13, eq4_e1040_d_n14, eq4_e1040_d_n15, eq4_e1040_d_n16, eq4_e1040_d_n17, eq4_e1040_d_b0, eq4_e1040_d_b1, eq4_e1040_d_b2, eq4_e1040_d_b3, eq4_e1040_d_b4, eq4_e1040_d_b5, eq4_e1040_d_b6, eq4_e1040_d_b7, eq4_e1040_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1042;
        let eq4_node_derivatives: [f64; 18] = [eq4_e1042_d_n0, eq4_e1042_d_n1, eq4_e1042_d_n2, eq4_e1042_d_n3, eq4_e1042_d_n4, eq4_e1042_d_n5, eq4_e1042_d_n6, eq4_e1042_d_n7, eq4_e1042_d_n8, eq4_e1042_d_n9, eq4_e1042_d_n10, eq4_e1042_d_n11, eq4_e1042_d_n12, eq4_e1042_d_n13, eq4_e1042_d_n14, eq4_e1042_d_n15, eq4_e1042_d_n16, eq4_e1042_d_n17];
        let eq4_branch_derivatives: [f64; 9] = [eq4_e1042_d_b0, eq4_e1042_d_b1, eq4_e1042_d_b2, eq4_e1042_d_b3, eq4_e1042_d_b4, eq4_e1042_d_b5, eq4_e1042_d_b6, eq4_e1042_d_b7, eq4_e1042_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[17]),
            None,
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
        let (eq5_e1047,) = {
    if (!(s.v[3306] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1047;
        stamper.stamp_potential(
            branches[2],
            eq5_value,
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
        let eq6_e1051: f64 = (s.v[134] + s.v[400]);
        let eq6_e1051_d_n0: f64 = (s.dn[134][0] + s.dn[400][0]);
        let eq6_e1051_d_n1: f64 = (s.dn[134][1] + s.dn[400][1]);
        let eq6_e1051_d_n2: f64 = (s.dn[134][2] + s.dn[400][2]);
        let eq6_e1051_d_n3: f64 = (s.dn[134][3] + s.dn[400][3]);
        let eq6_e1051_d_n4: f64 = (s.dn[134][4] + s.dn[400][4]);
        let eq6_e1051_d_n5: f64 = (s.dn[134][5] + s.dn[400][5]);
        let eq6_e1051_d_n6: f64 = (s.dn[134][6] + s.dn[400][6]);
        let eq6_e1051_d_n7: f64 = (s.dn[134][7] + s.dn[400][7]);
        let eq6_e1051_d_n8: f64 = (s.dn[134][8] + s.dn[400][8]);
        let eq6_e1051_d_n9: f64 = (s.dn[134][9] + s.dn[400][9]);
        let eq6_e1051_d_n10: f64 = (s.dn[134][10] + s.dn[400][10]);
        let eq6_e1051_d_n11: f64 = (s.dn[134][11] + s.dn[400][11]);
        let eq6_e1051_d_n12: f64 = (s.dn[134][12] + s.dn[400][12]);
        let eq6_e1051_d_n13: f64 = (s.dn[134][13] + s.dn[400][13]);
        let eq6_e1051_d_n14: f64 = (s.dn[134][14] + s.dn[400][14]);
        let eq6_e1051_d_n15: f64 = (s.dn[134][15] + s.dn[400][15]);
        let eq6_e1051_d_n16: f64 = (s.dn[134][16] + s.dn[400][16]);
        let eq6_e1051_d_n17: f64 = (s.dn[134][17] + s.dn[400][17]);
        let eq6_e1051_d_b0: f64 = (s.db[134][0] + s.db[400][0]);
        let eq6_e1051_d_b1: f64 = (s.db[134][1] + s.db[400][1]);
        let eq6_e1051_d_b2: f64 = (s.db[134][2] + s.db[400][2]);
        let eq6_e1051_d_b3: f64 = (s.db[134][3] + s.db[400][3]);
        let eq6_e1051_d_b4: f64 = (s.db[134][4] + s.db[400][4]);
        let eq6_e1051_d_b5: f64 = (s.db[134][5] + s.db[400][5]);
        let eq6_e1051_d_b6: f64 = (s.db[134][6] + s.db[400][6]);
        let eq6_e1051_d_b7: f64 = (s.db[134][7] + s.db[400][7]);
        let eq6_e1051_d_b8: f64 = (s.db[134][8] + s.db[400][8]);
        let eq6_e1053: f64 = (eq6_e1051 - s.v[738]);
        let eq6_e1053_d_n0: f64 = (eq6_e1051_d_n0 - s.dn[738][0]);
        let eq6_e1053_d_n1: f64 = (eq6_e1051_d_n1 - s.dn[738][1]);
        let eq6_e1053_d_n2: f64 = (eq6_e1051_d_n2 - s.dn[738][2]);
        let eq6_e1053_d_n3: f64 = (eq6_e1051_d_n3 - s.dn[738][3]);
        let eq6_e1053_d_n4: f64 = (eq6_e1051_d_n4 - s.dn[738][4]);
        let eq6_e1053_d_n5: f64 = (eq6_e1051_d_n5 - s.dn[738][5]);
        let eq6_e1053_d_n6: f64 = (eq6_e1051_d_n6 - s.dn[738][6]);
        let eq6_e1053_d_n7: f64 = (eq6_e1051_d_n7 - s.dn[738][7]);
        let eq6_e1053_d_n8: f64 = (eq6_e1051_d_n8 - s.dn[738][8]);
        let eq6_e1053_d_n9: f64 = (eq6_e1051_d_n9 - s.dn[738][9]);
        let eq6_e1053_d_n10: f64 = (eq6_e1051_d_n10 - s.dn[738][10]);
        let eq6_e1053_d_n11: f64 = (eq6_e1051_d_n11 - s.dn[738][11]);
        let eq6_e1053_d_n12: f64 = (eq6_e1051_d_n12 - s.dn[738][12]);
        let eq6_e1053_d_n13: f64 = (eq6_e1051_d_n13 - s.dn[738][13]);
        let eq6_e1053_d_n14: f64 = (eq6_e1051_d_n14 - s.dn[738][14]);
        let eq6_e1053_d_n15: f64 = (eq6_e1051_d_n15 - s.dn[738][15]);
        let eq6_e1053_d_n16: f64 = (eq6_e1051_d_n16 - s.dn[738][16]);
        let eq6_e1053_d_n17: f64 = (eq6_e1051_d_n17 - s.dn[738][17]);
        let eq6_e1053_d_b0: f64 = (eq6_e1051_d_b0 - s.db[738][0]);
        let eq6_e1053_d_b1: f64 = (eq6_e1051_d_b1 - s.db[738][1]);
        let eq6_e1053_d_b2: f64 = (eq6_e1051_d_b2 - s.db[738][2]);
        let eq6_e1053_d_b3: f64 = (eq6_e1051_d_b3 - s.db[738][3]);
        let eq6_e1053_d_b4: f64 = (eq6_e1051_d_b4 - s.db[738][4]);
        let eq6_e1053_d_b5: f64 = (eq6_e1051_d_b5 - s.db[738][5]);
        let eq6_e1053_d_b6: f64 = (eq6_e1051_d_b6 - s.db[738][6]);
        let eq6_e1053_d_b7: f64 = (eq6_e1051_d_b7 - s.db[738][7]);
        let eq6_e1053_d_b8: f64 = (eq6_e1051_d_b8 - s.db[738][8]);
        let eq6_e1054: f64 = (p.p87 * eq6_e1053);
        let eq6_e1054_d_n0: f64 = (p.p87 * eq6_e1053_d_n0);
        let eq6_e1054_d_n1: f64 = (p.p87 * eq6_e1053_d_n1);
        let eq6_e1054_d_n2: f64 = (p.p87 * eq6_e1053_d_n2);
        let eq6_e1054_d_n3: f64 = (p.p87 * eq6_e1053_d_n3);
        let eq6_e1054_d_n4: f64 = (p.p87 * eq6_e1053_d_n4);
        let eq6_e1054_d_n5: f64 = (p.p87 * eq6_e1053_d_n5);
        let eq6_e1054_d_n6: f64 = (p.p87 * eq6_e1053_d_n6);
        let eq6_e1054_d_n7: f64 = (p.p87 * eq6_e1053_d_n7);
        let eq6_e1054_d_n8: f64 = (p.p87 * eq6_e1053_d_n8);
        let eq6_e1054_d_n9: f64 = (p.p87 * eq6_e1053_d_n9);
        let eq6_e1054_d_n10: f64 = (p.p87 * eq6_e1053_d_n10);
        let eq6_e1054_d_n11: f64 = (p.p87 * eq6_e1053_d_n11);
        let eq6_e1054_d_n12: f64 = (p.p87 * eq6_e1053_d_n12);
        let eq6_e1054_d_n13: f64 = (p.p87 * eq6_e1053_d_n13);
        let eq6_e1054_d_n14: f64 = (p.p87 * eq6_e1053_d_n14);
        let eq6_e1054_d_n15: f64 = (p.p87 * eq6_e1053_d_n15);
        let eq6_e1054_d_n16: f64 = (p.p87 * eq6_e1053_d_n16);
        let eq6_e1054_d_n17: f64 = (p.p87 * eq6_e1053_d_n17);
        let eq6_e1054_d_b0: f64 = (p.p87 * eq6_e1053_d_b0);
        let eq6_e1054_d_b1: f64 = (p.p87 * eq6_e1053_d_b1);
        let eq6_e1054_d_b2: f64 = (p.p87 * eq6_e1053_d_b2);
        let eq6_e1054_d_b3: f64 = (p.p87 * eq6_e1053_d_b3);
        let eq6_e1054_d_b4: f64 = (p.p87 * eq6_e1053_d_b4);
        let eq6_e1054_d_b5: f64 = (p.p87 * eq6_e1053_d_b5);
        let eq6_e1054_d_b6: f64 = (p.p87 * eq6_e1053_d_b6);
        let eq6_e1054_d_b7: f64 = (p.p87 * eq6_e1053_d_b7);
        let eq6_e1054_d_b8: f64 = (p.p87 * eq6_e1053_d_b8);
        let eq6_value: f64 = eq6_e1054;
        let eq6_node_derivatives: [f64; 18] = [eq6_e1054_d_n0, eq6_e1054_d_n1, eq6_e1054_d_n2, eq6_e1054_d_n3, eq6_e1054_d_n4, eq6_e1054_d_n5, eq6_e1054_d_n6, eq6_e1054_d_n7, eq6_e1054_d_n8, eq6_e1054_d_n9, eq6_e1054_d_n10, eq6_e1054_d_n11, eq6_e1054_d_n12, eq6_e1054_d_n13, eq6_e1054_d_n14, eq6_e1054_d_n15, eq6_e1054_d_n16, eq6_e1054_d_n17];
        let eq6_branch_derivatives: [f64; 9] = [eq6_e1054_d_b0, eq6_e1054_d_b1, eq6_e1054_d_b2, eq6_e1054_d_b3, eq6_e1054_d_b4, eq6_e1054_d_b5, eq6_e1054_d_b6, eq6_e1054_d_b7, eq6_e1054_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
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
        let eq7_e1058: f64 = (s.v[424] - s.v[425]);
        let eq7_e1058_d_n0: f64 = (s.dn[424][0] - s.dn[425][0]);
        let eq7_e1058_d_n1: f64 = (s.dn[424][1] - s.dn[425][1]);
        let eq7_e1058_d_n2: f64 = (s.dn[424][2] - s.dn[425][2]);
        let eq7_e1058_d_n3: f64 = (s.dn[424][3] - s.dn[425][3]);
        let eq7_e1058_d_n4: f64 = (s.dn[424][4] - s.dn[425][4]);
        let eq7_e1058_d_n5: f64 = (s.dn[424][5] - s.dn[425][5]);
        let eq7_e1058_d_n6: f64 = (s.dn[424][6] - s.dn[425][6]);
        let eq7_e1058_d_n7: f64 = (s.dn[424][7] - s.dn[425][7]);
        let eq7_e1058_d_n8: f64 = (s.dn[424][8] - s.dn[425][8]);
        let eq7_e1058_d_n9: f64 = (s.dn[424][9] - s.dn[425][9]);
        let eq7_e1058_d_n10: f64 = (s.dn[424][10] - s.dn[425][10]);
        let eq7_e1058_d_n11: f64 = (s.dn[424][11] - s.dn[425][11]);
        let eq7_e1058_d_n12: f64 = (s.dn[424][12] - s.dn[425][12]);
        let eq7_e1058_d_n13: f64 = (s.dn[424][13] - s.dn[425][13]);
        let eq7_e1058_d_n14: f64 = (s.dn[424][14] - s.dn[425][14]);
        let eq7_e1058_d_n15: f64 = (s.dn[424][15] - s.dn[425][15]);
        let eq7_e1058_d_n16: f64 = (s.dn[424][16] - s.dn[425][16]);
        let eq7_e1058_d_n17: f64 = (s.dn[424][17] - s.dn[425][17]);
        let eq7_e1058_d_b0: f64 = (s.db[424][0] - s.db[425][0]);
        let eq7_e1058_d_b1: f64 = (s.db[424][1] - s.db[425][1]);
        let eq7_e1058_d_b2: f64 = (s.db[424][2] - s.db[425][2]);
        let eq7_e1058_d_b3: f64 = (s.db[424][3] - s.db[425][3]);
        let eq7_e1058_d_b4: f64 = (s.db[424][4] - s.db[425][4]);
        let eq7_e1058_d_b5: f64 = (s.db[424][5] - s.db[425][5]);
        let eq7_e1058_d_b6: f64 = (s.db[424][6] - s.db[425][6]);
        let eq7_e1058_d_b7: f64 = (s.db[424][7] - s.db[425][7]);
        let eq7_e1058_d_b8: f64 = (s.db[424][8] - s.db[425][8]);
        let eq7_e1059: f64 = (p.p87 * eq7_e1058);
        let eq7_e1059_d_n0: f64 = (p.p87 * eq7_e1058_d_n0);
        let eq7_e1059_d_n1: f64 = (p.p87 * eq7_e1058_d_n1);
        let eq7_e1059_d_n2: f64 = (p.p87 * eq7_e1058_d_n2);
        let eq7_e1059_d_n3: f64 = (p.p87 * eq7_e1058_d_n3);
        let eq7_e1059_d_n4: f64 = (p.p87 * eq7_e1058_d_n4);
        let eq7_e1059_d_n5: f64 = (p.p87 * eq7_e1058_d_n5);
        let eq7_e1059_d_n6: f64 = (p.p87 * eq7_e1058_d_n6);
        let eq7_e1059_d_n7: f64 = (p.p87 * eq7_e1058_d_n7);
        let eq7_e1059_d_n8: f64 = (p.p87 * eq7_e1058_d_n8);
        let eq7_e1059_d_n9: f64 = (p.p87 * eq7_e1058_d_n9);
        let eq7_e1059_d_n10: f64 = (p.p87 * eq7_e1058_d_n10);
        let eq7_e1059_d_n11: f64 = (p.p87 * eq7_e1058_d_n11);
        let eq7_e1059_d_n12: f64 = (p.p87 * eq7_e1058_d_n12);
        let eq7_e1059_d_n13: f64 = (p.p87 * eq7_e1058_d_n13);
        let eq7_e1059_d_n14: f64 = (p.p87 * eq7_e1058_d_n14);
        let eq7_e1059_d_n15: f64 = (p.p87 * eq7_e1058_d_n15);
        let eq7_e1059_d_n16: f64 = (p.p87 * eq7_e1058_d_n16);
        let eq7_e1059_d_n17: f64 = (p.p87 * eq7_e1058_d_n17);
        let eq7_e1059_d_b0: f64 = (p.p87 * eq7_e1058_d_b0);
        let eq7_e1059_d_b1: f64 = (p.p87 * eq7_e1058_d_b1);
        let eq7_e1059_d_b2: f64 = (p.p87 * eq7_e1058_d_b2);
        let eq7_e1059_d_b3: f64 = (p.p87 * eq7_e1058_d_b3);
        let eq7_e1059_d_b4: f64 = (p.p87 * eq7_e1058_d_b4);
        let eq7_e1059_d_b5: f64 = (p.p87 * eq7_e1058_d_b5);
        let eq7_e1059_d_b6: f64 = (p.p87 * eq7_e1058_d_b6);
        let eq7_e1059_d_b7: f64 = (p.p87 * eq7_e1058_d_b7);
        let eq7_e1059_d_b8: f64 = (p.p87 * eq7_e1058_d_b8);
        let eq7_value: f64 = eq7_e1059;
        let eq7_node_derivatives: [f64; 18] = [eq7_e1059_d_n0, eq7_e1059_d_n1, eq7_e1059_d_n2, eq7_e1059_d_n3, eq7_e1059_d_n4, eq7_e1059_d_n5, eq7_e1059_d_n6, eq7_e1059_d_n7, eq7_e1059_d_n8, eq7_e1059_d_n9, eq7_e1059_d_n10, eq7_e1059_d_n11, eq7_e1059_d_n12, eq7_e1059_d_n13, eq7_e1059_d_n14, eq7_e1059_d_n15, eq7_e1059_d_n16, eq7_e1059_d_n17];
        let eq7_branch_derivatives: [f64; 9] = [eq7_e1059_d_b0, eq7_e1059_d_b1, eq7_e1059_d_b2, eq7_e1059_d_b3, eq7_e1059_d_b4, eq7_e1059_d_b5, eq7_e1059_d_b6, eq7_e1059_d_b7, eq7_e1059_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[0]),
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
        let eq8_e1063: f64 = (s.v[203] + s.v[280]);
        let eq8_e1063_d_n0: f64 = (s.dn[203][0] + s.dn[280][0]);
        let eq8_e1063_d_n1: f64 = (s.dn[203][1] + s.dn[280][1]);
        let eq8_e1063_d_n2: f64 = (s.dn[203][2] + s.dn[280][2]);
        let eq8_e1063_d_n3: f64 = (s.dn[203][3] + s.dn[280][3]);
        let eq8_e1063_d_n4: f64 = (s.dn[203][4] + s.dn[280][4]);
        let eq8_e1063_d_n5: f64 = (s.dn[203][5] + s.dn[280][5]);
        let eq8_e1063_d_n6: f64 = (s.dn[203][6] + s.dn[280][6]);
        let eq8_e1063_d_n7: f64 = (s.dn[203][7] + s.dn[280][7]);
        let eq8_e1063_d_n8: f64 = (s.dn[203][8] + s.dn[280][8]);
        let eq8_e1063_d_n9: f64 = (s.dn[203][9] + s.dn[280][9]);
        let eq8_e1063_d_n10: f64 = (s.dn[203][10] + s.dn[280][10]);
        let eq8_e1063_d_n11: f64 = (s.dn[203][11] + s.dn[280][11]);
        let eq8_e1063_d_n12: f64 = (s.dn[203][12] + s.dn[280][12]);
        let eq8_e1063_d_n13: f64 = (s.dn[203][13] + s.dn[280][13]);
        let eq8_e1063_d_n14: f64 = (s.dn[203][14] + s.dn[280][14]);
        let eq8_e1063_d_n15: f64 = (s.dn[203][15] + s.dn[280][15]);
        let eq8_e1063_d_n16: f64 = (s.dn[203][16] + s.dn[280][16]);
        let eq8_e1063_d_n17: f64 = (s.dn[203][17] + s.dn[280][17]);
        let eq8_e1063_d_b0: f64 = (s.db[203][0] + s.db[280][0]);
        let eq8_e1063_d_b1: f64 = (s.db[203][1] + s.db[280][1]);
        let eq8_e1063_d_b2: f64 = (s.db[203][2] + s.db[280][2]);
        let eq8_e1063_d_b3: f64 = (s.db[203][3] + s.db[280][3]);
        let eq8_e1063_d_b4: f64 = (s.db[203][4] + s.db[280][4]);
        let eq8_e1063_d_b5: f64 = (s.db[203][5] + s.db[280][5]);
        let eq8_e1063_d_b6: f64 = (s.db[203][6] + s.db[280][6]);
        let eq8_e1063_d_b7: f64 = (s.db[203][7] + s.db[280][7]);
        let eq8_e1063_d_b8: f64 = (s.db[203][8] + s.db[280][8]);
        let eq8_e1065: f64 = (eq8_e1063 + s.v[431]);
        let eq8_e1065_d_n0: f64 = (eq8_e1063_d_n0 + s.dn[431][0]);
        let eq8_e1065_d_n1: f64 = (eq8_e1063_d_n1 + s.dn[431][1]);
        let eq8_e1065_d_n2: f64 = (eq8_e1063_d_n2 + s.dn[431][2]);
        let eq8_e1065_d_n3: f64 = (eq8_e1063_d_n3 + s.dn[431][3]);
        let eq8_e1065_d_n4: f64 = (eq8_e1063_d_n4 + s.dn[431][4]);
        let eq8_e1065_d_n5: f64 = (eq8_e1063_d_n5 + s.dn[431][5]);
        let eq8_e1065_d_n6: f64 = (eq8_e1063_d_n6 + s.dn[431][6]);
        let eq8_e1065_d_n7: f64 = (eq8_e1063_d_n7 + s.dn[431][7]);
        let eq8_e1065_d_n8: f64 = (eq8_e1063_d_n8 + s.dn[431][8]);
        let eq8_e1065_d_n9: f64 = (eq8_e1063_d_n9 + s.dn[431][9]);
        let eq8_e1065_d_n10: f64 = (eq8_e1063_d_n10 + s.dn[431][10]);
        let eq8_e1065_d_n11: f64 = (eq8_e1063_d_n11 + s.dn[431][11]);
        let eq8_e1065_d_n12: f64 = (eq8_e1063_d_n12 + s.dn[431][12]);
        let eq8_e1065_d_n13: f64 = (eq8_e1063_d_n13 + s.dn[431][13]);
        let eq8_e1065_d_n14: f64 = (eq8_e1063_d_n14 + s.dn[431][14]);
        let eq8_e1065_d_n15: f64 = (eq8_e1063_d_n15 + s.dn[431][15]);
        let eq8_e1065_d_n16: f64 = (eq8_e1063_d_n16 + s.dn[431][16]);
        let eq8_e1065_d_n17: f64 = (eq8_e1063_d_n17 + s.dn[431][17]);
        let eq8_e1065_d_b0: f64 = (eq8_e1063_d_b0 + s.db[431][0]);
        let eq8_e1065_d_b1: f64 = (eq8_e1063_d_b1 + s.db[431][1]);
        let eq8_e1065_d_b2: f64 = (eq8_e1063_d_b2 + s.db[431][2]);
        let eq8_e1065_d_b3: f64 = (eq8_e1063_d_b3 + s.db[431][3]);
        let eq8_e1065_d_b4: f64 = (eq8_e1063_d_b4 + s.db[431][4]);
        let eq8_e1065_d_b5: f64 = (eq8_e1063_d_b5 + s.db[431][5]);
        let eq8_e1065_d_b6: f64 = (eq8_e1063_d_b6 + s.db[431][6]);
        let eq8_e1065_d_b7: f64 = (eq8_e1063_d_b7 + s.db[431][7]);
        let eq8_e1065_d_b8: f64 = (eq8_e1063_d_b8 + s.db[431][8]);
        let eq8_e1066: f64 = (p.p87 * eq8_e1065);
        let eq8_e1066_d_n0: f64 = (p.p87 * eq8_e1065_d_n0);
        let eq8_e1066_d_n1: f64 = (p.p87 * eq8_e1065_d_n1);
        let eq8_e1066_d_n2: f64 = (p.p87 * eq8_e1065_d_n2);
        let eq8_e1066_d_n3: f64 = (p.p87 * eq8_e1065_d_n3);
        let eq8_e1066_d_n4: f64 = (p.p87 * eq8_e1065_d_n4);
        let eq8_e1066_d_n5: f64 = (p.p87 * eq8_e1065_d_n5);
        let eq8_e1066_d_n6: f64 = (p.p87 * eq8_e1065_d_n6);
        let eq8_e1066_d_n7: f64 = (p.p87 * eq8_e1065_d_n7);
        let eq8_e1066_d_n8: f64 = (p.p87 * eq8_e1065_d_n8);
        let eq8_e1066_d_n9: f64 = (p.p87 * eq8_e1065_d_n9);
        let eq8_e1066_d_n10: f64 = (p.p87 * eq8_e1065_d_n10);
        let eq8_e1066_d_n11: f64 = (p.p87 * eq8_e1065_d_n11);
        let eq8_e1066_d_n12: f64 = (p.p87 * eq8_e1065_d_n12);
        let eq8_e1066_d_n13: f64 = (p.p87 * eq8_e1065_d_n13);
        let eq8_e1066_d_n14: f64 = (p.p87 * eq8_e1065_d_n14);
        let eq8_e1066_d_n15: f64 = (p.p87 * eq8_e1065_d_n15);
        let eq8_e1066_d_n16: f64 = (p.p87 * eq8_e1065_d_n16);
        let eq8_e1066_d_n17: f64 = (p.p87 * eq8_e1065_d_n17);
        let eq8_e1066_d_b0: f64 = (p.p87 * eq8_e1065_d_b0);
        let eq8_e1066_d_b1: f64 = (p.p87 * eq8_e1065_d_b1);
        let eq8_e1066_d_b2: f64 = (p.p87 * eq8_e1065_d_b2);
        let eq8_e1066_d_b3: f64 = (p.p87 * eq8_e1065_d_b3);
        let eq8_e1066_d_b4: f64 = (p.p87 * eq8_e1065_d_b4);
        let eq8_e1066_d_b5: f64 = (p.p87 * eq8_e1065_d_b5);
        let eq8_e1066_d_b6: f64 = (p.p87 * eq8_e1065_d_b6);
        let eq8_e1066_d_b7: f64 = (p.p87 * eq8_e1065_d_b7);
        let eq8_e1066_d_b8: f64 = (p.p87 * eq8_e1065_d_b8);
        let eq8_value: f64 = eq8_e1066;
        let eq8_node_derivatives: [f64; 18] = [eq8_e1066_d_n0, eq8_e1066_d_n1, eq8_e1066_d_n2, eq8_e1066_d_n3, eq8_e1066_d_n4, eq8_e1066_d_n5, eq8_e1066_d_n6, eq8_e1066_d_n7, eq8_e1066_d_n8, eq8_e1066_d_n9, eq8_e1066_d_n10, eq8_e1066_d_n11, eq8_e1066_d_n12, eq8_e1066_d_n13, eq8_e1066_d_n14, eq8_e1066_d_n15, eq8_e1066_d_n16, eq8_e1066_d_n17];
        let eq8_branch_derivatives: [f64; 9] = [eq8_e1066_d_b0, eq8_e1066_d_b1, eq8_e1066_d_b2, eq8_e1066_d_b3, eq8_e1066_d_b4, eq8_e1066_d_b5, eq8_e1066_d_b6, eq8_e1066_d_b7, eq8_e1066_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[8]),
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
        let eq9_e1070: f64 = (s.v[204] + s.v[736]);
        let eq9_e1070_d_n0: f64 = (s.dn[204][0] + s.dn[736][0]);
        let eq9_e1070_d_n1: f64 = (s.dn[204][1] + s.dn[736][1]);
        let eq9_e1070_d_n2: f64 = (s.dn[204][2] + s.dn[736][2]);
        let eq9_e1070_d_n3: f64 = (s.dn[204][3] + s.dn[736][3]);
        let eq9_e1070_d_n4: f64 = (s.dn[204][4] + s.dn[736][4]);
        let eq9_e1070_d_n5: f64 = (s.dn[204][5] + s.dn[736][5]);
        let eq9_e1070_d_n6: f64 = (s.dn[204][6] + s.dn[736][6]);
        let eq9_e1070_d_n7: f64 = (s.dn[204][7] + s.dn[736][7]);
        let eq9_e1070_d_n8: f64 = (s.dn[204][8] + s.dn[736][8]);
        let eq9_e1070_d_n9: f64 = (s.dn[204][9] + s.dn[736][9]);
        let eq9_e1070_d_n10: f64 = (s.dn[204][10] + s.dn[736][10]);
        let eq9_e1070_d_n11: f64 = (s.dn[204][11] + s.dn[736][11]);
        let eq9_e1070_d_n12: f64 = (s.dn[204][12] + s.dn[736][12]);
        let eq9_e1070_d_n13: f64 = (s.dn[204][13] + s.dn[736][13]);
        let eq9_e1070_d_n14: f64 = (s.dn[204][14] + s.dn[736][14]);
        let eq9_e1070_d_n15: f64 = (s.dn[204][15] + s.dn[736][15]);
        let eq9_e1070_d_n16: f64 = (s.dn[204][16] + s.dn[736][16]);
        let eq9_e1070_d_n17: f64 = (s.dn[204][17] + s.dn[736][17]);
        let eq9_e1070_d_b0: f64 = (s.db[204][0] + s.db[736][0]);
        let eq9_e1070_d_b1: f64 = (s.db[204][1] + s.db[736][1]);
        let eq9_e1070_d_b2: f64 = (s.db[204][2] + s.db[736][2]);
        let eq9_e1070_d_b3: f64 = (s.db[204][3] + s.db[736][3]);
        let eq9_e1070_d_b4: f64 = (s.db[204][4] + s.db[736][4]);
        let eq9_e1070_d_b5: f64 = (s.db[204][5] + s.db[736][5]);
        let eq9_e1070_d_b6: f64 = (s.db[204][6] + s.db[736][6]);
        let eq9_e1070_d_b7: f64 = (s.db[204][7] + s.db[736][7]);
        let eq9_e1070_d_b8: f64 = (s.db[204][8] + s.db[736][8]);
        let eq9_e1072: f64 = (eq9_e1070 + s.v[432]);
        let eq9_e1072_d_n0: f64 = (eq9_e1070_d_n0 + s.dn[432][0]);
        let eq9_e1072_d_n1: f64 = (eq9_e1070_d_n1 + s.dn[432][1]);
        let eq9_e1072_d_n2: f64 = (eq9_e1070_d_n2 + s.dn[432][2]);
        let eq9_e1072_d_n3: f64 = (eq9_e1070_d_n3 + s.dn[432][3]);
        let eq9_e1072_d_n4: f64 = (eq9_e1070_d_n4 + s.dn[432][4]);
        let eq9_e1072_d_n5: f64 = (eq9_e1070_d_n5 + s.dn[432][5]);
        let eq9_e1072_d_n6: f64 = (eq9_e1070_d_n6 + s.dn[432][6]);
        let eq9_e1072_d_n7: f64 = (eq9_e1070_d_n7 + s.dn[432][7]);
        let eq9_e1072_d_n8: f64 = (eq9_e1070_d_n8 + s.dn[432][8]);
        let eq9_e1072_d_n9: f64 = (eq9_e1070_d_n9 + s.dn[432][9]);
        let eq9_e1072_d_n10: f64 = (eq9_e1070_d_n10 + s.dn[432][10]);
        let eq9_e1072_d_n11: f64 = (eq9_e1070_d_n11 + s.dn[432][11]);
        let eq9_e1072_d_n12: f64 = (eq9_e1070_d_n12 + s.dn[432][12]);
        let eq9_e1072_d_n13: f64 = (eq9_e1070_d_n13 + s.dn[432][13]);
        let eq9_e1072_d_n14: f64 = (eq9_e1070_d_n14 + s.dn[432][14]);
        let eq9_e1072_d_n15: f64 = (eq9_e1070_d_n15 + s.dn[432][15]);
        let eq9_e1072_d_n16: f64 = (eq9_e1070_d_n16 + s.dn[432][16]);
        let eq9_e1072_d_n17: f64 = (eq9_e1070_d_n17 + s.dn[432][17]);
        let eq9_e1072_d_b0: f64 = (eq9_e1070_d_b0 + s.db[432][0]);
        let eq9_e1072_d_b1: f64 = (eq9_e1070_d_b1 + s.db[432][1]);
        let eq9_e1072_d_b2: f64 = (eq9_e1070_d_b2 + s.db[432][2]);
        let eq9_e1072_d_b3: f64 = (eq9_e1070_d_b3 + s.db[432][3]);
        let eq9_e1072_d_b4: f64 = (eq9_e1070_d_b4 + s.db[432][4]);
        let eq9_e1072_d_b5: f64 = (eq9_e1070_d_b5 + s.db[432][5]);
        let eq9_e1072_d_b6: f64 = (eq9_e1070_d_b6 + s.db[432][6]);
        let eq9_e1072_d_b7: f64 = (eq9_e1070_d_b7 + s.db[432][7]);
        let eq9_e1072_d_b8: f64 = (eq9_e1070_d_b8 + s.db[432][8]);
        let eq9_e1073: f64 = (p.p87 * eq9_e1072);
        let eq9_e1073_d_n0: f64 = (p.p87 * eq9_e1072_d_n0);
        let eq9_e1073_d_n1: f64 = (p.p87 * eq9_e1072_d_n1);
        let eq9_e1073_d_n2: f64 = (p.p87 * eq9_e1072_d_n2);
        let eq9_e1073_d_n3: f64 = (p.p87 * eq9_e1072_d_n3);
        let eq9_e1073_d_n4: f64 = (p.p87 * eq9_e1072_d_n4);
        let eq9_e1073_d_n5: f64 = (p.p87 * eq9_e1072_d_n5);
        let eq9_e1073_d_n6: f64 = (p.p87 * eq9_e1072_d_n6);
        let eq9_e1073_d_n7: f64 = (p.p87 * eq9_e1072_d_n7);
        let eq9_e1073_d_n8: f64 = (p.p87 * eq9_e1072_d_n8);
        let eq9_e1073_d_n9: f64 = (p.p87 * eq9_e1072_d_n9);
        let eq9_e1073_d_n10: f64 = (p.p87 * eq9_e1072_d_n10);
        let eq9_e1073_d_n11: f64 = (p.p87 * eq9_e1072_d_n11);
        let eq9_e1073_d_n12: f64 = (p.p87 * eq9_e1072_d_n12);
        let eq9_e1073_d_n13: f64 = (p.p87 * eq9_e1072_d_n13);
        let eq9_e1073_d_n14: f64 = (p.p87 * eq9_e1072_d_n14);
        let eq9_e1073_d_n15: f64 = (p.p87 * eq9_e1072_d_n15);
        let eq9_e1073_d_n16: f64 = (p.p87 * eq9_e1072_d_n16);
        let eq9_e1073_d_n17: f64 = (p.p87 * eq9_e1072_d_n17);
        let eq9_e1073_d_b0: f64 = (p.p87 * eq9_e1072_d_b0);
        let eq9_e1073_d_b1: f64 = (p.p87 * eq9_e1072_d_b1);
        let eq9_e1073_d_b2: f64 = (p.p87 * eq9_e1072_d_b2);
        let eq9_e1073_d_b3: f64 = (p.p87 * eq9_e1072_d_b3);
        let eq9_e1073_d_b4: f64 = (p.p87 * eq9_e1072_d_b4);
        let eq9_e1073_d_b5: f64 = (p.p87 * eq9_e1072_d_b5);
        let eq9_e1073_d_b6: f64 = (p.p87 * eq9_e1072_d_b6);
        let eq9_e1073_d_b7: f64 = (p.p87 * eq9_e1072_d_b7);
        let eq9_e1073_d_b8: f64 = (p.p87 * eq9_e1072_d_b8);
        let eq9_value: f64 = eq9_e1073;
        let eq9_node_derivatives: [f64; 18] = [eq9_e1073_d_n0, eq9_e1073_d_n1, eq9_e1073_d_n2, eq9_e1073_d_n3, eq9_e1073_d_n4, eq9_e1073_d_n5, eq9_e1073_d_n6, eq9_e1073_d_n7, eq9_e1073_d_n8, eq9_e1073_d_n9, eq9_e1073_d_n10, eq9_e1073_d_n11, eq9_e1073_d_n12, eq9_e1073_d_n13, eq9_e1073_d_n14, eq9_e1073_d_n15, eq9_e1073_d_n16, eq9_e1073_d_n17];
        let eq9_branch_derivatives: [f64; 9] = [eq9_e1073_d_b0, eq9_e1073_d_b1, eq9_e1073_d_b2, eq9_e1073_d_b3, eq9_e1073_d_b4, eq9_e1073_d_b5, eq9_e1073_d_b6, eq9_e1073_d_b7, eq9_e1073_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let eq10_e1076: f64 = (p.p87 * s.v[281]);
        let eq10_e1076_d_n0: f64 = (p.p87 * s.dn[281][0]);
        let eq10_e1076_d_n1: f64 = (p.p87 * s.dn[281][1]);
        let eq10_e1076_d_n2: f64 = (p.p87 * s.dn[281][2]);
        let eq10_e1076_d_n3: f64 = (p.p87 * s.dn[281][3]);
        let eq10_e1076_d_n4: f64 = (p.p87 * s.dn[281][4]);
        let eq10_e1076_d_n5: f64 = (p.p87 * s.dn[281][5]);
        let eq10_e1076_d_n6: f64 = (p.p87 * s.dn[281][6]);
        let eq10_e1076_d_n7: f64 = (p.p87 * s.dn[281][7]);
        let eq10_e1076_d_n8: f64 = (p.p87 * s.dn[281][8]);
        let eq10_e1076_d_n9: f64 = (p.p87 * s.dn[281][9]);
        let eq10_e1076_d_n10: f64 = (p.p87 * s.dn[281][10]);
        let eq10_e1076_d_n11: f64 = (p.p87 * s.dn[281][11]);
        let eq10_e1076_d_n12: f64 = (p.p87 * s.dn[281][12]);
        let eq10_e1076_d_n13: f64 = (p.p87 * s.dn[281][13]);
        let eq10_e1076_d_n14: f64 = (p.p87 * s.dn[281][14]);
        let eq10_e1076_d_n15: f64 = (p.p87 * s.dn[281][15]);
        let eq10_e1076_d_n16: f64 = (p.p87 * s.dn[281][16]);
        let eq10_e1076_d_n17: f64 = (p.p87 * s.dn[281][17]);
        let eq10_e1076_d_b0: f64 = (p.p87 * s.db[281][0]);
        let eq10_e1076_d_b1: f64 = (p.p87 * s.db[281][1]);
        let eq10_e1076_d_b2: f64 = (p.p87 * s.db[281][2]);
        let eq10_e1076_d_b3: f64 = (p.p87 * s.db[281][3]);
        let eq10_e1076_d_b4: f64 = (p.p87 * s.db[281][4]);
        let eq10_e1076_d_b5: f64 = (p.p87 * s.db[281][5]);
        let eq10_e1076_d_b6: f64 = (p.p87 * s.db[281][6]);
        let eq10_e1076_d_b7: f64 = (p.p87 * s.db[281][7]);
        let eq10_e1076_d_b8: f64 = (p.p87 * s.db[281][8]);
        let eq10_value: f64 = eq10_e1076;
        let eq10_node_derivatives: [f64; 18] = [eq10_e1076_d_n0, eq10_e1076_d_n1, eq10_e1076_d_n2, eq10_e1076_d_n3, eq10_e1076_d_n4, eq10_e1076_d_n5, eq10_e1076_d_n6, eq10_e1076_d_n7, eq10_e1076_d_n8, eq10_e1076_d_n9, eq10_e1076_d_n10, eq10_e1076_d_n11, eq10_e1076_d_n12, eq10_e1076_d_n13, eq10_e1076_d_n14, eq10_e1076_d_n15, eq10_e1076_d_n16, eq10_e1076_d_n17];
        let eq10_branch_derivatives: [f64; 9] = [eq10_e1076_d_b0, eq10_e1076_d_b1, eq10_e1076_d_b2, eq10_e1076_d_b3, eq10_e1076_d_b4, eq10_e1076_d_b5, eq10_e1076_d_b6, eq10_e1076_d_b7, eq10_e1076_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[8]),
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
        let eq11_e1079: f64 = (p.p87 * s.v[737]);
        let eq11_e1079_d_n0: f64 = (p.p87 * s.dn[737][0]);
        let eq11_e1079_d_n1: f64 = (p.p87 * s.dn[737][1]);
        let eq11_e1079_d_n2: f64 = (p.p87 * s.dn[737][2]);
        let eq11_e1079_d_n3: f64 = (p.p87 * s.dn[737][3]);
        let eq11_e1079_d_n4: f64 = (p.p87 * s.dn[737][4]);
        let eq11_e1079_d_n5: f64 = (p.p87 * s.dn[737][5]);
        let eq11_e1079_d_n6: f64 = (p.p87 * s.dn[737][6]);
        let eq11_e1079_d_n7: f64 = (p.p87 * s.dn[737][7]);
        let eq11_e1079_d_n8: f64 = (p.p87 * s.dn[737][8]);
        let eq11_e1079_d_n9: f64 = (p.p87 * s.dn[737][9]);
        let eq11_e1079_d_n10: f64 = (p.p87 * s.dn[737][10]);
        let eq11_e1079_d_n11: f64 = (p.p87 * s.dn[737][11]);
        let eq11_e1079_d_n12: f64 = (p.p87 * s.dn[737][12]);
        let eq11_e1079_d_n13: f64 = (p.p87 * s.dn[737][13]);
        let eq11_e1079_d_n14: f64 = (p.p87 * s.dn[737][14]);
        let eq11_e1079_d_n15: f64 = (p.p87 * s.dn[737][15]);
        let eq11_e1079_d_n16: f64 = (p.p87 * s.dn[737][16]);
        let eq11_e1079_d_n17: f64 = (p.p87 * s.dn[737][17]);
        let eq11_e1079_d_b0: f64 = (p.p87 * s.db[737][0]);
        let eq11_e1079_d_b1: f64 = (p.p87 * s.db[737][1]);
        let eq11_e1079_d_b2: f64 = (p.p87 * s.db[737][2]);
        let eq11_e1079_d_b3: f64 = (p.p87 * s.db[737][3]);
        let eq11_e1079_d_b4: f64 = (p.p87 * s.db[737][4]);
        let eq11_e1079_d_b5: f64 = (p.p87 * s.db[737][5]);
        let eq11_e1079_d_b6: f64 = (p.p87 * s.db[737][6]);
        let eq11_e1079_d_b7: f64 = (p.p87 * s.db[737][7]);
        let eq11_e1079_d_b8: f64 = (p.p87 * s.db[737][8]);
        let eq11_value: f64 = eq11_e1079;
        let eq11_node_derivatives: [f64; 18] = [eq11_e1079_d_n0, eq11_e1079_d_n1, eq11_e1079_d_n2, eq11_e1079_d_n3, eq11_e1079_d_n4, eq11_e1079_d_n5, eq11_e1079_d_n6, eq11_e1079_d_n7, eq11_e1079_d_n8, eq11_e1079_d_n9, eq11_e1079_d_n10, eq11_e1079_d_n11, eq11_e1079_d_n12, eq11_e1079_d_n13, eq11_e1079_d_n14, eq11_e1079_d_n15, eq11_e1079_d_n16, eq11_e1079_d_n17];
        let eq11_branch_derivatives: [f64; 9] = [eq11_e1079_d_b0, eq11_e1079_d_b1, eq11_e1079_d_b2, eq11_e1079_d_b3, eq11_e1079_d_b4, eq11_e1079_d_b5, eq11_e1079_d_b6, eq11_e1079_d_b7, eq11_e1079_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
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
        let eq12_e1082: f64 = (p.p87 * s.v[862]);
        let eq12_e1082_d_n0: f64 = (p.p87 * s.dn[862][0]);
        let eq12_e1082_d_n1: f64 = (p.p87 * s.dn[862][1]);
        let eq12_e1082_d_n2: f64 = (p.p87 * s.dn[862][2]);
        let eq12_e1082_d_n3: f64 = (p.p87 * s.dn[862][3]);
        let eq12_e1082_d_n4: f64 = (p.p87 * s.dn[862][4]);
        let eq12_e1082_d_n5: f64 = (p.p87 * s.dn[862][5]);
        let eq12_e1082_d_n6: f64 = (p.p87 * s.dn[862][6]);
        let eq12_e1082_d_n7: f64 = (p.p87 * s.dn[862][7]);
        let eq12_e1082_d_n8: f64 = (p.p87 * s.dn[862][8]);
        let eq12_e1082_d_n9: f64 = (p.p87 * s.dn[862][9]);
        let eq12_e1082_d_n10: f64 = (p.p87 * s.dn[862][10]);
        let eq12_e1082_d_n11: f64 = (p.p87 * s.dn[862][11]);
        let eq12_e1082_d_n12: f64 = (p.p87 * s.dn[862][12]);
        let eq12_e1082_d_n13: f64 = (p.p87 * s.dn[862][13]);
        let eq12_e1082_d_n14: f64 = (p.p87 * s.dn[862][14]);
        let eq12_e1082_d_n15: f64 = (p.p87 * s.dn[862][15]);
        let eq12_e1082_d_n16: f64 = (p.p87 * s.dn[862][16]);
        let eq12_e1082_d_n17: f64 = (p.p87 * s.dn[862][17]);
        let eq12_e1082_d_b0: f64 = (p.p87 * s.db[862][0]);
        let eq12_e1082_d_b1: f64 = (p.p87 * s.db[862][1]);
        let eq12_e1082_d_b2: f64 = (p.p87 * s.db[862][2]);
        let eq12_e1082_d_b3: f64 = (p.p87 * s.db[862][3]);
        let eq12_e1082_d_b4: f64 = (p.p87 * s.db[862][4]);
        let eq12_e1082_d_b5: f64 = (p.p87 * s.db[862][5]);
        let eq12_e1082_d_b6: f64 = (p.p87 * s.db[862][6]);
        let eq12_e1082_d_b7: f64 = (p.p87 * s.db[862][7]);
        let eq12_e1082_d_b8: f64 = (p.p87 * s.db[862][8]);
        let eq12_value: f64 = eq12_e1082;
        let eq12_node_derivatives: [f64; 18] = [eq12_e1082_d_n0, eq12_e1082_d_n1, eq12_e1082_d_n2, eq12_e1082_d_n3, eq12_e1082_d_n4, eq12_e1082_d_n5, eq12_e1082_d_n6, eq12_e1082_d_n7, eq12_e1082_d_n8, eq12_e1082_d_n9, eq12_e1082_d_n10, eq12_e1082_d_n11, eq12_e1082_d_n12, eq12_e1082_d_n13, eq12_e1082_d_n14, eq12_e1082_d_n15, eq12_e1082_d_n16, eq12_e1082_d_n17];
        let eq12_branch_derivatives: [f64; 9] = [eq12_e1082_d_b0, eq12_e1082_d_b1, eq12_e1082_d_b2, eq12_e1082_d_b3, eq12_e1082_d_b4, eq12_e1082_d_b5, eq12_e1082_d_b6, eq12_e1082_d_b7, eq12_e1082_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[2]),
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
        let eq13_e1085: f64 = (p.p87 * s.v[861]);
        let eq13_e1085_d_n0: f64 = (p.p87 * s.dn[861][0]);
        let eq13_e1085_d_n1: f64 = (p.p87 * s.dn[861][1]);
        let eq13_e1085_d_n2: f64 = (p.p87 * s.dn[861][2]);
        let eq13_e1085_d_n3: f64 = (p.p87 * s.dn[861][3]);
        let eq13_e1085_d_n4: f64 = (p.p87 * s.dn[861][4]);
        let eq13_e1085_d_n5: f64 = (p.p87 * s.dn[861][5]);
        let eq13_e1085_d_n6: f64 = (p.p87 * s.dn[861][6]);
        let eq13_e1085_d_n7: f64 = (p.p87 * s.dn[861][7]);
        let eq13_e1085_d_n8: f64 = (p.p87 * s.dn[861][8]);
        let eq13_e1085_d_n9: f64 = (p.p87 * s.dn[861][9]);
        let eq13_e1085_d_n10: f64 = (p.p87 * s.dn[861][10]);
        let eq13_e1085_d_n11: f64 = (p.p87 * s.dn[861][11]);
        let eq13_e1085_d_n12: f64 = (p.p87 * s.dn[861][12]);
        let eq13_e1085_d_n13: f64 = (p.p87 * s.dn[861][13]);
        let eq13_e1085_d_n14: f64 = (p.p87 * s.dn[861][14]);
        let eq13_e1085_d_n15: f64 = (p.p87 * s.dn[861][15]);
        let eq13_e1085_d_n16: f64 = (p.p87 * s.dn[861][16]);
        let eq13_e1085_d_n17: f64 = (p.p87 * s.dn[861][17]);
        let eq13_e1085_d_b0: f64 = (p.p87 * s.db[861][0]);
        let eq13_e1085_d_b1: f64 = (p.p87 * s.db[861][1]);
        let eq13_e1085_d_b2: f64 = (p.p87 * s.db[861][2]);
        let eq13_e1085_d_b3: f64 = (p.p87 * s.db[861][3]);
        let eq13_e1085_d_b4: f64 = (p.p87 * s.db[861][4]);
        let eq13_e1085_d_b5: f64 = (p.p87 * s.db[861][5]);
        let eq13_e1085_d_b6: f64 = (p.p87 * s.db[861][6]);
        let eq13_e1085_d_b7: f64 = (p.p87 * s.db[861][7]);
        let eq13_e1085_d_b8: f64 = (p.p87 * s.db[861][8]);
        let eq13_value: f64 = eq13_e1085;
        let eq13_node_derivatives: [f64; 18] = [eq13_e1085_d_n0, eq13_e1085_d_n1, eq13_e1085_d_n2, eq13_e1085_d_n3, eq13_e1085_d_n4, eq13_e1085_d_n5, eq13_e1085_d_n6, eq13_e1085_d_n7, eq13_e1085_d_n8, eq13_e1085_d_n9, eq13_e1085_d_n10, eq13_e1085_d_n11, eq13_e1085_d_n12, eq13_e1085_d_n13, eq13_e1085_d_n14, eq13_e1085_d_n15, eq13_e1085_d_n16, eq13_e1085_d_n17];
        let eq13_branch_derivatives: [f64; 9] = [eq13_e1085_d_b0, eq13_e1085_d_b1, eq13_e1085_d_b2, eq13_e1085_d_b3, eq13_e1085_d_b4, eq13_e1085_d_b5, eq13_e1085_d_b6, eq13_e1085_d_b7, eq13_e1085_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[0]),
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
        let eq14_e1088: f64 = self.eval_ddt(3, s.v[66]);
        let eq14_e1088_d_n0: f64 = self.ddt_jacobian(s.dn[66][0]);
        let eq14_e1088_d_n1: f64 = self.ddt_jacobian(s.dn[66][1]);
        let eq14_e1088_d_n2: f64 = self.ddt_jacobian(s.dn[66][2]);
        let eq14_e1088_d_n3: f64 = self.ddt_jacobian(s.dn[66][3]);
        let eq14_e1088_d_n4: f64 = self.ddt_jacobian(s.dn[66][4]);
        let eq14_e1088_d_n5: f64 = self.ddt_jacobian(s.dn[66][5]);
        let eq14_e1088_d_n6: f64 = self.ddt_jacobian(s.dn[66][6]);
        let eq14_e1088_d_n7: f64 = self.ddt_jacobian(s.dn[66][7]);
        let eq14_e1088_d_n8: f64 = self.ddt_jacobian(s.dn[66][8]);
        let eq14_e1088_d_n9: f64 = self.ddt_jacobian(s.dn[66][9]);
        let eq14_e1088_d_n10: f64 = self.ddt_jacobian(s.dn[66][10]);
        let eq14_e1088_d_n11: f64 = self.ddt_jacobian(s.dn[66][11]);
        let eq14_e1088_d_n12: f64 = self.ddt_jacobian(s.dn[66][12]);
        let eq14_e1088_d_n13: f64 = self.ddt_jacobian(s.dn[66][13]);
        let eq14_e1088_d_n14: f64 = self.ddt_jacobian(s.dn[66][14]);
        let eq14_e1088_d_n15: f64 = self.ddt_jacobian(s.dn[66][15]);
        let eq14_e1088_d_n16: f64 = self.ddt_jacobian(s.dn[66][16]);
        let eq14_e1088_d_n17: f64 = self.ddt_jacobian(s.dn[66][17]);
        let eq14_e1088_d_b0: f64 = self.ddt_jacobian(s.db[66][0]);
        let eq14_e1088_d_b1: f64 = self.ddt_jacobian(s.db[66][1]);
        let eq14_e1088_d_b2: f64 = self.ddt_jacobian(s.db[66][2]);
        let eq14_e1088_d_b3: f64 = self.ddt_jacobian(s.db[66][3]);
        let eq14_e1088_d_b4: f64 = self.ddt_jacobian(s.db[66][4]);
        let eq14_e1088_d_b5: f64 = self.ddt_jacobian(s.db[66][5]);
        let eq14_e1088_d_b6: f64 = self.ddt_jacobian(s.db[66][6]);
        let eq14_e1088_d_b7: f64 = self.ddt_jacobian(s.db[66][7]);
        let eq14_e1088_d_b8: f64 = self.ddt_jacobian(s.db[66][8]);
        let eq14_e1089: f64 = (p.p87 * eq14_e1088);
        let eq14_e1089_d_n0: f64 = (p.p87 * eq14_e1088_d_n0);
        let eq14_e1089_d_n1: f64 = (p.p87 * eq14_e1088_d_n1);
        let eq14_e1089_d_n2: f64 = (p.p87 * eq14_e1088_d_n2);
        let eq14_e1089_d_n3: f64 = (p.p87 * eq14_e1088_d_n3);
        let eq14_e1089_d_n4: f64 = (p.p87 * eq14_e1088_d_n4);
        let eq14_e1089_d_n5: f64 = (p.p87 * eq14_e1088_d_n5);
        let eq14_e1089_d_n6: f64 = (p.p87 * eq14_e1088_d_n6);
        let eq14_e1089_d_n7: f64 = (p.p87 * eq14_e1088_d_n7);
        let eq14_e1089_d_n8: f64 = (p.p87 * eq14_e1088_d_n8);
        let eq14_e1089_d_n9: f64 = (p.p87 * eq14_e1088_d_n9);
        let eq14_e1089_d_n10: f64 = (p.p87 * eq14_e1088_d_n10);
        let eq14_e1089_d_n11: f64 = (p.p87 * eq14_e1088_d_n11);
        let eq14_e1089_d_n12: f64 = (p.p87 * eq14_e1088_d_n12);
        let eq14_e1089_d_n13: f64 = (p.p87 * eq14_e1088_d_n13);
        let eq14_e1089_d_n14: f64 = (p.p87 * eq14_e1088_d_n14);
        let eq14_e1089_d_n15: f64 = (p.p87 * eq14_e1088_d_n15);
        let eq14_e1089_d_n16: f64 = (p.p87 * eq14_e1088_d_n16);
        let eq14_e1089_d_n17: f64 = (p.p87 * eq14_e1088_d_n17);
        let eq14_e1089_d_b0: f64 = (p.p87 * eq14_e1088_d_b0);
        let eq14_e1089_d_b1: f64 = (p.p87 * eq14_e1088_d_b1);
        let eq14_e1089_d_b2: f64 = (p.p87 * eq14_e1088_d_b2);
        let eq14_e1089_d_b3: f64 = (p.p87 * eq14_e1088_d_b3);
        let eq14_e1089_d_b4: f64 = (p.p87 * eq14_e1088_d_b4);
        let eq14_e1089_d_b5: f64 = (p.p87 * eq14_e1088_d_b5);
        let eq14_e1089_d_b6: f64 = (p.p87 * eq14_e1088_d_b6);
        let eq14_e1089_d_b7: f64 = (p.p87 * eq14_e1088_d_b7);
        let eq14_e1089_d_b8: f64 = (p.p87 * eq14_e1088_d_b8);
        let eq14_value: f64 = eq14_e1089;
        let eq14_node_derivatives: [f64; 18] = [eq14_e1089_d_n0, eq14_e1089_d_n1, eq14_e1089_d_n2, eq14_e1089_d_n3, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n12, eq14_e1089_d_n13, eq14_e1089_d_n14, eq14_e1089_d_n15, eq14_e1089_d_n16, eq14_e1089_d_n17];
        let eq14_branch_derivatives: [f64; 9] = [eq14_e1089_d_b0, eq14_e1089_d_b1, eq14_e1089_d_b2, eq14_e1089_d_b3, eq14_e1089_d_b4, eq14_e1089_d_b5, eq14_e1089_d_b6, eq14_e1089_d_b7, eq14_e1089_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[2]),
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
        let eq15_e1092: f64 = self.eval_ddt(4, s.v[65]);
        let eq15_e1092_d_n0: f64 = self.ddt_jacobian(s.dn[65][0]);
        let eq15_e1092_d_n1: f64 = self.ddt_jacobian(s.dn[65][1]);
        let eq15_e1092_d_n2: f64 = self.ddt_jacobian(s.dn[65][2]);
        let eq15_e1092_d_n3: f64 = self.ddt_jacobian(s.dn[65][3]);
        let eq15_e1092_d_n4: f64 = self.ddt_jacobian(s.dn[65][4]);
        let eq15_e1092_d_n5: f64 = self.ddt_jacobian(s.dn[65][5]);
        let eq15_e1092_d_n6: f64 = self.ddt_jacobian(s.dn[65][6]);
        let eq15_e1092_d_n7: f64 = self.ddt_jacobian(s.dn[65][7]);
        let eq15_e1092_d_n8: f64 = self.ddt_jacobian(s.dn[65][8]);
        let eq15_e1092_d_n9: f64 = self.ddt_jacobian(s.dn[65][9]);
        let eq15_e1092_d_n10: f64 = self.ddt_jacobian(s.dn[65][10]);
        let eq15_e1092_d_n11: f64 = self.ddt_jacobian(s.dn[65][11]);
        let eq15_e1092_d_n12: f64 = self.ddt_jacobian(s.dn[65][12]);
        let eq15_e1092_d_n13: f64 = self.ddt_jacobian(s.dn[65][13]);
        let eq15_e1092_d_n14: f64 = self.ddt_jacobian(s.dn[65][14]);
        let eq15_e1092_d_n15: f64 = self.ddt_jacobian(s.dn[65][15]);
        let eq15_e1092_d_n16: f64 = self.ddt_jacobian(s.dn[65][16]);
        let eq15_e1092_d_n17: f64 = self.ddt_jacobian(s.dn[65][17]);
        let eq15_e1092_d_b0: f64 = self.ddt_jacobian(s.db[65][0]);
        let eq15_e1092_d_b1: f64 = self.ddt_jacobian(s.db[65][1]);
        let eq15_e1092_d_b2: f64 = self.ddt_jacobian(s.db[65][2]);
        let eq15_e1092_d_b3: f64 = self.ddt_jacobian(s.db[65][3]);
        let eq15_e1092_d_b4: f64 = self.ddt_jacobian(s.db[65][4]);
        let eq15_e1092_d_b5: f64 = self.ddt_jacobian(s.db[65][5]);
        let eq15_e1092_d_b6: f64 = self.ddt_jacobian(s.db[65][6]);
        let eq15_e1092_d_b7: f64 = self.ddt_jacobian(s.db[65][7]);
        let eq15_e1092_d_b8: f64 = self.ddt_jacobian(s.db[65][8]);
        let eq15_e1093: f64 = (p.p87 * eq15_e1092);
        let eq15_e1093_d_n0: f64 = (p.p87 * eq15_e1092_d_n0);
        let eq15_e1093_d_n1: f64 = (p.p87 * eq15_e1092_d_n1);
        let eq15_e1093_d_n2: f64 = (p.p87 * eq15_e1092_d_n2);
        let eq15_e1093_d_n3: f64 = (p.p87 * eq15_e1092_d_n3);
        let eq15_e1093_d_n4: f64 = (p.p87 * eq15_e1092_d_n4);
        let eq15_e1093_d_n5: f64 = (p.p87 * eq15_e1092_d_n5);
        let eq15_e1093_d_n6: f64 = (p.p87 * eq15_e1092_d_n6);
        let eq15_e1093_d_n7: f64 = (p.p87 * eq15_e1092_d_n7);
        let eq15_e1093_d_n8: f64 = (p.p87 * eq15_e1092_d_n8);
        let eq15_e1093_d_n9: f64 = (p.p87 * eq15_e1092_d_n9);
        let eq15_e1093_d_n10: f64 = (p.p87 * eq15_e1092_d_n10);
        let eq15_e1093_d_n11: f64 = (p.p87 * eq15_e1092_d_n11);
        let eq15_e1093_d_n12: f64 = (p.p87 * eq15_e1092_d_n12);
        let eq15_e1093_d_n13: f64 = (p.p87 * eq15_e1092_d_n13);
        let eq15_e1093_d_n14: f64 = (p.p87 * eq15_e1092_d_n14);
        let eq15_e1093_d_n15: f64 = (p.p87 * eq15_e1092_d_n15);
        let eq15_e1093_d_n16: f64 = (p.p87 * eq15_e1092_d_n16);
        let eq15_e1093_d_n17: f64 = (p.p87 * eq15_e1092_d_n17);
        let eq15_e1093_d_b0: f64 = (p.p87 * eq15_e1092_d_b0);
        let eq15_e1093_d_b1: f64 = (p.p87 * eq15_e1092_d_b1);
        let eq15_e1093_d_b2: f64 = (p.p87 * eq15_e1092_d_b2);
        let eq15_e1093_d_b3: f64 = (p.p87 * eq15_e1092_d_b3);
        let eq15_e1093_d_b4: f64 = (p.p87 * eq15_e1092_d_b4);
        let eq15_e1093_d_b5: f64 = (p.p87 * eq15_e1092_d_b5);
        let eq15_e1093_d_b6: f64 = (p.p87 * eq15_e1092_d_b6);
        let eq15_e1093_d_b7: f64 = (p.p87 * eq15_e1092_d_b7);
        let eq15_e1093_d_b8: f64 = (p.p87 * eq15_e1092_d_b8);
        let eq15_value: f64 = eq15_e1093;
        let eq15_node_derivatives: [f64; 18] = [eq15_e1093_d_n0, eq15_e1093_d_n1, eq15_e1093_d_n2, eq15_e1093_d_n3, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n12, eq15_e1093_d_n13, eq15_e1093_d_n14, eq15_e1093_d_n15, eq15_e1093_d_n16, eq15_e1093_d_n17];
        let eq15_branch_derivatives: [f64; 9] = [eq15_e1093_d_b0, eq15_e1093_d_b1, eq15_e1093_d_b2, eq15_e1093_d_b3, eq15_e1093_d_b4, eq15_e1093_d_b5, eq15_e1093_d_b6, eq15_e1093_d_b7, eq15_e1093_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[0]),
            self.multiplicity * (eq15_value),
            &nodes,
            &eq15_node_derivatives,
            &branches,
            &eq15_branch_derivatives,
            self.multiplicity,
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
        let (eq16_e1099, eq16_e1099_d_n0, eq16_e1099_d_n1, eq16_e1099_d_n2, eq16_e1099_d_n3, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n11, eq16_e1099_d_n12, eq16_e1099_d_n13, eq16_e1099_d_n14, eq16_e1099_d_n15, eq16_e1099_d_n16, eq16_e1099_d_n17, eq16_e1099_d_b0, eq16_e1099_d_b1, eq16_e1099_d_b2, eq16_e1099_d_b3, eq16_e1099_d_b4, eq16_e1099_d_b5, eq16_e1099_d_b6, eq16_e1099_d_b7, eq16_e1099_d_b8,) = {
    if (s.v[3405] != 0.0) {
        let eq16_e1097: f64 = (p.p87 * s.v[870]);
        let eq16_e1097_d_n0: f64 = (p.p87 * s.dn[870][0]);
        let eq16_e1097_d_n1: f64 = (p.p87 * s.dn[870][1]);
        let eq16_e1097_d_n2: f64 = (p.p87 * s.dn[870][2]);
        let eq16_e1097_d_n3: f64 = (p.p87 * s.dn[870][3]);
        let eq16_e1097_d_n4: f64 = (p.p87 * s.dn[870][4]);
        let eq16_e1097_d_n5: f64 = (p.p87 * s.dn[870][5]);
        let eq16_e1097_d_n6: f64 = (p.p87 * s.dn[870][6]);
        let eq16_e1097_d_n7: f64 = (p.p87 * s.dn[870][7]);
        let eq16_e1097_d_n8: f64 = (p.p87 * s.dn[870][8]);
        let eq16_e1097_d_n9: f64 = (p.p87 * s.dn[870][9]);
        let eq16_e1097_d_n10: f64 = (p.p87 * s.dn[870][10]);
        let eq16_e1097_d_n11: f64 = (p.p87 * s.dn[870][11]);
        let eq16_e1097_d_n12: f64 = (p.p87 * s.dn[870][12]);
        let eq16_e1097_d_n13: f64 = (p.p87 * s.dn[870][13]);
        let eq16_e1097_d_n14: f64 = (p.p87 * s.dn[870][14]);
        let eq16_e1097_d_n15: f64 = (p.p87 * s.dn[870][15]);
        let eq16_e1097_d_n16: f64 = (p.p87 * s.dn[870][16]);
        let eq16_e1097_d_n17: f64 = (p.p87 * s.dn[870][17]);
        let eq16_e1097_d_b0: f64 = (p.p87 * s.db[870][0]);
        let eq16_e1097_d_b1: f64 = (p.p87 * s.db[870][1]);
        let eq16_e1097_d_b2: f64 = (p.p87 * s.db[870][2]);
        let eq16_e1097_d_b3: f64 = (p.p87 * s.db[870][3]);
        let eq16_e1097_d_b4: f64 = (p.p87 * s.db[870][4]);
        let eq16_e1097_d_b5: f64 = (p.p87 * s.db[870][5]);
        let eq16_e1097_d_b6: f64 = (p.p87 * s.db[870][6]);
        let eq16_e1097_d_b7: f64 = (p.p87 * s.db[870][7]);
        let eq16_e1097_d_b8: f64 = (p.p87 * s.db[870][8]);
        (eq16_e1097, eq16_e1097_d_n0, eq16_e1097_d_n1, eq16_e1097_d_n2, eq16_e1097_d_n3, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, eq16_e1097_d_n12, eq16_e1097_d_n13, eq16_e1097_d_n14, eq16_e1097_d_n15, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_b0, eq16_e1097_d_b1, eq16_e1097_d_b2, eq16_e1097_d_b3, eq16_e1097_d_b4, eq16_e1097_d_b5, eq16_e1097_d_b6, eq16_e1097_d_b7, eq16_e1097_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1099;
        let eq16_node_derivatives: [f64; 18] = [eq16_e1099_d_n0, eq16_e1099_d_n1, eq16_e1099_d_n2, eq16_e1099_d_n3, eq16_e1099_d_n4, eq16_e1099_d_n5, eq16_e1099_d_n6, eq16_e1099_d_n7, eq16_e1099_d_n8, eq16_e1099_d_n9, eq16_e1099_d_n10, eq16_e1099_d_n11, eq16_e1099_d_n12, eq16_e1099_d_n13, eq16_e1099_d_n14, eq16_e1099_d_n15, eq16_e1099_d_n16, eq16_e1099_d_n17];
        let eq16_branch_derivatives: [f64; 9] = [eq16_e1099_d_b0, eq16_e1099_d_b1, eq16_e1099_d_b2, eq16_e1099_d_b3, eq16_e1099_d_b4, eq16_e1099_d_b5, eq16_e1099_d_b6, eq16_e1099_d_b7, eq16_e1099_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq16_value),
            &nodes,
            &eq16_node_derivatives,
            &branches,
            &eq16_branch_derivatives,
            self.multiplicity,
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
        let (eq17_e1105, eq17_e1105_d_n0, eq17_e1105_d_n1, eq17_e1105_d_n2, eq17_e1105_d_n3, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n11, eq17_e1105_d_n12, eq17_e1105_d_n13, eq17_e1105_d_n14, eq17_e1105_d_n15, eq17_e1105_d_n16, eq17_e1105_d_n17, eq17_e1105_d_b0, eq17_e1105_d_b1, eq17_e1105_d_b2, eq17_e1105_d_b3, eq17_e1105_d_b4, eq17_e1105_d_b5, eq17_e1105_d_b6, eq17_e1105_d_b7, eq17_e1105_d_b8,) = {
    if (s.v[3405] != 0.0) {
        let eq17_e1103: f64 = (p.p87 * s.v[869]);
        let eq17_e1103_d_n0: f64 = (p.p87 * s.dn[869][0]);
        let eq17_e1103_d_n1: f64 = (p.p87 * s.dn[869][1]);
        let eq17_e1103_d_n2: f64 = (p.p87 * s.dn[869][2]);
        let eq17_e1103_d_n3: f64 = (p.p87 * s.dn[869][3]);
        let eq17_e1103_d_n4: f64 = (p.p87 * s.dn[869][4]);
        let eq17_e1103_d_n5: f64 = (p.p87 * s.dn[869][5]);
        let eq17_e1103_d_n6: f64 = (p.p87 * s.dn[869][6]);
        let eq17_e1103_d_n7: f64 = (p.p87 * s.dn[869][7]);
        let eq17_e1103_d_n8: f64 = (p.p87 * s.dn[869][8]);
        let eq17_e1103_d_n9: f64 = (p.p87 * s.dn[869][9]);
        let eq17_e1103_d_n10: f64 = (p.p87 * s.dn[869][10]);
        let eq17_e1103_d_n11: f64 = (p.p87 * s.dn[869][11]);
        let eq17_e1103_d_n12: f64 = (p.p87 * s.dn[869][12]);
        let eq17_e1103_d_n13: f64 = (p.p87 * s.dn[869][13]);
        let eq17_e1103_d_n14: f64 = (p.p87 * s.dn[869][14]);
        let eq17_e1103_d_n15: f64 = (p.p87 * s.dn[869][15]);
        let eq17_e1103_d_n16: f64 = (p.p87 * s.dn[869][16]);
        let eq17_e1103_d_n17: f64 = (p.p87 * s.dn[869][17]);
        let eq17_e1103_d_b0: f64 = (p.p87 * s.db[869][0]);
        let eq17_e1103_d_b1: f64 = (p.p87 * s.db[869][1]);
        let eq17_e1103_d_b2: f64 = (p.p87 * s.db[869][2]);
        let eq17_e1103_d_b3: f64 = (p.p87 * s.db[869][3]);
        let eq17_e1103_d_b4: f64 = (p.p87 * s.db[869][4]);
        let eq17_e1103_d_b5: f64 = (p.p87 * s.db[869][5]);
        let eq17_e1103_d_b6: f64 = (p.p87 * s.db[869][6]);
        let eq17_e1103_d_b7: f64 = (p.p87 * s.db[869][7]);
        let eq17_e1103_d_b8: f64 = (p.p87 * s.db[869][8]);
        (eq17_e1103, eq17_e1103_d_n0, eq17_e1103_d_n1, eq17_e1103_d_n2, eq17_e1103_d_n3, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n12, eq17_e1103_d_n13, eq17_e1103_d_n14, eq17_e1103_d_n15, eq17_e1103_d_n16, eq17_e1103_d_n17, eq17_e1103_d_b0, eq17_e1103_d_b1, eq17_e1103_d_b2, eq17_e1103_d_b3, eq17_e1103_d_b4, eq17_e1103_d_b5, eq17_e1103_d_b6, eq17_e1103_d_b7, eq17_e1103_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1105;
        let eq17_node_derivatives: [f64; 18] = [eq17_e1105_d_n0, eq17_e1105_d_n1, eq17_e1105_d_n2, eq17_e1105_d_n3, eq17_e1105_d_n4, eq17_e1105_d_n5, eq17_e1105_d_n6, eq17_e1105_d_n7, eq17_e1105_d_n8, eq17_e1105_d_n9, eq17_e1105_d_n10, eq17_e1105_d_n11, eq17_e1105_d_n12, eq17_e1105_d_n13, eq17_e1105_d_n14, eq17_e1105_d_n15, eq17_e1105_d_n16, eq17_e1105_d_n17];
        let eq17_branch_derivatives: [f64; 9] = [eq17_e1105_d_b0, eq17_e1105_d_b1, eq17_e1105_d_b2, eq17_e1105_d_b3, eq17_e1105_d_b4, eq17_e1105_d_b5, eq17_e1105_d_b6, eq17_e1105_d_b7, eq17_e1105_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq17_value),
            &nodes,
            &eq17_node_derivatives,
            &branches,
            &eq17_branch_derivatives,
            self.multiplicity,
        );
    }
}

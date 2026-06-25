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
        let (eq1_e795, eq1_e795_d_n0, eq1_e795_d_n1, eq1_e795_d_n2, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8,) = {
    if (s.v[662] != 0.0) {
        let eq1_e792: f64 = (s.v[199] + s.v[211]);
        let eq1_e792_d_n0: f64 = (s.dn[199][0] + s.dn[211][0]);
        let eq1_e792_d_n1: f64 = (s.dn[199][1] + s.dn[211][1]);
        let eq1_e792_d_n2: f64 = (s.dn[199][2] + s.dn[211][2]);
        let eq1_e792_d_n3: f64 = (s.dn[199][3] + s.dn[211][3]);
        let eq1_e792_d_n4: f64 = (s.dn[199][4] + s.dn[211][4]);
        let eq1_e792_d_n5: f64 = (s.dn[199][5] + s.dn[211][5]);
        let eq1_e792_d_n6: f64 = (s.dn[199][6] + s.dn[211][6]);
        let eq1_e792_d_n7: f64 = (s.dn[199][7] + s.dn[211][7]);
        let eq1_e792_d_n8: f64 = (s.dn[199][8] + s.dn[211][8]);
        let eq1_e793: f64 = (s.v[212] * eq1_e792);
        let eq1_e793_d_n0: f64 = ((s.dn[212][0] * eq1_e792) + (s.v[212] * eq1_e792_d_n0));
        let eq1_e793_d_n1: f64 = ((s.dn[212][1] * eq1_e792) + (s.v[212] * eq1_e792_d_n1));
        let eq1_e793_d_n2: f64 = ((s.dn[212][2] * eq1_e792) + (s.v[212] * eq1_e792_d_n2));
        let eq1_e793_d_n3: f64 = ((s.dn[212][3] * eq1_e792) + (s.v[212] * eq1_e792_d_n3));
        let eq1_e793_d_n4: f64 = ((s.dn[212][4] * eq1_e792) + (s.v[212] * eq1_e792_d_n4));
        let eq1_e793_d_n5: f64 = ((s.dn[212][5] * eq1_e792) + (s.v[212] * eq1_e792_d_n5));
        let eq1_e793_d_n6: f64 = ((s.dn[212][6] * eq1_e792) + (s.v[212] * eq1_e792_d_n6));
        let eq1_e793_d_n7: f64 = ((s.dn[212][7] * eq1_e792) + (s.v[212] * eq1_e792_d_n7));
        let eq1_e793_d_n8: f64 = ((s.dn[212][8] * eq1_e792) + (s.v[212] * eq1_e792_d_n8));
        (eq1_e793, eq1_e793_d_n0, eq1_e793_d_n1, eq1_e793_d_n2, eq1_e793_d_n3, eq1_e793_d_n4, eq1_e793_d_n5, eq1_e793_d_n6, eq1_e793_d_n7, eq1_e793_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e795;
        let eq1_node_derivatives: [f64; 9] = [eq1_e795_d_n0, eq1_e795_d_n1, eq1_e795_d_n2, eq1_e795_d_n3, eq1_e795_d_n4, eq1_e795_d_n5, eq1_e795_d_n6, eq1_e795_d_n7, eq1_e795_d_n8];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq1_value),
            &nodes,
            &eq1_node_derivatives,
            &branches,
            &eq1_branch_derivatives,
            self.multiplicity,
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
        let (eq2_e801, eq2_e801_d_n0, eq2_e801_d_n1, eq2_e801_d_n2, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8,) = {
    if (s.v[662] != 0.0) {
        let eq2_e799: f64 = (s.v[212] * s.v[198]);
        let eq2_e799_d_n0: f64 = ((s.dn[212][0] * s.v[198]) + (s.v[212] * s.dn[198][0]));
        let eq2_e799_d_n1: f64 = ((s.dn[212][1] * s.v[198]) + (s.v[212] * s.dn[198][1]));
        let eq2_e799_d_n2: f64 = ((s.dn[212][2] * s.v[198]) + (s.v[212] * s.dn[198][2]));
        let eq2_e799_d_n3: f64 = ((s.dn[212][3] * s.v[198]) + (s.v[212] * s.dn[198][3]));
        let eq2_e799_d_n4: f64 = ((s.dn[212][4] * s.v[198]) + (s.v[212] * s.dn[198][4]));
        let eq2_e799_d_n5: f64 = ((s.dn[212][5] * s.v[198]) + (s.v[212] * s.dn[198][5]));
        let eq2_e799_d_n6: f64 = ((s.dn[212][6] * s.v[198]) + (s.v[212] * s.dn[198][6]));
        let eq2_e799_d_n7: f64 = ((s.dn[212][7] * s.v[198]) + (s.v[212] * s.dn[198][7]));
        let eq2_e799_d_n8: f64 = ((s.dn[212][8] * s.v[198]) + (s.v[212] * s.dn[198][8]));
        (eq2_e799, eq2_e799_d_n0, eq2_e799_d_n1, eq2_e799_d_n2, eq2_e799_d_n3, eq2_e799_d_n4, eq2_e799_d_n5, eq2_e799_d_n6, eq2_e799_d_n7, eq2_e799_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e801;
        let eq2_node_derivatives: [f64; 9] = [eq2_e801_d_n0, eq2_e801_d_n1, eq2_e801_d_n2, eq2_e801_d_n3, eq2_e801_d_n4, eq2_e801_d_n5, eq2_e801_d_n6, eq2_e801_d_n7, eq2_e801_d_n8];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq3_e809, eq3_e809_d_n0, eq3_e809_d_n1, eq3_e809_d_n2, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8,) = {
    if (s.v[662] != 0.0) {
        let eq3_e806: f64 = (s.v[193] + s.v[201]);
        let eq3_e806_d_n0: f64 = (s.dn[193][0] + s.dn[201][0]);
        let eq3_e806_d_n1: f64 = (s.dn[193][1] + s.dn[201][1]);
        let eq3_e806_d_n2: f64 = (s.dn[193][2] + s.dn[201][2]);
        let eq3_e806_d_n3: f64 = (s.dn[193][3] + s.dn[201][3]);
        let eq3_e806_d_n4: f64 = (s.dn[193][4] + s.dn[201][4]);
        let eq3_e806_d_n5: f64 = (s.dn[193][5] + s.dn[201][5]);
        let eq3_e806_d_n6: f64 = (s.dn[193][6] + s.dn[201][6]);
        let eq3_e806_d_n7: f64 = (s.dn[193][7] + s.dn[201][7]);
        let eq3_e806_d_n8: f64 = (s.dn[193][8] + s.dn[201][8]);
        let eq3_e807: f64 = (s.v[212] * eq3_e806);
        let eq3_e807_d_n0: f64 = ((s.dn[212][0] * eq3_e806) + (s.v[212] * eq3_e806_d_n0));
        let eq3_e807_d_n1: f64 = ((s.dn[212][1] * eq3_e806) + (s.v[212] * eq3_e806_d_n1));
        let eq3_e807_d_n2: f64 = ((s.dn[212][2] * eq3_e806) + (s.v[212] * eq3_e806_d_n2));
        let eq3_e807_d_n3: f64 = ((s.dn[212][3] * eq3_e806) + (s.v[212] * eq3_e806_d_n3));
        let eq3_e807_d_n4: f64 = ((s.dn[212][4] * eq3_e806) + (s.v[212] * eq3_e806_d_n4));
        let eq3_e807_d_n5: f64 = ((s.dn[212][5] * eq3_e806) + (s.v[212] * eq3_e806_d_n5));
        let eq3_e807_d_n6: f64 = ((s.dn[212][6] * eq3_e806) + (s.v[212] * eq3_e806_d_n6));
        let eq3_e807_d_n7: f64 = ((s.dn[212][7] * eq3_e806) + (s.v[212] * eq3_e806_d_n7));
        let eq3_e807_d_n8: f64 = ((s.dn[212][8] * eq3_e806) + (s.v[212] * eq3_e806_d_n8));
        (eq3_e807, eq3_e807_d_n0, eq3_e807_d_n1, eq3_e807_d_n2, eq3_e807_d_n3, eq3_e807_d_n4, eq3_e807_d_n5, eq3_e807_d_n6, eq3_e807_d_n7, eq3_e807_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e809;
        let eq3_node_derivatives: [f64; 9] = [eq3_e809_d_n0, eq3_e809_d_n1, eq3_e809_d_n2, eq3_e809_d_n3, eq3_e809_d_n4, eq3_e809_d_n5, eq3_e809_d_n6, eq3_e809_d_n7, eq3_e809_d_n8];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
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
        let (eq4_e817, eq4_e817_d_n0, eq4_e817_d_n1, eq4_e817_d_n2, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8,) = {
    if (s.v[662] != 0.0) {
        let eq4_e814: f64 = (s.v[194] + s.v[202]);
        let eq4_e814_d_n0: f64 = (s.dn[194][0] + s.dn[202][0]);
        let eq4_e814_d_n1: f64 = (s.dn[194][1] + s.dn[202][1]);
        let eq4_e814_d_n2: f64 = (s.dn[194][2] + s.dn[202][2]);
        let eq4_e814_d_n3: f64 = (s.dn[194][3] + s.dn[202][3]);
        let eq4_e814_d_n4: f64 = (s.dn[194][4] + s.dn[202][4]);
        let eq4_e814_d_n5: f64 = (s.dn[194][5] + s.dn[202][5]);
        let eq4_e814_d_n6: f64 = (s.dn[194][6] + s.dn[202][6]);
        let eq4_e814_d_n7: f64 = (s.dn[194][7] + s.dn[202][7]);
        let eq4_e814_d_n8: f64 = (s.dn[194][8] + s.dn[202][8]);
        let eq4_e815: f64 = (s.v[212] * eq4_e814);
        let eq4_e815_d_n0: f64 = ((s.dn[212][0] * eq4_e814) + (s.v[212] * eq4_e814_d_n0));
        let eq4_e815_d_n1: f64 = ((s.dn[212][1] * eq4_e814) + (s.v[212] * eq4_e814_d_n1));
        let eq4_e815_d_n2: f64 = ((s.dn[212][2] * eq4_e814) + (s.v[212] * eq4_e814_d_n2));
        let eq4_e815_d_n3: f64 = ((s.dn[212][3] * eq4_e814) + (s.v[212] * eq4_e814_d_n3));
        let eq4_e815_d_n4: f64 = ((s.dn[212][4] * eq4_e814) + (s.v[212] * eq4_e814_d_n4));
        let eq4_e815_d_n5: f64 = ((s.dn[212][5] * eq4_e814) + (s.v[212] * eq4_e814_d_n5));
        let eq4_e815_d_n6: f64 = ((s.dn[212][6] * eq4_e814) + (s.v[212] * eq4_e814_d_n6));
        let eq4_e815_d_n7: f64 = ((s.dn[212][7] * eq4_e814) + (s.v[212] * eq4_e814_d_n7));
        let eq4_e815_d_n8: f64 = ((s.dn[212][8] * eq4_e814) + (s.v[212] * eq4_e814_d_n8));
        (eq4_e815, eq4_e815_d_n0, eq4_e815_d_n1, eq4_e815_d_n2, eq4_e815_d_n3, eq4_e815_d_n4, eq4_e815_d_n5, eq4_e815_d_n6, eq4_e815_d_n7, eq4_e815_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e817;
        let eq4_node_derivatives: [f64; 9] = [eq4_e817_d_n0, eq4_e817_d_n1, eq4_e817_d_n2, eq4_e817_d_n3, eq4_e817_d_n4, eq4_e817_d_n5, eq4_e817_d_n6, eq4_e817_d_n7, eq4_e817_d_n8];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq5_e830, eq5_e830_d_n0, eq5_e830_d_n1, eq5_e830_d_n2, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8,) = {
    if (!(s.v[662] != 0.0)) {
        let eq5_e822: f64 = (s.v[212] * s.v[214]);
        let eq5_e822_d_n0: f64 = ((s.dn[212][0] * s.v[214]) + (s.v[212] * s.dn[214][0]));
        let eq5_e822_d_n1: f64 = ((s.dn[212][1] * s.v[214]) + (s.v[212] * s.dn[214][1]));
        let eq5_e822_d_n2: f64 = ((s.dn[212][2] * s.v[214]) + (s.v[212] * s.dn[214][2]));
        let eq5_e822_d_n3: f64 = ((s.dn[212][3] * s.v[214]) + (s.v[212] * s.dn[214][3]));
        let eq5_e822_d_n4: f64 = ((s.dn[212][4] * s.v[214]) + (s.v[212] * s.dn[214][4]));
        let eq5_e822_d_n5: f64 = ((s.dn[212][5] * s.v[214]) + (s.v[212] * s.dn[214][5]));
        let eq5_e822_d_n6: f64 = ((s.dn[212][6] * s.v[214]) + (s.v[212] * s.dn[214][6]));
        let eq5_e822_d_n7: f64 = ((s.dn[212][7] * s.v[214]) + (s.v[212] * s.dn[214][7]));
        let eq5_e822_d_n8: f64 = ((s.dn[212][8] * s.v[214]) + (s.v[212] * s.dn[214][8]));
        let eq5_e825: f64 = 1e-12;
        let eq5_e827: f64 = (eq5_e825 * (nv6 - nv5));
        let eq5_e827_d_n5: f64 = (-eq5_e825);
        let eq5_e828: f64 = (eq5_e822 + eq5_e827);
        let eq5_e828_d_n5: f64 = (eq5_e822_d_n5 + eq5_e827_d_n5);
        let eq5_e828_d_n6: f64 = (eq5_e822_d_n6 + eq5_e825);
        (eq5_e828, eq5_e822_d_n0, eq5_e822_d_n1, eq5_e822_d_n2, eq5_e822_d_n3, eq5_e822_d_n4, eq5_e828_d_n5, eq5_e828_d_n6, eq5_e822_d_n7, eq5_e822_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e830;
        let eq5_node_derivatives: [f64; 9] = [eq5_e830_d_n0, eq5_e830_d_n1, eq5_e830_d_n2, eq5_e830_d_n3, eq5_e830_d_n4, eq5_e830_d_n5, eq5_e830_d_n6, eq5_e830_d_n7, eq5_e830_d_n8];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq6_e839, eq6_e839_d_n0, eq6_e839_d_n1, eq6_e839_d_n2, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8,) = {
    if (!(s.v[662] != 0.0)) {
        let eq6_e836: f64 = (s.v[199] + s.v[211]);
        let eq6_e836_d_n0: f64 = (s.dn[199][0] + s.dn[211][0]);
        let eq6_e836_d_n1: f64 = (s.dn[199][1] + s.dn[211][1]);
        let eq6_e836_d_n2: f64 = (s.dn[199][2] + s.dn[211][2]);
        let eq6_e836_d_n3: f64 = (s.dn[199][3] + s.dn[211][3]);
        let eq6_e836_d_n4: f64 = (s.dn[199][4] + s.dn[211][4]);
        let eq6_e836_d_n5: f64 = (s.dn[199][5] + s.dn[211][5]);
        let eq6_e836_d_n6: f64 = (s.dn[199][6] + s.dn[211][6]);
        let eq6_e836_d_n7: f64 = (s.dn[199][7] + s.dn[211][7]);
        let eq6_e836_d_n8: f64 = (s.dn[199][8] + s.dn[211][8]);
        let eq6_e837: f64 = (s.v[212] * eq6_e836);
        let eq6_e837_d_n0: f64 = ((s.dn[212][0] * eq6_e836) + (s.v[212] * eq6_e836_d_n0));
        let eq6_e837_d_n1: f64 = ((s.dn[212][1] * eq6_e836) + (s.v[212] * eq6_e836_d_n1));
        let eq6_e837_d_n2: f64 = ((s.dn[212][2] * eq6_e836) + (s.v[212] * eq6_e836_d_n2));
        let eq6_e837_d_n3: f64 = ((s.dn[212][3] * eq6_e836) + (s.v[212] * eq6_e836_d_n3));
        let eq6_e837_d_n4: f64 = ((s.dn[212][4] * eq6_e836) + (s.v[212] * eq6_e836_d_n4));
        let eq6_e837_d_n5: f64 = ((s.dn[212][5] * eq6_e836) + (s.v[212] * eq6_e836_d_n5));
        let eq6_e837_d_n6: f64 = ((s.dn[212][6] * eq6_e836) + (s.v[212] * eq6_e836_d_n6));
        let eq6_e837_d_n7: f64 = ((s.dn[212][7] * eq6_e836) + (s.v[212] * eq6_e836_d_n7));
        let eq6_e837_d_n8: f64 = ((s.dn[212][8] * eq6_e836) + (s.v[212] * eq6_e836_d_n8));
        (eq6_e837, eq6_e837_d_n0, eq6_e837_d_n1, eq6_e837_d_n2, eq6_e837_d_n3, eq6_e837_d_n4, eq6_e837_d_n5, eq6_e837_d_n6, eq6_e837_d_n7, eq6_e837_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e839;
        let eq6_node_derivatives: [f64; 9] = [eq6_e839_d_n0, eq6_e839_d_n1, eq6_e839_d_n2, eq6_e839_d_n3, eq6_e839_d_n4, eq6_e839_d_n5, eq6_e839_d_n6, eq6_e839_d_n7, eq6_e839_d_n8];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let (eq7_e846, eq7_e846_d_n0, eq7_e846_d_n1, eq7_e846_d_n2, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8,) = {
    if (!(s.v[662] != 0.0)) {
        let eq7_e844: f64 = (s.v[212] * s.v[198]);
        let eq7_e844_d_n0: f64 = ((s.dn[212][0] * s.v[198]) + (s.v[212] * s.dn[198][0]));
        let eq7_e844_d_n1: f64 = ((s.dn[212][1] * s.v[198]) + (s.v[212] * s.dn[198][1]));
        let eq7_e844_d_n2: f64 = ((s.dn[212][2] * s.v[198]) + (s.v[212] * s.dn[198][2]));
        let eq7_e844_d_n3: f64 = ((s.dn[212][3] * s.v[198]) + (s.v[212] * s.dn[198][3]));
        let eq7_e844_d_n4: f64 = ((s.dn[212][4] * s.v[198]) + (s.v[212] * s.dn[198][4]));
        let eq7_e844_d_n5: f64 = ((s.dn[212][5] * s.v[198]) + (s.v[212] * s.dn[198][5]));
        let eq7_e844_d_n6: f64 = ((s.dn[212][6] * s.v[198]) + (s.v[212] * s.dn[198][6]));
        let eq7_e844_d_n7: f64 = ((s.dn[212][7] * s.v[198]) + (s.v[212] * s.dn[198][7]));
        let eq7_e844_d_n8: f64 = ((s.dn[212][8] * s.v[198]) + (s.v[212] * s.dn[198][8]));
        (eq7_e844, eq7_e844_d_n0, eq7_e844_d_n1, eq7_e844_d_n2, eq7_e844_d_n3, eq7_e844_d_n4, eq7_e844_d_n5, eq7_e844_d_n6, eq7_e844_d_n7, eq7_e844_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e846;
        let eq7_node_derivatives: [f64; 9] = [eq7_e846_d_n0, eq7_e846_d_n1, eq7_e846_d_n2, eq7_e846_d_n3, eq7_e846_d_n4, eq7_e846_d_n5, eq7_e846_d_n6, eq7_e846_d_n7, eq7_e846_d_n8];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let (eq8_e855, eq8_e855_d_n0, eq8_e855_d_n1, eq8_e855_d_n2, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8,) = {
    if (!(s.v[662] != 0.0)) {
        let eq8_e852: f64 = (s.v[193] + s.v[201]);
        let eq8_e852_d_n0: f64 = (s.dn[193][0] + s.dn[201][0]);
        let eq8_e852_d_n1: f64 = (s.dn[193][1] + s.dn[201][1]);
        let eq8_e852_d_n2: f64 = (s.dn[193][2] + s.dn[201][2]);
        let eq8_e852_d_n3: f64 = (s.dn[193][3] + s.dn[201][3]);
        let eq8_e852_d_n4: f64 = (s.dn[193][4] + s.dn[201][4]);
        let eq8_e852_d_n5: f64 = (s.dn[193][5] + s.dn[201][5]);
        let eq8_e852_d_n6: f64 = (s.dn[193][6] + s.dn[201][6]);
        let eq8_e852_d_n7: f64 = (s.dn[193][7] + s.dn[201][7]);
        let eq8_e852_d_n8: f64 = (s.dn[193][8] + s.dn[201][8]);
        let eq8_e853: f64 = (s.v[212] * eq8_e852);
        let eq8_e853_d_n0: f64 = ((s.dn[212][0] * eq8_e852) + (s.v[212] * eq8_e852_d_n0));
        let eq8_e853_d_n1: f64 = ((s.dn[212][1] * eq8_e852) + (s.v[212] * eq8_e852_d_n1));
        let eq8_e853_d_n2: f64 = ((s.dn[212][2] * eq8_e852) + (s.v[212] * eq8_e852_d_n2));
        let eq8_e853_d_n3: f64 = ((s.dn[212][3] * eq8_e852) + (s.v[212] * eq8_e852_d_n3));
        let eq8_e853_d_n4: f64 = ((s.dn[212][4] * eq8_e852) + (s.v[212] * eq8_e852_d_n4));
        let eq8_e853_d_n5: f64 = ((s.dn[212][5] * eq8_e852) + (s.v[212] * eq8_e852_d_n5));
        let eq8_e853_d_n6: f64 = ((s.dn[212][6] * eq8_e852) + (s.v[212] * eq8_e852_d_n6));
        let eq8_e853_d_n7: f64 = ((s.dn[212][7] * eq8_e852) + (s.v[212] * eq8_e852_d_n7));
        let eq8_e853_d_n8: f64 = ((s.dn[212][8] * eq8_e852) + (s.v[212] * eq8_e852_d_n8));
        (eq8_e853, eq8_e853_d_n0, eq8_e853_d_n1, eq8_e853_d_n2, eq8_e853_d_n3, eq8_e853_d_n4, eq8_e853_d_n5, eq8_e853_d_n6, eq8_e853_d_n7, eq8_e853_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e855;
        let eq8_node_derivatives: [f64; 9] = [eq8_e855_d_n0, eq8_e855_d_n1, eq8_e855_d_n2, eq8_e855_d_n3, eq8_e855_d_n4, eq8_e855_d_n5, eq8_e855_d_n6, eq8_e855_d_n7, eq8_e855_d_n8];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
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
        let (eq9_e864, eq9_e864_d_n0, eq9_e864_d_n1, eq9_e864_d_n2, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8,) = {
    if (!(s.v[662] != 0.0)) {
        let eq9_e861: f64 = (s.v[194] + s.v[202]);
        let eq9_e861_d_n0: f64 = (s.dn[194][0] + s.dn[202][0]);
        let eq9_e861_d_n1: f64 = (s.dn[194][1] + s.dn[202][1]);
        let eq9_e861_d_n2: f64 = (s.dn[194][2] + s.dn[202][2]);
        let eq9_e861_d_n3: f64 = (s.dn[194][3] + s.dn[202][3]);
        let eq9_e861_d_n4: f64 = (s.dn[194][4] + s.dn[202][4]);
        let eq9_e861_d_n5: f64 = (s.dn[194][5] + s.dn[202][5]);
        let eq9_e861_d_n6: f64 = (s.dn[194][6] + s.dn[202][6]);
        let eq9_e861_d_n7: f64 = (s.dn[194][7] + s.dn[202][7]);
        let eq9_e861_d_n8: f64 = (s.dn[194][8] + s.dn[202][8]);
        let eq9_e862: f64 = (s.v[212] * eq9_e861);
        let eq9_e862_d_n0: f64 = ((s.dn[212][0] * eq9_e861) + (s.v[212] * eq9_e861_d_n0));
        let eq9_e862_d_n1: f64 = ((s.dn[212][1] * eq9_e861) + (s.v[212] * eq9_e861_d_n1));
        let eq9_e862_d_n2: f64 = ((s.dn[212][2] * eq9_e861) + (s.v[212] * eq9_e861_d_n2));
        let eq9_e862_d_n3: f64 = ((s.dn[212][3] * eq9_e861) + (s.v[212] * eq9_e861_d_n3));
        let eq9_e862_d_n4: f64 = ((s.dn[212][4] * eq9_e861) + (s.v[212] * eq9_e861_d_n4));
        let eq9_e862_d_n5: f64 = ((s.dn[212][5] * eq9_e861) + (s.v[212] * eq9_e861_d_n5));
        let eq9_e862_d_n6: f64 = ((s.dn[212][6] * eq9_e861) + (s.v[212] * eq9_e861_d_n6));
        let eq9_e862_d_n7: f64 = ((s.dn[212][7] * eq9_e861) + (s.v[212] * eq9_e861_d_n7));
        let eq9_e862_d_n8: f64 = ((s.dn[212][8] * eq9_e861) + (s.v[212] * eq9_e861_d_n8));
        (eq9_e862, eq9_e862_d_n0, eq9_e862_d_n1, eq9_e862_d_n2, eq9_e862_d_n3, eq9_e862_d_n4, eq9_e862_d_n5, eq9_e862_d_n6, eq9_e862_d_n7, eq9_e862_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e864;
        let eq9_node_derivatives: [f64; 9] = [eq9_e864_d_n0, eq9_e864_d_n1, eq9_e864_d_n2, eq9_e864_d_n3, eq9_e864_d_n4, eq9_e864_d_n5, eq9_e864_d_n6, eq9_e864_d_n7, eq9_e864_d_n8];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let eq10_e867: f64 = (s.v[212] * s.v[187]);
        let eq10_e867_d_n0: f64 = ((s.dn[212][0] * s.v[187]) + (s.v[212] * s.dn[187][0]));
        let eq10_e867_d_n1: f64 = ((s.dn[212][1] * s.v[187]) + (s.v[212] * s.dn[187][1]));
        let eq10_e867_d_n2: f64 = ((s.dn[212][2] * s.v[187]) + (s.v[212] * s.dn[187][2]));
        let eq10_e867_d_n3: f64 = ((s.dn[212][3] * s.v[187]) + (s.v[212] * s.dn[187][3]));
        let eq10_e867_d_n4: f64 = ((s.dn[212][4] * s.v[187]) + (s.v[212] * s.dn[187][4]));
        let eq10_e867_d_n5: f64 = ((s.dn[212][5] * s.v[187]) + (s.v[212] * s.dn[187][5]));
        let eq10_e867_d_n6: f64 = ((s.dn[212][6] * s.v[187]) + (s.v[212] * s.dn[187][6]));
        let eq10_e867_d_n7: f64 = ((s.dn[212][7] * s.v[187]) + (s.v[212] * s.dn[187][7]));
        let eq10_e867_d_n8: f64 = ((s.dn[212][8] * s.v[187]) + (s.v[212] * s.dn[187][8]));
        let eq10_value: f64 = eq10_e867;
        let eq10_node_derivatives: [f64; 9] = [eq10_e867_d_n0, eq10_e867_d_n1, eq10_e867_d_n2, eq10_e867_d_n3, eq10_e867_d_n4, eq10_e867_d_n5, eq10_e867_d_n6, eq10_e867_d_n7, eq10_e867_d_n8];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
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
        let eq11_e870: f64 = (s.v[212] * s.v[188]);
        let eq11_e870_d_n0: f64 = ((s.dn[212][0] * s.v[188]) + (s.v[212] * s.dn[188][0]));
        let eq11_e870_d_n1: f64 = ((s.dn[212][1] * s.v[188]) + (s.v[212] * s.dn[188][1]));
        let eq11_e870_d_n2: f64 = ((s.dn[212][2] * s.v[188]) + (s.v[212] * s.dn[188][2]));
        let eq11_e870_d_n3: f64 = ((s.dn[212][3] * s.v[188]) + (s.v[212] * s.dn[188][3]));
        let eq11_e870_d_n4: f64 = ((s.dn[212][4] * s.v[188]) + (s.v[212] * s.dn[188][4]));
        let eq11_e870_d_n5: f64 = ((s.dn[212][5] * s.v[188]) + (s.v[212] * s.dn[188][5]));
        let eq11_e870_d_n6: f64 = ((s.dn[212][6] * s.v[188]) + (s.v[212] * s.dn[188][6]));
        let eq11_e870_d_n7: f64 = ((s.dn[212][7] * s.v[188]) + (s.v[212] * s.dn[188][7]));
        let eq11_e870_d_n8: f64 = ((s.dn[212][8] * s.v[188]) + (s.v[212] * s.dn[188][8]));
        let eq11_value: f64 = eq11_e870;
        let eq11_node_derivatives: [f64; 9] = [eq11_e870_d_n0, eq11_e870_d_n1, eq11_e870_d_n2, eq11_e870_d_n3, eq11_e870_d_n4, eq11_e870_d_n5, eq11_e870_d_n6, eq11_e870_d_n7, eq11_e870_d_n8];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
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
        let eq12_e873: f64 = self.eval_ddt(0, s.v[223]);
        let eq12_e873_d_n0: f64 = self.ddt_jacobian(s.dn[223][0]);
        let eq12_e873_d_n1: f64 = self.ddt_jacobian(s.dn[223][1]);
        let eq12_e873_d_n2: f64 = self.ddt_jacobian(s.dn[223][2]);
        let eq12_e873_d_n3: f64 = self.ddt_jacobian(s.dn[223][3]);
        let eq12_e873_d_n4: f64 = self.ddt_jacobian(s.dn[223][4]);
        let eq12_e873_d_n5: f64 = self.ddt_jacobian(s.dn[223][5]);
        let eq12_e873_d_n6: f64 = self.ddt_jacobian(s.dn[223][6]);
        let eq12_e873_d_n7: f64 = self.ddt_jacobian(s.dn[223][7]);
        let eq12_e873_d_n8: f64 = self.ddt_jacobian(s.dn[223][8]);
        let eq12_e874: f64 = (s.v[212] * eq12_e873);
        let eq12_e874_d_n0: f64 = ((s.dn[212][0] * eq12_e873) + (s.v[212] * eq12_e873_d_n0));
        let eq12_e874_d_n1: f64 = ((s.dn[212][1] * eq12_e873) + (s.v[212] * eq12_e873_d_n1));
        let eq12_e874_d_n2: f64 = ((s.dn[212][2] * eq12_e873) + (s.v[212] * eq12_e873_d_n2));
        let eq12_e874_d_n3: f64 = ((s.dn[212][3] * eq12_e873) + (s.v[212] * eq12_e873_d_n3));
        let eq12_e874_d_n4: f64 = ((s.dn[212][4] * eq12_e873) + (s.v[212] * eq12_e873_d_n4));
        let eq12_e874_d_n5: f64 = ((s.dn[212][5] * eq12_e873) + (s.v[212] * eq12_e873_d_n5));
        let eq12_e874_d_n6: f64 = ((s.dn[212][6] * eq12_e873) + (s.v[212] * eq12_e873_d_n6));
        let eq12_e874_d_n7: f64 = ((s.dn[212][7] * eq12_e873) + (s.v[212] * eq12_e873_d_n7));
        let eq12_e874_d_n8: f64 = ((s.dn[212][8] * eq12_e873) + (s.v[212] * eq12_e873_d_n8));
        let eq12_value: f64 = eq12_e874;
        let eq12_node_derivatives: [f64; 9] = [eq12_e874_d_n0, eq12_e874_d_n1, eq12_e874_d_n2, eq12_e874_d_n3, eq12_e874_d_n4, eq12_e874_d_n5, eq12_e874_d_n6, eq12_e874_d_n7, eq12_e874_d_n8];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let eq13_e876: f64 = self.eval_ddt(1, s.v[224]);
        let eq13_e876_d_n0: f64 = self.ddt_jacobian(s.dn[224][0]);
        let eq13_e876_d_n1: f64 = self.ddt_jacobian(s.dn[224][1]);
        let eq13_e876_d_n2: f64 = self.ddt_jacobian(s.dn[224][2]);
        let eq13_e876_d_n3: f64 = self.ddt_jacobian(s.dn[224][3]);
        let eq13_e876_d_n4: f64 = self.ddt_jacobian(s.dn[224][4]);
        let eq13_e876_d_n5: f64 = self.ddt_jacobian(s.dn[224][5]);
        let eq13_e876_d_n6: f64 = self.ddt_jacobian(s.dn[224][6]);
        let eq13_e876_d_n7: f64 = self.ddt_jacobian(s.dn[224][7]);
        let eq13_e876_d_n8: f64 = self.ddt_jacobian(s.dn[224][8]);
        let eq13_value: f64 = eq13_e876;
        let eq13_node_derivatives: [f64; 9] = [eq13_e876_d_n0, eq13_e876_d_n1, eq13_e876_d_n2, eq13_e876_d_n3, eq13_e876_d_n4, eq13_e876_d_n5, eq13_e876_d_n6, eq13_e876_d_n7, eq13_e876_d_n8];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
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
        let eq14_e879: f64 = self.eval_ddt(2, s.v[225]);
        let eq14_e879_d_n0: f64 = self.ddt_jacobian(s.dn[225][0]);
        let eq14_e879_d_n1: f64 = self.ddt_jacobian(s.dn[225][1]);
        let eq14_e879_d_n2: f64 = self.ddt_jacobian(s.dn[225][2]);
        let eq14_e879_d_n3: f64 = self.ddt_jacobian(s.dn[225][3]);
        let eq14_e879_d_n4: f64 = self.ddt_jacobian(s.dn[225][4]);
        let eq14_e879_d_n5: f64 = self.ddt_jacobian(s.dn[225][5]);
        let eq14_e879_d_n6: f64 = self.ddt_jacobian(s.dn[225][6]);
        let eq14_e879_d_n7: f64 = self.ddt_jacobian(s.dn[225][7]);
        let eq14_e879_d_n8: f64 = self.ddt_jacobian(s.dn[225][8]);
        let eq14_e880: f64 = (s.v[212] * eq14_e879);
        let eq14_e880_d_n0: f64 = ((s.dn[212][0] * eq14_e879) + (s.v[212] * eq14_e879_d_n0));
        let eq14_e880_d_n1: f64 = ((s.dn[212][1] * eq14_e879) + (s.v[212] * eq14_e879_d_n1));
        let eq14_e880_d_n2: f64 = ((s.dn[212][2] * eq14_e879) + (s.v[212] * eq14_e879_d_n2));
        let eq14_e880_d_n3: f64 = ((s.dn[212][3] * eq14_e879) + (s.v[212] * eq14_e879_d_n3));
        let eq14_e880_d_n4: f64 = ((s.dn[212][4] * eq14_e879) + (s.v[212] * eq14_e879_d_n4));
        let eq14_e880_d_n5: f64 = ((s.dn[212][5] * eq14_e879) + (s.v[212] * eq14_e879_d_n5));
        let eq14_e880_d_n6: f64 = ((s.dn[212][6] * eq14_e879) + (s.v[212] * eq14_e879_d_n6));
        let eq14_e880_d_n7: f64 = ((s.dn[212][7] * eq14_e879) + (s.v[212] * eq14_e879_d_n7));
        let eq14_e880_d_n8: f64 = ((s.dn[212][8] * eq14_e879) + (s.v[212] * eq14_e879_d_n8));
        let eq14_value: f64 = eq14_e880;
        let eq14_node_derivatives: [f64; 9] = [eq14_e880_d_n0, eq14_e880_d_n1, eq14_e880_d_n2, eq14_e880_d_n3, eq14_e880_d_n4, eq14_e880_d_n5, eq14_e880_d_n6, eq14_e880_d_n7, eq14_e880_d_n8];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[6]),
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
        let eq15_e882: f64 = self.eval_ddt(3, s.v[226]);
        let eq15_e882_d_n0: f64 = self.ddt_jacobian(s.dn[226][0]);
        let eq15_e882_d_n1: f64 = self.ddt_jacobian(s.dn[226][1]);
        let eq15_e882_d_n2: f64 = self.ddt_jacobian(s.dn[226][2]);
        let eq15_e882_d_n3: f64 = self.ddt_jacobian(s.dn[226][3]);
        let eq15_e882_d_n4: f64 = self.ddt_jacobian(s.dn[226][4]);
        let eq15_e882_d_n5: f64 = self.ddt_jacobian(s.dn[226][5]);
        let eq15_e882_d_n6: f64 = self.ddt_jacobian(s.dn[226][6]);
        let eq15_e882_d_n7: f64 = self.ddt_jacobian(s.dn[226][7]);
        let eq15_e882_d_n8: f64 = self.ddt_jacobian(s.dn[226][8]);
        let eq15_value: f64 = eq15_e882;
        let eq15_node_derivatives: [f64; 9] = [eq15_e882_d_n0, eq15_e882_d_n1, eq15_e882_d_n2, eq15_e882_d_n3, eq15_e882_d_n4, eq15_e882_d_n5, eq15_e882_d_n6, eq15_e882_d_n7, eq15_e882_d_n8];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
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
        let eq16_e884: f64 = self.eval_ddt(4, s.v[227]);
        let eq16_e884_d_n0: f64 = self.ddt_jacobian(s.dn[227][0]);
        let eq16_e884_d_n1: f64 = self.ddt_jacobian(s.dn[227][1]);
        let eq16_e884_d_n2: f64 = self.ddt_jacobian(s.dn[227][2]);
        let eq16_e884_d_n3: f64 = self.ddt_jacobian(s.dn[227][3]);
        let eq16_e884_d_n4: f64 = self.ddt_jacobian(s.dn[227][4]);
        let eq16_e884_d_n5: f64 = self.ddt_jacobian(s.dn[227][5]);
        let eq16_e884_d_n6: f64 = self.ddt_jacobian(s.dn[227][6]);
        let eq16_e884_d_n7: f64 = self.ddt_jacobian(s.dn[227][7]);
        let eq16_e884_d_n8: f64 = self.ddt_jacobian(s.dn[227][8]);
        let eq16_value: f64 = eq16_e884;
        let eq16_node_derivatives: [f64; 9] = [eq16_e884_d_n0, eq16_e884_d_n1, eq16_e884_d_n2, eq16_e884_d_n3, eq16_e884_d_n4, eq16_e884_d_n5, eq16_e884_d_n6, eq16_e884_d_n7, eq16_e884_d_n8];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq16_value),
            &nodes,
            &eq16_node_derivatives,
            &branches,
            &eq16_branch_derivatives,
            self.multiplicity,
        );
    }
}

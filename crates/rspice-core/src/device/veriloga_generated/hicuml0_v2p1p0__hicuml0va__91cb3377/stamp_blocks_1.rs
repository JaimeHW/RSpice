#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq7_e152: f64 = self.eval_ddt(4, s.v[197]);
        let eq7_e152_d_n0: f64 = self.ddt_jacobian(s.dn[197][0]);
        let eq7_e152_d_n1: f64 = self.ddt_jacobian(s.dn[197][1]);
        let eq7_e152_d_n2: f64 = self.ddt_jacobian(s.dn[197][2]);
        let eq7_e152_d_n3: f64 = self.ddt_jacobian(s.dn[197][3]);
        let eq7_e152_d_n4: f64 = self.ddt_jacobian(s.dn[197][4]);
        let eq7_e152_d_n5: f64 = self.ddt_jacobian(s.dn[197][5]);
        let eq7_e152_d_n6: f64 = self.ddt_jacobian(s.dn[197][6]);
        let eq7_e152_d_n7: f64 = self.ddt_jacobian(s.dn[197][7]);
        let eq7_e152_d_n8: f64 = self.ddt_jacobian(s.dn[197][8]);
        let eq7_e152_d_n9: f64 = self.ddt_jacobian(s.dn[197][9]);
        let eq7_e152_d_b0: f64 = self.ddt_jacobian(s.db[197][0]);
        let eq7_e152_d_b1: f64 = self.ddt_jacobian(s.db[197][1]);
        let eq7_e152_d_b2: f64 = self.ddt_jacobian(s.db[197][2]);
        let eq7_e152_d_b3: f64 = self.ddt_jacobian(s.db[197][3]);
        let eq7_value: f64 = eq7_e152;
        let eq7_node_derivatives: [f64; 10] = [eq7_e152_d_n0, eq7_e152_d_n1, eq7_e152_d_n2, eq7_e152_d_n3, eq7_e152_d_n4, eq7_e152_d_n5, eq7_e152_d_n6, eq7_e152_d_n7, eq7_e152_d_n8, eq7_e152_d_n9];
        let eq7_branch_derivatives: [f64; 4] = [eq7_e152_d_b0, eq7_e152_d_b1, eq7_e152_d_b2, eq7_e152_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[1]),
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
        let (eq8_e158, eq8_e158_d_n0, eq8_e158_d_n1, eq8_e158_d_n2, eq8_e158_d_n3, eq8_e158_d_n4, eq8_e158_d_n5, eq8_e158_d_n6, eq8_e158_d_n7, eq8_e158_d_n8, eq8_e158_d_n9, eq8_e158_d_b0, eq8_e158_d_b1, eq8_e158_d_b2, eq8_e158_d_b3,) = {
    if (s.v[356] != 0.0) {
        let eq8_e156: f64 = (s.v[188] / s.v[41]);
        let eq8_e156_d_n0: f64 = (((s.dn[188][0] * s.v[41]) - (s.v[188] * s.dn[41][0])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n1: f64 = (((s.dn[188][1] * s.v[41]) - (s.v[188] * s.dn[41][1])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n2: f64 = (((s.dn[188][2] * s.v[41]) - (s.v[188] * s.dn[41][2])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n3: f64 = (((s.dn[188][3] * s.v[41]) - (s.v[188] * s.dn[41][3])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n4: f64 = (((s.dn[188][4] * s.v[41]) - (s.v[188] * s.dn[41][4])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n5: f64 = (((s.dn[188][5] * s.v[41]) - (s.v[188] * s.dn[41][5])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n6: f64 = (((s.dn[188][6] * s.v[41]) - (s.v[188] * s.dn[41][6])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n7: f64 = (((s.dn[188][7] * s.v[41]) - (s.v[188] * s.dn[41][7])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n8: f64 = (((s.dn[188][8] * s.v[41]) - (s.v[188] * s.dn[41][8])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_n9: f64 = (((s.dn[188][9] * s.v[41]) - (s.v[188] * s.dn[41][9])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_b0: f64 = (((s.db[188][0] * s.v[41]) - (s.v[188] * s.db[41][0])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_b1: f64 = (((s.db[188][1] * s.v[41]) - (s.v[188] * s.db[41][1])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_b2: f64 = (((s.db[188][2] * s.v[41]) - (s.v[188] * s.db[41][2])) / (s.v[41] * s.v[41]));
        let eq8_e156_d_b3: f64 = (((s.db[188][3] * s.v[41]) - (s.v[188] * s.db[41][3])) / (s.v[41] * s.v[41]));
        (eq8_e156, eq8_e156_d_n0, eq8_e156_d_n1, eq8_e156_d_n2, eq8_e156_d_n3, eq8_e156_d_n4, eq8_e156_d_n5, eq8_e156_d_n6, eq8_e156_d_n7, eq8_e156_d_n8, eq8_e156_d_n9, eq8_e156_d_b0, eq8_e156_d_b1, eq8_e156_d_b2, eq8_e156_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e158;
        let eq8_node_derivatives: [f64; 10] = [eq8_e158_d_n0, eq8_e158_d_n1, eq8_e158_d_n2, eq8_e158_d_n3, eq8_e158_d_n4, eq8_e158_d_n5, eq8_e158_d_n6, eq8_e158_d_n7, eq8_e158_d_n8, eq8_e158_d_n9];
        let eq8_branch_derivatives: [f64; 4] = [eq8_e158_d_b0, eq8_e158_d_b1, eq8_e158_d_b2, eq8_e158_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[2]),
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
        let (eq9_e163,) = {
    if (!(s.v[356] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq9_value: f64 = eq9_e163;
        stamper.stamp_potential(
            branches[0],
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
        let (eq10_e169, eq10_e169_d_n0, eq10_e169_d_n1, eq10_e169_d_n2, eq10_e169_d_n3, eq10_e169_d_n4, eq10_e169_d_n5, eq10_e169_d_n6, eq10_e169_d_n7, eq10_e169_d_n8, eq10_e169_d_n9, eq10_e169_d_b0, eq10_e169_d_b1, eq10_e169_d_b2, eq10_e169_d_b3,) = {
    if (s.v[357] != 0.0) {
        let eq10_e167: f64 = (s.v[190] / s.v[40]);
        let eq10_e167_d_n0: f64 = (((s.dn[190][0] * s.v[40]) - (s.v[190] * s.dn[40][0])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n1: f64 = (((s.dn[190][1] * s.v[40]) - (s.v[190] * s.dn[40][1])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n2: f64 = (((s.dn[190][2] * s.v[40]) - (s.v[190] * s.dn[40][2])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n3: f64 = (((s.dn[190][3] * s.v[40]) - (s.v[190] * s.dn[40][3])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n4: f64 = (((s.dn[190][4] * s.v[40]) - (s.v[190] * s.dn[40][4])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n5: f64 = (((s.dn[190][5] * s.v[40]) - (s.v[190] * s.dn[40][5])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n6: f64 = (((s.dn[190][6] * s.v[40]) - (s.v[190] * s.dn[40][6])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n7: f64 = (((s.dn[190][7] * s.v[40]) - (s.v[190] * s.dn[40][7])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n8: f64 = (((s.dn[190][8] * s.v[40]) - (s.v[190] * s.dn[40][8])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_n9: f64 = (((s.dn[190][9] * s.v[40]) - (s.v[190] * s.dn[40][9])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_b0: f64 = (((s.db[190][0] * s.v[40]) - (s.v[190] * s.db[40][0])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_b1: f64 = (((s.db[190][1] * s.v[40]) - (s.v[190] * s.db[40][1])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_b2: f64 = (((s.db[190][2] * s.v[40]) - (s.v[190] * s.db[40][2])) / (s.v[40] * s.v[40]));
        let eq10_e167_d_b3: f64 = (((s.db[190][3] * s.v[40]) - (s.v[190] * s.db[40][3])) / (s.v[40] * s.v[40]));
        (eq10_e167, eq10_e167_d_n0, eq10_e167_d_n1, eq10_e167_d_n2, eq10_e167_d_n3, eq10_e167_d_n4, eq10_e167_d_n5, eq10_e167_d_n6, eq10_e167_d_n7, eq10_e167_d_n8, eq10_e167_d_n9, eq10_e167_d_b0, eq10_e167_d_b1, eq10_e167_d_b2, eq10_e167_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e169;
        let eq10_node_derivatives: [f64; 10] = [eq10_e169_d_n0, eq10_e169_d_n1, eq10_e169_d_n2, eq10_e169_d_n3, eq10_e169_d_n4, eq10_e169_d_n5, eq10_e169_d_n6, eq10_e169_d_n7, eq10_e169_d_n8, eq10_e169_d_n9];
        let eq10_branch_derivatives: [f64; 4] = [eq10_e169_d_b0, eq10_e169_d_b1, eq10_e169_d_b2, eq10_e169_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[0]),
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
        let (eq11_e174,) = {
    if (!(s.v[357] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e174;
        stamper.stamp_potential(
            branches[1],
            eq11_value,
            &[
            ],
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
        let (eq12_e180, eq12_e180_d_n0, eq12_e180_d_n1, eq12_e180_d_n2, eq12_e180_d_n3, eq12_e180_d_n4, eq12_e180_d_n5, eq12_e180_d_n6, eq12_e180_d_n7, eq12_e180_d_n8, eq12_e180_d_n9, eq12_e180_d_b0, eq12_e180_d_b1, eq12_e180_d_b2, eq12_e180_d_b3,) = {
    if (s.v[358] != 0.0) {
        let eq12_e178: f64 = (s.v[189] / s.v[156]);
        let eq12_e178_d_n0: f64 = (((s.dn[189][0] * s.v[156]) - (s.v[189] * s.dn[156][0])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n1: f64 = (((s.dn[189][1] * s.v[156]) - (s.v[189] * s.dn[156][1])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n2: f64 = (((s.dn[189][2] * s.v[156]) - (s.v[189] * s.dn[156][2])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n3: f64 = (((s.dn[189][3] * s.v[156]) - (s.v[189] * s.dn[156][3])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n4: f64 = (((s.dn[189][4] * s.v[156]) - (s.v[189] * s.dn[156][4])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n5: f64 = (((s.dn[189][5] * s.v[156]) - (s.v[189] * s.dn[156][5])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n6: f64 = (((s.dn[189][6] * s.v[156]) - (s.v[189] * s.dn[156][6])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n7: f64 = (((s.dn[189][7] * s.v[156]) - (s.v[189] * s.dn[156][7])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n8: f64 = (((s.dn[189][8] * s.v[156]) - (s.v[189] * s.dn[156][8])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_n9: f64 = (((s.dn[189][9] * s.v[156]) - (s.v[189] * s.dn[156][9])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_b0: f64 = (((s.db[189][0] * s.v[156]) - (s.v[189] * s.db[156][0])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_b1: f64 = (((s.db[189][1] * s.v[156]) - (s.v[189] * s.db[156][1])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_b2: f64 = (((s.db[189][2] * s.v[156]) - (s.v[189] * s.db[156][2])) / (s.v[156] * s.v[156]));
        let eq12_e178_d_b3: f64 = (((s.db[189][3] * s.v[156]) - (s.v[189] * s.db[156][3])) / (s.v[156] * s.v[156]));
        (eq12_e178, eq12_e178_d_n0, eq12_e178_d_n1, eq12_e178_d_n2, eq12_e178_d_n3, eq12_e178_d_n4, eq12_e178_d_n5, eq12_e178_d_n6, eq12_e178_d_n7, eq12_e178_d_n8, eq12_e178_d_n9, eq12_e178_d_b0, eq12_e178_d_b1, eq12_e178_d_b2, eq12_e178_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e180;
        let eq12_node_derivatives: [f64; 10] = [eq12_e180_d_n0, eq12_e180_d_n1, eq12_e180_d_n2, eq12_e180_d_n3, eq12_e180_d_n4, eq12_e180_d_n5, eq12_e180_d_n6, eq12_e180_d_n7, eq12_e180_d_n8, eq12_e180_d_n9];
        let eq12_branch_derivatives: [f64; 4] = [eq12_e180_d_b0, eq12_e180_d_b1, eq12_e180_d_b2, eq12_e180_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[1]),
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
        let (eq13_e185,) = {
    if (!(s.v[358] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq13_value: f64 = eq13_e185;
        stamper.stamp_potential(
            branches[2],
            eq13_value,
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
        let eq14_value: f64 = s.v[194];
        let eq14_node_derivatives: [f64; 10] = [s.dn[194][0], s.dn[194][1], s.dn[194][2], s.dn[194][3], s.dn[194][4], s.dn[194][5], s.dn[194][6], s.dn[194][7], s.dn[194][8], s.dn[194][9]];
        let eq14_branch_derivatives: [f64; 4] = [s.db[194][0], s.db[194][1], s.db[194][2], s.db[194][3]];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let eq15_e188: f64 = self.eval_ddt(5, s.v[198]);
        let eq15_e188_d_n0: f64 = self.ddt_jacobian(s.dn[198][0]);
        let eq15_e188_d_n1: f64 = self.ddt_jacobian(s.dn[198][1]);
        let eq15_e188_d_n2: f64 = self.ddt_jacobian(s.dn[198][2]);
        let eq15_e188_d_n3: f64 = self.ddt_jacobian(s.dn[198][3]);
        let eq15_e188_d_n4: f64 = self.ddt_jacobian(s.dn[198][4]);
        let eq15_e188_d_n5: f64 = self.ddt_jacobian(s.dn[198][5]);
        let eq15_e188_d_n6: f64 = self.ddt_jacobian(s.dn[198][6]);
        let eq15_e188_d_n7: f64 = self.ddt_jacobian(s.dn[198][7]);
        let eq15_e188_d_n8: f64 = self.ddt_jacobian(s.dn[198][8]);
        let eq15_e188_d_n9: f64 = self.ddt_jacobian(s.dn[198][9]);
        let eq15_e188_d_b0: f64 = self.ddt_jacobian(s.db[198][0]);
        let eq15_e188_d_b1: f64 = self.ddt_jacobian(s.db[198][1]);
        let eq15_e188_d_b2: f64 = self.ddt_jacobian(s.db[198][2]);
        let eq15_e188_d_b3: f64 = self.ddt_jacobian(s.db[198][3]);
        let eq15_value: f64 = eq15_e188;
        let eq15_node_derivatives: [f64; 10] = [eq15_e188_d_n0, eq15_e188_d_n1, eq15_e188_d_n2, eq15_e188_d_n3, eq15_e188_d_n4, eq15_e188_d_n5, eq15_e188_d_n6, eq15_e188_d_n7, eq15_e188_d_n8, eq15_e188_d_n9];
        let eq15_branch_derivatives: [f64; 4] = [eq15_e188_d_b0, eq15_e188_d_b1, eq15_e188_d_b2, eq15_e188_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
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
        let eq16_value: f64 = s.v[195];
        let eq16_node_derivatives: [f64; 10] = [s.dn[195][0], s.dn[195][1], s.dn[195][2], s.dn[195][3], s.dn[195][4], s.dn[195][5], s.dn[195][6], s.dn[195][7], s.dn[195][8], s.dn[195][9]];
        let eq16_branch_derivatives: [f64; 4] = [s.db[195][0], s.db[195][1], s.db[195][2], s.db[195][3]];
        stamper.stamp_current_dense(
            Some(nodes[6]),
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
        let eq17_e191: f64 = self.eval_ddt(6, s.v[199]);
        let eq17_e191_d_n0: f64 = self.ddt_jacobian(s.dn[199][0]);
        let eq17_e191_d_n1: f64 = self.ddt_jacobian(s.dn[199][1]);
        let eq17_e191_d_n2: f64 = self.ddt_jacobian(s.dn[199][2]);
        let eq17_e191_d_n3: f64 = self.ddt_jacobian(s.dn[199][3]);
        let eq17_e191_d_n4: f64 = self.ddt_jacobian(s.dn[199][4]);
        let eq17_e191_d_n5: f64 = self.ddt_jacobian(s.dn[199][5]);
        let eq17_e191_d_n6: f64 = self.ddt_jacobian(s.dn[199][6]);
        let eq17_e191_d_n7: f64 = self.ddt_jacobian(s.dn[199][7]);
        let eq17_e191_d_n8: f64 = self.ddt_jacobian(s.dn[199][8]);
        let eq17_e191_d_n9: f64 = self.ddt_jacobian(s.dn[199][9]);
        let eq17_e191_d_b0: f64 = self.ddt_jacobian(s.db[199][0]);
        let eq17_e191_d_b1: f64 = self.ddt_jacobian(s.db[199][1]);
        let eq17_e191_d_b2: f64 = self.ddt_jacobian(s.db[199][2]);
        let eq17_e191_d_b3: f64 = self.ddt_jacobian(s.db[199][3]);
        let eq17_value: f64 = eq17_e191;
        let eq17_node_derivatives: [f64; 10] = [eq17_e191_d_n0, eq17_e191_d_n1, eq17_e191_d_n2, eq17_e191_d_n3, eq17_e191_d_n4, eq17_e191_d_n5, eq17_e191_d_n6, eq17_e191_d_n7, eq17_e191_d_n8, eq17_e191_d_n9];
        let eq17_branch_derivatives: [f64; 4] = [eq17_e191_d_b0, eq17_e191_d_b1, eq17_e191_d_b2, eq17_e191_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq17_value),
            &nodes,
            &eq17_node_derivatives,
            &branches,
            &eq17_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq18_value: f64 = s.v[132];
        let eq18_node_derivatives: [f64; 10] = [s.dn[132][0], s.dn[132][1], s.dn[132][2], s.dn[132][3], s.dn[132][4], s.dn[132][5], s.dn[132][6], s.dn[132][7], s.dn[132][8], s.dn[132][9]];
        let eq18_branch_derivatives: [f64; 4] = [s.db[132][0], s.db[132][1], s.db[132][2], s.db[132][3]];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq18_value),
            &nodes,
            &eq18_node_derivatives,
            &branches,
            &eq18_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_19_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq19_e196,) = {
    if (s.v[360] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e196;
        stamper.stamp_potential(
            branches[3],
            eq19_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_20_block_0(
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
        let (eq20_e205, eq20_e205_d_n0, eq20_e205_d_n1, eq20_e205_d_n2, eq20_e205_d_n3, eq20_e205_d_n4, eq20_e205_d_n5, eq20_e205_d_n6, eq20_e205_d_n7, eq20_e205_d_n8, eq20_e205_d_n9, eq20_e205_d_b0, eq20_e205_d_b1, eq20_e205_d_b2, eq20_e205_d_b3,) = {
    if (!(s.v[360] != 0.0)) {
        let eq20_e201: f64 = ((nv4 - 0.0) / s.v[166]);
        let eq20_e201_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[166][0]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[166][1]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[166][2]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[166][3]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n4: f64 = ((s.v[166] - ((nv4 - 0.0) * s.dn[166][4])) / (s.v[166] * s.v[166]));
        let eq20_e201_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[166][5]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[166][6]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[166][7]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[166][8]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[166][9]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_b0: f64 = (-(((nv4 - 0.0) * s.db[166][0]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_b1: f64 = (-(((nv4 - 0.0) * s.db[166][1]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_b2: f64 = (-(((nv4 - 0.0) * s.db[166][2]) / (s.v[166] * s.v[166])));
        let eq20_e201_d_b3: f64 = (-(((nv4 - 0.0) * s.db[166][3]) / (s.v[166] * s.v[166])));
        let eq20_e203: f64 = (eq20_e201 - s.v[165]);
        let eq20_e203_d_n0: f64 = (eq20_e201_d_n0 - s.dn[165][0]);
        let eq20_e203_d_n1: f64 = (eq20_e201_d_n1 - s.dn[165][1]);
        let eq20_e203_d_n2: f64 = (eq20_e201_d_n2 - s.dn[165][2]);
        let eq20_e203_d_n3: f64 = (eq20_e201_d_n3 - s.dn[165][3]);
        let eq20_e203_d_n4: f64 = (eq20_e201_d_n4 - s.dn[165][4]);
        let eq20_e203_d_n5: f64 = (eq20_e201_d_n5 - s.dn[165][5]);
        let eq20_e203_d_n6: f64 = (eq20_e201_d_n6 - s.dn[165][6]);
        let eq20_e203_d_n7: f64 = (eq20_e201_d_n7 - s.dn[165][7]);
        let eq20_e203_d_n8: f64 = (eq20_e201_d_n8 - s.dn[165][8]);
        let eq20_e203_d_n9: f64 = (eq20_e201_d_n9 - s.dn[165][9]);
        let eq20_e203_d_b0: f64 = (eq20_e201_d_b0 - s.db[165][0]);
        let eq20_e203_d_b1: f64 = (eq20_e201_d_b1 - s.db[165][1]);
        let eq20_e203_d_b2: f64 = (eq20_e201_d_b2 - s.db[165][2]);
        let eq20_e203_d_b3: f64 = (eq20_e201_d_b3 - s.db[165][3]);
        (eq20_e203, eq20_e203_d_n0, eq20_e203_d_n1, eq20_e203_d_n2, eq20_e203_d_n3, eq20_e203_d_n4, eq20_e203_d_n5, eq20_e203_d_n6, eq20_e203_d_n7, eq20_e203_d_n8, eq20_e203_d_n9, eq20_e203_d_b0, eq20_e203_d_b1, eq20_e203_d_b2, eq20_e203_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e205;
        let eq20_node_derivatives: [f64; 10] = [eq20_e205_d_n0, eq20_e205_d_n1, eq20_e205_d_n2, eq20_e205_d_n3, eq20_e205_d_n4, eq20_e205_d_n5, eq20_e205_d_n6, eq20_e205_d_n7, eq20_e205_d_n8, eq20_e205_d_n9];
        let eq20_branch_derivatives: [f64; 4] = [eq20_e205_d_b0, eq20_e205_d_b1, eq20_e205_d_b2, eq20_e205_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq20_value),
            &nodes,
            &eq20_node_derivatives,
            &branches,
            &eq20_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_21_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq21_e210, eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9, eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3,) = {
    if (!(s.v[360] != 0.0)) {
        (s.v[167], s.dn[167][0], s.dn[167][1], s.dn[167][2], s.dn[167][3], s.dn[167][4], s.dn[167][5], s.dn[167][6], s.dn[167][7], s.dn[167][8], s.dn[167][9], s.db[167][0], s.db[167][1], s.db[167][2], s.db[167][3],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e210;
        let eq21_node_derivatives: [f64; 10] = [eq21_e210_d_n0, eq21_e210_d_n1, eq21_e210_d_n2, eq21_e210_d_n3, eq21_e210_d_n4, eq21_e210_d_n5, eq21_e210_d_n6, eq21_e210_d_n7, eq21_e210_d_n8, eq21_e210_d_n9];
        let eq21_branch_derivatives: [f64; 4] = [eq21_e210_d_b0, eq21_e210_d_b1, eq21_e210_d_b2, eq21_e210_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[4]),
            None,
            self.multiplicity * (eq21_value),
            &nodes,
            &eq21_node_derivatives,
            &branches,
            &eq21_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_22_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq22_value: f64 = s.v[209];
        let eq22_node_derivatives: [f64; 10] = [s.dn[209][0], s.dn[209][1], s.dn[209][2], s.dn[209][3], s.dn[209][4], s.dn[209][5], s.dn[209][6], s.dn[209][7], s.dn[209][8], s.dn[209][9]];
        let eq22_branch_derivatives: [f64; 4] = [s.db[209][0], s.db[209][1], s.db[209][2], s.db[209][3]];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            None,
            self.multiplicity * (eq22_value),
            &nodes,
            &eq22_node_derivatives,
            &branches,
            &eq22_branch_derivatives,
            self.multiplicity,
        );
    }
}

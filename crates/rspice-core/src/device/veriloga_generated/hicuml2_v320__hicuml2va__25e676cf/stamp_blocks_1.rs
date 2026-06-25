#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_block_7(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[494] != 0.0) && (!(s.v[495] != 0.0))) {
            s.store_scalar(94, 1.0);
        }

        s.copy_ad(242, 181);

        s.v[507] = if (s.v[234] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[507] != 0.0) {
            s.store_ad(504, &A::voltage(ctx, &nodes, Some(10), None));
        }

        if (s.v[507] != 0.0) {
            s.store_ad(505, &A::voltage(ctx, &nodes, Some(11), None));
        }

        if (s.v[507] != 0.0) {
            s.store_scale(239, 504, (p.p88 * p.p66));
        }

        if (s.v[507] != 0.0) {
            s.store_scale_ad(240, A::scale(s.ad_value(505), (p.p88 * 0.3333333333333333)), p.p66);
        }

        if (s.v[507] != 0.0) {
            s.store_ad(503, &A::voltage(ctx, &nodes, Some(12), None));
        }

        if (s.v[507] != 0.0) {
            s.store_scale(236, 503, (p.p87 * p.p66));
        }

        if (s.v[507] != 0.0) {
            s.copy_ad(242, 503);
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(239, 0.0);
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(240, 0.0);
        }

        if (!(s.v[507] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        s.v[508] = if ((p.p89 >= p.p149) && (p.p89 > 0.0)) { 1.0 } else { 0.0 };

        s.v[509] = if (p.p93 > 0.0) { 1.0 } else { 0.0 };

        s.v[517] = if ((p.p102 >= p.p149) && (p.p102 > 0.0)) { 1.0 } else { 0.0 };

        s.v[518] = if (p.p103 > 0.0) { 1.0 } else { 0.0 };

        s.v[519] = if (((p.p141 >= 1.0) && (p.p142 >= p.p149)) && (p.p142 > 0.0)) { 1.0 } else { 0.0 };

        s.v[520] = if (p.p145 > 0.0) { 1.0 } else { 0.0 };

        s.v[533] = if ((p.p109 == 1.0) && ((p.p88 > 0.0) && (p.p87 > 0.0))) { 1.0 } else { 0.0 };

        s.v[539] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[533] != 0.0) && (s.v[539] != 0.0)) {
            s.store_div(534, 184, 185);
        }

        if ((s.v[533] != 0.0) && (!(s.v[539] != 0.0))) {
            s.store_scalar(534, 1000000000.0);
        }

        if (s.v[533] != 0.0) {
            s.store_scalar(535, 1.0);
        }

        if (s.v[533] != 0.0) {
            s.store_scale(536, 219, p.p88);
        }

        if (s.v[533] != 0.0) {
            s.store_scale(538, 534, ((2.0 * p.p87) - (p.p88 * p.p88)));
        }

        s.v[540] = if (s.v[538] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[533] != 0.0) && (s.v[540] != 0.0)) {
            s.store_mul_ad_rhs(537, 219, A::sqrt(s.ad_value(538)));
        }

        if ((s.v[533] != 0.0) && (!(s.v[540] != 0.0))) {
            s.store_scalar(537, 0.0);
        }

    }

    pub(super) fn stamp_transient_equation_0_block_0(
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
        let eq0_e157: f64 = (s.v[185] + s.v[186]);
        let eq0_e157_d_n0: f64 = (s.dn[185][0] + s.dn[186][0]);
        let eq0_e157_d_n1: f64 = (s.dn[185][1] + s.dn[186][1]);
        let eq0_e157_d_n2: f64 = (s.dn[185][2] + s.dn[186][2]);
        let eq0_e157_d_n3: f64 = (s.dn[185][3] + s.dn[186][3]);
        let eq0_e157_d_n4: f64 = (s.dn[185][4] + s.dn[186][4]);
        let eq0_e157_d_n5: f64 = (s.dn[185][5] + s.dn[186][5]);
        let eq0_e157_d_n6: f64 = (s.dn[185][6] + s.dn[186][6]);
        let eq0_e157_d_n7: f64 = (s.dn[185][7] + s.dn[186][7]);
        let eq0_e157_d_n8: f64 = (s.dn[185][8] + s.dn[186][8]);
        let eq0_e157_d_n9: f64 = (s.dn[185][9] + s.dn[186][9]);
        let eq0_e157_d_n10: f64 = (s.dn[185][10] + s.dn[186][10]);
        let eq0_e157_d_n11: f64 = (s.dn[185][11] + s.dn[186][11]);
        let eq0_e157_d_n12: f64 = (s.dn[185][12] + s.dn[186][12]);
        let eq0_e157_d_n13: f64 = (s.dn[185][13] + s.dn[186][13]);
        let eq0_e157_d_n14: f64 = (s.dn[185][14] + s.dn[186][14]);
        let eq0_e157_d_b0: f64 = (s.db[185][0] + s.db[186][0]);
        let eq0_e157_d_b1: f64 = (s.db[185][1] + s.db[186][1]);
        let eq0_e157_d_b2: f64 = (s.db[185][2] + s.db[186][2]);
        let eq0_e157_d_b3: f64 = (s.db[185][3] + s.db[186][3]);
        let eq0_e157_d_b4: f64 = (s.db[185][4] + s.db[186][4]);
        let eq0_e157_d_b5: f64 = (s.db[185][5] + s.db[186][5]);
        let eq0_e159: f64 = (eq0_e157 + s.v[192]);
        let eq0_e159_d_n0: f64 = (eq0_e157_d_n0 + s.dn[192][0]);
        let eq0_e159_d_n1: f64 = (eq0_e157_d_n1 + s.dn[192][1]);
        let eq0_e159_d_n2: f64 = (eq0_e157_d_n2 + s.dn[192][2]);
        let eq0_e159_d_n3: f64 = (eq0_e157_d_n3 + s.dn[192][3]);
        let eq0_e159_d_n4: f64 = (eq0_e157_d_n4 + s.dn[192][4]);
        let eq0_e159_d_n5: f64 = (eq0_e157_d_n5 + s.dn[192][5]);
        let eq0_e159_d_n6: f64 = (eq0_e157_d_n6 + s.dn[192][6]);
        let eq0_e159_d_n7: f64 = (eq0_e157_d_n7 + s.dn[192][7]);
        let eq0_e159_d_n8: f64 = (eq0_e157_d_n8 + s.dn[192][8]);
        let eq0_e159_d_n9: f64 = (eq0_e157_d_n9 + s.dn[192][9]);
        let eq0_e159_d_n10: f64 = (eq0_e157_d_n10 + s.dn[192][10]);
        let eq0_e159_d_n11: f64 = (eq0_e157_d_n11 + s.dn[192][11]);
        let eq0_e159_d_n12: f64 = (eq0_e157_d_n12 + s.dn[192][12]);
        let eq0_e159_d_n13: f64 = (eq0_e157_d_n13 + s.dn[192][13]);
        let eq0_e159_d_n14: f64 = (eq0_e157_d_n14 + s.dn[192][14]);
        let eq0_e159_d_b0: f64 = (eq0_e157_d_b0 + s.db[192][0]);
        let eq0_e159_d_b1: f64 = (eq0_e157_d_b1 + s.db[192][1]);
        let eq0_e159_d_b2: f64 = (eq0_e157_d_b2 + s.db[192][2]);
        let eq0_e159_d_b3: f64 = (eq0_e157_d_b3 + s.db[192][3]);
        let eq0_e159_d_b4: f64 = (eq0_e157_d_b4 + s.db[192][4]);
        let eq0_e159_d_b5: f64 = (eq0_e157_d_b5 + s.db[192][5]);
        let eq0_e161: f64 = (eq0_e159 + s.v[190]);
        let eq0_e161_d_n0: f64 = (eq0_e159_d_n0 + s.dn[190][0]);
        let eq0_e161_d_n1: f64 = (eq0_e159_d_n1 + s.dn[190][1]);
        let eq0_e161_d_n2: f64 = (eq0_e159_d_n2 + s.dn[190][2]);
        let eq0_e161_d_n3: f64 = (eq0_e159_d_n3 + s.dn[190][3]);
        let eq0_e161_d_n4: f64 = (eq0_e159_d_n4 + s.dn[190][4]);
        let eq0_e161_d_n5: f64 = (eq0_e159_d_n5 + s.dn[190][5]);
        let eq0_e161_d_n6: f64 = (eq0_e159_d_n6 + s.dn[190][6]);
        let eq0_e161_d_n7: f64 = (eq0_e159_d_n7 + s.dn[190][7]);
        let eq0_e161_d_n8: f64 = (eq0_e159_d_n8 + s.dn[190][8]);
        let eq0_e161_d_n9: f64 = (eq0_e159_d_n9 + s.dn[190][9]);
        let eq0_e161_d_n10: f64 = (eq0_e159_d_n10 + s.dn[190][10]);
        let eq0_e161_d_n11: f64 = (eq0_e159_d_n11 + s.dn[190][11]);
        let eq0_e161_d_n12: f64 = (eq0_e159_d_n12 + s.dn[190][12]);
        let eq0_e161_d_n13: f64 = (eq0_e159_d_n13 + s.dn[190][13]);
        let eq0_e161_d_n14: f64 = (eq0_e159_d_n14 + s.dn[190][14]);
        let eq0_e161_d_b0: f64 = (eq0_e159_d_b0 + s.db[190][0]);
        let eq0_e161_d_b1: f64 = (eq0_e159_d_b1 + s.db[190][1]);
        let eq0_e161_d_b2: f64 = (eq0_e159_d_b2 + s.db[190][2]);
        let eq0_e161_d_b3: f64 = (eq0_e159_d_b3 + s.db[190][3]);
        let eq0_e161_d_b4: f64 = (eq0_e159_d_b4 + s.db[190][4]);
        let eq0_e161_d_b5: f64 = (eq0_e159_d_b5 + s.db[190][5]);
        let eq0_e162: f64 = (p.p148 * eq0_e161);
        let eq0_e162_d_n0: f64 = (p.p148 * eq0_e161_d_n0);
        let eq0_e162_d_n1: f64 = (p.p148 * eq0_e161_d_n1);
        let eq0_e162_d_n2: f64 = (p.p148 * eq0_e161_d_n2);
        let eq0_e162_d_n3: f64 = (p.p148 * eq0_e161_d_n3);
        let eq0_e162_d_n4: f64 = (p.p148 * eq0_e161_d_n4);
        let eq0_e162_d_n5: f64 = (p.p148 * eq0_e161_d_n5);
        let eq0_e162_d_n6: f64 = (p.p148 * eq0_e161_d_n6);
        let eq0_e162_d_n7: f64 = (p.p148 * eq0_e161_d_n7);
        let eq0_e162_d_n8: f64 = (p.p148 * eq0_e161_d_n8);
        let eq0_e162_d_n9: f64 = (p.p148 * eq0_e161_d_n9);
        let eq0_e162_d_n10: f64 = (p.p148 * eq0_e161_d_n10);
        let eq0_e162_d_n11: f64 = (p.p148 * eq0_e161_d_n11);
        let eq0_e162_d_n12: f64 = (p.p148 * eq0_e161_d_n12);
        let eq0_e162_d_n13: f64 = (p.p148 * eq0_e161_d_n13);
        let eq0_e162_d_n14: f64 = (p.p148 * eq0_e161_d_n14);
        let eq0_e162_d_b0: f64 = (p.p148 * eq0_e161_d_b0);
        let eq0_e162_d_b1: f64 = (p.p148 * eq0_e161_d_b1);
        let eq0_e162_d_b2: f64 = (p.p148 * eq0_e161_d_b2);
        let eq0_e162_d_b3: f64 = (p.p148 * eq0_e161_d_b3);
        let eq0_e162_d_b4: f64 = (p.p148 * eq0_e161_d_b4);
        let eq0_e162_d_b5: f64 = (p.p148 * eq0_e161_d_b5);
        let eq0_e165: f64 = (s.v[233] * (nv8 - nv6));
        let eq0_e165_d_n6: f64 = (-s.v[233]);
        let eq0_e165_d_n8: f64 = s.v[233];
        let eq0_e166: f64 = (eq0_e162 + eq0_e165);
        let eq0_e166_d_n6: f64 = (eq0_e162_d_n6 + eq0_e165_d_n6);
        let eq0_e166_d_n8: f64 = (eq0_e162_d_n8 + eq0_e165_d_n8);
        let eq0_value: f64 = eq0_e166;
        let eq0_node_derivatives: [f64; 15] = [eq0_e162_d_n0, eq0_e162_d_n1, eq0_e162_d_n2, eq0_e162_d_n3, eq0_e162_d_n4, eq0_e162_d_n5, eq0_e166_d_n6, eq0_e162_d_n7, eq0_e166_d_n8, eq0_e162_d_n9, eq0_e162_d_n10, eq0_e162_d_n11, eq0_e162_d_n12, eq0_e162_d_n13, eq0_e162_d_n14];
        let eq0_branch_derivatives: [f64; 6] = [eq0_e162_d_b0, eq0_e162_d_b1, eq0_e162_d_b2, eq0_e162_d_b3, eq0_e162_d_b4, eq0_e162_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq0_value),
            &nodes,
            &eq0_node_derivatives,
            &branches,
            &eq0_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let eq1_e170: f64 = (s.v[242] + s.v[179]);
        let eq1_e170_d_n0: f64 = (s.dn[242][0] + s.dn[179][0]);
        let eq1_e170_d_n1: f64 = (s.dn[242][1] + s.dn[179][1]);
        let eq1_e170_d_n2: f64 = (s.dn[242][2] + s.dn[179][2]);
        let eq1_e170_d_n3: f64 = (s.dn[242][3] + s.dn[179][3]);
        let eq1_e170_d_n4: f64 = (s.dn[242][4] + s.dn[179][4]);
        let eq1_e170_d_n5: f64 = (s.dn[242][5] + s.dn[179][5]);
        let eq1_e170_d_n6: f64 = (s.dn[242][6] + s.dn[179][6]);
        let eq1_e170_d_n7: f64 = (s.dn[242][7] + s.dn[179][7]);
        let eq1_e170_d_n8: f64 = (s.dn[242][8] + s.dn[179][8]);
        let eq1_e170_d_n9: f64 = (s.dn[242][9] + s.dn[179][9]);
        let eq1_e170_d_n10: f64 = (s.dn[242][10] + s.dn[179][10]);
        let eq1_e170_d_n11: f64 = (s.dn[242][11] + s.dn[179][11]);
        let eq1_e170_d_n12: f64 = (s.dn[242][12] + s.dn[179][12]);
        let eq1_e170_d_n13: f64 = (s.dn[242][13] + s.dn[179][13]);
        let eq1_e170_d_n14: f64 = (s.dn[242][14] + s.dn[179][14]);
        let eq1_e170_d_b0: f64 = (s.db[242][0] + s.db[179][0]);
        let eq1_e170_d_b1: f64 = (s.db[242][1] + s.db[179][1]);
        let eq1_e170_d_b2: f64 = (s.db[242][2] + s.db[179][2]);
        let eq1_e170_d_b3: f64 = (s.db[242][3] + s.db[179][3]);
        let eq1_e170_d_b4: f64 = (s.db[242][4] + s.db[179][4]);
        let eq1_e170_d_b5: f64 = (s.db[242][5] + s.db[179][5]);
        let eq1_e171: f64 = (p.p148 * eq1_e170);
        let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);
        let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);
        let eq1_e171_d_n2: f64 = (p.p148 * eq1_e170_d_n2);
        let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);
        let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);
        let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);
        let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);
        let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);
        let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);
        let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);
        let eq1_e171_d_n10: f64 = (p.p148 * eq1_e170_d_n10);
        let eq1_e171_d_n11: f64 = (p.p148 * eq1_e170_d_n11);
        let eq1_e171_d_n12: f64 = (p.p148 * eq1_e170_d_n12);
        let eq1_e171_d_n13: f64 = (p.p148 * eq1_e170_d_n13);
        let eq1_e171_d_n14: f64 = (p.p148 * eq1_e170_d_n14);
        let eq1_e171_d_b0: f64 = (p.p148 * eq1_e170_d_b0);
        let eq1_e171_d_b1: f64 = (p.p148 * eq1_e170_d_b1);
        let eq1_e171_d_b2: f64 = (p.p148 * eq1_e170_d_b2);
        let eq1_e171_d_b3: f64 = (p.p148 * eq1_e170_d_b3);
        let eq1_e171_d_b4: f64 = (p.p148 * eq1_e170_d_b4);
        let eq1_e171_d_b5: f64 = (p.p148 * eq1_e170_d_b5);
        let eq1_e172: f64 = self.eval_ddt(0, eq1_e171);
        let eq1_e172_d_n0: f64 = self.ddt_jacobian(eq1_e171_d_n0);
        let eq1_e172_d_n1: f64 = self.ddt_jacobian(eq1_e171_d_n1);
        let eq1_e172_d_n2: f64 = self.ddt_jacobian(eq1_e171_d_n2);
        let eq1_e172_d_n3: f64 = self.ddt_jacobian(eq1_e171_d_n3);
        let eq1_e172_d_n4: f64 = self.ddt_jacobian(eq1_e171_d_n4);
        let eq1_e172_d_n5: f64 = self.ddt_jacobian(eq1_e171_d_n5);
        let eq1_e172_d_n6: f64 = self.ddt_jacobian(eq1_e171_d_n6);
        let eq1_e172_d_n7: f64 = self.ddt_jacobian(eq1_e171_d_n7);
        let eq1_e172_d_n8: f64 = self.ddt_jacobian(eq1_e171_d_n8);
        let eq1_e172_d_n9: f64 = self.ddt_jacobian(eq1_e171_d_n9);
        let eq1_e172_d_n10: f64 = self.ddt_jacobian(eq1_e171_d_n10);
        let eq1_e172_d_n11: f64 = self.ddt_jacobian(eq1_e171_d_n11);
        let eq1_e172_d_n12: f64 = self.ddt_jacobian(eq1_e171_d_n12);
        let eq1_e172_d_n13: f64 = self.ddt_jacobian(eq1_e171_d_n13);
        let eq1_e172_d_n14: f64 = self.ddt_jacobian(eq1_e171_d_n14);
        let eq1_e172_d_b0: f64 = self.ddt_jacobian(eq1_e171_d_b0);
        let eq1_e172_d_b1: f64 = self.ddt_jacobian(eq1_e171_d_b1);
        let eq1_e172_d_b2: f64 = self.ddt_jacobian(eq1_e171_d_b2);
        let eq1_e172_d_b3: f64 = self.ddt_jacobian(eq1_e171_d_b3);
        let eq1_e172_d_b4: f64 = self.ddt_jacobian(eq1_e171_d_b4);
        let eq1_e172_d_b5: f64 = self.ddt_jacobian(eq1_e171_d_b5);
        let eq1_value: f64 = eq1_e172;
        let eq1_node_derivatives: [f64; 15] = [eq1_e172_d_n0, eq1_e172_d_n1, eq1_e172_d_n2, eq1_e172_d_n3, eq1_e172_d_n4, eq1_e172_d_n5, eq1_e172_d_n6, eq1_e172_d_n7, eq1_e172_d_n8, eq1_e172_d_n9, eq1_e172_d_n10, eq1_e172_d_n11, eq1_e172_d_n12, eq1_e172_d_n13, eq1_e172_d_n14];
        let eq1_branch_derivatives: [f64; 6] = [eq1_e172_d_b0, eq1_e172_d_b1, eq1_e172_d_b2, eq1_e172_d_b3, eq1_e172_d_b4, eq1_e172_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq2_e176: f64 = (s.v[187] - s.v[244]);
        let eq2_e176_d_n0: f64 = (s.dn[187][0] - s.dn[244][0]);
        let eq2_e176_d_n1: f64 = (s.dn[187][1] - s.dn[244][1]);
        let eq2_e176_d_n2: f64 = (s.dn[187][2] - s.dn[244][2]);
        let eq2_e176_d_n3: f64 = (s.dn[187][3] - s.dn[244][3]);
        let eq2_e176_d_n4: f64 = (s.dn[187][4] - s.dn[244][4]);
        let eq2_e176_d_n5: f64 = (s.dn[187][5] - s.dn[244][5]);
        let eq2_e176_d_n6: f64 = (s.dn[187][6] - s.dn[244][6]);
        let eq2_e176_d_n7: f64 = (s.dn[187][7] - s.dn[244][7]);
        let eq2_e176_d_n8: f64 = (s.dn[187][8] - s.dn[244][8]);
        let eq2_e176_d_n9: f64 = (s.dn[187][9] - s.dn[244][9]);
        let eq2_e176_d_n10: f64 = (s.dn[187][10] - s.dn[244][10]);
        let eq2_e176_d_n11: f64 = (s.dn[187][11] - s.dn[244][11]);
        let eq2_e176_d_n12: f64 = (s.dn[187][12] - s.dn[244][12]);
        let eq2_e176_d_n13: f64 = (s.dn[187][13] - s.dn[244][13]);
        let eq2_e176_d_n14: f64 = (s.dn[187][14] - s.dn[244][14]);
        let eq2_e176_d_b0: f64 = (s.db[187][0] - s.db[244][0]);
        let eq2_e176_d_b1: f64 = (s.db[187][1] - s.db[244][1]);
        let eq2_e176_d_b2: f64 = (s.db[187][2] - s.db[244][2]);
        let eq2_e176_d_b3: f64 = (s.db[187][3] - s.db[244][3]);
        let eq2_e176_d_b4: f64 = (s.db[187][4] - s.db[244][4]);
        let eq2_e176_d_b5: f64 = (s.db[187][5] - s.db[244][5]);
        let eq2_e177: f64 = (p.p148 * eq2_e176);
        let eq2_e177_d_n0: f64 = (p.p148 * eq2_e176_d_n0);
        let eq2_e177_d_n1: f64 = (p.p148 * eq2_e176_d_n1);
        let eq2_e177_d_n2: f64 = (p.p148 * eq2_e176_d_n2);
        let eq2_e177_d_n3: f64 = (p.p148 * eq2_e176_d_n3);
        let eq2_e177_d_n4: f64 = (p.p148 * eq2_e176_d_n4);
        let eq2_e177_d_n5: f64 = (p.p148 * eq2_e176_d_n5);
        let eq2_e177_d_n6: f64 = (p.p148 * eq2_e176_d_n6);
        let eq2_e177_d_n7: f64 = (p.p148 * eq2_e176_d_n7);
        let eq2_e177_d_n8: f64 = (p.p148 * eq2_e176_d_n8);
        let eq2_e177_d_n9: f64 = (p.p148 * eq2_e176_d_n9);
        let eq2_e177_d_n10: f64 = (p.p148 * eq2_e176_d_n10);
        let eq2_e177_d_n11: f64 = (p.p148 * eq2_e176_d_n11);
        let eq2_e177_d_n12: f64 = (p.p148 * eq2_e176_d_n12);
        let eq2_e177_d_n13: f64 = (p.p148 * eq2_e176_d_n13);
        let eq2_e177_d_n14: f64 = (p.p148 * eq2_e176_d_n14);
        let eq2_e177_d_b0: f64 = (p.p148 * eq2_e176_d_b0);
        let eq2_e177_d_b1: f64 = (p.p148 * eq2_e176_d_b1);
        let eq2_e177_d_b2: f64 = (p.p148 * eq2_e176_d_b2);
        let eq2_e177_d_b3: f64 = (p.p148 * eq2_e176_d_b3);
        let eq2_e177_d_b4: f64 = (p.p148 * eq2_e176_d_b4);
        let eq2_e177_d_b5: f64 = (p.p148 * eq2_e176_d_b5);
        let eq2_e180: f64 = (s.v[233] * (nv8 - nv5));
        let eq2_e180_d_n5: f64 = (-s.v[233]);
        let eq2_e180_d_n8: f64 = s.v[233];
        let eq2_e181: f64 = (eq2_e177 + eq2_e180);
        let eq2_e181_d_n5: f64 = (eq2_e177_d_n5 + eq2_e180_d_n5);
        let eq2_e181_d_n8: f64 = (eq2_e177_d_n8 + eq2_e180_d_n8);
        let eq2_value: f64 = eq2_e181;
        let eq2_node_derivatives: [f64; 15] = [eq2_e177_d_n0, eq2_e177_d_n1, eq2_e177_d_n2, eq2_e177_d_n3, eq2_e177_d_n4, eq2_e181_d_n5, eq2_e177_d_n6, eq2_e177_d_n7, eq2_e181_d_n8, eq2_e177_d_n9, eq2_e177_d_n10, eq2_e177_d_n11, eq2_e177_d_n12, eq2_e177_d_n13, eq2_e177_d_n14];
        let eq2_branch_derivatives: [f64; 6] = [eq2_e177_d_b0, eq2_e177_d_b1, eq2_e177_d_b2, eq2_e177_d_b3, eq2_e177_d_b4, eq2_e177_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let eq3_e185: f64 = (s.v[182] + s.v[178]);
        let eq3_e185_d_n0: f64 = (s.dn[182][0] + s.dn[178][0]);
        let eq3_e185_d_n1: f64 = (s.dn[182][1] + s.dn[178][1]);
        let eq3_e185_d_n2: f64 = (s.dn[182][2] + s.dn[178][2]);
        let eq3_e185_d_n3: f64 = (s.dn[182][3] + s.dn[178][3]);
        let eq3_e185_d_n4: f64 = (s.dn[182][4] + s.dn[178][4]);
        let eq3_e185_d_n5: f64 = (s.dn[182][5] + s.dn[178][5]);
        let eq3_e185_d_n6: f64 = (s.dn[182][6] + s.dn[178][6]);
        let eq3_e185_d_n7: f64 = (s.dn[182][7] + s.dn[178][7]);
        let eq3_e185_d_n8: f64 = (s.dn[182][8] + s.dn[178][8]);
        let eq3_e185_d_n9: f64 = (s.dn[182][9] + s.dn[178][9]);
        let eq3_e185_d_n10: f64 = (s.dn[182][10] + s.dn[178][10]);
        let eq3_e185_d_n11: f64 = (s.dn[182][11] + s.dn[178][11]);
        let eq3_e185_d_n12: f64 = (s.dn[182][12] + s.dn[178][12]);
        let eq3_e185_d_n13: f64 = (s.dn[182][13] + s.dn[178][13]);
        let eq3_e185_d_n14: f64 = (s.dn[182][14] + s.dn[178][14]);
        let eq3_e185_d_b0: f64 = (s.db[182][0] + s.db[178][0]);
        let eq3_e185_d_b1: f64 = (s.db[182][1] + s.db[178][1]);
        let eq3_e185_d_b2: f64 = (s.db[182][2] + s.db[178][2]);
        let eq3_e185_d_b3: f64 = (s.db[182][3] + s.db[178][3]);
        let eq3_e185_d_b4: f64 = (s.db[182][4] + s.db[178][4]);
        let eq3_e185_d_b5: f64 = (s.db[182][5] + s.db[178][5]);
        let eq3_e186: f64 = (p.p148 * eq3_e185);
        let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);
        let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);
        let eq3_e186_d_n2: f64 = (p.p148 * eq3_e185_d_n2);
        let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);
        let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);
        let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);
        let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);
        let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);
        let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);
        let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);
        let eq3_e186_d_n10: f64 = (p.p148 * eq3_e185_d_n10);
        let eq3_e186_d_n11: f64 = (p.p148 * eq3_e185_d_n11);
        let eq3_e186_d_n12: f64 = (p.p148 * eq3_e185_d_n12);
        let eq3_e186_d_n13: f64 = (p.p148 * eq3_e185_d_n13);
        let eq3_e186_d_n14: f64 = (p.p148 * eq3_e185_d_n14);
        let eq3_e186_d_b0: f64 = (p.p148 * eq3_e185_d_b0);
        let eq3_e186_d_b1: f64 = (p.p148 * eq3_e185_d_b1);
        let eq3_e186_d_b2: f64 = (p.p148 * eq3_e185_d_b2);
        let eq3_e186_d_b3: f64 = (p.p148 * eq3_e185_d_b3);
        let eq3_e186_d_b4: f64 = (p.p148 * eq3_e185_d_b4);
        let eq3_e186_d_b5: f64 = (p.p148 * eq3_e185_d_b5);
        let eq3_e187: f64 = self.eval_ddt(1, eq3_e186);
        let eq3_e187_d_n0: f64 = self.ddt_jacobian(eq3_e186_d_n0);
        let eq3_e187_d_n1: f64 = self.ddt_jacobian(eq3_e186_d_n1);
        let eq3_e187_d_n2: f64 = self.ddt_jacobian(eq3_e186_d_n2);
        let eq3_e187_d_n3: f64 = self.ddt_jacobian(eq3_e186_d_n3);
        let eq3_e187_d_n4: f64 = self.ddt_jacobian(eq3_e186_d_n4);
        let eq3_e187_d_n5: f64 = self.ddt_jacobian(eq3_e186_d_n5);
        let eq3_e187_d_n6: f64 = self.ddt_jacobian(eq3_e186_d_n6);
        let eq3_e187_d_n7: f64 = self.ddt_jacobian(eq3_e186_d_n7);
        let eq3_e187_d_n8: f64 = self.ddt_jacobian(eq3_e186_d_n8);
        let eq3_e187_d_n9: f64 = self.ddt_jacobian(eq3_e186_d_n9);
        let eq3_e187_d_n10: f64 = self.ddt_jacobian(eq3_e186_d_n10);
        let eq3_e187_d_n11: f64 = self.ddt_jacobian(eq3_e186_d_n11);
        let eq3_e187_d_n12: f64 = self.ddt_jacobian(eq3_e186_d_n12);
        let eq3_e187_d_n13: f64 = self.ddt_jacobian(eq3_e186_d_n13);
        let eq3_e187_d_n14: f64 = self.ddt_jacobian(eq3_e186_d_n14);
        let eq3_e187_d_b0: f64 = self.ddt_jacobian(eq3_e186_d_b0);
        let eq3_e187_d_b1: f64 = self.ddt_jacobian(eq3_e186_d_b1);
        let eq3_e187_d_b2: f64 = self.ddt_jacobian(eq3_e186_d_b2);
        let eq3_e187_d_b3: f64 = self.ddt_jacobian(eq3_e186_d_b3);
        let eq3_e187_d_b4: f64 = self.ddt_jacobian(eq3_e186_d_b4);
        let eq3_e187_d_b5: f64 = self.ddt_jacobian(eq3_e186_d_b5);
        let eq3_value: f64 = eq3_e187;
        let eq3_node_derivatives: [f64; 15] = [eq3_e187_d_n0, eq3_e187_d_n1, eq3_e187_d_n2, eq3_e187_d_n3, eq3_e187_d_n4, eq3_e187_d_n5, eq3_e187_d_n6, eq3_e187_d_n7, eq3_e187_d_n8, eq3_e187_d_n9, eq3_e187_d_n10, eq3_e187_d_n11, eq3_e187_d_n12, eq3_e187_d_n13, eq3_e187_d_n14];
        let eq3_branch_derivatives: [f64; 6] = [eq3_e187_d_b0, eq3_e187_d_b1, eq3_e187_d_b2, eq3_e187_d_b3, eq3_e187_d_b4, eq3_e187_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
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
        let eq4_e190: f64 = (p.p148 * s.v[241]);
        let eq4_e190_d_n0: f64 = (p.p148 * s.dn[241][0]);
        let eq4_e190_d_n1: f64 = (p.p148 * s.dn[241][1]);
        let eq4_e190_d_n2: f64 = (p.p148 * s.dn[241][2]);
        let eq4_e190_d_n3: f64 = (p.p148 * s.dn[241][3]);
        let eq4_e190_d_n4: f64 = (p.p148 * s.dn[241][4]);
        let eq4_e190_d_n5: f64 = (p.p148 * s.dn[241][5]);
        let eq4_e190_d_n6: f64 = (p.p148 * s.dn[241][6]);
        let eq4_e190_d_n7: f64 = (p.p148 * s.dn[241][7]);
        let eq4_e190_d_n8: f64 = (p.p148 * s.dn[241][8]);
        let eq4_e190_d_n9: f64 = (p.p148 * s.dn[241][9]);
        let eq4_e190_d_n10: f64 = (p.p148 * s.dn[241][10]);
        let eq4_e190_d_n11: f64 = (p.p148 * s.dn[241][11]);
        let eq4_e190_d_n12: f64 = (p.p148 * s.dn[241][12]);
        let eq4_e190_d_n13: f64 = (p.p148 * s.dn[241][13]);
        let eq4_e190_d_n14: f64 = (p.p148 * s.dn[241][14]);
        let eq4_e190_d_b0: f64 = (p.p148 * s.db[241][0]);
        let eq4_e190_d_b1: f64 = (p.p148 * s.db[241][1]);
        let eq4_e190_d_b2: f64 = (p.p148 * s.db[241][2]);
        let eq4_e190_d_b3: f64 = (p.p148 * s.db[241][3]);
        let eq4_e190_d_b4: f64 = (p.p148 * s.db[241][4]);
        let eq4_e190_d_b5: f64 = (p.p148 * s.db[241][5]);
        let eq4_value: f64 = eq4_e190;
        let eq4_node_derivatives: [f64; 15] = [eq4_e190_d_n0, eq4_e190_d_n1, eq4_e190_d_n2, eq4_e190_d_n3, eq4_e190_d_n4, eq4_e190_d_n5, eq4_e190_d_n6, eq4_e190_d_n7, eq4_e190_d_n8, eq4_e190_d_n9, eq4_e190_d_n10, eq4_e190_d_n11, eq4_e190_d_n12, eq4_e190_d_n13, eq4_e190_d_n14];
        let eq4_branch_derivatives: [f64; 6] = [eq4_e190_d_b0, eq4_e190_d_b1, eq4_e190_d_b2, eq4_e190_d_b3, eq4_e190_d_b4, eq4_e190_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[5]),
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
        let eq5_e193: f64 = (p.p148 * s.v[218]);
        let eq5_e193_d_n0: f64 = (p.p148 * s.dn[218][0]);
        let eq5_e193_d_n1: f64 = (p.p148 * s.dn[218][1]);
        let eq5_e193_d_n2: f64 = (p.p148 * s.dn[218][2]);
        let eq5_e193_d_n3: f64 = (p.p148 * s.dn[218][3]);
        let eq5_e193_d_n4: f64 = (p.p148 * s.dn[218][4]);
        let eq5_e193_d_n5: f64 = (p.p148 * s.dn[218][5]);
        let eq5_e193_d_n6: f64 = (p.p148 * s.dn[218][6]);
        let eq5_e193_d_n7: f64 = (p.p148 * s.dn[218][7]);
        let eq5_e193_d_n8: f64 = (p.p148 * s.dn[218][8]);
        let eq5_e193_d_n9: f64 = (p.p148 * s.dn[218][9]);
        let eq5_e193_d_n10: f64 = (p.p148 * s.dn[218][10]);
        let eq5_e193_d_n11: f64 = (p.p148 * s.dn[218][11]);
        let eq5_e193_d_n12: f64 = (p.p148 * s.dn[218][12]);
        let eq5_e193_d_n13: f64 = (p.p148 * s.dn[218][13]);
        let eq5_e193_d_n14: f64 = (p.p148 * s.dn[218][14]);
        let eq5_e193_d_b0: f64 = (p.p148 * s.db[218][0]);
        let eq5_e193_d_b1: f64 = (p.p148 * s.db[218][1]);
        let eq5_e193_d_b2: f64 = (p.p148 * s.db[218][2]);
        let eq5_e193_d_b3: f64 = (p.p148 * s.db[218][3]);
        let eq5_e193_d_b4: f64 = (p.p148 * s.db[218][4]);
        let eq5_e193_d_b5: f64 = (p.p148 * s.db[218][5]);
        let eq5_value: f64 = eq5_e193;
        let eq5_node_derivatives: [f64; 15] = [eq5_e193_d_n0, eq5_e193_d_n1, eq5_e193_d_n2, eq5_e193_d_n3, eq5_e193_d_n4, eq5_e193_d_n5, eq5_e193_d_n6, eq5_e193_d_n7, eq5_e193_d_n8, eq5_e193_d_n9, eq5_e193_d_n10, eq5_e193_d_n11, eq5_e193_d_n12, eq5_e193_d_n13, eq5_e193_d_n14];
        let eq5_branch_derivatives: [f64; 6] = [eq5_e193_d_b0, eq5_e193_d_b1, eq5_e193_d_b2, eq5_e193_d_b3, eq5_e193_d_b4, eq5_e193_d_b5];
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq6_e199, eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n2, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9, eq6_e199_d_n10, eq6_e199_d_n11, eq6_e199_d_n12, eq6_e199_d_n13, eq6_e199_d_n14, eq6_e199_d_b0, eq6_e199_d_b1, eq6_e199_d_b2, eq6_e199_d_b3, eq6_e199_d_b4, eq6_e199_d_b5,) = {
    if (s.v[508] != 0.0) {
        let eq6_e197: f64 = ((nv7 - nv8) / s.v[70]);
        let eq6_e197_d_n0: f64 = (-(((nv7 - nv8) * s.dn[70][0]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n1: f64 = (-(((nv7 - nv8) * s.dn[70][1]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n2: f64 = (-(((nv7 - nv8) * s.dn[70][2]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n3: f64 = (-(((nv7 - nv8) * s.dn[70][3]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n4: f64 = (-(((nv7 - nv8) * s.dn[70][4]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n5: f64 = (-(((nv7 - nv8) * s.dn[70][5]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n6: f64 = (-(((nv7 - nv8) * s.dn[70][6]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n7: f64 = ((s.v[70] - ((nv7 - nv8) * s.dn[70][7])) / (s.v[70] * s.v[70]));
        let eq6_e197_d_n8: f64 = (((-s.v[70]) - ((nv7 - nv8) * s.dn[70][8])) / (s.v[70] * s.v[70]));
        let eq6_e197_d_n9: f64 = (-(((nv7 - nv8) * s.dn[70][9]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n10: f64 = (-(((nv7 - nv8) * s.dn[70][10]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n11: f64 = (-(((nv7 - nv8) * s.dn[70][11]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n12: f64 = (-(((nv7 - nv8) * s.dn[70][12]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n13: f64 = (-(((nv7 - nv8) * s.dn[70][13]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_n14: f64 = (-(((nv7 - nv8) * s.dn[70][14]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b0: f64 = (-(((nv7 - nv8) * s.db[70][0]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b1: f64 = (-(((nv7 - nv8) * s.db[70][1]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b2: f64 = (-(((nv7 - nv8) * s.db[70][2]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b3: f64 = (-(((nv7 - nv8) * s.db[70][3]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b4: f64 = (-(((nv7 - nv8) * s.db[70][4]) / (s.v[70] * s.v[70])));
        let eq6_e197_d_b5: f64 = (-(((nv7 - nv8) * s.db[70][5]) / (s.v[70] * s.v[70])));
        (eq6_e197, eq6_e197_d_n0, eq6_e197_d_n1, eq6_e197_d_n2, eq6_e197_d_n3, eq6_e197_d_n4, eq6_e197_d_n5, eq6_e197_d_n6, eq6_e197_d_n7, eq6_e197_d_n8, eq6_e197_d_n9, eq6_e197_d_n10, eq6_e197_d_n11, eq6_e197_d_n12, eq6_e197_d_n13, eq6_e197_d_n14, eq6_e197_d_b0, eq6_e197_d_b1, eq6_e197_d_b2, eq6_e197_d_b3, eq6_e197_d_b4, eq6_e197_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e199;
        let eq6_node_derivatives: [f64; 15] = [eq6_e199_d_n0, eq6_e199_d_n1, eq6_e199_d_n2, eq6_e199_d_n3, eq6_e199_d_n4, eq6_e199_d_n5, eq6_e199_d_n6, eq6_e199_d_n7, eq6_e199_d_n8, eq6_e199_d_n9, eq6_e199_d_n10, eq6_e199_d_n11, eq6_e199_d_n12, eq6_e199_d_n13, eq6_e199_d_n14];
        let eq6_branch_derivatives: [f64; 6] = [eq6_e199_d_b0, eq6_e199_d_b1, eq6_e199_d_b2, eq6_e199_d_b3, eq6_e199_d_b4, eq6_e199_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5,) = {
    if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
        let eq7_e204: f64 = self.eval_ddt(2, s.v[183]);
        let eq7_e204_d_n0: f64 = self.ddt_jacobian(s.dn[183][0]);
        let eq7_e204_d_n1: f64 = self.ddt_jacobian(s.dn[183][1]);
        let eq7_e204_d_n2: f64 = self.ddt_jacobian(s.dn[183][2]);
        let eq7_e204_d_n3: f64 = self.ddt_jacobian(s.dn[183][3]);
        let eq7_e204_d_n4: f64 = self.ddt_jacobian(s.dn[183][4]);
        let eq7_e204_d_n5: f64 = self.ddt_jacobian(s.dn[183][5]);
        let eq7_e204_d_n6: f64 = self.ddt_jacobian(s.dn[183][6]);
        let eq7_e204_d_n7: f64 = self.ddt_jacobian(s.dn[183][7]);
        let eq7_e204_d_n8: f64 = self.ddt_jacobian(s.dn[183][8]);
        let eq7_e204_d_n9: f64 = self.ddt_jacobian(s.dn[183][9]);
        let eq7_e204_d_n10: f64 = self.ddt_jacobian(s.dn[183][10]);
        let eq7_e204_d_n11: f64 = self.ddt_jacobian(s.dn[183][11]);
        let eq7_e204_d_n12: f64 = self.ddt_jacobian(s.dn[183][12]);
        let eq7_e204_d_n13: f64 = self.ddt_jacobian(s.dn[183][13]);
        let eq7_e204_d_n14: f64 = self.ddt_jacobian(s.dn[183][14]);
        let eq7_e204_d_b0: f64 = self.ddt_jacobian(s.db[183][0]);
        let eq7_e204_d_b1: f64 = self.ddt_jacobian(s.db[183][1]);
        let eq7_e204_d_b2: f64 = self.ddt_jacobian(s.db[183][2]);
        let eq7_e204_d_b3: f64 = self.ddt_jacobian(s.db[183][3]);
        let eq7_e204_d_b4: f64 = self.ddt_jacobian(s.db[183][4]);
        let eq7_e204_d_b5: f64 = self.ddt_jacobian(s.db[183][5]);
        (eq7_e204, eq7_e204_d_n0, eq7_e204_d_n1, eq7_e204_d_n2, eq7_e204_d_n3, eq7_e204_d_n4, eq7_e204_d_n5, eq7_e204_d_n6, eq7_e204_d_n7, eq7_e204_d_n8, eq7_e204_d_n9, eq7_e204_d_n10, eq7_e204_d_n11, eq7_e204_d_n12, eq7_e204_d_n13, eq7_e204_d_n14, eq7_e204_d_b0, eq7_e204_d_b1, eq7_e204_d_b2, eq7_e204_d_b3, eq7_e204_d_b4, eq7_e204_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e206;
        let eq7_node_derivatives: [f64; 15] = [eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14];
        let eq7_branch_derivatives: [f64; 6] = [eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let (eq8_e211,) = {
    if (!(s.v[508] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq8_value: f64 = eq8_e211;
        stamper.stamp_potential(
            branches[0],
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
        let (eq9_e218, eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14, eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5,) = {
    if (s.v[510] != 0.0) {
        let eq9_e214: f64 = (-p.p148);
        let eq9_e216: f64 = (eq9_e214 * s.v[191]);
        let eq9_e216_d_n0: f64 = (eq9_e214 * s.dn[191][0]);
        let eq9_e216_d_n1: f64 = (eq9_e214 * s.dn[191][1]);
        let eq9_e216_d_n2: f64 = (eq9_e214 * s.dn[191][2]);
        let eq9_e216_d_n3: f64 = (eq9_e214 * s.dn[191][3]);
        let eq9_e216_d_n4: f64 = (eq9_e214 * s.dn[191][4]);
        let eq9_e216_d_n5: f64 = (eq9_e214 * s.dn[191][5]);
        let eq9_e216_d_n6: f64 = (eq9_e214 * s.dn[191][6]);
        let eq9_e216_d_n7: f64 = (eq9_e214 * s.dn[191][7]);
        let eq9_e216_d_n8: f64 = (eq9_e214 * s.dn[191][8]);
        let eq9_e216_d_n9: f64 = (eq9_e214 * s.dn[191][9]);
        let eq9_e216_d_n10: f64 = (eq9_e214 * s.dn[191][10]);
        let eq9_e216_d_n11: f64 = (eq9_e214 * s.dn[191][11]);
        let eq9_e216_d_n12: f64 = (eq9_e214 * s.dn[191][12]);
        let eq9_e216_d_n13: f64 = (eq9_e214 * s.dn[191][13]);
        let eq9_e216_d_n14: f64 = (eq9_e214 * s.dn[191][14]);
        let eq9_e216_d_b0: f64 = (eq9_e214 * s.db[191][0]);
        let eq9_e216_d_b1: f64 = (eq9_e214 * s.db[191][1]);
        let eq9_e216_d_b2: f64 = (eq9_e214 * s.db[191][2]);
        let eq9_e216_d_b3: f64 = (eq9_e214 * s.db[191][3]);
        let eq9_e216_d_b4: f64 = (eq9_e214 * s.db[191][4]);
        let eq9_e216_d_b5: f64 = (eq9_e214 * s.db[191][5]);
        (eq9_e216, eq9_e216_d_n0, eq9_e216_d_n1, eq9_e216_d_n2, eq9_e216_d_n3, eq9_e216_d_n4, eq9_e216_d_n5, eq9_e216_d_n6, eq9_e216_d_n7, eq9_e216_d_n8, eq9_e216_d_n9, eq9_e216_d_n10, eq9_e216_d_n11, eq9_e216_d_n12, eq9_e216_d_n13, eq9_e216_d_n14, eq9_e216_d_b0, eq9_e216_d_b1, eq9_e216_d_b2, eq9_e216_d_b3, eq9_e216_d_b4, eq9_e216_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e218;
        let eq9_node_derivatives: [f64; 15] = [eq9_e218_d_n0, eq9_e218_d_n1, eq9_e218_d_n2, eq9_e218_d_n3, eq9_e218_d_n4, eq9_e218_d_n5, eq9_e218_d_n6, eq9_e218_d_n7, eq9_e218_d_n8, eq9_e218_d_n9, eq9_e218_d_n10, eq9_e218_d_n11, eq9_e218_d_n12, eq9_e218_d_n13, eq9_e218_d_n14];
        let eq9_branch_derivatives: [f64; 6] = [eq9_e218_d_b0, eq9_e218_d_b1, eq9_e218_d_b2, eq9_e218_d_b3, eq9_e218_d_b4, eq9_e218_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let (eq10_e226, eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n2, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9, eq10_e226_d_n10, eq10_e226_d_n11, eq10_e226_d_n12, eq10_e226_d_n13, eq10_e226_d_n14, eq10_e226_d_b0, eq10_e226_d_b1, eq10_e226_d_b2, eq10_e226_d_b3, eq10_e226_d_b4, eq10_e226_d_b5,) = {
    if (!(s.v[510] != 0.0)) {
        let eq10_e222: f64 = (-p.p148);
        let eq10_e224: f64 = (eq10_e222 * s.v[191]);
        let eq10_e224_d_n0: f64 = (eq10_e222 * s.dn[191][0]);
        let eq10_e224_d_n1: f64 = (eq10_e222 * s.dn[191][1]);
        let eq10_e224_d_n2: f64 = (eq10_e222 * s.dn[191][2]);
        let eq10_e224_d_n3: f64 = (eq10_e222 * s.dn[191][3]);
        let eq10_e224_d_n4: f64 = (eq10_e222 * s.dn[191][4]);
        let eq10_e224_d_n5: f64 = (eq10_e222 * s.dn[191][5]);
        let eq10_e224_d_n6: f64 = (eq10_e222 * s.dn[191][6]);
        let eq10_e224_d_n7: f64 = (eq10_e222 * s.dn[191][7]);
        let eq10_e224_d_n8: f64 = (eq10_e222 * s.dn[191][8]);
        let eq10_e224_d_n9: f64 = (eq10_e222 * s.dn[191][9]);
        let eq10_e224_d_n10: f64 = (eq10_e222 * s.dn[191][10]);
        let eq10_e224_d_n11: f64 = (eq10_e222 * s.dn[191][11]);
        let eq10_e224_d_n12: f64 = (eq10_e222 * s.dn[191][12]);
        let eq10_e224_d_n13: f64 = (eq10_e222 * s.dn[191][13]);
        let eq10_e224_d_n14: f64 = (eq10_e222 * s.dn[191][14]);
        let eq10_e224_d_b0: f64 = (eq10_e222 * s.db[191][0]);
        let eq10_e224_d_b1: f64 = (eq10_e222 * s.db[191][1]);
        let eq10_e224_d_b2: f64 = (eq10_e222 * s.db[191][2]);
        let eq10_e224_d_b3: f64 = (eq10_e222 * s.db[191][3]);
        let eq10_e224_d_b4: f64 = (eq10_e222 * s.db[191][4]);
        let eq10_e224_d_b5: f64 = (eq10_e222 * s.db[191][5]);
        (eq10_e224, eq10_e224_d_n0, eq10_e224_d_n1, eq10_e224_d_n2, eq10_e224_d_n3, eq10_e224_d_n4, eq10_e224_d_n5, eq10_e224_d_n6, eq10_e224_d_n7, eq10_e224_d_n8, eq10_e224_d_n9, eq10_e224_d_n10, eq10_e224_d_n11, eq10_e224_d_n12, eq10_e224_d_n13, eq10_e224_d_n14, eq10_e224_d_b0, eq10_e224_d_b1, eq10_e224_d_b2, eq10_e224_d_b3, eq10_e224_d_b4, eq10_e224_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e226;
        let eq10_node_derivatives: [f64; 15] = [eq10_e226_d_n0, eq10_e226_d_n1, eq10_e226_d_n2, eq10_e226_d_n3, eq10_e226_d_n4, eq10_e226_d_n5, eq10_e226_d_n6, eq10_e226_d_n7, eq10_e226_d_n8, eq10_e226_d_n9, eq10_e226_d_n10, eq10_e226_d_n11, eq10_e226_d_n12, eq10_e226_d_n13, eq10_e226_d_n14];
        let eq10_branch_derivatives: [f64; 6] = [eq10_e226_d_b0, eq10_e226_d_b1, eq10_e226_d_b2, eq10_e226_d_b3, eq10_e226_d_b4, eq10_e226_d_b5];
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
        let eq11_e228: f64 = (-p.p148);
        let eq11_e230: f64 = (eq11_e228 * s.v[193]);
        let eq11_e230_d_n0: f64 = (eq11_e228 * s.dn[193][0]);
        let eq11_e230_d_n1: f64 = (eq11_e228 * s.dn[193][1]);
        let eq11_e230_d_n2: f64 = (eq11_e228 * s.dn[193][2]);
        let eq11_e230_d_n3: f64 = (eq11_e228 * s.dn[193][3]);
        let eq11_e230_d_n4: f64 = (eq11_e228 * s.dn[193][4]);
        let eq11_e230_d_n5: f64 = (eq11_e228 * s.dn[193][5]);
        let eq11_e230_d_n6: f64 = (eq11_e228 * s.dn[193][6]);
        let eq11_e230_d_n7: f64 = (eq11_e228 * s.dn[193][7]);
        let eq11_e230_d_n8: f64 = (eq11_e228 * s.dn[193][8]);
        let eq11_e230_d_n9: f64 = (eq11_e228 * s.dn[193][9]);
        let eq11_e230_d_n10: f64 = (eq11_e228 * s.dn[193][10]);
        let eq11_e230_d_n11: f64 = (eq11_e228 * s.dn[193][11]);
        let eq11_e230_d_n12: f64 = (eq11_e228 * s.dn[193][12]);
        let eq11_e230_d_n13: f64 = (eq11_e228 * s.dn[193][13]);
        let eq11_e230_d_n14: f64 = (eq11_e228 * s.dn[193][14]);
        let eq11_e230_d_b0: f64 = (eq11_e228 * s.db[193][0]);
        let eq11_e230_d_b1: f64 = (eq11_e228 * s.db[193][1]);
        let eq11_e230_d_b2: f64 = (eq11_e228 * s.db[193][2]);
        let eq11_e230_d_b3: f64 = (eq11_e228 * s.db[193][3]);
        let eq11_e230_d_b4: f64 = (eq11_e228 * s.db[193][4]);
        let eq11_e230_d_b5: f64 = (eq11_e228 * s.db[193][5]);
        let eq11_value: f64 = eq11_e230;
        let eq11_node_derivatives: [f64; 15] = [eq11_e230_d_n0, eq11_e230_d_n1, eq11_e230_d_n2, eq11_e230_d_n3, eq11_e230_d_n4, eq11_e230_d_n5, eq11_e230_d_n6, eq11_e230_d_n7, eq11_e230_d_n8, eq11_e230_d_n9, eq11_e230_d_n10, eq11_e230_d_n11, eq11_e230_d_n12, eq11_e230_d_n13, eq11_e230_d_n14];
        let eq11_branch_derivatives: [f64; 6] = [eq11_e230_d_b0, eq11_e230_d_b1, eq11_e230_d_b2, eq11_e230_d_b3, eq11_e230_d_b4, eq11_e230_d_b5];
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
        let eq12_e234: f64 = (s.v[188] + s.v[189]);
        let eq12_e234_d_n0: f64 = (s.dn[188][0] + s.dn[189][0]);
        let eq12_e234_d_n1: f64 = (s.dn[188][1] + s.dn[189][1]);
        let eq12_e234_d_n2: f64 = (s.dn[188][2] + s.dn[189][2]);
        let eq12_e234_d_n3: f64 = (s.dn[188][3] + s.dn[189][3]);
        let eq12_e234_d_n4: f64 = (s.dn[188][4] + s.dn[189][4]);
        let eq12_e234_d_n5: f64 = (s.dn[188][5] + s.dn[189][5]);
        let eq12_e234_d_n6: f64 = (s.dn[188][6] + s.dn[189][6]);
        let eq12_e234_d_n7: f64 = (s.dn[188][7] + s.dn[189][7]);
        let eq12_e234_d_n8: f64 = (s.dn[188][8] + s.dn[189][8]);
        let eq12_e234_d_n9: f64 = (s.dn[188][9] + s.dn[189][9]);
        let eq12_e234_d_n10: f64 = (s.dn[188][10] + s.dn[189][10]);
        let eq12_e234_d_n11: f64 = (s.dn[188][11] + s.dn[189][11]);
        let eq12_e234_d_n12: f64 = (s.dn[188][12] + s.dn[189][12]);
        let eq12_e234_d_n13: f64 = (s.dn[188][13] + s.dn[189][13]);
        let eq12_e234_d_n14: f64 = (s.dn[188][14] + s.dn[189][14]);
        let eq12_e234_d_b0: f64 = (s.db[188][0] + s.db[189][0]);
        let eq12_e234_d_b1: f64 = (s.db[188][1] + s.db[189][1]);
        let eq12_e234_d_b2: f64 = (s.db[188][2] + s.db[189][2]);
        let eq12_e234_d_b3: f64 = (s.db[188][3] + s.db[189][3]);
        let eq12_e234_d_b4: f64 = (s.db[188][4] + s.db[189][4]);
        let eq12_e234_d_b5: f64 = (s.db[188][5] + s.db[189][5]);
        let eq12_e235: f64 = (p.p148 * eq12_e234);
        let eq12_e235_d_n0: f64 = (p.p148 * eq12_e234_d_n0);
        let eq12_e235_d_n1: f64 = (p.p148 * eq12_e234_d_n1);
        let eq12_e235_d_n2: f64 = (p.p148 * eq12_e234_d_n2);
        let eq12_e235_d_n3: f64 = (p.p148 * eq12_e234_d_n3);
        let eq12_e235_d_n4: f64 = (p.p148 * eq12_e234_d_n4);
        let eq12_e235_d_n5: f64 = (p.p148 * eq12_e234_d_n5);
        let eq12_e235_d_n6: f64 = (p.p148 * eq12_e234_d_n6);
        let eq12_e235_d_n7: f64 = (p.p148 * eq12_e234_d_n7);
        let eq12_e235_d_n8: f64 = (p.p148 * eq12_e234_d_n8);
        let eq12_e235_d_n9: f64 = (p.p148 * eq12_e234_d_n9);
        let eq12_e235_d_n10: f64 = (p.p148 * eq12_e234_d_n10);
        let eq12_e235_d_n11: f64 = (p.p148 * eq12_e234_d_n11);
        let eq12_e235_d_n12: f64 = (p.p148 * eq12_e234_d_n12);
        let eq12_e235_d_n13: f64 = (p.p148 * eq12_e234_d_n13);
        let eq12_e235_d_n14: f64 = (p.p148 * eq12_e234_d_n14);
        let eq12_e235_d_b0: f64 = (p.p148 * eq12_e234_d_b0);
        let eq12_e235_d_b1: f64 = (p.p148 * eq12_e234_d_b1);
        let eq12_e235_d_b2: f64 = (p.p148 * eq12_e234_d_b2);
        let eq12_e235_d_b3: f64 = (p.p148 * eq12_e234_d_b3);
        let eq12_e235_d_b4: f64 = (p.p148 * eq12_e234_d_b4);
        let eq12_e235_d_b5: f64 = (p.p148 * eq12_e234_d_b5);
        let eq12_value: f64 = eq12_e235;
        let eq12_node_derivatives: [f64; 15] = [eq12_e235_d_n0, eq12_e235_d_n1, eq12_e235_d_n2, eq12_e235_d_n3, eq12_e235_d_n4, eq12_e235_d_n5, eq12_e235_d_n6, eq12_e235_d_n7, eq12_e235_d_n8, eq12_e235_d_n9, eq12_e235_d_n10, eq12_e235_d_n11, eq12_e235_d_n12, eq12_e235_d_n13, eq12_e235_d_n14];
        let eq12_branch_derivatives: [f64; 6] = [eq12_e235_d_b0, eq12_e235_d_b1, eq12_e235_d_b2, eq12_e235_d_b3, eq12_e235_d_b4, eq12_e235_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let eq13_e238: f64 = (p.p148 * s.v[180]);
        let eq13_e238_d_n0: f64 = (p.p148 * s.dn[180][0]);
        let eq13_e238_d_n1: f64 = (p.p148 * s.dn[180][1]);
        let eq13_e238_d_n2: f64 = (p.p148 * s.dn[180][2]);
        let eq13_e238_d_n3: f64 = (p.p148 * s.dn[180][3]);
        let eq13_e238_d_n4: f64 = (p.p148 * s.dn[180][4]);
        let eq13_e238_d_n5: f64 = (p.p148 * s.dn[180][5]);
        let eq13_e238_d_n6: f64 = (p.p148 * s.dn[180][6]);
        let eq13_e238_d_n7: f64 = (p.p148 * s.dn[180][7]);
        let eq13_e238_d_n8: f64 = (p.p148 * s.dn[180][8]);
        let eq13_e238_d_n9: f64 = (p.p148 * s.dn[180][9]);
        let eq13_e238_d_n10: f64 = (p.p148 * s.dn[180][10]);
        let eq13_e238_d_n11: f64 = (p.p148 * s.dn[180][11]);
        let eq13_e238_d_n12: f64 = (p.p148 * s.dn[180][12]);
        let eq13_e238_d_n13: f64 = (p.p148 * s.dn[180][13]);
        let eq13_e238_d_n14: f64 = (p.p148 * s.dn[180][14]);
        let eq13_e238_d_b0: f64 = (p.p148 * s.db[180][0]);
        let eq13_e238_d_b1: f64 = (p.p148 * s.db[180][1]);
        let eq13_e238_d_b2: f64 = (p.p148 * s.db[180][2]);
        let eq13_e238_d_b3: f64 = (p.p148 * s.db[180][3]);
        let eq13_e238_d_b4: f64 = (p.p148 * s.db[180][4]);
        let eq13_e238_d_b5: f64 = (p.p148 * s.db[180][5]);
        let eq13_e239: f64 = self.eval_ddt(3, eq13_e238);
        let eq13_e239_d_n0: f64 = self.ddt_jacobian(eq13_e238_d_n0);
        let eq13_e239_d_n1: f64 = self.ddt_jacobian(eq13_e238_d_n1);
        let eq13_e239_d_n2: f64 = self.ddt_jacobian(eq13_e238_d_n2);
        let eq13_e239_d_n3: f64 = self.ddt_jacobian(eq13_e238_d_n3);
        let eq13_e239_d_n4: f64 = self.ddt_jacobian(eq13_e238_d_n4);
        let eq13_e239_d_n5: f64 = self.ddt_jacobian(eq13_e238_d_n5);
        let eq13_e239_d_n6: f64 = self.ddt_jacobian(eq13_e238_d_n6);
        let eq13_e239_d_n7: f64 = self.ddt_jacobian(eq13_e238_d_n7);
        let eq13_e239_d_n8: f64 = self.ddt_jacobian(eq13_e238_d_n8);
        let eq13_e239_d_n9: f64 = self.ddt_jacobian(eq13_e238_d_n9);
        let eq13_e239_d_n10: f64 = self.ddt_jacobian(eq13_e238_d_n10);
        let eq13_e239_d_n11: f64 = self.ddt_jacobian(eq13_e238_d_n11);
        let eq13_e239_d_n12: f64 = self.ddt_jacobian(eq13_e238_d_n12);
        let eq13_e239_d_n13: f64 = self.ddt_jacobian(eq13_e238_d_n13);
        let eq13_e239_d_n14: f64 = self.ddt_jacobian(eq13_e238_d_n14);
        let eq13_e239_d_b0: f64 = self.ddt_jacobian(eq13_e238_d_b0);
        let eq13_e239_d_b1: f64 = self.ddt_jacobian(eq13_e238_d_b1);
        let eq13_e239_d_b2: f64 = self.ddt_jacobian(eq13_e238_d_b2);
        let eq13_e239_d_b3: f64 = self.ddt_jacobian(eq13_e238_d_b3);
        let eq13_e239_d_b4: f64 = self.ddt_jacobian(eq13_e238_d_b4);
        let eq13_e239_d_b5: f64 = self.ddt_jacobian(eq13_e238_d_b5);
        let eq13_value: f64 = eq13_e239;
        let eq13_node_derivatives: [f64; 15] = [eq13_e239_d_n0, eq13_e239_d_n1, eq13_e239_d_n2, eq13_e239_d_n3, eq13_e239_d_n4, eq13_e239_d_n5, eq13_e239_d_n6, eq13_e239_d_n7, eq13_e239_d_n8, eq13_e239_d_n9, eq13_e239_d_n10, eq13_e239_d_n11, eq13_e239_d_n12, eq13_e239_d_n13, eq13_e239_d_n14];
        let eq13_branch_derivatives: [f64; 6] = [eq13_e239_d_b0, eq13_e239_d_b1, eq13_e239_d_b2, eq13_e239_d_b3, eq13_e239_d_b4, eq13_e239_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let eq14_e242: f64 = (p.p148 * s.v[194]);
        let eq14_e242_d_n0: f64 = (p.p148 * s.dn[194][0]);
        let eq14_e242_d_n1: f64 = (p.p148 * s.dn[194][1]);
        let eq14_e242_d_n2: f64 = (p.p148 * s.dn[194][2]);
        let eq14_e242_d_n3: f64 = (p.p148 * s.dn[194][3]);
        let eq14_e242_d_n4: f64 = (p.p148 * s.dn[194][4]);
        let eq14_e242_d_n5: f64 = (p.p148 * s.dn[194][5]);
        let eq14_e242_d_n6: f64 = (p.p148 * s.dn[194][6]);
        let eq14_e242_d_n7: f64 = (p.p148 * s.dn[194][7]);
        let eq14_e242_d_n8: f64 = (p.p148 * s.dn[194][8]);
        let eq14_e242_d_n9: f64 = (p.p148 * s.dn[194][9]);
        let eq14_e242_d_n10: f64 = (p.p148 * s.dn[194][10]);
        let eq14_e242_d_n11: f64 = (p.p148 * s.dn[194][11]);
        let eq14_e242_d_n12: f64 = (p.p148 * s.dn[194][12]);
        let eq14_e242_d_n13: f64 = (p.p148 * s.dn[194][13]);
        let eq14_e242_d_n14: f64 = (p.p148 * s.dn[194][14]);
        let eq14_e242_d_b0: f64 = (p.p148 * s.db[194][0]);
        let eq14_e242_d_b1: f64 = (p.p148 * s.db[194][1]);
        let eq14_e242_d_b2: f64 = (p.p148 * s.db[194][2]);
        let eq14_e242_d_b3: f64 = (p.p148 * s.db[194][3]);
        let eq14_e242_d_b4: f64 = (p.p148 * s.db[194][4]);
        let eq14_e242_d_b5: f64 = (p.p148 * s.db[194][5]);
        let eq14_value: f64 = eq14_e242;
        let eq14_node_derivatives: [f64; 15] = [eq14_e242_d_n0, eq14_e242_d_n1, eq14_e242_d_n2, eq14_e242_d_n3, eq14_e242_d_n4, eq14_e242_d_n5, eq14_e242_d_n6, eq14_e242_d_n7, eq14_e242_d_n8, eq14_e242_d_n9, eq14_e242_d_n10, eq14_e242_d_n11, eq14_e242_d_n12, eq14_e242_d_n13, eq14_e242_d_n14];
        let eq14_branch_derivatives: [f64; 6] = [eq14_e242_d_b0, eq14_e242_d_b1, eq14_e242_d_b2, eq14_e242_d_b3, eq14_e242_d_b4, eq14_e242_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            self.multiplicity * (eq14_value),
            &nodes,
            &eq14_node_derivatives,
            &branches,
            &eq14_branch_derivatives,
            self.multiplicity,
        );
    }
}

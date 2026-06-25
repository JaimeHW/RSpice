#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq15_e246: f64 = (s.v[42] + s.v[199]);
        let eq15_e246_d_n0: f64 = (s.dn[42][0] + s.dn[199][0]);
        let eq15_e246_d_n1: f64 = (s.dn[42][1] + s.dn[199][1]);
        let eq15_e246_d_n2: f64 = (s.dn[42][2] + s.dn[199][2]);
        let eq15_e246_d_n3: f64 = (s.dn[42][3] + s.dn[199][3]);
        let eq15_e246_d_n4: f64 = (s.dn[42][4] + s.dn[199][4]);
        let eq15_e246_d_n5: f64 = (s.dn[42][5] + s.dn[199][5]);
        let eq15_e246_d_n6: f64 = (s.dn[42][6] + s.dn[199][6]);
        let eq15_e246_d_n7: f64 = (s.dn[42][7] + s.dn[199][7]);
        let eq15_e246_d_n8: f64 = (s.dn[42][8] + s.dn[199][8]);
        let eq15_e246_d_n9: f64 = (s.dn[42][9] + s.dn[199][9]);
        let eq15_e246_d_n10: f64 = (s.dn[42][10] + s.dn[199][10]);
        let eq15_e246_d_n11: f64 = (s.dn[42][11] + s.dn[199][11]);
        let eq15_e246_d_n12: f64 = (s.dn[42][12] + s.dn[199][12]);
        let eq15_e246_d_n13: f64 = (s.dn[42][13] + s.dn[199][13]);
        let eq15_e246_d_n14: f64 = (s.dn[42][14] + s.dn[199][14]);
        let eq15_e246_d_b0: f64 = (s.db[42][0] + s.db[199][0]);
        let eq15_e246_d_b1: f64 = (s.db[42][1] + s.db[199][1]);
        let eq15_e246_d_b2: f64 = (s.db[42][2] + s.db[199][2]);
        let eq15_e246_d_b3: f64 = (s.db[42][3] + s.db[199][3]);
        let eq15_e246_d_b4: f64 = (s.db[42][4] + s.db[199][4]);
        let eq15_e246_d_b5: f64 = (s.db[42][5] + s.db[199][5]);
        let eq15_e247: f64 = (p.p148 * eq15_e246);
        let eq15_e247_d_n0: f64 = (p.p148 * eq15_e246_d_n0);
        let eq15_e247_d_n1: f64 = (p.p148 * eq15_e246_d_n1);
        let eq15_e247_d_n2: f64 = (p.p148 * eq15_e246_d_n2);
        let eq15_e247_d_n3: f64 = (p.p148 * eq15_e246_d_n3);
        let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);
        let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);
        let eq15_e247_d_n6: f64 = (p.p148 * eq15_e246_d_n6);
        let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);
        let eq15_e247_d_n8: f64 = (p.p148 * eq15_e246_d_n8);
        let eq15_e247_d_n9: f64 = (p.p148 * eq15_e246_d_n9);
        let eq15_e247_d_n10: f64 = (p.p148 * eq15_e246_d_n10);
        let eq15_e247_d_n11: f64 = (p.p148 * eq15_e246_d_n11);
        let eq15_e247_d_n12: f64 = (p.p148 * eq15_e246_d_n12);
        let eq15_e247_d_n13: f64 = (p.p148 * eq15_e246_d_n13);
        let eq15_e247_d_n14: f64 = (p.p148 * eq15_e246_d_n14);
        let eq15_e247_d_b0: f64 = (p.p148 * eq15_e246_d_b0);
        let eq15_e247_d_b1: f64 = (p.p148 * eq15_e246_d_b1);
        let eq15_e247_d_b2: f64 = (p.p148 * eq15_e246_d_b2);
        let eq15_e247_d_b3: f64 = (p.p148 * eq15_e246_d_b3);
        let eq15_e247_d_b4: f64 = (p.p148 * eq15_e246_d_b4);
        let eq15_e247_d_b5: f64 = (p.p148 * eq15_e246_d_b5);
        let eq15_e248: f64 = self.eval_ddt(4, eq15_e247);
        let eq15_e248_d_n0: f64 = self.ddt_jacobian(eq15_e247_d_n0);
        let eq15_e248_d_n1: f64 = self.ddt_jacobian(eq15_e247_d_n1);
        let eq15_e248_d_n2: f64 = self.ddt_jacobian(eq15_e247_d_n2);
        let eq15_e248_d_n3: f64 = self.ddt_jacobian(eq15_e247_d_n3);
        let eq15_e248_d_n4: f64 = self.ddt_jacobian(eq15_e247_d_n4);
        let eq15_e248_d_n5: f64 = self.ddt_jacobian(eq15_e247_d_n5);
        let eq15_e248_d_n6: f64 = self.ddt_jacobian(eq15_e247_d_n6);
        let eq15_e248_d_n7: f64 = self.ddt_jacobian(eq15_e247_d_n7);
        let eq15_e248_d_n8: f64 = self.ddt_jacobian(eq15_e247_d_n8);
        let eq15_e248_d_n9: f64 = self.ddt_jacobian(eq15_e247_d_n9);
        let eq15_e248_d_n10: f64 = self.ddt_jacobian(eq15_e247_d_n10);
        let eq15_e248_d_n11: f64 = self.ddt_jacobian(eq15_e247_d_n11);
        let eq15_e248_d_n12: f64 = self.ddt_jacobian(eq15_e247_d_n12);
        let eq15_e248_d_n13: f64 = self.ddt_jacobian(eq15_e247_d_n13);
        let eq15_e248_d_n14: f64 = self.ddt_jacobian(eq15_e247_d_n14);
        let eq15_e248_d_b0: f64 = self.ddt_jacobian(eq15_e247_d_b0);
        let eq15_e248_d_b1: f64 = self.ddt_jacobian(eq15_e247_d_b1);
        let eq15_e248_d_b2: f64 = self.ddt_jacobian(eq15_e247_d_b2);
        let eq15_e248_d_b3: f64 = self.ddt_jacobian(eq15_e247_d_b3);
        let eq15_e248_d_b4: f64 = self.ddt_jacobian(eq15_e247_d_b4);
        let eq15_e248_d_b5: f64 = self.ddt_jacobian(eq15_e247_d_b5);
        let eq15_value: f64 = eq15_e248;
        let eq15_node_derivatives: [f64; 15] = [eq15_e248_d_n0, eq15_e248_d_n1, eq15_e248_d_n2, eq15_e248_d_n3, eq15_e248_d_n4, eq15_e248_d_n5, eq15_e248_d_n6, eq15_e248_d_n7, eq15_e248_d_n8, eq15_e248_d_n9, eq15_e248_d_n10, eq15_e248_d_n11, eq15_e248_d_n12, eq15_e248_d_n13, eq15_e248_d_n14];
        let eq15_branch_derivatives: [f64; 6] = [eq15_e248_d_b0, eq15_e248_d_b1, eq15_e248_d_b2, eq15_e248_d_b3, eq15_e248_d_b4, eq15_e248_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq16_e251: f64 = (s.v[172] * (nv7 - nv5));
        let eq16_e251_d_n0: f64 = (s.dn[172][0] * (nv7 - nv5));
        let eq16_e251_d_n1: f64 = (s.dn[172][1] * (nv7 - nv5));
        let eq16_e251_d_n2: f64 = (s.dn[172][2] * (nv7 - nv5));
        let eq16_e251_d_n3: f64 = (s.dn[172][3] * (nv7 - nv5));
        let eq16_e251_d_n4: f64 = (s.dn[172][4] * (nv7 - nv5));
        let eq16_e251_d_n5: f64 = ((s.dn[172][5] * (nv7 - nv5)) + (-s.v[172]));
        let eq16_e251_d_n6: f64 = (s.dn[172][6] * (nv7 - nv5));
        let eq16_e251_d_n7: f64 = ((s.dn[172][7] * (nv7 - nv5)) + s.v[172]);
        let eq16_e251_d_n8: f64 = (s.dn[172][8] * (nv7 - nv5));
        let eq16_e251_d_n9: f64 = (s.dn[172][9] * (nv7 - nv5));
        let eq16_e251_d_n10: f64 = (s.dn[172][10] * (nv7 - nv5));
        let eq16_e251_d_n11: f64 = (s.dn[172][11] * (nv7 - nv5));
        let eq16_e251_d_n12: f64 = (s.dn[172][12] * (nv7 - nv5));
        let eq16_e251_d_n13: f64 = (s.dn[172][13] * (nv7 - nv5));
        let eq16_e251_d_n14: f64 = (s.dn[172][14] * (nv7 - nv5));
        let eq16_e251_d_b0: f64 = (s.db[172][0] * (nv7 - nv5));
        let eq16_e251_d_b1: f64 = (s.db[172][1] * (nv7 - nv5));
        let eq16_e251_d_b2: f64 = (s.db[172][2] * (nv7 - nv5));
        let eq16_e251_d_b3: f64 = (s.db[172][3] * (nv7 - nv5));
        let eq16_e251_d_b4: f64 = (s.db[172][4] * (nv7 - nv5));
        let eq16_e251_d_b5: f64 = (s.db[172][5] * (nv7 - nv5));
        let eq16_e252: f64 = self.eval_ddt(5, eq16_e251);
        let eq16_e252_d_n0: f64 = self.ddt_jacobian(eq16_e251_d_n0);
        let eq16_e252_d_n1: f64 = self.ddt_jacobian(eq16_e251_d_n1);
        let eq16_e252_d_n2: f64 = self.ddt_jacobian(eq16_e251_d_n2);
        let eq16_e252_d_n3: f64 = self.ddt_jacobian(eq16_e251_d_n3);
        let eq16_e252_d_n4: f64 = self.ddt_jacobian(eq16_e251_d_n4);
        let eq16_e252_d_n5: f64 = self.ddt_jacobian(eq16_e251_d_n5);
        let eq16_e252_d_n6: f64 = self.ddt_jacobian(eq16_e251_d_n6);
        let eq16_e252_d_n7: f64 = self.ddt_jacobian(eq16_e251_d_n7);
        let eq16_e252_d_n8: f64 = self.ddt_jacobian(eq16_e251_d_n8);
        let eq16_e252_d_n9: f64 = self.ddt_jacobian(eq16_e251_d_n9);
        let eq16_e252_d_n10: f64 = self.ddt_jacobian(eq16_e251_d_n10);
        let eq16_e252_d_n11: f64 = self.ddt_jacobian(eq16_e251_d_n11);
        let eq16_e252_d_n12: f64 = self.ddt_jacobian(eq16_e251_d_n12);
        let eq16_e252_d_n13: f64 = self.ddt_jacobian(eq16_e251_d_n13);
        let eq16_e252_d_n14: f64 = self.ddt_jacobian(eq16_e251_d_n14);
        let eq16_e252_d_b0: f64 = self.ddt_jacobian(eq16_e251_d_b0);
        let eq16_e252_d_b1: f64 = self.ddt_jacobian(eq16_e251_d_b1);
        let eq16_e252_d_b2: f64 = self.ddt_jacobian(eq16_e251_d_b2);
        let eq16_e252_d_b3: f64 = self.ddt_jacobian(eq16_e251_d_b3);
        let eq16_e252_d_b4: f64 = self.ddt_jacobian(eq16_e251_d_b4);
        let eq16_e252_d_b5: f64 = self.ddt_jacobian(eq16_e251_d_b5);
        let eq16_value: f64 = eq16_e252;
        let eq16_node_derivatives: [f64; 15] = [eq16_e252_d_n0, eq16_e252_d_n1, eq16_e252_d_n2, eq16_e252_d_n3, eq16_e252_d_n4, eq16_e252_d_n5, eq16_e252_d_n6, eq16_e252_d_n7, eq16_e252_d_n8, eq16_e252_d_n9, eq16_e252_d_n10, eq16_e252_d_n11, eq16_e252_d_n12, eq16_e252_d_n13, eq16_e252_d_n14];
        let eq16_branch_derivatives: [f64; 6] = [eq16_e252_d_b0, eq16_e252_d_b1, eq16_e252_d_b2, eq16_e252_d_b3, eq16_e252_d_b4, eq16_e252_d_b5];
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
        let eq17_e255: f64 = (p.p148 * s.v[41]);
        let eq17_e255_d_n0: f64 = (p.p148 * s.dn[41][0]);
        let eq17_e255_d_n1: f64 = (p.p148 * s.dn[41][1]);
        let eq17_e255_d_n2: f64 = (p.p148 * s.dn[41][2]);
        let eq17_e255_d_n3: f64 = (p.p148 * s.dn[41][3]);
        let eq17_e255_d_n4: f64 = (p.p148 * s.dn[41][4]);
        let eq17_e255_d_n5: f64 = (p.p148 * s.dn[41][5]);
        let eq17_e255_d_n6: f64 = (p.p148 * s.dn[41][6]);
        let eq17_e255_d_n7: f64 = (p.p148 * s.dn[41][7]);
        let eq17_e255_d_n8: f64 = (p.p148 * s.dn[41][8]);
        let eq17_e255_d_n9: f64 = (p.p148 * s.dn[41][9]);
        let eq17_e255_d_n10: f64 = (p.p148 * s.dn[41][10]);
        let eq17_e255_d_n11: f64 = (p.p148 * s.dn[41][11]);
        let eq17_e255_d_n12: f64 = (p.p148 * s.dn[41][12]);
        let eq17_e255_d_n13: f64 = (p.p148 * s.dn[41][13]);
        let eq17_e255_d_n14: f64 = (p.p148 * s.dn[41][14]);
        let eq17_e255_d_b0: f64 = (p.p148 * s.db[41][0]);
        let eq17_e255_d_b1: f64 = (p.p148 * s.db[41][1]);
        let eq17_e255_d_b2: f64 = (p.p148 * s.db[41][2]);
        let eq17_e255_d_b3: f64 = (p.p148 * s.db[41][3]);
        let eq17_e255_d_b4: f64 = (p.p148 * s.db[41][4]);
        let eq17_e255_d_b5: f64 = (p.p148 * s.db[41][5]);
        let eq17_e256: f64 = self.eval_ddt(6, eq17_e255);
        let eq17_e256_d_n0: f64 = self.ddt_jacobian(eq17_e255_d_n0);
        let eq17_e256_d_n1: f64 = self.ddt_jacobian(eq17_e255_d_n1);
        let eq17_e256_d_n2: f64 = self.ddt_jacobian(eq17_e255_d_n2);
        let eq17_e256_d_n3: f64 = self.ddt_jacobian(eq17_e255_d_n3);
        let eq17_e256_d_n4: f64 = self.ddt_jacobian(eq17_e255_d_n4);
        let eq17_e256_d_n5: f64 = self.ddt_jacobian(eq17_e255_d_n5);
        let eq17_e256_d_n6: f64 = self.ddt_jacobian(eq17_e255_d_n6);
        let eq17_e256_d_n7: f64 = self.ddt_jacobian(eq17_e255_d_n7);
        let eq17_e256_d_n8: f64 = self.ddt_jacobian(eq17_e255_d_n8);
        let eq17_e256_d_n9: f64 = self.ddt_jacobian(eq17_e255_d_n9);
        let eq17_e256_d_n10: f64 = self.ddt_jacobian(eq17_e255_d_n10);
        let eq17_e256_d_n11: f64 = self.ddt_jacobian(eq17_e255_d_n11);
        let eq17_e256_d_n12: f64 = self.ddt_jacobian(eq17_e255_d_n12);
        let eq17_e256_d_n13: f64 = self.ddt_jacobian(eq17_e255_d_n13);
        let eq17_e256_d_n14: f64 = self.ddt_jacobian(eq17_e255_d_n14);
        let eq17_e256_d_b0: f64 = self.ddt_jacobian(eq17_e255_d_b0);
        let eq17_e256_d_b1: f64 = self.ddt_jacobian(eq17_e255_d_b1);
        let eq17_e256_d_b2: f64 = self.ddt_jacobian(eq17_e255_d_b2);
        let eq17_e256_d_b3: f64 = self.ddt_jacobian(eq17_e255_d_b3);
        let eq17_e256_d_b4: f64 = self.ddt_jacobian(eq17_e255_d_b4);
        let eq17_e256_d_b5: f64 = self.ddt_jacobian(eq17_e255_d_b5);
        let eq17_value: f64 = eq17_e256;
        let eq17_node_derivatives: [f64; 15] = [eq17_e256_d_n0, eq17_e256_d_n1, eq17_e256_d_n2, eq17_e256_d_n3, eq17_e256_d_n4, eq17_e256_d_n5, eq17_e256_d_n6, eq17_e256_d_n7, eq17_e256_d_n8, eq17_e256_d_n9, eq17_e256_d_n10, eq17_e256_d_n11, eq17_e256_d_n12, eq17_e256_d_n13, eq17_e256_d_n14];
        let eq17_branch_derivatives: [f64; 6] = [eq17_e256_d_b0, eq17_e256_d_b1, eq17_e256_d_b2, eq17_e256_d_b3, eq17_e256_d_b4, eq17_e256_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq18_e259: f64 = (s.v[171] * (nv1 - nv5));
        let eq18_e259_d_n0: f64 = (s.dn[171][0] * (nv1 - nv5));
        let eq18_e259_d_n1: f64 = ((s.dn[171][1] * (nv1 - nv5)) + s.v[171]);
        let eq18_e259_d_n2: f64 = (s.dn[171][2] * (nv1 - nv5));
        let eq18_e259_d_n3: f64 = (s.dn[171][3] * (nv1 - nv5));
        let eq18_e259_d_n4: f64 = (s.dn[171][4] * (nv1 - nv5));
        let eq18_e259_d_n5: f64 = ((s.dn[171][5] * (nv1 - nv5)) + (-s.v[171]));
        let eq18_e259_d_n6: f64 = (s.dn[171][6] * (nv1 - nv5));
        let eq18_e259_d_n7: f64 = (s.dn[171][7] * (nv1 - nv5));
        let eq18_e259_d_n8: f64 = (s.dn[171][8] * (nv1 - nv5));
        let eq18_e259_d_n9: f64 = (s.dn[171][9] * (nv1 - nv5));
        let eq18_e259_d_n10: f64 = (s.dn[171][10] * (nv1 - nv5));
        let eq18_e259_d_n11: f64 = (s.dn[171][11] * (nv1 - nv5));
        let eq18_e259_d_n12: f64 = (s.dn[171][12] * (nv1 - nv5));
        let eq18_e259_d_n13: f64 = (s.dn[171][13] * (nv1 - nv5));
        let eq18_e259_d_n14: f64 = (s.dn[171][14] * (nv1 - nv5));
        let eq18_e259_d_b0: f64 = (s.db[171][0] * (nv1 - nv5));
        let eq18_e259_d_b1: f64 = (s.db[171][1] * (nv1 - nv5));
        let eq18_e259_d_b2: f64 = (s.db[171][2] * (nv1 - nv5));
        let eq18_e259_d_b3: f64 = (s.db[171][3] * (nv1 - nv5));
        let eq18_e259_d_b4: f64 = (s.db[171][4] * (nv1 - nv5));
        let eq18_e259_d_b5: f64 = (s.db[171][5] * (nv1 - nv5));
        let eq18_e260: f64 = self.eval_ddt(7, eq18_e259);
        let eq18_e260_d_n0: f64 = self.ddt_jacobian(eq18_e259_d_n0);
        let eq18_e260_d_n1: f64 = self.ddt_jacobian(eq18_e259_d_n1);
        let eq18_e260_d_n2: f64 = self.ddt_jacobian(eq18_e259_d_n2);
        let eq18_e260_d_n3: f64 = self.ddt_jacobian(eq18_e259_d_n3);
        let eq18_e260_d_n4: f64 = self.ddt_jacobian(eq18_e259_d_n4);
        let eq18_e260_d_n5: f64 = self.ddt_jacobian(eq18_e259_d_n5);
        let eq18_e260_d_n6: f64 = self.ddt_jacobian(eq18_e259_d_n6);
        let eq18_e260_d_n7: f64 = self.ddt_jacobian(eq18_e259_d_n7);
        let eq18_e260_d_n8: f64 = self.ddt_jacobian(eq18_e259_d_n8);
        let eq18_e260_d_n9: f64 = self.ddt_jacobian(eq18_e259_d_n9);
        let eq18_e260_d_n10: f64 = self.ddt_jacobian(eq18_e259_d_n10);
        let eq18_e260_d_n11: f64 = self.ddt_jacobian(eq18_e259_d_n11);
        let eq18_e260_d_n12: f64 = self.ddt_jacobian(eq18_e259_d_n12);
        let eq18_e260_d_n13: f64 = self.ddt_jacobian(eq18_e259_d_n13);
        let eq18_e260_d_n14: f64 = self.ddt_jacobian(eq18_e259_d_n14);
        let eq18_e260_d_b0: f64 = self.ddt_jacobian(eq18_e259_d_b0);
        let eq18_e260_d_b1: f64 = self.ddt_jacobian(eq18_e259_d_b1);
        let eq18_e260_d_b2: f64 = self.ddt_jacobian(eq18_e259_d_b2);
        let eq18_e260_d_b3: f64 = self.ddt_jacobian(eq18_e259_d_b3);
        let eq18_e260_d_b4: f64 = self.ddt_jacobian(eq18_e259_d_b4);
        let eq18_e260_d_b5: f64 = self.ddt_jacobian(eq18_e259_d_b5);
        let eq18_value: f64 = eq18_e260;
        let eq18_node_derivatives: [f64; 15] = [eq18_e260_d_n0, eq18_e260_d_n1, eq18_e260_d_n2, eq18_e260_d_n3, eq18_e260_d_n4, eq18_e260_d_n5, eq18_e260_d_n6, eq18_e260_d_n7, eq18_e260_d_n8, eq18_e260_d_n9, eq18_e260_d_n10, eq18_e260_d_n11, eq18_e260_d_n12, eq18_e260_d_n13, eq18_e260_d_n14];
        let eq18_branch_derivatives: [f64; 6] = [eq18_e260_d_b0, eq18_e260_d_b1, eq18_e260_d_b2, eq18_e260_d_b3, eq18_e260_d_b4, eq18_e260_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq19_e266, eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14, eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5,) = {
    if (s.v[511] != 0.0) {
        let eq19_e264: f64 = ((nv1 - nv7) / s.v[71]);
        let eq19_e264_d_n0: f64 = (-(((nv1 - nv7) * s.dn[71][0]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n1: f64 = ((s.v[71] - ((nv1 - nv7) * s.dn[71][1])) / (s.v[71] * s.v[71]));
        let eq19_e264_d_n2: f64 = (-(((nv1 - nv7) * s.dn[71][2]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n3: f64 = (-(((nv1 - nv7) * s.dn[71][3]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n4: f64 = (-(((nv1 - nv7) * s.dn[71][4]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n5: f64 = (-(((nv1 - nv7) * s.dn[71][5]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n6: f64 = (-(((nv1 - nv7) * s.dn[71][6]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n7: f64 = (((-s.v[71]) - ((nv1 - nv7) * s.dn[71][7])) / (s.v[71] * s.v[71]));
        let eq19_e264_d_n8: f64 = (-(((nv1 - nv7) * s.dn[71][8]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n9: f64 = (-(((nv1 - nv7) * s.dn[71][9]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n10: f64 = (-(((nv1 - nv7) * s.dn[71][10]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n11: f64 = (-(((nv1 - nv7) * s.dn[71][11]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n12: f64 = (-(((nv1 - nv7) * s.dn[71][12]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n13: f64 = (-(((nv1 - nv7) * s.dn[71][13]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_n14: f64 = (-(((nv1 - nv7) * s.dn[71][14]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b0: f64 = (-(((nv1 - nv7) * s.db[71][0]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b1: f64 = (-(((nv1 - nv7) * s.db[71][1]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b2: f64 = (-(((nv1 - nv7) * s.db[71][2]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b3: f64 = (-(((nv1 - nv7) * s.db[71][3]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b4: f64 = (-(((nv1 - nv7) * s.db[71][4]) / (s.v[71] * s.v[71])));
        let eq19_e264_d_b5: f64 = (-(((nv1 - nv7) * s.db[71][5]) / (s.v[71] * s.v[71])));
        (eq19_e264, eq19_e264_d_n0, eq19_e264_d_n1, eq19_e264_d_n2, eq19_e264_d_n3, eq19_e264_d_n4, eq19_e264_d_n5, eq19_e264_d_n6, eq19_e264_d_n7, eq19_e264_d_n8, eq19_e264_d_n9, eq19_e264_d_n10, eq19_e264_d_n11, eq19_e264_d_n12, eq19_e264_d_n13, eq19_e264_d_n14, eq19_e264_d_b0, eq19_e264_d_b1, eq19_e264_d_b2, eq19_e264_d_b3, eq19_e264_d_b4, eq19_e264_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e266;
        let eq19_node_derivatives: [f64; 15] = [eq19_e266_d_n0, eq19_e266_d_n1, eq19_e266_d_n2, eq19_e266_d_n3, eq19_e266_d_n4, eq19_e266_d_n5, eq19_e266_d_n6, eq19_e266_d_n7, eq19_e266_d_n8, eq19_e266_d_n9, eq19_e266_d_n10, eq19_e266_d_n11, eq19_e266_d_n12, eq19_e266_d_n13, eq19_e266_d_n14];
        let eq19_branch_derivatives: [f64; 6] = [eq19_e266_d_b0, eq19_e266_d_b1, eq19_e266_d_b2, eq19_e266_d_b3, eq19_e266_d_b4, eq19_e266_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[7]),
            self.multiplicity * (eq19_value),
            &nodes,
            &eq19_node_derivatives,
            &branches,
            &eq19_branch_derivatives,
            self.multiplicity,
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
        let (eq20_e271,) = {
    if (!(s.v[511] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e271;
        stamper.stamp_potential(
            branches[1],
            eq20_value,
            &[
            ],
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq21_e277, eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14, eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5,) = {
    if (s.v[512] != 0.0) {
        let eq21_e275: f64 = ((nv6 - nv2) / s.v[73]);
        let eq21_e275_d_n0: f64 = (-(((nv6 - nv2) * s.dn[73][0]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n1: f64 = (-(((nv6 - nv2) * s.dn[73][1]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n2: f64 = (((-s.v[73]) - ((nv6 - nv2) * s.dn[73][2])) / (s.v[73] * s.v[73]));
        let eq21_e275_d_n3: f64 = (-(((nv6 - nv2) * s.dn[73][3]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n4: f64 = (-(((nv6 - nv2) * s.dn[73][4]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n5: f64 = (-(((nv6 - nv2) * s.dn[73][5]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n6: f64 = ((s.v[73] - ((nv6 - nv2) * s.dn[73][6])) / (s.v[73] * s.v[73]));
        let eq21_e275_d_n7: f64 = (-(((nv6 - nv2) * s.dn[73][7]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n8: f64 = (-(((nv6 - nv2) * s.dn[73][8]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n9: f64 = (-(((nv6 - nv2) * s.dn[73][9]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n10: f64 = (-(((nv6 - nv2) * s.dn[73][10]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n11: f64 = (-(((nv6 - nv2) * s.dn[73][11]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n12: f64 = (-(((nv6 - nv2) * s.dn[73][12]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n13: f64 = (-(((nv6 - nv2) * s.dn[73][13]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_n14: f64 = (-(((nv6 - nv2) * s.dn[73][14]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b0: f64 = (-(((nv6 - nv2) * s.db[73][0]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b1: f64 = (-(((nv6 - nv2) * s.db[73][1]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b2: f64 = (-(((nv6 - nv2) * s.db[73][2]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b3: f64 = (-(((nv6 - nv2) * s.db[73][3]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b4: f64 = (-(((nv6 - nv2) * s.db[73][4]) / (s.v[73] * s.v[73])));
        let eq21_e275_d_b5: f64 = (-(((nv6 - nv2) * s.db[73][5]) / (s.v[73] * s.v[73])));
        (eq21_e275, eq21_e275_d_n0, eq21_e275_d_n1, eq21_e275_d_n2, eq21_e275_d_n3, eq21_e275_d_n4, eq21_e275_d_n5, eq21_e275_d_n6, eq21_e275_d_n7, eq21_e275_d_n8, eq21_e275_d_n9, eq21_e275_d_n10, eq21_e275_d_n11, eq21_e275_d_n12, eq21_e275_d_n13, eq21_e275_d_n14, eq21_e275_d_b0, eq21_e275_d_b1, eq21_e275_d_b2, eq21_e275_d_b3, eq21_e275_d_b4, eq21_e275_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e277;
        let eq21_node_derivatives: [f64; 15] = [eq21_e277_d_n0, eq21_e277_d_n1, eq21_e277_d_n2, eq21_e277_d_n3, eq21_e277_d_n4, eq21_e277_d_n5, eq21_e277_d_n6, eq21_e277_d_n7, eq21_e277_d_n8, eq21_e277_d_n9, eq21_e277_d_n10, eq21_e277_d_n11, eq21_e277_d_n12, eq21_e277_d_n13, eq21_e277_d_n14];
        let eq21_branch_derivatives: [f64; 6] = [eq21_e277_d_b0, eq21_e277_d_b1, eq21_e277_d_b2, eq21_e277_d_b3, eq21_e277_d_b4, eq21_e277_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[2]),
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
        let (eq22_e282,) = {
    if (!(s.v[512] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e282;
        stamper.stamp_potential(
            branches[2],
            eq22_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_23_block_0(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq23_e288, eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14, eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5,) = {
    if (s.v[513] != 0.0) {
        let eq23_e286: f64 = ((nv5 - nv0) / s.v[72]);
        let eq23_e286_d_n0: f64 = (((-s.v[72]) - ((nv5 - nv0) * s.dn[72][0])) / (s.v[72] * s.v[72]));
        let eq23_e286_d_n1: f64 = (-(((nv5 - nv0) * s.dn[72][1]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n2: f64 = (-(((nv5 - nv0) * s.dn[72][2]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n3: f64 = (-(((nv5 - nv0) * s.dn[72][3]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n4: f64 = (-(((nv5 - nv0) * s.dn[72][4]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n5: f64 = ((s.v[72] - ((nv5 - nv0) * s.dn[72][5])) / (s.v[72] * s.v[72]));
        let eq23_e286_d_n6: f64 = (-(((nv5 - nv0) * s.dn[72][6]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n7: f64 = (-(((nv5 - nv0) * s.dn[72][7]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n8: f64 = (-(((nv5 - nv0) * s.dn[72][8]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n9: f64 = (-(((nv5 - nv0) * s.dn[72][9]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n10: f64 = (-(((nv5 - nv0) * s.dn[72][10]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n11: f64 = (-(((nv5 - nv0) * s.dn[72][11]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n12: f64 = (-(((nv5 - nv0) * s.dn[72][12]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n13: f64 = (-(((nv5 - nv0) * s.dn[72][13]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_n14: f64 = (-(((nv5 - nv0) * s.dn[72][14]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b0: f64 = (-(((nv5 - nv0) * s.db[72][0]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b1: f64 = (-(((nv5 - nv0) * s.db[72][1]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b2: f64 = (-(((nv5 - nv0) * s.db[72][2]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b3: f64 = (-(((nv5 - nv0) * s.db[72][3]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b4: f64 = (-(((nv5 - nv0) * s.db[72][4]) / (s.v[72] * s.v[72])));
        let eq23_e286_d_b5: f64 = (-(((nv5 - nv0) * s.db[72][5]) / (s.v[72] * s.v[72])));
        (eq23_e286, eq23_e286_d_n0, eq23_e286_d_n1, eq23_e286_d_n2, eq23_e286_d_n3, eq23_e286_d_n4, eq23_e286_d_n5, eq23_e286_d_n6, eq23_e286_d_n7, eq23_e286_d_n8, eq23_e286_d_n9, eq23_e286_d_n10, eq23_e286_d_n11, eq23_e286_d_n12, eq23_e286_d_n13, eq23_e286_d_n14, eq23_e286_d_b0, eq23_e286_d_b1, eq23_e286_d_b2, eq23_e286_d_b3, eq23_e286_d_b4, eq23_e286_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e288;
        let eq23_node_derivatives: [f64; 15] = [eq23_e288_d_n0, eq23_e288_d_n1, eq23_e288_d_n2, eq23_e288_d_n3, eq23_e288_d_n4, eq23_e288_d_n5, eq23_e288_d_n6, eq23_e288_d_n7, eq23_e288_d_n8, eq23_e288_d_n9, eq23_e288_d_n10, eq23_e288_d_n11, eq23_e288_d_n12, eq23_e288_d_n13, eq23_e288_d_n14];
        let eq23_branch_derivatives: [f64; 6] = [eq23_e288_d_b0, eq23_e288_d_b1, eq23_e288_d_b2, eq23_e288_d_b3, eq23_e288_d_b4, eq23_e288_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[0]),
            self.multiplicity * (eq23_value),
            &nodes,
            &eq23_node_derivatives,
            &branches,
            &eq23_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_24_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq24_e293,) = {
    if (!(s.v[513] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e293;
        stamper.stamp_potential(
            branches[3],
            eq24_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_25_block_0(
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
        let eq25_e296: f64 = (s.v[174] * (nv7 - nv2));
        let eq25_e296_d_n2: f64 = (-s.v[174]);
        let eq25_e296_d_n7: f64 = s.v[174];
        let eq25_e297: f64 = self.eval_ddt(8, eq25_e296);
        let eq25_e297_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n2: f64 = self.ddt_jacobian(eq25_e296_d_n2);
        let eq25_e297_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n7: f64 = self.ddt_jacobian(eq25_e296_d_n7);
        let eq25_e297_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq25_e297_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq25_value: f64 = eq25_e297;
        let eq25_node_derivatives: [f64; 15] = [eq25_e297_d_n0, eq25_e297_d_n1, eq25_e297_d_n2, eq25_e297_d_n3, eq25_e297_d_n4, eq25_e297_d_n5, eq25_e297_d_n6, eq25_e297_d_n7, eq25_e297_d_n8, eq25_e297_d_n9, eq25_e297_d_n10, eq25_e297_d_n11, eq25_e297_d_n12, eq25_e297_d_n13, eq25_e297_d_n14];
        let eq25_branch_derivatives: [f64; 6] = [eq25_e297_d_b0, eq25_e297_d_b1, eq25_e297_d_b2, eq25_e297_d_b3, eq25_e297_d_b4, eq25_e297_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq25_value),
            &nodes,
            &eq25_node_derivatives,
            &branches,
            &eq25_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let eq26_e300: f64 = (s.v[173] * (nv1 - nv2));
        let eq26_e300_d_n1: f64 = s.v[173];
        let eq26_e300_d_n2: f64 = (-s.v[173]);
        let eq26_e301: f64 = self.eval_ddt(9, eq26_e300);
        let eq26_e301_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n1: f64 = self.ddt_jacobian(eq26_e300_d_n1);
        let eq26_e301_d_n2: f64 = self.ddt_jacobian(eq26_e300_d_n2);
        let eq26_e301_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq26_e301_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq26_value: f64 = eq26_e301;
        let eq26_node_derivatives: [f64; 15] = [eq26_e301_d_n0, eq26_e301_d_n1, eq26_e301_d_n2, eq26_e301_d_n3, eq26_e301_d_n4, eq26_e301_d_n5, eq26_e301_d_n6, eq26_e301_d_n7, eq26_e301_d_n8, eq26_e301_d_n9, eq26_e301_d_n10, eq26_e301_d_n11, eq26_e301_d_n12, eq26_e301_d_n13, eq26_e301_d_n14];
        let eq26_branch_derivatives: [f64; 6] = [eq26_e301_d_b0, eq26_e301_d_b1, eq26_e301_d_b2, eq26_e301_d_b3, eq26_e301_d_b4, eq26_e301_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            self.multiplicity * (eq26_value),
            &nodes,
            &eq26_node_derivatives,
            &branches,
            &eq26_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_27_block_0(
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let eq27_e304: f64 = (p.p108 * (nv0 - nv2));
        let eq27_e304_d_n0: f64 = p.p108;
        let eq27_e304_d_n2: f64 = (-p.p108);
        let eq27_e305: f64 = self.eval_ddt(10, eq27_e304);
        let eq27_e305_d_n0: f64 = self.ddt_jacobian(eq27_e304_d_n0);
        let eq27_e305_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n2: f64 = self.ddt_jacobian(eq27_e304_d_n2);
        let eq27_e305_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_b4: f64 = self.ddt_jacobian(0.0);
        let eq27_e305_d_b5: f64 = self.ddt_jacobian(0.0);
        let eq27_value: f64 = eq27_e305;
        let eq27_node_derivatives: [f64; 15] = [eq27_e305_d_n0, eq27_e305_d_n1, eq27_e305_d_n2, eq27_e305_d_n3, eq27_e305_d_n4, eq27_e305_d_n5, eq27_e305_d_n6, eq27_e305_d_n7, eq27_e305_d_n8, eq27_e305_d_n9, eq27_e305_d_n10, eq27_e305_d_n11, eq27_e305_d_n12, eq27_e305_d_n13, eq27_e305_d_n14];
        let eq27_branch_derivatives: [f64; 6] = [eq27_e305_d_b0, eq27_e305_d_b1, eq27_e305_d_b2, eq27_e305_d_b3, eq27_e305_d_b4, eq27_e305_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_28_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq28_e308: f64 = (p.p148 * s.v[198]);
        let eq28_e308_d_n0: f64 = (p.p148 * s.dn[198][0]);
        let eq28_e308_d_n1: f64 = (p.p148 * s.dn[198][1]);
        let eq28_e308_d_n2: f64 = (p.p148 * s.dn[198][2]);
        let eq28_e308_d_n3: f64 = (p.p148 * s.dn[198][3]);
        let eq28_e308_d_n4: f64 = (p.p148 * s.dn[198][4]);
        let eq28_e308_d_n5: f64 = (p.p148 * s.dn[198][5]);
        let eq28_e308_d_n6: f64 = (p.p148 * s.dn[198][6]);
        let eq28_e308_d_n7: f64 = (p.p148 * s.dn[198][7]);
        let eq28_e308_d_n8: f64 = (p.p148 * s.dn[198][8]);
        let eq28_e308_d_n9: f64 = (p.p148 * s.dn[198][9]);
        let eq28_e308_d_n10: f64 = (p.p148 * s.dn[198][10]);
        let eq28_e308_d_n11: f64 = (p.p148 * s.dn[198][11]);
        let eq28_e308_d_n12: f64 = (p.p148 * s.dn[198][12]);
        let eq28_e308_d_n13: f64 = (p.p148 * s.dn[198][13]);
        let eq28_e308_d_n14: f64 = (p.p148 * s.dn[198][14]);
        let eq28_e308_d_b0: f64 = (p.p148 * s.db[198][0]);
        let eq28_e308_d_b1: f64 = (p.p148 * s.db[198][1]);
        let eq28_e308_d_b2: f64 = (p.p148 * s.db[198][2]);
        let eq28_e308_d_b3: f64 = (p.p148 * s.db[198][3]);
        let eq28_e308_d_b4: f64 = (p.p148 * s.db[198][4]);
        let eq28_e308_d_b5: f64 = (p.p148 * s.db[198][5]);
        let eq28_value: f64 = eq28_e308;
        let eq28_node_derivatives: [f64; 15] = [eq28_e308_d_n0, eq28_e308_d_n1, eq28_e308_d_n2, eq28_e308_d_n3, eq28_e308_d_n4, eq28_e308_d_n5, eq28_e308_d_n6, eq28_e308_d_n7, eq28_e308_d_n8, eq28_e308_d_n9, eq28_e308_d_n10, eq28_e308_d_n11, eq28_e308_d_n12, eq28_e308_d_n13, eq28_e308_d_n14];
        let eq28_branch_derivatives: [f64; 6] = [eq28_e308_d_b0, eq28_e308_d_b1, eq28_e308_d_b2, eq28_e308_d_b3, eq28_e308_d_b4, eq28_e308_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq28_value),
            &nodes,
            &eq28_node_derivatives,
            &branches,
            &eq28_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_29_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq29_e316, eq29_e316_d_n0, eq29_e316_d_n1, eq29_e316_d_n2, eq29_e316_d_n3, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9, eq29_e316_d_n10, eq29_e316_d_n11, eq29_e316_d_n12, eq29_e316_d_n13, eq29_e316_d_n14, eq29_e316_d_b0, eq29_e316_d_b1, eq29_e316_d_b2, eq29_e316_d_b3, eq29_e316_d_b4, eq29_e316_d_b5,) = {
    if ((s.v[514] != 0.0) && (s.v[515] != 0.0)) {
        let eq29_e314: f64 = (p.p148 * s.v[195]);
        let eq29_e314_d_n0: f64 = (p.p148 * s.dn[195][0]);
        let eq29_e314_d_n1: f64 = (p.p148 * s.dn[195][1]);
        let eq29_e314_d_n2: f64 = (p.p148 * s.dn[195][2]);
        let eq29_e314_d_n3: f64 = (p.p148 * s.dn[195][3]);
        let eq29_e314_d_n4: f64 = (p.p148 * s.dn[195][4]);
        let eq29_e314_d_n5: f64 = (p.p148 * s.dn[195][5]);
        let eq29_e314_d_n6: f64 = (p.p148 * s.dn[195][6]);
        let eq29_e314_d_n7: f64 = (p.p148 * s.dn[195][7]);
        let eq29_e314_d_n8: f64 = (p.p148 * s.dn[195][8]);
        let eq29_e314_d_n9: f64 = (p.p148 * s.dn[195][9]);
        let eq29_e314_d_n10: f64 = (p.p148 * s.dn[195][10]);
        let eq29_e314_d_n11: f64 = (p.p148 * s.dn[195][11]);
        let eq29_e314_d_n12: f64 = (p.p148 * s.dn[195][12]);
        let eq29_e314_d_n13: f64 = (p.p148 * s.dn[195][13]);
        let eq29_e314_d_n14: f64 = (p.p148 * s.dn[195][14]);
        let eq29_e314_d_b0: f64 = (p.p148 * s.db[195][0]);
        let eq29_e314_d_b1: f64 = (p.p148 * s.db[195][1]);
        let eq29_e314_d_b2: f64 = (p.p148 * s.db[195][2]);
        let eq29_e314_d_b3: f64 = (p.p148 * s.db[195][3]);
        let eq29_e314_d_b4: f64 = (p.p148 * s.db[195][4]);
        let eq29_e314_d_b5: f64 = (p.p148 * s.db[195][5]);
        (eq29_e314, eq29_e314_d_n0, eq29_e314_d_n1, eq29_e314_d_n2, eq29_e314_d_n3, eq29_e314_d_n4, eq29_e314_d_n5, eq29_e314_d_n6, eq29_e314_d_n7, eq29_e314_d_n8, eq29_e314_d_n9, eq29_e314_d_n10, eq29_e314_d_n11, eq29_e314_d_n12, eq29_e314_d_n13, eq29_e314_d_n14, eq29_e314_d_b0, eq29_e314_d_b1, eq29_e314_d_b2, eq29_e314_d_b3, eq29_e314_d_b4, eq29_e314_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e316;
        let eq29_node_derivatives: [f64; 15] = [eq29_e316_d_n0, eq29_e316_d_n1, eq29_e316_d_n2, eq29_e316_d_n3, eq29_e316_d_n4, eq29_e316_d_n5, eq29_e316_d_n6, eq29_e316_d_n7, eq29_e316_d_n8, eq29_e316_d_n9, eq29_e316_d_n10, eq29_e316_d_n11, eq29_e316_d_n12, eq29_e316_d_n13, eq29_e316_d_n14];
        let eq29_branch_derivatives: [f64; 6] = [eq29_e316_d_b0, eq29_e316_d_b1, eq29_e316_d_b2, eq29_e316_d_b3, eq29_e316_d_b4, eq29_e316_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_30_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq30_e324, eq30_e324_d_n5, eq30_e324_d_n9,) = {
    if ((s.v[514] != 0.0) && (s.v[515] != 0.0)) {
        let eq30_e322: f64 = (s.v[233] * (nv9 - nv5));
        let eq30_e322_d_n5: f64 = (-s.v[233]);
        let eq30_e322_d_n9: f64 = s.v[233];
        (eq30_e322, eq30_e322_d_n5, eq30_e322_d_n9,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e324;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[5]),
            self.multiplicity * (eq30_value),
            &[
                GeneratedDerivative::node(nodes[5], self.multiplicity * eq30_e324_d_n5),
                GeneratedDerivative::node(nodes[9], self.multiplicity * eq30_e324_d_n9),
            ],
        );
    }
}

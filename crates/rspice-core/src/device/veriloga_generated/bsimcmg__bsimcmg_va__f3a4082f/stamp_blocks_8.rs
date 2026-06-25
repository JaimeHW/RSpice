#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq71_e2519, eq71_e2519_d_n0, eq71_e2519_d_n1, eq71_e2519_d_n2, eq71_e2519_d_n3, eq71_e2519_d_n4, eq71_e2519_d_n5, eq71_e2519_d_n6, eq71_e2519_d_n7, eq71_e2519_d_n8, eq71_e2519_d_n9, eq71_e2519_d_n10, eq71_e2519_d_n11, eq71_e2519_d_n12, eq71_e2519_d_n13, eq71_e2519_d_n14, eq71_e2519_d_n15, eq71_e2519_d_n16,) = {
    if (s.v[1723] != 0.0) {
        let eq71_e2516: f64 = self.eval_ddt(24, (nv15 - 0.0));
        let eq71_e2516_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq71_e2516_d_n15: f64 = self.ddt_jacobian(1.0);
        let eq71_e2516_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq71_e2517: f64 = (1e-9 * eq71_e2516);
        let eq71_e2517_d_n0: f64 = (1e-9 * eq71_e2516_d_n0);
        let eq71_e2517_d_n1: f64 = (1e-9 * eq71_e2516_d_n1);
        let eq71_e2517_d_n2: f64 = (1e-9 * eq71_e2516_d_n2);
        let eq71_e2517_d_n3: f64 = (1e-9 * eq71_e2516_d_n3);
        let eq71_e2517_d_n4: f64 = (1e-9 * eq71_e2516_d_n4);
        let eq71_e2517_d_n5: f64 = (1e-9 * eq71_e2516_d_n5);
        let eq71_e2517_d_n6: f64 = (1e-9 * eq71_e2516_d_n6);
        let eq71_e2517_d_n7: f64 = (1e-9 * eq71_e2516_d_n7);
        let eq71_e2517_d_n8: f64 = (1e-9 * eq71_e2516_d_n8);
        let eq71_e2517_d_n9: f64 = (1e-9 * eq71_e2516_d_n9);
        let eq71_e2517_d_n10: f64 = (1e-9 * eq71_e2516_d_n10);
        let eq71_e2517_d_n11: f64 = (1e-9 * eq71_e2516_d_n11);
        let eq71_e2517_d_n12: f64 = (1e-9 * eq71_e2516_d_n12);
        let eq71_e2517_d_n13: f64 = (1e-9 * eq71_e2516_d_n13);
        let eq71_e2517_d_n14: f64 = (1e-9 * eq71_e2516_d_n14);
        let eq71_e2517_d_n15: f64 = (1e-9 * eq71_e2516_d_n15);
        let eq71_e2517_d_n16: f64 = (1e-9 * eq71_e2516_d_n16);
        (eq71_e2517, eq71_e2517_d_n0, eq71_e2517_d_n1, eq71_e2517_d_n2, eq71_e2517_d_n3, eq71_e2517_d_n4, eq71_e2517_d_n5, eq71_e2517_d_n6, eq71_e2517_d_n7, eq71_e2517_d_n8, eq71_e2517_d_n9, eq71_e2517_d_n10, eq71_e2517_d_n11, eq71_e2517_d_n12, eq71_e2517_d_n13, eq71_e2517_d_n14, eq71_e2517_d_n15, eq71_e2517_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e2519;
        let eq71_node_derivatives: [f64; 17] = [eq71_e2519_d_n0, eq71_e2519_d_n1, eq71_e2519_d_n2, eq71_e2519_d_n3, eq71_e2519_d_n4, eq71_e2519_d_n5, eq71_e2519_d_n6, eq71_e2519_d_n7, eq71_e2519_d_n8, eq71_e2519_d_n9, eq71_e2519_d_n10, eq71_e2519_d_n11, eq71_e2519_d_n12, eq71_e2519_d_n13, eq71_e2519_d_n14, eq71_e2519_d_n15, eq71_e2519_d_n16];
        let eq71_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq71_value),
            &nodes,
            &eq71_node_derivatives,
            &branches,
            &eq71_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_72_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq72_e2524,) = {
    if (!(s.v[1723] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq72_value: f64 = eq72_e2524;
        stamper.stamp_potential(
            branches[8],
            eq72_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_73_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq73_e2530, eq73_e2530_d_n0, eq73_e2530_d_n1, eq73_e2530_d_n2, eq73_e2530_d_n3, eq73_e2530_d_n4, eq73_e2530_d_n5, eq73_e2530_d_n6, eq73_e2530_d_n7, eq73_e2530_d_n8, eq73_e2530_d_n9, eq73_e2530_d_n10, eq73_e2530_d_n11, eq73_e2530_d_n12, eq73_e2530_d_n13, eq73_e2530_d_n14, eq73_e2530_d_n15, eq73_e2530_d_n16,) = {
    if (s.v[1724] != 0.0) {
        let eq73_e2528: f64 = ((nv1 - nv10) * s.v[456]);
        let eq73_e2528_d_n0: f64 = ((nv1 - nv10) * s.dn[456][0]);
        let eq73_e2528_d_n1: f64 = (s.v[456] + ((nv1 - nv10) * s.dn[456][1]));
        let eq73_e2528_d_n2: f64 = ((nv1 - nv10) * s.dn[456][2]);
        let eq73_e2528_d_n3: f64 = ((nv1 - nv10) * s.dn[456][3]);
        let eq73_e2528_d_n4: f64 = ((nv1 - nv10) * s.dn[456][4]);
        let eq73_e2528_d_n5: f64 = ((nv1 - nv10) * s.dn[456][5]);
        let eq73_e2528_d_n6: f64 = ((nv1 - nv10) * s.dn[456][6]);
        let eq73_e2528_d_n7: f64 = ((nv1 - nv10) * s.dn[456][7]);
        let eq73_e2528_d_n8: f64 = ((nv1 - nv10) * s.dn[456][8]);
        let eq73_e2528_d_n9: f64 = ((nv1 - nv10) * s.dn[456][9]);
        let eq73_e2528_d_n10: f64 = ((-s.v[456]) + ((nv1 - nv10) * s.dn[456][10]));
        let eq73_e2528_d_n11: f64 = ((nv1 - nv10) * s.dn[456][11]);
        let eq73_e2528_d_n12: f64 = ((nv1 - nv10) * s.dn[456][12]);
        let eq73_e2528_d_n13: f64 = ((nv1 - nv10) * s.dn[456][13]);
        let eq73_e2528_d_n14: f64 = ((nv1 - nv10) * s.dn[456][14]);
        let eq73_e2528_d_n15: f64 = ((nv1 - nv10) * s.dn[456][15]);
        let eq73_e2528_d_n16: f64 = ((nv1 - nv10) * s.dn[456][16]);
        (eq73_e2528, eq73_e2528_d_n0, eq73_e2528_d_n1, eq73_e2528_d_n2, eq73_e2528_d_n3, eq73_e2528_d_n4, eq73_e2528_d_n5, eq73_e2528_d_n6, eq73_e2528_d_n7, eq73_e2528_d_n8, eq73_e2528_d_n9, eq73_e2528_d_n10, eq73_e2528_d_n11, eq73_e2528_d_n12, eq73_e2528_d_n13, eq73_e2528_d_n14, eq73_e2528_d_n15, eq73_e2528_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e2530;
        let eq73_node_derivatives: [f64; 17] = [eq73_e2530_d_n0, eq73_e2530_d_n1, eq73_e2530_d_n2, eq73_e2530_d_n3, eq73_e2530_d_n4, eq73_e2530_d_n5, eq73_e2530_d_n6, eq73_e2530_d_n7, eq73_e2530_d_n8, eq73_e2530_d_n9, eq73_e2530_d_n10, eq73_e2530_d_n11, eq73_e2530_d_n12, eq73_e2530_d_n13, eq73_e2530_d_n14, eq73_e2530_d_n15, eq73_e2530_d_n16];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            self.multiplicity * (eq73_value),
            &nodes,
            &eq73_node_derivatives,
            &branches,
            &eq73_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_74_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq74_e2538, eq74_e2538_d_n0, eq74_e2538_d_n1, eq74_e2538_d_n2, eq74_e2538_d_n3, eq74_e2538_d_n4, eq74_e2538_d_n5, eq74_e2538_d_n6, eq74_e2538_d_n7, eq74_e2538_d_n8, eq74_e2538_d_n9, eq74_e2538_d_n10, eq74_e2538_d_n11, eq74_e2538_d_n12, eq74_e2538_d_n13, eq74_e2538_d_n14, eq74_e2538_d_n15, eq74_e2538_d_n16,) = {
    if ((s.v[1724] != 0.0) && (s.v[1725] != 0.0)) {
        let eq74_e2536: f64 = ((nv10 - nv12) * s.v[458]);
        let eq74_e2536_d_n0: f64 = ((nv10 - nv12) * s.dn[458][0]);
        let eq74_e2536_d_n1: f64 = ((nv10 - nv12) * s.dn[458][1]);
        let eq74_e2536_d_n2: f64 = ((nv10 - nv12) * s.dn[458][2]);
        let eq74_e2536_d_n3: f64 = ((nv10 - nv12) * s.dn[458][3]);
        let eq74_e2536_d_n4: f64 = ((nv10 - nv12) * s.dn[458][4]);
        let eq74_e2536_d_n5: f64 = ((nv10 - nv12) * s.dn[458][5]);
        let eq74_e2536_d_n6: f64 = ((nv10 - nv12) * s.dn[458][6]);
        let eq74_e2536_d_n7: f64 = ((nv10 - nv12) * s.dn[458][7]);
        let eq74_e2536_d_n8: f64 = ((nv10 - nv12) * s.dn[458][8]);
        let eq74_e2536_d_n9: f64 = ((nv10 - nv12) * s.dn[458][9]);
        let eq74_e2536_d_n10: f64 = (s.v[458] + ((nv10 - nv12) * s.dn[458][10]));
        let eq74_e2536_d_n11: f64 = ((nv10 - nv12) * s.dn[458][11]);
        let eq74_e2536_d_n12: f64 = ((-s.v[458]) + ((nv10 - nv12) * s.dn[458][12]));
        let eq74_e2536_d_n13: f64 = ((nv10 - nv12) * s.dn[458][13]);
        let eq74_e2536_d_n14: f64 = ((nv10 - nv12) * s.dn[458][14]);
        let eq74_e2536_d_n15: f64 = ((nv10 - nv12) * s.dn[458][15]);
        let eq74_e2536_d_n16: f64 = ((nv10 - nv12) * s.dn[458][16]);
        (eq74_e2536, eq74_e2536_d_n0, eq74_e2536_d_n1, eq74_e2536_d_n2, eq74_e2536_d_n3, eq74_e2536_d_n4, eq74_e2536_d_n5, eq74_e2536_d_n6, eq74_e2536_d_n7, eq74_e2536_d_n8, eq74_e2536_d_n9, eq74_e2536_d_n10, eq74_e2536_d_n11, eq74_e2536_d_n12, eq74_e2536_d_n13, eq74_e2536_d_n14, eq74_e2536_d_n15, eq74_e2536_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e2538;
        let eq74_node_derivatives: [f64; 17] = [eq74_e2538_d_n0, eq74_e2538_d_n1, eq74_e2538_d_n2, eq74_e2538_d_n3, eq74_e2538_d_n4, eq74_e2538_d_n5, eq74_e2538_d_n6, eq74_e2538_d_n7, eq74_e2538_d_n8, eq74_e2538_d_n9, eq74_e2538_d_n10, eq74_e2538_d_n11, eq74_e2538_d_n12, eq74_e2538_d_n13, eq74_e2538_d_n14, eq74_e2538_d_n15, eq74_e2538_d_n16];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[12]),
            self.multiplicity * (eq74_value),
            &nodes,
            &eq74_node_derivatives,
            &branches,
            &eq74_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_75_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq75_e2546, eq75_e2546_d_n0, eq75_e2546_d_n1, eq75_e2546_d_n2, eq75_e2546_d_n3, eq75_e2546_d_n4, eq75_e2546_d_n5, eq75_e2546_d_n6, eq75_e2546_d_n7, eq75_e2546_d_n8, eq75_e2546_d_n9, eq75_e2546_d_n10, eq75_e2546_d_n11, eq75_e2546_d_n12, eq75_e2546_d_n13, eq75_e2546_d_n14, eq75_e2546_d_n15, eq75_e2546_d_n16,) = {
    if ((s.v[1724] != 0.0) && (s.v[1725] != 0.0)) {
        let eq75_e2544: f64 = ((nv10 - nv13) * s.v[459]);
        let eq75_e2544_d_n0: f64 = ((nv10 - nv13) * s.dn[459][0]);
        let eq75_e2544_d_n1: f64 = ((nv10 - nv13) * s.dn[459][1]);
        let eq75_e2544_d_n2: f64 = ((nv10 - nv13) * s.dn[459][2]);
        let eq75_e2544_d_n3: f64 = ((nv10 - nv13) * s.dn[459][3]);
        let eq75_e2544_d_n4: f64 = ((nv10 - nv13) * s.dn[459][4]);
        let eq75_e2544_d_n5: f64 = ((nv10 - nv13) * s.dn[459][5]);
        let eq75_e2544_d_n6: f64 = ((nv10 - nv13) * s.dn[459][6]);
        let eq75_e2544_d_n7: f64 = ((nv10 - nv13) * s.dn[459][7]);
        let eq75_e2544_d_n8: f64 = ((nv10 - nv13) * s.dn[459][8]);
        let eq75_e2544_d_n9: f64 = ((nv10 - nv13) * s.dn[459][9]);
        let eq75_e2544_d_n10: f64 = (s.v[459] + ((nv10 - nv13) * s.dn[459][10]));
        let eq75_e2544_d_n11: f64 = ((nv10 - nv13) * s.dn[459][11]);
        let eq75_e2544_d_n12: f64 = ((nv10 - nv13) * s.dn[459][12]);
        let eq75_e2544_d_n13: f64 = ((-s.v[459]) + ((nv10 - nv13) * s.dn[459][13]));
        let eq75_e2544_d_n14: f64 = ((nv10 - nv13) * s.dn[459][14]);
        let eq75_e2544_d_n15: f64 = ((nv10 - nv13) * s.dn[459][15]);
        let eq75_e2544_d_n16: f64 = ((nv10 - nv13) * s.dn[459][16]);
        (eq75_e2544, eq75_e2544_d_n0, eq75_e2544_d_n1, eq75_e2544_d_n2, eq75_e2544_d_n3, eq75_e2544_d_n4, eq75_e2544_d_n5, eq75_e2544_d_n6, eq75_e2544_d_n7, eq75_e2544_d_n8, eq75_e2544_d_n9, eq75_e2544_d_n10, eq75_e2544_d_n11, eq75_e2544_d_n12, eq75_e2544_d_n13, eq75_e2544_d_n14, eq75_e2544_d_n15, eq75_e2544_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e2546;
        let eq75_node_derivatives: [f64; 17] = [eq75_e2546_d_n0, eq75_e2546_d_n1, eq75_e2546_d_n2, eq75_e2546_d_n3, eq75_e2546_d_n4, eq75_e2546_d_n5, eq75_e2546_d_n6, eq75_e2546_d_n7, eq75_e2546_d_n8, eq75_e2546_d_n9, eq75_e2546_d_n10, eq75_e2546_d_n11, eq75_e2546_d_n12, eq75_e2546_d_n13, eq75_e2546_d_n14, eq75_e2546_d_n15, eq75_e2546_d_n16];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            self.multiplicity * (eq75_value),
            &nodes,
            &eq75_node_derivatives,
            &branches,
            &eq75_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_76_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq76_e2554, eq76_e2554_d_n0, eq76_e2554_d_n1, eq76_e2554_d_n2, eq76_e2554_d_n3, eq76_e2554_d_n4, eq76_e2554_d_n5, eq76_e2554_d_n6, eq76_e2554_d_n7, eq76_e2554_d_n8, eq76_e2554_d_n9, eq76_e2554_d_n10, eq76_e2554_d_n11, eq76_e2554_d_n12, eq76_e2554_d_n13, eq76_e2554_d_n14, eq76_e2554_d_n15, eq76_e2554_d_n16,) = {
    if ((s.v[1724] != 0.0) && (s.v[1725] != 0.0)) {
        let eq76_e2552: f64 = ((nv10 - nv14) * s.v[459]);
        let eq76_e2552_d_n0: f64 = ((nv10 - nv14) * s.dn[459][0]);
        let eq76_e2552_d_n1: f64 = ((nv10 - nv14) * s.dn[459][1]);
        let eq76_e2552_d_n2: f64 = ((nv10 - nv14) * s.dn[459][2]);
        let eq76_e2552_d_n3: f64 = ((nv10 - nv14) * s.dn[459][3]);
        let eq76_e2552_d_n4: f64 = ((nv10 - nv14) * s.dn[459][4]);
        let eq76_e2552_d_n5: f64 = ((nv10 - nv14) * s.dn[459][5]);
        let eq76_e2552_d_n6: f64 = ((nv10 - nv14) * s.dn[459][6]);
        let eq76_e2552_d_n7: f64 = ((nv10 - nv14) * s.dn[459][7]);
        let eq76_e2552_d_n8: f64 = ((nv10 - nv14) * s.dn[459][8]);
        let eq76_e2552_d_n9: f64 = ((nv10 - nv14) * s.dn[459][9]);
        let eq76_e2552_d_n10: f64 = (s.v[459] + ((nv10 - nv14) * s.dn[459][10]));
        let eq76_e2552_d_n11: f64 = ((nv10 - nv14) * s.dn[459][11]);
        let eq76_e2552_d_n12: f64 = ((nv10 - nv14) * s.dn[459][12]);
        let eq76_e2552_d_n13: f64 = ((nv10 - nv14) * s.dn[459][13]);
        let eq76_e2552_d_n14: f64 = ((-s.v[459]) + ((nv10 - nv14) * s.dn[459][14]));
        let eq76_e2552_d_n15: f64 = ((nv10 - nv14) * s.dn[459][15]);
        let eq76_e2552_d_n16: f64 = ((nv10 - nv14) * s.dn[459][16]);
        (eq76_e2552, eq76_e2552_d_n0, eq76_e2552_d_n1, eq76_e2552_d_n2, eq76_e2552_d_n3, eq76_e2552_d_n4, eq76_e2552_d_n5, eq76_e2552_d_n6, eq76_e2552_d_n7, eq76_e2552_d_n8, eq76_e2552_d_n9, eq76_e2552_d_n10, eq76_e2552_d_n11, eq76_e2552_d_n12, eq76_e2552_d_n13, eq76_e2552_d_n14, eq76_e2552_d_n15, eq76_e2552_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e2554;
        let eq76_node_derivatives: [f64; 17] = [eq76_e2554_d_n0, eq76_e2554_d_n1, eq76_e2554_d_n2, eq76_e2554_d_n3, eq76_e2554_d_n4, eq76_e2554_d_n5, eq76_e2554_d_n6, eq76_e2554_d_n7, eq76_e2554_d_n8, eq76_e2554_d_n9, eq76_e2554_d_n10, eq76_e2554_d_n11, eq76_e2554_d_n12, eq76_e2554_d_n13, eq76_e2554_d_n14, eq76_e2554_d_n15, eq76_e2554_d_n16];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[14]),
            self.multiplicity * (eq76_value),
            &nodes,
            &eq76_node_derivatives,
            &branches,
            &eq76_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_77_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq77_e2561,) = {
    if ((s.v[1724] != 0.0) && (!(s.v[1725] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq77_value: f64 = eq77_e2561;
        stamper.stamp_potential(
            branches[9],
            eq77_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_78_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq78_e2568,) = {
    if ((s.v[1724] != 0.0) && (!(s.v[1725] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq78_value: f64 = eq78_e2568;
        stamper.stamp_potential(
            branches[10],
            eq78_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_79_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq79_e2575,) = {
    if ((s.v[1724] != 0.0) && (!(s.v[1725] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq79_value: f64 = eq79_e2575;
        stamper.stamp_potential(
            branches[11],
            eq79_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_80_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq80_e2580,) = {
    if (!(s.v[1724] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq80_value: f64 = eq80_e2580;
        stamper.stamp_potential(
            branches[12],
            eq80_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_81_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq81_e2585,) = {
    if (!(s.v[1724] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq81_value: f64 = eq81_e2585;
        stamper.stamp_potential(
            branches[13],
            eq81_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_82_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq82_e2590,) = {
    if (!(s.v[1724] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq82_value: f64 = eq82_e2590;
        stamper.stamp_potential(
            branches[14],
            eq82_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_83_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq83_e2595,) = {
    if (!(s.v[1724] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq83_value: f64 = eq83_e2595;
        stamper.stamp_potential(
            branches[15],
            eq83_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_84_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq84_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq84_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_85_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq85_e2613,) = {
    if (s.v[1726] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq85_value: f64 = eq85_e2613;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[9]),
            self.multiplicity * (eq85_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_86_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq86_e2627,) = {
    if ((s.v[1726] != 0.0) && (s.v[1727] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq86_value: f64 = eq86_e2627;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq86_value),
            &[
            ],
        );
    }
}

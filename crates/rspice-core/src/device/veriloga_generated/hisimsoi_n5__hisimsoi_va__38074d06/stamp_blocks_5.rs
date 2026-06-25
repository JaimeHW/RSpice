#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq17_e394: f64 = (s.v[614] * (nv14 - 0.0));
        let eq17_e394_d_n0: f64 = (s.dn[614][0] * (nv14 - 0.0));
        let eq17_e394_d_n1: f64 = (s.dn[614][1] * (nv14 - 0.0));
        let eq17_e394_d_n2: f64 = (s.dn[614][2] * (nv14 - 0.0));
        let eq17_e394_d_n3: f64 = (s.dn[614][3] * (nv14 - 0.0));
        let eq17_e394_d_n4: f64 = (s.dn[614][4] * (nv14 - 0.0));
        let eq17_e394_d_n5: f64 = (s.dn[614][5] * (nv14 - 0.0));
        let eq17_e394_d_n6: f64 = (s.dn[614][6] * (nv14 - 0.0));
        let eq17_e394_d_n7: f64 = (s.dn[614][7] * (nv14 - 0.0));
        let eq17_e394_d_n8: f64 = (s.dn[614][8] * (nv14 - 0.0));
        let eq17_e394_d_n9: f64 = (s.dn[614][9] * (nv14 - 0.0));
        let eq17_e394_d_n10: f64 = (s.dn[614][10] * (nv14 - 0.0));
        let eq17_e394_d_n11: f64 = (s.dn[614][11] * (nv14 - 0.0));
        let eq17_e394_d_n12: f64 = (s.dn[614][12] * (nv14 - 0.0));
        let eq17_e394_d_n13: f64 = (s.dn[614][13] * (nv14 - 0.0));
        let eq17_e394_d_n14: f64 = ((s.dn[614][14] * (nv14 - 0.0)) + s.v[614]);
        let eq17_e394_d_n15: f64 = (s.dn[614][15] * (nv14 - 0.0));
        let eq17_e394_d_n16: f64 = (s.dn[614][16] * (nv14 - 0.0));
        let eq17_e394_d_n17: f64 = (s.dn[614][17] * (nv14 - 0.0));
        let eq17_e394_d_n18: f64 = (s.dn[614][18] * (nv14 - 0.0));
        let eq17_e394_d_b0: f64 = (s.db[614][0] * (nv14 - 0.0));
        let eq17_e394_d_b1: f64 = (s.db[614][1] * (nv14 - 0.0));
        let eq17_e394_d_b2: f64 = (s.db[614][2] * (nv14 - 0.0));
        let eq17_e394_d_b3: f64 = (s.db[614][3] * (nv14 - 0.0));
        let eq17_e394_d_b4: f64 = (s.db[614][4] * (nv14 - 0.0));
        let eq17_e394_d_b5: f64 = (s.db[614][5] * (nv14 - 0.0));
        let eq17_e394_d_b6: f64 = (s.db[614][6] * (nv14 - 0.0));
        let eq17_e394_d_b7: f64 = (s.db[614][7] * (nv14 - 0.0));
        let eq17_e394_d_b8: f64 = (s.db[614][8] * (nv14 - 0.0));
        let eq17_e394_d_b9: f64 = (s.db[614][9] * (nv14 - 0.0));
        let eq17_e394_d_b10: f64 = (s.db[614][10] * (nv14 - 0.0));
        let eq17_e394_d_b11: f64 = (s.db[614][11] * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e394;
        let eq17_node_derivatives: [f64; 19] = [eq17_e394_d_n0, eq17_e394_d_n1, eq17_e394_d_n2, eq17_e394_d_n3, eq17_e394_d_n4, eq17_e394_d_n5, eq17_e394_d_n6, eq17_e394_d_n7, eq17_e394_d_n8, eq17_e394_d_n9, eq17_e394_d_n10, eq17_e394_d_n11, eq17_e394_d_n12, eq17_e394_d_n13, eq17_e394_d_n14, eq17_e394_d_n15, eq17_e394_d_n16, eq17_e394_d_n17, eq17_e394_d_n18];
        let eq17_branch_derivatives: [f64; 12] = [eq17_e394_d_b0, eq17_e394_d_b1, eq17_e394_d_b2, eq17_e394_d_b3, eq17_e394_d_b4, eq17_e394_d_b5, eq17_e394_d_b6, eq17_e394_d_b7, eq17_e394_d_b8, eq17_e394_d_b9, eq17_e394_d_b10, eq17_e394_d_b11];
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq18_e397: f64 = ((nv14 - 0.0) * s.v[617]);
        let eq18_e397_d_n0: f64 = ((nv14 - 0.0) * s.dn[617][0]);
        let eq18_e397_d_n1: f64 = ((nv14 - 0.0) * s.dn[617][1]);
        let eq18_e397_d_n2: f64 = ((nv14 - 0.0) * s.dn[617][2]);
        let eq18_e397_d_n3: f64 = ((nv14 - 0.0) * s.dn[617][3]);
        let eq18_e397_d_n4: f64 = ((nv14 - 0.0) * s.dn[617][4]);
        let eq18_e397_d_n5: f64 = ((nv14 - 0.0) * s.dn[617][5]);
        let eq18_e397_d_n6: f64 = ((nv14 - 0.0) * s.dn[617][6]);
        let eq18_e397_d_n7: f64 = ((nv14 - 0.0) * s.dn[617][7]);
        let eq18_e397_d_n8: f64 = ((nv14 - 0.0) * s.dn[617][8]);
        let eq18_e397_d_n9: f64 = ((nv14 - 0.0) * s.dn[617][9]);
        let eq18_e397_d_n10: f64 = ((nv14 - 0.0) * s.dn[617][10]);
        let eq18_e397_d_n11: f64 = ((nv14 - 0.0) * s.dn[617][11]);
        let eq18_e397_d_n12: f64 = ((nv14 - 0.0) * s.dn[617][12]);
        let eq18_e397_d_n13: f64 = ((nv14 - 0.0) * s.dn[617][13]);
        let eq18_e397_d_n14: f64 = (s.v[617] + ((nv14 - 0.0) * s.dn[617][14]));
        let eq18_e397_d_n15: f64 = ((nv14 - 0.0) * s.dn[617][15]);
        let eq18_e397_d_n16: f64 = ((nv14 - 0.0) * s.dn[617][16]);
        let eq18_e397_d_n17: f64 = ((nv14 - 0.0) * s.dn[617][17]);
        let eq18_e397_d_n18: f64 = ((nv14 - 0.0) * s.dn[617][18]);
        let eq18_e397_d_b0: f64 = ((nv14 - 0.0) * s.db[617][0]);
        let eq18_e397_d_b1: f64 = ((nv14 - 0.0) * s.db[617][1]);
        let eq18_e397_d_b2: f64 = ((nv14 - 0.0) * s.db[617][2]);
        let eq18_e397_d_b3: f64 = ((nv14 - 0.0) * s.db[617][3]);
        let eq18_e397_d_b4: f64 = ((nv14 - 0.0) * s.db[617][4]);
        let eq18_e397_d_b5: f64 = ((nv14 - 0.0) * s.db[617][5]);
        let eq18_e397_d_b6: f64 = ((nv14 - 0.0) * s.db[617][6]);
        let eq18_e397_d_b7: f64 = ((nv14 - 0.0) * s.db[617][7]);
        let eq18_e397_d_b8: f64 = ((nv14 - 0.0) * s.db[617][8]);
        let eq18_e397_d_b9: f64 = ((nv14 - 0.0) * s.db[617][9]);
        let eq18_e397_d_b10: f64 = ((nv14 - 0.0) * s.db[617][10]);
        let eq18_e397_d_b11: f64 = ((nv14 - 0.0) * s.db[617][11]);
        let eq18_e398: f64 = self.eval_ddt(3, eq18_e397);
        let eq18_e398_d_n0: f64 = self.ddt_jacobian(eq18_e397_d_n0);
        let eq18_e398_d_n1: f64 = self.ddt_jacobian(eq18_e397_d_n1);
        let eq18_e398_d_n2: f64 = self.ddt_jacobian(eq18_e397_d_n2);
        let eq18_e398_d_n3: f64 = self.ddt_jacobian(eq18_e397_d_n3);
        let eq18_e398_d_n4: f64 = self.ddt_jacobian(eq18_e397_d_n4);
        let eq18_e398_d_n5: f64 = self.ddt_jacobian(eq18_e397_d_n5);
        let eq18_e398_d_n6: f64 = self.ddt_jacobian(eq18_e397_d_n6);
        let eq18_e398_d_n7: f64 = self.ddt_jacobian(eq18_e397_d_n7);
        let eq18_e398_d_n8: f64 = self.ddt_jacobian(eq18_e397_d_n8);
        let eq18_e398_d_n9: f64 = self.ddt_jacobian(eq18_e397_d_n9);
        let eq18_e398_d_n10: f64 = self.ddt_jacobian(eq18_e397_d_n10);
        let eq18_e398_d_n11: f64 = self.ddt_jacobian(eq18_e397_d_n11);
        let eq18_e398_d_n12: f64 = self.ddt_jacobian(eq18_e397_d_n12);
        let eq18_e398_d_n13: f64 = self.ddt_jacobian(eq18_e397_d_n13);
        let eq18_e398_d_n14: f64 = self.ddt_jacobian(eq18_e397_d_n14);
        let eq18_e398_d_n15: f64 = self.ddt_jacobian(eq18_e397_d_n15);
        let eq18_e398_d_n16: f64 = self.ddt_jacobian(eq18_e397_d_n16);
        let eq18_e398_d_n17: f64 = self.ddt_jacobian(eq18_e397_d_n17);
        let eq18_e398_d_n18: f64 = self.ddt_jacobian(eq18_e397_d_n18);
        let eq18_e398_d_b0: f64 = self.ddt_jacobian(eq18_e397_d_b0);
        let eq18_e398_d_b1: f64 = self.ddt_jacobian(eq18_e397_d_b1);
        let eq18_e398_d_b2: f64 = self.ddt_jacobian(eq18_e397_d_b2);
        let eq18_e398_d_b3: f64 = self.ddt_jacobian(eq18_e397_d_b3);
        let eq18_e398_d_b4: f64 = self.ddt_jacobian(eq18_e397_d_b4);
        let eq18_e398_d_b5: f64 = self.ddt_jacobian(eq18_e397_d_b5);
        let eq18_e398_d_b6: f64 = self.ddt_jacobian(eq18_e397_d_b6);
        let eq18_e398_d_b7: f64 = self.ddt_jacobian(eq18_e397_d_b7);
        let eq18_e398_d_b8: f64 = self.ddt_jacobian(eq18_e397_d_b8);
        let eq18_e398_d_b9: f64 = self.ddt_jacobian(eq18_e397_d_b9);
        let eq18_e398_d_b10: f64 = self.ddt_jacobian(eq18_e397_d_b10);
        let eq18_e398_d_b11: f64 = self.ddt_jacobian(eq18_e397_d_b11);
        let eq18_value: f64 = eq18_e398;
        let eq18_node_derivatives: [f64; 19] = [eq18_e398_d_n0, eq18_e398_d_n1, eq18_e398_d_n2, eq18_e398_d_n3, eq18_e398_d_n4, eq18_e398_d_n5, eq18_e398_d_n6, eq18_e398_d_n7, eq18_e398_d_n8, eq18_e398_d_n9, eq18_e398_d_n10, eq18_e398_d_n11, eq18_e398_d_n12, eq18_e398_d_n13, eq18_e398_d_n14, eq18_e398_d_n15, eq18_e398_d_n16, eq18_e398_d_n17, eq18_e398_d_n18];
        let eq18_branch_derivatives: [f64; 12] = [eq18_e398_d_b0, eq18_e398_d_b1, eq18_e398_d_b2, eq18_e398_d_b3, eq18_e398_d_b4, eq18_e398_d_b5, eq18_e398_d_b6, eq18_e398_d_b7, eq18_e398_d_b8, eq18_e398_d_b9, eq18_e398_d_b10, eq18_e398_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[11]),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq19_e401: f64 = ((nv14 - 0.0) * s.v[618]);
        let eq19_e401_d_n0: f64 = ((nv14 - 0.0) * s.dn[618][0]);
        let eq19_e401_d_n1: f64 = ((nv14 - 0.0) * s.dn[618][1]);
        let eq19_e401_d_n2: f64 = ((nv14 - 0.0) * s.dn[618][2]);
        let eq19_e401_d_n3: f64 = ((nv14 - 0.0) * s.dn[618][3]);
        let eq19_e401_d_n4: f64 = ((nv14 - 0.0) * s.dn[618][4]);
        let eq19_e401_d_n5: f64 = ((nv14 - 0.0) * s.dn[618][5]);
        let eq19_e401_d_n6: f64 = ((nv14 - 0.0) * s.dn[618][6]);
        let eq19_e401_d_n7: f64 = ((nv14 - 0.0) * s.dn[618][7]);
        let eq19_e401_d_n8: f64 = ((nv14 - 0.0) * s.dn[618][8]);
        let eq19_e401_d_n9: f64 = ((nv14 - 0.0) * s.dn[618][9]);
        let eq19_e401_d_n10: f64 = ((nv14 - 0.0) * s.dn[618][10]);
        let eq19_e401_d_n11: f64 = ((nv14 - 0.0) * s.dn[618][11]);
        let eq19_e401_d_n12: f64 = ((nv14 - 0.0) * s.dn[618][12]);
        let eq19_e401_d_n13: f64 = ((nv14 - 0.0) * s.dn[618][13]);
        let eq19_e401_d_n14: f64 = (s.v[618] + ((nv14 - 0.0) * s.dn[618][14]));
        let eq19_e401_d_n15: f64 = ((nv14 - 0.0) * s.dn[618][15]);
        let eq19_e401_d_n16: f64 = ((nv14 - 0.0) * s.dn[618][16]);
        let eq19_e401_d_n17: f64 = ((nv14 - 0.0) * s.dn[618][17]);
        let eq19_e401_d_n18: f64 = ((nv14 - 0.0) * s.dn[618][18]);
        let eq19_e401_d_b0: f64 = ((nv14 - 0.0) * s.db[618][0]);
        let eq19_e401_d_b1: f64 = ((nv14 - 0.0) * s.db[618][1]);
        let eq19_e401_d_b2: f64 = ((nv14 - 0.0) * s.db[618][2]);
        let eq19_e401_d_b3: f64 = ((nv14 - 0.0) * s.db[618][3]);
        let eq19_e401_d_b4: f64 = ((nv14 - 0.0) * s.db[618][4]);
        let eq19_e401_d_b5: f64 = ((nv14 - 0.0) * s.db[618][5]);
        let eq19_e401_d_b6: f64 = ((nv14 - 0.0) * s.db[618][6]);
        let eq19_e401_d_b7: f64 = ((nv14 - 0.0) * s.db[618][7]);
        let eq19_e401_d_b8: f64 = ((nv14 - 0.0) * s.db[618][8]);
        let eq19_e401_d_b9: f64 = ((nv14 - 0.0) * s.db[618][9]);
        let eq19_e401_d_b10: f64 = ((nv14 - 0.0) * s.db[618][10]);
        let eq19_e401_d_b11: f64 = ((nv14 - 0.0) * s.db[618][11]);
        let eq19_e402: f64 = self.eval_ddt(4, eq19_e401);
        let eq19_e402_d_n0: f64 = self.ddt_jacobian(eq19_e401_d_n0);
        let eq19_e402_d_n1: f64 = self.ddt_jacobian(eq19_e401_d_n1);
        let eq19_e402_d_n2: f64 = self.ddt_jacobian(eq19_e401_d_n2);
        let eq19_e402_d_n3: f64 = self.ddt_jacobian(eq19_e401_d_n3);
        let eq19_e402_d_n4: f64 = self.ddt_jacobian(eq19_e401_d_n4);
        let eq19_e402_d_n5: f64 = self.ddt_jacobian(eq19_e401_d_n5);
        let eq19_e402_d_n6: f64 = self.ddt_jacobian(eq19_e401_d_n6);
        let eq19_e402_d_n7: f64 = self.ddt_jacobian(eq19_e401_d_n7);
        let eq19_e402_d_n8: f64 = self.ddt_jacobian(eq19_e401_d_n8);
        let eq19_e402_d_n9: f64 = self.ddt_jacobian(eq19_e401_d_n9);
        let eq19_e402_d_n10: f64 = self.ddt_jacobian(eq19_e401_d_n10);
        let eq19_e402_d_n11: f64 = self.ddt_jacobian(eq19_e401_d_n11);
        let eq19_e402_d_n12: f64 = self.ddt_jacobian(eq19_e401_d_n12);
        let eq19_e402_d_n13: f64 = self.ddt_jacobian(eq19_e401_d_n13);
        let eq19_e402_d_n14: f64 = self.ddt_jacobian(eq19_e401_d_n14);
        let eq19_e402_d_n15: f64 = self.ddt_jacobian(eq19_e401_d_n15);
        let eq19_e402_d_n16: f64 = self.ddt_jacobian(eq19_e401_d_n16);
        let eq19_e402_d_n17: f64 = self.ddt_jacobian(eq19_e401_d_n17);
        let eq19_e402_d_n18: f64 = self.ddt_jacobian(eq19_e401_d_n18);
        let eq19_e402_d_b0: f64 = self.ddt_jacobian(eq19_e401_d_b0);
        let eq19_e402_d_b1: f64 = self.ddt_jacobian(eq19_e401_d_b1);
        let eq19_e402_d_b2: f64 = self.ddt_jacobian(eq19_e401_d_b2);
        let eq19_e402_d_b3: f64 = self.ddt_jacobian(eq19_e401_d_b3);
        let eq19_e402_d_b4: f64 = self.ddt_jacobian(eq19_e401_d_b4);
        let eq19_e402_d_b5: f64 = self.ddt_jacobian(eq19_e401_d_b5);
        let eq19_e402_d_b6: f64 = self.ddt_jacobian(eq19_e401_d_b6);
        let eq19_e402_d_b7: f64 = self.ddt_jacobian(eq19_e401_d_b7);
        let eq19_e402_d_b8: f64 = self.ddt_jacobian(eq19_e401_d_b8);
        let eq19_e402_d_b9: f64 = self.ddt_jacobian(eq19_e401_d_b9);
        let eq19_e402_d_b10: f64 = self.ddt_jacobian(eq19_e401_d_b10);
        let eq19_e402_d_b11: f64 = self.ddt_jacobian(eq19_e401_d_b11);
        let eq19_value: f64 = eq19_e402;
        let eq19_node_derivatives: [f64; 19] = [eq19_e402_d_n0, eq19_e402_d_n1, eq19_e402_d_n2, eq19_e402_d_n3, eq19_e402_d_n4, eq19_e402_d_n5, eq19_e402_d_n6, eq19_e402_d_n7, eq19_e402_d_n8, eq19_e402_d_n9, eq19_e402_d_n10, eq19_e402_d_n11, eq19_e402_d_n12, eq19_e402_d_n13, eq19_e402_d_n14, eq19_e402_d_n15, eq19_e402_d_n16, eq19_e402_d_n17, eq19_e402_d_n18];
        let eq19_branch_derivatives: [f64; 12] = [eq19_e402_d_b0, eq19_e402_d_b1, eq19_e402_d_b2, eq19_e402_d_b3, eq19_e402_d_b4, eq19_e402_d_b5, eq19_e402_d_b6, eq19_e402_d_b7, eq19_e402_d_b8, eq19_e402_d_b9, eq19_e402_d_b10, eq19_e402_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[6]),
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
        let (eq20_e410,) = {
    if (p.p259 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e410;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq20_value),
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
        let (eq21_e418,) = {
    if (p.p260 != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e418;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[6]),
            self.multiplicity * (eq21_value),
            &[
            ],
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
        let (eq22_e428,) = {
    if (s.v[1847] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e428;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[6]),
            self.multiplicity * (eq22_value),
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
        let (eq23_e438,) = {
    if (s.v[1847] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e438;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[7]),
            self.multiplicity * (eq23_value),
            &[
            ],
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
        let (eq24_e448,) = {
    if (s.v[1847] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq24_value: f64 = eq24_e448;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[12]),
            self.multiplicity * (eq24_value),
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq25_e454, eq25_e454_d_n0, eq25_e454_d_n1, eq25_e454_d_n2, eq25_e454_d_n3, eq25_e454_d_n4, eq25_e454_d_n5, eq25_e454_d_n6, eq25_e454_d_n7, eq25_e454_d_n8, eq25_e454_d_n9, eq25_e454_d_n10, eq25_e454_d_n11, eq25_e454_d_n12, eq25_e454_d_n13, eq25_e454_d_n14, eq25_e454_d_n15, eq25_e454_d_n16, eq25_e454_d_n17, eq25_e454_d_n18, eq25_e454_d_b0, eq25_e454_d_b1, eq25_e454_d_b2, eq25_e454_d_b3, eq25_e454_d_b4, eq25_e454_d_b5, eq25_e454_d_b6, eq25_e454_d_b7, eq25_e454_d_b8, eq25_e454_d_b9, eq25_e454_d_b10, eq25_e454_d_b11,) = {
    if (p.p35 != 0.0) {
        let eq25_e452: f64 = (s.v[551] * (nv1 - nv11));
        let eq25_e452_d_n0: f64 = (s.dn[551][0] * (nv1 - nv11));
        let eq25_e452_d_n1: f64 = ((s.dn[551][1] * (nv1 - nv11)) + s.v[551]);
        let eq25_e452_d_n2: f64 = (s.dn[551][2] * (nv1 - nv11));
        let eq25_e452_d_n3: f64 = (s.dn[551][3] * (nv1 - nv11));
        let eq25_e452_d_n4: f64 = (s.dn[551][4] * (nv1 - nv11));
        let eq25_e452_d_n5: f64 = (s.dn[551][5] * (nv1 - nv11));
        let eq25_e452_d_n6: f64 = (s.dn[551][6] * (nv1 - nv11));
        let eq25_e452_d_n7: f64 = (s.dn[551][7] * (nv1 - nv11));
        let eq25_e452_d_n8: f64 = (s.dn[551][8] * (nv1 - nv11));
        let eq25_e452_d_n9: f64 = (s.dn[551][9] * (nv1 - nv11));
        let eq25_e452_d_n10: f64 = (s.dn[551][10] * (nv1 - nv11));
        let eq25_e452_d_n11: f64 = ((s.dn[551][11] * (nv1 - nv11)) + (-s.v[551]));
        let eq25_e452_d_n12: f64 = (s.dn[551][12] * (nv1 - nv11));
        let eq25_e452_d_n13: f64 = (s.dn[551][13] * (nv1 - nv11));
        let eq25_e452_d_n14: f64 = (s.dn[551][14] * (nv1 - nv11));
        let eq25_e452_d_n15: f64 = (s.dn[551][15] * (nv1 - nv11));
        let eq25_e452_d_n16: f64 = (s.dn[551][16] * (nv1 - nv11));
        let eq25_e452_d_n17: f64 = (s.dn[551][17] * (nv1 - nv11));
        let eq25_e452_d_n18: f64 = (s.dn[551][18] * (nv1 - nv11));
        let eq25_e452_d_b0: f64 = (s.db[551][0] * (nv1 - nv11));
        let eq25_e452_d_b1: f64 = (s.db[551][1] * (nv1 - nv11));
        let eq25_e452_d_b2: f64 = (s.db[551][2] * (nv1 - nv11));
        let eq25_e452_d_b3: f64 = (s.db[551][3] * (nv1 - nv11));
        let eq25_e452_d_b4: f64 = (s.db[551][4] * (nv1 - nv11));
        let eq25_e452_d_b5: f64 = (s.db[551][5] * (nv1 - nv11));
        let eq25_e452_d_b6: f64 = (s.db[551][6] * (nv1 - nv11));
        let eq25_e452_d_b7: f64 = (s.db[551][7] * (nv1 - nv11));
        let eq25_e452_d_b8: f64 = (s.db[551][8] * (nv1 - nv11));
        let eq25_e452_d_b9: f64 = (s.db[551][9] * (nv1 - nv11));
        let eq25_e452_d_b10: f64 = (s.db[551][10] * (nv1 - nv11));
        let eq25_e452_d_b11: f64 = (s.db[551][11] * (nv1 - nv11));
        (eq25_e452, eq25_e452_d_n0, eq25_e452_d_n1, eq25_e452_d_n2, eq25_e452_d_n3, eq25_e452_d_n4, eq25_e452_d_n5, eq25_e452_d_n6, eq25_e452_d_n7, eq25_e452_d_n8, eq25_e452_d_n9, eq25_e452_d_n10, eq25_e452_d_n11, eq25_e452_d_n12, eq25_e452_d_n13, eq25_e452_d_n14, eq25_e452_d_n15, eq25_e452_d_n16, eq25_e452_d_n17, eq25_e452_d_n18, eq25_e452_d_b0, eq25_e452_d_b1, eq25_e452_d_b2, eq25_e452_d_b3, eq25_e452_d_b4, eq25_e452_d_b5, eq25_e452_d_b6, eq25_e452_d_b7, eq25_e452_d_b8, eq25_e452_d_b9, eq25_e452_d_b10, eq25_e452_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e454;
        let eq25_node_derivatives: [f64; 19] = [eq25_e454_d_n0, eq25_e454_d_n1, eq25_e454_d_n2, eq25_e454_d_n3, eq25_e454_d_n4, eq25_e454_d_n5, eq25_e454_d_n6, eq25_e454_d_n7, eq25_e454_d_n8, eq25_e454_d_n9, eq25_e454_d_n10, eq25_e454_d_n11, eq25_e454_d_n12, eq25_e454_d_n13, eq25_e454_d_n14, eq25_e454_d_n15, eq25_e454_d_n16, eq25_e454_d_n17, eq25_e454_d_n18];
        let eq25_branch_derivatives: [f64; 12] = [eq25_e454_d_b0, eq25_e454_d_b1, eq25_e454_d_b2, eq25_e454_d_b3, eq25_e454_d_b4, eq25_e454_d_b5, eq25_e454_d_b6, eq25_e454_d_b7, eq25_e454_d_b8, eq25_e454_d_b9, eq25_e454_d_b10, eq25_e454_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[11]),
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
        let (eq26_e459,) = {
    if (!(p.p35 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e459;
        stamper.stamp_potential(
            branches[4],
            eq26_value,
            &[
            ],
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq27_e465, eq27_e465_d_n0, eq27_e465_d_n1, eq27_e465_d_n2, eq27_e465_d_n3, eq27_e465_d_n4, eq27_e465_d_n5, eq27_e465_d_n6, eq27_e465_d_n7, eq27_e465_d_n8, eq27_e465_d_n9, eq27_e465_d_n10, eq27_e465_d_n11, eq27_e465_d_n12, eq27_e465_d_n13, eq27_e465_d_n14, eq27_e465_d_n15, eq27_e465_d_n16, eq27_e465_d_n17, eq27_e465_d_n18, eq27_e465_d_b0, eq27_e465_d_b1, eq27_e465_d_b2, eq27_e465_d_b3, eq27_e465_d_b4, eq27_e465_d_b5, eq27_e465_d_b6, eq27_e465_d_b7, eq27_e465_d_b8, eq27_e465_d_b9, eq27_e465_d_b10, eq27_e465_d_b11,) = {
    if (s.v[1848] != 0.0) {
        let eq27_e463: f64 = ((nv10 - 0.0) * s.v[589]);
        let eq27_e463_d_n0: f64 = ((nv10 - 0.0) * s.dn[589][0]);
        let eq27_e463_d_n1: f64 = ((nv10 - 0.0) * s.dn[589][1]);
        let eq27_e463_d_n2: f64 = ((nv10 - 0.0) * s.dn[589][2]);
        let eq27_e463_d_n3: f64 = ((nv10 - 0.0) * s.dn[589][3]);
        let eq27_e463_d_n4: f64 = ((nv10 - 0.0) * s.dn[589][4]);
        let eq27_e463_d_n5: f64 = ((nv10 - 0.0) * s.dn[589][5]);
        let eq27_e463_d_n6: f64 = ((nv10 - 0.0) * s.dn[589][6]);
        let eq27_e463_d_n7: f64 = ((nv10 - 0.0) * s.dn[589][7]);
        let eq27_e463_d_n8: f64 = ((nv10 - 0.0) * s.dn[589][8]);
        let eq27_e463_d_n9: f64 = ((nv10 - 0.0) * s.dn[589][9]);
        let eq27_e463_d_n10: f64 = (s.v[589] + ((nv10 - 0.0) * s.dn[589][10]));
        let eq27_e463_d_n11: f64 = ((nv10 - 0.0) * s.dn[589][11]);
        let eq27_e463_d_n12: f64 = ((nv10 - 0.0) * s.dn[589][12]);
        let eq27_e463_d_n13: f64 = ((nv10 - 0.0) * s.dn[589][13]);
        let eq27_e463_d_n14: f64 = ((nv10 - 0.0) * s.dn[589][14]);
        let eq27_e463_d_n15: f64 = ((nv10 - 0.0) * s.dn[589][15]);
        let eq27_e463_d_n16: f64 = ((nv10 - 0.0) * s.dn[589][16]);
        let eq27_e463_d_n17: f64 = ((nv10 - 0.0) * s.dn[589][17]);
        let eq27_e463_d_n18: f64 = ((nv10 - 0.0) * s.dn[589][18]);
        let eq27_e463_d_b0: f64 = ((nv10 - 0.0) * s.db[589][0]);
        let eq27_e463_d_b1: f64 = ((nv10 - 0.0) * s.db[589][1]);
        let eq27_e463_d_b2: f64 = ((nv10 - 0.0) * s.db[589][2]);
        let eq27_e463_d_b3: f64 = ((nv10 - 0.0) * s.db[589][3]);
        let eq27_e463_d_b4: f64 = ((nv10 - 0.0) * s.db[589][4]);
        let eq27_e463_d_b5: f64 = ((nv10 - 0.0) * s.db[589][5]);
        let eq27_e463_d_b6: f64 = ((nv10 - 0.0) * s.db[589][6]);
        let eq27_e463_d_b7: f64 = ((nv10 - 0.0) * s.db[589][7]);
        let eq27_e463_d_b8: f64 = ((nv10 - 0.0) * s.db[589][8]);
        let eq27_e463_d_b9: f64 = ((nv10 - 0.0) * s.db[589][9]);
        let eq27_e463_d_b10: f64 = ((nv10 - 0.0) * s.db[589][10]);
        let eq27_e463_d_b11: f64 = ((nv10 - 0.0) * s.db[589][11]);
        (eq27_e463, eq27_e463_d_n0, eq27_e463_d_n1, eq27_e463_d_n2, eq27_e463_d_n3, eq27_e463_d_n4, eq27_e463_d_n5, eq27_e463_d_n6, eq27_e463_d_n7, eq27_e463_d_n8, eq27_e463_d_n9, eq27_e463_d_n10, eq27_e463_d_n11, eq27_e463_d_n12, eq27_e463_d_n13, eq27_e463_d_n14, eq27_e463_d_n15, eq27_e463_d_n16, eq27_e463_d_n17, eq27_e463_d_n18, eq27_e463_d_b0, eq27_e463_d_b1, eq27_e463_d_b2, eq27_e463_d_b3, eq27_e463_d_b4, eq27_e463_d_b5, eq27_e463_d_b6, eq27_e463_d_b7, eq27_e463_d_b8, eq27_e463_d_b9, eq27_e463_d_b10, eq27_e463_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e465;
        let eq27_node_derivatives: [f64; 19] = [eq27_e465_d_n0, eq27_e465_d_n1, eq27_e465_d_n2, eq27_e465_d_n3, eq27_e465_d_n4, eq27_e465_d_n5, eq27_e465_d_n6, eq27_e465_d_n7, eq27_e465_d_n8, eq27_e465_d_n9, eq27_e465_d_n10, eq27_e465_d_n11, eq27_e465_d_n12, eq27_e465_d_n13, eq27_e465_d_n14, eq27_e465_d_n15, eq27_e465_d_n16, eq27_e465_d_n17, eq27_e465_d_n18];
        let eq27_branch_derivatives: [f64; 12] = [eq27_e465_d_b0, eq27_e465_d_b1, eq27_e465_d_b2, eq27_e465_d_b3, eq27_e465_d_b4, eq27_e465_d_b5, eq27_e465_d_b6, eq27_e465_d_b7, eq27_e465_d_b8, eq27_e465_d_b9, eq27_e465_d_b10, eq27_e465_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            None,
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
        let (eq28_e470, eq28_e470_d_n0, eq28_e470_d_n1, eq28_e470_d_n2, eq28_e470_d_n3, eq28_e470_d_n4, eq28_e470_d_n5, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n8, eq28_e470_d_n9, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n13, eq28_e470_d_n14, eq28_e470_d_n15, eq28_e470_d_n16, eq28_e470_d_n17, eq28_e470_d_n18, eq28_e470_d_b0, eq28_e470_d_b1, eq28_e470_d_b2, eq28_e470_d_b3, eq28_e470_d_b4, eq28_e470_d_b5, eq28_e470_d_b6, eq28_e470_d_b7, eq28_e470_d_b8, eq28_e470_d_b9, eq28_e470_d_b10, eq28_e470_d_b11,) = {
    if (s.v[1848] != 0.0) {
        let eq28_e468: f64 = (-s.v[595]);
        let eq28_e468_d_n0: f64 = (-s.dn[595][0]);
        let eq28_e468_d_n1: f64 = (-s.dn[595][1]);
        let eq28_e468_d_n2: f64 = (-s.dn[595][2]);
        let eq28_e468_d_n3: f64 = (-s.dn[595][3]);
        let eq28_e468_d_n4: f64 = (-s.dn[595][4]);
        let eq28_e468_d_n5: f64 = (-s.dn[595][5]);
        let eq28_e468_d_n6: f64 = (-s.dn[595][6]);
        let eq28_e468_d_n7: f64 = (-s.dn[595][7]);
        let eq28_e468_d_n8: f64 = (-s.dn[595][8]);
        let eq28_e468_d_n9: f64 = (-s.dn[595][9]);
        let eq28_e468_d_n10: f64 = (-s.dn[595][10]);
        let eq28_e468_d_n11: f64 = (-s.dn[595][11]);
        let eq28_e468_d_n12: f64 = (-s.dn[595][12]);
        let eq28_e468_d_n13: f64 = (-s.dn[595][13]);
        let eq28_e468_d_n14: f64 = (-s.dn[595][14]);
        let eq28_e468_d_n15: f64 = (-s.dn[595][15]);
        let eq28_e468_d_n16: f64 = (-s.dn[595][16]);
        let eq28_e468_d_n17: f64 = (-s.dn[595][17]);
        let eq28_e468_d_n18: f64 = (-s.dn[595][18]);
        let eq28_e468_d_b0: f64 = (-s.db[595][0]);
        let eq28_e468_d_b1: f64 = (-s.db[595][1]);
        let eq28_e468_d_b2: f64 = (-s.db[595][2]);
        let eq28_e468_d_b3: f64 = (-s.db[595][3]);
        let eq28_e468_d_b4: f64 = (-s.db[595][4]);
        let eq28_e468_d_b5: f64 = (-s.db[595][5]);
        let eq28_e468_d_b6: f64 = (-s.db[595][6]);
        let eq28_e468_d_b7: f64 = (-s.db[595][7]);
        let eq28_e468_d_b8: f64 = (-s.db[595][8]);
        let eq28_e468_d_b9: f64 = (-s.db[595][9]);
        let eq28_e468_d_b10: f64 = (-s.db[595][10]);
        let eq28_e468_d_b11: f64 = (-s.db[595][11]);
        (eq28_e468, eq28_e468_d_n0, eq28_e468_d_n1, eq28_e468_d_n2, eq28_e468_d_n3, eq28_e468_d_n4, eq28_e468_d_n5, eq28_e468_d_n6, eq28_e468_d_n7, eq28_e468_d_n8, eq28_e468_d_n9, eq28_e468_d_n10, eq28_e468_d_n11, eq28_e468_d_n12, eq28_e468_d_n13, eq28_e468_d_n14, eq28_e468_d_n15, eq28_e468_d_n16, eq28_e468_d_n17, eq28_e468_d_n18, eq28_e468_d_b0, eq28_e468_d_b1, eq28_e468_d_b2, eq28_e468_d_b3, eq28_e468_d_b4, eq28_e468_d_b5, eq28_e468_d_b6, eq28_e468_d_b7, eq28_e468_d_b8, eq28_e468_d_b9, eq28_e468_d_b10, eq28_e468_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e470;
        let eq28_node_derivatives: [f64; 19] = [eq28_e470_d_n0, eq28_e470_d_n1, eq28_e470_d_n2, eq28_e470_d_n3, eq28_e470_d_n4, eq28_e470_d_n5, eq28_e470_d_n6, eq28_e470_d_n7, eq28_e470_d_n8, eq28_e470_d_n9, eq28_e470_d_n10, eq28_e470_d_n11, eq28_e470_d_n12, eq28_e470_d_n13, eq28_e470_d_n14, eq28_e470_d_n15, eq28_e470_d_n16, eq28_e470_d_n17, eq28_e470_d_n18];
        let eq28_branch_derivatives: [f64; 12] = [eq28_e470_d_b0, eq28_e470_d_b1, eq28_e470_d_b2, eq28_e470_d_b3, eq28_e470_d_b4, eq28_e470_d_b5, eq28_e470_d_b6, eq28_e470_d_b7, eq28_e470_d_b8, eq28_e470_d_b9, eq28_e470_d_b10, eq28_e470_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            None,
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq29_e476, eq29_e476_d_n10,) = {
    if (s.v[1848] != 0.0) {
        let eq29_e474: f64 = ((nv10 - 0.0) * 1e-12);
        let eq29_e474_d_n10: f64 = 1e-12;
        (eq29_e474, eq29_e474_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e476;
        stamper.stamp_current(
            Some(nodes[10]),
            None,
            self.multiplicity * (eq29_value),
            &[
                GeneratedDerivative::node(nodes[10], self.multiplicity * eq29_e476_d_n10),
            ],
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq30_e483, eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18, eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11,) = {
    if (s.v[1848] != 0.0) {
        let eq30_e480: f64 = (s.v[563] * (nv10 - 0.0));
        let eq30_e480_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq30_e480_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq30_e480_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq30_e480_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq30_e480_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq30_e480_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq30_e480_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq30_e480_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq30_e480_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq30_e480_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq30_e480_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq30_e480_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq30_e480_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq30_e480_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq30_e480_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq30_e480_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq30_e480_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq30_e480_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq30_e480_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq30_e480_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq30_e480_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq30_e480_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq30_e480_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq30_e480_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq30_e480_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq30_e480_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq30_e480_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq30_e480_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq30_e480_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq30_e480_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq30_e480_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq30_e481: f64 = self.eval_ddt(5, eq30_e480);
        let eq30_e481_d_n0: f64 = self.ddt_jacobian(eq30_e480_d_n0);
        let eq30_e481_d_n1: f64 = self.ddt_jacobian(eq30_e480_d_n1);
        let eq30_e481_d_n2: f64 = self.ddt_jacobian(eq30_e480_d_n2);
        let eq30_e481_d_n3: f64 = self.ddt_jacobian(eq30_e480_d_n3);
        let eq30_e481_d_n4: f64 = self.ddt_jacobian(eq30_e480_d_n4);
        let eq30_e481_d_n5: f64 = self.ddt_jacobian(eq30_e480_d_n5);
        let eq30_e481_d_n6: f64 = self.ddt_jacobian(eq30_e480_d_n6);
        let eq30_e481_d_n7: f64 = self.ddt_jacobian(eq30_e480_d_n7);
        let eq30_e481_d_n8: f64 = self.ddt_jacobian(eq30_e480_d_n8);
        let eq30_e481_d_n9: f64 = self.ddt_jacobian(eq30_e480_d_n9);
        let eq30_e481_d_n10: f64 = self.ddt_jacobian(eq30_e480_d_n10);
        let eq30_e481_d_n11: f64 = self.ddt_jacobian(eq30_e480_d_n11);
        let eq30_e481_d_n12: f64 = self.ddt_jacobian(eq30_e480_d_n12);
        let eq30_e481_d_n13: f64 = self.ddt_jacobian(eq30_e480_d_n13);
        let eq30_e481_d_n14: f64 = self.ddt_jacobian(eq30_e480_d_n14);
        let eq30_e481_d_n15: f64 = self.ddt_jacobian(eq30_e480_d_n15);
        let eq30_e481_d_n16: f64 = self.ddt_jacobian(eq30_e480_d_n16);
        let eq30_e481_d_n17: f64 = self.ddt_jacobian(eq30_e480_d_n17);
        let eq30_e481_d_n18: f64 = self.ddt_jacobian(eq30_e480_d_n18);
        let eq30_e481_d_b0: f64 = self.ddt_jacobian(eq30_e480_d_b0);
        let eq30_e481_d_b1: f64 = self.ddt_jacobian(eq30_e480_d_b1);
        let eq30_e481_d_b2: f64 = self.ddt_jacobian(eq30_e480_d_b2);
        let eq30_e481_d_b3: f64 = self.ddt_jacobian(eq30_e480_d_b3);
        let eq30_e481_d_b4: f64 = self.ddt_jacobian(eq30_e480_d_b4);
        let eq30_e481_d_b5: f64 = self.ddt_jacobian(eq30_e480_d_b5);
        let eq30_e481_d_b6: f64 = self.ddt_jacobian(eq30_e480_d_b6);
        let eq30_e481_d_b7: f64 = self.ddt_jacobian(eq30_e480_d_b7);
        let eq30_e481_d_b8: f64 = self.ddt_jacobian(eq30_e480_d_b8);
        let eq30_e481_d_b9: f64 = self.ddt_jacobian(eq30_e480_d_b9);
        let eq30_e481_d_b10: f64 = self.ddt_jacobian(eq30_e480_d_b10);
        let eq30_e481_d_b11: f64 = self.ddt_jacobian(eq30_e480_d_b11);
        (eq30_e481, eq30_e481_d_n0, eq30_e481_d_n1, eq30_e481_d_n2, eq30_e481_d_n3, eq30_e481_d_n4, eq30_e481_d_n5, eq30_e481_d_n6, eq30_e481_d_n7, eq30_e481_d_n8, eq30_e481_d_n9, eq30_e481_d_n10, eq30_e481_d_n11, eq30_e481_d_n12, eq30_e481_d_n13, eq30_e481_d_n14, eq30_e481_d_n15, eq30_e481_d_n16, eq30_e481_d_n17, eq30_e481_d_n18, eq30_e481_d_b0, eq30_e481_d_b1, eq30_e481_d_b2, eq30_e481_d_b3, eq30_e481_d_b4, eq30_e481_d_b5, eq30_e481_d_b6, eq30_e481_d_b7, eq30_e481_d_b8, eq30_e481_d_b9, eq30_e481_d_b10, eq30_e481_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e483;
        let eq30_node_derivatives: [f64; 19] = [eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18];
        let eq30_branch_derivatives: [f64; 12] = [eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            None,
            self.multiplicity * (eq30_value),
            &nodes,
            &eq30_node_derivatives,
            &branches,
            &eq30_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_31_block_0(
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
        let (eq31_e490, eq31_e490_d_n10,) = {
    if (!(s.v[1848] != 0.0)) {
        let eq31_e488: f64 = ((nv10 - 0.0) * 10000.0);
        let eq31_e488_d_n10: f64 = 10000.0;
        (eq31_e488, eq31_e488_d_n10,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e490;
        stamper.stamp_current(
            Some(nodes[10]),
            None,
            self.multiplicity * (eq31_value),
            &[
                GeneratedDerivative::node(nodes[10], self.multiplicity * eq31_e490_d_n10),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_32_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq32_e498, eq32_e498_d_n0, eq32_e498_d_n1, eq32_e498_d_n2, eq32_e498_d_n3, eq32_e498_d_n4, eq32_e498_d_n5, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n8, eq32_e498_d_n9, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n13, eq32_e498_d_n14, eq32_e498_d_n15, eq32_e498_d_n16, eq32_e498_d_n17, eq32_e498_d_n18, eq32_e498_d_b0, eq32_e498_d_b1, eq32_e498_d_b2, eq32_e498_d_b3, eq32_e498_d_b4, eq32_e498_d_b5, eq32_e498_d_b6, eq32_e498_d_b7, eq32_e498_d_b8, eq32_e498_d_b9, eq32_e498_d_b10, eq32_e498_d_b11,) = {
    if (s.v[1849] != 0.0) {
        let eq32_e495: f64 = (s.v[311] + s.v[263]);
        let eq32_e495_d_n0: f64 = (s.dn[311][0] + s.dn[263][0]);
        let eq32_e495_d_n1: f64 = (s.dn[311][1] + s.dn[263][1]);
        let eq32_e495_d_n2: f64 = (s.dn[311][2] + s.dn[263][2]);
        let eq32_e495_d_n3: f64 = (s.dn[311][3] + s.dn[263][3]);
        let eq32_e495_d_n4: f64 = (s.dn[311][4] + s.dn[263][4]);
        let eq32_e495_d_n5: f64 = (s.dn[311][5] + s.dn[263][5]);
        let eq32_e495_d_n6: f64 = (s.dn[311][6] + s.dn[263][6]);
        let eq32_e495_d_n7: f64 = (s.dn[311][7] + s.dn[263][7]);
        let eq32_e495_d_n8: f64 = (s.dn[311][8] + s.dn[263][8]);
        let eq32_e495_d_n9: f64 = (s.dn[311][9] + s.dn[263][9]);
        let eq32_e495_d_n10: f64 = (s.dn[311][10] + s.dn[263][10]);
        let eq32_e495_d_n11: f64 = (s.dn[311][11] + s.dn[263][11]);
        let eq32_e495_d_n12: f64 = (s.dn[311][12] + s.dn[263][12]);
        let eq32_e495_d_n13: f64 = (s.dn[311][13] + s.dn[263][13]);
        let eq32_e495_d_n14: f64 = (s.dn[311][14] + s.dn[263][14]);
        let eq32_e495_d_n15: f64 = (s.dn[311][15] + s.dn[263][15]);
        let eq32_e495_d_n16: f64 = (s.dn[311][16] + s.dn[263][16]);
        let eq32_e495_d_n17: f64 = (s.dn[311][17] + s.dn[263][17]);
        let eq32_e495_d_n18: f64 = (s.dn[311][18] + s.dn[263][18]);
        let eq32_e495_d_b0: f64 = (s.db[311][0] + s.db[263][0]);
        let eq32_e495_d_b1: f64 = (s.db[311][1] + s.db[263][1]);
        let eq32_e495_d_b2: f64 = (s.db[311][2] + s.db[263][2]);
        let eq32_e495_d_b3: f64 = (s.db[311][3] + s.db[263][3]);
        let eq32_e495_d_b4: f64 = (s.db[311][4] + s.db[263][4]);
        let eq32_e495_d_b5: f64 = (s.db[311][5] + s.db[263][5]);
        let eq32_e495_d_b6: f64 = (s.db[311][6] + s.db[263][6]);
        let eq32_e495_d_b7: f64 = (s.db[311][7] + s.db[263][7]);
        let eq32_e495_d_b8: f64 = (s.db[311][8] + s.db[263][8]);
        let eq32_e495_d_b9: f64 = (s.db[311][9] + s.db[263][9]);
        let eq32_e495_d_b10: f64 = (s.db[311][10] + s.db[263][10]);
        let eq32_e495_d_b11: f64 = (s.db[311][11] + s.db[263][11]);
        let eq32_e496: f64 = (p.p50 * eq32_e495);
        let eq32_e496_d_n0: f64 = (p.p50 * eq32_e495_d_n0);
        let eq32_e496_d_n1: f64 = (p.p50 * eq32_e495_d_n1);
        let eq32_e496_d_n2: f64 = (p.p50 * eq32_e495_d_n2);
        let eq32_e496_d_n3: f64 = (p.p50 * eq32_e495_d_n3);
        let eq32_e496_d_n4: f64 = (p.p50 * eq32_e495_d_n4);
        let eq32_e496_d_n5: f64 = (p.p50 * eq32_e495_d_n5);
        let eq32_e496_d_n6: f64 = (p.p50 * eq32_e495_d_n6);
        let eq32_e496_d_n7: f64 = (p.p50 * eq32_e495_d_n7);
        let eq32_e496_d_n8: f64 = (p.p50 * eq32_e495_d_n8);
        let eq32_e496_d_n9: f64 = (p.p50 * eq32_e495_d_n9);
        let eq32_e496_d_n10: f64 = (p.p50 * eq32_e495_d_n10);
        let eq32_e496_d_n11: f64 = (p.p50 * eq32_e495_d_n11);
        let eq32_e496_d_n12: f64 = (p.p50 * eq32_e495_d_n12);
        let eq32_e496_d_n13: f64 = (p.p50 * eq32_e495_d_n13);
        let eq32_e496_d_n14: f64 = (p.p50 * eq32_e495_d_n14);
        let eq32_e496_d_n15: f64 = (p.p50 * eq32_e495_d_n15);
        let eq32_e496_d_n16: f64 = (p.p50 * eq32_e495_d_n16);
        let eq32_e496_d_n17: f64 = (p.p50 * eq32_e495_d_n17);
        let eq32_e496_d_n18: f64 = (p.p50 * eq32_e495_d_n18);
        let eq32_e496_d_b0: f64 = (p.p50 * eq32_e495_d_b0);
        let eq32_e496_d_b1: f64 = (p.p50 * eq32_e495_d_b1);
        let eq32_e496_d_b2: f64 = (p.p50 * eq32_e495_d_b2);
        let eq32_e496_d_b3: f64 = (p.p50 * eq32_e495_d_b3);
        let eq32_e496_d_b4: f64 = (p.p50 * eq32_e495_d_b4);
        let eq32_e496_d_b5: f64 = (p.p50 * eq32_e495_d_b5);
        let eq32_e496_d_b6: f64 = (p.p50 * eq32_e495_d_b6);
        let eq32_e496_d_b7: f64 = (p.p50 * eq32_e495_d_b7);
        let eq32_e496_d_b8: f64 = (p.p50 * eq32_e495_d_b8);
        let eq32_e496_d_b9: f64 = (p.p50 * eq32_e495_d_b9);
        let eq32_e496_d_b10: f64 = (p.p50 * eq32_e495_d_b10);
        let eq32_e496_d_b11: f64 = (p.p50 * eq32_e495_d_b11);
        (eq32_e496, eq32_e496_d_n0, eq32_e496_d_n1, eq32_e496_d_n2, eq32_e496_d_n3, eq32_e496_d_n4, eq32_e496_d_n5, eq32_e496_d_n6, eq32_e496_d_n7, eq32_e496_d_n8, eq32_e496_d_n9, eq32_e496_d_n10, eq32_e496_d_n11, eq32_e496_d_n12, eq32_e496_d_n13, eq32_e496_d_n14, eq32_e496_d_n15, eq32_e496_d_n16, eq32_e496_d_n17, eq32_e496_d_n18, eq32_e496_d_b0, eq32_e496_d_b1, eq32_e496_d_b2, eq32_e496_d_b3, eq32_e496_d_b4, eq32_e496_d_b5, eq32_e496_d_b6, eq32_e496_d_b7, eq32_e496_d_b8, eq32_e496_d_b9, eq32_e496_d_b10, eq32_e496_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e498;
        let eq32_node_derivatives: [f64; 19] = [eq32_e498_d_n0, eq32_e498_d_n1, eq32_e498_d_n2, eq32_e498_d_n3, eq32_e498_d_n4, eq32_e498_d_n5, eq32_e498_d_n6, eq32_e498_d_n7, eq32_e498_d_n8, eq32_e498_d_n9, eq32_e498_d_n10, eq32_e498_d_n11, eq32_e498_d_n12, eq32_e498_d_n13, eq32_e498_d_n14, eq32_e498_d_n15, eq32_e498_d_n16, eq32_e498_d_n17, eq32_e498_d_n18];
        let eq32_branch_derivatives: [f64; 12] = [eq32_e498_d_b0, eq32_e498_d_b1, eq32_e498_d_b2, eq32_e498_d_b3, eq32_e498_d_b4, eq32_e498_d_b5, eq32_e498_d_b6, eq32_e498_d_b7, eq32_e498_d_b8, eq32_e498_d_b9, eq32_e498_d_b10, eq32_e498_d_b11];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[12]),
            self.multiplicity * (eq32_value),
            &nodes,
            &eq32_node_derivatives,
            &branches,
            &eq32_branch_derivatives,
            self.multiplicity,
        );
    }
}

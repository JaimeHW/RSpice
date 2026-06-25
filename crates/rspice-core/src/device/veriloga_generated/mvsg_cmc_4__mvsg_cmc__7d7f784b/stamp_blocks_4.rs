#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq4_e399,) = {
    if (s.v[308] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e399;
        stamper.stamp_potential(
            branches[4],
            eq4_value,
            &[
            ],
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
        let (eq5_e403,) = {
    if (s.v[308] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e403;
        stamper.stamp_potential(
            branches[5],
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
        let (eq6_e408, eq6_e408_d_n0, eq6_e408_d_n1, eq6_e408_d_n2, eq6_e408_d_n3, eq6_e408_d_n4, eq6_e408_d_n5, eq6_e408_d_n6, eq6_e408_d_n7, eq6_e408_d_n8, eq6_e408_d_n9, eq6_e408_d_n10, eq6_e408_d_n11, eq6_e408_d_n12, eq6_e408_d_n13, eq6_e408_d_n14, eq6_e408_d_n15, eq6_e408_d_n16, eq6_e408_d_n17, eq6_e408_d_n18, eq6_e408_d_n19, eq6_e408_d_n20, eq6_e408_d_n21, eq6_e408_d_n22, eq6_e408_d_n23, eq6_e408_d_n24, eq6_e408_d_n25, eq6_e408_d_n26, eq6_e408_d_n27, eq6_e408_d_n28, eq6_e408_d_n29,) = {
    if (s.v[308] != 0.0) {
        let eq6_e406: f64 = (-s.v[222]);
        let eq6_e406_d_n0: f64 = (-s.dn[222][0]);
        let eq6_e406_d_n1: f64 = (-s.dn[222][1]);
        let eq6_e406_d_n2: f64 = (-s.dn[222][2]);
        let eq6_e406_d_n3: f64 = (-s.dn[222][3]);
        let eq6_e406_d_n4: f64 = (-s.dn[222][4]);
        let eq6_e406_d_n5: f64 = (-s.dn[222][5]);
        let eq6_e406_d_n6: f64 = (-s.dn[222][6]);
        let eq6_e406_d_n7: f64 = (-s.dn[222][7]);
        let eq6_e406_d_n8: f64 = (-s.dn[222][8]);
        let eq6_e406_d_n9: f64 = (-s.dn[222][9]);
        let eq6_e406_d_n10: f64 = (-s.dn[222][10]);
        let eq6_e406_d_n11: f64 = (-s.dn[222][11]);
        let eq6_e406_d_n12: f64 = (-s.dn[222][12]);
        let eq6_e406_d_n13: f64 = (-s.dn[222][13]);
        let eq6_e406_d_n14: f64 = (-s.dn[222][14]);
        let eq6_e406_d_n15: f64 = (-s.dn[222][15]);
        let eq6_e406_d_n16: f64 = (-s.dn[222][16]);
        let eq6_e406_d_n17: f64 = (-s.dn[222][17]);
        let eq6_e406_d_n18: f64 = (-s.dn[222][18]);
        let eq6_e406_d_n19: f64 = (-s.dn[222][19]);
        let eq6_e406_d_n20: f64 = (-s.dn[222][20]);
        let eq6_e406_d_n21: f64 = (-s.dn[222][21]);
        let eq6_e406_d_n22: f64 = (-s.dn[222][22]);
        let eq6_e406_d_n23: f64 = (-s.dn[222][23]);
        let eq6_e406_d_n24: f64 = (-s.dn[222][24]);
        let eq6_e406_d_n25: f64 = (-s.dn[222][25]);
        let eq6_e406_d_n26: f64 = (-s.dn[222][26]);
        let eq6_e406_d_n27: f64 = (-s.dn[222][27]);
        let eq6_e406_d_n28: f64 = (-s.dn[222][28]);
        let eq6_e406_d_n29: f64 = (-s.dn[222][29]);
        (eq6_e406, eq6_e406_d_n0, eq6_e406_d_n1, eq6_e406_d_n2, eq6_e406_d_n3, eq6_e406_d_n4, eq6_e406_d_n5, eq6_e406_d_n6, eq6_e406_d_n7, eq6_e406_d_n8, eq6_e406_d_n9, eq6_e406_d_n10, eq6_e406_d_n11, eq6_e406_d_n12, eq6_e406_d_n13, eq6_e406_d_n14, eq6_e406_d_n15, eq6_e406_d_n16, eq6_e406_d_n17, eq6_e406_d_n18, eq6_e406_d_n19, eq6_e406_d_n20, eq6_e406_d_n21, eq6_e406_d_n22, eq6_e406_d_n23, eq6_e406_d_n24, eq6_e406_d_n25, eq6_e406_d_n26, eq6_e406_d_n27, eq6_e406_d_n28, eq6_e406_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e408;
        let eq6_node_derivatives: [f64; 30] = [eq6_e408_d_n0, eq6_e408_d_n1, eq6_e408_d_n2, eq6_e408_d_n3, eq6_e408_d_n4, eq6_e408_d_n5, eq6_e408_d_n6, eq6_e408_d_n7, eq6_e408_d_n8, eq6_e408_d_n9, eq6_e408_d_n10, eq6_e408_d_n11, eq6_e408_d_n12, eq6_e408_d_n13, eq6_e408_d_n14, eq6_e408_d_n15, eq6_e408_d_n16, eq6_e408_d_n17, eq6_e408_d_n18, eq6_e408_d_n19, eq6_e408_d_n20, eq6_e408_d_n21, eq6_e408_d_n22, eq6_e408_d_n23, eq6_e408_d_n24, eq6_e408_d_n25, eq6_e408_d_n26, eq6_e408_d_n27, eq6_e408_d_n28, eq6_e408_d_n29];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[21]),
            None,
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
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq7_e414, eq7_e414_d_n21,) = {
    if (s.v[308] != 0.0) {
        let eq7_e412: f64 = ((nv21 - 0.0) / p.p329);
        let eq7_e412_d_n21: f64 = (1.0 / p.p329);
        (eq7_e412, eq7_e412_d_n21,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e414;
        stamper.stamp_current(
            Some(nodes[21]),
            None,
            self.multiplicity * (eq7_value),
            &[
                GeneratedDerivative::node(nodes[21], self.multiplicity * eq7_e414_d_n21),
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
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let (eq8_e421, eq8_e421_d_n0, eq8_e421_d_n1, eq8_e421_d_n2, eq8_e421_d_n3, eq8_e421_d_n4, eq8_e421_d_n5, eq8_e421_d_n6, eq8_e421_d_n7, eq8_e421_d_n8, eq8_e421_d_n9, eq8_e421_d_n10, eq8_e421_d_n11, eq8_e421_d_n12, eq8_e421_d_n13, eq8_e421_d_n14, eq8_e421_d_n15, eq8_e421_d_n16, eq8_e421_d_n17, eq8_e421_d_n18, eq8_e421_d_n19, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_d_n22, eq8_e421_d_n23, eq8_e421_d_n24, eq8_e421_d_n25, eq8_e421_d_n26, eq8_e421_d_n27, eq8_e421_d_n28, eq8_e421_d_n29,) = {
    if (s.v[308] != 0.0) {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e418_d_n20: f64 = (-p.p330);
        let eq8_e418_d_n21: f64 = p.p330;
        let eq8_e419: f64 = self.eval_ddt(0, eq8_e418);
        let eq8_e419_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n20: f64 = self.ddt_jacobian(eq8_e418_d_n20);
        let eq8_e419_d_n21: f64 = self.ddt_jacobian(eq8_e418_d_n21);
        let eq8_e419_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq8_e419_d_n29: f64 = self.ddt_jacobian(0.0);
        (eq8_e419, eq8_e419_d_n0, eq8_e419_d_n1, eq8_e419_d_n2, eq8_e419_d_n3, eq8_e419_d_n4, eq8_e419_d_n5, eq8_e419_d_n6, eq8_e419_d_n7, eq8_e419_d_n8, eq8_e419_d_n9, eq8_e419_d_n10, eq8_e419_d_n11, eq8_e419_d_n12, eq8_e419_d_n13, eq8_e419_d_n14, eq8_e419_d_n15, eq8_e419_d_n16, eq8_e419_d_n17, eq8_e419_d_n18, eq8_e419_d_n19, eq8_e419_d_n20, eq8_e419_d_n21, eq8_e419_d_n22, eq8_e419_d_n23, eq8_e419_d_n24, eq8_e419_d_n25, eq8_e419_d_n26, eq8_e419_d_n27, eq8_e419_d_n28, eq8_e419_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e421;
        let eq8_node_derivatives: [f64; 30] = [eq8_e421_d_n0, eq8_e421_d_n1, eq8_e421_d_n2, eq8_e421_d_n3, eq8_e421_d_n4, eq8_e421_d_n5, eq8_e421_d_n6, eq8_e421_d_n7, eq8_e421_d_n8, eq8_e421_d_n9, eq8_e421_d_n10, eq8_e421_d_n11, eq8_e421_d_n12, eq8_e421_d_n13, eq8_e421_d_n14, eq8_e421_d_n15, eq8_e421_d_n16, eq8_e421_d_n17, eq8_e421_d_n18, eq8_e421_d_n19, eq8_e421_d_n20, eq8_e421_d_n21, eq8_e421_d_n22, eq8_e421_d_n23, eq8_e421_d_n24, eq8_e421_d_n25, eq8_e421_d_n26, eq8_e421_d_n27, eq8_e421_d_n28, eq8_e421_d_n29];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[21]),
            Some(nodes[20]),
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
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq9_e428, eq9_e428_d_n0, eq9_e428_d_n1, eq9_e428_d_n2, eq9_e428_d_n3, eq9_e428_d_n4, eq9_e428_d_n5, eq9_e428_d_n6, eq9_e428_d_n7, eq9_e428_d_n8, eq9_e428_d_n9, eq9_e428_d_n10, eq9_e428_d_n11, eq9_e428_d_n12, eq9_e428_d_n13, eq9_e428_d_n14, eq9_e428_d_n15, eq9_e428_d_n16, eq9_e428_d_n17, eq9_e428_d_n18, eq9_e428_d_n19, eq9_e428_d_n20, eq9_e428_d_n21, eq9_e428_d_n22, eq9_e428_d_n23, eq9_e428_d_n24, eq9_e428_d_n25, eq9_e428_d_n26, eq9_e428_d_n27, eq9_e428_d_n28, eq9_e428_d_n29,) = {
    if (s.v[308] != 0.0) {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));
        let eq9_e425_d_n20: f64 = p.p332;
        let eq9_e426: f64 = self.eval_ddt(1, eq9_e425);
        let eq9_e426_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n20: f64 = self.ddt_jacobian(eq9_e425_d_n20);
        let eq9_e426_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq9_e426_d_n29: f64 = self.ddt_jacobian(0.0);
        (eq9_e426, eq9_e426_d_n0, eq9_e426_d_n1, eq9_e426_d_n2, eq9_e426_d_n3, eq9_e426_d_n4, eq9_e426_d_n5, eq9_e426_d_n6, eq9_e426_d_n7, eq9_e426_d_n8, eq9_e426_d_n9, eq9_e426_d_n10, eq9_e426_d_n11, eq9_e426_d_n12, eq9_e426_d_n13, eq9_e426_d_n14, eq9_e426_d_n15, eq9_e426_d_n16, eq9_e426_d_n17, eq9_e426_d_n18, eq9_e426_d_n19, eq9_e426_d_n20, eq9_e426_d_n21, eq9_e426_d_n22, eq9_e426_d_n23, eq9_e426_d_n24, eq9_e426_d_n25, eq9_e426_d_n26, eq9_e426_d_n27, eq9_e426_d_n28, eq9_e426_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e428;
        let eq9_node_derivatives: [f64; 30] = [eq9_e428_d_n0, eq9_e428_d_n1, eq9_e428_d_n2, eq9_e428_d_n3, eq9_e428_d_n4, eq9_e428_d_n5, eq9_e428_d_n6, eq9_e428_d_n7, eq9_e428_d_n8, eq9_e428_d_n9, eq9_e428_d_n10, eq9_e428_d_n11, eq9_e428_d_n12, eq9_e428_d_n13, eq9_e428_d_n14, eq9_e428_d_n15, eq9_e428_d_n16, eq9_e428_d_n17, eq9_e428_d_n18, eq9_e428_d_n19, eq9_e428_d_n20, eq9_e428_d_n21, eq9_e428_d_n22, eq9_e428_d_n23, eq9_e428_d_n24, eq9_e428_d_n25, eq9_e428_d_n26, eq9_e428_d_n27, eq9_e428_d_n28, eq9_e428_d_n29];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[20]),
            None,
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
        let nv20 = ctx.node_voltage(nodes[20]);
        let (eq10_e432, eq10_e432_d_n20,) = {
    if (s.v[308] != 0.0) {
        ((nv20 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e432;
        stamper.stamp_current(
            Some(nodes[20]),
            None,
            self.multiplicity * (eq10_value),
            &[
                GeneratedDerivative::node(nodes[20], self.multiplicity * eq10_e432_d_n20),
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
        let (eq11_e439,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq11_value: f64 = eq11_e439;
        stamper.stamp_potential(
            branches[6],
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
        let (eq12_e446,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq12_value: f64 = eq12_e446;
        stamper.stamp_potential(
            branches[7],
            eq12_value,
            &[
            ],
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq13_e455, eq13_e455_d_n0, eq13_e455_d_n2,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq13_e453: f64 = (p.p6 * (nv0 - nv2));
        let eq13_e453_d_n0: f64 = p.p6;
        let eq13_e453_d_n2: f64 = (-p.p6);
        (eq13_e453, eq13_e453_d_n0, eq13_e453_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e455;
        stamper.stamp_potential(
            branches[8],
            eq13_value,
            &[
                GeneratedDerivative::node(nodes[0], eq13_e455_d_n0),
                GeneratedDerivative::node(nodes[2], eq13_e455_d_n2),
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
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let eq14_ad_e518: A = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq14_ad_e513: A = {
            if ((!(((nv24 - nv23) / s.v[113]) > 50.0)) && (!(((nv24 - nv23) / s.v[113]) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, &nodes, Some(24), Some(23)), s.ad_value(113)))
            } else {
                let eq14_ad_e512: A = {
                    if ((!(((nv24 - nv23) / s.v[113]) > 50.0)) && (((nv24 - nv23) / s.v[113]) < (-50.0))) {
                        A::exp(A::neg(A::constant(50.0)))
                    } else {
                        {
                            if (((nv24 - nv23) / s.v[113]) > 50.0) {
                                A::scale(A::offset(A::offset(A::div(A::voltage(ctx, &nodes, Some(24), Some(23)), s.ad_value(113)), (-50.0)), 1.0), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                };
                eq14_ad_e512
            }
        };
        A::scale(A::offset(eq14_ad_e513, (-1.0)), p.p346)
    } else {
        A::constant(0.0)
    }
};
        let eq14_ad: A = eq14_ad_e518;
        stamper.stamp_current_dense(
            Some(nodes[24]),
            Some(nodes[23]),
            self.multiplicity * eq14_ad.value,
            &nodes,
            &eq14_ad.dn,
            &branches,
            &eq14_ad.db,
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
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let (eq15_e527, eq15_e527_d_n22, eq15_e527_d_n24,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq15_e525: f64 = ((nv22 - nv24) / p.p340);
        let eq15_e525_d_n22: f64 = (1.0 / p.p340);
        let eq15_e525_d_n24: f64 = (-1.0 / p.p340);
        (eq15_e525, eq15_e525_d_n22, eq15_e525_d_n24,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e527;
        stamper.stamp_current(
            Some(nodes[22]),
            Some(nodes[24]),
            self.multiplicity * (eq15_value),
            &[
                GeneratedDerivative::node(nodes[22], self.multiplicity * eq15_e527_d_n22),
                GeneratedDerivative::node(nodes[24], self.multiplicity * eq15_e527_d_n24),
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
        let nv22 = ctx.node_voltage(nodes[22]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let (eq16_e536, eq16_e536_d_n22, eq16_e536_d_n23,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq16_e534: f64 = ((nv22 - nv23) / p.p339);
        let eq16_e534_d_n22: f64 = (1.0 / p.p339);
        let eq16_e534_d_n23: f64 = (-1.0 / p.p339);
        (eq16_e534, eq16_e534_d_n22, eq16_e534_d_n23,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e536;
        stamper.stamp_current(
            Some(nodes[22]),
            Some(nodes[23]),
            self.multiplicity * (eq16_value),
            &[
                GeneratedDerivative::node(nodes[22], self.multiplicity * eq16_e536_d_n22),
                GeneratedDerivative::node(nodes[23], self.multiplicity * eq16_e536_d_n23),
            ],
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
        let (eq17_e564, eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq17_e543: f64 = self.eval_ddt(2, s.v[225]);
        let eq17_e543_d_n0: f64 = self.ddt_jacobian(s.dn[225][0]);
        let eq17_e543_d_n1: f64 = self.ddt_jacobian(s.dn[225][1]);
        let eq17_e543_d_n2: f64 = self.ddt_jacobian(s.dn[225][2]);
        let eq17_e543_d_n3: f64 = self.ddt_jacobian(s.dn[225][3]);
        let eq17_e543_d_n4: f64 = self.ddt_jacobian(s.dn[225][4]);
        let eq17_e543_d_n5: f64 = self.ddt_jacobian(s.dn[225][5]);
        let eq17_e543_d_n6: f64 = self.ddt_jacobian(s.dn[225][6]);
        let eq17_e543_d_n7: f64 = self.ddt_jacobian(s.dn[225][7]);
        let eq17_e543_d_n8: f64 = self.ddt_jacobian(s.dn[225][8]);
        let eq17_e543_d_n9: f64 = self.ddt_jacobian(s.dn[225][9]);
        let eq17_e543_d_n10: f64 = self.ddt_jacobian(s.dn[225][10]);
        let eq17_e543_d_n11: f64 = self.ddt_jacobian(s.dn[225][11]);
        let eq17_e543_d_n12: f64 = self.ddt_jacobian(s.dn[225][12]);
        let eq17_e543_d_n13: f64 = self.ddt_jacobian(s.dn[225][13]);
        let eq17_e543_d_n14: f64 = self.ddt_jacobian(s.dn[225][14]);
        let eq17_e543_d_n15: f64 = self.ddt_jacobian(s.dn[225][15]);
        let eq17_e543_d_n16: f64 = self.ddt_jacobian(s.dn[225][16]);
        let eq17_e543_d_n17: f64 = self.ddt_jacobian(s.dn[225][17]);
        let eq17_e543_d_n18: f64 = self.ddt_jacobian(s.dn[225][18]);
        let eq17_e543_d_n19: f64 = self.ddt_jacobian(s.dn[225][19]);
        let eq17_e543_d_n20: f64 = self.ddt_jacobian(s.dn[225][20]);
        let eq17_e543_d_n21: f64 = self.ddt_jacobian(s.dn[225][21]);
        let eq17_e543_d_n22: f64 = self.ddt_jacobian(s.dn[225][22]);
        let eq17_e543_d_n23: f64 = self.ddt_jacobian(s.dn[225][23]);
        let eq17_e543_d_n24: f64 = self.ddt_jacobian(s.dn[225][24]);
        let eq17_e543_d_n25: f64 = self.ddt_jacobian(s.dn[225][25]);
        let eq17_e543_d_n26: f64 = self.ddt_jacobian(s.dn[225][26]);
        let eq17_e543_d_n27: f64 = self.ddt_jacobian(s.dn[225][27]);
        let eq17_e543_d_n28: f64 = self.ddt_jacobian(s.dn[225][28]);
        let eq17_e543_d_n29: f64 = self.ddt_jacobian(s.dn[225][29]);
        let eq17_e544: f64 = (p.p341 * eq17_e543);
        let eq17_e544_d_n0: f64 = (p.p341 * eq17_e543_d_n0);
        let eq17_e544_d_n1: f64 = (p.p341 * eq17_e543_d_n1);
        let eq17_e544_d_n2: f64 = (p.p341 * eq17_e543_d_n2);
        let eq17_e544_d_n3: f64 = (p.p341 * eq17_e543_d_n3);
        let eq17_e544_d_n4: f64 = (p.p341 * eq17_e543_d_n4);
        let eq17_e544_d_n5: f64 = (p.p341 * eq17_e543_d_n5);
        let eq17_e544_d_n6: f64 = (p.p341 * eq17_e543_d_n6);
        let eq17_e544_d_n7: f64 = (p.p341 * eq17_e543_d_n7);
        let eq17_e544_d_n8: f64 = (p.p341 * eq17_e543_d_n8);
        let eq17_e544_d_n9: f64 = (p.p341 * eq17_e543_d_n9);
        let eq17_e544_d_n10: f64 = (p.p341 * eq17_e543_d_n10);
        let eq17_e544_d_n11: f64 = (p.p341 * eq17_e543_d_n11);
        let eq17_e544_d_n12: f64 = (p.p341 * eq17_e543_d_n12);
        let eq17_e544_d_n13: f64 = (p.p341 * eq17_e543_d_n13);
        let eq17_e544_d_n14: f64 = (p.p341 * eq17_e543_d_n14);
        let eq17_e544_d_n15: f64 = (p.p341 * eq17_e543_d_n15);
        let eq17_e544_d_n16: f64 = (p.p341 * eq17_e543_d_n16);
        let eq17_e544_d_n17: f64 = (p.p341 * eq17_e543_d_n17);
        let eq17_e544_d_n18: f64 = (p.p341 * eq17_e543_d_n18);
        let eq17_e544_d_n19: f64 = (p.p341 * eq17_e543_d_n19);
        let eq17_e544_d_n20: f64 = (p.p341 * eq17_e543_d_n20);
        let eq17_e544_d_n21: f64 = (p.p341 * eq17_e543_d_n21);
        let eq17_e544_d_n22: f64 = (p.p341 * eq17_e543_d_n22);
        let eq17_e544_d_n23: f64 = (p.p341 * eq17_e543_d_n23);
        let eq17_e544_d_n24: f64 = (p.p341 * eq17_e543_d_n24);
        let eq17_e544_d_n25: f64 = (p.p341 * eq17_e543_d_n25);
        let eq17_e544_d_n26: f64 = (p.p341 * eq17_e543_d_n26);
        let eq17_e544_d_n27: f64 = (p.p341 * eq17_e543_d_n27);
        let eq17_e544_d_n28: f64 = (p.p341 * eq17_e543_d_n28);
        let eq17_e544_d_n29: f64 = (p.p341 * eq17_e543_d_n29);
        let eq17_e549: f64 = (s.v[111] - s.v[109]);
        let eq17_e550: f64 = (p.p342 * eq17_e549);
        let eq17_e550_d_n0: f64 = (p.p342 * s.dn[111][0]);
        let eq17_e550_d_n1: f64 = (p.p342 * s.dn[111][1]);
        let eq17_e550_d_n2: f64 = (p.p342 * s.dn[111][2]);
        let eq17_e550_d_n3: f64 = (p.p342 * s.dn[111][3]);
        let eq17_e550_d_n4: f64 = (p.p342 * s.dn[111][4]);
        let eq17_e550_d_n5: f64 = (p.p342 * s.dn[111][5]);
        let eq17_e550_d_n6: f64 = (p.p342 * s.dn[111][6]);
        let eq17_e550_d_n7: f64 = (p.p342 * s.dn[111][7]);
        let eq17_e550_d_n8: f64 = (p.p342 * s.dn[111][8]);
        let eq17_e550_d_n9: f64 = (p.p342 * s.dn[111][9]);
        let eq17_e550_d_n10: f64 = (p.p342 * s.dn[111][10]);
        let eq17_e550_d_n11: f64 = (p.p342 * s.dn[111][11]);
        let eq17_e550_d_n12: f64 = (p.p342 * s.dn[111][12]);
        let eq17_e550_d_n13: f64 = (p.p342 * s.dn[111][13]);
        let eq17_e550_d_n14: f64 = (p.p342 * s.dn[111][14]);
        let eq17_e550_d_n15: f64 = (p.p342 * s.dn[111][15]);
        let eq17_e550_d_n16: f64 = (p.p342 * s.dn[111][16]);
        let eq17_e550_d_n17: f64 = (p.p342 * s.dn[111][17]);
        let eq17_e550_d_n18: f64 = (p.p342 * s.dn[111][18]);
        let eq17_e550_d_n19: f64 = (p.p342 * s.dn[111][19]);
        let eq17_e550_d_n20: f64 = (p.p342 * s.dn[111][20]);
        let eq17_e550_d_n21: f64 = (p.p342 * s.dn[111][21]);
        let eq17_e550_d_n22: f64 = (p.p342 * s.dn[111][22]);
        let eq17_e550_d_n23: f64 = (p.p342 * s.dn[111][23]);
        let eq17_e550_d_n24: f64 = (p.p342 * s.dn[111][24]);
        let eq17_e550_d_n25: f64 = (p.p342 * s.dn[111][25]);
        let eq17_e550_d_n26: f64 = (p.p342 * s.dn[111][26]);
        let eq17_e550_d_n27: f64 = (p.p342 * s.dn[111][27]);
        let eq17_e550_d_n28: f64 = (p.p342 * s.dn[111][28]);
        let eq17_e550_d_n29: f64 = (p.p342 * s.dn[111][29]);
        let eq17_e551: f64 = (1.0 + eq17_e550);
        let eq17_e555: f64 = (s.v[111] - s.v[109]);
        let eq17_e556: f64 = (p.p344 * eq17_e555);
        let eq17_e556_d_n0: f64 = (p.p344 * s.dn[111][0]);
        let eq17_e556_d_n1: f64 = (p.p344 * s.dn[111][1]);
        let eq17_e556_d_n2: f64 = (p.p344 * s.dn[111][2]);
        let eq17_e556_d_n3: f64 = (p.p344 * s.dn[111][3]);
        let eq17_e556_d_n4: f64 = (p.p344 * s.dn[111][4]);
        let eq17_e556_d_n5: f64 = (p.p344 * s.dn[111][5]);
        let eq17_e556_d_n6: f64 = (p.p344 * s.dn[111][6]);
        let eq17_e556_d_n7: f64 = (p.p344 * s.dn[111][7]);
        let eq17_e556_d_n8: f64 = (p.p344 * s.dn[111][8]);
        let eq17_e556_d_n9: f64 = (p.p344 * s.dn[111][9]);
        let eq17_e556_d_n10: f64 = (p.p344 * s.dn[111][10]);
        let eq17_e556_d_n11: f64 = (p.p344 * s.dn[111][11]);
        let eq17_e556_d_n12: f64 = (p.p344 * s.dn[111][12]);
        let eq17_e556_d_n13: f64 = (p.p344 * s.dn[111][13]);
        let eq17_e556_d_n14: f64 = (p.p344 * s.dn[111][14]);
        let eq17_e556_d_n15: f64 = (p.p344 * s.dn[111][15]);
        let eq17_e556_d_n16: f64 = (p.p344 * s.dn[111][16]);
        let eq17_e556_d_n17: f64 = (p.p344 * s.dn[111][17]);
        let eq17_e556_d_n18: f64 = (p.p344 * s.dn[111][18]);
        let eq17_e556_d_n19: f64 = (p.p344 * s.dn[111][19]);
        let eq17_e556_d_n20: f64 = (p.p344 * s.dn[111][20]);
        let eq17_e556_d_n21: f64 = (p.p344 * s.dn[111][21]);
        let eq17_e556_d_n22: f64 = (p.p344 * s.dn[111][22]);
        let eq17_e556_d_n23: f64 = (p.p344 * s.dn[111][23]);
        let eq17_e556_d_n24: f64 = (p.p344 * s.dn[111][24]);
        let eq17_e556_d_n25: f64 = (p.p344 * s.dn[111][25]);
        let eq17_e556_d_n26: f64 = (p.p344 * s.dn[111][26]);
        let eq17_e556_d_n27: f64 = (p.p344 * s.dn[111][27]);
        let eq17_e556_d_n28: f64 = (p.p344 * s.dn[111][28]);
        let eq17_e556_d_n29: f64 = (p.p344 * s.dn[111][29]);
        let eq17_e559: f64 = (s.v[111] - s.v[109]);
        let eq17_e560: f64 = (eq17_e556 * eq17_e559);
        let eq17_e560_d_n0: f64 = ((eq17_e556_d_n0 * eq17_e559) + (eq17_e556 * s.dn[111][0]));
        let eq17_e560_d_n1: f64 = ((eq17_e556_d_n1 * eq17_e559) + (eq17_e556 * s.dn[111][1]));
        let eq17_e560_d_n2: f64 = ((eq17_e556_d_n2 * eq17_e559) + (eq17_e556 * s.dn[111][2]));
        let eq17_e560_d_n3: f64 = ((eq17_e556_d_n3 * eq17_e559) + (eq17_e556 * s.dn[111][3]));
        let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * s.dn[111][4]));
        let eq17_e560_d_n5: f64 = ((eq17_e556_d_n5 * eq17_e559) + (eq17_e556 * s.dn[111][5]));
        let eq17_e560_d_n6: f64 = ((eq17_e556_d_n6 * eq17_e559) + (eq17_e556 * s.dn[111][6]));
        let eq17_e560_d_n7: f64 = ((eq17_e556_d_n7 * eq17_e559) + (eq17_e556 * s.dn[111][7]));
        let eq17_e560_d_n8: f64 = ((eq17_e556_d_n8 * eq17_e559) + (eq17_e556 * s.dn[111][8]));
        let eq17_e560_d_n9: f64 = ((eq17_e556_d_n9 * eq17_e559) + (eq17_e556 * s.dn[111][9]));
        let eq17_e560_d_n10: f64 = ((eq17_e556_d_n10 * eq17_e559) + (eq17_e556 * s.dn[111][10]));
        let eq17_e560_d_n11: f64 = ((eq17_e556_d_n11 * eq17_e559) + (eq17_e556 * s.dn[111][11]));
        let eq17_e560_d_n12: f64 = ((eq17_e556_d_n12 * eq17_e559) + (eq17_e556 * s.dn[111][12]));
        let eq17_e560_d_n13: f64 = ((eq17_e556_d_n13 * eq17_e559) + (eq17_e556 * s.dn[111][13]));
        let eq17_e560_d_n14: f64 = ((eq17_e556_d_n14 * eq17_e559) + (eq17_e556 * s.dn[111][14]));
        let eq17_e560_d_n15: f64 = ((eq17_e556_d_n15 * eq17_e559) + (eq17_e556 * s.dn[111][15]));
        let eq17_e560_d_n16: f64 = ((eq17_e556_d_n16 * eq17_e559) + (eq17_e556 * s.dn[111][16]));
        let eq17_e560_d_n17: f64 = ((eq17_e556_d_n17 * eq17_e559) + (eq17_e556 * s.dn[111][17]));
        let eq17_e560_d_n18: f64 = ((eq17_e556_d_n18 * eq17_e559) + (eq17_e556 * s.dn[111][18]));
        let eq17_e560_d_n19: f64 = ((eq17_e556_d_n19 * eq17_e559) + (eq17_e556 * s.dn[111][19]));
        let eq17_e560_d_n20: f64 = ((eq17_e556_d_n20 * eq17_e559) + (eq17_e556 * s.dn[111][20]));
        let eq17_e560_d_n21: f64 = ((eq17_e556_d_n21 * eq17_e559) + (eq17_e556 * s.dn[111][21]));
        let eq17_e560_d_n22: f64 = ((eq17_e556_d_n22 * eq17_e559) + (eq17_e556 * s.dn[111][22]));
        let eq17_e560_d_n23: f64 = ((eq17_e556_d_n23 * eq17_e559) + (eq17_e556 * s.dn[111][23]));
        let eq17_e560_d_n24: f64 = ((eq17_e556_d_n24 * eq17_e559) + (eq17_e556 * s.dn[111][24]));
        let eq17_e560_d_n25: f64 = ((eq17_e556_d_n25 * eq17_e559) + (eq17_e556 * s.dn[111][25]));
        let eq17_e560_d_n26: f64 = ((eq17_e556_d_n26 * eq17_e559) + (eq17_e556 * s.dn[111][26]));
        let eq17_e560_d_n27: f64 = ((eq17_e556_d_n27 * eq17_e559) + (eq17_e556 * s.dn[111][27]));
        let eq17_e560_d_n28: f64 = ((eq17_e556_d_n28 * eq17_e559) + (eq17_e556 * s.dn[111][28]));
        let eq17_e560_d_n29: f64 = ((eq17_e556_d_n29 * eq17_e559) + (eq17_e556 * s.dn[111][29]));
        let eq17_e561: f64 = (eq17_e551 + eq17_e560);
        let eq17_e561_d_n0: f64 = (eq17_e550_d_n0 + eq17_e560_d_n0);
        let eq17_e561_d_n1: f64 = (eq17_e550_d_n1 + eq17_e560_d_n1);
        let eq17_e561_d_n2: f64 = (eq17_e550_d_n2 + eq17_e560_d_n2);
        let eq17_e561_d_n3: f64 = (eq17_e550_d_n3 + eq17_e560_d_n3);
        let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);
        let eq17_e561_d_n5: f64 = (eq17_e550_d_n5 + eq17_e560_d_n5);
        let eq17_e561_d_n6: f64 = (eq17_e550_d_n6 + eq17_e560_d_n6);
        let eq17_e561_d_n7: f64 = (eq17_e550_d_n7 + eq17_e560_d_n7);
        let eq17_e561_d_n8: f64 = (eq17_e550_d_n8 + eq17_e560_d_n8);
        let eq17_e561_d_n9: f64 = (eq17_e550_d_n9 + eq17_e560_d_n9);
        let eq17_e561_d_n10: f64 = (eq17_e550_d_n10 + eq17_e560_d_n10);
        let eq17_e561_d_n11: f64 = (eq17_e550_d_n11 + eq17_e560_d_n11);
        let eq17_e561_d_n12: f64 = (eq17_e550_d_n12 + eq17_e560_d_n12);
        let eq17_e561_d_n13: f64 = (eq17_e550_d_n13 + eq17_e560_d_n13);
        let eq17_e561_d_n14: f64 = (eq17_e550_d_n14 + eq17_e560_d_n14);
        let eq17_e561_d_n15: f64 = (eq17_e550_d_n15 + eq17_e560_d_n15);
        let eq17_e561_d_n16: f64 = (eq17_e550_d_n16 + eq17_e560_d_n16);
        let eq17_e561_d_n17: f64 = (eq17_e550_d_n17 + eq17_e560_d_n17);
        let eq17_e561_d_n18: f64 = (eq17_e550_d_n18 + eq17_e560_d_n18);
        let eq17_e561_d_n19: f64 = (eq17_e550_d_n19 + eq17_e560_d_n19);
        let eq17_e561_d_n20: f64 = (eq17_e550_d_n20 + eq17_e560_d_n20);
        let eq17_e561_d_n21: f64 = (eq17_e550_d_n21 + eq17_e560_d_n21);
        let eq17_e561_d_n22: f64 = (eq17_e550_d_n22 + eq17_e560_d_n22);
        let eq17_e561_d_n23: f64 = (eq17_e550_d_n23 + eq17_e560_d_n23);
        let eq17_e561_d_n24: f64 = (eq17_e550_d_n24 + eq17_e560_d_n24);
        let eq17_e561_d_n25: f64 = (eq17_e550_d_n25 + eq17_e560_d_n25);
        let eq17_e561_d_n26: f64 = (eq17_e550_d_n26 + eq17_e560_d_n26);
        let eq17_e561_d_n27: f64 = (eq17_e550_d_n27 + eq17_e560_d_n27);
        let eq17_e561_d_n28: f64 = (eq17_e550_d_n28 + eq17_e560_d_n28);
        let eq17_e561_d_n29: f64 = (eq17_e550_d_n29 + eq17_e560_d_n29);
        let eq17_e562: f64 = (eq17_e544 * eq17_e561);
        let eq17_e562_d_n0: f64 = ((eq17_e544_d_n0 * eq17_e561) + (eq17_e544 * eq17_e561_d_n0));
        let eq17_e562_d_n1: f64 = ((eq17_e544_d_n1 * eq17_e561) + (eq17_e544 * eq17_e561_d_n1));
        let eq17_e562_d_n2: f64 = ((eq17_e544_d_n2 * eq17_e561) + (eq17_e544 * eq17_e561_d_n2));
        let eq17_e562_d_n3: f64 = ((eq17_e544_d_n3 * eq17_e561) + (eq17_e544 * eq17_e561_d_n3));
        let eq17_e562_d_n4: f64 = ((eq17_e544_d_n4 * eq17_e561) + (eq17_e544 * eq17_e561_d_n4));
        let eq17_e562_d_n5: f64 = ((eq17_e544_d_n5 * eq17_e561) + (eq17_e544 * eq17_e561_d_n5));
        let eq17_e562_d_n6: f64 = ((eq17_e544_d_n6 * eq17_e561) + (eq17_e544 * eq17_e561_d_n6));
        let eq17_e562_d_n7: f64 = ((eq17_e544_d_n7 * eq17_e561) + (eq17_e544 * eq17_e561_d_n7));
        let eq17_e562_d_n8: f64 = ((eq17_e544_d_n8 * eq17_e561) + (eq17_e544 * eq17_e561_d_n8));
        let eq17_e562_d_n9: f64 = ((eq17_e544_d_n9 * eq17_e561) + (eq17_e544 * eq17_e561_d_n9));
        let eq17_e562_d_n10: f64 = ((eq17_e544_d_n10 * eq17_e561) + (eq17_e544 * eq17_e561_d_n10));
        let eq17_e562_d_n11: f64 = ((eq17_e544_d_n11 * eq17_e561) + (eq17_e544 * eq17_e561_d_n11));
        let eq17_e562_d_n12: f64 = ((eq17_e544_d_n12 * eq17_e561) + (eq17_e544 * eq17_e561_d_n12));
        let eq17_e562_d_n13: f64 = ((eq17_e544_d_n13 * eq17_e561) + (eq17_e544 * eq17_e561_d_n13));
        let eq17_e562_d_n14: f64 = ((eq17_e544_d_n14 * eq17_e561) + (eq17_e544 * eq17_e561_d_n14));
        let eq17_e562_d_n15: f64 = ((eq17_e544_d_n15 * eq17_e561) + (eq17_e544 * eq17_e561_d_n15));
        let eq17_e562_d_n16: f64 = ((eq17_e544_d_n16 * eq17_e561) + (eq17_e544 * eq17_e561_d_n16));
        let eq17_e562_d_n17: f64 = ((eq17_e544_d_n17 * eq17_e561) + (eq17_e544 * eq17_e561_d_n17));
        let eq17_e562_d_n18: f64 = ((eq17_e544_d_n18 * eq17_e561) + (eq17_e544 * eq17_e561_d_n18));
        let eq17_e562_d_n19: f64 = ((eq17_e544_d_n19 * eq17_e561) + (eq17_e544 * eq17_e561_d_n19));
        let eq17_e562_d_n20: f64 = ((eq17_e544_d_n20 * eq17_e561) + (eq17_e544 * eq17_e561_d_n20));
        let eq17_e562_d_n21: f64 = ((eq17_e544_d_n21 * eq17_e561) + (eq17_e544 * eq17_e561_d_n21));
        let eq17_e562_d_n22: f64 = ((eq17_e544_d_n22 * eq17_e561) + (eq17_e544 * eq17_e561_d_n22));
        let eq17_e562_d_n23: f64 = ((eq17_e544_d_n23 * eq17_e561) + (eq17_e544 * eq17_e561_d_n23));
        let eq17_e562_d_n24: f64 = ((eq17_e544_d_n24 * eq17_e561) + (eq17_e544 * eq17_e561_d_n24));
        let eq17_e562_d_n25: f64 = ((eq17_e544_d_n25 * eq17_e561) + (eq17_e544 * eq17_e561_d_n25));
        let eq17_e562_d_n26: f64 = ((eq17_e544_d_n26 * eq17_e561) + (eq17_e544 * eq17_e561_d_n26));
        let eq17_e562_d_n27: f64 = ((eq17_e544_d_n27 * eq17_e561) + (eq17_e544 * eq17_e561_d_n27));
        let eq17_e562_d_n28: f64 = ((eq17_e544_d_n28 * eq17_e561) + (eq17_e544 * eq17_e561_d_n28));
        let eq17_e562_d_n29: f64 = ((eq17_e544_d_n29 * eq17_e561) + (eq17_e544 * eq17_e561_d_n29));
        (eq17_e562, eq17_e562_d_n0, eq17_e562_d_n1, eq17_e562_d_n2, eq17_e562_d_n3, eq17_e562_d_n4, eq17_e562_d_n5, eq17_e562_d_n6, eq17_e562_d_n7, eq17_e562_d_n8, eq17_e562_d_n9, eq17_e562_d_n10, eq17_e562_d_n11, eq17_e562_d_n12, eq17_e562_d_n13, eq17_e562_d_n14, eq17_e562_d_n15, eq17_e562_d_n16, eq17_e562_d_n17, eq17_e562_d_n18, eq17_e562_d_n19, eq17_e562_d_n20, eq17_e562_d_n21, eq17_e562_d_n22, eq17_e562_d_n23, eq17_e562_d_n24, eq17_e562_d_n25, eq17_e562_d_n26, eq17_e562_d_n27, eq17_e562_d_n28, eq17_e562_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e564;
        let eq17_node_derivatives: [f64; 30] = [eq17_e564_d_n0, eq17_e564_d_n1, eq17_e564_d_n2, eq17_e564_d_n3, eq17_e564_d_n4, eq17_e564_d_n5, eq17_e564_d_n6, eq17_e564_d_n7, eq17_e564_d_n8, eq17_e564_d_n9, eq17_e564_d_n10, eq17_e564_d_n11, eq17_e564_d_n12, eq17_e564_d_n13, eq17_e564_d_n14, eq17_e564_d_n15, eq17_e564_d_n16, eq17_e564_d_n17, eq17_e564_d_n18, eq17_e564_d_n19, eq17_e564_d_n20, eq17_e564_d_n21, eq17_e564_d_n22, eq17_e564_d_n23, eq17_e564_d_n24, eq17_e564_d_n25, eq17_e564_d_n26, eq17_e564_d_n27, eq17_e564_d_n28, eq17_e564_d_n29];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[23]),
            None,
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
        let nv2 = ctx.node_voltage(nodes[2]);
        let (eq18_e573, eq18_e573_d_n1, eq18_e573_d_n2,) = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq18_e571: f64 = (p.p6 * (nv1 - nv2));
        let eq18_e571_d_n1: f64 = p.p6;
        let eq18_e571_d_n2: f64 = (-p.p6);
        (eq18_e571, eq18_e571_d_n1, eq18_e571_d_n2,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e573;
        stamper.stamp_potential(
            branches[9],
            eq18_value,
            &[
                GeneratedDerivative::node(nodes[1], eq18_e573_d_n1),
                GeneratedDerivative::node(nodes[2], eq18_e573_d_n2),
            ],
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
        let nv26 = ctx.node_voltage(nodes[26]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let eq19_ad_e636: A = {
    if ((!(s.v[308] != 0.0)) && (s.v[309] != 0.0)) {
        let eq19_ad_e631: A = {
            if ((!(((nv26 - nv27) / s.v[113]) > 50.0)) && (!(((nv26 - nv27) / s.v[113]) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, &nodes, Some(26), Some(27)), s.ad_value(113)))
            } else {
                let eq19_ad_e630: A = {
                    if ((!(((nv26 - nv27) / s.v[113]) > 50.0)) && (((nv26 - nv27) / s.v[113]) < (-50.0))) {
                        A::exp(A::neg(A::constant(50.0)))
                    } else {
                        {
                            if (((nv26 - nv27) / s.v[113]) > 50.0) {
                                A::scale(A::offset(A::offset(A::div(A::voltage(ctx, &nodes, Some(26), Some(27)), s.ad_value(113)), (-50.0)), 1.0), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                };
                eq19_ad_e630
            }
        };
        A::scale(A::offset(eq19_ad_e631, (-1.0)), p.p346)
    } else {
        A::constant(0.0)
    }
};
        let eq19_ad: A = eq19_ad_e636;
        stamper.stamp_current_dense(
            Some(nodes[26]),
            Some(nodes[27]),
            self.multiplicity * eq19_ad.value,
            &nodes,
            &eq19_ad.dn,
            &branches,
            &eq19_ad.db,
            self.multiplicity,
        );
    }
}

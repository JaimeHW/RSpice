#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_56_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n1, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n12, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_d_n15, eq56_e2426_d_n16, eq56_e2426_q, eq56_e2426_q_d_n0, eq56_e2426_q_d_n1, eq56_e2426_q_d_n2, eq56_e2426_q_d_n3, eq56_e2426_q_d_n4, eq56_e2426_q_d_n5, eq56_e2426_q_d_n6, eq56_e2426_q_d_n7, eq56_e2426_q_d_n8, eq56_e2426_q_d_n9, eq56_e2426_q_d_n10, eq56_e2426_q_d_n11, eq56_e2426_q_d_n12, eq56_e2426_q_d_n13, eq56_e2426_q_d_n14, eq56_e2426_q_d_n15, eq56_e2426_q_d_n16,) = {
    if (s.v[1710] != 0.0) {
        let eq56_e2423_q: f64 = s.v[496];
        let eq56_e2424: f64 = (s.v[114] * s.v[496]);
        let eq56_e2424_d_n0: f64 = ((s.dn[114][0] * s.v[496]) + (s.v[114] * s.dn[496][0]));
        let eq56_e2424_d_n1: f64 = ((s.dn[114][1] * s.v[496]) + (s.v[114] * s.dn[496][1]));
        let eq56_e2424_d_n2: f64 = ((s.dn[114][2] * s.v[496]) + (s.v[114] * s.dn[496][2]));
        let eq56_e2424_d_n3: f64 = ((s.dn[114][3] * s.v[496]) + (s.v[114] * s.dn[496][3]));
        let eq56_e2424_d_n4: f64 = ((s.dn[114][4] * s.v[496]) + (s.v[114] * s.dn[496][4]));
        let eq56_e2424_d_n5: f64 = ((s.dn[114][5] * s.v[496]) + (s.v[114] * s.dn[496][5]));
        let eq56_e2424_d_n6: f64 = ((s.dn[114][6] * s.v[496]) + (s.v[114] * s.dn[496][6]));
        let eq56_e2424_d_n7: f64 = ((s.dn[114][7] * s.v[496]) + (s.v[114] * s.dn[496][7]));
        let eq56_e2424_d_n8: f64 = ((s.dn[114][8] * s.v[496]) + (s.v[114] * s.dn[496][8]));
        let eq56_e2424_d_n9: f64 = ((s.dn[114][9] * s.v[496]) + (s.v[114] * s.dn[496][9]));
        let eq56_e2424_d_n10: f64 = ((s.dn[114][10] * s.v[496]) + (s.v[114] * s.dn[496][10]));
        let eq56_e2424_d_n11: f64 = ((s.dn[114][11] * s.v[496]) + (s.v[114] * s.dn[496][11]));
        let eq56_e2424_d_n12: f64 = ((s.dn[114][12] * s.v[496]) + (s.v[114] * s.dn[496][12]));
        let eq56_e2424_d_n13: f64 = ((s.dn[114][13] * s.v[496]) + (s.v[114] * s.dn[496][13]));
        let eq56_e2424_d_n14: f64 = ((s.dn[114][14] * s.v[496]) + (s.v[114] * s.dn[496][14]));
        let eq56_e2424_d_n15: f64 = ((s.dn[114][15] * s.v[496]) + (s.v[114] * s.dn[496][15]));
        let eq56_e2424_d_n16: f64 = ((s.dn[114][16] * s.v[496]) + (s.v[114] * s.dn[496][16]));
        let eq56_e2424_q: f64 = (s.v[114] * eq56_e2423_q);
        let eq56_e2424_q_d_n0: f64 = ((s.dn[114][0] * eq56_e2423_q) + (s.v[114] * s.dn[496][0]));
        let eq56_e2424_q_d_n1: f64 = ((s.dn[114][1] * eq56_e2423_q) + (s.v[114] * s.dn[496][1]));
        let eq56_e2424_q_d_n2: f64 = ((s.dn[114][2] * eq56_e2423_q) + (s.v[114] * s.dn[496][2]));
        let eq56_e2424_q_d_n3: f64 = ((s.dn[114][3] * eq56_e2423_q) + (s.v[114] * s.dn[496][3]));
        let eq56_e2424_q_d_n4: f64 = ((s.dn[114][4] * eq56_e2423_q) + (s.v[114] * s.dn[496][4]));
        let eq56_e2424_q_d_n5: f64 = ((s.dn[114][5] * eq56_e2423_q) + (s.v[114] * s.dn[496][5]));
        let eq56_e2424_q_d_n6: f64 = ((s.dn[114][6] * eq56_e2423_q) + (s.v[114] * s.dn[496][6]));
        let eq56_e2424_q_d_n7: f64 = ((s.dn[114][7] * eq56_e2423_q) + (s.v[114] * s.dn[496][7]));
        let eq56_e2424_q_d_n8: f64 = ((s.dn[114][8] * eq56_e2423_q) + (s.v[114] * s.dn[496][8]));
        let eq56_e2424_q_d_n9: f64 = ((s.dn[114][9] * eq56_e2423_q) + (s.v[114] * s.dn[496][9]));
        let eq56_e2424_q_d_n10: f64 = ((s.dn[114][10] * eq56_e2423_q) + (s.v[114] * s.dn[496][10]));
        let eq56_e2424_q_d_n11: f64 = ((s.dn[114][11] * eq56_e2423_q) + (s.v[114] * s.dn[496][11]));
        let eq56_e2424_q_d_n12: f64 = ((s.dn[114][12] * eq56_e2423_q) + (s.v[114] * s.dn[496][12]));
        let eq56_e2424_q_d_n13: f64 = ((s.dn[114][13] * eq56_e2423_q) + (s.v[114] * s.dn[496][13]));
        let eq56_e2424_q_d_n14: f64 = ((s.dn[114][14] * eq56_e2423_q) + (s.v[114] * s.dn[496][14]));
        let eq56_e2424_q_d_n15: f64 = ((s.dn[114][15] * eq56_e2423_q) + (s.v[114] * s.dn[496][15]));
        let eq56_e2424_q_d_n16: f64 = ((s.dn[114][16] * eq56_e2423_q) + (s.v[114] * s.dn[496][16]));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n1, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n12, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_d_n15, eq56_e2424_d_n16, eq56_e2424_q, eq56_e2424_q_d_n0, eq56_e2424_q_d_n1, eq56_e2424_q_d_n2, eq56_e2424_q_d_n3, eq56_e2424_q_d_n4, eq56_e2424_q_d_n5, eq56_e2424_q_d_n6, eq56_e2424_q_d_n7, eq56_e2424_q_d_n8, eq56_e2424_q_d_n9, eq56_e2424_q_d_n10, eq56_e2424_q_d_n11, eq56_e2424_q_d_n12, eq56_e2424_q_d_n13, eq56_e2424_q_d_n14, eq56_e2424_q_d_n15, eq56_e2424_q_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_reactive_node_derivatives: [f64; 17] = [eq56_e2426_q_d_n0, eq56_e2426_q_d_n1, eq56_e2426_q_d_n2, eq56_e2426_q_d_n3, eq56_e2426_q_d_n4, eq56_e2426_q_d_n5, eq56_e2426_q_d_n6, eq56_e2426_q_d_n7, eq56_e2426_q_d_n8, eq56_e2426_q_d_n9, eq56_e2426_q_d_n10, eq56_e2426_q_d_n11, eq56_e2426_q_d_n12, eq56_e2426_q_d_n13, eq56_e2426_q_d_n14, eq56_e2426_q_d_n15, eq56_e2426_q_d_n16];
        let eq56_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            &nodes,
            &eq56_reactive_node_derivatives,
            &branches,
            &eq56_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n1, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n12, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_d_n15, eq69_e2506_d_n16, eq69_e2506_q, eq69_e2506_q_d_n0, eq69_e2506_q_d_n1, eq69_e2506_q_d_n2, eq69_e2506_q_d_n3, eq69_e2506_q_d_n4, eq69_e2506_q_d_n5, eq69_e2506_q_d_n6, eq69_e2506_q_d_n7, eq69_e2506_q_d_n8, eq69_e2506_q_d_n9, eq69_e2506_q_d_n10, eq69_e2506_q_d_n11, eq69_e2506_q_d_n12, eq69_e2506_q_d_n13, eq69_e2506_q_d_n14, eq69_e2506_q_d_n15, eq69_e2506_q_d_n16,) = {
    if (s.v[1723] != 0.0) {
        let eq69_e2503: f64 = (s.v[138] - s.v[140]);
        let eq69_e2503_d_n0: f64 = (s.dn[138][0] - s.dn[140][0]);
        let eq69_e2503_d_n1: f64 = (s.dn[138][1] - s.dn[140][1]);
        let eq69_e2503_d_n2: f64 = (s.dn[138][2] - s.dn[140][2]);
        let eq69_e2503_d_n3: f64 = (s.dn[138][3] - s.dn[140][3]);
        let eq69_e2503_d_n4: f64 = (s.dn[138][4] - s.dn[140][4]);
        let eq69_e2503_d_n5: f64 = (s.dn[138][5] - s.dn[140][5]);
        let eq69_e2503_d_n6: f64 = (s.dn[138][6] - s.dn[140][6]);
        let eq69_e2503_d_n7: f64 = (s.dn[138][7] - s.dn[140][7]);
        let eq69_e2503_d_n8: f64 = (s.dn[138][8] - s.dn[140][8]);
        let eq69_e2503_d_n9: f64 = (s.dn[138][9] - s.dn[140][9]);
        let eq69_e2503_d_n10: f64 = (s.dn[138][10] - s.dn[140][10]);
        let eq69_e2503_d_n11: f64 = (s.dn[138][11] - s.dn[140][11]);
        let eq69_e2503_d_n12: f64 = (s.dn[138][12] - s.dn[140][12]);
        let eq69_e2503_d_n13: f64 = (s.dn[138][13] - s.dn[140][13]);
        let eq69_e2503_d_n14: f64 = (s.dn[138][14] - s.dn[140][14]);
        let eq69_e2503_d_n15: f64 = (s.dn[138][15] - s.dn[140][15]);
        let eq69_e2503_d_n16: f64 = (s.dn[138][16] - s.dn[140][16]);
        let eq69_e2504_q: f64 = eq69_e2503;
        (eq69_e2503, eq69_e2503_d_n0, eq69_e2503_d_n1, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n12, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2503_d_n15, eq69_e2503_d_n16, eq69_e2504_q, eq69_e2503_d_n0, eq69_e2503_d_n1, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n12, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2503_d_n15, eq69_e2503_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 17] = [eq69_e2506_q_d_n0, eq69_e2506_q_d_n1, eq69_e2506_q_d_n2, eq69_e2506_q_d_n3, eq69_e2506_q_d_n4, eq69_e2506_q_d_n5, eq69_e2506_q_d_n6, eq69_e2506_q_d_n7, eq69_e2506_q_d_n8, eq69_e2506_q_d_n9, eq69_e2506_q_d_n10, eq69_e2506_q_d_n11, eq69_e2506_q_d_n12, eq69_e2506_q_d_n13, eq69_e2506_q_d_n14, eq69_e2506_q_d_n15, eq69_e2506_q_d_n16];
        let eq69_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            &nodes,
            &eq69_reactive_node_derivatives,
            &branches,
            &eq69_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_71_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq71_e2519, eq71_e2519_d_n15, eq71_e2519_q, eq71_e2519_q_d_n15,) = {
    if (s.v[1723] != 0.0) {
        let eq71_e2516_q: f64 = (nv15 - 0.0);
        let eq71_e2517: f64 = (1e-9 * (nv15 - 0.0));
        let eq71_e2517_d_n15: f64 = 1e-9;
        let eq71_e2517_q: f64 = (1e-9 * eq71_e2516_q);
        let eq71_e2517_q_d_n15: f64 = 1e-9;
        (eq71_e2517, eq71_e2517_d_n15, eq71_e2517_q, eq71_e2517_q_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[15]),
            None,
            &[
                GeneratedDerivative::node(nodes[15], self.multiplicity * (eq71_e2519_q_d_n15)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_96_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n1, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n12, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n15, eq96_e2717_d_n16, eq96_e2717_q, eq96_e2717_q_d_n0, eq96_e2717_q_d_n1, eq96_e2717_q_d_n2, eq96_e2717_q_d_n3, eq96_e2717_q_d_n4, eq96_e2717_q_d_n5, eq96_e2717_q_d_n6, eq96_e2717_q_d_n7, eq96_e2717_q_d_n8, eq96_e2717_q_d_n9, eq96_e2717_q_d_n10, eq96_e2717_q_d_n11, eq96_e2717_q_d_n12, eq96_e2717_q_d_n13, eq96_e2717_q_d_n14, eq96_e2717_q_d_n15, eq96_e2717_q_d_n16,) = {
    if (!(s.v[1731] != 0.0)) {
        let eq96_e2712: f64 = (0.7071 * s.v[632]);
        let eq96_e2712_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq96_e2712_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq96_e2712_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq96_e2712_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq96_e2712_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq96_e2712_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq96_e2712_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq96_e2712_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq96_e2712_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq96_e2712_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq96_e2712_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq96_e2712_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq96_e2712_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq96_e2712_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq96_e2712_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq96_e2712_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq96_e2712_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n1: f64 = (eq96_e2712_d_n1 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n12: f64 = (eq96_e2712_d_n12 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2714_d_n15: f64 = (eq96_e2712_d_n15 * (nv16 - 0.0));
        let eq96_e2714_d_n16: f64 = ((eq96_e2712_d_n16 * (nv16 - 0.0)) + eq96_e2712);
        let eq96_e2715_q: f64 = eq96_e2714;
        (eq96_e2714, eq96_e2714_d_n0, eq96_e2714_d_n1, eq96_e2714_d_n2, eq96_e2714_d_n3, eq96_e2714_d_n4, eq96_e2714_d_n5, eq96_e2714_d_n6, eq96_e2714_d_n7, eq96_e2714_d_n8, eq96_e2714_d_n9, eq96_e2714_d_n10, eq96_e2714_d_n11, eq96_e2714_d_n12, eq96_e2714_d_n13, eq96_e2714_d_n14, eq96_e2714_d_n15, eq96_e2714_d_n16, eq96_e2715_q, eq96_e2714_d_n0, eq96_e2714_d_n1, eq96_e2714_d_n2, eq96_e2714_d_n3, eq96_e2714_d_n4, eq96_e2714_d_n5, eq96_e2714_d_n6, eq96_e2714_d_n7, eq96_e2714_d_n8, eq96_e2714_d_n9, eq96_e2714_d_n10, eq96_e2714_d_n11, eq96_e2714_d_n12, eq96_e2714_d_n13, eq96_e2714_d_n14, eq96_e2714_d_n15, eq96_e2714_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_reactive_node_derivatives: [f64; 17] = [eq96_e2717_q_d_n0, eq96_e2717_q_d_n1, eq96_e2717_q_d_n2, eq96_e2717_q_d_n3, eq96_e2717_q_d_n4, eq96_e2717_q_d_n5, eq96_e2717_q_d_n6, eq96_e2717_q_d_n7, eq96_e2717_q_d_n8, eq96_e2717_q_d_n9, eq96_e2717_q_d_n10, eq96_e2717_q_d_n11, eq96_e2717_q_d_n12, eq96_e2717_q_d_n13, eq96_e2717_q_d_n14, eq96_e2717_q_d_n15, eq96_e2717_q_d_n16];
        let eq96_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            &nodes,
            &eq96_reactive_node_derivatives,
            &branches,
            &eq96_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_97_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n1, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n12, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n15, eq97_e2727_d_n16, eq97_e2727_q, eq97_e2727_q_d_n0, eq97_e2727_q_d_n1, eq97_e2727_q_d_n2, eq97_e2727_q_d_n3, eq97_e2727_q_d_n4, eq97_e2727_q_d_n5, eq97_e2727_q_d_n6, eq97_e2727_q_d_n7, eq97_e2727_q_d_n8, eq97_e2727_q_d_n9, eq97_e2727_q_d_n10, eq97_e2727_q_d_n11, eq97_e2727_q_d_n12, eq97_e2727_q_d_n13, eq97_e2727_q_d_n14, eq97_e2727_q_d_n15, eq97_e2727_q_d_n16,) = {
    if (!(s.v[1731] != 0.0)) {
        let eq97_e2722: f64 = (0.7071 * s.v[632]);
        let eq97_e2722_d_n0: f64 = (0.7071 * s.dn[632][0]);
        let eq97_e2722_d_n1: f64 = (0.7071 * s.dn[632][1]);
        let eq97_e2722_d_n2: f64 = (0.7071 * s.dn[632][2]);
        let eq97_e2722_d_n3: f64 = (0.7071 * s.dn[632][3]);
        let eq97_e2722_d_n4: f64 = (0.7071 * s.dn[632][4]);
        let eq97_e2722_d_n5: f64 = (0.7071 * s.dn[632][5]);
        let eq97_e2722_d_n6: f64 = (0.7071 * s.dn[632][6]);
        let eq97_e2722_d_n7: f64 = (0.7071 * s.dn[632][7]);
        let eq97_e2722_d_n8: f64 = (0.7071 * s.dn[632][8]);
        let eq97_e2722_d_n9: f64 = (0.7071 * s.dn[632][9]);
        let eq97_e2722_d_n10: f64 = (0.7071 * s.dn[632][10]);
        let eq97_e2722_d_n11: f64 = (0.7071 * s.dn[632][11]);
        let eq97_e2722_d_n12: f64 = (0.7071 * s.dn[632][12]);
        let eq97_e2722_d_n13: f64 = (0.7071 * s.dn[632][13]);
        let eq97_e2722_d_n14: f64 = (0.7071 * s.dn[632][14]);
        let eq97_e2722_d_n15: f64 = (0.7071 * s.dn[632][15]);
        let eq97_e2722_d_n16: f64 = (0.7071 * s.dn[632][16]);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n1: f64 = (eq97_e2722_d_n1 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n12: f64 = (eq97_e2722_d_n12 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2724_d_n15: f64 = (eq97_e2722_d_n15 * (nv16 - 0.0));
        let eq97_e2724_d_n16: f64 = ((eq97_e2722_d_n16 * (nv16 - 0.0)) + eq97_e2722);
        let eq97_e2725_q: f64 = eq97_e2724;
        (eq97_e2724, eq97_e2724_d_n0, eq97_e2724_d_n1, eq97_e2724_d_n2, eq97_e2724_d_n3, eq97_e2724_d_n4, eq97_e2724_d_n5, eq97_e2724_d_n6, eq97_e2724_d_n7, eq97_e2724_d_n8, eq97_e2724_d_n9, eq97_e2724_d_n10, eq97_e2724_d_n11, eq97_e2724_d_n12, eq97_e2724_d_n13, eq97_e2724_d_n14, eq97_e2724_d_n15, eq97_e2724_d_n16, eq97_e2725_q, eq97_e2724_d_n0, eq97_e2724_d_n1, eq97_e2724_d_n2, eq97_e2724_d_n3, eq97_e2724_d_n4, eq97_e2724_d_n5, eq97_e2724_d_n6, eq97_e2724_d_n7, eq97_e2724_d_n8, eq97_e2724_d_n9, eq97_e2724_d_n10, eq97_e2724_d_n11, eq97_e2724_d_n12, eq97_e2724_d_n13, eq97_e2724_d_n14, eq97_e2724_d_n15, eq97_e2724_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_reactive_node_derivatives: [f64; 17] = [eq97_e2727_q_d_n0, eq97_e2727_q_d_n1, eq97_e2727_q_d_n2, eq97_e2727_q_d_n3, eq97_e2727_q_d_n4, eq97_e2727_q_d_n5, eq97_e2727_q_d_n6, eq97_e2727_q_d_n7, eq97_e2727_q_d_n8, eq97_e2727_q_d_n9, eq97_e2727_q_d_n10, eq97_e2727_q_d_n11, eq97_e2727_q_d_n12, eq97_e2727_q_d_n13, eq97_e2727_q_d_n14, eq97_e2727_q_d_n15, eq97_e2727_q_d_n16];
        let eq97_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            &nodes,
            &eq97_reactive_node_derivatives,
            &branches,
            &eq97_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_111_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n1, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n12, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_d_n15, eq111_e2904_d_n16, eq111_e2904_q, eq111_e2904_q_d_n0, eq111_e2904_q_d_n1, eq111_e2904_q_d_n2, eq111_e2904_q_d_n3, eq111_e2904_q_d_n4, eq111_e2904_q_d_n5, eq111_e2904_q_d_n6, eq111_e2904_q_d_n7, eq111_e2904_q_d_n8, eq111_e2904_q_d_n9, eq111_e2904_q_d_n10, eq111_e2904_q_d_n11, eq111_e2904_q_d_n12, eq111_e2904_q_d_n13, eq111_e2904_q_d_n14, eq111_e2904_q_d_n15, eq111_e2904_q_d_n16,) = {
    if (s.v[1736] != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * s.v[634]);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * s.dn[634][0]);
        let eq111_e2901_d_n1: f64 = ((nv4 - 0.0) * s.dn[634][1]);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * s.dn[634][2]);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * s.dn[634][3]);
        let eq111_e2901_d_n4: f64 = (s.v[634] + ((nv4 - 0.0) * s.dn[634][4]));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * s.dn[634][5]);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * s.dn[634][6]);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * s.dn[634][7]);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * s.dn[634][8]);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * s.dn[634][9]);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * s.dn[634][10]);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * s.dn[634][11]);
        let eq111_e2901_d_n12: f64 = ((nv4 - 0.0) * s.dn[634][12]);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * s.dn[634][13]);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * s.dn[634][14]);
        let eq111_e2901_d_n15: f64 = ((nv4 - 0.0) * s.dn[634][15]);
        let eq111_e2901_d_n16: f64 = ((nv4 - 0.0) * s.dn[634][16]);
        let eq111_e2902_q: f64 = eq111_e2901;
        (eq111_e2901, eq111_e2901_d_n0, eq111_e2901_d_n1, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n12, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2901_d_n15, eq111_e2901_d_n16, eq111_e2902_q, eq111_e2901_d_n0, eq111_e2901_d_n1, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n12, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2901_d_n15, eq111_e2901_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 17] = [eq111_e2904_q_d_n0, eq111_e2904_q_d_n1, eq111_e2904_q_d_n2, eq111_e2904_q_d_n3, eq111_e2904_q_d_n4, eq111_e2904_q_d_n5, eq111_e2904_q_d_n6, eq111_e2904_q_d_n7, eq111_e2904_q_d_n8, eq111_e2904_q_d_n9, eq111_e2904_q_d_n10, eq111_e2904_q_d_n11, eq111_e2904_q_d_n12, eq111_e2904_q_d_n13, eq111_e2904_q_d_n14, eq111_e2904_q_d_n15, eq111_e2904_q_d_n16];
        let eq111_reactive_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            &nodes,
            &eq111_reactive_node_derivatives,
            &branches,
            &eq111_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

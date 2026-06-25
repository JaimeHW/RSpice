#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_16_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq16_e252_q: f64 = eq16_e251;
        let eq16_reactive_node_derivatives: [f64; 15] = [eq16_e251_d_n0, eq16_e251_d_n1, eq16_e251_d_n2, eq16_e251_d_n3, eq16_e251_d_n4, eq16_e251_d_n5, eq16_e251_d_n6, eq16_e251_d_n7, eq16_e251_d_n8, eq16_e251_d_n9, eq16_e251_d_n10, eq16_e251_d_n11, eq16_e251_d_n12, eq16_e251_d_n13, eq16_e251_d_n14];
        let eq16_reactive_branch_derivatives: [f64; 6] = [eq16_e251_d_b0, eq16_e251_d_b1, eq16_e251_d_b2, eq16_e251_d_b3, eq16_e251_d_b4, eq16_e251_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq16_reactive_node_derivatives,
            &branches,
            &eq16_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_17_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq17_e256_q: f64 = eq17_e255;
        let eq17_reactive_node_derivatives: [f64; 15] = [eq17_e255_d_n0, eq17_e255_d_n1, eq17_e255_d_n2, eq17_e255_d_n3, eq17_e255_d_n4, eq17_e255_d_n5, eq17_e255_d_n6, eq17_e255_d_n7, eq17_e255_d_n8, eq17_e255_d_n9, eq17_e255_d_n10, eq17_e255_d_n11, eq17_e255_d_n12, eq17_e255_d_n13, eq17_e255_d_n14];
        let eq17_reactive_branch_derivatives: [f64; 6] = [eq17_e255_d_b0, eq17_e255_d_b1, eq17_e255_d_b2, eq17_e255_d_b3, eq17_e255_d_b4, eq17_e255_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &nodes,
            &eq17_reactive_node_derivatives,
            &branches,
            &eq17_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_18_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq18_e260_q: f64 = eq18_e259;
        let eq18_reactive_node_derivatives: [f64; 15] = [eq18_e259_d_n0, eq18_e259_d_n1, eq18_e259_d_n2, eq18_e259_d_n3, eq18_e259_d_n4, eq18_e259_d_n5, eq18_e259_d_n6, eq18_e259_d_n7, eq18_e259_d_n8, eq18_e259_d_n9, eq18_e259_d_n10, eq18_e259_d_n11, eq18_e259_d_n12, eq18_e259_d_n13, eq18_e259_d_n14];
        let eq18_reactive_branch_derivatives: [f64; 6] = [eq18_e259_d_b0, eq18_e259_d_b1, eq18_e259_d_b2, eq18_e259_d_b3, eq18_e259_d_b4, eq18_e259_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            &nodes,
            &eq18_reactive_node_derivatives,
            &branches,
            &eq18_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_25_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq25_e297_q: f64 = eq25_e296;
        stamper.stamp_current_reactive(
            Some(nodes[7]),
            Some(nodes[2]),
            &[
                GeneratedDerivative::node(nodes[2], self.multiplicity * (eq25_e296_d_n2)),
                GeneratedDerivative::node(nodes[7], self.multiplicity * (eq25_e296_d_n7)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_26_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq26_e301_q: f64 = eq26_e300;
        stamper.stamp_current_reactive(
            Some(nodes[1]),
            Some(nodes[2]),
            &[
                GeneratedDerivative::node(nodes[1], self.multiplicity * (eq26_e300_d_n1)),
                GeneratedDerivative::node(nodes[2], self.multiplicity * (eq26_e300_d_n2)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_27_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
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
        let eq27_e305_q: f64 = eq27_e304;
        stamper.stamp_current_reactive(
            Some(nodes[0]),
            Some(nodes[2]),
            &[
                GeneratedDerivative::node(nodes[0], self.multiplicity * (eq27_e304_d_n0)),
                GeneratedDerivative::node(nodes[2], self.multiplicity * (eq27_e304_d_n2)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq33_e343: f64 = (p.p148 * s.v[196]);
        let eq33_e343_d_n0: f64 = (p.p148 * s.dn[196][0]);
        let eq33_e343_d_n1: f64 = (p.p148 * s.dn[196][1]);
        let eq33_e343_d_n2: f64 = (p.p148 * s.dn[196][2]);
        let eq33_e343_d_n3: f64 = (p.p148 * s.dn[196][3]);
        let eq33_e343_d_n4: f64 = (p.p148 * s.dn[196][4]);
        let eq33_e343_d_n5: f64 = (p.p148 * s.dn[196][5]);
        let eq33_e343_d_n6: f64 = (p.p148 * s.dn[196][6]);
        let eq33_e343_d_n7: f64 = (p.p148 * s.dn[196][7]);
        let eq33_e343_d_n8: f64 = (p.p148 * s.dn[196][8]);
        let eq33_e343_d_n9: f64 = (p.p148 * s.dn[196][9]);
        let eq33_e343_d_n10: f64 = (p.p148 * s.dn[196][10]);
        let eq33_e343_d_n11: f64 = (p.p148 * s.dn[196][11]);
        let eq33_e343_d_n12: f64 = (p.p148 * s.dn[196][12]);
        let eq33_e343_d_n13: f64 = (p.p148 * s.dn[196][13]);
        let eq33_e343_d_n14: f64 = (p.p148 * s.dn[196][14]);
        let eq33_e343_d_b0: f64 = (p.p148 * s.db[196][0]);
        let eq33_e343_d_b1: f64 = (p.p148 * s.db[196][1]);
        let eq33_e343_d_b2: f64 = (p.p148 * s.db[196][2]);
        let eq33_e343_d_b3: f64 = (p.p148 * s.db[196][3]);
        let eq33_e343_d_b4: f64 = (p.p148 * s.db[196][4]);
        let eq33_e343_d_b5: f64 = (p.p148 * s.db[196][5]);
        let eq33_e344_q: f64 = eq33_e343;
        let eq33_reactive_node_derivatives: [f64; 15] = [eq33_e343_d_n0, eq33_e343_d_n1, eq33_e343_d_n2, eq33_e343_d_n3, eq33_e343_d_n4, eq33_e343_d_n5, eq33_e343_d_n6, eq33_e343_d_n7, eq33_e343_d_n8, eq33_e343_d_n9, eq33_e343_d_n10, eq33_e343_d_n11, eq33_e343_d_n12, eq33_e343_d_n13, eq33_e343_d_n14];
        let eq33_reactive_branch_derivatives: [f64; 6] = [eq33_e343_d_b0, eq33_e343_d_b1, eq33_e343_d_b2, eq33_e343_d_b3, eq33_e343_d_b4, eq33_e343_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            &nodes,
            &eq33_reactive_node_derivatives,
            &branches,
            &eq33_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq34_e347: f64 = (p.p148 * s.v[197]);
        let eq34_e347_d_n0: f64 = (p.p148 * s.dn[197][0]);
        let eq34_e347_d_n1: f64 = (p.p148 * s.dn[197][1]);
        let eq34_e347_d_n2: f64 = (p.p148 * s.dn[197][2]);
        let eq34_e347_d_n3: f64 = (p.p148 * s.dn[197][3]);
        let eq34_e347_d_n4: f64 = (p.p148 * s.dn[197][4]);
        let eq34_e347_d_n5: f64 = (p.p148 * s.dn[197][5]);
        let eq34_e347_d_n6: f64 = (p.p148 * s.dn[197][6]);
        let eq34_e347_d_n7: f64 = (p.p148 * s.dn[197][7]);
        let eq34_e347_d_n8: f64 = (p.p148 * s.dn[197][8]);
        let eq34_e347_d_n9: f64 = (p.p148 * s.dn[197][9]);
        let eq34_e347_d_n10: f64 = (p.p148 * s.dn[197][10]);
        let eq34_e347_d_n11: f64 = (p.p148 * s.dn[197][11]);
        let eq34_e347_d_n12: f64 = (p.p148 * s.dn[197][12]);
        let eq34_e347_d_n13: f64 = (p.p148 * s.dn[197][13]);
        let eq34_e347_d_n14: f64 = (p.p148 * s.dn[197][14]);
        let eq34_e347_d_b0: f64 = (p.p148 * s.db[197][0]);
        let eq34_e347_d_b1: f64 = (p.p148 * s.db[197][1]);
        let eq34_e347_d_b2: f64 = (p.p148 * s.db[197][2]);
        let eq34_e347_d_b3: f64 = (p.p148 * s.db[197][3]);
        let eq34_e347_d_b4: f64 = (p.p148 * s.db[197][4]);
        let eq34_e347_d_b5: f64 = (p.p148 * s.db[197][5]);
        let eq34_e348_q: f64 = eq34_e347;
        let eq34_reactive_node_derivatives: [f64; 15] = [eq34_e347_d_n0, eq34_e347_d_n1, eq34_e347_d_n2, eq34_e347_d_n3, eq34_e347_d_n4, eq34_e347_d_n5, eq34_e347_d_n6, eq34_e347_d_n7, eq34_e347_d_n8, eq34_e347_d_n9, eq34_e347_d_n10, eq34_e347_d_n11, eq34_e347_d_n12, eq34_e347_d_n13, eq34_e347_d_n14];
        let eq34_reactive_branch_derivatives: [f64; 6] = [eq34_e347_d_b0, eq34_e347_d_b1, eq34_e347_d_b2, eq34_e347_d_b3, eq34_e347_d_b4, eq34_e347_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[0]),
            &nodes,
            &eq34_reactive_node_derivatives,
            &branches,
            &eq34_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq36_e363, eq36_e363_d_n3, eq36_e363_d_n9, eq36_e363_q, eq36_e363_q_d_n3, eq36_e363_q_d_n9,) = {
    if ((s.v[517] != 0.0) && (s.v[518] != 0.0)) {
        let eq36_e360: f64 = (p.p103 * (nv9 - nv3));
        let eq36_e360_d_n3: f64 = (-p.p103);
        let eq36_e360_d_n9: f64 = p.p103;
        let eq36_e361_q: f64 = eq36_e360;
        (eq36_e360, eq36_e360_d_n3, eq36_e360_d_n9, eq36_e361_q, eq36_e360_d_n3, eq36_e360_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[9]),
            Some(nodes[3]),
            &[
                GeneratedDerivative::node(nodes[3], self.multiplicity * (eq36_e363_q_d_n3)),
                GeneratedDerivative::node(nodes[9], self.multiplicity * (eq36_e363_q_d_n9)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_39_block_0(
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
        let (eq39_e385, eq39_e385_d_n4, eq39_e385_q, eq39_e385_q_d_n4,) = {
    if ((s.v[519] != 0.0) && (s.v[520] != 0.0)) {
        let eq39_e382: f64 = (p.p145 * (nv4 - 0.0));
        let eq39_e382_d_n4: f64 = p.p145;
        let eq39_e383_q: f64 = eq39_e382;
        (eq39_e382, eq39_e382_d_n4, eq39_e383_q, eq39_e382_d_n4,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[4]),
            None,
            &[
                GeneratedDerivative::node(nodes[4], self.multiplicity * (eq39_e385_q_d_n4)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq42_e393_q: f64 = s.v[239];
        let eq42_reactive_node_derivatives: [f64; 15] = [s.dn[239][0], s.dn[239][1], s.dn[239][2], s.dn[239][3], s.dn[239][4], s.dn[239][5], s.dn[239][6], s.dn[239][7], s.dn[239][8], s.dn[239][9], s.dn[239][10], s.dn[239][11], s.dn[239][12], s.dn[239][13], s.dn[239][14]];
        let eq42_reactive_branch_derivatives: [f64; 6] = [s.db[239][0], s.db[239][1], s.db[239][2], s.db[239][3], s.db[239][4], s.db[239][5]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            &nodes,
            &eq42_reactive_node_derivatives,
            &branches,
            &eq42_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_e396_q: f64 = s.v[240];
        let eq44_reactive_node_derivatives: [f64; 15] = [s.dn[240][0], s.dn[240][1], s.dn[240][2], s.dn[240][3], s.dn[240][4], s.dn[240][5], s.dn[240][6], s.dn[240][7], s.dn[240][8], s.dn[240][9], s.dn[240][10], s.dn[240][11], s.dn[240][12], s.dn[240][13], s.dn[240][14]];
        let eq44_reactive_branch_derivatives: [f64; 6] = [s.db[240][0], s.db[240][1], s.db[240][2], s.db[240][3], s.db[240][4], s.db[240][5]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            None,
            &nodes,
            &eq44_reactive_node_derivatives,
            &branches,
            &eq44_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_e399_q: f64 = s.v[236];
        let eq46_reactive_node_derivatives: [f64; 15] = [s.dn[236][0], s.dn[236][1], s.dn[236][2], s.dn[236][3], s.dn[236][4], s.dn[236][5], s.dn[236][6], s.dn[236][7], s.dn[236][8], s.dn[236][9], s.dn[236][10], s.dn[236][11], s.dn[236][12], s.dn[236][13], s.dn[236][14]];
        let eq46_reactive_branch_derivatives: [f64; 6] = [s.db[236][0], s.db[236][1], s.db[236][2], s.db[236][3], s.db[236][4], s.db[236][5]];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            None,
            &nodes,
            &eq46_reactive_node_derivatives,
            &branches,
            &eq46_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_65_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5, eq65_e534_q, eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14, eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5,) = {
    if (s.v[533] != 0.0) {
        let eq65_e527: f64 = (s.v[537] / s.v[535]);
        let eq65_e527_d_n0: f64 = (((s.dn[537][0] * s.v[535]) - (s.v[537] * s.dn[535][0])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n1: f64 = (((s.dn[537][1] * s.v[535]) - (s.v[537] * s.dn[535][1])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n2: f64 = (((s.dn[537][2] * s.v[535]) - (s.v[537] * s.dn[535][2])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n3: f64 = (((s.dn[537][3] * s.v[535]) - (s.v[537] * s.dn[535][3])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n4: f64 = (((s.dn[537][4] * s.v[535]) - (s.v[537] * s.dn[535][4])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n5: f64 = (((s.dn[537][5] * s.v[535]) - (s.v[537] * s.dn[535][5])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n6: f64 = (((s.dn[537][6] * s.v[535]) - (s.v[537] * s.dn[535][6])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n7: f64 = (((s.dn[537][7] * s.v[535]) - (s.v[537] * s.dn[535][7])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n8: f64 = (((s.dn[537][8] * s.v[535]) - (s.v[537] * s.dn[535][8])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n9: f64 = (((s.dn[537][9] * s.v[535]) - (s.v[537] * s.dn[535][9])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n10: f64 = (((s.dn[537][10] * s.v[535]) - (s.v[537] * s.dn[535][10])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n11: f64 = (((s.dn[537][11] * s.v[535]) - (s.v[537] * s.dn[535][11])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n12: f64 = (((s.dn[537][12] * s.v[535]) - (s.v[537] * s.dn[535][12])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n13: f64 = (((s.dn[537][13] * s.v[535]) - (s.v[537] * s.dn[535][13])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_n14: f64 = (((s.dn[537][14] * s.v[535]) - (s.v[537] * s.dn[535][14])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b0: f64 = (((s.db[537][0] * s.v[535]) - (s.v[537] * s.db[535][0])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b1: f64 = (((s.db[537][1] * s.v[535]) - (s.v[537] * s.db[535][1])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b2: f64 = (((s.db[537][2] * s.v[535]) - (s.v[537] * s.db[535][2])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b3: f64 = (((s.db[537][3] * s.v[535]) - (s.v[537] * s.db[535][3])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b4: f64 = (((s.db[537][4] * s.v[535]) - (s.v[537] * s.db[535][4])) / (s.v[535] * s.v[535]));
        let eq65_e527_d_b5: f64 = (((s.db[537][5] * s.v[535]) - (s.v[537] * s.db[535][5])) / (s.v[535] * s.v[535]));
        let eq65_e530: f64 = (s.v[535] * (nv13 - 0.0));
        let eq65_e530_d_n0: f64 = (s.dn[535][0] * (nv13 - 0.0));
        let eq65_e530_d_n1: f64 = (s.dn[535][1] * (nv13 - 0.0));
        let eq65_e530_d_n2: f64 = (s.dn[535][2] * (nv13 - 0.0));
        let eq65_e530_d_n3: f64 = (s.dn[535][3] * (nv13 - 0.0));
        let eq65_e530_d_n4: f64 = (s.dn[535][4] * (nv13 - 0.0));
        let eq65_e530_d_n5: f64 = (s.dn[535][5] * (nv13 - 0.0));
        let eq65_e530_d_n6: f64 = (s.dn[535][6] * (nv13 - 0.0));
        let eq65_e530_d_n7: f64 = (s.dn[535][7] * (nv13 - 0.0));
        let eq65_e530_d_n8: f64 = (s.dn[535][8] * (nv13 - 0.0));
        let eq65_e530_d_n9: f64 = (s.dn[535][9] * (nv13 - 0.0));
        let eq65_e530_d_n10: f64 = (s.dn[535][10] * (nv13 - 0.0));
        let eq65_e530_d_n11: f64 = (s.dn[535][11] * (nv13 - 0.0));
        let eq65_e530_d_n12: f64 = (s.dn[535][12] * (nv13 - 0.0));
        let eq65_e530_d_n13: f64 = ((s.dn[535][13] * (nv13 - 0.0)) + s.v[535]);
        let eq65_e530_d_n14: f64 = (s.dn[535][14] * (nv13 - 0.0));
        let eq65_e530_d_b0: f64 = (s.db[535][0] * (nv13 - 0.0));
        let eq65_e530_d_b1: f64 = (s.db[535][1] * (nv13 - 0.0));
        let eq65_e530_d_b2: f64 = (s.db[535][2] * (nv13 - 0.0));
        let eq65_e530_d_b3: f64 = (s.db[535][3] * (nv13 - 0.0));
        let eq65_e530_d_b4: f64 = (s.db[535][4] * (nv13 - 0.0));
        let eq65_e530_d_b5: f64 = (s.db[535][5] * (nv13 - 0.0));
        let eq65_e531_q: f64 = eq65_e530;
        let eq65_e532: f64 = (eq65_e527 * eq65_e530);
        let eq65_e532_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e530) + (eq65_e527 * eq65_e530_d_n0));
        let eq65_e532_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e530) + (eq65_e527 * eq65_e530_d_n1));
        let eq65_e532_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e530) + (eq65_e527 * eq65_e530_d_n2));
        let eq65_e532_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e530) + (eq65_e527 * eq65_e530_d_n3));
        let eq65_e532_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e530) + (eq65_e527 * eq65_e530_d_n4));
        let eq65_e532_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e530) + (eq65_e527 * eq65_e530_d_n5));
        let eq65_e532_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e530) + (eq65_e527 * eq65_e530_d_n6));
        let eq65_e532_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e530) + (eq65_e527 * eq65_e530_d_n7));
        let eq65_e532_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e530) + (eq65_e527 * eq65_e530_d_n8));
        let eq65_e532_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e530) + (eq65_e527 * eq65_e530_d_n9));
        let eq65_e532_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e530) + (eq65_e527 * eq65_e530_d_n10));
        let eq65_e532_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e530) + (eq65_e527 * eq65_e530_d_n11));
        let eq65_e532_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e530) + (eq65_e527 * eq65_e530_d_n12));
        let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e530) + (eq65_e527 * eq65_e530_d_n13));
        let eq65_e532_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e530) + (eq65_e527 * eq65_e530_d_n14));
        let eq65_e532_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e530) + (eq65_e527 * eq65_e530_d_b0));
        let eq65_e532_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e530) + (eq65_e527 * eq65_e530_d_b1));
        let eq65_e532_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e530) + (eq65_e527 * eq65_e530_d_b2));
        let eq65_e532_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e530) + (eq65_e527 * eq65_e530_d_b3));
        let eq65_e532_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e530) + (eq65_e527 * eq65_e530_d_b4));
        let eq65_e532_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e530) + (eq65_e527 * eq65_e530_d_b5));
        let eq65_e532_q: f64 = (eq65_e527 * eq65_e531_q);
        let eq65_e532_q_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n0));
        let eq65_e532_q_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n1));
        let eq65_e532_q_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n2));
        let eq65_e532_q_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n3));
        let eq65_e532_q_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n4));
        let eq65_e532_q_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n5));
        let eq65_e532_q_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n6));
        let eq65_e532_q_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n7));
        let eq65_e532_q_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n8));
        let eq65_e532_q_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n9));
        let eq65_e532_q_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n10));
        let eq65_e532_q_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n11));
        let eq65_e532_q_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n12));
        let eq65_e532_q_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n13));
        let eq65_e532_q_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_n14));
        let eq65_e532_q_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b0));
        let eq65_e532_q_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b1));
        let eq65_e532_q_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b2));
        let eq65_e532_q_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b3));
        let eq65_e532_q_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b4));
        let eq65_e532_q_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e531_q) + (eq65_e527 * eq65_e530_d_b5));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n2, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n10, eq65_e532_d_n11, eq65_e532_d_n12, eq65_e532_d_n13, eq65_e532_d_n14, eq65_e532_d_b0, eq65_e532_d_b1, eq65_e532_d_b2, eq65_e532_d_b3, eq65_e532_d_b4, eq65_e532_d_b5, eq65_e532_q, eq65_e532_q_d_n0, eq65_e532_q_d_n1, eq65_e532_q_d_n2, eq65_e532_q_d_n3, eq65_e532_q_d_n4, eq65_e532_q_d_n5, eq65_e532_q_d_n6, eq65_e532_q_d_n7, eq65_e532_q_d_n8, eq65_e532_q_d_n9, eq65_e532_q_d_n10, eq65_e532_q_d_n11, eq65_e532_q_d_n12, eq65_e532_q_d_n13, eq65_e532_q_d_n14, eq65_e532_q_d_b0, eq65_e532_q_d_b1, eq65_e532_q_d_b2, eq65_e532_q_d_b3, eq65_e532_q_d_b4, eq65_e532_q_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_reactive_node_derivatives: [f64; 15] = [eq65_e534_q_d_n0, eq65_e534_q_d_n1, eq65_e534_q_d_n2, eq65_e534_q_d_n3, eq65_e534_q_d_n4, eq65_e534_q_d_n5, eq65_e534_q_d_n6, eq65_e534_q_d_n7, eq65_e534_q_d_n8, eq65_e534_q_d_n9, eq65_e534_q_d_n10, eq65_e534_q_d_n11, eq65_e534_q_d_n12, eq65_e534_q_d_n13, eq65_e534_q_d_n14];
        let eq65_reactive_branch_derivatives: [f64; 6] = [eq65_e534_q_d_b0, eq65_e534_q_d_b1, eq65_e534_q_d_b2, eq65_e534_q_d_b3, eq65_e534_q_d_b4, eq65_e534_q_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq65_reactive_node_derivatives,
            &branches,
            &eq65_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_66_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5, eq66_e545_q, eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n2, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, eq66_e545_q_d_n10, eq66_e545_q_d_n11, eq66_e545_q_d_n12, eq66_e545_q_d_n13, eq66_e545_q_d_n14, eq66_e545_q_d_b0, eq66_e545_q_d_b1, eq66_e545_q_d_b2, eq66_e545_q_d_b3, eq66_e545_q_d_b4, eq66_e545_q_d_b5,) = {
    if (s.v[533] != 0.0) {
        let eq66_e538: f64 = (s.v[536] / s.v[535]);
        let eq66_e538_d_n0: f64 = (((s.dn[536][0] * s.v[535]) - (s.v[536] * s.dn[535][0])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n1: f64 = (((s.dn[536][1] * s.v[535]) - (s.v[536] * s.dn[535][1])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n2: f64 = (((s.dn[536][2] * s.v[535]) - (s.v[536] * s.dn[535][2])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n3: f64 = (((s.dn[536][3] * s.v[535]) - (s.v[536] * s.dn[535][3])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n4: f64 = (((s.dn[536][4] * s.v[535]) - (s.v[536] * s.dn[535][4])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n5: f64 = (((s.dn[536][5] * s.v[535]) - (s.v[536] * s.dn[535][5])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n6: f64 = (((s.dn[536][6] * s.v[535]) - (s.v[536] * s.dn[535][6])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n7: f64 = (((s.dn[536][7] * s.v[535]) - (s.v[536] * s.dn[535][7])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n8: f64 = (((s.dn[536][8] * s.v[535]) - (s.v[536] * s.dn[535][8])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n9: f64 = (((s.dn[536][9] * s.v[535]) - (s.v[536] * s.dn[535][9])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n10: f64 = (((s.dn[536][10] * s.v[535]) - (s.v[536] * s.dn[535][10])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n11: f64 = (((s.dn[536][11] * s.v[535]) - (s.v[536] * s.dn[535][11])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n12: f64 = (((s.dn[536][12] * s.v[535]) - (s.v[536] * s.dn[535][12])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n13: f64 = (((s.dn[536][13] * s.v[535]) - (s.v[536] * s.dn[535][13])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_n14: f64 = (((s.dn[536][14] * s.v[535]) - (s.v[536] * s.dn[535][14])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b0: f64 = (((s.db[536][0] * s.v[535]) - (s.v[536] * s.db[535][0])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b1: f64 = (((s.db[536][1] * s.v[535]) - (s.v[536] * s.db[535][1])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b2: f64 = (((s.db[536][2] * s.v[535]) - (s.v[536] * s.db[535][2])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b3: f64 = (((s.db[536][3] * s.v[535]) - (s.v[536] * s.db[535][3])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b4: f64 = (((s.db[536][4] * s.v[535]) - (s.v[536] * s.db[535][4])) / (s.v[535] * s.v[535]));
        let eq66_e538_d_b5: f64 = (((s.db[536][5] * s.v[535]) - (s.v[536] * s.db[535][5])) / (s.v[535] * s.v[535]));
        let eq66_e541: f64 = (s.v[535] * (nv14 - 0.0));
        let eq66_e541_d_n0: f64 = (s.dn[535][0] * (nv14 - 0.0));
        let eq66_e541_d_n1: f64 = (s.dn[535][1] * (nv14 - 0.0));
        let eq66_e541_d_n2: f64 = (s.dn[535][2] * (nv14 - 0.0));
        let eq66_e541_d_n3: f64 = (s.dn[535][3] * (nv14 - 0.0));
        let eq66_e541_d_n4: f64 = (s.dn[535][4] * (nv14 - 0.0));
        let eq66_e541_d_n5: f64 = (s.dn[535][5] * (nv14 - 0.0));
        let eq66_e541_d_n6: f64 = (s.dn[535][6] * (nv14 - 0.0));
        let eq66_e541_d_n7: f64 = (s.dn[535][7] * (nv14 - 0.0));
        let eq66_e541_d_n8: f64 = (s.dn[535][8] * (nv14 - 0.0));
        let eq66_e541_d_n9: f64 = (s.dn[535][9] * (nv14 - 0.0));
        let eq66_e541_d_n10: f64 = (s.dn[535][10] * (nv14 - 0.0));
        let eq66_e541_d_n11: f64 = (s.dn[535][11] * (nv14 - 0.0));
        let eq66_e541_d_n12: f64 = (s.dn[535][12] * (nv14 - 0.0));
        let eq66_e541_d_n13: f64 = (s.dn[535][13] * (nv14 - 0.0));
        let eq66_e541_d_n14: f64 = ((s.dn[535][14] * (nv14 - 0.0)) + s.v[535]);
        let eq66_e541_d_b0: f64 = (s.db[535][0] * (nv14 - 0.0));
        let eq66_e541_d_b1: f64 = (s.db[535][1] * (nv14 - 0.0));
        let eq66_e541_d_b2: f64 = (s.db[535][2] * (nv14 - 0.0));
        let eq66_e541_d_b3: f64 = (s.db[535][3] * (nv14 - 0.0));
        let eq66_e541_d_b4: f64 = (s.db[535][4] * (nv14 - 0.0));
        let eq66_e541_d_b5: f64 = (s.db[535][5] * (nv14 - 0.0));
        let eq66_e542_q: f64 = eq66_e541;
        let eq66_e543: f64 = (eq66_e538 * eq66_e541);
        let eq66_e543_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e541) + (eq66_e538 * eq66_e541_d_n0));
        let eq66_e543_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e541) + (eq66_e538 * eq66_e541_d_n1));
        let eq66_e543_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e541) + (eq66_e538 * eq66_e541_d_n2));
        let eq66_e543_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e541) + (eq66_e538 * eq66_e541_d_n3));
        let eq66_e543_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e541) + (eq66_e538 * eq66_e541_d_n4));
        let eq66_e543_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e541) + (eq66_e538 * eq66_e541_d_n5));
        let eq66_e543_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e541) + (eq66_e538 * eq66_e541_d_n6));
        let eq66_e543_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e541) + (eq66_e538 * eq66_e541_d_n7));
        let eq66_e543_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e541) + (eq66_e538 * eq66_e541_d_n8));
        let eq66_e543_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e541) + (eq66_e538 * eq66_e541_d_n9));
        let eq66_e543_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e541) + (eq66_e538 * eq66_e541_d_n10));
        let eq66_e543_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e541) + (eq66_e538 * eq66_e541_d_n11));
        let eq66_e543_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e541) + (eq66_e538 * eq66_e541_d_n12));
        let eq66_e543_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e541) + (eq66_e538 * eq66_e541_d_n13));
        let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e541) + (eq66_e538 * eq66_e541_d_n14));
        let eq66_e543_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e541) + (eq66_e538 * eq66_e541_d_b0));
        let eq66_e543_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e541) + (eq66_e538 * eq66_e541_d_b1));
        let eq66_e543_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e541) + (eq66_e538 * eq66_e541_d_b2));
        let eq66_e543_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e541) + (eq66_e538 * eq66_e541_d_b3));
        let eq66_e543_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e541) + (eq66_e538 * eq66_e541_d_b4));
        let eq66_e543_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e541) + (eq66_e538 * eq66_e541_d_b5));
        let eq66_e543_q: f64 = (eq66_e538 * eq66_e542_q);
        let eq66_e543_q_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n0));
        let eq66_e543_q_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n1));
        let eq66_e543_q_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n2));
        let eq66_e543_q_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n3));
        let eq66_e543_q_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n4));
        let eq66_e543_q_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n5));
        let eq66_e543_q_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n6));
        let eq66_e543_q_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n7));
        let eq66_e543_q_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n8));
        let eq66_e543_q_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n9));
        let eq66_e543_q_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n10));
        let eq66_e543_q_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n11));
        let eq66_e543_q_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n12));
        let eq66_e543_q_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n13));
        let eq66_e543_q_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_n14));
        let eq66_e543_q_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b0));
        let eq66_e543_q_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b1));
        let eq66_e543_q_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b2));
        let eq66_e543_q_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b3));
        let eq66_e543_q_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b4));
        let eq66_e543_q_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e542_q) + (eq66_e538 * eq66_e541_d_b5));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n2, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n10, eq66_e543_d_n11, eq66_e543_d_n12, eq66_e543_d_n13, eq66_e543_d_n14, eq66_e543_d_b0, eq66_e543_d_b1, eq66_e543_d_b2, eq66_e543_d_b3, eq66_e543_d_b4, eq66_e543_d_b5, eq66_e543_q, eq66_e543_q_d_n0, eq66_e543_q_d_n1, eq66_e543_q_d_n2, eq66_e543_q_d_n3, eq66_e543_q_d_n4, eq66_e543_q_d_n5, eq66_e543_q_d_n6, eq66_e543_q_d_n7, eq66_e543_q_d_n8, eq66_e543_q_d_n9, eq66_e543_q_d_n10, eq66_e543_q_d_n11, eq66_e543_q_d_n12, eq66_e543_q_d_n13, eq66_e543_q_d_n14, eq66_e543_q_d_b0, eq66_e543_q_d_b1, eq66_e543_q_d_b2, eq66_e543_q_d_b3, eq66_e543_q_d_b4, eq66_e543_q_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_reactive_node_derivatives: [f64; 15] = [eq66_e545_q_d_n0, eq66_e545_q_d_n1, eq66_e545_q_d_n2, eq66_e545_q_d_n3, eq66_e545_q_d_n4, eq66_e545_q_d_n5, eq66_e545_q_d_n6, eq66_e545_q_d_n7, eq66_e545_q_d_n8, eq66_e545_q_d_n9, eq66_e545_q_d_n10, eq66_e545_q_d_n11, eq66_e545_q_d_n12, eq66_e545_q_d_n13, eq66_e545_q_d_n14];
        let eq66_reactive_branch_derivatives: [f64; 6] = [eq66_e545_q_d_b0, eq66_e545_q_d_b1, eq66_e545_q_d_b2, eq66_e545_q_d_b3, eq66_e545_q_d_b4, eq66_e545_q_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq66_reactive_node_derivatives,
            &branches,
            &eq66_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

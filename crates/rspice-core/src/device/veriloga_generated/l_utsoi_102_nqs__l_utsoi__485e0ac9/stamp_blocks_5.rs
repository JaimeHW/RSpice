#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq19_e613,) = {
    if (!(s.v[1770] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e613;
        stamper.stamp_potential(
            branches[2],
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq20_e623, eq20_e623_d_n0, eq20_e623_d_n1, eq20_e623_d_n2, eq20_e623_d_n3, eq20_e623_d_n4, eq20_e623_d_n5, eq20_e623_d_n6, eq20_e623_d_n7, eq20_e623_d_n8, eq20_e623_d_n9, eq20_e623_d_n10, eq20_e623_d_n11, eq20_e623_d_n12, eq20_e623_d_n13,) = {
    if (s.v[1771] != 0.0) {
        let eq20_e617: f64 = (p.p31 * s.v[13]);
        let eq20_e617_d_n0: f64 = (p.p31 * s.dn[13][0]);
        let eq20_e617_d_n1: f64 = (p.p31 * s.dn[13][1]);
        let eq20_e617_d_n2: f64 = (p.p31 * s.dn[13][2]);
        let eq20_e617_d_n3: f64 = (p.p31 * s.dn[13][3]);
        let eq20_e617_d_n4: f64 = (p.p31 * s.dn[13][4]);
        let eq20_e617_d_n5: f64 = (p.p31 * s.dn[13][5]);
        let eq20_e617_d_n6: f64 = (p.p31 * s.dn[13][6]);
        let eq20_e617_d_n7: f64 = (p.p31 * s.dn[13][7]);
        let eq20_e617_d_n8: f64 = (p.p31 * s.dn[13][8]);
        let eq20_e617_d_n9: f64 = (p.p31 * s.dn[13][9]);
        let eq20_e617_d_n10: f64 = (p.p31 * s.dn[13][10]);
        let eq20_e617_d_n11: f64 = (p.p31 * s.dn[13][11]);
        let eq20_e617_d_n12: f64 = (p.p31 * s.dn[13][12]);
        let eq20_e617_d_n13: f64 = (p.p31 * s.dn[13][13]);
        let eq20_e619: f64 = (eq20_e617 * s.v[327]);
        let eq20_e619_d_n0: f64 = ((eq20_e617_d_n0 * s.v[327]) + (eq20_e617 * s.dn[327][0]));
        let eq20_e619_d_n1: f64 = ((eq20_e617_d_n1 * s.v[327]) + (eq20_e617 * s.dn[327][1]));
        let eq20_e619_d_n2: f64 = ((eq20_e617_d_n2 * s.v[327]) + (eq20_e617 * s.dn[327][2]));
        let eq20_e619_d_n3: f64 = ((eq20_e617_d_n3 * s.v[327]) + (eq20_e617 * s.dn[327][3]));
        let eq20_e619_d_n4: f64 = ((eq20_e617_d_n4 * s.v[327]) + (eq20_e617 * s.dn[327][4]));
        let eq20_e619_d_n5: f64 = ((eq20_e617_d_n5 * s.v[327]) + (eq20_e617 * s.dn[327][5]));
        let eq20_e619_d_n6: f64 = ((eq20_e617_d_n6 * s.v[327]) + (eq20_e617 * s.dn[327][6]));
        let eq20_e619_d_n7: f64 = ((eq20_e617_d_n7 * s.v[327]) + (eq20_e617 * s.dn[327][7]));
        let eq20_e619_d_n8: f64 = ((eq20_e617_d_n8 * s.v[327]) + (eq20_e617 * s.dn[327][8]));
        let eq20_e619_d_n9: f64 = ((eq20_e617_d_n9 * s.v[327]) + (eq20_e617 * s.dn[327][9]));
        let eq20_e619_d_n10: f64 = ((eq20_e617_d_n10 * s.v[327]) + (eq20_e617 * s.dn[327][10]));
        let eq20_e619_d_n11: f64 = ((eq20_e617_d_n11 * s.v[327]) + (eq20_e617 * s.dn[327][11]));
        let eq20_e619_d_n12: f64 = ((eq20_e617_d_n12 * s.v[327]) + (eq20_e617 * s.dn[327][12]));
        let eq20_e619_d_n13: f64 = ((eq20_e617_d_n13 * s.v[327]) + (eq20_e617 * s.dn[327][13]));
        let eq20_e621: f64 = (eq20_e619 * (nv3 - nv8));
        let eq20_e621_d_n0: f64 = (eq20_e619_d_n0 * (nv3 - nv8));
        let eq20_e621_d_n1: f64 = (eq20_e619_d_n1 * (nv3 - nv8));
        let eq20_e621_d_n2: f64 = (eq20_e619_d_n2 * (nv3 - nv8));
        let eq20_e621_d_n3: f64 = ((eq20_e619_d_n3 * (nv3 - nv8)) + eq20_e619);
        let eq20_e621_d_n4: f64 = (eq20_e619_d_n4 * (nv3 - nv8));
        let eq20_e621_d_n5: f64 = (eq20_e619_d_n5 * (nv3 - nv8));
        let eq20_e621_d_n6: f64 = (eq20_e619_d_n6 * (nv3 - nv8));
        let eq20_e621_d_n7: f64 = (eq20_e619_d_n7 * (nv3 - nv8));
        let eq20_e621_d_n8: f64 = ((eq20_e619_d_n8 * (nv3 - nv8)) + (-eq20_e619));
        let eq20_e621_d_n9: f64 = (eq20_e619_d_n9 * (nv3 - nv8));
        let eq20_e621_d_n10: f64 = (eq20_e619_d_n10 * (nv3 - nv8));
        let eq20_e621_d_n11: f64 = (eq20_e619_d_n11 * (nv3 - nv8));
        let eq20_e621_d_n12: f64 = (eq20_e619_d_n12 * (nv3 - nv8));
        let eq20_e621_d_n13: f64 = (eq20_e619_d_n13 * (nv3 - nv8));
        (eq20_e621, eq20_e621_d_n0, eq20_e621_d_n1, eq20_e621_d_n2, eq20_e621_d_n3, eq20_e621_d_n4, eq20_e621_d_n5, eq20_e621_d_n6, eq20_e621_d_n7, eq20_e621_d_n8, eq20_e621_d_n9, eq20_e621_d_n10, eq20_e621_d_n11, eq20_e621_d_n12, eq20_e621_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e623;
        let eq20_node_derivatives: [f64; 14] = [eq20_e623_d_n0, eq20_e623_d_n1, eq20_e623_d_n2, eq20_e623_d_n3, eq20_e623_d_n4, eq20_e623_d_n5, eq20_e623_d_n6, eq20_e623_d_n7, eq20_e623_d_n8, eq20_e623_d_n9, eq20_e623_d_n10, eq20_e623_d_n11, eq20_e623_d_n12, eq20_e623_d_n13];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
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
        let (eq21_e633,) = {
    if (s.v[1771] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e633;
        stamper.stamp_current(
            Some(nodes[3]),
            Some(nodes[8]),
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
        let (eq22_e638,) = {
    if (!(s.v[1771] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e638;
        stamper.stamp_potential(
            branches[3],
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
        let eq23_e642: f64 = (s.v[1774] + s.v[1775]);
        let eq23_e642_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);
        let eq23_e642_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);
        let eq23_e642_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);
        let eq23_e642_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);
        let eq23_e642_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);
        let eq23_e642_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);
        let eq23_e642_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);
        let eq23_e642_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);
        let eq23_e642_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);
        let eq23_e642_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);
        let eq23_e642_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);
        let eq23_e642_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);
        let eq23_e642_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);
        let eq23_e642_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);
        let eq23_e643: f64 = (s.v[181] * eq23_e642);
        let eq23_e643_d_n0: f64 = ((s.dn[181][0] * eq23_e642) + (s.v[181] * eq23_e642_d_n0));
        let eq23_e643_d_n1: f64 = ((s.dn[181][1] * eq23_e642) + (s.v[181] * eq23_e642_d_n1));
        let eq23_e643_d_n2: f64 = ((s.dn[181][2] * eq23_e642) + (s.v[181] * eq23_e642_d_n2));
        let eq23_e643_d_n3: f64 = ((s.dn[181][3] * eq23_e642) + (s.v[181] * eq23_e642_d_n3));
        let eq23_e643_d_n4: f64 = ((s.dn[181][4] * eq23_e642) + (s.v[181] * eq23_e642_d_n4));
        let eq23_e643_d_n5: f64 = ((s.dn[181][5] * eq23_e642) + (s.v[181] * eq23_e642_d_n5));
        let eq23_e643_d_n6: f64 = ((s.dn[181][6] * eq23_e642) + (s.v[181] * eq23_e642_d_n6));
        let eq23_e643_d_n7: f64 = ((s.dn[181][7] * eq23_e642) + (s.v[181] * eq23_e642_d_n7));
        let eq23_e643_d_n8: f64 = ((s.dn[181][8] * eq23_e642) + (s.v[181] * eq23_e642_d_n8));
        let eq23_e643_d_n9: f64 = ((s.dn[181][9] * eq23_e642) + (s.v[181] * eq23_e642_d_n9));
        let eq23_e643_d_n10: f64 = ((s.dn[181][10] * eq23_e642) + (s.v[181] * eq23_e642_d_n10));
        let eq23_e643_d_n11: f64 = ((s.dn[181][11] * eq23_e642) + (s.v[181] * eq23_e642_d_n11));
        let eq23_e643_d_n12: f64 = ((s.dn[181][12] * eq23_e642) + (s.v[181] * eq23_e642_d_n12));
        let eq23_e643_d_n13: f64 = ((s.dn[181][13] * eq23_e642) + (s.v[181] * eq23_e642_d_n13));
        let eq23_e644: f64 = self.eval_ddt(0, eq23_e643);
        let eq23_e644_d_n0: f64 = self.ddt_jacobian(eq23_e643_d_n0);
        let eq23_e644_d_n1: f64 = self.ddt_jacobian(eq23_e643_d_n1);
        let eq23_e644_d_n2: f64 = self.ddt_jacobian(eq23_e643_d_n2);
        let eq23_e644_d_n3: f64 = self.ddt_jacobian(eq23_e643_d_n3);
        let eq23_e644_d_n4: f64 = self.ddt_jacobian(eq23_e643_d_n4);
        let eq23_e644_d_n5: f64 = self.ddt_jacobian(eq23_e643_d_n5);
        let eq23_e644_d_n6: f64 = self.ddt_jacobian(eq23_e643_d_n6);
        let eq23_e644_d_n7: f64 = self.ddt_jacobian(eq23_e643_d_n7);
        let eq23_e644_d_n8: f64 = self.ddt_jacobian(eq23_e643_d_n8);
        let eq23_e644_d_n9: f64 = self.ddt_jacobian(eq23_e643_d_n9);
        let eq23_e644_d_n10: f64 = self.ddt_jacobian(eq23_e643_d_n10);
        let eq23_e644_d_n11: f64 = self.ddt_jacobian(eq23_e643_d_n11);
        let eq23_e644_d_n12: f64 = self.ddt_jacobian(eq23_e643_d_n12);
        let eq23_e644_d_n13: f64 = self.ddt_jacobian(eq23_e643_d_n13);
        let eq23_value: f64 = eq23_e644;
        let eq23_node_derivatives: [f64; 14] = [eq23_e644_d_n0, eq23_e644_d_n1, eq23_e644_d_n2, eq23_e644_d_n3, eq23_e644_d_n4, eq23_e644_d_n5, eq23_e644_d_n6, eq23_e644_d_n7, eq23_e644_d_n8, eq23_e644_d_n9, eq23_e644_d_n10, eq23_e644_d_n11, eq23_e644_d_n12, eq23_e644_d_n13];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[13]),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq24_e647: f64 = (s.v[1773] * (nv10 - nv13));
        let eq24_e647_d_n0: f64 = (s.dn[1773][0] * (nv10 - nv13));
        let eq24_e647_d_n1: f64 = (s.dn[1773][1] * (nv10 - nv13));
        let eq24_e647_d_n2: f64 = (s.dn[1773][2] * (nv10 - nv13));
        let eq24_e647_d_n3: f64 = (s.dn[1773][3] * (nv10 - nv13));
        let eq24_e647_d_n4: f64 = (s.dn[1773][4] * (nv10 - nv13));
        let eq24_e647_d_n5: f64 = (s.dn[1773][5] * (nv10 - nv13));
        let eq24_e647_d_n6: f64 = (s.dn[1773][6] * (nv10 - nv13));
        let eq24_e647_d_n7: f64 = (s.dn[1773][7] * (nv10 - nv13));
        let eq24_e647_d_n8: f64 = (s.dn[1773][8] * (nv10 - nv13));
        let eq24_e647_d_n9: f64 = (s.dn[1773][9] * (nv10 - nv13));
        let eq24_e647_d_n10: f64 = ((s.dn[1773][10] * (nv10 - nv13)) + s.v[1773]);
        let eq24_e647_d_n11: f64 = (s.dn[1773][11] * (nv10 - nv13));
        let eq24_e647_d_n12: f64 = (s.dn[1773][12] * (nv10 - nv13));
        let eq24_e647_d_n13: f64 = ((s.dn[1773][13] * (nv10 - nv13)) + (-s.v[1773]));
        let eq24_value: f64 = eq24_e647;
        let eq24_node_derivatives: [f64; 14] = [eq24_e647_d_n0, eq24_e647_d_n1, eq24_e647_d_n2, eq24_e647_d_n3, eq24_e647_d_n4, eq24_e647_d_n5, eq24_e647_d_n6, eq24_e647_d_n7, eq24_e647_d_n8, eq24_e647_d_n9, eq24_e647_d_n10, eq24_e647_d_n11, eq24_e647_d_n12, eq24_e647_d_n13];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[13]),
            self.multiplicity * (eq24_value),
            &nodes,
            &eq24_node_derivatives,
            &branches,
            &eq24_branch_derivatives,
            self.multiplicity,
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq25_e650: f64 = (1e-9 * (nv10 - nv13));
        let eq25_e650_d_n10: f64 = 1e-9;
        let eq25_e650_d_n13: f64 = (-1e-9);
        let eq25_e651: f64 = self.eval_ddt(1, eq25_e650);
        let eq25_e651_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n10: f64 = self.ddt_jacobian(eq25_e650_d_n10);
        let eq25_e651_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq25_e651_d_n13: f64 = self.ddt_jacobian(eq25_e650_d_n13);
        let eq25_value: f64 = eq25_e651;
        let eq25_node_derivatives: [f64; 14] = [eq25_e651_d_n0, eq25_e651_d_n1, eq25_e651_d_n2, eq25_e651_d_n3, eq25_e651_d_n4, eq25_e651_d_n5, eq25_e651_d_n6, eq25_e651_d_n7, eq25_e651_d_n8, eq25_e651_d_n9, eq25_e651_d_n10, eq25_e651_d_n11, eq25_e651_d_n12, eq25_e651_d_n13];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[13]),
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
        let eq26_e653: f64 = self.eval_ddt(2, s.v[1776]);
        let eq26_e653_d_n0: f64 = self.ddt_jacobian(s.dn[1776][0]);
        let eq26_e653_d_n1: f64 = self.ddt_jacobian(s.dn[1776][1]);
        let eq26_e653_d_n2: f64 = self.ddt_jacobian(s.dn[1776][2]);
        let eq26_e653_d_n3: f64 = self.ddt_jacobian(s.dn[1776][3]);
        let eq26_e653_d_n4: f64 = self.ddt_jacobian(s.dn[1776][4]);
        let eq26_e653_d_n5: f64 = self.ddt_jacobian(s.dn[1776][5]);
        let eq26_e653_d_n6: f64 = self.ddt_jacobian(s.dn[1776][6]);
        let eq26_e653_d_n7: f64 = self.ddt_jacobian(s.dn[1776][7]);
        let eq26_e653_d_n8: f64 = self.ddt_jacobian(s.dn[1776][8]);
        let eq26_e653_d_n9: f64 = self.ddt_jacobian(s.dn[1776][9]);
        let eq26_e653_d_n10: f64 = self.ddt_jacobian(s.dn[1776][10]);
        let eq26_e653_d_n11: f64 = self.ddt_jacobian(s.dn[1776][11]);
        let eq26_e653_d_n12: f64 = self.ddt_jacobian(s.dn[1776][12]);
        let eq26_e653_d_n13: f64 = self.ddt_jacobian(s.dn[1776][13]);
        let eq26_value: f64 = eq26_e653;
        let eq26_node_derivatives: [f64; 14] = [eq26_e653_d_n0, eq26_e653_d_n1, eq26_e653_d_n2, eq26_e653_d_n3, eq26_e653_d_n4, eq26_e653_d_n5, eq26_e653_d_n6, eq26_e653_d_n7, eq26_e653_d_n8, eq26_e653_d_n9, eq26_e653_d_n10, eq26_e653_d_n11, eq26_e653_d_n12, eq26_e653_d_n13];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[13]),
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq27_e656: f64 = (s.v[1773] * (nv12 - nv13));
        let eq27_e656_d_n0: f64 = (s.dn[1773][0] * (nv12 - nv13));
        let eq27_e656_d_n1: f64 = (s.dn[1773][1] * (nv12 - nv13));
        let eq27_e656_d_n2: f64 = (s.dn[1773][2] * (nv12 - nv13));
        let eq27_e656_d_n3: f64 = (s.dn[1773][3] * (nv12 - nv13));
        let eq27_e656_d_n4: f64 = (s.dn[1773][4] * (nv12 - nv13));
        let eq27_e656_d_n5: f64 = (s.dn[1773][5] * (nv12 - nv13));
        let eq27_e656_d_n6: f64 = (s.dn[1773][6] * (nv12 - nv13));
        let eq27_e656_d_n7: f64 = (s.dn[1773][7] * (nv12 - nv13));
        let eq27_e656_d_n8: f64 = (s.dn[1773][8] * (nv12 - nv13));
        let eq27_e656_d_n9: f64 = (s.dn[1773][9] * (nv12 - nv13));
        let eq27_e656_d_n10: f64 = (s.dn[1773][10] * (nv12 - nv13));
        let eq27_e656_d_n11: f64 = (s.dn[1773][11] * (nv12 - nv13));
        let eq27_e656_d_n12: f64 = ((s.dn[1773][12] * (nv12 - nv13)) + s.v[1773]);
        let eq27_e656_d_n13: f64 = ((s.dn[1773][13] * (nv12 - nv13)) + (-s.v[1773]));
        let eq27_value: f64 = eq27_e656;
        let eq27_node_derivatives: [f64; 14] = [eq27_e656_d_n0, eq27_e656_d_n1, eq27_e656_d_n2, eq27_e656_d_n3, eq27_e656_d_n4, eq27_e656_d_n5, eq27_e656_d_n6, eq27_e656_d_n7, eq27_e656_d_n8, eq27_e656_d_n9, eq27_e656_d_n10, eq27_e656_d_n11, eq27_e656_d_n12, eq27_e656_d_n13];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[13]),
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq28_e659: f64 = (1e-9 * (nv12 - nv13));
        let eq28_e659_d_n12: f64 = 1e-9;
        let eq28_e659_d_n13: f64 = (-1e-9);
        let eq28_e660: f64 = self.eval_ddt(3, eq28_e659);
        let eq28_e660_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq28_e660_d_n12: f64 = self.ddt_jacobian(eq28_e659_d_n12);
        let eq28_e660_d_n13: f64 = self.ddt_jacobian(eq28_e659_d_n13);
        let eq28_value: f64 = eq28_e660;
        let eq28_node_derivatives: [f64; 14] = [eq28_e660_d_n0, eq28_e660_d_n1, eq28_e660_d_n2, eq28_e660_d_n3, eq28_e660_d_n4, eq28_e660_d_n5, eq28_e660_d_n6, eq28_e660_d_n7, eq28_e660_d_n8, eq28_e660_d_n9, eq28_e660_d_n10, eq28_e660_d_n11, eq28_e660_d_n12, eq28_e660_d_n13];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[13]),
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
        let eq29_e662: f64 = (s.v[182]).sqrt();
        let eq29_e662_d_n0: f64 = (s.dn[182][0] / (2.0 * eq29_e662));
        let eq29_e662_d_n1: f64 = (s.dn[182][1] / (2.0 * eq29_e662));
        let eq29_e662_d_n2: f64 = (s.dn[182][2] / (2.0 * eq29_e662));
        let eq29_e662_d_n3: f64 = (s.dn[182][3] / (2.0 * eq29_e662));
        let eq29_e662_d_n4: f64 = (s.dn[182][4] / (2.0 * eq29_e662));
        let eq29_e662_d_n5: f64 = (s.dn[182][5] / (2.0 * eq29_e662));
        let eq29_e662_d_n6: f64 = (s.dn[182][6] / (2.0 * eq29_e662));
        let eq29_e662_d_n7: f64 = (s.dn[182][7] / (2.0 * eq29_e662));
        let eq29_e662_d_n8: f64 = (s.dn[182][8] / (2.0 * eq29_e662));
        let eq29_e662_d_n9: f64 = (s.dn[182][9] / (2.0 * eq29_e662));
        let eq29_e662_d_n10: f64 = (s.dn[182][10] / (2.0 * eq29_e662));
        let eq29_e662_d_n11: f64 = (s.dn[182][11] / (2.0 * eq29_e662));
        let eq29_e662_d_n12: f64 = (s.dn[182][12] / (2.0 * eq29_e662));
        let eq29_e662_d_n13: f64 = (s.dn[182][13] / (2.0 * eq29_e662));
        let eq29_e665: f64 = (1.0 - s.v[181]);
        let eq29_e665_d_n0: f64 = (-s.dn[181][0]);
        let eq29_e665_d_n1: f64 = (-s.dn[181][1]);
        let eq29_e665_d_n2: f64 = (-s.dn[181][2]);
        let eq29_e665_d_n3: f64 = (-s.dn[181][3]);
        let eq29_e665_d_n4: f64 = (-s.dn[181][4]);
        let eq29_e665_d_n5: f64 = (-s.dn[181][5]);
        let eq29_e665_d_n6: f64 = (-s.dn[181][6]);
        let eq29_e665_d_n7: f64 = (-s.dn[181][7]);
        let eq29_e665_d_n8: f64 = (-s.dn[181][8]);
        let eq29_e665_d_n9: f64 = (-s.dn[181][9]);
        let eq29_e665_d_n10: f64 = (-s.dn[181][10]);
        let eq29_e665_d_n11: f64 = (-s.dn[181][11]);
        let eq29_e665_d_n12: f64 = (-s.dn[181][12]);
        let eq29_e665_d_n13: f64 = (-s.dn[181][13]);
        let eq29_e668: f64 = (s.v[1774] + s.v[1775]);
        let eq29_e668_d_n0: f64 = (s.dn[1774][0] + s.dn[1775][0]);
        let eq29_e668_d_n1: f64 = (s.dn[1774][1] + s.dn[1775][1]);
        let eq29_e668_d_n2: f64 = (s.dn[1774][2] + s.dn[1775][2]);
        let eq29_e668_d_n3: f64 = (s.dn[1774][3] + s.dn[1775][3]);
        let eq29_e668_d_n4: f64 = (s.dn[1774][4] + s.dn[1775][4]);
        let eq29_e668_d_n5: f64 = (s.dn[1774][5] + s.dn[1775][5]);
        let eq29_e668_d_n6: f64 = (s.dn[1774][6] + s.dn[1775][6]);
        let eq29_e668_d_n7: f64 = (s.dn[1774][7] + s.dn[1775][7]);
        let eq29_e668_d_n8: f64 = (s.dn[1774][8] + s.dn[1775][8]);
        let eq29_e668_d_n9: f64 = (s.dn[1774][9] + s.dn[1775][9]);
        let eq29_e668_d_n10: f64 = (s.dn[1774][10] + s.dn[1775][10]);
        let eq29_e668_d_n11: f64 = (s.dn[1774][11] + s.dn[1775][11]);
        let eq29_e668_d_n12: f64 = (s.dn[1774][12] + s.dn[1775][12]);
        let eq29_e668_d_n13: f64 = (s.dn[1774][13] + s.dn[1775][13]);
        let eq29_e669: f64 = (eq29_e665 * eq29_e668);
        let eq29_e669_d_n0: f64 = ((eq29_e665_d_n0 * eq29_e668) + (eq29_e665 * eq29_e668_d_n0));
        let eq29_e669_d_n1: f64 = ((eq29_e665_d_n1 * eq29_e668) + (eq29_e665 * eq29_e668_d_n1));
        let eq29_e669_d_n2: f64 = ((eq29_e665_d_n2 * eq29_e668) + (eq29_e665 * eq29_e668_d_n2));
        let eq29_e669_d_n3: f64 = ((eq29_e665_d_n3 * eq29_e668) + (eq29_e665 * eq29_e668_d_n3));
        let eq29_e669_d_n4: f64 = ((eq29_e665_d_n4 * eq29_e668) + (eq29_e665 * eq29_e668_d_n4));
        let eq29_e669_d_n5: f64 = ((eq29_e665_d_n5 * eq29_e668) + (eq29_e665 * eq29_e668_d_n5));
        let eq29_e669_d_n6: f64 = ((eq29_e665_d_n6 * eq29_e668) + (eq29_e665 * eq29_e668_d_n6));
        let eq29_e669_d_n7: f64 = ((eq29_e665_d_n7 * eq29_e668) + (eq29_e665 * eq29_e668_d_n7));
        let eq29_e669_d_n8: f64 = ((eq29_e665_d_n8 * eq29_e668) + (eq29_e665 * eq29_e668_d_n8));
        let eq29_e669_d_n9: f64 = ((eq29_e665_d_n9 * eq29_e668) + (eq29_e665 * eq29_e668_d_n9));
        let eq29_e669_d_n10: f64 = ((eq29_e665_d_n10 * eq29_e668) + (eq29_e665 * eq29_e668_d_n10));
        let eq29_e669_d_n11: f64 = ((eq29_e665_d_n11 * eq29_e668) + (eq29_e665 * eq29_e668_d_n11));
        let eq29_e669_d_n12: f64 = ((eq29_e665_d_n12 * eq29_e668) + (eq29_e665 * eq29_e668_d_n12));
        let eq29_e669_d_n13: f64 = ((eq29_e665_d_n13 * eq29_e668) + (eq29_e665 * eq29_e668_d_n13));
        let eq29_e670: f64 = self.eval_ddt(4, eq29_e669);
        let eq29_e670_d_n0: f64 = self.ddt_jacobian(eq29_e669_d_n0);
        let eq29_e670_d_n1: f64 = self.ddt_jacobian(eq29_e669_d_n1);
        let eq29_e670_d_n2: f64 = self.ddt_jacobian(eq29_e669_d_n2);
        let eq29_e670_d_n3: f64 = self.ddt_jacobian(eq29_e669_d_n3);
        let eq29_e670_d_n4: f64 = self.ddt_jacobian(eq29_e669_d_n4);
        let eq29_e670_d_n5: f64 = self.ddt_jacobian(eq29_e669_d_n5);
        let eq29_e670_d_n6: f64 = self.ddt_jacobian(eq29_e669_d_n6);
        let eq29_e670_d_n7: f64 = self.ddt_jacobian(eq29_e669_d_n7);
        let eq29_e670_d_n8: f64 = self.ddt_jacobian(eq29_e669_d_n8);
        let eq29_e670_d_n9: f64 = self.ddt_jacobian(eq29_e669_d_n9);
        let eq29_e670_d_n10: f64 = self.ddt_jacobian(eq29_e669_d_n10);
        let eq29_e670_d_n11: f64 = self.ddt_jacobian(eq29_e669_d_n11);
        let eq29_e670_d_n12: f64 = self.ddt_jacobian(eq29_e669_d_n12);
        let eq29_e670_d_n13: f64 = self.ddt_jacobian(eq29_e669_d_n13);
        let eq29_e671: f64 = (eq29_e662 * eq29_e670);
        let eq29_e671_d_n0: f64 = ((eq29_e662_d_n0 * eq29_e670) + (eq29_e662 * eq29_e670_d_n0));
        let eq29_e671_d_n1: f64 = ((eq29_e662_d_n1 * eq29_e670) + (eq29_e662 * eq29_e670_d_n1));
        let eq29_e671_d_n2: f64 = ((eq29_e662_d_n2 * eq29_e670) + (eq29_e662 * eq29_e670_d_n2));
        let eq29_e671_d_n3: f64 = ((eq29_e662_d_n3 * eq29_e670) + (eq29_e662 * eq29_e670_d_n3));
        let eq29_e671_d_n4: f64 = ((eq29_e662_d_n4 * eq29_e670) + (eq29_e662 * eq29_e670_d_n4));
        let eq29_e671_d_n5: f64 = ((eq29_e662_d_n5 * eq29_e670) + (eq29_e662 * eq29_e670_d_n5));
        let eq29_e671_d_n6: f64 = ((eq29_e662_d_n6 * eq29_e670) + (eq29_e662 * eq29_e670_d_n6));
        let eq29_e671_d_n7: f64 = ((eq29_e662_d_n7 * eq29_e670) + (eq29_e662 * eq29_e670_d_n7));
        let eq29_e671_d_n8: f64 = ((eq29_e662_d_n8 * eq29_e670) + (eq29_e662 * eq29_e670_d_n8));
        let eq29_e671_d_n9: f64 = ((eq29_e662_d_n9 * eq29_e670) + (eq29_e662 * eq29_e670_d_n9));
        let eq29_e671_d_n10: f64 = ((eq29_e662_d_n10 * eq29_e670) + (eq29_e662 * eq29_e670_d_n10));
        let eq29_e671_d_n11: f64 = ((eq29_e662_d_n11 * eq29_e670) + (eq29_e662 * eq29_e670_d_n11));
        let eq29_e671_d_n12: f64 = ((eq29_e662_d_n12 * eq29_e670) + (eq29_e662 * eq29_e670_d_n12));
        let eq29_e671_d_n13: f64 = ((eq29_e662_d_n13 * eq29_e670) + (eq29_e662 * eq29_e670_d_n13));
        let eq29_value: f64 = eq29_e671;
        let eq29_node_derivatives: [f64; 14] = [eq29_e671_d_n0, eq29_e671_d_n1, eq29_e671_d_n2, eq29_e671_d_n3, eq29_e671_d_n4, eq29_e671_d_n5, eq29_e671_d_n6, eq29_e671_d_n7, eq29_e671_d_n8, eq29_e671_d_n9, eq29_e671_d_n10, eq29_e671_d_n11, eq29_e671_d_n12, eq29_e671_d_n13];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[13]),
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq30_e674: f64 = (s.v[1773] * (nv11 - nv13));
        let eq30_e674_d_n0: f64 = (s.dn[1773][0] * (nv11 - nv13));
        let eq30_e674_d_n1: f64 = (s.dn[1773][1] * (nv11 - nv13));
        let eq30_e674_d_n2: f64 = (s.dn[1773][2] * (nv11 - nv13));
        let eq30_e674_d_n3: f64 = (s.dn[1773][3] * (nv11 - nv13));
        let eq30_e674_d_n4: f64 = (s.dn[1773][4] * (nv11 - nv13));
        let eq30_e674_d_n5: f64 = (s.dn[1773][5] * (nv11 - nv13));
        let eq30_e674_d_n6: f64 = (s.dn[1773][6] * (nv11 - nv13));
        let eq30_e674_d_n7: f64 = (s.dn[1773][7] * (nv11 - nv13));
        let eq30_e674_d_n8: f64 = (s.dn[1773][8] * (nv11 - nv13));
        let eq30_e674_d_n9: f64 = (s.dn[1773][9] * (nv11 - nv13));
        let eq30_e674_d_n10: f64 = (s.dn[1773][10] * (nv11 - nv13));
        let eq30_e674_d_n11: f64 = ((s.dn[1773][11] * (nv11 - nv13)) + s.v[1773]);
        let eq30_e674_d_n12: f64 = (s.dn[1773][12] * (nv11 - nv13));
        let eq30_e674_d_n13: f64 = ((s.dn[1773][13] * (nv11 - nv13)) + (-s.v[1773]));
        let eq30_value: f64 = eq30_e674;
        let eq30_node_derivatives: [f64; 14] = [eq30_e674_d_n0, eq30_e674_d_n1, eq30_e674_d_n2, eq30_e674_d_n3, eq30_e674_d_n4, eq30_e674_d_n5, eq30_e674_d_n6, eq30_e674_d_n7, eq30_e674_d_n8, eq30_e674_d_n9, eq30_e674_d_n10, eq30_e674_d_n11, eq30_e674_d_n12, eq30_e674_d_n13];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[13]),
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq31_e678: f64 = (1e-9 * (nv11 - nv13));
        let eq31_e678_d_n11: f64 = 1e-9;
        let eq31_e678_d_n13: f64 = (-1e-9);
        let eq31_e679: f64 = self.eval_ddt(5, eq31_e678);
        let eq31_e679_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n11: f64 = self.ddt_jacobian(eq31_e678_d_n11);
        let eq31_e679_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq31_e679_d_n13: f64 = self.ddt_jacobian(eq31_e678_d_n13);
        let eq31_e680: f64 = (s.v[182] * eq31_e679);
        let eq31_e680_d_n0: f64 = ((s.dn[182][0] * eq31_e679) + (s.v[182] * eq31_e679_d_n0));
        let eq31_e680_d_n1: f64 = ((s.dn[182][1] * eq31_e679) + (s.v[182] * eq31_e679_d_n1));
        let eq31_e680_d_n2: f64 = ((s.dn[182][2] * eq31_e679) + (s.v[182] * eq31_e679_d_n2));
        let eq31_e680_d_n3: f64 = ((s.dn[182][3] * eq31_e679) + (s.v[182] * eq31_e679_d_n3));
        let eq31_e680_d_n4: f64 = ((s.dn[182][4] * eq31_e679) + (s.v[182] * eq31_e679_d_n4));
        let eq31_e680_d_n5: f64 = ((s.dn[182][5] * eq31_e679) + (s.v[182] * eq31_e679_d_n5));
        let eq31_e680_d_n6: f64 = ((s.dn[182][6] * eq31_e679) + (s.v[182] * eq31_e679_d_n6));
        let eq31_e680_d_n7: f64 = ((s.dn[182][7] * eq31_e679) + (s.v[182] * eq31_e679_d_n7));
        let eq31_e680_d_n8: f64 = ((s.dn[182][8] * eq31_e679) + (s.v[182] * eq31_e679_d_n8));
        let eq31_e680_d_n9: f64 = ((s.dn[182][9] * eq31_e679) + (s.v[182] * eq31_e679_d_n9));
        let eq31_e680_d_n10: f64 = ((s.dn[182][10] * eq31_e679) + (s.v[182] * eq31_e679_d_n10));
        let eq31_e680_d_n11: f64 = ((s.dn[182][11] * eq31_e679) + (s.v[182] * eq31_e679_d_n11));
        let eq31_e680_d_n12: f64 = ((s.dn[182][12] * eq31_e679) + (s.v[182] * eq31_e679_d_n12));
        let eq31_e680_d_n13: f64 = ((s.dn[182][13] * eq31_e679) + (s.v[182] * eq31_e679_d_n13));
        let eq31_value: f64 = eq31_e680;
        let eq31_node_derivatives: [f64; 14] = [eq31_e680_d_n0, eq31_e680_d_n1, eq31_e680_d_n2, eq31_e680_d_n3, eq31_e680_d_n4, eq31_e680_d_n5, eq31_e680_d_n6, eq31_e680_d_n7, eq31_e680_d_n8, eq31_e680_d_n9, eq31_e680_d_n10, eq31_e680_d_n11, eq31_e680_d_n12, eq31_e680_d_n13];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[13]),
            self.multiplicity * (eq31_value),
            &nodes,
            &eq31_node_derivatives,
            &branches,
            &eq31_branch_derivatives,
            self.multiplicity,
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
        let eq32_e683: f64 = self.eval_ddt(6, s.v[362]);
        let eq32_e683_d_n0: f64 = self.ddt_jacobian(s.dn[362][0]);
        let eq32_e683_d_n1: f64 = self.ddt_jacobian(s.dn[362][1]);
        let eq32_e683_d_n2: f64 = self.ddt_jacobian(s.dn[362][2]);
        let eq32_e683_d_n3: f64 = self.ddt_jacobian(s.dn[362][3]);
        let eq32_e683_d_n4: f64 = self.ddt_jacobian(s.dn[362][4]);
        let eq32_e683_d_n5: f64 = self.ddt_jacobian(s.dn[362][5]);
        let eq32_e683_d_n6: f64 = self.ddt_jacobian(s.dn[362][6]);
        let eq32_e683_d_n7: f64 = self.ddt_jacobian(s.dn[362][7]);
        let eq32_e683_d_n8: f64 = self.ddt_jacobian(s.dn[362][8]);
        let eq32_e683_d_n9: f64 = self.ddt_jacobian(s.dn[362][9]);
        let eq32_e683_d_n10: f64 = self.ddt_jacobian(s.dn[362][10]);
        let eq32_e683_d_n11: f64 = self.ddt_jacobian(s.dn[362][11]);
        let eq32_e683_d_n12: f64 = self.ddt_jacobian(s.dn[362][12]);
        let eq32_e683_d_n13: f64 = self.ddt_jacobian(s.dn[362][13]);
        let eq32_e685: f64 = self.eval_ddt(7, s.v[377]);
        let eq32_e685_d_n0: f64 = self.ddt_jacobian(s.dn[377][0]);
        let eq32_e685_d_n1: f64 = self.ddt_jacobian(s.dn[377][1]);
        let eq32_e685_d_n2: f64 = self.ddt_jacobian(s.dn[377][2]);
        let eq32_e685_d_n3: f64 = self.ddt_jacobian(s.dn[377][3]);
        let eq32_e685_d_n4: f64 = self.ddt_jacobian(s.dn[377][4]);
        let eq32_e685_d_n5: f64 = self.ddt_jacobian(s.dn[377][5]);
        let eq32_e685_d_n6: f64 = self.ddt_jacobian(s.dn[377][6]);
        let eq32_e685_d_n7: f64 = self.ddt_jacobian(s.dn[377][7]);
        let eq32_e685_d_n8: f64 = self.ddt_jacobian(s.dn[377][8]);
        let eq32_e685_d_n9: f64 = self.ddt_jacobian(s.dn[377][9]);
        let eq32_e685_d_n10: f64 = self.ddt_jacobian(s.dn[377][10]);
        let eq32_e685_d_n11: f64 = self.ddt_jacobian(s.dn[377][11]);
        let eq32_e685_d_n12: f64 = self.ddt_jacobian(s.dn[377][12]);
        let eq32_e685_d_n13: f64 = self.ddt_jacobian(s.dn[377][13]);
        let eq32_e686: f64 = (eq32_e683 + eq32_e685);
        let eq32_e686_d_n0: f64 = (eq32_e683_d_n0 + eq32_e685_d_n0);
        let eq32_e686_d_n1: f64 = (eq32_e683_d_n1 + eq32_e685_d_n1);
        let eq32_e686_d_n2: f64 = (eq32_e683_d_n2 + eq32_e685_d_n2);
        let eq32_e686_d_n3: f64 = (eq32_e683_d_n3 + eq32_e685_d_n3);
        let eq32_e686_d_n4: f64 = (eq32_e683_d_n4 + eq32_e685_d_n4);
        let eq32_e686_d_n5: f64 = (eq32_e683_d_n5 + eq32_e685_d_n5);
        let eq32_e686_d_n6: f64 = (eq32_e683_d_n6 + eq32_e685_d_n6);
        let eq32_e686_d_n7: f64 = (eq32_e683_d_n7 + eq32_e685_d_n7);
        let eq32_e686_d_n8: f64 = (eq32_e683_d_n8 + eq32_e685_d_n8);
        let eq32_e686_d_n9: f64 = (eq32_e683_d_n9 + eq32_e685_d_n9);
        let eq32_e686_d_n10: f64 = (eq32_e683_d_n10 + eq32_e685_d_n10);
        let eq32_e686_d_n11: f64 = (eq32_e683_d_n11 + eq32_e685_d_n11);
        let eq32_e686_d_n12: f64 = (eq32_e683_d_n12 + eq32_e685_d_n12);
        let eq32_e686_d_n13: f64 = (eq32_e683_d_n13 + eq32_e685_d_n13);
        let eq32_e688: f64 = self.eval_ddt(8, s.v[381]);
        let eq32_e688_d_n0: f64 = self.ddt_jacobian(s.dn[381][0]);
        let eq32_e688_d_n1: f64 = self.ddt_jacobian(s.dn[381][1]);
        let eq32_e688_d_n2: f64 = self.ddt_jacobian(s.dn[381][2]);
        let eq32_e688_d_n3: f64 = self.ddt_jacobian(s.dn[381][3]);
        let eq32_e688_d_n4: f64 = self.ddt_jacobian(s.dn[381][4]);
        let eq32_e688_d_n5: f64 = self.ddt_jacobian(s.dn[381][5]);
        let eq32_e688_d_n6: f64 = self.ddt_jacobian(s.dn[381][6]);
        let eq32_e688_d_n7: f64 = self.ddt_jacobian(s.dn[381][7]);
        let eq32_e688_d_n8: f64 = self.ddt_jacobian(s.dn[381][8]);
        let eq32_e688_d_n9: f64 = self.ddt_jacobian(s.dn[381][9]);
        let eq32_e688_d_n10: f64 = self.ddt_jacobian(s.dn[381][10]);
        let eq32_e688_d_n11: f64 = self.ddt_jacobian(s.dn[381][11]);
        let eq32_e688_d_n12: f64 = self.ddt_jacobian(s.dn[381][12]);
        let eq32_e688_d_n13: f64 = self.ddt_jacobian(s.dn[381][13]);
        let eq32_e689: f64 = (eq32_e686 + eq32_e688);
        let eq32_e689_d_n0: f64 = (eq32_e686_d_n0 + eq32_e688_d_n0);
        let eq32_e689_d_n1: f64 = (eq32_e686_d_n1 + eq32_e688_d_n1);
        let eq32_e689_d_n2: f64 = (eq32_e686_d_n2 + eq32_e688_d_n2);
        let eq32_e689_d_n3: f64 = (eq32_e686_d_n3 + eq32_e688_d_n3);
        let eq32_e689_d_n4: f64 = (eq32_e686_d_n4 + eq32_e688_d_n4);
        let eq32_e689_d_n5: f64 = (eq32_e686_d_n5 + eq32_e688_d_n5);
        let eq32_e689_d_n6: f64 = (eq32_e686_d_n6 + eq32_e688_d_n6);
        let eq32_e689_d_n7: f64 = (eq32_e686_d_n7 + eq32_e688_d_n7);
        let eq32_e689_d_n8: f64 = (eq32_e686_d_n8 + eq32_e688_d_n8);
        let eq32_e689_d_n9: f64 = (eq32_e686_d_n9 + eq32_e688_d_n9);
        let eq32_e689_d_n10: f64 = (eq32_e686_d_n10 + eq32_e688_d_n10);
        let eq32_e689_d_n11: f64 = (eq32_e686_d_n11 + eq32_e688_d_n11);
        let eq32_e689_d_n12: f64 = (eq32_e686_d_n12 + eq32_e688_d_n12);
        let eq32_e689_d_n13: f64 = (eq32_e686_d_n13 + eq32_e688_d_n13);
        let eq32_e690: f64 = (p.p14 * eq32_e689);
        let eq32_e690_d_n0: f64 = (p.p14 * eq32_e689_d_n0);
        let eq32_e690_d_n1: f64 = (p.p14 * eq32_e689_d_n1);
        let eq32_e690_d_n2: f64 = (p.p14 * eq32_e689_d_n2);
        let eq32_e690_d_n3: f64 = (p.p14 * eq32_e689_d_n3);
        let eq32_e690_d_n4: f64 = (p.p14 * eq32_e689_d_n4);
        let eq32_e690_d_n5: f64 = (p.p14 * eq32_e689_d_n5);
        let eq32_e690_d_n6: f64 = (p.p14 * eq32_e689_d_n6);
        let eq32_e690_d_n7: f64 = (p.p14 * eq32_e689_d_n7);
        let eq32_e690_d_n8: f64 = (p.p14 * eq32_e689_d_n8);
        let eq32_e690_d_n9: f64 = (p.p14 * eq32_e689_d_n9);
        let eq32_e690_d_n10: f64 = (p.p14 * eq32_e689_d_n10);
        let eq32_e690_d_n11: f64 = (p.p14 * eq32_e689_d_n11);
        let eq32_e690_d_n12: f64 = (p.p14 * eq32_e689_d_n12);
        let eq32_e690_d_n13: f64 = (p.p14 * eq32_e689_d_n13);
        let eq32_value: f64 = eq32_e690;
        let eq32_node_derivatives: [f64; 14] = [eq32_e690_d_n0, eq32_e690_d_n1, eq32_e690_d_n2, eq32_e690_d_n3, eq32_e690_d_n4, eq32_e690_d_n5, eq32_e690_d_n6, eq32_e690_d_n7, eq32_e690_d_n8, eq32_e690_d_n9, eq32_e690_d_n10, eq32_e690_d_n11, eq32_e690_d_n12, eq32_e690_d_n13];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq32_value),
            &nodes,
            &eq32_node_derivatives,
            &branches,
            &eq32_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_33_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq33_e693: f64 = self.eval_ddt(9, s.v[371]);
        let eq33_e693_d_n0: f64 = self.ddt_jacobian(s.dn[371][0]);
        let eq33_e693_d_n1: f64 = self.ddt_jacobian(s.dn[371][1]);
        let eq33_e693_d_n2: f64 = self.ddt_jacobian(s.dn[371][2]);
        let eq33_e693_d_n3: f64 = self.ddt_jacobian(s.dn[371][3]);
        let eq33_e693_d_n4: f64 = self.ddt_jacobian(s.dn[371][4]);
        let eq33_e693_d_n5: f64 = self.ddt_jacobian(s.dn[371][5]);
        let eq33_e693_d_n6: f64 = self.ddt_jacobian(s.dn[371][6]);
        let eq33_e693_d_n7: f64 = self.ddt_jacobian(s.dn[371][7]);
        let eq33_e693_d_n8: f64 = self.ddt_jacobian(s.dn[371][8]);
        let eq33_e693_d_n9: f64 = self.ddt_jacobian(s.dn[371][9]);
        let eq33_e693_d_n10: f64 = self.ddt_jacobian(s.dn[371][10]);
        let eq33_e693_d_n11: f64 = self.ddt_jacobian(s.dn[371][11]);
        let eq33_e693_d_n12: f64 = self.ddt_jacobian(s.dn[371][12]);
        let eq33_e693_d_n13: f64 = self.ddt_jacobian(s.dn[371][13]);
        let eq33_e695: f64 = self.eval_ddt(10, s.v[373]);
        let eq33_e695_d_n0: f64 = self.ddt_jacobian(s.dn[373][0]);
        let eq33_e695_d_n1: f64 = self.ddt_jacobian(s.dn[373][1]);
        let eq33_e695_d_n2: f64 = self.ddt_jacobian(s.dn[373][2]);
        let eq33_e695_d_n3: f64 = self.ddt_jacobian(s.dn[373][3]);
        let eq33_e695_d_n4: f64 = self.ddt_jacobian(s.dn[373][4]);
        let eq33_e695_d_n5: f64 = self.ddt_jacobian(s.dn[373][5]);
        let eq33_e695_d_n6: f64 = self.ddt_jacobian(s.dn[373][6]);
        let eq33_e695_d_n7: f64 = self.ddt_jacobian(s.dn[373][7]);
        let eq33_e695_d_n8: f64 = self.ddt_jacobian(s.dn[373][8]);
        let eq33_e695_d_n9: f64 = self.ddt_jacobian(s.dn[373][9]);
        let eq33_e695_d_n10: f64 = self.ddt_jacobian(s.dn[373][10]);
        let eq33_e695_d_n11: f64 = self.ddt_jacobian(s.dn[373][11]);
        let eq33_e695_d_n12: f64 = self.ddt_jacobian(s.dn[373][12]);
        let eq33_e695_d_n13: f64 = self.ddt_jacobian(s.dn[373][13]);
        let eq33_e696: f64 = (eq33_e693 + eq33_e695);
        let eq33_e696_d_n0: f64 = (eq33_e693_d_n0 + eq33_e695_d_n0);
        let eq33_e696_d_n1: f64 = (eq33_e693_d_n1 + eq33_e695_d_n1);
        let eq33_e696_d_n2: f64 = (eq33_e693_d_n2 + eq33_e695_d_n2);
        let eq33_e696_d_n3: f64 = (eq33_e693_d_n3 + eq33_e695_d_n3);
        let eq33_e696_d_n4: f64 = (eq33_e693_d_n4 + eq33_e695_d_n4);
        let eq33_e696_d_n5: f64 = (eq33_e693_d_n5 + eq33_e695_d_n5);
        let eq33_e696_d_n6: f64 = (eq33_e693_d_n6 + eq33_e695_d_n6);
        let eq33_e696_d_n7: f64 = (eq33_e693_d_n7 + eq33_e695_d_n7);
        let eq33_e696_d_n8: f64 = (eq33_e693_d_n8 + eq33_e695_d_n8);
        let eq33_e696_d_n9: f64 = (eq33_e693_d_n9 + eq33_e695_d_n9);
        let eq33_e696_d_n10: f64 = (eq33_e693_d_n10 + eq33_e695_d_n10);
        let eq33_e696_d_n11: f64 = (eq33_e693_d_n11 + eq33_e695_d_n11);
        let eq33_e696_d_n12: f64 = (eq33_e693_d_n12 + eq33_e695_d_n12);
        let eq33_e696_d_n13: f64 = (eq33_e693_d_n13 + eq33_e695_d_n13);
        let eq33_e698: f64 = self.eval_ddt(11, s.v[380]);
        let eq33_e698_d_n0: f64 = self.ddt_jacobian(s.dn[380][0]);
        let eq33_e698_d_n1: f64 = self.ddt_jacobian(s.dn[380][1]);
        let eq33_e698_d_n2: f64 = self.ddt_jacobian(s.dn[380][2]);
        let eq33_e698_d_n3: f64 = self.ddt_jacobian(s.dn[380][3]);
        let eq33_e698_d_n4: f64 = self.ddt_jacobian(s.dn[380][4]);
        let eq33_e698_d_n5: f64 = self.ddt_jacobian(s.dn[380][5]);
        let eq33_e698_d_n6: f64 = self.ddt_jacobian(s.dn[380][6]);
        let eq33_e698_d_n7: f64 = self.ddt_jacobian(s.dn[380][7]);
        let eq33_e698_d_n8: f64 = self.ddt_jacobian(s.dn[380][8]);
        let eq33_e698_d_n9: f64 = self.ddt_jacobian(s.dn[380][9]);
        let eq33_e698_d_n10: f64 = self.ddt_jacobian(s.dn[380][10]);
        let eq33_e698_d_n11: f64 = self.ddt_jacobian(s.dn[380][11]);
        let eq33_e698_d_n12: f64 = self.ddt_jacobian(s.dn[380][12]);
        let eq33_e698_d_n13: f64 = self.ddt_jacobian(s.dn[380][13]);
        let eq33_e699: f64 = (eq33_e696 + eq33_e698);
        let eq33_e699_d_n0: f64 = (eq33_e696_d_n0 + eq33_e698_d_n0);
        let eq33_e699_d_n1: f64 = (eq33_e696_d_n1 + eq33_e698_d_n1);
        let eq33_e699_d_n2: f64 = (eq33_e696_d_n2 + eq33_e698_d_n2);
        let eq33_e699_d_n3: f64 = (eq33_e696_d_n3 + eq33_e698_d_n3);
        let eq33_e699_d_n4: f64 = (eq33_e696_d_n4 + eq33_e698_d_n4);
        let eq33_e699_d_n5: f64 = (eq33_e696_d_n5 + eq33_e698_d_n5);
        let eq33_e699_d_n6: f64 = (eq33_e696_d_n6 + eq33_e698_d_n6);
        let eq33_e699_d_n7: f64 = (eq33_e696_d_n7 + eq33_e698_d_n7);
        let eq33_e699_d_n8: f64 = (eq33_e696_d_n8 + eq33_e698_d_n8);
        let eq33_e699_d_n9: f64 = (eq33_e696_d_n9 + eq33_e698_d_n9);
        let eq33_e699_d_n10: f64 = (eq33_e696_d_n10 + eq33_e698_d_n10);
        let eq33_e699_d_n11: f64 = (eq33_e696_d_n11 + eq33_e698_d_n11);
        let eq33_e699_d_n12: f64 = (eq33_e696_d_n12 + eq33_e698_d_n12);
        let eq33_e699_d_n13: f64 = (eq33_e696_d_n13 + eq33_e698_d_n13);
        let eq33_e700: f64 = (p.p14 * eq33_e699);
        let eq33_e700_d_n0: f64 = (p.p14 * eq33_e699_d_n0);
        let eq33_e700_d_n1: f64 = (p.p14 * eq33_e699_d_n1);
        let eq33_e700_d_n2: f64 = (p.p14 * eq33_e699_d_n2);
        let eq33_e700_d_n3: f64 = (p.p14 * eq33_e699_d_n3);
        let eq33_e700_d_n4: f64 = (p.p14 * eq33_e699_d_n4);
        let eq33_e700_d_n5: f64 = (p.p14 * eq33_e699_d_n5);
        let eq33_e700_d_n6: f64 = (p.p14 * eq33_e699_d_n6);
        let eq33_e700_d_n7: f64 = (p.p14 * eq33_e699_d_n7);
        let eq33_e700_d_n8: f64 = (p.p14 * eq33_e699_d_n8);
        let eq33_e700_d_n9: f64 = (p.p14 * eq33_e699_d_n9);
        let eq33_e700_d_n10: f64 = (p.p14 * eq33_e699_d_n10);
        let eq33_e700_d_n11: f64 = (p.p14 * eq33_e699_d_n11);
        let eq33_e700_d_n12: f64 = (p.p14 * eq33_e699_d_n12);
        let eq33_e700_d_n13: f64 = (p.p14 * eq33_e699_d_n13);
        let eq33_value: f64 = eq33_e700;
        let eq33_node_derivatives: [f64; 14] = [eq33_e700_d_n0, eq33_e700_d_n1, eq33_e700_d_n2, eq33_e700_d_n3, eq33_e700_d_n4, eq33_e700_d_n5, eq33_e700_d_n6, eq33_e700_d_n7, eq33_e700_d_n8, eq33_e700_d_n9, eq33_e700_d_n10, eq33_e700_d_n11, eq33_e700_d_n12, eq33_e700_d_n13];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq33_value),
            &nodes,
            &eq33_node_derivatives,
            &branches,
            &eq33_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq34_e703: f64 = self.eval_ddt(12, s.v[376]);
        let eq34_e703_d_n0: f64 = self.ddt_jacobian(s.dn[376][0]);
        let eq34_e703_d_n1: f64 = self.ddt_jacobian(s.dn[376][1]);
        let eq34_e703_d_n2: f64 = self.ddt_jacobian(s.dn[376][2]);
        let eq34_e703_d_n3: f64 = self.ddt_jacobian(s.dn[376][3]);
        let eq34_e703_d_n4: f64 = self.ddt_jacobian(s.dn[376][4]);
        let eq34_e703_d_n5: f64 = self.ddt_jacobian(s.dn[376][5]);
        let eq34_e703_d_n6: f64 = self.ddt_jacobian(s.dn[376][6]);
        let eq34_e703_d_n7: f64 = self.ddt_jacobian(s.dn[376][7]);
        let eq34_e703_d_n8: f64 = self.ddt_jacobian(s.dn[376][8]);
        let eq34_e703_d_n9: f64 = self.ddt_jacobian(s.dn[376][9]);
        let eq34_e703_d_n10: f64 = self.ddt_jacobian(s.dn[376][10]);
        let eq34_e703_d_n11: f64 = self.ddt_jacobian(s.dn[376][11]);
        let eq34_e703_d_n12: f64 = self.ddt_jacobian(s.dn[376][12]);
        let eq34_e703_d_n13: f64 = self.ddt_jacobian(s.dn[376][13]);
        let eq34_e705: f64 = self.eval_ddt(13, s.v[382]);
        let eq34_e705_d_n0: f64 = self.ddt_jacobian(s.dn[382][0]);
        let eq34_e705_d_n1: f64 = self.ddt_jacobian(s.dn[382][1]);
        let eq34_e705_d_n2: f64 = self.ddt_jacobian(s.dn[382][2]);
        let eq34_e705_d_n3: f64 = self.ddt_jacobian(s.dn[382][3]);
        let eq34_e705_d_n4: f64 = self.ddt_jacobian(s.dn[382][4]);
        let eq34_e705_d_n5: f64 = self.ddt_jacobian(s.dn[382][5]);
        let eq34_e705_d_n6: f64 = self.ddt_jacobian(s.dn[382][6]);
        let eq34_e705_d_n7: f64 = self.ddt_jacobian(s.dn[382][7]);
        let eq34_e705_d_n8: f64 = self.ddt_jacobian(s.dn[382][8]);
        let eq34_e705_d_n9: f64 = self.ddt_jacobian(s.dn[382][9]);
        let eq34_e705_d_n10: f64 = self.ddt_jacobian(s.dn[382][10]);
        let eq34_e705_d_n11: f64 = self.ddt_jacobian(s.dn[382][11]);
        let eq34_e705_d_n12: f64 = self.ddt_jacobian(s.dn[382][12]);
        let eq34_e705_d_n13: f64 = self.ddt_jacobian(s.dn[382][13]);
        let eq34_e706: f64 = (eq34_e703 + eq34_e705);
        let eq34_e706_d_n0: f64 = (eq34_e703_d_n0 + eq34_e705_d_n0);
        let eq34_e706_d_n1: f64 = (eq34_e703_d_n1 + eq34_e705_d_n1);
        let eq34_e706_d_n2: f64 = (eq34_e703_d_n2 + eq34_e705_d_n2);
        let eq34_e706_d_n3: f64 = (eq34_e703_d_n3 + eq34_e705_d_n3);
        let eq34_e706_d_n4: f64 = (eq34_e703_d_n4 + eq34_e705_d_n4);
        let eq34_e706_d_n5: f64 = (eq34_e703_d_n5 + eq34_e705_d_n5);
        let eq34_e706_d_n6: f64 = (eq34_e703_d_n6 + eq34_e705_d_n6);
        let eq34_e706_d_n7: f64 = (eq34_e703_d_n7 + eq34_e705_d_n7);
        let eq34_e706_d_n8: f64 = (eq34_e703_d_n8 + eq34_e705_d_n8);
        let eq34_e706_d_n9: f64 = (eq34_e703_d_n9 + eq34_e705_d_n9);
        let eq34_e706_d_n10: f64 = (eq34_e703_d_n10 + eq34_e705_d_n10);
        let eq34_e706_d_n11: f64 = (eq34_e703_d_n11 + eq34_e705_d_n11);
        let eq34_e706_d_n12: f64 = (eq34_e703_d_n12 + eq34_e705_d_n12);
        let eq34_e706_d_n13: f64 = (eq34_e703_d_n13 + eq34_e705_d_n13);
        let eq34_e707: f64 = (p.p14 * eq34_e706);
        let eq34_e707_d_n0: f64 = (p.p14 * eq34_e706_d_n0);
        let eq34_e707_d_n1: f64 = (p.p14 * eq34_e706_d_n1);
        let eq34_e707_d_n2: f64 = (p.p14 * eq34_e706_d_n2);
        let eq34_e707_d_n3: f64 = (p.p14 * eq34_e706_d_n3);
        let eq34_e707_d_n4: f64 = (p.p14 * eq34_e706_d_n4);
        let eq34_e707_d_n5: f64 = (p.p14 * eq34_e706_d_n5);
        let eq34_e707_d_n6: f64 = (p.p14 * eq34_e706_d_n6);
        let eq34_e707_d_n7: f64 = (p.p14 * eq34_e706_d_n7);
        let eq34_e707_d_n8: f64 = (p.p14 * eq34_e706_d_n8);
        let eq34_e707_d_n9: f64 = (p.p14 * eq34_e706_d_n9);
        let eq34_e707_d_n10: f64 = (p.p14 * eq34_e706_d_n10);
        let eq34_e707_d_n11: f64 = (p.p14 * eq34_e706_d_n11);
        let eq34_e707_d_n12: f64 = (p.p14 * eq34_e706_d_n12);
        let eq34_e707_d_n13: f64 = (p.p14 * eq34_e706_d_n13);
        let eq34_value: f64 = eq34_e707;
        let eq34_node_derivatives: [f64; 14] = [eq34_e707_d_n0, eq34_e707_d_n1, eq34_e707_d_n2, eq34_e707_d_n3, eq34_e707_d_n4, eq34_e707_d_n5, eq34_e707_d_n6, eq34_e707_d_n7, eq34_e707_d_n8, eq34_e707_d_n9, eq34_e707_d_n10, eq34_e707_d_n11, eq34_e707_d_n12, eq34_e707_d_n13];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq34_value),
            &nodes,
            &eq34_node_derivatives,
            &branches,
            &eq34_branch_derivatives,
            self.multiplicity,
        );
    }
}

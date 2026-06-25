#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq22_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[11]),
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
        let eq23_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[12]),
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
        let eq24_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq25_e478, eq25_e478_d_n0, eq25_e478_d_n1, eq25_e478_d_n2, eq25_e478_d_n3, eq25_e478_d_n4, eq25_e478_d_n5, eq25_e478_d_n6, eq25_e478_d_n7, eq25_e478_d_n8, eq25_e478_d_n9, eq25_e478_d_n10, eq25_e478_d_n11, eq25_e478_d_n12, eq25_e478_d_b0, eq25_e478_d_b1, eq25_e478_d_b2, eq25_e478_d_b3,) = {
    if (p.p25 != 0.0) {
        let eq25_e476: f64 = (s.v[484] * (nv1 - nv5));
        let eq25_e476_d_n0: f64 = (s.dn[484][0] * (nv1 - nv5));
        let eq25_e476_d_n1: f64 = ((s.dn[484][1] * (nv1 - nv5)) + s.v[484]);
        let eq25_e476_d_n2: f64 = (s.dn[484][2] * (nv1 - nv5));
        let eq25_e476_d_n3: f64 = (s.dn[484][3] * (nv1 - nv5));
        let eq25_e476_d_n4: f64 = (s.dn[484][4] * (nv1 - nv5));
        let eq25_e476_d_n5: f64 = ((s.dn[484][5] * (nv1 - nv5)) + (-s.v[484]));
        let eq25_e476_d_n6: f64 = (s.dn[484][6] * (nv1 - nv5));
        let eq25_e476_d_n7: f64 = (s.dn[484][7] * (nv1 - nv5));
        let eq25_e476_d_n8: f64 = (s.dn[484][8] * (nv1 - nv5));
        let eq25_e476_d_n9: f64 = (s.dn[484][9] * (nv1 - nv5));
        let eq25_e476_d_n10: f64 = (s.dn[484][10] * (nv1 - nv5));
        let eq25_e476_d_n11: f64 = (s.dn[484][11] * (nv1 - nv5));
        let eq25_e476_d_n12: f64 = (s.dn[484][12] * (nv1 - nv5));
        let eq25_e476_d_b0: f64 = (s.db[484][0] * (nv1 - nv5));
        let eq25_e476_d_b1: f64 = (s.db[484][1] * (nv1 - nv5));
        let eq25_e476_d_b2: f64 = (s.db[484][2] * (nv1 - nv5));
        let eq25_e476_d_b3: f64 = (s.db[484][3] * (nv1 - nv5));
        (eq25_e476, eq25_e476_d_n0, eq25_e476_d_n1, eq25_e476_d_n2, eq25_e476_d_n3, eq25_e476_d_n4, eq25_e476_d_n5, eq25_e476_d_n6, eq25_e476_d_n7, eq25_e476_d_n8, eq25_e476_d_n9, eq25_e476_d_n10, eq25_e476_d_n11, eq25_e476_d_n12, eq25_e476_d_b0, eq25_e476_d_b1, eq25_e476_d_b2, eq25_e476_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e478;
        let eq25_node_derivatives: [f64; 13] = [eq25_e478_d_n0, eq25_e478_d_n1, eq25_e478_d_n2, eq25_e478_d_n3, eq25_e478_d_n4, eq25_e478_d_n5, eq25_e478_d_n6, eq25_e478_d_n7, eq25_e478_d_n8, eq25_e478_d_n9, eq25_e478_d_n10, eq25_e478_d_n11, eq25_e478_d_n12];
        let eq25_branch_derivatives: [f64; 4] = [eq25_e478_d_b0, eq25_e478_d_b1, eq25_e478_d_b2, eq25_e478_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[5]),
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
        let (eq26_e483,) = {
    if (!(p.p25 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e483;
        stamper.stamp_potential(
            branches[2],
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
        let eq27_value: f64 = 0.0;
        stamper.stamp_potential(
            branches[3],
            eq27_value,
            &[
            ],
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq28_e498, eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12, eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3,) = {
    if (s.v[1094] != 0.0) {
        let eq28_e487: f64 = (-s.v[547]);
        let eq28_e487_d_n0: f64 = (-s.dn[547][0]);
        let eq28_e487_d_n1: f64 = (-s.dn[547][1]);
        let eq28_e487_d_n2: f64 = (-s.dn[547][2]);
        let eq28_e487_d_n3: f64 = (-s.dn[547][3]);
        let eq28_e487_d_n4: f64 = (-s.dn[547][4]);
        let eq28_e487_d_n5: f64 = (-s.dn[547][5]);
        let eq28_e487_d_n6: f64 = (-s.dn[547][6]);
        let eq28_e487_d_n7: f64 = (-s.dn[547][7]);
        let eq28_e487_d_n8: f64 = (-s.dn[547][8]);
        let eq28_e487_d_n9: f64 = (-s.dn[547][9]);
        let eq28_e487_d_n10: f64 = (-s.dn[547][10]);
        let eq28_e487_d_n11: f64 = (-s.dn[547][11]);
        let eq28_e487_d_n12: f64 = (-s.dn[547][12]);
        let eq28_e487_d_b0: f64 = (-s.db[547][0]);
        let eq28_e487_d_b1: f64 = (-s.db[547][1]);
        let eq28_e487_d_b2: f64 = (-s.db[547][2]);
        let eq28_e487_d_b3: f64 = (-s.db[547][3]);
        let eq28_e490: f64 = (s.v[516] * (nv4 - 0.0));
        let eq28_e490_d_n0: f64 = (s.dn[516][0] * (nv4 - 0.0));
        let eq28_e490_d_n1: f64 = (s.dn[516][1] * (nv4 - 0.0));
        let eq28_e490_d_n2: f64 = (s.dn[516][2] * (nv4 - 0.0));
        let eq28_e490_d_n3: f64 = (s.dn[516][3] * (nv4 - 0.0));
        let eq28_e490_d_n4: f64 = ((s.dn[516][4] * (nv4 - 0.0)) + s.v[516]);
        let eq28_e490_d_n5: f64 = (s.dn[516][5] * (nv4 - 0.0));
        let eq28_e490_d_n6: f64 = (s.dn[516][6] * (nv4 - 0.0));
        let eq28_e490_d_n7: f64 = (s.dn[516][7] * (nv4 - 0.0));
        let eq28_e490_d_n8: f64 = (s.dn[516][8] * (nv4 - 0.0));
        let eq28_e490_d_n9: f64 = (s.dn[516][9] * (nv4 - 0.0));
        let eq28_e490_d_n10: f64 = (s.dn[516][10] * (nv4 - 0.0));
        let eq28_e490_d_n11: f64 = (s.dn[516][11] * (nv4 - 0.0));
        let eq28_e490_d_n12: f64 = (s.dn[516][12] * (nv4 - 0.0));
        let eq28_e490_d_b0: f64 = (s.db[516][0] * (nv4 - 0.0));
        let eq28_e490_d_b1: f64 = (s.db[516][1] * (nv4 - 0.0));
        let eq28_e490_d_b2: f64 = (s.db[516][2] * (nv4 - 0.0));
        let eq28_e490_d_b3: f64 = (s.db[516][3] * (nv4 - 0.0));
        let eq28_e491: f64 = self.eval_ddt(5, eq28_e490);
        let eq28_e491_d_n0: f64 = self.ddt_jacobian(eq28_e490_d_n0);
        let eq28_e491_d_n1: f64 = self.ddt_jacobian(eq28_e490_d_n1);
        let eq28_e491_d_n2: f64 = self.ddt_jacobian(eq28_e490_d_n2);
        let eq28_e491_d_n3: f64 = self.ddt_jacobian(eq28_e490_d_n3);
        let eq28_e491_d_n4: f64 = self.ddt_jacobian(eq28_e490_d_n4);
        let eq28_e491_d_n5: f64 = self.ddt_jacobian(eq28_e490_d_n5);
        let eq28_e491_d_n6: f64 = self.ddt_jacobian(eq28_e490_d_n6);
        let eq28_e491_d_n7: f64 = self.ddt_jacobian(eq28_e490_d_n7);
        let eq28_e491_d_n8: f64 = self.ddt_jacobian(eq28_e490_d_n8);
        let eq28_e491_d_n9: f64 = self.ddt_jacobian(eq28_e490_d_n9);
        let eq28_e491_d_n10: f64 = self.ddt_jacobian(eq28_e490_d_n10);
        let eq28_e491_d_n11: f64 = self.ddt_jacobian(eq28_e490_d_n11);
        let eq28_e491_d_n12: f64 = self.ddt_jacobian(eq28_e490_d_n12);
        let eq28_e491_d_b0: f64 = self.ddt_jacobian(eq28_e490_d_b0);
        let eq28_e491_d_b1: f64 = self.ddt_jacobian(eq28_e490_d_b1);
        let eq28_e491_d_b2: f64 = self.ddt_jacobian(eq28_e490_d_b2);
        let eq28_e491_d_b3: f64 = self.ddt_jacobian(eq28_e490_d_b3);
        let eq28_e492: f64 = (eq28_e487 + eq28_e491);
        let eq28_e492_d_n0: f64 = (eq28_e487_d_n0 + eq28_e491_d_n0);
        let eq28_e492_d_n1: f64 = (eq28_e487_d_n1 + eq28_e491_d_n1);
        let eq28_e492_d_n2: f64 = (eq28_e487_d_n2 + eq28_e491_d_n2);
        let eq28_e492_d_n3: f64 = (eq28_e487_d_n3 + eq28_e491_d_n3);
        let eq28_e492_d_n4: f64 = (eq28_e487_d_n4 + eq28_e491_d_n4);
        let eq28_e492_d_n5: f64 = (eq28_e487_d_n5 + eq28_e491_d_n5);
        let eq28_e492_d_n6: f64 = (eq28_e487_d_n6 + eq28_e491_d_n6);
        let eq28_e492_d_n7: f64 = (eq28_e487_d_n7 + eq28_e491_d_n7);
        let eq28_e492_d_n8: f64 = (eq28_e487_d_n8 + eq28_e491_d_n8);
        let eq28_e492_d_n9: f64 = (eq28_e487_d_n9 + eq28_e491_d_n9);
        let eq28_e492_d_n10: f64 = (eq28_e487_d_n10 + eq28_e491_d_n10);
        let eq28_e492_d_n11: f64 = (eq28_e487_d_n11 + eq28_e491_d_n11);
        let eq28_e492_d_n12: f64 = (eq28_e487_d_n12 + eq28_e491_d_n12);
        let eq28_e492_d_b0: f64 = (eq28_e487_d_b0 + eq28_e491_d_b0);
        let eq28_e492_d_b1: f64 = (eq28_e487_d_b1 + eq28_e491_d_b1);
        let eq28_e492_d_b2: f64 = (eq28_e487_d_b2 + eq28_e491_d_b2);
        let eq28_e492_d_b3: f64 = (eq28_e487_d_b3 + eq28_e491_d_b3);
        let eq28_e495: f64 = ((nv4 - 0.0) * s.v[557]);
        let eq28_e495_d_n0: f64 = ((nv4 - 0.0) * s.dn[557][0]);
        let eq28_e495_d_n1: f64 = ((nv4 - 0.0) * s.dn[557][1]);
        let eq28_e495_d_n2: f64 = ((nv4 - 0.0) * s.dn[557][2]);
        let eq28_e495_d_n3: f64 = ((nv4 - 0.0) * s.dn[557][3]);
        let eq28_e495_d_n4: f64 = (s.v[557] + ((nv4 - 0.0) * s.dn[557][4]));
        let eq28_e495_d_n5: f64 = ((nv4 - 0.0) * s.dn[557][5]);
        let eq28_e495_d_n6: f64 = ((nv4 - 0.0) * s.dn[557][6]);
        let eq28_e495_d_n7: f64 = ((nv4 - 0.0) * s.dn[557][7]);
        let eq28_e495_d_n8: f64 = ((nv4 - 0.0) * s.dn[557][8]);
        let eq28_e495_d_n9: f64 = ((nv4 - 0.0) * s.dn[557][9]);
        let eq28_e495_d_n10: f64 = ((nv4 - 0.0) * s.dn[557][10]);
        let eq28_e495_d_n11: f64 = ((nv4 - 0.0) * s.dn[557][11]);
        let eq28_e495_d_n12: f64 = ((nv4 - 0.0) * s.dn[557][12]);
        let eq28_e495_d_b0: f64 = ((nv4 - 0.0) * s.db[557][0]);
        let eq28_e495_d_b1: f64 = ((nv4 - 0.0) * s.db[557][1]);
        let eq28_e495_d_b2: f64 = ((nv4 - 0.0) * s.db[557][2]);
        let eq28_e495_d_b3: f64 = ((nv4 - 0.0) * s.db[557][3]);
        let eq28_e496: f64 = (eq28_e492 + eq28_e495);
        let eq28_e496_d_n0: f64 = (eq28_e492_d_n0 + eq28_e495_d_n0);
        let eq28_e496_d_n1: f64 = (eq28_e492_d_n1 + eq28_e495_d_n1);
        let eq28_e496_d_n2: f64 = (eq28_e492_d_n2 + eq28_e495_d_n2);
        let eq28_e496_d_n3: f64 = (eq28_e492_d_n3 + eq28_e495_d_n3);
        let eq28_e496_d_n4: f64 = (eq28_e492_d_n4 + eq28_e495_d_n4);
        let eq28_e496_d_n5: f64 = (eq28_e492_d_n5 + eq28_e495_d_n5);
        let eq28_e496_d_n6: f64 = (eq28_e492_d_n6 + eq28_e495_d_n6);
        let eq28_e496_d_n7: f64 = (eq28_e492_d_n7 + eq28_e495_d_n7);
        let eq28_e496_d_n8: f64 = (eq28_e492_d_n8 + eq28_e495_d_n8);
        let eq28_e496_d_n9: f64 = (eq28_e492_d_n9 + eq28_e495_d_n9);
        let eq28_e496_d_n10: f64 = (eq28_e492_d_n10 + eq28_e495_d_n10);
        let eq28_e496_d_n11: f64 = (eq28_e492_d_n11 + eq28_e495_d_n11);
        let eq28_e496_d_n12: f64 = (eq28_e492_d_n12 + eq28_e495_d_n12);
        let eq28_e496_d_b0: f64 = (eq28_e492_d_b0 + eq28_e495_d_b0);
        let eq28_e496_d_b1: f64 = (eq28_e492_d_b1 + eq28_e495_d_b1);
        let eq28_e496_d_b2: f64 = (eq28_e492_d_b2 + eq28_e495_d_b2);
        let eq28_e496_d_b3: f64 = (eq28_e492_d_b3 + eq28_e495_d_b3);
        (eq28_e496, eq28_e496_d_n0, eq28_e496_d_n1, eq28_e496_d_n2, eq28_e496_d_n3, eq28_e496_d_n4, eq28_e496_d_n5, eq28_e496_d_n6, eq28_e496_d_n7, eq28_e496_d_n8, eq28_e496_d_n9, eq28_e496_d_n10, eq28_e496_d_n11, eq28_e496_d_n12, eq28_e496_d_b0, eq28_e496_d_b1, eq28_e496_d_b2, eq28_e496_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e498;
        let eq28_node_derivatives: [f64; 13] = [eq28_e498_d_n0, eq28_e498_d_n1, eq28_e498_d_n2, eq28_e498_d_n3, eq28_e498_d_n4, eq28_e498_d_n5, eq28_e498_d_n6, eq28_e498_d_n7, eq28_e498_d_n8, eq28_e498_d_n9, eq28_e498_d_n10, eq28_e498_d_n11, eq28_e498_d_n12];
        let eq28_branch_derivatives: [f64; 4] = [eq28_e498_d_b0, eq28_e498_d_b1, eq28_e498_d_b2, eq28_e498_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[4]),
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
        let (eq29_e503,) = {
    if (!(s.v[1094] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e503;
        stamper.stamp_potential(
            branches[4],
            eq29_value,
            &[
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
        let (eq30_e512, eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12, eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3,) = {
    if (s.v[1095] != 0.0) {
        let eq30_e508: f64 = (1e-9 * (nv10 - 0.0));
        let eq30_e508_d_n10: f64 = 1e-9;
        let eq30_e509: f64 = self.eval_ddt(6, eq30_e508);
        let eq30_e509_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n10: f64 = self.ddt_jacobian(eq30_e508_d_n10);
        let eq30_e509_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq30_e509_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq30_e510: f64 = (s.v[558] + eq30_e509);
        let eq30_e510_d_n0: f64 = (s.dn[558][0] + eq30_e509_d_n0);
        let eq30_e510_d_n1: f64 = (s.dn[558][1] + eq30_e509_d_n1);
        let eq30_e510_d_n2: f64 = (s.dn[558][2] + eq30_e509_d_n2);
        let eq30_e510_d_n3: f64 = (s.dn[558][3] + eq30_e509_d_n3);
        let eq30_e510_d_n4: f64 = (s.dn[558][4] + eq30_e509_d_n4);
        let eq30_e510_d_n5: f64 = (s.dn[558][5] + eq30_e509_d_n5);
        let eq30_e510_d_n6: f64 = (s.dn[558][6] + eq30_e509_d_n6);
        let eq30_e510_d_n7: f64 = (s.dn[558][7] + eq30_e509_d_n7);
        let eq30_e510_d_n8: f64 = (s.dn[558][8] + eq30_e509_d_n8);
        let eq30_e510_d_n9: f64 = (s.dn[558][9] + eq30_e509_d_n9);
        let eq30_e510_d_n10: f64 = (s.dn[558][10] + eq30_e509_d_n10);
        let eq30_e510_d_n11: f64 = (s.dn[558][11] + eq30_e509_d_n11);
        let eq30_e510_d_n12: f64 = (s.dn[558][12] + eq30_e509_d_n12);
        let eq30_e510_d_b0: f64 = (s.db[558][0] + eq30_e509_d_b0);
        let eq30_e510_d_b1: f64 = (s.db[558][1] + eq30_e509_d_b1);
        let eq30_e510_d_b2: f64 = (s.db[558][2] + eq30_e509_d_b2);
        let eq30_e510_d_b3: f64 = (s.db[558][3] + eq30_e509_d_b3);
        (eq30_e510, eq30_e510_d_n0, eq30_e510_d_n1, eq30_e510_d_n2, eq30_e510_d_n3, eq30_e510_d_n4, eq30_e510_d_n5, eq30_e510_d_n6, eq30_e510_d_n7, eq30_e510_d_n8, eq30_e510_d_n9, eq30_e510_d_n10, eq30_e510_d_n11, eq30_e510_d_n12, eq30_e510_d_b0, eq30_e510_d_b1, eq30_e510_d_b2, eq30_e510_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e512;
        let eq30_node_derivatives: [f64; 13] = [eq30_e512_d_n0, eq30_e512_d_n1, eq30_e512_d_n2, eq30_e512_d_n3, eq30_e512_d_n4, eq30_e512_d_n5, eq30_e512_d_n6, eq30_e512_d_n7, eq30_e512_d_n8, eq30_e512_d_n9, eq30_e512_d_n10, eq30_e512_d_n11, eq30_e512_d_n12];
        let eq30_branch_derivatives: [f64; 4] = [eq30_e512_d_b0, eq30_e512_d_b1, eq30_e512_d_b2, eq30_e512_d_b3];
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
        let (eq31_e517,) = {
    if (!(s.v[1095] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e517;
        stamper.stamp_potential(
            branches[5],
            eq31_value,
            &[
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq32_e526, eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12, eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3,) = {
    if (p.p24 != 0.0) {
        let eq32_e522: f64 = (1e-9 * (nv8 - 0.0));
        let eq32_e522_d_n8: f64 = 1e-9;
        let eq32_e523: f64 = self.eval_ddt(7, eq32_e522);
        let eq32_e523_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n8: f64 = self.ddt_jacobian(eq32_e522_d_n8);
        let eq32_e523_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq32_e523_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq32_e524: f64 = (s.v[549] + eq32_e523);
        let eq32_e524_d_n0: f64 = (s.dn[549][0] + eq32_e523_d_n0);
        let eq32_e524_d_n1: f64 = (s.dn[549][1] + eq32_e523_d_n1);
        let eq32_e524_d_n2: f64 = (s.dn[549][2] + eq32_e523_d_n2);
        let eq32_e524_d_n3: f64 = (s.dn[549][3] + eq32_e523_d_n3);
        let eq32_e524_d_n4: f64 = (s.dn[549][4] + eq32_e523_d_n4);
        let eq32_e524_d_n5: f64 = (s.dn[549][5] + eq32_e523_d_n5);
        let eq32_e524_d_n6: f64 = (s.dn[549][6] + eq32_e523_d_n6);
        let eq32_e524_d_n7: f64 = (s.dn[549][7] + eq32_e523_d_n7);
        let eq32_e524_d_n8: f64 = (s.dn[549][8] + eq32_e523_d_n8);
        let eq32_e524_d_n9: f64 = (s.dn[549][9] + eq32_e523_d_n9);
        let eq32_e524_d_n10: f64 = (s.dn[549][10] + eq32_e523_d_n10);
        let eq32_e524_d_n11: f64 = (s.dn[549][11] + eq32_e523_d_n11);
        let eq32_e524_d_n12: f64 = (s.dn[549][12] + eq32_e523_d_n12);
        let eq32_e524_d_b0: f64 = (s.db[549][0] + eq32_e523_d_b0);
        let eq32_e524_d_b1: f64 = (s.db[549][1] + eq32_e523_d_b1);
        let eq32_e524_d_b2: f64 = (s.db[549][2] + eq32_e523_d_b2);
        let eq32_e524_d_b3: f64 = (s.db[549][3] + eq32_e523_d_b3);
        (eq32_e524, eq32_e524_d_n0, eq32_e524_d_n1, eq32_e524_d_n2, eq32_e524_d_n3, eq32_e524_d_n4, eq32_e524_d_n5, eq32_e524_d_n6, eq32_e524_d_n7, eq32_e524_d_n8, eq32_e524_d_n9, eq32_e524_d_n10, eq32_e524_d_n11, eq32_e524_d_n12, eq32_e524_d_b0, eq32_e524_d_b1, eq32_e524_d_b2, eq32_e524_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e526;
        let eq32_node_derivatives: [f64; 13] = [eq32_e526_d_n0, eq32_e526_d_n1, eq32_e526_d_n2, eq32_e526_d_n3, eq32_e526_d_n4, eq32_e526_d_n5, eq32_e526_d_n6, eq32_e526_d_n7, eq32_e526_d_n8, eq32_e526_d_n9, eq32_e526_d_n10, eq32_e526_d_n11, eq32_e526_d_n12];
        let eq32_branch_derivatives: [f64; 4] = [eq32_e526_d_b0, eq32_e526_d_b1, eq32_e526_d_b2, eq32_e526_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            None,
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq33_e535, eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12, eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3,) = {
    if (p.p24 != 0.0) {
        let eq33_e531: f64 = (1e-9 * (nv9 - 0.0));
        let eq33_e531_d_n9: f64 = 1e-9;
        let eq33_e532: f64 = self.eval_ddt(8, eq33_e531);
        let eq33_e532_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n9: f64 = self.ddt_jacobian(eq33_e531_d_n9);
        let eq33_e532_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_b0: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_b1: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_b2: f64 = self.ddt_jacobian(0.0);
        let eq33_e532_d_b3: f64 = self.ddt_jacobian(0.0);
        let eq33_e533: f64 = (s.v[550] + eq33_e532);
        let eq33_e533_d_n0: f64 = (s.dn[550][0] + eq33_e532_d_n0);
        let eq33_e533_d_n1: f64 = (s.dn[550][1] + eq33_e532_d_n1);
        let eq33_e533_d_n2: f64 = (s.dn[550][2] + eq33_e532_d_n2);
        let eq33_e533_d_n3: f64 = (s.dn[550][3] + eq33_e532_d_n3);
        let eq33_e533_d_n4: f64 = (s.dn[550][4] + eq33_e532_d_n4);
        let eq33_e533_d_n5: f64 = (s.dn[550][5] + eq33_e532_d_n5);
        let eq33_e533_d_n6: f64 = (s.dn[550][6] + eq33_e532_d_n6);
        let eq33_e533_d_n7: f64 = (s.dn[550][7] + eq33_e532_d_n7);
        let eq33_e533_d_n8: f64 = (s.dn[550][8] + eq33_e532_d_n8);
        let eq33_e533_d_n9: f64 = (s.dn[550][9] + eq33_e532_d_n9);
        let eq33_e533_d_n10: f64 = (s.dn[550][10] + eq33_e532_d_n10);
        let eq33_e533_d_n11: f64 = (s.dn[550][11] + eq33_e532_d_n11);
        let eq33_e533_d_n12: f64 = (s.dn[550][12] + eq33_e532_d_n12);
        let eq33_e533_d_b0: f64 = (s.db[550][0] + eq33_e532_d_b0);
        let eq33_e533_d_b1: f64 = (s.db[550][1] + eq33_e532_d_b1);
        let eq33_e533_d_b2: f64 = (s.db[550][2] + eq33_e532_d_b2);
        let eq33_e533_d_b3: f64 = (s.db[550][3] + eq33_e532_d_b3);
        (eq33_e533, eq33_e533_d_n0, eq33_e533_d_n1, eq33_e533_d_n2, eq33_e533_d_n3, eq33_e533_d_n4, eq33_e533_d_n5, eq33_e533_d_n6, eq33_e533_d_n7, eq33_e533_d_n8, eq33_e533_d_n9, eq33_e533_d_n10, eq33_e533_d_n11, eq33_e533_d_n12, eq33_e533_d_b0, eq33_e533_d_b1, eq33_e533_d_b2, eq33_e533_d_b3,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e535;
        let eq33_node_derivatives: [f64; 13] = [eq33_e535_d_n0, eq33_e535_d_n1, eq33_e535_d_n2, eq33_e535_d_n3, eq33_e535_d_n4, eq33_e535_d_n5, eq33_e535_d_n6, eq33_e535_d_n7, eq33_e535_d_n8, eq33_e535_d_n9, eq33_e535_d_n10, eq33_e535_d_n11, eq33_e535_d_n12];
        let eq33_branch_derivatives: [f64; 4] = [eq33_e535_d_b0, eq33_e535_d_b1, eq33_e535_d_b2, eq33_e535_d_b3];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            None,
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
        let (eq34_e540,) = {
    if (!(p.p24 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e540;
        stamper.stamp_potential(
            branches[6],
            eq34_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq35_e545,) = {
    if (!(p.p24 != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e545;
        stamper.stamp_potential(
            branches[7],
            eq35_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_10_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq10_e387: f64 = (s.v[561] + s.v[554]);
        let eq10_e387_d_n0: f64 = (s.dn[561][0] + s.dn[554][0]);
        let eq10_e387_d_n1: f64 = (s.dn[561][1] + s.dn[554][1]);
        let eq10_e387_d_n2: f64 = (s.dn[561][2] + s.dn[554][2]);
        let eq10_e387_d_n3: f64 = (s.dn[561][3] + s.dn[554][3]);
        let eq10_e387_d_n4: f64 = (s.dn[561][4] + s.dn[554][4]);
        let eq10_e387_d_n5: f64 = (s.dn[561][5] + s.dn[554][5]);
        let eq10_e387_d_n6: f64 = (s.dn[561][6] + s.dn[554][6]);
        let eq10_e387_d_n7: f64 = (s.dn[561][7] + s.dn[554][7]);
        let eq10_e387_d_n8: f64 = (s.dn[561][8] + s.dn[554][8]);
        let eq10_e387_d_n9: f64 = (s.dn[561][9] + s.dn[554][9]);
        let eq10_e387_d_n10: f64 = (s.dn[561][10] + s.dn[554][10]);
        let eq10_e387_d_n11: f64 = (s.dn[561][11] + s.dn[554][11]);
        let eq10_e387_d_n12: f64 = (s.dn[561][12] + s.dn[554][12]);
        let eq10_e387_d_b0: f64 = (s.db[561][0] + s.db[554][0]);
        let eq10_e387_d_b1: f64 = (s.db[561][1] + s.db[554][1]);
        let eq10_e387_d_b2: f64 = (s.db[561][2] + s.db[554][2]);
        let eq10_e387_d_b3: f64 = (s.db[561][3] + s.db[554][3]);
        let eq10_e388_q: f64 = eq10_e387;
        let eq10_e389: f64 = (p.p33 * eq10_e387);
        let eq10_e389_d_n0: f64 = (p.p33 * eq10_e387_d_n0);
        let eq10_e389_d_n1: f64 = (p.p33 * eq10_e387_d_n1);
        let eq10_e389_d_n2: f64 = (p.p33 * eq10_e387_d_n2);
        let eq10_e389_d_n3: f64 = (p.p33 * eq10_e387_d_n3);
        let eq10_e389_d_n4: f64 = (p.p33 * eq10_e387_d_n4);
        let eq10_e389_d_n5: f64 = (p.p33 * eq10_e387_d_n5);
        let eq10_e389_d_n6: f64 = (p.p33 * eq10_e387_d_n6);
        let eq10_e389_d_n7: f64 = (p.p33 * eq10_e387_d_n7);
        let eq10_e389_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_d_n9: f64 = (p.p33 * eq10_e387_d_n9);
        let eq10_e389_d_n10: f64 = (p.p33 * eq10_e387_d_n10);
        let eq10_e389_d_n11: f64 = (p.p33 * eq10_e387_d_n11);
        let eq10_e389_d_n12: f64 = (p.p33 * eq10_e387_d_n12);
        let eq10_e389_d_b0: f64 = (p.p33 * eq10_e387_d_b0);
        let eq10_e389_d_b1: f64 = (p.p33 * eq10_e387_d_b1);
        let eq10_e389_d_b2: f64 = (p.p33 * eq10_e387_d_b2);
        let eq10_e389_d_b3: f64 = (p.p33 * eq10_e387_d_b3);
        let eq10_e389_q: f64 = (p.p33 * eq10_e388_q);
        let eq10_e389_q_d_n0: f64 = (p.p33 * eq10_e387_d_n0);
        let eq10_e389_q_d_n1: f64 = (p.p33 * eq10_e387_d_n1);
        let eq10_e389_q_d_n2: f64 = (p.p33 * eq10_e387_d_n2);
        let eq10_e389_q_d_n3: f64 = (p.p33 * eq10_e387_d_n3);
        let eq10_e389_q_d_n4: f64 = (p.p33 * eq10_e387_d_n4);
        let eq10_e389_q_d_n5: f64 = (p.p33 * eq10_e387_d_n5);
        let eq10_e389_q_d_n6: f64 = (p.p33 * eq10_e387_d_n6);
        let eq10_e389_q_d_n7: f64 = (p.p33 * eq10_e387_d_n7);
        let eq10_e389_q_d_n8: f64 = (p.p33 * eq10_e387_d_n8);
        let eq10_e389_q_d_n9: f64 = (p.p33 * eq10_e387_d_n9);
        let eq10_e389_q_d_n10: f64 = (p.p33 * eq10_e387_d_n10);
        let eq10_e389_q_d_n11: f64 = (p.p33 * eq10_e387_d_n11);
        let eq10_e389_q_d_n12: f64 = (p.p33 * eq10_e387_d_n12);
        let eq10_e389_q_d_b0: f64 = (p.p33 * eq10_e387_d_b0);
        let eq10_e389_q_d_b1: f64 = (p.p33 * eq10_e387_d_b1);
        let eq10_e389_q_d_b2: f64 = (p.p33 * eq10_e387_d_b2);
        let eq10_e389_q_d_b3: f64 = (p.p33 * eq10_e387_d_b3);
        let eq10_reactive_node_derivatives: [f64; 13] = [eq10_e389_q_d_n0, eq10_e389_q_d_n1, eq10_e389_q_d_n2, eq10_e389_q_d_n3, eq10_e389_q_d_n4, eq10_e389_q_d_n5, eq10_e389_q_d_n6, eq10_e389_q_d_n7, eq10_e389_q_d_n8, eq10_e389_q_d_n9, eq10_e389_q_d_n10, eq10_e389_q_d_n11, eq10_e389_q_d_n12];
        let eq10_reactive_branch_derivatives: [f64; 4] = [eq10_e389_q_d_b0, eq10_e389_q_d_b1, eq10_e389_q_d_b2, eq10_e389_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[12]),
            &nodes,
            &eq10_reactive_node_derivatives,
            &branches,
            &eq10_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_11_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq11_e393: f64 = (s.v[93] + s.v[552]);
        let eq11_e393_d_n0: f64 = (s.dn[93][0] + s.dn[552][0]);
        let eq11_e393_d_n1: f64 = (s.dn[93][1] + s.dn[552][1]);
        let eq11_e393_d_n2: f64 = (s.dn[93][2] + s.dn[552][2]);
        let eq11_e393_d_n3: f64 = (s.dn[93][3] + s.dn[552][3]);
        let eq11_e393_d_n4: f64 = (s.dn[93][4] + s.dn[552][4]);
        let eq11_e393_d_n5: f64 = (s.dn[93][5] + s.dn[552][5]);
        let eq11_e393_d_n6: f64 = (s.dn[93][6] + s.dn[552][6]);
        let eq11_e393_d_n7: f64 = (s.dn[93][7] + s.dn[552][7]);
        let eq11_e393_d_n8: f64 = (s.dn[93][8] + s.dn[552][8]);
        let eq11_e393_d_n9: f64 = (s.dn[93][9] + s.dn[552][9]);
        let eq11_e393_d_n10: f64 = (s.dn[93][10] + s.dn[552][10]);
        let eq11_e393_d_n11: f64 = (s.dn[93][11] + s.dn[552][11]);
        let eq11_e393_d_n12: f64 = (s.dn[93][12] + s.dn[552][12]);
        let eq11_e393_d_b0: f64 = (s.db[93][0] + s.db[552][0]);
        let eq11_e393_d_b1: f64 = (s.db[93][1] + s.db[552][1]);
        let eq11_e393_d_b2: f64 = (s.db[93][2] + s.db[552][2]);
        let eq11_e393_d_b3: f64 = (s.db[93][3] + s.db[552][3]);
        let eq11_e394_q: f64 = eq11_e393;
        let eq11_e395: f64 = (p.p33 * eq11_e393);
        let eq11_e395_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_d_n1: f64 = (p.p33 * eq11_e393_d_n1);
        let eq11_e395_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_d_n3: f64 = (p.p33 * eq11_e393_d_n3);
        let eq11_e395_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_d_n7: f64 = (p.p33 * eq11_e393_d_n7);
        let eq11_e395_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_d_n9: f64 = (p.p33 * eq11_e393_d_n9);
        let eq11_e395_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_d_b0: f64 = (p.p33 * eq11_e393_d_b0);
        let eq11_e395_d_b1: f64 = (p.p33 * eq11_e393_d_b1);
        let eq11_e395_d_b2: f64 = (p.p33 * eq11_e393_d_b2);
        let eq11_e395_d_b3: f64 = (p.p33 * eq11_e393_d_b3);
        let eq11_e395_q: f64 = (p.p33 * eq11_e394_q);
        let eq11_e395_q_d_n0: f64 = (p.p33 * eq11_e393_d_n0);
        let eq11_e395_q_d_n1: f64 = (p.p33 * eq11_e393_d_n1);
        let eq11_e395_q_d_n2: f64 = (p.p33 * eq11_e393_d_n2);
        let eq11_e395_q_d_n3: f64 = (p.p33 * eq11_e393_d_n3);
        let eq11_e395_q_d_n4: f64 = (p.p33 * eq11_e393_d_n4);
        let eq11_e395_q_d_n5: f64 = (p.p33 * eq11_e393_d_n5);
        let eq11_e395_q_d_n6: f64 = (p.p33 * eq11_e393_d_n6);
        let eq11_e395_q_d_n7: f64 = (p.p33 * eq11_e393_d_n7);
        let eq11_e395_q_d_n8: f64 = (p.p33 * eq11_e393_d_n8);
        let eq11_e395_q_d_n9: f64 = (p.p33 * eq11_e393_d_n9);
        let eq11_e395_q_d_n10: f64 = (p.p33 * eq11_e393_d_n10);
        let eq11_e395_q_d_n11: f64 = (p.p33 * eq11_e393_d_n11);
        let eq11_e395_q_d_n12: f64 = (p.p33 * eq11_e393_d_n12);
        let eq11_e395_q_d_b0: f64 = (p.p33 * eq11_e393_d_b0);
        let eq11_e395_q_d_b1: f64 = (p.p33 * eq11_e393_d_b1);
        let eq11_e395_q_d_b2: f64 = (p.p33 * eq11_e393_d_b2);
        let eq11_e395_q_d_b3: f64 = (p.p33 * eq11_e393_d_b3);
        let eq11_reactive_node_derivatives: [f64; 13] = [eq11_e395_q_d_n0, eq11_e395_q_d_n1, eq11_e395_q_d_n2, eq11_e395_q_d_n3, eq11_e395_q_d_n4, eq11_e395_q_d_n5, eq11_e395_q_d_n6, eq11_e395_q_d_n7, eq11_e395_q_d_n8, eq11_e395_q_d_n9, eq11_e395_q_d_n10, eq11_e395_q_d_n11, eq11_e395_q_d_n12];
        let eq11_reactive_branch_derivatives: [f64; 4] = [eq11_e395_q_d_b0, eq11_e395_q_d_b1, eq11_e395_q_d_b2, eq11_e395_q_d_b3];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[12]),
            &nodes,
            &eq11_reactive_node_derivatives,
            &branches,
            &eq11_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

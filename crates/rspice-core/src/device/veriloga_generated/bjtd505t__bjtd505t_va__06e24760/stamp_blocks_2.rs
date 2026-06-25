#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq23_e324: f64 = (s.v[222] + s.v[235]);
        let eq23_e324_d_n0: f64 = (s.dn[222][0] + s.dn[235][0]);
        let eq23_e324_d_n1: f64 = (s.dn[222][1] + s.dn[235][1]);
        let eq23_e324_d_n2: f64 = (s.dn[222][2] + s.dn[235][2]);
        let eq23_e324_d_n3: f64 = (s.dn[222][3] + s.dn[235][3]);
        let eq23_e324_d_n4: f64 = (s.dn[222][4] + s.dn[235][4]);
        let eq23_e324_d_n5: f64 = (s.dn[222][5] + s.dn[235][5]);
        let eq23_e324_d_n6: f64 = (s.dn[222][6] + s.dn[235][6]);
        let eq23_e324_d_n7: f64 = (s.dn[222][7] + s.dn[235][7]);
        let eq23_e324_d_n8: f64 = (s.dn[222][8] + s.dn[235][8]);
        let eq23_e324_d_n9: f64 = (s.dn[222][9] + s.dn[235][9]);
        let eq23_e324_d_n10: f64 = (s.dn[222][10] + s.dn[235][10]);
        let eq23_e324_d_n11: f64 = (s.dn[222][11] + s.dn[235][11]);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
        let eq23_e326: f64 = self.eval_ddt(8, eq23_e325);
        let eq23_e326_d_n0: f64 = self.ddt_jacobian(eq23_e325_d_n0);
        let eq23_e326_d_n1: f64 = self.ddt_jacobian(eq23_e325_d_n1);
        let eq23_e326_d_n2: f64 = self.ddt_jacobian(eq23_e325_d_n2);
        let eq23_e326_d_n3: f64 = self.ddt_jacobian(eq23_e325_d_n3);
        let eq23_e326_d_n4: f64 = self.ddt_jacobian(eq23_e325_d_n4);
        let eq23_e326_d_n5: f64 = self.ddt_jacobian(eq23_e325_d_n5);
        let eq23_e326_d_n6: f64 = self.ddt_jacobian(eq23_e325_d_n6);
        let eq23_e326_d_n7: f64 = self.ddt_jacobian(eq23_e325_d_n7);
        let eq23_e326_d_n8: f64 = self.ddt_jacobian(eq23_e325_d_n8);
        let eq23_e326_d_n9: f64 = self.ddt_jacobian(eq23_e325_d_n9);
        let eq23_e326_d_n10: f64 = self.ddt_jacobian(eq23_e325_d_n10);
        let eq23_e326_d_n11: f64 = self.ddt_jacobian(eq23_e325_d_n11);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e326_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e326_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e326_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e326_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e326_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e326_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e326_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e326_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e326_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e326_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e326_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e326_d_n11 * p.p1);
        let eq23_value: f64 = eq23_e328;
        let eq23_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[10]),
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
        let (eq24_e338, eq24_e338_d_n0, eq24_e338_d_n1, eq24_e338_d_n2, eq24_e338_d_n3, eq24_e338_d_n4, eq24_e338_d_n5, eq24_e338_d_n6, eq24_e338_d_n7, eq24_e338_d_n8, eq24_e338_d_n9, eq24_e338_d_n10, eq24_e338_d_n11,) = {
    if (s.v[567] != 0.0) {
        let eq24_e332: f64 = (p.p3 * s.v[243]);
        let eq24_e332_d_n0: f64 = (p.p3 * s.dn[243][0]);
        let eq24_e332_d_n1: f64 = (p.p3 * s.dn[243][1]);
        let eq24_e332_d_n2: f64 = (p.p3 * s.dn[243][2]);
        let eq24_e332_d_n3: f64 = (p.p3 * s.dn[243][3]);
        let eq24_e332_d_n4: f64 = (p.p3 * s.dn[243][4]);
        let eq24_e332_d_n5: f64 = (p.p3 * s.dn[243][5]);
        let eq24_e332_d_n6: f64 = (p.p3 * s.dn[243][6]);
        let eq24_e332_d_n7: f64 = (p.p3 * s.dn[243][7]);
        let eq24_e332_d_n8: f64 = (p.p3 * s.dn[243][8]);
        let eq24_e332_d_n9: f64 = (p.p3 * s.dn[243][9]);
        let eq24_e332_d_n10: f64 = (p.p3 * s.dn[243][10]);
        let eq24_e332_d_n11: f64 = (p.p3 * s.dn[243][11]);
        let eq24_e334: f64 = (eq24_e332 * s.v[105]);
        let eq24_e334_d_n0: f64 = ((eq24_e332_d_n0 * s.v[105]) + (eq24_e332 * s.dn[105][0]));
        let eq24_e334_d_n1: f64 = ((eq24_e332_d_n1 * s.v[105]) + (eq24_e332 * s.dn[105][1]));
        let eq24_e334_d_n2: f64 = ((eq24_e332_d_n2 * s.v[105]) + (eq24_e332 * s.dn[105][2]));
        let eq24_e334_d_n3: f64 = ((eq24_e332_d_n3 * s.v[105]) + (eq24_e332 * s.dn[105][3]));
        let eq24_e334_d_n4: f64 = ((eq24_e332_d_n4 * s.v[105]) + (eq24_e332 * s.dn[105][4]));
        let eq24_e334_d_n5: f64 = ((eq24_e332_d_n5 * s.v[105]) + (eq24_e332 * s.dn[105][5]));
        let eq24_e334_d_n6: f64 = ((eq24_e332_d_n6 * s.v[105]) + (eq24_e332 * s.dn[105][6]));
        let eq24_e334_d_n7: f64 = ((eq24_e332_d_n7 * s.v[105]) + (eq24_e332 * s.dn[105][7]));
        let eq24_e334_d_n8: f64 = ((eq24_e332_d_n8 * s.v[105]) + (eq24_e332 * s.dn[105][8]));
        let eq24_e334_d_n9: f64 = ((eq24_e332_d_n9 * s.v[105]) + (eq24_e332 * s.dn[105][9]));
        let eq24_e334_d_n10: f64 = ((eq24_e332_d_n10 * s.v[105]) + (eq24_e332 * s.dn[105][10]));
        let eq24_e334_d_n11: f64 = ((eq24_e332_d_n11 * s.v[105]) + (eq24_e332 * s.dn[105][11]));
        let eq24_e336: f64 = (eq24_e334 * p.p1);
        let eq24_e336_d_n0: f64 = (eq24_e334_d_n0 * p.p1);
        let eq24_e336_d_n1: f64 = (eq24_e334_d_n1 * p.p1);
        let eq24_e336_d_n2: f64 = (eq24_e334_d_n2 * p.p1);
        let eq24_e336_d_n3: f64 = (eq24_e334_d_n3 * p.p1);
        let eq24_e336_d_n4: f64 = (eq24_e334_d_n4 * p.p1);
        let eq24_e336_d_n5: f64 = (eq24_e334_d_n5 * p.p1);
        let eq24_e336_d_n6: f64 = (eq24_e334_d_n6 * p.p1);
        let eq24_e336_d_n7: f64 = (eq24_e334_d_n7 * p.p1);
        let eq24_e336_d_n8: f64 = (eq24_e334_d_n8 * p.p1);
        let eq24_e336_d_n9: f64 = (eq24_e334_d_n9 * p.p1);
        let eq24_e336_d_n10: f64 = (eq24_e334_d_n10 * p.p1);
        let eq24_e336_d_n11: f64 = (eq24_e334_d_n11 * p.p1);
        (eq24_e336, eq24_e336_d_n0, eq24_e336_d_n1, eq24_e336_d_n2, eq24_e336_d_n3, eq24_e336_d_n4, eq24_e336_d_n5, eq24_e336_d_n6, eq24_e336_d_n7, eq24_e336_d_n8, eq24_e336_d_n9, eq24_e336_d_n10, eq24_e336_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e338;
        let eq24_node_derivatives: [f64; 12] = [eq24_e338_d_n0, eq24_e338_d_n1, eq24_e338_d_n2, eq24_e338_d_n3, eq24_e338_d_n4, eq24_e338_d_n5, eq24_e338_d_n6, eq24_e338_d_n7, eq24_e338_d_n8, eq24_e338_d_n9, eq24_e338_d_n10, eq24_e338_d_n11];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[10]),
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
        let (eq25_e343,) = {
    if (!(s.v[567] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e343;
        stamper.stamp_potential(
            branches[0],
            eq25_value,
            &[
            ],
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
        let (eq26_e353, eq26_e353_d_n0, eq26_e353_d_n1, eq26_e353_d_n2, eq26_e353_d_n3, eq26_e353_d_n4, eq26_e353_d_n5, eq26_e353_d_n6, eq26_e353_d_n7, eq26_e353_d_n8, eq26_e353_d_n9, eq26_e353_d_n10, eq26_e353_d_n11,) = {
    if (s.v[568] != 0.0) {
        let eq26_e347: f64 = (p.p3 * s.v[244]);
        let eq26_e347_d_n0: f64 = (p.p3 * s.dn[244][0]);
        let eq26_e347_d_n1: f64 = (p.p3 * s.dn[244][1]);
        let eq26_e347_d_n2: f64 = (p.p3 * s.dn[244][2]);
        let eq26_e347_d_n3: f64 = (p.p3 * s.dn[244][3]);
        let eq26_e347_d_n4: f64 = (p.p3 * s.dn[244][4]);
        let eq26_e347_d_n5: f64 = (p.p3 * s.dn[244][5]);
        let eq26_e347_d_n6: f64 = (p.p3 * s.dn[244][6]);
        let eq26_e347_d_n7: f64 = (p.p3 * s.dn[244][7]);
        let eq26_e347_d_n8: f64 = (p.p3 * s.dn[244][8]);
        let eq26_e347_d_n9: f64 = (p.p3 * s.dn[244][9]);
        let eq26_e347_d_n10: f64 = (p.p3 * s.dn[244][10]);
        let eq26_e347_d_n11: f64 = (p.p3 * s.dn[244][11]);
        let eq26_e349: f64 = (eq26_e347 * s.v[106]);
        let eq26_e349_d_n0: f64 = ((eq26_e347_d_n0 * s.v[106]) + (eq26_e347 * s.dn[106][0]));
        let eq26_e349_d_n1: f64 = ((eq26_e347_d_n1 * s.v[106]) + (eq26_e347 * s.dn[106][1]));
        let eq26_e349_d_n2: f64 = ((eq26_e347_d_n2 * s.v[106]) + (eq26_e347 * s.dn[106][2]));
        let eq26_e349_d_n3: f64 = ((eq26_e347_d_n3 * s.v[106]) + (eq26_e347 * s.dn[106][3]));
        let eq26_e349_d_n4: f64 = ((eq26_e347_d_n4 * s.v[106]) + (eq26_e347 * s.dn[106][4]));
        let eq26_e349_d_n5: f64 = ((eq26_e347_d_n5 * s.v[106]) + (eq26_e347 * s.dn[106][5]));
        let eq26_e349_d_n6: f64 = ((eq26_e347_d_n6 * s.v[106]) + (eq26_e347 * s.dn[106][6]));
        let eq26_e349_d_n7: f64 = ((eq26_e347_d_n7 * s.v[106]) + (eq26_e347 * s.dn[106][7]));
        let eq26_e349_d_n8: f64 = ((eq26_e347_d_n8 * s.v[106]) + (eq26_e347 * s.dn[106][8]));
        let eq26_e349_d_n9: f64 = ((eq26_e347_d_n9 * s.v[106]) + (eq26_e347 * s.dn[106][9]));
        let eq26_e349_d_n10: f64 = ((eq26_e347_d_n10 * s.v[106]) + (eq26_e347 * s.dn[106][10]));
        let eq26_e349_d_n11: f64 = ((eq26_e347_d_n11 * s.v[106]) + (eq26_e347 * s.dn[106][11]));
        let eq26_e351: f64 = (eq26_e349 * p.p1);
        let eq26_e351_d_n0: f64 = (eq26_e349_d_n0 * p.p1);
        let eq26_e351_d_n1: f64 = (eq26_e349_d_n1 * p.p1);
        let eq26_e351_d_n2: f64 = (eq26_e349_d_n2 * p.p1);
        let eq26_e351_d_n3: f64 = (eq26_e349_d_n3 * p.p1);
        let eq26_e351_d_n4: f64 = (eq26_e349_d_n4 * p.p1);
        let eq26_e351_d_n5: f64 = (eq26_e349_d_n5 * p.p1);
        let eq26_e351_d_n6: f64 = (eq26_e349_d_n6 * p.p1);
        let eq26_e351_d_n7: f64 = (eq26_e349_d_n7 * p.p1);
        let eq26_e351_d_n8: f64 = (eq26_e349_d_n8 * p.p1);
        let eq26_e351_d_n9: f64 = (eq26_e349_d_n9 * p.p1);
        let eq26_e351_d_n10: f64 = (eq26_e349_d_n10 * p.p1);
        let eq26_e351_d_n11: f64 = (eq26_e349_d_n11 * p.p1);
        (eq26_e351, eq26_e351_d_n0, eq26_e351_d_n1, eq26_e351_d_n2, eq26_e351_d_n3, eq26_e351_d_n4, eq26_e351_d_n5, eq26_e351_d_n6, eq26_e351_d_n7, eq26_e351_d_n8, eq26_e351_d_n9, eq26_e351_d_n10, eq26_e351_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e353;
        let eq26_node_derivatives: [f64; 12] = [eq26_e353_d_n0, eq26_e353_d_n1, eq26_e353_d_n2, eq26_e353_d_n3, eq26_e353_d_n4, eq26_e353_d_n5, eq26_e353_d_n6, eq26_e353_d_n7, eq26_e353_d_n8, eq26_e353_d_n9, eq26_e353_d_n10, eq26_e353_d_n11];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[7]),
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
        let (eq27_e358,) = {
    if (!(s.v[568] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq27_value: f64 = eq27_e358;
        stamper.stamp_potential(
            branches[1],
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
        let eq28_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            None,
            self.multiplicity * (eq28_value),
            &[
            ],
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq29_value: f64 = (nv11 - 0.0);
        stamper.stamp_current(
            Some(nodes[11]),
            None,
            self.multiplicity * (eq29_value),
            &[
                GeneratedDerivative::node(nodes[11], self.multiplicity * 1.0),
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq30_e367: f64 = self.eval_ddt(9, (nv11 - 0.0));
        let eq30_e367_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq30_e367_d_n11: f64 = self.ddt_jacobian(1.0);
        let eq30_e368: f64 = (s.v[312] * eq30_e367);
        let eq30_e368_d_n0: f64 = ((s.dn[312][0] * eq30_e367) + (s.v[312] * eq30_e367_d_n0));
        let eq30_e368_d_n1: f64 = ((s.dn[312][1] * eq30_e367) + (s.v[312] * eq30_e367_d_n1));
        let eq30_e368_d_n2: f64 = ((s.dn[312][2] * eq30_e367) + (s.v[312] * eq30_e367_d_n2));
        let eq30_e368_d_n3: f64 = ((s.dn[312][3] * eq30_e367) + (s.v[312] * eq30_e367_d_n3));
        let eq30_e368_d_n4: f64 = ((s.dn[312][4] * eq30_e367) + (s.v[312] * eq30_e367_d_n4));
        let eq30_e368_d_n5: f64 = ((s.dn[312][5] * eq30_e367) + (s.v[312] * eq30_e367_d_n5));
        let eq30_e368_d_n6: f64 = ((s.dn[312][6] * eq30_e367) + (s.v[312] * eq30_e367_d_n6));
        let eq30_e368_d_n7: f64 = ((s.dn[312][7] * eq30_e367) + (s.v[312] * eq30_e367_d_n7));
        let eq30_e368_d_n8: f64 = ((s.dn[312][8] * eq30_e367) + (s.v[312] * eq30_e367_d_n8));
        let eq30_e368_d_n9: f64 = ((s.dn[312][9] * eq30_e367) + (s.v[312] * eq30_e367_d_n9));
        let eq30_e368_d_n10: f64 = ((s.dn[312][10] * eq30_e367) + (s.v[312] * eq30_e367_d_n10));
        let eq30_e368_d_n11: f64 = ((s.dn[312][11] * eq30_e367) + (s.v[312] * eq30_e367_d_n11));
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivatives: [f64; 12] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n2, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, eq30_e368_d_n11];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[4]),
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
        let eq31_e371: f64 = (s.v[310] * (nv11 - 0.0));
        let eq31_e371_d_n0: f64 = (s.dn[310][0] * (nv11 - 0.0));
        let eq31_e371_d_n1: f64 = (s.dn[310][1] * (nv11 - 0.0));
        let eq31_e371_d_n2: f64 = (s.dn[310][2] * (nv11 - 0.0));
        let eq31_e371_d_n3: f64 = (s.dn[310][3] * (nv11 - 0.0));
        let eq31_e371_d_n4: f64 = (s.dn[310][4] * (nv11 - 0.0));
        let eq31_e371_d_n5: f64 = (s.dn[310][5] * (nv11 - 0.0));
        let eq31_e371_d_n6: f64 = (s.dn[310][6] * (nv11 - 0.0));
        let eq31_e371_d_n7: f64 = (s.dn[310][7] * (nv11 - 0.0));
        let eq31_e371_d_n8: f64 = (s.dn[310][8] * (nv11 - 0.0));
        let eq31_e371_d_n9: f64 = (s.dn[310][9] * (nv11 - 0.0));
        let eq31_e371_d_n10: f64 = (s.dn[310][10] * (nv11 - 0.0));
        let eq31_e371_d_n11: f64 = ((s.dn[310][11] * (nv11 - 0.0)) + s.v[310]);
        let eq31_value: f64 = eq31_e371;
        let eq31_node_derivatives: [f64; 12] = [eq31_e371_d_n0, eq31_e371_d_n1, eq31_e371_d_n2, eq31_e371_d_n3, eq31_e371_d_n4, eq31_e371_d_n5, eq31_e371_d_n6, eq31_e371_d_n7, eq31_e371_d_n8, eq31_e371_d_n9, eq31_e371_d_n10, eq31_e371_d_n11];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
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
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq32_value: f64 = (nv11 - 0.0);
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[4]),
            self.multiplicity * (eq32_value),
            &[
                GeneratedDerivative::node(nodes[11], self.multiplicity * 1.0),
            ],
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
        let eq33_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq33_value),
            &[
            ],
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
        let eq34_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[4]),
            self.multiplicity * (eq34_value),
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
        let eq35_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[4]),
            self.multiplicity * (eq35_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_36_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq36_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[1]),
            Some(nodes[5]),
            self.multiplicity * (eq36_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_37_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq37_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq37_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_38_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq38_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[4]),
            self.multiplicity * (eq38_value),
            &[
            ],
        );
    }
}

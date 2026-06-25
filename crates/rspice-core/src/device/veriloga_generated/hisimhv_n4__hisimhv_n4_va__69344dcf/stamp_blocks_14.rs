#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq34_e1200: f64 = (-p.p87);
        let eq34_e1202: f64 = self.eval_ddt(14, s.v[301]);
        let eq34_e1202_d_n0: f64 = self.ddt_jacobian(s.dn[301][0]);
        let eq34_e1202_d_n1: f64 = self.ddt_jacobian(s.dn[301][1]);
        let eq34_e1202_d_n2: f64 = self.ddt_jacobian(s.dn[301][2]);
        let eq34_e1202_d_n3: f64 = self.ddt_jacobian(s.dn[301][3]);
        let eq34_e1202_d_n4: f64 = self.ddt_jacobian(s.dn[301][4]);
        let eq34_e1202_d_n5: f64 = self.ddt_jacobian(s.dn[301][5]);
        let eq34_e1202_d_n6: f64 = self.ddt_jacobian(s.dn[301][6]);
        let eq34_e1202_d_n7: f64 = self.ddt_jacobian(s.dn[301][7]);
        let eq34_e1202_d_n8: f64 = self.ddt_jacobian(s.dn[301][8]);
        let eq34_e1202_d_n9: f64 = self.ddt_jacobian(s.dn[301][9]);
        let eq34_e1202_d_n10: f64 = self.ddt_jacobian(s.dn[301][10]);
        let eq34_e1202_d_n11: f64 = self.ddt_jacobian(s.dn[301][11]);
        let eq34_e1202_d_n12: f64 = self.ddt_jacobian(s.dn[301][12]);
        let eq34_e1202_d_n13: f64 = self.ddt_jacobian(s.dn[301][13]);
        let eq34_e1202_d_n14: f64 = self.ddt_jacobian(s.dn[301][14]);
        let eq34_e1202_d_n15: f64 = self.ddt_jacobian(s.dn[301][15]);
        let eq34_e1202_d_n16: f64 = self.ddt_jacobian(s.dn[301][16]);
        let eq34_e1202_d_n17: f64 = self.ddt_jacobian(s.dn[301][17]);
        let eq34_e1202_d_b0: f64 = self.ddt_jacobian(s.db[301][0]);
        let eq34_e1202_d_b1: f64 = self.ddt_jacobian(s.db[301][1]);
        let eq34_e1202_d_b2: f64 = self.ddt_jacobian(s.db[301][2]);
        let eq34_e1202_d_b3: f64 = self.ddt_jacobian(s.db[301][3]);
        let eq34_e1202_d_b4: f64 = self.ddt_jacobian(s.db[301][4]);
        let eq34_e1202_d_b5: f64 = self.ddt_jacobian(s.db[301][5]);
        let eq34_e1202_d_b6: f64 = self.ddt_jacobian(s.db[301][6]);
        let eq34_e1202_d_b7: f64 = self.ddt_jacobian(s.db[301][7]);
        let eq34_e1202_d_b8: f64 = self.ddt_jacobian(s.db[301][8]);
        let eq34_e1203: f64 = (eq34_e1200 * eq34_e1202);
        let eq34_e1203_d_n0: f64 = (eq34_e1200 * eq34_e1202_d_n0);
        let eq34_e1203_d_n1: f64 = (eq34_e1200 * eq34_e1202_d_n1);
        let eq34_e1203_d_n2: f64 = (eq34_e1200 * eq34_e1202_d_n2);
        let eq34_e1203_d_n3: f64 = (eq34_e1200 * eq34_e1202_d_n3);
        let eq34_e1203_d_n4: f64 = (eq34_e1200 * eq34_e1202_d_n4);
        let eq34_e1203_d_n5: f64 = (eq34_e1200 * eq34_e1202_d_n5);
        let eq34_e1203_d_n6: f64 = (eq34_e1200 * eq34_e1202_d_n6);
        let eq34_e1203_d_n7: f64 = (eq34_e1200 * eq34_e1202_d_n7);
        let eq34_e1203_d_n8: f64 = (eq34_e1200 * eq34_e1202_d_n8);
        let eq34_e1203_d_n9: f64 = (eq34_e1200 * eq34_e1202_d_n9);
        let eq34_e1203_d_n10: f64 = (eq34_e1200 * eq34_e1202_d_n10);
        let eq34_e1203_d_n11: f64 = (eq34_e1200 * eq34_e1202_d_n11);
        let eq34_e1203_d_n12: f64 = (eq34_e1200 * eq34_e1202_d_n12);
        let eq34_e1203_d_n13: f64 = (eq34_e1200 * eq34_e1202_d_n13);
        let eq34_e1203_d_n14: f64 = (eq34_e1200 * eq34_e1202_d_n14);
        let eq34_e1203_d_n15: f64 = (eq34_e1200 * eq34_e1202_d_n15);
        let eq34_e1203_d_n16: f64 = (eq34_e1200 * eq34_e1202_d_n16);
        let eq34_e1203_d_n17: f64 = (eq34_e1200 * eq34_e1202_d_n17);
        let eq34_e1203_d_b0: f64 = (eq34_e1200 * eq34_e1202_d_b0);
        let eq34_e1203_d_b1: f64 = (eq34_e1200 * eq34_e1202_d_b1);
        let eq34_e1203_d_b2: f64 = (eq34_e1200 * eq34_e1202_d_b2);
        let eq34_e1203_d_b3: f64 = (eq34_e1200 * eq34_e1202_d_b3);
        let eq34_e1203_d_b4: f64 = (eq34_e1200 * eq34_e1202_d_b4);
        let eq34_e1203_d_b5: f64 = (eq34_e1200 * eq34_e1202_d_b5);
        let eq34_e1203_d_b6: f64 = (eq34_e1200 * eq34_e1202_d_b6);
        let eq34_e1203_d_b7: f64 = (eq34_e1200 * eq34_e1202_d_b7);
        let eq34_e1203_d_b8: f64 = (eq34_e1200 * eq34_e1202_d_b8);
        let eq34_value: f64 = eq34_e1203;
        let eq34_node_derivatives: [f64; 18] = [eq34_e1203_d_n0, eq34_e1203_d_n1, eq34_e1203_d_n2, eq34_e1203_d_n3, eq34_e1203_d_n4, eq34_e1203_d_n5, eq34_e1203_d_n6, eq34_e1203_d_n7, eq34_e1203_d_n8, eq34_e1203_d_n9, eq34_e1203_d_n10, eq34_e1203_d_n11, eq34_e1203_d_n12, eq34_e1203_d_n13, eq34_e1203_d_n14, eq34_e1203_d_n15, eq34_e1203_d_n16, eq34_e1203_d_n17];
        let eq34_branch_derivatives: [f64; 9] = [eq34_e1203_d_b0, eq34_e1203_d_b1, eq34_e1203_d_b2, eq34_e1203_d_b3, eq34_e1203_d_b4, eq34_e1203_d_b5, eq34_e1203_d_b6, eq34_e1203_d_b7, eq34_e1203_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[2]),
            self.multiplicity * (eq34_value),
            &nodes,
            &eq34_node_derivatives,
            &branches,
            &eq34_branch_derivatives,
            self.multiplicity,
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
            Some(nodes[5]),
            Some(nodes[7]),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let eq36_e1214: f64 = (nv14 - 0.0);
        let eq36_value: f64 = eq36_e1214;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq36_value),
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * 1.0),
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
            Some(nodes[14]),
            None,
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
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq38_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_39_block_0(
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
        let eq39_e1229: f64 = (s.v[951] * (nv14 - 0.0));
        let eq39_e1229_d_n0: f64 = (s.dn[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_n1: f64 = (s.dn[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_n2: f64 = (s.dn[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_n3: f64 = (s.dn[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_n4: f64 = (s.dn[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_n5: f64 = (s.dn[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_n6: f64 = (s.dn[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_n7: f64 = (s.dn[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_n8: f64 = (s.dn[951][8] * (nv14 - 0.0));
        let eq39_e1229_d_n9: f64 = (s.dn[951][9] * (nv14 - 0.0));
        let eq39_e1229_d_n10: f64 = (s.dn[951][10] * (nv14 - 0.0));
        let eq39_e1229_d_n11: f64 = (s.dn[951][11] * (nv14 - 0.0));
        let eq39_e1229_d_n12: f64 = (s.dn[951][12] * (nv14 - 0.0));
        let eq39_e1229_d_n13: f64 = (s.dn[951][13] * (nv14 - 0.0));
        let eq39_e1229_d_n14: f64 = ((s.dn[951][14] * (nv14 - 0.0)) + s.v[951]);
        let eq39_e1229_d_n15: f64 = (s.dn[951][15] * (nv14 - 0.0));
        let eq39_e1229_d_n16: f64 = (s.dn[951][16] * (nv14 - 0.0));
        let eq39_e1229_d_n17: f64 = (s.dn[951][17] * (nv14 - 0.0));
        let eq39_e1229_d_b0: f64 = (s.db[951][0] * (nv14 - 0.0));
        let eq39_e1229_d_b1: f64 = (s.db[951][1] * (nv14 - 0.0));
        let eq39_e1229_d_b2: f64 = (s.db[951][2] * (nv14 - 0.0));
        let eq39_e1229_d_b3: f64 = (s.db[951][3] * (nv14 - 0.0));
        let eq39_e1229_d_b4: f64 = (s.db[951][4] * (nv14 - 0.0));
        let eq39_e1229_d_b5: f64 = (s.db[951][5] * (nv14 - 0.0));
        let eq39_e1229_d_b6: f64 = (s.db[951][6] * (nv14 - 0.0));
        let eq39_e1229_d_b7: f64 = (s.db[951][7] * (nv14 - 0.0));
        let eq39_e1229_d_b8: f64 = (s.db[951][8] * (nv14 - 0.0));
        let eq39_value: f64 = eq39_e1229;
        let eq39_node_derivatives: [f64; 18] = [eq39_e1229_d_n0, eq39_e1229_d_n1, eq39_e1229_d_n2, eq39_e1229_d_n3, eq39_e1229_d_n4, eq39_e1229_d_n5, eq39_e1229_d_n6, eq39_e1229_d_n7, eq39_e1229_d_n8, eq39_e1229_d_n9, eq39_e1229_d_n10, eq39_e1229_d_n11, eq39_e1229_d_n12, eq39_e1229_d_n13, eq39_e1229_d_n14, eq39_e1229_d_n15, eq39_e1229_d_n16, eq39_e1229_d_n17];
        let eq39_branch_derivatives: [f64; 9] = [eq39_e1229_d_b0, eq39_e1229_d_b1, eq39_e1229_d_b2, eq39_e1229_d_b3, eq39_e1229_d_b4, eq39_e1229_d_b5, eq39_e1229_d_b6, eq39_e1229_d_b7, eq39_e1229_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            self.multiplicity * (eq39_value),
            &nodes,
            &eq39_node_derivatives,
            &branches,
            &eq39_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_40_block_0(
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
        let eq40_e1232: f64 = ((nv14 - 0.0) * s.v[954]);
        let eq40_e1232_d_n0: f64 = ((nv14 - 0.0) * s.dn[954][0]);
        let eq40_e1232_d_n1: f64 = ((nv14 - 0.0) * s.dn[954][1]);
        let eq40_e1232_d_n2: f64 = ((nv14 - 0.0) * s.dn[954][2]);
        let eq40_e1232_d_n3: f64 = ((nv14 - 0.0) * s.dn[954][3]);
        let eq40_e1232_d_n4: f64 = ((nv14 - 0.0) * s.dn[954][4]);
        let eq40_e1232_d_n5: f64 = ((nv14 - 0.0) * s.dn[954][5]);
        let eq40_e1232_d_n6: f64 = ((nv14 - 0.0) * s.dn[954][6]);
        let eq40_e1232_d_n7: f64 = ((nv14 - 0.0) * s.dn[954][7]);
        let eq40_e1232_d_n8: f64 = ((nv14 - 0.0) * s.dn[954][8]);
        let eq40_e1232_d_n9: f64 = ((nv14 - 0.0) * s.dn[954][9]);
        let eq40_e1232_d_n10: f64 = ((nv14 - 0.0) * s.dn[954][10]);
        let eq40_e1232_d_n11: f64 = ((nv14 - 0.0) * s.dn[954][11]);
        let eq40_e1232_d_n12: f64 = ((nv14 - 0.0) * s.dn[954][12]);
        let eq40_e1232_d_n13: f64 = ((nv14 - 0.0) * s.dn[954][13]);
        let eq40_e1232_d_n14: f64 = (s.v[954] + ((nv14 - 0.0) * s.dn[954][14]));
        let eq40_e1232_d_n15: f64 = ((nv14 - 0.0) * s.dn[954][15]);
        let eq40_e1232_d_n16: f64 = ((nv14 - 0.0) * s.dn[954][16]);
        let eq40_e1232_d_n17: f64 = ((nv14 - 0.0) * s.dn[954][17]);
        let eq40_e1232_d_b0: f64 = ((nv14 - 0.0) * s.db[954][0]);
        let eq40_e1232_d_b1: f64 = ((nv14 - 0.0) * s.db[954][1]);
        let eq40_e1232_d_b2: f64 = ((nv14 - 0.0) * s.db[954][2]);
        let eq40_e1232_d_b3: f64 = ((nv14 - 0.0) * s.db[954][3]);
        let eq40_e1232_d_b4: f64 = ((nv14 - 0.0) * s.db[954][4]);
        let eq40_e1232_d_b5: f64 = ((nv14 - 0.0) * s.db[954][5]);
        let eq40_e1232_d_b6: f64 = ((nv14 - 0.0) * s.db[954][6]);
        let eq40_e1232_d_b7: f64 = ((nv14 - 0.0) * s.db[954][7]);
        let eq40_e1232_d_b8: f64 = ((nv14 - 0.0) * s.db[954][8]);
        let eq40_e1233: f64 = self.eval_ddt(15, eq40_e1232);
        let eq40_e1233_d_n0: f64 = self.ddt_jacobian(eq40_e1232_d_n0);
        let eq40_e1233_d_n1: f64 = self.ddt_jacobian(eq40_e1232_d_n1);
        let eq40_e1233_d_n2: f64 = self.ddt_jacobian(eq40_e1232_d_n2);
        let eq40_e1233_d_n3: f64 = self.ddt_jacobian(eq40_e1232_d_n3);
        let eq40_e1233_d_n4: f64 = self.ddt_jacobian(eq40_e1232_d_n4);
        let eq40_e1233_d_n5: f64 = self.ddt_jacobian(eq40_e1232_d_n5);
        let eq40_e1233_d_n6: f64 = self.ddt_jacobian(eq40_e1232_d_n6);
        let eq40_e1233_d_n7: f64 = self.ddt_jacobian(eq40_e1232_d_n7);
        let eq40_e1233_d_n8: f64 = self.ddt_jacobian(eq40_e1232_d_n8);
        let eq40_e1233_d_n9: f64 = self.ddt_jacobian(eq40_e1232_d_n9);
        let eq40_e1233_d_n10: f64 = self.ddt_jacobian(eq40_e1232_d_n10);
        let eq40_e1233_d_n11: f64 = self.ddt_jacobian(eq40_e1232_d_n11);
        let eq40_e1233_d_n12: f64 = self.ddt_jacobian(eq40_e1232_d_n12);
        let eq40_e1233_d_n13: f64 = self.ddt_jacobian(eq40_e1232_d_n13);
        let eq40_e1233_d_n14: f64 = self.ddt_jacobian(eq40_e1232_d_n14);
        let eq40_e1233_d_n15: f64 = self.ddt_jacobian(eq40_e1232_d_n15);
        let eq40_e1233_d_n16: f64 = self.ddt_jacobian(eq40_e1232_d_n16);
        let eq40_e1233_d_n17: f64 = self.ddt_jacobian(eq40_e1232_d_n17);
        let eq40_e1233_d_b0: f64 = self.ddt_jacobian(eq40_e1232_d_b0);
        let eq40_e1233_d_b1: f64 = self.ddt_jacobian(eq40_e1232_d_b1);
        let eq40_e1233_d_b2: f64 = self.ddt_jacobian(eq40_e1232_d_b2);
        let eq40_e1233_d_b3: f64 = self.ddt_jacobian(eq40_e1232_d_b3);
        let eq40_e1233_d_b4: f64 = self.ddt_jacobian(eq40_e1232_d_b4);
        let eq40_e1233_d_b5: f64 = self.ddt_jacobian(eq40_e1232_d_b5);
        let eq40_e1233_d_b6: f64 = self.ddt_jacobian(eq40_e1232_d_b6);
        let eq40_e1233_d_b7: f64 = self.ddt_jacobian(eq40_e1232_d_b7);
        let eq40_e1233_d_b8: f64 = self.ddt_jacobian(eq40_e1232_d_b8);
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivatives: [f64; 18] = [eq40_e1233_d_n0, eq40_e1233_d_n1, eq40_e1233_d_n2, eq40_e1233_d_n3, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n12, eq40_e1233_d_n13, eq40_e1233_d_n14, eq40_e1233_d_n15, eq40_e1233_d_n16, eq40_e1233_d_n17];
        let eq40_branch_derivatives: [f64; 9] = [eq40_e1233_d_b0, eq40_e1233_d_b1, eq40_e1233_d_b2, eq40_e1233_d_b3, eq40_e1233_d_b4, eq40_e1233_d_b5, eq40_e1233_d_b6, eq40_e1233_d_b7, eq40_e1233_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq40_value),
            &nodes,
            &eq40_node_derivatives,
            &branches,
            &eq40_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_41_block_0(
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
        let eq41_e1236: f64 = ((nv14 - 0.0) * s.v[955]);
        let eq41_e1236_d_n0: f64 = ((nv14 - 0.0) * s.dn[955][0]);
        let eq41_e1236_d_n1: f64 = ((nv14 - 0.0) * s.dn[955][1]);
        let eq41_e1236_d_n2: f64 = ((nv14 - 0.0) * s.dn[955][2]);
        let eq41_e1236_d_n3: f64 = ((nv14 - 0.0) * s.dn[955][3]);
        let eq41_e1236_d_n4: f64 = ((nv14 - 0.0) * s.dn[955][4]);
        let eq41_e1236_d_n5: f64 = ((nv14 - 0.0) * s.dn[955][5]);
        let eq41_e1236_d_n6: f64 = ((nv14 - 0.0) * s.dn[955][6]);
        let eq41_e1236_d_n7: f64 = ((nv14 - 0.0) * s.dn[955][7]);
        let eq41_e1236_d_n8: f64 = ((nv14 - 0.0) * s.dn[955][8]);
        let eq41_e1236_d_n9: f64 = ((nv14 - 0.0) * s.dn[955][9]);
        let eq41_e1236_d_n10: f64 = ((nv14 - 0.0) * s.dn[955][10]);
        let eq41_e1236_d_n11: f64 = ((nv14 - 0.0) * s.dn[955][11]);
        let eq41_e1236_d_n12: f64 = ((nv14 - 0.0) * s.dn[955][12]);
        let eq41_e1236_d_n13: f64 = ((nv14 - 0.0) * s.dn[955][13]);
        let eq41_e1236_d_n14: f64 = (s.v[955] + ((nv14 - 0.0) * s.dn[955][14]));
        let eq41_e1236_d_n15: f64 = ((nv14 - 0.0) * s.dn[955][15]);
        let eq41_e1236_d_n16: f64 = ((nv14 - 0.0) * s.dn[955][16]);
        let eq41_e1236_d_n17: f64 = ((nv14 - 0.0) * s.dn[955][17]);
        let eq41_e1236_d_b0: f64 = ((nv14 - 0.0) * s.db[955][0]);
        let eq41_e1236_d_b1: f64 = ((nv14 - 0.0) * s.db[955][1]);
        let eq41_e1236_d_b2: f64 = ((nv14 - 0.0) * s.db[955][2]);
        let eq41_e1236_d_b3: f64 = ((nv14 - 0.0) * s.db[955][3]);
        let eq41_e1236_d_b4: f64 = ((nv14 - 0.0) * s.db[955][4]);
        let eq41_e1236_d_b5: f64 = ((nv14 - 0.0) * s.db[955][5]);
        let eq41_e1236_d_b6: f64 = ((nv14 - 0.0) * s.db[955][6]);
        let eq41_e1236_d_b7: f64 = ((nv14 - 0.0) * s.db[955][7]);
        let eq41_e1236_d_b8: f64 = ((nv14 - 0.0) * s.db[955][8]);
        let eq41_e1237: f64 = self.eval_ddt(16, eq41_e1236);
        let eq41_e1237_d_n0: f64 = self.ddt_jacobian(eq41_e1236_d_n0);
        let eq41_e1237_d_n1: f64 = self.ddt_jacobian(eq41_e1236_d_n1);
        let eq41_e1237_d_n2: f64 = self.ddt_jacobian(eq41_e1236_d_n2);
        let eq41_e1237_d_n3: f64 = self.ddt_jacobian(eq41_e1236_d_n3);
        let eq41_e1237_d_n4: f64 = self.ddt_jacobian(eq41_e1236_d_n4);
        let eq41_e1237_d_n5: f64 = self.ddt_jacobian(eq41_e1236_d_n5);
        let eq41_e1237_d_n6: f64 = self.ddt_jacobian(eq41_e1236_d_n6);
        let eq41_e1237_d_n7: f64 = self.ddt_jacobian(eq41_e1236_d_n7);
        let eq41_e1237_d_n8: f64 = self.ddt_jacobian(eq41_e1236_d_n8);
        let eq41_e1237_d_n9: f64 = self.ddt_jacobian(eq41_e1236_d_n9);
        let eq41_e1237_d_n10: f64 = self.ddt_jacobian(eq41_e1236_d_n10);
        let eq41_e1237_d_n11: f64 = self.ddt_jacobian(eq41_e1236_d_n11);
        let eq41_e1237_d_n12: f64 = self.ddt_jacobian(eq41_e1236_d_n12);
        let eq41_e1237_d_n13: f64 = self.ddt_jacobian(eq41_e1236_d_n13);
        let eq41_e1237_d_n14: f64 = self.ddt_jacobian(eq41_e1236_d_n14);
        let eq41_e1237_d_n15: f64 = self.ddt_jacobian(eq41_e1236_d_n15);
        let eq41_e1237_d_n16: f64 = self.ddt_jacobian(eq41_e1236_d_n16);
        let eq41_e1237_d_n17: f64 = self.ddt_jacobian(eq41_e1236_d_n17);
        let eq41_e1237_d_b0: f64 = self.ddt_jacobian(eq41_e1236_d_b0);
        let eq41_e1237_d_b1: f64 = self.ddt_jacobian(eq41_e1236_d_b1);
        let eq41_e1237_d_b2: f64 = self.ddt_jacobian(eq41_e1236_d_b2);
        let eq41_e1237_d_b3: f64 = self.ddt_jacobian(eq41_e1236_d_b3);
        let eq41_e1237_d_b4: f64 = self.ddt_jacobian(eq41_e1236_d_b4);
        let eq41_e1237_d_b5: f64 = self.ddt_jacobian(eq41_e1236_d_b5);
        let eq41_e1237_d_b6: f64 = self.ddt_jacobian(eq41_e1236_d_b6);
        let eq41_e1237_d_b7: f64 = self.ddt_jacobian(eq41_e1236_d_b7);
        let eq41_e1237_d_b8: f64 = self.ddt_jacobian(eq41_e1236_d_b8);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivatives: [f64; 18] = [eq41_e1237_d_n0, eq41_e1237_d_n1, eq41_e1237_d_n2, eq41_e1237_d_n3, eq41_e1237_d_n4, eq41_e1237_d_n5, eq41_e1237_d_n6, eq41_e1237_d_n7, eq41_e1237_d_n8, eq41_e1237_d_n9, eq41_e1237_d_n10, eq41_e1237_d_n11, eq41_e1237_d_n12, eq41_e1237_d_n13, eq41_e1237_d_n14, eq41_e1237_d_n15, eq41_e1237_d_n16, eq41_e1237_d_n17];
        let eq41_branch_derivatives: [f64; 9] = [eq41_e1237_d_b0, eq41_e1237_d_b1, eq41_e1237_d_b2, eq41_e1237_d_b3, eq41_e1237_d_b4, eq41_e1237_d_b5, eq41_e1237_d_b6, eq41_e1237_d_b7, eq41_e1237_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq41_value),
            &nodes,
            &eq41_node_derivatives,
            &branches,
            &eq41_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_42_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq42_e1245,) = {
    if (s.v[76] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq42_value: f64 = eq42_e1245;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq42_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_43_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq43_e1253,) = {
    if (s.v[75] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq43_value: f64 = eq43_e1253;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[5]),
            self.multiplicity * (eq43_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_44_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq44_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[5]),
            self.multiplicity * (eq44_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_45_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq45_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq45_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq46_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq46_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_47_block_0(
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
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq47_e1283, eq47_e1283_d_n0, eq47_e1283_d_n1, eq47_e1283_d_n2, eq47_e1283_d_n3, eq47_e1283_d_n4, eq47_e1283_d_n5, eq47_e1283_d_n6, eq47_e1283_d_n7, eq47_e1283_d_n8, eq47_e1283_d_n9, eq47_e1283_d_n10, eq47_e1283_d_n11, eq47_e1283_d_n12, eq47_e1283_d_n13, eq47_e1283_d_n14, eq47_e1283_d_n15, eq47_e1283_d_n16, eq47_e1283_d_n17, eq47_e1283_d_b0, eq47_e1283_d_b1, eq47_e1283_d_b2, eq47_e1283_d_b3, eq47_e1283_d_b4, eq47_e1283_d_b5, eq47_e1283_d_b6, eq47_e1283_d_b7, eq47_e1283_d_b8,) = {
    if (s.v[3408] != 0.0) {
        let eq47_e1281: f64 = (s.v[643] * (nv1 - nv6));
        let eq47_e1281_d_n0: f64 = (s.dn[643][0] * (nv1 - nv6));
        let eq47_e1281_d_n1: f64 = ((s.dn[643][1] * (nv1 - nv6)) + s.v[643]);
        let eq47_e1281_d_n2: f64 = (s.dn[643][2] * (nv1 - nv6));
        let eq47_e1281_d_n3: f64 = (s.dn[643][3] * (nv1 - nv6));
        let eq47_e1281_d_n4: f64 = (s.dn[643][4] * (nv1 - nv6));
        let eq47_e1281_d_n5: f64 = (s.dn[643][5] * (nv1 - nv6));
        let eq47_e1281_d_n6: f64 = ((s.dn[643][6] * (nv1 - nv6)) + (-s.v[643]));
        let eq47_e1281_d_n7: f64 = (s.dn[643][7] * (nv1 - nv6));
        let eq47_e1281_d_n8: f64 = (s.dn[643][8] * (nv1 - nv6));
        let eq47_e1281_d_n9: f64 = (s.dn[643][9] * (nv1 - nv6));
        let eq47_e1281_d_n10: f64 = (s.dn[643][10] * (nv1 - nv6));
        let eq47_e1281_d_n11: f64 = (s.dn[643][11] * (nv1 - nv6));
        let eq47_e1281_d_n12: f64 = (s.dn[643][12] * (nv1 - nv6));
        let eq47_e1281_d_n13: f64 = (s.dn[643][13] * (nv1 - nv6));
        let eq47_e1281_d_n14: f64 = (s.dn[643][14] * (nv1 - nv6));
        let eq47_e1281_d_n15: f64 = (s.dn[643][15] * (nv1 - nv6));
        let eq47_e1281_d_n16: f64 = (s.dn[643][16] * (nv1 - nv6));
        let eq47_e1281_d_n17: f64 = (s.dn[643][17] * (nv1 - nv6));
        let eq47_e1281_d_b0: f64 = (s.db[643][0] * (nv1 - nv6));
        let eq47_e1281_d_b1: f64 = (s.db[643][1] * (nv1 - nv6));
        let eq47_e1281_d_b2: f64 = (s.db[643][2] * (nv1 - nv6));
        let eq47_e1281_d_b3: f64 = (s.db[643][3] * (nv1 - nv6));
        let eq47_e1281_d_b4: f64 = (s.db[643][4] * (nv1 - nv6));
        let eq47_e1281_d_b5: f64 = (s.db[643][5] * (nv1 - nv6));
        let eq47_e1281_d_b6: f64 = (s.db[643][6] * (nv1 - nv6));
        let eq47_e1281_d_b7: f64 = (s.db[643][7] * (nv1 - nv6));
        let eq47_e1281_d_b8: f64 = (s.db[643][8] * (nv1 - nv6));
        (eq47_e1281, eq47_e1281_d_n0, eq47_e1281_d_n1, eq47_e1281_d_n2, eq47_e1281_d_n3, eq47_e1281_d_n4, eq47_e1281_d_n5, eq47_e1281_d_n6, eq47_e1281_d_n7, eq47_e1281_d_n8, eq47_e1281_d_n9, eq47_e1281_d_n10, eq47_e1281_d_n11, eq47_e1281_d_n12, eq47_e1281_d_n13, eq47_e1281_d_n14, eq47_e1281_d_n15, eq47_e1281_d_n16, eq47_e1281_d_n17, eq47_e1281_d_b0, eq47_e1281_d_b1, eq47_e1281_d_b2, eq47_e1281_d_b3, eq47_e1281_d_b4, eq47_e1281_d_b5, eq47_e1281_d_b6, eq47_e1281_d_b7, eq47_e1281_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e1283;
        let eq47_node_derivatives: [f64; 18] = [eq47_e1283_d_n0, eq47_e1283_d_n1, eq47_e1283_d_n2, eq47_e1283_d_n3, eq47_e1283_d_n4, eq47_e1283_d_n5, eq47_e1283_d_n6, eq47_e1283_d_n7, eq47_e1283_d_n8, eq47_e1283_d_n9, eq47_e1283_d_n10, eq47_e1283_d_n11, eq47_e1283_d_n12, eq47_e1283_d_n13, eq47_e1283_d_n14, eq47_e1283_d_n15, eq47_e1283_d_n16, eq47_e1283_d_n17];
        let eq47_branch_derivatives: [f64; 9] = [eq47_e1283_d_b0, eq47_e1283_d_b1, eq47_e1283_d_b2, eq47_e1283_d_b3, eq47_e1283_d_b4, eq47_e1283_d_b5, eq47_e1283_d_b6, eq47_e1283_d_b7, eq47_e1283_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[6]),
            self.multiplicity * (eq47_value),
            &nodes,
            &eq47_node_derivatives,
            &branches,
            &eq47_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_48_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq48_e1288,) = {
    if (!(s.v[3408] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1288;
        stamper.stamp_potential(
            branches[5],
            eq48_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_49_block_0(
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq49_e1294, eq49_e1294_d_n0, eq49_e1294_d_n1, eq49_e1294_d_n2, eq49_e1294_d_n3, eq49_e1294_d_n4, eq49_e1294_d_n5, eq49_e1294_d_n6, eq49_e1294_d_n7, eq49_e1294_d_n8, eq49_e1294_d_n9, eq49_e1294_d_n10, eq49_e1294_d_n11, eq49_e1294_d_n12, eq49_e1294_d_n13, eq49_e1294_d_n14, eq49_e1294_d_n15, eq49_e1294_d_n16, eq49_e1294_d_n17, eq49_e1294_d_b0, eq49_e1294_d_b1, eq49_e1294_d_b2, eq49_e1294_d_b3, eq49_e1294_d_b4, eq49_e1294_d_b5, eq49_e1294_d_b6, eq49_e1294_d_b7, eq49_e1294_d_b8,) = {
    if (p.p52 != 0.0) {
        let eq49_e1292: f64 = (s.v[656] * (nv10 - nv8));
        let eq49_e1292_d_n0: f64 = (s.dn[656][0] * (nv10 - nv8));
        let eq49_e1292_d_n1: f64 = (s.dn[656][1] * (nv10 - nv8));
        let eq49_e1292_d_n2: f64 = (s.dn[656][2] * (nv10 - nv8));
        let eq49_e1292_d_n3: f64 = (s.dn[656][3] * (nv10 - nv8));
        let eq49_e1292_d_n4: f64 = (s.dn[656][4] * (nv10 - nv8));
        let eq49_e1292_d_n5: f64 = (s.dn[656][5] * (nv10 - nv8));
        let eq49_e1292_d_n6: f64 = (s.dn[656][6] * (nv10 - nv8));
        let eq49_e1292_d_n7: f64 = (s.dn[656][7] * (nv10 - nv8));
        let eq49_e1292_d_n8: f64 = ((s.dn[656][8] * (nv10 - nv8)) + (-s.v[656]));
        let eq49_e1292_d_n9: f64 = (s.dn[656][9] * (nv10 - nv8));
        let eq49_e1292_d_n10: f64 = ((s.dn[656][10] * (nv10 - nv8)) + s.v[656]);
        let eq49_e1292_d_n11: f64 = (s.dn[656][11] * (nv10 - nv8));
        let eq49_e1292_d_n12: f64 = (s.dn[656][12] * (nv10 - nv8));
        let eq49_e1292_d_n13: f64 = (s.dn[656][13] * (nv10 - nv8));
        let eq49_e1292_d_n14: f64 = (s.dn[656][14] * (nv10 - nv8));
        let eq49_e1292_d_n15: f64 = (s.dn[656][15] * (nv10 - nv8));
        let eq49_e1292_d_n16: f64 = (s.dn[656][16] * (nv10 - nv8));
        let eq49_e1292_d_n17: f64 = (s.dn[656][17] * (nv10 - nv8));
        let eq49_e1292_d_b0: f64 = (s.db[656][0] * (nv10 - nv8));
        let eq49_e1292_d_b1: f64 = (s.db[656][1] * (nv10 - nv8));
        let eq49_e1292_d_b2: f64 = (s.db[656][2] * (nv10 - nv8));
        let eq49_e1292_d_b3: f64 = (s.db[656][3] * (nv10 - nv8));
        let eq49_e1292_d_b4: f64 = (s.db[656][4] * (nv10 - nv8));
        let eq49_e1292_d_b5: f64 = (s.db[656][5] * (nv10 - nv8));
        let eq49_e1292_d_b6: f64 = (s.db[656][6] * (nv10 - nv8));
        let eq49_e1292_d_b7: f64 = (s.db[656][7] * (nv10 - nv8));
        let eq49_e1292_d_b8: f64 = (s.db[656][8] * (nv10 - nv8));
        (eq49_e1292, eq49_e1292_d_n0, eq49_e1292_d_n1, eq49_e1292_d_n2, eq49_e1292_d_n3, eq49_e1292_d_n4, eq49_e1292_d_n5, eq49_e1292_d_n6, eq49_e1292_d_n7, eq49_e1292_d_n8, eq49_e1292_d_n9, eq49_e1292_d_n10, eq49_e1292_d_n11, eq49_e1292_d_n12, eq49_e1292_d_n13, eq49_e1292_d_n14, eq49_e1292_d_n15, eq49_e1292_d_n16, eq49_e1292_d_n17, eq49_e1292_d_b0, eq49_e1292_d_b1, eq49_e1292_d_b2, eq49_e1292_d_b3, eq49_e1292_d_b4, eq49_e1292_d_b5, eq49_e1292_d_b6, eq49_e1292_d_b7, eq49_e1292_d_b8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e1294;
        let eq49_node_derivatives: [f64; 18] = [eq49_e1294_d_n0, eq49_e1294_d_n1, eq49_e1294_d_n2, eq49_e1294_d_n3, eq49_e1294_d_n4, eq49_e1294_d_n5, eq49_e1294_d_n6, eq49_e1294_d_n7, eq49_e1294_d_n8, eq49_e1294_d_n9, eq49_e1294_d_n10, eq49_e1294_d_n11, eq49_e1294_d_n12, eq49_e1294_d_n13, eq49_e1294_d_n14, eq49_e1294_d_n15, eq49_e1294_d_n16, eq49_e1294_d_n17];
        let eq49_branch_derivatives: [f64; 9] = [eq49_e1294_d_b0, eq49_e1294_d_b1, eq49_e1294_d_b2, eq49_e1294_d_b3, eq49_e1294_d_b4, eq49_e1294_d_b5, eq49_e1294_d_b6, eq49_e1294_d_b7, eq49_e1294_d_b8];
        stamper.stamp_current_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            self.multiplicity * (eq49_value),
            &nodes,
            &eq49_node_derivatives,
            &branches,
            &eq49_branch_derivatives,
            self.multiplicity,
        );
    }
}

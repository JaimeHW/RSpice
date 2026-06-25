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
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201: f64 = self.eval_ddt(13, s.v[299]);
        let eq34_e1201_d_n0: f64 = self.ddt_jacobian(s.dn[299][0]);
        let eq34_e1201_d_n1: f64 = self.ddt_jacobian(s.dn[299][1]);
        let eq34_e1201_d_n2: f64 = self.ddt_jacobian(s.dn[299][2]);
        let eq34_e1201_d_n3: f64 = self.ddt_jacobian(s.dn[299][3]);
        let eq34_e1201_d_n4: f64 = self.ddt_jacobian(s.dn[299][4]);
        let eq34_e1201_d_n5: f64 = self.ddt_jacobian(s.dn[299][5]);
        let eq34_e1201_d_n6: f64 = self.ddt_jacobian(s.dn[299][6]);
        let eq34_e1201_d_n7: f64 = self.ddt_jacobian(s.dn[299][7]);
        let eq34_e1201_d_n8: f64 = self.ddt_jacobian(s.dn[299][8]);
        let eq34_e1201_d_n9: f64 = self.ddt_jacobian(s.dn[299][9]);
        let eq34_e1201_d_n10: f64 = self.ddt_jacobian(s.dn[299][10]);
        let eq34_e1201_d_n11: f64 = self.ddt_jacobian(s.dn[299][11]);
        let eq34_e1201_d_n12: f64 = self.ddt_jacobian(s.dn[299][12]);
        let eq34_e1201_d_n13: f64 = self.ddt_jacobian(s.dn[299][13]);
        let eq34_e1201_d_n14: f64 = self.ddt_jacobian(s.dn[299][14]);
        let eq34_e1201_d_n15: f64 = self.ddt_jacobian(s.dn[299][15]);
        let eq34_e1201_d_n16: f64 = self.ddt_jacobian(s.dn[299][16]);
        let eq34_e1201_d_n17: f64 = self.ddt_jacobian(s.dn[299][17]);
        let eq34_e1201_d_n18: f64 = self.ddt_jacobian(s.dn[299][18]);
        let eq34_e1201_d_b0: f64 = self.ddt_jacobian(s.db[299][0]);
        let eq34_e1201_d_b1: f64 = self.ddt_jacobian(s.db[299][1]);
        let eq34_e1201_d_b2: f64 = self.ddt_jacobian(s.db[299][2]);
        let eq34_e1201_d_b3: f64 = self.ddt_jacobian(s.db[299][3]);
        let eq34_e1201_d_b4: f64 = self.ddt_jacobian(s.db[299][4]);
        let eq34_e1201_d_b5: f64 = self.ddt_jacobian(s.db[299][5]);
        let eq34_e1201_d_b6: f64 = self.ddt_jacobian(s.db[299][6]);
        let eq34_e1201_d_b7: f64 = self.ddt_jacobian(s.db[299][7]);
        let eq34_e1201_d_b8: f64 = self.ddt_jacobian(s.db[299][8]);
        let eq34_e1201_d_b9: f64 = self.ddt_jacobian(s.db[299][9]);
        let eq34_e1202: f64 = (eq34_e1199 * eq34_e1201);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * eq34_e1201_d_n0);
        let eq34_e1202_d_n1: f64 = (eq34_e1199 * eq34_e1201_d_n1);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * eq34_e1201_d_n2);
        let eq34_e1202_d_n3: f64 = (eq34_e1199 * eq34_e1201_d_n3);
        let eq34_e1202_d_n4: f64 = (eq34_e1199 * eq34_e1201_d_n4);
        let eq34_e1202_d_n5: f64 = (eq34_e1199 * eq34_e1201_d_n5);
        let eq34_e1202_d_n6: f64 = (eq34_e1199 * eq34_e1201_d_n6);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * eq34_e1201_d_n7);
        let eq34_e1202_d_n8: f64 = (eq34_e1199 * eq34_e1201_d_n8);
        let eq34_e1202_d_n9: f64 = (eq34_e1199 * eq34_e1201_d_n9);
        let eq34_e1202_d_n10: f64 = (eq34_e1199 * eq34_e1201_d_n10);
        let eq34_e1202_d_n11: f64 = (eq34_e1199 * eq34_e1201_d_n11);
        let eq34_e1202_d_n12: f64 = (eq34_e1199 * eq34_e1201_d_n12);
        let eq34_e1202_d_n13: f64 = (eq34_e1199 * eq34_e1201_d_n13);
        let eq34_e1202_d_n14: f64 = (eq34_e1199 * eq34_e1201_d_n14);
        let eq34_e1202_d_n15: f64 = (eq34_e1199 * eq34_e1201_d_n15);
        let eq34_e1202_d_n16: f64 = (eq34_e1199 * eq34_e1201_d_n16);
        let eq34_e1202_d_n17: f64 = (eq34_e1199 * eq34_e1201_d_n17);
        let eq34_e1202_d_n18: f64 = (eq34_e1199 * eq34_e1201_d_n18);
        let eq34_e1202_d_b0: f64 = (eq34_e1199 * eq34_e1201_d_b0);
        let eq34_e1202_d_b1: f64 = (eq34_e1199 * eq34_e1201_d_b1);
        let eq34_e1202_d_b2: f64 = (eq34_e1199 * eq34_e1201_d_b2);
        let eq34_e1202_d_b3: f64 = (eq34_e1199 * eq34_e1201_d_b3);
        let eq34_e1202_d_b4: f64 = (eq34_e1199 * eq34_e1201_d_b4);
        let eq34_e1202_d_b5: f64 = (eq34_e1199 * eq34_e1201_d_b5);
        let eq34_e1202_d_b6: f64 = (eq34_e1199 * eq34_e1201_d_b6);
        let eq34_e1202_d_b7: f64 = (eq34_e1199 * eq34_e1201_d_b7);
        let eq34_e1202_d_b8: f64 = (eq34_e1199 * eq34_e1201_d_b8);
        let eq34_e1202_d_b9: f64 = (eq34_e1199 * eq34_e1201_d_b9);
        let eq34_value: f64 = eq34_e1202;
        let eq34_node_derivatives: [f64; 19] = [eq34_e1202_d_n0, eq34_e1202_d_n1, eq34_e1202_d_n2, eq34_e1202_d_n3, eq34_e1202_d_n4, eq34_e1202_d_n5, eq34_e1202_d_n6, eq34_e1202_d_n7, eq34_e1202_d_n8, eq34_e1202_d_n9, eq34_e1202_d_n10, eq34_e1202_d_n11, eq34_e1202_d_n12, eq34_e1202_d_n13, eq34_e1202_d_n14, eq34_e1202_d_n15, eq34_e1202_d_n16, eq34_e1202_d_n17, eq34_e1202_d_n18];
        let eq34_branch_derivatives: [f64; 10] = [eq34_e1202_d_b0, eq34_e1202_d_b1, eq34_e1202_d_b2, eq34_e1202_d_b3, eq34_e1202_d_b4, eq34_e1202_d_b5, eq34_e1202_d_b6, eq34_e1202_d_b7, eq34_e1202_d_b8, eq34_e1202_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[0]),
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
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206: f64 = self.eval_ddt(14, s.v[301]);
        let eq35_e1206_d_n0: f64 = self.ddt_jacobian(s.dn[301][0]);
        let eq35_e1206_d_n1: f64 = self.ddt_jacobian(s.dn[301][1]);
        let eq35_e1206_d_n2: f64 = self.ddt_jacobian(s.dn[301][2]);
        let eq35_e1206_d_n3: f64 = self.ddt_jacobian(s.dn[301][3]);
        let eq35_e1206_d_n4: f64 = self.ddt_jacobian(s.dn[301][4]);
        let eq35_e1206_d_n5: f64 = self.ddt_jacobian(s.dn[301][5]);
        let eq35_e1206_d_n6: f64 = self.ddt_jacobian(s.dn[301][6]);
        let eq35_e1206_d_n7: f64 = self.ddt_jacobian(s.dn[301][7]);
        let eq35_e1206_d_n8: f64 = self.ddt_jacobian(s.dn[301][8]);
        let eq35_e1206_d_n9: f64 = self.ddt_jacobian(s.dn[301][9]);
        let eq35_e1206_d_n10: f64 = self.ddt_jacobian(s.dn[301][10]);
        let eq35_e1206_d_n11: f64 = self.ddt_jacobian(s.dn[301][11]);
        let eq35_e1206_d_n12: f64 = self.ddt_jacobian(s.dn[301][12]);
        let eq35_e1206_d_n13: f64 = self.ddt_jacobian(s.dn[301][13]);
        let eq35_e1206_d_n14: f64 = self.ddt_jacobian(s.dn[301][14]);
        let eq35_e1206_d_n15: f64 = self.ddt_jacobian(s.dn[301][15]);
        let eq35_e1206_d_n16: f64 = self.ddt_jacobian(s.dn[301][16]);
        let eq35_e1206_d_n17: f64 = self.ddt_jacobian(s.dn[301][17]);
        let eq35_e1206_d_n18: f64 = self.ddt_jacobian(s.dn[301][18]);
        let eq35_e1206_d_b0: f64 = self.ddt_jacobian(s.db[301][0]);
        let eq35_e1206_d_b1: f64 = self.ddt_jacobian(s.db[301][1]);
        let eq35_e1206_d_b2: f64 = self.ddt_jacobian(s.db[301][2]);
        let eq35_e1206_d_b3: f64 = self.ddt_jacobian(s.db[301][3]);
        let eq35_e1206_d_b4: f64 = self.ddt_jacobian(s.db[301][4]);
        let eq35_e1206_d_b5: f64 = self.ddt_jacobian(s.db[301][5]);
        let eq35_e1206_d_b6: f64 = self.ddt_jacobian(s.db[301][6]);
        let eq35_e1206_d_b7: f64 = self.ddt_jacobian(s.db[301][7]);
        let eq35_e1206_d_b8: f64 = self.ddt_jacobian(s.db[301][8]);
        let eq35_e1206_d_b9: f64 = self.ddt_jacobian(s.db[301][9]);
        let eq35_e1207: f64 = (eq35_e1204 * eq35_e1206);
        let eq35_e1207_d_n0: f64 = (eq35_e1204 * eq35_e1206_d_n0);
        let eq35_e1207_d_n1: f64 = (eq35_e1204 * eq35_e1206_d_n1);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * eq35_e1206_d_n2);
        let eq35_e1207_d_n3: f64 = (eq35_e1204 * eq35_e1206_d_n3);
        let eq35_e1207_d_n4: f64 = (eq35_e1204 * eq35_e1206_d_n4);
        let eq35_e1207_d_n5: f64 = (eq35_e1204 * eq35_e1206_d_n5);
        let eq35_e1207_d_n6: f64 = (eq35_e1204 * eq35_e1206_d_n6);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * eq35_e1206_d_n7);
        let eq35_e1207_d_n8: f64 = (eq35_e1204 * eq35_e1206_d_n8);
        let eq35_e1207_d_n9: f64 = (eq35_e1204 * eq35_e1206_d_n9);
        let eq35_e1207_d_n10: f64 = (eq35_e1204 * eq35_e1206_d_n10);
        let eq35_e1207_d_n11: f64 = (eq35_e1204 * eq35_e1206_d_n11);
        let eq35_e1207_d_n12: f64 = (eq35_e1204 * eq35_e1206_d_n12);
        let eq35_e1207_d_n13: f64 = (eq35_e1204 * eq35_e1206_d_n13);
        let eq35_e1207_d_n14: f64 = (eq35_e1204 * eq35_e1206_d_n14);
        let eq35_e1207_d_n15: f64 = (eq35_e1204 * eq35_e1206_d_n15);
        let eq35_e1207_d_n16: f64 = (eq35_e1204 * eq35_e1206_d_n16);
        let eq35_e1207_d_n17: f64 = (eq35_e1204 * eq35_e1206_d_n17);
        let eq35_e1207_d_n18: f64 = (eq35_e1204 * eq35_e1206_d_n18);
        let eq35_e1207_d_b0: f64 = (eq35_e1204 * eq35_e1206_d_b0);
        let eq35_e1207_d_b1: f64 = (eq35_e1204 * eq35_e1206_d_b1);
        let eq35_e1207_d_b2: f64 = (eq35_e1204 * eq35_e1206_d_b2);
        let eq35_e1207_d_b3: f64 = (eq35_e1204 * eq35_e1206_d_b3);
        let eq35_e1207_d_b4: f64 = (eq35_e1204 * eq35_e1206_d_b4);
        let eq35_e1207_d_b5: f64 = (eq35_e1204 * eq35_e1206_d_b5);
        let eq35_e1207_d_b6: f64 = (eq35_e1204 * eq35_e1206_d_b6);
        let eq35_e1207_d_b7: f64 = (eq35_e1204 * eq35_e1206_d_b7);
        let eq35_e1207_d_b8: f64 = (eq35_e1204 * eq35_e1206_d_b8);
        let eq35_e1207_d_b9: f64 = (eq35_e1204 * eq35_e1206_d_b9);
        let eq35_value: f64 = eq35_e1207;
        let eq35_node_derivatives: [f64; 19] = [eq35_e1207_d_n0, eq35_e1207_d_n1, eq35_e1207_d_n2, eq35_e1207_d_n3, eq35_e1207_d_n4, eq35_e1207_d_n5, eq35_e1207_d_n6, eq35_e1207_d_n7, eq35_e1207_d_n8, eq35_e1207_d_n9, eq35_e1207_d_n10, eq35_e1207_d_n11, eq35_e1207_d_n12, eq35_e1207_d_n13, eq35_e1207_d_n14, eq35_e1207_d_n15, eq35_e1207_d_n16, eq35_e1207_d_n17, eq35_e1207_d_n18];
        let eq35_branch_derivatives: [f64; 10] = [eq35_e1207_d_b0, eq35_e1207_d_b1, eq35_e1207_d_b2, eq35_e1207_d_b3, eq35_e1207_d_b4, eq35_e1207_d_b5, eq35_e1207_d_b6, eq35_e1207_d_b7, eq35_e1207_d_b8, eq35_e1207_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            self.multiplicity * (eq35_value),
            &nodes,
            &eq35_node_derivatives,
            &branches,
            &eq35_branch_derivatives,
            self.multiplicity,
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
            Some(nodes[6]),
            Some(nodes[8]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq37_e1218: f64 = (nv15 - 0.0);
        let eq37_value: f64 = eq37_e1218;
        stamper.stamp_current(
            Some(nodes[15]),
            None,
            self.multiplicity * (eq37_value),
            &[
                GeneratedDerivative::node(nodes[15], self.multiplicity * 1.0),
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
            Some(nodes[15]),
            None,
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
        let eq39_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[8]),
            self.multiplicity * (eq39_value),
            &[
            ],
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq40_e1233: f64 = (s.v[951] * (nv15 - 0.0));
        let eq40_e1233_d_n0: f64 = (s.dn[951][0] * (nv15 - 0.0));
        let eq40_e1233_d_n1: f64 = (s.dn[951][1] * (nv15 - 0.0));
        let eq40_e1233_d_n2: f64 = (s.dn[951][2] * (nv15 - 0.0));
        let eq40_e1233_d_n3: f64 = (s.dn[951][3] * (nv15 - 0.0));
        let eq40_e1233_d_n4: f64 = (s.dn[951][4] * (nv15 - 0.0));
        let eq40_e1233_d_n5: f64 = (s.dn[951][5] * (nv15 - 0.0));
        let eq40_e1233_d_n6: f64 = (s.dn[951][6] * (nv15 - 0.0));
        let eq40_e1233_d_n7: f64 = (s.dn[951][7] * (nv15 - 0.0));
        let eq40_e1233_d_n8: f64 = (s.dn[951][8] * (nv15 - 0.0));
        let eq40_e1233_d_n9: f64 = (s.dn[951][9] * (nv15 - 0.0));
        let eq40_e1233_d_n10: f64 = (s.dn[951][10] * (nv15 - 0.0));
        let eq40_e1233_d_n11: f64 = (s.dn[951][11] * (nv15 - 0.0));
        let eq40_e1233_d_n12: f64 = (s.dn[951][12] * (nv15 - 0.0));
        let eq40_e1233_d_n13: f64 = (s.dn[951][13] * (nv15 - 0.0));
        let eq40_e1233_d_n14: f64 = (s.dn[951][14] * (nv15 - 0.0));
        let eq40_e1233_d_n15: f64 = ((s.dn[951][15] * (nv15 - 0.0)) + s.v[951]);
        let eq40_e1233_d_n16: f64 = (s.dn[951][16] * (nv15 - 0.0));
        let eq40_e1233_d_n17: f64 = (s.dn[951][17] * (nv15 - 0.0));
        let eq40_e1233_d_n18: f64 = (s.dn[951][18] * (nv15 - 0.0));
        let eq40_e1233_d_b0: f64 = (s.db[951][0] * (nv15 - 0.0));
        let eq40_e1233_d_b1: f64 = (s.db[951][1] * (nv15 - 0.0));
        let eq40_e1233_d_b2: f64 = (s.db[951][2] * (nv15 - 0.0));
        let eq40_e1233_d_b3: f64 = (s.db[951][3] * (nv15 - 0.0));
        let eq40_e1233_d_b4: f64 = (s.db[951][4] * (nv15 - 0.0));
        let eq40_e1233_d_b5: f64 = (s.db[951][5] * (nv15 - 0.0));
        let eq40_e1233_d_b6: f64 = (s.db[951][6] * (nv15 - 0.0));
        let eq40_e1233_d_b7: f64 = (s.db[951][7] * (nv15 - 0.0));
        let eq40_e1233_d_b8: f64 = (s.db[951][8] * (nv15 - 0.0));
        let eq40_e1233_d_b9: f64 = (s.db[951][9] * (nv15 - 0.0));
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivatives: [f64; 19] = [eq40_e1233_d_n0, eq40_e1233_d_n1, eq40_e1233_d_n2, eq40_e1233_d_n3, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n12, eq40_e1233_d_n13, eq40_e1233_d_n14, eq40_e1233_d_n15, eq40_e1233_d_n16, eq40_e1233_d_n17, eq40_e1233_d_n18];
        let eq40_branch_derivatives: [f64; 10] = [eq40_e1233_d_b0, eq40_e1233_d_b1, eq40_e1233_d_b2, eq40_e1233_d_b3, eq40_e1233_d_b4, eq40_e1233_d_b5, eq40_e1233_d_b6, eq40_e1233_d_b7, eq40_e1233_d_b8, eq40_e1233_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[6]),
            Some(nodes[8]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq41_e1236: f64 = ((nv15 - 0.0) * s.v[954]);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * s.dn[954][0]);
        let eq41_e1236_d_n1: f64 = ((nv15 - 0.0) * s.dn[954][1]);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * s.dn[954][2]);
        let eq41_e1236_d_n3: f64 = ((nv15 - 0.0) * s.dn[954][3]);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * s.dn[954][4]);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * s.dn[954][5]);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * s.dn[954][6]);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * s.dn[954][7]);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * s.dn[954][8]);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * s.dn[954][9]);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * s.dn[954][10]);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * s.dn[954][11]);
        let eq41_e1236_d_n12: f64 = ((nv15 - 0.0) * s.dn[954][12]);
        let eq41_e1236_d_n13: f64 = ((nv15 - 0.0) * s.dn[954][13]);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * s.dn[954][14]);
        let eq41_e1236_d_n15: f64 = (s.v[954] + ((nv15 - 0.0) * s.dn[954][15]));
        let eq41_e1236_d_n16: f64 = ((nv15 - 0.0) * s.dn[954][16]);
        let eq41_e1236_d_n17: f64 = ((nv15 - 0.0) * s.dn[954][17]);
        let eq41_e1236_d_n18: f64 = ((nv15 - 0.0) * s.dn[954][18]);
        let eq41_e1236_d_b0: f64 = ((nv15 - 0.0) * s.db[954][0]);
        let eq41_e1236_d_b1: f64 = ((nv15 - 0.0) * s.db[954][1]);
        let eq41_e1236_d_b2: f64 = ((nv15 - 0.0) * s.db[954][2]);
        let eq41_e1236_d_b3: f64 = ((nv15 - 0.0) * s.db[954][3]);
        let eq41_e1236_d_b4: f64 = ((nv15 - 0.0) * s.db[954][4]);
        let eq41_e1236_d_b5: f64 = ((nv15 - 0.0) * s.db[954][5]);
        let eq41_e1236_d_b6: f64 = ((nv15 - 0.0) * s.db[954][6]);
        let eq41_e1236_d_b7: f64 = ((nv15 - 0.0) * s.db[954][7]);
        let eq41_e1236_d_b8: f64 = ((nv15 - 0.0) * s.db[954][8]);
        let eq41_e1236_d_b9: f64 = ((nv15 - 0.0) * s.db[954][9]);
        let eq41_e1237: f64 = self.eval_ddt(15, eq41_e1236);
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
        let eq41_e1237_d_n18: f64 = self.ddt_jacobian(eq41_e1236_d_n18);
        let eq41_e1237_d_b0: f64 = self.ddt_jacobian(eq41_e1236_d_b0);
        let eq41_e1237_d_b1: f64 = self.ddt_jacobian(eq41_e1236_d_b1);
        let eq41_e1237_d_b2: f64 = self.ddt_jacobian(eq41_e1236_d_b2);
        let eq41_e1237_d_b3: f64 = self.ddt_jacobian(eq41_e1236_d_b3);
        let eq41_e1237_d_b4: f64 = self.ddt_jacobian(eq41_e1236_d_b4);
        let eq41_e1237_d_b5: f64 = self.ddt_jacobian(eq41_e1236_d_b5);
        let eq41_e1237_d_b6: f64 = self.ddt_jacobian(eq41_e1236_d_b6);
        let eq41_e1237_d_b7: f64 = self.ddt_jacobian(eq41_e1236_d_b7);
        let eq41_e1237_d_b8: f64 = self.ddt_jacobian(eq41_e1236_d_b8);
        let eq41_e1237_d_b9: f64 = self.ddt_jacobian(eq41_e1236_d_b9);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivatives: [f64; 19] = [eq41_e1237_d_n0, eq41_e1237_d_n1, eq41_e1237_d_n2, eq41_e1237_d_n3, eq41_e1237_d_n4, eq41_e1237_d_n5, eq41_e1237_d_n6, eq41_e1237_d_n7, eq41_e1237_d_n8, eq41_e1237_d_n9, eq41_e1237_d_n10, eq41_e1237_d_n11, eq41_e1237_d_n12, eq41_e1237_d_n13, eq41_e1237_d_n14, eq41_e1237_d_n15, eq41_e1237_d_n16, eq41_e1237_d_n17, eq41_e1237_d_n18];
        let eq41_branch_derivatives: [f64; 10] = [eq41_e1237_d_b0, eq41_e1237_d_b1, eq41_e1237_d_b2, eq41_e1237_d_b3, eq41_e1237_d_b4, eq41_e1237_d_b5, eq41_e1237_d_b6, eq41_e1237_d_b7, eq41_e1237_d_b8, eq41_e1237_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[8]),
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq42_e1240: f64 = ((nv15 - 0.0) * s.v[955]);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * s.dn[955][0]);
        let eq42_e1240_d_n1: f64 = ((nv15 - 0.0) * s.dn[955][1]);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * s.dn[955][2]);
        let eq42_e1240_d_n3: f64 = ((nv15 - 0.0) * s.dn[955][3]);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * s.dn[955][4]);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * s.dn[955][5]);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * s.dn[955][6]);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * s.dn[955][7]);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * s.dn[955][8]);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * s.dn[955][9]);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * s.dn[955][10]);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * s.dn[955][11]);
        let eq42_e1240_d_n12: f64 = ((nv15 - 0.0) * s.dn[955][12]);
        let eq42_e1240_d_n13: f64 = ((nv15 - 0.0) * s.dn[955][13]);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * s.dn[955][14]);
        let eq42_e1240_d_n15: f64 = (s.v[955] + ((nv15 - 0.0) * s.dn[955][15]));
        let eq42_e1240_d_n16: f64 = ((nv15 - 0.0) * s.dn[955][16]);
        let eq42_e1240_d_n17: f64 = ((nv15 - 0.0) * s.dn[955][17]);
        let eq42_e1240_d_n18: f64 = ((nv15 - 0.0) * s.dn[955][18]);
        let eq42_e1240_d_b0: f64 = ((nv15 - 0.0) * s.db[955][0]);
        let eq42_e1240_d_b1: f64 = ((nv15 - 0.0) * s.db[955][1]);
        let eq42_e1240_d_b2: f64 = ((nv15 - 0.0) * s.db[955][2]);
        let eq42_e1240_d_b3: f64 = ((nv15 - 0.0) * s.db[955][3]);
        let eq42_e1240_d_b4: f64 = ((nv15 - 0.0) * s.db[955][4]);
        let eq42_e1240_d_b5: f64 = ((nv15 - 0.0) * s.db[955][5]);
        let eq42_e1240_d_b6: f64 = ((nv15 - 0.0) * s.db[955][6]);
        let eq42_e1240_d_b7: f64 = ((nv15 - 0.0) * s.db[955][7]);
        let eq42_e1240_d_b8: f64 = ((nv15 - 0.0) * s.db[955][8]);
        let eq42_e1240_d_b9: f64 = ((nv15 - 0.0) * s.db[955][9]);
        let eq42_e1241: f64 = self.eval_ddt(16, eq42_e1240);
        let eq42_e1241_d_n0: f64 = self.ddt_jacobian(eq42_e1240_d_n0);
        let eq42_e1241_d_n1: f64 = self.ddt_jacobian(eq42_e1240_d_n1);
        let eq42_e1241_d_n2: f64 = self.ddt_jacobian(eq42_e1240_d_n2);
        let eq42_e1241_d_n3: f64 = self.ddt_jacobian(eq42_e1240_d_n3);
        let eq42_e1241_d_n4: f64 = self.ddt_jacobian(eq42_e1240_d_n4);
        let eq42_e1241_d_n5: f64 = self.ddt_jacobian(eq42_e1240_d_n5);
        let eq42_e1241_d_n6: f64 = self.ddt_jacobian(eq42_e1240_d_n6);
        let eq42_e1241_d_n7: f64 = self.ddt_jacobian(eq42_e1240_d_n7);
        let eq42_e1241_d_n8: f64 = self.ddt_jacobian(eq42_e1240_d_n8);
        let eq42_e1241_d_n9: f64 = self.ddt_jacobian(eq42_e1240_d_n9);
        let eq42_e1241_d_n10: f64 = self.ddt_jacobian(eq42_e1240_d_n10);
        let eq42_e1241_d_n11: f64 = self.ddt_jacobian(eq42_e1240_d_n11);
        let eq42_e1241_d_n12: f64 = self.ddt_jacobian(eq42_e1240_d_n12);
        let eq42_e1241_d_n13: f64 = self.ddt_jacobian(eq42_e1240_d_n13);
        let eq42_e1241_d_n14: f64 = self.ddt_jacobian(eq42_e1240_d_n14);
        let eq42_e1241_d_n15: f64 = self.ddt_jacobian(eq42_e1240_d_n15);
        let eq42_e1241_d_n16: f64 = self.ddt_jacobian(eq42_e1240_d_n16);
        let eq42_e1241_d_n17: f64 = self.ddt_jacobian(eq42_e1240_d_n17);
        let eq42_e1241_d_n18: f64 = self.ddt_jacobian(eq42_e1240_d_n18);
        let eq42_e1241_d_b0: f64 = self.ddt_jacobian(eq42_e1240_d_b0);
        let eq42_e1241_d_b1: f64 = self.ddt_jacobian(eq42_e1240_d_b1);
        let eq42_e1241_d_b2: f64 = self.ddt_jacobian(eq42_e1240_d_b2);
        let eq42_e1241_d_b3: f64 = self.ddt_jacobian(eq42_e1240_d_b3);
        let eq42_e1241_d_b4: f64 = self.ddt_jacobian(eq42_e1240_d_b4);
        let eq42_e1241_d_b5: f64 = self.ddt_jacobian(eq42_e1240_d_b5);
        let eq42_e1241_d_b6: f64 = self.ddt_jacobian(eq42_e1240_d_b6);
        let eq42_e1241_d_b7: f64 = self.ddt_jacobian(eq42_e1240_d_b7);
        let eq42_e1241_d_b8: f64 = self.ddt_jacobian(eq42_e1240_d_b8);
        let eq42_e1241_d_b9: f64 = self.ddt_jacobian(eq42_e1240_d_b9);
        let eq42_value: f64 = eq42_e1241;
        let eq42_node_derivatives: [f64; 19] = [eq42_e1241_d_n0, eq42_e1241_d_n1, eq42_e1241_d_n2, eq42_e1241_d_n3, eq42_e1241_d_n4, eq42_e1241_d_n5, eq42_e1241_d_n6, eq42_e1241_d_n7, eq42_e1241_d_n8, eq42_e1241_d_n9, eq42_e1241_d_n10, eq42_e1241_d_n11, eq42_e1241_d_n12, eq42_e1241_d_n13, eq42_e1241_d_n14, eq42_e1241_d_n15, eq42_e1241_d_n16, eq42_e1241_d_n17, eq42_e1241_d_n18];
        let eq42_branch_derivatives: [f64; 10] = [eq42_e1241_d_b0, eq42_e1241_d_b1, eq42_e1241_d_b2, eq42_e1241_d_b3, eq42_e1241_d_b4, eq42_e1241_d_b5, eq42_e1241_d_b6, eq42_e1241_d_b7, eq42_e1241_d_b8, eq42_e1241_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            self.multiplicity * (eq42_value),
            &nodes,
            &eq42_node_derivatives,
            &branches,
            &eq42_branch_derivatives,
            self.multiplicity,
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
        let (eq43_e1249,) = {
    if (s.v[76] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq43_value: f64 = eq43_e1249;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[2]),
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
        let (eq44_e1257,) = {
    if (s.v[75] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq44_value: f64 = eq44_e1257;
        stamper.stamp_current(
            Some(nodes[0]),
            Some(nodes[6]),
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
            Some(nodes[7]),
            Some(nodes[6]),
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
            Some(nodes[7]),
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
        let eq47_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq47_value),
            &[
            ],
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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let (eq48_e1287, eq48_e1287_d_n0, eq48_e1287_d_n1, eq48_e1287_d_n2, eq48_e1287_d_n3, eq48_e1287_d_n4, eq48_e1287_d_n5, eq48_e1287_d_n6, eq48_e1287_d_n7, eq48_e1287_d_n8, eq48_e1287_d_n9, eq48_e1287_d_n10, eq48_e1287_d_n11, eq48_e1287_d_n12, eq48_e1287_d_n13, eq48_e1287_d_n14, eq48_e1287_d_n15, eq48_e1287_d_n16, eq48_e1287_d_n17, eq48_e1287_d_n18, eq48_e1287_d_b0, eq48_e1287_d_b1, eq48_e1287_d_b2, eq48_e1287_d_b3, eq48_e1287_d_b4, eq48_e1287_d_b5, eq48_e1287_d_b6, eq48_e1287_d_b7, eq48_e1287_d_b8, eq48_e1287_d_b9,) = {
    if (s.v[3410] != 0.0) {
        let eq48_e1285: f64 = (s.v[643] * (nv1 - nv7));
        let eq48_e1285_d_n0: f64 = (s.dn[643][0] * (nv1 - nv7));
        let eq48_e1285_d_n1: f64 = ((s.dn[643][1] * (nv1 - nv7)) + s.v[643]);
        let eq48_e1285_d_n2: f64 = (s.dn[643][2] * (nv1 - nv7));
        let eq48_e1285_d_n3: f64 = (s.dn[643][3] * (nv1 - nv7));
        let eq48_e1285_d_n4: f64 = (s.dn[643][4] * (nv1 - nv7));
        let eq48_e1285_d_n5: f64 = (s.dn[643][5] * (nv1 - nv7));
        let eq48_e1285_d_n6: f64 = (s.dn[643][6] * (nv1 - nv7));
        let eq48_e1285_d_n7: f64 = ((s.dn[643][7] * (nv1 - nv7)) + (-s.v[643]));
        let eq48_e1285_d_n8: f64 = (s.dn[643][8] * (nv1 - nv7));
        let eq48_e1285_d_n9: f64 = (s.dn[643][9] * (nv1 - nv7));
        let eq48_e1285_d_n10: f64 = (s.dn[643][10] * (nv1 - nv7));
        let eq48_e1285_d_n11: f64 = (s.dn[643][11] * (nv1 - nv7));
        let eq48_e1285_d_n12: f64 = (s.dn[643][12] * (nv1 - nv7));
        let eq48_e1285_d_n13: f64 = (s.dn[643][13] * (nv1 - nv7));
        let eq48_e1285_d_n14: f64 = (s.dn[643][14] * (nv1 - nv7));
        let eq48_e1285_d_n15: f64 = (s.dn[643][15] * (nv1 - nv7));
        let eq48_e1285_d_n16: f64 = (s.dn[643][16] * (nv1 - nv7));
        let eq48_e1285_d_n17: f64 = (s.dn[643][17] * (nv1 - nv7));
        let eq48_e1285_d_n18: f64 = (s.dn[643][18] * (nv1 - nv7));
        let eq48_e1285_d_b0: f64 = (s.db[643][0] * (nv1 - nv7));
        let eq48_e1285_d_b1: f64 = (s.db[643][1] * (nv1 - nv7));
        let eq48_e1285_d_b2: f64 = (s.db[643][2] * (nv1 - nv7));
        let eq48_e1285_d_b3: f64 = (s.db[643][3] * (nv1 - nv7));
        let eq48_e1285_d_b4: f64 = (s.db[643][4] * (nv1 - nv7));
        let eq48_e1285_d_b5: f64 = (s.db[643][5] * (nv1 - nv7));
        let eq48_e1285_d_b6: f64 = (s.db[643][6] * (nv1 - nv7));
        let eq48_e1285_d_b7: f64 = (s.db[643][7] * (nv1 - nv7));
        let eq48_e1285_d_b8: f64 = (s.db[643][8] * (nv1 - nv7));
        let eq48_e1285_d_b9: f64 = (s.db[643][9] * (nv1 - nv7));
        (eq48_e1285, eq48_e1285_d_n0, eq48_e1285_d_n1, eq48_e1285_d_n2, eq48_e1285_d_n3, eq48_e1285_d_n4, eq48_e1285_d_n5, eq48_e1285_d_n6, eq48_e1285_d_n7, eq48_e1285_d_n8, eq48_e1285_d_n9, eq48_e1285_d_n10, eq48_e1285_d_n11, eq48_e1285_d_n12, eq48_e1285_d_n13, eq48_e1285_d_n14, eq48_e1285_d_n15, eq48_e1285_d_n16, eq48_e1285_d_n17, eq48_e1285_d_n18, eq48_e1285_d_b0, eq48_e1285_d_b1, eq48_e1285_d_b2, eq48_e1285_d_b3, eq48_e1285_d_b4, eq48_e1285_d_b5, eq48_e1285_d_b6, eq48_e1285_d_b7, eq48_e1285_d_b8, eq48_e1285_d_b9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e1287;
        let eq48_node_derivatives: [f64; 19] = [eq48_e1287_d_n0, eq48_e1287_d_n1, eq48_e1287_d_n2, eq48_e1287_d_n3, eq48_e1287_d_n4, eq48_e1287_d_n5, eq48_e1287_d_n6, eq48_e1287_d_n7, eq48_e1287_d_n8, eq48_e1287_d_n9, eq48_e1287_d_n10, eq48_e1287_d_n11, eq48_e1287_d_n12, eq48_e1287_d_n13, eq48_e1287_d_n14, eq48_e1287_d_n15, eq48_e1287_d_n16, eq48_e1287_d_n17, eq48_e1287_d_n18];
        let eq48_branch_derivatives: [f64; 10] = [eq48_e1287_d_b0, eq48_e1287_d_b1, eq48_e1287_d_b2, eq48_e1287_d_b3, eq48_e1287_d_b4, eq48_e1287_d_b5, eq48_e1287_d_b6, eq48_e1287_d_b7, eq48_e1287_d_b8, eq48_e1287_d_b9];
        stamper.stamp_current_dense(
            Some(nodes[1]),
            Some(nodes[7]),
            self.multiplicity * (eq48_value),
            &nodes,
            &eq48_node_derivatives,
            &branches,
            &eq48_branch_derivatives,
            self.multiplicity,
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
        let (eq49_e1292,) = {
    if (!(s.v[3410] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1292;
        stamper.stamp_potential(
            branches[6],
            eq49_value,
            &[
            ],
        );
    }
}

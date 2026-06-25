#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_63_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq63_e519, eq63_e519_d_n13,) = {
    if (s.v[533] != 0.0) {
        let eq63_e517: f64 = (-(nv13 - 0.0));
        let eq63_e517_d_n13: f64 = (-1.0);
        (eq63_e517, eq63_e517_d_n13,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e519;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq63_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq63_e519_d_n13),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_64_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq64_e523, eq64_e523_d_n13,) = {
    if (s.v[533] != 0.0) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e523;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq64_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq64_e523_d_n13),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_65_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq65_e534, eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14, eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5,) = {
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
        let eq65_e531: f64 = self.eval_ddt(18, eq65_e530);
        let eq65_e531_d_n0: f64 = self.ddt_jacobian(eq65_e530_d_n0);
        let eq65_e531_d_n1: f64 = self.ddt_jacobian(eq65_e530_d_n1);
        let eq65_e531_d_n2: f64 = self.ddt_jacobian(eq65_e530_d_n2);
        let eq65_e531_d_n3: f64 = self.ddt_jacobian(eq65_e530_d_n3);
        let eq65_e531_d_n4: f64 = self.ddt_jacobian(eq65_e530_d_n4);
        let eq65_e531_d_n5: f64 = self.ddt_jacobian(eq65_e530_d_n5);
        let eq65_e531_d_n6: f64 = self.ddt_jacobian(eq65_e530_d_n6);
        let eq65_e531_d_n7: f64 = self.ddt_jacobian(eq65_e530_d_n7);
        let eq65_e531_d_n8: f64 = self.ddt_jacobian(eq65_e530_d_n8);
        let eq65_e531_d_n9: f64 = self.ddt_jacobian(eq65_e530_d_n9);
        let eq65_e531_d_n10: f64 = self.ddt_jacobian(eq65_e530_d_n10);
        let eq65_e531_d_n11: f64 = self.ddt_jacobian(eq65_e530_d_n11);
        let eq65_e531_d_n12: f64 = self.ddt_jacobian(eq65_e530_d_n12);
        let eq65_e531_d_n13: f64 = self.ddt_jacobian(eq65_e530_d_n13);
        let eq65_e531_d_n14: f64 = self.ddt_jacobian(eq65_e530_d_n14);
        let eq65_e531_d_b0: f64 = self.ddt_jacobian(eq65_e530_d_b0);
        let eq65_e531_d_b1: f64 = self.ddt_jacobian(eq65_e530_d_b1);
        let eq65_e531_d_b2: f64 = self.ddt_jacobian(eq65_e530_d_b2);
        let eq65_e531_d_b3: f64 = self.ddt_jacobian(eq65_e530_d_b3);
        let eq65_e531_d_b4: f64 = self.ddt_jacobian(eq65_e530_d_b4);
        let eq65_e531_d_b5: f64 = self.ddt_jacobian(eq65_e530_d_b5);
        let eq65_e532: f64 = (eq65_e527 * eq65_e531);
        let eq65_e532_d_n0: f64 = ((eq65_e527_d_n0 * eq65_e531) + (eq65_e527 * eq65_e531_d_n0));
        let eq65_e532_d_n1: f64 = ((eq65_e527_d_n1 * eq65_e531) + (eq65_e527 * eq65_e531_d_n1));
        let eq65_e532_d_n2: f64 = ((eq65_e527_d_n2 * eq65_e531) + (eq65_e527 * eq65_e531_d_n2));
        let eq65_e532_d_n3: f64 = ((eq65_e527_d_n3 * eq65_e531) + (eq65_e527 * eq65_e531_d_n3));
        let eq65_e532_d_n4: f64 = ((eq65_e527_d_n4 * eq65_e531) + (eq65_e527 * eq65_e531_d_n4));
        let eq65_e532_d_n5: f64 = ((eq65_e527_d_n5 * eq65_e531) + (eq65_e527 * eq65_e531_d_n5));
        let eq65_e532_d_n6: f64 = ((eq65_e527_d_n6 * eq65_e531) + (eq65_e527 * eq65_e531_d_n6));
        let eq65_e532_d_n7: f64 = ((eq65_e527_d_n7 * eq65_e531) + (eq65_e527 * eq65_e531_d_n7));
        let eq65_e532_d_n8: f64 = ((eq65_e527_d_n8 * eq65_e531) + (eq65_e527 * eq65_e531_d_n8));
        let eq65_e532_d_n9: f64 = ((eq65_e527_d_n9 * eq65_e531) + (eq65_e527 * eq65_e531_d_n9));
        let eq65_e532_d_n10: f64 = ((eq65_e527_d_n10 * eq65_e531) + (eq65_e527 * eq65_e531_d_n10));
        let eq65_e532_d_n11: f64 = ((eq65_e527_d_n11 * eq65_e531) + (eq65_e527 * eq65_e531_d_n11));
        let eq65_e532_d_n12: f64 = ((eq65_e527_d_n12 * eq65_e531) + (eq65_e527 * eq65_e531_d_n12));
        let eq65_e532_d_n13: f64 = ((eq65_e527_d_n13 * eq65_e531) + (eq65_e527 * eq65_e531_d_n13));
        let eq65_e532_d_n14: f64 = ((eq65_e527_d_n14 * eq65_e531) + (eq65_e527 * eq65_e531_d_n14));
        let eq65_e532_d_b0: f64 = ((eq65_e527_d_b0 * eq65_e531) + (eq65_e527 * eq65_e531_d_b0));
        let eq65_e532_d_b1: f64 = ((eq65_e527_d_b1 * eq65_e531) + (eq65_e527 * eq65_e531_d_b1));
        let eq65_e532_d_b2: f64 = ((eq65_e527_d_b2 * eq65_e531) + (eq65_e527 * eq65_e531_d_b2));
        let eq65_e532_d_b3: f64 = ((eq65_e527_d_b3 * eq65_e531) + (eq65_e527 * eq65_e531_d_b3));
        let eq65_e532_d_b4: f64 = ((eq65_e527_d_b4 * eq65_e531) + (eq65_e527 * eq65_e531_d_b4));
        let eq65_e532_d_b5: f64 = ((eq65_e527_d_b5 * eq65_e531) + (eq65_e527 * eq65_e531_d_b5));
        (eq65_e532, eq65_e532_d_n0, eq65_e532_d_n1, eq65_e532_d_n2, eq65_e532_d_n3, eq65_e532_d_n4, eq65_e532_d_n5, eq65_e532_d_n6, eq65_e532_d_n7, eq65_e532_d_n8, eq65_e532_d_n9, eq65_e532_d_n10, eq65_e532_d_n11, eq65_e532_d_n12, eq65_e532_d_n13, eq65_e532_d_n14, eq65_e532_d_b0, eq65_e532_d_b1, eq65_e532_d_b2, eq65_e532_d_b3, eq65_e532_d_b4, eq65_e532_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e534;
        let eq65_node_derivatives: [f64; 15] = [eq65_e534_d_n0, eq65_e534_d_n1, eq65_e534_d_n2, eq65_e534_d_n3, eq65_e534_d_n4, eq65_e534_d_n5, eq65_e534_d_n6, eq65_e534_d_n7, eq65_e534_d_n8, eq65_e534_d_n9, eq65_e534_d_n10, eq65_e534_d_n11, eq65_e534_d_n12, eq65_e534_d_n13, eq65_e534_d_n14];
        let eq65_branch_derivatives: [f64; 6] = [eq65_e534_d_b0, eq65_e534_d_b1, eq65_e534_d_b2, eq65_e534_d_b3, eq65_e534_d_b4, eq65_e534_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq65_value),
            &nodes,
            &eq65_node_derivatives,
            &branches,
            &eq65_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_66_block_0(
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
        let (eq66_e545, eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14, eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5,) = {
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
        let eq66_e542: f64 = self.eval_ddt(19, eq66_e541);
        let eq66_e542_d_n0: f64 = self.ddt_jacobian(eq66_e541_d_n0);
        let eq66_e542_d_n1: f64 = self.ddt_jacobian(eq66_e541_d_n1);
        let eq66_e542_d_n2: f64 = self.ddt_jacobian(eq66_e541_d_n2);
        let eq66_e542_d_n3: f64 = self.ddt_jacobian(eq66_e541_d_n3);
        let eq66_e542_d_n4: f64 = self.ddt_jacobian(eq66_e541_d_n4);
        let eq66_e542_d_n5: f64 = self.ddt_jacobian(eq66_e541_d_n5);
        let eq66_e542_d_n6: f64 = self.ddt_jacobian(eq66_e541_d_n6);
        let eq66_e542_d_n7: f64 = self.ddt_jacobian(eq66_e541_d_n7);
        let eq66_e542_d_n8: f64 = self.ddt_jacobian(eq66_e541_d_n8);
        let eq66_e542_d_n9: f64 = self.ddt_jacobian(eq66_e541_d_n9);
        let eq66_e542_d_n10: f64 = self.ddt_jacobian(eq66_e541_d_n10);
        let eq66_e542_d_n11: f64 = self.ddt_jacobian(eq66_e541_d_n11);
        let eq66_e542_d_n12: f64 = self.ddt_jacobian(eq66_e541_d_n12);
        let eq66_e542_d_n13: f64 = self.ddt_jacobian(eq66_e541_d_n13);
        let eq66_e542_d_n14: f64 = self.ddt_jacobian(eq66_e541_d_n14);
        let eq66_e542_d_b0: f64 = self.ddt_jacobian(eq66_e541_d_b0);
        let eq66_e542_d_b1: f64 = self.ddt_jacobian(eq66_e541_d_b1);
        let eq66_e542_d_b2: f64 = self.ddt_jacobian(eq66_e541_d_b2);
        let eq66_e542_d_b3: f64 = self.ddt_jacobian(eq66_e541_d_b3);
        let eq66_e542_d_b4: f64 = self.ddt_jacobian(eq66_e541_d_b4);
        let eq66_e542_d_b5: f64 = self.ddt_jacobian(eq66_e541_d_b5);
        let eq66_e543: f64 = (eq66_e538 * eq66_e542);
        let eq66_e543_d_n0: f64 = ((eq66_e538_d_n0 * eq66_e542) + (eq66_e538 * eq66_e542_d_n0));
        let eq66_e543_d_n1: f64 = ((eq66_e538_d_n1 * eq66_e542) + (eq66_e538 * eq66_e542_d_n1));
        let eq66_e543_d_n2: f64 = ((eq66_e538_d_n2 * eq66_e542) + (eq66_e538 * eq66_e542_d_n2));
        let eq66_e543_d_n3: f64 = ((eq66_e538_d_n3 * eq66_e542) + (eq66_e538 * eq66_e542_d_n3));
        let eq66_e543_d_n4: f64 = ((eq66_e538_d_n4 * eq66_e542) + (eq66_e538 * eq66_e542_d_n4));
        let eq66_e543_d_n5: f64 = ((eq66_e538_d_n5 * eq66_e542) + (eq66_e538 * eq66_e542_d_n5));
        let eq66_e543_d_n6: f64 = ((eq66_e538_d_n6 * eq66_e542) + (eq66_e538 * eq66_e542_d_n6));
        let eq66_e543_d_n7: f64 = ((eq66_e538_d_n7 * eq66_e542) + (eq66_e538 * eq66_e542_d_n7));
        let eq66_e543_d_n8: f64 = ((eq66_e538_d_n8 * eq66_e542) + (eq66_e538 * eq66_e542_d_n8));
        let eq66_e543_d_n9: f64 = ((eq66_e538_d_n9 * eq66_e542) + (eq66_e538 * eq66_e542_d_n9));
        let eq66_e543_d_n10: f64 = ((eq66_e538_d_n10 * eq66_e542) + (eq66_e538 * eq66_e542_d_n10));
        let eq66_e543_d_n11: f64 = ((eq66_e538_d_n11 * eq66_e542) + (eq66_e538 * eq66_e542_d_n11));
        let eq66_e543_d_n12: f64 = ((eq66_e538_d_n12 * eq66_e542) + (eq66_e538 * eq66_e542_d_n12));
        let eq66_e543_d_n13: f64 = ((eq66_e538_d_n13 * eq66_e542) + (eq66_e538 * eq66_e542_d_n13));
        let eq66_e543_d_n14: f64 = ((eq66_e538_d_n14 * eq66_e542) + (eq66_e538 * eq66_e542_d_n14));
        let eq66_e543_d_b0: f64 = ((eq66_e538_d_b0 * eq66_e542) + (eq66_e538 * eq66_e542_d_b0));
        let eq66_e543_d_b1: f64 = ((eq66_e538_d_b1 * eq66_e542) + (eq66_e538 * eq66_e542_d_b1));
        let eq66_e543_d_b2: f64 = ((eq66_e538_d_b2 * eq66_e542) + (eq66_e538 * eq66_e542_d_b2));
        let eq66_e543_d_b3: f64 = ((eq66_e538_d_b3 * eq66_e542) + (eq66_e538 * eq66_e542_d_b3));
        let eq66_e543_d_b4: f64 = ((eq66_e538_d_b4 * eq66_e542) + (eq66_e538 * eq66_e542_d_b4));
        let eq66_e543_d_b5: f64 = ((eq66_e538_d_b5 * eq66_e542) + (eq66_e538 * eq66_e542_d_b5));
        (eq66_e543, eq66_e543_d_n0, eq66_e543_d_n1, eq66_e543_d_n2, eq66_e543_d_n3, eq66_e543_d_n4, eq66_e543_d_n5, eq66_e543_d_n6, eq66_e543_d_n7, eq66_e543_d_n8, eq66_e543_d_n9, eq66_e543_d_n10, eq66_e543_d_n11, eq66_e543_d_n12, eq66_e543_d_n13, eq66_e543_d_n14, eq66_e543_d_b0, eq66_e543_d_b1, eq66_e543_d_b2, eq66_e543_d_b3, eq66_e543_d_b4, eq66_e543_d_b5,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e545;
        let eq66_node_derivatives: [f64; 15] = [eq66_e545_d_n0, eq66_e545_d_n1, eq66_e545_d_n2, eq66_e545_d_n3, eq66_e545_d_n4, eq66_e545_d_n5, eq66_e545_d_n6, eq66_e545_d_n7, eq66_e545_d_n8, eq66_e545_d_n9, eq66_e545_d_n10, eq66_e545_d_n11, eq66_e545_d_n12, eq66_e545_d_n13, eq66_e545_d_n14];
        let eq66_branch_derivatives: [f64; 6] = [eq66_e545_d_b0, eq66_e545_d_b1, eq66_e545_d_b2, eq66_e545_d_b3, eq66_e545_d_b4, eq66_e545_d_b5];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq66_value),
            &nodes,
            &eq66_node_derivatives,
            &branches,
            &eq66_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq67_e554,) = {
    if (s.v[533] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq67_value: f64 = eq67_e554;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq67_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_68_block_0(
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
        let (eq68_e559, eq68_e559_d_n14,) = {
    if (s.v[533] != 0.0) {
        let eq68_e557: f64 = (-(nv14 - 0.0));
        let eq68_e557_d_n14: f64 = (-1.0);
        (eq68_e557, eq68_e557_d_n14,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e559;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq68_value),
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * eq68_e559_d_n14),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_69_block_0(
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
        let (eq69_e563, eq69_e563_d_n14,) = {
    if (s.v[533] != 0.0) {
        ((nv14 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e563;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq69_value),
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * eq69_e563_d_n14),
            ],
        );
    }

    pub(super) fn stamp_transient_equation_70_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq70_e573,) = {
    if (!(s.v[533] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq70_value: f64 = eq70_e573;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[6]),
            self.multiplicity * (eq70_value),
            &[
            ],
        );
    }

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
        let (eq71_e583,) = {
    if (!(s.v[533] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq71_value: f64 = eq71_e583;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq71_value),
            &[
            ],
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq72_e588, eq72_e588_d_n13,) = {
    if (!(s.v[533] != 0.0)) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e588;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq72_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq72_e588_d_n13),
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
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq73_e593, eq73_e593_d_n14,) = {
    if (!(s.v[533] != 0.0)) {
        ((nv14 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e593;
        stamper.stamp_current(
            Some(nodes[14]),
            None,
            self.multiplicity * (eq73_value),
            &[
                GeneratedDerivative::node(nodes[14], self.multiplicity * eq73_e593_d_n14),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_1_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq1_e170: f64 = (s.v[242] + s.v[179]);
        let eq1_e170_d_n0: f64 = (s.dn[242][0] + s.dn[179][0]);
        let eq1_e170_d_n1: f64 = (s.dn[242][1] + s.dn[179][1]);
        let eq1_e170_d_n2: f64 = (s.dn[242][2] + s.dn[179][2]);
        let eq1_e170_d_n3: f64 = (s.dn[242][3] + s.dn[179][3]);
        let eq1_e170_d_n4: f64 = (s.dn[242][4] + s.dn[179][4]);
        let eq1_e170_d_n5: f64 = (s.dn[242][5] + s.dn[179][5]);
        let eq1_e170_d_n6: f64 = (s.dn[242][6] + s.dn[179][6]);
        let eq1_e170_d_n7: f64 = (s.dn[242][7] + s.dn[179][7]);
        let eq1_e170_d_n8: f64 = (s.dn[242][8] + s.dn[179][8]);
        let eq1_e170_d_n9: f64 = (s.dn[242][9] + s.dn[179][9]);
        let eq1_e170_d_n10: f64 = (s.dn[242][10] + s.dn[179][10]);
        let eq1_e170_d_n11: f64 = (s.dn[242][11] + s.dn[179][11]);
        let eq1_e170_d_n12: f64 = (s.dn[242][12] + s.dn[179][12]);
        let eq1_e170_d_n13: f64 = (s.dn[242][13] + s.dn[179][13]);
        let eq1_e170_d_n14: f64 = (s.dn[242][14] + s.dn[179][14]);
        let eq1_e170_d_b0: f64 = (s.db[242][0] + s.db[179][0]);
        let eq1_e170_d_b1: f64 = (s.db[242][1] + s.db[179][1]);
        let eq1_e170_d_b2: f64 = (s.db[242][2] + s.db[179][2]);
        let eq1_e170_d_b3: f64 = (s.db[242][3] + s.db[179][3]);
        let eq1_e170_d_b4: f64 = (s.db[242][4] + s.db[179][4]);
        let eq1_e170_d_b5: f64 = (s.db[242][5] + s.db[179][5]);
        let eq1_e171: f64 = (p.p148 * eq1_e170);
        let eq1_e171_d_n0: f64 = (p.p148 * eq1_e170_d_n0);
        let eq1_e171_d_n1: f64 = (p.p148 * eq1_e170_d_n1);
        let eq1_e171_d_n2: f64 = (p.p148 * eq1_e170_d_n2);
        let eq1_e171_d_n3: f64 = (p.p148 * eq1_e170_d_n3);
        let eq1_e171_d_n4: f64 = (p.p148 * eq1_e170_d_n4);
        let eq1_e171_d_n5: f64 = (p.p148 * eq1_e170_d_n5);
        let eq1_e171_d_n6: f64 = (p.p148 * eq1_e170_d_n6);
        let eq1_e171_d_n7: f64 = (p.p148 * eq1_e170_d_n7);
        let eq1_e171_d_n8: f64 = (p.p148 * eq1_e170_d_n8);
        let eq1_e171_d_n9: f64 = (p.p148 * eq1_e170_d_n9);
        let eq1_e171_d_n10: f64 = (p.p148 * eq1_e170_d_n10);
        let eq1_e171_d_n11: f64 = (p.p148 * eq1_e170_d_n11);
        let eq1_e171_d_n12: f64 = (p.p148 * eq1_e170_d_n12);
        let eq1_e171_d_n13: f64 = (p.p148 * eq1_e170_d_n13);
        let eq1_e171_d_n14: f64 = (p.p148 * eq1_e170_d_n14);
        let eq1_e171_d_b0: f64 = (p.p148 * eq1_e170_d_b0);
        let eq1_e171_d_b1: f64 = (p.p148 * eq1_e170_d_b1);
        let eq1_e171_d_b2: f64 = (p.p148 * eq1_e170_d_b2);
        let eq1_e171_d_b3: f64 = (p.p148 * eq1_e170_d_b3);
        let eq1_e171_d_b4: f64 = (p.p148 * eq1_e170_d_b4);
        let eq1_e171_d_b5: f64 = (p.p148 * eq1_e170_d_b5);
        let eq1_e172_q: f64 = eq1_e171;
        let eq1_reactive_node_derivatives: [f64; 15] = [eq1_e171_d_n0, eq1_e171_d_n1, eq1_e171_d_n2, eq1_e171_d_n3, eq1_e171_d_n4, eq1_e171_d_n5, eq1_e171_d_n6, eq1_e171_d_n7, eq1_e171_d_n8, eq1_e171_d_n9, eq1_e171_d_n10, eq1_e171_d_n11, eq1_e171_d_n12, eq1_e171_d_n13, eq1_e171_d_n14];
        let eq1_reactive_branch_derivatives: [f64; 6] = [eq1_e171_d_b0, eq1_e171_d_b1, eq1_e171_d_b2, eq1_e171_d_b3, eq1_e171_d_b4, eq1_e171_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq1_reactive_node_derivatives,
            &branches,
            &eq1_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_3_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq3_e185: f64 = (s.v[182] + s.v[178]);
        let eq3_e185_d_n0: f64 = (s.dn[182][0] + s.dn[178][0]);
        let eq3_e185_d_n1: f64 = (s.dn[182][1] + s.dn[178][1]);
        let eq3_e185_d_n2: f64 = (s.dn[182][2] + s.dn[178][2]);
        let eq3_e185_d_n3: f64 = (s.dn[182][3] + s.dn[178][3]);
        let eq3_e185_d_n4: f64 = (s.dn[182][4] + s.dn[178][4]);
        let eq3_e185_d_n5: f64 = (s.dn[182][5] + s.dn[178][5]);
        let eq3_e185_d_n6: f64 = (s.dn[182][6] + s.dn[178][6]);
        let eq3_e185_d_n7: f64 = (s.dn[182][7] + s.dn[178][7]);
        let eq3_e185_d_n8: f64 = (s.dn[182][8] + s.dn[178][8]);
        let eq3_e185_d_n9: f64 = (s.dn[182][9] + s.dn[178][9]);
        let eq3_e185_d_n10: f64 = (s.dn[182][10] + s.dn[178][10]);
        let eq3_e185_d_n11: f64 = (s.dn[182][11] + s.dn[178][11]);
        let eq3_e185_d_n12: f64 = (s.dn[182][12] + s.dn[178][12]);
        let eq3_e185_d_n13: f64 = (s.dn[182][13] + s.dn[178][13]);
        let eq3_e185_d_n14: f64 = (s.dn[182][14] + s.dn[178][14]);
        let eq3_e185_d_b0: f64 = (s.db[182][0] + s.db[178][0]);
        let eq3_e185_d_b1: f64 = (s.db[182][1] + s.db[178][1]);
        let eq3_e185_d_b2: f64 = (s.db[182][2] + s.db[178][2]);
        let eq3_e185_d_b3: f64 = (s.db[182][3] + s.db[178][3]);
        let eq3_e185_d_b4: f64 = (s.db[182][4] + s.db[178][4]);
        let eq3_e185_d_b5: f64 = (s.db[182][5] + s.db[178][5]);
        let eq3_e186: f64 = (p.p148 * eq3_e185);
        let eq3_e186_d_n0: f64 = (p.p148 * eq3_e185_d_n0);
        let eq3_e186_d_n1: f64 = (p.p148 * eq3_e185_d_n1);
        let eq3_e186_d_n2: f64 = (p.p148 * eq3_e185_d_n2);
        let eq3_e186_d_n3: f64 = (p.p148 * eq3_e185_d_n3);
        let eq3_e186_d_n4: f64 = (p.p148 * eq3_e185_d_n4);
        let eq3_e186_d_n5: f64 = (p.p148 * eq3_e185_d_n5);
        let eq3_e186_d_n6: f64 = (p.p148 * eq3_e185_d_n6);
        let eq3_e186_d_n7: f64 = (p.p148 * eq3_e185_d_n7);
        let eq3_e186_d_n8: f64 = (p.p148 * eq3_e185_d_n8);
        let eq3_e186_d_n9: f64 = (p.p148 * eq3_e185_d_n9);
        let eq3_e186_d_n10: f64 = (p.p148 * eq3_e185_d_n10);
        let eq3_e186_d_n11: f64 = (p.p148 * eq3_e185_d_n11);
        let eq3_e186_d_n12: f64 = (p.p148 * eq3_e185_d_n12);
        let eq3_e186_d_n13: f64 = (p.p148 * eq3_e185_d_n13);
        let eq3_e186_d_n14: f64 = (p.p148 * eq3_e185_d_n14);
        let eq3_e186_d_b0: f64 = (p.p148 * eq3_e185_d_b0);
        let eq3_e186_d_b1: f64 = (p.p148 * eq3_e185_d_b1);
        let eq3_e186_d_b2: f64 = (p.p148 * eq3_e185_d_b2);
        let eq3_e186_d_b3: f64 = (p.p148 * eq3_e185_d_b3);
        let eq3_e186_d_b4: f64 = (p.p148 * eq3_e185_d_b4);
        let eq3_e186_d_b5: f64 = (p.p148 * eq3_e185_d_b5);
        let eq3_e187_q: f64 = eq3_e186;
        let eq3_reactive_node_derivatives: [f64; 15] = [eq3_e186_d_n0, eq3_e186_d_n1, eq3_e186_d_n2, eq3_e186_d_n3, eq3_e186_d_n4, eq3_e186_d_n5, eq3_e186_d_n6, eq3_e186_d_n7, eq3_e186_d_n8, eq3_e186_d_n9, eq3_e186_d_n10, eq3_e186_d_n11, eq3_e186_d_n12, eq3_e186_d_n13, eq3_e186_d_n14];
        let eq3_reactive_branch_derivatives: [f64; 6] = [eq3_e186_d_b0, eq3_e186_d_b1, eq3_e186_d_b2, eq3_e186_d_b3, eq3_e186_d_b4, eq3_e186_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            &nodes,
            &eq3_reactive_node_derivatives,
            &branches,
            &eq3_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_7_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq7_e206, eq7_e206_d_n0, eq7_e206_d_n1, eq7_e206_d_n2, eq7_e206_d_n3, eq7_e206_d_n4, eq7_e206_d_n5, eq7_e206_d_n6, eq7_e206_d_n7, eq7_e206_d_n8, eq7_e206_d_n9, eq7_e206_d_n10, eq7_e206_d_n11, eq7_e206_d_n12, eq7_e206_d_n13, eq7_e206_d_n14, eq7_e206_d_b0, eq7_e206_d_b1, eq7_e206_d_b2, eq7_e206_d_b3, eq7_e206_d_b4, eq7_e206_d_b5, eq7_e206_q, eq7_e206_q_d_n0, eq7_e206_q_d_n1, eq7_e206_q_d_n2, eq7_e206_q_d_n3, eq7_e206_q_d_n4, eq7_e206_q_d_n5, eq7_e206_q_d_n6, eq7_e206_q_d_n7, eq7_e206_q_d_n8, eq7_e206_q_d_n9, eq7_e206_q_d_n10, eq7_e206_q_d_n11, eq7_e206_q_d_n12, eq7_e206_q_d_n13, eq7_e206_q_d_n14, eq7_e206_q_d_b0, eq7_e206_q_d_b1, eq7_e206_q_d_b2, eq7_e206_q_d_b3, eq7_e206_q_d_b4, eq7_e206_q_d_b5,) = {
    if ((s.v[508] != 0.0) && (s.v[509] != 0.0)) {
        let eq7_e204_q: f64 = s.v[183];
        (s.v[183], s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], s.dn[183][7], s.dn[183][8], s.dn[183][9], s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5], eq7_e204_q, s.dn[183][0], s.dn[183][1], s.dn[183][2], s.dn[183][3], s.dn[183][4], s.dn[183][5], s.dn[183][6], s.dn[183][7], s.dn[183][8], s.dn[183][9], s.dn[183][10], s.dn[183][11], s.dn[183][12], s.dn[183][13], s.dn[183][14], s.db[183][0], s.db[183][1], s.db[183][2], s.db[183][3], s.db[183][4], s.db[183][5],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 15] = [eq7_e206_q_d_n0, eq7_e206_q_d_n1, eq7_e206_q_d_n2, eq7_e206_q_d_n3, eq7_e206_q_d_n4, eq7_e206_q_d_n5, eq7_e206_q_d_n6, eq7_e206_q_d_n7, eq7_e206_q_d_n8, eq7_e206_q_d_n9, eq7_e206_q_d_n10, eq7_e206_q_d_n11, eq7_e206_q_d_n12, eq7_e206_q_d_n13, eq7_e206_q_d_n14];
        let eq7_reactive_branch_derivatives: [f64; 6] = [eq7_e206_q_d_b0, eq7_e206_q_d_b1, eq7_e206_q_d_b2, eq7_e206_q_d_b3, eq7_e206_q_d_b4, eq7_e206_q_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            &nodes,
            &eq7_reactive_node_derivatives,
            &branches,
            &eq7_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_13_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq13_e238: f64 = (p.p148 * s.v[180]);
        let eq13_e238_d_n0: f64 = (p.p148 * s.dn[180][0]);
        let eq13_e238_d_n1: f64 = (p.p148 * s.dn[180][1]);
        let eq13_e238_d_n2: f64 = (p.p148 * s.dn[180][2]);
        let eq13_e238_d_n3: f64 = (p.p148 * s.dn[180][3]);
        let eq13_e238_d_n4: f64 = (p.p148 * s.dn[180][4]);
        let eq13_e238_d_n5: f64 = (p.p148 * s.dn[180][5]);
        let eq13_e238_d_n6: f64 = (p.p148 * s.dn[180][6]);
        let eq13_e238_d_n7: f64 = (p.p148 * s.dn[180][7]);
        let eq13_e238_d_n8: f64 = (p.p148 * s.dn[180][8]);
        let eq13_e238_d_n9: f64 = (p.p148 * s.dn[180][9]);
        let eq13_e238_d_n10: f64 = (p.p148 * s.dn[180][10]);
        let eq13_e238_d_n11: f64 = (p.p148 * s.dn[180][11]);
        let eq13_e238_d_n12: f64 = (p.p148 * s.dn[180][12]);
        let eq13_e238_d_n13: f64 = (p.p148 * s.dn[180][13]);
        let eq13_e238_d_n14: f64 = (p.p148 * s.dn[180][14]);
        let eq13_e238_d_b0: f64 = (p.p148 * s.db[180][0]);
        let eq13_e238_d_b1: f64 = (p.p148 * s.db[180][1]);
        let eq13_e238_d_b2: f64 = (p.p148 * s.db[180][2]);
        let eq13_e238_d_b3: f64 = (p.p148 * s.db[180][3]);
        let eq13_e238_d_b4: f64 = (p.p148 * s.db[180][4]);
        let eq13_e238_d_b5: f64 = (p.p148 * s.db[180][5]);
        let eq13_e239_q: f64 = eq13_e238;
        let eq13_reactive_node_derivatives: [f64; 15] = [eq13_e238_d_n0, eq13_e238_d_n1, eq13_e238_d_n2, eq13_e238_d_n3, eq13_e238_d_n4, eq13_e238_d_n5, eq13_e238_d_n6, eq13_e238_d_n7, eq13_e238_d_n8, eq13_e238_d_n9, eq13_e238_d_n10, eq13_e238_d_n11, eq13_e238_d_n12, eq13_e238_d_n13, eq13_e238_d_n14];
        let eq13_reactive_branch_derivatives: [f64; 6] = [eq13_e238_d_b0, eq13_e238_d_b1, eq13_e238_d_b2, eq13_e238_d_b3, eq13_e238_d_b4, eq13_e238_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            &nodes,
            &eq13_reactive_node_derivatives,
            &branches,
            &eq13_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_15_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let eq15_e246: f64 = (s.v[42] + s.v[199]);
        let eq15_e246_d_n0: f64 = (s.dn[42][0] + s.dn[199][0]);
        let eq15_e246_d_n1: f64 = (s.dn[42][1] + s.dn[199][1]);
        let eq15_e246_d_n2: f64 = (s.dn[42][2] + s.dn[199][2]);
        let eq15_e246_d_n3: f64 = (s.dn[42][3] + s.dn[199][3]);
        let eq15_e246_d_n4: f64 = (s.dn[42][4] + s.dn[199][4]);
        let eq15_e246_d_n5: f64 = (s.dn[42][5] + s.dn[199][5]);
        let eq15_e246_d_n6: f64 = (s.dn[42][6] + s.dn[199][6]);
        let eq15_e246_d_n7: f64 = (s.dn[42][7] + s.dn[199][7]);
        let eq15_e246_d_n8: f64 = (s.dn[42][8] + s.dn[199][8]);
        let eq15_e246_d_n9: f64 = (s.dn[42][9] + s.dn[199][9]);
        let eq15_e246_d_n10: f64 = (s.dn[42][10] + s.dn[199][10]);
        let eq15_e246_d_n11: f64 = (s.dn[42][11] + s.dn[199][11]);
        let eq15_e246_d_n12: f64 = (s.dn[42][12] + s.dn[199][12]);
        let eq15_e246_d_n13: f64 = (s.dn[42][13] + s.dn[199][13]);
        let eq15_e246_d_n14: f64 = (s.dn[42][14] + s.dn[199][14]);
        let eq15_e246_d_b0: f64 = (s.db[42][0] + s.db[199][0]);
        let eq15_e246_d_b1: f64 = (s.db[42][1] + s.db[199][1]);
        let eq15_e246_d_b2: f64 = (s.db[42][2] + s.db[199][2]);
        let eq15_e246_d_b3: f64 = (s.db[42][3] + s.db[199][3]);
        let eq15_e246_d_b4: f64 = (s.db[42][4] + s.db[199][4]);
        let eq15_e246_d_b5: f64 = (s.db[42][5] + s.db[199][5]);
        let eq15_e247: f64 = (p.p148 * eq15_e246);
        let eq15_e247_d_n0: f64 = (p.p148 * eq15_e246_d_n0);
        let eq15_e247_d_n1: f64 = (p.p148 * eq15_e246_d_n1);
        let eq15_e247_d_n2: f64 = (p.p148 * eq15_e246_d_n2);
        let eq15_e247_d_n3: f64 = (p.p148 * eq15_e246_d_n3);
        let eq15_e247_d_n4: f64 = (p.p148 * eq15_e246_d_n4);
        let eq15_e247_d_n5: f64 = (p.p148 * eq15_e246_d_n5);
        let eq15_e247_d_n6: f64 = (p.p148 * eq15_e246_d_n6);
        let eq15_e247_d_n7: f64 = (p.p148 * eq15_e246_d_n7);
        let eq15_e247_d_n8: f64 = (p.p148 * eq15_e246_d_n8);
        let eq15_e247_d_n9: f64 = (p.p148 * eq15_e246_d_n9);
        let eq15_e247_d_n10: f64 = (p.p148 * eq15_e246_d_n10);
        let eq15_e247_d_n11: f64 = (p.p148 * eq15_e246_d_n11);
        let eq15_e247_d_n12: f64 = (p.p148 * eq15_e246_d_n12);
        let eq15_e247_d_n13: f64 = (p.p148 * eq15_e246_d_n13);
        let eq15_e247_d_n14: f64 = (p.p148 * eq15_e246_d_n14);
        let eq15_e247_d_b0: f64 = (p.p148 * eq15_e246_d_b0);
        let eq15_e247_d_b1: f64 = (p.p148 * eq15_e246_d_b1);
        let eq15_e247_d_b2: f64 = (p.p148 * eq15_e246_d_b2);
        let eq15_e247_d_b3: f64 = (p.p148 * eq15_e246_d_b3);
        let eq15_e247_d_b4: f64 = (p.p148 * eq15_e246_d_b4);
        let eq15_e247_d_b5: f64 = (p.p148 * eq15_e246_d_b5);
        let eq15_e248_q: f64 = eq15_e247;
        let eq15_reactive_node_derivatives: [f64; 15] = [eq15_e247_d_n0, eq15_e247_d_n1, eq15_e247_d_n2, eq15_e247_d_n3, eq15_e247_d_n4, eq15_e247_d_n5, eq15_e247_d_n6, eq15_e247_d_n7, eq15_e247_d_n8, eq15_e247_d_n9, eq15_e247_d_n10, eq15_e247_d_n11, eq15_e247_d_n12, eq15_e247_d_n13, eq15_e247_d_n14];
        let eq15_reactive_branch_derivatives: [f64; 6] = [eq15_e247_d_b0, eq15_e247_d_b1, eq15_e247_d_b2, eq15_e247_d_b3, eq15_e247_d_b4, eq15_e247_d_b5];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            &nodes,
            &eq15_reactive_node_derivatives,
            &branches,
            &eq15_reactive_branch_derivatives,
            self.multiplicity,
        );
    }
}

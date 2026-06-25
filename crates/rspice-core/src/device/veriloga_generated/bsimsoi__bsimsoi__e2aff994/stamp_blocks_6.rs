#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq10_e1600, eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq10_e1584: f64 = (1.0 + s.v[211]);
        let eq10_e1586: f64 = (eq10_e1584 * s.v[622]);
        let eq10_e1586_d_n0: f64 = ((s.dn[211][0] * s.v[622]) + (eq10_e1584 * s.dn[622][0]));
        let eq10_e1586_d_n1: f64 = ((s.dn[211][1] * s.v[622]) + (eq10_e1584 * s.dn[622][1]));
        let eq10_e1586_d_n2: f64 = ((s.dn[211][2] * s.v[622]) + (eq10_e1584 * s.dn[622][2]));
        let eq10_e1586_d_n3: f64 = ((s.dn[211][3] * s.v[622]) + (eq10_e1584 * s.dn[622][3]));
        let eq10_e1586_d_n4: f64 = ((s.dn[211][4] * s.v[622]) + (eq10_e1584 * s.dn[622][4]));
        let eq10_e1586_d_n5: f64 = ((s.dn[211][5] * s.v[622]) + (eq10_e1584 * s.dn[622][5]));
        let eq10_e1586_d_n6: f64 = ((s.dn[211][6] * s.v[622]) + (eq10_e1584 * s.dn[622][6]));
        let eq10_e1586_d_n7: f64 = ((s.dn[211][7] * s.v[622]) + (eq10_e1584 * s.dn[622][7]));
        let eq10_e1586_d_n8: f64 = ((s.dn[211][8] * s.v[622]) + (eq10_e1584 * s.dn[622][8]));
        let eq10_e1586_d_n9: f64 = ((s.dn[211][9] * s.v[622]) + (eq10_e1584 * s.dn[622][9]));
        let eq10_e1586_d_n10: f64 = ((s.dn[211][10] * s.v[622]) + (eq10_e1584 * s.dn[622][10]));
        let eq10_e1586_d_n11: f64 = ((s.dn[211][11] * s.v[622]) + (eq10_e1584 * s.dn[622][11]));
        let eq10_e1586_d_n12: f64 = ((s.dn[211][12] * s.v[622]) + (eq10_e1584 * s.dn[622][12]));
        let eq10_e1586_d_n13: f64 = ((s.dn[211][13] * s.v[622]) + (eq10_e1584 * s.dn[622][13]));
        let eq10_e1588: f64 = (eq10_e1586 * s.v[199]);
        let eq10_e1588_d_n0: f64 = (eq10_e1586_d_n0 * s.v[199]);
        let eq10_e1588_d_n1: f64 = (eq10_e1586_d_n1 * s.v[199]);
        let eq10_e1588_d_n2: f64 = (eq10_e1586_d_n2 * s.v[199]);
        let eq10_e1588_d_n3: f64 = (eq10_e1586_d_n3 * s.v[199]);
        let eq10_e1588_d_n4: f64 = (eq10_e1586_d_n4 * s.v[199]);
        let eq10_e1588_d_n5: f64 = (eq10_e1586_d_n5 * s.v[199]);
        let eq10_e1588_d_n6: f64 = (eq10_e1586_d_n6 * s.v[199]);
        let eq10_e1588_d_n7: f64 = (eq10_e1586_d_n7 * s.v[199]);
        let eq10_e1588_d_n8: f64 = (eq10_e1586_d_n8 * s.v[199]);
        let eq10_e1588_d_n9: f64 = (eq10_e1586_d_n9 * s.v[199]);
        let eq10_e1588_d_n10: f64 = (eq10_e1586_d_n10 * s.v[199]);
        let eq10_e1588_d_n11: f64 = (eq10_e1586_d_n11 * s.v[199]);
        let eq10_e1588_d_n12: f64 = (eq10_e1586_d_n12 * s.v[199]);
        let eq10_e1588_d_n13: f64 = (eq10_e1586_d_n13 * s.v[199]);
        let eq10_e1590: f64 = (eq10_e1588 * s.v[183]);
        let eq10_e1590_d_n0: f64 = (eq10_e1588_d_n0 * s.v[183]);
        let eq10_e1590_d_n1: f64 = (eq10_e1588_d_n1 * s.v[183]);
        let eq10_e1590_d_n2: f64 = (eq10_e1588_d_n2 * s.v[183]);
        let eq10_e1590_d_n3: f64 = (eq10_e1588_d_n3 * s.v[183]);
        let eq10_e1590_d_n4: f64 = (eq10_e1588_d_n4 * s.v[183]);
        let eq10_e1590_d_n5: f64 = (eq10_e1588_d_n5 * s.v[183]);
        let eq10_e1590_d_n6: f64 = (eq10_e1588_d_n6 * s.v[183]);
        let eq10_e1590_d_n7: f64 = (eq10_e1588_d_n7 * s.v[183]);
        let eq10_e1590_d_n8: f64 = (eq10_e1588_d_n8 * s.v[183]);
        let eq10_e1590_d_n9: f64 = (eq10_e1588_d_n9 * s.v[183]);
        let eq10_e1590_d_n10: f64 = (eq10_e1588_d_n10 * s.v[183]);
        let eq10_e1590_d_n11: f64 = (eq10_e1588_d_n11 * s.v[183]);
        let eq10_e1590_d_n12: f64 = (eq10_e1588_d_n12 * s.v[183]);
        let eq10_e1590_d_n13: f64 = (eq10_e1588_d_n13 * s.v[183]);
        let eq10_e1592: f64 = (eq10_e1590 * p.p2);
        let eq10_e1592_d_n0: f64 = (eq10_e1590_d_n0 * p.p2);
        let eq10_e1592_d_n1: f64 = (eq10_e1590_d_n1 * p.p2);
        let eq10_e1592_d_n2: f64 = (eq10_e1590_d_n2 * p.p2);
        let eq10_e1592_d_n3: f64 = (eq10_e1590_d_n3 * p.p2);
        let eq10_e1592_d_n4: f64 = (eq10_e1590_d_n4 * p.p2);
        let eq10_e1592_d_n5: f64 = (eq10_e1590_d_n5 * p.p2);
        let eq10_e1592_d_n6: f64 = (eq10_e1590_d_n6 * p.p2);
        let eq10_e1592_d_n7: f64 = (eq10_e1590_d_n7 * p.p2);
        let eq10_e1592_d_n8: f64 = (eq10_e1590_d_n8 * p.p2);
        let eq10_e1592_d_n9: f64 = (eq10_e1590_d_n9 * p.p2);
        let eq10_e1592_d_n10: f64 = (eq10_e1590_d_n10 * p.p2);
        let eq10_e1592_d_n11: f64 = (eq10_e1590_d_n11 * p.p2);
        let eq10_e1592_d_n12: f64 = (eq10_e1590_d_n12 * p.p2);
        let eq10_e1592_d_n13: f64 = (eq10_e1590_d_n13 * p.p2);
        let eq10_e1594: f64 = (eq10_e1592 * s.v[184]);
        let eq10_e1594_d_n0: f64 = (eq10_e1592_d_n0 * s.v[184]);
        let eq10_e1594_d_n1: f64 = (eq10_e1592_d_n1 * s.v[184]);
        let eq10_e1594_d_n2: f64 = (eq10_e1592_d_n2 * s.v[184]);
        let eq10_e1594_d_n3: f64 = (eq10_e1592_d_n3 * s.v[184]);
        let eq10_e1594_d_n4: f64 = (eq10_e1592_d_n4 * s.v[184]);
        let eq10_e1594_d_n5: f64 = (eq10_e1592_d_n5 * s.v[184]);
        let eq10_e1594_d_n6: f64 = (eq10_e1592_d_n6 * s.v[184]);
        let eq10_e1594_d_n7: f64 = (eq10_e1592_d_n7 * s.v[184]);
        let eq10_e1594_d_n8: f64 = (eq10_e1592_d_n8 * s.v[184]);
        let eq10_e1594_d_n9: f64 = (eq10_e1592_d_n9 * s.v[184]);
        let eq10_e1594_d_n10: f64 = (eq10_e1592_d_n10 * s.v[184]);
        let eq10_e1594_d_n11: f64 = (eq10_e1592_d_n11 * s.v[184]);
        let eq10_e1594_d_n12: f64 = (eq10_e1592_d_n12 * s.v[184]);
        let eq10_e1594_d_n13: f64 = (eq10_e1592_d_n13 * s.v[184]);
        let eq10_e1596: f64 = (eq10_e1594 * (nv12 - 0.0));
        let eq10_e1596_d_n0: f64 = (eq10_e1594_d_n0 * (nv12 - 0.0));
        let eq10_e1596_d_n1: f64 = (eq10_e1594_d_n1 * (nv12 - 0.0));
        let eq10_e1596_d_n2: f64 = (eq10_e1594_d_n2 * (nv12 - 0.0));
        let eq10_e1596_d_n3: f64 = (eq10_e1594_d_n3 * (nv12 - 0.0));
        let eq10_e1596_d_n4: f64 = (eq10_e1594_d_n4 * (nv12 - 0.0));
        let eq10_e1596_d_n5: f64 = (eq10_e1594_d_n5 * (nv12 - 0.0));
        let eq10_e1596_d_n6: f64 = (eq10_e1594_d_n6 * (nv12 - 0.0));
        let eq10_e1596_d_n7: f64 = (eq10_e1594_d_n7 * (nv12 - 0.0));
        let eq10_e1596_d_n8: f64 = (eq10_e1594_d_n8 * (nv12 - 0.0));
        let eq10_e1596_d_n9: f64 = (eq10_e1594_d_n9 * (nv12 - 0.0));
        let eq10_e1596_d_n10: f64 = (eq10_e1594_d_n10 * (nv12 - 0.0));
        let eq10_e1596_d_n11: f64 = (eq10_e1594_d_n11 * (nv12 - 0.0));
        let eq10_e1596_d_n12: f64 = ((eq10_e1594_d_n12 * (nv12 - 0.0)) + eq10_e1594);
        let eq10_e1596_d_n13: f64 = (eq10_e1594_d_n13 * (nv12 - 0.0));
        let eq10_e1597: f64 = (0.5 * eq10_e1596);
        let eq10_e1597_d_n0: f64 = (0.5 * eq10_e1596_d_n0);
        let eq10_e1597_d_n1: f64 = (0.5 * eq10_e1596_d_n1);
        let eq10_e1597_d_n2: f64 = (0.5 * eq10_e1596_d_n2);
        let eq10_e1597_d_n3: f64 = (0.5 * eq10_e1596_d_n3);
        let eq10_e1597_d_n4: f64 = (0.5 * eq10_e1596_d_n4);
        let eq10_e1597_d_n5: f64 = (0.5 * eq10_e1596_d_n5);
        let eq10_e1597_d_n6: f64 = (0.5 * eq10_e1596_d_n6);
        let eq10_e1597_d_n7: f64 = (0.5 * eq10_e1596_d_n7);
        let eq10_e1597_d_n8: f64 = (0.5 * eq10_e1596_d_n8);
        let eq10_e1597_d_n9: f64 = (0.5 * eq10_e1596_d_n9);
        let eq10_e1597_d_n10: f64 = (0.5 * eq10_e1596_d_n10);
        let eq10_e1597_d_n11: f64 = (0.5 * eq10_e1596_d_n11);
        let eq10_e1597_d_n12: f64 = (0.5 * eq10_e1596_d_n12);
        let eq10_e1597_d_n13: f64 = (0.5 * eq10_e1596_d_n13);
        let eq10_e1598: f64 = self.eval_ddt(1, eq10_e1597);
        let eq10_e1598_d_n0: f64 = self.ddt_jacobian(eq10_e1597_d_n0);
        let eq10_e1598_d_n1: f64 = self.ddt_jacobian(eq10_e1597_d_n1);
        let eq10_e1598_d_n2: f64 = self.ddt_jacobian(eq10_e1597_d_n2);
        let eq10_e1598_d_n3: f64 = self.ddt_jacobian(eq10_e1597_d_n3);
        let eq10_e1598_d_n4: f64 = self.ddt_jacobian(eq10_e1597_d_n4);
        let eq10_e1598_d_n5: f64 = self.ddt_jacobian(eq10_e1597_d_n5);
        let eq10_e1598_d_n6: f64 = self.ddt_jacobian(eq10_e1597_d_n6);
        let eq10_e1598_d_n7: f64 = self.ddt_jacobian(eq10_e1597_d_n7);
        let eq10_e1598_d_n8: f64 = self.ddt_jacobian(eq10_e1597_d_n8);
        let eq10_e1598_d_n9: f64 = self.ddt_jacobian(eq10_e1597_d_n9);
        let eq10_e1598_d_n10: f64 = self.ddt_jacobian(eq10_e1597_d_n10);
        let eq10_e1598_d_n11: f64 = self.ddt_jacobian(eq10_e1597_d_n11);
        let eq10_e1598_d_n12: f64 = self.ddt_jacobian(eq10_e1597_d_n12);
        let eq10_e1598_d_n13: f64 = self.ddt_jacobian(eq10_e1597_d_n13);
        (eq10_e1598, eq10_e1598_d_n0, eq10_e1598_d_n1, eq10_e1598_d_n2, eq10_e1598_d_n3, eq10_e1598_d_n4, eq10_e1598_d_n5, eq10_e1598_d_n6, eq10_e1598_d_n7, eq10_e1598_d_n8, eq10_e1598_d_n9, eq10_e1598_d_n10, eq10_e1598_d_n11, eq10_e1598_d_n12, eq10_e1598_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1600;
        let eq10_node_derivatives: [f64; 14] = [eq10_e1600_d_n0, eq10_e1600_d_n1, eq10_e1600_d_n2, eq10_e1600_d_n3, eq10_e1600_d_n4, eq10_e1600_d_n5, eq10_e1600_d_n6, eq10_e1600_d_n7, eq10_e1600_d_n8, eq10_e1600_d_n9, eq10_e1600_d_n10, eq10_e1600_d_n11, eq10_e1600_d_n12, eq10_e1600_d_n13];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq10_value),
            &nodes,
            &eq10_node_derivatives,
            &branches,
            &eq10_branch_derivatives,
            self.multiplicity,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq11_e1626, eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13,) = {
    if ((s.v[1620] != 0.0) && ((s.v[1794] != 0.0) && (!(s.v[1793] != 0.0)))) {
        let eq11_e1610: f64 = (1.0 - s.v[211]);
        let eq11_e1610_d_n0: f64 = (-s.dn[211][0]);
        let eq11_e1610_d_n1: f64 = (-s.dn[211][1]);
        let eq11_e1610_d_n2: f64 = (-s.dn[211][2]);
        let eq11_e1610_d_n3: f64 = (-s.dn[211][3]);
        let eq11_e1610_d_n4: f64 = (-s.dn[211][4]);
        let eq11_e1610_d_n5: f64 = (-s.dn[211][5]);
        let eq11_e1610_d_n6: f64 = (-s.dn[211][6]);
        let eq11_e1610_d_n7: f64 = (-s.dn[211][7]);
        let eq11_e1610_d_n8: f64 = (-s.dn[211][8]);
        let eq11_e1610_d_n9: f64 = (-s.dn[211][9]);
        let eq11_e1610_d_n10: f64 = (-s.dn[211][10]);
        let eq11_e1610_d_n11: f64 = (-s.dn[211][11]);
        let eq11_e1610_d_n12: f64 = (-s.dn[211][12]);
        let eq11_e1610_d_n13: f64 = (-s.dn[211][13]);
        let eq11_e1612: f64 = (eq11_e1610 * s.v[622]);
        let eq11_e1612_d_n0: f64 = ((eq11_e1610_d_n0 * s.v[622]) + (eq11_e1610 * s.dn[622][0]));
        let eq11_e1612_d_n1: f64 = ((eq11_e1610_d_n1 * s.v[622]) + (eq11_e1610 * s.dn[622][1]));
        let eq11_e1612_d_n2: f64 = ((eq11_e1610_d_n2 * s.v[622]) + (eq11_e1610 * s.dn[622][2]));
        let eq11_e1612_d_n3: f64 = ((eq11_e1610_d_n3 * s.v[622]) + (eq11_e1610 * s.dn[622][3]));
        let eq11_e1612_d_n4: f64 = ((eq11_e1610_d_n4 * s.v[622]) + (eq11_e1610 * s.dn[622][4]));
        let eq11_e1612_d_n5: f64 = ((eq11_e1610_d_n5 * s.v[622]) + (eq11_e1610 * s.dn[622][5]));
        let eq11_e1612_d_n6: f64 = ((eq11_e1610_d_n6 * s.v[622]) + (eq11_e1610 * s.dn[622][6]));
        let eq11_e1612_d_n7: f64 = ((eq11_e1610_d_n7 * s.v[622]) + (eq11_e1610 * s.dn[622][7]));
        let eq11_e1612_d_n8: f64 = ((eq11_e1610_d_n8 * s.v[622]) + (eq11_e1610 * s.dn[622][8]));
        let eq11_e1612_d_n9: f64 = ((eq11_e1610_d_n9 * s.v[622]) + (eq11_e1610 * s.dn[622][9]));
        let eq11_e1612_d_n10: f64 = ((eq11_e1610_d_n10 * s.v[622]) + (eq11_e1610 * s.dn[622][10]));
        let eq11_e1612_d_n11: f64 = ((eq11_e1610_d_n11 * s.v[622]) + (eq11_e1610 * s.dn[622][11]));
        let eq11_e1612_d_n12: f64 = ((eq11_e1610_d_n12 * s.v[622]) + (eq11_e1610 * s.dn[622][12]));
        let eq11_e1612_d_n13: f64 = ((eq11_e1610_d_n13 * s.v[622]) + (eq11_e1610 * s.dn[622][13]));
        let eq11_e1614: f64 = (eq11_e1612 * s.v[199]);
        let eq11_e1614_d_n0: f64 = (eq11_e1612_d_n0 * s.v[199]);
        let eq11_e1614_d_n1: f64 = (eq11_e1612_d_n1 * s.v[199]);
        let eq11_e1614_d_n2: f64 = (eq11_e1612_d_n2 * s.v[199]);
        let eq11_e1614_d_n3: f64 = (eq11_e1612_d_n3 * s.v[199]);
        let eq11_e1614_d_n4: f64 = (eq11_e1612_d_n4 * s.v[199]);
        let eq11_e1614_d_n5: f64 = (eq11_e1612_d_n5 * s.v[199]);
        let eq11_e1614_d_n6: f64 = (eq11_e1612_d_n6 * s.v[199]);
        let eq11_e1614_d_n7: f64 = (eq11_e1612_d_n7 * s.v[199]);
        let eq11_e1614_d_n8: f64 = (eq11_e1612_d_n8 * s.v[199]);
        let eq11_e1614_d_n9: f64 = (eq11_e1612_d_n9 * s.v[199]);
        let eq11_e1614_d_n10: f64 = (eq11_e1612_d_n10 * s.v[199]);
        let eq11_e1614_d_n11: f64 = (eq11_e1612_d_n11 * s.v[199]);
        let eq11_e1614_d_n12: f64 = (eq11_e1612_d_n12 * s.v[199]);
        let eq11_e1614_d_n13: f64 = (eq11_e1612_d_n13 * s.v[199]);
        let eq11_e1616: f64 = (eq11_e1614 * s.v[183]);
        let eq11_e1616_d_n0: f64 = (eq11_e1614_d_n0 * s.v[183]);
        let eq11_e1616_d_n1: f64 = (eq11_e1614_d_n1 * s.v[183]);
        let eq11_e1616_d_n2: f64 = (eq11_e1614_d_n2 * s.v[183]);
        let eq11_e1616_d_n3: f64 = (eq11_e1614_d_n3 * s.v[183]);
        let eq11_e1616_d_n4: f64 = (eq11_e1614_d_n4 * s.v[183]);
        let eq11_e1616_d_n5: f64 = (eq11_e1614_d_n5 * s.v[183]);
        let eq11_e1616_d_n6: f64 = (eq11_e1614_d_n6 * s.v[183]);
        let eq11_e1616_d_n7: f64 = (eq11_e1614_d_n7 * s.v[183]);
        let eq11_e1616_d_n8: f64 = (eq11_e1614_d_n8 * s.v[183]);
        let eq11_e1616_d_n9: f64 = (eq11_e1614_d_n9 * s.v[183]);
        let eq11_e1616_d_n10: f64 = (eq11_e1614_d_n10 * s.v[183]);
        let eq11_e1616_d_n11: f64 = (eq11_e1614_d_n11 * s.v[183]);
        let eq11_e1616_d_n12: f64 = (eq11_e1614_d_n12 * s.v[183]);
        let eq11_e1616_d_n13: f64 = (eq11_e1614_d_n13 * s.v[183]);
        let eq11_e1618: f64 = (eq11_e1616 * p.p2);
        let eq11_e1618_d_n0: f64 = (eq11_e1616_d_n0 * p.p2);
        let eq11_e1618_d_n1: f64 = (eq11_e1616_d_n1 * p.p2);
        let eq11_e1618_d_n2: f64 = (eq11_e1616_d_n2 * p.p2);
        let eq11_e1618_d_n3: f64 = (eq11_e1616_d_n3 * p.p2);
        let eq11_e1618_d_n4: f64 = (eq11_e1616_d_n4 * p.p2);
        let eq11_e1618_d_n5: f64 = (eq11_e1616_d_n5 * p.p2);
        let eq11_e1618_d_n6: f64 = (eq11_e1616_d_n6 * p.p2);
        let eq11_e1618_d_n7: f64 = (eq11_e1616_d_n7 * p.p2);
        let eq11_e1618_d_n8: f64 = (eq11_e1616_d_n8 * p.p2);
        let eq11_e1618_d_n9: f64 = (eq11_e1616_d_n9 * p.p2);
        let eq11_e1618_d_n10: f64 = (eq11_e1616_d_n10 * p.p2);
        let eq11_e1618_d_n11: f64 = (eq11_e1616_d_n11 * p.p2);
        let eq11_e1618_d_n12: f64 = (eq11_e1616_d_n12 * p.p2);
        let eq11_e1618_d_n13: f64 = (eq11_e1616_d_n13 * p.p2);
        let eq11_e1620: f64 = (eq11_e1618 * s.v[184]);
        let eq11_e1620_d_n0: f64 = (eq11_e1618_d_n0 * s.v[184]);
        let eq11_e1620_d_n1: f64 = (eq11_e1618_d_n1 * s.v[184]);
        let eq11_e1620_d_n2: f64 = (eq11_e1618_d_n2 * s.v[184]);
        let eq11_e1620_d_n3: f64 = (eq11_e1618_d_n3 * s.v[184]);
        let eq11_e1620_d_n4: f64 = (eq11_e1618_d_n4 * s.v[184]);
        let eq11_e1620_d_n5: f64 = (eq11_e1618_d_n5 * s.v[184]);
        let eq11_e1620_d_n6: f64 = (eq11_e1618_d_n6 * s.v[184]);
        let eq11_e1620_d_n7: f64 = (eq11_e1618_d_n7 * s.v[184]);
        let eq11_e1620_d_n8: f64 = (eq11_e1618_d_n8 * s.v[184]);
        let eq11_e1620_d_n9: f64 = (eq11_e1618_d_n9 * s.v[184]);
        let eq11_e1620_d_n10: f64 = (eq11_e1618_d_n10 * s.v[184]);
        let eq11_e1620_d_n11: f64 = (eq11_e1618_d_n11 * s.v[184]);
        let eq11_e1620_d_n12: f64 = (eq11_e1618_d_n12 * s.v[184]);
        let eq11_e1620_d_n13: f64 = (eq11_e1618_d_n13 * s.v[184]);
        let eq11_e1622: f64 = (eq11_e1620 * (nv12 - 0.0));
        let eq11_e1622_d_n0: f64 = (eq11_e1620_d_n0 * (nv12 - 0.0));
        let eq11_e1622_d_n1: f64 = (eq11_e1620_d_n1 * (nv12 - 0.0));
        let eq11_e1622_d_n2: f64 = (eq11_e1620_d_n2 * (nv12 - 0.0));
        let eq11_e1622_d_n3: f64 = (eq11_e1620_d_n3 * (nv12 - 0.0));
        let eq11_e1622_d_n4: f64 = (eq11_e1620_d_n4 * (nv12 - 0.0));
        let eq11_e1622_d_n5: f64 = (eq11_e1620_d_n5 * (nv12 - 0.0));
        let eq11_e1622_d_n6: f64 = (eq11_e1620_d_n6 * (nv12 - 0.0));
        let eq11_e1622_d_n7: f64 = (eq11_e1620_d_n7 * (nv12 - 0.0));
        let eq11_e1622_d_n8: f64 = (eq11_e1620_d_n8 * (nv12 - 0.0));
        let eq11_e1622_d_n9: f64 = (eq11_e1620_d_n9 * (nv12 - 0.0));
        let eq11_e1622_d_n10: f64 = (eq11_e1620_d_n10 * (nv12 - 0.0));
        let eq11_e1622_d_n11: f64 = (eq11_e1620_d_n11 * (nv12 - 0.0));
        let eq11_e1622_d_n12: f64 = ((eq11_e1620_d_n12 * (nv12 - 0.0)) + eq11_e1620);
        let eq11_e1622_d_n13: f64 = (eq11_e1620_d_n13 * (nv12 - 0.0));
        let eq11_e1623: f64 = (0.5 * eq11_e1622);
        let eq11_e1623_d_n0: f64 = (0.5 * eq11_e1622_d_n0);
        let eq11_e1623_d_n1: f64 = (0.5 * eq11_e1622_d_n1);
        let eq11_e1623_d_n2: f64 = (0.5 * eq11_e1622_d_n2);
        let eq11_e1623_d_n3: f64 = (0.5 * eq11_e1622_d_n3);
        let eq11_e1623_d_n4: f64 = (0.5 * eq11_e1622_d_n4);
        let eq11_e1623_d_n5: f64 = (0.5 * eq11_e1622_d_n5);
        let eq11_e1623_d_n6: f64 = (0.5 * eq11_e1622_d_n6);
        let eq11_e1623_d_n7: f64 = (0.5 * eq11_e1622_d_n7);
        let eq11_e1623_d_n8: f64 = (0.5 * eq11_e1622_d_n8);
        let eq11_e1623_d_n9: f64 = (0.5 * eq11_e1622_d_n9);
        let eq11_e1623_d_n10: f64 = (0.5 * eq11_e1622_d_n10);
        let eq11_e1623_d_n11: f64 = (0.5 * eq11_e1622_d_n11);
        let eq11_e1623_d_n12: f64 = (0.5 * eq11_e1622_d_n12);
        let eq11_e1623_d_n13: f64 = (0.5 * eq11_e1622_d_n13);
        let eq11_e1624: f64 = self.eval_ddt(2, eq11_e1623);
        let eq11_e1624_d_n0: f64 = self.ddt_jacobian(eq11_e1623_d_n0);
        let eq11_e1624_d_n1: f64 = self.ddt_jacobian(eq11_e1623_d_n1);
        let eq11_e1624_d_n2: f64 = self.ddt_jacobian(eq11_e1623_d_n2);
        let eq11_e1624_d_n3: f64 = self.ddt_jacobian(eq11_e1623_d_n3);
        let eq11_e1624_d_n4: f64 = self.ddt_jacobian(eq11_e1623_d_n4);
        let eq11_e1624_d_n5: f64 = self.ddt_jacobian(eq11_e1623_d_n5);
        let eq11_e1624_d_n6: f64 = self.ddt_jacobian(eq11_e1623_d_n6);
        let eq11_e1624_d_n7: f64 = self.ddt_jacobian(eq11_e1623_d_n7);
        let eq11_e1624_d_n8: f64 = self.ddt_jacobian(eq11_e1623_d_n8);
        let eq11_e1624_d_n9: f64 = self.ddt_jacobian(eq11_e1623_d_n9);
        let eq11_e1624_d_n10: f64 = self.ddt_jacobian(eq11_e1623_d_n10);
        let eq11_e1624_d_n11: f64 = self.ddt_jacobian(eq11_e1623_d_n11);
        let eq11_e1624_d_n12: f64 = self.ddt_jacobian(eq11_e1623_d_n12);
        let eq11_e1624_d_n13: f64 = self.ddt_jacobian(eq11_e1623_d_n13);
        (eq11_e1624, eq11_e1624_d_n0, eq11_e1624_d_n1, eq11_e1624_d_n2, eq11_e1624_d_n3, eq11_e1624_d_n4, eq11_e1624_d_n5, eq11_e1624_d_n6, eq11_e1624_d_n7, eq11_e1624_d_n8, eq11_e1624_d_n9, eq11_e1624_d_n10, eq11_e1624_d_n11, eq11_e1624_d_n12, eq11_e1624_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1626;
        let eq11_node_derivatives: [f64; 14] = [eq11_e1626_d_n0, eq11_e1626_d_n1, eq11_e1626_d_n2, eq11_e1626_d_n3, eq11_e1626_d_n4, eq11_e1626_d_n5, eq11_e1626_d_n6, eq11_e1626_d_n7, eq11_e1626_d_n8, eq11_e1626_d_n9, eq11_e1626_d_n10, eq11_e1626_d_n11, eq11_e1626_d_n12, eq11_e1626_d_n13];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq11_value),
            &nodes,
            &eq11_node_derivatives,
            &branches,
            &eq11_branch_derivatives,
            self.multiplicity,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq12_e1630, eq12_e1630_d_n13,) = {
    if (s.v[1620] != 0.0) {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1630;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
            self.multiplicity * (eq12_value),
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * eq12_e1630_d_n13),
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq13_e1634, eq13_e1634_d_n12,) = {
    if (s.v[1620] != 0.0) {
        ((nv12 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1634;
        stamper.stamp_current(
            Some(nodes[12]),
            None,
            self.multiplicity * (eq13_value),
            &[
                GeneratedDerivative::node(nodes[12], self.multiplicity * eq13_e1634_d_n12),
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
        let (eq14_e1649,) = {
    if ((s.v[1620] != 0.0) && (s.v[1797] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq14_value: f64 = eq14_e1649;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[7]),
            self.multiplicity * (eq14_value),
            &[
            ],
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
        let (eq15_e1664,) = {
    if ((s.v[1620] != 0.0) && (s.v[1797] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq15_value: f64 = eq15_e1664;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[6]),
            self.multiplicity * (eq15_value),
            &[
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
        let (eq16_e1677,) = {
    if ((s.v[1620] != 0.0) && (s.v[1798] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1677;
        stamper.stamp_current(
            Some(nodes[8]),
            Some(nodes[10]),
            self.multiplicity * (eq16_value),
            &[
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
        let (eq17_e1686, eq17_e1686_d_n0, eq17_e1686_d_n1, eq17_e1686_d_n2, eq17_e1686_d_n3, eq17_e1686_d_n4, eq17_e1686_d_n5, eq17_e1686_d_n6, eq17_e1686_d_n7, eq17_e1686_d_n8, eq17_e1686_d_n9, eq17_e1686_d_n10, eq17_e1686_d_n11, eq17_e1686_d_n12, eq17_e1686_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1947] != 0.0)) {
        let eq17_e1684: f64 = (s.v[379] * s.v[974]);
        let eq17_e1684_d_n0: f64 = ((s.dn[379][0] * s.v[974]) + (s.v[379] * s.dn[974][0]));
        let eq17_e1684_d_n1: f64 = ((s.dn[379][1] * s.v[974]) + (s.v[379] * s.dn[974][1]));
        let eq17_e1684_d_n2: f64 = ((s.dn[379][2] * s.v[974]) + (s.v[379] * s.dn[974][2]));
        let eq17_e1684_d_n3: f64 = ((s.dn[379][3] * s.v[974]) + (s.v[379] * s.dn[974][3]));
        let eq17_e1684_d_n4: f64 = ((s.dn[379][4] * s.v[974]) + (s.v[379] * s.dn[974][4]));
        let eq17_e1684_d_n5: f64 = ((s.dn[379][5] * s.v[974]) + (s.v[379] * s.dn[974][5]));
        let eq17_e1684_d_n6: f64 = ((s.dn[379][6] * s.v[974]) + (s.v[379] * s.dn[974][6]));
        let eq17_e1684_d_n7: f64 = ((s.dn[379][7] * s.v[974]) + (s.v[379] * s.dn[974][7]));
        let eq17_e1684_d_n8: f64 = ((s.dn[379][8] * s.v[974]) + (s.v[379] * s.dn[974][8]));
        let eq17_e1684_d_n9: f64 = ((s.dn[379][9] * s.v[974]) + (s.v[379] * s.dn[974][9]));
        let eq17_e1684_d_n10: f64 = ((s.dn[379][10] * s.v[974]) + (s.v[379] * s.dn[974][10]));
        let eq17_e1684_d_n11: f64 = ((s.dn[379][11] * s.v[974]) + (s.v[379] * s.dn[974][11]));
        let eq17_e1684_d_n12: f64 = ((s.dn[379][12] * s.v[974]) + (s.v[379] * s.dn[974][12]));
        let eq17_e1684_d_n13: f64 = ((s.dn[379][13] * s.v[974]) + (s.v[379] * s.dn[974][13]));
        (eq17_e1684, eq17_e1684_d_n0, eq17_e1684_d_n1, eq17_e1684_d_n2, eq17_e1684_d_n3, eq17_e1684_d_n4, eq17_e1684_d_n5, eq17_e1684_d_n6, eq17_e1684_d_n7, eq17_e1684_d_n8, eq17_e1684_d_n9, eq17_e1684_d_n10, eq17_e1684_d_n11, eq17_e1684_d_n12, eq17_e1684_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1686;
        let eq17_node_derivatives: [f64; 14] = [eq17_e1686_d_n0, eq17_e1686_d_n1, eq17_e1686_d_n2, eq17_e1686_d_n3, eq17_e1686_d_n4, eq17_e1686_d_n5, eq17_e1686_d_n6, eq17_e1686_d_n7, eq17_e1686_d_n8, eq17_e1686_d_n9, eq17_e1686_d_n10, eq17_e1686_d_n11, eq17_e1686_d_n12, eq17_e1686_d_n13];
        let eq17_branch_derivatives: [f64; 0] = [];
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
        let (eq18_e1698,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1950] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq18_value: f64 = eq18_e1698;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq18_value),
            &[
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
        let (eq19_e1711,) = {
    if ((!(s.v[1620] != 0.0)) && (!(s.v[1950] != 0.0))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e1711;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq19_value),
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
        let (eq20_e1720,) = {
    if ((!(s.v[1620] != 0.0)) && (s.v[1964] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1720;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
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
        let (eq21_e1732,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq21_value: f64 = eq21_e1732;
        stamper.stamp_current(
            Some(nodes[13]),
            None,
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
        let (eq22_e1750,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1750;
        stamper.stamp_current(
            Some(nodes[12]),
            None,
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq23_e1763, eq23_e1763_d_n0, eq23_e1763_d_n1, eq23_e1763_d_n2, eq23_e1763_d_n3, eq23_e1763_d_n4, eq23_e1763_d_n5, eq23_e1763_d_n6, eq23_e1763_d_n7, eq23_e1763_d_n8, eq23_e1763_d_n9, eq23_e1763_d_n10, eq23_e1763_d_n11, eq23_e1763_d_n12, eq23_e1763_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq23_e1759: f64 = (-s.v[629]);
        let eq23_e1759_d_n0: f64 = (-s.dn[629][0]);
        let eq23_e1759_d_n1: f64 = (-s.dn[629][1]);
        let eq23_e1759_d_n2: f64 = (-s.dn[629][2]);
        let eq23_e1759_d_n3: f64 = (-s.dn[629][3]);
        let eq23_e1759_d_n4: f64 = (-s.dn[629][4]);
        let eq23_e1759_d_n5: f64 = (-s.dn[629][5]);
        let eq23_e1759_d_n6: f64 = (-s.dn[629][6]);
        let eq23_e1759_d_n7: f64 = (-s.dn[629][7]);
        let eq23_e1759_d_n8: f64 = (-s.dn[629][8]);
        let eq23_e1759_d_n9: f64 = (-s.dn[629][9]);
        let eq23_e1759_d_n10: f64 = (-s.dn[629][10]);
        let eq23_e1759_d_n11: f64 = (-s.dn[629][11]);
        let eq23_e1759_d_n12: f64 = (-s.dn[629][12]);
        let eq23_e1759_d_n13: f64 = (-s.dn[629][13]);
        let eq23_e1761: f64 = (eq23_e1759 * (nv13 - 0.0));
        let eq23_e1761_d_n0: f64 = (eq23_e1759_d_n0 * (nv13 - 0.0));
        let eq23_e1761_d_n1: f64 = (eq23_e1759_d_n1 * (nv13 - 0.0));
        let eq23_e1761_d_n2: f64 = (eq23_e1759_d_n2 * (nv13 - 0.0));
        let eq23_e1761_d_n3: f64 = (eq23_e1759_d_n3 * (nv13 - 0.0));
        let eq23_e1761_d_n4: f64 = (eq23_e1759_d_n4 * (nv13 - 0.0));
        let eq23_e1761_d_n5: f64 = (eq23_e1759_d_n5 * (nv13 - 0.0));
        let eq23_e1761_d_n6: f64 = (eq23_e1759_d_n6 * (nv13 - 0.0));
        let eq23_e1761_d_n7: f64 = (eq23_e1759_d_n7 * (nv13 - 0.0));
        let eq23_e1761_d_n8: f64 = (eq23_e1759_d_n8 * (nv13 - 0.0));
        let eq23_e1761_d_n9: f64 = (eq23_e1759_d_n9 * (nv13 - 0.0));
        let eq23_e1761_d_n10: f64 = (eq23_e1759_d_n10 * (nv13 - 0.0));
        let eq23_e1761_d_n11: f64 = (eq23_e1759_d_n11 * (nv13 - 0.0));
        let eq23_e1761_d_n12: f64 = (eq23_e1759_d_n12 * (nv13 - 0.0));
        let eq23_e1761_d_n13: f64 = ((eq23_e1759_d_n13 * (nv13 - 0.0)) + eq23_e1759);
        (eq23_e1761, eq23_e1761_d_n0, eq23_e1761_d_n1, eq23_e1761_d_n2, eq23_e1761_d_n3, eq23_e1761_d_n4, eq23_e1761_d_n5, eq23_e1761_d_n6, eq23_e1761_d_n7, eq23_e1761_d_n8, eq23_e1761_d_n9, eq23_e1761_d_n10, eq23_e1761_d_n11, eq23_e1761_d_n12, eq23_e1761_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1763;
        let eq23_node_derivatives: [f64; 14] = [eq23_e1763_d_n0, eq23_e1763_d_n1, eq23_e1763_d_n2, eq23_e1763_d_n3, eq23_e1763_d_n4, eq23_e1763_d_n5, eq23_e1763_d_n6, eq23_e1763_d_n7, eq23_e1763_d_n8, eq23_e1763_d_n9, eq23_e1763_d_n10, eq23_e1763_d_n11, eq23_e1763_d_n12, eq23_e1763_d_n13];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq24_e1784, eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        let eq24_e1773: f64 = (s.v[622] * s.v[199]);
        let eq24_e1773_d_n0: f64 = (s.dn[622][0] * s.v[199]);
        let eq24_e1773_d_n1: f64 = (s.dn[622][1] * s.v[199]);
        let eq24_e1773_d_n2: f64 = (s.dn[622][2] * s.v[199]);
        let eq24_e1773_d_n3: f64 = (s.dn[622][3] * s.v[199]);
        let eq24_e1773_d_n4: f64 = (s.dn[622][4] * s.v[199]);
        let eq24_e1773_d_n5: f64 = (s.dn[622][5] * s.v[199]);
        let eq24_e1773_d_n6: f64 = (s.dn[622][6] * s.v[199]);
        let eq24_e1773_d_n7: f64 = (s.dn[622][7] * s.v[199]);
        let eq24_e1773_d_n8: f64 = (s.dn[622][8] * s.v[199]);
        let eq24_e1773_d_n9: f64 = (s.dn[622][9] * s.v[199]);
        let eq24_e1773_d_n10: f64 = (s.dn[622][10] * s.v[199]);
        let eq24_e1773_d_n11: f64 = (s.dn[622][11] * s.v[199]);
        let eq24_e1773_d_n12: f64 = (s.dn[622][12] * s.v[199]);
        let eq24_e1773_d_n13: f64 = (s.dn[622][13] * s.v[199]);
        let eq24_e1775: f64 = (eq24_e1773 * s.v[183]);
        let eq24_e1775_d_n0: f64 = (eq24_e1773_d_n0 * s.v[183]);
        let eq24_e1775_d_n1: f64 = (eq24_e1773_d_n1 * s.v[183]);
        let eq24_e1775_d_n2: f64 = (eq24_e1773_d_n2 * s.v[183]);
        let eq24_e1775_d_n3: f64 = (eq24_e1773_d_n3 * s.v[183]);
        let eq24_e1775_d_n4: f64 = (eq24_e1773_d_n4 * s.v[183]);
        let eq24_e1775_d_n5: f64 = (eq24_e1773_d_n5 * s.v[183]);
        let eq24_e1775_d_n6: f64 = (eq24_e1773_d_n6 * s.v[183]);
        let eq24_e1775_d_n7: f64 = (eq24_e1773_d_n7 * s.v[183]);
        let eq24_e1775_d_n8: f64 = (eq24_e1773_d_n8 * s.v[183]);
        let eq24_e1775_d_n9: f64 = (eq24_e1773_d_n9 * s.v[183]);
        let eq24_e1775_d_n10: f64 = (eq24_e1773_d_n10 * s.v[183]);
        let eq24_e1775_d_n11: f64 = (eq24_e1773_d_n11 * s.v[183]);
        let eq24_e1775_d_n12: f64 = (eq24_e1773_d_n12 * s.v[183]);
        let eq24_e1775_d_n13: f64 = (eq24_e1773_d_n13 * s.v[183]);
        let eq24_e1777: f64 = (eq24_e1775 * p.p2);
        let eq24_e1777_d_n0: f64 = (eq24_e1775_d_n0 * p.p2);
        let eq24_e1777_d_n1: f64 = (eq24_e1775_d_n1 * p.p2);
        let eq24_e1777_d_n2: f64 = (eq24_e1775_d_n2 * p.p2);
        let eq24_e1777_d_n3: f64 = (eq24_e1775_d_n3 * p.p2);
        let eq24_e1777_d_n4: f64 = (eq24_e1775_d_n4 * p.p2);
        let eq24_e1777_d_n5: f64 = (eq24_e1775_d_n5 * p.p2);
        let eq24_e1777_d_n6: f64 = (eq24_e1775_d_n6 * p.p2);
        let eq24_e1777_d_n7: f64 = (eq24_e1775_d_n7 * p.p2);
        let eq24_e1777_d_n8: f64 = (eq24_e1775_d_n8 * p.p2);
        let eq24_e1777_d_n9: f64 = (eq24_e1775_d_n9 * p.p2);
        let eq24_e1777_d_n10: f64 = (eq24_e1775_d_n10 * p.p2);
        let eq24_e1777_d_n11: f64 = (eq24_e1775_d_n11 * p.p2);
        let eq24_e1777_d_n12: f64 = (eq24_e1775_d_n12 * p.p2);
        let eq24_e1777_d_n13: f64 = (eq24_e1775_d_n13 * p.p2);
        let eq24_e1779: f64 = (eq24_e1777 * s.v[184]);
        let eq24_e1779_d_n0: f64 = (eq24_e1777_d_n0 * s.v[184]);
        let eq24_e1779_d_n1: f64 = (eq24_e1777_d_n1 * s.v[184]);
        let eq24_e1779_d_n2: f64 = (eq24_e1777_d_n2 * s.v[184]);
        let eq24_e1779_d_n3: f64 = (eq24_e1777_d_n3 * s.v[184]);
        let eq24_e1779_d_n4: f64 = (eq24_e1777_d_n4 * s.v[184]);
        let eq24_e1779_d_n5: f64 = (eq24_e1777_d_n5 * s.v[184]);
        let eq24_e1779_d_n6: f64 = (eq24_e1777_d_n6 * s.v[184]);
        let eq24_e1779_d_n7: f64 = (eq24_e1777_d_n7 * s.v[184]);
        let eq24_e1779_d_n8: f64 = (eq24_e1777_d_n8 * s.v[184]);
        let eq24_e1779_d_n9: f64 = (eq24_e1777_d_n9 * s.v[184]);
        let eq24_e1779_d_n10: f64 = (eq24_e1777_d_n10 * s.v[184]);
        let eq24_e1779_d_n11: f64 = (eq24_e1777_d_n11 * s.v[184]);
        let eq24_e1779_d_n12: f64 = (eq24_e1777_d_n12 * s.v[184]);
        let eq24_e1779_d_n13: f64 = (eq24_e1777_d_n13 * s.v[184]);
        let eq24_e1781: f64 = (eq24_e1779 * (nv12 - 0.0));
        let eq24_e1781_d_n0: f64 = (eq24_e1779_d_n0 * (nv12 - 0.0));
        let eq24_e1781_d_n1: f64 = (eq24_e1779_d_n1 * (nv12 - 0.0));
        let eq24_e1781_d_n2: f64 = (eq24_e1779_d_n2 * (nv12 - 0.0));
        let eq24_e1781_d_n3: f64 = (eq24_e1779_d_n3 * (nv12 - 0.0));
        let eq24_e1781_d_n4: f64 = (eq24_e1779_d_n4 * (nv12 - 0.0));
        let eq24_e1781_d_n5: f64 = (eq24_e1779_d_n5 * (nv12 - 0.0));
        let eq24_e1781_d_n6: f64 = (eq24_e1779_d_n6 * (nv12 - 0.0));
        let eq24_e1781_d_n7: f64 = (eq24_e1779_d_n7 * (nv12 - 0.0));
        let eq24_e1781_d_n8: f64 = (eq24_e1779_d_n8 * (nv12 - 0.0));
        let eq24_e1781_d_n9: f64 = (eq24_e1779_d_n9 * (nv12 - 0.0));
        let eq24_e1781_d_n10: f64 = (eq24_e1779_d_n10 * (nv12 - 0.0));
        let eq24_e1781_d_n11: f64 = (eq24_e1779_d_n11 * (nv12 - 0.0));
        let eq24_e1781_d_n12: f64 = ((eq24_e1779_d_n12 * (nv12 - 0.0)) + eq24_e1779);
        let eq24_e1781_d_n13: f64 = (eq24_e1779_d_n13 * (nv12 - 0.0));
        let eq24_e1782: f64 = self.eval_ddt(3, eq24_e1781);
        let eq24_e1782_d_n0: f64 = self.ddt_jacobian(eq24_e1781_d_n0);
        let eq24_e1782_d_n1: f64 = self.ddt_jacobian(eq24_e1781_d_n1);
        let eq24_e1782_d_n2: f64 = self.ddt_jacobian(eq24_e1781_d_n2);
        let eq24_e1782_d_n3: f64 = self.ddt_jacobian(eq24_e1781_d_n3);
        let eq24_e1782_d_n4: f64 = self.ddt_jacobian(eq24_e1781_d_n4);
        let eq24_e1782_d_n5: f64 = self.ddt_jacobian(eq24_e1781_d_n5);
        let eq24_e1782_d_n6: f64 = self.ddt_jacobian(eq24_e1781_d_n6);
        let eq24_e1782_d_n7: f64 = self.ddt_jacobian(eq24_e1781_d_n7);
        let eq24_e1782_d_n8: f64 = self.ddt_jacobian(eq24_e1781_d_n8);
        let eq24_e1782_d_n9: f64 = self.ddt_jacobian(eq24_e1781_d_n9);
        let eq24_e1782_d_n10: f64 = self.ddt_jacobian(eq24_e1781_d_n10);
        let eq24_e1782_d_n11: f64 = self.ddt_jacobian(eq24_e1781_d_n11);
        let eq24_e1782_d_n12: f64 = self.ddt_jacobian(eq24_e1781_d_n12);
        let eq24_e1782_d_n13: f64 = self.ddt_jacobian(eq24_e1781_d_n13);
        (eq24_e1782, eq24_e1782_d_n0, eq24_e1782_d_n1, eq24_e1782_d_n2, eq24_e1782_d_n3, eq24_e1782_d_n4, eq24_e1782_d_n5, eq24_e1782_d_n6, eq24_e1782_d_n7, eq24_e1782_d_n8, eq24_e1782_d_n9, eq24_e1782_d_n10, eq24_e1782_d_n11, eq24_e1782_d_n12, eq24_e1782_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1784;
        let eq24_node_derivatives: [f64; 14] = [eq24_e1784_d_n0, eq24_e1784_d_n1, eq24_e1784_d_n2, eq24_e1784_d_n3, eq24_e1784_d_n4, eq24_e1784_d_n5, eq24_e1784_d_n6, eq24_e1784_d_n7, eq24_e1784_d_n8, eq24_e1784_d_n9, eq24_e1784_d_n10, eq24_e1784_d_n11, eq24_e1784_d_n12, eq24_e1784_d_n13];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            None,
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
        let (eq25_e1802,) = {
    if ((!(s.v[1620] != 0.0)) && ((s.v[1965] != 0.0) && (!(s.v[1964] != 0.0)))) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1802;
        stamper.stamp_current(
            Some(nodes[6]),
            Some(nodes[7]),
            self.multiplicity * (eq25_value),
            &[
            ],
        );
    }
}

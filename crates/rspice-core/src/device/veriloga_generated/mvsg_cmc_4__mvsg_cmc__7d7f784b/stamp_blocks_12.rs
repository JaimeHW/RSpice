#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_132_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq132_e1590,) = {
    if (!(s.v[1495] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq132_value: f64 = eq132_e1590;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[12]),
            self.multiplicity * (eq132_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_133_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq133_e1595,) = {
    if (!(s.v[1495] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq133_value: f64 = eq133_e1595;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq133_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_134_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let eq134_e1597: f64 = self.eval_ddt(130, s.v[188]);
        let eq134_e1597_d_n0: f64 = self.ddt_jacobian(s.dn[188][0]);
        let eq134_e1597_d_n1: f64 = self.ddt_jacobian(s.dn[188][1]);
        let eq134_e1597_d_n2: f64 = self.ddt_jacobian(s.dn[188][2]);
        let eq134_e1597_d_n3: f64 = self.ddt_jacobian(s.dn[188][3]);
        let eq134_e1597_d_n4: f64 = self.ddt_jacobian(s.dn[188][4]);
        let eq134_e1597_d_n5: f64 = self.ddt_jacobian(s.dn[188][5]);
        let eq134_e1597_d_n6: f64 = self.ddt_jacobian(s.dn[188][6]);
        let eq134_e1597_d_n7: f64 = self.ddt_jacobian(s.dn[188][7]);
        let eq134_e1597_d_n8: f64 = self.ddt_jacobian(s.dn[188][8]);
        let eq134_e1597_d_n9: f64 = self.ddt_jacobian(s.dn[188][9]);
        let eq134_e1597_d_n10: f64 = self.ddt_jacobian(s.dn[188][10]);
        let eq134_e1597_d_n11: f64 = self.ddt_jacobian(s.dn[188][11]);
        let eq134_e1597_d_n12: f64 = self.ddt_jacobian(s.dn[188][12]);
        let eq134_e1597_d_n13: f64 = self.ddt_jacobian(s.dn[188][13]);
        let eq134_e1597_d_n14: f64 = self.ddt_jacobian(s.dn[188][14]);
        let eq134_e1597_d_n15: f64 = self.ddt_jacobian(s.dn[188][15]);
        let eq134_e1597_d_n16: f64 = self.ddt_jacobian(s.dn[188][16]);
        let eq134_e1597_d_n17: f64 = self.ddt_jacobian(s.dn[188][17]);
        let eq134_e1597_d_n18: f64 = self.ddt_jacobian(s.dn[188][18]);
        let eq134_e1597_d_n19: f64 = self.ddt_jacobian(s.dn[188][19]);
        let eq134_e1597_d_n20: f64 = self.ddt_jacobian(s.dn[188][20]);
        let eq134_e1597_d_n21: f64 = self.ddt_jacobian(s.dn[188][21]);
        let eq134_e1597_d_n22: f64 = self.ddt_jacobian(s.dn[188][22]);
        let eq134_e1597_d_n23: f64 = self.ddt_jacobian(s.dn[188][23]);
        let eq134_e1597_d_n24: f64 = self.ddt_jacobian(s.dn[188][24]);
        let eq134_e1597_d_n25: f64 = self.ddt_jacobian(s.dn[188][25]);
        let eq134_e1597_d_n26: f64 = self.ddt_jacobian(s.dn[188][26]);
        let eq134_e1597_d_n27: f64 = self.ddt_jacobian(s.dn[188][27]);
        let eq134_e1597_d_n28: f64 = self.ddt_jacobian(s.dn[188][28]);
        let eq134_e1597_d_n29: f64 = self.ddt_jacobian(s.dn[188][29]);
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1600_d_n3: f64 = p.p355;
        let eq134_e1600_d_n13: f64 = (-p.p355);
        let eq134_e1601: f64 = self.eval_ddt(131, eq134_e1600);
        let eq134_e1601_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n3: f64 = self.ddt_jacobian(eq134_e1600_d_n3);
        let eq134_e1601_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n13: f64 = self.ddt_jacobian(eq134_e1600_d_n13);
        let eq134_e1601_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq134_e1601_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq134_e1602: f64 = (eq134_e1597 + eq134_e1601);
        let eq134_e1602_d_n0: f64 = (eq134_e1597_d_n0 + eq134_e1601_d_n0);
        let eq134_e1602_d_n1: f64 = (eq134_e1597_d_n1 + eq134_e1601_d_n1);
        let eq134_e1602_d_n2: f64 = (eq134_e1597_d_n2 + eq134_e1601_d_n2);
        let eq134_e1602_d_n3: f64 = (eq134_e1597_d_n3 + eq134_e1601_d_n3);
        let eq134_e1602_d_n4: f64 = (eq134_e1597_d_n4 + eq134_e1601_d_n4);
        let eq134_e1602_d_n5: f64 = (eq134_e1597_d_n5 + eq134_e1601_d_n5);
        let eq134_e1602_d_n6: f64 = (eq134_e1597_d_n6 + eq134_e1601_d_n6);
        let eq134_e1602_d_n7: f64 = (eq134_e1597_d_n7 + eq134_e1601_d_n7);
        let eq134_e1602_d_n8: f64 = (eq134_e1597_d_n8 + eq134_e1601_d_n8);
        let eq134_e1602_d_n9: f64 = (eq134_e1597_d_n9 + eq134_e1601_d_n9);
        let eq134_e1602_d_n10: f64 = (eq134_e1597_d_n10 + eq134_e1601_d_n10);
        let eq134_e1602_d_n11: f64 = (eq134_e1597_d_n11 + eq134_e1601_d_n11);
        let eq134_e1602_d_n12: f64 = (eq134_e1597_d_n12 + eq134_e1601_d_n12);
        let eq134_e1602_d_n13: f64 = (eq134_e1597_d_n13 + eq134_e1601_d_n13);
        let eq134_e1602_d_n14: f64 = (eq134_e1597_d_n14 + eq134_e1601_d_n14);
        let eq134_e1602_d_n15: f64 = (eq134_e1597_d_n15 + eq134_e1601_d_n15);
        let eq134_e1602_d_n16: f64 = (eq134_e1597_d_n16 + eq134_e1601_d_n16);
        let eq134_e1602_d_n17: f64 = (eq134_e1597_d_n17 + eq134_e1601_d_n17);
        let eq134_e1602_d_n18: f64 = (eq134_e1597_d_n18 + eq134_e1601_d_n18);
        let eq134_e1602_d_n19: f64 = (eq134_e1597_d_n19 + eq134_e1601_d_n19);
        let eq134_e1602_d_n20: f64 = (eq134_e1597_d_n20 + eq134_e1601_d_n20);
        let eq134_e1602_d_n21: f64 = (eq134_e1597_d_n21 + eq134_e1601_d_n21);
        let eq134_e1602_d_n22: f64 = (eq134_e1597_d_n22 + eq134_e1601_d_n22);
        let eq134_e1602_d_n23: f64 = (eq134_e1597_d_n23 + eq134_e1601_d_n23);
        let eq134_e1602_d_n24: f64 = (eq134_e1597_d_n24 + eq134_e1601_d_n24);
        let eq134_e1602_d_n25: f64 = (eq134_e1597_d_n25 + eq134_e1601_d_n25);
        let eq134_e1602_d_n26: f64 = (eq134_e1597_d_n26 + eq134_e1601_d_n26);
        let eq134_e1602_d_n27: f64 = (eq134_e1597_d_n27 + eq134_e1601_d_n27);
        let eq134_e1602_d_n28: f64 = (eq134_e1597_d_n28 + eq134_e1601_d_n28);
        let eq134_e1602_d_n29: f64 = (eq134_e1597_d_n29 + eq134_e1601_d_n29);
        let eq134_value: f64 = eq134_e1602;
        let eq134_node_derivatives: [f64; 30] = [eq134_e1602_d_n0, eq134_e1602_d_n1, eq134_e1602_d_n2, eq134_e1602_d_n3, eq134_e1602_d_n4, eq134_e1602_d_n5, eq134_e1602_d_n6, eq134_e1602_d_n7, eq134_e1602_d_n8, eq134_e1602_d_n9, eq134_e1602_d_n10, eq134_e1602_d_n11, eq134_e1602_d_n12, eq134_e1602_d_n13, eq134_e1602_d_n14, eq134_e1602_d_n15, eq134_e1602_d_n16, eq134_e1602_d_n17, eq134_e1602_d_n18, eq134_e1602_d_n19, eq134_e1602_d_n20, eq134_e1602_d_n21, eq134_e1602_d_n22, eq134_e1602_d_n23, eq134_e1602_d_n24, eq134_e1602_d_n25, eq134_e1602_d_n26, eq134_e1602_d_n27, eq134_e1602_d_n28, eq134_e1602_d_n29];
        let eq134_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[13]),
            self.multiplicity * (eq134_value),
            &nodes,
            &eq134_node_derivatives,
            &branches,
            &eq134_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_135_block_0(
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
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq135_e1610, eq135_e1610_d_n0, eq135_e1610_d_n1, eq135_e1610_d_n2, eq135_e1610_d_n3, eq135_e1610_d_n4, eq135_e1610_d_n5, eq135_e1610_d_n6, eq135_e1610_d_n7, eq135_e1610_d_n8, eq135_e1610_d_n9, eq135_e1610_d_n10, eq135_e1610_d_n11, eq135_e1610_d_n12, eq135_e1610_d_n13, eq135_e1610_d_n14, eq135_e1610_d_n15, eq135_e1610_d_n16, eq135_e1610_d_n17, eq135_e1610_d_n18, eq135_e1610_d_n19, eq135_e1610_d_n20, eq135_e1610_d_n21, eq135_e1610_d_n22, eq135_e1610_d_n23, eq135_e1610_d_n24, eq135_e1610_d_n25, eq135_e1610_d_n26, eq135_e1610_d_n27, eq135_e1610_d_n28, eq135_e1610_d_n29,) = {
    if (s.v[1496] != 0.0) {
        let eq135_e1607: f64 = (s.v[0] * (nv13 - nv19));
        let eq135_e1607_d_n13: f64 = s.v[0];
        let eq135_e1607_d_n19: f64 = (-s.v[0]);
        let eq135_e1608: f64 = (s.v[154] + eq135_e1607);
        let eq135_e1608_d_n13: f64 = (s.dn[154][13] + eq135_e1607_d_n13);
        let eq135_e1608_d_n19: f64 = (s.dn[154][19] + eq135_e1607_d_n19);
        (eq135_e1608, s.dn[154][0], s.dn[154][1], s.dn[154][2], s.dn[154][3], s.dn[154][4], s.dn[154][5], s.dn[154][6], s.dn[154][7], s.dn[154][8], s.dn[154][9], s.dn[154][10], s.dn[154][11], s.dn[154][12], eq135_e1608_d_n13, s.dn[154][14], s.dn[154][15], s.dn[154][16], s.dn[154][17], s.dn[154][18], eq135_e1608_d_n19, s.dn[154][20], s.dn[154][21], s.dn[154][22], s.dn[154][23], s.dn[154][24], s.dn[154][25], s.dn[154][26], s.dn[154][27], s.dn[154][28], s.dn[154][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1610;
        let eq135_node_derivatives: [f64; 30] = [eq135_e1610_d_n0, eq135_e1610_d_n1, eq135_e1610_d_n2, eq135_e1610_d_n3, eq135_e1610_d_n4, eq135_e1610_d_n5, eq135_e1610_d_n6, eq135_e1610_d_n7, eq135_e1610_d_n8, eq135_e1610_d_n9, eq135_e1610_d_n10, eq135_e1610_d_n11, eq135_e1610_d_n12, eq135_e1610_d_n13, eq135_e1610_d_n14, eq135_e1610_d_n15, eq135_e1610_d_n16, eq135_e1610_d_n17, eq135_e1610_d_n18, eq135_e1610_d_n19, eq135_e1610_d_n20, eq135_e1610_d_n21, eq135_e1610_d_n22, eq135_e1610_d_n23, eq135_e1610_d_n24, eq135_e1610_d_n25, eq135_e1610_d_n26, eq135_e1610_d_n27, eq135_e1610_d_n28, eq135_e1610_d_n29];
        let eq135_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[13]),
            Some(nodes[19]),
            self.multiplicity * (eq135_value),
            &nodes,
            &eq135_node_derivatives,
            &branches,
            &eq135_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_136_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq136_e1615,) = {
    if (!(s.v[1496] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq136_value: f64 = eq136_e1615;
        stamper.stamp_potential(
            branches[26],
            eq136_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_137_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq137_e1623, eq137_e1623_d_n0, eq137_e1623_d_n1, eq137_e1623_d_n2, eq137_e1623_d_n3, eq137_e1623_d_n4, eq137_e1623_d_n5, eq137_e1623_d_n6, eq137_e1623_d_n7, eq137_e1623_d_n8, eq137_e1623_d_n9, eq137_e1623_d_n10, eq137_e1623_d_n11, eq137_e1623_d_n12, eq137_e1623_d_n13, eq137_e1623_d_n14, eq137_e1623_d_n15, eq137_e1623_d_n16, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n19, eq137_e1623_d_n20, eq137_e1623_d_n21, eq137_e1623_d_n22, eq137_e1623_d_n23, eq137_e1623_d_n24, eq137_e1623_d_n25, eq137_e1623_d_n26, eq137_e1623_d_n27, eq137_e1623_d_n28, eq137_e1623_d_n29,) = {
    if (s.v[1642] != 0.0) {
        let eq137_e1620: f64 = (s.v[0] * (nv18 - nv17));
        let eq137_e1620_d_n17: f64 = (-s.v[0]);
        let eq137_e1620_d_n18: f64 = s.v[0];
        let eq137_e1621: f64 = (s.v[160] + eq137_e1620);
        let eq137_e1621_d_n17: f64 = (s.dn[160][17] + eq137_e1620_d_n17);
        let eq137_e1621_d_n18: f64 = (s.dn[160][18] + eq137_e1620_d_n18);
        (eq137_e1621, s.dn[160][0], s.dn[160][1], s.dn[160][2], s.dn[160][3], s.dn[160][4], s.dn[160][5], s.dn[160][6], s.dn[160][7], s.dn[160][8], s.dn[160][9], s.dn[160][10], s.dn[160][11], s.dn[160][12], s.dn[160][13], s.dn[160][14], s.dn[160][15], s.dn[160][16], eq137_e1621_d_n17, eq137_e1621_d_n18, s.dn[160][19], s.dn[160][20], s.dn[160][21], s.dn[160][22], s.dn[160][23], s.dn[160][24], s.dn[160][25], s.dn[160][26], s.dn[160][27], s.dn[160][28], s.dn[160][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1623;
        let eq137_node_derivatives: [f64; 30] = [eq137_e1623_d_n0, eq137_e1623_d_n1, eq137_e1623_d_n2, eq137_e1623_d_n3, eq137_e1623_d_n4, eq137_e1623_d_n5, eq137_e1623_d_n6, eq137_e1623_d_n7, eq137_e1623_d_n8, eq137_e1623_d_n9, eq137_e1623_d_n10, eq137_e1623_d_n11, eq137_e1623_d_n12, eq137_e1623_d_n13, eq137_e1623_d_n14, eq137_e1623_d_n15, eq137_e1623_d_n16, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n19, eq137_e1623_d_n20, eq137_e1623_d_n21, eq137_e1623_d_n22, eq137_e1623_d_n23, eq137_e1623_d_n24, eq137_e1623_d_n25, eq137_e1623_d_n26, eq137_e1623_d_n27, eq137_e1623_d_n28, eq137_e1623_d_n29];
        let eq137_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[18]),
            Some(nodes[17]),
            self.multiplicity * (eq137_value),
            &nodes,
            &eq137_node_derivatives,
            &branches,
            &eq137_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_138_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq138_e1628,) = {
    if (!(s.v[1642] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq138_value: f64 = eq138_e1628;
        stamper.stamp_potential(
            branches[27],
            eq138_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_139_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq139_e1632,) = {
    if (s.v[1933] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq139_value: f64 = eq139_e1632;
        stamper.stamp_potential(
            branches[28],
            eq139_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_140_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq140_e1636,) = {
    if (s.v[1933] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq140_value: f64 = eq140_e1636;
        stamper.stamp_potential(
            branches[29],
            eq140_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_141_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq141_e1644, eq141_e1644_d_n0, eq141_e1644_d_n1, eq141_e1644_d_n2, eq141_e1644_d_n3, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n6, eq141_e1644_d_n7, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n10, eq141_e1644_d_n11, eq141_e1644_d_n12, eq141_e1644_d_n13, eq141_e1644_d_n14, eq141_e1644_d_n15, eq141_e1644_d_n16, eq141_e1644_d_n17, eq141_e1644_d_n18, eq141_e1644_d_n19, eq141_e1644_d_n20, eq141_e1644_d_n21, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n24, eq141_e1644_d_n25, eq141_e1644_d_n26, eq141_e1644_d_n27, eq141_e1644_d_n28, eq141_e1644_d_n29,) = {
    if (s.v[1933] != 0.0) {
        let eq141_e1641: f64 = (s.v[0] * (nv5 - nv9));
        let eq141_e1641_d_n5: f64 = s.v[0];
        let eq141_e1641_d_n9: f64 = (-s.v[0]);
        let eq141_e1642: f64 = (s.v[115] + eq141_e1641);
        let eq141_e1642_d_n5: f64 = (s.dn[115][5] + eq141_e1641_d_n5);
        let eq141_e1642_d_n9: f64 = (s.dn[115][9] + eq141_e1641_d_n9);
        (eq141_e1642, s.dn[115][0], s.dn[115][1], s.dn[115][2], s.dn[115][3], s.dn[115][4], eq141_e1642_d_n5, s.dn[115][6], s.dn[115][7], s.dn[115][8], eq141_e1642_d_n9, s.dn[115][10], s.dn[115][11], s.dn[115][12], s.dn[115][13], s.dn[115][14], s.dn[115][15], s.dn[115][16], s.dn[115][17], s.dn[115][18], s.dn[115][19], s.dn[115][20], s.dn[115][21], s.dn[115][22], s.dn[115][23], s.dn[115][24], s.dn[115][25], s.dn[115][26], s.dn[115][27], s.dn[115][28], s.dn[115][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1644;
        let eq141_node_derivatives: [f64; 30] = [eq141_e1644_d_n0, eq141_e1644_d_n1, eq141_e1644_d_n2, eq141_e1644_d_n3, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n6, eq141_e1644_d_n7, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n10, eq141_e1644_d_n11, eq141_e1644_d_n12, eq141_e1644_d_n13, eq141_e1644_d_n14, eq141_e1644_d_n15, eq141_e1644_d_n16, eq141_e1644_d_n17, eq141_e1644_d_n18, eq141_e1644_d_n19, eq141_e1644_d_n20, eq141_e1644_d_n21, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n24, eq141_e1644_d_n25, eq141_e1644_d_n26, eq141_e1644_d_n27, eq141_e1644_d_n28, eq141_e1644_d_n29];
        let eq141_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[9]),
            self.multiplicity * (eq141_value),
            &nodes,
            &eq141_node_derivatives,
            &branches,
            &eq141_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_142_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq142_e1656, eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29,) = {
    if (!(s.v[1933] != 0.0)) {
        let eq142_e1649: f64 = (s.v[115] - (nv29 - 0.0));
        let eq142_e1649_d_n29: f64 = (s.dn[115][29] - 1.0);
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1652_d_n28: f64 = p.p323;
        let eq142_e1653: f64 = self.eval_ddt(132, eq142_e1652);
        let eq142_e1653_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq142_e1653_d_n28: f64 = self.ddt_jacobian(eq142_e1652_d_n28);
        let eq142_e1653_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1653);
        let eq142_e1654_d_n0: f64 = (s.dn[115][0] - eq142_e1653_d_n0);
        let eq142_e1654_d_n1: f64 = (s.dn[115][1] - eq142_e1653_d_n1);
        let eq142_e1654_d_n2: f64 = (s.dn[115][2] - eq142_e1653_d_n2);
        let eq142_e1654_d_n3: f64 = (s.dn[115][3] - eq142_e1653_d_n3);
        let eq142_e1654_d_n4: f64 = (s.dn[115][4] - eq142_e1653_d_n4);
        let eq142_e1654_d_n5: f64 = (s.dn[115][5] - eq142_e1653_d_n5);
        let eq142_e1654_d_n6: f64 = (s.dn[115][6] - eq142_e1653_d_n6);
        let eq142_e1654_d_n7: f64 = (s.dn[115][7] - eq142_e1653_d_n7);
        let eq142_e1654_d_n8: f64 = (s.dn[115][8] - eq142_e1653_d_n8);
        let eq142_e1654_d_n9: f64 = (s.dn[115][9] - eq142_e1653_d_n9);
        let eq142_e1654_d_n10: f64 = (s.dn[115][10] - eq142_e1653_d_n10);
        let eq142_e1654_d_n11: f64 = (s.dn[115][11] - eq142_e1653_d_n11);
        let eq142_e1654_d_n12: f64 = (s.dn[115][12] - eq142_e1653_d_n12);
        let eq142_e1654_d_n13: f64 = (s.dn[115][13] - eq142_e1653_d_n13);
        let eq142_e1654_d_n14: f64 = (s.dn[115][14] - eq142_e1653_d_n14);
        let eq142_e1654_d_n15: f64 = (s.dn[115][15] - eq142_e1653_d_n15);
        let eq142_e1654_d_n16: f64 = (s.dn[115][16] - eq142_e1653_d_n16);
        let eq142_e1654_d_n17: f64 = (s.dn[115][17] - eq142_e1653_d_n17);
        let eq142_e1654_d_n18: f64 = (s.dn[115][18] - eq142_e1653_d_n18);
        let eq142_e1654_d_n19: f64 = (s.dn[115][19] - eq142_e1653_d_n19);
        let eq142_e1654_d_n20: f64 = (s.dn[115][20] - eq142_e1653_d_n20);
        let eq142_e1654_d_n21: f64 = (s.dn[115][21] - eq142_e1653_d_n21);
        let eq142_e1654_d_n22: f64 = (s.dn[115][22] - eq142_e1653_d_n22);
        let eq142_e1654_d_n23: f64 = (s.dn[115][23] - eq142_e1653_d_n23);
        let eq142_e1654_d_n24: f64 = (s.dn[115][24] - eq142_e1653_d_n24);
        let eq142_e1654_d_n25: f64 = (s.dn[115][25] - eq142_e1653_d_n25);
        let eq142_e1654_d_n26: f64 = (s.dn[115][26] - eq142_e1653_d_n26);
        let eq142_e1654_d_n27: f64 = (s.dn[115][27] - eq142_e1653_d_n27);
        let eq142_e1654_d_n28: f64 = (s.dn[115][28] - eq142_e1653_d_n28);
        let eq142_e1654_d_n29: f64 = (eq142_e1649_d_n29 - eq142_e1653_d_n29);
        (eq142_e1654, eq142_e1654_d_n0, eq142_e1654_d_n1, eq142_e1654_d_n2, eq142_e1654_d_n3, eq142_e1654_d_n4, eq142_e1654_d_n5, eq142_e1654_d_n6, eq142_e1654_d_n7, eq142_e1654_d_n8, eq142_e1654_d_n9, eq142_e1654_d_n10, eq142_e1654_d_n11, eq142_e1654_d_n12, eq142_e1654_d_n13, eq142_e1654_d_n14, eq142_e1654_d_n15, eq142_e1654_d_n16, eq142_e1654_d_n17, eq142_e1654_d_n18, eq142_e1654_d_n19, eq142_e1654_d_n20, eq142_e1654_d_n21, eq142_e1654_d_n22, eq142_e1654_d_n23, eq142_e1654_d_n24, eq142_e1654_d_n25, eq142_e1654_d_n26, eq142_e1654_d_n27, eq142_e1654_d_n28, eq142_e1654_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1656;
        let eq142_node_derivatives: [f64; 30] = [eq142_e1656_d_n0, eq142_e1656_d_n1, eq142_e1656_d_n2, eq142_e1656_d_n3, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n6, eq142_e1656_d_n7, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n10, eq142_e1656_d_n11, eq142_e1656_d_n12, eq142_e1656_d_n13, eq142_e1656_d_n14, eq142_e1656_d_n15, eq142_e1656_d_n16, eq142_e1656_d_n17, eq142_e1656_d_n18, eq142_e1656_d_n19, eq142_e1656_d_n20, eq142_e1656_d_n21, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n24, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n27, eq142_e1656_d_n28, eq142_e1656_d_n29];
        let eq142_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[28]),
            None,
            self.multiplicity * (eq142_value),
            &nodes,
            &eq142_node_derivatives,
            &branches,
            &eq142_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_143_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq143_e1670, eq143_e1670_d_n0, eq143_e1670_d_n1, eq143_e1670_d_n2, eq143_e1670_d_n3, eq143_e1670_d_n4, eq143_e1670_d_n5, eq143_e1670_d_n6, eq143_e1670_d_n7, eq143_e1670_d_n8, eq143_e1670_d_n9, eq143_e1670_d_n10, eq143_e1670_d_n11, eq143_e1670_d_n12, eq143_e1670_d_n13, eq143_e1670_d_n14, eq143_e1670_d_n15, eq143_e1670_d_n16, eq143_e1670_d_n17, eq143_e1670_d_n18, eq143_e1670_d_n19, eq143_e1670_d_n20, eq143_e1670_d_n21, eq143_e1670_d_n22, eq143_e1670_d_n23, eq143_e1670_d_n24, eq143_e1670_d_n25, eq143_e1670_d_n26, eq143_e1670_d_n27, eq143_e1670_d_n28, eq143_e1670_d_n29,) = {
    if (!(s.v[1933] != 0.0)) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1661_d_n29: f64 = (-1.0);
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667: f64 = self.eval_ddt(133, eq143_e1666);
        let eq143_e1667_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq143_e1667_d_n29: f64 = self.ddt_jacobian(eq143_e1664);
        let eq143_e1668: f64 = (eq143_e1661 - eq143_e1667);
        let eq143_e1668_d_n0: f64 = (-eq143_e1667_d_n0);
        let eq143_e1668_d_n1: f64 = (-eq143_e1667_d_n1);
        let eq143_e1668_d_n2: f64 = (-eq143_e1667_d_n2);
        let eq143_e1668_d_n3: f64 = (-eq143_e1667_d_n3);
        let eq143_e1668_d_n4: f64 = (-eq143_e1667_d_n4);
        let eq143_e1668_d_n5: f64 = (-eq143_e1667_d_n5);
        let eq143_e1668_d_n6: f64 = (-eq143_e1667_d_n6);
        let eq143_e1668_d_n7: f64 = (-eq143_e1667_d_n7);
        let eq143_e1668_d_n8: f64 = (-eq143_e1667_d_n8);
        let eq143_e1668_d_n9: f64 = (-eq143_e1667_d_n9);
        let eq143_e1668_d_n10: f64 = (-eq143_e1667_d_n10);
        let eq143_e1668_d_n11: f64 = (-eq143_e1667_d_n11);
        let eq143_e1668_d_n12: f64 = (-eq143_e1667_d_n12);
        let eq143_e1668_d_n13: f64 = (-eq143_e1667_d_n13);
        let eq143_e1668_d_n14: f64 = (-eq143_e1667_d_n14);
        let eq143_e1668_d_n15: f64 = (-eq143_e1667_d_n15);
        let eq143_e1668_d_n16: f64 = (-eq143_e1667_d_n16);
        let eq143_e1668_d_n17: f64 = (-eq143_e1667_d_n17);
        let eq143_e1668_d_n18: f64 = (-eq143_e1667_d_n18);
        let eq143_e1668_d_n19: f64 = (-eq143_e1667_d_n19);
        let eq143_e1668_d_n20: f64 = (-eq143_e1667_d_n20);
        let eq143_e1668_d_n21: f64 = (-eq143_e1667_d_n21);
        let eq143_e1668_d_n22: f64 = (-eq143_e1667_d_n22);
        let eq143_e1668_d_n23: f64 = (-eq143_e1667_d_n23);
        let eq143_e1668_d_n24: f64 = (-eq143_e1667_d_n24);
        let eq143_e1668_d_n25: f64 = (-eq143_e1667_d_n25);
        let eq143_e1668_d_n26: f64 = (-eq143_e1667_d_n26);
        let eq143_e1668_d_n27: f64 = (-eq143_e1667_d_n27);
        let eq143_e1668_d_n28: f64 = (1.0 - eq143_e1667_d_n28);
        let eq143_e1668_d_n29: f64 = (eq143_e1661_d_n29 - eq143_e1667_d_n29);
        (eq143_e1668, eq143_e1668_d_n0, eq143_e1668_d_n1, eq143_e1668_d_n2, eq143_e1668_d_n3, eq143_e1668_d_n4, eq143_e1668_d_n5, eq143_e1668_d_n6, eq143_e1668_d_n7, eq143_e1668_d_n8, eq143_e1668_d_n9, eq143_e1668_d_n10, eq143_e1668_d_n11, eq143_e1668_d_n12, eq143_e1668_d_n13, eq143_e1668_d_n14, eq143_e1668_d_n15, eq143_e1668_d_n16, eq143_e1668_d_n17, eq143_e1668_d_n18, eq143_e1668_d_n19, eq143_e1668_d_n20, eq143_e1668_d_n21, eq143_e1668_d_n22, eq143_e1668_d_n23, eq143_e1668_d_n24, eq143_e1668_d_n25, eq143_e1668_d_n26, eq143_e1668_d_n27, eq143_e1668_d_n28, eq143_e1668_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1670;
        let eq143_node_derivatives: [f64; 30] = [eq143_e1670_d_n0, eq143_e1670_d_n1, eq143_e1670_d_n2, eq143_e1670_d_n3, eq143_e1670_d_n4, eq143_e1670_d_n5, eq143_e1670_d_n6, eq143_e1670_d_n7, eq143_e1670_d_n8, eq143_e1670_d_n9, eq143_e1670_d_n10, eq143_e1670_d_n11, eq143_e1670_d_n12, eq143_e1670_d_n13, eq143_e1670_d_n14, eq143_e1670_d_n15, eq143_e1670_d_n16, eq143_e1670_d_n17, eq143_e1670_d_n18, eq143_e1670_d_n19, eq143_e1670_d_n20, eq143_e1670_d_n21, eq143_e1670_d_n22, eq143_e1670_d_n23, eq143_e1670_d_n24, eq143_e1670_d_n25, eq143_e1670_d_n26, eq143_e1670_d_n27, eq143_e1670_d_n28, eq143_e1670_d_n29];
        let eq143_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[29]),
            None,
            self.multiplicity * (eq143_value),
            &nodes,
            &eq143_node_derivatives,
            &branches,
            &eq143_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_144_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq144_e1679, eq144_e1679_d_n0, eq144_e1679_d_n1, eq144_e1679_d_n2, eq144_e1679_d_n3, eq144_e1679_d_n4, eq144_e1679_d_n5, eq144_e1679_d_n6, eq144_e1679_d_n7, eq144_e1679_d_n8, eq144_e1679_d_n9, eq144_e1679_d_n10, eq144_e1679_d_n11, eq144_e1679_d_n12, eq144_e1679_d_n13, eq144_e1679_d_n14, eq144_e1679_d_n15, eq144_e1679_d_n16, eq144_e1679_d_n17, eq144_e1679_d_n18, eq144_e1679_d_n19, eq144_e1679_d_n20, eq144_e1679_d_n21, eq144_e1679_d_n22, eq144_e1679_d_n23, eq144_e1679_d_n24, eq144_e1679_d_n25, eq144_e1679_d_n26, eq144_e1679_d_n27, eq144_e1679_d_n28, eq144_e1679_d_n29,) = {
    if (!(s.v[1933] != 0.0)) {
        let eq144_e1676: f64 = (s.v[0] * (nv5 - nv9));
        let eq144_e1676_d_n5: f64 = s.v[0];
        let eq144_e1676_d_n9: f64 = (-s.v[0]);
        let eq144_e1677: f64 = (s.v[116] + eq144_e1676);
        let eq144_e1677_d_n5: f64 = (s.dn[116][5] + eq144_e1676_d_n5);
        let eq144_e1677_d_n9: f64 = (s.dn[116][9] + eq144_e1676_d_n9);
        (eq144_e1677, s.dn[116][0], s.dn[116][1], s.dn[116][2], s.dn[116][3], s.dn[116][4], eq144_e1677_d_n5, s.dn[116][6], s.dn[116][7], s.dn[116][8], eq144_e1677_d_n9, s.dn[116][10], s.dn[116][11], s.dn[116][12], s.dn[116][13], s.dn[116][14], s.dn[116][15], s.dn[116][16], s.dn[116][17], s.dn[116][18], s.dn[116][19], s.dn[116][20], s.dn[116][21], s.dn[116][22], s.dn[116][23], s.dn[116][24], s.dn[116][25], s.dn[116][26], s.dn[116][27], s.dn[116][28], s.dn[116][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1679;
        let eq144_node_derivatives: [f64; 30] = [eq144_e1679_d_n0, eq144_e1679_d_n1, eq144_e1679_d_n2, eq144_e1679_d_n3, eq144_e1679_d_n4, eq144_e1679_d_n5, eq144_e1679_d_n6, eq144_e1679_d_n7, eq144_e1679_d_n8, eq144_e1679_d_n9, eq144_e1679_d_n10, eq144_e1679_d_n11, eq144_e1679_d_n12, eq144_e1679_d_n13, eq144_e1679_d_n14, eq144_e1679_d_n15, eq144_e1679_d_n16, eq144_e1679_d_n17, eq144_e1679_d_n18, eq144_e1679_d_n19, eq144_e1679_d_n20, eq144_e1679_d_n21, eq144_e1679_d_n22, eq144_e1679_d_n23, eq144_e1679_d_n24, eq144_e1679_d_n25, eq144_e1679_d_n26, eq144_e1679_d_n27, eq144_e1679_d_n28, eq144_e1679_d_n29];
        let eq144_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[9]),
            self.multiplicity * (eq144_value),
            &nodes,
            &eq144_node_derivatives,
            &branches,
            &eq144_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_145_block_0(
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
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq145_e1681: f64 = self.eval_ddt(134, s.v[117]);
        let eq145_e1681_d_n0: f64 = self.ddt_jacobian(s.dn[117][0]);
        let eq145_e1681_d_n1: f64 = self.ddt_jacobian(s.dn[117][1]);
        let eq145_e1681_d_n2: f64 = self.ddt_jacobian(s.dn[117][2]);
        let eq145_e1681_d_n3: f64 = self.ddt_jacobian(s.dn[117][3]);
        let eq145_e1681_d_n4: f64 = self.ddt_jacobian(s.dn[117][4]);
        let eq145_e1681_d_n5: f64 = self.ddt_jacobian(s.dn[117][5]);
        let eq145_e1681_d_n6: f64 = self.ddt_jacobian(s.dn[117][6]);
        let eq145_e1681_d_n7: f64 = self.ddt_jacobian(s.dn[117][7]);
        let eq145_e1681_d_n8: f64 = self.ddt_jacobian(s.dn[117][8]);
        let eq145_e1681_d_n9: f64 = self.ddt_jacobian(s.dn[117][9]);
        let eq145_e1681_d_n10: f64 = self.ddt_jacobian(s.dn[117][10]);
        let eq145_e1681_d_n11: f64 = self.ddt_jacobian(s.dn[117][11]);
        let eq145_e1681_d_n12: f64 = self.ddt_jacobian(s.dn[117][12]);
        let eq145_e1681_d_n13: f64 = self.ddt_jacobian(s.dn[117][13]);
        let eq145_e1681_d_n14: f64 = self.ddt_jacobian(s.dn[117][14]);
        let eq145_e1681_d_n15: f64 = self.ddt_jacobian(s.dn[117][15]);
        let eq145_e1681_d_n16: f64 = self.ddt_jacobian(s.dn[117][16]);
        let eq145_e1681_d_n17: f64 = self.ddt_jacobian(s.dn[117][17]);
        let eq145_e1681_d_n18: f64 = self.ddt_jacobian(s.dn[117][18]);
        let eq145_e1681_d_n19: f64 = self.ddt_jacobian(s.dn[117][19]);
        let eq145_e1681_d_n20: f64 = self.ddt_jacobian(s.dn[117][20]);
        let eq145_e1681_d_n21: f64 = self.ddt_jacobian(s.dn[117][21]);
        let eq145_e1681_d_n22: f64 = self.ddt_jacobian(s.dn[117][22]);
        let eq145_e1681_d_n23: f64 = self.ddt_jacobian(s.dn[117][23]);
        let eq145_e1681_d_n24: f64 = self.ddt_jacobian(s.dn[117][24]);
        let eq145_e1681_d_n25: f64 = self.ddt_jacobian(s.dn[117][25]);
        let eq145_e1681_d_n26: f64 = self.ddt_jacobian(s.dn[117][26]);
        let eq145_e1681_d_n27: f64 = self.ddt_jacobian(s.dn[117][27]);
        let eq145_e1681_d_n28: f64 = self.ddt_jacobian(s.dn[117][28]);
        let eq145_e1681_d_n29: f64 = self.ddt_jacobian(s.dn[117][29]);
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1684_d_n8: f64 = p.p355;
        let eq145_e1684_d_n9: f64 = (-p.p355);
        let eq145_e1685: f64 = self.eval_ddt(135, eq145_e1684);
        let eq145_e1685_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n8: f64 = self.ddt_jacobian(eq145_e1684_d_n8);
        let eq145_e1685_d_n9: f64 = self.ddt_jacobian(eq145_e1684_d_n9);
        let eq145_e1685_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq145_e1685_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq145_e1686: f64 = (eq145_e1681 + eq145_e1685);
        let eq145_e1686_d_n0: f64 = (eq145_e1681_d_n0 + eq145_e1685_d_n0);
        let eq145_e1686_d_n1: f64 = (eq145_e1681_d_n1 + eq145_e1685_d_n1);
        let eq145_e1686_d_n2: f64 = (eq145_e1681_d_n2 + eq145_e1685_d_n2);
        let eq145_e1686_d_n3: f64 = (eq145_e1681_d_n3 + eq145_e1685_d_n3);
        let eq145_e1686_d_n4: f64 = (eq145_e1681_d_n4 + eq145_e1685_d_n4);
        let eq145_e1686_d_n5: f64 = (eq145_e1681_d_n5 + eq145_e1685_d_n5);
        let eq145_e1686_d_n6: f64 = (eq145_e1681_d_n6 + eq145_e1685_d_n6);
        let eq145_e1686_d_n7: f64 = (eq145_e1681_d_n7 + eq145_e1685_d_n7);
        let eq145_e1686_d_n8: f64 = (eq145_e1681_d_n8 + eq145_e1685_d_n8);
        let eq145_e1686_d_n9: f64 = (eq145_e1681_d_n9 + eq145_e1685_d_n9);
        let eq145_e1686_d_n10: f64 = (eq145_e1681_d_n10 + eq145_e1685_d_n10);
        let eq145_e1686_d_n11: f64 = (eq145_e1681_d_n11 + eq145_e1685_d_n11);
        let eq145_e1686_d_n12: f64 = (eq145_e1681_d_n12 + eq145_e1685_d_n12);
        let eq145_e1686_d_n13: f64 = (eq145_e1681_d_n13 + eq145_e1685_d_n13);
        let eq145_e1686_d_n14: f64 = (eq145_e1681_d_n14 + eq145_e1685_d_n14);
        let eq145_e1686_d_n15: f64 = (eq145_e1681_d_n15 + eq145_e1685_d_n15);
        let eq145_e1686_d_n16: f64 = (eq145_e1681_d_n16 + eq145_e1685_d_n16);
        let eq145_e1686_d_n17: f64 = (eq145_e1681_d_n17 + eq145_e1685_d_n17);
        let eq145_e1686_d_n18: f64 = (eq145_e1681_d_n18 + eq145_e1685_d_n18);
        let eq145_e1686_d_n19: f64 = (eq145_e1681_d_n19 + eq145_e1685_d_n19);
        let eq145_e1686_d_n20: f64 = (eq145_e1681_d_n20 + eq145_e1685_d_n20);
        let eq145_e1686_d_n21: f64 = (eq145_e1681_d_n21 + eq145_e1685_d_n21);
        let eq145_e1686_d_n22: f64 = (eq145_e1681_d_n22 + eq145_e1685_d_n22);
        let eq145_e1686_d_n23: f64 = (eq145_e1681_d_n23 + eq145_e1685_d_n23);
        let eq145_e1686_d_n24: f64 = (eq145_e1681_d_n24 + eq145_e1685_d_n24);
        let eq145_e1686_d_n25: f64 = (eq145_e1681_d_n25 + eq145_e1685_d_n25);
        let eq145_e1686_d_n26: f64 = (eq145_e1681_d_n26 + eq145_e1685_d_n26);
        let eq145_e1686_d_n27: f64 = (eq145_e1681_d_n27 + eq145_e1685_d_n27);
        let eq145_e1686_d_n28: f64 = (eq145_e1681_d_n28 + eq145_e1685_d_n28);
        let eq145_e1686_d_n29: f64 = (eq145_e1681_d_n29 + eq145_e1685_d_n29);
        let eq145_value: f64 = eq145_e1686;
        let eq145_node_derivatives: [f64; 30] = [eq145_e1686_d_n0, eq145_e1686_d_n1, eq145_e1686_d_n2, eq145_e1686_d_n3, eq145_e1686_d_n4, eq145_e1686_d_n5, eq145_e1686_d_n6, eq145_e1686_d_n7, eq145_e1686_d_n8, eq145_e1686_d_n9, eq145_e1686_d_n10, eq145_e1686_d_n11, eq145_e1686_d_n12, eq145_e1686_d_n13, eq145_e1686_d_n14, eq145_e1686_d_n15, eq145_e1686_d_n16, eq145_e1686_d_n17, eq145_e1686_d_n18, eq145_e1686_d_n19, eq145_e1686_d_n20, eq145_e1686_d_n21, eq145_e1686_d_n22, eq145_e1686_d_n23, eq145_e1686_d_n24, eq145_e1686_d_n25, eq145_e1686_d_n26, eq145_e1686_d_n27, eq145_e1686_d_n28, eq145_e1686_d_n29];
        let eq145_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            self.multiplicity * (eq145_value),
            &nodes,
            &eq145_node_derivatives,
            &branches,
            &eq145_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_146_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let eq146_e1688: f64 = self.eval_ddt(136, s.v[118]);
        let eq146_e1688_d_n0: f64 = self.ddt_jacobian(s.dn[118][0]);
        let eq146_e1688_d_n1: f64 = self.ddt_jacobian(s.dn[118][1]);
        let eq146_e1688_d_n2: f64 = self.ddt_jacobian(s.dn[118][2]);
        let eq146_e1688_d_n3: f64 = self.ddt_jacobian(s.dn[118][3]);
        let eq146_e1688_d_n4: f64 = self.ddt_jacobian(s.dn[118][4]);
        let eq146_e1688_d_n5: f64 = self.ddt_jacobian(s.dn[118][5]);
        let eq146_e1688_d_n6: f64 = self.ddt_jacobian(s.dn[118][6]);
        let eq146_e1688_d_n7: f64 = self.ddt_jacobian(s.dn[118][7]);
        let eq146_e1688_d_n8: f64 = self.ddt_jacobian(s.dn[118][8]);
        let eq146_e1688_d_n9: f64 = self.ddt_jacobian(s.dn[118][9]);
        let eq146_e1688_d_n10: f64 = self.ddt_jacobian(s.dn[118][10]);
        let eq146_e1688_d_n11: f64 = self.ddt_jacobian(s.dn[118][11]);
        let eq146_e1688_d_n12: f64 = self.ddt_jacobian(s.dn[118][12]);
        let eq146_e1688_d_n13: f64 = self.ddt_jacobian(s.dn[118][13]);
        let eq146_e1688_d_n14: f64 = self.ddt_jacobian(s.dn[118][14]);
        let eq146_e1688_d_n15: f64 = self.ddt_jacobian(s.dn[118][15]);
        let eq146_e1688_d_n16: f64 = self.ddt_jacobian(s.dn[118][16]);
        let eq146_e1688_d_n17: f64 = self.ddt_jacobian(s.dn[118][17]);
        let eq146_e1688_d_n18: f64 = self.ddt_jacobian(s.dn[118][18]);
        let eq146_e1688_d_n19: f64 = self.ddt_jacobian(s.dn[118][19]);
        let eq146_e1688_d_n20: f64 = self.ddt_jacobian(s.dn[118][20]);
        let eq146_e1688_d_n21: f64 = self.ddt_jacobian(s.dn[118][21]);
        let eq146_e1688_d_n22: f64 = self.ddt_jacobian(s.dn[118][22]);
        let eq146_e1688_d_n23: f64 = self.ddt_jacobian(s.dn[118][23]);
        let eq146_e1688_d_n24: f64 = self.ddt_jacobian(s.dn[118][24]);
        let eq146_e1688_d_n25: f64 = self.ddt_jacobian(s.dn[118][25]);
        let eq146_e1688_d_n26: f64 = self.ddt_jacobian(s.dn[118][26]);
        let eq146_e1688_d_n27: f64 = self.ddt_jacobian(s.dn[118][27]);
        let eq146_e1688_d_n28: f64 = self.ddt_jacobian(s.dn[118][28]);
        let eq146_e1688_d_n29: f64 = self.ddt_jacobian(s.dn[118][29]);
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1691_d_n5: f64 = (-p.p355);
        let eq146_e1691_d_n8: f64 = p.p355;
        let eq146_e1692: f64 = self.eval_ddt(137, eq146_e1691);
        let eq146_e1692_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n5: f64 = self.ddt_jacobian(eq146_e1691_d_n5);
        let eq146_e1692_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n8: f64 = self.ddt_jacobian(eq146_e1691_d_n8);
        let eq146_e1692_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq146_e1692_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq146_e1693: f64 = (eq146_e1688 + eq146_e1692);
        let eq146_e1693_d_n0: f64 = (eq146_e1688_d_n0 + eq146_e1692_d_n0);
        let eq146_e1693_d_n1: f64 = (eq146_e1688_d_n1 + eq146_e1692_d_n1);
        let eq146_e1693_d_n2: f64 = (eq146_e1688_d_n2 + eq146_e1692_d_n2);
        let eq146_e1693_d_n3: f64 = (eq146_e1688_d_n3 + eq146_e1692_d_n3);
        let eq146_e1693_d_n4: f64 = (eq146_e1688_d_n4 + eq146_e1692_d_n4);
        let eq146_e1693_d_n5: f64 = (eq146_e1688_d_n5 + eq146_e1692_d_n5);
        let eq146_e1693_d_n6: f64 = (eq146_e1688_d_n6 + eq146_e1692_d_n6);
        let eq146_e1693_d_n7: f64 = (eq146_e1688_d_n7 + eq146_e1692_d_n7);
        let eq146_e1693_d_n8: f64 = (eq146_e1688_d_n8 + eq146_e1692_d_n8);
        let eq146_e1693_d_n9: f64 = (eq146_e1688_d_n9 + eq146_e1692_d_n9);
        let eq146_e1693_d_n10: f64 = (eq146_e1688_d_n10 + eq146_e1692_d_n10);
        let eq146_e1693_d_n11: f64 = (eq146_e1688_d_n11 + eq146_e1692_d_n11);
        let eq146_e1693_d_n12: f64 = (eq146_e1688_d_n12 + eq146_e1692_d_n12);
        let eq146_e1693_d_n13: f64 = (eq146_e1688_d_n13 + eq146_e1692_d_n13);
        let eq146_e1693_d_n14: f64 = (eq146_e1688_d_n14 + eq146_e1692_d_n14);
        let eq146_e1693_d_n15: f64 = (eq146_e1688_d_n15 + eq146_e1692_d_n15);
        let eq146_e1693_d_n16: f64 = (eq146_e1688_d_n16 + eq146_e1692_d_n16);
        let eq146_e1693_d_n17: f64 = (eq146_e1688_d_n17 + eq146_e1692_d_n17);
        let eq146_e1693_d_n18: f64 = (eq146_e1688_d_n18 + eq146_e1692_d_n18);
        let eq146_e1693_d_n19: f64 = (eq146_e1688_d_n19 + eq146_e1692_d_n19);
        let eq146_e1693_d_n20: f64 = (eq146_e1688_d_n20 + eq146_e1692_d_n20);
        let eq146_e1693_d_n21: f64 = (eq146_e1688_d_n21 + eq146_e1692_d_n21);
        let eq146_e1693_d_n22: f64 = (eq146_e1688_d_n22 + eq146_e1692_d_n22);
        let eq146_e1693_d_n23: f64 = (eq146_e1688_d_n23 + eq146_e1692_d_n23);
        let eq146_e1693_d_n24: f64 = (eq146_e1688_d_n24 + eq146_e1692_d_n24);
        let eq146_e1693_d_n25: f64 = (eq146_e1688_d_n25 + eq146_e1692_d_n25);
        let eq146_e1693_d_n26: f64 = (eq146_e1688_d_n26 + eq146_e1692_d_n26);
        let eq146_e1693_d_n27: f64 = (eq146_e1688_d_n27 + eq146_e1692_d_n27);
        let eq146_e1693_d_n28: f64 = (eq146_e1688_d_n28 + eq146_e1692_d_n28);
        let eq146_e1693_d_n29: f64 = (eq146_e1688_d_n29 + eq146_e1692_d_n29);
        let eq146_value: f64 = eq146_e1693;
        let eq146_node_derivatives: [f64; 30] = [eq146_e1693_d_n0, eq146_e1693_d_n1, eq146_e1693_d_n2, eq146_e1693_d_n3, eq146_e1693_d_n4, eq146_e1693_d_n5, eq146_e1693_d_n6, eq146_e1693_d_n7, eq146_e1693_d_n8, eq146_e1693_d_n9, eq146_e1693_d_n10, eq146_e1693_d_n11, eq146_e1693_d_n12, eq146_e1693_d_n13, eq146_e1693_d_n14, eq146_e1693_d_n15, eq146_e1693_d_n16, eq146_e1693_d_n17, eq146_e1693_d_n18, eq146_e1693_d_n19, eq146_e1693_d_n20, eq146_e1693_d_n21, eq146_e1693_d_n22, eq146_e1693_d_n23, eq146_e1693_d_n24, eq146_e1693_d_n25, eq146_e1693_d_n26, eq146_e1693_d_n27, eq146_e1693_d_n28, eq146_e1693_d_n29];
        let eq146_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq146_value),
            &nodes,
            &eq146_node_derivatives,
            &branches,
            &eq146_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_147_block_0(
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq147_e1701, eq147_e1701_d_n0, eq147_e1701_d_n1, eq147_e1701_d_n2, eq147_e1701_d_n3, eq147_e1701_d_n4, eq147_e1701_d_n5, eq147_e1701_d_n6, eq147_e1701_d_n7, eq147_e1701_d_n8, eq147_e1701_d_n9, eq147_e1701_d_n10, eq147_e1701_d_n11, eq147_e1701_d_n12, eq147_e1701_d_n13, eq147_e1701_d_n14, eq147_e1701_d_n15, eq147_e1701_d_n16, eq147_e1701_d_n17, eq147_e1701_d_n18, eq147_e1701_d_n19, eq147_e1701_d_n20, eq147_e1701_d_n21, eq147_e1701_d_n22, eq147_e1701_d_n23, eq147_e1701_d_n24, eq147_e1701_d_n25, eq147_e1701_d_n26, eq147_e1701_d_n27, eq147_e1701_d_n28, eq147_e1701_d_n29,) = {
    if (s.v[1934] != 0.0) {
        let eq147_e1698: f64 = (s.v[0] * (nv8 - nv13));
        let eq147_e1698_d_n8: f64 = s.v[0];
        let eq147_e1698_d_n13: f64 = (-s.v[0]);
        let eq147_e1699: f64 = (s.v[122] + eq147_e1698);
        let eq147_e1699_d_n8: f64 = (s.dn[122][8] + eq147_e1698_d_n8);
        let eq147_e1699_d_n13: f64 = (s.dn[122][13] + eq147_e1698_d_n13);
        (eq147_e1699, s.dn[122][0], s.dn[122][1], s.dn[122][2], s.dn[122][3], s.dn[122][4], s.dn[122][5], s.dn[122][6], s.dn[122][7], eq147_e1699_d_n8, s.dn[122][9], s.dn[122][10], s.dn[122][11], s.dn[122][12], eq147_e1699_d_n13, s.dn[122][14], s.dn[122][15], s.dn[122][16], s.dn[122][17], s.dn[122][18], s.dn[122][19], s.dn[122][20], s.dn[122][21], s.dn[122][22], s.dn[122][23], s.dn[122][24], s.dn[122][25], s.dn[122][26], s.dn[122][27], s.dn[122][28], s.dn[122][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1701;
        let eq147_node_derivatives: [f64; 30] = [eq147_e1701_d_n0, eq147_e1701_d_n1, eq147_e1701_d_n2, eq147_e1701_d_n3, eq147_e1701_d_n4, eq147_e1701_d_n5, eq147_e1701_d_n6, eq147_e1701_d_n7, eq147_e1701_d_n8, eq147_e1701_d_n9, eq147_e1701_d_n10, eq147_e1701_d_n11, eq147_e1701_d_n12, eq147_e1701_d_n13, eq147_e1701_d_n14, eq147_e1701_d_n15, eq147_e1701_d_n16, eq147_e1701_d_n17, eq147_e1701_d_n18, eq147_e1701_d_n19, eq147_e1701_d_n20, eq147_e1701_d_n21, eq147_e1701_d_n22, eq147_e1701_d_n23, eq147_e1701_d_n24, eq147_e1701_d_n25, eq147_e1701_d_n26, eq147_e1701_d_n27, eq147_e1701_d_n28, eq147_e1701_d_n29];
        let eq147_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[13]),
            self.multiplicity * (eq147_value),
            &nodes,
            &eq147_node_derivatives,
            &branches,
            &eq147_branch_derivatives,
            self.multiplicity,
        );
    }
}

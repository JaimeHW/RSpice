#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let (eq135_e1722, eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
        let eq135_e1719: f64 = self.eval_ddt(34, s.v[240]);
        let eq135_e1719_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq135_e1719_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq135_e1719_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq135_e1719_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq135_e1719_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq135_e1719_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq135_e1719_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq135_e1719_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq135_e1719_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq135_e1719_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq135_e1719_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq135_e1719_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq135_e1719_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq135_e1719_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq135_e1719_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq135_e1719_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq135_e1719_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq135_e1719_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq135_e1719_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq135_e1719_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq135_e1719_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq135_e1719_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq135_e1719_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq135_e1720: f64 = (p.p7 * eq135_e1719);
        let eq135_e1720_d_n0: f64 = (p.p7 * eq135_e1719_d_n0);
        let eq135_e1720_d_n1: f64 = (p.p7 * eq135_e1719_d_n1);
        let eq135_e1720_d_n2: f64 = (p.p7 * eq135_e1719_d_n2);
        let eq135_e1720_d_n3: f64 = (p.p7 * eq135_e1719_d_n3);
        let eq135_e1720_d_n4: f64 = (p.p7 * eq135_e1719_d_n4);
        let eq135_e1720_d_n5: f64 = (p.p7 * eq135_e1719_d_n5);
        let eq135_e1720_d_n6: f64 = (p.p7 * eq135_e1719_d_n6);
        let eq135_e1720_d_n7: f64 = (p.p7 * eq135_e1719_d_n7);
        let eq135_e1720_d_n8: f64 = (p.p7 * eq135_e1719_d_n8);
        let eq135_e1720_d_n9: f64 = (p.p7 * eq135_e1719_d_n9);
        let eq135_e1720_d_n10: f64 = (p.p7 * eq135_e1719_d_n10);
        let eq135_e1720_d_n11: f64 = (p.p7 * eq135_e1719_d_n11);
        let eq135_e1720_d_n12: f64 = (p.p7 * eq135_e1719_d_n12);
        let eq135_e1720_d_n13: f64 = (p.p7 * eq135_e1719_d_n13);
        let eq135_e1720_d_n14: f64 = (p.p7 * eq135_e1719_d_n14);
        let eq135_e1720_d_n15: f64 = (p.p7 * eq135_e1719_d_n15);
        let eq135_e1720_d_n16: f64 = (p.p7 * eq135_e1719_d_n16);
        let eq135_e1720_d_n17: f64 = (p.p7 * eq135_e1719_d_n17);
        let eq135_e1720_d_n18: f64 = (p.p7 * eq135_e1719_d_n18);
        let eq135_e1720_d_n19: f64 = (p.p7 * eq135_e1719_d_n19);
        let eq135_e1720_d_n20: f64 = (p.p7 * eq135_e1719_d_n20);
        let eq135_e1720_d_n21: f64 = (p.p7 * eq135_e1719_d_n21);
        let eq135_e1720_d_n22: f64 = (p.p7 * eq135_e1719_d_n22);
        (eq135_e1720, eq135_e1720_d_n0, eq135_e1720_d_n1, eq135_e1720_d_n2, eq135_e1720_d_n3, eq135_e1720_d_n4, eq135_e1720_d_n5, eq135_e1720_d_n6, eq135_e1720_d_n7, eq135_e1720_d_n8, eq135_e1720_d_n9, eq135_e1720_d_n10, eq135_e1720_d_n11, eq135_e1720_d_n12, eq135_e1720_d_n13, eq135_e1720_d_n14, eq135_e1720_d_n15, eq135_e1720_d_n16, eq135_e1720_d_n17, eq135_e1720_d_n18, eq135_e1720_d_n19, eq135_e1720_d_n20, eq135_e1720_d_n21, eq135_e1720_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1722;
        let eq135_node_derivatives: [f64; 23] = [eq135_e1722_d_n0, eq135_e1722_d_n1, eq135_e1722_d_n2, eq135_e1722_d_n3, eq135_e1722_d_n4, eq135_e1722_d_n5, eq135_e1722_d_n6, eq135_e1722_d_n7, eq135_e1722_d_n8, eq135_e1722_d_n9, eq135_e1722_d_n10, eq135_e1722_d_n11, eq135_e1722_d_n12, eq135_e1722_d_n13, eq135_e1722_d_n14, eq135_e1722_d_n15, eq135_e1722_d_n16, eq135_e1722_d_n17, eq135_e1722_d_n18, eq135_e1722_d_n19, eq135_e1722_d_n20, eq135_e1722_d_n21, eq135_e1722_d_n22];
        let eq135_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
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
        let (eq136_e1736, eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (!(s.v[577] != 0.0))) {
        let eq136_e1731: f64 = self.eval_ddt(35, s.v[240]);
        let eq136_e1731_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq136_e1731_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq136_e1731_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq136_e1731_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq136_e1731_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq136_e1731_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq136_e1731_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq136_e1731_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq136_e1731_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq136_e1731_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq136_e1731_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq136_e1731_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq136_e1731_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq136_e1731_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq136_e1731_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq136_e1731_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq136_e1731_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq136_e1731_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq136_e1731_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq136_e1731_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq136_e1731_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq136_e1731_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq136_e1731_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq136_e1732: f64 = (p.p7 * eq136_e1731);
        let eq136_e1732_d_n0: f64 = (p.p7 * eq136_e1731_d_n0);
        let eq136_e1732_d_n1: f64 = (p.p7 * eq136_e1731_d_n1);
        let eq136_e1732_d_n2: f64 = (p.p7 * eq136_e1731_d_n2);
        let eq136_e1732_d_n3: f64 = (p.p7 * eq136_e1731_d_n3);
        let eq136_e1732_d_n4: f64 = (p.p7 * eq136_e1731_d_n4);
        let eq136_e1732_d_n5: f64 = (p.p7 * eq136_e1731_d_n5);
        let eq136_e1732_d_n6: f64 = (p.p7 * eq136_e1731_d_n6);
        let eq136_e1732_d_n7: f64 = (p.p7 * eq136_e1731_d_n7);
        let eq136_e1732_d_n8: f64 = (p.p7 * eq136_e1731_d_n8);
        let eq136_e1732_d_n9: f64 = (p.p7 * eq136_e1731_d_n9);
        let eq136_e1732_d_n10: f64 = (p.p7 * eq136_e1731_d_n10);
        let eq136_e1732_d_n11: f64 = (p.p7 * eq136_e1731_d_n11);
        let eq136_e1732_d_n12: f64 = (p.p7 * eq136_e1731_d_n12);
        let eq136_e1732_d_n13: f64 = (p.p7 * eq136_e1731_d_n13);
        let eq136_e1732_d_n14: f64 = (p.p7 * eq136_e1731_d_n14);
        let eq136_e1732_d_n15: f64 = (p.p7 * eq136_e1731_d_n15);
        let eq136_e1732_d_n16: f64 = (p.p7 * eq136_e1731_d_n16);
        let eq136_e1732_d_n17: f64 = (p.p7 * eq136_e1731_d_n17);
        let eq136_e1732_d_n18: f64 = (p.p7 * eq136_e1731_d_n18);
        let eq136_e1732_d_n19: f64 = (p.p7 * eq136_e1731_d_n19);
        let eq136_e1732_d_n20: f64 = (p.p7 * eq136_e1731_d_n20);
        let eq136_e1732_d_n21: f64 = (p.p7 * eq136_e1731_d_n21);
        let eq136_e1732_d_n22: f64 = (p.p7 * eq136_e1731_d_n22);
        let eq136_e1734: f64 = (eq136_e1732 * p.p246);
        let eq136_e1734_d_n0: f64 = (eq136_e1732_d_n0 * p.p246);
        let eq136_e1734_d_n1: f64 = (eq136_e1732_d_n1 * p.p246);
        let eq136_e1734_d_n2: f64 = (eq136_e1732_d_n2 * p.p246);
        let eq136_e1734_d_n3: f64 = (eq136_e1732_d_n3 * p.p246);
        let eq136_e1734_d_n4: f64 = (eq136_e1732_d_n4 * p.p246);
        let eq136_e1734_d_n5: f64 = (eq136_e1732_d_n5 * p.p246);
        let eq136_e1734_d_n6: f64 = (eq136_e1732_d_n6 * p.p246);
        let eq136_e1734_d_n7: f64 = (eq136_e1732_d_n7 * p.p246);
        let eq136_e1734_d_n8: f64 = (eq136_e1732_d_n8 * p.p246);
        let eq136_e1734_d_n9: f64 = (eq136_e1732_d_n9 * p.p246);
        let eq136_e1734_d_n10: f64 = (eq136_e1732_d_n10 * p.p246);
        let eq136_e1734_d_n11: f64 = (eq136_e1732_d_n11 * p.p246);
        let eq136_e1734_d_n12: f64 = (eq136_e1732_d_n12 * p.p246);
        let eq136_e1734_d_n13: f64 = (eq136_e1732_d_n13 * p.p246);
        let eq136_e1734_d_n14: f64 = (eq136_e1732_d_n14 * p.p246);
        let eq136_e1734_d_n15: f64 = (eq136_e1732_d_n15 * p.p246);
        let eq136_e1734_d_n16: f64 = (eq136_e1732_d_n16 * p.p246);
        let eq136_e1734_d_n17: f64 = (eq136_e1732_d_n17 * p.p246);
        let eq136_e1734_d_n18: f64 = (eq136_e1732_d_n18 * p.p246);
        let eq136_e1734_d_n19: f64 = (eq136_e1732_d_n19 * p.p246);
        let eq136_e1734_d_n20: f64 = (eq136_e1732_d_n20 * p.p246);
        let eq136_e1734_d_n21: f64 = (eq136_e1732_d_n21 * p.p246);
        let eq136_e1734_d_n22: f64 = (eq136_e1732_d_n22 * p.p246);
        (eq136_e1734, eq136_e1734_d_n0, eq136_e1734_d_n1, eq136_e1734_d_n2, eq136_e1734_d_n3, eq136_e1734_d_n4, eq136_e1734_d_n5, eq136_e1734_d_n6, eq136_e1734_d_n7, eq136_e1734_d_n8, eq136_e1734_d_n9, eq136_e1734_d_n10, eq136_e1734_d_n11, eq136_e1734_d_n12, eq136_e1734_d_n13, eq136_e1734_d_n14, eq136_e1734_d_n15, eq136_e1734_d_n16, eq136_e1734_d_n17, eq136_e1734_d_n18, eq136_e1734_d_n19, eq136_e1734_d_n20, eq136_e1734_d_n21, eq136_e1734_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq136_value: f64 = eq136_e1736;
        let eq136_node_derivatives: [f64; 23] = [eq136_e1736_d_n0, eq136_e1736_d_n1, eq136_e1736_d_n2, eq136_e1736_d_n3, eq136_e1736_d_n4, eq136_e1736_d_n5, eq136_e1736_d_n6, eq136_e1736_d_n7, eq136_e1736_d_n8, eq136_e1736_d_n9, eq136_e1736_d_n10, eq136_e1736_d_n11, eq136_e1736_d_n12, eq136_e1736_d_n13, eq136_e1736_d_n14, eq136_e1736_d_n15, eq136_e1736_d_n16, eq136_e1736_d_n17, eq136_e1736_d_n18, eq136_e1736_d_n19, eq136_e1736_d_n20, eq136_e1736_d_n21, eq136_e1736_d_n22];
        let eq136_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            self.multiplicity * (eq136_value),
            &nodes,
            &eq136_node_derivatives,
            &branches,
            &eq136_branch_derivatives,
            self.multiplicity,
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
        let (eq137_e1747, eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22,) = {
    if ((s.v[575] != 0.0) && (s.v[576] != 0.0)) {
        let eq137_e1743: f64 = (p.p251 * s.v[240]);
        let eq137_e1743_d_n0: f64 = (p.p251 * s.dn[240][0]);
        let eq137_e1743_d_n1: f64 = (p.p251 * s.dn[240][1]);
        let eq137_e1743_d_n2: f64 = (p.p251 * s.dn[240][2]);
        let eq137_e1743_d_n3: f64 = (p.p251 * s.dn[240][3]);
        let eq137_e1743_d_n4: f64 = (p.p251 * s.dn[240][4]);
        let eq137_e1743_d_n5: f64 = (p.p251 * s.dn[240][5]);
        let eq137_e1743_d_n6: f64 = (p.p251 * s.dn[240][6]);
        let eq137_e1743_d_n7: f64 = (p.p251 * s.dn[240][7]);
        let eq137_e1743_d_n8: f64 = (p.p251 * s.dn[240][8]);
        let eq137_e1743_d_n9: f64 = (p.p251 * s.dn[240][9]);
        let eq137_e1743_d_n10: f64 = (p.p251 * s.dn[240][10]);
        let eq137_e1743_d_n11: f64 = (p.p251 * s.dn[240][11]);
        let eq137_e1743_d_n12: f64 = (p.p251 * s.dn[240][12]);
        let eq137_e1743_d_n13: f64 = (p.p251 * s.dn[240][13]);
        let eq137_e1743_d_n14: f64 = (p.p251 * s.dn[240][14]);
        let eq137_e1743_d_n15: f64 = (p.p251 * s.dn[240][15]);
        let eq137_e1743_d_n16: f64 = (p.p251 * s.dn[240][16]);
        let eq137_e1743_d_n17: f64 = (p.p251 * s.dn[240][17]);
        let eq137_e1743_d_n18: f64 = (p.p251 * s.dn[240][18]);
        let eq137_e1743_d_n19: f64 = (p.p251 * s.dn[240][19]);
        let eq137_e1743_d_n20: f64 = (p.p251 * s.dn[240][20]);
        let eq137_e1743_d_n21: f64 = (p.p251 * s.dn[240][21]);
        let eq137_e1743_d_n22: f64 = (p.p251 * s.dn[240][22]);
        let eq137_e1744: f64 = self.eval_ddt(36, eq137_e1743);
        let eq137_e1744_d_n0: f64 = self.ddt_jacobian(eq137_e1743_d_n0);
        let eq137_e1744_d_n1: f64 = self.ddt_jacobian(eq137_e1743_d_n1);
        let eq137_e1744_d_n2: f64 = self.ddt_jacobian(eq137_e1743_d_n2);
        let eq137_e1744_d_n3: f64 = self.ddt_jacobian(eq137_e1743_d_n3);
        let eq137_e1744_d_n4: f64 = self.ddt_jacobian(eq137_e1743_d_n4);
        let eq137_e1744_d_n5: f64 = self.ddt_jacobian(eq137_e1743_d_n5);
        let eq137_e1744_d_n6: f64 = self.ddt_jacobian(eq137_e1743_d_n6);
        let eq137_e1744_d_n7: f64 = self.ddt_jacobian(eq137_e1743_d_n7);
        let eq137_e1744_d_n8: f64 = self.ddt_jacobian(eq137_e1743_d_n8);
        let eq137_e1744_d_n9: f64 = self.ddt_jacobian(eq137_e1743_d_n9);
        let eq137_e1744_d_n10: f64 = self.ddt_jacobian(eq137_e1743_d_n10);
        let eq137_e1744_d_n11: f64 = self.ddt_jacobian(eq137_e1743_d_n11);
        let eq137_e1744_d_n12: f64 = self.ddt_jacobian(eq137_e1743_d_n12);
        let eq137_e1744_d_n13: f64 = self.ddt_jacobian(eq137_e1743_d_n13);
        let eq137_e1744_d_n14: f64 = self.ddt_jacobian(eq137_e1743_d_n14);
        let eq137_e1744_d_n15: f64 = self.ddt_jacobian(eq137_e1743_d_n15);
        let eq137_e1744_d_n16: f64 = self.ddt_jacobian(eq137_e1743_d_n16);
        let eq137_e1744_d_n17: f64 = self.ddt_jacobian(eq137_e1743_d_n17);
        let eq137_e1744_d_n18: f64 = self.ddt_jacobian(eq137_e1743_d_n18);
        let eq137_e1744_d_n19: f64 = self.ddt_jacobian(eq137_e1743_d_n19);
        let eq137_e1744_d_n20: f64 = self.ddt_jacobian(eq137_e1743_d_n20);
        let eq137_e1744_d_n21: f64 = self.ddt_jacobian(eq137_e1743_d_n21);
        let eq137_e1744_d_n22: f64 = self.ddt_jacobian(eq137_e1743_d_n22);
        let eq137_e1745: f64 = (p.p7 * eq137_e1744);
        let eq137_e1745_d_n0: f64 = (p.p7 * eq137_e1744_d_n0);
        let eq137_e1745_d_n1: f64 = (p.p7 * eq137_e1744_d_n1);
        let eq137_e1745_d_n2: f64 = (p.p7 * eq137_e1744_d_n2);
        let eq137_e1745_d_n3: f64 = (p.p7 * eq137_e1744_d_n3);
        let eq137_e1745_d_n4: f64 = (p.p7 * eq137_e1744_d_n4);
        let eq137_e1745_d_n5: f64 = (p.p7 * eq137_e1744_d_n5);
        let eq137_e1745_d_n6: f64 = (p.p7 * eq137_e1744_d_n6);
        let eq137_e1745_d_n7: f64 = (p.p7 * eq137_e1744_d_n7);
        let eq137_e1745_d_n8: f64 = (p.p7 * eq137_e1744_d_n8);
        let eq137_e1745_d_n9: f64 = (p.p7 * eq137_e1744_d_n9);
        let eq137_e1745_d_n10: f64 = (p.p7 * eq137_e1744_d_n10);
        let eq137_e1745_d_n11: f64 = (p.p7 * eq137_e1744_d_n11);
        let eq137_e1745_d_n12: f64 = (p.p7 * eq137_e1744_d_n12);
        let eq137_e1745_d_n13: f64 = (p.p7 * eq137_e1744_d_n13);
        let eq137_e1745_d_n14: f64 = (p.p7 * eq137_e1744_d_n14);
        let eq137_e1745_d_n15: f64 = (p.p7 * eq137_e1744_d_n15);
        let eq137_e1745_d_n16: f64 = (p.p7 * eq137_e1744_d_n16);
        let eq137_e1745_d_n17: f64 = (p.p7 * eq137_e1744_d_n17);
        let eq137_e1745_d_n18: f64 = (p.p7 * eq137_e1744_d_n18);
        let eq137_e1745_d_n19: f64 = (p.p7 * eq137_e1744_d_n19);
        let eq137_e1745_d_n20: f64 = (p.p7 * eq137_e1744_d_n20);
        let eq137_e1745_d_n21: f64 = (p.p7 * eq137_e1744_d_n21);
        let eq137_e1745_d_n22: f64 = (p.p7 * eq137_e1744_d_n22);
        (eq137_e1745, eq137_e1745_d_n0, eq137_e1745_d_n1, eq137_e1745_d_n2, eq137_e1745_d_n3, eq137_e1745_d_n4, eq137_e1745_d_n5, eq137_e1745_d_n6, eq137_e1745_d_n7, eq137_e1745_d_n8, eq137_e1745_d_n9, eq137_e1745_d_n10, eq137_e1745_d_n11, eq137_e1745_d_n12, eq137_e1745_d_n13, eq137_e1745_d_n14, eq137_e1745_d_n15, eq137_e1745_d_n16, eq137_e1745_d_n17, eq137_e1745_d_n18, eq137_e1745_d_n19, eq137_e1745_d_n20, eq137_e1745_d_n21, eq137_e1745_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1747;
        let eq137_node_derivatives: [f64; 23] = [eq137_e1747_d_n0, eq137_e1747_d_n1, eq137_e1747_d_n2, eq137_e1747_d_n3, eq137_e1747_d_n4, eq137_e1747_d_n5, eq137_e1747_d_n6, eq137_e1747_d_n7, eq137_e1747_d_n8, eq137_e1747_d_n9, eq137_e1747_d_n10, eq137_e1747_d_n11, eq137_e1747_d_n12, eq137_e1747_d_n13, eq137_e1747_d_n14, eq137_e1747_d_n15, eq137_e1747_d_n16, eq137_e1747_d_n17, eq137_e1747_d_n18, eq137_e1747_d_n19, eq137_e1747_d_n20, eq137_e1747_d_n21, eq137_e1747_d_n22];
        let eq137_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[19]),
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
        let (eq138_e1757, eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22,) = {
    if ((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) {
        let eq138_e1754: f64 = self.eval_ddt(37, s.v[241]);
        let eq138_e1754_d_n0: f64 = self.ddt_jacobian(s.dn[241][0]);
        let eq138_e1754_d_n1: f64 = self.ddt_jacobian(s.dn[241][1]);
        let eq138_e1754_d_n2: f64 = self.ddt_jacobian(s.dn[241][2]);
        let eq138_e1754_d_n3: f64 = self.ddt_jacobian(s.dn[241][3]);
        let eq138_e1754_d_n4: f64 = self.ddt_jacobian(s.dn[241][4]);
        let eq138_e1754_d_n5: f64 = self.ddt_jacobian(s.dn[241][5]);
        let eq138_e1754_d_n6: f64 = self.ddt_jacobian(s.dn[241][6]);
        let eq138_e1754_d_n7: f64 = self.ddt_jacobian(s.dn[241][7]);
        let eq138_e1754_d_n8: f64 = self.ddt_jacobian(s.dn[241][8]);
        let eq138_e1754_d_n9: f64 = self.ddt_jacobian(s.dn[241][9]);
        let eq138_e1754_d_n10: f64 = self.ddt_jacobian(s.dn[241][10]);
        let eq138_e1754_d_n11: f64 = self.ddt_jacobian(s.dn[241][11]);
        let eq138_e1754_d_n12: f64 = self.ddt_jacobian(s.dn[241][12]);
        let eq138_e1754_d_n13: f64 = self.ddt_jacobian(s.dn[241][13]);
        let eq138_e1754_d_n14: f64 = self.ddt_jacobian(s.dn[241][14]);
        let eq138_e1754_d_n15: f64 = self.ddt_jacobian(s.dn[241][15]);
        let eq138_e1754_d_n16: f64 = self.ddt_jacobian(s.dn[241][16]);
        let eq138_e1754_d_n17: f64 = self.ddt_jacobian(s.dn[241][17]);
        let eq138_e1754_d_n18: f64 = self.ddt_jacobian(s.dn[241][18]);
        let eq138_e1754_d_n19: f64 = self.ddt_jacobian(s.dn[241][19]);
        let eq138_e1754_d_n20: f64 = self.ddt_jacobian(s.dn[241][20]);
        let eq138_e1754_d_n21: f64 = self.ddt_jacobian(s.dn[241][21]);
        let eq138_e1754_d_n22: f64 = self.ddt_jacobian(s.dn[241][22]);
        let eq138_e1755: f64 = (p.p7 * eq138_e1754);
        let eq138_e1755_d_n0: f64 = (p.p7 * eq138_e1754_d_n0);
        let eq138_e1755_d_n1: f64 = (p.p7 * eq138_e1754_d_n1);
        let eq138_e1755_d_n2: f64 = (p.p7 * eq138_e1754_d_n2);
        let eq138_e1755_d_n3: f64 = (p.p7 * eq138_e1754_d_n3);
        let eq138_e1755_d_n4: f64 = (p.p7 * eq138_e1754_d_n4);
        let eq138_e1755_d_n5: f64 = (p.p7 * eq138_e1754_d_n5);
        let eq138_e1755_d_n6: f64 = (p.p7 * eq138_e1754_d_n6);
        let eq138_e1755_d_n7: f64 = (p.p7 * eq138_e1754_d_n7);
        let eq138_e1755_d_n8: f64 = (p.p7 * eq138_e1754_d_n8);
        let eq138_e1755_d_n9: f64 = (p.p7 * eq138_e1754_d_n9);
        let eq138_e1755_d_n10: f64 = (p.p7 * eq138_e1754_d_n10);
        let eq138_e1755_d_n11: f64 = (p.p7 * eq138_e1754_d_n11);
        let eq138_e1755_d_n12: f64 = (p.p7 * eq138_e1754_d_n12);
        let eq138_e1755_d_n13: f64 = (p.p7 * eq138_e1754_d_n13);
        let eq138_e1755_d_n14: f64 = (p.p7 * eq138_e1754_d_n14);
        let eq138_e1755_d_n15: f64 = (p.p7 * eq138_e1754_d_n15);
        let eq138_e1755_d_n16: f64 = (p.p7 * eq138_e1754_d_n16);
        let eq138_e1755_d_n17: f64 = (p.p7 * eq138_e1754_d_n17);
        let eq138_e1755_d_n18: f64 = (p.p7 * eq138_e1754_d_n18);
        let eq138_e1755_d_n19: f64 = (p.p7 * eq138_e1754_d_n19);
        let eq138_e1755_d_n20: f64 = (p.p7 * eq138_e1754_d_n20);
        let eq138_e1755_d_n21: f64 = (p.p7 * eq138_e1754_d_n21);
        let eq138_e1755_d_n22: f64 = (p.p7 * eq138_e1754_d_n22);
        (eq138_e1755, eq138_e1755_d_n0, eq138_e1755_d_n1, eq138_e1755_d_n2, eq138_e1755_d_n3, eq138_e1755_d_n4, eq138_e1755_d_n5, eq138_e1755_d_n6, eq138_e1755_d_n7, eq138_e1755_d_n8, eq138_e1755_d_n9, eq138_e1755_d_n10, eq138_e1755_d_n11, eq138_e1755_d_n12, eq138_e1755_d_n13, eq138_e1755_d_n14, eq138_e1755_d_n15, eq138_e1755_d_n16, eq138_e1755_d_n17, eq138_e1755_d_n18, eq138_e1755_d_n19, eq138_e1755_d_n20, eq138_e1755_d_n21, eq138_e1755_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq138_value: f64 = eq138_e1757;
        let eq138_node_derivatives: [f64; 23] = [eq138_e1757_d_n0, eq138_e1757_d_n1, eq138_e1757_d_n2, eq138_e1757_d_n3, eq138_e1757_d_n4, eq138_e1757_d_n5, eq138_e1757_d_n6, eq138_e1757_d_n7, eq138_e1757_d_n8, eq138_e1757_d_n9, eq138_e1757_d_n10, eq138_e1757_d_n11, eq138_e1757_d_n12, eq138_e1757_d_n13, eq138_e1757_d_n14, eq138_e1757_d_n15, eq138_e1757_d_n16, eq138_e1757_d_n17, eq138_e1757_d_n18, eq138_e1757_d_n19, eq138_e1757_d_n20, eq138_e1757_d_n21, eq138_e1757_d_n22];
        let eq138_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[2]),
            self.multiplicity * (eq138_value),
            &nodes,
            &eq138_node_derivatives,
            &branches,
            &eq138_branch_derivatives,
            self.multiplicity,
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
        let (eq139_e1769, eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (s.v[579] != 0.0)) {
        let eq139_e1766: f64 = self.eval_ddt(38, s.v[240]);
        let eq139_e1766_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq139_e1766_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq139_e1766_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq139_e1766_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq139_e1766_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq139_e1766_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq139_e1766_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq139_e1766_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq139_e1766_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq139_e1766_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq139_e1766_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq139_e1766_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq139_e1766_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq139_e1766_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq139_e1766_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq139_e1766_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq139_e1766_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq139_e1766_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq139_e1766_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq139_e1766_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq139_e1766_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq139_e1766_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq139_e1766_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq139_e1767: f64 = (p.p7 * eq139_e1766);
        let eq139_e1767_d_n0: f64 = (p.p7 * eq139_e1766_d_n0);
        let eq139_e1767_d_n1: f64 = (p.p7 * eq139_e1766_d_n1);
        let eq139_e1767_d_n2: f64 = (p.p7 * eq139_e1766_d_n2);
        let eq139_e1767_d_n3: f64 = (p.p7 * eq139_e1766_d_n3);
        let eq139_e1767_d_n4: f64 = (p.p7 * eq139_e1766_d_n4);
        let eq139_e1767_d_n5: f64 = (p.p7 * eq139_e1766_d_n5);
        let eq139_e1767_d_n6: f64 = (p.p7 * eq139_e1766_d_n6);
        let eq139_e1767_d_n7: f64 = (p.p7 * eq139_e1766_d_n7);
        let eq139_e1767_d_n8: f64 = (p.p7 * eq139_e1766_d_n8);
        let eq139_e1767_d_n9: f64 = (p.p7 * eq139_e1766_d_n9);
        let eq139_e1767_d_n10: f64 = (p.p7 * eq139_e1766_d_n10);
        let eq139_e1767_d_n11: f64 = (p.p7 * eq139_e1766_d_n11);
        let eq139_e1767_d_n12: f64 = (p.p7 * eq139_e1766_d_n12);
        let eq139_e1767_d_n13: f64 = (p.p7 * eq139_e1766_d_n13);
        let eq139_e1767_d_n14: f64 = (p.p7 * eq139_e1766_d_n14);
        let eq139_e1767_d_n15: f64 = (p.p7 * eq139_e1766_d_n15);
        let eq139_e1767_d_n16: f64 = (p.p7 * eq139_e1766_d_n16);
        let eq139_e1767_d_n17: f64 = (p.p7 * eq139_e1766_d_n17);
        let eq139_e1767_d_n18: f64 = (p.p7 * eq139_e1766_d_n18);
        let eq139_e1767_d_n19: f64 = (p.p7 * eq139_e1766_d_n19);
        let eq139_e1767_d_n20: f64 = (p.p7 * eq139_e1766_d_n20);
        let eq139_e1767_d_n21: f64 = (p.p7 * eq139_e1766_d_n21);
        let eq139_e1767_d_n22: f64 = (p.p7 * eq139_e1766_d_n22);
        (eq139_e1767, eq139_e1767_d_n0, eq139_e1767_d_n1, eq139_e1767_d_n2, eq139_e1767_d_n3, eq139_e1767_d_n4, eq139_e1767_d_n5, eq139_e1767_d_n6, eq139_e1767_d_n7, eq139_e1767_d_n8, eq139_e1767_d_n9, eq139_e1767_d_n10, eq139_e1767_d_n11, eq139_e1767_d_n12, eq139_e1767_d_n13, eq139_e1767_d_n14, eq139_e1767_d_n15, eq139_e1767_d_n16, eq139_e1767_d_n17, eq139_e1767_d_n18, eq139_e1767_d_n19, eq139_e1767_d_n20, eq139_e1767_d_n21, eq139_e1767_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq139_value: f64 = eq139_e1769;
        let eq139_node_derivatives: [f64; 23] = [eq139_e1769_d_n0, eq139_e1769_d_n1, eq139_e1769_d_n2, eq139_e1769_d_n3, eq139_e1769_d_n4, eq139_e1769_d_n5, eq139_e1769_d_n6, eq139_e1769_d_n7, eq139_e1769_d_n8, eq139_e1769_d_n9, eq139_e1769_d_n10, eq139_e1769_d_n11, eq139_e1769_d_n12, eq139_e1769_d_n13, eq139_e1769_d_n14, eq139_e1769_d_n15, eq139_e1769_d_n16, eq139_e1769_d_n17, eq139_e1769_d_n18, eq139_e1769_d_n19, eq139_e1769_d_n20, eq139_e1769_d_n21, eq139_e1769_d_n22];
        let eq139_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            self.multiplicity * (eq139_value),
            &nodes,
            &eq139_node_derivatives,
            &branches,
            &eq139_branch_derivatives,
            self.multiplicity,
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
        let (eq140_e1783, eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (s.v[579] != 0.0)) {
        let eq140_e1778: f64 = self.eval_ddt(39, s.v[240]);
        let eq140_e1778_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq140_e1778_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq140_e1778_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq140_e1778_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq140_e1778_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq140_e1778_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq140_e1778_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq140_e1778_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq140_e1778_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq140_e1778_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq140_e1778_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq140_e1778_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq140_e1778_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq140_e1778_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq140_e1778_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq140_e1778_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq140_e1778_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq140_e1778_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq140_e1778_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq140_e1778_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq140_e1778_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq140_e1778_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq140_e1778_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq140_e1779: f64 = (p.p7 * eq140_e1778);
        let eq140_e1779_d_n0: f64 = (p.p7 * eq140_e1778_d_n0);
        let eq140_e1779_d_n1: f64 = (p.p7 * eq140_e1778_d_n1);
        let eq140_e1779_d_n2: f64 = (p.p7 * eq140_e1778_d_n2);
        let eq140_e1779_d_n3: f64 = (p.p7 * eq140_e1778_d_n3);
        let eq140_e1779_d_n4: f64 = (p.p7 * eq140_e1778_d_n4);
        let eq140_e1779_d_n5: f64 = (p.p7 * eq140_e1778_d_n5);
        let eq140_e1779_d_n6: f64 = (p.p7 * eq140_e1778_d_n6);
        let eq140_e1779_d_n7: f64 = (p.p7 * eq140_e1778_d_n7);
        let eq140_e1779_d_n8: f64 = (p.p7 * eq140_e1778_d_n8);
        let eq140_e1779_d_n9: f64 = (p.p7 * eq140_e1778_d_n9);
        let eq140_e1779_d_n10: f64 = (p.p7 * eq140_e1778_d_n10);
        let eq140_e1779_d_n11: f64 = (p.p7 * eq140_e1778_d_n11);
        let eq140_e1779_d_n12: f64 = (p.p7 * eq140_e1778_d_n12);
        let eq140_e1779_d_n13: f64 = (p.p7 * eq140_e1778_d_n13);
        let eq140_e1779_d_n14: f64 = (p.p7 * eq140_e1778_d_n14);
        let eq140_e1779_d_n15: f64 = (p.p7 * eq140_e1778_d_n15);
        let eq140_e1779_d_n16: f64 = (p.p7 * eq140_e1778_d_n16);
        let eq140_e1779_d_n17: f64 = (p.p7 * eq140_e1778_d_n17);
        let eq140_e1779_d_n18: f64 = (p.p7 * eq140_e1778_d_n18);
        let eq140_e1779_d_n19: f64 = (p.p7 * eq140_e1778_d_n19);
        let eq140_e1779_d_n20: f64 = (p.p7 * eq140_e1778_d_n20);
        let eq140_e1779_d_n21: f64 = (p.p7 * eq140_e1778_d_n21);
        let eq140_e1779_d_n22: f64 = (p.p7 * eq140_e1778_d_n22);
        let eq140_e1781: f64 = (eq140_e1779 * p.p246);
        let eq140_e1781_d_n0: f64 = (eq140_e1779_d_n0 * p.p246);
        let eq140_e1781_d_n1: f64 = (eq140_e1779_d_n1 * p.p246);
        let eq140_e1781_d_n2: f64 = (eq140_e1779_d_n2 * p.p246);
        let eq140_e1781_d_n3: f64 = (eq140_e1779_d_n3 * p.p246);
        let eq140_e1781_d_n4: f64 = (eq140_e1779_d_n4 * p.p246);
        let eq140_e1781_d_n5: f64 = (eq140_e1779_d_n5 * p.p246);
        let eq140_e1781_d_n6: f64 = (eq140_e1779_d_n6 * p.p246);
        let eq140_e1781_d_n7: f64 = (eq140_e1779_d_n7 * p.p246);
        let eq140_e1781_d_n8: f64 = (eq140_e1779_d_n8 * p.p246);
        let eq140_e1781_d_n9: f64 = (eq140_e1779_d_n9 * p.p246);
        let eq140_e1781_d_n10: f64 = (eq140_e1779_d_n10 * p.p246);
        let eq140_e1781_d_n11: f64 = (eq140_e1779_d_n11 * p.p246);
        let eq140_e1781_d_n12: f64 = (eq140_e1779_d_n12 * p.p246);
        let eq140_e1781_d_n13: f64 = (eq140_e1779_d_n13 * p.p246);
        let eq140_e1781_d_n14: f64 = (eq140_e1779_d_n14 * p.p246);
        let eq140_e1781_d_n15: f64 = (eq140_e1779_d_n15 * p.p246);
        let eq140_e1781_d_n16: f64 = (eq140_e1779_d_n16 * p.p246);
        let eq140_e1781_d_n17: f64 = (eq140_e1779_d_n17 * p.p246);
        let eq140_e1781_d_n18: f64 = (eq140_e1779_d_n18 * p.p246);
        let eq140_e1781_d_n19: f64 = (eq140_e1779_d_n19 * p.p246);
        let eq140_e1781_d_n20: f64 = (eq140_e1779_d_n20 * p.p246);
        let eq140_e1781_d_n21: f64 = (eq140_e1779_d_n21 * p.p246);
        let eq140_e1781_d_n22: f64 = (eq140_e1779_d_n22 * p.p246);
        (eq140_e1781, eq140_e1781_d_n0, eq140_e1781_d_n1, eq140_e1781_d_n2, eq140_e1781_d_n3, eq140_e1781_d_n4, eq140_e1781_d_n5, eq140_e1781_d_n6, eq140_e1781_d_n7, eq140_e1781_d_n8, eq140_e1781_d_n9, eq140_e1781_d_n10, eq140_e1781_d_n11, eq140_e1781_d_n12, eq140_e1781_d_n13, eq140_e1781_d_n14, eq140_e1781_d_n15, eq140_e1781_d_n16, eq140_e1781_d_n17, eq140_e1781_d_n18, eq140_e1781_d_n19, eq140_e1781_d_n20, eq140_e1781_d_n21, eq140_e1781_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq140_value: f64 = eq140_e1783;
        let eq140_node_derivatives: [f64; 23] = [eq140_e1783_d_n0, eq140_e1783_d_n1, eq140_e1783_d_n2, eq140_e1783_d_n3, eq140_e1783_d_n4, eq140_e1783_d_n5, eq140_e1783_d_n6, eq140_e1783_d_n7, eq140_e1783_d_n8, eq140_e1783_d_n9, eq140_e1783_d_n10, eq140_e1783_d_n11, eq140_e1783_d_n12, eq140_e1783_d_n13, eq140_e1783_d_n14, eq140_e1783_d_n15, eq140_e1783_d_n16, eq140_e1783_d_n17, eq140_e1783_d_n18, eq140_e1783_d_n19, eq140_e1783_d_n20, eq140_e1783_d_n21, eq140_e1783_d_n22];
        let eq140_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
            self.multiplicity * (eq140_value),
            &nodes,
            &eq140_node_derivatives,
            &branches,
            &eq140_branch_derivatives,
            self.multiplicity,
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
        let (eq141_e1796, eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (!(s.v[579] != 0.0))) {
        let eq141_e1793: f64 = self.eval_ddt(40, s.v[240]);
        let eq141_e1793_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq141_e1793_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq141_e1793_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq141_e1793_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq141_e1793_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq141_e1793_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq141_e1793_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq141_e1793_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq141_e1793_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq141_e1793_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq141_e1793_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq141_e1793_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq141_e1793_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq141_e1793_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq141_e1793_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq141_e1793_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq141_e1793_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq141_e1793_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq141_e1793_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq141_e1793_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq141_e1793_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq141_e1793_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq141_e1793_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq141_e1794: f64 = (p.p7 * eq141_e1793);
        let eq141_e1794_d_n0: f64 = (p.p7 * eq141_e1793_d_n0);
        let eq141_e1794_d_n1: f64 = (p.p7 * eq141_e1793_d_n1);
        let eq141_e1794_d_n2: f64 = (p.p7 * eq141_e1793_d_n2);
        let eq141_e1794_d_n3: f64 = (p.p7 * eq141_e1793_d_n3);
        let eq141_e1794_d_n4: f64 = (p.p7 * eq141_e1793_d_n4);
        let eq141_e1794_d_n5: f64 = (p.p7 * eq141_e1793_d_n5);
        let eq141_e1794_d_n6: f64 = (p.p7 * eq141_e1793_d_n6);
        let eq141_e1794_d_n7: f64 = (p.p7 * eq141_e1793_d_n7);
        let eq141_e1794_d_n8: f64 = (p.p7 * eq141_e1793_d_n8);
        let eq141_e1794_d_n9: f64 = (p.p7 * eq141_e1793_d_n9);
        let eq141_e1794_d_n10: f64 = (p.p7 * eq141_e1793_d_n10);
        let eq141_e1794_d_n11: f64 = (p.p7 * eq141_e1793_d_n11);
        let eq141_e1794_d_n12: f64 = (p.p7 * eq141_e1793_d_n12);
        let eq141_e1794_d_n13: f64 = (p.p7 * eq141_e1793_d_n13);
        let eq141_e1794_d_n14: f64 = (p.p7 * eq141_e1793_d_n14);
        let eq141_e1794_d_n15: f64 = (p.p7 * eq141_e1793_d_n15);
        let eq141_e1794_d_n16: f64 = (p.p7 * eq141_e1793_d_n16);
        let eq141_e1794_d_n17: f64 = (p.p7 * eq141_e1793_d_n17);
        let eq141_e1794_d_n18: f64 = (p.p7 * eq141_e1793_d_n18);
        let eq141_e1794_d_n19: f64 = (p.p7 * eq141_e1793_d_n19);
        let eq141_e1794_d_n20: f64 = (p.p7 * eq141_e1793_d_n20);
        let eq141_e1794_d_n21: f64 = (p.p7 * eq141_e1793_d_n21);
        let eq141_e1794_d_n22: f64 = (p.p7 * eq141_e1793_d_n22);
        (eq141_e1794, eq141_e1794_d_n0, eq141_e1794_d_n1, eq141_e1794_d_n2, eq141_e1794_d_n3, eq141_e1794_d_n4, eq141_e1794_d_n5, eq141_e1794_d_n6, eq141_e1794_d_n7, eq141_e1794_d_n8, eq141_e1794_d_n9, eq141_e1794_d_n10, eq141_e1794_d_n11, eq141_e1794_d_n12, eq141_e1794_d_n13, eq141_e1794_d_n14, eq141_e1794_d_n15, eq141_e1794_d_n16, eq141_e1794_d_n17, eq141_e1794_d_n18, eq141_e1794_d_n19, eq141_e1794_d_n20, eq141_e1794_d_n21, eq141_e1794_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1796;
        let eq141_node_derivatives: [f64; 23] = [eq141_e1796_d_n0, eq141_e1796_d_n1, eq141_e1796_d_n2, eq141_e1796_d_n3, eq141_e1796_d_n4, eq141_e1796_d_n5, eq141_e1796_d_n6, eq141_e1796_d_n7, eq141_e1796_d_n8, eq141_e1796_d_n9, eq141_e1796_d_n10, eq141_e1796_d_n11, eq141_e1796_d_n12, eq141_e1796_d_n13, eq141_e1796_d_n14, eq141_e1796_d_n15, eq141_e1796_d_n16, eq141_e1796_d_n17, eq141_e1796_d_n18, eq141_e1796_d_n19, eq141_e1796_d_n20, eq141_e1796_d_n21, eq141_e1796_d_n22];
        let eq141_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[8]),
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
        let (eq142_e1811, eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22,) = {
    if (((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) && (!(s.v[579] != 0.0))) {
        let eq142_e1806: f64 = self.eval_ddt(41, s.v[240]);
        let eq142_e1806_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq142_e1806_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq142_e1806_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq142_e1806_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq142_e1806_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq142_e1806_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq142_e1806_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq142_e1806_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq142_e1806_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq142_e1806_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq142_e1806_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq142_e1806_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq142_e1806_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq142_e1806_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq142_e1806_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq142_e1806_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq142_e1806_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq142_e1806_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq142_e1806_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq142_e1806_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq142_e1806_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq142_e1806_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq142_e1806_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq142_e1807: f64 = (p.p7 * eq142_e1806);
        let eq142_e1807_d_n0: f64 = (p.p7 * eq142_e1806_d_n0);
        let eq142_e1807_d_n1: f64 = (p.p7 * eq142_e1806_d_n1);
        let eq142_e1807_d_n2: f64 = (p.p7 * eq142_e1806_d_n2);
        let eq142_e1807_d_n3: f64 = (p.p7 * eq142_e1806_d_n3);
        let eq142_e1807_d_n4: f64 = (p.p7 * eq142_e1806_d_n4);
        let eq142_e1807_d_n5: f64 = (p.p7 * eq142_e1806_d_n5);
        let eq142_e1807_d_n6: f64 = (p.p7 * eq142_e1806_d_n6);
        let eq142_e1807_d_n7: f64 = (p.p7 * eq142_e1806_d_n7);
        let eq142_e1807_d_n8: f64 = (p.p7 * eq142_e1806_d_n8);
        let eq142_e1807_d_n9: f64 = (p.p7 * eq142_e1806_d_n9);
        let eq142_e1807_d_n10: f64 = (p.p7 * eq142_e1806_d_n10);
        let eq142_e1807_d_n11: f64 = (p.p7 * eq142_e1806_d_n11);
        let eq142_e1807_d_n12: f64 = (p.p7 * eq142_e1806_d_n12);
        let eq142_e1807_d_n13: f64 = (p.p7 * eq142_e1806_d_n13);
        let eq142_e1807_d_n14: f64 = (p.p7 * eq142_e1806_d_n14);
        let eq142_e1807_d_n15: f64 = (p.p7 * eq142_e1806_d_n15);
        let eq142_e1807_d_n16: f64 = (p.p7 * eq142_e1806_d_n16);
        let eq142_e1807_d_n17: f64 = (p.p7 * eq142_e1806_d_n17);
        let eq142_e1807_d_n18: f64 = (p.p7 * eq142_e1806_d_n18);
        let eq142_e1807_d_n19: f64 = (p.p7 * eq142_e1806_d_n19);
        let eq142_e1807_d_n20: f64 = (p.p7 * eq142_e1806_d_n20);
        let eq142_e1807_d_n21: f64 = (p.p7 * eq142_e1806_d_n21);
        let eq142_e1807_d_n22: f64 = (p.p7 * eq142_e1806_d_n22);
        let eq142_e1809: f64 = (eq142_e1807 * p.p246);
        let eq142_e1809_d_n0: f64 = (eq142_e1807_d_n0 * p.p246);
        let eq142_e1809_d_n1: f64 = (eq142_e1807_d_n1 * p.p246);
        let eq142_e1809_d_n2: f64 = (eq142_e1807_d_n2 * p.p246);
        let eq142_e1809_d_n3: f64 = (eq142_e1807_d_n3 * p.p246);
        let eq142_e1809_d_n4: f64 = (eq142_e1807_d_n4 * p.p246);
        let eq142_e1809_d_n5: f64 = (eq142_e1807_d_n5 * p.p246);
        let eq142_e1809_d_n6: f64 = (eq142_e1807_d_n6 * p.p246);
        let eq142_e1809_d_n7: f64 = (eq142_e1807_d_n7 * p.p246);
        let eq142_e1809_d_n8: f64 = (eq142_e1807_d_n8 * p.p246);
        let eq142_e1809_d_n9: f64 = (eq142_e1807_d_n9 * p.p246);
        let eq142_e1809_d_n10: f64 = (eq142_e1807_d_n10 * p.p246);
        let eq142_e1809_d_n11: f64 = (eq142_e1807_d_n11 * p.p246);
        let eq142_e1809_d_n12: f64 = (eq142_e1807_d_n12 * p.p246);
        let eq142_e1809_d_n13: f64 = (eq142_e1807_d_n13 * p.p246);
        let eq142_e1809_d_n14: f64 = (eq142_e1807_d_n14 * p.p246);
        let eq142_e1809_d_n15: f64 = (eq142_e1807_d_n15 * p.p246);
        let eq142_e1809_d_n16: f64 = (eq142_e1807_d_n16 * p.p246);
        let eq142_e1809_d_n17: f64 = (eq142_e1807_d_n17 * p.p246);
        let eq142_e1809_d_n18: f64 = (eq142_e1807_d_n18 * p.p246);
        let eq142_e1809_d_n19: f64 = (eq142_e1807_d_n19 * p.p246);
        let eq142_e1809_d_n20: f64 = (eq142_e1807_d_n20 * p.p246);
        let eq142_e1809_d_n21: f64 = (eq142_e1807_d_n21 * p.p246);
        let eq142_e1809_d_n22: f64 = (eq142_e1807_d_n22 * p.p246);
        (eq142_e1809, eq142_e1809_d_n0, eq142_e1809_d_n1, eq142_e1809_d_n2, eq142_e1809_d_n3, eq142_e1809_d_n4, eq142_e1809_d_n5, eq142_e1809_d_n6, eq142_e1809_d_n7, eq142_e1809_d_n8, eq142_e1809_d_n9, eq142_e1809_d_n10, eq142_e1809_d_n11, eq142_e1809_d_n12, eq142_e1809_d_n13, eq142_e1809_d_n14, eq142_e1809_d_n15, eq142_e1809_d_n16, eq142_e1809_d_n17, eq142_e1809_d_n18, eq142_e1809_d_n19, eq142_e1809_d_n20, eq142_e1809_d_n21, eq142_e1809_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1811;
        let eq142_node_derivatives: [f64; 23] = [eq142_e1811_d_n0, eq142_e1811_d_n1, eq142_e1811_d_n2, eq142_e1811_d_n3, eq142_e1811_d_n4, eq142_e1811_d_n5, eq142_e1811_d_n6, eq142_e1811_d_n7, eq142_e1811_d_n8, eq142_e1811_d_n9, eq142_e1811_d_n10, eq142_e1811_d_n11, eq142_e1811_d_n12, eq142_e1811_d_n13, eq142_e1811_d_n14, eq142_e1811_d_n15, eq142_e1811_d_n16, eq142_e1811_d_n17, eq142_e1811_d_n18, eq142_e1811_d_n19, eq142_e1811_d_n20, eq142_e1811_d_n21, eq142_e1811_d_n22];
        let eq142_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
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
        let (eq143_e1823, eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22,) = {
    if ((!(s.v[575] != 0.0)) && (s.v[578] != 0.0)) {
        let eq143_e1819: f64 = (p.p251 * s.v[240]);
        let eq143_e1819_d_n0: f64 = (p.p251 * s.dn[240][0]);
        let eq143_e1819_d_n1: f64 = (p.p251 * s.dn[240][1]);
        let eq143_e1819_d_n2: f64 = (p.p251 * s.dn[240][2]);
        let eq143_e1819_d_n3: f64 = (p.p251 * s.dn[240][3]);
        let eq143_e1819_d_n4: f64 = (p.p251 * s.dn[240][4]);
        let eq143_e1819_d_n5: f64 = (p.p251 * s.dn[240][5]);
        let eq143_e1819_d_n6: f64 = (p.p251 * s.dn[240][6]);
        let eq143_e1819_d_n7: f64 = (p.p251 * s.dn[240][7]);
        let eq143_e1819_d_n8: f64 = (p.p251 * s.dn[240][8]);
        let eq143_e1819_d_n9: f64 = (p.p251 * s.dn[240][9]);
        let eq143_e1819_d_n10: f64 = (p.p251 * s.dn[240][10]);
        let eq143_e1819_d_n11: f64 = (p.p251 * s.dn[240][11]);
        let eq143_e1819_d_n12: f64 = (p.p251 * s.dn[240][12]);
        let eq143_e1819_d_n13: f64 = (p.p251 * s.dn[240][13]);
        let eq143_e1819_d_n14: f64 = (p.p251 * s.dn[240][14]);
        let eq143_e1819_d_n15: f64 = (p.p251 * s.dn[240][15]);
        let eq143_e1819_d_n16: f64 = (p.p251 * s.dn[240][16]);
        let eq143_e1819_d_n17: f64 = (p.p251 * s.dn[240][17]);
        let eq143_e1819_d_n18: f64 = (p.p251 * s.dn[240][18]);
        let eq143_e1819_d_n19: f64 = (p.p251 * s.dn[240][19]);
        let eq143_e1819_d_n20: f64 = (p.p251 * s.dn[240][20]);
        let eq143_e1819_d_n21: f64 = (p.p251 * s.dn[240][21]);
        let eq143_e1819_d_n22: f64 = (p.p251 * s.dn[240][22]);
        let eq143_e1820: f64 = self.eval_ddt(42, eq143_e1819);
        let eq143_e1820_d_n0: f64 = self.ddt_jacobian(eq143_e1819_d_n0);
        let eq143_e1820_d_n1: f64 = self.ddt_jacobian(eq143_e1819_d_n1);
        let eq143_e1820_d_n2: f64 = self.ddt_jacobian(eq143_e1819_d_n2);
        let eq143_e1820_d_n3: f64 = self.ddt_jacobian(eq143_e1819_d_n3);
        let eq143_e1820_d_n4: f64 = self.ddt_jacobian(eq143_e1819_d_n4);
        let eq143_e1820_d_n5: f64 = self.ddt_jacobian(eq143_e1819_d_n5);
        let eq143_e1820_d_n6: f64 = self.ddt_jacobian(eq143_e1819_d_n6);
        let eq143_e1820_d_n7: f64 = self.ddt_jacobian(eq143_e1819_d_n7);
        let eq143_e1820_d_n8: f64 = self.ddt_jacobian(eq143_e1819_d_n8);
        let eq143_e1820_d_n9: f64 = self.ddt_jacobian(eq143_e1819_d_n9);
        let eq143_e1820_d_n10: f64 = self.ddt_jacobian(eq143_e1819_d_n10);
        let eq143_e1820_d_n11: f64 = self.ddt_jacobian(eq143_e1819_d_n11);
        let eq143_e1820_d_n12: f64 = self.ddt_jacobian(eq143_e1819_d_n12);
        let eq143_e1820_d_n13: f64 = self.ddt_jacobian(eq143_e1819_d_n13);
        let eq143_e1820_d_n14: f64 = self.ddt_jacobian(eq143_e1819_d_n14);
        let eq143_e1820_d_n15: f64 = self.ddt_jacobian(eq143_e1819_d_n15);
        let eq143_e1820_d_n16: f64 = self.ddt_jacobian(eq143_e1819_d_n16);
        let eq143_e1820_d_n17: f64 = self.ddt_jacobian(eq143_e1819_d_n17);
        let eq143_e1820_d_n18: f64 = self.ddt_jacobian(eq143_e1819_d_n18);
        let eq143_e1820_d_n19: f64 = self.ddt_jacobian(eq143_e1819_d_n19);
        let eq143_e1820_d_n20: f64 = self.ddt_jacobian(eq143_e1819_d_n20);
        let eq143_e1820_d_n21: f64 = self.ddt_jacobian(eq143_e1819_d_n21);
        let eq143_e1820_d_n22: f64 = self.ddt_jacobian(eq143_e1819_d_n22);
        let eq143_e1821: f64 = (p.p7 * eq143_e1820);
        let eq143_e1821_d_n0: f64 = (p.p7 * eq143_e1820_d_n0);
        let eq143_e1821_d_n1: f64 = (p.p7 * eq143_e1820_d_n1);
        let eq143_e1821_d_n2: f64 = (p.p7 * eq143_e1820_d_n2);
        let eq143_e1821_d_n3: f64 = (p.p7 * eq143_e1820_d_n3);
        let eq143_e1821_d_n4: f64 = (p.p7 * eq143_e1820_d_n4);
        let eq143_e1821_d_n5: f64 = (p.p7 * eq143_e1820_d_n5);
        let eq143_e1821_d_n6: f64 = (p.p7 * eq143_e1820_d_n6);
        let eq143_e1821_d_n7: f64 = (p.p7 * eq143_e1820_d_n7);
        let eq143_e1821_d_n8: f64 = (p.p7 * eq143_e1820_d_n8);
        let eq143_e1821_d_n9: f64 = (p.p7 * eq143_e1820_d_n9);
        let eq143_e1821_d_n10: f64 = (p.p7 * eq143_e1820_d_n10);
        let eq143_e1821_d_n11: f64 = (p.p7 * eq143_e1820_d_n11);
        let eq143_e1821_d_n12: f64 = (p.p7 * eq143_e1820_d_n12);
        let eq143_e1821_d_n13: f64 = (p.p7 * eq143_e1820_d_n13);
        let eq143_e1821_d_n14: f64 = (p.p7 * eq143_e1820_d_n14);
        let eq143_e1821_d_n15: f64 = (p.p7 * eq143_e1820_d_n15);
        let eq143_e1821_d_n16: f64 = (p.p7 * eq143_e1820_d_n16);
        let eq143_e1821_d_n17: f64 = (p.p7 * eq143_e1820_d_n17);
        let eq143_e1821_d_n18: f64 = (p.p7 * eq143_e1820_d_n18);
        let eq143_e1821_d_n19: f64 = (p.p7 * eq143_e1820_d_n19);
        let eq143_e1821_d_n20: f64 = (p.p7 * eq143_e1820_d_n20);
        let eq143_e1821_d_n21: f64 = (p.p7 * eq143_e1820_d_n21);
        let eq143_e1821_d_n22: f64 = (p.p7 * eq143_e1820_d_n22);
        (eq143_e1821, eq143_e1821_d_n0, eq143_e1821_d_n1, eq143_e1821_d_n2, eq143_e1821_d_n3, eq143_e1821_d_n4, eq143_e1821_d_n5, eq143_e1821_d_n6, eq143_e1821_d_n7, eq143_e1821_d_n8, eq143_e1821_d_n9, eq143_e1821_d_n10, eq143_e1821_d_n11, eq143_e1821_d_n12, eq143_e1821_d_n13, eq143_e1821_d_n14, eq143_e1821_d_n15, eq143_e1821_d_n16, eq143_e1821_d_n17, eq143_e1821_d_n18, eq143_e1821_d_n19, eq143_e1821_d_n20, eq143_e1821_d_n21, eq143_e1821_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1823;
        let eq143_node_derivatives: [f64; 23] = [eq143_e1823_d_n0, eq143_e1823_d_n1, eq143_e1823_d_n2, eq143_e1823_d_n3, eq143_e1823_d_n4, eq143_e1823_d_n5, eq143_e1823_d_n6, eq143_e1823_d_n7, eq143_e1823_d_n8, eq143_e1823_d_n9, eq143_e1823_d_n10, eq143_e1823_d_n11, eq143_e1823_d_n12, eq143_e1823_d_n13, eq143_e1823_d_n14, eq143_e1823_d_n15, eq143_e1823_d_n16, eq143_e1823_d_n17, eq143_e1823_d_n18, eq143_e1823_d_n19, eq143_e1823_d_n20, eq143_e1823_d_n21, eq143_e1823_d_n22];
        let eq143_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
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
        let (eq144_e1832, eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22,) = {
    if ((s.v[580] != 0.0) && (s.v[581] != 0.0)) {
        let eq144_e1829: f64 = self.eval_ddt(43, s.v[253]);
        let eq144_e1829_d_n0: f64 = self.ddt_jacobian(s.dn[253][0]);
        let eq144_e1829_d_n1: f64 = self.ddt_jacobian(s.dn[253][1]);
        let eq144_e1829_d_n2: f64 = self.ddt_jacobian(s.dn[253][2]);
        let eq144_e1829_d_n3: f64 = self.ddt_jacobian(s.dn[253][3]);
        let eq144_e1829_d_n4: f64 = self.ddt_jacobian(s.dn[253][4]);
        let eq144_e1829_d_n5: f64 = self.ddt_jacobian(s.dn[253][5]);
        let eq144_e1829_d_n6: f64 = self.ddt_jacobian(s.dn[253][6]);
        let eq144_e1829_d_n7: f64 = self.ddt_jacobian(s.dn[253][7]);
        let eq144_e1829_d_n8: f64 = self.ddt_jacobian(s.dn[253][8]);
        let eq144_e1829_d_n9: f64 = self.ddt_jacobian(s.dn[253][9]);
        let eq144_e1829_d_n10: f64 = self.ddt_jacobian(s.dn[253][10]);
        let eq144_e1829_d_n11: f64 = self.ddt_jacobian(s.dn[253][11]);
        let eq144_e1829_d_n12: f64 = self.ddt_jacobian(s.dn[253][12]);
        let eq144_e1829_d_n13: f64 = self.ddt_jacobian(s.dn[253][13]);
        let eq144_e1829_d_n14: f64 = self.ddt_jacobian(s.dn[253][14]);
        let eq144_e1829_d_n15: f64 = self.ddt_jacobian(s.dn[253][15]);
        let eq144_e1829_d_n16: f64 = self.ddt_jacobian(s.dn[253][16]);
        let eq144_e1829_d_n17: f64 = self.ddt_jacobian(s.dn[253][17]);
        let eq144_e1829_d_n18: f64 = self.ddt_jacobian(s.dn[253][18]);
        let eq144_e1829_d_n19: f64 = self.ddt_jacobian(s.dn[253][19]);
        let eq144_e1829_d_n20: f64 = self.ddt_jacobian(s.dn[253][20]);
        let eq144_e1829_d_n21: f64 = self.ddt_jacobian(s.dn[253][21]);
        let eq144_e1829_d_n22: f64 = self.ddt_jacobian(s.dn[253][22]);
        let eq144_e1830: f64 = (p.p7 * eq144_e1829);
        let eq144_e1830_d_n0: f64 = (p.p7 * eq144_e1829_d_n0);
        let eq144_e1830_d_n1: f64 = (p.p7 * eq144_e1829_d_n1);
        let eq144_e1830_d_n2: f64 = (p.p7 * eq144_e1829_d_n2);
        let eq144_e1830_d_n3: f64 = (p.p7 * eq144_e1829_d_n3);
        let eq144_e1830_d_n4: f64 = (p.p7 * eq144_e1829_d_n4);
        let eq144_e1830_d_n5: f64 = (p.p7 * eq144_e1829_d_n5);
        let eq144_e1830_d_n6: f64 = (p.p7 * eq144_e1829_d_n6);
        let eq144_e1830_d_n7: f64 = (p.p7 * eq144_e1829_d_n7);
        let eq144_e1830_d_n8: f64 = (p.p7 * eq144_e1829_d_n8);
        let eq144_e1830_d_n9: f64 = (p.p7 * eq144_e1829_d_n9);
        let eq144_e1830_d_n10: f64 = (p.p7 * eq144_e1829_d_n10);
        let eq144_e1830_d_n11: f64 = (p.p7 * eq144_e1829_d_n11);
        let eq144_e1830_d_n12: f64 = (p.p7 * eq144_e1829_d_n12);
        let eq144_e1830_d_n13: f64 = (p.p7 * eq144_e1829_d_n13);
        let eq144_e1830_d_n14: f64 = (p.p7 * eq144_e1829_d_n14);
        let eq144_e1830_d_n15: f64 = (p.p7 * eq144_e1829_d_n15);
        let eq144_e1830_d_n16: f64 = (p.p7 * eq144_e1829_d_n16);
        let eq144_e1830_d_n17: f64 = (p.p7 * eq144_e1829_d_n17);
        let eq144_e1830_d_n18: f64 = (p.p7 * eq144_e1829_d_n18);
        let eq144_e1830_d_n19: f64 = (p.p7 * eq144_e1829_d_n19);
        let eq144_e1830_d_n20: f64 = (p.p7 * eq144_e1829_d_n20);
        let eq144_e1830_d_n21: f64 = (p.p7 * eq144_e1829_d_n21);
        let eq144_e1830_d_n22: f64 = (p.p7 * eq144_e1829_d_n22);
        (eq144_e1830, eq144_e1830_d_n0, eq144_e1830_d_n1, eq144_e1830_d_n2, eq144_e1830_d_n3, eq144_e1830_d_n4, eq144_e1830_d_n5, eq144_e1830_d_n6, eq144_e1830_d_n7, eq144_e1830_d_n8, eq144_e1830_d_n9, eq144_e1830_d_n10, eq144_e1830_d_n11, eq144_e1830_d_n12, eq144_e1830_d_n13, eq144_e1830_d_n14, eq144_e1830_d_n15, eq144_e1830_d_n16, eq144_e1830_d_n17, eq144_e1830_d_n18, eq144_e1830_d_n19, eq144_e1830_d_n20, eq144_e1830_d_n21, eq144_e1830_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq144_value: f64 = eq144_e1832;
        let eq144_node_derivatives: [f64; 23] = [eq144_e1832_d_n0, eq144_e1832_d_n1, eq144_e1832_d_n2, eq144_e1832_d_n3, eq144_e1832_d_n4, eq144_e1832_d_n5, eq144_e1832_d_n6, eq144_e1832_d_n7, eq144_e1832_d_n8, eq144_e1832_d_n9, eq144_e1832_d_n10, eq144_e1832_d_n11, eq144_e1832_d_n12, eq144_e1832_d_n13, eq144_e1832_d_n14, eq144_e1832_d_n15, eq144_e1832_d_n16, eq144_e1832_d_n17, eq144_e1832_d_n18, eq144_e1832_d_n19, eq144_e1832_d_n20, eq144_e1832_d_n21, eq144_e1832_d_n22];
        let eq144_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[16]),
            Some(nodes[15]),
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
        let (eq145_e1843, eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
        let eq145_e1840: f64 = self.eval_ddt(44, s.v[252]);
        let eq145_e1840_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq145_e1840_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq145_e1840_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq145_e1840_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq145_e1840_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq145_e1840_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq145_e1840_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq145_e1840_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq145_e1840_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq145_e1840_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq145_e1840_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq145_e1840_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq145_e1840_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq145_e1840_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq145_e1840_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq145_e1840_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq145_e1840_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq145_e1840_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq145_e1840_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq145_e1840_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq145_e1840_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq145_e1840_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq145_e1840_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq145_e1841: f64 = (p.p7 * eq145_e1840);
        let eq145_e1841_d_n0: f64 = (p.p7 * eq145_e1840_d_n0);
        let eq145_e1841_d_n1: f64 = (p.p7 * eq145_e1840_d_n1);
        let eq145_e1841_d_n2: f64 = (p.p7 * eq145_e1840_d_n2);
        let eq145_e1841_d_n3: f64 = (p.p7 * eq145_e1840_d_n3);
        let eq145_e1841_d_n4: f64 = (p.p7 * eq145_e1840_d_n4);
        let eq145_e1841_d_n5: f64 = (p.p7 * eq145_e1840_d_n5);
        let eq145_e1841_d_n6: f64 = (p.p7 * eq145_e1840_d_n6);
        let eq145_e1841_d_n7: f64 = (p.p7 * eq145_e1840_d_n7);
        let eq145_e1841_d_n8: f64 = (p.p7 * eq145_e1840_d_n8);
        let eq145_e1841_d_n9: f64 = (p.p7 * eq145_e1840_d_n9);
        let eq145_e1841_d_n10: f64 = (p.p7 * eq145_e1840_d_n10);
        let eq145_e1841_d_n11: f64 = (p.p7 * eq145_e1840_d_n11);
        let eq145_e1841_d_n12: f64 = (p.p7 * eq145_e1840_d_n12);
        let eq145_e1841_d_n13: f64 = (p.p7 * eq145_e1840_d_n13);
        let eq145_e1841_d_n14: f64 = (p.p7 * eq145_e1840_d_n14);
        let eq145_e1841_d_n15: f64 = (p.p7 * eq145_e1840_d_n15);
        let eq145_e1841_d_n16: f64 = (p.p7 * eq145_e1840_d_n16);
        let eq145_e1841_d_n17: f64 = (p.p7 * eq145_e1840_d_n17);
        let eq145_e1841_d_n18: f64 = (p.p7 * eq145_e1840_d_n18);
        let eq145_e1841_d_n19: f64 = (p.p7 * eq145_e1840_d_n19);
        let eq145_e1841_d_n20: f64 = (p.p7 * eq145_e1840_d_n20);
        let eq145_e1841_d_n21: f64 = (p.p7 * eq145_e1840_d_n21);
        let eq145_e1841_d_n22: f64 = (p.p7 * eq145_e1840_d_n22);
        (eq145_e1841, eq145_e1841_d_n0, eq145_e1841_d_n1, eq145_e1841_d_n2, eq145_e1841_d_n3, eq145_e1841_d_n4, eq145_e1841_d_n5, eq145_e1841_d_n6, eq145_e1841_d_n7, eq145_e1841_d_n8, eq145_e1841_d_n9, eq145_e1841_d_n10, eq145_e1841_d_n11, eq145_e1841_d_n12, eq145_e1841_d_n13, eq145_e1841_d_n14, eq145_e1841_d_n15, eq145_e1841_d_n16, eq145_e1841_d_n17, eq145_e1841_d_n18, eq145_e1841_d_n19, eq145_e1841_d_n20, eq145_e1841_d_n21, eq145_e1841_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq145_value: f64 = eq145_e1843;
        let eq145_node_derivatives: [f64; 23] = [eq145_e1843_d_n0, eq145_e1843_d_n1, eq145_e1843_d_n2, eq145_e1843_d_n3, eq145_e1843_d_n4, eq145_e1843_d_n5, eq145_e1843_d_n6, eq145_e1843_d_n7, eq145_e1843_d_n8, eq145_e1843_d_n9, eq145_e1843_d_n10, eq145_e1843_d_n11, eq145_e1843_d_n12, eq145_e1843_d_n13, eq145_e1843_d_n14, eq145_e1843_d_n15, eq145_e1843_d_n16, eq145_e1843_d_n17, eq145_e1843_d_n18, eq145_e1843_d_n19, eq145_e1843_d_n20, eq145_e1843_d_n21, eq145_e1843_d_n22];
        let eq145_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[15]),
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
        let (eq146_e1856, eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (s.v[582] != 0.0)) {
        let eq146_e1851: f64 = (p.p7 * p.p247);
        let eq146_e1853: f64 = self.eval_ddt(45, s.v[252]);
        let eq146_e1853_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq146_e1853_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq146_e1853_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq146_e1853_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq146_e1853_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq146_e1853_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq146_e1853_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq146_e1853_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq146_e1853_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq146_e1853_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq146_e1853_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq146_e1853_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq146_e1853_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq146_e1853_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq146_e1853_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq146_e1853_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq146_e1853_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq146_e1853_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq146_e1853_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq146_e1853_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq146_e1853_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq146_e1853_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq146_e1853_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq146_e1854: f64 = (eq146_e1851 * eq146_e1853);
        let eq146_e1854_d_n0: f64 = (eq146_e1851 * eq146_e1853_d_n0);
        let eq146_e1854_d_n1: f64 = (eq146_e1851 * eq146_e1853_d_n1);
        let eq146_e1854_d_n2: f64 = (eq146_e1851 * eq146_e1853_d_n2);
        let eq146_e1854_d_n3: f64 = (eq146_e1851 * eq146_e1853_d_n3);
        let eq146_e1854_d_n4: f64 = (eq146_e1851 * eq146_e1853_d_n4);
        let eq146_e1854_d_n5: f64 = (eq146_e1851 * eq146_e1853_d_n5);
        let eq146_e1854_d_n6: f64 = (eq146_e1851 * eq146_e1853_d_n6);
        let eq146_e1854_d_n7: f64 = (eq146_e1851 * eq146_e1853_d_n7);
        let eq146_e1854_d_n8: f64 = (eq146_e1851 * eq146_e1853_d_n8);
        let eq146_e1854_d_n9: f64 = (eq146_e1851 * eq146_e1853_d_n9);
        let eq146_e1854_d_n10: f64 = (eq146_e1851 * eq146_e1853_d_n10);
        let eq146_e1854_d_n11: f64 = (eq146_e1851 * eq146_e1853_d_n11);
        let eq146_e1854_d_n12: f64 = (eq146_e1851 * eq146_e1853_d_n12);
        let eq146_e1854_d_n13: f64 = (eq146_e1851 * eq146_e1853_d_n13);
        let eq146_e1854_d_n14: f64 = (eq146_e1851 * eq146_e1853_d_n14);
        let eq146_e1854_d_n15: f64 = (eq146_e1851 * eq146_e1853_d_n15);
        let eq146_e1854_d_n16: f64 = (eq146_e1851 * eq146_e1853_d_n16);
        let eq146_e1854_d_n17: f64 = (eq146_e1851 * eq146_e1853_d_n17);
        let eq146_e1854_d_n18: f64 = (eq146_e1851 * eq146_e1853_d_n18);
        let eq146_e1854_d_n19: f64 = (eq146_e1851 * eq146_e1853_d_n19);
        let eq146_e1854_d_n20: f64 = (eq146_e1851 * eq146_e1853_d_n20);
        let eq146_e1854_d_n21: f64 = (eq146_e1851 * eq146_e1853_d_n21);
        let eq146_e1854_d_n22: f64 = (eq146_e1851 * eq146_e1853_d_n22);
        (eq146_e1854, eq146_e1854_d_n0, eq146_e1854_d_n1, eq146_e1854_d_n2, eq146_e1854_d_n3, eq146_e1854_d_n4, eq146_e1854_d_n5, eq146_e1854_d_n6, eq146_e1854_d_n7, eq146_e1854_d_n8, eq146_e1854_d_n9, eq146_e1854_d_n10, eq146_e1854_d_n11, eq146_e1854_d_n12, eq146_e1854_d_n13, eq146_e1854_d_n14, eq146_e1854_d_n15, eq146_e1854_d_n16, eq146_e1854_d_n17, eq146_e1854_d_n18, eq146_e1854_d_n19, eq146_e1854_d_n20, eq146_e1854_d_n21, eq146_e1854_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq146_value: f64 = eq146_e1856;
        let eq146_node_derivatives: [f64; 23] = [eq146_e1856_d_n0, eq146_e1856_d_n1, eq146_e1856_d_n2, eq146_e1856_d_n3, eq146_e1856_d_n4, eq146_e1856_d_n5, eq146_e1856_d_n6, eq146_e1856_d_n7, eq146_e1856_d_n8, eq146_e1856_d_n9, eq146_e1856_d_n10, eq146_e1856_d_n11, eq146_e1856_d_n12, eq146_e1856_d_n13, eq146_e1856_d_n14, eq146_e1856_d_n15, eq146_e1856_d_n16, eq146_e1856_d_n17, eq146_e1856_d_n18, eq146_e1856_d_n19, eq146_e1856_d_n20, eq146_e1856_d_n21, eq146_e1856_d_n22];
        let eq146_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
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
        let (eq147_e1868, eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
        let eq147_e1865: f64 = self.eval_ddt(46, s.v[252]);
        let eq147_e1865_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq147_e1865_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq147_e1865_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq147_e1865_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq147_e1865_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq147_e1865_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq147_e1865_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq147_e1865_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq147_e1865_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq147_e1865_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq147_e1865_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq147_e1865_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq147_e1865_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq147_e1865_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq147_e1865_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq147_e1865_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq147_e1865_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq147_e1865_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq147_e1865_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq147_e1865_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq147_e1865_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq147_e1865_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq147_e1865_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq147_e1866: f64 = (p.p7 * eq147_e1865);
        let eq147_e1866_d_n0: f64 = (p.p7 * eq147_e1865_d_n0);
        let eq147_e1866_d_n1: f64 = (p.p7 * eq147_e1865_d_n1);
        let eq147_e1866_d_n2: f64 = (p.p7 * eq147_e1865_d_n2);
        let eq147_e1866_d_n3: f64 = (p.p7 * eq147_e1865_d_n3);
        let eq147_e1866_d_n4: f64 = (p.p7 * eq147_e1865_d_n4);
        let eq147_e1866_d_n5: f64 = (p.p7 * eq147_e1865_d_n5);
        let eq147_e1866_d_n6: f64 = (p.p7 * eq147_e1865_d_n6);
        let eq147_e1866_d_n7: f64 = (p.p7 * eq147_e1865_d_n7);
        let eq147_e1866_d_n8: f64 = (p.p7 * eq147_e1865_d_n8);
        let eq147_e1866_d_n9: f64 = (p.p7 * eq147_e1865_d_n9);
        let eq147_e1866_d_n10: f64 = (p.p7 * eq147_e1865_d_n10);
        let eq147_e1866_d_n11: f64 = (p.p7 * eq147_e1865_d_n11);
        let eq147_e1866_d_n12: f64 = (p.p7 * eq147_e1865_d_n12);
        let eq147_e1866_d_n13: f64 = (p.p7 * eq147_e1865_d_n13);
        let eq147_e1866_d_n14: f64 = (p.p7 * eq147_e1865_d_n14);
        let eq147_e1866_d_n15: f64 = (p.p7 * eq147_e1865_d_n15);
        let eq147_e1866_d_n16: f64 = (p.p7 * eq147_e1865_d_n16);
        let eq147_e1866_d_n17: f64 = (p.p7 * eq147_e1865_d_n17);
        let eq147_e1866_d_n18: f64 = (p.p7 * eq147_e1865_d_n18);
        let eq147_e1866_d_n19: f64 = (p.p7 * eq147_e1865_d_n19);
        let eq147_e1866_d_n20: f64 = (p.p7 * eq147_e1865_d_n20);
        let eq147_e1866_d_n21: f64 = (p.p7 * eq147_e1865_d_n21);
        let eq147_e1866_d_n22: f64 = (p.p7 * eq147_e1865_d_n22);
        (eq147_e1866, eq147_e1866_d_n0, eq147_e1866_d_n1, eq147_e1866_d_n2, eq147_e1866_d_n3, eq147_e1866_d_n4, eq147_e1866_d_n5, eq147_e1866_d_n6, eq147_e1866_d_n7, eq147_e1866_d_n8, eq147_e1866_d_n9, eq147_e1866_d_n10, eq147_e1866_d_n11, eq147_e1866_d_n12, eq147_e1866_d_n13, eq147_e1866_d_n14, eq147_e1866_d_n15, eq147_e1866_d_n16, eq147_e1866_d_n17, eq147_e1866_d_n18, eq147_e1866_d_n19, eq147_e1866_d_n20, eq147_e1866_d_n21, eq147_e1866_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1868;
        let eq147_node_derivatives: [f64; 23] = [eq147_e1868_d_n0, eq147_e1868_d_n1, eq147_e1868_d_n2, eq147_e1868_d_n3, eq147_e1868_d_n4, eq147_e1868_d_n5, eq147_e1868_d_n6, eq147_e1868_d_n7, eq147_e1868_d_n8, eq147_e1868_d_n9, eq147_e1868_d_n10, eq147_e1868_d_n11, eq147_e1868_d_n12, eq147_e1868_d_n13, eq147_e1868_d_n14, eq147_e1868_d_n15, eq147_e1868_d_n16, eq147_e1868_d_n17, eq147_e1868_d_n18, eq147_e1868_d_n19, eq147_e1868_d_n20, eq147_e1868_d_n21, eq147_e1868_d_n22];
        let eq147_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[15]),
            self.multiplicity * (eq147_value),
            &nodes,
            &eq147_node_derivatives,
            &branches,
            &eq147_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_148_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq148_e1882, eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22,) = {
    if (((s.v[580] != 0.0) && (s.v[581] != 0.0)) && (!(s.v[582] != 0.0))) {
        let eq148_e1877: f64 = (p.p7 * p.p247);
        let eq148_e1879: f64 = self.eval_ddt(47, s.v[252]);
        let eq148_e1879_d_n0: f64 = self.ddt_jacobian(s.dn[252][0]);
        let eq148_e1879_d_n1: f64 = self.ddt_jacobian(s.dn[252][1]);
        let eq148_e1879_d_n2: f64 = self.ddt_jacobian(s.dn[252][2]);
        let eq148_e1879_d_n3: f64 = self.ddt_jacobian(s.dn[252][3]);
        let eq148_e1879_d_n4: f64 = self.ddt_jacobian(s.dn[252][4]);
        let eq148_e1879_d_n5: f64 = self.ddt_jacobian(s.dn[252][5]);
        let eq148_e1879_d_n6: f64 = self.ddt_jacobian(s.dn[252][6]);
        let eq148_e1879_d_n7: f64 = self.ddt_jacobian(s.dn[252][7]);
        let eq148_e1879_d_n8: f64 = self.ddt_jacobian(s.dn[252][8]);
        let eq148_e1879_d_n9: f64 = self.ddt_jacobian(s.dn[252][9]);
        let eq148_e1879_d_n10: f64 = self.ddt_jacobian(s.dn[252][10]);
        let eq148_e1879_d_n11: f64 = self.ddt_jacobian(s.dn[252][11]);
        let eq148_e1879_d_n12: f64 = self.ddt_jacobian(s.dn[252][12]);
        let eq148_e1879_d_n13: f64 = self.ddt_jacobian(s.dn[252][13]);
        let eq148_e1879_d_n14: f64 = self.ddt_jacobian(s.dn[252][14]);
        let eq148_e1879_d_n15: f64 = self.ddt_jacobian(s.dn[252][15]);
        let eq148_e1879_d_n16: f64 = self.ddt_jacobian(s.dn[252][16]);
        let eq148_e1879_d_n17: f64 = self.ddt_jacobian(s.dn[252][17]);
        let eq148_e1879_d_n18: f64 = self.ddt_jacobian(s.dn[252][18]);
        let eq148_e1879_d_n19: f64 = self.ddt_jacobian(s.dn[252][19]);
        let eq148_e1879_d_n20: f64 = self.ddt_jacobian(s.dn[252][20]);
        let eq148_e1879_d_n21: f64 = self.ddt_jacobian(s.dn[252][21]);
        let eq148_e1879_d_n22: f64 = self.ddt_jacobian(s.dn[252][22]);
        let eq148_e1880: f64 = (eq148_e1877 * eq148_e1879);
        let eq148_e1880_d_n0: f64 = (eq148_e1877 * eq148_e1879_d_n0);
        let eq148_e1880_d_n1: f64 = (eq148_e1877 * eq148_e1879_d_n1);
        let eq148_e1880_d_n2: f64 = (eq148_e1877 * eq148_e1879_d_n2);
        let eq148_e1880_d_n3: f64 = (eq148_e1877 * eq148_e1879_d_n3);
        let eq148_e1880_d_n4: f64 = (eq148_e1877 * eq148_e1879_d_n4);
        let eq148_e1880_d_n5: f64 = (eq148_e1877 * eq148_e1879_d_n5);
        let eq148_e1880_d_n6: f64 = (eq148_e1877 * eq148_e1879_d_n6);
        let eq148_e1880_d_n7: f64 = (eq148_e1877 * eq148_e1879_d_n7);
        let eq148_e1880_d_n8: f64 = (eq148_e1877 * eq148_e1879_d_n8);
        let eq148_e1880_d_n9: f64 = (eq148_e1877 * eq148_e1879_d_n9);
        let eq148_e1880_d_n10: f64 = (eq148_e1877 * eq148_e1879_d_n10);
        let eq148_e1880_d_n11: f64 = (eq148_e1877 * eq148_e1879_d_n11);
        let eq148_e1880_d_n12: f64 = (eq148_e1877 * eq148_e1879_d_n12);
        let eq148_e1880_d_n13: f64 = (eq148_e1877 * eq148_e1879_d_n13);
        let eq148_e1880_d_n14: f64 = (eq148_e1877 * eq148_e1879_d_n14);
        let eq148_e1880_d_n15: f64 = (eq148_e1877 * eq148_e1879_d_n15);
        let eq148_e1880_d_n16: f64 = (eq148_e1877 * eq148_e1879_d_n16);
        let eq148_e1880_d_n17: f64 = (eq148_e1877 * eq148_e1879_d_n17);
        let eq148_e1880_d_n18: f64 = (eq148_e1877 * eq148_e1879_d_n18);
        let eq148_e1880_d_n19: f64 = (eq148_e1877 * eq148_e1879_d_n19);
        let eq148_e1880_d_n20: f64 = (eq148_e1877 * eq148_e1879_d_n20);
        let eq148_e1880_d_n21: f64 = (eq148_e1877 * eq148_e1879_d_n21);
        let eq148_e1880_d_n22: f64 = (eq148_e1877 * eq148_e1879_d_n22);
        (eq148_e1880, eq148_e1880_d_n0, eq148_e1880_d_n1, eq148_e1880_d_n2, eq148_e1880_d_n3, eq148_e1880_d_n4, eq148_e1880_d_n5, eq148_e1880_d_n6, eq148_e1880_d_n7, eq148_e1880_d_n8, eq148_e1880_d_n9, eq148_e1880_d_n10, eq148_e1880_d_n11, eq148_e1880_d_n12, eq148_e1880_d_n13, eq148_e1880_d_n14, eq148_e1880_d_n15, eq148_e1880_d_n16, eq148_e1880_d_n17, eq148_e1880_d_n18, eq148_e1880_d_n19, eq148_e1880_d_n20, eq148_e1880_d_n21, eq148_e1880_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1882;
        let eq148_node_derivatives: [f64; 23] = [eq148_e1882_d_n0, eq148_e1882_d_n1, eq148_e1882_d_n2, eq148_e1882_d_n3, eq148_e1882_d_n4, eq148_e1882_d_n5, eq148_e1882_d_n6, eq148_e1882_d_n7, eq148_e1882_d_n8, eq148_e1882_d_n9, eq148_e1882_d_n10, eq148_e1882_d_n11, eq148_e1882_d_n12, eq148_e1882_d_n13, eq148_e1882_d_n14, eq148_e1882_d_n15, eq148_e1882_d_n16, eq148_e1882_d_n17, eq148_e1882_d_n18, eq148_e1882_d_n19, eq148_e1882_d_n20, eq148_e1882_d_n21, eq148_e1882_d_n22];
        let eq148_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[15]),
            self.multiplicity * (eq148_value),
            &nodes,
            &eq148_node_derivatives,
            &branches,
            &eq148_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_149_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq149_e1893, eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22,) = {
    if ((s.v[580] != 0.0) && (s.v[581] != 0.0)) {
        let eq149_e1889: f64 = (p.p252 * s.v[252]);
        let eq149_e1889_d_n0: f64 = (p.p252 * s.dn[252][0]);
        let eq149_e1889_d_n1: f64 = (p.p252 * s.dn[252][1]);
        let eq149_e1889_d_n2: f64 = (p.p252 * s.dn[252][2]);
        let eq149_e1889_d_n3: f64 = (p.p252 * s.dn[252][3]);
        let eq149_e1889_d_n4: f64 = (p.p252 * s.dn[252][4]);
        let eq149_e1889_d_n5: f64 = (p.p252 * s.dn[252][5]);
        let eq149_e1889_d_n6: f64 = (p.p252 * s.dn[252][6]);
        let eq149_e1889_d_n7: f64 = (p.p252 * s.dn[252][7]);
        let eq149_e1889_d_n8: f64 = (p.p252 * s.dn[252][8]);
        let eq149_e1889_d_n9: f64 = (p.p252 * s.dn[252][9]);
        let eq149_e1889_d_n10: f64 = (p.p252 * s.dn[252][10]);
        let eq149_e1889_d_n11: f64 = (p.p252 * s.dn[252][11]);
        let eq149_e1889_d_n12: f64 = (p.p252 * s.dn[252][12]);
        let eq149_e1889_d_n13: f64 = (p.p252 * s.dn[252][13]);
        let eq149_e1889_d_n14: f64 = (p.p252 * s.dn[252][14]);
        let eq149_e1889_d_n15: f64 = (p.p252 * s.dn[252][15]);
        let eq149_e1889_d_n16: f64 = (p.p252 * s.dn[252][16]);
        let eq149_e1889_d_n17: f64 = (p.p252 * s.dn[252][17]);
        let eq149_e1889_d_n18: f64 = (p.p252 * s.dn[252][18]);
        let eq149_e1889_d_n19: f64 = (p.p252 * s.dn[252][19]);
        let eq149_e1889_d_n20: f64 = (p.p252 * s.dn[252][20]);
        let eq149_e1889_d_n21: f64 = (p.p252 * s.dn[252][21]);
        let eq149_e1889_d_n22: f64 = (p.p252 * s.dn[252][22]);
        let eq149_e1890: f64 = self.eval_ddt(48, eq149_e1889);
        let eq149_e1890_d_n0: f64 = self.ddt_jacobian(eq149_e1889_d_n0);
        let eq149_e1890_d_n1: f64 = self.ddt_jacobian(eq149_e1889_d_n1);
        let eq149_e1890_d_n2: f64 = self.ddt_jacobian(eq149_e1889_d_n2);
        let eq149_e1890_d_n3: f64 = self.ddt_jacobian(eq149_e1889_d_n3);
        let eq149_e1890_d_n4: f64 = self.ddt_jacobian(eq149_e1889_d_n4);
        let eq149_e1890_d_n5: f64 = self.ddt_jacobian(eq149_e1889_d_n5);
        let eq149_e1890_d_n6: f64 = self.ddt_jacobian(eq149_e1889_d_n6);
        let eq149_e1890_d_n7: f64 = self.ddt_jacobian(eq149_e1889_d_n7);
        let eq149_e1890_d_n8: f64 = self.ddt_jacobian(eq149_e1889_d_n8);
        let eq149_e1890_d_n9: f64 = self.ddt_jacobian(eq149_e1889_d_n9);
        let eq149_e1890_d_n10: f64 = self.ddt_jacobian(eq149_e1889_d_n10);
        let eq149_e1890_d_n11: f64 = self.ddt_jacobian(eq149_e1889_d_n11);
        let eq149_e1890_d_n12: f64 = self.ddt_jacobian(eq149_e1889_d_n12);
        let eq149_e1890_d_n13: f64 = self.ddt_jacobian(eq149_e1889_d_n13);
        let eq149_e1890_d_n14: f64 = self.ddt_jacobian(eq149_e1889_d_n14);
        let eq149_e1890_d_n15: f64 = self.ddt_jacobian(eq149_e1889_d_n15);
        let eq149_e1890_d_n16: f64 = self.ddt_jacobian(eq149_e1889_d_n16);
        let eq149_e1890_d_n17: f64 = self.ddt_jacobian(eq149_e1889_d_n17);
        let eq149_e1890_d_n18: f64 = self.ddt_jacobian(eq149_e1889_d_n18);
        let eq149_e1890_d_n19: f64 = self.ddt_jacobian(eq149_e1889_d_n19);
        let eq149_e1890_d_n20: f64 = self.ddt_jacobian(eq149_e1889_d_n20);
        let eq149_e1890_d_n21: f64 = self.ddt_jacobian(eq149_e1889_d_n21);
        let eq149_e1890_d_n22: f64 = self.ddt_jacobian(eq149_e1889_d_n22);
        let eq149_e1891: f64 = (p.p7 * eq149_e1890);
        let eq149_e1891_d_n0: f64 = (p.p7 * eq149_e1890_d_n0);
        let eq149_e1891_d_n1: f64 = (p.p7 * eq149_e1890_d_n1);
        let eq149_e1891_d_n2: f64 = (p.p7 * eq149_e1890_d_n2);
        let eq149_e1891_d_n3: f64 = (p.p7 * eq149_e1890_d_n3);
        let eq149_e1891_d_n4: f64 = (p.p7 * eq149_e1890_d_n4);
        let eq149_e1891_d_n5: f64 = (p.p7 * eq149_e1890_d_n5);
        let eq149_e1891_d_n6: f64 = (p.p7 * eq149_e1890_d_n6);
        let eq149_e1891_d_n7: f64 = (p.p7 * eq149_e1890_d_n7);
        let eq149_e1891_d_n8: f64 = (p.p7 * eq149_e1890_d_n8);
        let eq149_e1891_d_n9: f64 = (p.p7 * eq149_e1890_d_n9);
        let eq149_e1891_d_n10: f64 = (p.p7 * eq149_e1890_d_n10);
        let eq149_e1891_d_n11: f64 = (p.p7 * eq149_e1890_d_n11);
        let eq149_e1891_d_n12: f64 = (p.p7 * eq149_e1890_d_n12);
        let eq149_e1891_d_n13: f64 = (p.p7 * eq149_e1890_d_n13);
        let eq149_e1891_d_n14: f64 = (p.p7 * eq149_e1890_d_n14);
        let eq149_e1891_d_n15: f64 = (p.p7 * eq149_e1890_d_n15);
        let eq149_e1891_d_n16: f64 = (p.p7 * eq149_e1890_d_n16);
        let eq149_e1891_d_n17: f64 = (p.p7 * eq149_e1890_d_n17);
        let eq149_e1891_d_n18: f64 = (p.p7 * eq149_e1890_d_n18);
        let eq149_e1891_d_n19: f64 = (p.p7 * eq149_e1890_d_n19);
        let eq149_e1891_d_n20: f64 = (p.p7 * eq149_e1890_d_n20);
        let eq149_e1891_d_n21: f64 = (p.p7 * eq149_e1890_d_n21);
        let eq149_e1891_d_n22: f64 = (p.p7 * eq149_e1890_d_n22);
        (eq149_e1891, eq149_e1891_d_n0, eq149_e1891_d_n1, eq149_e1891_d_n2, eq149_e1891_d_n3, eq149_e1891_d_n4, eq149_e1891_d_n5, eq149_e1891_d_n6, eq149_e1891_d_n7, eq149_e1891_d_n8, eq149_e1891_d_n9, eq149_e1891_d_n10, eq149_e1891_d_n11, eq149_e1891_d_n12, eq149_e1891_d_n13, eq149_e1891_d_n14, eq149_e1891_d_n15, eq149_e1891_d_n16, eq149_e1891_d_n17, eq149_e1891_d_n18, eq149_e1891_d_n19, eq149_e1891_d_n20, eq149_e1891_d_n21, eq149_e1891_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1893;
        let eq149_node_derivatives: [f64; 23] = [eq149_e1893_d_n0, eq149_e1893_d_n1, eq149_e1893_d_n2, eq149_e1893_d_n3, eq149_e1893_d_n4, eq149_e1893_d_n5, eq149_e1893_d_n6, eq149_e1893_d_n7, eq149_e1893_d_n8, eq149_e1893_d_n9, eq149_e1893_d_n10, eq149_e1893_d_n11, eq149_e1893_d_n12, eq149_e1893_d_n13, eq149_e1893_d_n14, eq149_e1893_d_n15, eq149_e1893_d_n16, eq149_e1893_d_n17, eq149_e1893_d_n18, eq149_e1893_d_n19, eq149_e1893_d_n20, eq149_e1893_d_n21, eq149_e1893_d_n22];
        let eq149_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[15]),
            self.multiplicity * (eq149_value),
            &nodes,
            &eq149_node_derivatives,
            &branches,
            &eq149_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_150_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq150_e1903, eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22,) = {
    if ((!(s.v[580] != 0.0)) && (s.v[583] != 0.0)) {
        let eq150_e1900: f64 = self.eval_ddt(49, s.v[253]);
        let eq150_e1900_d_n0: f64 = self.ddt_jacobian(s.dn[253][0]);
        let eq150_e1900_d_n1: f64 = self.ddt_jacobian(s.dn[253][1]);
        let eq150_e1900_d_n2: f64 = self.ddt_jacobian(s.dn[253][2]);
        let eq150_e1900_d_n3: f64 = self.ddt_jacobian(s.dn[253][3]);
        let eq150_e1900_d_n4: f64 = self.ddt_jacobian(s.dn[253][4]);
        let eq150_e1900_d_n5: f64 = self.ddt_jacobian(s.dn[253][5]);
        let eq150_e1900_d_n6: f64 = self.ddt_jacobian(s.dn[253][6]);
        let eq150_e1900_d_n7: f64 = self.ddt_jacobian(s.dn[253][7]);
        let eq150_e1900_d_n8: f64 = self.ddt_jacobian(s.dn[253][8]);
        let eq150_e1900_d_n9: f64 = self.ddt_jacobian(s.dn[253][9]);
        let eq150_e1900_d_n10: f64 = self.ddt_jacobian(s.dn[253][10]);
        let eq150_e1900_d_n11: f64 = self.ddt_jacobian(s.dn[253][11]);
        let eq150_e1900_d_n12: f64 = self.ddt_jacobian(s.dn[253][12]);
        let eq150_e1900_d_n13: f64 = self.ddt_jacobian(s.dn[253][13]);
        let eq150_e1900_d_n14: f64 = self.ddt_jacobian(s.dn[253][14]);
        let eq150_e1900_d_n15: f64 = self.ddt_jacobian(s.dn[253][15]);
        let eq150_e1900_d_n16: f64 = self.ddt_jacobian(s.dn[253][16]);
        let eq150_e1900_d_n17: f64 = self.ddt_jacobian(s.dn[253][17]);
        let eq150_e1900_d_n18: f64 = self.ddt_jacobian(s.dn[253][18]);
        let eq150_e1900_d_n19: f64 = self.ddt_jacobian(s.dn[253][19]);
        let eq150_e1900_d_n20: f64 = self.ddt_jacobian(s.dn[253][20]);
        let eq150_e1900_d_n21: f64 = self.ddt_jacobian(s.dn[253][21]);
        let eq150_e1900_d_n22: f64 = self.ddt_jacobian(s.dn[253][22]);
        let eq150_e1901: f64 = (p.p7 * eq150_e1900);
        let eq150_e1901_d_n0: f64 = (p.p7 * eq150_e1900_d_n0);
        let eq150_e1901_d_n1: f64 = (p.p7 * eq150_e1900_d_n1);
        let eq150_e1901_d_n2: f64 = (p.p7 * eq150_e1900_d_n2);
        let eq150_e1901_d_n3: f64 = (p.p7 * eq150_e1900_d_n3);
        let eq150_e1901_d_n4: f64 = (p.p7 * eq150_e1900_d_n4);
        let eq150_e1901_d_n5: f64 = (p.p7 * eq150_e1900_d_n5);
        let eq150_e1901_d_n6: f64 = (p.p7 * eq150_e1900_d_n6);
        let eq150_e1901_d_n7: f64 = (p.p7 * eq150_e1900_d_n7);
        let eq150_e1901_d_n8: f64 = (p.p7 * eq150_e1900_d_n8);
        let eq150_e1901_d_n9: f64 = (p.p7 * eq150_e1900_d_n9);
        let eq150_e1901_d_n10: f64 = (p.p7 * eq150_e1900_d_n10);
        let eq150_e1901_d_n11: f64 = (p.p7 * eq150_e1900_d_n11);
        let eq150_e1901_d_n12: f64 = (p.p7 * eq150_e1900_d_n12);
        let eq150_e1901_d_n13: f64 = (p.p7 * eq150_e1900_d_n13);
        let eq150_e1901_d_n14: f64 = (p.p7 * eq150_e1900_d_n14);
        let eq150_e1901_d_n15: f64 = (p.p7 * eq150_e1900_d_n15);
        let eq150_e1901_d_n16: f64 = (p.p7 * eq150_e1900_d_n16);
        let eq150_e1901_d_n17: f64 = (p.p7 * eq150_e1900_d_n17);
        let eq150_e1901_d_n18: f64 = (p.p7 * eq150_e1900_d_n18);
        let eq150_e1901_d_n19: f64 = (p.p7 * eq150_e1900_d_n19);
        let eq150_e1901_d_n20: f64 = (p.p7 * eq150_e1900_d_n20);
        let eq150_e1901_d_n21: f64 = (p.p7 * eq150_e1900_d_n21);
        let eq150_e1901_d_n22: f64 = (p.p7 * eq150_e1900_d_n22);
        (eq150_e1901, eq150_e1901_d_n0, eq150_e1901_d_n1, eq150_e1901_d_n2, eq150_e1901_d_n3, eq150_e1901_d_n4, eq150_e1901_d_n5, eq150_e1901_d_n6, eq150_e1901_d_n7, eq150_e1901_d_n8, eq150_e1901_d_n9, eq150_e1901_d_n10, eq150_e1901_d_n11, eq150_e1901_d_n12, eq150_e1901_d_n13, eq150_e1901_d_n14, eq150_e1901_d_n15, eq150_e1901_d_n16, eq150_e1901_d_n17, eq150_e1901_d_n18, eq150_e1901_d_n19, eq150_e1901_d_n20, eq150_e1901_d_n21, eq150_e1901_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1903;
        let eq150_node_derivatives: [f64; 23] = [eq150_e1903_d_n0, eq150_e1903_d_n1, eq150_e1903_d_n2, eq150_e1903_d_n3, eq150_e1903_d_n4, eq150_e1903_d_n5, eq150_e1903_d_n6, eq150_e1903_d_n7, eq150_e1903_d_n8, eq150_e1903_d_n9, eq150_e1903_d_n10, eq150_e1903_d_n11, eq150_e1903_d_n12, eq150_e1903_d_n13, eq150_e1903_d_n14, eq150_e1903_d_n15, eq150_e1903_d_n16, eq150_e1903_d_n17, eq150_e1903_d_n18, eq150_e1903_d_n19, eq150_e1903_d_n20, eq150_e1903_d_n21, eq150_e1903_d_n22];
        let eq150_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
            self.multiplicity * (eq150_value),
            &nodes,
            &eq150_node_derivatives,
            &branches,
            &eq150_branch_derivatives,
            self.multiplicity,
        );
    }
}

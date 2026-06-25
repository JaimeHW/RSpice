#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_equation_116_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq116_e1456, eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29,) = {
    if (!(s.v[1348] != 0.0)) {
        let eq116_e1449: f64 = self.eval_ddt(108, s.v[179]);
        let eq116_e1449_d_n0: f64 = self.ddt_jacobian(s.dn[179][0]);
        let eq116_e1449_d_n1: f64 = self.ddt_jacobian(s.dn[179][1]);
        let eq116_e1449_d_n2: f64 = self.ddt_jacobian(s.dn[179][2]);
        let eq116_e1449_d_n3: f64 = self.ddt_jacobian(s.dn[179][3]);
        let eq116_e1449_d_n4: f64 = self.ddt_jacobian(s.dn[179][4]);
        let eq116_e1449_d_n5: f64 = self.ddt_jacobian(s.dn[179][5]);
        let eq116_e1449_d_n6: f64 = self.ddt_jacobian(s.dn[179][6]);
        let eq116_e1449_d_n7: f64 = self.ddt_jacobian(s.dn[179][7]);
        let eq116_e1449_d_n8: f64 = self.ddt_jacobian(s.dn[179][8]);
        let eq116_e1449_d_n9: f64 = self.ddt_jacobian(s.dn[179][9]);
        let eq116_e1449_d_n10: f64 = self.ddt_jacobian(s.dn[179][10]);
        let eq116_e1449_d_n11: f64 = self.ddt_jacobian(s.dn[179][11]);
        let eq116_e1449_d_n12: f64 = self.ddt_jacobian(s.dn[179][12]);
        let eq116_e1449_d_n13: f64 = self.ddt_jacobian(s.dn[179][13]);
        let eq116_e1449_d_n14: f64 = self.ddt_jacobian(s.dn[179][14]);
        let eq116_e1449_d_n15: f64 = self.ddt_jacobian(s.dn[179][15]);
        let eq116_e1449_d_n16: f64 = self.ddt_jacobian(s.dn[179][16]);
        let eq116_e1449_d_n17: f64 = self.ddt_jacobian(s.dn[179][17]);
        let eq116_e1449_d_n18: f64 = self.ddt_jacobian(s.dn[179][18]);
        let eq116_e1449_d_n19: f64 = self.ddt_jacobian(s.dn[179][19]);
        let eq116_e1449_d_n20: f64 = self.ddt_jacobian(s.dn[179][20]);
        let eq116_e1449_d_n21: f64 = self.ddt_jacobian(s.dn[179][21]);
        let eq116_e1449_d_n22: f64 = self.ddt_jacobian(s.dn[179][22]);
        let eq116_e1449_d_n23: f64 = self.ddt_jacobian(s.dn[179][23]);
        let eq116_e1449_d_n24: f64 = self.ddt_jacobian(s.dn[179][24]);
        let eq116_e1449_d_n25: f64 = self.ddt_jacobian(s.dn[179][25]);
        let eq116_e1449_d_n26: f64 = self.ddt_jacobian(s.dn[179][26]);
        let eq116_e1449_d_n27: f64 = self.ddt_jacobian(s.dn[179][27]);
        let eq116_e1449_d_n28: f64 = self.ddt_jacobian(s.dn[179][28]);
        let eq116_e1449_d_n29: f64 = self.ddt_jacobian(s.dn[179][29]);
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1452_d_n2: f64 = p.p355;
        let eq116_e1452_d_n12: f64 = (-p.p355);
        let eq116_e1453: f64 = self.eval_ddt(109, eq116_e1452);
        let eq116_e1453_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n2: f64 = self.ddt_jacobian(eq116_e1452_d_n2);
        let eq116_e1453_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n12: f64 = self.ddt_jacobian(eq116_e1452_d_n12);
        let eq116_e1453_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq116_e1453_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq116_e1454: f64 = (eq116_e1449 + eq116_e1453);
        let eq116_e1454_d_n0: f64 = (eq116_e1449_d_n0 + eq116_e1453_d_n0);
        let eq116_e1454_d_n1: f64 = (eq116_e1449_d_n1 + eq116_e1453_d_n1);
        let eq116_e1454_d_n2: f64 = (eq116_e1449_d_n2 + eq116_e1453_d_n2);
        let eq116_e1454_d_n3: f64 = (eq116_e1449_d_n3 + eq116_e1453_d_n3);
        let eq116_e1454_d_n4: f64 = (eq116_e1449_d_n4 + eq116_e1453_d_n4);
        let eq116_e1454_d_n5: f64 = (eq116_e1449_d_n5 + eq116_e1453_d_n5);
        let eq116_e1454_d_n6: f64 = (eq116_e1449_d_n6 + eq116_e1453_d_n6);
        let eq116_e1454_d_n7: f64 = (eq116_e1449_d_n7 + eq116_e1453_d_n7);
        let eq116_e1454_d_n8: f64 = (eq116_e1449_d_n8 + eq116_e1453_d_n8);
        let eq116_e1454_d_n9: f64 = (eq116_e1449_d_n9 + eq116_e1453_d_n9);
        let eq116_e1454_d_n10: f64 = (eq116_e1449_d_n10 + eq116_e1453_d_n10);
        let eq116_e1454_d_n11: f64 = (eq116_e1449_d_n11 + eq116_e1453_d_n11);
        let eq116_e1454_d_n12: f64 = (eq116_e1449_d_n12 + eq116_e1453_d_n12);
        let eq116_e1454_d_n13: f64 = (eq116_e1449_d_n13 + eq116_e1453_d_n13);
        let eq116_e1454_d_n14: f64 = (eq116_e1449_d_n14 + eq116_e1453_d_n14);
        let eq116_e1454_d_n15: f64 = (eq116_e1449_d_n15 + eq116_e1453_d_n15);
        let eq116_e1454_d_n16: f64 = (eq116_e1449_d_n16 + eq116_e1453_d_n16);
        let eq116_e1454_d_n17: f64 = (eq116_e1449_d_n17 + eq116_e1453_d_n17);
        let eq116_e1454_d_n18: f64 = (eq116_e1449_d_n18 + eq116_e1453_d_n18);
        let eq116_e1454_d_n19: f64 = (eq116_e1449_d_n19 + eq116_e1453_d_n19);
        let eq116_e1454_d_n20: f64 = (eq116_e1449_d_n20 + eq116_e1453_d_n20);
        let eq116_e1454_d_n21: f64 = (eq116_e1449_d_n21 + eq116_e1453_d_n21);
        let eq116_e1454_d_n22: f64 = (eq116_e1449_d_n22 + eq116_e1453_d_n22);
        let eq116_e1454_d_n23: f64 = (eq116_e1449_d_n23 + eq116_e1453_d_n23);
        let eq116_e1454_d_n24: f64 = (eq116_e1449_d_n24 + eq116_e1453_d_n24);
        let eq116_e1454_d_n25: f64 = (eq116_e1449_d_n25 + eq116_e1453_d_n25);
        let eq116_e1454_d_n26: f64 = (eq116_e1449_d_n26 + eq116_e1453_d_n26);
        let eq116_e1454_d_n27: f64 = (eq116_e1449_d_n27 + eq116_e1453_d_n27);
        let eq116_e1454_d_n28: f64 = (eq116_e1449_d_n28 + eq116_e1453_d_n28);
        let eq116_e1454_d_n29: f64 = (eq116_e1449_d_n29 + eq116_e1453_d_n29);
        (eq116_e1454, eq116_e1454_d_n0, eq116_e1454_d_n1, eq116_e1454_d_n2, eq116_e1454_d_n3, eq116_e1454_d_n4, eq116_e1454_d_n5, eq116_e1454_d_n6, eq116_e1454_d_n7, eq116_e1454_d_n8, eq116_e1454_d_n9, eq116_e1454_d_n10, eq116_e1454_d_n11, eq116_e1454_d_n12, eq116_e1454_d_n13, eq116_e1454_d_n14, eq116_e1454_d_n15, eq116_e1454_d_n16, eq116_e1454_d_n17, eq116_e1454_d_n18, eq116_e1454_d_n19, eq116_e1454_d_n20, eq116_e1454_d_n21, eq116_e1454_d_n22, eq116_e1454_d_n23, eq116_e1454_d_n24, eq116_e1454_d_n25, eq116_e1454_d_n26, eq116_e1454_d_n27, eq116_e1454_d_n28, eq116_e1454_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_value: f64 = eq116_e1456;
        let eq116_node_derivatives: [f64; 30] = [eq116_e1456_d_n0, eq116_e1456_d_n1, eq116_e1456_d_n2, eq116_e1456_d_n3, eq116_e1456_d_n4, eq116_e1456_d_n5, eq116_e1456_d_n6, eq116_e1456_d_n7, eq116_e1456_d_n8, eq116_e1456_d_n9, eq116_e1456_d_n10, eq116_e1456_d_n11, eq116_e1456_d_n12, eq116_e1456_d_n13, eq116_e1456_d_n14, eq116_e1456_d_n15, eq116_e1456_d_n16, eq116_e1456_d_n17, eq116_e1456_d_n18, eq116_e1456_d_n19, eq116_e1456_d_n20, eq116_e1456_d_n21, eq116_e1456_d_n22, eq116_e1456_d_n23, eq116_e1456_d_n24, eq116_e1456_d_n25, eq116_e1456_d_n26, eq116_e1456_d_n27, eq116_e1456_d_n28, eq116_e1456_d_n29];
        let eq116_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            self.multiplicity * (eq116_value),
            &nodes,
            &eq116_node_derivatives,
            &branches,
            &eq116_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_117_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq117_e1467, eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29,) = {
    if (!(s.v[1348] != 0.0)) {
        let eq117_e1460: f64 = self.eval_ddt(110, s.v[180]);
        let eq117_e1460_d_n0: f64 = self.ddt_jacobian(s.dn[180][0]);
        let eq117_e1460_d_n1: f64 = self.ddt_jacobian(s.dn[180][1]);
        let eq117_e1460_d_n2: f64 = self.ddt_jacobian(s.dn[180][2]);
        let eq117_e1460_d_n3: f64 = self.ddt_jacobian(s.dn[180][3]);
        let eq117_e1460_d_n4: f64 = self.ddt_jacobian(s.dn[180][4]);
        let eq117_e1460_d_n5: f64 = self.ddt_jacobian(s.dn[180][5]);
        let eq117_e1460_d_n6: f64 = self.ddt_jacobian(s.dn[180][6]);
        let eq117_e1460_d_n7: f64 = self.ddt_jacobian(s.dn[180][7]);
        let eq117_e1460_d_n8: f64 = self.ddt_jacobian(s.dn[180][8]);
        let eq117_e1460_d_n9: f64 = self.ddt_jacobian(s.dn[180][9]);
        let eq117_e1460_d_n10: f64 = self.ddt_jacobian(s.dn[180][10]);
        let eq117_e1460_d_n11: f64 = self.ddt_jacobian(s.dn[180][11]);
        let eq117_e1460_d_n12: f64 = self.ddt_jacobian(s.dn[180][12]);
        let eq117_e1460_d_n13: f64 = self.ddt_jacobian(s.dn[180][13]);
        let eq117_e1460_d_n14: f64 = self.ddt_jacobian(s.dn[180][14]);
        let eq117_e1460_d_n15: f64 = self.ddt_jacobian(s.dn[180][15]);
        let eq117_e1460_d_n16: f64 = self.ddt_jacobian(s.dn[180][16]);
        let eq117_e1460_d_n17: f64 = self.ddt_jacobian(s.dn[180][17]);
        let eq117_e1460_d_n18: f64 = self.ddt_jacobian(s.dn[180][18]);
        let eq117_e1460_d_n19: f64 = self.ddt_jacobian(s.dn[180][19]);
        let eq117_e1460_d_n20: f64 = self.ddt_jacobian(s.dn[180][20]);
        let eq117_e1460_d_n21: f64 = self.ddt_jacobian(s.dn[180][21]);
        let eq117_e1460_d_n22: f64 = self.ddt_jacobian(s.dn[180][22]);
        let eq117_e1460_d_n23: f64 = self.ddt_jacobian(s.dn[180][23]);
        let eq117_e1460_d_n24: f64 = self.ddt_jacobian(s.dn[180][24]);
        let eq117_e1460_d_n25: f64 = self.ddt_jacobian(s.dn[180][25]);
        let eq117_e1460_d_n26: f64 = self.ddt_jacobian(s.dn[180][26]);
        let eq117_e1460_d_n27: f64 = self.ddt_jacobian(s.dn[180][27]);
        let eq117_e1460_d_n28: f64 = self.ddt_jacobian(s.dn[180][28]);
        let eq117_e1460_d_n29: f64 = self.ddt_jacobian(s.dn[180][29]);
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1463_d_n2: f64 = p.p355;
        let eq117_e1463_d_n11: f64 = (-p.p355);
        let eq117_e1464: f64 = self.eval_ddt(111, eq117_e1463);
        let eq117_e1464_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n2: f64 = self.ddt_jacobian(eq117_e1463_d_n2);
        let eq117_e1464_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n11: f64 = self.ddt_jacobian(eq117_e1463_d_n11);
        let eq117_e1464_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq117_e1464_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq117_e1465: f64 = (eq117_e1460 + eq117_e1464);
        let eq117_e1465_d_n0: f64 = (eq117_e1460_d_n0 + eq117_e1464_d_n0);
        let eq117_e1465_d_n1: f64 = (eq117_e1460_d_n1 + eq117_e1464_d_n1);
        let eq117_e1465_d_n2: f64 = (eq117_e1460_d_n2 + eq117_e1464_d_n2);
        let eq117_e1465_d_n3: f64 = (eq117_e1460_d_n3 + eq117_e1464_d_n3);
        let eq117_e1465_d_n4: f64 = (eq117_e1460_d_n4 + eq117_e1464_d_n4);
        let eq117_e1465_d_n5: f64 = (eq117_e1460_d_n5 + eq117_e1464_d_n5);
        let eq117_e1465_d_n6: f64 = (eq117_e1460_d_n6 + eq117_e1464_d_n6);
        let eq117_e1465_d_n7: f64 = (eq117_e1460_d_n7 + eq117_e1464_d_n7);
        let eq117_e1465_d_n8: f64 = (eq117_e1460_d_n8 + eq117_e1464_d_n8);
        let eq117_e1465_d_n9: f64 = (eq117_e1460_d_n9 + eq117_e1464_d_n9);
        let eq117_e1465_d_n10: f64 = (eq117_e1460_d_n10 + eq117_e1464_d_n10);
        let eq117_e1465_d_n11: f64 = (eq117_e1460_d_n11 + eq117_e1464_d_n11);
        let eq117_e1465_d_n12: f64 = (eq117_e1460_d_n12 + eq117_e1464_d_n12);
        let eq117_e1465_d_n13: f64 = (eq117_e1460_d_n13 + eq117_e1464_d_n13);
        let eq117_e1465_d_n14: f64 = (eq117_e1460_d_n14 + eq117_e1464_d_n14);
        let eq117_e1465_d_n15: f64 = (eq117_e1460_d_n15 + eq117_e1464_d_n15);
        let eq117_e1465_d_n16: f64 = (eq117_e1460_d_n16 + eq117_e1464_d_n16);
        let eq117_e1465_d_n17: f64 = (eq117_e1460_d_n17 + eq117_e1464_d_n17);
        let eq117_e1465_d_n18: f64 = (eq117_e1460_d_n18 + eq117_e1464_d_n18);
        let eq117_e1465_d_n19: f64 = (eq117_e1460_d_n19 + eq117_e1464_d_n19);
        let eq117_e1465_d_n20: f64 = (eq117_e1460_d_n20 + eq117_e1464_d_n20);
        let eq117_e1465_d_n21: f64 = (eq117_e1460_d_n21 + eq117_e1464_d_n21);
        let eq117_e1465_d_n22: f64 = (eq117_e1460_d_n22 + eq117_e1464_d_n22);
        let eq117_e1465_d_n23: f64 = (eq117_e1460_d_n23 + eq117_e1464_d_n23);
        let eq117_e1465_d_n24: f64 = (eq117_e1460_d_n24 + eq117_e1464_d_n24);
        let eq117_e1465_d_n25: f64 = (eq117_e1460_d_n25 + eq117_e1464_d_n25);
        let eq117_e1465_d_n26: f64 = (eq117_e1460_d_n26 + eq117_e1464_d_n26);
        let eq117_e1465_d_n27: f64 = (eq117_e1460_d_n27 + eq117_e1464_d_n27);
        let eq117_e1465_d_n28: f64 = (eq117_e1460_d_n28 + eq117_e1464_d_n28);
        let eq117_e1465_d_n29: f64 = (eq117_e1460_d_n29 + eq117_e1464_d_n29);
        (eq117_e1465, eq117_e1465_d_n0, eq117_e1465_d_n1, eq117_e1465_d_n2, eq117_e1465_d_n3, eq117_e1465_d_n4, eq117_e1465_d_n5, eq117_e1465_d_n6, eq117_e1465_d_n7, eq117_e1465_d_n8, eq117_e1465_d_n9, eq117_e1465_d_n10, eq117_e1465_d_n11, eq117_e1465_d_n12, eq117_e1465_d_n13, eq117_e1465_d_n14, eq117_e1465_d_n15, eq117_e1465_d_n16, eq117_e1465_d_n17, eq117_e1465_d_n18, eq117_e1465_d_n19, eq117_e1465_d_n20, eq117_e1465_d_n21, eq117_e1465_d_n22, eq117_e1465_d_n23, eq117_e1465_d_n24, eq117_e1465_d_n25, eq117_e1465_d_n26, eq117_e1465_d_n27, eq117_e1465_d_n28, eq117_e1465_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_value: f64 = eq117_e1467;
        let eq117_node_derivatives: [f64; 30] = [eq117_e1467_d_n0, eq117_e1467_d_n1, eq117_e1467_d_n2, eq117_e1467_d_n3, eq117_e1467_d_n4, eq117_e1467_d_n5, eq117_e1467_d_n6, eq117_e1467_d_n7, eq117_e1467_d_n8, eq117_e1467_d_n9, eq117_e1467_d_n10, eq117_e1467_d_n11, eq117_e1467_d_n12, eq117_e1467_d_n13, eq117_e1467_d_n14, eq117_e1467_d_n15, eq117_e1467_d_n16, eq117_e1467_d_n17, eq117_e1467_d_n18, eq117_e1467_d_n19, eq117_e1467_d_n20, eq117_e1467_d_n21, eq117_e1467_d_n22, eq117_e1467_d_n23, eq117_e1467_d_n24, eq117_e1467_d_n25, eq117_e1467_d_n26, eq117_e1467_d_n27, eq117_e1467_d_n28, eq117_e1467_d_n29];
        let eq117_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[11]),
            self.multiplicity * (eq117_value),
            &nodes,
            &eq117_node_derivatives,
            &branches,
            &eq117_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_118_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq118_e1478, eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29,) = {
    if (!(s.v[1348] != 0.0)) {
        let eq118_e1471: f64 = self.eval_ddt(112, s.v[181]);
        let eq118_e1471_d_n0: f64 = self.ddt_jacobian(s.dn[181][0]);
        let eq118_e1471_d_n1: f64 = self.ddt_jacobian(s.dn[181][1]);
        let eq118_e1471_d_n2: f64 = self.ddt_jacobian(s.dn[181][2]);
        let eq118_e1471_d_n3: f64 = self.ddt_jacobian(s.dn[181][3]);
        let eq118_e1471_d_n4: f64 = self.ddt_jacobian(s.dn[181][4]);
        let eq118_e1471_d_n5: f64 = self.ddt_jacobian(s.dn[181][5]);
        let eq118_e1471_d_n6: f64 = self.ddt_jacobian(s.dn[181][6]);
        let eq118_e1471_d_n7: f64 = self.ddt_jacobian(s.dn[181][7]);
        let eq118_e1471_d_n8: f64 = self.ddt_jacobian(s.dn[181][8]);
        let eq118_e1471_d_n9: f64 = self.ddt_jacobian(s.dn[181][9]);
        let eq118_e1471_d_n10: f64 = self.ddt_jacobian(s.dn[181][10]);
        let eq118_e1471_d_n11: f64 = self.ddt_jacobian(s.dn[181][11]);
        let eq118_e1471_d_n12: f64 = self.ddt_jacobian(s.dn[181][12]);
        let eq118_e1471_d_n13: f64 = self.ddt_jacobian(s.dn[181][13]);
        let eq118_e1471_d_n14: f64 = self.ddt_jacobian(s.dn[181][14]);
        let eq118_e1471_d_n15: f64 = self.ddt_jacobian(s.dn[181][15]);
        let eq118_e1471_d_n16: f64 = self.ddt_jacobian(s.dn[181][16]);
        let eq118_e1471_d_n17: f64 = self.ddt_jacobian(s.dn[181][17]);
        let eq118_e1471_d_n18: f64 = self.ddt_jacobian(s.dn[181][18]);
        let eq118_e1471_d_n19: f64 = self.ddt_jacobian(s.dn[181][19]);
        let eq118_e1471_d_n20: f64 = self.ddt_jacobian(s.dn[181][20]);
        let eq118_e1471_d_n21: f64 = self.ddt_jacobian(s.dn[181][21]);
        let eq118_e1471_d_n22: f64 = self.ddt_jacobian(s.dn[181][22]);
        let eq118_e1471_d_n23: f64 = self.ddt_jacobian(s.dn[181][23]);
        let eq118_e1471_d_n24: f64 = self.ddt_jacobian(s.dn[181][24]);
        let eq118_e1471_d_n25: f64 = self.ddt_jacobian(s.dn[181][25]);
        let eq118_e1471_d_n26: f64 = self.ddt_jacobian(s.dn[181][26]);
        let eq118_e1471_d_n27: f64 = self.ddt_jacobian(s.dn[181][27]);
        let eq118_e1471_d_n28: f64 = self.ddt_jacobian(s.dn[181][28]);
        let eq118_e1471_d_n29: f64 = self.ddt_jacobian(s.dn[181][29]);
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1474_d_n7: f64 = p.p355;
        let eq118_e1474_d_n12: f64 = (-p.p355);
        let eq118_e1475: f64 = self.eval_ddt(113, eq118_e1474);
        let eq118_e1475_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n7: f64 = self.ddt_jacobian(eq118_e1474_d_n7);
        let eq118_e1475_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n12: f64 = self.ddt_jacobian(eq118_e1474_d_n12);
        let eq118_e1475_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq118_e1475_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq118_e1476: f64 = (eq118_e1471 + eq118_e1475);
        let eq118_e1476_d_n0: f64 = (eq118_e1471_d_n0 + eq118_e1475_d_n0);
        let eq118_e1476_d_n1: f64 = (eq118_e1471_d_n1 + eq118_e1475_d_n1);
        let eq118_e1476_d_n2: f64 = (eq118_e1471_d_n2 + eq118_e1475_d_n2);
        let eq118_e1476_d_n3: f64 = (eq118_e1471_d_n3 + eq118_e1475_d_n3);
        let eq118_e1476_d_n4: f64 = (eq118_e1471_d_n4 + eq118_e1475_d_n4);
        let eq118_e1476_d_n5: f64 = (eq118_e1471_d_n5 + eq118_e1475_d_n5);
        let eq118_e1476_d_n6: f64 = (eq118_e1471_d_n6 + eq118_e1475_d_n6);
        let eq118_e1476_d_n7: f64 = (eq118_e1471_d_n7 + eq118_e1475_d_n7);
        let eq118_e1476_d_n8: f64 = (eq118_e1471_d_n8 + eq118_e1475_d_n8);
        let eq118_e1476_d_n9: f64 = (eq118_e1471_d_n9 + eq118_e1475_d_n9);
        let eq118_e1476_d_n10: f64 = (eq118_e1471_d_n10 + eq118_e1475_d_n10);
        let eq118_e1476_d_n11: f64 = (eq118_e1471_d_n11 + eq118_e1475_d_n11);
        let eq118_e1476_d_n12: f64 = (eq118_e1471_d_n12 + eq118_e1475_d_n12);
        let eq118_e1476_d_n13: f64 = (eq118_e1471_d_n13 + eq118_e1475_d_n13);
        let eq118_e1476_d_n14: f64 = (eq118_e1471_d_n14 + eq118_e1475_d_n14);
        let eq118_e1476_d_n15: f64 = (eq118_e1471_d_n15 + eq118_e1475_d_n15);
        let eq118_e1476_d_n16: f64 = (eq118_e1471_d_n16 + eq118_e1475_d_n16);
        let eq118_e1476_d_n17: f64 = (eq118_e1471_d_n17 + eq118_e1475_d_n17);
        let eq118_e1476_d_n18: f64 = (eq118_e1471_d_n18 + eq118_e1475_d_n18);
        let eq118_e1476_d_n19: f64 = (eq118_e1471_d_n19 + eq118_e1475_d_n19);
        let eq118_e1476_d_n20: f64 = (eq118_e1471_d_n20 + eq118_e1475_d_n20);
        let eq118_e1476_d_n21: f64 = (eq118_e1471_d_n21 + eq118_e1475_d_n21);
        let eq118_e1476_d_n22: f64 = (eq118_e1471_d_n22 + eq118_e1475_d_n22);
        let eq118_e1476_d_n23: f64 = (eq118_e1471_d_n23 + eq118_e1475_d_n23);
        let eq118_e1476_d_n24: f64 = (eq118_e1471_d_n24 + eq118_e1475_d_n24);
        let eq118_e1476_d_n25: f64 = (eq118_e1471_d_n25 + eq118_e1475_d_n25);
        let eq118_e1476_d_n26: f64 = (eq118_e1471_d_n26 + eq118_e1475_d_n26);
        let eq118_e1476_d_n27: f64 = (eq118_e1471_d_n27 + eq118_e1475_d_n27);
        let eq118_e1476_d_n28: f64 = (eq118_e1471_d_n28 + eq118_e1475_d_n28);
        let eq118_e1476_d_n29: f64 = (eq118_e1471_d_n29 + eq118_e1475_d_n29);
        (eq118_e1476, eq118_e1476_d_n0, eq118_e1476_d_n1, eq118_e1476_d_n2, eq118_e1476_d_n3, eq118_e1476_d_n4, eq118_e1476_d_n5, eq118_e1476_d_n6, eq118_e1476_d_n7, eq118_e1476_d_n8, eq118_e1476_d_n9, eq118_e1476_d_n10, eq118_e1476_d_n11, eq118_e1476_d_n12, eq118_e1476_d_n13, eq118_e1476_d_n14, eq118_e1476_d_n15, eq118_e1476_d_n16, eq118_e1476_d_n17, eq118_e1476_d_n18, eq118_e1476_d_n19, eq118_e1476_d_n20, eq118_e1476_d_n21, eq118_e1476_d_n22, eq118_e1476_d_n23, eq118_e1476_d_n24, eq118_e1476_d_n25, eq118_e1476_d_n26, eq118_e1476_d_n27, eq118_e1476_d_n28, eq118_e1476_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_value: f64 = eq118_e1478;
        let eq118_node_derivatives: [f64; 30] = [eq118_e1478_d_n0, eq118_e1478_d_n1, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n5, eq118_e1478_d_n6, eq118_e1478_d_n7, eq118_e1478_d_n8, eq118_e1478_d_n9, eq118_e1478_d_n10, eq118_e1478_d_n11, eq118_e1478_d_n12, eq118_e1478_d_n13, eq118_e1478_d_n14, eq118_e1478_d_n15, eq118_e1478_d_n16, eq118_e1478_d_n17, eq118_e1478_d_n18, eq118_e1478_d_n19, eq118_e1478_d_n20, eq118_e1478_d_n21, eq118_e1478_d_n22, eq118_e1478_d_n23, eq118_e1478_d_n24, eq118_e1478_d_n25, eq118_e1478_d_n26, eq118_e1478_d_n27, eq118_e1478_d_n28, eq118_e1478_d_n29];
        let eq118_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            self.multiplicity * (eq118_value),
            &nodes,
            &eq118_node_derivatives,
            &branches,
            &eq118_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_119_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq119_e1483,) = {
    if (!(s.v[1348] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq119_value: f64 = eq119_e1483;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[11]),
            self.multiplicity * (eq119_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_120_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq120_e1488,) = {
    if (!(s.v[1348] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq120_value: f64 = eq120_e1488;
        stamper.stamp_current(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq120_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_121_block_0(
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
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq121_e1490: f64 = self.eval_ddt(114, s.v[182]);
        let eq121_e1490_d_n0: f64 = self.ddt_jacobian(s.dn[182][0]);
        let eq121_e1490_d_n1: f64 = self.ddt_jacobian(s.dn[182][1]);
        let eq121_e1490_d_n2: f64 = self.ddt_jacobian(s.dn[182][2]);
        let eq121_e1490_d_n3: f64 = self.ddt_jacobian(s.dn[182][3]);
        let eq121_e1490_d_n4: f64 = self.ddt_jacobian(s.dn[182][4]);
        let eq121_e1490_d_n5: f64 = self.ddt_jacobian(s.dn[182][5]);
        let eq121_e1490_d_n6: f64 = self.ddt_jacobian(s.dn[182][6]);
        let eq121_e1490_d_n7: f64 = self.ddt_jacobian(s.dn[182][7]);
        let eq121_e1490_d_n8: f64 = self.ddt_jacobian(s.dn[182][8]);
        let eq121_e1490_d_n9: f64 = self.ddt_jacobian(s.dn[182][9]);
        let eq121_e1490_d_n10: f64 = self.ddt_jacobian(s.dn[182][10]);
        let eq121_e1490_d_n11: f64 = self.ddt_jacobian(s.dn[182][11]);
        let eq121_e1490_d_n12: f64 = self.ddt_jacobian(s.dn[182][12]);
        let eq121_e1490_d_n13: f64 = self.ddt_jacobian(s.dn[182][13]);
        let eq121_e1490_d_n14: f64 = self.ddt_jacobian(s.dn[182][14]);
        let eq121_e1490_d_n15: f64 = self.ddt_jacobian(s.dn[182][15]);
        let eq121_e1490_d_n16: f64 = self.ddt_jacobian(s.dn[182][16]);
        let eq121_e1490_d_n17: f64 = self.ddt_jacobian(s.dn[182][17]);
        let eq121_e1490_d_n18: f64 = self.ddt_jacobian(s.dn[182][18]);
        let eq121_e1490_d_n19: f64 = self.ddt_jacobian(s.dn[182][19]);
        let eq121_e1490_d_n20: f64 = self.ddt_jacobian(s.dn[182][20]);
        let eq121_e1490_d_n21: f64 = self.ddt_jacobian(s.dn[182][21]);
        let eq121_e1490_d_n22: f64 = self.ddt_jacobian(s.dn[182][22]);
        let eq121_e1490_d_n23: f64 = self.ddt_jacobian(s.dn[182][23]);
        let eq121_e1490_d_n24: f64 = self.ddt_jacobian(s.dn[182][24]);
        let eq121_e1490_d_n25: f64 = self.ddt_jacobian(s.dn[182][25]);
        let eq121_e1490_d_n26: f64 = self.ddt_jacobian(s.dn[182][26]);
        let eq121_e1490_d_n27: f64 = self.ddt_jacobian(s.dn[182][27]);
        let eq121_e1490_d_n28: f64 = self.ddt_jacobian(s.dn[182][28]);
        let eq121_e1490_d_n29: f64 = self.ddt_jacobian(s.dn[182][29]);
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1493_d_n3: f64 = p.p355;
        let eq121_e1493_d_n12: f64 = (-p.p355);
        let eq121_e1494: f64 = self.eval_ddt(115, eq121_e1493);
        let eq121_e1494_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n3: f64 = self.ddt_jacobian(eq121_e1493_d_n3);
        let eq121_e1494_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n12: f64 = self.ddt_jacobian(eq121_e1493_d_n12);
        let eq121_e1494_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq121_e1494_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq121_e1495: f64 = (eq121_e1490 + eq121_e1494);
        let eq121_e1495_d_n0: f64 = (eq121_e1490_d_n0 + eq121_e1494_d_n0);
        let eq121_e1495_d_n1: f64 = (eq121_e1490_d_n1 + eq121_e1494_d_n1);
        let eq121_e1495_d_n2: f64 = (eq121_e1490_d_n2 + eq121_e1494_d_n2);
        let eq121_e1495_d_n3: f64 = (eq121_e1490_d_n3 + eq121_e1494_d_n3);
        let eq121_e1495_d_n4: f64 = (eq121_e1490_d_n4 + eq121_e1494_d_n4);
        let eq121_e1495_d_n5: f64 = (eq121_e1490_d_n5 + eq121_e1494_d_n5);
        let eq121_e1495_d_n6: f64 = (eq121_e1490_d_n6 + eq121_e1494_d_n6);
        let eq121_e1495_d_n7: f64 = (eq121_e1490_d_n7 + eq121_e1494_d_n7);
        let eq121_e1495_d_n8: f64 = (eq121_e1490_d_n8 + eq121_e1494_d_n8);
        let eq121_e1495_d_n9: f64 = (eq121_e1490_d_n9 + eq121_e1494_d_n9);
        let eq121_e1495_d_n10: f64 = (eq121_e1490_d_n10 + eq121_e1494_d_n10);
        let eq121_e1495_d_n11: f64 = (eq121_e1490_d_n11 + eq121_e1494_d_n11);
        let eq121_e1495_d_n12: f64 = (eq121_e1490_d_n12 + eq121_e1494_d_n12);
        let eq121_e1495_d_n13: f64 = (eq121_e1490_d_n13 + eq121_e1494_d_n13);
        let eq121_e1495_d_n14: f64 = (eq121_e1490_d_n14 + eq121_e1494_d_n14);
        let eq121_e1495_d_n15: f64 = (eq121_e1490_d_n15 + eq121_e1494_d_n15);
        let eq121_e1495_d_n16: f64 = (eq121_e1490_d_n16 + eq121_e1494_d_n16);
        let eq121_e1495_d_n17: f64 = (eq121_e1490_d_n17 + eq121_e1494_d_n17);
        let eq121_e1495_d_n18: f64 = (eq121_e1490_d_n18 + eq121_e1494_d_n18);
        let eq121_e1495_d_n19: f64 = (eq121_e1490_d_n19 + eq121_e1494_d_n19);
        let eq121_e1495_d_n20: f64 = (eq121_e1490_d_n20 + eq121_e1494_d_n20);
        let eq121_e1495_d_n21: f64 = (eq121_e1490_d_n21 + eq121_e1494_d_n21);
        let eq121_e1495_d_n22: f64 = (eq121_e1490_d_n22 + eq121_e1494_d_n22);
        let eq121_e1495_d_n23: f64 = (eq121_e1490_d_n23 + eq121_e1494_d_n23);
        let eq121_e1495_d_n24: f64 = (eq121_e1490_d_n24 + eq121_e1494_d_n24);
        let eq121_e1495_d_n25: f64 = (eq121_e1490_d_n25 + eq121_e1494_d_n25);
        let eq121_e1495_d_n26: f64 = (eq121_e1490_d_n26 + eq121_e1494_d_n26);
        let eq121_e1495_d_n27: f64 = (eq121_e1490_d_n27 + eq121_e1494_d_n27);
        let eq121_e1495_d_n28: f64 = (eq121_e1490_d_n28 + eq121_e1494_d_n28);
        let eq121_e1495_d_n29: f64 = (eq121_e1490_d_n29 + eq121_e1494_d_n29);
        let eq121_value: f64 = eq121_e1495;
        let eq121_node_derivatives: [f64; 30] = [eq121_e1495_d_n0, eq121_e1495_d_n1, eq121_e1495_d_n2, eq121_e1495_d_n3, eq121_e1495_d_n4, eq121_e1495_d_n5, eq121_e1495_d_n6, eq121_e1495_d_n7, eq121_e1495_d_n8, eq121_e1495_d_n9, eq121_e1495_d_n10, eq121_e1495_d_n11, eq121_e1495_d_n12, eq121_e1495_d_n13, eq121_e1495_d_n14, eq121_e1495_d_n15, eq121_e1495_d_n16, eq121_e1495_d_n17, eq121_e1495_d_n18, eq121_e1495_d_n19, eq121_e1495_d_n20, eq121_e1495_d_n21, eq121_e1495_d_n22, eq121_e1495_d_n23, eq121_e1495_d_n24, eq121_e1495_d_n25, eq121_e1495_d_n26, eq121_e1495_d_n27, eq121_e1495_d_n28, eq121_e1495_d_n29];
        let eq121_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[12]),
            self.multiplicity * (eq121_value),
            &nodes,
            &eq121_node_derivatives,
            &branches,
            &eq121_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_122_block_0(
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
        let (eq122_e1503, eq122_e1503_d_n0, eq122_e1503_d_n1, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n5, eq122_e1503_d_n6, eq122_e1503_d_n7, eq122_e1503_d_n8, eq122_e1503_d_n9, eq122_e1503_d_n10, eq122_e1503_d_n11, eq122_e1503_d_n12, eq122_e1503_d_n13, eq122_e1503_d_n14, eq122_e1503_d_n15, eq122_e1503_d_n16, eq122_e1503_d_n17, eq122_e1503_d_n18, eq122_e1503_d_n19, eq122_e1503_d_n20, eq122_e1503_d_n21, eq122_e1503_d_n22, eq122_e1503_d_n23, eq122_e1503_d_n24, eq122_e1503_d_n25, eq122_e1503_d_n26, eq122_e1503_d_n27, eq122_e1503_d_n28, eq122_e1503_d_n29,) = {
    if (s.v[1349] != 0.0) {
        let eq122_e1500: f64 = (s.v[0] * (nv12 - nv13));
        let eq122_e1500_d_n12: f64 = s.v[0];
        let eq122_e1500_d_n13: f64 = (-s.v[0]);
        let eq122_e1501: f64 = (s.v[184] + eq122_e1500);
        let eq122_e1501_d_n12: f64 = (s.dn[184][12] + eq122_e1500_d_n12);
        let eq122_e1501_d_n13: f64 = (s.dn[184][13] + eq122_e1500_d_n13);
        (eq122_e1501, s.dn[184][0], s.dn[184][1], s.dn[184][2], s.dn[184][3], s.dn[184][4], s.dn[184][5], s.dn[184][6], s.dn[184][7], s.dn[184][8], s.dn[184][9], s.dn[184][10], s.dn[184][11], eq122_e1501_d_n12, eq122_e1501_d_n13, s.dn[184][14], s.dn[184][15], s.dn[184][16], s.dn[184][17], s.dn[184][18], s.dn[184][19], s.dn[184][20], s.dn[184][21], s.dn[184][22], s.dn[184][23], s.dn[184][24], s.dn[184][25], s.dn[184][26], s.dn[184][27], s.dn[184][28], s.dn[184][29],)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1503;
        let eq122_node_derivatives: [f64; 30] = [eq122_e1503_d_n0, eq122_e1503_d_n1, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n5, eq122_e1503_d_n6, eq122_e1503_d_n7, eq122_e1503_d_n8, eq122_e1503_d_n9, eq122_e1503_d_n10, eq122_e1503_d_n11, eq122_e1503_d_n12, eq122_e1503_d_n13, eq122_e1503_d_n14, eq122_e1503_d_n15, eq122_e1503_d_n16, eq122_e1503_d_n17, eq122_e1503_d_n18, eq122_e1503_d_n19, eq122_e1503_d_n20, eq122_e1503_d_n21, eq122_e1503_d_n22, eq122_e1503_d_n23, eq122_e1503_d_n24, eq122_e1503_d_n25, eq122_e1503_d_n26, eq122_e1503_d_n27, eq122_e1503_d_n28, eq122_e1503_d_n29];
        let eq122_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[13]),
            self.multiplicity * (eq122_value),
            &nodes,
            &eq122_node_derivatives,
            &branches,
            &eq122_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_123_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq123_e1508,) = {
    if (!(s.v[1349] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq123_value: f64 = eq123_e1508;
        stamper.stamp_potential(
            branches[25],
            eq123_value,
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_124_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq124_e1518, eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29,) = {
    if (s.v[1495] != 0.0) {
        let eq124_e1511: f64 = self.eval_ddt(116, s.v[185]);
        let eq124_e1511_d_n0: f64 = self.ddt_jacobian(s.dn[185][0]);
        let eq124_e1511_d_n1: f64 = self.ddt_jacobian(s.dn[185][1]);
        let eq124_e1511_d_n2: f64 = self.ddt_jacobian(s.dn[185][2]);
        let eq124_e1511_d_n3: f64 = self.ddt_jacobian(s.dn[185][3]);
        let eq124_e1511_d_n4: f64 = self.ddt_jacobian(s.dn[185][4]);
        let eq124_e1511_d_n5: f64 = self.ddt_jacobian(s.dn[185][5]);
        let eq124_e1511_d_n6: f64 = self.ddt_jacobian(s.dn[185][6]);
        let eq124_e1511_d_n7: f64 = self.ddt_jacobian(s.dn[185][7]);
        let eq124_e1511_d_n8: f64 = self.ddt_jacobian(s.dn[185][8]);
        let eq124_e1511_d_n9: f64 = self.ddt_jacobian(s.dn[185][9]);
        let eq124_e1511_d_n10: f64 = self.ddt_jacobian(s.dn[185][10]);
        let eq124_e1511_d_n11: f64 = self.ddt_jacobian(s.dn[185][11]);
        let eq124_e1511_d_n12: f64 = self.ddt_jacobian(s.dn[185][12]);
        let eq124_e1511_d_n13: f64 = self.ddt_jacobian(s.dn[185][13]);
        let eq124_e1511_d_n14: f64 = self.ddt_jacobian(s.dn[185][14]);
        let eq124_e1511_d_n15: f64 = self.ddt_jacobian(s.dn[185][15]);
        let eq124_e1511_d_n16: f64 = self.ddt_jacobian(s.dn[185][16]);
        let eq124_e1511_d_n17: f64 = self.ddt_jacobian(s.dn[185][17]);
        let eq124_e1511_d_n18: f64 = self.ddt_jacobian(s.dn[185][18]);
        let eq124_e1511_d_n19: f64 = self.ddt_jacobian(s.dn[185][19]);
        let eq124_e1511_d_n20: f64 = self.ddt_jacobian(s.dn[185][20]);
        let eq124_e1511_d_n21: f64 = self.ddt_jacobian(s.dn[185][21]);
        let eq124_e1511_d_n22: f64 = self.ddt_jacobian(s.dn[185][22]);
        let eq124_e1511_d_n23: f64 = self.ddt_jacobian(s.dn[185][23]);
        let eq124_e1511_d_n24: f64 = self.ddt_jacobian(s.dn[185][24]);
        let eq124_e1511_d_n25: f64 = self.ddt_jacobian(s.dn[185][25]);
        let eq124_e1511_d_n26: f64 = self.ddt_jacobian(s.dn[185][26]);
        let eq124_e1511_d_n27: f64 = self.ddt_jacobian(s.dn[185][27]);
        let eq124_e1511_d_n28: f64 = self.ddt_jacobian(s.dn[185][28]);
        let eq124_e1511_d_n29: f64 = self.ddt_jacobian(s.dn[185][29]);
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1514_d_n7: f64 = p.p355;
        let eq124_e1514_d_n13: f64 = (-p.p355);
        let eq124_e1515: f64 = self.eval_ddt(117, eq124_e1514);
        let eq124_e1515_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n7: f64 = self.ddt_jacobian(eq124_e1514_d_n7);
        let eq124_e1515_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n13: f64 = self.ddt_jacobian(eq124_e1514_d_n13);
        let eq124_e1515_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq124_e1515_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq124_e1516: f64 = (eq124_e1511 + eq124_e1515);
        let eq124_e1516_d_n0: f64 = (eq124_e1511_d_n0 + eq124_e1515_d_n0);
        let eq124_e1516_d_n1: f64 = (eq124_e1511_d_n1 + eq124_e1515_d_n1);
        let eq124_e1516_d_n2: f64 = (eq124_e1511_d_n2 + eq124_e1515_d_n2);
        let eq124_e1516_d_n3: f64 = (eq124_e1511_d_n3 + eq124_e1515_d_n3);
        let eq124_e1516_d_n4: f64 = (eq124_e1511_d_n4 + eq124_e1515_d_n4);
        let eq124_e1516_d_n5: f64 = (eq124_e1511_d_n5 + eq124_e1515_d_n5);
        let eq124_e1516_d_n6: f64 = (eq124_e1511_d_n6 + eq124_e1515_d_n6);
        let eq124_e1516_d_n7: f64 = (eq124_e1511_d_n7 + eq124_e1515_d_n7);
        let eq124_e1516_d_n8: f64 = (eq124_e1511_d_n8 + eq124_e1515_d_n8);
        let eq124_e1516_d_n9: f64 = (eq124_e1511_d_n9 + eq124_e1515_d_n9);
        let eq124_e1516_d_n10: f64 = (eq124_e1511_d_n10 + eq124_e1515_d_n10);
        let eq124_e1516_d_n11: f64 = (eq124_e1511_d_n11 + eq124_e1515_d_n11);
        let eq124_e1516_d_n12: f64 = (eq124_e1511_d_n12 + eq124_e1515_d_n12);
        let eq124_e1516_d_n13: f64 = (eq124_e1511_d_n13 + eq124_e1515_d_n13);
        let eq124_e1516_d_n14: f64 = (eq124_e1511_d_n14 + eq124_e1515_d_n14);
        let eq124_e1516_d_n15: f64 = (eq124_e1511_d_n15 + eq124_e1515_d_n15);
        let eq124_e1516_d_n16: f64 = (eq124_e1511_d_n16 + eq124_e1515_d_n16);
        let eq124_e1516_d_n17: f64 = (eq124_e1511_d_n17 + eq124_e1515_d_n17);
        let eq124_e1516_d_n18: f64 = (eq124_e1511_d_n18 + eq124_e1515_d_n18);
        let eq124_e1516_d_n19: f64 = (eq124_e1511_d_n19 + eq124_e1515_d_n19);
        let eq124_e1516_d_n20: f64 = (eq124_e1511_d_n20 + eq124_e1515_d_n20);
        let eq124_e1516_d_n21: f64 = (eq124_e1511_d_n21 + eq124_e1515_d_n21);
        let eq124_e1516_d_n22: f64 = (eq124_e1511_d_n22 + eq124_e1515_d_n22);
        let eq124_e1516_d_n23: f64 = (eq124_e1511_d_n23 + eq124_e1515_d_n23);
        let eq124_e1516_d_n24: f64 = (eq124_e1511_d_n24 + eq124_e1515_d_n24);
        let eq124_e1516_d_n25: f64 = (eq124_e1511_d_n25 + eq124_e1515_d_n25);
        let eq124_e1516_d_n26: f64 = (eq124_e1511_d_n26 + eq124_e1515_d_n26);
        let eq124_e1516_d_n27: f64 = (eq124_e1511_d_n27 + eq124_e1515_d_n27);
        let eq124_e1516_d_n28: f64 = (eq124_e1511_d_n28 + eq124_e1515_d_n28);
        let eq124_e1516_d_n29: f64 = (eq124_e1511_d_n29 + eq124_e1515_d_n29);
        (eq124_e1516, eq124_e1516_d_n0, eq124_e1516_d_n1, eq124_e1516_d_n2, eq124_e1516_d_n3, eq124_e1516_d_n4, eq124_e1516_d_n5, eq124_e1516_d_n6, eq124_e1516_d_n7, eq124_e1516_d_n8, eq124_e1516_d_n9, eq124_e1516_d_n10, eq124_e1516_d_n11, eq124_e1516_d_n12, eq124_e1516_d_n13, eq124_e1516_d_n14, eq124_e1516_d_n15, eq124_e1516_d_n16, eq124_e1516_d_n17, eq124_e1516_d_n18, eq124_e1516_d_n19, eq124_e1516_d_n20, eq124_e1516_d_n21, eq124_e1516_d_n22, eq124_e1516_d_n23, eq124_e1516_d_n24, eq124_e1516_d_n25, eq124_e1516_d_n26, eq124_e1516_d_n27, eq124_e1516_d_n28, eq124_e1516_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1518;
        let eq124_node_derivatives: [f64; 30] = [eq124_e1518_d_n0, eq124_e1518_d_n1, eq124_e1518_d_n2, eq124_e1518_d_n3, eq124_e1518_d_n4, eq124_e1518_d_n5, eq124_e1518_d_n6, eq124_e1518_d_n7, eq124_e1518_d_n8, eq124_e1518_d_n9, eq124_e1518_d_n10, eq124_e1518_d_n11, eq124_e1518_d_n12, eq124_e1518_d_n13, eq124_e1518_d_n14, eq124_e1518_d_n15, eq124_e1518_d_n16, eq124_e1518_d_n17, eq124_e1518_d_n18, eq124_e1518_d_n19, eq124_e1518_d_n20, eq124_e1518_d_n21, eq124_e1518_d_n22, eq124_e1518_d_n23, eq124_e1518_d_n24, eq124_e1518_d_n25, eq124_e1518_d_n26, eq124_e1518_d_n27, eq124_e1518_d_n28, eq124_e1518_d_n29];
        let eq124_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            self.multiplicity * (eq124_value),
            &nodes,
            &eq124_node_derivatives,
            &branches,
            &eq124_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_125_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq125_e1528, eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29,) = {
    if (s.v[1495] != 0.0) {
        let eq125_e1521: f64 = self.eval_ddt(118, s.v[186]);
        let eq125_e1521_d_n0: f64 = self.ddt_jacobian(s.dn[186][0]);
        let eq125_e1521_d_n1: f64 = self.ddt_jacobian(s.dn[186][1]);
        let eq125_e1521_d_n2: f64 = self.ddt_jacobian(s.dn[186][2]);
        let eq125_e1521_d_n3: f64 = self.ddt_jacobian(s.dn[186][3]);
        let eq125_e1521_d_n4: f64 = self.ddt_jacobian(s.dn[186][4]);
        let eq125_e1521_d_n5: f64 = self.ddt_jacobian(s.dn[186][5]);
        let eq125_e1521_d_n6: f64 = self.ddt_jacobian(s.dn[186][6]);
        let eq125_e1521_d_n7: f64 = self.ddt_jacobian(s.dn[186][7]);
        let eq125_e1521_d_n8: f64 = self.ddt_jacobian(s.dn[186][8]);
        let eq125_e1521_d_n9: f64 = self.ddt_jacobian(s.dn[186][9]);
        let eq125_e1521_d_n10: f64 = self.ddt_jacobian(s.dn[186][10]);
        let eq125_e1521_d_n11: f64 = self.ddt_jacobian(s.dn[186][11]);
        let eq125_e1521_d_n12: f64 = self.ddt_jacobian(s.dn[186][12]);
        let eq125_e1521_d_n13: f64 = self.ddt_jacobian(s.dn[186][13]);
        let eq125_e1521_d_n14: f64 = self.ddt_jacobian(s.dn[186][14]);
        let eq125_e1521_d_n15: f64 = self.ddt_jacobian(s.dn[186][15]);
        let eq125_e1521_d_n16: f64 = self.ddt_jacobian(s.dn[186][16]);
        let eq125_e1521_d_n17: f64 = self.ddt_jacobian(s.dn[186][17]);
        let eq125_e1521_d_n18: f64 = self.ddt_jacobian(s.dn[186][18]);
        let eq125_e1521_d_n19: f64 = self.ddt_jacobian(s.dn[186][19]);
        let eq125_e1521_d_n20: f64 = self.ddt_jacobian(s.dn[186][20]);
        let eq125_e1521_d_n21: f64 = self.ddt_jacobian(s.dn[186][21]);
        let eq125_e1521_d_n22: f64 = self.ddt_jacobian(s.dn[186][22]);
        let eq125_e1521_d_n23: f64 = self.ddt_jacobian(s.dn[186][23]);
        let eq125_e1521_d_n24: f64 = self.ddt_jacobian(s.dn[186][24]);
        let eq125_e1521_d_n25: f64 = self.ddt_jacobian(s.dn[186][25]);
        let eq125_e1521_d_n26: f64 = self.ddt_jacobian(s.dn[186][26]);
        let eq125_e1521_d_n27: f64 = self.ddt_jacobian(s.dn[186][27]);
        let eq125_e1521_d_n28: f64 = self.ddt_jacobian(s.dn[186][28]);
        let eq125_e1521_d_n29: f64 = self.ddt_jacobian(s.dn[186][29]);
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1524_d_n7: f64 = p.p355;
        let eq125_e1524_d_n12: f64 = (-p.p355);
        let eq125_e1525: f64 = self.eval_ddt(119, eq125_e1524);
        let eq125_e1525_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n7: f64 = self.ddt_jacobian(eq125_e1524_d_n7);
        let eq125_e1525_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n12: f64 = self.ddt_jacobian(eq125_e1524_d_n12);
        let eq125_e1525_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq125_e1525_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq125_e1526: f64 = (eq125_e1521 + eq125_e1525);
        let eq125_e1526_d_n0: f64 = (eq125_e1521_d_n0 + eq125_e1525_d_n0);
        let eq125_e1526_d_n1: f64 = (eq125_e1521_d_n1 + eq125_e1525_d_n1);
        let eq125_e1526_d_n2: f64 = (eq125_e1521_d_n2 + eq125_e1525_d_n2);
        let eq125_e1526_d_n3: f64 = (eq125_e1521_d_n3 + eq125_e1525_d_n3);
        let eq125_e1526_d_n4: f64 = (eq125_e1521_d_n4 + eq125_e1525_d_n4);
        let eq125_e1526_d_n5: f64 = (eq125_e1521_d_n5 + eq125_e1525_d_n5);
        let eq125_e1526_d_n6: f64 = (eq125_e1521_d_n6 + eq125_e1525_d_n6);
        let eq125_e1526_d_n7: f64 = (eq125_e1521_d_n7 + eq125_e1525_d_n7);
        let eq125_e1526_d_n8: f64 = (eq125_e1521_d_n8 + eq125_e1525_d_n8);
        let eq125_e1526_d_n9: f64 = (eq125_e1521_d_n9 + eq125_e1525_d_n9);
        let eq125_e1526_d_n10: f64 = (eq125_e1521_d_n10 + eq125_e1525_d_n10);
        let eq125_e1526_d_n11: f64 = (eq125_e1521_d_n11 + eq125_e1525_d_n11);
        let eq125_e1526_d_n12: f64 = (eq125_e1521_d_n12 + eq125_e1525_d_n12);
        let eq125_e1526_d_n13: f64 = (eq125_e1521_d_n13 + eq125_e1525_d_n13);
        let eq125_e1526_d_n14: f64 = (eq125_e1521_d_n14 + eq125_e1525_d_n14);
        let eq125_e1526_d_n15: f64 = (eq125_e1521_d_n15 + eq125_e1525_d_n15);
        let eq125_e1526_d_n16: f64 = (eq125_e1521_d_n16 + eq125_e1525_d_n16);
        let eq125_e1526_d_n17: f64 = (eq125_e1521_d_n17 + eq125_e1525_d_n17);
        let eq125_e1526_d_n18: f64 = (eq125_e1521_d_n18 + eq125_e1525_d_n18);
        let eq125_e1526_d_n19: f64 = (eq125_e1521_d_n19 + eq125_e1525_d_n19);
        let eq125_e1526_d_n20: f64 = (eq125_e1521_d_n20 + eq125_e1525_d_n20);
        let eq125_e1526_d_n21: f64 = (eq125_e1521_d_n21 + eq125_e1525_d_n21);
        let eq125_e1526_d_n22: f64 = (eq125_e1521_d_n22 + eq125_e1525_d_n22);
        let eq125_e1526_d_n23: f64 = (eq125_e1521_d_n23 + eq125_e1525_d_n23);
        let eq125_e1526_d_n24: f64 = (eq125_e1521_d_n24 + eq125_e1525_d_n24);
        let eq125_e1526_d_n25: f64 = (eq125_e1521_d_n25 + eq125_e1525_d_n25);
        let eq125_e1526_d_n26: f64 = (eq125_e1521_d_n26 + eq125_e1525_d_n26);
        let eq125_e1526_d_n27: f64 = (eq125_e1521_d_n27 + eq125_e1525_d_n27);
        let eq125_e1526_d_n28: f64 = (eq125_e1521_d_n28 + eq125_e1525_d_n28);
        let eq125_e1526_d_n29: f64 = (eq125_e1521_d_n29 + eq125_e1525_d_n29);
        (eq125_e1526, eq125_e1526_d_n0, eq125_e1526_d_n1, eq125_e1526_d_n2, eq125_e1526_d_n3, eq125_e1526_d_n4, eq125_e1526_d_n5, eq125_e1526_d_n6, eq125_e1526_d_n7, eq125_e1526_d_n8, eq125_e1526_d_n9, eq125_e1526_d_n10, eq125_e1526_d_n11, eq125_e1526_d_n12, eq125_e1526_d_n13, eq125_e1526_d_n14, eq125_e1526_d_n15, eq125_e1526_d_n16, eq125_e1526_d_n17, eq125_e1526_d_n18, eq125_e1526_d_n19, eq125_e1526_d_n20, eq125_e1526_d_n21, eq125_e1526_d_n22, eq125_e1526_d_n23, eq125_e1526_d_n24, eq125_e1526_d_n25, eq125_e1526_d_n26, eq125_e1526_d_n27, eq125_e1526_d_n28, eq125_e1526_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1528;
        let eq125_node_derivatives: [f64; 30] = [eq125_e1528_d_n0, eq125_e1528_d_n1, eq125_e1528_d_n2, eq125_e1528_d_n3, eq125_e1528_d_n4, eq125_e1528_d_n5, eq125_e1528_d_n6, eq125_e1528_d_n7, eq125_e1528_d_n8, eq125_e1528_d_n9, eq125_e1528_d_n10, eq125_e1528_d_n11, eq125_e1528_d_n12, eq125_e1528_d_n13, eq125_e1528_d_n14, eq125_e1528_d_n15, eq125_e1528_d_n16, eq125_e1528_d_n17, eq125_e1528_d_n18, eq125_e1528_d_n19, eq125_e1528_d_n20, eq125_e1528_d_n21, eq125_e1528_d_n22, eq125_e1528_d_n23, eq125_e1528_d_n24, eq125_e1528_d_n25, eq125_e1528_d_n26, eq125_e1528_d_n27, eq125_e1528_d_n28, eq125_e1528_d_n29];
        let eq125_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[12]),
            self.multiplicity * (eq125_value),
            &nodes,
            &eq125_node_derivatives,
            &branches,
            &eq125_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_126_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq126_e1538, eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29,) = {
    if (s.v[1495] != 0.0) {
        let eq126_e1531: f64 = self.eval_ddt(120, s.v[187]);
        let eq126_e1531_d_n0: f64 = self.ddt_jacobian(s.dn[187][0]);
        let eq126_e1531_d_n1: f64 = self.ddt_jacobian(s.dn[187][1]);
        let eq126_e1531_d_n2: f64 = self.ddt_jacobian(s.dn[187][2]);
        let eq126_e1531_d_n3: f64 = self.ddt_jacobian(s.dn[187][3]);
        let eq126_e1531_d_n4: f64 = self.ddt_jacobian(s.dn[187][4]);
        let eq126_e1531_d_n5: f64 = self.ddt_jacobian(s.dn[187][5]);
        let eq126_e1531_d_n6: f64 = self.ddt_jacobian(s.dn[187][6]);
        let eq126_e1531_d_n7: f64 = self.ddt_jacobian(s.dn[187][7]);
        let eq126_e1531_d_n8: f64 = self.ddt_jacobian(s.dn[187][8]);
        let eq126_e1531_d_n9: f64 = self.ddt_jacobian(s.dn[187][9]);
        let eq126_e1531_d_n10: f64 = self.ddt_jacobian(s.dn[187][10]);
        let eq126_e1531_d_n11: f64 = self.ddt_jacobian(s.dn[187][11]);
        let eq126_e1531_d_n12: f64 = self.ddt_jacobian(s.dn[187][12]);
        let eq126_e1531_d_n13: f64 = self.ddt_jacobian(s.dn[187][13]);
        let eq126_e1531_d_n14: f64 = self.ddt_jacobian(s.dn[187][14]);
        let eq126_e1531_d_n15: f64 = self.ddt_jacobian(s.dn[187][15]);
        let eq126_e1531_d_n16: f64 = self.ddt_jacobian(s.dn[187][16]);
        let eq126_e1531_d_n17: f64 = self.ddt_jacobian(s.dn[187][17]);
        let eq126_e1531_d_n18: f64 = self.ddt_jacobian(s.dn[187][18]);
        let eq126_e1531_d_n19: f64 = self.ddt_jacobian(s.dn[187][19]);
        let eq126_e1531_d_n20: f64 = self.ddt_jacobian(s.dn[187][20]);
        let eq126_e1531_d_n21: f64 = self.ddt_jacobian(s.dn[187][21]);
        let eq126_e1531_d_n22: f64 = self.ddt_jacobian(s.dn[187][22]);
        let eq126_e1531_d_n23: f64 = self.ddt_jacobian(s.dn[187][23]);
        let eq126_e1531_d_n24: f64 = self.ddt_jacobian(s.dn[187][24]);
        let eq126_e1531_d_n25: f64 = self.ddt_jacobian(s.dn[187][25]);
        let eq126_e1531_d_n26: f64 = self.ddt_jacobian(s.dn[187][26]);
        let eq126_e1531_d_n27: f64 = self.ddt_jacobian(s.dn[187][27]);
        let eq126_e1531_d_n28: f64 = self.ddt_jacobian(s.dn[187][28]);
        let eq126_e1531_d_n29: f64 = self.ddt_jacobian(s.dn[187][29]);
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1534_d_n2: f64 = p.p355;
        let eq126_e1534_d_n13: f64 = (-p.p355);
        let eq126_e1535: f64 = self.eval_ddt(121, eq126_e1534);
        let eq126_e1535_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n2: f64 = self.ddt_jacobian(eq126_e1534_d_n2);
        let eq126_e1535_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n13: f64 = self.ddt_jacobian(eq126_e1534_d_n13);
        let eq126_e1535_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq126_e1535_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq126_e1536: f64 = (eq126_e1531 + eq126_e1535);
        let eq126_e1536_d_n0: f64 = (eq126_e1531_d_n0 + eq126_e1535_d_n0);
        let eq126_e1536_d_n1: f64 = (eq126_e1531_d_n1 + eq126_e1535_d_n1);
        let eq126_e1536_d_n2: f64 = (eq126_e1531_d_n2 + eq126_e1535_d_n2);
        let eq126_e1536_d_n3: f64 = (eq126_e1531_d_n3 + eq126_e1535_d_n3);
        let eq126_e1536_d_n4: f64 = (eq126_e1531_d_n4 + eq126_e1535_d_n4);
        let eq126_e1536_d_n5: f64 = (eq126_e1531_d_n5 + eq126_e1535_d_n5);
        let eq126_e1536_d_n6: f64 = (eq126_e1531_d_n6 + eq126_e1535_d_n6);
        let eq126_e1536_d_n7: f64 = (eq126_e1531_d_n7 + eq126_e1535_d_n7);
        let eq126_e1536_d_n8: f64 = (eq126_e1531_d_n8 + eq126_e1535_d_n8);
        let eq126_e1536_d_n9: f64 = (eq126_e1531_d_n9 + eq126_e1535_d_n9);
        let eq126_e1536_d_n10: f64 = (eq126_e1531_d_n10 + eq126_e1535_d_n10);
        let eq126_e1536_d_n11: f64 = (eq126_e1531_d_n11 + eq126_e1535_d_n11);
        let eq126_e1536_d_n12: f64 = (eq126_e1531_d_n12 + eq126_e1535_d_n12);
        let eq126_e1536_d_n13: f64 = (eq126_e1531_d_n13 + eq126_e1535_d_n13);
        let eq126_e1536_d_n14: f64 = (eq126_e1531_d_n14 + eq126_e1535_d_n14);
        let eq126_e1536_d_n15: f64 = (eq126_e1531_d_n15 + eq126_e1535_d_n15);
        let eq126_e1536_d_n16: f64 = (eq126_e1531_d_n16 + eq126_e1535_d_n16);
        let eq126_e1536_d_n17: f64 = (eq126_e1531_d_n17 + eq126_e1535_d_n17);
        let eq126_e1536_d_n18: f64 = (eq126_e1531_d_n18 + eq126_e1535_d_n18);
        let eq126_e1536_d_n19: f64 = (eq126_e1531_d_n19 + eq126_e1535_d_n19);
        let eq126_e1536_d_n20: f64 = (eq126_e1531_d_n20 + eq126_e1535_d_n20);
        let eq126_e1536_d_n21: f64 = (eq126_e1531_d_n21 + eq126_e1535_d_n21);
        let eq126_e1536_d_n22: f64 = (eq126_e1531_d_n22 + eq126_e1535_d_n22);
        let eq126_e1536_d_n23: f64 = (eq126_e1531_d_n23 + eq126_e1535_d_n23);
        let eq126_e1536_d_n24: f64 = (eq126_e1531_d_n24 + eq126_e1535_d_n24);
        let eq126_e1536_d_n25: f64 = (eq126_e1531_d_n25 + eq126_e1535_d_n25);
        let eq126_e1536_d_n26: f64 = (eq126_e1531_d_n26 + eq126_e1535_d_n26);
        let eq126_e1536_d_n27: f64 = (eq126_e1531_d_n27 + eq126_e1535_d_n27);
        let eq126_e1536_d_n28: f64 = (eq126_e1531_d_n28 + eq126_e1535_d_n28);
        let eq126_e1536_d_n29: f64 = (eq126_e1531_d_n29 + eq126_e1535_d_n29);
        (eq126_e1536, eq126_e1536_d_n0, eq126_e1536_d_n1, eq126_e1536_d_n2, eq126_e1536_d_n3, eq126_e1536_d_n4, eq126_e1536_d_n5, eq126_e1536_d_n6, eq126_e1536_d_n7, eq126_e1536_d_n8, eq126_e1536_d_n9, eq126_e1536_d_n10, eq126_e1536_d_n11, eq126_e1536_d_n12, eq126_e1536_d_n13, eq126_e1536_d_n14, eq126_e1536_d_n15, eq126_e1536_d_n16, eq126_e1536_d_n17, eq126_e1536_d_n18, eq126_e1536_d_n19, eq126_e1536_d_n20, eq126_e1536_d_n21, eq126_e1536_d_n22, eq126_e1536_d_n23, eq126_e1536_d_n24, eq126_e1536_d_n25, eq126_e1536_d_n26, eq126_e1536_d_n27, eq126_e1536_d_n28, eq126_e1536_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1538;
        let eq126_node_derivatives: [f64; 30] = [eq126_e1538_d_n0, eq126_e1538_d_n1, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n5, eq126_e1538_d_n6, eq126_e1538_d_n7, eq126_e1538_d_n8, eq126_e1538_d_n9, eq126_e1538_d_n10, eq126_e1538_d_n11, eq126_e1538_d_n12, eq126_e1538_d_n13, eq126_e1538_d_n14, eq126_e1538_d_n15, eq126_e1538_d_n16, eq126_e1538_d_n17, eq126_e1538_d_n18, eq126_e1538_d_n19, eq126_e1538_d_n20, eq126_e1538_d_n21, eq126_e1538_d_n22, eq126_e1538_d_n23, eq126_e1538_d_n24, eq126_e1538_d_n25, eq126_e1538_d_n26, eq126_e1538_d_n27, eq126_e1538_d_n28, eq126_e1538_d_n29];
        let eq126_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            self.multiplicity * (eq126_value),
            &nodes,
            &eq126_node_derivatives,
            &branches,
            &eq126_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_127_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq127_e1542,) = {
    if (s.v[1495] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq127_value: f64 = eq127_e1542;
        stamper.stamp_current(
            Some(nodes[2]),
            Some(nodes[12]),
            self.multiplicity * (eq127_value),
            &[
            ],
        );
    }

    pub(super) fn stamp_transient_equation_128_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq128_e1552, eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29,) = {
    if (s.v[1495] != 0.0) {
        let eq128_e1545: f64 = self.eval_ddt(122, s.v[189]);
        let eq128_e1545_d_n0: f64 = self.ddt_jacobian(s.dn[189][0]);
        let eq128_e1545_d_n1: f64 = self.ddt_jacobian(s.dn[189][1]);
        let eq128_e1545_d_n2: f64 = self.ddt_jacobian(s.dn[189][2]);
        let eq128_e1545_d_n3: f64 = self.ddt_jacobian(s.dn[189][3]);
        let eq128_e1545_d_n4: f64 = self.ddt_jacobian(s.dn[189][4]);
        let eq128_e1545_d_n5: f64 = self.ddt_jacobian(s.dn[189][5]);
        let eq128_e1545_d_n6: f64 = self.ddt_jacobian(s.dn[189][6]);
        let eq128_e1545_d_n7: f64 = self.ddt_jacobian(s.dn[189][7]);
        let eq128_e1545_d_n8: f64 = self.ddt_jacobian(s.dn[189][8]);
        let eq128_e1545_d_n9: f64 = self.ddt_jacobian(s.dn[189][9]);
        let eq128_e1545_d_n10: f64 = self.ddt_jacobian(s.dn[189][10]);
        let eq128_e1545_d_n11: f64 = self.ddt_jacobian(s.dn[189][11]);
        let eq128_e1545_d_n12: f64 = self.ddt_jacobian(s.dn[189][12]);
        let eq128_e1545_d_n13: f64 = self.ddt_jacobian(s.dn[189][13]);
        let eq128_e1545_d_n14: f64 = self.ddt_jacobian(s.dn[189][14]);
        let eq128_e1545_d_n15: f64 = self.ddt_jacobian(s.dn[189][15]);
        let eq128_e1545_d_n16: f64 = self.ddt_jacobian(s.dn[189][16]);
        let eq128_e1545_d_n17: f64 = self.ddt_jacobian(s.dn[189][17]);
        let eq128_e1545_d_n18: f64 = self.ddt_jacobian(s.dn[189][18]);
        let eq128_e1545_d_n19: f64 = self.ddt_jacobian(s.dn[189][19]);
        let eq128_e1545_d_n20: f64 = self.ddt_jacobian(s.dn[189][20]);
        let eq128_e1545_d_n21: f64 = self.ddt_jacobian(s.dn[189][21]);
        let eq128_e1545_d_n22: f64 = self.ddt_jacobian(s.dn[189][22]);
        let eq128_e1545_d_n23: f64 = self.ddt_jacobian(s.dn[189][23]);
        let eq128_e1545_d_n24: f64 = self.ddt_jacobian(s.dn[189][24]);
        let eq128_e1545_d_n25: f64 = self.ddt_jacobian(s.dn[189][25]);
        let eq128_e1545_d_n26: f64 = self.ddt_jacobian(s.dn[189][26]);
        let eq128_e1545_d_n27: f64 = self.ddt_jacobian(s.dn[189][27]);
        let eq128_e1545_d_n28: f64 = self.ddt_jacobian(s.dn[189][28]);
        let eq128_e1545_d_n29: f64 = self.ddt_jacobian(s.dn[189][29]);
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1548_d_n7: f64 = p.p355;
        let eq128_e1548_d_n9: f64 = (-p.p355);
        let eq128_e1549: f64 = self.eval_ddt(123, eq128_e1548);
        let eq128_e1549_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n7: f64 = self.ddt_jacobian(eq128_e1548_d_n7);
        let eq128_e1549_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n9: f64 = self.ddt_jacobian(eq128_e1548_d_n9);
        let eq128_e1549_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq128_e1549_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq128_e1550: f64 = (eq128_e1545 + eq128_e1549);
        let eq128_e1550_d_n0: f64 = (eq128_e1545_d_n0 + eq128_e1549_d_n0);
        let eq128_e1550_d_n1: f64 = (eq128_e1545_d_n1 + eq128_e1549_d_n1);
        let eq128_e1550_d_n2: f64 = (eq128_e1545_d_n2 + eq128_e1549_d_n2);
        let eq128_e1550_d_n3: f64 = (eq128_e1545_d_n3 + eq128_e1549_d_n3);
        let eq128_e1550_d_n4: f64 = (eq128_e1545_d_n4 + eq128_e1549_d_n4);
        let eq128_e1550_d_n5: f64 = (eq128_e1545_d_n5 + eq128_e1549_d_n5);
        let eq128_e1550_d_n6: f64 = (eq128_e1545_d_n6 + eq128_e1549_d_n6);
        let eq128_e1550_d_n7: f64 = (eq128_e1545_d_n7 + eq128_e1549_d_n7);
        let eq128_e1550_d_n8: f64 = (eq128_e1545_d_n8 + eq128_e1549_d_n8);
        let eq128_e1550_d_n9: f64 = (eq128_e1545_d_n9 + eq128_e1549_d_n9);
        let eq128_e1550_d_n10: f64 = (eq128_e1545_d_n10 + eq128_e1549_d_n10);
        let eq128_e1550_d_n11: f64 = (eq128_e1545_d_n11 + eq128_e1549_d_n11);
        let eq128_e1550_d_n12: f64 = (eq128_e1545_d_n12 + eq128_e1549_d_n12);
        let eq128_e1550_d_n13: f64 = (eq128_e1545_d_n13 + eq128_e1549_d_n13);
        let eq128_e1550_d_n14: f64 = (eq128_e1545_d_n14 + eq128_e1549_d_n14);
        let eq128_e1550_d_n15: f64 = (eq128_e1545_d_n15 + eq128_e1549_d_n15);
        let eq128_e1550_d_n16: f64 = (eq128_e1545_d_n16 + eq128_e1549_d_n16);
        let eq128_e1550_d_n17: f64 = (eq128_e1545_d_n17 + eq128_e1549_d_n17);
        let eq128_e1550_d_n18: f64 = (eq128_e1545_d_n18 + eq128_e1549_d_n18);
        let eq128_e1550_d_n19: f64 = (eq128_e1545_d_n19 + eq128_e1549_d_n19);
        let eq128_e1550_d_n20: f64 = (eq128_e1545_d_n20 + eq128_e1549_d_n20);
        let eq128_e1550_d_n21: f64 = (eq128_e1545_d_n21 + eq128_e1549_d_n21);
        let eq128_e1550_d_n22: f64 = (eq128_e1545_d_n22 + eq128_e1549_d_n22);
        let eq128_e1550_d_n23: f64 = (eq128_e1545_d_n23 + eq128_e1549_d_n23);
        let eq128_e1550_d_n24: f64 = (eq128_e1545_d_n24 + eq128_e1549_d_n24);
        let eq128_e1550_d_n25: f64 = (eq128_e1545_d_n25 + eq128_e1549_d_n25);
        let eq128_e1550_d_n26: f64 = (eq128_e1545_d_n26 + eq128_e1549_d_n26);
        let eq128_e1550_d_n27: f64 = (eq128_e1545_d_n27 + eq128_e1549_d_n27);
        let eq128_e1550_d_n28: f64 = (eq128_e1545_d_n28 + eq128_e1549_d_n28);
        let eq128_e1550_d_n29: f64 = (eq128_e1545_d_n29 + eq128_e1549_d_n29);
        (eq128_e1550, eq128_e1550_d_n0, eq128_e1550_d_n1, eq128_e1550_d_n2, eq128_e1550_d_n3, eq128_e1550_d_n4, eq128_e1550_d_n5, eq128_e1550_d_n6, eq128_e1550_d_n7, eq128_e1550_d_n8, eq128_e1550_d_n9, eq128_e1550_d_n10, eq128_e1550_d_n11, eq128_e1550_d_n12, eq128_e1550_d_n13, eq128_e1550_d_n14, eq128_e1550_d_n15, eq128_e1550_d_n16, eq128_e1550_d_n17, eq128_e1550_d_n18, eq128_e1550_d_n19, eq128_e1550_d_n20, eq128_e1550_d_n21, eq128_e1550_d_n22, eq128_e1550_d_n23, eq128_e1550_d_n24, eq128_e1550_d_n25, eq128_e1550_d_n26, eq128_e1550_d_n27, eq128_e1550_d_n28, eq128_e1550_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1552;
        let eq128_node_derivatives: [f64; 30] = [eq128_e1552_d_n0, eq128_e1552_d_n1, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n5, eq128_e1552_d_n6, eq128_e1552_d_n7, eq128_e1552_d_n8, eq128_e1552_d_n9, eq128_e1552_d_n10, eq128_e1552_d_n11, eq128_e1552_d_n12, eq128_e1552_d_n13, eq128_e1552_d_n14, eq128_e1552_d_n15, eq128_e1552_d_n16, eq128_e1552_d_n17, eq128_e1552_d_n18, eq128_e1552_d_n19, eq128_e1552_d_n20, eq128_e1552_d_n21, eq128_e1552_d_n22, eq128_e1552_d_n23, eq128_e1552_d_n24, eq128_e1552_d_n25, eq128_e1552_d_n26, eq128_e1552_d_n27, eq128_e1552_d_n28, eq128_e1552_d_n29];
        let eq128_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            self.multiplicity * (eq128_value),
            &nodes,
            &eq128_node_derivatives,
            &branches,
            &eq128_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_129_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq129_e1563, eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29,) = {
    if (!(s.v[1495] != 0.0)) {
        let eq129_e1556: f64 = self.eval_ddt(124, s.v[185]);
        let eq129_e1556_d_n0: f64 = self.ddt_jacobian(s.dn[185][0]);
        let eq129_e1556_d_n1: f64 = self.ddt_jacobian(s.dn[185][1]);
        let eq129_e1556_d_n2: f64 = self.ddt_jacobian(s.dn[185][2]);
        let eq129_e1556_d_n3: f64 = self.ddt_jacobian(s.dn[185][3]);
        let eq129_e1556_d_n4: f64 = self.ddt_jacobian(s.dn[185][4]);
        let eq129_e1556_d_n5: f64 = self.ddt_jacobian(s.dn[185][5]);
        let eq129_e1556_d_n6: f64 = self.ddt_jacobian(s.dn[185][6]);
        let eq129_e1556_d_n7: f64 = self.ddt_jacobian(s.dn[185][7]);
        let eq129_e1556_d_n8: f64 = self.ddt_jacobian(s.dn[185][8]);
        let eq129_e1556_d_n9: f64 = self.ddt_jacobian(s.dn[185][9]);
        let eq129_e1556_d_n10: f64 = self.ddt_jacobian(s.dn[185][10]);
        let eq129_e1556_d_n11: f64 = self.ddt_jacobian(s.dn[185][11]);
        let eq129_e1556_d_n12: f64 = self.ddt_jacobian(s.dn[185][12]);
        let eq129_e1556_d_n13: f64 = self.ddt_jacobian(s.dn[185][13]);
        let eq129_e1556_d_n14: f64 = self.ddt_jacobian(s.dn[185][14]);
        let eq129_e1556_d_n15: f64 = self.ddt_jacobian(s.dn[185][15]);
        let eq129_e1556_d_n16: f64 = self.ddt_jacobian(s.dn[185][16]);
        let eq129_e1556_d_n17: f64 = self.ddt_jacobian(s.dn[185][17]);
        let eq129_e1556_d_n18: f64 = self.ddt_jacobian(s.dn[185][18]);
        let eq129_e1556_d_n19: f64 = self.ddt_jacobian(s.dn[185][19]);
        let eq129_e1556_d_n20: f64 = self.ddt_jacobian(s.dn[185][20]);
        let eq129_e1556_d_n21: f64 = self.ddt_jacobian(s.dn[185][21]);
        let eq129_e1556_d_n22: f64 = self.ddt_jacobian(s.dn[185][22]);
        let eq129_e1556_d_n23: f64 = self.ddt_jacobian(s.dn[185][23]);
        let eq129_e1556_d_n24: f64 = self.ddt_jacobian(s.dn[185][24]);
        let eq129_e1556_d_n25: f64 = self.ddt_jacobian(s.dn[185][25]);
        let eq129_e1556_d_n26: f64 = self.ddt_jacobian(s.dn[185][26]);
        let eq129_e1556_d_n27: f64 = self.ddt_jacobian(s.dn[185][27]);
        let eq129_e1556_d_n28: f64 = self.ddt_jacobian(s.dn[185][28]);
        let eq129_e1556_d_n29: f64 = self.ddt_jacobian(s.dn[185][29]);
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1559_d_n2: f64 = p.p355;
        let eq129_e1559_d_n13: f64 = (-p.p355);
        let eq129_e1560: f64 = self.eval_ddt(125, eq129_e1559);
        let eq129_e1560_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n2: f64 = self.ddt_jacobian(eq129_e1559_d_n2);
        let eq129_e1560_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n13: f64 = self.ddt_jacobian(eq129_e1559_d_n13);
        let eq129_e1560_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq129_e1560_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq129_e1561: f64 = (eq129_e1556 + eq129_e1560);
        let eq129_e1561_d_n0: f64 = (eq129_e1556_d_n0 + eq129_e1560_d_n0);
        let eq129_e1561_d_n1: f64 = (eq129_e1556_d_n1 + eq129_e1560_d_n1);
        let eq129_e1561_d_n2: f64 = (eq129_e1556_d_n2 + eq129_e1560_d_n2);
        let eq129_e1561_d_n3: f64 = (eq129_e1556_d_n3 + eq129_e1560_d_n3);
        let eq129_e1561_d_n4: f64 = (eq129_e1556_d_n4 + eq129_e1560_d_n4);
        let eq129_e1561_d_n5: f64 = (eq129_e1556_d_n5 + eq129_e1560_d_n5);
        let eq129_e1561_d_n6: f64 = (eq129_e1556_d_n6 + eq129_e1560_d_n6);
        let eq129_e1561_d_n7: f64 = (eq129_e1556_d_n7 + eq129_e1560_d_n7);
        let eq129_e1561_d_n8: f64 = (eq129_e1556_d_n8 + eq129_e1560_d_n8);
        let eq129_e1561_d_n9: f64 = (eq129_e1556_d_n9 + eq129_e1560_d_n9);
        let eq129_e1561_d_n10: f64 = (eq129_e1556_d_n10 + eq129_e1560_d_n10);
        let eq129_e1561_d_n11: f64 = (eq129_e1556_d_n11 + eq129_e1560_d_n11);
        let eq129_e1561_d_n12: f64 = (eq129_e1556_d_n12 + eq129_e1560_d_n12);
        let eq129_e1561_d_n13: f64 = (eq129_e1556_d_n13 + eq129_e1560_d_n13);
        let eq129_e1561_d_n14: f64 = (eq129_e1556_d_n14 + eq129_e1560_d_n14);
        let eq129_e1561_d_n15: f64 = (eq129_e1556_d_n15 + eq129_e1560_d_n15);
        let eq129_e1561_d_n16: f64 = (eq129_e1556_d_n16 + eq129_e1560_d_n16);
        let eq129_e1561_d_n17: f64 = (eq129_e1556_d_n17 + eq129_e1560_d_n17);
        let eq129_e1561_d_n18: f64 = (eq129_e1556_d_n18 + eq129_e1560_d_n18);
        let eq129_e1561_d_n19: f64 = (eq129_e1556_d_n19 + eq129_e1560_d_n19);
        let eq129_e1561_d_n20: f64 = (eq129_e1556_d_n20 + eq129_e1560_d_n20);
        let eq129_e1561_d_n21: f64 = (eq129_e1556_d_n21 + eq129_e1560_d_n21);
        let eq129_e1561_d_n22: f64 = (eq129_e1556_d_n22 + eq129_e1560_d_n22);
        let eq129_e1561_d_n23: f64 = (eq129_e1556_d_n23 + eq129_e1560_d_n23);
        let eq129_e1561_d_n24: f64 = (eq129_e1556_d_n24 + eq129_e1560_d_n24);
        let eq129_e1561_d_n25: f64 = (eq129_e1556_d_n25 + eq129_e1560_d_n25);
        let eq129_e1561_d_n26: f64 = (eq129_e1556_d_n26 + eq129_e1560_d_n26);
        let eq129_e1561_d_n27: f64 = (eq129_e1556_d_n27 + eq129_e1560_d_n27);
        let eq129_e1561_d_n28: f64 = (eq129_e1556_d_n28 + eq129_e1560_d_n28);
        let eq129_e1561_d_n29: f64 = (eq129_e1556_d_n29 + eq129_e1560_d_n29);
        (eq129_e1561, eq129_e1561_d_n0, eq129_e1561_d_n1, eq129_e1561_d_n2, eq129_e1561_d_n3, eq129_e1561_d_n4, eq129_e1561_d_n5, eq129_e1561_d_n6, eq129_e1561_d_n7, eq129_e1561_d_n8, eq129_e1561_d_n9, eq129_e1561_d_n10, eq129_e1561_d_n11, eq129_e1561_d_n12, eq129_e1561_d_n13, eq129_e1561_d_n14, eq129_e1561_d_n15, eq129_e1561_d_n16, eq129_e1561_d_n17, eq129_e1561_d_n18, eq129_e1561_d_n19, eq129_e1561_d_n20, eq129_e1561_d_n21, eq129_e1561_d_n22, eq129_e1561_d_n23, eq129_e1561_d_n24, eq129_e1561_d_n25, eq129_e1561_d_n26, eq129_e1561_d_n27, eq129_e1561_d_n28, eq129_e1561_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1563;
        let eq129_node_derivatives: [f64; 30] = [eq129_e1563_d_n0, eq129_e1563_d_n1, eq129_e1563_d_n2, eq129_e1563_d_n3, eq129_e1563_d_n4, eq129_e1563_d_n5, eq129_e1563_d_n6, eq129_e1563_d_n7, eq129_e1563_d_n8, eq129_e1563_d_n9, eq129_e1563_d_n10, eq129_e1563_d_n11, eq129_e1563_d_n12, eq129_e1563_d_n13, eq129_e1563_d_n14, eq129_e1563_d_n15, eq129_e1563_d_n16, eq129_e1563_d_n17, eq129_e1563_d_n18, eq129_e1563_d_n19, eq129_e1563_d_n20, eq129_e1563_d_n21, eq129_e1563_d_n22, eq129_e1563_d_n23, eq129_e1563_d_n24, eq129_e1563_d_n25, eq129_e1563_d_n26, eq129_e1563_d_n27, eq129_e1563_d_n28, eq129_e1563_d_n29];
        let eq129_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[13]),
            self.multiplicity * (eq129_value),
            &nodes,
            &eq129_node_derivatives,
            &branches,
            &eq129_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_130_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq130_e1574, eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29,) = {
    if (!(s.v[1495] != 0.0)) {
        let eq130_e1567: f64 = self.eval_ddt(126, s.v[186]);
        let eq130_e1567_d_n0: f64 = self.ddt_jacobian(s.dn[186][0]);
        let eq130_e1567_d_n1: f64 = self.ddt_jacobian(s.dn[186][1]);
        let eq130_e1567_d_n2: f64 = self.ddt_jacobian(s.dn[186][2]);
        let eq130_e1567_d_n3: f64 = self.ddt_jacobian(s.dn[186][3]);
        let eq130_e1567_d_n4: f64 = self.ddt_jacobian(s.dn[186][4]);
        let eq130_e1567_d_n5: f64 = self.ddt_jacobian(s.dn[186][5]);
        let eq130_e1567_d_n6: f64 = self.ddt_jacobian(s.dn[186][6]);
        let eq130_e1567_d_n7: f64 = self.ddt_jacobian(s.dn[186][7]);
        let eq130_e1567_d_n8: f64 = self.ddt_jacobian(s.dn[186][8]);
        let eq130_e1567_d_n9: f64 = self.ddt_jacobian(s.dn[186][9]);
        let eq130_e1567_d_n10: f64 = self.ddt_jacobian(s.dn[186][10]);
        let eq130_e1567_d_n11: f64 = self.ddt_jacobian(s.dn[186][11]);
        let eq130_e1567_d_n12: f64 = self.ddt_jacobian(s.dn[186][12]);
        let eq130_e1567_d_n13: f64 = self.ddt_jacobian(s.dn[186][13]);
        let eq130_e1567_d_n14: f64 = self.ddt_jacobian(s.dn[186][14]);
        let eq130_e1567_d_n15: f64 = self.ddt_jacobian(s.dn[186][15]);
        let eq130_e1567_d_n16: f64 = self.ddt_jacobian(s.dn[186][16]);
        let eq130_e1567_d_n17: f64 = self.ddt_jacobian(s.dn[186][17]);
        let eq130_e1567_d_n18: f64 = self.ddt_jacobian(s.dn[186][18]);
        let eq130_e1567_d_n19: f64 = self.ddt_jacobian(s.dn[186][19]);
        let eq130_e1567_d_n20: f64 = self.ddt_jacobian(s.dn[186][20]);
        let eq130_e1567_d_n21: f64 = self.ddt_jacobian(s.dn[186][21]);
        let eq130_e1567_d_n22: f64 = self.ddt_jacobian(s.dn[186][22]);
        let eq130_e1567_d_n23: f64 = self.ddt_jacobian(s.dn[186][23]);
        let eq130_e1567_d_n24: f64 = self.ddt_jacobian(s.dn[186][24]);
        let eq130_e1567_d_n25: f64 = self.ddt_jacobian(s.dn[186][25]);
        let eq130_e1567_d_n26: f64 = self.ddt_jacobian(s.dn[186][26]);
        let eq130_e1567_d_n27: f64 = self.ddt_jacobian(s.dn[186][27]);
        let eq130_e1567_d_n28: f64 = self.ddt_jacobian(s.dn[186][28]);
        let eq130_e1567_d_n29: f64 = self.ddt_jacobian(s.dn[186][29]);
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1570_d_n2: f64 = p.p355;
        let eq130_e1570_d_n12: f64 = (-p.p355);
        let eq130_e1571: f64 = self.eval_ddt(127, eq130_e1570);
        let eq130_e1571_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n2: f64 = self.ddt_jacobian(eq130_e1570_d_n2);
        let eq130_e1571_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n7: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n12: f64 = self.ddt_jacobian(eq130_e1570_d_n12);
        let eq130_e1571_d_n13: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq130_e1571_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);
        let eq130_e1572_d_n0: f64 = (eq130_e1567_d_n0 + eq130_e1571_d_n0);
        let eq130_e1572_d_n1: f64 = (eq130_e1567_d_n1 + eq130_e1571_d_n1);
        let eq130_e1572_d_n2: f64 = (eq130_e1567_d_n2 + eq130_e1571_d_n2);
        let eq130_e1572_d_n3: f64 = (eq130_e1567_d_n3 + eq130_e1571_d_n3);
        let eq130_e1572_d_n4: f64 = (eq130_e1567_d_n4 + eq130_e1571_d_n4);
        let eq130_e1572_d_n5: f64 = (eq130_e1567_d_n5 + eq130_e1571_d_n5);
        let eq130_e1572_d_n6: f64 = (eq130_e1567_d_n6 + eq130_e1571_d_n6);
        let eq130_e1572_d_n7: f64 = (eq130_e1567_d_n7 + eq130_e1571_d_n7);
        let eq130_e1572_d_n8: f64 = (eq130_e1567_d_n8 + eq130_e1571_d_n8);
        let eq130_e1572_d_n9: f64 = (eq130_e1567_d_n9 + eq130_e1571_d_n9);
        let eq130_e1572_d_n10: f64 = (eq130_e1567_d_n10 + eq130_e1571_d_n10);
        let eq130_e1572_d_n11: f64 = (eq130_e1567_d_n11 + eq130_e1571_d_n11);
        let eq130_e1572_d_n12: f64 = (eq130_e1567_d_n12 + eq130_e1571_d_n12);
        let eq130_e1572_d_n13: f64 = (eq130_e1567_d_n13 + eq130_e1571_d_n13);
        let eq130_e1572_d_n14: f64 = (eq130_e1567_d_n14 + eq130_e1571_d_n14);
        let eq130_e1572_d_n15: f64 = (eq130_e1567_d_n15 + eq130_e1571_d_n15);
        let eq130_e1572_d_n16: f64 = (eq130_e1567_d_n16 + eq130_e1571_d_n16);
        let eq130_e1572_d_n17: f64 = (eq130_e1567_d_n17 + eq130_e1571_d_n17);
        let eq130_e1572_d_n18: f64 = (eq130_e1567_d_n18 + eq130_e1571_d_n18);
        let eq130_e1572_d_n19: f64 = (eq130_e1567_d_n19 + eq130_e1571_d_n19);
        let eq130_e1572_d_n20: f64 = (eq130_e1567_d_n20 + eq130_e1571_d_n20);
        let eq130_e1572_d_n21: f64 = (eq130_e1567_d_n21 + eq130_e1571_d_n21);
        let eq130_e1572_d_n22: f64 = (eq130_e1567_d_n22 + eq130_e1571_d_n22);
        let eq130_e1572_d_n23: f64 = (eq130_e1567_d_n23 + eq130_e1571_d_n23);
        let eq130_e1572_d_n24: f64 = (eq130_e1567_d_n24 + eq130_e1571_d_n24);
        let eq130_e1572_d_n25: f64 = (eq130_e1567_d_n25 + eq130_e1571_d_n25);
        let eq130_e1572_d_n26: f64 = (eq130_e1567_d_n26 + eq130_e1571_d_n26);
        let eq130_e1572_d_n27: f64 = (eq130_e1567_d_n27 + eq130_e1571_d_n27);
        let eq130_e1572_d_n28: f64 = (eq130_e1567_d_n28 + eq130_e1571_d_n28);
        let eq130_e1572_d_n29: f64 = (eq130_e1567_d_n29 + eq130_e1571_d_n29);
        (eq130_e1572, eq130_e1572_d_n0, eq130_e1572_d_n1, eq130_e1572_d_n2, eq130_e1572_d_n3, eq130_e1572_d_n4, eq130_e1572_d_n5, eq130_e1572_d_n6, eq130_e1572_d_n7, eq130_e1572_d_n8, eq130_e1572_d_n9, eq130_e1572_d_n10, eq130_e1572_d_n11, eq130_e1572_d_n12, eq130_e1572_d_n13, eq130_e1572_d_n14, eq130_e1572_d_n15, eq130_e1572_d_n16, eq130_e1572_d_n17, eq130_e1572_d_n18, eq130_e1572_d_n19, eq130_e1572_d_n20, eq130_e1572_d_n21, eq130_e1572_d_n22, eq130_e1572_d_n23, eq130_e1572_d_n24, eq130_e1572_d_n25, eq130_e1572_d_n26, eq130_e1572_d_n27, eq130_e1572_d_n28, eq130_e1572_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1574;
        let eq130_node_derivatives: [f64; 30] = [eq130_e1574_d_n0, eq130_e1574_d_n1, eq130_e1574_d_n2, eq130_e1574_d_n3, eq130_e1574_d_n4, eq130_e1574_d_n5, eq130_e1574_d_n6, eq130_e1574_d_n7, eq130_e1574_d_n8, eq130_e1574_d_n9, eq130_e1574_d_n10, eq130_e1574_d_n11, eq130_e1574_d_n12, eq130_e1574_d_n13, eq130_e1574_d_n14, eq130_e1574_d_n15, eq130_e1574_d_n16, eq130_e1574_d_n17, eq130_e1574_d_n18, eq130_e1574_d_n19, eq130_e1574_d_n20, eq130_e1574_d_n21, eq130_e1574_d_n22, eq130_e1574_d_n23, eq130_e1574_d_n24, eq130_e1574_d_n25, eq130_e1574_d_n26, eq130_e1574_d_n27, eq130_e1574_d_n28, eq130_e1574_d_n29];
        let eq130_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[12]),
            self.multiplicity * (eq130_value),
            &nodes,
            &eq130_node_derivatives,
            &branches,
            &eq130_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_transient_equation_131_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq131_e1585, eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29,) = {
    if (!(s.v[1495] != 0.0)) {
        let eq131_e1578: f64 = self.eval_ddt(128, s.v[187]);
        let eq131_e1578_d_n0: f64 = self.ddt_jacobian(s.dn[187][0]);
        let eq131_e1578_d_n1: f64 = self.ddt_jacobian(s.dn[187][1]);
        let eq131_e1578_d_n2: f64 = self.ddt_jacobian(s.dn[187][2]);
        let eq131_e1578_d_n3: f64 = self.ddt_jacobian(s.dn[187][3]);
        let eq131_e1578_d_n4: f64 = self.ddt_jacobian(s.dn[187][4]);
        let eq131_e1578_d_n5: f64 = self.ddt_jacobian(s.dn[187][5]);
        let eq131_e1578_d_n6: f64 = self.ddt_jacobian(s.dn[187][6]);
        let eq131_e1578_d_n7: f64 = self.ddt_jacobian(s.dn[187][7]);
        let eq131_e1578_d_n8: f64 = self.ddt_jacobian(s.dn[187][8]);
        let eq131_e1578_d_n9: f64 = self.ddt_jacobian(s.dn[187][9]);
        let eq131_e1578_d_n10: f64 = self.ddt_jacobian(s.dn[187][10]);
        let eq131_e1578_d_n11: f64 = self.ddt_jacobian(s.dn[187][11]);
        let eq131_e1578_d_n12: f64 = self.ddt_jacobian(s.dn[187][12]);
        let eq131_e1578_d_n13: f64 = self.ddt_jacobian(s.dn[187][13]);
        let eq131_e1578_d_n14: f64 = self.ddt_jacobian(s.dn[187][14]);
        let eq131_e1578_d_n15: f64 = self.ddt_jacobian(s.dn[187][15]);
        let eq131_e1578_d_n16: f64 = self.ddt_jacobian(s.dn[187][16]);
        let eq131_e1578_d_n17: f64 = self.ddt_jacobian(s.dn[187][17]);
        let eq131_e1578_d_n18: f64 = self.ddt_jacobian(s.dn[187][18]);
        let eq131_e1578_d_n19: f64 = self.ddt_jacobian(s.dn[187][19]);
        let eq131_e1578_d_n20: f64 = self.ddt_jacobian(s.dn[187][20]);
        let eq131_e1578_d_n21: f64 = self.ddt_jacobian(s.dn[187][21]);
        let eq131_e1578_d_n22: f64 = self.ddt_jacobian(s.dn[187][22]);
        let eq131_e1578_d_n23: f64 = self.ddt_jacobian(s.dn[187][23]);
        let eq131_e1578_d_n24: f64 = self.ddt_jacobian(s.dn[187][24]);
        let eq131_e1578_d_n25: f64 = self.ddt_jacobian(s.dn[187][25]);
        let eq131_e1578_d_n26: f64 = self.ddt_jacobian(s.dn[187][26]);
        let eq131_e1578_d_n27: f64 = self.ddt_jacobian(s.dn[187][27]);
        let eq131_e1578_d_n28: f64 = self.ddt_jacobian(s.dn[187][28]);
        let eq131_e1578_d_n29: f64 = self.ddt_jacobian(s.dn[187][29]);
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1581_d_n7: f64 = p.p355;
        let eq131_e1581_d_n13: f64 = (-p.p355);
        let eq131_e1582: f64 = self.eval_ddt(129, eq131_e1581);
        let eq131_e1582_d_n0: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n1: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n2: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n3: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n4: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n5: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n6: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n7: f64 = self.ddt_jacobian(eq131_e1581_d_n7);
        let eq131_e1582_d_n8: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n9: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n10: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n11: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n12: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n13: f64 = self.ddt_jacobian(eq131_e1581_d_n13);
        let eq131_e1582_d_n14: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n15: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n16: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n17: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n18: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n19: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n20: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n21: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n22: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n23: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n24: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n25: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n26: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n27: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n28: f64 = self.ddt_jacobian(0.0);
        let eq131_e1582_d_n29: f64 = self.ddt_jacobian(0.0);
        let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);
        let eq131_e1583_d_n0: f64 = (eq131_e1578_d_n0 + eq131_e1582_d_n0);
        let eq131_e1583_d_n1: f64 = (eq131_e1578_d_n1 + eq131_e1582_d_n1);
        let eq131_e1583_d_n2: f64 = (eq131_e1578_d_n2 + eq131_e1582_d_n2);
        let eq131_e1583_d_n3: f64 = (eq131_e1578_d_n3 + eq131_e1582_d_n3);
        let eq131_e1583_d_n4: f64 = (eq131_e1578_d_n4 + eq131_e1582_d_n4);
        let eq131_e1583_d_n5: f64 = (eq131_e1578_d_n5 + eq131_e1582_d_n5);
        let eq131_e1583_d_n6: f64 = (eq131_e1578_d_n6 + eq131_e1582_d_n6);
        let eq131_e1583_d_n7: f64 = (eq131_e1578_d_n7 + eq131_e1582_d_n7);
        let eq131_e1583_d_n8: f64 = (eq131_e1578_d_n8 + eq131_e1582_d_n8);
        let eq131_e1583_d_n9: f64 = (eq131_e1578_d_n9 + eq131_e1582_d_n9);
        let eq131_e1583_d_n10: f64 = (eq131_e1578_d_n10 + eq131_e1582_d_n10);
        let eq131_e1583_d_n11: f64 = (eq131_e1578_d_n11 + eq131_e1582_d_n11);
        let eq131_e1583_d_n12: f64 = (eq131_e1578_d_n12 + eq131_e1582_d_n12);
        let eq131_e1583_d_n13: f64 = (eq131_e1578_d_n13 + eq131_e1582_d_n13);
        let eq131_e1583_d_n14: f64 = (eq131_e1578_d_n14 + eq131_e1582_d_n14);
        let eq131_e1583_d_n15: f64 = (eq131_e1578_d_n15 + eq131_e1582_d_n15);
        let eq131_e1583_d_n16: f64 = (eq131_e1578_d_n16 + eq131_e1582_d_n16);
        let eq131_e1583_d_n17: f64 = (eq131_e1578_d_n17 + eq131_e1582_d_n17);
        let eq131_e1583_d_n18: f64 = (eq131_e1578_d_n18 + eq131_e1582_d_n18);
        let eq131_e1583_d_n19: f64 = (eq131_e1578_d_n19 + eq131_e1582_d_n19);
        let eq131_e1583_d_n20: f64 = (eq131_e1578_d_n20 + eq131_e1582_d_n20);
        let eq131_e1583_d_n21: f64 = (eq131_e1578_d_n21 + eq131_e1582_d_n21);
        let eq131_e1583_d_n22: f64 = (eq131_e1578_d_n22 + eq131_e1582_d_n22);
        let eq131_e1583_d_n23: f64 = (eq131_e1578_d_n23 + eq131_e1582_d_n23);
        let eq131_e1583_d_n24: f64 = (eq131_e1578_d_n24 + eq131_e1582_d_n24);
        let eq131_e1583_d_n25: f64 = (eq131_e1578_d_n25 + eq131_e1582_d_n25);
        let eq131_e1583_d_n26: f64 = (eq131_e1578_d_n26 + eq131_e1582_d_n26);
        let eq131_e1583_d_n27: f64 = (eq131_e1578_d_n27 + eq131_e1582_d_n27);
        let eq131_e1583_d_n28: f64 = (eq131_e1578_d_n28 + eq131_e1582_d_n28);
        let eq131_e1583_d_n29: f64 = (eq131_e1578_d_n29 + eq131_e1582_d_n29);
        (eq131_e1583, eq131_e1583_d_n0, eq131_e1583_d_n1, eq131_e1583_d_n2, eq131_e1583_d_n3, eq131_e1583_d_n4, eq131_e1583_d_n5, eq131_e1583_d_n6, eq131_e1583_d_n7, eq131_e1583_d_n8, eq131_e1583_d_n9, eq131_e1583_d_n10, eq131_e1583_d_n11, eq131_e1583_d_n12, eq131_e1583_d_n13, eq131_e1583_d_n14, eq131_e1583_d_n15, eq131_e1583_d_n16, eq131_e1583_d_n17, eq131_e1583_d_n18, eq131_e1583_d_n19, eq131_e1583_d_n20, eq131_e1583_d_n21, eq131_e1583_d_n22, eq131_e1583_d_n23, eq131_e1583_d_n24, eq131_e1583_d_n25, eq131_e1583_d_n26, eq131_e1583_d_n27, eq131_e1583_d_n28, eq131_e1583_d_n29,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1585;
        let eq131_node_derivatives: [f64; 30] = [eq131_e1585_d_n0, eq131_e1585_d_n1, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n5, eq131_e1585_d_n6, eq131_e1585_d_n7, eq131_e1585_d_n8, eq131_e1585_d_n9, eq131_e1585_d_n10, eq131_e1585_d_n11, eq131_e1585_d_n12, eq131_e1585_d_n13, eq131_e1585_d_n14, eq131_e1585_d_n15, eq131_e1585_d_n16, eq131_e1585_d_n17, eq131_e1585_d_n18, eq131_e1585_d_n19, eq131_e1585_d_n20, eq131_e1585_d_n21, eq131_e1585_d_n22, eq131_e1585_d_n23, eq131_e1585_d_n24, eq131_e1585_d_n25, eq131_e1585_d_n26, eq131_e1585_d_n27, eq131_e1585_d_n28, eq131_e1585_d_n29];
        let eq131_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[13]),
            self.multiplicity * (eq131_value),
            &nodes,
            &eq131_node_derivatives,
            &branches,
            &eq131_branch_derivatives,
            self.multiplicity,
        );
    }
}

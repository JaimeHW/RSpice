#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let eq119_e1529: f64 = (p.p250 * s.v[161]);
        let eq119_e1529_d_n0: f64 = (p.p250 * s.dn[161][0]);
        let eq119_e1529_d_n1: f64 = (p.p250 * s.dn[161][1]);
        let eq119_e1529_d_n2: f64 = (p.p250 * s.dn[161][2]);
        let eq119_e1529_d_n3: f64 = (p.p250 * s.dn[161][3]);
        let eq119_e1529_d_n4: f64 = (p.p250 * s.dn[161][4]);
        let eq119_e1529_d_n5: f64 = (p.p250 * s.dn[161][5]);
        let eq119_e1529_d_n6: f64 = (p.p250 * s.dn[161][6]);
        let eq119_e1529_d_n7: f64 = (p.p250 * s.dn[161][7]);
        let eq119_e1529_d_n8: f64 = (p.p250 * s.dn[161][8]);
        let eq119_e1529_d_n9: f64 = (p.p250 * s.dn[161][9]);
        let eq119_e1529_d_n10: f64 = (p.p250 * s.dn[161][10]);
        let eq119_e1529_d_n11: f64 = (p.p250 * s.dn[161][11]);
        let eq119_e1529_d_n12: f64 = (p.p250 * s.dn[161][12]);
        let eq119_e1529_d_n13: f64 = (p.p250 * s.dn[161][13]);
        let eq119_e1529_d_n14: f64 = (p.p250 * s.dn[161][14]);
        let eq119_e1529_d_n15: f64 = (p.p250 * s.dn[161][15]);
        let eq119_e1529_d_n16: f64 = (p.p250 * s.dn[161][16]);
        let eq119_e1529_d_n17: f64 = (p.p250 * s.dn[161][17]);
        let eq119_e1529_d_n18: f64 = (p.p250 * s.dn[161][18]);
        let eq119_e1529_d_n19: f64 = (p.p250 * s.dn[161][19]);
        let eq119_e1529_d_n20: f64 = (p.p250 * s.dn[161][20]);
        let eq119_e1529_d_n21: f64 = (p.p250 * s.dn[161][21]);
        let eq119_e1529_d_n22: f64 = (p.p250 * s.dn[161][22]);
        let eq119_e1530: f64 = self.eval_ddt(18, eq119_e1529);
        let eq119_e1530_d_n0: f64 = self.ddt_jacobian(eq119_e1529_d_n0);
        let eq119_e1530_d_n1: f64 = self.ddt_jacobian(eq119_e1529_d_n1);
        let eq119_e1530_d_n2: f64 = self.ddt_jacobian(eq119_e1529_d_n2);
        let eq119_e1530_d_n3: f64 = self.ddt_jacobian(eq119_e1529_d_n3);
        let eq119_e1530_d_n4: f64 = self.ddt_jacobian(eq119_e1529_d_n4);
        let eq119_e1530_d_n5: f64 = self.ddt_jacobian(eq119_e1529_d_n5);
        let eq119_e1530_d_n6: f64 = self.ddt_jacobian(eq119_e1529_d_n6);
        let eq119_e1530_d_n7: f64 = self.ddt_jacobian(eq119_e1529_d_n7);
        let eq119_e1530_d_n8: f64 = self.ddt_jacobian(eq119_e1529_d_n8);
        let eq119_e1530_d_n9: f64 = self.ddt_jacobian(eq119_e1529_d_n9);
        let eq119_e1530_d_n10: f64 = self.ddt_jacobian(eq119_e1529_d_n10);
        let eq119_e1530_d_n11: f64 = self.ddt_jacobian(eq119_e1529_d_n11);
        let eq119_e1530_d_n12: f64 = self.ddt_jacobian(eq119_e1529_d_n12);
        let eq119_e1530_d_n13: f64 = self.ddt_jacobian(eq119_e1529_d_n13);
        let eq119_e1530_d_n14: f64 = self.ddt_jacobian(eq119_e1529_d_n14);
        let eq119_e1530_d_n15: f64 = self.ddt_jacobian(eq119_e1529_d_n15);
        let eq119_e1530_d_n16: f64 = self.ddt_jacobian(eq119_e1529_d_n16);
        let eq119_e1530_d_n17: f64 = self.ddt_jacobian(eq119_e1529_d_n17);
        let eq119_e1530_d_n18: f64 = self.ddt_jacobian(eq119_e1529_d_n18);
        let eq119_e1530_d_n19: f64 = self.ddt_jacobian(eq119_e1529_d_n19);
        let eq119_e1530_d_n20: f64 = self.ddt_jacobian(eq119_e1529_d_n20);
        let eq119_e1530_d_n21: f64 = self.ddt_jacobian(eq119_e1529_d_n21);
        let eq119_e1530_d_n22: f64 = self.ddt_jacobian(eq119_e1529_d_n22);
        let eq119_e1531: f64 = (p.p7 * eq119_e1530);
        let eq119_e1531_d_n0: f64 = (p.p7 * eq119_e1530_d_n0);
        let eq119_e1531_d_n1: f64 = (p.p7 * eq119_e1530_d_n1);
        let eq119_e1531_d_n2: f64 = (p.p7 * eq119_e1530_d_n2);
        let eq119_e1531_d_n3: f64 = (p.p7 * eq119_e1530_d_n3);
        let eq119_e1531_d_n4: f64 = (p.p7 * eq119_e1530_d_n4);
        let eq119_e1531_d_n5: f64 = (p.p7 * eq119_e1530_d_n5);
        let eq119_e1531_d_n6: f64 = (p.p7 * eq119_e1530_d_n6);
        let eq119_e1531_d_n7: f64 = (p.p7 * eq119_e1530_d_n7);
        let eq119_e1531_d_n8: f64 = (p.p7 * eq119_e1530_d_n8);
        let eq119_e1531_d_n9: f64 = (p.p7 * eq119_e1530_d_n9);
        let eq119_e1531_d_n10: f64 = (p.p7 * eq119_e1530_d_n10);
        let eq119_e1531_d_n11: f64 = (p.p7 * eq119_e1530_d_n11);
        let eq119_e1531_d_n12: f64 = (p.p7 * eq119_e1530_d_n12);
        let eq119_e1531_d_n13: f64 = (p.p7 * eq119_e1530_d_n13);
        let eq119_e1531_d_n14: f64 = (p.p7 * eq119_e1530_d_n14);
        let eq119_e1531_d_n15: f64 = (p.p7 * eq119_e1530_d_n15);
        let eq119_e1531_d_n16: f64 = (p.p7 * eq119_e1530_d_n16);
        let eq119_e1531_d_n17: f64 = (p.p7 * eq119_e1530_d_n17);
        let eq119_e1531_d_n18: f64 = (p.p7 * eq119_e1530_d_n18);
        let eq119_e1531_d_n19: f64 = (p.p7 * eq119_e1530_d_n19);
        let eq119_e1531_d_n20: f64 = (p.p7 * eq119_e1530_d_n20);
        let eq119_e1531_d_n21: f64 = (p.p7 * eq119_e1530_d_n21);
        let eq119_e1531_d_n22: f64 = (p.p7 * eq119_e1530_d_n22);
        let eq119_value: f64 = eq119_e1531;
        let eq119_node_derivatives: [f64; 23] = [eq119_e1531_d_n0, eq119_e1531_d_n1, eq119_e1531_d_n2, eq119_e1531_d_n3, eq119_e1531_d_n4, eq119_e1531_d_n5, eq119_e1531_d_n6, eq119_e1531_d_n7, eq119_e1531_d_n8, eq119_e1531_d_n9, eq119_e1531_d_n10, eq119_e1531_d_n11, eq119_e1531_d_n12, eq119_e1531_d_n13, eq119_e1531_d_n14, eq119_e1531_d_n15, eq119_e1531_d_n16, eq119_e1531_d_n17, eq119_e1531_d_n18, eq119_e1531_d_n19, eq119_e1531_d_n20, eq119_e1531_d_n21, eq119_e1531_d_n22];
        let eq119_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            self.multiplicity * (eq119_value),
            &nodes,
            &eq119_node_derivatives,
            &branches,
            &eq119_branch_derivatives,
            self.multiplicity,
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
        let (eq120_e1540, eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22,) = {
    if ((s.v[570] != 0.0) && (s.v[571] != 0.0)) {
        let eq120_e1537: f64 = self.eval_ddt(19, s.v[229]);
        let eq120_e1537_d_n0: f64 = self.ddt_jacobian(s.dn[229][0]);
        let eq120_e1537_d_n1: f64 = self.ddt_jacobian(s.dn[229][1]);
        let eq120_e1537_d_n2: f64 = self.ddt_jacobian(s.dn[229][2]);
        let eq120_e1537_d_n3: f64 = self.ddt_jacobian(s.dn[229][3]);
        let eq120_e1537_d_n4: f64 = self.ddt_jacobian(s.dn[229][4]);
        let eq120_e1537_d_n5: f64 = self.ddt_jacobian(s.dn[229][5]);
        let eq120_e1537_d_n6: f64 = self.ddt_jacobian(s.dn[229][6]);
        let eq120_e1537_d_n7: f64 = self.ddt_jacobian(s.dn[229][7]);
        let eq120_e1537_d_n8: f64 = self.ddt_jacobian(s.dn[229][8]);
        let eq120_e1537_d_n9: f64 = self.ddt_jacobian(s.dn[229][9]);
        let eq120_e1537_d_n10: f64 = self.ddt_jacobian(s.dn[229][10]);
        let eq120_e1537_d_n11: f64 = self.ddt_jacobian(s.dn[229][11]);
        let eq120_e1537_d_n12: f64 = self.ddt_jacobian(s.dn[229][12]);
        let eq120_e1537_d_n13: f64 = self.ddt_jacobian(s.dn[229][13]);
        let eq120_e1537_d_n14: f64 = self.ddt_jacobian(s.dn[229][14]);
        let eq120_e1537_d_n15: f64 = self.ddt_jacobian(s.dn[229][15]);
        let eq120_e1537_d_n16: f64 = self.ddt_jacobian(s.dn[229][16]);
        let eq120_e1537_d_n17: f64 = self.ddt_jacobian(s.dn[229][17]);
        let eq120_e1537_d_n18: f64 = self.ddt_jacobian(s.dn[229][18]);
        let eq120_e1537_d_n19: f64 = self.ddt_jacobian(s.dn[229][19]);
        let eq120_e1537_d_n20: f64 = self.ddt_jacobian(s.dn[229][20]);
        let eq120_e1537_d_n21: f64 = self.ddt_jacobian(s.dn[229][21]);
        let eq120_e1537_d_n22: f64 = self.ddt_jacobian(s.dn[229][22]);
        let eq120_e1538: f64 = (p.p7 * eq120_e1537);
        let eq120_e1538_d_n0: f64 = (p.p7 * eq120_e1537_d_n0);
        let eq120_e1538_d_n1: f64 = (p.p7 * eq120_e1537_d_n1);
        let eq120_e1538_d_n2: f64 = (p.p7 * eq120_e1537_d_n2);
        let eq120_e1538_d_n3: f64 = (p.p7 * eq120_e1537_d_n3);
        let eq120_e1538_d_n4: f64 = (p.p7 * eq120_e1537_d_n4);
        let eq120_e1538_d_n5: f64 = (p.p7 * eq120_e1537_d_n5);
        let eq120_e1538_d_n6: f64 = (p.p7 * eq120_e1537_d_n6);
        let eq120_e1538_d_n7: f64 = (p.p7 * eq120_e1537_d_n7);
        let eq120_e1538_d_n8: f64 = (p.p7 * eq120_e1537_d_n8);
        let eq120_e1538_d_n9: f64 = (p.p7 * eq120_e1537_d_n9);
        let eq120_e1538_d_n10: f64 = (p.p7 * eq120_e1537_d_n10);
        let eq120_e1538_d_n11: f64 = (p.p7 * eq120_e1537_d_n11);
        let eq120_e1538_d_n12: f64 = (p.p7 * eq120_e1537_d_n12);
        let eq120_e1538_d_n13: f64 = (p.p7 * eq120_e1537_d_n13);
        let eq120_e1538_d_n14: f64 = (p.p7 * eq120_e1537_d_n14);
        let eq120_e1538_d_n15: f64 = (p.p7 * eq120_e1537_d_n15);
        let eq120_e1538_d_n16: f64 = (p.p7 * eq120_e1537_d_n16);
        let eq120_e1538_d_n17: f64 = (p.p7 * eq120_e1537_d_n17);
        let eq120_e1538_d_n18: f64 = (p.p7 * eq120_e1537_d_n18);
        let eq120_e1538_d_n19: f64 = (p.p7 * eq120_e1537_d_n19);
        let eq120_e1538_d_n20: f64 = (p.p7 * eq120_e1537_d_n20);
        let eq120_e1538_d_n21: f64 = (p.p7 * eq120_e1537_d_n21);
        let eq120_e1538_d_n22: f64 = (p.p7 * eq120_e1537_d_n22);
        (eq120_e1538, eq120_e1538_d_n0, eq120_e1538_d_n1, eq120_e1538_d_n2, eq120_e1538_d_n3, eq120_e1538_d_n4, eq120_e1538_d_n5, eq120_e1538_d_n6, eq120_e1538_d_n7, eq120_e1538_d_n8, eq120_e1538_d_n9, eq120_e1538_d_n10, eq120_e1538_d_n11, eq120_e1538_d_n12, eq120_e1538_d_n13, eq120_e1538_d_n14, eq120_e1538_d_n15, eq120_e1538_d_n16, eq120_e1538_d_n17, eq120_e1538_d_n18, eq120_e1538_d_n19, eq120_e1538_d_n20, eq120_e1538_d_n21, eq120_e1538_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq120_value: f64 = eq120_e1540;
        let eq120_node_derivatives: [f64; 23] = [eq120_e1540_d_n0, eq120_e1540_d_n1, eq120_e1540_d_n2, eq120_e1540_d_n3, eq120_e1540_d_n4, eq120_e1540_d_n5, eq120_e1540_d_n6, eq120_e1540_d_n7, eq120_e1540_d_n8, eq120_e1540_d_n9, eq120_e1540_d_n10, eq120_e1540_d_n11, eq120_e1540_d_n12, eq120_e1540_d_n13, eq120_e1540_d_n14, eq120_e1540_d_n15, eq120_e1540_d_n16, eq120_e1540_d_n17, eq120_e1540_d_n18, eq120_e1540_d_n19, eq120_e1540_d_n20, eq120_e1540_d_n21, eq120_e1540_d_n22];
        let eq120_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[15]),
            Some(nodes[7]),
            self.multiplicity * (eq120_value),
            &nodes,
            &eq120_node_derivatives,
            &branches,
            &eq120_branch_derivatives,
            self.multiplicity,
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
        let (eq121_e1551, eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22,) = {
    if (((s.v[570] != 0.0) && (s.v[571] != 0.0)) && (s.v[572] != 0.0)) {
        let eq121_e1548: f64 = self.eval_ddt(20, s.v[228]);
        let eq121_e1548_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq121_e1548_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq121_e1548_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq121_e1548_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq121_e1548_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq121_e1548_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq121_e1548_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq121_e1548_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq121_e1548_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq121_e1548_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq121_e1548_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq121_e1548_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq121_e1548_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq121_e1548_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq121_e1548_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq121_e1548_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq121_e1548_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq121_e1548_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq121_e1548_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq121_e1548_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq121_e1548_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq121_e1548_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq121_e1548_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq121_e1549: f64 = (p.p7 * eq121_e1548);
        let eq121_e1549_d_n0: f64 = (p.p7 * eq121_e1548_d_n0);
        let eq121_e1549_d_n1: f64 = (p.p7 * eq121_e1548_d_n1);
        let eq121_e1549_d_n2: f64 = (p.p7 * eq121_e1548_d_n2);
        let eq121_e1549_d_n3: f64 = (p.p7 * eq121_e1548_d_n3);
        let eq121_e1549_d_n4: f64 = (p.p7 * eq121_e1548_d_n4);
        let eq121_e1549_d_n5: f64 = (p.p7 * eq121_e1548_d_n5);
        let eq121_e1549_d_n6: f64 = (p.p7 * eq121_e1548_d_n6);
        let eq121_e1549_d_n7: f64 = (p.p7 * eq121_e1548_d_n7);
        let eq121_e1549_d_n8: f64 = (p.p7 * eq121_e1548_d_n8);
        let eq121_e1549_d_n9: f64 = (p.p7 * eq121_e1548_d_n9);
        let eq121_e1549_d_n10: f64 = (p.p7 * eq121_e1548_d_n10);
        let eq121_e1549_d_n11: f64 = (p.p7 * eq121_e1548_d_n11);
        let eq121_e1549_d_n12: f64 = (p.p7 * eq121_e1548_d_n12);
        let eq121_e1549_d_n13: f64 = (p.p7 * eq121_e1548_d_n13);
        let eq121_e1549_d_n14: f64 = (p.p7 * eq121_e1548_d_n14);
        let eq121_e1549_d_n15: f64 = (p.p7 * eq121_e1548_d_n15);
        let eq121_e1549_d_n16: f64 = (p.p7 * eq121_e1548_d_n16);
        let eq121_e1549_d_n17: f64 = (p.p7 * eq121_e1548_d_n17);
        let eq121_e1549_d_n18: f64 = (p.p7 * eq121_e1548_d_n18);
        let eq121_e1549_d_n19: f64 = (p.p7 * eq121_e1548_d_n19);
        let eq121_e1549_d_n20: f64 = (p.p7 * eq121_e1548_d_n20);
        let eq121_e1549_d_n21: f64 = (p.p7 * eq121_e1548_d_n21);
        let eq121_e1549_d_n22: f64 = (p.p7 * eq121_e1548_d_n22);
        (eq121_e1549, eq121_e1549_d_n0, eq121_e1549_d_n1, eq121_e1549_d_n2, eq121_e1549_d_n3, eq121_e1549_d_n4, eq121_e1549_d_n5, eq121_e1549_d_n6, eq121_e1549_d_n7, eq121_e1549_d_n8, eq121_e1549_d_n9, eq121_e1549_d_n10, eq121_e1549_d_n11, eq121_e1549_d_n12, eq121_e1549_d_n13, eq121_e1549_d_n14, eq121_e1549_d_n15, eq121_e1549_d_n16, eq121_e1549_d_n17, eq121_e1549_d_n18, eq121_e1549_d_n19, eq121_e1549_d_n20, eq121_e1549_d_n21, eq121_e1549_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq121_value: f64 = eq121_e1551;
        let eq121_node_derivatives: [f64; 23] = [eq121_e1551_d_n0, eq121_e1551_d_n1, eq121_e1551_d_n2, eq121_e1551_d_n3, eq121_e1551_d_n4, eq121_e1551_d_n5, eq121_e1551_d_n6, eq121_e1551_d_n7, eq121_e1551_d_n8, eq121_e1551_d_n9, eq121_e1551_d_n10, eq121_e1551_d_n11, eq121_e1551_d_n12, eq121_e1551_d_n13, eq121_e1551_d_n14, eq121_e1551_d_n15, eq121_e1551_d_n16, eq121_e1551_d_n17, eq121_e1551_d_n18, eq121_e1551_d_n19, eq121_e1551_d_n20, eq121_e1551_d_n21, eq121_e1551_d_n22];
        let eq121_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let (eq122_e1564, eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22,) = {
    if (((s.v[570] != 0.0) && (s.v[571] != 0.0)) && (s.v[572] != 0.0)) {
        let eq122_e1559: f64 = self.eval_ddt(21, s.v[228]);
        let eq122_e1559_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq122_e1559_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq122_e1559_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq122_e1559_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq122_e1559_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq122_e1559_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq122_e1559_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq122_e1559_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq122_e1559_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq122_e1559_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq122_e1559_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq122_e1559_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq122_e1559_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq122_e1559_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq122_e1559_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq122_e1559_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq122_e1559_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq122_e1559_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq122_e1559_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq122_e1559_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq122_e1559_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq122_e1559_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq122_e1559_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq122_e1560: f64 = (p.p7 * eq122_e1559);
        let eq122_e1560_d_n0: f64 = (p.p7 * eq122_e1559_d_n0);
        let eq122_e1560_d_n1: f64 = (p.p7 * eq122_e1559_d_n1);
        let eq122_e1560_d_n2: f64 = (p.p7 * eq122_e1559_d_n2);
        let eq122_e1560_d_n3: f64 = (p.p7 * eq122_e1559_d_n3);
        let eq122_e1560_d_n4: f64 = (p.p7 * eq122_e1559_d_n4);
        let eq122_e1560_d_n5: f64 = (p.p7 * eq122_e1559_d_n5);
        let eq122_e1560_d_n6: f64 = (p.p7 * eq122_e1559_d_n6);
        let eq122_e1560_d_n7: f64 = (p.p7 * eq122_e1559_d_n7);
        let eq122_e1560_d_n8: f64 = (p.p7 * eq122_e1559_d_n8);
        let eq122_e1560_d_n9: f64 = (p.p7 * eq122_e1559_d_n9);
        let eq122_e1560_d_n10: f64 = (p.p7 * eq122_e1559_d_n10);
        let eq122_e1560_d_n11: f64 = (p.p7 * eq122_e1559_d_n11);
        let eq122_e1560_d_n12: f64 = (p.p7 * eq122_e1559_d_n12);
        let eq122_e1560_d_n13: f64 = (p.p7 * eq122_e1559_d_n13);
        let eq122_e1560_d_n14: f64 = (p.p7 * eq122_e1559_d_n14);
        let eq122_e1560_d_n15: f64 = (p.p7 * eq122_e1559_d_n15);
        let eq122_e1560_d_n16: f64 = (p.p7 * eq122_e1559_d_n16);
        let eq122_e1560_d_n17: f64 = (p.p7 * eq122_e1559_d_n17);
        let eq122_e1560_d_n18: f64 = (p.p7 * eq122_e1559_d_n18);
        let eq122_e1560_d_n19: f64 = (p.p7 * eq122_e1559_d_n19);
        let eq122_e1560_d_n20: f64 = (p.p7 * eq122_e1559_d_n20);
        let eq122_e1560_d_n21: f64 = (p.p7 * eq122_e1559_d_n21);
        let eq122_e1560_d_n22: f64 = (p.p7 * eq122_e1559_d_n22);
        let eq122_e1562: f64 = (eq122_e1560 * p.p246);
        let eq122_e1562_d_n0: f64 = (eq122_e1560_d_n0 * p.p246);
        let eq122_e1562_d_n1: f64 = (eq122_e1560_d_n1 * p.p246);
        let eq122_e1562_d_n2: f64 = (eq122_e1560_d_n2 * p.p246);
        let eq122_e1562_d_n3: f64 = (eq122_e1560_d_n3 * p.p246);
        let eq122_e1562_d_n4: f64 = (eq122_e1560_d_n4 * p.p246);
        let eq122_e1562_d_n5: f64 = (eq122_e1560_d_n5 * p.p246);
        let eq122_e1562_d_n6: f64 = (eq122_e1560_d_n6 * p.p246);
        let eq122_e1562_d_n7: f64 = (eq122_e1560_d_n7 * p.p246);
        let eq122_e1562_d_n8: f64 = (eq122_e1560_d_n8 * p.p246);
        let eq122_e1562_d_n9: f64 = (eq122_e1560_d_n9 * p.p246);
        let eq122_e1562_d_n10: f64 = (eq122_e1560_d_n10 * p.p246);
        let eq122_e1562_d_n11: f64 = (eq122_e1560_d_n11 * p.p246);
        let eq122_e1562_d_n12: f64 = (eq122_e1560_d_n12 * p.p246);
        let eq122_e1562_d_n13: f64 = (eq122_e1560_d_n13 * p.p246);
        let eq122_e1562_d_n14: f64 = (eq122_e1560_d_n14 * p.p246);
        let eq122_e1562_d_n15: f64 = (eq122_e1560_d_n15 * p.p246);
        let eq122_e1562_d_n16: f64 = (eq122_e1560_d_n16 * p.p246);
        let eq122_e1562_d_n17: f64 = (eq122_e1560_d_n17 * p.p246);
        let eq122_e1562_d_n18: f64 = (eq122_e1560_d_n18 * p.p246);
        let eq122_e1562_d_n19: f64 = (eq122_e1560_d_n19 * p.p246);
        let eq122_e1562_d_n20: f64 = (eq122_e1560_d_n20 * p.p246);
        let eq122_e1562_d_n21: f64 = (eq122_e1560_d_n21 * p.p246);
        let eq122_e1562_d_n22: f64 = (eq122_e1560_d_n22 * p.p246);
        (eq122_e1562, eq122_e1562_d_n0, eq122_e1562_d_n1, eq122_e1562_d_n2, eq122_e1562_d_n3, eq122_e1562_d_n4, eq122_e1562_d_n5, eq122_e1562_d_n6, eq122_e1562_d_n7, eq122_e1562_d_n8, eq122_e1562_d_n9, eq122_e1562_d_n10, eq122_e1562_d_n11, eq122_e1562_d_n12, eq122_e1562_d_n13, eq122_e1562_d_n14, eq122_e1562_d_n15, eq122_e1562_d_n16, eq122_e1562_d_n17, eq122_e1562_d_n18, eq122_e1562_d_n19, eq122_e1562_d_n20, eq122_e1562_d_n21, eq122_e1562_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1564;
        let eq122_node_derivatives: [f64; 23] = [eq122_e1564_d_n0, eq122_e1564_d_n1, eq122_e1564_d_n2, eq122_e1564_d_n3, eq122_e1564_d_n4, eq122_e1564_d_n5, eq122_e1564_d_n6, eq122_e1564_d_n7, eq122_e1564_d_n8, eq122_e1564_d_n9, eq122_e1564_d_n10, eq122_e1564_d_n11, eq122_e1564_d_n12, eq122_e1564_d_n13, eq122_e1564_d_n14, eq122_e1564_d_n15, eq122_e1564_d_n16, eq122_e1564_d_n17, eq122_e1564_d_n18, eq122_e1564_d_n19, eq122_e1564_d_n20, eq122_e1564_d_n21, eq122_e1564_d_n22];
        let eq122_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
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
        let (eq123_e1576, eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22,) = {
    if (((s.v[570] != 0.0) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
        let eq123_e1573: f64 = self.eval_ddt(22, s.v[228]);
        let eq123_e1573_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq123_e1573_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq123_e1573_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq123_e1573_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq123_e1573_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq123_e1573_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq123_e1573_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq123_e1573_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq123_e1573_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq123_e1573_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq123_e1573_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq123_e1573_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq123_e1573_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq123_e1573_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq123_e1573_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq123_e1573_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq123_e1573_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq123_e1573_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq123_e1573_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq123_e1573_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq123_e1573_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq123_e1573_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq123_e1573_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq123_e1574: f64 = (p.p7 * eq123_e1573);
        let eq123_e1574_d_n0: f64 = (p.p7 * eq123_e1573_d_n0);
        let eq123_e1574_d_n1: f64 = (p.p7 * eq123_e1573_d_n1);
        let eq123_e1574_d_n2: f64 = (p.p7 * eq123_e1573_d_n2);
        let eq123_e1574_d_n3: f64 = (p.p7 * eq123_e1573_d_n3);
        let eq123_e1574_d_n4: f64 = (p.p7 * eq123_e1573_d_n4);
        let eq123_e1574_d_n5: f64 = (p.p7 * eq123_e1573_d_n5);
        let eq123_e1574_d_n6: f64 = (p.p7 * eq123_e1573_d_n6);
        let eq123_e1574_d_n7: f64 = (p.p7 * eq123_e1573_d_n7);
        let eq123_e1574_d_n8: f64 = (p.p7 * eq123_e1573_d_n8);
        let eq123_e1574_d_n9: f64 = (p.p7 * eq123_e1573_d_n9);
        let eq123_e1574_d_n10: f64 = (p.p7 * eq123_e1573_d_n10);
        let eq123_e1574_d_n11: f64 = (p.p7 * eq123_e1573_d_n11);
        let eq123_e1574_d_n12: f64 = (p.p7 * eq123_e1573_d_n12);
        let eq123_e1574_d_n13: f64 = (p.p7 * eq123_e1573_d_n13);
        let eq123_e1574_d_n14: f64 = (p.p7 * eq123_e1573_d_n14);
        let eq123_e1574_d_n15: f64 = (p.p7 * eq123_e1573_d_n15);
        let eq123_e1574_d_n16: f64 = (p.p7 * eq123_e1573_d_n16);
        let eq123_e1574_d_n17: f64 = (p.p7 * eq123_e1573_d_n17);
        let eq123_e1574_d_n18: f64 = (p.p7 * eq123_e1573_d_n18);
        let eq123_e1574_d_n19: f64 = (p.p7 * eq123_e1573_d_n19);
        let eq123_e1574_d_n20: f64 = (p.p7 * eq123_e1573_d_n20);
        let eq123_e1574_d_n21: f64 = (p.p7 * eq123_e1573_d_n21);
        let eq123_e1574_d_n22: f64 = (p.p7 * eq123_e1573_d_n22);
        (eq123_e1574, eq123_e1574_d_n0, eq123_e1574_d_n1, eq123_e1574_d_n2, eq123_e1574_d_n3, eq123_e1574_d_n4, eq123_e1574_d_n5, eq123_e1574_d_n6, eq123_e1574_d_n7, eq123_e1574_d_n8, eq123_e1574_d_n9, eq123_e1574_d_n10, eq123_e1574_d_n11, eq123_e1574_d_n12, eq123_e1574_d_n13, eq123_e1574_d_n14, eq123_e1574_d_n15, eq123_e1574_d_n16, eq123_e1574_d_n17, eq123_e1574_d_n18, eq123_e1574_d_n19, eq123_e1574_d_n20, eq123_e1574_d_n21, eq123_e1574_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq123_value: f64 = eq123_e1576;
        let eq123_node_derivatives: [f64; 23] = [eq123_e1576_d_n0, eq123_e1576_d_n1, eq123_e1576_d_n2, eq123_e1576_d_n3, eq123_e1576_d_n4, eq123_e1576_d_n5, eq123_e1576_d_n6, eq123_e1576_d_n7, eq123_e1576_d_n8, eq123_e1576_d_n9, eq123_e1576_d_n10, eq123_e1576_d_n11, eq123_e1576_d_n12, eq123_e1576_d_n13, eq123_e1576_d_n14, eq123_e1576_d_n15, eq123_e1576_d_n16, eq123_e1576_d_n17, eq123_e1576_d_n18, eq123_e1576_d_n19, eq123_e1576_d_n20, eq123_e1576_d_n21, eq123_e1576_d_n22];
        let eq123_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
            self.multiplicity * (eq123_value),
            &nodes,
            &eq123_node_derivatives,
            &branches,
            &eq123_branch_derivatives,
            self.multiplicity,
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
        let (eq124_e1590, eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22,) = {
    if (((s.v[570] != 0.0) && (s.v[571] != 0.0)) && (!(s.v[572] != 0.0))) {
        let eq124_e1585: f64 = self.eval_ddt(23, s.v[228]);
        let eq124_e1585_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq124_e1585_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq124_e1585_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq124_e1585_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq124_e1585_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq124_e1585_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq124_e1585_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq124_e1585_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq124_e1585_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq124_e1585_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq124_e1585_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq124_e1585_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq124_e1585_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq124_e1585_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq124_e1585_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq124_e1585_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq124_e1585_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq124_e1585_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq124_e1585_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq124_e1585_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq124_e1585_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq124_e1585_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq124_e1585_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq124_e1586: f64 = (p.p7 * eq124_e1585);
        let eq124_e1586_d_n0: f64 = (p.p7 * eq124_e1585_d_n0);
        let eq124_e1586_d_n1: f64 = (p.p7 * eq124_e1585_d_n1);
        let eq124_e1586_d_n2: f64 = (p.p7 * eq124_e1585_d_n2);
        let eq124_e1586_d_n3: f64 = (p.p7 * eq124_e1585_d_n3);
        let eq124_e1586_d_n4: f64 = (p.p7 * eq124_e1585_d_n4);
        let eq124_e1586_d_n5: f64 = (p.p7 * eq124_e1585_d_n5);
        let eq124_e1586_d_n6: f64 = (p.p7 * eq124_e1585_d_n6);
        let eq124_e1586_d_n7: f64 = (p.p7 * eq124_e1585_d_n7);
        let eq124_e1586_d_n8: f64 = (p.p7 * eq124_e1585_d_n8);
        let eq124_e1586_d_n9: f64 = (p.p7 * eq124_e1585_d_n9);
        let eq124_e1586_d_n10: f64 = (p.p7 * eq124_e1585_d_n10);
        let eq124_e1586_d_n11: f64 = (p.p7 * eq124_e1585_d_n11);
        let eq124_e1586_d_n12: f64 = (p.p7 * eq124_e1585_d_n12);
        let eq124_e1586_d_n13: f64 = (p.p7 * eq124_e1585_d_n13);
        let eq124_e1586_d_n14: f64 = (p.p7 * eq124_e1585_d_n14);
        let eq124_e1586_d_n15: f64 = (p.p7 * eq124_e1585_d_n15);
        let eq124_e1586_d_n16: f64 = (p.p7 * eq124_e1585_d_n16);
        let eq124_e1586_d_n17: f64 = (p.p7 * eq124_e1585_d_n17);
        let eq124_e1586_d_n18: f64 = (p.p7 * eq124_e1585_d_n18);
        let eq124_e1586_d_n19: f64 = (p.p7 * eq124_e1585_d_n19);
        let eq124_e1586_d_n20: f64 = (p.p7 * eq124_e1585_d_n20);
        let eq124_e1586_d_n21: f64 = (p.p7 * eq124_e1585_d_n21);
        let eq124_e1586_d_n22: f64 = (p.p7 * eq124_e1585_d_n22);
        let eq124_e1588: f64 = (eq124_e1586 * p.p246);
        let eq124_e1588_d_n0: f64 = (eq124_e1586_d_n0 * p.p246);
        let eq124_e1588_d_n1: f64 = (eq124_e1586_d_n1 * p.p246);
        let eq124_e1588_d_n2: f64 = (eq124_e1586_d_n2 * p.p246);
        let eq124_e1588_d_n3: f64 = (eq124_e1586_d_n3 * p.p246);
        let eq124_e1588_d_n4: f64 = (eq124_e1586_d_n4 * p.p246);
        let eq124_e1588_d_n5: f64 = (eq124_e1586_d_n5 * p.p246);
        let eq124_e1588_d_n6: f64 = (eq124_e1586_d_n6 * p.p246);
        let eq124_e1588_d_n7: f64 = (eq124_e1586_d_n7 * p.p246);
        let eq124_e1588_d_n8: f64 = (eq124_e1586_d_n8 * p.p246);
        let eq124_e1588_d_n9: f64 = (eq124_e1586_d_n9 * p.p246);
        let eq124_e1588_d_n10: f64 = (eq124_e1586_d_n10 * p.p246);
        let eq124_e1588_d_n11: f64 = (eq124_e1586_d_n11 * p.p246);
        let eq124_e1588_d_n12: f64 = (eq124_e1586_d_n12 * p.p246);
        let eq124_e1588_d_n13: f64 = (eq124_e1586_d_n13 * p.p246);
        let eq124_e1588_d_n14: f64 = (eq124_e1586_d_n14 * p.p246);
        let eq124_e1588_d_n15: f64 = (eq124_e1586_d_n15 * p.p246);
        let eq124_e1588_d_n16: f64 = (eq124_e1586_d_n16 * p.p246);
        let eq124_e1588_d_n17: f64 = (eq124_e1586_d_n17 * p.p246);
        let eq124_e1588_d_n18: f64 = (eq124_e1586_d_n18 * p.p246);
        let eq124_e1588_d_n19: f64 = (eq124_e1586_d_n19 * p.p246);
        let eq124_e1588_d_n20: f64 = (eq124_e1586_d_n20 * p.p246);
        let eq124_e1588_d_n21: f64 = (eq124_e1586_d_n21 * p.p246);
        let eq124_e1588_d_n22: f64 = (eq124_e1586_d_n22 * p.p246);
        (eq124_e1588, eq124_e1588_d_n0, eq124_e1588_d_n1, eq124_e1588_d_n2, eq124_e1588_d_n3, eq124_e1588_d_n4, eq124_e1588_d_n5, eq124_e1588_d_n6, eq124_e1588_d_n7, eq124_e1588_d_n8, eq124_e1588_d_n9, eq124_e1588_d_n10, eq124_e1588_d_n11, eq124_e1588_d_n12, eq124_e1588_d_n13, eq124_e1588_d_n14, eq124_e1588_d_n15, eq124_e1588_d_n16, eq124_e1588_d_n17, eq124_e1588_d_n18, eq124_e1588_d_n19, eq124_e1588_d_n20, eq124_e1588_d_n21, eq124_e1588_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1590;
        let eq124_node_derivatives: [f64; 23] = [eq124_e1590_d_n0, eq124_e1590_d_n1, eq124_e1590_d_n2, eq124_e1590_d_n3, eq124_e1590_d_n4, eq124_e1590_d_n5, eq124_e1590_d_n6, eq124_e1590_d_n7, eq124_e1590_d_n8, eq124_e1590_d_n9, eq124_e1590_d_n10, eq124_e1590_d_n11, eq124_e1590_d_n12, eq124_e1590_d_n13, eq124_e1590_d_n14, eq124_e1590_d_n15, eq124_e1590_d_n16, eq124_e1590_d_n17, eq124_e1590_d_n18, eq124_e1590_d_n19, eq124_e1590_d_n20, eq124_e1590_d_n21, eq124_e1590_d_n22];
        let eq124_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let (eq125_e1601, eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22,) = {
    if ((s.v[570] != 0.0) && (s.v[571] != 0.0)) {
        let eq125_e1597: f64 = (p.p251 * s.v[228]);
        let eq125_e1597_d_n0: f64 = (p.p251 * s.dn[228][0]);
        let eq125_e1597_d_n1: f64 = (p.p251 * s.dn[228][1]);
        let eq125_e1597_d_n2: f64 = (p.p251 * s.dn[228][2]);
        let eq125_e1597_d_n3: f64 = (p.p251 * s.dn[228][3]);
        let eq125_e1597_d_n4: f64 = (p.p251 * s.dn[228][4]);
        let eq125_e1597_d_n5: f64 = (p.p251 * s.dn[228][5]);
        let eq125_e1597_d_n6: f64 = (p.p251 * s.dn[228][6]);
        let eq125_e1597_d_n7: f64 = (p.p251 * s.dn[228][7]);
        let eq125_e1597_d_n8: f64 = (p.p251 * s.dn[228][8]);
        let eq125_e1597_d_n9: f64 = (p.p251 * s.dn[228][9]);
        let eq125_e1597_d_n10: f64 = (p.p251 * s.dn[228][10]);
        let eq125_e1597_d_n11: f64 = (p.p251 * s.dn[228][11]);
        let eq125_e1597_d_n12: f64 = (p.p251 * s.dn[228][12]);
        let eq125_e1597_d_n13: f64 = (p.p251 * s.dn[228][13]);
        let eq125_e1597_d_n14: f64 = (p.p251 * s.dn[228][14]);
        let eq125_e1597_d_n15: f64 = (p.p251 * s.dn[228][15]);
        let eq125_e1597_d_n16: f64 = (p.p251 * s.dn[228][16]);
        let eq125_e1597_d_n17: f64 = (p.p251 * s.dn[228][17]);
        let eq125_e1597_d_n18: f64 = (p.p251 * s.dn[228][18]);
        let eq125_e1597_d_n19: f64 = (p.p251 * s.dn[228][19]);
        let eq125_e1597_d_n20: f64 = (p.p251 * s.dn[228][20]);
        let eq125_e1597_d_n21: f64 = (p.p251 * s.dn[228][21]);
        let eq125_e1597_d_n22: f64 = (p.p251 * s.dn[228][22]);
        let eq125_e1598: f64 = self.eval_ddt(24, eq125_e1597);
        let eq125_e1598_d_n0: f64 = self.ddt_jacobian(eq125_e1597_d_n0);
        let eq125_e1598_d_n1: f64 = self.ddt_jacobian(eq125_e1597_d_n1);
        let eq125_e1598_d_n2: f64 = self.ddt_jacobian(eq125_e1597_d_n2);
        let eq125_e1598_d_n3: f64 = self.ddt_jacobian(eq125_e1597_d_n3);
        let eq125_e1598_d_n4: f64 = self.ddt_jacobian(eq125_e1597_d_n4);
        let eq125_e1598_d_n5: f64 = self.ddt_jacobian(eq125_e1597_d_n5);
        let eq125_e1598_d_n6: f64 = self.ddt_jacobian(eq125_e1597_d_n6);
        let eq125_e1598_d_n7: f64 = self.ddt_jacobian(eq125_e1597_d_n7);
        let eq125_e1598_d_n8: f64 = self.ddt_jacobian(eq125_e1597_d_n8);
        let eq125_e1598_d_n9: f64 = self.ddt_jacobian(eq125_e1597_d_n9);
        let eq125_e1598_d_n10: f64 = self.ddt_jacobian(eq125_e1597_d_n10);
        let eq125_e1598_d_n11: f64 = self.ddt_jacobian(eq125_e1597_d_n11);
        let eq125_e1598_d_n12: f64 = self.ddt_jacobian(eq125_e1597_d_n12);
        let eq125_e1598_d_n13: f64 = self.ddt_jacobian(eq125_e1597_d_n13);
        let eq125_e1598_d_n14: f64 = self.ddt_jacobian(eq125_e1597_d_n14);
        let eq125_e1598_d_n15: f64 = self.ddt_jacobian(eq125_e1597_d_n15);
        let eq125_e1598_d_n16: f64 = self.ddt_jacobian(eq125_e1597_d_n16);
        let eq125_e1598_d_n17: f64 = self.ddt_jacobian(eq125_e1597_d_n17);
        let eq125_e1598_d_n18: f64 = self.ddt_jacobian(eq125_e1597_d_n18);
        let eq125_e1598_d_n19: f64 = self.ddt_jacobian(eq125_e1597_d_n19);
        let eq125_e1598_d_n20: f64 = self.ddt_jacobian(eq125_e1597_d_n20);
        let eq125_e1598_d_n21: f64 = self.ddt_jacobian(eq125_e1597_d_n21);
        let eq125_e1598_d_n22: f64 = self.ddt_jacobian(eq125_e1597_d_n22);
        let eq125_e1599: f64 = (p.p7 * eq125_e1598);
        let eq125_e1599_d_n0: f64 = (p.p7 * eq125_e1598_d_n0);
        let eq125_e1599_d_n1: f64 = (p.p7 * eq125_e1598_d_n1);
        let eq125_e1599_d_n2: f64 = (p.p7 * eq125_e1598_d_n2);
        let eq125_e1599_d_n3: f64 = (p.p7 * eq125_e1598_d_n3);
        let eq125_e1599_d_n4: f64 = (p.p7 * eq125_e1598_d_n4);
        let eq125_e1599_d_n5: f64 = (p.p7 * eq125_e1598_d_n5);
        let eq125_e1599_d_n6: f64 = (p.p7 * eq125_e1598_d_n6);
        let eq125_e1599_d_n7: f64 = (p.p7 * eq125_e1598_d_n7);
        let eq125_e1599_d_n8: f64 = (p.p7 * eq125_e1598_d_n8);
        let eq125_e1599_d_n9: f64 = (p.p7 * eq125_e1598_d_n9);
        let eq125_e1599_d_n10: f64 = (p.p7 * eq125_e1598_d_n10);
        let eq125_e1599_d_n11: f64 = (p.p7 * eq125_e1598_d_n11);
        let eq125_e1599_d_n12: f64 = (p.p7 * eq125_e1598_d_n12);
        let eq125_e1599_d_n13: f64 = (p.p7 * eq125_e1598_d_n13);
        let eq125_e1599_d_n14: f64 = (p.p7 * eq125_e1598_d_n14);
        let eq125_e1599_d_n15: f64 = (p.p7 * eq125_e1598_d_n15);
        let eq125_e1599_d_n16: f64 = (p.p7 * eq125_e1598_d_n16);
        let eq125_e1599_d_n17: f64 = (p.p7 * eq125_e1598_d_n17);
        let eq125_e1599_d_n18: f64 = (p.p7 * eq125_e1598_d_n18);
        let eq125_e1599_d_n19: f64 = (p.p7 * eq125_e1598_d_n19);
        let eq125_e1599_d_n20: f64 = (p.p7 * eq125_e1598_d_n20);
        let eq125_e1599_d_n21: f64 = (p.p7 * eq125_e1598_d_n21);
        let eq125_e1599_d_n22: f64 = (p.p7 * eq125_e1598_d_n22);
        (eq125_e1599, eq125_e1599_d_n0, eq125_e1599_d_n1, eq125_e1599_d_n2, eq125_e1599_d_n3, eq125_e1599_d_n4, eq125_e1599_d_n5, eq125_e1599_d_n6, eq125_e1599_d_n7, eq125_e1599_d_n8, eq125_e1599_d_n9, eq125_e1599_d_n10, eq125_e1599_d_n11, eq125_e1599_d_n12, eq125_e1599_d_n13, eq125_e1599_d_n14, eq125_e1599_d_n15, eq125_e1599_d_n16, eq125_e1599_d_n17, eq125_e1599_d_n18, eq125_e1599_d_n19, eq125_e1599_d_n20, eq125_e1599_d_n21, eq125_e1599_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1601;
        let eq125_node_derivatives: [f64; 23] = [eq125_e1601_d_n0, eq125_e1601_d_n1, eq125_e1601_d_n2, eq125_e1601_d_n3, eq125_e1601_d_n4, eq125_e1601_d_n5, eq125_e1601_d_n6, eq125_e1601_d_n7, eq125_e1601_d_n8, eq125_e1601_d_n9, eq125_e1601_d_n10, eq125_e1601_d_n11, eq125_e1601_d_n12, eq125_e1601_d_n13, eq125_e1601_d_n14, eq125_e1601_d_n15, eq125_e1601_d_n16, eq125_e1601_d_n17, eq125_e1601_d_n18, eq125_e1601_d_n19, eq125_e1601_d_n20, eq125_e1601_d_n21, eq125_e1601_d_n22];
        let eq125_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
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
        let (eq126_e1611, eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22,) = {
    if ((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) {
        let eq126_e1608: f64 = self.eval_ddt(25, s.v[229]);
        let eq126_e1608_d_n0: f64 = self.ddt_jacobian(s.dn[229][0]);
        let eq126_e1608_d_n1: f64 = self.ddt_jacobian(s.dn[229][1]);
        let eq126_e1608_d_n2: f64 = self.ddt_jacobian(s.dn[229][2]);
        let eq126_e1608_d_n3: f64 = self.ddt_jacobian(s.dn[229][3]);
        let eq126_e1608_d_n4: f64 = self.ddt_jacobian(s.dn[229][4]);
        let eq126_e1608_d_n5: f64 = self.ddt_jacobian(s.dn[229][5]);
        let eq126_e1608_d_n6: f64 = self.ddt_jacobian(s.dn[229][6]);
        let eq126_e1608_d_n7: f64 = self.ddt_jacobian(s.dn[229][7]);
        let eq126_e1608_d_n8: f64 = self.ddt_jacobian(s.dn[229][8]);
        let eq126_e1608_d_n9: f64 = self.ddt_jacobian(s.dn[229][9]);
        let eq126_e1608_d_n10: f64 = self.ddt_jacobian(s.dn[229][10]);
        let eq126_e1608_d_n11: f64 = self.ddt_jacobian(s.dn[229][11]);
        let eq126_e1608_d_n12: f64 = self.ddt_jacobian(s.dn[229][12]);
        let eq126_e1608_d_n13: f64 = self.ddt_jacobian(s.dn[229][13]);
        let eq126_e1608_d_n14: f64 = self.ddt_jacobian(s.dn[229][14]);
        let eq126_e1608_d_n15: f64 = self.ddt_jacobian(s.dn[229][15]);
        let eq126_e1608_d_n16: f64 = self.ddt_jacobian(s.dn[229][16]);
        let eq126_e1608_d_n17: f64 = self.ddt_jacobian(s.dn[229][17]);
        let eq126_e1608_d_n18: f64 = self.ddt_jacobian(s.dn[229][18]);
        let eq126_e1608_d_n19: f64 = self.ddt_jacobian(s.dn[229][19]);
        let eq126_e1608_d_n20: f64 = self.ddt_jacobian(s.dn[229][20]);
        let eq126_e1608_d_n21: f64 = self.ddt_jacobian(s.dn[229][21]);
        let eq126_e1608_d_n22: f64 = self.ddt_jacobian(s.dn[229][22]);
        let eq126_e1609: f64 = (p.p7 * eq126_e1608);
        let eq126_e1609_d_n0: f64 = (p.p7 * eq126_e1608_d_n0);
        let eq126_e1609_d_n1: f64 = (p.p7 * eq126_e1608_d_n1);
        let eq126_e1609_d_n2: f64 = (p.p7 * eq126_e1608_d_n2);
        let eq126_e1609_d_n3: f64 = (p.p7 * eq126_e1608_d_n3);
        let eq126_e1609_d_n4: f64 = (p.p7 * eq126_e1608_d_n4);
        let eq126_e1609_d_n5: f64 = (p.p7 * eq126_e1608_d_n5);
        let eq126_e1609_d_n6: f64 = (p.p7 * eq126_e1608_d_n6);
        let eq126_e1609_d_n7: f64 = (p.p7 * eq126_e1608_d_n7);
        let eq126_e1609_d_n8: f64 = (p.p7 * eq126_e1608_d_n8);
        let eq126_e1609_d_n9: f64 = (p.p7 * eq126_e1608_d_n9);
        let eq126_e1609_d_n10: f64 = (p.p7 * eq126_e1608_d_n10);
        let eq126_e1609_d_n11: f64 = (p.p7 * eq126_e1608_d_n11);
        let eq126_e1609_d_n12: f64 = (p.p7 * eq126_e1608_d_n12);
        let eq126_e1609_d_n13: f64 = (p.p7 * eq126_e1608_d_n13);
        let eq126_e1609_d_n14: f64 = (p.p7 * eq126_e1608_d_n14);
        let eq126_e1609_d_n15: f64 = (p.p7 * eq126_e1608_d_n15);
        let eq126_e1609_d_n16: f64 = (p.p7 * eq126_e1608_d_n16);
        let eq126_e1609_d_n17: f64 = (p.p7 * eq126_e1608_d_n17);
        let eq126_e1609_d_n18: f64 = (p.p7 * eq126_e1608_d_n18);
        let eq126_e1609_d_n19: f64 = (p.p7 * eq126_e1608_d_n19);
        let eq126_e1609_d_n20: f64 = (p.p7 * eq126_e1608_d_n20);
        let eq126_e1609_d_n21: f64 = (p.p7 * eq126_e1608_d_n21);
        let eq126_e1609_d_n22: f64 = (p.p7 * eq126_e1608_d_n22);
        (eq126_e1609, eq126_e1609_d_n0, eq126_e1609_d_n1, eq126_e1609_d_n2, eq126_e1609_d_n3, eq126_e1609_d_n4, eq126_e1609_d_n5, eq126_e1609_d_n6, eq126_e1609_d_n7, eq126_e1609_d_n8, eq126_e1609_d_n9, eq126_e1609_d_n10, eq126_e1609_d_n11, eq126_e1609_d_n12, eq126_e1609_d_n13, eq126_e1609_d_n14, eq126_e1609_d_n15, eq126_e1609_d_n16, eq126_e1609_d_n17, eq126_e1609_d_n18, eq126_e1609_d_n19, eq126_e1609_d_n20, eq126_e1609_d_n21, eq126_e1609_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1611;
        let eq126_node_derivatives: [f64; 23] = [eq126_e1611_d_n0, eq126_e1611_d_n1, eq126_e1611_d_n2, eq126_e1611_d_n3, eq126_e1611_d_n4, eq126_e1611_d_n5, eq126_e1611_d_n6, eq126_e1611_d_n7, eq126_e1611_d_n8, eq126_e1611_d_n9, eq126_e1611_d_n10, eq126_e1611_d_n11, eq126_e1611_d_n12, eq126_e1611_d_n13, eq126_e1611_d_n14, eq126_e1611_d_n15, eq126_e1611_d_n16, eq126_e1611_d_n17, eq126_e1611_d_n18, eq126_e1611_d_n19, eq126_e1611_d_n20, eq126_e1611_d_n21, eq126_e1611_d_n22];
        let eq126_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[0]),
            Some(nodes[7]),
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
        let (eq127_e1623, eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (s.v[574] != 0.0)) {
        let eq127_e1620: f64 = self.eval_ddt(26, s.v[228]);
        let eq127_e1620_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq127_e1620_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq127_e1620_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq127_e1620_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq127_e1620_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq127_e1620_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq127_e1620_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq127_e1620_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq127_e1620_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq127_e1620_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq127_e1620_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq127_e1620_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq127_e1620_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq127_e1620_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq127_e1620_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq127_e1620_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq127_e1620_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq127_e1620_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq127_e1620_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq127_e1620_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq127_e1620_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq127_e1620_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq127_e1620_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq127_e1621: f64 = (p.p7 * eq127_e1620);
        let eq127_e1621_d_n0: f64 = (p.p7 * eq127_e1620_d_n0);
        let eq127_e1621_d_n1: f64 = (p.p7 * eq127_e1620_d_n1);
        let eq127_e1621_d_n2: f64 = (p.p7 * eq127_e1620_d_n2);
        let eq127_e1621_d_n3: f64 = (p.p7 * eq127_e1620_d_n3);
        let eq127_e1621_d_n4: f64 = (p.p7 * eq127_e1620_d_n4);
        let eq127_e1621_d_n5: f64 = (p.p7 * eq127_e1620_d_n5);
        let eq127_e1621_d_n6: f64 = (p.p7 * eq127_e1620_d_n6);
        let eq127_e1621_d_n7: f64 = (p.p7 * eq127_e1620_d_n7);
        let eq127_e1621_d_n8: f64 = (p.p7 * eq127_e1620_d_n8);
        let eq127_e1621_d_n9: f64 = (p.p7 * eq127_e1620_d_n9);
        let eq127_e1621_d_n10: f64 = (p.p7 * eq127_e1620_d_n10);
        let eq127_e1621_d_n11: f64 = (p.p7 * eq127_e1620_d_n11);
        let eq127_e1621_d_n12: f64 = (p.p7 * eq127_e1620_d_n12);
        let eq127_e1621_d_n13: f64 = (p.p7 * eq127_e1620_d_n13);
        let eq127_e1621_d_n14: f64 = (p.p7 * eq127_e1620_d_n14);
        let eq127_e1621_d_n15: f64 = (p.p7 * eq127_e1620_d_n15);
        let eq127_e1621_d_n16: f64 = (p.p7 * eq127_e1620_d_n16);
        let eq127_e1621_d_n17: f64 = (p.p7 * eq127_e1620_d_n17);
        let eq127_e1621_d_n18: f64 = (p.p7 * eq127_e1620_d_n18);
        let eq127_e1621_d_n19: f64 = (p.p7 * eq127_e1620_d_n19);
        let eq127_e1621_d_n20: f64 = (p.p7 * eq127_e1620_d_n20);
        let eq127_e1621_d_n21: f64 = (p.p7 * eq127_e1620_d_n21);
        let eq127_e1621_d_n22: f64 = (p.p7 * eq127_e1620_d_n22);
        (eq127_e1621, eq127_e1621_d_n0, eq127_e1621_d_n1, eq127_e1621_d_n2, eq127_e1621_d_n3, eq127_e1621_d_n4, eq127_e1621_d_n5, eq127_e1621_d_n6, eq127_e1621_d_n7, eq127_e1621_d_n8, eq127_e1621_d_n9, eq127_e1621_d_n10, eq127_e1621_d_n11, eq127_e1621_d_n12, eq127_e1621_d_n13, eq127_e1621_d_n14, eq127_e1621_d_n15, eq127_e1621_d_n16, eq127_e1621_d_n17, eq127_e1621_d_n18, eq127_e1621_d_n19, eq127_e1621_d_n20, eq127_e1621_d_n21, eq127_e1621_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq127_value: f64 = eq127_e1623;
        let eq127_node_derivatives: [f64; 23] = [eq127_e1623_d_n0, eq127_e1623_d_n1, eq127_e1623_d_n2, eq127_e1623_d_n3, eq127_e1623_d_n4, eq127_e1623_d_n5, eq127_e1623_d_n6, eq127_e1623_d_n7, eq127_e1623_d_n8, eq127_e1623_d_n9, eq127_e1623_d_n10, eq127_e1623_d_n11, eq127_e1623_d_n12, eq127_e1623_d_n13, eq127_e1623_d_n14, eq127_e1623_d_n15, eq127_e1623_d_n16, eq127_e1623_d_n17, eq127_e1623_d_n18, eq127_e1623_d_n19, eq127_e1623_d_n20, eq127_e1623_d_n21, eq127_e1623_d_n22];
        let eq127_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq127_value),
            &nodes,
            &eq127_node_derivatives,
            &branches,
            &eq127_branch_derivatives,
            self.multiplicity,
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
        let (eq128_e1637, eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (s.v[574] != 0.0)) {
        let eq128_e1632: f64 = self.eval_ddt(27, s.v[228]);
        let eq128_e1632_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq128_e1632_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq128_e1632_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq128_e1632_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq128_e1632_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq128_e1632_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq128_e1632_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq128_e1632_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq128_e1632_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq128_e1632_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq128_e1632_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq128_e1632_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq128_e1632_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq128_e1632_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq128_e1632_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq128_e1632_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq128_e1632_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq128_e1632_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq128_e1632_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq128_e1632_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq128_e1632_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq128_e1632_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq128_e1632_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq128_e1633: f64 = (p.p7 * eq128_e1632);
        let eq128_e1633_d_n0: f64 = (p.p7 * eq128_e1632_d_n0);
        let eq128_e1633_d_n1: f64 = (p.p7 * eq128_e1632_d_n1);
        let eq128_e1633_d_n2: f64 = (p.p7 * eq128_e1632_d_n2);
        let eq128_e1633_d_n3: f64 = (p.p7 * eq128_e1632_d_n3);
        let eq128_e1633_d_n4: f64 = (p.p7 * eq128_e1632_d_n4);
        let eq128_e1633_d_n5: f64 = (p.p7 * eq128_e1632_d_n5);
        let eq128_e1633_d_n6: f64 = (p.p7 * eq128_e1632_d_n6);
        let eq128_e1633_d_n7: f64 = (p.p7 * eq128_e1632_d_n7);
        let eq128_e1633_d_n8: f64 = (p.p7 * eq128_e1632_d_n8);
        let eq128_e1633_d_n9: f64 = (p.p7 * eq128_e1632_d_n9);
        let eq128_e1633_d_n10: f64 = (p.p7 * eq128_e1632_d_n10);
        let eq128_e1633_d_n11: f64 = (p.p7 * eq128_e1632_d_n11);
        let eq128_e1633_d_n12: f64 = (p.p7 * eq128_e1632_d_n12);
        let eq128_e1633_d_n13: f64 = (p.p7 * eq128_e1632_d_n13);
        let eq128_e1633_d_n14: f64 = (p.p7 * eq128_e1632_d_n14);
        let eq128_e1633_d_n15: f64 = (p.p7 * eq128_e1632_d_n15);
        let eq128_e1633_d_n16: f64 = (p.p7 * eq128_e1632_d_n16);
        let eq128_e1633_d_n17: f64 = (p.p7 * eq128_e1632_d_n17);
        let eq128_e1633_d_n18: f64 = (p.p7 * eq128_e1632_d_n18);
        let eq128_e1633_d_n19: f64 = (p.p7 * eq128_e1632_d_n19);
        let eq128_e1633_d_n20: f64 = (p.p7 * eq128_e1632_d_n20);
        let eq128_e1633_d_n21: f64 = (p.p7 * eq128_e1632_d_n21);
        let eq128_e1633_d_n22: f64 = (p.p7 * eq128_e1632_d_n22);
        let eq128_e1635: f64 = (eq128_e1633 * p.p246);
        let eq128_e1635_d_n0: f64 = (eq128_e1633_d_n0 * p.p246);
        let eq128_e1635_d_n1: f64 = (eq128_e1633_d_n1 * p.p246);
        let eq128_e1635_d_n2: f64 = (eq128_e1633_d_n2 * p.p246);
        let eq128_e1635_d_n3: f64 = (eq128_e1633_d_n3 * p.p246);
        let eq128_e1635_d_n4: f64 = (eq128_e1633_d_n4 * p.p246);
        let eq128_e1635_d_n5: f64 = (eq128_e1633_d_n5 * p.p246);
        let eq128_e1635_d_n6: f64 = (eq128_e1633_d_n6 * p.p246);
        let eq128_e1635_d_n7: f64 = (eq128_e1633_d_n7 * p.p246);
        let eq128_e1635_d_n8: f64 = (eq128_e1633_d_n8 * p.p246);
        let eq128_e1635_d_n9: f64 = (eq128_e1633_d_n9 * p.p246);
        let eq128_e1635_d_n10: f64 = (eq128_e1633_d_n10 * p.p246);
        let eq128_e1635_d_n11: f64 = (eq128_e1633_d_n11 * p.p246);
        let eq128_e1635_d_n12: f64 = (eq128_e1633_d_n12 * p.p246);
        let eq128_e1635_d_n13: f64 = (eq128_e1633_d_n13 * p.p246);
        let eq128_e1635_d_n14: f64 = (eq128_e1633_d_n14 * p.p246);
        let eq128_e1635_d_n15: f64 = (eq128_e1633_d_n15 * p.p246);
        let eq128_e1635_d_n16: f64 = (eq128_e1633_d_n16 * p.p246);
        let eq128_e1635_d_n17: f64 = (eq128_e1633_d_n17 * p.p246);
        let eq128_e1635_d_n18: f64 = (eq128_e1633_d_n18 * p.p246);
        let eq128_e1635_d_n19: f64 = (eq128_e1633_d_n19 * p.p246);
        let eq128_e1635_d_n20: f64 = (eq128_e1633_d_n20 * p.p246);
        let eq128_e1635_d_n21: f64 = (eq128_e1633_d_n21 * p.p246);
        let eq128_e1635_d_n22: f64 = (eq128_e1633_d_n22 * p.p246);
        (eq128_e1635, eq128_e1635_d_n0, eq128_e1635_d_n1, eq128_e1635_d_n2, eq128_e1635_d_n3, eq128_e1635_d_n4, eq128_e1635_d_n5, eq128_e1635_d_n6, eq128_e1635_d_n7, eq128_e1635_d_n8, eq128_e1635_d_n9, eq128_e1635_d_n10, eq128_e1635_d_n11, eq128_e1635_d_n12, eq128_e1635_d_n13, eq128_e1635_d_n14, eq128_e1635_d_n15, eq128_e1635_d_n16, eq128_e1635_d_n17, eq128_e1635_d_n18, eq128_e1635_d_n19, eq128_e1635_d_n20, eq128_e1635_d_n21, eq128_e1635_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1637;
        let eq128_node_derivatives: [f64; 23] = [eq128_e1637_d_n0, eq128_e1637_d_n1, eq128_e1637_d_n2, eq128_e1637_d_n3, eq128_e1637_d_n4, eq128_e1637_d_n5, eq128_e1637_d_n6, eq128_e1637_d_n7, eq128_e1637_d_n8, eq128_e1637_d_n9, eq128_e1637_d_n10, eq128_e1637_d_n11, eq128_e1637_d_n12, eq128_e1637_d_n13, eq128_e1637_d_n14, eq128_e1637_d_n15, eq128_e1637_d_n16, eq128_e1637_d_n17, eq128_e1637_d_n18, eq128_e1637_d_n19, eq128_e1637_d_n20, eq128_e1637_d_n21, eq128_e1637_d_n22];
        let eq128_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
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
        let (eq129_e1650, eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (!(s.v[574] != 0.0))) {
        let eq129_e1647: f64 = self.eval_ddt(28, s.v[228]);
        let eq129_e1647_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq129_e1647_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq129_e1647_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq129_e1647_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq129_e1647_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq129_e1647_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq129_e1647_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq129_e1647_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq129_e1647_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq129_e1647_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq129_e1647_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq129_e1647_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq129_e1647_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq129_e1647_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq129_e1647_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq129_e1647_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq129_e1647_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq129_e1647_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq129_e1647_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq129_e1647_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq129_e1647_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq129_e1647_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq129_e1647_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq129_e1648: f64 = (p.p7 * eq129_e1647);
        let eq129_e1648_d_n0: f64 = (p.p7 * eq129_e1647_d_n0);
        let eq129_e1648_d_n1: f64 = (p.p7 * eq129_e1647_d_n1);
        let eq129_e1648_d_n2: f64 = (p.p7 * eq129_e1647_d_n2);
        let eq129_e1648_d_n3: f64 = (p.p7 * eq129_e1647_d_n3);
        let eq129_e1648_d_n4: f64 = (p.p7 * eq129_e1647_d_n4);
        let eq129_e1648_d_n5: f64 = (p.p7 * eq129_e1647_d_n5);
        let eq129_e1648_d_n6: f64 = (p.p7 * eq129_e1647_d_n6);
        let eq129_e1648_d_n7: f64 = (p.p7 * eq129_e1647_d_n7);
        let eq129_e1648_d_n8: f64 = (p.p7 * eq129_e1647_d_n8);
        let eq129_e1648_d_n9: f64 = (p.p7 * eq129_e1647_d_n9);
        let eq129_e1648_d_n10: f64 = (p.p7 * eq129_e1647_d_n10);
        let eq129_e1648_d_n11: f64 = (p.p7 * eq129_e1647_d_n11);
        let eq129_e1648_d_n12: f64 = (p.p7 * eq129_e1647_d_n12);
        let eq129_e1648_d_n13: f64 = (p.p7 * eq129_e1647_d_n13);
        let eq129_e1648_d_n14: f64 = (p.p7 * eq129_e1647_d_n14);
        let eq129_e1648_d_n15: f64 = (p.p7 * eq129_e1647_d_n15);
        let eq129_e1648_d_n16: f64 = (p.p7 * eq129_e1647_d_n16);
        let eq129_e1648_d_n17: f64 = (p.p7 * eq129_e1647_d_n17);
        let eq129_e1648_d_n18: f64 = (p.p7 * eq129_e1647_d_n18);
        let eq129_e1648_d_n19: f64 = (p.p7 * eq129_e1647_d_n19);
        let eq129_e1648_d_n20: f64 = (p.p7 * eq129_e1647_d_n20);
        let eq129_e1648_d_n21: f64 = (p.p7 * eq129_e1647_d_n21);
        let eq129_e1648_d_n22: f64 = (p.p7 * eq129_e1647_d_n22);
        (eq129_e1648, eq129_e1648_d_n0, eq129_e1648_d_n1, eq129_e1648_d_n2, eq129_e1648_d_n3, eq129_e1648_d_n4, eq129_e1648_d_n5, eq129_e1648_d_n6, eq129_e1648_d_n7, eq129_e1648_d_n8, eq129_e1648_d_n9, eq129_e1648_d_n10, eq129_e1648_d_n11, eq129_e1648_d_n12, eq129_e1648_d_n13, eq129_e1648_d_n14, eq129_e1648_d_n15, eq129_e1648_d_n16, eq129_e1648_d_n17, eq129_e1648_d_n18, eq129_e1648_d_n19, eq129_e1648_d_n20, eq129_e1648_d_n21, eq129_e1648_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1650;
        let eq129_node_derivatives: [f64; 23] = [eq129_e1650_d_n0, eq129_e1650_d_n1, eq129_e1650_d_n2, eq129_e1650_d_n3, eq129_e1650_d_n4, eq129_e1650_d_n5, eq129_e1650_d_n6, eq129_e1650_d_n7, eq129_e1650_d_n8, eq129_e1650_d_n9, eq129_e1650_d_n10, eq129_e1650_d_n11, eq129_e1650_d_n12, eq129_e1650_d_n13, eq129_e1650_d_n14, eq129_e1650_d_n15, eq129_e1650_d_n16, eq129_e1650_d_n17, eq129_e1650_d_n18, eq129_e1650_d_n19, eq129_e1650_d_n20, eq129_e1650_d_n21, eq129_e1650_d_n22];
        let eq129_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[7]),
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
        let (eq130_e1665, eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22,) = {
    if (((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) && (!(s.v[574] != 0.0))) {
        let eq130_e1660: f64 = self.eval_ddt(29, s.v[228]);
        let eq130_e1660_d_n0: f64 = self.ddt_jacobian(s.dn[228][0]);
        let eq130_e1660_d_n1: f64 = self.ddt_jacobian(s.dn[228][1]);
        let eq130_e1660_d_n2: f64 = self.ddt_jacobian(s.dn[228][2]);
        let eq130_e1660_d_n3: f64 = self.ddt_jacobian(s.dn[228][3]);
        let eq130_e1660_d_n4: f64 = self.ddt_jacobian(s.dn[228][4]);
        let eq130_e1660_d_n5: f64 = self.ddt_jacobian(s.dn[228][5]);
        let eq130_e1660_d_n6: f64 = self.ddt_jacobian(s.dn[228][6]);
        let eq130_e1660_d_n7: f64 = self.ddt_jacobian(s.dn[228][7]);
        let eq130_e1660_d_n8: f64 = self.ddt_jacobian(s.dn[228][8]);
        let eq130_e1660_d_n9: f64 = self.ddt_jacobian(s.dn[228][9]);
        let eq130_e1660_d_n10: f64 = self.ddt_jacobian(s.dn[228][10]);
        let eq130_e1660_d_n11: f64 = self.ddt_jacobian(s.dn[228][11]);
        let eq130_e1660_d_n12: f64 = self.ddt_jacobian(s.dn[228][12]);
        let eq130_e1660_d_n13: f64 = self.ddt_jacobian(s.dn[228][13]);
        let eq130_e1660_d_n14: f64 = self.ddt_jacobian(s.dn[228][14]);
        let eq130_e1660_d_n15: f64 = self.ddt_jacobian(s.dn[228][15]);
        let eq130_e1660_d_n16: f64 = self.ddt_jacobian(s.dn[228][16]);
        let eq130_e1660_d_n17: f64 = self.ddt_jacobian(s.dn[228][17]);
        let eq130_e1660_d_n18: f64 = self.ddt_jacobian(s.dn[228][18]);
        let eq130_e1660_d_n19: f64 = self.ddt_jacobian(s.dn[228][19]);
        let eq130_e1660_d_n20: f64 = self.ddt_jacobian(s.dn[228][20]);
        let eq130_e1660_d_n21: f64 = self.ddt_jacobian(s.dn[228][21]);
        let eq130_e1660_d_n22: f64 = self.ddt_jacobian(s.dn[228][22]);
        let eq130_e1661: f64 = (p.p7 * eq130_e1660);
        let eq130_e1661_d_n0: f64 = (p.p7 * eq130_e1660_d_n0);
        let eq130_e1661_d_n1: f64 = (p.p7 * eq130_e1660_d_n1);
        let eq130_e1661_d_n2: f64 = (p.p7 * eq130_e1660_d_n2);
        let eq130_e1661_d_n3: f64 = (p.p7 * eq130_e1660_d_n3);
        let eq130_e1661_d_n4: f64 = (p.p7 * eq130_e1660_d_n4);
        let eq130_e1661_d_n5: f64 = (p.p7 * eq130_e1660_d_n5);
        let eq130_e1661_d_n6: f64 = (p.p7 * eq130_e1660_d_n6);
        let eq130_e1661_d_n7: f64 = (p.p7 * eq130_e1660_d_n7);
        let eq130_e1661_d_n8: f64 = (p.p7 * eq130_e1660_d_n8);
        let eq130_e1661_d_n9: f64 = (p.p7 * eq130_e1660_d_n9);
        let eq130_e1661_d_n10: f64 = (p.p7 * eq130_e1660_d_n10);
        let eq130_e1661_d_n11: f64 = (p.p7 * eq130_e1660_d_n11);
        let eq130_e1661_d_n12: f64 = (p.p7 * eq130_e1660_d_n12);
        let eq130_e1661_d_n13: f64 = (p.p7 * eq130_e1660_d_n13);
        let eq130_e1661_d_n14: f64 = (p.p7 * eq130_e1660_d_n14);
        let eq130_e1661_d_n15: f64 = (p.p7 * eq130_e1660_d_n15);
        let eq130_e1661_d_n16: f64 = (p.p7 * eq130_e1660_d_n16);
        let eq130_e1661_d_n17: f64 = (p.p7 * eq130_e1660_d_n17);
        let eq130_e1661_d_n18: f64 = (p.p7 * eq130_e1660_d_n18);
        let eq130_e1661_d_n19: f64 = (p.p7 * eq130_e1660_d_n19);
        let eq130_e1661_d_n20: f64 = (p.p7 * eq130_e1660_d_n20);
        let eq130_e1661_d_n21: f64 = (p.p7 * eq130_e1660_d_n21);
        let eq130_e1661_d_n22: f64 = (p.p7 * eq130_e1660_d_n22);
        let eq130_e1663: f64 = (eq130_e1661 * p.p246);
        let eq130_e1663_d_n0: f64 = (eq130_e1661_d_n0 * p.p246);
        let eq130_e1663_d_n1: f64 = (eq130_e1661_d_n1 * p.p246);
        let eq130_e1663_d_n2: f64 = (eq130_e1661_d_n2 * p.p246);
        let eq130_e1663_d_n3: f64 = (eq130_e1661_d_n3 * p.p246);
        let eq130_e1663_d_n4: f64 = (eq130_e1661_d_n4 * p.p246);
        let eq130_e1663_d_n5: f64 = (eq130_e1661_d_n5 * p.p246);
        let eq130_e1663_d_n6: f64 = (eq130_e1661_d_n6 * p.p246);
        let eq130_e1663_d_n7: f64 = (eq130_e1661_d_n7 * p.p246);
        let eq130_e1663_d_n8: f64 = (eq130_e1661_d_n8 * p.p246);
        let eq130_e1663_d_n9: f64 = (eq130_e1661_d_n9 * p.p246);
        let eq130_e1663_d_n10: f64 = (eq130_e1661_d_n10 * p.p246);
        let eq130_e1663_d_n11: f64 = (eq130_e1661_d_n11 * p.p246);
        let eq130_e1663_d_n12: f64 = (eq130_e1661_d_n12 * p.p246);
        let eq130_e1663_d_n13: f64 = (eq130_e1661_d_n13 * p.p246);
        let eq130_e1663_d_n14: f64 = (eq130_e1661_d_n14 * p.p246);
        let eq130_e1663_d_n15: f64 = (eq130_e1661_d_n15 * p.p246);
        let eq130_e1663_d_n16: f64 = (eq130_e1661_d_n16 * p.p246);
        let eq130_e1663_d_n17: f64 = (eq130_e1661_d_n17 * p.p246);
        let eq130_e1663_d_n18: f64 = (eq130_e1661_d_n18 * p.p246);
        let eq130_e1663_d_n19: f64 = (eq130_e1661_d_n19 * p.p246);
        let eq130_e1663_d_n20: f64 = (eq130_e1661_d_n20 * p.p246);
        let eq130_e1663_d_n21: f64 = (eq130_e1661_d_n21 * p.p246);
        let eq130_e1663_d_n22: f64 = (eq130_e1661_d_n22 * p.p246);
        (eq130_e1663, eq130_e1663_d_n0, eq130_e1663_d_n1, eq130_e1663_d_n2, eq130_e1663_d_n3, eq130_e1663_d_n4, eq130_e1663_d_n5, eq130_e1663_d_n6, eq130_e1663_d_n7, eq130_e1663_d_n8, eq130_e1663_d_n9, eq130_e1663_d_n10, eq130_e1663_d_n11, eq130_e1663_d_n12, eq130_e1663_d_n13, eq130_e1663_d_n14, eq130_e1663_d_n15, eq130_e1663_d_n16, eq130_e1663_d_n17, eq130_e1663_d_n18, eq130_e1663_d_n19, eq130_e1663_d_n20, eq130_e1663_d_n21, eq130_e1663_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1665;
        let eq130_node_derivatives: [f64; 23] = [eq130_e1665_d_n0, eq130_e1665_d_n1, eq130_e1665_d_n2, eq130_e1665_d_n3, eq130_e1665_d_n4, eq130_e1665_d_n5, eq130_e1665_d_n6, eq130_e1665_d_n7, eq130_e1665_d_n8, eq130_e1665_d_n9, eq130_e1665_d_n10, eq130_e1665_d_n11, eq130_e1665_d_n12, eq130_e1665_d_n13, eq130_e1665_d_n14, eq130_e1665_d_n15, eq130_e1665_d_n16, eq130_e1665_d_n17, eq130_e1665_d_n18, eq130_e1665_d_n19, eq130_e1665_d_n20, eq130_e1665_d_n21, eq130_e1665_d_n22];
        let eq130_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let (eq131_e1677, eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22,) = {
    if ((!(s.v[570] != 0.0)) && (s.v[573] != 0.0)) {
        let eq131_e1673: f64 = (p.p251 * s.v[228]);
        let eq131_e1673_d_n0: f64 = (p.p251 * s.dn[228][0]);
        let eq131_e1673_d_n1: f64 = (p.p251 * s.dn[228][1]);
        let eq131_e1673_d_n2: f64 = (p.p251 * s.dn[228][2]);
        let eq131_e1673_d_n3: f64 = (p.p251 * s.dn[228][3]);
        let eq131_e1673_d_n4: f64 = (p.p251 * s.dn[228][4]);
        let eq131_e1673_d_n5: f64 = (p.p251 * s.dn[228][5]);
        let eq131_e1673_d_n6: f64 = (p.p251 * s.dn[228][6]);
        let eq131_e1673_d_n7: f64 = (p.p251 * s.dn[228][7]);
        let eq131_e1673_d_n8: f64 = (p.p251 * s.dn[228][8]);
        let eq131_e1673_d_n9: f64 = (p.p251 * s.dn[228][9]);
        let eq131_e1673_d_n10: f64 = (p.p251 * s.dn[228][10]);
        let eq131_e1673_d_n11: f64 = (p.p251 * s.dn[228][11]);
        let eq131_e1673_d_n12: f64 = (p.p251 * s.dn[228][12]);
        let eq131_e1673_d_n13: f64 = (p.p251 * s.dn[228][13]);
        let eq131_e1673_d_n14: f64 = (p.p251 * s.dn[228][14]);
        let eq131_e1673_d_n15: f64 = (p.p251 * s.dn[228][15]);
        let eq131_e1673_d_n16: f64 = (p.p251 * s.dn[228][16]);
        let eq131_e1673_d_n17: f64 = (p.p251 * s.dn[228][17]);
        let eq131_e1673_d_n18: f64 = (p.p251 * s.dn[228][18]);
        let eq131_e1673_d_n19: f64 = (p.p251 * s.dn[228][19]);
        let eq131_e1673_d_n20: f64 = (p.p251 * s.dn[228][20]);
        let eq131_e1673_d_n21: f64 = (p.p251 * s.dn[228][21]);
        let eq131_e1673_d_n22: f64 = (p.p251 * s.dn[228][22]);
        let eq131_e1674: f64 = self.eval_ddt(30, eq131_e1673);
        let eq131_e1674_d_n0: f64 = self.ddt_jacobian(eq131_e1673_d_n0);
        let eq131_e1674_d_n1: f64 = self.ddt_jacobian(eq131_e1673_d_n1);
        let eq131_e1674_d_n2: f64 = self.ddt_jacobian(eq131_e1673_d_n2);
        let eq131_e1674_d_n3: f64 = self.ddt_jacobian(eq131_e1673_d_n3);
        let eq131_e1674_d_n4: f64 = self.ddt_jacobian(eq131_e1673_d_n4);
        let eq131_e1674_d_n5: f64 = self.ddt_jacobian(eq131_e1673_d_n5);
        let eq131_e1674_d_n6: f64 = self.ddt_jacobian(eq131_e1673_d_n6);
        let eq131_e1674_d_n7: f64 = self.ddt_jacobian(eq131_e1673_d_n7);
        let eq131_e1674_d_n8: f64 = self.ddt_jacobian(eq131_e1673_d_n8);
        let eq131_e1674_d_n9: f64 = self.ddt_jacobian(eq131_e1673_d_n9);
        let eq131_e1674_d_n10: f64 = self.ddt_jacobian(eq131_e1673_d_n10);
        let eq131_e1674_d_n11: f64 = self.ddt_jacobian(eq131_e1673_d_n11);
        let eq131_e1674_d_n12: f64 = self.ddt_jacobian(eq131_e1673_d_n12);
        let eq131_e1674_d_n13: f64 = self.ddt_jacobian(eq131_e1673_d_n13);
        let eq131_e1674_d_n14: f64 = self.ddt_jacobian(eq131_e1673_d_n14);
        let eq131_e1674_d_n15: f64 = self.ddt_jacobian(eq131_e1673_d_n15);
        let eq131_e1674_d_n16: f64 = self.ddt_jacobian(eq131_e1673_d_n16);
        let eq131_e1674_d_n17: f64 = self.ddt_jacobian(eq131_e1673_d_n17);
        let eq131_e1674_d_n18: f64 = self.ddt_jacobian(eq131_e1673_d_n18);
        let eq131_e1674_d_n19: f64 = self.ddt_jacobian(eq131_e1673_d_n19);
        let eq131_e1674_d_n20: f64 = self.ddt_jacobian(eq131_e1673_d_n20);
        let eq131_e1674_d_n21: f64 = self.ddt_jacobian(eq131_e1673_d_n21);
        let eq131_e1674_d_n22: f64 = self.ddt_jacobian(eq131_e1673_d_n22);
        let eq131_e1675: f64 = (p.p7 * eq131_e1674);
        let eq131_e1675_d_n0: f64 = (p.p7 * eq131_e1674_d_n0);
        let eq131_e1675_d_n1: f64 = (p.p7 * eq131_e1674_d_n1);
        let eq131_e1675_d_n2: f64 = (p.p7 * eq131_e1674_d_n2);
        let eq131_e1675_d_n3: f64 = (p.p7 * eq131_e1674_d_n3);
        let eq131_e1675_d_n4: f64 = (p.p7 * eq131_e1674_d_n4);
        let eq131_e1675_d_n5: f64 = (p.p7 * eq131_e1674_d_n5);
        let eq131_e1675_d_n6: f64 = (p.p7 * eq131_e1674_d_n6);
        let eq131_e1675_d_n7: f64 = (p.p7 * eq131_e1674_d_n7);
        let eq131_e1675_d_n8: f64 = (p.p7 * eq131_e1674_d_n8);
        let eq131_e1675_d_n9: f64 = (p.p7 * eq131_e1674_d_n9);
        let eq131_e1675_d_n10: f64 = (p.p7 * eq131_e1674_d_n10);
        let eq131_e1675_d_n11: f64 = (p.p7 * eq131_e1674_d_n11);
        let eq131_e1675_d_n12: f64 = (p.p7 * eq131_e1674_d_n12);
        let eq131_e1675_d_n13: f64 = (p.p7 * eq131_e1674_d_n13);
        let eq131_e1675_d_n14: f64 = (p.p7 * eq131_e1674_d_n14);
        let eq131_e1675_d_n15: f64 = (p.p7 * eq131_e1674_d_n15);
        let eq131_e1675_d_n16: f64 = (p.p7 * eq131_e1674_d_n16);
        let eq131_e1675_d_n17: f64 = (p.p7 * eq131_e1674_d_n17);
        let eq131_e1675_d_n18: f64 = (p.p7 * eq131_e1674_d_n18);
        let eq131_e1675_d_n19: f64 = (p.p7 * eq131_e1674_d_n19);
        let eq131_e1675_d_n20: f64 = (p.p7 * eq131_e1674_d_n20);
        let eq131_e1675_d_n21: f64 = (p.p7 * eq131_e1674_d_n21);
        let eq131_e1675_d_n22: f64 = (p.p7 * eq131_e1674_d_n22);
        (eq131_e1675, eq131_e1675_d_n0, eq131_e1675_d_n1, eq131_e1675_d_n2, eq131_e1675_d_n3, eq131_e1675_d_n4, eq131_e1675_d_n5, eq131_e1675_d_n6, eq131_e1675_d_n7, eq131_e1675_d_n8, eq131_e1675_d_n9, eq131_e1675_d_n10, eq131_e1675_d_n11, eq131_e1675_d_n12, eq131_e1675_d_n13, eq131_e1675_d_n14, eq131_e1675_d_n15, eq131_e1675_d_n16, eq131_e1675_d_n17, eq131_e1675_d_n18, eq131_e1675_d_n19, eq131_e1675_d_n20, eq131_e1675_d_n21, eq131_e1675_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1677;
        let eq131_node_derivatives: [f64; 23] = [eq131_e1677_d_n0, eq131_e1677_d_n1, eq131_e1677_d_n2, eq131_e1677_d_n3, eq131_e1677_d_n4, eq131_e1677_d_n5, eq131_e1677_d_n6, eq131_e1677_d_n7, eq131_e1677_d_n8, eq131_e1677_d_n9, eq131_e1677_d_n10, eq131_e1677_d_n11, eq131_e1677_d_n12, eq131_e1677_d_n13, eq131_e1677_d_n14, eq131_e1677_d_n15, eq131_e1677_d_n16, eq131_e1677_d_n17, eq131_e1677_d_n18, eq131_e1677_d_n19, eq131_e1677_d_n20, eq131_e1677_d_n21, eq131_e1677_d_n22];
        let eq131_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            self.multiplicity * (eq131_value),
            &nodes,
            &eq131_node_derivatives,
            &branches,
            &eq131_branch_derivatives,
            self.multiplicity,
        );
    }

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
        let (eq132_e1686, eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22,) = {
    if ((s.v[575] != 0.0) && (s.v[576] != 0.0)) {
        let eq132_e1683: f64 = self.eval_ddt(31, s.v[241]);
        let eq132_e1683_d_n0: f64 = self.ddt_jacobian(s.dn[241][0]);
        let eq132_e1683_d_n1: f64 = self.ddt_jacobian(s.dn[241][1]);
        let eq132_e1683_d_n2: f64 = self.ddt_jacobian(s.dn[241][2]);
        let eq132_e1683_d_n3: f64 = self.ddt_jacobian(s.dn[241][3]);
        let eq132_e1683_d_n4: f64 = self.ddt_jacobian(s.dn[241][4]);
        let eq132_e1683_d_n5: f64 = self.ddt_jacobian(s.dn[241][5]);
        let eq132_e1683_d_n6: f64 = self.ddt_jacobian(s.dn[241][6]);
        let eq132_e1683_d_n7: f64 = self.ddt_jacobian(s.dn[241][7]);
        let eq132_e1683_d_n8: f64 = self.ddt_jacobian(s.dn[241][8]);
        let eq132_e1683_d_n9: f64 = self.ddt_jacobian(s.dn[241][9]);
        let eq132_e1683_d_n10: f64 = self.ddt_jacobian(s.dn[241][10]);
        let eq132_e1683_d_n11: f64 = self.ddt_jacobian(s.dn[241][11]);
        let eq132_e1683_d_n12: f64 = self.ddt_jacobian(s.dn[241][12]);
        let eq132_e1683_d_n13: f64 = self.ddt_jacobian(s.dn[241][13]);
        let eq132_e1683_d_n14: f64 = self.ddt_jacobian(s.dn[241][14]);
        let eq132_e1683_d_n15: f64 = self.ddt_jacobian(s.dn[241][15]);
        let eq132_e1683_d_n16: f64 = self.ddt_jacobian(s.dn[241][16]);
        let eq132_e1683_d_n17: f64 = self.ddt_jacobian(s.dn[241][17]);
        let eq132_e1683_d_n18: f64 = self.ddt_jacobian(s.dn[241][18]);
        let eq132_e1683_d_n19: f64 = self.ddt_jacobian(s.dn[241][19]);
        let eq132_e1683_d_n20: f64 = self.ddt_jacobian(s.dn[241][20]);
        let eq132_e1683_d_n21: f64 = self.ddt_jacobian(s.dn[241][21]);
        let eq132_e1683_d_n22: f64 = self.ddt_jacobian(s.dn[241][22]);
        let eq132_e1684: f64 = (p.p7 * eq132_e1683);
        let eq132_e1684_d_n0: f64 = (p.p7 * eq132_e1683_d_n0);
        let eq132_e1684_d_n1: f64 = (p.p7 * eq132_e1683_d_n1);
        let eq132_e1684_d_n2: f64 = (p.p7 * eq132_e1683_d_n2);
        let eq132_e1684_d_n3: f64 = (p.p7 * eq132_e1683_d_n3);
        let eq132_e1684_d_n4: f64 = (p.p7 * eq132_e1683_d_n4);
        let eq132_e1684_d_n5: f64 = (p.p7 * eq132_e1683_d_n5);
        let eq132_e1684_d_n6: f64 = (p.p7 * eq132_e1683_d_n6);
        let eq132_e1684_d_n7: f64 = (p.p7 * eq132_e1683_d_n7);
        let eq132_e1684_d_n8: f64 = (p.p7 * eq132_e1683_d_n8);
        let eq132_e1684_d_n9: f64 = (p.p7 * eq132_e1683_d_n9);
        let eq132_e1684_d_n10: f64 = (p.p7 * eq132_e1683_d_n10);
        let eq132_e1684_d_n11: f64 = (p.p7 * eq132_e1683_d_n11);
        let eq132_e1684_d_n12: f64 = (p.p7 * eq132_e1683_d_n12);
        let eq132_e1684_d_n13: f64 = (p.p7 * eq132_e1683_d_n13);
        let eq132_e1684_d_n14: f64 = (p.p7 * eq132_e1683_d_n14);
        let eq132_e1684_d_n15: f64 = (p.p7 * eq132_e1683_d_n15);
        let eq132_e1684_d_n16: f64 = (p.p7 * eq132_e1683_d_n16);
        let eq132_e1684_d_n17: f64 = (p.p7 * eq132_e1683_d_n17);
        let eq132_e1684_d_n18: f64 = (p.p7 * eq132_e1683_d_n18);
        let eq132_e1684_d_n19: f64 = (p.p7 * eq132_e1683_d_n19);
        let eq132_e1684_d_n20: f64 = (p.p7 * eq132_e1683_d_n20);
        let eq132_e1684_d_n21: f64 = (p.p7 * eq132_e1683_d_n21);
        let eq132_e1684_d_n22: f64 = (p.p7 * eq132_e1683_d_n22);
        (eq132_e1684, eq132_e1684_d_n0, eq132_e1684_d_n1, eq132_e1684_d_n2, eq132_e1684_d_n3, eq132_e1684_d_n4, eq132_e1684_d_n5, eq132_e1684_d_n6, eq132_e1684_d_n7, eq132_e1684_d_n8, eq132_e1684_d_n9, eq132_e1684_d_n10, eq132_e1684_d_n11, eq132_e1684_d_n12, eq132_e1684_d_n13, eq132_e1684_d_n14, eq132_e1684_d_n15, eq132_e1684_d_n16, eq132_e1684_d_n17, eq132_e1684_d_n18, eq132_e1684_d_n19, eq132_e1684_d_n20, eq132_e1684_d_n21, eq132_e1684_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq132_value: f64 = eq132_e1686;
        let eq132_node_derivatives: [f64; 23] = [eq132_e1686_d_n0, eq132_e1686_d_n1, eq132_e1686_d_n2, eq132_e1686_d_n3, eq132_e1686_d_n4, eq132_e1686_d_n5, eq132_e1686_d_n6, eq132_e1686_d_n7, eq132_e1686_d_n8, eq132_e1686_d_n9, eq132_e1686_d_n10, eq132_e1686_d_n11, eq132_e1686_d_n12, eq132_e1686_d_n13, eq132_e1686_d_n14, eq132_e1686_d_n15, eq132_e1686_d_n16, eq132_e1686_d_n17, eq132_e1686_d_n18, eq132_e1686_d_n19, eq132_e1686_d_n20, eq132_e1686_d_n21, eq132_e1686_d_n22];
        let eq132_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[19]),
            self.multiplicity * (eq132_value),
            &nodes,
            &eq132_node_derivatives,
            &branches,
            &eq132_branch_derivatives,
            self.multiplicity,
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
        let (eq133_e1697, eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
        let eq133_e1694: f64 = self.eval_ddt(32, s.v[240]);
        let eq133_e1694_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq133_e1694_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq133_e1694_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq133_e1694_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq133_e1694_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq133_e1694_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq133_e1694_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq133_e1694_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq133_e1694_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq133_e1694_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq133_e1694_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq133_e1694_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq133_e1694_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq133_e1694_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq133_e1694_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq133_e1694_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq133_e1694_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq133_e1694_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq133_e1694_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq133_e1694_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq133_e1694_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq133_e1694_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq133_e1694_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq133_e1695: f64 = (p.p7 * eq133_e1694);
        let eq133_e1695_d_n0: f64 = (p.p7 * eq133_e1694_d_n0);
        let eq133_e1695_d_n1: f64 = (p.p7 * eq133_e1694_d_n1);
        let eq133_e1695_d_n2: f64 = (p.p7 * eq133_e1694_d_n2);
        let eq133_e1695_d_n3: f64 = (p.p7 * eq133_e1694_d_n3);
        let eq133_e1695_d_n4: f64 = (p.p7 * eq133_e1694_d_n4);
        let eq133_e1695_d_n5: f64 = (p.p7 * eq133_e1694_d_n5);
        let eq133_e1695_d_n6: f64 = (p.p7 * eq133_e1694_d_n6);
        let eq133_e1695_d_n7: f64 = (p.p7 * eq133_e1694_d_n7);
        let eq133_e1695_d_n8: f64 = (p.p7 * eq133_e1694_d_n8);
        let eq133_e1695_d_n9: f64 = (p.p7 * eq133_e1694_d_n9);
        let eq133_e1695_d_n10: f64 = (p.p7 * eq133_e1694_d_n10);
        let eq133_e1695_d_n11: f64 = (p.p7 * eq133_e1694_d_n11);
        let eq133_e1695_d_n12: f64 = (p.p7 * eq133_e1694_d_n12);
        let eq133_e1695_d_n13: f64 = (p.p7 * eq133_e1694_d_n13);
        let eq133_e1695_d_n14: f64 = (p.p7 * eq133_e1694_d_n14);
        let eq133_e1695_d_n15: f64 = (p.p7 * eq133_e1694_d_n15);
        let eq133_e1695_d_n16: f64 = (p.p7 * eq133_e1694_d_n16);
        let eq133_e1695_d_n17: f64 = (p.p7 * eq133_e1694_d_n17);
        let eq133_e1695_d_n18: f64 = (p.p7 * eq133_e1694_d_n18);
        let eq133_e1695_d_n19: f64 = (p.p7 * eq133_e1694_d_n19);
        let eq133_e1695_d_n20: f64 = (p.p7 * eq133_e1694_d_n20);
        let eq133_e1695_d_n21: f64 = (p.p7 * eq133_e1694_d_n21);
        let eq133_e1695_d_n22: f64 = (p.p7 * eq133_e1694_d_n22);
        (eq133_e1695, eq133_e1695_d_n0, eq133_e1695_d_n1, eq133_e1695_d_n2, eq133_e1695_d_n3, eq133_e1695_d_n4, eq133_e1695_d_n5, eq133_e1695_d_n6, eq133_e1695_d_n7, eq133_e1695_d_n8, eq133_e1695_d_n9, eq133_e1695_d_n10, eq133_e1695_d_n11, eq133_e1695_d_n12, eq133_e1695_d_n13, eq133_e1695_d_n14, eq133_e1695_d_n15, eq133_e1695_d_n16, eq133_e1695_d_n17, eq133_e1695_d_n18, eq133_e1695_d_n19, eq133_e1695_d_n20, eq133_e1695_d_n21, eq133_e1695_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq133_value: f64 = eq133_e1697;
        let eq133_node_derivatives: [f64; 23] = [eq133_e1697_d_n0, eq133_e1697_d_n1, eq133_e1697_d_n2, eq133_e1697_d_n3, eq133_e1697_d_n4, eq133_e1697_d_n5, eq133_e1697_d_n6, eq133_e1697_d_n7, eq133_e1697_d_n8, eq133_e1697_d_n9, eq133_e1697_d_n10, eq133_e1697_d_n11, eq133_e1697_d_n12, eq133_e1697_d_n13, eq133_e1697_d_n14, eq133_e1697_d_n15, eq133_e1697_d_n16, eq133_e1697_d_n17, eq133_e1697_d_n18, eq133_e1697_d_n19, eq133_e1697_d_n20, eq133_e1697_d_n21, eq133_e1697_d_n22];
        let eq133_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[19]),
            self.multiplicity * (eq133_value),
            &nodes,
            &eq133_node_derivatives,
            &branches,
            &eq133_branch_derivatives,
            self.multiplicity,
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
        let (eq134_e1710, eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22,) = {
    if (((s.v[575] != 0.0) && (s.v[576] != 0.0)) && (s.v[577] != 0.0)) {
        let eq134_e1705: f64 = self.eval_ddt(33, s.v[240]);
        let eq134_e1705_d_n0: f64 = self.ddt_jacobian(s.dn[240][0]);
        let eq134_e1705_d_n1: f64 = self.ddt_jacobian(s.dn[240][1]);
        let eq134_e1705_d_n2: f64 = self.ddt_jacobian(s.dn[240][2]);
        let eq134_e1705_d_n3: f64 = self.ddt_jacobian(s.dn[240][3]);
        let eq134_e1705_d_n4: f64 = self.ddt_jacobian(s.dn[240][4]);
        let eq134_e1705_d_n5: f64 = self.ddt_jacobian(s.dn[240][5]);
        let eq134_e1705_d_n6: f64 = self.ddt_jacobian(s.dn[240][6]);
        let eq134_e1705_d_n7: f64 = self.ddt_jacobian(s.dn[240][7]);
        let eq134_e1705_d_n8: f64 = self.ddt_jacobian(s.dn[240][8]);
        let eq134_e1705_d_n9: f64 = self.ddt_jacobian(s.dn[240][9]);
        let eq134_e1705_d_n10: f64 = self.ddt_jacobian(s.dn[240][10]);
        let eq134_e1705_d_n11: f64 = self.ddt_jacobian(s.dn[240][11]);
        let eq134_e1705_d_n12: f64 = self.ddt_jacobian(s.dn[240][12]);
        let eq134_e1705_d_n13: f64 = self.ddt_jacobian(s.dn[240][13]);
        let eq134_e1705_d_n14: f64 = self.ddt_jacobian(s.dn[240][14]);
        let eq134_e1705_d_n15: f64 = self.ddt_jacobian(s.dn[240][15]);
        let eq134_e1705_d_n16: f64 = self.ddt_jacobian(s.dn[240][16]);
        let eq134_e1705_d_n17: f64 = self.ddt_jacobian(s.dn[240][17]);
        let eq134_e1705_d_n18: f64 = self.ddt_jacobian(s.dn[240][18]);
        let eq134_e1705_d_n19: f64 = self.ddt_jacobian(s.dn[240][19]);
        let eq134_e1705_d_n20: f64 = self.ddt_jacobian(s.dn[240][20]);
        let eq134_e1705_d_n21: f64 = self.ddt_jacobian(s.dn[240][21]);
        let eq134_e1705_d_n22: f64 = self.ddt_jacobian(s.dn[240][22]);
        let eq134_e1706: f64 = (p.p7 * eq134_e1705);
        let eq134_e1706_d_n0: f64 = (p.p7 * eq134_e1705_d_n0);
        let eq134_e1706_d_n1: f64 = (p.p7 * eq134_e1705_d_n1);
        let eq134_e1706_d_n2: f64 = (p.p7 * eq134_e1705_d_n2);
        let eq134_e1706_d_n3: f64 = (p.p7 * eq134_e1705_d_n3);
        let eq134_e1706_d_n4: f64 = (p.p7 * eq134_e1705_d_n4);
        let eq134_e1706_d_n5: f64 = (p.p7 * eq134_e1705_d_n5);
        let eq134_e1706_d_n6: f64 = (p.p7 * eq134_e1705_d_n6);
        let eq134_e1706_d_n7: f64 = (p.p7 * eq134_e1705_d_n7);
        let eq134_e1706_d_n8: f64 = (p.p7 * eq134_e1705_d_n8);
        let eq134_e1706_d_n9: f64 = (p.p7 * eq134_e1705_d_n9);
        let eq134_e1706_d_n10: f64 = (p.p7 * eq134_e1705_d_n10);
        let eq134_e1706_d_n11: f64 = (p.p7 * eq134_e1705_d_n11);
        let eq134_e1706_d_n12: f64 = (p.p7 * eq134_e1705_d_n12);
        let eq134_e1706_d_n13: f64 = (p.p7 * eq134_e1705_d_n13);
        let eq134_e1706_d_n14: f64 = (p.p7 * eq134_e1705_d_n14);
        let eq134_e1706_d_n15: f64 = (p.p7 * eq134_e1705_d_n15);
        let eq134_e1706_d_n16: f64 = (p.p7 * eq134_e1705_d_n16);
        let eq134_e1706_d_n17: f64 = (p.p7 * eq134_e1705_d_n17);
        let eq134_e1706_d_n18: f64 = (p.p7 * eq134_e1705_d_n18);
        let eq134_e1706_d_n19: f64 = (p.p7 * eq134_e1705_d_n19);
        let eq134_e1706_d_n20: f64 = (p.p7 * eq134_e1705_d_n20);
        let eq134_e1706_d_n21: f64 = (p.p7 * eq134_e1705_d_n21);
        let eq134_e1706_d_n22: f64 = (p.p7 * eq134_e1705_d_n22);
        let eq134_e1708: f64 = (eq134_e1706 * p.p246);
        let eq134_e1708_d_n0: f64 = (eq134_e1706_d_n0 * p.p246);
        let eq134_e1708_d_n1: f64 = (eq134_e1706_d_n1 * p.p246);
        let eq134_e1708_d_n2: f64 = (eq134_e1706_d_n2 * p.p246);
        let eq134_e1708_d_n3: f64 = (eq134_e1706_d_n3 * p.p246);
        let eq134_e1708_d_n4: f64 = (eq134_e1706_d_n4 * p.p246);
        let eq134_e1708_d_n5: f64 = (eq134_e1706_d_n5 * p.p246);
        let eq134_e1708_d_n6: f64 = (eq134_e1706_d_n6 * p.p246);
        let eq134_e1708_d_n7: f64 = (eq134_e1706_d_n7 * p.p246);
        let eq134_e1708_d_n8: f64 = (eq134_e1706_d_n8 * p.p246);
        let eq134_e1708_d_n9: f64 = (eq134_e1706_d_n9 * p.p246);
        let eq134_e1708_d_n10: f64 = (eq134_e1706_d_n10 * p.p246);
        let eq134_e1708_d_n11: f64 = (eq134_e1706_d_n11 * p.p246);
        let eq134_e1708_d_n12: f64 = (eq134_e1706_d_n12 * p.p246);
        let eq134_e1708_d_n13: f64 = (eq134_e1706_d_n13 * p.p246);
        let eq134_e1708_d_n14: f64 = (eq134_e1706_d_n14 * p.p246);
        let eq134_e1708_d_n15: f64 = (eq134_e1706_d_n15 * p.p246);
        let eq134_e1708_d_n16: f64 = (eq134_e1706_d_n16 * p.p246);
        let eq134_e1708_d_n17: f64 = (eq134_e1706_d_n17 * p.p246);
        let eq134_e1708_d_n18: f64 = (eq134_e1706_d_n18 * p.p246);
        let eq134_e1708_d_n19: f64 = (eq134_e1706_d_n19 * p.p246);
        let eq134_e1708_d_n20: f64 = (eq134_e1706_d_n20 * p.p246);
        let eq134_e1708_d_n21: f64 = (eq134_e1706_d_n21 * p.p246);
        let eq134_e1708_d_n22: f64 = (eq134_e1706_d_n22 * p.p246);
        (eq134_e1708, eq134_e1708_d_n0, eq134_e1708_d_n1, eq134_e1708_d_n2, eq134_e1708_d_n3, eq134_e1708_d_n4, eq134_e1708_d_n5, eq134_e1708_d_n6, eq134_e1708_d_n7, eq134_e1708_d_n8, eq134_e1708_d_n9, eq134_e1708_d_n10, eq134_e1708_d_n11, eq134_e1708_d_n12, eq134_e1708_d_n13, eq134_e1708_d_n14, eq134_e1708_d_n15, eq134_e1708_d_n16, eq134_e1708_d_n17, eq134_e1708_d_n18, eq134_e1708_d_n19, eq134_e1708_d_n20, eq134_e1708_d_n21, eq134_e1708_d_n22,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq134_value: f64 = eq134_e1710;
        let eq134_node_derivatives: [f64; 23] = [eq134_e1710_d_n0, eq134_e1710_d_n1, eq134_e1710_d_n2, eq134_e1710_d_n3, eq134_e1710_d_n4, eq134_e1710_d_n5, eq134_e1710_d_n6, eq134_e1710_d_n7, eq134_e1710_d_n8, eq134_e1710_d_n9, eq134_e1710_d_n10, eq134_e1710_d_n11, eq134_e1710_d_n12, eq134_e1710_d_n13, eq134_e1710_d_n14, eq134_e1710_d_n15, eq134_e1710_d_n16, eq134_e1710_d_n17, eq134_e1710_d_n18, eq134_e1710_d_n19, eq134_e1710_d_n20, eq134_e1710_d_n21, eq134_e1710_d_n22];
        let eq134_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[2]),
            Some(nodes[19]),
            self.multiplicity * (eq134_value),
            &nodes,
            &eq134_node_derivatives,
            &branches,
            &eq134_branch_derivatives,
            self.multiplicity,
        );
    }
}

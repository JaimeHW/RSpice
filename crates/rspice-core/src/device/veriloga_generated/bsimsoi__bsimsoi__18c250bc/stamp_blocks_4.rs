#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq26_e1499, eq26_e1499_d_n0, eq26_e1499_d_n1, eq26_e1499_d_n2, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12, eq26_e1499_d_n13,) = {
    if (!(s.v[1548] != 0.0)) {
        let eq26_e1485: f64 = (p.p37 * p.p32);
        let eq26_e1488: f64 = (s.v[885] - s.v[933]);
        let eq26_e1488_d_n0: f64 = (s.dn[885][0] - s.dn[933][0]);
        let eq26_e1488_d_n1: f64 = (s.dn[885][1] - s.dn[933][1]);
        let eq26_e1488_d_n2: f64 = (s.dn[885][2] - s.dn[933][2]);
        let eq26_e1488_d_n3: f64 = (s.dn[885][3] - s.dn[933][3]);
        let eq26_e1488_d_n4: f64 = (s.dn[885][4] - s.dn[933][4]);
        let eq26_e1488_d_n5: f64 = (s.dn[885][5] - s.dn[933][5]);
        let eq26_e1488_d_n6: f64 = (s.dn[885][6] - s.dn[933][6]);
        let eq26_e1488_d_n7: f64 = (s.dn[885][7] - s.dn[933][7]);
        let eq26_e1488_d_n8: f64 = (s.dn[885][8] - s.dn[933][8]);
        let eq26_e1488_d_n9: f64 = (s.dn[885][9] - s.dn[933][9]);
        let eq26_e1488_d_n10: f64 = (s.dn[885][10] - s.dn[933][10]);
        let eq26_e1488_d_n11: f64 = (s.dn[885][11] - s.dn[933][11]);
        let eq26_e1488_d_n12: f64 = (s.dn[885][12] - s.dn[933][12]);
        let eq26_e1488_d_n13: f64 = (s.dn[885][13] - s.dn[933][13]);
        let eq26_e1489: f64 = (eq26_e1485 * eq26_e1488);
        let eq26_e1489_d_n0: f64 = (eq26_e1485 * eq26_e1488_d_n0);
        let eq26_e1489_d_n1: f64 = (eq26_e1485 * eq26_e1488_d_n1);
        let eq26_e1489_d_n2: f64 = (eq26_e1485 * eq26_e1488_d_n2);
        let eq26_e1489_d_n3: f64 = (eq26_e1485 * eq26_e1488_d_n3);
        let eq26_e1489_d_n4: f64 = (eq26_e1485 * eq26_e1488_d_n4);
        let eq26_e1489_d_n5: f64 = (eq26_e1485 * eq26_e1488_d_n5);
        let eq26_e1489_d_n6: f64 = (eq26_e1485 * eq26_e1488_d_n6);
        let eq26_e1489_d_n7: f64 = (eq26_e1485 * eq26_e1488_d_n7);
        let eq26_e1489_d_n8: f64 = (eq26_e1485 * eq26_e1488_d_n8);
        let eq26_e1489_d_n9: f64 = (eq26_e1485 * eq26_e1488_d_n9);
        let eq26_e1489_d_n10: f64 = (eq26_e1485 * eq26_e1488_d_n10);
        let eq26_e1489_d_n11: f64 = (eq26_e1485 * eq26_e1488_d_n11);
        let eq26_e1489_d_n12: f64 = (eq26_e1485 * eq26_e1488_d_n12);
        let eq26_e1489_d_n13: f64 = (eq26_e1485 * eq26_e1488_d_n13);
        let eq26_e1493: f64 = 0.0;
        let eq26_e1495: f64 = (eq26_e1493 * (nv8 - nv7));
        let eq26_e1495_d_n7: f64 = (-eq26_e1493);
        let eq26_e1496: f64 = (p.p32 * eq26_e1495);
        let eq26_e1496_d_n7: f64 = (p.p32 * eq26_e1495_d_n7);
        let eq26_e1496_d_n8: f64 = (p.p32 * eq26_e1493);
        let eq26_e1497: f64 = (eq26_e1489 + eq26_e1496);
        let eq26_e1497_d_n7: f64 = (eq26_e1489_d_n7 + eq26_e1496_d_n7);
        let eq26_e1497_d_n8: f64 = (eq26_e1489_d_n8 + eq26_e1496_d_n8);
        (eq26_e1497, eq26_e1489_d_n0, eq26_e1489_d_n1, eq26_e1489_d_n2, eq26_e1489_d_n3, eq26_e1489_d_n4, eq26_e1489_d_n5, eq26_e1489_d_n6, eq26_e1497_d_n7, eq26_e1497_d_n8, eq26_e1489_d_n9, eq26_e1489_d_n10, eq26_e1489_d_n11, eq26_e1489_d_n12, eq26_e1489_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1499;
        let eq26_node_derivatives: [f64; 14] = [eq26_e1499_d_n0, eq26_e1499_d_n1, eq26_e1499_d_n2, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12, eq26_e1499_d_n13];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
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
        let (eq27_e1508, eq27_e1508_d_n0, eq27_e1508_d_n1, eq27_e1508_d_n2, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12, eq27_e1508_d_n13,) = {
    if (!(s.v[1548] != 0.0)) {
        let eq27_e1504: f64 = (p.p37 * p.p32);
        let eq27_e1506: f64 = (eq27_e1504 * s.v[908]);
        let eq27_e1506_d_n0: f64 = (eq27_e1504 * s.dn[908][0]);
        let eq27_e1506_d_n1: f64 = (eq27_e1504 * s.dn[908][1]);
        let eq27_e1506_d_n2: f64 = (eq27_e1504 * s.dn[908][2]);
        let eq27_e1506_d_n3: f64 = (eq27_e1504 * s.dn[908][3]);
        let eq27_e1506_d_n4: f64 = (eq27_e1504 * s.dn[908][4]);
        let eq27_e1506_d_n5: f64 = (eq27_e1504 * s.dn[908][5]);
        let eq27_e1506_d_n6: f64 = (eq27_e1504 * s.dn[908][6]);
        let eq27_e1506_d_n7: f64 = (eq27_e1504 * s.dn[908][7]);
        let eq27_e1506_d_n8: f64 = (eq27_e1504 * s.dn[908][8]);
        let eq27_e1506_d_n9: f64 = (eq27_e1504 * s.dn[908][9]);
        let eq27_e1506_d_n10: f64 = (eq27_e1504 * s.dn[908][10]);
        let eq27_e1506_d_n11: f64 = (eq27_e1504 * s.dn[908][11]);
        let eq27_e1506_d_n12: f64 = (eq27_e1504 * s.dn[908][12]);
        let eq27_e1506_d_n13: f64 = (eq27_e1504 * s.dn[908][13]);
        (eq27_e1506, eq27_e1506_d_n0, eq27_e1506_d_n1, eq27_e1506_d_n2, eq27_e1506_d_n3, eq27_e1506_d_n4, eq27_e1506_d_n5, eq27_e1506_d_n6, eq27_e1506_d_n7, eq27_e1506_d_n8, eq27_e1506_d_n9, eq27_e1506_d_n10, eq27_e1506_d_n11, eq27_e1506_d_n12, eq27_e1506_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1508;
        let eq27_node_derivatives: [f64; 14] = [eq27_e1508_d_n0, eq27_e1508_d_n1, eq27_e1508_d_n2, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12, eq27_e1508_d_n13];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq27_value),
            &nodes,
            &eq27_node_derivatives,
            &branches,
            &eq27_branch_derivatives,
            self.multiplicity,
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
        let eq28_e1511: f64 = (p.p32 * s.v[88]);
        let eq28_e1511_d_n0: f64 = (p.p32 * s.dn[88][0]);
        let eq28_e1511_d_n1: f64 = (p.p32 * s.dn[88][1]);
        let eq28_e1511_d_n2: f64 = (p.p32 * s.dn[88][2]);
        let eq28_e1511_d_n3: f64 = (p.p32 * s.dn[88][3]);
        let eq28_e1511_d_n4: f64 = (p.p32 * s.dn[88][4]);
        let eq28_e1511_d_n5: f64 = (p.p32 * s.dn[88][5]);
        let eq28_e1511_d_n6: f64 = (p.p32 * s.dn[88][6]);
        let eq28_e1511_d_n7: f64 = (p.p32 * s.dn[88][7]);
        let eq28_e1511_d_n8: f64 = (p.p32 * s.dn[88][8]);
        let eq28_e1511_d_n9: f64 = (p.p32 * s.dn[88][9]);
        let eq28_e1511_d_n10: f64 = (p.p32 * s.dn[88][10]);
        let eq28_e1511_d_n11: f64 = (p.p32 * s.dn[88][11]);
        let eq28_e1511_d_n12: f64 = (p.p32 * s.dn[88][12]);
        let eq28_e1511_d_n13: f64 = (p.p32 * s.dn[88][13]);
        let eq28_value: f64 = eq28_e1511;
        let eq28_node_derivatives: [f64; 14] = [eq28_e1511_d_n0, eq28_e1511_d_n1, eq28_e1511_d_n2, eq28_e1511_d_n3, eq28_e1511_d_n4, eq28_e1511_d_n5, eq28_e1511_d_n6, eq28_e1511_d_n7, eq28_e1511_d_n8, eq28_e1511_d_n9, eq28_e1511_d_n10, eq28_e1511_d_n11, eq28_e1511_d_n12, eq28_e1511_d_n13];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[7]),
            Some(nodes[5]),
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
        let eq29_e1514: f64 = (p.p32 * s.v[89]);
        let eq29_e1514_d_n0: f64 = (p.p32 * s.dn[89][0]);
        let eq29_e1514_d_n1: f64 = (p.p32 * s.dn[89][1]);
        let eq29_e1514_d_n2: f64 = (p.p32 * s.dn[89][2]);
        let eq29_e1514_d_n3: f64 = (p.p32 * s.dn[89][3]);
        let eq29_e1514_d_n4: f64 = (p.p32 * s.dn[89][4]);
        let eq29_e1514_d_n5: f64 = (p.p32 * s.dn[89][5]);
        let eq29_e1514_d_n6: f64 = (p.p32 * s.dn[89][6]);
        let eq29_e1514_d_n7: f64 = (p.p32 * s.dn[89][7]);
        let eq29_e1514_d_n8: f64 = (p.p32 * s.dn[89][8]);
        let eq29_e1514_d_n9: f64 = (p.p32 * s.dn[89][9]);
        let eq29_e1514_d_n10: f64 = (p.p32 * s.dn[89][10]);
        let eq29_e1514_d_n11: f64 = (p.p32 * s.dn[89][11]);
        let eq29_e1514_d_n12: f64 = (p.p32 * s.dn[89][12]);
        let eq29_e1514_d_n13: f64 = (p.p32 * s.dn[89][13]);
        let eq29_value: f64 = eq29_e1514;
        let eq29_node_derivatives: [f64; 14] = [eq29_e1514_d_n0, eq29_e1514_d_n1, eq29_e1514_d_n2, eq29_e1514_d_n3, eq29_e1514_d_n4, eq29_e1514_d_n5, eq29_e1514_d_n6, eq29_e1514_d_n7, eq29_e1514_d_n8, eq29_e1514_d_n9, eq29_e1514_d_n10, eq29_e1514_d_n11, eq29_e1514_d_n12, eq29_e1514_d_n13];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            self.multiplicity * (eq29_value),
            &nodes,
            &eq29_node_derivatives,
            &branches,
            &eq29_branch_derivatives,
            self.multiplicity,
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
        let eq30_e1517: f64 = (p.p37 * p.p32);
        let eq30_e1519: f64 = (eq30_e1517 * s.v[935]);
        let eq30_e1519_d_n0: f64 = (eq30_e1517 * s.dn[935][0]);
        let eq30_e1519_d_n1: f64 = (eq30_e1517 * s.dn[935][1]);
        let eq30_e1519_d_n2: f64 = (eq30_e1517 * s.dn[935][2]);
        let eq30_e1519_d_n3: f64 = (eq30_e1517 * s.dn[935][3]);
        let eq30_e1519_d_n4: f64 = (eq30_e1517 * s.dn[935][4]);
        let eq30_e1519_d_n5: f64 = (eq30_e1517 * s.dn[935][5]);
        let eq30_e1519_d_n6: f64 = (eq30_e1517 * s.dn[935][6]);
        let eq30_e1519_d_n7: f64 = (eq30_e1517 * s.dn[935][7]);
        let eq30_e1519_d_n8: f64 = (eq30_e1517 * s.dn[935][8]);
        let eq30_e1519_d_n9: f64 = (eq30_e1517 * s.dn[935][9]);
        let eq30_e1519_d_n10: f64 = (eq30_e1517 * s.dn[935][10]);
        let eq30_e1519_d_n11: f64 = (eq30_e1517 * s.dn[935][11]);
        let eq30_e1519_d_n12: f64 = (eq30_e1517 * s.dn[935][12]);
        let eq30_e1519_d_n13: f64 = (eq30_e1517 * s.dn[935][13]);
        let eq30_value: f64 = eq30_e1519;
        let eq30_node_derivatives: [f64; 14] = [eq30_e1519_d_n0, eq30_e1519_d_n1, eq30_e1519_d_n2, eq30_e1519_d_n3, eq30_e1519_d_n4, eq30_e1519_d_n5, eq30_e1519_d_n6, eq30_e1519_d_n7, eq30_e1519_d_n8, eq30_e1519_d_n9, eq30_e1519_d_n10, eq30_e1519_d_n11, eq30_e1519_d_n12, eq30_e1519_d_n13];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[12]),
            Some(nodes[7]),
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
        let eq31_e1522: f64 = (p.p37 * p.p32);
        let eq31_e1524: f64 = (eq31_e1522 * s.v[934]);
        let eq31_e1524_d_n0: f64 = (eq31_e1522 * s.dn[934][0]);
        let eq31_e1524_d_n1: f64 = (eq31_e1522 * s.dn[934][1]);
        let eq31_e1524_d_n2: f64 = (eq31_e1522 * s.dn[934][2]);
        let eq31_e1524_d_n3: f64 = (eq31_e1522 * s.dn[934][3]);
        let eq31_e1524_d_n4: f64 = (eq31_e1522 * s.dn[934][4]);
        let eq31_e1524_d_n5: f64 = (eq31_e1522 * s.dn[934][5]);
        let eq31_e1524_d_n6: f64 = (eq31_e1522 * s.dn[934][6]);
        let eq31_e1524_d_n7: f64 = (eq31_e1522 * s.dn[934][7]);
        let eq31_e1524_d_n8: f64 = (eq31_e1522 * s.dn[934][8]);
        let eq31_e1524_d_n9: f64 = (eq31_e1522 * s.dn[934][9]);
        let eq31_e1524_d_n10: f64 = (eq31_e1522 * s.dn[934][10]);
        let eq31_e1524_d_n11: f64 = (eq31_e1522 * s.dn[934][11]);
        let eq31_e1524_d_n12: f64 = (eq31_e1522 * s.dn[934][12]);
        let eq31_e1524_d_n13: f64 = (eq31_e1522 * s.dn[934][13]);
        let eq31_value: f64 = eq31_e1524;
        let eq31_node_derivatives: [f64; 14] = [eq31_e1524_d_n0, eq31_e1524_d_n1, eq31_e1524_d_n2, eq31_e1524_d_n3, eq31_e1524_d_n4, eq31_e1524_d_n5, eq31_e1524_d_n6, eq31_e1524_d_n7, eq31_e1524_d_n8, eq31_e1524_d_n9, eq31_e1524_d_n10, eq31_e1524_d_n11, eq31_e1524_d_n12, eq31_e1524_d_n13];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[11]),
            Some(nodes[8]),
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
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq32_e1528: f64 = (s.v[94] + s.v[90]);
        let eq32_e1528_d_n0: f64 = (s.dn[94][0] + s.dn[90][0]);
        let eq32_e1528_d_n1: f64 = (s.dn[94][1] + s.dn[90][1]);
        let eq32_e1528_d_n2: f64 = (s.dn[94][2] + s.dn[90][2]);
        let eq32_e1528_d_n3: f64 = (s.dn[94][3] + s.dn[90][3]);
        let eq32_e1528_d_n4: f64 = (s.dn[94][4] + s.dn[90][4]);
        let eq32_e1528_d_n5: f64 = (s.dn[94][5] + s.dn[90][5]);
        let eq32_e1528_d_n6: f64 = (s.dn[94][6] + s.dn[90][6]);
        let eq32_e1528_d_n7: f64 = (s.dn[94][7] + s.dn[90][7]);
        let eq32_e1528_d_n8: f64 = (s.dn[94][8] + s.dn[90][8]);
        let eq32_e1528_d_n9: f64 = (s.dn[94][9] + s.dn[90][9]);
        let eq32_e1528_d_n10: f64 = (s.dn[94][10] + s.dn[90][10]);
        let eq32_e1528_d_n11: f64 = (s.dn[94][11] + s.dn[90][11]);
        let eq32_e1528_d_n12: f64 = (s.dn[94][12] + s.dn[90][12]);
        let eq32_e1528_d_n13: f64 = (s.dn[94][13] + s.dn[90][13]);
        let eq32_e1529: f64 = (p.p32 * eq32_e1528);
        let eq32_e1529_d_n0: f64 = (p.p32 * eq32_e1528_d_n0);
        let eq32_e1529_d_n1: f64 = (p.p32 * eq32_e1528_d_n1);
        let eq32_e1529_d_n2: f64 = (p.p32 * eq32_e1528_d_n2);
        let eq32_e1529_d_n3: f64 = (p.p32 * eq32_e1528_d_n3);
        let eq32_e1529_d_n4: f64 = (p.p32 * eq32_e1528_d_n4);
        let eq32_e1529_d_n5: f64 = (p.p32 * eq32_e1528_d_n5);
        let eq32_e1529_d_n6: f64 = (p.p32 * eq32_e1528_d_n6);
        let eq32_e1529_d_n7: f64 = (p.p32 * eq32_e1528_d_n7);
        let eq32_e1529_d_n8: f64 = (p.p32 * eq32_e1528_d_n8);
        let eq32_e1529_d_n9: f64 = (p.p32 * eq32_e1528_d_n9);
        let eq32_e1529_d_n10: f64 = (p.p32 * eq32_e1528_d_n10);
        let eq32_e1529_d_n11: f64 = (p.p32 * eq32_e1528_d_n11);
        let eq32_e1529_d_n12: f64 = (p.p32 * eq32_e1528_d_n12);
        let eq32_e1529_d_n13: f64 = (p.p32 * eq32_e1528_d_n13);
        let eq32_e1533: f64 = 0.0;
        let eq32_e1535: f64 = (eq32_e1533 * (nv9 - nv7));
        let eq32_e1535_d_n7: f64 = (-eq32_e1533);
        let eq32_e1536: f64 = (p.p32 * eq32_e1535);
        let eq32_e1536_d_n7: f64 = (p.p32 * eq32_e1535_d_n7);
        let eq32_e1536_d_n9: f64 = (p.p32 * eq32_e1533);
        let eq32_e1537: f64 = (eq32_e1529 + eq32_e1536);
        let eq32_e1537_d_n7: f64 = (eq32_e1529_d_n7 + eq32_e1536_d_n7);
        let eq32_e1537_d_n9: f64 = (eq32_e1529_d_n9 + eq32_e1536_d_n9);
        let eq32_value: f64 = eq32_e1537;
        let eq32_node_derivatives: [f64; 14] = [eq32_e1529_d_n0, eq32_e1529_d_n1, eq32_e1529_d_n2, eq32_e1529_d_n3, eq32_e1529_d_n4, eq32_e1529_d_n5, eq32_e1529_d_n6, eq32_e1537_d_n7, eq32_e1529_d_n8, eq32_e1537_d_n9, eq32_e1529_d_n10, eq32_e1529_d_n11, eq32_e1529_d_n12, eq32_e1529_d_n13];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[7]),
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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq33_e1541: f64 = (s.v[95] + s.v[91]);
        let eq33_e1541_d_n0: f64 = (s.dn[95][0] + s.dn[91][0]);
        let eq33_e1541_d_n1: f64 = (s.dn[95][1] + s.dn[91][1]);
        let eq33_e1541_d_n2: f64 = (s.dn[95][2] + s.dn[91][2]);
        let eq33_e1541_d_n3: f64 = (s.dn[95][3] + s.dn[91][3]);
        let eq33_e1541_d_n4: f64 = (s.dn[95][4] + s.dn[91][4]);
        let eq33_e1541_d_n5: f64 = (s.dn[95][5] + s.dn[91][5]);
        let eq33_e1541_d_n6: f64 = (s.dn[95][6] + s.dn[91][6]);
        let eq33_e1541_d_n7: f64 = (s.dn[95][7] + s.dn[91][7]);
        let eq33_e1541_d_n8: f64 = (s.dn[95][8] + s.dn[91][8]);
        let eq33_e1541_d_n9: f64 = (s.dn[95][9] + s.dn[91][9]);
        let eq33_e1541_d_n10: f64 = (s.dn[95][10] + s.dn[91][10]);
        let eq33_e1541_d_n11: f64 = (s.dn[95][11] + s.dn[91][11]);
        let eq33_e1541_d_n12: f64 = (s.dn[95][12] + s.dn[91][12]);
        let eq33_e1541_d_n13: f64 = (s.dn[95][13] + s.dn[91][13]);
        let eq33_e1542: f64 = (p.p32 * eq33_e1541);
        let eq33_e1542_d_n0: f64 = (p.p32 * eq33_e1541_d_n0);
        let eq33_e1542_d_n1: f64 = (p.p32 * eq33_e1541_d_n1);
        let eq33_e1542_d_n2: f64 = (p.p32 * eq33_e1541_d_n2);
        let eq33_e1542_d_n3: f64 = (p.p32 * eq33_e1541_d_n3);
        let eq33_e1542_d_n4: f64 = (p.p32 * eq33_e1541_d_n4);
        let eq33_e1542_d_n5: f64 = (p.p32 * eq33_e1541_d_n5);
        let eq33_e1542_d_n6: f64 = (p.p32 * eq33_e1541_d_n6);
        let eq33_e1542_d_n7: f64 = (p.p32 * eq33_e1541_d_n7);
        let eq33_e1542_d_n8: f64 = (p.p32 * eq33_e1541_d_n8);
        let eq33_e1542_d_n9: f64 = (p.p32 * eq33_e1541_d_n9);
        let eq33_e1542_d_n10: f64 = (p.p32 * eq33_e1541_d_n10);
        let eq33_e1542_d_n11: f64 = (p.p32 * eq33_e1541_d_n11);
        let eq33_e1542_d_n12: f64 = (p.p32 * eq33_e1541_d_n12);
        let eq33_e1542_d_n13: f64 = (p.p32 * eq33_e1541_d_n13);
        let eq33_e1546: f64 = 0.0;
        let eq33_e1548: f64 = (eq33_e1546 * (nv9 - nv8));
        let eq33_e1548_d_n8: f64 = (-eq33_e1546);
        let eq33_e1549: f64 = (p.p32 * eq33_e1548);
        let eq33_e1549_d_n8: f64 = (p.p32 * eq33_e1548_d_n8);
        let eq33_e1549_d_n9: f64 = (p.p32 * eq33_e1546);
        let eq33_e1550: f64 = (eq33_e1542 + eq33_e1549);
        let eq33_e1550_d_n8: f64 = (eq33_e1542_d_n8 + eq33_e1549_d_n8);
        let eq33_e1550_d_n9: f64 = (eq33_e1542_d_n9 + eq33_e1549_d_n9);
        let eq33_value: f64 = eq33_e1550;
        let eq33_node_derivatives: [f64; 14] = [eq33_e1542_d_n0, eq33_e1542_d_n1, eq33_e1542_d_n2, eq33_e1542_d_n3, eq33_e1542_d_n4, eq33_e1542_d_n5, eq33_e1542_d_n6, eq33_e1542_d_n7, eq33_e1550_d_n8, eq33_e1550_d_n9, eq33_e1542_d_n10, eq33_e1542_d_n11, eq33_e1542_d_n12, eq33_e1542_d_n13];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[8]),
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
        let eq34_e1553: f64 = (p.p32 * s.v[79]);
        let eq34_e1553_d_n0: f64 = (p.p32 * s.dn[79][0]);
        let eq34_e1553_d_n1: f64 = (p.p32 * s.dn[79][1]);
        let eq34_e1553_d_n2: f64 = (p.p32 * s.dn[79][2]);
        let eq34_e1553_d_n3: f64 = (p.p32 * s.dn[79][3]);
        let eq34_e1553_d_n4: f64 = (p.p32 * s.dn[79][4]);
        let eq34_e1553_d_n5: f64 = (p.p32 * s.dn[79][5]);
        let eq34_e1553_d_n6: f64 = (p.p32 * s.dn[79][6]);
        let eq34_e1553_d_n7: f64 = (p.p32 * s.dn[79][7]);
        let eq34_e1553_d_n8: f64 = (p.p32 * s.dn[79][8]);
        let eq34_e1553_d_n9: f64 = (p.p32 * s.dn[79][9]);
        let eq34_e1553_d_n10: f64 = (p.p32 * s.dn[79][10]);
        let eq34_e1553_d_n11: f64 = (p.p32 * s.dn[79][11]);
        let eq34_e1553_d_n12: f64 = (p.p32 * s.dn[79][12]);
        let eq34_e1553_d_n13: f64 = (p.p32 * s.dn[79][13]);
        let eq34_value: f64 = eq34_e1553;
        let eq34_node_derivatives: [f64; 14] = [eq34_e1553_d_n0, eq34_e1553_d_n1, eq34_e1553_d_n2, eq34_e1553_d_n3, eq34_e1553_d_n4, eq34_e1553_d_n5, eq34_e1553_d_n6, eq34_e1553_d_n7, eq34_e1553_d_n8, eq34_e1553_d_n9, eq34_e1553_d_n10, eq34_e1553_d_n11, eq34_e1553_d_n12, eq34_e1553_d_n13];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[5]),
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
        let eq35_e1556: f64 = (p.p32 * s.v[80]);
        let eq35_e1556_d_n0: f64 = (p.p32 * s.dn[80][0]);
        let eq35_e1556_d_n1: f64 = (p.p32 * s.dn[80][1]);
        let eq35_e1556_d_n2: f64 = (p.p32 * s.dn[80][2]);
        let eq35_e1556_d_n3: f64 = (p.p32 * s.dn[80][3]);
        let eq35_e1556_d_n4: f64 = (p.p32 * s.dn[80][4]);
        let eq35_e1556_d_n5: f64 = (p.p32 * s.dn[80][5]);
        let eq35_e1556_d_n6: f64 = (p.p32 * s.dn[80][6]);
        let eq35_e1556_d_n7: f64 = (p.p32 * s.dn[80][7]);
        let eq35_e1556_d_n8: f64 = (p.p32 * s.dn[80][8]);
        let eq35_e1556_d_n9: f64 = (p.p32 * s.dn[80][9]);
        let eq35_e1556_d_n10: f64 = (p.p32 * s.dn[80][10]);
        let eq35_e1556_d_n11: f64 = (p.p32 * s.dn[80][11]);
        let eq35_e1556_d_n12: f64 = (p.p32 * s.dn[80][12]);
        let eq35_e1556_d_n13: f64 = (p.p32 * s.dn[80][13]);
        let eq35_value: f64 = eq35_e1556;
        let eq35_node_derivatives: [f64; 14] = [eq35_e1556_d_n0, eq35_e1556_d_n1, eq35_e1556_d_n2, eq35_e1556_d_n3, eq35_e1556_d_n4, eq35_e1556_d_n5, eq35_e1556_d_n6, eq35_e1556_d_n7, eq35_e1556_d_n8, eq35_e1556_d_n9, eq35_e1556_d_n10, eq35_e1556_d_n11, eq35_e1556_d_n12, eq35_e1556_d_n13];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[9]),
            Some(nodes[4]),
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
        let (eq36_e1560,) = {
    if (s.v[1552] != 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e1560;
        stamper.stamp_potential(
            branches[9],
            eq36_value,
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
        let (eq37_e1569, eq37_e1569_d_n0, eq37_e1569_d_n1, eq37_e1569_d_n2, eq37_e1569_d_n3, eq37_e1569_d_n4, eq37_e1569_d_n5, eq37_e1569_d_n6, eq37_e1569_d_n7, eq37_e1569_d_n8, eq37_e1569_d_n9, eq37_e1569_d_n10, eq37_e1569_d_n11, eq37_e1569_d_n12, eq37_e1569_d_n13,) = {
    if (!(s.v[1552] != 0.0)) {
        let eq37_e1565: f64 = (p.p37 * p.p32);
        let eq37_e1567: f64 = (eq37_e1565 * s.v[907]);
        let eq37_e1567_d_n0: f64 = (eq37_e1565 * s.dn[907][0]);
        let eq37_e1567_d_n1: f64 = (eq37_e1565 * s.dn[907][1]);
        let eq37_e1567_d_n2: f64 = (eq37_e1565 * s.dn[907][2]);
        let eq37_e1567_d_n3: f64 = (eq37_e1565 * s.dn[907][3]);
        let eq37_e1567_d_n4: f64 = (eq37_e1565 * s.dn[907][4]);
        let eq37_e1567_d_n5: f64 = (eq37_e1565 * s.dn[907][5]);
        let eq37_e1567_d_n6: f64 = (eq37_e1565 * s.dn[907][6]);
        let eq37_e1567_d_n7: f64 = (eq37_e1565 * s.dn[907][7]);
        let eq37_e1567_d_n8: f64 = (eq37_e1565 * s.dn[907][8]);
        let eq37_e1567_d_n9: f64 = (eq37_e1565 * s.dn[907][9]);
        let eq37_e1567_d_n10: f64 = (eq37_e1565 * s.dn[907][10]);
        let eq37_e1567_d_n11: f64 = (eq37_e1565 * s.dn[907][11]);
        let eq37_e1567_d_n12: f64 = (eq37_e1565 * s.dn[907][12]);
        let eq37_e1567_d_n13: f64 = (eq37_e1565 * s.dn[907][13]);
        (eq37_e1567, eq37_e1567_d_n0, eq37_e1567_d_n1, eq37_e1567_d_n2, eq37_e1567_d_n3, eq37_e1567_d_n4, eq37_e1567_d_n5, eq37_e1567_d_n6, eq37_e1567_d_n7, eq37_e1567_d_n8, eq37_e1567_d_n9, eq37_e1567_d_n10, eq37_e1567_d_n11, eq37_e1567_d_n12, eq37_e1567_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1569;
        let eq37_node_derivatives: [f64; 14] = [eq37_e1569_d_n0, eq37_e1569_d_n1, eq37_e1569_d_n2, eq37_e1569_d_n3, eq37_e1569_d_n4, eq37_e1569_d_n5, eq37_e1569_d_n6, eq37_e1569_d_n7, eq37_e1569_d_n8, eq37_e1569_d_n9, eq37_e1569_d_n10, eq37_e1569_d_n11, eq37_e1569_d_n12, eq37_e1569_d_n13];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            self.multiplicity * (eq37_value),
            &nodes,
            &eq37_node_derivatives,
            &branches,
            &eq37_branch_derivatives,
            self.multiplicity,
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
        let (eq38_e1586,) = {
    if (!(s.v[1552] != 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq38_value: f64 = eq38_e1586;
        stamper.stamp_current(
            Some(nodes[5]),
            Some(nodes[4]),
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
            Some(nodes[12]),
            Some(nodes[7]),
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
        let eq40_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[11]),
            Some(nodes[8]),
            self.multiplicity * (eq40_value),
            &[
            ],
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
        let eq41_value: f64 = 0.0;
        stamper.stamp_current(
            Some(nodes[9]),
            Some(nodes[7]),
            self.multiplicity * (eq41_value),
            &[
            ],
        );
    }
}

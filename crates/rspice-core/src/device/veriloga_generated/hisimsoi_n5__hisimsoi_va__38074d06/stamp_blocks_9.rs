#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_reactive_equation_30_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq30_e483, eq30_e483_d_n0, eq30_e483_d_n1, eq30_e483_d_n2, eq30_e483_d_n3, eq30_e483_d_n4, eq30_e483_d_n5, eq30_e483_d_n6, eq30_e483_d_n7, eq30_e483_d_n8, eq30_e483_d_n9, eq30_e483_d_n10, eq30_e483_d_n11, eq30_e483_d_n12, eq30_e483_d_n13, eq30_e483_d_n14, eq30_e483_d_n15, eq30_e483_d_n16, eq30_e483_d_n17, eq30_e483_d_n18, eq30_e483_d_b0, eq30_e483_d_b1, eq30_e483_d_b2, eq30_e483_d_b3, eq30_e483_d_b4, eq30_e483_d_b5, eq30_e483_d_b6, eq30_e483_d_b7, eq30_e483_d_b8, eq30_e483_d_b9, eq30_e483_d_b10, eq30_e483_d_b11, eq30_e483_q, eq30_e483_q_d_n0, eq30_e483_q_d_n1, eq30_e483_q_d_n2, eq30_e483_q_d_n3, eq30_e483_q_d_n4, eq30_e483_q_d_n5, eq30_e483_q_d_n6, eq30_e483_q_d_n7, eq30_e483_q_d_n8, eq30_e483_q_d_n9, eq30_e483_q_d_n10, eq30_e483_q_d_n11, eq30_e483_q_d_n12, eq30_e483_q_d_n13, eq30_e483_q_d_n14, eq30_e483_q_d_n15, eq30_e483_q_d_n16, eq30_e483_q_d_n17, eq30_e483_q_d_n18, eq30_e483_q_d_b0, eq30_e483_q_d_b1, eq30_e483_q_d_b2, eq30_e483_q_d_b3, eq30_e483_q_d_b4, eq30_e483_q_d_b5, eq30_e483_q_d_b6, eq30_e483_q_d_b7, eq30_e483_q_d_b8, eq30_e483_q_d_b9, eq30_e483_q_d_b10, eq30_e483_q_d_b11,) = {
    if (s.v[1848] != 0.0) {
        let eq30_e480: f64 = (s.v[563] * (nv10 - 0.0));
        let eq30_e480_d_n0: f64 = (s.dn[563][0] * (nv10 - 0.0));
        let eq30_e480_d_n1: f64 = (s.dn[563][1] * (nv10 - 0.0));
        let eq30_e480_d_n2: f64 = (s.dn[563][2] * (nv10 - 0.0));
        let eq30_e480_d_n3: f64 = (s.dn[563][3] * (nv10 - 0.0));
        let eq30_e480_d_n4: f64 = (s.dn[563][4] * (nv10 - 0.0));
        let eq30_e480_d_n5: f64 = (s.dn[563][5] * (nv10 - 0.0));
        let eq30_e480_d_n6: f64 = (s.dn[563][6] * (nv10 - 0.0));
        let eq30_e480_d_n7: f64 = (s.dn[563][7] * (nv10 - 0.0));
        let eq30_e480_d_n8: f64 = (s.dn[563][8] * (nv10 - 0.0));
        let eq30_e480_d_n9: f64 = (s.dn[563][9] * (nv10 - 0.0));
        let eq30_e480_d_n10: f64 = ((s.dn[563][10] * (nv10 - 0.0)) + s.v[563]);
        let eq30_e480_d_n11: f64 = (s.dn[563][11] * (nv10 - 0.0));
        let eq30_e480_d_n12: f64 = (s.dn[563][12] * (nv10 - 0.0));
        let eq30_e480_d_n13: f64 = (s.dn[563][13] * (nv10 - 0.0));
        let eq30_e480_d_n14: f64 = (s.dn[563][14] * (nv10 - 0.0));
        let eq30_e480_d_n15: f64 = (s.dn[563][15] * (nv10 - 0.0));
        let eq30_e480_d_n16: f64 = (s.dn[563][16] * (nv10 - 0.0));
        let eq30_e480_d_n17: f64 = (s.dn[563][17] * (nv10 - 0.0));
        let eq30_e480_d_n18: f64 = (s.dn[563][18] * (nv10 - 0.0));
        let eq30_e480_d_b0: f64 = (s.db[563][0] * (nv10 - 0.0));
        let eq30_e480_d_b1: f64 = (s.db[563][1] * (nv10 - 0.0));
        let eq30_e480_d_b2: f64 = (s.db[563][2] * (nv10 - 0.0));
        let eq30_e480_d_b3: f64 = (s.db[563][3] * (nv10 - 0.0));
        let eq30_e480_d_b4: f64 = (s.db[563][4] * (nv10 - 0.0));
        let eq30_e480_d_b5: f64 = (s.db[563][5] * (nv10 - 0.0));
        let eq30_e480_d_b6: f64 = (s.db[563][6] * (nv10 - 0.0));
        let eq30_e480_d_b7: f64 = (s.db[563][7] * (nv10 - 0.0));
        let eq30_e480_d_b8: f64 = (s.db[563][8] * (nv10 - 0.0));
        let eq30_e480_d_b9: f64 = (s.db[563][9] * (nv10 - 0.0));
        let eq30_e480_d_b10: f64 = (s.db[563][10] * (nv10 - 0.0));
        let eq30_e480_d_b11: f64 = (s.db[563][11] * (nv10 - 0.0));
        let eq30_e481_q: f64 = eq30_e480;
        (eq30_e480, eq30_e480_d_n0, eq30_e480_d_n1, eq30_e480_d_n2, eq30_e480_d_n3, eq30_e480_d_n4, eq30_e480_d_n5, eq30_e480_d_n6, eq30_e480_d_n7, eq30_e480_d_n8, eq30_e480_d_n9, eq30_e480_d_n10, eq30_e480_d_n11, eq30_e480_d_n12, eq30_e480_d_n13, eq30_e480_d_n14, eq30_e480_d_n15, eq30_e480_d_n16, eq30_e480_d_n17, eq30_e480_d_n18, eq30_e480_d_b0, eq30_e480_d_b1, eq30_e480_d_b2, eq30_e480_d_b3, eq30_e480_d_b4, eq30_e480_d_b5, eq30_e480_d_b6, eq30_e480_d_b7, eq30_e480_d_b8, eq30_e480_d_b9, eq30_e480_d_b10, eq30_e480_d_b11, eq30_e481_q, eq30_e480_d_n0, eq30_e480_d_n1, eq30_e480_d_n2, eq30_e480_d_n3, eq30_e480_d_n4, eq30_e480_d_n5, eq30_e480_d_n6, eq30_e480_d_n7, eq30_e480_d_n8, eq30_e480_d_n9, eq30_e480_d_n10, eq30_e480_d_n11, eq30_e480_d_n12, eq30_e480_d_n13, eq30_e480_d_n14, eq30_e480_d_n15, eq30_e480_d_n16, eq30_e480_d_n17, eq30_e480_d_n18, eq30_e480_d_b0, eq30_e480_d_b1, eq30_e480_d_b2, eq30_e480_d_b3, eq30_e480_d_b4, eq30_e480_d_b5, eq30_e480_d_b6, eq30_e480_d_b7, eq30_e480_d_b8, eq30_e480_d_b9, eq30_e480_d_b10, eq30_e480_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e483_q_d_n0, eq30_e483_q_d_n1, eq30_e483_q_d_n2, eq30_e483_q_d_n3, eq30_e483_q_d_n4, eq30_e483_q_d_n5, eq30_e483_q_d_n6, eq30_e483_q_d_n7, eq30_e483_q_d_n8, eq30_e483_q_d_n9, eq30_e483_q_d_n10, eq30_e483_q_d_n11, eq30_e483_q_d_n12, eq30_e483_q_d_n13, eq30_e483_q_d_n14, eq30_e483_q_d_n15, eq30_e483_q_d_n16, eq30_e483_q_d_n17, eq30_e483_q_d_n18];
        let eq30_reactive_branch_derivatives: [f64; 12] = [eq30_e483_q_d_b0, eq30_e483_q_d_b1, eq30_e483_q_d_b2, eq30_e483_q_d_b3, eq30_e483_q_d_b4, eq30_e483_q_d_b5, eq30_e483_q_d_b6, eq30_e483_q_d_b7, eq30_e483_q_d_b8, eq30_e483_q_d_b9, eq30_e483_q_d_b10, eq30_e483_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            None,
            &nodes,
            &eq30_reactive_node_derivatives,
            &branches,
            &eq30_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_34_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq34_e515, eq34_e515_d_n0, eq34_e515_d_n1, eq34_e515_d_n2, eq34_e515_d_n3, eq34_e515_d_n4, eq34_e515_d_n5, eq34_e515_d_n6, eq34_e515_d_n7, eq34_e515_d_n8, eq34_e515_d_n9, eq34_e515_d_n10, eq34_e515_d_n11, eq34_e515_d_n12, eq34_e515_d_n13, eq34_e515_d_n14, eq34_e515_d_n15, eq34_e515_d_n16, eq34_e515_d_n17, eq34_e515_d_n18, eq34_e515_d_b0, eq34_e515_d_b1, eq34_e515_d_b2, eq34_e515_d_b3, eq34_e515_d_b4, eq34_e515_d_b5, eq34_e515_d_b6, eq34_e515_d_b7, eq34_e515_d_b8, eq34_e515_d_b9, eq34_e515_d_b10, eq34_e515_d_b11, eq34_e515_q, eq34_e515_q_d_n0, eq34_e515_q_d_n1, eq34_e515_q_d_n2, eq34_e515_q_d_n3, eq34_e515_q_d_n4, eq34_e515_q_d_n5, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n8, eq34_e515_q_d_n9, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n13, eq34_e515_q_d_n14, eq34_e515_q_d_n15, eq34_e515_q_d_n16, eq34_e515_q_d_n17, eq34_e515_q_d_n18, eq34_e515_q_d_b0, eq34_e515_q_d_b1, eq34_e515_q_d_b2, eq34_e515_q_d_b3, eq34_e515_q_d_b4, eq34_e515_q_d_b5, eq34_e515_q_d_b6, eq34_e515_q_d_b7, eq34_e515_q_d_b8, eq34_e515_q_d_b9, eq34_e515_q_d_b10, eq34_e515_q_d_b11,) = {
    if (s.v[1849] != 0.0) {
        let eq34_e511_q: f64 = s.v[283];
        let eq34_e512: f64 = (s.v[281] + s.v[283]);
        let eq34_e512_d_n0: f64 = (s.dn[281][0] + s.dn[283][0]);
        let eq34_e512_d_n1: f64 = (s.dn[281][1] + s.dn[283][1]);
        let eq34_e512_d_n2: f64 = (s.dn[281][2] + s.dn[283][2]);
        let eq34_e512_d_n3: f64 = (s.dn[281][3] + s.dn[283][3]);
        let eq34_e512_d_n4: f64 = (s.dn[281][4] + s.dn[283][4]);
        let eq34_e512_d_n5: f64 = (s.dn[281][5] + s.dn[283][5]);
        let eq34_e512_d_n6: f64 = (s.dn[281][6] + s.dn[283][6]);
        let eq34_e512_d_n7: f64 = (s.dn[281][7] + s.dn[283][7]);
        let eq34_e512_d_n8: f64 = (s.dn[281][8] + s.dn[283][8]);
        let eq34_e512_d_n9: f64 = (s.dn[281][9] + s.dn[283][9]);
        let eq34_e512_d_n10: f64 = (s.dn[281][10] + s.dn[283][10]);
        let eq34_e512_d_n11: f64 = (s.dn[281][11] + s.dn[283][11]);
        let eq34_e512_d_n12: f64 = (s.dn[281][12] + s.dn[283][12]);
        let eq34_e512_d_n13: f64 = (s.dn[281][13] + s.dn[283][13]);
        let eq34_e512_d_n14: f64 = (s.dn[281][14] + s.dn[283][14]);
        let eq34_e512_d_n15: f64 = (s.dn[281][15] + s.dn[283][15]);
        let eq34_e512_d_n16: f64 = (s.dn[281][16] + s.dn[283][16]);
        let eq34_e512_d_n17: f64 = (s.dn[281][17] + s.dn[283][17]);
        let eq34_e512_d_n18: f64 = (s.dn[281][18] + s.dn[283][18]);
        let eq34_e512_d_b0: f64 = (s.db[281][0] + s.db[283][0]);
        let eq34_e512_d_b1: f64 = (s.db[281][1] + s.db[283][1]);
        let eq34_e512_d_b2: f64 = (s.db[281][2] + s.db[283][2]);
        let eq34_e512_d_b3: f64 = (s.db[281][3] + s.db[283][3]);
        let eq34_e512_d_b4: f64 = (s.db[281][4] + s.db[283][4]);
        let eq34_e512_d_b5: f64 = (s.db[281][5] + s.db[283][5]);
        let eq34_e512_d_b6: f64 = (s.db[281][6] + s.db[283][6]);
        let eq34_e512_d_b7: f64 = (s.db[281][7] + s.db[283][7]);
        let eq34_e512_d_b8: f64 = (s.db[281][8] + s.db[283][8]);
        let eq34_e512_d_b9: f64 = (s.db[281][9] + s.db[283][9]);
        let eq34_e512_d_b10: f64 = (s.db[281][10] + s.db[283][10]);
        let eq34_e512_d_b11: f64 = (s.db[281][11] + s.db[283][11]);
        let eq34_e512_q: f64 = eq34_e511_q;
        let eq34_e513: f64 = (p.p50 * eq34_e512);
        let eq34_e513_d_n0: f64 = (p.p50 * eq34_e512_d_n0);
        let eq34_e513_d_n1: f64 = (p.p50 * eq34_e512_d_n1);
        let eq34_e513_d_n2: f64 = (p.p50 * eq34_e512_d_n2);
        let eq34_e513_d_n3: f64 = (p.p50 * eq34_e512_d_n3);
        let eq34_e513_d_n4: f64 = (p.p50 * eq34_e512_d_n4);
        let eq34_e513_d_n5: f64 = (p.p50 * eq34_e512_d_n5);
        let eq34_e513_d_n6: f64 = (p.p50 * eq34_e512_d_n6);
        let eq34_e513_d_n7: f64 = (p.p50 * eq34_e512_d_n7);
        let eq34_e513_d_n8: f64 = (p.p50 * eq34_e512_d_n8);
        let eq34_e513_d_n9: f64 = (p.p50 * eq34_e512_d_n9);
        let eq34_e513_d_n10: f64 = (p.p50 * eq34_e512_d_n10);
        let eq34_e513_d_n11: f64 = (p.p50 * eq34_e512_d_n11);
        let eq34_e513_d_n12: f64 = (p.p50 * eq34_e512_d_n12);
        let eq34_e513_d_n13: f64 = (p.p50 * eq34_e512_d_n13);
        let eq34_e513_d_n14: f64 = (p.p50 * eq34_e512_d_n14);
        let eq34_e513_d_n15: f64 = (p.p50 * eq34_e512_d_n15);
        let eq34_e513_d_n16: f64 = (p.p50 * eq34_e512_d_n16);
        let eq34_e513_d_n17: f64 = (p.p50 * eq34_e512_d_n17);
        let eq34_e513_d_n18: f64 = (p.p50 * eq34_e512_d_n18);
        let eq34_e513_d_b0: f64 = (p.p50 * eq34_e512_d_b0);
        let eq34_e513_d_b1: f64 = (p.p50 * eq34_e512_d_b1);
        let eq34_e513_d_b2: f64 = (p.p50 * eq34_e512_d_b2);
        let eq34_e513_d_b3: f64 = (p.p50 * eq34_e512_d_b3);
        let eq34_e513_d_b4: f64 = (p.p50 * eq34_e512_d_b4);
        let eq34_e513_d_b5: f64 = (p.p50 * eq34_e512_d_b5);
        let eq34_e513_d_b6: f64 = (p.p50 * eq34_e512_d_b6);
        let eq34_e513_d_b7: f64 = (p.p50 * eq34_e512_d_b7);
        let eq34_e513_d_b8: f64 = (p.p50 * eq34_e512_d_b8);
        let eq34_e513_d_b9: f64 = (p.p50 * eq34_e512_d_b9);
        let eq34_e513_d_b10: f64 = (p.p50 * eq34_e512_d_b10);
        let eq34_e513_d_b11: f64 = (p.p50 * eq34_e512_d_b11);
        let eq34_e513_q: f64 = (p.p50 * eq34_e512_q);
        let eq34_e513_q_d_n0: f64 = (p.p50 * s.dn[283][0]);
        let eq34_e513_q_d_n1: f64 = (p.p50 * s.dn[283][1]);
        let eq34_e513_q_d_n2: f64 = (p.p50 * s.dn[283][2]);
        let eq34_e513_q_d_n3: f64 = (p.p50 * s.dn[283][3]);
        let eq34_e513_q_d_n4: f64 = (p.p50 * s.dn[283][4]);
        let eq34_e513_q_d_n5: f64 = (p.p50 * s.dn[283][5]);
        let eq34_e513_q_d_n6: f64 = (p.p50 * s.dn[283][6]);
        let eq34_e513_q_d_n7: f64 = (p.p50 * s.dn[283][7]);
        let eq34_e513_q_d_n8: f64 = (p.p50 * s.dn[283][8]);
        let eq34_e513_q_d_n9: f64 = (p.p50 * s.dn[283][9]);
        let eq34_e513_q_d_n10: f64 = (p.p50 * s.dn[283][10]);
        let eq34_e513_q_d_n11: f64 = (p.p50 * s.dn[283][11]);
        let eq34_e513_q_d_n12: f64 = (p.p50 * s.dn[283][12]);
        let eq34_e513_q_d_n13: f64 = (p.p50 * s.dn[283][13]);
        let eq34_e513_q_d_n14: f64 = (p.p50 * s.dn[283][14]);
        let eq34_e513_q_d_n15: f64 = (p.p50 * s.dn[283][15]);
        let eq34_e513_q_d_n16: f64 = (p.p50 * s.dn[283][16]);
        let eq34_e513_q_d_n17: f64 = (p.p50 * s.dn[283][17]);
        let eq34_e513_q_d_n18: f64 = (p.p50 * s.dn[283][18]);
        let eq34_e513_q_d_b0: f64 = (p.p50 * s.db[283][0]);
        let eq34_e513_q_d_b1: f64 = (p.p50 * s.db[283][1]);
        let eq34_e513_q_d_b2: f64 = (p.p50 * s.db[283][2]);
        let eq34_e513_q_d_b3: f64 = (p.p50 * s.db[283][3]);
        let eq34_e513_q_d_b4: f64 = (p.p50 * s.db[283][4]);
        let eq34_e513_q_d_b5: f64 = (p.p50 * s.db[283][5]);
        let eq34_e513_q_d_b6: f64 = (p.p50 * s.db[283][6]);
        let eq34_e513_q_d_b7: f64 = (p.p50 * s.db[283][7]);
        let eq34_e513_q_d_b8: f64 = (p.p50 * s.db[283][8]);
        let eq34_e513_q_d_b9: f64 = (p.p50 * s.db[283][9]);
        let eq34_e513_q_d_b10: f64 = (p.p50 * s.db[283][10]);
        let eq34_e513_q_d_b11: f64 = (p.p50 * s.db[283][11]);
        (eq34_e513, eq34_e513_d_n0, eq34_e513_d_n1, eq34_e513_d_n2, eq34_e513_d_n3, eq34_e513_d_n4, eq34_e513_d_n5, eq34_e513_d_n6, eq34_e513_d_n7, eq34_e513_d_n8, eq34_e513_d_n9, eq34_e513_d_n10, eq34_e513_d_n11, eq34_e513_d_n12, eq34_e513_d_n13, eq34_e513_d_n14, eq34_e513_d_n15, eq34_e513_d_n16, eq34_e513_d_n17, eq34_e513_d_n18, eq34_e513_d_b0, eq34_e513_d_b1, eq34_e513_d_b2, eq34_e513_d_b3, eq34_e513_d_b4, eq34_e513_d_b5, eq34_e513_d_b6, eq34_e513_d_b7, eq34_e513_d_b8, eq34_e513_d_b9, eq34_e513_d_b10, eq34_e513_d_b11, eq34_e513_q, eq34_e513_q_d_n0, eq34_e513_q_d_n1, eq34_e513_q_d_n2, eq34_e513_q_d_n3, eq34_e513_q_d_n4, eq34_e513_q_d_n5, eq34_e513_q_d_n6, eq34_e513_q_d_n7, eq34_e513_q_d_n8, eq34_e513_q_d_n9, eq34_e513_q_d_n10, eq34_e513_q_d_n11, eq34_e513_q_d_n12, eq34_e513_q_d_n13, eq34_e513_q_d_n14, eq34_e513_q_d_n15, eq34_e513_q_d_n16, eq34_e513_q_d_n17, eq34_e513_q_d_n18, eq34_e513_q_d_b0, eq34_e513_q_d_b1, eq34_e513_q_d_b2, eq34_e513_q_d_b3, eq34_e513_q_d_b4, eq34_e513_q_d_b5, eq34_e513_q_d_b6, eq34_e513_q_d_b7, eq34_e513_q_d_b8, eq34_e513_q_d_b9, eq34_e513_q_d_b10, eq34_e513_q_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e515_q_d_n0, eq34_e515_q_d_n1, eq34_e515_q_d_n2, eq34_e515_q_d_n3, eq34_e515_q_d_n4, eq34_e515_q_d_n5, eq34_e515_q_d_n6, eq34_e515_q_d_n7, eq34_e515_q_d_n8, eq34_e515_q_d_n9, eq34_e515_q_d_n10, eq34_e515_q_d_n11, eq34_e515_q_d_n12, eq34_e515_q_d_n13, eq34_e515_q_d_n14, eq34_e515_q_d_n15, eq34_e515_q_d_n16, eq34_e515_q_d_n17, eq34_e515_q_d_n18];
        let eq34_reactive_branch_derivatives: [f64; 12] = [eq34_e515_q_d_b0, eq34_e515_q_d_b1, eq34_e515_q_d_b2, eq34_e515_q_d_b3, eq34_e515_q_d_b4, eq34_e515_q_d_b5, eq34_e515_q_d_b6, eq34_e515_q_d_b7, eq34_e515_q_d_b8, eq34_e515_q_d_b9, eq34_e515_q_d_b10, eq34_e515_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            &nodes,
            &eq34_reactive_node_derivatives,
            &branches,
            &eq34_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_35_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let (eq35_e524, eq35_e524_d_n0, eq35_e524_d_n1, eq35_e524_d_n2, eq35_e524_d_n3, eq35_e524_d_n4, eq35_e524_d_n5, eq35_e524_d_n6, eq35_e524_d_n7, eq35_e524_d_n8, eq35_e524_d_n9, eq35_e524_d_n10, eq35_e524_d_n11, eq35_e524_d_n12, eq35_e524_d_n13, eq35_e524_d_n14, eq35_e524_d_n15, eq35_e524_d_n16, eq35_e524_d_n17, eq35_e524_d_n18, eq35_e524_d_b0, eq35_e524_d_b1, eq35_e524_d_b2, eq35_e524_d_b3, eq35_e524_d_b4, eq35_e524_d_b5, eq35_e524_d_b6, eq35_e524_d_b7, eq35_e524_d_b8, eq35_e524_d_b9, eq35_e524_d_b10, eq35_e524_d_b11, eq35_e524_q, eq35_e524_q_d_n0, eq35_e524_q_d_n1, eq35_e524_q_d_n2, eq35_e524_q_d_n3, eq35_e524_q_d_n4, eq35_e524_q_d_n5, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n8, eq35_e524_q_d_n9, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n13, eq35_e524_q_d_n14, eq35_e524_q_d_n15, eq35_e524_q_d_n16, eq35_e524_q_d_n17, eq35_e524_q_d_n18, eq35_e524_q_d_b0, eq35_e524_q_d_b1, eq35_e524_q_d_b2, eq35_e524_q_d_b3, eq35_e524_q_d_b4, eq35_e524_q_d_b5, eq35_e524_q_d_b6, eq35_e524_q_d_b7, eq35_e524_q_d_b8, eq35_e524_q_d_b9, eq35_e524_q_d_b10, eq35_e524_q_d_b11,) = {
    if (s.v[1849] != 0.0) {
        let eq35_e520_q: f64 = s.v[284];
        let eq35_e521: f64 = (s.v[282] + s.v[284]);
        let eq35_e521_d_n0: f64 = (s.dn[282][0] + s.dn[284][0]);
        let eq35_e521_d_n1: f64 = (s.dn[282][1] + s.dn[284][1]);
        let eq35_e521_d_n2: f64 = (s.dn[282][2] + s.dn[284][2]);
        let eq35_e521_d_n3: f64 = (s.dn[282][3] + s.dn[284][3]);
        let eq35_e521_d_n4: f64 = (s.dn[282][4] + s.dn[284][4]);
        let eq35_e521_d_n5: f64 = (s.dn[282][5] + s.dn[284][5]);
        let eq35_e521_d_n6: f64 = (s.dn[282][6] + s.dn[284][6]);
        let eq35_e521_d_n7: f64 = (s.dn[282][7] + s.dn[284][7]);
        let eq35_e521_d_n8: f64 = (s.dn[282][8] + s.dn[284][8]);
        let eq35_e521_d_n9: f64 = (s.dn[282][9] + s.dn[284][9]);
        let eq35_e521_d_n10: f64 = (s.dn[282][10] + s.dn[284][10]);
        let eq35_e521_d_n11: f64 = (s.dn[282][11] + s.dn[284][11]);
        let eq35_e521_d_n12: f64 = (s.dn[282][12] + s.dn[284][12]);
        let eq35_e521_d_n13: f64 = (s.dn[282][13] + s.dn[284][13]);
        let eq35_e521_d_n14: f64 = (s.dn[282][14] + s.dn[284][14]);
        let eq35_e521_d_n15: f64 = (s.dn[282][15] + s.dn[284][15]);
        let eq35_e521_d_n16: f64 = (s.dn[282][16] + s.dn[284][16]);
        let eq35_e521_d_n17: f64 = (s.dn[282][17] + s.dn[284][17]);
        let eq35_e521_d_n18: f64 = (s.dn[282][18] + s.dn[284][18]);
        let eq35_e521_d_b0: f64 = (s.db[282][0] + s.db[284][0]);
        let eq35_e521_d_b1: f64 = (s.db[282][1] + s.db[284][1]);
        let eq35_e521_d_b2: f64 = (s.db[282][2] + s.db[284][2]);
        let eq35_e521_d_b3: f64 = (s.db[282][3] + s.db[284][3]);
        let eq35_e521_d_b4: f64 = (s.db[282][4] + s.db[284][4]);
        let eq35_e521_d_b5: f64 = (s.db[282][5] + s.db[284][5]);
        let eq35_e521_d_b6: f64 = (s.db[282][6] + s.db[284][6]);
        let eq35_e521_d_b7: f64 = (s.db[282][7] + s.db[284][7]);
        let eq35_e521_d_b8: f64 = (s.db[282][8] + s.db[284][8]);
        let eq35_e521_d_b9: f64 = (s.db[282][9] + s.db[284][9]);
        let eq35_e521_d_b10: f64 = (s.db[282][10] + s.db[284][10]);
        let eq35_e521_d_b11: f64 = (s.db[282][11] + s.db[284][11]);
        let eq35_e521_q: f64 = eq35_e520_q;
        let eq35_e522: f64 = (p.p50 * eq35_e521);
        let eq35_e522_d_n0: f64 = (p.p50 * eq35_e521_d_n0);
        let eq35_e522_d_n1: f64 = (p.p50 * eq35_e521_d_n1);
        let eq35_e522_d_n2: f64 = (p.p50 * eq35_e521_d_n2);
        let eq35_e522_d_n3: f64 = (p.p50 * eq35_e521_d_n3);
        let eq35_e522_d_n4: f64 = (p.p50 * eq35_e521_d_n4);
        let eq35_e522_d_n5: f64 = (p.p50 * eq35_e521_d_n5);
        let eq35_e522_d_n6: f64 = (p.p50 * eq35_e521_d_n6);
        let eq35_e522_d_n7: f64 = (p.p50 * eq35_e521_d_n7);
        let eq35_e522_d_n8: f64 = (p.p50 * eq35_e521_d_n8);
        let eq35_e522_d_n9: f64 = (p.p50 * eq35_e521_d_n9);
        let eq35_e522_d_n10: f64 = (p.p50 * eq35_e521_d_n10);
        let eq35_e522_d_n11: f64 = (p.p50 * eq35_e521_d_n11);
        let eq35_e522_d_n12: f64 = (p.p50 * eq35_e521_d_n12);
        let eq35_e522_d_n13: f64 = (p.p50 * eq35_e521_d_n13);
        let eq35_e522_d_n14: f64 = (p.p50 * eq35_e521_d_n14);
        let eq35_e522_d_n15: f64 = (p.p50 * eq35_e521_d_n15);
        let eq35_e522_d_n16: f64 = (p.p50 * eq35_e521_d_n16);
        let eq35_e522_d_n17: f64 = (p.p50 * eq35_e521_d_n17);
        let eq35_e522_d_n18: f64 = (p.p50 * eq35_e521_d_n18);
        let eq35_e522_d_b0: f64 = (p.p50 * eq35_e521_d_b0);
        let eq35_e522_d_b1: f64 = (p.p50 * eq35_e521_d_b1);
        let eq35_e522_d_b2: f64 = (p.p50 * eq35_e521_d_b2);
        let eq35_e522_d_b3: f64 = (p.p50 * eq35_e521_d_b3);
        let eq35_e522_d_b4: f64 = (p.p50 * eq35_e521_d_b4);
        let eq35_e522_d_b5: f64 = (p.p50 * eq35_e521_d_b5);
        let eq35_e522_d_b6: f64 = (p.p50 * eq35_e521_d_b6);
        let eq35_e522_d_b7: f64 = (p.p50 * eq35_e521_d_b7);
        let eq35_e522_d_b8: f64 = (p.p50 * eq35_e521_d_b8);
        let eq35_e522_d_b9: f64 = (p.p50 * eq35_e521_d_b9);
        let eq35_e522_d_b10: f64 = (p.p50 * eq35_e521_d_b10);
        let eq35_e522_d_b11: f64 = (p.p50 * eq35_e521_d_b11);
        let eq35_e522_q: f64 = (p.p50 * eq35_e521_q);
        let eq35_e522_q_d_n0: f64 = (p.p50 * s.dn[284][0]);
        let eq35_e522_q_d_n1: f64 = (p.p50 * s.dn[284][1]);
        let eq35_e522_q_d_n2: f64 = (p.p50 * s.dn[284][2]);
        let eq35_e522_q_d_n3: f64 = (p.p50 * s.dn[284][3]);
        let eq35_e522_q_d_n4: f64 = (p.p50 * s.dn[284][4]);
        let eq35_e522_q_d_n5: f64 = (p.p50 * s.dn[284][5]);
        let eq35_e522_q_d_n6: f64 = (p.p50 * s.dn[284][6]);
        let eq35_e522_q_d_n7: f64 = (p.p50 * s.dn[284][7]);
        let eq35_e522_q_d_n8: f64 = (p.p50 * s.dn[284][8]);
        let eq35_e522_q_d_n9: f64 = (p.p50 * s.dn[284][9]);
        let eq35_e522_q_d_n10: f64 = (p.p50 * s.dn[284][10]);
        let eq35_e522_q_d_n11: f64 = (p.p50 * s.dn[284][11]);
        let eq35_e522_q_d_n12: f64 = (p.p50 * s.dn[284][12]);
        let eq35_e522_q_d_n13: f64 = (p.p50 * s.dn[284][13]);
        let eq35_e522_q_d_n14: f64 = (p.p50 * s.dn[284][14]);
        let eq35_e522_q_d_n15: f64 = (p.p50 * s.dn[284][15]);
        let eq35_e522_q_d_n16: f64 = (p.p50 * s.dn[284][16]);
        let eq35_e522_q_d_n17: f64 = (p.p50 * s.dn[284][17]);
        let eq35_e522_q_d_n18: f64 = (p.p50 * s.dn[284][18]);
        let eq35_e522_q_d_b0: f64 = (p.p50 * s.db[284][0]);
        let eq35_e522_q_d_b1: f64 = (p.p50 * s.db[284][1]);
        let eq35_e522_q_d_b2: f64 = (p.p50 * s.db[284][2]);
        let eq35_e522_q_d_b3: f64 = (p.p50 * s.db[284][3]);
        let eq35_e522_q_d_b4: f64 = (p.p50 * s.db[284][4]);
        let eq35_e522_q_d_b5: f64 = (p.p50 * s.db[284][5]);
        let eq35_e522_q_d_b6: f64 = (p.p50 * s.db[284][6]);
        let eq35_e522_q_d_b7: f64 = (p.p50 * s.db[284][7]);
        let eq35_e522_q_d_b8: f64 = (p.p50 * s.db[284][8]);
        let eq35_e522_q_d_b9: f64 = (p.p50 * s.db[284][9]);
        let eq35_e522_q_d_b10: f64 = (p.p50 * s.db[284][10]);
        let eq35_e522_q_d_b11: f64 = (p.p50 * s.db[284][11]);
        (eq35_e522, eq35_e522_d_n0, eq35_e522_d_n1, eq35_e522_d_n2, eq35_e522_d_n3, eq35_e522_d_n4, eq35_e522_d_n5, eq35_e522_d_n6, eq35_e522_d_n7, eq35_e522_d_n8, eq35_e522_d_n9, eq35_e522_d_n10, eq35_e522_d_n11, eq35_e522_d_n12, eq35_e522_d_n13, eq35_e522_d_n14, eq35_e522_d_n15, eq35_e522_d_n16, eq35_e522_d_n17, eq35_e522_d_n18, eq35_e522_d_b0, eq35_e522_d_b1, eq35_e522_d_b2, eq35_e522_d_b3, eq35_e522_d_b4, eq35_e522_d_b5, eq35_e522_d_b6, eq35_e522_d_b7, eq35_e522_d_b8, eq35_e522_d_b9, eq35_e522_d_b10, eq35_e522_d_b11, eq35_e522_q, eq35_e522_q_d_n0, eq35_e522_q_d_n1, eq35_e522_q_d_n2, eq35_e522_q_d_n3, eq35_e522_q_d_n4, eq35_e522_q_d_n5, eq35_e522_q_d_n6, eq35_e522_q_d_n7, eq35_e522_q_d_n8, eq35_e522_q_d_n9, eq35_e522_q_d_n10, eq35_e522_q_d_n11, eq35_e522_q_d_n12, eq35_e522_q_d_n13, eq35_e522_q_d_n14, eq35_e522_q_d_n15, eq35_e522_q_d_n16, eq35_e522_q_d_n17, eq35_e522_q_d_n18, eq35_e522_q_d_b0, eq35_e522_q_d_b1, eq35_e522_q_d_b2, eq35_e522_q_d_b3, eq35_e522_q_d_b4, eq35_e522_q_d_b5, eq35_e522_q_d_b6, eq35_e522_q_d_b7, eq35_e522_q_d_b8, eq35_e522_q_d_b9, eq35_e522_q_d_b10, eq35_e522_q_d_b11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e524_q_d_n0, eq35_e524_q_d_n1, eq35_e524_q_d_n2, eq35_e524_q_d_n3, eq35_e524_q_d_n4, eq35_e524_q_d_n5, eq35_e524_q_d_n6, eq35_e524_q_d_n7, eq35_e524_q_d_n8, eq35_e524_q_d_n9, eq35_e524_q_d_n10, eq35_e524_q_d_n11, eq35_e524_q_d_n12, eq35_e524_q_d_n13, eq35_e524_q_d_n14, eq35_e524_q_d_n15, eq35_e524_q_d_n16, eq35_e524_q_d_n17, eq35_e524_q_d_n18];
        let eq35_reactive_branch_derivatives: [f64; 12] = [eq35_e524_q_d_b0, eq35_e524_q_d_b1, eq35_e524_q_d_b2, eq35_e524_q_d_b3, eq35_e524_q_d_b4, eq35_e524_q_d_b5, eq35_e524_q_d_b6, eq35_e524_q_d_b7, eq35_e524_q_d_b8, eq35_e524_q_d_b9, eq35_e524_q_d_b10, eq35_e524_q_d_b11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &nodes,
            &eq35_reactive_node_derivatives,
            &branches,
            &eq35_reactive_branch_derivatives,
            self.multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equation_46_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq46_e608, eq46_e608_d_n18, eq46_e608_q, eq46_e608_q_d_n18,) = {
    if ((s.v[1849] != 0.0) && (p.p34 != 0.0)) {
        let eq46_e603: f64 = (1e-9 / 0.0001);
        let eq46_e605: f64 = (eq46_e603 * (nv18 - 0.0));
        let eq46_e606_q: f64 = eq46_e605;
        (eq46_e605, eq46_e603, eq46_e606_q, eq46_e603,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[18]),
            None,
            &[
                GeneratedDerivative::node(nodes[18], self.multiplicity * (eq46_e608_q_d_n18)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_47_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq47_e619, eq47_e619_d_n13, eq47_e619_q, eq47_e619_q_d_n13,) = {
    if ((s.v[1849] != 0.0) && (p.p34 != 0.0)) {
        let eq47_e614: f64 = (1e-9 / 0.0001);
        let eq47_e616: f64 = (eq47_e614 * (nv13 - 0.0));
        let eq47_e617_q: f64 = eq47_e616;
        (eq47_e616, eq47_e614, eq47_e617_q, eq47_e614,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[13]),
            None,
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * (eq47_e619_q_d_n13)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_52_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq52_e658, eq52_e658_d_n17, eq52_e658_q, eq52_e658_q_d_n17,) = {
    if ((s.v[1849] != 0.0) && (s.v[1850] != 0.0)) {
        let eq52_e653: f64 = (1e-9 / 0.0001);
        let eq52_e655: f64 = (eq52_e653 * (nv17 - 0.0));
        let eq52_e656_q: f64 = eq52_e655;
        (eq52_e655, eq52_e653, eq52_e656_q, eq52_e653,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[17]),
            None,
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * (eq52_e658_q_d_n17)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_59_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq59_e716, eq59_e716_d_n17, eq59_e716_q, eq59_e716_q_d_n17,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p37 != 0.0)) {
        let eq59_e711: f64 = (1e-9 / 0.0001);
        let eq59_e713: f64 = (eq59_e711 * (nv17 - 0.0));
        let eq59_e714_q: f64 = eq59_e713;
        (eq59_e713, eq59_e711, eq59_e714_q, eq59_e711,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[17]),
            None,
            &[
                GeneratedDerivative::node(nodes[17], self.multiplicity * (eq59_e716_q_d_n17)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_67_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq67_e784, eq67_e784_d_n15, eq67_e784_q, eq67_e784_q_d_n15,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq67_e779: f64 = (1e-9 / 0.0001);
        let eq67_e781: f64 = (eq67_e779 * (nv15 - 0.0));
        let eq67_e782_q: f64 = eq67_e781;
        (eq67_e781, eq67_e779, eq67_e782_q, eq67_e779,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[15]),
            None,
            &[
                GeneratedDerivative::node(nodes[15], self.multiplicity * (eq67_e784_q_d_n15)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_68_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq68_e796, eq68_e796_d_n16, eq68_e796_q, eq68_e796_q_d_n16,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq68_e791: f64 = (1e-9 / 0.0001);
        let eq68_e793: f64 = (eq68_e791 * (nv16 - 0.0));
        let eq68_e794_q: f64 = eq68_e793;
        (eq68_e793, eq68_e791, eq68_e794_q, eq68_e791,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[16]),
            None,
            &[
                GeneratedDerivative::node(nodes[16], self.multiplicity * (eq68_e796_q_d_n16)),
            ],
        );
    }

    pub(super) fn stamp_reactive_equation_69_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq69_e808, eq69_e808_d_n13, eq69_e808_q, eq69_e808_q_d_n13,) = {
    if ((!(s.v[1849] != 0.0)) && (p.p34 != 0.0)) {
        let eq69_e803: f64 = (1e-9 / 0.0001);
        let eq69_e805: f64 = (eq69_e803 * (nv13 - 0.0));
        let eq69_e806_q: f64 = eq69_e805;
        (eq69_e805, eq69_e803, eq69_e806_q, eq69_e803,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive(
            Some(nodes[13]),
            None,
            &[
                GeneratedDerivative::node(nodes[13], self.multiplicity * (eq69_e808_q_d_n13)),
            ],
        );
    }
}
